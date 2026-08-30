mod common;

use common::{repo, scratch};
use pistol_cli::random_openings::{BookVersion, FILE_NAME};
use pistol_cli::sha256::sha256_hex;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The committed config: the only document this tool ships with.
fn committed_config() -> PathBuf {
    repo("configs/random_openings_v1.toml")
}

/// Run the binary with these arguments.
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_random-openings"))
        .args(args)
        .output()
        .expect("the binary is built by cargo test")
}

/// Run it against a config and an out-directory, and return the outcome.
fn generate_into(config: &Path, out: &Path) -> Output {
    run(&[
        "--config",
        &config.display().to_string(),
        "--out-dir",
        &out.display().to_string(),
    ])
}

/// The exit code, which is part of this program's contract.
fn code(output: &Output) -> i32 {
    output
        .status
        .code()
        .expect("the run was not killed by a signal")
}

/// A config document with one field changed from the committed one.
///
/// `book` is v1's throughout, so the file these cases write is the one
/// `FILE_NAME` names and the assertions below can go on reading it. What they
/// are about is the binary's REFUSALS, not which of the two books it writes.
fn config_with(name: &str, k_stones: usize, n_openings: usize, max_radius: u32) -> PathBuf {
    let path = scratch(name).join("random_openings_v1.toml");
    std::fs::write(
        &path,
        format!(
            "schema_version = 2\n[generate]\nbook = \"v1\"\nk_stones = {k_stones}\n\
             n_openings = {n_openings}\nmax_radius = {max_radius}\nseed = 7\n"
        ),
    )
    .expect("the scratch directory is writable");
    path
}

#[test]
fn random_openings_binary_writes_the_committed_book_byte_for_byte() {
    // The regeneration instruction printed in the config document and in the
    // book's own header, executed. If this fails, that instruction is wrong.
    let out = scratch("cli-regenerate");
    let output = generate_into(&committed_config(), &out);
    assert_eq!(
        code(&output),
        0,
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let written = std::fs::read(out.join(FILE_NAME)).expect("the run wrote the book");
    let committed = std::fs::read(repo("crates/pistol-cli/tests/fixtures").join(FILE_NAME))
        .expect("the book is committed");
    assert_eq!(
        sha256_hex(&written),
        sha256_hex(&committed),
        "the binary and the committed fixture disagree"
    );
}

#[test]
fn random_openings_binary_is_byte_identical_across_two_processes() {
    let first = scratch("cli-determinism-a");
    let second = scratch("cli-determinism-b");
    assert_eq!(code(&generate_into(&committed_config(), &first)), 0);
    assert_eq!(code(&generate_into(&committed_config(), &second)), 0);
    assert_eq!(
        sha256_hex(&std::fs::read(first.join(FILE_NAME)).expect("first run")),
        sha256_hex(&std::fs::read(second.join(FILE_NAME)).expect("second run")),
        "two processes over the same config wrote different books"
    );
}

#[test]
fn random_openings_binary_refuses_a_bad_command_line_by_name() {
    // Three different mistakes, and the program must not treat any of them as a
    // reason to guess (CLAUDE.md rule 3).
    let out = scratch("cli-flags");
    let config = committed_config().display().to_string();
    let dir = out.display().to_string();

    let unknown = run(&["--config", &config, "--out-dir", &dir, "--seed", "3"]);
    assert_eq!(code(&unknown), 2);
    assert!(
        String::from_utf8_lossy(&unknown.stderr).contains("unknown flag `--seed`"),
        "a flag that shapes the book must not be accepted on the command line"
    );

    let missing = run(&["--config", &config]);
    assert_eq!(code(&missing), 2);
    assert!(String::from_utf8_lossy(&missing.stderr).contains("`--out-dir` is required"));

    let no_value = run(&["--config", "--out-dir", &dir]);
    assert_eq!(code(&no_value), 2);
    assert!(
        String::from_utf8_lossy(&no_value.stderr).contains("needs a value, got the flag"),
        "a forgotten value must not silently become a path named after a flag"
    );

    let help = run(&["--help"]);
    assert_eq!(code(&help), 0);
    assert!(String::from_utf8_lossy(&help.stdout).contains("random-openings"));

    assert!(
        !out.join(FILE_NAME).exists(),
        "no refusal on this path may have written a book"
    );
}

#[test]
fn random_openings_binary_refuses_a_bad_document_and_writes_nothing() {
    let out = scratch("cli-bad-document");
    let mid_turn = config_with("cli-bad-document-config", 4, 10, 5);
    let output = generate_into(&mid_turn, &out);
    assert_eq!(code(&output), 2);
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("MID-TURN"),
        "the refusal names what is wrong: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::fs::read_dir(&out)
            .expect("the directory exists")
            .count(),
        0,
        "a refused run wrote something"
    );
}

#[test]
fn a_refused_run_leaves_an_earlier_book_exactly_as_it_was() {
    // The staged-then-renamed write, tested where it matters: a regeneration
    // that fails must not replace a good book with a partial one, and must not
    // leave a `.staged` file behind for the next reader to trip over.
    let out = scratch("cli-refusal-after-success");
    assert_eq!(code(&generate_into(&committed_config(), &out)), 0);
    let before = std::fs::read(out.join(FILE_NAME)).expect("the first run wrote a book");

    let impossible = config_with("cli-refusal-after-success-config", 5, 500, 1);
    let output = generate_into(&impossible, &out);
    assert_eq!(code(&output), 2);
    assert!(String::from_utf8_lossy(&output.stderr).contains("the pool ran dry"));

    let after = std::fs::read(out.join(FILE_NAME)).expect("the book is still there");
    assert_eq!(
        sha256_hex(&before),
        sha256_hex(&after),
        "a refused run replaced the book that was already there"
    );
    let leftovers: Vec<String> = std::fs::read_dir(&out)
        .expect("the directory exists")
        .map(|entry| {
            entry
                .expect("a directory entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name != FILE_NAME)
        .collect();
    assert!(
        leftovers.is_empty(),
        "a refused run left {leftovers:?} behind"
    );
}

#[test]
fn random_openings_binary_refuses_an_out_dir_it_cannot_write_into() {
    // Three ways the filesystem says no, each of which must be a named refusal
    // and none of which may leave a book behind. Chosen to be root-safe: a
    // permission bit stops meaning anything when the tests run as root, but a
    // directory standing where a file must go stops everyone.
    let config = committed_config();

    // The out-dir is an existing regular file, so it cannot be created.
    let occupied = scratch("cli-out-dir-is-a-file").join("in-the-way");
    std::fs::write(&occupied, b"not a directory").expect("scratch is writable");
    let output = generate_into(&config, &occupied);
    assert_eq!(code(&output), 2);
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("cannot create"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    // A directory stands where the staged file must be written.
    let blocked = scratch("cli-staged-name-taken");
    std::fs::create_dir(blocked.join(format!("{FILE_NAME}.staged"))).expect("scratch is writable");
    let output = generate_into(&config, &blocked);
    assert_eq!(code(&output), 2);
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("cannot write"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !blocked.join(FILE_NAME).exists(),
        "a run that could not stage its output still put a book in place"
    );
}

#[test]
fn a_failed_rename_removes_the_file_it_staged() {
    // The one branch of this binary that no other test reaches: the `let _ =`
    // cleanup after a rename that failed. A directory standing at the book's
    // own name lets the staged write succeed and the rename fail, which is the
    // only way to get here. What must not survive is the staged file — a
    // half-finished output left beside a name that does not exist yet is the
    // thing the staging was for.
    let out = scratch("cli-rename-blocked");
    let staged = out.join(format!("{FILE_NAME}.staged"));
    std::fs::create_dir(out.join(FILE_NAME)).expect("scratch is writable");

    let output = generate_into(&committed_config(), &out);
    assert_eq!(code(&output), 2);
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("cannot put"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !staged.exists(),
        "the staged file outlived the run that could not put it in place"
    );
}

/// The committed v2 document.
fn committed_v2_config() -> PathBuf {
    repo("configs/random_openings_v2.toml")
}

#[test]
fn a_v2_document_writes_v2_and_leaves_v1_alone() {
    // THE HAZARD, PINNED AT THE PLACE IT LIVED. Before the output name became a
    // closed set keyed by the document's own `[generate] book`, this exact
    // command with a `_v2` config OVERWROTE `random_openings_v1.txt` — the
    // artifact every closed SPRT verdict was taken over
    // (docs/experiments/stage3_detector_CLOSURE.md §5, docs/decisions.md D-513).
    //
    // Nothing else in the suite runs the BINARY with a v2 document, so nothing
    // else fails when `bin/random-openings.rs` goes back to writing `FILE_NAME`
    // unconditionally. This does: reverting that line makes both assertions
    // below fail, the second of them by writing v1's name.
    let out = scratch("cli-v2-writes-v2");
    let output = generate_into(&committed_v2_config(), &out);
    assert_eq!(code(&output), 0, "the committed v2 document generates");
    assert!(
        out.join(BookVersion::V2.file_name()).exists(),
        "a v2 document writes v2's name"
    );
    assert!(
        !out.join(FILE_NAME).exists(),
        "and writes NOTHING under v1's name — the whole reason the name is the \
         document's and not the tool's"
    );
    assert_eq!(
        std::fs::read_dir(&out)
            .expect("the directory exists")
            .count(),
        1,
        "one book, and no `.staged` left behind"
    );
}

#[test]
fn a_v2_run_reproduces_the_committed_v2_book_byte_for_byte() {
    // The regeneration instruction in `configs/random_openings_v2.toml` and in
    // the book's own header, executed. Its sibling for v1 is
    // `random_openings_binary_writes_the_committed_book_byte_for_byte`.
    let out = scratch("cli-v2-regenerate");
    assert_eq!(code(&generate_into(&committed_v2_config(), &out)), 0);
    let written = std::fs::read(out.join(BookVersion::V2.file_name())).expect("the run wrote it");
    let committed =
        std::fs::read(repo("crates/pistol-cli/tests/fixtures").join(BookVersion::V2.file_name()))
            .expect("the v2 book is committed");
    assert_eq!(
        sha256_hex(&written),
        sha256_hex(&committed),
        "the committed v2 bytes are the bytes this build writes"
    );
}
