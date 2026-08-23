# WP-1.6 — Threat-only zone-bounded quiescence: design

**Revision 2.** Revision 1 (`9fa27c8`) was reviewed by REVIEW-design and
FAILED (`docs/experiments/wp16_design_REVIEW.md`): four CONFIRMED blocking
findings (C1–C4) plus two non-blocking (C5, C6). This revision fixes all six
in place; §§3.2, 3.3, 3.4, 3.5, 7, 8 changed, §§1, 2, 4, 6, 9 did not (that
review's V1–V7 verified those unchanged). Per CLAUDE.md, an amendment reopens
the review however small the diff — this is the scoped re-review revision,
not a second independent design.

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

`threats.can_win_this_turn(us, left)` where `left = StonesLeft::from_state(state)`
(`query.rs:51–58`, C5-fixed: not the literal `StonesLeft::Two` revision 1
used — `from_state`'s own doc records `Phase::First` does NOT imply `Two`,
"or it is turn 1"; core reads the phase, this design never re-derives it,
rule 2) (`query.rs:231`, `PROTO-NODE` step 1). `Some(witness)` → terminal:
`mate_in(turns_from_root + 1)`, zero extra nodes, no recursion — identical
cost shape to `staged_candidates`'s `WinNow` row (`staged.rs:186–189`) and to
`pvs::visit`'s own `PlyOutcome::Win` handling (`pvs.rs:311–321`). Always
evaluated at every horizon **under `CandidatePolicy::Staged` only** (C6-fixed:
a `Radius` seat's `Position` carries no `ThreatState` at all —
`position.rs:48,67,187–194` — so trigger (a) is gated on the same
`self.policy` match `pvs::visit`'s `depth_plies > 0` branch already runs,
never a bare "always"), regardless of `q_budget` — it is not an extension, it
is the missing check §2.1 identified.

**Consequence for arena comparisons, named because it changes an existing
experiment's shape:** after this WP, a `staged` seat differs from a `radius`
seat in TWO ways (threat-first generation, per WP-1.5b, AND the horizon
win-now check), not one. Any future staged-vs-radius pairing is no longer the
single-variable comparison WP-1.3(a)/D-386's SPRT was.

### 3.2 Trigger (b) — opponent holds any plan (`LAW-FORCE`), t ∈ {1, 2}

**Revision 2, C4-fixed.** Revision 1 fired only on t = 1 and excluded t = 2 on
two grounds; REVIEW-design showed both wrong: (a) "already reachable by the
parent's FILTERED row" is false — the node under discussion sits at
`depth_plies == 0`, and it is precisely this node's own forced continuation
nobody has searched, the exact shape of gap §2.1 names for win-now, applied
here to the FILTERED row instead; (b) the `LAW-LEDGER` reading was inverted —
"the defender's turn is worth 2 − t free stones" means t = 1 is the class
that PAYS the mover a free stone and usually fizzles, while t = 2 pays
nothing and is where `LAW-RIPOSTE`'s flip-initiative danger actually lives
(a forced double-block is the one outcome a riposte check exists to catch).
Excluding t = 2 therefore extended on the weak class and stood pat on the
strong one, and it is also the cheap class: MEASURED
(`docs/experiments/U3_tier_t.md` §6.2 census, "cover union when FILTERED"
row) 2.17–2.27 cells per node across all four regimes — a t ∈ {1,2} extension
branches ~2 wide, nowhere near the width that made trigger (c) blow its own
bracket (§3.3).

Query `threats.blocking_covers(us, HitBudget::from(left))` where
`left = StonesLeft::from_state(state)` (C5-fixed, matching §3.1 and §4's
existing contract; `cover.rs:201`, `LAW-FORCE`'s survival set per D-267).
Classification, over `Cover` — now the WHOLE of `Cover::Minimal`, no case
split needed for the gate decision itself:

- `NothingToBlock` → t = 0, no opponent plan, no defensive obligation. Not
  this trigger.
- `Minimal(_)` → t ∈ {1, 2}, **trigger fires**, unconditionally on the
  covers' shape. `LAW-FORCE` (`threat_calculus_v1.md:49–53`): "every
  non-losing mover move hits all opponent plans" — this is the law's own
  condition, with no further reading required.
- `Impossible` → t ≥ 3, `LAW-OVERLOAD`. Not trigger (b); handled by §3.5.

Candidate cells on trigger: `Cover::cells()` (the union over ALL inclusion-
minimal covers, `cover.rs:108–116`) — identical in shape to the FILTERED
row's own `filtered()` (`staged.rs:251–257`). `LAW-FORCE` is the correct
citation for "every non-losing mover move hits all opponent plans," not
`LAW-RIPOSTE` — see §3.4.

**`MinimalCover::One` vs `Two` is kept, but only for §8's counters, not for
this gate.** `covers.iter().any(|c| matches!(c, MinimalCover::One(_)))` is
the exact t ≤ 1 test (REVIEW-design's V2, confirmed by counterexample: two
hot windows with empty families `{a,b}`/`{a,c}` make `blocking_covers` emit
`One(a)` AND `Two{b,c}` simultaneously — `.all(One)` would under-fire, and
the fix is correctly `.any(One)` — but the gate no longer needs the
distinction at all, since both t=1 and t=2 now fire the same way). Retained
in §8 to split `q_extend_defense` by t for the WP's own analysis, per core
§8, and because the finding that produced it is real and worth keeping
visible even though it stopped being load-bearing for correctness.

### 3.3 Trigger (c) — mover can activate a new plan this turn

**Revision 2, C1-fixed — the trigger set's central defect.** Revision 1 used
`tier_t_union(threats, us, params)` non-empty, which unions
`live_cells_at_count(Two)`, `live_cells_at_count(Three)` and `threat_cells`
for BOTH sides (`staged.rs:294–334`). REVIEW-design showed this fires on
plan-FREE positions: a `LiveCount::Two` window holds two own stones, `DEF-PLAN`
(`threat_calculus_v1.md:29`) requires ≥4 for a plan to exist at all, and
`PAT-O3` (`:108`) records an open three (three own stones — a fortiori two)
has t = 0, "no plan." MEASURED (`U3_tier_t.md` §6.2 census): trigger (c) as
revision 1 specified it is only reachable on a `Cover::NothingToBlock` row
(§3.2 now disposes of everything else), which occurs at 61.5–92.5 % of
nodes, and on those nodes Tier T averages 23–49 cells — the trigger fired on
the large majority of horizons, at main-search width, on exactly the
plan-free positions `PROTO-NODE` step 5's own qualifier (quoted in §1: "Tier
F + Tier T with **t ≥ 2**") excludes. This is the anti-pattern §1 names,
reproduced with a threat-shaped gate instead of a radius one — §9's
registered `<= 2.0x` bracket was already known, before any bench, to miss by
orders of magnitude (ESTIMATED: ~540–2400 child nodes per fired horizon at
23–49 candidates/ply × 2 plies, against a bracket built for a ~2-wide
extension).

**The fix:** condition trigger (c) on ACTIVATION, not on `LAW-SUPPORT`'s
2-turns-out completeness bound. `LAW-SUPPORT`'s k=2 case (`:68–72`,
"attacker candidates restricted to windows with ≥2 own stones — the
completeness license for the Tier-T staged generator") licenses `LiveCount::
Two` for the MAIN search's own multi-turn lookahead, where the search itself
supplies the remaining turns of depth to turn a count-2 window into a plan.
A horizon extension has no such remaining depth to spend — it grants at most
`q_depth_turns` MORE turns, and a count-2 window is arithmetically two turns
from hot regardless. It is the wrong basis for "can the mover create a plan
THIS turn."

Query: `threats.cells_raising_to_hot(us, NearHot::Three, &mut cells)`
(`query.rs:187–192`) — equivalently `live_cells_at_count(us, LiveCount::
Three, ...)` (`query.rs:206–208`; `NearHot::Three` and `LiveCount::Three`
name the same windows, D-267's map entry). `NearHot` is closed at `Three`
in the shipped surface for exactly this reason (`query.rs:101–107`, "no
single cell raises a count-2 window to hot") — the type itself already
refuses the count-2 reading revision 1 used. Non-empty → trigger fires;
candidate cells = the query's own output, `us` only (this trigger is about
the MOVER creating a plan, not the opponent's — trigger (b) already owns the
opponent's side). This is `BOUND-CONVERT`'s subject (`:93`, "one new stone
converts ≤3 pre-threats into threats on hex") and D-267's map entry for
`cells_raising_to_hot`: "the one-stone activation set."

Tier F for `us` is PROVABLY EMPTY here by the same argument `batched()`'s
own doc already gives (`staged.rs:262–266`): trigger (a) having answered
`None` forbids both a win-in-one-ply cell and a hot four-stone window at
`left`, which is exactly what `tier_f` would have contributed. No separate
Tier-F step to run.

**Open empirical question, deliberately left to Phase 2's own registered
bench (D-388), not pre-answered here:** whether a live-3-only, own-side-only
trigger keeps node inflation inside `<= 2.0x`. This document does not have
the census's live-3-only cell count broken out (only the live-2 row is shown
in `U3_tier_t.md` §6.2's table as reproduced in the review); Phase 2 measures
it against the ALREADY-registered bracket and ABORT threshold — exactly what
rule 5 is for. If it still exceeds the bracket, the fix is the same D-315/WP-
1.5c deferral pattern §3.2 no longer needs, not a silent threshold move.

### 3.4 Correcting the dispatch's `LAW-RIPOSTE` citation — REVISION 2, C3-fixed

Core §4 cites `LAW-RIPOSTE` for "a defense must hit every unanswerable plan."
That sentence is `LAW-FORCE`'s content (§3.2 above), not `LAW-RIPOSTE`'s —
this substitution stood up under REVIEW-design's attack (its V5) and is kept.
`LAW-RIPOSTE`'s actual content (`threat_calculus_v1.md:74–77`): "a forced
defensive stone can itself create a plan and flip initiative... any
forcing-line PROVER must check every forced reply for new plans; skipping the
check is unsound."

**Revision 1 misattributed D-267 here and REVIEW-design (C3) caught it: D-267
assigns `LAW-RIPOSTE` and `LAW-LEDGER` to THIS WP by name** ("`LAW-RIPOSTE`
and `LAW-LEDGER` are WP-1.6's", `docs/decisions.md:575`) — `ZONE-R` and
`LAW-DECOMP` alone are Stage 3's. §3 above already quotes this correctly;
revision 1's §3.4 contradicted its own §3. The correction is not clerical:
D-267 says WP-1.6 OWES `LAW-RIPOSTE` an expression, not that it is exempt
from it as a non-prover.

**What discharges the obligation, after the C4/C1 fixes above:** `LAW-
RIPOSTE`'s danger is a forced defensive stone flipping initiative — REVIEW-
design's C4 leg (c) showed revision 1's trigger set caught this only when the
flip reached t ≥ 3 (overload) or t = 1, and silently missed t = 2, "the
double threat, the strongest flip a riposte can produce short of a proven
overload." §3.2's fix (fire on any `Cover::Minimal`, t ∈ {1,2}) closes
exactly that hole: after a hitting cell is played, the child node re-runs the
SAME trigger protocol (§3 in full) on the resulting position, and a forced
reply that created a new t ∈ {1,2} plan — LAW-RIPOSTE's own case — now fires
trigger (b) at the child. `q_depth_turns` caps how far this can run; running
out of budget means "stand pat" (an admission of ignorance, sound for
alpha-beta search) rather than "unsound proof" (a df-pn-style prover's
failure mode, which is what `LAW-RIPOSTE`'s "skipping the check is unsound"
is actually warning against — a claim of proof this WP never makes). This is
the sense in which WP-1.6 expresses `LAW-RIPOSTE`'s content in `ThreatState`'s
existing vocabulary, per D-267's own assignment, rather than being exempt
from it.

### 3.5 The `Impossible` / losing-band case (core §4's D-105 pointer) — REVISION 2, ruled per REVIEW-design

`blocking_covers(us, HitBudget::from(left)) == Cover::Impossible` (C5-fixed:
`left = StonesLeft::from_state(state)`, not the literal `Two`) is
`LAW-OVERLOAD` (t ≥ 3): the mover cannot survive this turn. `staged_candidates`
already distinguishes this by PV-ness (`staged.rs:196–206`):

- `!is_pv` → `StagedRow::OverloadReturn`, zero-cost: return
  `-mate_in(turns_from_root + 2)` (`pvs.rs:277–279`) with no recursion. Free
  exactly like trigger (a) — runs regardless of `q_budget`.
- `is_pv` → `StagedRow::BatchedLost`: the PV must carry a provable line, so
  generation proceeds instead of returning early — **when budget remains**
  (`q_budget` is `None`, about to be granted, or `Some(k)` with `k > 0`):
  fall through to the SAME generation `batched()` already computes for this
  row (real Tier T, safety net included if Tier T is itself empty — the
  `Phase::Second` exception's reasoning applies here for the same reason,
  §4: an `is_pv` line that must be provable is not optional the way an
  offensive gate decision is), consuming one turn of `q_budget` exactly as
  triggers (b)/(c) do.

**RULED (§10 item 5 of revision 1, resolved by REVIEW-design with one
amendment which this revision adopts):** at a gate node whose budget is
EXHAUSTED (`q_budget == Some(0)`, or `q_budget == None` with
`params.q_depth_turns == 0`) and `blocking_covers` answers `Impossible`,
return `-mate_in(turns_from_root + 2)` **regardless of `is_pv`** — drop the
PV gate at exactly this combination, nowhere else. The stated default
(generation half: stand pat, no override) is otherwise adopted as ruled — a
second, PV-conditioned extension mechanism beyond §3.2/§3.3's uniformly-
capped path is exactly what §1's discipline excludes, and §9's bracket is
registered against a single capped path. But the reason `is_pv` gates
generation in the first place — "the PV must carry a provable line, so
generation proceeds" (`staged.rs:86–89`) — is ABSENT at an exhausted gate: no
line can be generated on either branch there (the alternative is
`self.position.value()` with an empty PV slot, `pvs.rs:194`), so nothing is
gained by declining the free, more-accurate mate-band answer. Soundness is
unaffected: `LAW-OVERLOAD` (`threat_calculus_v1.md:55–59`) needs only
"opponent t ≥ 3" (established) plus "defender cannot win this turn" (trigger
(a) already established `None`), and `is_pv` is not one of its conditions.
PV integrity is unaffected: the gate sits at a turn boundary, so a line
ending there is turn-whole and `turns_from_plies` accepts it.

`D-105` is cited by the dispatch only as the PRECEDENT for "a lost position's
score is a live, still-open question" (D-105's `DECIDED_WINDOW_VALUE`
flooring is an UNSTARTED Stage-1 arena experiment, not something this WP
implements) — nothing here depends on that experiment's outcome; this design
never floors an eval, it only chooses between a mate-band shortcut and a
capped stand-pat.

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
comes from `Cover::cells()` (empties of the opponent's HOT windows, §3.2) or
`cells_raising_to_hot(us, NearHot::Three, ...)` (empties of the mover's
LiveCount::Three windows, §3.3, revision 2) — never from `within_radius` (`crate::candidates::within_radius`,
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

**Configs that must gain the field in the same IMPL commit (N3, non-blocking
finding from REVIEW-design, fixed here as a checklist item):** every committed
config using `kind = "staged"` fails to deserialize the instant this field is
schema-required (`#[serde(deny_unknown_fields)]`, hard rule 1's "missing key =
error") — `configs/play_staged_v0.toml`, `configs/tactical_staged_v0.toml`,
`configs/gate_staged_v0.toml`, `configs/instrument_v0.toml`,
`configs/instrument_staged_v0.toml`, and the WP-1.5b arena seats
(`configs/arena_wp15b_staged_vs_r2.toml`,
`configs/arena_wp15b_staged_vs_r2_confirm.toml`,
`configs/arena_wp15b_dryrun.toml`). IMPL updates all of them in the landing
commit; `tools/config_check.sh` is the gate that would otherwise catch this
late.

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

**Recursion shape — one function, one new parameter, and the gate node does
NOT recurse into itself. Revision 2, C2-fixed.** No parallel `quiescence()`
function. `Run::visit` and `Run::child` each gain one parameter, `q_budget:
Option<u32>`. Revision 1 said the gate node (`q_budget == None`) "recurses
with `depth_plies = 2, q_budget = Some(k-1)`" and separately said the TT
store keys on `q_budget.is_some()` — REVIEW-design (C2) showed these two
sentences describe a node that does not exist: the gate node's OWN
`q_budget` is `None` by the first sentence, so the store rule's predicate,
read literally, never covers the gate node's own `Record` write, and each of
the two ways to read past that ambiguity breaks something: (reading A) the
gate node falls through to its own candidate loop with a locally rebound
`depth_plies = 2` and stores that node's `Record` BEFORE any inner
`q_budget.is_some()` node exists to claim depth 0 — a `depth_plies: 2` record
built from a quiescence-narrowed move set, which a later full-width probe at
the same key and `depth_plies == 2` then trusts as a real cutoff
(`record.depth_plies >= depth_plies`, `pvs.rs:229`), reachable across
iterative-deepening iterations at the same table key, not only via
transposition; (reading B) the gate node instead calls
`self.visit(2, alpha, beta, ply, Some(k-1))` as a genuine recursive call and
returns its result — the unsound record disappears, but `visit`'s prologue
(`self.nodes += 1`, `pvs.rs:193`) now runs twice for what is one node's work,
double-counting against `self.stop.is_spent(self.nodes)` — the exact quantity
WP-1.5b's own SPRT was node-matched on (`go nodes 50000`, D-386).

**The fix: the gate node enters the quiescence regime ITSELF, in place, not
through a recursive call.** The moment a gate node (`depth_plies == 0`,
`Phase::First`, `q_budget == None`) grants an extension (trigger (a) already
handled and returned; trigger (b) or (c) fires with budget available), it
REBINDS its own locals — `depth_plies := 2`, `q_budget := Some(k - 1)` (or,
for the §3.5 `is_pv`+budget-available case, the same rebinding) — and falls
through into the REST of this SAME `visit` invocation: the same TT probe/
store code path the `depth_plies > 0` branch already runs
(`pvs.rs:220–237`, `:359–377`), now executing with the rebound locals. There
is no second call frame, so `self.nodes` increments exactly once for this
node (matching `qnodes`'s definition, §8), and the node's own `Record.
depth_plies` is written from the SAME local that decides its candidate
generation — which is where the TT rule below now attaches.

- `None` everywhere in the main tree today — behavior byte-for-byte
  unchanged from the current code whenever `q_budget` stays `None`, which is
  every call site until the first horizon gate grants an extension.
- Once a gate node rebinds to `q_budget = Some(k)` (in place, as above):
  every node from there on — both plies of the granted turn, and every
  subsequent horizon this turn's completion reaches — carries `Some(_)`
  forward through NORMAL recursive `visit`/`child` calls (not rebinding;
  rebinding happens only at a fresh gate) UNTIL the next `depth_plies == 0`,
  `Phase::First` gate, where the SAME in-place-rebind decision re-runs with
  `k` in place of `params.q_depth_turns` (extend again, in place, only if
  `k > 0`; otherwise the ordinary stand-pat, unchanged).
- `child()` (`pvs.rs:387–425`) threads `q_budget` through its two recursive
  calls (`full`/the null-window scan) unchanged in value — it is orthogonal
  to the same-side/opponent window logic `child` already owns.

**TT sharing at depth 0 (core §7), predicate corrected.** A node's `Record`
stores `depth_plies: 0` iff THAT NODE's OWN `q_budget` (after any in-place
rebind it just performed, per the fix above) is `Some(_)` — equivalently:
every node reached by continuing past a granted gate, WHETHER OR NOT it is
itself the rebinding gate. This now includes the rebinding gate node itself,
closing the predicate gap C2 found. REGARDLESS of the local extended-turn ply
countdown (2 or 1) that node is using for its OWN recursion control —
`depth_plies` (recursion control) and the TT-stored depth (a claim about
search width already done) are DELIBERATELY DIFFERENT quantities from the
moment of rebinding onward: a full-width prober's cutoff test
(`record.depth_plies >= depth_plies`, `pvs.rs:229`) must never accept a
quiescence-narrowed bound as satisfying a real `depth_plies >= 1` requirement
— storing `0` guarantees this by construction. Probing (reading) is
unrestricted: a quiescence node probing the table is content with ANY stored
record regardless of its depth (its own requirement is `>= 0`, trivially
satisfied — true at every quiescence-regime node including the just-rebound
gate, since its probe also runs after the rebind), so a full-width `Exact`/
appropriate-bound entry already present is freely reused as a cutoff inside
qsearch — this is the actual content of "TT shared... with the existing key,"
and it needs no key change (phase bit + side to move are already in
`GameState::key()`, unchanged by this design).

**N1/N2, named for IMPL rather than fixed here (concerns, not blocking):**
`depth_plies` genuinely carries three roles across this function by the time
of a rebind (the pre-rebind horizon test at `pvs.rs:199`, the post-rebind
recursion-control countdown, and the always-0 TT-store value once
`q_budget.is_some()`) — the fix above keeps them apart by construction but
IMPL should not collapse them into one variable. Separately, two
quiescence-regime gate nodes at the same table key with different remaining
`q_budget` share one `Record` (the key carries side-to-move and the
intra-turn phase bit only, never `q_budget` itself, §7's determinism
argument below) — this is ordinary, accepted engine behavior (a shallower
quiescence bound is simply not trusted at a deeper requirement, per the
depth-0 rule above) and does not breach the determinism law, but it is
QUIESCENCE-TO-QUIESCENCE reuse, a different direction from the full-width-to-
quiescence reuse the paragraph above argues for.

**Determinism law.** No new nondeterminism source: `staged_candidates`,
`blocking_covers`, `cells_raising_to_hot` are all already total/deterministic/
sorted (`query.rs`'s module doc: "Every one is total, deterministic... None
consults a clock, a hasher's iteration order"). `q_budget` is plain recursion state,
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
- `q_extend_defense: u64` — trigger (b) granted an extension (any
  `Cover::Minimal`, t ∈ {1, 2}, revision 2/§3.2).
- `q_extend_defense_t1: u64` / `q_extend_defense_t2: u64` — `q_extend_defense`
  split by `covers.iter().any(MinimalCover::One)` (t=1) vs not (t=2) — the
  distinction §3.2 keeps for exactly this purpose after C4 removed it from
  the gate decision. `q_extend_defense_t1 + q_extend_defense_t2 ==
  q_extend_defense`.
- `q_extend_offense: u64` — trigger (c) granted an extension (mutually
  exclusive with defense: §3.2/§3.3 checked in that order, defense first,
  per `LAW-FORCE`'s "every non-losing move" precedence over pure offense).
- `q_stand_pat_no_trigger: u64` — gate reached, no trigger fired.
- `q_stand_pat_cap: u64` — a trigger fired but `q_budget` was already
  exhausted (§3.5's `is_pv`+`Impossible` case at an exhausted gate is counted
  under `q_overload_return`, not here — it returns the mate score, not a
  stand-pat, per §3.5's revision-2 ruling).

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
into the ADR before any bench"). D-388 stands unedited by this revision — the
registered numbers do not move (rule 5); revision 2 only changes what the
design being measured against them IS.

**N4, named per REVIEW-design, not yet resolved — Phase 2 must state it
before launching the bench:** "staged+q vs staged" does not by itself say
whether the right-hand `staged` seat is `q_depth_turns = 0` or the pre-WP-1.6
build. §6 is explicit that `q_depth_turns == 0` still runs the free trigger
(a)/`OverloadReturn` checks (§3.1/§3.5) — a `q=0` seat therefore already
returns mate scores at horizons the pre-WP-1.6 engine scores statically, and
is a different player from that pre-WP-1.6 build. The two right-hand seats
measure different things (the extension mechanism alone, vs. the extension
plus the free horizon checks). Phase 2 names which one D-388's registered
comparison uses, in the bench's own commit, before running it.

## 10. Revision 1 → revision 2 changelog, and what the scoped re-review must attack

Revision 1 (`9fa27c8`) FAILED REVIEW-design
(`docs/experiments/wp16_design_REVIEW.md`) on four CONFIRMED blocking findings
plus two non-blocking. Disposition, each tied to its fix above:

1. **C1 (blocking) — trigger (c) fired on plan-free positions at 61.5–92.5 %
   of nodes, dropping `PROTO-NODE` step 5's `t ≥ 2` qualifier.** Fixed: §3.3
   now uses `cells_raising_to_hot(us, NearHot::Three)` (own side, live-3
   activation only), not `tier_t_union`. Node-inflation consequence left as
   an explicit open empirical question for Phase 2's registered bench, not
   pre-answered.
2. **C2 (blocking) — the TT-store predicate did not cover the gate node; the
   two readings of the recursion were unsound / node-double-counting
   respectively.** Fixed: §7 now specifies the gate node rebinds its own
   `depth_plies`/`q_budget` in place and continues within the SAME `visit`
   call — no second call frame, `self.nodes` increments once, the store
   predicate (`q_budget.is_some()` AFTER any rebind) covers the node that
   writes it.
3. **C3 (blocking) — §3.4 misattributed D-267 (said `LAW-RIPOSTE` was "Stage
   3's"; D-267 says it is "WP-1.6's"), contradicting the document's own §3.**
   Fixed: §3.4 now states D-267's assignment correctly and grounds "the
   obligation is discharged" in the C4 fix (below) rather than in a claimed
   exemption.
4. **C4 (blocking) — the t=2 exclusion in §3.2 rested on two refuted
   arguments and left LAW-RIPOSTE's flip-initiative case (the double-threat
   forced reply) uncaught.** Fixed: §3.2 now fires trigger (b) on any
   `Cover::Minimal` (t ∈ {1,2}); MEASURED cost is cheap (2.17–2.27 cells/node).
5. **C5 (non-blocking) — §3.1/§3.2 used literal `StonesLeft::Two`/
   `HitBudget::Two` where §4 already used `StonesLeft::from_state`.** Fixed
   throughout §3.1, §3.2, §3.5.
6. **C6 (non-blocking) — §3.1's "always evaluated... regardless of q_budget"
   did not state the `CandidatePolicy::Staged`-only scoping, and panics under
   `Radius` (no `ThreatState`).** Fixed: §3.1 states the scoping and names the
   staged-vs-radius arena-comparison consequence.

Concerns N1–N3 addressed inline (§7's note, §6's config list); N4 left as an
explicit unresolved item for Phase 2 (§9); N5 was the §10-item-5 ruling,
applied in §3.5.

**§10 item 5 of revision 1 (the `is_pv`+`BatchedLost`+cap open question) is
RULED, not open**: REVIEW-design's amendment is adopted in §3.5 — the mate
return is not `is_pv`-gated at an EXHAUSTED gate specifically, and remains
`is_pv`-gated (generation proceeds when budget allows) everywhere else.

**What the scoped re-review (same dispatch pattern, per CLAUDE.md's "an
amendment reopens the review however small the diff") should attack, beyond
re-confirming V1–V7 still hold against this revision's line numbers:**

1. §3.2/§3.3's fixed triggers — do they actually close C1 and C4 as claimed,
   with no new over/under-firing introduced by the fix itself (in particular:
   does `cells_raising_to_hot(us, NearHot::Three)` ever return cells that
   `Position::place` would refuse, or that duplicate what trigger (b) already
   covers in a way that double-extends)?
2. §3.4/C3's corrected D-267 reading and the "obligation discharged via
   C4's fix" argument — does closing the t=2 gap in §3.2 actually suffice
   for what D-267 assigns WP-1.6, or is something else still owed?
3. §7's in-place-rebind fix (C2) — is "the same `visit` call frame,
   rebinding locals, no recursion" actually implementable as stated inside
   `pvs.rs`'s existing control flow (the `if depth_plies == 0 { ... }`
   early-return structure), or does it require a restructuring this document
   has not fully specified? Flag anything IMPL would have to invent.
4. §3.5's ruling — re-derive the soundness argument independently rather
   than accepting REVIEW-design's own conclusion on trust.
5. N4 (§9) — confirm it is adequately flagged as a Phase-2 obligation rather
   than something this design owed to resolve itself.
