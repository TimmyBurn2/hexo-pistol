# WP-2.2 Phase 1 continuation — §0 receipt

## 1. `dev` is green at `ae211f0`, cited from the gate lines themselves

`tools/ci.sh` run over a tree frozen for the run's whole length — this is D-617's
requirement and not a formality, because the citation gates read the WORKING TREE
while gates 2, 17 and 18 read tracked bytes. `git status --porcelain` was empty
at the start and at the end, and `git rev-parse HEAD` returned
`ae211f0d8eb773721bf45e9f0e48caad88041a37` at both.

Log: `artifacts/wp22p1_ci_ae211f0.txt`, 3 102 lines. The gates, as the script
printed them:

```
=== gate 1/21: cargo fmt --all --check
=== gate 2/21: build from the git-tracked file set
=== gate 3/21: cargo test --workspace --locked
=== gate 4/21: cargo clippy --workspace --all-targets -- -D clippy::all
=== gate 5/21: artifact rejection
=== gate 6/21: config validation
=== gate 7/21: perft oracle
=== gate 8/21: tactical fixture at its pre-registered threshold
=== gate 9/21: cross-process determinism
=== gate 10/21: differential search oracle
=== gate 11/21: staged generator soundness (four parts)
=== gate 12/21: solver oracle (four gates)
=== gate 13/21: solver determinism
=== gate 14/21: movetime ceiling on the D-95 reproducer class
=== gate 15/21: arena self-match smoke
=== gate 16/21: sealbot anchor platform suite
=== gate 17/21: file-justification check
=== gate 18/21: offline texel and census tooling
=== gate 19/21: decision-key uniqueness
=== gate 20/21: carve-document label consistency
=== gate 21/21: governing-document citations
ci: all gates passed
EXIT=0
```

**Twenty-one of twenty-one, `ci: all gates passed`, `EXIT=0`.** Not a wrapper's
exit status: the two lines above are the script's own.

## 2. The worktree — the dispatch's premise is stale, and that is the finding

The dispatch says *"Remove the one remaining worktree after export receipt."*
**There is no remaining worktree**, and the receipt it names verifies. Derived
rather than assumed, four ways:

| check | command | answer |
|---|---|---|
| registered worktrees | `git worktree list --porcelain` | the main tree only |
| stale registrations | `git worktree prune --dry-run -v` | nothing to prune |
| the admin directory | `ls .git/worktrees` | does not exist |
| stray checkouts | `/usr/bin/find /home/tom -maxdepth 3 -name .git -type f` | none |

The last row uses `/usr/bin/find` because the agent shell's `find` is
intercepted by a shim, which produced one empty digest receipt in the preceding
session that looked correct in a directory listing.

**The export receipt covering what the removed trees held verifies**:
`artifacts/wp22_worktree_residue_export/RECEIPT.sha256`, 19 lines,
`sha256sum -c` clean over all of them.

**And the thing D-469 exists for held**: the design review the removed worktree
produced is committed at `ae211f0` as
`docs/experiments/wp22_phase1_design_REVIEW.md`, 475 lines. It did not survive
only in a transcript.

## 3. What this receipt does NOT say

It says nothing about any revision after `ae211f0`. Every gate above ran against
that tree, and a later commit is green only when a later run says so.
