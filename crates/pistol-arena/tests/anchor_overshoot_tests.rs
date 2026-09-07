mod common;

use std::path::Path;
use std::process::{Command, Output};

use common::{Scratch, repo};

const READER: &str = "tools/anchor_overshoot.py";

fn read(dir: &Path, budgets: &[&str]) -> Output {
    Command::new("python3")
        .arg(repo().join(READER))
        .arg(dir)
        .args(budgets)
        .output()
        .expect("python3 runs the reader")
}

/// One game's transcript, in the shape the matchserver writes.
fn write_run(scratch: &Scratch, games: &[&[(&str, u64)]]) -> std::path::PathBuf {
    let dir = scratch.dir.join("run");
    std::fs::create_dir_all(&dir).expect("a run directory");
    for (index, turns) in games.iter().enumerate() {
        let mut body = format!(
            "{{\"a_is_p1\":true,\"event\":\"game_start\",\"game\":{},\"opening\":\"x\"}}\n",
            index + 1
        );
        for (engine, wall) in turns.iter() {
            body.push_str(&format!(
                "{{\"engine\":\"{engine}\",\"engine_time_ms\":null,\"event\":\"turn\",\
                 \"mover\":\"p1\",\"wall_ms\":{wall}}}\n"
            ));
        }
        std::fs::write(dir.join(format!("g{:03}.jsonl", index + 1)), body).expect("a transcript");
    }
    dir
}

/// THE SPLIT THIS READER EXISTS FOR: the first answer of a game is reported
/// apart from the rest, because a client that respawns its engine per game
/// charges process start-up to exactly that answer.
///
/// By construction here the first answer of each game overshoots by 100 ms and
/// every later one by 0, so a reader that pooled them would report a max excess
/// of 100 in both columns and cannot pass.
#[test]
fn the_first_answer_of_each_game_is_reported_apart_from_the_rest() {
    let scratch = Scratch::new("overshoot_split");
    let dir = write_run(
        &scratch,
        &[
            &[("slow", 400), ("slow", 300), ("slow", 300)],
            &[("slow", 400), ("slow", 300)],
        ],
    );
    let out = read(&dir, &["slow=300"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        text.contains("| slow | 100 / 100 | 0 / 0 |"),
        "first-of-game excess is 100 and every later answer is 0; got:\n{text}"
    );
    assert!(
        text.contains("| slow | 300 | 5 | 300 |"),
        "and the pooled row still reports all five answers; got:\n{text}"
    );
}

/// A budget nobody named leaves the overshoot columns empty rather than
/// inventing one — an engine's budget is the seat's, and a reader that guessed
/// would publish a number no document registered (CLAUDE.md hard rule 1).
#[test]
fn an_engine_with_no_named_budget_gets_no_overshoot_rather_than_a_default() {
    let scratch = Scratch::new("overshoot_nobudget");
    let dir = write_run(&scratch, &[&[("unbudgeted", 700)]]);
    let out = read(&dir, &[]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(out.status.code(), Some(0));
    assert!(
        text.contains("| unbudgeted | — | 1 | 700 |"),
        "no budget means no overshoot columns; got:\n{text}"
    );
}

/// A turn record missing the field the reader names is refused, not skipped.
#[test]
fn a_turn_record_without_a_wall_is_refused_by_name() {
    let scratch = Scratch::new("overshoot_short");
    let dir = scratch.dir.join("run");
    std::fs::create_dir_all(&dir).expect("a run directory");
    std::fs::write(
        dir.join("g001.jsonl"),
        "{\"engine\":\"a\",\"event\":\"turn\",\"mover\":\"p1\"}\n",
    )
    .expect("a transcript");
    let out = read(&dir, &[]);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("wall_ms"),
        "the refusal names the missing field: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A directory that exists and holds no transcripts is the answer `no` (1); a
/// directory that does not exist is a VOID (2). Spelling them alike is what
/// turns an environmental accident into a regression report
/// (tools/SHELL_CHECKLIST.md item 12).
#[test]
fn an_empty_run_is_an_answer_and_an_absent_run_is_a_void() {
    let scratch = Scratch::new("overshoot_void");
    let empty = scratch.dir.join("empty");
    std::fs::create_dir_all(&empty).expect("an empty directory");
    assert_eq!(
        read(&empty, &[]).status.code(),
        Some(1),
        "empty is an answer"
    );
    assert_eq!(
        read(&scratch.dir.join("never-made"), &[]).status.code(),
        Some(2),
        "absent is a void, and the two must not be spelled alike"
    );
}

/// One game's transcript with full control of every field.
fn write_raw(scratch: &Scratch, name: &str, lines: &[&str]) -> std::path::PathBuf {
    let dir = scratch.dir.join(name);
    std::fs::create_dir_all(&dir).expect("a run directory");
    std::fs::write(dir.join("g001.jsonl"), format!("{}\n", lines.join("\n")))
        .expect("a transcript");
    dir
}

const ANSWERED: &str = "{\"engine\":\"s\",\"engine_time_ms\":300,\"event\":\"turn\",\
\"outcome\":{\"kind\":\"continue\"},\"wall_ms\":301}";
const FORFEIT: &str = "{\"engine\":\"s\",\"engine_time_ms\":null,\"event\":\"turn\",\
\"outcome\":{\"kind\":\"engine_failure\",\"detail\":\"engine timeout\"},\"wall_ms\":0}";

/// A TURN THE ENGINE NEVER ANSWERED IS NOT AN ANSWER, and it is not silently
/// dropped either.
///
/// The referee writes a record with `wall_ms: 0` and a null engine time on any
/// engine failure. Counting those poisons every statistic here: with the
/// forfeit included this fixture reports `1 of 2` engine times and a
/// first-of-game median excess of **-150 ms**, which reads as a start-up charge
/// that has been fixed when in fact the seat never started.
#[test]
fn a_turn_the_engine_never_answered_is_excluded_and_the_exclusion_is_reported() {
    let scratch = Scratch::new("overshoot_forfeit");
    let dir = write_raw(&scratch, "run", &[FORFEIT, ANSWERED]);
    let out = read(&dir, &["s=300"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        text.contains("| s | 300 | 1 |"),
        "one answered turn, not two; got:\n{text}"
    );
    assert!(
        text.contains("1 of 1"),
        "and the engine-time count is over answered turns only; got:\n{text}"
    );
    assert!(
        text.contains("| 1 / 1 / 1 / 1 | 0 | 1 |"),
        "the refusal is REPORTED rather than silently dropped (hard rule 3); got:\n{text}"
    );
}

/// THE CONTROL: a run with no forfeits reports none, so the column above is not
/// simply always 1.
#[test]
fn a_run_without_a_forfeit_reports_no_refused_turns() {
    let scratch = Scratch::new("overshoot_noforfeit");
    let dir = write_raw(&scratch, "run", &[ANSWERED, ANSWERED]);
    let text = String::from_utf8_lossy(&read(&dir, &["s=300"]).stdout).into_owned();
    assert!(
        text.contains("| 1 / 1 / 1 / 1 | 0 | 0 |"),
        "no forfeit, so the refused column is 0 and not simply always 1; got:\n{text}"
    );
}

/// A SEAT REPORTING SOMETHING OTHER THAN ITS OWN ELAPSED TIME SHOWS AS NEGATIVE.
///
/// The defect this makes visible is a shim answering with its CONFIGURED budget
/// instead of the time it actually spent: on any answer that returned early the
/// reported time exceeds the wall it was measured over, and `wall - engine`
/// goes below zero. Nothing else in this reader would notice.
#[test]
fn a_seat_reporting_more_than_its_wall_shows_as_a_negative_gap() {
    let scratch = Scratch::new("overshoot_negative");
    let budget_reporter = "{\"engine\":\"s\",\"engine_time_ms\":300,\"event\":\"turn\",\
\"outcome\":{\"kind\":\"continue\"},\"wall_ms\":0}";
    let dir = write_raw(&scratch, "run", &[budget_reporter, ANSWERED]);
    let text = String::from_utf8_lossy(&read(&dir, &["s=300"]).stdout).into_owned();
    assert!(
        text.contains("| -300 / -150 / 1 / 1 | 1 |"),
        "the minimum and the negative count are what carry this; got:\n{text}"
    );
}

/// The per-side totals and the share of wall the engine accounts for — the
/// headline number a measured search-time ratio is built from.
#[test]
fn the_totals_row_reports_each_seat_s_engine_time_against_its_wall() {
    let scratch = Scratch::new("overshoot_totals");
    let dir = write_raw(&scratch, "run", &[ANSWERED, ANSWERED]);
    let text = String::from_utf8_lossy(&read(&dir, &["s=300"]).stdout).into_owned();
    assert!(
        text.contains("| s | 602 ms | 600 ms | 99.67% |"),
        "two answers of wall 301 and engine 300; got:\n{text}"
    );
}

/// A run whose every turn is a forfeit is refused, not answered with an empty
/// table: there is no answer to take.
#[test]
fn a_run_with_no_answered_turn_is_refused_by_name() {
    let scratch = Scratch::new("overshoot_allforfeit");
    let dir = write_raw(&scratch, "run", &[FORFEIT, FORFEIT]);
    let out = read(&dir, &["s=300"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("no answered turns"),
        "the refusal says what was missing: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}
