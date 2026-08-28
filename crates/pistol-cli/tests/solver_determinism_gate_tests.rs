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
