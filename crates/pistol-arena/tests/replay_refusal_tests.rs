//! What `arena --replay` REFUSES, and that a refusal is never a finding.
//!
//! Split from `replay_tests.rs` on that line: that file is about what a pass
//! FINDS in a report it can read, and this one is about the documents and
//! command lines it will not take an answer from at all. The distinction is the
//! one `tools/SHELL_CHECKLIST.md` item 12 exists for — "the answer is no" and "I
//! could not take the answer" are two different things, and a replay that
//! reported an unreadable report as a divergence would send a reader hunting a
//! seat-attribution defect that is not there.
//!
//! Exit 2 in every case here, and NO document written: the mode promises the
//! same thing the generation path promises (`arena.rs`'s own exit block).
//!
//! # The control
//!
//! `an_untouched_report_is_accepted` runs the same helper over the SAME report
//! with no edit at all. Without it, every case below would pass against a build
//! that refused everything, which is the way a refusal suite fails silently.

mod common;

use std::path::PathBuf;
use std::process::Command;

use common::{ARENA, ConfigSpec, Ran, STUB, Scratch, openings_prefix, run};

const OPENINGS: usize = 4;
const TURN_CAP: u32 = 12;

/// One corruption: what it is called, how it edits an honest report, and the
/// words the refusal it earns must carry.
type Case = (&'static str, Box<dyn Fn(&str) -> String>, &'static str);

/// A small, clean, replayable run to corrupt copies of.
fn played(scratch: &Scratch) -> Ran {
    let openings = scratch.write("openings.txt", &openings_prefix(OPENINGS));
    let config_a = scratch.stub_config("a.toml", "honest");
    let config_b = scratch.stub_config("b.toml", "honest_last");
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
    run(scratch, &spec, "source")
}

/// Replay an edited copy of a report and hand back `(exit code, stderr, whether
/// a document exists)`.
fn attempt(
    scratch: &Scratch,
    ran: &Ran,
    tag: &str,
    edit: impl Fn(&str) -> String,
) -> (i32, String, bool) {
    let report = scratch.write(&format!("report-{tag}.txt"), &edit(ran.report()));
    let out = scratch.path(&format!("replay-{tag}.txt"));
    at(&report, &out, "1")
}

/// Run the mode over an exact report path, out path and worker spelling.
fn at(report: &PathBuf, out: &PathBuf, workers: &str) -> (i32, String, bool) {
    let output = Command::new(ARENA)
        .arg("--replay")
        .arg(report)
        .arg("--out")
        .arg(out)
        .arg("--workers")
        .arg(workers)
        .output()
        .expect("the arena binary runs");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stderr).to_string(),
        out.exists(),
    )
}

/// Replace the first line matching `starts` using `swap`.
fn on_line(text: &str, starts: &str, swap: impl Fn(&str) -> String) -> String {
    let mut done = false;
    text.split('\n')
        .map(|line| {
            if !done && line.starts_with(starts) {
                done = true;
                swap(line)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// THE CONTROL. The same path, unedited, is taken.
#[test]
fn an_untouched_report_is_accepted() {
    let scratch = Scratch::new("replayref-control");
    let ran = played(&scratch);
    let (code, stderr, wrote) = attempt(&scratch, &ran, "control", str::to_string);
    assert_eq!(code, 0, "the control run is refused: {stderr}");
    assert!(wrote, "and it wrote its document");
}

/// Every document this mode will not take an answer from.
#[test]
fn a_report_this_mode_cannot_answer_about_is_refused_by_name() {
    let scratch = Scratch::new("replayref-docs");
    let ran = played(&scratch);

    let cases: Vec<Case> = vec![
        (
            "aborted",
            Box::new(|text: &str| text.replacen("arena_report ", "arena_report_aborted ", 1)),
            "carries no verdict",
        ),
        (
            "schema",
            Box::new(|text: &str| {
                on_line(text, "arena_report ", |_| String::from("arena_report 3"))
            }),
            "schema 3",
        ),
        (
            "movetime",
            Box::new(|text: &str| {
                on_line(text, "budget ", |_| String::from("budget movetime_ms 500"))
            }),
            "only a `nodes` budget replays",
        ),
        (
            "binary_digest",
            Box::new(|text: &str| {
                on_line(text, "engine a ", |line| {
                    let words: Vec<String> = line
                        .split(' ')
                        .scan(false, |next, word| {
                            let out = if *next {
                                "0".repeat(64)
                            } else {
                                word.to_string()
                            };
                            *next = word == "binary_sha256";
                            Some(out)
                        })
                        .collect();
                    words.join(" ")
                })
            }),
            "is not the build this run is written for",
        ),
        (
            "illegal_move",
            Box::new(|text: &str| {
                on_line(text, "moves 0 ", |line| {
                    let mut words: Vec<&str> = line.split(' ').collect();
                    // A BOOK turn, so no engine is ever asked about it: what
                    // this must reach is the document check, not a comparison.
                    // The origin is occupied from turn one, and turn two owes
                    // two stones, so a single stone there is doubly illegal.
                    words[3] = "0,0";
                    words.join(" ")
                })
            }),
            "not legal",
        ),
        (
            "repeated_key",
            Box::new(|text: &str| on_line(text, "game 0 ", |line| format!("{line} result p1_win"))),
            "appears twice",
        ),
        (
            "turn_count",
            Box::new(|text: &str| {
                on_line(text, "game 0 ", |line| {
                    line.replace(" turns ", " turns 99 ignored ")
                })
            }),
            "against",
        ),
        (
            "same_label",
            Box::new(|text: &str| {
                on_line(text, "engine b label b ", |line| {
                    line.replacen(" label b ", " label a ", 1)
                })
            }),
            "both seats carry the label",
        ),
    ];

    for (tag, edit, must_say) in cases {
        let (code, stderr, wrote) = attempt(&scratch, &ran, tag, &edit);
        assert_eq!(
            code, 2,
            "`{tag}` must be a void and not a finding — a divergence count read off it would be a \
             report about a document nobody can read. It said: {stderr}"
        );
        assert!(
            stderr.contains(must_say),
            "`{tag}` must refuse by name, saying `{must_say}`; it said: {stderr}"
        );
        assert!(
            !wrote,
            "`{tag}` promised no document at exit 2 and wrote one anyway"
        );
    }
}

/// A worker count spelled a way this program will not echo back.
#[test]
fn a_worker_count_is_refused_on_its_spelling_and_not_only_its_value() {
    let scratch = Scratch::new("replayref-workers");
    let ran = played(&scratch);
    let report = scratch.write("report-w.txt", ran.report());
    for spelling in ["04", "+4", "0", "four", "4.0"] {
        let out = scratch.path(&format!("replay-w{spelling}.txt"));
        let (code, stderr, wrote) = at(&report, &out, spelling);
        assert_eq!(code, 2, "`--workers {spelling}` was taken: {stderr}");
        assert!(!wrote, "`--workers {spelling}` wrote a document");
    }
    let out = scratch.path("replay-w4.txt");
    let (code, stderr, _) = at(&report, &out, "4");
    assert_eq!(code, 0, "and `--workers 4` is taken: {stderr}");
}

/// An out path that already exists is claimed by nobody twice.
#[test]
fn an_existing_out_path_is_refused_before_any_engine_is_spawned() {
    let scratch = Scratch::new("replayref-claim");
    let ran = played(&scratch);
    let report = scratch.write("report-c.txt", ran.report());
    let out = scratch.write("taken.txt", "somebody else's evidence\n");
    let (code, stderr, _) = at(&report, &out, "1");
    assert_eq!(code, 2, "an existing report was overwritten: {stderr}");
    assert_eq!(
        std::fs::read_to_string(&out).expect("the file is still there"),
        "somebody else's evidence\n",
        "and its bytes are untouched"
    );
}
