//! What the search is for: seeing a win, seeing a loss coming, and counting the
//! distance in turns.
//!
//! Every position here is built from a move list, so it is one a legal game
//! could have reached (docs/decisions.md D-6, D-42), and every candidate radius
//! is stated in the test body rather than inherited from a config file
//! (CLAUDE.md rule 1).

mod common;

use common::{blob, line, position, searcher};
use pistol_core::{Axis, Coord, Player, Turn};
use pistol_search::Stop;
use pistol_search::score::{ScoreKind, classify};

/// Cells a turn puts stones on.
fn cells(turn: Turn) -> Vec<Coord> {
    match turn {
        Turn::Single(at) => vec![at],
        Turn::Pair(first, second) => vec![first, second],
    }
}

#[test]
fn search_finds_mate_in_1_turn() {
    // P1 holds five in a row with one end blocked, so exactly one stone wins.
    // Rule 4 ends the turn on it: the winning turn is a single stone, not a
    // pair, and the second stone is never played.
    let p1 = line(Coord::ORIGIN, Axis::ConstR, 5);
    let mut p2 = vec![Coord::new(-1, 0)];
    p2.extend(blob(Coord::new(0, 3), 5));
    let state = position(&p1, &p2, Player::P1);

    let mut searcher = searcher(2);
    let outcome = searcher
        .search(&state, Stop::DepthTurns(1), &mut |_| {})
        .expect("an ongoing position at a turn boundary");

    assert_eq!(
        outcome.best,
        Turn::Single(Coord::new(5, 0)),
        "the one stone that completes six ends the turn (rule 4)"
    );
    assert_eq!(
        classify(outcome.info.score),
        ScoreKind::MateIn(1),
        "a win with a stone placed this turn is a mate in one turn"
    );
    assert_eq!(outcome.info.pv, vec![outcome.best]);
    assert_eq!(
        outcome.info.seldepth_turns, 1,
        "the winning line reached the turn it wins on, even though no child node \
         was visited to record it"
    );
}

#[test]
fn search_finds_mate_in_1_turn_completed_by_the_second_stone() {
    // Four in a row with one end blocked: no single stone wins, but the two
    // stones of one turn do. The turn is an ordinary pair, and the win still
    // completes on the turn the search is standing on.
    let mut p1 = line(Coord::ORIGIN, Axis::ConstR, 4);
    p1.push(Coord::new(0, 3));
    let mut p2 = vec![Coord::new(-1, 0)];
    p2.extend(blob(Coord::new(-1, 2), 5));
    let state = position(&p1, &p2, Player::P1);

    let mut searcher = searcher(2);
    let outcome = searcher
        .search(&state, Stop::DepthTurns(1), &mut |_| {})
        .expect("an ongoing position at a turn boundary");

    assert_eq!(
        outcome.best,
        Turn::Pair(Coord::new(4, 0), Coord::new(5, 0)),
        "both stones of the turn are needed, so the turn is a pair"
    );
    assert_eq!(classify(outcome.info.score), ScoreKind::MateIn(1));
}

#[test]
fn search_blocks_opponent_mate_in_1() {
    // P2 holds five in a row with one end blocked. P1 is to move and must
    // take the other end; anything else loses on P2's turn, which is two
    // turns from the root and so scores as being mated in two.
    let mut p1 = vec![Coord::ORIGIN, Coord::new(-1, 1)];
    p1.extend(blob(Coord::new(0, 3), 3));
    let mut p2 = line(Coord::new(0, 1), Axis::ConstR, 5);
    p2.push(Coord::new(-2, 2));
    let state = position(&p1, &p2, Player::P1);

    let mut searcher = searcher(1);
    let outcome = searcher
        .search(&state, Stop::DepthTurns(2), &mut |_| {})
        .expect("an ongoing position at a turn boundary");

    assert!(
        cells(outcome.best).contains(&Coord::new(5, 1)),
        "p1 must take p2's only winning cell, played {}",
        outcome.best
    );
    assert!(
        !matches!(classify(outcome.info.score), ScoreKind::MatedIn(_)),
        "the block saves the game, so the score is not a loss: {}",
        outcome.info.score
    );
}

#[test]
fn search_finds_forced_mate_in_2_turns() {
    // P1 holds two threes that share a stone. One turn extends both to a live
    // four, and P2's two stones cannot answer two live fours: killing one
    // costs both of them. So P1 wins on its second turn from here, which is
    // the third turn of the line and scores as a mate in three turns
    // (docs/decisions.md D-72).
    let p1 = vec![
        Coord::ORIGIN,
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(0, 1),
        Coord::new(0, 2),
    ];
    let p2 = vec![
        Coord::new(-1, 1),
        Coord::new(-2, 2),
        Coord::new(-1, 3),
        Coord::new(-3, 3),
        Coord::new(-2, 4),
        Coord::new(-3, 5),
    ];
    let state = position(&p1, &p2, Player::P1);

    let mut searcher = searcher(1);
    let outcome = searcher
        .search(&state, Stop::DepthTurns(3), &mut |_| {})
        .expect("an ongoing position at a turn boundary");

    assert_eq!(
        classify(outcome.info.score),
        ScoreKind::MateIn(3),
        "p1 wins on the third turn of the line, which is its second"
    );
    assert!(
        matches!(outcome.best, Turn::Pair(_, _)),
        "the double threat takes both stones of the turn"
    );
    assert_eq!(outcome.info.pv.first(), Some(&outcome.best));
}
