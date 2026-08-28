mod common;

use common::assert_pinned;
use common::boards::{GOLDEN_BOARDS_FILE, golden_cases, parse_boards};
use common::sha256::sha256_hex;
use pistol_core::win::winning_run;
use pistol_core::{Axis, WIN_LEN, wins_at};

/// The SHA-256 of `tests/fixtures/golden_boards_v1.txt`.
///
/// Changing the fixture means changing this line, in the same commit, having
/// looked at what changed.
const GOLDEN_BOARDS_SHA256: &str =
    "86354bf079e5130e3d9ccb08bd679fb47e25820d1fb707932b367b0d2758b26d";

#[test]
fn golden_boards_fixture_matches_its_pinned_sha256() {
    assert_pinned(GOLDEN_BOARDS_FILE, GOLDEN_BOARDS_SHA256);
}

#[test]
fn sha256_matches_published_test_vectors() {
    let vectors: [(&[u8], &str); 3] = [
        (
            b"",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            b"abc",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        ),
    ];
    for (input, expected) in vectors {
        assert_eq!(sha256_hex(input), expected, "vector {input:?}");
    }
    // A million 'a': many blocks, and a length that does not fit in a byte.
    let long = vec![b'a'; 1_000_000];
    assert_eq!(
        sha256_hex(&long),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}

#[test]
fn golden_boards_match_their_recorded_verdict() {
    for case in golden_cases() {
        let board = case.board();
        let name = &case.name;
        assert_eq!(
            board.get(case.last),
            Some(case.last_player),
            "case `{name}`: the last stone is not on the board"
        );

        if case.expect_win {
            assert!(
                wins_at(&board, case.last),
                "case `{name}`: {} on {} must complete a run",
                case.last_player,
                case.last
            );
            let run = winning_run(&board, case.last).expect("a winning run");
            assert!(run.len >= WIN_LEN, "case `{name}`: run of {}", run.len);
            assert_eq!(run.player, case.last_player, "case `{name}`");

            // The last stone has to be the one that wins: without it, nothing on
            // the board completes a run. That is what rule 4 is about.
            let before = case.board_without_last();
            for (at, _) in before.stones() {
                assert!(
                    !wins_at(&before, at),
                    "case `{name}`: {at} already won before the last stone was played"
                );
            }
        } else {
            for (at, _) in board.stones() {
                assert!(
                    !wins_at(&board, at),
                    "case `{name}`: {at} completes a run, but the case says no-win"
                );
            }
        }
    }
}

#[test]
fn golden_boards_cover_both_verdicts_and_all_three_axes() {
    let cases = golden_cases();
    assert_eq!(cases.len(), 15, "the fixture lost or gained cases");

    let wins = cases.iter().filter(|case| case.expect_win).count();
    assert_eq!(wins, 9, "win cases");
    assert_eq!(cases.len() - wins, 6, "no-win cases");

    let mut axes: Vec<Axis> = cases
        .iter()
        .filter(|case| case.expect_win)
        .map(|case| {
            winning_run(&case.board(), case.last)
                .unwrap_or_else(|| panic!("case `{}` has no winning run", case.name))
                .axis
        })
        .collect();
    axes.sort();
    axes.dedup();
    assert_eq!(axes, Axis::ALL.to_vec(), "every axis needs a win case");

    let overlines = cases
        .iter()
        .filter(|case| case.expect_win)
        .filter(|case| {
            winning_run(&case.board(), case.last)
                .expect("a winning run")
                .len
                > WIN_LEN
        })
        .count();
    assert!(overlines >= 1, "no overline case");

    let p2_wins = cases
        .iter()
        .filter(|case| case.expect_win && case.last_player == pistol_core::Player::P2)
        .count();
    assert!(p2_wins >= 1, "every win case is p1's");
}

#[test]
#[should_panic(expected = "unknown directive")]
fn golden_loader_refuses_a_line_it_does_not_understand() {
    // A fixture that is quietly half-read reports a pass for cases nobody ran.
    parse_boards("case a\nexpect win\np1 0,0\nlast p1 0,0\nvariant exact-six\n");
}

#[test]
#[should_panic(expected = "has no `expect`")]
fn golden_loader_refuses_a_case_without_a_verdict() {
    parse_boards("case a\np1 0,0\nlast p1 0,0\n");
}

#[test]
#[should_panic(expected = "is not among that player's stones")]
fn golden_loader_refuses_a_last_stone_that_is_not_on_the_board() {
    parse_boards("case a\nexpect win\np1 0,0\nlast p1 1,0\n");
}
