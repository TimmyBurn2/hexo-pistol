# Evaluation families for QUIET structure — hex Connect(6,2,1), 2026-09

**What this is.** A read-only research input for Phase 2's OPTION MATRIX, scoring
families against the corpus and engine that exist. **It selects nothing** — selection is
the matrix's, attacked by a fresh DECISION-RED-TEAM first. `MEASURED` (instrument named)
· `DERIVED` (arithmetic shown) · `ESTIMATED` · `ARCH` (architectural literature fact,
transfers as a shape) · `EMP` (empirical figure, tagged with its board,
**NON-TRANSFERABLE**). Repo claims carry `file:line`. D-622 (R6) scopes it: tactical
terms pinned, learned family on quiet structure, so a row proposing to learn tactical
value from search labels is dead on D-621/D-629. **One negative up front:** two
independent sweeps found **no published evaluation function for a hexagonal-lattice
k-in-a-row game**, and only four arXiv papers mention NNUE at all — this field lives in
engine source, not literature, and every source below is a square 4-axis board.

## §0 THE FINDING — the two curves that fix the window length do not overlap

The premise memo (`docs/experiments/wp22_phase2_premise.md` §3) measured
**observability** — how many observations a length-`L` cell gets. Nobody measured
**sufficiency** — whether length `L` separates structures that differ in value.
`threat_calculus_v1.md` §8 (THM-WINDOW, `[PROVEN]`) states that side — *"length-6
occupancy tables cannot classify live vs dead fours (needs 8-cell context; fives need
7)"* — and asks for *"the minimal sufficient hex length by enumeration"*.

### 0.1 Sufficiency has a closed-form lower bound, and it is 11 (DERIVED)

A length-6 window containing cell `c` starts at `c−5 … c`, so the union of all 6-windows
through `c` on one axis spans **`c−5 … c+5`, eleven cells** — so **`L ≥ 2·WIN_LEN − 1 =
11`** is required for one centred window to contain every 6-window through `c`. Below 11
a codebook cell is a *partial* view.

| engine | k | minimum `2k−1` | length used | source |
|---|---|---|---|---|
| Rapfi (gomoku) | 5 | 9 | **11** | arXiv:2503.13178 §3.2, *"−5 ≤ k ≤ 5"* — **ARCH** |
| Katagomo `ConnectSix2024nnue` (Connect6) | 6 | 11 | **13** | `cpp/nnue/NNUEglobal.h` (external), `featureHalfLen 6`, `featureLen 13`, head `0785b9af` |
| every **pre-net** classical line engine surveyed (9 of them) | 5 | 9 | **9** | engine source; 11 appears **only** in net-era engines |

So the ROADMAP's registered **length 11 is not a k=5 import: it is exactly this game's
covering minimum**, and the analogous convention would be 13 — a correction to §8's
*"k=5 lore"* reading that makes the registered figure better founded.

### 0.2 Observability, MEASURED and widened

Instrument: `artifacts/research_2026-09/probe_patterns_folded.py`, a derivative of the
premise memo's `probe_patterns.py` (receipted `sha256 0d3f43ac…447e`) changed in three
ways — the corpus path follows D-636, `tools/texel` is imported from the live tree not a
dead scratchpad, and each code is optionally folded to `min(code, reversed(code))`.
**The unfolded columns reproduce the committed census cell for cell** at all nine
growth-curve points and every summary statistic for L7, L9, L11. L8 is new.

| L | fold | cells | median obs/cell | cells < 10 obs | **cells covering 90 % of obs** | ceiling | % of ceiling |
|---|---|---|---|---|---|---|---|
| 7 | no | 1 990 | 660 | **0 (0.0 %)** | 368 | 2 186 | 91.0 % |
| 7 | **yes** | **1 029** | **1 282** | **0 (0.0 %)** | **190** | 1 133 | 90.8 % |
| 8 | no | 5 778 | 140 | 123 (2.1 %) | 577 | 6 560 | 88.1 % |
| 8 | **yes** | **2 920** | **273** | **24 (0.8 %)** | **293** | 3 320 | 88.0 % |
| 9 | no | 16 145 | 30 | 3 721 (23.0 %) | 852 | 19 682 | 82.0 % |
| 9 | **yes** | **8 374** | **56** | **1 079 (12.9 %)** | **434** | 9 962 | 84.1 % |
| 11 | no | 62 370 | 6 | 38 585 (61.9 %) | 1 599 | 177 146 | 35.2 % |
| 11 | **yes** | **38 983** | **7** | **22 142 (56.8 %)** | **810** | 88 937 | 43.8 % |

Ceilings `3^L − 1` and `(3^L + 3^⌈L/2⌉)/2 − 1` (DERIVED); walk = 45 271 quiet `eval`
rows, the R6 population. Window-observation totals are identical folded and unfolded at
every length — folding relabels and never drops, the run's own internal check. The
census counts windows by `(axis, start)`, the enumeration the shipped eval uses at L=6
(`crates/pistol-core/src/window.rs:96`); a centred-window codebook indexes the same
space differently, so these counts are indicative rather than exact for that variant.

### 0.3 THE CURVES DO NOT MEET

Sufficiency wants `L ≥ 11`; at `L = 11` the median cell has 6-7 observations and 56.8 %
of cells are seen under ten times even after the fold. **On this corpus no window length
is both covering and observable.** That is not a reason to shorten the window — it is
the reason a matrix row must break the identity between *cells* and *parameters*. §1
lists five published ways; only A1 fails to.

### 0.4 The reversal fold is not a compression — it makes the representation a function of the position

Instrument: `artifacts/research_2026-09/check_invariance.py` (receipted — see
`training_pipeline_2026-09.md` §9). **MEASURED**, 200 random positions × the 11
non-identity lattice symmetries (`crates/pistol-core/src/symmetry.rs:31`), L9:

- unfolded, axis-pooled: **1 760 of 2 200 images have a different code multiset**.
  Exactly **one** of the eleven symmetries preserves it on all 200 positions; the other
  ten agree on 13 to 44 of 200, by coincidence.
- **reversal-folded, axis-pooled: 0 of 2 200 differ.**

So an **unfolded** axis-pooled codebook gives twelve feature vectors to one position —
it needs 12-fold augmentation to average that away, or it learns twelve answers. A
**folded** one is exactly invariant, so augmentation is a **no-op**; and since the
corpus is already deduped by `key_full` (`crates/pistol-arena/src/labels_file.rs:26`),
augmenting would re-create rows dedup removed and inflate any per-position split. The
fold costs nothing at inference, roughly doubles observations per cell, and is *sound* —
reflection is a lattice symmetry and the win condition is direction-agnostic. **Rapfi
cannot take this saving**: it uses different weights per axis class *"to capture subtle
differences caused by directionality"* (§3.2) because a square board's axes are not
equivalent. **All three hex axes are equivalent under the symmetry group, so one shared
mapping suffices.**

### 0.5 THE ACCEPTANCE INSTRUMENT IS BIASED AGAINST THE FAMILY THAT WINS

`docs/ROADMAP.md:460-462` registers Stage-2 acceptance as a **node-matched** SPRT, and
hard rule 4's instrument mode is fixed-nodes by design. **Rapfi Table 2 measures that a
node-matched comparison systematically prefers the slower, more accurate evaluation**:
at equal nodes/playouts ResNet beats every Mixnet configuration, and at equal **wall
clock** the same Mixnets beat the same ResNets by *"300-400 ELO… a winning rate of 85
%-92 %"* `[MEASURED-ON: gomoku 15×15]` **EMP, NON-TRANSFERABLE** in magnitude, **ARCH**
in direction.

**A node-matched SPRT cannot see the throughput half of an eval change**, which is
exactly what this family trades accuracy for. That is not an argument against
node-matching — it is what makes the instrument deterministic, hard rule 4 — but **a
registration reporting only a node-matched verdict measures one of the two axes the
choice turns on.** The matrix should carry a **paired time-matched arm** beside it.

## §1 Family A — pattern codebook over axial windows: five ways to break §0.3

**The incumbent, so the learned rows have something to beat:** the strongest documented
line-game engines evaluate with a **hand-scored line-type table** — Wine (Piazzo,
Scarpiniti & Baccarelli, arXiv:2111.01016, 2021 §4.7, **ARCH**) *"scores each line type
according to its strength… stored in a vector"*, summed signed, at branching factor
*"B = 40"* **EMP**: `handcrafted_v0`'s family. A survey of nine classical line engines
found **none using more than 13 threat classes**.

### A1. Free per-cell table — the row §0.3 kills

Parameters = cells: at L11 unfolded, 62 370 parameters against 45 271 positions, **1.4
observations per parameter (DERIVED)**, and the fold does not save it. At L8 folded it
is 2 920 parameters at median 273 — admissible, but below §0.1's covering minimum.

**Buro's thresholds are the published calibration** (Michael Buro, CG'98, LNCS 1558 —
the closest ancestor: a linear eval over ternary line-pattern tables learned from
search-scored positions, 16-bit integers, alpha-beta engine). Verbatim (**ARCH**): *"If
the count is small (say ≤ 4)… the weight is set to 0"*, against *"sufficiently high (say
≥ 20)"* for a safe fit; his generator keeps only configurations seen in ≥ 75 of ~11 M
positions **EMP**. Logistello: ~1.5 M weights from ~11 M positions = **7.3 per weight**;
this corpus at L11 folded = **1.2**.

### A2. Mapped codebook — the codebook is compiled, not fitted

**Rapfi (Jin, Duan & Hang, arXiv:2503.13178, 2025)** hit §0.3 head-on, §3.3 (**ARCH**):

> "one might use an embedding layer of size N… However, this approach is prone to
> overfitting and may not sufficiently train all features, as not every pattern is
> guaranteed to appear often enough to generate meaningful gradients."

Instead a **shared-weight directional convolution** (five `Dir Conv` layers, receptive
field 11) is trained, then **every code is enumerated once and pushed through it** and
the outputs recorded as the codebook: *"Since the mapping network functions as a
convolution network with shared parameters, it can be trained robustly with a limited
amount of data."* Rapfi's codebook has **397 488 entries** from **14 160 parameters** at
its smallest configuration (Table 1) — **EMP** for the sizes, **ARCH** for the ratio.

**The memory wall says it from the other side:** Katagomo's Connect6 NNUE materialises
`int16 mapping[4 · 3^13][128]` = **≈ 1.52 GiB**, with the source comment *"the mapping
is larger than 2GB, so split to 4 parts"*. **A full length-13 table is not a deployable
artefact here.**

### A3. Factored table — a dense feature carries the sparse one

Stockfish's documented answer to exactly §0.3 (nnue-pytorch docs, §Feature
factorization, **ARCH**): sparsity *"directly impacts how much each feature is seen
during training, and that negatively impacts the learning of weights"*, so a redundant
**virtual** feature set is added **during training only**, *"results in even the rarest
of feature weights being populated quickly with reasonable values"*, and is then
**coalesced into the real weights — zero inference cost**. The factor is already
measured: **length 7 folded (1 029 cells, none under ten observations) is the natural
dense factor for a length-11 table.** Caveat, verbatim: *"just adding more factors… may
even cause it to regress"*, and it *"seems to only be relevant in the early stages"* —
said against 16 B positions where this corpus has 89 805, so "the early stage" is where
a Phase-2 fit permanently lives. Buro's third remedy is one free line: **normalise each
cell's gradient by that cell's own match count**, not by the batch size (**ARCH**).

### A4. Quotient by hand-designed threat classes — the cheapest, and it needs no corpus at all

**Read from Rapfi's own source.** Rapfi ships a *second*, classical eval over the *same*
length-11 line encoding: its 132 496 dense line keys (`DenseHalfCnt 364`, `364² = 132
496`; the centre cell is never encoded, being the candidate placement) **get no free
weights**. They map into a **4-bit `Pattern` enum with exactly 16 members**, and the
four directions combine into `PCODE_NB = 3876` — verified to be the multiset coefficient
**C(19,4) = 3 876**. **So the free parameter count is ~3 876, not ~132 496** — a ~34×
compression from hand-designed threat semantics rather than from learning. **The hex
analogue is `C(18,3) = 816` codes** (DERIVED), against **3 487 games / 74 672 `eval`
rows** — comfortably past Buro's `≥ 20` and his `n = 75`. This project already owns the
classification layer: exact threat number by minimum hitting set (RULE-EXACT), the
`LiveCount`/`NearHot`/`WinWitness` queries (`crates/pistol-solver/src/query.rs:81-142`)
and `threat_calculus_v1.md` §5. **A hex enum must be re-derived for 3 axes, never
imported.**

### A5. Top-K vocabulary plus one rare bucket, and the one in-domain datapoint

figrid keeps `PATTERN_TOP_K = 4265` window codes as free cells and routes the rare tail
to one shared bucket (**ARCH**). **The frequency census that sizes it already exists**:
§0.2's last-but-two column says **810 folded L11 cells carry 90 % of all observations**
(434 at folded L9). A top-810 vocabulary plus a rare bucket is **811 parameters covering
90 % of the evidence**, at near-zero build cost.

**figrid** (2026 Gomocup entrant; source code, not literature) trained a
**512-accumulator gomoku NNUE from ~93 000 Rapfi-labelled positions** — within 4 % of
this corpus's 89 805 — and it **worked**: 59.5 % over 1 000 games against its own
heuristic eval. The same source records a **1 024-accumulator net losing 23 percentage
points of arena score** to the 512 one at ~1.1 M samples, on a 4:1 parameter-to-sample
overfit, and an added feature block giving a **loss improvement uncorrelated with
tactical decisions**. `[MEASURED-ON: gomoku 15×15]` **EMP** — but a **line game at this
corpus size** is a tighter bracket than chess or Go can give, and **both halves are
pre-registrable: a few-hundred-unit accumulator is achievable at ~90 000 positions, and
doubling its width regressed.**

### A6. Cost shape on THIS engine

Today `WINDOWS_PER_CELL = 18` (`crates/pistol-core/src/window.rs:9`) is touched per
stone by `HandcraftedV0::apply` (`crates/pistol-eval/src/handcrafted.rs:191-210`); a
length-`L` codebook touches `3L` — **24 at L8, 33 at L11, 39 at L13** (DERIVED),
1.33×/1.83×/2.17× today's traffic, against Rapfi's *"at most 4 × 11 directional features
are affected"* per stone on 4 axes (§3.4). That traffic sits on the top hotspot:
`HandcraftedV0::delta` is **31.77 %** of the committed seat's profile
(`docs/audit/repo_audit_2026-09.md` A-02, MEASURED). **Two-stone turns need nothing
new** — the eval seam is per-stone (`crates/pistol-search/src/position.rs:111-120`), a
turn is two `apply` calls and rule-4 truncation is one, and `Eval::delta`'s default body
is the apply/value/undo roundtrip (`crates/pistol-eval/src/eval.rs:87-92`), so a backend
not overriding it is correct by construction. Katagomo's contract is literally `sumw =
sumw − mapping[oldshape] + mapping[newshape]` with `play`/`undo` — **the same contract
`Eval` states**. Nothing in the store API is Phase 2's to design (premise memo §4).

### A7. Quantization and determinism — the risk is overflow, not accuracy

Rapfi §A.3, verbatim (**ARCH**): *"we clamp all features in the pattern-based codebook
to [−16, 16], and quantize them into 16-bit integer using a scale factor of 32"*;
aggregation int16, accumulation int32; motivation *"to avoid any floating point error
that may be introduced during the accumulation process of incremental update"* — hard
rule 4's argument, made independently. **The `[-16, 16]` figure is confirmed and is
about the codebook features, not the weights.** Katagomo's Connect6 net is **int16 Q15**
throughout its trunk.

**The "quantization cliff" is not in the primary source** — Stockfish says the opposite
for shallow nets: *"in the case of NNUE networks, which are relatively shallow, this
error is negligible"*. What the sources name: (1) **weight range** — *"the training can
diverge from the quantized representation by more than just rounding"*, fixed by
clamping *during training* (±127/64 hidden, ±127·127/9600 output); (2) **accumulator
overflow** — *"The quantization scheme must be chosen such that no combination of
possible active features can exceed the maximum value"*, a **provable bound** here (`3L`
active windows × the clamped maximum), and KataGo's own FP16 overflow produced
*"confident but completely wrong evaluations"*; (3) **float drift is the reason to
quantize** — *"Repeatedly adding and subtracting floats results in error that
accumulates with each move"*. `EVAL_MAX = 16_000` (`eval.rs:10`) with an i64
intermediate and an end clamp (`handcrafted.rs:152-159, 254-260`) is the shape to keep,
and on cost: generic dynamic INT8 via ONNX Runtime bought only **1.19–1.50×** in the
Katagomo work against Rapfi's **~4×** from hand-written AVX2 **EMP** — the kernel pays.

**The width constraint, and three games agree on it.**

- **Rapfi** Table 3 **EMP**: under **α-β, Mixnet Large scores −82 / −58 / −45 / −38 Elo
  against Small** at 2/5/10/20 s while scoring **+57 to +84 under MCTS** — *"likely due
  to the higher codebook update cost diminishing the benefits of incremental updates"*
  (§5.3).
- **Logistello** `[MEASURED-ON: Othello]` **EMP**: three large patterns cost ~45 % speed
  and *"could not compensate"*; the winners added **exactly one**, at 11 % slowdown. *"a
  significant improvement of a sequential program may not be possible by adding further
  patterns based on the raw board representation."*
- **figrid** (§A5): 1 024 accumulators lost 23 points to 512 at this corpus scale.
- **Nasu 2018** (**ARCH**): NNUE was sized to *"the amount of positions which can be
  evaluated per time unit"* of the eval it replaced — **speed parity, not accuracy**.

**ARCH conclusion, and it transfers: under alpha-beta the eval-width optimum sits below
the accuracy optimum**; the location does not. **A row that does not pre-register a
nodes/sec floor is not testing what decides the outcome.** §0.5 is the same fact from
the instrument's side.

## §2 Family B — small MLP over window features (no codebook bake)

**Buys** the window interactions an additive table cannot express — LAW-DECOMP is
explicit that additivity is *false* for static evaluation (*"never sum regional eval as
if independent"*), and Wine names the aliasing consequence: it *"does not account for the
C43 and C33 patterns"*, and scoring them *"slightly improves the player"*. **Costs**
incrementality: Rapfi preserves it by making only the first-half channels non-linear
(§3.4), Nasu by making only the **first** affine transform incremental (§2.9), so **a
hidden layer before the accumulator destroys the `Eval` contract's cheap path**.
Throughput is not the objection (Mixnet Small reaches **428 K α-β nodes/s** **EMP**).
**Provenance caution:** the survey found **no classical line-game engine using an MLP
over pattern features** — every published instance is an accumulator NNUE, i.e. A2 under
another name.

## §3 Family C — SPSA in-engine (no labels, and D-621 never applies)

Tunes by match outcome, so it never asks the corpus what a five-window is worth. The
only family untouched by D-621, D-626 and D-629.

- **Precedent, ARCH:** Rapfi §A.4 tunes search parameters by SPSA, reporting
  *"significant ELO improvements"* — with no game or parameter counts.
- **The cost is games, and fishtest's live data prices it** `[MEASURED-ON: chess]`
  **EMP**: over 46 sampled runs, a **median of 120 000 games for 4 parameters**, max 221
  parameters at 300 000 games — not a rounding error against D-568's 1 000 reserved
  openings, and **not a codebook** either way. The fishtest wiki also names this
  project's exact problem: *"values that are only rarely used to score a position can
  have a hard time moving at all"*.
- **It fights the strength law:** rule 6 accepts by SPRT and SPSA *optimises on* match
  outcomes, so tuning and acceptance books must be disjoint or the acceptance run is a
  self-match (D-635's defect class).
- **Scheduling conflict — handoff Call 3.** `docs/ROADMAP.md` and
  `configs/eval_v0_weights.toml:46` both place SPSA/Texel in **Stage 4**; WP-2.2 ran a
  Texel fit in Stage 2, and **no ADR moves it**. Asked five times.

## §4 Family D — Texel with the deployed objective and pinned tactical terms

Phase 1, re-scoped. What remains identifiable is **measured** (D-622): on the 45 271-row
quiet population the quiet regressors have sign counts 12 243 +/32 285 −, 8 277 +/31 783
−, 3 467 +/27 841 −, OLS `t` = 6.4/31.7/5.6, condition number 90.5; both tactical
regressors are **identically zero** there.

**The ceiling is small, in one line:** after R6's pins and the two exchange-rate pins
the table has **one free integer**, six admissible values (D-633), and the ROADMAP's
`+150` bar was written for the codebook net (D-614). **It is still worth a power-checked
run** for a reason not about the weights: the estimators **disagree about the sign** —
the deployed no-intercept objective wants `w1 = 1`, the tempo-fitted one wants `4`,
committed is `2` (val MSE 724 169 / 728 957 / 724 743). One SPRT between 1 and 4 settles
which estimator makes the better player and needs no new corpus work (handoff Call 0);
size it with `sprt_power.rs` first (D-628). Texel fits the **quiescence** score, not the
full-search score — worth one line in any registration citing it as precedent.

## §5 Family E — distillation from a stronger search

**The corpus already is one, and the teacher's margin is measurable.** MEASURED over
`artifacts/wp22_review3_export/rows_fresh.txt` (89 805 rows): label `depth_turns` 1→7
272, 2→8 307, 3→20 026, 4→40 317, 5→13 161, 6→709, 7→13 — **median 4 turns** (the 74 672
`eval` rows likewise, range 2–7). Deployment reaches a **median of 2 turns at 0.5 s**
(`opt_arc_perf_finding.md`). **The teacher is about two turns deeper than the student.**

- **The strongest quantitative prior for this setup is TreeStrap** (Veness, Silver,
  Uther & Blair, *Bootstrapping from Game Tree Search*, NIPS 22, 2009): a **linear**
  eval over 1 812 features trained toward **alpha-beta search values** in an alpha-beta
  engine. `[MEASURED-ON: chess]` **EMP**: TreeStrap(αβ) 2157 ± 31 Elo against
  RootStrap(αβ) 1362 ± 59 — **updating from every node of the tree beat updating from
  the root alone by ~795 Elo, same engine, same features** — and it *"learn[s] effective
  weights in just a thousand training games"*. **ARCH: where labels are harvested
  dominates the eval family.** This corpus is root-only — the weaker arm, and the
  expensive one.
- **What that does NOT license.** Tree-harvesting does **not** restore `w4`/`w5` sign
  variation: D-621's mechanism is rule 4, which applies inside the tree too — a mover
  holding a live five is resolved as a win there and carries a mate label. It changes
  label **count and diversity**, not that one-sidedness.
- **Rapfi's distillation is a different axis** — big-net to small-net on self-play
  value/policy targets, *"75 % from the distillation labels and 25 % from the true
  labels"* (§A.2), ~30.8 M positions, **343× this corpus**; the blend is itself a hedge
  against teacher bias.
- **Teacher bias applies, measured rather than feared:** D-609 established that *deeper*
  search with this eval makes the engine **worse**, so 400 000-node labels are not
  straightforwardly better than the 244 000-node ones they would train. **No census
  answers that** — an SPRT does, or re-labelling a slice at a second budget. (**KL on
  policy is out of scope**: no policy head.)

## §6 The comparison, one row per family

| | parameters vs evidence | inference cost | determinism (D-7) | quantization risk | published failure mode |
|---|---|---|---|---|---|
| **A1** free table, L11 folded | 38 983 params, median 7 obs — **1.2 obs/param** | 33 lookups/stone | trivial | low | the premise memo's own kill; Buro zeroes weights below ~4 matches |
| **A2** mapped codebook | generator params ≪ cells (Rapfi 14 160 gen / 397 488 cells) | one lookup/window | integer after bake; training offline | int16 features, int32 accum; **overflow-bounded** | α-β size optimum below accuracy optimum (three games agree) |
| **A3** factored table | real params as A1, factor L7-folded (1 029 cells, 0 under ten) | identical to A1 — **zero** | trivial | as A1 | *"may even cause it to regress"* (Stockfish) |
| **A4** threat-class quotient | **816 codes** (DERIVED, C(18,3)) over 3 487 games | one class lookup + one code | trivial, integer | none | the enum is hand-designed — a wrong class is invisible to the fit |
| **A5** top-K + rare bucket | **811 params cover 90 % of observations** (MEASURED §0.2) | as A1, smaller table | trivial | low | the tail is exactly where a novel structure lands |
| **C** SPSA | ~a dozen integers | none (offline) | tuner stochastic, **engine** deterministic | none | **median 120 k games for 4 params** (fishtest, EMP) |
| **D** Texel, deployed objective | **one free integer**, 45 271 rows | none | exact closed form, no seed | none | ceiling six points wide |

## §7 Matrix rows the architect can lift

- **R-A4-CLASS — length-11 windows quotiented through a hex threat enum, scored on the
  unordered 3-axis multiset.** 816 parameters (DERIVED); the only row that is *covering*
  (§0.1) and *densely evidenced* at once, and the one this project is already tooled for
  (`crates/pistol-solver/src/query.rs:81-142`, RULE-EXACT). *Kill*: the hex enum cannot
  be derived without importing 4-axis thresholds, which `threat_calculus_v1.md`'s scope
  rule forbids.
- **R-A5-TOPK — top-810 folded L11 codes plus one rare bucket.** 811 parameters covering
  90 % of observations (MEASURED); near-zero build cost. *Kill*: the rare tail is where
  novel structure lives and one bucket scores all of it alike.
- **R-A2-L11F — mapped codebook, length 11, reversal-folded, axis-pooled.** The covering
  minimum made affordable by a shared-weight generator; symmetry exact (§0.4); 33
  touches/stone against today's 18 on a 31.77 % hotspot. *Kill*: it does not beat A3's
  validation loss, or the nps floor is missed.
- **R-A3-L11F+F7 — length-11 folded table with length-7-folded as a training factor.**
  Zero inference cost, the documented remedy for §0.3. *Risks*: Stockfish's regression
  caveat; Buro's ≤4-match rule.
- **R-A1-L11 — free table at length 11, unfolded.** Recorded so the matrix shows it
  scored and killed: the premise memo's number, and §0.4 independently.
- **R-C-SPSA — the only row D-621 does not touch.** Must state a game budget against
  D-568's reservation; the measured precedent is 120 000 games for four parameters.
- **R-D-W1 — the one-integer SPRT (`w1 = 1` vs `w1 = 4`)**, upstream of all of it.
- **Every row must state a nodes/sec floor and be scored on a time-matched arm beside
  the node-matched one (§0.5), or the instrument prefers the wrong row.**

## §8 What this document does NOT establish

- **The sufficiency enumeration.** §0.1 gives a *covering* lower bound by derivation;
  THM-WINDOW asks whether a shorter window still separates structures of different value
  once summed over overlaps. Minutes of compute, **not run** (D-291).
- **A hex threat enum.** A4 is the strongest row on this corpus and its enum does not
  exist; every published class set is 4-axis and `threat_calculus_v1.md`'s scope rule
  forbids importing the numbers.
- **Any strength claim** — no number here is Elo and none may move a committed config
  (D-614). **A cost in nps.** A6's multipliers are DERIVED window-traffic ratios; the
  premise memo §5 registers the absolute figure as OWED and this does not discharge it.
  **Nothing about the human corpus either** — D-453 and the corpus-grade section stand.
