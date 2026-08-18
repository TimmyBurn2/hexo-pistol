//! The generator's randomness, pinned against a published stream.
//!
//! A book is only reproducible if its number source is. This one is SplitMix64,
//! reimplemented here rather than taken as a dependency for the reason
//! pistol-core is std-only: a fixture's bytes must not move because someone
//! else's crate changed a constant. What keeps a reimplementation honest is an
//! oracle, and SplitMix64 has one — Vigna's reference `splitmix64.c`, whose
//! stream from state 0 is a published sequence.

use pistol_cli::random_openings::rng::{SplitMix64, rejection_remainder};

/// The first four outputs of the reference implementation seeded with 0.
const REFERENCE_FROM_ZERO: [u64; 4] = [
    0xE220_A839_7B1D_CDAF,
    0x6E78_9E6A_A1B9_65F4,
    0x06C4_5D18_8009_454F,
    0xF88B_B8A8_724C_81EC,
];

#[test]
fn splitmix64_reproduces_the_published_stream() {
    let mut rng = SplitMix64::from_seed(0);
    let drawn: Vec<u64> = (0..REFERENCE_FROM_ZERO.len())
        .map(|_| rng.next_u64())
        .collect();
    assert_eq!(
        drawn,
        REFERENCE_FROM_ZERO.to_vec(),
        "this is SplitMix64 and not something near it"
    );
}

#[test]
fn splitmix64_below_stays_in_range_and_covers_it() {
    // `below` is the only way the generator turns a word into a cell index, so
    // two things have to hold: it never returns the bound, and it can return
    // every value under it. A sampler that could not reach the last cell would
    // bias every book by a ring of the lattice.
    let mut rng = SplitMix64::from_seed(1);
    let bound = 7;
    let mut seen = [false; 7];
    for _ in 0..1000 {
        let drawn = rng
            .below(bound)
            .expect("a bound this small never rejects for long");
        assert!(drawn < bound, "{drawn} is under {bound}");
        seen[drawn as usize] = true;
    }
    assert!(
        seen.iter().all(|hit| *hit),
        "every index under {bound} occurs"
    );
}

#[test]
fn splitmix64_below_one_is_always_zero() {
    // The degenerate bound, which the rejection arithmetic has to survive: the
    // modulus is 1, the rejection zone is empty, and every draw is accepted.
    let mut rng = SplitMix64::from_seed(2);
    for _ in 0..16 {
        assert_eq!(rng.below(1).expect("bound 1 accepts every word"), 0);
    }
}

#[test]
fn splitmix64_below_discards_the_words_that_would_bias_it() {
    // The rejection branch, exercised rather than reasoned about. At this bound
    // the accepted set is `0..bound` and everything above is discarded, so
    // roughly half of all words are — where at a 91-cell ball the discarded
    // fraction is under one part in `2^57` and no test could ever reach it.
    //
    // What is pinned is the semantics: `below` returns the FIRST word under the
    // bound, and the words it skipped are gone from the stream. A biased
    // implementation (`next_u64() % bound`) would return the first word folded
    // instead, which for a rejected word is a different number.
    let bound = (1u64 << 63) + 1;
    let seed = 2;
    let mut raw = SplitMix64::from_seed(seed);
    let mut skipped = 0;
    let first_accepted = loop {
        let word = raw.next_u64();
        if word < bound {
            break word;
        }
        skipped += 1;
    };
    assert_eq!(
        skipped, 4,
        "seed {seed} opens with four words this bound rejects"
    );

    let mut rng = SplitMix64::from_seed(seed);
    assert_eq!(
        rng.below(bound).expect("a word under the bound arrives"),
        first_accepted,
        "the rejected words are discarded, not folded"
    );
}

#[test]
fn splitmix64_rejection_remainder_matches_a_wider_oracle() {
    // The one line of the sampler no behavioural test can reach. An off-by-one
    // here shifts one index by one word in 2^64 — a real bias, and invisible to
    // every sample this tool will ever draw, so it is checked against arithmetic
    // done a different way instead of against an outcome. `u128` is the oracle
    // precisely because the function under test exists to avoid it.
    for bound in [
        1,
        2,
        3,
        7,
        90,
        91,
        12481,
        u64::from(u32::MAX),
        (1u64 << 63) + 1,
        u64::MAX - 1,
        u64::MAX,
    ] {
        let oracle = ((1u128 << 64) % u128::from(bound)) as u64;
        assert_eq!(
            rejection_remainder(bound),
            oracle,
            "2^64 mod {bound} is {oracle}"
        );
        // What the remainder is FOR: the accepted words divide evenly by the
        // bound, so every residue gets the same number of them.
        let accepted = (1u128 << 64) - u128::from(oracle);
        assert_eq!(
            accepted % u128::from(bound),
            0,
            "the accepted range under {bound} is an exact multiple of it"
        );
    }
}
