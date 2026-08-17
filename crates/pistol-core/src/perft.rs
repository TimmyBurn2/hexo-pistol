//! Perft: how many distinct sequences of `n` turns a position has.
//!
//! Perft is the movegen oracle (CLAUDE.md rule 7). Its counts mean nothing on
//! their own — they are compared, position for position and depth for depth,
//! against an independently written brute-force generator in the test tree
//! (docs/decisions.md D-12), and any difference is a bug in one of the two.
//!
//! # What is counted
//!
//! **Turns**, not stones and not plies (docs/decisions.md D-9). One count of
//! one is one unordered pair of cells, or the single stone that turn 1 and
//! rule 4 make a whole turn. The two orderings of a pair are one turn and are
//! counted once; a first stone that completes a line and the pair that would
//! have contained it are different turns and are counted separately, because
//! they are.
//!
//! `perft(0)` is 1: the position itself is the one sequence of no turns. A
//! decided position generates no turns, so it contributes 0 at any depth beyond
//! the one that reached it — won lines are counted where they end and are never
//! expanded, which is the usual convention for a terminal node and is what rule
//! 4 means here.
//!
//! # Cost
//!
//! The branching factor is the size of the legal region squared over two, and
//! rule 5's region is 217 cells around a lone stone before any of them are
//! played: a one-stone position already has tens of thousands of turns, and a
//! midgame cloud has hundreds of thousands. Two full levels of that is 10^9
//! turns and more. Depth is therefore not free the way it is on a small board —
//! the test tree's fixture header records which depths are reachable, and the
//! ones that are not are a measured structural fact rather than a gap in the
//! oracle.
//!
//! # Not a search
//!
//! Nothing here prunes, orders, caches or narrows. Perft counts the full rule-5
//! legal region, never a search candidate radius (CLAUDE.md rule 2).

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
