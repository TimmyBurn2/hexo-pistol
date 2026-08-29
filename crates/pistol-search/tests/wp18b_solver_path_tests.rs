use pistol_core::{Coord, GameState, Player, Turn};
use pistol_eval::{HandcraftedV0, Weights};
use pistol_search::params::{SolverTrigger, SolverWiring};
use pistol_search::{Provenance, SearchParams, Searcher, Stop};
use pistol_solver::pn::Epsilon;
use pistol_solver::{AttackerPolicy, Solver, SolverParams};

fn wiring() -> SolverWiring {
    wiring_capped(16384)
}

/// The same wiring at a stated cap: §4d's budget gate needs a cap small
/// enough that a DEBUG build finishes, and the bound it asserts is stated in
/// terms of the cap rather than of a literal.
fn wiring_capped(per_call_node_cap: u64) -> SolverWiring {
    SolverWiring {
        per_call_node_cap,
        trigger: SolverTrigger::AnyOpenFour,
        inner: SolverParams {
            epsilon: Epsilon::new(1, 4).expect("1/4 is valid"),
            tt_entries: 1 << 20,
            attacker_policy: AttackerPolicy::OneFreeStone,
        },
    }
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

fn weights() -> Weights {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../configs/eval_v0_weights.toml");
    Weights::load(&path).expect("the committed weights load")
}

fn searcher(gate_on: bool) -> Searcher {
    searcher_capped(gate_on, 16384)
}

fn searcher_capped(gate_on: bool, cap: u64) -> Searcher {
    let params = SearchParams {
        tt_bytes: 1 << 24,
        solver: gate_on.then(|| wiring_capped(cap)),
        candidate_policy: pistol_search::CandidatePolicy::Staged(staged()),
    };
    Searcher::new(params, Box::new(HandcraftedV0::new(weights())))
        .expect("the wiring's parameters are accepted")
}

fn state_of(plies: &[&str]) -> GameState {
    let plies: Vec<Coord> = plies.iter().map(|word| word.parse().unwrap()).collect();
    GameState::from_plies(&plies).expect("a legal game prefix")
}

/// g001 turn 42: sealbot (P2) to move, the M4 flip position — v0 refuted
/// it (955 nodes), the widened solver proves the win (10,726 nodes, depth
/// 3). A root ATTACKER proof.
const G001_T42: &[&str] = &[
    "0,0", "2,-2", "1,0", "0,1", "0,5", "0,-2", "0,3", "1,-2", "2,-3", "-1,-1", "0,-1", "5,-6",
    "6,-7", "3,-4", "0,-4", "0,-3", "1,-3", "-1,-3", "1,-4", "2,-4", "5,-3", "3,-3", "-1,-4",
    "5,-5", "5,-4", "5,-7", "5,-2", "-2,-1", "-1,-2", "-2,-4", "4,-1", "-3,-4", "3,-1", "4,-2",
    "6,-3", "-4,1", "-1,1", "-3,0", "-3,1", "2,1", "4,1", "3,1", "-3,-1", "-3,3", "-2,-2", "7,-4",
    "-3,-3", "-3,2", "8,-5", "7,-2", "6,-2", "3,-2", "8,-2", "7,-3", "4,0", "-2,2", "5,-1", "7,-1",
    "-4,4", "7,-5", "7,1", "6,-5", "-4,-2", "-5,2", "-4,2", "-1,2", "-7,2", "8,-4", "8,-3", "8,-6",
    "8,0", "-2,0", "-2,1", "-2,3", "-2,-3", "1,4", "2,3", "3,2", "6,0", "-3,8", "6,-1",
];

/// g001 turn 45: PISTOL (P1) to move, one turn before sealbot converts at
/// 46 — the opponent holds an unblockable overload, so the DEFENDER
/// direction proves P2's win in one node (LAW-OVERLOAD at the AND root).
const G001_T45: &[&str] = &[
    "0,0", "2,-2", "1,0", "0,1", "0,5", "0,-2", "0,3", "1,-2", "2,-3", "-1,-1", "0,-1", "5,-6",
    "6,-7", "3,-4", "0,-4", "0,-3", "1,-3", "-1,-3", "1,-4", "2,-4", "5,-3", "3,-3", "-1,-4",
    "5,-5", "5,-4", "5,-7", "5,-2", "-2,-1", "-1,-2", "-2,-4", "4,-1", "-3,-4", "3,-1", "4,-2",
    "6,-3", "-4,1", "-1,1", "-3,0", "-3,1", "2,1", "4,1", "3,1", "-3,-1", "-3,3", "-2,-2", "7,-4",
    "-3,-3", "-3,2", "8,-5", "7,-2", "6,-2", "3,-2", "8,-2", "7,-3", "4,0", "-2,2", "5,-1", "7,-1",
    "-4,4", "7,-5", "7,1", "6,-5", "-4,-2", "-5,2", "-4,2", "-1,2", "-7,2", "8,-4", "8,-3", "8,-6",
    "8,0", "-2,0", "-2,1", "-2,3", "-2,-3", "1,4", "2,3", "3,2", "6,0", "-3,8", "6,-1", "5,0",
    "-2,7", "2,0", "7,0", "-1,-5", "4,-4",
];

/// A root attacker proof answers with the PROOF's first move, the mate
/// score at the proof's distance, the SolverProof provenance, and the
/// solver's nodes as the whole cost (design wp18b §2 D3).
#[test]
fn a_root_attacker_proof_answers_with_the_proofs_first_move() {
    let state = state_of(G001_T42);
    assert_eq!(state.to_move(), Player::P2);
    let mut engine = searcher(true);
    let outcome = engine
        .search(&state, Stop::Nodes(50_000), &mut |_| {})
        .expect("the search runs");
    assert_eq!(outcome.provenance, Provenance::SolverProof);
    assert_eq!(outcome.info.depth_turns, 3, "the proof's own depth");
    // mate at distance 2*3-1 = 5 from the root: odd, a win for the mover.
    assert_eq!(outcome.info.score, pistol_search::MATE - 5);
    // The proof's first move is an arm-B pair at this position (the M4
    // flip's own witness); what is pinned HERE is that the move comes from
    // the proof: one stone inside C, one outside it.
    let mut threat = pistol_solver::ThreatState::new();
    for (at, player) in state.board().stones() {
        threat.apply(at, player);
    }
    let mut candidates = Vec::new();
    pistol_solver::policy::candidate_cells(&threat, Player::P2, &mut candidates);
    let [first, second] = match outcome.best {
        Turn::Pair(a, b) => [a, b],
        Turn::Single(_) => panic!("the t42 witness is a pair"),
    };
    let inside_c = candidates.binary_search(&first).is_ok() as u8
        + candidates.binary_search(&second).is_ok() as u8;
    assert_eq!(
        inside_c, 1,
        "the proof's arm-B witness: exactly one stone inside C"
    );
    // The whole cost is the solver's: no search nodes were spent.
    assert_eq!(outcome.info.search_nodes, 0);
    assert!(outcome.info.solver_nodes > 0);
    assert_eq!(outcome.info.nodes, outcome.info.solver_nodes);
}

/// A root defender proof restricts the root's candidates to the proof's Z2
/// zone: every returned move's cells lie inside it, and nothing outside
/// leaks (design wp18b §2 D3 — the mutation "let a defender zone leak a
/// non-zone candidate" dies here).
#[test]
fn a_root_defender_proof_restricts_candidates_to_the_zone() {
    let state = state_of(G001_T45);
    assert_eq!(state.to_move(), Player::P1);
    // The zone, derived independently: the same defender proof, read
    // through the same exported seam the search uses.
    let mut oracle = Solver::new(
        Epsilon::new(1, 4).unwrap(),
        1 << 20,
        AttackerPolicy::OneFreeStone,
    );
    let proof = oracle.solve_defender(&state, 16384);
    let pistol_solver::SolveOutcome::Win(tree) = proof.outcome else {
        panic!("g001-t45 is defender-proven (the overload conversion)");
    };
    let zone = pistol_search::proof_root_zone(&tree);
    assert!(!zone.is_empty());
    let mut engine = searcher(true);
    let outcome = engine
        .search(&state, Stop::Nodes(50_000), &mut |_| {})
        .expect("the search runs");
    // The defender direction spent solver nodes at the root (one node at
    // this position: the AND root is an overload leaf).
    assert!(outcome.info.solver_nodes > 0);
    // And the answered move's PROMOTION stays inside the zone. The
    // restriction narrows the root's CANDIDATE CELLS (the first-stone
    // promotions, design wp18b §2 D3); the pair's second stone comes from
    // the child position's own policy and is not the root's to restrict.
    // A canonical `Turn` does not spell play order, so what is pinned is
    // the intersection: at least one stone of the answered pair lies in
    // the zone — the promotion — and with the restriction dropped (the
    // registered mutation) this position's tactical best lies wholly
    // outside it, which is the kill.
    let [first, second] = match outcome.best {
        Turn::Pair(a, b) => [a, b],
        Turn::Single(at) => [at, at],
    };
    assert!(
        zone.binary_search(&first).is_ok() || zone.binary_search(&second).is_ok(),
        "the answered pair's promotion lies inside the proof zone: {first:?} / {second:?}"
    );
}

/// The budget-sum law (design wp18b §3): `nodes` is the derived sum of the
/// two independent counters, exactly, on every outcome path. A one-node
/// drift fails this.
#[test]
fn nodes_is_exactly_the_sum_of_the_two_counters() {
    for gate_on in [false, true] {
        // t45, NOT t42: the t42 gate-on search answers through the root
        // proof constructor, whose `nodes` is the solver count directly —
        // mutation-proof by construction. t45 runs a FULL search with
        // interior calls, whose `nodes` is the derived sum the accounting
        // mutation breaks (the first mutation round survived exactly
        // because only t42 was driven; the fix is the fixture, not a
        // weaker claim).
        let state = state_of(G001_T45);
        let mut engine = searcher(gate_on);
        let outcome = engine
            .search(&state, Stop::Nodes(50_000), &mut |_| {})
            .expect("the search runs");
        assert_eq!(
            outcome.info.nodes,
            outcome
                .info
                .search_nodes
                .saturating_add(outcome.info.solver_nodes),
            "gate_on = {gate_on}: nodes is the derived sum, exactly"
        );
        if !gate_on {
            assert_eq!(outcome.info.solver_nodes, 0);
            assert_eq!(outcome.info.nodes, outcome.info.search_nodes);
        }
    }
}

/// The defender-parity law, end to end (design wp18b §4): a proven
/// opponent win scores the loss on one of the OPPONENT's turns — even
/// distance from the root. At g001-t45 the opponent converts at turn 47's
/// line of play: the score must read MATED at an even distance.
#[test]
fn a_proven_opponent_win_scores_an_even_distance_loss() {
    let state = state_of(G001_T45);
    let mut engine = searcher(true);
    let outcome = engine
        .search(&state, Stop::Nodes(50_000), &mut |_| {})
        .expect("the search runs");
    let score = outcome.info.score;
    assert!(score < 0, "the position is proven lost: {score}");
    let distance = (pistol_search::MATE as i64 - i64::from(score.abs())) as u32;
    assert!(
        distance.is_multiple_of(2),
        "a loss reads on the opponent's turn — even distance, got {distance}"
    );
}

/// The guards (design wp18b §5): the game's first turn owes ONE stone and
/// is never a legal solver position — a gate-on search there must not
/// panic, and simply search.
#[test]
fn the_first_turn_never_calls_the_solver() {
    let mut engine = searcher(true);
    let outcome = engine
        .search(&GameState::new_game(), Stop::DepthTurns(1), &mut |_| {})
        .expect("turn 1 searches without a solver call");
    assert_eq!(outcome.info.solver_nodes, 0);
}

/// The Radius refusal (design wp18b §2 D1): the gate under a Radius-kind
/// policy is refused BY NAME, never a silent no-op.
#[test]
fn the_gate_under_radius_is_refused_by_name() {
    let params = SearchParams {
        tt_bytes: 1 << 24,
        solver: Some(wiring()),
        candidate_policy: pistol_search::CandidatePolicy::Radius { radius: 2 },
    };
    let err = match Searcher::new(params, Box::new(HandcraftedV0::new(weights()))) {
        Err(err) => err,
        Ok(_) => panic!("the wiring is refused under radius"),
    };
    let rendered = format!("{err:?}");
    assert!(
        rendered.contains("solver.on_search_path"),
        "the refusal names its key: {rendered}"
    );
}

/// Newgame clears the solver (design wp18b §1): the honest form — epoch
/// isolation already makes the skip unobservable, so what is pinned is
/// that clearing CHANGES NOTHING observable (the predicted-unkillable
/// mutation's honest receipt: run it, see it agree, record why).
#[test]
fn newgames_solver_clear_is_observably_neutral() {
    let state = state_of(G001_T42);
    let mut once = searcher(true);
    let first = once
        .search(&state, Stop::Nodes(50_000), &mut |_| {})
        .unwrap();
    once.clear();
    let second = once
        .search(&state, Stop::Nodes(50_000), &mut |_| {})
        .unwrap();
    assert_eq!(
        (first.best, first.info.nodes, first.info.score),
        (second.best, second.info.nodes, second.info.score),
        "a cleared solver answers identically — the reset is hygiene, not state"
    );
}

/// WP-1.8c §4d: the node budget binds the ON seat too.
///
/// wp18b §3 masked the stop on the DERIVED total, which a solver call moves
/// by its whole node count at once — so the exact-multiple test stepped over
/// every multiple and the stop did not fire. MEASURED before the fix: a mean
/// 156,313 nodes per corpus position against a 50,000 budget, maximum 648,192.
/// The bound below is what wp18b §3 CLAIMED and did not have: one interval of
/// search nodes, plus one visit's own two capped calls.
///
/// THE POSITION IS PART OF THE GATE TOO. `G001_T45` answers through a root
/// proof in about a thousand nodes, so the budget never binds there and NO
/// budget check — broken or fixed — is ever consulted; the mutation round found
/// that too. This one is a midgame position where the gated seat runs long
/// enough to be stopped.
///
/// THE CAP IS PART OF THE GATE, not a convenience. A solver call moves the
/// derived total by up to `2 * CAP`, and the broken check misses a multiple of
/// `NODE_CHECK_INTERVAL` only when the jump can step over one — so at a cap
/// below half the interval the old check still lands often enough to pass this
/// bound, and the mutant survives. It did: the first version of this test used
/// 256 and the mutation round found it. At the registered 2048 a call's two
/// directions move the derived total by up to four intervals, which is what the
/// broken check cannot see past.
const CORPUS_B35_BUDGET_BOUND: &[&str] = &[
    "0,0", "0,-1", "1,0", "-1,1", "1,-1", "-4,4", "2,-2", "-2,1", "0,1", "-4,1", "1,1", "-4,3",
    "-3,2", "-5,2", "-5,4", "-7,4", "-2,4", "-3,0", "-2,-1", "-1,-2", "0,4", "0,3", "1,2", "-1,4",
    "4,-1", "-2,3", "-1,3", "1,3", "4,0", "3,-2", "4,-2", "1,-2", "6,-2", "0,2", "2,2",
];

#[test]
#[ignore = "minutes in a debug build (the solver's blanket agreement asserts): \
            tools/search_oracle_check.sh runs it in release"]
fn the_node_budget_binds_the_gated_seat() {
    // Small enough that a DEBUG build finishes — the blanket agreement asserts
    // make a debug solver visit expensive — and large enough that the seat is
    // stopped BY THE BUDGET, which the assertion below insists on rather than
    // hopes for.
    const BUDGET: u64 = 20_000;
    let state = state_of(CORPUS_B35_BUDGET_BOUND);
    // TWO CAPS, and the pair is the gate rather than a belt-and-braces. 2048 is
    // §4b's registered value, so the seat that ships is covered. 1024 is there
    // because the broken check's severity depends on ARITHMETIC the registered
    // cap happens to be kind to: two capped calls at 2048 move the derived
    // total by close to 4 x NODE_CHECK_INTERVAL, which PRESERVES the residue,
    // so the exact-multiple test keeps landing and the overshoot stays inside
    // the bound. MEASURED: with the fix reverted, cap 2048 stays under bound
    // and cap 1024 spends 11,264 nodes against a 4,000 budget. A gate run only
    // at the registered cap would have passed through the defect.
    for cap in [1024u64, 2048] {
        let mut engine = searcher_capped(true, cap);
        let outcome = engine
            .search(&state, Stop::Nodes(BUDGET), &mut |_| {})
            .expect("the search runs");
        assert!(
            outcome.info.solver_nodes > 0,
            "cap {cap}: the seat must actually call the solver, or this gate is vacuous"
        );
        // THE GATE'S OWN NON-VACUITY: a search that answers before its budget
        // binds satisfies ANY upper bound, including the one the broken check
        // satisfied. The first version of this test ran a position that did
        // exactly that, and the mutation round found it green with the fix
        // reverted.
        assert!(
            outcome.info.nodes >= BUDGET,
            "cap {cap}: the seat must be STOPPED by the budget, not finish under \
             it: {} nodes",
            outcome.info.nodes
        );
        let bound = BUDGET + pistol_search::NODE_CHECK_INTERVAL + 2 * cap;
        assert!(
            outcome.info.nodes <= bound,
            "cap {cap}: gated seat spent {} nodes against a {BUDGET} budget \
             (bound {bound})",
            outcome.info.nodes
        );
    }
}

/// The other half of §4d's claim: the ungated seat's own bound is the tight
/// one the gated seat had lost, and the added disjunct does not loosen it.
///
/// `solver_nodes` is zero for the whole life of a gate-off search, so the
/// disjunct never fires and the mask lands where it always landed. Byte-
/// identity of gate-off node counts is carried by `search_budget_tests.rs`,
/// which pins the stopping node at several ragged budgets and is what kills a
/// mutant that checks too eagerly — NOT by `tools/determinism.sh`, which runs
/// one binary twice and so is invariant under any code change (REVIEW-impl's
/// I-5). What this pins is the overshoot the gated seat measured at 3x its
/// budget before §4d.
#[test]
fn the_ungated_seat_keeps_the_tight_overshoot_bound() {
    const BUDGET: u64 = 20_000;
    let state = state_of(CORPUS_B35_BUDGET_BOUND);
    let mut engine = searcher(false);
    let outcome = engine
        .search(&state, Stop::Nodes(BUDGET), &mut |_| {})
        .expect("the search runs");
    assert_eq!(outcome.info.solver_nodes, 0);
    assert!(
        outcome.info.nodes >= BUDGET,
        "the ungated seat must be stopped by the budget too: {} nodes",
        outcome.info.nodes
    );
    assert!(
        outcome.info.nodes <= BUDGET + pistol_search::NODE_CHECK_INTERVAL,
        "an ungated search overshoots by less than one check interval, got {}",
        outcome.info.nodes
    );
}
