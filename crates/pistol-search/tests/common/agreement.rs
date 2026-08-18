//! One position, two implementations, and what has to be true of both.
//!
//! Every assertion this oracle makes is here, so that a test states WHICH
//! agreement it is about and over which positions, and never how the comparison
//! is done. The three the work package names are [`Agreement::value_agrees`],
//! [`Agreement::bestmove_is_in_the_argmax_set`] and
//! [`Agreement::bestmove_value_is_the_reference_maximum`].
//!
//! # What is deliberately not asserted
//!
//! **Move identity against the reference.** Among equal-scoring turns the
//! engine's pick is its move-ordering pass, which the reference deliberately
//! does not have; membership in the argmax set is the whole claim. WHICH tie
//! the engine takes is pinned already, by the cross-process determinism gate
//! (docs/decisions.md D-7, D-119).
//!
//! **Node counts.** Not a soundness property, and rule 5 keeps performance
//! claims out of this work package.
//!
//! # Which depths each assertion gets
//!
//! (b) and (c) run at depth 1 across every fixture in the always-on tier and get
//! their deeper coverage in the release tier, where [`Agreement::holds`] runs all
//! four at depths 2 and 3. That split is the debug budget and nothing else: a
//! second turn costs the candidate count to the fourth power, and the always-on
//! tier is pre-registered under 5 s in a debug build.

use pistol_core::{GameState, Outcome};
use pistol_eval::{HandcraftedV0, Weights};
use pistol_search::{CandidatePolicy, SearchOutcome, SearchParams, Searcher, Stop};

use super::fixtures::Fixture;
use super::ref_score::{RefScore, engine_score_as_reference};
use super::reference::{ReferenceRun, reference_root_values, reference_turn_value};

/// The committed transposition table size, which is what a deployment runs.
pub const COMMITTED_TT: u64 = 268_435_456;

/// One run of each implementation over the same position, depth and universe.
pub struct Agreement {
    /// The fixture's name, so a failure names a position rather than an index.
    pub name: String,
    /// The position both were asked about.
    pub root: GameState,
    /// Depth in turns, which is the only unit either side counts in.
    pub depth_turns: u32,
    /// The candidate policy both searched under.
    pub policy: CandidatePolicy,
    /// What plain full-width negamax found.
    pub reference: ReferenceRun,
    /// What the engine answered.
    pub outcome: SearchOutcome,
}

/// Run both implementations over one position.
///
/// The stop condition is always [`Stop::DepthTurns`]: it is the one budget that
/// completes every iteration, so neither side is answering a question about a
/// clock, and the engine's abort path is never on the comparison. The searcher
/// is built fresh, because a table carried in from an earlier position is an
/// input to the answer (docs/decisions.md D-109).
pub fn agreement(
    fixture: &Fixture,
    depth_turns: u32,
    radius: u32,
    tt_bytes: u64,
    weights: &Weights,
) -> Agreement {
    let policy = CandidatePolicy::Radius { radius };
    let reference = reference_root_values(&fixture.state, depth_turns, policy, weights)
        .unwrap_or_else(|error| {
            panic!(
                "{}: the reference refused the root: {error:?}",
                fixture.name
            )
        });
    let params = SearchParams {
        tt_bytes,
        candidate_policy: policy,
    };
    let mut searcher = Searcher::new(params, Box::new(HandcraftedV0::new(weights.clone())))
        .expect("the oracle's search parameters must be accepted");
    let outcome = searcher
        .search(&fixture.state, Stop::DepthTurns(depth_turns), &mut |_| {})
        .unwrap_or_else(|error| panic!("{}: the search refused the root: {error}", fixture.name));

    Agreement {
        name: fixture.name.clone(),
        root: fixture.state.clone(),
        depth_turns,
        policy,
        reference,
        outcome,
    }
}

/// Run only the engine, for the questions that vary something the reference
/// cannot see.
///
/// The transposition table is the case this exists for: its size is an input to
/// the search and not to the game, so re-deriving the reference for each size
/// would be the same walk three times over.
pub fn search_only(
    fixture: &Fixture,
    depth_turns: u32,
    radius: u32,
    tt_bytes: u64,
    weights: &Weights,
) -> SearchOutcome {
    let params = SearchParams {
        tt_bytes,
        candidate_policy: CandidatePolicy::Radius { radius },
    };
    let mut searcher = Searcher::new(params, Box::new(HandcraftedV0::new(weights.clone())))
        .expect("the oracle's search parameters must be accepted");
    searcher
        .search(&fixture.state, Stop::DepthTurns(depth_turns), &mut |_| {})
        .unwrap_or_else(|error| panic!("{}: the search refused the root: {error}", fixture.name))
}

impl Agreement {
    /// A search under a different table size, read against the same reference.
    ///
    /// Returns the engine's score in the reference's vocabulary, having first
    /// checked that whatever move it played is one the reference rates best.
    pub fn rerun_with_tt_bytes(&self, tt_bytes: u64, weights: &Weights) -> RefScore {
        let fixture = Fixture {
            name: self.name.clone(),
            state: self.root.clone(),
        };
        let CandidatePolicy::Radius { radius } = self.policy;
        let outcome = search_only(&fixture, self.depth_turns, radius, tt_bytes, weights);
        let value = self.reference.values.get(&outcome.best).unwrap_or_else(|| {
            panic!(
                "{}: with a {tt_bytes}-byte table the search played {}, which is not a turn the \
                 reference's universe contains",
                self.name, outcome.best
            )
        });
        assert_eq!(
            *value,
            self.reference.best(),
            "{}: with a {tt_bytes}-byte table the search played {}, which the reference rates \
             below its best",
            self.name,
            outcome.best
        );
        engine_score_as_reference(outcome.info.score)
    }

    /// What the engine said, in the reference's vocabulary.
    pub fn engine_score(&self) -> RefScore {
        engine_score_as_reference(self.outcome.info.score)
    }

    /// Assertion (a): the search's root value is the reference's maximum.
    ///
    /// The comparison is on the CLASSIFIED score, so a mate agrees only if its
    /// distance in turns agrees too.
    pub fn value_agrees(&self) {
        assert_eq!(
            self.engine_score(),
            self.reference.best(),
            "{}: the search's root value must be plain negamax's maximum over the same \
             universe ({} turns, {:?})",
            self.name,
            self.depth_turns,
            self.policy
        );
    }

    /// Assertion (b): the engine's move is one the reference also rates best.
    pub fn bestmove_is_in_the_argmax_set(&self) {
        let best = self.outcome.best;
        let value = self.reference.values.get(&best).unwrap_or_else(|| {
            panic!(
                "{}: the search played {best}, which is not a turn the reference's universe \
                 contains at all ({} turns, {:?})",
                self.name, self.depth_turns, self.policy
            )
        });
        assert_eq!(
            *value,
            self.reference.best(),
            "{}: the search played {best}, which the reference rates below its best of the {} \
             turns that tie ({} turns, {:?})",
            self.name,
            self.reference.argmax().len(),
            self.depth_turns,
            self.policy
        );
    }

    /// Assertion (c): the engine's move, valued through a different path, is
    /// still worth the maximum.
    ///
    /// [`reference_turn_value`] plays the turn with pistol-core's turn-level
    /// `make_turn` instead of walking plies and grouping them, so a root map
    /// that mis-keyed a turn passes (b) and fails this.
    pub fn bestmove_value_is_the_reference_maximum(&self, weights: &Weights) {
        let played = reference_turn_value(
            &self.root,
            self.outcome.best,
            self.depth_turns,
            self.policy,
            weights,
        )
        .unwrap_or_else(|error| panic!("{}: {error:?}", self.name));
        assert_eq!(
            played,
            self.reference.best(),
            "{}: the search played {}, whose value re-derived through `make_turn` is not the \
             reference's maximum ({} turns, {:?})",
            self.name,
            self.outcome.best,
            self.depth_turns,
            self.policy
        );
    }

    /// The reported line really proves the mate the score claims.
    ///
    /// (a), (b) and (c) all look only at the ROOT turn, so a principal variation
    /// whose tail is not the line the score was proved on passes all three —
    /// which is exactly what D-78 exists to prevent, and what the cross-process
    /// determinism gate cannot see because it compares two runs of the same
    /// engine. Replaying the line is the cheap half of closing that: a claimed
    /// win must actually arrive, by the side that claimed it, on the turn the
    /// distance names.
    pub fn pv_proves_its_mate(&self) {
        // Both sides of the band, not just the winning one: `compact_mated_in_2`
        // is the suite's only even distance, and checking a claimed win while
        // leaving a claimed loss unread would be asymmetric where the mate band
        // is not.
        let (winner, distance) = match self.engine_score() {
            RefScore::WinInTurns(turns) => (self.root.to_move(), turns),
            RefScore::LossInTurns(turns) => (self.root.to_move().opponent(), turns),
            RefScore::Value(_) => return,
        };
        let mut state = self.root.clone();
        // Played as TURNS, through pistol-core's own turn-level entry point: a
        // canonical pair is not always playable in canonical order — the smaller
        // cell can be one the larger one's stone opens up, or one that would end
        // the turn under rule 4 — and choosing the order is `make_turn`'s job
        // and nobody else's (docs/decisions.md D-51).
        for &turn in &self.outcome.info.pv {
            if state.outcome().is_decided() {
                break;
            }
            state
                .make_turn(turn)
                .unwrap_or_else(|error| panic!("{}: replaying {turn}: {error}", self.name));
        }
        let Outcome::Win { winner: won, turn } = state.outcome() else {
            panic!(
                "{}: the search claims a win in {distance} turns and its own line does not \
                 reach one",
                self.name
            );
        };
        assert_eq!(
            won, winner,
            "{}: the line is won by the other side",
            self.name
        );
        assert_eq!(
            turn - self.root.turn() + 1,
            distance,
            "{}: the line wins on turn {turn}, which is not the distance the score claims",
            self.name
        );
    }

    /// Every assertion that costs nothing beyond the run itself.
    pub fn holds(&self, weights: &Weights) {
        self.value_agrees();
        self.bestmove_is_in_the_argmax_set();
        self.bestmove_value_is_the_reference_maximum(weights);
        self.pv_proves_its_mate();
    }

    /// What this run cost the reference, for the record the runtime budget
    /// rests on. Visible with `--nocapture`.
    pub fn report(&self) {
        println!(
            "oracle {:44} d{} {:?} reference_nodes={:>12} value={:?}",
            self.name,
            self.depth_turns,
            self.policy,
            self.reference.nodes,
            self.reference.best()
        );
    }
}
