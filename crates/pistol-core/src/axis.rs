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
