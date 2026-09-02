mod common;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{ARENA, ConfigSpec, Ran, STUB, Scratch, openings_prefix, repo, run};

const OPENINGS: usize = 2;
const TURN_CAP: u32 = 8;
const LABEL_NODES: &str = "5000";

/// A self-play report, its capture, and the corpus `arena --labels` wrote from
/// them — the artefact the assembler reads, produced by the pipeline that
/// produces it in the sweep.
fn corpus(scratch: &Scratch, tag: &str) -> PathBuf {
    let openings = scratch.write(&format!("openings-{tag}.txt"), &openings_prefix(OPENINGS));
    let engine_config = scratch.stub_config(&format!("engine-{tag}.toml"), "honest");
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
        config_a: &engine_config,
        binary_b: STUB,
        config_b: &engine_config,
    };
    let ran: Ran = run(scratch, &spec, tag);
    let report = scratch.write(&format!("report-{tag}-copy.txt"), ran.report());
    let capture = scratch.path(&format!("capture-{tag}.txt"));
    let captured = Command::new(ARENA)
        .args(["--capture"])
        .arg(&report)
        .arg("--out")
        .arg(&capture)
        .args(["--label-nodes", LABEL_NODES])
        .output()
        .expect("the arena binary runs");
    assert!(
        capture.exists(),
        "no capture: {}",
        String::from_utf8_lossy(&captured.stderr)
    );
    let corpus = scratch.path(&format!("corpus-{tag}.txt"));
    let labelled = Command::new(ARENA)
        .arg("--labels")
        .arg(&capture)
        .arg("--report")
        .arg(&report)
        .arg("--out")
        .arg(&corpus)
        .output()
        .expect("the arena binary runs");
    assert!(
        corpus.exists(),
        "no corpus: {}",
        String::from_utf8_lossy(&labelled.stderr)
    );
    corpus
}

/// Drive the SHIPPED script, never a copy of it
/// (tools/SHELL_CHECKLIST.md item 10).
fn assemble(out_dir: &Path, corpora: &[&Path]) -> Output {
    let mut command = Command::new("python3");
    command
        .arg(repo().join("tools/wp21_assemble.py"))
        .arg("--out-dir")
        .arg(out_dir);
    for corpus in corpora {
        command.arg("--corpus").arg(corpus);
    }
    command.output().expect("the assembler runs")
}

/// The three codes spelled out, so a failure names what the code the test did
/// NOT get would have meant (tools/SHELL_CHECKLIST.md item 12).
fn meaning(output: &Output) -> String {
    format!(
        "exit {:?} — 0 is `both manifests written`, 1 is `REFUSED, an output exists`, 2 is `RUN \
         VOID, an input is not a corpus`.\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// One `# derived <key> <value>` off a manifest's header.
fn derived(text: &str, key: &str) -> String {
    let prefix = format!("# derived {key} ");
    text.lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .unwrap_or_else(|| panic!("no `{prefix}` line in:\n{text}"))
        .to_string()
}

/// A corpus's body lines, split into fields.
fn records(path: &Path) -> Vec<Vec<String>> {
    let text = std::fs::read_to_string(path).expect("the corpus is readable");
    pistol_cli::corpus::emit::body_of(&text)
        .expect("a corpus carries a body digest")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split('\t').map(str::to_string).collect())
        .collect()
}

/// A corpus's own claimed body digest.
fn body_digest(path: &Path) -> String {
    let text = std::fs::read_to_string(path).expect("the corpus is readable");
    text.lines()
        .find_map(|line| line.strip_prefix("# body_sha256 "))
        .expect("a corpus carries a body digest")
        .to_string()
}

/// Rewrite a corpus's records through `edit`, with the header digest brought
/// back into agreement so the doctored file is one the assembler will read.
fn rebuild(path: &Path, edit: impl Fn(usize, &mut Vec<String>)) -> String {
    let text = std::fs::read_to_string(path).expect("the corpus is readable");
    let header: String = text
        .lines()
        .take_while(|line| !line.starts_with("# body_sha256 "))
        .map(|line| format!("{line}\n"))
        .collect();
    let body: String = records(path)
        .into_iter()
        .enumerate()
        .map(|(at, mut fields)| {
            edit(at, &mut fields);
            format!("{}\n", fields.join("\t"))
        })
        .collect();
    format!(
        "{header}# body_sha256 {}\n{body}",
        pistol_cli::sha256::sha256_hex(body.as_bytes())
    )
}

fn out_dir(scratch: &Scratch, name: &str) -> PathBuf {
    let dir = scratch.path(name);
    std::fs::create_dir_all(&dir).expect("an output directory");
    dir
}

// ---------------------------------------------------------------------------
// THE CONTROL RUN, and the counts read off it against an independent tally.
// ---------------------------------------------------------------------------

#[test]
fn two_corpora_assemble_into_a_raw_manifest_naming_each_ones_own_digest_and_count() {
    let scratch = Scratch::new("asm-raw");
    let a = corpus(&scratch, "a");
    let b = corpus(&scratch, "b");
    let dir = out_dir(&scratch, "out");
    let output = assemble(&dir, &[&a, &b]);
    assert_eq!(output.status.code(), Some(0), "{}", meaning(&output));
    let raw = std::fs::read_to_string(dir.join("raw_manifest.txt")).expect("a raw manifest");
    let rows: Vec<Vec<&str>> = raw
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 2, "one row per corpus:\n{raw}");
    for (row, path) in rows.iter().zip([&a, &b]) {
        assert_eq!(
            row[2],
            body_digest(path),
            "the row carries the corpus's own digest"
        );
        assert_eq!(
            row[3].parse::<usize>().ok(),
            Some(records(path).len()),
            "the row carries the corpus's own record count"
        );
    }
    assert_eq!(
        derived(&raw, "records").parse::<usize>().ok(),
        Some(records(&a).len() + records(&b).len())
    );
}

#[test]
fn the_deduped_manifest_holds_one_row_per_three_key_distinct_position() {
    let scratch = Scratch::new("asm-dedup");
    let a = corpus(&scratch, "a");
    let b = corpus(&scratch, "b");
    let dir = out_dir(&scratch, "out");
    let output = assemble(&dir, &[&a, &b]);
    assert_eq!(output.status.code(), Some(0), "{}", meaning(&output));
    // The referent: the three key columns of every record, tallied here without
    // the script.
    let mut distinct: BTreeSet<(String, String, String)> = BTreeSet::new();
    let mut total = 0;
    for path in [&a, &b] {
        for fields in records(path) {
            total += 1;
            distinct.insert((fields[3].clone(), fields[4].clone(), fields[5].clone()));
        }
    }
    let deduped = std::fs::read_to_string(dir.join("deduped_manifest.txt")).expect("a manifest");
    let rows = deduped
        .lines()
        .filter(|line| !line.starts_with('#'))
        .count();
    assert_eq!(
        rows,
        distinct.len(),
        "one row per distinct position:\n{deduped}"
    );
    assert_eq!(
        derived(&deduped, "distinct_positions"),
        distinct.len().to_string()
    );
    assert_eq!(derived(&deduped, "records"), total.to_string());
    assert!(
        distinct.len() < total,
        "two corpora of one experiment must share positions, or the dedup was never exercised"
    );
    // Identical corpora: every tie goes to the FIRST, which is corpus 1.
    for line in deduped.lines().filter(|line| !line.starts_with('#')) {
        assert_eq!(
            line.split('\t').next(),
            Some("1"),
            "a tie did not go to the first: {line}"
        );
    }
}

#[test]
fn the_deeper_label_wins_a_position_two_records_share() {
    let scratch = Scratch::new("asm-deeper");
    let a = corpus(&scratch, "a");
    let b = corpus(&scratch, "b");
    let deeper = rebuild(&b, |at, fields| {
        if at == 0 {
            fields[10] = String::from("9");
        }
    });
    let b_deeper = scratch.write("corpus-b-deeper.txt", &deeper);
    let dir = out_dir(&scratch, "out");
    let output = assemble(&dir, &[&a, &b_deeper]);
    assert_eq!(output.status.code(), Some(0), "{}", meaning(&output));
    let deduped = std::fs::read_to_string(dir.join("deduped_manifest.txt")).expect("a manifest");
    let first = records(&a).into_iter().next().expect("a record");
    let row = deduped
        .lines()
        .filter(|line| !line.starts_with('#'))
        .find(|line| {
            let fields: Vec<&str> = line.split('\t').collect();
            fields[2] == first[3] && fields[3] == first[4] && fields[4] == first[5]
        })
        .expect("the first position has a row");
    let fields: Vec<&str> = row.split('\t').collect();
    assert_eq!(
        (fields[0], fields[1], fields[5]),
        ("2", "1", "9"),
        "the deeper label did not win: {row}"
    );
}

#[test]
fn a_record_agreeing_on_some_keys_and_not_all_is_counted_and_kept_distinct() {
    let scratch = Scratch::new("asm-disagree");
    let a = corpus(&scratch, "a");
    let b = corpus(&scratch, "b");
    let doctored = rebuild(&b, |at, fields| {
        if at == 1 {
            fields[3] = String::from("0,0 9,9/9,8");
        }
    });
    let b_doctored = scratch.write("corpus-b-doctored.txt", &doctored);
    let dir = out_dir(&scratch, "out");
    let clean = out_dir(&scratch, "clean");
    let output = assemble(&dir, &[&a, &b_doctored]);
    assert_eq!(output.status.code(), Some(0), "{}", meaning(&output));
    let control = assemble(&clean, &[&a, &b]);
    assert_eq!(control.status.code(), Some(0), "{}", meaning(&control));
    let deduped = std::fs::read_to_string(dir.join("deduped_manifest.txt")).expect("a manifest");
    let baseline = std::fs::read_to_string(clean.join("deduped_manifest.txt")).expect("a manifest");
    assert_eq!(derived(&baseline, "key_disagreements"), "0");
    assert_eq!(derived(&deduped, "key_disagreements"), "1");
    let distinct = |text: &str| {
        derived(text, "distinct_positions")
            .parse::<usize>()
            .unwrap()
    };
    assert_eq!(
        distinct(&deduped),
        distinct(&baseline) + 1,
        "a disagreement is a distinct position"
    );
}

// ---------------------------------------------------------------------------
// THE VOID AND REFUSAL CLASSES (tools/SHELL_CHECKLIST.md item 12).
// ---------------------------------------------------------------------------

#[test]
fn a_corpus_whose_body_does_not_digest_to_its_header_is_a_void_and_writes_nothing() {
    let scratch = Scratch::new("asm-void");
    let a = corpus(&scratch, "a");
    let text = std::fs::read_to_string(&a).expect("readable");
    let tampered = scratch.write("corpus-tampered.txt", &format!("{text}0\t0\t-\n"));
    let dir = out_dir(&scratch, "out");
    let output = assemble(&dir, &[&tampered]);
    assert_eq!(output.status.code(), Some(2), "{}", meaning(&output));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("RUN VOID"),
        "{}",
        meaning(&output)
    );
    assert!(
        std::fs::read_dir(&dir)
            .expect("the directory")
            .next()
            .is_none(),
        "a void left a manifest behind"
    );
}

#[test]
fn an_existing_manifest_is_refused_by_name_and_the_pair_is_not_half_written() {
    let scratch = Scratch::new("asm-refuse");
    let a = corpus(&scratch, "a");
    let dir = out_dir(&scratch, "out");
    std::fs::write(dir.join("deduped_manifest.txt"), "somebody else's\n").expect("a collision");
    let output = assemble(&dir, &[&a]);
    assert_eq!(output.status.code(), Some(1), "{}", meaning(&output));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("deduped_manifest.txt"),
        "the refusal did not name the file: {}",
        meaning(&output)
    );
    assert!(
        !dir.join("raw_manifest.txt").exists(),
        "a refused run left the other half of the pair behind"
    );
}
