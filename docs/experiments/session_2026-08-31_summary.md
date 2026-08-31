# Session summary — 2026-08-31. The detector arc closed; Stage 2's opener stopped twice.

> **Audience: the operator.** §0 is one paragraph. §1–§4 are the four packages
> with the reasoning behind each decision. §5 is the through-line, which is a
> finding about method. §6 is what is owed. Every claim names its ADR line or
> its artifact; nothing here is restated from a closure that owns it (D-423).
>
> `sessions/` is gitignored, so this lives at a tracked path with a pointer
> there — the defect `6812ddc` fixed once and this arc pre-empts.

## 0. ONE PARAGRAPH

**The Stage-3 detector arc closed with a measured verdict and the roadmap
flipped to Stage 2; Stage 2's opening package then stopped twice, and the second
stop is about how I write design documents rather than about the work.** Sixteen
commits, twenty-five ADR lines (D-521…D-545), seven adversarial reviews
dispatched, four CI runs green at nineteen gates. No engine behaviour changed, no
committed config moved, no strength claim was made. `dev` is clean.

---

## 1. THE STAGE-3 DETECTOR ARC — closed, and the flip is recorded

### What was asked and what happened

The arc was to finish an option matrix for a solver-call detector, select a row,
design and implement it, and take it to SPRT. **It reached the kill point
instead**, and the closure is `docs/experiments/stage3_detector_CLOSURE_round2.md`.

### The four obligations that changed the answer, and why each mattered

**One recall definition (D-522, corrected by D-529).** The matrix's headline row
`opp_hot >= 3` was credited with keeping "1.000 / 0.500 / 0.560" of the proofs.
The solver answers two questions per call — *does the mover force a win* and *is
the mover already lost* — and the ranking tool had been summing them. **Split, the
band the row was selected on has ZERO win-direction proofs**; its 1.000 was
1.000 losses. **The mechanism is not a coincidence**: `opp_hot` counts the
*opponent's* hot windows, so it rises exactly where the *defender* direction
proves. A threshold fitted to it is fitted to losses by construction, which is
why no other threshold rescues the row.

*Reasoning for the correction (D-529):* my first ruling said the gate pins wins,
full stop. A red team showed D-512 registers **two** gates and names D-510's
`proofs` counter — which reads both directions — for the census one. I had let
one gate's direction supply the other's. The narrowing was withdrawn as a claim
about D-512 and handed to the operator as a ruling.

**Out-of-sample validation (D-523).** The threshold had been chosen after seeing
which firings proved, on the only 24 positions anyone had drawn. I registered
the draw rule *before* drawing — continue `corpus-extract`'s own loop with its
own carried used-set — so round 0 reproduces the committed fixture to the same
sha256 and round 1 shares no game with it. **The row fell from 41.8x to 6.5x
against a required 24.3x**, with the fall in the numerator rather than the
budget.

*Reasoning for the shape:* an out-of-sample claim is only worth the disjointness
of its draw, so the instrument had to reproduce the in-sample draw exactly before
its new draw could be read. That check (Criterion V) is what licenses the table.

**The value fixture, run directly (D-526, narrowed by D-530).** D-512's recall
gate asks that a proof-bearing trigger point rank inside the budget. Run: **two
of five VALUE rows produce zero firings in an entire governed search**, and the
two that fire prove in **one visit** — so the gate is satisfied by any predicate
admitting `mover_hot > 0` and discriminates almost nothing.

*Reasoning for the narrowing:* my first statement said four of seven are not
trigger points. True *at the position*, false as a claim about the gate — two of
those four fire in-tree and find proofs there. The measurement replaced the
inference.

**Band-35 sample width.** n = 1 in both draws, on the face of the row.

### The two measurement defects, both caught by reviewers

**A warm transposition table (D-527).** The census instrument reused one
`Searcher` across every fixture entry, so entry *n*'s node count depended on
entry *n−1*'s — the hazard `crate::tt` names in its own words (D-7). The bench it
was being compared against runs one `newgame` per entry in a fresh process.
**The check meant to catch this passed vacuously**: it verified two bands whose
entries sit at the node-budget ceiling, which a warm table cannot lower, and
failed silently on the only band that could show it. Corrected, the seat
reproduces all three registered `T_off` values and D-516's own incumbent figure.

**Twenty-six hand-copied cells, nine in no artifact (D-532).** Fixed at the
cause rather than the instance: §5's tables are now **rendered from the artifact
by machine**. A number a human retypes drifts from its run.

### The verdict, and what it deliberately does not say (D-533)

**No row is selectable** — two independent adversarial sweeps agree, the second
sweeping 260 combinations and finding zero. D-471's clause fires and the roadmap
flips.

**It is not a finding that a detector cannot reach the bracket.** An allocator
reaches it *by construction*; the budget affords every proof at 17–25% of itself;
a score over the census columns could keep 0.857–1.000 of the wins where the best
ordering anyone wrote keeps 0.571. **The measured obstacle is the evidence** —
14 win-proving firings over 4 positions on the one fixture that cannot be sampled
again.

*Reasoning for recording it that way:* a flip that reads as "the detector is
impossible" would send a successor down a different road from a flip that reads
as "we are under-evidenced". The two have different remedies, and only one of
them is true.

### The rulings you then took (D-535…D-538)

Both directions for the census gate; the fixture defined over rows that fire,
with the two never-firing rows becoming trigger-coverage findings; the sample
debt discharged inside Stage 2's label pipeline. **The flip stands on grounds
none of my documents had**: a v0-fitted score would not survive the eval change,
which makes doing the detector first wasteful rather than merely under-evidenced.

---

## 2. WP-2.0 — premise STOP, then a scope ruling

**The premise verification returned P2 FAILS**, and I verified both halves myself
rather than acting on a subagent's word for a STOP: `pistol-cli` does not depend
on `pistol-search` (a boundary its own manifest defends and a test pins), and
`TriggerObservation` carries no position identity at all — so D-537's *"disjoint
positions"* is uncountable from census output.

**The dispatch's scope line forbade and permitted the same change**: *"any engine
diff at all"* against *"detector work beyond the logging flag"*. I stopped rather
than pick, because **the reading that licensed more work was the one I would have
picked**, and this dispatch's STOP protocol had no default-taking clause.

You ruled **reading B** (D-539/D-540): the flag becomes WP-2.0b, labels are cold
by construction with a fresh-process criterion owed in the pilot, seeds attach to
sampling only, `book_v2` stays out of committed configs.

---

## 3. WP-2.0's SHAPE — decided, red-teamed twice, selected

**Why a matrix at all:** the premise memo left twelve forced decisions and one
was architectural — whether the pipeline extends the crate CLAUDE.md calls *"the
judge every later work package is tried by"*. A design that just picks that is
the silent-architecture-drift breach.

**Revision 1 FELL** (3 BLOCKING). Two findings were mine and structural:

- **My coldness criterion never named the mechanism that provides coldness** —
  zero mentions of `newgame` — so it discriminated between no two rows. `newgame`
  is a true wipe pinned node-for-node against a fresh engine, which makes
  coldness cost a memset rather than a process spawn.
- **I mis-measured the reusable surface** (five items; it is 22 `pub mod`), and
  mis-priced row (b) on three duplications that are in fact public.

**Revision 2 STOOD WITH CORRECTIONS** and row (g) survived a red team **pointed
at it by name with your leaning quoted to it**, so it attacked the leaning rather
than inferring it. All four load-bearing claims settled by reading: zero seams,
coldness end to end, the report round-trips every position (rule 4's one-stone
turn surviving as `Turn::Single`), only `pistol-arena` touched.

**The selection (D-542): row (g) on branch B.** The strongest surviving attack is
recorded — a matrix may leave a design choice open, but not one whose branches it
has already priced as fatal to two other rows — which is why the branch was
picked rather than deferred.

**I ruled your field question and made it reversible (D-541)**, on the ground
that *"per design"* delegates and an exhaustive list would not also delegate. The
cost if wrong is stated: the selection collapses to (b) by elimination.

**And my own MEASURED block was wrong three revisions running (D-543)** — a
`tail -20` ate the head of a listing, and revision 2 re-asserted the figure
labelled `REPRODUCED` without re-running it. That is worse than the slip: a wrong
number wearing a check nobody made.

---

## 4. THE TWO DESIGN STOPS

**WP-2.0's design failed twice and split (D-544).** The mechanism cleared both
reviews; the record schema failed both. The line was drawn between **capture and
meaning**, because every failure was an interpretation question.

*The fix round introduced two defects of its own*: the provenance remedy
reproduced the defect it was fixing (`solver_nodes > 0` means the solver was
*consulted*, not that the answer is a proof — `Provenance` has four variants),
and a reversal reached two sites of four, leaving a test pinning the negation of
the invariant above it.

**WP-2.0-M's design then failed twice too (D-545).** Revision 1's central claim
was false: a "verbatim" capture cannot be reproducible, because the totals line
carries wall-clock fields that CI gate 9 exists to strip — **and the test I
registered would have passed vacuously**, since the only engine the arena suite
drives hardcodes `nps: 1, time_ms: 0`.

**Revision 2 is the clearest failure of the session.** Revision 1 contained the
true sentence — *"`validate_engines` refuses identical **labels**, not identical
binaries or configs"* — and **my rewrite deleted that clause and built a refusal
on its negation**, which would have rejected 100% of its inputs. The rewrite also
lost seven things including a whole section the previous review had *passed*.

**I did not take the scope-finding escape the reviewer offered.** The two hardest
mechanisms in revision 2 were verified *correct*. The content was reachable; the
authoring lost it.

---

## 5. THE THROUGH-LINE — five reviews, one method failure

Seven adversarial reviews were dispatched this session. **Five were design or
matrix reviews and all five failed**, and not one on judgement. Every failure was
a claim about the code the code does not make, or a true claim a revision
destroyed.

**The class was named once, mid-session, and the remedy not generalised.** D-543
fixed drifting tables by rendering them from the artifact by machine. It worked;
the tables have been right since. **It was never applied to prose, and prose is
where the next four failures happened.**

`tools/design_citation_check.py` is the generalisation: every path and
`path:line` a document attributes to the tree is checked against the tree, and a
design must *declare* the files it proposes to create. **Its own output says it
would not have caught the engine-label defect**, which was bare prose with no
citation — which is the honest measure of it, and the habit it teaches is the
one the expensive half needs.

*Why this matters more than any single package:* the reviews were doing the work
the author should have done. That is affordable twice and not five times.

---

## 6. WHAT IS OWED

**To you, one decision** (`docs/experiments/wp20m_DESIGN_STOP.md` §3): hand
WP-2.0-M's design to a fresh session with this arc's five reviews as input, or
have me re-attempt under the citation discipline, lifting the two verified
mechanisms **by quotation rather than by rewriting**. **I recommend the fresh
session**, against my interest in finishing what I started.

**Standing, and independent of everything above (D-534):** before any committed
play config arms the solver, the 725 ms median movetime overshoot must be fixed
and re-measured at the deployment budget. No SPRT discharges it — it is
abort-responsiveness, not strength.

**Sequencing (D-539):** production label runs wait on WP-2.0b, which is not
started.

**Unblocked and not started:** WP-2.0-S (the record schema), whose owed list is
in `docs/experiments/wp20_DESIGN_STOP_SPLIT.md`.
