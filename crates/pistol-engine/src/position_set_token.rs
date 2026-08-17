//! The stone-list form: `set b:<q,r> … w:<q,r> … tomove:<b|w> phase:<0|1>`.
//!
//! Both directions of one grammar, in one file, for the reason docs/decisions.md
//! D-39 gives: a formatter without its parser grows a second implementation of
//! the same grammar, and the two drift on the cases nobody tests. The choice
//! between this form and the move list is [`crate::position_token`].
//!
//! Four sections, in this order, each exactly once, every one required. A
//! section's first value is attached to its prefix — `b:0,0 1,0` is the position
//! and `b: 0,0 1,0` is a rejection — so a side with no stones yet is the bare
//! prefix and nothing else. That strictness is D-46's argument at the scale of a
//! line: one position, one spelling.

use std::fmt;

use pistol_core::{Color, Coord, Phase};

use crate::position::PositionSpec;

/// The sections of a `set` tail, in the order they are written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Black,
    White,
    ToMove,
    Phase,
}

/// Every section, in the order the grammar requires.
const SECTIONS: [Section; 4] = [
    Section::Black,
    Section::White,
    Section::ToMove,
    Section::Phase,
];

impl Section {
    /// The prefix that opens it.
    const fn prefix(self) -> &'static str {
        match self {
            Section::Black => "b:",
            Section::White => "w:",
            Section::ToMove => "tomove:",
            Section::Phase => "phase:",
        }
    }

    /// Whether it holds a list of stones rather than one value.
    const fn is_list(self) -> bool {
        matches!(self, Section::Black | Section::White)
    }
}

/// Write the tail after the form's own word.
pub(crate) fn write_set(
    f: &mut fmt::Formatter<'_>,
    black: &[Coord],
    white: &[Coord],
    to_move: Color,
    phase: Phase,
) -> fmt::Result {
    write_stones(f, Section::Black, black)?;
    f.write_str(" ")?;
    write_stones(f, Section::White, white)?;
    write!(
        f,
        " {}{} {}{}",
        Section::ToMove.prefix(),
        color_token(to_move),
        Section::Phase.prefix(),
        phase.index()
    )
}

/// `b:0,0 1,0`, or the bare prefix for a side with none yet.
fn write_stones(f: &mut fmt::Formatter<'_>, section: Section, stones: &[Coord]) -> fmt::Result {
    f.write_str(section.prefix())?;
    for (index, stone) in stones.iter().enumerate() {
        if index > 0 {
            f.write_str(" ")?;
        }
        write!(f, "{stone}")?;
    }
    Ok(())
}

/// A colour, as this form writes it.
const fn color_token(color: Color) -> &'static str {
    match color {
        Color::Black => "b",
        Color::White => "w",
    }
}

/// Read the tail after the form's own word.
pub(crate) fn parse_set(words: &[&str]) -> Result<PositionSpec, String> {
    let mut built = SetBuilder::default();
    // How many sections have been opened, which is also the index of the only one
    // that may be opened next: the order is part of the grammar, so a section out
    // of place and a section stated twice are the same rejection.
    let mut opened = 0usize;
    // The list a bare token continues. A list opened with no value accepts none,
    // because `b:` followed by a stone would spell what `b:<stone>` already does.
    let mut continuing: Option<Section> = None;

    for word in words {
        match section_of(word) {
            Some((section, value)) => {
                let expected = SECTIONS.get(opened);
                if expected != Some(&section) {
                    return Err(match expected {
                        Some(expected) => format!(
                            "expected `{}` next, got `{word}`: the sections come in the order {}",
                            expected.prefix(),
                            section_order(),
                        ),
                        None => format!("`{word}` follows the last section"),
                    });
                }
                opened += 1;
                continuing = None;
                if value.is_empty() {
                    if !section.is_list() {
                        return Err(format!(
                            "`{}` takes its value attached, as `{}b`",
                            section.prefix(),
                            section.prefix()
                        ));
                    }
                    continue;
                }
                built.take(section, value)?;
                if section.is_list() {
                    continuing = Some(section);
                }
            }
            None => {
                let Some(section) = continuing else {
                    return Err(format!(
                        "`{word}` belongs to no section: a stone follows the prefix that opens \
                         its list, as `{}0,0 1,0`",
                        Section::Black.prefix()
                    ));
                };
                built.take(section, word)?;
            }
        }
    }

    let (Some(to_move), Some(phase)) = (built.to_move, built.phase) else {
        return Err(format!(
            "every section is required, in the order {}",
            section_order()
        ));
    };
    Ok(PositionSpec::Set {
        black: built.black,
        white: built.white,
        to_move,
        phase,
    })
}

/// A `set` tail part way through being read.
#[derive(Debug, Default)]
struct SetBuilder {
    black: Vec<Coord>,
    white: Vec<Coord>,
    to_move: Option<Color>,
    phase: Option<Phase>,
}

impl SetBuilder {
    /// Read one value into a section.
    fn take(&mut self, section: Section, value: &str) -> Result<(), String> {
        match section {
            Section::Black => self.black.push(stone(value)?),
            Section::White => self.white.push(stone(value)?),
            Section::ToMove => self.to_move = Some(color(value)?),
            Section::Phase => self.phase = Some(phase(value)?),
        }
        Ok(())
    }
}

/// The section a token opens, and what it carried after the prefix.
fn section_of(word: &str) -> Option<(Section, &str)> {
    SECTIONS.iter().find_map(|&section| {
        word.strip_prefix(section.prefix())
            .map(|rest| (section, rest))
    })
}

/// The section order, for a rejection that names it.
fn section_order() -> String {
    SECTIONS
        .iter()
        .map(|section| section.prefix())
        .collect::<Vec<&str>>()
        .join(" ")
}

/// A stone token, by pistol-core's grammar (docs/decisions.md D-39).
fn stone(token: &str) -> Result<Coord, String> {
    token
        .parse::<Coord>()
        .map_err(|error| error.why.to_string())
}

/// `b` or `w`.
fn color(token: &str) -> Result<Color, String> {
    match token {
        "b" => Ok(Color::Black),
        "w" => Ok(Color::White),
        other => Err(format!(
            "`{}` takes `b` or `w`, got `{other}`",
            Section::ToMove.prefix()
        )),
    }
}

/// `0` or `1`, the phase index the rules and the key both count in.
fn phase(token: &str) -> Result<Phase, String> {
    match token {
        "0" => Ok(Phase::First),
        "1" => Ok(Phase::Second),
        other => Err(format!(
            "`{}` takes `0` (no stone of this turn placed) or `1` (one placed), got `{other}`",
            Section::Phase.prefix()
        )),
    }
}
