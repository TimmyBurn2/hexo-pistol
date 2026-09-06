# WP-2.2 Phase 1 design, revision 3 — REVIEW-design

**Named revision** `0220d83` (`dev`), still matching HEAD at the end; the live tree was clean at
both ends and was never modified. The reviewer's worktree (own `CARGO_TARGET_DIR` under `/home`)
is removed. Fresh context, not the author, not the implementer.

**VERDICT: FAIL. The registered SPRT may not launch as written.** Five MAJOR.

> The code is again in better shape than the documents: the candidate table, every count D-622 and
> D-626 carry, the oracle, the dry-run arithmetic and the tempo-cancellation mechanism all
> reproduce under attacks they were not asked to survive. What fails is what §9 licenses.

## Discharge — revision 2's 8 MAJOR, 10 MINOR, 4 QUESTIONS

M-1 **PARTLY** (bounds present; see MAJOR 1). M-2 **YES** (ledger row and arena config in one
commit). M-3 **YES**. M-4 **YES** (bounds are active-set members; the minimiser is interior).
M-5 **YES** (`verify_against_engine.py` ships; `sha256sum -c` 12 of 12). M-6 **YES in the design**,
not in the premise (m-9). M-7 **YES**. M-8 **YES as written**, class recurs (MAJOR 3).
m-1 **PARTLY** (m-1 below). m-2..m-10 **YES**. Q-1 **MOOT** (no constraint binds). Q-2 **YES**.
Q-3 **YES**. Q-4 **NO — undischarged**.

## MAJOR findings

### MAJOR 1. The registered SPRT cannot reach a verdict at its own bounds, and §9 states no power.

`sprt.rs` works in NORMALIZED Elo (`NELO_TO_T = ln(10)/800`, `Unit::t` multiplies by `sqrt(2)`),
so `t1 = 0.0407043`, boundaries `±ln(19) = 2.9444`, and the expected LLR per pair under either
hypothesis is `t1²/2 = 0.000828`.

```
pairs needed in expectation          3554        the registration caps at 400 (11 %)
at n = 400 crossing needs observed   +49.4 / -39.4 normalized Elo
P(verdict) at the dry run's 25 % capped rate:
     0 Elo  h1 0.000  h0 0.001  INCONCLUSIVE 0.999
    10 Elo  h1 0.001  h0 0.000  INCONCLUSIVE 0.999
    20 Elo  h1 0.008  h0 0.000  INCONCLUSIVE 0.992
    40 Elo  h1 0.151  h0 0.000  INCONCLUSIVE 0.849
```

Cross-checked against the project's own recorded runs: the combination crossed h0 at 455 pairs at
`-34.2` against a computed threshold of `-34.1`; S3 was undecided at the 600-pair cap at `-13.4`
against `-24.6`; S2 crossed at n = 90 at `-391.5` against `-192.5`.

**Why it blocks.** D-623 records R7: *"the SPRT on data-derived quiet weights answers 'does this
corpus move Elo at all'."* At 400 pairs `h0` has probability ≤ 0.001 at every effect simulated —
the run cannot return the outcome its governing ruling names as its point. Even D-568's entire
1 000-opening reservation would not (`|nelo| ≥ 22.8` at n = 1000). §9 discloses that the outcome
may be inconclusive and registers it; what fails is that it never puts the NUMBER on its face,
while the ledger tells a successor *"400 is the registered MAXIMUM and not a prediction"*.

**Minimal fix.** §9 states the pairs its bounds need and the observed nelo its cap requires; then
either widen `elo1`, or re-register as a SCREENING run and strike `h0` from what it can return —
which reopens whether R7's question is answered — or take an operator ruling on spending 400
reserved openings for a ~1 % chance of a verdict.

### MAJOR 2. `turn_cap 60` is an after-the-numbers choice with no registered threshold, taken in the measurement that also showed the candidate's lean.

Every other governed arena config carries 40. No threshold on capped fraction was registered before
the dry run; `docs/process.md` says *"Recording without a criterion is a dry run nothing can fail."*
The same two runs produced the arm comparison, and **60 is the setting at which the candidate looked
better** (22 W / 14 L and `nelo_pair +71.67` at 60, against 16 W / 10 L and `+56.36` at 40). At 24
pairs that is deep inside noise (`ci95 ±98.29`) — but the Process section forbids the post-hoc
threshold move, not the biased one. **Fix**: register the selecting rule first, and measure it on a
run carrying no arm information (one seat against itself).

### MAJOR 3. D-626's retired "forced win" ground survives in every artefact the design points a successor at.

`fit.py`'s module docstring; the predicate's own name `holds_forced_win` and its docstring stating
the retired mechanism; the **printed receipt field** `forced_win 29401`; three `test_texel.py`
assertions, one of which asserts the conclusion D-626 forbids (*"no fitted row holds a forced
win"* — 5 339 mate rows pass the filter); and `docs/ROADMAP.md`'s Stage-2 sentence, a governing
document. **Fix**: rename, rewrite both docstrings, rename the printed field, restate the tests,
amend the ROADMAP, re-take the receipt.

### MAJOR 4. The registered dry run's instruments are not the committed ones, and the committed dry-run config has never been run.

The three receipted reports name arena and engine configs living on the scratchpad tmpfs; the tree's
`configs/arena_wp22_phase1_quiet_dryrun.toml` (`97e55366…`) is named by no document and has produced
no report. `docs/process.md`: *"A pre-registration's literal commands are exercised before its
review passes."* **What this does NOT say**: the reviewer reconstructed the executed engine config
exactly and it hashes to the reported value, so no registered number is wrong. **Fix**: run the
committed config (49 seconds), replace the receipts, name it in §9 with its digest.

### MAJOR 5. §2's *"Both regressors have no sign variation anywhere in the corpus"* is false for `g4` by 1 593 rows.

Over all 89 805 positions, every `score_kind`: `g5` positive in **0** (structural, by rule 4);
`g4` positive in **1 593**, every one a `mate_in` row that clause 1 removes. The two entries are
pinned for two different reasons and §2 states one for both. The pin itself survives — both are
identically zero on the 45 271 fitted rows. **Fix**: separate the grounds; amend D-626 by ADR.

## MINOR

m-1 line 3 says four instruments are new; only `verify_against_engine.py` is, the other three are
modified. m-2 §1 says "at most three integers"; §4 pins `w3`, so at most two. m-3 §1 says two Rust
tests, §6 lists three. m-4 `DRYRUN.md`'s lean section is an **empty code block**, sha-receipted
empty. m-5 a third receipted report (`dryrun_report.txt`, 4 openings, cap 40) leans **negative**
(`nelo_pair −41.53`) and §9 mentions two. m-6 §9 never names its bounds' UNITS (normalized Elo)
while comparing them to the ROADMAP's +150 Elo. m-7 the sign-variation refusal lives in
`normal_equations`, which the ANSWER path never calls. m-8 the Rust test hard-codes both tables
rather than reading the committed documents. m-9 `wp22_phase1_premise.md` §7 still carries the
human-corpus holdout the design deleted, edited in the same commit. m-10 §8 registers a bench
bracket inside the DESIGN half, which D-483 places in the prereg; no hotspot, no abort threshold,
no receipt. m-11 §9 says nothing about what becomes of the two committed configs on h0 or
inconclusive. m-12 §4 lists a ceiling refusal that is unreachable by construction on the registered
path, in the section that deletes another guard for that reason.

## QUESTIONS

Q-1 should §9's expectation say the candidate fits the labels **worse** (val MSE 762 720 against
724 743) and better only on a rank correlation the matrix measures flat across the family?
Q-2 the previous review's Q-4 is undischarged: the ROADMAP still places SPSA/Texel in Stage 4 and
now carries this phase in Stage 2, with no ADR moving it. Q-3 is the dry-run criterion falsifiable
by the whole class it names — `weights_sha256` is a second read of the same file, so it cannot
falsify "read the file for the digest and evaluated with something else"; the non-degenerate
pentanomial already in the artifact would. Q-4 what licenses committing the candidate as
configuration before the run, given the quoted words are the committed table's own header and not
D-11's text?

## What the reviewer attacked that SURVIVED

1. **The tempo-cancellation claim, attacked hardest and surviving more completely than the design
   claims.** A patched engine taking a mover-relative constant from an env var, **control-checked
   both ways** (the patched binary's root score moves `−76 → +290 → +4924` at constants 0/366/5000;
   the shipped one does not move at all, and node counts, depths and pv are identical in every
   run). A draw **stratified by `score_kind`** — 70 `eval`, 70 `mate_in`, 70 `mated_in`, each
   spanning all sixteen tranches, with **41 of the 70 `mate_in` positions returning a mate score at
   the registered seat** — over constants `+1, +10, +100, ±366, +1000, +5000, +12999, +13000,
   +15000` (past the 13 000 mate gap), two weight tables, at `nodes 50000` and again at
   `nodes 400000`: **0 bestmove changes in 6 720 searches, with the full depth/seldepth/nodes
   stream identical in every one.**
2. **The candidate table**, by two genuinely independent instruments plus a dense integer grid, and
   the shipped `fit.py` run on the reviewer's own row file printing every line identically.
3. Every count D-622 and D-626 carry, at the reviewer's own scope.
4. The oracle: 41 215 / 39 195 reproduce; receipt 12 of 12; the six-stone stratum is empty so §5's
   wording is coextensive with the code's predicate; the 2 369-in-387-groups ground is genuinely
   the `key_full` number (`key_seq` 2 323/364, `key_pos` 0/0).
5. The dry-run arithmetic, recounted from the raw game lines: 0.4583 / 0.2500, 2.044 s/opening,
   13.6 minutes, 45.0 % more wall.
6. The engine side: the seats differ in exactly one line; `binary_sha256` matches HEAD's binary;
   three call-site tests pass; gates 6, 18 and 21 green.
7. The holdout: ledger row and arena config in one commit; `3500..3899` disjoint from `13..3499`;
   enforced mechanically by `no_tranche_reaches_the_reserved_holdout`.
8. D-483's line is held in §1-§8.
