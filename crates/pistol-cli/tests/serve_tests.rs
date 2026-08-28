mod common;

use std::io::Cursor;

use common::{GATE, engine};
use pistol_cli::{Session, serve};

/// Serve these bytes, and return the answer lines.
fn served(input: &[u8]) -> Vec<String> {
    let mut engine = engine(GATE);
    let mut session = Session::new(&mut engine);
    let mut reader = Cursor::new(input.to_vec());
    let mut written: Vec<u8> = Vec::new();
    serve(&mut session, &mut reader, &mut written).expect("writing to a vector cannot fail");
    String::from_utf8(written)
        .expect("every answer is text")
        .lines()
        .map(str::to_string)
        .collect()
}

#[test]
fn serve_answers_every_line_and_stops_at_quit() {
    let answers = served(b"pistol\nposition start\ngo depth_turns 1\nquit\nposition start\n");
    assert!(answers.iter().any(|line| line == "pistolok"));
    assert!(answers.iter().any(|line| line.starts_with("bestmove ")));
    assert!(
        !answers.iter().any(|line| line.starts_with("error ")),
        "nothing here is malformed: {answers:?}"
    );
    // `quit` stops the loop, so the line after it was never read — and if it had
    // been, it would have produced no answer either. What proves the stop is that
    // the loop returned before the input was exhausted, which it did.
}

#[test]
fn serve_ends_at_end_of_input_without_quit() {
    let answers = served(b"pistol\n");
    assert_eq!(answers.last().map(String::as_str), Some("pistolok"));
}

#[test]
fn serve_accepts_a_final_line_with_no_newline() {
    let answers = served(b"pistol");
    assert_eq!(answers.last().map(String::as_str), Some("pistolok"));
}

#[test]
fn serve_accepts_carriage_returns() {
    // A driver on another platform, or a fixture written there.
    let answers = served(b"position start\r\ngo depth_turns 1\r\nquit\r\n");
    assert!(
        !answers.iter().any(|line| line.starts_with("error ")),
        "a trailing carriage return is whitespace, not a syntax error: {answers:?}"
    );
    assert!(answers.iter().any(|line| line.starts_with("bestmove ")));
}

#[test]
fn serve_refuses_a_line_that_is_not_text_and_stays_alive() {
    // Lossy decoding would repair the input silently, and an early return would
    // let one byte end the session. Neither is acceptable, so it is a refusal like
    // any other.
    let answers = served(b"position start\n\xff\xfe not text\ngo depth_turns 1\nquit\n");
    let errors: Vec<&String> = answers
        .iter()
        .filter(|line| line.starts_with("error "))
        .collect();
    assert_eq!(errors.len(), 1, "one refusal for one bad line: {answers:?}");
    assert!(errors[0].starts_with("error Protocol: "), "{}", errors[0]);
    assert!(
        errors[0].contains("not UTF-8"),
        "the refusal says what is wrong: {}",
        errors[0]
    );
    assert!(
        answers.iter().any(|line| line.starts_with("bestmove ")),
        "the engine went on to answer the next line: {answers:?}"
    );
}

#[test]
fn serve_writes_nothing_but_protocol_lines() {
    // Every answer is one line, and every line starts with one of the protocol's
    // words. Anything else on this stream would desynchronize a driver.
    let answers = served(b"pistol\nnonsense\nposition start\ngo depth_turns 1\nquit\n");
    for line in &answers {
        let first = line
            .split_whitespace()
            .next()
            .unwrap_or_else(|| panic!("an empty answer line among {answers:?}"));
        assert!(
            matches!(first, "id" | "pistolok" | "info" | "bestmove" | "error"),
            "`{line}` is not a protocol answer"
        );
    }
}
