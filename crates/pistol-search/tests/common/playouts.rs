use pistol_core::{Coord, GameState, PlyOutcome};

/// How far from an existing stone a playout puts the next one. Well inside the
/// rules' radius-8 region, which is not this number's business: the region is
/// pistol-core's and `is_legal_placement` remains the authority on every sample
/// (CLAUDE.md rule 2, docs/decisions.md D-20).
const SPREAD: u32 = 2;

/// Samples a ply is given before the generator admits it cannot place one.
const SAMPLE_ATTEMPTS: usize = 512;

/// A toy PRNG: xorshift64\*, seeded by the caller.
pub struct Rng {
    state: u64,
}

impl Rng {
    /// A generator for `seed`. Xorshift needs a non-zero state, so zero is
    /// nudged rather than refused.
    pub fn new(seed: u64) -> Rng {
        Rng { state: seed | 1 }
    }

    fn next_u64(&mut self) -> u64 {
        let mut state = self.state;
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        self.state = state;
        state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, bound: usize) -> usize {
        assert!(bound > 0, "below(0) has no answer");
        (self.next_u64() % bound as u64) as usize
    }
}

/// A position `turns` turns into a game played from `seed`.
///
/// Every ply that would complete a line is rejected and resampled, so the
/// position is ongoing and the generator cannot hand back something the search
/// refuses. It stops on a turn boundary, which is where a search may start
/// (docs/decisions.md D-71).
///
/// # Panics
///
/// If no legal cell turns up in [`SAMPLE_ATTEMPTS`] samples, which on a board
/// this sparse means the generator is broken rather than the board full.
pub fn playout(seed: u64, turns: u32) -> GameState {
    let mut rng = Rng::new(seed);
    let mut state = GameState::new_game();
    let offsets = ball_offsets();

    while state.turn() <= turns {
        let at = sample(&state, &mut rng, &offsets);
        match state.place(at) {
            Ok(PlyOutcome::Win { .. }) => {
                // A win ends the game, and this generator is for ongoing
                // positions: take it back and try elsewhere.
                state.undo().expect("the stone was just placed");
            }
            Ok(_) => {}
            Err(error) => panic!("playout {seed}: the sampler offered {at}: {error}"),
        }
    }
    state
}

/// A cell the mover may place a stone on, sampled near an existing stone.
fn sample(state: &GameState, rng: &mut Rng, offsets: &[Coord]) -> Coord {
    let board = state.board();
    if board.is_empty() {
        // Rule 3 leaves exactly one choice, and it is not near anything.
        return Coord::ORIGIN;
    }
    for _ in 0..SAMPLE_ATTEMPTS {
        let (stone, _) = board
            .stones()
            .nth(rng.below(board.stone_count()))
            .expect("the index is below the stone count");
        let Some(cell) = stone.checked_offset(offsets[rng.below(offsets.len())]) else {
            continue;
        };
        if board.is_legal_placement(cell) {
            return cell;
        }
    }
    panic!(
        "no legal cell in {SAMPLE_ATTEMPTS} samples around {} stones",
        board.stone_count()
    );
}

/// The offsets of a radius-[`SPREAD`] ball, centre included.
fn ball_offsets() -> Vec<Coord> {
    let reach = i16::try_from(SPREAD).expect("the spread fits in a coordinate");
    let mut offsets = Vec::new();
    for dq in -reach..=reach {
        for dr in -reach..=reach {
            let delta = Coord::new(dq, dr);
            if Coord::ORIGIN.distance(delta) <= SPREAD {
                offsets.push(delta);
            }
        }
    }
    offsets
}
