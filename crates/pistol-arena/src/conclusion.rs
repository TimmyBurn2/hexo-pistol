use std::fmt::Write as _;

use crate::dedupe;
use crate::record::End;
use crate::report::{Written, float, maybe};
use crate::score::{self, Tally};
use crate::sprt::{NELO_TO_T, Sample, Unit, Verdict};

/// One record per game, plus its moves and any refusal it drew.
pub fn games(out: &mut String, written: &Written<'_>) {
    let duplicates = dedupe::duplicates(written.records);
    let trajectory = score::trajectory(written.records, &written.config.sprt);
    let (label_a, label_b) = (
        written.config.engine_a.label.as_str(),
        written.config.engine_b.label.as_str(),
    );
    for (position, record) in written.records.iter().enumerate() {
        let (p1, p2) = if record.a_is_p1 {
            (label_a, label_b)
        } else {
            (label_b, label_a)
        };
        let (end, reason) = match record.end {
            End::Normal => ("normal", "none"),
            End::Forfeit(why) => ("forfeit", why.token()),
        };
        let by = match record.forfeit_by {
            Some(0) => label_a,
            Some(_) => label_b,
            None => "none",
        };
        let dup = match duplicates[position] {
            Some(first) => first.to_string(),
            None => String::from("none"),
        };
        let (llr_game, llr_pair) = trajectory[position];
        let _ = writeln!(
            out,
            "game {} opening {} p1 {p1} p2 {p2} result {} end {end} forfeit_by {by} reason \
             {reason} turns {} dup_of {dup} nodes_a {} nodes_b {} depth_a {} depth_b {} \
             llr_game {} llr_pair {}",
            record.index,
            record.opening,
            record.result.token(),
            record.turns(),
            record.compute[0].nodes,
            record.compute[1].nodes,
            record.compute[0].max_depth,
            record.compute[1].max_depth,
            maybe(llr_game),
            maybe(llr_pair),
        );
        let mut moves = format!("moves {}", record.index);
        for turn in &record.moves {
            let _ = write!(moves, " {turn}");
        }
        let _ = writeln!(out, "{moves}");
        if let Some(refusal) = &record.refusal {
            // Last field of its own record kind, so free text needs no quoting.
            let _ = writeln!(out, "refusal {} {refusal}", record.index);
        }
    }
}

/// The counts, the pairs and the verdict.
pub fn found(out: &mut String, written: &Written<'_>) {
    let records = written.records;
    let sprt = &written.config.sprt;
    let counted: Tally = score::tally(records);
    let duplicates = dedupe::duplicates(records);
    for (index, bucket) in score::pair_buckets(records).into_iter().enumerate() {
        let _ = writeln!(
            out,
            "pair {index} opening {} bucket p{bucket} score_a {}",
            records[index * 2].opening,
            float(bucket as f64 / 4.0)
        );
    }
    let _ = writeln!(
        out,
        "counts n {} distinct_n {} wins_a {} capped {} losses_a {} forfeits {} decided {}",
        counted.n,
        dedupe::distinct_count(&duplicates),
        counted.wins_a,
        counted.capped,
        counted.losses_a,
        counted.forfeits,
        counted.decided
    );
    let p = counted.pentanomial;
    let _ = writeln!(
        out,
        "pentanomial p0 {} p1 {} p2 {} p3 {} p4 {}",
        p[0], p[1], p[2], p[3], p[4]
    );
    let _ = writeln!(out, "capped_fraction {}", float(counted.capped_fraction()));
    // Over decided NON-FORFEIT games: a capped game has no first-player winner,
    // and a win by forfeit measures a protocol bug in the loser, not the game
    // (docs/decisions.md D-201). The forfeit count sits adjacent because the
    // rate is only unconditional when that count is zero; a nonzero count puts
    // `conditional` in the token itself, where no reader can skim past it.
    // (`decided_non_forfeit` is deliberately a different word from the `counts`
    // line's forfeit-inclusive `decided`.)
    let conditional = if counted.forfeits > 0 {
        " conditional"
    } else {
        ""
    };
    let _ = writeln!(
        out,
        "first_player_wins {} of {} decided_non_forfeit forfeits {}{conditional}",
        counted.first_player_wins, counted.decided_clean, counted.forfeits
    );
    let game = score::game_sample(records);
    let pair = score::pair_sample(records);
    let _ = writeln!(
        out,
        "llr_game last {}",
        maybe(game.llr(Unit::Game, sprt.elo0, sprt.elo1))
    );
    let _ = writeln!(
        out,
        "llr_pair last {}",
        maybe(pair.llr(Unit::Pair, sprt.elo0, sprt.elo1))
    );
    let _ = writeln!(
        out,
        "nelo_pair {} ci95 {}",
        maybe(pair.normalized_elo(Unit::Pair)),
        maybe(confidence(&pair))
    );
    let decided = score::verdict(records, sprt);
    let _ = writeln!(out, "verdict {}", decided.token());
    let _ = writeln!(out, "verdict_unit {}", Unit::Pair.token());
    // A forfeited run still says what it WOULD have concluded, because hiding
    // it would be a silent skip. It drops whole PAIRS and not individual games:
    // the pentanomial is built from consecutive pairs, so filtering one game
    // out of a flat list shifts every later game by one and every "pair" after
    // the first forfeit would straddle two different openings — a number nobody
    // computed, printed on the line that exists to keep the run honest
    // (docs/decisions.md D-158). How many pairs went is reported, because a
    // conclusion drawn from a smaller sample than the run played is not the
    // same conclusion.
    let (clean, dropped) = if decided == Verdict::InvalidForfeit {
        let kept = score::pairs_without_forfeits(records);
        (
            score::verdict(&kept, sprt).token(),
            records.len() / 2 - kept.len() / 2,
        )
    } else {
        ("none", 0)
    };
    let _ = writeln!(out, "verdict_if_clean {clean} pairs_dropped {dropped}");
}

/// The half-width of a 95% interval on the pair-level normalized Elo.
fn confidence(sample: &Sample) -> Option<f64> {
    if sample.is_degenerate() {
        return None;
    }
    let n = sample.n as f64;
    Some(1.96 / (NELO_TO_T * std::f64::consts::SQRT_2 * n.sqrt()))
}
