mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{ARENA, ConfigSpec, Ran, STUB, Scratch, openings_fixture, openings_prefix, run};
use pistol_arena::label_cache::LabelCache;
use pistol_core::{Coord, GameState, Player, Turn, canonical_form};

const TURN_CAP: u32 = 8;
const LABEL_NODES: &str = "5000";

/// The three openings of the design's §5 fixture, as `position` tails.
///
/// Y's four-turn prefix is a TRANSPOSITION of X's (one stone set, two move
/// lists); Z's is a LATTICE IMAGE of X's (one canonical form, two stone sets);
/// the three whole openings are of distinct canonical form, which the book
/// loader requires. It is the only fixture on which a wrong-key mutant is
/// observable, and `collision_fixture` checks the geometry with `pistol-core`
/// before any test leans on it.
const X: &str = "start moves 0,0 0,5/2,5 1,0/2,0 4,5/6,5 0,1/1,1";
const Y: &str = "start moves 0,0 0,5/4,5 1,0/2,0 2,5/6,5 0,2/1,2";
const Z: &str = "start moves 0,0 -5,5/-5,7 0,1/0,2 -5,9/-5,11 1,1/2,1";

/// The design's T6(b) opening: P1's five stones on one axis at `(-4..0, 0)`
/// with P2's far off it, so the honest stub's smallest cluster neighbour
/// `(-5, 0)` completes six and it plays the single stone — a rule-4 win.
const RULE_4_WIN: &str = "start moves 0,0 0,5/2,5 -4,0/-3,0 4,5/6,5 -2,0/-1,0";

fn turns_of(tail: &str) -> Vec<Turn> {
    tail.strip_prefix("start moves ")
        .expect("a move-list tail")
        .split(' ')
        .map(|token| token.parse().expect("a turn token"))
        .collect()
}

/// The stones a prefix places, sorted: turn `i`'s cells under player `i mod 2`.
fn stones(turns: &[Turn]) -> Vec<(Coord, Player)> {
    let mut out = Vec::new();
    for (index, turn) in turns.iter().enumerate() {
        let player = if index % 2 == 0 {
            Player::P1
        } else {
            Player::P2
        };
        out.push((turn.first(), player));
        if let Some(second) = turn.second() {
            out.push((second, player));
        }
    }
    out.sort_unstable();
    out
}

/// The fixture, with its three properties checked before use.
fn collision_fixture() -> String {
    let (x, y, z) = (turns_of(X), turns_of(Y), turns_of(Z));
    for opening in [&x, &y, &z] {
        let mut state = GameState::new_game();
        for turn in opening {
            state.make_turn(*turn).expect("a legal opening turn");
        }
    }
    assert_eq!(
        stones(&y[..4]),
        stones(&x[..4]),
        "Y's four-turn prefix is not a transposition of X's"
    );
    assert_ne!(
        &y[..4],
        &x[..4],
        "Y's prefix is X's move list, not a transposition"
    );
    assert_eq!(
        canonical_form(&stones(&z[..4])),
        canonical_form(&stones(&x[..4])),
        "Z's four-turn prefix is not a lattice image of X's"
    );
    assert_ne!(
        stones(&z[..4]),
        stones(&x[..4]),
        "Z's prefix is X's stone set, not an image of it"
    );
    let whole: Vec<Vec<(Coord, Player)>> = [&x, &y, &z]
        .iter()
        .map(|opening| canonical_form(&stones(opening)))
        .collect();
    assert!(
        whole[0] != whole[1] && whole[0] != whole[2] && whole[1] != whole[2],
        "the three whole openings must be of distinct canonical form"
    );
    openings_fixture(&[X.to_string(), Y.to_string(), Z.to_string()])
}

/// A self-play run of one stub behaviour over one book — the only shape a
/// capture can be taken from.
fn play(scratch: &Scratch, behave: &str, book: &str, tag: &str) -> Ran {
    let openings = scratch.write(&format!("openings-{tag}.txt"), book);
    let take = book.lines().filter(|line| !line.starts_with('#')).count();
    let config = scratch.stub_config(&format!("engine-{tag}.toml"), behave);
    let spec = ConfigSpec {
        openings: &openings,
        take,
        skip: 0,
        turn_cap: TURN_CAP,
        workers: 1,
        hang_ms: 30_000,
        elo1: 4.0,
        budget_kind: "nodes",
        budget_value: 5_000,
        binary_a: STUB,
        config_a: &config,
        binary_b: STUB,
        config_b: &config,
    };
    run(scratch, &spec, tag)
}

fn report_at(scratch: &Scratch, ran: &Ran, tag: &str) -> PathBuf {
    scratch.write(&format!("report-{tag}-copy.txt"), ran.report())
}

/// Ask the shipped binary to capture one report, with the given tail words.
fn capture_with(scratch: &Scratch, report: &Path, tag: &str, tail: &[&str]) -> (Output, PathBuf) {
    let out = scratch.path(&format!("capture-{tag}.txt"));
    let mut command = Command::new(ARENA);
    command
        .arg("--capture")
        .arg(report)
        .arg("--out")
        .arg(&out)
        .arg("--label-nodes")
        .arg(LABEL_NODES);
    for word in tail {
        command.arg(word);
    }
    (command.output().expect("the arena binary runs"), out)
}

fn capture(scratch: &Scratch, report: &Path, tag: &str, cache: LabelCache) -> (Output, PathBuf) {
    let tail: &[&str] = match cache {
        LabelCache::Off => &[],
        LabelCache::On => &["--label-cache"],
    };
    capture_with(scratch, report, tag, tail)
}

/// The counts line the pass prints, as `name -> value`, plus the mode word.
fn counts(output: &Output) -> (String, BTreeMap<String, u64>) {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let line = stdout
        .lines()
        .find(|line| line.starts_with("arena: label cache "))
        .unwrap_or_else(|| {
            panic!(
                "no counts line was printed.\nstdout: {stdout}\nstderr: {}",
                String::from_utf8_lossy(&output.stderr)
            )
        });
    let (mode, rest) = line
        .strip_prefix("arena: label cache ")
        .expect("the prefix")
        .split_once(": ")
        .expect("a mode word then the counts");
    let words: Vec<&str> = rest.split(' ').collect();
    let map = words
        .chunks(2)
        .map(|pair| (pair[0].to_string(), pair[1].parse().expect("a count")))
        .collect();
    (mode.to_string(), map)
}

/// Both captures of one report, asserted byte-identical.
fn identical(scratch: &Scratch, report: &Path, tag: &str) {
    let (off, uncached) = capture(scratch, report, &format!("{tag}-off"), LabelCache::Off);
    let (on, cached) = capture(scratch, report, &format!("{tag}-on"), LabelCache::On);
    assert!(
        uncached.exists() && cached.exists(),
        "a capture was refused.\nuncached stderr: {}\ncached stderr: {}",
        String::from_utf8_lossy(&off.stderr),
        String::from_utf8_lossy(&on.stderr)
    );
    assert_eq!(
        std::fs::read(&uncached).expect("uncached"),
        std::fs::read(&cached).expect("cached"),
        "the cached capture is not byte-identical to the uncached one ({tag})"
    );
}

fn transcript_of(ran: &Ran) -> pistol_arena::transcript::Transcript {
    pistol_arena::transcript::read(
        ran.report(),
        pistol_cli::sha256::sha256_hex(ran.report().as_bytes()),
    )
    .expect("the report reads back")
}

// ---------------------------------------------------------------------------
// T1, T2 — the bytes do not move, and the counts line says the cache ran.
// ---------------------------------------------------------------------------

#[test]
fn a_cached_capture_is_byte_identical_to_an_uncached_one_over_the_collision_fixture() {
    let scratch = Scratch::new("label-cache-identity");
    let ran = play(&scratch, "honest", &collision_fixture(), "identity");
    let report = report_at(&scratch, &ran, "identity");
    identical(&scratch, &report, "identity");
}

#[test]
fn the_counts_line_reads_fewer_asks_than_records_cached_and_equal_uncached() {
    // The one row a dead cache fails: a cache parsed and dropped yields the
    // uncached bytes, and only a counter at the call to the engine can say
    // whether the engine was asked. Every opening is played from both colours
    // by one stub, so the second game duplicates the first and every prefix of
    // it is a hit.
    let scratch = Scratch::new("label-cache-counts");
    let ran = play(&scratch, "honest", &collision_fixture(), "counts");
    let report = report_at(&scratch, &ran, "counts");
    let records = u64::try_from(
        transcript_of(&ran)
            .games
            .iter()
            .map(|game| {
                pistol_arena::capture::asked_prefixes(game)
                    .expect("legal")
                    .len()
            })
            .sum::<usize>(),
    )
    .expect("a count");

    let (off, _) = capture(&scratch, &report, "counts-off", LabelCache::Off);
    let (mode, uncached) = counts(&off);
    assert_eq!(mode, "off");
    assert_eq!(uncached["records"], records, "the uncached record count");
    assert_eq!(
        uncached["asks"], records,
        "uncached, every record is an ask: {uncached:?}"
    );

    let (on, _) = capture(&scratch, &report, "counts-on", LabelCache::On);
    let (mode, cached) = counts(&on);
    assert_eq!(mode, "on");
    assert_eq!(cached["records"], records, "the cached record count");
    assert!(
        cached["asks"] < records,
        "cached, the engine was asked once per record, so the cache never answered: {cached:?}"
    );
    assert_eq!(
        cached["hits"],
        records - cached["asks"],
        "hits is records less asks: {cached:?}"
    );
    // At least every prefix of every second game is a hit.
    assert!(
        cached["hits"] >= records / 2,
        "the second game of every opening duplicates the first, so at least half the records \
         are hits: {cached:?}"
    );
}

// ---------------------------------------------------------------------------
// T3, T3b, T7 — the refusals, at the CLI and at the seam.
// ---------------------------------------------------------------------------

#[test]
fn label_cache_with_census_in_either_order_is_refused_naming_both_and_leaves_no_file() {
    let scratch = Scratch::new("label-cache-census");
    let ran = play(&scratch, "census_rows", &openings_prefix(1), "census");
    let report = report_at(&scratch, &ran, "census");
    for (tag, tail) in [
        ("census-first", ["--census", "--label-cache"]),
        ("cache-first", ["--label-cache", "--census"]),
    ] {
        let (output, out) = capture_with(&scratch, &report, tag, &tail);
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        assert_eq!(
            output.status.code(),
            Some(2),
            "{tag}: a cached census capture must be refused before any game (exit 2, no \
             report); got {:?}.\nstderr: {stderr}",
            output.status.code()
        );
        assert!(
            !out.exists() && !scratch.path(&format!("capture-{tag}.census.txt")).exists(),
            "{tag}: a refused run left a file behind"
        );
        // THE REFUSAL'S OWN LINE, not the usage text the catch-all appends after
        // its sentence: that text names every word this program has, so a
        // search of the whole of stderr cannot tell X1's arm from the
        // catch-all — which is exactly what the mutation receipt found.
        let own = stderr.lines().next().unwrap_or_default();
        assert!(
            own.contains("--label-cache") && own.contains("--census"),
            "{tag}: the refusal's own line must name both words: {own}\n(whole stderr: {stderr})"
        );
    }
}

#[test]
fn capture_run_refuses_a_census_sink_under_the_cache_before_spawning_any_engine() {
    // Over a report whose engine binary no longer exists: a `run` that reached
    // the spawn would refuse for THAT reason, so the census-under-cache refusal
    // arriving instead proves it is the first statement. The control below
    // shows the same broken report does reach the spawn when the mode is off.
    let scratch = Scratch::new("label-cache-seam");
    let ran = play(&scratch, "honest", &openings_prefix(1), "seam");
    let missing = scratch.path("no-such-engine");
    let text = ran.report().replace(STUB, &missing.display().to_string());
    assert_ne!(text, ran.report(), "the report names the stub binary");
    let transcript =
        pistol_arena::transcript::read(&text, pistol_cli::sha256::sha256_hex(text.as_bytes()))
            .expect("the edited report still reads");

    let mut rows = Vec::new();
    let mut sink = pistol_arena::capture::CensusSink {
        request: pistol_engine::CensusRequest::On,
        rows: &mut rows,
    };
    let error = pistol_arena::capture::run(&transcript, 5_000, &mut sink, LabelCache::On)
        .expect_err("a census under the cache is refused");
    let message = error.to_string();
    assert!(
        message.contains("--label-cache") && message.contains("--census"),
        "the refusal must name both words: {message}"
    );

    let mut rows = Vec::new();
    let mut sink = pistol_arena::capture::CensusSink {
        request: pistol_engine::CensusRequest::On,
        rows: &mut rows,
    };
    let control = pistol_arena::capture::run(&transcript, 5_000, &mut sink, LabelCache::Off)
        .expect_err("a missing engine is refused")
        .to_string();
    assert!(
        !control.contains("--label-cache"),
        "with the cache off the same report must fail later, at the engine: {control}"
    );
}

#[test]
fn label_cache_twice_or_anywhere_but_last_is_refused() {
    let scratch = Scratch::new("label-cache-spelling");
    let ran = play(&scratch, "honest", &openings_prefix(1), "spelling");
    let report = report_at(&scratch, &ran, "spelling");
    let (twice, out) = capture_with(
        &scratch,
        &report,
        "twice",
        &["--label-cache", "--label-cache"],
    );
    assert_eq!(
        twice.status.code(),
        Some(2),
        "`--label-cache` twice is refused"
    );
    assert!(!out.exists());

    let out = scratch.path("capture-early.txt");
    let early = Command::new(ARENA)
        .arg("--capture")
        .arg(&report)
        .arg("--label-cache")
        .arg("--out")
        .arg(&out)
        .arg("--label-nodes")
        .arg(LABEL_NODES)
        .output()
        .expect("the arena binary runs");
    assert_eq!(
        early.status.code(),
        Some(2),
        "`--label-cache` anywhere but last is refused"
    );
    assert!(!out.exists());
}

// ---------------------------------------------------------------------------
// T4 — a stray line in the pipe while the cached run is serving hits.
// ---------------------------------------------------------------------------

#[test]
fn a_stray_line_in_the_pipe_while_the_cached_run_serves_hits_is_refused_in_both_runs() {
    // ONE opening, so game 1 duplicates game 0 and every prefix of it is a hit.
    // The stub writes its stray after the answer that follows its n-th
    // `newgame`; the capture sends one per spawn and one per ask, so
    // `n = P0 + 1` puts the stray after game 0's LAST answer, where the only
    // thing that can find it is a check at a prefix that is not asked.
    let scratch = Scratch::new("label-cache-stray");
    let book = openings_prefix(1);
    let honest = play(&scratch, "honest", &book, "stray-honest");
    let asked_in_game_0 = pistol_arena::capture::asked_prefixes(&transcript_of(&honest).games[0])
        .expect("legal")
        .len();
    let n = asked_in_game_0 + 1;
    assert!(
        n >= 2,
        "play sends one `newgame` per spawn, so n must be at least 2"
    );

    let ran = play(
        &scratch,
        &format!("stray_after_newgame {n}"),
        &book,
        "stray",
    );
    let transcript = transcript_of(&ran);
    assert!(
        transcript.games.iter().all(|game| !game.forfeit),
        "the stray must never fire in play, where one `newgame` per spawn is all the stub sees"
    );
    assert_eq!(
        transcript.games[0].moves, transcript.games[1].moves,
        "one stub on both seats plays the two games of one opening identically"
    );
    assert_eq!(
        pistol_arena::capture::asked_prefixes(&transcript.games[0])
            .expect("legal")
            .len(),
        asked_in_game_0,
        "the stray stub's play asks the same prefixes the honest play did"
    );
    let report = report_at(&scratch, &ran, "stray");

    let (off, out) = capture(&scratch, &report, "stray-off", LabelCache::Off);
    let stderr = String::from_utf8_lossy(&off.stderr).to_string();
    assert!(
        !out.exists(),
        "the uncached run wrote a capture with a stray in the pipe"
    );
    assert!(
        stderr.contains("game 1, turn 0")
            && (stderr.contains("spoke before it was asked") || stderr.contains("no totals line")),
        "uncached, the stray is found at game 1's first ask, by the guard or by the ask: \
         {stderr}"
    );

    // FIVE cached runs, at least one of which must refuse within game 1. The
    // guard is a time-of-check: after game 0's last answer the main thread never
    // reads the pipe again, so the stray's delivery races game 1's hits, and the
    // red team MEASURED the cached arm exiting 0 in 2 of 20 runs under `cargo
    // test` load (design §3, D-595). The mutant arm has no such residual — with
    // the guard left inside `ask`, game 1 performs no channel operation at all
    // and every run exits 0 — so "at least one of five" kills it deterministically
    // and leaves correct code a flake bound of one in ten to the fifth under load.
    // What a completing run must have written: the honest play's own uncached
    // capture, body for body — the two reports differ only in the engine config
    // the stub read, which the header carries and the records do not.
    let (honest_output, honest_capture) = capture(
        &scratch,
        &report_at(&scratch, &honest, "stray-honest"),
        "stray-honest-off",
        LabelCache::Off,
    );
    assert!(
        honest_capture.exists(),
        "the honest report's capture was refused: {}",
        String::from_utf8_lossy(&honest_output.stderr)
    );
    let honest_body = pistol_cli::corpus::emit::body_of(
        &std::fs::read_to_string(&honest_capture).expect("readable"),
    )
    .expect("a body")
    .to_string();
    let mut refused_within_game_1 = 0;
    for run in 0..5 {
        let (on, out) = capture(
            &scratch,
            &report,
            &format!("stray-on-{run}"),
            LabelCache::On,
        );
        let stderr = String::from_utf8_lossy(&on.stderr).to_string();
        if stderr.contains("game 1,") && stderr.contains("spoke before it was asked") {
            assert!(
                !out.exists(),
                "a refused cached run left a capture behind: {stderr}"
            );
            refused_within_game_1 += 1;
        } else {
            // The stated residual and nothing else: exit 0 with a complete file,
            // every record of which came from a real answer.
            assert!(
                on.status.success() && out.exists(),
                "a cached run neither refused within game 1 nor completed cleanly: {stderr}"
            );
            let body = pistol_cli::corpus::emit::body_of(
                &std::fs::read_to_string(&out).expect("readable"),
            )
            .expect("a body")
            .to_string();
            assert_eq!(
                body, honest_body,
                "a completing cached run wrote records that are not the honest capture's"
            );
        }
    }
    assert!(
        refused_within_game_1 >= 1,
        "cached, every prefix of game 1 is a hit, and the guard at a hit is what refuses; \
         none of five runs did, which is what the un-hoisted guard produces every time"
    );
}

// ---------------------------------------------------------------------------
// T5 — the two counters, on the one fixture where a wrong key is observable.
// ---------------------------------------------------------------------------

#[test]
fn the_collision_counters_are_zero_over_one_opening_and_positive_over_the_fixture() {
    let scratch = Scratch::new("label-cache-counters");
    let one = play(&scratch, "honest", &openings_prefix(1), "one");
    let (output, _) = capture(
        &scratch,
        &report_at(&scratch, &one, "one"),
        "one",
        LabelCache::On,
    );
    let (_, counted) = counts(&output);
    assert_eq!(
        (
            counted["key_pos_collisions"],
            counted["key_full_collisions"]
        ),
        (0, 0),
        "over one opening every miss has a distinct stone count: {counted:?}"
    );

    let three = play(&scratch, "honest", &collision_fixture(), "three");
    let (output, _) = capture(
        &scratch,
        &report_at(&scratch, &three, "three"),
        "three",
        LabelCache::On,
    );
    let (_, counted) = counts(&output);
    assert!(
        counted["key_pos_collisions"] >= 1,
        "Y's transposed prefix is a stone-set collision: {counted:?}"
    );
    assert!(
        counted["key_full_collisions"] >= 2,
        "Z's image prefixes and Y's transposed one are canonical collisions: {counted:?}"
    );
    assert!(
        counted["key_full_collisions"] >= counted["key_pos_collisions"],
        "a stone-set collision is a canonical one too: {counted:?}"
    );
}

// ---------------------------------------------------------------------------
// T6 — the identity over a forfeit report and over a rule-4 win.
// ---------------------------------------------------------------------------

#[test]
fn a_cached_capture_is_byte_identical_over_a_report_whose_every_game_forfeits() {
    // `demands_newgame_per_ask` forfeits in play at the first mover's second
    // turn and answers honestly under the capture's per-ask `newgame`.
    let scratch = Scratch::new("label-cache-forfeit");
    let ran = play(
        &scratch,
        "demands_newgame_per_ask",
        &openings_prefix(2),
        "forfeit",
    );
    let transcript = transcript_of(&ran);
    assert!(
        !transcript.games.is_empty() && transcript.games.iter().all(|game| game.forfeit),
        "the fixture must forfeit every game"
    );
    identical(&scratch, &report_at(&scratch, &ran, "forfeit"), "forfeit");
}

#[test]
fn a_cached_capture_is_byte_identical_over_a_report_holding_a_rule_4_win() {
    let scratch = Scratch::new("label-cache-win");
    let book = openings_fixture(&[RULE_4_WIN.to_string()]);
    let ran = play(&scratch, "honest", &book, "win");
    let transcript = transcript_of(&ran);
    let won = transcript.games.iter().any(|game| {
        !game.forfeit
            && game.moves.len() == 7
            && game.moves[6] == Turn::single(Coord::new(-5, 0))
            && game.result == pistol_arena::record::GameResult::P1Win
    });
    assert!(
        won,
        "the fixture must end in the stub's single-stone win at turn 7: {:?}",
        transcript
            .games
            .iter()
            .map(|game| (game.forfeit, game.moves.len(), game.result))
            .collect::<Vec<_>>()
    );
    identical(&scratch, &report_at(&scratch, &ran, "win"), "win");
}
