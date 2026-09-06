# REVIEW-impl — WP-2.2 Phase 1, `0220d83`

**Named revision** `0220d8345cf6b4e61d73f63661fc379b2c181976` on `dev`, matching HEAD at both ends.
The live tree was clean at both ends and was never modified: all building, mutation and
re-derivation ran in a detached worktree under `/home` with its own `CARGO_TARGET_DIR`, and the
worktree was removed. **VERDICT: FAIL.** Six MAJOR; 36 mutants seeded, **19 killed, 17 survived**.

> The registered answer is correct and reproduces at the reviewer's own scope, the oracle is clean
> at full scale, the dry run reproduces from the committed config, and every gate is green.
> **What fails is the defence of that answer.**

## Gate table — each gate's own output line

| gate | the gate's own line |
|---|---|
| texel tools (18) | `test_texel: all checks passed (including census_classes)` — 72 `ok` lines |
| config validation (6) | `config_check: 22 engine config(s), 2 weight table(s), 20 arena config(s), …`; `ok configs/instrument_quiet_fit_v0.toml … weights=configs/eval_v0_quiet_fit_weights.toml`; `validate_arena_config: 20 document(s) ok` |
| governing citations (21) | `governing_citation_check: 15 governing document(s), 0 proposed path(s)`; `wp22_phase1_design.md: 16 citation(s) checked, 0 unreproduced` |
| fmt (1) | zero bytes of output |
| clippy (4) | `Finished dev profile … in 5.97s`, zero warning/error lines |
| new Rust test | `test result: ok. 3 passed; 0 failed` |

## Re-derivation ledger (abridged)

A fresh `extract.py` into a fresh path then `fit.py` prints **`[5, 34, 60, 300, 1500]`**; the row
file is a pure function of its inputs (two runs, one digest `228e8245…`); the fit is byte-identical
across runs; the oracle reproduces `41215 / 39195 / 0 disagreements` with saturation `0 / 3511 /
40016`; the registered sample rule genuinely holds at the shipped defaults (16/16 tranches, 3/3
kinds, the whole 39 195 stratum) **checked against the manifest rather than against `sample()`**;
`key_full` sharing is **2 369 rows in 387 groups**; the release binary is bit-identical to the
registered `d1d9465c…`; the dry run reproduces **from the committed config** down to
`nelo_pair 71.670488923` and `wall_ms 49051`; the criterion is MET; cost 2.044 s/opening → 13.6 min;
receipt 12 of 12 OK; the single skipped active set is `(0,1,2)`, three constraint vectors summing
to zero — **legitimate, counted, printed**.

## MAJOR findings

**M-1. `tools/texel/extract.py` has no test at all.** Nothing imports or drives it. It prints a
recorded number and writes the row file every other number rests on. Three mutants survive every
gate: the per-row `key_full` join check removed; **the mate-row exclusion restored**, which is the
invariant the design's stage-2 bullet is written to forbid; and a mis-indexed column. A direct
breach of the tools/ coverage rule and SHELL_CHECKLIST item 10.

**M-2. The Rust test never loads the committed candidate; it reconstructs both tables as code
literals.** Mutant **R3** — editing `configs/eval_v0_quiet_fit_weights.toml` to the committed
table — **passes the Rust test, `config_check.sh`, `texel_tests.sh` and the citation gate**. That is
exactly the defect class §9's dry-run criterion names (a self-match), and nothing in the committed
tree catches it before a run.

**M-3. Nothing enforces or reports §5's registered sample rule**, and two mutants on it survive:
dropping the tactical stratum from `sample()`, and dropping a clamp table. Both are inert on the
two-position stub the suite drives the oracle with. `ORACLE.txt` does not record the stride it ran
at.

**M-4. Three `features.py` mutants survive** — an axis changed, the `k=6` splice dropped (a
six-window scores 3 892 instead of 16 000), and the mover alternating per stone rather than per
turn, which `test_turn_structure`'s two-turn fixture is exactly the length to miss. The engine
oracle kills all three and is never executed by CI.

**M-5. Hard rule 3 — `signed()` reads any `to_move` that is not literally `"p1"` as p2, silently,
and it changes the answer.** Corrupting only the case of the token (`p1`→`P1`) produced
**`[1, 2, 60, 300, 1500]`** with no error, no refusal and no diagnostic: the sign-variation guard
still passed, `round_to_schema` passed, dominance passed. Combined with M-1's surviving column
mutant, a schema change produces a confident, schema-legal, wrong candidate.

**M-6. §4's registered scale-invariance fix is falsified in both halves.** (a) The literal `solve`
body from the parent commit — absolute pivot — **passes the entire suite**, so the test does not
discriminate the fixed routine from the defective one. (b) The shipped routine is not invariant
either: the scale vector is computed once and goes stale as elimination cancels the constraint rows
against the normal rows. Mitigating: on the registered path both agree exactly, and the failure mode
is a loud refusal rather than a wrong number.

## MINOR

m-1 `fit.py` and `test_texel.py` newly cross rule 9's cap with no registry entry (the gate covers
only `.rs`/`.sh`, and twelve tracked `.py` files are already over — the standing convention).
m-2 `fit.py` and `extract.py` have no usage block and no argument validation; `extract.py`'s output
path is unguarded. m-3 `fit()`'s `plain = solve(...)` would raise `TypeError`, not `FitError`, on a
`None`. m-4 `DRYRUN.md`'s lean section is an **empty code block** while §9 cites it. m-5 the
receipt's own digest is anchored in no tracked document, unlike this project's precedent. m-6 no
test drives `tools/texel_tests.sh` itself, so item 12(3) is unmet for it. m-7 three mutants each
move a number `fit.py` prints with no test. m-8 the matrix was added to the GOVERNING list and the
gate reports **`0 citation(s) checked`** for it — its one instrument citation is brace-expanded and
the checker does not recognise it as a path, so the addition is inert. m-9 the dispatch says the
list "gains four documents"; the diff adds two. m-10 `verify_against_engine.py`'s `__main__` is
never driven as a shipped script.

## SHELL_CHECKLIST, answered by name for `tools/texel/`

Items 1, 2, 3, 6, 7, 8, 9 clean; item 4 (`LC_ALL`) not pinned but nothing depends on a character
class; item 5 the suite reads the WORKTREE, which is the intended reading for a pre-commit suite;
**item 10 the COVERAGE RULE — FAILED** (M-1); item 11 one caller-supplied write, unguarded, no
deletes; item 12 the void/fail distinction is correct in the wrapper, obligation (3) unmet (m-6).

## Determinism (hard rule 4)

No unseeded iteration reaches a recorded number. Both walkers iterate `sorted(...)` over
`(record_number, key_full)` pairs whose first element is unique; `window_counts`' dict is consumed
only through `.values()`; the sign check uses sets only for a length; `constrained_min` breaks ties
by strict `<`; `ranks` sorts stably on index. Confirmed empirically: two extractions gave one
digest, two fits gave byte-identical output.

## Configs against §9, and the ledger

All ten §9 rows match the committed arena config; `diff` of the two seat configs is one hunk,
`weights_file`. The ledger row lands in the same commit as the arena config, takes the next
unconsumed range, and decrements the reservation 1000 → 600 with its reasoning. Meets the file's
own stated rule.

## What the reviewer attacked that SURVIVED

The registered number, by fresh extraction and fresh fit; purity and determinism of both stages by
digest; the oracle at full scale with the reviewer's own build and extraction; the registered sample
rule genuinely holding at the shipped defaults; a bit-identical binary; the dry run reproducing from
the COMMITTED config; the cost estimate; the legitimacy of the single skipped active set; every
load-bearing count (50 610 / 39 195 / 45 271 / 5 339 / 2 369 in 387 groups); **19 of 36 mutants
dying**, including every one that would have made the fit non-refusing, every filter clause, the
tempo model, the oracle's own refusal, and both call-site mutants of D-553's law; and all seven
gates green, cited from their own output.

## QUESTIONS

Q-1 at the registered optimum no constraint binds, so `constrained_min` reduces to the size-0 KKT
solve — what does the enumeration buy on this path? Q-2 the fitted table scores **worse** on the
fit's own objective than the table it replaces (val MSE 762 720 against 724 743), because the tempo
term is discarded before the diagnostic — is that direction registered anywhere as expected?
Q-3 no position holds a 6-window without also holding a 4- or 5-window, so the `k=6` term in the
predicate changes nothing — keep or delete under D-424? Q-4 `sample()` selects the tactical stratum
using the very code the oracle exists to check, so a bug that under-counted 4-windows would shrink
the stratum and still report 0 disagreements — should the stratum come from the engine side?
