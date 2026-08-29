use std::time::Instant;

use pistol_core::{Coord, Phase, PlyOutcome};

use crate::candidates::candidate_cells;
use crate::heuristics::HeuristicTables;
use crate::ordering::{OrderOutcome, order};
use crate::params::CandidatePolicy;
use crate::position::Position;
use crate::pv::PvTable;
use crate::score::{INFINITY, mate_in};
use crate::staged::{StagedRow, StagedSet, staged_candidates};
use crate::stop::{NODE_CHECK_INTERVAL, Stop};
use crate::tt::{Bound, Record, Table};

/// Named invariant: the candidate policy offered a cell the rules refuse.
pub const CANDIDATE_ILLEGAL: &str = "CANDIDATE_ILLEGAL";

/// Named invariant: the candidate policy offered nothing half way through a
/// turn, where no static value is an answer (docs/decisions.md D-104).
pub const NO_CANDIDATES_MID_TURN: &str = "NO_CANDIDATES_MID_TURN";

/// Named invariant: a static evaluation was returned as a node's answer half way
/// through a turn (docs/decisions.md D-111).
///
/// A mover who still owes a stone has not finished doing anything, so the
/// pattern tables are reading a position no player will ever face. The search
/// deepens in turns and every horizon therefore lands on a turn boundary; this
/// is what says so at the place that would first be wrong.
pub const STATIC_EVAL_MID_TURN: &str = "STATIC_EVAL_MID_TURN";

/// One search: everything that belongs to this call and nothing that outlives
/// it.
///
/// The table and the position do outlive it and are borrowed. Everything else —
/// the node count, the abort flag, the lines found — is created here and dropped
/// at the end, so no state can bleed from one search into the next
/// (docs/decisions.md D-7).
pub struct Run<'a> {
    /// `pub(crate)`: `crate::quiescence`'s own methods on `Run` need it too
    /// (docs/wp16_quiescence_design.md §3 — a dedicated module, not a
    /// duplicated one).
    pub(crate) position: &'a mut Position,
    /// `pub(crate)`, same reason as `position`.
    pub(crate) table: &'a mut Table,
    /// `pub(crate)`, same reason as `position`: `crate::quiescence`'s gate
    /// reads `StagedParams::q_triggers` off it (D-396).
    pub(crate) policy: CandidatePolicy,
    /// WP-1.7's ordering-heuristic tables — borrowed from the
    /// [`crate::search::Searcher`] so they persist across the searches of
    /// one game and are cleared by `newgame`
    /// (`docs/experiments/wp17_design.md` §3.1). `pub(crate)` for the same
    /// reason as `position`: nothing outside the crate constructs a `Run`.
    pub(crate) heuristics: &'a mut HeuristicTables,
    stop: Stop,
    root_turn: u32,
    /// SEARCH nodes visited, leaves included — ONE of the two independent
    /// counters (design wp18b §3): incremented once per visit and
    /// quiescence node, never by a solver call. The budget counter callers
    /// read is [`Run::total_nodes`], the derived sum.
    pub search_nodes: u64,
    /// SOLVER nodes spent on calls from this search — the OTHER independent
    /// counter, incremented the moment each call returns.
    pub solver_nodes: u64,
    /// How many solver calls were refused by the zone containment
    /// invariant (`NoWinUnderZone`) — loud, never swallowed (design
    /// wp18b §8).
    pub solver_refusals: u32,
    /// The solver on the search path and its wiring (design wp18b §2),
    /// bundled so the OFF gate is ONE `None` — no solver, no wiring, no
    /// dead values. Borrowed from the [`crate::search::Searcher`].
    pub(crate) solver: Option<(&'a mut pistol_solver::Solver, crate::params::SolverWiring)>,
    /// The root candidate restriction a proven defender loss imposes
    /// (design wp18b §2 D3): zone cells at the root, applied at ply 0,
    /// fail-open on an empty intersection.
    pub(crate) root_restrict: Option<Vec<pistol_core::Coord>>,
    /// The deepest turn count any line reached.
    pub seldepth_turns: u32,
    /// The node protocol's stage-share counters — all zero under
    /// `CandidatePolicy::Radius` (docs/decisions.md U2-M item 2). Written from
    /// the same point `nodes` is, on every node the staged dispatch reaches, so
    /// a caller that only ever reads it through [`SearchInfo::stages`] sees
    /// whole-search totals identically to `nodes` (`crate::search::Searcher::search`
    /// copies it there on every construction path, including both salvage
    /// ones).
    pub stages: crate::info::StageCounters,
    /// Set once the stop condition has fired; every node above unwinds without
    /// using its result.
    pub aborted: bool,
    /// Whether this iteration may be abandoned. Under a reproducible stop the
    /// first one may not — a search that returned no move at all would be a
    /// silent failure (docs/decisions.md D-74); under a wall-clock stop every
    /// iteration may, because the caller secured a fallback answer before
    /// deepening (D-74 as amended by WP-1.4).
    abortable: bool,
    /// The score of the last ply-0 promotion of the CURRENT iteration, reset by
    /// [`Run::iterate`]. Written on every ply-0 promotion whatever the stop
    /// kind — the write is behavior-neutral — and read only by
    /// [`Run::salvage`], which only a wall-clock caller consults.
    root_score: Option<i32>,
    /// `pub(crate)`, same reason as `position`.
    pub(crate) pv: PvTable,
}

impl<'a> Run<'a> {
    /// Begin a search from the position, which must be ongoing and at a turn
    /// boundary.
    pub fn new(
        position: &'a mut Position,
        table: &'a mut Table,
        policy: CandidatePolicy,
        stop: Stop,
        max_ply: usize,
        heuristics: &'a mut HeuristicTables,
    ) -> Run<'a> {
        let root_turn = position.state().turn();
        Run {
            position,
            table,
            policy,
            heuristics,
            stop,
            root_turn,
            search_nodes: 0,
            solver_nodes: 0,
            solver_refusals: 0,
            solver: None,
            root_restrict: None,
            seldepth_turns: 0,
            stages: crate::info::StageCounters::default(),
            aborted: false,
            abortable: false,
            root_score: None,
            pv: PvTable::new(max_ply),
        }
    }

    /// The budget counter: the derived sum of the two independent counters
    /// (design wp18b §3). Every stop check and every report reads THIS.
    pub fn total_nodes(&self) -> u64 {
        self.search_nodes + self.solver_nodes
    }

    /// Search one iteration to `depth_plies`, returning its score, or `None` if
    /// the stop condition fired before it finished.
    pub fn iterate(&mut self, depth_plies: u32, abortable: bool) -> Option<i32> {
        self.abortable = abortable;
        // Reset with the iteration, exactly as `visit` clears the ply-0 line on
        // entry: an abort landing before this iteration's first ply-0 promotion
        // must find nothing salvageable, not the previous iteration's score
        // beside an empty line (the decision-red-team's MAJOR-4a).
        self.root_score = None;
        let score = self.visit(depth_plies, -INFINITY, INFINITY, 0);
        (!self.aborted).then_some(score)
    }

    /// The line under the root, in plies.
    pub fn line(&self) -> &[Coord] {
        self.pv.line(0)
    }

    /// What the aborted iteration had already proved at the root: the last
    /// ply-0 promotion's line and its exact score, or `None` if the abort
    /// landed before any root candidate's subtree completed.
    ///
    /// Sound because a ply-0 promotion only ever happens on a COMPLETED child
    /// subtree — `visit` returns an aborted child's sentinel before its score
    /// can be used — and root beta is infinite, so a root promotion is never a
    /// fail-high adopting a truncated null-window line: the line is turn-whole
    /// and the score exact. The first root candidate is the table's move, which
    /// is the previous iteration's best, so a salvaged answer is never
    /// worse-informed than the last completed depth's (WP-1.4's decision,
    /// verified line-by-line by its decision-red-team).
    ///
    /// Only a wall-clock caller may consult this: using it under a node budget
    /// would change reproducible-budget answers, which are pinned byte-for-byte
    /// (CLAUDE.md rule 4, the instrument golden transcripts).
    pub fn salvage(&self) -> Option<(i32, &[Coord])> {
        if !self.aborted {
            return None;
        }
        let line = self.pv.line(0);
        // The score and the line are written together by a ply-0 promotion and
        // reset together at iteration entry; one without the other is a desync.
        assert_eq!(
            self.root_score.is_some(),
            !line.is_empty(),
            "pistol-search invariant: a salvaged root score and its line must exist together"
        );
        self.root_score.map(|score| (score, line))
    }

    /// How full the table is, in parts per thousand.
    pub fn hashfull_permille(&self) -> u32 {
        self.table.hashfull_permille()
    }

    /// The value of this position to the side to move, searched `depth_plies`
    /// deeper, given that the caller already has `alpha` and will not pay more
    /// than `beta`.
    fn visit(&mut self, depth_plies: u32, mut alpha: i32, beta: i32, ply: usize) -> i32 {
        self.search_nodes += 1;
        self.pv.clear(ply);
        self.seldepth_turns = self.seldepth_turns.max(self.turns_from_root());
        if self.should_stop() {
            return 0;
        }
        if depth_plies == 0 {
            // The horizon is a turn boundary or it is not a horizon
            // (docs/decisions.md D-111). `plies_for` sums the stones each turn
            // ahead owes, so the ply budget runs out exactly where a turn does,
            // and a mid-turn horizon would mean that arithmetic — or a future
            // extension's — had drifted. A diagnostic and not a correctness
            // invariant, so it is `debug_assert!` under D-129's taxonomy: it
            // runs at every leaf, and the profiles that read it are `cargo
            // test`'s and `release-checked`'s (D-128). The other horizon, the
            // one an empty candidate set reaches, is a correctness invariant and
            // carries the same statement as an always-on `assert!` under
            // `NO_CANDIDATES_MID_TURN` below.
            debug_assert!(
                self.position.state().phase() == Phase::First,
                "pistol-search invariant {STATIC_EVAL_MID_TURN}: the horizon landed at phase 1, \
                 where the mover still owes a stone — a horizon must extend one ply to complete \
                 the turn rather than evaluate half of it"
            );
            // WP-1.6 (docs/wp16_quiescence_design.md §3): threat-only
            // quiescence extends the horizon under `Staged` only — `Radius`
            // tracks no `ThreatState` (`position.rs`'s own doc) and this
            // match is the SAME one the `depth_plies > 0` branch below
            // already runs to dispatch on `self.policy`.
            return match self.policy {
                CandidatePolicy::Radius { .. } => self.position.value(),
                CandidatePolicy::Staged(params) => {
                    self.quiescence(alpha, beta, ply, params.q_depth_turns)
                }
            };
        }

        let key = self.position.state().key();
        let from_root = self.turns_from_root();
        // A principal variation node never takes a cutoff from the table: the
        // score would come back without the line that proves it, and the
        // reported variation would stop short of the horizon.
        let is_pv = beta - alpha > 1;
        let known = self.table.probe(key, from_root);
        if let Some(record) = known
            && !is_pv
            && record.depth_plies >= depth_plies
            && match record.bound {
                Bound::Exact => true,
                Bound::Lower => record.score >= beta,
                Bound::Upper => record.score <= alpha,
            }
        {
            return record.score;
        }

        // THE SOLVER ON THE SEARCH PATH (design wp18b §2 D1): at turn
        // boundaries under the Staged policy, when the trigger fires, a
        // proven policy-game value ends the node — a VALUE, never an
        // ordering hint. Ply 0 never calls: the root's own calls happened
        // before deepening (`crate::search::Searcher::search`), and paying
        // for them twice would double-count the same nodes against the
        // budget.
        if ply > 0
            && self.position.state().phase() == Phase::First
            && self.solver.is_some()
            && matches!(self.policy, CandidatePolicy::Staged(_))
            && let Some(verdict) = self.solver_verdict()
        {
            return verdict;
        }

        // A deadline can land inside the scoring loop — its length is the
        // candidate count, which the opponent partly grows (D-95) — so under a
        // wall-clock stop the ordering itself checks the clock and the node
        // aborts like any other; the partially scored order is discarded with
        // it. Reproducible stops pass `None` and read no clock (rule 4).
        let table_move = known.map(|record| record.best);
        // The unforced range's start under `Staged` (`None` under `Radius`:
        // the heuristic gates exist only inside `StagedParams`, so nothing
        // is recorded there and the bound is never read).
        let mut forced_bound: Option<usize> = None;
        // Whether this node's emitted set was cut by the safety-net cap. Read
        // again at the store below: a node that did not search its whole set
        // proved a lower bound and nothing else (§6.3's store rule).
        let mut truncated = false;
        let cells = match self.policy {
            CandidatePolicy::Radius { .. } => {
                let mut cells = candidate_cells(self.position.board(), self.policy);
                if cells.is_empty() {
                    self.no_candidates_at_a_turn_boundary();
                    return self.position.value();
                }
                if order(self.position, &mut cells, table_move, self.order_deadline())
                    == OrderOutcome::DeadlinePassed
                {
                    self.aborted = true;
                    return 0;
                }
                cells
            }
            CandidatePolicy::Staged(params) => {
                let (state, threats, eval) = self.position.staged_context();
                let mut set = StagedSet::default();
                let row = staged_candidates(state, threats, eval, is_pv, params, &mut set);
                self.stages.record(row);
                if set.used_quiet_safety_net {
                    self.stages.record_quiet_safety_net();
                }
                match row {
                    // `PROTO-NODE` step 2's early return (`U2_node_protocol.md`
                    // §5.2): the guard is step 1's `None` arm above, already
                    // taken; the distance is `k + 2` (our turn completes at
                    // `k+1`, the opponent's overload win at `k+2`); the
                    // `!is_pv` gate is `staged_candidates`'s own `is_pv`
                    // argument, so `visit` never re-asks it. No child is
                    // expanded — `self.search_nodes` was already incremented at entry,
                    // and that is the whole node cost this row pays.
                    StagedRow::OverloadReturn => {
                        return -mate_in(self.turns_from_root() + 2);
                    }
                    StagedRow::WinNow
                    | StagedRow::Filtered
                    | StagedRow::Batched
                    | StagedRow::BatchedLost => {}
                }
                if set.cells.is_empty() {
                    self.no_candidates_at_a_turn_boundary();
                    return self.position.value();
                }
                // THE SAFETY-NET CAP (docs/decisions.md D-478, D-482;
                // docs/experiments/wp15d_design.md §2.2). On the row where Tier
                // F and Tier T are both empty the emitted set is the whole quiet
                // ball, which is unbounded in the stone count. The exemption is
                // the ROOT TURN, spelled in turns rather than plies because rule
                // 3 gives turn 1 one stone and every later turn two, so no ply
                // threshold names the played turn at every turn number.
                // A K too large for `usize` is a cap above every possible pool,
                // which is a cap that never binds -- NOT `truncate(0)`, which an
                // `as` narrowing would produce on a 32-bit target and which
                // would empty the set and panic at `cells[0]`. No target is
                // pinned anywhere in this repository, so the saturation is the
                // guard rather than an argument about which one this is.
                let cap = usize::try_from(params.safety_net_top_k).unwrap_or(usize::MAX);
                if params.safety_net_top_k > 0
                    && self.turns_from_root() > 0
                    && set.used_quiet_safety_net
                    && set.cells.len() > cap
                {
                    truncated = true;
                    self.stages
                        .record_safety_net_cap(set.cells.len() as u64, cap as u64);
                    set.cells.truncate(cap);
                }
                // Promotion runs AFTER the truncation, so the table's move is
                // promoted within the cut set and can never re-admit a cell the
                // cap removed (§2.3).
                set.promote_table_move(table_move);
                // The root's proven-loss zone restriction (design wp18b §2
                // D3): candidates outside the opponent proof's Z2 zone drop
                // out at ply 0 ONLY, the forced prefix is never touched
                // (its cells are the plan covers, inside the zone by
                // construction), and an empty filtered set — or any forced
                // cell outside the zone, which the containment argument
                // says cannot happen — FAILS OPEN to the unrestricted set.
                if let Some(zone) = self.root_restrict.as_ref().filter(|_| ply == 0) {
                    let forced_intact = set.cells[..set.forced]
                        .iter()
                        .all(|cell| zone.binary_search(cell).is_ok());
                    if forced_intact {
                        let unrestricted = set.cells.clone();
                        set.cells.retain(|cell| zone.binary_search(cell).is_ok());
                        if set.cells.is_empty() {
                            set.cells = unrestricted;
                        }
                    }
                }
                // WP-1.7: the ordering heuristics reorder the unforced range
                // AFTER the table's own move, never across the Tier-F
                // boundary (`docs/experiments/wp17_design.md` §2-§3).
                if params.ordering.any() {
                    let (state, threats, _) = self.position.staged_context();
                    self.heuristics.order_candidates(
                        params.ordering,
                        state,
                        threats,
                        ply,
                        table_move,
                        &mut set,
                    );
                }
                forced_bound = Some(set.forced);
                set.cells
            }
        };

        let original_alpha = alpha;
        let mut best_score = -INFINITY;
        let mut best_cell = cells[0];
        let mut best_index = 0;

        for (index, &at) in cells.iter().enumerate() {
            let outcome = self.position.place(at).unwrap_or_else(|error| {
                panic!(
                    "pistol-search invariant {CANDIDATE_ILLEGAL}: the candidate policy offered \
                     {at}, and the rules say: {error}"
                )
            });
            let won = matches!(outcome, PlyOutcome::Win { .. });
            let score = match outcome {
                // Rule 4: this stone ended the turn and the game. The distance
                // is in turns from the root, and both stones of a turn share
                // one turn number, so a win with either scores the same
                // (docs/decisions.md D-72).
                PlyOutcome::Win { .. } => {
                    self.pv.clear(ply + 1);
                    // A won line reached the turn it wins on, and no child node
                    // is visited to record that: without this, a mate found on
                    // the first root candidate reports a completed depth of one
                    // turn beside a selective depth of none, which contradicts
                    // what `SearchInfo::seldepth_turns` promises.
                    let turns = self.turns_from_root() + 1;
                    self.seldepth_turns = self.seldepth_turns.max(turns);
                    mate_in(turns)
                }
                // The same side owes another stone: same window, no flip.
                PlyOutcome::TurnContinues => {
                    self.child(depth_plies - 1, alpha, beta, ply + 1, index == 0, true)
                }
                // The turn is complete and the opponent is to move.
                PlyOutcome::TurnComplete => {
                    self.child(depth_plies - 1, alpha, beta, ply + 1, index == 0, false)
                }
            };
            self.position.undo();
            if self.aborted {
                return 0;
            }

            if score > best_score {
                best_score = score;
                best_cell = at;
                best_index = index;
                if score > alpha {
                    alpha = score;
                    self.pv.promote(ply, at);
                    if ply == 0 {
                        // The root's best-so-far, kept beside the line the
                        // promotion just wrote so a wall-clock abort can answer
                        // with what this iteration already proved
                        // (`Run::salvage`). Behavior-neutral for every other
                        // stop kind: written always, read never.
                        self.root_score = Some(score);
                    }
                }
            }
            // A win is the best this node can do — every winning stone here
            // completes on the same turn — so there is nothing left to try.
            if won || alpha >= beta {
                break;
            }
        }

        if !self.aborted {
            // WP-1.7: a genuine beta cutoff whose winning candidate was in
            // the UNFORCED range is the one event that feeds the ordering
            // heuristics (`docs/experiments/wp17_design.md` §3.2). The index
            // test is what keeps Tier-F cells — and therefore winning
            // placements, which on every row with an unforced range can only
            // come from the forced prefix — out of the quiet-refutation
            // tables.
            if let CandidatePolicy::Staged(params) = self.policy
                && params.ordering.any()
                && best_score >= beta
                && forced_bound.is_some_and(|forced| best_index >= forced)
            {
                self.heuristics
                    .record_cutoff(self.position.state(), ply, best_cell);
            }
            let bound = if best_score <= original_alpha {
                Bound::Upper
            } else if best_score >= beta {
                Bound::Lower
            } else {
                Bound::Exact
            };
            // §6.3's store rule (`WPQ_seed.md` §7.2). A truncated node proved a
            // LOWER bound and nothing else: a move it did search reached beta,
            // and the full set can only do better. `Upper` and `Exact` claim
            // that nothing did better, over a set it never exhausted — and the
            // table outlives the search (`Searcher`'s own doc), so such a claim
            // is probed later at a node that emits the whole set, inside the
            // root turn this cap is chosen for leaving whole.
            if truncated && bound != Bound::Lower {
                match bound {
                    Bound::Upper => self.stages.safety_net_upper_withheld += 1,
                    _ => self.stages.safety_net_exact_withheld += 1,
                }
            } else {
                self.table.store(
                    key,
                    from_root,
                    Record {
                        depth_plies,
                        score: best_score,
                        static_eval: self.position.value(),
                        bound,
                        best: best_cell,
                        from_quiescence: false,
                    },
                );
            }
        }
        best_score
    }

    /// Search one child: in full for the first candidate, and behind a
    /// null window for the rest, re-searching only the ones that beat it.
    ///
    /// `same_side` is the second stone of the mover's own turn, which keeps the
    /// window as it stands; anything else is the opponent's reply and negates
    /// it.
    fn child(
        &mut self,
        depth_plies: u32,
        alpha: i32,
        beta: i32,
        ply: usize,
        first: bool,
        same_side: bool,
    ) -> i32 {
        let full = |run: &mut Self| {
            if same_side {
                run.visit(depth_plies, alpha, beta, ply)
            } else {
                -run.visit(depth_plies, -beta, -alpha, ply)
            }
        };
        if first {
            return full(self);
        }
        let scan = if same_side {
            self.visit(depth_plies, alpha, alpha + 1, ply)
        } else {
            -self.visit(depth_plies, -alpha - 1, -alpha, ply)
        };
        // An aborted scan returned the sentinel, not a score, and the sentinel
        // sits inside most windows — so without this the re-search fires on a
        // number that means nothing, spends a node the budget never granted, and
        // does it once per level of the unwind. That is what made a search given
        // n nodes report more than `n.next_multiple_of(NODE_CHECK_INTERVAL)`
        // (docs/decisions.md D-74).
        if self.aborted {
            return scan;
        }
        if scan > alpha && scan < beta {
            full(self)
        } else {
            scan
        }
    }

    /// How many turns this position is from the root. Both plies of a turn
    /// share it, because they share a turn number.
    ///
    /// `pub(crate)`: `crate::quiescence`'s own nodes compute mate distances
    /// the same way `visit`'s do (docs/wp16_quiescence_design.md §7).
    pub(crate) fn turns_from_root(&self) -> u32 {
        self.position.state().turn() - self.root_turn
    }

    /// Whether the budget has run out.
    ///
    /// A node budget is tested at a fixed node granularity so its stopping
    /// point is exact and reproducible (docs/decisions.md D-74). A deadline is
    /// tested at EVERY abortable node: a mask tuned for node budgets would let
    /// up to [`NODE_CHECK_INTERVAL`] nodes — each with a whole ordering pass —
    /// run past the clock, which is D-95's magnitude class, and a deadline stop
    /// is not reproducible anyway, so granularity buys it nothing.
    ///

    /// One trigger evaluation and its calls (design wp18b §2 D1's dispatch,
    /// D2's directions, §4's scores). Returns `Some(score)` when a proof
    /// ends the node, `None` when there is no verdict here. Solver nodes
    /// are absorbed into the budget the moment each call returns.
    ///
    #[allow(clippy::empty_line_after_doc_comments)]
    fn solver_verdict(&mut self) -> Option<i32> {
        // The &self reads first, so the position borrow below never has to
        // coexist with them through the receiver.
        let cap = self.solver.as_ref()?.1.per_call_node_cap;
        let from_root = self.turns_from_root();
        let (state, threats, _) = self.position.staged_context();
        let mover = state.to_move();
        let opponent = mover.opponent();
        let mover_hot = !threats.hot_windows(mover).is_empty();
        let opponent_hot = !threats.hot_windows(opponent).is_empty();
        if !mover_hot && !opponent_hot {
            return None;
        }
        // One clone serves both calls (the solver never mutates its input).
        let state_view = state.clone();
        // The attacker direction: does the MOVER force a policy-game win?
        // Asked whenever the trigger fired at all — a mover in check may
        // still force a deeper win through it (LAW-FORCE admits that).
        {
            let solver = &mut self.solver.as_mut()?.0;
            let result = solver.solve(&state_view, cap);
            self.solver_nodes = self.solver_nodes.saturating_add(result.nodes);
            match result.outcome {
                pistol_solver::SolveOutcome::Win(tree) => {
                    // The mover's d-th turn from this node sits at root
                    // offset from_root + 2d - 1 (design §4; the t-relative
                    // parity invariant is pinned by test).
                    let distance = from_root + 2 * tree.win_depth_turns() - 1;
                    return Some(crate::score::mate_in(distance));
                }
                pistol_solver::SolveOutcome::NoWin => {}
                pistol_solver::SolveOutcome::NoWinUnderZone => self.solver_refusals += 1,
                pistol_solver::SolveOutcome::Unknown => {}
            }
        }
        // The defender direction: does the OPPONENT force a win against the
        // mover's best defense? At a mover-hot-only node the race check
        // answers it in one visit (NoWin for the opponent) — correct, and
        // still made because one node is what it costs.
        {
            let solver = &mut self.solver.as_mut()?.0;
            let result = solver.solve_defender(&state_view, cap);
            self.solver_nodes = self.solver_nodes.saturating_add(result.nodes);
            match result.outcome {
                pistol_solver::SolveOutcome::Win(tree) => {
                    // The OPPONENT's d-th turn from this node sits at root
                    // offset from_root + 2d — the OverloadReturn shape's own
                    // d=1 instance (design §4).
                    let distance = from_root + 2 * tree.win_depth_turns();
                    return Some(-crate::score::mate_in(distance));
                }
                pistol_solver::SolveOutcome::NoWin => {}
                pistol_solver::SolveOutcome::NoWinUnderZone => self.solver_refusals += 1,
                pistol_solver::SolveOutcome::Unknown => {}
            }
        }
        None
    }

    /// `pub(crate)`: `crate::quiescence` calls this SAME method at its own
    /// entry rather than duplicating it (docs/wp16_quiescence_design.md §7).
    pub(crate) fn should_stop(&mut self) -> bool {
        if self.aborted {
            return true;
        }
        if !self.abortable {
            return false;
        }
        let check_now = match self.stop {
            Stop::Deadline(_) => true,
            Stop::DepthTurns(_) | Stop::Nodes(_) => {
                // THE MASK RUNS ON THE COUNTER THAT MOVES BY ONE (WP-1.8c §4d).
                // wp18b §3 put it on the DERIVED total, and a solver call
                // absorbs its whole node count at once — so an exact-multiple
                // test on that total steps OVER the multiples and does not
                // fire. MEASURED: the ON seat spent a mean 156,313 nodes per
                // position against a 50,000 budget, max 648,192. `search_nodes`
                // increments once per visit and so lands on every multiple; the
                // SPENT test below still reads the derived total, so the budget
                // stopped at is still the shared one.
                //
                // The second disjunct bounds the ON seat's overshoot by ONE
                // visit's own calls — which is what wp18b §3 claimed and did
                // not have. It is `false` for the whole life of every gate-off
                // search, so every committed config's node counts are
                // byte-unchanged.
                self.search_nodes.is_multiple_of(NODE_CHECK_INTERVAL) || self.solver_nodes > 0
            }
        };
        if !check_now {
            return false;
        }
        self.aborted = self.stop.is_spent(self.total_nodes());
        self.aborted
    }

    /// The deadline the ordering pass must respect, or `None` — and it is
    /// `None` for every reproducible stop and every non-abortable iteration,
    /// so no instrument path can reach a clock read through it.
    fn order_deadline(&self) -> Option<Instant> {
        match self.stop {
            Stop::Deadline(at) if self.abortable => Some(at),
            _ => None,
        }
    }

    /// The shared refusal both candidate policies raise when they offer
    /// nothing: correctness-invariant at `Phase::Second` (a policy running dry
    /// mid-turn has no honest static answer, docs/decisions.md D-104), and a
    /// no-op check at `Phase::First`, where the caller returns the static
    /// value as an honest leaf. Under `Staged` this is unreachable in practice
    /// — [`crate::staged::staged_candidates`]'s quiet-ball safety net (that
    /// module's doc) empties only when the rules themselves admit no move at
    /// all, which [`Position::place`] would already have refused — but the
    /// check stays, at the same strength `Radius` carries it at, rather than
    /// asserting the safety net can never come up short.
    fn no_candidates_at_a_turn_boundary(&self) {
        assert!(
            self.position.state().phase() == Phase::First,
            "pistol-search invariant {NO_CANDIDATES_MID_TURN}: the candidate policy offered \
             nothing at phase 1, where the mover still owes a stone — a policy that can run dry \
             must answer inside a turn, not only at its boundary"
        );
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use pistol_core::{Coord, GameState, Player};
    use pistol_eval::Eval;

    use super::*;
    use crate::pv::turns_from_plies;
    use crate::tt::Table;

    /// An evaluation that thinks nothing of anything, so the tests here are
    /// about the run's abort-and-salvage bookkeeping and not about scores.
    struct Flat;

    impl Eval for Flat {
        fn apply(&mut self, _at: Coord, _player: Player) {}
        fn undo(&mut self, _at: Coord, _player: Player) {}
        fn value(&self, _side_to_move: Player) -> i32 {
            0
        }
    }

    /// Turn 2 of a game: one stone on the origin, two plies owed.
    fn root() -> GameState {
        GameState::from_plies(&[Coord::ORIGIN]).expect("turn 1 on the origin is legal")
    }

    /// An instant that is already in the past — or, where the platform cannot
    /// step back, now, which every `>=` deadline test also accepts as spent.
    fn expired() -> Instant {
        let now = Instant::now();
        now.checked_sub(Duration::from_millis(1)).unwrap_or(now)
    }

    /// The decision-red-team's MAJOR-4a, pinned: an abort that lands before the
    /// new iteration promotes anything at the root must salvage NOTHING — not
    /// the previous iteration's score sitting beside a freshly cleared line.
    ///
    /// Mutation checked: removing the `root_score = None` reset in
    /// [`Run::iterate`] makes this test die on `salvage`'s pairing assertion.
    #[test]
    fn an_abort_before_any_root_promotion_salvages_nothing() {
        let state = root();
        let mut position = Position::new(Box::new(Flat), false);
        position.reset_to(&state);
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let policy = CandidatePolicy::Radius { radius: 1 };
        let mut heuristics = crate::heuristics::HeuristicTables::new();
        let mut run = Run::new(
            &mut position,
            &mut table,
            policy,
            Stop::Deadline(expired()),
            12,
            &mut heuristics,
        );

        // Iteration 1 is run non-abortable, so it completes over the expired
        // deadline and promotes at the root.
        let score = run.iterate(2, false);
        assert!(score.is_some(), "a non-abortable iteration completes");
        assert!(!run.line().is_empty(), "a completed iteration has a line");
        assert!(
            run.salvage().is_none(),
            "nothing is salvaged from a search that was not aborted"
        );

        // Iteration 2 is abortable and the deadline is long gone: the first
        // node aborts after clearing the root line and before any promotion.
        let score = run.iterate(4, true);
        assert!(score.is_none(), "the abortable iteration aborts");
        assert!(run.aborted, "the run records the abort");
        assert!(
            run.salvage().is_none(),
            "an abort before any root promotion salvages nothing from iteration 1"
        );
    }

    /// The salvage read path: a line the aborted iteration promoted at the root
    /// is turn-whole (it replays into whole turns from the root) and comes back
    /// with exactly the score its promotion recorded — the invariant that lets
    /// `Searcher::search` hand it to `turns_from_plies` without a
    /// `PV_NOT_PLAYABLE` panic being reachable from play mode.
    #[test]
    fn a_salvaged_root_line_is_turn_whole_and_keeps_its_promotion_score() {
        let state = root();
        let mut position = Position::new(Box::new(Flat), false);
        position.reset_to(&state);
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let policy = CandidatePolicy::Radius { radius: 1 };
        let far = Instant::now() + Duration::from_secs(3600);
        let mut heuristics = crate::heuristics::HeuristicTables::new();
        let mut run = Run::new(
            &mut position,
            &mut table,
            policy,
            Stop::Deadline(far),
            12,
            &mut heuristics,
        );

        let score = run
            .iterate(2, true)
            .expect("an hour of deadline completes a two-ply iteration");
        // Manufacture the abort AFTER the iteration completed its promotions:
        // the deterministic stand-in for a deadline landing between the last
        // root promotion and the iteration's end.
        run.aborted = true;

        let (salvaged, line) = run
            .salvage()
            .expect("an aborted iteration that promoted at the root salvages");
        assert_eq!(salvaged, score, "the salvage is the promotion's own score");
        let turns = turns_from_plies(&state, line);
        assert_eq!(
            turns.len(),
            1,
            "a two-ply line from turn 2 is exactly one whole turn"
        );
    }

    /// WP-1.7's M8 boundary, pinned at the node level: a beta cutoff whose
    /// cell sits in the FORCED prefix updates nothing AT ITS OWN PLY. The
    /// fixture is the staged tests' FILTERED-row one — P2 holds one hot
    /// window whose two empties are each a one-cell cover, so the whole
    /// candidate set is forced (`forced == cells.len()`) — and the visit
    /// runs under a null window at the bottom, so the FIRST candidate fails
    /// high: a genuine beta cutoff at index 0, a forced cell, at ply 0.
    /// Deeper nodes of the same visit take their own unforced cutoffs at
    /// deeper plies and legitimately record THERE; what must stay empty is
    /// the forced cutoff's own ply.
    ///
    /// Mutation checked: replacing `best_index >= forced` with `true` in
    /// `visit`'s post-loop recording gate makes this test red — Tier-F cells
    /// would enter the killer tables, the exact masquerade M8's row names as
    /// its failure mode.
    #[test]
    fn a_forced_prefix_cutoff_updates_nothing_at_its_own_ply() {
        // P1: (0,0), (-1,2), (5,3) — nothing aligned. P2: (0,2)..(3,2), one
        // hot window blocked behind at (-1,2), empties (4,2) and (5,2).
        let state = GameState::from_plies(&[
            Coord::ORIGIN,
            Coord::new(0, 2),
            Coord::new(1, 2),
            Coord::new(-1, 2),
            Coord::new(5, 3),
            Coord::new(2, 2),
            Coord::new(3, 2),
        ])
        .expect("the FILTERED-row fixture is a legal game");
        let mut position = Position::new(Box::new(Flat), true);
        position.reset_to(&state);
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let mut heuristics = crate::heuristics::HeuristicTables::new();
        let mut run = Run::new(
            &mut position,
            &mut table,
            CandidatePolicy::Staged(crate::params::StagedParams {
                quiet_radius: 2,
                safety_net_top_k: 0,
                tier_t_own_count: 2,
                tier_t_opponent_count: 3,
                q_depth_turns: 0,
                q_triggers: crate::params::QTriggers::DefensiveOnly,
                ordering: crate::params::OrderingHeuristics {
                    killers: true,
                    history: true,
                    countermove: true,
                },
            }),
            Stop::DepthTurns(1),
            12,
            &mut heuristics,
        );
        // The null window at the bottom: any finite child score fails high
        // against `beta`, so the first forced candidate cuts the node off.
        let score = run.visit(2, -INFINITY, -INFINITY + 1, 0);
        assert!(
            score > -INFINITY,
            "the null window must fail high for the cutoff to have happened at all"
        );
        // `killers[0]` is written only by a recording at ply 0 — the root's
        // own cutoff. Deeper plies write their own slots (and a phase-Second
        // cutoff at ply 1 legitimately writes `pair_killers[0]`, which is
        // why the assertion stops at the killer slots).
        assert!(
            heuristics.killers[0] == [None, None],
            "a forced-prefix cutoff is a tactical cell the tiers already front-load; \
             recording it would let Tier F masquerade as a quiet-refutation hint"
        );
    }

    /// WP-1.7's liveness pin: a staged search with the gates ON actually
    /// reaches `HeuristicTables::record_cutoff` from `visit` — the one thing
    /// no unit test of the tables themselves can prove. A beta cutoff at an
    /// unforced candidate in a two-turn search must leave a killer, a
    /// history entry and a countermove behind; a search that never cut off
    /// would leave all three empty and this test would fail.
    ///
    /// Mutation checked: deleting the `record_cutoff` call in `visit`'s
    /// post-loop block makes this test red.
    #[test]
    fn a_staged_search_with_the_gates_on_records_its_cutoffs() {
        // The search tests' own `quiet()` shape: eleven stones, nothing
        // decided, a BATCHED row's delta-ranked unforced range at every
        // node — exactly where cutoffs feed the tables.
        let state = GameState::from_plies(&[
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
        .expect("the quiet fixture is a legal game");
        let mut position = Position::new(Box::new(Flat), true);
        position.reset_to(&state);
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let mut heuristics = crate::heuristics::HeuristicTables::new();
        let mut run = Run::new(
            &mut position,
            &mut table,
            CandidatePolicy::Staged(crate::params::StagedParams {
                quiet_radius: 2,
                safety_net_top_k: 0,
                tier_t_own_count: 2,
                tier_t_opponent_count: 3,
                q_depth_turns: 0,
                q_triggers: crate::params::QTriggers::DefensiveOnly,
                ordering: crate::params::OrderingHeuristics {
                    killers: true,
                    history: true,
                    countermove: true,
                },
            }),
            Stop::DepthTurns(3),
            12,
            &mut heuristics,
        );
        run.iterate(6, false)
            .expect("a depth budget over a quiet position completes");

        let any_killer = heuristics
            .killers
            .iter()
            .any(|slots| slots[0].is_some() || slots[1].is_some());
        assert!(
            any_killer || !heuristics.history.is_empty(),
            "a two-turn search over a quiet position cuts off somewhere, and \
             every unforced cutoff lands in the tables"
        );
        assert!(
            !heuristics.countermove.is_empty(),
            "every cutoff whose node follows an opponent stone writes a countermove"
        );
    }

    /// The root-restriction mechanism (design wp18b §2 D3), pinned directly: a
    /// `root_restrict` of one far, legal, eval-terrible cell makes the answered
    /// pair's promotion that cell; with the restriction's application disabled
    /// (the registered zone-leak mutation), the answer is the unrestricted best
    /// and this dies. The anchor positions cannot pin this — there the eval-best
    /// block already sits inside the proof's zone, so answers coincide with and
    /// without the mechanism (MEASURED at g001-t45: OFF's answer `-5,-1/2,4`
    /// intersects the zone {-6,0 -5,-1 0,-6 1,-7 4,-5 4,-3} either way).
    #[test]
    fn a_root_restrict_zone_forces_the_answered_promotion_into_it() {
        let state = root();
        // Staged: the restriction is the staged policy's own (the wiring is
        // staged-only, design wp18b §2 D1).
        let mut position = Position::new(Box::new(Flat), true);
        position.reset_to(&state);
        let mut table = Table::new(1 << 16).expect("test table");
        let mut heuristics = crate::heuristics::HeuristicTables::new();
        // A cell inside the quiet ball (radius 2), legal, and never the flat
        // eval's lexicographic pick on its own: the restriction is the only
        // reason it answers. A cell OUTSIDE the ball would empty the filtered
        // set and fail open — the D3 clause working, tested elsewhere by the
        // fail-open shape itself.
        let far = Coord::new(2, 0);
        assert!(state.board().is_legal_placement(far) && !state.board().is_occupied(far));
        let staged = crate::StagedParams {
            quiet_radius: 2,
            safety_net_top_k: 0,
            tier_t_own_count: 2,
            tier_t_opponent_count: 3,
            q_depth_turns: 0,
            q_triggers: crate::QTriggers::DefensiveOnly,
            ordering: crate::OrderingHeuristics {
                killers: false,
                history: false,
                countermove: false,
            },
        };
        let mut run = Run::new(
            &mut position,
            &mut table,
            CandidatePolicy::Staged(staged),
            Stop::DepthTurns(1),
            MAX_PLY_FOR_RESTRICTION_TEST,
            &mut heuristics,
        );
        run.root_restrict = Some(vec![far]);
        let _ = run.iterate(2, false);
        let pv = turns_from_plies(&state, run.line());
        let first = pv.first().expect("the restricted root answers a move");
        let [a, b] = match first {
            pistol_core::Turn::Pair(a, b) => [*a, *b],
            pistol_core::Turn::Single(at) => [*at, *at],
        };
        assert!(
            a == far || b == far,
            "the answered pair's promotion is the restricted cell: {first:?}"
        );
    }

    const MAX_PLY_FOR_RESTRICTION_TEST: usize = 130;
}
