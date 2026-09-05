# WP-2.2 Phase 1 — the fit's own finding

> **SUPERSEDED IN ITS MECHANISM BY D-621.** This document's direction is
> BACKWARDS and its named successor cannot work. Every one of the 3 882
> five-window positions has the window owned by the side NOT to move, so the
> dropped mate rows are predominantly ones where the window FAILED (1 353 of
> 1 873 are `mate_in`), and the side-to-move-relative feature takes ONE SIGN
> ONLY. `w5` is not censored, it is NOT IDENTIFIABLE, and a censored likelihood
> cannot repair it. The arithmetic below reproduces; the causal story does not.
> Read D-621 first.

**Run revision** `71fa6f1`. Corpus
`artifacts/arc3r_sweep_deduped_manifest.txt`, 89 805 deduped positions.
Instruments: `tools/texel/{features,extract,fit}.py` and
`crates/pistol-eval/examples/static_eval.rs`. Diagnostics live in artifacts
(D-483); this document states the MECHANISM, and the numbers here are the ones
the mechanism is about.

## The correctness gate passed first, so the rest is readable

`docs/experiments/wp22_phase1_design.md` §4 registers an oracle: the Python
feature extractor's value must equal the engine's own `HandcraftedV0::value`,
exactly, on a registered sample. **0 of 500 positions disagree.** The mutant
that must die does: perturbing one table entry by 1 makes **410 of 500**
disagree. Permuting the axis enumeration order changes nothing, which is
correct — the eval sums over windows and the `Eval` contract says the value
depends only on the set of stones.

**So premise §2 holds: the v0 score is linear in the five weights**, and every
number below is about the fit rather than about the extractor.

**The join was verified on every row, not sampled.** `extract.py` refuses any
record whose `key_full` differs from the manifest's; all 89 805 passed.

## What the fit returned

The unconstrained least-squares solution to the search-score labels is

```
w = [1.41, 22.44, 18.13, 68.58, -7.05]
```

which **is not monotone and is negative at the top**: it says a window holding
five own stones — one stone from a win — is worth less than nothing.

The schema forbids that (`weights.rs:141-177`), and the **exact constrained
optimum** over the schema's set is

```
w = [1.42, 21.39, 22.39, 63.62, 64.62]   ->   [1, 21, 22, 64, 65]
```

with the gap constraints **BINDING between entries 2-3 and 4-5**. The committed
table is `[2, 12, 60, 300, 1500]`.

**A binding constraint is the whole finding.** The fit does not merely prefer
different numbers; it is pushed against the wall that stops it saying five
stones are worth no more than four. Its preferred table is FLAT where the game
is sharpest.

**One methodological correction is folded in here rather than hidden.** The
first implementation PROJECTED the unconstrained solution onto the schema. That
is not the constrained optimum — a projected point can sit on a face the true
optimum never touches — and it was replaced by exhaustive enumeration of the
sixteen active sets, which is exact and carries no step size, tolerance or
seed. The two answers differ (`[1,22,23,69,70]` against `[1,21,22,64,65]`), so
the error was real and not cosmetic.

## The mechanism, measured

The design excludes rows whose `score_kind` is `mate_in` or `mated_in`, because
a mate score is the search's mate band and is not a number this table can
produce. **That exclusion is correct in isolation and it is not neutral.**

| row set | n | carry a 5-stone-window term | carry a 4-stone-window term |
|---|---|---|---|
| `eval` — **fitted** | 74 672 | **2 009 (2.69 %)** | 29 165 (39.06 %) |
| mate — **excluded** | 15 133 | **1 873 (12.38 %)** | 9 318 (61.57 %) |

**A position carrying a five-stone window is 4.6x more likely to be scored as a
mate than as an eval.** Of the 3 882 positions in the corpus that hold such a
window, **1 873 — 48 % — are dropped before the fit sees them**, and they are
dropped on exactly one side of the question: the side where the five-window
converted.

**So `w5` is estimated from 2 009 positions selected for having a nearly
complete line that did NOT win.** The fit is not wrong about its own data. It
is answering a question nobody meant to ask.

## What this does and does not license

**It does not license shipping these weights, and it does not license refusing
to.** D-614 is explicit: SPRT is the only voice, and offline metrics are
diagnostics. The diagnostics here **improve** — validation MSE 705 709 against
the committed table's 999 066, Spearman 0.319 against 0.209 — and that is
precisely the reading D-614 forecloses. **A better fit to a censored target is
not a stronger engine**, and this document exists so that the improvement is
not reported as progress.

**It is not a STOP.** No premise failed: the oracle passed, the loader is
untouched, the extractor agrees with the engine. What failed is an assumption
the design did not know it was making.

## What Phase 1 owes next, named so a successor does not choose after the fact

1. **The registered SPRT still runs on the registered primary.** The procedure
   was registered before the data; running it is what makes the finding a
   measurement rather than an opinion. The honest expectation, stated before
   the run: **these weights lose**, and by a lot, because the mechanism above
   says the table cannot tell one-from-a-win from two-from-a-win.
2. **The named successor changes, and it is named BEFORE the SPRT rather than
   after it.** `wp22_phase1_design.md` §8 named the sigmoid-link variant as the
   successor if h0. This finding supersedes that: the first thing to fix is not
   the link function but the CENSORING — a `mate_in` row is not missing data,
   it is the observation *"this position's value is at or beyond the band"*,
   which a censored (Tobit-style) likelihood uses and a drop-by-kind throws
   away. The sigmoid variant moves behind it.
3. **R2's augmentation ruling is untouched** by this and is still owed its
   tabulation (D-611).
