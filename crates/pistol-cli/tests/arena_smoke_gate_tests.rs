//! `tools/arena_smoke.sh` — CI gate 12 of 14 (`tools/ci.sh:131`), the SPRT
//! instrument itself run end to end as a self-match.
//!
//! # The defect these tests pin
//!
//! The gate resolved its engine from cargo's artifact stream, validated it with
//! four refusals, and then NEVER READ IT AGAIN. The arena takes its engines from
//! the config's `binary = ` lines, so all three runs played the literal
//! `target/release/pistol` whatever cargo had just built and wherever it built
//! it: with `CARGO_TARGET_DIR` redirected, every engine invocation went to a
//! stale binary and the gate printed its verdict and exited 0. The variable was
//! validated and unread, which is the shape worth remembering — a guard can be
//! correct and reach nothing (docs/decisions.md D-252).
//!
//! # Why a STUB arena and a STUB engine
//!
//! The claim is about WHICH BINARY PLAYS, not about the engine or the arena. It
//! needs a `cargo build` whose target directory the test controls, and binaries
//! that can say which file they are; pointing either at the live checkout would
//! have a test redirect the builds of the repository it is being reviewed in.
//! The scratch tree holds the SHIPPED script and the SHIPPED config — the
//! config's `binary = ` spelling is what the rewrite matches on, so a change to
//! it has to break this test — over a zero-dependency workspace whose `--bin
//! pistol` and `--bin arena` stand in for the two the gate builds.
//!
//! The stub arena LAUNCHES the binary its config names and each stub engine logs
//! the file it is, so the assertion is about processes that really ran rather
//! than about a string this gate rewrote.
//!
//! # RULE9-JUSTIFICATION: one gate's choice of binary, over one scratch tree.
//! The two stubs are not a second subject: they are the fixture the claim needs,
//! since an assertion about WHICH binary played can only be made by binaries
//! that report which one they are, and splitting them out would put the fixture
//! and the assertion it exists for in different files.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

/// A stub `pistol`. It records WHICH FILE ran — the whole assertion — and then
/// answers the way the stub arena reads a seat: exit 0 is a game played,
/// nonzero is a forfeit. `EXIT_CODE` is replaced per build.
const ENGINE: &str = r#"use std::io::Write;

fn main() {
    if let Ok(path) = std::env::var("PISTOL_STUB_LOG") {
        let exe = std::env::current_exe().expect("the stub engine knows its own path");
        if let Ok(mut log) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(log, "{}", exe.display());
        }
    }
    println!("stub engine");
    std::process::exit(EXIT_CODE);
}
"#;

/// A stub `arena`: it reads the seats out of the config it is handed, launches
/// each one, and writes the report the gate asserts against. Every field the
/// gate reads is here and nothing else is.
const ARENA: &str = r##"fn field(text: &str, key: &str) -> String {
    text.lines()
        .filter_map(|line| line.strip_prefix(key))
        .filter_map(|rest| rest.split_whitespace().next())
        .map(|word| word.trim_matches('"').to_string())
        .next()
        .unwrap_or_default()
}

fn main() {
    let words: Vec<String> = std::env::args().skip(1).collect();
    assert_eq!(words[0], "--config", "the gate calls `--config C --out O`");
    assert_eq!(words[2], "--out", "the gate calls `--config C --out O`");
    let text = std::fs::read_to_string(&words[1]).expect("the config reads");
    let take: usize = field(&text, "openings_take =")
        .parse()
        .expect("openings_take is a number");
    let workers = field(&text, "n_workers =");
    let seats: Vec<String> = text
        .lines()
        .filter_map(|line| line.strip_prefix("binary = "))
        .map(|rest| rest.trim().trim_matches('"').to_string())
        .collect();
    // THE SEAT'S CONTENT BINDING, read out of the document the same way. The
    // real arena digests the file at `binary` and refuses a mismatch before it
    // spawns anything; this stub cannot hash without a dependency, so it carries
    // the DECLARED digest into the record the gate reads. That is enough for the
    // gate's own claim, which is that the document it wrote named the binary
    // cargo built — by path AND by content — and that the process at that path
    // is the one that ran.
    let digests: Vec<String> = text
        .lines()
        .filter_map(|line| line.strip_prefix("binary_sha256 = "))
        .map(|rest| rest.trim().trim_matches('"').to_string())
        .collect();
    assert_eq!(
        digests.len(),
        seats.len(),
        "every seat is bound by content or none is"
    );

    let mut report = format!("arena_report {take}\n");
    let mut forfeits = 0;
    for ((label, binary), digest) in ["a", "b"].iter().zip(seats.iter()).zip(digests.iter()) {
        // LAUNCH IT. A report that merely echoed the path would assert a string
        // rewrite; this asserts a process.
        let played = std::process::Command::new(binary).arg("--id").output();
        if !played.map(|out| out.status.success()).unwrap_or(false) {
            forfeits += 1;
        }
        report.push_str(&format!(
            "engine {label} label {label} binary {binary} binary_sha256 {digest} config stub\n"
        ));
    }
    for game in 0..take * 2 {
        report.push_str(&format!("game {game} nodes_a 1 nodes_b 1 depth_a 1 depth_b 1\n"));
    }
    report.push_str(&format!(
        "counts n {} distinct_n {take} forfeits {forfeits}\n",
        take * 2
    ));
    report.push_str(&format!("pentanomial p0 0 p1 0 p2 {take} p3 0 p4 0\n"));
    report.push_str("verdict inconclusive_degenerate\nverdict_unit pair\n");
    // Everything above the marker is the verdict block the gate compares byte
    // for byte across its three runs; the worker count belongs below it.
    report.push_str(&format!("# timing\ntiming n_workers {workers} wall_ms 0\n"));
    std::fs::write(&words[3], report).expect("the report writes");
}
"##;

/// The member that carries both bins the gate builds.
const ENGINE_MANIFEST: &str = r#"[package]
name = "pistol-arena-smoke-stub"
version = "0.0.0"
edition = "2021"

[[bin]]
name = "pistol"
path = "src/pistol.rs"

[[bin]]
name = "arena"
path = "src/arena.rs"
"#;

/// A SECOND member declaring a bin of the same name, which is what makes cargo
/// name two executables `pistol` for one `--bin pistol`.
const TWIN_MANIFEST: &str = r#"[package]
name = "pistol-arena-smoke-twin"
version = "0.0.0"
edition = "2021"

[[bin]]
name = "pistol"
path = "src/pistol.rs"
"#;

/// Write the stub engine with a given exit status, replacing whatever was there.
fn write_engine(root: &Path, exit_code: &str) {
    std::fs::write(
        root.join("engine/src/pistol.rs"),
        ENGINE.replace("EXIT_CODE", exit_code),
    )
    .expect("the stub engine writes");
}

/// The workspace manifest over these members.
fn write_workspace(root: &Path, members: &str) {
    std::fs::write(
        root.join("Cargo.toml"),
        format!("[workspace]\nmembers = [{members}]\nresolver = \"2\"\n"),
    )
    .expect("the workspace manifest writes");
    // The gate builds `--locked`, so the lockfile has to match the members.
    let ran = Command::new(env!("CARGO"))
        .current_dir(root)
        .args(["generate-lockfile", "--offline", "-q"])
        .output()
        .expect("cargo runs");
    assert!(
        ran.status.success(),
        "the stub workspace takes a lockfile: {}",
        String::from_utf8_lossy(&ran.stderr)
    );
}

/// A tree holding the shipped script, the shipped config, and a stub workspace
/// for the gate to build.
fn scratch_tree(name: &str) -> PathBuf {
    let root = scratch(name).join("repo");
    for dir in ["tools", "configs", "engine/src"] {
        std::fs::create_dir_all(root.join(dir)).expect("the scratch tree is created");
    }
    std::fs::copy(
        repo("tools/arena_smoke.sh"),
        root.join("tools/arena_smoke.sh"),
    )
    .expect("the shipped gate copies");
    // The gate preflights its scratch space through a SIBLING script and refuses
    // by name when it is absent, so the scratch tree carries it too.
    std::fs::copy(
        repo("tools/scratch_preflight.sh"),
        root.join("tools/scratch_preflight.sh"),
    )
    .expect("the sibling preflight copies");
    std::fs::copy(
        repo("configs/arena_smoke_v0.toml"),
        root.join("configs/arena_smoke_v0.toml"),
    )
    .expect("the shipped config copies");
    std::fs::write(root.join("engine/Cargo.toml"), ENGINE_MANIFEST).expect("the manifest writes");
    std::fs::write(root.join("engine/src/arena.rs"), ARENA).expect("the stub arena writes");
    write_engine(&root, "0");
    write_workspace(&root, "\"engine\"");
    root
}

/// Add the twin member and re-lock, so `--bin pistol` names two executables.
fn add_twin(root: &Path) {
    std::fs::create_dir_all(root.join("twin/src")).expect("the twin member is created");
    std::fs::write(root.join("twin/Cargo.toml"), TWIN_MANIFEST).expect("the twin manifest writes");
    std::fs::write(
        root.join("twin/src/pistol.rs"),
        ENGINE.replace("EXIT_CODE", "0"),
    )
    .expect("the twin's bin writes");
    write_workspace(root, "\"engine\", \"twin\"");
}

/// Run the SHIPPED gate in `root`, saying where cargo is to build.
///
/// The target directory is NAMED rather than removed from the environment:
/// `env_remove` covers the ENVIRONMENT layer only, and a user-level
/// `~/.cargo/config.toml` carrying `[build] target-dir` is a different one that
/// would move the build out from under this test for a reason unrelated to the
/// code under test. Measured here: an environment `CARGO_TARGET_DIR` beats a
/// config-file `[build] target-dir`, so naming it pins every layer.
fn arena_smoke(root: &Path, target_dir: &Path, log: &Path) -> Output {
    Command::new("bash")
        .arg(root.join("tools/arena_smoke.sh"))
        .current_dir(root)
        .env("CARGO_TARGET_DIR", target_dir)
        .env_remove("CARGO_BUILD_TARGET")
        .env("PISTOL_STUB_LOG", log)
        .output()
        .expect("the gate script runs")
}

/// Everything the run said: the gate prints its verdict on stdout and its
/// refusals on stderr, and a failure wants both.
fn said(ran: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    )
}

/// Every engine invocation the run made, as the engines themselves recorded it.
fn invocations(log: &Path) -> Vec<PathBuf> {
    std::fs::read_to_string(log)
        .expect("the invocation log reads")
        .lines()
        .map(PathBuf::from)
        .collect()
}

#[test]
fn arena_smoke_plays_the_binary_cargo_built_and_not_a_stale_one_at_the_configs_literal_path() {
    let root = scratch_tree("arena-smoke-stale");
    let log = root.join("invocations.log");
    let default_target = root.join("target");

    // THE CONTROL RUN. Nothing is wrong and the gate has to PASS — otherwise the
    // refusal below could come from a gate that refuses everything, which is the
    // pass-for-the-wrong-reason `tools/SHELL_CHECKLIST.md`'s coverage rule names.
    std::fs::write(&log, "").expect("the log starts empty");
    let control = arena_smoke(&root, &default_target, &log);
    let seen = said(&control);
    assert!(
        control.status.success(),
        "the gate passes when nothing is wrong:\n{seen}"
    );
    assert!(
        seen.contains("verdict inconclusive_degenerate"),
        "and it reads the self-match's knowable verdict off the report:\n{seen}"
    );

    // The control's own engine is now the STALE one, at exactly the path the
    // CONFIG names. If it is not there the test would pass for the wrong reason:
    // there would be no staleness left to be fooled by.
    let stale = default_target.join("release/pistol");
    assert!(
        stale.is_file(),
        "the control build lands where `binary = \"target/release/pistol\"` points — a \
         cargo-config `[build] target` would move it into a per-triple subdirectory, and \
         this assertion is where that shows up: {}",
        stale.display()
    );
    assert!(
        Command::new(&stale)
            .arg("--id")
            .output()
            .expect("the stale binary runs")
            .status
            .success(),
        "and the stale binary still plays a clean game, which is what makes it a trap"
    );

    // THE REGRESSION: the engine now forfeits every game it is handed, and cargo
    // is told to build somewhere else. The gate must play the binary it BUILT.
    write_engine(&root, "1");
    let alt = scratch("arena-smoke-stale-alt");
    std::fs::write(&log, "").expect("the log is cleared for the regression");
    let ran = arena_smoke(&root, &alt, &log);
    let out = said(&ran);
    let built = alt.join("release/pistol");
    assert!(
        built.is_file(),
        "cargo built into the redirected directory:\n{out}"
    );
    assert!(
        stale.is_file(),
        "and the stale engine is still sitting at the config's literal path:\n{out}"
    );

    // WHICH BINARY PLAYED, from the engines' own record of having been launched.
    let played = invocations(&log);
    let stale_real = stale.canonicalize().expect("the stale binary resolves");
    let built_real = built.canonicalize().expect("the built binary resolves");
    let stale_plays = played.iter().filter(|path| **path == stale_real).count();
    assert!(
        played.contains(&built_real),
        "the binary cargo BUILT is the one that played:\n{out}"
    );
    assert_eq!(
        stale_plays,
        0,
        "{stale_plays} of {} engine invocations went to the stale binary at the config's \
         literal path:\n{out}",
        played.len()
    );

    assert!(
        !ran.status.success(),
        "and a self-match its own engine forfeits is a refusal, not an exit-0 pass:\n{out}"
    );
    assert!(
        out.contains("forfeited a game"),
        "named by the reason the report actually gave:\n{out}"
    );
}

/// THE SECOND HALF OF THE SEAT. The arena binds each engine by CONTENT and
/// refuses a `binary` whose digest is not the one the document names
/// (docs/decisions.md D-283), so this gate has to rewrite `binary_sha256`
/// beside `binary` — rewriting the path alone leaves a document that refuses
/// every run, and rewriting neither leaves the defect D-252 reproduced. Both
/// rewrites are counted by the gate, and a document that binds one seat and not
/// the other is refused rather than half-bound.
#[test]
fn arena_smoke_refuses_a_config_that_does_not_bind_every_seat_by_content() {
    let root = scratch_tree("arena-smoke-digest");
    let log = root.join("invocations.log");
    let target = root.join("target");

    // THE CONTROL RUN, with the shipped config as committed.
    std::fs::write(&log, "").expect("the log starts empty");
    let control = arena_smoke(&root, &target, &log);
    let seen = said(&control);
    assert!(
        control.status.success(),
        "the gate passes on the shipped config:\n{seen}"
    );
    // The rewrite is REPORTED, and what it reports is the digest of the file
    // cargo built rather than the one committed in the document.
    let built = root.join("target/release/pistol");
    let bytes = std::fs::read(&built).expect("the built engine reads");
    let digest = pistol_cli::sha256::sha256_hex(&bytes);
    assert!(
        seen.contains(&digest),
        "the gate names the digest of the binary it built:\n{seen}"
    );

    // Now the document binds one seat only. The gate must refuse rather than
    // rewrite half of it.
    let config = root.join("configs/arena_smoke_v0.toml");
    let text = std::fs::read_to_string(&config).expect("the shipped config reads");
    let half: String = {
        let mut seen_one = false;
        text.lines()
            .filter(|line| {
                if line.starts_with("binary_sha256 = ") && !seen_one {
                    seen_one = true;
                    return false;
                }
                true
            })
            .map(|line| format!("{line}\n"))
            .collect()
    };
    assert_ne!(half, text, "one digest line was removed");
    std::fs::write(&config, &half).expect("the half-bound config writes");

    let ran = arena_smoke(&root, &target, &log);
    let out = said(&ran);
    assert!(
        !ran.status.success(),
        "a half-bound document is a refusal, not a run:\n{out}"
    );
    assert!(
        out.contains("every seat is bound by content or none is"),
        "named for what it is:\n{out}"
    );
}

#[test]
fn arena_smoke_refuses_when_the_file_name_does_not_choose_one_executable() {
    let root = scratch_tree("arena-smoke-twin");
    let log = root.join("invocations.log");
    let target = root.join("target");

    // THE CONTROL RUN, before there is a twin to be confused by.
    std::fs::write(&log, "").expect("the log starts empty");
    let control = arena_smoke(&root, &target, &log);
    assert!(
        control.status.success(),
        "the gate passes when one executable answers to each name:\n{}",
        said(&control)
    );

    // Two members declaring `[[bin]] name = "pistol"` make cargo name two
    // executables for one `--bin pistol` and exit 0. The gate selects its two
    // bins by FILE NAME, and a `case` that selects by name is last-one-wins.
    add_twin(&root);
    let ran = arena_smoke(&root, &target, &log);
    let out = said(&ran);
    assert!(
        !ran.status.success(),
        "two executables named `pistol` is a refusal, not a silent pick:\n{out}"
    );
    assert!(
        out.contains("executables whose file name is `pistol`"),
        "named for what it is:\n{out}"
    );
    // ONE REFUSAL PER REASON (item 8): not the build's, and not the match's.
    assert!(
        !out.contains("do not build"),
        "and not blamed on the build, which succeeded:\n{out}"
    );
    assert!(
        !out.contains("forfeited a game"),
        "and not blamed on the engines, which never played:\n{out}"
    );
}
