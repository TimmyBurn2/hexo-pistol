//! `replay-check` — the second instrument: replay every transcript a match
//! wrote, stone by stone, and confirm the record agrees with the rules.
//!
//! RULE9-JUSTIFICATION: the per-kind replay semantics the review round
//! demanded — frame checks (mover, turn), stone-count checks against the
//! rules' own owed count, count-vs-place illegality, first_stone_win — are
//! one contract over one pistol-core state machine; splitting them per kind
//! would duplicate the frame block and the placement helpers each branch
//! leans on.
//!
//! The referee and this tool share pistol-core deliberately: the rules are
//! not the stage under doubt. The stage under doubt is the RECORD — whether
//! what was written to disk is the game that was played. A transcript that
//! replays to the same outcome, turn count and winner is evidence about the
//! record that no in-run check can give, because the in-run check is the
//! thing being checked.
//!
//! What is checked, per recorded turn, against the replay's own pistol-core
//! state: the mover is the side the rules have to move; the turn number is
//! the turn the rules are at; the stone count matches what the rules say the
//! turn owed (a `continue` submits exactly the owed stones, an `incomplete`
//! fewer, an over-submission is illegal BY COUNT — the referee never places
//! a stone past the owed count, so neither does the replay); a win is
//! decided by one of the record's own stones at the recorded turn, seat and
//! `first_stone_win` flag (true iff the deciding stone was the first of a
//! two-stone turn); an illegal-by-place stone is refused by the rules at
//! exactly the recorded cell. The residual — a record whose stones differ
//! from the game's but stay self-consistently legal — is inherent to any
//! replay and is what the rules-not-under-doubt registration owns.
//!
//! Usage: replay-check <artifacts-dir>
//! Exit:  0 every game replayed to its recorded outcome;
//!        1 a game replayed to something else (named, with the file);
//!        2 refused (no transcripts, unreadable, malformed).

use std::path::Path;

use pistol_core::{Coord, GameState, Outcome, Player};

fn main() -> std::process::ExitCode {
    let owned: Vec<String> = std::env::args().skip(1).collect();
    if owned.len() != 1 || owned[0].starts_with('-') {
        eprintln!("usage: replay-check <artifacts-dir>");
        return std::process::ExitCode::from(2);
    }
    let dir = Path::new(&owned[0]);
    if !dir.is_dir() {
        eprintln!("replay-check: REFUSED: no directory at {}", dir.display());
        return std::process::ExitCode::from(2);
    }
    let mut transcripts: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .filter(|name| {
                    name.starts_with('g')
                        && name.ends_with(".jsonl")
                        && name[1..]
                            .trim_end_matches(".jsonl")
                            .bytes()
                            .all(|byte| byte.is_ascii_digit())
                })
                .collect()
        })
        .unwrap_or_default();
    if transcripts.is_empty() {
        eprintln!(
            "replay-check: REFUSED: no gNNN.jsonl transcripts in {}",
            dir.display()
        );
        return std::process::ExitCode::from(2);
    }
    transcripts.sort();
    let mut failures = 0usize;
    for name in &transcripts {
        if let Err(why) = replay_one(&dir.join(name)) {
            eprintln!("replay-check: FAIL: {name}: {why}");
            failures += 1;
        }
    }
    if failures > 0 {
        eprintln!(
            "replay-check: {failures} of {} transcript(s) disagree with the rules",
            transcripts.len()
        );
        return std::process::ExitCode::from(1);
    }
    println!(
        "replay-check: {} transcript(s) replayed to their recorded outcomes",
        transcripts.len()
    );
    std::process::ExitCode::SUCCESS
}

/// Replay one transcript file.
fn replay_one(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut state = GameState::new_game();
    // The server-played opening: p1's turn-1 stone at the origin.
    state
        .place(Coord::new(0, 0))
        .map_err(|error| format!("the opening stone was refused: {error}"))?;
    let mut recorded_end: Option<(String, String)> = None;
    let mut turn_records = 0usize;
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let entry: serde_json::Value =
            serde_json::from_str(line).map_err(|error| format!("line is not JSON: {error}"))?;
        match entry["event"].as_str() {
            Some("game_start") => {}
            Some("turn") => {
                turn_records += 1;
                replay_turn(&mut state, &entry)?;
            }
            Some("game_end") => {
                recorded_end = Some((
                    entry["kind"].as_str().unwrap_or("?").to_string(),
                    entry["detail"].as_str().unwrap_or("").to_string(),
                ));
            }
            other => return Err(format!("unknown event {other:?}")),
        }
    }
    let (kind, detail) =
        recorded_end.ok_or_else(|| "the transcript never recorded a game_end".to_string())?;
    match kind.as_str() {
        "win" => {
            let Outcome::Win { winner, turn } = state.outcome() else {
                return Err(format!(
                    "game_end says win ({detail}) but the rules left the game undecided"
                ));
            };
            let seat = seat_of(winner);
            if !detail.contains(&format!("winner {seat}")) || !detail.contains(&format!("turn {turn}")) {
                return Err(format!(
                    "game_end says \"{detail}\"; the rules say winner {seat} at turn {turn}"
                ));
            }
        }
        "capped" => {
            if state.outcome().is_decided() {
                return Err(format!(
                    "game_end says capped but the rules decided the game: {:?}",
                    state.outcome()
                ));
            }
        }
        "forfeit" => {
            // A forfeit freezes the record mid-flight; the turns the record
            // holds are what replay checks, and they all held.
        }
        other => return Err(format!("unknown game_end kind {other:?}")),
    }
    if turn_records == 0 {
        return Err("no turns recorded".to_string());
    }
    Ok(())
}

/// One recorded turn, replayed with the referee's own semantics.
fn replay_turn(state: &mut GameState, entry: &serde_json::Value) -> Result<(), String> {
    let stones = entry["stones"]
        .as_array()
        .ok_or_else(|| "turn without stones".to_string())?;
    let coords: Vec<Coord> = stones
        .iter()
        .map(coord_of)
        .collect::<Result<_, _>>()?;
    let kind = entry["outcome"]["kind"]
        .as_str()
        .ok_or_else(|| "turn without an outcome kind".to_string())?;

    // THE FRAME: the record's mover and turn number must be the rules' own.
    // An engine-failure record is the one exception on mover — the referee
    // records the seat that FAILED, which at a pregame spawn failure is not
    // the side to move.
    if kind != "engine_failure" {
        let mover = entry["mover"]
            .as_str()
            .ok_or_else(|| "turn without a mover".to_string())?;
        if mover != seat_of(state.to_move()) {
            return Err(format!(
                "record says mover {mover}; the rules have {} to move",
                seat_of(state.to_move())
            ));
        }
    }
    let turn = entry["turn"]
        .as_u64()
        .ok_or_else(|| "turn without a number".to_string())? as u32;
    if turn != state.turn() {
        return Err(format!(
            "record says turn {turn}; the rules are at turn {}",
            state.turn()
        ));
    }
    let owed = state.stones_owed();

    match kind {
        "continue" => {
            if coords.len() != owed as usize {
                return Err(format!(
                    "record says continue with {} stones; the turn owed {owed}",
                    coords.len()
                ));
            }
            place_all_without_winning(state, &coords)
        }
        "incomplete" => {
            if coords.len() >= owed as usize {
                return Err(format!(
                    "record says incomplete with {} stones; the turn owed {owed}",
                    coords.len()
                ));
            }
            place_all_without_winning(state, &coords)
        }
        "win" => {
            // Apply stones in submitted order until the rules decide; the
            // referee never places a stone past the owed count, and neither
            // does this.
            let mut deciding = None;
            for (index, at) in coords.iter().enumerate().take(owed as usize) {
                state
                    .place(*at)
                    .map_err(|error| format!("stone {at} was refused by the rules: {error}"))?;
                if let Outcome::Win { winner, turn } = state.outcome() {
                    deciding = Some((index, winner, turn));
                    break;
                }
            }
            let Some((index, winner, rules_turn)) = deciding else {
                return Err(
                    "record says win but no submitted stone decided the game".to_string()
                );
            };
            let seat = seat_of(winner);
            if entry["outcome"]["winner"] != seat {
                return Err(format!(
                    "record says winner {} but the rules say {seat}",
                    entry["outcome"]["winner"]
                ));
            }
            let recorded_turn = entry["outcome"]["turn"].as_u64().unwrap_or(u64::MAX) as u32;
            if recorded_turn != rules_turn {
                return Err(format!(
                    "record says the win is at turn {recorded_turn} but the rules say {rules_turn}"
                ));
            }
            let recorded_fsw = entry["outcome"]["first_stone_win"]
                .as_bool()
                .ok_or_else(|| "win without first_stone_win".to_string())?;
            let expected_fsw = owed == 2 && index == 0;
            if recorded_fsw != expected_fsw {
                return Err(format!(
                    "record says first_stone_win {recorded_fsw}; the deciding stone was \
                     index {index} of a turn owed {owed}, so it is {expected_fsw}"
                ));
            }
            Ok(())
        }
        "illegal" => {
            let recorded = coord_of(&entry["outcome"]["stone"])
                .map_err(|why| format!("the recorded illegal stone is malformed: {why}"))?;
            // The referee classifies by WHICH SUBMITTED STONE FAILED FIRST —
            // walking the stones, the count boundary is checked before each
            // placement and a refused cell fires wherever it sits — so a
            // three-stone submission whose second cell is occupied is BY
            // PLACE at that cell, and one whose first two cells are legal is
            // BY COUNT at the third. The replay walks the same walk: it must
            // stop at exactly the stone and for exactly the reason the
            // referee's record names, or agree with a record the referee
            // would never write.
            let mut stopped: Option<(Coord, bool)> = None;
            for (index, at) in coords.iter().enumerate() {
                if index >= owed as usize {
                    stopped = Some((*at, true));
                    break;
                }
                match state.place(*at) {
                    Err(_) => {
                        stopped = Some((*at, false));
                        break;
                    }
                    Ok(_) => {
                        if state.outcome().is_decided() {
                            return Err(format!(
                                "stone {at} decided the game; the record would have said \
                                 win, not illegal"
                            ));
                        }
                    }
                }
            }
            let Some((stone, by_count)) = stopped else {
                return Err(
                    "record says illegal but every submitted stone was accepted".to_string()
                );
            };
            if recorded != stone {
                return Err(format!(
                    "record names {recorded} as the illegal stone; replaying the submitted \
                     stones stops at {stone} ({})",
                    if by_count {
                        "past the owed count".to_string()
                    } else {
                        "refused by the rules".to_string()
                    }
                ));
            }
            Ok(())
        }
        "engine_failure" => {
            if !coords.is_empty() {
                return Err("an engine-failure record carries stones".to_string());
            }
            Ok(())
        }
        other => Err(format!("unknown turn outcome kind {other:?}")),
    }
}

/// Place every stone; any refusal or win is a record the referee never
/// would have written (it records `illegal` or `win` instead).
fn place_all_without_winning(state: &mut GameState, coords: &[Coord]) -> Result<(), String> {
    for at in coords {
        state
            .place(*at)
            .map_err(|error| format!("stone {at} was refused by the rules: {error}"))?;
        if state.outcome().is_decided() {
            return Err(format!(
                "stone {at} decided the game; the record would have said win, not this"
            ));
        }
    }
    Ok(())
}

/// A recorded stone as a `Coord`.
fn coord_of(stone: &serde_json::Value) -> Result<Coord, String> {
    let pair = stone
        .as_array()
        .filter(|pair| pair.len() == 2)
        .ok_or_else(|| format!("a stone is not a [q, r] pair: {stone}"))?;
    let q = pair[0].as_i64().ok_or("q is not an integer")?;
    let r = pair[1].as_i64().ok_or("r is not an integer")?;
    let q = i16::try_from(q).map_err(|_| format!("q out of range: {q}"))?;
    let r = i16::try_from(r).map_err(|_| format!("r out of range: {r}"))?;
    Ok(Coord::new(q, r))
}

/// p1/p2 for a player.
fn seat_of(player: Player) -> &'static str {
    if player == Player::P1 {
        "p1"
    } else {
        "p2"
    }
}
