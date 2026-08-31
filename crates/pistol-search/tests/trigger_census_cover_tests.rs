use pistol_core::{Coord, GameState};
use pistol_eval::{HandcraftedV0, Weights};
use pistol_search::census::CoverClass;
use pistol_search::params::{SolverTrigger, SolverWiring};
use pistol_search::{SearchParams, Searcher, Stop};
use pistol_solver::pn::Epsilon;
use pistol_solver::{AttackerPolicy, Cover, HitBudget, SolverParams, StonesLeft, ThreatState};

fn weights() -> Weights {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../configs/eval_v0_weights.toml");
    Weights::load(&path).expect("the committed weights load")
}

fn searcher(cap: u64) -> Searcher {
    let params = SearchParams {
        tt_bytes: 1 << 24,
        solver: Some(SolverWiring {
            per_call_node_cap: cap,
            trigger: SolverTrigger::AnyOpenFour,
            inner: SolverParams {
                epsilon: Epsilon::new(1, 4).expect("1/4 is valid"),
                tt_entries: 1 << 20,
                attacker_policy: AttackerPolicy::OneFreeStone,
            },
        }),
        candidate_policy: pistol_search::CandidatePolicy::Staged(pistol_search::StagedParams {
            quiet_radius: 2,
            safety_net_top_k: 0,
            tier_t_own_count: 2,
            tier_t_opponent_count: 3,
            q_depth_turns: 0,
            q_triggers: pistol_search::QTriggers::DefensiveOnly,
            ordering: pistol_search::OrderingHeuristics {
                killers: false,
                history: false,
                countermove: false,
            },
        }),
    };
    Searcher::new(params, Box::new(HandcraftedV0::new(weights())))
        .expect("the wiring's parameters are accepted")
}

fn state_of(cells: &[&str]) -> GameState {
    let mut state = GameState::new_game();
    for cell in cells {
        let coord: Coord = cell.parse().expect("a coordinate");
        state.place(coord).expect("a legal stone");
    }
    state
}

/// The cover class of a position, computed from the board and sharing no code
/// with the census: the census reads a maintained `ThreatState` the search
/// carries incrementally, this one is built from the stones in one pass.
fn cover_from_the_board(state: &GameState) -> CoverClass {
    let mut threats = ThreatState::new();
    for (at, player) in state.board().stones() {
        threats.apply(at, player);
    }
    let left = StonesLeft::from_state(state).expect("an undecided position owes stones");
    match threats.blocking_covers(state.to_move(), HitBudget::from(left)) {
        Cover::NothingToBlock => CoverClass::NothingToBlock,
        Cover::Impossible => CoverClass::Impossible,
        Cover::Minimal(covers) => CoverClass::Minimal(covers.len()),
    }
}

/// A midgame both seats reach hot — the position `solver_call_counter_tests`
/// uses for the same reason, so the two instruments are read on one workload.
const HOT_MIDGAME: &[&str] = &[
    "0,0", "0,-1", "1,0", "-1,1", "1,-1", "-4,4", "2,-2", "-2,1", "0,1", "-4,1", "1,1", "-4,3",
    "-3,2", "-5,2", "-5,4", "-7,4", "-2,4", "-3,0", "-2,-1", "-1,-2", "0,4", "0,3", "1,2", "-1,4",
    "4,-1", "-2,3", "-1,3", "1,3", "4,0", "3,-2", "4,-2", "1,-2", "6,-2", "0,2", "2,2",
];

/// `bench_positions_v1.txt`'s first band-15 entry (`src 00070cdd8fb87f42`),
/// stone for stone. A COMMITTED workload rather than a hand-built one, so the
/// property below is pinned on the same positions the option matrix is ranked
/// over rather than on a position chosen to make it hold.
const BENCH_B15_FIRST: &[&str] = &[
    "0,0", "-1,1", "1,0", "0,1", "0,2", "-1,0", "1,-1", "0,-1", "1,-2", "0,-2", "0,3", "-1,-1",
    "1,1", "-1,2", "-1,3",
];

#[test]
fn the_root_row_reports_the_cover_the_board_itself_answers() {
    let state = state_of(HOT_MIDGAME);
    let mut engine = searcher(512);
    engine.collect_trigger_census();
    engine
        .search(&state, Stop::Nodes(4_000), &mut |_| {})
        .expect("the search runs");
    let rows = engine.take_trigger_census();
    let root = rows
        .iter()
        .find(|row| row.columns.turns_from_root == 0)
        .expect("the root fires the trigger at this position");
    assert_eq!(root.columns.cover, cover_from_the_board(&state));
}

#[test]
fn the_cover_column_is_not_one_constant_wearing_a_column_name() {
    let mut engine = searcher(512);
    engine.collect_trigger_census();
    engine
        .search(&state_of(BENCH_B15_FIRST), Stop::Nodes(8_000), &mut |_| {})
        .expect("the search runs");
    let rows = engine.take_trigger_census();
    let distinct: std::collections::BTreeSet<_> = rows
        .iter()
        .map(|row| (row.columns.cover.token(), row.columns.cover.count()))
        .collect();
    assert!(
        distinct.len() >= 2,
        "one committed position's census reports a single cover value {distinct:?}: \
         a constant is not a column"
    );
}

/// `bench_positions_v1.txt`'s `src 006b761dec684d9c` (band 35), whose root
/// fires with the MOVER hot and the opponent not.
const BENCH_MOVER_HOT_ROOT: &[&str] = &[
    "0,0", "0,-1", "1,0", "0,-3", "1,-1", "-1,0", "0,1", "-1,1", "3,-3", "1,-2", "2,-2", "2,-3",
    "3,-2", "-2,1", "0,-2", "-3,2", "-2,2", "-3,3", "-2,3", "1,-3", "3,-1", "-1,-3", "4,-3",
    "-1,2", "2,-1", "0,2", "4,-1", "-4,2", "3,-4", "-5,2", "3,0",
];

#[test]
fn the_cover_is_asked_of_the_movers_side_and_swapping_it_would_show() {
    // The root here is hot for the MOVER and not for the opponent, so
    // `blocking_covers(mover, ..)` — "what is aimed at me" — has nothing to
    // block. A cover asked of the OTHER side would find the mover's own hot
    // window and answer `Minimal` or `Impossible`, so this position tells the
    // two apart and a position hot for both could not.
    let state = state_of(BENCH_MOVER_HOT_ROOT);
    let mut engine = searcher(512);
    engine.collect_trigger_census();
    engine
        .search(&state, Stop::Nodes(2_000), &mut |_| {})
        .expect("the search runs");
    let rows = engine.take_trigger_census();
    let root = rows
        .iter()
        .find(|row| row.columns.turns_from_root == 0)
        .expect("the root fires the trigger at this position");
    assert!(root.columns.mover_hot > 0, "the mover is the hot side here");
    assert_eq!(root.columns.opponent_hot, 0, "and the opponent is not");
    assert_eq!(root.columns.cover, CoverClass::NothingToBlock);
    assert_eq!(root.columns.cover, cover_from_the_board(&state));
}
