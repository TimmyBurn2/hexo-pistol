use std::process::ExitCode;

use pistol_core::{Coord, GameState, Player};
use pistol_eval::{Eval, HandcraftedV0, Weights};

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

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let (Some(weights_path), Some(fixture)) = (args.next(), args.next()) else {
        eprintln!("static_eval: <weights.toml> <fixture> is required");
        return ExitCode::FAILURE;
    };
    let weights = match Weights::load(std::path::Path::new(&weights_path)) {
        Ok(weights) => weights,
        Err(why) => {
            eprintln!("static_eval: {why}");
            return ExitCode::FAILURE;
        }
    };
    let text = match std::fs::read_to_string(&fixture) {
        Ok(text) => text,
        Err(why) => {
            eprintln!("static_eval: cannot read {fixture}: {why}");
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
                eprintln!("static_eval: line {number}: {why}");
                return ExitCode::FAILURE;
            }
        };
        let mut eval = HandcraftedV0::new(weights.clone());
        for (at, player) in state.board().stones() {
            eval.apply(at, player);
        }
        let to_move = state.to_move();
        println!(
            "{} {}",
            eval.value(to_move),
            match to_move {
                Player::P1 => "p1",
                Player::P2 => "p2",
            }
        );
    }
    ExitCode::SUCCESS
}
