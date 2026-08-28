mod common;

use common::{repo, scratch_file};
use pistol_cli::corpus::openings::{K_TURNS, OPENING_STONES};
use pistol_cli::corpus::record::Record;
use pistol_cli::corpus::verdict::Verdict;
use pistol_cli::corpus::{read, replay};
use pistol_core::{Coord, Turn};
use std::path::{Path, PathBuf};

/// The committed synthetic corpus.
fn synthetic() -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures/corpus_synthetic_v1.jsonl")
}

/// Read the committed synthetic corpus, or panic saying why not.
fn synthetic_records() -> Vec<Record> {
    let path = synthetic();
    let text = std::fs::read_to_string(&path).expect("the synthetic corpus is committed");
    read(&path, &text).expect("every line of the synthetic corpus is well formed")
}

/// A legal one-line corpus with these moves, as a record.
fn one_game(moves: &str, winner: i32) -> String {
    format!(
        "{{\"game_hash\":\"0123456789abcdef\",\"moves\":{moves},\"winner\":{winner},\
         \"source\":\"human\",\"elo\":[1200,1210]}}"
    )
}

#[test]
fn extractor_regroups_flat_moves_into_turns_correctly() {
    // Rule 3: one stone, then two, then two. Rule 4: a decisive game's last turn
    // may be one stone, and pistol-core is what insists that stone wins.
    let moves = [
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(3, 0),
        Coord::new(4, 0),
        Coord::new(5, 0),
    ];
    let turns = replay::group_turns(&moves).expect("this grouping is well formed");
    assert_eq!(
        turns.len(),
        4,
        "six stones group as 1, 2, 2, then a leftover 1"
    );
    assert_eq!(turns[0].turn, Turn::Single(Coord::new(0, 0)));
    assert_eq!(turns[0].move_index, 0);
    assert_eq!(
        turns[1].turn,
        Turn::pair(Coord::new(1, 0), Coord::new(2, 0)).unwrap()
    );
    assert_eq!(turns[1].move_index, 1);
    assert_eq!(
        turns[2].turn,
        Turn::pair(Coord::new(3, 0), Coord::new(4, 0)).unwrap()
    );
    assert_eq!(turns[2].move_index, 3);
    assert_eq!(turns[3].turn, Turn::Single(Coord::new(5, 0)));
    assert_eq!(turns[3].move_index, 5);

    // And the truncation is real in the committed fixture, not only in theory.
    let records = synthetic_records();
    let truncated = records
        .iter()
        .filter(|record| !record.moves.len().is_multiple_of(2))
        .count();
    assert!(
        truncated > 0 && truncated < records.len(),
        "the fixture must carry both a rule-4 truncated final turn and a second-stone win"
    );
}

#[test]
fn regrouping_keeps_the_recorded_order_even_though_the_turn_does_not() {
    // A turn is an unordered pair (docs/decisions.md D-49), so the canonical
    // spelling sorts its cells. The record's own order is kept beside it,
    // because the cross-check is the one thing that reads it.
    let moves = [Coord::new(0, 0), Coord::new(5, 0), Coord::new(1, 0)];
    let turns = replay::group_turns(&moves).unwrap();
    assert_eq!(
        turns[1].turn,
        Turn::Pair(Coord::new(1, 0), Coord::new(5, 0)),
        "the turn is spelled smaller cell first"
    );
    assert_eq!(
        turns[1].recorded_first,
        Coord::new(5, 0),
        "and the record still says which stone went down first"
    );
}

#[test]
fn every_game_in_the_synthetic_corpus_replays() {
    for record in &synthetic_records() {
        let replayed = replay::replay(record);
        assert_eq!(
            replayed.verdict,
            Verdict::Eligible,
            "game {} did not replay",
            record.game_hash
        );
    }
}

#[test]
fn extractor_reports_illegal_game_by_hash_and_move_index() {
    // (20,0) is twenty from the origin and eleven from every other stone, so no
    // ordering of turn 2 plays it (rule 5).
    let line = one_game("[[0,0],[20,0],[1,0]]", 1);
    let path = scratch_file("illegal", "corpus.jsonl", &line);
    let text = std::fs::read_to_string(&path).unwrap();
    let record = &read(&path, &text).unwrap()[0];
    let replayed = replay::replay(record);
    match &replayed.verdict {
        Verdict::IllegalTurn {
            turn_number,
            move_index,
            why,
            ..
        } => {
            assert_eq!(*turn_number, 2);
            assert_eq!(*move_index, 1, "the flat index of the turn's first stone");
            assert!(
                why.contains("outside the legal region"),
                "the reason is pistol-core's own: {why}"
            );
        }
        other => panic!("expected an illegal turn, got {other:?}"),
    }
    assert_eq!(replayed.verdict.name(), "illegal-turn");
    assert_eq!(replayed.verdict.move_index(), Some(1));
}

#[test]
fn extractor_reports_a_game_that_continues_after_a_win() {
    let mut records = synthetic_records();
    // A game whose final turn is a full pair — an odd stone count. Appending two
    // stones then leaves the winning turn untouched and adds a whole turn after
    // it. (Appending to a rule-4 truncated game instead would extend the
    // *winning* turn into a pair, which is a different thing entirely.)
    let record = records
        .iter_mut()
        .find(|record| !record.moves.len().is_multiple_of(2))
        .expect("the fixture carries a game won on a turn's second stone");
    // The stones need not be legal: rule 4 ends the game, so the position is
    // never asked about them.
    let continued_at = record.moves.len();
    record.moves.push(Coord::new(-30, 30));
    record.moves.push(Coord::new(-31, 31));
    let replayed = replay::replay(record);
    match replayed.verdict {
        Verdict::PostWinContinuation { move_index } => assert_eq!(
            move_index, continued_at,
            "the index of the first stone of the turn that should not exist"
        ),
        other => panic!("expected a post-win continuation, got {other:?}"),
    }
}

#[test]
fn extractor_reports_a_winner_the_replay_disagrees_with() {
    // The only check that catches a mis-grouped export: regrouping would assign
    // stones to the wrong sides and both replays would accept a legal game.
    let mut record = synthetic_records().remove(0);
    let recorded = record.winner;
    record.winner = recorded.opponent();
    match replay::replay(&record).verdict {
        Verdict::WinnerMismatch {
            replayed,
            recorded: said,
        } => {
            assert_eq!(replayed, recorded);
            assert_eq!(said, recorded.opponent());
        }
        other => panic!("expected a winner mismatch, got {other:?}"),
    }
}

#[test]
fn extractor_reports_a_game_that_never_finishes() {
    let mut record = synthetic_records().remove(0);
    record.moves.pop();
    assert_eq!(replay::replay(&record).verdict, Verdict::NotDecided);
}

#[test]
fn the_cross_check_counts_a_turn_whose_recorded_order_needs_rescuing() {
    // Turn 2 is recorded far stone first: (-16,0) is sixteen from the origin, so
    // in recorded order it is refused, while (-8,0) is exactly LEGAL_RADIUS away
    // and the pair plays the other way round. A count that no fixture can move
    // is not a measurement (docs/decisions.md D-141).
    let line = "{\"game_hash\":\"8c92e9c3ef3cb0d7\",\"moves\":[[0,0],[-16,0],[-8,0],[1,0],[-2,2],\
        [0,2],[-4,2],[2,0],[-4,4],[-2,4],[0,4],[3,0],[2,2],[-6,2],[-6,4],[4,0],[-6,6],[-4,6],\
        [-2,6],[5,0]],\"winner\":1,\"source\":\"human\",\"elo\":[1200,1210]}";
    let replayed = replayed_from(line);
    assert_eq!(
        replayed.verdict,
        Verdict::Eligible,
        "the game itself is legal"
    );
    assert_eq!(replayed.order_rescued, 1);
    assert_eq!(replayed.stone_after_win, 0);
}

#[test]
fn the_cross_check_counts_a_stone_recorded_after_the_winning_one() {
    // The record's first stone of the last turn completes the line, so in
    // recorded order the second stone follows a decided game — while the same
    // pair played the other way round is an ordinary turn that wins on its
    // second stone. This is rule 4 meeting a client that submits both at once.
    let line = "{\"game_hash\":\"4044f4912eb52159\",\"moves\":[[0,0],[-2,2],[0,2],[1,0],[-4,2],\
        [-4,4],[-2,4],[2,0],[0,4],[2,2],[-6,2],[3,0],[-6,4],[-6,6],[-4,6],[4,0],[-2,6],[0,6],\
        [2,4],[5,0],[0,8]],\"winner\":1,\"source\":\"human\",\"elo\":[1200,1210]}";
    let replayed = replayed_from(line);
    assert_eq!(
        replayed.verdict,
        Verdict::Eligible,
        "the turn is legal as a pair"
    );
    assert_eq!(replayed.stone_after_win, 1);
    assert_eq!(replayed.order_rescued, 0);
}

#[test]
fn the_synthetic_corpus_needs_no_rescuing_and_has_no_stone_after_a_win() {
    // The counts are zero here, which is only meaningful because the two tests
    // above show them moving.
    for record in &synthetic_records() {
        let replayed = replay::replay(record);
        assert_eq!(replayed.order_rescued, 0, "game {}", record.game_hash);
        assert_eq!(replayed.stone_after_win, 0, "game {}", record.game_hash);
    }
}

#[test]
fn an_opening_is_counted_in_turns_not_in_plies() {
    // K is four TURNS. An implementation reading it as four plies would emit
    // four-stone positions and pass anything that only counted entries.
    assert_eq!(OPENING_STONES, 2 * K_TURNS - 1);
    assert_eq!(OPENING_STONES, 7);
    let record = &synthetic_records()[0];
    let turns = replay::replay(record).turns;
    let state = replay::position_after(&turns, K_TURNS);
    assert_eq!(state.board().stones().count(), OPENING_STONES);
    assert_eq!(
        state.turn(),
        K_TURNS as u32 + 1,
        "four turns played, the fifth to come"
    );
}
/// Read one line and replay it.
fn replayed_from(line: &str) -> pistol_cli::corpus::verdict::Replayed {
    let path = scratch_file("special", "corpus.jsonl", line);
    let text = std::fs::read_to_string(&path).unwrap();
    let records = read(Path::new(&path), &text).expect("a well-formed line");
    replay::replay(&records[0])
}
