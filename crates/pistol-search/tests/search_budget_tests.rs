//! Budgets, and the reports a search makes on the way.
//!
//! A node budget is one of the two instrument budgets (docs/decisions.md D-4),
//! so where it stops has to be a property of the search and not of the machine:
//! the same position and the same count stop on the same node, every run.

mod common;

use std::cell::RefCell;
use std::time::{Duration, Instant};

use common::{blob, line, position, quiet, searcher};
use pistol_core::{Axis, Coord};
use pistol_search::{MAX_DEPTH_TURNS, NODE_CHECK_INTERVAL, SearchInfo, Stop};

/// Every report the search made, in order.
fn collect(
    searcher: &mut pistol_search::Searcher,
    state: &pistol_core::GameState,
    stop: Stop,
) -> (pistol_search::SearchOutcome, Vec<SearchInfo>) {
    let reports = RefCell::new(Vec::new());
    let outcome = searcher
        .search(state, stop, &mut |info| {
            reports.borrow_mut().push(info.clone());
        })
        .expect("an ongoing position at a turn boundary");
    (outcome, reports.into_inner())
}

#[test]
fn search_respects_node_budget_exactly_at_check_granularity() {
    let state = quiet();
    let mut under_budget = searcher(1);
    let budget = 8 * NODE_CHECK_INTERVAL;

    let (outcome, reports) = collect(&mut under_budget, &state, Stop::Nodes(budget));

    let first = reports
        .first()
        .expect("the first iteration always completes");
    assert!(
        first.nodes < budget,
        "this position must not spend the whole budget on depth 1, spent {}",
        first.nodes
    );
    assert_eq!(
        outcome.info.nodes,
        budget.next_multiple_of(NODE_CHECK_INTERVAL),
        "the budget is tested every {NODE_CHECK_INTERVAL} nodes, so the search stops on the \
         first check at or past it"
    );

    // A budget that is not a multiple of the granularity stops at the next check
    // past it, and says so rather than pretending to have stopped on the number
    // it was given.
    let mut fresh = searcher(1);
    let ragged = budget + 1;
    let (outcome, _) = collect(&mut fresh, &state, Stop::Nodes(ragged));
    assert_eq!(
        outcome.info.nodes,
        ragged.next_multiple_of(NODE_CHECK_INTERVAL)
    );
}

#[test]
fn iterative_deepening_reports_each_depth() {
    let state = quiet();
    let mut searcher = searcher(1);

    let (outcome, reports) = collect(&mut searcher, &state, Stop::DepthTurns(3));

    assert_eq!(
        reports
            .iter()
            .map(|info| info.depth_turns)
            .collect::<Vec<_>>(),
        vec![1, 2, 3],
        "one report per completed depth, in order"
    );
    for info in &reports {
        assert!(!info.pv.is_empty(), "a completed depth has a line");
        assert!(
            info.pv.len() as u32 <= info.depth_turns,
            "a line of {} turns cannot come from a depth of {}",
            info.pv.len(),
            info.depth_turns
        );
        assert_eq!(
            info.seldepth_turns, info.depth_turns,
            "Stage 0 has no extension that passes the horizon"
        );
    }
    assert!(
        reports.windows(2).all(|pair| pair[0].nodes < pair[1].nodes),
        "each deeper iteration costs more nodes than the one before"
    );
    assert_eq!(
        outcome.best,
        reports.last().expect("three reports").pv[0],
        "the move played is the first turn of the deepest line"
    );
}

#[test]
fn search_stops_at_a_deadline() {
    // The play-mode budget. It is not reproducible and says so, which is why the
    // engine refuses it in instrument mode (docs/decisions.md D-22, D-73); what
    // is checked here is that the search honours it at all, and that the clock
    // path yields the same kind of answer as the reproducible ones.
    let state = quiet();
    let mut searcher = searcher(1);
    let started = Instant::now();
    let outcome = searcher
        .search(
            &state,
            Stop::Deadline(started + Duration::from_millis(100)),
            &mut |_| {},
        )
        .expect("an ongoing position at a turn boundary");

    assert!(!outcome.info.pv.is_empty(), "a search always answers");
    assert_eq!(outcome.best, outcome.info.pv[0]);
    // Generous, because the first iteration is never interrupted and this runs
    // in a debug build: the assertion is that the deadline bounds the search at
    // all, not that it is precise.
    assert!(
        started.elapsed() < Duration::from_secs(30),
        "the deadline did not stop the search"
    );
}

#[test]
fn only_the_wall_clock_budget_is_irreproducible() {
    assert!(Stop::DepthTurns(4).is_reproducible());
    assert!(Stop::Nodes(1000).is_reproducible());
    assert!(!Stop::Deadline(Instant::now()).is_reproducible());
}

#[test]
fn search_refuses_a_depth_past_the_horizon() {
    let state = quiet();
    let mut searcher = searcher(1);
    let error = searcher
        .search(&state, Stop::DepthTurns(MAX_DEPTH_TURNS + 1), &mut |_| {})
        .expect_err("a depth past the horizon is refused, never clamped");
    assert!(
        error.to_string().contains("horizon"),
        "the refusal must say what it refused: {error}"
    );
}

#[test]
fn search_refuses_a_root_in_the_middle_of_a_turn() {
    let mut state = quiet();
    state.place(Coord::new(3, 0)).expect("a legal first stone");
    let mut searcher = searcher(1);
    let error = searcher
        .search(&state, Stop::DepthTurns(1), &mut |_| {})
        .expect_err("the search starts at a turn boundary");
    assert!(error.to_string().contains("half played"), "{error}");
}

#[test]
fn search_refuses_a_decided_root() {
    // Black completes six; there is no move to search for.
    let black = line(Coord::ORIGIN, Axis::ConstR, 5);
    let mut white = vec![Coord::new(-1, 0)];
    white.extend(blob(Coord::new(0, 3), 5));
    let mut state = position(&black, &white, pistol_core::Color::Black);
    state.place(Coord::new(5, 0)).expect("the winning stone");

    let mut searcher = searcher(1);
    let error = searcher
        .search(&state, Stop::DepthTurns(1), &mut |_| {})
        .expect_err("a decided game has no move");
    assert!(error.to_string().contains("won on turn"), "{error}");
}
