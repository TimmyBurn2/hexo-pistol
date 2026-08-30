//! What a book of N openings buys: the error rates a sequential test achieves
//! when the openings run out at a cap, measured rather than derived.
//!
//! The question a book's size answers is not "how many pairs does a run
//! expect" but "how many can it afford before the sample ends" — D-187
//! measured that for the v1 book and this is the same measurement, kept as a
//! shipped instrument instead of a scratch harness nobody can re-run.
//!
//! The model is an EXPONENTIAL TILT of a real pentanomial. The bucket shape
//! comes from a governed run's own `p0..p4`, so the pair-score variance is
//! play's rather than a coin's; the tilt moves the mean to a target normalized
//! Elo while keeping that shape. A tilt is used and not a re-weighting of two
//! extreme buckets because the latter changes the variance the LLR divides by,
//! which is the quantity the whole test is denominated in.

use std::process::ExitCode;

use pistol_arena::sprt::{Bounds, PAIR_SCORES, Sample, Unit, Verdict, crossing};

/// Deterministic and seeded: the same command answers the same numbers on any
/// machine (CLAUDE.md rule 4), which is what makes a registered figure from
/// this instrument re-checkable rather than merely re-runnable.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn unit(&mut self) -> f64 {
        // 53 bits, the mantissa: every draw is representable exactly.
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// The tilted bucket probabilities `q_i` proportional to `p_i * exp(theta*s_i)`.
fn tilt(base: [f64; 5], theta: f64) -> [f64; 5] {
    let mut out = [0.0; 5];
    let mut total = 0.0;
    for i in 0..5 {
        out[i] = base[i] * (theta * PAIR_SCORES[i]).exp();
        total += out[i];
    }
    for slot in &mut out {
        *slot /= total;
    }
    out
}

/// The standardized effect `t` a bucket distribution carries.
fn t_of(q: [f64; 5]) -> f64 {
    let mut mu = 0.0;
    let mut second = 0.0;
    for i in 0..5 {
        mu += q[i] * PAIR_SCORES[i];
        second += q[i] * PAIR_SCORES[i] * PAIR_SCORES[i];
    }
    let var = second - mu * mu;
    (mu - 0.5) / var.sqrt()
}

/// The tilt whose `t` is `target`, by bisection.
///
/// # Errors
/// When the bracket does not contain the target — a target this bucket shape
/// cannot reach is named, never silently clamped (CLAUDE.md rule 3).
fn solve_tilt(base: [f64; 5], target: f64) -> Result<f64, String> {
    let (mut lo, mut hi) = (-60.0f64, 60.0f64);
    if t_of(tilt(base, lo)) > target || t_of(tilt(base, hi)) < target {
        return Err(format!(
            "target t {target} is outside what this bucket shape reaches \
             ([{}, {}])",
            t_of(tilt(base, lo)),
            t_of(tilt(base, hi))
        ));
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if t_of(tilt(base, mid)) < target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Ok(0.5 * (lo + hi))
}

struct Args {
    pairs: u64,
    elo0: f64,
    elo1: f64,
    alpha: f64,
    beta: f64,
    truth: f64,
    runs: u64,
    seed: u64,
    buckets: [f64; 5],
}

fn parse(words: &[String]) -> Result<Args, String> {
    // WP-1.5d's own governed pentanomial (docs/decisions.md D-491) is the
    // default shape, because it is the run whose resolution this instrument
    // was first asked to size.
    let mut args = Args {
        pairs: 500,
        elo0: 0.0,
        elo1: 15.0,
        alpha: 0.05,
        beta: 0.05,
        truth: 0.0,
        runs: 40_000,
        seed: 1,
        buckets: [30.0, 75.0, 277.0, 68.0, 50.0],
    };
    let mut index = 0;
    while index < words.len() {
        let key = words[index].as_str();
        let value = words
            .get(index + 1)
            .ok_or_else(|| format!("{key} wants a value"))?;
        let number = || value.parse::<f64>().map_err(|why| format!("{key}: {why}"));
        let count = || value.parse::<u64>().map_err(|why| format!("{key}: {why}"));
        match key {
            "--pairs" => args.pairs = count()?,
            "--elo0" => args.elo0 = number()?,
            "--elo1" => args.elo1 = number()?,
            "--alpha" => args.alpha = number()?,
            "--beta" => args.beta = number()?,
            "--truth" => args.truth = number()?,
            "--runs" => args.runs = count()?,
            "--seed" => args.seed = count()?,
            "--buckets" => {
                let parts: Vec<&str> = value.split(',').collect();
                if parts.len() != 5 {
                    return Err("--buckets wants five comma-separated counts".to_string());
                }
                for (slot, part) in args.buckets.iter_mut().zip(parts) {
                    *slot = part.parse().map_err(|why| format!("--buckets: {why}"))?;
                }
            }
            other => return Err(format!("unknown option {other}")),
        }
        index += 2;
    }
    if args.pairs == 0 || args.runs == 0 {
        return Err("--pairs and --runs must be positive".to_string());
    }
    if args.buckets.iter().sum::<f64>() <= 0.0 || args.buckets.iter().any(|&c| c < 0.0) {
        return Err("--buckets must be non-negative and not all zero".to_string());
    }
    Ok(args)
}

fn run(args: &Args) -> Result<(), String> {
    let total: f64 = args.buckets.iter().sum();
    let base = [
        args.buckets[0] / total,
        args.buckets[1] / total,
        args.buckets[2] / total,
        args.buckets[3] / total,
        args.buckets[4] / total,
    ];
    let theta = solve_tilt(base, Unit::Pair.t(args.truth))?;
    let q = tilt(base, theta);
    let bounds = Bounds::of(args.alpha, args.beta);
    let mut rng = SplitMix64(args.seed);

    let (mut h1, mut h0, mut inconclusive, mut pairs_played) = (0u64, 0u64, 0u64, 0u64);
    for _ in 0..args.runs {
        let mut counts = [0u64; 5];
        let mut verdict = None;
        let mut played = 0u64;
        while played < args.pairs {
            let draw = rng.unit();
            let mut acc = 0.0;
            let mut bucket = 4;
            for (index, &weight) in q.iter().enumerate() {
                acc += weight;
                if draw < acc {
                    bucket = index;
                    break;
                }
            }
            counts[bucket] += 1;
            played += 1;
            let sample = Sample::of_pairs(counts);
            if let Some(seen) = crossing(&sample, Unit::Pair, args.elo0, args.elo1, bounds) {
                verdict = Some(seen);
                break;
            }
        }
        pairs_played += played;
        match verdict {
            Some(Verdict::H1) => h1 += 1,
            Some(Verdict::H0) => h0 += 1,
            _ => inconclusive += 1,
        }
    }

    let runs = args.runs as f64;
    println!(
        "sprt_power: pairs {} elo0 {} elo1 {} alpha {} beta {} truth {} runs {} seed {}",
        args.pairs, args.elo0, args.elo1, args.alpha, args.beta, args.truth, args.runs, args.seed
    );
    println!(
        "sprt_power: buckets {:?} theta {theta:.6} tilted_t {:.6} (target {:.6})",
        args.buckets,
        t_of(q),
        Unit::Pair.t(args.truth)
    );
    println!(
        "sprt_power: h1 {h1} ({:.4})  h0 {h0} ({:.4})  inconclusive {inconclusive} ({:.4})",
        h1 as f64 / runs,
        h0 as f64 / runs,
        inconclusive as f64 / runs
    );
    println!("sprt_power: mean_pairs {:.1}", pairs_played as f64 / runs);
    println!("SPRT_POWER_DONE");
    Ok(())
}

fn main() -> ExitCode {
    let words: Vec<String> = std::env::args().skip(1).collect();
    match parse(&words).and_then(|args| run(&args)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(why) => {
            eprintln!("sprt_power: FAIL: {why}");
            ExitCode::from(1)
        }
    }
}
