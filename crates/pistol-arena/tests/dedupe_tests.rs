use pistol_arena::dedupe;
use pistol_arena::record::{Compute, End, GameRecord, GameResult};
use pistol_arena::score;
use pistol_core::{Coord, Symmetry, Turn};

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
fn identical_games_counted_once_in_effective_n() {
    // The self-match shape: both games of a pair are the same game move for
    // move, because two identical deterministic engines play the same moves
    // whichever seat they sit in. Distinct-n must see that.
    let moves = game_moves();
    let records = vec![
        record(0, GameResult::P1Win, true, moves.clone()),
        record(1, GameResult::P1Win, false, moves.clone()),
    ];
    let duplicates = dedupe::duplicates(&records);
    assert_eq!(
        duplicates,
        vec![None, Some(0)],
        "the second game duplicates the first"
    );
    assert_eq!(dedupe::distinct_count(&duplicates), 1);
    assert_eq!(
        records.len(),
        2,
        "and n is still two — both numbers are reported"
    );

    // The pair is nonetheless 1-1: the same moves with the seats swapped means
    // each engine won once, which is the finding this arrangement produces.
    let counted = score::tally(&records);
    assert_eq!(counted.pentanomial[2], 1, "a 1-1 pair");
    assert_eq!(counted.wins_a, 1);
    assert_eq!(counted.losses_a, 1);
}

#[test]
fn a_mirrored_game_is_the_same_game() {
    // The class docs/decisions.md D-137 says a move-list dedupe "provably
    // cannot see". Keyed on the position's canonical form instead, this would
    // ALSO merge two different games that reach the same stones — which is why
    // the key is over the ordered sequence.
    let moves = game_moves();
    let mirrored = pistol_core::symmetry::transform_sequence(&moves, Symmetry::ALL[7]);
    assert_ne!(mirrored, moves, "the mirror is a different move list");

    let records = vec![
        record(0, GameResult::P1Win, true, moves),
        record(1, GameResult::P1Win, true, mirrored),
    ];
    let duplicates = dedupe::duplicates(&records);
    assert_eq!(
        dedupe::distinct_count(&duplicates),
        1,
        "a game and its mirror image are one distinct game"
    );
}

#[test]
fn two_different_games_are_two_distinct_games() {
    // Without this the dedupe could merge everything and both tests above
    // would still pass.
    let one = game_moves();
    let mut other = one.clone();
    other[2] = pair((5, 0), (6, 0));
    let records = vec![
        record(0, GameResult::P1Win, true, one),
        record(1, GameResult::P1Win, true, other),
    ];
    assert_eq!(dedupe::distinct_count(&dedupe::duplicates(&records)), 2);
}
