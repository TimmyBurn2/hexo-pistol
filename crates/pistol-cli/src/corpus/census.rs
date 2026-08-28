use std::collections::BTreeMap;
use std::fmt;

use pistol_core::symmetry::{Symmetry, canonical_form, canonical_sequence};

use pistol_core::{Coord, FIRST_TURN, Player, Turn};

use super::record::Record;
use super::replay::group_turns;

/// Where the early-position key is cut, as a turn boundary rather than a raw
/// stone count.
///
/// Turn 6 is the boundary at eleven stones, which is the size of the WP-P1
/// witness whose three-way symmetry ambiguity D-439 is about (docs/decisions.md
/// D-439). Cutting on a boundary rather than mid-turn keeps the key a position
/// some player actually faced.
pub const EARLY_TURNS: u32 = 6;

/// A game's three symmetry-invariant keys.
///
/// The position keys merge two games that reach the same stones; the sequence
/// key does not. Keeping both is what discriminates D-439's hypotheses: an
/// orientation-normalization duplicate shares the SEQUENCE key, while a genuine
/// line repetition shares only an early POSITION key and then diverges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keys {
    /// Canonical stone-set at the final position.
    pub final_position: Vec<(Coord, Player)>,
    /// Canonical stone-set at the [`EARLY_TURNS`] boundary, absent where the
    /// game is shorter than that.
    pub early_position: Option<Vec<(Coord, Player)>>,
    /// Canonical turn sequence over the whole game.
    pub sequence: Vec<Turn>,
    /// The turns in play order, uncanonicalized.
    ///
    /// A Sequence class must be described on THESE, not on a stone set: two
    /// games can reach the same board by different move orders, so a describer
    /// handed boards answers "are these the same position" for a class that
    /// asked "are these the same game" (WP-P1b REVIEW-impl F1, RED-TEAM
    /// MAJOR-1).
    pub turns: Vec<Turn>,
    /// The whole game's stones with their owners, uncanonicalized.
    pub stones: Vec<(Coord, Player)>,
    /// The [`EARLY_TURNS`] prefix's stones, uncanonicalized.
    ///
    /// Kept separately from [`Keys::stones`] because a class is described on
    /// the same stones it was keyed on: describing an early-position class by
    /// its members' WHOLE games compares boards no key ever looked at, and
    /// reports every such class as needing a non-trivial element.
    pub early_stones: Option<Vec<(Coord, Player)>>,
}

/// Who owns each turn: turn 1 is P1's, and the sides alternate (rule 3).
fn owner_of(offset: usize) -> Player {
    if offset.is_multiple_of(2) {
        Player::P1
    } else {
        Player::P2
    }
}

/// Named refusals of a single game.
pub const TURN_GROUPING_REFUSED: &str = "the turn structure does not group this game's stones";
/// See [`TURN_GROUPING_REFUSED`].
pub const OFF_ADDRESSABLE_LATTICE: &str = "a stone leaves the addressable lattice under some symmetry, so this game has no orbit to \
     take a minimum over";

/// The keys for one game, or `None` where the turn grouping refuses it.
///
/// The grouping is [`group_turns`], not a second walk of the turn structure:
/// a census that grouped turns its own way would be keyed on a different game
/// than every other corpus measurement (CLAUDE.md rule 2).
pub fn keys_of(moves: &[Coord]) -> Option<Keys> {
    keys_or_refusal(moves, EARLY_TURNS).ok()
}

/// The keys for one game, or the named reason there are none.
///
/// # Errors
///
/// [`TURN_GROUPING_REFUSED`] or [`OFF_ADDRESSABLE_LATTICE`]. The second is a
/// refusal rather than the panic `Symmetry::apply` would raise: the reader's
/// guard is the i16 range and a rotation's domain is strictly narrower, so a
/// schema-valid corpus can reach it (CLAUDE.md rule 3).
pub fn keys_or_refusal(moves: &[Coord], early_turns: u32) -> Result<Keys, &'static str> {
    for stone in moves {
        for symmetry in Symmetry::ALL {
            if symmetry.checked_apply(*stone).is_none() {
                return Err(OFF_ADDRESSABLE_LATTICE);
            }
        }
    }
    let grouped = group_turns(moves).map_err(|_| TURN_GROUPING_REFUSED)?;
    let mut stones: Vec<(Coord, Player)> = Vec::with_capacity(moves.len());
    let mut turns: Vec<Turn> = Vec::with_capacity(grouped.len());
    let mut early: Option<Vec<(Coord, Player)>> = None;
    let mut early_raw: Option<Vec<(Coord, Player)>> = None;

    for (offset, entry) in grouped.iter().enumerate() {
        let player = owner_of(offset);
        stones.push((entry.turn.first(), player));
        if let Some(second) = entry.turn.second() {
            stones.push((second, player));
        }
        turns.push(entry.turn);
        if FIRST_TURN + offset as u32 == early_turns {
            early = Some(canonical_form(&stones));
            early_raw = Some(stones.clone());
        }
    }

    Ok(Keys {
        final_position: canonical_form(&stones),
        early_position: early,
        sequence: canonical_sequence(&turns),
        turns,
        stones,
        early_stones: early_raw,
    })
}

/// One set of games sharing a key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    /// Corpus indices of the members, ascending.
    pub members: Vec<usize>,
    /// Whether every member is already equal under the identity element, so the
    /// class needs no symmetry to exist.
    pub identical_under_identity: bool,
    /// The non-identity elements that carry the first member onto each other
    /// member, empty where a member is only reachable under the identity.
    pub elements: Vec<Symmetry>,
}

/// Which key a set of classes was built on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Whole-game turn sequence: the orientation-normalization test.
    Sequence,
    /// Final stone-set: two games that ended on the same board.
    FinalPosition,
    /// Stone-set at the [`EARLY_TURNS`] boundary: the shared-opening test.
    EarlyPosition,
    /// Early stone-set with colour discarded.
    ///
    /// The WP-P1 mapping gate compared UNCOLOURED coordinate sets, because who
    /// owns a stone was a separate unknown at that point (D-437). A census that
    /// only keyed coloured sets would not be measuring the coincidence D-439 is
    /// about.
    EarlyPositionUncoloured,
    /// Final stone-set with colour discarded.
    FinalPositionUncoloured,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word = match self {
            Kind::Sequence => "canonical sequence (whole game)",
            Kind::FinalPosition => "canonical final position",
            Kind::EarlyPosition => "canonical early position",
            Kind::EarlyPositionUncoloured => "canonical early position, colour discarded",
            Kind::FinalPositionUncoloured => "canonical final position, colour discarded",
        };
        write!(f, "{word}")
    }
}

/// The classes of size two or more under one key, and what they are made of.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Census {
    /// Which key this census is on.
    pub kind: Kind,
    /// How many games carried a key at all.
    pub keyed: usize,
    /// Classes of size >= 2, in ascending order of their least member.
    pub classes: Vec<Class>,
}

impl Census {
    /// Group games by one key and keep the collisions.
    ///
    /// `key_of` returns `None` for a game this key does not apply to, which is
    /// counted out rather than bucketed under a shared absent key.
    pub fn build<K, R, F, I>(kind: Kind, keys: &[Option<Keys>], key_of: F, image_of: I) -> Census
    where
        K: Ord,
        R: Eq,
        F: Fn(&Keys) -> Option<K>,
        I: Fn(&Keys, Symmetry) -> R,
    {
        let mut buckets: BTreeMap<K, Vec<usize>> = BTreeMap::new();
        let mut keyed = 0usize;
        for (index, entry) in keys.iter().enumerate() {
            let Some(entry) = entry else { continue };
            let Some(key) = key_of(entry) else { continue };
            keyed += 1;
            buckets.entry(key).or_default().push(index);
        }

        let mut classes: Vec<Class> = buckets
            .into_values()
            .filter(|members| members.len() > 1)
            .map(|members| describe(&members, keys, &image_of))
            .collect();
        classes.sort_by_key(|class| class.members[0]);
        Census {
            kind,
            keyed,
            classes,
        }
    }

    /// How many games sit in a class of size two or more.
    pub fn colliding_games(&self) -> usize {
        self.classes.iter().map(|class| class.members.len()).sum()
    }

    /// Class sizes with how many classes have that size, ascending by size.
    pub fn size_distribution(&self) -> BTreeMap<usize, usize> {
        let mut sizes: BTreeMap<usize, usize> = BTreeMap::new();
        for class in &self.classes {
            *sizes.entry(class.members.len()).or_insert(0) += 1;
        }
        sizes
    }
}

/// Whether a class needs a symmetry to exist, and which elements carry it.
///
/// `image_of` must produce the SAME representation the class was keyed on. A
/// describer given any other shape adjudicates a different question than the
/// key asked, which has been this module's recurring defect.
fn describe<R, I>(members: &[usize], keys: &[Option<Keys>], image_of: &I) -> Class
where
    R: Eq,
    I: Fn(&Keys, Symmetry) -> R,
{
    let image = |index: usize, symmetry: Symmetry| {
        keys[index].as_ref().map(|entry| image_of(entry, symmetry))
    };
    let first = image(members[0], Symmetry::IDENTITY);
    let mut identical = true;
    let mut elements: Vec<Symmetry> = Vec::new();
    for &member in &members[1..] {
        let other = image(member, Symmetry::IDENTITY);
        if other != first {
            identical = false;
            // Only a member that is NOT already equal needs a carrying element;
            // crediting one to an exact repeat overstates what the class is.
            for symmetry in Symmetry::ALL {
                if symmetry != Symmetry::IDENTITY && image(members[0], symmetry) == other {
                    elements.push(symmetry);
                    break;
                }
            }
        }
    }
    elements.sort_unstable();
    elements.dedup();
    Class {
        members: members.to_vec(),
        identical_under_identity: identical,
        elements,
    }
}

impl fmt::Display for Census {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  key: {}", self.kind)?;
        writeln!(f, "  games keyed             {}", self.keyed)?;
        writeln!(f, "  classes of size >= 2    {}", self.classes.len())?;
        writeln!(f, "  games in such a class   {}", self.colliding_games())?;
        if self.classes.is_empty() {
            return write!(f, "  no two games share this key");
        }
        writeln!(f, "  class size   how many classes")?;
        for (size, count) in self.size_distribution() {
            writeln!(f, "  {size:<12} {count}")?;
        }
        let identical = self
            .classes
            .iter()
            .filter(|class| class.identical_under_identity)
            .count();
        writeln!(
            f,
            "  classes equal under the identity element    {identical}"
        )?;
        write!(
            f,
            "  classes needing a non-trivial element        {}",
            self.classes.len() - identical
        )
    }
}

/// The same stones with every owner collapsed to one, NOT canonicalized.
///
/// What [`Census::build`] hands to the class describer must be the same
/// representation the key was built from, or the describer compares boards the
/// key never looked at and reports no carrying element for a class that has one.
pub fn flatten(stones: &[(Coord, Player)]) -> Vec<(Coord, Player)> {
    stones.iter().map(|&(at, _)| (at, Player::P1)).collect()
}

/// The same stones with every owner collapsed to one.
///
/// Reuses [`canonical_form`] rather than growing a colour-blind twin of it: the
/// canonicalization is the part that must not have two implementations.
pub fn uncoloured(stones: &[(Coord, Player)]) -> Vec<(Coord, Player)> {
    canonical_form(&flatten(stones))
}

/// Every game's keys, in corpus order.
pub fn keys_of_all(records: &[Record]) -> Vec<Option<Keys>> {
    keys_of_all_at(records, EARLY_TURNS)
}

/// Every game's keys with the early cut taken at `early_turns`.
pub fn keys_of_all_at(records: &[Record], early_turns: u32) -> Vec<Option<Keys>> {
    records
        .iter()
        .map(|record| keys_or_refusal(&record.moves, early_turns).ok())
        .collect()
}
