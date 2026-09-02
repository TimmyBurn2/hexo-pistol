use pistol_cli::corpus::emit::{self, Fixture};

use crate::error::ArenaError;
use crate::transcript::Transcript;

/// The census record grammar this build writes.
///
/// A consumer refuses any other version: the fields are not promised to mean
/// the same thing across two of them.
pub const CENSUS_FORMAT_VERSION: u32 = 1;

/// The identity of a census run.
///
/// The capture's own identity, folded with THIS grammar's version — because a
/// version written only as a `# param` is a promise nothing binds, and a census
/// v2 would otherwise carry the same digest as the v1 it cannot be read as.
/// `crates/pistol-arena/tests/capture_tests.rs` pins that property for the
/// capture; the census needs its own or it has none.
/// `format_version` is a PARAMETER and not the constant read from inside, for
/// the reason [`crate::capture::capture_sha256`] takes one: a test can only
/// pin that the identity MOVES with the grammar if it can hand this function
/// two grammars. A version the function reads for itself is a version no test
/// can vary, and the check degenerates into "the census digest is not the
/// capture digest" — which a build that dropped the version entirely still
/// passes.
pub fn census_sha256(experiment_sha256: &str, go_line: &str, format_version: u32) -> String {
    let capture = crate::capture::capture_sha256(
        experiment_sha256,
        go_line,
        crate::capture_file::CAPTURE_FORMAT_VERSION,
    );
    pistol_cli::sha256::sha256_hex(
        format!("census_format {format_version}\ncapture_sha256 {capture}\n").as_bytes(),
    )
}

/// Write the whole census file — header, payload digest, then the rows.
///
/// **STREAMED, and the reason is a size.** Every sibling writer in this crate
/// renders its file into a `String` and hands it over; the census artifact is
/// estimated at some gigabytes (`docs/experiments/wp20b_design.md` §2), and a
/// rendered `String` beside a `Fixture`'s body beside the caller's own rows is
/// three resident copies of it. This walks the rows TWICE instead — once for
/// the digest, once for the bytes — and holds one line at a time.
///
/// The two passes cannot disagree about what they cover: the digest is taken
/// over exactly the bytes the second pass writes, in the same order, by the
/// same expression.
///
/// # Errors
/// Any write failure, named with the path — a partly written census is
/// abandoned by the caller, never left to be read as a whole one.
pub fn write_into(
    out: &mut impl std::io::Write,
    path: &std::path::Path,
    transcript: &Transcript,
    label_go: &str,
    rows: &[String],
) -> Result<(), ArenaError> {
    let io = |why: std::io::Error| ArenaError::io(format!("writing {}", path.display()), why);
    out.write_all(header(transcript, label_go, rows.len()).as_bytes())
        .map_err(io)?;
    writeln!(out, "{}{}", emit::BODY_DIGEST, body_digest(rows)).map_err(io)?;
    for row in rows {
        writeln!(out, "{row}").map_err(io)?;
    }
    out.flush().map_err(io)
}

/// The header a census file opens with, up to but not including the payload
/// digest line.
fn header(transcript: &Transcript, label_go: &str, rows: usize) -> String {
    let identity = census_sha256(
        &transcript.experiment_sha256,
        label_go,
        CENSUS_FORMAT_VERSION,
    );
    let mut fixture = Fixture::new(&[
        "pistol — one record per trigger firing, as the engine wrote it on the wire.",
        "",
        "The `go` line carries the census token, so `capture_sha256` here is NOT the digest",
        "of the otherwise identical census-off capture: it is a different instrument",
        "(docs/experiments/wp20b_design.md §3).",
        "",
        "One row per line, in the field order the protocol pins.",
    ]);
    fixture.param("census_format_version", CENSUS_FORMAT_VERSION);
    fixture.param("experiment_sha256", &transcript.experiment_sha256);
    fixture.param("source_sha256", &transcript.source_sha256);
    fixture.param("label_go", label_go);
    fixture.derived("census_sha256", &identity);
    fixture.derived(
        "capture_sha256",
        crate::capture::capture_sha256(
            &transcript.experiment_sha256,
            label_go,
            crate::capture_file::CAPTURE_FORMAT_VERSION,
        ),
    );
    fixture.derived("games", transcript.games.len());
    fixture.derived("rows", rows);
    // The header ALONE: `Fixture::render` would append a digest over an empty
    // body, and the body is what this file streams.
    fixture.render_header()
}

/// The digest of the payload a census file holds.
///
/// ONE expression, called by the writer and by the manifest row alike, so the
/// row cannot come to name a digest of something other than what was written.
fn body_digest(rows: &[String]) -> String {
    let mut digest = pistol_cli::sha256::Sha256::new();
    for row in rows {
        digest.update(row.as_bytes());
        digest.update(b"\n");
    }
    digest.finish_hex()
}

/// The manifest row a run prints for a human to commit.
///
/// PRINTED and never written, for [`crate::capture_file::manifest_row`]'s
/// reason. The body digest is what rule 8 and docs/decisions.md D-469 lean on
/// when the artifact is large, uncommitted, and sha-indexed by a committed
/// manifest.
///
/// INFALLIBLE, and its sibling is not: that one reads the digest back out of a
/// rendered string and can fail to find one, while this one computes it from
/// the rows by the same expression the writer used. A refusal that cannot fire
/// is not a guard.
pub fn manifest_row(
    transcript: &Transcript,
    label_go: &str,
    rows: &[String],
    out_path: &std::path::Path,
) -> String {
    let identity = census_sha256(
        &transcript.experiment_sha256,
        label_go,
        CENSUS_FORMAT_VERSION,
    );
    let body = body_digest(rows);
    format!(
        "census_manifest census_sha256 {identity} body_sha256 {body} experiment_sha256 {} \
         source_sha256 {} label_go {label_go} path {}",
        transcript.experiment_sha256,
        transcript.source_sha256,
        out_path.display()
    )
}
