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
//! # How the refusal is watched, now that nothing can raise the floor
//!
//! The script used to read `PISTOL_MIN_SCRATCH_KIB`, and every refusal in this
//! file was manufactured by raising it. That override was INVISIBLE CONFIG — a
//! tunable living outside the one schema place, which hard rule 1 forbids — so
//! it is gone (docs/decisions.md D-306).
//!
//! Item 10 still wants a test driving the SHIPPED script with a control, so the
//! refusal is watched against a REAL filesystem too small to hold the floor: a
//! 1 MiB tmpfs mounted inside an unprivileged user + mount namespace, which is
//! the facility D-297's red-team already used to reproduce the `df` column
//! defect on a real filesystem rather than a simulated one.
//!
//! WHAT THAT DOES AND DOES NOT BUY, measured rather than asserted — the first
//! version of this comment claimed more than it had (docs/decisions.md D-308).
//! A raised floor could only ever move the number on the RIGHT of the `<`, so it
//! exercised the comparison and never the reading. The small filesystem
//! exercises the reading in ONE respect and not another:
//!
//! - IT CATCHES A WRONG DIRECTORY. A script reading some other filesystem prints
//!   this machine's `/tmp` figure against a floor of 1 GiB and passes; here it
//!   must print at most 1024 KiB. Mutation-gated: `stat … -- /tmp` in place of
//!   `-- "$DIR"` fails this test and passes the referent test below.
//! - IT DOES NOT CATCH A WRONG FIELD. MEASURED on a fresh 1 MiB tmpfs,
//!   `%a`, `%f` and `%b` are all 256 — the fields collapse, so no assertion here
//!   can separate them. `%a` → `%f` survives this whole suite, because `/tmp` is
//!   itself a tmpfs with no root reservation and the referent below reads the
//!   same directory. That residual is D-308's and is not closed here.
//!
//! # RULE9-JUSTIFICATION: one probe, one script, one set of exit codes.

mod common;

use std::path::Path;
use std::process::{Command, Output};

use common::{repo, scratch};

/// The floor the shipped script carries, restated rather than imported: this
/// file is a CHECK on the script, and agreeing by construction proves nothing.
const MIN_SCRATCH_KIB: u64 = 1_048_576;

/// Printed by the harness inside the namespace once the mount has SUCCEEDED, so
/// "this machine cannot make a small filesystem" and "the gate did not void" are
/// two different readings of a failed run rather than one.
const MOUNTED: &str = "harness: the small filesystem is mounted";

fn preflight(dir: &Path) -> Output {
    Command::new("bash")
        .arg(repo("tools/scratch_preflight.sh"))
        .arg(dir)
        .output()
        .expect("bash runs the shipped script")
}

/// The same shipped script, run against a directory this first covers with a
/// tmpfs of `size` inside an unprivileged user + mount namespace.
///
/// `unshare -m` makes the new namespace's propagation private, so the mount is
/// invisible to this machine and to every other test in the run: the scratch
/// directory handed over stays empty on the host.
fn preflight_on_a_filesystem_of(dir: &Path, size: &str) -> Output {
    let ran = Command::new("unshare")
        .args(["-Ur", "-m", "bash", "-c"])
        .arg(
            r#"set -eu; mount -t tmpfs -o size="$1" tmpfs "$2"; printf '%s\n' "$MOUNTED" >&2; exec bash "$3" "$2""#,
        )
        .arg("harness")
        .arg(size)
        .arg(dir)
        .arg(repo("tools/scratch_preflight.sh"))
        .env("MOUNTED", MOUNTED)
        .output();
    // FAIL LOUD, NEVER SKIP (hard rule 3). A machine that cannot manufacture a
    // full filesystem leaves the shortage branch of a gate untested, and that is
    // a fact a reader needs on the record — not a green test that quietly
    // checked nothing.
    //
    // AND THE HARNESS SAYS SO ITSELF, rather than being inferred from an exit
    // code. `unshare` being ABSENT is an `Err`; every other way this fails —
    // user namespaces disabled by sysctl, a hardened container, seccomp, a
    // `size` the kernel refuses — leaves the binary present and returns `Ok`
    // with a non-zero status, which would land on the shortage assertion below
    // and read as the preflight gate failing to void. The sentinel is printed
    // only after the mount has actually happened, so its absence is the harness
    // speaking about itself (tools/SHELL_CHECKLIST.md item 12).
    let ran = match ran {
        Ok(ran) => ran,
        Err(why) => panic!(
            "this test drives the shipped script against a real {size} filesystem and needs \
             `unshare` with unprivileged user namespaces to make one; it could not be run \
             ({why}), so the SHORTAGE branch of tools/scratch_preflight.sh is UNTESTED here"
        ),
    };
    let out = said(&ran);
    assert!(
        out.contains(MOUNTED),
        "the harness could not mount a {size} filesystem on this machine, so the SHORTAGE branch \
         of tools/scratch_preflight.sh is UNTESTED here — this is NOT a finding about the gate:\n{out}"
    );
    ran
}

/// Assert the gate's exit code in a message that says what the OTHER codes
/// would have meant (tools/SHELL_CHECKLIST.md item 12, obligation 3).
fn assert_code(ran: &Output, want: i32, what: &str) {
    let got = ran.status.code();
    if got == Some(want) {
        return;
    }
    let meaning = match got {
        Some(0) => "0 — there is room, and the number is printed",
        Some(1) => "1 — the caller called this wrong; nothing was learned about the machine",
        Some(2) => "2 — RUN VOID: no room, or the question could not be answered",
        _ => "a code this gate does not define, or a signal",
    };
    panic!(
        "{what}: expected exit {want}, got {got:?} ({meaning})\n{}",
        said(ran)
    );
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
    let ran = preflight(&dir);
    let out = said(&ran);
    assert_code(&ran, 0, "there is room here");
    assert!(
        out.contains(&format!("floor {MIN_SCRATCH_KIB} KiB")),
        "the floor it applied is on the record:\n{out}"
    );
    assert!(
        out.contains("KiB available"),
        "and so is what it read:\n{out}"
    );
}

/// THE VOID, and it is spelled VOID rather than FAIL. Item 12's whole point: two
/// different things exit non-zero and a reader has to be able to tell them apart
/// without knowing which gate printed the line.
///
/// The shortage is REAL — a 1 MiB filesystem under a floor of 1 GiB — so this
/// exercises the reading and the comparison together. The removed override could
/// only ever move the number on the right of the `<`.
#[test]
fn a_filesystem_too_small_for_the_floor_is_a_named_run_void_and_not_a_failure() {
    let dir = scratch("preflight-void");
    let ran = preflight_on_a_filesystem_of(&dir, "1M");
    let out = said(&ran);
    assert_code(&ran, 2, "a shortage is the void class");
    assert!(out.contains("RUN VOID"), "spelled as a void:\n{out}");
    assert!(
        out.contains("nothing was measured and nothing failed"),
        "and it says which of the two it is:\n{out}"
    );
    assert!(
        out.contains(&format!("wants {MIN_SCRATCH_KIB} KiB")),
        "naming the floor it applied, which is the constant and nothing else:\n{out}"
    );
    // AND IT READ THE FILESYSTEM IT WAS GIVEN. A tmpfs made at 1 MiB has a
    // little of it spent on the mount itself, so the assertion is a bracket
    // rather than an equality — but a script reading a neighbouring field would
    // print this machine's `/tmp`, which is three orders of magnitude away.
    let printed: u64 = out
        .split_whitespace()
        .zip(out.split_whitespace().skip(1))
        .find_map(|(value, unit)| (unit == "KiB").then(|| value.parse().ok())?)
        .unwrap_or_else(|| panic!("the void names what it read:\n{out}"));
    assert!(
        (1..=1024).contains(&printed),
        "the number is the 1 MiB filesystem's own, not another one's and not zero: {printed} \
         KiB — and `0 KiB available` is one of the two directions D-297 measured a wrong field \
         reporting, so it is refused here rather than passing as `small enough`:\n{out}"
    );
}

/// A directory that is not there is a VOID too — the question could not be
/// asked — and never a "no".
#[test]
fn a_missing_directory_is_a_void_rather_than_an_answer() {
    let dir = scratch("preflight-missing").join("nothing-here");
    let ran = preflight(&dir);
    let out = said(&ran);
    assert_code(&ran, 2, "a directory that is not there");
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
    assert_code(&ran, 1, "no argument at all");
    assert!(
        !out.contains("RUN VOID"),
        "a usage error is not a void:\n{out}"
    );
    assert!(out.contains("usage"), "{out}");
}

/// NOTHING IN THE ENVIRONMENT MOVES THE FLOOR.
///
/// The name the removed override answered to is set here to a value that would
/// have changed the verdict, and the script must not notice. Asserted rather
/// than described, because "the override is gone" is a claim about behaviour and
/// a deleted `if` is only evidence of it (docs/decisions.md D-306).
#[test]
fn the_retired_environment_override_no_longer_moves_the_floor() {
    let dir = scratch("preflight-no-override");
    let ran = Command::new("bash")
        .arg(repo("tools/scratch_preflight.sh"))
        .arg(&dir)
        .env("PISTOL_MIN_SCRATCH_KIB", "999999999999")
        .output()
        .expect("bash runs the shipped script");
    let out = said(&ran);
    assert_code(
        &ran,
        0,
        "a binding nothing reads cannot void a healthy directory",
    );
    assert!(
        out.contains(&format!("floor {MIN_SCRATCH_KIB} KiB")),
        "the constant is the floor, whatever the environment says:\n{out}"
    );
    assert!(
        !out.contains("999999999999"),
        "and the value never reaches a record:\n{out}"
    );
}

/// THE NUMBER THE SCRIPT PRINTS AGREES WITH A REFERENT COMPUTED OUTSIDE IT.
///
/// This is the guard the suite lacked, and its absence is why a column-parsing
/// defect shipped. Every refusal here USED to be manufactured by raising
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
    let ran = preflight(&dir);
    let out = said(&ran);
    assert_code(&ran, 0, "there is room here");

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
