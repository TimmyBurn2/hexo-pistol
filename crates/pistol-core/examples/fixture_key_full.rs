use std::process::ExitCode;

use pistol_core::{Coord, GameState, Player, canonical_form};

/// The empty position's field, spelled as the corpus spells it.
const EMPTY_FIELD: &str = "-";

fn state_of(body: &str) -> Result<GameState, String> {
    let mut words = body.split_whitespace();
    if words.next() != Some("start") || words.next() != Some("moves") {
        return Err(format!("not a `start moves` entry: {body}"));
    }
    let mut state = GameState::new_game();
    for turn in words {
        for cell in turn.split('/') {
            let coord: Coord = cell.parse().map_err(|why| format!("{cell}: {why}"))?;
            state.place(coord).map_err(|why| format!("{cell}: {why}"))?;
        }
    }
    Ok(state)
}

fn render(stones: &[(Coord, Player)]) -> String {
    if stones.is_empty() {
        return EMPTY_FIELD.to_string();
    }
    stones
        .iter()
        .map(|(at, player)| format!("{at}:{}", player.name()))
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("fixture_key_full: a fixture path is required");
        return ExitCode::FAILURE;
    };
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(why) => {
            eprintln!("fixture_key_full: cannot read {path}: {why}");
            return ExitCode::FAILURE;
        }
    };
    for (number, line) in text.lines().enumerate() {
        let body = line.split('#').next().unwrap_or("");
        if body.trim().is_empty() {
            continue;
        }
        let state = match state_of(body) {
            Ok(state) => state,
            Err(why) => {
                eprintln!("fixture_key_full: line {number}: {why}");
                return ExitCode::FAILURE;
            }
        };
        let stones: Vec<(Coord, Player)> = state.board().stones().collect();
        println!("{}", render(&canonical_form(&stones)));
    }
    ExitCode::SUCCESS
}
