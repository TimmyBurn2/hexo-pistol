# REVIEW-design — WP-1.5b U1 (gate supersession), u-rev 2 (re-review of a repair)

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer (only HTML-escaped shell redirections restored to `2>&1`). The
dispatching session repaired U1; it did not review it. Reports live in the tree
rather than in a scratchpad because finding IDs are cited across units.
-->

## Header

- **Pinned revision:** `5baea10`
- **Matches HEAD:** YES — `git rev-parse HEAD` = `5baea10d2e91022fc0dec89708bd7617f105eeec`.
- **Document:** `docs/experiments/U1_gate_supersession.md`, u-rev 2 (330 lines), authored at `5baea10`, repairing u-rev 1 (`cf74594`).
- **Prior round:** `docs/experiments/wp15b_U1_REVIEW.md`, pinned `38f21b9`, FAIL (1 BLOCKING, 0 MAJOR, 1 MINOR).
- **Reviewer context:** fresh — no prior turns in this conversation touched the pistol repo before this review.
- **Scope:** re-review of the repair only, per dispatch — is BLOCKING 1 repaired (re-measured, not trusted), is MINOR 2 repaired by disclosure without taking a side, fold-in law across the whole document, and any out-of-scope content added under cover of the repair. B6's `pwd -P` fix, T7 and M0's selection are NOT re-litigated — the prior round cleared them and the repair diff (`git diff cf74594 5baea10`) does not touch those sites.

## VERDICT: **PASS**

0 BLOCKING, 0 MAJOR, 0 MINOR.

---

## Findings

None reach a reportable severity. One borderline wording item was considered and rejected (below).

---

## Verified with no finding

### 1. BLOCKING 1 (stale MEASURED test counts) — repaired, re-measured independently

Re-ran both commands myself against `5baea10`, not the document's pasted output:

```
$ cargo test -p pistol-cli --test solver_edge_check_tests -- --list 2>&1 | tail -3
the_shipped_workspace_has_no_normal_edge_on_the_solver: test

11 tests, 0 benchmarks
```
```
$ cargo test -p pistol-cli --test solver_link_check_tests -- --list 2>&1 | tail -3
no_solver_source_reaches_any_shipped_binary_of_this_workspace: test

19 tests, 0 benchmarks
```

MEASURED: 11 and 19, exactly matching §4.1's re-measured claim. §4.2 row (a)'s derived figure: (11−1)+(19−1) = 10+18 = **28** — MEASURED arithmetic, matches the document's "28", correctly re-tagged **DERIVED, MEASURED** (an improvement on the fix scope's minimum ask of "re-tagged MEASURED"; not a new defect — it still carries the required MEASURED mark).

Also confirmed the document's claim that its cited `HEAD 4a23677` is unchanged from the review's pin `38f21b9` for the two test files:
```
$ git rev-parse 5baea10^  →  4a23677
$ git diff --stat 38f21b9 4a23677 -- crates/pistol-cli/tests/solver_edge_check_tests.rs crates/pistol-cli/tests/solver_link_check_tests.rs
(empty — no changes)
```

### 2. MINOR 2 (silent SHA resolution) — repaired by disclosure, no side silently taken

U1-A's new paragraph (lines 56–65) states the row uses `2d07ff6` ("the header's SHA"), cites `wp15b_design_rev7_REVIEW.md` MAJOR finding 11 by name, states the two SHAs differ by 4 lines in §0 row 8 and §8.1, and explicitly disclaims resolving which is correct. Checked against finding 11 itself:

```
$ git show 5baea10:docs/experiments/wp15b_design_rev7_REVIEW.md | sed -n '/### 11\./,/^### 12/p'
```
confirms: `9c068a0` (§16) vs `2d07ff6` (header), reproducer "`git diff 2d07ff6 9c068a0` → 4 insertions, 4 deletions, in §0 row 8 and §8.1", reviewer "explicitly declined to pick a side." The repair's disclosure text is accurate to the finding it cites and takes no new position — it names which SHA the row already used and why (the header's), not which is "correct." Repaired as required.

### 3. Fold-in law — no stale restatement found

Grepped the whole document for every candidate site (test-count digits 9/11/19/26/28, the two revision-6 SHAs, `u-rev`, and review-status language FAIL/BLOCKING/MINOR/outstanding):

```
grep -n '\b9\b\|\b26\b\|\b28\b\|\b11\b\|\b19\b' docs/experiments/U1_gate_supersession.md
grep -n '2d07ff6\|9c068a0' docs/experiments/U1_gate_supersession.md
grep -n 'u-rev' docs/experiments/U1_gate_supersession.md
grep -n 'FAIL\|BLOCKING\|MINOR\|REVIEW-design\|outstanding\|landable' docs/experiments/U1_gate_supersession.md
```

Every site is consistent: §4.1 states 11/19 with the correction history; §4.2 row (a) states 28; U1-A states `2d07ff6` with the disclosure; the header, the review-status paragraph, and the closing line all read "u-rev 2" and correctly describe u-rev 1 as FAILED-and-repaired rather than PASSED. The one leftover "9" (line 53, "7 BLOCKING, 7 MAJOR, **9** MINOR") is a distinct referent — the revision-7 review's own finding count, unrelated to the test-count repair — confirmed unchanged and correct by the prior round's own verification (line 165–166 of the prior review). No cross-document reference to U1's u-rev label or test counts exists to go stale:
```
grep -n "U1.*u-rev\|u-rev.*U1" [section_owner_table.md, WPQ_seed.md, U2/U3/U4 docs, restructure_selection_15b.md, wp15b_census.rs]  →  no output
```

### 4. No content added beyond the two findings' scope

`git diff cf74594 5baea10 -- docs/experiments/U1_gate_supersession.md` shows exactly six edit sites: the u-rev bump (line 15), the review-status paragraph (lines 29–39, replacing "NOT BEEN REVIEWED"), the new SHA-disclosure paragraph (U1-A, lines 56–65), the §4.1 re-measurement block (lines 93–130), the §4.2 row (a) figure (line 136), and the closing line (327–329). Every site is either one of the two named findings' repair or a direct fold-in consequence of it (the u-rev bump and closing line are required by CLAUDE.md's label discipline once any append happens). No unrelated prose, no new matrix rows, no new ADR content.

---

## Rejected with attempted reproducer

**Candidate: "this count was itself stale at 9/19 as of u-rev 1" (§4.1) loosely implies both numbers were stale.**

Text: *"...**19** (revision 1 said 8 and 20 — transposed; this count was itself stale at 9/19 as of u-rev 1, per `wp15b_U1_REVIEW.md`'s BLOCKING finding 1... — the edge count rose from 9 to 11 when B6's fix added two tests...)"*.

Checked against the prior review, which found only the **edge** count (9) stale — the **link** count (19) was confirmed correct at u-rev 1 already ("`solver_link_check_tests.rs` is unaffected... correctly reports 19, matching the document"). Read in isolation, "stale at 9/19" could suggest both numbers changed. Rejected as a finding because: (a) the document's own next clause immediately disambiguates — "the edge count rose from 9 to 11" — naming only the edge test as having moved; (b) no numeric value anywhere in the document actually misstates 19 as having been wrong or as differing from 19; (c) this is loose phrasing describing "the u-rev-1 version of this figure pair" (which literally read "9" and "19"), not a false claim about which half was defective. No reproducer contradicts any number in the text — only a stricter reading of one clause's antecedent is available, which the same sentence resolves. Not BLOCKING/MAJOR/MINOR; noted for completeness since the task asked for rigorous scrutiny of exactly this repair.

---

## Summary for the caller

U1 u-rev 2 is a clean, proportional repair: BLOCKING 1 is genuinely fixed (independently re-measured 11/19/28, all correct), MINOR 2 is genuinely fixed (disclosed accurately against finding 11, no side taken), the fold-in law holds with no stale restatement anywhere in the document or in any external citer, and nothing was added beyond the two findings' scope. The document remains honestly self-flagged as still unreviewed at this u-rev pending this very review, which now returns PASS for this repair.
