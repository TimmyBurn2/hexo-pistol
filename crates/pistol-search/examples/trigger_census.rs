//! One row per solver trigger firing, over a fixture of positions.
//!
//! The premise memo's §3.6 named the measurement that ranks an option field
//! without arguing about it: *for each candidate narrowing of the trigger, what
//! FRACTION of the present trigger's firings does it keep?* Every candidate is
//! a predicate over the columns this prints, so ONE run answers it for the
//! whole field — and the same rows carry which firings PROVED, which is the
//! axis D-516 makes the matrix's rows comparable on.
//!
//! It prints and judges nothing. A row is an observation; the reading is the
//! matrix's.
//!
//! Usage:
//!   trigger_census --fixture <path> --nodes <n> --cap <n> [--quiet-radius <n>]
//! Exit:
//!   0 read and censused
//!   1 an argument or a fixture line this build refuses
//!   2 THE RUN IS VOID — the fixture is unreadable

use std::process::ExitCode;

use pistol_core::{Coord, GameState};
use pistol_eval::{HandcraftedV0, Weights};
use pistol_search::params::{SolverTrigger, SolverWiring};
use pistol_search::{
    CandidatePolicy, OrderingHeuristics, QTriggers, SearchParams, Searcher, StagedParams, Stop,
};
use pistol_solver::pn::Epsilon;
use pistol_solver::{AttackerPolicy, SolverParams};

fn void(why: &str) -> ExitCode {
    eprintln!("trigger_census: RUN VOID: {why}");
    ExitCode::from(2)
}

fn fail(why: &str) -> ExitCode {
    eprintln!("trigger_census: FAIL: {why}");
    ExitCode::from(1)
}

/// The stones of one fixture entry, in play order.
///
/// The `position` verb's own grammar, read here rather than through
/// `PositionSpec` because this crate does not depend on the one that owns it —
/// a turn is one cell or two separated by `/`, and the rules judge each stone
/// as it goes down (CLAUDE.md rule 2).
fn state_of(tail: &str) -> Result<GameState, String> {
    let body = tail.split('#').next().unwrap_or("");
    let mut words = body.split_whitespace();
    if words.next() != Some("start") || words.next() != Some("moves") {
        return Err(format!("not a `start moves` entry: {body}"));
    }
    let mut state = GameState::new_game();
    for turn in words {
        for cell in turn.split('/') {
            let coord: Coord = cell.parse().map_err(|why| format!("{cell}: {why}"))?;
            state.place(coord).map_err(|why| format!("{cell}: {why}"))?;
        }
    }
    Ok(state)
}

struct Args {
    fixture: String,
    nodes: u64,
    cap: u64,
    quiet_radius: u32,
}

fn parse(words: &[String]) -> Result<Args, String> {
    let mut fixture = None;
    let mut nodes = None;
    let mut cap = None;
    let mut quiet_radius = 2;
    let mut index = 0;
    while index < words.len() {
        let key = words[index].as_str();
        let value = words
            .get(index + 1)
            .ok_or_else(|| format!("{key} wants a value"))?;
        match key {
            "--fixture" => fixture = Some(value.clone()),
            "--nodes" => nodes = Some(value.parse().map_err(|why| format!("--nodes: {why}"))?),
            "--cap" => cap = Some(value.parse().map_err(|why| format!("--cap: {why}"))?),
            "--quiet-radius" => {
                quiet_radius = value
                    .parse()
                    .map_err(|why| format!("--quiet-radius: {why}"))?;
            }
            other => return Err(format!("unknown option {other}")),
        }
        index += 2;
    }
    Ok(Args {
        fixture: fixture.ok_or("--fixture is required")?,
        nodes: nodes.ok_or("--nodes is required: the budget is never guessed")?,
        cap: cap.ok_or("--cap is required: a call count without its cap is not a quantity")?,
        quiet_radius,
    })
}

fn searcher(args: &Args) -> Result<Searcher, String> {
    let weights_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../configs/eval_v0_weights.toml");
    let weights = Weights::load(&weights_path).map_err(|why| format!("weights: {why}"))?;
    let params = SearchParams {
        tt_bytes: 1 << 28,
        solver: Some(SolverWiring {
            per_call_node_cap: args.cap,
            trigger: SolverTrigger::AnyOpenFour,
            inner: SolverParams {
                epsilon: Epsilon::new(1, 4).ok_or("epsilon 1/4 is not valid")?,
                tt_entries: 1 << 20,
                attacker_policy: AttackerPolicy::OneFreeStone,
            },
        }),
        candidate_policy: CandidatePolicy::Staged(StagedParams {
            quiet_radius: args.quiet_radius,
            safety_net_top_k: 0,
            tier_t_own_count: 2,
            tier_t_opponent_count: 3,
            q_depth_turns: 0,
            q_triggers: QTriggers::DefensiveOnly,
            ordering: OrderingHeuristics {
                killers: false,
                history: false,
                countermove: false,
            },
        }),
    };
    Searcher::new(params, Box::new(HandcraftedV0::new(weights)))
        .map_err(|why| format!("searcher: {why}"))
}

fn main() -> ExitCode {
    let words: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse(&words) {
        Ok(args) => args,
        Err(why) => return fail(&why),
    };
    let text = match std::fs::read_to_string(&args.fixture) {
        Ok(text) => text,
        Err(why) => return void(&format!("cannot read {}: {why}", args.fixture)),
    };
    let mut engine = match searcher(&args) {
        Ok(engine) => engine,
        Err(why) => return fail(&why),
    };
    engine.collect_trigger_census();

    println!("trigger_census: argv {}", words.join(" "));
    println!(
        "trigger_census: fixture {} nodes {} cap {} quiet_radius {}",
        args.fixture, args.nodes, args.cap, args.quiet_radius
    );
    let mut entries = 0u32;
    for line in text.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let state = match state_of(line) {
            Ok(state) => state,
            Err(why) => return fail(&format!("entry {entries}: {why}")),
        };
        let outcome = match engine.search(&state, Stop::Nodes(args.nodes), &mut |_| {}) {
            Ok(outcome) => outcome,
            Err(why) => return fail(&format!("entry {entries}: {why}")),
        };
        let calls = outcome.info.solver_calls;
        println!(
            "trigger_census: entry {entries} search_nodes {} solver_nodes {} firings {} \
             invocations {} proofs {} root_nodes {}",
            outcome.info.search_nodes,
            outcome.info.solver_nodes,
            calls.firings,
            calls.invocations,
            calls.proofs,
            calls.root_nodes
        );
        for row in engine.take_trigger_census() {
            println!(
                "trigger_census: row entry {entries} turns {} mover_hot {} opp_hot {} \
                 mover_w1 {} opp_w1 {} mover_l3 {} opp_l3 {} att_visits {} att_proved {} \
                 def_asked {} def_visits {} def_proved {}",
                row.turns_from_root,
                row.mover_hot,
                row.opponent_hot,
                row.mover_win_in_one_ply,
                row.opponent_win_in_one_ply,
                row.mover_live_three,
                row.opponent_live_three,
                row.attacker.visits,
                row.attacker.proved,
                row.defender.is_some(),
                row.defender.map_or(0, |answer| answer.visits),
                row.defender.is_some_and(|answer| answer.proved)
            );
        }
        entries += 1;
    }
    if entries == 0 {
        return fail(&format!("{} holds no entry", args.fixture));
    }
    println!("trigger_census: done {entries} entries");
    ExitCode::SUCCESS
}
