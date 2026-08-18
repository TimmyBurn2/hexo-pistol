//! SplitMix64: the generator's one source of randomness.
//!
//! Reimplemented rather than depended on. A fixture's bytes are pinned, so the
//! number stream under them must not move because someone else's crate changed
//! a constant or its default algorithm — the same argument that keeps
//! pistol-core std-only. What makes a reimplementation safe is an oracle:
//! SplitMix64's reference implementation publishes its stream, and
//! `random_openings_rng_tests.rs` pins this one against it.
//!
//! Every operation is `wrapping_*` on `u64`, so the stream is identical on
//! every target and under `overflow-checks` (docs/decisions.md D-127).

use super::error::RandomOpeningsError;

/// The golden-ratio increment, and the two mixing multipliers.
const GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
/// First mixing constant of the finalizer.
const MIX_1: u64 = 0xBF58_476D_1CE4_E5B9;
/// Second mixing constant of the finalizer.
const MIX_2: u64 = 0x94D0_49BB_1331_11EB;

/// How many rejected words in a row [`SplitMix64::below`] tolerates.
///
/// The rejection zone is `2^64 mod bound` wide, and `bound` is a cell count
/// under thirty thousand, so a single rejection has probability under `2^-49`
/// and this many in a row is not a thing that happens. It exists so that the
/// loop has an exit that is not "eventually" (CLAUDE.md rule 3).
pub const REJECTION_TRIES: usize = 64;

/// How many words at the top of the range must be discarded for the rest to
/// divide evenly by `bound` — that is, `2^64 mod bound`.
///
/// Its own function because it is the one line of arithmetic here that a
/// behavioural test cannot reach: at a 91-cell ball the discarded fraction is
/// under one part in `2^57`, so an off-by-one in it biases nothing any sample
/// could show and every end-to-end test would stay green. Pulled out so it can
/// be compared against a 128-bit oracle directly, which is the only assertion
/// that can actually fail if it is wrong.
///
/// Computed without a 128-bit intermediate: `(2^64 - 1) mod bound` lies in
/// `0..bound`, so adding one lands in `1..=bound`, and the outer modulus maps
/// the single value `bound` back to `0`. `bound` is never zero.
pub fn rejection_remainder(bound: u64) -> u64 {
    ((u64::MAX % bound) + 1) % bound
}

/// A SplitMix64 stream.
#[derive(Debug, Clone)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// The stream this seed names. Every seed is a valid stream, including 0.
    pub fn from_seed(seed: u64) -> SplitMix64 {
        SplitMix64 { state: seed }
    }

    /// The next word.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GAMMA);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(MIX_1);
        z = (z ^ (z >> 27)).wrapping_mul(MIX_2);
        z ^ (z >> 31)
    }

    /// A uniform value in `0..bound`, without modulus bias.
    ///
    /// The plain `next_u64() % bound` is biased toward the low residues by up
    /// to one part in `2^64 / bound`, which is invisible at these sizes and is
    /// still a thumb on the scale of a book that exists to be unbiased. The
    /// words in the last, short block of `bound` are discarded instead, which
    /// makes the accepted set an exact multiple of `bound`.
    ///
    /// `bound` is a cell count and is never zero; a zero would be a bug in the
    /// caller rather than a document this tool was given, so it is an assertion
    /// and not a refusal.
    pub fn below(&mut self, bound: u64) -> Result<u64, RandomOpeningsError> {
        assert!(bound > 0, "a sampling bound is a cell count and is never 0");
        let widest = u64::MAX - rejection_remainder(bound);
        for _ in 0..REJECTION_TRIES {
            let word = self.next_u64();
            if word <= widest {
                return Ok(word % bound);
            }
        }
        Err(RandomOpeningsError::RejectionRunaway {
            bound,
            tries: REJECTION_TRIES,
        })
    }
}
