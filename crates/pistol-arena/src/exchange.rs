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
/// interrupted iteration (docs/decisions.md D-80). `pub(crate)` so the capture
/// pass recognises the closing line through this one reader rather than growing
/// a second (docs/experiments/wp20m_design.md §8).
pub(crate) fn totals_of(line: &str) -> Option<(u64, u64, u32)> {
    let words = fields_of(line)?;
    Some((
        value_of(&words, "nodes")?.parse().ok()?,
        value_of(&words, pistol_cli::report::TIME_FIELD)?
            .parse()
            .ok()?,
        value_of(&words, "depth_turns")?.parse().ok()?,
    ))
}

/// The words after the `totals` marker, in order, or `None` if this is not the
/// closing report.
///
/// A WORD LIST rather than a key-value map, because the score is TWO words after
/// its key — `cp <n>`, `mate <t>` or `-mate <t>` — and a map keyed by field name
/// yields the tag and loses the number
/// (`docs/experiments/wp20s_design.md` §3).
pub(crate) fn fields_of(line: &str) -> Option<Vec<&str>> {
    let prefix = format!(
        "{} {} ",
        pistol_cli::report::INFO_PREFIX,
        pistol_cli::report::TOTALS_MARKER
    );
    Some(line.strip_prefix(&prefix)?.split_whitespace().collect())
}

/// The word after `key`, matched whole so `nodes` cannot match `search_nodes`.
pub(crate) fn value_of<'a>(words: &[&'a str], key: &str) -> Option<&'a str> {
    words
        .iter()
        .position(|word| *word == key)
        .and_then(|at| words.get(at + 1))
        .copied()
}

#[cfg(test)]
mod tests {
    use super::{fields_of, totals_of, value_of};

    /// A totals line the engine could have written, with the solver block.
    const SOLVER: &str = "info totals depth_turns 3 seldepth 3 nodes 90 search_nodes 60 \
                          solver_nodes 30 solver_firings 2 solver_invocations 2 solver_proofs 1 \
                          solver_root_nodes 4 nps 900 time 100 hashfull 12 score cp 7 pv 0,0";
    /// The same without it, which is every gate-off line.
    const PLAIN: &str =
        "info totals depth_turns 1 seldepth 1 nodes 4 nps 1 time 0 hashfull 0 score mate 3 pv 0,0";

    #[test]
    fn fields_of_gives_the_word_after_score_and_the_word_after_that() {
        let words = fields_of(PLAIN).expect("a totals line");
        let at = words
            .iter()
            .position(|w| *w == "score")
            .expect("a score key");
        assert_eq!(words[at + 1], "mate");
        assert_eq!(words[at + 2], "3");
    }

    #[test]
    fn fields_of_reads_a_captured_line_that_has_no_time_field() {
        // What the capture writes: the two wall-clock fields removed.
        let captured = "info totals depth_turns 1 seldepth 1 nodes 4 hashfull 0 score cp 0 pv 0,0";
        let words = fields_of(captured).expect("a captured totals line is still a totals line");
        assert_eq!(value_of(&words, "nodes"), Some("4"));
        assert_eq!(value_of(&words, TIME_MISSING), None);
    }

    /// The field a captured line no longer carries.
    const TIME_MISSING: &str = "time";

    #[test]
    fn fields_of_refuses_a_line_without_the_totals_marker() {
        assert!(fields_of("info depth_turns 1 nodes 4").is_none());
        assert!(fields_of("bestmove 0,0").is_none());
    }

    #[test]
    fn value_of_matches_a_whole_word_so_nodes_is_not_search_nodes() {
        let words = fields_of(SOLVER).expect("a totals line");
        assert_eq!(value_of(&words, "nodes"), Some("90"));
        assert_eq!(value_of(&words, "search_nodes"), Some("60"));
        assert_eq!(value_of(&words, "solver_nodes"), Some("30"));
    }

    #[test]
    fn totals_of_still_refuses_a_line_missing_nodes_time_or_depth() {
        // All three lookups are load-bearing: this is what a fourth one, made
        // load-bearing, would break for the SPRT path.
        assert!(totals_of(PLAIN).is_some());
        for key in ["depth_turns 1 ", "nodes 4 ", "time 0 "] {
            let broken = PLAIN.replacen(key, "", 1);
            assert!(
                totals_of(&broken).is_none(),
                "a totals line missing `{key}` was billed anyway"
            );
        }
    }

    #[test]
    fn totals_of_reads_the_three_the_sprt_path_bills_from() {
        assert_eq!(totals_of(SOLVER), Some((90, 100, 3)));
    }
}
