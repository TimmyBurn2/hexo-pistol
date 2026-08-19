//! `arena` — play two pistol configurations against each other and say which
//! one the evidence favours.
//!
//! Every strength claim in this project comes from a run of this program
//! (CLAUDE.md rule 6). It therefore refuses more than it accepts: a wall-clock
//! budget, an engine that is not in instrument mode, an openings file that does
//! not match its own digest, and a run whose engines stopped answering are each
//! a named refusal rather than a number.
//!
//! Exit: 0 the run completed and no game was forfeited, 1 the run was abandoned
//! or a game was forfeited — in both cases a report was still written — or 2 a
//! document this build refuses, in which case there is no report at all.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use std::io::Write as _;

use pistol_arena::config::ArenaConfig;
use pistol_arena::error::ArenaError;
use pistol_arena::report::Written;
use pistol_arena::{identity, openings, outpath, report, schedule, score, summary};

/// What this program does, and what it refuses to guess.
const USAGE: &str = "\
arena — the paired-openings SPRT judge for pistol

usage:
  arena --config <path> --out <path>

  --config  an arena config. Always explicit: there is no default path and no
            built-in configuration (CLAUDE.md rule 1). It states the openings,
            the budget, the turn cap, the worker count and the SPRT bounds.
  --out     where to write the report. CLAIMED exclusively at dispatch
            (create_new/O_EXCL), so an existing file — a previous report, or
            another run in flight — is refused by name before any game: a run
            that silently overwrote a report would destroy the evidence for a
            claim somebody has already made. A refusal before any game removes
            the empty claim again. Match logs are artifacts and are never
            written inside the repository (CLAUDE.md rule 8).

  Only instrument budgets are accepted. A `movetime` budget is refused by name:
  wall-clock is not reproducible, and it is not even a ceiling — the first
  deepening iteration cannot be interrupted (docs/decisions.md D-74, D-95).

  The verdict is read off the PAIR-level LLR. The game-level LLR is reported
  beside it as a diagnostic and is not the verdict (docs/decisions.md D-154).

exit: 0 completed cleanly, 1 abandoned or forfeited (report still written),
      2 a document this build refuses (no report).
";

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
    let (config_path, out_path) = match words {
        ["--help" | "-h"] => {
            print!("{USAGE}");
            return Ok(ExitCode::SUCCESS);
        }
        ["--config", config, "--out", out] | ["--out", out, "--config", config] => {
            (PathBuf::from(config), PathBuf::from(out))
        }
        _ => return Err(format!("--config and --out are both required\n\n{USAGE}")),
    };
    // The claim IS the existence check: one O_EXCL syscall, no window for a
    // second run to slip through (docs/decisions.md D-200).
    let claimed = outpath::claim(&out_path).map_err(|error| error.to_string())?;
    match run(&config_path, &out_path, claimed) {
        Ok(code) => Ok(code),
        Err(error) => {
            // Exit 2 promises "no report at all", and the claim is this
            // process's own empty file (outpath::abandon says why that is
            // safe). A failed removal is reported, never swallowed.
            if let Err(cleanup) = outpath::abandon(&out_path) {
                eprintln!("arena: {cleanup}");
            }
            Err(error.to_string())
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
