//! This suite's scratch directories are swept by the OTHER suite's sweeper
//! (`crates/pistol-cli/tests/common/mod.rs`), because a `Drop` guard does not
//! run when a test binary aborts or is killed. Two crates therefore have to
//! agree on one prefix, and an agreement stated only in a comment is one that
//! breaks silently — this is the arena's half of it (docs/decisions.md D-239).

mod common;

use common::{SCRATCH_PREFIX, Scratch};

/// The prefix the cli suite's sweep matches, restated rather than imported: the
/// two crates are what must agree, and importing the constant from the other
/// side would make them agree by construction and prove nothing.
const SWEPT_PREFIX: &str = "pistol-testscratch-";

#[test]
fn every_arena_scratch_directory_is_one_the_cli_suites_sweep_will_recognize() {
    let scratch = Scratch::new("scratch-naming");
    let name = scratch
        .dir
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .expect("a scratch directory has a utf-8 name");
    assert!(
        name.starts_with(SWEPT_PREFIX),
        "a scratch directory outside the swept prefix is one nothing removes when \
         this binary dies without running a destructor: `{name}`"
    );
    assert!(
        SCRATCH_PREFIX.starts_with(SWEPT_PREFIX),
        "and the constant this suite names its directories with is inside it: \
         `{SCRATCH_PREFIX}`"
    );
    // It is NOT the workspace's own naming scheme, which is what the sweep used
    // to match: `pistol-arena` is a crate directory, and a sweep by that prefix
    // removed crate directories out of a `TMPDIR` near a checkout.
    assert!(
        !"pistol-arena".starts_with(SWEPT_PREFIX),
        "the swept prefix may not be one a crate directory carries"
    );
}
