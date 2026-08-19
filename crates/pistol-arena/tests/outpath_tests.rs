//! The report path is claimed atomically, so two racing runs cannot both
//! believe they own it (wp13_results §6b's TOCTOU, closed by D-200).

mod common;

use common::Scratch;
use pistol_arena::outpath;

#[test]
fn out_path_collision_fails_by_name() {
    // The unit under test is the claim itself, because a binary-level test
    // cannot see the race: the old code also refused a PRE-EXISTING file, so
    // only the claim's atomicity — create_new succeeding exactly once —
    // distinguishes the fix from the TOCTOU it kills.
    let scratch = Scratch::new("out-claim");
    let path = scratch.path("report.matchlog");

    let first = outpath::claim(&path).expect("the first claim takes the path");
    let second = outpath::claim(&path).expect_err("a second claim on the same path must fail");
    let message = second.to_string();
    assert!(
        message.contains("report.matchlog"),
        "the refusal names the path: {message}"
    );
    assert!(
        message.contains("does not overwrite"),
        "the refusal names the rule: {message}"
    );

    // A released claim frees the path — the refusal path (exit 2, "no report
    // at all") removes the empty claim it created and nothing else.
    drop(first);
    outpath::abandon(&path).expect("the claimant may release its own claim");
    assert!(!path.exists(), "an abandoned claim leaves no file behind");
    let _ = outpath::claim(&path).expect("a released path can be claimed again");
}
