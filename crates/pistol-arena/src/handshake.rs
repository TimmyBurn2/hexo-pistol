//! Who the engine on the other end is, and whether a strength claim may come
//! from it.
//!
//! Two refusals, both before a single game is played:
//!
//! **The protocol version.** The two sides may be distinct builds — that is the
//! point of naming a binary per configuration — so the arena cannot assume the
//! program it spawned speaks the protocol it was compiled against. It reads
//! `id protocol` and refuses anything else by name.
//!
//! **The mode.** Every strength claim comes from an instrument mode (CLAUDE.md
//! rule 6), and instrument mode is where `threads = 1` is enforced
//! (docs/decisions.md D-7). A play-mode engine given a depth budget is a
//! perfectly legal request that nothing else in the stack would question — the
//! engine's own `Budget::check_supported` refuses only a wall-clock budget — so
//! this is the only place the rule can be enforced (docs/decisions.md D-162).
//!
//! The `id` lines are kept verbatim and copied into the report, because a claim
//! ships the instrument it was measured with and an arena log that cannot
//! recover the candidate radius and the table size cannot be re-run.

use crate::channel::{Channel, Received};
use crate::error::ArenaError;

/// The mode token a run demands of both engines.
pub const REQUIRED_MODE: &str = "instrument";

/// How many lines the arena will read before deciding a handshake is not one.
///
/// The engine writes five `id` lines plus whatever it was configured to add;
/// this bounds a peer that answers forever without ever saying `pistolok`.
pub const MAX_HANDSHAKE_LINES: usize = 64;

/// What an engine said about itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    /// The `id` lines, verbatim, without the `id ` prefix, in the order given.
    pub lines: Vec<String>,
}

impl Identity {
    /// The value of the first `id` line starting with `key `.
    pub fn field(&self, key: &str) -> Option<&str> {
        self.lines
            .iter()
            .find_map(|line| line.strip_prefix(key)?.strip_prefix(' '))
    }
}

/// Shake hands, and refuse an engine a strength claim may not come from.
pub fn shake(channel: &mut Channel, timeout_ms: u64) -> Result<Identity, ArenaError> {
    let label = channel.label().to_string();
    let refuse = |why: String| ArenaError::Handshake {
        engine: label.clone(),
        why,
    };

    if channel.send(pistol_cli::protocol::HANDSHAKE).is_err() {
        return Err(refuse(String::from(
            "it closed its input before the handshake could be sent",
        )));
    }

    let mut lines: Vec<String> = Vec::new();
    let mut greeted = false;
    for _ in 0..MAX_HANDSHAKE_LINES {
        // Opening 0, turn 0: no game has started, and a hang here is still a
        // hang rather than a handshake complaint.
        match channel.receive(timeout_ms, 0, 0)? {
            Received::Closed => {
                return Err(refuse(String::from(
                    "it closed its output during the handshake",
                )));
            }
            Received::Overlong => {
                return Err(refuse(format!(
                    "it wrote more than {} bytes with no newline during the handshake, which is \
                     not a line",
                    crate::channel::MAX_LINE_BYTES
                )));
            }
            Received::Line(line) if line == pistol_cli::report::HANDSHAKE_OK => {
                greeted = true;
                break;
            }
            Received::Line(line) => {
                let Some(rest) = line.strip_prefix(pistol_cli::report::ID_PREFIX) else {
                    return Err(refuse(format!(
                        "it answered `{line}`, which is neither an `{}` line nor `{}`",
                        pistol_cli::report::ID_PREFIX,
                        pistol_cli::report::HANDSHAKE_OK
                    )));
                };
                lines.push(rest.trim_start().to_string());
            }
        }
    }
    if !greeted {
        return Err(refuse(format!(
            "it wrote {MAX_HANDSHAKE_LINES} lines without saying `{}`",
            pistol_cli::report::HANDSHAKE_OK
        )));
    }

    let identity = Identity { lines };

    let expected = pistol_cli::protocol::PROTOCOL_VERSION;
    match identity.field("protocol") {
        Some(version) if version == expected => {}
        Some(version) => {
            return Err(refuse(format!(
                "it speaks protocol `{version}` and this arena speaks `{expected}`; a referee \
                 that guessed at the difference would be relaying moves it did not understand"
            )));
        }
        None => {
            return Err(refuse(String::from(
                "its handshake names no protocol version",
            )));
        }
    }

    match identity.field("mode") {
        Some(REQUIRED_MODE) => {}
        Some(mode) => {
            return Err(refuse(format!(
                "it is in `{mode}` mode, and every strength claim comes from `{REQUIRED_MODE}` \
                 mode — which is where a single search thread and a stable tie-break are \
                 enforced (CLAUDE.md rule 6, docs/decisions.md D-7)"
            )));
        }
        None => {
            return Err(refuse(String::from("its handshake names no mode")));
        }
    }

    Ok(identity)
}
