//! Deterministic random playouts, for the test that checks the incremental
//! threat state against a from-scratch reference.
//!
//! Incremental state is only as good as the last position it was carried
//! through, so the test that checks it wants many *arbitrary* positions rather
//! than a handful of chosen ones. Arbitrary, not random: the seed is given by
//! the caller and the generator is written out here, so a failure names a
//! playout a later run reproduces exactly (CLAUDE.md rule 4).
//!
//! This is the same generator pistol-core's and pistol-eval's test trees use on
//! the zobrist key and the incremental eval, restated here rather than shared
//! because a test tree is per crate and this one takes no dev-dependency.
//! Nothing it produces reaches a value the engine plays on.
//!
//! THE SAMPLING SCHEDULE IS PART OF THE ORACLE'S REGISTRATION and is stated at
//! the one caller: ONE [`random_ply`] draw per ply, and that draw is the ply.
//! A second draw advances this generator and changes the trajectory, which
//! moves the oracle's coverage enough to fail its own floors on a correct
//! implementation (docs/decisions.md D-256).

use std::sync::OnceLock;

use pistol_core::{Board, Coord, LEGAL_RADIUS};

/// How many samples a ply is given before the sampler admits it cannot find a
/// cell. A ball holds 217 cells and a playout puts a few hundred stones on the
/// board, so this is never reached; it is here so that a saturated board is a
/// loud failure rather than a hang (CLAUDE.md rule 3).
const SAMPLE_ATTEMPTS: usize = 256;

/// A toy PRNG: xorshift64\*, seeded by the caller.
pub struct Rng {
    state: u64,
}

impl Rng {
    /// A generator for `seed`. Any seed is accepted; xorshift needs a non-zero
    /// state, so zero is nudged rather than refused.
    pub fn new(seed: u64) -> Rng {
        Rng { state: seed | 1 }
    }

    /// The next 64 bits.
    pub fn next_u64(&mut self) -> u64 {
        let mut state = self.state;
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        self.state = state;
        state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A number below `bound`, which must not be zero.
    pub fn below(&mut self, bound: usize) -> usize {
        assert!(bound > 0, "below(0) has no answer");
        (self.next_u64() % bound as u64) as usize
    }
}

/// A cell the mover may place a stone on, drawn from the legal region.
///
/// The cell is sampled — a random stone, a random offset within
/// [`LEGAL_RADIUS`] of it — rather than taken from an enumeration of the whole
/// region, which is quadratic in the stones on the board.
/// `is_legal_placement` is still the authority on whether the sample is
/// playable; this only proposes.
///
/// # Panics
///
/// If no legal cell turns up in [`SAMPLE_ATTEMPTS`] samples.
pub fn random_ply(board: &Board, rng: &mut Rng) -> Coord {
    // An empty board has one legal cell, and it is not near any stone (rule 3).
    if board.is_empty() {
        return Coord::ORIGIN;
    }
    let offsets = ball_offsets();
    for _ in 0..SAMPLE_ATTEMPTS {
        let index = rng.below(board.stone_count());
        let (stone, _) = board
            .stones()
            .nth(index)
            .expect("the index is below the stone count");
        let delta = offsets[rng.below(offsets.len())];
        let Some(cell) = stone.checked_offset(delta) else {
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

/// The offsets of a radius-[`LEGAL_RADIUS`] ball, built once.
fn ball_offsets() -> &'static [Coord] {
    static OFFSETS: OnceLock<Vec<Coord>> = OnceLock::new();
    OFFSETS.get_or_init(|| {
        let radius = i16::try_from(LEGAL_RADIUS).expect("the rule radius fits in a coordinate");
        let mut offsets = Vec::new();
        for dq in -radius..=radius {
            for dr in -radius..=radius {
                let delta = Coord::new(dq, dr);
                if Coord::ORIGIN.distance(delta) <= LEGAL_RADIUS {
                    offsets.push(delta);
                }
            }
        }
        offsets
    })
}
