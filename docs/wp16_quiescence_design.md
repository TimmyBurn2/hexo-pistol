# WP-1.6 — Threat-only zone-bounded quiescence: design

**Revision 3.** Authored from two architect rulings resolving `D-389`'s two
open questions (extension width; TT store rule), supplied verbatim with the
resume dispatch `[GROUNDWORK] WP-1.6 resume: design revision 3 from
architect rulings`. Revisions 1 (`9fa27c8`) and 2 (`b1ba746`) each failed
fresh-context REVIEW-design (`docs/experiments/wp16_design_REVIEW.md`,
`wp16_design_REVIEW_rev2.md`); `D-389` records the second FAIL and what
would unstick it. This is not a patch on revision 2 — the rulings replace
revision 2's core mechanism (reusing `staged_candidates` at a rebound depth)
with a dedicated generator and a dedicated TT rule, so this document is
written fresh from them rather than diffed. Where a section carries forward
unchanged in substance (§1, §2, most of §5, §6, most of §8), it says so and
does not restate settled argument at length.

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
`left' = StonesLeft::One`), recompute BOTH queries fresh against the new
position (the incrementally-updated `ThreatState` already reflects the
ply-1 stone, `position.rs:75,143`):

- `Cover2 = threats.blocking_covers(us, HitBudget::from(left'))` —
  `HitBudget::One`, since one stone remains. `NothingToBlock` → nothing left
  to hit. `Minimal(covers)` → **candidate cells include `Cover2::cells()`**
  (the "remaining live opponent plans" — plans the ply-1 stone did not fully
  resolve: reachable when ply-1 played a `MinimalCover::Two` member rather
  than the size-1 hit, or when a wholly different plan already existed
  alongside the t≤1 family the gate tested). `Impossible` → **"empty hitting
  set = losing band, no search"** applies again, symmetrically: return
  `-mate_in(turns_from_root + 2)`, no further generation for this branch —
  this is a per-BRANCH pruning decision inside the search tree (the branch
  that chose this particular ply-1 stone is scored and abandoned), not a
  claim that the real game ever plays fewer than two stones this turn
  (rule 3; other ply-1 branches, including the one that played the actual
  size-1 hit `MinimalCover::One` names, are unaffected and reach
  `NothingToBlock`/`Minimal` normally).
- `threats.cells_raising_to_hot(us, NearHot::Three, &mut cells)`, recomputed
  at the ply-2 position — "mover plan-making cells" — union'd in
  unconditionally (available whether or not `Cover2` found anything left to
  hit; `LAW-LEDGER`'s free stone, §3.7, is what this cell set spends).

Ply-2 candidates = `Cover2::cells() ∪ cells_raising_to_hot(us, NearHot::Three)`
at the ply-2 position (deduplicated), unless `Cover2::Impossible` fired the
early return above. **No quiet stage, no `within_radius`, no
`staged_candidates` call anywhere in this path** — every cell in both plies
traces to one of the three queries named in §3.1–§3.3/§3.5, per Ruling 1.

**Offensive pairs (Ruling 1's third clause).** When the gate fired trigger
(c) rather than (b) (§3.3, `Cover::NothingToBlock` at the gate), ply-1's
cells are `cells_raising_to_hot(us, NearHot::Three)` and ply-2 uses the SAME
uniform rule above — recomputing `Cover2` (in case the ply-1 offensive
stone incidentally landed inside an opponent window and changed something,
or a plan appeared from an unrelated cause) union'd with the offense query
recomputed. One ply-2 rule, not two.

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
the disable flag**: `quiescence()`'s gate still runs §3.1/§3.4's free checks
(they cost nothing and are not extensions), but §3.2/§3.3 never grant a turn.
This is the "quiescence disabled" state the Phase-2 differential oracle needs.

**Configs that must gain the field in the landing commit** (unchanged list
from revision 2, re-verified against the tree at this revision):
`configs/play_staged_v0.toml`, `configs/tactical_staged_v0.toml`,
`configs/gate_staged_v0.toml`, `configs/instrument_v0.toml`,
`configs/instrument_staged_v0.toml`,
`configs/arena_wp15b_staged_vs_r2.toml`,
`configs/arena_wp15b_staged_vs_r2_confirm.toml`,
`configs/arena_wp15b_dryrun.toml`.

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
2. **Add one bit, `from_quiescence: bool`, to `Entry`**, WITHOUT reducing
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
   already had, not a new one this design adds. **REVIEW-design (Phase 1'):
   verify this byte-budget arithmetic independently before trusting it as
   the resolution** — it is asserted here, not yet compiled.
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
4. **Probe rule — deliberately the simplest available, per "no alternative
   reading":** a probe returning a record with `from_quiescence: true` is
   treated by a FULL-WIDTH caller (`q_budget == None` context) exactly as if
   `probe` had returned `None` — no cutoff, no move-ordering hint, full stop.
   `quiescence()` itself never probes the table at all — it only ever
   WRITES, once per node it visits, so there is no quiescence-to-quiescence
   reuse question to answer (the second revision's review, NEW-2/N2, raised
   exactly this as unaddressed; this revision addresses it by removing the
   read path entirely rather than reasoning about what it would be sound
   for). This is the one place this document deliberately gives up a
   possible optimization (a granted turn could in principle reuse a sibling
   quiescence bound) in exchange for a rule simple enough that "no
   alternative reading appears anywhere" is actually true of it.

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

## 10. Revision 3's provenance, and what Phase 1' (fresh reviewer) must attack

**Two consecutive FAILs preceded this revision** (`D-389`). This revision is
authored from architect rulings supplied with the resume dispatch, not
self-generated fixes — its job is faithful, precisely-cited EXPANSION of
those rulings, which is a narrower task than revision 2's open design work
was, but the citations, the TT resolution (§6), and the cost derivation (§9)
are this session's own work product and are exactly what a fresh reviewer
should attack hardest, having NOT reviewed revisions 1 or 2 (per the resume
dispatch: "NOT the phase-1 reviewer slot — two fails earn new eyes"):

1. **§3's move-set specification** — is it now stated exactly once, with §3
   through §5 mutually consistent (the defect class that sank revision 2)?
   Confirm no remaining path calls `staged_candidates` from inside
   quiescence, and that §3.5's ply-2 rule is unambiguous enough for IMPL to
   build without inventing a reading.
2. **§6's TT resolution** — this document found `entry.rs`'s `depth_fits`
   assert makes a literal `depth_plies: 0` store panic, a fact neither prior
   review caught. Verify this independently against `entry.rs` at HEAD, and
   verify the proposed fix (a `flags: u8` field inside the existing 24-byte
   budget, `GENERATIONS` untouched, store-side eviction protection, no-probe
   for `quiescence()` itself) is a faithful realization of Ruling 4's intent
   and not an unlicensed departure from it.
3. **§9's cost derivation** — check the arithmetic, check every census
   citation against `U3_tier_t.md` directly, and assess whether presenting
   an ESTIMATE that may exceed the registered bracket (rather than a figure
   safely inside it) is the right disposition under Ruling 3, or whether
   the derivation itself has an error that changes the conclusion.
4. **§3.2/§3.7** — confirm Ruling 2's t≤1-only gate condition, combined with
   §3.5's ply-2 handling of "remaining" t=2 remnants, actually discharges
   what D-267 assigns this WP for `LAW-RIPOSTE`/`LAW-LEDGER`, the way the
   second review's independent re-derivation found revision 2's broader gate
   did — or whether narrowing back to t≤1 reopens a gap that broader gate
   had closed.
5. **§4/§7** — re-verify the zone claim and the win-detection/determinism/PV
   claims structurally against HEAD, not only against this document's
   description of them.

PASS → proceed directly into the original dispatch's Phase 2 (IMPL) and run
it to closure or STOP as written there — no re-entry to Phase 0 needed. FAIL
→ STOP immediately, land the report, collect for the architect; per the
resume dispatch, there is no revision 4 inside this session.
