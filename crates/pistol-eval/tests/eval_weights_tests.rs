//! The weight table is configuration, and it obeys the config rules: explicit,
//! complete, closed, versioned (CLAUDE.md rule 1, docs/decisions.md D-11, D-64).
//!
//! No value in the document has a code-side default, so every rejection below
//! has to name the key an operator can go and edit. The one entry that is *not*
//! in the document — a window of six own stones — is not a tunable either: that
//! is a win, and a win's score belongs to the search's mate band (D-3, D-63).

mod common;

use std::path::PathBuf;

use common::{VALID_WEIGHTS, committed_weights, replacing, weights_rejection, without_line};
use pistol_eval::error::DOCUMENT_KEY;
use pistol_eval::{EVAL_MAX, EvalError, WEIGHTS_SCHEMA_VERSION, Weights};

#[test]
fn eval_weights_file_rejects_unknown_key() {
    let (key, why) = weights_rejection(&replacing(
        "schema_version = 1",
        "schema_version = 1\nwindow_bonus = 7",
    ));
    assert_eq!(
        key, "window_bonus",
        "the rejection must name the typo: {why}"
    );

    // A sixth table entry is the interesting case: it looks like a natural
    // extension of the table and is exactly what must not be settable.
    let (key, why) = weights_rejection(&replacing("5 = 1500", "5 = 1500\n6 = 9000"));
    assert_eq!(key, "table.6", "{why}");
}

#[test]
fn eval_weights_missing_entry_is_named_error() {
    let (key, why) = weights_rejection(&without_line("3 ="));
    assert_eq!(key, "table.3", "a missing entry is not a zero: {why}");

    for (prefix, expected) in [
        ("1 =", "table.1"),
        ("5 =", "table.5"),
        ("backend", "backend"),
        ("schema_version", "schema_version"),
    ] {
        let (key, why) = weights_rejection(&without_line(prefix));
        assert_eq!(key, expected, "dropping `{prefix}` must name it: {why}");
    }

    // The whole table gone is the containing key, not the first entry: with no
    // table there is no entry to name.
    let no_table: String = VALID_WEIGHTS
        .lines()
        .take_while(|line| !line.trim_start().starts_with("[table]"))
        .collect::<Vec<&str>>()
        .join("\n");
    let (key, why) = weights_rejection(&no_table);
    assert_eq!(key, "table", "{why}");
}

#[test]
fn eval_weights_reject_a_table_that_is_not_strictly_increasing() {
    // More own stones in a window is never worth less. A table that says
    // otherwise is a typo or a broken tuner, and either way it is refused rather
    // than played with (docs/decisions.md D-65).
    let (key, why) = weights_rejection(&replacing("4 = 300", "4 = 12"));
    assert_eq!(key, "table.4", "{why}");

    let (key, why) = weights_rejection(&replacing("2 = 12", "2 = 2"));
    assert_eq!(key, "table.2", "equal is not increasing either: {why}");

    let (key, why) = weights_rejection(&replacing("1 = 2", "1 = 0"));
    assert_eq!(key, "table.1", "a stone must be worth something: {why}");

    let (key, why) = weights_rejection(&replacing("1 = 2", "1 = -5"));
    assert_eq!(key, "table.1", "{why}");
}

#[test]
fn eval_weights_reject_a_window_worth_more_than_the_eval_band() {
    // One window that alone saturates the clamp would make every deeper
    // distinction invisible (docs/decisions.md D-3, D-65).
    let over = i64::from(EVAL_MAX) + 1;
    let (key, why) = weights_rejection(&replacing("5 = 1500", &format!("5 = {over}")));
    assert_eq!(key, "table.5", "{why}");
    assert!(
        why.contains(&EVAL_MAX.to_string()),
        "the rejection should say what the bound is: {why}"
    );
}

#[test]
fn eval_weights_reject_a_schema_version_this_build_does_not_read() {
    let (key, why) = weights_rejection(&replacing("schema_version = 1", "schema_version = 2"));
    assert_eq!(key, "schema_version", "{why}");
}

#[test]
fn eval_weights_reject_a_foreign_backend() {
    // A Stage-2 codebook file is a different document for a different backend.
    // Reading it as a v0 table would be a silent wrong answer.
    let (key, why) = weights_rejection(&replacing(
        r#"backend = "handcrafted_v0""#,
        r#"backend = "codebook_v1""#,
    ));
    assert_eq!(key, "backend", "{why}");
}

#[test]
fn eval_weights_reject_documents_that_are_not_weight_tables() {
    // Adversarial shapes on the load path. Every one of them has to come back as
    // a named rejection rather than as a table with something invented in it
    // (CLAUDE.md rules 1 and 3).
    let (key, why) = weights_rejection("");
    assert_eq!(
        key, "schema_version",
        "an empty document is missing all of it: {why}"
    );

    let (key, why) = weights_rejection("this is not toml at all\n");
    assert_eq!(key, DOCUMENT_KEY, "a syntax error belongs to no key: {why}");

    let (key, why) = weights_rejection(&replacing("3 = 60", "3 = 60\n3 = 61"));
    assert_eq!(
        key, DOCUMENT_KEY,
        "a duplicated key is a syntax error: {why}"
    );

    for (edit, expected) in [
        ("3 = 60.5", "table.3"),
        (r#"3 = "60""#, "table.3"),
        ("3 = true", "table.3"),
        // In range for TOML, out of range for the field it lands in.
        ("3 = 3000000000", "table.3"),
        // Out of range for TOML itself, which is a syntax error and so belongs
        // to no key — the document never gets as far as having fields.
        ("3 = 9223372036854775808", DOCUMENT_KEY),
    ] {
        let (key, why) = weights_rejection(&replacing("3 = 60", edit));
        assert_eq!(key, expected, "`{edit}` must name its key: {why}");
    }

    let (key, why) = weights_rejection(&replacing("[table]", "[table.deeper]"));
    assert!(
        key.starts_with("table"),
        "a table nested one level too deep belongs to `table`, got `{key}`: {why}"
    );
}

#[test]
fn eval_weights_missing_file_is_a_named_error() {
    // docs/decisions.md D-21: config validation checks the path's shape, and a
    // file that is not there is pistol-eval's loud error at load time.
    let path = PathBuf::from("configs/no_such_weights.toml");
    match Weights::load(&path) {
        Err(EvalError::WeightsUnreadable { path: named, why }) => {
            assert_eq!(named, path);
            assert!(!why.is_empty(), "the reason must say what the OS said");
        }
        Err(other) => panic!("expected an unreadable-file error, got: {other}"),
        Ok(_) => panic!("a weight table was loaded from a path that does not exist"),
    }
}

#[test]
fn eval_weights_committed_table_holds_the_operator_confirmed_values() {
    // The committed file is a contract, and this is its pin: an edit to those
    // five integers is a strength claim and fails here until it is accounted for
    // (CLAUDE.md rule 6).
    let weights = committed_weights();
    assert_eq!(
        [
            weights.window_value(1),
            weights.window_value(2),
            weights.window_value(3),
            weights.window_value(4),
            weights.window_value(5),
        ],
        [2, 12, 60, 300, 1500],
        "OPERATOR-CONFIRM values changed; the commit that changes them says why"
    );
    assert_eq!(weights.window_value(0), 0, "an empty window scores nothing");
    assert_eq!(
        weights.window_value(6),
        EVAL_MAX,
        "a completed window saturates the eval band; the mate band is the \
         search's (docs/decisions.md D-3, D-63)"
    );
    assert_eq!(WEIGHTS_SCHEMA_VERSION, 1);

    // The fixture the rejection tests edit must agree with the committed file,
    // or those tests are checking a schema nobody ships.
    assert_eq!(
        Weights::parse(VALID_WEIGHTS).expect("the fixture must be valid"),
        weights
    );
}
