mod common;

use common::{Ran, Scratch, openings_prefix, run, self_match};
use pistol_arena::report::verdict_block;

/// Openings per run, and a cap that leaves room for a real game.
const OPENINGS: usize = 4;
const TURN_CAP: u32 = 10;

/// A self-match at one worker, which most tests start from.
fn honest(scratch: &Scratch, tag: &str, workers: usize) -> Ran {
    let openings = scratch.write("openings.txt", &openings_prefix(OPENINGS));
    let stub = scratch.stub_config("honest.toml", "honest");
    let spec = self_match(&openings, &stub, OPENINGS, TURN_CAP, workers);
    run(scratch, &spec, tag)
}

#[test]
fn arena_self_match_smoke() {
    let scratch = Scratch::new("smoke");
    let ran = honest(&scratch, "smoke", 1);
    assert_eq!(
        ran.code(),
        0,
        "a clean self-match exits zero:\n{}",
        ran.report()
    );

    assert_eq!(
        ran.games().len(),
        OPENINGS * 2,
        "every opening played from both seats"
    );
    // Two identical deterministic engines play the same game whichever seat
    // they sit in, so both games of every pair are the same game. The exact
    // number is asserted rather than "near-total dedupe", which is
    // unfalsifiable (CLAUDE.md rule 6).
    let counts = ran.field("counts");
    assert!(
        counts.contains(&format!("n {} distinct_n {}", OPENINGS * 2, OPENINGS)),
        "a self-match halves exactly: {counts}"
    );
    assert!(
        counts.contains("forfeits 0"),
        "nothing was forfeited: {counts}"
    );
    // Every pair is 1-1, so no likelihood ratio exists — which is the right
    // answer for two identical configurations and not an error.
    assert_eq!(ran.field("verdict"), "inconclusive_degenerate");
    assert_eq!(ran.field("verdict_unit"), "pair");
    let pentanomial = ran.field("pentanomial");
    assert!(
        pentanomial.contains(&format!("p2 {OPENINGS}")),
        "every pair scored one point: {pentanomial}"
    );
}

#[test]
fn paired_openings_play_both_sides() {
    let scratch = Scratch::new("paired");
    let ran = honest(&scratch, "paired", 1);
    let games = ran.games();
    for opening in 0..OPENINGS {
        let first = games[opening * 2];
        let second = games[opening * 2 + 1];
        assert!(
            first.contains(&format!("opening {opening} p1 a p2 b")),
            "the even game seats engine a first: {first}"
        );
        assert!(
            second.contains(&format!("opening {opening} p1 b p2 a")),
            "and the odd game swaps them: {second}"
        );
    }
    // Both games of a pair start from the same position, so their move lists
    // share the opening prefix.
    let moves: Vec<&str> = ran
        .report()
        .lines()
        .filter(|line| line.starts_with("moves "))
        .collect();
    for opening in 0..OPENINGS {
        let a: Vec<&str> = moves[opening * 2]
            .split_whitespace()
            .skip(2)
            .take(4)
            .collect();
        let b: Vec<&str> = moves[opening * 2 + 1]
            .split_whitespace()
            .skip(2)
            .take(4)
            .collect();
        assert_eq!(a, b, "the two games of a pair start from the same opening");
    }
}

#[test]
fn report_contains_per_side_compute_fields() {
    let scratch = Scratch::new("compute");
    let ran = honest(&scratch, "compute", 1);
    for game in ran.games() {
        for field in ["nodes_a", "nodes_b", "depth_a", "depth_b"] {
            let value: u64 = value_of(game, field);
            assert!(
                value > 0,
                "{field} must be recorded, not merely present: {game}"
            );
        }
    }
    // Time is per side too, and it lives in the TIMING block because it is not
    // worker-invariant (docs/decisions.md D-161).
    let report = ran.report();
    for slot in ["a", "b"] {
        assert!(
            report.contains(&format!("timing_engine {slot} time_ms ")),
            "per-side time is reported"
        );
    }
    assert!(
        !verdict_block(report).contains("time_ms"),
        "and it is NOT in the block two worker counts must agree on"
    );
    assert!(
        verdict_block(report).contains("nodes_a"),
        "while nodes are, because they are reproducible"
    );
}

#[test]
fn two_worker_run_report_identical_to_single_worker() {
    let scratch = Scratch::new("workers");
    let one = honest(&scratch, "w1", 1);
    let two = honest(&scratch, "w2", 2);

    // Non-vacuous first: two aborted runs would both write nothing and a naive
    // comparison would pass while asserting anything at all.
    for ran in [&one, &two] {
        assert_eq!(ran.games().len(), OPENINGS * 2, "the run actually played");
        assert!(ran.field("pentanomial").contains("p2"), "and scored");
    }

    assert_eq!(
        verdict_block(one.report()),
        verdict_block(two.report()),
        "one worker and two must agree on every number that decides the verdict"
    );
    // And the timing blocks DIFFER in the worker count, so a report that simply
    // omitted the field could not pass the assertion above.
    assert!(one.report().contains("n_workers 1"));
    assert!(two.report().contains("n_workers 2"));
}

/// The numeric value following `field` in a whitespace record.
fn value_of(record: &str, field: &str) -> u64 {
    let words: Vec<&str> = record.split_whitespace().collect();
    let at = words
        .iter()
        .position(|word| *word == field)
        .unwrap_or_else(|| panic!("no `{field}` in `{record}`"));
    words[at + 1]
        .parse()
        .unwrap_or_else(|_| panic!("`{field}` carries a number in `{record}`"))
}
