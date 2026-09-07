mod common;

use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Output};

use common::{repo, repo_root, scratch};

/// A `cargo` shim FIRST on PATH that names `engine` on the artifact stream, and
/// answers every other cargo verb with success.
///
/// This is how a gate that resolves its binary off cargo's own artifact stream
/// (docs/decisions.md D-250) is pointed at a stub WITHOUT changing one byte of
/// the gate. The pattern is this repository's own — `solver_determinism_gate_tests`
/// shims cargo the same way — and it is what an OPTION MATRIX proposing an
/// `--engine` flag missed (`docs/experiments/matrix_gate_seam_REDTEAM.md`).
/// A flag would also BYPASS the resolution ladder D-250 exists to defend; the
/// shim drives it.
fn shim(case: &str, engine_body: &str) -> std::path::PathBuf {
    let bin = scratch(case).join("bin");
    std::fs::create_dir_all(&bin).expect("the shim directory");

    let engine = bin.join("fake_engine");
    std::fs::write(&engine, engine_body).expect("the stub engine writes");
    make_executable(&engine);

    let cargo = bin.join("cargo");
    std::fs::write(
        &cargo,
        format!(
            "#!/usr/bin/env bash\n\
             # `build` is the only verb whose OUTPUT these gates parse; every other\n\
             # verb is answered with success so the gate reaches its comparison.\n\
             case \"$1\" in\n\
             build) printf '{{\"reason\":\"compiler-artifact\",\"executable\":\"{}\"}}\\n' ;;\n\
             esac\n\
             exit 0\n",
            engine.display()
        ),
    )
    .expect("the cargo shim writes");
    make_executable(&cargo);
    bin
}

fn make_executable(path: &Path) {
    let mut mode = std::fs::metadata(path).expect("it exists").permissions();
    mode.set_mode(0o755);
    std::fs::set_permissions(path, mode).expect("the bit is set");
}

fn gate(script: &str, bin: &Path, mode: &str) -> Output {
    let inherited = std::env::var("PATH").unwrap_or_default();
    Command::new("bash")
        .arg(repo(script))
        .current_dir(repo_root())
        .env("PATH", format!("{}:{inherited}", bin.display()))
        .env("FAKE_MODE", mode)
        .output()
        .expect("the shipped gate starts")
}

fn said(ran: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    )
}

/// An engine that answers `mv`, and under `FAKE_MODE=violate` answers something
/// else on its SECOND process — which is the one thing a determinism gate exists
/// to catch and the one thing no config or fixture can seed.
const ENGINE: &str = r#"#!/usr/bin/env bash
n=$(( $(cat "${FAKE_COUNT:-/tmp/pistol-fake-count}" 2>/dev/null || echo 0) + 1 ))
echo "$n" > "${FAKE_COUNT:-/tmp/pistol-fake-count}"
mv="5,0"
if [ "${FAKE_MODE:-honest}" = violate ] && [ "$n" = 2 ]; then mv="6,0"; fi
while IFS= read -r line; do
  case "$line" in
    go*) printf 'info depth_turns 1 seldepth 1 nodes 1 hashfull 0 nps 1 time 0 score cp 0 pv %s\ninfo totals depth_turns 1 seldepth 1 nodes 1 hashfull 0 nps 1 time 0 score cp 0 pv %s\nbestmove %s\n' "$mv" "$mv" "$mv" ;;
    quit) exit 0 ;;
  esac
done
"#;

/// An engine that answers nothing at all: item 10's control clause, which asks
/// that a pass cannot come from a gate that accepts everything.
const SILENT: &str =
    "#!/usr/bin/env bash\nwhile IFS= read -r l; do case \"$l\" in quit) exit 0;; esac; done\n";

/// THE CONTROL. The gate reaches its verdict and its recorded number under the
/// shim, so every refusal below is a refusal of the SEEDED violation and not of
/// the harness.
#[test]
fn the_determinism_gate_reaches_its_recorded_number_under_an_honest_engine() {
    let bin = shim("determinism_gate_honest", ENGINE);
    let counter = bin.join("count");
    let inherited = std::env::var("PATH").unwrap_or_default();
    let ran = Command::new("bash")
        .arg(repo("tools/determinism.sh"))
        .current_dir(repo_root())
        .env("PATH", format!("{}:{inherited}", bin.display()))
        .env("FAKE_MODE", "honest")
        .env("FAKE_COUNT", &counter)
        .output()
        .expect("the shipped gate starts");
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(0),
        "an honest engine agrees:\n{out}"
    );
    assert!(
        out.contains("determinism: ok — 5 seat(s)"),
        "and the gate's own recorded number is reached:\n{out}"
    );
}

/// THE SEEDED VIOLATION. Two processes of one engine, disagreeing.
#[test]
fn two_processes_that_disagree_are_refused_and_the_seat_is_named() {
    let bin = shim("determinism_gate_violate", ENGINE);
    let counter = bin.join("count");
    let inherited = std::env::var("PATH").unwrap_or_default();
    let ran = Command::new("bash")
        .arg(repo("tools/determinism.sh"))
        .current_dir(repo_root())
        .env("PATH", format!("{}:{inherited}", bin.display()))
        .env("FAKE_MODE", "violate")
        .env("FAKE_COUNT", &counter)
        .output()
        .expect("the shipped gate starts");
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "a disagreement is the answer being NO, not a void:\n{out}"
    );
    assert!(
        out.contains("two processes disagreed on the same input"),
        "and the gate says what it found:\n{out}"
    );
    assert!(
        out.contains("determinism: FAIL: radius:"),
        "naming the seat, which is how an operator knows where to look:\n{out}"
    );
}

/// ITEM 10's CONTROL CLAUSE: an engine that does no work is refused, so a pass
/// cannot come from a gate that compares two empty transcripts and calls them
/// equal.
#[test]
fn an_engine_that_answers_nothing_is_refused_rather_than_agreeing_with_itself() {
    let bin = shim("determinism_gate_silent", SILENT);
    let ran = gate("tools/determinism.sh", &bin, "honest");
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "two empty transcripts agree, and agreeing about nothing is not a pass:\n{out}"
    );
    assert!(
        out.contains("bestmove lines carrying a turn token"),
        "and the gate refuses on CONTENT, by name:\n{out}"
    );
}

/// The movetime gate's own seeded violation: an engine that reports a wall past
/// the seat's epsilon.
#[test]
fn the_movetime_gate_refuses_an_engine_that_reports_an_overshoot() {
    let slow = "#!/usr/bin/env bash\n\
                while IFS= read -r line; do\n\
                  case \"$line\" in\n\
                    go*) printf 'info totals depth_turns 1 seldepth 1 nodes 1 hashfull 0 nps 1 time 99999 score cp 0 pv 5,0\\nbestmove 5,0\\n' ;;\n\
                    quit) exit 0 ;;\n\
                  esac\n\
                done\n";
    let bin = shim("movetime_gate_overshoot", slow);
    let ran = gate("tools/movetime_check.sh", &bin, "honest");
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "an overshoot is the answer being NO:\n{out}"
    );
    assert!(
        out.contains("movetime: FAIL:") && out.contains("bound"),
        "and the gate names the bound it was measured against:\n{out}"
    );
}
