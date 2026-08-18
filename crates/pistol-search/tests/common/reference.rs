//! The differential search oracle's reference: plain full-width negamax.
//!
//! This is to `Searcher` what the brute-force generator in pistol-core's test
//! tree is to `generate_turns` (CLAUDE.md rule 7, docs/decisions.md D-12), and
//! what the from-scratch eval is to the incremental one (D-68). It exists
//! because everything Stage 1 adds to the search is a PRUNING change, and rule
//! 6's SPRT judges strength rather than soundness: a pruning commit that quietly
//! changed the value of the tree would pass one (D-106).
//!
//! # What it shares, and what it must not
//!
//! Shared, deliberately, so that agreement means something:
//!
//! - pistol-core's rules — `place`/`undo`/`make_turn`, `PlyOutcome`, `Turn`. No
//!   crate but pistol-core states a rule (CLAUDE.md rule 2).
//! - pistol-eval's backend and the committed weight table.
//! - `pistol_search::candidate_cells`, the candidate-policy entry point, and
//!   nothing else of that crate's search. Re-implementing the policy here would
//!   pin nothing — two proximity balls agreeing says nothing about whether the
//!   ball is the right one — and would make every disagreement ambiguous between
//!   "the search is wrong" and "the two universes differ". What the policy's own
//!   shape rests on instead: `candidate_policy_tests`, which checks it against
//!   an independent bounding-box scan at four radii.
//!
//! Not shared: `pvs`, `tt`, `ordering`, `position`, `pv`, `search`, and — via
//! [`RefScore`](super::ref_score::RefScore) — the score packing in `score`. In
//! particular this does not use `search::plies_for`: it recurses in TURNS and
//! reads the turn structure off `PlyOutcome`, which is an independent statement
//! of the same invariant.
//!
//! # How it enumerates the two stones of a turn
//!
//! Two modes, one [`PairOrder`] apart, because the choice buys different things
//! at different prices (docs/decisions.md D-120 and its amendment).
//! [`PairOrder::BothOrderings`] reaches each pair twice, once per order, as the
//! engine's ply-level recursion does before the transposition table collapses
//! them (D-79) — which is the only check in the workspace that the two orderings
//! of a pair are worth the same at search depth.
//! [`PairOrder::Deduped`] values each unordered turn once, and is what every
//! oracle assertion runs under: it is exact, it is three to five times cheaper,
//! and the depth the saving buys is what carries the mate distances and the
//! window claim. D-79's premise keeps a check of its own —
//! `reference_dedupe_matches_both_orderings_enumeration` runs both modes over
//! the same fixtures at the depths a debug build affords, and a disagreement
//! there is the same failure the interior assertion used to raise.

use std::collections::BTreeMap;

use pistol_core::{GameState, Outcome, Phase, Player, Turn};
use pistol_eval::Weights;
use pistol_search::{CandidatePolicy, MAX_DEPTH_TURNS, candidate_cells};

use super::ref_score::RefScore;
use super::reference_walk::Walk;

/// How the reference enumerates the two stones of a turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairOrder {
    /// Both orderings of every pair, each valued on its own subtree.
    BothOrderings,
    /// Each unordered turn once: the ordering `(a, b)` is not descended into
    /// when `b` has already enumerated its own pairs at this node.
    ///
    /// Rule 4's truncation survives that, and it is the reason the key is the
    /// turn rather than the cell set: a first stone that completes a line makes
    /// a ONE-stone turn, enumerates no pair at all, and therefore never
    /// suppresses the other ordering of a pair it belongs to. The pair whose
    /// smaller cell wins is realised by one ordering only, and that ordering is
    /// the one that runs.
    Deduped,
}

/// Named invariant: a horizon landed half way through a turn, where no static
/// value is an answer (docs/decisions.md D-111).
pub const REFERENCE_HORIZON_MID_TURN: &str = "REFERENCE_HORIZON_MID_TURN";

/// Named invariant: the candidate policy offered a cell the rules refuse.
pub const REFERENCE_CANDIDATE_ILLEGAL: &str = "REFERENCE_CANDIDATE_ILLEGAL";

/// Named invariant: the policy offered nothing for the mover's second stone
/// (docs/decisions.md D-104).
pub const REFERENCE_NO_CANDIDATES_MID_TURN: &str = "REFERENCE_NO_CANDIDATES_MID_TURN";

/// Named invariant: a turn asked for a third stone.
pub const REFERENCE_TURN_OWES_A_THIRD_STONE: &str = "REFERENCE_TURN_OWES_A_THIRD_STONE";

/// Named invariant: the two orderings of one pair valued it differently.
pub const REFERENCE_PAIR_ORDER_DISAGREES: &str = "REFERENCE_PAIR_ORDER_DISAGREES";

/// Named invariant: the dedupe ledger was fed out of the ascending, distinct
/// order `candidate_cells` promises, which is what it bisects on.
pub const REFERENCE_DEDUPE_KEY_UNSORTED: &str = "REFERENCE_DEDUPE_KEY_UNSORTED";

// Which of the five above can actually fire today, stated so that a reader does
// not count them as coverage they are not. Only `REFERENCE_PAIR_ORDER_DISAGREES`
// is live, it has never fired (measured over 230 669 root turns of undesigned
// positions), and under `PairOrder::Deduped` it cannot fire at all — the second
// ordering is never walked, so the check it carries is bought by the mode
// comparison instead and not by every run. The other four restate things
// pistol-core and `candidates` already guarantee — the walk descends only at
// turn boundaries so a horizon is always at phase 0, `place` at phase 1 cannot
// return `TurnContinues`, and `candidate_cells` has already asked the rules
// about every cell it offers. They are here for the reason `pvs.rs` carries the
// same set: the guarantee is one crate away, and an extension that broke it
// should fail where the assumption is made rather than three levels up.

/// Every way the reference refuses to answer.
///
/// A `Result` rather than a panic: each of these is a question a caller can
/// legitimately ask, and rule 3 wants the refusal named rather than guessed at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceError {
    /// The root is half way through a turn. The reference counts in turns and
    /// keys its answer by `Turn`, and a line beginning half way through one
    /// cannot be grouped into `Turn` values without inventing a one-stone turn
    /// the rules do not have (docs/decisions.md D-49, D-71).
    RootMidTurn {
        /// The turn that is half played.
        turn: u32,
    },
    /// The root is already decided. There is no turn to value.
    RootDecided {
        /// The side that completed a line.
        winner: Player,
        /// The turn it completed on.
        turn: u32,
    },
    /// The policy offers the root nothing, which `Searcher` refuses by name.
    RootNoCandidates {
        /// The turn the mover owes a stone on.
        turn: u32,
        /// The policy that came up empty.
        policy: CandidatePolicy,
    },
    /// A policy radius of zero, which reaches only occupied cells and which
    /// `Searcher::new` refuses by name.
    ///
    /// It is refused here and not left to [`ReferenceError::RootNoCandidates`]
    /// because on an EMPTY board a proximity policy restricts nothing — there is
    /// no stone to be near, so rule 3 decides alone (docs/decisions.md D-77) —
    /// and the reference would answer a question the engine will not.
    PolicyRadiusZero,
    /// A depth of zero turns, or one past the horizon this build searches —
    /// the same pair `Searcher::check_root` refuses under one name.
    ///
    /// Zero because the root's own value is not a turn value, and an empty map
    /// would be the silent skip rule 3 forbids. Past the horizon because
    /// otherwise the reference starts a walk that cannot finish while the engine
    /// refuses in constant time, so a caller comparing them hangs instead of
    /// seeing a refusal.
    DepthOutOfRange {
        /// The depth asked for, in turns.
        turns: u32,
        /// The deepest the engine searches, in turns.
        max: u32,
    },
}

/// One reference run: what it found, and what it cost.
///
/// The node count is reported rather than left to a stopwatch so that the
/// numbers this suite's budget rests on can be regenerated by running it, not
/// only by having been there when they were measured (docs/decisions.md D-55,
/// D-60 applied to a runtime budget).
#[derive(Debug, Clone)]
pub struct ReferenceRun {
    /// Every turn the root may play, and what it is worth to the side to move
    /// at the root.
    pub values: BTreeMap<Turn, RefScore>,
    /// Turn-boundary nodes visited, leaves included.
    pub nodes: u64,
}

impl ReferenceRun {
    /// The best any turn achieves — the value of the position.
    pub fn best(&self) -> RefScore {
        *self
            .values
            .values()
            .max()
            .expect("an ongoing position has at least one turn")
    }

    /// Every turn that achieves it.
    pub fn argmax(&self) -> Vec<Turn> {
        let best = self.best();
        self.values
            .iter()
            .filter(|&(_, &score)| score == best)
            .map(|(&turn, _)| turn)
            .collect()
    }
}

/// Every turn the root may play, and what plain full-width negamax says it is
/// worth to the side to move at the root.
///
/// Deduped, which is the mode every oracle assertion runs under. The other mode
/// has exactly one caller, and it is named in [`PairOrder`].
pub fn reference_root_values(
    root: &GameState,
    depth_turns: u32,
    policy: CandidatePolicy,
    weights: &Weights,
) -> Result<ReferenceRun, ReferenceError> {
    reference_root_values_under(root, depth_turns, policy, weights, PairOrder::Deduped)
}

/// The same question, asked of a named enumeration mode.
pub fn reference_root_values_under(
    root: &GameState,
    depth_turns: u32,
    policy: CandidatePolicy,
    weights: &Weights,
    pairs: PairOrder,
) -> Result<ReferenceRun, ReferenceError> {
    check_root(root, depth_turns, policy)?;
    Ok(Walk::new(root, policy, weights, pairs).root(depth_turns))
}

/// What ONE named turn is worth, re-derived through pistol-core's TURN-level
/// API instead of the ply loop.
///
/// [`reference_root_values`] builds its keys by playing plies and grouping them;
/// this plays the turn with `make_turn`, which chooses its own ordering and has
/// its own refusals (docs/decisions.md D-49, D-51), and rebuilds the evaluation
/// from empty on the position that reaches. So a root map that mis-keyed a turn
/// passes the first and fails this one — including every rule-4 case, where the
/// pair whose smaller cell wins is playable in one ordering only.
///
/// It is a second path to the CHILD and not a second valuation of it: below the
/// turn it calls the same walk. Nothing here re-derives the subtree twice, and
/// this doc says so rather than letting "different path" be read as more than
/// it is.
pub fn reference_turn_value(
    root: &GameState,
    turn: Turn,
    depth_turns: u32,
    policy: CandidatePolicy,
    weights: &Weights,
) -> Result<RefScore, ReferenceError> {
    check_root(root, depth_turns, policy)?;
    let mut after = root.clone();
    after
        .make_turn(turn)
        .unwrap_or_else(|error| panic!("the reference was asked about {turn}: {error}"));

    if let Outcome::Win { turn: won_on, .. } = after.outcome() {
        return Ok(RefScore::WinInTurns(won_on - root.turn() + 1));
    }
    Ok(
        Walk::anchored(&after, policy, weights, root.turn(), PairOrder::Deduped)
            .negamax(depth_turns - 1)
            .negate(),
    )
}

/// Everything that has to hold before the reference will answer.
fn check_root(
    root: &GameState,
    depth_turns: u32,
    policy: CandidatePolicy,
) -> Result<(), ReferenceError> {
    // The policy is refused before the position is looked at, as
    // `Searcher::new` does: a radius of zero is a parameter this build will not
    // honour, not a fact about this root.
    let CandidatePolicy::Radius { radius } = policy;
    if radius == 0 {
        return Err(ReferenceError::PolicyRadiusZero);
    }
    // The rest is `Searcher::check_root`'s order, deliberately: a root that
    // trips two of these must be named the same way by both, or a disagreement
    // about WHICH refusal applies would read as a disagreement about the
    // position.
    if let Outcome::Win { winner, turn } = root.outcome() {
        return Err(ReferenceError::RootDecided { winner, turn });
    }
    if root.phase() != Phase::First {
        return Err(ReferenceError::RootMidTurn { turn: root.turn() });
    }
    if candidate_cells(root.board(), policy).is_empty() {
        return Err(ReferenceError::RootNoCandidates {
            turn: root.turn(),
            policy,
        });
    }
    if depth_turns == 0 || depth_turns > MAX_DEPTH_TURNS {
        return Err(ReferenceError::DepthOutOfRange {
            turns: depth_turns,
            max: MAX_DEPTH_TURNS,
        });
    }
    Ok(())
}
