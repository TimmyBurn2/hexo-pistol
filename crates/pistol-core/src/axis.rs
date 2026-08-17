//! The three line axes of the hex lattice, and the six neighbour directions.
//!
//! A hex cell has six neighbours, which pair up into **three** axes — three
//! directions and their opposites (rule 1). Six-in-a-row is a run along one of
//! these three (rule 2), and everything downstream that says "per axis" —
//! eval windows, threat lines, zobrist-free line scans — means these three and
//! nothing else.
//!
//! **Naming rule: each axis is named for the coordinate that stays constant
//! along it** (docs/decisions.md D-33). A line of constant `q` runs in
//! direction `(0, +1)`, so it is [`Axis::ConstQ`]. The alternative convention —
//! naming an axis for the coordinate that varies — has no consistent answer for
//! the third axis, along which both `q` and `r` change.
//!
//! The `Const` prefix is there so that the invariant travels in the identifier
//! rather than in this paragraph: `step(Axis::ConstQ, k)` moves along `r`, and
//! an eval or threat table that reads `Axis::Q` and assumes otherwise is an
//! off-by-one-axis bug that no test would obviously catch.

use crate::coord::Coord;

/// One of the three line axes.
///
/// The direction vectors are pinned by
/// `axis_directions_are_the_three_hex_lines`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    /// `q` is constant along this line; direction `(0, +1)`.
    ConstQ,
    /// `r` is constant along this line; direction `(+1, 0)`.
    ConstR,
    /// `s = -q - r` is constant along this line; direction `(+1, -1)`.
    ConstS,
}

impl Axis {
    /// The three axes, in a fixed order. Iterating this is deterministic, which
    /// is why nothing in this crate ever iterates a set of axes instead.
    pub const ALL: [Axis; 3] = [Axis::ConstQ, Axis::ConstR, Axis::ConstS];

    /// The unit step along this axis, as a direction vector.
    ///
    /// The opposite direction is [`Coord::negated`] of this one; a line scan
    /// walks both (see [`crate::win`]).
    pub const fn direction(self) -> Coord {
        match self {
            Axis::ConstQ => Coord::new(0, 1),
            Axis::ConstR => Coord::new(1, 0),
            Axis::ConstS => Coord::new(1, -1),
        }
    }

    /// The coordinate that stays constant along this axis, as a word, for
    /// diagnostics and fixture reporting.
    pub const fn constant_coordinate(self) -> &'static str {
        match self {
            Axis::ConstQ => "q",
            Axis::ConstR => "r",
            Axis::ConstS => "s",
        }
    }
}

/// The six neighbour directions, in ring order starting at `(+1, 0)` and
/// turning consistently.
///
/// The order is fixed and pinned by a test: neighbour iteration feeds
/// candidate generation later, and an order that drifts is an order that can
/// change a move choice (CLAUDE.md rule 4).
pub const NEIGHBOUR_DIRECTIONS: [Coord; 6] = [
    Coord::new(1, 0),
    Coord::new(1, -1),
    Coord::new(0, -1),
    Coord::new(-1, 0),
    Coord::new(-1, 1),
    Coord::new(0, 1),
];
