//! The recall gate's own instrument: for each pinned anchor position, the
//! columns a detector decides on, and what the solver answers at each cap.
//!
//! D-512 makes the recall gate TWO gates. This answers the per-position
//! RANKING one — would a candidate predicate admit the firing that carries the
//! proof — and the cap question the census cannot reach: the census records
//! visits under ONE cap, and a row that proposes a smaller cap is proposing to
//! truncate proofs it never priced.
//!
//! Usage:
//!   value_fixture_recall --dir <probe-dir> --case <name> [--case <name>...]
//!                        [--cap <n>...]
//! Exit:
//!   0 every named case was read and probed
//!   1 an argument, or a case file this build refuses
//!   2 THE RUN IS VOID — a case file is unreadable

use std::process::ExitCode;

use pistol_core::{Coord, GameState};
use pistol_search::census::CoverClass;
use pistol_solver::pn::Epsilon;
use pistol_solver::{
    AttackerPolicy, Cover, HitBudget, SolveOutcome, Solver, StonesLeft, ThreatState,
};

fn void(why: &str) -> ExitCode {
    eprintln!("value_fixture_recall: RUN VOID: {why}");
    ExitCode::from(2)
}

fn fail(why: &str) -> ExitCode {
    eprintln!("value_fixture_recall: FAIL: {why}");
    ExitCode::from(1)
}

/// The `plies` line of a probe case file, replayed by the rules.
fn state_of(text: &str) -> Result<GameState, String> {
    let line = text
        .lines()
        .find_map(|line| line.strip_prefix("plies "))
        .ok_or("the case file carries no `plies` line")?;
    let mut state = GameState::new_game();
    for cell in line.split_whitespace() {
        let coord: Coord = cell.parse().map_err(|why| format!("{cell}: {why}"))?;
        state.place(coord).map_err(|why| format!("{cell}: {why}"))?;
    }
    Ok(state)
}

/// The census columns as the ROOT would read them at this position.
///
/// Deliberately NOT the search's own path: the point is the columns a detector
/// sees at a firing, and the fixture's positions are firing points by
/// construction — every one of them is hot.
struct Columns {
    mover_hot: usize,
    opponent_hot: usize,
    mover_w1: usize,
    opponent_w1: usize,
    mover_l3: usize,
    opponent_l3: usize,
    cover: CoverClass,
}

fn columns_of(state: &GameState) -> Result<Columns, String> {
    let mut threats = ThreatState::new();
    for (at, player) in state.board().stones() {
        threats.apply(at, player);
    }
    let mover = state.to_move();
    let opponent = mover.opponent();
    let left = StonesLeft::from_state(state).ok_or("a decided position owes no stones")?;
    let cover = match threats.blocking_covers(mover, HitBudget::from(left)) {
        Cover::NothingToBlock => CoverClass::NothingToBlock,
        Cover::Impossible => CoverClass::Impossible,
        Cover::Minimal(covers) => CoverClass::Minimal(covers.len()),
    };
    let live_three = |side| {
        threats
            .live_windows_at_count(side, pistol_solver::LiveCount::Three)
            .len()
    };
    Ok(Columns {
        mover_hot: threats.hot_windows(mover).len(),
        opponent_hot: threats.hot_windows(opponent).len(),
        mover_w1: threats.win_in_one_ply_windows(mover).len(),
        opponent_w1: threats.win_in_one_ply_windows(opponent).len(),
        mover_l3: live_three(mover),
        opponent_l3: live_three(opponent),
        cover,
    })
}

fn outcome_token(outcome: &SolveOutcome) -> &'static str {
    match outcome {
        SolveOutcome::Win(_) => "win",
        SolveOutcome::NoWin => "nowin",
        SolveOutcome::NoWinUnderZone => "nowin-under-zone",
        SolveOutcome::Unknown => "unknown",
    }
}

fn main() -> ExitCode {
    let words: Vec<String> = std::env::args().skip(1).collect();
    let mut dir = None;
    let mut cases: Vec<String> = Vec::new();
    let mut caps: Vec<u64> = Vec::new();
    let mut index = 0;
    while index < words.len() {
        let key = words[index].as_str();
        let Some(value) = words.get(index + 1) else {
            return fail(&format!("{key} wants a value"));
        };
        match key {
            "--dir" => dir = Some(value.clone()),
            "--case" => cases.push(value.clone()),
            "--cap" => match value.parse::<u64>() {
                Ok(cap) => caps.push(cap),
                Err(why) => return fail(&format!("--cap: {why}")),
            },
            other => return fail(&format!("unknown option {other}")),
        }
        index += 2;
    }
    let Some(dir) = dir else {
        return fail("--dir is required");
    };
    if cases.is_empty() {
        return fail("at least one --case is required: a recall gate names its rows");
    }
    if caps.is_empty() {
        return fail("at least one --cap is required: a proof without its cap is not a quantity");
    }

    println!("value_fixture_recall: argv {}", words.join(" "));
    println!(
        "value_fixture_recall: dir {dir} cases {} caps {}",
        cases.len(),
        caps.iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );
    // ONE solver across every case and cap, the way `solver_verdict` holds one
    // across a search: a fresh table per probe would answer a different
    // question from the one the engine asks.
    let mut solver = Solver::new(
        Epsilon::new(1, 4).expect("1/4 is valid"),
        1 << 20,
        AttackerPolicy::OneFreeStone,
    );
    for case in &cases {
        let path = format!("{dir}/{case}.txt");
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(why) => return void(&format!("cannot read {path}: {why}")),
        };
        let state = match state_of(&text) {
            Ok(state) => state,
            Err(why) => return fail(&format!("{case}: {why}")),
        };
        let columns = match columns_of(&state) {
            Ok(columns) => columns,
            Err(why) => return fail(&format!("{case}: {why}")),
        };
        println!(
            "value_fixture_recall: columns {case} stones {} mover_hot {} opp_hot {} \
             mover_w1 {} opp_w1 {} mover_l3 {} opp_l3 {} cover {} covers {}",
            state.board().stones().count(),
            columns.mover_hot,
            columns.opponent_hot,
            columns.mover_w1,
            columns.opponent_w1,
            columns.mover_l3,
            columns.opponent_l3,
            columns.cover.token(),
            columns.cover.count(),
        );
        // THE TRIGGER'S OWN PRECONDITION, honoured because the solver's is
        // inside it. `solver_verdict` reaches either call only when a side is
        // hot, and `solve_defender` needs it: its root is an AND node, where
        // `dfpn`'s NO_PLAN_ASSERT requires the attacker — here the OPPONENT —
        // to hold a hot window, unless the race check answers first (which it
        // does exactly when the MOVER is hot). Probing a position the trigger
        // does not fire at asks the solver a question the engine never asks,
        // and the assert says so by name.
        if columns.mover_hot == 0 && columns.opponent_hot == 0 {
            println!(
                "value_fixture_recall: probe {case} NOT-A-FIRING-POINT — the incumbent \
                 trigger does not fire here, so there is no trigger point to rank"
            );
            continue;
        }
        for &cap in &caps {
            let attacker = solver.solve(&state, cap);
            let attacker_proved = matches!(attacker.outcome, SolveOutcome::Win(_));
            // The defender direction is asked exactly when the engine asks it:
            // only where the attacker direction did not prove (`pvs.rs`'s
            // `solver_verdict` returns before it otherwise).
            let defender = (!attacker_proved).then(|| solver.solve_defender(&state, cap));
            let firing_visits = attacker.nodes + defender.as_ref().map_or(0, |result| result.nodes);
            println!(
                "value_fixture_recall: probe {case} cap {cap} att {} att_visits {} \
                 def {} def_visits {} firing_visits {} proved {}",
                outcome_token(&attacker.outcome),
                attacker.nodes,
                defender
                    .as_ref()
                    .map_or("skipped", |result| outcome_token(&result.outcome)),
                defender.as_ref().map_or(0, |result| result.nodes),
                firing_visits,
                attacker_proved
                    || defender
                        .as_ref()
                        .is_some_and(|result| matches!(result.outcome, SolveOutcome::Win(_))),
            );
        }
    }
    println!("value_fixture_recall: done {} cases", cases.len());
    ExitCode::SUCCESS
}
