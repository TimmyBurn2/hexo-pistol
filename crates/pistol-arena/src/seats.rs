//! Spawning the engines one game is played between, setting them up, and taking
//! them down again — ONE sequence, and every driver in this crate calls it.
//!
//! # Why this is a function and not a paragraph
//!
//! It used to be four statements inside `schedule::one_game`, and the replay
//! mode needs the identical four. Three consecutive fresh-context design
//! reviews each caught a hand-written DESCRIPTION or COPY of them being wrong —
//! the last one on a document written specifically to stop that from happening
//! (docs/decisions.md D-403, D-404, D-406, D-407). A copy that makes the same
//! calls today is not an inheritance: nothing fails when a future edit moves one
//! of them. So the sequence is extracted here, both drivers call it, and
//! `crates/pistol-arena/tests/seat_setup_identity_tests.rs` pins that the
//! extraction changed no game record byte.
//!
//! # The order, and why each step is where it is
//!
//! Every channel is STARTED before any is shaken. That is the order the
//! generation path has always run and it is preserved exactly: an engine
//! refused at handshake must be refused with the other side already spawned, so
//! the run fails the same way whichever side is at fault.
//!
//! `NEW_GAME` is sent unconditionally on every fresh spawn even though the
//! process is new. `schedule.rs`'s own module documentation says why the spawn
//! is fresh at all (worker-invariance, docs/decisions.md D-164); the send stays
//! because D-7's gate certifies what it clears and this crate has never relied
//! on fresh-spawn emptiness in its place.
//!
//! Teardown runs only on a driver that RETURNED. A driver that failed leaves its
//! channels to `Channel`'s own `Drop`, which kills the child rather than asking
//! it to quit — a search still running would otherwise be measured into the next
//! game's timings. That asymmetry is the generation path's, kept.

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
