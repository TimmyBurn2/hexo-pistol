//! `pistol-core` — the one source of game truth.
//!
//! This crate owns, and no other crate may re-implement (CLAUDE.md rule 2):
//!
//! - the hex lattice: axial coordinates [`Coord`] `(q, r)`, the three line
//!   [`Axis`]es, six neighbours per cell, unbounded board (rule 1);
//! - win detection ([`win`]): six or more contiguous own stones along one axis,
//!   overlines included, decided the instant a single placed stone completes a
//!   line (rules 2 and 4);
//! - the turn structure ([`GameState`]): one stone on turn 1, two stones on
//!   every later turn, with the second stone of a winning turn never played
//!   (rules 3 and 4);
//! - legal placement ([`Board::in_legal_region`]): the radius-[`LEGAL_RADIUS`]
//!   legal region, as a pinned constant (rule 5). It is a game rule, never a
//!   search knob, and it is never compared with the search candidate radius
//!   that lives in the engine config (docs/decisions.md D-20);
//! - the turn as a value ([`Turn`]) and every turn a position has
//!   ([`generate_turns`]), with [`GameState::make_turn`] /
//!   [`GameState::unmake_turn`] playing and taking one back, and [`perft`]
//!   counting them as the movegen oracle (CLAUDE.md rule 7);
//! - the identity of a position ([`Key128`], [`GameState::key`]): a computed,
//!   lazy zobrist key over an unbounded board, carrying the side to move and
//!   the intra-turn phase, which the search's transposition table and the
//!   solver will both index by (docs/decisions.md D-8).
//!
//! Still to come, in this crate and nowhere else: when the opening book needs
//! them in Stage 5, the twelve-fold hex symmetry transforms, which are geometry
//! and so belong here rather than in the book generator.
//!
//! Dependencies: `std` only, permanently — dev-dependencies included, which is
//! why the test tree carries its own SHA-256 for pinning fixtures
//! (docs/decisions.md D-37).
//!
//! # Determinism
//!
//! Nothing here holds per-process random state and nothing exposes an order
//! that could differ between two runs of the same position: occupancy is a
//! `BTreeMap` ordered by `(q, r)`, axis and neighbour iteration are fixed
//! arrays, and [`Coord`]'s derived ordering is the lexicographic tie-break the
//! rest of the engine uses (CLAUDE.md rule 4, docs/decisions.md D-7, D-32).
//!
//! # Failure
//!
//! A refusal a caller could reasonably provoke — an occupied cell, a cell
//! outside the legal region, a stone after the game is decided — is a named
//! [`CoreError`]. A violated internal invariant is a panic carrying a named
//! token ([`coord::COORD_OVERFLOW`], [`win::WIN_CHECK_ON_EMPTY_CELL`],
//! [`state::TURN_OVERFLOW`], [`state::HISTORY_DESYNC`]), because it is a bug in
//! pistol rather than an answer to a question anyone asked. Neither is ever
//! silent (CLAUDE.md rule 3).

pub mod axis;
pub mod board;
pub mod coord;
pub mod error;
pub mod movegen;
pub mod perft;
pub mod play;
pub mod rules;
pub mod state;
pub mod turn;
pub mod win;
pub mod zobrist;

pub use axis::{Axis, NEIGHBOUR_DIRECTIONS};
pub use board::{Board, Color};
pub use coord::{Coord, ParseCoordError};
pub use error::CoreError;
pub use movegen::{generate_turns, legal_placements};
pub use perft::perft;
pub use rules::{
    FIRST_TURN, FIRST_TURN_STONES, LEGAL_RADIUS, TURN_STONES, WIN_LEN, stones_in_turn,
};
pub use state::GameState;
pub use turn::{Outcome, ParseTurnError, Phase, PlyOutcome, Turn};
pub use win::{Run, wins_at};
pub use zobrist::{Key128, ZOBRIST_SEED, cell_key, from_scratch_key, phase_key, side_key};
