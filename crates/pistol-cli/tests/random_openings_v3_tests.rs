mod common;

use std::collections::BTreeSet;
use std::path::PathBuf;

use common::repo;
use pistol_cli::corpus::emit::{BODY_DIGEST, body_of, claimed_body_digest};
use pistol_cli::random_openings::config::RandomOpeningsConfig;
use pistol_cli::random_openings::error::RandomOpeningsError;
use pistol_cli::random_openings::{self, BookVersion, document, filter};
use pistol_cli::sha256::sha256_hex;

/// The SHA-256 of the committed `fixtures/random_openings_v3.txt`.
const RANDOM_OPENINGS_V3_SHA256: &str =
    "9453763625c83a8a95d31bf5ee62e5ec6db91552d0fe81a604b190067e32f240";

/// What the book must hold, and it is DERIVED rather than chosen: `pairs_v3` is
/// the smallest cap `sprt_power.rs` measures at power >= 0.90 for the R7
/// question, and `ceil_to_500(7800 + 500)` applies book_v2_registration.md §4
/// (docs/decisions.md D-643).
const OPENINGS: usize = 8500;

const CONFIG: &str = "configs/random_openings_v3.toml";

fn fixture(book: BookVersion) -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(book.file_name())
}

fn committed(book: BookVersion) -> String {
    std::fs::read_to_string(fixture(book))
        .unwrap_or_else(|why| panic!("{} is committed: {why}", book.label()))
}

fn keys(book: BookVersion) -> BTreeSet<filter::Key> {
    filter::keys_of_document(book.label(), &committed(book)).expect("a committed book reads")
}

/// The v3 book this build produces, through the whole shipped pipeline.
///
/// Generate, filter against the two committed books, render — the same library
/// calls `src/bin/build-book-v3.rs` makes, so this checks the pipeline rather
/// than a copy of it.
fn produced() -> String {
    let config = RandomOpeningsConfig::load(&repo(CONFIG)).expect("the v3 config loads");
    let mut excluded = keys(BookVersion::V1);
    excluded.extend(keys(BookVersion::V2));
    let drawn = random_openings::generate(&config).expect("the v3 config generates");
    let (book, _) = filter::retain_disjoint(drawn, &excluded, OPENINGS, config.generate.n_openings)
        .expect("the committed n_openings leaves exactly the size asked for");
    document::render(&config, &book)
}

#[test]
fn the_filter_rejects_exactly_the_gap_the_header_states() {
    // THE COUNT THE TOOL PRINTS AND THE HEADER IMPLIES, PINNED. Without this the
    // `rejected` return is defended by nothing: it reaches only a `println!`, so
    // a counter that drifted would have the tool announce one number while the
    // book it wrote in the same run implied another (REVIEW-impl M-5).
    let config = RandomOpeningsConfig::load(&repo(CONFIG)).expect("the v3 config loads");
    let mut excluded = keys(BookVersion::V1);
    excluded.extend(keys(BookVersion::V2));
    let drawn = random_openings::generate(&config).expect("the v3 config generates");
    let (_, rejected) =
        filter::retain_disjoint(drawn, &excluded, OPENINGS, config.generate.n_openings)
            .expect("the committed n_openings leaves exactly the size asked for");
    assert_eq!(
        rejected,
        config.generate.n_openings - OPENINGS,
        "the draw minus the book IS the rejection count, which is what the header's two \
         numbers say and what makes `nothing is truncated` checkable"
    );
    assert_eq!(rejected, 38, "the count D-647 states");
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

#[test]
fn random_openings_v3_matches_its_pinned_digest() {
    // The out-of-band pin (CLAUDE.md rule 7), which catches strictly more than
    // the in-band one: an edit rewriting the body AND its `# body_sha256` line
    // is self-consistent and is caught only here.
    assert_eq!(
        sha256_hex(committed(BookVersion::V3).as_bytes()),
        RANDOM_OPENINGS_V3_SHA256,
        "the book's bytes are pinned in this file (RANDOM_OPENINGS_V3_SHA256)"
    );
}

#[test]
fn random_openings_v3_is_what_this_build_produces() {
    // The rebuild instruction in `configs/random_openings_v3.toml`, executed.
    // Unlike v1's and v2's siblings this one has EXTERNAL INPUT — the two
    // committed books — so what it proves is the weaker property D-647 selected:
    // reproducible from the config and two digest-pinned files in this same
    // repository, which is still every input a fresh clone has.
    assert_eq!(
        produced(),
        committed(BookVersion::V3),
        "the committed v3 bytes are the bytes this build writes"
    );
}

#[test]
fn random_openings_v3_body_digest_describes_its_own_body() {
    let text = committed(BookVersion::V3);
    assert_eq!(
        claimed_body_digest(&text).expect("the header states a body digest"),
        sha256_hex(
            body_of(&text)
                .expect("the header ends and a body follows")
                .as_bytes()
        ),
        "{BODY_DIGEST} must describe the body under it"
    );
}

#[test]
fn random_openings_v3_holds_the_openings_it_was_asked_for_and_not_the_draw() {
    // The two numbers differ ON PURPOSE and the gap is the whole filter story:
    // `n_openings` is the OVER-generation and `openings` is the book.
    let text = committed(BookVersion::V3);
    let config = RandomOpeningsConfig::load(&repo(CONFIG)).expect("the v3 config loads");
    assert_eq!(
        body_of(&text)
            .expect("the header ends and a body follows")
            .lines()
            .count(),
        OPENINGS,
        "the body holds the size the registration derived"
    );
    assert!(
        config.generate.n_openings > OPENINGS,
        "n_openings is the over-generation and must exceed the book"
    );
    assert!(
        text.contains(&format!("derived openings {OPENINGS}")),
        "the header states the book's own size"
    );
    assert!(
        text.contains(&format!("param n_openings {}", config.generate.n_openings)),
        "the header states the draw it was filtered from"
    );
}

#[test]
fn book_v3_shares_no_canonical_form_with_either_committed_book() {
    // THE PROPERTY THE FILTER EXISTS FOR (D-644), and the contrast is the point:
    // v1 and v2 were drawn independently and share 10 positions up to symmetry,
    // which `the_two_books_overlap_only_as_far_as_independent_drawing_makes_them`
    // pins. v3 shares none, and that is not luck — 38 candidates were rejected
    // to make it so (docs/decisions.md D-647).
    let v3 = keys(BookVersion::V3);
    assert_eq!(
        v3.len(),
        OPENINGS,
        "every opening is its own canonical form"
    );
    assert_eq!(
        v3.intersection(&keys(BookVersion::V1)).count(),
        0,
        "v3 vs v1, under canonical form"
    );
    assert_eq!(
        v3.intersection(&keys(BookVersion::V2)).count(),
        0,
        "v3 vs v2, under canonical form"
    );
}

#[test]
fn the_filter_refuses_a_survivor_count_that_is_not_the_size_asked_for() {
    // A book shortened in silence is a sample size nobody chose (rule 6), so the
    // filter refuses and names the `n_openings` that would land on the size —
    // the number a reader then puts in the config.
    let config = RandomOpeningsConfig::load(&repo(CONFIG)).expect("the v3 config loads");
    let mut excluded = keys(BookVersion::V1);
    excluded.extend(keys(BookVersion::V2));
    let drawn = random_openings::generate(&config).expect("the v3 config generates");
    let asked = OPENINGS - 1;
    match filter::retain_disjoint(drawn, &excluded, asked, config.generate.n_openings) {
        Err(RandomOpeningsError::SurvivorCount {
            wanted,
            survived,
            suggestion,
        }) => {
            assert_eq!(wanted, asked);
            assert_eq!(survived, OPENINGS);
            assert_eq!(
                suggestion,
                config.generate.n_openings as i64 - 1,
                "one fewer survivor wanted is one fewer to draw"
            );
        }
        other => panic!("a survivor count that is not the size must be refused: {other:?}"),
    }
}

#[test]
fn the_three_books_are_written_to_different_files() {
    let names = [BookVersion::V1, BookVersion::V2, BookVersion::V3]
        .map(BookVersion::file_name)
        .into_iter()
        .collect::<BTreeSet<_>>();
    assert_eq!(names.len(), 3, "one file name per book");
}
