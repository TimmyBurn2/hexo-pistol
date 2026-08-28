use std::fmt;
use std::path::PathBuf;

/// The key reported when a failure belongs to no single field — a syntax error,
/// which happens before any field exists.
pub const DOCUMENT_KEY: &str = "<document>";

/// Every way the evaluation layer refuses to proceed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    /// The weights file could not be read at all.
    WeightsUnreadable {
        /// The path as configured.
        path: PathBuf,
        /// What the operating system said.
        why: String,
    },
    /// A weights document was absent a key, carried an unknown one, or stated a
    /// value this build refuses.
    Weights {
        /// Dotted path of the offending key, or [`DOCUMENT_KEY`].
        key: String,
        /// What is wrong with it.
        why: String,
    },
}

impl EvalError {
    /// Build an [`EvalError::Weights`] from anything string-shaped.
    pub fn weights(key: impl Into<String>, why: impl Into<String>) -> Self {
        EvalError::Weights {
            key: key.into(),
            why: why.into(),
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::WeightsUnreadable { path, why } => {
                write!(f, "eval weights: cannot read {}: {why}", path.display())
            }
            EvalError::Weights { key, why } => write!(f, "eval weights: `{key}`: {why}"),
        }
    }
}

impl std::error::Error for EvalError {}

/// Map a TOML syntax failure. No key exists yet at this stage, so the location
/// travels in the explanation instead.
pub(crate) fn from_toml_syntax<E: fmt::Display>(err: E) -> EvalError {
    EvalError::weights(DOCUMENT_KEY, summarize(&err.to_string()))
}

/// Map a deserialization failure, naming the key path that caused it.
///
/// This is deliberately a second, small copy of what pistol-engine does for its
/// own config (docs/decisions.md D-24, D-69): the dependency runs
/// engine -> eval, so eval cannot borrow the engine's mapper, and a shared
/// utility crate is not in the crate map (D-1). Thirty lines of duplication is
/// the cheaper of the two mistakes.
pub(crate) fn from_path_error<E: fmt::Display>(err: serde_path_to_error::Error<E>) -> EvalError {
    let why = summarize(&err.inner().to_string());
    let path = err.path().to_string();
    EvalError::weights(key_of(&path, &why), why)
}

/// The dotted key a deserialization failure belongs to.
///
/// `serde_path_to_error` reports how far into the document it got, which for an
/// absent or unknown member is the *containing* table rather than the member
/// itself. The member is in the message, in backticks; splicing the two gives
/// the key an operator can actually edit, unless the path already ends in it.
fn key_of(path: &str, why: &str) -> String {
    let names_member = why.starts_with("missing field") || why.starts_with("unknown field");
    let member = if names_member { backticked(why) } else { None };
    match (path, member) {
        ("" | ".", None) => DOCUMENT_KEY.to_string(),
        ("" | ".", Some(member)) => member.to_string(),
        (path, Some(member)) if path.rsplit('.').next() != Some(member) => {
            format!("{path}.{member}")
        }
        (path, _) => path.to_string(),
    }
}

/// The first backtick-delimited fragment of a message, if any.
fn backticked(text: &str) -> Option<&str> {
    let rest = text.split_once('`')?.1;
    rest.split_once('`').map(|(inside, _)| inside)
}

/// Collapse a multi-line library message into one line: the first line and the
/// last, which is all of a TOML diagnostic that survives usefully in a log.
/// Trailing ``in `a.b` `` lines are dropped — the key is reported separately.
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
