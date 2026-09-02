# WP-2.1 sweep throughput — a scaling study. PRE-REGISTRATION, revision 1.

> **ONE LINE.** The sweep's registered wall is **7.15 h at eight concurrent
> tranches, and that is a LOWER BOUND with contention assumed at 1.00** —
> `wp21_prereg.md` §3 says so in its own words. This study measures the two
> things that could move it: **what concurrency the box actually pays for**, and
> **whether the 2.14x duplication the corpus is REQUIRED to carry has to be paid
> for twice in SEARCH time as well**. It changes no label, no criterion and no
> corpus byte; it may only change how long producing them takes.

**WHY IT IS REGISTERED RATHER THAN JUST RUN.** The output is a number that
decides a run parameter of a 3,487-opening sweep, and a concurrency picked after
seeing which setting looked fastest on the day is the after-the-numbers decision
`docs/process.md` forbids. The decision rule is in §3.4 and §4.5, before either
lever is measured.

**WHAT IT MAY NOT DO** (stated first, because a throughput study is exactly where
this gets lost): it may not change the label budget, the game budget, the turn
cap, the seat, the criteria of `wp21_prereg.md` §4, or which openings are swept.
A faster sweep that answers a different question is not a faster sweep.

---

## 1. THE TWO LEVERS, AND WHY THESE TWO

**THE COST IS CAPTURE AND ALMOST NOTHING ELSE.** Per tranche, from
`wp21_prereg.md` §3's own arithmetic: capture **11 017 s of 12 870 s — 85.6%**.
Play is 11%, replay 3%, the cold check 0.4%. **A lever that is not on the capture
pass cannot matter**, which is what rules out the obvious ones: `n_workers` on
pass 1, a faster replay, a wider cold stride.

| lever | what it attacks | code change |
|---|---|---|
| **A — concurrency** | the sweep runs 16 tranches 8 at a time on a box with 8 physical cores and 16 threads. Whether 8 is the throughput optimum is **UNMEASURED** | none |
| **B — the label cache** | `capture::run` asks the engine at **every asked prefix of every game**, and the pilot MEASURED 742 records over **347 distinct positions**. The same position is searched 2.14 times at `nodes 400000` | `pistol-arena`, and it is a package with its own review |

**LEVER B'S SIZE IS NOT AN ESTIMATE AT THE GAME LEVEL.** The pilot's run log
reads `n 26 distinct-n 13 (13 duplicate games)` — **exactly 2.0**, and it is
structural rather than a setting: `arena --capture` refuses a report whose two
seats do not attest one engine (D-558(2)), so a self-match is the only capturable
shape, and a self-match of one deterministic engine plays each opening
identically in both seats. **Every asked prefix of every game is therefore asked
at least twice**, and the pilot's 742/347 = **2.1383 MEASURED** adds the
cross-game transpositions on top.

**AND IT DOES NOT TOUCH D-562(2).** That ruling forbids DEDUP AT CAPTURE because
*"a capture that dropped a duplicate would destroy the 2.14x duplication factor
D-560's whole arithmetic rests on"*. **A cache drops no record.** Every record
the uncached pass writes, the cached pass writes, with the same bytes; what
changes is only whether the engine was asked again for an answer already known.
The duplication factor survives intact because the duplication survives intact.

---

## 2. THE SOUNDNESS ARGUMENT FOR LEVER B, MADE BEFORE ANY MEASUREMENT

**A LABEL IS A PURE FUNCTION OF (position, budget, binary, config), AND THAT IS
STRUCTURAL IN THIS TREE RATHER THAN HOPED FOR.** `crates/pistol-arena/src/capture.rs`'s
`ask` sends `newgame` before every `position`/`go`
(`for line in [NEW_GAME, position, go]`); `pistol-engine`'s `new_game` calls
`Searcher::clear`, which clears the transposition table, the heuristic tables and
the solver. **No state crosses an ask.** So two asks at one position return the
same bytes, and that is what makes a cache a re-use rather than an approximation.

**THE KEY IS THE POSITION AND NOT ITS SPELLING, AND NOT A HASH.** Two prefixes
whose move orders differ but whose stones agree replay to the SAME `GameState` —
the board is a set of stones and the stone count fixes both `to_move` and
`phase` — so they must take the same label. The key is therefore the replayed
position's **sorted `(cell, player)` list**, compared exactly. **A 128-bit
zobrist would have been the obvious key and is deliberately not used**: a cache
that answers from a collision returns a label for a position nobody played, and
a probability argument is not a criterion. Building the exact key costs one
replay of at most eighty placements against a search of ~885 ms.

**THE CACHE'S EQUIVALENCE IS `key_pos`'s, AND SAYING SO CLOSES A QUESTION A
REVIEWER WOULD OTHERWISE HAVE TO DERIVE.** Keying on the replayed position's
sorted stone list folds TRANSPOSITIONS and not symmetries — exactly what
`GameState::key` folds. It is strictly finer than `key_full` (which folds
symmetry too) and strictly coarser than `key_seq` (which is move-order
sensitive). **D-562(2)'s unsettled three-key question is untouched by that**, and
the reason is that the cache decides nothing about the CORPUS: every record is
written, with its own `position` move-list string — which differs between two
transposed prefixes — and only `totals` and `bestmove` come from the cache, both
of which are functions of the `GameState` alone. *Which key rules a
disagreement* stays WP-2.0b's transposition question, as D-562(2) says.

**THE CRITERION THAT DECIDES IT IS AN EXTERNAL REFERENT AND IS REGISTERED HERE**:
§4.4. It is not internal agreement, not a plausible speedup, and not the cold
check alone.

---

## 3. LEVER A — CONCURRENCY. THE MEASUREMENT.

### 3.1 The workload, fixed before any setting runs

**ON THE PILOT'S ALREADY-CONSUMED RANGE, so this study spends nothing.**
`docs/book_v2_ledger.md` records `0..12` as consumed by the WP-2.0 pilot, and
D-539 says the pilot is not corpus — re-reading it buys no evidence about
anything and costs no unseen opening. **Openings `0..2`** (3 openings, 6 games)
are played once into ONE report, and every concurrent process in every setting
captures THAT SAME REPORT. One workload, so the settings differ in concurrency
and in nothing else.

### 3.2 The settings and the reps

**THE FIELD IS {1, 2, 4, 8, 16} AND 12 IS DELIBERATELY ABSENT.** With sixteen
tranches, wall is `ceil(16/N)` waves, so only an N that DIVIDES 16 gives balanced
waves. N=12 runs twelve tranches and then four, leaving two thirds of the box
idle for the whole second wave — it cannot beat N=8 and can only tie it. **The
consequence is a rule and not a note: if the sweep's concurrency changes, the
TRANCHE COUNT changes with it**, so that `TR mod N == 0`. A study that measured
12 and recommended it would be recommending a wave structure nobody costed.

Concurrency **N ∈ {1, 2, 4, 8, 16}**, **3 reps each**, run in the fixed order
`1, 2, 4, 8, 16` and then that order twice more — **not** three reps of one
setting before the next, so a thermal ramp lands on every setting equally rather
than on the one that ran last. (This is `wp20b_perf_guard.sh`'s rotation lesson,
which cost that package a whole run.)

At each setting, N processes each run
`arena --capture <the report> --label-nodes 400000 --out <distinct path>`
concurrently, started together, and the setting ends when the LAST one exits.

### 3.3 The statistic, and it is one

**Realised seconds per label at concurrency N** =
(wall from first start to last exit) x N / (N x records) = wall / records,
where `records` is the report's own record count — so the statistic is simply
**the wall of one process's capture at that concurrency**, and the aggregate
**throughput** is `N x records / wall`.

Reported per setting: **median over the three reps** of the throughput, and the
**contention factor** `c(N)` = (realised seconds per label) / **0.8854**, the
pilot's MEASURED serial rate.

### 3.4 THE DECISION RULE, REGISTERED BEFORE THE RUN

**The sweep runs at the N whose median THROUGHPUT is highest**, with two guards
that fire before it:

- **A tie inside 5% goes to the SMALLER N.** Two settings that produce the same
  throughput are not the same run: the smaller one leaves the box able to answer.
- **The selected N must divide the tranche count**, per §3.2. An N that does not
  is not selected; the tranche count is re-registered to a multiple of it, and
  that re-registration reopens `wp21_prereg.md`'s review.
- **Any N whose realised seconds-per-label exceeds `hang_timeout_ms`/1000 = 120 s
  for a single label is REFUSED whatever its throughput** — a setting that risks
  the watchdog turns a throughput win into a voided tranche.
- **If the highest-throughput N is 16, the wall figure reported for it carries
  the note that 16 is SMT-shared**, because a per-tranche time at 16 that is more
  than 2x the time at 8 means the second wave was cheaper than the sharing.

**WHAT THE RESULT MAY AND MAY NOT DO.** It may set `wp21_prereg.md`'s
concurrency, which is a run parameter and not a criterion, by an amendment that
reopens that document's review. **It may not** change tranche count, partition,
or any §4 criterion.

### 3.5 THE CRITERIA, AND THE DESIGN ELEMENTS THEY ARE NOT

**Kept apart on purpose.** A design element is something this study DOES; a
criterion is something the data can FAIL. Listing the two in one column is how a
document comes to report four criteria met when two of them were never at risk.

**DESIGN ELEMENTS** — fixed here, and no result can falsify them: one report
captured at every setting (so the settings differ in concurrency and nothing
else); three reps in rotated setting order (so drift lands on every setting
equally); the statistic is aggregate THROUGHPUT and not per-process time (so a
setting is not penalised for doing N times the work).

**CRITERIA** — each with the defect class it excludes, and each falsifiable:

| # | criterion | the defect class it excludes | can it fail? |
|---|---|---|---|
| **C1** | every process's corpus file, at every N and every rep, is byte-identical to every other's | a capture whose answers depend on machine load | **yes** — and §3.6 says what happens then |
| **C2** | every process's record count equals the report's own | a process that exited early and looked fast | yes |
| **C3** | no single label's wall exceeds 120 s at any N | a throughput win that risks the `hang_timeout_ms` watchdog and so buys a voided tranche | yes |
| **C4** | the N=1 realised seconds-per-label is within 20% of the pilot's MEASURED 0.8854 | a study whose serial baseline is not the sweep's baseline — i.e. a box, a binary or a workload that makes the whole comparison about something else | yes |

**C4 IS THE ONE THAT KEEPS THE REST HONEST.** Without it every other number is
internally consistent and externally unmoored: a run on a throttled box would
produce a perfectly self-agreeing scaling curve for a machine the sweep will not
run on. It is an EXTERNAL referent — the pilot's rate, measured by a different
run of a different document — which is what `docs/process.md` says a reviewer
looks for first.

### 3.6 THE REGISTERED CONSEQUENCE OF EACH FAILURE

`docs/process.md` requires an agreement criterion to carry a registered
consequence, stated before the run.

| failure | consequence |
|---|---|
| **C1** | **THE SWEEP STOPS, not this study.** Two concurrent captures of one report disagreeing means the sweep's labels depend on machine load, which no concurrency setting fixes and which voids the registered plan itself. It is reported as a finding about the ENGINE or the harness, and `wp21_prereg.md` may not run until it is closed. |
| **C2** | the affected setting is VOID and is re-run once; a second void at the same N drops that N from the field rather than being investigated here |
| **C3** | that N is refused whatever its throughput, per §3.4 |
| **C4** | **the whole study is VOID.** No N is selected, the sweep keeps its registered 8, and the gap between this box and the pilot's is the finding. |

---

## 4. LEVER B — THE LABEL CACHE. WHAT IS REGISTERED BEFORE THE CODE EXISTS.

### 4.1 The measurement that comes FIRST, and needs no code

Over the pilot's own report, count **asked prefixes** and **distinct replayed
positions** under §2's exact key. The ratio is what a cache could save, MEASURED
on a real report rather than argued from the pilot's 742/347 summary. If the
ratio is below **1.5**, **lever B is dropped and no code is written** — that
threshold is registered here, before the count.

### 4.2 What is implemented if it survives

A memo in `capture::run`: the exact key of §2 to the `(totals, bestmove)` the
engine returned. Nothing else changes — every record is still written, in the
same order, with the same fields.

### 4.3 What it costs to be wrong

A cache that returns a label for the wrong position writes a corpus whose records
are silently mislabelled, and **nothing downstream would notice**: the record
carries a plausible score for a plausible position. That is why §4.4's criterion
is byte-identity against an uncached run and not a sample.

### 4.4 THE CRITERION, AND IT IS AN EXTERNAL REFERENT

**An uncached capture and a cached capture of the SAME report produce
BYTE-IDENTICAL corpus files.** The uncached pass is the referent and it shares no
code with the cache — it is the pass that exists today. Run over the pilot's
report (742 records), and over one tranche-sized report, and compared with
`cmp -s`.

**A SINGLE DIFFERING BYTE VOIDS LEVER B**, and the cache is not repaired and
re-compared: a cache whose first byte-identity run failed is a cache whose
correctness is being fitted to the check.

The registered **consequence of disagreement** is stated here as
`docs/process.md` requires: lever B is abandoned, the sweep runs uncached, and
the disagreement is recorded as a finding about `newgame`'s isolation — because
under §2's argument a disagreement means state crossed an ask, which is a defect
in the ENGINE and not in the cache.

### 4.5 The decision rule

Lever B is taken **only if** §4.1's ratio is at least 1.5 **and** §4.4 returns
byte-identity on both reports **and** the cached capture's own measured
seconds-per-distinct-label is within 5% of the uncached seconds-per-label — the
last one excluding a cache whose bookkeeping ate its own saving.

---

## 5. THE COST OF THIS STUDY, ON ITS OWN FACE

`docs/process.md`: a pre-registration states what its governed run costs.

| part | ESTIMATED |
|---|---|
| the one play pass over openings `0..2` | ~5 s |
| lever A, 5 settings x 3 reps, ~171 s per process at N=1 and rising with N | **~1.1 h**, ESTIMATED at contention 1.0-2.0 |
| lever B §4.1's count | seconds — it reads a report |
| lever B §4.4's two byte-identity pairs | ~2 x (11 min + 3 h) if the tranche-sized referent is taken; **the tranche-sized pair is what makes it 6 h**, and it is registered anyway because §4.3 says why a sample is not enough |

**AGAINST WHAT IT COULD SAVE**: lever B's ESTIMATED saving is **3.26 h of the
7.15 h eight-way wall**, and the serial capture total falls from **48.95 h to
22.89 h** — both ESTIMATED by multiplying the pilot's MEASURED per-unit rates,
which is the same arithmetic `wp21_prereg.md` §3 uses and carries the same
limits.

---

## 6. WHAT THIS STUDY DOES NOT DO

1. **It does not parallelise a capture internally.** `capture::run` walks one
   channel by construction, and N concurrent tranches already give the box N-way
   parallelism — an internal split would buy the same throughput and add a
   deterministic-merge obligation for nothing.
2. **It does not touch the two seats.** The duplication is structural (D-558(2));
   removing it would mean a capturable shape the arena refuses.
3. **It makes no strength claim and produces no corpus.** Every artifact it
   writes is scratch and is deleted; nothing it produces may enter a corpus.
