//! `tools/wp18b_probe_extract.py` — the WP-1.8b anchor probe's data path
//! (docs/experiments/wp18b_anchor_probe.md §4).
//!
//! D-289's rule: any `tools/` script that produces a recorded number carries
//! at least one test driving the SHIPPED script. The probe's recorded numbers
//! (per-position verdicts, node counts) are all downstream of the position
//! list this script emits, so the test drives the script itself on stub
//! transcripts — same JSONL schema, shorter games — and pins the two things a
//! wrong extraction would silently corrupt: the case ORDER/count and the
//! stone PREFIX of each position (a prefix that leaked the current turn's own
//! stones would feed the solver edited positions, and the probe's branch rule
//! admits only unedited transcript prefixes).

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

const STUB_G1: &str = concat!(
    r#"{"a_is_p1":true,"event":"game_start","game":1,"opening":"server: p1 turn 1 at 0,0 (the platform's standard setup)"}"#,
    "\n",
    r#"{"engine":"sealbot-current","event":"turn","game":1,"mover":"p2","nodes":null,"outcome":{"kind":"continue"},"stones":[[2,-2],[1,0]],"turn":2,"wall_ms":352}"#,
    "\n",
    r#"{"engine":"pistol-e2280ca-instrument","event":"turn","game":1,"mover":"p1","nodes":50176,"outcome":{"kind":"continue"},"reply":"bestmove 0,1/0,5","stones":[[0,1],[0,5]],"turn":3,"wall_ms":141}"#,
    "\n",
    r#"{"engine":"sealbot-current","event":"turn","game":1,"mover":"p2","nodes":null,"outcome":{"first_stone_win":false,"kind":"win","turn":4,"winner":"p2"},"stones":[[0,-2],[0,-3]],"turn":4,"wall_ms":0}"#,
    "\n",
    r#"{"detail":"winner p2 at turn 4","event":"game_end","game":1,"kind":"win"}"#,
    "\n",
);

const STUB_G2: &str = concat!(
    r#"{"a_is_p1":false,"event":"game_start","game":2,"opening":"server: p1 turn 1 at 0,0 (the platform's standard setup)"}"#,
    "\n",
    r#"{"engine":"pistol-e2280ca-instrument","event":"turn","game":2,"mover":"p2","nodes":50176,"outcome":{"kind":"continue"},"reply":"bestmove 1,2/2,1","stones":[[1,2],[2,1]],"turn":2,"wall_ms":150}"#,
    "\n",
    r#"{"engine":"sealbot-current","event":"turn","game":2,"mover":"p1","nodes":null,"outcome":{"first_stone_win":false,"kind":"win","turn":3,"winner":"p1"},"stones":[[3,0],[4,0]],"turn":3,"wall_ms":0}"#,
    "\n",
    r#"{"detail":"winner p1 at turn 3","event":"game_end","game":2,"kind":"win"}"#,
    "\n",
);

#[test]
fn the_shipped_extractor_orders_counts_and_prefixes_the_stub_pair() {
    let scratch = std::env::temp_dir().join("wp18b_probe_extract_test");
    std::fs::create_dir_all(&scratch).expect("scratch dir");
    let out = scratch.join("probe");
    let _ = std::fs::remove_dir_all(&out);
    std::fs::write(scratch.join("stub_g1.jsonl"), STUB_G1).expect("stub g1");
    std::fs::write(scratch.join("stub_g2.jsonl"), STUB_G2).expect("stub g2");

    let output = Command::new("python3")
        .arg(repo("tools/wp18b_probe_extract.py"))
        .arg(&out)
        .arg(scratch.join("stub_g1.jsonl"))
        .arg(scratch.join("stub_g2.jsonl"))
        .output()
        .expect("python3 starts");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "the shipped script must exit 0; stderr:\n{stderr}"
    );
    assert!(
        stdout.contains("extracted 5 positions"),
        "the script names the position count it actually extracted: {stdout}"
    );

    // The attribution index: order = transcript order, count = one row per
    // turn event, and the running stone count is derivable by hand from the
    // stubs (origin + pairs): 1, 3, 5, then 1, 3.
    let index = std::fs::read_to_string(out.join("positions.tsv")).expect("positions.tsv");
    let rows: Vec<&str> = index.lines().skip(1).collect();
    let expected = [
        "stub_g1-t02-p2\tstub_g1\t2\tp2\tsealbot-current\t1",
        "stub_g1-t03-p1\tstub_g1\t3\tp1\tpistol-e2280ca-instrument\t3",
        "stub_g1-t04-p2\tstub_g1\t4\tp2\tsealbot-current\t5",
        "stub_g2-t02-p2\tstub_g2\t2\tp2\tpistol-e2280ca-instrument\t1",
        "stub_g2-t03-p1\tstub_g2\t3\tp1\tsealbot-current\t3",
    ];
    assert_eq!(rows, expected, "the index rows, in transcript order");

    // The prefix discipline: turn 4's fixture holds every stone placed
    // BEFORE turn 4 and not one of turn 4's own — an unedited transcript
    // prefix, which is the only thing the probe's branch rule admits.
    let fixture = std::fs::read_to_string(out.join("stub_g1-t04-p2.txt")).expect("fixture");
    assert_eq!(
        fixture, "case stub_g1-t04-p2\nplies 0,0 2,-2 1,0 0,1 0,5\nexpect nowin\n",
        "the turn-4 prefix is the origin plus turns 2 and 3, in placement order"
    );

    let _ = std::fs::remove_dir_all(&out);
    let _ = std::fs::remove_file(scratch.join("stub_g1.jsonl"));
    let _ = std::fs::remove_file(scratch.join("stub_g2.jsonl"));
    assert!(!Path::new(&out).exists(), "the test cleans up after itself");
}
