//! The sequential stop, and worker invariance on a sample that can actually
//! cross a boundary.
//!
//! Every other end-to-end test in this crate runs a SELF-MATCH, where both
//! games of a pair are the same game and every pair therefore scores alike.
//! That is the right shape for the smoke gate — its answer is knowable in
//! advance — but it means no sample in those tests can ever cross, so the stop
//! is never exercised and both worker-invariance checks would pass unchanged if
//! the stop were deleted outright. REVIEW-impl found that gap; this file is it
//! closed.
//!
//! The two engines are the honest stub and its mirror image, which grows its
//! cluster from the largest cells instead of the smallest. They are both
//! perfectly well-behaved and they win different openings, which is the only
//! way to get pair-to-pair variation out of two deterministic players.
//!
//! `elo1` is deliberately wide. The stop mechanism is what is under test, not a
//! particular effect size, and a bound reachable in a couple of hundred pairs
//! keeps this test in seconds rather than minutes.

mod common;

use common::{Scratch, openings_prefix, run, self_match};

/// Enough openings that the bound is reachable, and a cap that lets games end.
const OPENINGS: usize = 300;
const TURN_CAP: u32 = 14;
/// A wide alternative, so the crossing arrives at a testable size.
const ELO1: f64 = 30.0;

#[test]
fn the_run_stops_at_a_pair_boundary_when_the_llr_crosses() {
    let scratch = Scratch::new("stop");
    let openings = scratch.write("openings.txt", &openings_prefix(OPENINGS));
    let first = scratch.stub_config("first.toml", "honest");
    let last = scratch.stub_config("last.toml", "honest_last");
    let mut spec = self_match(&openings, &first, OPENINGS, TURN_CAP, 4);
    spec.config_b = &last;
    spec.elo1 = ELO1;
    let ran = run(&scratch, &spec, "stop");

    let played = ran.games().len();
    assert!(
        played < OPENINGS * 2,
        "the stop must have fired: {played} games played of {} available, so nothing was cut \
         short and this test proves nothing about stopping",
        OPENINGS * 2
    );
    assert!(
        played.is_multiple_of(2),
        "the stop fires only at a pair boundary, and {played} is not a whole number of pairs"
    );
    // The pentanomial covers every kept game exactly, which is the property an
    // odd truncation would break.
    let pentanomial: usize = ran
        .field("pentanomial")
        .split_whitespace()
        .skip(1)
        .step_by(2)
        .map(|count| count.parse::<usize>().expect("a count"))
        .sum();
    assert_eq!(
        pentanomial,
        played / 2,
        "every kept game is inside a scored pair"
    );
    assert_eq!(ran.field("verdict"), "h1", "and the run concluded");
    assert_eq!(ran.code(), 0, "a clean conclusive run exits zero");

    // Two different engines produce different games, so this run also exercises
    // the dedupe in the direction the self-match cannot: nothing to merge.
    let counts = ran.field("counts");
    assert!(
        counts.contains(&format!("n {played} distinct_n {played}")),
        "two different engines play {played} different games: {counts}"
    );
}

#[test]
fn the_stop_lands_in_the_same_place_at_any_worker_count() {
    // The strongest form of the invariance claim: the same crossing, the same
    // truncation point and the same verdict block on a sample that actually
    // decides — where the self-match version of this test can only compare two
    // runs that both went the distance (docs/decisions.md D-161, D-165).
    let scratch = Scratch::new("stop-workers");
    let openings = scratch.write("openings.txt", &openings_prefix(OPENINGS));
    let first = scratch.stub_config("first.toml", "honest");
    let last = scratch.stub_config("last.toml", "honest_last");

    let mut spec = self_match(&openings, &first, OPENINGS, TURN_CAP, 1);
    spec.config_b = &last;
    spec.elo1 = ELO1;
    let one = run(&scratch, &spec, "stop-w1");
    spec.workers = 4;
    let four = run(&scratch, &spec, "stop-w4");

    for ran in [&one, &four] {
        assert_eq!(ran.field("verdict"), "h1", "both runs concluded");
        assert!(!ran.games().is_empty(), "and both actually played");
    }
    assert_eq!(
        one.games().len(),
        four.games().len(),
        "the truncation point is a function of the results, not of how many games were in \
         flight when the bound was crossed"
    );
    assert_eq!(
        pistol_arena::report::verdict_block(one.report()),
        pistol_arena::report::verdict_block(four.report()),
        "one worker and four agree on every number that decides the verdict"
    );
}
