mod common;

use pistol_core::{Axis, Coord, GameState, Window};
use pistol_search::{CandidatePolicy, SearchParams, Searcher, params::StagedParams};
use pistol_solver::{EmittedNode, ProofKind, ProofTree, WinWitness, ZoneP};

fn eval() -> Box<dyn pistol_eval::Eval> {
    Box::new(pistol_eval::HandcraftedV0::new(common::committed_weights()))
}

fn staged(quiet_radius: u32, q_depth_turns: u32) -> StagedParams {
    StagedParams {
        quiet_radius,
        safety_net_top_k: 0,
        tier_t_top_k: 0,
        root_reorder: false,
        aspiration_delta: 0,
        extension_budget: 0,
        lmr_min_depth_turns: 0,
        lmr_late_index: 3,
        tier_t_own_count: 2,
        tier_t_opponent_count: 3,
        q_depth_turns,
        q_triggers: pistol_search::QTriggers::DefensiveAndOffensive,
        ordering: pistol_search::OrderingHeuristics {
            killers: false,
            history: false,
            countermove: false,
        },
    }
}

fn refusal(policy: CandidatePolicy) -> String {
    let refused = Searcher::new(
        SearchParams {
            tt_bytes: common::SMALL_TT,
            solver: None,
            candidate_policy: policy,
        },
        eval(),
    );
    match refused {
        Err(error) => error.to_string(),
        Ok(_) => panic!("this policy cannot be honoured, so it must be refused"),
    }
}

/// The radius ceiling says what a coordinate can hold, in the words that tell
/// an operator the value is not merely large but unrepresentable.
///
/// Audit row A-21: five guards in this crate print a refusal that no test
/// reads, so a message can be emptied, narrowed, or made to name the wrong key
/// with every gate green. D-553 is this project's own record of what that
/// costs.
#[test]
fn the_radius_refusal_says_a_ball_wider_than_a_coordinate_is_not_a_ball() {
    for policy in [
        CandidatePolicy::Radius { radius: u32::MAX },
        CandidatePolicy::Staged(staged(u32::MAX, 0)),
    ] {
        let said = refusal(policy);
        assert!(
            said.contains("a ball wider than a coordinate can step is not a ball"),
            "the refusal must say WHY the value is impossible rather than only that it is \
             too large: {said}"
        );
        assert!(
            said.contains(&i16::MAX.to_string()),
            "and name the ceiling the operator is under: {said}"
        );
    }
}

/// Both radius keys reach that refusal, and each names ITS OWN key. A guard
/// shared by two keys that names one of them sends an operator to the wrong
/// line of the document.
#[test]
fn each_radius_key_is_refused_under_its_own_name() {
    assert!(
        refusal(CandidatePolicy::Radius { radius: u32::MAX })
            .contains("search.candidate_policy.radius"),
        "the Radius policy's key"
    );
    let staged_said = refusal(CandidatePolicy::Staged(staged(u32::MAX, 0)));
    assert!(
        staged_said.contains("search.candidate_policy.quiet_radius"),
        "the Staged policy's key, which is a different line of the document: {staged_said}"
    );
}

/// The `q_depth_turns` ceiling names the structure that cannot hold the chain,
/// not merely the number.
#[test]
fn the_q_depth_refusal_names_the_table_the_chain_would_run_past() {
    let said = refusal(CandidatePolicy::Staged(staged(2, u32::MAX)));
    assert!(
        said.contains("search.candidate_policy.q_depth_turns"),
        "the key an operator edits: {said}"
    );
    assert!(
        said.contains("runs past") && said.contains("principal-variation table"),
        "and what it would run past, which is the only thing that makes the ceiling \
         a fact rather than a preference: {said}"
    );
}

/// A ONE-PLY witness on a board offering no partner cell, at BOTH public call
/// sites of the degeneration.
///
/// An empty board's legal region is exactly the origin (rule 3,
/// docs/decisions.md D-40), so a witness there has no second cell — the one
/// reachable input for a guard that a real position cannot provoke. Before
/// D-676 these two sites answered DIFFERENTLY on this very board:
/// `proof_first_move` returned `None` and `proof_line` panicked with an
/// unnamed `expect`.
fn one_ply_tree_at(at: Coord) -> ProofTree {
    let state = GameState::new_game();
    let key = state.key();
    ProofTree {
        root: key,
        nodes: vec![EmittedNode {
            key,
            kind: ProofKind::OrWinLeaf {
                witness: WinWitness::OnePly {
                    at,
                    window: Window::new(Axis::ConstR, Coord::ORIGIN)
                        .expect("a window at the origin"),
                },
            },
            zone: ZoneP::new(),
            children: Vec::new(),
        }],
    }
}

#[test]
fn a_one_ply_witness_with_no_partner_names_the_invariant_from_proof_first_move() {
    let state = GameState::new_game();
    let tree = one_ply_tree_at(Coord::ORIGIN);
    let panicked = std::panic::catch_unwind(|| pistol_search::proof_first_move(&tree, &state));
    let message = panic_message(panicked.expect_err("an empty board offers no partner cell"));
    assert!(
        message.contains("NO_ONE_PLY_PARTNER"),
        "the named invariant, not a bare expect: {message}"
    );
    assert!(
        message.contains("no other cell"),
        "and what was missing: {message}"
    );
}

#[test]
fn a_one_ply_witness_with_no_partner_names_the_same_invariant_from_proof_line() {
    let state = GameState::new_game();
    let tree = one_ply_tree_at(Coord::ORIGIN);
    let panicked = std::panic::catch_unwind(|| pistol_search::proof_line(&tree, &state, 4));
    let message = panic_message(panicked.expect_err("an empty board offers no partner cell"));
    assert!(
        message.contains("NO_ONE_PLY_PARTNER"),
        "the SAME invariant this site used to spell as an anonymous expect: {message}"
    );
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .unwrap_or_else(|| {
            payload
                .downcast_ref::<&str>()
                .map(|s| (*s).to_owned())
                .unwrap_or_else(|| "a panic payload this test cannot read".to_owned())
        })
}
