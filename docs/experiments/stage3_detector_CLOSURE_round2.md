# Stage-3 detector, round two — CLOSED at the option matrix. The roadmap flips to Stage 2.

> **Audience: the operator, on the first read.** §0 is one paragraph. §1 is
> plain language. Everything after §2 names the artifact or command behind each
> claim. ADR lines: **D-521 … D-534**. The matrix itself is
> `docs/experiments/matrix_stage3_detector.md` revision 4.

## 0. ONE LINE FOR THE MORNING

**The detector package is closed and the roadmap has flipped to Stage 2 — but
NOT because a detector cannot work, and that distinction is the whole result.
The option the last session was about to select turned out to have been chosen
by looking at which positions the solver had already proved on; measured on a
fresh set of positions it is six and a half times too expensive, and under the
one recall definition that matches the gate it keeps none of the wins it was
credited with. Two adversarial reviewers then broke my own measurements twice —
once for an arithmetic slip, once because the instrument was carrying a
transposition table across positions and quietly deflating the numbers — and
both times the fix left the conclusion standing. What is left is a field with no
selectable row, a genuinely new lever nobody had listed (the solver is asked two
questions at every call and one of them is 57–67 % of the cost), and a measured
reason for the deadlock: there are 14 solver wins over 4 positions to choose
between options with, on the one position set that cannot be sampled again.
Three rulings are yours and any of them can reverse the flip. `dev` is green,
19/19, nothing of Stage 2 is opened, and no engine behaviour changed.**

---

## 1. In plain language, before any technical detail

The solver is expensive. The plan was to call it only where it pays, and the way
to decide how was an option matrix: list the mechanisms, measure each against
the budget the bracket allows, pick one.

**The option that was about to be picked does not work.** It fired the solver
only when the opponent held three or more near-complete lines. On the twelve
positions it was measured on, that was cheap enough and kept every proof. On
twelve fresh positions drawn by the same rule from the same corpus, it was
**six and a half times over budget**. It had been tuned, unavoidably, on the
only positions anyone had looked at.

**And the proofs it "kept" were the wrong kind.** The solver answers two
questions at each call — *can the side to move force a win?* and *is the side to
move already lost?* The test that matters is stated in wins. The option was
scored on a total that mixed both, and on the band it was selected from **every
single proof was of the second kind**. Under the right count it keeps zero.

**The safety net that was supposed to catch this cannot.** It is seven positions
from real games where the solver is known to find something. Run properly: two
of them never call the solver at all during a real search, and two more are
answered in a single node. It cannot tell a good detector from a bad one.

**There is one lever nobody had listed, and it is large.** Of the two questions
the solver is asked, the second one is 57–67 % of the total cost and cannot
possibly cost a win, because the win question is answered first. Dropping it,
combined with two other cheap ideas, reaches the budget on the hardest fixture
while keeping every win — the first thing in this arc to do that. It is not
selectable, because it keeps nothing at all on the other fixture, and because
dropping that second question means giving up every "you are lost" answer, which
is a decision about what the engine is for and not a measurement.

**Why nothing could be chosen.** To pick between options you need proofs to
score them on. There are 14, spread over 4 positions, on the one position set
that cannot be drawn again — and 3 over 1 position on the set that can. That is
not enough to fit a rule and check it. **The budget is big enough for the proofs
and the detector's inputs can identify them; what is missing is proofs.**

---

## 2. What the session did, in order

| step | outcome |
|---|---|
| §0.1 `dev` green | `tools/ci.sh` 19/19 at `f62c676` — §6 |
| §0.2 two merged branch refs | verified ancestors of `dev`, deleted (**D-521**) |
| §0.3 row (b)'s cover column | census gains `blocking_covers`; row (b) scored on its mechanism and re-killed (**D-524**) |
| §1.1 one recall definition | wins and losses separated; the headline row keeps 0 (**D-522**, corrected by **D-529**) |
| §1.2 out-of-sample | draw rule registered first, round 0 reproduces the committed fixture; the headline row falls 41.8x → 6.5x (**D-523**) |
| §1.3 value fixture | four rows do not fire at the position; two never fire at all (**D-526**, narrowed by **D-530**) |
| §1.4 band-35 width | n = 1 in both draws, on the row |
| — | `Solver::solve_defender`'s undocumented precondition, found by a panic, documented and pinned (**D-525**) |
| **RED TEAM round 2** | **FALLS** — 3 BLOCKING, 7 MAJOR, 5 MINOR, twelve remedies |
| fix round | warm-table defect fixed and confirmed; row (n) added; every census re-taken (**D-527**, **D-528**) |
| **RE-REVIEW round 3** | **PASS WITH FINDINGS** — all twelve remedies applied, every number reproduced, 260 combinations swept, zero selectable |
| fix round | 26 transcription errors corrected; §5.2/§5.3 now rendered from the artifact by machine (**D-532**) |
| **SELECTION** | **none possible** — the registered kill point fires (**D-533**) |
| DESIGN / IMPL / BENCH / SPRT | **none reached.** A kill at the matrix is before the design by construction |

## 3. The two measurement defects, because they are the transferable lesson

**The reviewers did not break the argument. They broke the instrument, twice.**

1. **An arithmetic slip that survived because it looked plausible.** A required
   cut of 47.25x was published in seven places; the artifact said 43.32x. The
   47.25 was a numerator over the searches that fired against a denominator over
   all searches — the exact defect the instrument had already been corrected
   for, and about which it prints a `NOTE:`. A number can be wrong in a document
   whose own instrument refuses to produce it.
2. **A warm transposition table.** The census reused one `Searcher` across every
   fixture entry, so entry *n*'s node count depended on entry *n−1*'s — the
   hazard `crate::tt` names in its own words (D-7). `tools/bench_block.sh` runs
   one `newgame` per entry in a fresh process, so the census and the bench were
   not the same seat. **And the check meant to catch it passed vacuously**: it
   verified two bands whose entries sit at the node-budget ceiling, which a warm
   table cannot lower, and failed silently on the only band that could show it.
   Fixed, the corrected seat reproduces all three registered `T_off` values and
   the incumbent D-516 recorded, to the digit.

**The fix for the class, not the instance**: §5.2 and §5.3 of the matrix are now
generated from the artifact by machine. Twenty-six of their cells had been
hand-copied and nine of those existed in no artifact anywhere.

## 4. The verdict, and exactly what it does not say

**No row and no composition in the field is selectable** under the kill point
registered before the runs (`docs/experiments/stage3_oos_registration.md` §5) and
attacked without being struck. Two independent adversarial sweeps agree; the
round-3 re-review swept 260 combinations and found zero, with every combination
reaching both out-of-sample budgets keeping 0 of 3 out-of-sample band-15 wins.

**So D-471's clause fires and the roadmap flips to Stage 2 (D-533).**

**IT IS NOT A FINDING THAT A DETECTOR CANNOT REACH THE BRACKET.** Measured:

- an ALLOCATOR reaches the budget **by construction** — it stops when the budget
  is spent — so the bracket was never the binding constraint on that row;
- the budget **affords every proof** at 17 %–25 % of itself, aggregate;
- a score over the census columns could keep **0.857 / 1.000** of the wins,
  against **0.571 / 0.333** for the best ordering anyone wrote here.

**The measured obstacle is the evidence**, and it is stated with its own frame
qualification in the matrix §5.8: in the aggregate frame the barrier is sample
size; in the per-search frame the dispatch's own ruling 1 names, the ceiling is
0.571 and a written ordering already reaches it, so there the barrier is how the
proofs are distributed rather than how few there are. **A design has to choose a
frame before it knows which obstacle it faces**, and that choice was never made.

**THREE RULINGS ARE OWED AND ANY OF THEM CAN REVERSE THE FLIP** (matrix §7):

1. **The census gate's DIRECTION.** D-512 registers a both-directions counter for
   it; this arc ranked on wins alone. Under both directions the killed row keeps
   1.000 of one band's in-sample proofs inside the budget — and row (n), the
   arc's largest lever, goes from free to forfeiting half of what the solver
   finds. **Row (n) and this ruling are the same decision.**
2. **The recall FIXTURE.** Two of its five VALUE rows produce zero firings in an
   entire governed search; two more are answered in one node. D-512's flip clause
   does not cover a row that never fires.
3. **The trigger-rich SAMPLE.** The only band with a win denominator worth the
   name is twenty positions from the two sealbot-anchor games, and there is no
   third game. Widening it is a work package nobody has commissioned.

**No row's OPERATOR-registered kill condition has fired.** Rows (a) and (e) are
killed by *"excluding a VALUE row of the recall fixture"*, and the fixture is
measured unable to discriminate. What fired is the stricter instrument this
session registered — which is why every one of the three rulings above can
supersede it.

## 5. THE PLAY-MODE PRECONDITION — standing, and independent of all of the above

**Before any committed PLAY config sets `[solver] on_search_path = true`, the
movetime overshoot must be fixed and re-measured at the deployment budget
(D-534).** D-520 measured it on the governed anchor: 240 answers at a **500 ms**
budget, **median 1225 ms, maximum 1866 ms** — a **725 ms median overshoot** —
because a solver call absorbs its whole node count at once and the root's two
calls are made before anything is abortable. Gates off, the same instrument
measured an 8 ms maximum overshoot.

**On HeXO the server owns the clock and hard-clamps the call (D-478, D-503), so
an answer at 1866 ms against a 500 ms budget is a forfeit there.** It is a
D-95-class abort-responsiveness defect, not a strength question, and **an SPRT
`h1` on any detector would not discharge it** — a detector that made the solver
affordable in NODES leaves a capped call exactly as unabortable in
MILLISECONDS. This line stands whether the detector ships, whether it is
abandoned, and whether the roadmap flips.

## 6. Gates

`tools/ci.sh` in a detached worktree with its own `target/`, never with
`CARGO_TARGET_DIR` exported.

- **`f62c676`** (§0.1's green confirmation), log
  `artifacts/stage3b_ci_base_f62c676_v1.txt`: nineteen `=== gate N/19:` lines,
  final line **`ci: all gates passed`**,
  `/usr/bin/grep -cE "^ci: FAIL|^ci: RUN VOID|test result: FAILED"` returns
  **0**, gate 9 closes `determinism: ok — 5 seat(s), no difference outside
  nps/time in any of them`.
- **`6e7e0734`** (the revision-4 landing), log
  `artifacts/stage3c_ci_6e7e0734_v1.txt`: the same nineteen lines, the same
  final line, the same count of **0**, the same determinism line.
- **The closure HEAD** — §8 names it and its log, and it is the run that
  adjudicates this document, because a gate claim names the tree it read.

## 7. Where everything is

| document | what it is |
|---|---|
| `docs/experiments/matrix_stage3_detector.md` | **the matrix, revision 4** — the field, the measured ranking, the rulings owed |
| `docs/experiments/stage3_oos_registration.md` | the pre-registration: recall definition, draw rule, budget derivation, selection rule, all before the runs |
| `artifacts/matrix_stage3_DECISION_REDTEAM_round2.md` | red-team round 2 at `b6962ac`: **FALLS**, 3 / 7 / 5, twelve remedies |
| `artifacts/matrix_stage3_REREVIEW_round3.md` | the scoped re-review at `6e7e0734`: **PASS WITH FINDINGS**, 260 combinations swept |
| `artifacts/stage3c_census_*` | the censuses on the corrected seat — ON, OFF, and the value fixture |
| `artifacts/stage3c_census_rank_v2.txt` | **the ranking every §5.2/§5.3 cell is rendered from** |
| `artifacts/stage3c_allocator_bound_v1.txt` | row (e) and the bound over the census columns |
| `artifacts/stage3b_*` | **SUPERSEDED** (contaminated seat), kept because the round-2 report quotes them |
| `crates/pistol-search/examples/trigger_census.rs` | the census, now clearing between entries |
| `crates/pistol-cli/examples/stage3_oos_positions.rs` | the out-of-sample draw |
| `crates/pistol-search/examples/value_fixture_recall.rs` | the fixture, position by position |
| `tools/stage3_census_rank.py`, `tools/stage3_allocator_bound.py` | the two ranking instruments |
| `sessions/Stage-3-detector/` | a POINTER to this file only — `/sessions/` is gitignored |

## 8. Export receipt anchor

`artifacts/` is gitignored, so a digest list living only there anchors nothing.
The receipt and its own sha256 are recorded in
`docs/experiments/overnight_export_receipt.md`, which this document does not
restate (D-423).
