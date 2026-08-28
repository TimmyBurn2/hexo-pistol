mod common;

use common::{ConfigSpec, Ran, STUB, Scratch, openings_prefix, run};
use pistol_arena::error::ArenaError;
use pistol_arena::replay_report::{self, GameReplay, Replayed};
use pistol_arena::transcript;
use pistol_cli::sha256::sha256_hex;

const OPENINGS: usize = 2;
const TURN_CAP: u32 = 8;

/// A real report, because a hand-built one would agree with the reader by
/// provenance and this file's whole subject is the reader and the writer.
fn real_report(scratch: &Scratch) -> Ran {
    let openings = scratch.write("openings.txt", &openings_prefix(OPENINGS));
    let config_a = scratch.stub_config("doc-a.toml", "honest");
    let config_b = scratch.stub_config("doc-b.toml", "honest_last");
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
    run(scratch, &spec, "doc")
}

/// One replayed game, so a pass with a hole is not a pass with nothing.
fn one_game(index: usize, turns: usize) -> GameReplay {
    GameReplay {
        index,
        recorded_turns: turns,
        replayed_turns: turns,
        compared_turns: turns.saturating_sub(4),
        nodes: [1, 2],
        divergence: None,
    }
}

/// A pass that covered only some of its report says so in its first token, and
/// carries no divergence count for anybody to read.
#[test]
fn a_pass_that_did_not_cover_its_report_is_a_different_kind_of_document() {
    let scratch = Scratch::new("replaydoc-aborted");
    let ran = real_report(&scratch);
    let text = ran.report().to_string();
    let read = transcript::read(&text, sha256_hex(text.as_bytes())).expect("a real report reads");
    let turns = read.games[0].moves.len();

    let mut games: Vec<Option<GameReplay>> = read.games.iter().map(|_| None).collect();
    games[0] = Some(one_game(0, turns));
    let held = games.len();
    let played = Replayed {
        games,
        wall_ms: 7,
        workers: 1,
    };
    let error = ArenaError::config("replay", "an engine stopped answering");
    let document = replay_report::render(&read, &played, Some(&error));

    assert!(
        document.starts_with("warm_replay_aborted 1\n"),
        "the first token is what tells a consumer which kind this is, and it is the only \
         thing that does (docs/decisions.md D-160's own pattern): {document}"
    );
    assert!(
        !document.contains("\ndivergences "),
        "an incomplete pass carries NO divergence count — a consumer that read one off it \
         would be reading a criterion over a sample nobody registered: {document}"
    );
    assert!(
        document.contains(&format!("\ncovered 1 of {held}\n")),
        "it says how much it did cover: {document}"
    );
    assert!(
        document.contains("\naborted Config "),
        "and names why it stopped: {document}"
    );
}

/// A pass with no error but a missing game is ALSO the aborted kind: coverage is
/// the property, not the absence of an error.
#[test]
fn a_hole_with_no_error_attached_is_still_an_incomplete_pass() {
    let scratch = Scratch::new("replaydoc-hole");
    let ran = real_report(&scratch);
    let text = ran.report().to_string();
    let read = transcript::read(&text, sha256_hex(text.as_bytes())).expect("a real report reads");

    let games: Vec<Option<GameReplay>> = read.games.iter().map(|_| None).collect();
    let played = Replayed {
        games,
        wall_ms: 0,
        workers: 1,
    };
    let document = replay_report::render(&read, &played, None);
    assert!(
        document.starts_with("warm_replay_aborted 1\n"),
        "{document}"
    );
    assert!(!document.contains("\ndivergences "), "{document}");
    assert!(
        !document.contains("\naborted "),
        "there is no error to name, and inventing one would be worse than the hole: {document}"
    );

    // THE CONTROL: the same report with every game replayed IS the ordinary kind.
    let turns: Vec<usize> = read.games.iter().map(|game| game.moves.len()).collect();
    let played = Replayed {
        games: turns
            .iter()
            .enumerate()
            .map(|(index, count)| Some(one_game(index, *count)))
            .collect(),
        wall_ms: 0,
        workers: 1,
    };
    let document = replay_report::render(&read, &played, None);
    assert!(document.starts_with("warm_replay 1\n"), "{document}");
    assert!(document.contains("\ndivergences 0\n"), "{document}");
}
