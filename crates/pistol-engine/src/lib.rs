//! `pistol-engine` — composition root and the seam everything else wraps.
//!
//! This crate owns the [`Engine`] trait (`new_game` / `set_position` /
//! `go(Budget) -> SearchOutcome`) and the wiring that builds a search from a
//! config. It is the only seam the future API layer adapts (CLAUDE.md rule 11):
//! pistol-api stays empty until that layer is specified, and the line protocol
//! in pistol-cli is this trait spelled as text (docs/decisions.md D-5).
//!
//! - [`config`] — the complete, explicit, schema-versioned configuration. Every
//!   field is required, unknown fields are rejected, and no tunable has a
//!   code-side default.
//! - [`budget`] — the closed set of ways to tell the engine when to stop. An
//!   absent budget is an error, never a fallback.
//! - [`error`] — the named failures. Nothing in this workspace fails silently.
//! - [`engine`] — the trait, and what each verb promises.
//! - [`position`] — a stated position, and the replay through the rules that is
//!   the only way one becomes a game (docs/decisions.md D-42).
//! - [`instance`] — [`Pistol`], the engine those parts compose into.
//!
//! # Determinism
//!
//! In instrument mode nothing here may influence a move choice
//! nondeterministically (CLAUDE.md rule 4). This crate reads a clock in exactly
//! one place — translating a wall-clock budget into the instant it expires at,
//! which instrument mode refuses to be given at all (docs/decisions.md D-22,
//! D-73) — and holds no other nondeterministic state.

pub mod budget;
pub mod config;
pub mod engine;
pub mod error;
pub mod instance;
pub mod position;
pub mod position_token;

mod position_set_token;
mod validate;

pub use budget::Budget;
pub use config::{Config, EngineMode, SCHEMA_VERSION};
pub use engine::Engine;
pub use error::EngineError;
pub use instance::Pistol;
pub use position::PositionSpec;
pub use position_token::ParsePositionError;

// The search's reporting vocabulary, re-exported because `go` hands it out.
// A caller of this seam should not have to depend on pistol-search to read what
// the seam returns (CLAUDE.md rule 11).
pub use pistol_search::score::{MATE, ScoreKind, classify};
pub use pistol_search::{MAX_DEPTH_TURNS, SearchInfo, SearchOutcome};
