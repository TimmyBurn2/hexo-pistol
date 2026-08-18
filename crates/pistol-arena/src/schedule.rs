//! Playing the games, and deciding when to stop.
//!
//! # The invariance this module owes the report
//!
//! Every game is a pure function of its opening, its seating and the two engine
//! configurations: fresh subprocesses per game, no shared state, no shared
//! table, no shared clock. Results land in a pre-sized slot vector indexed by
//! game index and are never appended in completion order. So the SET of results
//! is worker-invariant, and everything the verdict block holds is a function of
//! that set (docs/decisions.md D-161).
//!
//! Fresh subprocesses per game rather than `newgame` on a reused pair: reuse
//! would be faster and D-7's gate already certifies that `newgame` clears
//! everything, but under N workers the assignment of games to processes changes
//! with N, so any residue would make the report depend on the worker count —
//! the one thing this module must guarantee it does not (docs/decisions.md
//! D-164).
//!
//! # Stopping
//!
//! The stop is evaluated ONLY at pair boundaries. An odd prefix would split a
//! pair — including one seat's game and not the other's — and the truncation
//! point is correlated with the outcome of the last included game, since a
//! crossing is likeliest right after a decisive one. That is optional-stopping
//! bias applied to the one game whose seat is unmatched (docs/decisions.md
//! D-165).
//!
//! `k` is the SMALLEST pair count whose sample crosses a boundary, recomputed
//! over the whole finished prefix rather than the first crossing a worker
//! happened to notice — which is what makes it a function of the results alone.
//! Games in flight past `k` are killed and discarded; they never contributed to
//! the report, so discarding them cannot bias it.

use std::sync::Mutex;

use crate::channel::Channel;
use crate::config::ArenaConfig;
use crate::error::ArenaError;
use crate::game::{self, Rules};
use crate::handshake;
use crate::openings::Openings;
use crate::record::GameRecord;
use crate::score;

/// What a run produced.
pub struct Played {
    /// The games of the verdict, in index order.
    pub records: Vec<GameRecord>,
    /// How many started games were abandoned when the stop fired. Schedule
    /// dependent, so it belongs to the timing block and to nothing else.
    pub discarded: usize,
}

/// Play the run.
///
/// Returns the completed prefix even when a run is abandoned, so that a hang on
/// the last game of a long run does not discard everything before it
/// (docs/decisions.md D-160).
pub fn run(
    config: &ArenaConfig,
    openings: &Openings,
    go_line: &str,
) -> (Result<(), ArenaError>, Played) {
    let total = openings.taken.len() * 2;
    let slots: Mutex<Vec<Option<GameRecord>>> = Mutex::new((0..total).map(|_| None).collect());
    let next = Mutex::new(0usize);
    let stop: Mutex<Option<usize>> = Mutex::new(None);
    let failure: Mutex<Option<ArenaError>> = Mutex::new(None);
    let rules = Rules {
        go_line,
        turn_cap: config.run.turn_cap,
        hang_timeout_ms: config.run.hang_timeout_ms,
    };

    std::thread::scope(|scope| {
        for _ in 0..config.run.n_workers.min(total.max(1)) {
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
                    if let Some(limit) = *stop.lock().expect("stop slot")
                        && index >= limit
                    {
                        return;
                    }
                    match one_game(config, openings, &rules, index) {
                        Ok(record) => {
                            slots.lock().expect("slots")[index] = Some(record);
                        }
                        Err(error) => {
                            let mut held = failure.lock().expect("failure slot");
                            if held.is_none() {
                                *held = Some(error);
                            }
                            return;
                        }
                    }
                    let finished = contiguous(&slots.lock().expect("slots"));
                    if let Some(pairs) = score::first_crossing_pairs(&finished, &config.sprt) {
                        let mut held = stop.lock().expect("stop slot");
                        let limit = pairs * 2;
                        if held.is_none_or(|current| limit < current) {
                            *held = Some(limit);
                        }
                    }
                }
            });
        }
    });

    let all = slots.into_inner().expect("slots");
    let limit = stop.into_inner().expect("stop slot");
    let finished: Vec<GameRecord> = contiguous(&all);
    let kept = match limit {
        Some(limit) => finished
            .into_iter()
            .take(limit)
            .collect::<Vec<GameRecord>>(),
        None => finished,
    };
    let started = all.iter().filter(|slot| slot.is_some()).count();
    let played = Played {
        discarded: started.saturating_sub(kept.len()),
        records: kept,
    };
    (
        match failure.into_inner().expect("failure slot") {
            Some(error) => Err(error),
            None => Ok(()),
        },
        played,
    )
}

/// The longest run of finished games from index zero.
fn contiguous(slots: &[Option<GameRecord>]) -> Vec<GameRecord> {
    slots
        .iter()
        .take_while(|slot| slot.is_some())
        .map(|slot| slot.clone().expect("checked"))
        .collect()
}

/// One game: two fresh engines, a handshake each, and the referee.
fn one_game(
    config: &ArenaConfig,
    openings: &Openings,
    rules: &Rules<'_>,
    index: usize,
) -> Result<GameRecord, ArenaError> {
    let opening = &openings.taken[index / 2];
    // Even index: engine A takes the first seat. Odd: engine B does. So the
    // report's order is opening index, then side assignment, by construction.
    let a_is_p1 = index.is_multiple_of(2);

    let mut channels = [
        Channel::start(
            &config.engine_a.label,
            &config.engine_a.binary,
            &config.engine_a.config,
        )?,
        Channel::start(
            &config.engine_b.label,
            &config.engine_b.binary,
            &config.engine_b.config,
        )?,
    ];
    for channel in &mut channels {
        handshake::shake(channel, config.run.hang_timeout_ms)?;
        if channel.send(pistol_cli::protocol::NEW_GAME).is_err() {
            return Err(ArenaError::Handshake {
                engine: channel.label().to_string(),
                why: String::from("it closed its input before the game started"),
            });
        }
    }

    let record = game::play(opening, a_is_p1, index, &mut channels, rules)?;
    for channel in &mut channels {
        channel.shutdown();
    }
    Ok(record)
}
