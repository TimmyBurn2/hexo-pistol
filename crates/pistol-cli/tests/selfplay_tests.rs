mod common;

use common::{GATE, engine, only_line};
use pistol_cli::Session;
use pistol_core::{GameState, Outcome, Turn};

/// How many turns the game is allowed to run for. A match may impose a cap; the
/// engine never treats one as a game rule (rule 6).
const TURN_CAP: u32 = 16;

/// What the engine is given for each turn. A depth budget, so the game is the
/// same game on every machine.
const BUDGET: &str = "go depth_turns 1";

#[test]
fn cli_selfplay_game_is_fully_legal() {
    let mut engine = engine(GATE);
    let mut session = Session::new(&mut engine);
    let mut referee = GameState::new_game();
    let mut played: Vec<Turn> = Vec::new();

    let outcome = loop {
        if referee.turn() > TURN_CAP {
            break None;
        }
        let answers = say(&mut session, &position_line(&played));
        assert!(
            answers.is_empty(),
            "the position was refused after {} turns: {answers:?}\nmoves: {}",
            played.len(),
            tokens(&played)
        );

        let answers = say(&mut session, BUDGET);
        assert!(
            !answers.iter().any(|line| line.starts_with("error ")),
            "the engine refused to move on turn {}: {answers:?}",
            referee.turn()
        );
        let best = only_line(&answers, "bestmove");
        let token = best
            .split_whitespace()
            .nth(1)
            .unwrap_or_else(|| panic!("a bestmove line carries a turn: {best}"));
        let turn: Turn = token
            .parse()
            .unwrap_or_else(|error| panic!("`{token}` is not a turn token: {error}"));

        // The referee is pistol-core, and it is the only judge of legality
        // (CLAUDE.md rule 2). A turn it refuses fails the test here.
        let result = referee.make_turn(turn).unwrap_or_else(|error| {
            panic!(
                "the engine played an illegal turn {turn} on turn {}: {error}\nmoves: {}",
                referee.turn(),
                tokens(&played)
            )
        });
        played.push(turn);

        // The engine's own view of the game must be the referee's.
        assert_eq!(
            session.engine().state().board().stone_count() + turn.stone_count() as usize,
            referee.board().stone_count(),
            "the engine was standing on a different position than the referee"
        );

        if let Outcome::Win { winner, turn } = result {
            break Some((winner, turn));
        }
    };

    // Both endings are legitimate. What is asserted is that the game was legal all
    // the way, that it made progress, and that a decided game is decided on the
    // turn the rules say it is.
    assert!(
        played.len() >= 2,
        "the game should have gone somewhere: {}",
        tokens(&played)
    );
    match outcome {
        Some((winner, turn)) => {
            assert_eq!(
                referee.outcome(),
                Outcome::Win { winner, turn },
                "the referee agrees the game is over"
            );
            assert_eq!(
                turn as usize,
                played.len(),
                "sudden death is scored in turns, and the win is on the last one played"
            );
            // Rule 4: a win completes the instant a stone forms six, so the
            // winning turn is one stone if the first stone did it.
            let last = *played.last().expect("a played turn");
            assert!(
                pistol_core::wins_at(referee.board(), last.second().unwrap_or(last.first()))
                    || pistol_core::wins_at(referee.board(), last.first()),
                "the winning turn's own stone completes the line: {last}"
            );
            // And the engine refuses to stand on the finished game.
            let answers = say(&mut session, &position_line(&played));
            assert!(
                answers.len() == 1 && answers[0].starts_with("error IllegalPosition"),
                "a won position is terminal: {answers:?}"
            );
        }
        None => {
            assert_eq!(referee.outcome(), Outcome::Ongoing);
            assert_eq!(
                referee.turn(),
                TURN_CAP + 1,
                "the cap is what ended it, not anything else"
            );
        }
    }
}

/// Say one line, and collect the answers.
fn say(session: &mut Session<'_>, line: &str) -> Vec<String> {
    let mut answers = Vec::new();
    session.line(line, &mut |answer| answers.push(answer.to_string()));
    answers
}

/// The `position` line for a game so far. The move list is the canonical encoding
/// of a position (docs/decisions.md D-6), so the engine is told the whole game
/// each turn rather than a diff — which is also what an arena will do.
fn position_line(played: &[Turn]) -> String {
    if played.is_empty() {
        return String::from("position start");
    }
    format!("position start moves {}", tokens(played))
}

/// A move list as turn tokens.
fn tokens(played: &[Turn]) -> String {
    played
        .iter()
        .map(Turn::to_string)
        .collect::<Vec<String>>()
        .join(" ")
}
