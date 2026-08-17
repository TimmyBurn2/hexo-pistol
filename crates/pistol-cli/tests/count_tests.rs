//! One spelling per number, wherever this crate reads one.
//!
//! `str::parse` would accept `+4` and `004` as four. pistol-core's stone token
//! refuses both by name because each cell has exactly one spelling
//! (docs/decisions.md D-46), and a budget, a threshold and a depth are inputs a
//! log or a diff compares just as a coordinate is.

use pistol_cli::count::plain_count;

#[test]
fn a_plain_decimal_is_the_only_accepted_spelling() {
    assert_eq!(plain_count::<u32>("0"), Ok(0));
    assert_eq!(plain_count::<u32>("4"), Ok(4));
    assert_eq!(plain_count::<u64>("200000"), Ok(200_000));
    assert_eq!(
        plain_count::<u64>("18446744073709551615"),
        Ok(u64::MAX),
        "the whole range of the target type"
    );
}

#[test]
fn every_other_spelling_is_refused_and_says_why() {
    for (text, expected) in [
        ("", "empty"),
        ("+4", "leading `+`"),
        ("-4", "leading `-`"),
        ("004", "leading zeros"),
        ("0x10", "digits only"),
        ("4 ", "digits only"),
        (" 4", "digits only"),
        ("4_000", "digits only"),
        ("１", "digits only"),
        ("18446744073709551616", "this build can hold"),
    ] {
        let why = plain_count::<u64>(text)
            .err()
            .unwrap_or_else(|| panic!("`{text}` should be refused"));
        assert!(
            why.contains(expected),
            "`{text}` should be refused for {expected}, got: {why}"
        );
    }
}

#[test]
fn a_value_the_target_type_cannot_hold_is_refused_per_type() {
    assert!(plain_count::<u16>("65535").is_ok());
    assert!(plain_count::<u16>("65536").is_err());
    assert!(plain_count::<u32>("4294967296").is_err());
}
