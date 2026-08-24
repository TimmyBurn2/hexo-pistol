# WP-1.6 — Threat-only zone-bounded quiescence: design

**Revision 5 — a small, self-correcting DELTA on revision 4, not a
rewrite.** Revision 4 (`82de3e4`) added §3.5a (THE COMPLETION STONE) from a
third architect ruling, closing revision 3's B1 and B3/B4; a SCOPED
fourth-slot review (`wp16_design_REVIEW_rev4.md`) PASSED the completion
mechanism itself (its scope items 1, 2, 3, 5, 6) but FAILED on scope item 4:
revision 4's own B2 fix was sound, but revision 4 separately introduced a
NEW defect in the same subsection — a ply-2 win-check the reviewer proved
both unreachable and false-in-its-stated-justification. **Revision 5 deletes
that clause, replaces it with the two-line unreachability proof the
reviewer supplied, and folds in three adjacent one-clause precision fixes
the same review named (N3, N4, N5/N6) — nothing else changes.** §10 (bottom)
carries the full revision history. Revision 3 (`a3b9e37`) was authored from
two architect rulings resolving `D-389`'s open questions (extension width;
TT store rule) after revisions 1 (`9fa27c8`) and 2 (`b1ba746`) each failed
fresh-context REVIEW-design (`docs/experiments/wp16_design_REVIEW.md`,
`wp16_design_REVIEW_rev2.md`); `D-389` and `D-391` record those and what
would unstick them. Sections not named in §10's revision-5 changelog are
UNCHANGED from revision 4 (or, further back, revision 3) and already passed
their own review.

Every claim below cites a calculus ID (`docs/research/threat_calculus_v1.md`),
an ADR (`docs/decisions.md`), or a `file:line`. Reader: this is the
expansion, not a tutorial — read the calculus and `staged.rs`/`pvs.rs`/
`tt/{mod,entry}.rs` first.

## 1. Problem (core §1, unchanged)

`Eval::value` sums window-pattern features (`pistol-eval`); it cannot see a
forced win or a forced loss that starts one turn past the horizon. The prior
"experiment-2" failure broadened the CANDIDATE SET at the horizon (report
§B: "your naive one-primitive threat-extension probe failed because it
broadened the candidate set globally"). **Both revision 1 and revision 2
reproduced this anti-pattern in a different dress** — revision 1's trigger
(c) fired on plan-free positions at a measured 61.5–92.5% of nodes
(`wp16_design_REVIEW.md` C1), and revision 2's fix, while it narrowed the
GATE condition, left `§4`'s generation still calling full `staged_candidates`
— a self-contradiction the scoped re-review caught (`wp16_design_REVIEW_rev2.md`
NEW-1/NEW-2). **Ruling 1 (§3 below) is the architect's response: the
extension's candidate cells are the TRIGGER's own query output, full stop,
and `staged_candidates` is never called from inside quiescence.** This is
stated exactly once, in §3, and nowhere else in this document restates or
contradicts it.

## 2. TURNS invariant (core §2, D-111, unchanged)

A quiescence node is a turn boundary; stand-pat is a static read at
`Phase::First` only, never mid-turn. An extension is granted or refused only
at a `depth_plies == 0`, `Phase::First` gate, and once granted it completes
as a whole turn (both plies) before the next gate decision.

### 2.1 The gap this closes — VERIFIED at `9fa27c8`, unchanged by revisions 2/3

`pvs.rs:199–217` (the `if depth_plies == 0` branch): a `debug_assert!` on
phase, then `return self.position.value()` — no threat query, no candidate
generation. Stage F's win-now check (`staged_candidates`'s step 1,
`staged.rs:186–189`) runs only inside `depth_plies > 0` (`pvs.rs:260–291`).
A mover who can complete six THIS TURN at a `depth_plies == 0` node is,
today, scored by the static evaluator instead of `mate_in(...)`. This is the
gap trigger (a) closes.

## 3. RULING 1 — the extension's move set, stated once

**The rule, quoted from the resume dispatch, in the architect's own words:**
extension searches trigger-derived cell sets only — defensive ply-1 = union
of hitting sets over opponent plans t ≤ 1 (completeness argument: non-hitting
moves lose to plan execution by definition); defensive ply-2 = hitting cells
of remaining live opponent plans t ≤ 2, union mover plan-making cells t ≤ 2;
offensive pairs from the t ≤ 2 plan's own making cells; empty hitting set =
losing band, no search. **Full `staged_candidates` width is EXCLUDED from
qsearch** — the reason is §1's anti-pattern, cited here and nowhere else: a
generator built to be complete over a `depth_plies > 0` subtree (Tier T's own
completeness licence is `LAW-SUPPORT`'s k-TURNS-OUT argument, §3.3 below) is
not narrow enough for a horizon extension that has no turns of remaining
depth to spend the completeness on.

**Consequence for the recursion shape.** Because qsearch no longer calls
`staged_candidates` and no longer needs `pvs::visit`'s `depth_plies`-driven
candidate dispatch at all, it is realized as a DEDICATED function —
`Run::quiescence(&mut self, alpha, beta, ply, q_budget) -> i32`, in a new
module `crates/pistol-search/src/quiescence.rs` (parallel to `staged.rs`) —
invoked exactly once, from `pvs::visit`'s existing `depth_plies == 0` branch,
replacing today's unconditional `return self.position.value()` with a call
to this function (which itself falls back to that same static read when no
trigger fires or the budget is spent). This is a DIFFERENT shape from
revisions 1–2, which tried to reuse `visit`'s own `depth_plies > 0` machinery
by rebinding locals in place — that reuse was only motivated by wanting
`staged_candidates`'s width, which Ruling 1 now forbids, so the reuse
argument is gone and a small dedicated function is the RULE9-JUSTIFICATION-
consistent shape (`pvs.rs`'s own doc: "the honest reductions are elsewhere
and are scheduled... Stage 1 moves candidate generation out entirely",
`pvs.rs:32–34`).

### 3.1 Gate step 1 — mover can win this turn (trigger a, unchanged in substance)

`threats.can_win_this_turn(us, left)` where `left = StonesLeft::from_state(state)`
(`query.rs:51–58,231`, `PROTO-NODE` step 1). `Some(witness)` → terminal:
`mate_in(turns_from_root + 1)`, zero extra nodes, no call into
`quiescence()` at all. Scoped to `CandidatePolicy::Staged` only — under
`Radius` there is no `ThreatState` (`position.rs:48,67,187–194`) and `visit`
never calls `quiescence()` there; `pvs::visit`'s existing `self.policy` match
is the gate.

(M10 from the rev-3 review — this trigger's PV/`seldepth_turns` bookkeeping —
is out of this delta's named scope, per "touch nothing the rev-3 review
verified clean" read together with "DELTA ONLY"; left for whoever next
touches §3.1, not silently dropped.)

### 3.2 Gate step 2 — opponent holds a plan with t ≤ 1 (trigger b, RE-SCOPED per Ruling 2)

**Ruling 2, quoted:** "(b) fires iff ThreatState reports ≥ 1 live opponent
plan with t ≤ 1." This is narrower than revision 2's `Cover::Minimal(_)` (t
∈ {1,2}) — it returns to t ≤ 1 only, the reading revision 1 originally had
before the first review's C4 broadened it. **This is an architect ruling
supplied with this dispatch, not re-derived here**; the design's job is to
express it precisely and cite it, not re-litigate C4's t=2 argument. (Ruling
1's ply-2 rule, §3.5 below, is where a t=2 case a gate at t≤1 did not fully
resolve gets picked up — see the walkthrough there.)

Query: `Cover = threats.blocking_covers(us, HitBudget::from(left))`
(`cover.rs:201`). Classification:

- `NothingToBlock` (t = 0): not this trigger; check §3.3.
- `Minimal(covers)` where `covers.iter().any(|c| matches!(c,
  MinimalCover::One(_)))` (t = 1, `MinimalCover::One`'s own doc,
  `cover.rs:60–61`; the exact test REVIEW-design's V2 confirmed by
  counterexample against `.all(..)` under-firing, `wp16_design_REVIEW.md`
  V2): **trigger fires.**
- `Minimal(covers)` with no `MinimalCover::One` (t = 2): **not this trigger**,
  per Ruling 2's literal scoping. Check §3.3 (offense may still fire).
- `Impossible` (t ≥ 3): `LAW-OVERLOAD`; handled by §3.4, not this trigger.

**Completeness argument for restricting ply-1 to hitting cells, quoted from
Ruling 1 and grounded:** "non-hitting moves lose to plan execution by
definition." `LAW-FORCE` (`threat_calculus_v1.md:49–53`): "if the opponent
has ≥1 plan and the mover cannot win this turn, every non-losing mover move
hits all opponent plans." A candidate that does not appear in some minimal
cover fails this by construction, so excluding it from the ply-1 set loses
nothing a sound line could use.

**Ply-1 candidate cells: `Cover::cells()`** (`cover.rs:108–116`) — the union
over every inclusion-minimal cover the `blocking_covers` call above already
computed. This can include cells from a `MinimalCover::Two` alongside the
`MinimalCover::One` that satisfied the t≤1 test (V2's counterexample:
families `{a,b}`/`{a,c}` admit `One(a)` and `Two{b,c}` simultaneously) —
those are still legitimate single-stone contributions to SOME minimal cover,
so they are not excluded; §3.5 (ply-2) is what makes offering `b` or `c` at
ply-1 sound even though neither alone fully hits.

### 3.3 Gate step 3 — mover can activate a new plan this turn (trigger c, unchanged from revision 2's C1 fix)

**Ruling 2, quoted:** "(c) iff a mover plan with t ≤ 2 is creatable this
turn." Query: `threats.cells_raising_to_hot(us, NearHot::Three, &mut cells)`
(`query.rs:187–192`, equivalently `live_cells_at_count(us, LiveCount::Three,
...)`, `query.rs:206–208` — D-267's map entry, the two name the same
windows). Non-empty → trigger fires; candidate cells = the query's own
output.

**Why this, and not `tier_t_union`'s `LiveCount::Two` term — the registered
counter-example Ruling 2 names.** Revision 1 used `tier_t_union`, which
includes `LiveCount::Two` (two own stones). `DEF-PLAN`
(`threat_calculus_v1.md:29`) requires ≥4 own stones for a plan to exist;
`PAT-O3` (`:108`) records an open three (a fortiori two) has t = 0, "no
plan." MEASURED, `U3_tier_t.md` §6.2 census (verified directly against the
file at this revision, not through a review's paraphrase): trigger (c) as
revision 1 specified it is reachable only on a `Cover::NothingToBlock` row
(§3.2 disposes of the rest), which is the `BATCHED nodes` row — `70.8% /
61.5% / 65.5% / 92.5%` (corpus roots / r2 draw / r8 draw / playouts,
`U3_tier_t.md:172`) — and on those nodes Tier T (`option C — Tier T
(threshold, ADOPTED)`, `:181`) averages `23.29 / 31.50 / 30.26 / 48.73`
cells. **This is the counter-example this ruling's scoping must kill**, per
the resume dispatch's own instruction: `cells_raising_to_hot(us,
NearHot::Three)` is a strict subset of one term of `tier_t_union`'s six-way
union (`live_cells_at_count(Two) ∪ live_cells_at_count(Three) ∪
threat_cells`, both sides, `staged.rs:294–334`) — the `LiveCount::Two` term,
whose windows cannot hold a plan by `DEF-PLAN`, is entirely absent, and so is
the opponent's side (this trigger is about the MOVER creating a plan;
trigger (b) already owns the opponent's). `NearHot` is closed at `Three` in
the shipped surface for exactly this reason (`query.rs:101–107`: "no single
cell raises a count-2 window to hot") — the type itself refuses the
`LiveCount::Two` reading.

`t ≤ 2` in Ruling 2's phrasing: with `HitBudget` closed at two
(`query.rs:70–77`), any plan family a single activation creates is
automatically within the representable range the defender's next turn can be
asked about — a lone newly-created plan has t = 1 (one cell hits it) unless
the SAME activating stone simultaneously completes two independent live-3
windows into hot (a genuine fork, `BOUND-CONVERT`'s subject,
`threat_calculus_v1.md:93`: "one new stone converts ≤3 pre-threats into
threats on hex"), in which case t = 2 for the resulting family. The query
does not need to compute this t itself at generation time — it is what the
CHILD gate's own `blocking_covers` call (§3.2, re-run at the new position by
the recursion, §3.7) establishes, exactly the way the main search already
composes one-ply decisions into a turn.

Tier F for `us` is PROVABLY EMPTY here (trigger (a) answered `None`, which at
`left` forbids both a win-in-one-ply cell and a hot four-stone window —
`batched()`'s own argument, `staged.rs:262–266`).

### 3.4 Gate step 4 — empty hitting set: losing band, no search (Ruling 1, simplified from revision 2's `is_pv` handling)

`Cover::Impossible` (t ≥ 3, `LAW-OVERLOAD`, `threat_calculus_v1.md:55–59`):
**"empty hitting set = losing band, no search"** — return
`-mate_in(turns_from_root + 2)` directly, unconditionally, with no candidate
generation and no dependence on `is_pv`. This REPLACES revision 2's §3.5
(`is_pv`-conditioned generation of a real BatchedLost line, ruled on by the
first re-review and then found to conflict with §4/§5 by the second, NEW-7):
Ruling 1's "no search" is unambiguous and removes the need for that
distinction entirely inside quiescence. `LAW-OVERLOAD` needs only "attacker
t ≥ 3" (established) and "defender cannot win this turn" (§3.1's `None`);
`is_pv` was never one of its conditions, and a line ending at a turn
boundary is turn-whole regardless (`pv.rs:76–79`, `turns_from_plies` panics
only on an illegal ply or a turn half played — neither applies at a gate).
**PV integrity note carried forward from the prior review's independent
re-derivation** (`wp16_design_REVIEW_rev2.md`, answer to question 4): this
holds without needing `is_pv` at all, which is why Ruling 1 can drop the
distinction Ruling-1-free revision 2 needed.

### 3.5 Ply-2 generation — one rule, applies uniformly after either a defensive or offensive ply-1

**Ruling 1, quoted:** "defensive ply-2 = hitting cells of remaining live
opponent plans t ≤ 2, union mover plan-making cells t ≤ 2." Realized as: at
the `Phase::Second` node reached after ply-1 (one stone left,
`left' = StonesLeft::One`), recompute both remaining queries fresh against
the new position (the incrementally-updated `ThreatState` already reflects
the ply-1 stone, `position.rs:143`):

**No ply-2 win check — PROVABLY unreachable, not merely omitted (revision 5
correction of a defect the fourth reviewer found in revision 4's own
addition, `wp16_design_REVIEW_rev4.md` F1).** A `Some` from
`can_win_this_turn(us, StonesLeft::One)` at this node can never occur, by
the same argument §3.3 already gives for Tier F: an extension is granted
only after the GATE's own trigger (a) answered `None` at
`StonesLeft::Two` (§3.1), which means `us` holds no live window at
`own >= 4` at all — neither a win-in-one-ply window (`own == 5`) nor a hot
window at `own == 4`. Ply-1 raises at most one window's own-count by one,
so after it the maximum reachable is `own == 4`, still short of the `own ==
5` a win-in-one-ply window requires. `can_win_this_turn(us, StonesLeft::One)`
is therefore `None` at every ply-2 node this design reaches, always.
Revision 4 added a check for this case with the opposite claim ("a position
could have zero live-3 windows and zero remaining opponent plans while
still holding a stone that wins outright") — false, and it contradicted
this same document's own §3.3 in the process; a `Some` branch reached the
`Phase::Second` node with no stone placed, `wp16_design_REVIEW_rev4.md`'s
F1 finding, the exact defect class §3.5's OTHER correction (below) exists
to close. Recording the proof rather than the check is both correct and
more informative.

- `Cover2 = threats.blocking_covers(us, HitBudget::from(left'))` —
  `HitBudget::One`, since one stone remains. `NothingToBlock` → nothing left
  to hit. `Minimal(covers)` → **candidate cells include `Cover2::cells()`**
  (the "remaining live opponent plans" — plans the ply-1 stone did not fully
  resolve: reachable when ply-1 played a `MinimalCover::Two` member rather
  than the size-1 hit, or when a wholly different plan already existed
  alongside the t≤1 family the gate tested). `Impossible` → **"empty hitting
  set = losing band, no search"** applies again — but "no search" means no
  RECURSION past this stone, not no stone. **Revision-4 correction, closing
  the second defect the fresh reviewer found (B2, `wp16_design_REVIEW_rev3.md`):**
  revision 3 returned `-mate_in(turns_from_root + 2)` here with NOTHING
  placed and the ply-2 PV slot left cleared — the parent then promoted a
  ONE-PLY line into a two-stone turn, which `turns_from_plies` refuses with
  `PV_NOT_PLAYABLE` (`pv.rs:101–105`) the moment that branch is the node's
  best, exactly the failure `D-104` describes: "the mover's second stone,"
  not a missing value. §3.4's own "no dependence on `is_pv`... a line ending
  at a turn boundary is turn-whole" argument is correct only at the GATE
  (`Phase::First`); the reviewer's finding is precise that it does not
  transfer to this `Phase::Second` node, and this document does not restate
  it here as if it did. **The fix: place §3.5a's completion stone (tier 1,
  falling back to tier 2) exactly as if the candidate union had come up
  empty by the ordinary route — because for the purposes of "this branch
  needs a legal second stone and nothing else," `Cover2::Impossible` and an
  empty non-`Impossible` union are the SAME situation — then score the
  resulting, now turn-complete position with `-mate_in(turns_from_root + 2)`
  directly, still with no further recursion.** This keeps "no search"
  meaning exactly what it always meant (no exploration of alternatives, no
  deepening) while making the branch's OWN PV turn-whole, the same way every
  other branch's is. Per-BRANCH: the branch that chose this particular ply-1
  stone is scored and abandoned at ply-2, not the whole turn (other ply-1
  branches, including the one that played the actual size-1 hit
  `MinimalCover::One` names, are unaffected and reach `NothingToBlock`/
  `Minimal` normally, per rule 3 and per the real game's own two-stones-per-turn
  rule, which this correction now honours rather than merely asserts).
- `threats.cells_raising_to_hot(us, NearHot::Three, &mut cells)`, recomputed
  at the ply-2 position — "mover plan-making cells" — union'd in
  unconditionally (available whether or not `Cover2` found anything left to
  hit; `LAW-LEDGER`'s free stone, §3.7, is what this cell set spends).

Ply-2 candidates = `Cover2::cells() ∪ cells_raising_to_hot(us, NearHot::Three)`
at the ply-2 position (deduplicated) — **except in the two cases §3.5a
handles: the union comes up empty, or `Cover2::Impossible` fired** (the
bullet above's revision-4 correction routes that case through §3.5a too,
rather than returning with nothing placed). No quiet stage, no
`within_radius`, no `staged_candidates` call anywhere in this path,
including §3.5a — every cell in every ply traces to one of the four queries
named in §3.1–§3.3/§3.5/§3.5a, per Ruling 1.

**Offensive pairs (Ruling 1's third clause).** When the gate fired trigger
(c) rather than (b) (§3.3, `Cover::NothingToBlock` at the gate), ply-1's
cells are `cells_raising_to_hot(us, NearHot::Three)` and ply-2 uses the SAME
uniform rule above — recomputing `Cover2` (in case the ply-1 offensive
stone incidentally landed inside an opponent window and changed something,
or a plan appeared from an unrelated cause) union'd with the offense query
recomputed. One ply-2 rule, not two.

### 3.5a THE COMPLETION STONE — Ruling 3 (resume-2), resolving `D-391`'s open question (B1/B2)

**The defect this section fixes, restated precisely.** The fresh reviewer of
revision 3 (`wp16_design_REVIEW_rev3.md`, B1) found `Cover2::cells() ∪
cells_raising_to_hot(us, NearHot::Three)` can be EMPTY at `Phase::Second` on
the design's own MODAL defensive line — gate fires at t=1, ply-1 plays the
single hitting cell, `LAW-HIT` kills the opponent's hot windows outright
(`Cover2` becomes `NothingToBlock`, contributing nothing), and the mover
owns no live-3 window (MEASURED ~47% of positions at corpus roots per §9's
own Poisson estimate off `U3_tier_t.md`'s `live-3 own` row). Ruling 1
excludes `staged_candidates`'s quiet-ball safety net and `within_radius` by
name (§3, §4) — the only two fallbacks the codebase already has for an
empty candidate set — so this state needed a THIRD, dedicated answer.
**D-104's `NO_CANDIDATES_MID_TURN` invariant is what an unaddressed empty
set at `Phase::Second` hits** (`pvs.rs:479–486`, a release-active `assert!`)
— that ADR's own flip clause anticipates exactly this WP: it "flips when a
candidate policy that can run dry mid-turn arrives, which must answer
inside a turn." This section is that answer.

**Ruling 3 (resume-2), quoted:** "Empty trigger-derived set at `Phase::Second`
→ exactly one deterministic completion stone, branching 1: argmax
`Eval::delta` over the union of live windows' support; if that support is
empty, argmax delta over the 6-neighbors of the mover's ply-1 stone; fixed
coordinate-order tie-break. Completion is not search; `staged_candidates`
and `within_radius` remain excluded; D-104's `NO_CANDIDATES_MID_TURN`
invariant is satisfied by construction."

**Trigger — two routes, one mechanism.** (1) `§3.5`'s union
(`Cover2::cells() ∪ cells_raising_to_hot(us, NearHot::Three)`, both
recomputed at the ply-2 position — a win at this node is already provably
impossible, §3.5's own proof, above) is empty; or (2) `Cover2::Impossible`
fired at ply-2 (§3.5's revision-5 correction — B2's fix), where the
completion stone is placed before the mate-band score is assigned rather
than in place of a score. A non-empty §3.5 union, with `Cover2` not
`Impossible`, is searched exactly as §3.5 already specifies and never
reaches this section.

**PV and TT participation — plain, one reading.** The completion stone is
not a special non-move: `self.position.place(at)` places it exactly as any
other candidate in this design's alpha-beta loop (§7's "same
full-window-first / null-window-scan shape `Run::child` already
implements"), so it clears and promotes the PV at its own ply
(`pvs.rs:194,341`) the same way, and if it is the best-scoring candidate at
its node — which, at branching 1, it always is, being the only candidate —
it becomes that node's `Record.best` under §6's store rule, unchanged by
being a completion pick rather than a threat-derived one. Route (2)'s
placed-then-shortcut-scored stone participates the same way: the PV records
the real stone played, and the score attached to it is `-mate_in(...)`
rather than a recursively-computed value — a real move paired with a
zero-recursion score, the same shape §3.5's `!is_pv` `OverloadReturn` case
already has at a non-`Phase::Second` node (`staged.rs:196–201`), just paid
for here with one placed stone instead of zero.

**Tier 1 — the union of live windows' support.** "Live windows' support," in
`ThreatState`'s own vocabulary and using only queries already in play
elsewhere in this design (no new solver primitive, matching §3's discipline
throughout): the deduplicated union of

- `threats.threat_cells(side, ...)` (hot windows' empties, `query.rs:178–180`)
- `threats.win_in_one_ply_cells(side, ...)` (`query.rs:165–167`) — for `us`
  this is PROVABLY EMPTY by the win-check just above (same argument §3.3
  gives for Tier F); included here only for symmetry with the opponent's
  side, where it is not provably empty
- `threats.live_cells_at_count(side, LiveCount::Two, ...)` (`query.rs:206–208`)
- `threats.live_cells_at_count(side, LiveCount::Three, ...)` (same)

for BOTH `side = us` and `side = us.opponent()` — eight calls, unioned and
deduplicated. **Deliberately NOT** `ThreatState::table_snapshot`
(`state.rs:148–150`), whose own doc reads "**Never on a choice path**: see
`WindowTable::snapshot`, whose doc says why the table underneath may be
hashed at all" — a completion pick is exactly a choice path, so this design
uses the same four per-side queries the rest of §3 already relies on rather
than the raw table. `LiveCount::Two` is INCLUDED here even though §3.3
(C1's own fix) deliberately excludes it from a TRIGGER — the two are
different questions: §3.3 asks "is this cell worth EXTENDING search for"
(where `LiveCount::Two` cannot license a plan, `DEF-PLAN`), this asks "is
this cell a PLAUSIBLE, non-arbitrary place to put a stone the position
already has no better answer for" (no plan claim, no extension, branching
exactly 1) — a materially weaker bar, and Ruling 3 sets it explicitly.

**Revision 5, N3-fixed:** the union of the eight queries' output is NOT
itself sorted — each query sorts and dedups only its OWN buffer
(`query.rs:10–12`'s module doc: every cell query clears its own `out` and
never appends), so the eight results must be concatenated and THEN sorted
and deduplicated as one step — `cells.sort_unstable(); cells.dedup();` —
exactly what `tier_t_union` already does when it combines multiple queries'
output (`staged.rs:331–333`). IMPL following this construction gets it
right by the same pattern already in the codebase; a literal "already
sorted" reading (revision 4's wording, corrected here) would concatenate in
call order, which is deterministic but not the ascending `(q, r)` order the
tie-break below assumes, and `Vec::dedup` alone would miss non-consecutive
duplicates across the eight buffers.

Among this (now genuinely sorted) union's cells, if non-empty, pick
`argmax` over `self.position.static_score_after(at)` (`position.rs:130–133`,
the existing `Eval::delta` roundtrip already used for move ordering
elsewhere in this codebase — no new eval entry point). **Fixed
coordinate-order tie-break, matching D-5/D-7's established convention**:
iterate the sorted `(q, r)`-ascending set and keep the running best only on
STRICT improvement, never on a tie — the first-encountered maximum survives
ties, exactly the "stable sort... leaves equal-scoring cells in the
ascending coordinate order they arrived in" rule `staged.rs`'s own
`delta_rank` already states (`staged.rs:336–339`).

**Tier 2 — the mover's ply-1 stone's own six neighbors, if Tier 1 is
empty.** `pistol_core::axis::NEIGHBOUR_DIRECTIONS` (`axis.rs:64–77`) gives
the six offsets — `Coord::offset` (`coord.rs:72`) applied to each yields the
candidate set, filtered to cells NOT `self.position.board().is_occupied(at)`
(`board.rs:90`). **Revision 5, N4-fixed:** iterating in
`NEIGHBOUR_DIRECTIONS`' own ring order and iterating the same six cells
sorted ascending `(q, r)` are TWO DIFFERENT orders and pick different cells
on a tie (worked example: the six offsets' ring order is `(+1,0), (+1,-1),
(0,-1), (-1,0), (-1,+1), (0,+1)`; their ascending-`(q,r)` order is
`(-1,0), (-1,+1), (0,-1), (0,+1), (+1,-1), (+1,0)`). Ruling 3 says "fixed
coordinate-order tie-break" — the SAME rule Tier 1 and D-5/D-7 use — so
Tier 2 sorts its (at most six) surviving candidates ascending `(q, r)`
before the argmax runs, exactly as Tier 1 does; `NEIGHBOUR_DIRECTIONS`'s
ring order is used only to enumerate the six candidate cells, never as the
tie-break order.

**Totality — what Phase 1'' is asked to verify, argued here rather than
merely asserted.** Every Tier-1 cell is, by `fill_empties`'s own contract,
already both empty and legal (§3.6's argument, unchanged). Every Tier-2
candidate is legal by construction — a neighbor of an already-placed stone
is trivially within hex-distance 1, hence within radius-8, of that stone
(rule 5's own legal-region definition) — and is filtered to unoccupied
before the argmax runs, so nothing illegal can be selected at either tier.
**Where this chain is NOT proven total, named rather than silently assumed
complete:** if Tier 1 is empty (no live window anywhere carries either
side's stones at count ≥2 through the mover's or opponent's classified
windows) AND all six of the ply-1 stone's neighbors are already occupied,
this section names no third tier. This is an EXTREME board-density
configuration — it requires the cell the search just legally placed a stone
on to already have six stones immediately surrounding it while nothing
anywhere on the board qualifies for any live classification — and this
document does not claim it is impossible, only implausible at any reachable
mid-search position. Per rule 3's fail-loud discipline (the same discipline
`StonesLeft::from_state` and `min_hitting_set_exceeds` already apply to
their own "should not happen through normal play" cases), IMPL names this
`NO_COMPLETION_STONE` and panics rather than silently falling through to a
forbidden generator or a mid-turn static read. **RED-TEAM (Phase 3 of the
overnight dispatch, when reached): construct a fixture for this exact
boundary** (a dense cluster around the ply-1 stone, zero live windows
elsewhere) to confirm the panic fires rather than something silent.

**Soundness — revision 5's restatement, using the stronger argument the
fourth reviewer independently re-derived (`wp16_design_REVIEW_rev4.md`,
scope item 3): the completion stone is always a member of the ply-2 node's
TRUE legal move set (§3.5a's own totality argument, above), and that node
is a MAX node over that set — restricting the set to one member can
therefore only lower the value the node reports, an under-claim,
unconditionally, by ordinary max-over-a-subset arithmetic and without
needing `LEM-MONO` at all.** `LEM-MONO` (`threat_calculus_v1.md:40`,
"stones are never removed; an own extra stone never hurts") would be the
load-bearing premise only if this design compared the completion stone
against NOT placing one — a null move, which rule 3 forbids outright — so
citing it here over-justifies rather than under-justifies the claim; this
revision keeps the citation as context, not as what the argument rests on.
On route (1) (§3.2 fired, a genuine defensive extension) the completion
stone's under-claim is a LOWER BOUND on the mover's survival, which is what
a defensive extension needs. On route (2) (`Cover2::Impossible`) the value
is not merely a lower bound — it is INVARIANT over the choice of completion
stone: `Cover2::Impossible` at `HitBudget::One` means some hot window
survives whichever single cell is played (`LAW-OVERLOAD`'s own criterion,
`cover.rs:201–244`), so the opponent completes it next turn regardless, and
neither the win-check just above nor the completion pick can change that.
On the offensive branch (§3.3 fired, not §3.2), the same argument gives an
under-claim — the search may fail to find the best continuation and report
a value lower than the position's true worth, which is the error direction
alpha-beta pruning already tolerates everywhere (a null-window scan that
under-explores also only ever under-claims, never over-claims, `pvs.rs:406–423`)
and never the direction that would corrupt a proof or a bound the rest of
the tree relies on.

### 3.6 What Ruling 1 does NOT need this document to re-derive

Ply-1's cells cannot include a cell `Position::place` would refuse: every
cell named above is the empty cell of a window some side already has ≥2
stones in (`DEF-WINDOW`, live windows occupy real board positions within
radius-8 of existing stones by construction) — the same cells the shipped
Tier T generator already hands to `place` today without `CANDIDATE_ILLEGAL`
firing (`wp16_design_REVIEW_rev2.md`, answer to question 1, re-derived
independently there and not re-derived again here).

### 3.7 LAW-RIPOSTE / LAW-LEDGER — what this design expresses, in D-267's assignment

D-267 (`docs/decisions.md:575`) assigns `LAW-RIPOSTE` and `LAW-LEDGER` to
THIS WP by name ("are WP-1.6's"), not to Stage 3 — this citation is verified
against the CURRENT `decisions.md` at this revision (Ruling 5, §7 below is
this fix's own section for the TT citation; this paragraph is the general
one). `LAW-LEDGER` (`threat_calculus_v1.md:79–83`): "the defender's turn is
worth 2 − t free stones." At the gate's t=1 case, the mover banks exactly
one free stone — §3.5's ply-2 rule is that free stone's spend: hit whatever
of the opponent's family ply-1 left live, OR develop, whichever the position
calls for, decided by the search itself rather than by this document.
`LAW-RIPOSTE` (`:74–77`): "a forced defensive stone can itself create a plan
and flip initiative... any forcing-line PROVER must check every forced
reply for new plans." Quiescence is not a prover — it never claims a proof —
but the SAME check happens for free by recursion: a forced ply-1 stone that
creates a new plan is exactly what §3.5's `cells_raising_to_hot` re-query
(and, one turn later, a fresh gate at §3.1–§3.4) would surface, bounded by
`q_budget` (§6); running out of budget means "stand pat" (an admission of
ignorance, sound for alpha-beta), never "unsound proof" (the failure mode
`LAW-RIPOSTE` warns provers against, and one this design never claims to
avoid by proof).

## 4. Zones — window-support bound, never radius

Every cell any qsearch node can generate (§3.2, §3.3, §3.5) is the empty
cell of a window some side already holds ≥2 (opponent, via `blocking_covers`
→ `hot_windows`, ≥4 in fact) or ≥3 (mover, via `cells_raising_to_hot`) own
stones in — never a `within_radius` ball
(`crate::candidates::within_radius`, `candidates.rs`). Verified exhaustively
(`wp16_design_REVIEW.md` V4, re-checked and still true at this revision):
`within_radius` is `pub(crate)` with exactly two call sites, `candidates.rs`
under the `Radius` policy arm and `staged.rs:283`'s quiet-ball safety net —
neither is `quiescence.rs`, which this design never has call into either.
This holds STRUCTURALLY now, more strongly than in revisions 1–2: those
designs suppressed `staged_candidates`'s safety net at gate decisions but
still literally CALLED `staged_candidates`, leaving the suppression a
runtime condition to get right; this design's generator never imports
`staged.rs` or `candidates.rs` at all, so there is no radius-based function
in the call graph to suppress.

Not `ZONE-R` (`threat_calculus_v1.md:137–141`, RZOP's finite-proof relevance
zones, Stage 3 / WP-1.8's per D-267) — a cheaper, weaker relative sharing
`ZONE-R`'s motivating principle without its proof-engine machinery.

## 5. Cap: `q_depth_turns` (unchanged from revision 2)

New field on the `staged` variant of `[search.candidate_policy]`
(`crates/pistol-engine/src/config.rs:161–182`) and on
`pistol_search::params::StagedParams` (`params.rs:58–70`) — schema-home per
hard rule 1, no code-side default. `u32`, validated range `0..=8`. **`0` is
the disable flag**: `quiescence()`'s gate still runs §3.1/§3.4's free
checks (they are not extensions — no turn is granted — but they are NOT
free of cost: `blocking_covers` is `D-388`'s own registered HOTSPOT, "trigger
evaluation at horizon nodes," `O(|universe|²)` over the hot-window family,
`cover.rs:216–239` — revision-4 correction of a sentence the rev-3 review's
M6 found contradicting §9's own quoted registration in the same document),
but §3.2/§3.3 never grant a turn. This is the "quiescence disabled" state
the Phase-2 differential oracle needs.

**Configs that must gain the field in the landing commit — revision-4
correction of the list, closing B4 (`wp16_design_REVIEW_rev3.md`: "wrong in
both directions"; the reviewer's own corrected enumeration, re-verified
against the tree at this revision, is what follows):**

- **Five `configs/*.toml` files carry `[search.candidate_policy]` with
  `kind = "staged"` and must gain the field**: `gate_staged_v0.toml`,
  `instrument_staged_v0.toml`, `instrument_v0.toml`, `play_staged_v0.toml`,
  `tactical_staged_v0.toml`. `instrument_v0.toml` really is the staged
  policy now (D-386, `9282dd0`).
- **`configs/arena_wp15b_staged_vs_r2.toml`,
  `configs/arena_wp15b_staged_vs_r2_confirm.toml` and
  `configs/arena_wp15b_dryrun.toml` do NOT carry the field and must NOT be
  edited** — revision 2 and 3 both listed them in error. They have no
  `[search.candidate_policy]` section of their own (they are dispatched to
  the ARENA schema by basename, `tools/config_check.sh:56–63`, and
  reference the engine configs above by path); adding an unknown key to
  them would be rejected by their own `deny_unknown_fields`, not by
  omission. They inherit the field automatically through whichever engine
  config they name.
- **`crates/pistol-engine/tests/common/mod.rs:43–70`'s `pub const
  VALID_STAGED: &str`** — a complete `kind = "staged"` instrument-mode
  document, the fixture `crates/pistol-engine/tests/config_validate_tests.rs`
  mutates for its twelve staged-validation cases — needs the field or every
  one of those twelve fails on a missing-field error.
- **`StagedParams`'s own construction sites** need the field added to their
  struct literals: `crates/pistol-engine/src/instance.rs:181` and six
  `pistol-search` test files —
  `crates/pistol-search/tests/common/mod.rs:74`,
  `staged_tests.rs:88`, `staged_colony_family_tests.rs:122,151`,
  `staged_differential_gate_tests.rs:126`,
  `staged_pattern_fixture_tests.rs:51`,
  `staged_tier_t_threshold_tests.rs:96`.

`config.rs:159–160` nominates `U3_tier_t.md` §10 as the schema document
naming staged documents' shape; that document's own count should move from
five keys to six alongside this landing commit, so the citation stays true
rather than merely the field.

## 6. RULING 4 — the TT rule, stated once, corrected against a hard constraint neither prior review found

**The rule, quoted from the resume dispatch:** "qnodes store at depth 0 with
a distinct quiescence bound-type flag, excluded from main-depth node
accounting. No alternative reading appears anywhere."

**A load-bearing finding this revision makes that revisions 1 and 2, and
both of their reviews, did not: the literal words "store at depth 0" cannot
be implemented against the current packed TT entry, and this is not a
judgment call.** `tt/entry.rs`'s module doc, verbatim: "**Zero depth means
empty.** No stored record has it: a leaf is not worth an entry, so the depth
field doubles as the occupancy flag" (`entry.rs:9–11`). Its `depth_fits`
function enforces this as a hard invariant: `assert!(depth >= 1, ...\"a leaf
is not worth an entry\")` (`entry.rs:174–178`), and `Entry::is_empty`
(`:138–140`) is LITERALLY `self.depth_plies == 0`. A `Record { depth_plies:
0, .. }` passed to `Table::store` (`tt/mod.rs:158–171`, which packs via
`Entry::packed`, `entry.rs:113–123`) panics with `TT_FIELD_OUT_OF_RANGE`
before it ever reaches a bucket. Neither revision 1's nor revision 2's
review caught this, because both focused on the CONSEQUENCES of a depth-0
store (whether a full-width prober would wrongly trust it) and neither
checked whether the packed format can hold a stored zero at all. It cannot.

**Resolution adopted by this revision — the ruling's DEPTH-0 is realized as
a semantic width-class, not a literal `depth_plies` value:**

1. **Store `depth_plies: 1`** for every quiescence-regime `Record` — the
   smallest value the occupancy-flag convention (`entry.rs:9–11,138–140`)
   allows a real, present entry to carry. `1` is otherwise a legitimate
   full-width depth (a node one ply from its own horizon reaches it
   constantly), so depth alone cannot distinguish a quiescence record from a
   genuine depth-1 full-width one — which is exactly why the ruling asks for
   a flag.
2. **Add one bit, `from_quiescence: bool`, to `Entry` AND to the public
   `Record` type it packs from/unpacks to** (`entry.rs:63–75`,
   `Entry::packed`/`Entry::record`, `entry.rs:113–135` — closing the rev-3
   review's M2: "the flag has to live on `Record`, not only on `Entry`...
   §6's stated resolution does not typecheck as written"; item 4 below's
   `Record { ..., from_quiescence: true, ... }` construction is this fix
   already applied), WITHOUT reducing
   `GENERATIONS` (64 → fewer would shrink the generation-wraparound horizon
   inside a single long game, a real cost this design does not want to pay)
   and WITHOUT touching `bound_age`'s existing `(generation << 2) |
   bound.index()` packing (`entry.rs:121`), which already uses all 8 of its
   bits. `Entry`'s declared fields sum to 18 bytes (`verification: u64` = 8,
   four `i16` fields = 8, `depth_plies: u8` + `bound_age: u8` = 2) against
   the asserted `ENTRY_BYTES = 24` (`entry.rs:16,100–103`) — six bytes the
   current layout spends on alignment padding and nothing else. **IMPL adds
   a `flags: u8` field** (one of those padding bytes, made an explicit field
   instead of implicit padding) carrying `from_quiescence` in its low bit;
   the existing `size_of::<Entry>() == ENTRY_BYTES` const assertion
   (`entry.rs:100–103`) is the gate that catches it if this layout
   assumption is wrong on some target — a compile-time check this codebase
   already had, not a new one this design adds. **VERIFIED, not merely
   asserted**: the fresh reviewer of revision 3 compiled the current
   `Entry` struct plus a `+ flags: u8` variant under `rustc -O --edition
   2024` (`#[repr(Rust)]`, field reordering in play) and confirmed both are
   24 bytes with `BUCKET_BYTES` unchanged at 96, headroom for six such
   bytes total (`wp16_design_REVIEW_rev3.md`, §6 assessment). Out of scope
   for Phase 1'' by name (§10) — this item is not re-opened.
3. **Store rule (protects existing data, closes the eviction gap the second
   revision's review found — `wp16_design_REVIEW_rev2.md` NEW-5 — before it
   could land):** `Table::store`'s victim selection (`tt/mod.rs:179–191`)
   gains one condition: if the record being stored has `from_quiescence:
   true` and the chosen slot already holds a non-empty, NON-quiescence
   entry, the store is DECLINED (no-op) rather than overwriting. A
   quiescence store may freely fill an empty slot or replace an existing
   quiescence entry (the table's ordinary `rank()` comparison,
   `tt/mod.rs:193–200`, decides which, unchanged). A full-width store is
   never declined by this rule — it always may evict, exactly as today.
4. **What a quiescence `Record` contains — revision-4 fix, one reading only,
   closing the fresh reviewer's B3 (`wp16_design_REVIEW_rev3.md`: "§6 never
   states what a quiescence `Record` is... IMPL cannot write this store
   without inventing the record").** Exactly the fields `visit`'s own
   end-of-node store already computes (`pvs.rs:359–377`), with two
   substituted: `Record { depth_plies: 1, from_quiescence: true, score:
   best_score, static_eval: self.position.value(), bound: <Exact if
   best_score is strictly between the node's alpha/beta, Lower if it met or
   beat beta, Upper if it never beat the original alpha — the same three-way
   rule `pvs.rs:367–373` already applies, unchanged>, best: best_cell }`,
   where `best_score`/`best_cell` are whatever `quiescence()`'s own
   alpha-beta loop (§7's "same shape `Run::child` already implements")
   already tracks by the time the node returns — including a completion
   stone (§3.5a) when it was the node's only candidate, and including
   route (2)'s placed-then-shortcut-scored stone (§3.5, B2's fix) with
   `score` set to the mate-band value that shortcut assigns. One
   construction, no second reading of what the fields mean.
5. **Probe rule, and its honest cost — read plainly, not softened.** A probe
   returning a record with `from_quiescence: true` is treated by a
   FULL-WIDTH caller (`q_budget == None` context) exactly as if `probe` had
   returned `None` — no cutoff, no move-ordering hint, full stop.
   `quiescence()` itself never probes the table at all — it only ever
   WRITES, once per node it visits (item 4, above), so there is no
   quiescence-to-quiescence reuse question to answer (the second revision's
   review, NEW-2/N2, raised exactly this as unaddressed; this revision
   removes the read path entirely rather than reasoning about what it would
   be sound for). **Consequence, named rather than left for B3 to name
   again:** combined with item 4, this means EVERY quiescence write is a
   pure cost with no reader anywhere in the system — one `Table::store` call
   (key, pack, four-slot victim scan) and one bucket slot held, reclaimable
   only by a full-width store or another quiescence store (item 3), for a
   record nothing ever reads back. This is not "an optimization foregone";
   it is a deliberate, stated choice to accept that cost — landing on `ttd`,
   D-388's PRIMARY metric — in exchange for a store/probe rule simple enough
   that "no alternative reading appears anywhere" is actually true of it. If
   Phase 2's bench shows this cost matters, the one-sentence fix is "item 4
   and item 5 do not run — quiescence does not store" — which deletes the
   `Entry` field, the `victim()` condition, and this item, and is exactly as
   sound (§4/§7's soundness arguments do not depend on the store existing at
   all). This document does not pre-empt that measurement by choosing
   between "store, unread" and "do not store" now; it states both readings
   are available, which one is shipped, and how cheap the other is to
   switch to.

**"Excluded from main-depth node accounting":** `StageCounters`'s existing
fields (`win_now`/`filtered`/`batched`/`batched_lost`/`cover_impossible`/
`overload_return`, `info.rs:39–65`) are fed exclusively by
`StagedRow`/`staged_candidates` (`staged.rs`/`info.rs:69–83`). Since Ruling 1
removes every call from quiescence into `staged_candidates`, this exclusion
is now STRUCTURAL — `quiescence()` cannot increment those fields because it
never reaches the code that does. Quiescence's own activity is recorded
exclusively under §7's new counters. **`self.nodes` is NOT excluded** — the
raw node-budget counter (`Run::nodes`, `pvs.rs:79–80`) still increments once
per node `quiescence()` visits, because `self.stop.is_spent(self.nodes)`
(`pvs.rs:450–456`) is what a `Stop::Nodes` budget spends against, and rule 6
requires per-side compute to be reported truthfully — a search that did
quiescence work for free, invisible to its own budget, would misreport the
compute WP-1.5b's SPRT was matched on (`go nodes 50000`, D-386). The ruling's
"excluded from main-depth node accounting" is read as excluding qsearch
activity from the MAIN-SEARCH-SHAPED counters (`StageCounters`'s existing
fields), not from the node-budget itself — the two are different
accounting systems and only the first is what `staged_candidates`'s counters
were ever about.

## 7. Correctness — the rest, carried forward or restated briefly

**Win detection.** Rule 2: unchanged, pistol-core's alone. `ThreatState` is
derived from and kept in step with `GameState`/`Board`
(`position.rs`'s doc, D-41 as amended by WP-1.5b); trigger (a) reads it but
never substitutes for `PlyOutcome::Win`, which still ends every line
(`pvs.rs:311–321`).

**Scores in turns.** `mate_in`/`to_table`/`from_table` (`score.rs`) used
unchanged; `quiescence()` computes `turns_from_root` the same way `visit`
does and never introduces a ply-counted distance.

**Determinism law.** `can_win_this_turn`, `blocking_covers`,
`cells_raising_to_hot` are all total/deterministic/sorted (`query.rs`'s
module doc). `q_budget` is plain recursion state. `quiescence()` calls
`self.should_stop()` at its own entry — the SAME method `visit` calls
(`pvs.rs:441–457`), not a duplicate — so a reproducible stop's exactness is
unaffected by the extra call site.

**PV tracking.** `quiescence()` calls `self.pv.clear(ply)` / `self.pv.promote(ply,
at)` at the same points `visit`'s own candidate loop does (`pvs.rs:194,341`),
so a granted turn's line is recorded exactly as a full-width one's would be —
required for `Run::salvage`'s pairing invariant (`pvs.rs:169–182`) to keep
holding when an abort lands mid-extension.

**Alpha-beta discipline inside `quiescence()`.** Both plies use the same
full-window-first / null-window-scan-then-conditional-re-search shape
`Run::child` already implements (`pvs.rs:387–425`) — IMPL may factor a
shared helper if it finds one natural; this document does not mandate code
sharing, only the alpha-beta discipline itself, which is what soundness
depends on.

## 8. Counters

`StageCounters` (`info.rs:39–65`) gains fields written only by
`quiescence()`, per §6's structural exclusion from the existing fields:

- `qnodes: u64` — every node `quiescence()` visits (both plies, every
  granted turn, every chain link).
- `q_win_now: u64` — §3.1 fired.
- `q_overload_return: u64` — §3.4 fired, at the gate or at ply-2 (§3.5's
  symmetric case) combined; the WP's own analysis can split gate-vs-ply-2 by
  reading `qnodes` alongside it if needed, not required as a separate field
  here.
- `q_extend_defense: u64` — §3.2 fired at the gate.
- `q_extend_offense: u64` — §3.3 fired at the gate (mutually exclusive with
  defense: `NothingToBlock` vs `Minimal` are disjoint `Cover` arms).
- `q_stand_pat_no_trigger: u64` — gate reached, neither §3.2 nor §3.3 fired
  (includes the t=2-at-the-gate case Ruling 2 deliberately excludes, §3.2).
- `q_stand_pat_cap: u64` — a trigger fired but `q_budget` was already spent.

## 9. Rule-5 registration and RULING 3 — the cost derivation

**D-388's registered text (`docs/decisions.md`), unedited by this
revision — numbers do not move, per rule 5 and per Ruling 3's own text:**

> HOTSPOT = trigger evaluation at horizon nodes (can_win_this_turn + plan-t
> queries per horizon node). INSTRUMENT: existing bench chain, staged+q vs
> staged, ttd PRIMARY per D-374's lesson, nps context. BRACKET (ESTIMATED):
> nodes-to-same-depth inflation <= 2.0x; ABORT if > 3.0x. ttd may worsen;
> strength is SPRT's alone. Numbers do not move.

**Ruling 3, quoted:** "derive expected node inflation from `U3_tier_t.md`
census at the narrow width, whole granted turn, both plies, chain length
capped by `q_depth_turns`; register the derivation ESTIMATED; D-388's
bracket and abort stand UNMOVED (one line stating why: the bracket was
impossible at wide width, which evidenced Ruling 1, not a wider bracket)."

**Why the bracket does not move — the one line Ruling 3 asks for:** the
wide-width design (revisions 1–2) blew the registered bracket by two to
three orders of magnitude BEFORE any bench ran (`wp16_design_REVIEW.md` C1);
that finding is evidence the WIDE design was wrong, which Ruling 1 corrects,
not evidence the BRACKET was set wrong — a design defect and a
mis-calibrated instrument are different failure modes, and only the design
was shown defective. `D-388`'s numbers stand as the standard the NARROW
design (this revision) is held to.

**The derivation, ESTIMATED throughout, MEASURED census inputs cited
directly against `U3_tier_t.md` at this revision (not through a prior
review's quotation):**

Per-ply widths (MEASURED where the census carries the quantity, ESTIMATED
where it is derived from a related quantity the census does carry):

| quantity | corpus roots | r2 draw | r8 draw | playouts | source |
|---|---|---|---|---|---|
| `q_extend_defense` upper-bound rate (FILTERED row, t∈{1,2} — an upper bound on Ruling 2's t≤1-only rate, since the census does not split FILTERED by t) | 25.0% | 18.4% | 13.7% | 3.1% | MEASURED, `U3_tier_t.md:170` |
| ply-1 width when defense fires (`cover union when FILTERED`) | 2.17 | 2.17 | 2.19 | 2.27 | MEASURED, `:168` |
| `live-3 own`, mean WINDOWS (not cells) | 0.75 | 1.78 | 1.61 | 1.71 | MEASURED, `:165` |
| `q_extend_offense` rate, ESTIMATED via a Poisson(mean) `P(≥1)` heuristic on the row above — a rough proxy, not a measured zero-fraction | ≈53% | ≈83% | ≈80% | ≈82% | ESTIMATED |
| ply-1 width when offense fires (empties of live-3 windows, ~3 cells/window before dedup) | ≈3–6 | ≈3–6 | ≈3–6 | ≈3–6 | ESTIMATED |
| ply-2 width (both branches, `Cover2::cells() ∪` offense cells, summed without modelling overlap) | ≈5–8 | ≈5–8 | ≈5–8 | ≈5–8 | ESTIMATED |

**What the census does NOT carry, named rather than silently assumed:** the
FILTERED row's t=1-vs-t=2 split (so the defense rate above is an upper bound,
conservative in the safe direction for a bracket check), and the zero-count
fraction for `live-3 own` (so the offense rate is a Poisson heuristic on the
mean, not a measured probability) — `crates/pistol-solver/tests/wp15b_census.rs`
is the committed, rerunnable harness (D-287) that could extract both exactly
in one pass; Phase 2's bench is where that exact number belongs, not a
further estimate stacked on this one.

**Combining, worst case (no intra-turn alpha-beta pruning — a real
mitigating factor §7 notes but does not quantify further here), one granted
turn's extra node cost ≈ `ply1_width × (1 + ply2_width)`:**

- defense-triggered: `2.2 × (1 + 7) ≈ 18` extra nodes (ESTIMATED)
- offense-triggered: `4.5 × (1 + 7) ≈ 36` extra nodes (ESTIMATED)

**Expected extra nodes per fired horizon** (`rate_b × 18 + rate_c × 36`,
using the upper-bound/heuristic rates above): **≈24 at corpus roots, ≈33 at
the r2 draw, ≈31 at the r8 draw, ≈30 at playouts** (ESTIMATED). Against
today's ~1 node per horizon, this is a **~24×–34× per-leaf inflation,
worst-case, before any chain (`q_depth_turns > 1` compounds this further per
granted extension, capped by the config value; before any intra-turn
alpha-beta pruning, which real search behavior is expected to reduce this
by an unquantified factor).**

**Read honestly, not spun toward the answer the ruling might hope for:**
this worst-case per-leaf figure is well above D-388's `2.0x`/`3.0x` bracket
if it held uniformly across a whole search tree, and it is a substantial
improvement over the wide-width design's ~540–2400 estimate (roughly
15×–70× narrower) without being demonstrably inside the registered bracket.
**This document does not resolve which side of `2.0x`/`3.0x` the real,
pruned, chain-capped figure lands on — that is what Phase 2's registered
bench measures, per Ruling 3's own instruction, and the ABORT clause stands
ready to fire exactly as registered if the bench confirms the worst case.**
Reporting an uncomfortable ESTIMATE rather than rounding it toward comfort
is what rule 5's registration discipline is for.

**N4 from the prior review, still open, still Phase 2's to name (unchanged
finding, carried forward):** `q_depth_turns == 0` already changes behavior
(§3.1/§3.4's free checks return mate scores at horizons a pre-WP-1.6 build
scores statically) — Phase 2's bench states which right-hand seat "staged+q
vs staged" uses before launching.

## 10. Revision history, and what the next (fifth) reviewer attacks

**Revisions 1–3, and revision 3's fresh-reviewer FAIL, are `D-389`/`D-391`'s
history** and are not restated here. **Revision 4** added §3.5a (THE
COMPLETION STONE) from the resume-2 dispatch's third architect ruling,
closing revision 3's B1 (an empty ply-2 candidate set on the modal
defensive line) and, via a correction to §3.5's `Cover2::Impossible` case,
B2 (a half-turn PV) — plus plain, one-reading fixes closing B3 (§6 items
4–5) and B4 (§5's config checklist). A SCOPED fourth-slot review
(`wp16_design_REVIEW_rev4.md`) **PASSED the completion mechanism itself**
(existence/totality, determinism, soundness, and the named B2 correction —
its scope items 1, 2, 3, 5, 6 all PASS) but **FAILED on scope item 4**:
revision 4 separately added a `Phase::Second` `can_win_this_turn` check,
justified as closing "a hole neither the win-set nor either prior review
named" — the reviewer proved this check is BOTH unreachable (the same
argument §3.3 already gives for Tier F, applied one ply later: the gate's
own trigger (a) already establishes `us` holds no live window at `own >= 4`,
and one stone cannot raise a window past `own == 4`) AND, when read as
written (`Some` → terminal, no stone placed), the EXACT B2 defect class
reintroduced six lines below where B2 was just fixed.

**Revision 5 is a small, self-correcting delta on revision 4**: it deletes
the false ply-2 win-check and replaces it with the reviewer's own
unreachability proof (§3.5, "No ply-2 win check"), and folds in three
adjacent one-clause precision findings the same review named — N3 (the
Tier-1 union must be sorted after concatenation, not assumed
"already-sorted"), N4 (Tier 2's tie-break is ascending `(q, r)`, not
`NEIGHBOUR_DIRECTIONS`' ring order — the two differ and the document now
says which), and N5/N6 (§3.5a's soundness paragraph restated using the
reviewer's own STRONGER, independently-derived argument — the completion
stone is always a legal-set member at a max node, so restricting to it is
an under-claim by ordinary max-over-a-subset arithmetic, with `LEM-MONO`
kept as context rather than as the load-bearing premise, and route (2)'s
score stated as INVARIANT over the completion choice rather than merely
safe). **Nothing else in the document changes.** The rev-4 review's
optional-polish items (N1) and its out-of-scope observations (O1 `MAX_PLY`/
`seldepth_turns`, O2 = rev-3's M3/M9, O3/O4 citation/housekeeping trivia)
are **deliberately NOT touched** — none is a normative contradiction (the
rev-4 review's own finding: "No STOP-3-class normative contradiction found
anywhere in or out of scope"), left as recorded, known debt.

**Everything revision 4's scoped review passed, and everything revision 3's
review passed before it, is UNCHANGED by this delta and is OUT OF SCOPE for
the next reviewer by name**: §3.5a's existence/totality/query-composition
and determinism arguments themselves (only the sort/tie-break WORDING
changed, not the claims — N3/N4 are precision fixes, not new claims), the
`is_pv`-drop B2 correction's own soundness (unchanged), §6 items 1–3 (the
TT byte-layout finding), §9 (the cost derivation), the `LAW-RIPOSTE`/
`LAW-LEDGER` discharge argument, and the move-set structure (§3.1–§3.4,
§3.6, §3.7, §4). Both prior review reports are the list of what already
passed; the next reviewer should not re-litigate them.

**The next (fifth) reviewer, scoped narrowly, attacks ONLY:**

1. **§3.5's "No ply-2 win check" replacement** — is the unreachability
   proof actually correct (re-derive it independently), and does deleting
   the runtime check rather than keeping-and-fixing it (the rev-4 review's
   alternative (b): place the witness before scoring) lose anything? The
   document chose deletion because it is "both correct and more
   informative"; confirm that trade explicitly.
2. **N3's fix** — does the stated sort-after-concatenate construction
   actually produce a correctly deduplicated, ascending-`(q,r)` Tier-1 set,
   verified against `tier_t_union`'s own pattern (`staged.rs:331–333`)?
3. **N4's fix** — does specifying ascending `(q, r)` for Tier 2 (rather than
   `NEIGHBOUR_DIRECTIONS`' ring order) actually match D-5/D-7's convention,
   and is the worked example in the document correct?
4. **N5/N6's restated soundness paragraph** — re-derive the max-node
   under-claim argument and the route-(2)-invariance argument independently
   rather than trusting this document's restatement of the rev-4 reviewer's
   own derivation.

PASS → proceed directly into the original overnight dispatch's Phase 2
(IMPL) and run it to closure or its STOP states as written there — no
re-entry to Phase 0 needed. FAIL on any of the four items above → the
orchestrating session weighs it the same way it weighed this round's
finding: a defect in the completion mechanism's actual soundness/
determinism/totality is the dispatch's hard stop; a narrow, provably
self-contained slip in the delta's own new wording is not, and gets one
more precise, minimal fix rather than a full stop — record which kind it is
before acting.
