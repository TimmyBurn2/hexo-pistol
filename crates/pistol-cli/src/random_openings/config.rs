use std::path::Path;

use serde::Deserialize;

use super::error::RandomOpeningsError;

/// The schema version this build understands.
///
/// 2 since `[generate] book`. The key is required and there is no code-side
/// default for it (CLAUDE.md rule 1), so a version-1 document cannot be read
/// by this build — which is the loud refusal the bump exists to give, rather
/// than a silent guess that it meant v1.
pub const RANDOM_OPENINGS_SCHEMA_VERSION: u32 = 2;

/// The stone counts this generator is specified at.
///
/// Both are TURN BOUNDARIES: three stones is the position after turn 2 (P1's
/// opening stone and P2's whole reply) and five is the position after turn 3.
/// The set stops at five because the argument for shipping no balance filter is
/// arithmetic about `k <= 5` (docs/decisions.md D-175); a longer book is
/// WP-1.2a's corpus fixture, which is seven.
pub const SUPPORTED_STONE_COUNTS: &[usize] = &[3, 5];

/// The widest generation radius this tool accepts.
///
/// A TYPO CEILING and nothing else. It is deliberately unrelated to game rule
/// 5's `LEGAL_RADIUS`, which is a pinned constant in pistol-core and is not a
/// knob: this number only bounds how far from the origin a *sampled* stone may
/// land, and every placement is still checked against rule 5 as it is made.
pub const MAX_RADIUS_CEILING: u32 = 64;

/// The largest book this tool accepts. A typo ceiling, as above.
pub const N_OPENINGS_CEILING: usize = 100_000;

/// A complete random-openings configuration.
///
/// Parsing an incomplete document is an error, never an empty-but-usable one:
///
/// ```
/// use pistol_cli::random_openings::config::RandomOpeningsConfig;
/// assert!(RandomOpeningsConfig::parse("").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RandomOpeningsConfig {
    /// Schema version of this document; must equal
    /// [`RANDOM_OPENINGS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// What to generate.
    pub generate: GenerateSection,
}

/// `[generate]`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerateSection {
    /// Which book this document generates, and so which file name and which
    /// header the run writes.
    pub book: super::BookVersion,
    /// Stones in each opening. A TURN-BOUNDARY count; see
    /// [`SUPPORTED_STONE_COUNTS`].
    pub k_stones: usize,
    /// How many openings the book holds. There is no cap on top of this: the
    /// number asked for is the number written, or the run refuses.
    pub n_openings: usize,
    /// How far from the ORIGIN a sampled stone may land, in hex distance.
    ///
    /// A GENERATION knob, config and never a constant, and never conflated
    /// with game rule 5's `LEGAL_RADIUS` — that one is a rule about where a
    /// stone may be played given the stones already down, this one is a choice
    /// about how spread out a synthetic opening should be.
    pub max_radius: u32,
    /// The PRNG seed. Recorded in the header of every file it produced, because
    /// it is the only reason those positions and not others.
    pub seed: u64,
}

impl RandomOpeningsConfig {
    /// Read and validate a document.
    pub fn load(path: &Path) -> Result<RandomOpeningsConfig, RandomOpeningsError> {
        let text = std::fs::read_to_string(path).map_err(|io| RandomOpeningsError::Read {
            path: path.to_path_buf(),
            why: io.to_string(),
        })?;
        RandomOpeningsConfig::parse(&text)
    }

    /// Parse and validate a document held as text.
    pub fn parse(text: &str) -> Result<RandomOpeningsConfig, RandomOpeningsError> {
        // Two stages, for two kinds of error, the way pistol-engine's config
        // does it: the first parse reports syntax with a line and column, the
        // second reports schema violations with the key path.
        let document: toml::Value =
            toml::from_str(text).map_err(|error| RandomOpeningsError::Schema {
                why: format!("not TOML: {error}"),
            })?;
        let config: RandomOpeningsConfig =
            serde_path_to_error::deserialize(document).map_err(|error| {
                RandomOpeningsError::Schema {
                    why: format!("at `{}`: {}", error.path(), error.inner()),
                }
            })?;
        config.validate()?;
        Ok(config)
    }

    /// The cross-field rules, in the order an operator meets them.
    ///
    /// Public and re-run by [`super::generate`] rather than trusted to have
    /// happened. Every field of this struct is `pub`, so a caller inside the
    /// workspace can build one by struct literal and never come through
    /// [`RandomOpeningsConfig::parse`] — and the invariants this checks are what
    /// keep the generator's work bounded and its output inside the arithmetic
    /// D-175 rests on. A validator that only runs on one of two doors is not a
    /// validator (CLAUDE.md rule 3).
    pub fn validate(&self) -> Result<(), RandomOpeningsError> {
        if self.schema_version != RANDOM_OPENINGS_SCHEMA_VERSION {
            return Err(RandomOpeningsError::SchemaVersion {
                found: self.schema_version,
                expected: RANDOM_OPENINGS_SCHEMA_VERSION,
            });
        }
        let GenerateSection {
            book: _,
            k_stones,
            n_openings,
            max_radius,
            seed: _,
        } = self.generate;

        // Two complaints about `k_stones`, and they are not the same one. An
        // even count is refused because it names no position a game is at; an
        // odd one outside the set names a real position this tool is simply not
        // specified at. Collapsing them would make the diagnostic a lie about
        // which is wrong (the precedent is pistol-core's own split of rule 3
        // from rule 5 in `Board::check_placement`).
        if k_stones % 2 == 0 {
            return Err(RandomOpeningsError::MidTurnStoneCount { k_stones });
        }
        if !SUPPORTED_STONE_COUNTS.contains(&k_stones) {
            return Err(RandomOpeningsError::UnsupportedStoneCount {
                k_stones,
                supported: SUPPORTED_STONE_COUNTS,
            });
        }

        if n_openings == 0 || n_openings > N_OPENINGS_CEILING {
            return Err(RandomOpeningsError::CountPastCeiling {
                n_openings,
                ceiling: N_OPENINGS_CEILING,
            });
        }
        // The ceiling is checked before the ball is enumerated, so a mistyped
        // radius refuses instead of allocating first.
        if max_radius > MAX_RADIUS_CEILING {
            return Err(RandomOpeningsError::RadiusPastCeiling {
                max_radius,
                ceiling: MAX_RADIUS_CEILING,
            });
        }
        let cells = super::ball(max_radius).len();
        if cells < k_stones {
            return Err(RandomOpeningsError::BallTooSmall {
                max_radius,
                cells,
                k_stones,
            });
        }
        Ok(())
    }
}
