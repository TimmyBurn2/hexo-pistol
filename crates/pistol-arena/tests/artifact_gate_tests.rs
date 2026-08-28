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
    // Reports renamed past every name pattern: no artifact extension, but the
    // first line is the schema header the arena writes — verbatim, with a
    // trailing space, and with trailing tokens, because a token-splitting
    // consumer reads all three as reports and RED-TEAM showed the last two
    // slipped an exact-match regex (docs/decisions.md D-205).
    scratch.write("report.txt", "arena_report 4\narena_version 0.0.1\n");
    scratch.write("padded.txt", "arena_report 4 \narena_version 0.0.1\n");
    scratch.write(
        "suffixed.txt",
        "arena_report 4 extra\narena_version 0.0.1\n",
    );
    git(&["add", "report.txt", "padded.txt", "suffixed.txt"]);

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
    for name in ["report.txt", "padded.txt", "suffixed.txt"] {
        assert!(
            stderr.contains(&format!("match report by content: {name}")),
            "the gate names {name} and the reason: {stderr}"
        );
    }

    // Control: the same repo without the reports passes, so the failure above
    // is the detection and not the scaffolding refusing everything.
    for name in ["report.txt", "padded.txt", "suffixed.txt"] {
        git(&["rm", "-q", "--cached", name]);
        std::fs::remove_file(scratch.path(name)).expect("the decoy removes");
    }
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

/// The same content recognizer, for the baseline snapshot record
/// (tools/baseline_snapshot.sh, docs/decisions.md D-230).
///
/// A new record kind could have been given a header that simply did not collide
/// with the recognizer, which would have re-opened D-203's hole for a new class
/// instead of closing it. It joined the recognizer instead, and this is the test
/// that says so — the same shape as the report's, including the token-splitting
/// variants D-205's RED-TEAM found slipped an exact match.
#[test]
fn artifact_check_catches_a_committed_baseline_snapshot() {
    let scratch = Scratch::new("artifact-gate-snapshot");
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
    scratch.write("snap.txt", "baseline_snapshot 1\nschema 1\n");
    scratch.write("partial.txt", "baseline_snapshot_incomplete 1\nschema 1\n");
    scratch.write("suffixed.txt", "baseline_snapshot 1 extra\nschema 1\n");
    // The three spellings that slipped BOTH recognizers until D-230: a consumer
    // splitting on whitespace reads all of them as records, and the recognizer's
    // own comment says that is the reason it matches tokens rather than the
    // exact line.
    scratch.write("lead_ws.txt", " baseline_snapshot 1\nschema 1\n");
    scratch.write("lead_tab.txt", "\tbaseline_snapshot 1\nschema 1\n");
    scratch.write("bom.txt", "\u{feff}baseline_snapshot 1\nschema 1\n");
    let caught = [
        "snap.txt",
        "partial.txt",
        "suffixed.txt",
        "lead_ws.txt",
        "lead_tab.txt",
        "bom.txt",
    ];
    // THE NEGATIVE CONTROL this test had none of. A recognizer that flagged
    // everything would pass every assertion above, and the two files this
    // change actually adds to the repository — the script and this suite — are
    // safe only because the gate reads the FIRST line. Nothing pinned that.
    let untouched = [
        ("near_miss.txt", "baseline_snapshotX 1\nnot a record\n"),
        ("no_schema.txt", "baseline_snapshot 1extra\nnot a record\n"),
        (
            "prose.md",
            "# baseline_snapshot 1 is the record this document describes\n",
        ),
        (
            "later_line.txt",
            "a first line\nbaseline_snapshot 1\nschema 1\n",
        ),
    ];
    for (name, body) in untouched {
        scratch.write(name, body);
    }
    let mut add = vec!["add"];
    add.extend(caught);
    add.extend(untouched.iter().map(|(name, _)| *name));
    git(&add);

    let ran = Command::new("bash")
        .arg(&script)
        .current_dir(&scratch.dir)
        .output()
        .expect("the gate runs");
    assert!(
        !ran.status.success(),
        "a committed snapshot record must fail the gate:\n{}",
        String::from_utf8_lossy(&ran.stdout)
    );
    let stderr = String::from_utf8_lossy(&ran.stderr);
    for name in caught {
        assert!(
            stderr.contains(&format!("baseline snapshot by content: {name}")),
            "the gate names {name} and the reason: {stderr}"
        );
    }
    for (name, _) in untouched {
        assert!(
            !stderr.contains(name),
            "the gate must not flag {name}, which no consumer reads as a record: {stderr}"
        );
    }
}

/// The gate reads the TRACKED BYTES, not the working tree
/// (docs/decisions.md D-233).
///
/// `git ls-files` names a path; `<"$path"` opened a DIFFERENT FILE — the
/// worktree's copy, which the committer controls independently of what reaches
/// HEAD. Two ways past it, both reproduced: stage the record and then overwrite
/// the worktree copy with something harmless, or commit the record and delete the
/// worktree copy without staging the deletion, which `[ -f "$path" ] || continue`
/// skipped outright along with the size ceiling. This is a CI gate that could be
/// WALKED PAST, which is worth more than its grade.
#[test]
fn artifact_check_reads_the_index_and_not_the_working_tree() {
    let scratch = Scratch::new("artifact-gate-index");
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
    let gate = || {
        Command::new("bash")
            .arg(&script)
            .current_dir(&scratch.dir)
            .output()
            .expect("the gate runs")
    };
    git(&["init", "-q"]);

    // (a) the record is what got staged; the worktree copy is a decoy.
    scratch.write("record.txt", "baseline_snapshot 1\nschema 1\n");
    git(&["add", "record.txt"]);
    scratch.write("record.txt", "nothing to see here\n");
    let ran = gate();
    assert!(
        !ran.status.success(),
        "the STAGED bytes are the tracked ones:\n{}",
        String::from_utf8_lossy(&ran.stdout)
    );
    assert!(
        String::from_utf8_lossy(&ran.stderr).contains("baseline snapshot by content: record.txt"),
        "and the gate names the path it read the record from: {}",
        String::from_utf8_lossy(&ran.stderr)
    );

    // (b) the record is committed and its worktree copy is simply gone. The
    // deletion is unstaged, so the blob is still tracked.
    git(&[
        "-c",
        "user.email=gate@test",
        "-c",
        "user.name=gate test",
        "commit",
        "-qm",
        "the record this gate must still see",
    ]);
    std::fs::remove_file(scratch.path("record.txt")).expect("the worktree copy removes");
    let ran = gate();
    assert!(
        !ran.status.success(),
        "a tracked-but-absent path is read from the index, never skipped:\n{}",
        String::from_utf8_lossy(&ran.stdout)
    );

    // Control: with the blob out of the index the same repository passes, so the
    // two failures above are the detection and not the scaffolding.
    git(&["rm", "-q", "--cached", "record.txt"]);
    let clean = gate();
    assert!(
        clean.status.success(),
        "a repository tracking no record passes: {}",
        String::from_utf8_lossy(&clean.stderr)
    );
}
