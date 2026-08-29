//! WP-1.5d (B) — THE ONE MEASUREMENT INSTRUMENT.
//!
//! `docs/decisions.md` D-482: every number session (B) consumes downstream comes
//! from ONE run by ONE registered instrument producing ONE artifact. This is
//! that instrument. It emits the calibration sweep, both bench fixtures and the
//! governed-shape sensitivity receipt in a single pass, so no downstream figure
//! is ever assembled from two runs — the defect class D-479 names and D-483
//! abolishes.
//!
//! `#[ignore]` by default: it is a ~45-minute measurement, not a gate, and
//! `cargo test --workspace` must not pay for it. It is run by name, and the
//! pre-registration quotes that command with this file's governing revision.

mod common;

use std::path::PathBuf;
use std::time::Instant;

use pistol_core::{Coord, GameState, Turn};
use pistol_search::{
    CandidatePolicy, OrderingHeuristics, Provenance, QTriggers, SearchParams, Searcher,
    StageCounters, StagedParams, Stop,
};

/// The registered node budget, `Stop::Nodes`, per D-22 and D-478: instrument
/// mode refuses a wall-clock budget, and every cell here must be reproducible.
const NODES: u64 = 50_000;

/// `quiet_radius` is PINNED at the committed instrument document's value and is
/// not swept. The addendum allows one of the two axes to be pinned with a
/// stated reason, and the reason is attribution: the SPRT compares one changed
/// key against the committed engine, and a run that moved two keys could not
/// say which one a verdict belonged to (CLAUDE.md rule 6, one change one SPRT).
const QUIET_RADIUS: u32 = 2;

/// The calibration grid, registered before the run. It spans widely enough that
/// the benefit's DECAY is falsifiable: if every point qualifies under the
/// selection rule the rule selects the largest, and that outcome is a finding
/// about the channel rather than a calibration.
const GRID: [u64; 6] = [4, 8, 16, 32, 64, 128];

/// Openings the calibration reads: `0..999`, all of them already consumed by
/// earlier verdicts and therefore spent for verdict purposes, and DISJOINT from
/// the `1500..1999` slice this WP's own SPRT draws. K is chosen on a different
/// sample from the one the verdict is read on.
const CAL_SKIP: usize = 0;
const CAL_TAKE: usize = 1000;

/// The sensitivity receipt reads the same disjoint region, so it is a
/// PREDICTION about the governed sample and never a look at it.
const SENS_GAMES: usize = 25;
const SENS_TURN_CAP: usize = 40;

fn fixture(rel: &str) -> String {
    std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)).expect("reads")
}

fn entries(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#') && l.contains("start moves"))
        .map(str::to_string)
        .collect()
}

fn state_of(tail: &str) -> GameState {
    let body = tail.split_once(" #").map_or(tail, |(head, _)| head);
    let body = body.strip_prefix("position ").unwrap_or(body);
    let mut game = GameState::new_game();
    for at in body
        .strip_prefix("start moves ")
        .expect("a `start moves` tail")
        .split_whitespace()
        .flat_map(|t| t.split('/'))
        .map(|c| c.parse::<Coord>().expect("a coordinate"))
    {
        game.place(at).expect("a legal fixture ply");
    }
    game
}

fn searcher(safety_net_top_k: u64) -> Searcher {
    Searcher::new(
        SearchParams {
            tt_bytes: common::SMALL_TT,
            solver: None,
            candidate_policy: CandidatePolicy::Staged(StagedParams {
                quiet_radius: QUIET_RADIUS,
                safety_net_top_k,
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
        },
        Box::new(pistol_eval::HandcraftedV0::new(common::committed_weights())),
    )
    .expect("the staged parameters are accepted")
}

struct One {
    depth: u32,
    nodes: u64,
    ms: u128,
    best: Turn,
    mate: bool,
    stages: StageCounters,
}

fn run_one(engine: &mut Searcher, state: &GameState) -> One {
    let started = Instant::now();
    let outcome = engine
        .search(state, Stop::Nodes(NODES), &mut |_| {})
        .expect("the search runs");
    One {
        depth: outcome.info.depth_turns,
        nodes: outcome.info.nodes,
        ms: started.elapsed().as_millis(),
        best: outcome.best,
        // A search that PROVED a mate did not fail to reach depth: it stopped
        // because it was finished. The excluded set is fixed across seats below.
        // A score inside the mate band means the search PROVED a result rather
        // than running out of depth (`score.rs`'s MATE_THRESHOLD).
        mate: matches!(outcome.provenance, Provenance::SolverProof)
            || outcome.info.score.abs() >= pistol_search::MATE - 64,
        stages: outcome.info.stages,
    }
}

/// SECTION CAL — the calibration sweep. One line per (K, opening) is too much
/// output to read, so the per-opening detail is folded here and only the
/// per-seat aggregate is printed, plus the mate-terminated set by index so the
/// FIXED cross-seat population is checkable rather than asserted.
fn calibration(book: &[String]) -> Option<u64> {
    println!(
        "## SECTION CAL — calibration sweep, Stop::Nodes({NODES}), quiet_radius {QUIET_RADIUS}"
    );
    println!(
        "CAL/POPULATION openings={} skip={CAL_SKIP} take={CAL_TAKE}",
        book.len()
    );
    let seats: Vec<u64> = std::iter::once(0).chain(GRID).collect();
    let mut mate_any: Vec<usize> = Vec::new();
    let mut rows: Vec<(u64, Vec<u32>, u64, u64, u64, u128)> = Vec::new();
    for &k in &seats {
        let mut depths = Vec::with_capacity(book.len());
        let (mut capped, mut withheld_u, mut withheld_e) = (0u64, 0u64, 0u64);
        let mut ms = 0u128;
        for (i, line) in book.iter().enumerate() {
            let mut engine = searcher(k);
            let one = run_one(&mut engine, &state_of(line));
            if one.mate && !mate_any.contains(&i) {
                mate_any.push(i);
            }
            depths.push(one.depth);
            capped += one.stages.safety_net_capped_rows;
            withheld_u += one.stages.safety_net_upper_withheld;
            withheld_e += one.stages.safety_net_exact_withheld;
            ms += one.ms;
        }
        rows.push((k, depths, capped, withheld_u, withheld_e, ms));
    }
    mate_any.sort_unstable();
    println!(
        "CAL/EXCLUDED mate_terminated_on_any_seat={} indices={:?}",
        mate_any.len(),
        mate_any
    );
    let mut means: Vec<(u64, f64)> = Vec::new();
    for (k, depths, capped, wu, we, ms) in &rows {
        let kept: Vec<u32> = depths
            .iter()
            .enumerate()
            .filter(|(i, _)| !mate_any.contains(i))
            .map(|(_, d)| *d)
            .collect();
        let mean = kept.iter().map(|d| f64::from(*d)).sum::<f64>() / kept.len() as f64;
        means.push((*k, mean));
        let hist = (0..8)
            .map(|b| kept.iter().filter(|d| **d as usize == b).count())
            .collect::<Vec<_>>();
        println!(
            "CAL/SEAT K={k} mean_depth={mean:.4} population={} depth_hist={hist:?} \
             capped_rows={capped} upper_withheld={wu} exact_withheld={we} sum_ms={ms}",
            kept.len()
        );
    }
    let line = selection(&means);
    println!("{line}");
    line.split_once("CAL/SELECTED K=")
        .and_then(|(_, rest)| rest.split_whitespace().next())
        .and_then(|k| k.parse::<u64>().ok())
}

/// THE REGISTERED SELECTION RULE, APPLIED BY THE INSTRUMENT ITSELF so that the
/// choice is not a human step taken after the numbers are seen.
///
/// Channel: MEAN completed `depth_turns` over the fixed population, larger is
/// better. It carries no threshold, which matters because a threshold picked
/// from data nobody may cite would be the after-the-numbers choice a
/// pre-registration exists to forbid.
///
/// `gain(K) = mean(K) - mean(0)`. If the best gain is not positive, NO K is
/// selected and the work package closes as a measured finding. Otherwise K is
/// the LARGEST grid point whose gain is at least 75 % of the best gain — the
/// weakest prune that keeps three quarters of the measured benefit — ties to
/// the larger K.
fn selection(means: &[(u64, f64)]) -> String {
    let base = means
        .iter()
        .find(|(k, _)| *k == 0)
        .expect("the incumbent seat is always present")
        .1;
    let gains: Vec<(u64, f64)> = means
        .iter()
        .filter(|(k, _)| *k > 0)
        .map(|(k, m)| (*k, m - base))
        .collect();
    let best = gains.iter().map(|(_, g)| *g).fold(f64::MIN, f64::max);
    let detail: Vec<String> = gains.iter().map(|(k, g)| format!("K{k}:{g:+.4}")).collect();
    if best <= 0.0 {
        return format!(
            "CAL/SELECTED none rule=largest-K-within-75pc-of-best-gain \
             base_mean={base:.4} best_gain={best:+.4} gains=[{}] \
             VERDICT=no-seat-gains-over-the-incumbent-the-package-closes-as-a-measured-finding",
            detail.join(" ")
        );
    }
    let threshold = 0.75 * best;
    let chosen = gains
        .iter()
        .filter(|(_, g)| *g >= threshold)
        .map(|(k, _)| *k)
        .max()
        .expect("a positive best gain qualifies at least its own seat");
    format!(
        "CAL/SELECTED K={chosen} rule=largest-K-within-75pc-of-best-gain \
         base_mean={base:.4} best_gain={best:+.4} threshold={threshold:+.4} gains=[{}]",
        detail.join(" ")
    )
}

/// SECTION BENCH — both fixtures, every seat, five reps, per-position median.
fn bench(label: &str, rel: &str, reps: usize) {
    println!("## SECTION BENCH/{label} — Stop::Nodes({NODES}), {reps} reps, per-position median");
    let text = fixture(rel);
    let list = entries(&text);
    for k in std::iter::once(0).chain(GRID) {
        for (i, line) in list.iter().enumerate() {
            let state = state_of(line);
            let stones = state.board().stones().count();
            let mut times: Vec<u128> = Vec::with_capacity(reps);
            let mut depth = 0;
            let mut nodes = 0;
            let mut capped = 0;
            for _ in 0..reps {
                let mut engine = searcher(k);
                let one = run_one(&mut engine, &state);
                times.push(one.ms);
                depth = one.depth;
                nodes = one.nodes;
                capped = one.stages.safety_net_capped_rows;
            }
            times.sort_unstable();
            let median = times[reps / 2];
            let iqr = times[reps * 3 / 4] - times[reps / 4];
            println!(
                "BENCH/{label} K={k} p{i:02} stones={stones} depth={depth} nodes={nodes} \
                 median_ms={median} iqr_ms={iqr} capped_rows={capped} reps={times:?}"
            );
        }
    }
}

/// SECTION SENS — the book-class sensitivity receipt the honest-expectation
/// section owes BEFORE game one: on trajectories of the incumbent's own play
/// and of the capped engine's own play, how often does the class occur and how
/// often does the played turn change.
fn sensitivity(book: &[String], k: u64) {
    println!("## SECTION SENS — governed-shape divergence, K={k}, both trajectories");
    for driver_capped in [false, true] {
        let (mut searches, mut bearing, mut diverged, mut decided) = (0u64, 0u64, 0u64, 0usize);
        for line in book.iter().take(SENS_GAMES) {
            let mut state = state_of(line);
            let mut base_engine = searcher(0);
            let mut capped_engine = searcher(k);
            for _ in 0..SENS_TURN_CAP {
                if state.outcome().is_decided() {
                    decided += 1;
                    break;
                }
                let base = run_one(&mut base_engine, &state);
                let capped = run_one(&mut capped_engine, &state);
                searches += 1;
                if base.stages.batched_quiet_safety_net > 0 {
                    bearing += 1;
                }
                if base.best != capped.best {
                    diverged += 1;
                }
                let played = if driver_capped {
                    capped.best
                } else {
                    base.best
                };
                for at in [Some(played.first()), played.second()]
                    .into_iter()
                    .flatten()
                {
                    if state.place(at).is_err() {
                        break;
                    }
                }
            }
        }
        println!(
            "SENS/TRAJECTORY={} games={SENS_GAMES} turn_cap={SENS_TURN_CAP} K={k} \
             searches={searches} decided_early={decided} bearing={bearing} diverged={diverged}",
            if driver_capped { "capped" } else { "incumbent" }
        );
    }
}

#[test]
#[ignore = "the registered measurement run, ~45 minutes; run by name, not by the workspace suite"]
fn wp15d_b_the_one_measurement_run() {
    println!("# WP-1.5d (B) — THE ONE MEASUREMENT RUN (docs/decisions.md D-482).");
    println!(
        "# Budget Stop::Nodes({NODES}) throughout (D-22, D-478). quiet_radius {QUIET_RADIUS}, pinned."
    );
    println!(
        "# tt_bytes = SMALL_TT (1<<20), the test harness's value, stated rather than implied."
    );
    println!("# Grid {GRID:?}, incumbent seat K=0.");

    let book = fixture("../pistol-cli/tests/fixtures/random_openings_v1.txt");
    let all = entries(&book);
    let cal: Vec<String> = all.iter().skip(CAL_SKIP).take(CAL_TAKE).cloned().collect();
    assert_eq!(cal.len(), CAL_TAKE, "the calibration slice must be whole");

    let selected = calibration(&cal);
    bench(
        "CORPUS",
        "../pistol-cli/tests/fixtures/bench_positions_v1.txt",
        5,
    );
    bench("SPREAD", "../pistol-cli/tests/fixtures/spread_v1.txt", 5);
    // The sensitivity receipt is taken at the SELECTED K, inside this same run,
    // so the honest-expectation section quotes a receipt for the seat that will
    // actually play rather than for a grid.
    match selected {
        Some(k) => sensitivity(&cal, k),
        None => {
            println!("SENS/SKIPPED no seat was selected, so there is no arm to be sensitive about")
        }
    }
    println!("# END OF RUN");
}
