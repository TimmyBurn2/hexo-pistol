//! Every way a run refuses, by name.
//!
//! One closed enum, `Display` and `Error` written by hand rather than derived
//! from a dependency, for the reason docs/decisions.md D-23 gives for
//! `EngineError`: the text is part of the operator contract, so it is written.
//!
//! The division that matters here is not between kinds of failure but between
//! **a game that was adjudicated** and **a run that cannot be trusted**. A
//! forfeit is not in this enum at all — it is a game outcome, carried in the
//! report (docs/decisions.md D-158). What is in this enum is everything that
//! stops the run: a document that will not load, an engine that is not the kind
//! of engine a strength claim may come from, and the two ways a child dies that
//! the arena must not turn into a result.

use std::fmt;
use std::path::Path;

/// A run that did not happen, or stopped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArenaError {
    /// A configuration key is missing, malformed, or out of range. `key` is the
    /// exact path an operator can go and edit.
    Config {
        /// The dotted key path.
        key: String,
        /// Why it was refused.
        why: String,
    },
    /// A `movetime` budget was asked for. Its own variant because it is the one
    /// refusal this crate exists to make loudly.
    MovetimeBudgetRefused {
        /// The value the document asked for, quoted back.
        asked: String,
    },
    /// The openings document is not one.
    Openings {
        /// Which file.
        path: String,
        /// Which line, when the fault is on one; `0` for a whole-file fault.
        line: usize,
        /// Why it was refused.
        why: String,
    },
    /// The openings body does not hash to what its own header says.
    OpeningsDigest {
        /// Which file.
        path: String,
        /// What the header claims.
        claimed: String,
        /// What the bytes are.
        found: String,
    },
    /// An engine could not be started.
    Spawn {
        /// Which side.
        engine: String,
        /// The binary that would not start.
        binary: String,
        /// The operating system's account of it.
        why: String,
    },
    /// An engine spoke, but not this protocol.
    Handshake {
        /// Which side.
        engine: String,
        /// Why the conversation could not continue.
        why: String,
    },
    /// An engine answered nothing for longer than the run allows.
    ///
    /// Never a game outcome: adjudicating silence would make the verdict a
    /// function of how fast the machine is (docs/decisions.md D-159).
    Hung {
        /// Which side.
        engine: String,
        /// The opening it was playing.
        opening: usize,
        /// The turn it went quiet on.
        turn: u32,
        /// The watchdog it outlasted.
        timeout_ms: u64,
    },
    /// An engine was killed by a signal, or the arena could not tell why it
    /// died. Distinct from an engine that chose to exit, which forfeits
    /// (docs/decisions.md D-159).
    Killed {
        /// Which side.
        engine: String,
        /// What is known about the death.
        why: String,
    },
    /// A file could not be read or written.
    Io {
        /// What was being attempted.
        what: String,
        /// The operating system's account of it.
        why: String,
    },
}

impl ArenaError {
    /// A configuration refusal naming its key.
    pub fn config(key: impl Into<String>, why: impl Into<String>) -> ArenaError {
        ArenaError::Config {
            key: key.into(),
            why: why.into(),
        }
    }

    /// An openings refusal naming its line. Line `0` means the file as a whole.
    pub fn openings(path: &Path, line: usize, why: impl Into<String>) -> ArenaError {
        ArenaError::Openings {
            path: path.display().to_string(),
            line,
            why: why.into(),
        }
    }

    /// An I/O refusal naming what was being attempted.
    pub fn io(what: impl Into<String>, why: impl fmt::Display) -> ArenaError {
        ArenaError::Io {
            what: what.into(),
            why: why.to_string(),
        }
    }

    /// The variant's name, as the operator sees it and as a test matches on it.
    pub const fn name(&self) -> &'static str {
        match self {
            ArenaError::Config { .. } => "Config",
            ArenaError::MovetimeBudgetRefused { .. } => "MovetimeBudgetRefused",
            ArenaError::Openings { .. } => "Openings",
            ArenaError::OpeningsDigest { .. } => "OpeningsDigest",
            ArenaError::Spawn { .. } => "Spawn",
            ArenaError::Handshake { .. } => "Handshake",
            ArenaError::Hung { .. } => "Hung",
            ArenaError::Killed { .. } => "Killed",
            ArenaError::Io { .. } => "Io",
        }
    }
}

impl fmt::Display for ArenaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArenaError::Config { key, why } => write!(f, "Config: `{key}`: {why}"),
            ArenaError::MovetimeBudgetRefused { asked } => write!(
                f,
                "MovetimeBudgetRefused: `{asked}` is a wall-clock budget, and every strength \
                 claim in this project comes from an instrument budget — depth_turns or nodes \
                 (CLAUDE.md rule 6). A movetime budget is also not a ceiling: the first \
                 deepening iteration cannot be interrupted, so a short budget overshoots by \
                 that iteration's cost (docs/decisions.md D-74, D-95). Wall-clock arrives with \
                 WP-1.4 and Stage 4, by ADR then."
            ),
            ArenaError::Openings { path, line, why } if *line == 0 => {
                write!(f, "Openings: {path}: {why}")
            }
            ArenaError::Openings { path, line, why } => {
                write!(f, "Openings: {path}:{line}: {why}")
            }
            ArenaError::OpeningsDigest {
                path,
                claimed,
                found,
            } => write!(
                f,
                "OpeningsDigest: {path}: the header claims body_sha256 {claimed} and the body \
                 hashes to {found}; the file has been edited since it was pinned"
            ),
            ArenaError::Spawn {
                engine,
                binary,
                why,
            } => write!(f, "Spawn: engine {engine}: cannot start `{binary}`: {why}"),
            ArenaError::Handshake { engine, why } => {
                write!(f, "Handshake: engine {engine}: {why}")
            }
            ArenaError::Hung {
                engine,
                opening,
                turn,
                timeout_ms,
            } => write!(
                f,
                "Hung: engine {engine} answered nothing for {timeout_ms} ms on opening {opening}, \
                 turn {turn}. Silence is not an answer and is never adjudicated: a timeout that \
                 forfeited a game would make the verdict a function of how fast this machine is \
                 (CLAUDE.md rule 4, docs/decisions.md D-159)."
            ),
            ArenaError::Killed { engine, why } => write!(
                f,
                "Killed: engine {engine}: {why}. A child the environment killed is not a move \
                 the engine made, so it aborts the run rather than forfeiting a game \
                 (docs/decisions.md D-159)."
            ),
            ArenaError::Io { what, why } => write!(f, "Io: {what}: {why}"),
        }
    }
}

impl std::error::Error for ArenaError {}
