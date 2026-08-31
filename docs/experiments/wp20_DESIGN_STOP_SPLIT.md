# WP-2.0 — DESIGN STOP AND SPLIT. The mechanism cleared twice; the record schema failed twice.

> **Audience: the operator, first read.** §0 is what happened and what it costs.
> §1 is the split line and why it falls there. Nothing here restates the design
> or its reviews (D-423); they are at tracked paths named in §5.

## 0. WHAT HAPPENED

`docs/experiments/wp20_design.md` revision 1 took a fresh-context REVIEW-design
and **FAILED** (2 BLOCKING, 8 MAJOR, 11 MINOR). Revision 2 was the one fix round
the standing caps allow. A scoped re-review of revision 2 returned **FAIL**
again: of the 21 findings, 4 APPLIED, 6 PARTIALLY APPLIED, 9 NOT APPLIED, and
**2 APPLIED BUT INTRODUCED A NEW DEFECT**.

**Two failures is a STOP and a split (D-481's form), and the overnight loop
grant has expired.** No implementation was started. `dev` carries the design and
both reviews as the record of a failed round, which is what they are.

**The failure is not the shape.** Two independent reviewers traced the mechanism
and could not break it, one of them re-deriving the whole coldness chain rather
than trusting three agreeing documents. **The failure is entirely in what the
RECORD CONTAINS**, and it failed there twice.

## 1. THE SPLIT LINE, drawn by where the findings clustered

**WP-2.0-M — THE MECHANISM. Cleared, and it lands.**

Verified by review, not asserted:

- **The two-pass shape.** Pass 1 is `arena --config` on the unmodified SPRT path;
  self-play is expressible today. Pass 2 reads the report and walks it.
- **Coldness end to end.** `newgame` → `Pistol`'s three fields → `Searcher`'s six,
  `clear` reaching three, `Table::clear` a true `fill(EMPTY)` and not the epoch
  bump beside it, `Solver::reset` rebuilding, `Position::reset_to` unwinding the
  eval, and the `PvTable` not being a `Searcher` field at all. **Nothing that
  could carry across a position survives.**
- **Branch B's output-neutrality**, because `totals_of`'s return value has exactly
  one consumer.
- **The forfeit reversal is SAFE** — and this was the re-review's own strongest
  attack that did not land. `transcript::read` legality-checks every game through
  pistol-core at read time, refusing the whole report on an illegal turn or on
  moves after a win, **so a forfeited game's move list is a guaranteed legal
  prefix before pass 2 exists.** It cannot panic and it cannot refuse.

**WP-2.0-S — THE RECORD SCHEMA. Cut out, and it gets its own design round.**

Every finding of both reviews lives here: what the label's provenance is, how the
score is represented, how many node columns there are and what each means,
whether book turns and forfeited games are labelled, how a board key is derived,
whether an undecided game is distinguishable from a capped one, and the
census-minimum rule.

**WHY THE LINE FALLS EXACTLY THERE, and it is not a preference.** Every failure
was an INTERPRETATION question and none was a CAPTURE question. The mechanism
gets the engine's answer out of the process coldly, reproducibly and identifiably;
the schema decides what the answer MEANS. **A package that captures verbatim
cannot be wrong about meaning**, which is why the mechanism can land before the
meaning is settled: WP-2.0-M writes the totals line **as the engine wrote it**,
beside the position and the run's identity, and WP-2.0-S decides the columns.

## 2. THE TWO NEW DEFECTS THE FIX ROUND ITSELF INTRODUCED

Recorded plainly, because a fix round that creates defects is the thing a second
review exists to catch and the thing a session is least able to see in itself.

**NEW BLOCKING 1 — the provenance remedy reproduced the defect it was fixing.**
Revision 2 marked a record as solver-provenance when the totals line carried the
solver fields, on the ground that `report.rs` emits them only inside
`if info.solver_nodes > 0`. That condition is true, and it means **"the solver was
CONSULTED"**, not "the answer is a proof": `Provenance` has **four** variants —
`CompletedDepth`, `PartialRoot`, `SolverProof`, `Fallback` — and the search-path
solver accrues `solver_nodes` on answers that return `CompletedDepth`. So the
remedy marks ordinary search answers as solver answers, which is the prior
BLOCKING's own defect wearing its fix.

**And a second limb the remedy missed**: `nodes` is `search_nodes + solver_nodes`
(`crates/pistol-search/src/pvs.rs:151`), and §3 sources `label nodes` from it —
**so the record does sum exactly what revision 2's own INVARIANT 9 says it never
sums.** The finer discriminator is already on the wire (`search_nodes == 0`), so
this is one predicate and not a re-scope.

**NEW BLOCKING 2 — the forfeit reversal was applied to two sites of four.** The
decision was right and safe; the edit was incomplete. §2 and INVARIANT 3 were
rewritten to label every position of every game; **§7 still says "non-forfeited"
and §11 still registers `a_forfeited_game_contributes_no_records_and_is_counted`
— a test that pins the NEGATION of the invariant above it.** The rewritten
invariant also silently dropped the book-turn exclusion. **The document now
answers four questions twice and differently**, which is why the re-review's
implementability verdict is *no*.

## 3. WHAT IS OWED, and to which package

**To WP-2.0-M (the mechanism):** nothing outstanding from the reviews. It needs a
design of its own — short, because its content is already verified — and one
review round.

**To WP-2.0-S (the schema):** every finding of both reviews, plus these the
re-review added and which no revision has answered:

1. **`undecided` discards information the transcript has.** `RecordedGame` carries
   a `forfeit` field, so capped and forfeited ARE separable; revision 2's
   `undecided` collapses them because `Outcome` has only two variants.
2. **The board key's "sorted, canonical order" does not say whether symmetries
   fold** — and §8's FIT/HELD-OUT split now depends on that key.
3. **`experiment_sha256` excludes the timing block but does not close over the
   games**, which the prescribed verdict block did.
4. **The `cp` / `mate` / `-mate` fixture triple is incomplete** — plain `mate` has
   no fixture.
5. **`pv` is widened with no consumer**, while `solver_nodes` — which the
   provenance fix needs — is not.

**To the operator, and it is the one thing neither package can take:** §8's
census-minimum rule binds its deadline to *"before the corpus is first counted
against any candidate minimum"*, and **the census count it names is WP-2.0b's,
which D-539 moved out of this package entirely.** So the rule's enforcement
mechanism points at a package that does not exist yet. That is a sequencing
question, not a design one.

## 4. STATE AT THE STOP

- **`dev` is clean.** The design and both reviews are committed as the record of a
  failed round; no WIP branch is needed because no implementation was started and
  no engine file was touched.
- No worktree, no detached process.
- CI at the closure HEAD is cited in §6.

## 5. WHERE EVERYTHING IS

| document | what it is |
|---|---|
| `docs/experiments/wp20_design.md` | the design, revision 2 — **superseded by this split, not by a revision 3** |
| `docs/experiments/wp20_design_REVIEW.md` | REVIEW-design on revision 1: FAIL, 2 / 8 / 11 |
| `docs/experiments/wp20_design_REVIEW_rev2.md` | the scoped re-review on revision 2: FAIL, and the source of §2 |
| `docs/experiments/matrix_wp20_shape_selection.md` | the selection this design implemented — **unaffected**, row (g) on branch B stands |
| `docs/experiments/matrix_wp20_pipeline_shape.md` | the shape matrix, revision 3 |
| `docs/experiments/wp20_premise_memo.md` | the premise verification, whose decision 9 revision 1 dropped |
| `docs/experiments/wp20_dispatches.md` | both governing dispatches, transcribed |
| `docs/trigger_coverage_ledger.md` | TC-1 and TC-2, from §0 of the original dispatch |

## 6. Gates

`tools/ci.sh` at `7689714` — the revision the second review adjudicated — run in
a detached worktree with its own `target/`, never with `CARGO_TARGET_DIR`
exported. Log `artifacts/wp20_ci_stop_7689714_v1.txt`, read from the gate log's
own lines: all nineteen `=== gate N/19:` lines, final line **`ci: all gates
passed`**, `/usr/bin/grep -cE "^ci: FAIL|^ci: RUN VOID|test result: FAILED"`
returns **0**, and gate 9 closes `determinism: ok — 5 seat(s), no difference
outside nps/time in any of them`.

**The commit that lands this document is documentation only** — this file, the
second review, one artifact and one ADR line, none on a gate path. It is not
re-adjudicated, which is the cut this project makes and states rather than
implies.

## 7. WHAT HAPPENS NEXT

**WP-2.0-M begins immediately** on the operator's instruction, and **WP-2.0-S
follows it.** Neither opens anything of Stage 2 beyond what D-539 already
licensed, and neither makes a strength claim.
