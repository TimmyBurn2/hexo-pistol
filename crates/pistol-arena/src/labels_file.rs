use std::fmt::Write as _;

use pistol_cli::corpus::emit::{self, Fixture};

use crate::error::ArenaError;
use crate::transcript::Transcript;

/// The corpus record grammar this build writes and reads.
pub const CORPUS_SCHEMA_VERSION: u32 = 1;

/// How many TAB-separated fields one record carries.
const FIELDS: usize = 16;

/// One labelled position: what it is, what the teacher said, and how the game
/// reached it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusRecord {
    /// The source report's game index.
    pub game: usize,
    /// How many turns the prefix holds. Not a turn NUMBER, which is one more.
    pub turns_played: usize,
    /// The move list (docs/decisions.md D-6), turn tokens alone.
    pub moves: String,
    /// `canonical_sequence` over the prefix: the same game up to a symmetry.
    pub key_seq: String,
    /// `GameState::key`: the same position up to transposition.
    pub key_pos: String,
    /// `canonical_form` over the stones: the same position up to BOTH.
    pub key_full: String,
    /// Which side places the next stone.
    pub to_move: String,
    /// `eval`, `mate_in` or `mated_in` — never the wire's `cp`.
    pub score_kind: String,
    /// The integer that goes with it.
    pub score_value: i64,
    /// The move the engine would play.
    pub best: String,
    /// The depth the label was produced at.
    pub depth_turns: u32,
    /// Search nodes the label cost.
    pub search_nodes: u64,
    /// Solver nodes the label cost.
    pub solver_nodes: u64,
    /// Whether every turn leading here was a book turn.
    pub book: bool,
    /// Which seat the report awarded the game to.
    pub result: String,
    /// `normal` or `forfeit`.
    pub end: String,
}

/// A corpus read back.
#[derive(Debug, Clone)]
pub struct Corpus {
    /// The capture these records were derived from.
    pub capture_sha256: String,
    /// The records, in the capture's own order.
    pub records: Vec<CorpusRecord>,
}

fn refuse(what: impl Into<String>) -> ArenaError {
    ArenaError::config("corpus file", what.into())
}

/// The body lines a set of records renders to, without the header.
pub fn render_records(records: &[CorpusRecord]) -> String {
    let mut out = String::new();
    for r in records {
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            r.game,
            r.turns_played,
            r.moves,
            r.key_seq,
            r.key_pos,
            r.key_full,
            r.to_move,
            r.score_kind,
            r.score_value,
            r.best,
            r.depth_turns,
            r.search_nodes,
            r.solver_nodes,
            if r.book { "yes" } else { "no" },
            r.result,
            r.end
        );
    }
    out
}

/// The whole corpus file, header and body.
pub fn render(
    capture: &crate::capture_file::Capture,
    transcript: &Transcript,
    records: &[CorpusRecord],
) -> String {
    let mut fixture = Fixture::new(&[
        "pistol — one labelled position per record.",
        "",
        "What each column MEANS is fixed by the keyed params below and by nothing else:",
        "a corpus whose units live only in a design document loses them on the first copy.",
    ]);
    fixture.param("corpus_schema_version", CORPUS_SCHEMA_VERSION);
    fixture.param("experiment_sha256", &transcript.experiment_sha256);
    fixture.param("source_sha256", &transcript.source_sha256);
    fixture.param("label_go", &capture.label_go);
    fixture.param("opening_turns", transcript.opening_turns);
    fixture.param(
        "score_units",
        "eval is pistol-eval's own integer units; there is no pawn on this board",
    );
    fixture.param(
        "score_sign",
        "from the point of view of the side to move at the root",
    );
    fixture.param(
        "mate_counts",
        "mate_in and mated_in count every turn from the root, both sides'",
    );
    fixture.param(
        "depth_meaning",
        "a completed search depth, except where search_nodes is zero, where it is a proof depth",
    );
    fixture.derived("capture_sha256", &capture.capture_sha256);
    fixture.derived("games", transcript.games.len());
    fixture.derived("records", records.len());
    for line in render_records(records).lines() {
        fixture.line(line);
    }
    fixture.render()
}

/// The manifest row a run prints for a human to commit.
///
/// # Errors
/// If the rendered corpus carries no body digest.
pub fn manifest_row(
    capture: &crate::capture_file::Capture,
    transcript: &Transcript,
    rendered: &str,
    out_path: &std::path::Path,
) -> Result<String, ArenaError> {
    let body = emit::claimed_body_digest(rendered).ok_or_else(|| {
        refuse(
            "the rendered corpus carries no body digest, so a manifest row would name bytes \
             nothing binds",
        )
    })?;
    Ok(format!(
        "corpus_manifest body_sha256 {body} corpus_schema_version {CORPUS_SCHEMA_VERSION} \
         capture_sha256 {} experiment_sha256 {} source_sha256 {} path {}",
        capture.capture_sha256,
        transcript.experiment_sha256,
        transcript.source_sha256,
        out_path.display()
    ))
}

/// One header value, or a refusal naming the key.
fn header<'a>(text: &'a str, kind: &str, key: &str) -> Result<&'a str, ArenaError> {
    let prefix = format!("# {kind} {key} ");
    text.split('\n')
        .find_map(|line| line.strip_prefix(&prefix))
        .map(str::trim)
        .ok_or_else(|| refuse(format!("it carries no `{kind} {key}` line")))
}

/// Read a corpus back.
///
/// # Errors
/// Naming the reason. The WHOLE file is refused rather than partially read.
pub fn read(text: &str) -> Result<Corpus, ArenaError> {
    let version: u32 = header(text, "param", "corpus_schema_version")?
        .parse()
        .map_err(|_| refuse("its `corpus_schema_version` is not a version number"))?;
    if version != CORPUS_SCHEMA_VERSION {
        return Err(refuse(format!(
            "it is corpus schema {version} and this build writes and reads \
             {CORPUS_SCHEMA_VERSION}"
        )));
    }
    for key in [
        "experiment_sha256",
        "source_sha256",
        "label_go",
        "opening_turns",
        "score_units",
        "score_sign",
        "mate_counts",
        "depth_meaning",
    ] {
        header(text, "param", key)?;
    }
    let claimed = emit::claimed_body_digest(text)
        .ok_or_else(|| refuse("it carries no body digest, so nothing binds its records"))?;
    let body = emit::body_of(text)
        .ok_or_else(|| refuse("it carries no body digest, so nothing binds its records"))?;
    let actual = pistol_cli::sha256::sha256_hex(body.as_bytes());
    if actual != claimed {
        return Err(refuse(format!(
            "its body digests to {actual} and its header claims {claimed}"
        )));
    }

    let mut records = Vec::new();
    for (at, line) in body.split('\n').filter(|line| !line.is_empty()).enumerate() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() != FIELDS {
            return Err(refuse(format!(
                "record {} carries {} TAB-separated field(s) and this format writes {FIELDS}",
                at + 1,
                f.len()
            )));
        }
        if let Some(empty) = f.iter().position(|field| field.is_empty()) {
            return Err(refuse(format!(
                "record {}'s field {} is empty, and no field of this record can be",
                at + 1,
                empty + 1
            )));
        }
        let number = |raw: &str, what: &str| -> Result<i64, ArenaError> {
            raw.parse()
                .map_err(|_| refuse(format!("record {}: `{raw}` is not a {what}", at + 1)))
        };
        if !matches!(f[7], "eval" | "mate_in" | "mated_in") {
            return Err(refuse(format!(
                "record {}: `{}` is not one of the three score kinds",
                at + 1,
                f[7]
            )));
        }
        let score_value = number(f[8], "score value")?;
        if f[7] != "eval" && score_value < 0 {
            return Err(refuse(format!(
                "record {}: a `{}` value is a turn count and cannot be negative",
                at + 1,
                f[7]
            )));
        }
        if f[4].len() != 32 || !f[4].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(refuse(format!(
                "record {}: `key_pos` is not thirty-two hex digits",
                at + 1
            )));
        }
        shape(f[3], at, "key_seq", |token| {
            token.parse::<pistol_core::Turn>().is_ok()
        })?;
        shape(f[5], at, "key_full", |token| {
            token.split_once(':').is_some_and(|(cell, side)| {
                cell.parse::<pistol_core::Coord>().is_ok() && matches!(side, "p1" | "p2")
            })
        })?;
        for (field, what, set) in [
            (f[6], "to_move", ["p1", "p2"].as_slice()),
            (f[13], "book", ["yes", "no"].as_slice()),
            (f[14], "result", ["p1_win", "p2_win", "capped"].as_slice()),
            (f[15], "end", ["normal", "forfeit"].as_slice()),
        ] {
            if !set.contains(&field) {
                return Err(refuse(format!(
                    "record {}: `{field}` is not a `{what}` this format writes",
                    at + 1
                )));
            }
        }
        records.push(CorpusRecord {
            game: usize::try_from(number(f[0], "game index")?)
                .map_err(|_| refuse("a game index is not negative"))?,
            turns_played: usize::try_from(number(f[1], "turn count")?)
                .map_err(|_| refuse("a turn count is not negative"))?,
            moves: f[2].to_string(),
            key_seq: f[3].to_string(),
            key_pos: f[4].to_string(),
            key_full: f[5].to_string(),
            to_move: f[6].to_string(),
            score_kind: f[7].to_string(),
            score_value,
            best: f[9].to_string(),
            depth_turns: u32::try_from(number(f[10], "depth")?)
                .map_err(|_| refuse("a depth is not negative"))?,
            search_nodes: u64::try_from(number(f[11], "node count")?)
                .map_err(|_| refuse("a node count is not negative"))?,
            solver_nodes: u64::try_from(number(f[12], "node count")?)
                .map_err(|_| refuse("a node count is not negative"))?,
            book: f[13] == "yes",
            result: f[14].to_string(),
            end: f[15].to_string(),
        });
    }
    Ok(Corpus {
        capture_sha256: header(text, "derived", "capture_sha256")?.to_string(),
        records,
    })
}

/// A key column whose elements must all be of one shape.
fn shape(field: &str, at: usize, what: &str, ok: impl Fn(&str) -> bool) -> Result<(), ArenaError> {
    if field == crate::labels::EMPTY_FIELD {
        return Ok(());
    }
    if field.split(' ').all(ok) {
        return Ok(());
    }
    Err(refuse(format!(
        "record {}: `{what}` is not the shape this format writes",
        at + 1
    )))
}
