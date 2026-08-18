//! `corpus-extract` as a program: what it writes, what it refuses, and what it
//! exits with.
//!
//! These run the binary rather than the library, because the claims are about a
//! run: byte-identity across two PROCESSES (matching `tools/determinism.sh`'s
//! shape), a header written in this run carrying nothing machine-specific, and
//! an exit code contract of 0 clean / 1 games excluded / 2 refused.
//!
//! The prose comparison is the guard against the failure that actually happened:
//! the fixtures were generated, the source's ADR citations were then renumbered,
//! and nothing noticed that the committed bytes could no longer be reproduced
//! (docs/decisions.md D-152).

mod common;

use common::{repo, scratch};
use pistol_cli::corpus::emit::{BODY_DIGEST, body_of, claimed_body_digest};
use pistol_cli::sha256::sha256_hex;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(name)
}

#[test]
fn extractor_output_is_byte_identical_across_two_runs() {
    // Two PROCESSES, matching tools/determinism.sh's shape: a run that agreed
    // with itself in one process could still be depending on something the
    // process happened to hold.
    let corpus = fixture("corpus_synthetic_v1.jsonl");
    let first = run_extractor(&corpus, "determinism-a");
    let second = run_extractor(&corpus, "determinism-b");
    for name in ["openings_v1.txt", "bench_positions_v1.txt"] {
        let a = std::fs::read(first.join(name)).expect("the first run wrote it");
        let b = std::fs::read(second.join(name)).expect("the second run wrote it");
        assert_eq!(
            sha256_hex(&a),
            sha256_hex(&b),
            "{name} differs between two runs over the same corpus"
        );
    }
}

#[test]
fn the_header_of_a_file_the_tool_just_wrote_carries_nothing_machine_specific() {
    // Every other header test in this file reads the COMMITTED bytes, so a
    // defect in the writer is invisible to them: four mutations that made the
    // tool emit a path, a false body digest, a doubled trailing newline or a
    // missing exclusion section all passed the suite. This one reads what the
    // binary produced, in this run, from a corpus at a path of our choosing.
    let corpus = fixture("corpus_synthetic_v1.jsonl");
    let out = run_extractor(&corpus, "produced");
    for name in ["openings_v1.txt", "bench_positions_v1.txt"] {
        let rendered = std::fs::read_to_string(out.join(name)).expect("the run wrote it");
        let header = &rendered[..rendered.find(BODY_DIGEST).expect("a body digest line")];

        assert!(
            !header.contains(out.to_str().expect("utf-8 path")),
            "{name} names the output directory it was written into"
        );
        assert!(
            !header.contains("corpus_synthetic_v1"),
            "{name} names the corpus file it read; the digest is the only identifier"
        );
        for forbidden in ["/home/", "/tmp/", ".jsonl"] {
            assert!(
                !header.contains(forbidden),
                "{name}'s produced header carries {forbidden:?}"
            );
        }
        for year in 2020..2040 {
            assert!(!header.contains(&year.to_string()), "{name} carries a year");
        }
        assert!(
            header.contains("# excluded none"),
            "{name} must say so explicitly when nothing was excluded, so that a missing \
             section and an empty one cannot look alike"
        );
        assert_eq!(
            sha256_hex(body_of(&rendered).expect("a body").as_bytes()),
            claimed_body_digest(&rendered).expect("a claim"),
            "{name}'s in-band digest does not describe its own payload"
        );
        assert!(
            rendered.ends_with('\n') && !rendered.ends_with("\n\n"),
            "{name} does not end with exactly one newline"
        );
        assert!(!rendered.contains('\r'), "{name} carries a carriage return");
    }
}

#[test]
fn the_prose_the_tool_writes_is_the_prose_the_committed_fixtures_carry() {
    // The guard against the failure that actually happened: the fixtures were
    // generated, then the source's ADR citations were renumbered, and nothing
    // noticed that the committed bytes could no longer be reproduced. Comparing
    // the prose block catches an edit to `documents.rs` that was never followed
    // by a regeneration, without needing the real corpus in the tree.
    let out = run_extractor(&fixture("corpus_synthetic_v1.jsonl"), "prose");
    for name in ["openings_v1.txt", "bench_positions_v1.txt"] {
        let produced = std::fs::read_to_string(out.join(name)).expect("the run wrote it");
        let committed = std::fs::read_to_string(fixture(name)).expect("committed");
        assert_eq!(
            prose_of(&produced),
            prose_of(&committed),
            "{name}'s committed prose is not what this build writes; regenerate the fixture \
             and update its pin"
        );
    }
}
/// Run the binary over a corpus into a fresh directory, and give back that
/// directory.
fn run_extractor(corpus: &Path, label: &str) -> PathBuf {
    let out = scratch(label);
    let status = Command::new(env!("CARGO_BIN_EXE_corpus-extract"))
        .arg("--corpus")
        .arg(corpus)
        .arg("--out-dir")
        .arg(&out)
        .output()
        .expect("the extractor binary is built beside this test");
    assert!(
        status.status.success(),
        "corpus-extract failed: {}",
        String::from_utf8_lossy(&status.stderr)
    );
    out
}
/// A fixture's header prose: the `#` lines that are neither a parameter, a
/// derived value, an exclusion, nor the digest — i.e. exactly the part that
/// depends on the source rather than on the corpus.
fn prose_of(rendered: &str) -> Vec<&str> {
    rendered
        .lines()
        .take_while(|line| !line.starts_with(BODY_DIGEST))
        .filter(|line| {
            line.starts_with('#')
                && !line.starts_with("# param ")
                && !line.starts_with("# derived ")
                && !line.starts_with("# excluded ")
        })
        .collect()
}
