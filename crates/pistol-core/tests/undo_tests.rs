mod common;

use common::games::golden_game;
use pistol_core::{Coord, CoreError, GameState, Outcome, Phase, Player, PlyOutcome};

#[test]
fn undo_across_a_turn_boundary_restores_side_phase_and_turn() {
    let mut game = GameState::new_game();
    game.place(Coord::ORIGIN).expect("turn 1");
    game.place(Coord::new(1, 0)).expect("turn 2, first stone");
    game.place(Coord::new(2, 0)).expect("turn 2, second stone");
    game.place(Coord::new(0, 1)).expect("turn 3, first stone");

    assert_eq!(game.turn(), 3);
    assert_eq!(game.to_move(), Player::P1);
    assert_eq!(game.phase(), Phase::Second);

    assert_eq!(game.undo(), Ok(Coord::new(0, 1)));
    assert_eq!(game.turn(), 3, "back to the start of turn 3");
    assert_eq!(game.to_move(), Player::P1);
    assert_eq!(game.phase(), Phase::First);

    // Across the boundary: back into the middle of P2's turn 2.
    assert_eq!(game.undo(), Ok(Coord::new(2, 0)));
    assert_eq!(game.turn(), 2);
    assert_eq!(game.to_move(), Player::P2);
    assert_eq!(game.phase(), Phase::Second);
    assert_eq!(game.board().stone_count(), 2);

    assert_eq!(game.undo(), Ok(Coord::new(1, 0)));
    assert_eq!(game.turn(), 2);
    assert_eq!(game.phase(), Phase::First);
    assert_eq!(game.undo(), Ok(Coord::ORIGIN));
    assert_eq!(game, GameState::new_game(), "back to a new game");
}

#[test]
fn undo_of_a_winning_stone_reopens_the_game() {
    let script = golden_game("p1_wins_on_the_first_stone_of_its_turn");
    let (prefix, winning) = script.split_last();
    let mut game = GameState::from_plies(prefix).expect("a legal game");
    game.place(winning).expect("the winning stone");
    assert!(game.outcome().is_decided());

    assert_eq!(game.undo(), Ok(winning));
    assert_eq!(game.outcome(), Outcome::Ongoing, "the game is open again");
    assert_eq!(game.turn(), 7);
    assert_eq!(game.to_move(), Player::P1);
    assert_eq!(game.phase(), Phase::First);
    assert_eq!(game.stones_owed(), 2);
    assert_eq!(game, GameState::from_plies(prefix).expect("a legal game"));

    // And the position really is playable again — a decided flag left standing
    // would refuse this.
    assert_eq!(
        game.place(Coord::new(1, 1)),
        Ok(PlyOutcome::TurnContinues),
        "a different stone plays normally"
    );
}

#[test]
fn undo_on_empty_history_is_nothing_to_undo() {
    let mut game = GameState::new_game();
    assert_eq!(game.undo(), Err(CoreError::NothingToUndo));
    assert_eq!(game, GameState::new_game());

    game.place(Coord::ORIGIN).expect("turn 1");
    game.undo().expect("the only stone");
    assert_eq!(game.undo(), Err(CoreError::NothingToUndo));
}

#[test]
fn place_undo_roundtrip_reproduces_the_identical_state() {
    let script = golden_game("p1_wins_on_the_first_stone_of_its_turn");
    let (plies, winning) = script.split_last();
    let mut game = GameState::new_game();
    for (index, &at) in plies.iter().enumerate() {
        let before = game.clone();
        game.place(at).expect("a legal game");
        game.undo().expect("the stone just placed");
        assert_eq!(game, before, "round trip at ply {index} ({at})");
        game.place(at).expect("a legal game");
    }

    let before_win = game.clone();
    game.place(winning).expect("the winning stone");
    game.undo().expect("the winning stone");
    assert_eq!(game, before_win, "round trip across the winning stone");
}
