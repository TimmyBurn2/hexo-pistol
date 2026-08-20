//! The rules window: three axes, six offsets, eighteen per cell (game rule 2).
//!
//! The enumeration moved here from `pistol-eval` when Stage 1's threat
//! generator came to need it — D-67's own flip clause (docs/decisions.md D-67,
//! D-253) — so its pins live here now, beside the constant that fixes its
//! length. What `pistol-eval` keeps is a re-export, and the test that the
//! re-export is this very type lives in that crate's tree.
//!
//! [`Window::cell`] and [`windows_through_indexed`] are new surface: the index
//! is the enumeration's own loop variable, and a consumer carrying per-window
//! occupancy masks needs it to say *which* cell of a window a stone landed in.
//! Both are pinned here against the enumeration they come from.

use std::collections::BTreeSet;

use pistol_core::window::{
    WINDOW_LEN, WINDOWS_PER_CELL, Window, windows_through, windows_through_indexed,
};
use pistol_core::{Axis, Coord, WIN_LEN};

/// A spread of cells, including two far from the origin: the enumeration must
/// not depend on where a cell sits.
const CELLS: [Coord; 4] = [
    Coord::ORIGIN,
    Coord::new(-4, 9),
    Coord::new(31, -17),
    Coord::new(-1200, 640),
];

#[test]
fn window_length_is_the_win_length() {
    // Rule 2, not a backend's convention: this is why the module is in core.
    assert_eq!(WINDOW_LEN, WIN_LEN);
    assert_eq!(
        WINDOWS_PER_CELL,
        Axis::ALL.len() * WINDOW_LEN as usize,
        "one window per axis per offset"
    );
    assert_eq!(WINDOWS_PER_CELL, 18, "3 axes x 6 offsets through a cell");
}

#[test]
fn windows_through_a_cell_are_the_distinct_windows_that_hold_it() {
    for cell in CELLS {
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
        }
    }
}

#[test]
fn window_cell_agrees_with_the_whole_cell_array() {
    for cell in CELLS {
        for window in windows_through(cell) {
            let cells = window.cells();
            for (index, &expected) in cells.iter().enumerate() {
                let index = u8::try_from(index).expect("six fits in a byte");
                assert_eq!(window.cell(index), expected, "{window:?} cell {index}");
            }
        }
    }
}

#[test]
#[should_panic(expected = "cell 6 of a 6-cell window")]
fn window_cell_refuses_an_index_off_the_end() {
    // Rule 3: a caller bug is named and loud, never wrapped or clamped.
    let window = Window::new(Axis::ConstR, Coord::ORIGIN).expect("the origin addresses a window");
    let _ = window.cell(WINDOW_LEN as u8);
}

#[test]
fn windows_through_indexed_names_the_index_the_cell_occupies() {
    for cell in CELLS {
        let indexed: Vec<(Window, u8)> = windows_through_indexed(cell).collect();
        // The plain enumeration is the indexed one with the index dropped, in
        // the same order — an incremental consumer and the eval must walk the
        // same windows in the same sequence or they are two enumerations.
        let plain: Vec<Window> = windows_through(cell).collect();
        assert_eq!(
            indexed
                .iter()
                .map(|&(window, _)| window)
                .collect::<Vec<_>>(),
            plain,
            "the two enumerations disagree at {cell}"
        );
        for (window, index) in indexed {
            assert_eq!(
                window.cell(index),
                cell,
                "{window:?} does not hold {cell} at index {index}"
            );
        }
    }
}

#[test]
fn a_window_that_runs_off_the_lattice_is_not_a_window() {
    // D-47's reading of `None`: there is no such window, which is a different
    // statement from "the window is empty". At the far corner most of the
    // eighteen do not exist.
    let corner = Coord::new(i16::MAX, i16::MAX);
    assert_eq!(
        Window::new(Axis::ConstR, corner),
        None,
        "a window from the far corner has nowhere to run"
    );
    let windows: Vec<Window> = windows_through(corner).collect();
    assert!(
        windows.len() < WINDOWS_PER_CELL,
        "the corner cell sits in fewer than {WINDOWS_PER_CELL} windows, got {}",
        windows.len()
    );
    assert!(
        !windows.is_empty(),
        "and in more than none: the windows ENDING at the corner exist"
    );
    for window in windows {
        assert!(window.cells().contains(&corner));
    }
}
