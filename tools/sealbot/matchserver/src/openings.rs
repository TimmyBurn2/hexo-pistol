use std::path::Path;
use std::str::FromStr;

use pistol_core::{Coord, GameState, Outcome, Phase, Player, Turn, canonical_form};

use crate::sha256::sha256_hex;

/// The header line carrying the body's digest.
const BODY_SHA_MARKER: &str = "# body_sha256 ";

/// One opening, replayed into the plies the referee seeds a game with.
#[derive(Debug, Clone)]
pub struct Opening {
    /// Place in the taken window, which is how the report labels it. The
    /// absolute book position is `skip + index`; `line` below stays absolute.
    pub index: usize,
    /// The file line it was read from, so a refusal can say where.
    pub line: usize,
    /// The `position` verb's tail, which is also the report's opening key.
    pub position_tail: String,
    /// The opening's stones in the order pistol-core actually played them.
    pub plies: Vec<(Coord, Player)>,
}

/// A loaded book: the window taken, and what the whole file said.
#[derive(Debug, Clone)]
pub struct Openings {
    pub taken: Vec<Opening>,
    pub body_sha256: String,
    pub opening_turns: u32,
    pub total: usize,
}

/// Why a book was refused. Every variant names the file and, where a line is
/// responsible, the line — because a book this reader accepted in part would
/// not be the book its digest covers.
#[derive(Debug)]
pub struct OpeningsError {
    pub path: String,
    pub line: usize,
    pub why: String,
}

impl std::fmt::Display for OpeningsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.line == 0 {
            write!(f, "{}: {}", self.path, self.why)
        } else {
            write!(f, "{}:{}: {}", self.path, self.line, self.why)
        }
    }
}

fn refuse(path: &Path, line: usize, why: impl Into<String>) -> OpeningsError {
    OpeningsError {
        path: path.display().to_string(),
        line,
        why: why.into(),
    }
}

/// Read `take` openings from `path` after skipping `skip`, refusing anything
/// that is not a fixture.
///
/// The WHOLE file is parsed, digest-verified and symmetry-deduped before the
/// window is cut, so a defect outside the window still refuses the file.
///
/// # Errors
///
/// Refuses R1-R9 of `docs/experiments/anchor_v3_openings_design.md` §2.2: a
/// disagreeing digest, a blank or commented body line, a line that is not
/// `start moves …`, an opening the rules refuse, an opening that does not leave
/// the game undecided and at a turn boundary, a symmetric duplicate, mixed turn
/// counts, a window the file cannot fill, and a `turn_cap` that does not exceed
/// the opening's turns.
pub fn load(
    path: &Path,
    take: usize,
    skip: usize,
    turn_cap: u32,
) -> Result<Openings, OpeningsError> {
    let bytes =
        std::fs::read(path).map_err(|io| refuse(path, 0, format!("cannot read: {io}")))?;
    // R1.
    let (claimed, body_offset) = header_digest(path, &bytes)?;
    let found = sha256_hex(&bytes[body_offset..]);
    if found != claimed {
        return Err(refuse(
            path,
            0,
            format!(
                "the body does not match the digest its own header carries: header says \
                 {claimed}, body is {found}"
            ),
        ));
    }

    let text = std::str::from_utf8(&bytes)
        .map_err(|error| refuse(path, 0, format!("not UTF-8: {error}")))?;
    let header_lines = text[..body_offset].lines().count();

    let mut parsed: Vec<Opening> = Vec::new();
    for (offset, raw) in text[body_offset..].lines().enumerate() {
        let line = header_lines + offset + 1;
        parsed.push(one_opening(path, line, parsed.len(), raw)?);
    }
    if parsed.is_empty() {
        return Err(refuse(path, 0, "the body states no openings"));
    }
    // R6.
    refuse_symmetry_duplicates(path, &parsed)?;
    // R7.
    let opening_turns = uniform_turn_count(path, &parsed)?;
    // R9. The engines are first asked at `opening_turns + 1`, so a cap that
    // does not exceed the opening ends every game before either engine moved —
    // and the run reports "no turns recorded" for what was a config typo.
    if turn_cap <= opening_turns {
        return Err(refuse(
            path,
            0,
            format!(
                "every opening here is {opening_turns} turns, so a turn_cap of {turn_cap} ends \
                 each game before either engine is asked anything; the cap counts from the start \
                 of the game"
            ),
        ));
    }
    // R8.
    if take < 1 {
        return Err(refuse(path, 0, "openings_take must be at least 1"));
    }
    let total = parsed.len();
    if skip.saturating_add(take) > total {
        return Err(refuse(
            path,
            0,
            format!(
                "the book holds {total} openings and the run asks for {take} after skipping \
                 {skip}; taking fewer silently would make the run a different experiment from \
                 the one written down"
            ),
        ));
    }
    let mut taken: Vec<Opening> = parsed.drain(skip..skip + take).collect();
    for (index, opening) in taken.iter_mut().enumerate() {
        opening.index = index;
    }
    Ok(Openings {
        taken,
        body_sha256: claimed,
        opening_turns,
        total,
    })
}

/// The claimed digest and the byte offset the body starts at.
fn header_digest(path: &Path, bytes: &[u8]) -> Result<(String, usize), OpeningsError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| refuse(path, 0, format!("not UTF-8: {error}")))?;
    let mut offset = 0usize;
    for line in text.split_inclusive('\n') {
        offset += line.len();
        if let Some(hex) = line.trim_end_matches('\n').strip_prefix(BODY_SHA_MARKER) {
            let hex = hex.trim();
            if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
                return Err(refuse(
                    path,
                    0,
                    format!("`{BODY_SHA_MARKER}` must carry 64 hex digits, and carries `{hex}`"),
                ));
            }
            return Ok((hex.to_string(), offset));
        }
    }
    Err(refuse(
        path,
        0,
        format!(
            "no `{BODY_SHA_MARKER}` line: an openings fixture pins its own body, and a file \
             without that line must not look like one whose body matches"
        ),
    ))
}

/// One body line, replayed through the rules.
fn one_opening(
    path: &Path,
    line: usize,
    index: usize,
    raw: &str,
) -> Result<Opening, OpeningsError> {
    // R2.
    if raw.trim().is_empty() {
        return Err(refuse(
            path,
            line,
            "a blank line in the body; a fixture states openings and nothing else",
        ));
    }
    if raw.starts_with('#') {
        return Err(refuse(
            path,
            line,
            "a comment inside the body; comments belong to the header, and a body this reader \
             skipped part of would not be the body the digest covers",
        ));
    }
    let tail = match raw.find(" #") {
        Some(at) => &raw[..at],
        None => raw,
    }
    .trim();
    // R3.
    let Some(rest) = tail.strip_prefix("start moves ") else {
        return Err(refuse(
            path,
            line,
            format!("not a `start moves …` line: `{tail}`"),
        ));
    };

    let mut state = GameState::new_game();
    for (ordinal, word) in rest.split_whitespace().enumerate() {
        let turn = parse_turn(path, line, ordinal, word)?;
        // R4: the rules decide legality, here as everywhere (CLAUDE.md rule 2).
        // `make_turn` tries the canonical order and then the reverse, so a pair
        // legal in only one order is accepted whichever way the file writes it.
        state
            .make_turn(turn)
            .map_err(|error| refuse(path, line, format!("the rules refuse `{word}`: {error}")))?;
    }
    // R5. A winning stone is not refused by `place` — it returns `Ok(Win)` — so
    // the check is explicit, and it is made here rather than mid-match.
    if !matches!(state.outcome(), Outcome::Ongoing) {
        return Err(refuse(
            path,
            line,
            "the opening is already decided; an anchor game must start from a position both \
             engines can still play",
        ));
    }
    if state.phase() != Phase::First {
        return Err(refuse(
            path,
            line,
            "the opening does not end at a turn boundary, so the first engine asked would owe \
             half a turn",
        ));
    }
    Ok(Opening {
        index,
        line,
        position_tail: tail.to_string(),
        plies: state.played().collect(),
    })
}

/// One turn word: `q,r` for turn 1, `q,r/q,r` after it.
fn parse_turn(
    path: &Path,
    line: usize,
    ordinal: usize,
    word: &str,
) -> Result<Turn, OpeningsError> {
    let cell = |text: &str| {
        Coord::from_str(text)
            .map_err(|error| refuse(path, line, format!("`{text}` is not a cell: {error}")))
    };
    match word.split_once('/') {
        None => {
            if ordinal != 0 {
                return Err(refuse(
                    path,
                    line,
                    format!("turn {} carries one stone and every turn after the first owes two: `{word}`", ordinal + 1),
                ));
            }
            Ok(Turn::single(cell(word)?))
        }
        Some((first, second)) => {
            if ordinal == 0 {
                return Err(refuse(
                    path,
                    line,
                    format!("turn 1 is one stone by game rule 3: `{word}`"),
                ));
            }
            if second.contains('/') {
                return Err(refuse(
                    path,
                    line,
                    format!("a turn is at most two stones: `{word}`"),
                ));
            }
            Turn::pair(cell(first)?, cell(second)?)
                .map_err(|error| refuse(path, line, format!("`{word}` is not a turn: {error}")))
        }
    }
}

/// R6: two openings equal up to a lattice symmetry are one opening, and
/// counting them twice doubles a reported n (D-137).
fn refuse_symmetry_duplicates(path: &Path, parsed: &[Opening]) -> Result<(), OpeningsError> {
    let mut seen: std::collections::BTreeMap<Vec<(Coord, Player)>, usize> =
        std::collections::BTreeMap::new();
    for opening in parsed {
        let key = canonical_form(&opening.plies);
        if let Some(first) = seen.insert(key, opening.line) {
            return Err(refuse(
                path,
                opening.line,
                format!(
                    "the same opening up to a lattice symmetry as line {first}; a symmetric pair \
                     counted twice doubles the reported n"
                ),
            ));
        }
    }
    Ok(())
}

/// R7: the turn-cap arithmetic has ONE opening length in it.
fn uniform_turn_count(path: &Path, parsed: &[Opening]) -> Result<u32, OpeningsError> {
    let turns = |opening: &Opening| -> u32 {
        // Turn 1 is one stone, every later turn two (game rule 3).
        u32::try_from(opening.plies.len().div_ceil(2)).unwrap_or(u32::MAX)
    };
    let first = turns(&parsed[0]);
    for opening in parsed {
        let here = turns(opening);
        if here != first {
            return Err(refuse(
                path,
                opening.line,
                format!(
                    "this opening is {here} turns and line {} is {first}; a book of mixed lengths \
                     has no single turn count for the cap to be checked against",
                    parsed[0].line
                ),
            ));
        }
    }
    Ok(first)
}
