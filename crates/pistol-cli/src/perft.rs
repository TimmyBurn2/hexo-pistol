//! The `perft` command: counting turns, as the movegen oracle counts them.
//!
//! Thin on purpose. The count itself is `pistol_core::perft`, the oracle it is
//! checked against is the brute-force reference in pistol-core's test tree, and
//! the gate is `tools/perft_check.sh` (CLAUDE.md rule 7, docs/decisions.md
//! D-12, D-54). What this adds is a way to ask for one number from a shell.
//!
//! The position is stated as a **ply list**, which is the rules' own canonical
//! encoding (docs/decisions.md D-6) and the same form the perft fixture uses, so
//! an operator can copy a fixture case's `plies` line and check it by hand.
//! Deliberately not a `position` protocol tail: perft is a rules tool, and a
//! decided position — which has no turns at any depth, and which the engine
//! refuses to stand on — is one of the cases worth counting.

use pistol_core::{Coord, CoreError, GameState, perft};

/// Count the turn sequences of `depth_turns` turns from the position `plies`
/// reaches.
pub fn count(plies: &[Coord], depth_turns: u32) -> Result<u64, CoreError> {
    let mut state = GameState::from_plies(plies)?;
    perft(&mut state, depth_turns)
}

/// Parse a whitespace-separated ply list, by pistol-core's own stone-token
/// grammar (docs/decisions.md D-39). An empty list is the empty board.
pub fn parse_plies(text: &str) -> Result<Vec<Coord>, String> {
    text.split_whitespace()
        .map(|token| {
            token
                .parse::<Coord>()
                .map_err(|error| format!("`{token}`: {}", error.why))
        })
        .collect()
}
