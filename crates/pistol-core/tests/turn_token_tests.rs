//! The turn token, `"q,r"` and `"q,r/q,r"` (docs/decisions.md D-5).
//!
//! The formatter and the parser ship together and are tested against each
//! other, for the reason the stone token's are (D-39): a grammar with two
//! implementations drifts on the cases nobody writes a test for, and a grammar
//! with a formatter and no parser grows its second implementation in whichever
//! crate needs one first.
//!
//! One turn has one spelling. The parser therefore refuses an uncanonical pair
//! rather than reordering it, exactly as the stone token refuses `007,0`
//! (D-46): if `"1,0/0,0"` and `"0,0/1,0"` both parsed, two protocol lines that
//! differ would mean the same turn, and an arena log or a dedupe by string
//! would eventually trip over it.

use pistol_core::error::{PAIR_NOT_CANONICAL, PAIR_OF_ONE_CELL};
use pistol_core::{Coord, ParseTurnError, Turn};

/// A cell, from its own token.
fn cell(token: &str) -> Coord {
    token.parse().expect("a stone token")
}

/// The `why` of a refused turn token.
fn why(token: &str) -> String {
    token
        .parse::<Turn>()
        .expect_err(&format!("`{token}` is not a turn token"))
        .why
}

#[test]
fn turn_tokens_round_trip_through_the_parser() {
    let tokens = [
        "0,0",
        "-1,0",
        "0,0/1,0",
        "-16,0/-8,0",
        "0,-1/5,0",
        "-32768,32767/32767,-32768",
    ];
    for token in tokens {
        let turn: Turn = token.parse().unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(turn.to_string(), token, "`{token}` did not come back");
        assert!(
            turn.is_canonical(),
            "`{token}` parsed to an uncanonical turn"
        );
    }

    // And the other way round: every turn a constructor makes is a token the
    // parser accepts.
    let single = Turn::Single(cell("3,-4"));
    let pair = Turn::pair(cell("5,0"), cell("0,-1")).expect("two distinct cells");
    for turn in [single, pair] {
        assert_eq!(
            turn.to_string().parse::<Turn>(),
            Ok(turn),
            "{turn} did not come back"
        );
    }
    assert_eq!(
        pair.to_string(),
        "0,-1/5,0",
        "the constructor canonicalizes"
    );
}

#[test]
fn turn_token_refuses_a_pair_that_is_not_one_turn_spelled_once() {
    assert_eq!(why("1,0/0,0"), PAIR_NOT_CANONICAL);
    assert_eq!(why("1,0/1,0"), PAIR_OF_ONE_CELL);
    assert!(why("0,0/1,0/2,0").contains("expected one `/`"));

    // The token is refused as a whole, and the refusal carries it back.
    assert_eq!(
        "1,0/0,0".parse::<Turn>(),
        Err(ParseTurnError {
            token: "1,0/0,0".to_string(),
            why: PAIR_NOT_CANONICAL.to_string(),
        })
    );
}

#[test]
fn turn_token_inherits_the_stone_token_grammar() {
    // Whatever `Coord` refuses, a turn refuses, on either side of the
    // separator — there is no second, looser reading of a cell here.
    for token in [
        "",
        "0",
        "0,0/",
        "/0,0",
        "0,0/+1,0",
        "0,0/ 1,0",
        "0,0 /1,0",
        "007,0/1,0",
        "0,-0/1,0",
        "0,0/1,0 ",
        "a,b/c,d",
    ] {
        assert!(
            token.parse::<Turn>().is_err(),
            "`{token}` was accepted as a turn"
        );
    }
}
