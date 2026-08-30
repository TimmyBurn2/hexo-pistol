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
