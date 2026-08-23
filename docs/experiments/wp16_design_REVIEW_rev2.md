# REVIEW-design, SCOPED RE-REVIEW — `docs/wp16_quiescence_design.md` revision 2 (WP-1.6)

**Revision reviewed:** `b1ba74651de9d076a05d08a7e1ff490d092d4b85` (`b1ba746`).
**Matches HEAD:** YES (`git rev-parse HEAD` = `b1ba746`; working tree clean apart
from this report, which is untracked).
**Reviewer:** fresh-context REVIEW-design subagent, not the implementing session,
not the reviewer of revision 1.
**Scope:** re-review of the six findings C1–C6 of
`docs/experiments/wp16_design_REVIEW.md` (against `9fa27c8`), of defects the fixes
themselves introduce, and of the two VERIFIED items revision 2 touched or claims
unchanged (V3, V4). V1/V2/V5/V6/V7 were not re-litigated except where the diff
lands next to them.
**Sources read directly, not through revision 2's or revision 1's review's
paraphrase:** `docs/experiments/wp16_design_REVIEW.md` (whole),
`docs/wp16_quiescence_design.md` (whole), `git diff 9fa27c8 b1ba746` on that file,
`docs/research/threat_calculus_v1.md` (whole), `docs/decisions.md` D-267 and D-352,
`crates/pistol-search/src/{pvs.rs,staged.rs,info.rs,params.rs,position.rs,pv.rs,tt/mod.rs}`,
`crates/pistol-solver/src/{query.rs,cover.rs}`,
`crates/pistol-engine/src/config.rs`, `docs/experiments/U3_tier_t.md` §6.2 census
block **and** `crates/pistol-solver/tests/wp15b_census.rs` (the instrument that
renders it).

## VERDICT: **FAIL** — STOP 1. The work package halts; the operator must intervene.

C2, C3, C4, C5 and C6 are genuinely closed and I could not break them. C1 is not:
its *gate condition* is fixed exactly as revision 1's reviewer recommended, but
C1's other half — the WIDTH the granted turn is searched at, which is where the
measured 61.5–92.5 % × 23–49 cells argument actually lived — is untouched, and the
edit that fixed the gate has put §3.3 and §5 into direct contradiction with §4 and
§7 about what the extension's move set even is. The two readings differ by more
than an order of magnitude in the single quantity §9's registered bracket and ABORT
threshold are stated in, and IMPL cannot pick one from the document.

This is the same defect class as C2 (a specification whose two available readings
differ materially and which licenses neither), now relocated from §7 to §3.3/§4/§5.

---

## Per-finding table

| # | verdict | one line |
|---|---|---|
| **C1** | **FIXED-BUT-NEW-ISSUE (blocking)** | The `LiveCount::Two` defect in the gate is genuinely gone and the new query is real; but §3.3's new candidate-cell claim contradicts §4/§7's unchanged "the gate node calls `staged_candidates`", the width half of C1 is unaddressed, and the licence reading C1 explicitly demanded is still not stated. See NEW-1, NEW-2, NEW-3. |
| **C2** | **FIXED-BUT-NEW-ISSUE (non-blocking)** | The in-place rebind IS implementable in `pvs.rs` as structured, and it eliminates both defects (no unsound `depth_plies: 2` record, no double `self.nodes`). But §7's probe sentence is now affirmatively false under its own fix (NEW-4), and the store-0 rule has a table-eviction consequence §7 does not consider (NEW-5). |
| **C3** | **FIXED** | §3.4 now states D-267's assignment verbatim and correctly; checked against `docs/decisions.md:575`. |
| **C4** | **FIXED** | Trigger (b) fires on the whole of `Cover::Minimal`; the LAW-LEDGER reading is now the right way round; the `MinimalCover::One` test is demoted to §8's counters, where V2's counterexample still makes `.any(..)` the correct spelling. |
| **C5** | **FIXED** | §3.1/§3.2/§3.5/§4 all use `left = StonesLeft::from_state(state)` and `HitBudget::from(left)`. One residue: `from_state` returns `Option` and the design's prose never names the `None` arm (NEW-6, minor). |
| **C6** | **FIXED** | §3.1 scopes trigger (a) to `CandidatePolicy::Staged` against the same `self.policy` match, cites `position.rs:48,67,187–194` accurately, and states the staged-vs-radius single-variable consequence. |

---

## NEW FINDINGS

### NEW-1 (BLOCKING). §3.3 and §5 say the extension's cells are the trigger's own query output; §3, §4 and §7 say they are `staged_candidates`'s. Both cannot hold, and the difference is C1's own quantity.

**The four sentences, verbatim.**

- §3 (unchanged): *"Every trigger below is realized by calling the EXISTING
  `staged_candidates` protocol (`crates/pistol-search/src/staged.rs:168–207`) at
  the horizon — no new `pistol-solver` query is added."*
- §3.3 (changed, C1's fix): *"Query: `threats.cells_raising_to_hot(us,
  NearHot::Three, &mut cells)` … Non-empty → trigger fires; **candidate cells =
  the query's own output, `us` only**."*
- §4 (unchanged): *"Every qsearch node — **the gate node** and both plies of every
  granted turn — calls the existing `staged_candidates` … exactly as
  `pvs::visit`'s `depth_plies > 0` branch already does (`pvs.rs:260–264`)."*
- §7 (changed, C2's fix): the gate node *"falls through into the REST of this SAME
  `visit` invocation: the same TT probe/store code path the `depth_plies > 0`
  branch already runs."* The rest of that invocation IS `pvs.rs:245–292`, whose
  `Staged` arm calls `staged_candidates` and takes `set.cells` (`:263`, `:290`).

In revision 1 these agreed, and that is why they were never in tension: revision
1's trigger queries were literally the two staged rows' own cell sources
(`Cover::cells()` = `filtered()`, `tier_t_union` = `batched()`'s set), so "the
trigger's cells" and "`staged_candidates`'s cells" were the same object. C1's fix
broke that identity for trigger (c) and nothing downstream was updated.

**Verified in code.** On the row trigger (c) is reachable from
(`Cover::NothingToBlock`, §3.2's own disposal), `staged_candidates` takes the
BATCHED arm (`staged.rs:193–196`) → `batched()` → `tier_t_union` (`:275`, `:294–308`),
which unions `live_cells_at_count(Two) ∪ live_cells_at_count(Three) ∪ threat_cells`
for BOTH sides at the shipped thresholds (`:321–334`).
`cells_raising_to_hot(us, NearHot::Three)` is `live_cells_at_count(us, Three)`
(`query.rs:187–192`, `:206–208`, D-352's recorded coincidence) — a **strict subset**
of one of that union's six terms. The two are not the same set and the gap is the
whole of C1's magnitude.

**Why it is blocking rather than editorial.** MEASURED, `U3_tier_t.md` §6.2 (the
census `wp15b_census.rs` renders; corpus roots / r2 draw / r8 / playouts):
option C Tier T = **23.29 / 31.50 / 30.26 / 48.73** cells; the trigger-(c) query's
own set is the empties of the mover's live-3 windows, **3 cells per window** by
construction, so ~3–6 cells when non-empty (ESTIMATED, from `DEF-WINDOW`'s
arithmetic). Under §4/§7's reading the granted turn is searched 23–49 wide per
ply; under §3.3/§5's it is ~3–6 wide on the first ply. §9's registered instrument
is *nodes-to-same-depth inflation*, bracket `<= 2.0x`, **ABORT if `> 3.0x`**, and
rule 5 forbids moving those numbers. A design that does not determine which of two
readings — differing by ~10x in exactly that quantity — IMPL is to build cannot be
measured against them. This is the reason C2 was blocking in revision 1, applied to
the same document's other half.

**A second, independent corollary of the same gap:** §5's revision-2 cell-source
list is now false under §4/§7's reading. §5 says every gate-trigger cell *"comes
from `Cover::cells()` … or `cells_raising_to_hot(us, NearHot::Three, ...)`"*; under
§4 the BATCHED gate node's cells are `tier_t_union`, which is neither. §5's
*conclusion* (no `within_radius` cell reaches a gate decision) still survives —
`tier_t_union` contains no radius cell, and V4's exhaustive call-site argument is
untouched — but the enumeration §5 offers as its proof no longer matches what the
document elsewhere specifies.

### NEW-2 (BLOCKING, and the substance of C1's unfixed half). Under EITHER reading, the granted turn's SECOND ply is at Tier-T width, so §3.2's and §3.3's cost arguments both understate the extension by an order of magnitude — and §9's ABORT is again knowably at risk before the bench.

§3.2 argues the C4 fix is safe because *"a t ∈ {1,2} extension branches ~2 wide,
nowhere near the width that made trigger (c) blow its own bracket"*, citing
MEASURED 2.17–2.27 cells (census row `cover union when FILTERED` — I confirmed the
row and the figures at `U3_tier_t.md:168`). That number is the **first ply's**
width. §4 (unchanged, and explicitly in force for *"both plies of every granted
turn"*) sends the second ply through `staged_candidates` again, at `Phase::Second`,
on a position whose row is re-decided from scratch. After the mover has played a
cover cell, the opponent's hot windows are hit — that is what `LAW-HIT` says a
cover cell does — so `blocking_covers` at the second ply commonly answers
`NothingToBlock`, i.e. the BATCHED row, i.e. `tier_t_union`: MEASURED 23.29–48.73
cells. The same holds for trigger (c)'s granted turn under §3.3's own narrow
reading.

So the cheapest honest accounting of one granted turn is `1 + n₂` nodes with
maximal intra-turn pruning and `n₁ · n₂` without, where `n₂` is 23–49 —
i.e. **≥ ~24–50 extra nodes per fired horizon**, not ~2 (ESTIMATED, from MEASURED
widths). Firing rates, MEASURED from the same census: trigger (b) = the FILTERED
row = **25.0 / 18.4 / 13.7 / 3.1 %**; trigger (c) is reachable on the BATCHED row
(70.8 / 61.5 / 65.5 / 92.5 %) and fires when the mover owns ≥1 live-3 window, whose
mean count per node is **0.75 / 1.78 / 1.61 / 1.71 windows** (census rows `live-3
own`, `U3_tier_t.md:165`). Even taking trigger (b) alone at its lowest measured
rate, and even assuming leaves are only ~half the tree, `1 + 0.031 · 0.5 · 24 ≈
1.4x`; at the corpus-root rate it is `1 + 0.25 · 0.5 · 24 ≈ 4.0x`, past the
registered ABORT. Trigger (c) is additive on top of that.

I am not claiming a number here; I am claiming that the document's own two cost
arguments (§3.2's "~2 wide", §3.3's "nowhere near the width that blew the bracket")
are computed against the first ply only, while §4 specifies two, and that the
correction is available from the census the document already cites. Revision 1's
review ruled that *"A design whose central trigger is known in advance to blow its
own registered bracket is a design defect, not a measurement"*, and revision 2
quotes that ruling approvingly in §3.3. It applies again, at reduced magnitude, to
revision 2.

**Related, and it is why the deferral does not carry:** §3.3 defers the inflation
question to Phase 2 on the stated ground that *"This document does not have the
census's live-3-only cell count broken out (only the live-2 row is shown in
`U3_tier_t.md` §6.2's table as reproduced in the review)"*. §6.2's table is not the
review's excerpt of it: it carries `live-3 own` and `live-3 opponent` rows at
`U3_tier_t.md:165–166`, immediately below the live-2 rows the same paragraph cites.
Those rows count WINDOWS, not cells (`wp15b_census.rs:415`,
`live_windows_at_count(us, LiveCount::Three).len()`), so the document's sentence is
defensible on the word "cell" and false on the word "shown" — but the window count
IS the firing-rate half of the question being deferred, the cell count follows from
it within a factor of three, and the census harness is committed
(`crates/pistol-solver/tests/wp15b_census.rs`, D-287) and rerunnable in seconds.
CLAIMED-UNAVAILABLE where the repo has it, on the quantity C1 turned on, is a
finding under CLAUDE.md's "an estimate that could have been measured in seconds is
a finding".

### NEW-3 (blocking as part of C1). C1 required the document to state which reading of `PROTO-NODE` step 5's `t ≥ 2` it adopts. It still does not, and under the calculus's only meaning of `t` neither revised trigger is inside the quoted licence.

C1's remediation paragraph: *"Whatever is chosen, the document must state which
reading of `PROTO-NODE` step 5's `t ≥ 2` it adopts and why, because §1 makes that
sentence the design's licence."* §1 is unchanged and still quotes *"Threat-only,
zone-bounded (Tier F + Tier T with t ≥ 2), never full-width"* as **"the registered
cure"**. §3.3 uses the qualifier only rhetorically — *"on exactly the plan-free
positions `PROTO-NODE` step 5's own qualifier … excludes"* — and never adopts a
reading.

Under DEF-T (`threat_calculus_v1.md:30`), the only meaning `t` carries anywhere in
the calculus, revision 2's trigger set is *broader* than the licence in both arms:
trigger (b) now deliberately fires at `t = 1` (the C4 fix, which I agree with on its
own merits), and trigger (c) fires on a live-3 window whose activation produces a
single hot window — `t = 1`, not `t ≥ 2` (`PAT-C4`, `:105`). The design may well be
right to exceed the sentence; LAW-FORCE and LAW-RIPOSTE are better authorities for
this gate than a parenthesis in step 5. But §1 nominates that parenthesis as the
licence, C1 asked for the reading to be stated, and it is not — so §1 and §3.2/§3.3
now disagree about what authorises the trigger set, with §3.3 citing the parenthesis
against revision 1 while itself standing outside it.

### NEW-4 (non-blocking, but §7 states a falsehood). The C2 fix makes §7's probe sentence untrue, and §7 was edited to assert it more strongly rather than to correct it.

§7, after the fix: *"Probing (reading) is unrestricted: a quiescence node probing
the table is content with ANY stored record regardless of its depth (its own
requirement is `>= 0`, trivially satisfied — **true at every quiescence-regime node
including the just-rebound gate, since its probe also runs after the rebind**)."*

The parenthetical derives the opposite of what it states. The probe test is
`record.depth_plies >= depth_plies` (`pvs.rs:229`) against the node's OWN local. A
gate node that has rebound `depth_plies := 2` and then reaches `:226` requires
`>= 2`; the second ply of a granted turn requires `>= 1`. No node in the
quiescence regime ever probes at `depth_plies == 0`, because a gate node either
stands pat and returns before the probe or rebinds and probes at 2. Revision 1's
review flagged exactly this as N1 (*"true of a gate node that has NOT raised its
local `depth_plies`, and false the instant it has"*); revision 2 chose the reading
where it has, and kept the sentence.

Direction is conservative (fewer cutoffs, never a wrong one), so this is not
unsoundness — but two things follow that §7 gets wrong:

1. Combined with the store-0 rule, **no quiescence record is ever usable by any
   quiescence probe** (stored 0, required ≥1). §7's *"a full-width
   `Exact`/appropriate-bound entry already present is freely reused as a cutoff
   inside qsearch"* holds only for full-width records at depth ≥ 2, not "freely",
   and revision 2's own N1/N2 paragraph asserts the quiescence-to-quiescence
   direction is live ("two quiescence-regime gate nodes at the same table key …
   share one `Record`") when the depth test makes that sharing inert as a cutoff.
2. An IMPL that implements the sentence literally — relaxing the depth test for
   `q_budget.is_some()` nodes because "probing is unrestricted" — would let a
   depth-0 quiescence bound cut off a 2-ply quiescence search. §7 should say
   "no change to the probe" rather than give a false reason for making none.

### NEW-5 (concern, new with C2's fix). The depth-0 store EVICTS the full-width record it is stored over; §7 argues only about cutoffs.

`Table::store` overwrites unconditionally, and `victim` returns *"the position's own
entry if it already has one"* (`tt/mod.rs:158–191`) before any depth ranking. So a
rebound gate node writing `depth_plies: 0` at key K replaces an existing full-width
record at K of any depth. Never unsound — a lost entry only costs information — but
it is a systematic quality regression at exactly the transposed keys the main search
revisits, and it is a plausible driver of the ttd column §9 registers as PRIMARY.
The standard remedies (do not store from qsearch; or store only when the slot holds
nothing deeper) are one sentence each and neither is in the document.

### NEW-6 (minor). `StonesLeft::from_state` returns `Option`, and the design's `left` is used as if it were a `StonesLeft`.

`query.rs:51–58` returns `Option<StonesLeft>` (`None` on a decided position).
§3.1/§3.2/§3.5/§4 all write `left = StonesLeft::from_state(state)` and then
`can_win_this_turn(us, left)` / `HitBudget::from(left)`, which do not typecheck
against an `Option`. `staged_candidates` handles this with a named panic
(`OVERLOAD_ON_A_DECIDED_POSITION`, `staged.rs:178–183`) and the gate node should
mirror it; the case is unreachable through `visit` (a decided child is scored by
`PlyOutcome::Win` and never recursed into). Inherited from C1's own recommended
wording, so not a regression — but rule 3 wants the arm named rather than elided.

### NEW-7 (minor, and it argues in the document's favour). §3.5's "safety net included if Tier T is itself empty" contradicts §4 and §5 — on a case that is provably unreachable.

§3.5's new `is_pv` + `Impossible` + budget-remaining path says generation *"falls
through to the SAME generation `batched()` already computes for this row (real Tier
T, **safety net included if Tier T is itself empty**)"*. That is a `Phase::First`
gate decision, so §4's suppression rule (*"stand pat, never the radius ball"*) and
§5's *"`within_radius` cells never reach a qsearch GATE decision"* both say the
opposite.

The contradiction is verbal only: `Cover::Impossible` requires
`hot_windows(us.opponent())` non-empty (`cover.rs:201–208`), and `tier_t_side`
always unions `threat_cells(side)` — the empties of exactly those hot windows
(`staged.rs:330`) — so Tier T is non-empty whenever `Impossible` holds, and the
safety net is unreachable on the BATCHED-lost row. (The same argument makes it
unreachable on trigger (c)'s row: the trigger fires only when the mover owns a
live-3 window, whose empties are in `tier_t_side(us)`.) §5's zone claim therefore
survives in fact. But the design asserts a reachable-sounding exception to the one
rule §5's structural proof rests on, and IMPL would implement dead code for it.

---

## Spot-checks of revision 1's VERIFIED items

- **V3 (§4's `Phase::Second` correction) — still correct and genuinely unchanged.**
  `git diff 9fa27c8 b1ba746` touches no line of §4. Re-checked the code path:
  suppressing unconditionally leaves `set.cells` empty at `Phase::Second`,
  `pvs.rs:285–288` calls `no_candidates_at_a_turn_boundary()`, and `:479–486` is a
  release-active `assert!` on `Phase::First` — panic with `NO_CANDIDATES_MID_TURN`,
  D-104. Unchanged and still right. Note NEW-7: §3.5's new text carves an exception
  to §4 without amending §4.
- **V4 (§5's zone claim) — conclusion survives, stated proof does not.** See NEW-1's
  second half. `within_radius` is still `pub(crate)` with exactly two call sites
  (`candidates.rs:47–48` under the `Radius` arm, `staged.rs:283` the safety net,
  which sets `used_quiet_safety_net` on the next line), so no radius cell reaches a
  gate decision; but §5's revision-2 enumeration of the cell sources no longer
  matches §4/§7's mechanism.
- **V1, V2, V5, V6, V7 — not re-derived; nothing in the diff lands on them.** Spot
  checks only: §3.2 still spells the `MinimalCover::One` test `.any(..)` (V2's
  counterexample still applies where §8 now uses it); §3.4's LAW-FORCE substitution
  (V5) is retained verbatim and is right; §3.3's Tier-F-empty argument still
  reproduces `batched()`'s own doc (`staged.rs:261–266`) correctly, now with `left`
  in place of the literal `Two`, which is strictly more accurate; §9's registered
  block is byte-unchanged (V6 holds — nothing here asks the numbers to move).
- **Line and ID citations checked, all accurate:** `query.rs:51–58`, `:101–107`,
  `:187–192`, `:206–208`, `:231`; `cover.rs:108–116`, `:201`; `staged.rs:168–207`,
  `:186–189`, `:196–206`, `:251–257`, `:262–266`, `:294–334`, `:86–89`;
  `pvs.rs:53–55`, `:193–194`, `:220–237`, `:229`, `:260–264`, `:277–279`,
  `:311–321`, `:359–377`, `:387–425`, `:429–431`, `:441–467`, `:479–486`;
  `position.rs:48,67,187–194`; `decisions.md:575`; `threat_calculus_v1.md:29`,
  `:49–53`, `:55–59`, `:68–72`, `:74–77`, `:93`, `:108`, `:137–141`. §6's config
  list is complete against `configs/*.toml` carrying `kind = "staged"`.

## Answers to the four questions revision 2's §10 asks the re-review

1. **Do the fixed triggers over/under-fire, and can (b) and (c) double-extend?**
   No double-extension: the two triggers key on disjoint `Cover` arms
   (`NothingToBlock` vs `Minimal`), so at most one fires per gate node, and §8's
   mutual-exclusivity claim is right for the right reason. `cells_raising_to_hot`
   cannot return a cell `Position::place` refuses — the empties of a window holding
   three own stones lie within hex-distance 5 of a stone and are empty by
   construction, and the shipped Tier T already feeds the same cells to `place`
   without `CANDIDATE_ILLEGAL` firing. Over-firing is now a question of rate rather
   than of kind (NEW-2).
2. **Does closing the t=2 gap discharge what D-267 assigns WP-1.6?** For
   `LAW-RIPOSTE`, yes as far as an alpha-beta search can: the flip-initiative cases
   are `t ≥ 3` (§3.5's overload return), `t ∈ {1,2}` (trigger (b) at the child), and
   the budget-exhausted admission of ignorance, which is not the unsound-proof
   failure mode the law warns provers about. `LAW-LEDGER` is a different matter and
   the document should notice: D-267 assigns WP-1.6 **both** IDs, and after C4 the
   ledger appears only as a refuted argument in §3.2's changelog prose. Nothing in
   the shipped trigger set expresses "the defender's turn is worth 2 − t free
   stones". Not blocking — a WP may leave an assigned ID for its successor — but it
   should be recorded as debt with an owner rather than silently dropped.
3. **Is §7's in-place rebind implementable as stated?** Yes. `visit`'s signature
   takes `depth_plies: u32` by value, so `mut depth_plies` and `mut q_budget` are
   ordinary Rust; the `if depth_plies == 0 { … return … }` block at `:199–218`
   becomes a block that either returns or falls out, and `:220` onward runs with the
   rebound locals. IMPL must still invent four things the document does not state,
   none of them hard but all of them decisions: (a) `is_pv` (`beta - alpha > 1`) is
   computed at `:225`, *after* the gate block, and §3.5's gate decision needs it, so
   it moves up; (b) the store expression becomes `if q_budget.is_some() { 0 } else
   { depth_plies }`; (c) the gate's own `can_win_this_turn`/`blocking_covers` calls
   are then repeated by `staged_candidates`'s steps 1–2 a few lines later — a
   doubled threat-query cost at exactly §9's registered HOTSPOT, unmentioned; (d)
   the counters' `covers` value (§8's t1/t2 split) is only available from the gate's
   own `blocking_covers` call, which fixes (c) as real duplicated work rather than
   an avoidable one. The node-count and TT-soundness claims both hold.
4. **§3.5's ruling, re-derived independently rather than trusted.** Sound, and I
   reach it by a different route than revision 1's reviewer. At an exhausted gate
   node the two candidate answers are `-mate_in(turns_from_root + 2)` and
   `self.position.value()`; `LAW-OVERLOAD` (`:55–59`) licenses the former from two
   premises, "attacker `t ≥ 3`" (which `Cover::Impossible` at `HitBudget::from(left)`
   establishes — and `left` is `Two` at every gate node, since D-111 puts the
   horizon at `Phase::First` and turn 1 is never a horizon under `plies_for`) and
   "defender cannot win this turn" (trigger (a)'s `None`, evaluated at the same
   node); `is_pv` is not among them. The PV consequence is also fine independently:
   `turns_from_plies` panics only on an illegal ply or *"a turn half played"*
   (`pv.rs:76–79`), and a gate node sits at a turn boundary, so a line ending there
   replays whole. The `is_pv` exception it drops exists to make generation produce a
   provable line (`staged.rs:85–89`), and at an exhausted gate no generation happens
   on either branch — so the exception's premise is absent, exactly as claimed. One
   thing the ruling does not say and should: it fires at `q_depth_turns == 0` too,
   so the "quiescence disabled" seat §6 offers Phase 2's differential oracle already
   returns mate scores where today's engine returns a static value. §9's N4
   paragraph names that consequence for trigger (a); the ruling adds a second source
   of it and does not.
5. **Is N4 adequately flagged?** Yes. §9 names the ambiguity, names both candidate
   right-hand seats, says what each measures, and puts the choice in the bench's own
   commit before the run. That is the right disposition for a design document and it
   does not touch D-388's registered numbers.

---

## What would close this

Small, and all inside §§3.3/4/5 plus two sentences in §7:

1. **Decide the extension's move set and say it once, in §4.** Either (i) the gate
   node and both plies use `staged_candidates` — then delete §3.3's "candidate cells
   = the query's own output", restore `tier_t_union` to §5's source list, and state
   that trigger (c) is a *gate condition only*, whose fix reduces the firing rate and
   not the width; or (ii) the gate node uses the trigger's own cells — then §3, §4
   and §5 must say so, and §4 must specify how those cells are ordered, how the
   table move is promoted into them, what `StagedSet::forced` is, and which
   `StagedRow` the counters record, none of which follows from `staged_candidates`
   any more.
2. **Cost the granted TURN, not its first ply**, using the census rows already in
   the repo (`U3_tier_t.md:161–184`), and say whether the result sits inside
   D-388's registered `2.0x` / ABORT `3.0x` or whether the design expects the ABORT
   to fire. Either answer is admissible; the absence of one is not, because rule 5
   forbids moving the numbers afterwards.
3. **State the `t ≥ 2` reading** §1 makes the licence, or amend §1 to cite
   LAW-FORCE/LAW-RIPOSTE as the trigger set's authority instead of a parenthesis it
   knowingly exceeds.
4. **Correct §7's probe sentence** to "the probe test is unchanged and, after the
   rebind, requires `>= 2` (or `>= 1` at the second ply) — which is conservative:
   only full-width records of sufficient depth cut off inside qsearch, and no
   quiescence record ever does," and add one sentence on the eviction consequence
   (NEW-5).
5. **Reconcile §3.5's safety-net parenthetical with §4/§5** — the simplest fix is to
   delete it and record the unreachability argument (NEW-7) instead.

*Report written by the scoped REVIEW-design re-review subagent against `b1ba746`.
Left uncommitted for the orchestrating session.*
