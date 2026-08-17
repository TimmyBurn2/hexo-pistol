//! Where the static evaluation is allowed to answer.
//!
//! The eval scores a position; a position half way through a turn is one no
//! player will ever be handed, because the mover still owes a stone. So a
//! horizon that lands there is not a horizon, and `pvs.rs` says so by name
//! (`STATIC_EVAL_MID_TURN`, docs/decisions.md D-111).
//!
//! What is pinned is that no static evaluation is returned as a node's
//! ANSWER at phase 1 — deliberately narrower than "the eval is never consulted
//! at phase 1", which is false and which these tests would therefore be lying
//! about. `Eval::value` is read at phase-1 nodes in two places, and neither is
//! an answer: move ordering scores a hypothetical stone it takes straight back
//! off (D-76), and the transposition table fills a `static_eval` field nothing
//! in Stage 0 reads. D-111 carries the census.
//!
//! The invariant is a debug assertion inside the recursion rather than something
//! a test can read off a result, so what these tests do is *drive* it: reach as
//! many horizons as possible, from roots at as many different turn numbers as
//! possible, and let the assertion be the thing that fails. Because "nothing
//! panicked" is also what a search that did no work would report, each test
//! additionally counts the horizons it reached and refuses to pass on zero
//! (the positive-content argument docs/decisions.md D-90 makes for the
//! determinism gate).

mod common;

use common::{blob, line, position, quiet, searcher};
use pistol_core::{Axis, Coord, GameState, Player};
use pistol_search::score::{ScoreKind, classify};
use pistol_search::{Searcher, Stop};

/// The radius these tests search at.
///
/// One, and deliberately the narrowest the search accepts: what is under test
/// is which *phase* a horizon lands on, which is the ply arithmetic and is the
/// same at every radius, while the cost of a node is not. A wider radius would
/// buy no coverage of this invariant and would make the playout below a test
/// nobody runs (docs/decisions.md D-81's floor).
const RADIUS: u32 = 1;

/// Search `state` to every depth from one turn to `max_depth`, and report how
/// many of those searches ended on a static evaluation rather than on a mate.
///
/// A mate score never came from the eval — the win branch returns it before any
/// child node is visited — so it is the *non*-mate answers that prove a horizon
/// was reached and the eval was asked.
fn horizons_reached(searcher: &mut Searcher, state: &GameState, max_depth: u32) -> u32 {
    let mut reached = 0;
    for depth in 1..=max_depth {
        let outcome = searcher
            .search(state, Stop::DepthTurns(depth), &mut |_| {})
            .expect("an ongoing position at a turn boundary is searchable");
        // Not equality: iterative deepening stops early on a proven mate, so
        // a depth budget can honestly report a shallower completed depth
        // (`search.rs`). What it may never do is report a deeper one.
        assert!(
            outcome.info.depth_turns <= depth,
            "a search may stop short of the depth it was given, on a mate, but never overrun it: \
             asked {depth}, reported {}",
            outcome.info.depth_turns
        );
        if matches!(classify(outcome.info.score), ScoreKind::Eval(_)) {
            reached += 1;
        }
    }
    reached
}

#[test]
fn static_eval_never_answers_a_node_at_phase_one() {
    // Roots at every turn number a short game passes through, because the ply
    // budget is a sum over the turns ahead and turn 1 owes ONE stone where
    // every later turn owes two — so the depth-to-plies arithmetic is a
    // different sum from every one of these roots, and an off-by-one in it
    // lands the horizon mid-turn from some of them and not others.
    let mut state = GameState::new_game();
    let mut driver = searcher(RADIUS);
    let mut reached = 0;

    for _ in 0..6 {
        let mut probe = searcher(RADIUS);
        reached += horizons_reached(&mut probe, &state, 3);

        // Play on with a search-chosen turn rather than a scripted one: the
        // positions a search actually walks into are the ones its horizons will
        // land in.
        let outcome = driver
            .search(&state, Stop::DepthTurns(2), &mut |_| {})
            .expect("the playout stays at a turn boundary and undecided");
        state
            .make_turn(outcome.best)
            .expect("the search answers with a turn the rules accept");
        if state.outcome().is_decided() {
            break;
        }
    }

    assert!(
        reached >= 12,
        "the playout must actually reach horizons or it proves nothing about where they land, \
         reached {reached}"
    );
}

#[test]
fn static_eval_never_answers_a_node_at_phase_one_from_a_deep_root() {
    // The playout above starts at turn 1 and stays shallow. This one starts
    // where a real search does — a midgame root, a live threat on the board —
    // and goes deeper, so the recursion alternates between the two kinds of
    // child (the mover's second stone, and the opponent's reply) many times
    // before any horizon is reached.
    let p1 = line(Coord::ORIGIN, Axis::ConstR, 5);
    let p2 = blob(Coord::new(0, 3), 4);
    let state = position(&p1, &p2, Player::P2);

    let mut searcher = searcher(RADIUS);
    let reached = horizons_reached(&mut searcher, &state, 3);

    assert!(
        reached >= 1,
        "a root with a live threat must still reach a static horizon at some depth"
    );
}

#[test]
fn static_eval_never_answers_a_node_at_phase_one_in_a_quiet_position() {
    // Nothing in `quiet()` completes six inside these depths, so every one of
    // these searches ends on the eval and none of them short-circuits on a
    // mate — the case that exercises the horizon the most times per node.
    let state = quiet();
    let mut searcher = searcher(RADIUS);

    let reached = horizons_reached(&mut searcher, &state, 3);

    assert_eq!(
        reached, 3,
        "no line in this position is decided inside three turns, so every depth answers with the \
         evaluation"
    );
}
