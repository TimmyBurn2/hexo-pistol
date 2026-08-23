//! `CandidatePolicy::Staged`: the node protocol (`crate::staged`) and its
//! wiring into `pvs::visit`.
//!
//! Positions here are replayed `GameState::from_plies` ply lists, several of
//! them the same plies `crates/pistol-solver/tests/fixtures/threat_v0.txt`
//! pins for its own cases — so a claim like "P1's hot windows are unblockable
//! by P2" is not re-derived by hand, it is the same position that fixture
//! already certifies.
//!
//! # RULE9-JUSTIFICATION: one generator, over the fixed set of fixtures its
//! rows need (CLAUDE.md rule 9).
//!
//! Every row the node protocol can take (WIN-NOW, FILTERED, the overload
//! return, BATCHED and BATCHED-lost, the opening's own safety net) needs its
//! own position, and several tests reuse the same one under a different
//! `is_pv` to isolate the one branch that decision controls. Splitting by row
//! would duplicate the shared `params()`/`eval()` helpers and the position
//! builders per file; it grows again only if the protocol gains a row.

mod common;

use pistol_core::{Coord, GameState, Phase, Player};
use pistol_search::StagedParams;
use pistol_search::staged::{StagedRow, StagedSet, staged_candidates};

use common::{committed_weights, staged_searcher, threats_for};
use pistol_eval::{Eval, HandcraftedV0};

/// The pinned tactical fixture's own `mate_in_1_five_in_a_row_blocked_at_one_end`
/// position (`crates/pistol-cli/tests/fixtures/tactical_v0.txt`): P1 to move,
/// phase First, five in a row on the `q`-axis at `r=0` blocked behind at
/// `(-1,0)`. `can_win_this_turn(P1, Two)` is `Some(OnePly { at: (5,0), .. })`,
/// and it is a legal ROOT (phase First), so it doubles as the radius-policy
/// sanity check below.
fn win_in_one_ply_position() -> GameState {
    common::position(
        &[
            Coord::new(0, 0),
            Coord::new(1, 0),
            Coord::new(2, 0),
            Coord::new(3, 0),
            Coord::new(4, 0),
        ],
        &[
            Coord::new(-1, 0),
            Coord::new(1, 3),
            Coord::new(2, 3),
            Coord::new(3, 3),
            Coord::new(1, 5),
            Coord::new(2, 5),
        ],
        Player::P1,
    )
}

/// `true_double_threat`'s plies: P1 holds four hot windows whose empties no
/// budget-2 cover meets (`threat_v0.txt`'s `p1 cover 1/2 impossible`), and the
/// state this ply list reaches has P2 to move, phase 0, two stones owed. P2
/// cannot win this turn (`p2 canwin 1/2 none`).
fn unblockable_for_the_opponent_position() -> GameState {
    GameState::from_plies(&[
        Coord::new(0, 0),
        Coord::new(-5, 6),
        Coord::new(1, -6),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(1, 6),
        Coord::new(7, -6),
        Coord::new(3, 0),
        Coord::new(0, 14),
        Coord::new(-1, 20),
        Coord::new(-6, 26),
        Coord::new(0, 20),
        Coord::new(1, 20),
        Coord::new(0, 26),
        Coord::new(6, 14),
        Coord::new(2, 20),
        Coord::new(3, 20),
    ])
    .expect("true_double_threat is a legal game")
}

fn eval() -> Box<dyn Eval> {
    Box::new(HandcraftedV0::new(committed_weights()))
}

fn params(quiet_radius: u32, own: u8, opponent: u8) -> StagedParams {
    StagedParams {
        quiet_radius,
        tier_t_own_count: own,
        tier_t_opponent_count: opponent,
    }
}

#[test]
fn a_win_now_node_generates_only_the_win_now_class() {
    let state = win_in_one_ply_position();
    let threats = threats_for(&state);
    let mut eval = eval();
    let mut out = StagedSet::default();
    let row = staged_candidates(
        &state,
        &threats,
        &mut *eval,
        false,
        params(2, 2, 3),
        &mut out,
    );
    assert_eq!(row, StagedRow::WinNow);
    // `(5,0)` is the win-in-one-ply cell (the five-window's own empty); `(6,0)`
    // rides along because the FOUR consecutive stones `(1,0)..(4,0)` are ALSO a
    // hot window in their own right (`(1,0)-(6,0)`, empties `(5,0)` and
    // `(6,0)`) — the win-now class is the union §5.1 specifies, both windows'
    // contributions included, not only the nearer one.
    assert_eq!(
        out.cells,
        vec![Coord::new(5, 0), Coord::new(6, 0)],
        "the emitted set is exactly the win-now class's union, nothing else"
    );
    assert_eq!(
        out.forced,
        out.cells.len(),
        "the whole set is forced, none of it Tier T"
    );
}

#[test]
fn overload_at_entry_scores_loss_without_expansion() {
    // The node protocol's own verdict, checked directly (not_pv): the
    // opponent's hot windows are unblockable within either budget, so the row
    // is OverloadReturn and `out` carries nothing — `pvs::visit` is the one
    // that turns this into `-mate_in(turns_from_root + 2)` without expanding a
    // child; this test pins the verdict the return is built on.
    let state = unblockable_for_the_opponent_position();
    assert_eq!(state.to_move(), Player::P2);
    assert_eq!(state.phase(), Phase::First);
    let threats = threats_for(&state);
    let mut eval = eval();
    let mut out = StagedSet::default();
    let row = staged_candidates(
        &state,
        &threats,
        &mut *eval,
        false,
        params(2, 2, 3),
        &mut out,
    );
    assert_eq!(
        row,
        StagedRow::OverloadReturn,
        "P2 cannot win now and P1's hot windows admit no cover within budget"
    );
    assert!(
        out.cells.is_empty(),
        "no cell is generated for the overload return"
    );
}

#[test]
fn a_pv_node_at_the_same_impossible_verdict_generates_a_batched_lost_row_instead() {
    // The one branch `is_pv` decides (`U2_node_protocol.md` §5.3's
    // BATCHED-lost row): the SAME position, with `is_pv = true`, must not take
    // the early return — it is lost, but a PV node must return the line that
    // proves its score, so it generates Tier T instead.
    let state = unblockable_for_the_opponent_position();
    let threats = threats_for(&state);
    let mut eval = eval();
    let mut out = StagedSet::default();
    let row = staged_candidates(
        &state,
        &threats,
        &mut *eval,
        true,
        params(2, 2, 3),
        &mut out,
    );
    assert_eq!(row, StagedRow::BatchedLost);
    assert!(
        !out.cells.is_empty(),
        "a PV node generates a candidate set even on a lost verdict"
    );
    assert_eq!(out.forced, 0, "Tier F is empty on every BATCHED-lost row");
}

#[test]
fn a_forced_row_emits_no_cell_outside_tier_f() {
    // WIN-NOW: `forced == cells.len()`, proven by construction above. FILTERED
    // is checked here on a position built for it: P1's three stones share no
    // axis with each other (so P1 has no live window of its own at all, and
    // certainly cannot win now), while P2 holds one hot window — `(0,2)` to
    // `(3,2)`, blocked behind at `(-1,2)` by P1's own stone so no OTHER
    // window over the same four cells stays live — with two empties, `(4,2)`
    // and `(5,2)`, each its own one-cell cover.
    let filtered_state = common::position(
        &[Coord::new(0, 0), Coord::new(-1, 2), Coord::new(5, 3)],
        &[
            Coord::new(0, 2),
            Coord::new(1, 2),
            Coord::new(2, 2),
            Coord::new(3, 2),
        ],
        Player::P1,
    );
    let threats = threats_for(&filtered_state);
    assert!(
        threats
            .can_win_this_turn(filtered_state.to_move(), pistol_solver::StonesLeft::Two)
            .is_none(),
        "the mover must not be able to win now, or this is testing the WIN-NOW row"
    );
    let mut eval = eval();
    let mut out = StagedSet::default();
    let row = staged_candidates(
        &filtered_state,
        &threats,
        &mut *eval,
        false,
        params(2, 2, 3),
        &mut out,
    );
    assert_eq!(
        row,
        StagedRow::Filtered,
        "the fixture is built for the FILTERED row"
    );
    assert_eq!(
        out.cells,
        vec![Coord::new(4, 2), Coord::new(5, 2)],
        "the cover union alone: each empty of the one hot window, and nothing else"
    );
    assert_eq!(
        out.forced,
        out.cells.len(),
        "a forced row's whole set is forced — no Tier T or Tier Q cell rides beside it"
    );
}

#[test]
fn staged_generation_is_deterministic_across_repeated_calls() {
    let state = unblockable_for_the_opponent_position();
    let threats = threats_for(&state);
    let mut runs = Vec::new();
    for _ in 0..3 {
        let mut eval = eval();
        let mut out = StagedSet::default();
        let row = staged_candidates(
            &state,
            &threats,
            &mut *eval,
            true,
            params(2, 2, 3),
            &mut out,
        );
        runs.push((row, out));
    }
    for pair in runs.windows(2) {
        assert_eq!(
            pair[0], pair[1],
            "three runs over one position must agree exactly"
        );
    }
}

/// THE EMPTY-TIER-T SAFETY NET (`crate::staged`'s module doc): at the game's
/// earliest plies no window anywhere has reached a live count, so Tier T is
/// empty, and without a fallback the BATCHED row would emit nothing — which
/// crashes the search at the root (`NO_MOVE_FROM_A_COMPLETED_ITERATION`).
#[test]
fn a_batched_node_at_the_games_start_falls_back_to_the_quiet_ball_rather_than_emitting_nothing() {
    let opening = GameState::new_game();
    let threats = threats_for(&opening);
    let mut eval = eval();
    let mut out = StagedSet::default();
    let row = staged_candidates(
        &opening,
        &threats,
        &mut *eval,
        false,
        params(2, 2, 3),
        &mut out,
    );
    assert_eq!(
        row,
        StagedRow::Batched,
        "an empty board has no hot window for anyone"
    );
    assert!(
        !out.cells.is_empty(),
        "the safety net must supply a candidate at the game's very first node"
    );
    assert_eq!(
        out.cells,
        vec![Coord::ORIGIN],
        "the same answer candidate_cells gives for an empty board under any policy"
    );
}

#[test]
fn a_full_search_under_staged_completes_from_the_opening_without_crashing() {
    // The integration-level version of the safety-net test above: a real
    // `Searcher::search` call, at a depth that reaches past the plies where
    // Tier T is empty, must return a move rather than panicking.
    let mut searcher = staged_searcher(2, 2, 3);
    let outcome = searcher
        .search(
            &GameState::new_game(),
            pistol_search::Stop::DepthTurns(2),
            &mut |_| {},
        )
        .expect("a staged search from the opening must not be refused");
    assert_eq!(
        outcome.best.first(),
        Coord::ORIGIN,
        "turn 1 is a single stone and the origin is the only legal one"
    );
}

#[test]
fn a_radius_policy_search_is_unaffected_by_stagedparams_existing() {
    // The scoping claim of `U2_node_protocol.md` §7.2, checked at this crate's
    // own boundary: a `Radius` search run beside a `Staged` variant existing
    // in the same enum behaves exactly as it did before this WP, over a
    // position both policies can search.
    let state = win_in_one_ply_position();
    let mut radius = common::searcher(2);
    let outcome = radius
        .search(&state, pistol_search::Stop::DepthTurns(1), &mut |_| {})
        .expect("a radius search must still succeed");
    assert_eq!(
        outcome.info.score,
        pistol_search::score::mate_in(1),
        "the mate this position holds is found identically"
    );
}

/// `stage_counters_reported_in_search_info` (`U2_node_protocol.md` §U2-T):
/// each counter non-zero on a position built to fire it, zero on a
/// `CandidatePolicy::Radius` search that never dispatches through the staged
/// node protocol at all.
#[test]
fn stage_counters_are_reported_in_search_info_and_zero_under_radius() {
    let mut win_now_search = staged_searcher(2, 2, 3);
    let outcome = win_now_search
        .search(
            &win_in_one_ply_position(),
            pistol_search::Stop::DepthTurns(1),
            &mut |_| {},
        )
        .expect("a staged search over a mate-in-1 root must not be refused");
    assert!(
        outcome.info.stages.win_now >= 1,
        "the root itself takes the WIN-NOW row: {:?}",
        outcome.info.stages
    );

    let mut opening_search = staged_searcher(2, 2, 3);
    let outcome = opening_search
        .search(
            &GameState::new_game(),
            pistol_search::Stop::DepthTurns(3),
            &mut |_| {},
        )
        .expect("a staged search from the opening must not be refused");
    assert!(
        outcome.info.stages.batched >= 1,
        "a multi-ply search from the opening must visit at least one BATCHED node — the \
         census puts BATCHED at the large majority of nodes: {:?}",
        outcome.info.stages
    );
    assert!(
        outcome.info.stages.batched_quiet_safety_net >= 1,
        "the opening's own earliest plies have no live window anywhere, so the safety net \
         must have fired at least once: {:?}",
        outcome.info.stages
    );

    let mut radius = common::searcher(2);
    let outcome = radius
        .search(
            &win_in_one_ply_position(),
            pistol_search::Stop::DepthTurns(1),
            &mut |_| {},
        )
        .expect("a radius search must still succeed");
    assert_eq!(
        outcome.info.stages,
        pistol_search::StageCounters::default(),
        "a Radius search never dispatches through staged_candidates, so every counter stays zero"
    );
}
