use pistol_core::{Coord, Player};

/// The largest magnitude a static evaluation may report.
///
/// Above this sits the mate band: `MATE = 30_000` and `MateIn(t) = MATE - t`,
/// scored in turns (docs/decisions.md D-3). A win is the search's to report —
/// it knows the distance — so eval saturates here rather than ever crossing
/// into that range. Keeping the two bands apart is what lets a search compare a
/// static value against a mate score without a special case.
pub const EVAL_MAX: i32 = 16_000;

/// A position evaluator, updated one stone at a time.
///
/// # Contract
///
/// - [`Eval::apply`] and [`Eval::undo`] are called by whoever moves the stone,
///   at the same seam the board and the zobrist key are updated
///   (docs/decisions.md D-41). The player travels with the cell so that an
///   implementation never has to consult a board — that is the whole reason,
///   and NOT so that it can cross-check what it is told against what it already
///   holds: this crate's checks are opportunistic and cannot see a caller that
///   mispairs an apply with an undo, so pairing them is the caller's invariant
///   (docs/decisions.md D-70, and D-108 which withdrew the cross-check clause
///   this sentence used to make).
/// - The two are inverses, and the value depends only on the *set* of stones
///   applied — never on the order they arrived in, nor on the order they are
///   taken back in. A search relies on the first half; the second half is what
///   makes the first cheap to be sure of.
/// - [`Eval::value`] is **side-relative**: positive means good for
///   `side_to_move`, so a negamax search negates a child's value instead of
///   tracking whose number it is. Its magnitude never exceeds [`EVAL_MAX`].
/// - Integer arithmetic only, no interior randomness, no dependence on
///   iteration order: two runs of the same position give the same number, on
///   any machine (CLAUDE.md rule 4).
/// - Neither call fails. A cell that may not hold a stone is refused by
///   `pistol_core::GameState` long before an eval hears about it; being told
///   something impossible anyway is a broken invariant in pistol, and an
///   implementation panics with a named token rather than returning an error
///   nobody could handle (CLAUDE.md rule 3).
///
/// The trait is object safe on purpose: the engine picks a backend from config
/// at construction, and `Box<dyn Eval>` is the cheapest way to hold that choice
/// without making every search type generic over it.
pub trait Eval {
    /// Account for a stone of `player` arriving on `at`.
    fn apply(&mut self, at: Coord, player: Player);

    /// Account for the stone of `player` on `at` being taken back.
    fn undo(&mut self, at: Coord, player: Player);

    /// What the position is worth to `side_to_move`, in `-EVAL_MAX..=EVAL_MAX`.
    fn value(&self, side_to_move: Player) -> i32;

    /// What the position would be worth **to `player`** with a hypothetical
    /// stone of theirs on `at` — leaving the eval indistinguishable from
    /// before the call.
    ///
    /// Despite the name, this is NOT a difference: it answers exactly what
    /// the D-76 roundtrip answers, the *value after* the stone, because that
    /// is the number move ordering sorts by and the equation the oracle test
    /// pins — `delta(c, p) == apply(c, p); value(p); undo(c, p)` — is the
    /// operative contract (docs/decisions.md D-110, whose prose "score
    /// change" is overruled by its own equation; D-214 records the
    /// amendment).
    ///
    /// # Contract
    ///
    /// - The default body IS that roundtrip, so a backend that does not
    ///   override this method computes precisely what the search computed
    ///   before the method existed — a performance path, never a correctness
    ///   fork (D-110). An override ships with a test asserting the equation
    ///   over seeded playouts plus adversarial cases; it may disagree with
    ///   the default on NOTHING, panics included.
    /// - "Indistinguishable" means OBSERVATIONAL equivalence through
    ///   [`Eval::apply`]/[`Eval::undo`]/[`Eval::value`]; a backend whose
    ///   whole state is comparable pins it as equality (D-214).
    /// - The receiver is `&mut self` because the default body applies and
    ///   takes back a stone; that amends D-110's `&self` parenthetical,
    ///   which is unsatisfiable beside the roundtrip default it mandates.
    ///   Object safety — what the parenthetical was guarding — is unaffected.
    /// - Being told an impossible stone (a window already full) is the same
    ///   broken invariant it is for [`Eval::apply`], and both paths panic
    ///   with the same named token. The post-panic state may differ between
    ///   the default and an override — the default panics mid-mutation, a
    ///   read-only override with state untouched — which is observable only
    ///   under `catch_unwind`, where an eval must never be reused.
    fn delta(&mut self, at: Coord, player: Player) -> i32 {
        self.apply(at, player);
        let value = self.value(player);
        self.undo(at, player);
        value
    }
}
