mod common;

use std::path::PathBuf;

use common::repo;
use pistol_cli::random_openings::BookVersion;

/// What the finished book holds. Restated rather than imported from the book's
/// own suite: this file checks the TOOL, and agreeing with that suite by
/// construction would prove nothing.
const OPENINGS: usize = 8500;

const CONFIG: &str = "configs/random_openings_v3.toml";

fn fixture(book: BookVersion) -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(book.file_name())
}

fn committed(book: BookVersion) -> String {
    std::fs::read_to_string(fixture(book))
        .unwrap_or_else(|why| panic!("{} is committed: {why}", book.label()))
}

/// The shipped `build-book-v3` binary, located the way cargo guarantees.
///
/// A BINARY and not an example, and the difference is not cosmetic: cargo sets
/// `CARGO_BIN_EXE_*` and builds every bin target for an integration test, but it
/// builds EXAMPLES only when target selection includes them. As an example this
/// tool was present under a bare `cargo test` and absent under
/// `cargo test --test random_openings_v3_tests`, so these two tests passed in CI
/// and failed under a narrower invocation — which voided a whole mutation run
/// while every gate stayed green.
fn shipped_builder() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_build-book-v3"))
}

#[test]
fn the_documented_rebuild_command_writes_the_committed_book() {
    // THE COMMAND `configs/random_openings_v3.toml` TELLS A READER TO RUN, run.
    // The library-level test checks the composition; this checks the TOOL that
    // composes it — the --against union, the --openings value and the output
    // path are the example's own and nothing else drives them (REVIEW-impl M-4;
    // D-518 records a real defect in exactly this composition).
    let out = common::scratch("v3-shipped-builder");
    let ran = std::process::Command::new(shipped_builder())
        .args(["--config", &repo(CONFIG).display().to_string()])
        .args(["--against", &fixture(BookVersion::V1).display().to_string()])
        .args(["--against", &fixture(BookVersion::V2).display().to_string()])
        .args(["--openings", &OPENINGS.to_string()])
        .args(["--out-dir", &out.display().to_string()])
        .output()
        .expect("the example runs");
    assert!(
        ran.status.success(),
        "the documented command must succeed: {}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
    let written = std::fs::read_to_string(out.join(BookVersion::V3.file_name()))
        .expect("the run wrote the book under its own version's name");
    assert_eq!(
        written,
        committed(BookVersion::V3),
        "the committed v3 bytes are the bytes the DOCUMENTED command writes"
    );
}

#[test]
fn the_builder_refuses_a_book_it_was_not_told_to_be_disjoint_from() {
    // Drop one --against and the tool must refuse rather than write a book that
    // overlaps v1: the survivor count no longer lands on the size asked for.
    let out = common::scratch("v3-missing-against");
    let ran = std::process::Command::new(shipped_builder())
        .args(["--config", &repo(CONFIG).display().to_string()])
        .args(["--against", &fixture(BookVersion::V2).display().to_string()])
        .args(["--openings", &OPENINGS.to_string()])
        .args(["--out-dir", &out.display().to_string()])
        .output()
        .expect("the example runs");
    assert!(
        !ran.status.success(),
        "a filter missing an input must refuse"
    );
    let said = String::from_utf8_lossy(&ran.stderr).to_string();
    assert!(
        said.contains("survivors") && said.contains("set n_openings to"),
        "the refusal names the survivor count and the number that would land: {said}"
    );
    assert!(
        !out.join(BookVersion::V3.file_name()).exists(),
        "a refused run writes no book"
    );
}
