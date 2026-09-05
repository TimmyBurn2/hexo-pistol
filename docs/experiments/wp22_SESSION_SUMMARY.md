# WP-2.2 — session summary

## ONE LINE FOR THE MORNING

**There is no Stage-2 Elo number yet, and the finding that stands in its place
is this: fitting eval v0's five weights to the corpus's search scores drives the
table FLAT at the top — one-from-a-win worth 65 against two-from-a-win's 64,
where the committed table says 1500 against 300 — because dropping `mate_in`
rows by kind removes 48 % of every position in the corpus that holds a
five-stone window, and removes them on exactly the side where such a window
won.**

## What is finished

**§0 — done, with receipts.** The thirteen worktrees are gone; `git worktree
list` holds the main tree alone. The 219-line export receipt was verified
(`sha256sum -c` clean, per-worktree counts matched) BEFORE anything was removed,
and five worktrees held evidence the receipt did not cover — exported to
`artifacts/wp22_worktree_residue_export/` (19 files, own receipt) first. dev is
green: **20 of 20 gates, `ci: all gates passed`, `EXIT=0`**
(`artifacts/wp22_ci_baseline.log`).

**§R — landed as D-610..D-614.** R2's own citation `D-56q` names no line that
exists; D-611 records that rather than normalising it.

**Two ADR lines carry the session's durable findings**: **D-615** (the
canonical-image census shortcut is measured false) and **D-616** (the label
corpus cannot be fitted by dropping its mate rows). The ROADMAP's Stage 2
carries D-616, per hard rule 10.

**R2's tabulation — closed (D-611).** Recomputed independently from the manifest
body and agrees with its derived header at 2 369. **Zero of them are
transposition disagreements**; all 2 369 are symmetry equivalences, which every
one of the three consumers' rules handles — so D-611's flip clause does not
fire. Collateral: the symmetry fold yields 2.6 % at the ROOT population where
D-560 and D-570 measured zero in tree.

**Phase 1's loader is proven inert — the DONE list's "loader inert by identity".**
A weights file differing from the committed one only in comments and spacing
gives **byte-identical** deterministic engine output (same nodes at every depth,
same PV, same bestmove); perturbing one entry from 60 to 61 changes the output
at every depth, so the file is genuinely read and the test is not vacuous.
`nps` and `time` are excluded and the exclusion is stated — they are wall-clock
fields and a byte-identity claim including them would be false for reasons
unrelated to the loader. Receipt: `artifacts/wp22_phase1_loader/`.

**One thing that receipt records because it bears on the SPRT**: the two files
that play identically advertise **different** `weights_sha256`, because the
digest is over file bytes rather than values, and the arena refuses a mismatch.
That is deliberate — content-identification is what closes WP-1.3's provenance
hole — but it means a cosmetic edit to the tuned table after a run invalidates
that run's identity though it cannot change a move.

**Phase 1's premise and correctness gate — done.** The v0 eval is LINEAR in its
five weights, and an oracle says so against the engine: 0 of 500 positions
disagree, while perturbing one entry by 1 kills 410 of 500. The corpus join is
verified on all 89 805 rows. Two premise findings: the dispatch's
"byte-identity when the key is absent" cannot be built (hard rule 1 makes an
absent key an error, and there are no code-side literals), and the engine-side
loader the dispatch asks for **already ships**, digest-bound in the identity
line.

## What is in flight

**§B's cap calibration is at revision 4 and NOT yet run.** It has been through
**two fresh-context reviews, both FAIL**, and every finding either landed or is
recorded as rejected with its reproducer:

- **Round 1 (13 MAJOR)** — D-563's cap figures are at `--nodes 400000` while the
  registered seat is 50 000; the tree's own 50 000-node censuses disprove
  revision 2's expectation that cap 2048 proves no wins; and a 131072 cap is not
  a cap at this budget, because `total_nodes() = search_nodes + solver_nodes`.
- **Round 2 (7 findings, 3 measured)** — the one that matters is **M2**: round
  1 had argued the join to the corpus files was unnecessary because the census
  counts symmetry-invariant keys. Round 2 MEASURED that substitution and it is
  not neutral — 229 against 236 firings on the same twenty rows. **I reproduced
  it before reverting**: the keys are invariant, the search that finds them is
  not, because its tie-breaks are coordinate-lexicographic. A measurement beats
  an argument and the join is back.

**Revision 4 is remedies-only** and re-measured its dry run on 100 positions
rather than 20, which surfaced the fact that dominates the sizing: **the
win-proving keys are clustered in a handful of root positions** — 86 keys from
**8** roots at cap 2048. The effective sample is the roots. **Revision 4's own
review is owed, and the governed run must not be taken until it passes.**

**§B's census registration is drafted** (`wp22_census_prereg.md`) and waits on
the cap.

**R3's seat-swap anchor is configured and not run** —
`local/sealbot_anchor_v6_seatswap.toml` (under `local/`, which is gitignored,
exactly as v5's own config is), byte-identical to v5 but for the slot
assignment and the output directory. It needs a quiet box.

## What has not been started

**Phase 2 in its entirety** — no matrix, no DECISION-RED-TEAM, no selection.
**Phase 1's SPRT** — registered, not run.

## D-534, restated because this package touched the solver

**D-534 stands, untriggered, and nothing here weakens it.** It records that
arming the solver in a committed PLAY config overshoots the deployment budget by
a measured **725 ms median at a 500 ms budget** (max 1866 ms), which on HeXO —
where the server owns the clock and hard-clamps the call — is a FORFEIT. Its
precondition is that the overshoot be fixed and re-measured at the deployment
budget before any committed play config sets `on_search_path = true`.

**This package arms the solver only through `trigger_census`, an INSTRUMENT seat
at a NODE budget**, never through a committed play config and never under
movetime. No committed config moved. R3's seat-swap anchor runs
`configs/play_staged_v0.toml` **as committed**, with the solver gate off, so it
does not touch the precondition either. D-534's flip clause is untouched: it
flips when the overshoot is fixed and re-measured, which remains its own work
package and is not scheduled here.

## What a successor must not redo

Three things in this session were argued one way and then MEASURED the other,
and each is now pinned so the argument cannot be re-derived into the tree:

1. **The census draw** (D-615). Building fixtures from the manifest's canonical
   sequence removes a join, a defect class and an out-of-tree dependency, and
   the invariance argument for it is correct about the keys. It is still wrong.
2. **The fit's constraint handling.** Projecting the unconstrained solution onto
   the schema is not the constrained optimum; the two answers differ on this
   corpus. Pinned by `tools/texel/test_texel.py`.
3. **Dropping mate rows** (D-616). Correct in isolation, and it censors exactly
   the top of the table.

## The next session's first three acts

1. Read `wp22_cap_prereg_rev3_REVIEW.md`; fix or proceed; then run the
   calibration (~1 h 38 m, detached) and the census after it.
2. Run Phase 1's SPRT on the registered primary. The honest expectation is
   already on record: **these weights lose**, and the finding above says why.
3. R3's anchor, in the calibration's idle time, on a quiet box.

## The trap this session walked into and did not fall for

The fitted weights score BETTER on every offline diagnostic — validation MSE
705 709 against 999 066, Spearman 0.319 against 0.209. D-614 forecloses reading
that as progress, and the mechanism above is why it would have been wrong to.
