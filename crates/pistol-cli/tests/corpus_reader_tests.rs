mod common;

use common::{repo, scratch_file};
use pistol_cli::corpus::{read, record};
use pistol_cli::sha256::sha256_hex;
use pistol_core::{Coord, Player};
use std::path::PathBuf;

/// The SHA-256 of `tests/fixtures/corpus_synthetic_v1.jsonl`.
const SYNTHETIC_SHA256: &str = "8c62654c08729b68212a086dedd2ac8c829bbbc7e3620995519238631c92b9e4";

/// The committed synthetic corpus.
fn synthetic() -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures/corpus_synthetic_v1.jsonl")
}

/// Read one line of corpus text, expecting a refusal, and give back its wording.
fn refusal(line: &str) -> String {
    let path = scratch_file("refusal", "corpus.jsonl", line);
    let text = std::fs::read_to_string(&path).expect("just written");
    match read(&path, &text) {
        Ok(records) => panic!("expected a refusal, read {} records", records.len()),
        Err(error) => error.to_string(),
    }
}

/// A legal one-line corpus with these moves, as a record.
fn one_game(moves: &str, winner: i32) -> String {
    format!(
        "{{\"game_hash\":\"0123456789abcdef\",\"moves\":{moves},\"winner\":{winner},\
         \"source\":\"human\",\"elo\":[1200,1210]}}"
    )
}

#[test]
fn synthetic_corpus_matches_its_pinned_sha256() {
    let bytes = std::fs::read(synthetic()).expect("the synthetic corpus is committed");
    assert_eq!(
        sha256_hex(&bytes),
        SYNTHETIC_SHA256,
        "the synthetic corpus changed; every expectation below is a claim about its contents"
    );
}

#[test]
fn extractor_rejects_malformed_jsonl_line_with_named_error() {
    // Each of these is valid JSON, or nearly so, and none of them is this
    // schema. A reader that repaired any of them would be curating from a
    // document other than the one its header names (CLAUDE.md rule 3).
    let cases: [(&str, &str); 9] = [
        ("{", "expected a quoted string"),
        ("not json at all", pistol_cli::corpus::json::EXPECTED_OBJECT),
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[0,0]],\"winner\":1,\"source\":\"human\",\"elo\":[1200,1210],\"extra\":1}",
            record::UNKNOWN_KEY,
        ),
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[0,0]],\"winner\":1,\"source\":\"human\"}",
            record::ELO_KEY_REQUIRED,
        ),
        (
            "{\"moves\":[[0,0]],\"winner\":1,\"source\":\"human\",\"elo\":[1,2]}",
            record::MISSING_KEY,
        ),
        (
            "{\"game_hash\":\"ABCDEF0123456789\",\"moves\":[[0,0]],\"winner\":1,\"source\":\"human\",\"elo\":[1,2]}",
            record::BAD_GAME_HASH,
        ),
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[0,0]],\"winner\":2,\"source\":\"human\",\"elo\":[1,2]}",
            record::BAD_WINNER,
        ),
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[0,0]],\"winner\":1,\"source\":\"engine\",\"elo\":[1,2]}",
            record::BAD_SOURCE,
        ),
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[99999,0]],\"winner\":1,\"source\":\"human\",\"elo\":[1,2]}",
            record::COORD_OUT_OF_RANGE,
        ),
    ];
    for (line, expected) in cases {
        let said = refusal(line);
        assert!(
            said.contains(expected),
            "reading {line:?}\n  expected a refusal naming {expected:?}\n  got {said:?}"
        );
        assert!(
            said.contains(":1"),
            "the refusal must name the line: {said}"
        );
    }
}

#[test]
fn extractor_rejects_a_line_this_reader_deliberately_narrows_away() {
    // Valid JSON that this reader refuses on purpose. Each is a feature the
    // schema does not use, and reading one would mean reading a different
    // document (docs/decisions.md D-139).
    for (line, expected) in [
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[0,0]],\"winner\":1,\"source\":\"hum\\u0061n\",\"elo\":[1,2]}",
            pistol_cli::corpus::json::ESCAPE_UNSUPPORTED,
        ),
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[0.0,0]],\"winner\":1,\"source\":\"human\",\"elo\":[1,2]}",
            pistol_cli::corpus::json::NOT_AN_INTEGER,
        ),
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[00,0]],\"winner\":1,\"source\":\"human\",\"elo\":[1,2]}",
            pistol_cli::corpus::json::LEADING_ZERO,
        ),
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[0,0]],\"winner\":1,\"source\":\"human\",\"elo\":[1,2]} trailing",
            pistol_cli::corpus::json::TRAILING_INPUT,
        ),
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[0,0,0]],\"winner\":1,\"source\":\"human\",\"elo\":[1,2]}",
            record::BAD_MOVE_ARITY,
        ),
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[0,0]],\"winner\":1,\"source\":\"human\",\"elo\":[1,2,3]}",
            record::BAD_ELO_ARITY,
        ),
        (
            "{\"game_hash\":\"0123456789abcdef\",\"moves\":[[0,0]],\"winner\":1,\"source\":\"human\",\"elo\":[9999,1]}",
            record::ELO_OUT_OF_RANGE,
        ),
    ] {
        let said = refusal(line);
        assert!(
            said.contains(expected),
            "reading {line:?}\n  expected {expected:?}\n  got {said:?}"
        );
    }
}

#[test]
fn extractor_rejects_a_carriage_return_and_a_blank_line() {
    // Both would change the file's digest without changing a single game, which
    // is exactly what CLAUDE.md rule 4 cannot allow to pass silently.
    let good = one_game("[[0,0]]", 1);
    assert!(refusal(&format!("{good}\r")).contains(pistol_cli::corpus::CARRIAGE_RETURN));
    assert!(refusal(&format!("{good}\n\n{good}")).contains(pistol_cli::corpus::BLANK_LINE));
}

#[test]
fn extractor_rejects_a_repeated_game_hash_and_says_which_kind() {
    // The corpus states game_hash is its dedupe key. A repeat with the same
    // moves is the same game exported twice; a repeat with different moves is a
    // truncated-digest collision or a corrupt export. They are different
    // findings and are named differently.
    let same = one_game("[[0,0]]", 1);
    let said = refusal(&format!("{same}\n{same}"));
    assert!(
        said.contains(pistol_cli::corpus::REPEATED_GAME_HASH_SAME_MOVES),
        "{said}"
    );
    assert!(
        said.contains("line 1"),
        "it must name the first sighting: {said}"
    );

    let other = one_game("[[0,0],[1,0],[2,0]]", 1);
    let said = refusal(&format!("{same}\n{other}"));
    assert!(
        said.contains(pistol_cli::corpus::REPEATED_GAME_HASH_OTHER_MOVES),
        "{said}"
    );
}

#[test]
fn element_zero_is_q_not_r() {
    // Nothing downstream can catch a swap: q<->r is one of the twelve lattice
    // symmetries, so a swapped reader yields a legal, self-consistent, reflected
    // corpus with zero exclusions (docs/decisions.md D-140). So it is pinned
    // here, on a move whose two components differ.
    let line = one_game("[[0,0],[3,-1]]", 1);
    let path = scratch_file("mapping", "corpus.jsonl", &line);
    let text = std::fs::read_to_string(&path).unwrap();
    let records = read(&path, &text).expect("well formed");
    assert_eq!(records[0].moves[1], Coord::new(3, -1));
    assert_eq!(records[0].moves[1].q, 3);
    assert_eq!(records[0].moves[1].r, -1);
}

#[test]
fn a_record_reports_its_own_rating_summary() {
    let path = scratch_file(
        "ratings",
        "corpus.jsonl",
        &format!(
            "{}\n{}",
            one_game("[[0,0]]", 1),
            "{\"game_hash\":\"fedcba9876543210\",\"moves\":[[0,0]],\"winner\":-1,\
             \"source\":\"human\",\"elo\":[null,1200]}"
        ),
    );
    let text = std::fs::read_to_string(&path).unwrap();
    let records = read(&path, &text).unwrap();
    assert_eq!(records[0].min_elo(), Some(1200));
    assert_eq!(records[0].elo_gap(), Some(10));
    assert_eq!(records[0].winner, Player::P1);
    assert_eq!(
        records[1].min_elo(),
        None,
        "a missing rating has no minimum"
    );
    assert_eq!(records[1].elo_gap(), None);
}
