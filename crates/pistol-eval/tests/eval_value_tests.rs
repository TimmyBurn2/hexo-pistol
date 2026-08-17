//! What the value *means*: it is side-relative, and the two players are treated
//! exactly alike.
//!
//! Both properties are load bearing for a search. Side-relative is what lets a
//! negamax search negate a child's value instead of tracking whose number it is,
//! and player symmetry is what stops the engine from playing better as one
//! player than the other — a bug that no strength measurement over paired
//! openings would attribute correctly (CLAUDE.md rule 6).

mod common;

use common::reference::value_from_scratch;
use common::{built, committed_weights};
use pistol_core::{Coord, Player};
use pistol_eval::Eval;

/// A position with a lopsided P1 advantage, a blocked line, and stones on all
/// three axes — enough shape that a symmetry test has something to say.
fn lopsided() -> Vec<(Coord, Player)> {
    vec![
        (Coord::new(0, 0), Player::P1),
        (Coord::new(0, 1), Player::P1),
        (Coord::new(0, 2), Player::P1),
        (Coord::new(1, 0), Player::P1),
        (Coord::new(2, -1), Player::P1),
        (Coord::new(0, 3), Player::P2),
        (Coord::new(3, -3), Player::P2),
        (Coord::new(-2, 1), Player::P2),
    ]
}

#[test]
fn eval_antisymmetric_under_player_swap() {
    let weights = committed_weights();
    let stones = lopsided();
    let swapped: Vec<(Coord, Player)> = stones
        .iter()
        .map(|&(at, player)| (at, player.opponent()))
        .collect();

    let (board, eval) = built(&weights, &stones);
    let (swapped_board, swapped_eval) = built(&weights, &swapped);

    assert_ne!(
        eval.value(Player::P1),
        0,
        "a symmetry test on a balanced position asserts nothing"
    );
    for side in [Player::P1, Player::P2] {
        assert_eq!(
            eval.value(side),
            -swapped_eval.value(side),
            "swapping every stone's player must negate the value for {side}"
        );
        assert_eq!(
            value_from_scratch(&board, &weights, side),
            -value_from_scratch(&swapped_board, &weights, side),
            "and the reference must agree, for {side}"
        );
    }
}

#[test]
fn eval_value_is_relative_to_the_side_to_move() {
    let weights = committed_weights();
    let (board, eval) = built(&weights, &lopsided());

    assert!(eval.value(Player::P1) > 0, "p1 is ahead in this position");
    assert_eq!(
        eval.value(Player::P1),
        -eval.value(Player::P2),
        "one position, two readings of the same number"
    );
    for side in [Player::P1, Player::P2] {
        assert_eq!(eval.value(side), value_from_scratch(&board, &weights, side));
    }
}
