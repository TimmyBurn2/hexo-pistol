//! The half of the differential search oracle a debug build cannot afford.
//!
//! `search_oracle_tests.rs` carries what runs on every commit. This carries the
//! depths that do not: a full-width reference pays the candidate count squared
//! per TURN, and the smallest position this game has still offers six cells, so
//! a third turn costs at least a million reference nodes anywhere — seconds in
//! release and half a minute in debug. Every test here is therefore `#[ignore]`d
//! and run by `tools/search_oracle_check.sh` in release, which is the split
//! `perft_tests.rs` already uses for the movegen oracle's wide sample
//! (docs/decisions.md D-54, D-120).
//!
//! # What depth 3 buys, and what depth 4 would have cost
//!
//! Depth 3 is where an exact mate distance greater than two first appears, and
//! therefore the only place D-72's root-anchored re-basing is exercised across
//! more than one turn of each side. Depth 4 was measured rather than estimated:
//! on the cheapest position that exists — one stone, radius 1 — it is
//! 273 594 001 reference nodes and 614 s, against 999 841 nodes and 2.3 s at
//! depth 3. It is not run, and that number is recorded in D-120 rather than left
//! as a guess.
//!
//! Measured on the development machine, release, the three tests in parallel as
//! the gate runs them: **84.4 s**, of which the mate-in-three case at depth 3 is
//! 37 050 968 reference nodes and essentially all of it. The whole gate, this
//! file plus the always-on tiers re-run in release, is **86 s**. Every run
//! reports its own reference node count under `--nocapture`, which the gate
//! script passes, so those numbers regenerate instead of being remembered.
//!
//! Where the power is, as against where the cost is: the mate-in-three case buys
//! the only exact mate distance above two and catches NEITHER of the two window
//! bugs both reviews injected, while `tight_cluster_at_a_turn_boundary` at depth
//! 3 — 9 308 846 nodes, about 21 s — is the only comparison in the suite that
//! catches either (docs/decisions.md D-125). A trim of this gate has to know
//! which second is which.

mod common;

use common::agreement::agreement;
use common::ref_score::RefScore;
use common::{SMALL_TT, committed_weights, fixtures};

/// The positions a third turn is affordable on, and what each one is for.
///
/// `empty_board_turn_1` is where rule 3's one-stone turn makes the engine's ply
/// arithmetic stop being twice the depth, and it is nearly free (5 378 reference
/// nodes at depth 3) because the policy offers one cell on an empty board.
/// `one_stone_at_the_origin` is the smallest position with a real branching
/// factor. `tight_cluster_at_a_turn_boundary` is the expensive one and it earns
/// it: measured, it is the ONLY comparison in this suite that catches a deleted
/// PVS re-search or an unconditional transposition cutoff (D-125).
const DEPTH_3_AFFORDABLE: [&str; 3] = [
    "empty_board_turn_1",
    "one_stone_at_the_origin",
    "tight_cluster_at_a_turn_boundary",
];

/// The fixtures that get a second turn here. Everything the always-on tier
/// reaches only at depth 1, minus the mate-in-one cases that a second turn
/// cannot change — except three of them, one per axis, which are kept precisely
/// because the engine answers them from its FIRST iteration and stops on the
/// mate (`search.rs`'s `is_mate` break), while the reference searches the full
/// requested depth. That the two still agree is the load-bearing claim behind
/// every depth this suite compares at.
const DEPTH_2_BREADTH: [&str; 18] = [
    "mate_in_1_five_in_a_row_blocked_at_one_end",
    "mate_in_1_five_in_a_column_blocked_at_one_end",
    "mate_in_1_five_in_a_diagonal_blocked_at_one_end",
    "must_block_p2_five_in_a_row",
    "must_block_p2_five_in_a_column",
    "must_block_p2_five_on_a_diagonal",
    "must_block_p2_gap_fill",
    "mate_in_3_double_three_becomes_double_four",
    "mate_in_3_double_three_becomes_double_four_reflected",
    "mate_in_3_double_three_becomes_double_four_rotated",
    "quiet_no_tactic_for_either_side",
    "quiet_two_short_clusters",
    "two_lobes_joined_by_a_bridge",
    "a_win_the_mover_can_take",
    "playout_seed_5eed0001_turns_3",
    "playout_seed_5eed0003_turns_4",
    "playout_seed_5eed0005_turns_4",
    "playout_seed_5eed0007_turns_5",
];

/// The value the search reports is the value plain full-width negamax computes
/// over the same move universe, at every depth from one turn to three.
#[test]
#[ignore = "minutes in a debug build: tools/search_oracle_check.sh runs it in release"]
fn search_value_matches_reference_negamax_depths_1_to_3() {
    let weights = committed_weights();
    for name in DEPTH_3_AFFORDABLE {
        for depth in 1..=3 {
            let run = agreement(&fixtures::named(name), depth, 1, SMALL_TT, &weights);
            run.report();
            run.holds(&weights);
        }
    }
}

/// A mate three turns from the root agrees exactly — the distance, not merely
/// the fact of a win.
///
/// This is the one position in the suite whose value needs a third turn to
/// exist, and it is built as small as a forced mate in three can be for exactly
/// that reason. Its cost is the measured floor, not an accident of the fixture:
/// eleven stones is the fewest such a mate can have, and fifteen candidates at
/// radius 1 is the packing floor for eleven stones.
#[test]
#[ignore = "the measured floor for certifying a mate distance of three: tools/search_oracle_check.sh"]
fn search_mate_distances_match_reference_at_distance_3() {
    let weights = committed_weights();
    let fixture = fixtures::named("compact_mate_in_3");
    let mut deepest = None;
    for depth in 1..=3 {
        let run = agreement(&fixture, depth, 1, SMALL_TT, &weights);
        run.report();
        run.holds(&weights);
        deepest = Some(run);
    }
    // The same run, not a second one: walking this tree again to read one field
    // off it would be another eighty seconds for a number already in hand.
    let deepest = deepest.expect("three depths were run");
    assert_eq!(
        deepest.engine_score(),
        RefScore::WinInTurns(3),
        "the mover completes six on its SECOND turn from here, which is three turns from the \
         root counting both sides' (docs/decisions.md D-72)"
    );
}

/// Every fixture, at both radii at one turn and at two turns where the always-on
/// tier only reaches one.
#[test]
#[ignore = "minutes in a debug build: tools/search_oracle_check.sh runs it in release"]
fn search_reference_agreement_holds_over_the_full_fixture_set() {
    let weights = committed_weights();
    for fixture in fixtures::searchable() {
        for radius in [1, 2] {
            agreement(&fixture, 1, radius, SMALL_TT, &weights).holds(&weights);
        }
    }
    for name in DEPTH_2_BREADTH {
        let run = agreement(&fixtures::named(name), 2, 1, SMALL_TT, &weights);
        run.report();
        run.holds(&weights);
    }
    for name in [
        "one_stone_at_the_origin",
        "tight_cluster_at_a_turn_boundary",
    ] {
        let run = agreement(&fixtures::named(name), 2, 2, SMALL_TT, &weights);
        run.report();
        run.holds(&weights);
    }
}
