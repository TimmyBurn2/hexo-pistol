//! Rules a config must satisfy that `serde` cannot express: value ranges, and
//! agreements between fields in different sections.
//!
//! Every rejection here names the key an operator has to go and edit. Nothing
//! here repairs a value — a config is right or it is refused (CLAUDE.md rule 3).

use crate::config::{
    CandidatePolicy, Config, EngineMode, EvalSection, InstrumentSection, MAX_CANDIDATE_RADIUS,
    MAX_TT_BYTES, MIN_TT_BYTES, SCHEMA_VERSION, SearchSection,
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

        let CandidatePolicy::Radius { radius } = self.candidate_policy;
        if radius == 0 || radius > MAX_CANDIDATE_RADIUS {
            return Err(EngineError::config(
                "search.candidate_policy.radius",
                format!("must be in 1..={MAX_CANDIDATE_RADIUS}, got {radius}"),
            ));
        }

        Ok(())
    }
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
