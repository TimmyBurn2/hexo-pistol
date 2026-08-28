use pistol_core::{Coord, Player};

use super::json::Scanner;

/// The corpus's own name for a game, and this tool's dedupe key for lines: the
/// leading 16 hex characters of a SHA-256 over the move sequence.
pub const GAME_HASH_LEN: usize = 16;

/// The only `source` this tool curates from.
pub const SOURCE_HUMAN: &str = "human";

/// The widest rating this reader will take. A rating outside it is refused
/// rather than clamped: an elo is an input to eligibility, and a clamp would
/// silently move a game across the threshold (CLAUDE.md rule 3).
pub const ELO_MAX: i64 = 4000;

/// Named refusals. Constants so a test pins the reason without restating it.
pub const UNKNOWN_KEY: &str = "a key this schema does not have";
/// See [`UNKNOWN_KEY`].
pub const DUPLICATE_KEY: &str = "the same key twice in one object";
/// See [`UNKNOWN_KEY`].
pub const MISSING_KEY: &str = "the object is missing a key this schema requires";
/// See [`UNKNOWN_KEY`].
pub const ELO_KEY_REQUIRED: &str = "this tool requires `elo`, which SCHEMA.md allows to be absent when a corpus is exported with \
     --no-meta: opening eligibility is rated, so a corpus without ratings is a different document";
/// See [`UNKNOWN_KEY`].
pub const BAD_GAME_HASH: &str = "`game_hash` is 16 lower-case hex characters";
/// See [`UNKNOWN_KEY`].
pub const BAD_SOURCE: &str = "this tool curates only `human` games: a corpus mixing in engine games would need its own \
     eligibility rule, and curating from one silently would not be that rule";
/// See [`UNKNOWN_KEY`].
pub const BAD_WINNER: &str = "`winner` is 1 or -1; the corpus holds only decisive games";
/// See [`UNKNOWN_KEY`].
pub const BAD_ELO_ARITY: &str = "`elo` is a two-element array";
/// See [`UNKNOWN_KEY`].
pub const ELO_OUT_OF_RANGE: &str = "an elo outside the range this reader accepts";
/// See [`UNKNOWN_KEY`].
pub const BAD_MOVE_ARITY: &str = "a move is a two-element array `[q, r]`";
/// See [`UNKNOWN_KEY`].
pub const COORD_OUT_OF_RANGE: &str =
    "a coordinate outside the i16 lattice pistol-core addresses (docs/decisions.md D-34)";
/// See [`UNKNOWN_KEY`].
pub const NO_MOVES: &str = "`moves` is empty, so the line describes no game";

/// One line of the corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// The corpus's content hash for this game.
    pub game_hash: String,
    /// Every stone, in the order it was played.
    pub moves: Vec<Coord>,
    /// Who the corpus says won.
    pub winner: Player,
    /// Each side's rating at game time, absent where the corpus has `null`.
    pub elo: [Option<u16>; 2],
}

impl Record {
    /// The lower of the two ratings, where both are present.
    ///
    /// This is the quantity eligibility is stated in: a game is as good as its
    /// weaker player, so the pair's floor is what says whether both sides could
    /// play (docs/decisions.md D-142).
    pub fn min_elo(&self) -> Option<u16> {
        match self.elo {
            [Some(a), Some(b)] => Some(a.min(b)),
            _ => None,
        }
    }

    /// How far apart the two ratings are, where both are present.
    pub fn elo_gap(&self) -> Option<u16> {
        match self.elo {
            [Some(a), Some(b)] => Some(a.abs_diff(b)),
            _ => None,
        }
    }
}

/// The five keys, in the order the schema documents them.
const KEYS: [&str; 5] = ["game_hash", "moves", "winner", "source", "elo"];

/// Read one line.
pub fn parse(line: &str) -> Result<Record, super::json::ScanError> {
    let mut scanner = Scanner::new(line);
    // Named rather than left to `expect`'s generic message: this is the refusal
    // that says "the file is not this document at all", which is a different
    // thing from a token going wrong inside one.
    if !scanner.accept(b'{') {
        return scanner.refuse(super::json::EXPECTED_OBJECT);
    }

    let mut game_hash: Option<String> = None;
    let mut moves: Option<Vec<Coord>> = None;
    let mut winner: Option<Player> = None;
    let mut source_seen = false;
    let mut elo: Option<[Option<u16>; 2]> = None;

    if !scanner.accept(b'}') {
        loop {
            let key_at = scanner.at();
            let key = scanner.string()?.to_string();
            scanner.expect(b':')?;
            match key.as_str() {
                "game_hash" => {
                    if game_hash.is_some() {
                        return scanner.refuse(DUPLICATE_KEY);
                    }
                    let text = scanner.string()?;
                    if text.len() != GAME_HASH_LEN
                        || !text
                            .bytes()
                            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                    {
                        return scanner.refuse(format!("{BAD_GAME_HASH}, got `{text}`"));
                    }
                    game_hash = Some(text.to_string());
                }
                "moves" => {
                    if moves.is_some() {
                        return scanner.refuse(DUPLICATE_KEY);
                    }
                    moves = Some(read_moves(&mut scanner)?);
                }
                "winner" => {
                    if winner.is_some() {
                        return scanner.refuse(DUPLICATE_KEY);
                    }
                    winner = Some(match scanner.integer()? {
                        1 => Player::P1,
                        -1 => Player::P2,
                        other => return scanner.refuse(format!("{BAD_WINNER}, got `{other}`")),
                    });
                }
                "source" => {
                    if source_seen {
                        return scanner.refuse(DUPLICATE_KEY);
                    }
                    let text = scanner.string()?;
                    if text != SOURCE_HUMAN {
                        return scanner.refuse(format!("{BAD_SOURCE}, got `{text}`"));
                    }
                    source_seen = true;
                }
                "elo" => {
                    if elo.is_some() {
                        return scanner.refuse(DUPLICATE_KEY);
                    }
                    elo = Some(read_elo(&mut scanner)?);
                }
                _ => {
                    return Err(super::json::ScanError {
                        at: key_at,
                        why: format!(
                            "{UNKNOWN_KEY}: `{key}`; this schema has {}",
                            KEYS.join(", ")
                        ),
                    });
                }
            }
            if !scanner.accept(b',') {
                break;
            }
        }
        scanner.expect(b'}')?;
    }
    scanner.expect_end()?;

    let missing = |name: &str| -> super::json::ScanError {
        super::json::ScanError {
            at: 0,
            why: format!("{MISSING_KEY}: `{name}`"),
        }
    };
    let moves = moves.ok_or_else(|| missing("moves"))?;
    if moves.is_empty() {
        return Err(super::json::ScanError {
            at: 0,
            why: NO_MOVES.to_string(),
        });
    }
    if !source_seen {
        return Err(missing("source"));
    }
    Ok(Record {
        game_hash: game_hash.ok_or_else(|| missing("game_hash"))?,
        moves,
        winner: winner.ok_or_else(|| missing("winner"))?,
        elo: elo.ok_or_else(|| super::json::ScanError {
            at: 0,
            why: ELO_KEY_REQUIRED.to_string(),
        })?,
    })
}

/// `[[q,r], [q,r], …]` — element zero is `q`. See this module's documentation.
fn read_moves(scanner: &mut Scanner<'_>) -> Result<Vec<Coord>, super::json::ScanError> {
    scanner.expect(b'[')?;
    let mut moves = Vec::new();
    if scanner.accept(b']') {
        return Ok(moves);
    }
    loop {
        scanner.expect(b'[')?;
        let q = coordinate(scanner)?;
        scanner.expect(b',')?;
        let r = coordinate(scanner)?;
        if scanner.accept(b',') {
            return scanner.refuse(BAD_MOVE_ARITY);
        }
        scanner.expect(b']')?;
        moves.push(Coord::new(q, r));
        if !scanner.accept(b',') {
            break;
        }
    }
    scanner.expect(b']')?;
    Ok(moves)
}

/// One axial coordinate, refused rather than truncated if it leaves the lattice.
fn coordinate(scanner: &mut Scanner<'_>) -> Result<i16, super::json::ScanError> {
    let value = scanner.integer()?;
    match i16::try_from(value) {
        Ok(coordinate) => Ok(coordinate),
        Err(_) => scanner.refuse(format!("{COORD_OUT_OF_RANGE}: `{value}`")),
    }
}

/// `[elo, elo]`, each an integer or `null`.
fn read_elo(scanner: &mut Scanner<'_>) -> Result<[Option<u16>; 2], super::json::ScanError> {
    scanner.expect(b'[')?;
    let first = one_elo(scanner)?;
    scanner.expect(b',')?;
    let second = one_elo(scanner)?;
    if scanner.accept(b',') {
        return scanner.refuse(BAD_ELO_ARITY);
    }
    scanner.expect(b']')?;
    Ok([first, second])
}

/// One rating, or `null`.
fn one_elo(scanner: &mut Scanner<'_>) -> Result<Option<u16>, super::json::ScanError> {
    if scanner.accept_null() {
        return Ok(None);
    }
    let value = scanner.integer()?;
    if !(0..=ELO_MAX).contains(&value) {
        return scanner.refuse(format!(
            "{ELO_OUT_OF_RANGE}: `{value}` is not in 0..={ELO_MAX}"
        ));
    }
    Ok(Some(value as u16))
}
