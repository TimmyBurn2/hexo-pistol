//! THE PATTERN FIXTURES UNDER STAGED (docs/decisions.md D-316;
//! `U4_soundness_instrument.md` §8.3), one of the four soundness-gate names.
//!
//! Re-scoped so it is ABOUT THE STAGE: `crates/pistol-solver/tests/fixtures/pattern_v0.txt`
//! pins the calculus's own named patterns as claims about `ThreatState`
//! alone; this suite runs the same positions through the staged generator
//! and asserts `PAT-GAP`'s singleton gap cell is forced — `LAW-HIT`, the
//! singleton plan must be hit, and every row this crate's node protocol can
//! take at a node with a singleton plan (`Cover::Minimal` with a
//! `MinimalCover::One`/`Two` containing it) makes the WHOLE emitted set
//! forced, not only the plan's own cell.
//!
//! The position is `pattern_v0.txt`'s own `PAT-GAP` ply list, replayed here
//! rather than re-derived by hand — the same plies that fixture's own case
//! pins, so a claim about the position is not a second, drifting copy of it.

mod common;

use pistol_core::{Coord, GameState};
use pistol_search::staged::{StagedRow, StagedSet, staged_candidates};
use pistol_search::{OrderingHeuristics, QTriggers, StagedParams};

use common::{committed_weights, threats_for};
use pistol_eval::HandcraftedV0;

/// `pattern_v0.txt`'s `PAT-GAP` case: `plies 0,0 0,-8 8,-8 2,0 3,0 0,-16
/// 8,-16 4,0 5,0`. P1 holds `X.XXXX` on the `ConstR` axis at `r=0` — a gap at
/// `(1,0)` — which the calculus's own plan enumeration reads as four hot
/// windows sharing the gap cell: `{-1,0 1,0}`, `{1,0}` (the singleton),
/// `{1,0 6,0}`, `{6,0 7,0}`. After the ply list, it is P2's turn.
fn pat_gap_position() -> GameState {
    GameState::from_plies(&[
        Coord::new(0, 0),
        Coord::new(0, -8),
        Coord::new(8, -8),
        Coord::new(2, 0),
        Coord::new(3, 0),
        Coord::new(0, -16),
        Coord::new(8, -16),
        Coord::new(4, 0),
        Coord::new(5, 0),
    ])
    .expect("PAT-GAP is a legal game (pattern_v0.txt)")
}

#[test]
fn pat_gaps_singleton_cell_is_forced_by_the_staged_generator() {
    let state = pat_gap_position();
    let threats = threats_for(&state);
    let mut eval = Box::new(HandcraftedV0::new(committed_weights()));
    let params = StagedParams {
        quiet_radius: 2,
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
    let mut out = StagedSet::default();
    let row = staged_candidates(&state, &threats, &mut *eval, false, params, &mut out);

    assert_eq!(
        row,
        StagedRow::Filtered,
        "P1's four hot windows share the singleton {{1,0}}, so P2's cover is Minimal and \
         nothing shorter than the FILTERED row applies"
    );
    assert!(
        out.cells.contains(&Coord::new(1, 0)),
        "PAT-GAP's singleton gap cell must be in the emitted set — LAW-HIT, the singleton \
         plan is a must-hit no minimal cover can omit: {:?}",
        out.cells
    );
    assert_eq!(
        out.forced,
        out.cells.len(),
        "a FILTERED row is entirely forced, so the gap cell is not merely present but \
         un-prunable by any delta ranking"
    );
    // The full derivation, not only the containment claim: two minimal
    // covers, {1,0;6,0} and {1,0;7,0}, whose union is exactly this.
    assert_eq!(
        out.cells,
        vec![Coord::new(1, 0), Coord::new(6, 0), Coord::new(7, 0)],
        "the exact cover union derived from the four plans PAT-GAP names"
    );
}
