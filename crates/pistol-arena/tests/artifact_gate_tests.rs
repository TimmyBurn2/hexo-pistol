//! `tools/artifact_check.sh` recognizes a match report by CONTENT — the report
//! schema header on its first line — not by extension alone: a report renamed
//! `report.txt` passed both `.gitignore` and the gate's name patterns
//! (wp13_results §6b, closed by D-203).
//!
//! The gate is exercised in a scratch git repository rather than the real one,
//! because proving it catches a committed report would otherwise require
//! committing one (CLAUDE.md rule 8).

mod common;

use std::process::Command;

use common::{Scratch, repo};

#[test]
fn artifact_check_catches_renamed_report() {
    let scratch = Scratch::new("artifact-gate");
    // The script roots itself one directory above its own location, so it must
    // sit at <scratch>/tools/ exactly as it sits at <repo>/tools/ — copied to
    // the scratch repo's top level it would check the scratch's PARENT.
    std::fs::create_dir_all(scratch.path("tools")).expect("a tools directory");
    let script = scratch.path("tools/artifact_check.sh");
    std::fs::copy(repo().join("tools/artifact_check.sh"), &script).expect("the gate copies");

    let git = |args: &[&str]| {
        let output = Command::new("git")
            .current_dir(&scratch.dir)
            .args(args)
            .output()
            .expect("git runs");
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    git(&["init", "-q"]);
    // A report renamed past every name pattern: no artifact extension, but the
    // first line is the schema header the arena writes.
    scratch.write("report.txt", "arena_report 4\narena_version 0.0.1\n");
    git(&["add", "report.txt"]);

    let ran = Command::new("bash")
        .arg(&script)
        .current_dir(&scratch.dir)
        .output()
        .expect("the gate runs");
    assert!(
        !ran.status.success(),
        "a renamed report must fail the gate:\n{}",
        String::from_utf8_lossy(&ran.stdout)
    );
    let stderr = String::from_utf8_lossy(&ran.stderr);
    assert!(
        stderr.contains("match report by content: report.txt"),
        "the gate names the file and the reason: {stderr}"
    );

    // Control: the same repo without the report passes, so the failure above
    // is the detection and not the scaffolding refusing everything.
    git(&["rm", "-q", "--cached", "report.txt"]);
    std::fs::remove_file(scratch.path("report.txt")).expect("the decoy removes");
    let clean = Command::new("bash")
        .arg(&script)
        .current_dir(&scratch.dir)
        .output()
        .expect("the gate runs again");
    assert!(
        clean.status.success(),
        "a clean repository passes: {}",
        String::from_utf8_lossy(&clean.stderr)
    );
}
