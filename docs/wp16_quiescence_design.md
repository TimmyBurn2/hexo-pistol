# WP-1.6 — Threat-only zone-bounded quiescence: design

Governing dispatch: `[GROUNDWORK] WP-1.6 threat-only quiescence: full cycle,
overnight`. Binding core: the dispatch's nine-point ARCHITECT DESIGN CORE. This
document expands it; it contradicts none of it, and where the dispatch's prose
cites a calculus ID loosely, §3.4 below states the precise correction and why
it changes nothing about the ruling's intent. Unblocked by D-386 / ROADMAP.md
"WP-1.6 is UNBLOCKED."

Every claim below cites a calculus ID (`docs/research/threat_calculus_v1.md`),
an ADR (`docs/decisions.md`), or a `file:line`. Reader: this is the expansion,
not a tutorial — read the calculus and `staged.rs`/`pvs.rs` first.

## 1. Problem (core §1)

`Eval::value` sums window-pattern features (`pistol-eval`); it cannot see a
forced win or a forced loss that starts one turn past the horizon. The prior
"experiment-2" failure broadened the CANDIDATE SET at the horizon (report
§B, "your naive one-primitive threat-extension probe failed because it
broadened the candidate set globally"). The registered cure is threat-only,
zone-bounded extension (report §B; calculus `PROTO-NODE` step 5: "Threat-only,
zone-bounded (Tier F + Tier T with t ≥ 2), never full-width").

## 2. TURNS invariant (core §2, D-111)

A quiescence node is a turn boundary; stand-pat is a static read at
`Phase::First` only, never mid-turn. This binds every design choice below: an
extension is granted or refused only when `depth_plies` would hit 0 at
`Phase::First`, and once granted it always completes as a whole turn (both
plies) before the next stand-pat/extend decision is made.

### 2.1 The gap this closes — VERIFIED, not assumed

Core §3a asks: "verify stage F already returns this [win-now]; if so, qsearch
never sees the case — state which holds." Verified by reading `pvs.rs:199–217`
(the `if depth_plies == 0` branch): it runs a `debug_assert!` on phase and then
`return self.position.value()` — no threat query, no candidate generation, no
call to `staged_candidates`. Stage F's win-now check
(`staged_candidates`'s step 1, `staged.rs:186–189`) executes only inside the
`depth_plies > 0` branch (`pvs.rs:260–291`), reached from a PARENT that still
had ply budget. **Stage F does NOT already cover the horizon**: a mover who
can complete six THIS TURN at a `depth_plies == 0` node is, today, scored by
the static evaluator instead of `mate_in(...)`. This is the gap trigger (a)
(§3.1 below) closes, not a redundant re-check — the first concrete finding of
this design and the reason `Eval::value`'s window sums cannot be trusted at
any leaf without at least this one free query first.

## 3. Triggers, in ThreatState's actual vocabulary

Every trigger below is realized by calling the EXISTING `staged_candidates`
protocol (`crates/pistol-search/src/staged.rs:168–207`) at the horizon —
no new `pistol-solver` query is added. Where core §3 cites `LAW-LEDGER` and
`LAW-RIPOSTE`, D-267 records that neither has "any counterpart anywhere in the
shipped surface" and that both are "WP-1.6's" — this WP's job is to express
their CONTENT in `ThreatState`'s existing calls, not to add new ones. §3.4
below is where that expansion happens precisely.

### 3.1 Trigger (a) — mover can win this turn

`threats.can_win_this_turn(us, StonesLeft::Two)` (`query.rs:231`, `PROTO-NODE`
step 1). `Some(witness)` → terminal: `mate_in(turns_from_root + 1)`, zero
extra nodes, no recursion — identical cost shape to `staged_candidates`'s
`WinNow` row (`staged.rs:186–189`) and to `pvs::visit`'s own
`PlyOutcome::Win` handling (`pvs.rs:311–321`). Always evaluated at every
horizon regardless of `q_budget` — it is not an extension, it is the missing
check §2.1 identified.

### 3.2 Trigger (b) — opponent holds a t ≤ 1 plan (`LAW-LEDGER`'s t=1 case)

Query `threats.blocking_covers(us, HitBudget::Two)` (`cover.rs:201`,
`LAW-FORCE`'s survival set per D-267). Classification, over `Cover`:

- `NothingToBlock` → t = 0, no opponent plan, no defensive obligation.
- `Minimal(covers)` where `covers.iter().any(|c| matches!(c,
  MinimalCover::One(_)))` → **t = 1**, trigger fires. A single cell exists
  that hits every opponent hot window on its own
  (`MinimalCover::One`'s own doc, `cover.rs:60–61`); `LAW-LEDGER`: "the
  defender's turn is worth 2 − t free stones" — at t=1 the mover banks
  exactly one free stone, the class `LAW-LEDGER` names as chainable ("t=1
  chains bank the defender one stone/turn").
- `Minimal(covers)` with no `MinimalCover::One` present → t = 2. **NOT this
  trigger.** At t=2 the whole turn is consumed hitting (2 − t = 0 free
  stones); LAW-LEDGER gives the mover no spare tempo to develop, and the
  position is a fully-determined forced double-block already reachable by
  the ordinary FILTERED row at the PARENT's own depth budget — it is not a
  horizon blind spot the way a chainable t=1 sequence is. Left out of scope
  by this design; not silently dropped (rule 3) — recorded here as the
  reasoned boundary, and it is the class WP-1.5c's Tier-Q/dominance work
  (ROADMAP.md, D-315) is the natural next place to revisit if measurement
  ever shows it matters.
- `Impossible` → t ≥ 3, `LAW-OVERLOAD`. Not trigger (b); handled by §3.5.

Candidate cells on trigger: `Cover::cells()` (the union over ALL inclusion-
minimal covers, `cover.rs:108–116`) — identical in shape to the FILTERED
row's own `filtered()` (`staged.rs:251–257`), not restricted to only the
`MinimalCover::One` cells. `LAW-FORCE` is the correct citation for "every
non-losing mover move hits all opponent plans," not `LAW-RIPOSTE` — see §3.4.

### 3.3 Trigger (c) — mover can create a t ≤ 2 plan this turn

Content: does the mover have ANY Tier-T-qualified offensive continuation.
Query: `tier_t_union(threats, us, params)` (`staged.rs:294–308`, made
`pub(crate)`, reused verbatim — no reimplementation). Non-empty → trigger
fires. Tier F for `us` is PROVABLY EMPTY here by the same argument
`batched()`'s own doc already gives (`staged.rs:262–266`): trigger (a) having
answered `None` forbids both a win-in-one-ply cell and a hot four-stone
window at `StonesLeft::Two`, which is exactly what `tier_f` would have
contributed. So `tier_t_union`'s output IS the candidate set on this trigger,
with no separate Tier-F step to run.

### 3.4 Correcting the dispatch's `LAW-RIPOSTE` citation

Core §4 cites `LAW-RIPOSTE` for "a defense must hit every unanswerable plan."
That sentence is `LAW-FORCE`'s content (§3.2 above), not `LAW-RIPOSTE`'s.
`LAW-RIPOSTE`'s actual content (`threat_calculus_v1.md:74–77`): "a forced
defensive stone can itself create a plan and flip initiative... any forcing-
line PROVER must check every forced reply for new plans; skipping the check
is unsound." That is a soundness obligation on PROVERS (df-pn/RZOP, Stage 3 /
WP-1.8 — D-267 lists `LAW-RIPOSTE` alongside `ZONE-R`/`LAW-DECOMP` as "Stage
3's" for the proof-engine sense of the term). Quiescence is not a prover: it
never claims a proof, so it owes no soundness obligation there. What it DOES
inherit, for free, is the SPIRIT of the check: after a hitting cell is played,
the child node re-runs the SAME trigger protocol (§3 in full) on the resulting
position — if the forced stone happened to create a new plan, trigger (c) (or
a fresh trigger (a)/(b)) fires at the child and the search extends again,
exactly the riposte check `LAW-RIPOSTE` asks a prover to make. The difference
from a prover: `q_depth_turns` caps how far this can run, and running out of
budget means "stand pat" (an admission of ignorance, sound for alpha-beta)
rather than "unsound proof" (which only a df-pn-style claim of certainty could
be). **REVIEW-design: confirm this recharacterization does not change any of
the dispatch's four load-bearing sentences in core §3–4** — it changes only
which calculus ID is cited for which sentence.

### 3.5 The `Impossible` / losing-band case (core §4's D-105 pointer)

`blocking_covers(us, HitBudget::Two) == Cover::Impossible` is `LAW-OVERLOAD`
(t ≥ 3): the mover cannot survive this turn. `staged_candidates` already
distinguishes this by PV-ness (`staged.rs:196–206`):

- `!is_pv` → `StagedRow::OverloadReturn`, zero-cost: return
  `-mate_in(turns_from_root + 2)` (`pvs.rs:277–279`) with no recursion. Free
  exactly like trigger (a) — runs regardless of `q_budget`.
- `is_pv` → `StagedRow::BatchedLost`: the PV must carry a provable line, so
  generation proceeds instead of returning early. **Open question for
  REVIEW-design**, not resolved by the dispatch: at the horizon with
  `q_budget == 0`, does an `is_pv` `BatchedLost` row override the cap (extend
  anyway to keep the PV honest) or does it stand pat like any other
  `q_budget`-exhausted node? RECOMMENDATION (default if REVIEW-design does
  not override): treat it like any other capped node — stand pat. Reasons:
  (1) a position already provably lost by `LAW-OVERLOAD` typically already
  scores badly under the uncapped pattern-table sum, so the marginal
  information from forcing the proof through is small; (2) granting
  `BatchedLost` a free pass at the horizon is a second, PV-conditioned
  extension mechanism the anti-pattern discipline (§1) did not license, and
  the dispatch's own bracket (§9 below) assumes a SINGLE, uniformly-capped
  extension path. `D-105` is cited by the dispatch only as the PRECEDENT for
  "a lost position's score is a live, still-open question" (D-105's
  `DECIDED_WINDOW_VALUE` flooring is an UNSTARTED Stage-1 arena experiment,
  not something this WP implements) — nothing here depends on that
  experiment's outcome; this design never floors an eval, it only chooses
  between a mate-band shortcut and a capped stand-pat.

## 4. Move set inside qsearch

**No new generator.** Every qsearch node — the gate node and both plies of
every granted turn — calls the existing `staged_candidates` with
`left = StonesLeft::from_state(state)` exactly as `pvs::visit`'s
`depth_plies > 0` branch already does (`pvs.rs:260–264`). The one deviation:

**Quiet-safety-net suppression, at `Phase::First` gate decisions only.** If
`staged_candidates` returns `StagedRow::Batched`/`BatchedLost` with
`StagedSet::used_quiet_safety_net == true` (`staged.rs:118–122`, meaning Tier
T was empty and the generator fell back to `within_radius`,
`staged.rs:276–285`) AT A GATE NODE (`Phase::First`, deciding whether to
extend), this is treated as "no offensive trigger" — stand pat, never the
radius ball. This is what "no quiet stage" (core §4) means operationally and
is exactly the anti-pattern §1 excludes: standing pat here costs nothing,
since D-111 already permits a static read at any turn boundary.

**Suppression does NOT apply at `Phase::Second` inside an already-granted
turn.** A turn, once granted, must complete — `Position::place`'s legality and
the `NO_CANDIDATES_MID_TURN` invariant (`pvs.rs:53–55`, D-104) mean the second
stone is not optional. If the second ply's `staged_candidates` call comes back
`Batched` with the safety net used, qsearch uses those cells (radius ball
included) exactly as the main search would — completing the committed turn
honestly, never inventing a static answer mid-turn. **This is a correction to
core §4's literal "no quiet stage," stated because the alternative (suppress
unconditionally) hits `NO_CANDIDATES_MID_TURN`'s panic path
(`pvs.rs:479–486`) on the first position where Tier T empties out between a
turn's two stones — RED-TEAM (Phase 3): construct this fixture explicitly**
(a granted turn whose first stone is a `MinimalCover` hit and whose second
stone finds Tier T empty).

## 5. Zones — window-support bound, never radius

By construction, every cell qsearch's gate-trigger candidate sets can contain
comes from `Cover::cells()` (empties of the opponent's HOT windows) or
`tier_t_union` (empties of LiveCount::Two/Three and hot windows, both sides) —
never from `within_radius` (`crate::candidates::within_radius`,
`candidates.rs`), which is the ONLY radius-based cell source in
`pistol-search`. §4's suppression rule guarantees `within_radius` cells never
reach a qsearch GATE decision. This is the "union of live windows' support"
bound core §5 asks for, satisfied structurally rather than by a separate
runtime check — REVIEW-design should confirm no path threads
`within_radius`/`candidate_cells` into a gate node.

This is NOT `ZONE-R` (`threat_calculus_v1.md:137–141`, RZOP's combinatorial
relevance zones for FINITE PROOFS, order ≤ 3) — `ZONE-R` is Stage 3 / WP-1.8's
(D-267). Quiescence's window-support bound is a cheaper, weaker relative that
shares `ZONE-R`'s motivating principle (bound the search combinatorially, not
by an arbitrary radius) without `ZONE-R`'s proof-engine machinery. Do not cite
`ZONE-R` as implemented by this WP.

## 6. Cap: `q_depth_turns`

New field on the `staged` variant of `[search.candidate_policy]`
(`crates/pistol-engine/src/config.rs:161–182`, alongside `quiet_radius`,
`quiet_top_k`, `widen_schedule`, `tier_t_own_count`, `tier_t_opponent_count`)
and on `pistol_search::params::StagedParams` (`params.rs:58–70`) —
schema-home per hard rule 1, no code-side default. `u32`, validated range
`0..=8` in `pistol-engine`'s `validate.rs` (mirroring `MAX_CANDIDATE_RADIUS`'s
existing validated-range precedent for `quiet_radius`/`radius`).
**`q_depth_turns == 0` IS the disable flag** — the gate check at §3 still
runs (trigger (a)/OverloadReturn are always free, §3.1/§3.5), but no
extension is ever granted, which is exactly the "quiescence disabled" state
Phase 2's differential-oracle comparison needs (core, Phase 2: "the oracle
comparison runs with quiescence disabled-flag identical"). No separate
boolean field — a second flag alongside a numeric cap would be two ways to say
the same thing (rule 3's closed-enum discipline, and rule 9's
no-redundant-state spirit). The SHIPPED value is a closed enum of tried
values, decided by SPRT only (core §6) — this document fixes no number.

## 7. Correctness

**Win detection.** Rule 2: unchanged, pistol-core's alone. Trigger (a) reads
`ThreatState`, which is itself derived from and kept in step with
`GameState`/`Board` (`position.rs` doc, `D-41` as amended by WP-1.5b) — it
never substitutes for `PlyOutcome::Win`, which is still what actually ends a
line (`pvs.rs:311–321`, unchanged by this design).

**Scores in turns.** `mate_in`/`to_table`/`from_table` (`score.rs`) are used
unchanged; a qsearch extension's mate distances are computed exactly as the
main tree's (`turns_from_root`, `pvs.rs:429–431`), since qsearch never leaves
the `visit`/`child` recursion (§7.1).

**Recursion shape — one function, one new parameter.** No parallel
`quiescence()` function. `Run::visit` and `Run::child` each gain one
parameter, `q_budget: Option<u32>`:

- `None` everywhere in the main tree today — behavior byte-for-byte
  unchanged from the current code whenever `q_budget` stays `None`, which is
  every call site until the first horizon gate grants an extension.
- At a `depth_plies == 0`, `Phase::First` gate node with `q_budget == None`:
  run §3's checks. Trigger (a)/`OverloadReturn` (§3.1/§3.5) return directly,
  no recursion. Trigger (b) or (c) firing (with `used_quiet_safety_net` false
  per §4) and `params.q_depth_turns > 0`: recurse with
  `depth_plies = 2, q_budget = Some(params.q_depth_turns - 1)`. Otherwise:
  today's unconditional stand-pat, unchanged
  (`return self.position.value()`).
- Once `q_budget = Some(k)`: every node — both plies of the granted turn, and
  every subsequent horizon this turn's completion reaches — carries
  `Some(_)` forward unchanged UNTIL the next `depth_plies == 0`,
  `Phase::First` gate, where the same decision re-runs with `k` in place of
  `params.q_depth_turns` (extend again only if `k > 0`).
- `child()` (`pvs.rs:387–425`) threads `q_budget` through its two recursive
  calls (`full`/the null-window scan) unchanged in value — it is orthogonal
  to the same-side/opponent window logic `child` already owns.

**TT sharing at depth 0 (core §7).** Every TT `Record` written from a node
where `q_budget.is_some()` stores `depth_plies: 0` in the record
(`tt/entry.rs`'s `Record.depth_plies` field), REGARDLESS of the local
extended-turn ply countdown (2 or 1) that node is using for its OWN recursion
control. This is the one place `depth_plies` (recursion control) and the
TT-stored depth (a claim about search width already done) must be kept
DELIBERATELY DIFFERENT quantities: a full-width prober's cutoff test
(`record.depth_plies >= depth_plies`, `pvs.rs:229`) must never accept a
quiescence-narrowed bound as satisfying a real `depth_plies >= 1` requirement
— storing `0` guarantees this by construction, the same reasoning the main
search already applies to its own depth-gated cutoffs. Probing (reading) is
unrestricted: a quiescence node probing the table is content with ANY stored
record regardless of its depth (its own requirement is `>= 0`, trivially
satisfied), so a full-width `Exact`/appropriate-bound entry already present is
freely reused as a cutoff inside qsearch — this is the actual content of
"TT shared... with the existing key," and it needs no key change (phase bit +
side to move are already in `GameState::key()`, unchanged by this design).

**Determinism law.** No new nondeterminism source: `staged_candidates`,
`blocking_covers`, `tier_t_union` are all already total/deterministic/sorted
(`query.rs`'s module doc: "Every one is total, deterministic... None consults
a clock, a hasher's iteration order"). `q_budget` is plain recursion state,
not a wall-clock or thread-order read. The existing `should_stop`/
`order_deadline` machinery (`pvs.rs:441–467`) is untouched; a qsearch node
checks the stop condition on entry exactly as any other `visit` call does
(the `self.nodes += 1; ... if self.should_stop()` prologue, `pvs.rs:193–198`,
is shared code, not duplicated).

## 8. Counters

Extend `StageCounters` (`info.rs:39–65`) with fields written only from
`q_budget.is_some()` paths, following the file's own "whole-search totals,
written from the same point `nodes` is" convention (`info.rs:29–33`):

- `qnodes: u64` — nodes visited with `q_budget.is_some()` (every `visit` call
  in that regime, both plies).
- `q_win_now: u64` — trigger (a) fired at a gate node.
- `q_overload_return: u64` — §3.5's zero-cost shortcut fired at a gate node.
- `q_extend_defense: u64` — trigger (b) granted an extension.
- `q_extend_offense: u64` — trigger (c) granted an extension (mutually
  exclusive with defense: §3.2/§3.3 checked in that order, defense first,
  per `LAW-FORCE`'s "every non-losing move" precedence over pure offense).
- `q_stand_pat_no_trigger: u64` — gate reached, no trigger fired (including
  the t=2 and safety-net-suppressed cases, §3.2/§4).
- `q_stand_pat_cap: u64` — a trigger fired but `q_budget` was already
  exhausted.

`StagedRow`/`record()` (`staged.rs`/`info.rs:69–83`) are unchanged — a
qsearch node's OWN `staged_candidates` calls still feed the existing
`win_now`/`filtered`/`batched`/... counters exactly as a main-tree node's
would (§4: same function, same rows), so `StageCounters` after this WP
answers both "how did the whole search's candidate rows split" (existing
fields, now additionally covering qsearch-regime nodes) and "how much of that
was quiescence, and why" (the seven new fields above).

## 9. Rule-5 registration — verbatim, landed into the ADR before any bench

> HOTSPOT = trigger evaluation at horizon nodes (can_win_this_turn + plan-t
> queries per horizon node). INSTRUMENT: existing bench chain, staged+q vs
> staged, ttd PRIMARY per D-374's lesson, nps context. BRACKET (ESTIMATED):
> nodes-to-same-depth inflation <= 2.0x; ABORT if > 3.0x. ttd may worsen;
> strength is SPRT's alone. Numbers do not move.

Recorded as `D-388` (this document's landing commit) before Phase 2 IMPL
starts any bench, per the dispatch's own instruction ("architect's, verbatim
into the ADR before any bench").

## 10. Summary of what REVIEW-design must attack

1. §2.1's gap finding (`pvs.rs:199–217` never queries threats at the
   horizon) — re-verify against HEAD.
2. §3.2/§3.3's trigger-to-query mapping — confirm `MinimalCover::One`
   presence is the correct t≤1 test (not `covers.iter().all(...)`, which
   under-fires when a t=1 position also admits an unrelated 2-cell cover;
   §3.2's own text explains why).
3. §3.4's `LAW-RIPOSTE`→`LAW-FORCE` citation correction — confirm it changes
   no ruling, only the citation.
4. §4's `Phase::Second` correction to "no quiet stage" — confirm the
   alternative reading (suppress unconditionally) really does hit
   `NO_CANDIDATES_MID_TURN`, and that the correction is the minimal fix.
5. §3.5's open question (`is_pv` `BatchedLost` vs the cap) — rule on it or
   confirm the stated default.
6. §7's `q_budget: Option<u32>` threading and the depth-0 TT-store rule — the
   one place this design asks IMPL to hold two different meanings of "depth"
   apart (recursion control vs. TT-claim width); confirm the separation is
   sufficient and that no path can conflate them.
7. Confirm §5's zone claim structurally (no `within_radius` reachable from a
   gate decision) against HEAD, not just this document's description of it.
