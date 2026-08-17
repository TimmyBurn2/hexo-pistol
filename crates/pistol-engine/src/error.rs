//! Named failures.
//!
//! Nothing in this workspace fails silently, falls back to a default, or
//! swallows an error (CLAUDE.md rule 3). Every rejection surfaces as one of the
//! variants below, and a configuration rejection always names the key that
//! caused it.

use std::fmt;

/// The key reported when a failure cannot be attributed to a single field —
/// a syntax error, for instance, which happens before any field exists.
pub const DOCUMENT_KEY: &str = "<document>";

/// Every way the engine refuses to proceed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    /// A configuration value was absent, unknown, malformed, or inconsistent
    /// with another value. `key` is the dotted path of the offending key.
    Config {
        /// Dotted path of the offending key, or [`DOCUMENT_KEY`].
        key: String,
        /// What is wrong with it.
        why: String,
    },
    /// A move was rejected: off the legal region, onto an occupied cell, wrong
    /// stone count for the turn, or played after the game was already decided.
    IllegalMove {
        /// The turn the rejected move belongs to, counting from 1.
        turn: u32,
        /// Why it was rejected.
        why: String,
    },
    /// A position could not exist under the rules — an unreachable stone count,
    /// a board already containing a completed line, a stone outside the legal
    /// region.
    IllegalPosition {
        /// Why the position is unreachable.
        why: String,
    },
    /// The position is legal, and there is still nothing here to search: the
    /// mover is half way through a turn, or the game is already decided.
    ///
    /// This is a different refusal from [`EngineError::IllegalPosition`], which
    /// says a position could not exist at all. A `phase:1` position exists, is
    /// reachable, and is a position the protocol can be handed
    /// (docs/decisions.md D-6) — but the search counts, deepens and reports in
    /// turns and so starts at a turn boundary (D-50, D-71), and the ply-level
    /// rules entry point is what finishes a half-played turn.
    PositionNotSearchable {
        /// What about the position leaves nothing to search.
        why: String,
    },
    /// A line of the engine protocol could not be understood or obeyed.
    Protocol {
        /// The offending line, as whoever refused it chose to quote it — which is
        /// usually verbatim, but is a truncated prefix for a very long line and a
        /// hex dump for one that was not text at all. A refusal is one readable
        /// line (docs/decisions.md D-5, D-88), and a megabyte of nonsense answered
        /// with a megabyte of nonsense is not.
        line: String,
        /// Why it was rejected.
        why: String,
    },
    /// A search was requested without a budget. A budget is always explicit;
    /// there is no default budget anywhere in this workspace.
    BudgetMissing,
    /// A budget kind was requested that instrument mode cannot honour without
    /// breaking reproducibility.
    InstrumentBudgetUnsupported,
    /// An invariant this crate maintains was found violated. This is a bug in
    /// pistol, never operator error.
    InternalInvariant {
        /// The invariant that failed.
        what: String,
    },
}

impl EngineError {
    /// Build a [`EngineError::Config`] from anything string-shaped.
    pub fn config(key: impl Into<String>, why: impl Into<String>) -> Self {
        EngineError::Config {
            key: key.into(),
            why: why.into(),
        }
    }

    /// Build an [`EngineError::IllegalPosition`] from anything string-shaped.
    pub fn illegal_position(why: impl Into<String>) -> Self {
        EngineError::IllegalPosition { why: why.into() }
    }

    /// Build an [`EngineError::PositionNotSearchable`] from anything
    /// string-shaped.
    pub fn not_searchable(why: impl Into<String>) -> Self {
        EngineError::PositionNotSearchable { why: why.into() }
    }

    /// Build an [`EngineError::InternalInvariant`] from anything string-shaped.
    pub fn internal(what: impl Into<String>) -> Self {
        EngineError::InternalInvariant { what: what.into() }
    }

    /// The explanation the line protocol carries after the variant's name.
    ///
    /// [`fmt::Display`] writes prose for a log ("illegal move on turn 4: …");
    /// this writes the same facts without the kind, because the protocol states
    /// the kind separately and `error IllegalMove: illegal move on turn 4: …`
    /// says it twice. The two are composed from the same fields, so neither can
    /// tell an operator something the other does not.
    ///
    /// The result is always one line: a protocol answer is one line
    /// (docs/decisions.md D-5), and the only multi-line text that reaches these
    /// variants is a parser diagnostic, which is collapsed before it gets here.
    pub fn detail(&self) -> String {
        match self {
            EngineError::Config { key, why } => format!("`{key}`: {why}"),
            EngineError::IllegalMove { turn, why } => format!("turn {turn}: {why}"),
            EngineError::IllegalPosition { why } => why.clone(),
            EngineError::PositionNotSearchable { why } => why.clone(),
            EngineError::Protocol { line, why } => format!("{why} (in: {line:?})"),
            EngineError::BudgetMissing | EngineError::InstrumentBudgetUnsupported => {
                self.to_string()
            }
            EngineError::InternalInvariant { what } => what.clone(),
        }
    }

    /// The variant's name, as the line protocol spells it.
    ///
    /// The protocol answers a rejected line with `error <NamedError>: <why>`
    /// (docs/decisions.md D-5), so this name is part of the operator contract
    /// and part of what a driver may match on: it is written here rather than
    /// derived from `Debug`, which would let a field rename change the wire
    /// format.
    pub const fn name(&self) -> &'static str {
        match self {
            EngineError::Config { .. } => "Config",
            EngineError::IllegalMove { .. } => "IllegalMove",
            EngineError::IllegalPosition { .. } => "IllegalPosition",
            EngineError::PositionNotSearchable { .. } => "PositionNotSearchable",
            EngineError::Protocol { .. } => "Protocol",
            EngineError::BudgetMissing => "BudgetMissing",
            EngineError::InstrumentBudgetUnsupported => "InstrumentBudgetUnsupported",
            EngineError::InternalInvariant { .. } => "InternalInvariant",
        }
    }
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::Config { key, why } => write!(f, "config: `{key}`: {why}"),
            EngineError::IllegalMove { turn, why } => {
                write!(f, "illegal move on turn {turn}: {why}")
            }
            EngineError::IllegalPosition { why } => write!(f, "illegal position: {why}"),
            EngineError::PositionNotSearchable { why } => {
                write!(f, "nothing to search: {why}")
            }
            EngineError::Protocol { line, why } => write!(f, "protocol: {why} (in: {line:?})"),
            EngineError::BudgetMissing => {
                f.write_str("no budget given: a search budget is always explicit, never defaulted")
            }
            EngineError::InstrumentBudgetUnsupported => f.write_str(
                "instrument mode takes only depth_turns or nodes budgets: a wall-clock \
                 budget cannot be reproduced",
            ),
            EngineError::InternalInvariant { what } => {
                write!(f, "internal invariant violated: {what}")
            }
        }
    }
}

impl std::error::Error for EngineError {}

/// Map a TOML syntax failure. No key exists yet at this stage, so the location
/// travels in the explanation instead.
pub(crate) fn from_toml_syntax<E: fmt::Display>(err: E) -> EngineError {
    EngineError::config(DOCUMENT_KEY, summarize(&err.to_string()))
}

/// Map a deserialization failure, naming the key path that caused it.
pub(crate) fn from_path_error<E: fmt::Display>(err: serde_path_to_error::Error<E>) -> EngineError {
    let path = err.path().to_string();
    let why = summarize(&err.inner().to_string());
    EngineError::config(key_of(&path, &why), why)
}

/// The dotted key a deserialization failure belongs to.
///
/// `serde_path_to_error` reports how far into the document it got, which for an
/// absent member — and for anything inside a tagged table, whose contents are
/// buffered — is the containing table rather than the member itself. The member
/// is in the message, in backticks; splicing the two gives the key an operator
/// can actually go and edit, unless the path already ends in it.
fn key_of(path: &str, why: &str) -> String {
    let container = match path {
        "" | "." => None,
        other => Some(other),
    };
    let member = if why.starts_with("missing field") || why.starts_with("unknown field") {
        backticked(why)
    } else {
        None
    };
    match (container, member) {
        (Some(container), Some(member)) if !ends_with_segment(container, member) => {
            format!("{container}.{member}")
        }
        (Some(container), _) => container.to_string(),
        (None, Some(member)) => member.to_string(),
        (None, None) => DOCUMENT_KEY.to_string(),
    }
}

/// Whether a dotted path's last segment is exactly `segment`.
fn ends_with_segment(path: &str, segment: &str) -> bool {
    path.rsplit('.').next() == Some(segment)
}

/// The first backtick-delimited fragment of a message, if any.
fn backticked(text: &str) -> Option<&str> {
    let rest = text.split_once('`')?.1;
    rest.split_once('`').map(|(inside, _)| inside)
}

/// Collapse a multi-line library message into one line: the location line and
/// the explanation line, which is all of a TOML diagnostic that survives
/// usefully in a log. Trailing ``in `a.b` `` lines are dropped — the key is
/// reported separately and saying it twice helps nobody.
fn summarize(message: &str) -> String {
    let mut lines = message
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("in `"));
    let Some(first) = lines.next() else {
        return String::from("(no detail)");
    };
    match lines.next_back() {
        Some(last) if last != first => format!("{first}; {last}"),
        _ => first.to_string(),
    }
}
