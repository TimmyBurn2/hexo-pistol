//! Reading lines and writing answers.
//!
//! The loop is separate from the protocol so that the protocol can be exercised
//! without a pipe: [`crate::protocol::Session`] turns one line into its answers
//! in memory, and this adds the input, the output and the flushing.
//!
//! Input is read as **bytes** and only then read as text. A line that is not
//! UTF-8 is a line the engine cannot understand, which is a refusal like any
//! other — one `error Protocol` line, and the engine stays alive (CLAUDE.md
//! rule 3). Decoding lossily instead would repair the input silently, which is
//! the failure mode this project forbids; killing the loop would let one bad
//! byte end a match.

use std::io::{self, BufRead, Write};

use pistol_engine::EngineError;

use crate::protocol::{Flow, Session};
use crate::report::error_line;

/// How many bytes of an undecodable line are quoted back in the refusal. Enough
/// to recognise it, short enough not to flood a log with a binary file somebody
/// piped in by accident.
const QUOTED_BYTES: usize = 32;

/// Serve the line protocol over `input`, answering on `output`, until `quit` or
/// end of input.
///
/// Everything the protocol says goes to `output` — refusals included, because a
/// refusal is an answer (docs/decisions.md D-5). One flush per input line, on
/// purpose: a driver that writes a line and waits for the answer must never be
/// able to wait forever because the answer is sitting in a buffer. That matters
/// for the cross-process determinism gate, which is two of these talking to a
/// script.
///
/// An I/O failure is returned rather than swallowed, and the answers already
/// written for the failing line are not retried (CLAUDE.md rule 3).
pub fn serve(
    session: &mut Session<'_>,
    input: &mut dyn BufRead,
    output: &mut dyn Write,
) -> io::Result<()> {
    let mut bytes: Vec<u8> = Vec::new();
    loop {
        bytes.clear();
        if input.read_until(b'\n', &mut bytes)? == 0 {
            // End of input is end of conversation: a driver that closed its pipe
            // has asked for nothing more.
            return Ok(());
        }
        let mut failure: Option<io::Error> = None;
        let mut write = |answer: &str| {
            if failure.is_some() {
                // The line's remaining answers are dropped, and the failure is
                // returned below rather than hidden.
                return;
            }
            if let Err(io) = writeln!(output, "{answer}") {
                failure = Some(io);
            }
        };

        let flow = match std::str::from_utf8(&bytes) {
            Ok(line) => session.line(line, &mut write),
            Err(error) => {
                write(&error_line(&undecodable(&bytes, error)));
                Flow::Continue
            }
        };

        if let Some(io) = failure {
            return Err(io);
        }
        output.flush()?;
        if flow == Flow::Quit {
            return Ok(());
        }
    }
}

/// The refusal for a line that is not text.
fn undecodable(bytes: &[u8], error: std::str::Utf8Error) -> EngineError {
    let head: Vec<String> = bytes
        .iter()
        .take(QUOTED_BYTES)
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let elided = if bytes.len() > QUOTED_BYTES {
        "…"
    } else {
        ""
    };
    EngineError::Protocol {
        line: format!("{}{elided}", head.join(" ")),
        why: format!(
            "not UTF-8 at byte {}: every protocol line is text",
            error.valid_up_to()
        ),
    }
}
