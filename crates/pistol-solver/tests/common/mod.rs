//! Shared test scaffolding: the sha-pinned fixture, the hash that pins it, the
//! two references and the playout generator.
//!
//! The loaders are strict on purpose. A fixture line they do not understand is
//! a panic naming the line number, and so is a fixture record that is missing an
//! expectation: a golden file that is quietly half-read reports a pass for cases
//! nobody ran (CLAUDE.md rule 3, docs/decisions.md D-37's own list, which
//! refuses an unknown directive AND a missing verdict).
//!
//! The SHA-256 here is copied from pistol-core's test tree rather than depended
//! on, because this crate takes no dev-dependency either.
#![allow(dead_code)] // each test binary uses a subset of these helpers.

pub mod fixtures;
pub mod patterns;
pub mod plans;
pub mod playouts;
pub mod reference;
pub mod region;
pub mod sha256;

use std::fs;
use std::path::PathBuf;

use pistol_core::window::Window;
use pistol_core::{Axis, Coord, GameState, Player};
use pistol_solver::ThreatState;

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

/// The stone-token grammar lives in pistol-core and this tree does not write a
/// second one (docs/decisions.md D-39).
pub fn parse_coord(token: &str, line: usize) -> Coord {
    token
        .parse::<Coord>()
        .unwrap_or_else(|error| panic!("line {line}: {error}"))
}

/// Every stone token on a line, as coordinates.
pub fn parse_coords(rest: &str, line: usize) -> Vec<Coord> {
    rest.split_whitespace()
        .map(|token| parse_coord(token, line))
        .collect()
}

/// `p1` or `p2`.
pub fn parse_player(token: &str, line: usize) -> Player {
    match token {
        "p1" => Player::P1,
        "p2" => Player::P2,
        other => panic!("line {line}: `{other}` is not a side; write p1 or p2"),
    }
}

/// The axis names are the enum's own spellings, so a fixture and a `{:?}` agree.
pub fn parse_axis(token: &str, line: usize) -> Axis {
    match token {
        "ConstQ" => Axis::ConstQ,
        "ConstR" => Axis::ConstR,
        "ConstS" => Axis::ConstS,
        other => panic!("line {line}: `{other}` is not an axis; write ConstQ, ConstR or ConstS"),
    }
}

/// `<axis>@<q,r>`, the fixture's window token.
pub fn parse_window(token: &str, line: usize) -> Window {
    let (axis, start) = token
        .split_once('@')
        .unwrap_or_else(|| panic!("line {line}: `{token}` is not `<axis>@<q,r>`"));
    let axis = parse_axis(axis, line);
    let start = parse_coord(start, line);
    Window::new(axis, start)
        .unwrap_or_else(|| panic!("line {line}: `{token}` names no window on this lattice"))
}

/// How a window is written in the fixture and in a failure message.
pub fn window_token(window: Window) -> String {
    format!("{:?}@{}", window.axis, window.start)
}

/// How a list of windows is written: `-` when there are none, so that an empty
/// answer is spelled rather than blank.
pub fn window_list(windows: &[Window]) -> String {
    if windows.is_empty() {
        return String::from("-");
    }
    windows
        .iter()
        .map(|&window| window_token(window))
        .collect::<Vec<_>>()
        .join(" ")
}

/// The same for cells.
pub fn cell_list(cells: &[Coord]) -> String {
    if cells.is_empty() {
        return String::from("-");
    }
    cells
        .iter()
        .map(|cell| cell.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Play a ply list through the rules and carry a threat state alongside it.
///
/// The MOVER comes from `GameState` on every ply — never from ply parity, which
/// is not this game's turn structure (rule 3) — and legality, phase and outcome
/// come from core as well. A test that built its own board would be testing the
/// threat state against a position the rules do not have.
pub fn play(plies: &[Coord]) -> (GameState, ThreatState) {
    let mut game = GameState::new_game();
    let mut threats = ThreatState::new();
    for (index, &at) in plies.iter().enumerate() {
        let mover = game.to_move();
        game.place(at)
            .unwrap_or_else(|error| panic!("ply {index} at {at}: {error}"));
        threats.apply(at, mover);
    }
    (game, threats)
}
