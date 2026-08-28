mod common;

use std::collections::BTreeSet;

use common::reference::{value_by_region_scan, value_from_scratch};
use common::{built, committed_weights, line};
use pistol_core::{Axis, Coord, Player};
use pistol_eval::{EVAL_MAX, Eval, WINDOW_LEN, WINDOWS_PER_CELL, Window, windows_through};

#[test]
fn eval_single_stone_scores_every_window_through_it() {
    let weights = committed_weights();
    let (board, eval) = built(&weights, &[(Coord::ORIGIN, Player::P1)]);

    // Three axes, six offsets: eighteen windows hold that stone and nothing
    // else, and each is worth one stone's table entry to P1.
    assert_eq!(WINDOWS_PER_CELL, 18, "3 axes x 6 offsets through a cell");
    let expected =
        i32::try_from(WINDOWS_PER_CELL).expect("eighteen fits") * weights.window_value(1);
    assert_eq!(eval.value(Player::P1), expected);
    assert_eq!(eval.value(Player::P2), -expected);
    assert_eq!(
        eval.value(Player::P1),
        value_from_scratch(&board, &weights, Player::P1)
    );
    assert_eq!(
        eval.value(Player::P1),
        value_by_region_scan(&board, &weights, Player::P1),
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

    // Two P1 stones. Along the shared line: starts -5 and 1 hold one stone,
    // starts -4..=0 hold both. Along the other two axes each stone is alone in
    // its line, six windows each.
    let (pair_board, pair) = built(&weights, &[(a, Player::P1), (b, Player::P1)]);
    let pair_expected = 2 * one + 5 * two + 2 * 6 * one + 2 * 6 * one;
    assert_eq!(pair.value(Player::P1), pair_expected);
    assert_eq!(
        pair.value(Player::P1),
        value_from_scratch(&pair_board, &weights, Player::P1)
    );

    // Now a P2 stone on the third cell. Every window that held both P1
    // stones — starts -4, -3, -2, -1, 0 — now holds a P2 one too, and so do
    // the windows that held only the nearer P1 stone; five windows worth
    // 4 * two + one between them go dead. P2 brings its own: one window on
    // the shared line (start 2) and six on each of its other two axes.
    let (mixed_board, mixed) = built(
        &weights,
        &[(a, Player::P1), (b, Player::P1), (c, Player::P2)],
    );
    let killed = 4 * two + one;
    let p2_own = one + 6 * one + 6 * one;
    assert_eq!(
        mixed.value(Player::P1),
        pair_expected - killed - p2_own,
        "a window holding both players contributes nothing to either side"
    );
    // The same total, computed straight from the enumeration: on the shared line
    // a surviving window of two P1 stones and one of a lone P1 stone
    // against one lone P2 window, and on each of the other two axes twelve
    // P1 single-stone windows against six P2 ones.
    assert_eq!(mixed.value(Player::P1), two + 2 * (12 * one - 6 * one));
    assert_eq!(
        mixed.value(Player::P1),
        value_from_scratch(&mixed_board, &weights, Player::P1)
    );
    assert_eq!(
        mixed.value(Player::P1),
        value_by_region_scan(&mixed_board, &weights, Player::P1)
    );

    // The control: the same geometry with the third stone P1 instead. Nothing
    // is dead, the shared line holds four windows of three, and the position is
    // worth far more than the mixed one.
    let (_, all_p1) = built(
        &weights,
        &[(a, Player::P1), (b, Player::P1), (c, Player::P1)],
    );
    assert_eq!(
        all_p1.value(Player::P1),
        2 * one + 2 * two + 4 * three + 3 * 6 * one + 3 * 6 * one
    );
    assert!(
        all_p1.value(Player::P1) > mixed.value(Player::P1),
        "blocking with p2 must be worth something"
    );
}

#[test]
fn eval_only_windows_touching_a_stone_can_score() {
    // The incremental eval and the cheap reference share one assumption: on an
    // unbounded board, only a window holding a stone can score. A scan of the
    // whole region around the stones is the independent statement of it.
    let weights = committed_weights();
    let positions: Vec<Vec<(Coord, Player)>> = vec![
        Vec::new(),
        vec![(Coord::new(-3, 7), Player::P2)],
        line(Coord::new(0, 0), Axis::ConstS, 4, Player::P1),
        vec![
            (Coord::new(0, 0), Player::P1),
            (Coord::new(1, 0), Player::P2),
            (Coord::new(0, 1), Player::P1),
            (Coord::new(2, -1), Player::P2),
            (Coord::new(-1, 3), Player::P1),
            (Coord::new(4, -4), Player::P2),
        ],
    ];

    for (index, stones) in positions.iter().enumerate() {
        let (board, eval) = built(&weights, stones);
        for side in [Player::P1, Player::P2] {
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
    let stones = line(Coord::ORIGIN, Axis::ConstR, 30, Player::P1);
    let (board, eval) = built(&weights, &stones);

    assert_eq!(eval.value(Player::P1), EVAL_MAX);
    assert_eq!(eval.value(Player::P2), -EVAL_MAX);
    assert_eq!(
        eval.value(Player::P1),
        value_from_scratch(&board, &weights, Player::P1)
    );
}

#[test]
fn eval_of_an_empty_position_is_zero() {
    let weights = committed_weights();
    let (board, eval) = built(&weights, &[]);
    for side in [Player::P1, Player::P2] {
        assert_eq!(eval.value(side), 0);
        assert_eq!(value_from_scratch(&board, &weights, side), 0);
    }
}

#[test]
fn eval_window_reexport_is_the_core_window() {
    // The v0 eval window and the rules window are the SAME type, not two types
    // that agree: `pistol_eval::window` is a re-export of `pistol_core::window`
    // (docs/decisions.md D-67's flip, D-253). This assignment does not compile
    // if that stops being true, which is the assertion — a value-level equality
    // would still pass for two structurally identical types.
    let from_core = pistol_core::window::Window::new(Axis::ConstR, Coord::ORIGIN)
        .expect("the origin addresses a window");
    let as_eval: Window = from_core;
    assert_eq!(as_eval, from_core);

    // And the length the eval crate exports is the rules length, which is what
    // makes the re-export honest for v0 rather than a coincidence nobody checks.
    assert_eq!(WINDOW_LEN, pistol_core::WIN_LEN);
    assert_eq!(WINDOWS_PER_CELL, pistol_core::window::WINDOWS_PER_CELL);
    assert_eq!(
        windows_through(Coord::new(3, -2)).collect::<Vec<_>>(),
        pistol_core::window::windows_through(Coord::new(3, -2)).collect::<Vec<_>>(),
        "one enumeration, reachable by two paths"
    );
}
