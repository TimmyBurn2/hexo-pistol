use serde::Deserialize;

use crate::pn::Epsilon;

/// The schema version this code reads.
pub const SOLVER_SCHEMA_VERSION: u32 = 1;

/// The zone-order count v0 implements (Wu & Lin's order-3 cap).
pub const SUPPORTED_ZONE_ORDERS: u32 = 3;

/// The free-stone radius v0 implements: the full legal region, no pruning.
pub const SUPPORTED_FREE_STONE_RADIUS: u32 = 8;

/// The attacker policy the solver plays (design wp18b_m4 §2): which pairs
/// the OR node may move with. `BothStonesRelevant` is v0; `OneFreeStone`
/// is the M4 widening — arm B (`raiser × legal-region cell not in C`)
/// appended after v0's arm A, dedup-free by construction because arm B's
/// free stone is never a `C`-cell while arm A's pairs are always both-in-`C`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttackerPolicy {
    /// v0: both stones threat-relevant (both cells in `C`).
    BothStonesRelevant,
    /// M4: one raiser stone plus one free stone anywhere legal.
    OneFreeStone,
}

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
    /// `attacker_policy` is not one of the two policies this code reads.
    AttackerPolicy { found: String },
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
    /// The attacker policy (see [`AttackerPolicy`]).
    pub attacker_policy: AttackerPolicy,
}

/// The validated parameters a solver is built from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolverParams {
    /// The threshold loosening, applied to thresholds only.
    pub epsilon: Epsilon,
    /// The table's entry count.
    pub tt_entries: usize,
    /// The attacker policy.
    pub attacker_policy: AttackerPolicy,
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
            attacker_policy: self.attacker_policy,
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
        let mut policy: Option<AttackerPolicy> = None;
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
            let value = value.trim();
            if !section {
                if key != "schema_version" {
                    return Err(format!("line {}: unknown key {key}", index + 1));
                }
                schema_version = Some(
                    u32::try_from(
                        value
                            .parse::<i64>()
                            .map_err(|_| format!("line {}: not an integer", index + 1))?,
                    )
                    .map_err(|_| "schema_version does not fit".to_owned())?,
                );
            } else if key == "attacker_policy" {
                // The one string-valued key: quoted per the TOML files the
                // committed configs write, tolerated unquoted so hand edits
                // fail on the VALUE (below) rather than on spelling. Given
                // twice is refused exactly like every integer key — a
                // last-wins branch would be the one lax spot in a strict
                // parser (REVIEW-impl MINOR, closed at the fix round).
                if policy.is_some() {
                    return Err(format!(
                        "line {}: key attacker_policy given twice",
                        index + 1
                    ));
                }
                let spelling = value.trim_matches('"');
                policy = Some(match spelling {
                    "one_free_stone" => AttackerPolicy::OneFreeStone,
                    "both_stones_relevant" => AttackerPolicy::BothStonesRelevant,
                    other => {
                        return Err(format!(
                            "line {}: attacker_policy {other:?} is not one of \
                             `one_free_stone`, `both_stones_relevant`",
                            index + 1
                        ));
                    }
                });
            } else {
                let value: i64 = value
                    .parse()
                    .map_err(|_| format!("line {}: not an integer", index + 1))?;
                if keys.insert(key.to_owned(), value).is_some() {
                    return Err(format!("line {}: key {key} given twice", index + 1));
                }
            }
        }
        if keys.len() != 5 {
            return Err(format!(
                "the [solver] table holds {} integer keys, expected 5",
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
                attacker_policy: policy.ok_or("missing key attacker_policy")?,
            },
        })
    }
}

#[cfg(test)]
mod wp18b_config_tests {
    use super::*;

    #[test]
    fn a_doubled_attacker_policy_is_refused_like_every_integer_key() {
        let text = "schema_version = 1\n[solver]\nepsilon_num = 1\nepsilon_den = 4\n\
                    zone_orders = 3\nfree_stone_radius = 8\ntt_entries = 1024\n\
                    attacker_policy = \"one_free_stone\"\nattacker_policy = \"both_stones_relevant\"\n";
        assert_eq!(
            SolverConfigFile::parse(text).unwrap_err(),
            "line 9: key attacker_policy given twice"
        );
    }

    #[test]
    fn the_widened_and_narrow_spellings_both_parse() {
        for (spelling, expected) in [
            ("one_free_stone", AttackerPolicy::OneFreeStone),
            ("both_stones_relevant", AttackerPolicy::BothStonesRelevant),
        ] {
            let text = format!(
                "schema_version = 1\n[solver]\nepsilon_num = 1\nepsilon_den = 4\n\
                 zone_orders = 3\nfree_stone_radius = 8\ntt_entries = 1024\n\
                 attacker_policy = \"{spelling}\"\n"
            );
            let params = SolverConfigFile::parse(&text).unwrap().validate().unwrap();
            assert_eq!(params.attacker_policy, expected);
        }
        let refused = SolverConfigFile::parse(
            "schema_version = 1\n[solver]\nepsilon_num = 1\nepsilon_den = 4\n\
             zone_orders = 3\nfree_stone_radius = 8\ntt_entries = 1024\n\
             attacker_policy = \"narrow\"\n",
        )
        .unwrap_err();
        assert!(refused.contains("is not one of"), "{refused}");
    }
}
