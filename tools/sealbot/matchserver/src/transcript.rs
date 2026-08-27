//! The per-game transcript: one JSON line per judged turn.
//!
//! A game is replayable from its own transcript — every stone, whose it was,
//! what the engine said, and what the referee made of it — which is the whole
//! point of writing it before the match starts rather than reconstructing it
//! after something looks wrong.

use std::io::Write;
use std::path::Path;

use pistol_core::Player;

use crate::referee::{GameSummary, TurnOutcome};

/// Write one game's transcript to `<dir>/g<NNN>.jsonl`.
pub fn write_game(dir: &Path, summary: &GameSummary) -> Result<std::path::PathBuf, String> {
    let path = dir.join(format!("g{:03}.jsonl", summary.game));
    let mut file = std::fs::File::create(&path).map_err(|error| {
        format!("create {}: {error}", path.display())
    })?;
    let mut line = serde_json::to_string(&serde_json::json!({
        "game": summary.game,
        "a_is_p1": summary.a_is_p1,
        "event": "game_start",
        "opening": "server: p1 turn 1 at 0,0 (the platform's standard setup)",
    }))
    .map_err(|error| format!("serialise game_start: {error}"))?;
    line.push('\n');
    file.write_all(line.as_bytes())
        .map_err(|error| format!("write {}: {error}", path.display()))?;
    for record in &summary.turns {
        let entry = serde_json::json!({
            "event": "turn",
            "game": summary.game,
            "turn": record.turn,
            "mover": seat_of(record.mover),
            "engine": record.engine,
            "stones": record
                .stones
                .iter()
                .map(|at| vec![at.q, at.r])
                .collect::<Vec<_>>(),
            "nodes": record.nodes,
            "engine_time_ms": record.engine_time_ms,
            "wall_ms": record.wall_ms,
            "reply": record.raw,
            "outcome": outcome_json(&record.outcome),
        });
        let mut line = serde_json::to_string(&entry)
            .map_err(|error| format!("serialise turn: {error}"))?;
        line.push('\n');
        file.write_all(line.as_bytes())
            .map_err(|error| format!("write {}: {error}", path.display()))?;
    }
    let mut line = serde_json::to_string(&serde_json::json!({
        "event": "game_end",
        "game": summary.game,
        "kind": summary.kind(),
        "detail": crate::report::game_detail(&summary.result),
    }))
    .map_err(|error| format!("serialise game_end: {error}"))?;
    line.push('\n');
    file.write_all(line.as_bytes())
        .map_err(|error| format!("write {}: {error}", path.display()))?;
    file.flush()
        .map_err(|error| format!("flush {}: {error}", path.display()))?;
    Ok(path)
}

/// One outcome as JSON.
fn outcome_json(outcome: &TurnOutcome) -> serde_json::Value {
    match outcome {
        TurnOutcome::Continue => serde_json::json!({"kind": "continue"}),
        TurnOutcome::Win {
            winner,
            turn,
            first_stone_win,
        } => serde_json::json!({
            "kind": "win",
            "winner": seat_of(*winner),
            "turn": turn,
            "first_stone_win": first_stone_win,
        }),
        TurnOutcome::Illegal { stone, why } => serde_json::json!({
            "kind": "illegal",
            "stone": [stone.q, stone.r],
            "why": why,
        }),
        TurnOutcome::Incomplete { submitted, owed } => serde_json::json!({
            "kind": "incomplete",
            "submitted": submitted,
            "owed": owed,
        }),
        TurnOutcome::EngineFailure { detail } => {
            serde_json::json!({"kind": "engine_failure", "detail": detail})
        }
    }
}

/// p1/p2 for a player.
fn seat_of(player: Player) -> &'static str {
    if player == Player::P1 {
        "p1"
    } else {
        "p2"
    }
}
