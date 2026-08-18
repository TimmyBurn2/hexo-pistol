//! The walk itself: one position, one evaluation kept in step with it, and the
//! recursion that values them.
//!
//! Split from [`super::reference`] for size discipline (CLAUDE.md rule 9), not
//! because it is a second concept: that module is the contract and its refusals,
//! and this is the tree walk behind it. Everything the walk needs to be correct
//! is stated against the walk, so the two read in either order.

use std::collections::BTreeMap;

use pistol_core::{Coord, GameState, Phase, Player, PlyOutcome, Turn};
use pistol_eval::{Eval, HandcraftedV0, Weights};
use pistol_search::{CandidatePolicy, candidate_cells};

use super::pair_dedupe::Paired;
use super::ref_score::RefScore;
use super::reference::{PairOrder, ReferenceRun};
use super::reference_invariants::{
    REFERENCE_CANDIDATE_ILLEGAL, REFERENCE_EVERY_PAIR_ALREADY_VALUED, REFERENCE_HORIZON_MID_TURN,
    REFERENCE_NO_CANDIDATES_MID_TURN, REFERENCE_PAIR_ORDER_DISAGREES,
    REFERENCE_TURN_OWES_A_THIRD_STONE,
};

/// One walk of the tree: the position it moves, the evaluation kept in step
/// with it, and the count of what it cost.
pub struct Walk {
    state: GameState,
    eval: Box<dyn Eval>,
    policy: CandidatePolicy,
    /// Whether both orderings of a pair are walked, or one.
    pairs: PairOrder,
    /// The turn every mate distance is measured from (docs/decisions.md D-72).
    root_turn: u32,
    nodes: u64,
}

impl Walk {
    /// A walk over `root`, with an evaluation holding exactly its stones.
    ///
    /// The evaluation is built from empty rather than by unwinding another
    /// position: the trait makes a value depend only on the SET of stones
    /// applied, so this is the number the search's own `reset_to` arrives at
    /// (docs/decisions.md D-61, D-62), without touching `pistol_search::position`.
    pub fn new(
        root: &GameState,
        policy: CandidatePolicy,
        weights: &Weights,
        pairs: PairOrder,
    ) -> Walk {
        let mut eval: Box<dyn Eval> = Box::new(HandcraftedV0::new(weights.clone()));
        for (at, player) in root.board().stones() {
            eval.apply(at, player);
        }
        Walk {
            state: root.clone(),
            eval,
            policy,
            pairs,
            root_turn: root.turn(),
            nodes: 0,
        }
    }

    /// A walk whose mate distances are measured from someone else's root:
    /// [`super::reference::reference_turn_value`] starts one turn inside the
    /// position it answers about, and D-72's distances are anchored at the root
    /// rather than at the node, so the anchor is passed in.
    pub fn anchored(
        root: &GameState,
        policy: CandidatePolicy,
        weights: &Weights,
        root_turn: u32,
        pairs: PairOrder,
    ) -> Walk {
        let mut walk = Walk::new(root, policy, weights, pairs);
        walk.root_turn = root_turn;
        walk
    }

    /// The root's turns and their values.
    ///
    /// The same loop [`Walk::negamax`] runs, keeping each turn instead of only
    /// the maximum. Kept separate rather than shared with a collecting callback
    /// because the interior runs tens of millions of times and must not
    /// allocate per node beyond the candidate set and the [`Paired`] ledger.
    pub fn root(&mut self, depth_turns: u32) -> ReferenceRun {
        self.nodes += 1;
        let mut values: BTreeMap<Turn, RefScore> = BTreeMap::new();
        let mut paired = Paired::new(self.pairs);
        for first in candidate_cells(self.state.board(), self.policy) {
            let (outcome, mover) = self.place(first);
            match outcome {
                // Rule 4 ends the turn on the stone that completes a line, so
                // this turn is one stone and the second is never played.
                PlyOutcome::Win { .. } => {
                    record(&mut values, Turn::Single(first), self.win_here());
                }
                // Turn 1 and nothing else: rule 3's one-stone opening turn.
                PlyOutcome::TurnComplete => {
                    let score = self.negamax(depth_turns - 1).negate();
                    record(&mut values, Turn::Single(first), score);
                }
                PlyOutcome::TurnContinues => {
                    for second in self.second_stone_cells() {
                        if paired.holds(second) {
                            continue;
                        }
                        let (outcome, mover) = self.place(second);
                        let score = self.after_second_stone(outcome, depth_turns);
                        self.undo(second, mover);
                        let turn = Turn::pair(first, second).unwrap_or_else(|error| {
                            panic!("the reference paired {first} and {second}: {error}")
                        });
                        record(&mut values, turn, score);
                    }
                    paired.push(first);
                }
            }
            self.undo(first, mover);
        }
        ReferenceRun {
            values,
            nodes: self.nodes,
        }
    }

    /// What this position is worth to the side to move, searched `turns_left`
    /// turns deeper. No window, no table, no ordering, no deepening.
    pub fn negamax(&mut self, turns_left: u32) -> RefScore {
        self.nodes += 1;
        if turns_left == 0 {
            return self.horizon();
        }
        let cells = candidate_cells(self.state.board(), self.policy);
        if cells.is_empty() {
            // The rules still admit a move and the policy is what excluded it,
            // so a static value is the honest answer — at a turn boundary. Half
            // way through a turn it is not an answer at all, which is what
            // `second_stone_cells` says (docs/decisions.md D-104).
            return self.horizon();
        }

        let mut best: Option<RefScore> = None;
        let mut paired = Paired::new(self.pairs);
        for first in cells {
            let (outcome, mover) = self.place(first);
            // `None` where every pair of this first stone was valued under
            // another ordering, so it adds nothing new to the maximum. Never
            // reached today — the last candidate in ascending order has always
            // opened at least one cell nothing else did — and kept because that
            // is a claim about today's radius-ball policy rather than about
            // every policy, which is the argument `pvs.rs` makes about its own
            // empty-candidate branch (docs/decisions.md D-104).
            let score = match outcome {
                PlyOutcome::Win { .. } => Some(self.win_here()),
                PlyOutcome::TurnComplete => Some(self.negamax(turns_left - 1).negate()),
                PlyOutcome::TurnContinues => {
                    let mut inner: Option<RefScore> = None;
                    for second in self.second_stone_cells() {
                        if paired.holds(second) {
                            continue;
                        }
                        let (outcome, mover) = self.place(second);
                        let score = self.after_second_stone(outcome, turns_left);
                        self.undo(second, mover);
                        inner = Some(inner.map_or(score, |held: RefScore| held.max(score)));
                    }
                    paired.push(first);
                    assert!(
                        inner.is_some() || self.pairs == PairOrder::Deduped,
                        "pistol-search reference invariant \
                         {REFERENCE_EVERY_PAIR_ALREADY_VALUED}: every pair at turn {} was \
                         skipped, and only the deduped mode skips anything",
                        self.state.turn()
                    );
                    inner
                }
            };
            self.undo(first, mover);
            if let Some(score) = score {
                best = Some(best.map_or(score, |held: RefScore| held.max(score)));
            }
        }
        best.expect("the first candidate is never skipped, so a value was folded in")
    }

    /// What the mover's SECOND stone is worth. The same side is still to move
    /// through it, so there is no negation here: only the stone that completes
    /// the turn hands the position to the opponent.
    fn after_second_stone(&mut self, outcome: PlyOutcome, turns_left: u32) -> RefScore {
        match outcome {
            PlyOutcome::Win { .. } => self.win_here(),
            PlyOutcome::TurnComplete => self.negamax(turns_left - 1).negate(),
            PlyOutcome::TurnContinues => panic!(
                "pistol-search reference invariant {REFERENCE_TURN_OWES_A_THIRD_STONE}: turn {} \
                 asked for a third stone",
                self.state.turn()
            ),
        }
    }

    /// The cells the mover's second stone may go on, on the board its first
    /// stone widened.
    fn second_stone_cells(&self) -> Vec<Coord> {
        let cells = candidate_cells(self.state.board(), self.policy);
        assert!(
            !cells.is_empty(),
            "pistol-search reference invariant {REFERENCE_NO_CANDIDATES_MID_TURN}: the policy \
             offered nothing at phase 1, where the mover still owes a stone"
        );
        cells
    }

    /// A horizon: the static value, and the statement that it is a turn
    /// boundary.
    fn horizon(&self) -> RefScore {
        assert!(
            self.state.phase() == Phase::First,
            "pistol-search reference invariant {REFERENCE_HORIZON_MID_TURN}: the horizon landed \
             at phase 1, where the mover still owes a stone"
        );
        RefScore::Value(self.eval.value(self.state.to_move()))
    }

    /// The distance of the win the stone just placed completed, in turns from
    /// the root. `place` does not advance the turn counter on a win — rule 4
    /// ends the turn there — so the turn it froze at is the one the win scores
    /// on (docs/decisions.md D-72).
    fn win_here(&self) -> RefScore {
        RefScore::WinInTurns(self.state.turn() - self.root_turn + 1)
    }

    /// Place a stone and tell the evaluation about it, in that order.
    fn place(&mut self, at: Coord) -> (PlyOutcome, Player) {
        let mover = self.state.to_move();
        let outcome = self.state.place(at).unwrap_or_else(|error| {
            panic!(
                "pistol-search reference invariant {REFERENCE_CANDIDATE_ILLEGAL}: the candidate \
                 policy offered {at}, and the rules say: {error}"
            )
        });
        self.eval.apply(at, mover);
        (outcome, mover)
    }

    /// Take one back.
    fn undo(&mut self, at: Coord, mover: Player) {
        let taken = self
            .state
            .undo()
            .unwrap_or_else(|error| panic!("the reference takes back {at}: {error}"));
        assert_eq!(taken, at, "the reference takes back only what it placed");
        self.eval.undo(at, mover);
    }
}

/// Record a turn's value, checking the assumption the engine makes about the two
/// orderings of a pair — which only [`PairOrder::BothOrderings`] reaches twice
/// (docs/decisions.md D-79).
fn record(values: &mut BTreeMap<Turn, RefScore>, turn: Turn, score: RefScore) {
    if let Some(&held) = values.get(&turn) {
        assert_eq!(
            held, score,
            "pistol-search reference invariant {REFERENCE_PAIR_ORDER_DISAGREES}: the two \
             orderings of {turn} reach the same position and must value it the same"
        );
        return;
    }
    values.insert(turn, score);
}
