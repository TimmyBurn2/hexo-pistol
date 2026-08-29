mod common;

use pistol_core::{Coord, GameState};
use pistol_search::{
    CandidatePolicy, OrderingHeuristics, QTriggers, SearchParams, Searcher, StagedParams, Stop,
};

/// The staged policy the committed instrument config states, with the cap set
/// by the caller. Every value is stated here (CLAUDE.md rule 1).
fn searcher(quiet_radius: u32, safety_net_top_k: u64) -> Searcher {
    Searcher::new(
        SearchParams {
            tt_bytes: common::SMALL_TT,
            solver: None,
            candidate_policy: CandidatePolicy::Staged(StagedParams {
                quiet_radius,
                safety_net_top_k,
                tier_t_own_count: 2,
                tier_t_opponent_count: 3,
                q_depth_turns: 0,
                q_triggers: QTriggers::DefensiveOnly,
                ordering: OrderingHeuristics {
                    killers: false,
                    history: false,
                    countermove: false,
                },
            }),
        },
        Box::new(pistol_eval::HandcraftedV0::new(common::committed_weights())),
    )
    .expect("the staged parameters are accepted")
}

/// One capped search from the opening, returning its counters — the three
/// store-rule halves below all read the same tree.
fn capped_search(depth_turns: u32) -> pistol_search::StageCounters {
    searcher(2, 8)
        .search(
            &GameState::new_game(),
            Stop::DepthTurns(depth_turns),
            &mut |_| {},
        )
        .expect("the search runs")
        .info
        .stages
}

/// THE `ply > 1` FALSIFIER (docs/experiments/wp15d_design.md §7).
///
/// An empty-board root is the ONE position where a turn-indexed exemption and a
/// ply-indexed one disagree, because rule 3 gives turn 1 a single stone: the
/// ply-0 stone COMPLETES the root turn, so the ply-1 node is already at
/// `turns_from_root() == 1` and the shipped guard truncates it while `ply > 1`
/// would leave it whole.
///
/// The constant is 9 and the fixture is its own receipt — the test builds the
/// CAPPED tree and reads the counter, rather than asserting a count taken on the
/// uncapped one (docs/decisions.md D-481, D-482).
#[test]
fn at_a_turn_one_root_the_cap_binds_at_ply_one_because_that_ply_is_a_new_turn() {
    let outcome = searcher(2, 8)
        .search(&GameState::new_game(), Stop::DepthTurns(2), &mut |_| {})
        .expect("the search runs");
    let stages = outcome.info.stages;

    // ply 0 is the root turn and exempt; ply 1 is turn 2, pool 18, cut to 8;
    // each of those 8 children is a capped row too. 1 + 8.
    assert_eq!(
        stages.safety_net_capped_rows, 9,
        "the capped tree has one ply-1 row and its eight children; the 19 a \
         reader may expect is the predicate's count on the UNCAPPED tree, which \
         capping is what changes (D-481)"
    );
    assert_eq!(
        stages.safety_net_emitted_cells,
        9 * 8,
        "every capped row emits exactly the cap"
    );
    assert!(
        stages.safety_net_pool_cells > stages.safety_net_emitted_cells,
        "and every one of them held more than it emitted"
    );
}

/// The gate's off-value changes nothing: with the cap disabled the counters are
/// silent and no row is truncated, which is the property every committed config
/// relies on.
#[test]
fn the_off_value_truncates_nothing_and_records_nothing() {
    let outcome = searcher(2, 0)
        .search(&GameState::new_game(), Stop::DepthTurns(2), &mut |_| {})
        .expect("the search runs");
    let stages = outcome.info.stages;
    assert_eq!(stages.safety_net_capped_rows, 0);
    assert_eq!(stages.safety_net_emitted_cells, 0);
    assert_eq!(stages.safety_net_pool_cells, 0);
    assert_eq!(stages.safety_net_upper_withheld, 0);
    assert_eq!(stages.safety_net_exact_withheld, 0);
    assert!(
        stages.batched_quiet_safety_net > 0,
        "the rows were there to cap; the gate is what declined to"
    );
}

/// K and K+1 differ by exactly one cell per capped row — the cap boundary, read
/// as a difference rather than as an absolute (the off-by-one mutant dies here).
#[test]
fn k_and_k_plus_one_differ_by_exactly_one_cell_on_every_capped_row() {
    let at = |k: u64| {
        let outcome = searcher(2, k)
            .search(&GameState::new_game(), Stop::DepthTurns(2), &mut |_| {})
            .expect("the search runs");
        outcome.info.stages
    };
    let (small, large) = (at(6), at(7));
    assert_eq!(
        small.safety_net_emitted_cells,
        small.safety_net_capped_rows * 6
    );
    assert_eq!(
        large.safety_net_emitted_cells,
        large.safety_net_capped_rows * 7
    );
}

/// The root turn is whole with the cap armed. Both of its nodes emit exactly
/// what the gate-off engine emits there, which is the property D-478 selects
/// this scope for.
#[test]
fn the_root_turn_emits_the_same_set_with_the_cap_armed() {
    // A spread position: every batched row here is a safety-net row, so if the
    // cap could reach the root turn it would.
    let mut state = GameState::new_game();
    for at in [(0, 0), (8, 0), (16, 0), (24, 0), (32, 0)] {
        state
            .place(Coord::new(at.0, at.1))
            .expect("a legal spread ply");
    }
    let widths = |k: u64| {
        let mut first = None;
        let outcome = searcher(2, k)
            .search(&state, Stop::DepthTurns(1), &mut |info| {
                if first.is_none() {
                    first = Some(info.nodes);
                }
            })
            .expect("the search runs");
        (outcome.best, outcome.info.stages.safety_net_capped_rows)
    };
    let (off_best, off_capped) = widths(0);
    let (on_best, on_capped) = widths(8);
    assert_eq!(off_capped, 0, "the gate-off seat truncates nothing");
    assert_eq!(
        on_capped, 0,
        "and at depth_turns 1 the whole search IS the root turn, so an armed \
         cap must still truncate nothing"
    );
    assert_eq!(
        off_best, on_best,
        "so the move is the gate-off engine's, exactly"
    );
}

/// The cap never fires off a safety-net row: a position whose Tier T is
/// non-empty emits its Tier T whole however small K is.
#[test]
fn the_cap_never_fires_off_a_safety_net_row() {
    // Two own stones in one window put Tier T above empty at every node here.
    let mut state = GameState::new_game();
    for at in [(0, 0), (5, 0), (6, 0), (1, 0), (2, 0)] {
        state.place(Coord::new(at.0, at.1)).expect("a legal ply");
    }
    let outcome = searcher(2, 1)
        .search(&state, Stop::DepthTurns(2), &mut |_| {})
        .expect("the search runs");
    let stages = outcome.info.stages;
    assert!(
        stages.safety_net_capped_rows <= stages.batched_quiet_safety_net,
        "the capped rows are a subset of the safety-net rows: {} capped against \
         {} safety-net",
        stages.safety_net_capped_rows,
        stages.batched_quiet_safety_net
    );
    assert!(
        stages.batched + stages.filtered > stages.batched_quiet_safety_net,
        "and this position has rows of other kinds, which the cap left alone \
         however small K is"
    );
}

/// Determinism: the truncation boundary is a total order with no clock, no
/// thread and no hash iteration in it (CLAUDE.md rule 4, D-7).
#[test]
fn the_capped_search_is_reproducible_across_runs() {
    let run = || {
        let outcome = searcher(2, 8)
            .search(&GameState::new_game(), Stop::DepthTurns(2), &mut |_| {})
            .expect("the search runs");
        (
            outcome.best,
            outcome.info.nodes,
            outcome.info.stages.safety_net_capped_rows,
        )
    };
    assert_eq!(run(), run());
}

/// §6.3's STORE RULE, HALF ONE: a truncated node withholds its FAIL-LOW.
///
/// `Bound::Upper` says nothing did better, over a set the node never
/// exhausted. The counter is split by bound kind because one total cannot tell
/// a rule that wrongly stores `Exact` from one that wrongly stores `Upper` —
/// both leave a non-zero total, and the aggregate assertion this test replaces
/// survived each of them (REVIEW-impl MAJOR 1).
#[test]
fn a_truncated_fail_low_stores_no_transposition_record() {
    let stages = capped_search(3);
    assert!(
        stages.safety_net_capped_rows > 0,
        "the tree must contain truncated nodes for this to say anything"
    );
    assert!(
        stages.safety_net_upper_withheld > 0,
        "a rule that stored a truncated node's Upper bound would read 0 here, \
         and the table outlives the search, so that bound reaches a later root \
         turn (docs/experiments/wp15d_design.md §6.2)"
    );
}

/// §6.3's STORE RULE, HALF TWO: a truncated node withholds its EXACT score.
#[test]
fn a_truncated_exact_score_stores_no_transposition_record() {
    let stages = capped_search(3);
    assert!(
        stages.safety_net_exact_withheld > 0,
        "an Exact score over a set that was not exhausted is a claim the node \
         did not prove, and a rule that stored it would read 0 here"
    );
}

/// §6.3's STORE RULE, HALF THREE: the PROVED bound survives it. A rule that
/// withheld everything from a truncated node would throw away the fail-highs,
/// which a subset genuinely proves — a move it did search reached beta.
#[test]
fn a_truncated_fail_high_still_stores_its_lower_bound() {
    let stages = capped_search(3);
    let withheld = stages.safety_net_upper_withheld + stages.safety_net_exact_withheld;
    assert!(
        withheld < stages.safety_net_capped_rows,
        "{withheld} withheld against {} truncated rows: a rule that withheld \
         every bound would reach the row count",
        stages.safety_net_capped_rows
    );
}

/// §6.4's WARM-TABLE CLASS, INSTANTIATED. Every other test in this file builds
/// a fresh `Searcher` per search, which is exactly the blind spot §6.2 names:
/// the transposition table is KEPT across the searches of a game
/// (`search.rs`'s own doc, "successive searches in one game share what they
/// learned"), so a node truncated a turn down is the NEXT search's root turn.
/// That is the class §6.3's store rule exists for, and without this test it
/// ships never once instantiated (REVIEW-impl BLOCKING 2).
///
/// **What this pins, stated exactly.** That the class OCCURS on a warm table —
/// truncated nodes reach the store across successive searches of one game and
/// the rule fires there — and that a warm capped game stays reproducible. The
/// census the design's §6.4 describes, counting cutoffs taken on a record whose
/// storer was truncated, needs provenance the shipped `Record` does not carry;
/// that instrumentation is not added here and the shortfall is recorded rather
/// than papered over.
#[test]
fn the_store_rule_fires_across_the_searches_of_one_warm_game() {
    let play = || {
        // ONE searcher for the whole game: the table is warm from the second
        // search onward, as it is in play.
        let mut engine = searcher(2, 8);
        let mut state = GameState::new_game();
        let mut withheld = 0u64;
        let mut capped = 0u64;
        let mut line = Vec::new();
        for _ in 0..6 {
            if state.outcome().is_decided() {
                break;
            }
            let outcome = engine
                .search(&state, Stop::DepthTurns(2), &mut |_| {})
                .expect("the search runs");
            let stages = outcome.info.stages;
            withheld += stages.safety_net_upper_withheld + stages.safety_net_exact_withheld;
            capped += stages.safety_net_capped_rows;
            line.push(outcome.best);
            for at in [Some(outcome.best.first()), outcome.best.second()]
                .into_iter()
                .flatten()
            {
                state.place(at).expect("the engine's own turn is legal");
            }
        }
        (withheld, capped, line)
    };
    let (withheld, capped, line) = play();
    assert!(
        capped > 0,
        "the warm game must reach truncated nodes at all"
    );
    assert!(
        withheld > 0,
        "and the store rule must fire on them: {withheld} withheld over \
         {capped} truncated rows across a warm game"
    );
    assert_eq!(
        play(),
        (withheld, capped, line),
        "and a warm capped game is reproducible, table contents included (D-7)"
    );
}

/// THE TIE-BREAK THE CAP'S BOUNDARY RESTS ON (docs/decisions.md D-5, D-7;
/// docs/experiments/wp15d_design.md §2.5).
///
/// `delta_rank` sorts by `Reverse(Eval::delta)` with a STABLE sort over a ball
/// that `within_radius` returns ascending, so equal-scoring cells keep
/// ascending `(q, r)` order. That is what decides which of several tied cells
/// falls inside K and which is truncated away — so without it the cap's
/// boundary is whatever the sort implementation happens to do.
///
/// **The assertion is the ORDER, not agreement between two runs.** Two runs
/// agree under an unstable sort too: it is deterministic for a fixed input, so
/// run-to-run agreement is a property the defect PRESERVES, which
/// `docs/process.md`'s vacuous-criterion clause forbids as a criterion.
/// MEASURED before this test existed: `sort_by_key` → `sort_unstable_by_key`
/// survived every suite in the workspace.
#[test]
fn equal_scoring_safety_net_cells_are_emitted_in_ascending_coordinate_order() {
    use pistol_eval::Eval;
    use pistol_search::staged::{StagedRow, StagedSet, staged_candidates};

    // A spread position: stones eight apart, so no length-six window holds two
    // of anybody's and Tier F and Tier T are both empty — the row is the safety
    // net, the whole quiet ball. Spread rather than a single stone because the
    // discriminating fixture needs GROUPS of ties: around one stone every ball
    // cell scores the same, and an all-equal input is the one case an unstable
    // sort provably leaves alone. MEASURED — the single-stone fixture gave 36
    // cells all scoring 36, and the mutant survived it.
    let mut state = GameState::new_game();
    for at in [(0, 0), (8, 0), (16, 0), (24, 0), (32, 0)] {
        state
            .place(Coord::new(at.0, at.1))
            .expect("a legal spread ply");
    }
    let threats = common::threats_for(&state);
    // The eval is INCREMENTAL (`Eval`'s own contract: apply/undo per placed
    // stone), so it has to be walked onto this board before it can rank cells
    // on it. Without this the ranking is taken against an EMPTY board, where
    // every ball cell is symmetric and scores the same — which is a ranking
    // that discriminates nothing and a test that asserts nothing. The
    // `groups > 1` guard below is what makes that failure visible rather than
    // silent.
    let mut eval: Box<dyn Eval> =
        Box::new(pistol_eval::HandcraftedV0::new(common::committed_weights()));
    for (at, player) in state.board().stones() {
        eval.apply(at, player);
    }
    let mut out = StagedSet::default();
    let row = staged_candidates(
        &state,
        &threats,
        &mut *eval,
        false,
        // Radius 3: the ball must also stay clear of the insertion-sort
        // fallback, which is stable in practice on a short slice.
        common::staged_params_for_cap(3, 0),
        &mut out,
    );
    assert_eq!(row, StagedRow::Batched);
    assert!(out.used_quiet_safety_net, "this row IS the safety net");
    assert!(
        out.cells.len() > 20,
        "and it must stay above the insertion-sort fallback's reach: {} cells",
        out.cells.len()
    );

    let scored: Vec<(i32, Coord)> = out
        .cells
        .iter()
        .map(|&at| (eval.delta(at, state.to_move()), at))
        .collect();
    let ties = scored.windows(2).filter(|w| w[0].0 == w[1].0).count();
    let groups = 1 + scored.windows(2).filter(|w| w[0].0 != w[1].0).count();
    assert!(
        ties > 0 && groups > 1,
        "the fixture must contain SEVERAL groups of ties or it discriminates \
         nothing: {groups} group(s), {ties} adjacent tie(s)"
    );
    for pair in scored.windows(2) {
        let (before, after) = (pair[0], pair[1]);
        assert!(
            before.0 > after.0 || before.1 < after.1,
            "the ranking must fall in score, and where it does not the cells \
             must ascend: {before:?} then {after:?}"
        );
    }
}
