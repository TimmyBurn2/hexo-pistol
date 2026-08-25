//! Rules a config must satisfy that `serde` cannot express: value ranges, and
//! agreements between fields in different sections.
//!
//! Every rejection here names the key an operator has to go and edit. Nothing
//! here repairs a value — a config is right or it is refused (CLAUDE.md rule 3).

use crate::config::{
    CandidatePolicy, Config, EngineMode, EvalSection, InstrumentSection, MAX_CANDIDATE_RADIUS,
    MAX_MOVETIME_EPSILON_MS, MAX_Q_DEPTH_TURNS, MAX_TT_BYTES, MIN_TT_BYTES, PlaySection,
    SCHEMA_VERSION, SearchSection,
};
use crate::error::EngineError;

impl Config {
    /// Check every value range and every cross-field agreement.
    ///
    /// [`Config::load`] runs this for you; call it directly only when you
    /// parsed with [`Config::parse_unvalidated`].
    pub fn validate(&self) -> Result<(), EngineError> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(EngineError::config(
                "schema_version",
                format!(
                    "this build reads schema version {SCHEMA_VERSION}, the document says {}",
                    self.schema_version
                ),
            ));
        }

        self.search.validate()?;
        self.eval.validate()?;
        self.instrument.validate()?;
        self.play.validate()?;

        // The determinism law: instrument mode is the source of every strength
        // claim, so it may not race (CLAUDE.md rule 4).
        if self.engine.mode == EngineMode::Instrument && self.instrument.threads != 1 {
            return Err(EngineError::config(
                "instrument.threads",
                format!(
                    "engine.mode is \"instrument\", which runs single-threaded; \
                     must be 1, got {}",
                    self.instrument.threads
                ),
            ));
        }

        Ok(())
    }
}

impl SearchSection {
    fn validate(&self) -> Result<(), EngineError> {
        if self.tt_bytes < MIN_TT_BYTES {
            return Err(EngineError::config(
                "search.tt_bytes",
                format!(
                    "must be at least {MIN_TT_BYTES} bytes, got {}",
                    self.tt_bytes
                ),
            ));
        }
        if self.tt_bytes > MAX_TT_BYTES {
            return Err(EngineError::config(
                "search.tt_bytes",
                format!(
                    "must be at most {MAX_TT_BYTES} bytes, got {}",
                    self.tt_bytes
                ),
            ));
        }
        // The table indexes by masking, so a size that is not a power of two
        // would have to be rounded, and rounding a stated value silently is
        // exactly what this project forbids (docs/decisions.md D-19).
        if !self.tt_bytes.is_power_of_two() {
            return Err(EngineError::config(
                "search.tt_bytes",
                format!("must be a power of two, got {}", self.tt_bytes),
            ));
        }

        match &self.candidate_policy {
            CandidatePolicy::Radius { radius } => {
                check_radius("search.candidate_policy.radius", *radius)?;
            }
            CandidatePolicy::Staged {
                quiet_radius,
                quiet_top_k,
                widen_schedule,
                tier_t_own_count,
                tier_t_opponent_count,
                q_depth_turns,
                q_triggers: _,
                killers: _,
                history: _,
                countermove: _,
            } => {
                check_radius("search.candidate_policy.quiet_radius", *quiet_radius)?;
                if *quiet_top_k == 0 {
                    return Err(EngineError::config(
                        "search.candidate_policy.quiet_top_k",
                        format!("must be at least 1, got {quiet_top_k}"),
                    ));
                }
                // Stage Q's own schema (`U3_tier_t.md` §10), validated for
                // completeness even though this D-scope's search does not
                // read the schedule (docs/decisions.md D-353): non-empty,
                // strictly increasing, and every entry strictly greater than
                // `quiet_top_k` — the cross-field rule revision 3's validator
                // lacked, which let `quiet_top_k = 64` with `[32]` pass as a
                // widening that NARROWS.
                if widen_schedule.is_empty() {
                    return Err(EngineError::config(
                        "search.candidate_policy.widen_schedule",
                        "must be non-empty",
                    ));
                }
                let mut previous = *quiet_top_k;
                for &boundary in widen_schedule {
                    if boundary <= previous {
                        return Err(EngineError::config(
                            "search.candidate_policy.widen_schedule",
                            format!(
                                "must be strictly increasing and every entry strictly greater \
                                 than quiet_top_k ({quiet_top_k}), got {boundary} after {previous}"
                            ),
                        ));
                    }
                    previous = boundary;
                }
                for (key, count) in [
                    ("search.candidate_policy.tier_t_own_count", tier_t_own_count),
                    (
                        "search.candidate_policy.tier_t_opponent_count",
                        tier_t_opponent_count,
                    ),
                ] {
                    if !(2..=3).contains(count) {
                        return Err(EngineError::config(
                            key,
                            format!(
                                "must be 2 or 3 — LAW-SUPPORT's threshold reading admits no \
                                 other count (U3_tier_t.md §6.1), got {count}"
                            ),
                        ));
                    }
                }
                // WP-1.6 (docs/wp16_quiescence_design.md §6): 0 is a real
                // value, not a missing one — it disables the extension while
                // the horizon's free checks still run. No lower bound beyond
                // that.
                if *q_depth_turns > MAX_Q_DEPTH_TURNS {
                    return Err(EngineError::config(
                        "search.candidate_policy.q_depth_turns",
                        format!("must be at most {MAX_Q_DEPTH_TURNS}, got {q_depth_turns}"),
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The bound every candidate-policy radius shares, named once so the two
/// callers (`Radius`'s own, `Staged`'s `quiet_radius`) cannot drift apart.
fn check_radius(key: &'static str, radius: u32) -> Result<(), EngineError> {
    if radius == 0 || radius > MAX_CANDIDATE_RADIUS {
        return Err(EngineError::config(
            key,
            format!("must be in 1..={MAX_CANDIDATE_RADIUS}, got {radius}"),
        ));
    }
    Ok(())
}

impl EvalSection {
    fn validate(&self) -> Result<(), EngineError> {
        // Shape only. Whether the file exists, and whether its contents suit
        // the backend, is pistol-eval's loud error at load time; config
        // validation stays pure and offline (docs/decisions.md D-21).
        if self.weights_file.as_os_str().is_empty() {
            return Err(EngineError::config(
                "eval.weights_file",
                "must name a weights file, got an empty path",
            ));
        }
        Ok(())
    }
}

impl InstrumentSection {
    fn validate(&self) -> Result<(), EngineError> {
        if self.threads == 0 {
            return Err(EngineError::config(
                "instrument.threads",
                "must be at least 1, got 0",
            ));
        }
        Ok(())
    }
}

impl PlaySection {
    fn validate(&self) -> Result<(), EngineError> {
        // Zero would promise an unmeasurable instantaneous reply; past the
        // ceiling is the typo class. Both are rejection bounds, not values
        // (docs/decisions.md D-18).
        if self.movetime_epsilon_ms == 0 || self.movetime_epsilon_ms > MAX_MOVETIME_EPSILON_MS {
            return Err(EngineError::config(
                "play.movetime_epsilon_ms",
                format!(
                    "must be in 1..={MAX_MOVETIME_EPSILON_MS}, got {}",
                    self.movetime_epsilon_ms
                ),
            ));
        }
        Ok(())
    }
}
