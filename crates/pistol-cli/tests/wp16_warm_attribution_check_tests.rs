//! `tools/wp16_warm_attribution_check.py` — the instrument that produces WP-1.6's
//! Criterion 1'' verdict (tools/SHELL_CHECKLIST.md item 10;
//! docs/experiments/wp16_sprt_prereg.md).
//!
//! # What this file exists to prevent
//!
//! WP-1.5b's own history: two revisions of a pre-registration registered dry-run
//! criteria that PASSED on an arena mutated to invert the entire verdict. So every
//! branch of this criterion is driven here against documents corrupted in exactly
//! the way that branch is supposed to notice, plus a CONTROL that must pass —
//! without which the whole file would be green against a checker that refuses
//! everything.
//!
//! # Three of the design's five driving tests live here
//!
//! (i) and (ii) need a real arena run and live in
//! `crates/pistol-arena/tests/replay_tests.rs`. The three that need documents built
//! to order are here: **(iii)** an inert pair recognised, excluded and its flip
//! cross-check a no-op, with a sibling identical except that one game FORFEITS and
//! is therefore NOT excluded; **(iv)** a forfeit-containing pair handled; **(v)** a
//! replay mismatch under CORRECT seats exiting on the distinct determinism-violation
//! code — beside the genuinely ambiguous case that, under a single-engine mechanism,
//! would have been indistinguishable from an attribution failure.
//!
//! # No report is committed
//!
//! A match log is an artifact and is never committed (CLAUDE.md rule 8), so every
//! document read here is built here, in a scratch directory, from the spellings
//! `conclusion.rs`, `report.rs` and `replay_report.rs` write. That is deliberate
//! twice over: a fixture copied from a run would agree with the instrument by
//! provenance, and these cases have to disagree with it on purpose.
//!
//! # The engine is a shim, and that is the point of the cold probe
//!
//! The one thing this checker runs an engine for is the cold cross-check on a
//! divergence. Its referent is therefore whatever binary it is handed, and here that
//! is a shell shim answering from a table this file writes — which is how a case can
//! make the other seat agree with the record (an inversion) or disagree with it (a
//! determinism violation) at will.
//!
//! # RULE9-JUSTIFICATION: one instrument, one file, and the corrupted documents ARE
//! the test cases — splitting the builders from the cases they exist to corrupt would
//! put the two halves of every case in different files, which is the arrangement that
//! lets a fixture drift away from the defect it was written to seed.

mod common;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{repo, scratch};
use pistol_cli::sha256::sha256_hex;

/// Turns 1 and 2 are the book, so the first turn any engine searched is turn 3.
const OPENING_TURNS: usize = 2;
/// The second turn of each opening, which is also the shim's key.
const OPENINGS: [&str; 2] = ["1,1/1,2", "2,2/2,3"];
/// The two labels, seat A first.
const LABELS: [&str; 2] = ["ra", "rb"];

/// One way a pair can fail clause (b)'s premise: what it is called, how it
/// corrupts an honest pair, and the words the refusal it earns must carry.
type Premise = (&'static str, fn(&mut Vec<Game>), &'static str);

/// One game as a fixture states it.
struct Game {
    /// What the report DECLARES this game's opening to be.
    opening: usize,
    /// Which book the move list actually starts from. Equal to `opening` in
    /// every honest fixture — separable so a case can seed a pair whose two
    /// games claim one opening and played two.
    book: usize,
    /// Whether seat A holds the first seat.
    a_is_p1: bool,
    result: &'static str,
    end: &'static str,
    /// The engine slot that forfeited, if one did.
    forfeit_by: Option<usize>,
    /// The turns AFTER the book.
    free: Vec<String>,
    nodes: [u64; 2],
}

impl Game {
    fn moves(&self) -> Vec<String> {
        let mut all = vec![String::from("0,0"), OPENINGS[self.book].to_string()];
        all.extend(self.free.iter().cloned());
        all
    }

    fn p1(&self) -> &'static str {
        LABELS[usize::from(!self.a_is_p1)]
    }

    fn p2(&self) -> &'static str {
        LABELS[usize::from(self.a_is_p1)]
    }

    /// Seat A's score, by `GameRecord::score_a`'s own rule.
    fn score_a(&self) -> f64 {
        match self.result {
            "capped" => 0.5,
            "p1_win" => f64::from(u8::from(self.p1() == LABELS[0])),
            _ => f64::from(u8::from(self.p2() == LABELS[0])),
        }
    }
}

/// A whole fixture: the games, and what each one's replay found.
struct Fixture {
    games: Vec<Game>,
    /// Per game, `Some((one-based turn, what the credited seat answered))`.
    divergences: Vec<Option<(usize, String)>>,
}

/// The report a fixture states, spelled the way `report.rs` spells one.
fn report(dir: &Path, fixture: &Fixture) -> String {
    let zero = "0".repeat(64);
    let mut out = String::from("arena_report 4\narena_version 0.0.1\n");
    out.push_str(&format!("experiment_sha256 {zero}\n"));
    out.push_str(&format!("opening_turns {OPENING_TURNS}\n"));
    out.push_str("budget nodes 50000\nturn_cap 40\n");
    out.push_str("sprt elo0 0.000000000 elo1 25.000000000 alpha 0.050000000 beta 0.050000000\n");
    for (slot, label) in ["a", "b"].iter().zip(LABELS) {
        out.push_str(&format!(
            "engine {slot} label {label} binary shim binary_sha256 {zero} config {} \
             config_sha256 {zero} weights_sha256 {zero}\n",
            dir.join(format!("cfg-{label}.toml")).display()
        ));
        out.push_str(&format!("engine_id {slot} candidate_policy radius 2\n"));
    }
    for (index, game) in fixture.games.iter().enumerate() {
        let moves = game.moves();
        out.push_str(&format!(
            "game {index} opening {} p1 {} p2 {} result {} end {} forfeit_by {} reason {} turns \
             {} dup_of none nodes_a {} nodes_b {} depth_a 2 depth_b 2 llr_game none llr_pair \
             none\n",
            game.opening,
            game.p1(),
            game.p2(),
            game.result,
            game.end,
            game.forfeit_by.map_or("none", |slot| LABELS[slot]),
            if game.end == "forfeit" {
                "illegal_turn"
            } else {
                "none"
            },
            moves.len(),
            game.nodes[0],
            game.nodes[1],
        ));
        out.push_str(&format!("moves {index} {}\n", moves.join(" ")));
    }
    let buckets = buckets(fixture);
    for (pair, bucket) in buckets.iter().enumerate() {
        out.push_str(&format!(
            "pair {pair} opening {} bucket p{bucket} score_a {:.9}\n",
            fixture.games[pair * 2].opening,
            f64::from(*bucket) / 4.0
        ));
    }
    let forfeits = fixture
        .games
        .iter()
        .filter(|game| game.end == "forfeit")
        .count();
    let wins = fixture.games.iter().filter(|g| g.score_a() == 1.0).count();
    let losses = fixture.games.iter().filter(|g| g.score_a() == 0.0).count();
    let capped = fixture
        .games
        .iter()
        .filter(|g| g.result == "capped")
        .count();
    let decided = fixture.games.len() - capped;
    out.push_str(&format!(
        "counts n {} distinct_n {} wins_a {wins} capped {capped} losses_a {losses} forfeits \
         {forfeits} decided {decided}\n",
        fixture.games.len(),
        fixture.games.len(),
    ));
    let histogram: Vec<usize> = (0..5)
        .map(|slot| buckets.iter().filter(|b| usize::from(**b) == slot).count())
        .collect();
    out.push_str(&format!(
        "pentanomial p0 {} p1 {} p2 {} p3 {} p4 {}\n",
        histogram[0], histogram[1], histogram[2], histogram[3], histogram[4]
    ));
    out.push_str("capped_fraction 0.000000000\nllr_pair last none\n");
    // Every fixture here puts all its pairs in ONE bucket, so the sample's
    // variance is zero and `sprt.rs` calls it degenerate — which is what the
    // checker's own self-check against this line will recompute. A forfeit makes
    // the arena say `invalid_forfeit` first, and the checker skips the
    // recomputation in that case rather than reading this line at all.
    out.push_str(if forfeits > 0 {
        "verdict invalid_forfeit\n"
    } else {
        "verdict inconclusive_degenerate\n"
    });
    out.push_str("# timing — machine- and schedule-dependent; excluded from every comparison\n");
    out.push_str("timing n_workers 1 wall_ms 1 discarded_in_flight 0 hang_timeout_ms 30000\n");
    out
}

/// Every pair's bucket, off the same `score_a` the report writes.
fn buckets(fixture: &Fixture) -> Vec<u8> {
    fixture
        .games
        .chunks_exact(2)
        .map(|pair| ((pair[0].score_a() + pair[1].score_a()) * 2.0).round() as u8)
        .collect()
}

/// The warm-replay document a fixture states.
fn replay(fixture: &Fixture, source_sha256: &str) -> String {
    let zero = "0".repeat(64);
    let mut out = String::from("warm_replay 1\narena_version 0.0.1\n");
    out.push_str(&format!("source_report_sha256 {source_sha256}\n"));
    out.push_str(&format!("source_experiment_sha256 {zero}\n"));
    out.push_str("budget nodes 50000\n");
    out.push_str(&format!("opening_turns {OPENING_TURNS}\nturn_cap 40\n"));
    for (slot, label) in ["a", "b"].iter().zip(LABELS) {
        out.push_str(&format!(
            "engine {slot} label {label} binary_sha256 {zero} config_sha256 {zero} \
             weights_sha256 {zero}\n"
        ));
    }
    out.push_str(&format!("games {}\n", fixture.games.len()));
    let mut found = 0;
    let mut records = String::new();
    for (index, game) in fixture.games.iter().enumerate() {
        let moves = game.moves();
        let diverged = fixture.divergences.get(index).and_then(Option::as_ref);
        // The halt rule, spelled the way `replay.rs` spells it: a divergence at
        // turn `at` means `at - 1` turns were fed, and the compared count runs
        // from the end of the book to the divergence inclusive. Getting this
        // wrong in the FIXTURE would have hidden the checker's own cross-check
        // of it, which is why the checker now derives both rather than trusting
        // the record's word.
        let (replayed, compared, status) = match diverged {
            Some((at, _)) => (at - 1, at - OPENING_TURNS, "divergence"),
            None => (
                moves.len(),
                moves.len().saturating_sub(OPENING_TURNS),
                "clean",
            ),
        };
        records.push_str(&format!(
            "replay {index} recorded_turns {} replayed_turns {replayed} compared_turns {compared} \
             nodes_a {} nodes_b {} status {status}\n",
            moves.len(),
            game.nodes[0],
            game.nodes[1],
        ));
        if let Some((at, answered)) = diverged {
            found += 1;
            let mover = if (at - 1) % 2 == 0 {
                game.p1()
            } else {
                game.p2()
            };
            records.push_str(&format!(
                "divergence {index} at_turn {at} mover {mover} mover_slot {} recorded {} answered \
                 {answered} kind move reason none\n",
                if mover == LABELS[0] { "a" } else { "b" },
                moves[at - 1],
            ));
        }
    }
    out.push_str(&records);
    out.push_str(&format!("divergences {found}\n"));
    out.push_str("# timing — machine- and schedule-dependent; excluded from every comparison\n");
    out.push_str("timing n_workers 1 wall_ms 1 hang_timeout_ms 30000\n");
    out
}

/// A shim engine answering from a per-config table keyed by the position's turn
/// count and its opening.
fn shim(dir: &Path, table: &[(&str, &str, &str)]) -> PathBuf {
    let mut tables = [String::new(), String::new()];
    for (label, key, answer) in table {
        let slot = usize::from(*label == LABELS[1]);
        tables[slot].push_str(&format!("{key} {answer}\n"));
    }
    for (label, body) in LABELS.iter().zip(&tables) {
        fs::write(dir.join(format!("cfg-{label}.toml.answers")), body).expect("answer table");
        fs::write(dir.join(format!("cfg-{label}.toml")), "# a shim\n").expect("shim config");
    }
    let path = dir.join("shim-engine");
    fs::write(
        &path,
        r#"#!/usr/bin/env bash
set -euo pipefail
CONFIG=""
while [ "$#" -gt 0 ]; do
	case "$1" in
	--config)
		CONFIG="$2"
		shift 2
		;;
	*) shift ;;
	esac
done
# `position start moves 0,0 <opening> …`: the turn count is the token count
# after `moves`, and the opening is the second of those tokens. An empty result
# from grep is legitimate here and is refused by name below, not by pipefail.
LINE="$(grep '^position start moves ' || true)"
[ -n "$LINE" ] || { echo "shim: no position line" >&2; exit 1; }
set -- $LINE
TURNS=$(( $# - 3 ))
OPENING="$5"
ANSWER="$(awk -v k="${TURNS}_${OPENING}" '$1 == k { print $2 }' "$CONFIG.answers")"
[ -n "$ANSWER" ] || { echo "shim: no answer for ${TURNS}_${OPENING} in $CONFIG.answers" >&2; exit 1; }
echo "bestmove $ANSWER"
"#,
    )
    .expect("the shim is written");
    let mut mode = fs::metadata(&path).expect("the shim exists").permissions();
    mode.set_mode(0o755);
    fs::set_permissions(&path, mode).expect("the shim is executable");
    path
}

/// Run the checker over two documents.
fn check(dir: &Path, report_text: &str, replay_text: &str, engine: &Path) -> Output {
    let report_path = dir.join("report.txt");
    let replay_path = dir.join("replay.txt");
    fs::write(&report_path, report_text).expect("the report is written");
    fs::write(&replay_path, replay_text).expect("the replay is written");
    Command::new("python3")
        .arg(repo("tools/wp16_warm_attribution_check.py"))
        .arg(&report_path)
        .arg(&replay_path)
        .arg(engine)
        .output()
        .expect("python3 runs the checker")
}

/// What a run of the checker said, for a message that carries it.
fn said(output: &Output) -> String {
    format!(
        "exit {:?}\nstdout: {}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// A decided pair whose two games differ at the first searched turn.
fn distinct_pair(opening: usize) -> Vec<Game> {
    vec![
        Game {
            opening,
            book: opening,
            a_is_p1: true,
            result: "p1_win",
            end: "normal",
            forfeit_by: None,
            free: vec![
                String::from("3,3/3,4"),
                String::from("4,4/4,5"),
                String::from("5,5/5,6"),
            ],
            nodes: [10, 20],
        },
        Game {
            opening,
            book: opening,
            a_is_p1: false,
            result: "p1_win",
            end: "normal",
            forfeit_by: None,
            free: vec![
                String::from("6,6/6,7"),
                String::from("7,7/7,8"),
                String::from("8,8/8,9"),
            ],
            nodes: [20, 10],
        },
    ]
}

/// A decided pair whose two games are the same game.
fn inert_pair(opening: usize) -> Vec<Game> {
    let free = vec![
        String::from("3,3/3,4"),
        String::from("4,4/4,5"),
        String::from("5,5/5,6"),
    ];
    vec![
        Game {
            opening,
            book: opening,
            a_is_p1: true,
            result: "p1_win",
            end: "normal",
            forfeit_by: None,
            free: free.clone(),
            nodes: [10, 20],
        },
        Game {
            opening,
            book: opening,
            a_is_p1: false,
            result: "p1_win",
            end: "normal",
            forfeit_by: None,
            free,
            nodes: [20, 10],
        },
    ]
}

/// A fixture whose replay found nothing.
fn clean(games: Vec<Game>) -> Fixture {
    let divergences = games.iter().map(|_| None).collect();
    Fixture { games, divergences }
}

/// THE CONTROL: an honest report whose replay found nothing is a measurement.
#[test]
fn a_clean_replay_of_an_honest_report_is_attributable() {
    let dir = scratch("wp16warm-control");
    let engine = shim(&dir, &[]);
    let fixture = clean(distinct_pair(0));
    let text = report(&dir, &fixture);
    let out = check(
        &dir,
        &text,
        &replay(&fixture, &sha256_hex(text.as_bytes())),
        &engine,
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "the control is refused: {}",
        said(&out)
    );
    let printed = String::from_utf8_lossy(&out.stdout);
    assert!(
        printed.contains("1 pair(s) directly attributed"),
        "clause (b) is satisfied by the theorem in reverse, and says so: {printed}"
    );
}

/// (iii) An inert pair is recognised, excluded, and the flip cross-check is a no-op.
#[test]
fn an_inert_pair_is_excluded_by_the_theorem_and_its_cross_check_is_a_no_op() {
    let dir = scratch("wp16warm-inert");
    let engine = shim(&dir, &[]);
    let fixture = clean(inert_pair(0));
    let text = report(&dir, &fixture);
    let out = check(
        &dir,
        &text,
        &replay(&fixture, &sha256_hex(text.as_bytes())),
        &engine,
    );
    assert_eq!(out.status.code(), Some(0), "{}", said(&out));
    let printed = String::from_utf8_lossy(&out.stdout);
    assert!(
        printed.contains("1 inert pair(s) excluded by theorem"),
        "the pair is excluded, not attributed: {printed}"
    );
    assert!(
        printed.contains("leaves the verdict `inconclusive_degenerate` unchanged"),
        "and the cross-check was RUN, not merely argued: {printed}"
    );
}

/// (iii, sibling) The same pair with one game FORFEITED is not excluded, and with
/// nothing to tell its seats apart it is reported unattributable rather than passed.
#[test]
fn a_forfeit_sibling_of_an_inert_pair_is_not_excluded() {
    let dir = scratch("wp16warm-forfeit-sibling");
    let engine = shim(&dir, &[]);
    let mut games = inert_pair(0);
    // THE MOVE LISTS STAY IDENTICAL, and that is the whole point of this case.
    // The forfeiting engine's refused answer has no recorded move, so "zero
    // divergence over every recorded move" is vacuously true at exactly the ply
    // that decided the result — and the ONLY thing standing between this pair
    // and the inert exclusion is the forfeit conjunct. A fixture whose two lists
    // also differed in LENGTH would be excluded by `one == two` before that
    // conjunct was ever consulted, which is how this case came to be vacuous for
    // the very thing it is named after (docs/decisions.md D-413, MAJOR 3).
    games[1].end = "forfeit";
    games[1].forfeit_by = Some(0);
    games.extend(distinct_pair(1));
    let fixture = clean(games);
    let text = report(&dir, &fixture);
    let out = check(
        &dir,
        &text,
        &replay(&fixture, &sha256_hex(text.as_bytes())),
        &engine,
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "a forfeit pair nothing tells apart is not a measurement: {}",
        said(&out)
    );
    let printed = String::from_utf8_lossy(&out.stdout);
    assert!(
        printed.contains("FAIL (b) pair 0") && printed.contains("a forfeit ended one of its games"),
        "and the refusal names the pair and why: {printed}"
    );
}

/// (iv) A forfeit-containing pair whose games DO differ at a searched turn is
/// attributed, and the cross-check says out loud that it skipped rather than passed.
#[test]
fn a_forfeit_containing_pair_that_differs_at_a_searched_turn_is_attributed() {
    let dir = scratch("wp16warm-forfeit");
    let engine = shim(&dir, &[]);
    let mut games = inert_pair(0);
    games.extend(distinct_pair(1));
    games[3].free.pop();
    games[3].end = "forfeit";
    games[3].result = "p2_win";
    games[3].forfeit_by = Some(0);
    let fixture = clean(games);
    let text = report(&dir, &fixture);
    let out = check(
        &dir,
        &text,
        &replay(&fixture, &sha256_hex(text.as_bytes())),
        &engine,
    );
    assert_eq!(out.status.code(), Some(0), "{}", said(&out));
    let printed = String::from_utf8_lossy(&out.stdout);
    assert!(
        printed.contains("1 inert pair(s) excluded by theorem")
            && printed.contains("1 pair(s) directly attributed"),
        "the forfeit pair is attributed by its own differing turn: {printed}"
    );
    assert!(
        printed.contains("skipped, not silently passed"),
        "and the pentanomial cross-check says why it did not run: {printed}"
    );
}

/// (v) A divergence the OTHER seat also cannot explain exits on the distinct
/// determinism-violation code — and the same shape, with the other seat matching,
/// is an ordinary attribution failure. One mechanism, two answers.
#[test]
fn the_dual_engine_probe_tells_a_determinism_violation_from_an_inversion() {
    let recorded = "3,3/3,4";
    for (name, other_answers, want, must_say) in [
        (
            "inversion",
            recorded,
            1,
            "the seats are the wrong way round",
        ),
        (
            "violation",
            "9,9/9,8",
            3,
            "Nothing known explains what was played",
        ),
    ] {
        let dir = scratch(&format!("wp16warm-probe-{name}"));
        // Seat `rb` is the OTHER engine at game 0 turn 3, where `ra` is credited.
        let engine = shim(&dir, &[(LABELS[1], "2_1,1/1,2", other_answers)]);
        let mut fixture = clean(distinct_pair(0));
        fixture.divergences[0] = Some((3, String::from("0,9/0,8")));
        let text = report(&dir, &fixture);
        let out = check(
            &dir,
            &text,
            &replay(&fixture, &sha256_hex(text.as_bytes())),
            &engine,
        );
        assert_eq!(
            out.status.code(),
            Some(want),
            "`{name}` took the wrong exit — a determinism violation counted as an attribution \
             failure is the reading the dual-engine probe exists to prevent: {}",
            said(&out)
        );
        assert!(
            String::from_utf8_lossy(&out.stdout).contains(must_say),
            "`{name}` did not say what it found: {}",
            said(&out)
        );
        assert_eq!(recorded, "3,3/3,4", "the recorded move is the pair's own");
    }
}

/// Equal moves and unequal nodes is the same guarantee failing more quietly, and it
/// takes the same distinct exit.
#[test]
fn a_clean_game_that_spent_different_nodes_replaying_is_a_determinism_violation() {
    let dir = scratch("wp16warm-nodes");
    let engine = shim(&dir, &[]);
    let fixture = clean(distinct_pair(0));
    let text = report(&dir, &fixture);
    let honest = replay(&fixture, &sha256_hex(text.as_bytes()));
    let tampered = honest.replacen("nodes_a 10", "nodes_a 11", 1);
    assert_ne!(honest, tampered, "the edit landed");
    let out = check(&dir, &text, &tampered, &engine);
    assert_eq!(
        out.status.code(),
        Some(3),
        "equal moves at unequal cost takes the DETERMINISM-VIOLATION exit and no other. Exit 1 \
         would have counted an instrument failure as an attribution failure — the exact \
         misreading item 12 exists to stop — and exit 0 would have passed it: {}",
        said(&out)
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("replaying the identical sequence"),
        "{}",
        said(&out)
    );
}

/// An inert pair recorded at anything but a 1-1 split contradicts the theorem that
/// excludes it, and that is a finding rather than a silent exclusion.
#[test]
fn an_inert_pair_the_report_did_not_score_one_all_is_a_finding() {
    let dir = scratch("wp16warm-inert-bucket");
    let engine = shim(&dir, &[]);
    let mut games = inert_pair(0);
    // Two IDENTICAL move lists, seats properly swapped, no forfeit — so the pair
    // IS inert and the theorem forces a 1-1 split — recorded as a win for one
    // game and a cap for the other. Impossible under the theorem, since a result
    // is a pure function of the move sequence, and that is exactly why the bucket
    // is asserted rather than assumed. One pair only, so the sample stays
    // degenerate and the pentanomial self-check has nothing to disagree about:
    // this case is about the theorem, not about the arithmetic.
    //
    // NOT a seat that fails to swap. That is a different defect — the pair
    // PREMISE rather than the theorem's conclusion — and it now takes exit 2 in
    // its own case below.
    games[1].result = "capped";
    let fixture = clean(games);
    let text = report(&dir, &fixture);
    let out = check(
        &dir,
        &text,
        &replay(&fixture, &sha256_hex(text.as_bytes())),
        &engine,
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "a pair that contradicts the theorem excluding it is a FINDING. Exit 0 would mean the \
         bucket was assumed rather than checked, and exit 2 that a premise check fired first — \
         which would be a different defect: {}",
        said(&out)
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("the inert-pair theorem forces a 1-1 split"),
        "{}",
        said(&out)
    );
}

/// Documents this checker will not take an answer from at all.
#[test]
fn documents_that_are_not_about_each_other_are_a_void_and_not_a_finding() {
    let dir = scratch("wp16warm-void");
    let engine = shim(&dir, &[]);
    let fixture = clean(distinct_pair(0));
    let text = report(&dir, &fixture);
    let honest = replay(&fixture, &sha256_hex(text.as_bytes()));

    let cases: Vec<(&str, String, &str)> = vec![
        (
            "wrong source digest",
            honest.replacen(&sha256_hex(text.as_bytes()), &"0".repeat(64), 1),
            "not about each other",
        ),
        (
            "abandoned pass",
            honest.replacen("warm_replay 1", "warm_replay_aborted 1", 1),
            "ABANDONED pass",
        ),
        (
            "a replay record for a game that is not in the report",
            honest.replacen("replay 1 recorded_turns", "replay 9 recorded_turns", 1),
            "game 1 has no `replay` record",
        ),
        (
            "fewer replay records than games",
            honest.replacen("replay 1 recorded_turns", "replayed_1 recorded_turns", 1),
            "1 `replay` record(s) for 2 game(s)",
        ),
        (
            "a divergence count that does not match",
            honest.replacen("divergences 0", "divergences 1", 1),
            "carries 0 `divergence` record",
        ),
        (
            "a clean game that fed fewer turns than it holds",
            honest.replacen("replayed_turns 5", "replayed_turns 4", 1),
            "fed only 4 of 5",
        ),
        (
            "a game count that does not match",
            honest.replacen("games 2", "games 3", 1),
            "covers 3 game(s)",
        ),
    ];
    for (name, document, must_say) in cases {
        assert_ne!(document, honest, "`{name}`'s edit landed");
        let out = check(&dir, &text, &document, &engine);
        assert_eq!(
            out.status.code(),
            Some(2),
            "`{name}` must be a void: {}",
            said(&out)
        );
        assert!(
            String::from_utf8_lossy(&out.stdout).contains(must_say),
            "`{name}` must refuse by name, saying `{must_say}`: {}",
            said(&out)
        );
    }
    // THE CONTROL, last, so the cases above cannot all be passing against a checker
    // that refuses whatever it is handed.
    let out = check(&dir, &text, &honest, &engine);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the control is refused: {}",
        said(&out)
    );
}

/// The PREMISE of clause (b)'s proof is checked, not assumed.
///
/// Both arms of the proof begin "its two games share a book prefix and swap
/// which label sits in which seat". The arena guarantees that by construction,
/// and this instrument exists to judge reports that might not be what they say
/// they are — so it must not take the guarantee on trust. A fresh-context review
/// found the witness search skipping a BOOK-level difference and attributing the
/// pair at a later turn, where the proof's own first step ("both games agree up
/// to t") is false (docs/decisions.md D-413, BLOCKING 1).
#[test]
fn a_pair_that_does_not_satisfy_the_proofs_premise_is_a_void_and_not_an_attribution() {
    let cases: [Premise; 3] = [
        (
            "book",
            |games| {
                // Both games still DECLARE opening 0; only the moves disagree,
                // and they disagree inside the book. This is the case that used
                // to be reported `directly attributed`.
                games[1].book = 1;
            },
            "inside the 2-turn book",
        ),
        (
            "opening",
            |games| games[1].opening = 1,
            "not one opening played from both seats",
        ),
        (
            "seating",
            |games| games[1].a_is_p1 = true,
            "not one seating and its reverse",
        ),
    ];
    for (name, corrupt, must_say) in cases {
        let dir = scratch(&format!("wp16warm-premise-{name}"));
        let engine = shim(&dir, &[]);
        let mut games = distinct_pair(0);
        corrupt(&mut games);
        let fixture = clean(games);
        let text = report(&dir, &fixture);
        let out = check(
            &dir,
            &text,
            &replay(&fixture, &sha256_hex(text.as_bytes())),
            &engine,
        );
        assert_eq!(
            out.status.code(),
            Some(2),
            "`{name}` must be a VOID: nothing here can attribute such a pair, and exit 0 would \
             be the proof applied to a case it does not cover — which is what a review \
             measured. Exit 1 would be a finding about engines, which this is not: {}",
            said(&out)
        );
        assert!(
            String::from_utf8_lossy(&out.stdout).contains(must_say),
            "`{name}` must refuse by name, saying `{must_say}`: {}",
            said(&out)
        );
    }
}

/// The replay document's own `status` word is not taken on trust.
///
/// Everything that makes coverage non-vacuous — the turn counts and the node
/// equality that is the externally derived referent — sat behind a `continue`
/// keyed on that word, so a record that merely CLAIMED to have diverged skipped
/// all of it and was still summarised as "replayed in full" (D-413, MAJOR 2).
#[test]
fn a_replay_record_cannot_skip_its_own_coverage_checks_by_claiming_a_divergence() {
    let dir = scratch("wp16warm-status");
    let engine = shim(&dir, &[]);
    let mut fixture = clean(distinct_pair(0));
    let text = report(&dir, &fixture);
    let honest = replay(&fixture, &sha256_hex(text.as_bytes()));

    let cases: Vec<(&str, String, &str)> = vec![
        (
            "a status word that is neither",
            honest.replacen("status clean", "status wobbly", 1),
            "is neither `clean` nor `divergence`",
        ),
        (
            "a claimed divergence with no divergence record",
            honest
                .replacen("status clean", "status divergence", 1)
                .replacen("nodes_a 10", "nodes_a 999999", 1),
            "carries 0 `divergence` record",
        ),
        (
            "a clean record that compared the wrong number of turns",
            honest.replacen("compared_turns 3", "compared_turns 1", 1),
            "compares 3 of them",
        ),
    ];
    for (name, document, must_say) in cases {
        assert_ne!(document, honest, "`{name}`'s edit landed");
        let out = check(&dir, &text, &document, &engine);
        assert_eq!(
            out.status.code(),
            Some(2),
            "`{name}` must be a void; exit 0 would be the instrument printing `replayed in \
             full` about a game it never checked: {}",
            said(&out)
        );
        assert!(
            String::from_utf8_lossy(&out.stdout).contains(must_say),
            "`{name}` must refuse by name, saying `{must_say}`: {}",
            said(&out)
        );
    }

    // A DIVERGENT record whose own halt invariant does not hold. The halt rule
    // is D-409's, and until this case nothing read it back off the document.
    fixture.divergences[0] = Some((3, String::from("0,9/0,8")));
    let text = report(&dir, &fixture);
    let diverged = replay(&fixture, &sha256_hex(text.as_bytes()));
    let unhalted = diverged.replacen("replayed_turns 2", "replayed_turns 5", 1);
    assert_ne!(unhalted, diverged, "the edit landed");
    let out = check(&dir, &text, &unhalted, &engine);
    assert_eq!(
        out.status.code(),
        Some(2),
        "a replay that fed every turn of a game it says diverged at turn 3 did not halt where \
         it says it halted, and that is a void rather than a finding: {}",
        said(&out)
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("did not halt where it says it halted"),
        "{}",
        said(&out)
    );
}

/// A forfeiting seat may spend LESS replaying than it did in the run — its last,
/// refused ask has no recorded move — but never more.
#[test]
fn a_forfeiting_seat_that_spent_more_replaying_than_it_did_live_is_a_determinism_violation() {
    let dir = scratch("wp16warm-overspend");
    let engine = shim(&dir, &[]);
    let mut games = inert_pair(0);
    games[1].end = "forfeit";
    games[1].forfeit_by = Some(0);
    games.extend(distinct_pair(1));
    let fixture = clean(games);
    let text = report(&dir, &fixture);
    let honest = replay(&fixture, &sha256_hex(text.as_bytes()));
    // Game 1's seat A forfeited having spent 20 nodes; the replay claims 21.
    let tampered = honest.replace(
        "replay 1 recorded_turns 5 replayed_turns 5 compared_turns 3 nodes_a 20",
        "replay 1 recorded_turns 5 replayed_turns 5 compared_turns 3 nodes_a 21",
    );
    assert_ne!(tampered, honest, "the edit landed");
    let out = check(&dir, &text, &tampered, &engine);
    assert_eq!(
        out.status.code(),
        Some(3),
        "spending more on the turns it DID complete than the whole game cost it live is the \
         instrument-or-engine determinism exit, not an attribution failure: {}",
        said(&out)
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("more than the whole game cost it"),
        "{}",
        said(&out)
    );
}
