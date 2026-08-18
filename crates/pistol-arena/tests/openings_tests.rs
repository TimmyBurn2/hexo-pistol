//! The openings fixture is verified against its own in-band digest, and
//! anything that is not a fixture is refused by name.
//!
//! docs/decisions.md D-148 pre-registered this crate as the consumer that would
//! either use the body digest or retire it as dead weight. These tests are the
//! use.

mod common;

use common::{Scratch, committed_body, openings_fixture, openings_prefix, repo};
use pistol_arena::openings;

/// A cap comfortably clear of the fixture's four-turn openings.
const CAP: u32 = 12;

#[test]
fn arena_loads_primary_book_with_digest() {
    // The one thing neither crate could assert on its own. `random_openings_v1.txt`
    // is written by pistol-cli and read here, and every property the reader
    // requires was asserted only on the WRITER's side — the header and body
    // offset, a payload of position tails with no stray `#` line, one turn count
    // for every opening, and no two openings equal up to a lattice symmetry.
    // That is not the same as this reader having read it, and D-175 makes this
    // file the book every SPRT runs over, so the day the two serializations
    // drift apart must be a red test rather than a failed experiment
    // (docs/decisions.md D-182, which this closes).
    //
    // The digest is checked by value as well as by loading. `load` verifies the
    // body against the header's own claim and refuses a mismatch, so a load that
    // succeeded already proves them consistent; what the constant adds is that
    // the bytes are the ones this test was written against, so a regenerated
    // book with a different seed cannot slip through as "still loads".
    let path = repo().join("crates/pistol-cli/tests/fixtures/random_openings_v1.txt");
    let loaded = openings::load(&path, 500, CAP).expect("the primary book loads");
    assert_eq!(
        loaded.total, 500,
        "the whole book (docs/decisions.md D-175)"
    );
    assert_eq!(loaded.taken.len(), 500, "and all of it was taken");
    assert_eq!(
        loaded.body_sha256, "f0bf76c5f53ae192d970a32f8127f3aae1910e5a8d4fb4374238e4450c6a152e",
        "the in-band digest pistol-cli wrote, read back here"
    );
    assert_eq!(
        loaded.opening_turns, 3,
        "five stones is three turns: one on turn 1, two thereafter (game rule 3)"
    );
}

#[test]
fn a_correct_fixture_loads_and_reports_what_it_holds() {
    let scratch = Scratch::new("openings-good");
    let path = scratch.write("openings.txt", &openings_prefix(6));
    let loaded = openings::load(&path, 4, CAP).expect("a correct fixture loads");
    assert_eq!(loaded.taken.len(), 4, "the prefix asked for");
    assert_eq!(
        loaded.total, 6,
        "and the file's own size, so a run can say what it sampled from"
    );
    assert_eq!(
        loaded.opening_turns, 4,
        "openings are four turns (docs/decisions.md D-138)"
    );
    assert_eq!(loaded.taken[0].index, 0);
    assert!(loaded.taken[0].position_tail.starts_with("start moves "));
    assert!(
        !loaded.taken[0].position_tail.contains('#'),
        "the commentary is stripped before the line goes down a pipe"
    );
}

#[test]
fn a_body_that_does_not_match_its_digest_is_refused() {
    let scratch = Scratch::new("openings-digest");
    let good = openings_prefix(4);
    // Change one coordinate in the body, leaving the header's claim alone.
    let tampered = good.replacen("start moves 0,0", "start moves 0,0 ", 1);
    assert_ne!(tampered, good, "the tamper actually changed the bytes");
    let path = scratch.write("openings.txt", &tampered);
    let error = openings::load(&path, 4, CAP).expect_err("a tampered body is refused");
    assert_eq!(error.name(), "OpeningsDigest");
}

#[test]
fn a_file_with_no_digest_line_is_refused() {
    // A missing pin and a satisfied pin must not look alike (D-147's rule for a
    // missing section versus an empty one).
    let scratch = Scratch::new("openings-nopin");
    let body = committed_body();
    let unpinned = format!("# no pin here\n{}\n", body[0]);
    let path = scratch.write("openings.txt", &unpinned);
    let error = openings::load(&path, 1, CAP).expect_err("an unpinned file is refused");
    assert_eq!(error.name(), "Openings");
    assert!(error.to_string().contains("body_sha256"));
}

#[test]
fn arena_refuses_duplicate_opening_up_to_symmetry() {
    let scratch = Scratch::new("openings-dup");
    let body = committed_body();

    // Byte-identical first.
    let repeated = openings_fixture(&[body[0].clone(), body[1].clone(), body[0].clone()]);
    let path = scratch.write("repeated.txt", &repeated);
    let error = openings::load(&path, 3, CAP).expect_err("a repeated opening is refused");
    assert_eq!(error.name(), "Openings");
    assert!(
        error.to_string().contains("symmetry"),
        "the refusal says what makes them the same opening: {error}"
    );

    // And a MIRROR, which a move-list comparison would not catch. The q<->r
    // transposition is one of the twelve lattice automorphisms
    // (docs/decisions.md D-140), so swapping every coordinate's components
    // produces the same opening in a different spelling.
    let mirrored = mirror(&body[0]);
    assert_ne!(mirrored, body[0], "the mirror is a different spelling");
    let both = openings_fixture(&[body[0].clone(), mirrored]);
    let path = scratch.write("mirrored.txt", &both);
    let error = openings::load(&path, 2, CAP)
        .expect_err("a mirrored opening is the same opening (docs/decisions.md D-137)");
    assert_eq!(error.name(), "Openings");
}

/// One opening under the `q <-> r` transposition.
fn mirror(line: &str) -> String {
    let tail = match line.find(" #") {
        Some(at) => &line[..at],
        None => line,
    };
    let mut out = String::from("start moves");
    for token in tail.trim_start_matches("start moves").split_whitespace() {
        out.push(' ');
        let swapped: Vec<String> = token
            .split('/')
            .map(|stone| {
                let (q, r) = stone.split_once(',').expect("a stone token");
                format!("{r},{q}")
            })
            .collect();
        // A transformed pair must be re-canonicalized; the protocol pins the
        // pair token to `(q, r)` order (docs/decisions.md D-5).
        let mut cells = swapped;
        cells.sort_by_key(|cell| {
            let (q, r) = cell.split_once(',').expect("a stone token");
            (q.parse::<i32>().expect("q"), r.parse::<i32>().expect("r"))
        });
        out.push_str(&cells.join("/"));
    }
    out
}

#[test]
fn a_blank_or_commented_line_in_the_body_is_refused() {
    let scratch = Scratch::new("openings-body");
    let body = committed_body();
    for (name, bad) in [
        ("blank", String::new()),
        ("comment", String::from("# not out here")),
    ] {
        let text = openings_fixture(&[body[0].clone(), bad, body[1].clone()]);
        let path = scratch.write(&format!("{name}.txt"), &text);
        let Err(error) = openings::load(&path, 3, CAP) else {
            panic!("a {name} line in the body must be refused, not skipped");
        };
        assert_eq!(error.name(), "Openings", "a {name} line is a named refusal");
    }
}

#[test]
fn a_file_that_mixes_opening_lengths_is_refused() {
    let scratch = Scratch::new("openings-length");
    let body = committed_body();
    // A three-turn prefix of the first opening: legal, but a different length.
    let short: String = body[0]
        .split_whitespace()
        .take(5)
        .collect::<Vec<&str>>()
        .join(" ");
    let text = openings_fixture(&[body[0].clone(), short]);
    let path = scratch.write("mixed.txt", &text);
    let error = openings::load(&path, 2, CAP).expect_err("mixed lengths are refused");
    assert!(
        error.to_string().contains("horizon"),
        "the refusal says why it matters — one turn cap cannot mean two horizons: {error}"
    );
}

#[test]
fn taking_more_openings_than_the_file_holds_is_refused() {
    let scratch = Scratch::new("openings-take");
    let path = scratch.write("openings.txt", &openings_prefix(3));
    let error = openings::load(&path, 4, CAP)
        .expect_err("silently taking fewer would be a different experiment");
    assert_eq!(error.name(), "Config");
    assert!(error.to_string().contains("run.openings_take"));
}

#[test]
fn a_turn_cap_that_does_not_clear_the_opening_is_refused() {
    // With four-turn openings and a cap of four, every game would be capped
    // before either engine moved — and the run would still print a verdict.
    let scratch = Scratch::new("openings-cap");
    let path = scratch.write("openings.txt", &openings_prefix(2));
    let error = openings::load(&path, 2, 4).expect_err("a cap inside the opening is refused");
    assert_eq!(error.name(), "Config");
    assert!(error.to_string().contains("run.turn_cap"));
    // And one turn past it is accepted, so the bound is the stated one.
    openings::load(&path, 2, 5).expect("a cap of one turn past the opening is a real game");
}
