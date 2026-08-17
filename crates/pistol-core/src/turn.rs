//! What a turn is made of: its phase, what one stone did to it, the whole turn
//! as one value, and how the game ends.
//!
//! These types are the vocabulary the state machine in [`crate::state`]
//! produces and every other crate consumes; the machine that moves between them
//! lives there, and the turn-level transitions live in [`crate::play`].
//!
//! There is no draw. Rule 6 gives the game no drawing mechanism at all, so
//! [`Outcome`] has no variant for one; a match turn cap is an evaluation
//! horizon that lives in the search, and never reaches these types.
//!
//! There is also no stalemate, and that is a lemma rather than an oversight: an
//! ongoing game always has a legal move. Take the stone with the largest `q`
//! (ties broken by `r` — the board is finite and non-empty); the cell eight
//! steps further along `+q` is within [`crate::LEGAL_RADIUS`] of it and cannot
//! hold a stone, since a stone there would have had the larger `q`. So
//! [`Outcome`] needs no "no legal move" variant, and no caller needs a branch
//! for one.

use std::fmt;
use std::str::FromStr;

use crate::board::Color;
use crate::coord::Coord;
use crate::error::{CoreError, PAIR_NOT_CANONICAL, PAIR_OF_ONE_CELL};

/// How far into the current turn the mover is.
///
/// This is the bit that goes into the zobrist key beside the side to move
/// (docs/decisions.md D-8): two positions with the same stones but different
/// phase are different positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phase {
    /// No stone of this turn has been placed yet.
    First,
    /// One stone of this turn has been placed.
    Second,
}

impl Phase {
    /// `0` for [`Phase::First`], `1` for [`Phase::Second`] — the phase index as
    /// the rules and the key both count it.
    pub const fn index(self) -> u32 {
        match self {
            Phase::First => 0,
            Phase::Second => 1,
        }
    }
}

/// Whether the game is still going, and who won if it is not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// No line has been completed. The game continues; there is no other way
    /// for it to end (rule 6).
    Ongoing,
    /// A stone completed a run of at least [`crate::WIN_LEN`].
    Win {
        /// The side that placed it.
        winner: Color,
        /// The turn it completed on.
        turn: u32,
    },
}

impl Outcome {
    /// Whether the game is over.
    pub const fn is_decided(self) -> bool {
        matches!(self, Outcome::Win { .. })
    }

    /// The winner, if there is one.
    pub const fn winner(self) -> Option<Color> {
        match self {
            Outcome::Ongoing => None,
            Outcome::Win { winner, .. } => Some(winner),
        }
    }
}

/// What placing one stone did to the turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlyOutcome {
    /// The stone was placed and the mover still owes the second stone of this
    /// turn.
    TurnContinues,
    /// The stone completed the turn; the other side is now to move.
    TurnComplete,
    /// The stone completed a line. The game is over, and if this was the first
    /// stone of the turn the second is never played (rule 4).
    Win {
        /// The side that placed it.
        winner: Color,
        /// The turn it completed on.
        turn: u32,
    },
}

/// A whole turn: the stones one side places to complete it.
///
/// A turn is an **unordered** pair of distinct cells, except where the rules
/// make it a single stone — turn 1 (rule 3), and a first stone that completes a
/// line, after which the second is never played (rule 4). Which of the two
/// stones of a pair goes down first is not part of the turn's identity: it is a
/// legality question the rules answer (a pair is legal iff *some* ordering of
/// its two placements is, docs/decisions.md D-6), and
/// [`crate::GameState::make_turn`] is what answers it.
///
/// [`Turn::Pair`] is **canonical**: `first < second` in [`Coord`]'s
/// lexicographic order by `q`, then `r` (docs/decisions.md D-5). One turn
/// therefore has one representation, so two turns that differ as values are
/// different turns — which is what lets perft dedupe by value and the search
/// tie-break by order. Every constructor here upholds it, generation only ever
/// emits it, and `make_turn` refuses a pair that violates it rather than
/// quietly reordering the caller's value (CLAUDE.md rule 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Turn {
    /// One stone: turn 1, or a stone that completes a line (rules 3 and 4).
    Single(Coord),
    /// Two distinct cells, smaller first.
    Pair(Coord, Coord),
}

/// The character between the two stones of a pair token, `"q,r/q,r"`
/// (docs/decisions.md D-5).
pub const PAIR_SEPARATOR: char = '/';

impl Turn {
    /// The turn that places one stone.
    pub const fn single(at: Coord) -> Turn {
        Turn::Single(at)
    }

    /// The turn that places both cells, in canonical order.
    ///
    /// Refuses one cell given twice: a turn places two stones, and a cell holds
    /// one (rules 3 and 5).
    pub fn pair(a: Coord, b: Coord) -> Result<Turn, CoreError> {
        if a == b {
            return Err(CoreError::IllegalTurn {
                turn: Turn::Pair(a, b),
                why: PAIR_OF_ONE_CELL,
            });
        }
        Ok(canonical_pair(a, b))
    }

    /// The cell that comes first in canonical order — which is not a claim
    /// about play order.
    pub const fn first(self) -> Coord {
        match self {
            Turn::Single(at) => at,
            Turn::Pair(first, _) => first,
        }
    }

    /// The second cell, if this turn has one.
    pub const fn second(self) -> Option<Coord> {
        match self {
            Turn::Single(_) => None,
            Turn::Pair(_, second) => Some(second),
        }
    }

    /// How many stones this turn puts on the board.
    pub const fn stone_count(self) -> u32 {
        match self {
            Turn::Single(_) => 1,
            Turn::Pair(_, _) => 2,
        }
    }

    /// Whether a pair is two distinct cells in canonical order. A single stone
    /// is trivially canonical.
    pub fn is_canonical(self) -> bool {
        match self {
            Turn::Single(_) => true,
            Turn::Pair(first, second) => first < second,
        }
    }
}

/// The canonical spelling of a turn over two known-distinct cells.
///
/// Generation emits pairs by the hundred thousand and knows its two cells are
/// distinct by construction, so it takes this rather than paying for the
/// refusal in [`Turn::pair`].
pub(crate) fn canonical_pair(a: Coord, b: Coord) -> Turn {
    if a < b {
        Turn::Pair(a, b)
    } else {
        Turn::Pair(b, a)
    }
}

impl fmt::Display for Turn {
    /// `"q,r"` or `"q,r/q,r"` — the turn token of the line protocol
    /// (docs/decisions.md D-5).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Turn::Single(at) => write!(f, "{at}"),
            Turn::Pair(first, second) => write!(f, "{first}{PAIR_SEPARATOR}{second}"),
        }
    }
}

/// A turn token that is not one.
///
/// The parser ships beside the formatter for the same reason the stone token's
/// does (docs/decisions.md D-39): a grammar with one implementation cannot
/// drift from itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTurnError {
    /// The token as given.
    pub token: String,
    /// What is wrong with it.
    pub why: String,
}

impl fmt::Display for ParseTurnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bad turn token {:?}: {}", self.token, self.why)
    }
}

impl std::error::Error for ParseTurnError {}

impl FromStr for Turn {
    type Err = ParseTurnError;

    /// Parse `"q,r"` or `"q,r/q,r"`: stone tokens by [`Coord`]'s own strict
    /// grammar, one separator, two distinct cells in canonical order.
    ///
    /// An uncanonical pair is refused rather than reordered. The token is a
    /// protocol contract, and one turn has one spelling (docs/decisions.md D-5,
    /// D-46) — accepting `"1,0/0,0"` would let two lines that differ mean the
    /// same turn, which an arena log, a fixture diff or a dedupe by string will
    /// eventually trip over.
    fn from_str(token: &str) -> Result<Self, Self::Err> {
        let reject = |why: String| ParseTurnError {
            token: token.to_string(),
            why,
        };
        let cell = |text: &str| -> Result<Coord, ParseTurnError> {
            text.parse::<Coord>()
                .map_err(|error| reject(error.why.to_string()))
        };
        let Some((left, right)) = token.split_once(PAIR_SEPARATOR) else {
            return Ok(Turn::Single(cell(token)?));
        };
        if right.contains(PAIR_SEPARATOR) {
            return Err(reject(format!("expected one `{PAIR_SEPARATOR}`, got more")));
        }
        let (first, second) = (cell(left)?, cell(right)?);
        if first == second {
            return Err(reject(PAIR_OF_ONE_CELL.to_string()));
        }
        if second < first {
            return Err(reject(PAIR_NOT_CANONICAL.to_string()));
        }
        Ok(Turn::Pair(first, second))
    }
}
