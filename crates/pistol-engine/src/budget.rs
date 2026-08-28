use crate::config::EngineMode;
use crate::error::EngineError;

/// Largest wall-clock budget this build accepts, in milliseconds: one hour.
///
/// A rejection bound, not a value (docs/decisions.md D-18), catching the typo
/// class offline: the deployment design point is half a second, and the
/// platform's `Instant` representation holds u64::MAX milliseconds without
/// overflowing, so without this bound a fat-fingered `movetime` became a
/// multi-century search no protocol verb can end (WP-1.4 RED-TEAM finding F1).
pub const MAX_MOVETIME_MS: u64 = 3_600_000;

/// Every way to bound a search.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Budget {
    /// Search to a fixed depth in **turns**, not plies: a turn is two stones
    /// after the first. Reproducible.
    DepthTurns(u32),
    /// Search a fixed number of nodes. Reproducible.
    Nodes(u64),
    /// Search for a wall-clock duration in milliseconds. Not reproducible, and
    /// therefore refused by instrument mode.
    MovetimeMs(u64),
}

impl Budget {
    /// The config-style key naming this budget kind.
    pub const fn key(self) -> &'static str {
        match self {
            Budget::DepthTurns(_) => "budget.depth_turns",
            Budget::Nodes(_) => "budget.nodes",
            Budget::MovetimeMs(_) => "budget.movetime_ms",
        }
    }

    /// Whether two runs of this budget on the same position must agree.
    pub const fn is_reproducible(self) -> bool {
        match self {
            Budget::DepthTurns(_) | Budget::Nodes(_) => true,
            Budget::MovetimeMs(_) => false,
        }
    }

    /// Turn a caller's optional budget into a budget, or into the named error
    /// that says one was required.
    pub fn require(budget: Option<Budget>) -> Result<Budget, EngineError> {
        budget.ok_or(EngineError::BudgetMissing)
    }

    /// Reject a budget that asks for no work at all.
    pub fn validate(self) -> Result<(), EngineError> {
        let amount = match self {
            Budget::DepthTurns(turns) => u64::from(turns),
            Budget::Nodes(nodes) => nodes,
            Budget::MovetimeMs(millis) => millis,
        };
        if amount == 0 {
            return Err(EngineError::config(self.key(), "must be at least 1, got 0"));
        }
        if let Budget::MovetimeMs(millis) = self
            && millis > MAX_MOVETIME_MS
        {
            return Err(EngineError::config(
                self.key(),
                format!("must be at most {MAX_MOVETIME_MS} ms (one hour), got {millis}"),
            ));
        }
        Ok(())
    }

    /// Reject a budget the given mode cannot honour.
    pub fn check_supported(self, mode: EngineMode) -> Result<(), EngineError> {
        match mode {
            EngineMode::Instrument if !self.is_reproducible() => {
                Err(EngineError::InstrumentBudgetUnsupported)
            }
            _ => Ok(()),
        }
    }

    /// The whole gate in one call: present, non-zero, and legal for the mode.
    pub fn resolve(budget: Option<Budget>, mode: EngineMode) -> Result<Budget, EngineError> {
        let budget = Budget::require(budget)?;
        budget.validate()?;
        budget.check_supported(mode)?;
        Ok(budget)
    }
}
