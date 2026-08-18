//! The sha-pinned tactical suite (CLAUDE.md rule 7).
//!
//! Three tests, and the split is the one `tools/perft_check.sh` already sets for
//! the movegen oracle (docs/decisions.md D-54): what is cheap runs in every
//! `cargo test`, and what needs a release build is `#[ignore]`d and run by a
//! script wired into `tools/ci.sh`.
//!
//! - the fixture is pinned, and every position in it replays through the rules;
//! - the cases that resolve at the first depth are solved, which is the plumbing
//!   from fixture to engine to answer, end to end, in a debug build;
//! - the whole suite meets its pre-registered threshold — `#[ignore]`, because at
//!   the deployment candidate radius a completed depth of three turns costs
//!   84-100 s in a *release* build (see `configs/gate_v0.toml`'s measurement
//!   table), and a debug build is an order of magnitude slower again.
//!
//! The threshold itself is read from the fixture and is never chosen here: it was
//! pre-registered in the file's header before the suite was first run
//! (CLAUDE.md §Process).

mod common;

use std::collections::BTreeSet;

use common::repo;
use pistol_cli::fixture_loader;
use pistol_cli::fixtures::Suite;
use pistol_cli::selftest;
use pistol_cli::sha256::sha256_hex;
use pistol_engine::{Budget, Config};

/// The fixture's name under `tests/fixtures/`.
const TACTICAL_V0_FILE: &str = "tactical_v0.txt";

/// The fixture's digest. Editing the fixture without updating this is a red test,
/// which is the point: the suite's positions, budgets, thresholds and expectations
/// are a pre-registration, and a pre-registration that can be edited quietly is
/// not one.
const TACTICAL_V0_SHA256: &str = "2d0b76532f0d9476da7b3542fcb582a73062822829b71eff7640c1339f2dde3a";

/// CLAUDE.md rule 7's ceiling on a fixture file.
const FIXTURE_MAX_BYTES: usize = 10 * 1024 * 1024;

/// The fixture's path.
fn fixture_path() -> std::path::PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(TACTICAL_V0_FILE)
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
fn tactical_v0_fixture_is_pinned_and_every_position_is_legal() {
    let bytes = std::fs::read(fixture_path()).expect("the fixture is committed");
    assert!(
        bytes.len() <= FIXTURE_MAX_BYTES,
        "{TACTICAL_V0_FILE} is {} bytes, over the {FIXTURE_MAX_BYTES}-byte ceiling",
        bytes.len()
    );
    assert_eq!(
        sha256_hex(&bytes),
        TACTICAL_V0_SHA256,
        "{TACTICAL_V0_FILE} changed; update its pinned sha in the same commit"
    );

    let suite = suite();
    assert_eq!(suite.cases.len(), 20, "twenty positions (CLAUDE.md rule 7)");
    assert_eq!(
        suite.required, 20,
        "the pre-registered threshold is every case"
    );

    // Every position replays through the rules, and no two cases are the same
    // position under the same budget.
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
        // Every position is at a turn boundary, and that is a requirement rather
        // than an accident: `tools/determinism.sh` issues `go` on every `position`
        // line in this file, and a half-played turn is refused by name
        // (docs/decisions.md D-71, D-83) — which would fail the determinism gate
        // for a reason that has nothing to do with determinism.
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
        // A wall-clock budget would make the case unreproducible; the loader
        // refuses one, and this is the pin that the fixture never asks for it.
        assert!(
            matches!(case.budget, Budget::DepthTurns(_) | Budget::Nodes(_)),
            "case `{}` states a budget that cannot be reproduced",
            case.name
        );
    }

    // The configs the fixture names are the committed ones, and every case's
    // claim is therefore about a search an operator can reproduce.
    let named: Vec<String> = suite
        .configs()
        .iter()
        .map(|path| path.display().to_string())
        .collect();
    assert_eq!(
        named,
        vec![
            String::from("configs/instrument_v0.toml"),
            String::from("configs/gate_v0.toml")
        ],
        "the suite is a claim about the deployment config and the gate config, in that order"
    );
}

#[test]
fn tactical_v0_first_depth_cases_are_solved() {
    // The cases that resolve in one completed turn: a mate in one is found at
    // depth one and the deepening loop stops there, so these cost a handful of
    // nodes each even at the deployment candidate radius, and a debug build can
    // afford them. They exercise the whole path — fixture, config, engine,
    // expectation — which is what makes the ignored test below a strength
    // question rather than a plumbing question.
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
#[ignore = "release only: at the deployment radius a completed depth 3 costs ~100 s; \
            run by tools/tactical_check.sh"]
fn tactical_v0_suite_meets_its_pre_registered_threshold() {
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
