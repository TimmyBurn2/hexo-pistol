//! Reading the openings fixture, and refusing anything that is not one.
//!
//! The form is WP-1.2a's (docs/decisions.md D-147): `#` comment lines, a
//! `# body_sha256 <hex>` line, and then a body that begins at the first byte
//! after the newline ending that line. A body line is a position in the
//! canonical move-list encoding — the exact tail the `position` verb takes
//! (D-6) — with everything from ` #` onward commentary a reader may strip.
//!
//! Three refusals here are worth their own sentence, because each closes a way
//! for a run to report a number that is not a measurement:
//!
//! 1. **The in-band digest is verified.** D-148 pre-registered this crate as the
//!    consumer that would either use the body digest or retire it. It is used:
//!    the arena holds only the file at load time and has no business carrying an
//!    out-of-band constant from someone else's test tree (CLAUDE.md rule 3).
//! 2. **Two openings equal up to a lattice symmetry are refused, naming both
//!    lines.** A repeated opening silently halves the information while doubling
//!    the reported n, which is the class of error rule 6's distinct-n exists to
//!    expose. Up to symmetry rather than byte-equal because a mirrored position
//!    is the same position (D-137) — and because this reader takes whatever file
//!    it is pointed at, so it may not assume its input was canonicalized.
//! 3. **Every opening must have the same turn count**, and `turn_cap` must
//!    exceed it. A mixed-length file would make one `turn_cap` mean a different
//!    horizon per game.
//!
//! Nothing here skips a line. A blank line, a stray `#` line inside the body,
//! and a line that does not replay are each a named refusal (D-139's precedent).

use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;

use pistol_core::{GameState, Turn, canonical_form};
use pistol_engine::PositionSpec;

use crate::error::ArenaError;

/// The header line carrying the body's digest.
pub const BODY_SHA_MARKER: &str = "# body_sha256 ";

/// One opening: where it came from, and the position it is.
#[derive(Debug, Clone)]
pub struct Opening {
    /// Index in the taken WINDOW, which is also its place in the report — a
    /// consumer trap worth naming: two disjoint-window runs over one book both
    /// label their openings `0..take`, and the absolute book position is
    /// `openings_skip + index`, with the skip on the report's instrument block
    /// (docs/decisions.md D-202). `line` below stays absolute.
    pub index: usize,
    /// The line of the file it was read from, for a refusal that has to say so.
    pub line: usize,
    /// The `position` verb's tail, comment stripped — what goes down the pipe.
    pub position_tail: String,
    /// The turns of the opening, which the arena's referee replays.
    pub moves: Vec<Turn>,
}

/// A loaded openings document.
#[derive(Debug, Clone)]
pub struct Openings {
    /// The taken window, in file order.
    pub taken: Vec<Opening>,
    /// The digest the header carried, echoed into the report so a run names the
    /// book it played from by content rather than by path (D-147).
    pub body_sha256: String,
    /// How many turns every opening has.
    pub opening_turns: u32,
    /// How many openings the whole file holds.
    pub total: usize,
}

/// Read `take` openings from `path`, starting after `skip`, refusing anything
/// that is not a fixture.
///
/// A contiguous window: the book is emitted in content-hash order, so any
/// window is as much a sample as a prefix is (docs/decisions.md D-143), and
/// `skip t, take t` is disjoint from `skip 0, take t` by construction — which
/// is what a confirmatory run on the SAME book needs (docs/decisions.md D-202;
/// WP-1.3's confirmation had to move to the other book for want of this knob).
/// The WHOLE file is still parsed, digest-verified and symmetry-deduped before
/// the window is cut, so a defect outside the window still refuses the file.
///
/// `turn_cap` is passed in because the rule it participates in — that a cap must
/// leave room for at least one engine move — needs both documents to state it.
pub fn load(path: &Path, take: usize, skip: usize, turn_cap: u32) -> Result<Openings, ArenaError> {
    let bytes = std::fs::read(path)
        .map_err(|io| ArenaError::openings(path, 0, format!("cannot read: {io}")))?;
    let (claimed, body_offset) = header_digest(path, &bytes)?;
    let found = pistol_cli::sha256::sha256_hex(&bytes[body_offset..]);
    if found != claimed {
        return Err(ArenaError::OpeningsDigest {
            path: path.display().to_string(),
            claimed,
            found,
        });
    }

    let text = std::str::from_utf8(&bytes)
        .map_err(|error| ArenaError::openings(path, 0, format!("not UTF-8: {error}")))?;
    let header_lines = text[..body_offset].lines().count();
    let body = &text[body_offset..];

    let mut parsed: Vec<Opening> = Vec::new();
    for (offset, raw) in body.lines().enumerate() {
        let line = header_lines + offset + 1;
        parsed.push(one_opening(path, line, parsed.len(), raw)?);
    }
    if parsed.is_empty() {
        return Err(ArenaError::openings(path, 0, "the body states no openings"));
    }
    refuse_symmetry_duplicates(path, &parsed)?;

    let opening_turns = uniform_turn_count(path, &parsed)?;
    if turn_cap <= opening_turns {
        return Err(ArenaError::config(
            "run.turn_cap",
            format!(
                "every opening in {} is {opening_turns} turns, so a cap of {turn_cap} ends each \
                 game before either engine has moved; the cap counts from the start of the game",
                path.display()
            ),
        ));
    }

    let total = parsed.len();
    if skip.saturating_add(take) > total {
        return Err(ArenaError::config(
            "run.openings_take",
            format!(
                "{} holds {total} openings and the run asks for {take} after skipping {skip} \
                 (run.openings_skip); taking fewer silently would make the run a different \
                 experiment from the one written down",
                path.display()
            ),
        ));
    }
    let mut taken: Vec<Opening> = parsed.drain(skip..skip + take).collect();
    for (index, opening) in taken.iter_mut().enumerate() {
        // Window-relative, so the schedule and the report index games the same
        // way whatever the skip; the file line stays absolute for refusals.
        opening.index = index;
    }
    Ok(Openings {
        taken,
        body_sha256: claimed,
        opening_turns,
        total,
    })
}

/// The claimed digest and where the body starts.
fn header_digest(path: &Path, bytes: &[u8]) -> Result<(String, usize), ArenaError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| ArenaError::openings(path, 0, format!("not UTF-8: {error}")))?;
    let mut offset = 0usize;
    for line in text.split_inclusive('\n') {
        offset += line.len();
        let trimmed = line.trim_end_matches('\n');
        if let Some(hex) = trimmed.strip_prefix(BODY_SHA_MARKER) {
            let hex = hex.trim();
            if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
                return Err(ArenaError::openings(
                    path,
                    0,
                    format!("`{BODY_SHA_MARKER}` must carry 64 hex digits, and carries `{hex}`"),
                ));
            }
            return Ok((hex.to_string(), offset));
        }
    }
    Err(ArenaError::openings(
        path,
        0,
        format!(
            "no `{BODY_SHA_MARKER}` line: an openings fixture pins its own body, and a file \
             without that line and a file whose body matches must not look alike \
             (docs/decisions.md D-147, D-148)"
        ),
    ))
}

/// One body line.
fn one_opening(path: &Path, line: usize, index: usize, raw: &str) -> Result<Opening, ArenaError> {
    if raw.trim().is_empty() {
        return Err(ArenaError::openings(
            path,
            line,
            "a blank line in the body; a fixture states openings and nothing else",
        ));
    }
    if raw.starts_with('#') {
        return Err(ArenaError::openings(
            path,
            line,
            "a comment inside the body; comments belong to the header, and a body this reader \
             skipped part of would not be the body the digest covers",
        ));
    }
    // Everything from " #" onward is commentary (docs/decisions.md D-143).
    let tail = match raw.find(" #") {
        Some(at) => &raw[..at],
        None => raw,
    }
    .trim();
    let spec = PositionSpec::from_str(tail).map_err(|error| {
        ArenaError::openings(path, line, format!("not a position: {}", error.why))
    })?;
    let PositionSpec::Start { moves } = &spec else {
        return Err(ArenaError::openings(
            path,
            line,
            "an opening is a move list (`start moves ...`), which is the canonical encoding of a \
             position on an unbounded board (docs/decisions.md D-6)",
        ));
    };
    // Replayed through the rules, so an illegal or already-decided opening is a
    // rules refusal and not something the arena decides for itself (rule 2).
    spec.replay()
        .map_err(|error| ArenaError::openings(path, line, error.to_string()))?;
    Ok(Opening {
        index,
        line,
        position_tail: tail.to_string(),
        moves: moves.clone(),
    })
}

/// Refuse two openings that are the same position up to a lattice symmetry.
fn refuse_symmetry_duplicates(path: &Path, openings: &[Opening]) -> Result<(), ArenaError> {
    let mut seen: BTreeMap<Vec<(pistol_core::Coord, pistol_core::Player)>, usize> = BTreeMap::new();
    for opening in openings {
        let state = replayed(opening);
        let stones: Vec<(pistol_core::Coord, pistol_core::Player)> = state.played().collect();
        let key = canonical_form(&stones);
        if let Some(&first) = seen.get(&key) {
            return Err(ArenaError::openings(
                path,
                opening.line,
                format!(
                    "the same opening as line {first} up to a lattice symmetry. A repeated \
                     opening is a forced 1-1 pair: it doubles the reported n and adds nothing, \
                     which is the error distinct-n exists to expose (CLAUDE.md rule 6, \
                     docs/decisions.md D-137)"
                ),
            ));
        }
        seen.insert(key, opening.line);
    }
    Ok(())
}

/// Every opening's turn count, refusing a file that mixes them.
fn uniform_turn_count(path: &Path, openings: &[Opening]) -> Result<u32, ArenaError> {
    let first = &openings[0];
    let expected = first.moves.len();
    for opening in openings {
        if opening.moves.len() != expected {
            return Err(ArenaError::openings(
                path,
                opening.line,
                format!(
                    "{} turns, where line {} has {expected}. One turn cap cannot mean the same \
                     horizon for two openings of different lengths",
                    opening.moves.len(),
                    first.line
                ),
            ));
        }
    }
    u32::try_from(expected).map_err(|_| {
        ArenaError::openings(path, first.line, "an opening longer than a turn counter")
    })
}

/// The state an opening leaves the board in. Infallible here: `one_opening`
/// already replayed it and refused anything the rules would not accept.
pub fn replayed(opening: &Opening) -> GameState {
    let mut state = GameState::new_game();
    for &turn in &opening.moves {
        state
            .make_turn(turn)
            .unwrap_or_else(|error| unreachable!("an opening already replayed clean: {error}"));
    }
    state
}
