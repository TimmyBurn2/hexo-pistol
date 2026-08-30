# `random_openings_v2.txt` — the consumed-ranges ledger

**What this file is.** One row per governed pre-registration that draws a slice
of `crates/pistol-cli/tests/fixtures/random_openings_v2.txt`, recording the
range it consumed. A slice appears here **when its pre-registration is
committed**, not when its run finishes — a range reserved by a document that
was never run is still spent, because a later run over it would be a second
reading of a sample someone else chose.

**Why it exists.** `random_openings_v1.txt` had no ledger, and what its ranges
were had to be recovered by reading `openings_skip`/`openings_take` out of six
arena configs after the fact (D-491, D-492). Every slice of v1 is now consumed
and v1 is **RETIRED FOR GOVERNED USE** (D-505). This file is so that the same
question about v2 is answered by reading one document.

**The rule.** A new pre-registration takes the next unconsumed range, adds its
row here in the same commit that adds its arena config, and never re-reads a
range this table already holds. Reading a consumed range for a CLOSED verdict
is not a new use and needs no row.

**Fresh by construction, and what that does and does not say.** v2 is drawn
from a different seed than v1, so **no RANGE of v2 is a range of v1** and no run
over this book can be a re-read of a consumed sample. It does not say that no
individual position appears in both — both books draw independently from one
finite pool, and the measured overlap (1 identical line, 10 positions up to
symmetry, against expectations of 0.59 and 7.04) is what chance gives. Both
counts are pinned by
`the_two_books_overlap_only_as_far_as_independent_drawing_makes_them` in
`crates/pistol-cli/tests/random_openings_document_tests.rs`; the arithmetic is
in `docs/experiments/book_v2_registration.md` §6.1. **So this ledger's only job
is to keep v2's own ranges disjoint from each other.**

**The book holds 4500 openings** (`configs/random_openings_v2.toml`), a size
registered from a measurement rather than chosen — `docs/experiments/book_v2_registration.md`
§4 registered the rule before the sweep and §4.1 records the sweep.

## Consumed ranges

| `openings_skip` | `openings_take` | range | consumed by | pre-registration |
|---|---|---|---|---|

*(empty: no range of `book_v2` has been consumed)*

## Standing claims on the book, not yet consumed

| claimant | status | what it will need |
|---|---|---|
| The Stage-3 detector's SPRT | SCHEDULED | one slice of the standing shape |
| The WP-1.5d ±21.5 resolution run | **LICENSED, NOT SCHEDULED** (D-505, D-492) | a slice large enough to resolve an interval that spanned zero at 500 pairs; it runs under a NEW pre-registration and never as a re-read |

Neither has a row above, because neither has a committed pre-registration
drawing from this book. The size these two imply is registered with its grounds
in `docs/experiments/book_v2_registration.md`.
