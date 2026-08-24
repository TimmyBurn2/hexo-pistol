//! `tier_t_own_count`/`tier_t_opponent_count` behavioural coverage
//! (`docs/decisions.md` D-365; WP-1.5b Phase 4's MAJOR 1/MAJOR 2 findings,
//! `docs/experiments/wp15b_IMPL_REVIEW_REDTEAM.md`).
//!
//! Before this file, every test in this crate instantiated `StagedParams` at
//! exactly `(own = 2, opponent = 3)`, so nothing distinguished the two keys'
//! CONFIGURED values from the code that reads them — a mutant deleting the
//! whole opponent half of Tier T, or hard-coding the mover's own threshold to
//! `>= 3` regardless of `tier_t_own_count`, passed every gate in the tree
//! (Phase 4's mutation 2 and mutation M7, both reproduced and reverted in the
//! review). These four tests hold the position and the eval fixed and vary
//! only the count under test, so the SIGNAL is that one key's value, not the
//! position.
//!
//! The signature used throughout is `used_quiet_safety_net`, not a direct
//! private-function call: `tier_t_side`/`tier_t_union` are private to
//! `crate::staged`, so an external test observes their effect only through the
//! public `staged_candidates` entry point. Each position is built so the
//! union is EXACTLY the cells the count under test admits or excludes — empty
//! when excluded, which flips the safety net on and is therefore observable
//! without inspecting `out.cells` at all for the exclusion half of each pair.

mod common;

use pistol_core::{Coord, Player};
use pistol_search::staged::{StagedRow, StagedSet, staged_candidates};
use pistol_search::{QTriggers, StagedParams};

use common::{committed_weights, threats_for};
use pistol_eval::HandcraftedV0;

/// The live-two window's own eight empties, on the `ConstR@r=0` axis around
/// the pair at `(0,0)`/`(1,0)` — two overlapping length-six windows' union,
/// verified live against the shipped generator before this constant was
/// written down (both the own- and opponent-side positions below reach the
/// identical set, since the local geometry around the pair is the same
/// either way).
fn live_two_window_empties() -> Vec<Coord> {
    vec![
        Coord::new(-4, 0),
        Coord::new(-3, 0),
        Coord::new(-2, 0),
        Coord::new(-1, 0),
        Coord::new(2, 0),
        Coord::new(3, 0),
        Coord::new(4, 0),
        Coord::new(5, 0),
    ]
}

fn sorted(mut cells: Vec<Coord>) -> Vec<Coord> {
    cells.sort_unstable();
    cells
}

/// P1 (to move) holds a live-two pair at `(0,0)`/`(1,0)` plus one isolated
/// spectator at `(4,4)` sharing no axis with the pair; P2 holds two more
/// live-two pairs, far enough from the origin pair and from each other that
/// no window spans two of the three clusters. P2's own pairs are irrelevant
/// here — `tier_t_opponent_count` is pinned at 3 in every call below, which
/// excludes them regardless, isolating the signal to `tier_t_own_count`
/// alone.
fn own_threshold_position() -> pistol_core::GameState {
    common::position(
        &[Coord::new(0, 0), Coord::new(1, 0), Coord::new(4, 4)],
        &[
            Coord::new(0, -8),
            Coord::new(-1, -8),
            Coord::new(0, -16),
            Coord::new(-1, -16),
        ],
        Player::P1,
    )
}

/// The mirror of `own_threshold_position`: P1 (not to move) holds the live-two
/// pair, P2 (to move) holds two isolated stones sharing no axis with anything.
/// `tier_t_own_count` is pinned at 3 in every call below, excluding P2's own
/// (empty) contribution regardless, isolating the signal to
/// `tier_t_opponent_count` alone.
fn opponent_threshold_position() -> pistol_core::GameState {
    common::position(
        &[Coord::new(0, 0), Coord::new(1, 0), Coord::new(4, 4)],
        &[Coord::new(0, -8), Coord::new(8, -8)],
        Player::P2,
    )
}

fn run(
    state: &pistol_core::GameState,
    tier_t_own_count: u8,
    tier_t_opponent_count: u8,
) -> (StagedRow, StagedSet) {
    let threats = threats_for(state);
    let mut eval = Box::new(HandcraftedV0::new(committed_weights()));
    let params = StagedParams {
        quiet_radius: 2,
        tier_t_own_count,
        tier_t_opponent_count,
        q_depth_turns: 0,
        q_triggers: QTriggers::DefensiveAndOffensive,
    };
    let mut out = StagedSet::default();
    let row = staged_candidates(state, &threats, &mut *eval, false, params, &mut out);
    (row, out)
}

#[test]
fn tier_t_own_count_two_includes_the_movers_own_live_two_window() {
    let state = own_threshold_position();
    let (row, out) = run(&state, 2, 3);
    assert_eq!(row, StagedRow::Batched);
    assert!(
        !out.used_quiet_safety_net,
        "the mover's own live-two window is non-empty at threshold 2, so the \
         quiet-ball safety net must not fire"
    );
    assert_eq!(sorted(out.cells), live_two_window_empties());
}

#[test]
fn tier_t_own_count_three_excludes_the_same_live_two_window_and_falls_back_to_the_quiet_ball() {
    // The MINIMAL REPRODUCER for mutation M7 (`tier_t_side`'s `threshold <= 2`
    // branch hard-coded off): under the mutant, `tier_t_own_count_two_...`
    // above would ALSO see the safety net fire, since the mover's own
    // live-two window would never contribute regardless of the configured
    // count. This test alone does not catch M7 — the PAIR does, by disagreeing
    // with each other only when the code actually reads the configured value.
    let state = own_threshold_position();
    let (row, out) = run(&state, 3, 3);
    assert_eq!(row, StagedRow::Batched);
    assert!(
        out.used_quiet_safety_net,
        "at threshold 3 the mover's only structure (a live-two window) does \
         not qualify, and the opponent's own live-two pairs are excluded by \
         the same threshold pinned here, so Tier T is empty and the safety \
         net must fire"
    );
}

#[test]
fn tier_t_opponent_count_two_includes_the_non_movers_live_two_window() {
    let state = opponent_threshold_position();
    let (row, out) = run(&state, 3, 2);
    assert_eq!(row, StagedRow::Batched);
    assert!(
        !out.used_quiet_safety_net,
        "the non-mover's live-two window is non-empty at opponent threshold \
         2, so the quiet-ball safety net must not fire"
    );
    assert_eq!(sorted(out.cells), live_two_window_empties());
}

#[test]
fn tier_t_opponent_count_three_excludes_the_same_live_two_window_and_falls_back_to_the_quiet_ball()
{
    // The MINIMAL REPRODUCER for mutation 2 (`tier_t_union`'s
    // `cells.extend(opponent)` deleted): under that mutant, the opponent half
    // never contributes at ANY threshold, so
    // `tier_t_opponent_count_two_...` above would ALSO see the safety net
    // fire. The pair disagrees with each other only when the code actually
    // reads the configured value AND actually extends with the opponent's
    // contribution.
    let state = opponent_threshold_position();
    let (row, out) = run(&state, 3, 3);
    assert_eq!(row, StagedRow::Batched);
    assert!(
        out.used_quiet_safety_net,
        "at opponent threshold 3 the non-mover's only structure does not \
         qualify, and the mover's own contribution is excluded by the same \
         threshold pinned here, so Tier T is empty and the safety net must \
         fire"
    );
}
