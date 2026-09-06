# WP-2.2 Phase 1 — the quiet-term fit of eval v0: design and run registration, revision 5.

> **REVISION 4 UNDER THE OPERATOR'S GRANT (D-631), AND EVERY FIX HERE WAS RUN
> RATHER THAN ARGUED.** Revision 3 FAILED its fresh REVIEW-design with five
> MAJOR (`wp22_phase1_design_rev3_REVIEW.md`) and D-630 recorded the STOP; the
> operator granted at most two further rounds on the condition that the next be
> clean. The three failed rounds share one shape — a fix that discharges a
> finding's SENTENCE and re-creates its PROPERTY one step to the left — so
> nothing below is asserted about an instrument that has not been executed.
>
> - **§9's power is MEASURED**, with the project's own shipped simulator, which
>   existed in the tree the whole time and which revision 3 never ran. The
>   registered 400-pair / `elo1 10` test had power **0.0015**.
> - **§9's alternative and its turn cap now come from RULES stated before the
>   readings**, and the cap's measurement is taken on a SELF-MATCH carrying no
>   arm information.
> - **§2 separates the two grounds** for pinning `w4` and `w5` (D-629).
> - **The committed dry-run config has actually been run** and its report
>   replaces the scratchpad ones.

## §1 What this phase changes, and what it does not

**It may change at most TWO integers** in the committed weight table, and only
on h1: §4 pins the third at its committed value. It changes no code on the search path, no config key, no schema and no
protocol line. It adds offline tooling under `tools/texel/`, which **gate 18
already runs**; three behaviour-named Rust tests; one candidate weight document;
and two arena configs with the ledger row that spends their openings.

**The loader it would otherwise have had to build already exists** (premise §4),
so this phase is smaller than the dispatch assumed and its risk sits in the
trainer, which is offline, and in the SPRT, which is the only voice (D-614).

## §2 The split R6 fixes, and why it is a division of labour

D-621 measured that the top table entry is **not identifiable** from this
corpus: game rule 4 completes a win the instant a stone forms six and a corpus
position is recorded at a turn boundary, so the mover never holds a live
five-window and the mover-relative regressor takes one sign only. R6 (D-622)
generalises that into a rule.

- **TACTICAL — `w4`, `w5`, and they are pinned for TWO DIFFERENT REASONS**
  (D-629, correcting a sentence this section stated for both). **`g5` has no
  sign variation anywhere in the corpus** — measured, 0 positive of 89 805. The
  reason is NOT that the configuration cannot occur: **the mover owns a live
  five-window in 11 positions** (D-626's own collateral correction), all of them
  mate rows in which the opponent owns one too, so `g5` — a DIFFERENCE — is
  never positive. Five stones is not six, so game rule 4 forbids nothing here;
  what makes the class rare is that the opponent usually blocks. Revision 4 said
  such a position *"cannot be recorded at a turn boundary at all"*, and those 11
  rows are exactly what a successor revisiting the `w5` pin would need. **`g4` has no
  sign variation in the FITTED POPULATION**, which is a weaker and different
  fact: it is positive in 1 593 positions of the corpus, every one of them a
  `mate_in` row that §3's clause 1 removes. So the positive evidence for `w4`
  exists and this phase's own exclusion is what drops it. **What pins both is
  the same measurement**: on the fitted rows their regressors are identically
  zero, so the objective's curvature in them is exactly zero.
- **QUIET — `w1`, `w2`, `w3`.** What the search cannot resolve inside its own
  window and must ask the evaluation about.

**The tactical entries are carried VERBATIM from the committed table, and the
reason is a measurement rather than a preference**: under §3's filter the
tactical regressors are identically zero on every fitted row, so the objective's
curvature in `w4` and `w5` is exactly zero and no feasible point is preferred to
any other. R6's alternative — dominance constraints — BOUNDS those entries and
cannot choose among the points it admits, so it survives here as a registered
CHECK on the assembled table: `w4 > w1 + w2 + w3` and `w5 > w1 + w2 + w3 + w4`.

## §3 The row filter, as a predicate on the schema

For a corpus record, let `a_k` be the number of length-6 windows holding exactly
`k` P1 stones and no P2 stone, and `b_k` the same for P2. A row is **fitted**
when all of:

1. `score_kind == "eval"`. A mate score is the search's band and is not a number
   this table can produce (premise §6). **This clause is what makes the fitted
   population free of decided positions**, and D-626 measures that it is the
   only clause that does.
2. `a_4 == b_4 == a_5 == b_5 == a_6 == b_6 == 0`. **What this removes is the
   rows on which the tactical regressors are non-zero** — and D-626 measures
   that it removes nothing FORCED: every dropped row's threats are killable
   within the two stones the turn provides. R6's own words are *"neither side
   holds an in-window forced win"*; that description is wrong about this
   predicate and the predicate is kept on the one-sidedness ground instead.
3. `|label| >= EVAL_MAX` excludes the row: the prediction is the band edge
   whatever the weights do. **Whether this clause ever fires is reported**,
   because a rule that never fires is not doing the work its presence claims.

**The populations of every clause are printed by the shipped fit** and receipted;
per D-483 they are not repeated here. **The fit refuses**, by named error, if the
filtered population is empty.

## §4 The trainer: the tempo term is fitted so that it cannot hide in the weights

**The model is linear in the weights** on the filtered rows (premise §2), so the
squared-error fit has a closed form. No seed, no learning rate, no
initialisation and no stopping rule enters the answer.

**The model that produces the answer**, and both of its departures from revision
2 are forced by measurement recorded in the matrix's §3:

```
label  =  c  +  w1·g1  +  (TOTAL-TOP-w1)·g2  +  TOP·g3        free in (w1, c)
```

- **`c` is a mover-relative TEMPO term the v0 schema has no entry for.** On the
  fitted population the mover is systematically behind in window count and ahead
  in label — it is about to place two stones — and a model with no constant can
  absorb that only by moving the weights. Fitting `c` and then DISCARDING it is
  what stops the weights paying for it. **A mover-relative constant is added to
  every sibling at a ply and negated at the next, so under negamax at a fixed
  depth it cancels exactly between the moves being compared**; the committed
  instrument config carries `q_depth_turns = 0`, `extension_budget = 0` and
  `lmr_min_depth_turns = 0`, so an iteration's leaves sit at one ply. A term that
  cannot change a move is the worst thing to spend the weight vector on.
- **`TOP` holds `w3` and `TOTAL` holds the quiet SUM, both at their committed
  values.** The filtered rows carry no evidence about ANY quiet-to-tactical
  exchange rate — the tactical regressors are identically zero on every one of
  them — so a single pin holds one rate and moves the others. Holding both
  leaves exactly the one degree of freedom the corpus can speak to: how the
  committed quiet total divides between the one- and two-stone entries. This is
  D-627's replacement principle applied fully rather than stopped at the first
  pin, and `matrix_wp22_quiet_scale.md` §5 is where it is argued.

**The no-intercept free-scale solve is computed and REPORTED anyway**, as the
contrast: the difference between the two is the phase's finding, and a
successor who reads only the answer would not see it.

**Stages, each a pure function of the one before**, each an instrument whose
governing revision is this document's commit:

1. `tools/texel/features.py` — the engine's window bookkeeping, replicated.
   **Gains `per_side_counts`**, without which §3's clause 2 cannot be stated.
2. `tools/texel/extract.py` — walks the deduped manifest, joins to the corpus
   records, verifies `key_full` on EVERY row, and writes per-side counts with
   **mate rows KEPT and labelled by kind**, so every clause of §3 is applied
   downstream and can be counted. Every corpus digest it read goes in the row
   file's header.
3. `tools/texel/fit.py` — the exact constrained solve, the pin, the dominance
   check and the diagnostics.
4. `tools/texel/verify_against_engine.py` — §5's oracle.

**The constraint set, and the two corrections this revision makes.**
`crates/pistol-eval/src/weights.rs` requires `w1 >= 1`, strict increase, and
every entry strictly below the decided window's value.

- **Every constraint is an active-set MEMBER, bounds included** (review M-4).
  Revision 2 enumerated the gap constraints and then CLAMPED at the bounds
  before the feasibility test — the same defect its own paragraph called fatal.
  The minimiser is now found by enumerating subsets of the whole constraint set,
  solving each equality-constrained KKT system exactly, and keeping the feasible
  minimiser; for a convex quadratic that is the exact optimum.
- **The pivot threshold is absolute and small, and the equilibration that used
  to sit here is DELETED.** Revision 4 claimed row equilibration *"is what makes
  the pivot test invariant"*; it does the opposite, driving the constraint
  columns down rather than up, and the routine's scale tolerance came from the
  threshold's headroom instead. **The two figures revision 5 registered in its
  place are also deleted**: they were measured on a synthetic system written for
  the probe, not on the one `constrained_min` builds, and a range measured on
  `schema_constraints` does not describe the `tempo_constraints` problem the
  answer path actually solves. What remains is what a test pins and nothing
  more — the minimiser is unchanged under the scalings the test applies, and
  beyond them the routine refuses rather than answering.

**Nothing is projected and no skip is silent.** `fit.py` REFUSES by named error
when the filtered population is empty; when a regressor takes ONE SIGN on the
fitted rows — which is D-621's condition as a machine check, and it is on the
SIGNS because one-sidedness leaves the normal matrix well conditioned and a rank
test cannot see it. **That guard sits on the CONTRAST solve and not on the
answer path**, which computes the pinned-intercept model instead; it is reached
on every run because the contrast is always computed, and a successor who
deleted the contrast would remove the guard with every test still green; when the real-valued answer it is about to round is not
schema-feasible; and when rounding would carry an entry onto the pinned ceiling — the last of
which is UNREACHABLE on the registered path, where the ceiling is the pinned
tactical entry and the quiet entries sit far below it. It is kept for a caller
that pins a smaller one, and it is named as unreachable here rather than left to
look like coverage.
Singular active sets are COUNTED and reported rather than swallowed.

**There is no post-hoc optimality check beside the enumeration, deliberately.**
A cheap one passes vacuously wherever the optimum is interior, which is where
this runs, and reads as verification that did not happen. `test_texel.py` checks
the enumeration against a **brute-force grid** instead, which is an oracle rather
than a restatement. **The rounding guard, by contrast, EXISTS, and the theorem
that said it could not fire is FALSE.** Python rounds half to EVEN, so a feasible
pair one apart on a half-integer rounds to the same integer twice — `[1.5, 2.5]`
both become 2. `round_to_schema` checks its output, and `test_texel.py` draws in
half-integer steps so the tie is reachable and asserts the guard fires. An
earlier revision asserted the opposite in this paragraph and pinned it with a
draw that could not produce a tie; it is the one finding of D-630's four that
survived three fix rounds in this document.

**The split is content-derived**: train and validation are separated by the last
hex digit of the position's `key_full` digest, a **1-in-8** validation slice. No
seed. It is per-POSITION and not per-game; with three parameters and a
five-figure row count overfitting cannot arise, and that — not "it answers
overfitting" — is what the split is worth here (review m-10).

## §5 The correctness gate the fit rests on

`features.py` re-implements the engine's window bookkeeping in another language,
and a re-implementation that silently disagrees would produce a confident fit to
the wrong features.

**The oracle ships as `tools/texel/verify_against_engine.py`** — the name
`features.py`'s docstring has promised since revision 1 while no such file
existed (review M-5). It is driven by `test_texel.py` and its run is receipted.

**The sample is REGISTERED rather than drawn at run time**, because the default
draw is nearly blind to the entries the phase is about. It is the union of:

1. **every** corpus position holding a four- or five-stone window, mate rows
   included — the stratum the finding is about;
2. a **stride draw** across all sixteen tranches and all three score kinds;
3. the same positions re-run under weight tables that **saturate the clamp** on
   both sides.

**Nothing is deduped by `key_full`**: rows sharing one are not rare, and folding
on it would silently shrink the registered sample.

**The criterion**: the offline value equals the engine's own
`HandcraftedV0::value` on the same position, exactly, for every position and
every table. **The defect class it excludes** is a Python/Rust bookkeeping
divergence, and the engine side is an externally derived referent that does not
share the Python enumeration. **The mutant that must die**: a disagreeing engine
stops the oracle by name, which `test_texel.py` drives with a stub whose value
is computed by a formula sharing no code with `features.py`.

**A single mismatch stops the phase.** It falsifies premise §2.

## §6 The engine side: nothing is built, and two TESTS say so

The dispatch's "byte-identity when the key is absent" **cannot be built**: hard
rule 1 makes an absent key a named error and there are no code-side literals to
fall back to. Revision 2 called the two replacement properties "tests" while
nothing in the tree re-ran them (review m-4). They land here as behaviour-named
Rust tests driving the call site (D-553):

1. **Inert to presentation.** A weights document differing only in comments and
   whitespace gives byte-identical engine output at a fixed instrument budget,
   with the two documents asserted to differ as BYTES first — or the test
   compares a file with itself.
2. **Not inert to values** — the call-site mutant that must die. One perturbed
   entry gives different output.
3. **The candidate table loads and changes the search**, so an SPRT against it
   is a measurement rather than a self-match.

## §7 Diagnostics, which gate nothing (D-614)

Train and validation MSE, rank correlation with **tied ranks averaged** (review
m-9), the fitted ratios against the committed ones, the population of every §3
clause, the **saturation rate** premise §3 registered and revision 2 dropped,
and a **by-depth breakdown**, which revision 2 promised while `fit.py` read the
depth column and discarded it. **They live in artifacts, never in a document**
(D-483). They answer whether the trainer is broken, not whether the engine is
stronger.

**The human corpus is not a holdout here and revision 2's paragraph claiming it
was is DELETED** (review M-6). `docs/ROADMAP.md` blocks D-434's Stage-2
calibration holdout until a population-grade corpus supersedes the
artifact-grade one, and D-453 licenses statements about the artifact and not
about the platform's players. Deleted rather than restated (D-424).

## §8 Bench

The eval's cost is unchanged **by construction**: the same terms, the same
windows, the same table lookup, different integers in the table. A registered
spot-check runs anyway, because "by construction" is an argument and the bench
is a measurement. Expected bracket: **no change outside noise**; a measured
change is a finding that something other than the table moved.

## §9 THE RUN REGISTRATION — complete in method, and NOT LAUNCHED

**The method below is registered. The run is not.** Revision 4 registered 400
openings of D-568's reservation; §9.2's power analysis is what withdrew them,
and `docs/book_v2_ledger.md` records the withdrawal rather than erasing it.
The governed arena config was removed with
the claim, so no committed config names the slice — which is why this paragraph
does not name it either.

**Seats.** One binary, two configs differing in exactly one key, `weights_file`:
`configs/instrument_v0.toml` against `configs/instrument_quiet_fit_v0.toml`. The
arena's handshake carries each seat's `weights_sha256`.

**The candidate is `[4, 10, 60, 300, 1500]`** — the two-pin minimiser of
`matrix_wp22_quiet_scale.md` §5, holding the top quiet entry AND the quiet sum
at their committed values so that the only thing the comparison varies is the
one degree of freedom the corpus has an opinion about. It is COMMITTED
CONFIGURATION, not an artifact (D-11), at
`configs/eval_v0_quiet_fit_weights.toml`.

**Fixed by this registration**: `budget` nodes 50 000; `n_workers` 4;
`hang_timeout_ms` 120 000; `alpha` = `beta` = 0.05; `turn_cap` **60** by §9.1;
`elo0` = 0. **Not fixed, because they are what the operator's ruling decides**:
the openings source and take, and therefore `elo1`, which §9.2's rule derives
from the pair cap the openings allow.

### §9.1 The turn cap, by a rule stated before its reading

**THE RULE**: the smallest value in `{40, 60, 80}` whose capped fraction is at
most **0.30**. **THE MEASUREMENT, on a SELF-MATCH carrying no arm information** —
the committed engine in BOTH seats, 24 openings of `random_openings_v1.txt` at
`openings_skip = 100`, a different slice from the dry run's:

| `turn_cap` | capped fraction | wall, 24 openings |
|---|---|---|
| 40 | 0.375 | 35.9 s |
| **60** | **0.250** | 46.1 s |
| 80 | 0.208 | 56.2 s |

The rule returns **60**, the same value revision 3 read off the arm-carrying dry
run. **The answer did not change; the ground did.** Two limits are registered
rather than left to be found: the threshold 0.30 was written by an author who
had already seen 0.4583 and 0.2500, so the self-match removes the ARMS'
information and not the author's; and the self-match understates the registered
workload's own cap-40 fraction by 0.083, so it is a proxy whose disagreement
with the workload is real and unmodelled.

### §9.2 The power, MEASURED — and why the run is not launched

**Instrument**: `crates/pistol-arena/examples/sprt_power.rs`, invoked as
`--buckets 2,3,8,7,4 --runs 20000 --seed 1`. **Those buckets are the SUPERSEDED
dry run's, not the registered one's** — `dry_60.txt` ran the withdrawn
`[5, 34, 60]`; the committed dry run at the registered candidate returns
`3, 4, 13, 1, 3` and leans **−28.12 nelo**. Re-measured on the correct shape the
table moves by under 0.005 and **the selection rule returns the same answers**
(50 at 400 pairs, 8 000 for `elo1 = 10`), so no conclusion here depends on it —
but the provenance was wrong and is recorded rather than quietly re-run. Receipted at
`artifacts/wp22_phase1_quiet/POWER.txt`.

| pairs | `elo1` | power at `truth = elo1` | alpha at `truth = 0` | inconclusive |
|---|---|---|---|---|
| **400** | **10** | **0.0015** | 0.0000 | 0.9985 |
| 1 000 | 10 | 0.0866 | 0.0044 | 0.9091 |
| 4 000 | 10 | 0.6970 | **0.0386** | 0.2670 |
| 8 000 | 10 | **0.9045** | **0.0493** | 0.0484 |
| 400 | 40 | 0.8625 | 0.0558 | 0.0954 |
| 400 | 50 | 0.9335 | 0.0656 | 0.0211 |
| 1 000 | 30 | 0.9189 | 0.0518 | 0.0328 |

The two bold alpha cells are corrected: revision 4 carried the `h0` rates from
the `truth = elo1` runs beside them, which is a different statistic.

**THE RULE for the alternative**: the smallest value in `{10, 20, 30, 40, 50, 60}`
whose measured power at the pair cap is at least 0.90. **Two of its parameters
are unjustified and are named as such**: the threshold 0.90 is not the
registration's own `1 - beta = 0.95` (at 0.95 the grid returns 60), and the
grid's step of 10 does work (on a step-1 grid the answer is 44). Both happen to
fall toward a more informative test.

**WHAT AN SPRT VERDICT LICENSES, and revision 4 stated this falsely.** An SPRT
decides between two simple hypotheses; error control holds at the endpoints
only. At 400 pairs against `elo1 = 50`, measured:

| true effect | h1 | h0 | inconclusive |
|---|---|---|---|
| 0 | 0.0656 | **0.9149** | 0.0196 |
| 10 | 0.1566 | 0.7764 | 0.0669 |
| 20 | 0.3332 | 0.5436 | 0.1231 |
| 30 | **0.5856** | 0.2978 | 0.1167 |
| 40 | 0.8094 | 0.1273 | 0.0634 |
| 50 | 0.9335 | 0.0454 | 0.0211 |

**h1 does NOT mean "at least 50 normalized Elo better"** — it is already the
modal outcome at a true effect of 30. And the crossing thresholds, derived from
`sprt.rs`'s own constants (`t1 = 0.203522`, boundaries `±ln(19)`): at n = 400,
h1 needs an observed `nelo_pair ≥ +33.89` and **h0 needs only `≤ +16.11`** — so
the run would report h0 for a candidate up to about +16 normalized Elo BETTER
than the committed table.

**WHY THE RUN IS NOT LAUNCHED, and it follows from the table above.** Under this
document's own registered expectation — null-to-small in either direction —
the outcome is **h0 with probability 0.91**, and h0 licenses only *"not 50
normalized Elo better"*, which is a proposition this document already states it
believes. The second most likely outcome is h1 at 0.0656, which at `truth = 0`
is a false positive. **The run's expected information is low and its openings are
scarce and claimed**, and R7's actual question — `elo1 = 10` — needs **8 000
pairs**, hence a `book_v3` of about **8 500 openings** (`ceil_to_500(8000 + 500)`
by `book_v2_registration.md` §4's own rule), which is a package: `BookVersion`
has no `V3` variant.

**THE QUESTION THIS PUTS TO THE OPERATOR**, which D-630 has been waiting on and
which is now answerable with numbers rather than adjectives: *spend 400 of the
1 000 reserved openings on a screening test whose most likely outcome restates
the prior, or generate `book_v3` and answer R7 properly, or neither.* **It is not
taken here** because the reservation is shared and the ruling is the operator's.

**THE DRY RUN**, on an input of the same kind that is not a registered workload:
24 openings of `random_openings_v1.txt` at `openings_skip = 0`, from
`configs/arena_wp22_phase1_quiet_dryrun.toml` **as committed** — the report's own
`timing config_sha256` is that file's digest.

- **THE CRITERION, corrected**: the report's pentanomial must be
  **NON-DEGENERATE**, and its verdict must not be `inconclusive_degenerate`.
- **The defect class**: both seats silently evaluating with the same weight
  table, which makes the SPRT a self-match (D-156).
- **Why the previous criterion did not work.** Revision 4 registered the two
  seats' `weights_sha256` differing. `crates/pistol-cli/src/bin/pistol.rs` says
  in terms that this digest is *"a SECOND read of the file, after the eval loaded
  it, not a digest of the bytes the eval parsed"* — so the named defect leaves
  the two digests different and the criterion green. It is a property the defect
  PRESERVES, which `docs/process.md` says is not a criterion.
- **Why the pentanomial does work, and it is already receipted**: the self-match
  of §9.1 returns `p0 0 p1 0 p2 24 p3 0 p4 0` and
  `verdict inconclusive_degenerate`; the two-table dry run returns
  `p0 3 p1 4 p2 13 p3 1 p4 3` and `inconclusive_at_game_cap`. It discriminates
  perfectly and costs nothing.
- **Result: the criterion is MET.** Receipted in
  `artifacts/wp22_phase1_quiet/DRYRUN.md`.

**The honest expectation, and one thing it must not hide.** The candidate fits
the labels WORSE than the table it would run against — validation MSE **728 957**
against 724 743, train **719 751** against 715 454 — because the tempo term it
was fitted with is discarded before the diagnostic. That is coherent with §4 and
D-614 makes it gate nothing, but it is what was known before any run.

**Why the labels are 400 000-node searches and any verdict would be at 50 000**:
the label is a TARGET and the seat is an INSTRUMENT. The seat is the standing
budget every other strength claim in this project was taken at. The gap is a
registered limit: a table fitted to deep scores may suit a deeper seat better
than the one it is judged at, and this design does not measure that.

## §10 What this design does not decide

**The censored-likelihood successor revision 2 named is RETIRED before it was
built** (D-621, review M-3). Revision 2 said the exclusion drops the rows where a
five-window CONVERTED; measured, the direction is inverted. Adding those rows
back adds no sign variation, so a censored likelihood cannot repair a regressor
that has nothing to identify. **What replaces it is the split of §2.**

**Nothing here decides Phase 2**, whose premise memo is
`wp22_phase2_premise.md` and whose measured finding is that the ROADMAP's
length-11 codebook is not supported by this corpus.

**Nothing here decides the quiet-to-tactical balance.** §4 pins it because the
filtered rows carry no evidence about it; that is a choice no evidence in this
corpus can make, and it is registered as a limit rather than argued away.

**Nothing here licenses reading the discarded tempo term as a finding about the
game.** It is a nuisance parameter estimated from labels whose unit is the
committed table's own.
