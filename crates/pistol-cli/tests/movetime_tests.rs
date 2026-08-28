mod common;

use std::str::FromStr;
use std::time::Instant;

use common::{PLAY, committed, engine, repo};
use pistol_cli::sha256::sha256_hex;
use pistol_core::Turn;
use pistol_engine::{Budget, Engine, Pistol, PositionSpec, SearchInfo, SearchOutcome};

/// The reproducer fixture, and the pin that catches an edited copy (D-37).
const SPREAD_FIXTURE: &str = "crates/pistol-cli/tests/fixtures/spread_v1.txt";
const SPREAD_SHA256: &str = "c3bd8cc6f1fe876c781ebfc4f8b090b279a40f78d9b180df1c459b24c74ac97d";

/// What the wall-clock bound relaxes to per search in a debug build.
const DEBUG_SLACK_MS: u64 = 1500;

/// One reproducer position: the stone count it advertises, and the
/// `position` verb's argument.
struct Spread {
    stones: u32,
    spec: PositionSpec,
}

/// The fixture, loaded strictly: unknown directives panic by line, the pin is
/// checked before anything is parsed, and every stated stone count is verified
/// against the replayed position rather than trusted.
fn spread_positions() -> Vec<Spread> {
    let path = repo(SPREAD_FIXTURE);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    assert_eq!(
        sha256_hex(&bytes),
        SPREAD_SHA256,
        "{SPREAD_FIXTURE} does not match its pin; regenerating it is an ADR-level change"
    );
    let text = String::from_utf8(bytes).expect("the fixture is text");

    let mut positions = Vec::new();
    let mut stated: Option<u32> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(count) = line.strip_prefix("stones ") {
            assert!(stated.is_none(), "two `stones` lines without a position");
            stated = Some(count.parse().expect("a stone count"));
        } else if let Some(rest) = line.strip_prefix("position ") {
            let stones = stated.take().expect("a `stones` line before each position");
            let spec = PositionSpec::from_str(rest)
                .unwrap_or_else(|error| panic!("fixture position does not parse: {error:?}"));
            let state = spec
                .replay()
                .unwrap_or_else(|error| panic!("fixture position does not replay: {error}"));
            let on_board = state.board().stones().count();
            assert_eq!(
                on_board as u32, stones,
                "the fixture says {stones} stones and the replay has {on_board}"
            );
            positions.push(Spread { stones, spec });
        } else {
            panic!("unknown fixture directive: {line}");
        }
    }
    assert_eq!(positions.len(), 4, "the reproducer class is four positions");
    positions
}

/// Epsilon as the committed play config states it — the tests read the promise,
/// they do not define it.
fn epsilon_ms() -> u64 {
    committed(PLAY).play.movetime_epsilon_ms
}

/// The per-search wall-clock bound this build profile is held to.
fn ceiling_ms(movetime: u64) -> u64 {
    if cfg!(debug_assertions) {
        movetime + DEBUG_SLACK_MS
    } else {
        movetime + epsilon_ms()
    }
}

/// Stand the engine on `spread` and search under `movetime`; measure, and check
/// the answer is a turn pistol-core accepts from that exact position.
fn timed_search(
    engine: &mut Pistol,
    spread: &Spread,
    movetime: u64,
    report: &mut dyn FnMut(&SearchInfo),
) -> (u64, SearchOutcome) {
    engine.new_game();
    engine
        .set_position(&spread.spec)
        .unwrap_or_else(|error| panic!("the fixture position was refused: {error}"));
    let started = Instant::now();
    let outcome = engine
        .go_reporting(Budget::MovetimeMs(movetime), report)
        .unwrap_or_else(|error| {
            panic!(
                "movetime {movetime} on {} stones was refused: {error}",
                spread.stones
            )
        });
    let elapsed = u64::try_from(started.elapsed().as_millis()).expect("fits");

    let mut replay = spread
        .spec
        .replay()
        .expect("the position replayed once already");
    replay.make_turn(outcome.best).unwrap_or_else(|error| {
        panic!(
            "movetime {movetime} on {} stones answered the illegal turn {}: {error}",
            spread.stones, outcome.best
        )
    });
    (elapsed, outcome)
}

/// R1: the D-95 reproducer class, both required budgets, N + epsilon asserted.
#[test]
fn movetime_is_a_ceiling_on_spread_positions() {
    let mut engine = engine(PLAY);
    for spread in &spread_positions() {
        for movetime in [500u64, 50] {
            let (elapsed, outcome) = timed_search(&mut engine, spread, movetime, &mut |_| {});
            let bound = ceiling_ms(movetime);
            assert!(
                elapsed <= bound,
                "movetime {movetime} on {} spread stones returned after {elapsed} ms \
                 (bound {bound} ms, epsilon {} ms) — D-95's floor is back",
                spread.stones,
                epsilon_ms()
            );
            assert_eq!(
                outcome.best,
                *outcome.info.pv.first().expect("the pv is never empty"),
                "the answer and the evidence disagree"
            );
        }
    }
}

/// R2: the engine can answer at ANY budget. One millisecond on the worst
/// measured instance — 99 spread stones, where the pre-WP first iteration cost
/// 58.6 s (D-95) — still yields a legal move inside the promise.
#[test]
fn engine_answers_legal_move_at_one_millisecond() {
    let mut engine = engine(PLAY);
    let positions = spread_positions();
    let worst = positions
        .iter()
        .max_by_key(|spread| spread.stones)
        .expect("the fixture is not empty");
    assert_eq!(worst.stones, 99, "the worst instance is D-95's own point");

    let (elapsed, outcome) = timed_search(&mut engine, worst, 1, &mut |_| {});
    let bound = ceiling_ms(1);
    assert!(
        elapsed <= bound,
        "movetime 1 on 99 spread stones returned after {elapsed} ms (bound {bound} ms)"
    );
    assert!(
        !outcome.info.pv.is_empty(),
        "an answer under any budget carries its move as evidence"
    );
}

/// R4: what the totals report says under an abort is what actually happened —
/// `depth_turns` equals the number of per-depth reports (a depth is completed
/// exactly when it was reported), whatever instant the deadline landed at, and
/// the totals never bill fewer nodes than the last completed depth reported.
#[test]
fn partial_depth_reported_honestly() {
    let mut engine = engine(PLAY);
    let positions = spread_positions();

    // A sweep of budgets so the interrupt lands in different places: inside the
    // fallback window, inside the first iteration, and past completed depths.
    for spread in &positions {
        for movetime in [1u64, 5, 25, 200] {
            let mut reported: Vec<SearchInfo> = Vec::new();
            let (_, outcome) = timed_search(&mut engine, spread, movetime, &mut |info| {
                reported.push(info.clone())
            });

            assert_eq!(
                outcome.info.depth_turns,
                u32::try_from(reported.len()).expect("fits"),
                "movetime {movetime} on {} stones: the totals claim depth {} but {} depths \
                 were completed and reported",
                spread.stones,
                outcome.info.depth_turns,
                reported.len()
            );
            if let Some(last) = reported.last() {
                assert!(
                    outcome.info.nodes >= last.nodes,
                    "the totals bill less work than the last completed depth"
                );
                assert_eq!(
                    last.depth_turns, outcome.info.depth_turns,
                    "the last report and the totals disagree about the completed depth"
                );
            }
            assert!(
                !outcome.info.pv.is_empty(),
                "every answer carries at least the turn it plays"
            );
            assert_eq!(
                outcome.best, outcome.info.pv[0],
                "best is always the first move of the reported line"
            );
        }
    }
}

/// The protocol spelling survives the trip: a `go movetime` through the line
/// protocol answers `bestmove` with a parseable turn token even when nothing
/// but the fallback was reachable.
#[test]
fn movetime_answers_a_turn_token_through_the_protocol() {
    let mut engine = engine(PLAY);
    let positions = spread_positions();
    let worst = &positions[3];
    engine.new_game();
    engine
        .set_position(&worst.spec)
        .expect("the fixture replays");

    let outcome = engine
        .go(Budget::MovetimeMs(1))
        .expect("movetime 1 answers");
    let token = outcome.best.to_string();
    let parsed = Turn::from_str(&token)
        .unwrap_or_else(|error| panic!("`{token}` is not a turn token: {error:?}"));
    assert_eq!(parsed, outcome.best, "the token round-trips");
}

/// The deadline mechanism a step past the fixture envelope: 299 spread
/// stones, three times D-95's measured worst point, where the pre-WP first
/// iteration cost MINUTES. The N + epsilon promise (D-207) is stated for the
/// <= 99-stone domain; what this asserts is that at 299 stones the mechanism
/// still answers inside the same bound — the fallback (~20 ms here) plus the
/// per-node interrupt tail — so a regression of D-95's class (an
/// uninterruptible iteration creeping back) fails violently rather than only
/// inside the fixture.
///
/// Stated honestly after a mutation round: this test does NOT see the
/// intra-ordering (64-cell) check — at this scale the per-node check alone
/// keeps the bound, and the band where the ordering check binds (~1000
/// stones) has margins under timing noise (docs/decisions.md D-213). What it
/// pins is the ceiling beyond the envelope, not that one check.
#[test]
fn deadline_holds_on_a_spread_past_the_fixture_envelope() {
    // 299 stones: turn 1 at the origin, then 149 pairs marching along one
    // axis at hex-distance 8 — the D-95 spread geometry, twice the fixture's
    // worst point.
    let mut moves = String::from("0,0");
    let mut q = 8i32;
    for _ in 0..149 {
        moves.push_str(&format!(" {q},0/{},0", q + 8));
        q += 16;
    }
    let spec = PositionSpec::from_str(&format!("start moves {moves}"))
        .expect("the constructed spread parses");
    let spread = Spread { stones: 299, spec };

    let mut engine = engine(PLAY);
    let (elapsed, outcome) = timed_search(&mut engine, &spread, 40, &mut |_| {});
    let bound = ceiling_ms(40);
    assert!(
        elapsed <= bound,
        "movetime 40 on 299 spread stones returned after {elapsed} ms (bound {bound} ms): \
         a D-95-class uninterruptible stretch is back"
    );
    assert!(
        !outcome.info.pv.is_empty(),
        "the interrupted search still answers with its evidence"
    );
}
