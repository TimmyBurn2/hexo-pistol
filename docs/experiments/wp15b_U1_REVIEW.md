<!--
PROVENANCE.
- Pinned SHA: 38f21b9 (branch dev). Confirmed still HEAD at review time
  (`git rev-parse HEAD` = 38f21b93217f639772ad4235d84a8c666ef5bd91).
- Document reviewed: docs/experiments/U1_gate_supersession.md, u-rev 1
  (274 lines, single commit cf74594, no append since).
- Reviewer context: FRESH. No prior turns in this conversation touched the
  pistol repo; this session read CLAUDE.md, the named background documents,
  and the pinned revision's tree before forming any judgement.
-->

# REVIEW-design — WP-1.5b U1 (gate supersession), u-rev 1

## Header

- **Pinned revision:** `38f21b9`
- **Matches HEAD:** YES — `git rev-parse HEAD` = `38f21b93217f639772ad4235d84a8c666ef5bd91`
  at review time.
- **Document:** `docs/experiments/U1_gate_supersession.md`, u-rev 1, 274 lines,
  single authoring commit `cf74594` (the carve), unappended since.
- **Reviewer context:** fresh — no memory of this document beyond what this
  review's own reading established.
- **Scope:** per dispatch — (1) §4 lineage checkability, (2) B6 / `pwd -P` fix
  verified against the shipped script, (3) T7 build-order note, (4) M0
  selection consistency with its own matrix.

## VERDICT: **FAIL**

1 BLOCKING, 0 MAJOR, 1 MINOR. T7 and B6's fix are both independently
VERIFIED CORRECT against the shipped tree. M0's adopted option is consistent
with its own matrix and lineage. The BLOCKING finding is a stale MEASURED
test-count, of the exact same shape (a repaired fact in one place, an
un-repaired restatement of it in another) that produced B4/B5/B6 of the
revision-7 review this unit descends from.

---

## Findings

### 1. §4.1's "9 tests" and §4.2's derived "26 scratch tests" are false at the pinned revision — BLOCKING

**Claim reviewed** (§4.1): "Of `solver_edge_check_tests.rs`'s **9** tests and
`solver_link_check_tests.rs`'s **19** (revision 1 said 8 and 20 — transposed),
exactly ONE each is an assertion about this workspace." Tagged **MEASURED**.

The same "9" feeds an unmarked derived figure in §4.2's matrix, option (a):
"Delete both live assertions; keep both scripts and their **26** scratch
tests" (= (9−1) + (19−1)).

**Contradicting evidence.** The B6 fix this SAME unit document discharges in
§4.4 ("**LANDED at `8af9c5b`**... with `a_colour_forcing_environment_leaves_
no_escape_sequence_in_the_printed_tree` driving the shipped script... with
`a_symlinked_workspace_root_is_still_substituted_out_of_the_printed_tree`
driving the shipped script") added exactly two new tests to
`solver_edge_check_tests.rs`. §4.1/§4.2's counts were never updated to match.

**Reproducer, at the pinned revision:**

```
$ cargo test -p pistol-cli --test solver_edge_check_tests -- --list 2>&1 | tail -3
a_workspace_with_no_edge_at_all_is_accepted: test
an_unreadable_workspace_voids_the_run_by_its_own_reason: test
the_shipped_workspace_has_no_normal_edge_on_the_solver: test

11 tests, 0 benchmarks
```

11, not 9. (`solver_link_check_tests.rs` is unaffected — `cargo test -p
pistol-cli --test solver_link_check_tests -- --list` correctly reports 19,
matching the document; the fix commit touched only the edge-check pair.)

The document's own "exactly ONE each is an assertion about this workspace"
sub-claim survives — confirmed by grep:

```
$ grep -n "repo_root()" crates/pistol-cli/tests/solver_edge_check_tests.rs \
                          crates/pistol-cli/tests/solver_link_check_tests.rs
crates/pistol-cli/tests/solver_edge_check_tests.rs:288:    let ran = edge_check(&repo_root(), "pistol-solver");
crates/pistol-cli/tests/solver_link_check_tests.rs:735:    let ran = link_check(&repo_root(), "crates/pistol-solver");
```

— exactly one each, both of the two new tests are scratch-workspace tests
(`workspace("edge-symlink", ...)`, `workspace("edge-colour", ...)`), so the
"what expires is two test functions" structural argument is not itself
broken. But the correct current scratch-test count is (11−1) + (19−1) = **28**,
not the 26 the matrix states, and neither the "9"/"26" total nor its
correction is marked MEASURED or flagged stale anywhere in the document.

**Why it breaks.** This is the identical shape to B4, B5 and B6 of the
revision-7 review that closed the superseded document (a fact repaired in one
section — here, §4.4's own "LANDED at `8af9c5b`" — left un-propagated to a
sibling section's MEASURED number), which is the recurring defect class
CLAUDE.md's process exists to catch and D-305 records as this document's own
base rate. Scope item 4 asks specifically whether M0's matrix cells carry
accurate, marked numbers; row (a)'s "26" does not, and it is wrong. It does
not change B6's discharge, T7's binding order, or M0's adopted option — all
three were independently verified correct in this review — but a MEASURED
claim that is false, sitting two paragraphs from the fix that falsified it, is
exactly what this project's review history treats as BLOCKING (B4, B5, B6 were
all this shape and all BLOCKING) rather than cosmetic.

**Fix scope:** update §4.1's "9" to "11" (and its "revision 1 said 8 and 20 —
transposed" aside, if it still needs the note), and §4.2 row (a)'s "26" to
"28", both re-tagged MEASURED.

---

### 2. U1-A's lineage table silently resolves an unresolved SHA dispute (rev-7 review finding 11) for "revision 6" — MINOR

**Claim reviewed** (U1-A): "REVIEW-design ×5 | revisions 2–6, `182f389`
`7ad466b` `f762c9a` `64af80c` `2d07ff6` | all FAIL, none on M0's merits; no
round reopened M0."

**Contradicting evidence.** MAJOR finding 11 of `wp15b_design_rev7_REVIEW.md`
(the review that closed the superseded document, cited by D-309) found the
superseded document self-contradictory about which SHA "revision 6" actually
is: its own §16 cited `9c068a0` for the revision-6 REVIEW-design round, while
its header listed `2d07ff6`. The two differ by 4 lines (§0 row 8, §8.1) and
the reviewer explicitly declined to pick a side: "Either §16 misnames the
reviewed revision, or the header misnames revision 6." That finding was never
discharged anywhere in the tree — `9c068a0` occurs in exactly two files at the
pinned revision (the review report itself, and `wp15b_sprt_prereg.md`, whose
own subject it is), and nowhere in `docs/decisions.md` or any carved unit is
the dispute mentioned or resolved.

**Reproducer:**

```
$ git show 38f21b9:docs/experiments/wp15b_design_rev7_REVIEW.md | sed -n '249,262p'
### 11. §16 names a revision-6 review SHA the header contradicts, and the two SHAs differ
...
- **Why it breaks:** ... The document gives two different SHAs for the text
  revision 6's reviewer saw, and they are not the same text. Either §16
  misnames the reviewed revision, or the header misnames revision 6 ...

$ for f in $(git ls-tree -r 38f21b9 --name-only docs/); do \
    git show 38f21b9:"$f" | grep -q "9c068a0" && echo "$f"; done
docs/experiments/wp15b_design_rev7_REVIEW.md
docs/experiments/wp15b_sprt_prereg.md
```

`2d07ff6` and `9c068a0` are both real, reachable commits (`git cat-file -t`
returns `commit` for each), so the *referent* exists either way — this is a
resolution/attribution gap, not a broken pointer. Note also that U2-A, facing
the same underlying ambiguity, sidesteps it by collapsing "REVIEW-design |
revisions 3–6" into one un-SHA'd row rather than picking a side; U1-A is the
more specific (and thus more exposed) of the two.

**Why it breaks.** Scope item 1 asks whether the lineage is "checkable as
stated" rather than resting on an unstated assumption. Silently adopting one
horn of an explicitly-recorded, explicitly-unresolved dispute — without citing
finding 11 or stating which side was taken and why — leaves a reader unable to
tell whether the choice was deliberate or accidental. **Correctly out of
scope for this table's content, though:** the disputed 4-line diff sits in §0
and §8.1, neither of which U1 owns, and it touches no claim about M0 or the
gate scripts. MINOR.

---

## Scope items verified with no finding

- **§4 lineage, otherwise.** All cited SHAs (`ec8f7fb`, `182f389`, `7ad466b`,
  `f762c9a`, `64af80c`, `6feb40a`, `eea480b`, `d6f6cbb`, `f317385`) exist and
  are reachable. The claim "B6 is M0's; no other finding is §4's" was checked
  against the full text of `wp15b_design_rev7_REVIEW.md` (all 7 BLOCKING, all
  7 MAJOR, all 9 MINOR, matching the document's own "7/7/9" count) — confirmed
  true; only B6 names §4/M0. U1-B's claims about U2's content
  (`win_in_one_ply_cells`, `can_win_this_turn`, `blocking_covers`, all defined
  in `crates/pistol-solver/src/{query,cover}.rs`, and `pvs::visit` in
  `docs/experiments/U2_node_protocol.md`) are stated as checkable facts and do
  check out, satisfying scope item 1's "named imports a reader of THIS unit
  can check" requirement.

- **B6 / `pwd -P` fix — VERIFIED against the shipped script, not the
  document's claim.** `tools/solver_edge_check.sh:117` (current) reads
  `ROOT_ABS="$(cd "$ROOT" && pwd -P)"`, and all three `cargo tree` invocations
  (lines 83, 89, 94) carry `--color never`. Driving the shipped script through
  `crates/pistol-cli/tests/solver_edge_check_tests.rs` (`Command::new("bash")
  .arg(repo("tools/solver_edge_check.sh"))` — not a copy, not a
  reimplementation):

  ```
  $ cargo test -p pistol-cli --test solver_edge_check_tests -- --nocapture 2>&1 | tail -5
  test a_symlinked_workspace_root_is_still_substituted_out_of_the_printed_tree ... ok
  test a_normal_dependency_edge_is_refused_and_the_dependent_is_named ... ok
  test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  ```

  RED reproduced independently, in a `/home` worktree (per CLAUDE.md's
  worktree-isolation rule) at `08cf4f7` — which is `8af9c5b^`, exactly the
  commit §4.4 cites — with the tests from `8af9c5b` run against the unrepaired
  script:

  ```
  $ git worktree add --detach /home/tom/verify-wt/u1review 8af9c5b
  $ cd /home/tom/verify-wt/u1review && git show 8af9c5b^:tools/solver_edge_check.sh \
      > tools/solver_edge_check.sh
  $ CARGO_TARGET_DIR=/home/tom/verify-wt/u1review-target cargo test -p pistol-cli \
      --test solver_edge_check_tests -- --nocapture 2>&1 | tail -20
  thread '...a_symlinked_workspace_root...' panicked: the dependent is named under
    the substituted root: ... (/tmp/pistol-testscratch-.../ws/crates/thecrate) ...
    /tmp/pistol-testscratch-.../ws/crates/user) [no <workspace> substitution at all]
  thread '...a_colour_forcing_environment...' panicked: an SGR escape reached the
    record: "...\u{1b}[2m└──\u{1b}[0m user..."
  test result: FAILED. 9 passed; 2 failed
  ```

  This matches the document's own description of the RED run almost verbatim
  (the escape sequence, the unsubstituted `/tmp/pistol-testscratch-...` path).
  `SHELL_CHECKLIST.md` item 10 (the coverage rule — "any `tools/` script that
  produces a recorded number carries at least one test driving the shipped
  script") is met: both tests drive the shipped script directly, in scratch
  workspaces, each with its own control assertion, exactly as item 10
  requires. Item 12 (void-vs-fail) is also satisfied by these two tests: both
  assert `Some(1)` by name rather than bare success. **B6 is genuinely
  discharged.**

- **T7 (IMPL-order note) — VERIFIED against the manifests.** U1-B states
  `pistol-solver` is absent from `[workspace.dependencies]` and that
  `crates/pistol-search/Cargo.toml` takes only `pistol-core` and
  `pistol-eval`. Confirmed at the pinned revision:

  ```
  $ git show 38f21b9:Cargo.toml | sed -n '/workspace.dependencies/,/^$/p'
  pistol-core = { path = "crates/pistol-core" }
  pistol-eval = { path = "crates/pistol-eval" }
  pistol-search = { path = "crates/pistol-search" }
  pistol-engine = { path = "crates/pistol-engine" }
  pistol-cli = { path = "crates/pistol-cli" }
  ...
  $ git show 38f21b9:crates/pistol-search/Cargo.toml | sed -n '/\[dependencies\]/,/^$/p'
  [dependencies]
  pistol-core = { workspace = true }
  pistol-eval = { workspace = true }
  ```

  `pistol-solver` absent from the table, confirmed; `pistol-search` takes only
  the two crates named, confirmed. (Note: `pistol-solver` IS a workspace
  *member* via the `crates/*` glob and has its own Cargo.toml depending on
  `pistol-core` — but that is membership, not the shared-dependency-table
  entry the claim is about, and the claim is precise about which one it
  means.) The note is present as U1-B, stated as binding ("The binding order:
  U2's IMPL lands before U1's gates are armed... Neither is a defect in either
  unit"), matching T7 from `restructure_selection_15b.md` verbatim in
  substance.

- **M0 selection vs its matrix.** §4.2's matrix table is present (one of the
  three `| Option |` tables B1 of the rev-7 review confirmed exist), option
  (f) is marked ADOPTED in §4.4, and this is the same option U1-A's lineage
  table says the original DECISION-RED-TEAM (at `ec8f7fb`) produced against
  (e) — consistent with `docs/decisions.md` D-305's independent record ("Four
  matrices in one work package recommended an option a fresh context then
  dominated — **M0 (f)**..."). The ADR line for M0/option (f) is honestly
  flagged as **not yet landed** ("item 1 is this unit's own and has not
  landed") — correctly scoped OPEN, consistent with "a WP is not landable
  while its reviews are outstanding" (this is that outstanding review). Every
  numeric cell present in the M0 matrix (aside from finding 1's stale "26") is
  tagged MEASURED; no unmarked ESTIMATED-shaped number was found, so no
  D-291-shaped finding beyond finding 1 applies.

---

## What this review does not re-litigate

Per dispatch scope, this review did not attempt to reproduce §4.3's `NOTES.txt`
`include_str!` construction (option (e)'s fall) or the `f317385`
gate-expiry re-measurement's exact hit counts ("6 tree lines", "30 hits over 5
binaries") — both are historical re-measurements in scratch worktrees whose
SHAs exist and are reachable but whose exact figures were not independently
redone here, since neither bears on B6, T7, or M0's adopted option, which were
the review's binding checks.

---

*Reviewed against `38f21b9`. Fresh context throughout.*
