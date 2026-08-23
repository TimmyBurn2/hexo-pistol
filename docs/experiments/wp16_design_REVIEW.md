# REVIEW-design — `docs/wp16_quiescence_design.md` (WP-1.6)

**Revision reviewed:** `9fa27c874d56212e3d33cc2a2aa4ac35a54c191f` (`9fa27c8`).
**Matches HEAD:** YES at review start (`git rev-parse HEAD` = `9fa27c8`, working
tree clean apart from this report, which is untracked).
**Reviewer:** REVIEW-design subagent, fresh context, not the implementing session.
**Sources read directly (not via the design's paraphrase):**
`docs/research/threat_calculus_v1.md` (whole), `docs/decisions.md` D-41, D-104,
D-105, D-111, D-267, D-310, D-315, D-352, D-374, D-386, D-388,
`crates/pistol-search/src/{pvs.rs,staged.rs,info.rs,params.rs,candidates.rs,position.rs}`,
`crates/pistol-engine/src/config.rs`,
`crates/pistol-solver/src/{query.rs,cover.rs}`,
`docs/experiments/U3_tier_t.md` §6.2 census block, `configs/*.toml`,
`docs/ROADMAP.md`.

## VERDICT: **FAIL**

Four CONFIRMED blocking findings. Per CLAUDE.md's STOP-1 rule this is the first
FAIL: fix the findings once (the document is small and the fixes are local), then
a scoped re-review by the same dispatch pattern; a second FAIL stops the WP.

The document is unusually careful and five of its seven self-nominated attack
surfaces (§10 items 1, 2, 4, 7 and the citation half of item 3) survive attack
intact — those are recorded as VERIFIED below. What fails is the **trigger set
itself** (§3.2/§3.3), which as specified fires on plan-free positions at a
measured 61.5–92.5 % of nodes while declining to fire on the one threat class
that flips initiative, and the **TT-store rule** (§7), whose stated predicate
does not cover the node it exists to cover.

---

## CONFIRMED — BLOCKING

### C1. §3.3's trigger (c) is not a threat trigger. It drops `PROTO-NODE` step 5's own `t ≥ 2` qualifier, which §1 quotes verbatim as "the registered cure".

**The claim under attack.** §1: *"The registered cure is threat-only,
zone-bounded extension (report §B; calculus `PROTO-NODE` step 5: 'Threat-only,
zone-bounded (Tier F + Tier T with t ≥ 2), never full-width')."*
§3.3: *"Query: `tier_t_union(threats, us, params)` … Non-empty → trigger fires."*

**What the calculus actually says.** `threat_calculus_v1.md:135`, PROTO-NODE
step 5, in full: `**Quiescence.** Threat-only, zone-bounded (Tier F + Tier T with
t ≥ 2), never full-width (was S3).` The design quotes this sentence and then
never mentions the `t ≥ 2` qualifier again. Both available readings of `t` refute
§3.3:

- **DEF-T reading (`t` = threat number, `threat_calculus_v1.md:30`).** This is
  the only meaning `t` carries anywhere in the calculus — LAW-FORCE, LAW-OVERLOAD,
  RULE-EXACT, LAW-LEDGER, the §5 pattern table and §6 tempo table all use `t` for
  the minimum hitting set and nothing else. Under it, step 5 restricts quiescence
  to the Tier-T subset carrying a threat number of at least 2. §3.3's trigger has
  no `t` condition at all.
- **Support reading (`t ≥ 2` = LAW-SUPPORT's ≥2-own-stone qualification).** Under
  this reading the qualifier is redundant — Tier T *is* the ≥2-own-stone set
  (`LAW-SUPPORT`, `threat_calculus_v1.md:72`) — which is an argument against the
  reading, not for it. But even granting it, §3.2's exclusion of `t = 2` (C4
  below) is still unexplained.

**Why this is not a wording quibble — the trigger fires on positions with no
plan on the board.** `tier_t_union` (`staged.rs:294–308`) unions
`tier_t_side(us, own_count)` and `tier_t_side(us.opponent(), opponent_count)`, and
`tier_t_side` (`staged.rs:321–334`) is
`live_cells_at_count(Two) ∪ live_cells_at_count(Three) ∪ threat_cells`. A
`LiveCount::Two` window holds **two** own stones. `DEF-PLAN`
(`threat_calculus_v1.md:29`) requires **≥4** own stones for a plan to exist at
all, and `PAT-O3` (`:108`) records that an open three has `t = 0` and *"no plan"*.
So trigger (c) fires on positions where neither side has a plan — the textbook
definition of a quiet position — which is the exact inverse of "threat-only".

Note further that trigger (c) is only *reachable* on a `Cover::NothingToBlock`
row (§3.2 disposes of `Minimal`, §3.5 of `Impossible`), and `NothingToBlock`
means the opponent has **no hot window whatsoever** (`cover.rs:202–205`,
`blocking_covers` returns it when `hot_windows(defender.opponent())` is empty).
Trigger (c) is therefore *by construction* only ever evaluated at nodes where the
side not to move has no plan; the trigger then asks whether anybody, anywhere,
owns a live window at count two. In the shipped config
(`tier_t_own_count = 2`, `tier_t_opponent_count = 3`,
`configs/play_staged_v0.toml:35–36`, `configs/instrument_staged_v0.toml:42–44`)
this is "does either side have any two stones on a common open line".

**MEASURED, from this repo's own registered instrument** — the census block in
`docs/experiments/U3_tier_t.md` §6.2 (rendered by
`crates/pistol-solver/tests/wp15b_census.rs`, pinned by
`the_carved_design_units_carry_this_censuss_table_verbatim`), columns
*corpus roots / +1..3 turns r2 draw / r8 draw / playouts*:

| quantity (census row) | corpus roots | r2 draw | r8 draw | playouts |
|---|---|---|---|---|
| BATCHED nodes (`NothingToBlock`) | **70.8 %** | **61.5 %** | 65.5 % | **92.5 %** |
| live-2 own, mean | 7.21 | 11.18 | 11.07 | 23.78 |
| live-2 opponent, mean | 12.17 | 12.45 | 10.90 | 25.43 |
| option C — Tier T (threshold, ADOPTED = the shipped `own 2 / opp 3` reading) | **23.29** | **31.50** | 30.26 | **48.73** |

So: on 61.5–92.5 % of nodes the row is BATCHED, and on those nodes Tier T is
non-empty with a mean of 23–49 cells. Trigger (c) therefore fires at **the large
majority of horizons**, and §4 then searches the granted turn at the *same width
the main search uses on those very nodes* ("calls the existing
`staged_candidates` … exactly as `pvs::visit`'s `depth_plies > 0` branch already
does").

**ESTIMATED consequence for §9's own registered bracket.** One granted turn is
two plies at ~23–49 candidates each: ≈ 540–2400 child nodes per fired horizon
(ESTIMATED, from the MEASURED set sizes above), at 61.5–92.5 % of horizons, and
horizons are the bulk of a fixed-depth tree. §9 registers
`nodes-to-same-depth inflation <= 2.0x; ABORT if > 3.0x`. That abort fires by two
to three orders of magnitude, and it fires for a reason knowable from the
document plus a table already in the repo — i.e. **before** the bench, which is
what a design review is for. A design whose central trigger is known in advance
to blow its own registered bracket is a design defect, not a measurement.

**This is the anti-pattern §1 names, in a different dress.** §1: *"The prior
'experiment-2' failure broadened the CANDIDATE SET at the horizon."* §5's
structural argument (VERIFIED below) shows the broadening is not to
`within_radius`; it does not show the broadening is threat-conditioned. Extending
by a full Tier-T-wide turn at ~70 % of leaves, on positions where DEF-PLAN says no
plan exists, is a horizon broadening whose gate happens to be spelled
`tier_t_union(..).is_empty()` instead of `radius`.

**What a fix looks like** (recommendation, not a requirement): condition trigger
(c) on an actual threat quantity rather than on Tier T's non-emptiness — e.g. the
mover owns a hot window (`hot_windows(us)` non-empty / `threat_cells(us)`
non-empty), or `cells_raising_to_hot(us, NearHot::Three)` non-empty (one stone
from a plan, `BOUND-CONVERT`'s subject, D-267's map entry). Either is already in
the shipped surface and neither adds a query. Whatever is chosen, the document
must state which reading of PROTO-NODE step 5's `t ≥ 2` it adopts and why,
because §1 makes that sentence the design's licence.

---

### C2. §7's TT-store predicate does not cover the gate node, and the document's two available readings of "recurse with `depth_plies = 2`" each carry a distinct defect. IMPL cannot implement §7 without making a choice §7 does not license.

**The exact sentences.**
§7, store rule: *"Every TT `Record` written from a node where `q_budget.is_some()`
stores `depth_plies: 0` in the record …, REGARDLESS of the local extended-turn
ply countdown (2 or 1) that node is using for its OWN recursion control."*
§7, bullet 2: *"At a `depth_plies == 0`, `Phase::First` gate node **with
`q_budget == None`**: run §3's checks. … Trigger (b) or (c) firing … : recurse
with `depth_plies = 2, q_budget = Some(params.q_depth_turns - 1)`."*

The gate node is the node whose local countdown is the "2" the store rule
parenthesises. Its own `q_budget` is `None` by bullet 2's own words. The store
rule's predicate therefore **excludes exactly the node the rule's parenthesis
names**.

**Reading A — the gate node falls through into its own candidate loop with
`depth_plies := 2`.** Then `pvs.rs:359–377` stores
`Record { depth_plies, .. }` = `depth_plies: 2` for a search whose move set was
`Cover::cells()` or `tier_t_union` — a quiescence-narrowed set. A later
full-width node at the same key with `depth_plies == 2` passes
`record.depth_plies >= depth_plies` (`pvs.rs:229`), and with `!is_pv` and
`Bound::Exact` returns that score at line 237. That is precisely the event §7
says *"storing `0` guarantees this by construction"* prevents. **Reachable on
every iterative-deepening iteration**, not only via transposition: iteration *d*
reaches key K at `depth_plies == 0` (gate, stores a narrowed depth-2 record);
iteration *d+1* reaches K at `depth_plies == 2` inside a null-window scan and
takes the cutoff. The narrowing direction is not conservative: on a max node a
narrowed move set yields `best_score <= true value`, so the `Bound::Upper`
(`best_score <= original_alpha`) and `Bound::Exact` cases are both unsound as
stored.

**Reading B — the gate node re-invokes `visit(2, alpha, beta, ply)` with
`q_budget = Some(k)` and returns its result.** The unsound record disappears
(the inner call has `q_budget.is_some()` and stores 0; the outer returns before
its own store). But `visit`'s prologue is `self.nodes += 1; self.pv.clear(ply)`
(`pvs.rs:193–194`), so every granted extension **double-counts one node** at the
same ply. `self.nodes` is not a diagnostic: it is the node-budget unit
(`should_stop` → `self.stop.is_spent(self.nodes)`, `pvs.rs:450–456`), it is what
`Stop::Nodes` spends, and it is the quantity WP-1.5b's own SPRT was matched on
(`go nodes 50000`, D-386). §8's `qnodes` definition (*"every `visit` call in that
regime"*) reads as if written for reading B without saying so.

Both readings are defensible from the text; they differ in soundness and in what
a node budget buys. §10 item 6 asks REVIEW-design to *"confirm the separation is
sufficient and that no path can conflate them"*: it is **not** sufficient, and
the conflation is in the specification rather than in a hypothetical IMPL slip.

**Minimal fix.** State that the gate node enters the quiescence regime *itself*
at the moment it grants — i.e. it sets its own `q_budget := Some(k)` before
continuing (reading A plus predicate coverage) — and say explicitly that the node
is not re-entered through `visit`, so `self.nodes` counts it once. Then §7's
predicate and its parenthesis agree, and §8's `qnodes` is unambiguous.

---

### C3. §3.4 misquotes D-267 on the one point its argument rests on, and contradicts the same document's §3.

**§3.4:** *"D-267 lists `LAW-RIPOSTE` alongside `ZONE-R`/`LAW-DECOMP` as 'Stage
3's' for the proof-engine sense of the term."*

**D-267, verbatim** (`docs/decisions.md:575`): *"`DEF-STAR`, `DEF-TEMPO`/`ADOPT-TEMPO`,
`LAW-RIPOSTE`, `LAW-LEDGER`, `LAW-DECOMP`, `ZONE-R`, `THM-WINDOW` and `E-INIT`
have NO counterpart anywhere in the shipped surface — **`LAW-RIPOSTE` and
`LAW-LEDGER` are WP-1.6's**, `ZONE-R` and `LAW-DECOMP` are Stage 3's, and
`THM-WINDOW`/`E-INIT` are eval's."*

D-267 assigns `LAW-RIPOSTE` to **this WP**, explicitly and by name, and assigns
`ZONE-R`/`LAW-DECOMP` — and only those two — to Stage 3. §3.4 states the
opposite. The document's own §3 quotes it correctly two paragraphs earlier
(*"D-267 records that neither has 'any counterpart anywhere in the shipped
surface' and that both are 'WP-1.6's'"*), so the document contradicts itself as
well as the ADR.

**Load-bearing, not clerical.** §3.4 uses the misattribution to conclude
*"Quiescence is not a prover: it never claims a proof, so it owes no soundness
obligation there."* D-267's actual sentence says the opposite about ownership:
WP-1.6 is the package that owes `LAW-RIPOSTE` an expression. §3's own framing —
*"this WP's job is to express their CONTENT in `ThreatState`'s existing calls"* —
is the correct one and is what §3.4 then walks back. The concrete cost of the
walk-back is C4's riposte hole.

Note that §3.4's *substantive* citation correction is **right** and survives (see
VERIFIED V5): "a defense must hit every unanswerable plan" is LAW-FORCE
(`threat_calculus_v1.md:49–53`), not LAW-RIPOSTE (`:74–77`). Only the D-267
attribution and the conclusion drawn from it are wrong.

---

### C4. §3.2's `t = 2` exclusion is unlicensed: both stated reasons are wrong, and the excluded class is exactly LAW-RIPOSTE's flip-initiative case.

**The exact sentence.** §3.2: *"`Minimal(covers)` with no `MinimalCover::One`
present → t = 2. **NOT this trigger.** At t=2 the whole turn is consumed hitting
(2 − t = 0 free stones); LAW-LEDGER gives the mover no spare tempo to develop,
and the position is a fully-determined forced double-block already reachable by
the ordinary FILTERED row at the PARENT's own depth budget — it is not a horizon
blind spot the way a chainable t=1 sequence is."*

**Leg (a) — "already reachable by the ordinary FILTERED row at the PARENT's own
depth budget" is false.** The parent's FILTERED row generated the *parent's*
candidate cells. The node under discussion is at `depth_plies == 0`; its own
forced continuation is searched by nobody — the parent spent the last ply
arriving here. `pvs.rs:199–218` returns `self.position.value()` and expands no
child. The sentence conflates "the parent took a FILTERED row" with "this node's
forced reply was searched". This is the same class of gap §2.1 correctly
identifies for win-now, applied to a different row, and §3.2 argues it away.

**Leg (b) — the LAW-LEDGER reading is inverted.** `LAW-LEDGER`
(`threat_calculus_v1.md:79–83`): *"Against a plan family with threat number t, the
defender's turn is worth **2 − t** free stones. t=1 chains bank the defender one
stone/turn ⇒ a t=1 chain wins only if it terminates in a win before the bank funds
a counter-fork."* The ledger's own content is that **t = 1 is the weak class** —
the defender is *paid* a free stone every turn, which is why a t=1 chain usually
fizzles — and **t = 2 is the strong class**, where the defender banks nothing and
the attacker keeps the initiative for free. §3.2 extends on the class the ledger
says peters out and stands pat on the class the ledger says does not. "No spare
tempo to develop" is a description of danger, not of quiet; a node where the
mover is *entirely* forced (LAW-FORCE, `:49–53`, every non-losing move hits all
plans) is the worst possible place for a static read, not the safest.

**Leg (c) — the excluded class is LAW-RIPOSTE's, and this is where C3's
walk-back costs something concrete.** `LAW-RIPOSTE` (`:74–77`): *"A forced
defensive stone can itself create a plan and **flip initiative**."* §3.4's
consolation is that the riposte check is inherited for free because *"after a
hitting cell is played, the child node re-runs the SAME trigger protocol"*. Walk
that through with the trigger set as specified. The mover plays the cover cell(s);
the turn completes; the opponent is to move at the next gate. If the mover's
forced stone created:

- `t ≥ 3` for the mover ⇒ `Cover::Impossible` at that gate ⇒ §3.5's
  `OverloadReturn` mate. **Caught.**
- `t = 1` ⇒ trigger (b). **Caught.**
- **`t = 2` ⇒ stand pat. NOT caught** — and `t = 2` is the double threat, the
  strongest flip a riposte can produce short of a proven overload.

So the one outcome LAW-RIPOSTE exists to warn about is the one outcome the
trigger set drops. §3.4's *"exactly the riposte check `LAW-RIPOSTE` asks a prover
to make"* is therefore not what §3.2 delivers.

**Cost of including it is near zero.** MEASURED, `U3_tier_t.md` §6.2 census row
`cover union when FILTERED`: **2.17 / 2.17 / 2.19 / 2.27** cells per node across
the four regimes, against Tier T's 23–49 (C1's table). A `t = 2` extension
branches ~2 wide; trigger (c) as written branches ~23–49 wide. The design
excludes the cheap forced case and admits the expensive quiet one.

**Recommendation.** Fire trigger (b) on `Cover::Minimal(_)` — i.e. on any
`t ∈ {1, 2}` — which is LAW-FORCE's own condition and needs no case analysis; or
state a positive argument for the `t = 2` exclusion that does not rest on either
refuted leg. If the class is deferred to WP-1.5c as §3.2 offers, the deferral
must be recorded as debt with an owner (the D-310/D-315 pattern), not as a
reasoned boundary, since the reasoning given does not hold.

---

## CONFIRMED — NON-BLOCKING (fix in the same pass)

### C5. §3.1 and §3.2 state `StonesLeft::Two` / `HitBudget::Two` as literals where core owns the answer; §4 of the same document states the contract correctly.

§3.1: *"`threats.can_win_this_turn(us, StonesLeft::Two)`"*. §3.2: *"Query
`threats.blocking_covers(us, HitBudget::Two)`"*. §4: *"calls the existing
`staged_candidates` with `left = StonesLeft::from_state(state)`"*.

`StonesLeft::from_state` reads `GameState::stones_owed`, *"which is pistol-core's
own answer, never re-derived here (CLAUDE.md rule 2)"* (`query.rs:34–58`), and its
doc records that `Phase::First` does **not** imply `Two`: *"One stone left: the
mover is at `Phase::Second`, **or it is turn 1**"* (`query.rs:28`). A literal
`StonesLeft::Two` at a turn-1 node would admit `WinWitness::Pair`
(`query.rs:125–126`: *"Valid ONLY at `StonesLeft::Two` — D-243's phase
conditioning"*), i.e. claim a mate on a turn that owes one stone. I could not
construct a reachable turn-1 horizon (`plies_for` gives turn 1 one ply, so the
first horizon below a turn-1 root is turn 2 at `Phase::First`), so this is a
**contract defect, not a demonstrated bug**. Write
`left = StonesLeft::from_state(state)` and `HitBudget::from(left)`
(`query.rs:79–86`) in §3.1/§3.2 as §4 already does.

### C6. §3.1's "Always evaluated at every horizon" panics under `CandidatePolicy::Radius`; the scoping to `Staged` is derivable but never stated.

`Position` carries a `ThreatState` only when constructed with `tracks_threats`
(`position.rs:48, 67`), which is *"the caller's `CandidatePolicy::Staged` test"*,
and `staged_context` panics with `POSITION_DESYNC` otherwise
(`position.rs:187–194`). §3.1 says trigger (a) is *"Always evaluated at every
horizon regardless of `q_budget`"*. Under a Radius seat there is no
`ThreatState` to evaluate it against. §6 puts `q_depth_turns` on the `staged`
variant only, so the intended scoping is inferable — but it is inferable, not
stated, and §3.1's word is "always".

One sentence fixes it. It has an experiment consequence worth stating in the same
sentence: after WP-1.6 a staged-vs-radius arena match changes **two** things
(staged generation *and* the horizon win-now check), so any such pairing is no
longer a single-variable comparison.

---

## CONCERNS (not FAIL drivers)

**N1. §7 needs three distinct depth quantities and names two.** `depth_plies` is
read at four sites: the horizon test (`pvs.rs:199`), the TT cutoff test
(`:229`), `child(depth_plies - 1, ..)` (`:324, :328`) and the store (`:364`). §7
specifies the store (0) and the recursion (2/1) and asserts of the probe that
*"its own requirement is `>= 0`, trivially satisfied"* — which is true of a gate
node that has **not** raised its local `depth_plies`, and false the instant it
has (the test becomes `record.depth_plies >= 2`). The direction is conservative
(fewer cutoffs, no unsoundness), so this is a specification gap rather than a
defect; but §10 item 6 asks specifically about holding the two meanings apart,
and the honest answer is that there are three.

**N2. Quiescence-to-quiescence TT reuse across different remaining budgets is
unaddressed.** The key carries side-to-move and the intra-turn phase bit and
nothing about `q_budget` (§7 says so approvingly). Two gate nodes at the same key
with different remaining budget — one able to extend twice more, one exhausted —
share an entry. §7's *"a full-width `Exact`/appropriate-bound entry already
present is freely reused as a cutoff inside qsearch"* is sound (a full-width
record dominates), but it argues only the full-width→quiescence direction; the
quiescence→quiescence direction is the one that makes a leaf value depend on
table history. This is a well-known and generally accepted engine behaviour, and
it does not breach the determinism law (within one `Run` the table is
deterministic, `Run::new` owns no clock or hasher order) — but §7 claims
soundness "by construction" and should say which construction it means.

**N3. Five committed configs break the moment `q_depth_turns` lands.**
`configs/{play_staged_v0,tactical_staged_v0,gate_staged_v0,instrument_v0,instrument_staged_v0}.toml`
carry `kind = "staged"`, and `CandidatePolicy` is
`#[serde(tag = "kind", deny_unknown_fields)]` (`config.rs:150–151`) under hard
rule 1's "missing key = error". §6 specifies the new field and the schema home
correctly but does not name the configs that must be updated in the same commit,
nor the arena configs (`arena_wp15b_*.toml`) that pin the WP-1.5b seats.

**N4. `q_depth_turns == 0` is not "pre-WP-1.6 behaviour", and §9's baseline is
ambiguous about which "staged" it means.** §6 is honest that at 0 *"the gate
check at §3 still runs (trigger (a)/OverloadReturn are always free)"* — so a
`q = 0` seat already returns mate scores at horizons that today's engine scores
statically, and is therefore a different player. D-388/§9 registers
`staged+q vs staged` without saying whether the right-hand seat is `q = 0` or the
pre-WP-1.6 build. Those measure different things (the extension alone, vs the
extension plus the free horizon checks). Name it before Phase 2.

**N5. Consequence of the §10 item 5 ruling, recorded with the ruling** — see the
next section.

---

## RULING ON §10's OPEN ITEMS

### §10 item 5 — `is_pv` + `BatchedLost` + cap exhausted. **RULED, with one amendment.**

**The generation half: the stated default is correct — stand pat, no override.**
Both of §3.5's grounds hold. A second, PV-conditioned extension mechanism is
exactly the kind of unlicensed path §1's discipline exists to exclude, and §9's
bracket is registered against a single uniformly-capped path. Adopt it.

**Amendment — the mate return should not be `is_pv`-conditioned at a capped gate
node.** §3.5 makes `-mate_in(turns_from_root + 2)` free at `!is_pv` gate nodes and
silent at `is_pv` ones, so at the same position a null-window scan proves a forced
loss while the PV re-search reports a pattern-table sum. The `is_pv` gate in
shipped code exists for one reason and the code says so: *"the PV must carry a
provable line, so generation proceeds"* (`staged.rs:86–89`,
`StagedRow::BatchedLost`). At a **capped** gate node no line is generated on
either branch — the alternative to the mate score is `self.position.value()` with
an empty PV slot (`pvs.rs:194` cleared it), which is what today's horizon already
returns. So the reason for the `is_pv` exception is absent exactly there.

Recommend: when the gate node's budget is exhausted and
`blocking_covers` answers `Impossible`, return `-mate_in(turns_from_root + 2)`
**regardless of `is_pv`**. Soundness is unaffected — `LAW-OVERLOAD`
(`threat_calculus_v1.md:55–59`) needs only "opponent `t ≥ 3`" plus "defender
cannot win this turn", the second of which trigger (a) has just established, and
`is_pv` is not one of its conditions. PV integrity is unaffected — the gate sits
at a turn boundary, so a line ending there is turn-whole and `turns_from_plies`
accepts it. Counter it if you disagree; but the asymmetry should be a decision,
not a side effect.

### §10 item 3 — the `LAW-RIPOSTE` → `LAW-FORCE` citation correction. **HALF-CONFIRMED.**
The substitution itself is correct (V5). The D-267 attribution attached to it is
wrong and the conclusion drawn from it does not follow — see C3, and C4 leg (c)
for what it costs.

### §10 items 1, 2, 4, 7. **All four survive attack.** See V1–V4.

---

## VERIFIED — the design's claims that survive attack

**V1 (§10 item 1) — §2.1's gap finding is CORRECT, re-verified at `9fa27c8`.**
`pvs.rs:199–218` is the `depth_plies == 0` branch: a `debug_assert!` on
`Phase::First` (the `STATIC_EVAL_MID_TURN` invariant) and
`return self.position.value()`. No threat query, no candidate generation, no
`staged_candidates` call, and — verified — the branch returns **before** the TT
probe at `:220–237` as well. The staged dispatch including step 1's win-now check
(`staged.rs:185–189`) is reached only from `pvs.rs:260–291`, inside
`depth_plies > 0`. Stage F does **not** already cover the horizon; the line
numbers cited are accurate. Under `CandidatePolicy::Radius` the gap is wider
still (no `ThreatState` at all — see C6).

**V2 (§10 item 2) — `any(MinimalCover::One)` is the correct `t ≤ 1` test;
`all(..)` would under-fire. CONFIRMED by counterexample against the shipped
enumeration.** Take two hot windows with empty families `{a, b}` and `{a, c}`.
`blocking_covers` at `HitBudget::Two` (`cover.rs:216–244`) pushes `One(a)`
because `covers(families, a, None)` holds (`:220–224`), and then pushes
`Two { first: b, second: c }` because `covers(families, b, Some(c))` holds while
neither `covers(.., b, None)` (misses `{a, c}`) nor `covers(.., c, None)`
(misses `{a, b}`) does (`:225–239`). So `Cover::Minimal` legitimately carries a
`One` **and** a `Two` simultaneously, `.all(One)` returns false, and the trigger
would miss a genuine `t = 1`. The design's `.any(..)` is right, and its stated
reason in §10 item 2 ("under-fires when a t=1 position also admits an unrelated
2-cell cover") is the correct reason. The classification is complete and
exhaustive over `Cover`'s three arms given `HitBudget::Two`: `NothingToBlock` ⇒
`t = 0` (`:203–205`), any `One` ⇒ `t = 1`, `Minimal` with no `One` ⇒ `t = 2`,
`Impossible` ⇒ `t ≥ 3` (`:240–242`), matching D-267's map entry exactly.
*(One recorded caveat, inherited from shipped code and not introduced here:
`Impossible` also answers for a family containing a completed window — an empty
`empty_families` entry can never be met by `covers` (`:309–313`) — which D-267
records for `min_hitting_set_exceeds`. Unreachable through `visit`, since
`place` returns `PlyOutcome::Win` and the parent never recurses into a decided
child.)*

**V3 (§10 item 4) — §4's `Phase::Second` correction is CORRECT and the
alternative really does panic. CONFIRMED by code path.** Suppressing the safety
net unconditionally leaves `set.cells` empty at a `Phase::Second` node; `pvs.rs`
`:285–288` then calls `no_candidates_at_a_turn_boundary()` and returns
`self.position.value()`. That function (`:479–486` — the design's line cite is
exact) is a release-active `assert!` on `phase() == Phase::First`, so it panics
with `NO_CANDIDATES_MID_TURN`, D-104's invariant. It would also breach D-111
independently (a static value as a node's answer mid-turn). The correction —
suppress at `Phase::First` gate decisions only — is the minimal fix: it changes
nothing about a granted turn's second stone, which `Position::place`'s legality
makes non-optional. §4's hedge is also correct: the panic requires a position
where Tier T empties out *between* a turn's two stones, and §4 properly hands
constructing it to RED-TEAM rather than asserting it exists. Recorded as a
correct hedge, not a gap.

**V4 (§10 item 7) — §5's zone claim holds structurally. CONFIRMED by
exhaustive call-site search at `9fa27c8`.** `within_radius` is `pub(crate)`
(`candidates.rs:59`) and has exactly two call sites in the workspace:
`candidates.rs:47–48` inside `candidate_cells`, reached only from `pvs.rs:247`'s
`CandidatePolicy::Radius` arm; and `staged.rs:283`, the safety net, which sets
`out.used_quiet_safety_net = true` on the same line (`:284`). There is no third
path and no other radius-based cell source in `pistol-search`. §4's suppression
rule keys on exactly that flag, so a gate decision cannot see a radius cell. The
design's refusal to call this `ZONE-R` (§5, second paragraph) is also correct
against D-267, which assigns `ZONE-R` to Stage 3.

**V5 — §3.4's citation substitution proper is CORRECT.** *"A defense must hit
every unanswerable plan"* is `LAW-FORCE` (`threat_calculus_v1.md:49–53`: *"If the
opponent has ≥1 plan and the mover cannot win this turn, every non-losing mover
move hits **all** opponent plans"*), not `LAW-RIPOSTE` (`:74–77`), whose two
sentences are a fact about forced defensive stones plus a soundness obligation on
*provers*. The design is right that the two IDs were swapped in the dispatch's
prose. See C3 for the part of §3.4 that does not survive.

**V6 — §9's rule-5 registration is landed verbatim and matches.** `D-388`
(`docs/decisions.md:832`) carries the four registered sentences byte-identical to
§9's block, landed in this document's own commit `9fa27c8`, before any bench.
Rule 5's pre-registration discipline is satisfied on its face. (C1 is a finding
against the *design*, not against the registration: the registered numbers must
not move, and nothing here asks them to.)

**V7 — smaller claims checked and correct.** §3.3's "Tier F is provably empty
here" reproduces `batched()`'s own argument (`staged.rs:262–266`) correctly.
§3.5's `!is_pv` / `is_pv` split matches `staged.rs:201–205` and `pvs.rs:277–279`,
including the `k + 2` distance. §8's mutual exclusivity of `q_extend_defense` and
`q_extend_offense` is right: a `Cover::Minimal` row takes the FILTERED branch and
never reaches `batched()`. §7's determinism argument is right: `staged_candidates`,
`blocking_covers` and `tier_t_union` all sort and dedup, and `q_budget` is plain
recursion state. §6's "no separate boolean flag" is consistent with hard rule 1.
The ROADMAP unblock claim (§0) checks out (`docs/ROADMAP.md:120`).

---

## Summary

| | count |
|---|---|
| CONFIRMED, blocking | 4 (C1, C2, C3, C4) |
| CONFIRMED, non-blocking | 2 (C5, C6) |
| Concerns | 5 (N1–N5) |
| Design claims attacked and VERIFIED | 7 (V1–V7) |

The two structural halves of the design — "no new generator, reuse
`staged_candidates`" (§4) and "zones are window-support, never radius" (§5) — are
sound and verified. The gap-finding (§2.1) is real and well-evidenced, and the
`MinimalCover` and `Phase::Second` readings are both right for the right reasons.
What must be fixed before IMPL is **what makes the extension fire** (§3.2/§3.3:
it fires on plan-free positions at a measured 61.5–92.5 % of nodes and declines to
fire on `t = 2`, which inverts both PROTO-NODE step 5 and LAW-LEDGER and leaves a
LAW-RIPOSTE hole at the flip-initiative case), **what the gate node writes to the
table** (§7's predicate does not cover it), and **the D-267 attribution** §3.4's
argument rests on.

*Report written by the REVIEW-design subagent against `9fa27c8`. Left
uncommitted for the orchestrating session.*
