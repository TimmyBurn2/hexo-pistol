# WP-1.9 — the rule-5 bench, RUN 1: BELOW BRACKET. A finding, not an acceptance.

**The bracket registered at `562c8eb` is not moved (D-374).** It said
[1.60, 2.10] both bands, and it said what a miss means: *"Below bracket but above
abort (1.00-1.60) — a FINDING, not an acceptance: the module split costs
something, and the record says how much."* This document is that record.

## The run

`tools/bench_delta.sh rev:723758b rev:07f518b 5`, artifact
`artifacts/wp19_bench_shipped_v1.txt`, exit 0, node identity holding per position
under both budgets in every rep.

| band | baseline nps | candidate nps | **nps ratio** | time-to-depth-2 ratio | bracket |
|---|---|---|---|---|---|
| early | 245359.4 (IQR 1704.6) | 369167.4 (IQR 678.7) | **1.505** | 1.564 (dev 0.060) | **BELOW [1.60, 2.10]** |
| late | 204196.1 (IQR 910.5) | 317466.2 (IQR 2013.1) | **1.555** | 1.576 (dev 0.021) | **BELOW [1.60, 2.10]** |

Above the abort threshold (< 1.00) in both bands, so this is not an abort. The
cross-check agrees with the fixed-node ratio to within 0.060 and 0.021.

## The registered hypothesis is FALSIFIED, and the stated cause was wrong

The prereg registered that **the module split is free**, on the ground that the
matrix measured this storage shape at 1.783 / 1.909 inline (`wp19/mx-O2`). The
shipped module-split version reaches 1.505 / 1.555, so roughly **1.18x early and
1.23x late of the available gain is not there**.

**But the module boundary is not the cause, and the prereg's framing pointed at
the wrong thing.** The cause is an extra hash lookup per window in `undo`,
introduced by the shipped `WindowMap` API and not by the split:

- **Shipped `07f518b`** — `undo` calls `WindowMap::get` and then `WindowMap::set`.
  **Two lookups per window.**
- **Matrix variant `wp19/mx-O2`** — `undo` takes a single `entry()`, holds it as
  `Entry::Occupied`, mutates through it and removes through it. **One lookup.**

`apply` (one lookup, through `entry_or_default`) and `delta` (one lookup, through
`get`) are unaffected and match the matrix variant. So the regression is confined
to take-back, at `WINDOWS_PER_CELL` = 18 extra lookups per stone unwound — and a
search unwinds every stone it applies.

## What this changes

The finding is a defect in the shipped API's shape, not in its correctness: Track
E held (`artifacts/wp19_byte_identity_v1.txt`, IDENTICAL), every test passes and
every registered mutant died. **A `WindowMap` operation that resolves the entry
once** — read, mutate, and conditionally remove through one lookup — restores the
matrix variant's lookup count without giving up the module split or the newtype.

That fix and its re-measurement are RUN 2. **This run and its numbers stay
recorded whatever run 2 says**: a registered bracket that a session quietly
re-runs until it passes is not a bracket.

---

# RUN 2, and the cause named by measurement

## Run 2 — the one-probe `undo`, at `16c6b70`

`tools/bench_delta.sh rev:723758b rev:16c6b70 5`, artifact
`artifacts/wp19_bench_shipped_v2.txt`, exit 0, node identity holding.

| band | nps ratio | time-to-depth-2 | bracket |
|---|---|---|---|
| early | **1.508** | 1.549 (dev 0.041) | still BELOW [1.60, 2.10] |
| late | **1.579** | 1.593 (dev 0.015) | still BELOW [1.60, 2.10] |

**Run 1's diagnosis is FALSIFIED.** Removing the extra probe moved the ratio by
0.003 early and 0.024 late. The double probe was real — REVIEW-impl found it
independently and it is fixed — but it was never the cause.

## RUN 3 — the cause, measured head to head

Two hypotheses had now failed, so the third was measured instead of guessed:
`tools/bench_delta.sh rev:9a986c6 rev:16c6b70 5` puts the INLINE storage shape
(the matrix's `wp19/mx-O2`) on the baseline side and the SHIPPED module on the
candidate side. Artifact `artifacts/wp19_bench_inline_vs_module_v1.txt`, exit 0,
node identity holding per position under both budgets in every rep.

| band | inline nps | module nps | **ratio** |
|---|---|---|---|
| early | 438218.3 (IQR 3180.1) | 369847.7 (IQR 2515.9) | **0.844** |
| late | 385494.6 (IQR 2361.4) | 319080.7 (IQR 1831.4) | **0.828** |

**THE MODULE SPLIT IS THE COST, AND THE PREREG'S REGISTERED HYPOTHESIS IS
FALSIFIED.** It registered that the split was free. It is not: it costs
15.6 % early and 17.2 % late of whole-engine nps.

**The three runs compose, which is the check that they are measuring one thing:**
1.783 x 0.844 = **1.505** against run 1's measured 1.505, and
1.909 x 0.828 = **1.581** against run 2's measured 1.579. Independent runs, and
the arithmetic closes to within 0.002.

`bench_delta.sh` prints `VERDICT ABORT` for run 3 against its own 1.15 threshold.
**That wording does not apply here** — run 3 is a diagnostic comparison between
two candidates, not a change proposed against a baseline, and nothing is being
reverted on it. It is reproduced because the instrument's own output is what
gets cited.

## What ships, and the debt that leaves with it

**The module version ships.** It is the version REVIEW-impl reviewed, the version
every mutant was re-run against, and the version Track E proved bit-identical
twice. Restructuring at the end of the round would ship code no reviewer has
seen, which is the one thing this process exists to prevent.

**The cost is recorded, not explained away**, and the remedy is named and cheap:
rule 9's ~300-line cap is a SOFT cap whose own remedy is a why-justification
entry in `docs/rule9_justifications.md`. Inlining the storage back into
`handcrafted.rs` costs 15 lines over that cap and one justification entry, and
buys back 1.18x-1.21x. That is a follow-up package with its own REVIEW-impl and
its own bench, not an unreviewed edit here.

**The bracket is not moved (D-374).** WP-1.9 closes BELOW its registered bracket
at 1.508 / 1.579, above its abort line, with the cause measured.
