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

use pistol_arena::channel::Channel;
use pistol_arena::config::ArenaConfig;
use pistol_arena::error::ArenaError;
use pistol_arena::handshake;
use pistol_arena::report::{EngineIdentity, Written};
use pistol_arena::{openings, report, schedule, score, summary};
use pistol_cli::sha256::sha256_hex;

/// What this program does, and what it refuses to guess.
const USAGE: &str = "\
arena — the paired-openings SPRT judge for pistol

usage:
  arena --config <path> --out <path>

  --config  an arena config. Always explicit: there is no default path and no
            built-in configuration (CLAUDE.md rule 1). It states the openings,
            the budget, the turn cap, the worker count and the SPRT bounds.
  --out     where to write the report. Refused if it already exists: a run that
            silently overwrote a previous report would destroy the evidence for
            a claim somebody has already made. Match logs are artifacts and are
            never written inside the repository (CLAUDE.md rule 8).

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
    if out_path.exists() {
        return Err(format!(
            "{} already exists; a run does not overwrite a previous report",
            out_path.display()
        ));
    }
    run(&config_path, &out_path).map_err(|error| error.to_string())
}

fn run(config_path: &Path, out_path: &Path) -> Result<ExitCode, ArenaError> {
    let config = ArenaConfig::load(config_path)?;
    let config_sha = digest_of(config_path)?;
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
    // is a different program after every build (docs/decisions.md D-147).
    let identities = [
        identity_of(&config.engine_a, config.run.hang_timeout_ms)?,
        identity_of(&config.engine_b, config.run.hang_timeout_ms)?,
    ];

    let started = Instant::now();
    let (outcome, played) = schedule::run(&config, &openings, &go_line);
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
    std::fs::write(out_path, &rendered)
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

/// One engine's identity: what it says about itself, and what it is by content.
///
/// The engine is started once here, before any game, purely to shake hands.
/// That costs one process per side and buys the run's instrument (CLAUDE.md
/// rule 6): the `id` lines carry the candidate radius and the table size, and a
/// log that cannot recover those cannot be re-run. It also fails the run early
/// on an engine a strength claim may not come from, rather than on the first
/// game.
///
/// The digests are what the `id` lines cannot say: they name a config by PATH,
/// and a path is the same string after an edit (docs/decisions.md D-147).
fn identity_of(
    engine: &pistol_arena::config::EngineSection,
    timeout_ms: u64,
) -> Result<EngineIdentity, ArenaError> {
    let mut channel = Channel::start(&engine.label, &engine.binary, &engine.config)?;
    let spoken = handshake::shake(&mut channel, timeout_ms)?;
    channel.shutdown();
    Ok(EngineIdentity {
        id_lines: spoken.lines,
        binary_sha256: digest_of(&engine.binary)?,
        config_sha256: digest_of(&engine.config)?,
    })
}

/// The SHA-256 of a file this run depends on.
fn digest_of(path: &Path) -> Result<String, ArenaError> {
    let bytes = std::fs::read(path)
        .map_err(|io| ArenaError::io(format!("reading {}", path.display()), io))?;
    Ok(sha256_hex(&bytes))
}
