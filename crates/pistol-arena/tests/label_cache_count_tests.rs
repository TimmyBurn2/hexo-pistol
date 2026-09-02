mod common;

use std::process::{Command, Output};

use common::{Scratch, repo};
use pistol_cli::sha256::sha256_hex;

/// The shipped script, driven as a program (docs/process.md's tools/ coverage
/// rule).
fn count(scratch: &Scratch, name: &str, body: &str) -> Output {
    let digest = sha256_hex(body.as_bytes());
    let capture = scratch.write(name, &format!("# param capture_format_version 1\n# body_sha256 {digest}\n{body}"));
    Command::new("python3")
        .arg(repo().join("tools/label_cache_count.py"))
        .arg("--capture")
        .arg(&capture)
        .output()
        .expect("python3 runs the counter")
}

/// A void is not an answer of zero (tools/SHELL_CHECKLIST.md item 12).
const VOID: i32 = 2;

fn said(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn record(game: u32, turns: u32, position: &str) -> String {
    format!("{game}\t{turns}\t{position}\tinfo totals depth_turns 1 nodes 1\tbestmove 0,0")
}

#[test]
fn one_position_asked_twice_is_one_miss_and_one_hit() {
    let scratch = Scratch::new("lcc-hit");
    let body = format!(
        "{}\n{}\n",
        record(0, 1, "position start moves 0,0"),
        record(1, 1, "position start moves 0,0")
    );
    let output = count(&scratch, "capture.txt", &body);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let said = said(&output);
    assert!(said.contains("asked prefixes                          2"), "{said}");
    assert!(said.contains("distinct `position` lines (cache key)   1"), "{said}");
    assert!(said.contains("cache hits                              1"), "{said}");
    assert!(said.contains("cache misses                            1"), "{said}");
}

#[test]
fn two_transposed_prefixes_are_two_cache_keys_and_one_stone_set() {
    let scratch = Scratch::new("lcc-transposition");
    // The same nine stones, the same movers, two play orders — and every pair
    // token canonical, so this is a transposition a real capture could hold
    // rather than a re-spelling one cannot. The exact key separates them and
    // the stone-set key does not, which is the measurement the registration's
    // choice of key rests on, pinned rather than argued.
    let body = format!(
        "{}\n{}\n",
        record(0, 5, "position start moves 0,0 1,0/2,0 3,0/4,0 5,0/6,0 7,0/8,0"),
        record(1, 5, "position start moves 0,0 1,0/2,0 7,0/8,0 5,0/6,0 3,0/4,0")
    );
    let output = count(&scratch, "capture.txt", &body);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let said = said(&output);
    assert!(said.contains("distinct `position` lines (cache key)   2"), "{said}");
    assert!(said.contains("distinct sorted (cell, player) lists    1"), "{said}");
}

#[test]
fn a_mirrored_position_folds_only_under_the_symmetry_column() {
    let scratch = Scratch::new("lcc-symmetry");
    // `0,0 1,0/2,0` and its reflection `0,0 0,1/0,2` are different stone sets
    // and one symmetry class, so the two coarser columns disagree by one.
    let body = format!(
        "{}\n{}\n",
        record(0, 2, "position start moves 0,0 1,0/2,0"),
        record(1, 2, "position start moves 0,0 0,1/0,2")
    );
    let output = count(&scratch, "capture.txt", &body);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let said = said(&output);
    assert!(said.contains("distinct sorted (cell, player) lists    2"), "{said}");
    assert!(said.contains("distinct symmetry-folded stone lists    1"), "{said}");
}

#[test]
fn a_capture_whose_body_does_not_digest_to_its_header_is_a_void() {
    let scratch = Scratch::new("lcc-digest");
    let capture = scratch.write(
        "capture.txt",
        "# body_sha256 0000000000000000000000000000000000000000000000000000000000000000\n\
         0\t0\tposition start\tinfo totals depth_turns 1 nodes 1\tbestmove 0,0\n",
    );
    let output = Command::new("python3")
        .arg(repo().join("tools/label_cache_count.py"))
        .arg("--capture")
        .arg(&capture)
        .output()
        .expect("python3 runs the counter");
    assert_eq!(
        output.status.code(),
        Some(VOID),
        "a body that does not digest to its header is a VOID and not an answer of zero. \
         stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn a_record_of_the_wrong_arity_is_a_void_naming_the_record() {
    let scratch = Scratch::new("lcc-arity");
    let output = count(&scratch, "capture.txt", "0\t0\tposition start\n");
    assert_eq!(output.status.code(), Some(VOID));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("TAB-separated field(s)"), "{stderr}");
}
