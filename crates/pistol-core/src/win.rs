//! Win detection, at the completing stone.
//!
//! A game is won the instant a single placed stone completes a contiguous own
//! run of [`WIN_LEN`] or more along one of the three axes (rules 2 and 4). The
//! run may be longer: an overline of seven or more wins exactly as a six does,
//! so the test is `>=` and there is nothing to un-win.
//!
//! The only question this module answers is about the stone just placed. There
//! is no whole-board scan here: rule 4 makes the completing stone the unit that
//! matters, a search that placed a stone already knows which one it was, and a
//! scan on the hot path would cost the whole board per node. Position
//! validation — *does this board handed to me from outside already contain a
//! completed line* — is a different question, asked once at a seam, and it
//! arrives with `set_position` (docs/decisions.md D-6, D-36).
//!
//! The colour is read from the board rather than passed in. A colour argument
//! admits a caller that passes the wrong one, and there is no legitimate caller
//! who would; asking about an empty cell is a bug and panics with the named
//! invariant [`WIN_CHECK_ON_EMPTY_CELL`].
//!
//! That panic is not a way for bad input to kill the engine. The one caller
//! inside pistol is [`crate::GameState::place`], which asks only about the
//! stone it has just successfully placed, and every refusal a protocol line
//! could provoke — occupied cell, cell outside the legal region, stone after
//! the game is decided — has already been named and returned before the win
//! check runs, so the engine answers and stays alive (docs/decisions.md D-5,
//! D-36). [`crate::CoreError::UnoccupiedCell`], the error shape of the same
//! mistake, belongs to [`Board::undo`], where a caller genuinely can ask about
//! a cell whose contents it does not know.

use crate::axis::Axis;
use crate::board::{Board, Color};
use crate::coord::Coord;
use crate::rules::WIN_LEN;

/// Named invariant: win detection was asked about a cell holding no stone.
pub const WIN_CHECK_ON_EMPTY_CELL: &str = "WIN_CHECK_ON_EMPTY_CELL";

/// A contiguous single-colour run along one axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Run {
    /// The axis the run lies on.
    pub axis: Axis,
    /// The end of the run reached by walking against [`Axis::direction`].
    ///
    /// Every axis direction has a positive `q`, or a zero `q` and a positive
    /// `r`, so this end is also the lexicographically smaller of the two — one
    /// endpoint, defined two ways that agree, and neither of them "whichever
    /// the scan happened to reach first" (CLAUDE.md rule 4).
    pub start: Coord,
    /// How many stones long it is.
    pub len: u32,
    /// Whose stones they are.
    pub color: Color,
}

/// Whether the stone on `at` completes a line, and so wins the game.
///
/// # Panics
///
/// With [`WIN_CHECK_ON_EMPTY_CELL`] if `at` holds no stone.
pub fn wins_at(board: &Board, at: Coord) -> bool {
    let color = color_at(board, at);
    Axis::ALL
        .iter()
        .any(|&axis| run_length(board, at, color, axis) >= WIN_LEN)
}

/// The length of the run through `at` along `axis`, counting `at` itself.
///
/// # Panics
///
/// With [`WIN_CHECK_ON_EMPTY_CELL`] if `at` holds no stone.
pub fn run_through(board: &Board, at: Coord, axis: Axis) -> u32 {
    let color = color_at(board, at);
    run_length(board, at, color, axis)
}

/// The run through `at` that wins the game, if one does.
///
/// This is the diagnostic form of [`wins_at`] — it says which axis and where
/// the run starts, which is what a fixture report or an error message wants.
/// The search asks [`wins_at`].
///
/// One stone can complete two axes at once, so which one is reported is a
/// choice, and it is fixed: the first in [`Axis::ALL`] order. Nothing here is
/// allowed to depend on which one a scan reached first (CLAUDE.md rule 4).
///
/// # Panics
///
/// With [`WIN_CHECK_ON_EMPTY_CELL`] if `at` holds no stone.
pub fn winning_run(board: &Board, at: Coord) -> Option<Run> {
    let color = color_at(board, at);
    Axis::ALL.iter().find_map(|&axis| {
        let (backward, forward) = extents(board, at, color, axis);
        let len = backward.count + forward.count + 1;
        if len < WIN_LEN {
            return None;
        }
        // The scan already stood on the far end, so the start is carried out of
        // the walk rather than recomputed from the length. Recomputing it would
        // mean converting a run length into a step count, and a line of this
        // lattice is longer than a step count can be — a run of more than 32768
        // stones is a position no game reaches but `Board::apply` can build, and
        // it must not turn a diagnostic into a panic.
        Some(Run {
            axis,
            start: backward.end,
            len,
            color,
        })
    })
}

/// The colour of the stone whose placement is being judged.
fn color_at(board: &Board, at: Coord) -> Color {
    match board.get(at) {
        Some(color) => color,
        None => panic!(
            "pistol-core invariant {WIN_CHECK_ON_EMPTY_CELL}: asked whether the stone on \
             {at} wins, but that cell is empty"
        ),
    }
}

/// Length of the contiguous `color` run through `at` along `axis`.
fn run_length(board: &Board, at: Coord, color: Color, axis: Axis) -> u32 {
    let (backward, forward) = extents(board, at, color, axis);
    backward.count + forward.count + 1
}

/// One direction's half of a run: how many stones, and where it ends.
struct Reach {
    count: u32,
    end: Coord,
}

/// How far the run reaches from `at` in each direction along `axis`, not
/// counting `at` itself.
fn extents(board: &Board, at: Coord, color: Color, axis: Axis) -> (Reach, Reach) {
    let direction = axis.direction();
    (
        reach(board, at, color, direction.negated()),
        reach(board, at, color, direction),
    )
}

/// The consecutive `color` stones following `from` in `direction`: how many,
/// and the last cell the walk stood on.
///
/// A step that leaves the addressable lattice ends the run: there is no cell
/// there, so there is no stone there either. That is the answer to the
/// question, not a swallowed overflow — the arithmetic that must never leave
/// the range is a *placement*, and that one panics (see
/// [`crate::coord::COORD_OVERFLOW`]).
fn reach(board: &Board, from: Coord, color: Color, direction: Coord) -> Reach {
    let mut count = 0;
    let mut cell = from;
    loop {
        let Some(next) = cell.checked_offset(direction) else {
            return Reach { count, end: cell };
        };
        if board.get(next) != Some(color) {
            return Reach { count, end: cell };
        }
        count += 1;
        cell = next;
    }
}
