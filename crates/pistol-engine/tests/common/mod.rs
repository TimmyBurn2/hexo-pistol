//! Shared fixture for the config tests: one valid document, and edits to it.
//!
//! Every test states its case as a difference from [`VALID`], so a test that
//! fails says exactly which edit the schema failed to catch.
#![allow(dead_code)] // each test file uses a subset of these helpers.

use pistol_engine::{Config, EngineError};

/// A complete, in-range, instrument-mode document.
pub const VALID: &str = r#"
schema_version = 1

[engine]
mode = "instrument"

[search]
tt_bytes = 1048576

[search.candidate_policy]
kind = "radius"
radius = 3

[eval]
backend = "handcrafted_v0"
weights_file = "configs/eval_v0_weights.toml"

[instrument]
threads = 1
tie_break = "lexicographic"
"#;

/// [`VALID`] with one substring rewritten.
pub fn replacing(from: &str, to: &str) -> String {
    assert!(VALID.contains(from), "fixture has no `{from}` to replace");
    VALID.replace(from, to)
}

/// [`VALID`] with every line whose trimmed form starts with `prefix` removed.
pub fn without_key(prefix: &str) -> String {
    let kept: Vec<&str> = VALID
        .lines()
        .filter(|line| !line.trim_start().starts_with(prefix))
        .collect();
    assert!(
        kept.len() < VALID.lines().count(),
        "fixture has no line starting with `{prefix}`"
    );
    kept.join("\n")
}

/// [`VALID`] with a whole table removed, sub-tables included.
pub fn without_table(name: &str) -> String {
    let header = format!("[{name}]");
    let sub_prefix = format!("[{name}.");
    let mut kept = Vec::new();
    let mut dropping = false;
    for line in VALID.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('[') {
            dropping = trimmed.starts_with(&header) || trimmed.starts_with(&sub_prefix);
        }
        if !dropping {
            kept.push(line);
        }
    }
    assert!(
        kept.len() < VALID.lines().count(),
        "fixture has no `{header}`"
    );
    kept.join("\n")
}

/// Parse and validate, expecting a rejection; yields `(key, why)`.
///
/// Panics loudly if the document is accepted — a schema test that silently
/// passes because nothing was checked is worse than no test.
pub fn rejection(document: &str) -> (String, String) {
    let outcome = Config::parse_unvalidated(document).and_then(|config| {
        config.validate()?;
        Ok(config)
    });
    match outcome {
        Err(EngineError::Config { key, why }) => (key, why),
        Err(other) => panic!("expected a config rejection, got: {other}"),
        Ok(_) => panic!("expected a rejection, but this was accepted:\n{document}"),
    }
}

/// Parse and validate, expecting success.
pub fn accepted(document: &str) -> Config {
    let config = Config::parse_unvalidated(document)
        .unwrap_or_else(|error| panic!("fixture should parse, got: {error}"));
    config
        .validate()
        .unwrap_or_else(|error| panic!("fixture should validate, got: {error}"));
    config
}
