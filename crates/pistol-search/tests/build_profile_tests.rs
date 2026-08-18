//! What the build profiles keep, asserted by a build rather than by reading
//! `Cargo.toml`.
//!
//! A profile flag is exactly the kind of setting that reverts silently: nothing
//! fails when `overflow-checks` goes missing from `[profile.release]`, every
//! test still passes, and the first thing anybody notices is a wrong answer the
//! engine played. So the flag gets a test, and the test has to run in the
//! profile it is about — a debug build has overflow checks by default and would
//! pass this without saying anything (docs/decisions.md D-127).
//!
//! # Why this file is in pistol-search
//!
//! It is about the workspace and not about the search. It lives here because
//! `tools/search_oracle_check.sh` owns the only release `cargo test` in the gate
//! set, so this is where a release-profile assertion can be run without adding a
//! gate for one test; that script names the ride-along where it runs it.

use std::hint::black_box;
use std::panic;

/// Bare integer arithmetic that overflows panics in whatever profile this test
/// was built for.
///
/// In a debug build that is the language default and this asserts nothing. In
/// release it is `[profile.release] overflow-checks = true` and nothing else —
/// remove that line and this goes red, which is the whole point of it
/// (docs/decisions.md D-127).
///
/// `black_box` is load-bearing twice over: it keeps the addition out of const
/// evaluation, where an overflow is a compile error rather than a panic, and it
/// keeps the optimizer from folding away a computation whose result is unused.
#[test]
fn bare_integer_overflow_panics_under_this_profile() {
    let hushed = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let outcome = panic::catch_unwind(|| black_box(black_box(i32::MAX) + black_box(1)));
    panic::set_hook(hushed);

    assert!(
        outcome.is_err(),
        "`i32::MAX + 1` wrapped instead of panicking, so this build has overflow checks off: in \
         release that is `[profile.release] overflow-checks = true` gone missing \
         (docs/decisions.md D-127)"
    );
}
