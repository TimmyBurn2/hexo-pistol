mod common;

use common::{repo, scratch};
use std::path::PathBuf;
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    repo("crates/pistol-cli/tests/fixtures").join(name)
}

#[test]
fn the_extractor_refuses_a_corpus_it_cannot_read_and_says_so() {
    let out = scratch("refuse-run");
    let missing = out.join("no-such-corpus.jsonl");
    let result = Command::new(env!("CARGO_BIN_EXE_corpus-extract"))
        .arg("--corpus")
        .arg(&missing)
        .arg("--out-dir")
        .arg(&out)
        .output()
        .expect("the binary runs");
    assert_eq!(result.status.code(), Some(2), "a refusal exits 2");
    let said = String::from_utf8_lossy(&result.stderr);
    assert!(
        said.contains("cannot read"),
        "it must say what it refused: {said}"
    );

    // With both required flags present, so this isolates the unknown-flag path
    // rather than the missing-flag one. (A missing required flag is reported
    // first, matching `pistol`'s own flag handling.)
    let unknown = Command::new(env!("CARGO_BIN_EXE_corpus-extract"))
        .arg("--corpus")
        .arg(&missing)
        .arg("--out-dir")
        .arg(&out)
        .arg("--nonsense")
        .arg("x")
        .output()
        .expect("the binary runs");
    assert_eq!(unknown.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&unknown.stderr).contains("unknown flag"),
        "an ignored flag is an instruction that silently did nothing"
    );
}

#[test]
fn a_corpus_with_an_excluded_game_exits_one_and_still_writes_both_fixtures() {
    // Design D.5's answer to the silent-failure risk: an inverted winner mapping,
    // or any real win-detection regression, must not exit 0. Nothing tested it,
    // so removing the exit-code branch left the suite green.
    let text = std::fs::read_to_string(fixture("corpus_synthetic_v1.jsonl")).expect("committed");
    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    lines[0] = lines[0].replace("\"winner\":1", "\"winner\":-1");
    let hash = lines[0]
        .split("\"game_hash\":\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("a hash")
        .to_string();

    let out = scratch("excluded");
    let corpus = out.join("corpus.jsonl");
    std::fs::write(&corpus, lines.join("\n") + "\n").expect("written");
    let result = Command::new(env!("CARGO_BIN_EXE_corpus-extract"))
        .arg("--corpus")
        .arg(&corpus)
        .arg("--out-dir")
        .arg(&out)
        .output()
        .expect("the binary runs");

    assert_eq!(
        result.status.code(),
        Some(1),
        "a run with an excluded game exits 1: stderr {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let said = String::from_utf8_lossy(&result.stdout);
    assert!(
        said.contains("REPLAY FAILURES: 1"),
        "the banner must be loud: {said}"
    );
    assert!(
        said.contains(&hash),
        "the stats block must name every excluded game: {said}"
    );
    for name in ["openings_v1.txt", "bench_positions_v1.txt"] {
        let rendered = std::fs::read_to_string(out.join(name))
            .unwrap_or_else(|error| panic!("{name} must still be written: {error}"));
        assert!(
            rendered.contains(&format!("# excluded {hash} winner-mismatch")),
            "{name}'s header must list the excluded game by hash with its named verdict"
        );
    }
}

#[test]
fn a_corpus_this_tool_cannot_curate_from_is_refused_by_name_and_keeps_its_stats() {
    // The escalation path: a corpus whose games all fail, or which carries no
    // ratings, used to abort with a panic and take the stats block with it —
    // losing the measurement that motivates the escalation.
    let out = scratch("uncuratable");
    let corpus = out.join("corpus.jsonl");
    let text = std::fs::read_to_string(fixture("corpus_synthetic_v1.jsonl")).expect("committed");
    let unrated: Vec<String> = text
        .lines()
        .map(|line| {
            let start = line.find("\"elo\":[").expect("every line has ratings");
            let end = line[start..].find(']').expect("a closing bracket") + start;
            format!("{}\"elo\":[null,null{}", &line[..start], &line[end..])
        })
        .collect();
    std::fs::write(&corpus, unrated.join("\n") + "\n").expect("written");

    let result = Command::new(env!("CARGO_BIN_EXE_corpus-extract"))
        .arg("--corpus")
        .arg(&corpus)
        .arg("--out-dir")
        .arg(&out)
        .output()
        .expect("the binary runs");
    assert_eq!(
        result.status.code(),
        Some(2),
        "a refusal exits 2, never 101"
    );
    assert!(
        String::from_utf8_lossy(&result.stderr)
            .contains("no game in this corpus can be an opening"),
        "the refusal is named: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        String::from_utf8_lossy(&result.stdout).contains("games read"),
        "and the stats block survives the refusal, because it is the finding"
    );

    // An empty document is refused too, rather than curated from.
    let empty = out.join("empty.jsonl");
    std::fs::write(&empty, "").expect("written");
    let result = Command::new(env!("CARGO_BIN_EXE_corpus-extract"))
        .arg("--corpus")
        .arg(&empty)
        .arg("--out-dir")
        .arg(&out)
        .output()
        .expect("the binary runs");
    assert_eq!(result.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("holds no games"),
        "an empty corpus is named: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn a_refused_write_leaves_the_output_directory_as_it_was() {
    // The two fixtures are a matched pair carrying the same provenance block, so
    // an exit of 2 — "refused before doing work" — must not leave one of them
    // replaced beside a stale partner.
    let out = scratch("blocked-write");
    std::fs::create_dir(out.join("bench_positions_v1.txt")).expect("block the second target");
    let result = Command::new(env!("CARGO_BIN_EXE_corpus-extract"))
        .arg("--corpus")
        .arg(fixture("corpus_synthetic_v1.jsonl"))
        .arg("--out-dir")
        .arg(&out)
        .output()
        .expect("the binary runs");

    assert_eq!(result.status.code(), Some(2));
    assert!(
        !out.join("openings_v1.txt").exists(),
        "the first fixture was put in place even though the run was refused"
    );
    let staged: Vec<_> = std::fs::read_dir(&out)
        .expect("the directory is readable")
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().contains("staged"))
        .collect();
    assert!(staged.is_empty(), "a refused run left staged files behind");
}
