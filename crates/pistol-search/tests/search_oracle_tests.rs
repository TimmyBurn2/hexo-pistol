mod common;

use common::agreement::{COMMITTED_TT, agreement};
use common::ref_score::RefScore;
use common::{SMALL_TT, committed_weights, fixtures};

/// Fixtures small enough for a second turn in a debug build.
///
/// Measured, not guessed: a second turn costs the candidate count squared, and
/// the seeded playouts (124 879 and 423 349 reference nodes at depth 2) are
/// three to twelve times what these two cost. They get their second turn in the
/// release tier instead.
const CHEAP_AT_DEPTH_2: [&str; 2] = [
    "one_stone_at_the_origin",
    "tight_cluster_at_a_turn_boundary",
];

/// Two fixtures for the radius cross-check, one opening and one midgame.
const BOTH_RADII: [&str; 3] = [
    "one_stone_at_the_origin",
    "tight_cluster_at_a_turn_boundary",
    "quiet_no_tactic_for_either_side",
];

/// The value the search reports is the value plain full-width negamax computes
/// over the same move universe.
#[test]
fn search_value_matches_reference_negamax_depths_1_and_2() {
    let weights = committed_weights();
    for fixture in fixtures::searchable() {
        let run = agreement(&fixture, 1, 1, SMALL_TT, &weights);
        run.report();
        run.value_agrees();
    }
    for name in CHEAP_AT_DEPTH_2 {
        let run = agreement(&fixtures::named(name), 2, 1, SMALL_TT, &weights);
        run.report();
        run.value_agrees();
    }
}

/// The move the search plays is one the reference also rates best.
///
/// Membership, and not identity: among equal-scoring turns the engine's pick is
/// its ordering pass, which the reference deliberately lacks (D-119).
#[test]
fn search_bestmove_is_in_reference_argmax_set() {
    let weights = committed_weights();
    for fixture in fixtures::searchable() {
        agreement(&fixture, 1, 1, SMALL_TT, &weights).bestmove_is_in_the_argmax_set();
    }
}

/// The move the search plays, valued through pistol-core's turn-level API
/// rather than through the reference's own ply loop, is still worth the
/// maximum.
#[test]
fn search_bestmove_value_equals_reference_max() {
    let weights = committed_weights();
    for fixture in fixtures::searchable() {
        agreement(&fixture, 1, 1, SMALL_TT, &weights)
            .bestmove_value_is_the_reference_maximum(&weights);
    }
}

/// Mate distances agree exactly, and a win is an odd number of turns from the
/// root while a loss is an even one (docs/decisions.md D-72).
#[test]
fn search_mate_distances_match_reference() {
    let weights = committed_weights();
    let mut wins = 0;
    for fixture in fixtures::tactical_v0() {
        if !fixture.name.starts_with("mate_in_1_") {
            continue;
        }
        let run = agreement(&fixture, 1, 1, SMALL_TT, &weights);
        run.value_agrees();
        assert_eq!(
            run.engine_score(),
            RefScore::WinInTurns(1),
            "{}: a stone placed this turn completes six (rule 4)",
            fixture.name
        );
        run.pv_proves_its_mate();
        wins += 1;
    }
    assert_eq!(wins, 11, "the fixture states eleven mate-in-one cases");

    // The one position in the suite whose value is an EVEN distance: the mover
    // is the one who loses, and D-72 says that is what a loss always looks like.
    let lost = agreement(
        &fixtures::named("compact_mated_in_2"),
        2,
        1,
        SMALL_TT,
        &weights,
    );
    lost.report();
    lost.value_agrees();
    assert_eq!(
        lost.engine_score(),
        RefScore::LossInTurns(2),
        "the opponent's double four completes on the turn after this one"
    );
    // The only place the reported line is replayed for a claimed LOSS.
    lost.pv_proves_its_mate();
}

/// The value does not depend on how big the transposition table is.
///
/// The MOVE can: two turns that tie on score are separated by whichever the
/// ordering tried first, and the table's move goes first, so table contents
/// decide the tie (docs/decisions.md D-109). What must not move is the score,
/// which is the soundness claim — so that is what this pins, together with
/// every table size still playing a turn the reference rates best.
#[test]
fn search_value_independent_of_tt_size() {
    let weights = committed_weights();
    // One bucket: the smallest table the type can build, not the smallest a
    // config may state. Maximum collision pressure is where a verification word
    // or the table's mate re-basing would show, and it is what D-109's own
    // measurement used.
    const ONE_BUCKET: u64 = 96;
    const SIZES: [u64; 3] = [ONE_BUCKET, SMALL_TT, COMMITTED_TT];

    for name in CHEAP_AT_DEPTH_2 {
        // The reference is a property of the position and the universe, so it
        // is walked once and read three times.
        let run = agreement(&fixtures::named(name), 2, 1, SIZES[0], &weights);
        run.bestmove_is_in_the_argmax_set();
        let scores: Vec<RefScore> = SIZES
            .iter()
            .map(|&bytes| run.rerun_with_tt_bytes(bytes, &weights))
            .collect();
        assert!(
            scores.windows(2).all(|pair| pair[0] == pair[1]),
            "{name}: the score moved with the table size {SIZES:?}: {scores:?}"
        );
    }
}

/// Agreement is a property of the universe the policy names, not of one radius.
#[test]
fn search_reference_agreement_holds_across_radius_1_and_2() {
    let weights = committed_weights();
    for name in BOTH_RADII {
        for radius in [1, 2] {
            let run = agreement(&fixtures::named(name), 1, radius, SMALL_TT, &weights);
            run.report();
            run.holds(&weights);
        }
    }
}
