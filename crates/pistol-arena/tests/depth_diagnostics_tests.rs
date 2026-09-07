mod common;

use std::path::Path;
use std::process::Command;

use common::{ConfigSpec, Ran, STUB, Scratch, openings_prefix, repo, run};

const READER: &str = "tools/depth_diagnostics.py";

fn read(report: &Path) -> std::process::Output {
    Command::new("python3")
        .arg(repo().join(READER))
        .arg(report)
        .output()
        .expect("python3 runs the reader")
}

/// A real arena report, produced by the real arena over the stub engine.
fn a_real_report(scratch: &Scratch, tag: &str) -> Ran {
    let openings = scratch.write(&format!("openings-{tag}.txt"), &openings_prefix(2));
    let config_a = scratch.stub_config(&format!("a-{tag}.toml"), "honest");
    let config_b = scratch.stub_config(&format!("b-{tag}.toml"), "honest");
    run(
        scratch,
        &ConfigSpec {
            openings: &openings,
            take: 2,
            skip: 0,
            turn_cap: 8,
            workers: 1,
            hang_ms: 30_000,
            elo1: 4.0,
            budget_kind: "nodes",
            budget_value: 5_000,
            binary_a: STUB,
            config_a: &config_a,
            binary_b: STUB,
            config_b: &config_b,
        },
        tag,
    )
}

/// The reader answers a report the arena actually wrote.
///
/// A hand-built record exercises the parser's syntax; only a report the arena
/// produced exercises the reader against the grammar the arena emits, which is
/// what `docs/process.md`'s dry-run rule is about.
#[test]
fn the_reader_answers_a_report_the_arena_itself_wrote() {
    let scratch = Scratch::new("depthdiag_real");
    let ran = a_real_report(&scratch, "real");
    let path = scratch.write("report-real-copy.txt", ran.report());

    let out = read(&path);
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        text.contains("largest pentanomial cell"),
        "the reader must render its verdict row; got:\n{text}"
    );
    assert!(
        text.contains("root-score fields present on `game` lines: NONE"),
        "an arena report carries no root score, and the reader says so by name; got:\n{text}"
    );
}

/// THE ATTRIBUTION GUARD: reached depth is read per SLOT and reported per LABEL.
///
/// The defect this excludes is the one a synthetic dry run cannot reach — the
/// reader crediting slot `a`'s depth to the label that sat in slot `b`. The
/// referent is external to the reader: the `game` lines' own `p1`/`p2` words,
/// which name the label, against `depth_a`/`depth_b`, which name the slot.
#[test]
fn reached_depth_is_credited_to_the_label_that_occupied_the_slot() {
    let scratch = Scratch::new("depthdiag_attr");
    let body = "\
arena_report 4
turn_cap 40
budget nodes 200000
game 0 opening 0 p1 deep p2 shallow result p1_win end normal forfeit_by none \
reason none turns 20 dup_of none nodes_a 10 nodes_b 20 depth_a 9 depth_b 3 \
llr_game none llr_pair none
game 1 opening 0 p1 shallow p2 deep result p1_win end normal forfeit_by none \
reason none turns 30 dup_of none nodes_a 11 nodes_b 21 depth_a 9 depth_b 3 \
llr_game none llr_pair none
pair 0 opening 0 bucket p2 score_a 0.500000000
";
    let path = scratch.write("attr.txt", body);
    let out = read(&path);
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // `deep` is p1 of game 0, so it holds slot a, whose depth is 9 in both games.
    let deep = text
        .lines()
        .find(|l| l.contains("| deep |"))
        .unwrap_or_else(|| panic!("no `deep` row:\n{text}"));
    let shallow = text
        .lines()
        .find(|l| l.contains("| shallow |"))
        .unwrap_or_else(|| panic!("no `shallow` row:\n{text}"));
    assert!(deep.contains("| 9.00 |"), "deep held slot a: {deep}");
    assert!(
        shallow.contains("| 3.00 |"),
        "shallow held slot b: {shallow}"
    );
    // Each won exactly one game, and the turns differ, so a swap shows here too.
    assert!(deep.contains("| 20.0 |"), "deep's win ran 20 turns: {deep}");
    assert!(
        shallow.contains("| 30.0 |"),
        "shallow's win ran 30 turns: {shallow}"
    );
}

/// THE CONTROL: the reader refuses rather than answering a report it cannot read.
///
/// Without this, every assertion above could be satisfied by a reader that
/// printed a table for anything at all.
#[test]
fn a_game_line_missing_a_field_is_refused_by_name_rather_than_answered() {
    let scratch = Scratch::new("depthdiag_refuse");
    let body = "\
arena_report 4
game 0 opening 0 p1 deep p2 shallow result p1_win end normal forfeit_by none \
reason none turns 20 dup_of none nodes_a 10 nodes_b 20 depth_a 9 \
llr_game none llr_pair none
";
    let path = scratch.write("short.txt", body);
    let out = read(&path);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a report the reader cannot read is the answer `no` (1), not a void (2) \
         and not a table (0); stdout: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("depth_b"),
        "the refusal names the missing field; got: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A file that is not an arena report at all is refused before any parsing.
#[test]
fn a_file_that_is_not_an_arena_report_is_refused_at_its_first_line() {
    let scratch = Scratch::new("depthdiag_kind");
    let path = scratch.write("nope.txt", "matchserver_report 1\ngames 4\n");
    let out = read(&path);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("arena_report"),
        "the refusal names the grammar it wanted"
    );
}

/// A path that does not exist is a VOID, not a `no` (SHELL_CHECKLIST item 12).
#[test]
fn an_absent_report_is_a_void_and_not_an_answer() {
    let scratch = Scratch::new("depthdiag_void");
    let out = read(&scratch.path("was-never-written.txt"));
    assert_eq!(
        out.status.code(),
        Some(2),
        "no answer could be taken, which is a different thing from the answer \
         being no (1) or the table being produced (0)"
    );
}
