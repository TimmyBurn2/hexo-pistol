use pistol_core::{Coord, NEIGHBOUR_DIRECTIONS, Player};
use pistol_solver::{LiveCount, ThreatState};

/// Tier 1 is non-empty on every colouring of the minimal enclosure.
///
/// `quiescence.rs`'s completion-stone guard fires only when the ply-1 stone's
/// six neighbours are all occupied AND tier 1 is empty, and the comment above
/// it claims sixty-four colourings were swept with no empty tier 1. That claim
/// was carried in shipped source with no receipt anywhere; it is a test here so
/// it re-runs, because a measurement a reader cannot repeat is not evidence
/// (docs/decisions.md D-553's guard, CLAUDE.md's "the test carries it").
///
/// The union is the one `completion_stone` builds — the same four queries over
/// both sides — so this measures the shipped predicate rather than a
/// restatement of it. Sixty-four and not one hundred and twenty-eight: the
/// centre's colour is fixed, the other being its mirror.
#[test]
fn no_colouring_of_the_minimal_enclosure_leaves_tier_one_empty() {
    for colouring in 0u32..64 {
        let mut threats = ThreatState::new();
        threats.apply(Coord::ORIGIN, Player::P1);
        for (bit, direction) in NEIGHBOUR_DIRECTIONS.iter().enumerate() {
            let side = if colouring >> bit & 1 == 0 {
                Player::P1
            } else {
                Player::P2
            };
            threats.apply(Coord::ORIGIN.offset(*direction), side);
        }

        let mut cells = Vec::new();
        let mut buf = Vec::new();
        for side in [Player::P1, Player::P2] {
            threats.threat_cells(side, &mut buf);
            cells.extend_from_slice(&buf);
            threats.win_in_one_ply_cells(side, &mut buf);
            cells.extend_from_slice(&buf);
            threats.live_cells_at_count(side, LiveCount::Two, &mut buf);
            cells.extend_from_slice(&buf);
            threats.live_cells_at_count(side, LiveCount::Three, &mut buf);
            cells.extend_from_slice(&buf);
        }
        cells.sort_unstable();
        cells.dedup();

        assert!(
            !cells.is_empty(),
            "colouring {colouring:#08b} left no live window for either side, \
             which is the arrangement the guard exists for"
        );
    }
}

/// The control: this probe CAN see an empty tier 1.
///
/// Without it the sweep above passes whether or not the union is ever computed
/// — item 10's rule that a pass must not be available to an instrument that
/// answers the same way to everything.
#[test]
fn the_enclosure_probe_reports_an_empty_tier_one_when_there_is_one() {
    let threats = ThreatState::new();
    let mut cells = Vec::new();
    let mut buf = Vec::new();
    for side in [Player::P1, Player::P2] {
        threats.threat_cells(side, &mut buf);
        cells.extend_from_slice(&buf);
        threats.win_in_one_ply_cells(side, &mut buf);
        cells.extend_from_slice(&buf);
        threats.live_cells_at_count(side, LiveCount::Two, &mut buf);
        cells.extend_from_slice(&buf);
        threats.live_cells_at_count(side, LiveCount::Three, &mut buf);
        cells.extend_from_slice(&buf);
    }
    assert!(
        cells.is_empty(),
        "an empty board offers no live window, so the sweep's assertion is one \
         this probe could have failed: {cells:?}"
    );
}
