//! Shared test scaffolding: positions stated as stones per side, and the
//! searcher the tests run.
//!
//! A tactical fixture is easiest to read as "P1 has these stones, P2 has
//! these" — but a position is a move list (docs/decisions.md D-6), and the only
//! way to reach one is to play it (D-42). [`position`] bridges the two: it
//! interleaves the two sides' stones into the turn structure rule 3 imposes and
//! replays them through `GameState`, so a fixture that no legal game could reach
//! fails loudly here rather than quietly producing a position the search should
//! never see.
#![allow(dead_code)] // each test binary uses a subset of these helpers.

pub mod agreement;
pub mod fixture_text;
pub mod fixtures;
pub mod pair_dedupe;
pub mod playouts;
pub mod ref_score;
pub mod reference;
pub mod reference_invariants;
pub mod reference_walk;

use std::path::PathBuf;

use pistol_core::{Axis, Coord, GameState, Player};
use pistol_eval::{HandcraftedV0, Weights};
use pistol_search::{
    CandidatePolicy, OrderingHeuristics, QTriggers, SearchParams, Searcher, StagedParams,
};
use pistol_solver::ThreatState;

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

/// `CandidatePolicy::Staged` search parameters, everything stated (CLAUDE.md
/// rule 1): no test inherits a value from anywhere but its own body — the
/// ordering-heuristic gates included, so a test that wants them states them.
pub fn staged_params(
    quiet_radius: u32,
    tier_t_own_count: u8,
    tier_t_opponent_count: u8,
    q_depth_turns: u32,
    q_triggers: QTriggers,
    ordering: OrderingHeuristics,
    tt_bytes: u64,
) -> SearchParams {
    SearchParams {
        tt_bytes,
        candidate_policy: CandidatePolicy::Staged(StagedParams {
            quiet_radius,
            tier_t_own_count,
            tier_t_opponent_count,
            q_depth_turns,
            q_triggers,
            ordering,
        }),
    }
}

/// A searcher under `CandidatePolicy::Staged`, over the committed weights.
pub fn staged_searcher(
    quiet_radius: u32,
    tier_t_own_count: u8,
    tier_t_opponent_count: u8,
    q_depth_turns: u32,
    q_triggers: QTriggers,
    ordering: OrderingHeuristics,
) -> Searcher {
    Searcher::new(
        staged_params(
            quiet_radius,
            tier_t_own_count,
            tier_t_opponent_count,
            q_depth_turns,
            q_triggers,
            ordering,
            SMALL_TT,
        ),
        Box::new(HandcraftedV0::new(committed_weights())),
    )
    .expect("test staged parameters must be accepted")
}

/// A [`ThreatState`] matching `state`, built by replaying the same stones —
/// the same D-41-shaped construction `Position::reset_to` does internally,
/// available here for a test that wants the threat state alone.
pub fn threats_for(state: &GameState) -> ThreatState {
    let mut threats = ThreatState::new();
    for (at, player) in state.board().stones() {
        threats.apply(at, player);
    }
    threats
}

/// A position, stated as the stones each side holds.
///
/// The turn structure is rule 3's: P1 opens with one stone on the origin,
/// and every turn after that is two stones by the side to move. So P1 always
/// holds an odd number of stones, and P2 holds one more than P1 when it
/// is P1 to move and one fewer when it is P2's.
///
/// Stones are played in the order given, which is what makes an intermediate
/// position legal or not; a fixture that wins early, plays out of the legal
/// region, or does not fit the turn structure panics here.
pub fn position(p1: &[Coord], p2: &[Coord], to_move: Player) -> GameState {
    assert!(
        !p1.is_empty() && p1[0] == Coord::ORIGIN,
        "p1's first stone is turn 1's, and turn 1 is the origin (rule 3)"
    );
    let (b, w) = (p1.len(), p2.len());
    assert!(b % 2 == 1, "p1 holds an odd number of stones, got {b}");
    let expected_p2 = match to_move {
        Player::P1 => b + 1,
        Player::P2 => b - 1,
    };
    assert_eq!(
        w, expected_p2,
        "with {b} p1 stones and {to_move} to move, p2 holds {expected_p2}"
    );

    let mut plies = vec![p1[0]];
    let (mut next_p1, mut next_p2) = (1, 0);
    while next_p1 < b || next_p2 < w {
        for _ in 0..2 {
            if next_p2 < w {
                plies.push(p2[next_p2]);
                next_p2 += 1;
            }
        }
        for _ in 0..2 {
            if next_p1 < b {
                plies.push(p1[next_p1]);
                next_p1 += 1;
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
/// Every line of it is dead or short: P1's pairs each have a P2 stone in
/// the gap, so no side can complete six inside the depths these tests reach and
/// no iteration ends early on a mate. It is also *small* — eleven stones in a
/// three-by-five patch — because every extra stone widens the candidate set at
/// every node.
pub fn quiet() -> GameState {
    let p1 = [
        Coord::ORIGIN,
        Coord::new(2, 0),
        Coord::new(0, 2),
        Coord::new(2, 2),
        Coord::new(1, 4),
    ];
    let p2 = [
        Coord::new(1, 0),
        Coord::new(0, 1),
        Coord::new(1, 1),
        Coord::new(2, 1),
        Coord::new(0, 3),
        Coord::new(1, 3),
    ];
    position(&p1, &p2, Player::P1)
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
