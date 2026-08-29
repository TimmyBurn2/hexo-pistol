mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};

/// A stand-in for `bench_positions_v1.txt` — the same KIND (position-verb
/// tails, ` # …` commentary carrying `stones N`), never the registered
/// workload (`docs/process.md`, dry-run discipline). Entry 1 is malformed:
/// `BOGUS` is not a `q,r` pair, so the engine refuses the `position` line and
/// the `go` that follows measures the board `newgame` just emptied.
const TAIL_WITH_A_REFUSAL: &str = "\
# stand-in, not the registered workload
start moves 0,0 -1,1/1,0 0,1/0,2 # src standin stones 5
start moves 0,0 -1,1/BOGUS 0,1/0,2 # src standin stones 5
start moves 0,0 -1,1/1,0 0,1/0,2 -1,0/1,-1 # src standin stones 7
";

/// The same stand-in with the malformed entry repaired: the CONTROL, so a
/// refusal below cannot come from a script that refuses everything
/// (`tools/SHELL_CHECKLIST.md` item 10).
const TAIL_CLEAN: &str = "\
# stand-in, not the registered workload
start moves 0,0 -1,1/1,0 0,1/0,2 # src standin stones 5
start moves 0,0 -1,1/1,0 0,1/0,2 -1,0/1,-1 # src standin stones 7
";

/// A stand-in for `spread_v1.txt` — the OTHER committed bench fixture's shape:
/// whole `position …` protocol lines, each preceded by a bare `stones N` line.
/// Under `--grammar tail` every entry of this file is refused, because the
/// block would send `position position start moves …`; that refusal is the
/// point of the second test below.
const LINE_CLEAN: &str = "\
# stand-in, not the registered workload
stones 5
position start moves 0,0 8,0/16,0 24,0/32,0
stones 7
position start moves 0,0 8,0/16,0 24,0/32,0 40,0/48,0
";

/// An `--engine` that answers every `go` with TWO totals lines. A sweep that
/// aggregates whichever one `sed` happened to keep is a sweep whose number
/// nobody chose.
const STUB_TWO_TOTALS: &str = "\
#!/usr/bin/env bash
while IFS= read -r line; do
  case \"$line\" in
    'go '*) echo 'info totals nodes 1000 time 10'; echo 'info totals nodes 2000 time 20' ;;
    quit) break ;;
  esac
done
";

/// An `--engine` that refuses on STDERR rather than on stdout. The shipped
/// engine refuses on stdout (docs/decisions.md D-88) and that is what the
/// copied block dropped, but a guard that watched only stdout would be a guard
/// written to one stream's habit rather than to the class.
const STUB_REFUSES_ON_STDERR: &str = "\
#!/usr/bin/env bash
while IFS= read -r line; do
  case \"$line\" in
    'position '*) echo 'error Protocol: refused on stderr' >&2 ;;
    'go '*) echo 'info totals nodes 1000 time 10' ;;
    quit) break ;;
  esac
done
";

/// An `--engine` that answers nothing at all: a sweep that reports a clean run
/// over zero measurements is the EXIT-0-WRONG-ANSWER class with the numbers
/// removed instead of replaced.
const STUB_SILENT: &str = "\
#!/usr/bin/env bash
while IFS= read -r line; do
  case \"$line\" in
    quit) break ;;
  esac
done
";

fn engine() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_pistol"))
}

fn stub(name: &str, source: &str) -> PathBuf {
    let path = scratch(name).join("stub-engine");
    std::fs::write(&path, source).expect("the stub engine writes");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
            .expect("the stub engine is executable");
    }
    path
}

fn fixture(name: &str, text: &str) -> PathBuf {
    let path = scratch(name).join("standin.txt");
    std::fs::write(&path, text).expect("the stand-in fixture writes");
    path
}

/// Run the SHIPPED script. The budget is the smallest the protocol admits: the
/// first iteration is not interruptible (docs/decisions.md D-74), so `nodes 1`
/// costs one full-width iteration per invocation and nothing more.
fn bench_block(engine: &Path, fixture: &Path, grammar: &str, extra: &[&str]) -> Output {
    let mut command = Command::new("bash");
    command
        .arg(repo("tools/bench_block.sh"))
        .args(["--engine", &engine.display().to_string()])
        .args([
            "--config",
            &repo("configs/instrument_staged_v0.toml")
                .display()
                .to_string(),
        ])
        .args(["--fixture", &fixture.display().to_string()])
        .args(["--grammar", grammar])
        .args(["--budget", "nodes 1"])
        .args(["--reps", "1"])
        .args(extra)
        .current_dir(repo(""));
    command.output().expect("the bench block runs")
}

fn said(ran: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&ran.stdout),
        String::from_utf8_lossy(&ran.stderr)
    )
}

#[test]
fn an_entry_the_engine_refuses_fails_the_sweep_instead_of_measuring_the_empty_board() {
    // THE CONTROL: the same stand-in with the malformed entry repaired has to
    // complete, or the refusal below proves nothing.
    let clean = fixture("bench-block-control", TAIL_CLEAN);
    let control = bench_block(&engine(), &clean, "tail", &[]);
    let seen = said(&control);
    assert!(
        control.status.success(),
        "a clean stand-in sweeps to completion:\n{seen}"
    );
    assert!(
        seen.contains("done: 2 entries x 1 reps = 2 totals lines, 0 refused"),
        "and says so with its own counts:\n{seen}"
    );

    let refused = fixture("bench-block-refusal", TAIL_WITH_A_REFUSAL);
    let ran = bench_block(&engine(), &refused, "tail", &[]);
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "a refused entry is a FAIL, not a void (2) and not a pass (0):\n{out}"
    );
    assert!(
        out.contains("entry 1 rep 1 was REFUSED"),
        "named by the entry that was refused:\n{out}"
    );
    // The defect this guard exists for: the engine printed a well-formed
    // totals line for the empty board after the refusal. Entry 1 must not
    // appear as a record.
    assert!(
        !out.contains("record entry 1 "),
        "and no record line is emitted for the entry that never loaded:\n{out}"
    );
}

#[test]
fn the_position_prefixed_fixture_shape_loads_under_line_and_is_refused_under_tail() {
    let spread = fixture("bench-block-grammar", LINE_CLEAN);

    let ran = bench_block(&engine(), &spread, "line", &[]);
    let out = said(&ran);
    assert!(
        ran.status.success(),
        "spread_v1.txt's shape sweeps clean under --grammar line:\n{out}"
    );
    assert!(
        out.contains("done: 2 entries x 1 reps = 2 totals lines, 0 refused"),
        "with both entries loaded and none refused:\n{out}"
    );
    assert!(
        out.contains("record entry 0 stones 5 "),
        "and the `stones N` line preceding an entry is carried onto its record:\n{out}"
    );

    // The same bytes under the wrong grammar. This is what the copied block
    // did unconditionally, and it exited 0 with eight empty-board totals
    // lines; here it is a refusal that names the reason.
    let wrong = bench_block(&engine(), &spread, "tail", &[]);
    let out = said(&wrong);
    assert_eq!(
        wrong.status.code(),
        Some(1),
        "and the same file under --grammar tail is refused, never measured:\n{out}"
    );
}

#[test]
fn a_refusal_printed_on_stderr_is_caught_the_same_as_one_printed_on_stdout() {
    let engine = stub("bench-block-stderr", STUB_REFUSES_ON_STDERR);
    let clean = fixture("bench-block-stderr-fixture", TAIL_CLEAN);
    let ran = bench_block(&engine, &clean, "tail", &[]);
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "a refusal on stderr fails the sweep too:\n{out}"
    );
    assert!(
        out.contains("was REFUSED"),
        "by the same named refusal:\n{out}"
    );
}

#[test]
fn an_invocation_that_answers_two_totals_lines_is_refused_rather_than_aggregated() {
    let engine = stub("bench-block-two-totals", STUB_TWO_TOTALS);
    let clean = fixture("bench-block-two-totals-fixture", TAIL_CLEAN);
    let ran = bench_block(&engine, &clean, "tail", &[]);
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "two totals lines for one search is a refusal:\n{out}"
    );
    assert!(
        out.contains("produced 2 `info totals` lines, wanted exactly 1"),
        "named by the count it saw and the count it wanted:\n{out}"
    );
}

#[test]
fn an_engine_that_answers_nothing_is_refused_rather_than_reported_as_a_clean_sweep() {
    let engine = stub("bench-block-silent", STUB_SILENT);
    let clean = fixture("bench-block-silent-fixture", TAIL_CLEAN);
    let ran = bench_block(&engine, &clean, "tail", &[]);
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "zero totals lines is a refusal, not a sweep of zero measurements:\n{out}"
    );
    assert!(
        out.contains("produced 0 `info totals` lines, wanted exactly 1"),
        "named by the count:\n{out}"
    );
}

#[test]
fn an_engine_that_is_not_there_is_a_void_and_not_a_regression() {
    let missing = scratch("bench-block-void").join("no-such-engine");
    let clean = fixture("bench-block-void-fixture", TAIL_CLEAN);
    let ran = bench_block(&missing, &clean, "tail", &[]);
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(2),
        "exit 2 is `no answer was taken`; exit 1 would have read as a bench \
         regression and exit 0 as a bench that passed \
         (tools/SHELL_CHECKLIST.md item 12):\n{out}"
    );
    assert!(
        out.contains("RUN VOID"),
        "and it says so in the gate's own vocabulary:\n{out}"
    );
}

#[test]
fn a_reps_spelling_bash_reads_as_octal_is_refused_before_any_search_runs() {
    let clean = fixture("bench-block-reps", TAIL_CLEAN);
    let mut command = Command::new("bash");
    let ran = command
        .arg(repo("tools/bench_block.sh"))
        .args(["--engine", &engine().display().to_string()])
        .args([
            "--config",
            &repo("configs/instrument_staged_v0.toml")
                .display()
                .to_string(),
        ])
        .args(["--fixture", &clean.display().to_string()])
        .args(["--grammar", "tail"])
        .args(["--budget", "nodes 1"])
        .args(["--reps", "010"])
        .current_dir(repo(""))
        .output()
        .expect("the bench block runs");
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "`010` is octal 8 to bash and decimal 10 to a reader; the spelling is \
         refused rather than recorded (tools/SHELL_CHECKLIST.md item 8):\n{out}"
    );
    assert!(
        out.contains("no leading zero"),
        "named by the spelling rule it broke:\n{out}"
    );
    assert!(
        !out.contains("record entry"),
        "and nothing was measured before the refusal:\n{out}"
    );
}

#[test]
fn a_budget_the_protocol_does_not_spell_is_refused_before_any_search_runs() {
    let clean = fixture("bench-block-budget", TAIL_CLEAN);
    let mut command = Command::new("bash");
    let ran = command
        .arg(repo("tools/bench_block.sh"))
        .args(["--engine", &engine().display().to_string()])
        .args([
            "--config",
            &repo("configs/instrument_staged_v0.toml")
                .display()
                .to_string(),
        ])
        .args(["--fixture", &clean.display().to_string()])
        .args(["--grammar", "tail"])
        .args(["--budget", "plies 4"])
        .args(["--reps", "1"])
        .current_dir(repo(""))
        .output()
        .expect("the bench block runs");
    let out = said(&ran);
    assert_eq!(
        ran.status.code(),
        Some(1),
        "an unspellable budget is refused here, so it is never confused with a \
         refused POSITION at the per-entry guard:\n{out}"
    );
    assert!(
        out.contains("--budget reads"),
        "named by the argument that was wrong:\n{out}"
    );
}
