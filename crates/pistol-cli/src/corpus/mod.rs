pub mod audit;
pub mod bench;
pub mod census;
pub mod distance;
pub mod documents;
pub mod emit;
pub mod json;
pub mod openings;
pub mod record;
pub mod replay;
pub mod stats;
pub mod verdict;

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use record::Record;

/// Named refusals of the corpus document itself.
pub const REPEATED_GAME_HASH_SAME_MOVES: &str = "the same game twice: two lines share a game_hash and hold identical moves, though the corpus \
     states game_hash is its dedupe key and that no duplicates were dropped";
/// See [`REPEATED_GAME_HASH_SAME_MOVES`].
pub const REPEATED_GAME_HASH_OTHER_MOVES: &str = "two lines share a game_hash and hold different moves: game_hash is 16 hex characters, so this \
     is a truncated-digest collision or a corrupted export, and either way the file is not the \
     document its metadata describes";
/// See [`REPEATED_GAME_HASH_SAME_MOVES`].
pub const CARRIAGE_RETURN: &str = "a carriage return ends this line: a CRLF corpus would hash differently on two platforms, so \
     it is refused rather than trimmed";
/// See [`REPEATED_GAME_HASH_SAME_MOVES`].
pub const BLANK_LINE: &str = "a blank line, where every line is one game";
/// See [`REPEATED_GAME_HASH_SAME_MOVES`].
pub const NO_GAMES: &str = "this corpus holds no games: an empty document is refused rather than curated from, because \
     every number a run would report would be a fact about nothing";

/// A corpus this tool refuses to read, and where it gave up.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusError {
    /// The file.
    pub path: PathBuf,
    /// The 1-based line, where the problem is on one.
    pub line: Option<usize>,
    /// The byte offset within that line, where it is at one.
    pub column: Option<usize>,
    /// What is wrong.
    pub why: String,
}

impl fmt::Display for CorpusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())?;
        if let Some(line) = self.line {
            write!(f, ":{line}")?;
            if let Some(column) = self.column {
                write!(f, ":{column}")?;
            }
        }
        write!(f, ": {}", self.why)
    }
}

impl std::error::Error for CorpusError {}

/// Read every line of a corpus, refusing the whole document on any problem.
///
/// A malformed line is fatal rather than excluded, and the reason is not that it
/// has no hash to be listed under — the line number is a perfectly good name.
/// It is that the corpus is a sha-pinned document with a uniform schema, so a
/// line that does not fit means either the wrong file or a changed format, and
/// in both cases every number the run would go on to report was derived from a
/// different document than the header claims (CLAUDE.md rule 3).
///
/// A rules-level failure is a different thing and is *not* fatal: those games
/// are excluded, listed by hash with their named error in a committed header,
/// and counted in the stats block, which is the opposite of silent.
pub fn read(path: &Path, text: &str) -> Result<Vec<Record>, CorpusError> {
    let refuse = |line: Option<usize>, column: Option<usize>, why: String| CorpusError {
        path: path.to_path_buf(),
        line,
        column,
        why,
    };
    let mut records: Vec<Record> = Vec::new();
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();

    for (index, raw) in text.split('\n').enumerate() {
        let number = index + 1;
        // A final newline leaves one empty piece, which is the file ending
        // properly rather than a blank line in it.
        if raw.is_empty() && index + 1 == text.split('\n').count() {
            break;
        }
        if raw.ends_with('\r') {
            return Err(refuse(Some(number), None, CARRIAGE_RETURN.to_string()));
        }
        if raw.trim().is_empty() {
            return Err(refuse(Some(number), None, BLANK_LINE.to_string()));
        }
        let record =
            record::parse(raw).map_err(|error| refuse(Some(number), Some(error.at), error.why))?;
        if let Some(&first) = seen.get(&record.game_hash) {
            let same = records[first].moves == record.moves;
            let why = if same {
                REPEATED_GAME_HASH_SAME_MOVES
            } else {
                REPEATED_GAME_HASH_OTHER_MOVES
            };
            return Err(refuse(
                Some(number),
                None,
                format!("{why} (first seen on line {})", first + 1),
            ));
        }
        seen.insert(record.game_hash.clone(), records.len());
        records.push(record);
    }
    if records.is_empty() {
        return Err(refuse(None, None, NO_GAMES.to_string()));
    }
    Ok(records)
}
