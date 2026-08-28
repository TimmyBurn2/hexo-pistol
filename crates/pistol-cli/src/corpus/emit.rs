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
