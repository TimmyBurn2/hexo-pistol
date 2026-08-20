//! R1 — the from-scratch reference: the WHOLE query surface, computed the slow
//! obvious way from a `Board`.
//!
//! D-68's pattern, third instance after the eval and the search (D-106). It
//! shares only `pistol-core`: it never touches [`ThreatState`], never calls
//! `windows_through`, and never carries anything incrementally. Every window it
//! knows about it built by stepping back from a stone and reading all six cells
//! with `Board::get`.
//!
//! **Its scope is deliberately the whole query surface and not the table.** A
//! reference scoped to the table, with the shipped queries then run over it,
//! would compare the cover enumeration, the phase conditioning and the witness
//! selection AGAINST THEMSELVES — and those are exactly the arithmetic that has
//! been wrong before. So the covers here are enumerated by a different
//! algorithm (all subsets within budget, then keep the ones no proper subset
//! covers), the minimum hitting set is computed by trying sizes rather than by
//! a per-budget predicate, and the witness is chosen by sorting the candidates
//! rather than by a running minimum.
//!
//! R1 still shares the implementation's CENTRAL ASSUMPTION — that only a window
//! holding a stone can matter — which is why [`super::region`] exists.
//!
//! # RULE9-JUSTIFICATION: one oracle over one query surface (CLAUDE.md rule 9).
//!
//! This is a second implementation of everything the shipped queries answer, and
//! its value is that it is ONE reading of the definitions rather than several.
//! Splitting it by query would either share helpers with the code under test or
//! duplicate the window table per part; either way the independence that makes
//! it an oracle is what gets spent. It shrinks only if the query surface does.

use std::collections::BTreeMap;

use pistol_core::window::{WINDOW_LEN, Window};
use pistol_core::{Axis, Board, Coord, Player};
use pistol_solver::{Cover, HitBudget, LiveCount, MinimalCover, NearHot, StonesLeft, WinWitness};

/// What one window holds, read straight off the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RefWindow {
    /// P1's cells, one bit per window index.
    pub p1: u8,
    /// P2's cells.
    pub p2: u8,
}

impl RefWindow {
    /// Read every cell of `window` off `board`.
    pub fn read(window: Window, board: &Board) -> RefWindow {
        let mut held = RefWindow::default();
        for index in 0..WINDOW_LEN as u8 {
            match board.get(window.cell(index)) {
                Some(Player::P1) => held.p1 |= 1 << index,
                Some(Player::P2) => held.p2 |= 1 << index,
                None => {}
            }
        }
        held
    }

    fn count(self, side: Player) -> u32 {
        match side {
            Player::P1 => self.p1.count_ones(),
            Player::P2 => self.p2.count_ones(),
        }
    }

    fn holds_a_stone(self) -> bool {
        self.p1 != 0 || self.p2 != 0
    }

    /// Live for `side`: at least one own stone and no opponent stone (D-243).
    fn is_live(self, side: Player) -> bool {
        self.count(side) >= 1 && self.count(side.opponent()) == 0
    }
}

/// Every window that holds a stone, and what it holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    windows: BTreeMap<Window, RefWindow>,
}

impl Reference {
    /// Stone-driven: for every occupied cell, for every axis, for every offset,
    /// the window starting that many steps back.
    pub fn from_board(board: &Board) -> Reference {
        let mut windows = BTreeMap::new();
        for (at, _) in board.stones() {
            for axis in Axis::ALL {
                for back in 0..WINDOW_LEN as i16 {
                    let Some(start) = at.checked_step(axis, -back) else {
                        continue;
                    };
                    let Some(window) = Window::new(axis, start) else {
                        continue;
                    };
                    windows
                        .entry(window)
                        .or_insert_with(|| RefWindow::read(window, board));
                }
            }
        }
        Reference { windows }
    }

    /// The table, for the comparison against the incremental state's own.
    pub fn table(&self) -> &BTreeMap<Window, RefWindow> {
        &self.windows
    }

    fn held(&self, window: Window) -> RefWindow {
        self.windows.get(&window).copied().unwrap_or_default()
    }

    fn empties(&self, window: Window) -> Vec<Coord> {
        let held = self.held(window);
        (0..WINDOW_LEN as u8)
            .filter(|&index| (held.p1 | held.p2) & (1 << index) == 0)
            .map(|index| window.cell(index))
            .collect()
    }

    fn matching(&self, keep: impl Fn(&RefWindow) -> bool) -> Vec<Window> {
        self.windows
            .iter()
            .filter(|(_, held)| keep(held))
            .map(|(&window, _)| window)
            .collect()
    }

    /// Live with at least four own stones.
    pub fn hot(&self, side: Player) -> Vec<Window> {
        self.matching(|held| held.is_live(side) && held.count(side) >= 4)
    }

    /// Live with exactly five own stones.
    pub fn win_in_one_ply(&self, side: Player) -> Vec<Window> {
        self.matching(|held| held.is_live(side) && held.count(side) == 5)
    }

    /// Every cell own.
    pub fn completed(&self, side: Player) -> Vec<Window> {
        self.matching(|held| held.count(side) == WINDOW_LEN)
    }

    /// Live with exactly `count` own stones.
    pub fn live_at(&self, side: Player, count: LiveCount) -> Vec<Window> {
        let wanted = match count {
            LiveCount::Two => 2,
            LiveCount::Three => 3,
        };
        self.matching(|held| held.is_live(side) && held.count(side) == wanted)
    }

    fn union_of_empties(&self, windows: &[Window]) -> Vec<Coord> {
        let mut cells: Vec<Coord> = windows
            .iter()
            .flat_map(|&window| self.empties(window))
            .collect();
        cells.sort_unstable();
        cells.dedup();
        cells
    }

    /// The empties of the hot windows.
    pub fn threat_cells(&self, side: Player) -> Vec<Coord> {
        self.union_of_empties(&self.hot(side))
    }

    /// The empties of the win-in-one-ply windows.
    pub fn win_in_one_ply_cells(&self, side: Player) -> Vec<Coord> {
        self.union_of_empties(&self.win_in_one_ply(side))
    }

    /// The empties of the count-three live windows.
    pub fn cells_raising_to_hot(&self, side: Player, count: NearHot) -> Vec<Coord> {
        let windows = match count {
            NearHot::Three => self.live_at(side, LiveCount::Three),
        };
        self.union_of_empties(&windows)
    }

    /// The size of a minimum hitting set of `windows`' empties, or `None` if no
    /// set of at most [`CEILING`] cells hits them all — which includes the
    /// genuinely unhittable case, where some window has no empty at all.
    ///
    /// Computed by TRYING SIZES, smallest first — the definition — rather than
    /// by a per-budget predicate.
    ///
    /// THE CEILING IS FIXED AND IS NOT AN ARGUMENT. It was an argument, on the
    /// stated ground that "the two callers want different ceilings"; there is
    /// ONE caller, the predicate below, and the diagnostic that would have
    /// wanted to report "the minimum is three" was never written. So the
    /// parameter and the sentence justifying it were both stale, and a stale
    /// justification in an oracle is worse than none: it describes a shape the
    /// code no longer has.
    pub fn min_hitting_set(&self, windows: &[Window]) -> Option<usize> {
        let families: Vec<Vec<Coord>> = windows.iter().map(|&w| self.empties(w)).collect();
        if families.is_empty() {
            return Some(0);
        }
        let universe = universe(&families);
        (1..=CEILING).find(|&size| {
            subsets(&universe, size)
                .into_iter()
                .any(|candidate| hits_all(&families, &candidate))
        })
    }

    /// Whether that minimum exceeds `budget`.
    pub fn min_hitting_set_exceeds(&self, budget: HitBudget, windows: &[Window]) -> bool {
        let allowed = match budget {
            HitBudget::Zero => 0,
            HitBudget::One => 1,
            HitBudget::Two => 2,
        };
        match self.min_hitting_set(windows) {
            Some(size) => size > allowed,
            None => true,
        }
    }

    /// The inclusion-minimal covers of the attacker's hot windows, by the
    /// definition: every subset within budget that covers, minus every one with
    /// a proper subset that also covers.
    pub fn blocking_covers(&self, defender: Player, budget: HitBudget) -> Cover {
        let windows = self.hot(defender.opponent());
        if windows.is_empty() {
            return Cover::NothingToBlock;
        }
        let allowed = match budget {
            HitBudget::Zero => 0,
            HitBudget::One => 1,
            HitBudget::Two => 2,
        };
        let families: Vec<Vec<Coord>> = windows.iter().map(|&w| self.empties(w)).collect();
        let universe = universe(&families);
        let mut covering: Vec<Vec<Coord>> = Vec::new();
        for size in 1..=allowed {
            for candidate in subsets(&universe, size) {
                if hits_all(&families, &candidate) {
                    covering.push(candidate);
                }
            }
        }
        let minimal: Vec<Vec<Coord>> = covering
            .iter()
            .filter(|candidate| {
                !covering
                    .iter()
                    .any(|other| other.len() < candidate.len() && is_subset(other, candidate))
            })
            .cloned()
            .collect();
        if minimal.is_empty() {
            return Cover::Impossible;
        }
        let mut covers: Vec<MinimalCover> = minimal
            .iter()
            .map(|cells| match cells.as_slice() {
                [only] => MinimalCover::One(*only),
                [first, second] => MinimalCover::Two {
                    first: *first,
                    second: *second,
                },
                other => panic!(
                    "a cover of {} cells is not one this budget allows",
                    other.len()
                ),
            })
            .collect();
        covers.sort_unstable();
        Cover::Minimal(covers)
    }

    /// True iff `side` owns a hot window whose family admits no cover within
    /// `defender_budget`. A statement about hitting sets, not about the game.
    pub fn unblockable_double_threat(&self, side: Player, defender_budget: HitBudget) -> bool {
        self.min_hitting_set_exceeds(defender_budget, &self.hot(side))
    }

    /// How `side` completes six this turn, if it can — by sorting the
    /// candidates and taking the first.
    pub fn can_win_this_turn(&self, side: Player, left: StonesLeft) -> Option<WinWitness> {
        let mut one_ply: Vec<(Coord, Window)> = self
            .win_in_one_ply(side)
            .into_iter()
            .map(|window| {
                let empties = self.empties(window);
                assert_eq!(empties.len(), 1, "a live five-window has one empty");
                (empties[0], window)
            })
            .collect();
        one_ply.sort_unstable();
        if let Some(&(at, window)) = one_ply.first() {
            return Some(WinWitness::OnePly { at, window });
        }
        if left == StonesLeft::One {
            return None;
        }
        let mut fours: Vec<Window> = self
            .matching(|held| held.is_live(side) && held.count(side) == 4)
            .into_iter()
            .collect();
        fours.sort_unstable();
        let window = *fours.first()?;
        let mut empties = self.empties(window);
        empties.sort_unstable();
        assert_eq!(empties.len(), 2, "a live four-window has two empties");
        Some(WinWitness::Pair {
            first: empties[0],
            second: empties[1],
            window,
        })
    }
}

/// The largest hitting set this reference looks for.
///
/// Two, and two is the whole domain: `HitBudget` is closed at two, so a minimum
/// above the largest budget exceeds every budget and HOW FAR above is a question
/// no caller asks.
const CEILING: usize = 2;

/// Every cell in some family, sorted and deduplicated.
fn universe(families: &[Vec<Coord>]) -> Vec<Coord> {
    let mut cells: Vec<Coord> = families.iter().flatten().copied().collect();
    cells.sort_unstable();
    cells.dedup();
    cells
}

/// Every `size`-cell subset of `universe`, in sorted order.
fn subsets(universe: &[Coord], size: usize) -> Vec<Vec<Coord>> {
    match size {
        0 => vec![Vec::new()],
        1 => universe.iter().map(|&cell| vec![cell]).collect(),
        2 => {
            let mut pairs = Vec::new();
            for (index, &first) in universe.iter().enumerate() {
                for &second in &universe[index + 1..] {
                    pairs.push(vec![first, second]);
                }
            }
            pairs
        }
        3 => {
            let mut triples = Vec::new();
            for (first_index, &first) in universe.iter().enumerate() {
                for (offset, &second) in universe[first_index + 1..].iter().enumerate() {
                    for &third in &universe[first_index + offset + 2..] {
                        triples.push(vec![first, second, third]);
                    }
                }
            }
            triples
        }
        other => panic!("this reference enumerates subsets of size at most three, not {other}"),
    }
}

/// Whether `cells` meets every family.
fn hits_all(families: &[Vec<Coord>], cells: &[Coord]) -> bool {
    families
        .iter()
        .all(|empties| cells.iter().any(|cell| empties.contains(cell)))
}

/// Whether every cell of `inner` is in `outer`.
fn is_subset(inner: &[Coord], outer: &[Coord]) -> bool {
    inner.iter().all(|cell| outer.contains(cell))
}
