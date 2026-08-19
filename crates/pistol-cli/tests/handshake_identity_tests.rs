//! The handshake identifies the eval weight table by CONTENT.
//!
//! Two configs differing only in `eval.weights_file` used to produce
//! byte-identical arena identities while `nelo_pair` moved by 98 points — the
//! provenance hole WP-1.3 recorded (docs/decisions.md D-188). The engine now
//! digests the weights file it loads and says so in its own handshake
//! (docs/decisions.md D-198), so the claim has to be tested against the real
//! binary: the digest is assembled in the binary's entry point, not in the
//! library the in-process tests drive.

mod common;

use std::io::Write;
use std::process::{Command, Stdio};

use common::repo_root;
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
