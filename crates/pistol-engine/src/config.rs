//! The engine configuration schema.
//!
//! Three properties hold for every type below and are enforced, not merely
//! intended (CLAUDE.md rule 1):
//!
//! 1. **Complete.** Every field is required. There is no `serde` field default,
//!    no `Default` implementation, and no code path that invents a tunable's
//!    value. A value exists because an operator wrote it in a file.
//! 2. **Closed.** Every struct denies unknown fields, so a mistyped key is a
//!    loud error rather than a setting that silently does nothing.
//! 3. **Versioned.** `schema_version` is checked against [`SCHEMA_VERSION`], so
//!    a config written for another build fails at load rather than at the first
//!    divergent behaviour.
//!
//! Cross-field rules live in `validate.rs` and run in [`Config::validate`].

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{self, EngineError};

/// The configuration schema version this build understands.
pub const SCHEMA_VERSION: u32 = 1;

/// Smallest transposition table this build accepts, in bytes.
///
/// This is a rejection bound, not a value: no config is completed with it, and
/// a config that omits `search.tt_bytes` is still an error.
pub const MIN_TT_BYTES: u64 = 1 << 20;

/// Largest transposition table this build accepts, in bytes.
///
/// The same kind of bound as [`MIN_TT_BYTES`] and [`MAX_CANDIDATE_RADIUS`]: a
/// rejection, not a value. It catches the typo class — a document asking for a
/// terabyte — offline and deterministically, which is what config validation is
/// allowed to do (docs/decisions.md D-21, D-66). It cannot catch "more than this
/// machine has", because that answer is not the same on two machines and
/// validation may not ask; the table refuses that one itself, by name, when it
/// tries to allocate.
pub const MAX_TT_BYTES: u64 = 1 << 36;

/// Largest search candidate radius this build accepts.
///
/// A sanity ceiling that catches typos and absurd values. It is deliberately
/// unrelated to the rules' radius-8 legal region, which lives in pistol-core;
/// the two radii are different concepts and are never compared
/// (CLAUDE.md rule 2, docs/decisions.md D-20).
pub const MAX_CANDIDATE_RADIUS: u32 = 64;

/// A complete engine configuration.
///
/// Parsing an incomplete document is an error, never an empty-but-usable
/// configuration:
///
/// ```
/// use pistol_engine::config::Config;
/// assert!(Config::parse_unvalidated("").is_err());
/// ```
///
/// and there is no way to conjure one without a document, so this must not
/// compile:
///
/// ```compile_fail
/// use pistol_engine::config::Config;
/// let _ = Config::default();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Schema version of this document; must equal [`SCHEMA_VERSION`].
    pub schema_version: u32,
    /// How the engine as a whole behaves.
    pub engine: EngineSection,
    /// Search sizing and candidate selection.
    pub search: SearchSection,
    /// Which evaluation backend to build, and from which weights.
    pub eval: EvalSection,
    /// Settings that only bind in instrument mode, but are always stated.
    pub instrument: InstrumentSection,
}

/// `[engine]`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineSection {
    /// Reproducible measurement, or playing strength.
    pub mode: EngineMode,
}

/// How the engine trades reproducibility against strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum EngineMode {
    /// Single-threaded, fixed-budget, stable tie-breaking, CPU evaluation.
    /// Every strength claim comes from a run in this mode.
    #[serde(rename = "instrument")]
    Instrument,
    /// Free to use every resource the deployment budget allows.
    #[serde(rename = "play")]
    Play,
}

impl EngineMode {
    /// The document's spelling of this mode, which is also the protocol's.
    ///
    /// It must match the `serde` rename above; `mode_token_matches_the_document`
    /// pins that it does. The two exist separately because a `serde` attribute
    /// cannot be read at run time, and the handshake has to name the mode
    /// somehow — spelling it twice in two files is what this avoids.
    pub const fn token(self) -> &'static str {
        match self {
            EngineMode::Instrument => "instrument",
            EngineMode::Play => "play",
        }
    }
}

/// `[search]`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchSection {
    /// Transposition table size in bytes. Power of two, at least
    /// [`MIN_TT_BYTES`]; see docs/decisions.md D-19.
    pub tt_bytes: u64,
    /// Which cells the search is allowed to consider.
    pub candidate_policy: CandidatePolicy,
}

/// `[search.candidate_policy]`, tagged by `kind`.
///
/// This is a search knob and is config, never a literal. It restricts what the
/// search looks at; it does not decide what is legal.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum CandidatePolicy {
    /// Every empty cell within `radius` of some stone.
    #[serde(rename = "radius")]
    Radius {
        /// Hex distance from the nearest stone, in `1..=`[`MAX_CANDIDATE_RADIUS`].
        radius: u32,
    },
}

/// `[eval]`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalSection {
    /// Which implementation of the `Eval` trait to build.
    pub backend: EvalBackend,
    /// Where that backend reads its weights. Checked for shape here and opened
    /// by pistol-eval, which raises the loud error if it is missing or corrupt
    /// (docs/decisions.md D-21).
    pub weights_file: PathBuf,
}

/// The evaluation backends this build can construct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum EvalBackend {
    /// Handcrafted three-axis line-window pattern tables.
    #[serde(rename = "handcrafted_v0")]
    HandcraftedV0,
}

impl EvalBackend {
    /// The document's spelling of this backend. See [`EngineMode::token`].
    pub const fn token(self) -> &'static str {
        match self {
            EvalBackend::HandcraftedV0 => "handcrafted_v0",
        }
    }
}

/// `[instrument]`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstrumentSection {
    /// Search threads. Must be 1 whenever the mode is
    /// [`EngineMode::Instrument`] — the determinism law admits no thread races
    /// on a choice path (CLAUDE.md rule 4).
    pub threads: u16,
    /// How equal-scoring moves are ordered against each other.
    pub tie_break: TieBreak,
}

/// How the engine breaks a tie between equally scored moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum TieBreak {
    /// By coordinate order. Stable across runs, machines and thread counts.
    #[serde(rename = "lexicographic")]
    Lexicographic,
}

impl Config {
    /// Read, parse and validate a config file. This is the entry point; it
    /// never yields a configuration that has not passed [`Config::validate`].
    pub fn load(path: &Path) -> Result<Self, EngineError> {
        let text = std::fs::read_to_string(path).map_err(|io| {
            EngineError::config(path.display().to_string(), format!("cannot read: {io}"))
        })?;
        let config = Self::parse_unvalidated(&text)?;
        config.validate()?;
        Ok(config)
    }

    /// Parse a document without applying cross-field rules.
    ///
    /// The blunt name is the point: a caller that skips [`Config::validate`] is
    /// visible at the call site (docs/decisions.md D-17). Prefer
    /// [`Config::load`].
    pub fn parse_unvalidated(text: &str) -> Result<Self, EngineError> {
        // Two stages, for two kinds of error. The first parse reports syntax
        // with a line and column; the second reports schema violations with the
        // key path that `serde_path_to_error` recovers.
        let document: toml::Value = toml::from_str(text).map_err(error::from_toml_syntax)?;
        serde_path_to_error::deserialize(document).map_err(error::from_path_error)
    }
}
