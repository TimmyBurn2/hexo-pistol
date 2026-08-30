//! The power instrument's own tests.
//!
//! `crates/pistol-arena/examples/sprt_power.rs` produces a REGISTERED number —
//! `book_v2`'s size (docs/experiments/book_v2_registration.md §4) — so it is
//! reviewed and tested as an instrument rather than as a script. What these
//! pin is the arithmetic the example carries that this crate's library does
//! not: the exponential tilt, its bisection, and the refusals.
//!
//! The example's OTHER half — the sequential test itself — is this crate's own
//! `Sample`/`crossing` and is pinned by `sprt_tests.rs`; an instrument that
//! re-implemented it would be the thing this file exists to make impossible.

use std::path::PathBuf;
use std::process::Command;

use pistol_arena::sprt::{PAIR_SCORES, Unit};

/// The example binary, beside this test's own.
///
/// `CARGO_BIN_EXE_` exists for `[[bin]]` targets and this is an EXAMPLE — which
/// is the right target kind for it, because an instrument that shipped in
/// `src/bin/` would be a program this project distributes. `cargo test` builds
/// examples, so the path below exists whenever this test can run; if it does
/// not, the test says which command makes it rather than skipping (rule 3).
fn instrument() -> PathBuf {
    let mut path = std::env::current_exe().expect("the test binary knows its own path");
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.push("examples");
    path.push("sprt_power");
    assert!(
        path.exists(),
        "no example binary at {}: build it with `cargo build -p pistol-arena --example sprt_power`",
        path.display()
    );
    path
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(instrument())
        .args(args)
        .output()
        .expect("the example runs")
}

fn field(text: &str, key: &str) -> f64 {
    let words: Vec<&str> = text.split_whitespace().collect();
    let at = words
        .iter()
        .position(|word| *word == key)
        .unwrap_or_else(|| panic!("`{key}` is printed: {text}"));
    words[at + 1]
        .trim_end_matches(')')
        .trim_start_matches('(')
        .parse()
        .unwrap_or_else(|why| panic!("`{key}` reads as a number: {why}"))
}

/// `t` of a bucket distribution, computed here so the example's own `t_of` is
/// checked against something and not against itself.
fn t_of(q: [f64; 5]) -> f64 {
    let mu: f64 = (0..5).map(|i| q[i] * PAIR_SCORES[i]).sum();
    let second: f64 = (0..5).map(|i| q[i] * PAIR_SCORES[i] * PAIR_SCORES[i]).sum();
    (mu - 0.5) / (second - mu * mu).sqrt()
}

#[test]
fn the_tilt_lands_on_the_effect_it_was_asked_for() {
    // The bisection's whole job. A solver that reported success on a bracket it
    // never entered would print a `tilted_t` that is not the `target`, and a
    // target scaled in the wrong unit would miss by the sqrt(2) `Unit::Pair`
    // carries.
    for truth in ["-25", "0", "15", "25"] {
        let output = run(&["--pairs", "10", "--runs", "1", "--truth", truth]);
        assert!(output.status.success(), "truth {truth} runs");
        let text = String::from_utf8_lossy(&output.stdout);
        let tilted = field(&text, "tilted_t");
        let target = field(&text, "(target");
        assert!(
            (tilted - target).abs() < 1e-6,
            "the tilt lands on its target at truth {truth}: {tilted} vs {target}"
        );
        // And the target is the one this crate's own unit conversion gives.
        let expected = Unit::Pair.t(truth.parse().expect("a number"));
        // 1e-6 and not tighter: the instrument prints six decimals, so this
        // compares what a reader of the artifact can compare.
        assert!(
            (target - expected).abs() < 1e-6,
            "the target is `Unit::Pair.t(truth)` at truth {truth}: {target} vs {expected}"
        );
    }
}

#[test]
fn the_tilted_distribution_is_a_distribution_and_carries_the_effect() {
    // Recomputed here from the printed theta over the DEFAULT bucket shape, so
    // a tilt that renormalised wrongly — or one that tilted by score index
    // rather than by score — is caught by arithmetic this file owns.
    let output = run(&["--pairs", "10", "--runs", "1", "--truth", "15"]);
    let text = String::from_utf8_lossy(&output.stdout);
    let theta = field(&text, "theta");
    let base = [30.0, 75.0, 277.0, 68.0, 50.0];
    let total: f64 = base.iter().sum();
    let mut q = [0.0; 5];
    let mut weight = 0.0;
    for index in 0..5 {
        q[index] = (base[index] / total) * (theta * PAIR_SCORES[index]).exp();
        weight += q[index];
    }
    for slot in &mut q {
        *slot /= weight;
    }
    assert!(
        (q.iter().sum::<f64>() - 1.0).abs() < 1e-12,
        "it is a distribution"
    );
    assert!(
        (t_of(q) - Unit::Pair.t(15.0)).abs() < 1e-6,
        "the printed theta reproduces the asked-for effect: {} vs {}",
        t_of(q),
        Unit::Pair.t(15.0)
    );
}

#[test]
fn a_target_the_bucket_shape_cannot_reach_is_refused_by_name() {
    // The clamp that must not exist. A bisection that answered its bracket end
    // when the target is outside would report a power for an effect it never
    // simulated (CLAUDE.md rule 3).
    let output = run(&["--truth", "1000000", "--runs", "10"]);
    assert_eq!(output.status.code(), Some(1));
    let text = String::from_utf8_lossy(&output.stderr);
    assert!(
        text.contains("outside what this bucket shape reaches"),
        "the refusal names what is wrong: {text}"
    );
}

#[test]
fn a_malformed_argument_is_refused_by_name_and_never_defaulted() {
    for (args, wanted) in [
        (
            vec!["--buckets", "0,0,0,0,0"],
            "non-negative and not all zero",
        ),
        (vec!["--buckets", "1,2,3"], "five comma-separated counts"),
        (vec!["--runs", "0"], "must be positive"),
        (vec!["--pairs", "0"], "must be positive"),
        (vec!["--nonsense", "1"], "unknown option"),
        (vec!["--pairs"], "wants a value"),
    ] {
        let output = run(&args);
        assert_eq!(output.status.code(), Some(1), "{args:?} is refused");
        let text = String::from_utf8_lossy(&output.stderr);
        assert!(
            text.contains(wanted),
            "{args:?} is refused by name, wanted `{wanted}`, got: {text}"
        );
    }
}

#[test]
fn the_same_argv_twice_answers_the_same_bytes() {
    // A registered figure a successor cannot re-check is a figure nobody can
    // check (CLAUDE.md rule 4).
    let args = [
        "--pairs", "500", "--runs", "2000", "--truth", "15", "--seed", "7",
    ];
    let first = run(&args);
    let second = run(&args);
    assert_eq!(first.stdout, second.stdout);
}

#[test]
fn a_bigger_pair_cap_never_lowers_the_power_it_measures() {
    // The monotonicity the size rule leans on: §4 reads `P` off a grid as the
    // SMALLEST cap reaching the threshold, which is only a well-posed reading
    // if power rises with the cap. Two points far apart, so this pins the shape
    // and not the noise.
    let small = run(&[
        "--pairs", "500", "--runs", "4000", "--truth", "15", "--seed", "3",
    ]);
    let large = run(&[
        "--pairs", "4000", "--runs", "4000", "--truth", "15", "--seed", "3",
    ]);
    // The `h1` fraction, which the line prints parenthesised after the count.
    let power = |output: &std::process::Output| {
        let text = String::from_utf8_lossy(&output.stdout).to_string();
        let line = text
            .lines()
            .find(|line| line.contains("h1 "))
            .unwrap_or_else(|| panic!("an h1 line: {text}"))
            .to_string();
        let after = line
            .split_once('(')
            .expect("the fraction is parenthesised")
            .1;
        after
            .split_once(')')
            .expect("and closed")
            .0
            .parse::<f64>()
            .expect("and is a number")
    };
    assert!(
        power(&large) > power(&small),
        "power rises with the cap: {} at 500, {} at 4000",
        power(&small),
        power(&large)
    );
}
