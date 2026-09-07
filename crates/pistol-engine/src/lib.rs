//! `pistol-engine` — composition root, and the only seam the future API layer
//! adapts (CLAUDE.md rule 11).
//!
//! It owns the [`Engine`] trait and the wiring that builds a search from a
//! config. The line protocol in `pistol-cli` is this trait spelled as text
//! (docs/decisions.md D-5), and `pistol-api` stays empty until that layer is
//! specified.

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
pub use engine::{CensusRequest, Engine};
pub use error::EngineError;
pub use instance::Pistol;
pub use position::PositionSpec;
pub use position_token::ParsePositionError;

// The search's reporting vocabulary, re-exported because `go` hands it out.
// A caller of this seam should not have to depend on pistol-search to read what
// the seam returns (CLAUDE.md rule 11).
pub use pistol_search::score::{MATE, ScoreKind, classify};
pub use pistol_search::{
    CoverClass, MAX_DEPTH_TURNS, Provenance, SearchInfo, SearchOutcome, SolverCallCounters,
    StageCounters, TriggerAnswer, TriggerColumns, TriggerObservation, WidthHistogram,
};
