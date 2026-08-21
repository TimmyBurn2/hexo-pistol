//! `tools/scratch_preflight.sh` — is there room for the work, asked BEFORE the
//! work, in the gate's own vocabulary (tools/SHELL_CHECKLIST.md items 10 and 12;
//! docs/decisions.md D-285).
//!
//! # The accident this exists for
//!
//! `/tmp` on this machine is RAM-backed at 24 GiB. A session filled it, `cargo`
//! answered `Disk quota exceeded (os error 122)`, and
//! `tools/solver_link_check.sh` correctly exited 2 with `cannot build the
//! workspace's binaries` — which read, in the log and to the test driving it, as
//! a solver-link REGRESSION. The gate was honest; the vocabulary was cargo's,
//! and cargo's vocabulary describes cargo.
//!
//! # Why the floor can be raised and not lowered
//!
//! Item 10 wants a test driving the SHIPPED script with a control, and nothing
//! else can make a 24 GiB tmpfs look full. `PISTOL_MIN_SCRATCH_KIB` is combined
//! with the built-in constant by MAXIMUM, so the binding can only tighten the
//! check — a caller who sets it to zero gets the constant, and that is asserted
//! below rather than described.
//!
//! # RULE9-JUSTIFICATION: one probe, one script, one set of exit codes.

mod common;

use std::path::Path;
use std::process::{Command, Output};

use common::{repo, scratch};

/// The floor the shipped script carries, restated rather than imported: this
/// file is a CHECK on the script, and agreeing by construction proves nothing.
const MIN_SCRATCH_KIB: u64 = 1_048_576;

fn preflight(dir: &Path, raised: Option<&str>) -> Output {
    let mut command = Command::new("bash");
    command.arg(repo("tools/scratch_preflight.sh")).arg(dir);
    match raised {
        Some(value) => command.env("PISTOL_MIN_SCRATCH_KIB", value),
        None => command.env_remove("PISTOL_MIN_SCRATCH_KIB"),
    };
    command.output().expect("bash runs the shipped script")
}

fn said(ran: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    )
}

/// THE CONTROL. Without it every refusal below is satisfied by a probe that
/// refuses everything.
#[test]
fn a_directory_with_room_passes_and_prints_the_number_it_read() {
    let dir = scratch("preflight-control");
    let ran = preflight(&dir, None);
    let out = said(&ran);
    assert_eq!(ran.status.code(), Some(0), "there is room here:\n{out}");
    assert!(
        out.contains(&format!("floor {MIN_SCRATCH_KIB} KiB")),
        "the floor it applied is on the record:\n{out}"
    );
    assert!(
        out.contains("KiB available"),
        "and so is what it read:\n{out}"
    );
}

/// THE VOID, and it is spelled VOID rather than FAIL. Item 12's whole point:
/// two different things exit non-zero and a reader has to be able to tell them
/// apart without knowing which gate printed the line.
#[test]
fn a_directory_without_room_is_a_named_run_void_and_not_a_failure() {
    let dir = scratch("preflight-void");
    let ran = preflight(&dir, Some("999999999999"));
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(2),
        "a shortage is exit 2, the void class:\n{out}"
    );
    assert!(out.contains("RUN VOID"), "spelled as a void:\n{out}");
    assert!(
        out.contains("nothing was measured and nothing failed"),
        "and it says which of the two it is:\n{out}"
    );
    assert!(
        out.contains("999999999999"),
        "naming what it wanted:\n{out}"
    );
}

/// THE BINDING ONLY TIGHTENS. A value below the constant leaves the constant in
/// force, so nothing can disable this check by setting it small.
#[test]
fn a_floor_below_the_constant_does_not_lower_it() {
    let dir = scratch("preflight-tighten");
    let ran = preflight(&dir, Some("1"));
    let out = said(&ran);
    assert_eq!(ran.status.code(), Some(0), "{out}");
    assert!(
        out.contains(&format!("floor {MIN_SCRATCH_KIB} KiB")),
        "the constant is what was applied, not the 1:\n{out}"
    );
}

/// A directory that is not there is a VOID too — the question could not be
/// asked — and never a "no".
#[test]
fn a_missing_directory_is_a_void_rather_than_an_answer() {
    let dir = scratch("preflight-missing").join("nothing-here");
    let ran = preflight(&dir, None);
    let out = said(&ran);
    assert_eq!(ran.status.code(), Some(2), "{out}");
    assert!(out.contains("RUN VOID"), "{out}");
    assert!(
        out.contains("no such directory"),
        "named for what it is:\n{out}"
    );
}

/// A CALLER BUG IS A THIRD THING. Exit 1, and deliberately not the void class:
/// a script invoked wrongly has not discovered anything about the machine.
#[test]
fn calling_it_wrongly_is_exit_one_and_is_not_a_void() {
    let ran = Command::new("bash")
        .arg(repo("tools/scratch_preflight.sh"))
        .output()
        .expect("bash runs the shipped script");
    let out = said(&ran);
    assert_eq!(ran.status.code(), Some(1), "{out}");
    assert!(
        !out.contains("RUN VOID"),
        "a usage error is not a void:\n{out}"
    );
    assert!(out.contains("usage"), "{out}");
}

/// ONE SPELLING PER NUMBER (item 8): a floor that is not written in decimal is a
/// caller bug, refused before any arithmetic reads it as octal.
#[test]
fn a_floor_that_is_not_decimal_is_refused_at_the_binding() {
    let dir = scratch("preflight-spelling");
    for bad in ["", "  12", "0x10", "01048576", "1048576k"] {
        let ran = preflight(&dir, Some(bad));
        let out = said(&ran);
        if bad.is_empty() {
            // An empty binding is indistinguishable from an unset one and
            // leaves the constant in force; that is stated rather than assumed.
            assert_eq!(ran.status.code(), Some(0), "`{bad}`:\n{out}");
            continue;
        }
        assert_eq!(ran.status.code(), Some(1), "`{bad}`:\n{out}");
        assert!(
            out.contains("PISTOL_MIN_SCRATCH_KIB"),
            "`{bad}` is refused at its own binding:\n{out}"
        );
    }
}

/// THE NUMBER THE SCRIPT PRINTS AGREES WITH A REFERENT COMPUTED OUTSIDE IT.
///
/// This is the guard the suite lacked, and its absence is why a column-parsing
/// defect shipped. Every other refusal here is manufactured by raising
/// `PISTOL_MIN_SCRATCH_KIB`, which exercises the COMPARISON and never the
/// READING — so a script that read the wrong column satisfied all of them, its
/// number being a well-formed decimal from a neighbouring field.
///
/// `df` answers in columns and a mount source containing a space shifts them
/// left, turning Available into Used. Measured on a real tmpfs mounted as
/// `my dev`: an empty 2 GiB filesystem reported 0 KiB and voided a healthy run,
/// and the same filesystem with ~1 MiB left reported 2096132 KiB and PASSED.
/// The referent below is `stat -f`, which shares the DIRECTORY with the script
/// but not the parse, so a re-introduced column read moves the script's number
/// and not this one.
#[test]
fn the_printed_available_number_agrees_with_a_referent_the_script_did_not_compute() {
    let dir = scratch("preflight-referent");
    let ran = preflight(&dir, None);
    let out = said(&ran);
    assert_eq!(ran.status.code(), Some(0), "there is room here:\n{out}");

    let referent = Command::new("stat")
        .args(["-f", "-c", "%a %S"])
        .arg(&dir)
        .output()
        .expect("stat reads the filesystem");
    let fields = String::from_utf8_lossy(&referent.stdout);
    let mut parts = fields.split_whitespace();
    let blocks: u64 = parts
        .next()
        .expect("free blocks")
        .parse()
        .expect("a number");
    let block_size: u64 = parts.next().expect("block size").parse().expect("a number");
    let expected = blocks * block_size / 1024;

    let printed: u64 = out
        .split_whitespace()
        .zip(out.split_whitespace().skip(1))
        .find_map(|(value, unit)| (unit == "KiB").then(|| value.parse().ok())?)
        .unwrap_or_else(|| panic!("the script prints a KiB figure:\n{out}"));

    // A filesystem this size moves between the two reads; a column shift does
    // not move by a percent, it moves by an order of magnitude or to zero.
    let drift = printed.abs_diff(expected);
    assert!(
        drift * 100 < expected.max(1),
        "the script says {printed} KiB and stat says {expected} KiB — a \
         disagreement this large is a different FIELD, not a busy filesystem:\n{out}"
    );
}

/// A FLOOR TOO LARGE FOR THE ARITHMETIC IS A CALLER BUG, NOT A PASS.
///
/// `[ x -le y ]` above 2^63-1 is an ERROR rather than a comparison, and an
/// erroring `[` in a CONDITION is exempt from `set -e` — so the floor silently
/// stayed at the constant and the check passed, which is the guard failing open
/// in the one direction its own header says it cannot ("can only ever tighten").
/// The boundary is exact and both sides are asserted.
#[test]
fn a_floor_too_large_for_the_arithmetic_is_refused_instead_of_ignored() {
    let dir = scratch("preflight-overflow");

    let biggest = preflight(&dir, Some("9223372036854775807"));
    assert_eq!(
        biggest.status.code(),
        Some(2),
        "the largest representable floor still REFUSES, it does not error:\n{}",
        said(&biggest)
    );

    let over = preflight(&dir, Some("9223372036854775808"));
    let out = said(&over);
    assert_eq!(
        over.status.code(),
        Some(1),
        "one past it is the caller calling this wrong (1), never a pass (0):\n{out}"
    );
    assert!(
        out.contains("does not fit"),
        "the refusal names why, rather than dying under `set -e`:\n{out}"
    );
}
