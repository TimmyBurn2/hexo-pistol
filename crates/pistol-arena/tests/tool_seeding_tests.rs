mod common;

use common::{Scratch, seed_tool};

/// The arena crate's own seeder computes the same transitive closure.
///
/// `pistol-cli`'s twin is pinned by its own suite; this one was not, so the two
/// hand-duplicated copies could drift with nothing observing it — which is what
/// the REVIEW-impl of `common::seed_tool` found (tools/SHELL_CHECKLIST.md item
/// 10: a number nothing tests is a number nothing defends).
#[test]
fn seeding_a_gate_lands_the_siblings_it_reaches_for_transitively() {
    let scratch = Scratch::new("arena_seed_closure");
    let root = scratch.dir.join("repo");
    seed_tool(&root, "tools/determinism.sh");

    for expected in [
        "tools/determinism.sh",
        "tools/scratch_preflight.sh",
        "tools/require_tool.sh",
    ] {
        assert!(
            root.join(expected).is_file(),
            "the closure must reach {expected}"
        );
    }
}

/// THE CONTROL: it follows references rather than sweeping `tools/`.
#[test]
fn the_closure_follows_references_rather_than_sweeping_the_directory() {
    let scratch = Scratch::new("arena_seed_control");
    let root = scratch.dir.join("repo");
    seed_tool(&root, "tools/require_tool.sh");

    let landed = std::fs::read_dir(root.join("tools"))
        .expect("the tools directory")
        .count();
    assert_eq!(
        landed, 1,
        "a script that references no sibling seeds only itself; anything more \
         means the seeder is sweeping rather than following"
    );
}

/// A `tools/` script's NON-shell siblings land too.
///
/// `governing_citation_check.sh` resolves two `.py` files by path literal and
/// VOIDS without them. A `.sh`-only closure therefore seeded a tree the gate
/// refuses in — the eight-times class, one file extension away from where it
/// was closed.
#[test]
fn the_closure_reaches_a_python_sibling_a_gate_resolves() {
    let scratch = Scratch::new("arena_seed_python");
    let root = scratch.dir.join("repo");
    seed_tool(&root, "tools/governing_citation_check.sh");

    for expected in [
        "tools/governing_citation_check.sh",
        "tools/design_citation_check.py",
        "tools/revision_citation_check.py",
    ] {
        assert!(
            root.join(expected).is_file(),
            "the gate resolves {expected} by path literal and voids without it"
        );
    }
}

/// A sibling reached through `..` from a SUBDIRECTORY script lands.
///
/// A script under `tools/<dir>/` cannot spell a flat sibling any other way, so
/// a scan that does not normalize `..` seeds such a script alone.
#[test]
fn a_subdirectory_script_still_reaches_its_flat_sibling() {
    let scratch = Scratch::new("arena_seed_subdir");
    let root = scratch.dir.join("repo");
    seed_tool(&root, "tools/sealbot/run_match.sh");

    assert!(
        root.join("tools/require_tool.sh").is_file(),
        "run_match.sh reaches the resolver as `../require_tool.sh` and refuses \
         without it"
    );
}
