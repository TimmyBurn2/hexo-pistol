mod common;

use common::{repo_root, scratch};

/// The seeder brings a script's SIBLINGS, and the closure is transitive.
///
/// `determinism.sh` reaches for `scratch_preflight.sh`, which reaches for
/// `require_tool.sh`. Seeding the first must land all three, or the gate
/// refuses in a tree that looks complete.
#[test]
fn seeding_a_gate_lands_the_siblings_it_reaches_for_transitively() {
    let root = scratch("seed_closure").join("repo");
    common::seed_tool(&root, "tools/determinism.sh");

    for expected in [
        "tools/determinism.sh",
        "tools/scratch_preflight.sh",
        "tools/require_tool.sh",
    ] {
        assert!(
            root.join(expected).is_file(),
            "the closure must reach {expected}; a tree missing it is one the gate \
             refuses in, which is how this class was found eight times"
        );
    }
}

/// THE CONTROL: the closure FOLLOWS references, it does not sweep `tools/`.
///
/// A seeder that copied everything would satisfy the transitivity test above
/// and prove nothing. `require_tool.sh` reaches for no other script — it runs
/// on bash builtins alone — so its closure is itself.
///
/// Note the closure deliberately OVER-approximates within what it follows: a
/// script named only in a comment is copied too (`determinism.sh` mentions
/// `ci.sh`, and `ci.sh` lands). That is the safe direction, and it is why this
/// control is drawn at "does not sweep" rather than "copies nothing extra".
#[test]
fn the_closure_follows_references_rather_than_sweeping_the_directory() {
    let root = scratch("seed_control").join("repo");
    common::seed_tool(&root, "tools/require_tool.sh");

    assert!(root.join("tools/require_tool.sh").is_file(), "itself");
    let landed = std::fs::read_dir(root.join("tools"))
        .expect("the tools directory")
        .count();
    assert_eq!(
        landed, 1,
        "a script that references no sibling seeds only itself; anything more \
         means the seeder is sweeping rather than following"
    );
}

/// A seeded script keeps its execute bit, so the scratch tree can run it.
#[test]
fn a_seeded_script_is_still_executable() {
    use std::os::unix::fs::PermissionsExt;
    let root = scratch("seed_mode").join("repo");
    common::seed_tool(&root, "tools/require_tool.sh");
    let mode = std::fs::metadata(root.join("tools/require_tool.sh"))
        .expect("it landed")
        .permissions()
        .mode();
    assert!(mode & 0o111 != 0, "a copy that cannot be run is not a seed");
}

/// NO HARNESS COPIES A `tools/` SCRIPT BY HAND, so the class cannot come back.
///
/// **This is the guard, and it is a PROPERTY rather than a list.** The failure
/// it forbids is not hypothetical: a `tools/` script that gains a sibling
/// dependency breaks every harness that copies it without the sibling, and it
/// breaks them one red suite at a time. That happened for the preflight and
/// again for the resolver — eight harnesses across three discovery passes — and
/// a hand-written detector missed two of them because it matched
/// `repo("tools/x.sh")` and one harness passed `repo(file)` out of an array.
///
/// `common::seed_tool` computes the closure, so a new dependency needs no edit
/// anywhere. This test is what stops a future harness from bypassing it and
/// re-opening the class.
#[test]
fn no_test_copies_a_tools_script_outside_the_seeder() {
    let mut offenders = Vec::new();
    let mut stack = vec![repo_root().join("crates")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("a readable directory") {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            if !path.components().any(|c| c.as_os_str() == "tests") {
                continue;
            }
            // The seeder itself, and this test, name the paths legitimately.
            if path.ends_with("common/mod.rs") || path.ends_with("tool_seeding_tests.rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("a readable test");
            for (line, _) in hand_written_copies(&text) {
                offenders.push(format!(
                    "{}:{line}",
                    path.strip_prefix(repo_root()).unwrap_or(&path).display()
                ));
            }
        }
    }
    offenders.sort();
    offenders.dedup();
    assert!(
        offenders.is_empty(),
        "these copy a `tools/` script by hand instead of through \
         `common::seed_tool`, which is what re-opens the missing-sibling class:\n  {}",
        offenders.join("\n  ")
    );
}

/// The line of every `fs::copy` whose SOURCE argument names a `tools/` path.
///
/// **THE SOURCE ARGUMENT, PARSED — not a window.** A window-based version of
/// this flagged two already-converted harnesses, because a `seed_tool` call and
/// an unrelated config copy sat within six lines of each other: the same
/// wrong-population defect the guard exists to prevent, committed inside the
/// guard.
///
/// **AND THE OFFSET IS A CHARACTER OFFSET, WHICH IT WAS NOT.** `str::find`
/// answers in BYTES and this walks `char`s; every test file in this repository
/// holds non-ASCII prose, so the two disagree by the skew accumulated before
/// the site — 26 characters at `decision_key_check_tests.rs`'s own copy — and
/// the scan read out of the middle of a later token. It extracted `ion_key_check.sh")`
/// where the source argument is `repo("tools/decision_key_check.sh")`, which
/// does not contain `tools/`, so the guard reported no offender while one was
/// live in the tree it had just walked.
fn hand_written_copies(text: &str) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    let mut at = 0usize;
    while let Some(offset) = text[at..].find("fs::copy(") {
        let open = at + offset + "fs::copy(".len();
        let mut depth = 1usize;
        let mut source = String::new();
        for ch in text[open..].chars() {
            match ch {
                '(' => depth += 1,
                ')' => depth -= 1,
                ',' if depth == 1 => break,
                _ => {}
            }
            if depth == 0 {
                break;
            }
            source.push(ch);
        }
        if source.contains("tools/") {
            found.push((text[..open].lines().count(), source.trim().to_owned()));
        }
        at = open;
    }
    found
}

/// THE CONTROL for the test above, and it DRIVES THE SCAN.
///
/// The control this replaces asserted that a string literal it had just written
/// contained substrings of itself. It never called the parser, it would have
/// passed with the parser deleted, and it was green throughout the whole period
/// the parser was extracting `ion_key_check.sh")` — which is the definition of
/// a control that controls nothing (docs/decisions.md D-690, D-691).
#[test]
fn the_seeder_guard_sees_a_hand_written_copy_and_leaves_a_seeded_one_alone() {
    // The non-ASCII prose is the point: it is what made the shipped scan read
    // from the wrong offset, so a control without it cannot fail that way.
    let offender = "\
/// A comment with an em dash — and a quotation mark ‘like this’.
fn seeds() {
    std::fs::copy(repo(\"tools/determinism.sh\"), root.join(\"x\")).unwrap();
}
";
    // THE PARSED ARGUMENT, not just the count. A count-only assertion passes
    // under an offset error whose skew happens to land back inside the token —
    // MEASURED: the shipped byte-indexed scan read this very sample from six
    // characters late, which is exactly `repo("`, and still saw `tools/`.
    assert_eq!(
        hand_written_copies(offender),
        vec![(3usize, "repo(\"tools/determinism.sh\")".to_owned())],
        "the scan must extract the SOURCE ARGUMENT, at the right offset, \
         through non-ASCII prose"
    );

    let innocent = "\
/// A comment with an em dash — and a quotation mark ‘like this’.
fn seeds() {
    common::seed_tool(&root, \"tools/determinism.sh\");
    std::fs::copy(scratch.path(\"a.toml\"), root.join(\"b.toml\")).unwrap();
}
";
    assert!(
        hand_written_copies(innocent).is_empty(),
        "and it must leave a seeded harness alone: a guard that flags everything \
         is a guard nobody can keep green"
    );
}

/// The guard's exemptions do not swallow the crates it is supposed to walk.
#[test]
fn the_guard_walks_both_crates_that_seed_tools_scripts() {
    let mut crates = std::collections::BTreeSet::new();
    let mut stack = vec![repo_root().join("crates")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("a readable directory") {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            if !path.components().any(|c| c.as_os_str() == "tests") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("a readable test");
            if text.contains("seed_tool(") {
                let rel = path.strip_prefix(repo_root()).unwrap_or(&path);
                let name = rel.components().nth(1).expect("crates/<name>/…");
                crates.insert(name.as_os_str().to_string_lossy().into_owned());
            }
        }
    }
    assert!(
        crates.contains("pistol-cli") && crates.contains("pistol-arena"),
        "the seeder is used in both crates and the guard must reach both; saw {crates:?}"
    );
}

/// THE CONTAINMENT GUARD: a candidate that climbs above `tools/` is refused.
///
/// `next` is joined onto the caller's scratch root and it is lifted out of a
/// FILE'S TEXT, so an unguarded `..` writes outside the tree the harness asked
/// for — tools/SHELL_CHECKLIST.md item 11, whose whole point is that a `cd` (or
/// here a `join`) constrains relative paths and nothing else.
#[test]
fn a_reference_that_climbs_above_the_root_is_refused_rather_than_joined() {
    for escaping in ["tools/../../victim.sh", "../victim.sh", "tools/../.."] {
        assert_eq!(
            common::lexically_normal(escaping),
            None,
            "`{escaping}` leaves the root and must not become a destination"
        );
    }
}

/// THE CONTROL for it: an ordinary `..` INSIDE the tree still resolves.
///
/// A guard that refused every `..` would pass the test above and break the
/// subdirectory case the seeder needs (`tools/sealbot/../require_tool.sh`).
#[test]
fn a_reference_that_stays_inside_the_root_still_resolves() {
    assert_eq!(
        common::lexically_normal("tools/sealbot/../require_tool.sh").as_deref(),
        Some("tools/require_tool.sh")
    );
    assert_eq!(
        common::lexically_normal("tools//require_tool.sh").as_deref(),
        Some("tools/require_tool.sh"),
        "and a doubled separator is one path, not two closure entries"
    );
}
