//! The determinism law, in one process (CLAUDE.md rule 4, docs/decisions.md
//! D-7).
//!
//! The cross-process half of the gate belongs to the CLI and compares two runs
//! over sha-pinned fixtures. This half catches what that one cannot: state that
//! bleeds from one search into the next inside a single process — a table that
//! was not cleared, a counter that was not reset, an ordering that depends on
//! what the previous search left behind.
//!
//! Time and nodes-per-second are deliberately not compared: they measure the
//! machine, not the search.

mod common;

use common::{blob, position, quiet, searcher};
use pistol_core::{Coord, GameState, Player};
use pistol_search::{SearchOutcome, Stop};

/// Another position, to be searched in between so that anything carried over
/// from it would show up.
fn distraction() -> GameState {
    let p1 = vec![Coord::ORIGIN, Coord::new(0, 1), Coord::new(2, 1)];
    let p2 = blob(Coord::new(1, -1), 4);
    position(&p1, &p2, Player::P1)
}

fn assert_same(left: &SearchOutcome, right: &SearchOutcome, what: &str) {
    assert_eq!(left.best, right.best, "{what}: different move");
    assert_eq!(
        left.info.nodes, right.info.nodes,
        "{what}: different node count"
    );
    assert_eq!(left.info.pv, right.info.pv, "{what}: different line");
    assert_eq!(left.info.score, right.info.score, "{what}: different score");
    assert_eq!(
        left.info.depth_turns, right.info.depth_turns,
        "{what}: different depth"
    );
    assert_eq!(
        left.info.hashfull_permille, right.info.hashfull_permille,
        "{what}: different table occupancy"
    );
}

#[test]
fn search_fixed_depth_double_run_identical() {
    let state = quiet();
    let budget = Stop::DepthTurns(3);

    let mut first = searcher(1);
    let one = first
        .search(&state, budget, &mut |_| {})
        .expect("an ongoing position");

    // A second search built the same way, in the same process.
    let mut second = searcher(1);
    let two = second
        .search(&state, budget, &mut |_| {})
        .expect("an ongoing position");
    assert_same(&one, &two, "two searchers");

    // The same searcher, told to forget: this is what a new game does, and it
    // has to leave no trace of the game before it.
    let mut third = searcher(1);
    third
        .search(&distraction(), budget, &mut |_| {})
        .expect("an ongoing position");
    third.clear();
    let three = third
        .search(&state, budget, &mut |_| {})
        .expect("an ongoing position");
    assert_same(&one, &three, "after a different game and a clear");
}

#[test]
fn search_node_budget_double_run_identical() {
    // The other instrument budget. A node budget stops mid-iteration, so it
    // exercises the abort path that a depth budget never reaches.
    let state = quiet();
    let budget = Stop::Nodes(5 * pistol_search::NODE_CHECK_INTERVAL);

    let mut first = searcher(2);
    let one = first.search(&state, budget, &mut |_| {}).expect("ongoing");
    let mut second = searcher(2);
    let two = second.search(&state, budget, &mut |_| {}).expect("ongoing");
    assert_same(&one, &two, "two searchers under a node budget");
}
