use pistol_arena::sprt::{Bounds, NELO_TO_T, Sample, Unit, Verdict, crossing};

/// How close a hand-computed value has to be.
const TOLERANCE: f64 = 1e-12;

fn close(left: f64, right: f64, what: &str) {
    assert!(
        (left - right).abs() <= TOLERANCE,
        "{what}: computed {left:.15}, hand-computed {right:.15}, difference {:.3e}",
        (left - right).abs()
    );
}

#[test]
fn sprt_llr_matches_reference_table() {
    // elo0 = 0, elo1 = 4 normalized. t1 = 4 * ln(10)/800 = 0.011512925464970229,
    // and (t1^2 - t0^2)/2 = 6.627372638097999e-5.
    let (elo0, elo1) = (0.0, 4.0);

    // Row 1 — H1 side. n = 8, w = 5, capped = 2, l = 1.
    //   mu     = (5*1 + 2*0.5 + 1*0)/8 = 6/8   = 0.75
    //   E[s^2] = (5*1 + 2*0.25 + 0)/8  = 5.5/8 = 0.6875
    //   var    = 0.6875 - 0.5625       = 0.125
    //   t_hat  = 0.25/sqrt(0.125)      = 0.7071067811865475
    //   LLR    = 8 * (0.011512925464970229 * 0.7071067811865475 - 6.627372638097999e-5)
    let row1 = Sample::of_games(5, 2, 1);
    close(row1.mu, 0.75, "row 1 mu");
    close(row1.var, 0.125, "row 1 var");
    close(
        row1.t_hat().expect("row 1 is not degenerate"),
        0.7071067811865475,
        "row 1 t_hat",
    );
    close(
        row1.llr(Unit::Game, elo0, elo1).expect("row 1 has an LLR"),
        0.06459675152955803,
        "row 1 LLR",
    );

    // Row 2 — H0 side, the mirror of row 1. Same var, t_hat negated.
    let row2 = Sample::of_games(1, 2, 5);
    close(row2.mu, 0.25, "row 2 mu");
    close(
        row2.t_hat().expect("not degenerate"),
        -0.7071067811865475,
        "row 2 t_hat",
    );
    close(
        row2.llr(Unit::Game, elo0, elo1).expect("row 2 has an LLR"),
        -0.06565713115165371,
        "row 2 LLR",
    );

    // Row 3 — dead even. t_hat = 0, so the LLR is the drift term alone:
    //   LLR = 4 * -(t1^2)/2 = -0.00026509490552391996
    let row3 = Sample::of_games(1, 2, 1);
    close(row3.t_hat().expect("not degenerate"), 0.0, "row 3 t_hat");
    close(
        row3.llr(Unit::Game, elo0, elo1).expect("row 3 has an LLR"),
        -0.00026509490552391996,
        "row 3 LLR",
    );

    // Row 4 — a sample large enough to cross. n = 4096, w = 2560, capped = 512,
    // l = 1024.  mu = 2816/4096 = 0.6875, E[s^2] = 2688/4096 = 0.65625,
    // var = 0.65625 - 0.47265625 = 0.18359375.
    let row4 = Sample::of_games(2560, 512, 1024);
    close(row4.mu, 0.6875, "row 4 mu");
    close(row4.var, 0.18359375, "row 4 var");
    close(
        row4.llr(Unit::Game, elo0, elo1).expect("row 4 has an LLR"),
        20.36418395672719,
        "row 4 LLR",
    );

    // Row 5 — row 1's counts against a SHIFTED pair of hypotheses, elo0 = 1 and
    // elo1 = 5, so the drift term moves and the evidence term does not.
    close(
        row1.llr(Unit::Game, 1.0, 5.0).expect("row 5 has an LLR"),
        0.06433165662403412,
        "row 5 LLR",
    );

    // Row 6 — the PAIR unit, which is the verdict's. Buckets ascending by pair
    // score: 1 at 0, 1 at 1/4, 2 at 1/2, 2 at 3/4, 2 at 1.
    //   mu     = (0 + 0.25 + 1.0 + 1.5 + 2.0)/8 = 4.75/8   = 0.59375
    //   E[s^2] = (0 + 0.0625 + 0.5 + 1.125 + 2)/8 = 3.6875/8 = 0.4609375
    //   var    = 0.4609375 - 0.3525390625        = 0.1083984375
    // and t_k carries the extra sqrt(2) that puts a per-game normalized Elo
    // into per-pair sigma units.
    let row6 = Sample::of_pairs([1, 1, 2, 2, 2]);
    close(row6.mu, 0.59375, "row 6 mu");
    close(row6.var, 0.1083984375, "row 6 var");
    close(
        row6.llr(Unit::Pair, elo0, elo1).expect("row 6 has an LLR"),
        0.03602907464530836,
        "row 6 LLR",
    );
}

#[test]
fn the_wald_bounds_are_the_stated_logs() {
    // ln(0.95/0.05) = ln 19 = 2.9444389791664403, and the lower bound is its
    // negation only because alpha equals beta here.
    let bounds = Bounds::of(0.05, 0.05);
    close(bounds.h1, 2.9444389791664403, "upper bound");
    close(bounds.h0, -2.9444389791664403, "lower bound");

    // Asymmetric rates must NOT be symmetric bounds, or the test above would
    // pass against an implementation that negated one number.
    let skewed = Bounds::of(0.01, 0.2);
    close(
        skewed.h1,
        (0.8f64 / 0.01).ln(),
        "upper bound at alpha 0.01 beta 0.2",
    );
    close(
        skewed.h0,
        (0.2f64 / 0.99).ln(),
        "lower bound at alpha 0.01 beta 0.2",
    );
    assert!(
        (skewed.h1 + skewed.h0).abs() > 1.0,
        "asymmetric error rates give asymmetric bounds"
    );
}

#[test]
fn the_pair_unit_answers_the_same_hypothesis_as_the_game_unit() {
    // The sqrt(2) in `Unit::Pair` is what makes this true, and it is the whole
    // reason the two numbers may be printed beside each other: without it the
    // pair-level LLR answers a different hypothesis from the game-level one.
    //
    // The fixture is ASYMMETRIC on purpose. An earlier version used a sample
    // with mu exactly 1/2, where t_hat is zero and the evidence term
    // `(t1 - t0) * t_hat` vanishes on both sides — so only the drift term was
    // ever compared, and the property the scaling exists to give went untested.
    // Here mu is 0.6 on both sides, so both terms carry weight.
    //
    // 200 games at 120-80 and 100 pairs at [16, 0, 48, 0, 36] are the same
    // record: 120 wins and 80 losses, distributed over pairs as 36 sweeps,
    // 48 splits and 16 reverse sweeps. Independent within a pair, which is the
    // limit in which the two estimators must agree.
    let games = Sample::of_games(120, 0, 80);
    let pairs = Sample::of_pairs([16, 0, 48, 0, 36]);
    close(games.mu, 0.6, "game-level mu");
    close(pairs.mu, 0.6, "pair-level mu");
    close(games.var, 0.24, "game-level var");
    close(
        pairs.var,
        0.12,
        "pair-level var — exactly half, which is the sqrt(2)",
    );
    let game_llr = games.llr(Unit::Game, 0.0, 4.0).expect("not degenerate");
    let pair_llr = pairs.llr(Unit::Pair, 0.0, 4.0).expect("not degenerate");
    close(
        game_llr,
        0.4567584686550011,
        "game-level LLR, hand-computed",
    );
    close(
        pair_llr,
        0.4567584686550011,
        "pair-level LLR, hand-computed",
    );
    close(pair_llr, game_llr, "and the two units agree");

    // The evidence term really is carrying weight here: without the sqrt(2) the
    // pair-level value would differ, and it does.
    let unscaled = pairs.n as f64
        * (Unit::Game.t(4.0) * pairs.t_hat().expect("not degenerate")
            - Unit::Game.t(4.0) * Unit::Game.t(4.0) / 2.0);
    assert!(
        (unscaled - game_llr).abs() > 0.1,
        "a missing sqrt(2) must be visible: unscaled {unscaled}, correct {game_llr}"
    );

    // And a sample where every pair scored alike has no statistic at all.
    assert!(
        Sample::of_pairs([0, 0, 200, 0, 0]).is_degenerate(),
        "every pair scoring the same is a degenerate sample"
    );
}

#[test]
fn a_sample_with_no_variance_has_no_likelihood_ratio() {
    // The ruling this pins: a shutout gets `inconclusive_degenerate` and not a
    // number invented by a pseudo-count. The alternative was measured to DECIDE
    // the verdict at n = 8 (LLR 1.163 at a pseudo-count of 1e-2 and 3.683 at
    // 1e-3), which makes it a tunable, and a tunable lives in a schema
    // (CLAUDE.md rule 1, docs/decisions.md D-156).
    for sample in [
        Sample::of_games(8, 0, 0),
        Sample::of_games(0, 0, 8),
        Sample::of_games(0, 8, 0),
        Sample::of_pairs([0, 0, 4, 0, 0]),
    ] {
        assert!(sample.is_degenerate(), "{sample:?} has no variance");
        assert_eq!(sample.t_hat(), None);
        assert_eq!(sample.llr(Unit::Game, 0.0, 4.0), None);
        assert_eq!(sample.normalized_elo(Unit::Game), None);
        assert_eq!(
            crossing(&sample, Unit::Pair, 0.0, 4.0, Bounds::of(0.05, 0.05)),
            None,
            "a degenerate sample can never trigger a stop either"
        );
    }
}

#[test]
fn a_crossing_is_reported_on_the_side_it_crossed() {
    let bounds = Bounds::of(0.05, 0.05);
    let winning = Sample::of_games(2560, 512, 1024);
    assert_eq!(
        crossing(&winning, Unit::Game, 0.0, 4.0, bounds),
        Some(Verdict::H1)
    );
    let losing = Sample::of_games(1024, 512, 2560);
    assert_eq!(
        crossing(&losing, Unit::Game, 0.0, 4.0, bounds),
        Some(Verdict::H0)
    );
    // And a sample between them decides nothing, so the two assertions above
    // are not passing on a function that always answers.
    assert_eq!(
        crossing(&Sample::of_games(5, 2, 1), Unit::Game, 0.0, 4.0, bounds),
        None
    );
}

#[test]
fn normalized_elo_is_the_inverse_of_the_hypothesis_scale() {
    // The constant is asserted by VALUE. An earlier version computed
    // `t_hat / NELO_TO_T` and compared it against `normalized_elo`, which is
    // that same expression — both sides moved together under any change and the
    // test had no content.
    //
    // 800/ln(10) = 347.43558552260146. A sample whose t_hat is one standard
    // deviation therefore reports that many normalized Elo.
    close(1.0 / NELO_TO_T, 347.43558552260146, "800/ln(10)");

    // Row 1's t_hat is sqrt(1/2), so its normalized Elo is that constant times
    // sqrt(1/2) = 245.67405854855016
    let sample = Sample::of_games(5, 2, 1);
    close(
        sample.normalized_elo(Unit::Game).expect("not degenerate"),
        245.67405854855016,
        "normalized Elo at the game unit",
    );
    // The hypothesis scale round-trips in both units, so a pair-level estimate
    // is comparable with a game-level bound.
    close(Unit::Game.t(4.0) / NELO_TO_T, 4.0, "game-unit t round-trip");
    close(
        Sample::of_pairs([16, 0, 48, 0, 36])
            .normalized_elo(Unit::Pair)
            .expect("not degenerate"),
        Sample::of_games(120, 0, 80)
            .normalized_elo(Unit::Game)
            .expect("not degenerate"),
        "the same record reports the same normalized Elo at either unit",
    );
}
