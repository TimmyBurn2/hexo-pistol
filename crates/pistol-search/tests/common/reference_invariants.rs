//! What the reference asserts about ITSELF, by name.
//!
//! Seven statements the walk makes about its own workings, split out because
//! they are one concept read from three modules and because the inventory below
//! — which of them can actually fire — is the part a reader needs and the part
//! that goes stale (CLAUDE.md rule 9's soft cap on [`super::reference`] is what
//! forced the question; being one concept is why the answer is a module).
//!
//! The naming convention is `pvs.rs`'s: a violated assumption names itself, so a
//! failure says which assumption rather than which line.

/// Named invariant: a horizon landed half way through a turn, where no static
/// value is an answer (docs/decisions.md D-111).
pub const REFERENCE_HORIZON_MID_TURN: &str = "REFERENCE_HORIZON_MID_TURN";

/// Named invariant: the candidate policy offered a cell the rules refuse.
pub const REFERENCE_CANDIDATE_ILLEGAL: &str = "REFERENCE_CANDIDATE_ILLEGAL";

/// Named invariant: the policy offered nothing for the mover's second stone
/// (docs/decisions.md D-104).
pub const REFERENCE_NO_CANDIDATES_MID_TURN: &str = "REFERENCE_NO_CANDIDATES_MID_TURN";

/// Named invariant: a turn asked for a third stone.
pub const REFERENCE_TURN_OWES_A_THIRD_STONE: &str = "REFERENCE_TURN_OWES_A_THIRD_STONE";

/// Named invariant: the two orderings of one pair valued it differently.
pub const REFERENCE_PAIR_ORDER_DISAGREES: &str = "REFERENCE_PAIR_ORDER_DISAGREES";

/// Named invariant: the dedupe ledger was fed out of the ascending, distinct
/// order `candidate_cells` promises, which is what it bisects on.
pub const REFERENCE_DEDUPE_KEY_UNSORTED: &str = "REFERENCE_DEDUPE_KEY_UNSORTED";

/// Named invariant: a first stone contributed no value because every pair it
/// belongs to was already walked — which only a deduping walk may do.
pub const REFERENCE_EVERY_PAIR_ALREADY_VALUED: &str = "REFERENCE_EVERY_PAIR_ALREADY_VALUED";

// Which of the seven above can fire today, stated so a reader does not count
// them as coverage they are not. Two can. `REFERENCE_PAIR_ORDER_DISAGREES` fires
// only under `BothOrderings`, since a deduped walk never reaches a pair twice;
// it never has (measured over 230 669 root turns of undesigned positions), and
// the claim it carries is bought by the mode comparison now. `REFERENCE_DEDUPE_
// KEY_UNSORTED` fires under either mode and is the likeliest for an extension to
// trip: under mutation it named a reversed interior candidate loop.
//
// The other five restate what pistol-core, `candidates` and the ledger already
// guarantee — the walk descends only at turn boundaries, `place` at phase 1
// cannot return `TurnContinues`, `candidate_cells` has asked the rules about
// every cell it offers, and the last candidate at a node has always opened a
// cell nothing else did, so `REFERENCE_EVERY_PAIR_ALREADY_VALUED` has never been
// reached (measured over 12 272 distinct nodes, both modes). They are here for
// the reason `pvs.rs` carries the same set: the guarantee is one crate away, and
// an extension that broke it should fail where the assumption is made.
