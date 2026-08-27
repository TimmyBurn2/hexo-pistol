//! `solver-selftest`: the solver's instrument (design §7).
//!
//! Prints one line per fixture position — name, value, nodes, seesaw, proof
//! digest, zone status — plus a summary line, and exits 0 only when every
//! case matches its registered expectation. A malformed fixture refuses by
//! name at exit 2 (the void); a value mismatch is the finding, exit 1.
//!
//! Deterministic end to end (D-7): the solver consults no clock and no
//! hasher iteration order, so two runs over the same fixture are
//! byte-identical — which is what `tools/solver_determinism.sh` diffs.

use std::env;
use std::process::ExitCode;

use pistol_solver::fixture::{Expectation, load};
use pistol_solver::{SolveOutcome, Solver, SolverParams};

const USAGE: &str = "usage: solver-selftest <fixture> [config]";
const MALFORMED: &str = "solver-selftest: CANNOT READ:";
const MISMATCH: &str = "solver-selftest: FAIL:";

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    // The config is optional ONLY in the sense that the registered default
    // (configs/solver_v0.toml's values) is compiled in when no path is
    // given; the values themselves are never defaulted piecemeal (rule 1).
    // The registered default IS the committed config file — never
    // compiled-in literals (rule 1: the tunables live in exactly one
    // schema place, configs/solver_v0.toml).
    let params = match args.next().as_deref() {
        None => read_config("configs/solver_v0.toml"),
        Some(config_path) => read_config(config_path),
    };
    let params = match params {
        Ok(params) => params,
        Err(what) => {
            eprintln!("{MALFORMED} {what}");
            return ExitCode::from(2);
        }
    };
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("{MALFORMED} {path}: {error}");
            return ExitCode::from(2);
        }
    };
    let cases = match load(&text) {
        Ok(cases) => cases,
        Err(error) => {
            eprintln!("{MALFORMED} {path}: {error}");
            return ExitCode::from(2);
        }
    };
    let mut solver = Solver::new(params.epsilon, params.tt_entries, params.attacker_policy);
    let mut failures = 0u32;
    let mut wins = 0u32;
    for case in &cases {
        let position = match case.position() {
            Ok(position) => position,
            Err(error) => {
                eprintln!("{MALFORMED} {path}: {error}");
                return ExitCode::from(2);
            }
        };
        let result = solver.solve(&position);
        let (value, digest, zone) = match &result.outcome {
            SolveOutcome::Win(tree) => ("win", tree.digest(), "ok"),
            SolveOutcome::NoWin => ("nowin", 0, "-"),
            SolveOutcome::NoWinUnderZone => ("nowin-under-zone", 0, "OVERFLOW"),
        };
        println!(
            "case {} value {} nodes {} seesaw {} digest {digest:016x} zone {zone}",
            case.name, value, result.nodes, result.seesaw
        );
        let expected = match case.expect {
            Expectation::Win => "win",
            Expectation::NoWin => "nowin",
        };
        if value != expected {
            eprintln!("{MISMATCH} {}: expected {expected}, got {value}", case.name);
            failures += 1;
        } else if value == "win" {
            wins += 1;
        }
    }
    println!(
        "summary {} cases {} wins {} failures",
        cases.len(),
        wins,
        failures
    );
    if failures == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn read_config(path: &str) -> Result<SolverParams, String> {
    let text = std::fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    let file =
        pistol_solver::SolverConfigFile::parse(&text).map_err(|what| format!("{path}: {what}"))?;
    file.validate()
        .map_err(|error| format!("{path}: {error:?}"))
}
