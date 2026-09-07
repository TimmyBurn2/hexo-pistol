//! `pistol-eval` — position evaluation behind the [`Eval`] trait.
//!
//! The contract is incremental by construction — apply/undo per placed stone,
//! integer arithmetic, side-relative value — so that swapping the backend never
//! becomes a search change (docs/decisions.md D-11, D-61). [`HandcraftedV0`] is
//! the v0 backend and [`Weights`] its committed table; the Stage-2 codebook net
//! will be an artifact and is never committed (CLAUDE.md rule 8).

pub mod error;
pub mod eval;
pub mod handcrafted;
pub mod weights;
pub mod window;

pub use error::EvalError;
pub use eval::{EVAL_MAX, Eval};
pub use handcrafted::{EVAL_DESYNC, HandcraftedV0};
pub use weights::{DECIDED_WINDOW_VALUE, WEIGHTS_SCHEMA_VERSION, Weights};
pub use window::{WINDOW_LEN, WINDOWS_PER_CELL, Window, windows_through};
