//! `wp18b_probe`: the WP-1.8b anchor-probe instrument.
//!
//! Reads a solver fixture, solves every case, prints one line per case with
//! the shipped solver's verdict plus the witness tree's win depth in TURNS —
//! the diagnostic the anchor probe records (`docs/experiments/
//! wp18b_anchor_probe.md`). Unlike `solver-selftest` it asserts NO
//! expectation: the probe's purpose is to learn the verdicts, and a fixture
//! whose `expect` is a placeholder must not fail the run.
//!
//! Deterministic end to end (D-7): the solver consults no clock, and this
//! binary adds no nondeterminism of its own.

use std::env;
use std::process::ExitCode;

use pistol_solver::fixture::load;
use pistol_solver::{SolveOutcome, Solver, SolverParams};

const USAGE: &str = "usage: wp18b-probe <fixture> <config>";
const MALFORMED: &str = "wp18b-probe: CANNOT READ:";

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let (Some(fixture_path), Some(config_path)) = (args.next(), args.next()) else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }
    let params = match read_config(&config_path) {
        Ok(params) => params,
        Err(what) => {
            eprintln!("{MALFORMED} {what}");
            return ExitCode::from(2);
        }
    };
    let text = match std::fs::read_to_string(&fixture_path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("{MALFORMED} {fixture_path}: {error}");
            return ExitCode::from(2);
        }
    };
    let cases = match load(&text) {
        Ok(cases) => cases,
        Err(error) => {
            eprintln!("{MALFORMED} {fixture_path}: {error}");
            return ExitCode::from(2);
        }
    };
    let mut solver = Solver::new(params.epsilon, params.tt_entries, params.attacker_policy);
    for case in &cases {
        let position = match case.position() {
            Ok(position) => position,
            Err(error) => {
                eprintln!("{MALFORMED} {fixture_path}: {error}");
                return ExitCode::from(2);
            }
        };
        let result = solver.solve(&position, pistol_solver::UNCAPPED);
        let (value, depth) = match &result.outcome {
            SolveOutcome::Win(tree) => ("win", tree.win_depth_turns()),
            SolveOutcome::NoWin => ("nowin", 0),
            SolveOutcome::NoWinUnderZone => ("nowin-under-zone", 0),
            SolveOutcome::Unknown => ("unknown", 0),
        };
        println!(
            "case {} value {} nodes {} seesaw {} depth_turns {depth}",
            case.name, value, result.nodes, result.seesaw
        );
    }
    println!("summary {} cases", cases.len());
    ExitCode::SUCCESS
}

fn read_config(path: &str) -> Result<SolverParams, String> {
    let text = std::fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    let file =
        pistol_solver::SolverConfigFile::parse(&text).map_err(|what| format!("{path}: {what}"))?;
    file.validate()
        .map_err(|error| format!("{path}: {error:?}"))
}
