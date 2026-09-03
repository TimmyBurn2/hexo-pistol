use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use std::io::Write as _;

use pistol_arena::config::ArenaConfig;
use pistol_arena::error::ArenaError;
use pistol_arena::label_cache::LabelCache;
use pistol_arena::report::Written;
use pistol_arena::usage::{USAGE, capture_tail, count_of, usage_error, workers_of};
use pistol_arena::{
    identity, openings, outpath, passes, replay, replay_report, report, schedule, score, summary,
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
        Capture(PathBuf, u64, bool, LabelCache),
        Labels(PathBuf, PathBuf),
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
        [
            "--capture",
            source,
            "--out",
            out,
            "--label-nodes",
            nodes,
            tail @ ..,
        ] => {
            let (census, cache) = capture_tail(tail)?;
            (
                Mode::Capture(
                    PathBuf::from(source),
                    count_of(nodes, "label node count")?,
                    census,
                    cache,
                ),
                PathBuf::from(out),
            )
        }
        ["--labels", capture, "--report", report, "--out", out] => (
            Mode::Labels(PathBuf::from(capture), PathBuf::from(report)),
            PathBuf::from(out),
        ),
        _ => return Err(usage_error()),
    };
    // The claim IS the existence check: one O_EXCL syscall, no window for a
    // second run to slip through (docs/decisions.md D-200).
    let claimed = outpath::claim(&out_path).map_err(|error| error.to_string())?;
    // The census file is claimed HERE, beside `--out` and before any game, for
    // the same reason `--out` is (docs/decisions.md D-200). A claim that failed
    // leaves no report either: the `--out` claim is given back first, so the
    // refusal is the same "no report at all" exit 2 every pre-game refusal is.
    let census = match &mode {
        Mode::Capture(_, _, true, _) => {
            let path = outpath::census_path(&out_path).map_err(|error| error.to_string())?;
            match outpath::claim(&path) {
                Ok(file) => Some((path, file)),
                Err(error) => {
                    if let Err(cleanup) = outpath::abandon(&out_path) {
                        eprintln!("arena: {cleanup}");
                    }
                    return Err(error.to_string());
                }
            }
        }
        _ => None,
    };
    // The path is copied out before the pair is handed over, because the
    // failure arm below has to name a file it no longer owns.
    let census_cleanup: Option<PathBuf> = census.as_ref().map(|(path, _)| path.clone());
    let outcome = match &mode {
        Mode::Play(config) => run(config, &out_path, claimed),
        Mode::Replay(source, workers) => replay_pass(source, &out_path, claimed, *workers),
        Mode::Capture(source, nodes, _, cache) => {
            passes::capture(source, &out_path, claimed, *nodes, census, *cache)
        }
        Mode::Labels(capture, report) => passes::labels(capture, report, &out_path, claimed),
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
            if let Some(path) = &census_cleanup
                && let Err(cleanup) = outpath::abandon(path)
            {
                eprintln!("arena: {cleanup}");
            }
            Err(error.to_string())
        }
    }
}

/// Re-drive one report's games through the engines that report attests.
fn replay_pass(
    source: &Path,
    out_path: &Path,
    mut claimed: std::fs::File,
    workers: usize,
) -> Result<ExitCode, ArenaError> {
    let transcript = passes::read_report(source)?;
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
