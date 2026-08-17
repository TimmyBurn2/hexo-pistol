//! Perft: pair-move generation against an independently written brute-force
//! reference (CLAUDE.md rule 7, docs/decisions.md D-12).
//!
//! Every comparison here is exact equality of counts, and at the first level of
//! exact equality of the turn **sets** — which catches a generator that emits
//! the right number of the wrong turns, something a count alone cannot see.
//!
//! The reference is not a second copy of the generator: it sweeps a bounding
//! box, asks rule 5 of one cell at a time by measuring against every stone,
//! enumerates ordered placements and dedupes them, and keeps its own game state
//! (see `common/bruteforce.rs`). Two implementations that agree on a hundred
//! thousand turns per position agree about the rules.
//!
//! # Depth
//!
//! A midgame position has of the order of 10^5 turns, so a second full level is
//! 10^9 to 10^10 and is not enumerable in a test. Depth 2 is checked in full
//! where it is reachable — from the initial position, whose first turn is one
//! stone, and at a decided position, which has none — and elsewhere by a
//! **divide**: the turns after a deterministic sample of first turns, compared
//! set for set against the same reference. The fixture header records the
//! measured branching factors this rests on.

mod common;

use common::assert_pinned;
use common::bruteforce::{RefGame, RefTurn};
use common::perft_positions::{PERFT_POSITIONS_FILE, PerftCase, parse_cases, perft_cases};
use pistol_core::{Coord, GameState, Turn, generate_turns, perft};

/// The SHA-256 of `tests/fixtures/perft_positions_v1.txt`.
///
/// Changing the fixture means changing this line, in the same commit, having
/// looked at what changed.
const PERFT_POSITIONS_SHA256: &str =
    "3ea61678535a98ce30197c368e8ea36896671c2a679620f885203f7f1ace5f65";

/// How many first turns the default run divides on, per case. One level of a
/// midgame position is a second or two of reference time, and the wider sweep
/// below is what the perft gate runs in release.
const DEFAULT_DIVIDE_SAMPLE: usize = 1;

/// How many the release-mode gate divides on. Sized by measurement: the whole
/// release run stays inside a few seconds, which is what a gate can spend.
const WIDE_DIVIDE_SAMPLE: usize = 25;

#[test]
fn perft_positions_fixture_matches_its_pinned_sha256() {
    assert_pinned(PERFT_POSITIONS_FILE, PERFT_POSITIONS_SHA256);
}

#[test]
fn perft_depth1_origin_matches_bruteforce() {
    let mut state = GameState::new_game();
    let reference = RefGame::new();

    // Rule 3: turn 1 is one stone, and the lattice being homogeneous it goes on
    // the origin — so the initial position has exactly one turn.
    let turns = generate_turns(&state).expect("the initial position is a turn boundary");
    assert_eq!(shapes(&turns), reference.turns(), "the turns of a new game");
    assert_eq!(turns, vec![Turn::Single(Coord::ORIGIN)]);

    assert_eq!(perft(&mut state, 1).expect("depth 1"), reference.perft(1));
    assert_eq!(perft(&mut state, 1).expect("depth 1"), 1);
    assert_eq!(
        state,
        GameState::new_game(),
        "perft left the position as it was"
    );
}

#[test]
fn perft_depth2_origin_matches_bruteforce() {
    let mut state = GameState::new_game();
    let real = perft(&mut state, 2).expect("depth 2");
    assert_eq!(
        real,
        RefGame::new().perft(2),
        "the turns of turn 2, counted two ways"
    );
    assert_eq!(
        state,
        GameState::new_game(),
        "perft left the position as it was"
    );

    // Turn 1 has exactly one turn, so this is also the number of turns the
    // position after the origin stone has — which the fixture states in its own
    // right, reached by its own move list.
    let after_first_stone = expectation(&perft_case_named("one_stone_at_the_origin"), 1);
    assert_eq!(
        real, after_first_stone,
        "perft(2) is perft(1) one turn later"
    );
}

#[test]
fn perft_midgame_fixtures_match_bruteforce() {
    for case in perft_cases() {
        check_case(&case, DEFAULT_DIVIDE_SAMPLE);
    }
}

/// The same fixtures, dividing on more first turns than a debug run can afford.
/// `tools/perft_check.sh` runs this in release; it is the perft CI gate.
#[test]
#[ignore = "minutes in a debug build: tools/perft_check.sh runs it in release"]
fn perft_midgame_fixtures_match_bruteforce_over_a_wider_second_level_sample() {
    for case in perft_cases() {
        check_case(&case, WIDE_DIVIDE_SAMPLE);
    }
}

#[test]
#[should_panic(expected = "unknown directive")]
fn perft_loader_refuses_a_line_it_does_not_understand() {
    parse_cases("case a\nplies 0,0\nexpect depth 1 turns 1\nradius 2\n");
}

#[test]
#[should_panic(expected = "states no expected count")]
fn perft_loader_refuses_a_case_that_expects_nothing() {
    parse_cases("case a\nplies 0,0\n");
}

#[test]
#[should_panic(expected = "states depth 1 twice")]
fn perft_loader_refuses_two_counts_for_one_depth() {
    parse_cases("case a\nplies 0,0\nexpect depth 1 turns 1\nexpect depth 1 turns 2\n");
}

/// One fixture case: the position both implementations replay, the turns they
/// generate, the counts the fixture states, and the second level under a
/// sample of first turns.
fn check_case(case: &PerftCase, divide_sample: usize) {
    let name = &case.name;
    let mut state = GameState::from_plies(&case.plies)
        .unwrap_or_else(|error| panic!("case `{name}` (line {}): {error}", case.line));
    let reference = RefGame::from_plies(&case.plies);
    assert_eq!(
        state.board(),
        reference.board(),
        "case `{name}`: the two replays of the move list reached different positions"
    );
    assert_eq!(state.turn(), reference.turn(), "case `{name}`: turn number");
    assert_eq!(
        state.outcome().is_decided(),
        reference.is_decided(),
        "case `{name}`: whether the game is decided"
    );

    let generated = generate_turns(&state).unwrap_or_else(|error| {
        panic!("case `{name}`: the fixture position is not a turn boundary: {error}")
    });
    let reference_turns = reference.turns();
    assert_same_turns(
        &shapes(&generated),
        &reference_turns,
        &format!("case `{name}`"),
    );

    // No turns is one sequence of no turns, decided or not.
    assert_eq!(perft(&mut state, 0), Ok(1), "case `{name}` at depth 0");

    for expectation in &case.expect {
        let depth = expectation.depth_turns;
        let before = state.clone();
        let real = perft(&mut state, depth)
            .unwrap_or_else(|error| panic!("case `{name}` at depth {depth}: {error}"));
        assert_eq!(
            state, before,
            "case `{name}`: perft at depth {depth} did not restore the position"
        );
        assert_eq!(
            real, expectation.turns,
            "case `{name}` (line {}): perft at depth {depth}",
            expectation.line
        );
        let reference_count = if depth == 1 {
            reference_turns.len() as u64
        } else {
            reference.perft(depth)
        };
        assert_eq!(
            reference_count, expectation.turns,
            "case `{name}` (line {}): the reference disagrees at depth {depth}",
            expectation.line
        );
    }

    for index in sample_indices(generated.len(), divide_sample) {
        let turn = generated[index];
        state
            .make_turn(turn)
            .unwrap_or_else(|error| panic!("case `{name}`: generated turn {turn}: {error}"));
        let after = generate_turns(&state).unwrap_or_else(|error| {
            panic!("case `{name}`: after the generated turn {turn}: {error}")
        });
        let reference_after = reference.child(shape(turn)).turns();
        assert_same_turns(
            &shapes(&after),
            &reference_after,
            &format!("case `{name}`, after {turn}"),
        );
        let unmade = state
            .unmake_turn()
            .unwrap_or_else(|error| panic!("case `{name}`: taking back {turn}: {error}"));
        assert_eq!(unmade, turn, "case `{name}`: unmake returned another turn");
    }
}

/// Two turn sets, compared as sets and reported as a handful of turns.
///
/// A position has of the order of 10^5 turns, and a failure that prints two
/// lists that long is a failure nobody reads: what is wanted is which turns one
/// side has and the other does not.
fn assert_same_turns(real: &[RefTurn], reference: &[RefTurn], context: &str) {
    if real == reference {
        return;
    }
    let missing: Vec<RefTurn> = reference
        .iter()
        .filter(|turn| real.binary_search(turn).is_err())
        .take(5)
        .copied()
        .collect();
    let extra: Vec<RefTurn> = real
        .iter()
        .filter(|turn| reference.binary_search(turn).is_err())
        .take(5)
        .copied()
        .collect();
    panic!(
        "{context}: the generated turns differ from the reference's — {} generated, {} expected; \
         missing (up to five): {missing:?}; generated but not legal (up to five): {extra:?}",
        real.len(),
        reference.len()
    );
}

/// One generated turn as the reference spells it.
fn shape(turn: Turn) -> RefTurn {
    assert!(turn.is_canonical(), "{turn} is not canonically spelled");
    match turn {
        Turn::Single(at) => RefTurn::Single(at),
        Turn::Pair(first, second) => RefTurn::Pair(first, second),
    }
}

/// The generated turns as the reference spells them, sorted — which is also
/// where a turn that is not canonical would be caught, since the reference has
/// no way to spell one.
fn shapes(turns: &[Turn]) -> Vec<RefTurn> {
    let mut shapes: Vec<RefTurn> = turns.iter().map(|&turn| shape(turn)).collect();
    shapes.sort();
    let unique = shapes.len();
    shapes.dedup();
    assert_eq!(unique, shapes.len(), "the same turn was generated twice");
    shapes
}

/// `count` indices spread evenly over `len`, ends included, deterministic and
/// duplicate-free.
fn sample_indices(len: usize, count: usize) -> Vec<usize> {
    if len == 0 || count == 0 {
        return Vec::new();
    }
    if count == 1 {
        // The middle of the region rather than its rim: for a compact cloud
        // that is a first stone beside the stones already on the board.
        return vec![len / 2];
    }
    let mut indices: Vec<usize> = (0..count)
        .map(|step| step * (len - 1) / (count - 1))
        .collect();
    indices.dedup();
    indices
}

/// One fixture case by name.
fn perft_case_named(name: &str) -> PerftCase {
    common::perft_positions::perft_case(name)
}

/// The count a case states for a depth.
fn expectation(case: &PerftCase, depth_turns: u32) -> u64 {
    case.expect
        .iter()
        .find(|expectation| expectation.depth_turns == depth_turns)
        .unwrap_or_else(|| panic!("case `{}` states no depth {depth_turns}", case.name))
        .turns
}
