use crate::axis::Axis;
use crate::coord::Coord;
use crate::rules::WIN_LEN;

/// Cells in a window. Six, because a win is six (rule 2).
pub const WINDOW_LEN: u32 = WIN_LEN;

/// How many windows hold a given cell: one per axis per offset.
pub const WINDOWS_PER_CELL: usize = Axis::ALL.len() * WINDOW_LEN as usize;

/// [`WINDOW_LEN`] as a step count along an axis.
const WINDOW_STEPS: i16 = WINDOW_LEN as i16;

// The assert that is not vacuous. A `WINDOW_LEN == WIN_LEN` assert beside the
// definition above would assert nothing now that the two are the same constant
// in the same crate; it was a real cross-crate pin while the enumeration lived
// in pistol-eval, and it is dropped rather than carried across as a tautology
// (docs/decisions.md D-253). This one pins a length against a coordinate step
// count, which is a claim about two different things.
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
    /// because the alternative is a panic in an incremental update.
    #[inline]
    pub fn new(axis: Axis, start: Coord) -> Option<Window> {
        // The far end is the only one that can fall off: the components move
        // monotonically along an axis, so if both ends are addressable, every
        // cell between them is.
        start.checked_step(axis, WINDOW_STEPS - 1)?;
        Some(Window { axis, start })
    }

    /// The window's cells, from [`Window::start`] outward.
    #[inline]
    pub fn cells(self) -> [Coord; WINDOW_LEN as usize] {
        std::array::from_fn(|index| {
            let steps = i16::try_from(index).expect("a window is WINDOW_LEN cells long");
            // Cannot overflow: `new` established that the far end is addressable.
            self.start.step(self.axis, steps)
        })
    }

    /// The window's `index`-th cell, counting from [`Window::start`].
    ///
    /// # Panics
    ///
    /// If `index` is not below [`WINDOW_LEN`]. That is a caller bug rather than
    /// an answer to a question anyone asked, so it is named and loud rather
    /// than wrapped or clamped (CLAUDE.md rule 3). It is unreachable from
    /// [`windows_through_indexed`], whose index is the enumeration's own loop
    /// variable.
    #[inline]
    pub fn cell(self, index: u8) -> Coord {
        assert!(
            u32::from(index) < WINDOW_LEN,
            "cell {index} of a {WINDOW_LEN}-cell window"
        );
        let steps = i16::from(index);
        // Cannot overflow: `new` established that the far end is addressable.
        self.start.step(self.axis, steps)
    }
}

/// Every window that holds `at`: [`WINDOWS_PER_CELL`] of them, less any that run
/// off the addressable lattice.
///
/// These are exactly the windows one placed stone changes, which is what makes
/// an incremental consumer incremental (docs/decisions.md D-11).
#[inline]
pub fn windows_through(at: Coord) -> impl Iterator<Item = Window> {
    windows_through_indexed(at).map(|(window, _)| window)
}

/// Every window that holds `at`, with the index `at` occupies in each.
///
/// The index is free — it is the enumeration's own loop variable — and it is
/// what lets a consumer carry per-window occupancy masks rather than bare
/// counts, so that *which* cells of a window are empty is answerable without a
/// second reading of the board (docs/decisions.md D-253).
///
/// THIS IS THE ENUMERATION BODY, so it carries the `#[inline]` too: with the
/// attribute on [`windows_through`] alone, the wrapper inlines across the crate
/// boundary and the loop it wraps stays behind a call no LTO is configured to
/// cross (D-14). Insurance, unmeasured, exactly as the other four are.
#[inline]
pub fn windows_through_indexed(at: Coord) -> impl Iterator<Item = (Window, u8)> {
    Axis::ALL.into_iter().flat_map(move |axis| {
        (0..WINDOW_STEPS).filter_map(move |back| {
            let start = at.checked_step(axis, -back)?;
            let window = Window::new(axis, start)?;
            let index = u8::try_from(back).expect("an offset below WINDOW_LEN fits in a byte");
            Some((window, index))
        })
    })
}
