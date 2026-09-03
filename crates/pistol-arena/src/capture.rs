use std::fmt::Write as _;

use pistol_core::{GameState, Outcome, Turn};
use pistol_engine::CensusRequest;

use crate::channel::{Channel, Received};
use crate::error::ArenaError;
use crate::exchange;
use crate::label_cache::{CaptureCounts, LabelCache, Memo};
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
///
/// THE CENSUS TOKEN IS APPENDED HERE and not in
/// [`crate::config::BudgetSection::go_line`], which spells a budget for every
/// pass this crate has: the token is a property of the CAPTURE ask, and a
/// budget section that carried it would put it on pass 1's `go` lines too.
///
/// A census-on capture therefore has a different `capture_sha256` from the
/// otherwise identical census-off one, because the digest covers this line
/// ([`capture_sha256`]). That is correct — it is a different instrument — and
/// it is said out loud because a tranche registered against one digest is not
/// a tranche run under the other.
pub fn label_go_line(nodes: u64, census: CensusRequest) -> String {
    let budget = crate::config::BudgetSection::Nodes { value: nodes }
        .go_line()
        .unwrap_or_else(|| unreachable!("a nodes budget always spells a go line"));
    match census {
        CensusRequest::Off => budget,
        CensusRequest::On => format!("{budget} {}", pistol_cli::budget_token::CENSUS_TOKEN),
    }
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

/// What a line the engine wrote means to the capture pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// The closing report: keep it, keep reading.
    Totals,
    /// A trigger-census row, carried whole. KEPT, and that is the whole point:
    /// it begins `info `, so the catch-all below would throw it away and a
    /// census-on capture would write zero census bytes at exit 0
    /// (docs/experiments/wp20b_design.md §3.1).
    Census(String),
    /// A per-depth report: not the answer and not an error.
    Ignore,
    /// Nothing this pass can proceed from, with the reason.
    Refuse(String),
}

/// Classify one line the engine wrote, `bestmove` excepted.
///
/// A pure function so the refusals INVARIANT 9 promises can be pinned without an
/// engine that plays a whole report honestly and then misbehaves — a shape no
/// static stub behaviour has, because the capture verifies its engine against
/// the identity the report attests.
pub fn classify(line: &str) -> Step {
    if line.starts_with(&format!("{} ", pistol_cli::report::ERROR_PREFIX)) {
        return Step::Refuse(format!("the engine refused: `{line}`"));
    }
    if exchange::totals_of(line).is_some() {
        return Step::Totals;
    }
    // BEFORE the `info` catch-all, which a census row would otherwise match.
    if line.starts_with(&census_prefix()) {
        return Step::Census(line.to_string());
    }
    if line.starts_with(&format!("{} ", pistol_cli::report::INFO_PREFIX)) {
        return Step::Ignore;
    }
    Step::Refuse(format!(
        "the engine wrote `{line}`, which is not a line this protocol has"
    ))
}

/// Where a capture's census rows go, and whether it asked for any.
///
/// The two travel together because they are one decision: a row arriving when
/// `request` is `Off` is a protocol deviation rather than something to file,
/// and a sink that carried only the vector could not tell the difference.
pub struct CensusSink<'a> {
    /// What the `go` line asked for.
    pub request: CensusRequest,
    /// The rows, in the order the engine wrote them.
    pub rows: &'a mut Vec<String>,
}

/// The prefix every census row on the wire carries.
fn census_prefix() -> String {
    format!(
        "{} {} ",
        pistol_cli::report::INFO_PREFIX,
        pistol_cli::report::CENSUS_MARKER
    )
}

/// Ask one position and return the engine's own two lines.
///
/// `census` is a SINK rather than a changed return type, so this function's
/// `Result<(String, String), _>` contract and every existing caller stay as
/// they are. Rows are appended in the order the engine wrote them.
fn ask(
    channel: &mut Channel,
    position: &str,
    go: &str,
    timeout_ms: u64,
    game: usize,
    k: usize,
    census: &mut CensusSink,
) -> Result<(String, String), ArenaError> {
    let where_ = || format!("game {game}, turn {k}");
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
                match classify(&line) {
                    Step::Totals => {
                        totals = Some(line);
                        continue;
                    }
                    Step::Census(row) => {
                        // OFF THE TOKEN THE BLOCK DOES NOT EXIST — the design's
                        // §4 says so in those words, and it is what makes the
                        // gate-off byte-identity obligation satisfiable. A
                        // census row on an ask that did not request one is a
                        // protocol deviation by the engine, refused by name
                        // rather than collected into a vector this pass then
                        // drops (CLAUDE.md rule 3).
                        if census.request == CensusRequest::Off {
                            return Err(refuse(format!(
                                "{}: the engine wrote a census row for a `{}` that did not ask \
                                 for one: `{row}`",
                                where_(),
                                pistol_cli::budget_token::CENSUS_TOKEN
                            )));
                        }
                        // THE SAME ARITY GUARD EVERY OTHER CAPTURED FIELD PASSES.
                        // A census row's own grammar is whitespace-delimited, so a
                        // TAB splits like a space rather than shifting a column —
                        // but this row reaches an artifact a committed manifest
                        // indexes, and it came from bytes an engine chose. §3
                        // requires the two seats to attest ONE engine, not to be
                        // `pistol`, so it is checked rather than assumed.
                        if row.contains('\t') {
                            return Err(refuse(format!(
                                "{}: a census row carries a TAB, which this artifact's arity \
                                 cannot survive: `{row}`",
                                where_()
                            )));
                        }
                        census.rows.push(row);
                        continue;
                    }
                    Step::Ignore => continue,
                    Step::Refuse(why) => return Err(refuse(format!("{}: {why}", where_()))),
                }
            }
        }
    }
}

/// Refuse a census under the label cache, naming both words.
///
/// A hit performs no search and emits no census row, so a cached census
/// capture would write fewer rows than positions asked — indistinguishable
/// from a quiet search. The CLI refuses the spelling; this is the same refusal
/// at the seam where the damage would be done, because `run` is `pub`.
fn refuse_census_under_cache(census: CensusRequest, cache: LabelCache) -> Result<(), ArenaError> {
    if census == CensusRequest::On && cache == LabelCache::On {
        return Err(refuse(
            "`--label-cache` with `--census`: a cache hit performs no search and emits no census row, so a \
             cached census capture would under-report firings at exit 0",
        ));
    }
    Ok(())
}

/// Walk one report, asking every asked position on one channel.
///
/// Under [`LabelCache::On`] a prefix whose `position` line this run has already
/// asked takes the pair it already has and the engine is not asked; every
/// record is still built, in order, from the same normalised strings, so the
/// file is byte-identical to the uncached pass's.
///
/// # Errors
/// Any failure refuses the WHOLE run: a capture that silently omits positions is
/// a corpus whose gaps are invisible to its consumer.
pub fn run(
    transcript: &Transcript,
    label_nodes: u64,
    census: &mut CensusSink,
    cache: LabelCache,
) -> Result<(Vec<CaptureRecord>, CaptureCounts), ArenaError> {
    refuse_census_under_cache(census.request, cache)?;
    one_engine(transcript)?;
    crate::replay::verify_engines(transcript)?;
    let go = label_go_line(label_nodes, census.request);
    let seats = [Seat {
        section: &transcript.engines[0],
        identity: &transcript.identities[0],
    }];
    seats::with_seats(&seats, transcript.hang_timeout_ms, |channels| {
        let mut memo = Memo::new(cache);
        let mut out: Vec<CaptureRecord> = Vec::new();
        for game in &transcript.games {
            for k in asked_prefixes(game)? {
                // BEFORE the lookup, at every prefix: a guard that ran only on
                // the prefixes that ask would leave a stray in the pipe at a
                // hit to be read as the answer to a LATER miss, or never.
                if let Some(stray) = channels[0].unsolicited() {
                    return Err(refuse(format!(
                        "game {}, turn {k}: the engine spoke before it was asked ({stray:?})",
                        game.index
                    )));
                }
                let position = position_line(&game.moves[..k]);
                let (totals, bestmove) = match memo.lookup(&position) {
                    Some(pair) => pair,
                    None => {
                        memo.asked();
                        let (totals, bestmove) = ask(
                            &mut channels[0],
                            &position,
                            &go,
                            transcript.hang_timeout_ms,
                            game.index,
                            k,
                            census,
                        )?;
                        let pair = (normalise(&totals)?, bestmove);
                        memo.insert(&position, &game.moves[..k], pair.clone());
                        pair
                    }
                };
                let record = CaptureRecord {
                    game: game.index,
                    turns_played: k,
                    position,
                    totals,
                    bestmove,
                };
                no_tab(&record)?;
                memo.recorded();
                out.push(record);
            }
        }
        Ok((out, memo.into_counts()))
    })
}

/// A field carrying a TAB would destroy the record's arity, and the record is
/// built from bytes an engine chose.
///
/// # Errors
/// Naming the field and the position it was captured at. No engine in this tree
/// can emit a TAB, so this is checked rather than assumed: §3 requires the two
/// seats to attest ONE engine, not to be `pistol`.
pub fn no_tab(record: &CaptureRecord) -> Result<(), ArenaError> {
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
