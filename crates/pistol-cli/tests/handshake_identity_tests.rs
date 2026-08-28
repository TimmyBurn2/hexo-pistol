mod common;

use std::io::Write;
use std::process::{Command, Stdio};

use common::{repo_root, scratch};
use pistol_cli::sha256::sha256_hex;

#[test]
fn handshake_reports_weights_sha256() {
    // The committed instrument config names configs/eval_v0_weights.toml
    // relative to the working directory, which the binary's own usage text says
    // must be the directory the config was written for — the repository root.
    let root = repo_root();
    let weights = std::fs::read(root.join("configs/eval_v0_weights.toml"))
        .expect("the committed weight table is readable");
    let expected = sha256_hex(&weights);

    let mut child = Command::new(env!("CARGO_BIN_EXE_pistol"))
        .current_dir(&root)
        .args(["--config", "configs/instrument_v0.toml"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the engine binary starts");
    child
        .stdin
        .as_mut()
        .expect("a stdin pipe")
        .write_all(b"pistol\nquit\n")
        .expect("the handshake is sent");
    let output = child
        .wait_with_output()
        .expect("the engine answers and quits");
    assert!(
        output.status.success(),
        "the engine refused its own committed config: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("protocol output is text");
    let line = format!("id weights_sha256 {expected}");
    assert!(
        stdout.lines().any(|answer| answer == line),
        "the handshake names the weight table by content — expected `{line}` in:\n{stdout}"
    );
}

/// The movetime ceiling contract travels on the play-mode handshake — and only
/// there: instrument mode refuses a movetime by name, and its handshake is
/// pinned byte-for-byte against the pre-WP-1.4 revision, so an epsilon line in
/// it would be drift (WP-1.4, docs/decisions.md D-95 superseded).
#[test]
fn play_handshake_advertises_movetime_epsilon_and_instrument_does_not() {
    let root = repo_root();
    let epsilon_line = "id movetime_epsilon_ms 50";

    for (config, advertised) in [
        ("configs/play_v0.toml", true),
        ("configs/instrument_v0.toml", false),
    ] {
        let mut child = Command::new(env!("CARGO_BIN_EXE_pistol"))
            .current_dir(&root)
            .args(["--config", config])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("the engine binary starts");
        child
            .stdin
            .as_mut()
            .expect("a stdin pipe")
            .write_all(b"pistol\nquit\n")
            .expect("the handshake is sent");
        let output = child
            .wait_with_output()
            .expect("the engine answers and quits");
        assert!(
            output.status.success(),
            "the engine refused {config}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).expect("protocol output is text");
        assert_eq!(
            stdout.lines().any(|answer| answer == epsilon_line),
            advertised,
            "{config}: expected `{epsilon_line}` present = {advertised} in:\n{stdout}"
        );
    }
}

/// Say the handshake to the binary with this `--config`, and hand back what it
/// answered. Not `assert!(success)`: half of this suite's point is a refusal.
fn handshake_with(config: &std::path::Path) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_pistol"))
        .current_dir(repo_root())
        .arg("--config")
        .arg(config)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the engine binary starts");
    child
        .stdin
        .as_mut()
        .expect("a stdin pipe")
        .write_all(b"pistol\nquit\n")
        .expect("the handshake is sent");
    child
        .wait_with_output()
        .expect("the engine answers and quits")
}

/// ONE RECORD'S TWO COPIES OF ONE CONFIG PATH MUST BE THE SAME STRING.
///
/// `tools/baseline_snapshot.sh` writes both of them into the invariant block:
/// `echo "config $CONFIG $CONFIG_SHA256"` is the path RAW, and
/// `sed 's/^id /engine_id /'` is the engine's own copy of the same path out of
/// the handshake. That copy used to pass through the escape every answer passes
/// through, which rewrites a control character to `?` — so REPRODUCED at b067d47
/// against the shipped release binary, one path spelled `inst\u{1}v0.toml` gave
/// `config …/inst<0x01>v0.toml` and `engine_id config …/inst?v0.toml` in one
/// block, at exit 0, and the engine's copy named a file that does not exist
/// (docs/decisions.md D-324). Reachable only by driving the engine directly today
/// — `$CONFIG` is a constant of that script and no flag sets it — which is why
/// this test is here and not in the script's suite.
///
/// The escape is right for a refusal's prose and wrong for provenance, so the
/// path is REFUSED by name before anything is echoed, and the two copies agree by
/// construction. Asserted as the CODE `2` — what this binary's usage block calls
/// "refused before doing work" — because a `1` is a gate that ran and did not
/// hold and anything else is a death with no named reason, which would be a
/// different finding (tools/SHELL_CHECKLIST.md item 12 obligation 3).
///
/// THE CONTROL is the first half, and it is what keeps this from passing against
/// an engine that refuses every config: an admissible path must still produce the
/// line, and the two copies are compared as the two expressions that write them.
#[test]
fn the_handshakes_config_line_is_the_path_it_was_given_or_the_path_is_refused() {
    // The control: an ordinary path, spelled by both expressions.
    let ordinary = repo_root().join("configs/instrument_v0.toml");
    let output = handshake_with(&ordinary);
    assert!(
        output.status.success(),
        "the engine refused an ordinary config path: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("protocol output is text");
    let engines_copy = stdout
        .lines()
        .find_map(|line| line.strip_prefix("id config "))
        .unwrap_or_else(|| panic!("the handshake states a config line:\n{stdout}"));
    assert_eq!(
        engines_copy,
        ordinary.display().to_string(),
        "the engine's copy of the path is the script's raw copy, character for character"
    );

    // The defect: the same document, under a name the escape would rewrite.
    let held = scratch("handshake-control-character-config");
    let folded = held.join("inst\u{1}v0.toml");
    std::fs::copy(&ordinary, &folded).expect("the committed config copies under a folded name");
    let refused = handshake_with(&folded);
    let said = String::from_utf8_lossy(&refused.stdout).into_owned()
        + &String::from_utf8_lossy(&refused.stderr);
    assert_eq!(
        refused.status.code(),
        Some(2),
        "a path the handshake cannot echo verbatim is refused before any work; \
         0 is the defect itself — the record's two config lines were written \
         disagreeing — 1 would be a gate that ran, and anything else a death with \
         no named reason:\n{said}"
    );
    assert!(
        said.contains("holds a control character"),
        "the refusal names what it found: {said}"
    );
    assert!(
        !said.contains("id config "),
        "and nothing echoed a rewritten path before refusing: {said}"
    );
}
