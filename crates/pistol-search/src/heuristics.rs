//! WP-1.7's three ordering heuristics — killers, history, countermove — and
//! the tables they live in (`docs/experiments/wp17_design.md`).
//!
//! What these tables do is REORDER: they promote validated remembered cells
//! to the front of the staged candidate set's UNFORCED range, after the
//! table's own move and never across the Tier-F boundary
//! (`crate::staged::StagedSet::promote_table_move`'s rule, theirs too). They
//! never add a cell, never touch threat generation, the eval or the TT, and
//! never run under `CandidatePolicy::Radius` or inside `crate::quiescence`.
//!
//! # Determinism (CLAUDE.md rule 4, docs/decisions.md D-7)
//!
//! Storage is plain arrays and `BTreeMap` — no hasher, so no randomized
//! iteration exists to reach a choice path; the only iteration the tables
//! themselves perform is `begin_search`'s aging sweep, over a sorted map.
//! Nothing here reads a clock, a node count or a thread. Every tie breaks
//! left-to-right over the delta-ranked candidates, so the lexicographic
//! final tie-break (D-5, D-7) survives underneath.
//!
//! # RULE9-JUSTIFICATION: the three tables and their one seam (CLAUDE.md
//! rule 9).
//!
//! The killers, the pair killers, the history map and the countermove map
//! are one mechanism: they are written by ONE event (a beta cutoff at an
//! unforced candidate), read by ONE call (`order_candidates`), aged by ONE
//! lifecycle (`begin_search`/`clear`), and validated by ONE pair of
//! predicates. Splitting them by table would put the promotion order — the
//! load-bearing decision, since it decides which hint outranks which — in a
//! different file from the tables it sequences.

use std::collections::BTreeMap;

use pistol_core::{Board, Coord, GameState, Phase, Player};
use pistol_solver::ThreatState;

use crate::params::OrderingHeuristics;
use crate::search::MAX_PLY;
use crate::staged::StagedSet;

/// The two most recent cutoff stones at one ply, oldest-last: slot 0 is the
/// most recent.
type KillerSlots = [Option<Coord>; 2];

/// The tables, owned by [`crate::search::Searcher`] so they persist across
/// the searches of one game and are cleared by `newgame`
/// (`Searcher::clear`). The fields are `pub(crate)` so the crate's own
/// tests can observe them through a `Run` — the liveness of the recording
/// path is a property of `pvs::visit` calling it, which no unit test of the
/// tables alone can pin.
pub(crate) struct HeuristicTables {
    /// Two single-cell slots per PLY (D-9: two same-side plies per turn; the
    /// phase bit distinguishes them). Reset at every `begin_search`, because
    /// (mover, phase) is a function of the ply only WITHIN one search.
    pub(crate) killers: Vec<KillerSlots>,
    /// One canonical pair per ply, written at the ply the pair's turn
    /// STARTED at (the phase-First ply of a phase-Second cutoff).
    pub(crate) pair_killers: Vec<Option<(Coord, Coord)>>,
    /// (mover, cell) → cutoff score. Halved, floor, at every `begin_search`.
    pub(crate) history: BTreeMap<(Player, Coord), i32>,
    /// The opponent's last placed stone → the reply cell that refuted it.
    /// Last-write-wins, so it replaces its own stale entries and is never
    /// aged.
    pub(crate) countermove: BTreeMap<Coord, Coord>,
}

impl HeuristicTables {
    /// Empty tables, sized once for the deepest recursion this build can run.
    pub(crate) fn new() -> HeuristicTables {
        HeuristicTables {
            killers: vec![[None, None]; MAX_PLY],
            pair_killers: vec![None; MAX_PLY],
            history: BTreeMap::new(),
            countermove: BTreeMap::new(),
        }
    }

    /// What a new search does: the ply-keyed tables reset (ply indices
    /// restart at every search), history ages by halving, countermove — a
    /// last-write-wins table that overwrites its own stale entries — stays.
    pub(crate) fn begin_search(&mut self) {
        self.killers.fill([None, None]);
        self.pair_killers.fill(None);
        for score in self.history.values_mut() {
            *score /= 2;
        }
    }

    /// What `newgame` does: everything to empty
    /// (`crate::search::Searcher::clear`).
    pub(crate) fn clear(&mut self) {
        self.killers.fill([None, None]);
        self.pair_killers.fill(None);
        self.history.clear();
        self.countermove.clear();
    }

    /// Record the cutoff: `cutoff` is the stone whose subtree failed high at
    /// `ply`, `state` is the node's own position (before the stone was
    /// placed — the caller records after `undo`).
    pub(crate) fn record_cutoff(&mut self, state: &GameState, ply: usize, cutoff: Coord) {
        let mover = state.to_move();
        // History: flat +1, saturating (docs/experiments/wp17_design.md §4
        // M7 — magnitude is relative, history is only ever read as an
        // argmax).
        self.history
            .entry((mover, cutoff))
            .and_modify(|score| *score = score.saturating_add(1))
            .or_insert(1);
        // Killers: shift unless the stone already sits in slot 0.
        if self.killers[ply][0] != Some(cutoff) {
            self.killers[ply][1] = self.killers[ply][0];
            self.killers[ply][0] = Some(cutoff);
        }
        // The last two stones on the board, in play order. At a phase-First
        // node the last stone is the opponent's second; at a phase-Second
        // node it is this turn's own first stone.
        let mut last: Option<(Coord, Player)> = None;
        let mut second_last: Option<(Coord, Player)> = None;
        for played in state.played() {
            second_last = last;
            last = Some(played);
        }
        if state.phase() == Phase::Second
            && let Some((prev, _)) = last
        {
            // The pair's first stone is on the board; the cutoff stone is
            // its partner. Stored canonical (D-5), the dispatch's own
            // keying.
            self.pair_killers[ply - 1] = Some(canonical(prev, cutoff));
        }
        let opponent_last = match state.phase() {
            Phase::First => last,
            Phase::Second => second_last,
        };
        if let Some((at, _)) = opponent_last {
            self.countermove.insert(at, cutoff);
        }
    }

    /// Promote the validated remembered cells to the front of `set`'s
    /// unforced range, after the table's own move — the retrieval half of
    /// the mechanism, and the ONLY place the promotion order
    /// (TT → killers → pair → countermove → history, the report's line 83
    /// stack as `docs/experiments/wp17_design.md` §2 maps it) is stated in
    /// code.
    pub(crate) fn order_candidates(
        &self,
        gates: OrderingHeuristics,
        state: &GameState,
        threats: &ThreatState,
        ply: usize,
        table_move: Option<Coord>,
        set: &mut StagedSet,
    ) {
        let mover = state.to_move();
        let mut front = set.forced;
        // The table's move keeps absolute priority: if it was promoted it
        // sits at the head of the unforced range, and every heuristic
        // promotion starts one past it.
        if let Some(best) = table_move
            && set.cells.get(front) == Some(&best)
        {
            front += 1;
        }
        if gates.killers {
            for slot in self.killers[ply] {
                if let Some(at) = slot
                    && usable_cell(state.board(), at)
                {
                    front = promote(&mut set.cells, front, at);
                }
            }
            if state.phase() == Phase::First
                && let Some(pair) = self.pair_killers[ply]
                && usable_pair(state.board(), threats, mover, pair)
            {
                // The front advances through the first cell so the second is
                // promoted behind it, not over it.
                front = promote(&mut set.cells, front, pair.0);
                front = promote(&mut set.cells, front, pair.1);
            }
        }
        if gates.countermove
            && state.phase() == Phase::First
            && let Some(at) = last_stone(state)
            && let Some(reply) = self.countermove.get(&at).copied()
            && usable_cell(state.board(), reply)
        {
            front = promote(&mut set.cells, front, reply);
        }
        if gates.history {
            // The argmax over what is LEFT of the unforced range — cells the
            // promotions above already fronted are behind `front` and
            // therefore not candidates for this one. Left-to-right, so ties
            // fall back to the delta order.
            let mut best: Option<(i32, Coord)> = None;
            for &at in &set.cells[front.min(set.cells.len())..] {
                if let Some(&score) = self.history.get(&(mover, at))
                    && score > 0
                    && best.is_none_or(|(top, _)| score > top)
                {
                    best = Some((score, at));
                }
            }
            if let Some((_, at)) = best {
                // The last promotion of the node: the returned front has
                // nothing left to advance past, so it is not kept.
                promote(&mut set.cells, front, at);
            }
        }
    }
}

/// Canonical pair order: the lexicographic `(q, r)` order the protocol's
/// pair token and the search's final tie-break share (D-5, D-7).
fn canonical(a: Coord, b: Coord) -> (Coord, Coord) {
    if a <= b { (a, b) } else { (b, a) }
}

/// The last stone on the board, if any.
fn last_stone(state: &GameState) -> Option<Coord> {
    state.played().last().map(|(at, _)| at)
}

/// A retrieved single-cell entry is usable only if the board would accept a
/// stone there: EMPTY and inside the rule-5 region (`pistol_core::rules::LEGAL_RADIUS`
/// — a game rule, never a search knob). A stale entry is skipped, never
/// repaired.
fn usable_cell(board: &Board, at: Coord) -> bool {
    !board.is_occupied(at) && board.in_legal_region(at)
}

/// A retrieved pair entry is usable only if its cells are DISTINCT, the pair
/// is CANONICAL (`a < b` in the derived `Coord` order, D-5), both cells pass
/// [`usable_cell`], and it is LEGAL UNDER RULE 4: placing the FIRST cell
/// must not complete a win for the mover — a turn whose first stone wins is
/// a turn that never exists as a pair. The win check is the threat state's
/// own reading (`win_in_one_ply_windows` empty ⇒ no single placement wins),
/// with the fast path that costs nothing on every BATCHED row.
fn usable_pair(board: &Board, threats: &ThreatState, mover: Player, pair: (Coord, Coord)) -> bool {
    let (a, b) = pair;
    a < b && usable_cell(board, a) && usable_cell(board, b) && !first_stone_wins(threats, mover, a)
}

fn first_stone_wins(threats: &ThreatState, mover: Player, at: Coord) -> bool {
    if threats.win_in_one_ply_windows(mover).is_empty() {
        return false;
    }
    let mut cells = Vec::new();
    threats.win_in_one_ply_cells(mover, &mut cells);
    cells.contains(&at)
}

/// Rotate `entry` to `front` within `cells`, stably, and return the front
/// that follows it — the same rotation `StagedSet::promote_table_move` uses,
/// so the cells behind keep the order they arrived in and the lexicographic
/// tie-break survives underneath. An entry that is not in `cells[front..]`
/// is skipped, and the front does not move.
fn promote(cells: &mut [Coord], front: usize, entry: Coord) -> usize {
    let Some(found) = cells[front.min(cells.len())..]
        .iter()
        .position(|&at| at == entry)
        .map(|offset| front + offset)
    else {
        return front;
    };
    if found > front {
        cells[front..=found].rotate_right(1);
    }
    front + 1
}

#[cfg(test)]
mod tests {
    use pistol_core::GameState;

    use super::*;

    /// A quiet position with stones to talk about: P1's five and P2's six,
    /// nothing decided, from the search tests' own `quiet()` shape.
    fn quiet() -> GameState {
        GameState::from_plies(&[
            Coord::ORIGIN,
            Coord::new(1, 0),
            Coord::new(0, 1),
            Coord::new(2, 0),
            Coord::new(1, 1),
            Coord::new(0, 2),
            Coord::new(2, 1),
            Coord::new(1, 3),
            Coord::new(2, 2),
            Coord::new(0, 3),
            Coord::new(1, 4),
        ])
        .expect("the quiet fixture is a legal game")
    }

    /// The threat state matching `quiet()`, built the way
    /// `crate::position::Position::reset_to` builds its own.
    fn threats_for(state: &GameState) -> ThreatState {
        let mut threats = ThreatState::new();
        for (at, player) in state.board().stones() {
            threats.apply(at, player);
        }
        threats
    }

    /// A validated cell that is empty and in-region in `quiet()`.
    fn free_cell(_state: &GameState) -> Coord {
        Coord::new(3, 3)
    }

    #[test]
    fn a_retrieved_entry_on_an_occupied_cell_is_skipped_not_repaired() {
        let state = quiet();
        // (2,0) holds a P1 stone in `quiet()`.
        assert!(!usable_cell(state.board(), Coord::new(2, 0)));
        // The control: a neighbouring empty cell is usable.
        assert!(usable_cell(state.board(), free_cell(&state)));
    }

    #[test]
    fn a_retrieved_entry_outside_the_rule_5_region_is_skipped() {
        let state = quiet();
        // Far past radius 8 from every stone in the fixture.
        let far = Coord::new(900, -900);
        assert!(state.board().get(far).is_none(), "the far cell is empty");
        assert!(!state.board().in_legal_region(far));
        assert!(!usable_cell(state.board(), far));
    }

    #[test]
    fn an_uncanonical_pair_entry_is_skipped() {
        let state = quiet();
        let threats = threats_for(&state);
        let mover = state.to_move();
        let a = Coord::new(3, 3);
        let b = Coord::new(4, 4);
        assert!(a < b);
        // The canonical order is accepted…
        assert!(usable_pair(state.board(), &threats, mover, (a, b)));
        // …and the same pair spelled backwards is refused: the stored form
        // is canonical by construction (D-5), so a non-canonical retrieval
        // is a corrupted or foreign entry.
        assert!(!usable_pair(state.board(), &threats, mover, (b, a)));
        // Identical cells are refused too — a pair needs two stones.
        assert!(!usable_pair(state.board(), &threats, mover, (a, a)));
    }

    #[test]
    fn a_pair_whose_first_stone_wins_is_skipped_under_rule_4() {
        // P1 holds five in a row along axis 0 — the origin through (4,0) —
        // with (5,0) and (-1,0) both empty, and it is P1 to move (P2 has
        // just completed turn 6 far away). Placing on either empty WINS.
        let plies = [
            Coord::ORIGIN,
            // Turn 2, P2 — scattered, so no side forms six anywhere.
            Coord::new(0, 6),
            Coord::new(0, 8),
            // Turn 3, P1.
            Coord::new(1, 0),
            Coord::new(2, 0),
            // Turn 4, P2.
            Coord::new(1, 7),
            Coord::new(2, 7),
            // Turn 5, P1.
            Coord::new(3, 0),
            Coord::new(4, 0),
            // Turn 6, P2.
            Coord::new(0, 10),
            Coord::new(3, 7),
        ];
        let state = GameState::from_plies(&plies).expect("legal game");
        assert_eq!(state.to_move(), Player::P1);
        let threats = threats_for(&state);
        // (5,0) completes P1's six.
        let winning = Coord::new(5, 0);
        assert!(!threats.win_in_one_ply_windows(Player::P1).is_empty());
        assert!(first_stone_wins(&threats, Player::P1, winning));
        // A pair whose FIRST cell wins is refused — rule 4 means that turn
        // is one stone long and the pair never existed.
        assert!(!usable_pair(
            state.board(),
            &threats,
            Player::P1,
            (winning, Coord::new(1, 1))
        ));
        // The same winning cell as the SECOND stone of the pair is fine:
        // that is the normal winning pair.
        assert!(usable_pair(
            state.board(),
            &threats,
            Player::P1,
            (Coord::new(0, 1), winning)
        ));
    }

    #[test]
    fn record_cutoff_trusts_its_caller_for_the_unforced_test() {
        // The unforced-only gate itself lives in `pvs::visit`
        // (`best_index >= forced`) and is pinned there, at the node level, by
        // `a_forced_prefix_cutoff_updates_nothing_at_its_own_ply`. What this
        // test pins is the OTHER half of the contract: `record_cutoff` does
        // not re-derive the boundary — it records exactly what it is handed,
        // so the caller's test is the only defence the boundary has.
        let state = quiet();
        let mut tables = HeuristicTables::new();
        tables.record_cutoff(&state, 3, Coord::new(3, 3));
        assert_eq!(tables.killers[3][0], Some(Coord::new(3, 3)));
        assert_eq!(
            tables.history.get(&(state.to_move(), Coord::new(3, 3))),
            Some(&1)
        );
    }

    #[test]
    fn killers_reset_history_ages_and_countermove_survives_a_new_search() {
        let state = quiet();
        let mut tables = HeuristicTables::new();
        tables.record_cutoff(&state, 3, Coord::new(3, 3));
        tables.record_cutoff(&state, 3, Coord::new(4, 3));
        tables.record_cutoff(&state, 3, Coord::new(3, 3));
        // The repeated cutoff accumulated history: 1 + 1 = 2.
        assert_eq!(
            tables.history.get(&(state.to_move(), Coord::new(3, 3))),
            Some(&2)
        );
        // Slot order: the repeat left slot 0 alone, slot 1 holds the other.
        assert_eq!(tables.killers[3][0], Some(Coord::new(3, 3)));
        assert_eq!(tables.killers[3][1], Some(Coord::new(4, 3)));

        tables.begin_search();
        // Killers are gone…
        assert_eq!(tables.killers[3], [None, None]);
        assert_eq!(tables.pair_killers[3], None);
        // …history HALVED (floor)…
        assert_eq!(
            tables.history.get(&(state.to_move(), Coord::new(3, 3))),
            Some(&1)
        );
        // …and countermove survives.
        let opp_last = state.played().last().map(|(at, _)| at).unwrap();
        assert_eq!(tables.countermove.get(&opp_last), Some(&Coord::new(3, 3)));
    }

    #[test]
    fn history_scores_are_halved_at_each_new_search() {
        let state = quiet();
        let mover = state.to_move();
        let mut tables = HeuristicTables::new();
        let at = Coord::new(3, 3);
        for _ in 0..5 {
            tables.record_cutoff(&state, 2, at);
        }
        assert_eq!(tables.history.get(&(mover, at)), Some(&5));
        tables.begin_search();
        assert_eq!(tables.history.get(&(mover, at)), Some(&2));
        tables.begin_search();
        assert_eq!(tables.history.get(&(mover, at)), Some(&1));
        tables.begin_search();
        // Floor division: a single cutoff's residue is gone at the next
        // search — the thin-residue reading the design's §4 M5 states.
        assert_eq!(tables.history.get(&(mover, at)), Some(&0));
    }

    #[test]
    fn newgame_clears_every_heuristic_table() {
        let state = quiet();
        let mut tables = HeuristicTables::new();
        tables.record_cutoff(&state, 3, Coord::new(3, 3));
        tables.clear();
        assert_eq!(tables.killers[3], [None, None]);
        assert_eq!(tables.pair_killers[3], None);
        assert!(tables.history.is_empty());
        assert!(tables.countermove.is_empty());
    }

    #[test]
    fn promotions_never_cross_the_forced_boundary() {
        // Three unforced cells behind one forced cell; the pair killer
        // points at the forced cell itself, which must NOT be promoted —
        // promoting a forced cell would let a Tier-T memory masquerade as
        // Tier F's own answer (`StagedSet::promote_table_move`'s own rule).
        let state = quiet();
        let threats = threats_for(&state);
        let mover = state.to_move();
        let forced_cell = Coord::new(3, 3);
        let mut tables = HeuristicTables::new();
        let mut set = StagedSet {
            cells: vec![
                forced_cell,
                Coord::new(-1, 4),
                Coord::new(-2, 4),
                Coord::new(0, 5),
            ],
            forced: 1,
            used_quiet_safety_net: false,
        };
        let before_forced = set.cells[0];

        // The pair CONTAINS the forced cell: usable_pair passes (both cells
        // are legal empty in-region non-winning cells), and the promotion
        // must still leave the forced cell exactly where it is — the
        // membership scan starts at `forced` and never reaches it.
        tables.pair_killers[0] = Some(canonical(forced_cell, Coord::new(-1, 4)));
        tables.order_candidates(
            OrderingHeuristics {
                killers: true,
                history: false,
                countermove: false,
            },
            &state,
            &threats,
            0,
            None,
            &mut set,
        );
        assert_eq!(set.cells[0], before_forced);
        assert_eq!(set.forced, 1);
        // The pair's cells are promoted within the unforced range only:
        // (-1,4) was already the head of it, so the order is unchanged.
        assert_eq!(
            &set.cells[1..],
            &[Coord::new(-1, 4), Coord::new(-2, 4), Coord::new(0, 5)][..]
        );

        // Now a pair entirely inside the unforced range: both cells rotate
        // to its head, canonical order first, the rest keep their order.
        let mut set = StagedSet {
            cells: vec![
                forced_cell,
                Coord::new(-1, 4),
                Coord::new(-2, 4),
                Coord::new(0, 5),
            ],
            forced: 1,
            used_quiet_safety_net: false,
        };
        tables.pair_killers[0] = Some(canonical(Coord::new(0, 5), Coord::new(-2, 4)));
        tables.order_candidates(
            OrderingHeuristics {
                killers: true,
                history: false,
                countermove: false,
            },
            &state,
            &threats,
            0,
            None,
            &mut set,
        );
        assert_eq!(set.cells[0], before_forced);
        assert_eq!(
            &set.cells[1..],
            &[Coord::new(-2, 4), Coord::new(0, 5), Coord::new(-1, 4)][..]
        );
        let _ = mover;
    }

    #[test]
    fn the_history_argmax_promotes_only_a_positive_score_and_only_after_killers() {
        let state = quiet();
        let threats = threats_for(&state);
        let mover = state.to_move();
        let mut tables = HeuristicTables::new();
        // (0,5) has history 3; (-2,4) has history 1; (-1,4) has none.
        for _ in 0..3 {
            tables.record_cutoff(&state, 2, Coord::new(0, 5));
        }
        tables.record_cutoff(&state, 2, Coord::new(-2, 4));

        let mut set = StagedSet {
            cells: vec![Coord::new(-1, 4), Coord::new(-2, 4), Coord::new(0, 5)],
            forced: 0,
            used_quiet_safety_net: false,
        };
        tables.order_candidates(
            OrderingHeuristics {
                killers: false,
                history: true,
                countermove: false,
            },
            &state,
            &threats,
            4,
            None,
            &mut set,
        );
        assert_eq!(
            set.cells,
            vec![Coord::new(0, 5), Coord::new(-1, 4), Coord::new(-2, 4)],
            "the best-history cell is promoted; the rest keep their order"
        );

        // With killers on and a killer at (0,5) too, the killer promotion
        // happens first and the history argmax then sees only what is left —
        // here (-2,4), whose score is 1, so it is promoted second and
        // (-1,4), with no score, keeps the tail.
        tables.killers[4] = [Some(Coord::new(0, 5)), None];
        let mut set = StagedSet {
            cells: vec![Coord::new(-1, 4), Coord::new(-2, 4), Coord::new(0, 5)],
            forced: 0,
            used_quiet_safety_net: false,
        };
        tables.order_candidates(
            OrderingHeuristics {
                killers: true,
                history: true,
                countermove: false,
            },
            &state,
            &threats,
            4,
            None,
            &mut set,
        );
        assert_eq!(
            set.cells,
            vec![Coord::new(0, 5), Coord::new(-2, 4), Coord::new(-1, 4)]
        );
        let _ = mover;
    }

    #[test]
    fn the_table_move_keeps_absolute_priority_over_every_heuristic() {
        let state = quiet();
        let threats = threats_for(&state);
        let mut tables = HeuristicTables::new();
        tables.killers[2] = [Some(Coord::new(0, 5)), Some(Coord::new(-2, 4))];
        let mut set = StagedSet {
            cells: vec![Coord::new(-1, 4), Coord::new(-2, 4), Coord::new(0, 5)],
            forced: 0,
            used_quiet_safety_net: false,
        };
        // The caller has already promoted the table's move to the head…
        set.promote_table_move(Some(Coord::new(-1, 4)));
        // …and the heuristics must start one past it.
        tables.order_candidates(
            OrderingHeuristics {
                killers: true,
                history: false,
                countermove: false,
            },
            &state,
            &threats,
            2,
            Some(Coord::new(-1, 4)),
            &mut set,
        );
        assert_eq!(
            set.cells,
            vec![Coord::new(-1, 4), Coord::new(0, 5), Coord::new(-2, 4)],
            "table move first, then the killer slots, and nothing demotes the table's move"
        );
    }
}
