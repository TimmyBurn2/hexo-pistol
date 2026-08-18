//! The human sentence, on stdout.
//!
//! Everything here is also in the report file; this exists so that an operator
//! reading a terminal does not have to parse one. The one thing it adds is
//! emphasis: a run with a forfeit in it says so in a way that cannot be skimmed
//! past, because such a run is not a measurement (docs/decisions.md D-158).

use std::fmt::Write as _;

use crate::config::ArenaConfig;
use crate::dedupe;
use crate::record::{Compute, GameRecord};
use crate::score::{self};
use crate::sprt::{Bounds, Unit, Verdict};

/// The summary for a finished run.
pub fn render(config: &ArenaConfig, records: &[GameRecord], wall_ms: u64) -> String {
    let mut out = String::new();
    let counted = score::tally(records);
    let duplicates = dedupe::duplicates(records);
    let sprt = &config.sprt;
    let pair = score::pair_sample(records);
    let game = score::game_sample(records);
    let bounds = Bounds::of(sprt.alpha, sprt.beta);

    let verdict = score::verdict(records, sprt);

    let _ = writeln!(
        out,
        "arena: {} vs {}, budget {} {}, {} openings",
        config.engine_a.label,
        config.engine_b.label,
        config.budget.report_tokens().0,
        config.budget.report_tokens().1,
        config.run.openings_take
    );
    let _ = writeln!(
        out,
        "  n {}  distinct-n {}  ({} duplicate games)",
        counted.n,
        dedupe::distinct_count(&duplicates),
        counted.n - dedupe::distinct_count(&duplicates)
    );
    let _ = writeln!(
        out,
        "  {} W / {} L / {} capped for {}  (capped fraction {:.3})",
        counted.wins_a,
        counted.losses_a,
        counted.capped,
        config.engine_a.label,
        counted.capped_fraction()
    );
    let p = counted.pentanomial;
    let _ = writeln!(
        out,
        "  pair outcomes  p0 {} p1 {} p2 {} p3 {} p4 {}  ({} pairs)",
        p[0],
        p[1],
        p[2],
        p[3],
        p[4],
        records.len() / 2
    );
    let _ = writeln!(
        out,
        "  first player won {} of {} decided games",
        counted.first_player_wins, counted.decided
    );
    let _ = writeln!(
        out,
        "  LLR pair  {}   <- the verdict's unit",
        show(pair.llr(Unit::Pair, sprt.elo0, sprt.elo1))
    );
    let _ = writeln!(
        out,
        "  LLR game  {}   <- diagnostic only (docs/decisions.md D-154)",
        show(game.llr(Unit::Game, sprt.elo0, sprt.elo1))
    );
    let _ = writeln!(
        out,
        // Every float here is width-bounded. `Display` on an f64 spells a
        // subnormal out in full, which is several hundred characters on one
        // terminal line (docs/decisions.md D-173).
        "  bounds    h0 {:.4}  h1 {:.4}   (elo0 {:.2} elo1 {:.2} alpha {:.3e} beta {:.3e}, \
         normalized Elo)",
        bounds.h0, bounds.h1, sprt.elo0, sprt.elo1, sprt.alpha, sprt.beta
    );
    if let Some(nelo) = pair.normalized_elo(Unit::Pair) {
        let _ = writeln!(out, "  normalized Elo estimate {nelo:.2}");
    }

    let mut totals = [Compute::default(); 2];
    for record in records {
        totals[0].absorb(record.compute[0]);
        totals[1].absorb(record.compute[1]);
    }
    for (label, total) in [
        (&config.engine_a.label, totals[0]),
        (&config.engine_b.label, totals[1]),
    ] {
        let _ = writeln!(
            out,
            "  compute {label}: {} nodes, {} ms, {} searches, deepest {} turns",
            total.nodes, total.time_ms, total.searches, total.max_depth
        );
    }
    let _ = writeln!(
        out,
        "  wall {wall_ms} ms at {} workers",
        config.run.n_workers
    );
    let _ = writeln!(out, "  VERDICT {}", verdict.token());
    if verdict == Verdict::InvalidForfeit {
        let _ = writeln!(
            out,
            "\n  *** {} game(s) were forfeited. One of these two configurations is broken,\n  \
             *** so this run is NOT a strength claim, whatever the LLR says.",
            counted.forfeits
        );
    }
    if verdict == Verdict::InconclusiveDegenerate {
        let _ = writeln!(
            out,
            "  (every pair scored the same, so no likelihood ratio is defined — which is the\n   \
             expected answer when two identical configurations play each other)"
        );
    }
    out
}

/// An LLR, or the reason there is not one.
fn show(value: Option<f64>) -> String {
    match value {
        Some(value) => format!("{value:.6}"),
        None => String::from("none (degenerate sample)"),
    }
}
