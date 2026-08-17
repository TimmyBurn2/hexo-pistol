//! The turn structure (game rules 3 and 4).
//!
//! One stone on turn 1, two on every turn after it, by the same side. A stone
//! that completes a line ends the game where it stands: the turn does not
//! advance, the second stone of that turn is never played, and the turn number
//! the win is recorded on is the one that was being played — that number is
//! what a mate distance is measured in (docs/decisions.md D-3).
//!
//! Taking a turn back is the other half of the same machine, and it is tested
//! in `undo_tests.rs`.

mod common;

use common::games::golden_game;
use pistol_core::{Coord, CoreError, GameState, Outcome, Phase, Player, PlyOutcome};

/// The scripted games live in the sha-pinned golden-game fixture, not in an
/// array in this file: rule 4's truncation is one of the things D-12 promises a
/// pinned fixture covers, and a script that can drift pins nothing.
const FIRST_STONE_WIN: &str = "p1_wins_on_the_first_stone_of_its_turn";
const SECOND_STONE_WIN: &str = "p1_wins_on_the_second_stone_of_its_turn";

#[test]
fn first_turn_is_one_stone_at_the_origin() {
    let mut game = GameState::new_game();
    assert_eq!(game.to_move(), Player::P1);
    assert_eq!(game.turn(), 1);
    assert_eq!(game.phase(), Phase::First);
    assert_eq!(game.stones_owed(), 1, "one stone on turn 1");
    assert_eq!(game.outcome(), Outcome::Ongoing);

    assert_eq!(
        game.place(Coord::new(1, 0)),
        Err(CoreError::FirstStoneNotAtOrigin {
            at: Coord::new(1, 0)
        })
    );
    assert_eq!(game.board().stone_count(), 0, "a refusal places nothing");

    assert_eq!(game.place(Coord::ORIGIN), Ok(PlyOutcome::TurnComplete));
    assert_eq!(game.turn(), 2, "one stone ends the first turn");
    assert_eq!(game.to_move(), Player::P2);
    assert_eq!(game.phase(), Phase::First);
    assert_eq!(game.stones_owed(), 2);
}

#[test]
fn later_turns_are_two_stones_by_the_same_side() {
    let mut game = GameState::new_game();
    game.place(Coord::ORIGIN).expect("turn 1");

    assert_eq!(game.place(Coord::new(1, 0)), Ok(PlyOutcome::TurnContinues));
    assert_eq!(game.to_move(), Player::P2, "still p2's turn");
    assert_eq!(game.turn(), 2);
    assert_eq!(game.phase(), Phase::Second);
    assert_eq!(game.stones_owed(), 1);

    assert_eq!(game.place(Coord::new(2, 0)), Ok(PlyOutcome::TurnComplete));
    assert_eq!(game.to_move(), Player::P1);
    assert_eq!(game.turn(), 3);
    assert_eq!(game.phase(), Phase::First);
    assert_eq!(game.stones_owed(), 2);

    assert_eq!(
        game.board().get(Coord::new(1, 0)),
        Some(Player::P2),
        "both stones of the turn are p2"
    );
    assert_eq!(game.board().get(Coord::new(2, 0)), Some(Player::P2));
}

#[test]
fn win_on_first_stone_ends_turn_second_not_played() {
    let script = golden_game(FIRST_STONE_WIN);
    let (prefix, winning) = script.split_last();
    let mut game = GameState::from_plies(prefix).expect("a legal game");
    assert_eq!(game.turn(), 7);
    assert_eq!(game.to_move(), Player::P1);
    assert_eq!(game.phase(), Phase::First);
    assert_eq!(game.outcome(), Outcome::Ongoing);
    let stones_before = game.board().stone_count();
    assert_eq!(
        game.place(winning),
        Ok(PlyOutcome::Win {
            winner: Player::P1,
            turn: 7
        })
    );

    assert_eq!(
        game.outcome(),
        Outcome::Win {
            winner: Player::P1,
            turn: 7
        },
        "the win is scored on the turn it completed on"
    );
    assert_eq!(game.turn(), 7, "the turn does not advance past the win");
    assert_eq!(game.phase(), Phase::First, "the turn ended on this stone");
    assert_eq!(
        game.to_move(),
        Player::P1,
        "the state freezes on the completing stone: the mover does not change"
    );
    assert_eq!(game.stones_owed(), 0, "the second stone is not owed");

    // Rule 4: the second stone of the turn is never played. Not skipped — there
    // is no way to play it.
    let second = Coord::new(6, 0);
    assert_eq!(
        game.place(second),
        Err(CoreError::GameDecided {
            winner: Player::P1,
            turn: 7
        })
    );
    assert_eq!(
        game.board().stone_count(),
        stones_before + 1,
        "exactly one stone was added by the winning turn"
    );
    assert_eq!(game.board().get(second), None);
}

#[test]
fn win_on_second_stone_detected() {
    let script = golden_game(SECOND_STONE_WIN);
    let (prefix, winning) = script.split_last();
    let (first_stone_of_turn, opening) = prefix.split_last().expect("a scripted game");
    let mut game = GameState::from_plies(opening).expect("a legal game");
    assert_eq!(game.turn(), 7);
    assert_eq!(game.to_move(), Player::P1);

    assert_eq!(
        game.place(*first_stone_of_turn),
        Ok(PlyOutcome::TurnContinues),
        "five in a row is not a win"
    );
    assert_eq!(game.phase(), Phase::Second);

    assert_eq!(
        game.place(winning),
        Ok(PlyOutcome::Win {
            winner: Player::P1,
            turn: 7
        })
    );
    assert_eq!(
        game.outcome(),
        Outcome::Win {
            winner: Player::P1,
            turn: 7
        },
        "the same turn number as the first stone of the turn, not the next one"
    );
    assert_eq!(game.turn(), 7);
    assert_eq!(
        game.to_move(),
        Player::P1,
        "the state freezes on the completing stone"
    );
    assert_eq!(game.stones_owed(), 0);
}

#[test]
fn a_stone_is_refused_on_an_occupied_cell() {
    let mut game = GameState::new_game();
    game.place(Coord::ORIGIN).expect("turn 1");
    assert_eq!(
        game.place(Coord::ORIGIN),
        Err(CoreError::OccupiedCell { at: Coord::ORIGIN })
    );
    assert_eq!(game.board().stone_count(), 1);
    assert_eq!(game.turn(), 2, "a refusal advances nothing");
    assert_eq!(game.phase(), Phase::First);
}

#[test]
fn played_lists_every_stone_in_play_order() {
    let script = golden_game(FIRST_STONE_WIN);
    let game = GameState::from_plies(&script.plies).expect("a legal game");
    let played: Vec<Coord> = game.played().map(|(at, _)| at).collect();
    assert_eq!(played, script.plies, "the move list is the position");

    let players: Vec<Player> = game.played().map(|(_, player)| player).collect();
    assert_eq!(
        players[..3],
        [Player::P1, Player::P2, Player::P2],
        "one stone on turn 1, two on turn 2"
    );
}

#[test]
fn phase_index_is_zero_then_one() {
    // WP-04 puts this index in the zobrist key; it is not free to drift.
    assert_eq!(Phase::First.index(), 0);
    assert_eq!(Phase::Second.index(), 1);
}

#[test]
fn from_plies_refuses_a_game_that_continues_past_a_win() {
    let mut script = golden_game(FIRST_STONE_WIN).plies; // ends on the winning stone
    script.push(Coord::new(6, 0)); // one stone too many
    assert_eq!(
        GameState::from_plies(&script),
        Err(CoreError::GameDecided {
            winner: Player::P1,
            turn: 7
        })
    );
}
