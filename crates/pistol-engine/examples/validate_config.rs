//! Parse and validate every config file named on the command line.
//!
//! This is the executable half of `tools/config_check.sh`. It lives here rather
//! than in pistol-cli because pistol-cli's surface mirrors the `Engine` trait
//! one to one, and config checking is a tools-side gate
//! (docs/decisions.md D-25).
//!
//! Exit codes: 0 all valid, 1 at least one rejected, 2 nothing to check.

use std::path::PathBuf;
use std::process::ExitCode;

use pistol_engine::Config;

fn main() -> ExitCode {
    let paths: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    if paths.is_empty() {
        eprintln!("validate_config: expected at least one config path, got none");
        return ExitCode::from(2);
    }

    let mut rejected = 0usize;
    for path in &paths {
        match Config::load(path) {
            Ok(config) => println!(
                "ok   {}  schema={} mode={:?} tt_bytes={} candidates={:?} eval={:?} threads={} tie_break={:?}",
                path.display(),
                config.schema_version,
                config.engine.mode,
                config.search.tt_bytes,
                config.search.candidate_policy,
                config.eval.backend,
                config.instrument.threads,
                config.instrument.tie_break,
            ),
            Err(error) => {
                eprintln!("FAIL {}  {error}", path.display());
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
