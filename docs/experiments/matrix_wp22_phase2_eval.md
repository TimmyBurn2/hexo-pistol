# OPTION MATRIX — the quiet-structure eval family for Phase 2, revision 1.

**Governing revision**: `54eb3ba` (`dev`) for every file, line and count quoted
below (D-692), except where a paragraph names a later one.

**THIS MATRIX SELECTS NOTHING** (D-708). It states the field, prices every row on
its own instrument, ranks them, and hands the selection to the architect. A fresh
DECISION-RED-TEAM attacks it before anything is chosen; the ADR that selects is a
separate act and records the strongest surviving attack.

**Every number is MEASURED, DERIVED or ESTIMATED**, and an estimate that could
have been measured in seconds is a finding (D-291). Where a number came out of a
run, the run's artifact and its receipt digest are named (D-483). §11 is the
failed-precedent check the last matrix in this package earned.

---

## §1 PREMISES, QUOTED FIRST (D-477)

A matrix's own axis is a premise. So is every quantity a row is priced against.

### 1.1 The corpus, and it is re-derived by this package's own walk

- **89 805 deduped positions**, digest
  `00f61780cc1654958696786051dbd8de7d1bbab3f5d49153b1694770caf35968`
  (`docs/book_v3_ledger.md:50` quotes the same digest).
- **74 672 `eval` rows; 45 271 quiet (60.63 %)** under R6's filter (D-622),
  `wp22_phase2_premise.md` §2.
- **3 487 distinct games** (`training_pipeline_2026-09.md` §1).
- **RE-DERIVED, MEASURED**, by `tools/hex_enum/seed_pilot.py`, a walk written
  from scratch against the record grammar and sharing no code with either census:
  *"walked 45271 quiet rows over 3487 games, 38983 distinct folded L11 codes"* —
  the premise memo's population, the pipeline's game count and
  `eval_families_2026-09.md` §0.2's folded L11 cell count, from one pass.

### 1.2 The instrument seat's throughput — the premise memo's OWED §5 figure, now MEASURED

`wp22_phase2_premise.md` §5 registers the absolute nps as **OWED**: *"It is
measured at the standing instrument seat and recorded in §5 of this memo's next
revision, before any matrix row claims a cost. Quoting a remembered absolute
here is the estimate D-291 calls a finding."* **It is taken here.**

Instrument: `measure_nps.sh` at `c5123c1` in a detached worktree, the seat
`tools/bench_delta.sh` registers — `configs/instrument_v0.toml`,
`crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` (24 positions,
12 early / 12 late), `go nodes 50000`, five reps, one binary and no verdict.

| seat | band | nps, median of 5 reps | min | max |
|---|---|---|---|---|
| `instrument_v0`, `go nodes 50000` | **all** | **531 548** | 531 037 | 532 574 |
| | early | 570 722 | 566 960 | 571 806 |
| | late | 491 110 | 489 672 | 495 473 |
| `instrument_v0`, `go depth_turns 2` | all | 450 233 (73 388 nodes in 163 ms) | | |

**REPLICATED, because the run is cheap and the rule says to replicate rather
than derive a margin** (`docs/process.md`, *"Cost, replication, and the second
instrument"*): an independent second execution of the same script gives **529 255
nps** all-bands, 0.43 % from the first. Receipt
`artifacts/wp22_phase2a/nps_seat/`, `sha256sum -c` clean, receipt digest
`211b6e1002f32ee89baace6ed4ff510302b2aad7817094e109dcac121d51ca9e`; the binary
that produced it is
`41b2c5df832fb37ee55ddff62901d641e73841cbe22965aadc1b0e3da66e0607`.

### 1.3 THE PLAY SEAT IS NOT ONE SEAT, AND THE TWO DIFFER BY 2.8x

D-705 registers a **time-matched arm at the play seat**. There are two committed
play seats and they are not the same search: `configs/play_v0.toml` commits
`candidate_policy kind = "radius", radius = 3` and `configs/play_staged_v0.toml`
commits `kind = "staged"`. `play_v0.toml`'s own comment states the disagreement
is deliberate and unresolved — *"this is the FIRST TIME this file deliberately
disagrees with `configs/instrument_v0.toml` … Until that verdict this value does
not move"*.

MEASURED, same worktree, same 24 positions, `go movetime 500`, 3 reps:

| config | nps | mean `depth_turns` reached |
|---|---|---|
| `configs/play_v0.toml` | **172 627** / 172 665 / 172 211 | **1.21** |
| `configs/play_staged_v0.toml` | **478 718** / 480 449 / 482 105 | **3.42** |

**A row's time-matched arm therefore has no defined seat until the architect
names one**, and the two candidates differ by 2.8x in throughput and 2.8x in
depth reached at the deployment budget. D-705's flip clause — *"flips if the play
seat is redefined"* — is engaged before its first run.

### 1.4 THE ARENA REFUSES A MOVETIME BUDGET BY NAME, SO NO TIME-MATCHED ARM IS RUNNABLE TODAY

`crates/pistol-arena/src/validate.rs:45-51` is *"The one refusal this crate
exists to make loudly"*, and `crates/pistol-arena/src/error.rs:164-172` gives its
message verbatim:

> `MovetimeBudgetRefused`: … is a wall-clock budget, and every strength claim in
> this project comes from an instrument budget — depth_turns or nodes (CLAUDE.md
> rule 6). A movetime budget is also not a ceiling … Wall-clock arrives with
> WP-1.4 and Stage 4, by ADR then.

**So D-705's time-matched arm is not a per-row cost at all — it is a capability
nothing in this repository has**, and it is the same for every row. Discharging
it needs (i) an ADR moving the arena's refusal, which the refusal's own text
says is Stage 4's, (ii) a play seat named (§1.3), and (iii) a book (§1.6). The
matrix records this once, here, and does not repeat it in eight rows: **the
two-arm rule does not discriminate between rows and cannot rank them.**

### 1.5 The `book_v3` reuse rule, verbatim

`docs/book_v3_ledger.md`, *"The rule"*:

> A new pre-registration takes the next unconsumed range, adds its row here in
> the same commit that adds its arena config, and never re-reads a range this
> table already holds. Reading a consumed range for a CLOSED verdict is not a
> new use and needs no row.

And its consumed-ranges table reads **`nothing yet`** — the book is whole. Its
sizing rule, from `book_v2_registration.md` §4 and applied at
`book_v3_registration.md` §R1: `n_openings = ceil_to_500(P + 500)`, which at
`P = 8000` gives the committed **8 500**.

`book_v2`'s last **1 000** openings are reserved and never labelled (D-568), and
D-653 fixes what they are for: *"book_v2's 1,000 reserved openings are RETAINED
whole as the large-effect screening book (elo1 >= 30, power computed per D-628)
and as the only corpus-disjoint v2 slice; book_v3 carries every elo1 = 10
question; no run spends the holdout below computed power 0.9"*.

### 1.6 THE BOOKS ARITHMETIC, MEASURED, AND IT FUNDS ONE ARM

Closed form (`training_pipeline_2026-09.md` §5): expected pairs
`2·ln(19)/t1²`, worst case `ln(19)²/t1²`, `t1 = Δnelo · ln(10)/800 · √2`.
**DERIVED and cross-checked against two of this project's own figures**: at
Δ = 10 the first gives **3 554**, which is D-628's number exactly, and the second
gives **5 233**, which is fishtest's `T = 1046535/Δ²` exactly.

| Δnelo | expected pairs | worst-case pairs | openings, `ceil_to_500(P+500)` |
|---|---|---|---|
| 5 | 14 217 | 20 931 | 15 000 / 21 500 |
| **10** | **3 554** | **5 233** | **4 500 / 6 000** |
| 20 | 889 | 1 308 | 1 500 / 2 000 |
| **30** | **395** | **581** | **1 000 / 1 500** |
| 50 | 142 | 209 | 1 000 / 1 000 |
| 150 | 16 | 23 | 1 000 / 1 000 |

**And the closed form is not what D-653 requires**: it requires *computed power
0.9*, which is `crates/pistol-arena/examples/sprt_power.rs` RUN, not computed
(D-628 §7). Run here, release build at `54eb3ba`, `--buckets 2,3,8,7,4 --runs
20000 --seed 1` — the WP-2.2 Phase 1 pentanomial, which
`book_v3_registration.md` §P6 already records as *"a thin base for a sizing
decision"*, 24 pairs in total. **Instrument cross-check**: `--pairs 8000 --elo1
10 --truth 10` returns `h1 18090 (0.9045)`, digit for digit the committed
`book_v3` figure.

| arm | book | cap by the openings rule | power (`h1`) at truth = Δ |
|---|---|---|---|
| node-matched, Δ = 10 | `book_v3`, 8 500 openings | **8 000 pairs** | **0.9045** |
| node-matched, Δ = 10, half the book | `book_v3` | 4 000 pairs | **0.6970** |
| time-matched screen, Δ = 30 | `book_v2` holdout, 1 000 openings | **500 pairs** | **0.7430** |
| the same at 1 000 pairs | needs 1 500 openings | 1 000 pairs | 0.9189 |

**THREE THINGS FOLLOW AND NONE OF THEM IS A PREFERENCE.**

1. **`book_v3` funds exactly ONE acceptance arm at Δ = 10.** Splitting it between
   two arms drops each to **0.6970**, below D-653's own floor of 0.9.
2. **`book_v2`'s holdout cannot fund a time-matched screen at D-653's floor.**
   The rule `ceil_to_500(P+500) ≤ 1000` caps it at 500 pairs, whose measured
   power at Δ = 30 is **0.7430**; reaching 0.9189 needs 1 000 pairs and 1 500
   openings, which the holdout does not have. And the veto's own failure is
   measured too: at truth = 0 a 500-pair cap returns `h0` in only 0.7385 of runs,
   so the veto **fails to fire in 26 % of the cases it exists for**.
3. **So the field is priced against ONE acceptance run.** Whichever row the
   architect picks, the books do not fund a second row's acceptance without a new
   book. That is why D-708's separation of matrix from selection matters here
   more than usual: the selection is close to irreversible on the committed
   books.

Receipt `artifacts/wp22_phase2a/sprt_power/`, digest
`37a86fe1991ec1c54b5d7e299a92075dc4115d0594cd00f6b34715a1e404030d`.

**AND THE PENTANOMIAL IS THE INSTRUMENT SEAT'S.** Every power figure above is
tilted from `2,3,8,7,4`, a 24-pair instrument-seat sample. No play-seat
pentanomial exists, so the time-matched arm's power is not computable at all
today — a third thing §1.4's capability owes.

### 1.7 The wall cost

- **Instrument seat**: the registered dry-run figure is **2.046 s per opening**
  (`matrix_wp22_quiet_scale.md:250-253`, at a 50 000-node seat). 8 500 openings
  is **4.83 h**, DERIVED at that seat and valid only there.
- **Play seat**: ESTIMATED and marked as such, because it cannot be measured
  without the capability §1.4 says does not exist. Arithmetic: 500 ms per answer
  configured, and the v5 anchor's own 1 040 pistol answers over 100 games
  (`sealbot_anchor_v7_protocol.md` §A2) is 10.4 answers per game per side, so
  ~20.8 s per pair; 500 pairs ≈ **2.9 h**, 1 000 pairs ≈ 5.8 h.
- **Training**: `training_pipeline_2026-09.md` §6 — *"the whole run is minutes on
  one GPU and well under an hour on CPU"* against *"one acceptance SPRT at Δ = 10
  is 2-3 hours at the instrument seat"*. **Any plan that treats training compute
  as the scarce resource has the ratio backwards.**

### 1.8 The eval's share of the profile, and what a codebook does to it

`docs/audit/repo_audit_2026-09.md` A-02, MEASURED at
`perf record -F 2000` over 2 105 samples on `bench_positions_v1.txt` line 2 at
`go nodes 400000`: **`31.77 % HandcraftedV0::delta`**, the top entry, with
`ThreatState::touch` at 13.45 % behind it.

`WINDOWS_PER_CELL = Axis::ALL.len() * WINDOW_LEN` = **18**
(`crates/pistol-core/src/window.rs:9`), touched per stone. A length-`L` codebook
touches `3L` (`eval_families_2026-09.md` §A6) — 21 at `L = 7`, 33 at `L = 11`,
39 at `L = 13`.

### 1.9 Fixed for every learned row, by rule rather than by this matrix

Trainer objective = deployed objective, no constant term (D-633, R-C). Tactical
terms pinned, quiet terms learned (D-622). Split by game
(`training_pipeline_2026-09.md` §1; `game` is one column in `extract.py`, D-659).
No symmetry augmentation; reversal-folded, axis-pooled codes
(`eval_families_2026-09.md` §0.4). Outcome term masked on rows without one
(`training_pipeline_2026-09.md` §2). The weight file is an artifact,
`weights_sha256` is the identity, a shape check refuses a mismatched load
(`training_pipeline_2026-09.md` §3). Quantization bound by construction, exact
oracle acceptance (§4). Codebook length chosen by observation receipt (D-641).
Offline metrics gate nothing (D-614).

---

## §2 STAGE E's RECEIPTS — what the enum turned out to be

`docs/experiments/hex_threat_enum_v1.md` revision 2, §7. Receipt
`artifacts/wp22_phase2a/census/`, digest
`aab7f4f6a8d450fac5609dce4635a21ea86ec658e4c42c11c9d89b98835e14a8`.

**`k` is COMPUTED and no cell of it is 816.** The enum is the quotient of a
property tuple drawn from the calculus's own queries, and the ladder is four
projections of that tuple whose boundaries are `LAW-SUPPORT`'s and
`DEF-WINDOW`'s.

| L | T4 `k` / `C(k+2,3)` | T3 | T2 | T1 |
|---|---|---|---|---|
| 7 | 25 / 2 925 | 25 / 2 925 | 15 / 680 | 12 / 364 |
| 9 | 98 / 161 700 | 57 / 32 509 | 27 / 3 654 | 20 / 1 540 |
| **11** | **357 / 7 647 059** | 121 / 302 621 | **47 / 18 424** | 36 / 8 436 |
| 13 | 357 / 7 647 059 | 121 / 302 621 | 47 / 18 424 | 36 / 8 436 |

**Four things the matrix prices on:**

1. **`L = 11` is the covering length and `L = 13` buys nothing.** `k(13) = k(11)`
   exactly, and over the corpus `L = 13` sees **232 distinct T4 classes against
   231 and 108 075 codes against 108 074** — one class and one code, for **18 %
   more window traffic per stone** (39 against 33). Every rung's `ω²` is LOWER at
   13 than at 11.
2. **The full tuple at `L = 11` is not affordable**: 7 647 059 nominal parameters
   against 8 174 025 scored-cell observations is **1.07 per nominal parameter**,
   against `eval_families` §A1's kill of `R-A1-L11` at 1.2 and Buro's ≥ 20.
   108 074 codes are observed at all, median 3, and the growth curve has no flat
   stretch.
3. **T2 at `L = 11` is the rung the corpus supports**: 18 424 nominal, **1 533
   observed codes, median 41 observations, 322 at or below Buro's ≤ 4 line and
   620 below his ≥ 20 line**, and a growth curve that moves 35 % over nine times
   the data.
4. **The registered purity criterion is MET at all sixteen cells, by 2.0x to
   3.6x** — the enum's `ω²` beats every matched-null replicate at every length
   and rung, so the classes group codes that share label value beyond what a
   partition of the same shape earns by chance. At `L = 11` T4 retains **62 % of
   the un-quotiented ceiling's `ω²` while collapsing 11 909 observed codes to
   231**.

**AND ONE FINDING AGAINST THE ROW AS `eval_families` §7 PROPOSES IT**, recorded
in the enum memo's §5.5 and not softened here: **the tuple merges patterns whose
exact `DEF-T` after a stone at the cell differs — 27 of the 357 classes, 5 348 of
59 049 patterns** — and `t = 1` against `t = 2` is the calculus §5 table's own
`PAT-C4` / `PAT-O4` split, which `RULE-EXACT` makes the truth. The maximum exact
`DEF-T` a single axis can carry is **2** (MEASURED over all `3^11` lines), so
`LAW-OVERLOAD`'s `t ≥ 3` is unreachable by any single-axis class and the merge is
not repairable inside one.

## §3 THE SEED BUDGET, MEASURED

Instrument `tools/hex_enum/seed_pilot.py` at `54eb3ba`, the R-A5-TOPK shape — a
free weight per top-`K` folded `L = 11` window code plus one rare bucket, ordinary
least squares against the search label, solved from per-GAME sufficient
statistics so that eight splits cost one corpus walk. **45 271 quiet rows over
3 487 games, 1-in-8 of GAMES held out, 8 seeds.** Receipt
`artifacts/wp22_phase2a/seed_pilot/`, digest
`f3518ad221413a042a616d3c6348458afc089fba6f30382734c802540592c7bd`.

| K | parameters | val MSE, mean of 8 | spread (max − min) | spread as % | worst weight deviation, % of table scale |
|---|---|---|---|---|---|
| 8 | 9 | 657 864 | 36 705 | **5.58 %** | 8.06 % |
| 32 | 33 | 634 392 | 43 863 | **6.91 %** | **61.32 %** |
| 64 | 65 | 620 985 | 42 952 | **6.92 %** | **64.92 %** |

**THREE READINGS, AND THE THIRD IS THE ONE THAT PRICES THE ROWS.**

1. A single-seed validation number on this corpus is worth **±3.5 %**.
2. **Individual weights move by up to 65 % of the table's scale between splits**
   at 33 parameters and above. A fitted table is not a stable object here.
3. **The gain from 9 parameters to 65 is 5.6 %, and the seed spread is 6.9 %.**
   So on this corpus **an architecture comparison by offline loss is not
   separable from split noise at a single seed** — which is D-614 arrived at from
   a direction D-614 did not take, and it is why the seed column below reads in
   ACCEPTANCE RUNS rather than in training runs.

**The closed-form rows have no training seed at all** — `tools/texel/fit.py`'s own
docstring, *"no seed, no learning rate, no stopping rule"* — so their seed budget
is one fit and ≥ 8 splits for any reported validation number. **A trained row's
is different in kind**: `training_pipeline_2026-09.md` §3 quotes the reference
trainer, *"networks trained from identical settings with different seeds vary in
strength measurably… There's a good chance that at least one run from a set of 4
is of relatively good quality"*, so a trained row needs ≥ 4 nets — and §1.6
measures that the books fund **one** acceptance run, so those four cannot each be
accepted.

---

## §4 THE ROWS

**Every row is present, killed rows included, and each is priced on its own
stated unit.** The unit matters and §5 says why: `R-A4-CLASS` is a per-CELL row
by construction (`C(k+2,3)` is a size-3 multiset over the three axes at one
cell), and `R-A1`, `R-A2`, `R-A3`, `R-A5` are per-WINDOW rows in the shape
`eval_families` states them, which is the shape `handcrafted_v0` already sums in.

### R-A4-CLASS — length-11 windows quotiented through the Stage E enum, scored on the 3-axis multiset

| column | value |
|---|---|
| **parameters** (DERIVED) | at `L = 11`: **T4 7 647 059**, T3 302 621, **T2 18 424**, T1 8 436. At `L = 7`: T4 2 925, T2 680. |
| **observations per parameter** (MEASURED, per scored cell) | T4 **1.07 per nominal**, 75.6 per observed code, **median 3**; T2 **444 per nominal**, 5 332 per observed code, **median 41**; T1 969 per nominal, median 53. Population 8 174 025 scored cells over 45 271 positions (180.6 per position). |
| **window touches / stone, and the ratio to 18** (DERIVED) | `3L` = **33**, **1.83x**. `L = 13` is 39 (2.17x) and buys one class and one code. |
| **nodes/sec floor, PRE-REGISTERED as a bench bracket** (ESTIMATED) | Arithmetic on the page: today 531 548 nps with the eval at 31.77 % of the profile; a codebook at 1.83x the traffic and a per-touch cost ratio `c ∈ [0.5, 1.5]` gives `nps ∈ [341 617, 546 003]` = **[0.64x, 1.03x]**. **FLOOR REGISTERED AT 341 617 nps (0.64x)**; below it the row aborts on cost, per hard rule 5's abort-threshold discipline. |
| **determinism and quantization** | Closed-form fit, no seed. Integer after bake; one class lookup plus one code lookup per touch. The accumulator is bounded by construction: `3L` active windows times the clamped maximum, `training_pipeline_2026-09.md` §4's *"no combination of possible active features can exceed the maximum value"*. `EVAL_MAX = 16 000` with an i64 intermediate is the shape to keep. |
| **seed budget** (MEASURED, §3) | **1 fit, ≥ 8 splits** for any validation number; ±3.5 % on a single split. No training seed. |
| **books, BOTH arms** | node-matched Δ = 10: **8 000 pairs on `book_v3`, power 0.9045, the whole book**. Time-matched: **not runnable** (§1.4) — no arena movetime, no named play seat, no play-seat pentanomial. |
| **kill condition** | The registered purity criterion failing at the chosen rung — MET at all sixteen cells, so **not fired**. The row's own §7.1 arithmetic kills **T4 and T3 at `L = 11`** on density (1.07 and 27 observations per nominal parameter). |
| **strongest known attack** | **The `t = 1` / `t = 2` merge** (§2, enum memo §5.5): 27 of 357 classes contain patterns whose exact `DEF-T` after a stone at the cell differs, and `RULE-EXACT` (`threat_calculus_v1.md:64-66`) makes `t` the truth. The row therefore assigns one weight to a cell buying a hittable single plan and to one buying an unhittable pair, in the simplest four-shapes the game has — and it is not repairable within a single-axis class (max single-axis `t` is 2). |

### R-A5-TOPK — top-K folded codes plus one rare bucket

| column | value |
|---|---|
| **parameters** (DERIVED) | **`K + 1`, and `K` DEPENDS ON THE UNIT** — see the attack. At `eval_families` §0.2's `(axis, start)` window unit, **811** (810 folded L11 codes cover 90 %). At the cell-centred window unit this package measures, **143** (142 codes cover 90 % of 12 980 519 observations). |
| **observations per parameter** (MEASURED) | at §0.2's unit, 16 613 729 window observations over 38 983 codes; at the cell-centred unit, 12 980 519 over 11 909 codes, median 7, **4 730 codes at or below Buro's ≤ 4 line**. The top-`K` head is dense either way; the rare bucket absorbs the tail. |
| **window touches / stone** (DERIVED) | `3L` = **33**, **1.83x**. |
| **nodes/sec floor** (ESTIMATED) | same traffic as A4, **[341 617, 546 003]**, floor **341 617 (0.64x)**. Cheaper per touch than A4 in principle (one table lookup, no class indirection), so the bracket is the same and the row should land nearer its top. |
| **determinism and quantization** | closed form, no seed; trivial determinism; low quantization risk. |
| **seed budget** (MEASURED, §3) | **this row IS the pilot's shape**: 1 fit, ≥ 8 splits, ±3.5 % on one. At `K = 64` the worst weight moved **64.92 %** of the table scale between splits. |
| **books** | as A4: one node-matched arm, whole book. |
| **kill condition** | `eval_families` §A5: *"the rare tail is where novel structure lives and one bucket scores all of it alike"* — unmeasured, and this matrix does not measure it either. |
| **strongest known attack** | **The row's headline parameter count is a different number at each unit, and `eval_families` §7 states only one of them.** 811 is §0.2's window census; 143 is the same 90 % rule at the unit `R-A4-CLASS` must use. A field that prices A4 per cell and A5 per window is pricing two rows on two instruments (D-477's own class), and the row is not well posed until the architect names the evaluator's summand. |

### R-A2-L11F — mapped codebook, shared-weight generator, length 11, folded

| column | value |
|---|---|
| **parameters** (DERIVED from the source's ratio, EMP for the sizes) | Rapfi's smallest configuration is **14 160 generator parameters producing a 397 488-entry codebook** (`eval_families` §A2, Table 1). The hex analogue's codebook is the folded L11 code space, **38 983 observed cells** at §0.2's unit; the generator's size is a design choice Phase 2b makes. |
| **observations per parameter** (DERIVED) | against 14 160 generator parameters and 45 271 positions, **3.2 positions per parameter** — and `training_pipeline_2026-09.md` §6 reports two published sizing rules that disagree by 100x (nodchip's ≥ 10 positions per parameter, which caps this at ~4 500; ChessBench's measured overfit at 0.08), so **the rules do not settle it and both are reported**. |
| **window touches / stone** (DERIVED) | **33**, 1.83x — the bake makes inference one lookup per window, as A5. |
| **nodes/sec floor** (ESTIMATED) | **[341 617, 546 003]**, floor **341 617 (0.64x)** — identical to A4/A5 at inference, because the generator is offline. |
| **determinism and quantization** | **The training run is NOT reproducible** — `training_pipeline_2026-09.md` §3 quotes PyTorch's own page, *"Completely reproducible results are not guaranteed across PyTorch releases, individual commits, or different platforms"* — so **the reproducible artifact is the weight file by digest, never the run**. The engine stays deterministic after the bake (int16 features, int32 accumulation, `eval_families` §A7). **AND THE DEPENDENCY DOES NOT EXIST HERE**: this workstation's `python3` has no `numpy`, let alone torch, so the trainer is a package before it is a row. |
| **seed budget** (MEASURED + ARCH) | **≥ 4 trained nets** by the reference trainer's own standard, on top of the ≥ 8 splits §3 measures. **The books fund ONE acceptance run**, so four nets cannot each be accepted and the row must choose one by an offline metric — which Rapfi Table 2 measures ranks nets WRONGLY for strength (*"300-400 ELO… at equal wall clock"* against the loss ordering). |
| **books** | one node-matched arm, whole book; the seed budget makes that a selection problem the other rows do not have. |
| **kill condition** | `eval_families` §7: *"it does not beat A3's validation loss, or the nps floor is missed"* — and §3 measures that the first half is not separable from split noise at 65 parameters, so the kill as written cannot fire cleanly. |
| **strongest known attack** | **Three games agree that the alpha-beta eval-width optimum sits BELOW the accuracy optimum** (`eval_families` §A7): Rapfi's Mixnet Large scores **−82 / −58 / −45 / −38 Elo** against Small under α-β while scoring **+57 to +84** under MCTS; Logistello's three large patterns cost ~45 % speed and *"could not compensate"*; figrid's 1 024-accumulator net lost **23 points** to the 512 one at ~1.1 M samples. A2 is the row that buys accuracy with throughput, under the paradigm where that trade is measured to lose. |

### R-A3-L11F+F7 — free length-11 folded table with the length-7 folded factor

| column | value |
|---|---|
| **parameters** (DERIVED) | real parameters are A1's: **38 983** folded L11 codes at §0.2's unit. The factor — 1 029 folded L7 cells, none under ten observations — is a **training-time device with zero inference cost**, coalesced into the real weights (`eval_families` §A3). |
| **observations per parameter** (MEASURED) | **1.2** at §0.2's unit — `eval_families` §A1's own kill figure for A1, unchanged, because the factor does not add observations. The factor's own density is 1 282 median. |
| **window touches / stone** (DERIVED) | **33**, 1.83x. |
| **nodes/sec floor** (ESTIMATED) | **[341 617, 546 003]**, floor **341 617 (0.64x)** — identical to A1 by construction, which is the row's selling point. |
| **determinism and quantization** | closed form if the virtual features are fitted by least squares; trivial determinism; risk as A1. |
| **seed budget** (MEASURED, §3) | 1 fit, ≥ 8 splits. |
| **books** | one node-matched arm, whole book. |
| **kill condition** | Buro's `≤ 4`-match rule zeroing most of the table, and Stockfish's own caveat. |
| **strongest known attack** | **The remedy's source disowns it at this scale.** `eval_families` §A3 quotes Stockfish verbatim — *"just adding more factors… may even cause it to regress"*, and it *"seems to only be relevant in the early stages"* — said against **16 billion** positions where this corpus has 89 805. *"The early stage" is where a Phase-2 fit permanently lives*, which is the document's own sentence, and it is an argument for the row and an argument that its evidence base is 200 000x away. |

### R-A1-L11 — free table at length 11, unfolded. **KILLED ON THE RECORD, shown so the matrix scores it**

| column | value |
|---|---|
| **parameters** | **62 370** unfolded L11 cells (`wp22_phase2_premise.md` §3); 38 983 folded. |
| **observations per parameter** | **1.4** unfolded, **1.2** folded (`eval_families` §A1, DERIVED). Against Logistello's **7.3** and Buro's *"the weight is set to 0"* below ~4 matches. |
| **kill** | `wp22_phase2_premise.md` §3, MEASURED: at `L = 11` **61.9 % of unfolded cells carry fewer than ten observations and 16.0 % carry exactly one**; folded, 56.8 % under ten. §0.4 kills it independently — an unfolded axis-pooled codebook gives twelve feature vectors to one position (**1 760 of 2 200 images differ**), so it learns twelve answers or needs a 12-fold augmentation the corpus's own dedup forbids. |
| **why it is still a row** | so the field shows a scored kill rather than an omission, which is round 1's M-4 against the last matrix. |

### R-C-SPSA — in-engine tuning, no labels

| column | value |
|---|---|
| **parameters** | ~a dozen integers (`eval_families` §6). |
| **observations per parameter** | none — it never reads the corpus, and it is **the only family D-621, D-626 and D-629 do not touch**. |
| **window touches / stone** | unchanged, **18, 1.00x** — it tunes the existing table. |
| **nodes/sec floor** | **no change registered**; the row costs no throughput at all. |
| **determinism** | the tuner is stochastic, the **engine** stays deterministic (D-7 unaffected). |
| **seed budget** | the tuner's own; not a fit. |
| **books — AND THIS IS THE KILL** | fishtest's live data, **EMP**, over 46 sampled runs: a **median of 120 000 games for 4 parameters**. 120 000 games is 60 000 pairs is **`ceil_to_500(60 500) = 61 000` openings — 7.2x the whole of `book_v3`** — and `book_v3` may not be used for it at all: D-644 makes it *"acceptance-only (no labels, no tuning, no corpus input)"*. D-635's defect class applies on top: tuning and acceptance books must be disjoint or the acceptance run is a self-match. **At the instrument seat's 2.046 s per opening that is 34.7 h of games before any acceptance run.** |
| **kill condition** | fired: no committed book funds it and the one book that could is barred by ADR. |
| **strongest known attack — against the kill** | the 120 000 figure is a chess median and **EMP, NON-TRANSFERABLE**; a dozen integers on a smaller board might converge far sooner, and nothing here measures that. The kill is a cost kill on the only evidence available, not a proof of infeasibility. |

### R-H-EXT — extended handcrafted: the null-plus, and the row the learned rows must beat on cost

| column | value |
|---|---|
| **parameters** | v0's five, plus the terms the calculus already names as candidates: a **tempo census** per side (`ADOPT-TEMPO`, `threat_calculus_v1.md:177`, *"tempo census … eval-term candidate"*, audited exact at §6), **exact-`t` counters as eval terms** — `THM-WINDOW`'s own cheapest listed fix, `:153`, *"exact-t counters as eval terms (free from\n  PROTO-NODE)"* spanning `:153-154` — and `E-INIT`'s initiative discount (`:158`). **~3 to 6 more integers.** |
| **observations per parameter** | not applicable: the terms are hand-set and SPRT-gated, which is what the calculus says they are. |
| **window touches / stone** (DERIVED) | **18, 1.00x**. The exact-`t` counters are computed by `PROTO-NODE` already; `ThreatState::touch` is 13.45 % of the profile whether or not a node reads a threat query (`repo_audit_2026-09.md` A-03), so the counters are close to free. |
| **nodes/sec floor** (ESTIMATED) | traffic unchanged, arithmetic per touch up: `c ∈ [1.2, 2.0]` gives `nps ∈ [403 391, 499 791]` = **[0.76x, 0.94x]**. Floor registered at **403 391 (0.76x)**. |
| **determinism and quantization** | unchanged from v0 — integer, no fit, no artifact, no digest problem, no shape check owed. |
| **seed budget** | **zero**. There is nothing to seed. |
| **books** | one arm. **AND IT IS THE ONLY ROW THAT CAN USE THE SCREENING BOOK**: a hand term either moves play a lot or it does not, so it is a large-effect question, and §1.6 measures that `book_v2`'s holdout reaches D-653's 0.9 floor at **elo1 = 40 (0.9067)** and **50 (0.9467)** at a 500-pair cap. |
| **kill condition** | it fails its own SPRT, at which point the incumbent stands and the learned rows are measured against v0 rather than against v0-plus. |
| **strongest known attack — and it is an attack on the FIELD, not on the row** | **The calculus has called the tempo layer *"a cheap eval-term candidate (SPRT-gated)"* since v1.0 and nothing has SPRT-gated it.** So R-H-EXT is not a Phase-2 alternative at all — it is Stage-0 work nobody did, and pricing it beside a learned family makes an unrun cheap experiment look like a considered rival. The counter is D-614's own bar: the ROADMAP's Stage-2 acceptance is *"vs handcrafted_v0"*, so if v0-plus beats v0 the bar itself moves and every learned row was measured against the wrong incumbent. **That is a reason to run it FIRST, not a reason to leave it out.** |

### R-D-W1 — the one-integer SPRT (`w1 = 1` vs `w1 = 4`). **KILLED BY RULING (D-704), shown so the matrix carries the kill**

| column | value |
|---|---|
| **parameters** | one integer, six admissible values (D-633). |
| **the question it would have asked** | which of two ESTIMATORS makes the better player — the no-intercept objective the engine minimises (which wants `w1 = 1`) or the tempo-fitted one (which wants 4), where committed is 2 and *"the disagreement is in SIGN, not in magnitude"*. |
| **the kill, in two measured halves** | (a) **R-C fixes the trainer objective to the deployed objective with no constant term for every learned row**, so the estimator question is settled by standing rule for the family that will ship; (b) D-633 measures the two tables' validation MSE at **724 169 and 728 957**, a **0.66 %** difference, and §1.6 prices an SPRT at Δ = 10 at 8 000 pairs and the whole book. Nothing measured suggests one unit of `w1` is a 10-nelo change. |
| **what answers R7 instead** | the selected family's acceptance SPRT, which has to happen anyway. |
| **strongest attack against the kill** | D-633's own flip clause said the line *"flips when that comparison is run"*, and killing the run means D-633's finding — that two estimators disagree in sign — is never adjudicated. The answer is that it is adjudicated by rule (R-C) rather than by measurement, and a rule is what the operator chose. |

---

## §5 THE TWO ACCEPTANCE ARMS, REGISTERED PER ROW — AND THE FINDING IS THAT THEY DO NOT DISCRIMINATE

D-705: *"a Stage 2 eval row registers a node-matched arm at the instrument seat
and a time-matched arm at the play seat; acceptance = node-matched h1 AND
time-matched not h0; each row pre-registers a nodes/sec floor for the bench
bracket."*

| row | node-matched arm | time-matched arm | nodes/sec floor registered |
|---|---|---|---|
| R-A4-CLASS | Δ = 10, 8 000 pairs, `book_v3` whole, power **0.9045** | **not runnable** (§1.4) | **341 617** (0.64x) |
| R-A5-TOPK | as above | not runnable | 341 617 (0.64x) |
| R-A2-L11F | as above, and it needs **≥ 4 nets** for one arm (§3) | not runnable | 341 617 (0.64x) |
| R-A3-L11F+F7 | as above | not runnable | 341 617 (0.64x) |
| R-A1-L11 | killed before the arm | — | — |
| R-C-SPSA | needs 61 000 openings of tuning FIRST (§4) | not runnable | no change |
| R-H-EXT | Δ = 10 on `book_v3`, **or** a large-effect screen on `book_v2`'s holdout at **elo1 = 40, power 0.9067** | not runnable | **403 391** (0.76x) |
| R-D-W1 | killed by ruling | — | — |

**THE COLUMN THAT WAS MEANT TO SEPARATE THE ROWS IS CONSTANT.** The
time-matched arm is unavailable for every row for the same three reasons — the
arena refuses a movetime budget by name, no play seat is chosen, no play-seat
pentanomial exists — and none of them is a property of any row. **So the two-arm
rule cannot rank this field**, and a matrix that presented it as a per-row cost
would be presenting a package obligation as a discriminator. It is stated once,
in §1.4, and its discharge is one of the two questions the HANDUP puts to the
architect.

**AND ONE THING D-705 DOES NOT SAY IS TRUE OF THE COMMITTED BOOKS.** Its
acceptance rule needs two arms; §1.6 measures that `book_v3` funds one at
D-653's power floor and that `book_v2`'s holdout clears that floor only at
`elo1 ≥ 40`. **D-653 designates the holdout as the screening book at `elo1 ≥ 30`
and requires computed power ≥ 0.9, and at `elo1 = 30` with the holdout's own
1 000-opening cap the computed power is 0.7430.** That is D-628's shape — a
registered test that cannot return its registered outcome — found in a standing
ADR rather than in a registration, and it is reported here rather than worked
around.

## §6 THE RECOMMENDED ORDER — AND IT IS AN ORDER, NOT A SELECTION (D-708)

The books fund one acceptance run (§1.6). So the field's real question is not
"which row is best" but "which row is run first", and the ranking below is by
**what a row costs before it can be run at all**, with the measured content of
§2 and §3 breaking ties.

1. **R-H-EXT.** Zero new capability, zero seeds, zero artifacts, traffic
   unchanged at 18 touches per stone, and the only row that can be asked at a
   large effect on the screening book (power 0.9067 at `elo1 = 40`) without
   spending `book_v3` at all. It also produces the two things every later power
   computation needs and nobody has: a **play-seat pentanomial** and a
   **second instrument-seat pentanomial** to replace the 24-pair sample every
   figure in §1.6 is tilted from. And if it wins, D-614's bar moves and every
   learned row is measured against the right incumbent.
2. **R-A4-CLASS at `L = 11`, rung T2.** 18 424 nominal parameters, 1 533 observed
   codes at median 41 observations, a growth curve that has flattened, the
   registered purity criterion met at **3.58x**, a closed-form fit with no
   training seed, and no dependency this workstation lacks. Its own §7.1
   arithmetic kills T4 and T3 at this length, so the rung is not a free choice.
3. **R-A5-TOPK.** The same cost shape and the same throughput bracket, decided
   against A4 by whichever unit the evaluator's summand turns out to be — which
   is why it ranks below rather than beside: **its parameter count is 811 or 143
   depending on a question this matrix cannot answer.**
4. **R-A3-L11F+F7.** Zero inference cost over A1 and the documented remedy for
   §0.3, but its source disowns the remedy at 200 000x this corpus's size and
   its real parameter count is A1's killed 1.2 observations each.
5. **R-A2-L11F.** The strongest published shape and the one this project is
   furthest from being able to run: a trainer that does not exist here, a
   non-reproducible training run, **≥ 4 nets against one funded acceptance
   arm**, and the paradigm under which three games measure its trade to lose.
6. **R-A1-L11** — scored and killed (§4). **R-C-SPSA** — killed on cost against
   the ledger (§4). **R-D-W1** — killed by ruling (D-704).

**WHAT WOULD CHANGE THIS ORDER, stated so the architect can overrule it on
evidence rather than on taste**: a second acceptance book moves R-A2 up, because
its seed budget stops being a selection problem; a decision that the evaluator
sums over windows rather than cells swaps 2 and 3; and an ADR moving the arena's
movetime refusal makes §5's column real, at which point the throughput brackets
start discriminating and the row with the lowest floor is no longer indifferent.

**NOTHING IS SELECTED HERE** and no ADR of selection is written. §2's finding
against R-A4-CLASS — the `t = 1` / `t = 2` merge — stands beside its rank and is
not netted against it.

## §7 COSTS AND FAILURE MODES, one line each

| row | cost before it can be run | failure mode |
|---|---|---|
| R-H-EXT | none beyond writing the terms | it loses, and the incumbent stands — which is a finding and not a waste, because the calculus has been calling these terms candidates since v1.0 |
| R-A4-CLASS | a fit and a backend | the `t` merge (§2) is invisible to the fit: a wrong class is scored consistently wrong and the loss cannot see it |
| R-A5-TOPK | a fit and a backend | the rare bucket scores all novel structure alike, and the tail is where novel structure is |
| R-A3-L11F+F7 | a fit with virtual features and a coalescing step | the factor regresses, which its own source warns of, and 1.2 observations per real parameter is the density that killed A1 |
| R-A2-L11F | a trainer, a dependency, ≥ 4 nets, a digest discipline, a shape check | the α-β width optimum sits below the accuracy optimum, measured in three games |
| R-A1-L11 | — | killed: 1.2 observations per parameter, and unfolded it learns twelve answers |
| R-C-SPSA | 61 000 openings of tuning on a book that does not exist | killed on cost; the kill rests on a chess median and is marked EMP |
| R-D-W1 | — | killed by ruling: the estimator question is settled by R-C and a 0.66 % MSE difference is not a 10-nelo effect |

---

## §8 FAILED-PRECEDENT CHECK — every attack that felled `matrix_wp22_quiet_scale.md`, and how this matrix does not repeat it

That document reached **revision 5**, its own header records *"Rounds 1 through 4
all FAILED"*, D-630 stopped it and D-631 granted two more rounds that did not
close it, and **no selection ever landed**. One line each.

**AND ONE OF ITS CITATIONS DOES NOT RESOLVE, RECORDED RATHER THAN LEFT TO BE
REDISCOVERED**: its *"Reports:"* list names
`matrix_wp22_quiet_scale_REDTEAM_round4.md`, and `git log --all -- <that path>`
returns nothing at `54eb3ba` — the tree holds rounds 1, 2 and 3 only. Round 4's
report exists in a transcript and not in the repository, which is D-469's loss
shape in a document rather than in a worktree. **This matrix's own reviews land
as tracked files in the same commit as the revision they attack.**

| round | the attack that landed | how this matrix does not repeat it |
|---|---|---|
| 1 M-1 | the recommendation's argument was circular — the labels were the committed table's own opinion, so *"data-derived"* named nothing | §6 recommends an ORDER on measured cost-before-runnable, and the ranking's inputs (§1.6's power figures, §2's densities, §3's spreads) are all external to any table this matrix would produce |
| 1 M-2 | the option's whole diagnostic gain was absorption of a constant the feature set could not express | no row here is scored on a diagnostic at all: §6.5's purity criterion is registered as a constraint on what may be CONCLUDED, and D-614 is quoted at it |
| 1 M-3 | shape and scale were not separable — one decision presented as two | the one decision is which family, and §5 states plainly that the two-arm rule does not separate the rows rather than dressing it as a second axis |
| 1 M-4 | the option set was incomplete in the dimension the decision was about | every row `eval_families` §7 lists is present, killed rows shown with their kill, and the coarsening ladder gives A4 four priced rungs rather than one |
| 1 M-5 | the filter did not implement the predicate it was said to | this matrix defines no filter; it inherits R6's, whose coextension round 2 measured and round 3 reproduced |
| 1 M-6 | integer rounding at the chosen scale was a first-order confound of the ratios that decide moves | no table is fitted here and no rounding happens; the numbers are counts, powers and throughputs |
| 1 M-7 / 2 M-1 / 3 MAJOR-2 | **three consecutive revisions pointed at instruments that carried nothing, then did not exist, then could not run** | every instrument here is committed under `tools/hex_enum/`, driven by two suites through **CI gate 19**, and each was RUN in this session with its output receipted by digest (§10) |
| 1 M-8 | the operator ruling was quoted with four words it did not contain | every quotation here is reproduced at `54eb3ba` with `/usr/bin/grep`, and the two ADRs this matrix leans on hardest — D-653 and D-705 — are quoted whole rather than glossed |
| 2 M-2 | the registered table was not what the registered instrument computes | the instrument's outputs ARE the numbers: §2's table is `census_L*.txt` transcribed, and §3's is `pilot_L11.txt` |
| 2 M-3 / 3 MAJOR-1 | *"steps 1-4 admit exactly one table"*, then *"the only normalisation"* — a uniqueness claim the document's own adjacent column refuted | §6 claims no uniqueness at all. It claims an ORDER, states what would change it, and leaves the row's strongest attack standing beside its rank |
| 2 M-4 | the diagnostics were scale statistics and the licensing sentence was backwards | `ω²` is used, not `η²`, precisely because `η²` rises with class count; the matched null is what makes the comparison readable and its spread is printed |
| 2 M-5 | the play-change receipt could not be reproduced from what it recorded | every receipt here names its command, its scope and its digest, and the census files carry their own invocation parameters in their first lines |
| 2 M-6 / 3 MAJOR-3 | an ADR's binding consequence forbade the selected option and was never confronted | §5 confronts D-653 and D-705 head on, and reports that D-653's two clauses do not both hold on the committed books |
| 3 m-1 | a false invariant asserted in a governing document, pinned by a test that could not see the counterexample | Stage E's self-checks are enumerations over the whole pattern space, and the two that could pass vacuously (`Completed`, the `r4` count) are named and one is deleted |
| 3 m-7 | the dry run's registered "lean" recorded nothing | this matrix registers no run and therefore no lean; the runs it prices are Phase 2b's |
| 4 (report not in tree) | revision 4 selected on an authority that quantified the boundary as the SUM where the argument named the top entry; *"one of the two had to be"* was false | §6 names no authority that selects; it ranks on cost and says what would reorder it |
| the meta-pattern (D-630, D-631) | *"a fix round discharges the finding's SENTENCE and re-creates its PROPERTY one step to the left"* | Stage E's own fix round was confirmed by a fresh context RUNNING the attacks rather than reading the text (D-691), and this matrix's red team is dispatched the same way |

## §9 WHAT THIS MATRIX DOES NOT ESTABLISH

- **No selection, no ADR of selection, and no order that binds** (D-708).
- **No strength claim, no Elo, nothing that moves a committed file.** Nothing
  here touches `configs/`.
- **No time-matched number of any kind**, because §1.4 measures that the
  capability does not exist.
- **No play-seat pentanomial**, so every power figure in §1.6 is tilted from a
  24-pair instrument-seat sample and says so.
- **Nothing about the evaluator's summand**, which §4 and §6 both record as the
  question that reorders rows 2 and 3.
- **Nothing about `THM-WINDOW`**, which the enum memo §6.6 records as unclosed.

## §10 INSTRUMENTS AND RECEIPTS

| instrument | what it produced | receipt digest |
|---|---|---|
| `tools/hex_enum/{hexenum,census,report}.py` at `54eb3ba`, CI gate 19 | §2's `k`, codes, observations, purity, growth curves | `artifacts/wp22_phase2a/census/`, `aab7f4f6a8d450fac5609dce4635a21ea86ec658e4c42c11c9d89b98835e14a8` |
| `tools/hex_enum/seed_pilot.py` at `54eb3ba`, CI gate 19 | §3's seed spread, and §1.1's independent re-derivation of the corpus counts | `artifacts/wp22_phase2a/seed_pilot/`, `f3518ad221413a042a616d3c6348458afc089fba6f30382734c802540592c7bd` |
| `measure_nps.sh` / `measure_play.sh`, detached worktree at `c5123c1` | §1.2's and §1.3's throughputs, replicated | `artifacts/wp22_phase2a/nps_seat/`, `211b6e1002f32ee89baace6ed4ff510302b2aad7817094e109dcac121d51ca9e` |
| `crates/pistol-arena/examples/sprt_power.rs`, release at `54eb3ba` | §1.6's and §5's power figures, cross-checked digit for digit against the committed `book_v3` figure | `artifacts/wp22_phase2a/sprt_power/`, `d82fce37add53453423e264a6220e2b47636302c7f2980c4a60f6c600005709b` |
| `confirm_wedge.sh`, detached worktree at `0585e25` | §0's clock-fix confirmation (`wp22_phase2a_section0_receipt.md`) | `artifacts/wp22_phase2a/clockfix_confirm/`, `7678648c5dcf2418a5a0eb320f0faac3835c21ae792723742e737fac2345bb66` |

`artifacts/` is gitignored (hard rule 8), so an artifact directory is evidence
only while a tracked document carries its receipt's own digest. **This table is
that anchor.** Every worktree named above was exported before removal (D-469) and
`git worktree list` shows only the main tree.

**Cost of this matrix, MEASURED**: four census processes over the whole quiet
population, 6 to 20 minutes each in parallel; the seed pilot, under 10 minutes;
two release builds under 20 s each; the nps measurement, under 2 minutes per
replicate; `sprt_power` at 20 000 runs per cell, seconds. **No governed run was
taken and no opening was spent.**
