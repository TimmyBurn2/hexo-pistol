use std::sync::Mutex;

use pistol_core::{GameState, Outcome, Turn};

use crate::channel::Channel;
use crate::error::ArenaError;
use crate::exchange::{Answer, ask};
use crate::game::{Rules, seat_of};
use crate::record::Compute;
use crate::replay_report::{Answered, Divergence, GameReplay, Replayed};
use crate::seats::{self, Seat};
use crate::transcript::{RecordedGame, Transcript};

/// Replay every game of a report.
///
/// Returns the completed work even when the pass is abandoned, for the same
/// reason the generation path does (docs/decisions.md D-160) — and a caller must
/// treat a pass with any game missing as no answer at all, because a criterion
/// over SOME of a report's games is a criterion over a sample nobody registered.
pub fn run(transcript: &Transcript, workers: usize) -> (Result<(), ArenaError>, Replayed) {
    let started = std::time::Instant::now();
    let total = transcript.games.len();
    let slots: Mutex<Vec<Option<GameReplay>>> = Mutex::new((0..total).map(|_| None).collect());
    let next = Mutex::new(0usize);
    let failure: Mutex<Option<ArenaError>> = Mutex::new(None);
    let rules = Rules {
        go_line: &transcript.go_line,
        turn_cap: transcript.turn_cap,
        hang_timeout_ms: transcript.hang_timeout_ms,
    };

    std::thread::scope(|scope| {
        for _ in 0..workers.min(total.max(1)) {
            scope.spawn(|| {
                loop {
                    let index = {
                        let mut cursor = next.lock().expect("work cursor");
                        if *cursor >= total {
                            return;
                        }
                        let index = *cursor;
                        *cursor += 1;
                        index
                    };
                    if failure.lock().expect("failure slot").is_some() {
                        return;
                    }
                    // NO EARLY STOP. `schedule::run` stops at the first SPRT
                    // boundary crossing, which is right for a run and wrong
                    // here: a criterion is over the report's whole game set,
                    // and a replay that stopped early would answer about a
                    // prefix while reporting about the report.
                    match one_game(transcript, &rules, index) {
                        Ok(replayed) => {
                            slots.lock().expect("slots")[index] = Some(replayed);
                        }
                        Err(error) => {
                            let mut held = failure.lock().expect("failure slot");
                            if held.is_none() {
                                *held = Some(error);
                            }
                            return;
                        }
                    }
                }
            });
        }
    });

    let games = slots.into_inner().expect("slots");
    let wall_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    (
        match failure.into_inner().expect("failure slot") {
            Some(error) => Err(error),
            None => Ok(()),
        },
        Replayed {
            games,
            wall_ms,
            workers,
        },
    )
}

/// One game: both seats spawned by the shared sequence, then the walk.
fn one_game(
    transcript: &Transcript,
    rules: &Rules<'_>,
    index: usize,
) -> Result<GameReplay, ArenaError> {
    let game = &transcript.games[index];
    let seats = [
        Seat {
            section: &transcript.engines[0],
            identity: &transcript.identities[0],
        },
        Seat {
            section: &transcript.engines[1],
            identity: &transcript.identities[1],
        },
    ];
    let labels = [
        transcript.engines[0].label.as_str(),
        transcript.engines[1].label.as_str(),
    ];
    seats::with_seats(&seats, transcript.hang_timeout_ms, |channels| {
        walk(game, transcript.opening_turns, labels, channels, rules)
    })
}

/// Feed the recorded moves, asking the seat whose turn it is at each of its own
/// turns, and stop at the first disagreement.
fn walk(
    game: &RecordedGame,
    opening_turns: u32,
    labels: [&str; 2],
    channels: &mut [Channel; 2],
    rules: &Rules<'_>,
) -> Result<GameReplay, ArenaError> {
    let mut state = GameState::new_game();
    let mut fed: Vec<Turn> = Vec::with_capacity(game.moves.len());
    let mut compute = [Compute::default(); 2];
    let mut compared = 0usize;

    let finish = |replayed: usize,
                  compared: usize,
                  compute: [Compute; 2],
                  divergence: Option<Divergence>| GameReplay {
        index: game.index,
        recorded_turns: game.moves.len(),
        replayed_turns: replayed,
        compared_turns: compared,
        nodes: [compute[0].nodes, compute[1].nodes],
        divergence,
    };

    for (at, recorded) in game.moves.iter().enumerate() {
        if at >= opening_turns as usize {
            let mover_is_p1 = state.to_move() == pistol_core::Player::P1;
            let engine = seat_of(mover_is_p1, game.a_is_p1);
            let answer = ask(
                &mut channels[engine],
                &fed,
                rules,
                game.opening,
                state.turn(),
                &mut compute[engine],
            )?;
            compared += 1;
            let disagreed = match answer {
                Answer::Move(turn) if turn == *recorded => None,
                Answer::Move(turn) => Some(Answered::Move(turn)),
                Answer::Forfeit { reason, line } => Some(Answered::Forfeit { reason, line }),
            };
            if let Some(answered) = disagreed {
                return Ok(finish(
                    at,
                    compared,
                    compute,
                    Some(Divergence {
                        at_turn: at + 1,
                        mover_label: labels[engine].to_string(),
                        mover_engine: engine,
                        recorded: *recorded,
                        answered,
                    }),
                ));
            }
        }
        // pistol-core is the only judge of legality (rule 2). `transcript::read`
        // already replayed this whole move list and refused an illegal one
        // before any engine was spawned, so this arm is unreachable on a
        // document that got this far — it is kept as a refusal rather than an
        // `expect` because a state machine that can only be wrong by panicking
        // is one nobody can diagnose.
        let outcome = state.make_turn(*recorded).map_err(|error| {
            ArenaError::config(
                "replay report",
                format!(
                    "game {}: recorded turn {} (`{recorded}`) is not legal in the position the \
                     moves before it reach: {error}",
                    game.index,
                    at + 1
                ),
            )
        })?;
        fed.push(*recorded);
        if matches!(outcome, Outcome::Win { .. }) && at + 1 != game.moves.len() {
            return Err(ArenaError::config(
                "replay report",
                format!(
                    "game {}: recorded turn {} wins, and the report records {} further turns after \
                     a game that was already over",
                    game.index,
                    at + 1,
                    game.moves.len() - at - 1
                ),
            ));
        }
    }
    Ok(finish(game.moves.len(), compared, compute, None))
}

/// Refuse a replay whose engines are no longer the ones the report was written
/// by, before any game is replayed.
///
/// The generation path captures each engine's identity once, before its first
/// game, and re-verifies every later spawn against that capture
/// (docs/decisions.md D-199). A replay has the ORIGINAL capture already — the
/// report carries it, `engine_id` line by `engine_id` line — so this is the
/// strictly stronger check: not "the engine has not changed since this pass
/// started" but "the engine is the one the report attests". `identity::capture`
/// is the generation path's own function and digests the binary before it
/// spawns anything, so a decoy at the recorded path is refused here too
/// (docs/decisions.md D-252).
pub fn verify_engines(transcript: &Transcript) -> Result<(), ArenaError> {
    for slot in 0..2 {
        let section = &transcript.engines[slot];
        let found = crate::identity::capture(section, transcript.hang_timeout_ms)?;
        let expected = &transcript.identities[slot];
        if &found != expected {
            return Err(ArenaError::IdentityDrift {
                engine: section.label.clone(),
                what: String::from("the identity the source report attests"),
                expected: format!(
                    "binary {} config {} weights {} and {} id line(s)",
                    expected.binary_sha256,
                    expected.config_sha256,
                    expected.weights_sha256,
                    expected.id_lines.len()
                ),
                found: format!(
                    "binary {} config {} weights {} and {} id line(s)",
                    found.binary_sha256,
                    found.config_sha256,
                    found.weights_sha256,
                    found.id_lines.len()
                ),
            });
        }
    }
    Ok(())
}
