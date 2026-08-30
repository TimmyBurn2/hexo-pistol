# MATRIX WP-19-S — SELECTION RECORD

**Selected: O-2**, `HashMap<u64, Counts, BuildHasherDefault<WindowHasher>>` over the
order-preserving packed key, **PROVISIONALLY**, with O-3 registered as a flip
trigger.

**Authority.** Operator ruling taking branch B of the two-way choice D-500 handed
back. D-500's cap is lifted for this selection only: the matrix is NOT given a third
revision, and this record does not repair it. It selects on the matrix's ONE
surviving ground and carries the attack that ground does not answer.

**Governing documents.** Field: `matrix_wp19_storage.md` revision 2 (`c9befe6`).
Attacks: `matrix_wp19_storage_REDTEAM.md` (round 1, FAIL, 3 BLOCKING) and
`matrix_wp19_storage_REDTEAM_rev2.md` (round 2, FAIL, 1 BLOCKING). Stop record:
D-500.

---

## 1. What the selection rests on, and what it does not

**It rests on one ground: O-2 is the fastest shape measured.** Whole-engine, one
instrument (`tools/bench_delta.sh`, on D-289's DRIVEN list), baseline `a5c5661`,
5 reps, both budgets, node identity holding per position in every rep, every run
exit 0:

| row | governing revision | nps early | nps late | artifact |
|---|---|---|---|---|
| O-1 packed tree | `abf3d5d` | 1.198 | 1.242 | `wp19_mx_bench_O1_fmt_v1.txt` |
| **O-2 hashed** | `9a986c6` | **1.783** | **1.909** | `wp19_mx_bench_O2_fmt_v1.txt` |
| O-4 direct | `22bbd96` | 1.737 | 1.837 | `wp19_mx_bench_O4_v1.txt` |

The round-2 reviewer verified the O-2/O-4 margin is real against noise: 5.1x/7.9x the
within-run IQR and ~3x the cross-run drift.

**It does NOT rest on either ground revision 1 also claimed.** Both were withdrawn on
measurement and neither is revived here:

- **D-498 canonical equality is a small cost, not a filter.** Every row can satisfy
  it; `std` supplies it free and a hand-written `impl` is ~11 lines. It does not
  discriminate.
- **The memory ground was measured FALSE IN O-2's OWN DISFAVOUR.** O-2 costs
  33.9-54.7 B per live entry against O-0's ~19.5, and retains ~66.6 % of peak bytes
  at zero entries where both tree rows release ~99 %. **O-2 is selected DESPITE its
  memory.** What makes that affordable is absolute magnitude — 6.5-13 KB at the real
  occupancy of 120-386 windows, against the 268 MB transposition table the bench's own
  identity block records — and not a bound in live entries.

---

## 2. The strongest surviving attack, quoted verbatim

CLAUDE.md requires the surviving option's ADR line to record the strongest surviving
attack rather than a paraphrase. This is the round-2 reviewer's own wording:

> O-2 is the fastest shape ever measured against anything, and the record should
> say only that. The one row that could contest it on the one ground the
> selection uses — O-3, the hand-rolled probing table D-225 named and D-249
> reproduced at a table-only 4.07-4.20x over a packed tree — was never
> implemented, and the matrix's two attempts to exclude it both fail: §4.1's cell
> "bounded above by O-4" is contradicted four paragraphs later by §6's own
> concession that "a hand-rolled table could plausibly beat O-4", and §6's
> argument needs O-4 to be the arithmetic floor when the shipped lookup at
> `22bbd96` is four comparisons, two multiplies including a `w * h` recomputed on
> every call, and a bounds-checked index — not the "one bounds check, one
> multiply-add, one load" §2 claims. A row that is a floor on neither arithmetic
> nor locality bounds nothing, so O-4's 1.837 says nothing about O-3, and "the
> storage layer has little left to give" is an inference the measurements do not
> carry. O-2 is selected on a field that is incomplete on the exact axis it is
> selected for.

**The attack is accepted, not answered.** The record says O-2 is the best MEASURED
shape and does not say it is the best shape.

---

## 3. The registered flip trigger

**O-3 — a hand-rolled open-addressing probing table over the packed key — flips this
selection if it is implemented and measured above O-2** on the same instrument, the
same baseline, and the same fixture, at a margin exceeding the within-run IQR.

Registered now, before any run, so a later session cannot move it (D-374):

- **Instrument:** `tools/bench_delta.sh rev:<baseline> rev:<O-3> 5`, baseline
  `wp19/mx-base` (`a5c5661`), config `configs/instrument_v0.toml`, fixed nodes 50000
  and depth_turns 2, 24 positions, 5 reps.
- **Comparand:** O-2 at `wp19/mx-O2` (`9a986c6`), whose numbers are 1.783 / 1.909 and
  DO NOT MOVE.
- **Flip condition:** O-3's nps ratio exceeds O-2's in BOTH bands. A win in one band
  only is a finding, not a flip.
- **Consequence if it flips:** the storage shape is re-selected and this record is
  superseded, not amended. The `Eval` contract does not change either way, so a flip
  is a container swap behind the same seam this package is already building.
- **Consequence if it is never run:** the debt stays named and open. It is NOT
  discharged by silence.

**Every measured revision is tagged** so the comparison reproduces rather than being
re-measured: `wp19/mx-base`, `wp19/mx-O1`, `wp19/mx-O2`, `wp19/mx-O4`, and the two
superseded pre-`rustfmt` revisions.

---

## 4. Conditions the selection carries into the design

1. **The storage lands in its own module, not in `handcrafted.rs`.** MEASURED: O-2
   inline takes that file to 315 lines, past rule 9's ~300 soft cap, and there is no
   entry for it in `docs/rule9_justifications.md`. O-4 demonstrates the remedy — a
   storage module leaves `handcrafted.rs` at 267. A hasher and a key packing are not
   the evaluation.
2. **The packed key owes an order-preservation test**, not a comment. Round 1
   verified the arithmetic exhaustively over the full `i16` range on both coordinates
   and all three axes with zero mismatches, but that receipt lives in a review report
   and not in the suite.
3. **The hasher is seedless by construction and owes a test saying so.** No
   `RandomState`, nothing environment-derived; this is what keeps the map clear of
   rule 4 and D-32, and the `windows` field's doc comment claiming the map is
   "Ordered" must be corrected rather than carried (round 1 m2, which survives at both
   candidate revisions).
4. **The memory figure is stated as a number with its derivation in a test.** This is
   a DISPATCH requirement (`wp19_storage_DISPATCH.md`), not registered scope, and it
   is honoured here because it is cheap and useful — NOT used as a ground against any
   row, which is the error B1 caught.
5. **Track E is unchanged and is what the design must prove.** Node identity held in
   all three benched candidates, which is evidence for it but not the proof; the
   equivalence harness is.
