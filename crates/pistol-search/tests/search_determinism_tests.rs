mod common;

use common::{blob, position, quiet, searcher, staged_searcher};
use pistol_core::{Coord, GameState, Player};
use pistol_search::{OrderingHeuristics, SearchOutcome, Stop};

/// WP-1.7's gates, all ON — stated here, in the test's own body, because no
/// code-side default exists (CLAUDE.md rule 1).
fn all_on() -> OrderingHeuristics {
    OrderingHeuristics {
        killers: true,
        history: true,
        countermove: true,
    }
}

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
    // exercises the abort path a depth budget never reaches.
    let state = quiet();
    let budget = Stop::Nodes(5 * pistol_search::NODE_CHECK_INTERVAL);

    let mut first = searcher(2);
    let one = first.search(&state, budget, &mut |_| {}).expect("ongoing");
    let mut second = searcher(2);
    let two = second.search(&state, budget, &mut |_| {}).expect("ongoing");
    assert_same(&one, &two, "two searchers under a node budget");
}

/// WP-1.7: the whole determinism law, replayed with every ordering-heuristic
/// gate ON. The heuristic tables are cross-search state — history and
/// countermove persist within a game — so this is the arm that catches a
/// table `clear` fails to empty and state that bleeds from one search into
/// the next through the new seam.
#[test]
fn search_with_heuristics_on_double_run_identical() {
    let state = quiet();
    let budget = Stop::DepthTurns(3);

    let mut first = staged_searcher(
        2,
        2,
        3,
        0,
        pistol_search::QTriggers::DefensiveAndOffensive,
        all_on(),
    );
    let one = first
        .search(&state, budget, &mut |_| {})
        .expect("an ongoing position");

    // A second searcher built the same way, in the same process.
    let mut second = staged_searcher(
        2,
        2,
        3,
        0,
        pistol_search::QTriggers::DefensiveAndOffensive,
        all_on(),
    );
    let two = second
        .search(&state, budget, &mut |_| {})
        .expect("an ongoing position");
    assert_same(&one, &two, "two searchers, heuristics on");

    // The same searcher, told to forget after a different game: a table the
    // clear fails to empty answers the second search differently.
    let mut third = staged_searcher(
        2,
        2,
        3,
        0,
        pistol_search::QTriggers::DefensiveAndOffensive,
        all_on(),
    );
    third
        .search(&distraction(), budget, &mut |_| {})
        .expect("an ongoing position");
    third.clear();
    let three = third
        .search(&state, budget, &mut |_| {})
        .expect("an ongoing position");
    assert_same(
        &one,
        &three,
        "after a different game and a clear, heuristics on",
    );
}

/// WP-1.7: within one game the heuristic tables persist across searches —
/// that is their design (the TT does the same), and it must be VISIBLE
/// rather than accidental: a second search from the same searcher may
/// legitimately differ from the first, because history aged and countermove
/// carried. What must hold is the reproducibility of the whole SEQUENCE:
/// two searchers playing the same two-search sequence agree exactly.
#[test]
fn the_heuristic_tables_persist_within_a_game_and_the_sequence_reproduces() {
    let state = quiet();
    let budget = Stop::DepthTurns(3);

    let play_two = |searcher: &mut pistol_search::Searcher| {
        let first = searcher
            .search(&state, budget, &mut |_| {})
            .expect("an ongoing position");
        let second = searcher
            .search(&state, budget, &mut |_| {})
            .expect("an ongoing position");
        (first, second)
    };

    let mut one = staged_searcher(
        2,
        2,
        3,
        0,
        pistol_search::QTriggers::DefensiveAndOffensive,
        all_on(),
    );
    let mut two = staged_searcher(
        2,
        2,
        3,
        0,
        pistol_search::QTriggers::DefensiveAndOffensive,
        all_on(),
    );
    let (a_first, a_second) = play_two(&mut one);
    let (b_first, b_second) = play_two(&mut two);
    assert_same(&a_first, &b_first, "first search of the sequence");
    assert_same(
        &a_second,
        &b_second,
        "second search of the sequence, tables warm",
    );
}
