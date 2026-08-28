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
