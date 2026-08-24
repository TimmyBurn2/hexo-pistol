//! Rules `serde` cannot express: value ranges, and agreements between fields
//! in different sections.
//!
//! # RULE9-JUSTIFICATION: one cross-field rule set, over one shared fixture
//! (CLAUDE.md rule 9).
//!
//! Every test here is a difference from `common::VALID` or `common::VALID_STAGED`,
//! stated as `replacing`/`replacing_staged` edits to one complete document each —
//! `Config::validate`'s whole surface, `Radius` and `Staged` alike. Splitting by
//! section would duplicate the two base fixtures or hoist them behind a shared
//! module neither half of the split owns; it grows again only if the schema
//! gains a third candidate policy.

mod common;

use std::path::Path;

use common::{accepted, rejection, replacing};
use pistol_engine::config::{
    EngineMode, MAX_CANDIDATE_RADIUS, MAX_MOVETIME_EPSILON_MS, MAX_Q_DEPTH_TURNS, MAX_TT_BYTES,
    MIN_TT_BYTES,
};
use pistol_engine::{Config, SCHEMA_VERSION};

/// A document that is instrument mode but asks for more than one thread parses
/// cleanly and is then refused, naming the thread count.
#[test]
fn instrument_mode_rejects_multithread_config() {
    let document = replacing("threads = 1", "threads = 2");

    // It is well-formed: this is a cross-field rule, not a parse error.
    let config = Config::parse_unvalidated(&document).expect("document should parse");
    assert_eq!(config.engine.mode, EngineMode::Instrument);
    assert_eq!(config.instrument.threads, 2);

    let (key, why) = rejection(&document);
    assert_eq!(key, "instrument.threads");
    assert!(why.contains("instrument"), "unexpected reason: {why}");
    assert!(
        why.contains('2'),
        "the reason should quote the value: {why}"
    );
}

#[test]
fn config_validation_allows_multiple_threads_in_play_mode() {
    let document = replacing("mode = \"instrument\"", "mode = \"play\"");
    let config = accepted(&replacing_in(&document, "threads = 1", "threads = 8"));
    assert_eq!(config.engine.mode, EngineMode::Play);
    assert_eq!(config.instrument.threads, 8);
}

#[test]
fn config_rejects_zero_threads() {
    let document = replacing("mode = \"instrument\"", "mode = \"play\"");
    let (key, why) = rejection(&replacing_in(&document, "threads = 1", "threads = 0"));
    assert_eq!(key, "instrument.threads");
    assert!(why.contains("at least 1"), "unexpected reason: {why}");
}

#[test]
fn config_rejects_foreign_schema_version() {
    let (key, why) = rejection(&replacing("schema_version = 2", "schema_version = 1"));
    assert_eq!(key, "schema_version");
    assert!(
        why.contains(&SCHEMA_VERSION.to_string()),
        "the reason should state the version this build reads: {why}"
    );
}

#[test]
fn config_rejects_undersized_transposition_table() {
    let too_small = MIN_TT_BYTES / 2;
    let (key, why) = rejection(&replacing(
        "tt_bytes = 1048576",
        &format!("tt_bytes = {too_small}"),
    ));
    assert_eq!(key, "search.tt_bytes");
    assert!(why.contains("at least"), "unexpected reason: {why}");
}

#[test]
fn config_rejects_oversized_transposition_table() {
    // The bound at the other end, and it exists because the failure without it
    // is not a refusal at all: an engine handed a table it cannot allocate ended
    // the process through `handle_alloc_error`, naming no key and leaving a core
    // dump where CLAUDE.md rule 3 requires one readable line.
    let too_big = MAX_TT_BYTES * 2;
    let (key, why) = rejection(&replacing(
        "tt_bytes = 1048576",
        &format!("tt_bytes = {too_big}"),
    ));
    assert_eq!(key, "search.tt_bytes");
    assert!(why.contains("at most"), "unexpected reason: {why}");
}

#[test]
fn config_rejects_transposition_table_that_is_not_a_power_of_two() {
    let (key, why) = rejection(&replacing("tt_bytes = 1048576", "tt_bytes = 3000000"));
    assert_eq!(key, "search.tt_bytes");
    assert!(why.contains("power of two"), "unexpected reason: {why}");
}

#[test]
fn config_rejects_out_of_range_candidate_radius() {
    for radius in [0, MAX_CANDIDATE_RADIUS + 1] {
        let (key, why) = rejection(&replacing("radius = 3", &format!("radius = {radius}")));
        assert_eq!(key, "search.candidate_policy.radius", "radius {radius}");
        assert!(why.contains("1..="), "radius {radius} gave: {why}");
    }
}

/// The rules' radius-8 legal region and the search candidate radius are
/// different concepts and are never compared (CLAUDE.md rule 2). A candidate
/// radius of 8 is therefore no more and no less special than any other.
#[test]
fn candidate_radius_is_not_tied_to_the_rules_radius() {
    for radius in [1, 8, MAX_CANDIDATE_RADIUS] {
        accepted(&replacing("radius = 3", &format!("radius = {radius}")));
    }
}

#[test]
fn config_rejects_empty_weights_path() {
    let (key, why) = rejection(&replacing(
        "weights_file = \"configs/eval_v0_weights.toml\"",
        "weights_file = \"\"",
    ));
    assert_eq!(key, "eval.weights_file");
    assert!(why.contains("empty path"), "unexpected reason: {why}");
}

/// Validation is pure: it never touches the filesystem, so a weights file that
/// does not exist is somebody else's loud error (docs/decisions.md D-21). This
/// is what lets the committed config name `configs/eval_v0_weights.toml`
/// before WP-05 writes it.
#[test]
fn config_accepts_a_weights_path_that_does_not_exist_yet() {
    let config = accepted(&replacing(
        "weights_file = \"configs/eval_v0_weights.toml\"",
        "weights_file = \"configs/nothing-here-yet.toml\"",
    ));
    assert!(!config.eval.weights_file.exists());
}

/// The epsilon is a promise with rejection bounds, not a free integer: zero
/// promises an unmeasurable instantaneous reply, and past the ceiling is the
/// typo class (docs/decisions.md D-18).
#[test]
fn config_rejects_out_of_range_movetime_epsilon() {
    let (key, why) = rejection(&replacing(
        "movetime_epsilon_ms = 50",
        "movetime_epsilon_ms = 0",
    ));
    assert_eq!(key, "play.movetime_epsilon_ms");
    assert!(
        why.contains(&MAX_MOVETIME_EPSILON_MS.to_string()),
        "the reason should state the ceiling: {why}"
    );

    let over = MAX_MOVETIME_EPSILON_MS + 1;
    let (key, why) = rejection(&replacing(
        "movetime_epsilon_ms = 50",
        &format!("movetime_epsilon_ms = {over}"),
    ));
    assert_eq!(key, "play.movetime_epsilon_ms");
    assert!(
        why.contains(&over.to_string()),
        "the reason should quote the value: {why}"
    );
}

/// The committed config is part of the contract, so CI loads it for real.
#[test]
fn committed_instrument_config_loads_and_validates() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../configs/instrument_v0.toml");
    let config = Config::load(&path)
        .unwrap_or_else(|error| panic!("configs/instrument_v0.toml must load: {error}"));
    assert_eq!(config.schema_version, SCHEMA_VERSION);
    assert_eq!(config.engine.mode, EngineMode::Instrument);
    assert_eq!(config.instrument.threads, 1);
}

/// `common::replacing` edits the shared fixture; this edits an already-edited
/// document, with the same insistence that the substring was really there.
fn replacing_in(document: &str, from: &str, to: &str) -> String {
    assert!(
        document.contains(from),
        "document has no `{from}` to replace"
    );
    document.replace(from, to)
}

// ---- CandidatePolicy::Staged (`U3_tier_t.md` §10) --------------------------

#[test]
fn a_staged_document_with_every_key_in_range_is_accepted() {
    let config = accepted(common::VALID_STAGED);
    let pistol_engine::config::CandidatePolicy::Staged {
        quiet_radius,
        quiet_top_k,
        widen_schedule,
        tier_t_own_count,
        tier_t_opponent_count,
        q_depth_turns,
    } = config.search.candidate_policy
    else {
        panic!("the committed staged fixture must parse as Staged");
    };
    assert_eq!(quiet_radius, 2);
    assert_eq!(quiet_top_k, 16);
    assert_eq!(widen_schedule, vec![32]);
    assert_eq!(tier_t_own_count, 2);
    assert_eq!(tier_t_opponent_count, 3);
    assert_eq!(q_depth_turns, 0);
}

#[test]
fn a_staged_quiet_radius_out_of_range_is_refused() {
    for quiet_radius in [0, MAX_CANDIDATE_RADIUS + 1] {
        let (key, why) = rejection(&common::replacing_staged(
            "quiet_radius = 2",
            &format!("quiet_radius = {quiet_radius}"),
        ));
        assert_eq!(key, "search.candidate_policy.quiet_radius");
        assert!(
            why.contains("1..="),
            "quiet_radius {quiet_radius} gave: {why}"
        );
    }
}

#[test]
fn a_staged_quiet_top_k_of_zero_is_refused() {
    let (key, why) = rejection(&common::replacing_staged(
        "quiet_top_k = 16",
        "quiet_top_k = 0",
    ));
    assert_eq!(key, "search.candidate_policy.quiet_top_k");
    assert!(why.contains("at least 1"), "unexpected reason: {why}");
}

#[test]
fn an_empty_widen_schedule_is_refused() {
    let (key, why) = rejection(&common::replacing_staged(
        "widen_schedule = [32]",
        "widen_schedule = []",
    ));
    assert_eq!(key, "search.candidate_policy.widen_schedule");
    assert!(why.contains("non-empty"), "unexpected reason: {why}");
}

/// `quiet_top_k = 64` with `widen_schedule = [32]` passes a naive
/// "non-empty and strictly increasing" check while describing a widening
/// that NARROWS — the cross-field rule revision 3's validator lacked
/// (`U3_tier_t.md` §10).
#[test]
fn a_widen_schedule_entry_that_does_not_exceed_quiet_top_k_is_refused() {
    let document = common::replacing_staged("quiet_top_k = 16", "quiet_top_k = 64");
    let (key, why) = rejection(&document);
    assert_eq!(key, "search.candidate_policy.widen_schedule");
    assert!(
        why.contains("greater than quiet_top_k"),
        "unexpected reason: {why}"
    );
}

#[test]
fn a_widen_schedule_that_does_not_strictly_increase_is_refused() {
    let document = common::replacing_staged("widen_schedule = [32]", "widen_schedule = [40, 40]");
    let (key, why) = rejection(&document);
    assert_eq!(key, "search.candidate_policy.widen_schedule");
    assert!(
        why.contains("strictly increasing"),
        "unexpected reason: {why}"
    );
}

#[test]
fn staged_tier_t_counts_outside_two_or_three_are_refused() {
    for (needle, key) in [
        (
            "tier_t_own_count = 2",
            "search.candidate_policy.tier_t_own_count",
        ),
        (
            "tier_t_opponent_count = 3",
            "search.candidate_policy.tier_t_opponent_count",
        ),
    ] {
        for bad in [0, 1, 4] {
            let replacement = format!("{} = {bad}", needle.split(" = ").next().unwrap());
            let (got_key, why) = rejection(&common::replacing_staged(needle, &replacement));
            assert_eq!(got_key, key, "{needle} -> {replacement}");
            assert!(why.contains("2 or 3"), "unexpected reason: {why}");
        }
    }
}

#[test]
fn staged_tier_t_counts_of_two_or_three_are_accepted() {
    for own in [2, 3] {
        for opponent in [2, 3] {
            let document = replacing_in(
                &common::replacing_staged(
                    "tier_t_own_count = 2",
                    &format!("tier_t_own_count = {own}"),
                ),
                "tier_t_opponent_count = 3",
                &format!("tier_t_opponent_count = {opponent}"),
            );
            accepted(&document);
        }
    }
}

/// WP-1.6 (docs/wp16_quiescence_design.md §6): a `q_depth_turns` past the
/// build's ceiling is refused, by name.
#[test]
fn a_staged_q_depth_turns_past_the_ceiling_is_refused() {
    let bad = MAX_Q_DEPTH_TURNS + 1;
    let (key, why) = rejection(&common::replacing_staged(
        "q_depth_turns = 0",
        &format!("q_depth_turns = {bad}"),
    ));
    assert_eq!(key, "search.candidate_policy.q_depth_turns");
    assert!(
        why.contains(&format!("at most {MAX_Q_DEPTH_TURNS}")),
        "unexpected reason: {why}"
    );
}

/// Zero (disabled) through the ceiling are all accepted — zero is a real
/// value, not a missing one (§6).
#[test]
fn a_staged_q_depth_turns_in_range_is_accepted() {
    for q_depth_turns in [0, 1, MAX_Q_DEPTH_TURNS] {
        let document = common::replacing_staged(
            "q_depth_turns = 0",
            &format!("q_depth_turns = {q_depth_turns}"),
        );
        accepted(&document);
    }
}
