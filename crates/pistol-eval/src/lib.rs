//! `pistol-eval` — position evaluation behind one trait.
//!
//! This crate owns the [`Eval`] trait and its implementations: the v0
//! handcrafted three-axis line-window pattern tables now, an incremental
//! pattern-codebook net in Stage 2. The contract is incremental by construction
//! — apply/undo per placed stone, integer arithmetic, side-relative value — so
//! that swapping the backend never becomes a search change
//! (docs/decisions.md D-11, D-61).
//!
//! - [`Eval`] is the contract, and [`EVAL_MAX`] the band a static value lives
//!   in; the mate band above it belongs to the search (D-3).
//! - [`HandcraftedV0`] is the v0 backend: every length-[`WINDOW_LEN`] window on
//!   each of the three axes, a window holding both players dead, one carried
//!   sum, [`WINDOWS_PER_CELL`] entries touched per stone.
//! - [`Weights`] is its table, read from committed configuration
//!   (`configs/eval_v0_weights.toml`) with every entry required and no code-side
//!   default (CLAUDE.md rule 1).
//!
//! The v0 weight table is committed configuration, not an artifact: it is a
//! handful of integers an operator can read and edit. The Stage-2 codebook net
//! *is* an artifact, and that one is never committed (CLAUDE.md rule 8). Either
//! way, a missing or malformed weights file is a loud load-time [`EvalError`]
//! raised here and not a config parse error — config validation stays pure and
//! offline (docs/decisions.md D-21).
//!
//! # Determinism
//!
//! Integer arithmetic throughout and no interior mutability. The window
//! bookkeeping is a hash map, and it is clear of the determinism law two ways:
//! its hasher is SEEDLESS BY CONSTRUCTION, and nothing in this crate iterates
//! the map on a path that reaches a value, so no iteration order can differ
//! between two runs of the same position (CLAUDE.md rule 4, D-7, D-32, D-498).
//! Nothing here reads a clock, a thread count, or an environment variable.
//!
//! # Failure
//!
//! A malformed weights document is a named [`EvalError`] carrying the key an
//! operator must edit. Being told about a stone that contradicts what an eval
//! already holds is not operator input — it means a caller's board and its eval
//! have drifted — so it panics with [`EVAL_DESYNC`](handcrafted::EVAL_DESYNC)
//! rather than returning an error nobody could handle (CLAUDE.md rule 3).

pub mod error;
pub mod eval;
pub mod handcrafted;
pub mod weights;
pub mod window;
mod window_map;

pub use error::EvalError;
pub use eval::{EVAL_MAX, Eval};
pub use handcrafted::{EVAL_DESYNC, HandcraftedV0};
pub use weights::{DECIDED_WINDOW_VALUE, WEIGHTS_SCHEMA_VERSION, Weights};
pub use window::{WINDOW_LEN, WINDOWS_PER_CELL, Window, windows_through};
