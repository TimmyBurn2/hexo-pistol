use pistol_core::window::Window;
use pistol_core::{Coord, GameState, Player};

use crate::sets::Class;
use crate::state::ThreatState;
use crate::table::empty_cells;

/// How many stones the mover may still place THIS turn.
///
/// Closed, because rule 3 admits one or two and nothing else. A `u32` here
/// invites 0 and 3, and the answer at zero stones is not "no threats" but "not
/// a question about this turn" (docs/decisions.md D-257).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StonesLeft {
    /// One stone left: the mover is at `Phase::Second`, or it is turn 1.
    One,
    /// Two stones left: the mover is at `Phase::First` of a later turn.
    Two,
}

impl StonesLeft {
    /// What `state` owes this turn — read from `GameState::stones_owed`, which
    /// is pistol-core's own answer, never re-derived here (CLAUDE.md rule 2).
    ///
    /// `None` on a DECIDED position, where `stones_owed` is 0: the game ended
    /// and the turn owes nothing (rule 4). Deriving `stones_in_turn(turn) -
    /// phase.index()` instead would answer `Two` there, because `place` freezes
    /// the state with `phase` back at `First` — a phase that is no longer being
    /// played. `StonesLeft` is closed at two and there is no honest third value,
    /// so the absence is in the return type rather than invented (rule 3,
    /// docs/decisions.md D-257).
    ///
    /// # Panics
    ///
    /// If `stones_owed` is neither 0, 1 nor 2. Unreachable through core's own
    /// turn structure, and named rather than silently mapped, because a fourth
    /// answer would mean rule 3 had changed under this type.
    pub fn from_state(state: &GameState) -> Option<StonesLeft> {
        match state.stones_owed() {
            0 => None,
            1 => Some(StonesLeft::One),
            2 => Some(StonesLeft::Two),
            other => panic!("a turn owing {other} stones is not a turn this game has"),
        }
    }
}

/// What a hitting-set question allows a cover to spend.
///
/// Zero is a real question — "does anything remain to block?" — and it is why
/// this is a DIFFERENT type from [`StonesLeft`]: conflating "stones the turn
/// owes" with "cells a cover may use" is the class of error this pair exists to
/// prevent. Both are closed at two because the enumeration in [`crate::cover`]
/// is exact only for budgets of at most two over families of at most two cells;
/// an open integer would carry a promise of exactness it cannot keep.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HitBudget {
    /// No stones: only the empty set is available, and it covers nothing.
    Zero,
    /// One stone.
    One,
    /// Two stones.
    Two,
}

impl From<StonesLeft> for HitBudget {
    fn from(left: StonesLeft) -> HitBudget {
        match left {
            StonesLeft::One => HitBudget::One,
            StonesLeft::Two => HitBudget::Two,
        }
    }
}

/// A live-window count that is maintained and is not its own query.
///
/// Closed at `{Two, Three}`: count 1 is not maintained and count 1 is therefore
/// not askable — a compile error rather than a run-time refusal
/// (docs/decisions.md D-255). Hot and win-in-one-ply have queries of their own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiveCount {
    /// Exactly two own stones, no opponent stone.
    Two,
    /// Exactly three own stones, no opponent stone.
    Three,
}

/// The one live count that is a single stone short of HOT.
///
/// Hot is `own >= 4`, so exactly one count — three — is one stone away from it,
/// and the type says so rather than the doc. A `LiveCount::Two` here has no
/// honest answer: no single cell raises a count-2 window to hot, so the query
/// would return an empty list, which reads as "no such cell in this position"
/// instead of "not a question a single cell can answer" (rule 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NearHot {
    /// Three own stones: one more makes the window hot.
    Three,
}

/// How a side completes six THIS TURN.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WinWitness {
    /// One stone completes a window already holding five. Valid at either
    /// budget.
    OnePly {
        /// The cell to play.
        at: Coord,
        /// The window it completes.
        window: Window,
    },
    /// Two stones complete a window holding four. Valid ONLY at
    /// [`StonesLeft::Two`] — D-243's phase conditioning.
    Pair {
        /// The lexicographically lesser cell.
        first: Coord,
        /// The greater one, so `Turn::pair(first, second)` is canonical without
        /// the caller reordering.
        second: Coord,
        /// The window the pair completes.
        window: Window,
    },
}

impl ThreatState {
    /// `side`'s HOT windows: live, four or more own stones (D-243).
    pub fn hot_windows(&self, side: Player) -> &[Window] {
        self.class_windows(side, Class::Hot)
    }

    /// `side`'s WIN-IN-ONE-PLY windows: live, exactly five own stones (D-243).
    pub fn win_in_one_ply_windows(&self, side: Player) -> &[Window] {
        self.class_windows(side, Class::WinInOnePly)
    }

    /// The windows `side` has filled: a completed run of six or more (rule 2).
    pub fn completed_windows(&self, side: Player) -> &[Window] {
        self.class_windows(side, Class::Completed)
    }

    /// `side`'s live windows at exactly `count` own stones.
    pub fn live_windows_at_count(&self, side: Player, count: LiveCount) -> &[Window] {
        let class = match count {
            LiveCount::Two => Class::LiveTwo,
            LiveCount::Three => Class::LiveThree,
        };
        self.class_windows(side, class)
    }

    /// The empty cells of `side`'s win-in-one-ply windows: each completes six
    /// with a SINGLE stone.
    pub fn win_in_one_ply_cells(&self, side: Player, out: &mut Vec<Coord>) {
        self.fill_empties(self.win_in_one_ply_windows(side), out);
    }

    /// The union of the empty cells of `side`'s HOT windows.
    ///
    /// One defender stone on such a cell kills the window it came from, and NO
    /// SINGLE CELL need kill them all: which cells suffice TOGETHER is
    /// [`ThreatState::blocking_covers`]' question, not this one.
    ///
    /// These are the cells of windows that are ALREADY hot;
    /// [`ThreatState::cells_raising_to_hot`] gives the cells that would MAKE one
    /// hot. They are different sets and neither is the other.
    pub fn threat_cells(&self, side: Player, out: &mut Vec<Coord>) {
        self.fill_empties(self.hot_windows(side), out);
    }

    /// The cells that raise a live count-`count` window of `side` to HOT with
    /// one stone: the deduplicated union of those windows' empties.
    ///
    /// The union, and not "each window's empties": a side with four count-three
    /// windows gets one set back, and no single window's three empties equal it.
    pub fn cells_raising_to_hot(&self, side: Player, count: NearHot, out: &mut Vec<Coord>) {
        let windows = match count {
            NearHot::Three => self.live_windows_at_count(side, LiveCount::Three),
        };
        self.fill_empties(windows, out);
    }

    /// The empty cells of `side`'s live windows at exactly `count` own stones:
    /// the deduplicated union, not any one window's own empties.
    ///
    /// `LAW-SUPPORT`'s k=2 qualification (`docs/decisions.md` D-267, D-352):
    /// WP-1.5b's Tier T reads these windows' own empties directly, which is a
    /// different question from [`ThreatState::cells_raising_to_hot`] — that
    /// query answers which cell would MAKE a window hot, this one answers which
    /// cells a window ALREADY LICENSES under `LAW-SUPPORT`. The two happen to
    /// coincide at [`LiveCount::Three`], because [`NearHot::Three`] and
    /// [`LiveCount::Three`] name the same windows; there is no such coincidence
    /// at [`LiveCount::Two`], which [`NearHot`] cannot even express (it is closed
    /// at `Three`, D-243's one-stone-short reading).
    pub fn live_cells_at_count(&self, side: Player, count: LiveCount, out: &mut Vec<Coord>) {
        self.fill_empties(self.live_windows_at_count(side, count), out);
    }

    /// True, with a witness, iff `side` moving now with `left` stones can
    /// complete six THIS TURN. D-243's phase conditioning is in the signature.
    ///
    /// SELECTION IS TOTAL AND WRITTEN DOWN, because rule 4 wants a stable
    /// tie-break stated rather than emergent:
    ///
    /// 1. `OnePly` whenever any win-in-one-ply cell exists — it is available at
    ///    either budget and spends one stone.
    /// 2. Among `OnePly` candidates, the lexicographically least cell; then,
    ///    among the win-in-one-ply windows sharing that cell, the least by
    ///    `(axis, start)`. THE SECOND HALF IS NOT DECORATION: a cell can be the
    ///    single empty of more than one five-window — measured at about 4 % of
    ///    plies over the registered playout regime — and this witness carries a
    ///    window, so without it two conforming implementations differ on a field
    ///    an oracle compares.
    /// 3. Otherwise, at [`StonesLeft::Two`] only, among live windows with
    ///    EXACTLY four own stones — hence exactly two empties — the least by
    ///    `(axis, start)`, and that window's two empties in lexicographic order.
    ///
    /// The witness is a witness and not advice: a caller wanting the best of
    /// several wins asks for the sets and ranks them itself.
    pub fn can_win_this_turn(&self, side: Player, left: StonesLeft) -> Option<WinWitness> {
        let mut best: Option<(Coord, Window)> = None;
        for &window in self.win_in_one_ply_windows(side) {
            let at = empty_cells(window, self.masks(window))
                .next()
                .expect("a live window with five own stones has one empty cell");
            let candidate = (at, window);
            if best.is_none_or(|current| candidate < current) {
                best = Some(candidate);
            }
        }
        if let Some((at, window)) = best {
            return Some(WinWitness::OnePly { at, window });
        }
        if left == StonesLeft::One {
            return None;
        }
        // The hot set is sorted, so the first window with exactly four own
        // stones is the least such by `(axis, start)`.
        let window = *self
            .hot_windows(side)
            .iter()
            .find(|&&window| self.masks(window).own_count(side) == 4)?;
        let mut empties = empty_cells(window, self.masks(window));
        let first = empties
            .next()
            .expect("a hot window at four has two empties");
        let second = empties
            .next()
            .expect("a hot window at four has two empties");
        Some(WinWitness::Pair {
            first: first.min(second),
            second: first.max(second),
            window,
        })
    }

    /// The deduplicated, sorted empty cells of `windows`.
    fn fill_empties(&self, windows: &[Window], out: &mut Vec<Coord>) {
        out.clear();
        for &window in windows {
            out.extend(empty_cells(window, self.masks(window)));
        }
        out.sort_unstable();
        out.dedup();
    }
}
