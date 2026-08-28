use std::str::FromStr;

use pistol_engine::{Budget, EngineError, EngineMode};

use crate::count::plain_count;

use crate::protocol::{GO, protocol, quoted};

/// A fixed depth in turns.
pub const DEPTH_TURNS_BUDGET: &str = "depth_turns";
/// A fixed node count.
pub const NODES_BUDGET: &str = "nodes";
/// A wall-clock duration in milliseconds.
pub const MOVETIME_BUDGET: &str = "movetime";

/// One of each budget kind, to ask the engine's mode which ones it honours.
///
/// The amounts are placeholders; only the kinds are read.
const BUDGET_KINDS: [Budget; 3] = [
    Budget::DepthTurns(1),
    Budget::Nodes(1),
    Budget::MovetimeMs(1),
];

/// The budget a `go` line asks for.
///
/// A `go` with no budget at all is [`EngineError::BudgetMissing`] and not a
/// protocol complaint: a budget is always explicit, and that variant exists to
/// say exactly this (CLAUDE.md rule 1, docs/decisions.md D-4). A budget of zero
/// is likewise the engine's named refusal rather than a parse failure — the line
/// was understood, the amount was not acceptable.
pub(crate) fn parse_budget(line: &str, rest: &str) -> Result<Budget, EngineError> {
    let words: Vec<&str> = rest.split_whitespace().collect();
    match words.as_slice() {
        [] => Err(EngineError::BudgetMissing),
        [kind] => Err(protocol(
            line,
            format!(
                "`{}` needs an amount, as `{GO} {} 4`",
                quoted(kind),
                quoted(kind)
            ),
        )),
        [kind, amount] => budget_of(line, kind, amount),
        [_, _, extra, ..] => Err(protocol(
            line,
            format!(
                "`{GO}` takes one budget, and `{}` follows it",
                quoted(extra)
            ),
        )),
    }
}

/// One budget kind and its amount.
fn budget_of(line: &str, kind: &str, amount: &str) -> Result<Budget, EngineError> {
    match kind {
        DEPTH_TURNS_BUDGET => Ok(Budget::DepthTurns(count(line, kind, amount)?)),
        NODES_BUDGET => Ok(Budget::Nodes(count(line, kind, amount)?)),
        MOVETIME_BUDGET => Ok(Budget::MovetimeMs(count(line, kind, amount)?)),
        other => Err(protocol(
            line,
            format!(
                "unknown budget `{}`; the budgets are {DEPTH_TURNS_BUDGET}, {NODES_BUDGET}, \
                 {MOVETIME_BUDGET}",
                quoted(other)
            ),
        )),
    }
}

/// A plain non-negative decimal amount.
fn count<T: FromStr>(line: &str, kind: &str, amount: &str) -> Result<T, EngineError> {
    plain_count(amount).map_err(|why| {
        protocol(
            line,
            format!(
                "`{}` is not an amount for `{}`: {why}",
                quoted(amount),
                quoted(kind)
            ),
        )
    })
}

/// The budget kinds this mode honours, in protocol spelling.
pub(crate) fn budget_tokens(mode: EngineMode) -> Vec<&'static str> {
    BUDGET_KINDS
        .iter()
        .filter(|kind| kind.check_supported(mode).is_ok())
        .map(|kind| budget_token(*kind))
        .collect()
}

/// The protocol spelling of a budget kind.
fn budget_token(budget: Budget) -> &'static str {
    match budget {
        Budget::DepthTurns(_) => DEPTH_TURNS_BUDGET,
        Budget::Nodes(_) => NODES_BUDGET,
        Budget::MovetimeMs(_) => MOVETIME_BUDGET,
    }
}
