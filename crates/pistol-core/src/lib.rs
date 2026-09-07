//! `pistol-core` — the one source of game truth: the hex lattice, win
//! detection, the turn structure, legal placement, the line windows a stone
//! sits in, the turn as a value, and a position's identity.
//!
//! No other crate re-implements any of it (CLAUDE.md rule 2), and the
//! radius-[`LEGAL_RADIUS`] legal region is a game RULE — never a search knob,
//! and never compared with the engine config's candidate radius
//! (docs/decisions.md D-20).
//!
//! Dependencies: `std` only, permanently, dev-dependencies included — which is
//! why the test tree carries its own SHA-256 (docs/decisions.md D-37).

pub mod axis;
pub mod board;
pub mod coord;
pub mod error;
pub mod movegen;
pub mod perft;
pub mod play;
pub mod rules;
pub mod state;
pub mod symmetry;
pub mod turn;
pub mod win;
pub mod window;
pub mod zobrist;

pub use axis::{Axis, NEIGHBOUR_DIRECTIONS};
pub use board::{Board, Player};
pub use coord::{Coord, ParseCoordError};
pub use error::CoreError;
pub use movegen::{generate_turns, legal_placements};
pub use perft::perft;
pub use rules::{
    FIRST_TURN, FIRST_TURN_STONES, LEGAL_RADIUS, TURN_STONES, WIN_LEN, stones_in_turn,
};
pub use state::GameState;
pub use symmetry::{Symmetry, canonical_form, canonical_key, canonical_sequence};
pub use turn::{Outcome, ParseTurnError, Phase, PlyOutcome, Turn};
pub use win::{Run, wins_at};
pub use window::{WINDOW_LEN, WINDOWS_PER_CELL, Window, windows_through, windows_through_indexed};
pub use zobrist::{Key128, ZOBRIST_SEED, cell_key, from_scratch_key, phase_key, side_key};
