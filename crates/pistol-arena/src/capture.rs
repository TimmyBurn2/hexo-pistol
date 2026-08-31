use std::fmt::Write as _;

use pistol_core::{GameState, Outcome, Turn};

use crate::channel::{Channel, Received};
use crate::error::ArenaError;
use crate::exchange;
use crate::seats::{self, Seat};
use crate::transcript::{RecordedGame, Transcript};

pub use crate::capture_file::{CAPTURE_FORMAT_VERSION, CaptureRecord, read, render};

/// A refusal naming the capture and what was wrong with it.
fn refuse(what: impl Into<String>) -> ArenaError {
    ArenaError::config("capture", what.into())
}

/// The prefixes of one game the engine can legally be asked about.
///
/// `k` from zero to the recorded length, less the last when the game's final
/// turn wins: a won position is terminal and `set_position` refuses it, so it is
/// not a position any engine can be asked about (rule 4).
///
/// # Errors
/// If the recorded move list is not a legal game. `transcript::read` has already
/// refused such a report, so this is a guard rather than a path.
pub fn asked_prefixes(game: &RecordedGame) -> Result<Vec<usize>, ArenaError> {
    let mut state = GameState::new_game();
    let mut decided = false;
    for (at, turn) in game.moves.iter().enumerate() {
        let outcome = state.make_turn(*turn).map_err(|error| {
            refuse(format!(
                "game {}: recorded turn {} is not legal in the position the moves before it \
                 reach: {error}",
                game.index,
                at + 1
            ))
        })?;
        decided = matches!(outcome, Outcome::Win { .. });
    }
    let last = game.moves.len();
    Ok((0..=last).filter(|k| !(decided && *k == last)).collect())
}

/// The `position` line for a prefix.
///
/// `position start` at zero, never `position start moves` with nothing after
/// it — which the engine refuses by name.
fn position_line(moves: &[Turn]) -> String {
    if moves.is_empty() {
        pistol_cli::protocol::POSITION.to_string() + " start"
    } else {
        exchange::position_line(moves)
    }
}

/// The totals line without ` nps <n> time <n>`.
///
/// Gate 9's own rule: those two are the only fields two runs may disagree about
/// (`tools/determinism.sh`). Everything else is the engine's own bytes.
///
/// # Errors
/// If the pair is not present in that order, which no line this engine writes
/// can manage.
pub fn normalise(line: &str) -> Result<String, ArenaError> {
    let nps = format!(" {} ", pistol_cli::report::NPS_FIELD);
    let time = format!(" {} ", pistol_cli::report::TIME_FIELD);
    let at = line
        .find(&nps)
        .ok_or_else(|| refuse(format!("a totals line carries no `nps` field: `{line}`")))?;
    let after_nps = at + nps.len();
    let digits = line[after_nps..]
        .find(|c: char| !c.is_ascii_digit())
        .map_or(line.len(), |n| after_nps + n);
    if !line[after_nps..digits].chars().all(|c| c.is_ascii_digit()) || after_nps == digits {
        return Err(refuse(format!(
            "`nps` is not followed by a count: `{line}`"
        )));
    }
    if !line[digits..].starts_with(&time) {
        return Err(refuse(format!(
            "`time` does not follow `nps` on a totals line: `{line}`"
        )));
    }
    let after_time = digits + time.len();
    let end = line[after_time..]
        .find(|c: char| !c.is_ascii_digit())
        .map_or(line.len(), |n| after_time + n);
    if after_time == end {
        return Err(refuse(format!(
            "`time` is not followed by a count: `{line}`"
        )));
    }
    Ok(format!("{}{}", &line[..at], &line[end..]))
}

/// The identity of a capture run: what was played, what was asked, and the
/// grammar the answer is written in.
///
/// NOT the source report's `source_sha256`, which digests the whole file
/// including its timing block: two captures over reports of one experiment taken
/// on different days would otherwise differ for a reason that changes no answer.
pub fn capture_sha256(experiment_sha256: &str, go_line: &str, format_version: u32) -> String {
    let mut canonical = String::new();
    let _ = writeln!(canonical, "capture_format {format_version}");
    let _ = writeln!(canonical, "experiment_sha256 {experiment_sha256}");
    let _ = writeln!(canonical, "label_go {go_line}");
    pistol_cli::sha256::sha256_hex(canonical.as_bytes())
}

/// The `go` line a label ask is made at.
pub fn label_go_line(nodes: u64) -> String {
    crate::config::BudgetSection::Nodes { value: nodes }
        .go_line()
        .unwrap_or_else(|| unreachable!("a nodes budget always spells a go line"))
}

/// Refuse a report whose two seats do not attest the same engine.
///
/// The comparison is over [`crate::identity::EngineIdentity`], which carries no
/// label — `validate` and `transcript::read` both REFUSE identical labels, so a
/// self-play report's two sections always differ there and a comparison over the
/// sections would refuse every input this pipeline can produce.
///
/// # Errors
/// Naming the field that differed, because "different engines" is unreadable on
/// two sections that name one binary and one config.
pub fn one_engine(transcript: &Transcript) -> Result<(), ArenaError> {
    let (a, b) = (&transcript.identities[0], &transcript.identities[1]);
    let differing = if a.binary_sha256 != b.binary_sha256 {
        "binary_sha256"
    } else if a.config_sha256 != b.config_sha256 {
        "config_sha256"
    } else if a.weights_sha256 != b.weights_sha256 {
        "weights_sha256"
    } else if a.id_lines != b.id_lines {
        "id_lines — the two seats spell the engine config by different paths, and the \
         handshake's first line is the path as spelled"
    } else {
        return Ok(());
    };
    Err(refuse(format!(
        "its two seats attest different engines: they differ at `{differing}`, so a capture over \
         it would carry labels from two teachers without saying so"
    )))
}

/// Ask one position and return the engine's own two lines.
fn ask(
    channel: &mut Channel,
    position: &str,
    go: &str,
    timeout_ms: u64,
    game: usize,
    k: usize,
) -> Result<(String, String), ArenaError> {
    let where_ = || format!("game {game}, turn {k}");
    if let Some(stray) = channel.unsolicited() {
        return Err(refuse(format!(
            "{}: the engine spoke before it was asked ({stray:?})",
            where_()
        )));
    }
    for line in [pistol_cli::protocol::NEW_GAME, position, go] {
        if channel.send(line).is_err() {
            return Err(refuse(format!("{}: the engine closed its input", where_())));
        }
    }
    let mut totals: Option<String> = None;
    loop {
        match channel.receive(timeout_ms, game, u32::try_from(k).unwrap_or(u32::MAX))? {
            Received::Closed => {
                return Err(refuse(format!("{}: the engine closed its pipe", where_())));
            }
            Received::Overlong => {
                return Err(refuse(format!(
                    "{}: the engine wrote more than {} bytes with no newline, which is not a line",
                    where_(),
                    crate::channel::MAX_LINE_BYTES
                )));
            }
            Received::Line(line) => {
                if line.starts_with(&format!("{} ", pistol_cli::report::BESTMOVE_PREFIX)) {
                    let totals = totals.ok_or_else(|| {
                        refuse(format!(
                            "{}: the search closed with no totals line this driver recognised",
                            where_()
                        ))
                    })?;
                    return Ok((totals, line));
                }
                if line.starts_with(&format!("{} ", pistol_cli::report::ERROR_PREFIX)) {
                    return Err(refuse(format!(
                        "{}: the engine refused: `{line}`",
                        where_()
                    )));
                }
                if exchange::totals_of(&line).is_some() {
                    totals = Some(line);
                    continue;
                }
                if line.starts_with(&format!("{} ", pistol_cli::report::INFO_PREFIX)) {
                    continue;
                }
                return Err(refuse(format!(
                    "{}: the engine wrote `{line}`, which is not a line this protocol has",
                    where_()
                )));
            }
        }
    }
}

/// Walk one report, asking every asked position on one channel.
///
/// # Errors
/// Any failure refuses the WHOLE run: a capture that silently omits positions is
/// a corpus whose gaps are invisible to its consumer.
pub fn run(transcript: &Transcript, label_nodes: u64) -> Result<Vec<CaptureRecord>, ArenaError> {
    one_engine(transcript)?;
    crate::replay::verify_engines(transcript)?;
    let go = label_go_line(label_nodes);
    let seats = [Seat {
        section: &transcript.engines[0],
        identity: &transcript.identities[0],
    }];
    seats::with_seats(&seats, transcript.hang_timeout_ms, |channels| {
        let mut out: Vec<CaptureRecord> = Vec::new();
        for game in &transcript.games {
            for k in asked_prefixes(game)? {
                let position = position_line(&game.moves[..k]);
                let (totals, bestmove) = ask(
                    &mut channels[0],
                    &position,
                    &go,
                    transcript.hang_timeout_ms,
                    game.index,
                    k,
                )?;
                let record = CaptureRecord {
                    game: game.index,
                    turns_played: k,
                    position,
                    totals: normalise(&totals)?,
                    bestmove,
                };
                no_tab(&record)?;
                out.push(record);
            }
        }
        Ok(out)
    })
}

/// A field carrying a TAB would destroy the record's arity, and the record is
/// built from bytes an engine chose.
fn no_tab(record: &CaptureRecord) -> Result<(), ArenaError> {
    for (name, field) in [
        ("position", &record.position),
        ("totals", &record.totals),
        ("bestmove", &record.bestmove),
    ] {
        if field.contains('\t') {
            return Err(refuse(format!(
                "game {}, turn {}: the `{name}` field carries a TAB, which this record's arity \
                 cannot survive",
                record.game, record.turns_played
            )));
        }
    }
    Ok(())
}
