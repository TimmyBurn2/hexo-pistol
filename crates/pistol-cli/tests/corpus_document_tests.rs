//! The committed fixtures as documents: their payload round-trips, their
//! in-band digest describes their own body, and their bytes match their pins.
//!
//! Nothing here reads the real corpus — it is an external artifact and is not in
//! the tree (CLAUDE.md rule 8), so a test that needed it would break the
//! fresh-clone gate. The pins are what stand in for it.

mod common;

use common::repo;
use pistol_cli::corpus::emit::{BODY_DIGEST, body_of, claimed_body_digest};
use pistol_cli::sha256::sha256_hex;
use pistol_core::GameState;
use pistol_engine::PositionSpec;
use std::path::PathBuf;

/// The SHA-256 of the committed `fixtures/openings_v1.txt`.
const OPENINGS_V1_SHA256: &str = "947284be96d89aa8ad9c4723ddb55a1e4077ad7712fca7fa10e2228fc5744727";
/// The SHA-256 of the committed `fixtures/bench_positions_v1.txt`.
const BENCH_POSITIONS_V1_SHA256: &str =
    "22beb57feb845c0c10c0e132ebe59cdce9b21c7ffc8e42ded3c6774d6416af00";

fn fixture(name: &str) -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(name)
}

/// The position a payload line names, replayed.
fn replay_line(line: &str) -> GameState {
    let tail = line.split(" #").next().expect("a line has a first piece");
    let spec: PositionSpec = tail.parse().unwrap_or_else(|error| {
        panic!("an emitted line must parse as a position: {line:?}: {error}")
    });
    spec.replay()
        .unwrap_or_else(|error| panic!("an emitted position must replay: {line:?}: {error}"))
}

/// The payload lines of a rendered fixture, comments dropped.
fn payload_lines(rendered: &str) -> Vec<&str> {
    body_of(rendered)
        .expect("a rendered fixture has a body")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .collect()
}

#[test]
fn every_emitted_line_replays_to_the_position_it_claims() {
    // The round trip the whole fixture form rests on: what is written parses
    // back through the same grammar and reaches the same position.
    for name in ["openings_v1.txt", "bench_positions_v1.txt"] {
        let rendered = std::fs::read_to_string(fixture(name))
            .unwrap_or_else(|error| panic!("{name} is committed: {error}"));
        let lines = payload_lines(&rendered);
        assert!(!lines.is_empty(), "{name} has payload lines");
        for line in lines {
            let tail = line.split(" #").next().expect("a line has a first piece");
            let spec: PositionSpec = tail
                .parse()
                .unwrap_or_else(|error| panic!("{line:?} does not parse: {error}"));
            assert_eq!(
                spec.to_string(),
                tail,
                "one position has one spelling: what was written must be what the grammar \
                 writes back (docs/decisions.md D-46)"
            );
            let state = replay_line(line);
            assert!(
                !state.outcome().is_decided(),
                "an emitted position must be one a search can stand on: {line}"
            );
        }
    }
}

#[test]
fn fixture_headers_carry_corpus_sha_and_exclusions() {
    for name in ["openings_v1.txt", "bench_positions_v1.txt"] {
        let rendered = std::fs::read_to_string(fixture(name))
            .unwrap_or_else(|error| panic!("{name} is committed: {error}"));
        let header = &rendered[..rendered.find(BODY_DIGEST).expect("a body digest line")];

        assert!(
            header.contains("# derived corpus_sha256 "),
            "{name} must name the corpus by digest"
        );
        assert!(
            header.contains("# excluded "),
            "{name} must have an exclusion section, even when it is empty"
        );
        assert!(
            header.contains("# derived games_read "),
            "{name} must say how many games it read"
        );
        // No path, no timestamp, no hostname: two machines given the same bytes
        // must write the same file (CLAUDE.md rule 4). A previous version of
        // this loop listed a year-like substring and then skipped it, which read
        // as if dates were covered when nothing was checking them.
        for forbidden in [
            "/home/",
            "/tmp/",
            "hexo_human_corpus.jsonl",
            ".jsonl",
            "T00:",
            "Z\n",
        ] {
            assert!(
                !header.contains(forbidden),
                "{name}'s header carries {forbidden:?}, which is machine- or run-specific"
            );
        }
        for year in 2020..2040 {
            assert!(
                !header.contains(&year.to_string()),
                "{name}'s header carries {year}, which looks like a timestamp"
            );
        }
    }
}

#[test]
fn a_fixture_header_states_the_digest_of_its_own_payload() {
    for name in ["openings_v1.txt", "bench_positions_v1.txt"] {
        let rendered = std::fs::read_to_string(fixture(name))
            .unwrap_or_else(|error| panic!("{name} is committed: {error}"));
        let body = body_of(&rendered).expect("a body");
        let claimed = claimed_body_digest(&rendered).expect("a claim");
        assert_eq!(
            sha256_hex(body.as_bytes()),
            claimed,
            "{name}'s payload does not match the digest its header states"
        );
        assert!(
            rendered.ends_with('\n') && !rendered.ends_with("\n\n"),
            "{name} ends with exactly one newline"
        );
        assert!(!rendered.contains('\r'), "{name} has no carriage returns");
    }
}

#[test]
fn committed_fixtures_match_their_pinned_sha256() {
    for (name, pin) in [
        ("openings_v1.txt", OPENINGS_V1_SHA256),
        ("bench_positions_v1.txt", BENCH_POSITIONS_V1_SHA256),
    ] {
        let bytes = std::fs::read(fixture(name))
            .unwrap_or_else(|error| panic!("{name} is committed: {error}"));
        assert_eq!(
            sha256_hex(&bytes),
            pin,
            "{name} changed without its pin being updated, which is what the pin is for"
        );
    }
}
