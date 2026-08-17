//! Stones on the lattice, and the legal region they create.
//!
//! The board is unbounded, so occupancy is sparse: a map from cell to player,
//! holding only the stones that exist. It is a [`BTreeMap`], ordered by
//! `(q, r)`. That is a determinism decision, not a performance one — a
//! `HashMap` carries per-process random state, and any iteration a caller can
//! observe would then be able to change a move choice between two runs of the
//! same position (CLAUDE.md rule 4, docs/decisions.md D-7, D-32).
//!
//! A `Board` is storage plus one rules question: *is this cell in the legal
//! region* (rule 5). It deliberately does **not** enforce that a position is
//! reachable — [`Board::apply`] refuses an occupied cell and nothing else.
//! Reachability is a property of a *game*, and [`crate::GameState::place`] is
//! the single place that enforces it (docs/decisions.md D-35). Golden fixtures
//! and the legal-region tests need synthetic positions that no game reaches,
//! and a container that refused them would be lying about what it is.

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fmt;

use crate::coord::Coord;
use crate::error::CoreError;
use crate::rules::LEGAL_RADIUS;
use crate::zobrist::{Key128, cell_key};

/// Which side a stone belongs to.
///
/// P1 moves first; the first turn is one stone (rule 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Player {
    /// The side that moves first.
    P1,
    /// The side that moves second.
    P2,
}

impl Player {
    /// The other side.
    pub const fn opponent(self) -> Player {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }

    /// The protocol and fixture spelling of this player.
    pub const fn name(self) -> &'static str {
        match self {
            Player::P1 => "p1",
            Player::P2 => "p2",
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The stones on the lattice.
///
/// The zobrist key of those stones is carried alongside them. It is a pure
/// function of the map, so it changes nothing about what a board *is* — two
/// boards are equal exactly when their stones are — but it is maintained here
/// because this is the one place a stone moves (docs/decisions.md D-41).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    stones: BTreeMap<Coord, Player>,
    key: Key128,
}

impl Board {
    /// A board with no stones. The legal region of one is the origin alone
    /// (rules 3 and 5).
    pub fn empty() -> Self {
        Board {
            stones: BTreeMap::new(),
            key: Key128::ZERO,
        }
    }

    /// The player of the stone on `at`, if any.
    pub fn get(&self, at: Coord) -> Option<Player> {
        self.stones.get(&at).copied()
    }

    /// Whether `at` holds a stone of either player.
    pub fn is_occupied(&self, at: Coord) -> bool {
        self.stones.contains_key(&at)
    }

    /// Whether the board holds no stones at all.
    pub fn is_empty(&self) -> bool {
        self.stones.is_empty()
    }

    /// How many stones are on the board.
    pub fn stone_count(&self) -> usize {
        self.stones.len()
    }

    /// Every stone, in ascending `(q, r)` order.
    ///
    /// The order is part of the contract: it is what makes two runs over the
    /// same position agree (CLAUDE.md rule 4).
    pub fn stones(&self) -> impl Iterator<Item = (Coord, Player)> + '_ {
        self.stones.iter().map(|(&at, &player)| (at, player))
    }

    /// Put a stone on an empty cell.
    ///
    /// This is the storage operation. It refuses an occupied cell and asks
    /// nothing else: whether the cell is *legal* is [`Board::is_legal_placement`],
    /// and whether the resulting position is *reachable* is
    /// [`crate::GameState::place`]'s question (docs/decisions.md D-35).
    pub fn apply(&mut self, at: Coord, player: Player) -> Result<(), CoreError> {
        match self.stones.entry(at) {
            Entry::Occupied(_) => Err(CoreError::OccupiedCell { at }),
            Entry::Vacant(slot) => {
                slot.insert(player);
                self.key ^= cell_key(at, player);
                Ok(())
            }
        }
    }

    /// Take the stone off `at`, returning its player.
    pub fn undo(&mut self, at: Coord) -> Result<Player, CoreError> {
        let player = self
            .stones
            .remove(&at)
            .ok_or(CoreError::UnoccupiedCell { at })?;
        // The same XOR that put it there takes it back out; a refused undo
        // above has not touched the map, so the key still describes it.
        self.key ^= cell_key(at, player);
        Ok(player)
    }

    /// The zobrist key of the stones: one [`cell_key`] per stone, XORed.
    ///
    /// This is the part of a position key that is about stones, and all of the
    /// part that costs anything to compute. Whose move it is and how far into
    /// the turn they are is not the board's to know —
    /// [`crate::GameState::key`] composes the whole key from this and those
    /// (docs/decisions.md D-58).
    pub fn stones_key(&self) -> Key128 {
        self.key
    }

    /// Whether `at` is in the legal region (rule 5).
    ///
    /// The region is the **union** of the radius-[`LEGAL_RADIUS`] balls around
    /// every stone — within eight of *some* stone, not of any particular one,
    /// and not of their centre. On an empty board the region is the origin
    /// alone: the first turn is one stone and the lattice is homogeneous, so
    /// the origin is where it goes (rule 3).
    ///
    /// This asks nothing about occupancy; an occupied cell is still in the
    /// region, and it asks nothing about player — rule 5 says *an existing
    /// stone*, not an own one. [`Board::is_legal_placement`] is the question a
    /// mover asks.
    ///
    /// The probe is linear in the stones on the board. That is the honest cost
    /// of asking about one arbitrary cell, and it is not how the search will
    /// ask: candidate generation (WP-03) enumerates the balls around the stones
    /// rather than probing every cell, so it never pays this per candidate.
    pub fn in_legal_region(&self, at: Coord) -> bool {
        if self.is_empty() {
            return at == Coord::ORIGIN;
        }
        self.stones
            .keys()
            .any(|&stone| stone.distance(at) <= LEGAL_RADIUS)
    }

    /// Whether a stone may be placed on `at`: in the legal region, and empty.
    ///
    /// The predicate and the named refusal are the same test, asked twice; see
    /// [`Board::check_placement`].
    pub fn is_legal_placement(&self, at: Coord) -> bool {
        self.check_placement(at).is_ok()
    }

    /// The named refusal for `at`, or `Ok(())` if a stone may be placed there.
    ///
    /// Two rules refuse a cell for two different reasons and say so: an empty
    /// board refuses anything but the origin under rule 3
    /// ([`CoreError::FirstStoneNotAtOrigin`]), and a board with stones on it
    /// refuses anything beyond [`LEGAL_RADIUS`] of all of them under rule 5
    /// ([`CoreError::OutsideLegalRegion`]). Collapsing the two into one message
    /// would make the empty-board diagnostic a lie about which rule bit.
    pub fn check_placement(&self, at: Coord) -> Result<(), CoreError> {
        if self.is_occupied(at) {
            return Err(CoreError::OccupiedCell { at });
        }
        if self.is_empty() {
            if at != Coord::ORIGIN {
                return Err(CoreError::FirstStoneNotAtOrigin { at });
            }
            return Ok(());
        }
        if !self.in_legal_region(at) {
            return Err(CoreError::OutsideLegalRegion { at });
        }
        Ok(())
    }
}
