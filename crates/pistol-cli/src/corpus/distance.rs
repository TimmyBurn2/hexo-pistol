//! How far each stone landed from the stones already on the board.
//!
//! One number the corpus can state that nothing else in this tool can: the
//! platform's legal radius, measured from the platform's own accepted games
//! rather than asserted.
//!
//! # The one-sided evidence this exists to complete
//!
//! `LEGAL_RADIUS` is 8 by operator ruling against the htttx spec and the HeXO
//! server (docs/decisions.md D-101), and a sealbot log note claims 6 instead
//! (docs/research/sealbot_deep_dive.md SB-65). WP-1.2a's replay result — every
//! corpus game legal under the radius-8 region, zero violations (D-149) — could
//! never separate them, because radius-6 games are a strict *subset* of radius-8
//! legality: a clean replay under 8 is exactly what both hypotheses predict.
//! That is the necessary side of the test. This is the sufficient side. An
//! observed placement at distance 7 or 8 is a game the platform accepted and a
//! radius-6 rule forbids.
//!
//! # Measured against the record's own order, and why that is conservative
//!
//! Each stone is measured against every stone *earlier in the flat `moves`
//! array*, not against the position at the start of its turn. A pair is legal
//! iff SOME ordering of its two placements is (D-6, D-51), so the record's
//! intra-turn order is an export artefact rather than a legality claim — which
//! raises the question of whether an order-sensitive measurement can say
//! anything. It can, in the direction that matters:
//!
//! If a stone's nearest earlier stone sits at distance `d >= 7`, then its
//! distance to every pre-turn stone is `>= 7` *and* its distance to its own pair
//! partner is `>= 7`. Played first it is beyond 6 of everything on the board;
//! played second it is beyond 6 of the board and of the partner too. So no
//! ordering rescues it, and the observation refutes radius 6 whichever way the
//! platform ordered the turn. The converse fails — a `d <= 6` reading is
//! consistent with both radii — and that is the whole asymmetry: this histogram
//! can refute the small radius and can never confirm it.
//!
//! This module states no rule (CLAUDE.md rule 2). Hex distance is
//! [`Coord::distance`], pistol-core's, and the histogram compares it to nothing.

use std::collections::BTreeMap;
use std::fmt;

use pistol_core::Coord;

use super::record::Record;

/// Every stone after a game's first, counted by its distance to the nearest
/// stone already placed.
///
/// A `BTreeMap` rather than a hash map so the reported order is the distance
/// order on every run and every machine (CLAUDE.md rule 4).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlacementDistances {
    counts: BTreeMap<u32, usize>,
}

impl PlacementDistances {
    /// An empty histogram.
    pub fn new() -> PlacementDistances {
        PlacementDistances {
            counts: BTreeMap::new(),
        }
    }

    /// The histogram over every game of a corpus, in corpus order.
    ///
    /// Every record is counted, including games a replay later excludes: the
    /// question is what the platform *accepted*, and a game pistol-core refuses
    /// is still a game the platform recorded.
    pub fn of(records: &[Record]) -> PlacementDistances {
        let mut distances = PlacementDistances::new();
        for record in records {
            distances.add_game(&record.moves);
        }
        distances
    }

    /// Count one game's stones, in the order the record plays them.
    ///
    /// The first stone is not measured: it has nothing to be near, and counting
    /// it against the origin would report a distance nobody placed.
    pub fn add_game(&mut self, moves: &[Coord]) {
        for (index, stone) in moves.iter().enumerate().skip(1) {
            let nearest = moves[..index]
                .iter()
                .map(|earlier| earlier.distance(*stone))
                .min()
                .expect("a stone after the first has at least one earlier stone");
            *self.counts.entry(nearest).or_insert(0) += 1;
        }
    }

    /// How many stones landed at exactly this distance.
    pub fn count(&self, distance: u32) -> usize {
        self.counts.get(&distance).copied().unwrap_or(0)
    }

    /// Every observed distance with its count, ascending by distance.
    pub fn counts(&self) -> impl Iterator<Item = (u32, usize)> + '_ {
        self.counts.iter().map(|(distance, count)| (*distance, *count))
    }

    /// The furthest a stone landed from every stone already down, or `None`
    /// where nothing was measured.
    ///
    /// This is the number the radius question turns on.
    pub fn max(&self) -> Option<u32> {
        self.counts.keys().next_back().copied()
    }

    /// The smallest observed distance, or `None` where nothing was measured.
    pub fn min(&self) -> Option<u32> {
        self.counts.keys().next().copied()
    }

    /// How many stones were measured.
    pub fn total(&self) -> usize {
        self.counts.values().sum()
    }
}

impl fmt::Display for PlacementDistances {
    /// The full distribution, then the maximum.
    ///
    /// Every distance from the smallest observed to the largest is printed, zero
    /// counts included, because a gap in the distribution is a fact about it and
    /// a listing that skipped empty rows would hide one.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (Some(min), Some(max)) = (self.min(), self.max()) else {
            return writeln!(f, "  no stone measured: every game is one stone long");
        };
        for distance in min..=max {
            writeln!(f, "  distance {distance:<15} {}", self.count(distance))?;
        }
        writeln!(f, "  stones measured         {}", self.total())?;
        write!(f, "  MAX distance            {max}")
    }
}
