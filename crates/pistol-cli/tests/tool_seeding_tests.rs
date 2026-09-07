mod common;

use common::{repo, repo_root, scratch};

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
    let tests = repo_root().join("crates");
    let mut offenders = Vec::new();
    let mut stack = vec![tests];
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
            // The seeder itself, and this test, name the paths legitimately.
            if path.ends_with("common/mod.rs") || path.ends_with("tool_seeding_tests.rs") {
                continue;
            }
            // THE SOURCE ARGUMENT, PARSED — not a window. A window-based
            // version of this guard flagged two ALREADY-CONVERTED harnesses,
            // because a `seed_tool` call and an unrelated config copy sat
            // within six lines of each other: the same wrong-population defect
            // the guard exists to prevent, committed inside the guard.
            let bytes: Vec<char> = text.chars().collect();
            let mut at = 0usize;
            while let Some(found) = text[at..].find("fs::copy(") {
                let open = at + found + "fs::copy(".len();
                let mut depth = 1usize;
                let mut i = open;
                let mut source = String::new();
                while i < bytes.len() && depth > 0 {
                    match bytes[i] {
                        '(' => depth += 1,
                        ')' => depth -= 1,
                        ',' if depth == 1 => break,
                        _ => {}
                    }
                    if depth > 0 {
                        source.push(bytes[i]);
                    }
                    i += 1;
                }
                if source.contains("tools/") {
                    let line = text[..open].lines().count();
                    offenders.push(format!(
                        "{}:{line}",
                        path.strip_prefix(repo_root()).unwrap_or(&path).display()
                    ));
                }
                at = open;
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

/// THE CONTROL for the test above: it can actually see an offender.
#[test]
fn the_seeder_guard_would_notice_a_hand_written_copy() {
    let sample = "std::fs::copy(repo(\"tools/determinism.sh\"), root.join(\"x\")).unwrap();";
    assert!(
        sample.contains("fs::copy") && sample.contains("\"tools/"),
        "the shape the guard keys on is the shape a hand-written copy has; if \
         this ever stops being true the guard above is green for the wrong reason"
    );
    assert!(
        repo("tools/require_tool.sh").is_file(),
        "and the repository still holds the sibling the class is about"
    );
}
