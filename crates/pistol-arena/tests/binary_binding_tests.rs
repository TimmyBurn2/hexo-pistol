//! Each engine seat is bound to a BINARY BY CONTENT, and a file that is not
//! that binary refuses the run before a game is played (docs/decisions.md
//! D-283).
//!
//! # The defect class this closes, which has exited 0 twice
//!
//! `target/release/pistol` is a different program after every build. A run
//! played by a STALE binary at that path is a different experiment reported
//! under this one's name, and the failure mode is silence: a decoy sitting where
//! cargo did not write is a regular file, is executable, speaks the protocol,
//! and plays every game. That was reproduced twice — `tools/tactical_check.sh`
//! (D-250) and `tools/arena_smoke.sh` (D-252), the second with 54 of 54 engine
//! invocations going to a decoy while the gate printed `ok` and exited 0.
//!
//! Both of those were closed inside `tools/`. The four OPERATOR-RUN SPRT
//! documents were not, and CLAUDE.md rule 6 makes them the judge of every search
//! and eval change: WP-1.5b's acceptance runs on a document of exactly this
//! shape. The binding lives in the DOCUMENT because the arena's only arguments
//! are `--config` and `--out` — the document is the seam.
//!
//! # RULE9-JUSTIFICATION: one binding, its schema half and its run half.
//!
//! The refusal is a shape rule at validation time and a content comparison at
//! run start, and testing either alone leaves the other's failure invisible: a
//! well-formed digest that is never compared, or a comparison against a value
//! the schema would have let through as an empty string. The sweep over the
//! committed documents is the third half — a rule with no instances in the tree
//! is a rule the next document will not have (D-251's class-versus-list ruling).

mod common;

use std::process::Command;

use common::{ARENA, Ran, Scratch, binding_digest, openings_prefix, repo, self_match};
use pistol_arena::config::ArenaConfig;
use pistol_arena::error::ArenaError;

/// Openings per run, and a cap that leaves room for a real game.
const OPENINGS: usize = 2;
const TURN_CAP: u32 = 8;

/// A digest that is well-formed and belongs to nothing.
const WRONG_DIGEST: &str = "deadbeef00000000000000000000000000000000000000000000000000000000";

/// Every committed arena document, by path.
fn committed_arena_configs() -> Vec<std::path::PathBuf> {
    let dir = repo().join("configs");
    let mut found: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
        .expect("configs/ is committed")
        .map(|entry| entry.expect("a readable directory entry").path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("arena_") && name.ends_with(".toml"))
        })
        .collect();
    found.sort();
    assert!(
        !found.is_empty(),
        "no arena documents under configs/, so the sweep below checks nothing"
    );
    found
}

/// Run the arena over `text` as its whole document.
fn run_document(scratch: &Scratch, tag: &str, text: &str) -> Ran {
    let config = scratch.write(&format!("arena-{tag}.toml"), text);
    let out = scratch.path(&format!("report-{tag}.txt"));
    let output = Command::new(ARENA)
        .arg("--config")
        .arg(&config)
        .arg("--out")
        .arg(&out)
        .output()
        .expect("the arena binary runs");
    Ran {
        report: std::fs::read_to_string(&out).ok(),
        output,
    }
}

fn stderr(ran: &Ran) -> String {
    String::from_utf8_lossy(&ran.output.stderr).into_owned()
}

/// The document a runnable self-match writes, and the digest both its seats
/// carry.
fn honest_document(scratch: &Scratch) -> String {
    let openings = scratch.write("openings.txt", &openings_prefix(OPENINGS));
    let stub = scratch.stub_config("honest.toml", "honest");
    self_match(&openings, &stub, OPENINGS, TURN_CAP, 1).render()
}

/// THE CONTROL. Without it the refusal below is satisfied by an arena that
/// refuses every document.
#[test]
fn a_document_whose_digest_matches_the_binary_plays_its_games() {
    let scratch = Scratch::new("bind-control");
    let text = honest_document(&scratch);
    assert!(
        text.contains(&binding_digest(common::STUB)),
        "the control document names the stub's own digest"
    );
    let ran = run_document(&scratch, "control", &text);
    assert_eq!(
        ran.code(),
        0,
        "a correctly bound self-match runs\nstderr: {}",
        stderr(&ran)
    );
    assert_eq!(
        ran.games().len(),
        OPENINGS * 2,
        "and it played its games, so the refusal below is about the digest"
    );
}

/// THE REFUSAL, end to end: one seat's digest is changed to a well-formed value
/// that belongs to no file, and the run stops before any game.
#[test]
fn arena_refuses_engine_binary_with_wrong_digest() {
    let scratch = Scratch::new("bind-wrong");
    let honest = honest_document(&scratch);
    let real = binding_digest(common::STUB);
    // ONE seat, so the test is about a mismatch and not about a document with
    // no correct digest in it anywhere.
    let text = honest.replacen(
        &format!("binary_sha256 = \"{real}\""),
        &format!("binary_sha256 = \"{WRONG_DIGEST}\""),
        1,
    );
    assert_ne!(text, honest, "the document was edited");
    assert!(
        text.contains(&real),
        "the other seat still carries the right digest"
    );

    let ran = run_document(&scratch, "wrong", &text);
    assert_ne!(ran.code(), 0, "a mismatched binary is not a run");
    let said = stderr(&ran);
    assert!(
        said.contains("EngineBinaryDigestMismatch"),
        "the refusal is the named one, not a spawn failure or a hang: {said}"
    );
    assert!(
        said.contains(WRONG_DIGEST) && said.contains(&real),
        "and it names both digests, so the operator can tell which end is stale: {said}"
    );
    assert!(
        ran.report.is_none(),
        "no report is written for a run that never started: {:?}",
        ran.report
    );
}

/// The schema half: a document that names no digest for a seat is refused by
/// the key, at load, with nothing spawned.
///
/// Driven over the COMMITTED documents rather than a synthetic one, because the
/// question is whether the binding reached the four operator-run SPRT
/// instruments rule 6 makes the judge — the exact place the last three sweeps
/// missed by scoping themselves to a directory (D-251's ruling).
#[test]
fn sprt_config_missing_digest_is_named_error() {
    for path in committed_arena_configs() {
        let text = std::fs::read_to_string(&path).expect("a committed document reads");
        let name = path.display().to_string();
        // The whole document loads first, or the refusal below could come from
        // anything.
        ArenaConfig::parse_unvalidated(&text)
            .and_then(|config| config.validate().map(|()| config))
            .unwrap_or_else(|error| panic!("{name} does not load as it stands: {error}"));

        let stripped: String = text
            .lines()
            .filter(|line| !line.starts_with("binary_sha256 = "))
            .map(|line| format!("{line}\n"))
            .collect();
        assert_ne!(stripped, text, "{name} carries a digest line to strip");

        let error = ArenaConfig::parse_unvalidated(&stripped)
            .and_then(|config| config.validate().map(|()| config))
            .expect_err(&format!("{name} without a digest must not load"));
        assert_eq!(error.name(), "Config", "{name}: {error}");
        assert!(
            error.to_string().contains("binary_sha256"),
            "{name}: the refusal names the missing key: {error}"
        );
    }
}

/// The sweep, in the class's own words: EVERY committed arena document binds
/// EVERY seat by content, and the digests are well-formed.
#[test]
fn every_committed_arena_config_binds_every_seat_by_content() {
    for path in committed_arena_configs() {
        let text = std::fs::read_to_string(&path).expect("a committed document reads");
        let name = path.display().to_string();
        let seats = text
            .lines()
            .filter(|line| line.starts_with("binary = "))
            .count();
        let digests: Vec<&str> = text
            .lines()
            .filter_map(|line| line.strip_prefix("binary_sha256 = "))
            .collect();
        assert_eq!(seats, 2, "{name} is a match between exactly two seats");
        assert_eq!(
            digests.len(),
            seats,
            "{name} binds {} of {seats} seats by content",
            digests.len()
        );
        for digest in digests {
            let inner = digest
                .strip_prefix('"')
                .and_then(|rest| rest.strip_suffix('"'))
                .unwrap_or_else(|| panic!("{name}: {digest} is not a quoted string"));
            assert_eq!(inner.len(), 64, "{name}: {inner} is not 64 characters");
            assert!(
                inner
                    .bytes()
                    .all(|b| b.is_ascii_digit() || b.is_ascii_lowercase() && b <= b'f'),
                "{name}: {inner} is not lowercase hex"
            );
        }
    }
}

/// A digest is validated by its SPELLING and not by its emptiness, because a
/// transcription that is uppercase, short or long would otherwise survive to a
/// comparison and be reported as a stale binary — sending the operator to look
/// at the wrong end of it.
#[test]
fn a_misspelled_digest_is_refused_at_the_key_rather_than_at_the_comparison() {
    let scratch = Scratch::new("bind-shape");
    let honest = honest_document(&scratch);
    let real = binding_digest(common::STUB);
    for bad in [
        String::new(),
        real.to_uppercase(),
        real[..63].to_string(),
        format!("{real}0"),
        format!("0x{}", &real[2..]),
    ] {
        let text = honest.replacen(
            &format!("binary_sha256 = \"{real}\""),
            &format!("binary_sha256 = \"{bad}\""),
            1,
        );
        let error = ArenaConfig::parse_unvalidated(&text)
            .and_then(|config| config.validate().map(|()| config))
            .expect_err(&format!("`{bad}` is not a digest"));
        assert!(
            matches!(&error, ArenaError::Config { key, .. } if key == "engine_a.binary_sha256"),
            "`{bad}` is refused at its own key: {error}"
        );
    }
}

/// A `binary` with no path separator is digested as a CWD-relative file and
/// SPAWNED THROUGH `$PATH`, so the file weighed and the file run are two
/// different files whenever both exist.
///
/// `identity::digest_of` calls `std::fs::read(path)`, which resolves against the
/// process's working directory and never consults `$PATH`. `Channel::start`
/// calls `Command::new(binary)`, which for a name containing no separator execs
/// through `execvp`, i.e. through `$PATH`. The comparison therefore attests one
/// file while the run plays another, and the report names the digest of the file
/// that did not move a stone. This is D-226's decided class, closed in
/// `tools/bench_delta.sh` and `tools/baseline_snapshot.sh` with `command -v` and
/// `realpath`, and it re-entered through the Rust seat.
#[test]
fn a_binary_with_no_path_separator_is_refused_rather_than_resolved_through_path() {
    let scratch = Scratch::new("bind-bare-name");
    let honest = honest_document(&scratch);
    let text = honest.replacen(
        &format!("binary = \"{}\"", common::STUB),
        "binary = \"pistol\"",
        1,
    );
    let error = ArenaConfig::parse_unvalidated(&text)
        .and_then(|config| config.validate().map(|()| config))
        .expect_err("a bare name is digested from the CWD and spawned from $PATH");
    assert!(
        matches!(&error, ArenaError::Config { key, .. } if key == "engine_a.binary"),
        "a separator-less binary is refused at its own key: {error}"
    );
}

/// A `binary` THAT IS NOT A REGULAR FILE REFUSES INSTEAD OF WEDGING THE RUN.
///
/// `digest_of` read the path with `fs::read`, which on a FIFO BLOCKS until a
/// writer appears. That read happens in `identity::capture`, before any channel
/// exists, so `run.hang_timeout_ms` does not apply: the arena waited forever and
/// printed nothing — a hang where a named refusal belongs. Both shell gates
/// already guard this case by name (`usable()` in `tools/arena_smoke.sh` tests
/// `-f` as well as `-x`); the Rust seat did not.
#[test]
fn a_binary_that_is_not_a_regular_file_is_refused_rather_than_waited_on() {
    let scratch = Scratch::new("bind-fifo");
    let fifo = scratch.path("engine.fifo");
    let made = Command::new("mkfifo")
        .arg(&fifo)
        .status()
        .expect("mkfifo runs");
    assert!(made.success(), "the fixture needs a FIFO at {fifo:?}");

    let error = pistol_arena::identity::digest_of(&fifo)
        .expect_err("a FIFO is not a build, and reading one never returns");
    let said = error.to_string();
    assert!(
        said.contains("not a regular file"),
        "the refusal says what the path is, rather than blocking: {said}"
    );
}
