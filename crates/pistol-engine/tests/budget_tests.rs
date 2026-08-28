use pistol_engine::config::EngineMode;
use pistol_engine::{Budget, EngineError};

#[test]
fn budget_absent_is_named_error() {
    assert_eq!(Budget::require(None), Err(EngineError::BudgetMissing));

    // And through the full gate, which is what callers actually use: no budget
    // is refused before anything else is even looked at.
    assert_eq!(
        Budget::resolve(None, EngineMode::Play),
        Err(EngineError::BudgetMissing)
    );
    assert_eq!(
        Budget::resolve(None, EngineMode::Instrument),
        Err(EngineError::BudgetMissing)
    );

    let rendered = EngineError::BudgetMissing.to_string();
    assert!(
        rendered.contains("budget"),
        "the error must say what was missing: {rendered}"
    );
}

#[test]
fn budget_present_is_returned_unchanged() {
    for budget in [Budget::DepthTurns(4), Budget::Nodes(100_000)] {
        assert_eq!(Budget::require(Some(budget)), Ok(budget));
        assert_eq!(
            Budget::resolve(Some(budget), EngineMode::Instrument),
            Ok(budget)
        );
        assert_eq!(Budget::resolve(Some(budget), EngineMode::Play), Ok(budget));
    }
}

#[test]
fn budget_of_zero_is_named_error() {
    for (budget, key) in [
        (Budget::DepthTurns(0), "budget.depth_turns"),
        (Budget::Nodes(0), "budget.nodes"),
        (Budget::MovetimeMs(0), "budget.movetime_ms"),
    ] {
        match Budget::resolve(Some(budget), EngineMode::Play) {
            Err(EngineError::Config { key: got, why }) => {
                assert_eq!(got, key, "{budget:?} named the wrong key");
                assert!(why.contains("at least 1"), "{budget:?} gave: {why}");
            }
            other => panic!("{budget:?} should be refused, got: {other:?}"),
        }
    }
}

/// Instrument mode is where every strength claim comes from, so it takes only
/// budgets that two runs must agree on (CLAUDE.md rule 4).
#[test]
fn instrument_mode_rejects_wall_clock_budget() {
    assert_eq!(
        Budget::resolve(Some(Budget::MovetimeMs(500)), EngineMode::Instrument),
        Err(EngineError::InstrumentBudgetUnsupported)
    );
    assert!(!Budget::MovetimeMs(500).is_reproducible());
    assert!(Budget::DepthTurns(6).is_reproducible());
    assert!(Budget::Nodes(1_000_000).is_reproducible());
}

#[test]
fn play_mode_accepts_wall_clock_budget() {
    let budget = Budget::MovetimeMs(500);
    assert_eq!(Budget::resolve(Some(budget), EngineMode::Play), Ok(budget));
}

#[test]
fn every_budget_kind_names_itself() {
    assert_eq!(Budget::DepthTurns(1).key(), "budget.depth_turns");
    assert_eq!(Budget::Nodes(1).key(), "budget.nodes");
    assert_eq!(Budget::MovetimeMs(1).key(), "budget.movetime_ms");
}

/// The RED-TEAM's F1 (WP-1.4): `movetime 18446744073709551615` used to be
/// ACCEPTED — the `Instant::checked_add` refusal in the deadline translation is
/// unreachable on this platform, where u64::MAX milliseconds fits an `Instant`
/// — turning a fat-fingered budget into a multi-century search with no stop
/// verb to end it. A movetime past [`pistol_engine::budget::MAX_MOVETIME_MS`]
/// is now refused by name, offline, like every other absurd-value typo
/// (docs/decisions.md D-18).
#[test]
fn budget_rejects_absurd_movetime_by_name() {
    use pistol_engine::budget::MAX_MOVETIME_MS;

    for millis in [MAX_MOVETIME_MS + 1, u64::MAX] {
        match Budget::resolve(Some(Budget::MovetimeMs(millis)), EngineMode::Play) {
            Err(EngineError::Config { key, why }) => {
                assert_eq!(key, "budget.movetime_ms");
                assert!(
                    why.contains(&MAX_MOVETIME_MS.to_string()),
                    "the refusal should state the bound: {why}"
                );
            }
            other => panic!("movetime {millis} should be refused, got: {other:?}"),
        }
    }

    // The bound is a rejection of the typo class, not a narrowing of use: the
    // largest accepted value passes the whole gate.
    assert_eq!(
        Budget::resolve(Some(Budget::MovetimeMs(MAX_MOVETIME_MS)), EngineMode::Play),
        Ok(Budget::MovetimeMs(MAX_MOVETIME_MS))
    );
}
