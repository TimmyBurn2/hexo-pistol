use crate::error::CoreError;
use crate::movegen::generate_turns;
use crate::state::GameState;
use crate::turn::Phase;

/// Named invariant: a perft count left the `u64` range.
pub const PERFT_OVERFLOW: &str = "PERFT_OVERFLOW";

/// The number of distinct sequences of `depth_turns` turns playable from
/// `state`.
///
/// The state is borrowed mutably because the walk plays and takes back every
/// turn it counts; it is left exactly as it was found, on the counted path and
/// on the error path alike (every refusal in
/// [`GameState::make_turn`](crate::GameState::make_turn) is atomic).
///
/// # Panics
///
/// With [`PERFT_OVERFLOW`] if the count exceeds `u64`.
pub fn perft(state: &mut GameState, depth_turns: u32) -> Result<u64, CoreError> {
    if depth_turns == 0 {
        // Counted at a turn boundary or not counted at all. The turn-level API is
        // defined only there (docs/decisions.md D-50), and answering `1` for a
        // half-played position would be this function's one unlocked door into a
        // position it is not defined on — a count where every other depth raises
        // `TurnInProgress` by name (CLAUDE.md rule 3).
        if state.phase() != Phase::First {
            return Err(CoreError::TurnInProgress { turn: state.turn() });
        }
        return Ok(1);
    }
    let turns = generate_turns(state)?;
    if depth_turns == 1 {
        return Ok(count(turns.len()));
    }
    let mut total: u64 = 0;
    for turn in turns {
        state.make_turn(turn)?;
        let below = perft(state, depth_turns - 1)?;
        state.unmake_turn()?;
        total = match total.checked_add(below) {
            Some(total) => total,
            None => panic!(
                "pistol-core invariant {PERFT_OVERFLOW}: counting turn sequences of depth \
                 {depth_turns} exhausted u64"
            ),
        };
    }
    Ok(total)
}

/// A generated turn count as a perft count.
fn count(turns: usize) -> u64 {
    match u64::try_from(turns) {
        Ok(count) => count,
        Err(_) => panic!("pistol-core invariant {PERFT_OVERFLOW}: {turns} turns exceed u64"),
    }
}
