# DECISION-RED-TEAM — MATRIX M2 (WP-1.5d, the safety-net candidate cap)

## Header

**Revision reviewed.** `docs/experiments/matrix_M2.md` at `5b2c97284f3fbca49d0d3ded539a57f96f0772d7`.
`git rev-parse HEAD` = `5b2c97284f3fbca49d0d3ded539a57f96f0772d7`; `git status --short` empty.
**The revision reviewed IS HEAD, and the working tree matches it.**

**Digests verified** (`sha256sum`, live tree, this session):

```
db8a8793d6a2b4a5f2635c60139b3577a68c6a872331fc89acfa43b2fb327be5  artifacts/wp15d_m2_evidence_v1.txt
081c928a0900ae9332a2e1f2b3fe012732a2b0e56b97c4de2064e06fd34add76  artifacts/wp15d_m2_evidence_instrument_v1.txt
```

Both match the matrix's header (§ preamble, lines 20-24). No third artifact is cited by the matrix.

**Read in full.** `docs/experiments/matrix_M2.md`; `CLAUDE.md`; `docs/process.md`;
`docs/experiments/WPQ_seed.md` (M2 debt note, §7.1, §7.2, the test rows);
`docs/decisions.md` D-74, D-95, D-124, D-291, D-310, D-315, D-318, D-353, D-356,
D-374, D-388, D-423, D-424, D-473, D-474, D-475; `sessions/WP-1.9/2026-08-29-WP19-STOPPED.md`;
`sessions/WP-1.9/wp19_design_REVIEW_rev2.md` BLOCKING N1 (a)(b)(c);
`crates/pistol-search/src/{staged,pvs,candidates,params,stop,quiescence,search}.rs`,
`crates/pistol-search/src/tt/{mod,entry}.rs`, `crates/pistol-engine/src/{validate,params}.rs`,
and every committed config that states `quiet_top_k`.

**Where the re-runs were taken.** A NEW detached worktree,
`git worktree add --detach /home/tom/Projects/pistol-wt-redteam fbc8e620c760db0e5602c3cd36abe6a84cdb75d5`,
with `CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-redteam-target`, carrying the matrix's own
instrument (PART 1 + PART 2 of `artifacts/wp15d_m2_evidence_instrument_v1.txt`, applied by
`git apply` from the wp15d worktree's diff) plus my own additions, described per finding.
A second CLEAN detached worktree, `/home/tom/Projects/pistol-wt-clean` at the same base with its
own target dir, built `--release --workspace --locked` for the shipped-binary reproduction in m1.
**Nothing was written to the live tree except this file.**

**What I re-ran.** R0 cost split; R1 F8 at the committed `tt_bytes`; R2 the corpus at the
recommended scope; R3 a proximity-prefilter seat (W-P as a seat, not an estimate); R4 the book's
2 000 openings at the recommended scope; R5 the root prefix behind a `Provenance::PartialRoot`
answer; R6 a key-to-emitted-set consistency probe over 1 367 369 observations; R7 the cost
number's denominator; R8 a ply-0-only cap scope. Raw lines are quoted in the findings.

---

## VERDICT

# **FALLS.**

The recommendation of **W-K1** does not survive. Three independent grounds, each sufficient:

1. **The field omits a row that dominates W-K1 on the matrix's own stated cost.** Capping
   **ply 0 only** ("W-K0") takes the 99-stone position from completed depth **0** to completed
   depth **2** at every K on the grid, from a **completed** iteration rather than a salvage, while
   capping **3 nodes per search** instead of every safety-net node — and its exclusion set is a
   strict subset of W-K1's. This is D-318's shape exactly (BLOCKING 1).
2. **F2 — the fact that kills every root-uncapped row — is a property of W-K1's chosen placement,
   not of the row.** Truncating before the eval pass makes a safety-net node **11.9x to 32.5x**
   cheaper, and a root-UNCAPPED prefilter seat then completes iterations at 51 and 99 stones where
   the matrix states only a root-inclusive cap can. Grounds 1 and 2 of the recommendation fall
   together (BLOCKING 2).
3. **The cost paragraph's central factual claim is false as measured.** The incumbent's
   completed-depth-0 answer is not "an unsearched answer": at 99 stones it is the exact depth-1
   argmax over **956 of 3 564** completed root children. W-K1 replaces that with 4-8. The
   recommendation's stated ground for distinguishing itself from the W-A that fell is refuted
   (BLOCKING 3).

**The row that should be selected instead.** On the matrix's own grounds the answer is **not** a
substitution but a re-authored field: two viable rows are missing and one of them is measured
better than the recommendation on the axis the matrix says decides everything. If a single row
must be named today, it is **W-K0 — cap ply 0 only** — because it pays F6's debt at every stone
count, its answer comes from a completed iteration rather than a salvage, and its wrong-exclusion
set is a proper subset of W-K1's. But W-K0 inherits BLOCKING 3 in full, so naming it settles the
cost question no better than W-K1 does; what settles it is the prefilter class (BLOCKING 2),
which the field carries only in its degenerate instance and only as an ESTIMATE.

---

## FINDINGS

### BLOCKING 1 — the field omits "cap ply 0 only", and it dominates the recommendation on the matrix's own cost statement

**Claim attacked**, matrix §1 line 175-179 and §4 ground 1:

> "Seven options. The ply scope is part of an option's mechanism rather than a second axis,
> because F7 measures the scope as the term that decides whether an option works at all — a field
> that factored it out would score six rows on a question the measurement has already answered."

and

> "1. **It is the only option in the field that moves the debt.** F7 and §2: W-K2, W-K3, W-CFG and
> W-2R all leave the 99-stone position at completed depth 0 at `movetime 500`; W-K1 reaches 3 at
> K = 8 and 5 at K = 4."

**The attack.** The matrix carries three ply scopes: every ply (W-K1), every ply but 0 (W-K2),
non-PV only (W-K3). The complement of W-K2 — **ply 0 only** — is not in the field, and it is the
scope F7's own diagnosis points at. F7 line 136-140 says the cost is the root's fanout:

> "With the root uncapped, the first iteration still expands ~3 578 ply-1 nodes and **each one
> pays the whole ball's construction and ranking whatever K is.**"

If the root's fanout is what costs, then capping the root alone is what removes it, and every
interior exclusion W-K1 makes is cost the matrix never priced. W-K0's wrong-exclusion set is
`{root cells below rank K}`; W-K1's is that set **plus every interior safety-net node's tail**. So
if W-K0 pays the debt, it dominates W-K1 on the one axis the matrix calls "THE COST OF THE
RECOMMENDATION, STATED AND NOT DISCOUNTED".

**REPRODUCED.** Added `scope 3 => ply == 0` to the instrument's two scope matches
(`crates/pistol-search/src/pvs.rs`, the scratch block) and ran
`crates/pistol-search/tests/zz_redteam_root_only.rs`:

```
cd /home/tom/Projects/pistol-wt-redteam && CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-redteam-target \
  cargo test --release -p pistol-search --test zz_redteam_root_only -- --nocapture --test-threads=1

R8/SPREAD/movetime500/r3/ply0-only/K4  p00 stones= 11 depth=2 prov=completed nodes=238744 ms=500 sn_rows=292 sn_capped=3
R8/SPREAD/movetime500/r3/ply0-only/K4  p01 stones= 21 depth=2 prov=completed nodes=225488 ms=500 sn_rows=165 sn_capped=3
R8/SPREAD/movetime500/r3/ply0-only/K4  p02 stones= 51 depth=2 prov=completed nodes=197263 ms=500 sn_rows= 79 sn_capped=3
R8/SPREAD/movetime500/r3/ply0-only/K4  p03 stones= 99 depth=2 prov=completed nodes=162669 ms=500 sn_rows= 49 sn_capped=3
R8/SPREAD/movetime500/r3/ply0-only/K8  p03 stones= 99 depth=2 prov=completed nodes=130365 ms=500 sn_rows= 69 sn_capped=3
R8/SPREAD/movetime500/r3/ply0-only/K32 p03 stones= 99 depth=1 prov=partial   nodes=  1508 ms=502 sn_rows=160 sn_capped=2
```

At `quiet_radius = 3`, `movetime 500`, completed `depth_turns` at 11/21/51/99 stones:

| row | 11 | 21 | 51 | 99 | safety-net rows CAPPED per search (99 stones) |
|---|---|---|---|---|---|
| W-N (incumbent, F7) | 1 | 0 | 0 | 0 | 0 of 28 |
| W-K2 `except-ply0` K=8 (F7) | 1 | 1 | 0 | 0 | 160 of 161 |
| **W-K0 `ply0-only` K=4** | **2** | **2** | **2** | **2** | **3 of 49** |
| **W-K0 `ply0-only` K=8** | **2** | **2** | **2** | **2** | **3 of 69** |
| W-K1 `every-ply` K=8 (F7) | 4 | 4 | 3 | 3 | 155 of 155 |

The missing row pays F6's debt — *"At 21 stones and above the engine returns a move no completed
iteration produced"* (F6, line 116-118) — at **all four** stone counts, and does it with
`prov=completed`, i.e. the answer comes from a finished iteration rather than from `salvage`.
W-K1 at K=8 and r=3 returns `prov=partial` at three of the four (receipt §A lines 48-51 read
`prov=partial` at p00, p01, p02).

Ground 1's sentence "**W-K2, W-K3, W-CFG and W-2R all leave the 99-stone position at completed
depth 0**" is true of the rows it names and false of the field, because the field is incomplete.

**Why BLOCKING and not MAJOR.** D-318 stopped matrix M4 in part because *"THE FIELD IS STILL
INCOMPLETE AT THE SECOND REVISION, and the missing row dominates the recommended option on the
matrix's own trigger"*. This matrix is at its first revision and the missing row dominates on the
matrix's own explicitly-stated cost. The same disposition applies.

---

### BLOCKING 2 — F2 is a property of where W-K1 puts the truncation, not of the row; the option class that escapes it is in the field once, degenerate, and marked ESTIMATED (D-291)

**Claim attacked**, F2, lines 58-65:

> "**A top-K cap never reduces the cost of a safety-net node; it reduces how many nodes pay it.**
> This is WP-1.5c's own `delta_rank` lesson, restated where it bears on this decision."

and the recommendation's ground 2, lines 426-430:

> "**The reason the others fail is measured and structural, not incidental.** F2: a safety-net
> node's cost is Θ(|ball|) whatever K is, so an option that leaves the root uncapped leaves
> ~|ball| nodes each paying it."

**The attack.** F2 is true of a cap applied AFTER `delta_rank` — which is W-K1's mechanism
("Inside F1's branch, after `delta_rank`, keep the first K", line 220) and also the instrument's
(`artifacts/wp15d_m2_evidence_instrument_v1.txt` PART 2 truncates in `pvs.rs` after
`staged_candidates` has returned). It is **false** of a cap applied before it. `staged.rs:230-233`
builds the whole ball and then hands **all** of it to `delta_rank`, whose cost is one `Eval::delta`
per ball cell (`staged.rs:287-295`). A truncation placed between those two lines removes that term.
The matrix carries exactly one member of that class — W-P — states its cost as **ESTIMATED** and
kills it on a different axis:

> "**Cost shape.** ESTIMATED: saves |ball| − K `Eval::delta` calls per safety-net node, which F2
> makes the larger half of that node's cost … **Not measured**, and marked so: the structural
> objection above is what keeps it out of the recommendation, not the cost."

D-291's clause: *"an estimate that could have been measured in seconds is a finding"*, judged by
whether the measurement was *"AVAILABLE to the session writing the matrix, on the machine it was
standing on, without a build it had not already made"*. The session had the worktree, the
instrument, the fixtures and a compiled `pistol-search`. I measured it in **1.45 s of test time**.

**REPRODUCED (a) — the cost split.** Added `SN_PROX` to the scratch module and a ring-ordered
truncation inside `staged::batched`, BEFORE `delta_rank`, then timed `staged_candidates` directly
(`crates/pistol-search/tests/zz_redteam_m2.rs::r0_cost_split`, 200 reps per cell):

```
R0/COSTSPLIT r3 p00 stones= 11 safety_net=true width= 396 whole_us= 204.7 prox_k8_us=  7.3 ratio=28.02
R0/COSTSPLIT r3 p01 stones= 21 safety_net=true width= 756 whole_us= 434.7 prox_k8_us= 13.4 ratio=32.50
R0/COSTSPLIT r3 p02 stones= 51 safety_net=true width=1836 whole_us=1202.8 prox_k8_us= 43.2 ratio=27.83
R0/COSTSPLIT r3 p03 stones= 99 safety_net=true width=3564 whole_us=2657.9 prox_k8_us=118.9 ratio=22.36
R0/COSTSPLIT r2 p03 stones= 99 safety_net=true width=1782 whole_us=1414.9 prox_k8_us=119.4 ratio=11.85
```

**A safety-net node at 99 stones and r = 3 costs 2 657.9 µs whole and 118.9 µs truncated first —
22.4x.** F2's Θ-form is correct; the sentence the matrix reasons from ("never reduces the cost of
a safety-net node") is not.

**REPRODUCED (b) — a root-UNCAPPED seat then moves F6.**
`crates/pistol-search/tests/zz_redteam_m2.rs::r3_wp_seat`, `movetime 500`, completed `depth_turns`
at 11/21/51/99:

```
R3/SPREAD/movetime500/r3/except-ply0/K8/eval      p02 stones= 51 depth=0 prov=partial
R3/SPREAD/movetime500/r3/except-ply0/K8/eval      p03 stones= 99 depth=0 prov=partial
R3/SPREAD/movetime500/r3/except-ply0/K8/proximity p00 stones= 11 depth=2 prov=partial
R3/SPREAD/movetime500/r3/except-ply0/K8/proximity p01 stones= 21 depth=2 prov=partial
R3/SPREAD/movetime500/r3/except-ply0/K8/proximity p02 stones= 51 depth=1 prov=partial
R3/SPREAD/movetime500/r3/except-ply0/K8/proximity p03 stones= 99 depth=1 prov=partial
R3/SPREAD/movetime500/r3/every-ply/K8/proximity   p03 stones= 99 depth=4 prov=partial
```

| row, r = 3, K = 8 | 11 | 21 | 51 | 99 |
|---|---|---|---|---|
| W-N incumbent | 1 | 0 | 0 | 0 |
| W-K2 (eval-ranked, root uncapped) | 1 | 1 | 0 | 0 |
| **prefilter, root UNCAPPED** | **2** | **2** | **1** | **1** |
| W-K1 (eval-ranked, every ply) | 4 | 4 | 3 | 3 |
| **prefilter, every ply** | **5** | **4** | **4** | **4** |

Two consequences. (i) F7's headline — *"only a ROOT-INCLUSIVE cap moves F6"* — is refuted: a
root-uncapped prefilter takes 51 and 99 stones off completed depth 0. (ii) The prefilter also
beats W-K1 at the same K and the same scope (5/4/4/4 against 4/4/3/3), so the cost term F2 waves
away is worth roughly a whole iteration of depth.

**My own rebuttal, recorded.** The matrix's structural objection to W-P is REAL and my measurement
confirms it: at `every-ply` the proximity seat plays `0,-2/0,-1` at 11, 51 and 99 stones alike
(R3 lines above) — the positional bias with no game meaning the matrix names, exactly. **That does
not save F2.** The degeneracy is a property of *this* key (ring index with a `(q, r)` tie-break),
not of the class; the class is "spend the eval pass on M ≪ |ball| survivors", of which a
non-degenerate member is a proximity or geometry PREFILTER to M followed by `delta_rank` over the
M and a top-K of that — the ranking that decides is still the eval's. The matrix carries the class
once, in its worst instance, and prices it by estimate. That is the D-291 finding and the field
gap together.

---

### BLOCKING 3 — the recommendation's second separating reason is refuted by measurement: the incumbent's root is not unsearched, and W-K1 narrows a root that was working

**Claim attacked**, §4, lines 447-451:

> "MEASURED F6: on this class the incumbent's root completes **no iteration** and answers
> `Provenance::PartialRoot` — the best move found inside an aborted iteration. W-K1 replaces an
> unsearched answer with a searched one; **it does not narrow a root that was working.**"

**The attack.** `Run::salvage`'s own doc (`crates/pistol-search/src/pvs.rs:174-186`) says what a
`PartialRoot` answer is:

> "Sound because a ply-0 promotion only ever happens on a COMPLETED child subtree — `visit`
> returns an aborted child's sentinel before its score can be used — and root beta is infinite, so
> a root promotion is never a fail-high adopting a truncated null-window line: **the line is
> turn-whole and the score exact.**"

`Provenance::Fallback` is this engine's unsearched answer — `search.rs:410-416` scores it as
*"anything else is unsearched and carries the root's static value"*. `PartialRoot` is the exact
argmax over however many root children finished. So the whole force of the sentence rests on a
number the matrix never took: **how many**.

**REPRODUCED.** Added two counters to `pvs.rs` — `ROOT_WIDTH` (the emitted set size at ply 0) and
`ROOT_DONE` (root candidates whose subtree returned without `self.aborted`) — and ran
`crates/pistol-search/tests/zz_redteam_root.rs`:

```
R5/ROOT r3 K0 p01 stones= 21 depth=0 prov=partial root_width= 756 root_children_completed=335 nodes=227504 ms=500
R5/ROOT r3 K0 p02 stones= 51 depth=0 prov=partial root_width=1836 root_children_completed=529 nodes=209838 ms=500
R5/ROOT r3 K0 p03 stones= 99 depth=0 prov=partial root_width=3564 root_children_completed=956 nodes=199063 ms=500
R5/ROOT r3 K4 p03 stones= 99 depth=3 prov=completed root_width=  4 root_children_completed=  0 nodes=  6845 ms=501
R5/ROOT r3 K8 p03 stones= 99 depth=3 prov=completed root_width=  8 root_children_completed=  0 nodes= 21184 ms=500
R5/ROOT r2 K0 p03 stones= 99 depth=0 prov=partial root_width=1782 root_children_completed=641 nodes=200606 ms=500
R5/ROOT r2 K0 p02 stones= 51 depth=0 prov=partial root_width= 918 root_children_completed=404 nodes=198797 ms=500
```

At the matrix's own worst case — 99 stones, `quiet_radius 3`, `movetime 500` — the incumbent's
`PartialRoot` answer is the **exact depth-1 (one whole turn) argmax over 956 of its 3 564 root
candidates**, taken in `delta_rank` order. At 51 stones it is 529 of 1 836; at 21 stones 335 of 756.

Both clauses fail:

- **"an unsearched answer"** — it is a searched answer with an exact score and a turn-whole line,
  over 27 % of the root ball at 99 stones. The engine's actual unsearched answer has a different
  provenance and does not occur here.
- **"it does not narrow a root that was working"** — W-K1 at K = 4 replaces 956 examined root
  candidates with **4**, and those 4 are the top 4 of the same ranking the 956 were a prefix of.
  The trade is 956 candidates at depth 1 against 4 candidates at depth 3. That is a narrowing of
  the root by a factor of 239, and which side of it is stronger is exactly the question D-124 says
  no oracle answers and the SPRT must.

**This is the attack `WPQ_seed.md` §7.2 already refused to let a measurement license**, and the
matrix does not cite that paragraph:

> "Recorded against this WP's own conclusion … the incumbent's root argmax sat inside the first 16
> ordered candidates in 47/47 root iterations at depth 2 … On this corpus a root cap would not have
> changed the move played. **That is not a licence — it is D-124's blindness restated from the
> other side, and it is why the root exemption rests on the argument above and not on that
> measurement.**"

The matrix's cost paragraph is a measurement-based licence of exactly the kind the governing prior
document names and refuses, resting on a fact about the root that measurement contradicts.

---

### BLOCKING 4 — §3 and the matrix's headline cost number are measured at `except-ply0`, the scope of the option the matrix rejects; at the recommended scope both move by a factor of about three

**Claims attacked**, §3 lines 379-391 and §4 lines 452-454:

> "**MEASURED, the book's own openings** (§D of the receipt): `random_openings_v1.txt`, all
> **2 000** openings, `Stop::Nodes(50_000)`, `quiet_radius 2`, **cap every ply but 0** … Openings
> reaching completed depth ≥ 3 go from **121 / 2 000 (6.1 %)** to **682 / 2 000 (34.1 %)**."

> "MEASURED: at K = 32 over the book's 2 000 openings the cap removes **2 425 555 of 9 384 074
> (25.8 %)** of the opponent's live-count-two cells"

**The attack.** Both come from the receipt's section D, and section D is driven with
`scratch::SN_SCOPE.store(1, Relaxed)` — `except-ply0` — in the instrument
(`artifacts/wp15d_m2_evidence_instrument_v1.txt` PART 3, the `// D.` block). The matrix says so
for the histogram and does **not** say so for the 25.8 %, which appears in W-K1's own
"what it can wrongly exclude" bullet and in the recommendation's cost paragraph as a property of
**W-K1**. On this book the root **is** a safety-net row — the receipt's own K=32 line shows
`sn_rows=1214170 sn_capped=1209126`, a gap of 5 044 uncapped rows, i.e. the ply-0 rows — so the
scopes are not interchangeable here the way they are on the corpus.

**REPRODUCED.** `crates/pistol-search/tests/zz_redteam_book.rs`, same 2 000 openings, same budget
and same radius as section D, over both scopes and both K values (25 min wall, single-threaded):

```
R4/BOOK/nodes50000/r2/every-ply/K0 openings=2000 nodes=100352000 sn_rows=1936431 sn_capped=0
    pool_mean=78.16 emit_mean=78.16 opp2=11404333 opp2cut=0       hist=[0, 10, 1869, 113, 8, 0, 0, 0]
R4/BOOK/nodes50000/r2/every-ply/K8 openings=2000 nodes=100352000 sn_rows= 263291 sn_capped=263291
    pool_mean=83.27 emit_mean= 8.00 opp2= 2241129 opp2cut=1377486 hist=[0, 0, 0, 1205, 790, 5, 0, 0]
R4/BOOK/nodes50000/r2/every-ply/K32 openings=2000 nodes=100352000 sn_rows=485009 sn_capped=485009
    pool_mean=77.03 emit_mean=32.00 opp2= 3812720 opp2cut=1086502 hist=[0, 0, 927, 958, 115, 0, 0, 0]
R4/BOOK/nodes50000/r2/except-ply0/K8 openings=2000 nodes=100352000 sn_rows=616627 sn_capped=611121
    pool_mean=76.12 emit_mean= 8.51 opp2= 5181856 opp2cut=2926562 hist=[0, 0, 917, 789, 290, 4, 0, 0]
```

The K=0 line replicates the receipt's `D/BOOK/nodes50000/r2/except-ply0/K0` cell for cell
(`sn_rows=1936431`, `nodes=100352000`, `opp2_cells=11404333`, `depth_hist=[0,10,1869,113,8,0,0,0]`),
which is the control that the two runs are the same instrument. (At K = 0 the cap is inert, so the
scope label on that line carries no meaning and it is directly comparable to the receipt's.)

The full 2x2 over scope and K, all four seats on the same 2 000 openings, the same
`Stop::Nodes(50_000)` and the same `quiet_radius 2`:

| | incumbent | **§3's quoted seat**: `except-ply0`, K=32 | `except-ply0`, K=8 | `every-ply`, K=32 | **`every-ply`, K=8** |
|---|---|---|---|---|---|
| openings at completed depth ≥ 3 | 121 / 2 000 (6.1 %) | 682 / 2 000 (34.1 %) | 1 083 / 2 000 (54.2 %) | **1 073 / 2 000 (53.7 %)** | **2 000 / 2 000 (100 %)** |
| opponent count-two cells removed, matrix's denominator | 0 | 25.8 % | 56.5 % | **28.5 %** (1 086 502 / 3 812 720) | **61.5 %** (1 377 486 / 2 241 129) |
| … against the pool denominator (MAJOR 5) | 0 | 33.0 % | — | **36.2 %** | **79.5 %** |

Two controls, each isolating one substitution. **Hold K = 32 and change only the scope** to the
recommended one: the histogram moves 682 → 1 073 and the exclusion 25.8 % → 28.5 %. **Hold the
scope at `except-ply0` and change only K** to the small end its own grid points at: 682 → 1 083 and
25.8 % → 56.5 %. Apply both, which is what the recommendation actually is: **2 000 / 2 000 and
61.5 %** — 79.5 % against the denominator MAJOR 5 shows is the right one.

The seat §3 quotes is the weakest of the four on the benefit side and the mildest of the four on
the cost side, simultaneously. It is not a conservative reading of the recommendation; it is a
reading of a different option at a K the recommendation's own data argues against.

§3 is headed "WHAT THE GOVERNED RUN WOULD SEE". The governed run would see none of those numbers.
Both errors come from the same substitution, and the cost error is the one that matters, because
the cost paragraph is what the ADR line must record.

---

### MAJOR 5 — the 25.8 % is computed against a denominator that includes cells the cap could not have removed

**Claim attacked**, §1 W-K1 (ii), lines 222-227:

> "MEASURED over the SPRT book's own 2 000 openings at K = 32: 2 425 555 of 9 384 074 such cells
> removed (**25.8 %**)."

**The attack.** The instrument's `opp2` is `threats.live_cells_at_count(opponent, LiveCount::Two)`
— every such cell on the board, ball membership not required
(`artifacts/wp15d_m2_evidence_instrument_v1.txt` PART 2, the `let mut opp2 = Vec::new();` block).
`opp2_cut` counts only those in `dropped`, which is a suffix of `set.cells`. A count-two cell
outside the quiet ball was never a candidate, so its non-emission is not the cap's doing and it
belongs in no denominator the cap is measured against. The matrix's own gloss — *"the one
threat-shaped class a safety-net row can **hold**"* — names the right denominator and then does not
use it.

**REPRODUCED.** Added `SN_OPP2_IN_POOL` (count of `opp2` cells present in `set.cells` before
truncation) and ran `crates/pistol-search/tests/zz_redteam_opp2.rs` over the book's first 300
openings, same budget and radius:

```
R7/OPP2 openings=300 scope=1 K32 opp2_all=1446931 opp2_in_pool=1126669 opp2_cut=371460 cut_over_all=0.2567 cut_over_in_pool=0.3297
R7/OPP2 openings=300 scope=0 K32 opp2_all= 585660 opp2_in_pool= 452906 opp2_cut=163739 cut_over_all=0.2796 cut_over_in_pool=0.3615
R7/OPP2 openings=300 scope=0 K8  opp2_all= 333761 opp2_in_pool= 255436 opp2_cut=203040 cut_over_all=0.6083 cut_over_in_pool=0.7949
```

The first line replicates the matrix's 25.8 % at its own seat (0.2567 on a 300-opening sample).
The correct denominator at that seat gives **33.0 %**; at the recommended scope and K = 32,
**36.2 %**; and at the recommended scope and the K the matrix's own calibration data points at,
K = 8, **79.5 %**.

The compounding matters. The matrix's calibration discussion (open item 1) records that its grid is
monotone toward small K, so the shipped K would be at the small end — where the exclusion rate is
**79.5 %**, not 25.8 %. The cost the matrix states "and does not discount" is quoted at the largest
K on the grid, against the widest available denominator, at the scope of a rejected option.

---

### MAJOR 6 — `quiet_top_k` and `widen_schedule` already exist in every committed config, with the opposite semantics, and the matrix never names either

**Claim attacked**, §4 line 416-418:

> "**W-K1: score-ranked top-K of the quiet ball, at every ply, inside F1's `tier_t.is_empty()`
> branch, behind a gate that is `false` in every committed config.**"

**The attack.** `/usr/bin/grep -c "quiet_top_k\|widen_schedule" docs/experiments/matrix_M2.md`
returns **0**. Meanwhile:

```
$ /usr/bin/grep -rn "quiet_top_k" configs/ | LC_ALL=C sort
configs/bench_wp18c_solver_off.toml:30:quiet_top_k = 16
configs/bench_wp18c_solver_on.toml:30:quiet_top_k = 16
configs/gate_staged_heuristics_v0.toml:28:quiet_top_k = 128
configs/gate_staged_solver_v0.toml:32:quiet_top_k = 16
configs/gate_staged_v0.toml:31:quiet_top_k = 128
configs/instrument_staged_h_v0.toml:30:quiet_top_k = 16
configs/instrument_staged_q_defensive_and_offensive_v0.toml:30:quiet_top_k = 16
configs/instrument_staged_q_defensive_only_v0.toml:29:quiet_top_k = 16
configs/instrument_staged_v0.toml:37:quiet_top_k = 16
configs/instrument_v0.toml:67:quiet_top_k = 16
configs/play_staged_v0.toml:33:quiet_top_k = 16
configs/tactical_staged_v0.toml:36:quiet_top_k = 1024
```

Three separate problems follow.

**(a) There is no off-value.** `crates/pistol-engine/src/validate.rs:94-97` refuses
`quiet_top_k == 0` (*"must be at least 1"*). The schema cannot express "no cap". So "behind a gate
that is `false` in every committed config" is either a THIRD knob layered over two that already
exist — two schema places for one default, against hard rule 1 and against D-423's
"A CLAIM THE DOCUMENT MAKES TWICE IS A DEFECT WAITING" — or it means editing twelve committed
configs, which the matrix does not say and does not price.

**(b) The committed semantics are the opposite direction.** `crates/pistol-search/src/params.rs:47-57`
records that `StagedParams` deliberately omits both keys, *"validated at the config layer
(`pistol-engine`) for schema completeness and go no further"*. `sessions/WP-1.9/2026-08-29-WP19-STOPPED.md`
established what they mean:

> "it is not a cap on what the search already looks at. It is a **quiet tier ADDED beyond Tier T**.
> `docs/experiments/U3_tier_t.md` §10 says it in one sentence … Four other committed places agree,
> including a gate-run test (`wp15b_census.rs`) that computes exactly `TierT + min(quiet, K)`, and
> both configs that 'disable the cut' do so by setting K **above the whole ball**."

`configs/instrument_staged_v0.toml:10-15` — the SPRT seat — says
*"THE CUT BINDS HERE … a seat with the quiet cut disabled would make the SPRT measure nothing
about the prune"*, at `quiet_top_k = 16, widen_schedule = [32]`. So the committed document family
is pre-wired for a cut of a **different shape** to bind in the SPRT seat and to be disabled by a
large K in the gate and tactical seats. Selecting W-K1 either re-purposes those keys — silently
changing what twelve committed headers assert — or strands them. The matrix says nothing about
which, and D-474's own text names *"WP-1.5c's `quiet_top_k` semantics"* as a motivating instance of
exactly this class of unverified mechanism claim.

**(c) `widen_schedule` is cross-validated against `quiet_top_k`.** `validate.rs:104-121` requires
every schedule entry to be strictly greater than `quiet_top_k`. The matrix kills W-W (the schedule)
on F4 and never says what happens to the committed key that only a schedule reads.

**REPRODUCED** — the greps above, run with `/usr/bin/grep` and `LC_ALL=C sort` per CLAUDE.md's
Environment rule, plus `sed -n '86,105p' crates/pistol-engine/src/validate.rs`.

**A related elision.** F1 (lines 38-49) presents a quoted block labelled
`crates/pistol-search/src/staged.rs:222-236`. The real 222-236 contains a six-line comment that the
quote drops, and that comment's last clause is:

> "The same ball `candidate_cells`'s `Staged` arm answers with, uncapped — **`quiet_top_k` is stage
> Q's own knob and this D-scope does not arm stage Q.**"

The matrix quotes the one line range in the tree that names the key, elides the naming, and then
never mentions the key in 475 lines.

---

### MAJOR 7 — `diverged_and_bearing == diverged` is a theorem of the instrument, not a check

**Claim attacked**, §3 lines 402-404:

> "`diverged_and_bearing == diverged == 58`: **every divergence is on a safety-net-bearing
> search**, which is the check that the cap is the whole cause and the measurement is not reading
> noise."

**The attack.** `run()` in the §H driver constructs a **fresh** `Searcher` per call
(`common::staged_searcher(...)`, hence a fresh transposition table), the budget is `Stop::Nodes`
(reproducible, `stop.rs:65-70`), and the cap is a no-op at every node that is not a safety-net row
(the instrument's truncation is inside `if set.used_quiet_safety_net`). So if the uncapped run
visits zero safety-net rows, the capped run walks a bit-identical tree and returns the identical
turn. `diverged ⟹ bearing` therefore **cannot come out false**, whatever the cap does.

`docs/process.md`'s "Criterion and defect class" names this exact shape:

> "A criterion that is a property the named defect class PRESERVES — internal agreement between
> components sharing an input, output shape, plausible magnitude, exit status — passes vacuously
> and is not a criterion; it must be one that defect could falsify."

The equality is internal agreement between two runs sharing every input; it is invariant under the
defect it is offered against. It is quoted as one of four bullets in §3 and reappears in ground 4.

**REPRODUCED — by construction, from the instrument's own text and `stop.rs`.** I did not build a
counterexample, because the point is that no counterexample can exist; that is the finding. The
supporting code is quoted above at `artifacts/wp15d_m2_evidence_instrument_v1.txt` PART 4 (`fn run`,
which calls `common::staged_searcher` inside the function body) and
`crates/pistol-search/src/stop.rs:65-70`.

---

### MAJOR 8 — ground 5 slides from "not tactical at this node" to "not tactical"

**Claim attacked**, §4 ground 5, lines 436-438:

> "5. **What it prunes is provably not tactical.** F1: both tiers are empty inside the branch, so
> no forced defence, no win-in-one and no cover cell can be cut."

**The attack.** F1's guard proves exactly that the cut cell is not in Tier F or Tier T **at the
node being generated**. It proves nothing about the subtree that cell roots. A cut cell is a MOVE
that is never played, so every continuation below it — including a forced win three turns later —
is never searched. The matrix's own W-K1 (ii) concedes the first step of this (*"a count-two window
needs four more stones, two whole turns"*), which is inconsistent with ground 5's flat "provably
not tactical". D-124's whole content is that a candidate-set narrowing at the tail is invisible to
the value oracle precisely because the pruned move is the one the ordering rated worst — not
because it is harmless.

The correct sentence is "what it prunes carries no forced obligation at the node where it is
pruned", which is a much weaker claim and does not support a ground.

**REPRODUCED** — reading. `staged.rs:209-213` and `staged.rs:222-236` establish the node-local
claim; nothing in the matrix or the tree establishes the subtree claim.

---

### MAJOR 9 — the deciding channel is a maximand of degeneracy, and the matrix uses it to decide anyway

**Claim attacked**, §4 open item 1, lines 458-465, read against grounds 1 and 2:

> "F7's advisory table is monotone in the wrong direction for a naive rule (smaller K is always
> deeper), so a rule that maximises depth selects the grid minimum, which is the mirror image of the
> defect that failed WP-1.5c."

**The attack.** The matrix states this against its own calibration and then does not apply it to
its own field scoring. Grounds 1 and 2 discriminate the field on completed depth on `spread_v1`;
that quantity is monotonically improved by narrowing the search, without bound. At K = 1 the root
would hold one candidate and "complete" arbitrary depth while playing a fixed cell. So the channel
cannot rank options that differ in how hard they narrow — it ranks them by how hard they narrow.
My own R8 and R3 show the ranking inverting under both a scope change and a ranking-key change,
which is what a degenerate channel does.

What the channel CAN carry is F6's binary defect — *"the engine returns a move no completed
iteration produced"* — which is not a maximand. Read that way, BLOCKING 1 and BLOCKING 2 both show
the defect fixed by rows the field does not contain, so ground 1 falls under either reading.

**REPRODUCED** — the R8 and R3 tables above, plus the matrix's own F7 grid, which is weakly
monotone decreasing in K at all four stone counts.

---

### MAJOR 10 — the `root_restrict` interaction is listed as an open choice; the sibling package classified the same interaction as a correctness hole and fixed it the other way

**Claim attacked**, §4, lines 466-467:

> "**The `root_restrict` interaction (F3).** W-K1 caps ply 0, so the fail-open restores the capped
> set. The design must say what happens there and pin it."

**The attack.** `sessions/WP-1.9/2026-08-29-WP19-STOPPED.md` records the same interaction under a
different name:

> "**It had a correctness hole at the root.** `pvs.rs` restricts the root to a solver proof's zone
> and *fails open* if that would empty the set. Nothing is forced on a batched row, so … the root
> searches `topK(quiet) ∩ zone` **with every Tier-T cell removed** … **Revision 2 excludes ply 0
> under `root_restrict`; the re-review confirmed that fix is right and sufficient.**"

Under W-K1 the mechanism is the same one: `set.forced == 0` on a safety-net row, so `forced_intact`
at `pvs.rs:337-340` is vacuously true, the `retain` runs over the CAPPED set, and the fail-open at
`pvs.rs:343-345` restores the capped set rather than the ball. The comment at `pvs.rs:318-327` that
documents the mechanism would then be false as written ("FAILS OPEN to the unrestricted set").
`configs/gate_staged_solver_v0.toml` and `configs/bench_wp18c_solver_on.toml` arm the solver, so
this is not a hypothetical seat.

D-424: *"a finding that names a way the code can produce a wrong answer is never overruled, only
fixed"*. The immediate predecessor package named this one and fixed it by excluding ply 0 — which
is incompatible with "cap at every ply" wherever the solver is armed. Presenting it as one of three
open **choices**, with no citation of the precedent, understates it.

**Partly NOT REPRODUCED.** I did not build a solver-armed seat that exhibits a narrowed root: the
matrix's whole receipt and all of my re-runs use `solver: None`, and constructing a proven-loss root
zone is a larger build than this review's budget. The code path is quoted and the mechanism is
plain; the *magnitude* is unmeasured on both sides.

---

### MINOR 11 — F6 is the one MEASURED cell with no receipt in the named artifact; it reproduces

**Claim attacked**, F6, lines 111-118: *"MEASURED this session at `fbc8e62` through the shipped
`target/release/pistol` and `tools/bench_block.sh` … completed `depth_turns` **1 / 0 / 0 / 0** …
under `configs/play_v0.toml` (radius 3) **and** under `configs/play_staged_v0.toml`"*, against the
matrix's header at line 19-21: *"The instrument for every MEASURED cell taken this session is
`artifacts/wp15d_m2_evidence_v1.txt`"*.

`/usr/bin/grep -n "bench_block\|play_v0\|play_staged" artifacts/wp15d_m2_evidence_v1.txt` returns
nothing. F6's stated instrument produced no committed receipt, and its `play_v0.toml` half — a
`Radius` policy, not staged at all — has no in-process counterpart anywhere in the receipt either.

**REPRODUCED.** Second clean worktree at `fbc8e62`, `cargo build --release --workspace --locked`:

```
=== configs/play_v0.toml ===
entry1: info totals depth_turns 1 ... time 499 ... bestmove -2,0/-1,0
entry2: info totals depth_turns 0 ... time 499 ... bestmove 6,0/7,0
entry3: info totals depth_turns 0 ... time 499 ... bestmove -2,0/-1,0
entry4: info totals depth_turns 0 ... time 499 ... bestmove -2,0/-1,0
=== configs/play_staged_v0.toml ===
entry1: info totals depth_turns 1 ... time 500 ... bestmove -2,2/-1,1
entry2: info totals depth_turns 0 ... time 499 ... bestmove 6,0/7,0
entry3: info totals depth_turns 0 ... time 499 ... bestmove -2,0/-1,0
entry4: info totals depth_turns 0 ... time 499 ... bestmove -2,0/-1,0
```

1 / 0 / 0 / 0 under both, exactly as stated. The number is right; only its provenance is missing.
Worth recording because F6 is the fact the whole WP exists to move.

---

### MINOR 12 — ground 1's "5 at K = 4" is F8's number quoted inside an F7 sentence

> "W-K2, W-K3, W-CFG and W-2R all leave the 99-stone position at completed depth 0 at
> `movetime 500`; W-K1 reaches 3 at K = 8 and **5 at K = 4**."

F7's `every ply` column at K = 4 reads `5 / 5 / 4 / 3`: at 99 stones and `movetime 500` it is **3**,
not 5. 5 is F8's `Stop::Nodes(50_000)` figure. REPRODUCED from the receipt: `A/SPREAD/movetime500/
r3/every-ply/K4 p03 … depth=3` against `B/SPREAD/nodes50000/r2/every-ply/K4 p03 … depth=5`.

---

### MINOR 13 — §2's r = 3 mean width is the wrong section's number

> "MEASURED mean widths at 99 stones: 596.0 at r = 1, 1 789.2 at r = 2, **3 578.5** at r = 3"

§2's own table is section E; section E's r3/K0/p03 line reads `pool_mean=3579.25`. 3 578.4x is
section A's `except-ply0` value. REPRODUCED: receipt line 331 against lines 71/75.

---

### MINOR 14 — §2's "every capped cell beats every uncapped cell" is false as a strict claim

> "Every capped cell in the table beats every uncapped cell at the same radius."

Six of the 36 comparisons in §2's own table are ties, not wins: r = 1 at 51 stones (1 vs 1);
r = 2 at 51 (1 vs 1) and 99 (0 vs 0); r = 3 at 11 (1 vs 1), 51 (0 vs 0) and 99 (0 vs 0), all in the
`every ply but 0` column. "At least as good, and strictly better in 30 of 36" is what the table
shows. REPRODUCED by reading the matrix's own table.

---

### MINOR 15 — W-K3's width figure is W-K2's, and the mechanism it is attributed to is not what produced it

> "MEASURED, `emit_mean` runs 4.40 to **26.39** at K = 4 rather than 4.00."

26.39 is `A/SPREAD/movetime500/r3/except-ply0/K4 p03`; the `non-pv-only` seat's maximum is
**25.98** (receipt line 95). More substantively, the excess over K is arithmetic from the handful
of EXEMPT rows, not from instability at capped ones: at `non-pv-only/K4 p03`, `sn_rows=162`,
`sn_capped=161`, so one uncapped row of ~3 578 among 161 rows of exactly 4 gives
`(3578 + 161×4)/162 = 26.1`, matching. Capped rows emit exactly K — 161 of the 162 rows in the cell quoted. The stated
conclusion — *"its effective width is a function of the ordering rather than of K"* — overstates
what the number shows, even though the underlying PVS-re-search mechanism is real (see NOT
REPRODUCED 3 below, where I measure it directly at 502 occurrences on the book).

---

### MINOR 16 — one citation has drifted

`pvs.rs:503-507` is cited for *"a scan landing inside the window is re-run at the full window"*.
At HEAD, 503-507 is the D-74 comment about the aborted scan; the re-search is at 511-513
(`if scan > alpha && scan < beta { full(self) }`). Every other `file:line` in the matrix that I
checked resolves exactly (see coverage below).

---

### MINOR 17 — F9's inertness is asserted of W-K1 and measured of W-K2; it does transfer, and the matrix does not say why

Ground 3 reads F9 as a property of W-K1. F9's header says `cap at every ply but 0`. The inference
is sound but unstated, and I verified it rather than assume it.

**REPRODUCED** (`zz_redteam_m2.rs::r2_corpus_every_ply`):

```
R2/CORPUS/nodes50000/r2/every-ply/K8    sum_nodes=1104026 sum_ms=4780 uncapped_rows=0 depths=3 2 3 3 3 3 2 3 3 2 3 3 4 1 2 3 1 3 3 2 3 2 2 3
R2/CORPUS/nodes50000/r2/every-ply/K32   sum_nodes=1104026 sum_ms=4817 uncapped_rows=0 depths=3 2 3 3 3 3 2 3 3 2 3 3 4 1 2 3 1 3 3 2 3 2 2 3
R2/CORPUS/nodes50000/r2/except-ply0/K8  sum_nodes=1104026 sum_ms=4782 uncapped_rows=0 depths=(identical)
R2/CORPUS/nodes50000/r2/except-ply0/K32 sum_nodes=1104026 sum_ms=4789 uncapped_rows=0 depths=(identical)
```

`uncapped_rows = sn_rows − sn_capped = 0` at the `every-ply` seats confirms the reason: on this
corpus no safety-net row occurs at ply 0 at all, so the two scopes walk the same tree. F9's
conclusion holds for W-K1, on evidence the matrix does not present.

---

### MINOR 18 — the instrument charges the capped seats for work the shipped engine would not do

The instrument computes `threats.live_cells_at_count(...)` on **every** safety-net row in every
seat, and on capped rows adds an `opp2.iter().filter(|c| dropped.contains(c))` scan that is
O(|opp2| · |dropped|) — inside the timed region of the `movetime 500` sections. Capped seats visit
more safety-net rows and carry larger `dropped`, so the instrumentation penalises them. **The bias
is conservative for the recommendation** and I record it as an observation rather than a defect.
Magnitude at the worst cell (99 stones, K = 4, r = 3): 172 rows × ~5.9 count-two cells × ~3 595
dropped ≈ 3.6 M comparisons in 500 ms — single-digit milliseconds, not decisive.

---

### MAJOR 19 — §H measures divergence on a distribution no paired SPRT walks, and the matrix's own sensitivity claim inherits that

*(Graded MAJOR; numbered last because it was written after the block above.)*

**Claim attacked**, §3 lines 392-410:

> "**MEASURED, on trajectories the governed run itself would walk** (§H): 25 games from the book's
> first 25 openings, turn cap 40, **both sides the UNCAPPED engine** … and at every turn the capped
> engine (K = 8, every ply) asked the same question from the same position … **58 searches (9.19 %)
> would have been answered with a DIFFERENT turn.** … **So the SPRT is not predictably
> insensitive.**"

**The attack.** The sample is the incumbent's self-play trajectory. A paired SPRT walks trajectories
on which the capped engine's own moves are played — and by §H's own number the capped engine
answers differently on 9.19 % of searches, so the two trajectories separate at the first divergence
and never reconverge. Everything §H measures after that first divergence is measured on positions a
governed game would not reach. Three specific biases follow, and none is in the direction the
matrix needs:

- **The safety-net RATE is the incumbent's.** 19.02 % of searches are safety-net-bearing *on the
  uncapped engine's trajectory*. A capped engine plays narrower, faster, deeper moves; whether it
  steers toward or away from Tier-T-empty positions is unmeasured and is exactly what would set
  the SPRT's exposure to this change.
- **The game lengths are the incumbent's.** `decided_early=16` of 25 — nearly two thirds of the
  sample ends before the turn cap, under the incumbent's own play, so the position distribution is
  weighted by how the incumbent finishes games.
- **The divergence rate is a first-move rate, not a game-outcome rate.** "Would have answered with
  a different turn" bounds nothing about how often the two engines' *games* differ, which is what
  an SPRT reads.

The matrix guards one flank of this — *"That is a statement about SENSITIVITY and not about
direction: whether those 58 divergences are better turns is the SPRT's to say"* — and that guard is
correct and well drawn. It does not cover the distribution, which is a separate objection: §H's
sentence is "trajectories the governed run itself would walk", and they are trajectories the
governed run's *incumbent seat* would walk if it played itself.

**NOT REPRODUCED — and deliberately so.** The measurement that would settle it is 25 self-play
games with the CAPPED engine on both sides, counting the safety-net rate and the divergence rate on
*that* trajectory, then reporting both. That is a ~15-minute run and it is the run §3 owes. I did
not take it because the finding is that the matrix drew a conclusion about the governed run's
sample from the incumbent's sample, and taking the second sample here would supply evidence the
design should register rather than inherit from a red team. The direction of the bias is not
predictable from the armchair, which is the reason it needs measuring rather than arguing.

---

## ATTACKS THAT DID NOT REPRODUCE

These are recorded because they are the record, and because two of them are the ones the dispatch
named.

**NOT REPRODUCED 1 — `tt_bytes = 1<<20` versus the committed 268 435 456.** The hypothesis was
that a 32 768-entry table is not neutral between a seat that visits 4.28 M nodes and one that visits
50 176, and that the small table therefore favours the cap. It does not. Every seat is
**byte-identical** between the two table sizes — same completed depth, same node count, same best
move, same counters:

```
R1/SPREAD/nodes50000/r2/tt1MiB/every-ply/K0    p03 stones= 99 depth=1 prov=completed nodes=4283392 ms=10609 best=-2,0/-1,0
R1/SPREAD/nodes50000/r2/tt256MiB/every-ply/K0  p03 stones= 99 depth=1 prov=completed nodes=4283392 ms=10594 best=-2,0/-1,0
R1/SPREAD/nodes50000/r2/tt1MiB/every-ply/K4    p03 stones= 99 depth=5 prov=completed nodes=  50176 ms=  898 best=-2,2/-1,1
R1/SPREAD/nodes50000/r2/tt256MiB/every-ply/K4  p03 stones= 99 depth=5 prov=completed nodes=  50176 ms=  893 best=-2,2/-1,1
R1/SPREAD/nodes50000/r2/tt1MiB/except-ply0/K4  p03 stones= 99 depth=1 prov=completed nodes=  50176 ms=16063 best=-2,0/-1,0
R1/SPREAD/nodes50000/r2/tt256MiB/except-ply0/K4 p03 stones=99 depth=1 prov=completed nodes=  50176 ms=16006 best=-2,0/-1,0
```

The sizes really do differ, so this is not a wiring accident:

```
TTCHECK small_buckets=8192 small_bytes=786432 big_buckets=2097152 big_bytes=201326592
```

**The attack fails and the matrix is right to have taken the receipt at `SMALL_TT`.** This run also
replicates F8 cell for cell (58 368 / 201 728 / 1 149 952 / 4 283 392 nodes; 5/4/5/5 at K = 4;
2/2/2/1 at `except-ply0` K = 4; 10 609 ms against the matrix's 10 648; 16 063 ms against 16 441).

**NOT REPRODUCED 2 — the `Stop::Deadline` non-reproducibility.** `stop.rs:56-60` says a Deadline is
*"**Not** reproducible: the engine refuses to build one of these in instrument mode"*, and F7 and §2
are taken under it with n = 1 and no variance statement. But the receipt itself contains an
unclaimed replication: sections A and E measure the same four seats at r = 3, in two separate test
binaries and two separate `cargo test` invocations, and the completed-depth grid agrees in **16 of
16 cells** (`A/…/r3/every-ply/K0|K8|K32` and `A/…/r3/except-ply0/K8` against
`E/…/r3/scope0/K0|K8|K32` and `E/…/r3/scope1/K8`). My own R3 `eval` seats are a third independent
run and agree again (r3 `except-ply0` K = 8 → 1/1/0/0; r2 → 1/1/0 at p01-p03). Node counts vary by
up to ~7 % across the three runs; **completed depth does not vary at all.** The channel the matrix
decides on is stable under the stop kind it is taken with. The matrix should say so — it has the
receipt for it — but the attack does not stand.

**NOT REPRODUCED 3 — an unsound transposition bound reaching a node that searched a superset, under
W-K1.** `WPQ_seed.md` §7.1(3) names the class: *"A PV node that truncates stores `Bound::Exact`,
which the probe consumes unconditionally at every later non-PV hit. An exact score over a SUBSET is
a lower bound only."* The matrix never mentions it, and dismisses the seed's store rule as covering
*"a case that F4 says does not arise on this recursion"* — but F4 is about widening triggers, and
W-K1 truncates unconditionally, so F4 cannot be the reason.

Traced in code: the store at `pvs.rs:449-467` ranges over `cells`, i.e. the capped set; the probe at
`pvs.rs:245-256` takes a cutoff on `!is_pv` alone. The bound is therefore sound for a later visitor
**iff the same key always emits the same set**. Under W-K1 it does: `staged_candidates` reads
`is_pv` at exactly one place, `staged.rs:148-152`, and that branch is `Cover::Impossible`, not the
`NothingToBlock` arm the safety net lives under; `promote_table_move` (`staged.rs:86-96`) rotates
and never inserts, so it cannot re-add a truncated cell; `root_restrict` changes membership but only
at ply 0, whose key is unique because the stone count is a function of the key.

Measured rather than assumed. `zz_redteam_tt2.rs` hashes the emitted set membership at the point
`visit` will store over it — after the cap, after the table-move promotion, after the root
restriction — and compares it against every previous emission for the same key, with **all three
ordering heuristics ON** so a promotion that changed membership would show:

```
R6/SETFN fixture=spread_v1.txt          positions= 4 scope=0 K8  observations= 41417 disagreements=0
R6/SETFN fixture=spread_v1.txt          positions= 4 scope=0 K32 observations= 21363 disagreements=0
R6/SETFN fixture=bench_positions_v1.txt positions=24 scope=0 K8  observations=129098 disagreements=0
R6/SETFN fixture=bench_positions_v1.txt positions=24 scope=0 K32 observations=129172 disagreements=0
R6/SETFN fixture=random_openings_v1.txt positions=60 scope=0 K8  observations=625743 disagreements=0
R6/SETFN fixture=random_openings_v1.txt positions=60 scope=0 K32 observations=420576 disagreements=0
R6/SETFN fixture=spread_v1.txt          positions= 4 scope=2 K8  observations= 46500 disagreements=31
R6/SETFN fixture=bench_positions_v1.txt positions=24 scope=2 K8  observations=129779 disagreements=1
R6/SETFN fixture=random_openings_v1.txt positions=60 scope=2 K8  observations=464606 disagreements=502
```

**1 367 369 observations, 0 disagreements at W-K1's scope.** The capped tree is a well-defined game
tree and every stored bound ranges over the set any later visitor would search. **The attack fails
for W-K1, and W-K1 is the only scope in the field for which it fails** — `scope=2` (W-K3) produces
534 disagreements, which is the seed's poisoned-entry class arriving exactly as §7.1(3) describes:
a PV visit searches the full ball, a non-PV visit of the same key would search the top-K, and
`Bound::Lower` from the superset is not a valid lower bound for the subset. The matrix kills W-K3 on
F7 and on width instability and never names the defect that actually distinguishes it. That is a
gap in the matrix's reasoning, but it is a gap in the recommendation's **favour**, and I record it
as such rather than as a finding against it.

**One residual, out of this WP's scope.** `quiescence::extend` (`quiescence.rs:122-138`) stores a
`Record` with `depth_plies: 1` over a threat-only cell set, and `visit`'s probe does not test
`from_quiescence`. With `q_depth_turns = 0` — every seat in the receipt and every one of my re-runs
— `extend` returns before the store, so nothing is written. Under a config with
`q_depth_turns > 0` the class exists and it pre-dates this decision; the safety net is explicitly
out of quiescence's reach (`quiescence.rs:26-28` names `within_radius` a *forbidden* generator), so
W-K1 neither creates nor worsens it.

---

## EVERY FALLEN REASON, AND THE STRONGEST SURVIVING ATTACK

**Fallen.**

| Matrix claim | Status after this review |
|---|---|
| Ground 1: "the only option in the field that moves the debt" | **FALLEN** — B1 (ply-0-only) and B2 (prefilter, root uncapped) both move it |
| Ground 2: "the reason the others fail is measured and structural" | **FALLEN** — B2; the reason is W-K1's own truncation placement |
| Ground 3: "inert where the class is rare" | **STANDS**, on evidence the matrix does not present (m17) |
| Ground 4: "visible to the SPRT" | **STANDS in substance** (§H is at the recommended scope and K), its cited corroboration is a tautology (M7), §3's headline is at the wrong scope (B4), and its sample is the incumbent's own trajectory (M19) |
| Ground 5: "what it prunes is provably not tactical" | **FALLEN** — M8; F1 proves a node-local claim only |
| Cost para. bullet 1: "W-A capped … where Tier F's wins live; W-K1 caps a row where both tiers are empty" | **STANDS** — verified at `staged.rs:209-236` and both call sites |
| Cost para. bullet 2: "replaces an unsearched answer with a searched one; does not narrow a root that was working" | **FALLEN** — B3; 956 of 3 564 root children completed |
| Cost para. bullet 3: "25.8 % of the opponent's live-count-two cells" | **FALLEN as stated** — B4 and M5; 79.5 % at the recommended scope and the K the calibration points at |
| F2's operative sentence | **FALLEN** — B2 |
| F7's "only a ROOT-INCLUSIVE cap moves F6" | **FALLEN** — B1 and B2 |
| F4, F5, F8, F9, §2's tables, §H's counters | **REPRODUCE** cell for cell |

**THE STRONGEST SURVIVING ATTACK, for the ADR line.**

> **BLOCKING 3.** The recommendation's ground for distinguishing itself from the W-A that fell is
> that the incumbent's root "completes no iteration" and returns "an unsearched answer", so W-K1
> "does not narrow a root that was working". MEASURED at the matrix's own worst case — 99 stones,
> `quiet_radius 3`, `movetime 500` — that root completes **956 of its 3 564** candidates' subtrees
> to full depth 1 and returns their exact argmax; `Provenance::PartialRoot` is a searched answer
> with a turn-whole line and an exact score, and the engine's unsearched answer is
> `Provenance::Fallback`, which does not occur here. W-K1 replaces 956 examined root candidates
> with K = 4. It narrows a root that was working, by a factor of 239, and `WPQ_seed.md` §7.2 has
> already refused to let a measurement of this kind license a root cap.

I chose this one over BLOCKING 1 and 2 after trying to rebut it and failing. It is the only finding
that is not repaired by completing the field: adding W-K0 leaves it standing unchanged, because
W-K0 caps the same root. Only the prefilter class (BLOCKING 2) escapes it, and only in its
root-uncapped scope — which is the option the matrix's F2 was written to eliminate.

---

## WHAT I CHECKED AND FOUND SOUND

- **The digests.** Both artifacts match their stated `sha256`.
- **F1's guard argument.** `batched` is reached only from `Cover::NothingToBlock` and
  `Cover::Impossible` (`staged.rs:139-153`), both under `can_win_this_turn(...) == None`, and the
  safety net fires only when `tier_t_union` is empty. The doc at `staged.rs:209-213` is accurate and
  `out.forced` is 0. Nothing forced, no win-in-one and no cover cell can be cut **at the node**.
  The matrix is right that this is a guard and not a construction.
- **F4's structural facts, at HEAD.** `pvs.rs:153` opens the root at `(-INFINITY, INFINITY, 0)`;
  `pvs.rs:368` starts `best_score` at `-INFINITY`; `pvs.rs:498/500` open a non-PV child at
  `(alpha, alpha+1)`; `pvs.rs:427-429` is the `won || alpha >= beta` break. All four resolve
  exactly. F4's conclusion — a widening trigger cannot bind on this recursion — follows, and W-W's
  kill is sound.
- **F7, F8, F9, §2 and §H reproduce from the receipt**, cell for cell. I checked all 24 cells of
  F7, all 20 of F8, all 3 rows of F9 (including re-summing the 24 per-position wall times to
  4 825 / 4 836 / 4 801 ms and re-summing the node totals to 1 104 026), all 12 cells of §2, the
  three mean widths at r = 1 and r = 2, and every number in §H. One arithmetic slip (m13) and two
  misquotations (m12, m15) are all I found.
- **F5's census cells** are attributed to `artifacts/wp19_design_census_v1.txt`, a different
  package's receipt, and are marked as such. Not re-derived here.
- **Open item 1 is handled correctly.** It defers K to the design, names direction and the treatment
  of undefined positions as things that must be registered, and cites WP-1.5c's re-review as the
  checklist — which is exactly BLOCKING N1 (a) and (b) of `sessions/WP-1.9/wp19_design_REVIEW_rev2.md`.
  It also states the trap in its own data (monotone toward the grid minimum) rather than hiding it.
  This is the best-executed paragraph in the document.
- **The scope statement holds for quiescence.** `quiescence.rs:26-28` names `staged_candidates`'s
  quiet ball a forbidden generator; the quiescence path never reaches the safety net, so the cap
  cannot leak there.
- **Rule-4 determinism.** The mechanism reads no clock, no hash order and no thread; `delta_rank`
  is a stable sort by score over an ascending-coordinate input (`staged.rs:283-295`), so the
  truncation boundary is deterministic. Nothing in W-K1 threatens `tools/determinism.sh`.
- **The matrix's honesty about what it does not settle.** Three open items are named as open, the
  refuting measurement (F7) is placed before the field rather than after it, and the document says
  in its own preamble that the arithmetic it would otherwise have been built on was refuted. That
  posture is why the findings above are findings about scope, denominators and a missing row rather
  than about concealment.

---

*DECISION-RED-TEAM report on `docs/experiments/matrix_M2.md` at `5b2c972`. Nothing here selects an
option. The re-run worktrees are `/home/tom/Projects/pistol-wt-redteam` (instrumented) and
`/home/tom/Projects/pistol-wt-clean` (shipped binary), both detached at `fbc8e62`, both outside the
live tree; their scratch drivers are `crates/pistol-search/tests/zz_redteam_*.rs` and are named in
each finding.*
