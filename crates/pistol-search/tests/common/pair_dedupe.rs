use pistol_core::Coord;

use super::reference::PairOrder;
use super::reference_invariants::REFERENCE_DEDUPE_KEY_UNSORTED;

/// The first stones at ONE node whose own pair enumeration has already valued
/// every pair they belong to. This is the whole of the dedupe.
///
/// Why membership is the right test, both ways. If `second` is in here, its ply
/// CONTINUED the turn and it walked every cell its own board offered — `first`
/// among them, since the candidate set only grows as stones are added, so a
/// phase-0 cell is still a candidate one stone later; that pair is valued. If
/// `second` is NOT in here, no ordering of the pair has been walked: it is later
/// in the ascending order, or a cell only `first` opened up, or its ply WON and
/// made a one-stone turn under rule 4 — the case D-120 named, the pair whose
/// smaller cell wins being realised by one ordering only, and realised here.
///
/// `candidate_cells` is ascending and distinct, so the ledger is bisected.
pub struct Paired {
    pairs: PairOrder,
    cells: Vec<Coord>,
}

impl Paired {
    pub fn new(pairs: PairOrder) -> Paired {
        Paired {
            pairs,
            cells: Vec::new(),
        }
    }

    /// Whether the pair this cell would complete has already been valued.
    pub fn holds(&self, second: Coord) -> bool {
        self.pairs == PairOrder::Deduped && self.cells.binary_search(&second).is_ok()
    }

    /// Record that `first` has now walked every pair it belongs to.
    pub fn push(&mut self, first: Coord) {
        assert!(
            self.cells.last().is_none_or(|&last| last < first),
            "pistol-search reference invariant {REFERENCE_DEDUPE_KEY_UNSORTED}: the candidate \
             set is ascending and distinct, and {first} does not follow what came before it"
        );
        self.cells.push(first);
    }
}
