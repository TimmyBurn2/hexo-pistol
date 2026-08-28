use std::sync::Mutex;

use crate::config::ArenaConfig;
use crate::error::ArenaError;
use crate::game::{self, Rules};
use crate::identity::EngineIdentity;
use crate::openings::Openings;
use crate::record::GameRecord;
use crate::score;
use crate::seats::{self, Seat};

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
    identities: &[EngineIdentity; 2],
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
                    match one_game(config, openings, identities, &rules, index) {
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

/// One game: two fresh engines set up by [`seats::with_seats`] — the same
/// sequence the replay mode runs — and the referee between them.
fn one_game(
    config: &ArenaConfig,
    openings: &Openings,
    identities: &[EngineIdentity; 2],
    rules: &Rules<'_>,
    index: usize,
) -> Result<GameRecord, ArenaError> {
    let opening = &openings.taken[index / 2];
    // Even index: engine A takes the first seat. Odd: engine B does. So the
    // report's order is opening index, then side assignment, by construction.
    let a_is_p1 = index.is_multiple_of(2);

    let seats = [
        Seat {
            section: &config.engine_a,
            identity: &identities[0],
        },
        Seat {
            section: &config.engine_b,
            identity: &identities[1],
        },
    ];
    seats::with_seats(&seats, config.run.hang_timeout_ms, |channels| {
        game::play(opening, a_is_p1, index, channels, rules)
    })
}
