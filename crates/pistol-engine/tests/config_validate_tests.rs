//! Rules `serde` cannot express: value ranges, and agreements between fields
//! in different sections.

mod common;

use std::path::Path;

use common::{accepted, rejection, replacing};
use pistol_engine::config::{EngineMode, MAX_CANDIDATE_RADIUS, MIN_TT_BYTES};
use pistol_engine::{Config, SCHEMA_VERSION};

/// A document that is instrument mode but asks for more than one thread parses
/// cleanly and is then refused, naming the thread count.
#[test]
fn instrument_mode_rejects_multithread_config() {
    let document = replacing("threads = 1", "threads = 2");

    // It is well-formed: this is a cross-field rule, not a parse error.
    let config = Config::parse_unvalidated(&document).expect("document should parse");
    assert_eq!(config.engine.mode, EngineMode::Instrument);
    assert_eq!(config.instrument.threads, 2);

    let (key, why) = rejection(&document);
    assert_eq!(key, "instrument.threads");
    assert!(why.contains("instrument"), "unexpected reason: {why}");
    assert!(
        why.contains('2'),
        "the reason should quote the value: {why}"
    );
}

#[test]
fn config_validation_allows_multiple_threads_in_play_mode() {
    let document = replacing("mode = \"instrument\"", "mode = \"play\"");
    let config = accepted(&replacing_in(&document, "threads = 1", "threads = 8"));
    assert_eq!(config.engine.mode, EngineMode::Play);
    assert_eq!(config.instrument.threads, 8);
}

#[test]
fn config_rejects_zero_threads() {
    let document = replacing("mode = \"instrument\"", "mode = \"play\"");
    let (key, why) = rejection(&replacing_in(&document, "threads = 1", "threads = 0"));
    assert_eq!(key, "instrument.threads");
    assert!(why.contains("at least 1"), "unexpected reason: {why}");
}

#[test]
fn config_rejects_foreign_schema_version() {
    let (key, why) = rejection(&replacing("schema_version = 1", "schema_version = 2"));
    assert_eq!(key, "schema_version");
    assert!(
        why.contains(&SCHEMA_VERSION.to_string()),
        "the reason should state the version this build reads: {why}"
    );
}

#[test]
fn config_rejects_undersized_transposition_table() {
    let too_small = MIN_TT_BYTES / 2;
    let (key, why) = rejection(&replacing(
        "tt_bytes = 1048576",
        &format!("tt_bytes = {too_small}"),
    ));
    assert_eq!(key, "search.tt_bytes");
    assert!(why.contains("at least"), "unexpected reason: {why}");
}

#[test]
fn config_rejects_transposition_table_that_is_not_a_power_of_two() {
    let (key, why) = rejection(&replacing("tt_bytes = 1048576", "tt_bytes = 3000000"));
    assert_eq!(key, "search.tt_bytes");
    assert!(why.contains("power of two"), "unexpected reason: {why}");
}

#[test]
fn config_rejects_out_of_range_candidate_radius() {
    for radius in [0, MAX_CANDIDATE_RADIUS + 1] {
        let (key, why) = rejection(&replacing("radius = 3", &format!("radius = {radius}")));
        assert_eq!(key, "search.candidate_policy.radius", "radius {radius}");
        assert!(why.contains("1..="), "radius {radius} gave: {why}");
    }
}

/// The rules' radius-8 legal region and the search candidate radius are
/// different concepts and are never compared (CLAUDE.md rule 2). A candidate
/// radius of 8 is therefore no more and no less special than any other.
#[test]
fn candidate_radius_is_not_tied_to_the_rules_radius() {
    for radius in [1, 8, MAX_CANDIDATE_RADIUS] {
        accepted(&replacing("radius = 3", &format!("radius = {radius}")));
    }
}

#[test]
fn config_rejects_empty_weights_path() {
    let (key, why) = rejection(&replacing(
        "weights_file = \"configs/eval_v0_weights.toml\"",
        "weights_file = \"\"",
    ));
    assert_eq!(key, "eval.weights_file");
    assert!(why.contains("empty path"), "unexpected reason: {why}");
}

/// Validation is pure: it never touches the filesystem, so a weights file that
/// does not exist is somebody else's loud error (docs/decisions.md D-21). This
/// is what lets the committed config name `configs/eval_v0_weights.toml`
/// before WP-05 writes it.
#[test]
fn config_accepts_a_weights_path_that_does_not_exist_yet() {
    let config = accepted(&replacing(
        "weights_file = \"configs/eval_v0_weights.toml\"",
        "weights_file = \"configs/nothing-here-yet.toml\"",
    ));
    assert!(!config.eval.weights_file.exists());
}

/// The committed config is part of the contract, so CI loads it for real.
#[test]
fn committed_instrument_config_loads_and_validates() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../configs/instrument_v0.toml");
    let config = Config::load(&path)
        .unwrap_or_else(|error| panic!("configs/instrument_v0.toml must load: {error}"));
    assert_eq!(config.schema_version, SCHEMA_VERSION);
    assert_eq!(config.engine.mode, EngineMode::Instrument);
    assert_eq!(config.instrument.threads, 1);
}

/// `common::replacing` edits the shared fixture; this edits an already-edited
/// document, with the same insistence that the substring was really there.
fn replacing_in(document: &str, from: &str, to: &str) -> String {
    assert!(
        document.contains(from),
        "document has no `{from}` to replace"
    );
    document.replace(from, to)
}
