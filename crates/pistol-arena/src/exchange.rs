use pistol_core::Turn;

use crate::channel::{Channel, Received};
use crate::error::ArenaError;
use crate::game::Rules;
use crate::reap::Death;
use crate::record::{Compute, ForfeitReason};

/// What one `go` produced.
pub enum Answer {
    /// The turn the engine would play.
    Move(Turn),
    /// It stopped playing legally.
    Forfeit {
        /// Why.
        reason: ForfeitReason,
        /// The line that earned it, verbatim, when there was one.
        line: Option<String>,
    },
}

/// Send the position and the budget, and read to `bestmove`.
pub fn ask(
    channel: &mut Channel,
    moves: &[Turn],
    rules: &Rules<'_>,
    opening: usize,
    turn: u32,
    compute: &mut Compute,
) -> Result<Answer, ArenaError> {
    // Nothing may be waiting before the question is asked. A line the engine
    // volunteered would otherwise be read as the answer to THIS `go`, from a
    // position it was never shown (docs/decisions.md D-172).
    if let Some(stray) = channel.unsolicited() {
        return out_of_turn(channel, stray, rules.hang_timeout_ms);
    }
    let position = position_line(moves);
    for line in [position.as_str(), rules.go_line] {
        if channel.send(line).is_err() {
            return closed(channel, ForfeitReason::EngineExited, rules.hang_timeout_ms);
        }
    }

    loop {
        match channel.receive(rules.hang_timeout_ms, opening, turn)? {
            Received::Closed => {
                return closed(channel, ForfeitReason::EngineExited, rules.hang_timeout_ms);
            }
            Received::Overlong => {
                return Ok(Answer::Forfeit {
                    reason: ForfeitReason::ProtocolError,
                    line: Some(format!(
                        "it wrote more than {} bytes with no newline, which is not a line",
                        crate::channel::MAX_LINE_BYTES
                    )),
                });
            }
            Received::Line(line) => {
                if let Some(rest) =
                    line.strip_prefix(&format!("{} ", pistol_cli::report::BESTMOVE_PREFIX))
                {
                    return Ok(match rest.trim().parse::<Turn>() {
                        Ok(turn) => Answer::Move(turn),
                        Err(_) => Answer::Forfeit {
                            reason: ForfeitReason::BadBestmove,
                            line: Some(line),
                        },
                    });
                }
                if line.starts_with(&format!("{} ", pistol_cli::report::ERROR_PREFIX)) {
                    return Ok(Answer::Forfeit {
                        reason: ForfeitReason::ProtocolError,
                        line: Some(line),
                    });
                }
                if let Some(totals) = totals_of(&line) {
                    compute.add(totals.0, totals.1, totals.2);
                    continue;
                }
                if line.starts_with(&format!("{} ", pistol_cli::report::INFO_PREFIX)) {
                    continue;
                }
                return Ok(Answer::Forfeit {
                    reason: ForfeitReason::ProtocolError,
                    line: Some(line),
                });
            }
        }
    }
}

/// An engine that spoke when it was not asked.
///
/// A forfeit, because it is a deterministic protocol violation — except when the
/// "stray" is the pipe closing, which is the ordinary end-of-engine path and is
/// classified as such.
fn out_of_turn(
    channel: &mut Channel,
    stray: Received,
    timeout_ms: u64,
) -> Result<Answer, ArenaError> {
    let why = match stray {
        Received::Line(line) => format!("it wrote `{line}` when nothing had been asked of it"),
        Received::Overlong => format!(
            "it wrote more than {} bytes with no newline when nothing had been asked of it",
            crate::channel::MAX_LINE_BYTES
        ),
        Received::Closed => {
            return closed(channel, ForfeitReason::EngineExited, timeout_ms);
        }
    };
    Ok(Answer::Forfeit {
        reason: ForfeitReason::ProtocolError,
        line: Some(why),
    })
}

/// A child that closed its pipe: a chosen exit forfeits, a killed one abandons
/// the run (docs/decisions.md D-159).
fn closed(
    channel: &mut Channel,
    reason: ForfeitReason,
    timeout_ms: u64,
) -> Result<Answer, ArenaError> {
    match channel.death(timeout_ms) {
        Death::Exited(code) => {
            let mut why = format!("exited with code {code}");
            let tail = channel.stderr_tail();
            if !tail.is_empty() {
                why.push_str("; stderr: ");
                why.push_str(&tail.join(" | "));
            }
            Ok(Answer::Forfeit {
                reason,
                line: Some(why),
            })
        }
        Death::Killed(why) => {
            let tail = channel.stderr_tail();
            let detail = if tail.is_empty() {
                why
            } else {
                format!("{why}; stderr: {}", tail.join(" | "))
            };
            Err(ArenaError::Killed {
                engine: channel.label().to_string(),
                why: detail,
            })
        }
    }
}

/// `position start moves …` — the whole game so far (docs/decisions.md D-6).
pub fn position_line(moves: &[Turn]) -> String {
    let mut line = format!("{} start moves", pistol_cli::protocol::POSITION);
    for turn in moves {
        line.push(' ');
        line.push_str(&turn.to_string());
    }
    line
}

/// `(nodes, time_ms, depth_turns)` from the closing report, or `None` if this
/// is not one.
///
/// The `totals` marker is what tells the closing line from a per-depth one; a
/// driver that billed compute to the wrong one would under-count every
/// interrupted iteration (docs/decisions.md D-80).
fn totals_of(line: &str) -> Option<(u64, u64, u32)> {
    let prefix = format!(
        "{} {} ",
        pistol_cli::report::INFO_PREFIX,
        pistol_cli::report::TOTALS_MARKER
    );
    let rest = line.strip_prefix(&prefix)?;
    let words: Vec<&str> = rest.split_whitespace().collect();
    let value = |key: &str| -> Option<&str> {
        words
            .iter()
            .position(|word| *word == key)
            .and_then(|at| words.get(at + 1))
            .copied()
    };
    Some((
        value("nodes")?.parse().ok()?,
        value(pistol_cli::report::TIME_FIELD)?.parse().ok()?,
        value("depth_turns")?.parse().ok()?,
    ))
}
