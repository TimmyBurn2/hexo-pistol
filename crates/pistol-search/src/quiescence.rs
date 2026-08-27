//! Threat-only, zone-bounded quiescence at the search horizon (WP-1.6,
//! docs/wp16_quiescence_design.md).
//!
//! A dedicated module, not a reuse of [`crate::staged::staged_candidates`]:
//! Ruling 1 (the design's §3) excludes `staged_candidates`' width from
//! quiescence entirely — its completeness licence is `LAW-SUPPORT`'s
//! k-TURNS-OUT argument, and a horizon extension has no turns of remaining
//! depth to spend that completeness on. Every candidate cell here traces to
//! one of four `pistol-solver` queries named in the design's §3.1–§3.3/§3.5a:
//! `can_win_this_turn`, `blocking_covers`, `cells_raising_to_hot`, and the
//! completion stone's own eight-query "live windows' support" union — never
//! `staged_candidates`, never `within_radius`.
//!
//! # RULE9-JUSTIFICATION: one extension, over the one recursion that realises
//! it (CLAUDE.md rule 9).
//!
//! [`Run::quiescence`] (the gate), [`Run::granted_turn`] (ply-1),
//! [`Run::quiescence_ply2`] (ply-2) and [`Run::completion_stone`] are one
//! argument split across the shape the game's own turn structure imposes —
//! a gate decision, then two plies — not four independent features. Splitting
//! the gate's trigger classification into a free function that does not touch
//! `self` is the one reduction that is honest without re-threading the whole
//! position through call boundaries the design's own §3 already keeps inside
//! one `staged_context()` borrow.

use pistol_core::{Coord, NEIGHBOUR_DIRECTIONS, PlyOutcome};
use pistol_solver::{Cover, HitBudget, LiveCount, MinimalCover, NearHot, StonesLeft};

use crate::params::{CandidatePolicy, QTriggers};
use crate::pvs::{CANDIDATE_ILLEGAL, Run};
use crate::score::{self, INFINITY};
use crate::tt::{Bound, Record};

/// Named invariant: the quiescence gate was asked about a position the rules
/// have already decided.
///
/// Unreachable through `Run::visit`'s own recursion, the same reason
/// `staged::OVERLOAD_ON_A_DECIDED_POSITION` gives: `place` returns
/// `PlyOutcome::Win` and the parent scores without recursing into a decided
/// child.
pub const QUIESCENCE_ON_A_DECIDED_POSITION: &str = "QUIESCENCE_ON_A_DECIDED_POSITION";

/// Named invariant: neither the completion stone's live-window-support union
/// nor the mover's own six neighbours offered a legal cell.
///
/// An EXTREME board-density configuration (design §3.5a): the cell the search
/// just legally placed a stone on already has six stones immediately
/// surrounding it, while nothing anywhere on the board qualifies for any live
/// classification. Not proven impossible, only implausible at any reachable
/// mid-search position — per rule 3's fail-loud discipline this panics rather
/// than falling through to a forbidden generator (`staged_candidates`'s quiet
/// ball, `within_radius`) or a mid-turn static read.
pub const NO_COMPLETION_STONE: &str = "NO_COMPLETION_STONE";

/// Which row the quiescence gate takes, and — for the two `Extend` rows — the
/// candidate cells it grants (design §3.1–§3.4).
enum GateRow {
    /// Trigger (a): the mover can win this turn. Terminal, no cells.
    WinNow,
    /// `Cover::Impossible`: `LAW-OVERLOAD`. Terminal, no cells.
    Overload,
    /// Trigger (b): the opponent holds a plan with t <= 1. `Cover::cells()`.
    ExtendDefense(Vec<Coord>),
    /// Trigger (c): the mover can activate a new plan this turn.
    /// `cells_raising_to_hot`'s own output.
    ExtendOffense(Vec<Coord>),
    /// Neither trigger fired.
    StandPat,
}

/// Which row ply-2's own generation takes (design §3.5).
enum Ply2Row {
    /// `Cover2::Impossible`: `LAW-OVERLOAD`, one remaining stone cannot hit
    /// it either. Routed through the completion stone (design's revision-4
    /// correction, closing B2) rather than returning with nothing placed.
    Overload,
    /// `Cover2::cells() ∪ cells_raising_to_hot`'s own output, deduplicated.
    /// Empty is a real value here — routed through the completion stone
    /// (route 1) rather than treated as a special case by this type.
    Cells(Vec<Coord>),
}

impl<'a> Run<'a> {
    /// The quiescence gate: trigger (a)/(b)/(c)/`LAW-OVERLOAD`, in that order
    /// (design §3.1–§3.4), then — if a trigger granted an extension — ply-1's
    /// candidate loop (`Run::granted_turn`).
    ///
    /// Called exactly once per horizon, from `Run::visit`'s `depth_plies == 0`
    /// branch, under `CandidatePolicy::Staged` only. `q_budget` is how many
    /// FURTHER whole turns an extension may grant from here — `0` means the
    /// free checks (win-now, `LAW-OVERLOAD`) still run, but neither trigger
    /// may grant.
    pub(crate) fn quiescence(&mut self, alpha: i32, beta: i32, ply: usize, q_budget: u32) -> i32 {
        self.search_nodes += 1;
        self.pv.clear(ply);
        self.seldepth_turns = self.seldepth_turns.max(self.turns_from_root());
        if self.should_stop() {
            return 0;
        }

        match self.gate_row() {
            GateRow::WinNow => {
                self.stages.q_win_now += 1;
                score::mate_in(self.turns_from_root() + 1)
            }
            GateRow::Overload => {
                self.stages.q_overload_return += 1;
                -score::mate_in(self.turns_from_root() + 2)
            }
            GateRow::ExtendDefense(cells) => self.extend(cells, false, alpha, beta, ply, q_budget),
            GateRow::ExtendOffense(cells) => self.extend(cells, true, alpha, beta, ply, q_budget),
            GateRow::StandPat => {
                self.stages.q_stand_pat_no_trigger += 1;
                self.position.value()
            }
        }
    }

    /// Grant an extension (or refuse it for want of budget) and, if granted,
    /// store what ply-1's own alpha-beta loop found — the whole-search
    /// `Record` `visit`'s own end-of-node store computes, with `depth_plies`
    /// and `from_quiescence` substituted (design §6 item 4).
    fn extend(
        &mut self,
        cells: Vec<Coord>,
        offense: bool,
        alpha: i32,
        beta: i32,
        ply: usize,
        q_budget: u32,
    ) -> i32 {
        if q_budget == 0 {
            self.stages.q_stand_pat_cap += 1;
            return self.position.value();
        }
        if offense {
            self.stages.q_extend_offense += 1;
        } else {
            self.stages.q_extend_defense += 1;
        }
        self.stages.qnodes += 1;

        let key = self.position.state().key();
        let from_root = self.turns_from_root();
        let (best_score, best_cell) = self.granted_turn(&cells, alpha, beta, ply, q_budget - 1);
        if !self.aborted {
            self.table.store(
                key,
                from_root,
                Record {
                    depth_plies: 1,
                    score: best_score,
                    static_eval: self.position.value(),
                    bound: if best_score <= alpha {
                        Bound::Upper
                    } else if best_score >= beta {
                        Bound::Lower
                    } else {
                        Bound::Exact
                    },
                    best: best_cell,
                    from_quiescence: true,
                },
            );
        }
        best_score
    }

    /// The gate's own trigger classification (design §3.1–§3.4), entirely
    /// within one `staged_context()` borrow — the same discipline
    /// `pvs::visit`'s `Staged` arm already uses.
    fn gate_row(&mut self) -> GateRow {
        let (state, threats, _eval) = self.position.staged_context();
        let left = StonesLeft::from_state(state).unwrap_or_else(|| {
            panic!(
                "pistol-search invariant {QUIESCENCE_ON_A_DECIDED_POSITION}: the quiescence gate \
                 was called on a decided position, where no turn's stones-owed count applies"
            )
        });
        let us = state.to_move();

        if threats.can_win_this_turn(us, left).is_some() {
            return GateRow::WinNow;
        }

        let cover = threats.blocking_covers(us, HitBudget::from(left));
        if matches!(&cover, Cover::Impossible) {
            return GateRow::Overload;
        }
        let t_le_1 = matches!(
            &cover,
            Cover::Minimal(covers) if covers.iter().any(|c| matches!(c, MinimalCover::One(_)))
        );
        if t_le_1 {
            return GateRow::ExtendDefense(cover.cells());
        }

        // D-396: trigger (c) is evaluated only under
        // `QTriggers::DefensiveAndOffensive` — under `DefensiveOnly` the
        // gate falls straight to `StandPat` when trigger (b) does not fire,
        // exactly as if trigger (c) did not exist. `self.policy` is
        // `CandidatePolicy::Staged` here by construction: `gate_row` is
        // reachable only from `Run::quiescence`, itself reachable only from
        // `Run::visit`'s own `Staged` arm (`pvs.rs`).
        let CandidatePolicy::Staged(params) = self.policy else {
            unreachable!(
                "pistol-search invariant: the quiescence gate is reachable only under \
                 CandidatePolicy::Staged"
            );
        };
        if params.q_triggers == QTriggers::DefensiveAndOffensive {
            let mut offense = Vec::new();
            threats.cells_raising_to_hot(us, NearHot::Three, &mut offense);
            if !offense.is_empty() {
                return GateRow::ExtendOffense(offense);
            }
        }

        GateRow::StandPat
    }

    /// Ply-1's candidate loop: plain alpha-beta (no PVS null-window scan —
    /// the design permits IMPL discretion on code sharing, design §7, and
    /// these candidate sets are small enough that the discipline alone is
    /// what soundness needs). No candidate here can win — trigger (a) already
    /// ruled out both a win-in-one-ply cell and a hot four-stone window at
    /// `StonesLeft::Two` (`batched()`'s own argument, design §3.3) — so every
    /// placement continues the turn.
    ///
    /// Returns the best score and the cell that produced it, for the gate's
    /// own `Record` (`Run::extend`).
    fn granted_turn(
        &mut self,
        cells: &[Coord],
        mut alpha: i32,
        beta: i32,
        ply: usize,
        q_budget: u32,
    ) -> (i32, Coord) {
        let mut best_score = -INFINITY;
        let mut best_cell = cells[0];
        for &at in cells {
            let outcome = self.position.place(at).unwrap_or_else(|error| {
                panic!(
                    "pistol-search invariant {CANDIDATE_ILLEGAL}: quiescence ply-1 offered {at}, \
                     and the rules say: {error}"
                )
            });
            debug_assert!(
                matches!(outcome, PlyOutcome::TurnContinues),
                "pistol-search invariant: a quiescence ply-1 candidate won or completed the turn, \
                 which trigger (a) already ruled out"
            );
            let score = self.quiescence_ply2(at, alpha, beta, ply + 1, q_budget);
            self.position.undo();
            if self.aborted {
                return (0, best_cell);
            }
            if score > best_score {
                best_score = score;
                best_cell = at;
                if score > alpha {
                    alpha = score;
                    self.pv.promote(ply, at);
                }
            }
            if alpha >= beta {
                break;
            }
        }
        (best_score, best_cell)
    }

    /// Ply-2: `LAW-OVERLOAD` (routed through the completion stone, closing
    /// B2), the completion stone (route 1, an empty union), or the ordinary
    /// alpha-beta loop chaining into the next gate (design §3.5).
    fn quiescence_ply2(
        &mut self,
        ply1_stone: Coord,
        mut alpha: i32,
        beta: i32,
        ply: usize,
        q_budget: u32,
    ) -> i32 {
        self.search_nodes += 1;
        self.pv.clear(ply);
        self.stages.qnodes += 1;
        self.seldepth_turns = self.seldepth_turns.max(self.turns_from_root());
        if self.should_stop() {
            return 0;
        }

        match self.ply2_row() {
            Ply2Row::Overload => {
                self.stages.q_overload_return += 1;
                let at = self.completion_stone(ply1_stone);
                self.place_completion(at, ply);
                let score = -score::mate_in(self.turns_from_root() + 2);
                self.position.undo();
                score
            }
            Ply2Row::Cells(cells) if cells.is_empty() => {
                let at = self.completion_stone(ply1_stone);
                self.place_completion(at, ply);
                let score = -self.position.value();
                self.position.undo();
                score
            }
            Ply2Row::Cells(cells) => {
                let mut best_score = -INFINITY;
                for &at in &cells {
                    let outcome = self.position.place(at).unwrap_or_else(|error| {
                        panic!(
                            "pistol-search invariant {CANDIDATE_ILLEGAL}: quiescence ply-2 \
                             offered {at}, and the rules say: {error}"
                        )
                    });
                    debug_assert!(
                        matches!(outcome, PlyOutcome::TurnComplete),
                        "pistol-search invariant: a quiescence ply-2 candidate won, which \
                         design §3.5's own proof rules out (no live window of ours reaches \
                         own == 5 by the time ply-2 is reached)"
                    );
                    let score = -self.quiescence(-beta, -alpha, ply + 1, q_budget);
                    self.position.undo();
                    if self.aborted {
                        return 0;
                    }
                    if score > best_score {
                        best_score = score;
                        if score > alpha {
                            alpha = score;
                            self.pv.promote(ply, at);
                        }
                    }
                    if alpha >= beta {
                        break;
                    }
                }
                best_score
            }
        }
    }

    /// Place a completion stone as ply-2's own move: promote it into the PV
    /// at `ply` (clearing the would-be grandchild ply first, since neither
    /// completion route recurses further — "completion is not search",
    /// design §3.5a) so the parent's eventual promotion carries both stones
    /// of the turn, not one.
    fn place_completion(&mut self, at: Coord, ply: usize) {
        let outcome = self.position.place(at).unwrap_or_else(|error| {
            panic!(
                "pistol-search invariant {CANDIDATE_ILLEGAL}: the completion stone {at} was \
                 illegal, and the rules say: {error}"
            )
        });
        debug_assert!(
            matches!(outcome, PlyOutcome::TurnComplete),
            "pistol-search invariant: the completion stone is this turn's second, so it always \
             completes the turn — no ply-2 win is reachable (design §3.5's own proof)"
        );
        self.pv.clear(ply + 1);
        self.pv.promote(ply, at);
    }

    /// Ply-2's own trigger classification (design §3.5), within one
    /// `staged_context()` borrow.
    fn ply2_row(&mut self) -> Ply2Row {
        let (state, threats, _eval) = self.position.staged_context();
        let left = StonesLeft::from_state(state).unwrap_or_else(|| {
            panic!(
                "pistol-search invariant {QUIESCENCE_ON_A_DECIDED_POSITION}: quiescence ply-2 \
                 was called on a decided position"
            )
        });
        let us = state.to_move();

        // No win check here: design §3.5's own proof — trigger (a) at the
        // gate already ruled out every live window of `us` at own >= 4, and
        // one stone raises at most one window's own-count by one, so no
        // window reaches own == 5 by the time ply-2 is reached.
        let cover2 = threats.blocking_covers(us, HitBudget::from(left));
        if matches!(&cover2, Cover::Impossible) {
            return Ply2Row::Overload;
        }
        let mut cells = cover2.cells();
        let mut offense = Vec::new();
        threats.cells_raising_to_hot(us, NearHot::Three, &mut offense);
        cells.extend(offense);
        cells.sort_unstable();
        cells.dedup();
        Ply2Row::Cells(cells)
    }

    /// THE COMPLETION STONE (design §3.5a): exactly one deterministic pick,
    /// branching 1, never search. Tier 1 — the union of live windows'
    /// support, both sides — falling back to Tier 2 — the mover's own ply-1
    /// stone's six neighbours — if Tier 1 is empty. Does not place the cell;
    /// the caller does (`Run::place_completion`), so this method borrows
    /// `self.position` only for reading.
    fn completion_stone(&mut self, ply1_stone: Coord) -> Coord {
        let tier1 = {
            let (state, threats, _eval) = self.position.staged_context();
            let us = state.to_move();
            let them = us.opponent();
            let mut cells = Vec::new();
            let mut buf = Vec::new();
            for side in [us, them] {
                threats.threat_cells(side, &mut buf);
                cells.extend_from_slice(&buf);
                threats.win_in_one_ply_cells(side, &mut buf);
                cells.extend_from_slice(&buf);
                threats.live_cells_at_count(side, LiveCount::Two, &mut buf);
                cells.extend_from_slice(&buf);
                threats.live_cells_at_count(side, LiveCount::Three, &mut buf);
                cells.extend_from_slice(&buf);
            }
            // Design's revision-5, N3-fixed: each query above sorts and
            // dedups only its OWN buffer (`query.rs`'s module doc) — the
            // UNION of eight such buffers is not itself sorted, so it is
            // concatenated and THEN sorted and deduplicated as one step,
            // exactly what `crate::staged::tier_t_union` already does.
            cells.sort_unstable();
            cells.dedup();
            cells
        };
        if !tier1.is_empty() {
            return self.argmax_by_delta(&tier1);
        }

        let board = self.position.board();
        let mut tier2: Vec<Coord> = NEIGHBOUR_DIRECTIONS
            .iter()
            .map(|&direction| ply1_stone.offset(direction))
            .filter(|&at| !board.is_occupied(at))
            .collect();
        // Design's revision-5, N4-fixed: `NEIGHBOUR_DIRECTIONS`' own ring
        // order is used only to enumerate the six candidate cells — the
        // tie-break is ascending `(q, r)`, the same convention Tier 1 and
        // D-5/D-7 use, sorted here rather than assumed from enumeration
        // order.
        tier2.sort_unstable();
        if tier2.is_empty() {
            panic!(
                "pistol-search invariant {NO_COMPLETION_STONE}: neither the live-window-support \
                 union nor the mover's own six neighbours offered a legal cell"
            );
        }
        self.argmax_by_delta(&tier2)
    }

    /// Argmax over `Eval::delta` (via `Position::static_score_after`, which
    /// never mutates the position), fixed coordinate-order tie-break: the
    /// running best is replaced only on STRICT improvement, so the
    /// first-encountered maximum over an ascending-`(q, r)` set survives ties
    /// — the same rule `crate::staged::delta_rank` states.
    ///
    /// # Panics
    ///
    /// If `cells` is empty — both callers guard against that themselves.
    fn argmax_by_delta(&mut self, cells: &[Coord]) -> Coord {
        let mut best = cells[0];
        let mut best_score = self.position.static_score_after(best);
        for &at in &cells[1..] {
            let score = self.position.static_score_after(at);
            if score > best_score {
                best_score = score;
                best = at;
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use pistol_core::{Coord, GameState, Player};
    use pistol_eval::Eval;

    use super::*;
    use crate::params::{CandidatePolicy, QTriggers, StagedParams};
    use crate::position::Position;
    use crate::score::mate_in;
    use crate::stop::Stop;
    use crate::tt::Table;

    /// An evaluation that thinks nothing of anything — as `pvs::tests::Flat`
    /// does for the main search's own tests, duplicated here rather than
    /// imported: `Flat` is private to `pvs`'s own test module, and these
    /// tests are about the gate's own decisions, not about scores an eval
    /// backend would supply.
    struct Flat;

    impl Eval for Flat {
        fn apply(&mut self, _at: Coord, _player: Player) {}
        fn undo(&mut self, _at: Coord, _player: Player) {}
        fn value(&self, _side_to_move: Player) -> i32 {
            0
        }
    }

    /// `win_in_one_ply_position()`'s own plies (`staged_tests.rs`), the SAME
    /// position via a manually-interleaved `GameState::from_plies` call:
    /// `common::position`'s interleaving convenience lives in the
    /// integration test tree and is not reachable from a `src`-level unit
    /// test. P1 to move, phase First, five in a row on `r=0` from `(0,0)` to
    /// `(4,0)`, needing `(5,0)` or `(-1,0)` to complete.
    fn win_in_one_ply_position() -> GameState {
        GameState::from_plies(&[
            Coord::new(0, 0),
            Coord::new(-1, 0),
            Coord::new(1, 3),
            Coord::new(1, 0),
            Coord::new(2, 0),
            Coord::new(2, 3),
            Coord::new(3, 3),
            Coord::new(3, 0),
            Coord::new(4, 0),
            Coord::new(1, 5),
            Coord::new(2, 5),
        ])
        .expect("win_in_one_ply_position is a legal game")
    }

    /// P1 to move, phase First: P2 holds a single hot window (own count 4,
    /// closed on one side by P1's own `(2,3)`, live only at `(7,3)`/`(8,3)`)
    /// — a single plan of size two, hit by either cell alone, so t = 1
    /// (`LAW-HIT`: one defender stone in any empty cell of a window kills it
    /// permanently). P1 itself holds no threat of any kind, isolating
    /// trigger (b) from trigger (a)/(c).
    fn opponent_t_le_1_position() -> GameState {
        GameState::from_plies(&[
            Coord::new(0, 0),
            Coord::new(3, 3),
            Coord::new(4, 3),
            Coord::new(2, 3),
            Coord::new(-1, -1),
            Coord::new(5, 3),
            Coord::new(6, 3),
        ])
        .expect("opponent_t_le_1_position is a legal game")
    }

    /// P1 to move, phase First: P1 holds a live-3 window (own count 3, both
    /// ends open at `(-1,0)`/`(3,0)`) — `DEF-PLAN` requires own count >= 4,
    /// so this is not a plan and trigger (a)/(b) do not fire from it, but
    /// `cells_raising_to_hot(P1, NearHot::Three)` is non-empty (trigger c's
    /// own query). P2 holds four stones split 2-and-2 across two axes, no
    /// four in a row on any of them, so P2 has no hot window at all —
    /// `blocking_covers(P1, ..)` is `Cover::NothingToBlock`, isolating
    /// trigger (c) from trigger (b).
    fn mover_can_activate_position() -> GameState {
        GameState::from_plies(&[
            Coord::new(0, 0),
            Coord::new(3, 3),
            Coord::new(4, 3),
            Coord::new(1, 0),
            Coord::new(2, 0),
            Coord::new(3, 5),
            Coord::new(4, 6),
        ])
        .expect("mover_can_activate_position is a legal game")
    }

    fn staged_params(q_depth_turns: u32, q_triggers: QTriggers) -> StagedParams {
        StagedParams {
            quiet_radius: 2,
            tier_t_own_count: 2,
            tier_t_opponent_count: 3,
            q_depth_turns,
            q_triggers,
            ordering: crate::params::OrderingHeuristics {
                killers: false,
                history: false,
                countermove: false,
            },
        }
    }

    /// A search built to walk this one position as its own root — `ply` 0 is
    /// exactly the horizon `pvs::visit` would hand `Run::quiescence`, which
    /// is what lets these tests call it directly rather than driving a whole
    /// multi-turn `Searcher::search` to land on one by chance.
    fn run_over<'a>(
        position: &'a mut Position,
        table: &'a mut Table,
        q_depth_turns: u32,
        q_triggers: QTriggers,
        heuristics: &'a mut crate::heuristics::HeuristicTables,
    ) -> Run<'a> {
        Run::new(
            position,
            table,
            CandidatePolicy::Staged(staged_params(q_depth_turns, q_triggers)),
            Stop::Nodes(u64::MAX),
            64,
            heuristics,
        )
    }

    #[test]
    fn trigger_a_at_the_gate_finds_a_win_the_static_eval_cannot_see() {
        // The gap docs/wp16_quiescence_design.md §2.1 names: before this WP,
        // `pvs::visit`'s horizon returned `self.position.value()`
        // unconditionally, with no threat query at all.
        let mut position = Position::new(Box::new(Flat), true);
        position.reset_to(&win_in_one_ply_position());
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let mut heuristics = crate::heuristics::HeuristicTables::new();
        let mut run = run_over(
            &mut position,
            &mut table,
            0,
            QTriggers::DefensiveAndOffensive,
            &mut heuristics,
        );

        let score = run.quiescence(-INFINITY, INFINITY, 0, 0);
        assert_eq!(
            score,
            mate_in(1),
            "P1's win-in-one must be found at the horizon, not scored as a flat 0"
        );
        assert_eq!(run.stages.q_win_now, 1);
        assert_eq!(
            run.stages.q_extend_defense + run.stages.q_extend_offense,
            0,
            "trigger (a) is a zero-cost shortcut — it must never look like an extension"
        );
    }

    #[test]
    fn q_depth_turns_zero_stands_pat_even_when_a_trigger_fires() {
        let mut position = Position::new(Box::new(Flat), true);
        position.reset_to(&opponent_t_le_1_position());
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let mut heuristics = crate::heuristics::HeuristicTables::new();
        let mut run = run_over(
            &mut position,
            &mut table,
            0,
            QTriggers::DefensiveAndOffensive,
            &mut heuristics,
        );

        let score = run.quiescence(-INFINITY, INFINITY, 0, 0);
        assert_eq!(
            score, 0,
            "with the extension disabled, a fired trigger must still stand pat on the static value"
        );
        assert_eq!(
            run.stages.q_stand_pat_cap, 1,
            "the trigger fired but the budget refused it — that must be counted separately from \
             `q_stand_pat_no_trigger`"
        );
        assert_eq!(run.stages.q_extend_defense, 0);
    }

    #[test]
    fn a_defensive_trigger_grants_an_extension_when_budget_allows() {
        let mut position = Position::new(Box::new(Flat), true);
        position.reset_to(&opponent_t_le_1_position());
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let mut heuristics = crate::heuristics::HeuristicTables::new();
        let mut run = run_over(
            &mut position,
            &mut table,
            1,
            QTriggers::DefensiveAndOffensive,
            &mut heuristics,
        );

        run.quiescence(-INFINITY, INFINITY, 0, 1);
        assert_eq!(
            run.stages.q_extend_defense, 1,
            "t <= 1 at the gate must grant an extension when the budget allows one"
        );
        assert_eq!(run.stages.q_stand_pat_cap, 0);
        assert!(
            run.stages.qnodes >= 1,
            "a granted turn visits at least its own ply-1 node"
        );
    }

    #[test]
    fn a_defensive_trigger_grants_an_extension_under_defensive_only_too() {
        // D-396: q_triggers gates trigger (c) only — trigger (b) is
        // unaffected by either arm.
        let mut position = Position::new(Box::new(Flat), true);
        position.reset_to(&opponent_t_le_1_position());
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let mut heuristics = crate::heuristics::HeuristicTables::new();
        let mut run = run_over(
            &mut position,
            &mut table,
            1,
            QTriggers::DefensiveOnly,
            &mut heuristics,
        );

        run.quiescence(-INFINITY, INFINITY, 0, 1);
        assert_eq!(
            run.stages.q_extend_defense, 1,
            "trigger (b) must fire under DefensiveOnly exactly as it does under \
             DefensiveAndOffensive"
        );
    }

    #[test]
    fn an_offensive_trigger_grants_an_extension_under_defensive_and_offensive() {
        let mut position = Position::new(Box::new(Flat), true);
        position.reset_to(&mover_can_activate_position());
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let mut heuristics = crate::heuristics::HeuristicTables::new();
        let mut run = run_over(
            &mut position,
            &mut table,
            1,
            QTriggers::DefensiveAndOffensive,
            &mut heuristics,
        );

        run.quiescence(-INFINITY, INFINITY, 0, 1);
        assert_eq!(
            run.stages.q_extend_offense, 1,
            "trigger (c) must fire under DefensiveAndOffensive when the mover can activate a \
             new plan"
        );
        assert_eq!(run.stages.q_extend_defense, 0);
    }

    #[test]
    fn an_offensive_trigger_cannot_fire_under_defensive_only() {
        // The dispatch's own required proof (D-396, Step 2): the SAME
        // position that grants an offensive extension under
        // DefensiveAndOffensive must stand pat under DefensiveOnly, exactly
        // as if trigger (c) did not exist.
        let mut position = Position::new(Box::new(Flat), true);
        position.reset_to(&mover_can_activate_position());
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let mut heuristics = crate::heuristics::HeuristicTables::new();
        let mut run = run_over(
            &mut position,
            &mut table,
            1,
            QTriggers::DefensiveOnly,
            &mut heuristics,
        );

        let score = run.quiescence(-INFINITY, INFINITY, 0, 1);
        assert_eq!(
            run.stages.q_extend_offense, 0,
            "trigger (c) must never fire under DefensiveOnly"
        );
        assert_eq!(
            run.stages.q_stand_pat_no_trigger, 1,
            "with trigger (c) gated off and trigger (b) not applicable (P2 has no hot window), \
             the gate must stand pat"
        );
        assert_eq!(score, 0, "Flat's static value, unconditionally");
    }

    #[test]
    fn quiescence_is_deterministic_across_repeated_calls() {
        // CLAUDE.md rule 4: the same position and budget must produce the
        // same score and the same node count every time, single-threaded.
        let run_once = || {
            let mut position = Position::new(Box::new(Flat), true);
            position.reset_to(&opponent_t_le_1_position());
            let mut table = Table::new(1 << 20).expect("the smallest table");
            let mut heuristics = crate::heuristics::HeuristicTables::new();
            let mut run = run_over(
                &mut position,
                &mut table,
                3,
                QTriggers::DefensiveAndOffensive,
                &mut heuristics,
            );
            let score = run.quiescence(-INFINITY, INFINITY, 0, 3);
            (score, run.total_nodes(), run.stages)
        };
        let first = run_once();
        let second = run_once();
        assert_eq!(first, second, "two identical calls must agree exactly");
    }
}
