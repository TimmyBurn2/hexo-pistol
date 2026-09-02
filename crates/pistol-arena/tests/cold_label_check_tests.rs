mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{ARENA, ConfigSpec, Ran, STUB, Scratch, openings_prefix, repo, run};

const OPENINGS: usize = 2;
const TURN_CAP: u32 = 8;
const LABEL_NODES: &str = "5000";

/// What one run of the pipeline leaves behind for the checker to read.
struct Staged {
    capture: PathBuf,
    engine_config: PathBuf,
}

/// A self-play report and the capture taken from it.
fn staged(scratch: &Scratch, tag: &str) -> Staged {
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
    let output = Command::new(ARENA)
        .arg("--capture")
        .arg(&report)
        .arg("--out")
        .arg(&capture)
        .arg("--label-nodes")
        .arg(LABEL_NODES)
        .output()
        .expect("the arena binary runs");
    assert!(
        capture.exists(),
        "the capture the checker reads was not written: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Staged {
        capture,
        engine_config,
    }
}

/// Drive the SHIPPED script, never a copy of it
/// (tools/SHELL_CHECKLIST.md item 10).
fn check(capture: &Path, engine_config: &Path, stride: &str) -> Output {
    partitioned_check(capture, engine_config, stride, "all")
}

/// The same script with the cache class named. `--partition` is REQUIRED and has
/// no default: a default would answer about `all` while a criterion said `hits`
/// (docs/experiments/wp21_prereg.md revision 4, T-A1 and T-A2).
fn partitioned_check(
    capture: &Path,
    engine_config: &Path,
    stride: &str,
    partition: &str,
) -> Output {
    Command::new("python3")
        .arg(repo().join("tools/cold_label_check.py"))
        .arg("--capture")
        .arg(capture)
        .arg("--binary")
        .arg(STUB)
        .arg("--engine-config")
        .arg(engine_config)
        .arg("--stride")
        .arg(stride)
        .arg("--partition")
        .arg(partition)
        .output()
        .expect("the cold-label checker runs")
}

/// The three codes spelled out, so a failure message says what the code the
/// test did NOT get would have meant (tools/SHELL_CHECKLIST.md item 12,
/// obligation 3).
fn meaning(output: &Output) -> String {
    format!(
        "exit {:?} — 0 is `every sampled record agrees`, 1 is `a sampled record disagrees`, \
         2 is `RUN VOID, no answer was taken`.\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// Rewrite a capture's body through `edit`, with the header's digest brought
/// back into agreement so the doctored file is one the checker will read.
fn rebuild(capture: &Path, edit: impl Fn(usize, &mut Vec<String>)) -> String {
    let text = std::fs::read_to_string(capture).expect("the capture is readable");
    let header: String = text
        .lines()
        .take_while(|line| !line.starts_with("# body_sha256 "))
        .map(|line| format!("{line}\n"))
        .collect();
    let body: String = pistol_cli::corpus::emit::body_of(&text)
        .expect("a capture carries a body digest")
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(at, line)| {
            let mut fields: Vec<String> = line.split('\t').map(str::to_string).collect();
            edit(at, &mut fields);
            format!("{}\n", fields.join("\t"))
        })
        .collect();
    format!(
        "{header}# body_sha256 {}\n{body}",
        pistol_cli::sha256::sha256_hex(body.as_bytes())
    )
}

/// A capture holding only its first record, with the header's digest brought
/// back into agreement.
///
/// The first ask at a `position` is always a MISS, so such a file has an empty
/// HIT class — the degenerate case a criterion must not answer vacuously.
fn first_record_only(capture: &Path) -> String {
    let text = std::fs::read_to_string(capture).expect("the capture is readable");
    let header: String = text
        .lines()
        .take_while(|line| !line.starts_with("# body_sha256 "))
        .map(|line| format!("{line}\n"))
        .collect();
    let body = pistol_cli::corpus::emit::body_of(&text)
        .expect("a capture carries a body digest")
        .split('\n')
        .find(|line| !line.is_empty())
        .map(|line| format!("{line}\n"))
        .expect("the capture holds at least one record");
    format!(
        "{header}# body_sha256 {}\n{body}",
        pistol_cli::sha256::sha256_hex(body.as_bytes())
    )
}

// ---------------------------------------------------------------------------
// THE CONTROL RUN. A checker that refuses everything would pass every refusal
// test in this file and answer nothing (tools/SHELL_CHECKLIST.md item 10).
// ---------------------------------------------------------------------------

#[test]
fn a_capture_a_fresh_process_reproduces_is_reported_as_agreeing() {
    let scratch = Scratch::new("cold-agree");
    let staged = staged(&scratch, "agree");
    let output = check(&staged.capture, &staged.engine_config, "1");
    assert_eq!(
        output.status.code(),
        Some(0),
        "a capture whose every record a fresh process reproduces was not reported as \
         agreeing: {}",
        meaning(&output)
    );
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert!(
        stdout.contains("agree byte for byte"),
        "the checker did not say what it found: {stdout}"
    );
}

#[test]
fn a_record_a_fresh_process_does_not_reproduce_is_a_disagreement_naming_it() {
    let scratch = Scratch::new("cold-disagree");
    let staged = staged(&scratch, "disagree");
    // The captured totals line, edited: the engine still answers what it
    // answered, so the two no longer agree and the checker must say so.
    let doctored = rebuild(&staged.capture, |at, fields| {
        if at == 0 {
            fields[3] = fields[3].replace("depth_turns 1", "depth_turns 9");
        }
    });
    let path = scratch.write("capture-doctored.txt", &doctored);
    let output = check(&path, &staged.engine_config, "1");
    assert_eq!(
        output.status.code(),
        Some(1),
        "an edited totals line was not reported as a disagreement: {}",
        meaning(&output)
    );
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert!(
        stdout.contains("DISAGREEMENT") && stdout.contains("record 0"),
        "the checker did not name the record that disagreed: {stdout}"
    );
}

#[test]
fn a_bestmove_a_fresh_process_does_not_reproduce_is_a_disagreement_naming_it() {
    let scratch = Scratch::new("cold-best");
    let staged = staged(&scratch, "best");
    let doctored = rebuild(&staged.capture, |at, fields| {
        if at == 0 {
            fields[4] = String::from("bestmove 7,7");
        }
    });
    let path = scratch.write("capture-doctored-best.txt", &doctored);
    let output = check(&path, &staged.engine_config, "1");
    assert_eq!(
        output.status.code(),
        Some(1),
        "an edited bestmove line was not reported as a disagreement: {}",
        meaning(&output)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("bestmove"),
        "the checker did not name the field that disagreed: {}",
        meaning(&output)
    );
}

// ---------------------------------------------------------------------------
// THE VOID CLASS, which is not the disagreement class
// (tools/SHELL_CHECKLIST.md item 12).
// ---------------------------------------------------------------------------

#[test]
fn a_capture_whose_body_does_not_digest_to_its_header_is_a_void_and_not_a_disagreement() {
    let scratch = Scratch::new("cold-void-digest");
    let staged = staged(&scratch, "voiddigest");
    let text = std::fs::read_to_string(&staged.capture).expect("readable");
    let tampered = format!("{text}0\t0\tposition start\tinfo totals x\tbestmove 0,0\n");
    let path = scratch.write("capture-tampered.txt", &tampered);
    let output = check(&path, &staged.engine_config, "1");
    assert_eq!(
        output.status.code(),
        Some(2),
        "a capture whose body does not digest to its header was not reported as a VOID; \
         reading it as a disagreement is the defect item 12 exists for: {}",
        meaning(&output)
    );
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(
        stderr.contains("RUN VOID") && stderr.contains("NOT a disagreement"),
        "the void did not say it was not a disagreement: {stderr}"
    );
}

#[test]
fn a_stride_spelled_a_way_this_program_will_not_echo_back_is_a_void() {
    let scratch = Scratch::new("cold-void-stride");
    let staged = staged(&scratch, "voidstride");
    let output = check(&staged.capture, &staged.engine_config, "01");
    assert_eq!(
        output.status.code(),
        Some(2),
        "`01` was accepted as a stride, and a receipt would then quote a number nobody can \
         copy back (tools/SHELL_CHECKLIST.md item 8): {}",
        meaning(&output)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("will not echo back"),
        "the void did not name the spelling: {}",
        meaning(&output)
    );
}

#[test]
fn an_engine_binary_that_is_not_a_file_is_a_void() {
    let scratch = Scratch::new("cold-void-binary");
    let staged = staged(&scratch, "voidbinary");
    let missing = scratch.path("no-such-engine");
    let output = Command::new("python3")
        .arg(repo().join("tools/cold_label_check.py"))
        .arg("--capture")
        .arg(&staged.capture)
        .arg("--binary")
        .arg(&missing)
        .arg("--engine-config")
        .arg(&staged.engine_config)
        .arg("--stride")
        .arg("1")
        .output()
        .expect("the cold-label checker runs");
    assert_eq!(
        output.status.code(),
        Some(2),
        "an engine that does not exist was not reported as a VOID: {}",
        meaning(&output)
    );
}

// ---------------------------------------------------------------------------
// THE SAMPLE RULE, which the pre-registration registers by its stride.
// ---------------------------------------------------------------------------

#[test]
fn the_sample_is_every_stride_th_record_and_the_run_says_which() {
    let scratch = Scratch::new("cold-stride");
    let staged = staged(&scratch, "stride");
    let text = std::fs::read_to_string(&staged.capture).expect("readable");
    let total = pistol_cli::corpus::emit::body_of(&text)
        .expect("a body")
        .split('\n')
        .filter(|line| !line.is_empty())
        .count();
    let output = check(&staged.capture, &staged.engine_config, "3");
    assert_eq!(
        output.status.code(),
        Some(0),
        "the strided sample did not agree: {}",
        meaning(&output)
    );
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let expected = total.div_ceil(3);
    assert!(
        stdout.contains(&format!("multiple of 3, which is {expected} of them")),
        "the run did not state the sample it took ({expected} of {total}): {stdout}"
    );
}

#[test]
fn the_class_a_sample_was_drawn_from_is_on_the_line_it_is_reported_on() {
    let scratch = Scratch::new("cold-partition-named");
    let staged = staged(&scratch, "named");
    for partition in ["hits", "misses", "all"] {
        let output = partitioned_check(&staged.capture, &staged.engine_config, "1", partition);
        let said = String::from_utf8_lossy(&output.stdout).to_string();
        // Two invocations of one shape cannot be told apart in a run log unless
        // the class is on the line, which is the whole point of the amendment.
        assert!(
            said.contains(&partition.to_uppercase()),
            "the {partition} run did not name its class. {said}"
        );
    }
}

#[test]
fn the_two_classes_partition_the_records_and_neither_is_the_whole() {
    let scratch = Scratch::new("cold-partition-splits");
    let staged = staged(&scratch, "splits");
    let counted = |partition: &str| -> usize {
        let output = partitioned_check(&staged.capture, &staged.engine_config, "1", partition);
        let said = String::from_utf8_lossy(&output.stdout).to_string();
        let marker = format!("are {}", partition.to_uppercase());
        let at = said
            .find(&marker)
            .unwrap_or_else(|| panic!("no `{marker}` in {said}"));
        said[..at]
            .rsplit(|c: char| !c.is_ascii_digit())
            .find(|word| !word.is_empty())
            .and_then(|word| word.parse().ok())
            .unwrap_or_else(|| panic!("no count before `{marker}` in {said}"))
    };
    let (hits, misses, all) = (counted("hits"), counted("misses"), counted("all"));
    assert_eq!(
        hits + misses,
        all,
        "the two classes must partition the records"
    );
    assert!(misses > 0, "a self-match capture always asks a first time");
}

#[test]
fn a_partition_the_capture_cannot_fill_is_a_void_and_not_a_pass() {
    let scratch = Scratch::new("cold-partition-empty");
    let staged = staged(&scratch, "empty");
    // One record cannot be a hit: the first ask at a position is always a miss.
    let capture = scratch.write("one-record.txt", &first_record_only(&staged.capture));
    let output = partitioned_check(&capture, &staged.engine_config, "1", "hits");
    assert_eq!(
        output.status.code(),
        Some(2),
        "an empty class is a VOID (exit 2) and not a vacuous `0 of 0 agree`. {}",
        meaning(&output)
    );
}
