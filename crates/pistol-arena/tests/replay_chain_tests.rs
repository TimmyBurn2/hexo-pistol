mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{ARENA, ConfigSpec, Ran, STUB, Scratch, openings_prefix, repo, run};

const OPENINGS: usize = 4;
const TURN_CAP: u32 = 12;

/// A run of two DIFFERENT deterministic engines at a replayable budget.
fn played(scratch: &Scratch) -> Ran {
    let openings = scratch.write("openings.txt", &openings_prefix(OPENINGS));
    let config_a = scratch.stub_config("chain-a.toml", "honest");
    let config_b = scratch.stub_config("chain-b.toml", "honest_last");
    let spec = ConfigSpec {
        openings: &openings,
        take: OPENINGS,
        skip: 0,
        turn_cap: TURN_CAP,
        workers: 1,
        hang_ms: 30_000,
        elo1: 4.0,
        budget_kind: "nodes",
        budget_value: 5_000,
        binary_a: STUB,
        config_a: &config_a,
        binary_b: STUB,
        config_b: &config_b,
    };
    run(scratch, &spec, "chain")
}

/// Replay a report and hand back the document's path.
fn replay(scratch: &Scratch, report: &Path, tag: &str) -> PathBuf {
    let out = scratch.path(&format!("replay-{tag}.txt"));
    let ran = Command::new(ARENA)
        .arg("--replay")
        .arg(report)
        .arg("--out")
        .arg(&out)
        .arg("--workers")
        .arg("2")
        .output()
        .expect("the arena binary runs");
    assert!(
        out.exists(),
        "the replay wrote no document for `{tag}`: {}",
        String::from_utf8_lossy(&ran.stderr)
    );
    out
}

/// Run the SHIPPED checker over two real documents.
fn check(report: &Path, replayed: &Path) -> Output {
    Command::new("python3")
        .arg(repo().join("tools/wp16_warm_attribution_check.py"))
        .arg(report)
        .arg(replayed)
        .arg(STUB)
        .output()
        .expect("python3 runs the checker")
}

fn said(output: &Output) -> String {
    format!(
        "exit {:?}\nstdout: {}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// An honest run, its own warm replay, and the checker over both: a measurement.
#[test]
fn a_real_run_and_its_real_replay_are_attributable_to_the_shipped_checker() {
    let scratch = Scratch::new("chain-honest");
    let ran = played(&scratch);
    assert_eq!(ran.code(), 0, "the run itself is clean:\n{}", ran.report());
    let report = scratch.write("report.txt", ran.report());
    let replayed = replay(&scratch, &report, "honest");

    let out = check(&report, &replayed);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the shipped checker could not read documents the shipped arena wrote. Exit 2 means \
         the two documents do not fit each other — a field the writer emits and the reader \
         does not expect, which is exactly the seam this file exists to hold: {}",
        said(&out)
    );
    let printed = String::from_utf8_lossy(&out.stdout);
    assert!(
        printed.contains("0 divergence(s), 0 confirmed inversion(s), 0 unexplained")
            && printed.contains("PASS — 0 failure(s)"),
        "{printed}"
    );
}

/// The same chain over a report whose seats are transposed and nothing else:
/// caught, and classified as an inversion by the cold probe.
#[test]
fn a_transposed_report_is_caught_by_the_shipped_checker_over_real_documents() {
    let scratch = Scratch::new("chain-swapped");
    let ran = played(&scratch);
    // Only the labels move. Every move, every result and every digest in the
    // document is the honest run's own, so nothing but a replay could tell this
    // report from an honest one.
    let swapped: String = ran
        .report()
        .split('\n')
        .map(|line| {
            if !line.starts_with("game ") {
                return line.to_string();
            }
            line.replace(" p1 a p2 b ", " p1 \u{1}b p2 \u{1}a ")
                .replace(" p1 b p2 a ", " p1 \u{1}a p2 \u{1}b ")
                .replace('\u{1}', "")
        })
        .collect::<Vec<String>>()
        .join("\n");
    let report = scratch.write("swapped.txt", &swapped);
    let replayed = replay(&scratch, &report, "swapped");

    let out = check(&report, &replayed);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a transposed report is NOT a measurement. Exit 0 would be the criterion failing to \
         see the one corruption it exists for; exit 3 would mean the cold probe could not \
         explain the recorded moves, which is a different finding: {}",
        said(&out)
    );
    let printed = String::from_utf8_lossy(&out.stdout);
    assert!(
        printed.contains("confirmed inversion(s), 0 unexplained")
            && printed.contains("the seats are the wrong way round"),
        "the corruption is classified, not merely flagged: {printed}"
    );
}

/// The checker's cold probe spells the protocol in Python; `exchange` spells it
/// in Rust. Neither can share the other, so the two spellings are pinned to
/// each other here.
///
/// A change to `position_line` that left the checker behind would leave the cold
/// probe silently asking a stale question — and the probe is what tells a
/// CONFIRMED INVERSION from a DETERMINISM VIOLATION, so a stale question there
/// mislabels the most consequential thing this instrument decides.
#[test]
fn the_checkers_cold_probe_spells_the_position_verb_the_way_exchange_does() {
    let source = std::fs::read_to_string(repo().join("tools/wp16_warm_attribution_check.py"))
        .expect("the checker is readable");
    let spelled = pistol_arena::exchange::position_line(&[]);
    assert!(
        source.contains(&format!("\"{spelled}\"")),
        "`exchange::position_line` writes `{spelled}` and the checker does not spell it that \
         way; its cold probe would ask a question no engine in this workspace answers"
    );
    let go = "go nodes ";
    assert!(
        source.contains(go),
        "the checker must spell the `go` verb as `{go}…`, the way `transcript::read` builds it"
    );
}
