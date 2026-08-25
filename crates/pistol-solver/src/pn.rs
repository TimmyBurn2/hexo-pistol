//! Proof and disproof numbers: the INF sentinel, the 1+ε loosening, and the
//! three-valued reading of a (pn, dn) pair.
//!
//! The threshold FORMULAS that consume these numbers live in `dfpn`; this
//! module owns only the arithmetic they are safe over, and the safety is the
//! point (docs/experiments/wp18a_design.md §4):
//!
//! - `INF` is `1 << 62`, far from `u64::MAX`, so no saturating sum and no
//!   ε-multiple of a sentinel value can wrap;
//! - every addition is saturating, so `INF + x = INF` and `INF + INF = INF`
//!   by construction rather than by hope;
//! - the values are unsigned, so no negative exists to produce;
//! - the one subtraction the formulas perform (`dt − d`, `pt − p`) is argued
//!   where it is performed, in `dfpn`: it runs only after the caller
//!   confirmed `p < pt ∧ d < dt`, which makes `dt − d ≥ 1`.
//!
//! `Epsilon::loosen` is Pawlewicz & Lew's ⌈x(1+ε)⌉ (CG 2006, §3.2), computed
//! as an exact rational ceiling in `u128`. It is applied to THRESHOLDS only;
//! the stored pn/dn never pass through it.

/// The saturating sentinel for "no finite proof/disproof number".
///
/// Chosen so that `INF + INF` and `INF` scaled by the largest ε the config
/// admits all fit `u64`: the config validates `ε ≤ 2`, and
/// `INF · 3 < u64::MAX`.
pub const INF: u64 = 1 << 62;

/// A positive rational ε, config-carried, applied to thresholds only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Epsilon {
    /// The numerator. Strictly positive: ε = 0 would silently disable the
    /// trick rather than refuse, so the config rejects it.
    num: u32,
    /// The denominator. Strictly positive.
    den: u32,
}

impl Epsilon {
    /// ε = num/den, or `None` when the pair is not a strictly positive
    /// rational the sentinel arithmetic is safe over (`num`/`den` ≤ 0,
    /// `den` zero, or `num > 2·den`, the bound that keeps `loosen(INF)`
    /// inside `u64`).
    pub fn new(num: u32, den: u32) -> Option<Epsilon> {
        if den == 0 || num == 0 || num > 2 * den {
            return None;
        }
        Some(Epsilon { num, den })
    }

    /// ⌈x·(1 + ε)⌉, Pawlewicz & Lew §3.2's loosened constraint.
    ///
    /// Exact: the rational ceiling is taken in `u128`, so no intermediate
    /// rounds and no float appears. For `x` at or above `INF` the answer is a
    /// threshold no value can reach, which is the correct reading of "no
    /// bound" — it is NOT clamped to `INF`, because a value of `INF` would
    /// make a child stop at the sentinel rather than run to a definitive
    /// answer.
    pub fn loosen(&self, x: u64) -> u64 {
        let scaled = (u128::from(x) * (u128::from(self.num) + u128::from(self.den)))
            .div_ceil(u128::from(self.den));
        u64::try_from(scaled).unwrap_or(u64::MAX)
    }
}

/// What a (pn, dn) pair says about a node.
///
/// `pn == 0` is proven, `dn == 0` is disproven, and the pair `(0, 0)` is not a
/// state any conforming producer can emit: every producer sets the other
/// number to `INF` when it zeroes one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    /// Proven: pn is 0, dn is `INF`.
    Proven,
    /// Disproven: dn is 0, pn is `INF`.
    Disproven,
    /// Neither: both numbers are strictly positive.
    Unknown,
}

/// The three-valued reading of a pair.
///
/// # Panics
///
/// On `(0, 0)`, which no conforming producer emits and which this function
/// exists to refuse rather than arbitrate.
pub fn value_of(pn: u64, dn: u64) -> Value {
    match (pn, dn) {
        (0, 0) => panic!("pistol-solver invariant SOLVER_ZERO_PAIR: pn and dn are both zero"),
        (0, _) => Value::Proven,
        (_, 0) => Value::Disproven,
        _ => Value::Unknown,
    }
}

/// Saturating summation, the only addition pn/dn ever receive: the cap is
/// `INF`, not `u64::MAX`, so `INF + x = INF` holds for EVERY x and the
/// sentinel stays comparable after summation.
pub fn saturating_sum(values: impl IntoIterator<Item = u64>) -> u64 {
    values
        .into_iter()
        .fold(0u64, |acc, x| acc.saturating_add(x).min(INF))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inf_absorbs_everything_it_is_added_to() {
        assert_eq!(saturating_sum([INF, 1]), INF);
        assert_eq!(saturating_sum([INF, INF, INF]), INF);
        assert_eq!(saturating_sum([INF, u64::MAX]), INF);
        assert_eq!(saturating_sum([u64::MAX / 2, u64::MAX / 2]), INF);
    }

    #[test]
    fn inf_sits_far_from_the_wrap_edge() {
        // The margin the config's ε bound and the threshold formulas spend.
        const _: () = {
            assert!(INF + INF < u64::MAX);
            assert!(INF * 3 < u64::MAX);
        };
    }

    #[test]
    fn values_read_the_zeroes() {
        assert_eq!(value_of(0, INF), Value::Proven);
        assert_eq!(value_of(INF, 0), Value::Disproven);
        assert_eq!(value_of(1, 1), Value::Unknown);
        assert_eq!(value_of(3, INF), Value::Unknown);
    }

    #[test]
    #[should_panic(expected = "SOLVER_ZERO_PAIR")]
    fn the_zero_pair_is_refused_loudly() {
        value_of(0, 0);
    }

    #[test]
    fn epsilon_is_an_exact_rational_ceiling() {
        let quarter = Epsilon::new(1, 4).unwrap();
        // ⌈4·(5/4)⌉ = 5 exactly.
        assert_eq!(quarter.loosen(4), 5);
        // ⌈1·(5/4)⌉ = 2, so even a value of 1 loosens to a real bound.
        assert_eq!(quarter.loosen(1), 2);
        // ⌈5·(5/4)⌉ = ⌈6.25⌉ = 7.
        assert_eq!(quarter.loosen(5), 7);
        let half = Epsilon::new(1, 2).unwrap();
        assert_eq!(half.loosen(3), 5);
        assert_eq!(half.loosen(4), 6);
    }

    #[test]
    fn epsilon_never_tightens_and_never_wraps() {
        for den in 1..=6u32 {
            for num in 1..=(2 * den) {
                let Some(eps) = Epsilon::new(num, den) else {
                    continue;
                };
                for x in [1u64, 2, 3, 7, 1000, 1 << 40, INF - 1, INF] {
                    let loosened = eps.loosen(x);
                    assert!(loosened > x, "loosening must loosen: {x} -> {loosened}");
                }
                // The sentinel scales inside u64 for every admitted ε.
                assert!(eps.loosen(INF) > INF);
                assert!(eps.loosen(INF) < u64::MAX);
            }
        }
    }

    #[test]
    fn epsilon_refuses_its_invalid_forms() {
        assert!(Epsilon::new(0, 4).is_none());
        assert!(Epsilon::new(1, 0).is_none());
        assert!(Epsilon::new(7, 2).is_none());
        assert!(Epsilon::new(2, 1).is_some());
    }
}
