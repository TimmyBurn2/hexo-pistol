# WP-2.2 §0 — first actions, with their receipts

## §0.1 The thirteen worktrees are removed

**Before**: `git worktree list` held the main tree and thirteen others —
`arc3-base`, `ci-arc`, `i2`, `p1-measure`, `p1-mutants`, `p2`, `p3`, `s1`,
`s2`, `s3`, `t1-ci`, `w1`, `w2`.

**The export receipt was verified before anything was removed, not assumed.**
`artifacts/opt_arc_worktree_export/RECEIPT.sha256` holds **219** lines, its own
sha256 is `a3e36bd64ab6ad41811dcb87a59222bdc18f1c518a10e4f58bc79921dfdbf775`
(the digest `opt_arc_CLOSURE.md` records), and `sha256sum -c` over it reported
**no failures**. Per-worktree counts from the receipt matched the live counts
under each worktree's `artifacts/` and `sessions/` exactly, worktree for
worktree.

**WHAT THE RECEIPT DID NOT COVER, FOUND BY LOOKING RATHER THAN BY TRUSTING IT.**
The receipt covers `artifacts/` and `sessions/` only. Five worktrees also held
untracked files OUTSIDE those two directories, and `w2` held two modified
tracked files:

- `i2`, `p1-measure`, `p2`, `p3`, `w2` — session logs, suites, probe outputs
  and generator scripts, plus compiled binaries, `perf` `.data` files and five
  `target-*` directories.
- `w2` — `crates/pistol-engine/tests/config_validate_tests.rs` and
  `crates/pistol-search/src/pvs.rs`, modified.

**The w2 diff is superseded and nothing is lost by it**: `dev` already carries
`root_reorder: _` at `crates/pistol-engine/tests/config_validate_tests.rs:191`,
and the `pvs.rs` hunk is a rustfmt reformat of a `let`-chain the worktree's
older base spelled differently. Checked before removal, not asserted.

**The text evidence was exported before removal** to
`artifacts/wp22_worktree_residue_export/`, **19 files**, receipt
`RECEIPT.sha256` whose own sha256 is
`58c811ac83a0749946f0534cf8459a50d6f7c1ca405f998da0a0fe15215bc066`. Compiled
binaries, `perf` captures and the `target-*` trees are excluded and the
exclusion is named here: they are re-derivable build products, and `p3`'s five
target directories alone are 1.7 GB.

**Four directories under `/home/tom/pistol-wt/` are NOT worktrees** —
`asm-review`, `p1-patches`, `rev3-logs`, `review-p1-impl-logs`, 182 files
between them, including the P1 review-round material `opt_arc_CLOSURE.md`
called out as surviving only by indirection. `git worktree remove` does not
touch them and they are untouched. They remain uncovered by any receipt, which
is recorded here as a standing exposure rather than repaired by this package.

**After**: `git worktree list` holds `/home/tom/Projects/HeXO-AlphaBeta` alone.

## §0.2 dev is green

Recorded in §0.2 of this document's closure section, from the gate lines of
`artifacts/wp22_ci_baseline.log`, never from a wrapper's exit status.

## §0.3 §B is started

`docs/experiments/wp22_cap_prereg.md` registers the cap calibration; its fresh
review is dispatched against `71fa6f1`.

## §R — the five rulings are ADR lines

R1 -> **D-610**, R2 -> **D-611**, R3 -> **D-612**, R4 -> **D-613**,
R5 -> **D-614**. Each quotes the operator's words verbatim and states what the
ruling forecloses and what it leaves open.

**One citation in R2 does not resolve**: the ruling's text says *"the census
keeps C2 (D-56q)"* and `D-56q` is not a line that exists. D-611 records that
rather than normalising it silently, and names D-570 as the line that selected
and named C2. This follows D-575's precedent for a dispatch citing a D-line the
tree does not hold.
