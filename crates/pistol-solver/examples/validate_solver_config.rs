//! Parse and validate every solver config file named on the command line.
//!
//! The solver half of `tools/config_check.sh` (WP-1.8a): reads each file's
//! bytes, parses them with the crate's own strict reader, and runs
//! `SolverConfigFile::validate` — the same validation `solver-selftest`
//! applies, so the gate and the instrument cannot drift apart.
//!
//! Exit codes: 0 all valid, 1 at least one rejected, 2 nothing named.

use std::process::ExitCode;

fn main() -> ExitCode {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("validate_solver_config: no files named");
        return ExitCode::from(2);
    }
    let mut status = 0;
    for path in &paths {
        match validate(path) {
            Ok(params) => println!("validate_solver_config: {path} ok ({params:?})"),
            Err(what) => {
                eprintln!("validate_solver_config: {path} REJECTED: {what}");
                status = 1;
            }
        }
    }
    if status == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn validate(path: &str) -> Result<pistol_solver::SolverParams, String> {
    let text = std::fs::read_to_string(path).map_err(|error| format!("{error}"))?;
    let file = pistol_solver::SolverConfigFile::parse(&text)?;
    file.validate().map_err(|error| format!("{error:?}"))
}
