//! Writing a fixture: the header, the payload, and the digest over the payload.
//!
//! CLAUDE.md rule 7 wants a fixture sha-pinned, and rule 4 wants the same inputs
//! to produce the same bytes. Both are properties of *serialization*, so the
//! serialization is spelled out here rather than left to whatever the writer did
//! (docs/decisions.md D-147):
//!
//! - lines end with `\n`, and the file ends with exactly one;
//! - the header identifies the corpus by its SHA-256 and by nothing else. No
//!   path, no timestamp, no hostname, no version. A path is machine-specific,
//!   and a header carrying one would make "identical inputs, identical outputs"
//!   false between two machines that both did everything right;
//! - `# param` lines carry inputs, `# derived` lines carry computed values. A
//!   floor that was measured is not a knob that was chosen, and a reader
//!   reproducing the file needs to know which is which;
//! - the exclusion list is ordered by game hash and written as `# excluded none`
//!   when it is empty, because a missing section and an empty one must not look
//!   alike (CLAUDE.md rule 3);
//! - `# body_sha256` is the last header line, and **the body begins at the first
//!   byte after its newline**. The payload contains `#` comment lines of its
//!   own, so "the leading run of comments" would not have been a usable rule.
//!
//! The whole-file digest is pinned in the test that reads the file, matching the
//! convention `tactical_v0.txt` already uses. The body digest is not a weaker
//! copy of it: the whole-file digest catches strictly more edits, but it lives
//! out of band, and the body digest is in-band — a consumer holding only the
//! file can refuse a corrupted one without carrying a constant from a test
//! (docs/decisions.md D-148).

use std::fmt::Write as _;

use crate::sha256::sha256_hex;

/// The marker that ends the header and introduces the payload digest.
pub const BODY_DIGEST: &str = "# body_sha256 ";

/// How many excluded games a header lists before it summarizes the rest. A
/// fixture header is a document, not a log.
pub const EXCLUSION_LIST_LIMIT: usize = 64;

/// A fixture being built: the header lines, then the payload lines.
pub struct Fixture {
    header: String,
    body: String,
}

impl Fixture {
    /// A fixture whose header opens with these prose lines.
    pub fn new(title: &[&str]) -> Fixture {
        let mut header = String::new();
        for line in title {
            if line.is_empty() {
                header.push_str("#\n");
            } else {
                let _ = writeln!(header, "# {line}");
            }
        }
        Fixture {
            header,
            body: String::new(),
        }
    }

    /// An input the extraction was run with.
    pub fn param(&mut self, name: &str, value: impl std::fmt::Display) {
        let _ = writeln!(self.header, "# param {name} {value}");
    }

    /// A value the extraction computed. Never a `param`: a reader has to be able
    /// to tell a choice from a measurement.
    pub fn derived(&mut self, name: &str, value: impl std::fmt::Display) {
        let _ = writeln!(self.header, "# derived {name} {value}");
    }

    /// A blank comment line, for separating blocks.
    pub fn gap(&mut self) {
        self.header.push_str("#\n");
    }

    /// A prose line in the header.
    pub fn note(&mut self, line: &str) {
        if line.is_empty() {
            self.header.push_str("#\n");
        } else {
            let _ = writeln!(self.header, "# {line}");
        }
    }

    /// The excluded games, by hash, in hash order, capped.
    pub fn exclusions(&mut self, mut listed: Vec<String>) {
        listed.sort();
        if listed.is_empty() {
            self.header.push_str("# excluded none\n");
            return;
        }
        let total = listed.len();
        for line in listed.iter().take(EXCLUSION_LIST_LIMIT) {
            let _ = writeln!(self.header, "# excluded {line}");
        }
        if total > EXCLUSION_LIST_LIMIT {
            let _ = writeln!(
                self.header,
                "# excluded ... and {} more, {total} in all. A header is a document rather \
                 than a log, so the rest are not here; the run's own stdout enumerates every one",
                total - EXCLUSION_LIST_LIMIT
            );
        }
    }

    /// One payload line.
    pub fn line(&mut self, line: &str) {
        self.body.push_str(line);
        self.body.push('\n');
    }

    /// A comment line inside the payload.
    pub fn body_note(&mut self, line: &str) {
        let _ = writeln!(self.body, "# {line}");
    }

    /// The finished file: header, the payload digest, then the payload.
    pub fn render(&self) -> String {
        let mut out = String::with_capacity(self.header.len() + self.body.len() + 80);
        out.push_str(&self.header);
        let _ = writeln!(out, "{BODY_DIGEST}{}", sha256_hex(self.body.as_bytes()));
        out.push_str(&self.body);
        out
    }
}

/// The payload of a rendered fixture: everything after the body-digest line.
///
/// The split the digest promises, available to a reader as well as to the
/// writer, so a consumer can verify the claim without re-deriving where the
/// header stopped.
pub fn body_of(rendered: &str) -> Option<&str> {
    let start = rendered.find(BODY_DIGEST)?;
    let newline = rendered[start..].find('\n')? + start;
    Some(&rendered[newline + 1..])
}

/// The digest a rendered fixture's header claims for its payload.
pub fn claimed_body_digest(rendered: &str) -> Option<&str> {
    let start = rendered.find(BODY_DIGEST)? + BODY_DIGEST.len();
    let end = rendered[start..].find('\n')? + start;
    Some(rendered[start..end].trim())
}
