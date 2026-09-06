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
| 0 | 13 | `0..12` | the WP-2.0 label-pipeline PILOT, `configs/arena_wp20_label_pilot.toml` | `docs/experiments/wp20_pilot_prereg.md` revision 4 |
| 13 | 3487 | `13..3499` | the WP-2.1 PRODUCTION LABEL SWEEP, sixteen tranches from `tools/wp21_tranche_config.py` | `docs/experiments/wp21_prereg.md` revision 7 |
| 3500 | 400 | `3500..3899` | the WP-2.2 PHASE 1 quiet-fit SPRT, `configs/arena_wp22_phase1_quiet.toml` | `docs/experiments/wp22_phase1_design.md` revision 3 |
| 3900 | 600 | `3900..4499` | **STILL RESERVED FOR GOVERNED RUNS — NEVER LABELLED.** What is left of D-568's holdout after the row above | `docs/decisions.md` D-568 |

**THE SECOND ROW IS THE SWEEP AND ITS CONFIG IS NOT ONE FILE.** The sweep is
sixteen tranches of one shape, and this table's rule — *"adds its row here in the
same commit that adds its arena config"* — is satisfied by the GENERATOR rather
than by sixteen near-identical committed documents: every value they carry is
fixed in the pre-registration and written by `tools/wp21_tranche_config.py`,
which has a test driving the shipped script and whose own partition claim —
contiguous, disjoint, exhaustive over `13..3499` — is what that test asserts.
**The range is consumed by the DOCUMENT, not by the tranche**: a tranche that
voids does not give its openings back, because a second run over them would be a
second reading of a sample this registration chose.

**THE THIRD ROW IS THE FIRST DRAW FROM THE HOLDOUT, AND WHAT IT LEAVES IS THE
FOURTH.** D-568 reserved the last 1,000 openings for governed runs; WP-2.2's
Phase 1 SPRT is the first governed run to take from it and takes **400**, leaving
**600** for the two standing claimants below. Four hundred is the registered
MAXIMUM and not a prediction: an SPRT that crosses a bound earlier stops earlier,
and the range is spent either way, because a range reserved by a committed
pre-registration is spent whether or not its run finishes. **Why 400 and not
more**: the dry run measures 2.04 s per opening at the registered `turn_cap 60`,
so 400 is fourteen minutes — the clock is not what bounds this, the holdout is,
and the two claimants below have prior standing on it.

**THE FOURTH ROW IS A HOLDOUT AND IT IS A DIFFERENT KIND OF ROW.** Every other
row here records a range something SPENT. This one records a range nothing may
spend: D-568 reserves the LAST 1,000 openings for governed runs, of which 600 remain, and the rule
fixing it — the last 1,000, by position and by nothing observable — is stated
before the sweep starts, because a holdout chosen after seeing which openings
label well is not a holdout. **It is enforced mechanically and not by this
sentence**: `no_tranche_reaches_the_reserved_holdout` drives the shipped
generator over all sixteen tranches and fails if any slice crosses `3500`. An
earlier revision of this ledger recorded the book as FULLY CLAIMED, which is why
the reservation is worth its own row — the two standing claimants below, the
Stage-3 detector's SPRT and the WP-1.5d resolution run, now have a range to draw
from. **Dividing it between them is not decided here** and belongs to whichever
package needs it first.

**THE FIRST ROW, and what it does and does not spend.** Thirteen is what the
pre-registration's RULE-1 returns from the per-unit costs its dry run measured
(§6.3). **RULE-1 WAS AMENDED AFTER THOSE COSTS EXISTED, and this row says so
because the ledger is the durable record a successor reads first**: the rule as
first registered returned 56, was found to maximise against the clock rather than
against what the pilot's criteria need, and was replaced by a minimum against
stated floors — the whole disclosure, including the floor whose ground was
withdrawn and the sensitivity of 13 to that floor, is §6.1. A reader who wants to
know how firm `13` is should read that section and not this row. **The pilot is
not corpus** (D-539), so this range buys a pipeline shakedown and no evidence
about anything — which is exactly why the ledger records it: a range spent on a
run that concludes nothing is still spent, and a successor that re-read `0..12`
for a governed verdict would be reading a sample this session chose.

## Standing claims on the book, not yet consumed

| claimant | status | what it will need |
|---|---|---|
| The Stage-3 detector's SPRT | SCHEDULED | one slice of the standing shape |
| The WP-1.5d ±21.5 resolution run | **LICENSED, NOT SCHEDULED** (D-505, D-492) | a slice large enough to resolve an interval that spanned zero at 500 pairs; it runs under a NEW pre-registration and never as a re-read |

Neither has a row above, because neither has a committed pre-registration
drawing from this book. The size these two imply is registered with its grounds
in `docs/experiments/book_v2_registration.md`.
