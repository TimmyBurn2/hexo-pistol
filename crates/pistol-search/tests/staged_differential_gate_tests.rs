// R1, reused whole and not rewritten (D-323 condition 1) — this test binary
// exercises only `Reference::from_board` and `Reference::blocking_covers`, so
// its other public methods (`hot`, `live_at`, `can_win_this_turn`, ...) are
// unused HERE without being unused in `pistol-solver`'s own test tree, which
// is not this crate's dead code to report.
#[allow(dead_code)]
#[path = "../../pistol-solver/tests/common/reference.rs"]
mod reference;

mod common;

use pistol_core::GameState;
use pistol_search::staged::{StagedRow, StagedSet, staged_candidates};
use pistol_search::{OrderingHeuristics, QTriggers, StagedParams};
use pistol_solver::HitBudget;

use common::playouts::playout;
use common::{committed_weights, threats_for};
use pistol_eval::HandcraftedV0;
use reference::Reference;

/// The corpus: 30 seeds, five turn depths each, plus one extra ply from every
/// root to reach the opposite phase — 300 positions in all.
fn corpus() -> Vec<GameState> {
    let mut states = Vec::new();
    for seed in 0..30u64 {
        for turns in [3, 7, 11, 15, 19] {
            let root = playout(seed, turns);
            let extended = one_more_ply(&root, seed);
            states.push(root);
            states.push(extended);
        }
    }
    states
}

/// `root` plus one legal stone, reaching `StonesLeft::One` at the same turn.
/// Deterministic in `seed` so a failure names a reproducible position, the
/// same discipline `common::playouts::playout` states for its own output.
fn one_more_ply(root: &GameState, seed: u64) -> GameState {
    let candidates: Vec<pistol_core::Coord> = {
        let board = root.board();
        let (stone, _) = board
            .stones()
            .nth((seed as usize) % board.stone_count())
            .expect("the corpus root has at least one stone");
        [(1, 0), (0, 1), (-1, 0), (0, -1), (1, -1), (-1, 1)]
            .into_iter()
            .filter_map(|(dq, dr)| stone.checked_offset(pistol_core::Coord::new(dq, dr)))
            .filter(|&cell| board.is_legal_placement(cell))
            .collect()
    };
    let mut state = root.clone();
    for cell in candidates {
        match state.place(cell) {
            Ok(pistol_core::PlyOutcome::TurnContinues) => return state,
            Ok(_) => {
                state.undo().expect("the stone was just placed");
            }
            Err(_) => {}
        }
    }
    // No neighbour of the sampled stone was legal and non-winning: rare
    // (radius-8 legality is dense at these stone counts) and handled by
    // falling back to the root itself, so the corpus never panics on a
    // sampling edge case it does not need to be exhaustive over.
    root.clone()
}

/// The FILTERED row's emitted set must equal R1's own cover union, at every
/// FILTERED node of the corpus — S-M's whole criterion.
#[test]
fn the_filtered_row_matches_r1_at_every_filtered_node_of_the_corpus() {
    let weights = committed_weights();
    let params = StagedParams {
        quiet_radius: 2,
        safety_net_top_k: 0,
        tier_t_own_count: 2,
        tier_t_opponent_count: 3,
        q_depth_turns: 0,
        q_triggers: QTriggers::DefensiveAndOffensive,
        ordering: OrderingHeuristics {
            killers: false,
            history: false,
            countermove: false,
        },
    };
    let mut filtered_nodes = 0usize;
    for state in corpus() {
        let threats = threats_for(&state);
        let mut eval = Box::new(HandcraftedV0::new(weights.clone()));
        let mut out = StagedSet::default();
        // `is_pv` does not affect the FILTERED row's own generation (only the
        // Impossible branch reads it), so `false` exercises the same code
        // path a non-PV search node would.
        let row = staged_candidates(&state, &threats, &mut *eval, false, params, &mut out);
        // SCOPE, NAMED RATHER THAN LEFT IMPLICIT (docs/decisions.md D-370;
        // WP-1.5b Phase 4 MINOR 8): this gate is blind, by construction, to a
        // shipped row that DECLINES to be FILTERED where R1 says it should be
        // (a false `Impossible`/`NothingToBlock` — exactly what an over-eager
        // `three_pairwise_disjoint_families` early-out would produce). That
        // direction is covered elsewhere:
        // `pistol-solver`'s `threat_oracle_tests::threat_incremental_matches_reference_on_random_playouts`
        // asserts full `Cover` equality against R1 at every ply of the same
        // corpus shape, and is what caught the review's mutation of the
        // early-out (docs/decisions.md D-363).
        if row != StagedRow::Filtered {
            continue;
        }
        filtered_nodes += 1;

        let us = state.to_move();
        let left = pistol_solver::StonesLeft::from_state(&state)
            .expect("a corpus position is always ongoing");
        let budget = HitBudget::from(left);
        let reference = Reference::from_board(state.board());
        let reference_cover = reference.blocking_covers(us, budget);
        let expected = match &reference_cover {
            pistol_solver::Cover::Minimal(covers) => {
                let mut cells: Vec<pistol_core::Coord> =
                    covers.iter().flat_map(|cover| cover.cells()).collect();
                cells.sort_unstable();
                cells.dedup();
                cells
            }
            other => panic!(
                "the shipped generator took the FILTERED row (to_move {us}, {left:?}) but R1 \
                 answers {other:?} for the same position — a row-classification disagreement, \
                 not only a cell-set one"
            ),
        };

        assert_eq!(
            out.cells, expected,
            "FILTERED row disagrees with R1 at a corpus position (to_move {us}, {left:?}): \
             shipped {:?}, R1 {:?}",
            out.cells, expected
        );
    }
    // Non-vacuity (CLAUDE.md's own clause against a criterion no mutation can
    // fail): the corpus must actually have reached the row under test.
    assert!(
        filtered_nodes >= 10,
        "the corpus reached only {filtered_nodes} FILTERED nodes; the gate proves nothing at \
         a count this low"
    );
}
