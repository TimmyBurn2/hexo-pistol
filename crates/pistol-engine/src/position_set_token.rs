use std::fmt;

use pistol_core::{Coord, Phase, Player};

use crate::position::PositionSpec;

/// The sections of a `set` tail, in the order they are written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    P1,
    P2,
    ToMove,
    Phase,
}

/// Every section, in the order the grammar requires.
const SECTIONS: [Section; 4] = [Section::P1, Section::P2, Section::ToMove, Section::Phase];

impl Section {
    /// The prefix that opens it.
    const fn prefix(self) -> &'static str {
        match self {
            Section::P1 => "p1:",
            Section::P2 => "p2:",
            Section::ToMove => "tomove:",
            Section::Phase => "phase:",
        }
    }

    /// Whether it holds a list of stones rather than one value.
    const fn is_list(self) -> bool {
        matches!(self, Section::P1 | Section::P2)
    }
}

/// Write the tail after the form's own word.
pub(crate) fn write_set(
    f: &mut fmt::Formatter<'_>,
    p1: &[Coord],
    p2: &[Coord],
    to_move: Player,
    phase: Phase,
) -> fmt::Result {
    write_stones(f, Section::P1, p1)?;
    f.write_str(" ")?;
    write_stones(f, Section::P2, p2)?;
    write!(
        f,
        " {}{} {}{}",
        Section::ToMove.prefix(),
        player_token(to_move),
        Section::Phase.prefix(),
        phase.index()
    )
}

/// `p1:0,0 1,0`, or the bare prefix for a side with none yet.
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

/// A player, as this form writes it.
const fn player_token(player: Player) -> &'static str {
    match player {
        Player::P1 => "p1",
        Player::P2 => "p2",
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
    // because `p1:` followed by a stone would spell what `p1:<stone>` already does.
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
                            "`{}` takes its value attached, as `{}p1`",
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
                        Section::P1.prefix()
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
        p1: built.p1,
        p2: built.p2,
        to_move,
        phase,
    })
}

/// A `set` tail part way through being read.
#[derive(Debug, Default)]
struct SetBuilder {
    p1: Vec<Coord>,
    p2: Vec<Coord>,
    to_move: Option<Player>,
    phase: Option<Phase>,
}

impl SetBuilder {
    /// Read one value into a section.
    fn take(&mut self, section: Section, value: &str) -> Result<(), String> {
        match section {
            Section::P1 => self.p1.push(stone(value)?),
            Section::P2 => self.p2.push(stone(value)?),
            Section::ToMove => self.to_move = Some(player(value)?),
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

/// `p1` or `p2`.
fn player(token: &str) -> Result<Player, String> {
    match token {
        "p1" => Ok(Player::P1),
        "p2" => Ok(Player::P2),
        other => Err(format!(
            "`{}` takes `p1` or `p2`, got `{other}`",
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
