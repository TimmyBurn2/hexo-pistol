mod common;

use std::time::Duration;

use common::{SCRATCH_PREFIX, scratch, sweep_scratch_in};

/// Every crate directory in this workspace, spelled out rather than derived: the
/// point is that the names a `TMPDIR` may hold are the names the sweep must
/// spare, and deriving them from the same string the sweep matches would test
/// nothing.
const WORKSPACE_DIRECTORY_NAMES: [&str; 8] = [
    "pistol-core",
    "pistol-eval",
    "pistol-search",
    "pistol-solver",
    "pistol-engine",
    "pistol-arena",
    "pistol-cli",
    "pistol-api",
];

#[test]
fn the_scratch_sweep_spares_every_directory_it_did_not_make() {
    let dir = scratch("scratch-sweep-spares");
    for name in WORKSPACE_DIRECTORY_NAMES {
        std::fs::create_dir(dir.join(name)).expect("a decoy directory");
    }
    std::fs::create_dir(dir.join("unrelated-dir")).expect("a decoy directory");
    // Both suites' spellings, because they share one temp directory and this
    // sweep is what removes what the arena's `Drop` guard could not: a test
    // binary that aborts or is killed runs no destructor.
    let mine = dir.join(format!("{SCRATCH_PREFIX}1234-something-0"));
    let arena = dir.join(format!("{SCRATCH_PREFIX}arena-something-1234"));
    std::fs::create_dir(&mine).expect("a scratch directory of this suite's own");
    std::fs::create_dir(&arena).expect("a scratch directory of the arena suite's");

    // Everything in there is older than nothing, so age decides nothing here and
    // the NAME is the whole of what is under test.
    sweep_scratch_in(&dir, Duration::ZERO);

    for name in WORKSPACE_DIRECTORY_NAMES.iter().chain(&["unrelated-dir"]) {
        assert!(
            dir.join(name).is_dir(),
            "the sweep removed `{name}`, which it did not create"
        );
    }
    assert!(!mine.exists(), "and it still removes its own leftovers");
    assert!(
        !arena.exists(),
        "including the arena suite's, which nothing else sweeps"
    );
}

#[test]
fn the_sweep_leaves_a_scratch_directory_that_is_not_stale_alone() {
    // The age half, so the name test above cannot pass a sweep that removes
    // everything named right regardless of when it was written — which would
    // delete the directories of a CONCURRENT test process.
    let dir = scratch("scratch-sweep-fresh");
    let fresh = dir.join(format!("{SCRATCH_PREFIX}5678-fresh-0"));
    std::fs::create_dir(&fresh).expect("a scratch directory");

    sweep_scratch_in(&dir, Duration::from_secs(6 * 60 * 60));

    assert!(
        fresh.is_dir(),
        "a directory younger than the staleness bound belongs to a live run"
    );
}

#[test]
fn every_scratch_directory_this_suite_makes_carries_the_prefix_the_sweep_matches() {
    let path = scratch("scratch-sweep-naming");
    let name = path
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .expect("a scratch directory has a utf-8 name");
    assert!(
        name.starts_with(SCRATCH_PREFIX),
        "a scratch directory the sweep cannot recognize is one nothing ever removes: `{name}`"
    );
    assert!(
        !WORKSPACE_DIRECTORY_NAMES
            .iter()
            .any(|crate_dir| crate_dir.starts_with(SCRATCH_PREFIX)),
        "and the prefix may not be one a crate directory carries"
    );
}
