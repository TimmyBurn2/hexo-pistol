//! `tools/wp15b_attribution_check.py` — the instrument that produces WP-1.5b's
//! §8.3 Criterion 1 verdict (tools/SHELL_CHECKLIST.md item 10;
//! docs/experiments/wp15b_sprt_prereg.md).
//!
//! # What this file exists to prevent
//!
//! Two revisions of that pre-registration registered dry-run criteria that
//! PASSED on an arena mutated to invert the entire verdict. The criterion is now
//! a chain of three links, and a chain is only worth its weakest link, so each
//! link is driven here against a report corrupted in exactly the way that link
//! is supposed to notice — and, for the link that could pass by accident,
//! against the input on which it WOULD.
//!
//! # No report is committed
//!
//! A match log is an artifact and is never committed (CLAUDE.md rule 8), so the
//! report every case reads is built here, in a scratch directory, from the
//! spellings `conclusion.rs` and `report.rs` write. That is deliberate a second
//! time: a fixture copied from a run would agree with the instrument by
//! provenance, and these cases have to disagree with it on purpose.
//!
//! # The engine is a shim, and that is the whole point of link 1a
//!
//! Link 1a replays a turn and asks whether the engine the report NAMES returns
//! the move the report RECORDS. The referent is therefore whatever binary it is
//! handed. Here that is a shell shim answering from a table this file writes, so
//! a case can make the two seats answer differently (1a discriminates) or
//! identically (1a is vacuous, and must say so rather than pass).
//!
//! # RULE9-JUSTIFICATION: one instrument, one file, and the corrupted reports
//! are the test cases — splitting the builder from the cases it exists to
//! corrupt would put the two halves of every case in different files.

mod common;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

/// The book's length in the synthetic report: turns 1 and 2 are the opening, so
/// the two turns the instrument replays are 3 (p1's first search) and 4 (p2's).
const OPENING_TURNS: usize = 2;

/// What each seat's shim answers, by the number of turns already on the board.
/// The two seats differ at both replayed turns, so link 1a discriminates.
const RA_AT_2: &str = "1,1/2,2";
const RA_AT_3: &str = "3,3/4,4";
const RB_AT_2: &str = "5,5/6,6";
const RB_AT_3: &str = "7,7/8,8";

/// An honest report over two games and one pair.
///
/// Game 0: `ra` first, four turns, so the last turn is p2's and `rb` won it.
/// Game 1: `rb` first, five turns, so the last turn is p1's and `rb` won it too.
/// Seat A is `ra` and loses both, which is `pentanomial p0 1`.
fn honest_report(dir: &Path) -> String {
    let mut out = String::from("arena_report 4\n");
    out.push_str("arena_version 0.0.1\n");
    out.push_str(
        "experiment_sha256 0000000000000000000000000000000000000000000000000000000000000000\n",
    );
    out.push_str(&format!("opening_turns {OPENING_TURNS}\n"));
    out.push_str("budget nodes 50000\n");
    out.push_str("turn_cap 40\n");
    for (slot, label) in [("a", "ra"), ("b", "rb")] {
        out.push_str(&format!(
            "engine {slot} label {label} binary shim binary_sha256 {0} config {1} config_sha256 \
             {0} weights_sha256 {0}\n",
            "0".repeat(64),
            dir.join(format!("cfg-{label}.toml")).display()
        ));
        out.push_str(&format!("engine_id {slot} candidate_policy radius 2\n"));
    }
    out.push_str(
        "game 0 opening 0 p1 ra p2 rb result p2_win end normal forfeit_by none reason none turns \
         4 dup_of none nodes_a 1 nodes_b 1 depth_a 2 depth_b 3 llr_game none llr_pair none\n",
    );
    out.push_str(&format!("moves 0 0,0 0,1/1,-1 {RA_AT_2} {RB_AT_3}\n"));
    out.push_str(
        "game 1 opening 0 p1 rb p2 ra result p1_win end normal forfeit_by none reason none turns \
         5 dup_of none nodes_a 1 nodes_b 1 depth_a 2 depth_b 3 llr_game none llr_pair none\n",
    );
    out.push_str(&format!(
        "moves 1 0,0 0,1/1,-1 {RB_AT_2} {RA_AT_3} 9,9/9,8\n"
    ));
    out.push_str("pair 0 opening 0 bucket p0 score_a 0.000000000\n");
    out.push_str("counts n 2 distinct_n 2 wins_a 0 capped 0 losses_a 2 forfeits 0 decided 2\n");
    out.push_str("pentanomial p0 1 p1 0 p2 0 p3 0 p4 0\n");
    out.push_str("capped_fraction 0.000000000\n");
    out.push_str("llr_pair last -1.000000000\n");
    out.push_str("verdict inconclusive_at_game_cap\n");
    out.push_str("# timing — machine- and schedule-dependent; excluded from every comparison\n");
    out.push_str("timing_engine a time_ms 1 searches 1\n");
    out
}

/// A shim engine answering from a table, so a case decides whether the two seats
/// are distinguishable. `--config <path>` selects the table; the position line's
/// turn count selects the row.
fn shim(dir: &Path, ra: (&str, &str), rb: (&str, &str)) -> PathBuf {
    for (label, (at_two, at_three)) in [("ra", ra), ("rb", rb)] {
        fs::write(
            dir.join(format!("cfg-{label}.toml.answers")),
            format!("2 {at_two}\n3 {at_three}\n"),
        )
        .expect("the answer table is written");
    }
    let path = dir.join("shim-engine");
    fs::write(
        &path,
        r#"#!/usr/bin/env bash
set -euo pipefail
CONFIG=""
while [ "$#" -gt 0 ]; do
	case "$1" in
	--config)
		CONFIG="$2"
		shift 2
		;;
	*) shift ;;
	esac
done
# The turn count is the number of tokens after `moves` on the position line.
LINE="$(grep '^position start moves ' || true)"
[ -n "$LINE" ] || { echo "shim: no position line" >&2; exit 1; }
set -- $LINE
TURNS=$(( $# - 3 ))
ANSWER="$(awk -v n="$TURNS" '$1 == n { print $2 }' "$CONFIG.answers")"
[ -n "$ANSWER" ] || { echo "shim: no answer for $TURNS turns in $CONFIG.answers" >&2; exit 1; }
printf 'bestmove %s\n' "$ANSWER"
"#,
    )
    .expect("the shim is written");
    let mut mode = fs::metadata(&path).expect("the shim exists").permissions();
    mode.set_mode(0o755);
    fs::set_permissions(&path, mode).expect("the shim is executable");
    path
}

fn check(report: &Path, engine: &Path) -> Output {
    Command::new("python3")
        .arg(repo("tools/wp15b_attribution_check.py"))
        .arg(report)
        .arg(engine)
        .output()
        .expect("python3 runs the shipped instrument")
}

fn said(ran: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    )
}

/// Build the honest case, apply `corrupt` to the report text, and run the
/// shipped instrument over the result.
fn case(name: &str, corrupt: impl Fn(String) -> String) -> (Output, String) {
    let dir = scratch(name);
    let engine = shim(&dir, (RA_AT_2, RA_AT_3), (RB_AT_2, RB_AT_3));
    let report = dir.join("report.txt");
    fs::write(&report, corrupt(honest_report(&dir))).expect("the report is written");
    let ran = check(&report, &engine);
    let out = said(&ran);
    (ran, out)
}

/// THE CONTROL. Without it every refusal below is satisfied by an instrument
/// that refuses everything.
#[test]
fn an_honest_report_passes_all_three_links() {
    let (ran, out) = case("attribution-honest", |text| text);
    assert_eq!(
        ran.status.code(),
        Some(0),
        "nothing is wrong with it:\n{out}"
    );
    assert!(out.contains("PASS — 0 failure(s)"), "{out}");
    assert!(
        out.contains("1a: 4 turns replayed, 4 of them discriminating"),
        "and it says how much of link 1a was live:\n{out}"
    );
    assert!(
        out.contains("1b: 2 decided non-forfeit games adjudicated"),
        "{out}"
    );
}

/// LINK 1c, and this is the mutation both earlier revisions of the
/// pre-registration failed to catch. `GameRecord::score_a` is inverted: the
/// `game` lines are untouched and every derived count, pair and pentanomial
/// slot mirrors. A criterion reading only the report's own totals sees a
/// perfectly consistent document.
#[test]
fn a_verdict_inverted_downstream_of_the_game_lines_is_rejected() {
    let (ran, out) = case("attribution-inverted-score", |text| {
        text.replace(
            "counts n 2 distinct_n 2 wins_a 0 capped 0 losses_a 2",
            "counts n 2 distinct_n 2 wins_a 2 capped 0 losses_a 0",
        )
        .replace(
            "pair 0 opening 0 bucket p0 score_a 0.000000000",
            "pair 0 opening 0 bucket p4 score_a 1.000000000",
        )
        .replace(
            "pentanomial p0 1 p1 0 p2 0 p3 0 p4 0",
            "pentanomial p0 0 p1 0 p2 0 p3 0 p4 1",
        )
    });
    assert_eq!(
        ran.status.code(),
        Some(1),
        "an inverted verdict FAILS:\n{out}"
    );
    assert!(out.contains("FAIL 1c `counts wins_a 2` against 0"), "{out}");
    assert!(
        out.contains("FAIL 1c `pentanomial p4 1` against 0"),
        "{out}"
    );
}

/// LINK 1b. The result tokens are inverted, so the report credits the seat that
/// did NOT play the last turn — which game rule 3 makes visible from the move
/// list alone, without adjudicating a single stone.
#[test]
fn a_result_credited_to_the_seat_that_did_not_move_last_is_rejected() {
    let (ran, out) = case("attribution-inverted-result", |text| {
        text.replace("result p2_win", "result P1WIN")
            .replace("result p1_win", "result p2_win")
            .replace("result P1WIN", "result p1_win")
    });
    assert_eq!(ran.status.code(), Some(1), "{out}");
    assert!(
        out.contains("FAIL 1b game 0: 4 turns were played, so the last turn was p2's"),
        "{out}"
    );
    assert!(out.contains("FAIL 1b game 1:"), "{out}");
}

/// LINK 1a. The seat LABELS are swapped on the `game` lines, which no amount of
/// reading the report against itself can catch — the document stays internally
/// consistent. Only the engine, replayed outside the arena, disagrees.
#[test]
fn a_seat_label_attached_to_the_wrong_engine_is_rejected() {
    let (ran, out) = case("attribution-swapped-labels", |text| {
        text.replace("p1 ra p2 rb", "p1 rb p2 ra")
            .replace("p1 rb p2 ra result p1_win", "p1 ra p2 rb result p1_win")
    });
    assert_eq!(ran.status.code(), Some(1), "{out}");
    assert!(out.contains("FAIL 1a game 0"), "{out}");
    assert!(
        out.contains("answers"),
        "and the refusal names what the engine actually said:\n{out}"
    );
}

/// LINK 1a CAN PASS BY ACCIDENT, AND MUST SAY SO.
///
/// Two engines that answer identically satisfy the replay under ANY labelling,
/// so on such an input link 1a is not a criterion at all. It is required to
/// refuse rather than report a pass — which is the defect class this whole
/// document exists for, one level down: a check that cannot fail.
#[test]
fn a_replay_that_cannot_discriminate_the_seats_is_refused_as_vacuous() {
    let dir = scratch("attribution-vacuous");
    // Both seats answer the same, and the report's moves are built from those
    // answers, so every replay agrees with the report.
    let engine = shim(&dir, (RA_AT_2, RA_AT_3), (RA_AT_2, RA_AT_3));
    let report = dir.join("report.txt");
    let text = honest_report(&dir)
        .replace(RB_AT_2, RA_AT_2)
        .replace(RB_AT_3, RA_AT_3);
    fs::write(&report, text).expect("the report is written");
    let ran = check(&report, &engine);
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "a check that cannot fail is not a pass:\n{out}"
    );
    assert!(out.contains("1a is VACUOUS"), "{out}");
    assert!(
        out.contains("1a: 4 turns replayed, 0 of them discriminating"),
        "and the count is on the record either way:\n{out}"
    );
}

/// A REPORT IT CANNOT READ IS A THIRD THING (tools/SHELL_CHECKLIST.md item 12).
/// Exit 2, and deliberately not exit 1: an unreadable input has not shown that
/// anything is wrong with the run.
#[test]
fn an_unreadable_report_is_exit_two_and_not_a_finding() {
    let dir = scratch("attribution-unreadable");
    let engine = shim(&dir, (RA_AT_2, RA_AT_3), (RB_AT_2, RB_AT_3));

    let aborted = dir.join("aborted.txt");
    fs::write(&aborted, "arena_report_aborted 4\naborted Whatever\n").expect("written");
    let ran = check(&aborted, &engine);
    let out = said(&ran);
    assert_eq!(ran.status.code(), Some(2), "{out}");
    assert!(out.contains("CANNOT READ"), "{out}");
    assert!(
        out.contains("not a report carrying a verdict"),
        "named for what it is:\n{out}"
    );

    // A report whose two seats share a label carries no attribution to check.
    let same = dir.join("same-label.txt");
    fs::write(&same, honest_report(&dir).replace("label rb", "label ra")).expect("written");
    let ran = check(&same, &engine);
    let out = said(&ran);
    assert_eq!(ran.status.code(), Some(2), "{out}");
    assert!(out.contains("both seats carry the label"), "{out}");
}
