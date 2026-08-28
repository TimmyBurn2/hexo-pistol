mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{ARENA, ConfigSpec, Ran, STUB, Scratch, openings_prefix, run};

/// Openings per run, and a cap that leaves room for a real game.
const OPENINGS: usize = 4;
const TURN_CAP: u32 = 12;
/// The committed fixture's openings are this long, so the first turn either
/// engine searches is the fifth.
const OPENING_TURNS: usize = 4;

/// A run of two DIFFERENT deterministic engines at a replayable budget.
fn played(scratch: &Scratch, tag: &str) -> Ran {
    let openings = scratch.write(&format!("{tag}-openings.txt"), &openings_prefix(OPENINGS));
    let config_a = scratch.stub_config(&format!("{tag}-a.toml"), "honest");
    let config_b = scratch.stub_config(&format!("{tag}-b.toml"), "honest_last");
    let spec = ConfigSpec {
        openings: &openings,
        take: OPENINGS,
        skip: 0,
        turn_cap: TURN_CAP,
        workers: 1,
        hang_ms: 30_000,
        elo1: 4.0,
        // Only a `nodes` budget replays, so only a `nodes` budget is played.
        budget_kind: "nodes",
        budget_value: 5_000,
        binary_a: STUB,
        config_a: &config_a,
        binary_b: STUB,
        config_b: &config_b,
    };
    run(scratch, spec_ref(&spec), tag)
}

/// `run` takes a reference; this keeps the borrow readable at the call site.
fn spec_ref<'a>(spec: &'a ConfigSpec<'a>) -> &'a ConfigSpec<'a> {
    spec
}

/// What a replay pass produced.
struct Pass {
    output: Output,
    document: Option<String>,
}

impl Pass {
    fn document(&self) -> &str {
        self.document.as_deref().unwrap_or_else(|| {
            panic!(
                "no replay document was written.\nstdout: {}\nstderr: {}",
                String::from_utf8_lossy(&self.output.stdout),
                String::from_utf8_lossy(&self.output.stderr)
            )
        })
    }

    fn code(&self) -> i32 {
        self.output.status.code().unwrap_or(-1)
    }

    fn stderr(&self) -> String {
        String::from_utf8_lossy(&self.output.stderr).to_string()
    }

    /// Every `key …` record, whole.
    fn records(&self, key: &str) -> Vec<&str> {
        let prefix = format!("{key} ");
        self.document()
            .split('\n')
            .filter(|line| line.starts_with(&prefix))
            .collect()
    }

    /// The part two worker counts must agree on.
    fn invariant(&self) -> &str {
        self.document()
            .split_once("\n# timing")
            .map_or_else(|| self.document(), |(head, _)| head)
    }
}

/// Replay a report at `workers`.
fn replay(scratch: &Scratch, report: &Path, tag: &str, workers: &str) -> Pass {
    let out = scratch.path(&format!("replay-{tag}.txt"));
    let output = Command::new(ARENA)
        .arg("--replay")
        .arg(report)
        .arg("--out")
        .arg(&out)
        .arg("--workers")
        .arg(workers)
        .output()
        .expect("the arena binary runs");
    Pass {
        document: std::fs::read_to_string(&out).ok(),
        output,
    }
}

/// Write a report file, edited, and hand back its path.
fn edited(scratch: &Scratch, ran: &Ran, tag: &str, edit: impl Fn(&str) -> String) -> PathBuf {
    scratch.write(&format!("report-{tag}.txt"), &edit(ran.report()))
}

/// The report a run wrote, as a file this suite can point `--replay` at.
fn report_of(scratch: &Scratch, ran: &Ran, tag: &str) -> PathBuf {
    edited(scratch, ran, tag, str::to_string)
}

/// (i) A real run replays with zero divergence, and spends the same nodes.
#[test]
fn a_real_run_replays_with_no_divergence_and_the_same_node_counts() {
    let scratch = Scratch::new("replay-clean");
    let ran = played(&scratch, "clean");
    assert_eq!(ran.code(), 0, "the run itself is clean:\n{}", ran.report());
    let report = report_of(&scratch, &ran, "clean");

    let pass = replay(&scratch, &report, "clean", "1");
    assert_eq!(
        pass.code(),
        0,
        "a report replays against its own engines:\n{}\n{}",
        pass.document(),
        pass.stderr()
    );
    assert!(
        pass.document().starts_with("warm_replay 1\n"),
        "the pass covered the whole report: {}",
        pass.document()
    );
    assert_eq!(
        pass.records("divergences"),
        vec![format!("divergences 0")],
        "nothing disagreed: {}",
        pass.document()
    );
    assert_eq!(
        pass.records("replay").len(),
        OPENINGS * 2,
        "every game was replayed: {}",
        pass.document()
    );

    // THE REFERENT THE REPLAY DOES NOT COMPUTE ABOUT ITSELF: the run's own
    // per-game node counts. Equal counts mean the same searches were asked for,
    // in the same order, at the same budget — which is what "the engine saw the
    // same game" means, and it is checkable without trusting either document's
    // account of the other.
    for (game, replayed) in ran.games().iter().zip(pass.records("replay")) {
        let of = |line: &str, key: &str| -> String {
            let words: Vec<&str> = line.split_whitespace().collect();
            let at = words
                .iter()
                .position(|word| *word == key)
                .unwrap_or_else(|| panic!("no `{key}` on `{line}`"));
            words[at + 1].to_string()
        };
        for key in ["nodes_a", "nodes_b"] {
            assert_eq!(
                of(game, key),
                of(replayed, key),
                "game {} spent different {key} replaying:\n{game}\n{replayed}",
                of(game, "game")
            );
        }
    }
}

/// The pass is worker-invariant, for the same reason a run is.
#[test]
fn a_replay_pass_says_the_same_thing_at_one_worker_and_at_four() {
    let scratch = Scratch::new("replay-workers");
    let ran = played(&scratch, "workers");
    let report = report_of(&scratch, &ran, "workers");
    let one = replay(&scratch, &report, "one", "1");
    let four = replay(&scratch, &report, "four", "4");
    assert_eq!(one.code(), 0, "{}", one.stderr());
    assert_eq!(four.code(), 0, "{}", four.stderr());
    assert_eq!(
        one.invariant(),
        four.invariant(),
        "the pass depends on the worker count"
    );
}

/// (ii) A report whose seats are swapped diverges at the first turn the two
/// engines answer differently.
#[test]
fn a_swapped_seat_label_diverges_at_the_first_differing_turn() {
    let scratch = Scratch::new("replay-swap");
    let ran = played(&scratch, "swap");
    // The corruption is exactly the one Criterion 1'' exists to catch: the
    // MOVES are untouched and only the labels move, so nothing but a replay
    // could tell the report apart from an honest one.
    let report = edited(&scratch, &ran, "swap", |text| {
        text.split('\n')
            .map(|line| {
                if line.starts_with("game ") {
                    line.replace(" p1 a p2 b ", " p1 \u{1}b p2 \u{1}a ")
                        .replace(" p1 b p2 a ", " p1 \u{1}a p2 \u{1}b ")
                        .replace('\u{1}', "")
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    });

    let pass = replay(&scratch, &report, "swap", "1");
    assert_eq!(
        pass.code(),
        1,
        "a swapped report is not clean:\n{}\n{}",
        pass.document(),
        pass.stderr()
    );
    let found = pass.records("divergence");
    assert_eq!(
        found.len(),
        OPENINGS * 2,
        "every game's seats were swapped, so every game diverges: {}",
        pass.document()
    );
    for line in &found {
        assert!(
            line.contains(&format!("at_turn {}", OPENING_TURNS + 1)),
            "the first turn either engine searched is where it shows: {line}"
        );
    }

    // THE HALT RULE, D-409's, asserted rather than only written down. Feeding an
    // engine past a divergence feeds it a move it did not choose and
    // desynchronises its table from what the live game had, so every later
    // comparison in that game would be against a state the run never reached. A
    // replay that kept walking would report `replayed_turns` equal to
    // `recorded_turns`; a fresh-context review measured that nothing caught it
    // (docs/decisions.md D-413, MAJOR 5).
    for line in pass.records("replay") {
        let of = |key: &str| -> usize {
            let words: Vec<&str> = line.split_whitespace().collect();
            let at = words
                .iter()
                .position(|word| *word == key)
                .unwrap_or_else(|| panic!("no `{key}` on `{line}`"));
            words[at + 1].parse().expect("a count")
        };
        assert_eq!(
            of("replayed_turns"),
            OPENING_TURNS,
            "the replay halted at the divergence, so it fed exactly the turns before it: {line}"
        );
        assert!(
            of("replayed_turns") < of("recorded_turns"),
            "and stopped short of the game: {line}"
        );
        assert_eq!(
            of("compared_turns"),
            1,
            "having compared just the one: {line}"
        );
    }
}

/// The replay path spawns its seats through the SAME setup the generation path
/// runs, so it sends `newgame` on every fresh spawn too.
///
/// `demands_newgame` refuses any position it is given before it has been told
/// the verb, which is the only way an engine can witness a send that is
/// otherwise a functional no-op on a fresh process (docs/decisions.md D-413,
/// MAJOR 6). If the replay reached engines by any path but
/// `seats::with_seats`, every game here would forfeit at its first ask.
#[test]
fn the_replay_path_sends_newgame_on_every_fresh_spawn_too() {
    let scratch = Scratch::new("replay-newgame");
    let openings = scratch.write("ng-openings.txt", &openings_prefix(OPENINGS));
    let config_a = scratch.stub_config("ng-a.toml", "demands_newgame");
    let config_b = scratch.stub_config("ng-b.toml", "demands_newgame");
    let spec = ConfigSpec {
        openings: &openings,
        take: OPENINGS,
        skip: 0,
        turn_cap: TURN_CAP,
        workers: 1,
        hang_ms: 30_000,
        elo1: 4.0,
        budget_kind: "nodes",
        budget_value: 5_000,
        binary_a: STUB,
        config_a: &config_a,
        binary_b: STUB,
        config_b: &config_b,
    };
    let ran = run(&scratch, &spec, "newgame");
    assert_eq!(ran.code(), 0, "the run itself is clean:\n{}", ran.report());
    let report = report_of(&scratch, &ran, "newgame");

    let pass = replay(&scratch, &report, "newgame", "1");
    assert_eq!(
        pass.code(),
        0,
        "a seat that demands `newgame` forfeited during REPLAY, so the replay reached it by \
         some path other than the shared setup:\n{}\n{}",
        pass.document(),
        pass.stderr()
    );
    assert_eq!(
        pass.records("divergences"),
        vec![String::from("divergences 0")],
        "{}",
        pass.document()
    );
}
