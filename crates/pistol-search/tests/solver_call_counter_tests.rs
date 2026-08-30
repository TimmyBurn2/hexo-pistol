//! What the solver was ASKED, as opposed to what it SPENT.
//!
//! D-465 named a `solver_calls` counter *"the first thing the next package
//! should do"*; D-508 measured what its absence costs — the visits-per-call
//! factor `K` had to be carried as a named assumption because nothing counted
//! the denominator.

use pistol_core::{Coord, GameState};
use pistol_eval::{HandcraftedV0, Weights};
use pistol_search::params::{SolverTrigger, SolverWiring};
use pistol_search::{SearchParams, Searcher, Stop};
use pistol_solver::pn::Epsilon;
use pistol_solver::{AttackerPolicy, SolverParams};

fn weights() -> Weights {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../configs/eval_v0_weights.toml");
    Weights::load(&path).expect("the committed weights load")
}

fn staged() -> pistol_search::StagedParams {
    pistol_search::StagedParams {
        quiet_radius: 2,
        safety_net_top_k: 0,
        tier_t_own_count: 2,
        tier_t_opponent_count: 3,
        q_depth_turns: 0,
        q_triggers: pistol_search::QTriggers::DefensiveOnly,
        ordering: pistol_search::OrderingHeuristics {
            killers: false,
            history: false,
            countermove: false,
        },
    }
}

fn searcher(gate_on: bool, cap: u64) -> Searcher {
    let params = SearchParams {
        tt_bytes: 1 << 24,
        solver: gate_on.then_some(SolverWiring {
            per_call_node_cap: cap,
            trigger: SolverTrigger::AnyOpenFour,
            inner: SolverParams {
                epsilon: Epsilon::new(1, 4).expect("1/4 is valid"),
                tt_entries: 1 << 20,
                attacker_policy: AttackerPolicy::OneFreeStone,
            },
        }),
        candidate_policy: pistol_search::CandidatePolicy::Staged(staged()),
    };
    Searcher::new(params, Box::new(HandcraftedV0::new(weights())))
        .expect("the wiring's parameters are accepted")
}

fn state_of(cells: &[&str]) -> GameState {
    let mut state = GameState::new_game();
    for cell in cells {
        let coord: Coord = cell.parse().expect("a coordinate");
        state.place(coord).expect("a legal stone");
    }
    state
}

/// A midgame position both seats reach hot, so the trigger fires often enough
/// that the counters have something to count.
const HOT_MIDGAME: &[&str] = &[
    "0,0", "0,-1", "1,0", "-1,1", "1,-1", "-4,4", "2,-2", "-2,1", "0,1", "-4,1", "1,1", "-4,3",
    "-3,2", "-5,2", "-5,4", "-7,4", "-2,4", "-3,0", "-2,-1", "-1,-2", "0,4", "0,3", "1,2", "-1,4",
    "4,-1", "-2,3", "-1,3", "1,3", "4,0", "3,-2", "4,-2", "1,-2", "6,-2", "0,2", "2,2",
];

#[test]
fn a_gate_off_search_counts_nothing_because_it_asks_nothing() {
    let mut engine = searcher(false, 2048);
    let outcome = engine
        .search(&state_of(HOT_MIDGAME), Stop::Nodes(20_000), &mut |_| {})
        .expect("the search runs");
    assert_eq!(outcome.info.solver_nodes, 0);
    assert_eq!(outcome.info.solver_calls.firings, 0);
    assert_eq!(outcome.info.solver_calls.invocations, 0);
    assert_eq!(outcome.info.solver_calls.proofs, 0);
    assert_eq!(outcome.info.solver_calls.root_nodes, 0);
}

#[test]
#[ignore = "minutes in a debug build (the solver's blanket agreement asserts): \
            tools/search_oracle_check.sh runs it in release"]
fn every_firing_makes_one_or_two_invocations_and_never_more() {
    // THE RATIO IS THE POINT, and it is what a designer reading this counter
    // needs: a firing asks the attacker direction, and then the defender one
    // unless the attacker already proved a win. So invocations sit in
    // [firings, 2*firings], and a counter incremented at the wrong place
    // leaves that interval.
    let mut engine = searcher(true, 2048);
    let outcome = engine
        .search(&state_of(HOT_MIDGAME), Stop::Nodes(20_000), &mut |_| {})
        .expect("the search runs");
    let calls = outcome.info.solver_calls;
    assert!(
        calls.firings > 0,
        "the fixture must actually fire, or this test is vacuous"
    );
    assert!(
        calls.invocations >= calls.firings,
        "a firing makes at least one invocation: {calls:?}"
    );
    assert!(
        calls.invocations <= 2 * calls.firings,
        "a firing makes at most two invocations: {calls:?}"
    );
}

#[test]
#[ignore = "minutes in a debug build (the solver's blanket agreement asserts): \
            tools/search_oracle_check.sh runs it in release"]
fn the_visits_are_shared_out_over_the_invocations_that_earned_them() {
    // `K = solver_nodes / invocations` is the quantity D-508 could only assume.
    // What pins it here is the two-sided bound it must satisfy by
    // construction: every invocation spends at least one visit, and none
    // spends more than the cap.
    const CAP: u64 = 2048;
    let mut engine = searcher(true, CAP);
    let outcome = engine
        .search(&state_of(HOT_MIDGAME), Stop::Nodes(20_000), &mut |_| {})
        .expect("the search runs");
    let calls = outcome.info.solver_calls;
    let invocations = u64::from(calls.invocations);
    assert!(invocations > 0, "the fixture must invoke the solver");
    assert!(
        outcome.info.solver_nodes >= invocations,
        "each invocation spends at least its own visit: {} visits over {invocations}",
        outcome.info.solver_nodes
    );
    assert!(
        outcome.info.solver_nodes <= invocations * CAP,
        "no invocation spends past the cap: {} visits over {invocations} at cap {CAP}",
        outcome.info.solver_nodes
    );
}

#[test]
#[ignore = "minutes in a debug build (the solver's blanket agreement asserts): \
            tools/search_oracle_check.sh runs it in release"]
fn the_roots_own_visits_are_a_share_of_the_searchs_own() {
    // The root's two calls are made before the first deepening iteration and
    // are SEEDED into the tree's counter, so they are inside the same budget
    // the bracket measures. A `root_nodes` larger than `solver_nodes` would
    // mean the seed had been double-counted or the tree's total reset.
    let mut engine = searcher(true, 2048);
    let outcome = engine
        .search(&state_of(HOT_MIDGAME), Stop::Nodes(20_000), &mut |_| {})
        .expect("the search runs");
    assert!(
        outcome.info.solver_calls.root_nodes <= outcome.info.solver_nodes,
        "the root's visits are part of the search's, not extra to them: {:?} of {}",
        outcome.info.solver_calls,
        outcome.info.solver_nodes
    );
}

#[test]
#[ignore = "minutes in a debug build (the solver's blanket agreement asserts): \
            tools/search_oracle_check.sh runs it in release"]
fn a_second_search_counts_only_its_own_calls() {
    // The counters are per-search and hold no state across one, which is what
    // makes a per-search budget a quantity at all. A counter that accumulated
    // would make the second search's figure the sum of both.
    let state = state_of(HOT_MIDGAME);
    let mut engine = searcher(true, 2048);
    let first = engine
        .search(&state, Stop::Nodes(20_000), &mut |_| {})
        .expect("the search runs");
    let second = engine
        .search(&state, Stop::Nodes(20_000), &mut |_| {})
        .expect("the search runs");
    assert!(first.info.solver_calls.firings > 0, "the fixture fires");
    assert_eq!(
        first.info.solver_calls.firings, second.info.solver_calls.firings,
        "the same position under the same budget asks the same number of times"
    );
    assert_eq!(
        first.info.solver_calls.invocations,
        second.info.solver_calls.invocations
    );
}

#[test]
#[ignore = "minutes in a debug build (the solver's blanket agreement asserts): \
            tools/search_oracle_check.sh runs it in release"]
fn a_proof_is_counted_where_the_search_answers_with_one() {
    // WIN IN ONE PLY, at a turn boundary the root's own trigger reaches: P1
    // holds five in a row on the q axis with both ends empty, P2's stones are
    // scattered and block neither. The root's attacker call proves it, the
    // search answers with the proof's first move, and `proofs` must say so —
    // the increment site is otherwise unexercised, and a counter no test
    // watches fire is not a counter.
    let state = state_of(&[
        "0,0", // turn 1, P1
        "0,5", "2,6", // turn 2, P2
        "1,0", "2,0", // turn 3, P1
        "-3,7", "4,8", // turn 4, P2
        "3,0", "4,0", // turn 5, P1
        "-5,2", "6,-4", // turn 6, P2
    ]);
    let mut engine = searcher(true, 16384);
    let outcome = engine
        .search(&state, Stop::Nodes(50_000), &mut |_| {})
        .expect("the search runs");
    assert_eq!(
        outcome.provenance,
        pistol_search::Provenance::SolverProof,
        "the root's own call proves this position: {:?}",
        outcome.info.solver_calls
    );
    let calls = outcome.info.solver_calls;
    assert_eq!(
        calls.firings, 1,
        "the root's own firing, and no tree searched"
    );
    assert_eq!(
        calls.invocations, 1,
        "the attacker direction proved, so the defender one is never asked"
    );
    assert_eq!(calls.proofs, 1, "and the proof is counted: {calls:?}");
    assert_eq!(
        calls.root_nodes, outcome.info.solver_nodes,
        "a root proof spends nothing outside the root"
    );
}
