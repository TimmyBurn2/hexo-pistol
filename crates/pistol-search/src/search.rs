use std::time::Instant;

use pistol_core::{Coord, GameState, Outcome, Phase, Turn, stones_in_turn};
use pistol_eval::Eval;

use crate::candidates::candidate_cells;
use crate::error::SearchError;
use crate::fallback::{FallbackAnswer, fallback_turn};
use crate::info::{Provenance, SearchInfo, SearchOutcome};
use crate::params::{CandidatePolicy, SearchParams};
use crate::position::Position;
use crate::pv::turns_from_plies;
use crate::pvs::Run;
use crate::score::{MAX_MATE_TURNS, is_mate, mate_in};
use crate::stop::Stop;
use crate::tt::Table;

/// The deepest this build searches, in turns.
///
/// It bounds the recursion, the packed depth field of a table entry, and the
/// principal variation table. It is not a budget: a budget is always stated by
/// the caller (CLAUDE.md rule 1), and a depth budget past this is refused by
/// name rather than clamped.
pub const MAX_DEPTH_TURNS: u32 = 64;

/// The most whole turns one quiescence chain can add past the horizon
/// (`crate::quiescence`, WP-1.6): `2 * q_depth_turns` plies at the deepest
/// point of the deepest iteration. Kept generously above
/// `pistol_engine::config::MAX_Q_DEPTH_TURNS`'s own ceiling (8 turns) rather
/// than linked to it — `pistol-search` does not depend on `pistol-engine`
/// (the crate map's composition direction is the other way), so this is an
/// independent, deliberately loose bound `[MAX_PLY]` sizes against; a config
/// ceiling raised past what this covers is a `pistol-search` change too, not
/// a config-only one.
const MAX_Q_EXTENSION_PLIES: usize = 32;

/// Plies the recursion — main search plus the deepest possible quiescence
/// chain past its horizon — can reach: two per turn, room for the root, and
/// room for one quiescence chain at the deepest iteration
/// ([`MAX_Q_EXTENSION_PLIES`]).
pub(crate) const MAX_PLY: usize = 2 * MAX_DEPTH_TURNS as usize + 2 + MAX_Q_EXTENSION_PLIES;

const _: () = assert!(
    2 * MAX_DEPTH_TURNS < MAX_MATE_TURNS,
    "every mate this build can find must fit in the mate band"
);

/// Named invariant: a completed iteration that produced no move.
pub const NO_MOVE_FROM_A_COMPLETED_ITERATION: &str = "NO_MOVE_FROM_A_COMPLETED_ITERATION";

/// A search, and everything it keeps between calls.
///
/// The table is kept: successive searches in one game share what they learned,
/// which is the point of having one. [`Searcher::clear`] is what a new game
/// does, and a determinism gate compares two searches that both started from a
/// cleared table (docs/decisions.md D-7).
pub struct Searcher {
    params: SearchParams,
    table: Table,
    /// The solver on the search path (design wp18b §2): constructed IFF the
    /// gate is on and the policy is Staged; `None` otherwise, which is what
    /// makes a gate-off search byte-identical to the pre-wiring search by
    /// construction — no solver, no calls, no counters, no fields printed.
    solver: Option<pistol_solver::Solver>,
    position: Position,
    /// WP-1.7's ordering-heuristic tables — they persist across the searches
    /// of one game like the transposition table does, are begun afresh by
    /// every [`Searcher::search`], and are cleared by [`Searcher::clear`],
    /// which is what a new game is (`docs/experiments/wp17_design.md` §3.1).
    heuristics: crate::heuristics::HeuristicTables,
}

impl Searcher {
    /// Build a search from parameters and an evaluation backend.
    ///
    /// Refuses parameters it cannot honour, by name. The engine validates its
    /// config before it gets here, but a [`SearchParams`] can be built by
    /// anyone, and a search that quietly repaired one would be the silent
    /// fallback CLAUDE.md rule 3 forbids.
    pub fn new(params: SearchParams, eval: Box<dyn Eval>) -> Result<Searcher, SearchError> {
        let (key, radius) = match params.candidate_policy {
            CandidatePolicy::Radius { radius } => ("search.candidate_policy.radius", radius),
            CandidatePolicy::Staged(staged) => {
                ("search.candidate_policy.quiet_radius", staged.quiet_radius)
            }
        };
        if radius == 0 {
            return Err(SearchError::params(
                key,
                "must be at least 1: a radius of 0 reaches only occupied cells",
            ));
        }
        // The other end, and it is this crate's to refuse rather than the
        // engine's: the engine's ceiling binds documents, and a `SearchParams`
        // built in code never passes through it. A radius no `Coord` can step is
        // not a wide search, it is a radius the geometry cannot express — and
        // the generator used to answer it by quietly substituting the largest
        // one it could, which is the silent repair rule 3 forbids. This bound is
        // about what a coordinate can hold and is never compared with the rules'
        // legal region (docs/decisions.md D-20, D-77).
        if i16::try_from(radius).is_err() {
            return Err(SearchError::params(
                key,
                format!(
                    "must be at most {}: a ball wider than a coordinate can step is not a ball, \
                     got {radius}",
                    i16::MAX
                ),
            ));
        }
        if let CandidatePolicy::Staged(staged) = params.candidate_policy {
            for (key, count) in [
                (
                    "search.candidate_policy.tier_t_own_count",
                    staged.tier_t_own_count,
                ),
                (
                    "search.candidate_policy.tier_t_opponent_count",
                    staged.tier_t_opponent_count,
                ),
            ] {
                if !(2..=3).contains(&count) {
                    return Err(SearchError::params(
                        key,
                        format!(
                            "must be 2 or 3 — LAW-SUPPORT's threshold reading admits no other \
                             count (U3_tier_t.md §6.1), got {count}"
                        ),
                    ));
                }
            }
            // This crate's own ceiling to refuse, same reason as `radius`
            // above: `pistol-engine`'s validator binds documents, and a
            // `SearchParams` built in code never passes through it. A
            // `q_depth_turns` past what `MAX_PLY` was sized for is not a
            // deeper quiescence extension, it is a chain `PvTable` cannot
            // index into (docs/wp16_quiescence_design.md §6).
            let max_q_depth_turns = MAX_Q_EXTENSION_PLIES as u32 / 2;
            if staged.q_depth_turns > max_q_depth_turns {
                return Err(SearchError::params(
                    "search.candidate_policy.q_depth_turns",
                    format!(
                        "must be at most {max_q_depth_turns}: a chain deeper than that runs past \
                         the principal-variation table's fixed size, got {}",
                        staged.q_depth_turns
                    ),
                ));
            }
        }
        let tracks_threats = matches!(params.candidate_policy, CandidatePolicy::Staged(_));
        // The wiring's own refusal (design wp18b §2 D1, the engine refuses
        // first; this is the belt for a `SearchParams` built in code): the
        // trigger reads a `ThreatState` only the Staged policy maintains,
        // and a gate that silently did nothing would be rule 3's sin.
        let solver = match params.solver {
            Some(_wiring) if !tracks_threats => {
                return Err(SearchError::params(
                    "solver.on_search_path",
                    "is true under a Radius-kind candidate_policy: the trigger reads the staged \
                     policy's threat state, and a silent no-op is refused — set the policy staged \
                     or the gate false",
                ));
            }
            Some(wiring) => {
                let inner = wiring.inner;
                Some(pistol_solver::Solver::new(
                    inner.epsilon,
                    inner.tt_entries,
                    inner.attacker_policy,
                ))
            }
            None => None,
        };
        Ok(Searcher {
            params,
            table: Table::new(params.tt_bytes)?,
            solver,
            position: Position::new(eval, tracks_threats),
            heuristics: crate::heuristics::HeuristicTables::new(),
        })
    }

    /// The parameters this search was built with.
    pub fn params(&self) -> SearchParams {
        self.params
    }

    /// How many bytes the transposition table took.
    pub fn table_bytes(&self) -> u64 {
        self.table.bytes()
    }

    /// Forget everything learned so far. This is what a new game does.
    pub fn clear(&mut self) {
        self.table.clear();
        self.heuristics.clear();
        // Wholesale (design wp18b §1): epoch isolation already makes
        // earlier solves read as absent, so this is memory hygiene and
        // defence-in-depth, stated once and honestly there.
        if let Some(solver) = self.solver.as_mut() {
            solver.reset();
        }
    }

    /// Search `state` under `stop`, reporting once per completed depth.
    ///
    /// The report is a borrow, so a caller can print it, collect it, or ignore
    /// it without the search allocating on its behalf.
    pub fn search(
        &mut self,
        state: &GameState,
        stop: Stop,
        report: &mut dyn FnMut(&SearchInfo),
    ) -> Result<SearchOutcome, SearchError> {
        let max_depth = self.check_root(state, stop)?;

        self.position.reset_to(state);
        self.table.new_generation();
        // WP-1.7: the heuristic tables' per-search lifecycle — killers and
        // pair killers reset (ply indices restart here), history ages by
        // halving, countermove survives (`docs/experiments/wp17_design.md`
        // §3.1). Before the first iteration, like the position reset: a
        // heuristic that had already read the tables would be reading the
        // previous search's ply numbering.
        self.heuristics.begin_search();
        // Under a wall-clock stop the answer is secured BEFORE any deepening:
        // the fallback's cost is bounded and paid up front, so every iteration
        // — the first included — may then be interrupted (D-74 as amended by
        // WP-1.4; the old first-iteration rule stands for reproducible stops).
        // The root's static value rides along for the fallback's report.
        let started = Instant::now();
        let fallback = match stop {
            Stop::Deadline(_) => Some((
                fallback_turn(state, self.params.candidate_policy),
                self.position.value(),
            )),
            Stop::DepthTurns(_) | Stop::Nodes(_) => None,
        };
        // THE ROOT'S OWN CALLS (design wp18b §2 D3), before any deepening
        // and only at a two-stone turn boundary (turn 1 owes one stone and
        // is never a legal solver position). An attacker proof ANSWERS the
        // search with the proof's own first move; a defender proof restricts
        // the root's candidates to the proof's Z2 zone. Ply 0 in `Run` never
        // calls again — these are the root's calls, and double-paying them
        // would double-count the same nodes against the budget.
        let wiring_copy = self.params.solver;
        let mut root_restrict: Option<Vec<Coord>> = None;
        let mut root_solver_nodes = 0u64;
        let mut root_refusals = 0u32;
        if let Some(wiring) = self.params.solver
            && self.solver.is_some()
            && state.stones_owed() == 2
            && state.phase() == Phase::First
            && root_triggers(&mut self.position, wiring.trigger)
        {
            // THE TRIGGER GATES THE ROOT TOO (design §2 D1; the S-4 note
            // was this bug in a cost costume): without it, a quiet root's
            // defender call is the ZERO-PLAN AND root the §2 red team
            // proved unreachable ONLY under the trigger — the
            // SOLVER_NO_PLAN panic the solver seat's own session
            // reproduced.
            let cap = wiring.per_call_node_cap;
            let Some(solver) = self.solver.as_mut() else {
                unreachable!("the wiring exists only when the solver does")
            };
            let attacker = solver.solve(state, cap);
            root_solver_nodes += attacker.nodes;
            if let pistol_solver::SolveOutcome::Win(tree) = attacker.outcome {
                return Ok(solver_proof_outcome(
                    state,
                    &tree,
                    root_solver_nodes,
                    root_refusals,
                    started,
                ));
            }
            if matches!(
                attacker.outcome,
                pistol_solver::SolveOutcome::NoWinUnderZone
            ) {
                root_refusals += 1;
            }
            let defender = solver.solve_defender(state, cap);
            root_solver_nodes += defender.nodes;
            if let pistol_solver::SolveOutcome::Win(tree) = defender.outcome {
                // The proof's Z2 zone, sorted for the binary search at
                // ply 0: the opponent's plan cells are where the forced
                // defense lives, and the restriction fails open there.
                let zone = proof_root_zone(&tree);
                if !zone.is_empty() {
                    root_restrict = Some(zone);
                }
            } else if matches!(
                defender.outcome,
                pistol_solver::SolveOutcome::NoWinUnderZone
            ) {
                root_refusals += 1;
            }
        }
        let mut run = Run::new(
            &mut self.position,
            &mut self.table,
            self.params.candidate_policy,
            stop,
            MAX_PLY,
            &mut self.heuristics,
        );
        run.solver = self.solver.as_mut().map(|solver| {
            (
                solver,
                wiring_copy.expect("wiring exists when the solver does"),
            )
        });
        run.root_restrict = root_restrict;
        run.solver_nodes = root_solver_nodes;
        run.solver_refusals = root_refusals;

        let mut outcome = None;
        for depth_turns in 1..=max_depth {
            let depth_plies = plies_for(state.turn(), depth_turns);
            // Every iteration is abortable once a fallback answer is secured;
            // under a reproducible stop the first one still is not (D-74).
            let abortable = depth_turns > 1 || fallback.is_some();
            let Some(score) = run.iterate(depth_plies, abortable) else {
                break;
            };

            let pv = turns_from_plies(state, run.line());
            let best = *pv.first().unwrap_or_else(|| {
                panic!(
                    "pistol-search invariant {NO_MOVE_FROM_A_COMPLETED_ITERATION}: depth \
                     {depth_turns} finished with an empty principal variation"
                )
            });
            let elapsed = started.elapsed();
            let info = SearchInfo {
                depth_turns,
                seldepth_turns: run.seldepth_turns,
                nodes: run.total_nodes(),
                search_nodes: run.search_nodes,
                solver_nodes: run.solver_nodes,
                solver_refusals: run.solver_refusals,
                nps: per_second(run.total_nodes(), elapsed),
                time_ms: elapsed.as_millis() as u64,
                pv,
                score,
                hashfull_permille: run.hashfull_permille(),
                stages: run.stages,
            };
            report(&info);
            outcome = Some(SearchOutcome {
                best,
                info,
                provenance: Provenance::CompletedDepth,
            });

            if is_mate(score) {
                break;
            }
        }

        // What the answer is, in order of information: the aborted iteration's
        // completed root prefix where one exists (it starts from the table's
        // move, so it is never worse-informed than the last completed depth),
        // then the last completed depth, then — only under a wall-clock stop —
        // the fallback secured before deepening. The salvage is read for a
        // Deadline stop ONLY: under a node budget it would change answers that
        // are pinned byte-for-byte (CLAUDE.md rule 4, the golden transcripts).
        let completed_depth = outcome.as_ref().map_or(0, |done| done.info.depth_turns);
        let salvage = match stop {
            Stop::Deadline(_) => run
                .salvage()
                .map(|(score, line)| (score, turns_from_plies(state, line))),
            Stop::DepthTurns(_) | Stop::Nodes(_) => None,
        };
        let mut outcome = if let Some((score, pv)) = salvage {
            let best = *pv.first().unwrap_or_else(|| {
                panic!(
                    "pistol-search invariant {NO_MOVE_FROM_A_COMPLETED_ITERATION}: a salvaged \
                     root line is a promotion's line and cannot be empty"
                )
            });
            SearchOutcome {
                best,
                info: SearchInfo {
                    depth_turns: completed_depth,
                    seldepth_turns: 0,
                    nodes: 0,
                    search_nodes: 0,
                    solver_nodes: 0,
                    solver_refusals: 0,
                    nps: 0,
                    time_ms: 0,
                    pv,
                    score,
                    hashfull_permille: 0,
                    stages: crate::info::StageCounters::default(),
                },
                provenance: Provenance::PartialRoot,
            }
        } else if let Some(done) = outcome {
            done
        } else {
            let (answer, root_value) = fallback.unwrap_or_else(|| {
                panic!(
                    "pistol-search invariant {NO_MOVE_FROM_A_COMPLETED_ITERATION}: under a \
                     reproducible stop the first iteration cannot be interrupted, so one of \
                     them completed"
                )
            });
            // An instant win the fallback proved is a mate in one turn (D-3:
            // either stone of the turn completes on the same turn count);
            // anything else is unsearched and carries the root's static value.
            let score = match answer {
                FallbackAnswer::WinsThisTurn(_) => mate_in(1),
                FallbackAnswer::Ordinary(_) => root_value,
            };
            SearchOutcome {
                best: answer.turn(),
                info: SearchInfo {
                    depth_turns: 0,
                    seldepth_turns: 0,
                    nodes: 0,
                    search_nodes: 0,
                    solver_nodes: 0,
                    solver_refusals: 0,
                    nps: 0,
                    time_ms: 0,
                    pv: vec![answer.turn()],
                    score,
                    hashfull_permille: 0,
                    stages: crate::info::StageCounters::default(),
                },
                provenance: Provenance::Fallback,
            }
        };
        // What the last completed depth found, and what the whole search cost —
        // an interrupted iteration is discarded as an answer but not as work,
        // and per-side compute accounting is a reporting requirement
        // (CLAUDE.md rule 6). `stages` is a whole-search total exactly like
        // `nodes`, written on every path for the same reason (docs/decisions.md
        // U2-M item 2: "a counter that silently reads zero on a wall-clock path
        // would make the play-mode stage shares unreadable").
        let elapsed = started.elapsed();
        outcome.info.nodes = run.total_nodes();
        outcome.info.search_nodes = run.search_nodes;
        // REVIEW-impl W-1: the salvage/fallback arms construct these as
        // zero, and without this overwrite a Deadline answer would break
        // the registered sum law the moment a solver call had spent
        // anything before the abort.
        outcome.info.solver_nodes = run.solver_nodes;
        outcome.info.solver_refusals = run.solver_refusals;
        outcome.info.nps = per_second(run.total_nodes(), elapsed);
        outcome.info.time_ms = elapsed.as_millis() as u64;
        outcome.info.seldepth_turns = run.seldepth_turns;
        outcome.info.hashfull_permille = run.hashfull_permille();
        outcome.info.stages = run.stages;
        Ok(outcome)
    }

    /// Everything that has to hold before a search starts, and the depth it will
    /// run to.
    fn check_root(&self, state: &GameState, stop: Stop) -> Result<u32, SearchError> {
        if let Outcome::Win { winner, turn } = state.outcome() {
            return Err(SearchError::GameDecided { winner, turn });
        }
        // The search counts, deepens and reports in turns, so it starts where a
        // turn does. A half-played turn is finished by the rules' own ply-level
        // entry point, not by a search (docs/decisions.md D-50, D-71).
        if state.phase() != Phase::First {
            return Err(SearchError::TurnInProgress { turn: state.turn() });
        }
        if candidate_cells(state.board(), self.params.candidate_policy).is_empty() {
            return Err(SearchError::NoCandidates {
                turn: state.turn(),
                policy: self.params.candidate_policy,
            });
        }
        match stop {
            Stop::DepthTurns(turns) if turns == 0 || turns > MAX_DEPTH_TURNS => {
                Err(SearchError::DepthOutOfRange {
                    turns,
                    max: MAX_DEPTH_TURNS,
                })
            }
            Stop::DepthTurns(turns) => Ok(turns),
            Stop::Nodes(_) | Stop::Deadline(_) => Ok(MAX_DEPTH_TURNS),
        }
    }
}

/// How many plies `depth_turns` turns are, starting from `root_turn`.
///
/// Not twice the depth: the first turn of a game owes one stone (rule 3), so a
/// search from the opening position is one ply shallower than the same depth
/// from anywhere else.
fn plies_for(root_turn: u32, depth_turns: u32) -> u32 {
    (0..depth_turns)
        .map(|ahead| stones_in_turn(root_turn.saturating_add(ahead)))
        .sum()
}

/// Nodes per second, or zero if no time has passed yet.
fn per_second(nodes: u64, elapsed: std::time::Duration) -> u64 {
    let nanos = elapsed.as_nanos();
    if nanos == 0 {
        return 0;
    }
    u64::try_from(u128::from(nodes) * 1_000_000_000 / nanos).unwrap_or(u64::MAX)
}

/// Whether the root position fires the solver trigger (design §2 D1):
/// any hot window — an open four or better — held by either side, read off
/// the staged policy's own threat state.
fn root_triggers(position: &mut Position, trigger: crate::params::SolverTrigger) -> bool {
    let (state, threats, _) = position.staged_context();
    match trigger {
        crate::params::SolverTrigger::AnyOpenFour => {
            let mover = state.to_move();
            !threats.hot_windows(mover).is_empty()
                || !threats.hot_windows(mover.opponent()).is_empty()
        }
    }
}

/// The proof's first move: the root `OrStep`'s witness turn, or the
/// `OrWinLeaf`'s completing stones as a turn (design wp18b §2 D3). Keyed by
/// `tree.root` — the emission is POST-ORDER, so the root is never
/// `nodes.first()`.
///
/// A ONE-PLY completing witness (a mover live five — the shape the
/// determinism seat's own fixture is full of, REVIEW-impl C-1's
/// characterization) is a single cell, and a turn at `stones_owed == 2` is
/// a PAIR: the completing stone plus the lexicographically-least other
/// legal placement. Rule 4 ends the turn on the completing stone, so the
/// partner changes nothing about the proof — it only makes the answer a
/// legal turn. REFUSES (None) if the board offers no partner, which a
/// real position cannot do.
pub fn proof_first_move(tree: &pistol_solver::ProofTree, state: &GameState) -> Option<Turn> {
    let node = tree.nodes.iter().find(|node| node.key == tree.root)?;
    match &node.kind {
        pistol_solver::ProofKind::OrStep { witness } => Some(*witness),
        pistol_solver::ProofKind::OrWinLeaf { witness } => match witness {
            pistol_solver::WinWitness::Pair { first, second, .. } => {
                Turn::pair(*first, *second).ok()
            }
            pistol_solver::WinWitness::OnePly { at, .. } => {
                let partner = pistol_core::legal_placements(state.board())
                    .into_iter()
                    .find(|cell| cell != at)?;
                Turn::pair(*at, partner).ok()
            }
        },
        // An AND-rooted proof (the defender direction) has no single first
        // move of the mover's — the search never asks for one there.
        pistol_solver::ProofKind::AndStep | pistol_solver::ProofKind::AndOverloadLeaf => None,
    }
}

/// The Z2 zone cells of the proof's root node (design wp18b §2 D3): the
/// committed restriction order, `order(1)`.
pub fn proof_root_zone(tree: &pistol_solver::ProofTree) -> Vec<Coord> {
    let node = tree
        .nodes
        .iter()
        .find(|node| node.key == tree.root)
        .expect("the emitted tree carries its root");
    let mut cells: Vec<Coord> = node.zone.order(1).iter().copied().collect();
    cells.sort_unstable();
    cells.dedup();
    cells
}

/// The witness line as a principal variation, for a solver-proved root:
/// the attacker's own turns following `OrStep` witnesses, the defender's
/// first listed reply between them — a line the PROOF certifies, not one
/// the search ordered.
pub fn proof_line(
    tree: &pistol_solver::ProofTree,
    state: &GameState,
    max_turns: usize,
) -> Vec<Turn> {
    use std::collections::BTreeMap;
    let by_key: BTreeMap<pistol_core::Key128, &pistol_solver::EmittedNode> =
        tree.nodes.iter().map(|node| (node.key, node)).collect();
    let mut line = Vec::new();
    let mut cursor = tree.root;
    while line.len() < max_turns {
        let Some(node) = by_key.get(&cursor) else {
            break;
        };
        match &node.kind {
            pistol_solver::ProofKind::OrWinLeaf { witness } => {
                match witness {
                    pistol_solver::WinWitness::Pair { first, second, .. } => {
                        line.push(
                            Turn::pair(*first, *second)
                                .expect("a pair witness is two distinct cells"),
                        );
                    }
                    pistol_solver::WinWitness::OnePly { at, .. } => {
                        // Same degeneration as `proof_first_move`: the
                        // completing stone paired with the least legal
                        // partner, so the PV line ends with a legal TURN
                        // (REVIEW-impl's catch that this arm silently
                        // dropped the completing turn).
                        let partner = pistol_core::legal_placements(state.board())
                            .into_iter()
                            .find(|cell| cell != at)
                            .expect("a live five's board has a partner cell");
                        line.push(
                            Turn::pair(*at, partner)
                                .expect("the completing cell and a partner differ"),
                        );
                    }
                }
                break;
            }
            pistol_solver::ProofKind::OrStep { witness } => {
                line.push(*witness);
                let Some((_, child)) = node.children.first() else {
                    break;
                };
                cursor = *child;
            }
            pistol_solver::ProofKind::AndStep => {
                let Some((turn, child)) = node.children.first() else {
                    break;
                };
                line.push(*turn);
                cursor = *child;
            }
            pistol_solver::ProofKind::AndOverloadLeaf => break,
        }
    }
    line
}

/// The outcome a root attacker proof answers with (design wp18b §2 D3):
/// the proof's first move, its own witness line as the PV, the mate score
/// at the proof's distance, and the solver's nodes as the whole cost.
fn solver_proof_outcome(
    state: &GameState,
    tree: &pistol_solver::ProofTree,
    solver_nodes: u64,
    refusals: u32,
    started: Instant,
) -> SearchOutcome {
    let depth = tree.win_depth_turns();
    let best = proof_first_move(tree, state)
        .unwrap_or_else(|| panic!("pistol-search invariant SOLVER_PROOF WITHOUT A MOVE: an OR-rooted proof always carries one"));
    let pv = proof_line(tree, state, 2 * depth as usize + 1);
    let elapsed = started.elapsed();
    SearchOutcome {
        best,
        info: SearchInfo {
            depth_turns: depth,
            seldepth_turns: depth,
            nodes: solver_nodes,
            search_nodes: 0,
            solver_nodes,
            solver_refusals: refusals,
            nps: per_second(solver_nodes, elapsed),
            time_ms: elapsed.as_millis() as u64,
            pv,
            score: crate::score::mate_in(2 * depth - 1),
            hashfull_permille: 0,
            stages: crate::info::StageCounters::default(),
        },
        provenance: Provenance::SolverProof,
    }
}
