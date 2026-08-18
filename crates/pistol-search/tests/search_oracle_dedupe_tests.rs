//! What the reference's ENUMERATION is: that valuing each unordered turn once
//! reaches the same tree as walking both orderings of every pair.
//!
//! Neither an agreement assertion nor a universe one — the engine is not in this
//! file at all. It is the reference checking itself, and it exists because the
//! reference is the one component in this oracle that must have no correctness
//! argument of its own: it stopped walking both orderings to buy depth
//! (docs/decisions.md D-126), and this is what pays for that.
//!
//! It also inherits a claim. The two orderings of a pair reaching the same value
//! at search depth is D-79's premise, and the reference used to assert it on
//! every run from inside the walk; a mode that walks one ordering cannot. So the
//! premise is checked here instead — over four fixtures at the depths a debug
//! build affords, and one turn deeper in the release tier, where rule 4's
//! truncation happens below the root rather than at it.
//!
//! Split from `search_oracle_universe_tests.rs` for rule 9's soft cap; the four
//! oracle files are one suite and the gate script runs all of them.

mod common;

use std::collections::BTreeSet;

use common::reference::{PairOrder, reference_root_values, reference_root_values_under};
use common::{committed_weights, fixtures};
use pistol_core::{Coord, Turn};
use pistol_search::CandidatePolicy;

/// The dedupe is exact: valuing each unordered turn once reaches the same map
/// as walking both orderings of every pair.
///
/// This is what the reference's cheap mode rests on, and it is also where
/// D-79's premise is now asserted — the interior check that the two orderings of
/// a pair agree cannot fire in a mode that walks one of them, so the claim moves
/// here rather than being dropped (docs/decisions.md D-120's amendment).
///
/// Rule 4's truncation is the case that makes this a real question rather than
/// a formality, and it is the reason `a_win_the_mover_can_take` is in the list:
/// a first stone that completes a line makes a ONE-stone turn, so the pair it
/// would otherwise have formed exists in the OTHER ordering only, and a dedupe
/// keyed on the cell set rather than on the turn would merge the two and lose
/// it. The structural half of that is asserted directly below the map
/// comparison, so a mode comparison that agreed by both losing the turn would
/// still be red.
///
/// The depths are the debug budget (D-120) and nothing else: both modes are
/// walked, so a case costs the expensive mode plus the cheap one. The three
/// quiet positions run at one turn and two, which is where the INTERIOR dedupe
/// is exercised at all; `a_win_the_mover_can_take` runs at one, because both
/// modes over its second turn is 583 200 reference nodes and would take this
/// binary past its pre-registered budget on its own. Its second turn — a rule-4
/// truncation below the root rather than at it — is bought in release instead,
/// by `search_oracle_deep_tests.rs`.
#[test]
fn reference_dedupe_matches_both_orderings_enumeration() {
    let weights = committed_weights();
    let radius_1 = CandidatePolicy::Radius { radius: 1 };
    for (name, deepest) in [
        ("empty_board_turn_1", 2),
        ("one_stone_at_the_origin", 2),
        ("tight_cluster_at_a_turn_boundary", 2),
        ("a_win_the_mover_can_take", 1),
    ] {
        let fixture = fixtures::named(name);
        for depth in 1..=deepest {
            let run = |pairs| {
                reference_root_values_under(&fixture.state, depth, radius_1, &weights, pairs)
                    .unwrap_or_else(|error| panic!("{name}: the reference refused: {error:?}"))
            };
            let both = run(PairOrder::BothOrderings);
            let deduped = run(PairOrder::Deduped);
            assert_eq!(
                both.values, deduped.values,
                "{name}: valuing each unordered turn once must reach the same value for every \
                 turn as walking both orderings ({depth} turns)"
            );
            println!(
                "dedupe {name:44} d{depth} both_orderings={:>10} deduped={:>10}",
                both.nodes, deduped.nodes
            );
        }
    }
}

/// The turn a stone that WINS makes is a one-stone turn, and the pairs that
/// stone belongs to are still turns — reached by the other ordering, and kept by
/// the dedupe.
///
/// The map comparison above would pass if both modes lost the same turn, so the
/// shape rule 4 imposes is asserted on its own: on a position where the mover
/// can win outright there is a `Single`, and the winning cell is also in a
/// `Pair` — which is only reachable by placing something else first. A dedupe
/// keyed on the cell set rather than on the turn merges the two and this goes
/// red (docs/decisions.md D-120's amendment).
#[test]
fn dedupe_keeps_the_pair_whose_ordering_rule_4_truncates() {
    let fixture = fixtures::named("a_win_the_mover_can_take");
    let run = reference_root_values(
        &fixture.state,
        1,
        CandidatePolicy::Radius { radius: 1 },
        &committed_weights(),
    )
    .expect("an ongoing root at a turn boundary");

    let won: BTreeSet<Coord> = run
        .values
        .keys()
        .filter_map(|&turn| match turn {
            Turn::Single(at) => Some(at),
            Turn::Pair(..) => None,
        })
        .collect();
    assert!(
        !won.is_empty(),
        "the mover can complete a line here, and rule 4 makes that a one-stone turn"
    );
    for at in &won {
        assert!(
            run.values.keys().any(|&turn| matches!(
                turn,
                Turn::Pair(first, second) if first == *at || second == *at
            )),
            "{at} wins as a first stone, so its pairs exist in the other ordering only, and \
             the deduped reference must still carry them"
        );
    }
}
