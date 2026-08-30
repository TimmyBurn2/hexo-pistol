//! The match configuration: one TOML document, explicit and complete.
//!
//! Same discipline as the engine configs (CLAUDE.md rule 1): every field is
//! required, unknown fields are refused, and no tunable has a code-side
//! default. A match that is not fully described by its document is not run.
//!
//! Paths inside the document resolve against the repository root, which is
//! where `run_match.sh` runs from — the same convention the pistol binary's
//! own config paths follow.

use serde::Deserialize;

/// The only schema this harness has read so far.
const SCHEMA_VERSION: u32 = 1;

/// A match: two engines, N games, a turn cap, an output directory.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchConfig {
    /// Which document shape this is.
    pub schema_version: u32,
    /// Games to play. Seats alternate per game; engine A is p1 in game 1.
    pub games: u32,
    /// The evaluation horizon: a game that reaches this many turns without a
    /// decision is reported "capped", never a win.
    pub turn_cap: u32,
    /// The first engine.
    pub engine_a: EngineSpec,
    /// The second engine.
    pub engine_b: EngineSpec,
    /// Where transcripts, stderr and the report are written.
    pub output_dir: String,
}

/// One engine seat: a subprocess to drive, and its budget.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineSpec {
    /// Which client drives it: `pistol` (line protocol) or `sealbot`
    /// (JSON lines). The extension seam — a new engine is a new kind.
    pub kind: EngineKind,
    /// What appears in the report for this seat.
    pub label: String,
    /// The argv that starts one engine process (one per game).
    pub command: Vec<String>,
    /// The working directory for the process (repository-root-relative).
    pub cwd: String,
    /// pistol only: the node budget sent as `go nodes <n>`. Instrument mode.
    pub nodes: Option<u64>,
    /// pistol only: the wall budget sent as `go movetime <ms>`. Play mode.
    ///
    /// Exactly one of this and [`EngineSpec::nodes`] is present: the budget a
    /// seat runs under is the seat's, and a document that names two of them
    /// names none.
    pub movetime_ms: Option<u64>,
    /// sealbot only: the per-turn time budget handed to the shim.
    pub time_limit_seconds: Option<f64>,
    /// The per-turn wall cap: no answer by this many seconds is a forfeit.
    pub turn_timeout_seconds: f64,
}

/// The two clients this harness knows.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EngineKind {
    /// pistol's own line protocol, instrument mode.
    Pistol,
    /// The JSON-lines contract of `sealbot_shim.py`.
    Sealbot,
}

/// Refuse the document loudly rather than guess at any of it.
pub fn load(text: &str) -> Result<MatchConfig, String> {
    let config: MatchConfig =
        toml::from_str(text).map_err(|error| format!("bad match config: {error}"))?;
    if config.schema_version != SCHEMA_VERSION {
        return Err(format!(
            "match config schema_version {} is not {SCHEMA_VERSION}",
            config.schema_version
        ));
    }
    if config.games == 0 {
        return Err("match config games must be at least 1".to_string());
    }
    if config.turn_cap < 2 {
        return Err("match config turn_cap must be at least 2 (the engines are first asked at turn 2)".to_string());
    }
    check_engine(&config.engine_a, "engine_a")?;
    check_engine(&config.engine_b, "engine_b")?;
    if config.command_is_empty() {
        return Err("match config command must not be empty".to_string());
    }
    Ok(config)
}

impl MatchConfig {
    fn command_is_empty(&self) -> bool {
        self.engine_a.command.is_empty() || self.engine_b.command.is_empty()
    }
}

/// The kind-specific fields each client requires, and nothing else.
fn check_engine(engine: &EngineSpec, name: &str) -> Result<(), String> {
    if engine.command.is_empty() {
        return Err(format!("{name}.command must not be empty"));
    }
    if engine.turn_timeout_seconds <= 0.0 {
        return Err(format!("{name}.turn_timeout_seconds must be positive"));
    }
    match engine.kind {
        EngineKind::Pistol => {
            // Two refusals and not one: naming neither budget and naming both
            // are different mistakes, and a single message about "the budget"
            // would tell an operator which file to open and not what to do in
            // it.
            match (engine.nodes, engine.movetime_ms) {
                (None, None) => {
                    return Err(format!(
                        "{name} of kind pistol requires nodes or movetime_ms"
                    ));
                }
                (Some(_), Some(_)) => {
                    return Err(format!(
                        "{name} of kind pistol names both nodes and movetime_ms: a seat \
                         runs under one budget"
                    ));
                }
                _ => {}
            }
            if engine.movetime_ms == Some(0) {
                return Err(format!("{name}.movetime_ms must be positive"));
            }
            if engine.time_limit_seconds.is_some() {
                return Err(format!(
                    "{name} of kind pistol refuses time_limit_seconds: its budget is \
                     nodes or movetime_ms"
                ));
            }
        }
        EngineKind::Sealbot => {
            if engine.time_limit_seconds.is_none() {
                return Err(format!("{name} of kind sealbot requires time_limit_seconds"));
            }
            if engine.nodes.is_some() {
                return Err(format!(
                    "{name} of kind sealbot refuses nodes: its budget is time"
                ));
            }
            if engine.movetime_ms.is_some() {
                return Err(format!(
                    "{name} of kind sealbot refuses movetime_ms: its budget is time"
                ));
            }
        }
    }
    Ok(())
}
