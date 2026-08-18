//! What the random-openings config document refuses, and why each refusal has
//! its own name.
//!
//! Every parameter that shapes `random_openings_v1.txt` lives in a committed
//! TOML document, so these are the tests that keep the document a contract:
//! CLAUDE.md rule 1 wants it explicit and complete, and rule 3 wants a
//! wrong-shape one refused by a name rather than repaired.

mod common;

use common::repo;
use pistol_cli::random_openings::config::{
    GenerateSection, MAX_RADIUS_CEILING, N_OPENINGS_CEILING, RANDOM_OPENINGS_SCHEMA_VERSION,
    RandomOpeningsConfig,
};
use pistol_cli::random_openings::error::RandomOpeningsError;

/// The committed document, as text, with one line rewritten.
fn document(k_stones: usize, n_openings: usize, max_radius: u32, seed: u64) -> String {
    format!(
        "schema_version = {RANDOM_OPENINGS_SCHEMA_VERSION}\n\
         [generate]\n\
         k_stones = {k_stones}\n\
         n_openings = {n_openings}\n\
         max_radius = {max_radius}\n\
         seed = {seed}\n"
    )
}

#[test]
fn random_openings_reject_k4_as_mid_turn_config_error() {
    // Four stones is one stone into P1's second turn: rule 3 makes a turn two
    // stones, so a four-stone position is not one any game is *at*, it is one a
    // game is halfway through. The refusal says mid-turn rather than "not
    // supported", because the two are different complaints and only one of them
    // would be answered by widening the supported set.
    let error = RandomOpeningsConfig::parse(&document(4, 10, 5, 1))
        .expect_err("four stones is not a turn boundary");
    assert!(
        matches!(
            error,
            RandomOpeningsError::MidTurnStoneCount { k_stones: 4 }
        ),
        "k_stones = 4 is refused as mid-turn, got {error}"
    );
    // Every even count is mid-turn for the same reason, not just this one.
    for even in [2, 6, 8] {
        let error = RandomOpeningsConfig::parse(&document(even, 10, 5, 1))
            .expect_err("an even stone count is mid-turn");
        assert!(
            matches!(error, RandomOpeningsError::MidTurnStoneCount { .. }),
            "k_stones = {even} is refused as mid-turn, got {error}"
        );
    }
}

#[test]
fn random_openings_reject_a_turn_boundary_outside_the_supported_set() {
    // Odd counts ARE turn boundaries, so the mid-turn complaint would be a lie
    // about them. They are refused for the other reason: this generator is
    // specified at the turn-2 and turn-3 boundaries and nowhere else, and the
    // arithmetic that lets it skip a balance filter is an argument about k <= 5
    // (docs/decisions.md D-175).
    for odd in [1, 7, 9] {
        let error = RandomOpeningsConfig::parse(&document(odd, 10, 5, 1))
            .expect_err("only 3 and 5 are generated");
        assert!(
            matches!(error, RandomOpeningsError::UnsupportedStoneCount { .. }),
            "k_stones = {odd} is refused as unsupported, got {error}"
        );
    }
    for supported in [3, 5] {
        RandomOpeningsConfig::parse(&document(supported, 10, 5, 1))
            .unwrap_or_else(|error| panic!("k_stones = {supported} is generated: {error}"));
    }
}

#[test]
fn random_openings_refuse_a_document_with_an_unknown_key() {
    // `deny_unknown_fields`, and the test exists because the attribute is one
    // line that is easy to lose: a typo'd key that parsed would silently leave
    // the shipped value in force, which is the fallback rule 3 forbids.
    let text = format!("{}extra = 1\n", document(5, 10, 5, 1));
    let error = RandomOpeningsConfig::parse(&text).expect_err("an unknown key is refused");
    assert!(
        matches!(error, RandomOpeningsError::Schema { .. }),
        "an unknown key is a schema refusal, got {error}"
    );
}

#[test]
fn random_openings_refuse_a_document_missing_a_key() {
    // Complete, or refused. There is no code-side default for any of the four
    // (CLAUDE.md rule 1): the one place a value lives is the document.
    for missing in ["k_stones", "n_openings", "max_radius", "seed"] {
        let text: String = document(5, 10, 5, 1)
            .lines()
            .filter(|line| !line.starts_with(missing))
            .map(|line| format!("{line}\n"))
            .collect();
        let error =
            RandomOpeningsConfig::parse(&text).expect_err("an incomplete document is refused");
        assert!(
            matches!(error, RandomOpeningsError::Schema { .. }),
            "a missing `{missing}` is a schema refusal, got {error}"
        );
    }
}

#[test]
fn random_openings_refuse_a_schema_version_this_build_does_not_speak() {
    let text = document(5, 10, 5, 1).replace(
        &format!("schema_version = {RANDOM_OPENINGS_SCHEMA_VERSION}"),
        "schema_version = 99",
    );
    let error = RandomOpeningsConfig::parse(&text).expect_err("a future document is refused");
    assert!(
        matches!(
            error,
            RandomOpeningsError::SchemaVersion {
                found: 99,
                expected: RANDOM_OPENINGS_SCHEMA_VERSION
            }
        ),
        "a schema version mismatch is named, got {error}"
    );
}

#[test]
fn random_openings_refuse_a_radius_that_cannot_hold_the_stones() {
    // A radius-0 ball is the origin alone, so there is nowhere for the second
    // stone to go. Refused with the two numbers in it, because "invalid radius"
    // would not tell an operator which way to move it.
    let error =
        RandomOpeningsConfig::parse(&document(5, 10, 0, 1)).expect_err("a ball of one cell");
    assert!(
        matches!(
            error,
            RandomOpeningsError::BallTooSmall {
                cells: 1,
                k_stones: 5,
                ..
            }
        ),
        "a ball smaller than the position is named, got {error}"
    );
}

#[test]
fn random_openings_refuse_a_radius_or_a_count_past_its_typo_ceiling() {
    // Neither ceiling is a rule about the game and neither is LEGAL_RADIUS
    // (docs/decisions.md D-177). They exist so that a mistyped document fails
    // in the validator instead of allocating for a while first.
    let error = RandomOpeningsConfig::parse(&document(5, 10, MAX_RADIUS_CEILING + 1, 1))
        .expect_err("past the radius ceiling");
    assert!(
        matches!(error, RandomOpeningsError::RadiusPastCeiling { .. }),
        "a radius past the ceiling is named, got {error}"
    );
    let error = RandomOpeningsConfig::parse(&document(5, N_OPENINGS_CEILING + 1, 5, 1))
        .expect_err("past the count ceiling");
    assert!(
        matches!(error, RandomOpeningsError::CountPastCeiling { .. }),
        "a count past the ceiling is named, got {error}"
    );
    let error =
        RandomOpeningsConfig::parse(&document(5, 0, 5, 1)).expect_err("a book of no openings");
    assert!(
        matches!(error, RandomOpeningsError::CountPastCeiling { .. }),
        "asking for no openings is named, got {error}"
    );
}

#[test]
fn the_committed_config_states_the_operator_ruling() {
    // The document is the ruling's only home, so a silent edit of it would move
    // the book without moving anything a reader of docs/decisions.md D-175 can
    // see. These four numbers are that line, in force.
    let config = RandomOpeningsConfig::load(&repo("configs/random_openings_v1.toml"))
        .expect("the committed config loads");
    assert_eq!(config.generate.k_stones, 5, "k = 5 (D-175)");
    assert_eq!(config.generate.n_openings, 500, "N = 500 (D-175)");
    assert_eq!(config.generate.max_radius, 5, "max_radius = 5 (D-175)");
}

#[test]
fn generate_revalidates_a_config_it_was_handed() {
    // `generate` is public and every field of the config is public, so a caller
    // inside this workspace can build one by struct literal and never pass
    // through `parse`. A validator that only runs on one of two doors is not a
    // validator: the ceilings are what keep the generator's work finite, and
    // the stone count is what D-175's no-balance-filter arithmetic rests on —
    // at seven stones a side holds four, which is exactly the mate-in-one-turn
    // threshold that argument depends on being out of reach.
    let unvalidated = RandomOpeningsConfig {
        schema_version: 999,
        generate: GenerateSection {
            k_stones: 7,
            n_openings: 3,
            max_radius: 5,
            seed: 1,
        },
    };
    let error = pistol_cli::random_openings::generate(&unvalidated)
        .expect_err("a config that never went through parse is still validated");
    assert!(
        matches!(error, RandomOpeningsError::SchemaVersion { found: 999, .. }),
        "the first rule that fails is the one reported, got {error}"
    );

    let bad_k = RandomOpeningsConfig {
        schema_version: RANDOM_OPENINGS_SCHEMA_VERSION,
        generate: GenerateSection {
            k_stones: 7,
            ..unvalidated.generate
        },
    };
    let error = pistol_cli::random_openings::generate(&bad_k)
        .expect_err("seven stones is outside the supported set here too");
    assert!(
        matches!(error, RandomOpeningsError::UnsupportedStoneCount { .. }),
        "got {error}"
    );
}
