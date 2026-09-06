# WP-2.2 Phase 1 — the quiet-term fit of eval v0: design and run registration, revision 4.

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
  sign variation anywhere in the corpus**, and the reason is structural: game
  rule 4 completes a win the instant a stone forms six, so a mover holding a
  live five-window cannot be recorded at a turn boundary at all. **`g4` has no
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
label  =  c  +  w1·g1  +  w2·g2  +  PIN·g3            free in (w1, w2, c)
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
- **`PIN` holds `w3` at its committed value.** The filtered rows carry no
  evidence whatever about the quiet-to-tactical balance, and an unpinned
  absolute scale sets that balance anyway — from labels whose unit is the
  committed table's own (the matrix's §3(a), verified by digest).

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
- **The pivot test is scale-relative.** An absolute threshold against a KKT
  system that mixes the normal matrix with constraint rows of order one reads
  the small rows as singular, and whether it does so depends on the labels'
  units rather than on the problem. A test pins that the minimiser is invariant
  when the objective is scaled.

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
than a restatement. For the same reason there is no guard against a rounding
clamp: rounding a feasible answer cannot break the schema's increase, so a guard
for it could never fire, and a test pins that property over two thousand draws.

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

## §9 THE RUN REGISTRATION — SPRT, the only voice

**Seats.** One binary, two configs differing in exactly one key, `weights_file`:
`configs/instrument_v0.toml` against `configs/instrument_quiet_fit_v0.toml`. The
arena's handshake carries each seat's `weights_sha256`, so the two tables are
pinned by content in the report.

**The candidate is `[5, 34, 60, 300, 1500]`** — what the shipped instrument
prints, which is the constrained minimiser with `w3` pinned inside the
regression rather than a rescaling of a free solve. **It is COMMITTED
CONFIGURATION, not an artifact**, at `configs/eval_v0_quiet_fit_weights.toml`. D-11 says so of a table of this kind
in the committed table's own words — *"a handful of integers an operator can
read and edit"* — and `tools/config_check.sh` refuses an engine config whose
weights document it cannot read, so a gitignored candidate would fail gate 6 in
a clean checkout and make the run unreproducible from the tree.

**Arena config**: `configs/arena_wp22_phase1_quiet.toml`.

| key | value | why |
|---|---|---|
| `budget` | `nodes`, 50 000 | the standing instrument budget; node-matched |
| `openings_file` | `random_openings_v2.txt` | the successor book |
| `openings_skip` / `openings_take` | **3500 / 400** | the reserved holdout's first 400. The fit's corpus was drawn from `13..3499`, so this slice is unseen BY CONSTRUCTION |
| `turn_cap` | **60** | an evaluation horizon, never a game rule (rule 6). Fixed by the RULE and the SELF-MATCH of §9.1, not by a reading of the arm comparison |
| `n_workers` | 4 | as every prior governed arena run |
| `hang_timeout_ms` | 120 000 | the liveness watchdog; it can end a run and can never produce a result (D-159) |
| `elo0` / `elo1` | **0.0 / 50.0** | fixed by the RULE and the measured power of §9.2. NORMALIZED Elo, which is not the unit of the ROADMAP's +150 Elo bar |
| `alpha` / `beta` | 0.05 / 0.05 | as every prior SPRT in this project |
| maximum pairs | **400**, the take | a cap the registration fixes, not the run |
| measured power | **0.9335** at `truth = elo1`; alpha **0.0656** at `truth = elo0` | §9.2, and both are on this document's face because a registration that states its cost and not what it can conclude is incomplete (D-628) |

**The ledger row lands in the same commit as this config** —
`docs/book_v2_ledger.md`'s own rule, which revision 2 did not meet (review M-2).
The row records `3500..3899` as consumed. **The "if the holdout is thin" clause
is DELETED**: it named no threshold and so left the openings source to be chosen
at run time.

**Reported**: n, distinct_n, pentanomial, llr_pair, verdict, per-side compute.

**A THIRD OUTCOME IS REGISTERED, because two are not enough.** An SPRT at these
bounds need not cross either within 400 pairs. **That outcome is neither h0 nor
h1 and licenses neither**, and its measured probability is in §9.2. What bounds
the run at 400 is the HOLDOUT and not the clock. **What it does not block**:
Phase 2, which proceeds on any of the three outcomes (R7, D-623).

### §9.1 The turn cap, by a rule stated before its reading

Revision 3 chose `turn_cap 60` from the dry run's capped fractions — a reading
taken after the numbers existed, in the same two runs that showed which cap
favoured the candidate. That is the post-hoc threshold move the Process section
forbids, whether or not it biased anything, and it is replaced.

**THE RULE, stated first**: the cap is the smallest value in the grid
`{40, 60, 80}` whose capped fraction is at most **0.30**.

**THE MEASUREMENT, on a SELF-MATCH that carries no arm information**: the
committed engine in BOTH seats, 24 openings of `random_openings_v1.txt` at
`openings_skip = 100` — a different slice from the dry run's — so the run cannot
prefer a cap on the arms' behalf.

| `turn_cap` | capped fraction | wall, 24 openings |
|---|---|---|
| 40 | 0.375 | 35.9 s |
| **60** | **0.250** | 46.1 s |
| 80 | 0.208 | 56.2 s |

The rule returns **60**. **It is the same value revision 3 chose, and saying so
is the point**: what changed is the ground, not the answer, and a rule that
happens to confirm a previous reading is still the only thing that makes the
reading defensible.

### §9.2 The power, MEASURED — and what this run cannot answer

**Revision 3 registered bounds, a cap, a cost and a dry run, and never computed
what the test could return.** `crates/pistol-arena/examples/sprt_power.rs` is a
shipped instrument that has been in the tree since the book_v2 registration and
was not run. It is run here, on the DRY RUN'S OWN PENTANOMIAL `2, 3, 8, 7, 4`,
which is this engine pair's own pair-outcome shape rather than a coin's.

| pairs | `elo1` | power at `truth = elo1` | alpha at `truth = 0` | inconclusive |
|---|---|---|---|---|
| **400** | **10** | **0.0015** | — | 0.9985 |
| 1 000 | 10 | 0.0866 | — | 0.9091 |
| 4 000 | 10 | 0.6970 | 0.0360 | 0.2670 |
| 8 000 | 10 | **0.9045** | 0.0471 | 0.0484 |
| 400 | 40 | 0.8625 | 0.0558 | 0.0954 |
| **400** | **50** | **0.9335** | **0.0656** | 0.0211 |
| 1 000 | 30 | 0.9189 | 0.0518 | 0.0328 |

**THE RULE, stated before the reading**: the alternative is the smallest value
in the grid `{10, 20, 30, 40, 50, 60}` whose measured power at the registered
pair cap is at least **0.90**; if no value reaches it, the run is not
registered. At 400 pairs the rule returns **50**.

**WHAT THIS RUN CAN AND CANNOT CONCLUDE, registered so that neither is read
after the fact.** h1 says the candidate is at least 50 normalized Elo better,
which would be a large effect and a real finding. **h0 says only that it is not
50 normalized Elo better.** It does **NOT** answer R7's *"does this corpus move
Elo at all"*: that question is `elo1 = 10`, and at 400 pairs its power is
**0.0015**. A reader who takes h0 here for R7's h0 has taken a much weaker
statement for a much stronger one.

**The alpha at the registered point is 0.0656 against a nominal 0.05**, because
a capped sequential test truncates. It is stated rather than left to be found.

**THE SUCCESSOR THAT WOULD ANSWER R7, sized rather than gestured at.** `elo1 = 10`
reaches power 0.90 at **8 000 pairs** — measured, not extrapolated. That is
about **4.5 hours** at the dry run's measured 2.04 s per opening, which is
affordable; what is not available is the openings. `docs/book_v2_ledger.md`
holds 600 unspent after this run, and the sweep's `13..3499` are the fit's own
training openings. So it needs a **`book_v3` of about 8 500 openings** —
`ceil_to_500(8000 + 500)` by `book_v2_registration.md` §4's own rule — and that
is a PACKAGE and not a paragraph: `BookVersion` in
`crates/pistol-cli/src/random_openings/mod.rs` has no `V3` variant, so it needs
a code change, a registration under book_v2's discipline, and a committed
fixture.

**COST, measured rather than remembered.** The dry run below took 49.0 s of wall
for 24 openings at 4 workers with `turn_cap 60`, so the registered 400 openings
is **ESTIMATED at 14 minutes** from a MEASURED 2.04 s per opening. If the
governed run exceeds one hour it is stopped and the registration is amended,
which reopens this review.

**THE DRY RUN**, on an input of the same kind that is not the registered
workload (`docs/process.md`): 24 openings of `random_openings_v1.txt` at
`openings_skip = 0`, every other key identical. `book_v1` is retired for GOVERNED
use (D-505) and a dry run is not a governed use.

- **The criterion**: the report's two seats carry **different** `weights_sha256`
  values, each equal to the sha256 of the document its config names; both seats'
  compute is non-zero; and a verdict line is written.
- **The defect class it excludes**: both seats silently evaluating with the same
  weight table, which would make the SPRT a self-match — every pair scoring
  alike, no likelihood ratio defined, and a null that looks like a finding
  (D-156).
- **Why the criterion is not vacuous**: the digests come from the engines' own
  identity lines, not from the configs the arena read, so they are an externally
  derived referent that the named defect would falsify.
- **The registered consequence of failure**: the run does not launch and the
  phase returns to this document.
- **Result: the criterion is MET.** Receipted in
  `artifacts/wp22_phase1_quiet/DRYRUN.md`.

**The honest expectation, registered before the run.** The v0 feature set is
three numbers describing how many windows hold how few stones, with the two that
describe threats pinned. It cannot see shape, cannot see whose turn it is beyond
the sign, and cannot see a threat. **The expectation is null-to-small in EITHER
direction**, and against an alternative of 50 normalized Elo that makes h0 the
likely outcome. Three things are registered rather than left to be explained
afterwards:

- the ROADMAP's Stage-2 bar of **+150 Elo is a different unit** from this
  registration's `elo1 = 50` normalized Elo, and the two are not comparable;
- **the dry run leaned POSITIVE for the candidate** (`nelo_pair +71.67`,
  `ci95 ±98.29` at 24 pairs), so a positive governed result is not a surprise;
- and **a third receipted run leaned NEGATIVE** — `dryrun_report.txt`, four
  openings at `turn_cap 40`, `nelo_pair −41.53`. Recording only the two that
  agreed with each other would be the selection this paragraph exists to
  prevent. All three are inconclusive and none is evidence.

**There is no pre-named second arm.** The matrix's revision 2 named one on a
diagnostic that round 2 measured to be a scale statistic, and naming a second arm
on such a number is D-614's own case one arm later.

**WHAT EACH OUTCOME LICENSES, and h0 here is WEAKER than R7's h0.** This is the
correction §9.2 forces and it is the one a reader is most likely to get wrong:

- **h1** — the candidate is at least 50 normalized Elo better. A large effect,
  and a real finding: the corpus moved Elo. **h1 moves the committed weights**,
  with the pin re-recorded and R4's cap re-test scheduled (D-613).
- **h0** — the candidate is **not** 50 normalized Elo better. **It does NOT say
  the v0 feature set is the limit**, which is R7's h0 and needs `elo1 = 10` and
  8 000 pairs. Reading this h0 as that one takes a weak statement for a strong
  one.
- **inconclusive at the cap** — neither, and its measured probability is 0.0211.

**Phase 2 proceeds on all three** (R7, D-623 as amended by D-627 and D-632), so
none of them gates anything downstream; the run's value is evidential.

**WHAT BECOMES OF THE TWO COMMITTED CONFIGS ON A NON-h1 VERDICT**, because
otherwise the tree keeps a gate-validated engine config pointing at an
unaccepted table with no document saying so: `configs/eval_v0_quiet_fit_weights.toml`
and `configs/instrument_quiet_fit_v0.toml` STAY, and the verdict is recorded in
this document's own closure section. They are the pin of a run that happened,
which is the same reason `configs/instrument_r2_v0.toml` outlived the policy it
lost to (D-194).

**Why the labels are 400 000-node searches and the verdict is at 50 000**
(review Q-3): the label is a TARGET and the seat is an INSTRUMENT. The target is
the best estimate of a position's value the project can afford; the seat is the
standing budget every other strength claim in this project was taken at, so a
verdict here is comparable with those. The gap is registered as a limit: a table
fitted to deep scores may suit a deeper seat better than the one it is judged
at, and this design does not measure that.

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
