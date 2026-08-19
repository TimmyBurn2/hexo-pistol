//! Turning games into the numbers a verdict is read off.
//!
//! # The observation unit
//!
//! The PAIR decides. Under determinism at a fixed budget an opening yields
//! exactly one pair outcome, so the pair — not the game — is the independent
//! observation, and a game-level test treats correlated samples as independent.
//! The game-level LLR is still computed and reported, as a diagnostic beside
//! the verdict rather than as the verdict (docs/decisions.md D-154).
//!
//! The magnitude of the difference is derived rather than asserted, because an
//! earlier draft asserted it and had the DIRECTION backwards. Writing the LLR as
//! evidence minus drift, the sample mean is identical at both units, so the two
//! differ only through `sigma` — and `var_pair <= var_game` always. The upper
//! bound on the game-level statistic is therefore attained when no pair is 1-1,
//! and once 1-1 pairs are common enough the game-level test is CONSERVATIVE
//! rather than anti-conservative. Measured on a constructed sample of 200 pairs
//! of which 150 are 1-1: `llr_game` 1.1625 against `llr_pair` 1.8535. Two
//! deterministic engines produce exactly that regime, which is why the direction
//! an operator would have guessed is the wrong one.

use crate::config::SprtSection;
use crate::record::{GameRecord, GameResult};
use crate::sprt::{Bounds, Sample, Unit, Verdict, crossing};

/// The counts a report carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Tally {
    /// Games in the verdict.
    pub n: usize,
    /// Games engine A won.
    pub wins_a: u64,
    /// Games that reached the horizon.
    pub capped: u64,
    /// Games engine A lost.
    pub losses_a: u64,
    /// Pair outcomes by score, ascending: index `i` is a pair scoring `i/4`.
    pub pentanomial: [u64; 5],
    /// Games that ended in a forfeit.
    pub forfeits: usize,
    /// Games the rules decided, which is every game that was not capped.
    /// Forfeits are decided — the offender lost — so they are in here.
    pub decided: usize,
    /// Decided games that were NOT forfeits: the first-player rate's
    /// denominator (docs/decisions.md D-201).
    pub decided_clean: usize,
    /// How many decided NON-FORFEIT games the first seat won.
    ///
    /// A win by forfeit measures a protocol bug in the loser, not the game, so
    /// counting it here made the rate readable only beside a zero `forfeits`
    /// count — wp13_results §6b's debt, paid by excluding forfeits from the
    /// numerator AND the denominator and printing the forfeit count adjacent.
    pub first_player_wins: usize,
}

impl Tally {
    /// The fraction of games that reached the horizon.
    ///
    /// Reported rather than thresholded on. It is not inert: a mass point at a
    /// half shrinks the variance the normalized Elo is divided by, so for a
    /// FIXED decisive record adding capped games accelerates the LLR (measured:
    /// a 100-80 record scores 0.2198 with no capped games and 1.1109 with
    /// 16000). An operator reads this number and judges the run
    /// (docs/decisions.md D-157).
    pub fn capped_fraction(&self) -> f64 {
        if self.n == 0 {
            return 0.0;
        }
        self.capped as f64 / self.n as f64
    }
}

/// Count a run.
pub fn tally(records: &[GameRecord]) -> Tally {
    let mut out = Tally {
        n: records.len(),
        ..Tally::default()
    };
    for record in records {
        match record.result {
            GameResult::Capped => out.capped += 1,
            _ => {
                out.decided += 1;
                if !record.is_forfeit() {
                    out.decided_clean += 1;
                    if record.result == GameResult::P1Win {
                        out.first_player_wins += 1;
                    }
                }
                if record.score_a() > 0.5 {
                    out.wins_a += 1;
                } else {
                    out.losses_a += 1;
                }
            }
        }
        if record.is_forfeit() {
            out.forfeits += 1;
        }
    }
    for bucket in pair_buckets(records) {
        out.pentanomial[bucket] += 1;
    }
    out
}

/// The pentanomial bucket of each COMPLETE pair, in pair order.
///
/// A pair is complete when both its games are present; an odd trailing game
/// contributes nothing, which is why the stop only ever fires at a pair
/// boundary (docs/decisions.md D-165).
pub fn pair_buckets(records: &[GameRecord]) -> Vec<usize> {
    records
        .chunks_exact(2)
        .map(|pair| {
            // Two games, each scoring 0, 1/2 or 1 for engine A, so the sum is a
            // multiple of a half in `0..=2`, and twice the sum indexes the five
            // buckets exactly.
            let sum = pair[0].score_a() + pair[1].score_a();
            (sum * 2.0).round() as usize
        })
        .collect()
}

/// The pair-level sample: the verdict's own.
pub fn pair_sample(records: &[GameRecord]) -> Sample {
    let mut buckets = [0u64; 5];
    for bucket in pair_buckets(records) {
        buckets[bucket] += 1;
    }
    Sample::of_pairs(buckets)
}

/// The game-level sample: the diagnostic.
pub fn game_sample(records: &[GameRecord]) -> Sample {
    let counted = tally(records);
    Sample::of_games(counted.wins_a, counted.capped, counted.losses_a)
}

/// The smallest number of complete pairs whose sample crosses a boundary.
///
/// Recomputed over the whole prefix rather than reported as the first crossing
/// noticed, which is what makes it a function of the results and not of the
/// order they finished in.
pub fn first_crossing_pairs(records: &[GameRecord], sprt: &SprtSection) -> Option<usize> {
    let bounds = Bounds::of(sprt.alpha, sprt.beta);
    // Accumulated rather than re-derived per prefix. The scan runs after every
    // completed game, so re-sampling each prefix from scratch made the stop
    // cost the pair count SQUARED per game and the run's length cubed overall —
    // some 8e9 record visits at the 1591-opening book this arena exists to play
    // (docs/decisions.md D-165). The verdict is unchanged: these are the same
    // counts in the same order.
    let mut buckets = [0u64; 5];
    for (index, bucket) in pair_buckets(records).into_iter().enumerate() {
        buckets[bucket] += 1;
        let sample = Sample::of_pairs(buckets);
        if crossing(&sample, Unit::Pair, sprt.elo0, sprt.elo1, bounds).is_some() {
            return Some(index + 1);
        }
    }
    None
}

/// The games of every pair that contains no forfeit, in order.
///
/// Whole PAIRS, and that is the whole point. The pentanomial is built from
/// consecutive pairs, so filtering individual games out of a flat list shifts
/// every later game by one: after the first forfeit each "pair" would straddle
/// two different openings, and `chunks_exact` would silently drop the trailing
/// game. That would be a sample nobody computed, feeding the one line whose job
/// is to say what a forfeited run would have concluded (docs/decisions.md
/// D-158).
pub fn pairs_without_forfeits(records: &[GameRecord]) -> Vec<GameRecord> {
    records
        .chunks_exact(2)
        .filter(|pair| !pair[0].is_forfeit() && !pair[1].is_forfeit())
        .flatten()
        .cloned()
        .collect()
}

/// The run's verdict.
///
/// The order of these tests is the ruling. A forfeit outranks everything: it
/// means one of the two configurations is broken, and rule 6's judge may not
/// turn a correctness bug into an Elo number — so the token itself says so, not
/// merely a count on a neighbouring line and not merely an exit code that is
/// gone by the time the report is read (docs/decisions.md D-158).
pub fn verdict(records: &[GameRecord], sprt: &SprtSection) -> Verdict {
    if records.iter().any(GameRecord::is_forfeit) {
        return Verdict::InvalidForfeit;
    }
    let sample = pair_sample(records);
    if sample.is_degenerate() {
        return Verdict::InconclusiveDegenerate;
    }
    let bounds = Bounds::of(sprt.alpha, sprt.beta);
    // No crossing and a well-formed sample means the openings ran out. There is
    // no second inconclusive token here on purpose: the only way to reach this
    // function is a completed run, and an `exhausted` flag whose two branches
    // returned the same value was a parameter no test could ever have caught
    // going wrong.
    crossing(&sample, Unit::Pair, sprt.elo0, sprt.elo1, bounds)
        .unwrap_or(Verdict::InconclusiveAtGameCap)
}

/// The LLR after each game, in index order, at both units.
///
/// `None` where the prefix has no defined statistic — a pair-level value does
/// not exist until a pair is complete, and neither exists while every
/// observation has scored the same.
pub fn trajectory(records: &[GameRecord], sprt: &SprtSection) -> Vec<(Option<f64>, Option<f64>)> {
    (1..=records.len())
        .map(|prefix| {
            let slice = &records[..prefix];
            (
                game_sample(slice).llr(Unit::Game, sprt.elo0, sprt.elo1),
                pair_sample(slice).llr(Unit::Pair, sprt.elo0, sprt.elo1),
            )
        })
        .collect()
}
