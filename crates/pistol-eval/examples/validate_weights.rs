//! Parse and validate every eval weight table named on the command line.
//!
//! The weights half of `tools/config_check.sh`. A weight table is a different
//! document kind from an engine config (docs/decisions.md D-64), so it has its
//! own validator; it lives here as an example rather than as a pistol-cli
//! subcommand for the same reason `validate_config` does — pistol-cli's surface
//! mirrors the `Engine` trait one to one, and checking a committed document is a
//! tools-side gate (docs/decisions.md D-25).
//!
//! Exit codes: 0 all valid, 1 at least one rejected, 2 nothing to check.

use std::path::PathBuf;
use std::process::ExitCode;

use pistol_core::WIN_LEN;
use pistol_eval::{WEIGHTS_SCHEMA_VERSION, Weights};

fn main() -> ExitCode {
    let paths: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    if paths.is_empty() {
        eprintln!("validate_weights: expected at least one weights path, got none");
        return ExitCode::from(2);
    }

    let mut rejected = 0usize;
    for path in &paths {
        match Weights::load(path) {
            Ok(weights) => println!(
                "ok   {}  schema={WEIGHTS_SCHEMA_VERSION} table=[{}]",
                path.display(),
                // Every count the document states: one short of a win, which is
                // not a number this file may carry (docs/decisions.md D-63).
                (1..WIN_LEN)
                    .map(|count| {
                        let count = u8::try_from(count).expect("a window length fits a byte");
                        weights.window_value(count).to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            ),
            Err(error) => {
                eprintln!("FAIL {}  {error}", path.display());
                rejected += 1;
            }
        }
    }

    if rejected > 0 {
        eprintln!("validate_weights: {rejected} of {} rejected", paths.len());
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
