# Training pipeline for Phase 2 — hex Connect(6,2,1), 2026-09

**What this is.** The practical pipeline, read-only, written to be built from.
Nothing here registers a run or moves a committed file. §7 reviews `tools/texel/`
against the three gaps `wp22_HANDOFF.md` §6 Call 4 leaves open, **with `file:line`
and no fixes**.

**Labels.** `MEASURED` (instrument named) · `DERIVED` (arithmetic shown) ·
`ESTIMATED` · `ARCH` (architectural literature fact — transfers) · `EMP`
(empirical figure, tagged with its game, **NON-TRANSFERABLE**).

## §1 Splits — the current one is per-position, and every project that ships one splits by game

`tools/texel/fit.py:334-342` is honest about it in its own docstring: *"Train/
validation by the position's own key digest, a 1-in-8 slice… **Per POSITION and
not per game.**"* Content-derived, seed-free, deterministic — all good properties,
and the wrong grouping.

Every primary pipeline found splits at the **game** level (**ARCH**): lczero
slices the *chunk file list* (a chunk is a game); Maia splits by 200 000-game
blocks and by year; ChessBench splits by month. Peter Österlund's own accounting
for Texel states the mechanism: with *"about 140 positions per game"*, an error
measure over positions *"has contributions from 64 000 independent events and the
fact that there are 140 times more positions"* does not add independence. **The
effective `n` is games, not positions.**

**MEASURED, and nobody had this number.** Instrument:
`artifacts/research_2026-09/game_count.txt`, walking the corpora the deduped manifest
selects and reading `CorpusRecord.game`. The 89 805 deduped positions come from
**3 487 distinct games** — **25.75 positions per game, median 23** (min 8, max 39), and
every one of the 3 487 carries at least one `eval` row. **So for the outcome-blended
term the effective `n` is 3 487, not 89 805.** For the search-score term each position
carries its own independently computed label, so its effective `n` sits somewhere
between 3 487 and 74 672 and **is not established** — but the *split* must be at game
level either way.

**The corpus already carries the grouping key and the extractor drops it.**
`CorpusRecord.game` is field 0 of the record grammar
(`crates/pistol-arena/src/labels_file.rs:19`); `tools/texel/extract.py:24-25`
names the columns it keeps — `MOVES, KEY_FULL, TO_MOVE, SCORE_KIND, SCORE_VALUE,
DEPTH, BOOK, RESULT` — and `game` is not among them. **A game-level split is one
column in `extract.py`.** Note `BOOK` is a *bool* (*"whether every turn leading
here was a book turn"*, `labels_file.rs:44-45`), not an opening id, so opening-level
grouping needs the report and is a bigger question than game-level.

**The honest counterweight, and it is worth quoting.** ChessBench measured **14.7 %
board overlap between its i.i.d. test set and its training set and deliberately
kept it**, carrying a separate non-overlapping puzzle set for the generalisation
claim (**EMP**, chess). So a per-position split is a defensible *choice* — but it
has to be a stated one, with the generalisation claim carried somewhere else.

**Symmetry augmentation: do not.** For an axis-pooled, **reversal-folded** window
codebook the feature vector is exactly invariant under all twelve lattice
symmetries — MEASURED, 0 of 2 200 images differ
(`docs/research/eval_families_2026-09.md` §0.4), so augmentation adds byte-identical
rows. For an **unfolded** codebook it is not invariant (1 760 of 2 200 differ), and
augmentation there is not a bonus but a *repair*. The corpus is in any case already
symmetry-deduped by `key_full` = `canonical_form` (D-570, `labels_file.rs:26`), so
augmenting re-creates rows dedup removed and inflates any per-position split.

## §2 Labels

**The primary/auxiliary split is already ruled and the literature agrees.** D-562(1):
search-score labels primary, outcome auxiliary and only on the **decided** subset
(*"328 of the pilot's 742 records read `result capped`, and a rule that says use
outcome without saying where outcome exists fits a label to `turn_cap`"*).
Stockfish's trainer default is `lambda = 1.0` — pure score, zero outcome weight
(**ARCH**) — and its two blend forms are given verbatim:

```
wdl_value = lambda_ * wdl_space_eval + (1 - lambda_) * game_result      # blend the target
loss      = lambda_ * loss_eval      + (1 - lambda_) * loss_result      # blend the losses
```

with the docs declining to prefer one: *"Which way works better depends on your
case"*. **Take the second form here**, because it is the one that handles a corpus
where only 52 % of rows carry an outcome: mask `loss_result` on rows without one,
rather than inventing a `game_result` for them. Shipped Stockfish nets **ramp**
lambda across training (`start-lambda 1.0 → end-lambda 0.7`, PR #4635) rather than
fixing it — the *ramp* transfers, the endpoint does not.

**The score→outcome scaling constant must be fitted, and three chess sources give
three different ones** `[MEASURED-ON: chess]` **EMP, NON-TRANSFERABLE**: Stockfish
`sigmoid(cp/410)` with its own code comment *"this depends on the engine, and maybe
even on the data"*; ChessBench/Lichess `1/(1+exp(−0.00368208·cp))` ≈ `sigmoid(cp/271.6)`;
Texel `1/(1+10^(−1.13·s/400))`. **A constant is not portable; it is fitted.**

**Censoring and mate rows.** D-621's `w5` one-sidedness is structural (rule 4) and
`w4`'s 1 593 positive rows exist but are removed by this phase's own `score_kind`
clause (D-629). Two consequences for a Phase-2 trainer: (a) mate rows must be
*kept in the extract and excluded at fit time*, which `extract.py`'s docstring
already enforces (*"MATE ROWS ARE KEPT AND LABELLED BY KIND… a filter applied here
could not be counted downstream"*); (b) **a mate score is not a value** — pinning
it into the objective is what D-616 tried and D-621 retired.

**One clause that fires on nothing, noted so a successor does not trust it.**
`fit.py:83-96`'s third clause drops rows with `|label| ≥ EVAL_MAX`. MEASURED over
the 74 672 `eval` rows: **0 rows** have `|label| ≥ 16 000`, and `select()`'s own
counts confirm it (`dropped_saturated_label: 0`). It is a guard, not a filter, on
this corpus.

**Two free notes from the closest ancestor** (Buro, CG'98, **ARCH**): the link function
is a **training-time device only** — *"There is no need to compute g [during search],
because `g(x1) > g(x2) ⟺ x1 > x2`"* — so a sigmoid objective costs the engine nothing;
and his terminal targets are **0.9 / 0.5 / 0.1**, not 1/0.5/0, because *"an optimal
weight vector may not exist if the extreme values 1.0 and 0.0 are chosen"*.

**A filter is a hypothesis, and the strongest evidence found runs against filtering.**
Texel's single largest lifetime gain — **+39.4 Elo of a +99.6 Elo total** — came
from **deleting** a plausible filter, verbatim: *"I believed that including those
positions would just raise the 'noise level'… Apparently this is not the case…
the q-search function has to deal with them all the time in real games, so trying
to learn how those positions should be evaluated on average is still beneficial"*
`[MEASURED-ON: chess/Texel]` **EMP**.

**The variant this licenses, and it is cheap and NOT foreclosed.** R6's filter drops
29 401 `eval` rows, and D-626 measured that **every one of them is a position the
mover blocks within the turn that follows** — positions the engine plays. D-622
forecloses *"fitting `w4` or `w5` from search labels"*; it does **not** forecloses
fitting the *quiet* terms over all 74 672 `eval` rows with the tactical entries
**pinned**. That is a different instrument, a closed-form solve, and it would say
whether the quiet weights are better supervised on the population they are actually
applied to. **This document proposes it and deliberately does not run it**: the
output is a candidate weight table, which is governed by the package's own
registration machinery, not by a research memo. Its one real caveat is the clamp —
`features.py:71-78` saturates at `EVAL_MAX` and the fit's linear form does not, so
the extended population must be checked for rows where the two diverge.

## §3 Objective, determinism, and the artifact

- **The objective is the deployed objective** (D-633): no constant term, because
  the schema has none. A tempo/intercept term may be *fitted and discarded* — that
  is what the estimator disagreement is about — but it may not be shipped.
- **Determinism.** The v0 fit is closed-form: *"no seed, no learning rate, no
  stopping rule"* (`fit.py` docstring). A codebook generator is not, and
  **PyTorch's own reproducibility page says so**: *"Completely reproducible results
  are not guaranteed across PyTorch releases, individual commits, or different
  platforms"* (**ARCH**). **The reproducible artifact is therefore the weight file
  by digest, never the training run.** Texel's own documentation states the same
  hazard from the engine side: *"The algorithm assumes the qScore function is
  deterministic. If transposition tables or the history heuristic is used in the
  qScore function this may not be the case."*
- **Seed variance is a strength effect, per the reference trainer** (**ARCH**,
  chess): networks trained from identical settings with different seeds *"vary in
  strength measurably… There's a good chance that at least one run from a set of 4
  is of relatively good quality."* **A single-run-per-arm architecture comparison
  is under-powered by the source's own standard** — a matrix row comparing two
  families needs a seed budget, not one net each.
- **The digest seam already exists here.** The engine reports `weights_sha256` in
  its identity line (`crates/pistol-cli/src/bin/pistol.rs:97`), the arena captures
  it (`crates/pistol-arena/src/identity.rs:9,20-21`), and the convention matches
  Stockfish's `nn-<sha256[:12]>.nnue`. **Phase 2 changes one thing:** the v0 table
  is committed configuration, the Stage-2 net is an artifact and is never committed
  (hard rule 8, `crates/pistol-eval/src/lib.rs:20-22`), so `weights_sha256` becomes
  the *only* provenance link between a match log and the weights that played it.
  Stockfish carries **two** digests — the filename hash for identity and an
  architecture hash in the header that refuses a shape-mismatched load. **The second
  has no analogue here and is worth one:** a net whose shape does not match the
  backend must be a named load-time error (rule 3), not a misread.

## §4 Quantization — a design obligation with an executable check

The constraint is stated as an obligation, not a runtime test: *"The quantization
scheme must be chosen such that no combination of possible active features can
exceed the maximum value"* (**ARCH**). Weight clipping is **clamp-after-step**
inside the trainer (±127/64 hidden, ±127·127/9600 output) — QAT-lite, not
fake-quantize.

**No primary source quantifies quantization's accuracy cost.** The docs assert
*"negligible"* for shallow nets. What the reference project ships instead is
`cross_check_eval.py`, which compares the **engine's quantized** evaluation against
the **trainer's float** model on the same positions (default `8·2^10`). *That* is
the transferable practice, and this project already owns the shape:
`tools/texel/verify_against_engine.py` drove the engine at **41 215 positions,
three weight tables, 0 disagreements** (handoff §4). **The quantization step's
acceptance is that same oracle re-pointed at the quantized backend, with an exact
agreement criterion registered before it runs** — exact, not "within a margin",
because an integer backend either reproduces the exported table or it does not.

## §5 SPRT sizing — and the two figures are different, which matters

`crates/pistol-arena/src/sprt.rs` works in **normalized** Elo: `NELO_TO_T =
ln(10)/800` and `Unit::t` multiplies by `√2` for pairs. Two sizing formulas follow,
and D-628 quotes only the first:

- **expected** pairs under either hypothesis: `2·ln(19) / t1²`
- **worst case** (at the indifference midpoint): `ln(19)² / t1²`

with `t1 = Δnelo · ln(10)/800 · √2`. Both reproduce the project's and fishtest's
own numbers: at Δ = 10 the first gives **3 554 pairs**, which is D-628's figure
exactly, and the second gives 5 233 pairs = **10 465 games**, which is fishtest's
`T = 1046535/Δ²` exactly. **A registration should budget the worst case and state
the expected** — D-628's rule ("state the pairs your bounds need") is satisfied by
either, but a run capped at the expected figure fails to conclude half the time.

| Δnelo (`elo1 − elo0`) | expected pairs | worst-case pairs | openings (book rule) | wall at 2.046 s/opening |
|---|---|---|---|---|
| 5 | 14 217 | 20 931 | 15 106 | 8.1 h / 11.9 h |
| **10** | **3 554** | **5 233** | **3 776** | **2.0 h / 3.0 h** |
| 20 | 889 | 1 308 | 944 | 0.5 h / 0.7 h |
| 30 | 395 | 581 | 420 | 0.2 h / 0.3 h |
| 50 | 142 | 209 | 151 | 0.1 h / 0.1 h |

DERIVED from the formulas above; the wall column uses the **MEASURED 2.046 s per
opening** from the design's registered dry run (`matrix_wp22_quiet_scale.md:250-253`)
and is valid only at that seat's node budget — a movetime seat costs more and that
figure is not measured. `crates/pistol-arena/examples/sprt_power.rs` is the shipped
instrument and it measures the same thing by simulation over a real pentanomial
(`artifacts/wp22_phase1_quiet/POWER.txt`); **run it, do not only compute** (D-628 §7).

**The instrument itself is biased, and this is the sharpest methodological finding in
the three documents.** Rapfi Table 2 `[MEASURED-ON: gomoku 15×15]` **EMP**: at equal
nodes/playouts the big slow net wins everywhere, and at equal **wall clock** the small
fast nets beat it by *"300-400 ELO… a winning rate of 85 %-92 %"*. **A node-matched SPRT
cannot see the throughput half of an eval change**, and throughput is exactly what the
incremental-codebook family trades accuracy for. Node-matching is what makes the
instrument deterministic (hard rule 4) and must stay — but **a Phase-2 verdict reported
only node-matched has measured one of the two axes the choice turns on.** Register a
**paired time-matched arm** beside it. `docs/research/eval_families_2026-09.md` §0.5
carries the same finding from the eval side.

**The unit gap the ROADMAP still carries.** `docs/ROADMAP.md:460-462` registers the
Stage-2 bar as *"node-matched SPRT >= +150 Elo vs handcrafted_v0"*. The arena reports
**normalized** Elo (`nelo_pair`), and D-628's whole finding was that these are not
the same quantity. **The bar's unit is unregistered.** At Δ = 150 nelo the test needs
16 pairs expected / 23 worst case (DERIVED) — trivially affordable, which is itself a
reason to state the unit before someone reads the bar as unreachable or as free.

## §6 Compute — training is not the scarce resource, by three orders of magnitude

**ESTIMATED, arithmetic shown.** 45 271 quiet rows at batch 1024 is **44 steps per
epoch**; 400 epochs (Stockfish's stated typical) is **≈ 17 700 steps**. For scale,
Rapfi trained for **600 000 iterations at batch 128** on 30.8 M positions, and
ChessBench for ~10 M steps `[MEASURED-ON: gomoku / chess]` **EMP**. A generator of
Rapfi-small size is **14 160 parameters**; the whole run is minutes on one GPU and
well under an hour on CPU.

Against that, **one acceptance SPRT at Δ = 10 is 2–3 hours at the instrument seat**
(§5) and far more at the deployment budget, and the corpus that feeds the training
cost **~63 hours to label** at 0.885 s per label (D-560). **Any plan that treats
training compute as the scarce resource has the ratio backwards** — the scarce
resources here are *labelled positions* and *reserved openings* (D-568's 1 000).

**The one published route to a bigger corpus decouples the two costs.** Katagomo's
`selfplay_distill` branch (commit `fc7490680`, 2025-02-22) states it in its own message:
*"Generate distill training data: **Use one weak model for fast game generating, one big
model for labeling**"* (**ARCH**). D-560's arithmetic is exactly this shape — pass 1 ran
26 games in 21.5 s while the capture pass spent 657 s on 742 labels, so **games are
free and labels are the whole cost**. It does not follow that a weak generator is safe
here: it changes the position DISTRIBUTION, which is Buro's *"training positions have to
be representative of the positions that will be evaluated later"*. It is a route, priced,
not a recommendation.

**Two published sizing rules that disagree by 100×, reported because neither should
be trusted alone:** nodchip's *≥ 10 positions per parameter* (which would cap a
Phase-2 model at ~4 500 parameters on the quiet population — attribution via a
community wiki, **UNVERIFIED**), against ChessBench's measurement that 7 M-parameter
transformers overfit at **~0.08 positions per parameter** `[MEASURED-ON: chess]`
**EMP**. Report both; derive the curve here.

**And the offline/strength gap is citable, not folklore.** Rapfi Table 2 `[MEASURED-ON:
gomoku 15×15]` **EMP**: Mixnet Large has strictly lower train *and* validation loss
on both heads than Resnet 4b64f yet is weaker at 1 playout, and Resnet 20b256f has
the best loss in the table while being **300-400 Elo weaker at equal wall clock**.
The reference trainer's own answer is procedural: it ships `run_games.py` + `ordo`
+ `delete_bad_nets.py`, which **ranks by Elo and deletes by Elo**, at fixed nodes.
**That is D-614 with a citation** — offline metrics say whether the trainer is
broken, never whether the engine is stronger.

## §7 `tools/texel/` against Call 4's three open gaps — review only, no fixes

Call 4's verdict was *"yes for the arithmetic, no for the instruments"*. The
arithmetic half holds: `features.py` and `extract.py` are 13 of 13 mutants dead,
`extract.py:57-65` verifies the corpus join **on every row** rather than sampling,
and `verify_against_engine.py` ran clean at 41 215 positions. The three named gaps
reproduce exactly as stated.

**Gap A — `tempo_constraints` omits `w3 ≥ w2 + 1`; `round_to_schema`'s refusal is
load-bearing rather than redundant.** CONFIRMED by reading.
`tools/texel/fit.py:374-381` returns exactly two constraints — `w1 ≥ 1` and
`−2·w1 ≥ 1 − (total − top)`, i.e. `w2 ≥ w1 + 1`. The schema's third relation,
`w3 ≥ w2 + 1`, is enforced **only** by `round_to_schema`'s *input* check at
`fit.py:255`. On the registered pins (`top = 60`, `total = 74`) it cannot bind, so
the registered run is unaffected; on other admissible pins it can, and then the
refusal is the only thing standing between the solve and an infeasible answer. The
consequence for a Phase-2 trainer is the general one: **a constraint set that is
not the schema is a constraint set that relies on a downstream refusal to be
correct**, and refusals are not where feasibility should live.

**Gap B — the candidate's five digits are pinned by no mechanism.** CONFIRMED.
`tools/texel/test_texel.py:857-873` is the only mechanism, and its own docstring
says so: *"The registered candidate is pinned by nothing but this… Editing
`configs/eval_v0_quiet_fit_weights.toml` to any other legal table passed every
gate."* Its four checks are the tactical entries verbatim, `w3` at committed, the
quiet **sum** at committed, and `candidate != committed`. Since `w1` is the single
free integer with six admissible values (D-633), **`[1,13,60,…]`, `[3,11,…]`,
`[5,9,…]` and `[6,8,…]` all pass every one of those checks.** The test pins the
*pins*, which is what it says it does; the digits are unpinned, and a Phase-2
artifact — a net, not five integers — will need a digest check rather than a
property check to close the same gap.

**Gap C — `options.py`'s minimiser check covers three of six rows and misses the
recommended one.** CONFIRMED, and it is the sharpest of the three. The
orthogonality test — the one that can actually tell a constrained minimiser from a
rescale — runs **only** inside `if kind == "index"`
(`tools/texel/test_texel.py:769-790`), which covers 3 of the 6 rows in
`options.py:25-32`. The other three branches check only that the pin holds:
`sum` at `:791-793`, `both` at `:794-798`, `free` at `:799-802`. **The test's own
comment says why that is not enough** — *"HOLDING THE PIN IS WHAT A RESCALE DOES
TOO, so it cannot tell a minimiser from one — and a rescale of the free solve is
exactly the construction the matrix was corrected for"* (`:772-776`). **The
recommended row is `T`, the two-pin `both` branch (`options.py:31`)**, so the row
the matrix leans on is checked by exactly the property its own test declares
insufficient. `options.py:38-44`'s docstring asserts *"Every row this prints is
strictly interior, so the two agree here"* — an assertion the script does not check
and no test drives.

**One further note, offered because it will bite Phase 2 rather than Phase 1.** The
premise memo's census instrument
(`artifacts/wp22_phase2_premise/probe_patterns.py`, receipted) is **no longer
runnable as committed**: line 13 imports `tools/texel` from a dead scratchpad path
and line 17 reads the corpus at `/home/tom/pistol-runs/arc3r-sweep/`, which D-636
moved. The receipt still verifies the *bytes*; it does not verify that the
instrument runs. `artifacts/research_2026-09/probe_patterns_folded.py` is a
path-corrected derivative and it **reproduces the committed census cell for cell**
at every growth-curve point and summary statistic for L7, L9 and L11 — so the
numbers stand and only the instrument had rotted.

## §8 The pipeline, in order

1. **Extract** — add `game` to `extract.py`'s kept columns (§1); keep every
   `score_kind` as it already does.
2. **Split** — group by `game`, not by position digest (§1); state the rule and its
   counts, as `select()` already prints its own.
3. **Encode** — axis-pooled, reversal-folded window codes (`eval_families_2026-09.md`
   §0.4); no symmetry augmentation.
4. **Fit** — deployed objective, tactical entries pinned, loss-blend form with the
   outcome term masked on the 48 % of rows without one (§2).
5. **Quantize** — clamp during training, bound the accumulator by construction (§4).
6. **Verify** — `verify_against_engine.py` re-pointed at the quantized backend, exact
   agreement, criterion registered first (§4).
7. **Ship** — the net is an artifact, never committed; `weights_sha256` is the only
   provenance link; add a shape check that refuses a mismatched load (§3).
8. **Accept** — SPRT, sized at the worst case with `sprt_power.rs` run rather than
   computed (§5). Offline metrics gate nothing (D-614).

## §9 Receipt — the evidence these three documents rest on

`artifacts/` is gitignored (hard rule 8), so an artifact directory is evidence only
while a tracked document carries its receipt's own digest. This section is that anchor
for `docs/research/{eval_families,search_next,training_pipeline}_2026-09.md`.

| file | what it establishes | cited by |
|---|---|---|
| `probe_patterns_folded.py` | the census instrument, path-corrected per D-636 | eval §0.2 |
| `census_nofold.txt` | reproduces the committed census cell for cell; adds L8 | eval §0.2 |
| `census_fold.txt` | the reversal-folded census, L7/L8/L9/L11 | eval §0.2 |
| `check_invariance.py`, `invariance.txt` | 0 of 2 200 folded images differ; 1 760 of 2 200 unfolded do | eval §0.4, training §1 |
| `granularity.txt` | `g` = 790 quiet / 1 877 all `eval` rows | search §1.3 |
| `game_count.txt` | 3 487 distinct games, 25.75 positions each | training §1 |

`artifacts/research_2026-09/RECEIPT_research_2026-09.sha256` verifies **7 of 7**; the
receipt's own `sha256` is
`13fe5712dafa94e2916267800f7b4162db380e27b03a16e00fd97f0981916dab`.

**None of these is a governed run.** Each is a read-only measurement over the corpus or
over the committed weight table, taken to replace an estimate a matrix row would
otherwise have carried (D-291). No weight table, candidate or Elo number was produced.
