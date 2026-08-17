//! Shared test scaffolding: positions stated as stones per side, and the
//! searcher the tests run.
//!
//! A tactical fixture is easiest to read as "black has these stones, white has
//! these" — but a position is a move list (docs/decisions.md D-6), and the only
//! way to reach one is to play it (D-42). [`position`] bridges the two: it
//! interleaves the two sides' stones into the turn structure rule 3 imposes and
//! replays them through `GameState`, so a fixture that no legal game could reach
//! fails loudly here rather than quietly producing a position the search should
//! never see.
#![allow(dead_code)] // each test binary uses a subset of these helpers.

use std::path::PathBuf;

use pistol_core::{Axis, Color, Coord, GameState};
use pistol_eval::{HandcraftedV0, Weights};
use pistol_search::{CandidatePolicy, SearchParams, Searcher};

/// The committed weight table, loaded. A failure here is a broken contract
/// file, not a broken test.
pub fn committed_weights() -> Weights {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../configs/eval_v0_weights.toml");
    Weights::load(&path).unwrap_or_else(|error| {
        panic!(
            "the committed weight table must load: {} rejected: {error}",
            path.display()
        )
    })
}

/// Search parameters with everything stated: no test inherits a value from
/// anywhere but its own body (CLAUDE.md rule 1).
pub fn params(radius: u32, tt_bytes: u64) -> SearchParams {
    SearchParams {
        tt_bytes,
        candidate_policy: CandidatePolicy::Radius { radius },
    }
}

/// The smallest table this build accepts, which is what a test wants: a search
/// that fits in cache says more about the search than about the machine.
pub const SMALL_TT: u64 = 1 << 20;

/// A searcher over the committed weights.
pub fn searcher(radius: u32) -> Searcher {
    Searcher::new(
        params(radius, SMALL_TT),
        Box::new(HandcraftedV0::new(committed_weights())),
    )
    .expect("test search parameters must be accepted")
}

/// A position, stated as the stones each side holds.
///
/// The turn structure is rule 3's: black opens with one stone on the origin,
/// and every turn after that is two stones by the side to move. So black always
/// holds an odd number of stones, and white holds one more than black when it
/// is black to move and one fewer when it is white's.
///
/// Stones are played in the order given, which is what makes an intermediate
/// position legal or not; a fixture that wins early, plays out of the legal
/// region, or does not fit the turn structure panics here.
pub fn position(black: &[Coord], white: &[Coord], to_move: Color) -> GameState {
    assert!(
        !black.is_empty() && black[0] == Coord::ORIGIN,
        "black's first stone is turn 1's, and turn 1 is the origin (rule 3)"
    );
    let (b, w) = (black.len(), white.len());
    assert!(b % 2 == 1, "black holds an odd number of stones, got {b}");
    let expected_white = match to_move {
        Color::Black => b + 1,
        Color::White => b - 1,
    };
    assert_eq!(
        w, expected_white,
        "with {b} black stones and {to_move} to move, white holds {expected_white}"
    );

    let mut plies = vec![black[0]];
    let (mut next_black, mut next_white) = (1, 0);
    while next_black < b || next_white < w {
        for _ in 0..2 {
            if next_white < w {
                plies.push(white[next_white]);
                next_white += 1;
            }
        }
        for _ in 0..2 {
            if next_black < b {
                plies.push(black[next_black]);
                next_black += 1;
            }
        }
    }

    GameState::from_plies(&plies).unwrap_or_else(|error| {
        panic!("test fixture is not a legal game: {error} (plies: {plies:?})")
    })
}

/// A run of `count` cells from `from`, stepping along `axis`.
pub fn line(from: Coord, axis: Axis, count: i16) -> Vec<Coord> {
    (0..count).map(|k| from.step(axis, k)).collect()
}

/// Cells spread far enough apart to be worth nothing, for a side that is only
/// present to satisfy the turn structure.
///
/// They sit on one axis at a spacing of two, so no two of them share a window
/// (docs/decisions.md D-11: a window is six contiguous cells), and the eval reads
/// them as isolated stones rather than as a formation the search should react to.
pub fn spectators(from: Coord, axis: Axis, count: i16) -> Vec<Coord> {
    (0..count).map(|k| from.step(axis, k * 2)).collect()
}

/// A small position with nothing decided in it, for the tests that are about
/// the machinery rather than about tactics.
///
/// Every line of it is dead or short: black's pairs each have a white stone in
/// the gap, so no side can complete six inside the depths these tests reach and
/// no iteration ends early on a mate. It is also *small* — eleven stones in a
/// three-by-five patch — because every extra stone widens the candidate set at
/// every node.
pub fn quiet() -> GameState {
    let black = [
        Coord::ORIGIN,
        Coord::new(2, 0),
        Coord::new(0, 2),
        Coord::new(2, 2),
        Coord::new(1, 4),
    ];
    let white = [
        Coord::new(1, 0),
        Coord::new(0, 1),
        Coord::new(1, 1),
        Coord::new(2, 1),
        Coord::new(0, 3),
        Coord::new(1, 3),
    ];
    position(&black, &white, Color::Black)
}

/// A compact cluster of harmless stones: rows of three, two apart.
///
/// Nothing in it runs longer than three cells on any axis, so the side holding
/// it cannot complete six in one turn however it plays — and unlike
/// [`spectators`], it does not spread the stones over a wide area. That matters
/// for more than tidiness: the candidate policy reaches a fixed distance around
/// every stone, so a scattered filler side makes every node of the search
/// broader, and a test that takes minutes is a test that stops being run.
pub fn blob(from: Coord, count: usize) -> Vec<Coord> {
    (0..count)
        .map(|n| {
            let (row, column) = ((n / 3) as i16, (n % 3) as i16);
            from.offset(Coord::new(column, row * 2))
        })
        .collect()
}
