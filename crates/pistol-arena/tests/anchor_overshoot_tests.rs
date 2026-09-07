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
