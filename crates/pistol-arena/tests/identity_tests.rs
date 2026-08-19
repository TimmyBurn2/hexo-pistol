//! Engine identity closes over the eval weights, and stays closed for the
//! whole run.
//!
//! WP-1.3 recorded the hole both tests pin (docs/decisions.md D-188,
//! wp13_results §6b): two engines differing only in the weight table were
//! byte-identical in every recorded digest while `nelo_pair` moved by 98
//! points, and a config edited eighteen seconds into a live run produced exit
//! 0 and a report attesting the old config. D-198 and D-199 close both.

mod common;

use common::{Scratch, openings_prefix, run, self_match};
use pistol_arena::config::ArenaConfig;
use pistol_arena::identity::EngineIdentity;
use pistol_arena::openings::Openings;
use pistol_arena::report::{self, Written};

/// A cap that leaves room for a real game past the four-turn openings.
const TURN_CAP: u32 = 10;

/// A complete arena document. Nothing here is opened: `experiment_digest`
/// reads values, not files.
fn document() -> ArenaConfig {
    ArenaConfig::parse_unvalidated(
        "schema_version = 2\n\
         [run]\n\
         openings_file = \"openings.txt\"\n\
         openings_take = 4\n\
         openings_skip = 0\n\
         turn_cap = 12\n\
         n_workers = 2\n\
         hang_timeout_ms = 1000\n\
         [budget]\n\
         kind = \"depth_turns\"\n\
         value = 2\n\
         [sprt]\n\
         elo0 = 0.0\n\
         elo1 = 4.0\n\
         alpha = 0.05\n\
         beta = 0.05\n\
         [engine_a]\n\
         label = \"a\"\n\
         binary = \"a-bin\"\n\
         config = \"a.toml\"\n\
         [engine_b]\n\
         label = \"b\"\n\
         binary = \"b-bin\"\n\
         config = \"b.toml\"\n",
    )
    .expect("a complete document parses")
}

fn identity(weights: &str) -> EngineIdentity {
    EngineIdentity {
        id_lines: vec![format!("weights_sha256 {weights}")],
        binary_sha256: "b".repeat(64),
        config_sha256: "c".repeat(64),
        weights_sha256: weights.to_string(),
    }
}

fn digest(config: &ArenaConfig, openings: &Openings, identities: &[EngineIdentity; 2]) -> String {
    report::experiment_digest(&Written {
        config,
        config_sha256: "unused-here",
        openings,
        identities,
        records: &[],
        wall_ms: 0,
        discarded: 0,
        aborted: None,
    })
}

#[test]
fn weights_digest_changes_engine_identity() {
    let config = document();
    let openings = Openings {
        taken: Vec::new(),
        body_sha256: "0".repeat(64),
        opening_turns: 3,
        total: 4,
    };
    let shared = identity(&"1".repeat(64));
    let base = [shared.clone(), identity(&"2".repeat(64))];
    let same = [shared.clone(), identity(&"2".repeat(64))];
    let moved = [shared, identity(&"3".repeat(64))];

    assert_eq!(
        digest(&config, &openings, &base),
        digest(&config, &openings, &same),
        "the digest is a function of the identity values"
    );
    assert_ne!(
        digest(&config, &openings, &base),
        digest(&config, &openings, &moved),
        "two engines differing ONLY in the weight table are different \
         experiments — the 98-nElo hole D-188 recorded"
    );
}

#[test]
fn arena_aborts_on_mid_run_config_swap() {
    // Engine B appends a comment to its own config file on the first `go` —
    // a VALID document afterwards, so an arena that missed the drift would
    // play on cleanly and this test would see exit 0. One worker makes the
    // order certain: game 0 spawns clean, the edit lands during game 0, and
    // game 1's spawn re-verification must catch it.
    let scratch = Scratch::new("mid-run-swap");
    let openings = scratch.write("openings.txt", &openings_prefix(2));
    let honest = scratch.stub_config("honest.toml", "honest");
    let editing = scratch.stub_config("editing.toml", "edit_own_config");
    let mut spec = self_match(&openings, &honest, 2, TURN_CAP, 1);
    spec.config_b = &editing;

    let ran = run(&scratch, &spec, "mid-run-swap");
    assert_eq!(
        ran.code(),
        1,
        "a run whose engine drifted is abandoned, not reported as a measurement.\nstdout: \
         {}\nstderr: {}",
        String::from_utf8_lossy(&ran.output.stdout),
        String::from_utf8_lossy(&ran.output.stderr)
    );
    let report = ran.report();
    assert!(
        report.starts_with("arena_report_aborted "),
        "the report says it is not a verdict-carrying one:\n{report}"
    );
    assert!(
        report.contains("aborted IdentityDrift"),
        "the abort names the drift:\n{report}"
    );
    assert!(
        report.contains("game 0 "),
        "the game finished before the drift is kept as a diagnostic:\n{report}"
    );
}
