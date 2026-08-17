//! What the value *means*: it is side-relative, and the two colours are treated
//! exactly alike.
//!
//! Both properties are load bearing for a search. Side-relative is what lets a
//! negamax search negate a child's value instead of tracking whose number it is,
//! and colour symmetry is what stops the engine from playing better as one
//! colour than the other — a bug that no strength measurement over paired
//! openings would attribute correctly (CLAUDE.md rule 6).

mod common;

use common::reference::value_from_scratch;
use common::{built, committed_weights};
use pistol_core::{Color, Coord};
use pistol_eval::Eval;

/// A position with a lopsided black advantage, a blocked line, and stones on all
/// three axes — enough shape that a symmetry test has something to say.
fn lopsided() -> Vec<(Coord, Color)> {
    vec![
        (Coord::new(0, 0), Color::Black),
        (Coord::new(0, 1), Color::Black),
        (Coord::new(0, 2), Color::Black),
        (Coord::new(1, 0), Color::Black),
        (Coord::new(2, -1), Color::Black),
        (Coord::new(0, 3), Color::White),
        (Coord::new(3, -3), Color::White),
        (Coord::new(-2, 1), Color::White),
    ]
}

#[test]
fn eval_antisymmetric_under_color_swap() {
    let weights = committed_weights();
    let stones = lopsided();
    let swapped: Vec<(Coord, Color)> = stones
        .iter()
        .map(|&(at, color)| (at, color.opponent()))
        .collect();

    let (board, eval) = built(&weights, &stones);
    let (swapped_board, swapped_eval) = built(&weights, &swapped);

    assert_ne!(
        eval.value(Color::Black),
        0,
        "a symmetry test on a balanced position asserts nothing"
    );
    for side in [Color::Black, Color::White] {
        assert_eq!(
            eval.value(side),
            -swapped_eval.value(side),
            "swapping every stone's colour must negate the value for {side}"
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

    assert!(
        eval.value(Color::Black) > 0,
        "black is ahead in this position"
    );
    assert_eq!(
        eval.value(Color::Black),
        -eval.value(Color::White),
        "one position, two readings of the same number"
    );
    for side in [Color::Black, Color::White] {
        assert_eq!(eval.value(side), value_from_scratch(&board, &weights, side));
    }
}
