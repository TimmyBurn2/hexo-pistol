use pistol_cli::corpus::emit::{self, Fixture};

use crate::error::ArenaError;
use crate::transcript::Transcript;

/// The capture record grammar this build writes and reads.
///
/// A reader refuses any other version: the fields are not promised to mean the
/// same thing across two of them.
pub const CAPTURE_FORMAT_VERSION: u32 = 1;

/// How many TAB-separated fields one record carries.
const FIELDS: usize = 5;

/// One asked position and the engine's answer to it, as the engine wrote it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureRecord {
    /// The source report's game index.
    pub game: usize,
    /// How many turns the asked prefix holds. Not a turn NUMBER, which is one
    /// more.
    pub turns_played: usize,
    /// The `position` line as sent.
    pub position: String,
    /// The `info totals` line, less the two wall-clock fields.
    pub totals: String,
    /// The `bestmove` line, untouched.
    pub bestmove: String,
}

/// A capture read back: what identifies it, and its records.
#[derive(Debug, Clone)]
pub struct Capture {
    /// The identity of the run that produced it.
    pub capture_sha256: String,
    /// The experiment the source report is of.
    pub experiment_sha256: String,
    /// The digest of the whole source report, so a consumer can find it.
    pub source_sha256: String,
    /// The `go` line every label was asked at.
    pub label_go: String,
    /// The records, in the order they were asked.
    pub records: Vec<CaptureRecord>,
}

fn refuse(what: impl Into<String>) -> ArenaError {
    ArenaError::config("capture file", what.into())
}

/// The whole capture file, header and body.
pub fn render(transcript: &Transcript, label_go: &str, records: &[CaptureRecord]) -> String {
    let identity = crate::capture::capture_sha256(
        &transcript.experiment_sha256,
        label_go,
        CAPTURE_FORMAT_VERSION,
    );
    let mut fixture = Fixture::new(&[
        "pistol — one record per asked position, as the engine wrote it.",
        "",
        "The two wall-clock fields are removed from the totals line by gate 9's own rule",
        "(tools/determinism.sh); nothing else is touched. What the fields MEAN is not",
        "decided here (docs/experiments/wp20m_design.md).",
        "",
        "TAB-separated, five fields: game, turns_played, position, totals, bestmove.",
    ]);
    fixture.param("capture_format_version", CAPTURE_FORMAT_VERSION);
    fixture.param("experiment_sha256", &transcript.experiment_sha256);
    fixture.param("source_sha256", &transcript.source_sha256);
    fixture.param("label_go", label_go);
    fixture.derived("capture_sha256", &identity);
    fixture.derived("games", transcript.games.len());
    fixture.derived("records", records.len());
    for record in records {
        fixture.line(&format!(
            "{}\t{}\t{}\t{}\t{}",
            record.game, record.turns_played, record.position, record.totals, record.bestmove
        ));
    }
    fixture.render()
}

/// The manifest row a run prints for a human to commit.
///
/// PRINTED and never written: `pistol-arena` writes nothing inside the
/// repository (CLAUDE.md rule 8), and a row a human retypes drifts from its run.
pub fn manifest_row(
    transcript: &Transcript,
    label_go: &str,
    rendered: &str,
    out_path: &std::path::Path,
) -> String {
    let identity = crate::capture::capture_sha256(
        &transcript.experiment_sha256,
        label_go,
        CAPTURE_FORMAT_VERSION,
    );
    let body = emit::claimed_body_digest(rendered).unwrap_or("<none>");
    format!(
        "capture_manifest capture_sha256 {identity} body_sha256 {body} experiment_sha256 {} \
         source_sha256 {} label_go {label_go} path {}",
        transcript.experiment_sha256,
        transcript.source_sha256,
        out_path.display()
    )
}

/// One header value, or a refusal naming the key.
///
/// `kind` is `param` for an input and `derived` for a computed value: the
/// fixture format keeps the two apart so a reader can tell a choice from a
/// measurement, and so must a reader of it.
fn header<'a>(text: &'a str, kind: &str, key: &str) -> Result<&'a str, ArenaError> {
    let prefix = format!("# {kind} {key} ");
    let mut found = text
        .split('\n')
        .filter_map(|line| line.strip_prefix(&prefix));
    let first = found
        .next()
        .ok_or_else(|| refuse(format!("it carries no `{kind} {key}` line")))?;
    if found.next().is_some() {
        return Err(refuse(format!(
            "it carries more than one `{kind} {key}` line, so there is no one answer to read"
        )));
    }
    Ok(first.trim())
}

/// Read a capture file back.
///
/// The WHOLE file is refused rather than partially read: a capture with a hole
/// is worse than none.
///
/// # Errors
/// Naming the reason: a format version this build does not write, a body whose
/// digest is not the one the header claims, a record whose field count is wrong,
/// or a field that is empty.
pub fn read(text: &str) -> Result<Capture, ArenaError> {
    let version: u32 = header(text, "param", "capture_format_version")?
        .parse()
        .map_err(|_| refuse("its `capture_format_version` is not a version number"))?;
    if version != CAPTURE_FORMAT_VERSION {
        return Err(refuse(format!(
            "it is capture format {version} and this build writes and reads \
             {CAPTURE_FORMAT_VERSION}; the fields are not promised to mean the same thing across \
             the two"
        )));
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

    let mut records: Vec<CaptureRecord> = Vec::new();
    for (at, line) in body.split('\n').enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != FIELDS {
            return Err(refuse(format!(
                "record {} carries {} TAB-separated field(s) and this format writes {FIELDS}",
                at + 1,
                fields.len()
            )));
        }
        if let Some(empty) = fields.iter().position(|field| field.is_empty()) {
            return Err(refuse(format!(
                "record {}'s field {} is empty, and no field of this record can be",
                at + 1,
                empty + 1
            )));
        }
        records.push(CaptureRecord {
            game: fields[0].parse().map_err(|_| {
                refuse(format!(
                    "record {}: `{}` is not a game index",
                    at + 1,
                    fields[0]
                ))
            })?,
            turns_played: fields[1].parse().map_err(|_| {
                refuse(format!(
                    "record {}: `{}` is not a turn count",
                    at + 1,
                    fields[1]
                ))
            })?,
            position: fields[2].to_string(),
            totals: fields[3].to_string(),
            bestmove: fields[4].to_string(),
        });
    }
    Ok(Capture {
        capture_sha256: header(text, "derived", "capture_sha256")?.to_string(),
        experiment_sha256: header(text, "param", "experiment_sha256")?.to_string(),
        source_sha256: header(text, "param", "source_sha256")?.to_string(),
        label_go: header(text, "param", "label_go")?.to_string(),
        records,
    })
}
