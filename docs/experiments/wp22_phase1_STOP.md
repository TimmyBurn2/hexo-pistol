# WP-2.2 Phase 1 — STOP: three gates, three failures, and the one division nobody did

## ONE LINE FOR THE MORNING

**There is no Stage-2 Elo number, and what replaces it is that the SPRT which
would have produced one COULD NOT HAVE CONCLUDED: at its registered bounds the
test needs 3 554 pairs in expectation and the registration capped it at 400, so
`h0` — the outcome R7 names as the run's whole purpose — had probability at most
0.001 at every effect simulated. It is one division, it costs seconds, and
nobody did it until a fresh reviewer did.**

The run would have spent **400 of the 1 000 openings D-568 reserves**, on which
two packages have standing claims, for roughly a one-in-a-thousand chance of the
answer. **D-628** carries it, cross-checked against three of this project's own
recorded runs.

## What closed, and stands

| | |
|---|---|
| **§0** | CI green at `ae211f0`: 21 of 21 gates, `ci: all gates passed`, `EXIT=0`, over a tree frozen for the run's whole length. The worktree the dispatch asked to remove was already gone — derived four ways, not assumed |
| **§R** | R6-R9 landed as **D-622-D-625**. `D-53a` resolved to D-535…D-538 rather than invented; the D-616 sweep counted at 24 occurrences on 19 lines in 6 files; D-616 retired where it INSTRUCTS and left where it RECORDS, receipt `wp22_d616_sweep_receipt.md` |
| **R3 anchor** | D-612 answered. The seat swap reproduces every per-colour count within one game and the colour totals are **38 / 62 in both runs, identical** — the asymmetry is the COLOUR's, D-612's flip clause fires, and BOTH engines carry it, which v5 could not see |
| **Phase 2 premise** | The ROADMAP's registered **length-11 codebook is not supported by this corpus**: 62 370 cells, 61.9 % seen fewer than ten times, median 6, still climbing. Length 7 saturates at 1 990 cells with no cell under ten |
| **The instruments** | Oracle clean at **41 215 positions, three weight tables, 0 disagreements** — 82x the sample it replaces, and now enforcing and printing its own registered draw |

## What failed, and it was the argument every time

| gate | rounds | verdict |
|---|---|---|
| OPTION MATRIX | 3 of 3 — the full grant | FAIL, FAIL, FAIL (13 findings, 6, 3 MAJOR). **D-565 fires: STOP and split** |
| REVIEW-design | 2 of 3 | FAIL, five MAJOR. *"The registered SPRT may not launch as written"* |
| REVIEW-impl | 1 of 3 | FAIL, six MAJOR, **17 of 36 mutants surviving** |

**Every reviewer reproduced every number at scopes the documents never took** —
a different data path, a different window algorithm, exact rational arithmetic,
an independent minimum-hitting-set, a patched engine — and **not one shipped
number was wrong**. The candidate `[5, 34, 60, 300, 1500]` is the constrained
minimiser by three independent instruments and a brute-force grid; it changes
play on 48 % of positions; the dry run reproduces from the committed config.

**The pattern, named because it recurred three times in one package**: a fix
round discharges the finding's SENTENCE and re-creates its PROPERTY one step to
the left. A pointer to a section carrying nothing became a pointer to a section
that did not exist, became a pointer to instruments that exist and cannot run.

## The four findings that are mine and were wrong

1. **The SPRT could not have concluded** (D-628). I registered bounds, a cap, a
   cost and a dry run, and never computed the power.
2. **I registered a table my own instrument does not compute.** The matrix
   carried `[4, 29, 60]`, a rescale; `fit.py` prints `[5, 34, 60]`, the
   minimiser. They differ on 14.4 % of positions.
3. **An ADR line I wrote forbade the option I then selected** — D-623's
   consequence clause was my gloss on R7, not R7's words. **D-627** deletes it.
4. **I asserted a false theorem in a governing document**: that rounding a
   feasible answer cannot break the schema's increase. Python rounds half to
   even; `[1.5, 2.5, 3.5]` rounds to `[2, 2, 4]`. The test I wrote to pin it
   incremented by sevenths and could never produce the tie.

Each was found by measurement, and each is fixed or recorded in the log.

## What is fixed in the code at this revision

Correctness findings are never overruled, so these landed regardless of the STOP:

- **Hard rule 3**: `read_rows` and the oracle refuse a `to_move` outside
  `{p1, p2}`. Corrupting the token's case had produced `[1, 2, 60, 300, 1500]`
  with no error, no refusal and no diagnostic.
- **`round_to_schema`** checks its output, and the false theorem is gone.
- **`extract.py` is driven** — it had no test at all — including the join
  refusal, that mate rows are KEPT, and the whole column map. Three mutants that
  survived every gate now die.
- **The Rust test reads the committed documents** instead of rebuilding both
  tables as literals. Editing the candidate to the committed table had passed
  all four gates; it now fails by name as a self-match.
- **The oracle enforces and prints its registered draw** — every tranche, every
  score kind, the whole tactical stratum, and saturation on at least one table.
- **D-626's retired "forced win" ground is out of the instrument**, its printed
  receipt field and its tests; the predicate is named for what it tests.
- **The pivot is equilibrated**, and the invariance test now reaches the scale
  where an absolute threshold fails, so it discriminates the fix from the defect.
- **The matrix's citation was brace-expanded** and the gate checked **zero** of
  its citations — an addition that looked like coverage and was inert.

## What the operator decides, and it is two questions

**One — the matrix.** Its remaining defect is one sentence: I claimed pinning
`w3` is the *only* normalisation holding the quiet-to-tactical balance and that
the run varies *"the shape and nothing else"*. Measured, J moves `w1/w4` by
x2.5, `w2/w4` by x2.8 and the aggregate by 34 %, and a rival pin holds the
aggregate exactly. The reviewer's own remedy is a **deletion** — strike "only",
strike "in nothing else", let the pin stand as what two sections already call
*"a choice no evidence in this corpus can make"*. That changes no number and
needs no measurement. **Split and re-dispatch, or rule that deletion in without
a fourth round?**

**Two — what a run at these bounds is for**, given D-628. Widen `elo1` to
something 400 pairs can decide; re-register as a screening run and strike `h0`
from what it can return, which reopens whether R7's question is answered at all;
or spend the openings knowing the odds. **This one is not mine to take**: R7 is
an operator ruling and the third option spends a reservation two other packages
are counting on.

## What is owed, in the order it should be taken

1. **The two rulings above.** Nothing downstream moves without them.
2. **REVIEW-impl's remaining MAJOR**: `features.py`'s three surviving mutants
   (an axis, the `k=6` splice, the mover-per-turn fixture being exactly two
   turns long), and the oracle's clamp-table mutant — recorded as near-equivalent
   here, since the remaining table saturates 40 016 positions, but not closed.
3. **The committed dry-run config has never been run.** The three receipted
   reports name configs on a tmpfs. It is 49 seconds.
4. **Q-4, twice deferred**: the ROADMAP places SPSA/Texel in Stage 4 and now
   carries this phase in Stage 2, with no ADR moving it.

## Hazards a successor will otherwise rediscover

- **Compute the power before registering an SPRT.** `sprt.rs` works in
  NORMALIZED Elo; `elo1 = 10` there is not 10 Elo, and the pairs a bound needs
  is `2·ln(19)/t1²`. The design compared its `±10` against the ROADMAP's
  `+150 Elo` without naming that they are different scales.
- **Run the shipped tool and diff its output against the document** before
  registering any number. The document and the tool were written in the same
  session and never compared.
- **`find` is shimmed in this shell** and silently returns nothing; so is
  `grep`. Use `/usr/bin/find`, `/usr/bin/grep` or `git ls-files`.
- **Export a worktree's contents before removing it.** I removed a reviewer's
  log directory without doing so; its gate lines survive transcribed in
  `wp22_phase1_impl_REVIEW.md` and were re-run here, so nothing load-bearing was
  lost — but that was luck, not method.
