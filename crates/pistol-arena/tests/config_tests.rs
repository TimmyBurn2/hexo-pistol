//! The arena config is complete, closed, versioned, and refuses a wall-clock
//! budget by name.
//!
//! Every test here asserts a CONSEQUENCE rather than a constant against itself.
//! The WP-1.2a review round found four clauses whose tests read
//! `gap <= ELO_GAP_CEILING`, which is true whatever the constant holds
//! (docs/decisions.md D-152); the same shape would be available here and is
//! avoided.

mod common;

use pistol_arena::config::ArenaConfig;
use pistol_arena::error::ArenaError;

/// A complete document, which each test then breaks in exactly one place.
fn complete() -> String {
    String::from(
        "schema_version = 2\n\
         [run]\n\
         openings_file = \"openings.txt\"\n\
         openings_take = 4\n\
         openings_skip = 0\n\
         turn_cap = 12\n\
         n_workers = 2\n\
         hang_timeout_ms = 1000\n\
         [budget]\n\
         kind = \"depth_turns\"\n\
         value = 2\n\
         [sprt]\n\
         elo0 = 0.0\n\
         elo1 = 4.0\n\
         alpha = 0.05\n\
         beta = 0.05\n\
         [engine_a]\n\
         label = \"a\"\n\
         binary = \"a-bin\"\n\
         config = \"a.toml\"\n\
         [engine_b]\n\
         label = \"b\"\n\
         binary = \"b-bin\"\n\
         config = \"b.toml\"\n",
    )
}

fn load(text: &str) -> Result<ArenaConfig, ArenaError> {
    let config = ArenaConfig::parse_unvalidated(text)?;
    config.validate()?;
    Ok(config)
}

#[test]
fn the_complete_document_is_accepted() {
    // Without this every refusal test below could pass against a parser that
    // refuses everything.
    load(&complete()).expect("the complete document validates");
}

#[test]
fn arena_config_rejects_missing_sprt_bound() {
    for key in ["elo0", "elo1", "alpha", "beta"] {
        let broken: String = complete()
            .lines()
            .filter(|line| !line.starts_with(&format!("{key} =")))
            .map(|line| format!("{line}\n"))
            .collect();
        let error = load(&broken).expect_err("a missing SPRT bound is an error");
        let ArenaError::Config { key: named, why } = &error else {
            panic!("a missing key is a Config error, got {error}");
        };
        assert!(
            named.contains("sprt") || why.contains(key),
            "the refusal for a missing `{key}` must name it; got `{named}`: {why}"
        );
    }
}

#[test]
fn arena_refuses_movetime_budget_with_named_error() {
    let asked = complete().replace(
        "kind = \"depth_turns\"\nvalue = 2",
        "kind = \"movetime_ms\"\nvalue = 500",
    );
    let error = load(&asked).expect_err("a wall-clock budget is refused");
    assert_eq!(
        error.name(),
        "MovetimeBudgetRefused",
        "it is its own named refusal, not a generic config complaint"
    );
    let text = error.to_string();
    for cited in ["rule 6", "D-95"] {
        assert!(
            text.contains(cited),
            "the refusal points at the reason, not just the schema; it must cite {cited}: {text}"
        );
    }
    // And it is NOT serde's `unknown variant`, which would point at the schema.
    assert!(
        !text.contains("unknown variant"),
        "the variant is spelled in the schema precisely so the refusal can be this one: {text}"
    );

    // The same call accepts both instrument budgets, so the test above cannot
    // pass against a validator that refuses every budget.
    load(&complete()).expect("depth_turns is accepted");
    let nodes = complete().replace(
        "kind = \"depth_turns\"\nvalue = 2",
        "kind = \"nodes\"\nvalue = 20000",
    );
    load(&nodes).expect("nodes is accepted");
}

#[test]
fn an_unknown_key_is_refused_rather_than_ignored() {
    let extra = format!("{}spare_knob = 3\n", complete());
    let error = load(&extra).expect_err("an unknown key is an error");
    assert_eq!(error.name(), "Config");
}

#[test]
fn the_schema_version_is_the_arenas_own() {
    let wrong = complete().replace("schema_version = 2", "schema_version = 3");
    let error = load(&wrong).expect_err("a version this build does not read is an error");
    let text = error.to_string();
    assert!(
        text.contains("ARENA"),
        "the refusal says which document's version it is, because the engine config has one \
         too and they are never compared: {text}"
    );
}

#[test]
fn every_cross_field_rule_names_its_key() {
    // Each row breaks one rule and names the key the refusal must point at.
    let cases: Vec<(String, &str)> = vec![
        (
            complete().replace("n_workers = 2", "n_workers = 0"),
            "run.n_workers",
        ),
        (
            complete().replace("openings_take = 4", "openings_take = 0"),
            "run.openings_take",
        ),
        (
            complete().replace("openings_skip = 0", "openings_skip = 1000001"),
            "run.openings_skip",
        ),
        (
            complete().replace("turn_cap = 12", "turn_cap = 0"),
            "run.turn_cap",
        ),
        (
            complete().replace("hang_timeout_ms = 1000", "hang_timeout_ms = 0"),
            "run.hang_timeout_ms",
        ),
        (
            complete().replace("alpha = 0.05", "alpha = 0.0"),
            "sprt.alpha",
        ),
        (complete().replace("beta = 0.05", "beta = 1.0"), "sprt.beta"),
        (complete().replace("elo1 = 4.0", "elo1 = -1.0"), "sprt.elo1"),
        (complete().replace("value = 2", "value = 0"), "budget.value"),
        (
            complete().replace("label = \"b\"", "label = \"a\""),
            "engine_b.label",
        ),
        (
            complete().replace("label = \"a\"", "label = \"two words\""),
            "engine_a.label",
        ),
    ];
    for (broken, expected) in cases {
        let error = load(&broken).expect_err("a broken cross-field rule is an error");
        let ArenaError::Config { key, .. } = &error else {
            panic!("expected a Config error naming {expected}, got {error}");
        };
        assert_eq!(key, expected, "the refusal names the key an operator edits");
    }
    // alpha + beta >= 1 is its own rule and would otherwise be untested.
    let crossed = complete()
        .replace("alpha = 0.05", "alpha = 0.6")
        .replace("beta = 0.05", "beta = 0.5");
    let error = load(&crossed).expect_err("crossed boundaries are an error");
    assert!(error.to_string().contains("alpha + beta"));
}

#[test]
fn no_tunable_has_a_code_side_default() {
    // D-29's argument, applied to this crate: a rule that lives only in a
    // review checklist rots quietly, so reintroducing a default is a red test.
    let root = common::repo().join("crates/pistol-arena/src");
    for name in ["config.rs", "validate.rs"] {
        let source = std::fs::read_to_string(root.join(name)).expect("the source is readable");
        for forbidden in ["serde(default", "impl Default for", "#[derive(Default"] {
            assert!(
                !source.contains(forbidden),
                "{name} carries `{forbidden}`; every value in an arena config exists because an \
                 operator wrote it (CLAUDE.md rule 1)"
            );
        }
    }
}

#[test]
fn an_error_rate_that_makes_a_wald_bound_infinite_is_refused() {
    // RED-TEAM's finding. `alpha > 0.0` admits subnormals, and
    // `ln((1 - beta)/alpha)` then overflows: the run exited ZERO with
    // `bounds h0 -2.995732274 h1 inf` in the verdict block — a non-numeric field
    // in the machine-readable report, and an H1 that can never be reached, so a
    // silently mis-calibrated one-sided test nothing refused
    // (docs/decisions.md D-173).
    let asked = complete().replace("alpha = 0.05", "alpha = 1e-320");
    let error = load(&asked).expect_err("a boundary that cannot be crossed is refused");
    let ArenaError::Config { key, why } = &error else {
        panic!("expected a Config error, got {error}");
    };
    assert_eq!(key, "sprt.alpha");
    assert!(
        why.contains("inf"),
        "the refusal shows the boundary it computed: {why}"
    );
    // The refusal is readable. `Display` on a subnormal spells out every digit,
    // which was a several-hundred-character message on one line.
    assert!(
        why.len() < 500,
        "the refusal is {} characters; a float here is rendered in exponent form",
        why.len()
    );
    // And an ordinary small alpha still works, so this is a bound and not a ban.
    let fine = complete().replace("alpha = 0.05", "alpha = 0.001");
    load(&fine).expect("0.001 is a perfectly usable error rate");
}
