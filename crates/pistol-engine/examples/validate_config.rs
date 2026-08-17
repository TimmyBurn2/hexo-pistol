//! Parse and validate every config file named on the command line.
//!
//! This is the executable half of `tools/config_check.sh`. It lives here rather
//! than in pistol-cli because pistol-cli's surface mirrors the `Engine` trait
//! one to one, and config checking is a tools-side gate
//! (docs/decisions.md D-25).
//!
//! With `--check-weights-file` it also checks that the file each config's
//! `eval.weights_file` names is there and readable. That check belongs to a gate
//! and not to `Config::validate`, which stays pure, offline and independent of
//! the working directory: a missing weights file is pistol-eval's loud error at
//! load time, and this is the deployment gate docs/decisions.md D-21 defers to
//! and D-66 places here. It says nothing about the file's *contents* — that is
//! the pistol-eval example `validate_weights`, which reads a different schema.
//!
//! Exit codes: 0 all valid, 1 at least one rejected, 2 nothing to check or an
//! unknown flag.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use pistol_engine::Config;

fn main() -> ExitCode {
    let mut check_weights_file = false;
    let mut paths: Vec<PathBuf> = Vec::new();
    for argument in std::env::args_os().skip(1) {
        match argument.to_str() {
            Some("--check-weights-file") => check_weights_file = true,
            Some(flag) if flag.starts_with("--") => {
                eprintln!("validate_config: unknown flag `{flag}`");
                return ExitCode::from(2);
            }
            _ => paths.push(PathBuf::from(argument)),
        }
    }
    if paths.is_empty() {
        eprintln!("validate_config: expected at least one config path, got none");
        return ExitCode::from(2);
    }

    let mut rejected = 0usize;
    for path in &paths {
        match check(path, check_weights_file) {
            Ok(summary) => println!("ok   {}  {summary}", path.display()),
            Err(why) => {
                eprintln!("FAIL {}  {why}", path.display());
                rejected += 1;
            }
        }
    }

    if rejected > 0 {
        eprintln!("validate_config: {rejected} of {} rejected", paths.len());
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

/// Validate one config, and — if asked — that the weights file it names exists.
fn check(path: &Path, check_weights_file: bool) -> Result<String, String> {
    let config = Config::load(path).map_err(|error| error.to_string())?;
    if check_weights_file {
        let weights = &config.eval.weights_file;
        match std::fs::metadata(weights) {
            Ok(found) if found.is_file() => {}
            Ok(_) => {
                return Err(format!(
                    "config: `eval.weights_file`: {} is not a file",
                    weights.display()
                ));
            }
            Err(io) => {
                return Err(format!(
                    "config: `eval.weights_file`: cannot read {}: {io} (relative paths \
                     resolve against the working directory)",
                    weights.display()
                ));
            }
        }
    }
    Ok(format!(
        "schema={} mode={:?} tt_bytes={} candidates={:?} eval={:?} weights={} threads={} \
         tie_break={:?}",
        config.schema_version,
        config.engine.mode,
        config.search.tt_bytes,
        config.search.candidate_policy,
        config.eval.backend,
        config.eval.weights_file.display(),
        config.instrument.threads,
        config.instrument.tie_break,
    ))
}
