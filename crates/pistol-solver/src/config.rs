//! The solver's configuration schema (design §8).
//!
//! Hard rule 1: explicit and complete, `deny_unknown_fields`, every key
//! required, no code-side default for any tunable — a default lives in
//! exactly one schema place, and for the solver that place is
//! `configs/solver_v0.toml`. Validation is fail-loud by named error: a value
//! v0 does not implement is refused, never silently reinterpreted.

use serde::Deserialize;

use crate::pn::Epsilon;

/// The schema version this code reads.
pub const SOLVER_SCHEMA_VERSION: u32 = 1;

/// The zone-order count v0 implements (Wu & Lin's order-3 cap).
pub const SUPPORTED_ZONE_ORDERS: u32 = 3;

/// The free-stone radius v0 implements: the full legal region, no pruning.
pub const SUPPORTED_FREE_STONE_RADIUS: u32 = 8;

/// A named refusal: what was wrong, and the value that was wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolverConfigError {
    /// The schema version is not the one this code reads.
    SchemaVersion { found: u32 },
    /// ε is not a strictly positive rational the sentinel arithmetic is
    /// safe over (`0 < num/den ≤ 2`).
    Epsilon { num: u32, den: u32 },
    /// `zone_orders` is not the order count v0 implements.
    ZoneOrders { found: u32 },
    /// `free_stone_radius` is not the radius v0 implements.
    FreeStoneRadius { found: u32 },
    /// `tt_entries` is not a power of two ≥ 2.
    TtEntries { found: u32 },
}

/// The file's whole shape.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SolverConfigFile {
    /// Must equal [`SOLVER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// The solver's knobs.
    pub solver: SolverSection,
}

/// The `[solver]` table.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SolverSection {
    /// ε's numerator.
    pub epsilon_num: u32,
    /// ε's denominator.
    pub epsilon_den: u32,
    /// The zone sequence's order count.
    pub zone_orders: u32,
    /// The defender free-stone range, as a radius.
    pub free_stone_radius: u32,
    /// The transposition table's entry count.
    pub tt_entries: u32,
}

/// The validated parameters a solver is built from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolverParams {
    /// The threshold loosening, applied to thresholds only.
    pub epsilon: Epsilon,
    /// The table's entry count.
    pub tt_entries: usize,
}

impl SolverSection {
    /// Validate into [`SolverParams`], or refuse by name.
    pub fn validate(&self) -> Result<SolverParams, SolverConfigError> {
        let epsilon =
            Epsilon::new(self.epsilon_num, self.epsilon_den).ok_or(SolverConfigError::Epsilon {
                num: self.epsilon_num,
                den: self.epsilon_den,
            })?;
        if self.zone_orders != SUPPORTED_ZONE_ORDERS {
            return Err(SolverConfigError::ZoneOrders {
                found: self.zone_orders,
            });
        }
        if self.free_stone_radius != SUPPORTED_FREE_STONE_RADIUS {
            return Err(SolverConfigError::FreeStoneRadius {
                found: self.free_stone_radius,
            });
        }
        if self.tt_entries < 2 || !self.tt_entries.is_power_of_two() {
            return Err(SolverConfigError::TtEntries {
                found: self.tt_entries,
            });
        }
        Ok(SolverParams {
            epsilon,
            tt_entries: self.tt_entries as usize,
        })
    }
}

impl SolverConfigFile {
    /// Validate the version and the section together.
    pub fn validate(&self) -> Result<SolverParams, SolverConfigError> {
        if self.schema_version != SOLVER_SCHEMA_VERSION {
            return Err(SolverConfigError::SchemaVersion {
                found: self.schema_version,
            });
        }
        self.solver.validate()
    }
}

impl SolverConfigFile {
    /// Parse and validate the config's own text — THE one parser for this
    /// schema. The bin, the validator example and the oracle gates all call
    /// this same function (CLAUDE.md rule 1: the tunables live in exactly
    /// one schema place, and nothing re-reads them from literals). The
    /// grammar is deliberately tiny — five integer keys in one `[solver]`
    /// table plus a schema_version — and anything else is refused by name.
    pub fn parse(text: &str) -> Result<SolverConfigFile, String> {
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
                schema_version = Some(
                    u32::try_from(value).map_err(|_| "schema_version does not fit".to_owned())?,
                );
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
        Ok(SolverConfigFile {
            schema_version: schema_version.ok_or("missing schema_version")?,
            solver: SolverSection {
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
        })
    }
}
