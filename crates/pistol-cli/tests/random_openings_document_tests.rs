//! `random_openings_v1.txt` as a document: its pin, its in-band digest, its
//! header, and the one property its corpus siblings can never have — that this
//! build reproduces it byte for byte.
//!
//! WP-1.2a's review found fixtures whose committed bytes the build could no
//! longer produce, because every test read the committed file and none compared
//! it against freshly generated output (docs/decisions.md D-152). That hole is
//! closed here rather than mitigated: this book's only input is a committed
//! config, so the regeneration is a test rather than an errand with an external
//! corpus in it.

mod common;

use common::repo;
use pistol_cli::corpus::emit::{BODY_DIGEST, body_of, claimed_body_digest};
use pistol_cli::random_openings::config::RandomOpeningsConfig;
use pistol_cli::random_openings::{self, FILE_NAME, document};
use pistol_cli::sha256::sha256_hex;
use std::path::PathBuf;

/// The SHA-256 of the committed `fixtures/random_openings_v1.txt`.
const RANDOM_OPENINGS_V1_SHA256: &str =
    "895a05edb53a0a8d89c262bb058e3bc3dd24d446405d375458aaf067e2f076e7";

fn book_path() -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(FILE_NAME)
}

fn committed_text() -> String {
    std::fs::read_to_string(book_path()).expect("the book is committed")
}

/// The book this build produces from the committed config.
fn produced() -> String {
    let config = RandomOpeningsConfig::load(&repo("configs/random_openings_v1.toml"))
        .expect("the committed config loads");
    let book = random_openings::generate(&config).expect("the committed config generates");
    document::render(&config, &book)
}

#[test]
fn random_openings_v1_is_what_this_build_produces() {
    // The strongest form of a fixture pin: not "these bytes have this digest"
    // but "these bytes are the ones the tool writes today". An ADR citation
    // edited in the header prose without a regeneration is a red test here,
    // which is exactly what D-152 could not see.
    assert_eq!(
        produced(),
        committed_text(),
        "regenerate with `cargo run -p pistol-cli --bin random-openings -- \
         --config configs/random_openings_v1.toml \
         --out-dir crates/pistol-cli/tests/fixtures`"
    );
}

#[test]
fn random_openings_v1_matches_its_pinned_digest() {
    // The out-of-band pin, the convention `tactical_v0.txt` and the corpus
    // fixtures already use (CLAUDE.md rule 7). It catches strictly more than the
    // in-band digest: an edit that rewrote the body AND its `# body_sha256`
    // line is self-consistent and is caught only here.
    assert_eq!(
        sha256_hex(committed_text().as_bytes()),
        RANDOM_OPENINGS_V1_SHA256,
        "the book's bytes are pinned in this file (RANDOM_OPENINGS_V1_SHA256)"
    );
}

#[test]
fn random_openings_v1_body_digest_describes_its_own_body() {
    // The in-band digest D-148 exists for: a consumer holding only the file can
    // refuse a corrupted one without carrying a constant from someone else's
    // test tree. The arena's openings reader verifies exactly this.
    let text = committed_text();
    let body = body_of(&text).expect("a rendered fixture has a body");
    assert_eq!(
        claimed_body_digest(&text).expect("the header claims a body digest"),
        sha256_hex(body.as_bytes()),
        "the header's body digest is the digest of the body under it"
    );
}

#[test]
fn random_openings_v1_states_every_parameter_it_was_made_with() {
    // A fixture that does not say what produced it is not reproducible from
    // itself (docs/decisions.md D-147). The seed is in here because it is the
    // only reason these particular positions and not others.
    let text = committed_text();
    let header = text
        .split_once(BODY_DIGEST)
        .expect("the header ends at the body digest")
        .0
        .to_string();
    let config = RandomOpeningsConfig::load(&repo("configs/random_openings_v1.toml"))
        .expect("the committed config loads");
    for expected in [
        format!("# param k_stones {}", config.generate.k_stones),
        format!("# param n_openings {}", config.generate.n_openings),
        format!("# param max_radius {}", config.generate.max_radius),
        format!("# param seed {}", config.generate.seed),
    ] {
        assert!(
            header.lines().any(|line| line == expected),
            "the header states `{expected}`"
        );
    }
    assert!(
        header.contains("LEGAL_RADIUS"),
        "the header says in words that max_radius is not the legal radius"
    );
}

#[test]
fn random_openings_v1_holds_the_number_of_openings_it_was_asked_for() {
    let config = RandomOpeningsConfig::load(&repo("configs/random_openings_v1.toml"))
        .expect("the committed config loads");
    let text = committed_text();
    let lines = body_of(&text)
        .expect("a body")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .count();
    assert_eq!(
        lines, config.generate.n_openings,
        "one payload line per opening, and no cap"
    );
}

#[test]
fn random_openings_v1_ends_with_exactly_one_newline() {
    // Part of D-147's serialization contract, and the one part a digest cannot
    // tell you about after the fact.
    let text = committed_text();
    assert!(text.ends_with('\n'), "the file ends with a newline");
    assert!(!text.ends_with("\n\n"), "and with only one");
}
