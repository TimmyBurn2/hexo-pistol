//! The BIT-IDENTITY witness for the seat-setup extraction (docs/decisions.md
//! D-407, D-408).
//!
//! # What this file is for
//!
//! `schedule::one_game` used to spawn, hand-shake, identity-verify and
//! `newgame` its two engines inline. That sequence is now
//! [`pistol_arena::seats::with_seats`], called by BOTH `one_game` and the
//! replay mode, so the two cannot silently diverge — which is the whole reason
//! the extraction exists (three design-review rounds each caught a hand-written
//! description or copy of that sequence being wrong: D-403, D-404, D-406).
//!
//! An extraction advertised as PURE is a claim, and this file is the proof
//! rather than the claim. Each scenario below plays a sha-pinned fixture set
//! through the LIVE path — the shipped `arena` binary, two real subprocesses per
//! game — and digests the GAME RECORDS the run produced. The digests were
//! RECORDED at the pre-extraction commit and are pinned here; the same file, run
//! at the post-extraction commit, must reproduce them byte for byte. A behaviour
//! change anywhere in spawn, setup, drive or teardown moves a digest.
//!
//! # What is digested, and what is deliberately not
//!
//! The `game …` and `moves …` records, in report order, newline-terminated —
//! that is what "game record" means here, and it is what `record.rs` is a value
//! of. NOT the instrument block: it carries the stub binary's own path and
//! digest, which differ between two build trees for reasons that have nothing to
//! do with this extraction. NOT the timing block: `report.rs` names it
//! machine- and schedule-dependent. And NOT `refusal …`, whose text embeds an
//! engine's stderr tail — collected by a reader thread, so byte-pinning it would
//! be pinning a race. The refusal is asserted by SUBSTANCE instead, below.
//!
//! # Recording mode
//!
//! `PISTOL_RECORD_SEAT_IDENTITY=1 cargo test -p pistol-arena --test
//! seat_setup_identity_tests -- --nocapture` prints the table and then FAILS on
//! purpose. A recording run that could exit 0 would be an environment variable
//! that silently disables the gate.
//!
//! # RULE9-JUSTIFICATION: the scenario table, the digest function and the cases
//! are one artefact — the pinned constants mean nothing apart from the exact
//! runs that produced them, and splitting them would let a scenario drift from
//! the digest that certifies it.

mod common;

use std::path::PathBuf;

use common::{ConfigSpec, Ran, STUB, Scratch, openings_prefix, run};
use pistol_cli::sha256::sha256_hex;

/// Openings per scenario, and a cap that leaves room for a real game. Same
/// values `run_tests.rs` uses, so a reader comparing the two is comparing runs
/// of the same shape.
const OPENINGS: usize = 4;
const TURN_CAP: u32 = 10;

/// The environment binding that turns this suite into a recorder.
const RECORD: &str = "PISTOL_RECORD_SEAT_IDENTITY";

/// One run whose game records must survive the extraction unchanged.
struct Scenario {
    /// Names the scratch files and the recorded row.
    name: &'static str,
    /// `behave` for each seat.
    behave: [&'static str; 2],
    /// Workers. Two rows differ only here on purpose.
    workers: usize,
    /// The digest recorded at the pre-extraction commit.
    records_sha256: &'static str,
}

/// Every scenario, and the digest each produced at the PRE-EXTRACTION commit
/// (`7649ba0`, whose arena code is `8ca4063`'s unchanged — the commit between
/// them is docs-only).
///
/// The rows are chosen to reach every branch of the extracted sequence: a clean
/// two-engine spawn and teardown, an asymmetric matchup that actually decides
/// games, a mid-game forfeit, an engine that closes its pipe (the teardown path
/// with a dead child), and an identity drift that fires inside the extracted
/// SETUP itself at the second game's spawn. `honest_last_4w` repeats
/// `honest_last` at four workers: the extraction must not have made the record
/// set worker-dependent, so its digest is required to equal that row's.
const SCENARIOS: [Scenario; 6] = [
    Scenario {
        name: "self_match",
        behave: ["honest", "honest"],
        workers: 1,
        records_sha256: "bf8b2de0f75e9df1517661690dd5c240ca1220d6511bad16fec60ebe595e008f",
    },
    Scenario {
        name: "honest_last",
        behave: ["honest", "honest_last"],
        workers: 1,
        records_sha256: "694f1d4d898f10629530ece9d4edf19fe9c66933d1b58303b1d30d41ae0d483f",
    },
    Scenario {
        name: "honest_last_4w",
        behave: ["honest", "honest_last"],
        workers: 4,
        records_sha256: "694f1d4d898f10629530ece9d4edf19fe9c66933d1b58303b1d30d41ae0d483f",
    },
    Scenario {
        name: "illegal",
        behave: ["honest", "illegal"],
        workers: 1,
        records_sha256: "5cf9c6d06346e01d3d73df9e493f0116a7517f512fa9b5cb54ebf385f0e1d2b3",
    },
    Scenario {
        name: "exited",
        behave: ["honest", "exit"],
        workers: 1,
        records_sha256: "b35da73fd3606be6b804b561e0bcced95fe90364d45860682b7264dd251eac3d",
    },
    Scenario {
        name: "identity_drift",
        behave: ["honest", "edit_own_config"],
        workers: 1,
        records_sha256: "898314d13d594b6ad4dbbff43bb84f0b2689e85372329360ecc2aff10da40296",
    },
];

/// The `game …` and `moves …` records, in report order, as one digest.
fn records_digest(report: &str) -> String {
    let mut stream = String::new();
    // `split('\n')` and not `lines()`: an engine's verbatim refusal is free text
    // this format copies through unquoted, and two notions of "line" over one
    // document is how such text injects a record.
    for line in report.split('\n') {
        if line.starts_with("game ") || line.starts_with("moves ") {
            stream.push_str(line);
            stream.push('\n');
        }
    }
    sha256_hex(stream.as_bytes())
}

/// Play one scenario through the shipped `arena` binary.
fn play(scratch: &Scratch, scenario: &Scenario) -> (Ran, PathBuf) {
    let openings = scratch.write(
        &format!("{}-openings.txt", scenario.name),
        &openings_prefix(OPENINGS),
    );
    // One config document per SEAT even where the two behaviours are equal:
    // `edit_own_config` appends to the file it was started with, and a shared
    // document would make the drift the other seat's too.
    let config_a = scratch.stub_config(&format!("{}-a.toml", scenario.name), scenario.behave[0]);
    let config_b = scratch.stub_config(&format!("{}-b.toml", scenario.name), scenario.behave[1]);
    let spec = ConfigSpec {
        openings: &openings,
        take: OPENINGS,
        skip: 0,
        turn_cap: TURN_CAP,
        workers: scenario.workers,
        hang_ms: 30_000,
        elo1: 4.0,
        budget_kind: "depth_turns",
        budget_value: 1,
        binary_a: STUB,
        config_a: &config_a,
        binary_b: STUB,
        config_b: &config_b,
    };
    let ran = run(scratch, &spec, scenario.name);
    (ran, config_b)
}

/// Every scenario's game records reproduce the digest recorded before the
/// extraction.
#[test]
fn game_records_are_byte_identical_across_the_seat_setup_extraction() {
    let recording = std::env::var(RECORD).is_ok();
    let mut recorded: Vec<String> = Vec::new();
    let mut wrong: Vec<String> = Vec::new();
    for scenario in &SCENARIOS {
        let scratch = Scratch::new(&format!("seatid-{}", scenario.name));
        let (ran, _) = play(&scratch, scenario);
        let found = records_digest(ran.report());
        recorded.push(format!("    {:<16} {found}", scenario.name));
        if found != scenario.records_sha256 {
            wrong.push(format!(
                "{}: recorded {}, this build produced {}\n{}",
                scenario.name,
                scenario.records_sha256,
                found,
                ran.report()
            ));
        }
    }
    if recording {
        panic!(
            "RECORDING, not asserting — {RECORD} is set, so this run is not a gate:\n{}\n",
            recorded.join("\n")
        );
    }
    assert!(
        wrong.is_empty(),
        "the extraction is NOT pure — a game record moved:\n{}",
        wrong.join("\n---\n")
    );
}

/// The digests are not vacuous: two scenarios that must differ, do.
///
/// A digest function that returned a constant would pass the case above on a
/// broken extraction, which is the way a pinned-constant test fails silently.
#[test]
fn the_scenarios_do_not_all_share_one_digest() {
    let mut seen: Vec<&str> = SCENARIOS.iter().map(|s| s.records_sha256).collect();
    seen.sort_unstable();
    seen.dedup();
    assert!(
        seen.len() >= 4,
        "the pinned digests collapse to {} distinct values, so most of them certify nothing",
        seen.len()
    );
}

/// The forfeit and drift scenarios end the way the extracted sequence must make
/// them end — asserted by substance, because their verbatim text embeds an
/// engine's stderr tail and pinning that would pin a race.
#[test]
fn the_error_paths_through_the_extracted_sequence_still_fire() {
    let scratch = Scratch::new("seatid-paths");

    let (illegal, _) = play(&scratch, &SCENARIOS[3]);
    assert_eq!(illegal.code(), 1, "a forfeited run exits one");
    assert!(
        illegal.games()[0].contains("end forfeit forfeit_by b reason illegal_turn"),
        "seat b forfeits on its first answer: {}",
        illegal.games()[0]
    );

    let (exited, _) = play(&scratch, &SCENARIOS[4]);
    assert_eq!(exited.code(), 1, "an engine that quit forfeits its game");
    assert!(
        exited.games()[0].contains("end forfeit forfeit_by b reason engine_exited"),
        "seat b closed its pipe: {}",
        exited.games()[0]
    );

    // The drift fires inside the extracted SETUP — at the second game's
    // `verify_respawn`, after the first game edited the document. It aborts the
    // RUN by name and is never a game result (docs/decisions.md D-199).
    let (drift, _) = play(&scratch, &SCENARIOS[5]);
    assert_eq!(drift.code(), 1, "an aborted run still writes its prefix");
    assert!(
        drift.report().starts_with("arena_report_aborted "),
        "a drift abandons the run: {}",
        drift.report()
    );
    assert!(
        drift.report().contains("aborted IdentityDrift"),
        "and it is named: {}",
        drift.report()
    );
    assert_eq!(
        drift.games().len(),
        1,
        "one game finished before the second spawn refused: {}",
        drift.report()
    );
}
