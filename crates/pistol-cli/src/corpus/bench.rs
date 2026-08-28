use pistol_engine::PositionSpec;

use super::openings::Candidate;
use super::replay::position_after;

/// One band: a stone count to aim at, and how far below it to accept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Band {
    /// The stone count this band is about.
    pub centre: usize,
    /// How far below the centre a position may fall.
    pub width: usize,
}

/// The two bands, in emission order: an opening-ish midgame and a crowded one.
pub const BANDS: [Band; 2] = [
    Band {
        centre: 15,
        width: 2,
    },
    Band {
        centre: 35,
        width: 5,
    },
];

/// How many positions each band contributes.
pub const PER_BAND: usize = 12;

impl Band {
    /// The stone count this band takes from a game of `total` stones, if any.
    ///
    /// Strictly fewer than `total`, so the position is one the game continued
    /// from — and therefore ongoing, since only the last stone of a decisive
    /// game completes a line.
    pub fn count_for(self, total: usize) -> Option<usize> {
        let highest = self.centre.min(total.saturating_sub(1));
        let odd = if highest.is_multiple_of(2) {
            highest.checked_sub(1)?
        } else {
            highest
        };
        (odd + self.width >= self.centre && odd > 0).then_some(odd)
    }

    /// The turn boundary that carries `stones` stones.
    pub fn turn_for(stones: usize) -> usize {
        stones.div_ceil(2)
    }
}

/// One bench position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchPosition {
    /// Which band it belongs to.
    pub centre: usize,
    /// How many stones it actually has — not always the centre, so each entry
    /// states its own.
    pub stones: usize,
    /// The position.
    pub position: PositionSpec,
    /// The game it came from.
    pub game_hash: String,
}

/// Choose the bench positions.
///
/// Bands are taken in order and no game contributes to two of them: twelve games
/// measured at two stone counts measure twelve position shapes twice, while
/// twenty-four games measure twenty-four, at the same cost.
pub fn select(replayed: &[Candidate<'_>]) -> Vec<BenchPosition> {
    let mut ordered: Vec<&Candidate<'_>> = replayed.iter().collect();
    ordered.sort_by(|a, b| a.record.game_hash.cmp(&b.record.game_hash));

    let mut used: Vec<&str> = Vec::new();
    let mut chosen = Vec::new();
    for band in BANDS {
        let mut taken = 0usize;
        for candidate in &ordered {
            if taken == PER_BAND {
                break;
            }
            if used.contains(&candidate.record.game_hash.as_str()) {
                continue;
            }
            let Some(stones) = band.count_for(candidate.record.moves.len()) else {
                continue;
            };
            let turns_wanted = Band::turn_for(stones);
            if candidate.turns.len() <= turns_wanted {
                continue;
            }
            let state = position_after(candidate.turns, turns_wanted);
            debug_assert_eq!(
                state.board().stones().count(),
                stones,
                "the band's arithmetic and the replay disagree about the stone count"
            );
            chosen.push(BenchPosition {
                centre: band.centre,
                stones,
                position: PositionSpec::Start {
                    moves: candidate
                        .turns
                        .iter()
                        .take(turns_wanted)
                        .map(|grouped| grouped.turn)
                        .collect(),
                },
                game_hash: candidate.record.game_hash.clone(),
            });
            used.push(&candidate.record.game_hash);
            taken += 1;
        }
    }
    chosen
}
