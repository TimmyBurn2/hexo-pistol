mod common;

use pistol_core::{Coord, GameState};
use pistol_search::staged::{StagedRow, StagedSet, staged_candidates};
use pistol_search::{OrderingHeuristics, QTriggers, StagedParams};

use common::{committed_weights, threats_for};
use pistol_eval::HandcraftedV0;

/// One step along `dir`, `n` of them from the origin.
fn step(dir: Coord, n: i16) -> Coord {
    Coord::new(dir.q * n, dir.r * n)
}

/// A cell `n` steps along `dir`, offset `k` steps along the perpendicular
/// direction — where the harmless "keep the turn structure legal" stones go,
/// clear of the line `dir` itself builds.
fn spectator(dir: Coord, n: i16, k: i16) -> Coord {
    let perp = Coord::new(-dir.r, dir.q);
    let base = step(dir, n);
    Coord::new(base.q + perp.q * k, base.r + perp.r * k)
}

/// The bridge every colony case shares: nine stones from the origin to
/// `step(dir, 32)`, each placement within radius 8 of the one before it
/// (spacing 4), turns 1 through 5. Verified once here rather than per case:
/// `GameState::place` panics loudly on an illegal cell, so a construction bug
/// in any of the six cases below fails at the position it was building, not
/// silently.
fn bridge(state: &mut GameState, dir: Coord) {
    for n in [0, 4, 8, 12, 16, 20, 24, 28, 32] {
        state
            .place(step(dir, n))
            .unwrap_or_else(|error| panic!("colony bridge step {n} along {dir:?}: {error}"));
    }
}

/// A WIN-NOW colony: past the bridge, P2 blocks one end of a P1 four
/// (`step(39)`), P1 extends it to five over its next two turns
/// (`step(40..43)`), and P2's harmless turns keep the game legal without
/// touching the line. P1's own hot-4 window (empties `step(44)`/`step(45)`)
/// is the win-now pair — the same shape `WinWitness::Pair` names — sitting
/// forty-plus cells from the origin.
fn win_now_colony(dir: Coord) -> GameState {
    let mut state = GameState::new_game();
    bridge(&mut state, dir);
    for at in [
        step(dir, 39),
        spectator(dir, 41, 3),
        step(dir, 40),
        step(dir, 42),
        spectator(dir, 43, 3),
        spectator(dir, 44, 3),
        step(dir, 41),
        step(dir, 43),
        spectator(dir, 45, 3),
        spectator(dir, 46, 3),
    ] {
        state
            .place(at)
            .unwrap_or_else(|error| panic!("colony win-now tail along {dir:?}: {error}"));
    }
    state
}

/// A FILTERED colony: past the bridge, P2 builds its own four
/// (`step(37..41)`) blocked at one end by P1's own bridge-end stone
/// (`step(32)`, four cells short of `step(37)`, D-243's own counting
/// identity), leaving P1 to move with three overlapping hot windows to
/// cover — a non-trivial `Cover::Minimal` far from the origin.
fn filtered_colony(dir: Coord) -> GameState {
    let mut state = GameState::new_game();
    bridge(&mut state, dir);
    for at in [
        step(dir, 37),
        step(dir, 38),
        spectator(dir, 37, 3),
        spectator(dir, 38, 3),
        step(dir, 39),
        step(dir, 40),
    ] {
        state
            .place(at)
            .unwrap_or_else(|error| panic!("colony filtered tail along {dir:?}: {error}"));
    }
    state
}

fn assert_win_now(name: &str, dir: Coord) {
    let state = win_now_colony(dir);
    let threats = threats_for(&state);
    let mut eval = Box::new(HandcraftedV0::new(committed_weights()));
    let params = StagedParams {
        quiet_radius: 2,
        safety_net_top_k: 0,
        tier_t_own_count: 2,
        tier_t_opponent_count: 3,
        q_depth_turns: 0,
        q_triggers: QTriggers::DefensiveAndOffensive,
        ordering: OrderingHeuristics {
            killers: false,
            history: false,
            countermove: false,
        },
    };
    let mut out = StagedSet::default();
    let row = staged_candidates(&state, &threats, &mut *eval, false, params, &mut out);
    assert_eq!(
        row,
        StagedRow::WinNow,
        "{name}: the distant hot-4 window is a win-now pair"
    );
    assert_eq!(
        out.cells,
        vec![step(dir, 44), step(dir, 45)],
        "{name}: the win-now cells are the distant window's own empties, found regardless of \
         how far they sit from the origin"
    );
    assert_eq!(
        out.forced,
        out.cells.len(),
        "{name}: the whole set is forced"
    );
}

fn assert_filtered(name: &str, dir: Coord) {
    let state = filtered_colony(dir);
    let threats = threats_for(&state);
    let mut eval = Box::new(HandcraftedV0::new(committed_weights()));
    let params = StagedParams {
        quiet_radius: 2,
        safety_net_top_k: 0,
        tier_t_own_count: 2,
        tier_t_opponent_count: 3,
        q_depth_turns: 0,
        q_triggers: QTriggers::DefensiveAndOffensive,
        ordering: OrderingHeuristics {
            killers: false,
            history: false,
            countermove: false,
        },
    };
    let mut out = StagedSet::default();
    let row = staged_candidates(&state, &threats, &mut *eval, false, params, &mut out);
    assert_eq!(
        row,
        StagedRow::Filtered,
        "{name}: P2's distant hot windows need covering"
    );
    assert_eq!(
        out.cells,
        vec![step(dir, 35), step(dir, 36), step(dir, 41), step(dir, 42),],
        "{name}: the cover union over three overlapping distant hot windows"
    );
    assert_eq!(
        out.forced,
        out.cells.len(),
        "{name}: a FILTERED row is entirely forced"
    );
}

#[test]
fn win_now_colony_on_const_r() {
    assert_win_now("ConstR", Coord::new(1, 0));
}

#[test]
fn win_now_colony_on_const_q() {
    assert_win_now("ConstQ", Coord::new(0, 1));
}

#[test]
fn win_now_colony_on_const_s() {
    assert_win_now("ConstS", Coord::new(1, -1));
}

#[test]
fn filtered_colony_on_const_r() {
    assert_filtered("ConstR", Coord::new(1, 0));
}

#[test]
fn filtered_colony_on_const_q() {
    assert_filtered("ConstQ", Coord::new(0, 1));
}

#[test]
fn filtered_colony_on_const_s() {
    assert_filtered("ConstS", Coord::new(1, -1));
}
