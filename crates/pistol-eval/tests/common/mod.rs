//! Shared test scaffolding: the committed weight table, one valid weights
//! document to state edits against, and the from-scratch reference the
//! incremental eval is checked against.
//!
//! The reference is the oracle here, in the same role the brute-force generator
//! plays for movegen (CLAUDE.md rule 7, docs/decisions.md D-12, D-68): it reads
//! the same rule out of the same weights the slow obvious way, sharing no code
//! with `pistol_eval::window` or `pistol_eval::handcrafted`.
#![allow(dead_code)] // each test binary uses a subset of these helpers.

pub mod playouts;
pub mod reference;

use std::path::PathBuf;

use pistol_core::{Board, Color, Coord};
use pistol_eval::{Eval, EvalError, HandcraftedV0, Weights};

/// A complete, in-range weights document.
///
/// Every weights test states its case as a difference from this, so a failure
/// says exactly which edit the schema failed to catch. It is deliberately *not*
/// the committed file: a test that mutates the real document would report a pass
/// for whatever that file happens to say today.
pub const VALID_WEIGHTS: &str = r#"
schema_version = 1
backend = "handcrafted_v0"

[table]
1 = 2
2 = 12
3 = 60
4 = 300
5 = 1500
"#;

/// The path of the committed weight table, from this crate's manifest.
pub fn committed_weights_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../configs/eval_v0_weights.toml")
}

/// The committed weight table, loaded. A failure here is a broken contract
/// file, not a broken test.
pub fn committed_weights() -> Weights {
    let path = committed_weights_path();
    Weights::load(&path).unwrap_or_else(|error| {
        panic!(
            "the committed weight table must load: {} rejected: {error}",
            path.display()
        )
    })
}

/// [`VALID_WEIGHTS`] with one substring rewritten.
pub fn replacing(from: &str, to: &str) -> String {
    assert!(
        VALID_WEIGHTS.contains(from),
        "fixture has no `{from}` to replace"
    );
    VALID_WEIGHTS.replace(from, to)
}

/// [`VALID_WEIGHTS`] with every line whose trimmed form starts with `prefix`
/// removed.
pub fn without_line(prefix: &str) -> String {
    let kept: Vec<&str> = VALID_WEIGHTS
        .lines()
        .filter(|line| !line.trim_start().starts_with(prefix))
        .collect();
    assert!(
        kept.len() < VALID_WEIGHTS.lines().count(),
        "fixture has no line starting with `{prefix}`"
    );
    kept.join("\n")
}

/// Parse a weights document, expecting a rejection; yields `(key, why)`.
///
/// Panics loudly if the document is accepted — a schema test that passes
/// because nothing was checked is worse than no test (CLAUDE.md rule 3).
pub fn weights_rejection(document: &str) -> (String, String) {
    match Weights::parse(document) {
        Err(EvalError::Weights { key, why }) => (key, why),
        Err(other) => panic!("expected a weights rejection, got: {other}"),
        Ok(_) => panic!("expected a rejection, but this was accepted:\n{document}"),
    }
}

/// A board and an eval carrying the same stones, applied in the given order.
///
/// The board is what the reference reads; the eval is what carried the value
/// incrementally. Nothing here goes through `GameState`, so a position no legal
/// game reaches is buildable — which the window tests need
/// (docs/decisions.md D-35).
pub fn built(weights: &Weights, stones: &[(Coord, Color)]) -> (Board, HandcraftedV0) {
    let mut board = Board::empty();
    let mut eval = HandcraftedV0::new(weights.clone());
    for &(at, color) in stones {
        board
            .apply(at, color)
            .unwrap_or_else(|error| panic!("test position must have distinct cells: {error}"));
        eval.apply(at, color);
    }
    (board, eval)
}

/// A run of `count` stones of one colour from `from`, stepping along `axis`.
pub fn line(from: Coord, axis: pistol_core::Axis, count: i16, color: Color) -> Vec<(Coord, Color)> {
    (0..count).map(|k| (from.step(axis, k), color)).collect()
}
