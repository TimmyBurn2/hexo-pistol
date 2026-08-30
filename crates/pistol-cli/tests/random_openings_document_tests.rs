mod common;

use common::repo;
use pistol_cli::corpus::emit::{BODY_DIGEST, body_of, claimed_body_digest};
use pistol_cli::random_openings::config::RandomOpeningsConfig;
use pistol_cli::random_openings::{self, BookVersion, FILE_NAME, document};
use pistol_cli::sha256::sha256_hex;
use pistol_core::{Coord, Player, canonical_form};
use pistol_engine::PositionSpec;
use std::path::PathBuf;

/// The SHA-256 of the committed `fixtures/random_openings_v1.txt`.
const RANDOM_OPENINGS_V1_SHA256: &str =
    "895a05edb53a0a8d89c262bb058e3bc3dd24d446405d375458aaf067e2f076e7";

/// The SHA-256 of the committed `fixtures/random_openings_v2.txt`.
const RANDOM_OPENINGS_V2_SHA256: &str =
    "829361a9ae61d0d4369b5291bfc893133fa8160867f11cc638b11f432b6cc29a";

fn book_path() -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(FILE_NAME)
}

fn committed_text() -> String {
    std::fs::read_to_string(book_path()).expect("the book is committed")
}

/// The book this build produces from the committed config.
fn produced() -> String {
    produced_from("configs/random_openings_v1.toml")
}

fn produced_from(config_path: &str) -> String {
    let config = RandomOpeningsConfig::load(&repo(config_path)).expect("the config loads");
    let book = random_openings::generate(&config).expect("the config generates");
    document::render(&config, &book)
}

fn v2_path() -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(BookVersion::V2.file_name())
}

fn v2_committed_text() -> String {
    std::fs::read_to_string(v2_path()).expect("the v2 book is committed")
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

// --- v2, the successor book -------------------------------------------------
//
// The v1 tests above go on reading `FILE_NAME` and go on pinning v1: that book
// is retired for governed use but is the artifact every closed SPRT verdict was
// taken over, and a test that stopped watching it would be the first step to
// losing it (docs/decisions.md D-505).

#[test]
fn random_openings_v2_is_what_this_build_produces() {
    assert_eq!(
        produced_from("configs/random_openings_v2.toml"),
        v2_committed_text(),
        "regenerate with `cargo run -p pistol-cli --bin random-openings -- \
         --config configs/random_openings_v2.toml \
         --out-dir crates/pistol-cli/tests/fixtures`"
    );
}

#[test]
fn random_openings_v2_matches_its_pinned_digest() {
    assert_eq!(
        sha256_hex(v2_committed_text().as_bytes()),
        RANDOM_OPENINGS_V2_SHA256,
        "the book's bytes are pinned in this file (RANDOM_OPENINGS_V2_SHA256)"
    );
}

#[test]
fn random_openings_v2_body_digest_describes_its_own_body() {
    let text = v2_committed_text();
    let body = body_of(&text).expect("a rendered fixture has a body");
    assert_eq!(
        claimed_body_digest(&text).expect("the header claims a body digest"),
        sha256_hex(body.as_bytes()),
        "the header's body digest is the digest of the body under it"
    );
}

#[test]
fn random_openings_v2_holds_the_number_of_openings_it_was_asked_for() {
    let config = RandomOpeningsConfig::load(&repo("configs/random_openings_v2.toml"))
        .expect("the committed config loads");
    let lines = body_of(&v2_committed_text())
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
fn the_two_books_overlap_only_as_far_as_independent_drawing_makes_them() {
    // FRESHNESS IS ABOUT SLICES, AND AT THAT LEVEL IT IS ABSOLUTE (D-505): the
    // two books are different files drawn from different seeds, so no range of
    // v2 is a range of v1 and no governed run can re-read a consumed sample.
    //
    // At the level of an individual POSITION it is not absolute and cannot be,
    // because both books draw independently from one finite pool. The pool is
    // countable: the origin is fixed, P2 takes an unordered pair of the
    // remaining ninety cells and P1 an unordered pair of the eighty-eight left,
    // so there are C(90,2) x C(88,2) = 15,331,140 distinct assignments — about
    // 1.28 million once the twelve lattice symmetries are folded, which is what
    // each book dedupes by. Drawing 2000 and 4500 from those gives an EXPECTED
    // overlap near 0.6 exact lines and near 7 canonical forms.
    //
    // Both counts are pinned exactly rather than bounded, because a pin fails on
    // any change and a bound quietly absorbs one. If either moves, a seed or a
    // size moved with it and this test is the place that says so.
    let v1_text = committed_text();
    let v2_text = v2_committed_text();
    assert_eq!(
        body_of_lines(&v1_text)
            .intersection(&body_of_lines(&v2_text))
            .count(),
        1,
        "exact shared lines"
    );
    assert_eq!(
        canonical_forms(&v1_text)
            .intersection(&canonical_forms(&v2_text))
            .count(),
        10,
        "shared positions up to the twelve lattice symmetries"
    );
}

/// The payload lines of a rendered fixture, as a set.
fn body_of_lines(text: &str) -> std::collections::BTreeSet<&str> {
    body_of(text)
        .expect("a body")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect()
}

/// The same lines as POSITIONS, folded by the lattice symmetries the books
/// dedupe by — which is the set a "has this been played before" question is
/// really asked over.
fn canonical_forms(text: &str) -> std::collections::BTreeSet<Vec<(Coord, Player)>> {
    body_of_lines(text)
        .into_iter()
        .map(|line| {
            let spec: PositionSpec = line.parse().expect("a payload line is a position");
            let state = spec.replay().expect("and it replays");
            canonical_form(&state.played().collect::<Vec<_>>())
        })
        .collect()
}

#[test]
fn the_two_books_are_written_to_different_files() {
    // The compile-time constant that made a naive regeneration overwrite v1
    // (the hazard the Stage-3 premise closure recorded) is now a closed set,
    // and this is the test that says the set has two distinct members.
    assert_ne!(BookVersion::V1.file_name(), BookVersion::V2.file_name());
    assert_eq!(BookVersion::V1.file_name(), FILE_NAME);
}
