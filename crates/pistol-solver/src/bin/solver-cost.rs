//! `solver-cost`: the per-visit wall instrument (WP-1.8c design §2, §6).
//!
//! Microseconds per visit rather than nps, because node counts are held
//! identical across the change being measured and an nps ratio over a changing
//! node mix is not a clean per-leg reading. It asserts NO expectation —
//! learning the cost is the job, and `solver-selftest` is the gate that
//! adjudicates values. Wall time is the one thing here that is not
//! reproducible and the one thing being measured; every value beside it is
//! deterministic (docs/decisions.md D-7).

use std::env;
use std::process::ExitCode;
use std::time::Instant;

use pistol_solver::fixture::load;
use pistol_solver::{SolveOutcome, Solver, SolverParams};

const USAGE: &str = "usage: solver-cost <fixture> <config> <node-cap>";
const MALFORMED: &str = "solver-cost: CANNOT READ:";

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let (Some(fixture_path), Some(config_path), Some(cap)) =
        (args.next(), args.next(), args.next())
    else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }
    let Ok(cap) = cap.parse::<u64>() else {
        eprintln!("{MALFORMED} {cap:?} is not a node cap");
        return ExitCode::from(2);
    };
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
    let (mut total_nodes, mut total_micros) = (0u64, 0u128);
    for case in &cases {
        let position = match case.position() {
            Ok(position) => position,
            Err(error) => {
                eprintln!("{MALFORMED} {fixture_path}: {error}");
                return ExitCode::from(2);
            }
        };
        let started = Instant::now();
        let result = solver.solve(&position, cap);
        let micros = started.elapsed().as_micros();
        let value = match &result.outcome {
            SolveOutcome::Win(_) => "win",
            SolveOutcome::NoWin => "nowin",
            SolveOutcome::NoWinUnderZone => "nowin-under-zone",
            SolveOutcome::Unknown => "unknown",
        };
        println!(
            "case {} value {value} nodes {} seesaw {} us {micros} us_per_visit {:.2}",
            case.name,
            result.nodes,
            result.seesaw,
            per_visit(micros, result.nodes),
        );
        total_nodes += result.nodes;
        total_micros += micros;
    }
    println!(
        "TOTAL cases {} nodes {total_nodes} us {total_micros} us_per_visit {:.2}",
        cases.len(),
        per_visit(total_micros, total_nodes),
    );
    ExitCode::SUCCESS
}

/// Zero visits cost zero per visit, rather than dividing by zero: a solve that
/// answers without entering `dfpn` is a real case, not a malformed one.
fn per_visit(micros: u128, nodes: u64) -> f64 {
    if nodes == 0 {
        return 0.0;
    }
    micros as f64 / nodes as f64
}

fn read_config(path: &str) -> Result<SolverParams, String> {
    let text = std::fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    let file =
        pistol_solver::SolverConfigFile::parse(&text).map_err(|what| format!("{path}: {what}"))?;
    file.validate()
        .map_err(|error| format!("{path}: {error:?}"))
}
