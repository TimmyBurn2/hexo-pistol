//! `tools/baseline_snapshot.sh` — the standing before/after instrument for
//! Stage-1 work packages (docs/decisions.md D-230, D-232).
//!
//! These drive the SHIPPED script rather than re-deriving its record in Rust:
//! a second implementation tests itself, which is the defect D-219 recorded
//! when `unrescuable_beyond` had no production call site and the headline
//! number was hand-summed off the rendered rows.
//!
//! The workload is shrunk with the script's WORKLOAD-SCOPE flags (`--corpus`,
//! `--ladder-depth`, `--binary`), which leave the registered node budget alone,
//! so no record produced here says `OVERRIDE` and none of it could be mistaken
//! for a baseline.
//!
//! # Why half of these drive a STUB and not the engine
//!
//! The first round of this suite bound the record's SHAPE and none of its
//! SEMANTICS: five mutants survived it, four of them semantic (the kind token
//! nailed to `complete`, the `ladder_terminal` line deleted, the weights digest
//! re-derived by the script instead of read from the engine, the setup flag
//! nailed to `ok`). Every one of those is a claim about what the script does
//! with an answer it did not expect, and the real engine cannot be made to give
//! an unexpected answer. The stub can, and `--binary` is a shipped flag, so
//! nothing here patches anything (D-230). The second round then found EIGHT MORE
//! repaired failure paths that no test bound at all — the CRLF guard, the
//! both-bands refusal, the binary resolution, the bestmove count, the score
//! parse, the corpus exit status — and this round binds them (D-232).
//!
//! # Why the determinism test runs in a SCRATCH GIT REPOSITORY
//!
//! `snapshot_deterministic_across_a_clean_and_a_dirty_working_tree` is named for
//! two tree states, and in the tree it is actually run in — an uncommitted work
//! package under review — `git status --porcelain` is ALREADY non-empty, so both
//! of its snapshots saw `dirty` and the test was VACUOUS: a mutant emitting the
//! tree token both above and below the marker survived it. The earlier fix, an
//! untracked marker file written into the repository ROOT, also broke the
//! invariant this suite's own scaffolding states — `git add -A` is the first step
//! of this project's review procedure, and a concurrent one would have staged
//! that file into the reviewed revision. A throwaway repository the test both
//! commits and dirties settles both: the two states are the test's to choose,
//! and nothing it writes is anywhere near the live index (D-232).
//!
//! # RULE9-JUSTIFICATION: one record's semantics, over one script. The stub, the
//! provenance claim, the terminal reasons and the refusals are one argument —
//! that the record states only what it read — and splitting them would separate
//! the mutants from the assertions that kill them.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, repo_root, scratch};

/// Where the record's invariant block ends.
const TIMING_MARKER: &str = "# timing";

/// A corpus stating one EARLY and one LATE position, in the pinned fixture's own
/// line form. The script takes one ladder rung from each band, so a single-band
/// corpus is refused — and a two-band one is also the only shape that shows the
/// bands doing anything.
fn two_band_corpus(dir: &Path) -> PathBuf {
    let path = dir.join("corpus.txt");
    let (early, late) = band_entries();
    std::fs::write(&path, format!("# test corpus\n{early}\n{late}\n"))
        .expect("the test corpus writes");
    path
}

/// One real fixture entry from each band.
fn band_entries() -> (String, String) {
    let fixture = std::fs::read_to_string(repo(
        "crates/pistol-cli/tests/fixtures/bench_positions_v1.txt",
    ))
    .expect("the bench fixture reads");
    let entries: Vec<&str> = fixture
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
    // 17 is the script's own band edge, restated here rather than imported:
    // this corpus is an INPUT to the script, so agreeing with it by accident is
    // the failure mode to avoid.
    let early = entries
        .iter()
        .find(|entry| stones(entry) <= 17)
        .expect("the fixture states an early position");
    let late = entries
        .iter()
        .find(|entry| stones(entry) > 17)
        .expect("the fixture states a late position");
    ((*early).to_owned(), (*late).to_owned())
}

/// How the shipped script is invoked: the flags, the engine, and the tree it
/// runs in.
struct Run {
    root: PathBuf,
    /// `--config`, required with no default (N-E, docs/decisions.md D-329).
    /// Every `Run` states one, so a test that wants the missing-flag refusal
    /// uses [`Run::no_config`] rather than an absent field doing it by
    /// omission.
    config: Option<PathBuf>,
    corpus: PathBuf,
    binary: PathBuf,
    out: Option<PathBuf>,
    ladder_depth: String,
    ladder_cap_s: Option<String>,
    nodes: Option<String>,
    mode: Option<String>,
    path_prefix: Option<PathBuf>,
}

impl Run {
    /// A run against the live repository root, which every test that only READS
    /// the tree may use: the script's own writes all go to `--out` and `$TMPDIR`.
    fn new(name: &str, binary: &Path) -> Self {
        let dir = scratch(name);
        Self {
            root: repo_root(),
            // Relative, resolved against whatever `self.root` ends up being
            // at `go()` time — the real repository root by default, or a
            // `ScratchRepo`, which copies this exact file in.
            config: Some(PathBuf::from("configs/instrument_v0.toml")),
            corpus: two_band_corpus(&dir),
            binary: binary.to_path_buf(),
            out: Some(dir.join("record.txt")),
            ladder_depth: "1".to_owned(),
            ladder_cap_s: None,
            nodes: None,
            mode: None,
            path_prefix: None,
        }
    }

    fn config(mut self, config: PathBuf) -> Self {
        self.config = Some(config);
        self
    }
    /// The missing-`--config` refusal class (N-E condition 3, D-329): a
    /// `Run` that omits the flag entirely rather than naming a bad value.
    fn no_config(mut self) -> Self {
        self.config = None;
        self
    }

    fn corpus(mut self, corpus: PathBuf) -> Self {
        self.corpus = corpus;
        self
    }
    fn ladder_depth(mut self, depth: &str) -> Self {
        self.ladder_depth = depth.to_owned();
        self
    }
    fn ladder_cap_s(mut self, cap: &str) -> Self {
        self.ladder_cap_s = Some(cap.to_owned());
        self
    }
    fn nodes(mut self, nodes: &str) -> Self {
        self.nodes = Some(nodes.to_owned());
        self
    }
    fn mode(mut self, mode: &str) -> Self {
        self.mode = Some(mode.to_owned());
        self
    }
    fn no_out(mut self) -> Self {
        self.out = None;
        self
    }
    fn out(mut self, out: PathBuf) -> Self {
        self.out = Some(out);
        self
    }
    fn root(mut self, root: PathBuf) -> Self {
        self.root = root;
        self
    }
    /// Prepend a directory to `PATH`, so `--binary` can be given as a bare name.
    fn path_prefix(mut self, dir: &Path) -> Self {
        self.path_prefix = Some(dir.to_path_buf());
        self
    }

    fn go(&self) -> Output {
        let script = self.root.join("tools/baseline_snapshot.sh");
        let mut command = Command::new("bash");
        command
            .arg(&script)
            .args(["--corpus", self.corpus.to_str().expect("utf-8 path")])
            .args(["--ladder-depth", &self.ladder_depth])
            .args(["--binary", self.binary.to_str().expect("utf-8 path")])
            .current_dir(&self.root);
        if let Some(config) = &self.config {
            command.args(["--config", config.to_str().expect("utf-8 path")]);
        }
        if let Some(out) = &self.out {
            command.args(["--out", out.to_str().expect("utf-8 path")]);
        }
        if let Some(cap) = &self.ladder_cap_s {
            command.args(["--ladder-cap-s", cap]);
        }
        if let Some(nodes) = &self.nodes {
            command.args(["--nodes", nodes]);
        }
        if let Some(mode) = &self.mode {
            command.env("PISTOL_STUB_MODE", mode);
        }
        if let Some(prefix) = &self.path_prefix {
            let existing = std::env::var("PATH").unwrap_or_default();
            command.env(
                "PATH",
                format!("{}:{existing}", prefix.to_str().expect("utf-8 path")),
            );
        }
        command.output().expect("the snapshot script runs")
    }

    /// Run it and read the record it must have written.
    fn record(&self) -> String {
        let ran = self.go();
        assert!(
            ran.status.success(),
            "the snapshot script must succeed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&ran.stdout),
            String::from_utf8_lossy(&ran.stderr)
        );
        let out = self.out.as_ref().expect("this run writes a record");
        std::fs::read_to_string(out).expect("the record reads")
    }

    /// Run it and read the refusal it must have printed.
    fn refusal(&self) -> String {
        let ran = self.go();
        let stderr = String::from_utf8_lossy(&ran.stderr).into_owned();
        assert!(
            !ran.status.success(),
            "this input is a refusal, not a record:\nstdout: {}\nstderr: {stderr}",
            String::from_utf8_lossy(&ran.stdout)
        );
        stderr
    }
}

/// A completed record from the real engine.
fn snapshot(name: &str) -> String {
    Run::new(name, Path::new(env!("CARGO_BIN_EXE_pistol"))).record()
}

/// The stub's weights digest: a value the on-disk weights file cannot have, so
/// a record carrying it can only have read it from the engine.
const STUB_WEIGHTS_SHA256: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

/// Where the `timex` stub's proof-of-execution file would be written.
fn pwned_path(dir: &Path) -> PathBuf {
    dir.join("PWNED")
}

/// An engine that answers exactly enough of the line protocol for the script,
/// and answers however `PISTOL_STUB_MODE` says.
///
/// | mode        | what it does                                                 |
/// |-------------|--------------------------------------------------------------|
/// | `follow`    | completes the ladder at depth 1 (the default)                 |
/// | `exit3`     | fails the ladder with a status that is not the cap's          |
/// | `short`     | stops the ladder below the requested depth with a `cp` score  |
/// | `capped`    | answers depth 1 and then HANGS, so the cap really fires       |
/// | `fastcap`   | answers depth 1 and exits 124 at once, which the cap did not  |
/// | `depthx`    | reports a depth that is not a number                          |
/// | `timex`     | reports a `time` that is not a number                         |
/// | `emptypv`   | reports a score with an empty `pv` after it                   |
/// | `shortbest` | answers one `bestmove` fewer than there are positions         |
/// | `corpusrc`  | fails the CORPUS session with a nonzero status                |
/// | `backward`  | reports depth 2 and then depth 1, so the ladder goes SHALLOWER |
fn stub_engine(name: &str) -> PathBuf {
    stub_engine_in(&scratch(name), "stub-engine")
}

/// The same stub, at a chosen directory and file name — so a test can put it on
/// `PATH` and name it without a slash.
fn stub_engine_in(dir: &Path, file: &str) -> PathBuf {
    let path = dir.join(file);
    // Where the `timex` payload would land if the guard against it were gone.
    // Inside the stub's own scratch directory and never the repository root: a
    // test whose MUTANT writes an untracked file into the live tree is the hazard
    // R3-M2 was about, and a proof-of-execution file is exactly such a write.
    let pwned = pwned_path(dir);
    let pwned = pwned.to_str().expect("utf-8 path");
    assert!(
        !pwned.contains(char::is_whitespace),
        "the payload is one whitespace-delimited token, so its path may not contain a space"
    );
    std::fs::write(
        &path,
        format!(
            r#"#!/usr/bin/env bash
input="$(cat)"
if grep -q '^pistol$' <<<"$input"; then
	printf 'id name pistol\nid version 0.0.1\nid protocol v0\nid mode instrument\n'
	printf 'id budgets depth_turns nodes\nid config configs/instrument_v0.toml\n'
	printf 'id eval handcrafted_v0\nid tt_bytes 268435456\nid candidate_policy radius 2\n'
	printf 'id weights_sha256 {STUB_WEIGHTS_SHA256}\n'
	echo pistolok
	exit 0
fi
rung='info depth_turns 1 seldepth 1 nodes 1843 nps 1000 time 1 hashfull 0 score cp 308 pv 0,0/0,1'
if grep -q '^go depth_turns ' <<<"$input"; then
	case "${{PISTOL_STUB_MODE:-follow}}" in
	exit3) exit 3 ;;
	short) echo "$rung"; exit 0 ;;
	capped) echo "$rung"; sleep 30; exit 0 ;;
	fastcap) echo "$rung"; exit 124 ;;
	depthx)
		echo 'info depth_turns x seldepth 1 nodes 1843 nps 1000 time 1 hashfull 0 score cp 308 pv 0,0/0,1'
		exit 0 ;;
	backward)
		echo 'info depth_turns 2 seldepth 2 nodes 3200 nps 1000 time 2 hashfull 0 score cp 291 pv 0,0/0,1'
		echo "$rung"
		echo 'bestmove 0,0/0,1'
		exit 0 ;;
	*)
		echo "$rung"
		echo 'bestmove 0,0/0,1'
		exit 0 ;;
	esac
fi
positions="$(grep -c '^go ' <<<"$input")"
totals='info totals depth_turns 2 seldepth 3 nodes 50176 nps 1000 time 5000 hashfull 0 score cp -326 pv 0,0/0,1'
case "${{PISTOL_STUB_MODE:-follow}}" in
timex) totals='info totals depth_turns 2 seldepth 3 nodes 50176 nps 1000 time PIPESTATUS[$(>{pwned})] hashfull 0 score cp -326 pv 0,0/0,1' ;;
emptypv) totals='info totals depth_turns 2 seldepth 3 nodes 50176 nps 1000 time 5000 hashfull 0 score cp -326 pv' ;;
corpusrc) echo "$totals"; echo 'bestmove 0,0/0,1'; exit 9 ;;
esac
for i in $(seq 1 "$positions"); do
	echo "$totals"
	if [ "${{PISTOL_STUB_MODE:-follow}}" = shortbest ] && [ "$i" -eq "$positions" ]; then continue; fi
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

/// The invariant block: every line strictly before the marker line.
///
/// Line-anchored, because the producer is: an unanchored search would find the
/// marker's text inside a record line and cut the block in the wrong place.
fn invariant(record: &str) -> String {
    let mut block = String::new();
    let mut seen = false;
    for line in record.lines() {
        if line.starts_with(TIMING_MARKER) {
            seen = true;
            break;
        }
        block.push_str(line);
        block.push('\n');
    }
    assert!(
        seen,
        "the record must carry the {TIMING_MARKER} marker:\n{record}"
    );
    block
}

/// Everything from the marker line on.
fn timing(record: &str) -> String {
    let mut block = String::new();
    for line in record.lines().skip_while(|l| !l.starts_with(TIMING_MARKER)) {
        block.push_str(line);
        block.push('\n');
    }
    block
}

/// A throwaway git repository holding copies of every path the script reads from
/// its own root, so a test can decide whether the tree is clean or dirty.
///
/// The config names its weights file relative to the root, and the script cds to
/// the root before launching the engine, so the copy is what the engine loads.
struct ScratchRepo {
    root: PathBuf,
}

impl ScratchRepo {
    fn new(name: &str) -> Self {
        let root = scratch(name).join("repo");
        for dir in ["tools", "configs", "crates/pistol-cli/tests/fixtures"] {
            std::fs::create_dir_all(root.join(dir)).expect("the scratch tree is created");
        }
        for file in [
            "tools/baseline_snapshot.sh",
            "configs/instrument_v0.toml",
            "configs/eval_v0_weights.toml",
            "crates/pistol-cli/tests/fixtures/openings_v1.txt",
        ] {
            std::fs::copy(repo(file), root.join(file)).expect("the pinned document copies");
        }
        let repository = Self { root };
        repository.git(&["init", "-q"]);
        repository.git(&["add", "-A"]);
        repository.git(&[
            "-c",
            "user.email=snapshot@test",
            "-c",
            "user.name=snapshot test",
            "commit",
            "-qm",
            "the tree this test calls clean",
        ]);
        repository
    }

    fn git(&self, args: &[&str]) {
        let ran = Command::new("git")
            .current_dir(&self.root)
            .args(args)
            .output()
            .expect("git runs");
        assert!(
            ran.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
    }

    /// `git status --porcelain`, so a test can assert the state it arranged.
    fn porcelain(&self) -> String {
        let ran = Command::new("git")
            .current_dir(&self.root)
            .args(["status", "--porcelain"])
            .output()
            .expect("git runs");
        String::from_utf8_lossy(&ran.stdout).into_owned()
    }

    fn dirty(&self) {
        std::fs::write(
            self.root.join("untracked-dirt.txt"),
            "this file exists to dirty the working tree\n",
        )
        .expect("the dirt writes");
    }
}

#[test]
fn snapshot_record_carries_engine_identity() {
    let record = snapshot("snapshot-identity");
    let block = invariant(&record);

    assert!(
        record.starts_with("baseline_snapshot 1\n"),
        "the record leads with its kind token and schema: {record}"
    );
    // The identity the ENGINE printed, not one the script re-derived: a digest
    // computed beside the engine could attest bytes the engine never read
    // (docs/decisions.md D-198).
    for field in [
        "engine_id config ",
        "engine_id eval ",
        "engine_id tt_bytes ",
        "engine_id candidate_policy ",
        "engine_id weights_sha256 ",
    ] {
        assert!(
            block.contains(field),
            "the invariant block must carry `{field}`:\n{block}"
        );
    }
    assert!(
        block.contains("binary_sha256 "),
        "the binary's own digest:\n{block}"
    );
    assert!(
        block.contains("revision "),
        "the revision it was taken at:\n{block}"
    );
    // The weights digest is the engine's, character for character.
    let engine_weights = block
        .lines()
        .find_map(|line| line.strip_prefix("engine_id weights_sha256 "))
        .expect("the engine printed a weights digest");
    let on_disk = pistol_cli::sha256::sha256_hex(
        &std::fs::read(repo("configs/eval_v0_weights.toml")).expect("the weights read"),
    );
    assert_eq!(
        engine_weights, on_disk,
        "the record carries the digest the engine printed for the file it loaded"
    );
    // A record taken with workload-scope flags is still AT the registered
    // budget, so nothing here can be mistaken for an off-budget number.
    assert!(
        block.contains("budget nodes 50000 registered"),
        "workload-scope flags leave the registered budget alone:\n{block}"
    );
}

#[test]
fn snapshot_deterministic_across_a_clean_and_a_dirty_working_tree() {
    // The two tree states this test is NAMED for, both of them arranged here
    // rather than inherited: in the tree the suite is actually run in the work
    // is uncommitted, so `git status --porcelain` is already non-empty and both
    // snapshots would see `dirty` — the arrangement in which a tree token above
    // the marker is invisible, which is exactly the defect (docs/decisions.md
    // D-230's BLOCKING, and D-232 for why this moved into a scratch repository).
    let repository = ScratchRepo::new("snapshot-determinism-repo");
    let engine = Path::new(env!("CARGO_BIN_EXE_pistol"));
    let dir = scratch("snapshot-determinism-runs");

    assert_eq!(
        repository.porcelain(),
        "",
        "the first snapshot is taken on a CLEAN tree, or this test cannot see the transition"
    );
    let first = Run::new("snapshot-determinism-a", engine)
        .root(repository.root.clone())
        .out(dir.join("first.txt"))
        .record();

    repository.dirty();
    assert_ne!(repository.porcelain(), "", "and the second on a DIRTY one");
    let second = Run::new("snapshot-determinism-b", engine)
        .root(repository.root.clone())
        .out(dir.join("second.txt"))
        .record();

    // The claim, stated exactly as pistol-arena's report states its own
    // (docs/decisions.md D-161): a run that COMPLETES has an invariant block,
    // and two such runs agree on it byte for byte.
    assert!(
        first.starts_with("baseline_snapshot 1\n") && second.starts_with("baseline_snapshot 1\n"),
        "both runs completed, so the invariance claim applies at all"
    );
    assert_eq!(
        invariant(&first),
        invariant(&second),
        "two runs at one revision, machine and budget share an invariant block — \
         the state of the working tree is not one of the three"
    );
    // The transition was EXERCISED: the two runs disagree about the tree, below
    // the marker, and agree above it. Without this the equality above would pass
    // on two runs that saw the same state.
    assert!(
        timing(&first).contains("timing tree clean"),
        "the first run saw a clean tree and said so: {}",
        timing(&first)
    );
    assert!(
        timing(&second).contains("timing tree dirty"),
        "the second run saw a dirty tree and said so: {}",
        timing(&second)
    );
    // And the whole CLASS is closed, not just the one token: no line above the
    // marker may say anything about the checkout. A mutant that emitted the
    // token in BOTH places passed every other assertion here.
    for line in invariant(&first).lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        assert!(
            !tokens.contains(&"dirty") && !tokens.contains(&"clean"),
            "no line above the marker states anything about the working tree: `{line}`"
        );
    }

    // Non-vacuity, STRUCTURALLY. Never "the two records differ": at a small
    // budget `time` is integer milliseconds and can be 0 in both runs, which
    // would fail a determinism test for a reason that has nothing to do with
    // determinism.
    assert!(
        invariant(&first).len() < first.len(),
        "the invariant block is a strict prefix, so something was excluded from it"
    );
    assert!(
        timing(&first).contains("timing position "),
        "the timing block carries the per-position measurements it exists for: {}",
        timing(&first)
    );
    assert!(
        invariant(&first).contains("position 1 "),
        "and the invariant block carries the per-position node counts"
    );
}

#[test]
fn the_invariant_block_states_the_revision_and_nothing_about_the_checkout() {
    let record = snapshot("snapshot-revision-line");
    let revision = invariant(&record)
        .lines()
        .find(|line| line.starts_with("revision "))
        .map(str::to_owned)
        .expect("the record states its revision");
    let tokens: Vec<&str> = revision.split_whitespace().collect();
    assert_eq!(
        tokens.len(),
        2,
        "`revision <sha>` and nothing else: a token that varies with the checkout \
         is not a property of the revision (D-230), got `{revision}`"
    );
}

#[test]
fn every_ladder_rung_states_a_terminal_reason_the_engine_supports() {
    let record = snapshot("snapshot-ladder-terminal");
    let block = invariant(&record);
    for rung in ["opening", "early_mid", "late_mid"] {
        let line = block
            .lines()
            .find(|line| line.starts_with(&format!("ladder_terminal {rung} ")))
            .unwrap_or_else(|| {
                panic!("every rung states a terminal reason; {rung} did not:\n{block}")
            });
        let reason = line
            .split_whitespace()
            .next_back()
            .expect("the reason is the last token");
        assert!(
            ["complete", "cap", "mate"].contains(&reason),
            "`{line}` states a reason this record is allowed to state"
        );
        // The rung reached the depth it claims: with `--ladder-depth 1` a
        // completed rung reached 1, and a rung that reached nothing cannot be
        // `complete`.
        assert!(
            line.contains(" reached 1 complete"),
            "at ladder depth 1 the real engine completes the rung: `{line}`"
        );
    }
    // And the ladder rungs came from the bands they are named after.
    let late = block
        .lines()
        .find(|line| line.starts_with("ladder late_mid depth "))
        .expect("the late rung ran");
    let early = block
        .lines()
        .find(|line| line.starts_with("ladder early_mid depth "))
        .expect("the early rung ran");
    assert_ne!(
        late.split_once(" depth ").map(|(_, rest)| rest),
        early.split_once(" depth ").map(|(_, rest)| rest),
        "the two rungs are different positions, not entry 1 twice under two names"
    );
}

#[test]
fn every_position_is_labelled_with_the_band_its_stone_count_states() {
    // The band is what makes an early number comparable to an early number, and
    // a mutant labelling every position `early` survived the first two rounds of
    // this suite (docs/decisions.md D-232).
    let record = snapshot("snapshot-bands");
    let block = invariant(&record);
    let bands: Vec<&str> = block
        .lines()
        .filter(|line| line.starts_with("position "))
        .map(|line| line.split_whitespace().nth(2).expect("the band token"))
        .collect();
    assert_eq!(
        bands,
        vec!["early", "late"],
        "the two-band corpus states one of each, in fixture order:\n{block}"
    );
}

#[test]
fn the_record_carries_the_digest_the_engine_printed_not_the_one_on_disk() {
    let stub = stub_engine("snapshot-provenance");
    let record = Run::new("snapshot-provenance-run", &stub)
        .mode("follow")
        .record();
    let carried = invariant(&record)
        .lines()
        .find_map(|line| {
            line.strip_prefix("engine_id weights_sha256 ")
                .map(str::to_owned)
        })
        .expect("the record carries a weights digest");
    let on_disk = pistol_cli::sha256::sha256_hex(
        &std::fs::read(repo("configs/eval_v0_weights.toml")).expect("the weights read"),
    );
    assert_eq!(
        carried, STUB_WEIGHTS_SHA256,
        "the record follows the ENGINE (docs/decisions.md D-198). A script that \
         re-derived this digest from configs/eval_v0_weights.toml would print \
         {on_disk} here and no test that compares the record to the file could \
         tell the difference"
    );
    assert_ne!(
        carried, on_disk,
        "and the stub's digest is one the file cannot produce, or this proves nothing"
    );
}

#[test]
fn a_binary_named_without_a_slash_is_digested_as_the_file_that_will_run() {
    // A bare name is PATH-resolved by the shell at exec time while `sha256sum`
    // would read the cwd-relative file of the same name, so an unresolved digest
    // can attest a file that never ran (the reason D-226 gives). Nothing bound
    // that here until now: the mutant removing the resolution survived.
    let dir = scratch("snapshot-bare-name");
    let stub = stub_engine_in(&dir, "pistol-stub-on-path");
    let record = Run::new("snapshot-bare-name-run", Path::new("pistol-stub-on-path"))
        .path_prefix(&dir)
        .mode("follow")
        .record();
    let carried = invariant(&record)
        .lines()
        .find_map(|line| line.strip_prefix("binary_sha256 ").map(str::to_owned))
        .expect("the record carries a binary digest");
    let on_path =
        pistol_cli::sha256::sha256_hex(&std::fs::read(&stub).expect("the stub engine reads"));
    assert_eq!(
        carried, on_path,
        "the digest is of the file the shell would exec, not of a name"
    );
}

#[test]
fn a_binary_that_is_not_a_regular_executable_file_is_refused_by_its_own_reason() {
    // `[ -x ]` admits a directory and a FIFO, and one combined test told an
    // operator who had named a directory to go and build the engine — a WRONG
    // diagnosis where a named one belongs (D-232).
    let dir = scratch("snapshot-bad-binary");
    let stderr = Run::new("snapshot-bad-binary-dir", &dir).refusal();
    assert!(
        stderr.contains("is a directory"),
        "the refusal names what it actually found: {stderr}"
    );
    let plain = dir.join("not-executable");
    std::fs::write(&plain, "#!/bin/sh\n").expect("the file writes");
    let stderr = Run::new("snapshot-bad-binary-plain", &plain).refusal();
    assert!(
        stderr.contains("is not an executable file"),
        "and so does this one: {stderr}"
    );
    // The FIFO is the case the REGULAR-FILE check exists for and the one neither
    // earlier round bound: it passes `[ -x ]` and `command -v` alike, and a read
    // of it never returns, which is indistinguishable from a slow machine.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let fifo = dir.join("fifo-engine");
        let made = Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .expect("mkfifo runs");
        assert!(made.success(), "this test needs a FIFO to point at");
        std::fs::set_permissions(&fifo, std::fs::Permissions::from_mode(0o755))
            .expect("the FIFO is executable, so only the regular-file check can refuse it");
        let stderr = Run::new("snapshot-bad-binary-fifo", &fifo).refusal();
        assert!(
            stderr.contains("is not a regular file"),
            "a FIFO is refused for being one, before anything reads it: {stderr}"
        );
    }
}

#[test]
fn a_setup_fraction_that_cannot_be_right_is_flagged_and_never_ok() {
    // The stub reports five seconds of engine time per position and returns at
    // once, so the measured setup fraction is large and NEGATIVE — the engine
    // claiming more time than the wall clock, i.e. one of the two numbers is
    // wrong. That used to print as `ok`.
    let stub = stub_engine("snapshot-setup-flag");
    let record = Run::new("snapshot-setup-flag-run", &stub)
        .mode("follow")
        .record();
    let line = timing(&record)
        .lines()
        .find(|line| line.starts_with("timing setup_fraction_pct "))
        .map(str::to_owned)
        .expect("the record measures its own setup fraction");
    assert!(
        line.ends_with(" SETUP-NEGATIVE"),
        "an impossible setup fraction carries a flag of its own: `{line}`"
    );
}

#[test]
fn a_capped_ladder_makes_the_record_a_different_kind_of_document() {
    // The cap is a WALL CLOCK, so the only honest evidence it fired is that the
    // run lasted it: the stub answers one depth and then hangs, against a cap
    // the flag makes short enough for a test to wait out (docs/decisions.md
    // D-160: a token, not a flag, so no consumer can diff a partial record
    // against a complete one).
    let stub = stub_engine("snapshot-capped");
    let record = Run::new("snapshot-capped-run", &stub)
        .mode("capped")
        .ladder_depth("3")
        .ladder_cap_s("1")
        .record();
    assert!(
        record.starts_with("baseline_snapshot_incomplete 1\n"),
        "a capped rung changes the KIND TOKEN, not a field: {record}"
    );
    // `reached` is the field that tells a cap at depth 1 from a cap at depth 0,
    // and a mutant writing the REQUESTED depth here survived the earlier rounds.
    for rung in ["opening", "early_mid", "late_mid"] {
        assert!(
            invariant(&record).contains(&format!("ladder_terminal {rung} reached 1 cap\n")),
            "the rung states the depth it actually reached, not the one asked for: {}",
            invariant(&record)
        );
    }
}

#[test]
fn a_status_the_cap_did_not_earn_is_not_recorded_as_the_cap() {
    // `timeout` passes the CHILD'S OWN status through unchanged, so 124 is also
    // what an engine exiting 124 yields and 137 is exactly what an OOM kill
    // yields. Both used to be recorded as "the 30 s cap fired", in milliseconds
    // (docs/decisions.md D-232).
    let stub = stub_engine("snapshot-fastcap");
    let stderr = Run::new("snapshot-fastcap-run", &stub)
        .mode("fastcap")
        .ladder_depth("3")
        .refusal();
    assert!(
        stderr.contains("exited 124 on the ladder position"),
        "the refusal names the status: {stderr}"
    );
    assert!(
        stderr.contains("did not reach the cap"),
        "and says why the status alone did not settle it: {stderr}"
    );
}

#[test]
fn an_engine_that_fails_the_ladder_is_not_recorded_as_a_cap() {
    let stub = stub_engine("snapshot-exit3");
    let stderr = Run::new("snapshot-exit3-run", &stub)
        .mode("exit3")
        .refusal();
    assert!(
        stderr.contains("the engine exited 3 on the ladder position"),
        "the refusal names the status it actually got: {stderr}"
    );
}

#[test]
fn a_ladder_that_stopped_short_without_a_mate_is_refused() {
    // Depth 2 requested, one depth completed, `cp` on the last line: the record
    // used to call that a mate, one line below the score contradicting it.
    let stub = stub_engine("snapshot-short");
    let stderr = Run::new("snapshot-short-run", &stub)
        .mode("short")
        .ladder_depth("2")
        .refusal();
    assert!(
        stderr.contains("neither the cap nor a mate"),
        "and the refusal says what it could not conclude: {stderr}"
    );
}

#[test]
fn a_ladder_depth_that_is_not_a_count_is_refused_before_it_reaches_a_comparison() {
    // `[ "$last" -lt "$LADDER_DEPTH" ]` with a non-numeric `last` writes
    // `[: x: integer expected` to stderr, evaluates FALSE, and falls through to
    // `reason=complete` — exit 0, a COMPLETE kind token and a terminal reason
    // nothing verified. F5's mechanism, reborn on the line F4's fix wrote
    // (docs/decisions.md D-232).
    let stub = stub_engine("snapshot-depthx");
    let stderr = Run::new("snapshot-depthx-run", &stub)
        .mode("depthx")
        .ladder_depth("3")
        .refusal();
    assert!(
        stderr.contains("`depth_turns` as `x`"),
        "the refusal quotes the token it could not read: {stderr}"
    );
}

#[test]
fn an_engine_time_that_is_not_a_count_is_refused_before_it_reaches_arithmetic() {
    // `time` is the one engine-reported field that reaches `$(( ))`, which
    // performs command substitution on its operand: `time PIPESTATUS[$(cmd)]`
    // RAN cmd, with the record still written and exit 0 (docs/decisions.md
    // D-232). The stub's payload creates a file inside its OWN scratch directory,
    // so a regression leaves evidence rather than passing quietly — and leaves it
    // somewhere no `git add -A` can reach.
    let stub = stub_engine("snapshot-timex");
    let pwned = pwned_path(stub.parent().expect("the stub has a directory"));
    let stderr = Run::new("snapshot-timex-run", &stub)
        .mode("timex")
        .refusal();
    assert!(
        stderr.contains("`time` as `PIPESTATUS[$(>"),
        "the refusal quotes the token: {stderr}"
    );
    assert!(
        !pwned.exists(),
        "and the token never reached an arithmetic context, so nothing ran"
    );
}

#[test]
fn a_score_with_an_empty_pv_after_it_leaves_no_pv_token_in_the_record() {
    // `s/ pv.*$//` and not `s/ pv .*//`: an empty pv leaves a bare ` pv` at the
    // end of the line, which the spaced form leaves sitting in the score field.
    let stub = stub_engine("snapshot-emptypv");
    let record = Run::new("snapshot-emptypv-run", &stub)
        .mode("emptypv")
        .record();
    for line in invariant(&record).lines().filter(|l| l.contains(" score ")) {
        let score = line.split_once(" score ").expect("a score field").1;
        assert!(
            !score.contains("pv"),
            "the score field carries a score and not a leaked pv token: `{line}`"
        );
    }
}

#[test]
fn a_short_bestmove_list_is_refused_rather_than_left_empty_in_a_record() {
    // One `bestmove` fewer than there are positions left `… bestmove ` with an
    // empty value at the end of an invariant line.
    let stub = stub_engine("snapshot-shortbest");
    let stderr = Run::new("snapshot-shortbest-run", &stub)
        .mode("shortbest")
        .refusal();
    assert!(
        stderr.contains("bestmove lines for 2 positions"),
        "the refusal counts what it got against what it needed: {stderr}"
    );
}

#[test]
fn an_engine_that_fails_the_corpus_session_is_refused_by_name() {
    let stub = stub_engine("snapshot-corpusrc");
    let stderr = Run::new("snapshot-corpusrc-run", &stub)
        .mode("corpusrc")
        .refusal();
    assert!(
        stderr.contains("the engine exited 9 on the corpus session"),
        "the refusal names the status: {stderr}"
    );
}

#[test]
fn a_corpus_entry_without_a_stones_count_is_refused_by_name() {
    let dir = scratch("snapshot-bad-corpus");
    let corpus = dir.join("corpus.txt");
    std::fs::write(&corpus, "# no stone counts here\nstart moves 0,0\n")
        .expect("the test corpus writes");
    let stderr = Run::new(
        "snapshot-bad-corpus-run",
        Path::new(env!("CARGO_BIN_EXE_pistol")),
    )
    .corpus(corpus)
    .no_out()
    .refusal();
    assert!(
        stderr.contains("entry without a stones count"),
        "and the refusal quotes the entry: {stderr}"
    );
}

#[test]
fn a_crlf_corpus_is_refused_rather_than_put_a_carriage_return_in_the_record() {
    // `[ 15$'\r' -le 17 ]` is ACCEPTED by bash, so a CRLF corpus put a bare CR
    // mid-line in the invariant block — where two records differing only in a
    // corpus's line endings look identical in any diff viewer (D-230).
    let dir = scratch("snapshot-crlf-corpus");
    let (early, late) = band_entries();
    let corpus = dir.join("corpus.txt");
    std::fs::write(&corpus, format!("# crlf\r\n{early}\r\n{late}\r\n"))
        .expect("the test corpus writes");
    let stderr = Run::new(
        "snapshot-crlf-corpus-run",
        Path::new(env!("CARGO_BIN_EXE_pistol")),
    )
    .corpus(corpus)
    .no_out()
    .refusal();
    assert!(
        stderr.contains("CRLF line endings"),
        "the refusal names the line endings: {stderr}"
    );
}

#[test]
fn a_corpus_missing_a_band_is_refused_because_the_ladder_names_both() {
    // The ladder takes one rung per BAND by name, so a corpus with an empty band
    // has no `late_mid` to name — and the shipped record used to call a
    // 15-stone position `late_mid` three lines below labelling it `early`.
    let dir = scratch("snapshot-one-band");
    let (early, late) = band_entries();
    for (name, body, wanted) in [
        (
            "early-only.txt",
            format!("# early only\n{early}\n"),
            "no LATE position",
        ),
        (
            "late-only.txt",
            format!("# late only\n{late}\n"),
            "no EARLY position",
        ),
    ] {
        let corpus = dir.join(name);
        std::fs::write(&corpus, body).expect("the test corpus writes");
        let stderr = Run::new(
            "snapshot-one-band-run",
            Path::new(env!("CARGO_BIN_EXE_pistol")),
        )
        .corpus(corpus)
        .no_out()
        .refusal();
        assert!(
            stderr.contains(wanted),
            "the refusal names the band that is missing (`{wanted}`): {stderr}"
        );
    }
}

#[test]
fn a_corpus_name_carrying_a_newline_cannot_write_lines_into_the_record() {
    // The corpus BASENAME reaches the invariant block, so a name containing a
    // newline injected attacker-chosen LINES into it — a forged
    // `ladder_terminal` and a forged `position`, with exit 0 and the COMPLETE
    // kind token (REPRODUCED, docs/decisions.md D-232).
    let dir = scratch("snapshot-forged-name");
    let (early, late) = band_entries();
    let forged = dir.join("mini.txt\nladder_terminal opening reached 99 mate\nx.txt");
    std::fs::write(&forged, format!("# forged\n{early}\n{late}\n"))
        .expect("the test corpus writes");
    let stderr = Run::new(
        "snapshot-forged-name-run",
        Path::new(env!("CARGO_BIN_EXE_pistol")),
    )
    .corpus(forged)
    .no_out()
    .refusal();
    assert!(
        stderr.contains("outside printable ASCII in its file name"),
        "the refusal names what it found: {stderr}"
    );
}

#[test]
fn an_empty_flag_value_is_refused_rather_than_silently_defaulted() {
    // `--out ''` fell back to stdout in silence — the skip-with-default CLAUDE.md
    // rule 3 forbids, in the flag whose whole job is to say where the record goes.
    let stub = stub_engine("snapshot-empty-flag");
    let dir = scratch("snapshot-empty-flag-run");
    let corpus = two_band_corpus(&dir);
    let config = repo("configs/instrument_v0.toml");
    let ran = Command::new("bash")
        .arg(repo("tools/baseline_snapshot.sh"))
        .args(["--config", config.to_str().expect("utf-8 path")])
        .args(["--corpus", corpus.to_str().expect("utf-8 path")])
        .args(["--ladder-depth", "1"])
        .args(["--binary", stub.to_str().expect("utf-8 path")])
        .args(["--out", ""])
        .current_dir(repo_root())
        .output()
        .expect("the snapshot script runs");
    let stderr = String::from_utf8_lossy(&ran.stderr);
    assert!(
        !ran.status.success(),
        "an empty value is refused, not defaulted:\n{}",
        String::from_utf8_lossy(&ran.stdout)
    );
    assert!(
        stderr.contains("--out was given an empty value"),
        "and the refusal names the flag: {stderr}"
    );
}

#[test]
fn a_budget_spelling_the_engine_would_read_differently_is_refused() {
    // `[ 010 -ge 1 ]` is true because bash reads a leading zero as OCTAL, while
    // the engine parses the same token as decimal 10 and the invariant block
    // quotes `010`; ` 50000` puts a DOUBLE SPACE inside an invariant line.
    let stub = stub_engine("snapshot-budget-spelling");
    for spelling in ["010", "+50000", " 50000", "0"] {
        let stderr = Run::new("snapshot-budget-spelling-run", &stub)
            .nodes(spelling)
            .refusal();
        assert!(
            stderr.contains("--nodes takes a positive integer"),
            "`{spelling}` is refused by name: {stderr}"
        );
    }
    // And the registered spelling still passes, or this proves only that the
    // guard refuses everything.
    let record = Run::new("snapshot-budget-spelling-ok", &stub)
        .nodes("50000")
        .record();
    assert!(
        invariant(&record).contains("budget nodes 50000 OVERRIDE"),
        "an explicit budget is recorded as an override:\n{}",
        invariant(&record)
    );
}

#[test]
fn an_engine_this_script_cannot_read_is_refused_rather_than_digested_as_nothing() {
    // `echo "binary_sha256 $(sha256sum "$BINARY" | ...)"` DISCARDS the
    // substitution's status, because it is an argument: an engine that is
    // executable and not readable (mode 0111) wrote the line with nothing after
    // it, exited 0, and carried the COMPLETE kind token. `binary_sha256` is the
    // one line separating a debug-build record from a release one, so two such
    // records were BYTE-IDENTICAL in their whole invariant block — REPRODUCED at
    // ccba146 with two engines whose digests differ.
    let dir = scratch("engine-unreadable");
    let engine = dir.join("engine");
    std::fs::copy(env!("CARGO_BIN_EXE_pistol"), &engine).expect("the engine copies");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&engine, std::fs::Permissions::from_mode(0o111))
            .expect("the copy is executable and not readable");
    }
    let run = Run::new("engine-unreadable-run", &engine);
    let stderr = run.refusal();
    assert!(
        stderr.contains("cannot read the engine at"),
        "the refusal names the engine it could not digest: {stderr}"
    );
    assert!(
        stderr.contains("states nothing about which bytes ran"),
        "and says why a missing digest is fatal to a record: {stderr}"
    );
    assert!(
        !run.out.as_ref().expect("this run names a record").exists(),
        "and no record is written at all"
    );
}

#[test]
fn a_corpus_name_carrying_a_unicode_separator_is_refused_like_an_ascii_newline() {
    // The sibling of the newline case, and the one the guard MISSED: U+2028 and
    // U+0085 are control characters by every Unicode reading, but the guard was
    // `[[:cntrl:]]` under the `LC_ALL=C` the script pins, and that class is
    // ASCII — so the ASCII newline was refused while these two walked into the
    // record's invariant block (REPRODUCED at ccba146, both writing a `corpus`
    // line and exiting 0 with the COMPLETE kind token).
    let (early, late) = band_entries();
    for (label, name) in [
        ("u2028", "mini\u{2028}sep.txt"),
        ("u0085", "mini\u{0085}nel.txt"),
    ] {
        let dir = scratch(&format!("snapshot-unicode-name-{label}"));
        let corpus = dir.join(name);
        std::fs::write(&corpus, format!("# unicode\n{early}\n{late}\n"))
            .expect("the test corpus writes");
        let stderr = Run::new(
            &format!("snapshot-unicode-name-run-{label}"),
            Path::new(env!("CARGO_BIN_EXE_pistol")),
        )
        .corpus(corpus)
        .no_out()
        .refusal();
        assert!(
            stderr.contains("outside printable ASCII in its file name"),
            "{label}: the refusal names what it found: {stderr}"
        );
    }
}

#[test]
fn the_invariant_block_states_the_ladder_cap_the_kind_token_depends_on() {
    // D-232's argument for NOT giving the cap a provenance token of its own is
    // that the cap's value is in the block for a reader to see. That argument
    // needs the line to exist, and nothing checked that it did: a mutant
    // deleting it survived the whole suite. The value is a NON-DEFAULT one, so a
    // mutant nailing the line to a constant dies here too.
    let record = Run::new(
        "snapshot-ladder-cap-line",
        Path::new(env!("CARGO_BIN_EXE_pistol")),
    )
    .ladder_cap_s("25")
    .record();
    let block = invariant(&record);
    assert!(
        block.lines().any(|line| line == "ladder_cap_s 25"),
        "the invariant block states the cap the kind token depends on:\n{block}"
    );
}

#[test]
fn a_ladder_that_reports_a_shallower_depth_than_one_already_recorded_is_refused() {
    // Iterative deepening only goes deeper. An engine re-reporting a shallower
    // depth leaves `last` below depths the block ALREADY STATES, so the record
    // contradicts itself one line after stating them — and the refusal written
    // for it was unbound: a mutant deleting the comparison survived.
    let stub = stub_engine("snapshot-ladder-backward");
    let stderr = Run::new("snapshot-ladder-backward-run", &stub)
        .ladder_depth("2")
        .mode("backward")
        .refusal();
    assert!(
        stderr.contains("reported depth 1 after depth 2"),
        "the refusal names both depths: {stderr}"
    );
}

/// A RELATIVE `--out` BELONGS TO THE CALLER'S DIRECTORY, NOT THE REPOSITORY ROOT.
///
/// This script `cd`s to `$ROOT` before it does anything, and a `cd` silently
/// redefines what every relative path the caller supplied means. MEASURED before
/// the fix: `--out relative_probe.txt` issued from `/tmp` wrote its record into
/// the repository root — a file the caller never asked for, in a tree whose
/// cleanliness other gates then adjudicate on, and one directory away from the
/// `SUBJECT_PATH` defect that deleted from the same tree for the same reason
/// (tools/SHELL_CHECKLIST.md item 11).
///
/// The stray is removed before the assertion fires, so a regression fails loudly
/// without leaving the repository dirty for every later gate in the run.
#[test]
fn a_relative_out_lands_in_the_callers_directory_and_not_the_repository_root() {
    let dir = scratch("relative-out");
    let corpus = two_band_corpus(&dir);
    let caller = dir.join("caller");
    std::fs::create_dir_all(&caller).expect("the caller's directory is created");

    let config = repo("configs/instrument_v0.toml");
    let ran = Command::new("bash")
        .arg(repo("tools/baseline_snapshot.sh"))
        .args(["--config", config.to_str().expect("utf-8 path")])
        .args(["--corpus", corpus.to_str().expect("utf-8 path")])
        .args(["--ladder-depth", "1"])
        .args(["--binary", env!("CARGO_BIN_EXE_pistol")])
        .args(["--out", "record.txt"])
        .current_dir(&caller)
        .output()
        .expect("bash runs the shipped script");
    assert!(
        ran.status.success(),
        "the run must succeed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );

    let in_root = repo("record.txt");
    let leaked = in_root.is_file();
    if leaked {
        let _ = std::fs::remove_file(&in_root);
    }
    assert!(
        !leaked,
        "a relative --out must not write into the repository root"
    );
    assert!(
        caller.join("record.txt").is_file(),
        "and it must write into the directory the caller was standing in"
    );
}

/// A SPACE IN A CORPUS FILE NAME SHIFTS EVERY FIELD AFTER IT ON ITS OWN RECORD
/// LINE, AND THE PRINTABLE-ASCII GUARD ADMITTED IT BY CONSTRUCTION.
///
/// `[:print:]` includes the space — in `LC_ALL=C` and in every other locale — so
/// the sibling guard that refuses U+2028 and an ASCII newline let this through by
/// definition rather than by oversight. The record is whitespace-token-delimited
/// with nothing quoted, so REPRODUCED at 369d43a against the shipped script:
/// `--corpus '/…/mini corpus.txt'` wrote
/// `corpus mini corpus.txt sha256 70b3402… positions 2`, exit 0, under the
/// COMPLETE kind token, and a reader taking the digest from the line's fourth
/// token got the literal string `sha256` (docs/decisions.md D-324).
///
/// Refused rather than supported: making it work is a change to the record SCHEMA
/// and to every reader of it, where refusing is one guard.
///
/// The CONTROL is the second half. Only the BASENAME reaches the record, so a
/// corpus inside a SPACED DIRECTORY must still produce one — without it this test
/// would pass just as well against a guard that refused everything, which
/// tools/SHELL_CHECKLIST.md item 10 names as the way a coverage test comes to
/// prove nothing.
#[test]
fn a_corpus_name_carrying_a_space_is_refused_rather_than_shift_the_records_fields() {
    let (early, late) = band_entries();
    let body = format!("# spaced\n{early}\n{late}\n");

    let dir = scratch("snapshot-spaced-name");
    let spaced = dir.join("mini corpus.txt");
    std::fs::write(&spaced, &body).expect("the test corpus writes");
    let run = Run::new(
        "snapshot-spaced-name-run",
        Path::new(env!("CARGO_BIN_EXE_pistol")),
    )
    .corpus(spaced)
    .no_out();
    let ran = run.go();
    let stderr =
        String::from_utf8_lossy(&ran.stdout).into_owned() + &String::from_utf8_lossy(&ran.stderr);
    // The CODE and not merely `!success` (tools/SHELL_CHECKLIST.md item 12
    // obligation 3): this script has no void class, so 1 is its only refusal
    // code. A 0 is the defect itself — the record was written with its fields
    // shifted. Anything else is bash or a signal killing it somewhere it has no
    // named refusal for, which is a different finding and must not be read as
    // this one.
    assert_eq!(
        ran.status.code(),
        Some(1),
        "a spaced corpus name is refused with the script's only refusal code; \
         0 would mean the record was written with its `corpus` fields shifted, \
         and any other code that it died without a named refusal:\n{stderr}"
    );
    assert!(
        stderr.contains("has a SPACE in its file name"),
        "the refusal names what it found: {stderr}"
    );

    // The control: the same space, one directory up, where it never reaches the
    // record. This must still be a complete record naming the unspaced basename.
    let held = scratch("snapshot-spaced-dir").join("corpus dir");
    std::fs::create_dir_all(&held).expect("the spaced directory is created");
    let inside = held.join("mini.txt");
    std::fs::write(&inside, &body).expect("the test corpus writes");
    let record = Run::new(
        "snapshot-spaced-dir-run",
        Path::new(env!("CARGO_BIN_EXE_pistol")),
    )
    .corpus(inside)
    .record();
    let block = invariant(&record);
    let corpus_line = block
        .lines()
        .find(|line| line.starts_with("corpus "))
        .unwrap_or_else(|| panic!("the invariant block states a corpus line:\n{block}"));
    let fields: Vec<&str> = corpus_line.split_whitespace().collect();
    assert_eq!(
        fields.get(1).copied(),
        Some("mini.txt"),
        "a spaced DIRECTORY is not a spaced name and still runs: {corpus_line}"
    );
    assert_eq!(
        fields.get(2).copied(),
        Some("sha256"),
        "and the digest keyword still sits where the record's own parse rule puts it: {corpus_line}"
    );
    assert!(
        fields.get(3).is_some_and(
            |digest| digest.len() == 64 && digest.chars().all(|c| c.is_ascii_hexdigit())
        ),
        "so the fourth token is the digest and not a shifted keyword: {corpus_line}"
    );
}

/// Run the shipped script FROM A DIRECTORY THAT IS NOT THE REPOSITORY ROOT, with
/// the arguments given exactly as written — the only shape in which the caller's
/// base and the repository root are distinguishable at all.
fn from_directory(dir: &Path, args: &[&str]) -> Output {
    Command::new("bash")
        .arg(repo("tools/baseline_snapshot.sh"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the snapshot script runs")
}

#[test]
fn a_relative_out_and_a_relative_corpus_resolve_from_the_same_base() {
    // ONE BASE, AND IT IS THE CALLER'S. `--out` was resolved against the caller's
    // directory and `--corpus` and `--binary` against the repository root, one
    // flag apart, so the same relative word meant two different files depending
    // on which flag it followed. This is the control run as well as the claim: it
    // SUCCEEDS, so a guard that refused everything could not produce it.
    let dir = scratch("snapshot-one-base");
    let stub = stub_engine_in(&dir, "stub-engine");
    two_band_corpus(&dir);
    let config = repo("configs/instrument_v0.toml");
    let ran = from_directory(
        &dir,
        &[
            "--config",
            config.to_str().expect("utf-8 path"),
            "--corpus",
            "./corpus.txt",
            "--binary",
            "./stub-engine",
            "--ladder-depth",
            "1",
            "--out",
            "./record.txt",
        ],
    );
    assert!(
        ran.status.success(),
        "every relative argument names a file in the caller's own directory:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
    let record = std::fs::read_to_string(dir.join("record.txt"))
        .expect("the record is written beside the caller, not into the repository root");
    assert!(
        !repo("record.txt").exists(),
        "and nothing of that name is written into the repository root"
    );
    let block = invariant(&record);
    let corpus_line = block
        .lines()
        .find(|line| line.starts_with("corpus "))
        .unwrap_or_else(|| panic!("the invariant block states a corpus line:\n{block}"));
    let stub_digest =
        pistol_cli::sha256::sha256_hex(&std::fs::read(&stub).expect("the stub engine reads"));
    let corpus_digest = pistol_cli::sha256::sha256_hex(
        &std::fs::read(dir.join("corpus.txt")).expect("the caller's corpus reads"),
    );
    assert!(
        corpus_line.contains(&corpus_digest),
        "the corpus digested is the caller's file: {corpus_line}"
    );
    assert!(
        block.contains(&format!("binary_sha256 {stub_digest}")),
        "and so is the engine:\n{block}"
    );
}

#[test]
fn a_relative_input_that_exists_only_under_the_repository_root_is_refused_not_silently_read() {
    // The base CHANGED for `--corpus` and `--binary`, and a caller who relied on
    // the old root-relative reading must be TOLD (CLAUDE.md rule 3), not quietly
    // redirected to a file they did not name. The refusal states both paths.
    let dir = scratch("snapshot-root-only-input");
    let stub = stub_engine_in(&dir, "stub-engine");
    let ran = from_directory(
        &dir,
        &[
            "--corpus",
            "crates/pistol-cli/tests/fixtures/bench_positions_v1.txt",
            "--binary",
            stub.to_str().expect("utf-8 path"),
            "--ladder-depth",
            "1",
        ],
    );
    let stderr = String::from_utf8_lossy(&ran.stderr);
    assert!(
        !ran.status.success(),
        "this is a refusal, not a record:\n{}",
        String::from_utf8_lossy(&ran.stdout)
    );
    assert!(
        stderr.contains("--corpus") && stderr.contains("It DOES exist at"),
        "and the refusal names the flag and the path it did NOT read: {stderr}"
    );
    assert!(
        stderr.contains(dir.to_str().expect("utf-8 path")),
        "and states the base it did resolve against: {stderr}"
    );
}

#[test]
fn a_relative_input_naming_two_different_files_is_refused_as_ambiguous() {
    // Both readings exist and disagree about which bytes the record would attest.
    // Choosing either silently is how a digest line comes to name a file the
    // caller never meant, so the script refuses and says so.
    let dir = scratch("snapshot-ambiguous-input");
    let stub = stub_engine_in(&dir, "stub-engine");
    let twin = dir.join("crates/pistol-cli/tests/fixtures");
    std::fs::create_dir_all(&twin).expect("the twin path is created");
    std::fs::write(twin.join("bench_positions_v1.txt"), "# not the fixture\n")
        .expect("the twin corpus writes");
    let ran = from_directory(
        &dir,
        &[
            "--corpus",
            "crates/pistol-cli/tests/fixtures/bench_positions_v1.txt",
            "--binary",
            stub.to_str().expect("utf-8 path"),
            "--ladder-depth",
            "1",
        ],
    );
    let stderr = String::from_utf8_lossy(&ran.stderr);
    assert!(
        !ran.status.success(),
        "this is a refusal, not a record:\n{}",
        String::from_utf8_lossy(&ran.stdout)
    );
    assert!(
        stderr.contains("is AMBIGUOUS"),
        "and the refusal says which defect it is: {stderr}"
    );
    assert!(
        stderr.contains(twin.to_str().expect("utf-8 path"))
            && stderr.contains(
                repo("crates/pistol-cli/tests/fixtures/bench_positions_v1.txt")
                    .to_str()
                    .expect("utf-8 path")
            ),
        "and names BOTH readings, since naming one is what it refuses to do: {stderr}"
    );
}

#[test]
fn the_usage_text_states_the_resolution_base_and_the_exit_status_classes() {
    // The usage text is the HOME of the resolution claim (docs/decisions.md
    // D-331). Before it existed the rule was stated in a comment beside one
    // flag's implementation and was true of that flag alone.
    let dir = scratch("snapshot-usage");
    let ran = from_directory(&dir, &["--help"]);
    let stdout = String::from_utf8_lossy(&ran.stdout);
    assert!(ran.status.success(), "--help is not a refusal");
    assert!(
        stdout.contains("THE DIRECTORY YOU RAN THIS SCRIPT FROM"),
        "the usage text states the base: {stdout}"
    );
    assert!(
        stdout.contains("no VOID class") && stdout.contains("exit 1"),
        "and answers SHELL_CHECKLIST item 12 by name: {stdout}"
    );
    assert!(
        stdout.contains("--config is REQUIRED and has NO DEFAULT"),
        "N-E's item-12 sentence names the config refusal by name too: {stdout}"
    );
}

/// N-E's FIRST refusal class (docs/decisions.md D-329): `--config` is
/// required, no default, no code-side fallback.
#[test]
fn a_missing_config_flag_is_refused_as_required() {
    let stub = stub_engine("snapshot-no-config");
    let stderr = Run::new("snapshot-no-config-run", &stub)
        .no_config()
        .refusal();
    assert!(
        stderr.contains("--config is required"),
        "the refusal names the missing flag: {stderr}"
    );
    assert!(
        stderr.contains("no default"),
        "and says why an inherited value is not an option: {stderr}"
    );
}

/// N-E's SECOND refusal class: the WHOLE config path is guarded, not only its
/// basename — `configs/spaced dir/instrument_v0.toml` is D-329's own named
/// example of what the basename-loop spelling would miss, since the space
/// sits in a DIRECTORY component the basename never sees.
#[test]
fn a_config_path_with_a_space_in_a_directory_component_is_refused() {
    let dir = scratch("snapshot-config-space");
    let spaced = dir.join("spaced dir");
    std::fs::create_dir_all(&spaced).expect("the spaced directory is created");
    let config = spaced.join("instrument_v0.toml");
    std::fs::copy(repo("configs/instrument_v0.toml"), &config)
        .expect("the committed config copies");

    let stub = stub_engine("snapshot-config-space");
    let stderr = Run::new("snapshot-config-space-run", &stub)
        .config(config)
        .refusal();
    assert!(
        stderr.contains("has a SPACE"),
        "the refusal names what it found: {stderr}"
    );
    assert!(
        stderr.contains("config path"),
        "and names which flag's value it was: {stderr}"
    );
}

/// The control, paired with the test above: the IDENTICAL setup — a fresh
/// scratch directory, a copied config, a `Run` built the same way — with the
/// one difference the claim is about (no space in the directory component)
/// removed. It succeeds, so the refusal above is provably about the space and
/// not about "a config outside the default location" in general
/// (tools/SHELL_CHECKLIST.md item 10's coverage rule: a pass must not come
/// from a guard that refuses everything).
#[test]
fn the_same_config_path_shape_without_a_space_is_accepted() {
    let dir = scratch("snapshot-config-space-control");
    let unspaced = dir.join("unspaced_dir");
    std::fs::create_dir_all(&unspaced).expect("the directory is created");
    let config = unspaced.join("instrument_v0.toml");
    std::fs::copy(repo("configs/instrument_v0.toml"), &config)
        .expect("the committed config copies");

    let stub = stub_engine("snapshot-config-space-control");
    let record = Run::new("snapshot-config-space-control-run", &stub)
        .config(config)
        .mode("follow")
        .record();
    assert!(
        invariant(&record).contains("config "),
        "the same shape of path, without the space, is accepted:\n{}",
        invariant(&record)
    );
}
