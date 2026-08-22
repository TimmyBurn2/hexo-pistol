# WP-Q SEED — the quiet stage and its widening schedule

<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->

**HOW TO RESOLVE A `§n` IN THIS FILE.** Every `§n` is the SUPERSEDED document's
own numbering, kept unchanged so an existing citation still resolves. A `§n` that
names a section this unit does not own is prefixed with the unit that does
(**U1**–**U4**, or `WPQ_seed.md`) wherever it appears in prose written or
retargeted by the carve. Inside text carried VERBATIM — matrix cells, quoted
sentences, the seed — a bare `§n` may still name a section that now lives
elsewhere; `docs/experiments/section_owner_table.md` maps every one of them to
its owner, and that is what it is for.


**u-rev 2. THIS IS NOT A DESIGN AND IS NOT REVIEWABLE.** It is the text D-310
excised from WP-1.5b when option D cut the work package to stages F and T, kept
verbatim so the follow-up work package starts from what was written rather than
from memory. **Nothing here is selected, and nothing here may be cited as
adopted.** A reviewer asked to review this file should decline and say why: the
follow-up WP owes its own design, its own option matrix, its own
DECISION-RED-TEAM and its own SPRT (D-310), and this seed is an input to that
design, not a draft of it.

**Architect ruling R5 (settled).** This is where stage Q is picked up, not where
it is armed: **U2**'s node protocol carries Tier Q SPECIFIED BUT UNARMED,
WP-1.5b's shipped D-scope is stages F and T only, and the pre-registration
registers F+T only. (u-rev 2, one sentence, per D-311.)

**LABEL DISCIPLINE — D-311, travelling item T5.** Any append to this seed bumps
its u-rev, however small the diff. The rule binds a seed as it binds a unit,
because the follow-up WP's design will cite this text and a citation needs a
revision to name.

**Provenance.** `docs/experiments/wp15b_design.md` §7 at `6feb40a`, minus the two
bullets of §7.2 that are not about the schedule — those went to **U3** §7, where
a reviewer can reach them, because U2's
`a_radius_policy_search_is_byte_identical_to_the_committed_engine` rests on one
of them. Plus §12 item 3, three test rows, the WP-1.4 spread baseline, and two
ADR lines. Every **MEASURED** and **ESTIMATED** mark is the mark the superseded
text carried.

---

## THE M2 DEBT NOTE — read this before the text below

**M2 IS AN OPEN SELECTION AND HAS NEVER BEEN IN A MATRIX IN ITS ADOPTED FORM.**

- MEASURED, restructure red team F7: `W-E` occurs **zero** times at `ec8f7fb`.
  M2 at `ec8f7fb` held `W-A`..`W-D` and recommended **W-A**; W-A then FELL, and
  W-E was supplied by the DECISION-RED-TEAM that killed it.
- MEASURED, revision-7 review B1: the superseded document contains exactly three
  `| Option |` tables — **U1** §4.2, **U2** §5.6, **U3** §6.3. **§7 has none.** `W-B` and `W-C` occur
  zero times anywhere in it. The matrix was deleted at revision 2 and never
  restored.
- So M2 is a **FRESH matrix that has never been authored**, not a recovery. There
  is nothing at any revision to recover it from.

**What that means for the follow-up WP, stated so it is not rediscovered:** it
owes an option matrix over the widening schedule with every numeric claim marked
**MEASURED** or **ESTIMATED** (D-291, whose clause this work package has already
tripped three times), attacked by a fresh-context DECISION-RED-TEAM BEFORE any
option is selected, and only then an ADR line recording the strongest surviving
attack. The text below is the record of why W-A fell and of what W-E was written
to do. **It is evidence for that matrix. It is not that matrix.**

**And the strength debt is real, not escaped** — the strongest attack surviving
against option D, recorded in D-310: WP-1.5b's own SPRT delta shrinks by exactly
the axis D removes, and this follow-up WP is the only place that debt can be
paid. D-310 flips if this WP is never scheduled.

**One more debt travels with it:** a work-package cut is a ROADMAP change and
`docs/ROADMAP.md` changes only by ADR. The superseded §15 already owed two
unlanded ROADMAP-by-ADR lines (items 9 and 10); D-310's follow-up WP adds a
third, and item 9 below is one of the two.

---

## 7. MATRIX M2 — the widening schedule — FELL

### 7.1 Why W-A fell

Three structural facts, each confirmed by this session by reading `pvs.rs`:

1. **The root can never widen.** `iterate` opens at
   `visit(depth, -INFINITY, INFINITY, 0)` (`pvs.rs:134`) with `INFINITY = MATE+1`
   (`score.rs:39`); `best_score` starts at `-INFINITY` (`pvs.rs:266`) and the
   first completed child raises it, so `best_score <= original_alpha` is
   unsatisfiable. MEASURED: 0 root widenings in 101 root iterations.
2. **A non-PV node can never truncate.** A child opens at `(alpha, alpha+1)`
   (`pvs.rs:378/380`), so any `score > alpha` sets `alpha >= beta` and breaks
   (`pvs.rs:325`). Such a node either fails high early or exhausts its batch
   having failed low — and W-A then widens it to full width. MEASURED: 0 of
   2 022 904 non-PV interior nodes truncated.
3. **A PV node that truncates stores `Bound::Exact`**, which the probe consumes
   unconditionally at every later non-PV hit. An exact score over a SUBSET is a
   lower bound only.

**So W-A caps exactly where it must not — the root and the PV, where the move is
chosen and where D-124 says no oracle will catch a mistake — and widens exactly
where widening restores full width.** Revision 1's sentence "a node that … returns
an exact score inside the window is unaffected" is what concealed it.

The reviewer's prior question, which the matrix never asked: **in this recursion a
widening trigger that is SOUND cannot narrow anything.** Narrowing requires
accepting an unsound cut somewhere; the design must name where and defend it.

Two further claims of revision 1 fell with it. The worst-case bound — "no node is
ever more expensive than full-width plus the tier bookkeeping", offered as the
reason a reader could CHECK the option — is **MEASURED false**: a fully widened
node searches a mean **84.79** cells against the incumbent's **77.96**, 1.088×,
each extra cell a subtree. And the fail-low rate marked ESTIMATED and argued
unmeasurable took **23.5 s** to measure on the shipped recursion — D-291's clause,
the second instance this round.

### 7.2 ADOPTED: W-E

- **The root (ply 0) and every PV node (`beta − alpha > 1`) are NEVER capped.**
  They search the full staged universe. This closes both structural defects at
  once: no root cap, so the engine can always play any candidate it generates;
  and no PV node ever returns a value it has not proved, so no unsound
  `Bound::Exact` is ever written.
- **Non-PV nodes carry the cut**, as an escalating batch schedule whose **last
  entry is FINITE**. Revision 1's `[0]` / "all remaining" is precisely what made
  the widening vacuous. This is a forward prune of the LMR family and the ADR
  line calls it one — scoped by a stated rule, counted per node class, and
  SPRT-judged, which is what makes it not BARE.
- **The transposition store gains a truncation rule**, removing the poisoned-entry
  class rather than living with it. A subset maximum `>= beta` is a genuine lower
  bound, so **fail-high stores `Bound::Lower` as today**; a fail-low or exact
  score from a set that was **not exhausted** is unsound in the bound it claims,
  so **it stores nothing**. The lost TT entries are a rule-5 measurement (§12.3).
- **Mechanism.** The ordered universe is built ONCE per node; the loop runs over
  batches; a further batch is entered only on a fail-low so far. Widening searches
  only the ADDITIONAL cells at the same window — every already-searched cell
  scored at or below `original_alpha ≤ alpha`. (The argument is per direction: a
  cell scoring ≤ alpha did so via a child fail-high, whose `Bound::Lower` is valid
  even from a truncated set. Revision 1 asserted it undirected.)
- ~~`Cover::Impossible` at phase 1 inherits the same schedule~~ — **inert under
  revision 4 onward and struck**: **U2** §5.3 routes `Impossible` to the candidate loop
  only at a PV node or the root, and §7.2 exempts both from every schedule. A
  leftover from when `Impossible` could reach a non-PV loop.
- Revision 1 recommended `[16, 0]` in §7 and committed `[0]` in §10, and its own
  validator rejects `[16, 0]`. Both are withdrawn; §10 commits one schedule.

**Registered reading, with its denominator, depth and table size pinned before
the run** (revision 1's spanned 0.84 %–99.95 % on one instrument and one corpus
depending on an unpinned denominator, crossing both its thresholds — and a LOW
rate signified not health but that the cut never bound, so the criterion was
anti-correlated with the property it certified): the widening rate is reported
**per node class** (root / PV / non-PV), denominator = non-PV interior nodes that
ran the candidate loop and whose quiet pool exceeded the first batch, at depths 2
and 3, at the committed `tt_bytes`. **The falsifiable criterion**, whose defect
class is "the cap never bites": the count of non-PV nodes that exhausted the
schedule while still truncated must be non-zero, and it is reported.

**Recorded against this WP's own conclusion**, as the reviewer recorded it
against theirs: the incumbent's root argmax sat inside the first 16 ordered
candidates in 47/47 root iterations at depth 2 and 18/18 at depth 3, and only 1
PV node in 255 had its argmax past index 16. On this corpus a root cap would not
have changed the move played. That is not a licence — it is D-124's blindness
restated from the other side, and it is why the root exemption rests on the
argument above and not on that measurement.

---


---

## The test rows that come with the schedule

Carried from the superseded §11. They are registered by the follow-up WP, not by
WP-1.5b.

| Test | Watches |
|---|---|
| `widening_schedule_fires_on_fail_low_and_is_deterministic` | the per-class widening counters AND that a non-PV node exhausting a finite schedule while truncated exists. Two runs agreeing is a property the defect preserves and is not the criterion |
| `the_root_and_every_pv_node_search_the_full_staged_universe` | the emitted set size at ply 0 and at PV nodes against the unbatched universe |
| `a_truncated_fail_low_stores_no_transposition_record` | the table's contents after a node that stopped truncated |

---

## The measurement that comes with it

ADVISORY on the machine it was taken on; the operator re-runs for the record.

3. **The WP-1.4 adversarial-spread debt at `movetime 500` under Staged.**
   **BASELINE MEASURED** at `f317385`, release, `configs/play_v0.toml`:

   | stones | completed depth_turns | nodes | time_ms |
   |---|---|---|---|
   | 11 | **1** | 180246 | 499 |
   | 21 | **0** | 170259 | 499 |
   | 51 | **0** | 160321 | 499 |
   | 99 | **0** | 149839 | 499 |

   Depth 0 means no iteration completed. On this class no length-6 window holds
   two stones of either colour, so Tier F and Tier T are both EMPTY and Staged
   reduces to Tier Q batching over the same ball — the cleanest possible test of
   the cut alone, with every threat mechanism inert.

**The WP-1.4 spread baseline, MEASURED at `f317385` under `configs/play_v0.toml`**:
completed depth 1 / 0 / 0 / 0 at 11 / 21 / 51 / 99 stones under `go movetime 500`.
That is the debt this stage exists to move, and no AFTER exists, because no engine
code was written. It sits here rather than in a unit because on this class Tier F
and Tier T are both EMPTY — the measurement is of the cut alone, with every threat
mechanism inert, which is exactly what D-310 deferred.

---

## The ADR lines that come with it

Carried from the superseded §15. Item numbers are retained so an existing
cross-reference to "§15 item n" still resolves. **Neither may be written while M2
is an open selection**; they are recorded here as debt.

3. W-E, naming the non-PV cut as a forward prune, the TT truncation rule, and the
   cut's binding under `Staged` only.

9. **The dominance-pruning half of ROADMAP WP-1.5 is deferred, not dropped.**
   §14 says "no dominance pruning beyond the staged scheme"; the ROADMAP sentence
   that item 10 takes a line for reads "staged pair generation **with dominance
   pruning** SUPERSEDES the radius policy". The ROADMAP changes only by ADR, so
   both halves take a line, not one.

---

*WP-Q seed, u-rev 2. Not a design. Not reviewable. Nothing here is selected.*
