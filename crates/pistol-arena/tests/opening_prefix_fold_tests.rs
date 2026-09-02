mod common;

use std::process::{Command, Output};

use common::{Scratch, repo};

/// The shipped script, driven as a program (docs/process.md's tools/ coverage
/// rule).
fn fold(scratch: &Scratch, body: &str, skip: &str, take: &str) -> Output {
    let book = scratch.write("book.txt", &format!("# a test book\n{body}"));
    Command::new("python3")
        .arg(repo().join("tools/opening_prefix_fold.py"))
        .arg("--book")
        .arg(&book)
        .arg("--skip")
        .arg(skip)
        .arg("--take")
        .arg(take)
        .output()
        .expect("python3 runs the folder")
}

/// A void is not an answer of zero (tools/SHELL_CHECKLIST.md item 12).
const VOID: i32 = 2;

/// The tool's rows with runs of spaces collapsed, so a test pins the NUMBERS and
/// not the column widths.
fn rows(output: &Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect()
}

#[test]
fn a_mirrored_two_turn_prefix_folds_only_under_the_symmetry_column() {
    let scratch = Scratch::new("opf-mirror");
    // Two openings whose FIRST TWO TURNS are reflections of each other and
    // whose third turns differ: the pair `canonical_form` dedupe leaves in the
    // book and whose prefix the a-priori argument wrongly said could not exist.
    let body = "start moves 0,0 1,0/2,0 5,0/6,0\nstart moves 0,0 0,1/0,2 7,0/8,0\n";
    let output = fold(&scratch, body, "0", "2");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let rows = rows(&output);
    // k, exact keys, stone sets, sharing, symmetry classes, sharing.
    // At k=2 the prefixes are reflections: two exact keys, two stone sets, ONE
    // symmetry class — the case the a-priori argument said could not exist.
    assert!(
        rows.contains(&"opening_prefix_fold: 2 2 2 0 1 2".to_string()),
        "{rows:#?}"
    );
    // At k=3 the whole openings differ, which is what `canonical_form` dedupes.
    assert!(
        rows.contains(&"opening_prefix_fold: 3 2 2 0 2 0".to_string()),
        "{rows:#?}"
    );
}

#[test]
fn a_window_past_the_end_of_the_book_is_a_void() {
    let scratch = Scratch::new("opf-window");
    let output = fold(&scratch, "start moves 0,0 1,0/2,0\n", "0", "5");
    assert_eq!(
        output.status.code(),
        Some(VOID),
        "a window past the end is a VOID and not an answer of zero. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("exceeds the book's"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn a_line_that_is_not_an_opening_is_a_void_naming_it() {
    let scratch = Scratch::new("opf-grammar");
    let output = fold(&scratch, "moves 0,0 1,0/2,0\n", "0", "1");
    assert_eq!(output.status.code(), Some(VOID));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("is not a `start moves"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
