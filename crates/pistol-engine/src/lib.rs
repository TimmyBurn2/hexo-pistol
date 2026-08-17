//! `pistol-engine` — composition root and the seam everything else wraps.
//!
//! This crate will own the `Engine` trait (`new_game` / `set_position` /
//! `go(Budget) -> BestMove + SearchInfo`) and the wiring that builds a search
//! from a config. It is the only seam the future API layer adapts
//! (CLAUDE.md rule 11).
//!
//! WP-01 lands the three pieces the rest of the workspace has to agree on
//! before any of it exists:
//!
//! - [`config`] — the complete, explicit, schema-versioned configuration. Every
//!   field is required, unknown fields are rejected, and no tunable has a
//!   code-side default.
//! - [`budget`] — the closed set of ways to tell the engine when to stop. An
//!   absent budget is an error, never a fallback.
//! - [`error`] — the named failures. Nothing in this workspace fails silently.

pub mod budget;
pub mod config;
pub mod error;

mod validate;

pub use budget::Budget;
pub use config::{Config, SCHEMA_VERSION};
pub use error::EngineError;
