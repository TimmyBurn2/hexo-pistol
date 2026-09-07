mod common;

use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Output};

use common::{repo, scratch};

/// A PATH whose first entry is `dir`, so a seeded name shadows the real tool.
fn run(dir: Option<&Path>, name: &str) -> Output {
    let mut command = Command::new("bash");
    command.arg(repo("tools/require_tool.sh")).arg(name);
    if let Some(dir) = dir {
        let inherited = std::env::var("PATH").unwrap_or_default();
        command.env("PATH", format!("{}:{inherited}", dir.display()));
    }
    command.output().expect("the shipped script starts")
}

fn seeded(case: &str) -> std::path::PathBuf {
    let dir = scratch(case).join("bin");
    std::fs::create_dir_all(&dir).expect("the seed directory");
    dir
}

fn executable(path: &Path) {
    let mut mode = std::fs::metadata(path).expect("it exists").permissions();
    mode.set_mode(0o755);
    std::fs::set_permissions(path, mode).expect("the bit is set");
}

fn said(ran: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    )
}

/// THE CONTROL. Every refusal below is satisfied by a resolver that refuses
/// everything, which is `tools/SHELL_CHECKLIST.md` item 10's named failure mode.
#[test]
fn a_real_program_resolves_to_its_absolute_path() {
    let ran = run(None, "cargo");
    assert_eq!(
        ran.status.code(),
        Some(0),
        "cargo is on this machine's PATH: {}",
        said(&ran)
    );
    let out = String::from_utf8_lossy(&ran.stdout).trim().to_owned();
    assert!(
        Path::new(&out).is_absolute() && Path::new(&out).is_file(),
        "and what comes back is an absolute path to a real file: `{out}`"
    );
}

/// The five outcomes `command -v` conflates, each refused for its OWN reason.
///
/// `command -v` DECLINES an unfindable name and a directory identically, and
/// ACCEPTS a FIFO, a non-executable file, and a function or alias — the last
/// returning the NAME rather than a path. MEASURED on bash 5.3.15; the last two
/// are the fourth and fifth cases WP-1.10 left open.
#[test]
fn each_way_a_name_can_fail_to_be_a_program_is_refused_on_its_own_terms() {
    let dir = seeded("require_tool_shapes");

    let real = dir.join("shapes_real");
    std::fs::write(&real, "#!/bin/sh\nexit 0\n").expect("writes");
    executable(&real);
    assert_eq!(
        run(Some(&dir), "shapes_real").status.code(),
        Some(0),
        "the control: a real program in the seeded directory resolves"
    );

    std::fs::create_dir_all(dir.join("shapes_dir")).expect("a directory");
    let plain = dir.join("shapes_plain");
    std::fs::write(&plain, "not executable\n").expect("writes");
    let mut mode = std::fs::metadata(&plain).expect("it exists").permissions();
    mode.set_mode(0o644);
    std::fs::set_permissions(&plain, mode).expect("the bit is cleared");

    for (name, expected) in [
        ("shapes_absent", "no `shapes_absent` on PATH"),
        ("shapes_dir", "names a DIRECTORY at"),
        ("shapes_plain", "carries no execute bit"),
    ] {
        let ran = run(Some(&dir), name);
        assert_eq!(
            ran.status.code(),
            Some(2),
            "`{name}` is not a runnable program: {}",
            said(&ran)
        );
        assert!(
            said(&ran).contains(expected),
            "and the refusal says WHICH way, not merely that it failed — wanted \
             `{expected}` in: {}",
            said(&ran)
        );
    }
}

/// A FIFO is the case that does not fail — it HANGS — so the refusal has to
/// come before anything reads it.
#[test]
fn a_fifo_on_path_is_refused_rather_than_read() {
    let dir = seeded("require_tool_fifo");
    let fifo = dir.join("shapes_fifo");
    let made = Command::new("mkfifo")
        .arg(&fifo)
        .status()
        .expect("mkfifo runs");
    assert!(made.success(), "the fixture needs a FIFO");

    let ran = run(Some(&dir), "shapes_fifo");
    assert_eq!(
        ran.status.code(),
        Some(2),
        "a FIFO is not a program: {}",
        said(&ran)
    );
    assert!(
        said(&ran).contains("not a regular file"),
        "and it is refused as one, before any read of it can block: {}",
        said(&ran)
    );
}

/// A function or alias shadowing the name is the case `command -v` answers
/// with the NAME instead of a path, so a gate checking for a tool runs
/// something that is not that tool.
#[test]
fn a_shell_function_shadowing_the_name_is_refused_as_not_a_program() {
    let ran = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "shapes_fn() {{ echo shadowed; }}; export -f shapes_fn; bash {} shapes_fn",
            repo("tools/require_tool.sh").display()
        ))
        .output()
        .expect("bash runs");
    assert_eq!(
        ran.status.code(),
        Some(2),
        "an exported function is not a program on PATH: {}",
        said(&ran)
    );
    assert!(
        said(&ran).contains("resolves to a shell function"),
        "and the refusal names the kind, which is what tells an operator where \
         to look: {}",
        said(&ran)
    );
}

/// The caller's own mistakes are a different exit code from the tool's absence,
/// so a gate cannot mistake "I called this wrong" for "the tool is missing".
#[test]
fn a_caller_error_is_exit_one_and_a_missing_tool_is_exit_two() {
    for args in [vec![], vec!["a", "b"], vec!["/usr/bin/cargo"], vec![""]] {
        let mut command = Command::new("bash");
        command.arg(repo("tools/require_tool.sh")).args(&args);
        let ran = command.output().expect("the shipped script starts");
        assert_eq!(
            ran.status.code(),
            Some(1),
            "`{args:?}` is the caller calling this wrong, not a missing tool: {}",
            said(&ran)
        );
    }
}
