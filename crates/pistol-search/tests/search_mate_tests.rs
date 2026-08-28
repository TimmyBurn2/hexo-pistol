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

#[test]
fn equal_scoring_turns_are_separated_by_the_coordinate_tie_break() {
    // P1 holds five in a row open at BOTH ends, so two distinct cells complete
    // six and both saturate the ordering score at the top of the eval band. The
    // scores are equal, so nothing but the tie-break can choose between them —
    // and the tie-break is the ascending `(q, r)` order the candidates arrive
    // in, held there by a STABLE sort (docs/decisions.md D-5, D-7).
    //
    // This is the only place the stability is observable from outside the
    // crate: swapping `sort_by_key` for `sort_unstable_by_key` leaves every
    // other test green, because pdqsort is deterministic too and the
    // cross-process gate compares two runs of one build rather than the order
    // itself.
    let p1 = line(Coord::ORIGIN, Axis::ConstR, 5);
    let p2 = blob(Coord::new(0, 3), 6);
    let state = position(&p1, &p2, Player::P1);

    let low = Coord::new(-1, 0);
    let high = Coord::new(5, 0);
    let mut searcher = searcher(2);
    let outcome = searcher
        .search(&state, Stop::DepthTurns(1), &mut |_| {})
        .expect("an ongoing position at a turn boundary");

    assert_eq!(
        outcome.best,
        Turn::Single(low),
        "both {low} and {high} complete six for the same score, so the smaller cell wins the tie"
    );
}

#[test]
fn a_retained_table_reports_the_same_mate_from_a_new_root() {
    // A mate score is a distance, so it means something different at every
    // node, and the table re-bases one on the way in and out. That ARITHMETIC
    // is pinned in `tt_tests`, which stores at one distance and probes at
    // another and so fails if the two directions are transposed. What is not
    // pinned there is the end of it a game actually exercises: a table filled
    // by a search at ONE root and then read by a search at ANOTHER, which is
    // what happens between two moves of a game, because the engine clears the
    // table on `newgame` and not between moves (docs/decisions.md D-72, D-98).
    //
    // Within one search the question cannot even arise — the turn number is
    // fixed by the stone count, so every probe of a given key happens at the
    // same distance from the root and the two conversions cancel.
    //
    // This test does not catch the arithmetic being wrong; `tt_tests` does. It
    // catches the weaker and otherwise untested claim in the last assertion: a
    // retained table changes what a search costs and not what it answers.
    //
    // The radius is 1 because the subject here is the agreement between a warm
    // table and a cold one at the SAME policy, which a narrow policy pins as
    // well as a wide one and two orders of magnitude faster (searching the
    // second root at radius 2 costs three minutes). That the mate is a game
    // fact rather than an artefact of a narrow search is a different claim, and
    // `search_finds_forced_mate_in_2_turns` pins it above at a wider one
    // (docs/decisions.md D-91 makes the same split).
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
    let mut state = position(&p1, &p2, Player::P1);

    // Root A: the double threat, seen as a mate in three turns.
    let mut warm = searcher(1);
    let first = warm
        .search(&state, Stop::DepthTurns(3), &mut |_| {})
        .expect("an ongoing position at a turn boundary");
    assert_eq!(classify(first.info.score), ScoreKind::MateIn(3));

    // Root B is one turn further in, so every node under it was a node under
    // root A and the table is full of scores anchored at A.
    state
        .make_turn(first.best)
        .unwrap_or_else(|error| panic!("the search's own turn must be legal: {error}"));
    let warm_second = warm
        .search(&state, Stop::DepthTurns(2), &mut |_| {})
        .expect("still ongoing: p1 wins on its next turn, not this one");

    // The same root, reached by a search that has never seen root A.
    let mut cold = searcher(1);
    let cold_second = cold
        .search(&state, Stop::DepthTurns(2), &mut |_| {})
        .expect("an ongoing position at a turn boundary");

    assert_eq!(
        classify(warm_second.info.score),
        classify(cold_second.info.score),
        "a re-based mate distance is the distance from THIS root, not the one it was stored at"
    );
    assert_eq!(
        classify(warm_second.info.score),
        ScoreKind::MatedIn(2),
        "p2 is to move and loses on p1's next turn, which is the second of this line"
    );
    assert_eq!(
        warm_second.best, cold_second.best,
        "a retained table changes what a search costs, never what it answers"
    );
}
