//! The sha-pinned tactical suite under `CandidatePolicy::Staged` — THE
//! TACTICAL SUITE UNDER STAGED (docs/decisions.md D-316;
//! `U4_soundness_instrument.md` §8.3), one of the four soundness-gate names.
//!
//! Mirrors `tactical_v0_tests.rs`'s own three-test split (CLAUDE.md rule 7,
//! `tools/perft_check.sh`'s precedent): the fixture's pin and legality run in
//! every `cargo test`; the depth-one cases are solved as a debug-build
//! plumbing check; the whole suite meeting its pre-registered threshold is
//! `#[ignore]`d on the DEBUG cost and run by `tools/staged_soundness_check.sh`.
//!
//! The twenty positions are `tactical_v0.txt`'s own twenty — see that
//! fixture's header for the soundness argument
//! (`crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt`'s own header
//! restates it for the staged generator: three `[PROVEN]` laws bound how
//! Staged and Radius can differ at a node, and none of the three directions
//! can turn a game fact into a different one with the quiet cut disabled).

mod common;

use std::collections::BTreeSet;

use common::repo;
use pistol_cli::fixture_loader;
use pistol_cli::fixtures::Suite;
use pistol_cli::selftest;
use pistol_cli::sha256::sha256_hex;
use pistol_engine::{Budget, Config};

/// The fixture's name under `tests/fixtures/`.
const TACTICAL_STAGED_V0_FILE: &str = "tactical_staged_v0.txt";

/// The fixture's digest. Editing the fixture without updating this is a red
/// test, which is the point.
const TACTICAL_STAGED_V0_SHA256: &str =
    "fbd9be4cf7fa845e0ee65894c333db63e7fbb5de0a54088857c6e5401da9f53e";

/// CLAUDE.md rule 7's ceiling on a fixture file.
const FIXTURE_MAX_BYTES: usize = 10 * 1024 * 1024;

/// The fixture's path.
fn fixture_path() -> std::path::PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(TACTICAL_STAGED_V0_FILE)
}

/// The loaded suite.
fn suite() -> Suite {
    fixture_loader::load(&fixture_path())
        .unwrap_or_else(|error| panic!("the fixture must load: {error}"))
}

/// The configs the suite names, loaded, with their weights paths made absolute.
fn configs(suite: &Suite) -> Vec<(std::path::PathBuf, Config)> {
    suite
        .configs()
        .into_iter()
        .map(|named| {
            let config =
                common::committed(named.to_str().unwrap_or_else(|| {
                    panic!("{} is not a path this test can read", named.display())
                }));
            (named, config)
        })
        .collect()
}

#[test]
fn tactical_staged_v0_fixture_is_pinned_and_every_position_is_legal() {
    let bytes = std::fs::read(fixture_path()).expect("the fixture is committed");
    assert!(
        bytes.len() <= FIXTURE_MAX_BYTES,
        "{TACTICAL_STAGED_V0_FILE} is {} bytes, over the {FIXTURE_MAX_BYTES}-byte ceiling",
        bytes.len()
    );
    assert_eq!(
        sha256_hex(&bytes),
        TACTICAL_STAGED_V0_SHA256,
        "{TACTICAL_STAGED_V0_FILE} changed; update its pinned sha in the same commit"
    );

    let suite = suite();
    assert_eq!(suite.cases.len(), 20, "twenty positions (CLAUDE.md rule 7)");
    assert_eq!(
        suite.required, 20,
        "the pre-registered threshold is every case"
    );

    let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
    for case in &suite.cases {
        let state = case.position.replay().unwrap_or_else(|error| {
            panic!(
                "case `{}` (line {}) is not a position: {error}",
                case.name, case.line
            )
        });
        assert!(
            !state.outcome().is_decided(),
            "case `{}` is already decided",
            case.name
        );
        assert_eq!(
            state.phase(),
            pistol_core::Phase::First,
            "case `{}` is half way through a turn, which nothing that reads this \
             fixture can search",
            case.name
        );
        assert!(
            seen.insert((case.position.to_string(), format!("{:?}", case.budget))),
            "case `{}` repeats an earlier case's position and budget",
            case.name
        );
        assert!(
            matches!(case.budget, Budget::DepthTurns(_) | Budget::Nodes(_)),
            "case `{}` states a budget that cannot be reproduced",
            case.name
        );
    }

    let named: Vec<String> = suite
        .configs()
        .iter()
        .map(|path| path.display().to_string())
        .collect();
    assert_eq!(
        named,
        vec![
            String::from("configs/tactical_staged_v0.toml"),
            String::from("configs/gate_staged_v0.toml")
        ],
        "the suite is a claim about the two staged configs, in that order — the quiet cut \
         disabled in both (U4_soundness_instrument.md §8.3)"
    );
}

#[test]
fn tactical_staged_v0_first_depth_cases_are_solved() {
    let mut suite = suite();
    suite
        .cases
        .retain(|case| case.budget == Budget::DepthTurns(1));
    assert!(
        suite.cases.len() >= 10,
        "the fixture should still hold its depth-one cases, found {}",
        suite.cases.len()
    );
    suite.required = suite.cases.len();

    let report = selftest::run(&configs(&suite), &suite).expect("every case must be runnable");
    assert!(
        report.holds(),
        "the depth-one cases must all be solved and reproduce:\n{report}"
    );
}

#[test]
#[ignore = "release only, on the DEBUG cost — the same reason tactical_v0_tests.rs's own \
            suite-threshold test is ignored; run by tools/staged_soundness_check.sh"]
fn tactical_staged_v0_suite_meets_its_pre_registered_threshold() {
    let suite = suite();
    let report = selftest::run(&configs(&suite), &suite).expect("every case must be runnable");
    assert_eq!(
        report.determinism_failures(),
        0,
        "the determinism law is not a percentage:\n{report}"
    );
    assert!(
        report.tactical_passes() >= suite.required,
        "the suite must meet the threshold pre-registered in the fixture:\n{report}"
    );
}
