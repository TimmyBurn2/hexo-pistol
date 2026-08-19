//! The stats block: what the corpus turned out to contain.
//!
//! Printed, never committed — it is a measurement of an artifact that is itself
//! never committed (CLAUDE.md rule 8), and its headline numbers land in
//! docs/decisions.md by hand.
//!
//! # Conventions, pinned
//!
//! Every quantile is nearest-rank on the ascending sort: the p90 of `n` values
//! is the element at index `ceil(0.9n) - 1`. Every rate is printed as the
//! integer ratio it was counted as, never as a formatted float, so nothing here
//! depends on float formatting or rounding (CLAUDE.md rule 4).
//!
//! # The placement-distance histogram
//!
//! Reported here and computed in [`super::distance`], where the argument for it
//! is written out: the replay's zero legality violations are consistent with a
//! radius-6 platform and a radius-8 one alike, and only an observed placement
//! beyond 6 tells them apart (docs/decisions.md D-101, D-149).
//!
//! # What the first-player rate does and does not say
//!
//! The research report flags 3-axis fairness as unmeasured, and this is the
//! first look at real data — but a rated human corpus cannot answer the
//! game-theoretic question. Seat assignment is independent of strength, so the
//! aggregate first-player rate is about fifty per cent by construction of the
//! rating pool, whatever the game's intrinsic tilt. The split at the rating
//! median inherits the same confound, which is why the evenly-matched subset is
//! reported beside it: among games whose ratings are close, the rating is not
//! doing the predicting. Section F's question stays open for engine self-play
//! (docs/decisions.md D-149).

use std::fmt;

use pistol_core::Player;

use super::distance::PlacementDistances;
use super::openings::ELO_GAP_CEILING;
use super::record::Record;
use super::verdict::Replayed;

/// Ratings this close count as evenly matched.
pub const EVEN_MATCH_GAP: u16 = 25;

/// A count out of a total, printed as the ratio it was counted as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ratio {
    /// How many.
    pub part: usize,
    /// Out of how many.
    pub whole: usize,
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.whole == 0 {
            return write!(f, "0/0");
        }
        // Four decimal places from integer arithmetic: no float anywhere on the
        // path from the count to the character.
        let scaled = (self.part * 10_000 + self.whole / 2) / self.whole;
        write!(
            f,
            "{}/{} ({}.{:04})",
            self.part,
            self.whole,
            scaled / 10_000,
            scaled % 10_000
        )
    }
}

/// The ascending-sort quantile by nearest rank.
fn quantile(sorted: &[usize], numerator: usize, denominator: usize) -> usize {
    if sorted.is_empty() {
        return 0;
    }
    let rank = (sorted.len() * numerator).div_ceil(denominator).max(1);
    sorted[rank - 1]
}

/// min / median / p90 / max of an ascending-sorted list.
struct Spread {
    min: usize,
    median: usize,
    p90: usize,
    max: usize,
}

impl Spread {
    fn of(values: &mut [usize]) -> Spread {
        values.sort_unstable();
        Spread {
            min: values.first().copied().unwrap_or(0),
            median: quantile(values, 1, 2),
            p90: quantile(values, 9, 10),
            max: values.last().copied().unwrap_or(0),
        }
    }
}

impl fmt::Display for Spread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "min {} median {} p90 {} max {}",
            self.min, self.median, self.p90, self.max
        )
    }
}

/// Everything the run measured.
pub struct Stats {
    /// The corpus's digest, so a pasted block says which document it is about.
    pub corpus_sha256: String,
    /// Lines read.
    pub games_read: usize,
    /// Per-verdict counts, in a fixed order.
    pub by_verdict: Vec<(&'static str, usize)>,
    /// Turns whose recorded first stone was outside the legal region.
    pub order_rescued: usize,
    /// Turns whose recorded first stone already won.
    pub stone_after_win: usize,
    /// Every stone after a game's first, by its distance to the nearest stone
    /// already placed — the sufficient side of the legal-radius question that
    /// the replay's zero violations could only answer one way round.
    pub placements: PlacementDistances,
    /// Game lengths in turns, over eligible games.
    pub turns: Vec<usize>,
    /// The lower of each eligible game's two ratings.
    pub min_elos: Vec<usize>,
    /// Eligible games, kept for the rates.
    pub eligible: Vec<(u16, u16, Player)>,
}

impl Stats {
    /// Gather what a run measured. `replayed` is every line, in corpus order.
    pub fn gather(corpus_sha256: String, records: &[Record], replays: &[Replayed]) -> Stats {
        let names = [
            "eligible",
            "illegal-turn",
            "post-win-continuation",
            "winner-mismatch",
            "not-decided",
        ];
        let mut by_verdict: Vec<(&'static str, usize)> =
            names.iter().map(|name| (*name, 0)).collect();
        let mut order_rescued = 0;
        let mut stone_after_win = 0;
        let mut turns = Vec::new();
        let mut min_elos = Vec::new();
        let mut eligible = Vec::new();

        for (record, replay) in records.iter().zip(replays) {
            order_rescued += replay.order_rescued;
            stone_after_win += replay.stone_after_win;
            let name = replay.verdict.name();
            for slot in by_verdict.iter_mut() {
                if slot.0 == name {
                    slot.1 += 1;
                }
            }
            if !replay.verdict.is_eligible() {
                continue;
            }
            turns.push(replay.turns.len());
            if let [Some(a), Some(b)] = record.elo {
                min_elos.push(usize::from(a.min(b)));
                eligible.push((a, b, record.winner));
            }
        }
        Stats {
            corpus_sha256,
            games_read: records.len(),
            by_verdict,
            order_rescued,
            stone_after_win,
            placements: PlacementDistances::of(records),
            turns,
            min_elos,
            eligible,
        }
    }

    /// How many games were excluded.
    pub fn excluded(&self) -> usize {
        self.by_verdict
            .iter()
            .filter(|(name, _)| *name != "eligible")
            .map(|(_, count)| count)
            .sum()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "corpus_sha256 {}", self.corpus_sha256)?;
        writeln!(f, "games read     {}", self.games_read)?;
        writeln!(f)?;
        let excluded = self.excluded();
        if excluded == 0 {
            writeln!(
                f,
                "REPLAY FAILURES: none. All {} games replay through pistol-core under the full \
                 rule set,",
                self.games_read
            )?;
            writeln!(
                f,
                "  radius-{} legal region included — real platform evidence for D-101.",
                pistol_core::LEGAL_RADIUS
            )?;
        } else {
            writeln!(f, "REPLAY FAILURES: {excluded} — ESCALATE. Per class:")?;
        }
        for (name, count) in &self.by_verdict {
            writeln!(f, "  {name:<24} {count}")?;
        }
        writeln!(f)?;
        writeln!(
            f,
            "recorded-order cross-check (excludes nothing; counts turns from EVERY game read,"
        )?;
        writeln!(f, "  including games later excluded):")?;
        writeln!(f, "  order-rescued turns      {}", self.order_rescued)?;
        writeln!(f, "  stone-after-win turns    {}", self.stone_after_win)?;
        writeln!(f)?;
        writeln!(
            f,
            "PLACEMENT DISTANCE (every stone after a game's first, to the NEAREST stone already"
        )?;
        writeln!(
            f,
            "  placed; counted over every game read. A stone beyond distance 6 is a placement the"
        )?;
        writeln!(
            f,
            "  platform accepted and a radius-6 rule forbids — see the module note and SB-65):"
        )?;
        writeln!(f, "{}", self.placements)?;
        writeln!(f)?;
        let mut turns = self.turns.clone();
        writeln!(f, "game length, turns:  {}", Spread::of(&mut turns))?;
        let mut elos = self.min_elos.clone();
        let elo_spread = Spread::of(&mut elos);
        writeln!(f, "min-elo:             {elo_spread}")?;
        writeln!(f)?;

        let median = elo_spread.median as u16;
        let rate = |games: &[(u16, u16, Player)]| Ratio {
            part: games
                .iter()
                .filter(|(_, _, winner)| *winner == Player::P1)
                .count(),
            whole: games.len(),
        };
        writeln!(
            f,
            "FIRST-PLAYER WIN RATE (human population; see the module note)"
        )?;
        writeln!(f, "  overall                  {}", rate(&self.eligible))?;
        let (high, low): (Vec<_>, Vec<_>) = self
            .eligible
            .iter()
            .copied()
            .partition(|&(a, b, _)| a.min(b) >= median);
        writeln!(f, "  min-elo >= {median:<14} {}", rate(&high))?;
        writeln!(f, "  min-elo <  {median:<14} {}", rate(&low))?;
        let even: Vec<_> = self
            .eligible
            .iter()
            .copied()
            .filter(|(a, b, _)| a.abs_diff(*b) < EVEN_MATCH_GAP)
            .collect();
        writeln!(
            f,
            "  evenly matched (gap < {EVEN_MATCH_GAP})  {}",
            rate(&even)
        )?;
        let mismatched: Vec<_> = self
            .eligible
            .iter()
            .copied()
            .filter(|(a, b, _)| a.abs_diff(*b) > ELO_GAP_CEILING)
            .collect();
        let favourite = Ratio {
            part: mismatched
                .iter()
                .filter(|(a, b, winner)| (*winner == Player::P1) == (a > b))
                .count(),
            whole: mismatched.len(),
        };
        writeln!(
            f,
            "  higher-rated scores, gap > {ELO_GAP_CEILING}: {favourite}  <- the confound the split inherits"
        )
    }
}
