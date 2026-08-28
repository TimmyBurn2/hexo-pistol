mod common;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

/// Turns 1 and 2 are the book, so the turns the instrument replays are 3 (p1's
/// first search of the game) and 4 (p2's).
const OPENING_TURNS: usize = 2;

/// The second turn of each opening. It is the shim's key, so two openings can
/// be given different discriminating behaviour in one report.
const OPENINGS: [&str; 2] = ["0,1/1,-1", "2,2/3,3"];

/// One opening's answers: what `ra` plays at turn 3 and turn 4, then what `rb`
/// plays at the same two turns. Where the two seats' pair is equal, link 1a
/// cannot tell them apart on that opening's games.
struct Answers {
    ra: (&'static str, &'static str),
    rb: (&'static str, &'static str),
}

/// Both openings distinguish the seats: every game is attributable.
fn both_openings_discriminate() -> [Answers; 2] {
    [
        Answers {
            ra: ("1,1/2,2", "3,3/4,4"),
            rb: ("5,5/6,6", "7,7/8,8"),
        },
        Answers {
            ra: ("1,2/2,3", "3,4/4,5"),
            rb: ("5,6/6,7", "7,8/8,9"),
        },
    ]
}

/// The SECOND opening does not: its two games are unattributable by link 1a,
/// whatever the labels on them say.
fn the_second_opening_does_not() -> [Answers; 2] {
    let mut answers = both_openings_discriminate();
    answers[1].rb = answers[1].ra;
    answers
}

/// An honest report over four games and two pairs, one pair per opening.
///
/// Even game: `ra` first, four turns, so the last turn is p2's and `rb` won it.
/// Odd game: `rb` first, five turns, so the last turn is p1's and `rb` won it
/// too. Seat A is `ra` and loses all four, which is `pentanomial p0 2`.
fn honest_report(dir: &Path, answers: &[Answers; 2]) -> String {
    let mut out = String::from("arena_report 4\n");
    out.push_str("arena_version 0.0.1\n");
    out.push_str(
        "experiment_sha256 0000000000000000000000000000000000000000000000000000000000000000\n",
    );
    out.push_str(&format!("opening_turns {OPENING_TURNS}\n"));
    out.push_str("budget nodes 50000\n");
    out.push_str("turn_cap 40\n");
    // Criterion 1' clause (b) recomputes off these exact bounds — production's
    // own values (D-190's elo1, D-375's architect ruling), so a fixture's
    // arithmetic means the same thing a governed run's does.
    out.push_str("sprt elo0 0.000000000 elo1 25.000000000 alpha 0.050000000 beta 0.050000000\n");
    for (slot, label) in [("a", "ra"), ("b", "rb")] {
        out.push_str(&format!(
            "engine {slot} label {label} binary shim binary_sha256 {0} config {1} config_sha256 \
             {0} weights_sha256 {0}\n",
            "0".repeat(64),
            dir.join(format!("cfg-{label}.toml")).display()
        ));
        out.push_str(&format!("engine_id {slot} candidate_policy radius 2\n"));
    }
    for (opening, answer) in answers.iter().enumerate() {
        let book = OPENINGS[opening];
        // Even game: ra moves at turn 3, rb at turn 4, and the game ends there.
        out.push_str(&format!(
            "game {0} opening {opening} p1 ra p2 rb result p2_win end normal forfeit_by none \
             reason none turns 4 dup_of none nodes_a 1 nodes_b 1 depth_a 2 depth_b 3 llr_game \
             none llr_pair none\n",
            opening * 2
        ));
        out.push_str(&format!(
            "moves {} 0,0 {book} {} {}\n",
            opening * 2,
            answer.ra.0,
            answer.rb.1
        ));
        // Odd game: the seats are reversed and one more turn is played.
        out.push_str(&format!(
            "game {0} opening {opening} p1 rb p2 ra result p1_win end normal forfeit_by none \
             reason none turns 5 dup_of none nodes_a 1 nodes_b 1 depth_a 2 depth_b 3 llr_game \
             none llr_pair none\n",
            opening * 2 + 1
        ));
        out.push_str(&format!(
            "moves {} 0,0 {book} {} {} 9,9/9,8\n",
            opening * 2 + 1,
            answer.rb.0,
            answer.ra.1
        ));
    }
    out.push_str("pair 0 opening 0 bucket p0 score_a 0.000000000\n");
    out.push_str("pair 1 opening 1 bucket p0 score_a 0.000000000\n");
    out.push_str("counts n 4 distinct_n 4 wins_a 0 capped 0 losses_a 4 forfeits 0 decided 4\n");
    out.push_str("pentanomial p0 2 p1 0 p2 0 p3 0 p4 0\n");
    out.push_str("capped_fraction 0.000000000\n");
    // n=2, both pairs bucket p0: mu=0, var=0 — DEGENERATE, per sprt.rs, so
    // `llr_pair` is genuinely undefined and `verdict` is genuinely
    // `inconclusive_degenerate` (MEASURED against the ported arithmetic in
    // tools/wp15b_attribution_check.py, not asserted) — necessary now that
    // Criterion 1' clause (b) self-checks this fixture's own numbers before
    // trusting them on any flipped pentanomial.
    out.push_str("llr_pair last none\n");
    out.push_str("verdict inconclusive_degenerate\n");
    out.push_str("# timing — machine- and schedule-dependent; excluded from every comparison\n");
    out.push_str("timing_engine a time_ms 1 searches 1\n");
    out
}

/// A shim engine answering from a table. `--config <path>` selects the table;
/// the position line's turn count and its OPENING select the row, so one report
/// can hold an opening the seats differ on and one they do not.
fn shim(dir: &Path, answers: &[Answers; 2]) -> PathBuf {
    let mut tables = [String::new(), String::new()];
    for (opening, answer) in answers.iter().enumerate() {
        let book = OPENINGS[opening];
        for (table, seat) in tables.iter_mut().zip([answer.ra, answer.rb]) {
            table.push_str(&format!("2_{book} {}\n3_{book} {}\n", seat.0, seat.1));
        }
    }
    for (label, table) in ["ra", "rb"].iter().zip(&tables) {
        fs::write(dir.join(format!("cfg-{label}.toml.answers")), table)
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
# `position start moves 0,0 <opening> …`: the turn count is the token count
# after `moves`, and the opening is the second of those tokens.
LINE="$(grep '^position start moves ' || true)"
[ -n "$LINE" ] || { echo "shim: no position line" >&2; exit 1; }
set -- $LINE
TURNS=$(( $# - 3 ))
OPENING="$5"
ANSWER="$(awk -v k="${TURNS}_${OPENING}" '$1 == k { print $2 }' "$CONFIG.answers")"
[ -n "$ANSWER" ] || { echo "shim: no answer for ${TURNS}_${OPENING} in $CONFIG.answers" >&2; exit 1; }
printf 'bestmove %s\n' "$ANSWER"
"#,
    )
    .expect("the shim is written");
    let mut mode = fs::metadata(&path).expect("the shim exists").permissions();
    mode.set_mode(0o755);
    fs::set_permissions(&path, mode).expect("the shim is executable");
    path
}

/// A report of `n_pairs` pairs: pair 0 is a p2 split (so the sample is not
/// wholly degenerate), every other pair is a p4 sweep for seat `ra`, and
/// `vacuous_pair` (which must not be 0) is ALSO a p4 sweep but with `ra` and
/// `rb` answering identically at both checked turns, so link 1a cannot
/// discriminate it. Self-contained from `honest_report`/`shim` above (which
/// are fixed at exactly 2 openings): duplicating the shim script here is
/// deliberate rather than a shared helper neither caller needs the full
/// generality of.
fn many_pairs_report_and_shim(
    dir: &Path,
    n_pairs: usize,
    vacuous_pair: usize,
) -> (String, PathBuf) {
    assert!(
        vacuous_pair != 0,
        "pair 0 is the split pair, never the vacuous one"
    );
    assert!(vacuous_pair < n_pairs);

    let book = |i: usize| format!("{i},0/0,{i}");
    let ra_t3 = |i: usize| format!("1,{i}/2,{i}");
    let ra_t4 = |i: usize| format!("5,{i}/6,{i}");
    let rb_t3 = |i: usize| format!("3,{i}/4,{i}");
    let rb_t4 = |i: usize| format!("7,{i}/8,{i}");
    const FILLER: &str = "9,9/9,8";

    let mut ra_table = String::new();
    let mut rb_table = String::new();
    for i in 0..n_pairs {
        let vacuous = i == vacuous_pair;
        ra_table.push_str(&format!(
            "2_{} {}\n3_{} {}\n",
            book(i),
            ra_t3(i),
            book(i),
            ra_t4(i)
        ));
        rb_table.push_str(&format!(
            "2_{} {}\n3_{} {}\n",
            book(i),
            if vacuous { ra_t3(i) } else { rb_t3(i) },
            book(i),
            if vacuous { ra_t4(i) } else { rb_t4(i) },
        ));
    }
    fs::write(dir.join("cfg-ra.toml.answers"), ra_table).expect("ra's table is written");
    fs::write(dir.join("cfg-rb.toml.answers"), rb_table).expect("rb's table is written");

    let engine = dir.join("shim-engine");
    fs::write(
        &engine,
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
LINE="$(grep '^position start moves ' || true)"
[ -n "$LINE" ] || { echo "shim: no position line" >&2; exit 1; }
set -- $LINE
TURNS=$(( $# - 3 ))
OPENING="$5"
ANSWER="$(awk -v k="${TURNS}_${OPENING}" '$1 == k { print $2 }' "$CONFIG.answers")"
[ -n "$ANSWER" ] || { echo "shim: no answer for ${TURNS}_${OPENING} in $CONFIG.answers" >&2; exit 1; }
printf 'bestmove %s\n' "$ANSWER"
"#,
    )
    .expect("the shim is written");
    let mut mode = fs::metadata(&engine)
        .expect("the shim exists")
        .permissions();
    mode.set_mode(0o755);
    fs::set_permissions(&engine, mode).expect("the shim is executable");

    let mut out = String::from("arena_report 4\narena_version 0.0.1\n");
    out.push_str(
        "experiment_sha256 0000000000000000000000000000000000000000000000000000000000000000\n",
    );
    out.push_str(&format!("opening_turns {OPENING_TURNS}\n"));
    out.push_str("budget nodes 50000\n");
    out.push_str("turn_cap 40\n");
    out.push_str("sprt elo0 0.000000000 elo1 25.000000000 alpha 0.050000000 beta 0.050000000\n");
    for (slot, label) in [("a", "ra"), ("b", "rb")] {
        out.push_str(&format!(
            "engine {slot} label {label} binary shim binary_sha256 {0} config {1} config_sha256 \
             {0} weights_sha256 {0}\n",
            "0".repeat(64),
            dir.join(format!("cfg-{label}.toml")).display()
        ));
    }

    let mut wins_a = 0u64;
    for i in 0..n_pairs {
        let book_i = book(i);
        if i == 0 {
            // The split pair: ra wins as p1 in the even game, rb wins as p1
            // in the odd game — one win, one loss for seat A.
            out.push_str(
                "game 0 opening 0 p1 ra p2 rb result p1_win end normal forfeit_by none reason \
                 none turns 5 dup_of none nodes_a 1 nodes_b 1 depth_a 2 depth_b 3 llr_game none \
                 llr_pair none\n",
            );
            out.push_str(&format!(
                "moves 0 0,0 {book_i} {} {} {FILLER}\n",
                ra_t3(0),
                rb_t4(0)
            ));
            out.push_str(
                "game 1 opening 0 p1 rb p2 ra result p1_win end normal forfeit_by none reason \
                 none turns 5 dup_of none nodes_a 1 nodes_b 1 depth_a 2 depth_b 3 llr_game none \
                 llr_pair none\n",
            );
            out.push_str(&format!(
                "moves 1 0,0 {book_i} {} {} {FILLER}\n",
                rb_t3(0),
                ra_t4(0)
            ));
            wins_a += 1;
        } else {
            let vacuous = i == vacuous_pair;
            let (t3a, t4b) = if vacuous {
                (ra_t3(i), ra_t4(i))
            } else {
                (ra_t3(i), rb_t4(i))
            };
            out.push_str(&format!(
                "game {0} opening {i} p1 ra p2 rb result p1_win end normal forfeit_by none \
                 reason none turns 5 dup_of none nodes_a 1 nodes_b 1 depth_a 2 depth_b 3 \
                 llr_game none llr_pair none\n",
                2 * i
            ));
            out.push_str(&format!(
                "moves {0} 0,0 {book_i} {t3a} {t4b} {FILLER}\n",
                2 * i
            ));
            let (t3b, t4a) = if vacuous {
                (ra_t3(i), ra_t4(i))
            } else {
                (rb_t3(i), ra_t4(i))
            };
            out.push_str(&format!(
                "game {0} opening {i} p1 rb p2 ra result p2_win end normal forfeit_by none \
                 reason none turns 4 dup_of none nodes_a 1 nodes_b 1 depth_a 2 depth_b 3 \
                 llr_game none llr_pair none\n",
                2 * i + 1
            ));
            out.push_str(&format!("moves {0} 0,0 {book_i} {t3b} {t4a}\n", 2 * i + 1));
            wins_a += 2;
        }
    }
    let n_games = 2 * n_pairs;
    let losses_a = n_games as u64 - wins_a;

    out.push_str("pair 0 opening 0 bucket p2 score_a 0.500000000\n");
    for i in 1..n_pairs {
        out.push_str(&format!(
            "pair {i} opening {i} bucket p4 score_a 1.000000000\n"
        ));
    }
    out.push_str(&format!(
        "counts n {n_games} distinct_n {n_games} wins_a {wins_a} capped 0 losses_a {losses_a} \
         forfeits 0 decided {n_games}\n"
    ));
    out.push_str(&format!(
        "pentanomial p0 0 p1 0 p2 1 p3 0 p4 {}\n",
        n_pairs - 1
    ));
    out.push_str("capped_fraction 0.000000000\n");
    out.push_str(&format!(
        "first_player_wins {} of {n_games} decided_non_forfeit forfeits 0\n",
        n_pairs + 1
    ));
    // MEASURED (not asserted) against tools/wp15b_attribution_check.py's own
    // ported arithmetic at n_pairs=20 (`[0,0,1,0,19]`): llr_pair 8.7678,
    // bounds ±2.9444 — h1. Recomputing with the vacuous pair's bucket
    // adversarially reassigned (`[1,0,1,0,18]`) still gives llr_pair 3.5234 —
    // still h1, the case this fixture exists to build.
    out.push_str("llr_pair last 8.767752171\n");
    out.push_str("verdict h1\n");
    out.push_str("verdict_unit pair\n");
    out.push_str("verdict_if_clean none pairs_dropped 0\n");
    out.push_str("# timing — machine- and schedule-dependent; excluded from every comparison\n");
    out.push_str("timing_engine a time_ms 1 searches 1\n");

    (out, engine)
}

fn check(report: &Path, engine: &Path) -> Output {
    Command::new("python3")
        .arg(repo("tools/wp15b_attribution_check.py"))
        .arg(report)
        .arg(engine)
        .output()
        .expect("python3 runs the shipped instrument")
}

/// Assert the instrument's exit code in a message that says what the OTHER
/// codes would have meant.
///
/// `tools/SHELL_CHECKLIST.md` item 12, obligation 3. A bare
/// `assert_eq!(code, Some(1))` reports a VOID — the instrument saying it could
/// not take the answer — as "it failed to reject an inverted verdict", which
/// sends the reader at the wrong end of it.
fn assert_code(ran: &Output, want: i32, what: &str) {
    let got = ran.status.code();
    if got == Some(want) {
        return;
    }
    let meaning = match got {
        Some(0) => "0 — all three links agree with the report",
        Some(1) => "1 — a link disagrees, and the failing records are named",
        Some(2) => "2 — the answer could not be taken; NOT a finding about the run",
        _ => "a code this instrument does not define, or a signal",
    };
    panic!(
        "{what}: expected exit {want}, got {got:?} ({meaning})\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    );
}

fn said(ran: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    )
}

/// Build a case: choose which openings distinguish the seats, corrupt the
/// report, and run the shipped instrument over the result.
fn case(name: &str, answers: [Answers; 2], corrupt: impl Fn(String) -> String) -> (Output, String) {
    let dir = scratch(name);
    let engine = shim(&dir, &answers);
    let report = dir.join("report.txt");
    fs::write(&report, corrupt(honest_report(&dir, &answers))).expect("the report is written");
    let ran = check(&report, &engine);
    let out = said(&ran);
    (ran, out)
}

/// THE CONTROL. Without it every refusal below is satisfied by an instrument
/// that refuses everything.
#[test]
fn an_honest_report_passes_all_three_links() {
    let (ran, out) = case("attribution-honest", both_openings_discriminate(), |t| t);
    assert_code(&ran, 0, "nothing is wrong with this report");
    assert!(out.contains("PASS — 0 failure(s)"), "{out}");
    assert!(
        out.contains(
            "1a: 8 turns replayed, 8 of them discriminating, 4 of 4 games directly attributed by replay"
        ),
        "and it says how much of link 1a was live:\n{out}"
    );
    assert!(
        out.contains("1b: 4 decided non-forfeit games adjudicated"),
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
    let (ran, out) = case(
        "attribution-inverted-score",
        both_openings_discriminate(),
        |text| {
            text.replace(
                "counts n 4 distinct_n 4 wins_a 0 capped 0 losses_a 4",
                "counts n 4 distinct_n 4 wins_a 4 capped 0 losses_a 0",
            )
            .replace(
                "bucket p0 score_a 0.000000000",
                "bucket p4 score_a 1.000000000",
            )
            .replace(
                "pentanomial p0 2 p1 0 p2 0 p3 0 p4 0",
                "pentanomial p0 0 p1 0 p2 0 p3 0 p4 2",
            )
        },
    );
    assert_code(&ran, 1, "an inverted verdict is a finding");
    assert!(out.contains("FAIL 1c `counts wins_a 4` against 0"), "{out}");
    assert!(
        out.contains("FAIL 1c `pentanomial p4 2` against 0"),
        "{out}"
    );
}

/// LINK 1b. The result tokens are inverted, so the report credits the seat that
/// did NOT play the last turn — which game rule 3 makes visible from the move
/// list alone, without adjudicating a single stone.
#[test]
fn a_result_credited_to_the_seat_that_did_not_move_last_is_rejected() {
    let (ran, out) = case(
        "attribution-inverted-result",
        both_openings_discriminate(),
        |text| {
            text.replace("result p2_win", "result P1WIN")
                .replace("result p1_win", "result p2_win")
                .replace("result P1WIN", "result p1_win")
        },
    );
    assert_code(&ran, 1, "a result credited to the wrong seat is a finding");
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
    let (ran, out) = case(
        "attribution-swapped-labels",
        both_openings_discriminate(),
        |text| {
            text.replace("p1 ra p2 rb", "p1 XX p2 YY")
                .replace("p1 rb p2 ra", "p1 ra p2 rb")
                .replace("p1 XX p2 YY", "p1 rb p2 ra")
        },
    );
    assert_code(&ran, 1, "a label on the wrong engine is a finding");
    assert!(out.contains("FAIL 1a game 0"), "{out}");
    assert!(
        out.contains("answers"),
        "and the refusal names what the engine actually said:\n{out}"
    );
}

/// **THE REVIEW FINDING, KEPT AS AN ATTACK, CAUGHT A DIFFERENT WAY**
/// (docs/decisions.md D-308, D-384).
///
/// A seat swap confined to the pair link 1a cannot attribute is invisible to
/// 1b and 1c by construction: 1b does not read labels at all, and 1c DERIVES
/// seat A's score from the very `game` line the swap corrupts, so it agrees
/// with totals mirrored to match — the report stays internally consistent,
/// `verdict` included (a real corrupted run's verdict line is computed off
/// the same corrupted pentanomial, so this fixture updates it too, not left
/// stale). Under the prior rule any vacuous game was itself a FAILURE, which
/// caught this by construction; under Criterion 1' (D-384) vacuity alone is
/// not a failure, so this attack is now caught (or not) by clause (b) alone —
/// and here it IS caught, because reassigning pair 1 back to its honest
/// bucket p0 changes an n=2 sample from non-degenerate (`inconclusive_at_
/// game_cap`) to fully degenerate (`inconclusive_degenerate`), a genuine
/// verdict-token change. MEASURED against the ported arithmetic, not
/// asserted: honest `[2,0,0,0,0]` recomputes `inconclusive_degenerate`;
/// corrupted `[1,0,0,0,1]` recomputes `inconclusive_at_game_cap`.
#[test]
fn a_seat_swap_confined_to_a_vacuous_pair_fails_robustness() {
    let (ran, out) = case(
        "attribution-confined-swap",
        the_second_opening_does_not(),
        |text| {
            // Only the second opening's pair is corrupted, and its totals —
            // including the verdict a real corrupted run would recompute —
            // are mirrored so the document stays internally consistent.
            text.replace(
                "game 2 opening 1 p1 ra p2 rb",
                "game 2 opening 1 p1 rb p2 ra",
            )
            .replace(
                "game 3 opening 1 p1 rb p2 ra",
                "game 3 opening 1 p1 ra p2 rb",
            )
            .replace(
                "counts n 4 distinct_n 4 wins_a 0 capped 0 losses_a 4",
                "counts n 4 distinct_n 4 wins_a 2 capped 0 losses_a 2",
            )
            .replace(
                "pair 1 opening 1 bucket p0 score_a 0.000000000",
                "pair 1 opening 1 bucket p4 score_a 1.000000000",
            )
            .replace(
                "pentanomial p0 2 p1 0 p2 0 p3 0 p4 0",
                "pentanomial p0 1 p1 0 p2 0 p3 0 p4 1",
            )
            .replace(
                "verdict inconclusive_degenerate",
                "verdict inconclusive_at_game_cap",
            )
        },
    );
    assert_code(
        &ran,
        1,
        "a swap that moves the verdict under reassignment is not a pass",
    );
    assert!(
        out.contains("1a robustness FAILS"),
        "the refusal names it as a clause-(b) robustness failure, not a clause-(a) inversion:\n{out}"
    );
    assert!(
        out.contains("1 (opening 1)"),
        "and it names WHICH pair:\n{out}"
    );
    assert!(
        out.contains("from `inconclusive_at_game_cap` to `inconclusive_degenerate`"),
        "and both verdict tokens, before and after reassignment:\n{out}"
    );
}

/// AND A VACUOUS PAIR THAT DOES NOT MOVE THE VERDICT IS CERTIFIED, not
/// refused — the point of D-384's TOLERATE-WITH-ROBUSTNESS: an unattributable
/// pair is only a problem if the verdict depends on which way it went.
/// Without this case, the attack test above could be satisfied by an
/// instrument that still fires on any vacuity at all — which is exactly the
/// rule Criterion 1' replaced — because at only 2 pairs (as `the_second_
/// opening_does_not` builds), reassigning ANY pair always flips a
/// degenerate n=2 sample to non-degenerate or back, so no 2-pair fixture can
/// demonstrate genuine robustness. Sized at 20 pairs instead — the smallest
/// size MEASURED (not asserted; see `many_pairs_report_and_shim`) to survive
/// one pair's adversarial reassignment without moving the verdict off `h1`,
/// mirroring the shape `wp15b_vacuity_diagnostics.md` found in the actual
/// governed run (D-381): mostly decisive pairs for one seat, one split pair,
/// and one pair link 1a cannot discriminate.
#[test]
fn a_vacuous_pair_that_does_not_move_the_verdict_is_certified() {
    let dir = scratch("attribution-robust-vacuity");
    const N_PAIRS: usize = 20;
    const VACUOUS_PAIR: usize = 5;
    let (report, engine) = many_pairs_report_and_shim(&dir, N_PAIRS, VACUOUS_PAIR);
    let path = dir.join("report.txt");
    fs::write(&path, &report).expect("the report is written");
    let ran = check(&path, &engine);
    let out = said(&ran);
    assert_code(
        &ran,
        0,
        "one vacuous pair that does not move the verdict is not a failure",
    );
    assert!(out.contains("PASS — 0 failure(s)"), "{out}");
    assert!(
        out.contains("1a robustness: 1 vacuous pair(s)"),
        "the pass is not silent about the vacuity it tolerated:\n{out}"
    );
    assert!(out.contains("verdict `h1` unchanged"), "{out}");
}

/// A REPORT OR AN ENGINE IT CANNOT USE IS A THIRD THING
/// (tools/SHELL_CHECKLIST.md item 12). Exit 2, and deliberately not exit 1: no
/// answer was taken, so nothing has been shown about the run. An engine binary
/// that was never built must not read as a seat-attribution defect.
#[test]
fn an_answer_that_could_not_be_taken_is_exit_two_and_not_a_finding() {
    let dir = scratch("attribution-void");
    let answers = both_openings_discriminate();
    let engine = shim(&dir, &answers);
    let honest = honest_report(&dir, &answers);

    for (name, text, expect) in [
        (
            "aborted.txt",
            String::from("arena_report_aborted 4\naborted Whatever\n"),
            "not a report carrying a verdict",
        ),
        (
            "same-label.txt",
            honest.replace("label rb", "label ra"),
            "both seats carry the label",
        ),
        (
            "movetime.txt",
            honest.replace("budget nodes 50000", "budget movetime_ms 400"),
            "replays only a `nodes` budget",
        ),
        (
            "twice.txt",
            honest.replace("turns 4 dup_of none", "turns 4 turns 5 dup_of none"),
            "appears twice on one record",
        ),
    ] {
        let path = dir.join(name);
        fs::write(&path, text).expect("written");
        let ran = check(&path, &engine);
        let out = said(&ran);
        assert_code(&ran, 2, name);
        assert!(out.contains("CANNOT READ"), "{name}:\n{out}");
        assert!(
            out.contains(expect),
            "{name} is named for what it is:\n{out}"
        );
    }

    let missing = dir.join("nothing-here.txt");
    let ran = check(&missing, &engine);
    assert_code(&ran, 2, "a report that is not there");

    let report = dir.join("report.txt");
    fs::write(&report, &honest).expect("written");
    let ran = check(&report, &dir.join("no-such-engine"));
    let out = said(&ran);
    assert_code(&ran, 2, "an engine binary that was never built");
    assert!(out.contains("could not be run"), "{out}");
}
