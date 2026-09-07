mod common;

use std::process::{Command, Output};

use common::{repo, scratch};

/// Assert the gate's exit code, in a message that says what the OTHER codes
/// would have meant.
///
/// `tools/SHELL_CHECKLIST.md` item 12 obligation 3: a test that drives a gate
/// asserts the code it expects AND says what the others mean, because
/// `assert!(status.success())` reports a VOID as a regression. That is this
/// gate's own history — `tools/ci.sh` in a worktree with `CARGO_TARGET_DIR`
/// exported went red here with `RUN VOID: no binary at
/// target/release/solver-selftest`, indistinguishable in the log from the
/// solver having lost determinism (docs/decisions.md D-672, D-281, D-285).
fn assert_code(ran: &Output, want: i32, what: &str) {
    let got = ran.status.code();
    if got == Some(want) {
        return;
    }
    let meaning = match got {
        Some(0) => "0 — the two runs agree",
        Some(1) => "1 — they do not, or a run refused: a REGRESSION in the solver",
        Some(2) => "2 — RUN VOID: the question could not be asked, which is NOT a regression",
        _ => "a code this gate does not define, or a signal",
    };
    panic!(
        "{what}: expected exit {want}, got {got:?} ({meaning})\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
}

fn run_gate(target_dir: Option<&std::path::Path>, shadowed_cargo: Option<&str>) -> Output {
    let mut command = Command::new("bash");
    command.arg(repo("tools/solver_determinism.sh"));
    if let Some(dir) = target_dir {
        command.env("CARGO_TARGET_DIR", dir);
    }
    if let Some(body) = shadowed_cargo {
        // A stub named `cargo` FIRST on PATH, rather than a PATH with cargo
        // removed: where the real one lives is machine-dependent (rustup's
        // `~/.cargo/bin` here, a distribution's `/usr/bin` elsewhere), and a
        // driver that depends on that answers differently on two machines.
        // Shadowing depends on nothing.
        let bin = scratch("solver_determinism_stub_cargo").join("bin");
        std::fs::create_dir_all(&bin).expect("the stub directory");
        let stub = bin.join("cargo");
        std::fs::write(&stub, body).expect("the stub is written");
        set_executable(&stub);
        let inherited = std::env::var("PATH").unwrap_or_default();
        command.env("PATH", format!("{}:{inherited}", bin.display()));
    }
    command.output().expect("the shipped script starts")
}

fn set_executable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut mode = std::fs::metadata(path)
        .expect("the stub exists")
        .permissions();
    mode.set_mode(0o755);
    std::fs::set_permissions(path, mode).expect("the stub is executable");
}

#[test]
fn the_shipped_solver_determinism_script_passes_and_says_so() {
    let ran = run_gate(None, None);
    assert_code(&ran, 0, "the committed solver is determinism-clean");
    let stdout = String::from_utf8_lossy(&ran.stdout);
    // Positive content: a pass that printed nothing proved nothing. The
    // script names the case count and the byte-identical verdict itself.
    let pass_line = stdout
        .lines()
        .find(|line| line.starts_with("solver_determinism: PASS"))
        .unwrap_or_else(|| panic!("no PASS line in the script's stdout:\n{stdout}"));
    assert!(
        pass_line.contains("byte-identical transcripts"),
        "the PASS line must say what was compared: {pass_line}"
    );
    assert!(
        pass_line.contains("61 cases"),
        "the PASS line must name the case count it actually ran: {pass_line}"
    );
}

/// D-672's own reproducer, as a test: the gate runs the binary CARGO named,
/// so a redirected target directory is a PASS and not a void.
///
/// Against the path literal this replaced, both outcomes were wrong and this
/// test would have caught either: in a tree holding no earlier build the path
/// is absent and the gate voids on a subject that compiled fine, and in a tree
/// that does hold one the path RESOLVES to a binary this run did not build
/// (docs/decisions.md D-250's class, reproduced both ways before the fix).
#[test]
fn a_redirected_target_directory_is_a_pass_and_not_a_void() {
    let target = scratch("solver_determinism_redirected").join("target");
    let ran = run_gate(Some(&target), None);
    assert_code(
        &ran,
        0,
        "the gate must run the binary cargo named, wherever cargo put it",
    );
    let stdout = String::from_utf8_lossy(&ran.stdout);
    assert!(
        stdout.contains("solver_determinism: PASS"),
        "a redirected target directory must not change the verdict:\n{stdout}"
    );
    assert!(
        target.join("release/solver-selftest").is_file(),
        "the build went to the redirected directory, so that is where the gate had to look"
    );
}

/// The other half of item 12: a question that cannot be asked exits 2 and says
/// RUN VOID, spelled differently from a disagreement.
#[test]
fn a_build_that_refuses_is_a_void_and_not_a_failure() {
    let ran = run_gate(
        None,
        Some("#!/bin/sh\necho 'error: could not compile' >&2\nexit 101\n"),
    );
    assert_code(&ran, 2, "a refused build means no answer was taken");
    let stderr = String::from_utf8_lossy(&ran.stderr);
    assert!(
        stderr.contains("solver_determinism: RUN VOID"),
        "a void names itself as one: {stderr}"
    );
    assert!(
        !stderr.contains("solver_determinism: FAIL"),
        "and it must not also spell itself a failure: {stderr}"
    );
}

/// The resolution ladder itself, driven: a green build that names no
/// executable is a void naming THAT, not a determinism verdict.
///
/// This is the case the replaced path literal could not distinguish. It saw an
/// absent file and could not say whether cargo had built nothing, built it
/// somewhere else, or built it here yesterday.
#[test]
fn a_green_build_that_names_no_executable_is_a_void_naming_that() {
    let ran = run_gate(
        None,
        Some("#!/bin/sh\necho '{\"reason\":\"compiler-artifact\",\"executable\":null}'\nexit 0\n"),
    );
    assert_code(&ran, 2, "a build that produced no binary took no answer");
    let stderr = String::from_utf8_lossy(&ran.stderr);
    assert!(
        stderr.contains("cargo built no executable for --bin solver-selftest"),
        "the void names what was missing, in this gate's vocabulary: {stderr}"
    );
}

/// The gate ASKS FOR ITS SCRATCH SPACE BEFORE THE BUILD, and the ORDER is what
/// this asserts.
///
/// `tools/SHELL_CHECKLIST.md` item 12 obligation 2. Two things are needed and
/// the first version of this test had only one. PRESENCE: without an assertion
/// the preflight CALL is invisible, because a preflight that passes is a no-op
/// and deleting it leaves the gate green (docs/decisions.md D-553's
/// call-removed mutant). ORDER: this gate's preflight sat BELOW the build, so
/// on a full filesystem it voided with cargo's words rather than its own — the
/// D-281 reading item 12 exists to close — and a test that reads only presence
/// cannot see that. It is checked by OFFSET in the gate's own output, so the
/// block cannot drift back down without this going red.
#[test]
fn the_gate_asks_for_its_scratch_before_it_builds_anything() {
    let ran = run_gate(None, None);
    let out = String::from_utf8_lossy(&ran.stdout);
    let combined = format!("{out}{}", String::from_utf8_lossy(&ran.stderr));
    let preflight = combined
        .find("scratch_preflight:")
        .unwrap_or_else(|| panic!("the gate must ask for room at all:\n{combined}"));
    assert!(
        combined.contains("KiB available"),
        "and name what it found:\n{combined}"
    );
    // `Compiling`/`Finished` is cargo's first word about the build. On a warm
    // tree only `Finished` appears, so both are searched and the earliest wins.
    let build = ["Compiling", "Finished", "Blocking"]
        .iter()
        .filter_map(|word| combined.find(word))
        .min();
    if let Some(build) = build {
        assert!(
            preflight < build,
            "the preflight must come BEFORE the build it protects, or a full \
             filesystem answers in cargo's vocabulary:\n{combined}"
        );
    }
}
