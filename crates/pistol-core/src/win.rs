use crate::axis::Axis;
use crate::board::{Board, Player};
use crate::coord::Coord;
use crate::rules::WIN_LEN;

/// Named invariant: win detection was asked about a cell holding no stone.
pub const WIN_CHECK_ON_EMPTY_CELL: &str = "WIN_CHECK_ON_EMPTY_CELL";

/// A contiguous single-player run along one axis.
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
    pub player: Player,
}

/// Whether the stone on `at` completes a line, and so wins the game.
///
/// # Panics
///
/// With [`WIN_CHECK_ON_EMPTY_CELL`] if `at` holds no stone.
pub fn wins_at(board: &Board, at: Coord) -> bool {
    let player = player_at(board, at);
    Axis::ALL
        .iter()
        .any(|&axis| run_length(board, at, player, axis) >= WIN_LEN)
}

/// The length of the run through `at` along `axis`, counting `at` itself.
///
/// # Panics
///
/// With [`WIN_CHECK_ON_EMPTY_CELL`] if `at` holds no stone.
pub fn run_through(board: &Board, at: Coord, axis: Axis) -> u32 {
    let player = player_at(board, at);
    run_length(board, at, player, axis)
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
    let player = player_at(board, at);
    Axis::ALL.iter().find_map(|&axis| {
        let (backward, forward) = extents(board, at, player, axis);
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
            player,
        })
    })
}

/// The player of the stone whose placement is being judged.
fn player_at(board: &Board, at: Coord) -> Player {
    match board.get(at) {
        Some(player) => player,
        None => panic!(
            "pistol-core invariant {WIN_CHECK_ON_EMPTY_CELL}: asked whether the stone on \
             {at} wins, but that cell is empty"
        ),
    }
}

/// Length of the contiguous `player` run through `at` along `axis`.
fn run_length(board: &Board, at: Coord, player: Player, axis: Axis) -> u32 {
    let (backward, forward) = extents(board, at, player, axis);
    backward.count + forward.count + 1
}

/// One direction's half of a run: how many stones, and where it ends.
struct Reach {
    count: u32,
    end: Coord,
}

/// How far the run reaches from `at` in each direction along `axis`, not
/// counting `at` itself.
fn extents(board: &Board, at: Coord, player: Player, axis: Axis) -> (Reach, Reach) {
    let direction = axis.direction();
    (
        reach(board, at, player, direction.negated()),
        reach(board, at, player, direction),
    )
}

/// The consecutive `player` stones following `from` in `direction`: how many,
/// and the last cell the walk stood on.
///
/// A step that leaves the addressable lattice ends the run: there is no cell
/// there, so there is no stone there either. That is the answer to the
/// question, not a swallowed overflow — the arithmetic that must never leave
/// the range is a *placement*, and that one panics (see
/// [`crate::coord::COORD_OVERFLOW`]).
fn reach(board: &Board, from: Coord, player: Player, direction: Coord) -> Reach {
    let mut count = 0;
    let mut cell = from;
    loop {
        let Some(next) = cell.checked_offset(direction) else {
            return Reach { count, end: cell };
        };
        if board.get(next) != Some(player) {
            return Reach { count, end: cell };
        }
        count += 1;
        cell = next;
    }
}
