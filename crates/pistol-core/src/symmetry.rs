use std::fmt;

use crate::board::Player;
use crate::coord::{Coord, overflow};
use crate::turn::{Turn, canonical_pair};

/// One of the twelve symmetries of the lattice about the origin.
///
/// The derived ordering is the order of [`Symmetry::ALL`], which is fixed: a
/// canonical form is a minimum over this set, and a set whose order could drift
/// is a canonical form that could drift with it (CLAUDE.md rule 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symmetry {
    /// Whether to reflect first. FIRST FIELD, so that the derived ordering is
    /// the order of [`Symmetry::ALL`] rather than an interleaving of the two
    /// halves — the doc above promises that, and a derive is what keeps it true.
    reflected: bool,
    /// How many sixth-turns to rotate by, `0..6`.
    sixths: u8,
}

impl Symmetry {
    /// The identity.
    pub const IDENTITY: Symmetry = Symmetry {
        sixths: 0,
        reflected: false,
    };

    /// All twelve, in a fixed order. Iterating this is deterministic, which is
    /// why nothing ever iterates a set of symmetries instead.
    pub const ALL: [Symmetry; 12] = [
        Symmetry {
            sixths: 0,
            reflected: false,
        },
        Symmetry {
            sixths: 1,
            reflected: false,
        },
        Symmetry {
            sixths: 2,
            reflected: false,
        },
        Symmetry {
            sixths: 3,
            reflected: false,
        },
        Symmetry {
            sixths: 4,
            reflected: false,
        },
        Symmetry {
            sixths: 5,
            reflected: false,
        },
        Symmetry {
            sixths: 0,
            reflected: true,
        },
        Symmetry {
            sixths: 1,
            reflected: true,
        },
        Symmetry {
            sixths: 2,
            reflected: true,
        },
        Symmetry {
            sixths: 3,
            reflected: true,
        },
        Symmetry {
            sixths: 4,
            reflected: true,
        },
        Symmetry {
            sixths: 5,
            reflected: true,
        },
    ];

    /// This cell under this symmetry, or `None` if that leaves the lattice we
    /// can address.
    ///
    /// Every intermediate is one of `±q`, `±r`, `±(q + r)`, so this answers
    /// `Some` for exactly the cells within hex distance `i16::MAX` of the
    /// origin. The reflection is a transposition and can never overflow; only
    /// the rotation does arithmetic.
    pub fn checked_apply(self, cell: Coord) -> Option<Coord> {
        let mut cell = if self.reflected {
            Coord::new(cell.r, cell.q)
        } else {
            cell
        };
        for _ in 0..self.sixths {
            cell = rotate(cell)?;
        }
        Some(cell)
    }

    /// This cell under this symmetry.
    ///
    /// # Panics
    ///
    /// With [`crate::coord::COORD_OVERFLOW`] if the cell lies further than
    /// `i16::MAX` from the origin. No cell a game can reach does: rule 3 anchors
    /// the first stone on the origin and rule 5 grows the legal region by at
    /// most [`crate::LEGAL_RADIUS`] per stone, so reaching that distance would
    /// take some two thousand turns (docs/decisions.md D-34).
    ///
    /// The report names the cell the CALLER gave and the symmetry applied to it,
    /// never the intermediate the arithmetic happened to fail on — that cell is
    /// one nobody mentioned, which is the same reason [`Coord::step`] reports
    /// its own arguments.
    pub fn apply(self, cell: Coord) -> Coord {
        match self.checked_apply(cell) {
            Some(image) => image,
            None => overflow(format_args!("{cell} under the symmetry {self}")),
        }
    }
}

impl fmt::Display for Symmetry {
    /// `"rot<n>"` or `"refl-rot<n>"` — for diagnostics and fixture reporting.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.reflected {
            write!(f, "refl-rot{}", self.sixths)
        } else {
            write!(f, "rot{}", self.sixths)
        }
    }
}

/// One sixth turn: `(q, r) -> (-r, q + r)`, or `None` off the addressable
/// lattice. The caller reports, because the caller knows which cell was asked
/// about.
fn rotate(cell: Coord) -> Option<Coord> {
    let q = cell.r.checked_neg()?;
    let r = cell.q.checked_add(cell.r)?;
    Some(Coord::new(q, r))
}

/// A position under a symmetry: every stone transformed, then sorted.
///
/// Sorting is what makes two images comparable — a position is a set of placed
/// stones, and the order they were played in is not part of the identity this
/// module is about.
pub fn transform(stones: &[(Coord, Player)], symmetry: Symmetry) -> Vec<(Coord, Player)> {
    let mut image: Vec<(Coord, Player)> = stones
        .iter()
        .map(|&(cell, player)| (symmetry.apply(cell), player))
        .collect();
    image.sort_unstable();
    image
}

/// The canonical spelling of a position: the least of its twelve images.
///
/// "Least" is by the derived lexicographic order on the sorted stone list,
/// which is total, so one position has one canonical form and two positions
/// with the same canonical form are the same position up to a symmetry of the
/// lattice.
///
/// The input need not be sorted and its play order is not read.
pub fn canonical_form(stones: &[(Coord, Player)]) -> Vec<(Coord, Player)> {
    let mut best: Option<Vec<(Coord, Player)>> = None;
    for symmetry in Symmetry::ALL {
        let image = transform(stones, symmetry);
        match &best {
            Some(current) if *current <= image => {}
            _ => best = Some(image),
        }
    }
    // `Symmetry::ALL` is non-empty, so the loop assigned at least once. Said
    // with `unreachable!` rather than a default, because the default here would
    // be the empty position — a legitimate answer, and so indistinguishable from
    // a correct one at the seam that decides opening identity (rule 3).
    best.unwrap_or_else(|| unreachable!("Symmetry::ALL is never empty"))
}

/// A GAME under a symmetry: every turn transformed, in the order it was played.
///
/// Not [`transform`], and the difference is the whole point. That function
/// sorts, because a position is a set of stones and play order is not part of
/// the identity it computes. A game is a SEQUENCE — two different games can
/// reach the same stones — so keying one on a position's canonical form would
/// merge them, which is the opposite of the error symmetry folding exists to
/// prevent (docs/decisions.md D-137).
///
/// A transformed pair is re-canonicalized, because a symmetry does not preserve
/// the `(q, r)` order D-5 pins the pair token to: the image of a canonical pair
/// need not be canonical, and a key that skipped this would not be
/// symmetry-invariant at all.
pub fn transform_sequence(turns: &[Turn], symmetry: Symmetry) -> Vec<Turn> {
    turns
        .iter()
        .map(|&turn| match turn {
            Turn::Single(at) => Turn::Single(symmetry.apply(at)),
            Turn::Pair(first, second) => {
                canonical_pair(symmetry.apply(first), symmetry.apply(second))
            }
        })
        .collect()
}

/// The canonical spelling of a game: the least of its twelve images.
///
/// "Least" is by the derived lexicographic order on the turn list, which is
/// total, so one game has one canonical form and two games with the same
/// canonical form are the same game up to a symmetry of the lattice. This is
/// the key an arena counts DISTINCT games by (CLAUDE.md rule 6).
///
/// It has no false positives. It does have false negatives, and that is a fact
/// about the search rather than about this function: D-7's final tie-break is
/// lexicographic by `(q, r)` and is therefore not symmetry-invariant, so two
/// mirrored openings usually do not produce mirrored games (D-137 records the
/// same limitation). Games this misses are strongly correlated samples, not
/// identical ones.
pub fn canonical_sequence(turns: &[Turn]) -> Vec<Turn> {
    let mut best: Option<Vec<Turn>> = None;
    for symmetry in Symmetry::ALL {
        let image = transform_sequence(turns, symmetry);
        match &best {
            Some(current) if *current <= image => {}
            _ => best = Some(image),
        }
    }
    // As in `canonical_form`: said with `unreachable!` rather than a default,
    // because the default would be the empty game — a legitimate answer, and so
    // indistinguishable from a correct one at the seam that decides how many
    // distinct games a run measured (rule 3).
    best.unwrap_or_else(|| unreachable!("Symmetry::ALL is never empty"))
}
