use crate::channel::Channel;
use crate::config::EngineSection;
use crate::error::ArenaError;
use crate::handshake;
use crate::identity::{self, EngineIdentity};

/// One seat: the engine document that spawns it, and who that engine was when
/// the run this seat belongs to started.
pub struct Seat<'a> {
    /// The document naming the binary and its config.
    pub section: &'a EngineSection,
    /// The run-start capture every later spawn is re-verified against
    /// (docs/decisions.md D-199).
    pub identity: &'a EngineIdentity,
}

/// Spawn every seat, set each one up, hand the live channels to `drive`, and
/// take them down again.
///
/// `drive` receives the channels indexed exactly as `seats` was, which is how
/// compute and forfeits are attributed.
pub fn with_seats<const N: usize, T>(
    seats: &[Seat<'_>; N],
    hang_timeout_ms: u64,
    drive: impl FnOnce(&mut [Channel; N]) -> Result<T, ArenaError>,
) -> Result<T, ArenaError> {
    let mut started: Vec<Channel> = Vec::with_capacity(N);
    for seat in seats {
        started.push(Channel::start(
            &seat.section.label,
            &seat.section.binary,
            &seat.section.config,
        )?);
    }
    let mut channels: [Channel; N] = started
        .try_into()
        .unwrap_or_else(|_| unreachable!("one channel was started per seat"));

    for (seat, channel) in seats.iter().zip(channels.iter_mut()) {
        let spoken = handshake::shake(channel, hang_timeout_ms)?;
        // Digests were captured once before the first game, and engines are
        // respawned from disk per game: without this, a config or weights file
        // edited mid-run silently changes the experiment while the report
        // attests the old one (docs/decisions.md D-188's operating rule,
        // D-199). Drift aborts the RUN by name; it is never a game result.
        identity::verify_respawn(seat.section, seat.identity, &spoken)?;
        if channel.send(pistol_cli::protocol::NEW_GAME).is_err() {
            return Err(ArenaError::Handshake {
                engine: channel.label().to_string(),
                why: String::from("it closed its input before the game started"),
            });
        }
    }

    let driven = drive(&mut channels)?;
    for channel in &mut channels {
        channel.shutdown();
    }
    Ok(driven)
}
