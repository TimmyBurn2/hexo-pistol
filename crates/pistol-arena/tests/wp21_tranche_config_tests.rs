mod common;

use std::path::PathBuf;
use std::process::{Command, Output};

use common::{Scratch, repo};

/// The shipped script, driven as a program (docs/process.md's tools/ coverage
/// rule: a script that produces a recorded artefact carries a test driving the
/// script itself, not a re-implementation of it).
fn generate(scratch: &Scratch, tranche: &str, name: &str, sha: &str) -> (Output, PathBuf) {
    let out = scratch.path(name);
    let output = Command::new("python3")
        .arg(repo().join("tools/wp21_tranche_config.py"))
        .arg("--tranche")
        .arg(tranche)
        .arg("--out")
        .arg(&out)
        .arg("--binary-sha256")
        .arg(sha)
        .output()
        .expect("python3 runs the generator");
    (output, out)
}

const SHA: &str = "180b4c406b225fc81342bb8218b8546dda1ffac1a99f7eb91cdaf73d20253476";
const TRANCHES: u32 = 16;
/// The generator's own exit codes, and a REFUSAL is not a VOID.
///
/// `!status.success()` accepts both, so a test written to prove the script said
/// "the answer is NO" would pass on a script that said "no answer was taken" —
/// an unwritable `--out`, a full filesystem. The distinction is the script's and
/// it has to survive the seam that reads it (tools/SHELL_CHECKLIST.md item 12).
const REFUSED: i32 = 1;
const VOID: i32 = 2;

/// Assert one refusal, by its code and not merely by its failure.
fn refused(output: &Output, what: &str) {
    let code = output.status.code();
    assert_eq!(
        code,
        Some(REFUSED),
        "{what}: expected exit {REFUSED} (a named refusal — the answer is NO). Exit {VOID} \
         would have meant the run was VOID and no answer was taken; exit 0 would have meant \
         the document was written. Got {code:?}. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// The value of one `key = value` line of the generated document.
fn value(text: &str, key: &str) -> String {
    text.lines()
        .find(|line| line.starts_with(&format!("{key} = ")))
        .unwrap_or_else(|| panic!("no `{key}` line in:\n{text}"))
        .split_once(" = ")
        .expect("a key = value line")
        .1
        .trim_matches('"')
        .to_string()
}

#[test]
fn the_sixteen_tranches_partition_the_books_unconsumed_range_exactly() {
    // THE CLAIM THE WHOLE SWEEP RESTS ON: no opening is labelled twice and none
    // is skipped. A gap would be a corpus quietly smaller than its registration
    // says; an overlap would be a corpus counting one position as two, which is
    // the over-count docs/decisions.md D-537's floor exists to prevent.
    let scratch = Scratch::new("wp21-partition");
    let mut expected_skip: u32 = 13;
    let mut total: u32 = 0;
    for tranche in 1..=TRANCHES {
        let (output, out) = generate(
            &scratch,
            &tranche.to_string(),
            &format!("tranche-{tranche}.toml"),
            SHA,
        );
        assert!(
            output.status.success(),
            "tranche {tranche} was refused: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = std::fs::read_to_string(&out).expect("the config was written");
        let skip: u32 = value(&text, "openings_skip").parse().expect("a count");
        let take: u32 = value(&text, "openings_take").parse().expect("a count");
        assert_eq!(
            skip,
            expected_skip,
            "tranche {tranche} starts where {} ended",
            tranche - 1
        );
        assert!(take > 0, "tranche {tranche} takes nothing");
        expected_skip = skip + take;
        total += take;
    }
    assert_eq!(
        total, 3487,
        "the sixteen tranches take the whole SWEPT range — the book's 4,500 openings less the \
         pilot's 13 and the 1,000-opening holdout"
    );
    assert_eq!(
        expected_skip, 3500,
        "the last tranche ends where the holdout begins"
    );
}

#[test]
fn no_tranche_reaches_the_reserved_holdout() {
    // D-568 reserves the LAST 1,000 openings of the book for governed runs and
    // forbids labelling them. The test above pins the partition's arithmetic;
    // this one pins what the arithmetic is FOR, so a successor who widens the
    // range to reclaim a thousand openings meets a named refusal rather than a
    // corpus that quietly consumed the Stage-3 detector's slice.
    const HOLDOUT_FIRST: u32 = 3_500;
    let scratch = Scratch::new("wp21-holdout");
    for tranche in 1..=TRANCHES {
        let (output, out) = generate(
            &scratch,
            &tranche.to_string(),
            &format!("tranche-{tranche}.toml"),
            SHA,
        );
        assert!(output.status.success());
        let text = std::fs::read_to_string(&out).expect("the config was written");
        let skip: u32 = value(&text, "openings_skip").parse().expect("a count");
        let take: u32 = value(&text, "openings_take").parse().expect("a count");
        assert!(
            skip + take <= HOLDOUT_FIRST,
            "tranche {tranche} labels openings {}..{}, which reaches into the holdout that \
             begins at {HOLDOUT_FIRST}",
            skip,
            skip + take - 1
        );
    }
}

#[test]
fn a_generated_config_carries_the_seat_the_registration_fixes() {
    let scratch = Scratch::new("wp21-seat");
    let (output, out) = generate(&scratch, "1", "tranche-1.toml", SHA);
    assert!(output.status.success());
    let text = std::fs::read_to_string(&out).expect("the config was written");
    for (key, want) in [
        ("schema_version", "2"),
        ("turn_cap", "40"),
        ("n_workers", "1"),
        ("hang_timeout_ms", "120000"),
        ("kind", "nodes"),
        ("value", "50000"),
        ("config", "configs/instrument_v0.toml"),
        ("binary", "target/release/pistol"),
        ("binary_sha256", SHA),
    ] {
        assert_eq!(value(&text, key), want, "`{key}` in:\n{text}");
    }
    assert!(
        text.contains(
            "openings_file = \"crates/pistol-cli/tests/fixtures/random_openings_v2.txt\""
        ),
        "the successor book and no other: {text}"
    );
    assert!(
        text.contains("CARRIES NO CENSUS"),
        "the document says on its own face that it starts no clock: {text}"
    );
}

#[test]
fn a_tranche_outside_the_partition_is_refused_by_name() {
    let scratch = Scratch::new("wp21-range");
    for bad in ["0", "17", "-1"] {
        let (output, out) = generate(&scratch, bad, &format!("never-{bad}.toml"), SHA);
        refused(&output, &format!("tranche {bad}"));
        assert!(!out.exists(), "tranche {bad} wrote a document anyway");
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        assert!(
            stderr.contains("REFUSED") && stderr.contains(bad),
            "the refusal names the argument: {stderr}"
        );
    }
}

#[test]
fn a_tranche_number_spelled_a_way_this_program_will_not_echo_back_is_refused() {
    let scratch = Scratch::new("wp21-spelling");
    let (output, out) = generate(&scratch, "01", "never.toml", SHA);
    refused(&output, "the tranche number `01`");
    assert!(!out.exists());
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(
        stderr.contains("will not echo back"),
        "a config named by a spelling the log cannot reproduce: {stderr}"
    );
}

#[test]
fn a_binary_digest_that_is_not_one_is_refused_before_anything_is_written() {
    let scratch = Scratch::new("wp21-digest");
    for bad in ["", "deadbeef", &SHA.to_uppercase(), &format!("{SHA}0")] {
        let (output, out) = generate(&scratch, "1", "never.toml", bad);
        refused(&output, &format!("the digest `{bad}`"));
        assert!(!out.exists(), "`{bad}` wrote a document anyway");
    }
}

#[test]
fn an_existing_out_path_is_refused_rather_than_rewritten() {
    // A config rewritten under a live run is a document that drifted from the
    // run reading it (docs/decisions.md D-199).
    let scratch = Scratch::new("wp21-exists");
    let (first, out) = generate(&scratch, "1", "tranche-1.toml", SHA);
    assert!(first.status.success());
    let before = std::fs::read_to_string(&out).expect("the first write");
    let (second, _) = generate(&scratch, "2", "tranche-1.toml", SHA);
    refused(&second, "a second write to an existing --out");
    assert_eq!(
        std::fs::read_to_string(&out).expect("still there"),
        before,
        "the existing document was changed"
    );
    let stderr = String::from_utf8_lossy(&second.stderr).to_string();
    assert!(stderr.contains("already exists"), "{stderr}");
}

#[test]
fn the_printed_digest_is_the_digest_of_the_bytes_that_were_written() {
    // The run log records this line and nothing re-derives it there, so a
    // digest of anything but the file would be a manifest naming bytes nobody
    // holds (CLAUDE.md rule 8).
    let scratch = Scratch::new("wp21-receipt");
    let (output, out) = generate(&scratch, "9", "tranche-9.toml", SHA);
    assert!(output.status.success());
    let printed = String::from_utf8_lossy(&output.stdout).to_string();
    let claimed = printed
        .split_whitespace()
        .last()
        .expect("the line ends with the digest")
        .to_string();
    let bytes = std::fs::read(&out).expect("the config was written");
    assert_eq!(claimed, pistol_cli::sha256::sha256_hex(&bytes));
    assert!(
        printed.contains("openings_skip 1757 openings_take 218"),
        "the line states the slice it wrote: {printed}"
    );
}

#[test]
fn an_unwritable_out_directory_is_a_void_and_not_a_refusal() {
    // ITEM 12'S OWN POINT, DRIVEN. The suite has named `VOID = 2` since it was
    // written and no test ever made the script take that path, so the class a
    // reader is most likely to misread as a regression — "no answer was taken"
    // — was defended by a constant and a comment. A read-only directory is the
    // cheapest way to reach it, and the assertion is that the exit is 2 and not
    // 1: a REFUSAL says the answer is NO, and there is no answer here.
    let scratch = Scratch::new("wp21-void");
    let locked = scratch.path("locked");
    std::fs::create_dir_all(&locked).expect("the scratch directory is made");
    let mut permissions = std::fs::metadata(&locked)
        .expect("the directory exists")
        .permissions();
    permissions.set_readonly(true);
    std::fs::set_permissions(&locked, permissions).expect("the directory is made read-only");

    let out = locked.join("tranche-1.toml");
    let output = Command::new("python3")
        .arg(repo().join("tools/wp21_tranche_config.py"))
        .arg("--tranche")
        .arg("1")
        .arg("--out")
        .arg(&out)
        .arg("--binary-sha256")
        .arg(SHA)
        .output()
        .expect("python3 runs the generator");

    // Restored before the assertion, so a failing assertion still leaves a
    // scratch tree the harness can sweep (tools/SHELL_CHECKLIST.md item 11).
    let mut restore = std::fs::metadata(&locked)
        .expect("the directory exists")
        .permissions();
    #[allow(clippy::permissions_set_readonly_false)]
    restore.set_readonly(false);
    std::fs::set_permissions(&locked, restore).expect("the directory is writable again");

    let code = output.status.code();
    assert_eq!(
        code,
        Some(VOID),
        "an unwritable --out must be VOID (exit {VOID}), not a refusal (exit {REFUSED}): \
         no answer was taken and none was written. Got {code:?}. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("VOID"),
        "the void says so on its own line: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !out.exists(),
        "a voided run wrote a config anyway: {}",
        out.display()
    );
}
