mod common;

use common::reference::value_from_scratch;
use common::{built, committed_weights};
use pistol_core::{Axis, Coord, Player};
use pistol_eval::{Eval, HandcraftedV0};

#[test]
#[should_panic(expected = "EVAL_DESYNC")]
fn eval_apply_that_overfills_a_window_panics() {
    // Six cells of one line fill a window; a seventh stone in it can only come
    // from a cell applied twice, which is a caller whose board says otherwise.
    let weights = committed_weights();
    let mut eval = HandcraftedV0::new(weights);
    for step in 0..6 {
        eval.apply(Coord::ORIGIN.step(Axis::ConstQ, step), Player::P1);
    }
    eval.apply(Coord::new(0, 3), Player::P1);
}

#[test]
#[should_panic(expected = "EVAL_DESYNC")]
fn eval_undo_of_a_player_the_window_never_held_panics() {
    let weights = committed_weights();
    let mut eval = HandcraftedV0::new(weights);
    eval.apply(Coord::ORIGIN, Player::P1);
    eval.undo(Coord::ORIGIN, Player::P2);
}

#[test]
#[should_panic(expected = "EVAL_DESYNC")]
fn eval_undo_of_a_cell_that_holds_nothing_panics() {
    let weights = committed_weights();
    let mut eval = HandcraftedV0::new(weights);
    eval.apply(Coord::ORIGIN, Player::P1);
    eval.undo(Coord::new(20, -20), Player::P1);
}

#[test]
fn eval_windows_stop_at_the_edge_of_the_addressable_lattice() {
    // A window that would need a cell outside the `i16` lattice is not a window
    // (docs/decisions.md D-47, D-67). No game reaches these cells — the edge is
    // thousands of turns of legal-region growth away — but an eval that panicked
    // there would be a crash the type system invites, so it is checked.
    let weights = committed_weights();
    let corners = [
        (Coord::new(i16::MAX, 0), Player::P1),
        (Coord::new(i16::MIN, 0), Player::P2),
        (Coord::new(0, i16::MAX), Player::P1),
        (Coord::new(i16::MIN, i16::MAX), Player::P2),
        (Coord::new(i16::MAX, i16::MIN), Player::P1),
    ];

    let (board, eval) = built(&weights, &corners);
    for side in [Player::P1, Player::P2] {
        assert_eq!(
            eval.value(side),
            value_from_scratch(&board, &weights, side),
            "a stone at the lattice edge is worth what a recompute says, for {side}"
        );
    }

    // Fewer windows hold a corner cell than hold an interior one, and the eval
    // has to take back exactly the ones it created.
    let mut unwound = eval;
    for &(at, player) in corners.iter().rev() {
        unwound.undo(at, player);
    }
    assert_eq!(
        unwound,
        HandcraftedV0::new(weights),
        "an edge position must unwind to a fresh eval too"
    );
}
