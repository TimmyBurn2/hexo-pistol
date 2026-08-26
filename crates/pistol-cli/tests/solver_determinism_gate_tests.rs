//! `tools/solver_determinism.sh` — CI gate 13 of 18 (`tools/ci.sh`), the
//! solver's determinism seat (WP-1.8a, D-7's law gaining a solver seat).
//!
//! D-289's rule: any `tools/` script that produces a recorded number carries
//! at least one test driving the SHIPPED script. This is that test — it does
//! not simulate the script's logic; it runs the script itself, against the
//! committed fixture and config, and asserts both the exit status and the
//! PASS line's positive content. The companion gate
//! `tools/solver_oracle_check.sh` is driven by CI itself (gate 12 of 18,
//! minutes of release CPU) and its driving test is the registered debt
//! recorded in the WP-1.8a closure: this file covers the cheap seat, the
//! oracle leg is covered by the CI gate that runs it every push.
//!
//! The script's own failure modes are exercised by mutation in the WP's
//! receipt table (artifacts/wp18a_mutant_*.log); what this test pins is that
//! the SHIPPED script, unmodified, passes and says so in its own words.

use std::path::PathBuf;
use std::process::Command;

fn repo(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

#[test]
fn the_shipped_solver_determinism_script_passes_and_says_so() {
    let output = Command::new("bash")
        .arg(repo("tools/solver_determinism.sh"))
        .output()
        .expect("the shipped script starts");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "the shipped script must exit 0; stderr:\n{stderr}"
    );
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
