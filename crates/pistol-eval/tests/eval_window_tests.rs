//! What a length-six window is worth, and which windows exist at all.
//!
//! These are the tests that pin D-11's scoring rule itself: three axes, six
//! window offsets through a cell, a window holding both colours is dead, and on
//! an unbounded board only a window that holds a stone can score. Each
//! expectation is written as an expression in the weight table rather than as a
//! number, so the tests pin the rule and not the operator's current values —
//! except where a full enumeration is spelled out in a comment, which is the
//! point of that test.

mod common;

use std::collections::BTreeSet;

use common::reference::{value_by_region_scan, value_from_scratch};
use common::{built, committed_weights, line};
use pistol_core::{Axis, Color, Coord};
use pistol_eval::{EVAL_MAX, Eval, WINDOW_LEN, WINDOWS_PER_CELL, Window, windows_through};

#[test]
fn eval_single_stone_scores_every_window_through_it() {
    let weights = committed_weights();
    let (board, eval) = built(&weights, &[(Coord::ORIGIN, Color::Black)]);

    // Three axes, six offsets: eighteen windows hold that stone and nothing
    // else, and each is worth one stone's table entry to black.
    assert_eq!(WINDOWS_PER_CELL, 18, "3 axes x 6 offsets through a cell");
    let expected =
        i32::try_from(WINDOWS_PER_CELL).expect("eighteen fits") * weights.window_value(1);
    assert_eq!(eval.value(Color::Black), expected);
    assert_eq!(eval.value(Color::White), -expected);
    assert_eq!(
        eval.value(Color::Black),
        value_from_scratch(&board, &weights, Color::Black)
    );
    assert_eq!(
        eval.value(Color::Black),
        value_by_region_scan(&board, &weights, Color::Black),
        "a region scan finds no other window worth anything"
    );
}

#[test]
fn eval_windows_through_a_cell_are_the_ones_that_hold_it() {
    // The enumeration the incremental update rests on: every window it visits
    // holds the stone, there are eighteen of them, and they are distinct.
    for cell in [Coord::ORIGIN, Coord::new(-4, 9), Coord::new(31, -17)] {
        let windows: Vec<Window> = windows_through(cell).collect();
        assert_eq!(windows.len(), WINDOWS_PER_CELL, "windows through {cell}");
        assert_eq!(
            windows.iter().collect::<BTreeSet<&Window>>().len(),
            WINDOWS_PER_CELL,
            "windows through {cell} must be distinct"
        );
        for window in windows {
            assert!(
                window.cells().contains(&cell),
                "{window:?} does not hold {cell}"
            );
            assert_eq!(window.cells().len(), WINDOW_LEN as usize);
        }
    }
}

#[test]
fn eval_mixed_window_is_dead() {
    let weights = committed_weights();
    let (one, two, three) = (
        weights.window_value(1),
        weights.window_value(2),
        weights.window_value(3),
    );

    // Three cells of one ConstQ line — q = 0, r = 0, 1, 2 — so the three stones
    // share that line and sit on three distinct ConstR and three distinct ConstS
    // lines. Windows along the shared line are named by their start r below.
    let (a, b, c) = (Coord::new(0, 0), Coord::new(0, 1), Coord::new(0, 2));

    // Two black stones. Along the shared line: starts -5 and 1 hold one stone,
    // starts -4..=0 hold both. Along the other two axes each stone is alone in
    // its line, six windows each.
    let (pair_board, pair) = built(&weights, &[(a, Color::Black), (b, Color::Black)]);
    let pair_expected = 2 * one + 5 * two + 2 * 6 * one + 2 * 6 * one;
    assert_eq!(pair.value(Color::Black), pair_expected);
    assert_eq!(
        pair.value(Color::Black),
        value_from_scratch(&pair_board, &weights, Color::Black)
    );

    // Now a white stone on the third cell. Every window that held both black
    // stones — starts -4, -3, -2, -1, 0 — now holds a white one too, and so do
    // the windows that held only the nearer black stone; five windows worth
    // 4 * two + one between them go dead. White brings its own: one window on
    // the shared line (start 2) and six on each of its other two axes.
    let (mixed_board, mixed) = built(
        &weights,
        &[(a, Color::Black), (b, Color::Black), (c, Color::White)],
    );
    let killed = 4 * two + one;
    let whites_own = one + 6 * one + 6 * one;
    assert_eq!(
        mixed.value(Color::Black),
        pair_expected - killed - whites_own,
        "a window holding both colours contributes nothing to either side"
    );
    // The same total, computed straight from the enumeration: on the shared line
    // a surviving window of two black stones and one of a lone black stone
    // against one lone white window, and on each of the other two axes twelve
    // black single-stone windows against six white ones.
    assert_eq!(mixed.value(Color::Black), two + 2 * (12 * one - 6 * one));
    assert_eq!(
        mixed.value(Color::Black),
        value_from_scratch(&mixed_board, &weights, Color::Black)
    );
    assert_eq!(
        mixed.value(Color::Black),
        value_by_region_scan(&mixed_board, &weights, Color::Black)
    );

    // The control: the same geometry with the third stone black instead. Nothing
    // is dead, the shared line holds four windows of three, and the position is
    // worth far more than the mixed one.
    let (_, all_black) = built(
        &weights,
        &[(a, Color::Black), (b, Color::Black), (c, Color::Black)],
    );
    assert_eq!(
        all_black.value(Color::Black),
        2 * one + 2 * two + 4 * three + 3 * 6 * one + 3 * 6 * one
    );
    assert!(
        all_black.value(Color::Black) > mixed.value(Color::Black),
        "blocking with white must be worth something"
    );
}

#[test]
fn eval_only_windows_touching_a_stone_can_score() {
    // The incremental eval and the cheap reference share one assumption: on an
    // unbounded board, only a window holding a stone can score. A scan of the
    // whole region around the stones is the independent statement of it.
    let weights = committed_weights();
    let positions: Vec<Vec<(Coord, Color)>> = vec![
        Vec::new(),
        vec![(Coord::new(-3, 7), Color::White)],
        line(Coord::new(0, 0), Axis::ConstS, 4, Color::Black),
        vec![
            (Coord::new(0, 0), Color::Black),
            (Coord::new(1, 0), Color::White),
            (Coord::new(0, 1), Color::Black),
            (Coord::new(2, -1), Color::White),
            (Coord::new(-1, 3), Color::Black),
            (Coord::new(4, -4), Color::White),
        ],
    ];

    for (index, stones) in positions.iter().enumerate() {
        let (board, eval) = built(&weights, stones);
        for side in [Color::Black, Color::White] {
            let scanned = value_by_region_scan(&board, &weights, side);
            assert_eq!(
                value_from_scratch(&board, &weights, side),
                scanned,
                "position {index}: the stone-driven reference disagrees with a region scan"
            );
            assert_eq!(
                eval.value(side),
                scanned,
                "position {index}: the incremental value disagrees with a region scan"
            );
        }
    }
}

#[test]
fn eval_saturates_at_the_eval_band_and_never_reaches_the_mate_band() {
    // A completed line is a win, and a win's score is the search's mate band,
    // never eval's (docs/decisions.md D-3, D-63). The most a static value may
    // say is the top of the eval band.
    let weights = committed_weights();
    let stones = line(Coord::ORIGIN, Axis::ConstR, 30, Color::Black);
    let (board, eval) = built(&weights, &stones);

    assert_eq!(eval.value(Color::Black), EVAL_MAX);
    assert_eq!(eval.value(Color::White), -EVAL_MAX);
    assert_eq!(
        eval.value(Color::Black),
        value_from_scratch(&board, &weights, Color::Black)
    );
}

#[test]
fn eval_of_an_empty_position_is_zero() {
    let weights = committed_weights();
    let (board, eval) = built(&weights, &[]);
    for side in [Color::Black, Color::White] {
        assert_eq!(eval.value(side), 0);
        assert_eq!(value_from_scratch(&board, &weights, side), 0);
    }
}
