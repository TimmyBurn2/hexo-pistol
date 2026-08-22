# MATRIX M4, AXIS A — **SELECTED: N-E**, and it is not what the matrix recommended

**Selected at `7e0a328`**, the revision carrying the attack, by the session
dispatching WP-1.5b design closure under the architect's round-4 ladder (R12).
Field: `docs/experiments/matrix_M4_axisA_round4.md` at `7866bcf`. Attack:
`docs/experiments/matrix_M4_axisA_REDTEAM.md`. ADR line: **D-329**.

> **NEITHER THE MATRIX NOR ITS RED TEAM IS EDITED BY THIS RECORD.** Everything
> here is stated above them. The matrix recommended N-Q; the attack killed that
> recommendation; this record selects the row the attack recommends instead.

## THE PATH THAT FIRED

The architect's dispatch registered a three-rung tiebreak ladder BEFORE any
measurement was taken: **(a)** hard-rule-1 config law conformance — explicit,
closed-enum, deny-unknown, one schema home; **(b)** fewest MEASURED added lines;
**(c)** recorded-arbitrary. What follows is that ladder run on the landed
evidence.

**FIRST, N-M IS ELIMINATED, AND NOT BY THE LADDER.** Red-team F10, MEASURED:
`docs/experiments/wp15b_sprt_prereg.md` §7A.2 registers this script as the
instrument for DOUBT 2 and names the invocation
`tools/baseline_snapshot.sh --config configs/gate_v0.toml`; §10 registers the
document's flip *"if `tools/baseline_snapshot.sh` lands `--config` in a shape the
§7A.2 criterion cannot be taken under."* Driven against all three rows, N-E and
N-Q take that invocation at exit 0 and **N-M refuses it at exit 1** —
``--config takes `instrument` or `staged`, got `configs/gate_v0.toml` ``. N-M lands
the flag in exactly the shape the registered criterion cannot be taken under, in
a pre-registration whose §11 records that it has never passed a review. **That is
an elimination on registered ground, not a tiebreak preference**, and the matrix
never states it.

**RUNG (a) DOES NOT FIRE.** The matrix selected N-Q here and the attack destroyed
the reading, on three independent grounds this author has verified and accepts:
hard rule 1's fourth clause governs DEFAULTS and MEASURED none of the three rows
has one, all refusing an absent `--config` by name at exit 1; the round-3 red team
had already ruled that same clause a WOUND and not a KILL for that reason; and
`configs/` is not one schema — it holds four engine configs, six arena match
configs and a weights table, with the engine exiting 2 on two of three sampled.
Rung (a) is silent across the field.

**RUNG (b) FIRES, AND SELECTS N-E.** With N-M eliminated, the comparison is
N-E against N-Q, and it is not close on either reading of the rung:

| | N-E | N-Q |
|---|---|---|
| added lines | **22** | 32 |
| of which CODE | **7** | 12 |
| whole-path guard owed | 4 | 4 |
| containment lines | 0 | 5 |
| item-10 driving tests owed (F7) | **2 classes** | 5 classes + an unpinned normalisation |

Every count was produced by a separate measurement agent under D-328 and
independently re-derived by the red team, which reproduced all twelve cells of
the line-count table exactly.

**Rung (c) is not reached.** The selection is not arbitrary and is not recorded
as such.

## WHAT THE ATTACK ADDED BEYOND COST, and why it makes rung (b)'s answer the right one rather than merely the cheap one

Rung (b) would select N-E on lines alone. Three measured findings say the same
thing on substance, and they are recorded so the selection does not rest on a
line count:

1. **N-Q's own selling property is false. MEASURED (F4):** `.gitignore:7` is
   `*.bin` under rule 8, so `configs/ghost_v0.bin` is invisible to
   `git status --porcelain`, passes containment, and its name reaches the
   invariant block at **exit 0** beside a revision at which no commit contains it.
   Containment bounds by DIRECTORY, never by COMMIT. *"The record names a
   committed, re-runnable document BY CONSTRUCTION"* is false for N-Q exactly as
   it is for N-E.
2. **N-Q is the only row that ADDS a defect. MEASURED (F5):** `ROOT` is bash's
   logical `pwd` and `realpath -m` is physical, so invoked through a symlinked
   checkout path N-Q refuses the repository's own `configs/instrument_v0.toml`
   with *"resolves to …, which is not under …/configs/"* — a directory it plainly
   is under — on an invocation N-E completes at exit 0. A script that declares no
   void class must carry the VOID/FAIL distinction in its message, and this
   message reports a void as a fail (`SHELL_CHECKLIST` items 8 and 12).
3. **N-Q's extra lines are required by no rule in this tree.** Round 4 itself
   established, and the attack confirmed first-hand (F13), that item 11 does not
   reach `$CONFIG` — its scope is a binding consumed by `rm`, `mv` or a write —
   and that item 9, which does govern it, is discharged by the whole-path guard
   **both** rows owe. And the converse bites: `--out` is this script's one real
   item-11 binding, a caller path consumed by a write, and it is deliberately
   resolved and NOT contained. N-Q would contain a READ binding more tightly than
   this script contains its WRITE binding.

## WHAT N-E MUST SHIP, and none of it was costed for any row

- **The 4 whole-path guard lines**, on the whole path the record writes and not
  on a basename. FACT 5's surviving half is why: the existing loop guards
  `${named##*/}` while the `config` line writes `$CONFIG`, so reusing that loop
  leaves the digest displaced at exit 0 under the COMPLETE kind token.
- **An item-10 driving test for both new refusal classes**, in two halves with a
  control, per the coverage rule. The precedent is one commit old: `b067d47` paid
  91 test lines for ONE guard arm.
- **An item-12 sentence** in the usage block saying what a config refusal is — a
  FAIL, since this script declares no void class.
- **The 2-line test retrofit** (FACT 9), identical across the field.

## CONDITIONS THAT RIDE WITH THIS SELECTION

1. **The `config` line's digest is `$3`, not `$4`.** Verified at selection:
   `config <path> <sha>` is three fields. Any future guard, test or reader of
   that line uses `$3`, and the four-token reasoning belongs to the `corpus`
   line, which has a different shape. This is the correction F1 forced.
2. **N-E's whole-path guard may NOT be spelled as a reuse of the line-289
   basename loop.** Measured twice, by the measurer and again by the attacker:
   that spelling leaves `configs/spaced dir/instrument_v0.toml` reaching the
   record at exit 0.
3. **D-324's "three lines" remedy is superseded by this record**, both in count
   (four at `b067d47`) and in kind (a new whole-path guard, not a copy).
4. **The relative-base inconsistency is recorded, not fixed** (F6): a relative
   `--config` resolves against `$ROOT` while a relative `--out` resolves against
   `$CALLER_PWD`. N-E inherits it; unlike N-Q it does not make it load-bearing
   for a refusal. It is OPEN.

## THE STRONGEST SURVIVING ATTACK AGAINST N-E — **ASSEMBLED, NOT QUOTED, AND THAT IS A RESIDUAL**

The red team was dispatched to attack N-Q and the tied set's interaction with it.
It recommends N-E, but **it was not asked to break N-E, and no fresh context has
been.** So the paragraph below is assembled by this author from the round's own
measured findings rather than quoted from a dedicated attack, and the difference
is recorded rather than smoothed over:

> N-E is selected because it is the cheapest row that adds no defect — not
> because it delivers what the record's `config` line exists for. That line is
> provenance: a reader re-runs the run from it. N-E bounds the admissible set
> nowhere, so a record may name a document outside the repository that no other
> reader can obtain, and its provenance rests on caller discipline plus a digest
> of bytes nobody else holds. The round measured that the one row which tried to
> fix this does not fix it either — a gitignored file inside `configs/` reaches
> the invariant block at exit 0 — so the weakness is not resolved by this
> selection, only left where it was. N-E therefore ships a seam whose provenance
> guarantee is exactly as strong as the caller, and this round chose it knowing
> that and having measured that the alternative bought nothing for its five extra
> lines except a new false refusal.

**RESIDUAL, OPEN AND THE ARCHITECT'S:** N-E has not been attacked by a
fresh-context DECISION-RED-TEAM in its own right. The matrix law is satisfied for
the FIELD — a matrix was authored and attacked before selection — and this record
does not claim more than that.

---

*Selection record for MATRIX M4, axis A, round 4. Ladder rung (b) fired, after
N-M's elimination on registered ground outside the ladder. Selected N-E against
the matrix's own recommendation of N-Q, on its attack. D-329.*
