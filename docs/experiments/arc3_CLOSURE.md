# Arc III — CLOSURE, 2026-09-04. The sweep ran and the corpus is delivered.

> **ONE LINE FOR THE MORNING.** The production label sweep ran to completion:
> **sixteen tranches of sixteen PASSED, no VOID, no re-run, and the corpus holds
> 89 805 distinct labelled positions** — 96.5 % of what the registration
> estimated — with every criterion met on every tranche and the label cache's
> byte-identity verified on a tranche-sized workload. **What did NOT close is the
> census count against D-537's floor of 28**, which needs two registrations that
> do not exist and which was unblocked, not done, by the corpus landing; and
> **anchor v3**, still blocked on a platform capability nobody has built.

## The numbers, first

| what the dispatch asked for | delivered |
|---|---|
| **distinct labelled positions** | **89 805** (registration ESTIMATED 93 076; 96.5 %) |
| records | 195 874 (ESTIMATED 199 027; 98.4 %) |
| decided subset, outcome coverage | 46 653, **0.5195** |
| duplication factor | **2.1811** (2.1383 on the pilot) |
| key disagreements | **2 369** — the first reading at scale of D-562(2)'s open question |
| **census win-direction count vs 28** | **NOT TAKEN.** The sweep ran census OFF by D-596; the count needs its own run, and that run needs two registrations that do not exist |
| **anchor v3 W/L, distinct-game, distinct-opening** | **NOT RUN.** Blocked on a platform capability, as it was before this dispatch began |

## What ran, and what held

**The sweep.** Sixteen tranches launched together at 12:38 UTC on 2026-09-03,
detached, polled, the box otherwise idle; last verdict 20:00 UTC. **7 h 22 m**
against §3's ESTIMATED 6 h 10 m, the overshoot exactly where the registration
said it would be — capture was MEASURED at the sweep's own concurrency and the
other three terms kept uncontended rates and were declared lower bounds.

**Every criterion, every tranche.** T-A1 and T-A2: every sampled record agreed
byte for byte in both cache classes, 28–30 misses and 32–35 hits per tranche,
each above the ten-sample floor this arc added. T-B: 0 divergences over 6 974
games. T-C: zero forfeits, `end normal` throughout. T-D: every corpus read by the
shipped loader. **And the cache's absence is mechanical, not asserted**: every
block carries `asks == records`.

**T-F passed both halves.** Two captures of the twenty-opening sub-range were
byte-identical at 1 106 records; then those 1 106 records matched tranche one's
own first 1 106 exactly — a separately played report reaching the same bytes,
which is the cross-report referent the registration's later revisions registered
rather than the same instrument twice.

**The label cache's capability is verified.** Tranche one re-captured with the
cache on is **byte-identical** to its uncached capture, 12 362 records, 6 620
answered from the memo. Its counters gave D-586's flip clause its first real
reading: `key_full_collisions` **42** against tranche one's registered floor of
**42**, and `key_pos_collisions` **0** — the symmetry fold merged nothing beyond
the opening book's own shape, so the yield beyond the book is zero and the
one-percent line is nowhere near. `fold_ms 110` makes the counters' cost MEASURED
and negligible, which D-586 could only estimate.

**No tranche of this sweep used the cache**, and that is a consequence of lever A
rather than a failure: the measured optimum was sixteen concurrent tranches, one
wave, and a gate that needs tranche one's uncached capture cannot admit a tranche
of a sweep that ends with it.

## The decisions this arc took

**D-588 to D-594** — the operator's seven rulings, appended before any other tree
work: the design gate closed at revision 6; **a design governs mechanism and
fixture-level claims are IMPL's, adjudicated by execution**; a remedy naming a
fixture or a command is executed before the revision carrying it is dispatched;
reviewers run cargo in their own worktrees and REVIEW-impl must; the drain after
`quit` is not taken; the gate-9 correction; and the STOP of 2026-09-03 stands as
taken.

**D-595 to D-598** — the residual measured rather than estimated and T-4's cached
arm made five runs; a review killed by the harness is **VOID and spends no round
of a grant**; a terminal-round failure whose findings are all corrections is
disposed of by a scoped verification over the corrected lines **and their defect
class**, never by reading the failure away.

**D-599 to D-602** — the stale cross-document revision citation: the check that
catches it, the correction of its own history, **the withdrawal of the count as
prose that licenses nothing**, and the rule that a search reported at a revision
is run at that revision, because a working tree is not a revision.

## What this arc kept getting wrong, stated once

**A claim transcribed instead of derived, and a claim checked against the wrong
population.** Both are named in `docs/process.md`, and this arc produced them
repeatedly *inside the very lines diagnosing them*: a file count copied from a
reviewer's prose; a defect class counted three times over three populations, each
narrower than the class; a search labelled with a revision it was not run at. The
sustainable answers taken were not more diligence but structural: **delete the
number that licenses nothing** (D-601), **report the search rather than its
result** (D-601), **run it at the revision you name** (D-602), and **build the
mechanism where the class is mechanical** (the revision-citation gate).

## The instrument this arc added

**The label cache** (`crates/pistol-arena/src/label_cache.rs`, the mode in
`capture::run`, `--label-cache`), reviewed twice with a RED-TEAM that found no
wrong byte and a **mutation receipt in which twenty of twenty non-equivalent
mutants die at the row the design names**. The cold checker's ten-sample floor.
The generator's `--pilot-range` form. The assembly instrument, three tools/ review
rounds. **And gate 20's second half**, the revision-citation check, whose own
review found two exit-0 wrong answers in it before it landed.

## Receipts

**CI green at HEAD.** `tools/ci.sh` at `486aaa4` in a detached worktree on
`/home`: **twenty gate lines, `gate 1/20` through `gate 20/20`, `ci: all gates
passed`, `CI_EXIT=0`**, under `rustc 1.98.0 (88d9e12ae 2026-08-18)`. Log
`artifacts/arc3r_ci_486aaa4.txt`, sha256
`3f59d85469175ce004ff5c867628a244ecd92d508126c41bc282c85fb157c5a8`.

**Exports** (rule 8, D-469): the sixteen corpora stay out of the tree under
`/home/tom/pistol-runs/arc3r-sweep/` and are sha-indexed in `arc3_ledger.md` §5;
the run log, both manifests and the cached re-capture are in `artifacts/` with
digests there. The mutation driver, which lived only in a worktree the receipt
pointed at, is exported to `artifacts/arc3r_mutation_driver.py` before removal
could take it.

**Worktrees**: all removed. `git worktree list` holds the main tree alone.

## What is owed, and to whom

1. **The census count against D-537's floor of 28.** It needs a **cap calibration
   registration** (which was waiting on the deduped corpus — now delivered) and a
   **census run registration**, neither of which exists. This is the arc's open
   question and it is a dispatch of its own, not the tail of this one.
2. **Anchor v3.** Blocked on a platform capability. Unchanged by this arc.
3. **The 2 369 key disagreements.** D-562(2) leaves open which of the three keys
   rules such a pair, and this sweep is the first run to measure it at scale. The
   corpus is unaffected — its dedup required all three keys to agree — but the
   ruling is now answerable with data instead of argument.
4. **Two contrived-stream minors in the assembler**, accepted and queued rather
   than landed under a registration in flight (`arc3_ledger.md` §2.4).
