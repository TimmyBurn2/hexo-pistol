//! Choosing the openings: who is eligible, which position is which, and the
//! order they come out in.
//!
//! Every number here is a pre-registered extraction parameter, stated as a named
//! constant and echoed into the fixture header, so a file says what produced it
//! (CLAUDE.md rule 7). None of them is a tunable with a code-side default: there
//! is nothing to override, because there is no config that reaches this tool.
//!
//! # The three rules, and what each is for
//!
//! - **The mismatch ceiling** ([`ELO_GAP_CEILING`]). Measured on the corpus, the
//!   higher-rated player scores .532, .558 and .609 in the gap buckets below
//!   100, then .675, .747, .860 and .941 above it. Below the ceiling an opening
//!   is a contest both sides contributed to; above it one side was outclassed,
//!   and half of the opening is that player's mistake (docs/decisions.md D-142).
//! - **The rating floor** — the lower median of `min(elo)` over the candidates,
//!   computed rather than asserted, because a quantile travels to another corpus
//!   and a hard-coded rating does not. Lower median means the element at index
//!   `(n - 1) / 2` of the ascending sort: an integer, no averaging, no rounding
//!   rule to get wrong (CLAUDE.md rule 4).
//! - **The canonical form** ([`pistol_core::canonical_form`]). Two openings that
//!   are the same shape reflected are one opening; deduplicating by position
//!   identity alone keeps both, and CLAUDE.md rule 6 counts distinct games
//!   (docs/decisions.md D-137).
//!
//! # Order, and why it is not the priority rule
//!
//! Within a class the representative is the highest `min(elo)`, then the lowest
//! `game_hash` — a total order, so the representative is deterministic. But the
//! file is *emitted* in `game_hash` order, not in that priority order, because
//! there is no cap: a runner takes a prefix, and a prefix of a rating-sorted
//! list is the extreme tail rather than a sample of the pool (D-143).

use std::collections::BTreeMap;

use pistol_core::{Coord, GameState, Key128, Player, canonical_form};
use pistol_engine::PositionSpec;

use super::record::Record;
use super::replay::position_after;
use super::verdict::GroupedTurn;

/// How many turns of a game an opening is.
///
/// Four, so both sides have had two turns: at three, P1 has moved twice and P2
/// once, and the position is P1's shape with one reply. Supply is monotone in K
/// — prefixes are nested — so a longer prefix is never scarcer; what a longer
/// one costs is neutrality, since it encodes more of one human plan
/// (docs/decisions.md D-138).
pub const K_TURNS: usize = 4;

/// The widest rating gap an opening's game may have. See this module's docs.
pub const ELO_GAP_CEILING: u16 = 100;

/// Stones on the board after [`K_TURNS`] turns: one on turn 1, two thereafter.
pub const OPENING_STONES: usize = 2 * K_TURNS - 1;

/// Named invariant: two different positions share a zobrist key.
pub const KEY_COLLISION: &str = "KEY_COLLISION";

/// Named refusal: nothing in the corpus could be an opening.
///
/// Reachable from a schema-valid corpus — one where every game is unrated, or
/// every game's ratings are further apart than [`ELO_GAP_CEILING`], or which
/// holds no game long enough to have an opening — so it is a refusal and not a
/// broken invariant (CLAUDE.md rule 3). Returning a floor of zero instead would
/// emit the whole corpus unfiltered, which is the silent fallback that rule
/// forbids.
pub const NO_CANDIDATE_GAMES: &str = "no game in this corpus can be an opening: openings are rated, are drawn from games whose \
     ratings are close, and must be positions the game continued from";

/// One opening, and the evidence the corpus carries about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opening {
    /// The position, as the canonical move-list encoding (docs/decisions.md D-6).
    pub position: PositionSpec,
    /// The game this spelling came from.
    pub game_hash: String,
    /// The lower of that game's two ratings.
    pub min_elo: u16,
    /// How many corpus games reach this opening up to a symmetry.
    pub class_games: usize,
    /// How many of those the first player won. Together with `class_games` this
    /// is the corpus's own answer to how balanced the opening is — which
    /// WP-1.2a is charged with and cannot answer from the engine without
    /// putting the engine under test into its own book (docs/decisions.md D-145).
    pub class_p1_wins: usize,
}

/// A game that replayed, with what the selection needs to read from it.
pub struct Candidate<'a> {
    /// The corpus line.
    pub record: &'a Record,
    /// Its turns, as replayed.
    pub turns: &'a [GroupedTurn],
}

impl Candidate<'_> {
    /// Whether the game is long enough for the opening to be a position it
    /// continued from, rather than the whole game.
    fn has_opening(&self) -> bool {
        self.record.moves.len() > OPENING_STONES && self.turns.len() > K_TURNS
    }
}

/// The lower median of a sorted-in-place list of ratings, or `None` for an
/// empty list — which is a question about the corpus, not an invariant.
pub fn lower_median(values: &mut [u16]) -> Option<u16> {
    if values.is_empty() {
        return None;
    }
    values.sort_unstable();
    Some(values[(values.len() - 1) / 2])
}

/// What the selection decided, with the numbers a header has to state.
#[derive(Debug, Clone)]
pub struct Selection {
    /// The openings, in emission order.
    pub openings: Vec<Opening>,
    /// Games that met every clause but the rating floor.
    pub candidates: usize,
    /// The computed rating floor.
    pub floor: u16,
    /// Games at or above the floor.
    pub eligible: usize,
    /// Distinct positions before the symmetry collapse.
    pub distinct_positions: usize,
}

/// Choose the openings.
///
/// `replayed` is every game that replayed eligibly, paired with its turns. The
/// balance evidence is counted over all of them — an opening's outcome record is
/// a fact about the opening, and narrowing it to the rated band would throw away
/// evidence without making it truer.
pub fn select(replayed: &[Candidate<'_>]) -> Result<Selection, &'static str> {
    let mut class_games: BTreeMap<Vec<(Coord, Player)>, (usize, usize)> = BTreeMap::new();
    for candidate in replayed {
        if !candidate.has_opening() {
            continue;
        }
        let state = position_after(candidate.turns, K_TURNS);
        let entry = class_games.entry(canonical_of(&state)).or_insert((0, 0));
        entry.0 += 1;
        if candidate.record.winner == Player::P1 {
            entry.1 += 1;
        }
    }

    let candidates: Vec<&Candidate<'_>> = replayed
        .iter()
        .filter(|candidate| {
            candidate.has_opening()
                && candidate.record.min_elo().is_some()
                && candidate
                    .record
                    .elo_gap()
                    .is_some_and(|gap| gap <= ELO_GAP_CEILING)
        })
        .collect();
    let mut ratings: Vec<u16> = candidates
        .iter()
        .map(|candidate| {
            candidate
                .record
                .min_elo()
                .unwrap_or_else(|| unreachable!("filtered to games with both ratings"))
        })
        .collect();
    let Some(floor) = lower_median(&mut ratings) else {
        return Err(NO_CANDIDATE_GAMES);
    };

    // Deterministic throughout: a BTreeMap keyed on the canonical stone list,
    // which is the identity of record here. The zobrist key is carried beside it
    // only as a cross-check that two spellings of one position agree.
    let mut keys: BTreeMap<Key128, Vec<(Coord, Player)>> = BTreeMap::new();
    let mut classes: BTreeMap<Vec<(Coord, Player)>, &Candidate<'_>> = BTreeMap::new();
    let mut distinct: BTreeMap<Vec<(Coord, Player)>, ()> = BTreeMap::new();
    let mut eligible = 0usize;

    for candidate in candidates {
        let rating = candidate
            .record
            .min_elo()
            .unwrap_or_else(|| unreachable!("filtered to games with both ratings"));
        if rating < floor {
            continue;
        }
        eligible += 1;
        let state = position_after(candidate.turns, K_TURNS);
        let stones = sorted_stones(&state);
        if let Some(seen) = keys.insert(state.key(), stones.clone())
            && seen != stones
        {
            panic!(
                "pistol-cli invariant {KEY_COLLISION}: two different positions share the zobrist \
                 key {:?}; game {} is one of them",
                state.key(),
                candidate.record.game_hash
            );
        }
        distinct.insert(stones, ());
        let canonical = canonical_of(&state);
        classes
            .entry(canonical)
            .and_modify(|held| {
                if better_representative(candidate, held) {
                    *held = candidate;
                }
            })
            .or_insert(candidate);
    }

    let mut openings: Vec<Opening> = classes
        .into_iter()
        .map(|(canonical, candidate)| {
            let (games, p1) = class_games.get(&canonical).copied().unwrap_or((0, 0));
            Opening {
                position: PositionSpec::Start {
                    moves: candidate
                        .turns
                        .iter()
                        .take(K_TURNS)
                        .map(|grouped| grouped.turn)
                        .collect(),
                },
                game_hash: candidate.record.game_hash.clone(),
                min_elo: candidate
                    .record
                    .min_elo()
                    .unwrap_or_else(|| unreachable!("filtered to games with both ratings")),
                class_games: games,
                class_p1_wins: p1,
            }
        })
        .collect();
    openings.sort_by(|a, b| a.game_hash.cmp(&b.game_hash));

    Ok(Selection {
        openings,
        candidates: ratings.len(),
        floor,
        eligible,
        distinct_positions: distinct.len(),
    })
}

/// Whether `contender` should represent a class instead of `held`: higher
/// rating first, then the lower game hash. A total order over distinct games,
/// because game hashes are unique — the reader refuses a repeat.
fn better_representative(contender: &Candidate<'_>, held: &Candidate<'_>) -> bool {
    let key = |candidate: &Candidate<'_>| {
        (
            std::cmp::Reverse(candidate.record.min_elo()),
            candidate.record.game_hash.clone(),
        )
    };
    key(contender) < key(held)
}

/// The stones of a position, ordered — the identity a position has here.
fn sorted_stones(state: &GameState) -> Vec<(Coord, Player)> {
    let mut stones: Vec<(Coord, Player)> = state.board().stones().collect();
    stones.sort_unstable();
    stones
}

/// The canonical spelling of a position, up to the twelve lattice symmetries.
fn canonical_of(state: &GameState) -> Vec<(Coord, Player)> {
    canonical_form(&sorted_stones(state))
}
