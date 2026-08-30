use pistol_cli::report::{
    TOTALS_MARKER, bestmove_line, error_line, id_line, info_line, score_token, totals_line,
};
use pistol_core::{Coord, Turn};
use pistol_engine::{EngineError, MATE, SearchInfo, SolverCallCounters, StageCounters};

/// A report with every field set to something recognisable.
fn info() -> SearchInfo {
    SearchInfo {
        depth_turns: 3,
        seldepth_turns: 4,
        nodes: 1234,
        search_nodes: 0,
        solver_nodes: 0,
        solver_refusals: 0,
        solver_calls: SolverCallCounters::default(),
        nps: 5678,
        time_ms: 90,
        pv: vec![
            Turn::Single(Coord::ORIGIN),
            Turn::pair(Coord::new(1, 0), Coord::new(2, 0)).expect("two distinct cells"),
        ],
        score: 42,
        hashfull_permille: 12,
        stages: StageCounters::default(),
    }
}

#[test]
fn info_line_states_every_field_in_one_fixed_order() {
    assert_eq!(
        info_line(&info()),
        "info depth_turns 3 seldepth 4 nodes 1234 nps 5678 time 90 hashfull 12 score cp 42 \
         pv 0,0 1,0/2,0"
    );
}

#[test]
fn the_totals_line_is_the_same_fields_behind_a_marker() {
    // It repeats a depth already reported with the whole search's cost, so a
    // driver has to be able to tell the two apart (docs/decisions.md D-80, D-88).
    let totals = totals_line(&info());
    assert_eq!(
        totals,
        "info totals depth_turns 3 seldepth 4 nodes 1234 nps 5678 time 90 hashfull 12 \
         score cp 42 pv 0,0 1,0/2,0"
    );
    assert_eq!(
        totals.replace(&format!(" {TOTALS_MARKER}"), ""),
        info_line(&info()),
        "the marker is the only difference"
    );
}

#[test]
fn score_spellings_are_cp_mate_and_negative_mate() {
    // A distance counts every turn from the root, both sides', so a win for the
    // side to move is an odd number and a loss an even one (docs/decisions.md
    // D-72). `-mate` is the one a driver must read to learn it is losing.
    assert_eq!(score_token(0), "cp 0");
    assert_eq!(score_token(42), "cp 42");
    assert_eq!(score_token(-16_000), "cp -16000");
    assert_eq!(score_token(MATE - 1), "mate 1");
    assert_eq!(score_token(MATE - 3), "mate 3");
    assert_eq!(score_token(-(MATE - 2)), "-mate 2");
    assert_eq!(score_token(-(MATE - 4)), "-mate 4");
}

#[test]
fn bestmove_line_carries_one_canonical_turn_token() {
    assert_eq!(
        bestmove_line(Turn::Single(Coord::new(5, 0))),
        "bestmove 5,0"
    );
    assert_eq!(
        bestmove_line(Turn::pair(Coord::new(5, 0), Coord::new(4, 0)).expect("two cells")),
        "bestmove 4,0/5,0",
        "a pair is written smaller cell first, whichever way round it was built"
    );
}

#[test]
fn an_error_line_names_the_error_and_stays_one_line() {
    assert_eq!(
        error_line(&EngineError::config(
            "search.tt_bytes",
            "must be a power of two, got 3"
        )),
        "error Config: `search.tt_bytes`: must be a power of two, got 3"
    );
    assert_eq!(
        error_line(&EngineError::BudgetMissing)
            .split_once(": ")
            .map(|(name, _)| name),
        Some("error BudgetMissing")
    );
    // A multi-line explanation is folded rather than trusted not to exist: two
    // lines for one refusal would desynchronize a driver.
    let folded = error_line(&EngineError::illegal_position("first\nsecond"));
    assert_eq!(folded, "error IllegalPosition: first; second");
    assert!(!folded.contains('\n'));
}

#[test]
fn an_id_line_is_prefixed_and_folded() {
    assert_eq!(id_line("name pistol"), "id name pistol");
    assert_eq!(id_line("config a\nconfig b"), "id config a; config b");
}

/// The solver field's print discipline (design wp18b §3): it appears
/// STRICTLY AFTER `nodes`, and only when nonzero — so a gate-off search's
/// line is byte-identical to the pre-wiring engine's, and the one
/// substring parser in the tree (`tools/sealbot`, which matches `"nodes "`)
/// reads the true `nodes` even on an ON seat's line.
#[test]
fn solver_nodes_prints_after_nodes_and_only_when_nonzero() {
    let mut with_solver = info();
    with_solver.search_nodes = 934;
    with_solver.solver_nodes = 300;
    let line = totals_line(&with_solver);
    let nodes_at = line.find("nodes 1234").expect("the nodes field prints");
    let search_at = line
        .find("search_nodes 934")
        .expect("a nonzero solver pair prints the search counter too");
    let solver_at = line
        .find("solver_nodes 300")
        .expect("a nonzero solver_nodes prints");
    assert!(
        nodes_at < search_at && search_at < solver_at,
        "the pair prints strictly after nodes, search first: {line}"
    );
    // And a zero solver_nodes prints NOTHING (the gate-off shape).
    let without = totals_line(&info());
    assert!(
        !without.contains("solver_nodes"),
        "a gate-off line carries no solver field: {without}"
    );
}

/// The call counters ride with the node pair and never without it: a gate-off
/// line carries none of the six fields, so its bytes are the pre-wiring
/// engine's (docs/decisions.md D-88's pinned order), and an ON seat's line
/// carries all six in a fixed order a field-name parser can read.
#[test]
fn the_solver_call_counters_print_with_the_node_pair_and_only_then() {
    let mut with_solver = info();
    with_solver.search_nodes = 934;
    with_solver.solver_nodes = 300;
    with_solver.solver_calls = SolverCallCounters {
        firings: 7,
        invocations: 13,
        proofs: 2,
        root_nodes: 41,
    };
    let line = totals_line(&with_solver);
    let mut at = line
        .find("solver_nodes 300")
        .expect("the node pair prints first");
    for expected in [
        "solver_firings 7",
        "solver_invocations 13",
        "solver_proofs 2",
        "solver_root_nodes 41",
    ] {
        let next = line
            .find(expected)
            .unwrap_or_else(|| panic!("`{expected}` prints: {line}"));
        assert!(
            at < next,
            "`{expected}` prints after the field before it: {line}"
        );
        at = next;
    }

    // The gate-off shape: nonzero counters with a zero `solver_nodes` cannot
    // happen (a firing that spends nothing still spends the call's own visit),
    // and the line must not invent a field for them if it ever did.
    let mut counters_only = info();
    counters_only.solver_calls = SolverCallCounters {
        firings: 7,
        invocations: 13,
        proofs: 2,
        root_nodes: 0,
    };
    let without = totals_line(&counters_only);
    assert!(
        !without.contains("solver_firings"),
        "the counters ride with the node pair, never alone: {without}"
    );
}
