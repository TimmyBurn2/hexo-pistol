//! The M4 one-free-stone widening's own tests (design wp18b_m4 §4).
//!
//! The flip fixture is anchor position g001-t42 (sealbot to move, one turn
//! before v0 first proves the conversion): under `OneFreeStone` the solver
//! proves the win at 10,726 nodes / depth 3 (MEASURED at impl; v0
//! WALL-CAPPED at 60 s on the same position — the anchor probe's receipt,
//! `artifacts/wp18b_probe_v1_results.txt`). The position is pinned by its
//! plies, and the test does not depend on the node count: what it pins is
//! the VALUE and the STRUCTURE (the witness needs arm B, which v0 cannot
//! even represent).

use pistol_core::{Coord, GameState, Player};
use pistol_solver::config::AttackerPolicy;
use pistol_solver::policy;
use pistol_solver::state::ThreatState;
use pistol_solver::{SolveOutcome, Solver};

// The verifier and its board readings, included beside this test exactly
// as the oracle tests include them (one common root, so the verifier's own
// `super::r3_zone` references resolve).
#[path = "common/r3_zone.rs"]
mod r3_zone;
#[path = "common/verifier.rs"]
mod verifier;

/// The verifier's wall cap for the flip tree (release-only, minutes-scale:
/// the AND nodes re-derive covering defender pairs over a ~570-cell legal
/// region; the MEASURED cost is recorded in the test's receipt comment
/// below once first run).
const FLIP_VERIFY_WALL: std::time::Duration = std::time::Duration::from_secs(600);

/// Anchor game 1, sealbot (P2) to move at turn 42 — the collapse-adjacent
/// position the v0 probe could not answer in 60 s and the widened solver
/// proves in seconds.
const G001_T42: &[&str] = &[
    "0,0", "2,-2", "1,0", "0,1", "0,5", "0,-2", "0,3", "1,-2", "2,-3", "-1,-1", "0,-1", "5,-6",
    "6,-7", "3,-4", "0,-4", "0,-3", "1,-3", "-1,-3", "1,-4", "2,-4", "5,-3", "3,-3", "-1,-4",
    "5,-5", "5,-4", "5,-7", "5,-2", "-2,-1", "-1,-2", "-2,-4", "4,-1", "-3,-4", "3,-1", "4,-2",
    "6,-3", "-4,1", "-1,1", "-3,0", "-3,1", "2,1", "4,1", "3,1", "-3,-1", "-3,3", "-2,-2", "7,-4",
    "-3,-3", "-3,2", "8,-5", "7,-2", "6,-2", "3,-2", "8,-2", "7,-3", "4,0", "-2,2", "5,-1", "7,-1",
    "-4,4", "7,-5", "7,1", "6,-5", "-4,-2", "-5,2", "-4,2", "-1,2", "-7,2", "8,-4", "8,-3", "8,-6",
    "8,0", "-2,0", "-2,1", "-2,3", "-2,-3", "1,4", "2,3", "3,2", "6,0", "-3,8", "6,-1",
];

fn g001_t42() -> GameState {
    let plies: Vec<Coord> = G001_T42.iter().map(|word| word.parse().unwrap()).collect();
    let state = GameState::from_plies(&plies).expect("an anchor transcript prefix");
    assert_eq!(state.to_move(), Player::P2, "sealbot moves at turn 42");
    state
}

fn epsilon() -> pistol_solver::pn::Epsilon {
    pistol_solver::pn::Epsilon::new(1, 4).unwrap()
}

/// The committed table size (configs/solver_v0.toml's 1,048,576): the knee
/// probe (wp18a §9a) measured deep solves HANGING at small tables — the
/// M4 proofs' working set is larger than v0's, and a 1024-entry table does
/// not terminate this solve in bounded time. Not a tunable here: the
/// registered instrument value.
fn table_entries() -> usize {
    1_048_576
}

/// The widening's value in one position (REVIEW-impl M-1's honest form):
/// v0 REFUTES g001-t42 — `nowin` at 955 nodes, the anchor probe's own
/// receipt (`artifacts/wp18b_probe_v1_results.txt`, line 41) — and the
/// widened solver proves the WIN at 10,726 nodes / depth 3. A true value
/// flip, exactly design §4's first bullet, with both halves asserted.
#[test]
fn the_widening_proves_a_win_v0_refutes() {
    let state = g001_t42();
    // The v0 half, receipt-backed (955 nodes, sub-second).
    let mut narrow = Solver::new(
        epsilon(),
        table_entries(),
        AttackerPolicy::BothStonesRelevant,
    );
    let narrow_result = narrow.solve(&state);
    assert_eq!(narrow_result.outcome, SolveOutcome::NoWin);
    assert_eq!(
        narrow_result.nodes, 955,
        "pinned by the probe artifact's receipt"
    );
    // The widened half.
    let mut solver = Solver::new(epsilon(), table_entries(), AttackerPolicy::OneFreeStone);
    let result = solver.solve(&state);
    let SolveOutcome::Win(tree) = result.outcome else {
        panic!("the widened solver proves g001-t42 (10,726 nodes at impl)");
    };
    // Some OR step's witness pair carries a cell OUTSIDE the position's C:
    // arm B. If every witness pair were both-in-C, v0 could in principle
    // find this same line — and v0's refutation above would then be a
    // solver bug, which gate (a)'s differential exists to catch.
    let mut threat = ThreatState::new();
    for (at, player) in state.board().stones() {
        threat.apply(at, player);
    }
    let mut candidates = Vec::new();
    policy::candidate_cells(&threat, Player::P2, &mut candidates);
    let mut used_arm_b = false;
    for node in &tree.nodes {
        if let pistol_solver::ProofKind::OrStep { witness } = &node.kind {
            for at in policy::turn_cells(witness) {
                if candidates.binary_search(&at).is_err() {
                    used_arm_b = true;
                }
            }
        }
    }
    assert!(
        used_arm_b,
        "the witness must need arm B — else v0's refutation and M4's win cannot both hold"
    );
    assert!(tree.win_depth_turns() >= 1);
}

/// REVIEW-impl M-2: the flip's own proof tree, re-proved full-width by the
/// INDEPENDENT verifier (gate (b)'s instrument, invoked per design M4-4's
/// flip seat). Release-only and minutes-scale; the wall cap is the test's
/// own registered watchdog.
#[test]
#[ignore = "release-only, minutes-scale: the verifier re-derives covering defender pairs at every AND node"]
fn the_flips_proof_tree_reverifies_full_width() {
    let state = g001_t42();
    let mut solver = Solver::new(epsilon(), table_entries(), AttackerPolicy::OneFreeStone);
    let result = solver.solve(&state);
    let SolveOutcome::Win(tree) = result.outcome else {
        panic!("the widened solver proves g001-t42");
    };
    let started = std::time::Instant::now();
    match verifier::verify(&state, &tree, AttackerPolicy::OneFreeStone) {
        verifier::Verdict::Verified { .. } => {}
        verifier::Verdict::Failed(what) => panic!("the flip's tree failed verification: {what}"),
    }
    assert!(
        started.elapsed() < FLIP_VERIFY_WALL,
        "FLIP-VERIFY-OVERRUN: the verifier exceeded its 600 s wall cap"
    );
}

/// Arm B's shape, order and dedup on a live-three position (design §2):
/// the widened set is a strict superset of v0's, every widened-only pair
/// has its first cell a raiser and its second outside `C`, and no
/// canonical pair appears twice.
#[test]
fn arm_b_is_a_dedup_free_superset_in_the_registered_order() {
    let state = g001_t42();
    let mut threat = ThreatState::new();
    for (at, player) in state.board().stones() {
        threat.apply(at, player);
    }
    let mut narrow = Vec::new();
    policy::threat_pairs(
        &state,
        &mut threat,
        Player::P2,
        AttackerPolicy::BothStonesRelevant,
        &mut narrow,
    );
    let mut wide = Vec::new();
    policy::threat_pairs(
        &state,
        &mut threat,
        Player::P2,
        AttackerPolicy::OneFreeStone,
        &mut wide,
    );
    assert!(
        wide.len() > narrow.len(),
        "arm B adds pairs at a live-three root"
    );
    // Dedup: no canonical pair twice in the union (the intra-arm-B class
    // included — two raisers would otherwise emit their pair twice).
    let mut sorted = wide.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        wide.len(),
        "every canonical pair appears once"
    );
    // Arm A is the widened set's prefix, verbatim (the design's order).
    assert_eq!(&wide[..narrow.len()], &narrow[..]);
    // `C`, for the free-cell filter and the membership invariants below.
    let mut candidates = Vec::new();
    policy::candidate_cells(&threat, Player::P2, &mut candidates);
    // The arm-B suffix equals the design §2 enumeration EXACTLY — raisers
    // ascending, free cells ascending, raiser-major — recomputed here from
    // the same queries. This pins order, shape and dedup in one equality:
    // an unsorted enumeration, a duplicate, or a wrong free-cell set all
    // break it.
    let mut raisers = Vec::new();
    threat.cells_raising_to_hot(Player::P2, pistol_solver::NearHot::Three, &mut raisers);
    raisers.sort_unstable();
    raisers.dedup();
    let in_c = |cell: Coord| candidates.binary_search(&cell).is_ok();
    let mut free: Vec<Coord> = pistol_core::legal_placements(state.board())
        .into_iter()
        .filter(|&cell| !in_c(cell))
        .collect();
    free.sort_unstable();
    free.dedup();
    let mut expected_suffix = Vec::new();
    for &raiser in &raisers {
        for &cell in &free {
            expected_suffix.push(
                pistol_core::Turn::pair(raiser, cell).expect("a raiser and a legal cell differ"),
            );
        }
    }
    assert_eq!(
        &wide[narrow.len()..],
        &expected_suffix[..],
        "arm B's exact enumeration"
    );
    // Membership invariants, independently of the equality above (so the
    // test names what it is pinning): every arm-B pair is raiser +
    // outside-C cell, and both cells are legal placements (rule 5).
    for turn in &wide[narrow.len()..] {
        let [first, second] = policy::turn_cells(turn);
        let raiser_count = [first, second]
            .iter()
            .filter(|at| raisers.binary_search(at).is_ok())
            .count();
        let outside_c = [first, second]
            .iter()
            .filter(|at| candidates.binary_search(at).is_err())
            .count();
        assert_eq!(
            (raiser_count, outside_c),
            (1, 1),
            "an arm-B pair is raiser + outside-C cell: {turn:?}"
        );
        for at in [first, second] {
            assert!(state.board().is_legal_placement(at), "{at:?} is legal");
        }
    }
}

/// The M4-4 theorem, spot-checked where it is observable: a one-node v0
/// NoWin stays a one-node widened NoWin, because `C = ∅` implies the
/// raiser set is empty and arm B never fires.
#[test]
fn a_scattered_position_is_identically_nowin_under_both_policies() {
    let plies: Vec<Coord> = ["0,0", "0,8", "0,-8", "8,0", "-7,7", "1,8", "1,-8"]
        .iter()
        .map(|word| word.parse().unwrap())
        .collect();
    let state = GameState::from_plies(&plies).expect("a legal scattered prefix");
    for policy_kind in [
        AttackerPolicy::BothStonesRelevant,
        AttackerPolicy::OneFreeStone,
    ] {
        let mut solver = Solver::new(epsilon(), 1024, policy_kind);
        let result = solver.solve(&state);
        assert_eq!(result.outcome, SolveOutcome::NoWin, "{policy_kind:?}");
        assert_eq!(result.nodes, 1, "an empty policy set is one visit");
    }
}

/// REVIEW-impl M-3: the three policy implementations AGREE, elementwise,
/// on a live-three position with live twos (so arm A and arm B both fire)
/// — the agreement M4-3's answer rests on, which no shipped gate exercises
/// for arm B (the bounded fixture is all one-node, where arm B never
/// executes). `r3.rs`'s board scan must produce the same sequence as
/// `policy.rs`'s ThreatState path and `r3_zone.rs`'s board scan, under
/// BOTH policies.
#[path = "common/r3.rs"]
mod r3;

#[test]
fn the_three_policy_sites_agree_elementwise_under_both_policies() {
    let state = g001_t42();

    let mut threat = ThreatState::new();
    for (at, player) in state.board().stones() {
        threat.apply(at, player);
    }
    for policy_kind in [
        AttackerPolicy::BothStonesRelevant,
        AttackerPolicy::OneFreeStone,
    ] {
        let mut mine = Vec::new();
        policy::threat_pairs(&state, &mut threat, Player::P2, policy_kind, &mut mine);
        let reference = r3::Reference::moves_only(Player::P2);
        let verifier_moves = r3_zone::threat_moves(&state, Player::P2, policy_kind);
        assert!(
            !mine.is_empty(),
            "g001-t42 has policy moves under {policy_kind:?}"
        );
        let reference_moves = reference.threat_moves(&state, policy_kind);
        assert_eq!(
            mine, reference_moves,
            "policy.rs and R3' agree elementwise under {policy_kind:?}"
        );
        assert_eq!(
            mine, verifier_moves,
            "policy.rs and the verifier's r3_zone agree elementwise under {policy_kind:?}"
        );
        if policy_kind == AttackerPolicy::OneFreeStone {
            let narrow = Vec::<pistol_core::Turn>::new();
            let _ = narrow; // the strict-superset check lives in the shape test above
        }
    }
}
