//! The searcher: what holds the table, the evaluation and the position between
//! calls, and the iterative deepening loop that drives one search.
//!
//! # Deepening by turns
//!
//! Each iteration searches one turn deeper than the last, and the depth is
//! converted to plies here — the first turn of a game owes one stone and every
//! other turn owes two (rule 3), so the ply count is a sum over the turns ahead
//! rather than twice the depth. Every iteration therefore ends on a turn
//! boundary, which is what lets the principal variation be reported as turns.
//!
//! An iteration that a REPRODUCIBLE budget interrupts is **discarded**: its
//! move ordering was only partly informed by the previous iteration, and half
//! of a depth is not a depth. Under those budgets the first iteration is not
//! interruptible, because a search that returned no move would be a silent
//! failure (CLAUDE.md rule 3, docs/decisions.md D-74); a node budget is
//! honoured from the second iteration on, and what the first one cost is
//! reported in `nodes` rather than hidden.
//!
//! A WALL-CLOCK budget cannot afford that rule — the first iteration's cost
//! grows with the candidate count, which the opponent partly controls (D-95) —
//! so before deepening the search secures a bounded fallback answer
//! ([`crate::fallback`]), makes every iteration abortable, and under an abort
//! answers with the best information it holds: the aborted iteration's
//! completed root prefix, else the last completed depth, else the fallback.
//! [`SearchOutcome::provenance`] says which, and `depth_turns` counts only
//! completed depths whichever way the answer came (WP-1.4, amending D-74).
//!
//! The loop also stops early on a proven mate: once a line is won or lost by
//! force, deeper search cannot improve on the distance the shallower one found.
//!
//! # RULE9-JUSTIFICATION: one deepening loop and the one decision about what
//! its answer is (CLAUDE.md rule 9).
//!
//! The wall-clock fallback staging, the salvage-else-completed-else-fallback
//! resolution, and the totals accounting all read and settle the same state the
//! loop produced, in one order that the abort-honesty argument is stated
//! against; splitting them would put the reasoning that justifies the answer in
//! a different file from the loop whose interruption it answers for.

use std::time::Instant;

use pistol_core::{GameState, Outcome, Phase, stones_in_turn};
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

/// Plies the recursion can reach: two per turn, and room for the root.
pub(crate) const MAX_PLY: usize = 2 * MAX_DEPTH_TURNS as usize + 2;

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
    position: Position,
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
        }
        let tracks_threats = matches!(params.candidate_policy, CandidatePolicy::Staged(_));
        Ok(Searcher {
            params,
            table: Table::new(params.tt_bytes)?,
            position: Position::new(eval, tracks_threats),
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
        let mut run = Run::new(
            &mut self.position,
            &mut self.table,
            self.params.candidate_policy,
            stop,
            MAX_PLY,
        );

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
                nodes: run.nodes,
                nps: per_second(run.nodes, elapsed),
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
        outcome.info.nodes = run.nodes;
        outcome.info.nps = per_second(run.nodes, elapsed);
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
