# Arc III — STOP summary, 2026-09-03. The design gate spent its fifth round.

> **ONE LINE FOR THE MORNING.** The label cache's design converged — round 5
> found **0 BLOCKING, 1 MAJOR, 2 minor**, the MAJOR being one wrong stub word in
> one test row with a one-word fix the reviewer verified — but a fifth failed
> round is a STOP under D-585 and the dispatch, so nothing of the cache is
> implemented, no sweep has run, and **the decision owed is what to do with a FAIL
> of this shape**: a scoped verification pass for the one-word remedy (D-568's
> shape), a split, or a re-grant. The sweep, the census and the anchor are all
> downstream of it.

## The numbers

| what | value |
|---|---|
| distinct labelled positions delivered | **0** — the sweep did not start |
| census win-direction count vs D-537's minimum of 28 | **not started** |
| anchor v3 W/L | **not run**; its platform capability is unimplemented |
| design review rounds spent | 5 of 5; finding counts 22, 15, 16, 11, 3 |
| assembler tools/ review | round 1 FAIL 0B / 6M / 7m; round 2 remedies written, eleven tests, **unreviewed** |
| CI | last green `artifacts/arc3_ci_198_v3.txt` at `3d559b0` under rustc 1.98.0, twenty gate lines; **not re-run since** |
| `rustc --version` | 1.98.0 (88d9e12ae 2026-08-18), unchanged through this session |

## What landed on `dev` (`5c77a8f`, clean)

- `wp21_label_cache_design.md` revision 6 — rewritten whole at revision 5 from the
  settled mechanism, remedied at 6; every citation reproduces; mechanism, refusals,
  mutants and D-586 survived two fresh reviews.
- **D-586**: D-581's flip clause gets a number (one percent of the cached
  tranches' misses beyond each tranche's own book floor), the counters' cost
  corrected, *"settles D-562(2)"* withdrawn. **D-587**: D-586's one misquote of
  D-581 corrected.
- `tools/wp21_assemble.py` and its six-test suite, revision 1.

## What is on `arc3-stopped` (this branch, never `dev`)

- Both review reports: `wp21_label_cache_design_rev6_REVIEW.md`,
  `wp21_assemble_REVIEW.md`.
- The assembler's round-2 remedies and eleven tests — WIP, unreviewed.
- `wp21_prereg_rev5_DRAFT.md`, `wp21_throughput_prereg_rev4_DRAFT.md` — whole
  rewrites answering every round-2 finding of both registrations, marked NOT
  GOVERNING; slots empty; each names the generator's `--pilot-range` form it needs.
- `tools/governing_citation_check.sh` without the assembler's stale `--proposes`
  entry; the ledger's §1e; this summary.

## Decisions taken as architect defaults this session

1. The design rewritten whole as revision 5 at 56% of revision 4's words, not
   "well under half": the rest is tables each row of which answers a finding.
2. D-586 corrects the matrix in the log rather than by a fourth matrix revision.
3. The assembler written during the design review, independent of the cache,
   tested in a detached worktree on `/home`.
4. The assembler's deduped output is an INDEX into the raw corpora, not a merged
   corpus (sixteen tranches, sixteen experiment digests).
5. The STOP taken as written rather than read as a sixth round.

## Errors made, and how each was found

- **T6's forfeit fixture named a stub that cannot be captured** — adopted from
  round 4's suggested fix without reproducing the fix; found by round 5. The
  session had written *"every finding reproduced before its fix"* the same hour.
- **"No engine in the tree writes an unsolicited line"** — counted the word, not
  the behaviour (`protocol_abuse_tests.rs`'s `doubled.sh`); found by round 4.
- **The stray placed at a miss** — `seats::with_seats` sends a `newgame` per
  spawn before the capture's per-ask one; an off-by-one found by round 4.
- **D-586 misquoted D-581** by one phrase; found by round 4, corrected by D-587.
- **The assembler's first revision** accepted a corpus twice, a schema it does not
  read, a result token outside the closed set, and counted disagreements
  order-dependently; all found by its tools/ review with reproducers.
- **The registrations' dry runs registered a generator window the generator
  refuses** (`skip 0` is below the sweep's first opening 13) — found while drafting
  their next revisions, not by a run; the `--pilot-range` form is the remedy.

## Decisions owed to the operator

1. **The design gate.** FAIL at round 5 with one MAJOR that is one word. Options:
   a scoped verification pass over the one-word remedy plus minors A/B (D-568's
   precedent), a split that lands the design without T6(a), or a re-grant.
2. **Minor A's mechanism question**: whether the capture pass gains a drain after
   `quit` so a stray at a hit is caught deterministically (a mechanism change), or
   T4's cached arm keeps its microsecond residual, stated.
3. **D-588**: the correction to D-581's gate-9 sentence, stated in the ledger §1e,
   is appended when the throughput registration that cites it lands.

## Exports

Gitignored artifacts of this session, with digests, all under `artifacts/`:

```
7b88f9895d0bfc6614bda74053efe194c6f98938056068c014b159a457cc0a39  arc3_wp21_assemble_worktree_test.log   (round 1: 6 passed, at b07d774)
511e1527431b10051f4bb5278759120d28334fad8ce92f679a474e9ca530eb0f  arc3_wp21_assemble_test_r5.log             (round 2: 11 passed, 0 failed, at this branch's head)
```

No worktree exists; the verification worktree at `b07d774` held nothing
gitignored but its target directory and was removed after its log was copied out.
No process of this project's is running.
