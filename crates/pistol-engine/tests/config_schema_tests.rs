//! The schema is complete and closed: nothing may be left out, nothing unknown
//! may be smuggled in, and no value may come from anywhere but the document.

mod common;

use common::{rejection, replacing, without_key, without_staged_key, without_table};
use pistol_engine::Config;

/// The config sources, scanned by `config_rejects_code_side_default_probe`.
/// These are the files that carry operator-facing values; if a default is ever
/// reintroduced, it has to be here.
const CONFIG_SOURCES: [(&str, &str); 3] = [
    ("src/config.rs", include_str!("../src/config.rs")),
    ("src/validate.rs", include_str!("../src/validate.rs")),
    ("src/budget.rs", include_str!("../src/budget.rs")),
];

/// Constructs that would let code supply a value an operator did not write.
const VALUE_SUPPLYING_CONSTRUCTS: [&str; 6] = [
    "impl Default for",
    "derive(Default",
    ", Default",
    "Default,",
    "serde(default",
    "unwrap_or",
];

#[test]
fn config_rejects_unknown_field() {
    // At the top level.
    let (key, why) = rejection(&replacing(
        "schema_version = 2",
        "schema_version = 2\nbogus_top = true",
    ));
    assert_eq!(key, "bogus_top");
    assert!(why.contains("unknown field"), "unexpected reason: {why}");

    // Inside a plain table.
    let (key, why) = rejection(&replacing(
        "tt_bytes = 1048576",
        "tt_bytes = 1048576\nbogus_knob = 7",
    ));
    assert_eq!(key, "search.bogus_knob");
    assert!(why.contains("unknown field"), "unexpected reason: {why}");

    // Inside the tagged candidate-policy table, whose contents serde buffers.
    let (key, why) = rejection(&replacing("radius = 3", "radius = 3\nbogus_policy = 1"));
    assert_eq!(key, "search.candidate_policy.bogus_policy");
    assert!(why.contains("unknown field"), "unexpected reason: {why}");
}

#[test]
fn config_rejects_missing_field() {
    for (dropped, expected_key) in [
        ("schema_version", "schema_version"),
        ("mode", "engine.mode"),
        ("tt_bytes", "search.tt_bytes"),
        ("kind", "search.candidate_policy.kind"),
        ("radius", "search.candidate_policy.radius"),
        ("backend", "eval.backend"),
        ("weights_file", "eval.weights_file"),
        ("threads", "instrument.threads"),
        ("tie_break", "instrument.tie_break"),
        ("movetime_epsilon_ms", "play.movetime_epsilon_ms"),
    ] {
        let (key, why) = rejection(&without_key(dropped));
        assert_eq!(
            key, expected_key,
            "dropping `{dropped}` named the wrong key"
        );
        assert!(
            why.contains("missing field") || why.contains("unknown variant"),
            "dropping `{dropped}` gave: {why}"
        );
    }
}

/// WP-1.7's three ordering-heuristic gates are required keys in the staged
/// variant, exactly like `q_depth_turns` before them: a missing gate is an
/// error, never an implicit OFF (CLAUDE.md rule 1,
/// docs/experiments/wp17_design.md §6).
#[test]
fn a_staged_document_missing_an_ordering_heuristic_gate_is_refused() {
    for dropped in ["killers", "history", "countermove"] {
        let expected_key = format!("search.candidate_policy.{dropped}");
        let (key, why) = rejection(&without_staged_key(dropped));
        assert_eq!(
            key, expected_key,
            "dropping `{dropped}` named the wrong key"
        );
        assert!(
            why.contains("missing field"),
            "dropping `{dropped}` gave: {why}"
        );
    }
}

#[test]
fn config_rejects_missing_table() {
    for table in ["engine", "search", "eval", "instrument", "play"] {
        let (key, why) = rejection(&without_table(table));
        assert_eq!(key, table, "dropping `[{table}]` named the wrong key");
        assert!(why.contains("missing field"), "dropping `[{table}]`: {why}");
    }
}

#[test]
fn config_rejects_code_side_default_probe() {
    // An absent document is a rejection, not an empty-but-usable config.
    let (key, why) = rejection("");
    assert_eq!(key, "schema_version");
    assert!(why.contains("missing field"), "unexpected reason: {why}");

    // No section can be conjured either — `config_rejects_missing_table`
    // covers that, and this restates it as part of the same guarantee.
    for table in ["engine", "search", "eval", "instrument", "play"] {
        assert_eq!(rejection(&without_table(table)).0, table);
    }

    // And no construct that could supply a value survives in the sources. The
    // `compile_fail` doctest on `Config` covers `Config::default()`; this
    // covers the quieter ways a default could creep back in.
    for (name, source) in CONFIG_SOURCES {
        assert!(!source.is_empty(), "include_str! of {name} came back empty");
        for construct in VALUE_SUPPLYING_CONSTRUCTS {
            assert!(
                !source.contains(construct),
                "{name} contains `{construct}`: config values come from the \
                 document only (CLAUDE.md rule 1)"
            );
        }
    }
}

#[test]
fn config_rejects_unknown_enum_variant() {
    for (from, to, expected_key) in [
        ("mode = \"instrument\"", "mode = \"turbo\"", "engine.mode"),
        (
            "kind = \"radius\"",
            "kind = \"knight\"",
            "search.candidate_policy.kind",
        ),
        (
            "backend = \"handcrafted_v0\"",
            "backend = \"net_v9\"",
            "eval.backend",
        ),
        (
            "tie_break = \"lexicographic\"",
            "tie_break = \"random\"",
            "instrument.tie_break",
        ),
    ] {
        let (key, why) = rejection(&replacing(from, to));
        assert_eq!(key, expected_key, "`{to}` named the wrong key");
        assert!(why.contains("unknown variant"), "`{to}` gave: {why}");
    }
}

#[test]
fn config_rejects_wrong_typed_value() {
    let (key, why) = rejection(&replacing("threads = 1", "threads = \"one\""));
    assert_eq!(key, "instrument.threads");
    assert!(why.contains("invalid type"), "unexpected reason: {why}");
}

#[test]
fn config_rejects_malformed_document() {
    let (key, why) = rejection(&replacing("[engine]", "[engine"));
    assert_eq!(key, pistol_engine::error::DOCUMENT_KEY);
    assert!(
        why.contains("line"),
        "a syntax error must locate itself: {why}"
    );
}

#[test]
fn config_load_of_absent_file_names_the_path() {
    let missing = std::path::Path::new("configs/definitely-not-here.toml");
    let error = Config::load(missing).expect_err("a missing config file is an error");
    let rendered = error.to_string();
    assert!(
        rendered.contains("definitely-not-here.toml"),
        "the error must name the path: {rendered}"
    );
}
