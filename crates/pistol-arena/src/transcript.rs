//! Reading an arena report back as the run it describes.
//!
//! # Why this exists at all
//!
//! The replay mode is handed a REPORT, not a config: it must re-drive the exact
//! engines, at the exact budget, over the exact games some earlier run produced.
//! Everything it needs is already on the report's own face — `report.rs` writes
//! the engine binaries and their digests, the budget, the opening length and
//! every game's move list — and a report is the only artefact that attests all
//! of it together. A replay parameterised by a config document instead could be
//! pointed at a different experiment than the one it claims to be checking.
//!
//! Named here because it was the one component D-406 found missing from the
//! design outright (its MAJOR 5), and a component nobody names is a component
//! nobody reviews.
//!
//! # Everything is refused by name
//!
//! This parses a document to decide whether a strength claim may be read, so
//! nothing here skips, defaults or guesses. A report that does not carry a
//! verdict, a schema this build does not write, a budget that is not
//! reproducible, a path with whitespace in it, a repeated key on one record, a
//! move token that is not a turn, a game index that is not where it should be —
//! each is its own refusal, with its own sentence.
//!
//! # RULE9-JUSTIFICATION: a parser's refusals and the record shapes they refuse
//! against are one artefact. The field readers exist only to make the record
//! readers refuse by name, and splitting them apart would put half of every
//! refusal in another file — which is the arrangement that lets a shape drift
//! away from the sentence that was supposed to guard it.

use std::path::PathBuf;

use pistol_core::{GameState, Outcome, Turn};

use crate::config::EngineSection;
use crate::error::ArenaError;
use crate::identity::EngineIdentity;
use crate::report::{REPORT_KIND, REPORT_SCHEMA};

/// One game as the report recorded it.
#[derive(Debug, Clone)]
pub struct RecordedGame {
    /// Its index in the run, which is also its place in the report.
    pub index: usize,
    /// Which opening it started from, window-relative (docs/decisions.md D-202).
    pub opening: usize,
    /// Whether engine A held the first seat.
    pub a_is_p1: bool,
    /// Whether a forfeit ended it.
    pub forfeit: bool,
    /// The whole game, opening included.
    pub moves: Vec<Turn>,
    /// What each engine spent on it, indexed `0` for A.
    pub nodes: [u64; 2],
}

/// An arena report, read back.
#[derive(Debug, Clone)]
pub struct Transcript {
    /// Per slot, `0` is engine A: enough to spawn it again.
    pub engines: [EngineSection; 2],
    /// Per slot, who that engine was — the run's own capture, so a replay
    /// verifies against the ORIGINAL run and not merely against itself.
    pub identities: [EngineIdentity; 2],
    /// The `go` line, already spelled, at the run's own budget.
    pub go_line: String,
    /// That budget's node count, for the record.
    pub budget_nodes: u64,
    /// How many turns of every game are book.
    pub opening_turns: u32,
    /// The horizon the run played to.
    pub turn_cap: u32,
    /// The watchdog the run used, off the timing block — a run mechanic, which
    /// is exactly why it lives there and why a replay needs it anyway.
    pub hang_timeout_ms: u64,
    /// The experiment this report is of, by content.
    pub experiment_sha256: String,
    /// The digest of the whole report file, so a consumer can bind the replay
    /// it produces to the report it was taken from.
    pub source_sha256: String,
    /// The games, in index order.
    pub games: Vec<RecordedGame>,
}

/// A refusal naming the report and what was wrong with it.
fn refuse(what: impl Into<String>) -> ArenaError {
    ArenaError::config("replay report", what.into())
}

/// The single line starting with `key `, or a refusal.
fn one_line<'a>(text: &'a str, key: &str) -> Result<&'a str, ArenaError> {
    let prefix = format!("{key} ");
    // `split('\n')` and not `lines()`: an engine's verbatim refusal is free text
    // this format copies through unquoted, and two notions of "line" over one
    // document is how such text injects a record.
    let mut found = text
        .split('\n')
        .filter_map(|line| line.strip_prefix(&prefix));
    let first = found
        .next()
        .ok_or_else(|| refuse(format!("it carries no `{key}` record")))?;
    if found.next().is_some() {
        return Err(refuse(format!(
            "it carries more than one `{key}` record, so there is no one answer to read"
        )));
    }
    Ok(first)
}

/// `a 1 b 2` as a map, refusing a repeated key.
///
/// ONE SPELLING PER NUMBER: a repeated key read last-wins would let a human
/// read the first `result` and this read the second.
fn keyed(record: &str) -> Result<Vec<(&str, &str)>, ArenaError> {
    let words: Vec<&str> = record.split_whitespace().collect();
    if !words.len().is_multiple_of(2) {
        return Err(refuse(format!(
            "`{record}` is not a sequence of key-value pairs"
        )));
    }
    let mut out: Vec<(&str, &str)> = Vec::with_capacity(words.len() / 2);
    for pair in words.chunks_exact(2) {
        if out.iter().any(|(key, _)| *key == pair[0]) {
            return Err(refuse(format!(
                "the key `{}` appears twice on `{record}`",
                pair[0]
            )));
        }
        out.push((pair[0], pair[1]));
    }
    Ok(out)
}

/// One value out of a keyed record.
fn value<'a>(
    fields: &[(&'a str, &'a str)],
    key: &str,
    record: &str,
) -> Result<&'a str, ArenaError> {
    fields
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, held)| *held)
        .ok_or_else(|| refuse(format!("`{record}` carries no `{key}`")))
}

/// A number, refusing a spelling that is not the one this format writes.
fn number<T: std::str::FromStr>(word: &str, what: &str) -> Result<T, ArenaError> {
    word.parse::<T>()
        .map_err(|_| refuse(format!("`{word}` is not a {what}")))
}

/// A path off a report line, refusing one this format cannot round-trip.
fn path(word: &str, what: &str) -> Result<PathBuf, ArenaError> {
    // The format is whitespace-delimited and does not quote, so a path with a
    // space in it was written unrecoverably. Refused rather than misparsed: the
    // wrong half of a path is a different binary.
    if word.is_empty() {
        return Err(refuse(format!("the {what} path is empty")));
    }
    Ok(PathBuf::from(word))
}

/// Read a report.
pub fn read(text: &str, source_sha256: String) -> Result<Transcript, ArenaError> {
    let head = text
        .split('\n')
        .next()
        .ok_or_else(|| refuse("it is empty"))?;
    let mut head_words = head.split_whitespace();
    match head_words.next() {
        Some(kind) if kind == REPORT_KIND => {}
        Some(kind) => {
            return Err(refuse(format!(
                "its first token is `{kind}` and not `{REPORT_KIND}`, so it carries no verdict and \
                 its games are explicitly not a sample — there is nothing here a replay could \
                 make into a measurement"
            )));
        }
        None => return Err(refuse("its first line is blank")),
    }
    let schema: u32 = number(head_words.next().unwrap_or(""), "report schema version")?;
    if schema != REPORT_SCHEMA {
        return Err(refuse(format!(
            "it is schema {schema} and this build writes and reads schema {REPORT_SCHEMA}; the \
             fields a replay needs are not promised to mean the same thing across the two"
        )));
    }

    let budget = keyed(one_line(text, "budget")?)?;
    let (kind, held) = *budget
        .first()
        .ok_or_else(|| refuse("the `budget` record is empty"))?;
    if kind != "nodes" {
        return Err(refuse(format!(
            "the run used a `{kind}` budget and only a `nodes` budget replays: the whole premise \
             is that a re-driven engine answers what it answered, which wall-clock does not \
             promise (CLAUDE.md rule 4)"
        )));
    }
    let budget_nodes: u64 = number(held, "node count")?;

    let opening_turns: u32 = number(one_line(text, "opening_turns")?.trim(), "turn count")?;
    let turn_cap: u32 = number(one_line(text, "turn_cap")?.trim(), "turn cap")?;
    let experiment_sha256 = one_line(text, "experiment_sha256")?.trim().to_string();
    // Matched on TWO words, because `report.rs` writes two records whose first
    // word is `timing` and a reader that took either would be reading whichever
    // one came first. What `one_line` hands back therefore begins with
    // `n_workers`'s own VALUE, so the key is put back before the record is read
    // as the key-value sequence it is.
    let timing_record = format!("n_workers {}", one_line(text, "timing n_workers")?);
    let timing = keyed(&timing_record)?;
    let hang_timeout_ms: u64 = number(
        value(&timing, "hang_timeout_ms", &timing_record)?,
        "millisecond count",
    )?;

    let (engines, identities) = read_engines(text)?;
    if engines[0].label == engines[1].label {
        return Err(refuse(format!(
            "both seats carry the label `{}`, so no game in it can be attributed to a seat at all",
            engines[0].label
        )));
    }
    let games = read_games(text, &engines[0].label, &engines[1].label)?;

    Ok(Transcript {
        engines,
        identities,
        go_line: format!("{} nodes {budget_nodes}", pistol_cli::protocol::GO),
        budget_nodes,
        opening_turns,
        turn_cap,
        hang_timeout_ms,
        experiment_sha256,
        source_sha256,
        games,
    })
}

/// The two `engine` records and the `engine_id` lines under each.
fn read_engines(text: &str) -> Result<([EngineSection; 2], [EngineIdentity; 2]), ArenaError> {
    let mut sections: Vec<EngineSection> = Vec::with_capacity(2);
    let mut identities: Vec<EngineIdentity> = Vec::with_capacity(2);
    for slot in ["a", "b"] {
        let record = one_line(text, &format!("engine {slot}"))?;
        let fields = keyed(record)?;
        let label = value(&fields, "label", record)?;
        let binary = value(&fields, "binary", record)?;
        let config = value(&fields, "config", record)?;
        // `report.rs` writes six key-value pairs after the two words
        // `one_line` already stripped — label, binary, binary_sha256, config,
        // config_sha256, weights_sha256 — so the record is exactly twelve
        // words. A path carrying whitespace splits into more, and handing back
        // the wrong half of a path is handing back a different binary; this
        // whitespace-delimited format cannot quote, so it refuses instead.
        if record.split_whitespace().count() != 12 {
            return Err(refuse(format!(
                "`engine {slot}` does not carry the six fields this format writes as one word \
                 each; a binary or config path with whitespace in it cannot be read back, and the \
                 record is `{record}`"
            )));
        }
        let id_prefix = format!("engine_id {slot} ");
        let id_lines: Vec<String> = text
            .split('\n')
            .filter_map(|line| line.strip_prefix(&id_prefix))
            .map(str::to_string)
            .collect();
        if id_lines.is_empty() {
            return Err(refuse(format!(
                "seat {slot} has no `engine_id` lines, so there is no identity a respawn could be \
                 verified against"
            )));
        }
        identities.push(EngineIdentity {
            id_lines,
            binary_sha256: value(&fields, "binary_sha256", record)?.to_string(),
            config_sha256: value(&fields, "config_sha256", record)?.to_string(),
            weights_sha256: value(&fields, "weights_sha256", record)?.to_string(),
        });
        sections.push(EngineSection {
            label: label.to_string(),
            binary: path(binary, "engine binary")?,
            binary_sha256: value(&fields, "binary_sha256", record)?.to_string(),
            config: path(config, "engine config")?,
        });
    }
    Ok((
        sections
            .try_into()
            .unwrap_or_else(|_| unreachable!("two slots were read")),
        identities
            .try_into()
            .unwrap_or_else(|_| unreachable!("two slots were read")),
    ))
}

/// Every `game` record and the `moves` record under it.
fn read_games(text: &str, label_a: &str, label_b: &str) -> Result<Vec<RecordedGame>, ArenaError> {
    let mut games: Vec<RecordedGame> = Vec::new();
    for line in text.split('\n') {
        if !line.starts_with("game ") {
            continue;
        }
        // The whole line, key included: `conclusion.rs` writes the game's own
        // index as the value of `game`, so stripping the prefix would leave a
        // record whose first word is a value with no key in front of it.
        let record = line;
        let fields = keyed(record)?;
        let index: usize = number(value(&fields, "game", record)?, "game index")?;
        if index != games.len() {
            return Err(refuse(format!(
                "the report's {}th `game` record is `game {index}`; the records must be the run's \
                 own contiguous prefix or the pairing is not what the report says it is",
                games.len()
            )));
        }
        let p1 = value(&fields, "p1", record)?;
        let p2 = value(&fields, "p2", record)?;
        let a_is_p1 = match (p1 == label_a, p2 == label_b) {
            (true, true) => true,
            (false, false) if p1 == label_b && p2 == label_a => false,
            _ => {
                return Err(refuse(format!(
                    "`game {index}` seats `{p1}` and `{p2}`, which is not the two labels `{label_a}` \
                     and `{label_b}` this report's engines carry"
                )));
            }
        };
        let turns: usize = number(value(&fields, "turns", record)?, "turn count")?;
        let moves = read_moves(text, index, turns)?;
        games.push(RecordedGame {
            index,
            opening: number(value(&fields, "opening", record)?, "opening index")?,
            a_is_p1,
            forfeit: value(&fields, "end", record)? == "forfeit",
            moves,
            nodes: [
                number(value(&fields, "nodes_a", record)?, "node count")?,
                number(value(&fields, "nodes_b", record)?, "node count")?,
            ],
        });
    }
    if games.is_empty() {
        return Err(refuse("it records no games"));
    }
    if !games.len().is_multiple_of(2) {
        return Err(refuse(format!(
            "{} games is not an even number of games, so the pairing every pair-unit claim rests \
             on is undefined",
            games.len()
        )));
    }
    Ok(games)
}

/// The `moves` record for one game, as turns.
fn read_moves(text: &str, index: usize, turns: usize) -> Result<Vec<Turn>, ArenaError> {
    let record = one_line(text, &format!("moves {index}"))?;
    let mut moves: Vec<Turn> = Vec::with_capacity(turns);
    for word in record.split_whitespace() {
        moves.push(
            word.parse::<Turn>()
                .map_err(|why| refuse(format!("game {index}: `{word}` is not a turn: {why}")))?,
        );
    }
    if moves.len() != turns {
        return Err(refuse(format!(
            "game {index} records `turns {turns}` against {} moves",
            moves.len()
        )));
    }
    replays(index, &moves)?;
    Ok(moves)
}

/// A recorded move list must be a game that could have been played.
///
/// Checked HERE, on the document, and not part-way through a replay: the mode
/// promises that a report it cannot answer about costs exit 2 and no document,
/// and a legality refusal discovered at turn nine — after nine searches and two
/// spawned engines — would have already produced part of an answer. It also
/// keeps the two apart in the reader's hands, which is the whole of
/// tools/SHELL_CHECKLIST.md item 12: an illegal recorded move is a corrupt
/// document, never a divergence some engine is responsible for.
///
/// pistol-core is the only judge of legality in this workspace (rule 2).
fn replays(index: usize, moves: &[Turn]) -> Result<(), ArenaError> {
    let mut state = GameState::new_game();
    for (at, turn) in moves.iter().enumerate() {
        let outcome = state.make_turn(*turn).map_err(|error| {
            refuse(format!(
                "game {index}: recorded turn {} (`{turn}`) is not legal in the position the moves \
                 before it reach: {error}",
                at + 1
            ))
        })?;
        if matches!(outcome, Outcome::Win { .. }) && at + 1 != moves.len() {
            return Err(refuse(format!(
                "game {index}: recorded turn {} wins, and {} further turn(s) are recorded after a \
                 game that was already over",
                at + 1,
                moves.len() - at - 1
            )));
        }
    }
    Ok(())
}
