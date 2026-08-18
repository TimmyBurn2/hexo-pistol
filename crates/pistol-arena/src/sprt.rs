//! The sequential test, and the only arithmetic in this crate that decides
//! anything.
//!
//! # The estimator
//!
//! One closed form, called at two observation units. For `n` observations with
//! sample mean `mu` and sample variance `var`, and hypotheses expressed as
//! `t_k` (the standardized effect, see below):
//!
//! ```text
//! LLR = n * ((t1 - t0) * (mu - 1/2)/sqrt(var) - (t1^2 - t0^2)/2)
//! ```
//!
//! It is the Gaussian log-likelihood ratio with the hypotheses written as
//! `m_k = 1/2 + t_k * sigma`, which is what normalized Elo means: normalized
//! Elo equals logistic Elo when `sigma = 1/2`, and the logistic relation near
//! equality is `mu - 1/2 ~= Elo * ln(10)/1600`, so `t = (mu - 1/2)/sigma =
//! Elo * ln(10)/800`. That is [`NELO_TO_T`].
//!
//! **Why not the exact multinomial GSPRT.** This work package is required to
//! pin the math with a table of HAND-COMPUTED values. Nobody hand-computes the
//! constrained maximum-likelihood tilt the exact GSPRT needs, and a formula
//! whose reference table can only be produced by the implementation under test
//! pins nothing (docs/decisions.md D-155).
//!
//! # The two units
//!
//! A game scores `{0, 1/2, 1}`. A pair — the same opening played from both
//! seats — scores `{0, 1/4, 1/2, 3/4, 1}`. Normalized Elo is defined on the
//! PER-GAME scale, so a pair's hypotheses must be written in per-game sigma:
//! `sigma_per_game = sqrt(2 * var_pair)`. Substituting `m_k = 1/2 + t_k *
//! sqrt(2 * var_pair)` into the Gaussian LLR reproduces the expression above
//! exactly, with `t_k` scaled by `sqrt(2)`. So the two units share one function
//! and answer the SAME hypothesis; without the scaling they would not
//! (docs/decisions.md D-154).
//!
//! # What this deliberately does not do
//!
//! There is no regularization of zero counts. A sample with zero variance has
//! no defined LLR here and yields [`Verdict::InconclusiveDegenerate`] rather
//! than a number. The alternative — Fishtest's pseudo-count — was measured to
//! DECIDE the verdict at the sizes this crate's own smoke gate runs at, which
//! makes it a tunable, and a tunable lives in a schema (docs/decisions.md
//! D-156).
//!
//! # Floating point
//!
//! Every operation here is `+ - * /` or `sqrt`, all correctly rounded, in a
//! FIXED expression order — not "exact", which they are not. No `mul_add` and
//! no reordered summation, so two machines agree bit for bit. [`NELO_TO_T`]
//! divides by a compile-time constant; the only libm calls in a whole verdict
//! are the two [`Bounds`], which is why a verdict could in principle differ
//! across libm implementations only when an LLR lands on a bound to the last
//! bit. Said rather than claimed away.
//!
//! The Wald bounds are approximate, as they are in Fishtest: `sigma` is
//! re-estimated at every prefix, so this is a profile likelihood ratio and not
//! a sum of independent increments.

/// Normalized Elo to the standardized effect `t`, per game.
///
/// `ln(10)/800`. Written as a division by a compile-time constant so that the
/// value is the same on every machine.
pub const NELO_TO_T: f64 = std::f64::consts::LN_10 / 800.0;

/// The per-game score of a win, a capped game and a loss, in this order.
///
/// The order is load-bearing: [`Sample::of_games`] sums in it, and a fixed
/// summation order is part of what makes the arithmetic reproducible.
pub const GAME_SCORES: [f64; 3] = [0.0, 0.5, 1.0];

/// The score of each pentanomial bucket, ascending.
///
/// Indexed by how many half-points the pair scored, so bucket 2 is a 1-1 pair
/// — which is NOT the same as two capped games, and is why the report spells
/// these `p0..p4` rather than borrowing chess's `dd`.
pub const PAIR_SCORES: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];

/// Which observation the test is over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    /// One game. The diagnostic unit (docs/decisions.md D-154).
    Game,
    /// One opening played from both seats. The verdict unit.
    Pair,
}

impl Unit {
    /// The hypothesis `t_k` for a normalized-Elo bound in this unit.
    ///
    /// The `sqrt(2)` is the module doc's derivation and not a fudge factor:
    /// it re-expresses a per-game normalized Elo in per-pair sigma units.
    pub fn t(self, normalized_elo: f64) -> f64 {
        match self {
            Unit::Game => normalized_elo * NELO_TO_T,
            Unit::Pair => normalized_elo * NELO_TO_T * std::f64::consts::SQRT_2,
        }
    }

    /// The report's word for this unit.
    pub const fn token(self) -> &'static str {
        match self {
            Unit::Game => "game",
            Unit::Pair => "pair",
        }
    }
}

/// The first two moments of a scored sample.
///
/// Held rather than recomputed because both the LLR and the reported normalized
/// Elo need them, and computing them twice invites two summation orders.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sample {
    /// How many observations. The true count; nothing is ever added to it.
    pub n: u64,
    /// Sample mean of the score.
    pub mu: f64,
    /// Sample variance, in the raw-moment form `E[s^2] - mu^2` — stated because
    /// the reference table pins this arithmetic and not an algebraically equal
    /// rearrangement of it.
    pub var: f64,
}

impl Sample {
    /// A sample from counts and the score of each count's bucket.
    ///
    /// `counts` and `scores` are index-aligned and summed in index order.
    fn of(counts: &[u64], scores: &[f64]) -> Sample {
        debug_assert_eq!(counts.len(), scores.len(), "index-aligned buckets");
        let n: u64 = counts.iter().sum();
        if n == 0 {
            return Sample {
                n: 0,
                mu: 0.0,
                var: 0.0,
            };
        }
        let total = n as f64;
        let mut first = 0.0;
        let mut second = 0.0;
        for (&count, &score) in counts.iter().zip(scores) {
            let weight = count as f64;
            first += weight * score;
            second += weight * score * score;
        }
        let mu = first / total;
        Sample {
            n,
            mu,
            var: second / total - mu * mu,
        }
    }

    /// Wins, capped games and losses, from the first engine's point of view.
    pub fn of_games(wins: u64, capped: u64, losses: u64) -> Sample {
        Sample::of(&[losses, capped, wins], &GAME_SCORES)
    }

    /// The five pentanomial buckets, ascending by pair score.
    pub fn of_pairs(buckets: [u64; 5]) -> Sample {
        Sample::of(&buckets, &PAIR_SCORES)
    }

    /// Whether this sample can carry a likelihood ratio at all.
    ///
    /// False when every observation scored the same. That is not a failure of
    /// the sample — an engine that wins every game produces it — but the
    /// statistic is undefined there and this crate refuses to invent one
    /// (docs/decisions.md D-156).
    pub fn is_degenerate(&self) -> bool {
        self.n == 0 || self.var <= 0.0
    }

    /// The standardized deviation from an even score, `(mu - 1/2)/sigma`.
    pub fn t_hat(&self) -> Option<f64> {
        if self.is_degenerate() {
            return None;
        }
        Some((self.mu - 0.5) / self.var.sqrt())
    }

    /// The point estimate, in the normalized Elo the bounds are stated in.
    pub fn normalized_elo(&self, unit: Unit) -> Option<f64> {
        let scale = match unit {
            Unit::Game => 1.0,
            Unit::Pair => std::f64::consts::SQRT_2,
        };
        self.t_hat().map(|t| t / (NELO_TO_T * scale))
    }

    /// The log-likelihood ratio against two normalized-Elo hypotheses.
    ///
    /// `None` when the sample is degenerate — never a substituted value.
    pub fn llr(&self, unit: Unit, elo0: f64, elo1: f64) -> Option<f64> {
        let t_hat = self.t_hat()?;
        let (t0, t1) = (unit.t(elo0), unit.t(elo1));
        Some((self.n as f64) * ((t1 - t0) * t_hat - (t1 * t1 - t0 * t0) / 2.0))
    }
}

/// Wald's stopping boundaries.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    /// At or below this, accept H0.
    pub h0: f64,
    /// At or above this, accept H1.
    pub h1: f64,
}

impl Bounds {
    /// The boundaries for these error rates.
    ///
    /// The only two calls to a transcendental in a whole verdict; see the
    /// module doc on what that does and does not cost.
    pub fn of(alpha: f64, beta: f64) -> Bounds {
        Bounds {
            h0: (beta / (1.0 - alpha)).ln(),
            h1: ((1.0 - beta) / alpha).ln(),
        }
    }
}

/// What a finished run concluded.
///
/// A closed set, because a verdict is read by scripts and every token here is
/// part of that contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// The LLR crossed the lower boundary.
    H0,
    /// The LLR crossed the upper boundary.
    H1,
    /// Neither boundary was crossed before the openings ran out.
    InconclusiveAtGameCap,
    /// Every observation scored the same, so no LLR is defined
    /// (docs/decisions.md D-156).
    InconclusiveDegenerate,
    /// At least one game was forfeited, so one of the two configurations is
    /// broken and this run is not a measurement (docs/decisions.md D-158).
    InvalidForfeit,
}

impl Verdict {
    /// The report's word for this verdict.
    pub const fn token(self) -> &'static str {
        match self {
            Verdict::H0 => "h0",
            Verdict::H1 => "h1",
            Verdict::InconclusiveAtGameCap => "inconclusive_at_game_cap",
            Verdict::InconclusiveDegenerate => "inconclusive_degenerate",
            Verdict::InvalidForfeit => "invalid_forfeit",
        }
    }

    /// Whether this verdict is a measurement a strength claim may cite.
    pub const fn is_conclusive(self) -> bool {
        matches!(self, Verdict::H0 | Verdict::H1)
    }
}

/// Whether this sample has already decided, and which way.
///
/// `None` means keep playing: either the LLR is undefined or it sits between
/// the boundaries.
pub fn crossing(
    sample: &Sample,
    unit: Unit,
    elo0: f64,
    elo1: f64,
    bounds: Bounds,
) -> Option<Verdict> {
    let llr = sample.llr(unit, elo0, elo1)?;
    if llr >= bounds.h1 {
        return Some(Verdict::H1);
    }
    if llr <= bounds.h0 {
        return Some(Verdict::H0);
    }
    None
}
