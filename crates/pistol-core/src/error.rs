use std::fmt;

use crate::board::Player;
use crate::coord::Coord;
use crate::rules::LEGAL_RADIUS;
use crate::turn::Turn;

/// Why a turn was refused, as a named constant rather than a literal.
///
/// [`CoreError::IllegalTurn`] carries one of these. They are constants so that
/// a test can pin the exact refusal without restating its wording, and so that
/// the engine's `IllegalMove { turn, why }` (docs/decisions.md D-10) can pass
/// the reason through unchanged.
pub const PAIR_OF_ONE_CELL: &str = "a turn places two stones, and one cell holds one";
/// See [`PAIR_OF_ONE_CELL`].
pub const PAIR_NOT_CANONICAL: &str =
    "a pair is written smaller cell first, lexicographic by (q, then r)";
/// See [`PAIR_OF_ONE_CELL`].
pub const SINGLE_THAT_DOES_NOT_WIN: &str = "a turn of one stone completes only when that stone completes a line (rule 4); this turn owes \
     two";
/// See [`PAIR_OF_ONE_CELL`].
pub const PAIR_ON_THE_FIRST_TURN: &str = "turn 1 places one stone (rule 3)";
/// See [`PAIR_OF_ONE_CELL`].
pub const EITHER_STONE_ALREADY_WINS: &str = "each cell completes a line on its own, so rule 4 ends the turn on whichever goes down first: \
     each is a turn of one stone, and neither is this pair";

/// Every way the rules layer refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreError {
    /// A stone was placed on a cell that already holds one.
    OccupiedCell {
        /// The cell that is already occupied.
        at: Coord,
    },
    /// A stone was placed outside the legal region: further than
    /// [`LEGAL_RADIUS`] from every stone on the board (rule 5).
    OutsideLegalRegion {
        /// The cell that was refused.
        at: Coord,
    },
    /// The first stone of the game was placed somewhere other than the origin.
    ///
    /// This is rule 3, not rule 5: an empty board has no stone for rule 5 to
    /// measure from, and the first stone is at the origin because the lattice
    /// is homogeneous and one cell may be chosen without loss of generality. A
    /// game recorded elsewhere with a different first stone is replayed by
    /// translating every stone by the same vector ([`Coord::offset`]) — the
    /// rebasing is geometry, and it lives here (docs/decisions.md D-40).
    FirstStoneNotAtOrigin {
        /// The cell that was refused.
        at: Coord,
    },
    /// A stone was placed after the game was already won. There is no play
    /// after a completed line, and in particular the second stone of a winning
    /// turn is never played (rule 4).
    GameDecided {
        /// Who won.
        winner: Player,
        /// The turn the win completed on. Sudden death is scored in turns.
        turn: u32,
    },
    /// A stone was taken back from a cell that holds none.
    UnoccupiedCell {
        /// The cell that is empty.
        at: Coord,
    },
    /// A stone was taken back from a game in which none has been played.
    NothingToUndo,
    /// A turn does not fit the position it was played into, in a way that is
    /// about the turn as a whole rather than about one of its cells.
    ///
    /// A cell that is occupied or outside the legal region is refused by the
    /// variants above, which name it. This one covers what only a whole turn
    /// can get wrong: a malformed pair, a stone count the turn does not owe,
    /// and rule 4's truncation.
    IllegalTurn {
        /// The turn as given.
        turn: Turn,
        /// The named reason — one of [`PAIR_OF_ONE_CELL`] and its neighbours.
        why: &'static str,
    },
    /// A turn-level operation was asked of a position in the middle of a turn.
    ///
    /// Generating, making and unmaking whole turns are defined at turn
    /// boundaries: a position that already holds one stone of the current turn
    /// owes a single stone, which is a ply and not a turn. The ply-level
    /// [`crate::GameState::place`] is what finishes it.
    TurnInProgress {
        /// The turn that is half played.
        turn: u32,
    },
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::OccupiedCell { at } => write!(f, "cell {at} already holds a stone"),
            CoreError::OutsideLegalRegion { at } => write!(
                f,
                "cell {at} is outside the legal region: a stone must be placed within \
                 hex-distance {LEGAL_RADIUS} of some stone already on the board"
            ),
            CoreError::FirstStoneNotAtOrigin { at } => write!(
                f,
                "the first stone of the game goes on 0,0, not on {at}: the board is empty, \
                 so there is nothing to measure a distance from, and the lattice is \
                 homogeneous"
            ),
            CoreError::GameDecided { winner, turn } => write!(
                f,
                "the game was already won by {winner} on turn {turn}: no stone follows a \
                 completed line"
            ),
            CoreError::UnoccupiedCell { at } => write!(f, "cell {at} holds no stone to take back"),
            CoreError::NothingToUndo => f.write_str("no stone has been played to take back"),
            CoreError::IllegalTurn { turn, why } => write!(f, "turn {turn} is not legal: {why}"),
            CoreError::TurnInProgress { turn } => write!(
                f,
                "turn {turn} is half played and owes one more stone: a whole turn is generated, \
                 made and unmade at a turn boundary, and this position is not at one"
            ),
        }
    }
}

impl std::error::Error for CoreError {}
