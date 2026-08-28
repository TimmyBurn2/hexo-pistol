#![allow(dead_code)] // each test binary uses a subset of these helpers.

pub mod boards;
pub mod bruteforce;
pub mod games;
pub mod perft_positions;
pub mod playouts;
pub mod sha256;

use std::fs;
use std::path::PathBuf;

use pistol_core::Coord;

use sha256::sha256_hex;

/// CLAUDE.md rule 7's ceiling on a fixture file.
pub const FIXTURE_MAX_BYTES: usize = 10 * 1024 * 1024;

/// A fixture's path, from its name under `tests/fixtures/`.
pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// A fixture's bytes, exactly as they are on disk — what the pin hashes.
pub fn fixture_bytes(name: &str) -> Vec<u8> {
    let path = fixture_path(name);
    fs::read(&path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

/// A fixture's text.
pub fn fixture_text(name: &str) -> String {
    String::from_utf8(fixture_bytes(name)).unwrap_or_else(|_| panic!("{name} is not UTF-8"))
}

/// Check a fixture against its pin and against rule 7's size ceiling.
///
/// The pin lives in the test that reads the fixture, so `cargo test` alone
/// catches a fixture edit that nobody accounted for (docs/decisions.md D-37).
pub fn assert_pinned(name: &str, expected_sha256: &str) {
    let bytes = fixture_bytes(name);
    assert!(
        bytes.len() <= FIXTURE_MAX_BYTES,
        "{name} is {} bytes, over the {FIXTURE_MAX_BYTES}-byte fixture ceiling",
        bytes.len()
    );
    assert_eq!(
        sha256_hex(&bytes),
        expected_sha256,
        "{name} changed; update its pinned sha in the same commit"
    );
}

/// The meaningful lines of a fixture, as `(line number, directive, rest)`.
///
/// Blank lines and `#` comments are dropped; nothing else is.
pub fn directives(text: &str) -> Vec<(usize, &str, &str)> {
    text.lines()
        .enumerate()
        .filter_map(|(index, raw)| {
            let content = raw.trim();
            if content.is_empty() || content.starts_with('#') {
                return None;
            }
            let (directive, rest) = match content.split_once(char::is_whitespace) {
                Some((directive, rest)) => (directive, rest.trim()),
                None => (content, ""),
            };
            Some((index + 1, directive, rest))
        })
        .collect()
}

/// The fixtures and the protocol share one stone-token grammar, and it lives in
/// pistol-core (docs/decisions.md D-39). This test tree does not write a second
/// one.
pub fn parse_coord(token: &str, line: usize) -> Coord {
    token
        .parse::<Coord>()
        .unwrap_or_else(|error| panic!("line {line}: {error}"))
}

/// Every stone token on a directive line, as coordinates.
pub fn parse_coords(rest: &str, line: usize) -> Vec<Coord> {
    rest.split_whitespace()
        .map(|token| parse_coord(token, line))
        .collect()
}
