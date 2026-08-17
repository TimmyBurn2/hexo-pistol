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

use pistol_core::{Coord, PlyOutcome};

use crate::candidates::candidate_cells;
use crate::ordering::order;
use crate::params::CandidatePolicy;
use crate::position::Position;
use crate::pv::PvTable;
use crate::score::{INFINITY, mate_in};
use crate::stop::{NODE_CHECK_INTERVAL, Stop};
use crate::tt::{Bound, Record, Table};

/// Named invariant: the candidate policy offered a cell the rules refuse.
pub const CANDIDATE_ILLEGAL: &str = "CANDIDATE_ILLEGAL";

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
    /// Whether this iteration may be abandoned. The first one may not: a search
    /// that returned no move at all would be a silent failure
    /// (docs/decisions.md D-74).
    abortable: bool,
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
            pv: PvTable::new(max_ply),
        }
    }

    /// Search one iteration to `depth_plies`, returning its score, or `None` if
    /// the stop condition fired before it finished.
    pub fn iterate(&mut self, depth_plies: u32, abortable: bool) -> Option<i32> {
        self.abortable = abortable;
        let score = self.visit(depth_plies, -INFINITY, INFINITY, 0);
        (!self.aborted).then_some(score)
    }

    /// The line under the root, in plies.
    pub fn line(&self) -> &[Coord] {
        self.pv.line(0)
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
            // because that is a claim about today's policies. If a later policy
            // does run dry, this is a horizon rather than a rules statement: the
            // rules still admit a move, and the honest answer is what the
            // position is worth, not a cell the policy excluded. The root
            // refuses the same case by name, because there a move is owed
            // (`SearchError::NoCandidates`).
            return self.position.value();
        }
        order(self.position, &mut cells, known.map(|record| record.best));

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

    /// Whether the budget has run out, tested at a fixed node granularity so
    /// that the stopping point of a node budget is exact and reproducible.
    fn should_stop(&mut self) -> bool {
        if self.aborted {
            return true;
        }
        if !self.abortable || !self.nodes.is_multiple_of(NODE_CHECK_INTERVAL) {
            return false;
        }
        self.aborted = self.stop.is_spent(self.nodes);
        self.aborted
    }
}
