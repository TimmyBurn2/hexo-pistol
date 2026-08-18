//! Counting a run: distinct games, pair buckets, and which verdict outranks
//! which.
//!
//! These are over hand-built records rather than over played games, because the
//! shapes that matter — a mirrored duplicate, an all-capped run, a forfeit
//! beside a decisive result — are cheap to construct and expensive to provoke
//! through two subprocesses.

use pistol_arena::config::SprtSection;
use pistol_arena::record::{Compute, End, ForfeitReason, GameRecord, GameResult};
use pistol_arena::score;
use pistol_arena::sprt::Verdict;
use pistol_core::{Coord, Turn};

fn cell(q: i16, r: i16) -> Coord {
    Coord::new(q, r)
}

fn pair(a: (i16, i16), b: (i16, i16)) -> Turn {
    Turn::pair(cell(a.0, a.1), cell(b.0, b.1)).expect("two distinct cells")
}

/// A short legal-shaped game: one stone, then pairs.
fn game_moves() -> Vec<Turn> {
    vec![
        Turn::single(cell(0, 0)),
        pair((1, 0), (2, 0)),
        pair((0, 1), (0, 2)),
    ]
}

fn record(index: usize, result: GameResult, a_is_p1: bool, moves: Vec<Turn>) -> GameRecord {
    GameRecord {
        index,
        opening: index / 2,
        a_is_p1,
        result,
        end: End::Normal,
        forfeit_by: None,
        refusal: None,
        moves,
        compute: [Compute::default(); 2],
    }
}

#[test]
fn capped_game_recorded_distinct_from_wins() {
    let moves = game_moves();
    let records = vec![
        record(0, GameResult::Capped, true, moves.clone()),
        record(1, GameResult::Capped, false, moves.clone()),
    ];
    let counted = score::tally(&records);
    assert_eq!(counted.capped, 2, "capped games are counted as capped");
    assert_eq!(counted.wins_a, 0, "and not as wins");
    assert_eq!(counted.losses_a, 0, "and not as losses");
    assert_eq!(counted.decided, 0, "a capped game is not decided");
    assert_eq!(
        counted.first_player_wins, 0,
        "and has no first-player winner to count"
    );
    assert!((counted.capped_fraction() - 1.0).abs() < f64::EPSILON);
    // Two capped games score a half each, which is the middle pair bucket.
    assert_eq!(counted.pentanomial[2], 1);
}

#[test]
fn the_first_player_rate_is_over_decided_games_only() {
    // A capped game has no first-player winner, so putting it in the
    // denominator would report a rate nobody measured.
    let moves = game_moves();
    let records = vec![
        record(0, GameResult::P1Win, true, moves.clone()),
        record(1, GameResult::P1Win, false, moves.clone()),
        record(2, GameResult::Capped, true, moves.clone()),
        record(3, GameResult::Capped, false, moves),
    ];
    let counted = score::tally(&records);
    assert_eq!(counted.decided, 2);
    assert_eq!(
        counted.first_player_wins, 2,
        "both decided games went to the first seat"
    );
    assert_eq!(counted.n, 4, "over four games");
}

#[test]
fn pair_buckets_are_indexed_by_score_not_by_a_chess_word() {
    // Bucket 2 is a pair scoring one point. That is a 1-1 split of two DECISIVE
    // games as often as it is two capped games, which is why the report spells
    // these p0..p4 rather than borrowing `dd`.
    let moves = game_moves();
    let cases = [
        (GameResult::P2Win, GameResult::P1Win, 0usize), // A lost both
        (GameResult::P1Win, GameResult::P1Win, 2),      // A won one, lost one
        (GameResult::P1Win, GameResult::P2Win, 4),      // A won both
    ];
    for (first, second, expected) in cases {
        let records = vec![
            record(0, first, true, moves.clone()),
            record(1, second, false, moves.clone()),
        ];
        assert_eq!(
            score::pair_buckets(&records),
            vec![expected],
            "{first:?} then {second:?}"
        );
    }
}

fn sprt() -> SprtSection {
    SprtSection {
        elo0: 0.0,
        elo1: 4.0,
        alpha: 0.05,
        beta: 0.05,
    }
}

#[test]
fn a_forfeit_invalidates_the_verdict_rather_than_scoring_it() {
    // A forfeit means one of the two configurations is broken, and rule 6's
    // judge may not turn a correctness bug into an Elo number. The token itself
    // has to say so: a `verdict h1` line with a `forfeits 1` count on a
    // neighbouring line reads green to anything that greps for the verdict.
    //
    // The sample is two pairs with DIFFERENT outcomes — a sweep and a split —
    // because a sample where every pair scored alike is degenerate for its own
    // reason and would mask what this test is about.
    let moves = game_moves();
    let mut records = vec![
        record(0, GameResult::P1Win, true, moves.clone()),
        record(1, GameResult::P2Win, false, moves.clone()),
        record(2, GameResult::P1Win, true, moves.clone()),
        record(3, GameResult::P1Win, false, moves.clone()),
    ];
    assert_eq!(
        score::pair_buckets(&records),
        vec![4, 2],
        "a sweep and a split"
    );
    assert_eq!(
        score::verdict(&records, &sprt()),
        Verdict::InconclusiveAtGameCap,
        "without a forfeit this sample is well formed and simply has not decided"
    );
    records[3].end = End::Forfeit(ForfeitReason::IllegalTurn);
    records[3].forfeit_by = Some(1);
    assert_eq!(
        score::verdict(&records, &sprt()),
        Verdict::InvalidForfeit,
        "and one forfeit anywhere outranks whatever the sample said"
    );
}

#[test]
fn an_all_capped_run_is_degenerate_rather_than_decided() {
    // Every pair scores the same, so no likelihood ratio exists. This is what
    // stops a run that measured nothing from reporting `h0`.
    let moves = game_moves();
    let records: Vec<GameRecord> = (0..8)
        .map(|index| record(index, GameResult::Capped, index % 2 == 0, moves.clone()))
        .collect();
    assert_eq!(
        score::verdict(&records, &sprt()),
        Verdict::InconclusiveDegenerate
    );
    assert_eq!(score::first_crossing_pairs(&records, &sprt()), None);
}

#[test]
fn the_stop_fires_only_at_a_pair_boundary() {
    // A crossing found at an odd prefix would split a pair: one seat's game in
    // and the other's out, with the truncation point correlated with the last
    // included result. `first_crossing_pairs` counts PAIRS, so whatever it
    // returns describes whole pairs by construction.
    //
    // Two thirds of pairs are a sweep for A and one third a split, which is a
    // non-degenerate sample strong enough to cross.
    let moves = game_moves();
    let mut records: Vec<GameRecord> = Vec::new();
    for index in 0..400usize {
        let sweep = (index / 2) % 3 != 2;
        let a_is_p1 = index.is_multiple_of(2);
        // A wins its game when the pair is a sweep, or when it holds the first
        // seat in a split.
        let a_wins = sweep || a_is_p1;
        let result = if a_wins == a_is_p1 {
            GameResult::P1Win
        } else {
            GameResult::P2Win
        };
        records.push(record(index, result, a_is_p1, moves.clone()));
    }
    let buckets = score::pair_buckets(&records);
    assert!(
        buckets.contains(&4) && buckets.contains(&2),
        "the construction produced both sweeps and splits: {:?}",
        &buckets[..6]
    );

    let crossed = score::first_crossing_pairs(&records, &sprt())
        .expect("a sample this lopsided crosses a boundary");
    assert!(crossed >= 1 && crossed <= records.len() / 2);

    // The stop is the SMALLEST crossing, so no shorter prefix crosses. That is
    // what makes it a function of the results and not of the order they
    // finished in.
    for shorter in 1..crossed {
        assert_eq!(
            score::first_crossing_pairs(&records[..shorter * 2], &sprt()),
            None,
            "prefix of {shorter} pairs must not have crossed"
        );
    }
    // And the kept games are exactly whole pairs.
    assert_eq!(score::pair_buckets(&records[..crossed * 2]).len(), crossed);
}

#[test]
fn the_clean_verdict_drops_whole_pairs_not_single_games() {
    // REVIEW-impl's finding, pinned. Filtering individual games out of a flat
    // list shifts every later game by one, so after the first forfeit every
    // pair would be built from one game of opening i and one of opening i+1,
    // and the trailing game would vanish unmentioned. The clean verdict is the
    // line that exists to keep a forfeited run honest, so a sample nobody
    // computed is the worst possible thing to put behind it.
    let moves = game_moves();
    let mut records: Vec<GameRecord> = (0..6)
        .map(|index| {
            let mut game = record(
                index,
                GameResult::P1Win,
                index.is_multiple_of(2),
                moves.clone(),
            );
            game.opening = index / 2;
            game
        })
        .collect();
    // One forfeit, in the middle pair.
    records[2].end = End::Forfeit(ForfeitReason::IllegalTurn);
    records[2].forfeit_by = Some(1);

    let kept = score::pairs_without_forfeits(&records);
    assert_eq!(kept.len(), 4, "one pair went, not one game");
    assert!(
        kept.iter().all(|game| game.opening != 1),
        "and it was the whole of opening 1: {:?}",
        kept.iter().map(|game| game.opening).collect::<Vec<usize>>()
    );
    // Every surviving pair is still two games of ONE opening, which is the
    // property a game-wise filter destroys.
    for pair in kept.chunks_exact(2) {
        assert_eq!(
            pair[0].opening, pair[1].opening,
            "a pair must not straddle two openings: {} and {}",
            pair[0].opening, pair[1].opening
        );
    }
    // And nothing is dropped when nothing forfeited.
    records[2].end = End::Normal;
    records[2].forfeit_by = None;
    assert_eq!(score::pairs_without_forfeits(&records).len(), 6);
}
