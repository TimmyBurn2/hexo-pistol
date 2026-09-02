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

/// The armed seat's shape, cap and all: `configs/gate_staged_solver_v0.toml`'s
/// values, spelled here because this crate reads no config.
fn searcher(gate_on: bool) -> Searcher {
    let params = SearchParams {
        tt_bytes: 1 << 24,
        solver: gate_on.then_some(SolverWiring {
            per_call_node_cap: 512,
            trigger: SolverTrigger::AnyOpenFour,
            inner: SolverParams {
                epsilon: Epsilon::new(1, 4).expect("1/4 is valid"),
                tt_entries: 1 << 20,
                attacker_policy: AttackerPolicy::OneFreeStone,
            },
        }),
        candidate_policy: pistol_search::CandidatePolicy::Staged(pistol_search::StagedParams {
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
        }),
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

/// `bench_positions_v1.txt`'s first band-15 entry, stone for stone — the
/// position `trigger_census_cover_tests` reads, so the fold is counted over a
/// COMMITTED workload rather than one chosen to make a property hold.
///
/// Fifteen stones rather than the thirty-five of that suite's other fixture,
/// and the difference is not cosmetic: the legal region is the union of
/// radius-8 balls, so a longer game costs a test build orders more per node.
/// MEASURED, whole suite: 198 s on the long fixture against seconds here.
const BENCH_B15_FIRST: &[&str] = &[
    "0,0", "-1,1", "1,0", "0,1", "0,2", "-1,0", "1,-1", "0,-1", "1,-2", "0,-2", "0,3", "-1,-1",
    "1,1", "-1,2", "-1,3",
];

#[test]
fn the_fold_is_entered_exactly_once_per_firing() {
    // The STRUCTURAL check the perf guard cannot make: at 23 microseconds a
    // fold, a hoist out of the census closure, a call per NODE and a double
    // call are all below what an nps comparison resolves. A counter is not.
    let mut engine = searcher(true);
    engine.collect_trigger_census();
    engine
        .search(&state_of(BENCH_B15_FIRST), Stop::Nodes(1_500), &mut |_| {})
        .expect("the search runs");
    let rows = engine.take_trigger_census();
    assert!(
        rows.len() > 1,
        "the budget is too small for this check: {} firing(s), and a counter that agrees with a \
         single row agrees with almost anything",
        rows.len()
    );
    assert_eq!(
        engine.census_fold_entries(),
        rows.len() as u64,
        "the fold ran {} times for {} firings",
        engine.census_fold_entries(),
        rows.len()
    );
}

#[test]
fn the_non_census_path_does_not_compute_a_canonical_key() {
    // The guard, from the side that has no census at all: a fold hoisted out
    // of `is_some().then(..)` is paid by every shipped search in the tree, and
    // the row set cannot see that because there is no row set.
    //
    // THE FIRST HALF IS WHAT STOPS THIS PASSING VACUOUSLY, and it is here
    // because the test without it DID. A firing has TWO sites — the root's, in
    // `search.rs`, and the in-tree one in `pvs.rs` — and at a budget the root's
    // own solver calls exhaust, the in-tree site is never reached at all. A
    // hoist there is then invisible, and a MUTATION RUN measured exactly that:
    // the fold hoisted out of the in-tree guard survived this test at 600
    // nodes. So the workload is made to reach that site, and the assertion is
    // that it reached it.
    let mut armed = searcher(true);
    armed.collect_trigger_census();
    armed
        .search(&state_of(BENCH_B15_FIRST), Stop::Nodes(1_500), &mut |_| {})
        .expect("the search runs");
    assert!(
        armed
            .take_trigger_census()
            .iter()
            .any(|row| row.columns.turns_from_root > 0),
        "this budget never reaches the IN-TREE firing site, so a fold hoisted there would be \
         invisible to the check below"
    );

    let mut engine = searcher(true);
    engine
        .search(&state_of(BENCH_B15_FIRST), Stop::Nodes(1_500), &mut |_| {})
        .expect("the search runs");
    assert_eq!(
        engine.census_fold_entries(),
        0,
        "a search with no census armed entered the canonical-key fold"
    );
}

#[test]
fn a_search_with_the_solver_gate_off_produces_no_census_row_under_the_token() {
    // A LIMITATION, pinned so a successor meets it in the suite rather than
    // after a months-long sweep: the census fires where the solver does, and
    // the seat the production sweep runs has the gate off (wp20b_design.md F3).
    let mut engine = searcher(false);
    engine.collect_trigger_census();
    engine
        .search(&state_of(BENCH_B15_FIRST), Stop::Nodes(600), &mut |_| {})
        .expect("the search runs");
    assert!(
        engine.take_trigger_census().is_empty(),
        "an unarmed seat produced a census row, so the census reaches past the solver wiring"
    );
    assert_eq!(engine.census_fold_entries(), 0);
}

#[test]
fn each_identity_column_is_the_derivation_it_is_named_for() {
    // THE PROPERTY, not a correlate of it. Its predecessor asserted
    // `key != key_pos`, which is true for essentially any value of `key` other
    // than literally `state.key()` — so EXCHANGING the two derivations left the
    // identity column carrying the position key, the option the design's F2
    // rules out, and **seventy tests green**, including that one. A review
    // demonstrated it with a live mutant.
    //
    // Both columns are now pinned against referents computed here, sharing no
    // code with the census: `canonical_key` over the board's own stones, and
    // `GameState::key`. An exchange inside `CensusKeys::at` fails BOTH
    // assertions. It says NOTHING about the two firing sites, which name the
    // fields themselves — that is
    // `an_in_tree_rows_identity_is_the_canonical_key_of_the_position_it_fired_on`
    // below and the root half of the test after this one.
    for cells in [
        BENCH_B15_FIRST,
        &BENCH_B15_FIRST[..7],
        &["0,0", "1,0", "0,1", "2,0", "1,-1"][..],
    ] {
        let state = state_of(cells);
        let stones: Vec<(Coord, pistol_core::Player)> = state.board().stones().collect();
        let keys = pistol_search::CensusKeys::at(&state);
        assert_eq!(
            keys.key,
            pistol_core::canonical_key(&stones),
            "the identity column is not the canonical key of {cells:?}"
        );
        assert_eq!(
            keys.key_pos,
            state.key(),
            "the position column is not `GameState::key` of {cells:?}"
        );
        assert_ne!(
            keys.key, keys.key_pos,
            "the two derivations coincide, which no position should make happen"
        );
    }
}

#[test]
fn every_census_row_carries_the_two_columns_that_function_produces() {
    // The CALL, at both firing sites. The test above pins what the function
    // returns; this pins that the rows carry what it returned and not something
    // else — D-553's pairing, at the site whose rows D-537's denominator will be
    // counted over.
    let mut engine = searcher(true);
    engine.collect_trigger_census();
    engine
        .search(&state_of(BENCH_B15_FIRST), Stop::Nodes(4_000), &mut |_| {})
        .expect("the search runs");
    let rows = engine.take_trigger_census();
    assert!(
        rows.iter().any(|row| row.columns.turns_from_root == 0),
        "this workload never fired at the ROOT"
    );
    assert!(
        rows.iter().any(|row| row.columns.turns_from_root > 0),
        "this workload never fired IN-TREE, so that site's columns are unread"
    );
    let root = rows
        .iter()
        .find(|row| row.columns.turns_from_root == 0)
        .expect("the root fired");
    let at_root = pistol_search::CensusKeys::at(&state_of(BENCH_B15_FIRST));
    assert_eq!(root.key, at_root.key);
    assert_eq!(root.key_pos, at_root.key_pos);
    for row in &rows {
        assert_ne!(
            row.key, row.key_pos,
            "a firing at turn {} from the root carries one derivation in both columns",
            row.columns.turns_from_root
        );
    }
}

#[test]
fn a_second_collect_starts_the_fold_count_again() {
    // The counter is per census, not per searcher: without this a second
    // census's `fold entries == rows` check would read the first one's total.
    let mut engine = searcher(true);
    engine.collect_trigger_census();
    engine
        .search(&state_of(BENCH_B15_FIRST), Stop::Nodes(600), &mut |_| {})
        .expect("the search runs");
    let first = engine.take_trigger_census().len() as u64;
    assert_eq!(engine.census_fold_entries(), first);
    engine.collect_trigger_census();
    assert_eq!(engine.census_fold_entries(), 0);
}

/// The position the IN-TREE firing happens on when `BENCH_B15_FIRST` is
/// searched at 1,500 nodes on the seat above — those fifteen stones plus the
/// four the search's first two turns add.
///
/// **DERIVED FROM THE FIRING SITE, NOT CHOSEN**: the run that read it off, and
/// the instrumentation that re-reads it, are recorded in
/// `docs/experiments/wp20b_B1_remedy.md` §1. Nothing here is taken on trust —
/// the test REPLAYS these stones and computes both identities from
/// `pistol-core` alone, so a fixture that stopped being the firing position
/// would fail the test rather than quietly weaken it.
const IN_TREE_FIRING_AT_1500: &[&str] = &[
    "0,0", "-1,1", "1,0", "0,1", "0,2", "-1,0", "1,-1", "0,-1", "1,-2", "0,-2", "0,3", "-1,-1",
    "1,1", "-1,2", "-1,3", "-1,4", "4,-2", "-2,3", "-3,3",
];

#[test]
fn an_in_tree_rows_identity_is_the_canonical_key_of_the_position_it_fired_on() {
    // THE ASSERTION ROUND 2 SPECIFIED AND ROUND 3 FOUND MISSING, and the one
    // every other in-tree check in this repository is a correlate of. The
    // others compare an in-tree row's two columns to each other
    // (`key != key_pos`), and an EXCHANGE of the columns PRESERVES that — which
    // is how the option-A identity design F2 forbids reached every in-tree
    // census row with seventy-three tests green.
    //
    // The referent here is derived OUTSIDE the search twice over: from a
    // position replayed stone by stone, and through the MIRROR — the value
    // asserted is the canonical key of TRANSFORMED spellings of those stones.
    // Only a symmetry-folded identity equals that. `GameState::key` of any
    // spelling does not, so an exchange at the in-tree push site fails here
    // instead of passing.
    let replayed = state_of(IN_TREE_FIRING_AT_1500);
    let stones: Vec<(Coord, pistol_core::Player)> = replayed.board().stones().collect();
    let referents: std::collections::BTreeSet<pistol_core::Key128> = pistol_core::Symmetry::ALL
        .iter()
        .map(|&symmetry| {
            pistol_core::canonical_key(&pistol_core::symmetry::transform(&stones, symmetry))
        })
        .collect();
    assert_eq!(
        referents.len(),
        1,
        "the twelve images of one position do not share a canonical key, so this referent \
         proves nothing about folding: {referents:?}"
    );
    let referent = *referents
        .iter()
        .next()
        .expect("a one-element set has an element");
    assert_ne!(
        referent,
        replayed.key(),
        "the canonical key and the position key of the firing position coincide, so this \
         position cannot tell the two columns apart"
    );

    let mut engine = searcher(true);
    engine.collect_trigger_census();
    engine
        .search(&state_of(BENCH_B15_FIRST), Stop::Nodes(1_500), &mut |_| {})
        .expect("the search runs");
    let rows = engine.take_trigger_census();
    let in_tree: Vec<_> = rows
        .iter()
        .filter(|row| row.columns.turns_from_root > 0)
        .collect();
    assert!(
        !in_tree.is_empty(),
        "this budget never reached the IN-TREE firing site, so the assertion below would pass \
         vacuously"
    );
    let carrying = in_tree
        .iter()
        .find(|row| row.key == referent)
        .unwrap_or_else(|| {
            panic!(
                "no IN-TREE census row carries the canonical key of the position it fired on. \
                 TWO THINGS PRODUCE THIS and they are told apart by whether the search moved: \
                 (a) the identity column is not a canonical key at the in-tree push site — the \
                 defect this test exists for; (b) the search now fires in-tree somewhere else, \
                 so IN_TREE_FIRING_AT_1500 is stale and must be re-derived by \
                 docs/experiments/wp20b_B1_remedy.md section 2. Referent {referent}, rows \
                 {in_tree:?}"
            )
        });
    assert_eq!(
        carrying.key_pos,
        replayed.key(),
        "the IN-TREE row whose identity column is the firing position's canonical key carries \
         some other position's `GameState::key` beside it"
    );
}
