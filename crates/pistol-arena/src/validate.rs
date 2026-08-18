//! The cross-field rules, which `serde` cannot express.
//!
//! Every refusal names the exact key an operator can go and edit, and every
//! bound here is a REJECTION rather than a value: nothing in this file
//! completes a configuration, so a document that omits a key is still an error
//! (docs/decisions.md D-18).

use crate::config::{ARENA_SCHEMA_VERSION, ArenaConfig, BudgetSection};
use crate::error::ArenaError;
use crate::sprt::Bounds;

/// The largest worker count this build accepts.
///
/// A typo ceiling, not a value: it catches a document asking for a million
/// concurrent games. It cannot catch "more than this machine has", because that
/// answer differs between machines and validation may not ask.
pub const MAX_WORKERS: usize = 4096;

/// The largest opening count this build accepts, as a typo ceiling.
pub const MAX_OPENINGS_TAKE: usize = 1_000_000;

impl ArenaConfig {
    /// Apply every rule `serde` could not.
    pub fn validate(&self) -> Result<(), ArenaError> {
        if self.schema_version != ARENA_SCHEMA_VERSION {
            return Err(ArenaError::config(
                "schema_version",
                format!(
                    "this build reads arena schema {ARENA_SCHEMA_VERSION}, and the document is \
                     written against {}. This is the ARENA's schema and is unrelated to the \
                     engine config's version.",
                    self.schema_version
                ),
            ));
        }

        self.validate_budget()?;
        self.validate_run()?;
        self.validate_sprt()?;
        self.validate_engines()
    }

    /// The one refusal this crate exists to make loudly.
    fn validate_budget(&self) -> Result<(), ArenaError> {
        if let BudgetSection::MovetimeMs { value } = self.budget {
            return Err(ArenaError::MovetimeBudgetRefused {
                asked: format!("budget.kind = \"movetime_ms\", budget.value = {value}"),
            });
        }
        let (kind, value) = self.budget.report_tokens();
        if value == 0 {
            return Err(ArenaError::config(
                "budget.value",
                format!("a budget of zero searches nothing; `{kind}` needs an amount above zero"),
            ));
        }
        Ok(())
    }

    fn validate_run(&self) -> Result<(), ArenaError> {
        let run = &self.run;
        if run.openings_take == 0 {
            return Err(ArenaError::config(
                "run.openings_take",
                "a run with no openings measures nothing",
            ));
        }
        if run.openings_take > MAX_OPENINGS_TAKE {
            return Err(ArenaError::config(
                "run.openings_take",
                format!("at most {MAX_OPENINGS_TAKE}; this is a typo ceiling, not a sizing rule"),
            ));
        }
        if run.n_workers == 0 {
            return Err(ArenaError::config(
                "run.n_workers",
                "a run needs at least one worker",
            ));
        }
        if run.n_workers > MAX_WORKERS {
            return Err(ArenaError::config(
                "run.n_workers",
                format!("at most {MAX_WORKERS}; this is a typo ceiling, not a sizing rule"),
            ));
        }
        if run.hang_timeout_ms == 0 {
            return Err(ArenaError::config(
                "run.hang_timeout_ms",
                "a watchdog of zero would abandon every run on its first turn",
            ));
        }
        // `turn_cap` is checked against the openings file's own turn count when
        // that file is read, because the number it must exceed is a property of
        // the document and not of this one (openings.rs).
        if run.turn_cap == 0 {
            return Err(ArenaError::config(
                "run.turn_cap",
                "a cap of zero ends every game before it starts",
            ));
        }
        Ok(())
    }

    fn validate_sprt(&self) -> Result<(), ArenaError> {
        let sprt = &self.sprt;
        for (key, value) in [("sprt.alpha", sprt.alpha), ("sprt.beta", sprt.beta)] {
            if !(value > 0.0 && value < 1.0) {
                return Err(ArenaError::config(
                    key,
                    format!("an error rate is strictly between 0 and 1; got {value:.3e}"),
                ));
            }
        }
        if sprt.alpha + sprt.beta >= 1.0 {
            return Err(ArenaError::config(
                "sprt.alpha",
                format!(
                    "alpha + beta must be below 1, or the two boundaries cross and every sample \
                     decides both ways at once; got {:.3e} + {:.3e}",
                    sprt.alpha, sprt.beta
                ),
            ));
        }
        for (key, value) in [("sprt.elo0", sprt.elo0), ("sprt.elo1", sprt.elo1)] {
            if !value.is_finite() {
                return Err(ArenaError::config(
                    key,
                    format!("a normalized-Elo bound is a finite number; got {value}"),
                ));
            }
        }
        // The bounds are computed here rather than trusted later. `alpha > 0.0`
        // admits subnormals, and `ln((1 - beta)/alpha)` then overflows to
        // infinity — which put a non-numeric token in the verdict block and made
        // H1 permanently unreachable, a silently mis-calibrated one-sided test
        // that nothing refused (docs/decisions.md D-173). A bound is a rejection
        // here, not a value: this refuses an impossible document rather than
        // clamping it.
        let bounds = Bounds::of(sprt.alpha, sprt.beta);
        if !bounds.h0.is_finite() || !bounds.h1.is_finite() {
            return Err(ArenaError::config(
                "sprt.alpha",
                format!(
                    // Exponent form: `Display` on a subnormal spells out every
                    // digit, which is a several-hundred-character refusal.
                    "alpha {:.3e} and beta {:.3e} give Wald boundaries of {} and {}, and a \
                     boundary that is not a finite number can never be crossed — H1 would be \
                     unreachable and the report would carry a non-numeric verdict field. Error \
                     rates this small are outside what an f64 log can express.",
                    sprt.alpha, sprt.beta, bounds.h0, bounds.h1
                ),
            ));
        }
        if sprt.elo1 <= sprt.elo0 {
            return Err(ArenaError::config(
                "sprt.elo1",
                format!(
                    "the alternative must exceed the null, or the test is oriented backwards; \
                     got elo0 = {} and elo1 = {}",
                    sprt.elo0, sprt.elo1
                ),
            ));
        }
        Ok(())
    }

    fn validate_engines(&self) -> Result<(), ArenaError> {
        for (key, engine) in [("engine_a", &self.engine_a), ("engine_b", &self.engine_b)] {
            if engine.label.trim().is_empty() {
                return Err(ArenaError::config(
                    format!("{key}.label"),
                    "a label names a side in every line of the report and cannot be blank",
                ));
            }
            if engine.label.split_whitespace().count() != 1 {
                return Err(ArenaError::config(
                    format!("{key}.label"),
                    format!(
                        "a label is one whitespace-free token, because the report is \
                         whitespace-delimited; got `{}`",
                        engine.label
                    ),
                ));
            }
            // Shape only, never existence: validation stays pure and offline,
            // and a missing binary is the spawn's loud error
            // (docs/decisions.md D-21).
            if engine.binary.as_os_str().is_empty() {
                return Err(ArenaError::config(format!("{key}.binary"), "no path given"));
            }
            if engine.config.as_os_str().is_empty() {
                return Err(ArenaError::config(format!("{key}.config"), "no path given"));
            }
        }
        if self.engine_a.label == self.engine_b.label {
            return Err(ArenaError::config(
                "engine_b.label",
                format!(
                    "the two sides must be told apart in the report; both are labelled `{}`",
                    self.engine_a.label
                ),
            ));
        }
        Ok(())
    }
}
