mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::scratch;

/// Five space-separated tokens, because the script's corpus reader keeps only
/// the rows whose `key_full` holds exactly five stones.
fn key(tag: &str) -> String {
    format!("0,0:p1 1,0:p2 2,0:p2 3,0:p1 {tag}:p1")
}

/// A scratch tree with the shipped script one directory down, exactly as it
/// sits in the real tree: the script roots itself at `dirname/..`.
fn tree(name: &str) -> PathBuf {
    let root = scratch(name).join("repo");
    std::fs::create_dir_all(root.join("tools")).expect("a tools directory");
    common::seed_tool(&root, "tools/book_v3_disjointness.sh");
    // A stand-in for `book_keys`: every body line of these fixtures IS its own
    // key, so the script's set arithmetic is exercised without a nested cargo
    // build, which would block on the target directory's lock.
    let stub = root.join("tools/keys_stub.sh");
    std::fs::write(
        &stub,
        "#!/usr/bin/env bash\ngrep -v '^#' -- \"$1\" | grep -v '^$'\n",
    )
    .expect("the stub writes");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&stub, std::fs::Permissions::from_mode(0o755))
            .expect("the stub is executable");
    }
    root
}

fn book(root: &Path, name: &str, keys: &[String]) -> PathBuf {
    let path = root.join(name);
    std::fs::write(&path, format!("# a book\n{}\n", keys.join("\n"))).expect("the book writes");
    path
}

/// A corpus manifest in the real one's shape: `# columns:` header, then
/// tab-separated rows whose FIFTH field is `key_full`.
fn corpus(root: &Path, keys: &[String]) -> PathBuf {
    let path = root.join("corpus.txt");
    let mut text = String::from(
        "# columns: corpus_index, record_number, key_seq, key_pos, key_full, depth_turns, result, end\n",
    );
    for (index, k) in keys.iter().enumerate() {
        text.push_str(&format!(
            "{index}\t{index}\t-\tpos\t{k}\t4\tp1_win\tnormal\n"
        ));
    }
    // A three-stone row, which the reader must drop: only a five-stone key can
    // ever equal a five-stone opening.
    text.push_str("99\t99\t-\tpos\t0,0:p1 1,0:p2 2,0:p2\t4\tp1_win\tnormal\n");
    std::fs::write(&path, text).expect("the corpus writes");
    path
}

fn ran(root: &Path, args: &[&str]) -> Output {
    Command::new("bash")
        .arg(root.join("tools/book_v3_disjointness.sh"))
        .args(args)
        .arg("--keys-bin")
        .arg(root.join("tools/keys_stub.sh"))
        .current_dir(root)
        .output()
        .expect("the script runs")
}

fn said(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// What each exit code MEANS, so a void never reads as a regression
/// (tools/SHELL_CHECKLIST.md item 12, obligation 3).
fn code(out: &Output) -> i32 {
    out.status.code().expect("not killed by a signal")
}
const MEANING: &str = "0 = disjoint, 1 = an overlap was found, 2 = the run was VOID (no answer \
                       taken: a missing file, an unbuildable tool, a failed control)";

fn args_for<'a>(v1: &'a str, v2: &'a str, v3: &'a str, c: &'a str) -> Vec<&'a str> {
    vec!["--v1", v1, "--v2", v2, "--v3", v3, "--corpus", c]
}

#[test]
fn a_disjoint_book_answers_yes_and_names_both_terms_on_every_line() {
    let root = tree("disjoint-yes");
    let shared = key("9");
    let v1 = book(&root, "v1.txt", &[key("1"), key("2")]);
    let v2 = book(&root, "v2.txt", &[shared.clone(), key("3")]);
    let v3 = book(&root, "v3.txt", &[key("5"), key("6"), key("7")]);
    let c = corpus(&root, &[shared]);
    let out = ran(
        &root,
        &args_for(
            &v1.display().to_string(),
            &v2.display().to_string(),
            &v3.display().to_string(),
            &c.display().to_string(),
        ),
    );
    let text = said(&out);
    assert_eq!(code(&out), 0, "{MEANING}\n{text}");
    for line in [
        "v3 vs v1: 0 of 3",
        "v3 vs v2: 0 of 3",
        "v3 vs corpus: 0 of 3",
        "v3 internal: 0 of 3",
    ] {
        assert!(text.contains(line), "missing `{line}` in:\n{text}");
    }
    // AND IT ASKED FOR ITS SCRATCH SPACE FIRST (tools/SHELL_CHECKLIST.md item
    // 12 obligation 2). Asserted here rather than in a case of its own because
    // the preflight CALL is otherwise invisible: deleting it leaves this gate
    // passing, a preflight that passes being a no-op — D-553's call-removed
    // mutant, in the shape that survives a direct unit test.
    assert!(
        text.contains("scratch_preflight:") && text.contains("KiB available"),
        "the gate must ask for room before it writes any, and name what it \
         found:\n{text}"
    );
}

#[test]
fn an_overlap_with_a_committed_book_answers_no() {
    // THE CONTROL FOR THE TEST ABOVE: a script that refused everything would
    // pass that one, and a script that accepted everything passes this one.
    let root = tree("overlap-no");
    let shared = key("9");
    let clash = key("1");
    let v1 = book(&root, "v1.txt", std::slice::from_ref(&clash));
    let v2 = book(&root, "v2.txt", std::slice::from_ref(&shared));
    let v3 = book(&root, "v3.txt", &[clash, key("6")]);
    let c = corpus(&root, &[shared]);
    let out = ran(
        &root,
        &args_for(
            &v1.display().to_string(),
            &v2.display().to_string(),
            &v3.display().to_string(),
            &c.display().to_string(),
        ),
    );
    let text = said(&out);
    assert_eq!(code(&out), 1, "{MEANING}\n{text}");
    assert!(text.contains("v3 vs v1: 1 of 2"), "{text}");
}

#[test]
fn a_book_holding_one_position_twice_answers_no_on_the_internal_term() {
    let root = tree("internal-no");
    let shared = key("9");
    let twice = key("6");
    let v1 = book(&root, "v1.txt", &[key("1")]);
    let v2 = book(&root, "v2.txt", std::slice::from_ref(&shared));
    let v3 = book(&root, "v3.txt", &[twice.clone(), twice]);
    let c = corpus(&root, &[shared]);
    let out = ran(
        &root,
        &args_for(
            &v1.display().to_string(),
            &v2.display().to_string(),
            &v3.display().to_string(),
            &c.display().to_string(),
        ),
    );
    let text = said(&out);
    assert_eq!(code(&out), 1, "{MEANING}\n{text}");
    assert!(text.contains("v3 internal: 1 of 2"), "{text}");
}

#[test]
fn a_corpus_that_is_not_book_v2s_openings_voids_rather_than_passing() {
    // THE FALSE PASS THIS GUARD EXISTS FOR. If the key rendering here ever drifts
    // from the arena's `key_full`, every corpus comparison finds nothing and
    // reports `0 of N` — a pass produced by comparing two vocabularies. The
    // control catches it, and it is a VOID and not a FAIL: nothing was learned
    // about book_v3.
    let root = tree("control-void");
    let v1 = book(&root, "v1.txt", &[key("1")]);
    let v2 = book(&root, "v2.txt", &[key("2")]);
    let v3 = book(&root, "v3.txt", &[key("6")]);
    let c = corpus(&root, &[key("8")]);
    let out = ran(
        &root,
        &args_for(
            &v1.display().to_string(),
            &v2.display().to_string(),
            &v3.display().to_string(),
            &c.display().to_string(),
        ),
    );
    let text = said(&out);
    assert_eq!(code(&out), 2, "{MEANING}\n{text}");
    assert!(text.contains("control failed"), "{text}");
}

#[test]
fn a_missing_path_is_a_void_and_not_a_finding() {
    let root = tree("missing-void");
    let v1 = book(&root, "v1.txt", &[key("1")]);
    let out = ran(
        &root,
        &args_for(
            &v1.display().to_string(),
            "/nonexistent/v2.txt",
            &v1.display().to_string(),
            &v1.display().to_string(),
        ),
    );
    let text = said(&out);
    assert_eq!(code(&out), 2, "{MEANING}\n{text}");
    assert!(text.contains("cannot read"), "{text}");
}

#[test]
fn an_omitted_path_is_refused_rather_than_defaulted() {
    // CLAUDE.md rule 1: no code-side default for a path this script compares.
    let root = tree("omitted-void");
    let v1 = book(&root, "v1.txt", &[key("1")]);
    let out = ran(&root, &["--v1", &v1.display().to_string()]);
    let text = said(&out);
    assert_eq!(code(&out), 2, "{MEANING}\n{text}");
    assert!(text.contains("is required"), "{text}");
}
