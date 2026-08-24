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
//!
//! # RULE9-JUSTIFICATION: one schema, over the one document it describes
//! (CLAUDE.md rule 9).
//!
//! Every type here is a section or a field of ONE `toml` document
//! (`[engine]`, `[search]`, `[search.candidate_policy]`, `[eval]`,
//! `[instrument]`, `[play]`), and the three properties in this module's own
//! doc — complete, closed, versioned — are stated once and hold for all of
//! them together. Splitting by section would separate a table
//! (`[search.candidate_policy]`) from the enum tag (`kind`) that selects its
//! shape, or the schema from the one comment that states what "complete"
//! and "closed" mean for every field below it; a reader checking whether a
//! new key is schema-complete would then read two files to confirm one
//! property. It grows again only if the document gains a new top-level
//! section.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{self, EngineError};

/// The configuration schema version this build understands.
///
/// 2 since WP-1.4: the schema gained the required `[play]` section, and a
/// version-1 document must fail by version rather than by a puzzling
/// missing-key error (docs/decisions.md D-16).
pub const SCHEMA_VERSION: u32 = 2;

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

/// Largest movetime overshoot allowance this build accepts, in milliseconds.
///
/// A rejection bound like [`MAX_CANDIDATE_RADIUS`], not a value: it catches the
/// typo class — an epsilon longer than the budget it excuses — offline. The
/// promise itself (`go movetime N` answers within N + epsilon) is validated by
/// the release movetime gate, and raising the config value to green a failing
/// gate is a post-hoc threshold move CLAUDE.md forbids.
pub const MAX_MOVETIME_EPSILON_MS: u64 = 1000;

/// Largest search candidate radius this build accepts.
///
/// A sanity ceiling that catches typos and absurd values. It is deliberately
/// unrelated to the rules' radius-8 legal region, which lives in pistol-core;
/// the two radii are different concepts and are never compared
/// (CLAUDE.md rule 2, docs/decisions.md D-20).
pub const MAX_CANDIDATE_RADIUS: u32 = 64;

/// Largest `q_depth_turns` this build accepts (WP-1.6,
/// docs/wp16_quiescence_design.md §6).
///
/// A rejection bound like [`MAX_CANDIDATE_RADIUS`], not a value: the shipped
/// number is a closed enum of tried values decided by SPRT alone, never
/// picked here. `pistol_search`'s principal-variation table is fixed-size
/// and sized to cover a quiescence chain up to 16 turns deep
/// (`pistol_search::search::MAX_Q_EXTENSION_PLIES`, private to that crate) —
/// this ceiling is chosen well inside that headroom; raising it past 8 is a
/// `pistol-search` sizing change too, not a config-only one.
pub const MAX_Q_DEPTH_TURNS: u32 = 8;

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
    /// Settings that only bind in play mode, but are always stated — the same
    /// completeness rule that puts `[instrument]` in a play document.
    pub play: PlaySection,
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
    /// Threat-first staged pair generation (docs/decisions.md D-310;
    /// `docs/experiments/U3_tier_t.md` §10 is this document's schema, the one
    /// place the count of staged documents and their shape is stated).
    #[serde(rename = "staged")]
    Staged {
        /// Hex distance the fallback's quiet ball reaches, in
        /// `1..=`[`MAX_CANDIDATE_RADIUS`].
        quiet_radius: u32,
        /// Stage Q's own knob: the first batch's quiet-cell count. Validated
        /// for schema completeness against `U3_tier_t.md` §10; this D-scope's
        /// search does not read it (docs/decisions.md D-353 — stage Q's
        /// widening schedule is not armed).
        quiet_top_k: u64,
        /// Stage Q's own knob: cumulative quiet-cell batch boundaries after
        /// the first, strictly increasing, each greater than `quiet_top_k`.
        /// Validated for schema completeness; not read by this D-scope's
        /// search (docs/decisions.md D-353).
        widen_schedule: Vec<u64>,
        /// `LAW-SUPPORT`'s threshold for the side to move's own qualifying
        /// windows: 2 or 3 (`U3_tier_t.md` §6.1, the THRESHOLD reading).
        tier_t_own_count: u8,
        /// `LAW-SUPPORT`'s threshold for the opponent's qualifying windows:
        /// 2 or 3.
        tier_t_opponent_count: u8,
        /// How many further whole turns a threat-only quiescence extension
        /// may grant at a horizon, in `0..=`[`MAX_Q_DEPTH_TURNS`]. `0`
        /// disables the extension: the horizon's free win-now and
        /// `LAW-OVERLOAD` checks still run (WP-1.6,
        /// docs/wp16_quiescence_design.md §6), but no turn is ever granted.
        q_depth_turns: u32,
        /// Which of `docs/wp16_quiescence_design.md` §3's two gate triggers
        /// may grant an extension (D-396: gated so the first SPRT experiment
        /// tests the cheapest coherent hypothesis, not the compound one
        /// D-395 measured). Trigger (a) (win-now) and `LAW-OVERLOAD`'s
        /// zero-cost shortcut are unconditional either way — this field
        /// gates only trigger (b) (`DefensiveOnly`) versus triggers (b) and
        /// (c) together (`DefensiveAndOffensive`).
        q_triggers: QTriggers,
    },
}

/// Which quiescence gate triggers may grant an extension
/// (`[search.candidate_policy.q_triggers]`, D-396).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QTriggers {
    /// Trigger (b) only (`docs/wp16_quiescence_design.md` §3.2, `LAW-FORCE`'s
    /// forced-reply case) — the cheaper of the two, per D-395's own cost
    /// accounting, and the first SPRT experiment's own seat (D-396).
    DefensiveOnly,
    /// Trigger (b) and trigger (c) (`docs/wp16_quiescence_design.md` §3.3,
    /// the offensive activation case) — the compound configuration D-395
    /// measured. Licensed as a future experiment, not scheduled by this
    /// dispatch (D-396).
    DefensiveAndOffensive,
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

/// `[play]`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaySection {
    /// The overshoot allowance of the movetime ceiling, in milliseconds:
    /// `go movetime N` answers within N + this. In `1..=`
    /// [`MAX_MOVETIME_EPSILON_MS`].
    ///
    /// A PROMISE the search mechanism must measure under, not a knob the search
    /// reads: the internal deadline stays at N, and epsilon covers the bounded
    /// uninterruptible sections (the fallback stage, one node's candidate
    /// generation and ordering tail, the unwind, report I/O). It is advertised
    /// on the play-mode handshake so a driver can size its clamp. Its measured
    /// domain is recorded in docs/decisions.md beside the mechanism decision.
    pub movetime_epsilon_ms: u64,
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
