//! The windows a stone sits in: three axes, six offsets, eighteen windows.
//!
//! A v0 window is [`WINDOW_LEN`] consecutive cells along one of the three line
//! axes, named by the cell it starts at. That is the same length as a win
//! (rule 2), which is why a window that fills with one player is a won position
//! — but the *length* is a property of the eval backend and not a rule: the
//! Stage-2 codebook reads length-eleven windows under the same trait. So the
//! enumeration lives here rather than in pistol-core, and composes core's
//! geometry ([`Axis::direction`] via [`Coord::checked_step`]) rather than
//! restating any of it (CLAUDE.md rule 2, docs/decisions.md D-67).
//!
//! On an unbounded board only a window holding a stone can score, so nothing
//! here ever enumerates windows in general: [`windows_through`] enumerates the
//! windows through *a cell*, which is how the incremental eval finds exactly the
//! entries one stone changes.

use pistol_core::{Axis, Coord, WIN_LEN};

/// Cells in a v0 eval window.
pub const WINDOW_LEN: u32 = WIN_LEN;

/// How many windows hold a given cell: one per axis per offset.
pub const WINDOWS_PER_CELL: usize = Axis::ALL.len() * WINDOW_LEN as usize;

/// [`WINDOW_LEN`] as a step count along an axis.
const WINDOW_STEPS: i16 = WINDOW_LEN as i16;

const _: () = assert!(
    WINDOW_STEPS as u32 == WINDOW_LEN,
    "a window length that does not fit a coordinate step count is not a window"
);

/// A window: [`WINDOW_LEN`] cells from `start`, stepping along `axis`.
///
/// The ordering is `(axis, start)` with `start` lexicographic by `(q, r)` —
/// derived, and deterministic, which is what lets the bookkeeping live in an
/// ordered map with no hasher anywhere near a value the engine plays on
/// (CLAUDE.md rule 4, docs/decisions.md D-32).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Window {
    /// The axis the window runs along.
    pub axis: Axis,
    /// The cell it starts at; the window runs in the axis direction from here.
    pub start: Coord,
}

impl Window {
    /// The window of [`WINDOW_LEN`] cells from `start` along `axis`, or `None` if
    /// it would run off the addressable lattice.
    ///
    /// `None` means *there is no such window*, the same way
    /// [`Coord::checked_step`] means there is no such cell (docs/decisions.md
    /// D-47). It is unreachable in any game — the far end of the lattice is
    /// thousands of turns of legal-region growth away — and it is checked
    /// because the alternative is a panic in an eval update.
    pub fn new(axis: Axis, start: Coord) -> Option<Window> {
        // The far end is the only one that can fall off: the components move
        // monotonically along an axis, so if both ends are addressable, every
        // cell between them is.
        start.checked_step(axis, WINDOW_STEPS - 1)?;
        Some(Window { axis, start })
    }

    /// The window's cells, from [`Window::start`] outward.
    pub fn cells(self) -> [Coord; WINDOW_LEN as usize] {
        std::array::from_fn(|index| {
            let steps = i16::try_from(index).expect("a window is WINDOW_LEN cells long");
            // Cannot overflow: `new` established that the far end is addressable.
            self.start.step(self.axis, steps)
        })
    }
}

/// Every window that holds `at`: [`WINDOWS_PER_CELL`] of them, less any that run
/// off the addressable lattice.
///
/// These are exactly the windows one placed stone changes, which is what makes
/// the eval incremental (docs/decisions.md D-11).
pub fn windows_through(at: Coord) -> impl Iterator<Item = Window> {
    Axis::ALL.into_iter().flat_map(move |axis| {
        (0..WINDOW_STEPS).filter_map(move |back| Window::new(axis, at.checked_step(axis, -back)?))
    })
}
