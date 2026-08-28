use std::collections::BTreeMap;
use std::fmt;

use pistol_core::{Coord, FIRST_TURN, stones_in_turn};

use super::record::Record;

/// The radius SB-65's log note claims the platform enforces, and which this
/// histogram exists to test (docs/research/sealbot_deep_dive.md, D-101).
///
/// Named rather than written inline at the one place it is reported, so the
/// printed line says which hypothesis its count is about. It is not a tunable
/// and nothing reads it as one: the game's own radius is
/// [`pistol_core::LEGAL_RADIUS`], which this module never compares anything to.
pub const CLAIMED_RADIUS: u32 = 6;

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
        // The headline the adjudication rests on, printed rather than left to be
        // hand-summed off the rows above: transcribing a total out of a rendered
        // block by eye is the path that carried this module's earlier false
        // claim (docs/decisions.md D-219).
        writeln!(
            f,
            "  order-independent beyond {CLAIMED_RADIUS}   {}  <- refutes radius {CLAIMED_RADIUS}",
            self.unrescuable_beyond(CLAIMED_RADIUS)
        )?;
        write!(f, "  MAX distance            {max}")
    }
}
