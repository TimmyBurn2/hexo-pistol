use std::path::Path;

use pistol_cli::fixture_loader;
use pistol_cli::fixtures::Suite;

/// A minimal well-formed suite, as a template to break.
const ONE_CASE: &str = "\
require 1
case only
config configs/gate_v0.toml
position start moves 0,0
budget depth_turns 1
expect not-mated
";

fn parsed(text: &str) -> Suite {
    fixture_loader::parse(text, Path::new("fixture.txt"))
        .unwrap_or_else(|error| panic!("this should load: {error}"))
}

fn refused(text: &str) -> String {
    fixture_loader::parse(text, Path::new("fixture.txt"))
        .err()
        .unwrap_or_else(|| panic!("this should have been refused:\n{text}"))
        .to_string()
}

#[test]
fn a_well_formed_suite_loads_with_everything_it_states() {
    let suite = parsed(ONE_CASE);
    assert_eq!(suite.required, 1);
    assert_eq!(suite.cases.len(), 1);
    let case = &suite.cases[0];
    assert_eq!(case.name, "only");
    assert_eq!(case.config, Path::new("configs/gate_v0.toml"));
    assert_eq!(case.budget, pistol_engine::Budget::DepthTurns(1));
    assert_eq!(case.expect.len(), 1);
    assert_eq!(case.line, 2, "the case's own line, for a failure to name");
}

#[test]
fn carriage_returns_do_not_change_what_a_fixture_means() {
    let suite = parsed(&ONE_CASE.replace('\n', "\r\n"));
    assert_eq!(suite.cases[0].name, "only");
}

#[test]
fn a_fixture_that_states_no_threshold_is_refused() {
    let why = refused(&ONE_CASE.replace("require 1\n", ""));
    assert!(why.contains("require"), "{why}");
}

#[test]
fn a_threshold_no_run_can_fail_is_refused() {
    // `require 0` would be a gate that passes with nothing solved.
    let why = refused(&ONE_CASE.replace("require 1", "require 0"));
    assert!(why.contains("no run can fail"), "{why}");
    // And one larger than the suite is refused from the other side.
    let why = refused(&ONE_CASE.replace("require 1", "require 2"));
    assert!(why.contains("more passes"), "{why}");
}

#[test]
fn a_threshold_with_more_than_one_spelling_is_refused() {
    // The same argument the stone token makes: one number, one spelling
    // (docs/decisions.md D-46).
    for spelling in ["+1", "01", "0001", "1x", "one"] {
        let why = refused(&ONE_CASE.replace("require 1", &format!("require {spelling}")));
        assert!(
            why.contains("case count"),
            "`require {spelling}` should be refused as a count: {why}"
        );
    }
}

#[test]
fn a_case_missing_any_part_of_its_claim_is_refused() {
    for dropped in [
        "config configs/gate_v0.toml\n",
        "position start moves 0,0\n",
        "budget depth_turns 1\n",
        "expect not-mated\n",
    ] {
        let why = refused(&ONE_CASE.replace(dropped, ""));
        assert!(
            why.contains("states no") && why.contains("only"),
            "dropping `{}` should name the case and what is missing: {why}",
            dropped.trim()
        );
    }
}

#[test]
fn a_part_stated_twice_is_refused() {
    for doubled in [
        "config configs/gate_v0.toml",
        "position start moves 0,0",
        "budget depth_turns 1",
    ] {
        let text = ONE_CASE.replace(doubled, &format!("{doubled}\n{doubled}"));
        let why = refused(&text);
        assert!(why.contains("twice"), "{why}");
    }
}

#[test]
fn a_fixture_with_nothing_in_it_is_refused() {
    for text in ["", "\n\n", "# only comments\n# and more\n", "require 1\n"] {
        let why = refused(text);
        assert!(
            why.contains("no cases") || why.contains("require"),
            "`{text}`: {why}"
        );
    }
}

#[test]
fn a_directive_the_loader_does_not_know_is_refused_by_line() {
    let why = refused(&format!("{ONE_CASE}nonsense here\n"));
    assert!(why.contains("unknown directive"), "{why}");
    assert!(why.contains("fixture.txt:7"), "the line is named: {why}");
}

#[test]
fn a_directive_before_any_case_is_refused() {
    let why = refused("require 1\nbudget depth_turns 1\n");
    assert!(why.contains("before any"), "{why}");
}

#[test]
fn two_cases_with_one_name_are_refused() {
    let why = refused(&format!(
        "{ONE_CASE}{}",
        ONE_CASE.replace("require 1\n", "")
    ));
    assert!(why.contains("a second case is named"), "{why}");
}

#[test]
fn a_wall_clock_budget_is_refused() {
    // A case whose answer depends on how fast the machine is would pin nothing
    // (CLAUDE.md rule 4, docs/decisions.md D-4).
    let why = refused(&ONE_CASE.replace("budget depth_turns 1", "budget movetime 100"));
    assert!(why.contains("budget"), "{why}");
    let why = refused(&ONE_CASE.replace("budget depth_turns 1", "budget depth_turns 0"));
    assert!(why.contains("searches nothing"), "{why}");
}

#[test]
fn a_malformed_expectation_is_refused() {
    for expectation in [
        "expect",
        "expect nothing",
        "expect move",
        "expect move 1,0/0,0",
        "expect cell x,y",
        "expect mate",
        "expect mate -1",
        "expect not-mated please",
    ] {
        let why = refused(&ONE_CASE.replace("expect not-mated", expectation));
        assert!(!why.is_empty(), "`{expectation}` should be refused");
    }
}
