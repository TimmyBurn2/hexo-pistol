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

use std::collections::BTreeMap;
use std::env;
use std::process::ExitCode;

use pistol_solver::fixture::{Expectation, load};
use pistol_solver::{Epsilon, SolveOutcome, Solver, SolverParams};

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
    let params = match args.next().as_deref() {
        None => SolverParams {
            epsilon: Epsilon::new(1, 4).expect("the registered start value 1/4 is valid"),
            tt_entries: 1 << 20,
        },
        Some(config_path) => match read_config(config_path) {
            Ok(params) => params,
            Err(what) => {
                eprintln!("{MALFORMED} {what}");
                return ExitCode::from(2);
            }
        },
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
    let mut solver = Solver::new(params.epsilon, params.tt_entries);
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
    // Parsed by hand into the config's own types: the file's shape is five
    // integer keys in one `[solver]` table plus a schema_version, and
    // refusing anything else — here, without a TOML dependency — is
    // `deny_unknown_fields` made executable.
    let mut schema_version: Option<u32> = None;
    let mut keys: BTreeMap<String, i64> = BTreeMap::new();
    let mut section = String::new();
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            let found = line[1..line.len() - 1].trim();
            if found != "solver" {
                return Err(format!(
                    "{}: line {}: unknown section [{found}]",
                    path,
                    index + 1
                ));
            }
            section = found.to_owned();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("{}: line {}: not `key = value`", path, index + 1));
        };
        let key = key.trim();
        let value: i64 = value
            .trim()
            .parse()
            .map_err(|_| format!("{}: line {}: not an integer", path, index + 1))?;
        if section.is_empty() {
            if key != "schema_version" {
                return Err(format!("{}: line {}: unknown key {key}", path, index + 1));
            }
            schema_version = Some(
                u32::try_from(value)
                    .map_err(|_| format!("{}: schema_version does not fit", path))?,
            );
        } else {
            if keys.insert(key.to_owned(), value).is_some() {
                return Err(format!(
                    "{}: line {}: key {key} given twice",
                    path,
                    index + 1
                ));
            }
        }
    }
    let integer = |name: &str| -> Result<i64, String> {
        keys.get(name)
            .copied()
            .ok_or_else(|| format!("{path}: missing key {name}"))
    };
    if keys.len() != 5 {
        return Err(format!(
            "{path}: the [solver] table holds {} keys, expected 5",
            keys.len()
        ));
    }
    let file = pistol_solver::SolverConfigFile {
        schema_version: schema_version.ok_or_else(|| format!("{path}: missing schema_version"))?,
        solver: pistol_solver::SolverSection {
            epsilon_num: u32::try_from(integer("epsilon_num")?)
                .map_err(|_| format!("{path}: epsilon_num does not fit"))?,
            epsilon_den: u32::try_from(integer("epsilon_den")?)
                .map_err(|_| format!("{path}: epsilon_den does not fit"))?,
            zone_orders: u32::try_from(integer("zone_orders")?)
                .map_err(|_| format!("{path}: zone_orders does not fit"))?,
            free_stone_radius: u32::try_from(integer("free_stone_radius")?)
                .map_err(|_| format!("{path}: free_stone_radius does not fit"))?,
            tt_entries: u32::try_from(integer("tt_entries")?)
                .map_err(|_| format!("{path}: tt_entries does not fit"))?,
        },
    };
    file.validate()
        .map_err(|error| format!("{path}: {error:?}"))
}
