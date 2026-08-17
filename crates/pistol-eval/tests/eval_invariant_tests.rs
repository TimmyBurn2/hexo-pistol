//! Adversarial inputs on the two paths that can be handed something impossible:
//! a caller whose stones contradict what the eval holds, and a cell at the edge
//! of the addressable lattice.
//!
//! Neither is operator input. A cell that may not hold a stone is refused by
//! `pistol_core::GameState` long before an eval hears about it, so being told
//! about one anyway means a caller's board and its eval have drifted — a bug in
//! pistol, reported as a named panic rather than as an error nobody could handle
//! (CLAUDE.md rule 3).
//!
//! The checks below are *opportunistic*, and the tests say which case each one
//! catches. The guarantee that the bookkeeping is right is
//! `eval_incremental_matches_from_scratch_on_random_playouts`, not these; these
//! are here so that the cheap cross-checks the update already affords are loud
//! rather than absent (docs/decisions.md D-70).

mod common;

use common::reference::value_from_scratch;
use common::{built, committed_weights};
use pistol_core::{Axis, Color, Coord};
use pistol_eval::{Eval, HandcraftedV0};

#[test]
#[should_panic(expected = "EVAL_DESYNC")]
fn eval_apply_that_overfills_a_window_panics() {
    // Six cells of one line fill a window; a seventh stone in it can only come
    // from a cell applied twice, which is a caller whose board says otherwise.
    let weights = committed_weights();
    let mut eval = HandcraftedV0::new(weights);
    for step in 0..6 {
        eval.apply(Coord::ORIGIN.step(Axis::ConstQ, step), Color::Black);
    }
    eval.apply(Coord::new(0, 3), Color::Black);
}

#[test]
#[should_panic(expected = "EVAL_DESYNC")]
fn eval_undo_of_a_colour_the_window_never_held_panics() {
    let weights = committed_weights();
    let mut eval = HandcraftedV0::new(weights);
    eval.apply(Coord::ORIGIN, Color::Black);
    eval.undo(Coord::ORIGIN, Color::White);
}

#[test]
#[should_panic(expected = "EVAL_DESYNC")]
fn eval_undo_of_a_cell_that_holds_nothing_panics() {
    let weights = committed_weights();
    let mut eval = HandcraftedV0::new(weights);
    eval.apply(Coord::ORIGIN, Color::Black);
    eval.undo(Coord::new(20, -20), Color::Black);
}

#[test]
fn eval_windows_stop_at_the_edge_of_the_addressable_lattice() {
    // A window that would need a cell outside the `i16` lattice is not a window
    // (docs/decisions.md D-47, D-67). No game reaches these cells — the edge is
    // thousands of turns of legal-region growth away — but an eval that panicked
    // there would be a crash the type system invites, so it is checked.
    let weights = committed_weights();
    let corners = [
        (Coord::new(i16::MAX, 0), Color::Black),
        (Coord::new(i16::MIN, 0), Color::White),
        (Coord::new(0, i16::MAX), Color::Black),
        (Coord::new(i16::MIN, i16::MAX), Color::White),
        (Coord::new(i16::MAX, i16::MIN), Color::Black),
    ];

    let (board, eval) = built(&weights, &corners);
    for side in [Color::Black, Color::White] {
        assert_eq!(
            eval.value(side),
            value_from_scratch(&board, &weights, side),
            "a stone at the lattice edge is worth what a recompute says, for {side}"
        );
    }

    // Fewer windows hold a corner cell than hold an interior one, and the eval
    // has to take back exactly the ones it created.
    let mut unwound = eval;
    for &(at, color) in corners.iter().rev() {
        unwound.undo(at, color);
    }
    assert_eq!(
        unwound,
        HandcraftedV0::new(weights),
        "an edge position must unwind to a fresh eval too"
    );
}
