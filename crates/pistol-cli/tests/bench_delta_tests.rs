//! `tools/bench_delta.sh` — the pre-registered `Eval::delta` bench harness
//! (CLAUDE.md rule 5; docs/decisions.md D-215, D-220, D-226, D-231).
//!
//! # Why this file exists at all
//!
//! NOTHING IN THIS REPOSITORY DROVE THIS SCRIPT. It produced the project's
//! official perf verdict (D-220) and two consecutive review rounds found defects
//! in it — a leaked worktree, a second `mktemp` that destroyed its own trap, a
//! false refusal on a version bump, and then a SILENT ABORT on the very
//! difference the fix for that false refusal introduced — and each time the
//! defect was found by a reviewer running the script by hand, because no test
//! ran it at all. That is the gap these tests close (D-231).
//!
//! # Why they run in a SCRATCH GIT REPOSITORY
//!
//! The script roots itself at `dirname/..`, reads three pinned documents from
//! there, and on exit runs `git worktree prune` and `git worktree list` against
//! whatever repository it landed in. Pointing it at the live checkout would make
//! a test mutate the repository it is being reviewed in. A scratch repository
//! holding copies of the four paths the script actually reads is both safer and
//! sharper: the FIXTURE is an input the test controls, so the empty-fixture
//! refusal can be exercised without touching the sha-pinned committed one.
//!
//! # Why the engines are stubs
//!
//! The claims here are about the script's CONTROL FLOW — that a handshake
//! difference outside the guarded set is a note and not a death, and that an
//! unusable fixture is a named refusal and not a death. Both need two binaries
//! whose handshakes differ in a chosen field, which the real engine cannot be
//! asked for. `bench_delta.sh` takes each side as a path to an executable, so
//! nothing here patches anything.
//!
//! # RULE9-JUSTIFICATION: one script's control flow, over one pre-registration.
//! Every test here is the same claim — that this harness reaches a verdict or
//! refuses by name, and never dies saying nothing — and each needs the same two
//! stubs and the same scratch tree. Splitting them would put the stubs in one
//! file and the claims that depend on their exact handshake in another.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

/// The five fields `bench_delta.sh` guards, printed identically by both stubs so
/// that only the UNGUARDED `id version` differs. Kept in the stub rather than
/// imported: this is the engine's side of the contract, and agreeing with the
/// script by construction is the failure mode to avoid.
const GUARDED_ID_LINES: &str = "id config configs/instrument_v0.toml\\nid eval handcrafted_v0\\n\
     id tt_bytes 268435456\\nid candidate_policy radius 2\\n\
     id weights_sha256 0000000000000000000000000000000000000000000000000000000000000000\\n";

/// A stub engine that answers the `pistol` handshake and every `go` with one
/// `info totals` line, reporting `version` as given.
///
/// The totals are constant, so the two sides agree on nodes per position — the
/// node-identity assertion the bench makes of a search-identical change — and
/// the run reaches a verdict rather than dying inside it.
fn stub_engine(dir: &Path, name: &str, version: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(
        &path,
        format!(
            r#"#!/usr/bin/env bash
input="$(cat)"
if grep -q '^pistol$' <<<"$input"; then
	printf 'id name pistol\nid version {version}\nid protocol v0\nid mode instrument\n'
	printf 'id budgets depth_turns nodes\n'
	printf '{GUARDED_ID_LINES}'
	echo pistolok
	exit 0
fi
for _ in $(seq 1 "$(grep -c '^go ' <<<"$input")"); do
	echo 'info totals depth_turns 2 seldepth 3 nodes 50000 nps 10000000 time 5 hashfull 0 score cp 1 pv 0,0/0,1'
	echo 'bestmove 0,0/0,1'
done
exit 0
"#
        ),
    )
    .expect("the stub engine writes");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
            .expect("the stub engine is executable");
    }
    path
}

/// A git repository holding copies of exactly the paths `bench_delta.sh` reads,
/// with `fixture` as the bench fixture's contents.
fn scratch_repo(name: &str, fixture: &str) -> PathBuf {
    let root = scratch_tree(name, fixture);
    let ran = Command::new("git")
        .current_dir(&root)
        .args(["init", "-q"])
        .output()
        .expect("git runs");
    assert!(
        ran.status.success(),
        "git init: {}",
        String::from_utf8_lossy(&ran.stderr)
    );
    root
}

/// The same tree with NO repository in it — the case the exit trap has to
/// survive, since every `git` call the script makes lives in `cleanup`.
fn scratch_tree(name: &str, fixture: &str) -> PathBuf {
    let root = scratch(name).join("repo");
    for dir in ["tools", "configs", "crates/pistol-cli/tests/fixtures"] {
        std::fs::create_dir_all(root.join(dir)).expect("the scratch tree is created");
    }
    for file in [
        "tools/bench_delta.sh",
        "configs/instrument_v0.toml",
        "configs/eval_v0_weights.toml",
    ] {
        std::fs::copy(repo(file), root.join(file)).expect("the pinned document copies");
    }
    std::fs::write(
        root.join("crates/pistol-cli/tests/fixtures/bench_positions_v1.txt"),
        fixture,
    )
    .expect("the bench fixture writes");
    root
}

/// Two real fixture entries, one from each band, in the pinned fixture's own line
/// form: the smallest input that makes both `report_band` calls measure
/// something. The band edge is restated rather than imported for the reason
/// `two_band_corpus` restates it — this file is an INPUT to the script.
fn two_band_fixture() -> String {
    let pinned = std::fs::read_to_string(repo(
        "crates/pistol-cli/tests/fixtures/bench_positions_v1.txt",
    ))
    .expect("the bench fixture reads");
    let entries: Vec<&str> = pinned
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .collect();
    let stones = |entry: &str| -> u32 {
        entry
            .rsplit(" stones ")
            .next()
            .and_then(|tail| tail.trim().parse().ok())
            .unwrap_or_else(|| panic!("every fixture entry states its stone count: {entry}"))
    };
    let early = entries
        .iter()
        .find(|entry| stones(entry) <= 17)
        .expect("the fixture states an early position");
    let late = entries
        .iter()
        .find(|entry| stones(entry) > 17)
        .expect("the fixture states a late position");
    format!("# a two-band bench fixture\n{early}\n{late}\n")
}

/// Run the shipped script inside `root` with two sides.
fn bench_delta(root: &Path, base: &Path, cand: &Path) -> Output {
    Command::new("bash")
        .arg(root.join("tools/bench_delta.sh"))
        .arg(base)
        .arg(cand)
        .arg("5")
        .current_dir(root)
        .output()
        .expect("the bench script runs")
}

#[test]
fn bench_delta_drives_to_a_verdict_when_the_handshakes_differ_outside_the_guarded_set() {
    let root = scratch_repo("bench-delta-note", &two_band_fixture());
    let stubs = scratch("bench-delta-note-stubs");
    let base = stub_engine(&stubs, "engine-a", "0.0.1");
    let cand = stub_engine(&stubs, "engine-b", "0.0.2");

    let ran = bench_delta(&root, &base, &cand);
    let out = format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );

    // THE REGRESSION. Two revisions differ in `name`/`version` BY CONSTRUCTION —
    // D-226 says so in those words — so this is the commonest run there is, and
    // it used to die at the NOTE with exit 1, no `FAIL:` line, no identity block,
    // no rep and no verdict: CLAUDE.md rule 3 broken in the most literal way, by
    // a `set -e` + `pipefail` interaction inside the branch that exists to
    // TOLERATE the difference (docs/decisions.md D-231).
    assert!(
        ran.status.success(),
        "a difference outside the guarded set is a note, not a death:\n{out}"
    );
    assert!(
        out.contains("NOTE the handshakes differ outside the guarded set"),
        "and the note is printed:\n{out}"
    );
    for line in ["-id version 0.0.1", "+id version 0.0.2"] {
        assert!(
            out.contains(line),
            "the note names the fields that differ (`{line}`):\n{out}"
        );
    }
    // Everything the silent abort swallowed, in the order the script prints it.
    for marker in [
        "bench_delta: identity id config configs/instrument_v0.toml",
        "bench_delta: rep 1/5 base",
        "node identity holds per position",
        "band early: VERDICT",
        "band late: VERDICT",
        "bench_delta: done —",
    ] {
        assert!(out.contains(marker), "the run reaches `{marker}`:\n{out}");
    }
    // The script closes on the invariant D-217 and D-219 close their rounds on,
    // printed rather than asserted — and in a scratch repository, so nothing
    // here could have touched the live one.
    assert!(
        out.contains("bench_delta: worktrees at exit:"),
        "the worktree listing is printed at exit:\n{out}"
    );
}

#[test]
fn bench_delta_refuses_a_fixture_with_no_positions_by_name() {
    // The same defect class one line into the measurement half: both greps exit
    // 1 on a fixture with no entries, `pipefail` propagates it, and `set -e`
    // killed the script one line BEFORE the refusal written for this case
    // (docs/decisions.md D-231's named exception to the byte-identical half).
    let root = scratch_repo(
        "bench-delta-empty-fixture",
        "# every line here is a comment\n# and none of them is a position\n",
    );
    let stubs = scratch("bench-delta-empty-fixture-stubs");
    let base = stub_engine(&stubs, "engine-a", "0.0.1");
    let cand = stub_engine(&stubs, "engine-b", "0.0.1");
    // The two stubs differ in nothing the handshake shows, so the digest refusal
    // must not fire first: give the candidate a byte the baseline lacks.
    let mut body = std::fs::read_to_string(&cand).expect("the stub reads");
    body.push_str("# a comment, so the two sides are not the same file\n");
    std::fs::write(&cand, body).expect("the stub rewrites");

    let ran = bench_delta(&root, &base, &cand);
    let out = format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
    assert!(
        !ran.status.success(),
        "an unusable fixture is a refusal:\n{out}"
    );
    assert!(
        out.contains("bench_delta: FAIL: the fixture states no positions"),
        "and the refusal is the NAMED one, not a bare exit 1:\n{out}"
    );
}

#[test]
fn a_bench_that_printed_its_verdict_exits_0_even_when_the_worktree_listing_cannot_be_taken() {
    // `cleanup` is the EXIT trap and its last command was
    // `git worktree list | sed …` — a pipeline, under `pipefail`, whose status
    // becomes the script's. Outside a git directory that is 128, so a COMPLETED
    // bench that had already printed `bench_delta: done —` exited 128
    // (REPRODUCED at ccba146). A verdict printed and a run failed are the two
    // things this exit status exists to tell apart, and a housekeeping listing
    // may decide neither.
    let root = scratch_tree("bench-delta-nogit", &two_band_fixture());
    let stubs = scratch("bench-delta-nogit-stubs");
    let base = stub_engine(&stubs, "engine-a", "0.0.1");
    let cand = stub_engine(&stubs, "engine-b", "0.0.1");
    let mut body = std::fs::read_to_string(&cand).expect("the stub reads");
    body.push_str("# a comment, so the two sides are not the same file\n");
    std::fs::write(&cand, body).expect("the stub rewrites");

    let ran = bench_delta(&root, &base, &cand);
    let out = format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
    assert!(
        out.contains("bench_delta: done —"),
        "the run reaches its verdict:\n{out}"
    );
    assert!(
        ran.status.success(),
        "and a listing it could not take does not change that:\n{out}"
    );
    assert!(
        out.contains("WARNING: `git worktree list` could not be taken"),
        "the listing that failed is named rather than swallowed:\n{out}"
    );
}

#[test]
fn a_side_this_harness_cannot_read_is_refused_by_name() {
    // The sibling of tools/baseline_snapshot.sh's empty-digest defect, one
    // spelling over: here `BASE_SHA="$(sha256sum "$BASE" | …)"` let `set -e`
    // kill the run with NO `bench_delta: FAIL:` line at all — rule 3's other
    // failure, a death that names nothing (REPRODUCED at ccba146, exit 128 with
    // only `sha256sum: … Permission denied` on stderr). An executable that is
    // not readable is exactly the case, and it matters because two unreadable
    // sides would both have digested to the empty string and been refused as
    // "the same binary", which is a wrong diagnosis.
    let root = scratch_repo("bench-delta-unreadable", &two_band_fixture());
    let stubs = scratch("bench-delta-unreadable-stubs");
    let base = stub_engine(&stubs, "engine-a", "0.0.1");
    let cand = stub_engine(&stubs, "engine-b", "0.0.2");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&base, std::fs::Permissions::from_mode(0o111))
            .expect("the baseline side is executable and not readable");
    }

    let ran = bench_delta(&root, &base, &cand);
    let out = format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
    assert!(
        !ran.status.success(),
        "an undigestible side is a refusal:\n{out}"
    );
    assert!(
        out.contains("bench_delta: FAIL: cannot read the baseline side at"),
        "and the refusal is the NAMED one, naming the side:\n{out}"
    );
}
