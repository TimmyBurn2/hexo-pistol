mod common;

use common::{repo_root, scratch};
use std::process::Command;

/// The audit binary carries logic the library does not: the offender listing,
/// the `rated` prose branch, the verdict strings, and the 0/1/2 exit mapping.
/// The exit code IS the audit's machine-readable verdict, and nothing exercised
/// it (WP-P1d REVIEW-impl MA5).
fn audit(corpus: &std::path::Path, extra: &[&str]) -> (i32, String) {
    let mut command = Command::new(repo_root().join("target/release/corpus-audit"));
    command.arg("--corpus").arg(corpus);
    for word in extra {
        command.arg(word);
    }
    let done = command.output().expect("the release binary is built");
    let mut text = String::from_utf8_lossy(&done.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&done.stderr));
    (done.status.code().expect("the process exited"), text)
}

/// One JSONL line per game, written into a scratch corpus.
fn corpus_of(name: &str, lines: &[String]) -> std::path::PathBuf {
    let dir = scratch(name);
    let path = dir.join("corpus.jsonl");
    std::fs::write(&path, format!("{}\n", lines.join("\n"))).expect("scratch is writable");
    path
}

/// A decisive P1 game of exactly 20 stones: P1 owns flat indices
/// 0,3,4,7,8,11,12,15,16,19, and the last six of those are `(0,0)..(0,5)`.
fn decisive_line(hash: &str, winner: i32) -> String {
    let p1: [usize; 10] = [0, 3, 4, 7, 8, 11, 12, 15, 16, 19];
    let run: Vec<(i16, i16)> = (0..6i16).map(|step| (0, step)).collect();
    let mut moves: Vec<(i16, i16)> = Vec::new();
    let mut filler = 0i16;
    for index in 0..20usize {
        match p1.iter().position(|&i| i == index) {
            Some(slot) if slot >= 4 => moves.push(run[slot - 4]),
            _ => {
                moves.push((200 + filler * 5, 300 + filler * 9));
                filler += 1;
            }
        }
    }
    let listed: Vec<String> = moves.iter().map(|(q, r)| format!("[{q},{r}]")).collect();
    format!(
        "{{\"game_hash\":\"{hash}\",\"moves\":[{}],\"winner\":{winner},\"source\":\"human\",\"elo\":[1500,1500]}}",
        listed.join(",")
    )
}

#[test]
fn a_corpus_satisfying_the_filter_exits_zero_and_says_both_hold() {
    let path = corpus_of("audit-cli-clean", &[decisive_line("aaaa000000000001", 1)]);
    let (code, text) = audit(&path, &[]);
    assert_eq!(code, 0, "{text}");
    assert!(
        text.contains("source_filter auditable conjuncts: BOTH HOLD"),
        "{text}"
    );
    assert!(text.contains("games audited           1"), "{text}");
}

#[test]
fn a_corpus_breaking_a_conjunct_exits_one_and_names_the_offender() {
    // A 7-move game: under the floor, and its last stone completes nothing.
    let short = "{\"game_hash\":\"bbbb000000000001\",\"moves\":[[0,0],[5,0],[6,0],[0,1],[0,2],[7,0],[8,0]],\"winner\":1,\"source\":\"human\",\"elo\":[1500,1500]}";
    let path = corpus_of("audit-cli-short", &[short.to_string()]);
    let (code, text) = audit(&path, &[]);
    assert_eq!(code, 1, "a false source_filter is exit 1: {text}");
    assert!(
        text.contains("AT LEAST ONE IS FALSE — the metadata is wrong (D-456 STOP)"),
        "{text}"
    );
    assert!(
        text.contains("bbbb000000000001"),
        "the offender is named by game_hash: {text}"
    );
}

#[test]
fn a_record_contradicting_its_own_winner_is_named_without_failing_the_filter() {
    let path = corpus_of("audit-cli-winner", &[decisive_line("cccc000000000001", -1)]);
    let (code, text) = audit(&path, &[]);
    assert!(
        text.contains("`winner` disagrees with play 1"),
        "the contradiction is counted: {text}"
    );
    assert!(
        text.contains("cccc000000000001"),
        "and the offender named: {text}"
    );
    assert_eq!(
        code, 0,
        "`winner` is not a source_filter conjunct, so it does not STOP: {text}"
    );
}

#[test]
fn a_wrong_expect_sha_is_refused_before_any_audit_runs() {
    let path = corpus_of("audit-cli-sha", &[decisive_line("dddd000000000001", 1)]);
    let (code, text) = audit(&path, &["--expect-sha", "deadbeef"]);
    assert_eq!(code, 2, "{text}");
    assert!(text.contains("not the expected deadbeef"), "{text}");
    assert!(
        !text.contains("games audited"),
        "nothing was audited: {text}"
    );
}

#[test]
fn expect_sha_given_twice_is_refused_rather_than_silently_skipped() {
    // The sibling census binary had exactly this bug: `if let Ok(..)` over a
    // required-flag reader discarded the "given more than once" refusal along
    // with the check, so the gate stopped gating in silence (rule 3).
    let path = corpus_of(
        "audit-cli-sha-twice",
        &[decisive_line("eeee000000000001", 1)],
    );
    let (code, text) = audit(
        &path,
        &["--expect-sha", "deadbeef", "--expect-sha", "cafebabe"],
    );
    assert_eq!(code, 2, "{text}");
    assert!(text.contains("given more than once"), "{text}");
}

#[test]
fn an_unknown_flag_and_a_missing_corpus_are_both_refused() {
    let path = corpus_of("audit-cli-flags", &[decisive_line("ffff000000000001", 1)]);
    let (code, text) = audit(&path, &["--nonsense", "x"]);
    assert_eq!(code, 2, "{text}");
    assert!(text.contains("unknown flag"), "{text}");

    let missing = repo_root().join("no-such-corpus.jsonl");
    let (code, text) = audit(&missing, &[]);
    assert_eq!(code, 2, "{text}");
    assert!(text.contains("cannot read"), "{text}");
}
