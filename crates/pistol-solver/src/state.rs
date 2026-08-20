//! [`ThreatState`]: what each side holds in every window, carried incrementally.
//!
//! One stone touches exactly the [`WINDOWS_PER_CELL`](pistol_core::window::WINDOWS_PER_CELL)
//! windows through its cell, less any that run off the addressable lattice.
//! Each touch is one mask bit and at most two moves between maintained sets
//! (see `crate::sets`, which is private), so the whole per-stone cost is bounded
//! by the enumeration and not by the position.
//!
//! # apply then undo leaves the state EQUAL, not merely equivalent
//!
//! [`ThreatState`] is `PartialEq + Eq` over everything it carries, and
//! `apply(c, p)` followed by `undo(c, p)` restores that whole state. What makes
//! that true rather than nearly true is the pruning rule in
//! `WindowTable::set`: an entry exists exactly while its window holds a stone,
//! so an undone stone leaves nothing behind. Without it the table would grow
//! with the search PATH rather than with the position, and two states holding
//! the same stones would compare unequal (docs/decisions.md D-62's rule for the
//! eval's own map, D-214's discipline).
//!
//! # A position the rules have already decided
//!
//! `GameState::place` returns `Ok` on the winning ply, so a search legitimately
//! stands on a decided position and this state tolerates one: `completed_windows`
//! is non-empty exactly there and every other query keeps answering. They answer
//! about a position the rules have already ended, so **a caller that must not
//! stand on one asks `GameState::outcome()`** — this type does not, because a
//! legal search path reaches such a position and rule 3's loudness is for
//! IMPOSSIBLE states, not for decided ones.

use std::collections::BTreeMap;

use pistol_core::window::{Window, windows_through_indexed};
use pistol_core::{Coord, Player};

use crate::sets::{Class, ClassSet, WindowSets};
use crate::table::{WindowMasks, WindowTable};

/// Named invariant: the stones this state was fed and the stones a caller
/// believes it holds have drifted apart.
///
/// The token appears verbatim in the panic message, so a test can pin it and a
/// log can be grepped for it — the same contract `EVAL_DESYNC` carries in
/// `pistol-eval` (CLAUDE.md rule 3, docs/decisions.md D-45).
pub const THREAT_DESYNC: &str = "THREAT_DESYNC";

/// Which side's sets a player's are.
const fn slot(side: Player) -> usize {
    match side {
        Player::P1 => 0,
        Player::P2 => 1,
    }
}

/// The incremental threat state: a per-window occupancy record plus the ten
/// sorted sets every query answers out of.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ThreatState {
    table: WindowTable,
    sets: [WindowSets; 2],
}

impl ThreatState {
    /// An empty state: no stones, no entries, no set members.
    pub fn new() -> ThreatState {
        ThreatState::default()
    }

    /// One stone down.
    ///
    /// # Panics
    ///
    /// With [`THREAT_DESYNC`] if any window through `at` already holds a stone
    /// there. That means this state and the board it is supposed to mirror have
    /// drifted apart, which is a caller bug rather than operator input.
    pub fn apply(&mut self, at: Coord, player: Player) {
        self.touch(at, player, true);
    }

    /// The same stone back up.
    ///
    /// # Panics
    ///
    /// With [`THREAT_DESYNC`] if `player` has no stone at `at` — including the
    /// case where the OTHER player does, which is the same drift by a different
    /// door.
    pub fn undo(&mut self, at: Coord, player: Player) {
        self.touch(at, player, false);
    }

    fn touch(&mut self, at: Coord, player: Player, placing: bool) {
        for (window, index) in windows_through_indexed(at) {
            let before = self.table.masks(window);
            let bit = 1u8 << index;
            if placing {
                assert!(
                    (before.p1 | before.p2) & bit == 0,
                    "{THREAT_DESYNC}: {player} stone on {at} lands on cell {index} of \
                     {window:?}, which already holds one"
                );
            } else {
                assert!(
                    before.own(player) & bit != 0,
                    "{THREAT_DESYNC}: taking back a {player} stone at {at} that cell {index} of \
                     {window:?} does not hold"
                );
            }
            let after = before.with(player, index, placing);

            // BOTH sides, on every window: the stone changes the mover's own
            // count and the opponent's LIVENESS, and a state that updated only
            // the mover's sets would leave a dead window standing in the
            // opponent's hot set (crate::sets).
            for side in [Player::P1, Player::P2] {
                let was = ClassSet::of(before.own_count(side), before.opp_count(side));
                let now = ClassSet::of(after.own_count(side), after.opp_count(side));
                if was != now {
                    self.sets[slot(side)].transition(window, was, now);
                }
            }
            self.table.set(window, after);
        }
    }

    /// What `window` holds, for both sides.
    pub fn masks(&self, window: Window) -> WindowMasks {
        self.table.masks(window)
    }

    /// The windows `side` holds in `class`, sorted by `(axis, start)`.
    pub(crate) fn class_windows(&self, side: Player, class: Class) -> &[Window] {
        self.sets[slot(side)].windows(class)
    }

    /// How many windows hold a stone.
    pub fn window_count(&self) -> usize {
        self.table.len()
    }

    /// Whether no window holds a stone.
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// The whole window table, sorted by window — for oracles and diagnostics.
    ///
    /// Never on a choice path: see `WindowTable::snapshot`, whose doc says why
    /// the table underneath may be hashed at all.
    pub fn table_snapshot(&self) -> BTreeMap<Window, WindowMasks> {
        self.table.snapshot()
    }
}
