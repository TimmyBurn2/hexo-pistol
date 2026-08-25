//! `replay-check` — the second instrument: replay every transcript a match
//! wrote, stone by stone, and confirm the record agrees with the rules.
//!
//! The referee and this tool share pistol-core deliberately: the rules are
//! not the stage under doubt. The stage under doubt is the RECORD — whether
//! what was written to disk (stone order, turn boundaries, the win the
//! referee claims) is the game that was played. A transcript that replays to
//! the same outcome, turn count and winner is evidence about the record that
//! no in-run check can give, because the in-run check is the thing being
//! checked.
//!
//! Replay semantics per recorded outcome, mirroring the referee's own:
//! `continue` turns apply all their stones; a `win` turn applies stones only
//! until the rules decide the game (a first-stone win's second submitted
//! stone is never applied); an `illegal` turn applies its prefix and must be
//! refused exactly at the recorded stone; `incomplete` applies all its
//! stones; `engine_failure` records carry none.
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
    match entry["outcome"]["kind"].as_str() {
        Some("continue") | Some("incomplete") => {
            for at in &coords {
                state
                    .place(*at)
                    .map_err(|error| format!("stone {at} was refused by the rules: {error}"))?;
            }
            Ok(())
        }
        Some("win") => {
            for at in &coords {
                state
                    .place(*at)
                    .map_err(|error| format!("stone {at} was refused by the rules: {error}"))?;
                if let Outcome::Win { winner, turn } = state.outcome() {
                    check_win_fields(entry, winner, turn)?;
                    // The instant a stone completes a line the game is over;
                    // any further submitted stone was never played.
                    return Ok(());
                }
            }
            Err("turn records a win but no stone of it decided the game".to_string())
        }
        Some("illegal") => {
            let illegal = coord_of(&entry["outcome"]["stone"])
                .map_err(|why| format!("the recorded illegal stone is malformed: {why}"))?;
            for at in &coords {
                if *at == illegal {
                    return match state.place(*at) {
                        Err(_) => Ok(()), // refused exactly where the record says
                        Ok(_) => Err(format!(
                            "the record calls {at} illegal but the rules accepted it"
                        )),
                    };
                }
                state
                    .place(*at)
                    .map_err(|error| format!("stone {at} was refused by the rules: {error}"))?;
            }
            Err(format!("the recorded illegal stone {illegal} was never submitted"))
        }
        Some("engine_failure") => Ok(()),
        other => Err(format!("unknown turn outcome kind {other:?}")),
    }
}

/// The win fields a recorded turn claims must be the ones the rules reached.
fn check_win_fields(
    entry: &serde_json::Value,
    winner: Player,
    turn: u32,
) -> Result<(), String> {
    let seat = seat_of(winner);
    if entry["outcome"]["winner"] != seat {
        return Err(format!(
            "turn records winner {} but the rules say {seat}",
            entry["outcome"]["winner"]
        ));
    }
    let recorded_turn = entry["outcome"]["turn"].as_u64().unwrap_or(u64::MAX) as u32;
    if recorded_turn != turn {
        return Err(format!(
            "turn records the win at turn {recorded_turn} but the rules say {turn}"
        ));
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
