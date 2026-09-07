# `hex_threat_enum_v1.md` revision 4 — round 4 of four, CONFIRMATION BY BEHAVIOUR

## 0 Header

**Named revision reviewed**: `c7a3ee1aa9910388107c05e2ec21099b42070674` (`dev`),
subject `docs/experiments/hex_threat_enum_v1.md` revision 4.

**Match with HEAD**: **NO — HEAD MOVED WHILE THIS REVIEW RAN.** At dispatch
`git rev-parse HEAD` was `c7a3ee1`; at submission it is
`90908cf9d59fcd6820318ed61a254f3b8668d52f`:

```
$ git log --oneline c7a3ee1..HEAD
90908cf docs(matrix): revision five adds the second missing row, compares in one currency,
        and lands a checker for the digest class that recurred three times

$ git diff --stat c7a3ee1 HEAD -- docs/experiments/hex_threat_enum_v1.md tools/hex_enum/
 docs/experiments/hex_threat_enum_v1.md |  6 ++++--
 tools/hex_enum/census.py               | 19 ++++++++++++++-----
 tools/hex_enum/report.py               |  2 +-
```

**That commit touches the SUBJECT and BOTH halves of the NAMED INSTRUMENT.**
`docs/process.md`'s *"Instrument governing revision"* clause — which §6 of the
memo itself cites — says a change to `census.py` or `report.py` reopens this
memo's review, so `90908cf` is a revision this report does not govern and which
has had no review of its own.

**Everything below is adjudicated at `c7a3ee1`.** I then re-checked every finding
against HEAD: the memo's change is two digest citations (`n6_mutants` and
`census`), the instrument's is a new `--null-replicates` argument, and **no
finding in this report is closed by either.** Line numbers at HEAD are shifted by
+2 from `:782` onward. The in-flight `--null-replicates` work is noted where it
bears (NEW-2); it does not reach the finding that decides this round. NEW-10 in
particular still lands at HEAD: `git grep -c "census_r4" HEAD --
docs/experiments/hex_threat_enum_v1.md` returns no hits.

**Reviewer independence.** I am not the author and not any prior reviewer. Every
number below comes from an instrument I wrote for this review, or from the
committed receipts read directly. I did not copy a reproducer from any of the
three prior reports; where my number and a report's number agree, that is a
fourth independent implementation agreeing, not a transcription. D-691 binds:
nothing below checks whether a sentence is present — every entry ran the defect
against the fix.

### 0.1 My instruments

All under `/tmp/claude-1000/…/scratchpad/r4/` (ephemeral — the numbers are
reproduced in full below because the paths will not survive; cf. the standing
memory note on scratch-path finding IDs).

| instrument | what it is | independence |
|---|---|---|
| `mytuple.py` | my own §5 tuple, ladder, and an exhaustive set-partition enumeration of the clip space | shares no line with `tools/hex_enum/` |
| `walk.py` / `walkcode.py` | my own corpus walk: own record parse, own R6 quiet filter, own line projection, own code extraction; emits per-POSITION histograms keyed by join cell or by raw code, with the game id | shares no line with `census.py` |
| `perm.py` / `perm2.py` | **cluster-level label permutation** — the same partitions held fixed, the LABEL permuted at position and at game level | the matched null the memo never built |
| `nested.py` | **resolution-matched null for the nested test** — the class table permuted WITHIN each count stratum, so the null join has the join's own cell count and per-cell code multiplicities | the memo's own stated matching notion, applied to the nested statistic |
| `tmerge.py` | my own exact `DEF-T` minimum hitting set over `DEF-PLAN`'s family, all `3^10` patterns | independent of §5.5's arithmetic |
| `mut.sh` + `tree/` | my own mutation driver over a scratch copy of `tools/`; the live tree was never mutated | 13 mutants, 6 of them at code no prior round tested |

**Cross-validation of my walk against the shipped one, before any attack.** My
walker, written from the memo alone, reproduces the census's own totals exactly
at every length: `45 271` positions kept; window-unit `n` = **8 241 249 /
10 613 918 / 12 980 519** at `L = 7 / 9 / 11`; and at `L = 7` over the first
3 000 positions, **530 785** window observations — the confirmation round's own
pilot figure. Every `ω²` I report for a partition the census also reports agrees
with the receipt to six decimals. The instruments below are therefore
disagreeing with the memo's *inference*, not with its arithmetic.

### 0.2 Receipts

All four verify.

```
$ (cd artifacts/wp22_phase2a/census    && sha256sum -c RECEIPT_census.sha256)     -> 8 of 8 OK
$ (cd artifacts/wp22_phase2a/census_r3 && sha256sum -c RECEIPT_census_r3.sha256)  -> 8 of 8 OK
$ (cd artifacts/wp22_phase2a/census_r4 && sha256sum -c RECEIPT_census_r4.sha256)  -> 8 of 8 OK
$ (cd artifacts/wp22_phase2a/n6_mutants && sha256sum -c RECEIPT_n6_mutants.sha256)-> 4 of 4 OK

$ sha256sum artifacts/wp22_phase2a/census_r3/RECEIPT_census_r3.sha256
9bb8fc6f2e0500bcf10f48e493dd9e3615897d935b73b63d8a8f47bf179e9c39   <- §7's digest, exact
$ sha256sum artifacts/wp22_phase2a/census_r4/RECEIPT_census_r4.sha256
aa88c298ac422a87df6505afcc33fcb1e2cb156783e81ec525b13dc4681bbdd1   <- CITED NOWHERE (NEW-10)
```

**`census_r4` is the run every number in §7.4b comes from and the memo never
names it.** §7 cites `census_r3` and its digest; §7.4b's nested table exists only
in `census_r4`. D-483 wants the run's artifact cited by digest, and the round-4
run is cited by neither path nor digest.

---

## 1 Confirmation table — every finding from all three prior reports

Severities are the originating report's. My verdict column is what MY attack
found, not what the memo claims.

### 1.1 Round 1 (`hex_threat_enum_v1_REVIEW.md`)

| ID | defect | MY attack | output | verdict |
|---|---|---|---|---|
| **B-1** BLOCKING | stabilisation length is a definition presented as a measurement | `mytuple.py` at `L = 7, 9, 11, 13`; read §6.6/§7.2's tagging | `k = 25 / 98 / 357 / 357`; §6.6 says *"derived, not measured"*, §7.2 says *"CONFIRMED, not measured"* | **CLOSED** (residue at P-6c, below) |
| **B-2** BLOCKING | even `L` under-specified | `python3 tools/hex_enum/hexenum.py 8` (read-only); my own `pats()` refuses even `L` | `ValueError: length 8 is even and a cell-centred class has no centre there` | **CLOSED** |
| **B-3** BLOCKING | T1's clip lands on the imported 816 | `mytuple.py 11` full clip enumeration | shipped T1 `k = 36`, `C(38,3) = 8 436` — off 816 | **CLOSED at the rung**; the enumeration around it is **P-2 / NEW-5, STILL LANDS** |
| **M-1** MAJOR | the `t = 1` / `t = 2` merge | `tmerge.py`, my own exact hitting set over all `3^10` | **27 of 335 classes, 5 348 of 57 996 patterns** — §5.5 exact | **CLOSED** |
| **M-2** MAJOR | the cubic is never applied at the memo's own bound | arithmetic: `1371·1370·1369/6` | `428 558 605` — §5.2 exact | **CLOSED** |
| **M-3** MAJOR | §6.5 registers no criterion | see P-1 | — | **CLOSED BUT MOVED** → P-1 → NEW-1/NEW-2 |
| **M-4** MAJOR | T2's boolean refines nothing | `mytuple.py 11`, T2 | `k = 47`; no boolean in `project` | **CLOSED** |
| **M-5** MAJOR | T3's justification names the wrong quantity | `tmerge.py` max exact `t` over all length-11 lines | **max `t` = 2**, distribution `{0: 46 686, 1: 10 109, 2: 1 201}` over the rule-4-excluded own patterns; §6.4's T3 row now reads *"A CHOICE, not a derivation"* | **CLOSED** |
| **M-6** MAJOR | D-706 and the memo are one act | read §1.5 | *"D-706 records the ruling; it does not license it"* | **CLOSED** |
| **M-7** MAJOR | *"discharges"* THM-WINDOW | `git grep -n discharge` at `c7a3ee1` | one hit, and it is the withdrawal | **CLOSED** |
| **m-1** MINOR | the containment convention binds nothing | `L − 5 − min(6, L−5)` | `0 / 0 / 0` at 7, 9, 11; `2 of 8` at 13 — §4 exact | **CLOSED** |
| **m-2** MINOR | self-check 5's `Completed` arm cannot fail | read §6.7 item 5 | the arm is named as unchecked, with its reason | **CLOSED** |
| **m-3** MINOR | Buro's floors imported untagged | read §6.3 | `≤4` / `≥20` **ARCH**, 75 **EMP, NON-TRANSFERABLE** | **CLOSED** |
| **m-4** MINOR | clip points are hand-written and called otherwise | read §6.4 | both clip rungs say **"A CHOICE"** | **CLOSED** |
| **m-5** MINOR | the scored-cell restriction is well-definedness, not convenience | read §4 | stated as convergence of the additive sum | **CLOSED** |
| **m-6** MINOR | code-unit `η²` ≈ 1 by memorisation | read §6.5 | `ω²` primary, `η²` beside it with the reason | **CLOSED** |
| **m-7** MINOR | `extract.py:24-27` off by one | `awk 'NR>=23&&NR<=27' tools/texel/extract.py` | `24 GAME = 0`, `25 MOVES…`, `26 SCORE_KIND…` — **`:24-26` exact** | **CLOSED** |
| **m-8** MINOR | swap equivariance unclaimed | my own orbit enumeration at `L = 11` | **0 mismatches over 59 049; 357 classes; 17 swap-fixed; 187 orbits** — §5.3 exact | **CLOSED** |

### 1.2 Round 2 (`hex_threat_enum_v1_CONFIRM.md`)

| ID | defect | MY attack | output | verdict |
|---|---|---|---|---|
| **N-1** BLOCKING | a value-free quotient passes the registered criterion | see P-1 | — | **CLOSED BUT MOVED** → P-1 → NEW-1/NEW-2 |
| **N-2** MAJOR | §5.5's merge count published without its population | `tmerge.py`, both populations | rule-4 excluded **27 of 335 / 5 348 of 57 996**; §5.5 prints both this and the unexcluded pair | **CLOSED** |
| **N-3** MAJOR | *"not something a single-axis class can hold"* is false | `mytuple.py` seventh component; arithmetic on `C(k+2,3)` | `357→384` (`7 647 059→9 511 040`), `121→132` (`302 621→392 084`), `47→58` (`18 424→34 220`, **+86 %**), `36→45` (`8 436→16 215`) — every cell of §5.5's price table exact | **CLOSED** |
| **N-4** MAJOR | the coarsest clip expressing both boundaries is 816 | `mytuple.py 11`, exhaustive set partitions | **the memo now says the coarsest is `k = 9` / 165 in §6.4 and `k = 16` / 816 in §7.1 and §10** | **STILL LANDS** → **NEW-5** |
| **N-5** MAJOR | T3's clip merges more `t`-inconsistent patterns than T4 | `tmerge.py` per rung | **T4 27 / 5 348, T3 11 / 7 056, T2 11 / 11 164, T1 9 / 11 164** over 57 996 — §6.4's cost table exact | **CLOSED** |
| **N-6** MAJOR | gate 19 cannot see a constant `ω²`; four `report.py` mutants survive | my own `mut.sh`: `between += 0.0`-class mutants | the four the receipt names are DEAD, and my `omega2 = eta2` (X6) is DEAD | **CLOSED for those four**; the formula half is **P-3, STILL LANDS** |
| **N-7** MINOR | merge curve tagged DERIVED, no named instrument produces it | read §6.6/§7.2; `refinement()` returns the two-way spread | retagged MEASURED; `--curve` exists and returns `(min, max, mean)` both ways | **CLOSED** |
| **N-8** MINOR | the null is matched on the code space, not the observed population | receipt: enum T4 realises 231 classes, its nulls 294–310 | demoted to a FLOOR in §6.5 | **CLOSED for the unnested null** — **and the identical defect is now un-mitigated in the NESTED null: NEW-1** |
| **N-9** MINOR | residual D-706 leans | read §1.5 and `:114` | permitted under D-424 | **CLOSED** |
| **N-10** MINOR | nothing in the artifact says which unit the criterion binds | read `report.py`'s purity table | still one table, `unit` column, no criterion marker | **CLOSED IN PROSE, artifact unchanged** (trivial; folded into P-9) |
| **N-11** TRIVIAL | stale even-`L` prose in the instrument | `/usr/bin/grep -n "if even" tools/hex_enum/*.py` | `hexenum.py:75: """The class key of one pattern: §5's 6-tuple, or a pair of them if even."""` | **STILL LANDS** → P-9 |
| **N-12** PROCESS | §7 landed while its confirmation was outstanding | `git diff --stat c7a3ee1` | §7 lands with the round that ran it; but see §0 — the instrument moved again **during this review** | **CLOSED, with the class recurring** |

### 1.3 Round 3 (`hex_threat_enum_v1_ROUND3.md`) — the ten findings this round was granted for

| ID | defect | MY attack | output | verdict |
|---|---|---|---|---|
| **P-1** MAJOR | the registered criterion is class-count monotone, so its PASS side licenses nothing | `perm2.py`: hold the partitions fixed, permute the LABEL at position and at game level, 25–40 replicates | §7.4 withdraws the four passes; the replacement nested test carries a worse version of the same defect | **CLOSED AS STATED, MOVED** → **NEW-1, NEW-2**; and the withdrawal is over-broad → **NEW-6** |
| **P-2** MAJOR | §6.4's clip table lists 4 of 8 (or 4 of 20); one table, two criteria | `mytuple.py`, exhaustive set partitions of `{1,2,3,4,5,6,∞}` | **20 clips under the header's criterion, 8 under §6.4's own T1 row.** The table is now complete for the weaker criterion; **both criteria are still on the page and the document now states two different "coarsest" answers** | **STILL LANDS** → **NEW-5**, plus **NEW-7** ("six distinct `k`" is seven) |
| **P-3** MAJOR | gate 19 cannot see a broken `count_key`; `ω²`'s control is a range check and three formula mutants pass | my own `mut.sh`, 13 mutants, scratch tree | **`count_key` half CLOSED** — C1 (total stones only), C2 (per-side presence), C3 (colliding radix) all **DEAD**. **`ω²` half STILL LANDS** — X3, X4, X5 **all ALIVE** | **HALF CLOSED, HALF STILL LANDS** |
| **P-4** MAJOR | §7.4's heading is falsified by §7.1 and §7.3: T4 is affordable at `L = 7` and `L = 9` | my own arithmetic on §7.1's `k` and §7.3's observations | `2 925 → 1 720.1`; `161 700 → 40.43`; `7 647 059 → 1.069`; `7 647 059 → 1.305`. The heading is corrected and §7.3 now says *"comfortably affordable at `L = 7`, affordable at `L = 9`"* | **CLOSED at §7.4 — REOPENED at §7.4c** → **NEW-3** |
| **P-5** MINOR | §7.3 says 180.6 at `L = 11` **and `L = 13`**; the receipt says 220.4 at 13 | `/usr/bin/grep -n "scored cells per position" artifacts/wp22_phase2a/census_r4/*.txt` | `L7 111.1  L9 144.4  L11 180.6  **L13 220.4**`; memo `:859` still reads *"**180.6** at `L = 11` and `L = 13`"* | **STILL LANDS** |
| **P-6(a)** MINOR | the declared governing revision does not contain the instrument | `git ls-tree -r --name-only c5123c1 -- tools/hex_enum` | **0 files**; memo `:33` still declares `c5123c1` | **STILL LANDS** |
| **P-6(b)** MINOR | *"REVISION 2 IS THE ONE FIX ROUND"* is stale | `git grep -n "ONE FIX ROUND" c7a3ee1` | `:34` — in a document whose banner three paragraphs above says round 4 of four | **STILL LANDS** |
| **P-6(c)** MINOR | §6.6's *"always splits"* contradicts its own next paragraph | `git grep -n "always splits" c7a3ee1` | `:721` *"so `L → L+2` always splits"*, against `:727` *"Neither `class_L` nor `class_11` refines the other for `L < 11`"* | **STILL LANDS** |
| **P-7** MINOR | the mutation receipt has no digest; its M3 mutant is mislabelled | read `n6_mutants/run_mutants.sh`; `git grep -n n6_mutants c7a3ee1` | at `c7a3ee1` the citation is a bare path (the digest is added only in the uncommitted working tree). **The mislabel is unfixed at both**: M3 is named *"eta2 and omega2 both 0.5"* and its replacement pair touches `eta2` only | **STILL LANDS** |
| **P-8** TRIVIAL | §5.5's price table mixes two populations in one row | read §5.5 | the prose above the table now states *"27 of the 335 classes … 5 348 of the 57 996"*; the table's last column is unlabelled but the population is one line up | **CLOSED IN PROSE** |
| **P-9** TRIVIAL | *"stale even-`L` prose gone"* is false | `/usr/bin/grep -n "if even" tools/hex_enum/*.py` | `hexenum.py:75` unchanged | **STILL LANDS** |
| **P-10** TRIVIAL | T1's curve called *"flat from 15 000 on"* where it moves `470 → 535` | memo `:893` and the receipt's own row | `+13.8 %`, in the block that calls T2's `+35 %` a climb | **STILL LANDS** |

**Score.** Of the 38 findings across three rounds: **26 CLOSED**, **1 closed in
prose**, **1 half-closed** (P-3), **2 CLOSED BUT MOVED into new findings**
(P-1 → NEW-1/NEW-2/NEW-6; P-4 → NEW-3), and **8 STILL LAND** (P-2, P-3's `ω²`
half, P-5, P-6a, P-6b, P-6c, P-7, P-9, P-10 — N-11 being P-9).

---

## 2 NEW findings against revision 4

### NEW-1 — BLOCKING. The nested test is NOT immune to the monotonicity: it inherits it through its null, and matching the null reverses every *"adds nothing"* verdict — including the one §7.4c's conclusion is built on

**The claim under attack.** §6.5: *"The measurement that is immune to it is
NESTED."* §7.4b: the referent is *"the same refinement by a permuted class
table"*. §7.4c: *"The nested test says the enum adds NOTHING there: 0.93, below
its own null."*

**The defect is visible in the memo's own receipt before any new run.** The
memo's null is not the same refinement. `census.py`'s `permuted()` shuffles the
class table over the WHOLE `3^(L−1)` code space, so joining it with `count`
produces a partition three to five times finer than the join it is the null for:

| L | rung | `JOIN` cells | `JOINNULL` cells (r0/r1/r2) | null / join |
|---|---|---|---|---|
| 7 | T4 | **36** | 172 / 179 / 175 | **4.9x** |
| 7 | T2 | **36** | 135 / 145 / 136 | 3.9x |
| 9 | T4 | **211** | 905 / 898 / 915 | 4.3x |
| 11 | T4 | **812** | 2 782 / 2 814 / 2 805 | 3.4x |
| 11 | T1 | **195** | 908 / 898 / 924 | **4.6x** |
| 13 | T4 | **1 468** | 4 067 / 4 062 / 4 054 | 2.8x |

(`/usr/bin/grep -E "^window.*JOIN" artifacts/wp22_phase2a/census_r4/census_L*.txt`.)

Round 3's P-1 established that on this corpus a finer partition of the code space
scores higher whatever it means. **The nested null is the finer partition.** So
the monotonicity has not been removed — it has been moved onto the null's side,
where it manufactures FALSE NEGATIVES. This is N-8's defect exactly, conceded for
the unnested null and left standing for the nested one, in the same section.

**The reproducer: a null matched the way the memo says its null is matched.**
`nested.py` permutes the class table WITHIN each count stratum. That leaves the
join's cell count and its per-cell code multiplicities equal to the real join's
and changes only which codes share a cell — §6.5's own words, *"the class count
and the number of codes per class are exactly the enum's and the only thing that
changes is which codes share a class"*, applied to the nested statistic instead
of the unnested one. Full corpus, window unit, my own walk:

| L | rung | published ratio | MY reproduction of the memo's null | MATCHED null cells | **MATCHED ratio** | verdict flips? |
|---|---|---|---|---|---|---|
| 7 | **T4** | **0.93 "adds nothing"** | 0.815 (15 reps, cells 163–183) | 40–42 | **1.634 ADDS** | **YES** |
| 7 | T2 | 1.01 adds | 1.059 (10 reps, cells 129–143) | 41–42 | **1.947 ADDS** | no (stronger) |
| 9 | T4 | 1.56 adds | 1.448 (10 reps, cells 901–938) | 266–271 | **1.493 ADDS** | no |
| 11 | T4 | 1.44 adds | 1.327 (6 reps, cells 2 807–2 860) | 1 136–1 151 | **1.539 ADDS** | no |
| 11 | **T1** | **0.98 "adds nothing"** | 0.854 (6 reps, cells 891–935) | 288–295 | **2.857 ADDS** | **YES** |

My reproduction of the memo's own null lands on the memo's own numbers (0.93 vs
my 0.94 at five replicates), so the disagreement is not an implementation
difference — it is the null.

**A second, independent instrument agrees.** `perm2.py` holds every partition
fixed and permutes the LABEL, at the position level and at the game level. That
measures the df-and-clustering entitlement of the actual refinement, with class
count, cell sizes and clustering all exactly preserved:

```
L=7  T4  observed join-count increment +0.000642   permutation null: mean +0.000001,
                                                   range [-0.000001,+0.000003]
                                                   exceeded by 0 of 30 (position) and 0 of 30 (game)
L=9  T4  observed                      +0.001902   null mean +0.000003, max +0.000011
                                                   exceeded by 0 of 25 and 0 of 25
```

**The entitlement of the real refinement is +0.000001. The memo's null claims it
is +0.000690** — a factor of ~500. The memo's null is not measuring what a
value-free refinement of this shape earns; it is measuring what a refinement five
times finer earns.

**Why this is BLOCKING and not MAJOR.** §7.4c's conclusion — *"No cell is
simultaneously affordable and supported by both measurements"* — rests on exactly
one cell reading *"adds nothing"*: `L = 7` T4. Under a null matched on the
property §6.5 says it is matched on, that cell reads **1.63, adds**. The
conclusion inverts. And it is already priced downstream: `matrix_wp22_phase2_eval.md:814`
and `:937` carry the sentence verbatim and rank `R-A4-CLASS` **UNDECIDED** on it.

---

### NEW-2 — MAJOR. The published nested ratio is a max-of-N order statistic over an unregistered replicate count, and it is the free parameter §6.5 boasts of not having

§6.5: *"No margin, no threshold, no number that could be tuned after the
fact — a direction, which is the only form that cannot be moved post hoc."* That
is said of the registered criterion. The statistic that actually carries §7.4c's
conclusion is §7.4b's, and §7.4b defines its referent as *"worst of three
replicates"* — an order statistic over a replicate count nothing registers.

**Reproducer 1 — receipt only, no new run.** Replace "worst of three" with "mean
of the same three", from `census_r4`'s own `JOINNULL` rows:

| | worst-of-3 | mean-of-3 |
|---|---|---|
| `L = 7` T4 | 0.93 **adds nothing** | **1.11 adds** |
| `L = 7` T3 | 0.93 **adds nothing** | **1.11 adds** |
| `L = 11` T1 | 0.98 **adds nothing** | **1.14 adds** |
| all sixteen cells | **13 of 16 add** | **16 of 16 add** |

Every one of the three exceptions is produced by the choice of summary, on the
same three draws. Nothing in §6.5 or §7.4b registers `max` rather than `mean`,
and nothing registers `3`.

**Reproducer 2 — the statistic is not stable in N.** `max` over replicates grows
monotonically with the replicate count, so the published number gets worse for
the enum the more evidence is collected. My reproduction of the memo's own null
at `L = 7` T4:

```
worst over  5 replicates: +0.000684   ->  ratio 0.939
worst over 15 replicates: +0.000788   ->  ratio 0.815
mean over  15 replicates: +0.000597   ->  ratio 1.075
```

**The verdict at the deciding cell is a function of an unregistered parameter,
and it crosses 1.00 inside that parameter's range.**

**The author found half of this while this review ran.** The uncommitted working
tree adds `census.py --null-replicates` with the comment: *"Three replicates left
the widest cell unresolved — its ratio was 0.93 against the worst draw and 1.11
against their mean — and choosing between those two summaries after seeing them
is the post-hoc move the process forbids."* That is the right diagnosis of this
half. It does not touch NEW-1: raising the replicate count characterises an
unmatched null more precisely, which does not make it matched.

---

### NEW-3 — MAJOR. §7.4c's *"No cell is simultaneously affordable and supported by both measurements"* is false on the memo's own §7.3 and §7.4/§7.4b, and it re-commits P-4 one section later

Read the memo's three tables against each other at `L = 9` T4:

| test | §-owner | value at `L = 9` T4 | verdict |
|---|---|---|---|
| affordability | §7.3 | **40.4** observations per nominal parameter, against Buro's `≥ 20` | §7.3's own word: *"**affordable at `L = 9`**"* |
| registered criterion | §7.4 | `ω²` 0.004606 vs 0.004158 = **1.108** | **MET** |
| nested test | §7.4b | **1.56** | **adds** — the second-highest ratio in the table |

`L = 9` T4 is affordable, and positive on both measurements. §7.4c reaches the
opposite conclusion by quoting only the raw parameter count (*"the parameter
counts are 161 700 and 7 647 059"*) and never the ratio §7.3 computed from it,
and by silently replacing §7.3's standard, "affordable" (`≥ 20`), with an
undefined stricter one, "comfortably affordable" (met only at 1 720). **P-4 was
the finding that §7.4 priced T4 at every length with `L = 11`'s number. §7.4c
does the same thing with `L = 9`'s.** Under D-555's remedy-breakage class this is
the fix breaking one section downstream of itself.

The escape route is also closed. If the reply is *"the registered PASS is
withdrawn as unsound, so 1.108 is not support"*, then `L = 9` T4 is still
affordable and still positive on the nested test, which §7.4c treats as the sound
measurement; and if the reply is *"the nested test is only a finding"*, then
`L = 9` T4 is still affordable and still MET. There is no reading of the memo's
own sections on which no cell is both.

---

### NEW-4 — MAJOR. Gate 19 has NO coverage of the nested measurement revision 4 added. Six mutants, six alive, two of them pinning the published verdict

P-3's defect class, at the code revision 4 wrote to answer P-1. The whole nested
apparatus — `window_join`, `window_join_null`, and the `(count, class)` keys — is
untested:

```
$ /usr/bin/grep -rin "join\|nested\|permut" tools/hex_enum/test_hex_enum.py tools/hex_enum/test_census.py
(only str.join, and refinement()'s partition join — nothing about count x class)
```

My own mutation driver, one mutation at a time in a scratch copy of `tools/`,
restoring between each, `bash tools/hex_enum_tests.sh`:

```
baseline (unmutated)                                       exit 0
J1 the nested NULL uses the TRUE class (ratio pinned 1.00) exit 0  ALIVE
J2 the JOIN drops the class (increment pinned at 0)        exit 0  ALIVE
J3 the nested NULL is the raw code (enum can never add)    exit 0  ALIVE
J4 the JOIN is the raw code (enum always adds)             exit 0  ALIVE
J5 the JOIN ignores counts (nesting removed)               exit 0  ALIVE
J6 only ONE null replicate is drawn                        exit 0  ALIVE
```

**J1 and J4 each produce a wrong published answer, in opposite directions, with a
green gate.** J1 is proved non-equivalent by behaviour — the shipped census and
the J1 census over the same 500 positions at `L = 7`:

```
clean  JOINNULL count x T4 r0/r1/r2 : 154/152/156 cells, omega2 0.008192/0.007797/0.008119
J1     JOINNULL count x T4 r0/r1/r2 :  35/ 35/ 35 cells, omega2 0.006739/0.006739/0.006739
```

Under J1 every replicate collapses onto the JOIN row and every ratio in §7.4b
becomes exactly 1.00. The suite does not notice.

**And P-3's `ω²` half is still open.** The three formula mutants round 3 named
are alive at `c7a3ee1`:

```
X3 df correction uses classes, not classes-1               exit 0  ALIVE
X4 mean_within denominator is count, not count-classes     exit 0  ALIVE
X5 omega2 denominator is total, not total+mean_within      exit 0  ALIVE
X6 omega2 is eta2 (correction removed entirely)            exit 1  DEAD
```

The control at `test_hex_enum.py:198-200` is still `0.0 < omega2 < eta2` — a
range, not a value. Hand arithmetic on that fixture (`within = 8`, `between =
200`, `total = 208`, `count = 8`, `classes = 2`) puts the true `ω² = 0.94904` and
all three mutants at 0.94268 / 0.95215 / 0.95513 — inside the range, every time.
**Credit where it is due**: the `count_key` half of P-3 IS closed. My C1 (total
stones only), C2 (per-side presence) and C3 (colliding radix) are all DEAD, and
`the_count_only_baseline_is_blind_to_everything_the_tuple_carries` now pins the
referent's key count exactly.

---

### NEW-5 — MAJOR. §6.4 states two incompatible criteria for "faithful", and §7.1 and §10 still assert the answer §6.4 withdraws

Three sentences in one document, on one question:

```
$ git grep -n "3–4\|six distinct\|coarsest faithful\|the coarsest gives" c7a3ee1 -- docs/experiments/hex_threat_enum_v1.md
:537  | **T1** | … the law CONSTRAINS the clip to refine `{≤2 | 3–4 | ≥5 | dead}` and does not determine it. |
:569  It is not the coarsest faithful answer — that is `k = 9` and 165 …
:829  … §6.4's coarsest faithful clip would give `k = 16` and 816, and is not shipped.
:1091 | **N-4** … All four clips expressing both `LAW-SUPPORT` boundaries are enumerated, the coarsest gives `k = 16` and `C(18,3) = 816` …
```

**My own exhaustive enumeration** (`mytuple.py 11`, every set partition of
`{1,2,3,4,5,6,∞}`, `k` computed under my own tuple):

| faithfulness criterion | source in the memo | clips | coarsest |
|---|---|---|---|
| `{cost ≤ 2}` and `{cost ≤ 4}` each a union of blocks | §6.4's paragraph at `:549` | **20** | `1,2 \| 3,4 \| 5,6,dead` → `k = 9`, `C(11,3) = 165` |
| refines `{≤2 \| 3–4 \| ≥5 \| dead}` | §6.4's own T1 row at `:537` | **8** | `1,2 \| 3,4 \| 5,6 \| dead` → **`k = 16`, `C(18,3) = 816`** |

The 20-clip count is right. But the two criteria differ by exactly one thing:
whether a permanently DEAD window may share a block with a live window needing
five more stones. `LAW-HIT` (*"One defender stone in any empty cell of a window
kills that window permanently"*) says it may not, §6.4's T1 row says it may not,
and the shipped `hexenum.py`'s own `clip()` docstring says it may not — *"`INF`
stays apart because `DEF-WINDOW`'s dead is not an expensive live window."*

**So the paragraph that withdraws revision 3's claim does so under the ONLY one
of the memo's two criteria that contradicts `LAW-HIT`.** Under the criterion the
memo states one row earlier, revision 3 was right: the coarsest faithful clip is
`k = 16` and `C(18,3) = 816`, and the coincidence with the import is not
dissolved by the enumeration. §7.1 and §10 still say so. **A reader cannot take
either answer out of this document**, and P-2's core sentence — *"One table, two
criteria"* — is untouched: revision 4 corrected the count and kept the ambiguity
that made the count wrong.

---

### NEW-6 — MINOR. §7.4's withdrawal of the four passes is over-broad; at `L = 7` the monotonicity runs the other way

§7.4: *"**The four METS do not** [stand]: the statistic is class-count monotone
… and T4 has 231 classes against the referent's 52 at `L = 11`."* The reason is
sound at `L = 9`, `L = 11` and `L = 13`, where the enum has more classes than the
referent. **At `L = 7` the enum has 16 classes against the referent's 23** — the
monotonicity penalises the enum there, so the pass at `L = 7` T4/T3 is if
anything conservative, and withdrawing it on this ground is withdrawing it on a
reason that does not apply.

Measured, `perm2.py`, the properly matched null for the registered statistic:

```
L=7  T4  observed enum-count = +0.000048   permutation null mean +0.000000,
                                           range [-0.000002,+0.000002]
                                           exceeded by 0 of 40 (position) and 0 of 40 (game)
L=9  T4  observed enum-count = +0.000448   null mean +0.000001, max +0.000007
                                           exceeded by 0 of 25 and 0 of 25
```

Both passes survive a null that holds class count, cell sizes and clustering
exactly fixed. The withdrawal costs the memo the one piece of evidence that
would have decided §7.4c.

---

### NEW-7 — MINOR. *"they take six distinct `k` at `L = 11`"* — there are seven, and the table under the sentence lists all seven

`:549`. My enumeration: `k ∈ {9, 16, 23, 25, 34, 36, 47}`, with multiplicities
`1, 5, 1, 7, 2, 3, 1` summing to 20. The memo's own table has seven rows.

---

### NEW-8 — MINOR. §10 is a revision behind and contradicts §7.4c on the headline

```
:1058 ## §10 What the two review rounds changed          <- there have been three
:1065 This revision is round 3 under D-709                <- it is round 4
:1084 ### Round 3's remedies …                            <- the table lists round 2's N-findings
:1099 N-1's remedy does not rescue the row — **it kills it** … That is the finding this package delivers
:986  **SO THE ROW IS UNDECIDED** … **It is not killed**
```

There is no table of what round 4 changed, so the six round-3 findings revision 4
did not address (P-5 through P-10) are not recorded as unaddressed anywhere. And
`:1099` and `:986` state opposite conclusions about the row two sections apart.

---

### NEW-9 — MINOR. §7.4b publishes 10 of the 16 cells while its prose counts 13 of 16

The reader cannot check *"THE ENUM ADDS AT 13 OF 16 CELLS"* against the table
given; six rows (`7` T1, `9` T3, `9` T1, `11` T3, `13` T3, `13` T1) are absent.
I computed them from `census_r4`: all six ADD, so **13 of 16 is arithmetically
correct** — but it is correct on evidence the section withholds, in a document
whose §7.4 prints all sixteen rows of the comparable table.

---

### NEW-10 — MINOR. The run §7.4b reports is cited by neither path nor digest

§7 cites `census_r3` and its digest `9bb8fc6f…`. §7.4b's numbers exist only in
`census_r4` (digest `aa88c298ac422a87df6505afcc33fcb1e2cb156783e81ec525b13dc4681bbdd1`),
which the memo never names. `git grep -n "census_r4" c7a3ee1` returns nothing.
D-483 wants the run's artifact cited by digest.

---

### NEW-11 — MINOR. The named instrument still asserts the claim §6.4 calls false

`hexenum.py`'s `clip()` docstring: *"expressing both needs 1, 2, 3 and 4 apart"*.
§6.4 `:543`: *"Revision 2 fixed the clip and claimed the boundaries were
'expressible only if 1, 2, 3 and 4 stay apart', **which is false**."* Gate 19's
own instrument carries the withdrawn sentence.

---

## 3 What I attacked and it SURVIVED

After three rounds this is worth as much as the failures, and a great deal of
this document is now finished.

1. **THE ENUM'S ARITHMETIC IS RIGHT, at a fourth independent implementation.**
   `mytuple.py` reproduces every cell of §7.1: `k = 25 / 98 / 357 / 357` at T4,
   `25 / 57 / 121 / 121` at T3, `15 / 27 / 47 / 47` at T2, and every
   `C(k+2,3)` — 2 925, 161 700, 7 647 059, 32 509, 302 621, 680, 3 654, 18 424,
   364, 1 540, 8 436. Four implementations, zero disagreements, across four
   rounds.
2. **The corpus walk is right.** My own walker — own parse, own quiet filter, own
   projection — reproduces `45 271` positions and the window-unit `n` of
   `8 241 249 / 10 613 918 / 12 980 519` exactly, and every `ω²` I recomputed for
   a partition the census reports matches the receipt to six decimals. The
   `census_r4` receipt is clean and its content is what the memo says it is.
3. **§7.4's registered-criterion table is exact at all sixteen cells.** I
   recomputed every ratio from the receipt: 1.014 / 1.014 / 0.793 / 0.686;
   1.108 / 0.760 / 0.673 / 0.533; 1.125 / 0.651 / 0.637 / 0.510; 0.834 / 0.483 /
   0.472 / 0.378. Every one exact, including the class counts.
4. **§7.4b's arithmetic is exact.** Every increment, every "worst null
   increment", and every ratio in the ten published rows recomputes from
   `census_r4`'s `JOINNULL` rows. My objection is to the null, not to the
   division.
5. **§7.4d is exact and its count is right.** Five code-unit cells exceed the
   referent — `L7` T4 1.140, `L7` T3 1.140, `L9` T4 1.620, `L11` T4 2.269,
   `L13` T4 1.636 — and **exactly one flips the verdict between units**
   (`L13` T4, 0.834 window / 1.636 code). I checked the other direction too:
   no cell passes on the window and fails on the code. The reviewer's claim to
   have counted these himself holds.
6. **§5.5 and §6.4's `t`-consistency measurements are exact**, under my own
   minimum-hitting-set implementation over all `3^10` patterns: T4 **27 of 335 /
   5 348 of 57 996**, T3 **11 of 109 / 7 056**, T2 **11 of 40 / 11 164**, T1
   **9 of 30 / 11 164**; max single-axis exact `t` = **2**. And the repair price
   `357→384 / 121→132 / 47→58 / 36→45` with `C(k+2,3)` `9 511 040 / 392 084 /
   34 220 / 16 215`, the `+86 %` at T2 included.
7. **§5.3's equivariance is exact**: 0 mismatches over 59 049, 17 swap-fixed
   classes, 187 orbits — reproduced from scratch.
8. **P-4's affordability arithmetic is right and the correction is real.**
   1 720.1 / 40.43 / 1.069 / 1.305. §7.3 now states the qualifier §7.4 dropped,
   and §7.4's own heading is fixed. My objection is that §7.4c then loses it
   again — the fix itself is sound.
9. **P-2's 20 is right.** The clip enumeration is complete under the criterion
   its own paragraph states, and the five clips landing on 816, the `k = 23` and
   `k = 34` rows round 3 said were missing, and the shipped rung's position as
   second-finest are all confirmed.
10. **P-3's `count_key` half is genuinely closed** — three mutants of mine that
    no prior round tried are DEAD, and the new test pins the referent's key count
    against `(own, opp)` pairs rather than against a witness.
11. **N-6's four `report.py` mutants are dead, and so is a fifth of mine**
    (`omega2 = eta2`). `terms()` really does compute the total without the
    partition and raise by name.
12. **The disclosure discipline is real, not decorative.** §6.5 discloses the
    pilot the criterion was registered in knowledge of, and discloses that the
    nested test was built after the criterion fired. §7.4d's three disclaimers
    are accurate. §1.5 refuses to cite D-706 as an external premise. §6.6
    refuses to claim `THM-WINDOW` is closed. None of that is where this
    revision fails.

**On §6.5's post-hoc disclosure specifically** (the prompt asks whether
disclosing is enough): disclosing it is the right move and it does not
contaminate the registered criterion's verdict. §7.4 reports the registered
result first, states the withdrawal, and does not let §7.4b overwrite it. The
problem with §7.4b is not that it is post-hoc; it is that it is wrong (NEW-1,
NEW-2). A post-hoc measurement that were correct would be a legitimate finding
beside the verdict, exactly as §6.5 frames it.

**On §7.4's split** (twelve fails are evidence, four passes are not): the
asymmetry is sound in principle — a partition scoring below a COARSER value-free
one has lost without any entitlement argument, and that is a real one-sided
inference. It is misapplied at `L = 7` only (NEW-6), where the enum is the
coarser partition.

---

## 4 VERDICT

# FAIL

- **1 new BLOCKING** (NEW-1), **4 new MAJOR** (NEW-2, NEW-3, NEW-4, NEW-5),
  6 new MINOR.
- **8 prior findings STILL LAND**: P-2, P-3 (`ω²` half), P-5, P-6(a), P-6(b),
  P-6(c), P-7, P-9, P-10.
- PASS required every finding CLOSED and no new BLOCKING or MAJOR. Neither holds.

**This is the fourth round. Under D-709 there is no fifth: STOP and split.**

I record what I think the split should respect, because the failure is not
uniform. **The enum itself is finished** — four independent implementations, no
arithmetic error in four rounds, every structural measurement (§5.3, §5.5, §6.4's
cost table, §7.1, §7.2, §7.3) reproduced exactly. What has failed four times is
one thing: **the statistical apparatus in §6.5/§7.4/§7.4b/§7.4c that tries to
turn `ω²` over position-level labels into a verdict on the row.** Round 1's M-3,
round 2's N-1, round 3's P-1 and this round's NEW-1 are the same defect arriving
in four costumes: a referent that is not matched on the property that drives its
score. A split that carries §1–§5 and §6.1–§6.4 and §7.1–§7.3 forward as settled,
and sends §6.5 and §7.4-onward back as its own package with its own gate, is the
split the evidence supports.

**Nothing here is overruleable under D-424.** NEW-1 and NEW-3 each name a way the
package produces a wrong answer that a matrix has already priced from; NEW-4
names a way it can produce a wrong answer with a green gate; NEW-5 names a
document that states two answers to one question. NEW-6 through NEW-11 are
corrections, and NEW-7, NEW-9 and NEW-11 would be fair overrule candidates if
anyone wanted them.

---

## 5 If I would state §7.4c's conclusion differently — and I would

**§7.4c is not honest, and its dishonesty is not the usual kind.** It is not
hiding a bad result; it is declining a good one. Every mechanism that produces
"UNDECIDED" is an artifact I can name and measure:

- the *"adds nothing"* at `L = 7` T4 is an unmatched null (NEW-1) compounded by
  a max-of-three order statistic (NEW-2). Fix either and the cell reads *adds*;
- the *"not affordable"* at `L = 9` is a standard silently narrowed from
  "affordable" to "comfortably affordable" between §7.3 and §7.4c (NEW-3);
- the *"the PASS side is unsound"* at `L = 7` is a monotonicity argument applied
  where the monotonicity runs the other way (NEW-6).

Remove all three and no cell is left undecided. **The evidence supports a
verdict, and the verdict is that the row is PRICED and WEAK.**

### What I would write instead

> **§7.4c — THE ROW IS PRICED, AT `L = 9` T4, AND IT IS WEAK.**
>
> `L = 9` T4 is the cell that survives every test this package can run. It costs
> `C(100,3) = 161 700` nominal parameters against 6 537 654 scored-cell
> observations — **40.4 per parameter**, twice Buro's `≥ 20` safe-fit line
> (**ARCH**) and thirty times the 1.2 at which `eval_families` §A1 kills
> `R-A1-L11`. The registered criterion is MET there at **1.108**, and that pass
> is not the class-count artifact round 3 named: holding the partitions fixed and
> permuting the label at the position and at the game level, the enum's excess
> over stone counts is **+0.000448 against a null of mean +0.000001 and maximum
> +0.000007, exceeded by 0 of 25 permutations at both cluster levels**. The
> nested test agrees at **1.49** against a null matched on the join's own cell
> count. `L = 7` T4 is the same story at a twentieth of the price — 1 720
> observations per parameter, MET at 1.014, 0 of 40 permutations, nested **1.63**
> — and it is the cell a matrix that wants the cheapest defensible codebook
> should price.
>
> **AND THE EFFECT IS SMALL, WHICH IS THE REAL LIMITATION AND IS NOT THE SAME AS
> UNDECIDED.** The enum's `ω²` excess over the stone-count referent is
> **+0.000048** at `L = 7` and **+0.000448** at `L = 9`, on a statistic where the
> referent itself reaches only 0.0042. Threat structure carries label information
> that stone counting does not, on this corpus, at both cluster levels, and it
> carries a little of it. `ω²` over position-level labels cannot convert that
> into a fit quality, an Elo, or a bound on either, and D-621 and D-622 foreclose
> reading tactical value out of these labels at all. **What is owed before the
> row is ranked is not another purity statistic — it is a fit.**
>
> **WHAT REMAINS GENUINELY UNDECIDED IS THE COVERING LENGTH, AND ONLY IT.** At
> `L = 11` T4 the row costs 7 647 059 parameters against 1.07 observations each,
> which no measurement in this document can rescue; the coarse rungs that are
> affordable there lose to stone counting at 0.51–0.65 and that FAIL is sound
> without any entitlement argument. So `R-A4-CLASS` is priceable below the
> covering length and unpriceable at it, and a matrix should carry the length in
> the row's name.

### The evidence that sentence rests on

| claim | instrument | number |
|---|---|---|
| `L = 9` T4 affordability | §7.1 `k = 98`, §7.3 observations | 161 700; 6 537 654; **40.43** |
| registered pass is not an artifact | my `perm2.py`, position and game level | +0.000448 vs null mean +0.000001, **0 of 25** exceedances at each level |
| nested agrees under a matched null | my `nested.py`, class permuted within count stratum | **1.493** (266–271 null cells against the join's 211) |
| `L = 7` T4 likewise | same two instruments | +0.000048, **0 of 40**; nested **1.634** |
| the covering length is genuinely dead | §7.1, §7.3, §7.4 | 7 647 059 / 1.069; coarse rungs 0.510–0.651, FAIL side sound |

**One caveat I hold against my own recommendation.** My cluster-level permutation
null destroys the label association for BOTH partitions at once, so it measures
the entitlement of the refinement rather than the conditional entitlement given
that stone counts already explain something. That second-order term is bounded by
the observed `ω²` scale (≈0.005) and cannot account for a 0.000448 excess against
a 0.000007 maximum, so the direction is not in doubt — but a reader who wants the
conditional version should ask for a within-count-stratum label permutation, and
that is an hour of work, not a new corpus.
