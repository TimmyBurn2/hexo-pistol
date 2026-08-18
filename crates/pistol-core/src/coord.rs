//! Axial coordinates on the unbounded hex lattice.
//!
//! A cell is `(q, r)`; the third cube coordinate `s = -q - r` is derived and
//! never stored. The board is unbounded (rule 1), so a coordinate type is a
//! choice about how much board we can address, not about where the edges are:
//! `i16` addresses ±32767 in each direction, which is some four thousand turns
//! of legal-region growth away from any game (docs/decisions.md D-34).
//!
//! Arithmetic here therefore never wraps and never saturates. Leaving the
//! range is a bug in pistol, not operator input, so it panics with the named
//! invariant [`COORD_OVERFLOW`] (CLAUDE.md rule 3). Callers that have a
//! legitimate reason to ask whether a cell exists at all — a line scan walking
//! off the addressable lattice, where "no such cell" and "no stone there" are
//! the same answer — use the `checked_` forms.

use std::fmt;
use std::str::FromStr;

use crate::axis::{Axis, NEIGHBOUR_DIRECTIONS};

/// Named invariant: a coordinate computation left the `i16` range.
///
/// The token appears verbatim in the panic message, so a test can pin it and a
/// log can be grepped for it.
pub const COORD_OVERFLOW: &str = "COORD_OVERFLOW";

/// A cell of the hex lattice, in axial coordinates.
///
/// The derived ordering is lexicographic by `q`, then `r` — the same total
/// order the protocol uses to canonicalize a pair and the search uses as its
/// final tie-break (docs/decisions.md D-5, D-7). Field order is therefore load
/// bearing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    /// First axial coordinate.
    pub q: i16,
    /// Second axial coordinate.
    pub r: i16,
}

impl Coord {
    /// The cell every game starts on (rule 3: turn 1 is one stone, at the
    /// origin without loss of generality, the lattice being homogeneous).
    pub const ORIGIN: Coord = Coord::new(0, 0);

    /// A cell, or a direction vector — the two share a type, and which one is
    /// meant is always said at the use site.
    pub const fn new(q: i16, r: i16) -> Self {
        Coord { q, r }
    }

    /// The third cube coordinate, `s = -q - r`.
    ///
    /// Widened to `i32` because the sum of two `i16` need not be one.
    pub const fn s(self) -> i32 {
        -(self.q as i32) - (self.r as i32)
    }

    /// This cell translated by `delta`, or `None` if that leaves the lattice we
    /// can address.
    pub fn checked_offset(self, delta: Coord) -> Option<Coord> {
        let q = self.q.checked_add(delta.q)?;
        let r = self.r.checked_add(delta.r)?;
        Some(Coord::new(q, r))
    }

    /// This cell translated by `delta`.
    ///
    /// # Panics
    ///
    /// With [`COORD_OVERFLOW`] if the result leaves the `i16` range.
    pub fn offset(self, delta: Coord) -> Coord {
        match self.checked_offset(delta) {
            Some(cell) => cell,
            None => overflow(format_args!("{self} offset by {delta}")),
        }
    }

    /// This cell moved `steps` steps along `axis`; negative goes the other way.
    ///
    /// # Panics
    ///
    /// With [`COORD_OVERFLOW`] if the result leaves the `i16` range.
    pub fn step(self, axis: Axis, steps: i16) -> Coord {
        match self.checked_step(axis, steps) {
            Some(cell) => cell,
            None => overflow(format_args!("{self} stepped {steps} along {axis:?}")),
        }
    }

    /// This cell moved `steps` steps along `axis`, or `None` if that leaves the
    /// lattice we can address.
    ///
    /// The walk is computed in `i32` and only the destination is required to
    /// fit. Checking the intermediate `direction * steps` instead would refuse
    /// a cell that exists: stepping `i16::MIN` along [`Axis::ConstS`], whose
    /// `r` component is `-1`, needs an `r` delta of `32768`, which is out of
    /// range even where the destination is not. `None` means "no such cell",
    /// and a caller reading it as "nothing there" — which is exactly how the
    /// line scan in [`crate::win`] reads it — must not be told that about a
    /// cell that could hold a stone.
    pub fn checked_step(self, axis: Axis, steps: i16) -> Option<Coord> {
        let direction = axis.direction();
        let steps = i32::from(steps);
        // Bounded by |direction| <= 1: the products fit in i32 with room to
        // spare, and so does the sum with an i16.
        let q = i32::from(self.q) + i32::from(direction.q) * steps;
        let r = i32::from(self.r) + i32::from(direction.r) * steps;
        match (i16::try_from(q), i16::try_from(r)) {
            (Ok(q), Ok(r)) => Some(Coord::new(q, r)),
            _ => None,
        }
    }

    /// This direction vector reversed.
    ///
    /// # Panics
    ///
    /// With [`COORD_OVERFLOW`] if either component is `i16::MIN`, which has no
    /// negation in range.
    pub fn negated(self) -> Coord {
        match (self.q.checked_neg(), self.r.checked_neg()) {
            (Some(q), Some(r)) => Coord::new(q, r),
            _ => overflow(format_args!("{self} negated")),
        }
    }

    /// Hex distance: the number of single-cell steps between the two cells.
    ///
    /// This is the cube distance written in axial terms. It cannot overflow —
    /// the differences are computed in `i32`, where two `i16` always fit — so
    /// it is total, unlike the translations above.
    pub fn distance(self, other: Coord) -> u32 {
        let dq = i32::from(other.q) - i32::from(self.q);
        let dr = i32::from(other.r) - i32::from(self.r);
        (dq.unsigned_abs() + dr.unsigned_abs() + (dq + dr).unsigned_abs()) / 2
    }

    /// The six cells one step away, in the fixed order of
    /// [`NEIGHBOUR_DIRECTIONS`].
    ///
    /// # Panics
    ///
    /// With [`COORD_OVERFLOW`] if this cell sits at the edge of the addressable
    /// lattice, where a neighbour would not fit.
    pub fn neighbours(self) -> [Coord; 6] {
        NEIGHBOUR_DIRECTIONS.map(|direction| self.offset(direction))
    }
}

impl fmt::Display for Coord {
    /// `"q,r"` — the stone token of the line protocol (docs/decisions.md D-5).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.q, self.r)
    }
}

/// A stone token that is not one.
///
/// The token is a protocol and fixture contract, so it is parsed here beside
/// the formatter that writes it. A second parser elsewhere — in the CLI, in a
/// fixture loader — is a second implementation of the same grammar, and the two
/// drift on exactly the cases nobody writes a test for: a leading `+`, a space
/// after the comma, an empty field (docs/decisions.md D-39).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCoordError {
    /// The token as given.
    pub token: String,
    /// What is wrong with it.
    pub why: &'static str,
}

impl fmt::Display for ParseCoordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bad stone token {:?}: {}", self.token, self.why)
    }
}

impl std::error::Error for ParseCoordError {}

impl FromStr for Coord {
    type Err = ParseCoordError;

    /// Parse `"q,r"`: two signed decimals, one comma, nothing else — no
    /// surrounding space, no space after the comma, no leading `+`, no empty
    /// field, no leading zeros, and no negative zero.
    ///
    /// The last two are what make the token **canonical**: every cell has
    /// exactly one spelling, so `display(parse(t)) == t` for every token this
    /// accepts, and two protocol lines that differ have to mean different
    /// moves. Strict, because the loose readings differ between
    /// implementations and the strict one does not.
    fn from_str(token: &str) -> Result<Self, Self::Err> {
        let reject = |why: &'static str| ParseCoordError {
            token: token.to_string(),
            why,
        };
        let (left, right) = token
            .split_once(',')
            .ok_or_else(|| reject("expected `q,r`"))?;
        if right.contains(',') {
            return Err(reject("expected one comma, got more"));
        }
        let coordinate = |text: &str| -> Result<i16, ParseCoordError> {
            if text.starts_with('+') || text.trim() != text {
                return Err(reject(
                    "a coordinate is a signed decimal with no space and no `+`",
                ));
            }
            let digits = text.strip_prefix('-').unwrap_or(text);
            if digits.len() > 1 && digits.starts_with('0') {
                return Err(reject(
                    "a coordinate has no leading zeros: one cell, one spelling",
                ));
            }
            if text == "-0" {
                return Err(reject("zero is written `0`, never `-0`"));
            }
            text.parse::<i16>()
                .map_err(|_| reject("a coordinate must be a whole number in the i16 range"))
        };
        Ok(Coord::new(coordinate(left)?, coordinate(right)?))
    }
}

/// Report a coordinate that left the representable lattice, loudly.
///
/// `pub(crate)` because [`crate::symmetry`] does its own checked arithmetic on
/// coordinates and must fail the same way this file does — one overflow
/// report, one token, one message shape.
#[cold]
#[inline(never)]
pub(crate) fn overflow(detail: fmt::Arguments<'_>) -> ! {
    panic!("pistol-core invariant {COORD_OVERFLOW}: {detail} leaves the i16 coordinate range");
}
