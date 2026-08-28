use pistol_core::Coord;

#[test]
fn coord_token_round_trips_through_display_and_parse() {
    for q in [i16::MIN, -7, -1, 0, 1, 42, i16::MAX] {
        for r in [i16::MIN, -3, 0, 5, i16::MAX] {
            let cell = Coord::new(q, r);
            let token = cell.to_string();
            assert_eq!(token, format!("{q},{r}"));
            assert_eq!(token.parse::<Coord>(), Ok(cell), "token {token}");
        }
    }
}

#[test]
fn coord_token_parser_rejects_loose_forms() {
    for token in [
        "", "0", "0,", ",0", "0 , 0", " 0,0", "0,0 ", "+0,0", "0,+0", "0,0,0", "a,0", "0.5,0",
        "32768,0", "-32769,0",
    ] {
        assert!(
            token.parse::<Coord>().is_err(),
            "{token:?} should not parse as a stone"
        );
    }
    let refusal = "0 , 0".parse::<Coord>().unwrap_err();
    assert!(
        refusal.to_string().contains("0 , 0"),
        "the refusal has to quote the token: {refusal}"
    );
}

/// A step whose intermediate `direction * steps` leaves the range, but whose
/// destination does not. `Axis::ConstS` has an `r` component of `-1`, so
/// `i16::MIN` steps needs an `r` delta of `32768` — which is unrepresentable
/// even where the cell it lands on is perfectly ordinary.

#[test]
fn every_accepted_stone_token_is_the_only_spelling_of_its_cell() {
    // `display(parse(t)) == t` for everything the parser accepts, so two
    // protocol lines that differ mean different moves (docs/decisions.md D-46).
    for token in [
        "-0,0",
        "0,-0",
        "-0,-0",
        "007,0",
        "0,0007",
        "00,0",
        "0000000000000005,0",
    ] {
        assert!(
            token.parse::<Coord>().is_err(),
            "{token:?} is a second spelling of a cell that already has one"
        );
    }
    for token in [
        "0,0",
        "-1,0",
        "0,-1",
        "32767,-32768",
        "-32768,32767",
        "10,20",
    ] {
        let cell: Coord = token.parse().expect("a canonical token");
        assert_eq!(cell.to_string(), token, "round trip through the cell");
    }
}
