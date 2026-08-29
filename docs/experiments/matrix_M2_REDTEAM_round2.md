# DECISION-RED-TEAM ROUND 2 — MATRIX M2 (WP-1.5d, the safety-net candidate cap)

## Header

**Revision reviewed.** `docs/experiments/matrix_M2.md` at **revision 2**, commit
`f8754b857825671ed1f31ca04fe5bea439d3aea1`.
`git rev-parse HEAD` = `f8754b857825671ed1f31ca04fe5bea439d3aea1`; `git status --short` empty.
**The revision reviewed IS HEAD, and the working tree matches it.**

**Digests verified** (`sha256sum`, live tree, this session). All four match the matrix header
(lines 44-49) exactly:

```
db8a8793d6a2b4a5f2635c60139b3577a68c6a872331fc89acfa43b2fb327be5  artifacts/wp15d_m2_evidence_v1.txt
081c928a0900ae9332a2e1f2b3fe012732a2b0e56b97c4de2064e06fd34add76  artifacts/wp15d_m2_evidence_instrument_v1.txt
455aef9e235785986290a0ce43c5fe6cb835532e5ede0923027c169a0d0c3b7f  artifacts/wp15d_m2_evidence_v2.txt
f73608dd3693762e02968e6ec9a4c8078ac109fcc44bf3580ab8b1fc437d632c  artifacts/wp15d_m2_evidence_instrument_v2.txt
```

**Read in full.** `docs/experiments/matrix_M2.md` (rev 2); `docs/experiments/matrix_M2_REDTEAM.md`
(round 1, `c075bcc`) **in full, including its NOT-REPRODUCED and SOUND sections**; `CLAUDE.md`;
`docs/process.md`; `docs/experiments/WPQ_seed.md` (M2 debt note, §7.1, §7.2);
`docs/decisions.md` D-7, D-74, D-95, D-124, D-291, D-310, D-315, D-318, D-374, D-388, D-423,
D-424, D-473, D-474, D-475; `sessions/WP-1.9/2026-08-29-WP19-STOPPED.md`;
`sessions/WP-1.9/wp19_design_REVIEW_rev2.md` BLOCKING N1;
`crates/pistol-search/src/{staged,pvs,candidates,params,stop,search,info}.rs`,
`crates/pistol-search/src/tt/`, `crates/pistol-engine/src/validate.rs`, all twelve committed
configs naming `quiet_top_k`; both v2 artifacts end to end;
`artifacts/wp19_design_census_v1.txt`; `artifacts/wp15d_bench_block_receipt_v1.txt`.

**Where the re-runs were taken.** A NEW detached worktree,
`git worktree add --detach /home/tom/Projects/pistol-wt-rt2 c075bcc`, with
`CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rt2-target`, both on `/home`. The matrix's own
revision-2 instrument was reconstituted there from
`artifacts/wp15d_m2_evidence_instrument_v2.txt` PARTS 1, 3, 4, 5 and 6 (PART 6 supersedes PART 2)
and **applied clean** — `git apply` reported no rejects, which is itself a check that the
committed instrument is complete and reproducible. My own additions are one scope value
(`4 => ply > 1`), two counters (`SN_ROWS_PLY1`, `SN_CAPPED_PLY1`) and one test file,
`crates/pistol-search/tests/zz_rt2_redteam.rs`. **Nothing was written to the live tree except
this file**, and no other session's worktree was touched.

**A base-revision note, checked so nobody re-derives it.** The matrix header says all four
artifacts were "taken in a detached worktree at `fbc8e62`"; both v2 artifacts' own headers say
`c075bcc`. `git diff fbc8e62 c075bcc -- crates/ configs/ tools/` is **empty** (the two commits
differ only in `docs/experiments/matrix_M2.md` and `matrix_M2_REDTEAM.md`), so no measured cell
is affected. Recorded, not charged.

**What I re-ran.**

- **T1** — the ply-1 census on `spread_v1`, `movetime 500`, r ∈ {2, 3}, five scopes.
- **T2** — the same on the SPRT book, all 2 000 openings, `Stop::Nodes(50_000)`, `quiet_radius 2`,
  four seats (25 min wall, single-threaded, detached).
- **T3** — §3's governed-trajectory divergence, re-taken and DECOMPOSED into
  "first stone differs" vs "only the second stone differs" (10 min).
- **T4** — F11's soundness probe across the SEARCHES OF ONE GAME, with the transposition table
  both shared and fresh, which the committed probe never took.

Every seat that overlaps the receipt **replicates it**: T2's `nocap` control reproduces
`RF/BOOK/.../scope0/K0` cell for cell (`sn_rows=1936431`, `pool_mean=78.16`, `opp2=11404333`,
`depth_hist=[0, 10, 1869, 113, 8, 0, 0, 0]`); T2's `except-ply0/K8` reproduces `RF/.../scope1/K8`
cell for cell (`sn_rows=616627 sn_capped=611121 pool_mean=76.12 emit_mean=8.51 opp2=5181856
opp2cut=2926562`, same histogram); T3 reproduces `RG/GOVERNED` on both trajectories
(631/42/0.0666 and 654/34/0.0520) exactly; T1 reproduces F7's `W-N`, `W-K2` and `W-K0` rows at
both radii. **The instrument is sound and the receipt is honest.** What follows is about what the
document concludes from it.

---

## ROUND 1 CLOSURE LEDGER

| # | round-1 finding | status in revision 2 | evidence |
|---|---|---|---|
| **B1** | field omits "cap ply 0 only" | **CLOSED (row added), BUT ITS DISPOSITION IS SELF-CONTRADICTORY** | `W-K0` is a full row with MEASURED cells (§1, F7, §3). Its own **Kill** field reads "F3 and F6 together"; §5 says "deferred, **not killed**". One document, two dispositions, on the recommendation's strongest competitor — **FINDING 6** |
| **B2** | F2 is about placement; the prefilter class is one degenerate ESTIMATED row | **CLOSED** | F2 re-measured at 11.9–32.5× (four cells, 200 reps); `W-PFR` and `W-PFO` added as MEASURED rows; the D-291 miss is recorded against revision 1 rather than absorbed |
| **B3** | the incumbent's root is searched, not unsearched (956/3 564) | **CLOSED, AND CLOSED PROPERLY** | F6 rewritten; `Provenance::PartialRoot` vs `Fallback` distinguished at `search.rs:410-416`; 957/3 564 re-measured independently; and the selected row no longer caps ply 0 at all, which is the only way to close B3 rather than restate it |
| **B4** | §3 measured at the rejected scope | **CLOSED** | §3 re-taken at every scope over all 2 000 openings; I verified all seven rows' histograms sum to the stated depth-≥3 counts (121 / 644 / 682 / 964 / 1 083 / 1 280 / 2 000) |
| **M5** | wrong denominator | **CLOSED** | `SN_OPP2_IN_POOL` added; §3's second table matches `RH/OPP2` to four figures at all six seats |
| **M6** | `quiet_top_k` never named | **CLOSED IN SUBSTANCE, ONE NEW ELISION** | F12 added and §5 answers it. But F12's own supporting quote elides the disclaimer three lines below it — the same defect M6 caught in revision 1, committed inside the paragraph written to fix it — **FINDING 9** |
| **M7** | `diverged_and_bearing == diverged` is a theorem | **CLOSED** | §3 withdraws it by name and cites `docs/process.md`'s vacuous-criterion clause |
| **M8** | "provably not tactical" oversteps | **CLOSED** | ground 6 now reads "carries no forced obligation at the node where it is pruned"; F1 states the node-local scope explicitly |
| **M9** | the deciding channel is a maximand of degeneracy | **CLOSED BUT INTRODUCED SOMETHING** | F10 is added, is correct, and is demonstrated with the field's own worst row. Then §5 ground 2 and the whole of §3 rank on that channel anyway, at a threshold of ≥3, monotone in K exactly as F10 warns — **FINDING 5** |
| **M10** | `root_restrict` is a correctness hole, not an open choice | **CLOSED** | F3 rewritten as a priced cost with the WP-1.9 precedent cited; "open choice" language gone; the selected row avoids it entirely |
| **M11** | F6's `bench_block.sh` measurement has no receipt | **NOT CLOSED** | `/usr/bin/grep -c "bench_block" docs/experiments/matrix_M2.md` → 1, in the claim itself. `artifacts/wp15d_bench_block_receipt_v1.txt` exists but is the §0 guard receipt (its own header: "the shared bench command block's per-entry guard: red-then-green") and is not cited by the matrix. The number reproduces (round 1 m11); only its provenance is still missing — **FINDING 10** |
| **M12** | "5 at K = 4" misquoted | **CLOSED by deletion** | revision 1's ground 1 sentence is gone |
| **M13** | 3 578.5 vs 3 579.25 | **CLOSED** | §2 now reads "3 579.3 (r=3)"; census `SPREAD p03 safety_mean=1789.23` and F5's weighted mean 1 218.33 both re-derived here |
| **M14** | "every capped cell beats every uncapped cell" | **CLOSED** | §2 now reads "at least as good, and strictly better in 30 of 36" |
| **M15** | W-K3's width figure was W-K2's | **CLOSED by deletion** | the width claim is gone; W-K3 is killed on F11 instead |
| **M16** | `pvs.rs:503-507` drifted | **CLOSED** | `/usr/bin/grep -c "503-507"` → 0. I re-resolved every `file:line` in the document; all resolve (see SOUND) |
| **M17** | F9's inertness asserted of W-K1, measured of W-K2 | **CLOSED** | F9 now states the `sn_rows − sn_capped = 0` reason and cites round 1's R2 |
| **M18** | the instrument charges capped seats for `opp2` bookkeeping inside the timed region | **NOT ADDRESSED — but I bounded it and it does not bite** | the bias is still there. Worst seat (`scope1/K8` on the book): 616 627 rows × ~8.4 `opp2` cells × (~76 pool + ~68 dropped) ≈ 7.4 × 10⁸ comparisons over 376 927 ms ≈ **0.2 %**. Revision 2 now reads wall time as decision-relevant ("25 % FASTER", "16 % SLOWER", "wall neutral"), so the exposure grew — but the deltas being read are 60–120× the bias. **Sound; recorded so the design does not inherit an unbounded one** |
| **M19** | §H's distribution is the incumbent's | **CLOSED, WITH ONE RESIDUAL** | the capped trajectory is measured and reported (5.20 %). Residual: `run()` builds a fresh `Searcher` per turn, so the TT is discarded between turns, while `search.rs:53-56` says "The table is kept: successive searches in one game share what they learned" (D-7). §3's sample is still not the arena's game — **FINDING 7** |

**Two of nineteen are not closed (M11, M18-as-stated), one is closed with a self-contradiction
(B1), one is closed with a new elision (M6), and one is closed by a fix that created a new defect
(M9).** The adjudication table at the head of revision 2 is otherwise accurate: I checked each of
its nine rows against the round-1 text and none overstates what was conceded.

---

## VERDICT

# **FALLS.**

The recommendation of **W-K2** does not survive. **Ground 1 — the ground on which the strongest
competitor is deferred rather than selected — is refuted by measurement, and on the fixture the
package exists to fix it is refuted totally.**

> "1. **It introduces no exclusion at the node where the move is chosen.**"
> — §5, ground 1
>
> "The root keeps full width, so D-124 is not engaged at the node where the move is chosen"
> — §1, W-K2
>
> "interior tails only; the root is untouched" — §4, the W-K2 column

**A turn in this game is TWO stones (CLAUDE.md rule 3), and the engine chooses both of them
inside the search.** `ply` increments once per STONE: `PlyOutcome::TurnContinues` recurses through
`self.child(depth_plies - 1, alpha, beta, ply + 1, index == 0, true)` with `same_side = true`
(`crates/pistol-search/src/pvs.rs:400`), and the answer the engine returns is
`turns_from_plies(state, run.line())`'s first `Turn` (`crates/pistol-search/src/search.rs:328-329`)
— **plies 0 AND 1**. `except-ply0` guards on `ply > 0`. **It caps ply 1. Ply 1 is the second stone
of the turn the engine plays.**

MEASURED, T1, `spread_v1`, `movetime 500`, `quiet_radius 3`, K = 8, `except-ply0`:

| position | safety-net rows | rows the cap bound at | of those, at **ply 1** |
|---|---|---|---|
| 99 stones | 166 | 165 | **165 — 100 %** |
| 51 stones | 364 | 363 | **363 — 100 %** |
| 21 stones | 979 | 977 | 566 — 58 % |
| 11 stones | 1 966 | 1 964 | 382 — 19 % |

**At 51 and 99 stones — the two positions whose completed depth 0 is the entire reason this work
package exists — every single cell W-K2 excludes is excluded at the node that chooses half the
turn it plays.** The one row it leaves whole is ply 0.

And the converse, measured: exempting the root **turn** (a new scope, `ply > 1`) is **inert
exactly where the debt is** — `sn_capped = 0` at 51 and 99 stones at r = 3, completed depth back
to the incumbent's `1 / 0 / 0 / 0`. **W-K2's entire measured effect on the `spread_v1` class is
the ply-1 cap.** Its benefit and the exclusion ground 1 denies are the same object.

The matrix never considers this. `/usr/bin/grep -c "ply 1\|second stone\|root turn\|root TURN"
docs/experiments/matrix_M2.md` returns **0** across all 643 lines.

Grounds 3, 4 and 6 survive. Ground 2 rides the channel F10 disqualifies (FINDING 5). Ground 5 is
measured on a sample that provably excludes the only case where its own scope predicate can vary
for a fixed key (FINDING 4). **Ground 1 falls, and with it the reason W-K0 is deferred.**

### The row that should be selected instead

**W-K0 — cap ply 0 only** — if a single row must be named today. Not because it escapes the
objection, but because it is the same objection at a twenty-fifth of the price, and it is the only
row measured to do the job the package was scheduled for:

| on the matrix's own MEASURED cells | W-K0 | W-K2 |
|---|---|---|
| pays the D-95 / WP-1.4 depth debt (F7, r = 3) | **2 / 2 / 2 / 2, from a COMPLETED iteration** | 1 / 1 / 0 / 0 — does not pay it |
| safety-net rows capped, book | 5 400 / 520 164 (**1.0 %**) | 611 121 / 616 627 (99.1 %) |
| opponent count-two cells cut, pool denominator | **0.44 %** | **73.9 %** |
| wall on the book at fixed nodes | **286 645 ms (25 % faster than the incumbent)** | 378 694 ms (neutral) |
| book openings at depth ≥ 3 | 644 | 1 083 |
| TT-sound at the document's own standard (F11) | yes | yes |

The decisive comparison is one the matrix does not draw, and I had to add a row to the field to
draw it. **MEASURED, T2, book, K = 8: capping ply 0 alone — 5 400 rows of 520 164, 1.0 % — buys
644 openings at depth ≥ 3. Capping EVERYTHING below the root turn — 1 593 643 rows of 1 724 042,
92.4 % — buys 524.** Three nodes per search at the root beat one and a half million nodes below
it. That is the whole decision, and it is measured, and it is not in the document.

W-K2 keeps exactly one advantage over W-K0 that survives F10: it does not make
`pvs.rs:329-335`'s fail-open comment false, so it does not inherit WP-1.9's correctness fix. That
is a real cost for W-K0 — **and it is a fix the immediately preceding package has already written
and had re-reviewed as "right and sufficient"**, not an open problem.

**A defensible alternative disposition is D-318's: STOP.** The field is incomplete at its second
revision (FINDINGS 2 and 3); the only row that satisfies the recommendation's own leading ground
is measured to be worth nothing where the debt is; and both surviving candidates cap a node of the
root turn, which `WPQ_seed.md` §7.1 — the governing prior — names as the place a cap "must not"
go. If the architect's reading is that neither half is licensable on this evidence, that reading
is supported.

---

## FINDINGS

### BLOCKING 1 — ground 1 is false: `except-ply0` caps the node where the second stone of the played turn is chosen, and at 51 and 99 stones that is 100 % of what it caps

**Claims attacked**, verbatim:

> "1. **It introduces no exclusion at the node where the move is chosen.** F3 stays true as
> written, D-124 is not engaged, and the correctness hole the immediately preceding package found
> and fixed by excluding ply 0 does not reappear. Every other row that pays the D-95 debt does
> engage it." — §5, ground 1

> "**Wrongly excludes.** Interior tails only. The root keeps full width, so D-124 is not engaged
> at the node where the move is chosen and **F3's fail-open is untouched**" — §1, W-K2

> "| **W-K2**, the interior half | … | interior tails only; the root is untouched |" — §4

**The attack.** Rule 3: "Every later turn = TWO stones by the mover." The engine's answer is a
`Turn`, a pair. Both of its stones are chosen inside `pvs::visit`, at plies 0 and 1, and the
scope guard is on `ply`, not on the turn.

The chain, at `file:line` in the live tree at HEAD:

1. `crates/pistol-search/src/pvs.rs:153` — the root opens at `visit(depth_plies, -INFINITY,
   INFINITY, 0)`, so the root position is at `ply = 0`.
2. `crates/pistol-search/src/pvs.rs:398-401` — a stone that does not end the turn recurses as
   ```rust
   PlyOutcome::TurnContinues => {
       self.child(depth_plies - 1, alpha, beta, ply + 1, index == 0, true)
   }
   ```
   `ply + 1`, `same_side = true`. **Ply 1 under the root is the root mover placing its own second
   stone.**
3. `crates/pistol-search/src/pvs.rs:487-492` — `child` passes that `ply` straight into `visit`
   without a further increment, so the increment is exactly one per stone.
4. `crates/pistol-search/src/search.rs:328-329` — the returned move is
   ```rust
   let pv = turns_from_plies(state, run.line());
   let best = *pv.first()...
   ```
   The first `Turn` of the ply-indexed principal variation. **Its second stone is the ply-1 cell.**
5. The instrument's guard, `pvs.rs` (PART 6 of the committed instrument):
   `1 => ply > 0`. **Ply 1 is capped.**

Every `spread_v1` root is at `Phase::First` with two stones owed (11 = 1+2·5, 21 = 1+2·10,
51 = 1+2·25, 99 = 1+2·49), so ply 1 is unambiguously the mover's own second stone at every cell
below.

**REPRODUCED.** Added `SN_ROWS_PLY1` / `SN_CAPPED_PLY1` to the committed instrument's counter
block and a fifth scope `4 => ply > 1`:

```
cd /home/tom/Projects/pistol-wt-rt2 && CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rt2-target \
  cargo test --release -p pistol-search --test zz_rt2_redteam -- --nocapture --test-threads=1 t1_ply1

T1/SPREAD/mt500/r3/nocap             p02 stones= 51 depth=0 prov=PartialRoot sn_rows= 57 sn_capped=  0 ply1_rows= 56 ply1_capped=  0
T1/SPREAD/mt500/r3/nocap             p03 stones= 99 depth=0 prov=PartialRoot sn_rows= 27 sn_capped=  0 ply1_rows= 26 ply1_capped=  0
T1/SPREAD/mt500/r3/except-ply0/K8    p02 stones= 51 depth=0 prov=PartialRoot sn_rows=364 sn_capped=363 ply1_rows=363 ply1_capped=363
T1/SPREAD/mt500/r3/except-ply0/K8    p03 stones= 99 depth=0 prov=PartialRoot sn_rows=166 sn_capped=165 ply1_rows=165 ply1_capped=165
T1/SPREAD/mt500/r3/except-root-turn/K8 p02 stones= 51 depth=0 prov=PartialRoot sn_rows= 57 sn_capped=  0 ply1_rows= 56 ply1_capped=  0
T1/SPREAD/mt500/r3/except-root-turn/K8 p03 stones= 99 depth=0 prov=PartialRoot sn_rows= 28 sn_capped=  0 ply1_rows= 27 ply1_capped=  0
T1/SPREAD/mt500/r2/except-ply0/K8    p03 stones= 99 depth=0 prov=PartialRoot sn_rows=313 sn_capped=312 ply1_rows=312 ply1_capped=312
```

**165 of 165, and 363 of 363.** At the two stone counts F7 uses to say W-K2 "does not pay the
D-95 debt", every capped row is the root turn's second stone. This same run replicates F7's
`W-N` (r2 `1/1/0/0`, r3 `1/0/0/0`), `W-K2` (r2 `2/1/1/0`, r3 `1/1/0/0`) and `W-K0`
(r3 `2/2/2/2`) rows exactly, so the instrument is the matrix's, not a rival.

**On the book — the sample §3 says the SPRT walks — it is 24.6 %, not 100 %, and still large.**
MEASURED, T2, all 2 000 openings, `Stop::Nodes(50_000)`, `quiet_radius 2`, K = 8:

```
T2/BOOK/nodes50000/r2/except-ply0/K8 openings=2000 sn_rows=616627 sn_capped=611121 \
  ply1_rows=150102 ply1_capped=150102 ... depth_hist=[0, 0, 917, 789, 290, 4, 0, 0] ms=376927
```

`ply1_capped = ply1_rows = 150 102` — **the cap binds at every single ply-1 safety-net row it
meets**, 24.6 % of the 611 121 rows it caps.

**And the exclusion is where the benefit is.** The new `ply > 1` scope isolates it:

| seat, book, K = 8 | rows capped | openings at depth ≥ 3 | Σ wall ms |
|---|---|---|---|
| W-N incumbent | 0 / 1 936 431 | 121 | 379 279 |
| **`ply > 1` (root TURN exempt)** | 1 593 643 / 1 724 042 | **524** | 421 822 |
| **`ply > 0` (W-K2, the recommendation)** | 611 121 / 616 627 | **1 083** | 376 927 |

**The ply-1 cap alone accounts for 559 of W-K2's 962-opening lift over the incumbent — 58 % of
it — and 100 % of its lift on `spread_v1`.** Ground 1 does not merely mis-state a boundary; it
denies the mechanism that produces the row's headline result.

**What this does to the decision.** Ground 1 is the only ground that separates W-K2 from W-K0,
and §5 uses it twice — once to select W-K2 and once to defer W-K0 ("its whole benefit is bought
at the one node this project's own governing documents single out"). Both halves cap a node of
the root turn. W-K0 caps the first stone and prices it at **0.37 %** of the risk class; W-K2 caps
the second stone and prices it at **56.5 %**. §4's "their risks sit at different nodes" is true
only in the sense that the two nodes are the two halves of the same move.

**The governing prior says so in the words the matrix's ground borrows.**
`docs/experiments/WPQ_seed.md` §7.1, on why W-A fell:

> "So W-A caps exactly where it must not — **the root and the PV, where the move is chosen** and
> where D-124 says no oracle will catch a mistake"

and §7.2, the option that document ADOPTED:

> "**The root (ply 0) and every PV node (`beta − alpha > 1`) are NEVER capped.** They search the
> full staged universe."

The matrix cites §7.2 for W-K3's *scope* and never once for its *root exemption*, which is a
turn-level exemption in the prior's own language and a ply-level one in the matrix's.

**BLOCKING**, because it is not a mis-statement inside a surviving argument: the sentence is the
selection's leading ground and the deferral's stated reason, and it is false at the two
measurements the package was scheduled to move.

---

### BLOCKING 2 — the field is still incomplete at its second revision, and the missing row is the one ground 1 describes

**Claim attacked**, §1 line 176:

> "Eleven options."

and §5:

> "**W-K2: score-ranked top-K of the quiet ball, at every ply but ply 0** …"

**The attack.** The field carries four ply scopes: every ply (W-K1), every ply but 0 (W-K2),
ply 0 only (W-K0), non-PV only (W-K3). The scope that leaves the **root TURN** whole — `ply > 1`,
the exemption the prior document actually specifies and the one ground 1 claims for W-K2 — is not
among them. Round 1 found two missing rows; this is a third, and it is the row the recommendation
describes itself as being.

**REPRODUCED.** Scope `4 => ply > 1` added, run on both fixtures (tables in BLOCKING 1). Summary,
K = 8:

| | spread r3 completed depth 11/21/51/99 | book, openings at depth ≥ 3 | book opp2 cut / all |
|---|---|---|---|
| W-N | 1 / 0 / 0 / 0 | 121 | 0 |
| **W-K2T `ply > 1`** | **1 / 0 / 0 / 0** | **524** | 5 179 074 / 9 405 805 (55.1 %) |
| W-K2 `ply > 0` | 1 / 1 / 0 / 0 | 1 083 | 2 926 562 / 5 181 856 (56.5 %) |

**The row is real and non-degenerate on the book** — 121 → 524 openings at depth ≥ 3 with the
root turn kept whole — **and it is measured to be worth nothing on `spread_v1`**, where the D-95
debt lives: identical to the incumbent at all four stone counts, `sn_capped = 0` at 51 and 99
stones.

That is the finding, and it is worse for the matrix than a simple omission would be. **There is
no row in this family that both honours ground 1 and moves the class the package exists for.**
Ground 1 is therefore not a property any selectable option has; it is a property W-K2 was
described as having and does not. Once that is on the table the field's own numbers order the
live candidates by how much of the root turn each one caps, and W-K0 buys more per excluded cell
than anything below it by two orders of magnitude (VERDICT).

**D-318's disposition applies on its own terms**: *"THE FIELD IS STILL INCOMPLETE AT THE SECOND
REVISION, and the missing row dominates the recommended option on the matrix's own trigger."*
Here the missing row dominates on the recommendation's own **leading ground** and loses on the
depth channel — which is the channel this document's own **F10** says cannot rank options. A
matrix cannot both disqualify a channel and use it to beat the row that is better on the ground
it leads with.

---

### MAJOR 3 — W-E as ADOPTED is still not in the field: W-K3 is killed on a defect its own governing specification removes, and its book cell was never taken

**Claim attacked**, §1, W-K3:

> "- **Kill.** **F11, and it is a correctness ground rather than a performance one.** This is
> `WPQ_seed.md` §7.1(3) verbatim, and it is the reason W-E's own specification carried a TT
> truncation rule."

**The attack.** The sentence names the cure and then kills the row without it. `WPQ_seed.md`
§7.2 — the option that document ADOPTED, and the one D-315 records M2 as owing as "an option
among options" — is not "cap non-PV nodes". It is that scope **plus** a store rule:

> "**The transposition store gains a truncation rule**, removing the poisoned-entry class rather
> than living with it. A subset maximum `>= beta` is a genuine lower bound, so **fail-high stores
> `Bound::Lower` as today**; a fail-low or exact score from a set that was **not exhausted** is
> unsound in the bound it claims, so **it stores nothing**."

That rule is exactly and only what F11's 821 disagreements can hurt: the store at
`crates/pistol-search/src/pvs.rs:449-467` writes `Bound::Upper` on a fail-low and `Bound::Exact`
in-window; under the seed's rule a truncated node writes neither. `Bound::Lower` survives and is
valid over a subset. **W-E as specified is TT-sound by construction, and the matrix kills the row
by measuring a version of it the specification does not propose.**

`WPQ_seed.md`'s own M2 debt note is explicit that this is what M2 owes:

> "`W-E` occurs **zero** times at `ec8f7fb` … was supplied by the DECISION-RED-TEAM that killed
> it … So M2 is a **FRESH matrix that has never been authored** … it owes an option matrix over
> the widening schedule … attacked by a fresh-context DECISION-RED-TEAM BEFORE any option is
> selected"

The matrix's own preamble quotes D-315 to the same effect and then does not carry the row.

**Second half: the row has no cell on the channel the recommendation leads with.** §3's book
table scores W-N, W-K0, W-K2 at four K values and W-K1. **W-K3 is absent. So is W-PFR.** The
recommendation's ground 2 is "Its effect is large on the sample the SPRT actually walks", and two
of the four live competitors were never measured on that sample.

**REPRODUCED — I took the missing cell.**

```
T2/BOOK/nodes50000/r2/non-pv-only/K8 openings=2000 sn_rows=933082 sn_capped=910553 \
  ply1_rows=137348 ply1_capped=135157 pool_mean=78.45 emit_mean=9.57 \
  opp2=7878771 opp2cut=4440517 depth_hist=[0, 0, 1118, 679, 202, 1, 0, 0] ms=394025
```

**W-K3 on the book: 882 of 2 000 openings at depth ≥ 3 (44.1 %)** — between W-K0's 644 and
W-K2's 1 083 — at a 56.4 % cut rate indistinguishable from W-K2's 56.5 %. The row is live on the
channel the matrix decides on and was scored without that number.

**One thing this measurement does NOT support, recorded against my own line of attack.** I
expected the PV exemption to protect the second stone of the played turn. It does not:
`ply1_capped = 135 157` of `ply1_rows = 137 348` — **98.4 %**. Only the first-ordered candidate
at ply 1 is a PV node (`index == 0` at `pvs.rs:400`); every sibling opens at `(alpha, alpha+1)`
and is capped. So W-E-as-adopted does not satisfy ground 1 either, and BLOCKING 2 stands
unrelieved by it. The finding here is field completeness and a kill on a strawman, not a
replacement recommendation.

---

### MAJOR 4 — F11's structural claim is false across the searches of one game, the committed probe could not have seen it, and the fact that actually protects the engine is unstated

**Claim attacked**, F11:

> "**F11 — NEW. The transposition store is sound under every UNIFORM ply scope and unsound under
> the non-PV one, MEASURED.** … The reason is structural and worth stating: **the stone count is a
> function of the key and only increases down the tree, so ply 0's key can never recur below it,
> and a scope that is a function of ply alone therefore emits one set per key.**"

and §5, ground 5:

> "5. **It is sound in the transposition table, measured, not argued.** F11: 0 membership
> disagreements at this scope"

**The attack.** The argument establishes that ply is a function of the key **relative to a fixed
root**. The root is not fixed across the searches of a game, and the transposition table is not
cleared between them:

> `crates/pistol-search/src/search.rs:53-56` — "The table is kept: successive searches in one
> game share what they learned … cleared table (docs/decisions.md D-7)."

`crates/pistol-search/src/search.rs:218` calls `self.table.new_generation()`, not `clear()`. So a
key visited at ply ≥ 1 in the search for turn *t* is visited at **ply 0** in the search for turn
*t + 1* — that is precisely the position the game moved to. Under `except-ply0` the first
emission is the top K and the second is the whole ball. **The same key emits two different sets,
which is exactly what F11 declares cannot happen.**

The committed probe could not observe this. `zz_wp15d_m2_rev2b.rs::zz_e_setfn_by_scope` calls
`scratch::setfn_reset()` once per `(fixture, scope, K)` and then iterates over **independent
opening positions**, each with a freshly constructed `Searcher` inside `run()`. Independent
openings rarely share keys, and within any one search ply *is* a function of the key — so
3 329 995 observations and 0 disagreements is very nearly a theorem of the sampling design.
`docs/process.md` names the shape: *"two instruments blind to the same stage are one instrument
reported twice, and their agreement is invariant under a defect in what they are both blind
to."* The structural argument and the probe are blind to the same stage — a moving root — and
they agree.

**REPRODUCED.** `t4_setfn_across_searches_in_one_game`: one game, 6 openings from the book,
24 turns, `Stop::Nodes(20_000)`, all three ordering heuristics ON, the probe's map spanning the
whole game, run with the `Searcher` both rebuilt per turn and shared across turns:

```
T4/SETFN games=6 scope=0 table=fresh-per-turn        observations=279960 disagreements=0
T4/SETFN games=6 scope=0 table=shared-across-turns   observations=256161 disagreements=0
T4/SETFN games=6 scope=1 table=fresh-per-turn        observations=315862 disagreements=15
T4/SETFN games=6 scope=1 table=shared-across-turns   observations=271965 disagreements=18
T4/SETFN games=6 scope=3 table=fresh-per-turn        observations=270898 disagreements=7
T4/SETFN games=6 scope=3 table=shared-across-turns   observations=244039 disagreements=9
```

**Scope 0 (`every-ply`) holds at 0 in both modes — it is the only scope in the field that is a
pure function of the position. Scopes 1 and 3 do not.** F11's sentence "a scope that is a
function of ply alone therefore emits one set per key" is refuted for both of the scopes it
certifies, and the matrix's own §5 open item 2 already names the right criterion — *"whether the
emitted set is a pure function of the position"* — and then certifies two scopes that fail it.

**Is it an actual unsoundness? No — and the reason is a fact the matrix never states.** The only
TT consumer is `crates/pistol-search/src/pvs.rs:245-256`, and it is gated on `!is_pv`:

```rust
let is_pv = beta - alpha > 1;
let known = self.table.probe(key, from_root);
if let Some(record) = known
    && !is_pv
    && record.depth_plies >= depth_plies
    && match record.bound { Bound::Exact => true, ... }
{ return record.score; }
```

`iterate` opens the root at `visit(depth_plies, -INFINITY, INFINITY, 0)` (`pvs.rs:153`), so
`is_pv` is true at ply 0 and **the root never consumes a table bound.** Stone count is monotone,
so the disagreeing key can only be the new root; the disagreement is therefore never read.

**So the ground survives, and its stated support does not.** Ground 5 is load-bearing for the
kill of W-K3, and it rests on an unstated premise that a **root aspiration window** — the
ordinary next step for an iterative-deepening PVS — would delete in one line. Graded MAJOR
rather than BLOCKING because I could not produce a wrong answer, and I looked: `pvs.rs:245` is
the only `.probe(` call site in the crate.

**What would make it sound as written:** state the premise (`iterate` opens at
`(-INFINITY, INFINITY)`, so ply 0 is a PV node and takes no cutoff), pin it with a test, and say
in the ADR that a root aspiration window flips it.

---

### MAJOR 5 — F10 disqualifies the deciding channel, and §5 ground 2 plus the whole of §3 rank on it anyway, at a threshold, monotone in K exactly as F10 warns

**Claims attacked.** F10:

> "**F10 — NEW, AND IT DISQUALIFIES THE CHANNEL REVISION 1 DECIDED ON.** Completed depth on
> `spread_v1` is monotonically improved by narrowing, without bound, so it ranks options by how
> hard they narrow. … **Depth cannot decide this field. What the channel CAN carry is F6's
> binary defect — completed depth 0, no iteration finished — and that is the only way it is read
> below.**"

against §5, ground 2:

> "2. **Its effect is large on the sample the SPRT actually walks.** MEASURED §3: 121 → 1 083 of
> 2 000 book openings at completed depth ≥ 3 at K = 8."

**The attack.** "That is the only way it is read below" is false of the document below it. §3's
entire table — seven rows, the one the recommendation's ground 2 cites — is headed "openings at
completed depth **≥ 3**". That is a threshold at 3, not F10's binary at 0. And it is monotone in
the narrowing direction across the matrix's own grid, which the matrix states three paragraphs
later as a trap for the K calibration and does not apply to its own field scoring:

| K, `except-ply0` | 4 | 8 | 16 | 32 |
|---|---|---|---|---|
| openings at depth ≥ 3 (§3) | **1 280** | 1 083 | 964 | 682 |

Extending it across the field with my measured rows makes the point unarguable — the column is a
near-perfect ranking by how much of the root turn a row caps, which is F10's degeneracy in the
place F10 says it was removed from:

| seat, book, K = 8 | openings at depth ≥ 3 |
|---|---|
| W-N (caps nothing) | 121 |
| W-K2T (`ply > 1`, root turn whole) | 524 |
| W-K0 (`ply == 0`, 3 rows per search) | 644 |
| W-K3 (`!is_pv`) | 882 |
| W-K2 (`ply > 0`) | 1 083 |
| W-K1 (every ply) | 2 000 |

On this column W-K1 wins the field outright, and §4 rejects it. So the column is not being used
to rank — but ground 2 is written as though a bigger number is a better row, and it is the ground
that carries the recommendation's benefit case. **CLOSED BUT INTRODUCED SOMETHING:** M9 was
answered by adding F10, and the fix's own scope sentence is contradicted by two sections of the
document it was added to.

**REPRODUCED** — §3's own table (I re-derived all seven histogram sums), plus T2's three new
rows, commands in BLOCKING 1 and 2.

**The repair is small**: say that §3's histogram is offered as evidence of SPRT SENSITIVITY —
which is what ground 3 already claims and what §3's divergence table actually supports — and not
as a benefit magnitude that ranks rows. As written, ground 2 and F10 cannot both stand.

---

### MAJOR 6 — W-K0 is disposed of two ways in one document, and it is the row the recommendation is chosen against

**Claims attacked.** §1, W-K0's own field row:

> "- **Kill.** F3 and F6 together: it is a pure root prune, and its whole benefit is bought at the
> one node this project's own governing documents single out."

against §5:

> "- **W-K0, the root half.** … It is **deferred, not killed**, and it owes its own package with
> its own SPRT arm"

**The attack.** Every other row in the field uses **Kill** for a falsifiable condition — W-N: "Any
row that lifts the book's depth distribution while holding the corpus"; W-K2: "A corpus bench
outside its registered bracket"; W-CFG: "MEASURED, its own floor"; W-PFR: "Not killed." W-K0's
**Kill** field states no condition at all. It restates two costs, both of which the same row's
**Cost** field already prices, one of which (F6) is a *measured advantage* for W-K0 — 2/2/2/2
from a completed iteration against the incumbent's depth-1 salvage.

The consequence is not cosmetic. A reader scanning the field sees the recommendation's strongest
competitor marked KILLED; a reader reaching §5 sees it marked DEFERRED and owed its own package.
The two readings license different next actions, which is precisely the test CLAUDE.md sets
("THE TEST IS WHETHER THE DISPUTED CLAIM CHANGES WHAT ANYONE MAY CONCLUDE"). Here it does: under
one reading W-K0 is out of the field and under the other it is the next package.

**REPRODUCED** — reading, `docs/experiments/matrix_M2.md` §1 W-K0 and §5 "WHAT IS DEFERRED".
Both quoted verbatim above.

---

### MAJOR 7 — §3's governed sample discards the transposition table between turns, which the engine does not, and §3 is headed "WHAT THE GOVERNED RUN WOULD SEE"

**Claim attacked**, §3:

> "**The SPRT is not predictably insensitive, and that conclusion no longer rests on one
> trajectory.** A change that moves the played turn on 5–7 % of a governed game's searches is
> inside what a paired SPRT over this book can see."

**The attack.** The driver
(`artifacts/wp15d_m2_evidence_instrument_v2.txt`, `zz_g_governed_both_trajectories`) plays 40
turns of a game by calling `run(&state, …)` twice per turn, and `run` constructs a fresh
`Searcher` on every call:

```rust
fn run(state: &GameState, cap: usize, scope: usize, ordering_on: bool) -> Row {
    ...
    let mut searcher = common::staged_searcher(2, 2, 3, 0, QTriggers::DefensiveOnly, ...);
```

The engine does not do that. `crates/pistol-search/src/search.rs:53-56` states the opposite as a
design commitment under D-7, and `search.rs:218` calls `new_generation()` rather than `clear()`.
A retained table changes move ordering, effective depth and — by MAJOR 4's own measurement — the
membership consistency of the emitted sets, all three of which feed the divergence rate §3 reads.
The direction is not predictable from the armchair, which is why it needs measuring rather than
arguing.

The two seats are otherwise **genuinely identical**, and I checked that specifically because the
dispatch asked: same `staged_searcher(2, 2, 3, 0, DefensiveOnly, ordering all false)`, same
`Stop::Nodes(50_000)` (reproducible, `stop.rs:65-70`), same `state`, the cap the only difference.
That part of the design is right.

**REPRODUCED — the divergence itself replicates exactly, on my own harness.**

```
T3/GOVERNED scope=except-ply0 trajectory=incumbent searches=631 diverged=42 (0.0666) \
  first_stone_differs=31 only_second_stone_differs=11
T3/GOVERNED scope=except-ply0 trajectory=capped    searches=654 diverged=34 (0.0520) \
  first_stone_differs=25 only_second_stone_differs=9
```

631/42/0.0666 and 654/34/0.0520 — §3's cells to the digit. **§3's numbers are right; what they
are numbers ABOUT is not the arena's game.** Graded MAJOR because §3's heading and its
conclusion are both about the governed run.

**A by-product worth keeping.** 11 of 42 and 9 of 34 divergences — **26 %** — are turns whose
FIRST stone agrees and whose second stone does not. That is consistent with BLOCKING 1 and not
with ground 1, though it does not isolate the ply-1 cap on its own (a second-stone change can
also come from a changed subtree); the `ply1_capped` counters are the direct evidence.

---

### MINOR 8 — the F11 digest is sound, and one property of it should be stated rather than left to a reader

**Claim examined**, F11:

> "(… Revision 1's own first attempt at this probe was VOID — an order-sensitive digest reporting
> every table-move promotion as a disagreement — and the void run is kept in the receipt so the
> correction is visible.)"

**Audited, and it holds.** The re-take (`artifacts/wp15d_m2_evidence_instrument_v2.txt`, the
`pvs.rs` hunk) folds a splitmix64-style finalizer over the cells with **addition**, seeded with
the length:

```rust
let mut digest: u64 = cells.len() as u64;
for cell in &cells {
    let mut mix = (cell.q as i64 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (cell.r as i64 as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f);
    mix ^= mix >> 29; mix = mix.wrapping_mul(0xbf58_476d_1ce4_e5b9); mix ^= mix >> 32;
    digest = digest.wrapping_add(mix);
}
```

- **Order-insensitive**: addition is commutative, so a `promote_table_move` rotation and every
  ordering heuristic leave it fixed. That is the correction, and it is the right one.
- **Collision-resistant enough for 3.3 M observations**: each `mix` is a bijection of a
  well-separated 64-bit input, so two distinct sets collide with probability ≈ 2⁻⁶⁴ per
  comparison. Over the 3 329 995 comparisons the matrix reports, P(any hidden disagreement)
  ≈ 3.33 × 10⁶ / 1.8 × 10¹⁹ ≈ **1.8 × 10⁻¹³**. **A collision cannot plausibly be hiding a
  disagreement.**
- **Correctly placed**: the fold sits immediately after the `let cells = match … };` block — after
  the cap, after `promote_table_move`, after the `root_restrict` retain — and nothing mutates
  `cells` between there and the `self.table.store(key, from_root, Record { … })` at
  `pvs.rs:449-467`. It also records at nodes that later abort and therefore never store, which
  over-counts rather than under-counts. Both directions are conservative.

**The one thing to state**: the digest is a MULTISET fold, so it is a membership check only
because `cells` is duplicate-free (`candidates.rs:42-61` builds a `BTreeSet`). That is true here
and it should be the sentence the design pins, because a future generator that emits a cell twice
would silently weaken the probe. MINOR, and recorded as a strength of the re-take rather than a
defect.

---

### MINOR 9 — F12's supporting quote elides the disclaimer in the same comment, which is the defect round 1's MAJOR 6 caught in revision 1

**Claim attacked**, F12:

> "`configs/instrument_staged_v0.toml:10-15` further asserts that "the cut BINDS here"."

**The attack.** The comment continues, in the same paragraph the citation spans:

```
# THE CUT BINDS HERE (U2_node_protocol.md §7.2, carried from WPQ_seed.md): a
# seat with the quiet cut disabled would make the SPRT measure nothing about
# the prune (CLAUDE.md rule 6). `quiet_top_k`/`widen_schedule` are stage Q's
# own knobs, validated here for schema completeness and not yet read by the
# search this D-scope ships (docs/decisions.md D-353, D-356) — carrying them
# is what a `deny_unknown_fields` document requires
```

The file **already says** the two keys are not what binds. F12's conclusion — "re-purposing these
two silently changes what twelve committed headers assert" — is materially weaker than stated for
this header, which asserts the opposite. Round 1's MAJOR 6 closed with: *"The matrix quotes the
one line range in the tree that names the key, elides the naming, and then never mentions the key
in 475 lines."* Revision 2 fixed that elision (F1 now quotes `staged.rs:222-236` whole, verified)
and committed a new one in the paragraph written to fix it. MINOR because F12's substantive point
— that the keys exist with the opposite semantics and have no off-value — is correct and
independently supported by `U3_tier_t.md` §10, `validate.rs:94-97` and
`configs/tactical_staged_v0.toml:12`.

---

### MINOR 10 — §5's third "open" item is not open: hard rule 1 decides it

**Claim attacked**, §5:

> "- **The gate's own surface.** F12 says a new key is owed; where it lives, what its off-value is,
> and **which of the twelve documents gain it** are the design's."

**The attack.** Hard rule 1: *"Explicit + complete; `serde(deny_unknown_fields)`; missing key =
error; NO code-side default for any tunable."* A new field on the staged candidate-policy section
must appear in **every** config that carries that section, or those configs stop loading. There is
no subset to choose. `/usr/bin/grep -rln "quiet_top_k" configs/ | LC_ALL=C sort | wc -l` → **12**;
all twelve gain it. Listing a settled consequence as an open design choice invites a design that
re-opens it. The other two clauses of the bullet (where it lives, what its off-value is) are
genuinely open.

---

### MINOR 11 — `ROOT_DONE` accumulates across iterative-deepening iterations; F6's four cells are safe because all four are single-iteration, and the column heading is not what the counter computes

**Claim examined**, F6's table heading: "root children completed".

**The attack.** The counter is `crate::scratch::ROOT_DONE.fetch_add(1, …)` inside the candidate
loop under `if ply == 0`, reset only by `scratch::reset()` at the top of a whole search — never
between iterations, while `ROOT_WIDTH` is `store`d and therefore overwritten. So `ROOT_DONE` is
the SUM over every iterative-deepening iteration. The receipt shows it plainly:

```
RA/SPREAD/mt500/r2/ply0-only/K8 p03 stones= 99 depth=2 prov=completed root_width=8 root_done=16
RA/SPREAD/mt500/r2/nocap        p00 stones= 11 depth=1 prov=partial   root_width=198 root_done=267
```

`root_width = 8, root_done = 16` is two completed iterations of 8, not "16 of 8 children";
`root_done = 267 > root_width = 198` is the same thing.

**The four cells F6 actually reads are safe**, and I checked each: all four are `depth=0
prov=partial`, i.e. the depth-1 iteration was the only one attempted, so the sum has one term.
`957 / 3 564`, `530 / 1 836`, `337 / 756`, `642 / 1 782` all read directly off
`RA/SPREAD/mt500/r{2,3}/nocap`. B3's closure is therefore sound. The heading is not, and a design
that reuses the counter at a completed cell would read it wrong.

---

### MINOR 12 — F6's provenance is still uncited (round 1's M11, re-raised)

Listed in the ledger; repeated here so it is not lost. F6's four completed-depth numbers are
attributed to *"the shipped `target/release/pistol` and `tools/bench_block.sh`"* under
`configs/play_v0.toml` and `configs/play_staged_v0.toml`. The matrix's header names five
artifacts; none contains that run.
`/usr/bin/grep -n "bench_block\|play_staged" artifacts/wp15d_m2_evidence_v2.txt` returns nothing,
and `artifacts/wp15d_bench_block_receipt_v1.txt` is a different measurement (the §0 per-entry
guard, D-475). Round 1 reproduced the numbers from a clean worktree, so this is provenance, not
accuracy — but F6 is the fact the whole work package exists to move, and it is the one MEASURED
cell in the document with no receipt behind it.

---

## THE STRONGEST SURVIVING ATTACK

> **BLOCKING 1.** The recommendation's leading ground, and the sole reason its strongest
> competitor is deferred rather than selected, is that W-K2 "introduces no exclusion at the node
> where the move is chosen". A turn in this game is TWO stones (rule 3) and the engine chooses
> both inside the search: `ply` increments once per stone (`pvs.rs:398-401`,
> `PlyOutcome::TurnContinues → child(…, ply + 1, …, same_side = true)`), and the returned move is
> the first `Turn` of the ply-indexed principal variation (`search.rs:328-329`) — plies 0 **and**
> 1. `except-ply0` guards on `ply > 0`, so it caps ply 1, which is the second stone of the turn
> the engine plays. MEASURED at `movetime 500`, `quiet_radius 3`, K = 8: at 99 stones **165 of the
> 165** safety-net rows the cap binds at are ply-1 rows, and at 51 stones **363 of 363** — at the
> two positions whose completed depth 0 is the reason this package exists, *everything* W-K2
> excludes is excluded at the node that chooses half the played turn. The converse is measured
> too: a scope that exempts the root TURN (`ply > 1`) is inert there — `sn_capped = 0` at 51 and
> 99 stones, completed depth back to the incumbent's `1/0/0/0` — and on the book it buys 524
> openings at depth ≥ 3 against W-K2's 1 083, so **58 % of W-K2's book lift and 100 % of its
> spread lift are the ply-1 cap**. The row's benefit and the exclusion ground 1 denies are the
> same object. `docs/experiments/WPQ_seed.md` §7.1 already names this as the place a cap "must
> not" go — "the root and the PV, **where the move is chosen**" — and §7.2's adopted option
> exempts the root and every PV node accordingly. The matrix never considers it:
> `/usr/bin/grep -c "ply 1\|second stone\|root turn" docs/experiments/matrix_M2.md` returns **0**
> across 643 lines. Once ground 1 goes, W-K0 leads on every remaining measured cell — it pays the
> D-95 debt at all four stone counts from a completed iteration, excludes 0.44 % of the risk class
> against W-K2's 73.9 %, and runs 25 % faster on the book — and the decisive comparison, which the
> field had to be extended to draw, is that capping ply 0 alone (**5 400 rows of 520 164**) buys
> **644** book openings at depth ≥ 3 while capping everything below the root turn (**1 593 643
> rows of 1 724 042**) buys **524**.

I chose this over BLOCKING 2 after trying to rebut it three ways and failing. It is not repaired
by completing the field — BLOCKING 2 measures the completing row and finds it worth nothing where
the debt is — and it is not repaired by re-wording ground 1, because the sentence is not
decorative: it is the entire separation between the selected row and the deferred one, and the
measurement puts both caps inside the same turn.

**What would flip this report's verdict**, stated so the architect can check it rather than take
it: a measurement showing that the ply-1 cap's exclusion is harmless *as a move choice* — the
natural instrument is the paired second stone, held to the incumbent's ply-0 choice, scored by
the same SPRT arm — or a demonstration that `spread_v1`'s 100 % ply-1 concentration is an artefact
of that fixture and does not hold on the book, which T2 already contradicts at 24.6 %.

---

## WHAT I CHECKED AND FOUND SOUND

- **All four digests** match the matrix header exactly. The committed revision-2 instrument
  reconstitutes and `git apply`s **clean** onto `c075bcc`; PART 6 does supersede PART 2 as its
  header says.
- **§3's book table, every cell.** All seven depth-≥3 counts re-derived from the histograms in
  `RF/BOOK/*` (121, 644, 682, 964, 1 083, 1 280, 2 000); all seven `sn_capped / sn_rows` and
  `opp2cut / opp2` ratios recomputed (1.0 %, 25.85 %, 44.1 %, 56.48 %, 66.87 %, 61.46 %, 0.370 %).
  Every one matches to the stated precision.
- **§3's pool-denominator table** matches `RH/OPP2` at all six seats to four figures
  (0.8560 / 0.7390 / 0.5862 / 0.3288 / 0.7949 / 0.0044). MAJOR 5's correction is properly landed.
- **F11's observation counts.** `Σ` over scopes 0, 1, 3 across three fixtures and two K values =
  **3 329 995**, exactly as stated; scope 2 = **1 254 229** with **821** disagreements
  (30+3+3+3+505+277). No arithmetic slip.
- **F11's digest** is order-insensitive, correctly placed, conservative in both directions, and
  collision-resistant to ~10⁻¹³ over the reported sample (MINOR 8). **The re-take is a proper
  correction of a genuinely void first attempt, and the void run being kept in the receipt is the
  right disposition.**
- **F5's census cells, re-derived from `artifacts/wp19_design_census_v1.txt`**: corpus
  66 / 108 662 = **0.000607** ✓, mean width **78.12** ✓ (57 rows × 76.60 + 9 × 87.78, weighted);
  spread 2 468 / 4 015 = **0.6147** ✓, mean width **1 218.33** ✓ (235×208.99 + 265×385.34 +
  772×927.07 + 1 196×1 789.23, weighted), share of all batched cells **0.9966** ✓.
- **F7, F8 and F9 replicate on my own instrument** where I re-took them: W-N `1/1/0/0` (r2) and
  `1/0/0/0` (r3); W-K2 `2/1/1/0` (r2) and `1/1/0/0` (r3); W-K0 `2/2/2/2` (r3). F9's inertness
  argument (`sn_rows − sn_capped = 0` on the corpus) is correct and correctly explained.
- **Every `file:line` citation in the document resolves at HEAD**, and I checked them
  individually: `staged.rs:222-236` (quoted **whole** — round 1's elision is closed),
  `staged.rs:209-213`, `staged.rs:230, 233`, `staged.rs:283-295`, `pvs.rs:153`, `pvs.rs:174-186`,
  `pvs.rs:245-256`, `pvs.rs:329-335`, `pvs.rs:336-347`, `pvs.rs:368`, `pvs.rs:449-467`,
  `pvs.rs:498, 500`, `search.rs:410-416`, `validate.rs:94-97`, `validate.rs:104-121`.
  (`pvs.rs:427-429` is the comment and the `if`; the `break` is at 430. Not worth a finding.)
- **F12's config census.** 12 distinct committed files, 12 assignment lines; `validate.rs:94-97`
  does refuse `quiet_top_k == 0`; `validate.rs:104-121` does cross-validate `widen_schedule`
  strictly greater than `quiet_top_k`; `configs/tactical_staged_v0.toml:12` does disable the cut
  by setting K above the pool. **The semantics claim is correct**, MINOR 9 notwithstanding.
- **§2's W-CFG kill.** `check_radius` (`validate.rs:162-170`) accepts `1..=MAX_CANDIDATE_RADIUS`,
  so `quiet_radius = 1` really is the floor, and the 99-stone position really does complete zero
  iterations there. The kill fires on its own terms.
- **F4 at HEAD**, re-verified independently of round 1: `pvs.rs:153` opens at
  `(-INFINITY, INFINITY, 0)`; `pvs.rs:368` starts `best_score` at `-INFINITY`; `pvs.rs:498/500`
  open non-PV children at `(alpha, alpha + 1)`; the `won || alpha >= beta` break is at 429-430.
  W-W's kill is structural and sound.
- **The `opp2` instrumentation bias (round 1's m18) is bounded and does not bite**: ≈ 0.2 % at the
  worst seat against wall-time deltas of 16-25 %.
- **F10 is correct and is the most valuable new fact in the revision.** The ring-order prefilter
  really is the deepest seat in the document (`RC/…/ringorder/M2/every-ply`: 5 / 6 / 5 / 5) and
  really does play `-4,1/-2,1` at 11, 51 and 99 stones alike on a `root_width = 2` tree. Carrying
  the degenerate row so the degeneracy is measurable rather than argued is exactly right, and
  MAJOR 5 is a complaint that the document does not follow its own finding, not that the finding
  is wrong.
- **W-PFR's deferral condition is sound and checkable** — "a measured whole-engine cost finding
  that names the safety-net eval pass as a hotspot on a governed-shaped workload, which no rule-5
  receipt currently does". That is the right shape for a promotion clause. Its evidence is
  incomplete in the same way W-K3's is (no book cell — MAJOR 3), but the condition itself is
  well-formed and I could not break it.
- **The recommendation's honesty about what it costs.** "WHAT SELECTING W-K2 COSTS, STATED AND
  NOT DISCOUNTED" states the D-95 non-payment, the 56.5 % exclusion and the worst wall time in the
  field, and "THREE THINGS THIS RECOMMENDATION DOES NOT SETTLE" names the F10 calibration trap in
  its own data. Revision 2 concedes every one of round 1's four BLOCKING findings without
  softening, and says so in a table a reader can check. That posture is why this round's findings
  are about a mechanism the document never modelled rather than about anything concealed.

---

*DECISION-RED-TEAM round 2 on `docs/experiments/matrix_M2.md` at `f8754b8`. Nothing here selects
an option. Re-runs were taken in `/home/tom/Projects/pistol-wt-rt2`, detached at `c075bcc`, with
`CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rt2-target`, both on `/home`; the scratch driver is
`crates/pistol-search/tests/zz_rt2_redteam.rs` and the instrument delta is one scope value
(`4 => ply > 1`) plus two counters. The live tree was never edited except to write this file, and
no other session's worktree was touched.*
