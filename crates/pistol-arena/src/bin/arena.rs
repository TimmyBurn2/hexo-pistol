use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use std::io::Write as _;

use pistol_arena::config::ArenaConfig;
use pistol_arena::error::ArenaError;
use pistol_arena::report::Written;
use pistol_arena::usage::USAGE;
use pistol_arena::{
    capture, capture_file, identity, openings, outpath, replay, replay_report, report, schedule,
    score, summary, transcript,
};

/// A run that produced a report but is not a measurement.
const RUN_FAILED: u8 = 1;
/// Anything refused before a game was played.
const REFUSED: u8 = 2;

fn main() -> ExitCode {
    let owned: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = owned.iter().map(String::as_str).collect();
    match dispatch(&words) {
        Ok(code) => code,
        Err(why) => {
            eprintln!("arena: {why}");
            ExitCode::from(REFUSED)
        }
    }
}

fn dispatch(words: &[&str]) -> Result<ExitCode, String> {
    enum Mode {
        Play(PathBuf),
        Replay(PathBuf, usize),
        Capture(PathBuf, u64),
    }
    let (mode, out_path) = match words {
        ["--help" | "-h"] => {
            print!("{USAGE}");
            return Ok(ExitCode::SUCCESS);
        }
        ["--config", config, "--out", out] | ["--out", out, "--config", config] => {
            (Mode::Play(PathBuf::from(config)), PathBuf::from(out))
        }
        ["--replay", source, "--out", out, "--workers", workers] => (
            Mode::Replay(PathBuf::from(source), workers_of(workers)?),
            PathBuf::from(out),
        ),
        ["--capture", source, "--out", out, "--label-nodes", nodes] => (
            Mode::Capture(PathBuf::from(source), count_of(nodes, "label node count")?),
            PathBuf::from(out),
        ),
        _ => {
            return Err(format!(
                "--config and --out are both required, or --replay, --out and --workers, or \
                 --capture, --out and --label-nodes, each in that order\n\n{USAGE}"
            ));
        }
    };
    // The claim IS the existence check: one O_EXCL syscall, no window for a
    // second run to slip through (docs/decisions.md D-200).
    let claimed = outpath::claim(&out_path).map_err(|error| error.to_string())?;
    let outcome = match &mode {
        Mode::Play(config) => run(config, &out_path, claimed),
        Mode::Replay(source, workers) => replay_pass(source, &out_path, claimed, *workers),
        Mode::Capture(source, nodes) => capture_pass(source, &out_path, claimed, *nodes),
    };
    match outcome {
        Ok(code) => Ok(code),
        Err(error) => {
            // Exit 2 promises "no report at all". This branch is every
            // pre-game refusal AND a report write that failed partway — in
            // both cases the file holds no report, and it is this process's
            // own claim (outpath::abandon says why removing it is safe). A
            // failed removal is reported, never swallowed.
            if let Err(cleanup) = outpath::abandon(&out_path) {
                eprintln!("arena: {cleanup}");
            }
            Err(error.to_string())
        }
    }
}

/// The worker count, with its SPELLING validated and not merely its value.
///
/// `+4`, ` 4` and `04` all parse to four and would land in a document's timing
/// block unnormalised, describing a run nobody can reproduce by copying the line
/// back (tools/SHELL_CHECKLIST.md item 8).
fn workers_of(word: &str) -> Result<usize, String> {
    let parsed = usize::try_from(count_of(word, "worker count")?)
        .map_err(|_| format!("`{word}` is more workers than this machine can address"))?;
    if parsed == 0 {
        return Err(String::from("--workers 0 would replay nothing at all"));
    }
    Ok(parsed)
}

/// A count off the command line, with its SPELLING validated and not merely its
/// value.
///
/// `+4`, ` 4` and `04` all parse to four and would land in a document
/// describing a run nobody can reproduce by copying the line back
/// (tools/SHELL_CHECKLIST.md item 8).
fn count_of(word: &str, what: &str) -> Result<u64, String> {
    let parsed: u64 = word
        .parse()
        .map_err(|_| format!("`{word}` is not a {what}\n\n{USAGE}"))?;
    if parsed.to_string() != word {
        return Err(format!(
            "`{word}` is a {what} spelled a way this program will not echo back; write it as \
             `{parsed}`\n\n{USAGE}"
        ));
    }
    if parsed == 0 {
        return Err(format!("a {what} of zero asks for nothing at all"));
    }
    Ok(parsed)
}

/// One report, read back as the run it describes.
///
/// A REGULAR FILE, CHECKED BEFORE IT IS READ. `fs::read` on a FIFO blocks until
/// a writer appears, with no channel yet in existence and so no watchdog to end
/// it — a hang where a refusal belongs (docs/decisions.md D-252's sibling case
/// in `identity::digest_of`).
fn read_report(source: &Path) -> Result<transcript::Transcript, ArenaError> {
    let meta = std::fs::metadata(source)
        .map_err(|io| ArenaError::io(format!("reading {}", source.display()), io))?;
    if !meta.is_file() {
        return Err(ArenaError::config(
            "replay report",
            format!("{} is not a regular file", source.display()),
        ));
    }
    let bytes = std::fs::read(source)
        .map_err(|io| ArenaError::io(format!("reading {}", source.display()), io))?;
    let source_sha256 = pistol_cli::sha256::sha256_hex(&bytes);
    let text = std::str::from_utf8(&bytes).map_err(|why| {
        ArenaError::config(
            "replay report",
            format!("{} is not UTF-8: {why}", source.display()),
        )
    })?;
    transcript::read(text, source_sha256)
}

/// Walk one report position by position, asking each at the label budget.
fn capture_pass(
    source: &Path,
    out_path: &Path,
    mut claimed: std::fs::File,
    label_nodes: u64,
) -> Result<ExitCode, ArenaError> {
    let transcript = read_report(source)?;
    let go_line = capture::label_go_line(label_nodes);
    let records = capture::run(&transcript, label_nodes)?;
    let rendered = capture_file::render(&transcript, &go_line, &records);
    claimed
        .write_all(rendered.as_bytes())
        .and_then(|()| claimed.flush())
        .map_err(|io| ArenaError::io(format!("writing {}", out_path.display()), io))?;
    println!(
        "arena: captured {} position(s) from {} game(s) at {go_line}",
        records.len(),
        transcript.games.len()
    );
    println!(
        "{}",
        capture_file::manifest_row(&transcript, &go_line, &rendered, out_path)
    );
    println!("arena: capture written to {}", out_path.display());
    Ok(ExitCode::SUCCESS)
}

/// Re-drive one report's games through the engines that report attests.
fn replay_pass(
    source: &Path,
    out_path: &Path,
    mut claimed: std::fs::File,
    workers: usize,
) -> Result<ExitCode, ArenaError> {
    let transcript = read_report(source)?;
    replay::verify_engines(&transcript)?;

    let (outcome, played) = replay::run(&transcript, workers);
    let failure = outcome.err();
    let rendered = replay_report::render(&transcript, &played, failure.as_ref());
    claimed
        .write_all(rendered.as_bytes())
        .and_then(|()| claimed.flush())
        .map_err(|io| ArenaError::io(format!("writing {}", out_path.display()), io))?;

    let covered = played.covered();
    let total = played.games.len();
    match failure {
        Some(error) => {
            eprintln!("arena: {error}");
            eprintln!(
                "arena: {covered} of {total} game(s) were replayed; a criterion over some of a \
                 report's games is not one anybody registered, so {} is a diagnostic",
                out_path.display()
            );
            Ok(ExitCode::from(RUN_FAILED))
        }
        None => {
            let divergences = played.divergences();
            println!(
                "arena: replayed {covered} of {total} game(s) from {}, {divergences} \
                 divergence(s)",
                source.display()
            );
            println!("arena: replay written to {}", out_path.display());
            Ok(if divergences == 0 && covered == total {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(RUN_FAILED)
            })
        }
    }
}

fn run(
    config_path: &Path,
    out_path: &Path,
    mut claimed: std::fs::File,
) -> Result<ExitCode, ArenaError> {
    let config = ArenaConfig::load(config_path)?;
    let config_sha = identity::digest_of(config_path)?;
    let openings = openings::load(
        &config.run.openings_file,
        config.run.openings_take,
        config.run.openings_skip,
        config.run.turn_cap,
    )?;
    let go_line = config
        .budget
        .go_line()
        .unwrap_or_else(|| unreachable!("validate refuses a movetime budget before this point"));

    // The instrument, by content. A path is not identity: `target/release/pistol`
    // is a different program after every build (docs/decisions.md D-147), and
    // the identity closes over the eval weight table the engine itself digests
    // (docs/decisions.md D-198). Every later spawn is re-verified against this
    // capture (docs/decisions.md D-199).
    let identities = [
        identity::capture(&config.engine_a, config.run.hang_timeout_ms)?,
        identity::capture(&config.engine_b, config.run.hang_timeout_ms)?,
    ];

    let started = Instant::now();
    let (outcome, played) = schedule::run(&config, &openings, &identities, &go_line);
    let wall_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    let failure = outcome.err();

    let written = Written {
        config: &config,
        config_sha256: &config_sha,
        openings: &openings,
        identities: &identities,
        records: &played.records,
        wall_ms,
        discarded: played.discarded,
        aborted: failure.as_ref(),
    };
    let rendered = report::render(&written);
    claimed
        .write_all(rendered.as_bytes())
        .and_then(|()| claimed.flush())
        .map_err(|io| ArenaError::io(format!("writing {}", out_path.display()), io))?;

    match failure {
        Some(error) => {
            eprintln!("arena: {error}");
            eprintln!(
                "arena: the {} game(s) that finished are in {} as a diagnostic, not as a sample",
                played.records.len(),
                out_path.display()
            );
            Ok(ExitCode::from(RUN_FAILED))
        }
        None => {
            print!("{}", summary::render(&config, &played.records, wall_ms));
            println!("arena: report written to {}", out_path.display());
            let clean = score::tally(&played.records).forfeits == 0;
            Ok(if clean {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(RUN_FAILED)
            })
        }
    }
}
