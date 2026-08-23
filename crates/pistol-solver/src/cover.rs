//! The cover arithmetic: which cells, TOGETHER, answer a set of hot windows.
//!
//! Let `H` be one side's hot windows and `E_w` the empty cells of `w ∈ H`.
//! Every `E_w` holds at most two cells, because hot means at least four own
//! stones of six (D-243), and a budget is at most two. Hitting set is NP-hard
//! in general; it is not hard here, because `|E_w| <= 2` makes it vertex cover
//! and a budget of at most two makes vertex cover a finite enumeration. That is
//! why [`HitBudget`] is closed at two: an open integer would carry a promise of
//! exactness this enumeration cannot keep.
//!
//! `Cover` has three answers and they are not interchangeable — nothing to
//! block, no cover within budget, and a non-empty list of minimal covers
//! (docs/decisions.md D-257).
//!
//! # INCLUSION-MINIMAL is the notion, and it is load-bearing
//!
//! Under the standard reading any superset of a cover is a cover, so beside a
//! one-cell cover `{c}` with two stones in hand, `{c, x}` covers for every `x`
//! and the union of cells over "all covers of size <= 2" is the whole legal
//! region — which restricts nothing. `{c}` is minimal; `{c, x}` is not. Minimal
//! covers still deliver what D-243 consequence (4) exists for: two hot windows
//! needing different cells give a minimal `{a, b}` with `a` in one and `b` in
//! the other, so the two-stone CROSS-WINDOW split stays reachable at ply 1.
//!
//! # And a flat cell list is provably insufficient
//!
//! Three hot windows with empties `{a, b}`, `{b, d}`, `{d, e}` have no one-cell
//! cover and three minimal two-cell covers; `{a, e}` is drawn from the same cell
//! union and covers nothing in the middle. A defender handed the flat list has
//! to guess which cells go together, which is why [`Cover::Minimal`] carries
//! SETS and why [`Cover::cells`] warns in its own doc.
//!
//! # RULE9-JUSTIFICATION: one arithmetic, its early-out, and the inline unit
//! tests that pin the early-out alone (CLAUDE.md rule 9).
//!
//! `three_pairwise_disjoint_families` (docs/decisions.md D-263, D-363) is a
//! necessary-condition check for the exact same `Impossible`/`exceeds` answer
//! [`ThreatState::blocking_covers`] and [`ThreatState::min_hitting_set_exceeds`]
//! already compute the slow way — splitting it into its own file would put a
//! guard clause and the arithmetic it guards on opposite sides of a module
//! boundary for no reader's benefit. The five `#[cfg(test)]` cases pin that one
//! private helper directly, which the crate's public surface cannot: this
//! module's own doc names family size and budget as bounded types the public
//! functions already close over, so a private pure-function unit test is the
//! only place a "three disjoint families" shape is stated as itself rather than
//! folded into a golden fixture's game position. Splits when a second
//! early-out earns its own guard.

use pistol_core::window::Window;
use pistol_core::{Coord, Player};

use crate::query::HitBudget;
use crate::state::ThreatState;
use crate::table::empty_cells;

/// One inclusion-minimal cover: cells that together hit every hot window, and
/// from which no cell can be dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MinimalCover {
    /// One cell hits every hot window on its own.
    One(Coord),
    /// Two cells that hit every hot window and neither of which hits them all
    /// alone. This is a CROSS-WINDOW pair, which is the case a flat cell list
    /// cannot carry.
    Two {
        /// The lexicographically lesser cell.
        first: Coord,
        /// The greater one.
        second: Coord,
    },
}

impl MinimalCover {
    /// The cells this cover uses.
    pub fn cells(self) -> Vec<Coord> {
        match self {
            MinimalCover::One(at) => vec![at],
            MinimalCover::Two { first, second } => vec![first, second],
        }
    }
}

/// What a defender can do about an attacker's hot windows within a budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cover {
    /// The attacker has no hot window. There is nothing to block, and this is
    /// NOT the same as "no cell blocks" — an empty list would read as the
    /// opposite, and rule 3 wants the two distinguishable.
    NothingToBlock,
    /// No cover of size at most the budget exists. THIS is the unblockable
    /// case. At [`HitBudget::Zero`] it is the answer whenever any hot window
    /// stands, the only cover of size zero being the empty set.
    Impossible,
    /// Every inclusion-minimal cover within budget, sorted and NON-EMPTY.
    /// `Minimal(vec![])` is forbidden by construction: no cover of that size
    /// existing IS [`Cover::Impossible`], and a caller matching on `Minimal`
    /// must never have to test emptiness to learn which answer it got.
    Minimal(Vec<MinimalCover>),
}

impl Cover {
    /// The union of cells appearing in some minimal cover, sorted.
    ///
    /// A CONVENIENCE, and a trap if used alone: a generator handed only these
    /// cells must still guess which pairs go together, and one that pairs within
    /// a single window misses every cross-window split. See this module's doc
    /// for the counterexample.
    pub fn cells(&self) -> Vec<Coord> {
        let mut cells: Vec<Coord> = match self {
            Cover::NothingToBlock | Cover::Impossible => Vec::new(),
            Cover::Minimal(covers) => covers.iter().flat_map(|cover| cover.cells()).collect(),
        };
        cells.sort_unstable();
        cells.dedup();
        cells
    }
}

impl ThreatState {
    /// Whether the minimum hitting set of `windows`' empty cells is LARGER than
    /// `budget`.
    ///
    /// Exact and bounded. The cases, in the order they are decided:
    ///
    /// ```text
    /// windows is empty     -> false   // the empty set covers it; nothing exceeds 0
    /// some window has no empty -> true    // a completed window cannot be hit at all
    /// three families pairwise disjoint -> true // no budget here exceeds two (D-263)
    /// Zero                 -> true    // some window stands unhit
    /// One                  -> no cell lies in every window's empties
    /// Two                  -> no pair of cells hits every window
    /// ```
    ///
    /// THE EMPTY-FAMILY CASE IS FIRST AND IT IS THE ONE THAT MATTERS
    /// (docs/decisions.md D-257): the minimum hitting set of an empty family is
    /// zero, so it exceeds nothing, at every budget. Scanning for a cell instead finds none and answers "true",
    /// which claims an unhittable threat for a side that owns no hot window —
    /// a mate score for the wrong side, arrived at by arithmetic rather than by
    /// a missing guard.
    ///
    /// Total over ADDRESSABLE windows not in this state's table: such a window
    /// has six empties and hits nothing, and since adding a window adds a
    /// CONSTRAINT, the minimum hitting set is monotone non-decreasing and an
    /// extra window can only push this predicate toward `true`.
    ///
    /// ADDRESSABLE IS THE WHOLE OF THE QUALIFICATION AND IT IS NOT DECORATION.
    /// `Window`'s fields are public in `pistol-core`, so `Window::new`'s refusal
    /// belongs to that constructor and not to the type, and a window written
    /// down at the far end of the lattice names cells that do not exist: reading
    /// its empties panics with `COORD_OVERFLOW` through `Coord::step` rather
    /// than answering. The earlier wording promised totality over windows
    /// SIMPLICITER, which this is not and cannot be while the fields are public.
    /// Nothing in this crate can reach that case — every caller passes
    /// [`ThreatState::hot_windows`], whose members were entered by
    /// `pistol_core::window`'s own enumeration, and the two doors that could
    /// produce a window otherwise, the packed key's `unpack` and `empty_cells`
    /// itself, are both crate-private — which the crate root turns into
    /// compile-fail examples rather than leaving as prose, since a sentence
    /// about visibility is falsified by any commit that re-publishes what it
    /// names. The one door that guard does not cover is named there
    /// (docs/decisions.md D-261).
    pub fn min_hitting_set_exceeds(&self, budget: HitBudget, windows: &[Window]) -> bool {
        if windows.is_empty() {
            return false;
        }
        let families = self.empty_families(windows);
        if families.iter().any(|empties| empties.is_empty()) {
            return true;
        }
        // D-263's registered remedy 1: three families with pairwise-disjoint
        // empties need three cells to hit, which exceeds every `HitBudget` this
        // type has (closed at two) — skip the O(|universe|^2) enumeration below
        // for a case it can only ever answer `true` for.
        if three_pairwise_disjoint_families(&families) {
            return true;
        }
        match budget {
            HitBudget::Zero => true,
            HitBudget::One => !universe(&families)
                .into_iter()
                .any(|at| covers(&families, at, None)),
            HitBudget::Two => {
                let universe = universe(&families);
                // `second == first` is deliberately included: it is the size-one
                // case, which is also a cover of size at most two.
                !universe.iter().enumerate().any(|(index, &first)| {
                    universe[index..]
                        .iter()
                        .any(|&second| covers(&families, first, Some(second)))
                })
            }
        }
    }

    /// The minimal covers `defender` may play to block the attacker's hot
    /// windows with `budget` stones.
    ///
    /// Returns SETS rather than a cell list, for the reason in this module's
    /// doc. The attacker is `defender`'s opponent: this answers "what can I do
    /// about what is aimed at me".
    pub fn blocking_covers(&self, defender: Player, budget: HitBudget) -> Cover {
        let windows = self.hot_windows(defender.opponent());
        if windows.is_empty() {
            return Cover::NothingToBlock;
        }
        if budget == HitBudget::Zero {
            return Cover::Impossible;
        }
        let families = self.empty_families(windows);
        // Same early-out as `min_hitting_set_exceeds`, and for the same reason:
        // three pairwise-disjoint families need three cells, which is more than
        // `budget` ever holds here.
        if three_pairwise_disjoint_families(&families) {
            return Cover::Impossible;
        }
        let universe = universe(&families);
        let mut minimal = Vec::new();
        // Any one-cell cover is automatically inclusion-minimal: the empty set
        // covers nothing while a hot window stands.
        for &at in &universe {
            if covers(&families, at, None) {
                minimal.push(MinimalCover::One(at));
            }
        }
        if budget == HitBudget::Two {
            for (index, &first) in universe.iter().enumerate() {
                for &second in &universe[index + 1..] {
                    // "and neither covers alone" is the whole of minimality
                    // here: without it every superset of a one-cell cover comes
                    // back too, and the answer degenerates into the universe.
                    if covers(&families, first, Some(second))
                        && !covers(&families, first, None)
                        && !covers(&families, second, None)
                    {
                        minimal.push(MinimalCover::Two { first, second });
                    }
                }
            }
        }
        if minimal.is_empty() {
            return Cover::Impossible;
        }
        minimal.sort_unstable();
        Cover::Minimal(minimal)
    }

    /// PURE. A statement about hitting sets, and NOT about the game.
    ///
    /// True iff `side` owns at least one hot window AND those windows admit no
    /// cover of size at most `defender_budget`. The first conjunct is not
    /// implemented here: it is [`ThreatState::min_hitting_set_exceeds`]'s own
    /// empty-family case, and this function is that predicate applied to
    /// `hot_windows(side)`.
    ///
    /// # It means "`side` wins" only under TWO further conditions
    ///
    /// D-243 consequence (3) conditions on THE SIDE TO MOVE, and this predicate
    /// describes the position AFTER `side` has moved. So a caller may read it as
    /// a win only when both hold:
    ///
    /// 1. the game is not already decided — `GameState::outcome()` — and
    ///    `side.opponent()` is the side to move; and
    /// 2. `can_win_this_turn(side.opponent(), StonesLeft::from_state(state)?)`
    ///    is `None`.
    ///
    /// Drop the second half of (1) and the rule is wrong in the commonest way
    /// there is: if `side` is itself to move it simply wins now, and asking
    /// whether its opponent can win is asking about a turn that opponent does
    /// not have.
    ///
    /// THE OUTCOME CHECK IS NAMED HERE AND `state.to_move() != side` IS NOT A
    /// SPELLING OF IT. This doc used to gloss (1) that way, and the gloss is
    /// wrong on a class a search reaches: `GameState::to_move()` FREEZES on the
    /// winning ply and reads as the WINNER, so on a decided position
    /// `to_move() != side` is satisfied by the side that LOST — which can still
    /// own hot windows admitting no two-cell cover, making this predicate `true`
    /// for it. What stands between that and a mate score for the loser is (2)
    /// alone, because `StonesLeft::from_state` answers `None` on a decided
    /// position. That is correct, and it is thin: a caller that checked the
    /// conditions in the other order, or took the stones left from anywhere but
    /// core, would score the loser as winning (docs/decisions.md D-257).
    ///
    /// This function never consults either condition and never will. Composing
    /// them is the caller's, because a primitive that quietly answered a
    /// different question than its name would be unusable in the one place it
    /// matters (docs/decisions.md D-257).
    pub fn unblockable_double_threat(&self, side: Player, defender_budget: HitBudget) -> bool {
        self.min_hitting_set_exceeds(defender_budget, self.hot_windows(side))
    }

    /// The empty cells of each window, in the windows' own order.
    fn empty_families(&self, windows: &[Window]) -> Vec<Vec<Coord>> {
        windows
            .iter()
            .map(|&window| empty_cells(window, self.masks(window)).collect())
            .collect()
    }
}

/// Every cell that appears in some family, sorted and deduplicated.
fn universe(families: &[Vec<Coord>]) -> Vec<Coord> {
    let mut cells: Vec<Coord> = families.iter().flatten().copied().collect();
    cells.sort_unstable();
    cells.dedup();
    cells
}

/// Whether the one or two given cells meet every family.
fn covers(families: &[Vec<Coord>], first: Coord, second: Option<Coord>) -> bool {
    families.iter().all(|empties| {
        empties.contains(&first) || second.is_some_and(|second| empties.contains(&second))
    })
}

/// Whether two families share no cell.
fn disjoint(a: &[Coord], b: &[Coord]) -> bool {
    !a.iter().any(|cell| b.contains(cell))
}

/// Whether some three of `families` are pairwise disjoint (D-263 remedy 1).
///
/// A stone lies in at most one of three pairwise-disjoint families, so no pair
/// of stones — this crate's whole budget, [`crate::query::HitBudget`] closed
/// at two — hits all three: this is a NECESSARY condition for `Impossible`
/// that is cheap to check up front, before the O(|universe|^2) enumeration
/// [`covers`]'s callers run to establish the same answer the slow way.
/// `O(m^3)` in the family count `m`, each comparison `O(1)`-ish since a family
/// holds at most two cells (this module's own doc) — cheaper than the
/// enumeration it guards whenever `m` is not tiny, and a no-op cost when it
/// is.
fn three_pairwise_disjoint_families(families: &[Vec<Coord>]) -> bool {
    for i in 0..families.len() {
        for j in (i + 1)..families.len() {
            if !disjoint(&families[i], &families[j]) {
                continue;
            }
            for family_k in &families[(j + 1)..] {
                if disjoint(&families[i], family_k) && disjoint(&families[j], family_k) {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::three_pairwise_disjoint_families;
    use pistol_core::Coord;

    fn cell(q: i16, r: i16) -> Coord {
        Coord::new(q, r)
    }

    #[test]
    fn three_families_with_no_shared_cell_are_detected() {
        let families = vec![
            vec![cell(0, 0), cell(1, 0)],
            vec![cell(10, 0), cell(11, 0)],
            vec![cell(20, 0), cell(21, 0)],
        ];
        assert!(three_pairwise_disjoint_families(&families));
    }

    #[test]
    fn two_disjoint_families_alone_are_not_three() {
        let families = vec![vec![cell(0, 0)], vec![cell(10, 0)]];
        assert!(!three_pairwise_disjoint_families(&families));
    }

    #[test]
    fn a_third_family_sharing_a_cell_with_one_of_two_disjoint_families_is_not_three_pairwise_disjoint()
     {
        // {0,0} and {10,0} are disjoint from each other, but the third family
        // shares 10,0 with the second — not every PAIR among the three is
        // disjoint, so this is not the case the early-out may answer for.
        let families = vec![
            vec![cell(0, 0)],
            vec![cell(10, 0)],
            vec![cell(10, 0), cell(11, 0)],
        ];
        assert!(!three_pairwise_disjoint_families(&families));
    }

    #[test]
    fn an_empty_or_singleton_family_list_has_no_three_pairwise_disjoint_families() {
        assert!(!three_pairwise_disjoint_families(&[]));
        assert!(!three_pairwise_disjoint_families(&[vec![cell(0, 0)]]));
    }

    #[test]
    fn four_families_where_only_three_are_pairwise_disjoint_are_still_detected() {
        // The fourth family overlaps the first, so not all four are pairwise
        // disjoint — but the early-out only needs SOME three, and the
        // second/third/fourth triple qualifies.
        let families = vec![
            vec![cell(0, 0)],
            vec![cell(10, 0)],
            vec![cell(20, 0)],
            vec![cell(0, 0), cell(30, 0)],
        ];
        assert!(three_pairwise_disjoint_families(&families));
    }
}
