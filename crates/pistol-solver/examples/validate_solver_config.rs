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
    let mut schema_version: Option<u32> = None;
    let mut keys: std::collections::BTreeMap<String, i64> = std::collections::BTreeMap::new();
    let mut section = false;
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            let found = line[1..line.len() - 1].trim();
            if found != "solver" {
                return Err(format!("line {}: unknown section [{found}]", index + 1));
            }
            section = true;
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("line {}: not `key = value`", index + 1));
        };
        let key = key.trim();
        let value: i64 = value
            .trim()
            .parse()
            .map_err(|_| format!("line {}: not an integer", index + 1))?;
        if !section {
            if key != "schema_version" {
                return Err(format!("line {}: unknown key {key}", index + 1));
            }
            schema_version =
                Some(u32::try_from(value).map_err(|_| "schema_version does not fit".to_owned())?);
        } else if keys.insert(key.to_owned(), value).is_some() {
            return Err(format!("line {}: key {key} given twice", index + 1));
        }
    }
    if keys.len() != 5 {
        return Err(format!(
            "the [solver] table holds {} keys, expected 5",
            keys.len()
        ));
    }
    let integer = |name: &str| -> Result<i64, String> {
        keys.get(name)
            .copied()
            .ok_or_else(|| format!("missing key {name}"))
    };
    let file = pistol_solver::SolverConfigFile {
        schema_version: schema_version.ok_or("missing schema_version")?,
        solver: pistol_solver::SolverSection {
            epsilon_num: u32::try_from(integer("epsilon_num")?)
                .map_err(|_| "epsilon_num does not fit".to_owned())?,
            epsilon_den: u32::try_from(integer("epsilon_den")?)
                .map_err(|_| "epsilon_den does not fit".to_owned())?,
            zone_orders: u32::try_from(integer("zone_orders")?)
                .map_err(|_| "zone_orders does not fit".to_owned())?,
            free_stone_radius: u32::try_from(integer("free_stone_radius")?)
                .map_err(|_| "free_stone_radius does not fit".to_owned())?,
            tt_entries: u32::try_from(integer("tt_entries")?)
                .map_err(|_| "tt_entries does not fit".to_owned())?,
        },
    };
    file.validate().map_err(|error| format!("{error:?}"))
}
