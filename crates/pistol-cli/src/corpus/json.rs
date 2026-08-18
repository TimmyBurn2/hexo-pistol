//! A scanner for the corpus line's grammar — which is a *subset* of JSON, on
//! purpose.
//!
//! This reads exactly what `SCHEMA.md` describes and names everything else. It
//! is deliberately not a JSON parser, and the difference is the whole argument
//! (docs/decisions.md D-139): the corpus is external data, so the failure that
//! matters is a malformed line read as a plausible game rather than refused
//! (CLAUDE.md rule 3). A general parser's risk sits in the features this corpus
//! never uses — `\uXXXX` and surrogate pairs, the number grammar's exponents and
//! fractions, nested containers — which is surface that no test here would
//! exercise and no corpus would reveal. A reader that accepts only the schema's
//! shape cannot silently mis-read, because every deviation is a refusal.
//!
//! So: no escapes, no floats, no exponents, no nesting beyond the one array of
//! two-element arrays the schema has. Each of those is a named refusal, and the
//! refusal *is* the rule-3 property.
//!
//! Whitespace between tokens is spaces and tabs. A newline cannot appear — the
//! caller has already split on it — and a carriage return is refused by the
//! caller rather than skipped, so a CRLF corpus fails loudly instead of hashing
//! differently on two platforms (CLAUDE.md rule 4).

/// A refusal, as a named constant so a test can pin it without restating the
/// wording — the convention `pistol_core::error` uses.
pub const EXPECTED_OBJECT: &str = "a corpus line is one JSON object";
/// See [`EXPECTED_OBJECT`].
pub const EXPECTED_STRING: &str = "expected a quoted string";
/// See [`EXPECTED_OBJECT`].
pub const UNTERMINATED_STRING: &str = "a quoted string has no closing quote";
/// See [`EXPECTED_OBJECT`].
pub const ESCAPE_UNSUPPORTED: &str = "this reader accepts no string escapes, and the schema has none: a backslash here means the \
     line is not the document this tool reads";
/// See [`EXPECTED_OBJECT`].
pub const CONTROL_IN_STRING: &str = "a control character inside a quoted string";
/// See [`EXPECTED_OBJECT`].
pub const EXPECTED_INTEGER: &str = "expected an integer";
/// See [`EXPECTED_OBJECT`].
pub const LEADING_ZERO: &str = "an integer with a leading zero";
/// See [`EXPECTED_OBJECT`].
pub const NOT_AN_INTEGER: &str =
    "a fraction or an exponent, where the schema has an integer: this reader takes no floats";
/// See [`EXPECTED_OBJECT`].
pub const TRAILING_INPUT: &str = "text after the end of the object";

/// Where in a line something was refused, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanError {
    /// The byte offset the refusal happened at, counting from zero.
    pub at: usize,
    /// The named reason.
    pub why: String,
}

impl ScanError {
    fn new(at: usize, why: impl Into<String>) -> ScanError {
        ScanError {
            at,
            why: why.into(),
        }
    }
}

/// A cursor over one corpus line.
pub struct Scanner<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Scanner<'a> {
    /// A scanner over one line, without its terminator.
    pub fn new(line: &'a str) -> Scanner<'a> {
        Scanner {
            bytes: line.as_bytes(),
            at: 0,
        }
    }

    /// The current byte offset, for an error that wants to point at it.
    pub fn at(&self) -> usize {
        self.at
    }

    /// Refuse here, by name.
    pub fn refuse<T>(&self, why: impl Into<String>) -> Result<T, ScanError> {
        Err(ScanError::new(self.at, why))
    }

    /// Skip spaces and tabs.
    fn skip_space(&mut self) {
        while matches!(self.bytes.get(self.at), Some(b' ' | b'\t')) {
            self.at += 1;
        }
    }

    /// The next byte, without consuming it.
    fn peek(&mut self) -> Option<u8> {
        self.skip_space();
        self.bytes.get(self.at).copied()
    }

    /// Consume this exact byte, or refuse naming what was expected.
    pub fn expect(&mut self, wanted: u8) -> Result<(), ScanError> {
        match self.peek() {
            Some(byte) if byte == wanted => {
                self.at += 1;
                Ok(())
            }
            Some(byte) => self.refuse(format!(
                "expected `{}`, got `{}`",
                wanted as char, byte as char
            )),
            None => self.refuse(format!("expected `{}`, got end of line", wanted as char)),
        }
    }

    /// Consume this byte if it is next; say whether it was.
    pub fn accept(&mut self, wanted: u8) -> bool {
        if self.peek() == Some(wanted) {
            self.at += 1;
            return true;
        }
        false
    }

    /// A quoted string, with no escape ever accepted.
    pub fn string(&mut self) -> Result<&'a str, ScanError> {
        self.skip_space();
        let opened = self.at;
        if self.bytes.get(self.at) != Some(&b'"') {
            return self.refuse(EXPECTED_STRING);
        }
        self.at += 1;
        let from = self.at;
        loop {
            match self.bytes.get(self.at) {
                None => return Err(ScanError::new(opened, UNTERMINATED_STRING)),
                Some(b'"') => break,
                Some(b'\\') => return self.refuse(ESCAPE_UNSUPPORTED),
                Some(&byte) if byte < 0x20 => return self.refuse(CONTROL_IN_STRING),
                Some(_) => self.at += 1,
            }
        }
        let text = std::str::from_utf8(&self.bytes[from..self.at])
            .unwrap_or_else(|error| unreachable!("a &str slice is utf-8: {error}"));
        self.at += 1;
        Ok(text)
    }

    /// An integer, in `i64` so every field can range-check for itself.
    ///
    /// Refuses a leading `+`, a leading zero, a fraction and an exponent. Those
    /// are all valid JSON in some position and none of them is in this schema,
    /// so accepting any of them would be accepting a different document.
    pub fn integer(&mut self) -> Result<i64, ScanError> {
        self.skip_space();
        let from = self.at;
        let negative = self.bytes.get(self.at) == Some(&b'-');
        if negative {
            self.at += 1;
        }
        let digits_from = self.at;
        while matches!(self.bytes.get(self.at), Some(b'0'..=b'9')) {
            self.at += 1;
        }
        if self.at == digits_from {
            self.at = from;
            return self.refuse(EXPECTED_INTEGER);
        }
        if self.at - digits_from > 1 && self.bytes[digits_from] == b'0' {
            self.at = from;
            return self.refuse(LEADING_ZERO);
        }
        if matches!(self.bytes.get(self.at), Some(b'.' | b'e' | b'E')) {
            return self.refuse(NOT_AN_INTEGER);
        }
        let text = std::str::from_utf8(&self.bytes[from..self.at])
            .unwrap_or_else(|error| unreachable!("digits are utf-8: {error}"));
        match text.parse::<i64>() {
            Ok(value) => Ok(value),
            Err(_) => {
                self.at = from;
                self.refuse(format!("integer `{text}` does not fit i64"))
            }
        }
    }

    /// The literal `null`, if it is next.
    pub fn accept_null(&mut self) -> bool {
        self.skip_space();
        if self.bytes[self.at..].starts_with(b"null") {
            self.at += 4;
            return true;
        }
        false
    }

    /// Refuse anything after the object.
    pub fn expect_end(&mut self) -> Result<(), ScanError> {
        match self.peek() {
            None => Ok(()),
            Some(_) => self.refuse(TRAILING_INPUT),
        }
    }
}
