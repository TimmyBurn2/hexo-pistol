use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::ArenaError;

/// The arena schema version this build understands.
///
/// 2: `[run]` gained the required `openings_skip` (docs/decisions.md D-202).
pub const ARENA_SCHEMA_VERSION: u32 = 2;

/// A complete arena configuration.
///
/// Parsing an incomplete document is an error, never an empty-but-usable one:
///
/// ```
/// use pistol_arena::config::ArenaConfig;
/// assert!(ArenaConfig::parse_unvalidated("").is_err());
/// ```
///
/// and there is no way to conjure one without a document, so this must not
/// compile:
///
/// ```compile_fail
/// use pistol_arena::config::ArenaConfig;
/// let _ = ArenaConfig::default();
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArenaConfig {
    /// Schema version of this document; must equal [`ARENA_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// What to play, and how much of it.
    pub run: RunSection,
    /// What each engine is given per turn.
    pub budget: BudgetSection,
    /// The hypotheses and error rates the run is judged against.
    pub sprt: SprtSection,
    /// The first engine.
    pub engine_a: EngineSection,
    /// The second engine.
    pub engine_b: EngineSection,
}

/// `[run]`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunSection {
    /// The openings fixture. Every opening in it is played from both seats.
    pub openings_file: PathBuf,
    /// How many openings to take, starting after `openings_skip`.
    ///
    /// A contiguous window rather than a selection, because the fixture is
    /// emitted in content-hash order precisely so that any contiguous window is
    /// a sample rather than a rating tail (docs/decisions.md D-143). This is
    /// also the run's whole size: each opening is exactly one pair, so the game
    /// cap is twice this and is derived rather than stated again
    /// (docs/decisions.md D-157).
    pub openings_take: usize,
    /// How many openings to SKIP before taking, so two runs can draw DISJOINT
    /// samples from one book: skip 0/take t and skip t/take t share nothing
    /// (docs/decisions.md D-202). `skip + take` must fit inside the file, or
    /// the run is a different experiment from the one written down. Skip
    /// changes which games are played, so it is part of `experiment_sha256`.
    pub openings_skip: usize,
    /// How many turns a game may run for before it is recorded `capped`.
    ///
    /// An evaluation horizon, never a game rule (CLAUDE.md game rule 6). The
    /// engine is never told about it.
    pub turn_cap: u32,
    /// How many games may be in flight at once.
    ///
    /// Scheduling may not reach any field of the report's verdict block
    /// (docs/decisions.md D-161).
    pub n_workers: usize,
    /// How long an engine may answer nothing before the RUN is abandoned.
    ///
    /// A liveness device and never an adjudication: it can end a run, and it
    /// can never produce a game result (docs/decisions.md D-159). Size it
    /// against the worst turn at the worker count you are actually running,
    /// because contention scales it.
    pub hang_timeout_ms: u64,
}

/// `[budget]`, tagged by `kind`.
///
/// All three of the engine's budget kinds are spelled here, INCLUDING the one
/// this crate refuses. Leaving `movetime_ms` out would make `serde` answer
/// "unknown variant", which points at the schema; the refusal has to point at
/// the reason (docs/decisions.md D-153).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum BudgetSection {
    /// A fixed depth in turns.
    #[serde(rename = "depth_turns")]
    DepthTurns {
        /// Turns.
        value: u32,
    },
    /// A fixed node count.
    #[serde(rename = "nodes")]
    Nodes {
        /// Nodes.
        value: u64,
    },
    /// Wall-clock milliseconds. Parsed so it can be refused by name.
    #[serde(rename = "movetime_ms")]
    MovetimeMs {
        /// Milliseconds.
        value: u64,
    },
}

impl BudgetSection {
    /// The `go` line this budget asks for, as the line protocol spells it
    /// (docs/decisions.md D-5).
    ///
    /// Only reachable for the two instrument budgets: `validate` refuses the
    /// third before any engine is started.
    pub fn go_line(self) -> Option<String> {
        match self {
            BudgetSection::DepthTurns { value } => Some(format!(
                "{} {} {value}",
                pistol_cli::protocol::GO,
                pistol_cli::budget_token::DEPTH_TURNS_BUDGET
            )),
            BudgetSection::Nodes { value } => Some(format!(
                "{} {} {value}",
                pistol_cli::protocol::GO,
                pistol_cli::budget_token::NODES_BUDGET
            )),
            BudgetSection::MovetimeMs { .. } => None,
        }
    }

    /// How the report names this budget.
    pub fn report_tokens(self) -> (&'static str, u64) {
        match self {
            BudgetSection::DepthTurns { value } => ("depth_turns", u64::from(value)),
            BudgetSection::Nodes { value } => ("nodes", value),
            BudgetSection::MovetimeMs { value } => ("movetime_ms", value),
        }
    }
}

/// `[sprt]`.
///
/// Bounds are in NORMALIZED Elo, which is what makes the sample size roughly
/// independent of how many games are decisive.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SprtSection {
    /// The null hypothesis, in normalized Elo.
    pub elo0: f64,
    /// The alternative, in normalized Elo. Must exceed `elo0`.
    pub elo1: f64,
    /// Probability of accepting H1 when H0 holds.
    pub alpha: f64,
    /// Probability of accepting H0 when H1 holds.
    pub beta: f64,
}

/// `[engine_a]` / `[engine_b]`.
///
/// Two CONFIGURATIONS, which may be two builds. Nothing here is specific to any
/// engine other than pistol: both sides speak the line protocol, and an
/// external opponent is the bridge's job.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineSection {
    /// How the report names this side. Short, and distinct from the other's.
    pub label: String,
    /// The pistol CLI binary to run.
    pub binary: PathBuf,
    /// The SHA-256 the file at [`EngineSection::binary`] must have, lowercase
    /// hex.
    ///
    /// A PATH IS NOT AN IDENTITY. `target/release/pistol` is a different program
    /// after every build, and a run whose engine was rebuilt — or never built,
    /// with a stale file left at that path — is a different experiment reported
    /// under the old one's name (docs/decisions.md D-147, D-252). This is
    /// required rather than optional because an optional binding is a binding
    /// nobody has, and the four operator-run SPRT documents that CLAUDE.md rule
    /// 6 makes the judge of every search and eval change are exactly the ones
    /// that would have gone without it.
    ///
    /// Checked at RUN START, before either engine is spawned, and never at
    /// validation time: validation stays pure and offline (docs/decisions.md
    /// D-21), so what this crate validates is the SPELLING.
    pub binary_sha256: String,
    /// The engine config to run it with.
    pub config: PathBuf,
}

impl ArenaConfig {
    /// Read, parse and validate. Never yields an unvalidated configuration.
    pub fn load(path: &Path) -> Result<Self, ArenaError> {
        let text = std::fs::read_to_string(path).map_err(|io| {
            ArenaError::config(path.display().to_string(), format!("cannot read: {io}"))
        })?;
        let config = Self::parse_unvalidated(&text)?;
        config.validate()?;
        Ok(config)
    }

    /// Parse a document without applying cross-field rules.
    ///
    /// The blunt name is the point (docs/decisions.md D-17). Prefer
    /// [`ArenaConfig::load`].
    pub fn parse_unvalidated(text: &str) -> Result<Self, ArenaError> {
        // Two stages for two kinds of error, as pistol-engine does it: syntax
        // with a line and column, then schema violations with the key path
        // `serde_path_to_error` recovers (docs/decisions.md D-24).
        let document: toml::Value = toml::from_str(text)
            .map_err(|error| ArenaError::config("<document>", format!("not TOML: {error}")))?;
        serde_path_to_error::deserialize(document).map_err(|error| {
            let key = error.path().to_string();
            let key = if key.is_empty() || key == "." {
                String::from("<document>")
            } else {
                key
            };
            ArenaError::config(key, error.into_inner().to_string())
        })
    }
}
