use pistol_core::{Coord, GameState, Player, Turn, canonical_form, canonical_sequence};

use crate::capture_file::CaptureRecord;
use crate::error::ArenaError;
use crate::exchange;
use crate::labels_file::CorpusRecord;
use crate::transcript::Transcript;

/// What a field holds when the value it renders is empty.
///
/// One character no turn token and no cell-colour pair can be, so no field of a
/// record is ever empty and a loader needs no per-column exception
/// (`docs/experiments/wp20s_design.md` §2.10).
pub const EMPTY_FIELD: &str = "-";

fn refuse(what: impl Into<String>) -> ArenaError {
    ArenaError::config("labels", what.into())
}

/// The three spellings the protocol writes a score in, mapped onto names that
/// cannot be misread.
///
/// `cp` becomes `eval` because a column called `cp` will be read as centipawns,
/// and there is no pawn on this board to be a hundredth of
/// (`crates/pistol-cli/src/report.rs:145-158`).
fn score_columns(tag: &str, number: &str) -> Result<(&'static str, i64), ArenaError> {
    let value: i64 = number
        .parse()
        .map_err(|_| refuse(format!("`{number}` is not a score value")))?;
    match tag {
        "cp" => Ok(("eval", value)),
        "mate" => Ok(("mate_in", value)),
        "-mate" => Ok(("mated_in", value)),
        other => Err(refuse(format!(
            "`{other}` is not one of the three score spellings this protocol writes"
        ))),
    }
}

/// The two INDEPENDENT node counters, never the sum the wire prints.
///
/// The solver block is all six fields or none — `render_info` emits them inside
/// one conditional — so any other subset is refused rather than defaulted.
fn node_columns(words: &[&str]) -> Result<(u64, u64), ArenaError> {
    let present = [
        "search_nodes",
        "solver_nodes",
        "solver_firings",
        "solver_invocations",
        "solver_proofs",
        "solver_root_nodes",
    ]
    .iter()
    .filter(|key| exchange::value_of(words, key).is_some())
    .count();
    let nodes = |key: &str| -> Result<u64, ArenaError> {
        exchange::value_of(words, key)
            .ok_or_else(|| refuse(format!("a totals line carries no `{key}`")))?
            .parse()
            .map_err(|_| refuse(format!("`{key}` is not a node count")))
    };
    match present {
        6 => Ok((nodes("search_nodes")?, nodes("solver_nodes")?)),
        // Gate off: `search_nodes` IS `nodes` and `solver_nodes` is zero, which
        // `SearchInfo::search_nodes`'s own doc states and
        // `crates/pistol-search/src/search.rs:513-514` enforces.
        0 => Ok((nodes("nodes")?, 0)),
        other => Err(refuse(format!(
            "a totals line carries {other} of the solver block's six fields; the block is written \
             all or none, so this line was not produced by an engine this build knows"
        ))),
    }
}

/// `q,r:p1` / `q,r:p2` pairs, space-joined in canonical order.
///
/// The colour is spelled, because `Coord`'s own `Display` is the bare cell token
/// and a key rendered as cells would merge two positions over one cell set with
/// different colour partitions — which is the identity §8's denominator counts
/// over.
fn render_key_full(stones: &[(Coord, Player)]) -> String {
    if stones.is_empty() {
        return EMPTY_FIELD.to_string();
    }
    stones
        .iter()
        .map(|(at, player)| format!("{at}:{}", player.name()))
        .collect::<Vec<String>>()
        .join(" ")
}

/// Turn tokens, space-joined.
fn render_turns(turns: &[Turn]) -> String {
    if turns.is_empty() {
        return EMPTY_FIELD.to_string();
    }
    turns
        .iter()
        .map(Turn::to_string)
        .collect::<Vec<String>>()
        .join(" ")
}

/// One corpus record from one capture record and the game it belongs to.
fn one(
    record: &CaptureRecord,
    transcript: &Transcript,
    opening_turns: usize,
) -> Result<CorpusRecord, ArenaError> {
    let game = transcript.games.get(record.game).ok_or_else(|| {
        refuse(format!(
            "record names game {}, which the report does not hold",
            record.game
        ))
    })?;
    if record.turns_played > game.moves.len() {
        return Err(refuse(format!(
            "game {} record names {} turns and the report records {}",
            record.game,
            record.turns_played,
            game.moves.len()
        )));
    }
    let prefix = &game.moves[..record.turns_played];

    // pistol-core is the only judge of legality and of whose stone comes next
    // (rule 2). The replay also supplies the position key and the stones.
    let mut state = GameState::new_game();
    for (at, turn) in prefix.iter().enumerate() {
        state.make_turn(*turn).map_err(|error| {
            refuse(format!(
                "game {}: recorded turn {} is not legal: {error}",
                record.game,
                at + 1
            ))
        })?;
    }
    let sent = if prefix.is_empty() {
        format!("{} start", pistol_cli::protocol::POSITION)
    } else {
        exchange::position_line(prefix)
    };
    if sent != record.position {
        return Err(refuse(format!(
            "game {} turn {}: the captured position is not the report's own prefix",
            record.game, record.turns_played
        )));
    }

    let words = exchange::fields_of(&record.totals).ok_or_else(|| {
        refuse(format!(
            "game {} turn {}: the captured totals line carries no totals marker",
            record.game, record.turns_played
        ))
    })?;
    let tag = exchange::value_of(&words, "score").ok_or_else(|| {
        refuse(format!(
            "game {} turn {}: the totals line carries no score",
            record.game, record.turns_played
        ))
    })?;
    let at = words
        .iter()
        .position(|word| *word == "score")
        .ok_or_else(|| refuse("the score key vanished between two reads"))?;
    let number = words.get(at + 2).ok_or_else(|| {
        refuse(format!(
            "game {} turn {}: no number follows the score tag",
            record.game, record.turns_played
        ))
    })?;
    let (score_kind, score_value) = score_columns(tag, number)?;
    if score_kind != "eval" && score_value < 0 {
        return Err(refuse(format!(
            "game {} turn {}: a `{score_kind}` value is a turn count and cannot be negative",
            record.game, record.turns_played
        )));
    }
    let (search_nodes, solver_nodes) = node_columns(&words)?;
    let depth_turns: u32 = exchange::value_of(&words, "depth_turns")
        .ok_or_else(|| refuse("a totals line carries no `depth_turns`"))?
        .parse()
        .map_err(|_| refuse("`depth_turns` is not a turn count"))?;
    let best = record
        .bestmove
        .strip_prefix(&format!("{} ", pistol_cli::report::BESTMOVE_PREFIX))
        .ok_or_else(|| {
            refuse(format!(
                "game {} turn {}: the captured answer is not a bestmove line",
                record.game, record.turns_played
            ))
        })?
        .trim()
        .to_string();

    let stones: Vec<(Coord, Player)> = state.board().stones().collect();
    Ok(CorpusRecord {
        game: record.game,
        turns_played: record.turns_played,
        moves: render_turns(prefix),
        key_seq: render_turns(&canonical_sequence(prefix)),
        key_pos: state.key().to_string(),
        key_full: render_key_full(&canonical_form(&stones)),
        to_move: state.to_move().name().to_string(),
        score_kind: score_kind.to_string(),
        score_value,
        best,
        depth_turns,
        search_nodes,
        solver_nodes,
        book: record.turns_played <= opening_turns,
        result: game.result.token().to_string(),
        end: if game.forfeit { "forfeit" } else { "normal" }.to_string(),
    })
}

/// The outcome pistol-core derives, checked against the one the report recorded.
///
/// The referent is the arena's RECORDED VERDICT, and the defect class this
/// excludes is a defect in THIS transform's replay and mapping. It is NOT an
/// independent check of pistol-core's win detection: `game.rs` derives `result`
/// from the same `make_turn` this replays with.
fn agrees(transcript: &Transcript) -> Result<(), ArenaError> {
    for game in &transcript.games {
        if game.forfeit {
            continue;
        }
        let mut state = GameState::new_game();
        let mut outcome = pistol_core::Outcome::Ongoing;
        for turn in &game.moves {
            outcome = state
                .make_turn(*turn)
                .map_err(|error| refuse(format!("game {}: {error}", game.index)))?;
        }
        let derived = match outcome {
            pistol_core::Outcome::Win { winner, .. } => match winner {
                Player::P1 => "p1_win",
                Player::P2 => "p2_win",
            },
            pistol_core::Outcome::Ongoing => "capped",
        };
        if derived != game.result.token() {
            return Err(refuse(format!(
                "game {}: its moves reach `{derived}` and the report records `{}`",
                game.index,
                game.result.token()
            )));
        }
    }
    Ok(())
}

/// Turn one capture into the training corpus.
///
/// A pure function of the capture and the report: no process is spawned, no
/// channel opened, no clock read. So a disagreement about what a column MEANS
/// costs a re-run of this transform and never a re-run of the engine.
///
/// # Errors
/// Any failure refuses the WHOLE run, naming the record or the game.
pub fn run(
    capture: &crate::capture_file::Capture,
    transcript: &Transcript,
) -> Result<Vec<CorpusRecord>, ArenaError> {
    if capture.source_sha256 != transcript.source_sha256 {
        return Err(refuse(format!(
            "the capture was taken from a report digesting {} and this one digests {}",
            capture.source_sha256, transcript.source_sha256
        )));
    }
    let expected = crate::capture::capture_sha256(
        &transcript.experiment_sha256,
        &capture.label_go,
        crate::capture_file::CAPTURE_FORMAT_VERSION,
    );
    if capture.capture_sha256 != expected {
        return Err(refuse(format!(
            "the capture's header names identity {} and its own inputs produce {expected}",
            capture.capture_sha256
        )));
    }
    agrees(transcript)?;
    let opening_turns = transcript.opening_turns as usize;
    let records: Vec<CorpusRecord> = capture
        .records
        .iter()
        .map(|record| one(record, transcript, opening_turns))
        .collect::<Result<_, _>>()?;
    for game in &transcript.games {
        if !records.iter().any(|record| record.game == game.index) {
            return Err(refuse(format!(
                "game {} has no record in the capture, and a corpus over some of a report's games \
                 is a corpus over a sample nobody registered",
                game.index
            )));
        }
    }
    Ok(records)
}
