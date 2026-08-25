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
