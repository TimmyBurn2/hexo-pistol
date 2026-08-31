use pistol_core::{Coord, GameState};
use pistol_solver::pn::Epsilon;
use pistol_solver::{AttackerPolicy, Solver, ThreatState};

fn state_of(cells: &[&str]) -> GameState {
    let mut state = GameState::new_game();
    for cell in cells {
        let coord: Coord = cell.parse().expect("a coordinate");
        state.place(coord).expect("a legal stone");
    }
    state
}

fn solver() -> Solver {
    Solver::new(
        Epsilon::new(1, 4).expect("1/4 is valid"),
        1 << 16,
        AttackerPolicy::OneFreeStone,
    )
}

/// A turn boundary where P1 holds three in a line and P2 two, so no live window
/// holds four for either side.
const NEITHER_SIDE_HOT: &[&str] = &["0,0", "1,0", "2,0", "0,1", "0,2"];

#[test]
fn the_position_the_precondition_is_about_really_has_no_hot_window() {
    let state = state_of(NEITHER_SIDE_HOT);
    let mut threats = ThreatState::new();
    for (at, player) in state.board().stones() {
        threats.apply(at, player);
    }
    let mover = state.to_move();
    assert_eq!(threats.hot_windows(mover).len(), 0);
    assert_eq!(threats.hot_windows(mover.opponent()).len(), 0);
}

#[test]
fn the_attacker_direction_answers_where_neither_side_is_hot() {
    // The asymmetry is the whole finding: `solve` has an OR step above every
    // AND node it reaches, so it needs no hot window at its root.
    let result = solver().solve(&state_of(NEITHER_SIDE_HOT), 512);
    assert!(matches!(
        result.outcome,
        pistol_solver::SolveOutcome::NoWin | pistol_solver::SolveOutcome::Unknown
    ));
}

#[test]
#[should_panic(expected = "SOLVER_NO_PLAN")]
fn the_defender_direction_refuses_a_position_where_neither_side_is_hot() {
    // Fail-loud rather than an answer about a question the AND root cannot
    // pose. `pistol-search`'s trigger never asks it; a probe that skips the
    // trigger can, and this is what it gets (CLAUDE.md rule 3).
    solver().solve_defender(&state_of(NEITHER_SIDE_HOT), 512);
}
