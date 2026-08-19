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
//! never separate them, because radius-6-legal games are a strict *subset* of
//! radius-8-legal games: a clean replay under 8 is exactly what both hypotheses
//! predict. That is the necessary side. This is the sufficient side. A placement
//! the platform accepted and a radius-6 rule forbids refutes the smaller radius.
//!
//! # Which placements carry that argument, and which do not
//!
//! Every stone is measured against every stone EARLIER IN THE RECORD'S FLAT
//! `moves` ARRAY. That quantity is order-sensitive and legality is not — a turn
//! is legal iff SOME ordering of its stones is (D-6, D-51) — so the measurement
//! divides into two classes, and only one of them proves anything.
//!
//! A stone is **order-independent** when it is the LAST stone of its own turn as
//! recorded: every stone of its turn is then already inside its minimum.
//! Reordering that turn can only take turn-mates *off* the board at the moment
//! this stone is placed, never add one, so its measured distance is a LOWER
//! BOUND over every ordering. Measured at `d`, it is at least `d` from
//! everything on the board however the platform ordered the turn — so if
//! `d > 6`, no ordering makes it legal under radius 6.
//!
//! A stone that is NOT the last of its turn has a turn-mate LATER in the array,
//! which its minimum never saw. That partner can bridge: play the partner first,
//! within 6 of the board, then this stone within 6 of the partner. Its measured
//! distance proves nothing about radius 6, and the corpus holds real examples —
//! `dff648bcbc1833d0` index 1 sits at distance 7 from the only stone before it,
//! while its partner is 6 from that stone and 1 from it, so the reversed
//! ordering is legal under radius 6.
//!
//! **The refutation is therefore carried by the order-independent tail alone**,
//! which is why it is counted and reported separately. An earlier revision of
//! this module claimed the whole tail refuted radius 6, on the false premise
//! that a far stone must also be far from its own partner; that holds only when
//! the partner is already inside the minimum, which is exactly the
//! order-independent case. The converse is claimed in neither class: a short
//! measured distance was never evidence *for* radius 6.
//!
//! This module states no rule (CLAUDE.md rule 2). Hex distance is
//! [`Coord::distance`], pistol-core's; the turn structure is
//! [`stones_in_turn`]'s; and the histogram compares neither to anything.

use std::collections::BTreeMap;
use std::fmt;

use pistol_core::{Coord, FIRST_TURN, stones_in_turn};

use super::record::Record;

/// Every stone after a game's first, counted by its distance to the nearest
/// stone already placed.
///
/// `BTreeMap`s rather than hash maps so the reported order is the distance order
/// on every run and every machine (CLAUDE.md rule 4).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlacementDistances {
    all: BTreeMap<u32, usize>,
    order_independent: BTreeMap<u32, usize>,
}

impl PlacementDistances {
    /// An empty histogram.
    pub fn new() -> PlacementDistances {
        PlacementDistances::default()
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
        let last_of_turn = last_of_turn_flags(moves.len());
        for (index, stone) in moves.iter().enumerate().skip(1) {
            let nearest = moves[..index]
                .iter()
                .map(|earlier| earlier.distance(*stone))
                .min()
                .expect("a stone after the first has at least one earlier stone");
            *self.all.entry(nearest).or_insert(0) += 1;
            if last_of_turn[index] {
                *self.order_independent.entry(nearest).or_insert(0) += 1;
            }
        }
    }

    /// How many stones landed at exactly this distance.
    pub fn count(&self, distance: u32) -> usize {
        self.all.get(&distance).copied().unwrap_or(0)
    }

    /// How many of those were the last stone of their own turn, and so are a
    /// lower bound over every ordering of that turn.
    pub fn order_independent_count(&self, distance: u32) -> usize {
        self.order_independent.get(&distance).copied().unwrap_or(0)
    }

    /// Every observed distance with its count, ascending by distance.
    pub fn counts(&self) -> impl Iterator<Item = (u32, usize)> + '_ {
        self.all.iter().map(|(distance, count)| (*distance, *count))
    }

    /// Every observed order-independent distance with its count, ascending.
    pub fn order_independent_counts(&self) -> impl Iterator<Item = (u32, usize)> + '_ {
        self.order_independent
            .iter()
            .map(|(distance, count)| (*distance, *count))
    }

    /// How many stones strictly beyond `radius` are order-independent, and so
    /// cannot be made legal at that radius by any reordering of their turn.
    ///
    /// This is the count that refutes a proposed radius.
    pub fn unrescuable_beyond(&self, radius: u32) -> usize {
        self.order_independent
            .range(radius + 1..)
            .map(|(_, count)| count)
            .sum()
    }

    /// The furthest a stone landed from every stone already down, or `None`
    /// where nothing was measured.
    pub fn max(&self) -> Option<u32> {
        self.all.keys().next_back().copied()
    }

    /// The smallest observed distance, or `None` where nothing was measured.
    pub fn min(&self) -> Option<u32> {
        self.all.keys().next().copied()
    }

    /// How many stones were measured.
    pub fn total(&self) -> usize {
        self.all.values().sum()
    }

    /// How many of those are order-independent.
    pub fn order_independent_total(&self) -> usize {
        self.order_independent.values().sum()
    }
}

/// Which flat indices are the last stone of their own turn.
///
/// The turn structure is [`stones_in_turn`]'s, walked exactly the way
/// [`super::replay::group_turns`] walks it, so a turn the record cuts short
/// (rule 4's truncation) makes its one stone the last of that turn — which is
/// right: a stone with no turn-mate at all has nothing that could bridge it.
fn last_of_turn_flags(stones: usize) -> Vec<bool> {
    let mut flags = vec![false; stones];
    let mut index = 0usize;
    let mut number = FIRST_TURN;
    while index < stones {
        let owed = stones_in_turn(number) as usize;
        let taken = owed.min(stones - index);
        flags[index + taken - 1] = true;
        index += taken;
        number += 1;
    }
    flags
}

impl fmt::Display for PlacementDistances {
    /// The full distribution, the order-independent share of it, then the
    /// maximum.
    ///
    /// Every distance from the smallest observed to the largest is printed, zero
    /// counts included, because a gap in the distribution is a fact about it and
    /// a listing that skipped empty rows would hide one.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (Some(min), Some(max)) = (self.min(), self.max()) else {
            return write!(f, "  no stone was measured");
        };
        writeln!(f, "  distance   count      of which order-independent")?;
        for distance in min..=max {
            writeln!(
                f,
                "  {distance:<10} {:<10} {}",
                self.count(distance),
                self.order_independent_count(distance)
            )?;
        }
        writeln!(f, "  stones measured         {}", self.total())?;
        writeln!(
            f,
            "  order-independent       {}",
            self.order_independent_total()
        )?;
        write!(f, "  MAX distance            {max}")
    }
}
