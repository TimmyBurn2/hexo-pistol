# OPTION MATRIX — the quiet-structure eval family for Phase 2, revision 6.

> **ROUND 4 OF FOUR UNDER D-709 — THE LAST ROUND THIS GATE HAS.** Three
> DECISION-RED-TEAM rounds have failed this matrix: `0641504` (4 BLOCKING, 11
> MAJOR, 10 MINOR), `564dc9e` (1 BLOCKING, 7 MAJOR, and all ten of round 1's
> MINORs still landing) and `c7a3ee1` (3 BLOCKING, 8 MAJOR, 10 MINOR). §12, §13
> and §14 are the round tables.
>
> **THE FIELD HAS EIGHT LIVE ROWS AND `R-A4-CLASS` HAS MOVED UP TWICE.** Revision
> 2 dropped it on a spent round cap; revision 3 dropped it on a conclusion false
> on its own census; revision 5 ranked it fourth as UNDECIDED on a nested figure
> of 0.93. **Stage E's fourth round measured that 0.93 to be an artifact of a
> null permuted over the whole code space — ~4x the cells of the join it
> refereed. Corrected, it is 1.71, the enum ADDS at all sixteen cells, and the
> row is PRICED AND WEAK.** It now ranks second, on the ground that it is the
> only row whose criterion was registered, could fail, was run, and did not fire
> — and it is kept out of first place by the SIZE of what it buys, 1.4 % to
> 10.8 % over counting stones.
>
> **TWO ROWS WERE MISSING FROM THE FIELD AND BOTH WERE ON THIS PAGE BEFORE THEY
> WERE PRICED**: `R-A1-L8F` (round 1) and `R-A1-L7F` (round 3), the second
> quoted in `R-A3`'s own factor cell.
>
> **AND ONE CORRECTION REACHES PAST THIS DOCUMENT**: `eval_families_2026-09.md`
> §A1's *"1.4 observations per parameter"* is the RECIPROCAL — the real density
> is **0.73 positions per parameter**.

**Governing revision**: `54eb3ba` (`dev`) for every file, line and count quoted
below (D-692), except where a paragraph names a later one.

**THIS MATRIX SELECTS NOTHING** (D-708). It states the field, prices every row on
its own instrument, ranks them, and hands the selection to the architect. A fresh
DECISION-RED-TEAM attacks it before anything is chosen; the ADR that selects is a
separate act and records the strongest surviving attack.

**Every number is MEASURED, DERIVED or ESTIMATED**, and an estimate that could
have been measured in seconds is a finding (D-291). Where a number came out of a
run, the run's artifact and its receipt digest are named (D-483). §8 is the
failed-precedent check the last matrix in this package earned; §12 is the round
table.

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

**THE FIGURE IS THE RECEIPTED ONE AND AN EARLIER REVISION PRINTED THE OTHER.**
The script was executed twice; the receipt
`artifacts/wp22_phase2a/nps_seat/nps_instrument.txt` holds the SECOND execution
and revision 2's table printed the FIRST, so its headline was a number the
receipt did not contain. **Corrected: every figure below is the one in the
receipt**, and the first execution is reported as the replicate it is.

| seat | band | nps, median of 5 reps (RECEIPTED) | min | max |
|---|---|---|---|---|
| `instrument_v0`, `go nodes 50000` | **all** | **529 255** | 526 479 | 531 293 |
| | early | 564 304 | 559 064 | 568 030 |
| | late | 492 073 | 491 110 | 494 010 |
| `instrument_v0`, `go depth_turns 2` | all | 453 012 (73 388 nodes in 162 ms) | | |

**REPLICATED, because the run is cheap and the rule says to replicate rather
than derive a margin** (`docs/process.md`, *"Cost, replication, and the second
instrument"*): the unreceipted first execution gave **531 548** all-bands,
**0.43 %** from the receipted one. Receipt `artifacts/wp22_phase2a/nps_seat/`,
`sha256sum -c` clean, receipt digest
`211b6e1002f32ee89baace6ed4ff510302b2aad7817094e109dcac121d51ca9e`; the binary
that produced it is
`41b2c5df832fb37ee55ddff62901d641e73841cbe22965aadc1b0e3da66e0607`.
**EVERY nps FLOOR IN §4 IS RECOMPUTED FROM 529 255.**

### 1.3 THE PLAY SEAT IS NOT ONE SEAT, AND THE TWO DIFFER BY 2.8x

D-705 registers a **time-matched arm at the play seat**. **THREE committed
configs carry `mode = "play"`** — `configs/play_v0.toml`,
`configs/play_staged_v0.toml` and `configs/play_staged_solver_v0.toml`, the last
of which its own header calls a MEASUREMENT seat rather than a deployment one,
which is why the two below are the candidates. They are not the same search: `configs/play_v0.toml` commits
`candidate_policy kind = "radius", radius = 3` and `configs/play_staged_v0.toml`
commits `kind = "staged"`. `play_v0.toml`'s own comment states the disagreement
is deliberate and unresolved — *"this is the FIRST TIME this file deliberately
disagrees with `configs/instrument_v0.toml` … Until that verdict this value does
not move"*.

MEASURED, same worktree, same 24 positions, `go movetime 500`, 3 reps:

| config | nps, three reps | mean `depth_turns` reached |
|---|---|---|
| `configs/play_v0.toml` | **172 627** / 172 665 / 172 211 | **1.21** |
| `configs/play_staged_v0.toml` | **478 718** / 480 449 / 482 105 | **3.42** |

**THE RECEIPT HOLDS THE SCRIPT AND A SECOND EXECUTION'S OUTPUT, NOT THESE SIX
NUMBERS**, for the same reason §1.2 records: `measure_play.sh` was re-run into
`nps_play.txt` after the numbers above were taken, and the two executions are not
byte-identical because the wall figures are not. **The 2.8x ratio and the depth
figures reproduce in the receipted run**; the six digits above are the first
execution's and are labelled so rather than presented as receipted.

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

And its consumed-ranges table reads **`nothing yet`** — the book is whole. **AND
IT IS NOT UNCLAIMED, WHICH REVISION 3's "book cost is EQUAL across every live
row" NEEDS AND DID NOT SAY.** The ledger's *"Standing claims on the book"*
section names one: *"The R7 acceptance SPRT | **THE REASON THIS BOOK EXISTS**
(D-638, D-643) | 8000 pairs at the registered bounds; the book covers it with 500
openings to spare."* So the eight thousand pairs `book_v3` funds are already
spoken for by R7, and every row in §4 is competing for the same claim rather than
for an unclaimed book. **What makes that survivable is that R7's question —
*does this corpus move Elo at all* — is answered by whichever family's
acceptance SPRT runs** (D-704), so the claim and the rows are the same run. That
is an argument, not a measurement, and it is the architect's to accept. Its
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

| arm | book | pairs the run is capped at | power (`h1`) at truth = Δ |
|---|---|---|---|
| node-matched, Δ = 10 | `book_v3`, 8 500 openings | **8 000 pairs** | **0.9045** |
| node-matched, Δ = 10, half the book | `book_v3` | 4 000 pairs | **0.6970** |
| screen, Δ = 30 | `book_v2` holdout, 1 000 openings | **1 000 pairs** | **0.9189** |
| the same at a 500-pair cap | half the holdout | 500 pairs | 0.7430 |
| screen, Δ = 40 | half the holdout | 500 pairs | 0.9067 |
| screen, Δ = 50 | half the holdout | 500 pairs | 0.9467 |

**THREE THINGS FOLLOW AND NONE OF THEM IS A PREFERENCE.**

1. **`book_v3` funds exactly ONE acceptance arm at Δ = 10.** Splitting it between
   two arms drops each to **0.6970**, below D-653's own floor of 0.9.
2. **`book_v2`'s holdout DOES clear D-653's floor, and revision 2's claim that
   it does not was a misreading twice over.** D-653 writes `elo1 >= 30`, not
   `= 30`, so a screen at `elo1 = 40` (power **0.9067**) or 50 (**0.9467**)
   satisfies both of its clauses — as this matrix's own §5 row already said three
   lines away. And the 500-pair cap that produced the alleged conflict came from
   INVERTING `ceil_to_500(P + 500)`, which is a rule for SIZING a book to be
   built with margin, not a bound on how many pairs an existing book seats: a
   paired opening is played twice, so **1 000 openings seat 1 000 pairs**. At
   1 000 pairs the measured power at Δ = 30 is **0.9189** and the veto's failure
   at truth = 0 is **8 %**, not 26 %. The finding against the ADR is DELETED.
3. **So the field is priced against ONE acceptance run.** Whichever row the
   architect picks, the books do not fund a second row's acceptance without a new
   book. That is why D-708's separation of matrix from selection matters here
   more than usual: the selection is close to irreversible on the committed
   books.

Receipt `artifacts/wp22_phase2a/sprt_power/`, digest
`d82fce37add53453423e264a6220e2b47636302c7f2980c4a60f6c600005709b` — the
digest AFTER `screen_power.txt` joined the directory; revision 2 printed the
digest from before it, which matched nothing on disk.

**AND THE PENTANOMIAL IS THE INSTRUMENT SEAT'S.** Every power figure above is
tilted from `2,3,8,7,4`, a 24-pair instrument-seat sample. No play-seat
pentanomial exists, so the time-matched arm's power is not computable at all
today — a third thing §1.4's capability owes.

### 1.7 The wall cost

- **Instrument seat, and the two figures below are not in conflict**: 8 500
  openings is the WHOLE BOOK and 4.83 h is what playing all of it costs, while
  `training_pipeline_2026-09.md` §5's *"one acceptance SPRT at Δ = 10 is 2-3
  hours"* is what a run EXPECTS to spend — a sequential test stops when it
  crosses, and `book_v3_registration.md` §R1 measures `mean_pairs 3102.1` at the
  registered bounds, which at 2.046 s is **1.76 h**. The book is sized for the
  worst case and the run pays the mean. The registered dry-run figure is
  **2.046 s per opening**
  (`matrix_wp22_quiet_scale.md:252-253`, *"a MEASURED 2.046 s per opening in the design's registered dry run"*, at a 50 000-node seat). 8 500 openings
  is **4.83 h**, DERIVED at that seat and valid only there.
- **Play seat**: ESTIMATED and marked as such, because it cannot be measured
  without the capability §1.4 says does not exist. Arithmetic, with the per-answer term MEASURED rather
  than configured (D-291, and the measurement was inside this package): the
  receipted `nps_play.txt` gives **11 502 ms and 10 978 ms for 24 answers —
  479 ms and 457 ms each**, not the 500 configured. At 479 ms and the v5
  anchor's own 1 040 pistol answers over 100 games
  (`sealbot_anchor_v7_protocol.md` §A2, 10.4 per game per side), a pair is
  ~19.9 s; 500 pairs ≈ **2.8 h**, 1 000 pairs ≈ 5.5 h.
- **Training**: `training_pipeline_2026-09.md` §6 — *"the whole run is minutes on
  one GPU and well under an hour on CPU"* against *"one acceptance SPRT at Δ = 10
  is 2-3 hours at the instrument seat"*. **Any plan that treats training compute
  as the scarce resource has the ratio backwards.**

### 1.8 The eval's share of the profile, and what a codebook does to it

`docs/audit/repo_audit_2026-09.md` A-02, MEASURED at
`perf record -F 2000` over 2 105 samples on `bench_positions_v1.txt` line 2 at
`go nodes 400000`. **THE EVAL'S SHARE IS 44.70 %, NOT THE 31.77 % AN EARLIER
REVISION USED**: the same receipt lists `31.77 % HandcraftedV0::delta`,
`6.64 % HandcraftedV0::undo` and `6.29 % HandcraftedV0::apply`, and all three are
`HandcraftedV0` methods, and `perf report --no-children` reports SELF time, so
the sum is a clean partition with no double count. *(An earlier revision
justified this by the `Eval` trait's default `delta` being the apply/undo
roundtrip. That is FALSE at the code: `HandcraftedV0` OVERRIDES `delta` at
`crates/pistol-eval/src/handcrafted.rs:302` with a read-only incremental body
that calls neither, so the trait default never runs at the profiled seat. The
override makes the partition MORE certainly clean, not less — the number was
right and the reason was wrong.)* Taking only the top line understated
the share by 12.93 points and made **every registered nps floor about 14 % too
lenient**; §4's floors are recomputed at 44.70 %. `ThreatState::touch` at
13.45 % is the next entry and is NOT the eval.

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

`docs/experiments/hex_threat_enum_v1.md` **revision 3**, §7, whose census is
receipted at `artifacts/wp22_phase2a/census_r3/`, digest
`9bb8fc6f2e0500bcf10f48e493dd9e3615897d935b73b63d8a8f47bf179e9c39`, and which
**replicates round 2's run (`artifacts/wp22_phase2a/census/`, digest
`aab7f4f6a8d450fac5609dce4635a21ea86ec658e4c42c11c9d89b98835e14a8`) on all 200
cells the two share, 0 differences**.

**`k` is COMPUTED, and no SHIPPED cell of it is 816** — with the qualification
the enum memo's own §6.4 now carries: of the **twenty** clips that express
`LAW-SUPPORT`'s two boundaries, five give `k = 16` and `C(18,3) = 816`, and the
coarsest gives `k = 9` and 165. The shipped clip is one of twenty and is the
second-finest. **816 is reachable by derivation from this game's own laws**, and
that it coincides with `C(16+2,3)` for Rapfi's hand-designed enum is arithmetic
about small multisets, not provenance. The enum is the quotient of a
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
4. **THE ROW IS LIVE AND SMALL.** The stone-count criterion fails at 12 of 16
   cells and passes at four, all T4; the corrected nested test says the enum ADDS
   at **all sixteen**; and **`L = 9` T4 is affordable (40.4 observations per
   nominal parameter), MET (1.108) and adds (1.64)**. The margins — 1.4 % at
   `L = 7`, 10.8 % at `L = 9` — are the finding. **§2.5.**

   *(Revision 2 reported *"MET at all sixteen cells, by 2.0x to 3.6x"*, which was
   the RANDOM-permutation referent — a FLOOR whose only claim is that the enum is
   not noise. Revision 3 replaced it and then over-read the replacement. Both are
   named now, and §2.5 says which half of each is evidence.)*

### §2.5 THE ROW IS PRICED AND WEAK

`docs/experiments/wp22_phase2a_STOP_E.md`, which carries Stage E's corrected
measurement after its fourth round. **Two criteria, and they now agree on the
one thing that matters.**

**(a) The REGISTERED criterion** — the enum's `ω²` must exceed the stone-count
quotient's, window unit, no free parameter — **fails at 12 of 16 cells and
passes at four, all at T4**:

| L | rung | ratio | | L | rung | ratio | |
|---|---|---|---|---|---|---|---|
| 7 | **T4 / T3** | 1.014 | MET | 11 | **T4** | 1.125 | MET |
| 7 | T2 | 0.793 | FAILS | 11 | T2 | 0.637 | FAILS |
| 9 | **T4** | 1.108 | MET | 13 | T4 | 0.834 | FAILS |

Its FAIL side is evidence; **its PASS side is not** — the statistic is
class-count monotone under position-clustered labels, and T4 carries 231 classes
against the referent's 52 at `L = 11`.

**(b) The NESTED test, with its null CORRECTED.** Revision 5 reported this test
against a null permuted over the whole code space, whose join had **~4x the cells
of the join it refereed**. That is a correctness defect, it was fixed at the code
(`permuted_within_counts`, matching at 1.00x everywhere), and the corrected run
over the full population at twelve replicates says:

| L | rung | increment | null mean | ratio, mean | ratio, max |
|---|---|---|---|---|---|
| 7 | T4 | +0.000642 | +0.000241 | **2.67** | **1.71** |
| 9 | T4 | +0.001902 | +0.001011 | **1.88** | **1.64** |
| 11 | T2 | +0.001576 | +0.000485 | **3.25** | **2.51** |
| 13 | T2 | +0.001962 | +0.000599 | **3.27** | **2.48** |

**THE ENUM ADDS AT ALL SIXTEEN CELLS, UNDER BOTH SUMMARIES**, 1.36x to 3.50x.
Revision 5's `L = 7` T4 figure of 0.93 — the number that made the row UNDECIDED
and set its rank — was an artifact of the unmatched null and is **1.71**
corrected.

**(c) SO THE ROW IS LIVE, AND ITS SIZE IS SMALL.**

- **`L = 9` T4 is AFFORDABLE** (40.4 observations per nominal parameter, above
  the Buro `≥ 20` line this matrix cites), **MET** (1.108), and **ADDS** (1.64
  worst / 1.88 mean). `L = 7` T4 is comfortably affordable (1 720 per parameter),
  MET (1.014) and ADDS (1.71 / 2.67).
- **Every coarse rung fails the registered criterion and adds on the nested
  one**, and both are true for the same reason: the enum ALONE has fewer classes
  than stone counts alone, and CONDITIONAL on counts it carries real structure.
- **The margins are the whole story.** 1.014 on the registered criterion at
  `L = 7` and 1.108 at `L = 9` are 1.4 % and 10.8 %. A row that is real and
  small is what this measures.

**WHAT REVISION 5 GOT WRONG AND WHY IT IS RECORDED**: it wrote *"no cell is
simultaneously affordable and supported by both measurements"*, which was false
in the row's FAVOUR-denying direction — a hedge where the evidence supported a
modest positive. Round 4 of Stage E's gate named it: the conclusion *"declines a
good result rather than hiding a bad one"*.

**WHAT THE CODE UNIT SAYS.** Five cells put the enum above the referent there —
`L = 7` T4 and T3 (1.140), `L = 9` T4 (1.62), `L = 11` T4 (2.269), `L = 13` T4
(1.636) — and **exactly one flips the verdict between units**: `L = 13` T4, 0.834
window against 1.636 code. All five are at T4 where the class-count gap is
widest. **None is evidence in either direction.**

**WHAT NONE OF THIS TRANSFERS TO**: `R-A5-TOPK`, `R-A1-L7F`, `R-A1-L8F`,
`R-A2-L11F` and `R-A3-L11F+F7`, which are not quotients of this tuple.

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

**THREE READINGS, AND THE THIRD IS THE ONE REVISION 2 GOT BACKWARDS.**

1. A single-seed validation number on this corpus is worth **±3.5 %**, and the
   arithmetic is `half-range / mean`: 2.79 % at `K = 8`, **3.46 %** at `K = 32`
   and **3.46 %** at `K = 64`. *(The tighter statistics, since the half-range is
   the loosest of them: `sd/mean` is 2.00 / 2.34 / 2.25 %, and the standard error
   of the mean of eight is 0.71 / 0.83 / 0.80 %. ±3.5 % is the spread a SINGLE
   split can land anywhere in, which is what the cells that quote it are
   about.)*
2. **Individual weights move by up to 65 % of the table's scale between splits**
   at 33 parameters and above. A fitted table is not a stable object here, and
   this reading is untouched.
3. **THE DESIGN IS PAIRED AND THE COMPARISON MUST BE TOO.**
   `tools/hex_enum/seed_pilot.py` shuffles the game list with
   `random.Random(1000 + seed)`, keyed on the seed and nothing else, with the
   game list fixed before the `K` loop — so **split `s` is the identical
   train/validation partition at every `K` by construction**, and the 6.9 %
   spread is COMMON MODE that cancels in a comparison. Revision 2 set a
   between-condition mean gain (5.6 %) against a within-condition half-range
   (6.9 %) and concluded the architectures were not separable. **MEASURED,
   paired, over the pilot's own receipted rows:**

   | comparison | **the LARGER K wins on** | paired mean gain | sd | `t` |
   |---|---|---|---|---|
   | K = 8 vs K = 32 | **8 of 8 splits** | 23 472 | 5 168 | **12.85** |
   | K = 8 vs K = 64 | **8 of 8 splits** | 36 879 | 7 021 | **14.86** |
   | K = 32 vs K = 64 | **8 of 8 splits** | 13 406 | 3 146 | **12.05** |

   **Offline loss orders these three SIZES cleanly** — one-sided sign-test
   `p = 0.5^8 = 0.0039` on each (two-sided 0.0078; the direction was not
   pre-registered), with `mean/sd = 5.25` for the widest, which is an EFFECT
   SIZE and not a significance level. So the claim that an architecture
   comparison by offline loss drowns in split noise is FALSE on this
   instrument's own data and is withdrawn, and with it revision 2's use of it
   against R-A2's kill condition. **What remains true is reading 2**: the loss
   separates, the individual weights do not.

   **AND THE STEP FROM THIS TO R-A2's KILL IS ONE THE PILOT DID NOT TAKE.** It
   measured three SIZES of one architecture; `eval_families` §7's kill for R-A2
   is *"it does not beat **A3's** validation loss"*, which is two
   ARCHITECTURES. The instrument's resolution — a 2.1 % difference detectable at
   `t = 12` — makes the step reasonable, and it is still a step, marked as one
   here rather than carried under the word "measures".

**AND D-614 IS NOT CORROBORATED BY THIS**, which revision 2 also claimed:
D-614 rules that offline metrics never gate STRENGTH, and the pilot measures that
offline loss does order table SIZES. The two are compatible and neither supports
the other; Rapfi Table 2 is the evidence that loss ranks architectures wrongly for
strength, and it stands alone.

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
stated unit.**

**AND THE DENSITY COLUMN IS FOUR CURRENCIES, WHICH IS NAMED HERE RATHER THAN
LEFT FOR A READER TO TRIP OVER.** *"Observations per parameter"* below means one
of four things depending on the row, and they are not interchangeable:

| currency | which rows | what one observation is |
|---|---|---|
| **positions per parameter** | R-A1-L11, R-A2 | one of the 45 271 quiet positions |
| **window observations per parameter** | R-A5-TOPK, R-A1-L8F, R-A3 | one `(axis, start)` window holding a stone — 16 613 729 of them at `L = 11` |
| **parameters per position** — the RECIPROCAL, and it is here because a source this matrix quotes prints it under the other name | see the correction below | — |
| **scored-cell observations per parameter** | R-A4-CLASS | one empty cell whose neighbourhood holds a stone — 8 174 025 at `L = 11` |
| **games per parameter** | the outcome-blended term only | one of the 3 487 games (`training_pipeline_2026-09.md` §1) |

**AND ONE CORRECTION THE RED TEAM FOUND THAT REACHES PAST THIS MATRIX.**
`eval_families_2026-09.md` §A1 prints *"62 370 parameters against 45 271
positions, **1.4 observations per parameter**"*. **1.378 is `62 370 / 45 271`,
which is PARAMETERS PER POSITION — the reciprocal.** Positions per parameter is
**0.73**, so R-A1-L11 is worse than its own source says, and this matrix
inherited the mislabel into a table written to fix exactly that class of error.
R-A3's cell said *"1.2 at §0.2's unit"* where §0.2's unit is window observations,
in which A3's density is `16 613 729 / 38 983 = 426`; both cells are corrected to
their own currency.

**So `R-A1-L11`'s disqualifying density and `R-A5-TOPK`'s comfortable one are not
the same measurement**, and a comparison across the column is not a comparison.
Each cell below names its own currency; the ranking in §6 does not rest on any
cross-currency comparison, and where two rows are separated it is stated on what. The unit matters and §5 says why: `R-A4-CLASS` is a per-CELL row
by construction (`C(k+2,3)` is a size-3 multiset over the three axes at one
cell), and `R-A1`, `R-A2`, `R-A3`, `R-A5` are per-WINDOW rows in the shape
`eval_families` states them, which is the shape `handcrafted_v0` already sums in.

### R-A4-CLASS — windows quotiented through the Stage E enum, scored on the 3-axis multiset. **LIVE, at `L = 9` or `L = 7`, rung T4**

| column | value |
|---|---|
| **parameters** (DERIVED) | **The live cells are T4 at `L = 7` (2 925) and `L = 9` (161 700)** — §2.5. For completeness: at `L = 11`, T4 7 647 059, T3 302 621, T2 18 424, T1 8 436; at `L = 7`, T2 680. |
| **observations per parameter** (MEASURED, per scored cell) | T4 **1.07 per nominal**, 75.6 per observed code, **median 3**; T2 **444 per nominal**, 5 332 per observed code, **median 41**; T1 969 per nominal, median 53. Population 8 174 025 scored cells over 45 271 positions (180.6 per position). |
| **window touches / stone, and the ratio to 18** (DERIVED) | at the live lengths, `3L` = **21** (`L = 7`, 1.17x) and **27** (`L = 9`, 1.50x) — the two cheapest codebook figures in the field after `R-A1-L7F`'s 21. `L = 11` is 33 (1.83x) and `L = 13` is 39 (2.17x) for one extra class and one extra code. |
| **nodes/sec floor, PRE-REGISTERED as a bench bracket** (ESTIMATED) | Arithmetic on the page: today **529 255** nps (§1.2, receipted) with the eval at **44.70 %** of the profile (§1.8); a codebook at 1.83x the traffic and a per-touch cost ratio `c ∈ [0.5, 1.5]` gives `nps ∈ [296 959, 549 733]` = **[0.56x, 1.04x]**. **FLOOR REGISTERED AT 296 959 nps (0.56x)**; below it the row aborts on cost, per hard rule 5's abort-threshold discipline. *(Revision 2 registered 341 617 from the unreceipted run and the understated share.)* |
| **determinism and quantization** | Closed-form fit, no seed. Integer after bake; one class lookup plus one code lookup per touch. The accumulator is bounded by construction: `3L` active windows times the clamped maximum, `training_pipeline_2026-09.md` §4's *"no combination of possible active features can exceed the maximum value"*. `EVAL_MAX = 16 000` with an i64 intermediate is the shape to keep. |
| **seed budget** (MEASURED, §3) | **1 fit, ≥ 8 splits** for any validation number; ±3.5 % on a single split. No training seed. |
| **books, BOTH arms** | node-matched Δ = 10: **8 000 pairs on `book_v3`, power 0.9045, the whole book**. Time-matched: **not runnable** (§1.4) — no arena movetime, no named play seat, no play-seat pentanomial. |
| **kill condition — REGISTERED, RUN, AND NOT FIRED AT T4** | The stone-count criterion is the row's kill; it fires at every coarse rung and does not fire at T4, where the corrected nested test also says the enum adds. **The rung is therefore not a free choice: T4 at `L = 7` or `L = 9`, and nowhere else.** *(An earlier revision cited "§2.5 measures a value-free quotient passes it at 1.77x"; that 1.77x is `hex_threat_enum_v1.md` §6.5's figure against the WITHDRAWN random-permutation criterion, and §2.5 contains no such number. The cross-reference is corrected.)* The row's own §7.1 arithmetic separately kills **T4 and T3 at `L = 11`** on density (1.07 and 27 observations per nominal parameter), which leaves T2 and T1 — and those are the rungs the criterion was the only positive evidence for. |
| **strongest known attack, and it landed** | **The `t = 1` / `t = 2` merge** (enum memo §5.5): classes contain patterns whose exact `DEF-T` after a stone at the cell differs, and `RULE-EXACT` (`threat_calculus_v1.md:64-66`) makes `t` the truth. **The count depends on the population and the memo now prints both**: over patterns where a stone at the cell does NOT complete a six (rule 4 ends the turn, so no threat question survives), **27 of the 335 classes present and 5 348 of 57 996 patterns**; without that exclusion, **32 of 357 and 6 099 of 59 049**. The memo's earlier reason for not repairing it — *"not something a single-axis class can hold"* — is **false**, and the price IS quoted: adding single-axis `t` as a seventh component removes the merge entirely at **`k` 357 → 384** under the rule-4-excluded reading (the confirming reviewer's 393 is the same computation without that exclusion, and the two differ by exactly the completing patterns). At the rung a matrix would use, T2, the repair is `k` 47 → 58 and **+86 % parameters**. |

### R-A5-TOPK — top-K folded codes plus one rare bucket

| column | value |
|---|---|
| **parameters** (DERIVED) | **`K + 1`, and `K` DEPENDS ON THE UNIT** — see the attack. At `eval_families` §0.2's `(axis, start)` window unit, **811** (810 folded L11 codes cover 90 %). At the cell-centred window unit this package measures, **143** (142 codes cover 90 % of 12 980 519 observations). |
| **observations per parameter** (MEASURED) | at §0.2's unit, 16 613 729 window observations over 38 983 codes; at the cell-centred unit, 12 980 519 over 11 909 codes, median 7, **4 730 codes at or below Buro's ≤ 4 line**. The top-`K` head is dense either way; the rare bucket absorbs the tail. |
| **window touches / stone** (DERIVED) | `3L` = **33**, **1.83x**. |
| **nodes/sec floor** (ESTIMATED) | same traffic as A4, **[296 959, 549 733]**, floor **296 959 (0.56x)**. Cheaper per touch than A4 in principle (one table lookup, no class indirection), so the bracket is the same and the row should land nearer its top. |
| **determinism and quantization** | closed form, no seed; trivial determinism; low quantization risk. |
| **seed budget** (MEASURED, §3) | **this row IS the pilot's shape**: 1 fit, ≥ 8 splits, ±3.5 % on one. At `K = 64` the worst weight moved **64.92 %** of the table scale between splits. |
| **books** | as A4: one node-matched arm, whole book. |
| **kill condition** | `eval_families` §A5: *"the rare tail is where novel structure lives and one bucket scores all of it alike"* — unmeasured, and this matrix does not measure it either. |
| **strongest known attack** | **The row's headline parameter count is a different number at each unit, and `eval_families` §7 states only one of them.** 811 is §0.2's window census; 143 is the same 90 % rule at the unit `R-A4-CLASS` must use. A field that prices A4 per cell and A5 per window is pricing two rows on two instruments (D-477's own class), and the row is not well posed until the architect names the evaluator's summand. |

### R-A1-L7F — free folded table at length 7, window-indexed. **THE SECOND ROW THE FIELD WAS MISSING**

The third red team found it, and it was quoted on this matrix's own page before
it was priced: `R-A3`'s factor cell calls length-7 folded *"the natural dense
factor"* and prices nothing on it as a row.

| column | value |
|---|---|
| **parameters** (MEASURED, `eval_families` §0.2) | **1 029** folded L7 cells, 90.8 % of the 1 133 ceiling |
| **observations per parameter** (MEASURED, window observations) | **median 1 282**, and **NOT ONE CELL under ten observations** — the only free table in the field of which that is true. 190 cells cover 90 %. |
| **window touches / stone** (DERIVED) | `3L` = **21**, **1.17x** — the cheapest codebook traffic in the field |
| **nodes/sec floor** (ESTIMATED, §1.8's arithmetic) | `nps ∈ [396 371, 650 390]` = [0.75x, 1.23x]. **FLOOR 396 371 (0.75x)** — the highest floor of any codebook row, hence the least throughput risk |
| **determinism and quantization** | closed form, no seed, trivial determinism, low risk |
| **seed budget** (MEASURED, §3) | 1 fit, ≥ 8 splits |
| **books** | one node-matched arm, as every learned row |
| **kill condition** | the same as R-A1-L8F's and it bites harder: at `L = 7` the code sees `c ± 3` where `eval_families` §0.1 derives `L ≥ 11` as the length at which one centred window contains every 6-window through a cell. `THM-WINDOW` (`threat_calculus_v1.md:152-155`) says a four needs 8 cells of context and a five needs 7. |
| **strongest known attack — against its own kill** | the same unrun enumeration: `eval_families` §8 still lists it as *"Minutes of compute, **not run** (D-291)"*. **AND ONE MEASUREMENT THIS PACKAGE DID TAKE CUTS AGAINST THE ROW**: `hex_threat_enum_v1.md` §7.4 measures that at `L = 7` a quotient knowing only stone counts explains as much as the threat tuple does, which is evidence — on one tuple, one statistic, one unit — that seven cells is where structure stops being visible on this corpus. |

### R-A1-L8F — free folded table at length 8, window-indexed. **THE ROW REVISION 2's FIELD WAS MISSING**

The red team found it and it is added rather than argued about: `eval_families`
§0.2 measures a length-8 folded window census this matrix quoted in §1.4 and
then priced no row against.

| column | value |
|---|---|
| **parameters** (MEASURED, `eval_families` §0.2) | **2 920** folded L8 cells, 88.0 % of the `(3^8 + 3^4)/2 − 1 = 3 320` ceiling |
| **observations per parameter** (MEASURED) | **median 273**, only **24 cells (0.8 %) under ten observations**, 293 cells covering 90 %. Denser than A1-L11 and A3 by two orders of magnitude — **and NOT the densest free table in the field**, which is `R-A1-L7F` above at median 1 282 with no cell under ten. |
| **window touches / stone** (DERIVED) | `3L` = **24**, **1.33x** — the cheapest codebook traffic in the field |
| **nodes/sec floor** (ESTIMATED, §1.8's arithmetic) | `nps ∈ [365 760, 621 921]` = [0.69x, 1.18x]. **FLOOR 365 760 (0.69x)** |
| **determinism and quantization** | closed form, no seed, trivial determinism, low risk |
| **seed budget** (MEASURED, §3) | 1 fit, ≥ 8 splits |
| **books** | one node-matched arm, as every learned row |
| **kill condition — and it is the one that keeps it below A5** | **It is below the covering minimum.** `eval_families` §0.1 derives `L ≥ 2·WIN_LEN − 1 = 11` as the length at which one centred window contains every 6-window through a cell; at `L = 8` the code is a partial view of the structure it scores. |
| **strongest known attack — against its own kill** | **The covering bound is a DERIVATION, not a measurement, and the measurement it asks for has never been taken.** `threat_calculus_v1.md:152-155` (THM-WINDOW) says *"re-derive the minimal sufficient hex length by enumeration"*, and `eval_families` §8 lists that enumeration as *"Minutes of compute, **not run** (D-291)"* — still true at `54eb3ba`. Stage E answered a CELL-CENTRED version of the question and its §6.6 states in terms that it does not close THM-WINDOW. **So the boundary that excludes the densest affordable free table in this field is an unrun theorem**, and that is a finding against the field's completeness rather than against this row. |

### R-A2-L11F — mapped codebook, shared-weight generator, length 11, folded

| column | value |
|---|---|
| **parameters** (DERIVED from the source's ratio, EMP for the sizes) | Rapfi's smallest configuration is **14 160 generator parameters producing a 397 488-entry codebook** (`eval_families` §A2, Table 1). The hex analogue's codebook is the folded L11 code space, **38 983 observed cells** at §0.2's unit; the generator's size is a design choice Phase 2b makes. |
| **observations per parameter** (DERIVED) | against 14 160 generator parameters and 45 271 positions, **3.2 positions per parameter** — and `training_pipeline_2026-09.md` §6 reports two published sizing rules that disagree by 100x (nodchip's ≥ 10 positions per parameter, which caps this at ~4 500; ChessBench's measured overfit at 0.08), so **the rules do not settle it and both are reported**. |
| **window touches / stone** (DERIVED) | **33**, 1.83x — the bake makes inference one lookup per window, as A5. |
| **nodes/sec floor** (ESTIMATED) | **[296 959, 549 733]**, floor **296 959 (0.56x)** — identical to A4/A5 at inference, because the generator is offline. |
| **determinism and quantization** | **The training run is NOT reproducible** — `training_pipeline_2026-09.md` §3 quotes PyTorch's own page, *"Completely reproducible results are not guaranteed across PyTorch releases, individual commits, or different platforms"* — so **the reproducible artifact is the weight file by digest, never the run**. The engine stays deterministic after the bake (int16 features, int32 accumulation, `eval_families` §A7). **AND THE TRAINER IS A PACKAGE BEFORE IT IS A ROW**: nothing in this repository trains anything, and `tools/texel/` is a closed-form solver. *(An earlier revision said this workstation's `python3` has no `numpy`; that is false — `/usr/bin/python3` has numpy 2.5.2 and only the `mise`-managed interpreter on PATH does not. The claim is deleted rather than repaired, because which interpreter is on PATH is not a property of the row.)* |
| **seed budget** (MEASURED + ARCH) | **≥ 4 trained nets** by the reference trainer's own standard, on top of the ≥ 8 splits §3 measures. **The books fund ONE acceptance run**, so four nets cannot each be accepted and the row must choose one by an offline metric — which Rapfi Table 2 measures ranks nets WRONGLY for strength (*"300-400 ELO… at equal wall clock"* against the loss ordering). |
| **books** | one node-matched arm, whole book; the seed budget makes that a selection problem the other rows do not have. |
| **kill condition** | `eval_families` §7: *"it does not beat A3's validation loss, or the nps floor is missed"*. **Both halves can fire**: §3's paired analysis measures that validation loss separates table sizes at `t = 12` to `15`, so the first half is a live test, and the floor below is the second. |
| **strongest known attack** | **Three games agree that the alpha-beta eval-width optimum sits BELOW the accuracy optimum** (`eval_families` §A7): Rapfi's Mixnet Large scores **−82 / −58 / −45 / −38 Elo** against Small under α-β while scoring **+57 to +84** under MCTS; Logistello's three large patterns cost ~45 % speed and *"could not compensate"*; figrid's 1 024-accumulator net lost **23 points** to the 512 one at ~1.1 M samples. A2 is the row that buys accuracy with throughput, under the paradigm where that trade is measured to lose. |

### R-A3-L11F+F7 — free length-11 folded table with the length-7 folded factor

| column | value |
|---|---|
| **parameters** (DERIVED) | real parameters are A1's: **38 983** folded L11 codes at §0.2's unit. The factor — 1 029 folded L7 cells, none under ten observations — is a **training-time device with zero inference cost**, coalesced into the real weights (`eval_families` §A3). |
| **observations per parameter** (MEASURED) | **426 window observations per parameter** (`16 613 729 / 38 983`), or **1.16 POSITIONS per parameter** — `eval_families` §A1's kill figure for A1 is the second of those, and the factor adds no observations either way. The factor's own density is 1 282 median. |
| **window touches / stone** (DERIVED) | **33**, 1.83x. |
| **nodes/sec floor** (ESTIMATED) | **[296 959, 549 733]**, floor **296 959 (0.56x)** — identical to A1 by construction, which is the row's selling point. |
| **determinism and quantization** | closed form if the virtual features are fitted by least squares; trivial determinism; risk as A1. |
| **seed budget** (MEASURED, §3) | 1 fit, ≥ 8 splits. |
| **books** | one node-matched arm, whole book. |
| **kill condition** | Buro's `≤ 4`-match rule zeroing most of the table, and Stockfish's own caveat. |
| **strongest known attack** | **The remedy's source disowns it at this scale.** `eval_families` §A3 quotes Stockfish verbatim — *"just adding more factors… may even cause it to regress"*, and it *"seems to only be relevant in the early stages"* — said against **16 billion** positions where this corpus has 89 805. *"The early stage" is where a Phase-2 fit permanently lives*, which is the document's own sentence, and it is an argument for the row and an argument that its evidence base is 200 000x away. |

### R-A1-L11 — free table at length 11, unfolded. **KILLED ON THE RECORD, shown so the matrix scores it**

| column | value |
|---|---|
| **parameters** | **62 370** unfolded L11 cells (`wp22_phase2_premise.md` §3); 38 983 folded. |
| **observations per parameter** | **0.73 positions per parameter** unfolded (`45 271 / 62 370`) and 1.16 folded. *(`eval_families` §A1 prints "1.4 observations per parameter"; 1.378 is the RECIPROCAL — parameters per position. The row is worse than its source states.)* Against Logistello's **7.3** and Buro's *"the weight is set to 0"* below ~4 matches. |
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
| **books — AND THIS IS THE KILL** | fishtest's live data, **EMP**, over 46 sampled runs: a **median of 120 000 games for 4 parameters**. 120 000 games is 60 000 pairs is **`ceil_to_500(60 000 + 500) = 60 500` openings — 7.1x the whole of `book_v3`** — and `book_v3` may not be used for it at all: D-644 makes it *"acceptance-only (no labels, no tuning, no corpus input)"*. D-635's defect class applies on top: tuning and acceptance books must be disjoint or the acceptance run is a self-match. **At the instrument seat's 2.046 s per opening that is 34.4 h of games before any acceptance run.** |
| **kill condition** | fired: no committed book funds it and the one book that could is barred by ADR. |
| **strongest known attack — against the kill** | the 120 000 figure is a chess median and **EMP, NON-TRANSFERABLE**; a dozen integers on a smaller board might converge far sooner, and nothing here measures that. The kill is a cost kill on the only evidence available, not a proof of infeasibility. |

### R-H-EXT — extended handcrafted: the null-plus, and the row the learned rows must beat on cost

| column | value |
|---|---|
| **parameters** | v0's five, plus the terms the calculus already names as candidates: a **tempo census** per side (`ADOPT-TEMPO`, `threat_calculus_v1.md:177`, *"tempo census … eval-term candidate"*, audited exact at §6), **exact-`t` counters as eval terms** — `THM-WINDOW`'s own cheapest listed fix, `:153`, *"exact-t counters as eval terms (free from\n  PROTO-NODE)"* spanning `:153-154` — and `E-INIT`'s initiative discount (`:158`). **~3 to 6 more integers.** |
| **observations per parameter** | not applicable: the terms are hand-set and SPRT-gated, which is what the calculus says they are. |
| **window touches / stone** (DERIVED) | **18, 1.00x**. The exact-`t` counters are computed by `PROTO-NODE` already; `ThreatState::touch` is 13.45 % of the profile whether or not a node reads a threat query (`repo_audit_2026-09.md` A-03), so the counters are close to free. |
| **nodes/sec floor** (ESTIMATED) | traffic unchanged at 18 touches, arithmetic per touch up: **`c ∈ [1.0, 2.0]`** — the row keeps `handcrafted_v0`'s per-touch work and adds terms, so `c ≥ 1` by construction, which is the bracket's ground and the reason it differs from the codebook rows' `[0.5, 1.5]` (a table lookup can be cheaper per touch than the arithmetic it replaces, and adding hand terms cannot). That gives `nps ∈ [365 760, 529 255]` = **[0.69x, 1.00x]**, and **THE FLOOR THAT FALLS OUT OF THAT BRACKET CANNOT FIRE**: the same cell says the exact-`t` counters are close to free and the honest expectation is ≈ 1.00x, so an abort threshold at 0.69x is one nothing can trip, which is the vacuous check hard rule 5's abort discipline exists to forbid and which §8 credits this document with naming elsewhere. **Floor registered at 0.95x = 502 792 nps** instead, which is where a row expecting parity actually aborts. The `[1.0, 2.0]` bracket stays as the honest uncertainty on `c`; the FLOOR is a decision about when to stop, and it is set where it can fire. |
| **determinism and quantization** | unchanged from v0 — integer, no fit, no artifact, no digest problem, no shape check owed. |
| **seed budget** | **zero** — there is nothing to seed. **AND THAT IS NOT THE SAME AS A ZERO BUILD COST, which revision 3 elided.** The row is 3 to 6 integers and **no procedure and no metric chooses their VALUES**: `handcrafted_v0`'s own five came from Stage 0 and the calculus's §6 tempo table gives τ per pre-emp but no scale against `w1..w5`. Against one funded acceptance run, a row that must pick a point in a 3-to-6-dimensional integer space with nothing to pick it by has the same structure as R-A2's ≥ 4 nets against one arm — which is the ground this matrix ranks R-A2 LAST on. **The difference is one of degree and is stated rather than hidden**: R-A2's four nets each cost a training run and a digest, where these integers cost a text edit, and the calculus's τ table plus the dominance constraints (D-622's *"each tactical class exceeds the sum of everything below it"*) bound the space where nothing bounds a net's. It is a real cost and it is not nil. |
| **books — AND IT IS NOT FREE** | Either a `book_v3` arm at Δ = 10 like every other row, **or** a large-effect screen on `book_v2`'s holdout, which §1.6 measures reaches D-653's floor at 1 000 pairs / Δ = 30 (0.9189) and at 500 pairs / Δ = 40 (0.9067). **The screen consumes an irreplaceable slice**: the holdout is 1 000 openings reserved by D-568, never labelled, the only corpus-disjoint `book_v2` sample, and D-628 records two packages with standing claims on it. **Spending it is the architect's ruling and not this matrix's**, and revision 2's *"the only row that can use the screening book"* is deleted — it was asserted, and nothing here measures that another row could not. |
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

D-705, whole: *"a Stage 2 eval row registers a node-matched arm at the
instrument seat and a time-matched arm at the play seat; acceptance =
node-matched h1 AND time-matched not h0; each row pre-registers a nodes/sec
floor for the bench bracket — flips if the play seat is redefined."*

| row | node-matched arm | time-matched arm | nodes/sec floor registered |
|---|---|---|---|
| R-A4-CLASS | Δ = 10, 8 000 pairs, `book_v3` whole — as every learned row | not runnable | **396 371** (0.75x) at `L = 7`, **339 538** (0.64x) at `L = 9`; the `L = 11` rungs are out on density |
| R-A5-TOPK | as above | not runnable | **296 959** (0.56x) |
| R-A1-L7F | as above | not runnable | **396 371** (0.75x) |
| R-A1-L8F | as above | not runnable | **365 760** (0.69x) |
| R-A2-L11F | as above, and it needs **≥ 4 nets** for one arm (§3) | not runnable | **296 959** (0.56x) |
| R-A3-L11F+F7 | as above | not runnable | **296 959** (0.56x) |
| R-A1-L11 | killed before the arm | — | — (a killed row registers no floor) |
| R-C-SPSA | needs 60 500 openings of tuning FIRST (§4) | not runnable | no change registered: the row alters no per-touch work |
| R-H-EXT | Δ = 10 on `book_v3`, **or** a screen on `book_v2`'s holdout at Δ = 30 / 1 000 pairs (0.9189) or Δ = 40 / 500 pairs (0.9067) — see its books cell for what that spends | not runnable | **502 792** (0.95x) |
| R-D-W1 | killed by ruling | — | — (a killed row registers no floor) |

**AND ONE DISTINCTION REVISION 2 BLURRED, BECAUSE §6 THEN RANKS ON ONE HALF OF
IT.** *Window traffic and measured nps* are BENCH quantities — hard rule 5's, and
this session measured them today without an arena (§1.2, §1.3). *A time-matched
ARM* is an SPRT at a wall-clock budget, which §1.4 measures the arena refuses.
**Ranking on the first is legitimate and ranking on the second is not**, and §6
ranks only on the first. The two were run together in revision 2's sentence and
are separated here.

**THE COLUMN THAT WAS MEANT TO SEPARATE THE ROWS IS CONSTANT.** The
time-matched arm is unavailable for every row for the same three reasons — the
arena refuses a movetime budget by name, no play seat is chosen, no play-seat
pentanomial exists — and none of them is a property of any row. **So the two-arm
rule cannot rank this field**, and a matrix that presented it as a per-row cost
would be presenting a package obligation as a discriminator. It is stated once,
in §1.4, and its discharge is one of the two questions the HANDUP puts to the
architect.

**AND §1.6's HEADLINE IS D-705's OWN PREMISE, MEASURED — NOT A GAP IN IT.**
Revision 2 wrote *"one thing D-705 does not say"* and D-705 says it, verbatim:
*"AND THE ASYMMETRY IS DELIBERATE: two `h1` bars would double the openings a row
costs, and D-568's reservation plus `book_v3`'s 8 500 cannot fund that for every
row."* That is the ADR's stated reason the veto's bar is *"not h0"* rather than
*"h1"*, and §1.6 confirms it with numbers the ADR did not have. **The false
finding is deleted; the confirmation is what stands.**

**AND §1.6's SPLIT ARITHMETIC PRICES A DESIGN D-705 REJECTS.** *"Splitting
`book_v3` between two arms drops each to 0.6970"* models two `h1` arms at
Δ = 10 — which is exactly the doubling D-705 declines. **The question D-705
actually poses is unasked here and is owed by whoever runs the arm**: what does a
*"not h0"* veto cost in openings, and what is its power to FIRE? §1.6 has half of
it — at a 500-pair cap and truth = 0 the test returns `h0` in 0.7385 of runs and
at 1 000 pairs in 0.9172 — and the other half needs a play-seat pentanomial that
does not exist (§1.4).

**AND D-705's LAST CLAUSE BINDS THIS TABLE**: *"a row without one is not testing
what decides the outcome"*, said of the nodes/sec floor. Every live row below
registers one; the killed and stopped rows do not, and that is stated in their
cells rather than left blank.

## §6 THE RECOMMENDED ORDER — AND IT IS AN ORDER, NOT A SELECTION (D-708)

The books fund one acceptance run at Δ = 10 (§1.6, and D-705 says so itself). So
the field's real question is not "which row is best" but "which row is run
first", and the ranking below is by **what a row costs to BUILD before any book
is spent** — with §2's densities and §3's paired losses breaking ties.

**AND THE AXIS DOES NOT ANSWER THE QUESTION IT NAMES, WHICH IS SAID HERE RATHER
THAN LEFT FOR A READER.** With one funded acceptance run (§1.6, and `book_v3`'s
ledger already names R7 as its claimant), *"which row is run first"* is *"which
row is run"*. An ordering by BUILD COST answers "which is cheapest to get to the
start line" and does not answer "which is likeliest to be worth the one run" —
that second question needs a prior about Elo that nothing in this package
measures and D-614 forbids substituting an offline metric for. **The order below
is therefore an order of readiness, not of expected value**, and the architect's
first question in the HANDUP is exactly the one this axis cannot answer.

**THE AXIS IS NARROWER THAN REVISION 2's AND THE NARROWING IS THE POINT.**
Revision 2 ranked by "cost before it can be run at all" and then priced
`R-H-EXT`'s book cost at nothing, which was false (§4). **Book cost is now
EQUAL across every live row** — one `book_v3` arm — so it cannot rank them, and
what remains is build cost, on which the rows genuinely differ.

1. **R-H-EXT.** No fit, no artifact, no digest discipline, no shape check, no
   seed, no quantization, no new dependency, and 18 touches per stone against
   every codebook row's 21 to 33. **Its build cost is NOT nil and §4 says so**:
   3 to 6 integers with no procedure and no metric to choose their values, which
   is R-A2's structure at a smaller scale — bounded by the calculus's τ table and
   D-622's dominance constraints where a net's weights are bounded by nothing,
   but a real cost against one funded run. Running it first also moves D-614's
   bar, which is *"vs handcrafted_v0"*, so every learned row would otherwise be
   measured against an incumbent nobody has tried to improve.

2. **R-A4-CLASS at `L = 9` or `L = 7`, rung T4 — and it MOVES UP on the
   corrected measurement.** Revision 5 ranked it fourth on a nested figure of
   0.93 that was an artifact of an unmatched null; corrected, it is **1.71**.
   At `L = 9` T4 the row is affordable (40.4 observations per nominal
   parameter), MET on the registered criterion (1.108) and adding on the nested
   one (1.64 worst / 1.88 mean). **It is the only row in the field carrying a
   criterion that was registered before its run, could fail, was run, and did
   not fire.** Traffic 21 or 27 touches; floor 396 371 or 339 538. It beats R-A3
   by **13.3x** in the one currency both share — positions per parameter, 15.5
   against 1.16. *(An earlier revision claimed "three orders of magnitude" by
   setting scored-cell observations against window observations, the
   cross-currency comparison §4 promises never to make.)* **What keeps it out of
   rank 1 among the learned rows is the SIZE of what it buys**: 1.4 % at `L = 7`
   and 10.8 % at `L = 9` over a quotient that only counts stones.

3. **R-A5-TOPK.** A closed-form fit, no training seed, no dependency this
   repository lacks. **Its parameter count is 811 or 143 depending on the
   evaluator's summand** (§4) — the architect's question, and it does not stop
   the row being run, because both are affordable against 12 980 519 window
   observations. It ranks below R-A4 because R-A4's criterion has been run and
   R-A5's kill — that one bucket scores all novel structure alike — has not been
   measured by anything.

4. **R-A1-L7F.** The densest free table in the field: 1 029 parameters at median
   1 282 observations with **not one cell under ten**, and the cheapest traffic
   of any codebook row (`3L = 21`, floor 396 371). It ranks here and not higher
   because it is furthest below `eval_families` §0.1's covering minimum, **and
   because §2.5 measures that at `L = 7` a quotient knowing only stone counts
   explains as much as the threat tuple does** — evidence, on one tuple and one
   statistic, that seven cells is near where structure stops being visible on
   this corpus.

5. **R-A1-L8F.** 2 920 parameters at median 273, 24 cells under ten, traffic 24.
   Below R-A1-L7F on density and above it on covering; both share one kill and
   **the enumeration that would settle it has never been run** — `eval_families`
   §8 still lists it as *"minutes of compute, not run (D-291)"*.

6. **R-A3-L11F+F7.** Zero inference cost over A1 and the documented remedy for
   §0.3, but its source disowns the remedy at 200 000x this corpus's size and its
   real density is 1.16 positions per parameter.

7. **R-A2-L11F.** The strongest published shape and the furthest from runnable
   here: a trainer that does not exist in this repository, a non-reproducible
   training run, **≥ 4 nets against one funded acceptance arm**, and the paradigm
   under which three games measure its trade to lose.

8. **R-A1-L11** — scored and killed (§4). **R-C-SPSA** — killed on cost against
   the ledger. **R-D-W1** — killed by ruling (D-704).

**WHAT WOULD REORDER THIS**: a second acceptance book moves R-A2 up, because its
seed budget stops being a selection problem; running `THM-WINDOW`'s enumeration
(minutes, never taken) settles whether R-A1-L7F's and R-A1-L8F's shared kill is
real; an ADR moving the arena's movetime refusal makes §5's column real; a ruling
that R-H-EXT is out of Phase 2's scope removes rank 1; and a decision on the
evaluator's summand settles R-A5's parameter count.

**NOTHING IS SELECTED HERE** and no ADR of selection is written.

### THE STRONGEST SURVIVING ATTACK ON THIS ORDER, and it is the red team's own

The DECISION-RED-TEAM would rank **R-A5-TOPK first and R-H-EXT second, and only
after the operator rules on the holdout**, on this ground: *"on the matrix's own
axis, corrected, R-H-EXT is the MOST expensive row"* — because the screen that
makes it look cheap consumes the whole of `book_v2`'s 1 000-opening holdout,
which is never labelled, is the only corpus-disjoint v2 sample, and carries two
standing claims.

**It is recorded and not conceded, and the disagreement is one sentence.** The
attack prices R-H-EXT as though the screen were compulsory; it is not — the row
can take a `book_v3` arm at Δ = 10 exactly like every other row, at which point
its book cost is the field's common cost and its build cost is nil. **What the
attack does establish, and this matrix now says in §4, is that the screen is not
free and spending the holdout is the architect's ruling.**

**AND THE SECOND HALF OF THE ATTACK IS NOT ANSWERED AT ALL, so it is carried to
the HANDUP**: R-H-EXT is arguably not a Phase-2 row. Phase 2's field is the
QUIET-STRUCTURE LEARNED family, and a package of hand terms is Stage-0 work
nobody did. Ranking it first may be reading the dispatch's own row list too
literally. **The architect decides that, not this document.**

**WHAT WOULD REORDER THIS**: a second acceptance book moves R-A2 up, because its
seed budget stops being a selection problem; running THM-WINDOW's enumeration
(minutes, never taken) settles whether R-A1-L8F's kill is real and could move it
above R-A5; an ADR moving the arena's movetime refusal makes §5's column real,
at which point the throughput brackets discriminate; and a ruling that R-H-EXT
is out of scope removes rank 1 entirely.

**NOTHING IS SELECTED HERE** and no ADR of selection is written.

## §7 COSTS AND FAILURE MODES, one line each

| row | **BUILD cost** (§6's axis; book cost is the field's common one) | failure mode |
|---|---|---|
| R-H-EXT | no fit, no artifact, no seed — **but 3 to 6 integers with no procedure and no metric to choose their values** (§4) | it loses, and the incumbent stands — which is a finding and not a waste, because the calculus has been calling these terms candidates since v1.0 |
| R-A4-CLASS | a fit and a backend, as any codebook row | its effect is SMALL — 1.4 % at `L = 7` and 10.8 % at `L = 9` over stone counting — and its known defect is the `t` merge: 27 of 335 classes, repairable at T4 for `k` 357 → 384 |
| R-A5-TOPK | a fit and a backend | the rare bucket scores all novel structure alike, and the tail is where novel structure is |
| R-A1-L7F | a fit and a backend | it is furthest below the covering minimum, and §2.5 measures that at `L = 7` stone counts explain as much as threat structure |
| R-A1-L8F | a fit and a backend | it is below the covering minimum — and the enumeration that would settle whether that matters has never been run |
| R-A3-L11F+F7 | a fit with virtual features and a coalescing step | the factor regresses, which its own source warns of, and 1.2 observations per real parameter is the density that killed A1 |
| R-A2-L11F | a trainer, a dependency, ≥ 4 nets, a digest discipline, a shape check | the α-β width optimum sits below the accuracy optimum, measured in three games |
| R-A1-L11 | — | killed: 1.2 observations per parameter, and unfolded it learns twelve answers |
| R-C-SPSA | 60 500 openings of tuning on a book that does not exist | killed on cost; the kill rests on a chess median and is marked EMP |
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
| 1 M-2 | the option's whole diagnostic gain was absorption of a constant the feature set could not express | no row here is scored on a diagnostic at all: `hex_threat_enum_v1.md` §6.5's purity criterion is registered as a constraint on what may be CONCLUDED, and D-614 is quoted at it |
| 1 M-3 | shape and scale were not separable — one decision presented as two | the one decision is which family, and §5 states plainly that the two-arm rule does not separate the rows rather than dressing it as a second axis |
| 1 M-4 | the option set was incomplete in the dimension the decision was about | every row `eval_families` §7 lists is present, killed rows shown with their kill, and the coarsening ladder gives A4 four priced rungs rather than one |
| 1 M-5 | the filter did not implement the predicate it was said to | this matrix defines no filter; it inherits R6's, whose coextension round 2 measured and round 3 reproduced |
| 1 M-6 | integer rounding at the chosen scale was a first-order confound of the ratios that decide moves | no table is fitted here and no rounding happens; the numbers are counts, powers and throughputs |
| 1 M-7 / 2 M-1 / 3 MAJOR-2 | **three consecutive revisions pointed at instruments that carried nothing, then did not exist, then could not run** | **five instruments produce numbers here and THREE OF THEM ARE UNTRACKED** — `measure_nps.sh`, `measure_play.sh` and `confirm_wedge.sh` live only inside gitignored `artifacts/`, and `git ls-files` returns none of them, which matters most for `measure_nps.sh` because every registered nps floor is recomputed from its output. The two that ARE committed under `tools/hex_enum/`, driven by two suites through **CI gate 19**, and each was RUN in this session with its output receipted by digest (§10) |
| 1 M-8 | the operator ruling was quoted with four words it did not contain | every quotation here is reproduced at `54eb3ba` with `/usr/bin/grep`, and D-705 is quoted with its flip clause; **D-653 is NOT quoted whole — its flip clause, *"flips if `book_v3`'s extension makes a holdout redundant"*, is dropped, and §6's *"a second acceptance book moves R-A2 up"* is that very flip fired, unconnected.** The claim that both are quoted whole is withdrawn. D-653 and D-705 — are quoted whole rather than glossed |
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
| `tools/hex_enum/{hexenum,census,report}.py`, CI gate 19 | §2's `k`, codes, observations, growth curves, and §2.5's criterion table | `artifacts/wp22_phase2a/census_r3/`, `9bb8fc6f2e0500bcf10f48e493dd9e3615897d935b73b63d8a8f47bf179e9c39` |
| the same, at round 2, without the stone-count referent | the replication: 200 shared cells, 0 differences | `artifacts/wp22_phase2a/census/`, `aab7f4f6a8d450fac5609dce4635a21ea86ec658e4c42c11c9d89b98835e14a8` |
| four mutants of `report.py`, scratch copy of `tools/`, live tree untouched | that gate 19 can now see a constant `ω²` — all four DEAD | `artifacts/wp22_phase2a/n6_mutants/`, `36b4c999ba780800e40369cff20641db4982b05d3a3497bf84a88e4c40607d2e` |
| the same instrument at 3 null replicates, then at 24 | §2.5's NESTED test, and the referent's own spread | `artifacts/wp22_phase2a/census_r4/`, `aa88c298ac422a87df6505afcc33fcb1e2cb156783e81ec525b13dc4681bbdd1` |
| `tools/receipt_digest_check.py` | **that every digest THIS table prints matches a receipt that verifies** — the class that recurred three times | run, not receipted: it reads `artifacts/` and cannot be a CI gate for the reason `book_v3_ledger.md` gives about its own disjointness script |
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

---

## §12 WHAT THE RED TEAM CHANGED

`matrix_wp22_phase2_eval_REDTEAM.md`, fresh context, at `0641504` — **FAIL**,
4 BLOCKING, 11 MAJOR, 10 MINOR, with its own independent corpus walk, its own
SPRT power simulator, its own purity re-derivation and its own paired
re-analysis of the seed pilot. **Every finding below was re-derived by this
session before it was acted on.** Round 2 of four (D-709); remedies only.

| finding | what it said | what changed, and what was run |
|---|---|---|
| **B-1** | §5 says *"one thing D-705 does not say"* and D-705 says it | the false finding is **DELETED** and §5 now reports §1.6 as the ADR's own premise measured. RUN: `sed -n '1476p' docs/decisions.md`. |
| **B-2** | §3's reading 3 is false — the pilot is a PAIRED design | **DELETED and replaced** by the paired table. RUN: re-derived from the receipted `pilot_L11.txt` — K=64 beats K=32 beats K=8 on **8 of 8 splits**, `t` = 12.05 to 14.86. R-A2's kill condition is restored to a live test. |
| **B-3** | *"D-653's two clauses conflict"* is a misreading | **DELETED.** D-653 writes `elo1 >= 30`; the 500-pair cap came from inverting a book-SIZING rule, and 1 000 openings seat 1 000 pairs, where the measured power is **0.9189**. |
| **B-4** | the matrix is stale against Stage E round 3 | §2, §2.5 and §6 re-pointed at `hex_threat_enum_v1.md` §7.4, where the replaced criterion **fires at 12 of 16 cells**. |
| **M-1** | the headline throughputs are not in the receipt they cite | **§1.2 is fixed** — it prints the RECEIPTED run (**529 255**) with the other execution as the replicate, and every floor is recomputed from it. **§1.3 is NOT**: its six play-seat digits are still the first execution's, where the receipt holds `171 851 / 171 625 / 172 499` and `482 274 / 481 548 / 482 127`. They are labelled as unreceipted and nothing is computed from them — the receipted 2.8x ratio and the 1.21 / 3.42 depths are what §1.3 uses — but the row's earlier claim that both were fixed was false and is corrected here. |
| **M-2** | §1.6's `sprt_power` digest matches nothing | corrected to `d82fce37…`, the digest after `screen_power.txt` joined the directory. |
| **M-3** | §1.8 understates the eval's share | corrected to **44.70 %** (`delta` + `undo` + `apply`, all `HandcraftedV0`). Every registered floor recomputed: the codebook floor **341 617 → 296 959**, R-H-EXT's **403 391 → 365 760**. |
| **M-4** | the density column is four currencies | §4 opens with the currency table — **and revision 3's table misnamed two of its own rows**, putting R-A1-L11 and R-A3 under "positions per parameter" while their cells printed a reciprocal and a window figure. Both cells and the table are corrected, and the `eval_families` §A1 mislabel they came from is named. |
| **M-5** | §5 declares the discriminating column dead, then ranks on it | §5 separates BENCH quantities (measured today) from the time-matched ARM (not runnable); §6 ranks only on the first. |
| **M-6, M-7** | R-H-EXT's cost is priced at "none", and *"the only row that can use the screening book"* is asserted | both **DELETED**. Its books cell now states what the screen spends and that spending it is the architect's ruling. |
| **M-8** | *"this workstation's `python3` has no `numpy`"* is false | **DELETED**; `/usr/bin/python3` has numpy 2.5.2. |
| **M-9** | the transferable "4 %" is an L=7 3 000-position subsample | replaced by the full-corpus margins per length (+1.4 %, +10.8 %, +12.5 %, **−16.6 %**), and the generalisation drawn from it is deleted. |
| **M-10** | *"MET at all sixteen cells"* is one of two units and one of three referents | §2 item 4 names the referent, the unit, and what clearing a floor does and does not say. |
| **M-11** | the field is missing a row | **R-A1-L8F is added**, priced, and ranked — with the finding that the boundary excluding it is an enumeration `eval_families` §8 marks *"not run"*. |

**THE ONE THING THE RED TEAM WOULD STILL DO DIFFERENTLY** is §6's rank 1, and it
is recorded in §6 as the strongest surviving attack rather than settled here.

---

## §13 WHAT THE SECOND RED TEAM CHANGED

`matrix_wp22_phase2_eval_REDTEAM_round2.md`, fresh context, at `564dc9e` —
**FAIL**, 1 new BLOCKING, 7 new MAJOR, 5 new MINOR, **and 10 of 10 of round 1's
MINORs still landing, 8 of them untouched.** Its confirmation closed all four of
round 1's BLOCKINGs at their own claim, eight MAJORs, and verified `R-A1-L8F` as
*"legitimate and correctly priced"*. Round 3 of four (D-709); remedies only.

| finding | what changed, and what was run |
|---|---|
| **NEW-1** (BLOCKING) — §2.5's conclusion is false on its own receipted census: `L = 7` T4 is affordable AND met | **§2.5 rewritten and `R-A4-CLASS` RESTORED to the field as UNDECIDED, at rank 4.** RUN: the round-4 census added a NESTED test, and it says the enum adds nothing at exactly that cell (0.93) while adding at 13 of 16 elsewhere. **No cell is both affordable and supported by both measurements**, which is the finding. |
| **NEW-2** — five code-unit cells read the other way, one flips the verdict | all five printed in §2.5, with the flip named (`L = 13` T4) and the reason none is evidence. |
| **NEW-3** — the currency table misnames two of its own rows | corrected, **and the source mislabel is named**: `eval_families` §A1's *"1.4 observations per parameter"* is `62 370/45 271` — parameters per POSITION, the reciprocal. R-A1-L11 is at **0.73**. |
| **NEW-4** — §1.5 omits `book_v3`'s standing claim | the R7 claim is quoted, and the *"book cost is EQUAL"* move now rests on D-704's argument that R7 and the rows are the same run — stated as an argument, not a measurement. |
| **NEW-5** — §6's axis does not answer its own question | §6 says so: with one funded run, *"run first"* is *"run only"*, and an order of readiness is not an order of expected value. |
| **NEW-6** — §12 claims two fixes that did not happen | both rows corrected; §1.3's six digits are still unreceipted and now say so in §12 as well as in §1.3. |
| **NEW-7 / M-6** — §7 retains the axis and the cell §6 disavows | §7's heading is now BUILD cost and R-H-EXT's cell carries NEW-8's finding. |
| **NEW-8** — R-H-EXT's build cost is priced at nil | corrected: 3 to 6 integers with **no procedure and no metric** for their values, which is R-A2's structure at a smaller scale, and the difference is stated rather than hidden. |
| **NEW-9** — §1.8's REASON for 44.70 % is false at the code | corrected: `HandcraftedV0` OVERRIDES `delta`; the number is right and the justification was not. |
| **NEW-10** — §4 cites §2.5 for a 1.77x it does not contain | corrected; 1.77x is the enum memo's figure against the WITHDRAWN criterion. |
| **NEW-11** — §1.6's column header describes one of six rows | corrected to *"pairs the run is capped at"*. |
| **NEW-12** — four loosenesses in §3's table | all four: the winning side named, `p` marked one-sided, `5.25` marked an effect size, and the sizes-to-architectures step marked as a step. |
| **NEW-13** — R-H-EXT's abort threshold cannot fire | **the floor is moved from 0.69x to 0.95x (502 792 nps)**, where a row expecting parity actually aborts. The `c` bracket stays as the honest uncertainty. |
| **m-1 … m-10** (round 1, all still landing) | §11→§8 pointer; the bare §6.5; the ±3.5 % arithmetic on the page; the THIRD play seat; the 4.83 h / 2-3 h reconciliation via `mean_pairs 3102.1`; the per-answer term measured (479 / 457 ms) rather than configured; the arithmetic file's two pre-correction rows; `L = 7` T4 and T3 are ONE partition (fifteen distinct, not sixteen); **three of five instruments are untracked and the claim that all were committed is withdrawn**; and D-653's flip clause with the *"quoted whole"* claim. |

**WHAT THE SECOND RED TEAM WOULD STILL DO DIFFERENTLY**: it reaches round 1's
ordering by a different route — R-H-EXT's access to `book_v3` asserted rather
than established, and its build cost not nil. **The second half is conceded and
fixed (NEW-8); the first is answered by NEW-4's paragraph and remains an
argument.** Its bigger re-ranking — that `R-A4-CLASS` at `L = 7` T4 belongs in
the field — **is accepted and acted on.**

---

## §14 WHAT THE THIRD RED TEAM CHANGED, AND WHAT STAGE E's FOURTH ROUND CHANGED WITH IT

`matrix_wp22_phase2_eval_REDTEAM_round3.md`, fresh context, at `c7a3ee1` —
**FAIL**, 3 BLOCKING, 8 MAJOR, 10 MINOR. It verified all eight receipts, re-derived
all sixteen registered-criterion ratios and all sixteen nested ratios with its own
parser (144/144 shared cells agreeing between `census_r3` and `census_r4`), and
re-did §3's paired analysis from the pilot receipt. Round 4 of four (D-709).

| finding | what changed, and what was run |
|---|---|
| **R3-B1** (BLOCKING) — the decisive nested figure was the MAX of three replicates over an unregistered replicate count; the mean of the same three gives 1.11 | **Resolved by MEASUREMENT, not by choosing a summary.** `--null-replicates` was added, and Stage E's round 4 then found the deeper defect below. At twelve replicates with the corrected null the enum adds under **both** summaries at all sixteen cells, so the choice no longer decides anything. |
| **Stage E round 4, NEW-1** (BLOCKING, found while R3-B1 was being fixed) — **the nested null was permuted over the whole code space and its join had ~4x the cells of the join it refereed** | fixed at the code: `permuted_within_counts` permutes within each stone-count stratum, matching the join's cell count at **1.00x** at every rung and length (MEASURED: 3.96 → 1.00 at `L = 7` T4). `test_census.py` pins that it matches AND that it is not the identity. **The corrected run reverses §2.5.** |
| **R3-B2** — `census_r4`'s digest is in no tracked document, and `n6_mutants`' printed digest matches nothing on disk | both wired, **and the recurring class got a structural fix**: `tools/receipt_digest_check.py` verifies every digest a document prints against the receipt on disk and flags a document that names a receipt directory without printing its digest. **It caught three instances on its first run, two of which no reviewer had listed.** It cannot be a CI gate — `artifacts/` is gitignored, the reason `book_v3_ledger.md` gives about its own disjointness script. |
| **R3-B3** — the field is still incomplete | **`R-A1-L7F` added and ranked fourth**: 1 029 parameters, median 1 282, **no cell under ten**, traffic 21. `R-A1-L8F`'s *"densest free table the corpus supports"* is corrected. |
| **MAJOR** — *"three orders of magnitude"* is cross-currency | corrected to **13.3x** in positions per parameter, the only unit R-A4 and R-A3 share. *(The red team's own 6.6x was cross-currency too.)* |
| **MAJOR** — the `t`-merge figures mix populations, and 393 against 384 | both populations printed with their denominators, and the 393/384 difference explained: they are the same computation with and without the rule-4 exclusion. |
| **MAJOR** — NEW-3's, NEW-8's and NEW-13's fixes each missed a site | all sites swept: the currency table's two rows, §6's build-cost sentence, and the two *"highest floor"* claims. |
| **MAJOR** — the ARITHMETIC file is headed for a revision it does not check | corrected, and it now says which revisions it checks and which it does not. |
| **MINOR ×10** | the four round-1 minors round 2 found still landing, plus round 3's — all closed. |

**THE RANKING DISPUTE, ACROSS THREE RED TEAMS.** Round 1 and round 2 would rank
`R-A5-TOPK` first and `R-H-EXT` second or not at all; round 3 would order
A5 → L7F → L8F → A4 and **not rank R-H-EXT until the operator rules**. This
matrix ranks `R-H-EXT` first on build cost with its book cost equalised and its
build cost no longer priced at nil, and ranks `R-A4-CLASS` second on the
corrected measurement. **The disagreement is real, it is recorded in §6, and its
two open questions go to the architect** — may `book_v3` fund a row that does not
answer R7, and is `R-H-EXT` in Phase 2's scope.
