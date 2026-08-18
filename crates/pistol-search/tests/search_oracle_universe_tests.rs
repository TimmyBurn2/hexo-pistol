//! What the oracle's universe is, and what both implementations refuse.
//!
//! The agreement assertions live in `search_oracle_tests.rs`; this is the ground
//! they stand on. Two different questions, and neither is about a value:
//!
//! - the reference walks the SAME move universe the engine does, because it
//!   calls the same `candidate_cells`. That is deliberate — re-implementing the
//!   policy would pin nothing — and it means a systematically wrong candidate set
//!   would be invisible to every agreement assertion in the suite. So the
//!   universe is tied to something outside both: the movegen oracle.
//! - the two refuse the same questions, by name. A reference that answered where
//!   the search refuses would make a comparison meaningless one position at a
//!   time, and one that walked forever where the search refuses in constant time
//!   would hang the suite instead of failing it (CLAUDE.md rule 3).
//!
//! Split from the agreement half for rule 9's soft cap, and `search_oracle_
//! dedupe_tests.rs` split from this one for the same reason; the three are one
//! suite.

mod common;

use std::collections::BTreeSet;

use common::reference::{ReferenceError, reference_root_values};
use common::{SMALL_TT, committed_weights, fixtures, params};
use pistol_core::{Coord, GameState, LEGAL_RADIUS, Phase, Turn, generate_turns};
use pistol_eval::HandcraftedV0;
use pistol_search::{CandidatePolicy, MAX_DEPTH_TURNS, SearchError, Searcher, Stop};

/// The reference's own universe is the movegen oracle's.
///
/// Sharing `candidate_cells` with the engine is deliberate, and it means a
/// systematically wrong candidate set would be invisible to every other
/// assertion here. At a policy radius that reaches past the rules' region the
/// policy restricts nothing, so the turns the reference builds by playing plies
/// and grouping them must be exactly the turns `generate_turns` emits — the
/// generator the perft oracle pins (CLAUDE.md rule 7, docs/decisions.md D-12).
#[test]
fn reference_root_turns_are_the_movegen_oracles_turns() {
    let state = GameState::from_plies(&[Coord::ORIGIN]).expect("turn 1 is the origin");
    let run = reference_root_values(
        &state,
        1,
        CandidatePolicy::Radius {
            radius: LEGAL_RADIUS,
        },
        &committed_weights(),
    )
    .expect("an ongoing root at a turn boundary");

    let mine: Vec<Turn> = run.values.keys().copied().collect();
    let mut theirs = generate_turns(&state).expect("an ongoing root at a turn boundary");
    theirs.sort_unstable();
    assert_eq!(
        mine, theirs,
        "the reference's universe and the movegen oracle's must be the same set of turns"
    );
}

/// A root half way through a turn has no turn-structured value, and the
/// reference says so rather than guessing at the semantics.
#[test]
fn reference_negamax_rejects_phase1_root() {
    let state = GameState::from_plies(&[Coord::ORIGIN, Coord::new(1, 0)])
        .expect("turn 1's stone and the first stone of turn 2 are a legal opening");
    assert_eq!(
        state.phase(),
        Phase::Second,
        "the fixture must actually be half way through a turn"
    );

    let refused = reference_root_values(
        &state,
        1,
        CandidatePolicy::Radius { radius: 1 },
        &committed_weights(),
    )
    .expect_err("the reference refuses a mid-turn root");

    assert_eq!(
        refused,
        ReferenceError::RootMidTurn { turn: state.turn() },
        "the refusal names the turn that is half played, as `Searcher` does"
    );
}

/// Both refuse a decided root, which is an agreement worth having: a reference
/// that answered where the search refuses would make every later comparison
/// meaningless one position at a time.
#[test]
fn search_and_reference_both_refuse_a_decided_root() {
    let decided = fixtures::named("already_won_is_not_expanded");
    let policy = CandidatePolicy::Radius { radius: 1 };

    let refused = reference_root_values(&decided.state, 1, policy, &committed_weights())
        .expect_err("the reference refuses a decided root");
    assert!(
        matches!(refused, ReferenceError::RootDecided { .. }),
        "the reference names the win: {refused:?}"
    );

    let error = Searcher::new(
        params(1, SMALL_TT),
        Box::new(HandcraftedV0::new(committed_weights())),
    )
    .expect("the parameters are accepted")
    .search(&decided.state, Stop::DepthTurns(1), &mut |_| {})
    .expect_err("the search refuses a decided root");
    assert!(
        matches!(error, SearchError::GameDecided { .. }),
        "the search names the win too: {error}"
    );
}

/// The oracle's fixture set is the sha-pinned tactical positions, the perft
/// oracle's midgame positions, the seeded playouts and the two positions built
/// for the mate distances — loaded, replayed through the rules, and every one
/// of them a position the search may be asked about.
#[test]
fn oracle_fixtures_are_positions_the_search_can_be_asked_about() {
    assert_eq!(
        fixtures::tactical_v0().len(),
        20,
        "the sha-pinned tactical fixture states twenty cases"
    );
    assert_eq!(
        fixtures::perft_midgame().len(),
        5,
        "the perft fixture states five positions"
    );
    assert_eq!(fixtures::playouts().len(), 4, "four seeded playouts");
    assert_eq!(fixtures::built().len(), 2, "the two built mate positions");
    assert_eq!(fixtures::opening().len(), 1, "the turn-1 root");

    // Distinct POSITIONS, not merely distinct names. Two playout seeds once
    // aliased onto one generator, so the suite paid for the same position twice
    // under two names while asserting it had four; a count cannot see that and
    // the position's own key can.
    let mut keys = BTreeSet::new();
    for fixture in fixtures::all() {
        assert!(
            keys.insert(fixture.state.key()),
            "{}: a second fixture is the same position",
            fixture.name
        );
    }

    for fixture in fixtures::searchable() {
        assert_eq!(
            fixture.state.phase(),
            Phase::First,
            "{}: the search starts at a turn boundary",
            fixture.name
        );
        assert!(
            !fixture.state.outcome().is_decided(),
            "{}: a decided position has no move to search for",
            fixture.name
        );
    }
    assert_eq!(
        fixtures::searchable().len(),
        31,
        "everything but the one decided perft position is searchable"
    );
}

/// Everything the search refuses, the reference refuses too, by name.
///
/// Parity matters twice over. A reference that ANSWERED where the search refuses
/// would make a comparison meaningless one position at a time. A reference that
/// merely walked forever would be worse: a depth past the horizon is refused by
/// the engine in constant time and would otherwise start a reference walk of some
/// 10^30 nodes, so the suite would hang rather than fail (CLAUDE.md rule 3).
///
/// The radius-zero case is here and not left to `RootNoCandidates` because on an
/// EMPTY board a proximity policy restricts nothing — there is no stone to be
/// near, and rule 3 decides alone (docs/decisions.md D-77) — so the reference
/// would happily answer the one question `Searcher::new` refuses outright.
#[test]
fn reference_refuses_every_question_the_search_refuses() {
    let weights = committed_weights();
    let state = GameState::from_plies(&[Coord::ORIGIN]).expect("turn 1 is the origin");
    let radius_1 = CandidatePolicy::Radius { radius: 1 };

    assert_eq!(
        reference_root_values(
            &GameState::new_game(),
            1,
            CandidatePolicy::Radius { radius: 0 },
            &weights
        )
        .expect_err("a radius of zero reaches only occupied cells"),
        ReferenceError::PolicyRadiusZero,
    );
    assert!(
        Searcher::new(
            params(0, SMALL_TT),
            Box::new(HandcraftedV0::new(committed_weights()))
        )
        .is_err(),
        "the search refuses the same radius"
    );

    for depth in [0, MAX_DEPTH_TURNS + 1] {
        assert_eq!(
            reference_root_values(&state, depth, radius_1, &weights)
                .expect_err("a depth outside the horizon is not a question this build answers"),
            ReferenceError::DepthOutOfRange {
                turns: depth,
                max: MAX_DEPTH_TURNS,
            },
        );
        let refused = Searcher::new(
            params(1, SMALL_TT),
            Box::new(HandcraftedV0::new(committed_weights())),
        )
        .expect("the parameters are accepted")
        .search(&state, Stop::DepthTurns(depth), &mut |_| {})
        .expect_err("the search refuses the same depth");
        assert!(
            matches!(refused, SearchError::DepthOutOfRange { .. }),
            "the search names it the same way: {refused}"
        );
    }
}
