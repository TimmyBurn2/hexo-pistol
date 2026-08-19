//! Principal variation search, one ply at a time.
//!
//! A turn is two stones by the same side, so the recursion alternates between
//! two kinds of child: the **second stone of this turn**, which is the same
//! side moving again and inherits the window unchanged, and the **first stone
//! of the opponent's turn**, which is the ordinary negamax flip. The alpha-beta
//! cutoff between the two stones of a turn is where most of the pruning in this
//! game comes from (the research report, Section A), and it falls out of the
//! ply-level recursion rather than being arranged for (docs/decisions.md D-9).
//!
//! Rule 4 also falls out: a stone that completes a line ends the turn, so the
//! node returns a mate score without the second stone ever being placed.
//!
//! # Stage 0 scope
//!
//! Null-window scans with a re-search on a fail-high, a transposition table, and
//! move ordering. No quiescence, no null move, no reductions, no aspiration
//! window: the research report parks each of those behind an SPRT the arena
//! cannot run yet (docs/ROADMAP.md, Stage 1 and Stage 4).
//!
//! # RULE9-JUSTIFICATION: `visit` is one recursion and splitting it would not
//! reduce what a reader has to hold (CLAUDE.md rule 9).
//!
//! Its parts are not independent: the window a child inherits depends
//! on whether the ply was the mover's second stone or the opponent's reply, the
//! table's bound depends on the alpha it started with, the abort path has to
//! unwind through every one of them without a result being used, and the
//! invariants that make it correct — the mate re-basing, the no-cutoff-at-a-PV
//! -node rule, the two horizons — are stated against the whole. Cutting it into
//! free functions would pass six values back and forth and put the reasoning
//! that justifies them in a different file from the code they constrain. The
//! honest reductions are elsewhere and are scheduled: the doc comments here
//! carry the arguments, and Stage 1 moves candidate generation out entirely
//! (docs/decisions.md D-117, WP-1.5).

use std::time::Instant;

use pistol_core::{Coord, Phase, PlyOutcome};

use crate::candidates::candidate_cells;
use crate::ordering::{OrderOutcome, order};
use crate::params::CandidatePolicy;
use crate::position::Position;
use crate::pv::PvTable;
use crate::score::{INFINITY, mate_in};
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
    position: &'a mut Position,
    table: &'a mut Table,
    policy: CandidatePolicy,
    stop: Stop,
    root_turn: u32,
    /// Nodes visited, leaves included. Incremented once per [`Run::visit`].
    pub nodes: u64,
    /// The deepest turn count any line reached.
    pub seldepth_turns: u32,
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
    pv: PvTable,
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
    ) -> Run<'a> {
        let root_turn = position.state().turn();
        Run {
            position,
            table,
            policy,
            stop,
            root_turn,
            nodes: 0,
            seldepth_turns: 0,
            aborted: false,
            abortable: false,
            root_score: None,
            pv: PvTable::new(max_ply),
        }
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
        self.nodes += 1;
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
            return self.position.value();
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

        let mut cells = candidate_cells(self.position.board(), self.policy);
        if cells.is_empty() {
            // Unreachable for any radius of at least one, by the argument
            // `pistol_core::turn` gives for there being no stalemate — and kept
            // because that is a claim about today's policies rather than about
            // every policy (docs/decisions.md D-104).
            //
            // At a turn boundary a static value is the honest answer: the rules
            // still admit a move, the policy is what excluded it, and the line
            // reported so far ends on a whole turn. Half way through a turn it
            // is not an answer at all — the parent would promote a line ending
            // on a lone stone, and `turns_from_plies` refuses that by name at
            // the root, far from the node that caused it. A policy that runs dry
            // mid-turn has to say what the mover's second stone is; there is no
            // value this node can return that makes that question go away.
            assert!(
                self.position.state().phase() == Phase::First,
                "pistol-search invariant {NO_CANDIDATES_MID_TURN}: the candidate policy offered \
                 nothing at phase 1, where the mover still owes a stone — a policy that can run \
                 dry must answer inside a turn, not only at its boundary"
            );
            return self.position.value();
        }
        // A deadline can land inside the scoring loop — its length is the
        // candidate count, which the opponent partly grows (D-95) — so under a
        // wall-clock stop the ordering itself checks the clock and the node
        // aborts like any other; the partially scored order is discarded with
        // it. Reproducible stops pass `None` and read no clock (rule 4).
        let table_move = known.map(|record| record.best);
        if order(self.position, &mut cells, table_move, self.order_deadline())
            == OrderOutcome::DeadlinePassed
        {
            self.aborted = true;
            return 0;
        }

        let original_alpha = alpha;
        let mut best_score = -INFINITY;
        let mut best_cell = cells[0];

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
            self.table.store(
                key,
                from_root,
                Record {
                    depth_plies,
                    score: best_score,
                    static_eval: self.position.value(),
                    bound: if best_score <= original_alpha {
                        Bound::Upper
                    } else if best_score >= beta {
                        Bound::Lower
                    } else {
                        Bound::Exact
                    },
                    best: best_cell,
                },
            );
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
    fn turns_from_root(&self) -> u32 {
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
    fn should_stop(&mut self) -> bool {
        if self.aborted {
            return true;
        }
        if !self.abortable {
            return false;
        }
        let check_now = match self.stop {
            Stop::Deadline(_) => true,
            Stop::DepthTurns(_) | Stop::Nodes(_) => self.nodes.is_multiple_of(NODE_CHECK_INTERVAL),
        };
        if !check_now {
            return false;
        }
        self.aborted = self.stop.is_spent(self.nodes);
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
        let mut position = Position::new(Box::new(Flat));
        position.reset_to(&state);
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let policy = CandidatePolicy::Radius { radius: 1 };
        let mut run = Run::new(
            &mut position,
            &mut table,
            policy,
            Stop::Deadline(expired()),
            12,
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
        let mut position = Position::new(Box::new(Flat));
        position.reset_to(&state);
        let mut table = Table::new(1 << 20).expect("the smallest table");
        let policy = CandidatePolicy::Radius { radius: 1 };
        let far = Instant::now() + Duration::from_secs(3600);
        let mut run = Run::new(&mut position, &mut table, policy, Stop::Deadline(far), 12);

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
}
