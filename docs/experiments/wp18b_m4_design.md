# WP-1.8b/M4 — the one-free-stone attacker-policy widening (design)

**Revision 3. Closes REVIEW-design's one MAJOR and six minors (report
landed with this revision). M-1: the enumeration order and dedup are now
stated ONCE, in §2, with the intra-arm-B duplicate class named (`{c1,c2}`
with both cells raisers is emitted twice within arm B itself — dedup is
over the whole union, not just against arm A); §1 and M4-3 point at §2.
m-1: the bounded set's immobility is a THEOREM, not an estimate — one-node
NoWin implies `C = ∅` implies `R = ∅` implies arm B is empty, so no bounded
case can flip or stop being one-node; the probe-side re-solve stays as
cheap confirmation and the hang contingency is retired. m-2: the flip
adjudication's seat and refusal consequence are named. m-3: gate (c)'s
narrow pin is a named second committed config. m-4: the SolverParams /
Solver::new growth and its five call sites are named in §1. m-5: the §7c
citation corrected (that figure is an estimate; the anchor regions EXCEED
it, as late-game regions should). m-6: §6 records the unconsumed deep-NoWin
licence.**

**Revision 2 closed the DECISION-RED-TEAM's findings (MATRIX FALLS:
M4-1's option-(c) cell rested on a false equivalence with R3' — restated
on the assertion argument; M4-4's re-derivation referent was refuted by
the repo's own R3' intractability measurement on exactly the flip class
— every nowin→win flip is AND-containing by construction — so flipped
cases are adjudicated by gate (b)'s verifier, with a registered hang
contingency for gate (a); and the MINORs: the |R|·|L| cost claim is now
MEASURED over the 85 anchor positions, the `free_stone_radius` conflation
and the M1 category error are corrected, the union's enumeration order is
stated, the σ-sweep number's home is cited and reconciled, and the
wall-cap-on-a-winner-position reading is pinned). Every recommendation
survived; the matrix's REASONS did not all survive, and the reasons are
what selection stands on — hence revision 2 before selection.**

**The licensed widening the anchor probe's branch B activates**
(`docs/experiments/wp18b_anchor_probe.md` §8: the v0 policy proved the
winner's conversions only two turns after sealbot's own tree collapsed —
and proved the loser wins it missed; the policy is the bottleneck).
Licensed at WP-1.8a design §6 M4 option (b) and §9a ("the concrete
argument for the M4 one-free-stone widening"); commissioned here by the
WP-1.8b dispatch's branch B, verbatim: "ONE design round, ONE impl round,
both reviewed, gates (a) (b) (d) re-run green, then the probe re-runs
once. Whatever it says the second time, proceed to section 2. No second
widening."

## 1. What changes, and what may not

The policy game's ATTACKER step 2 (wp18a_design §2) is widened. v0: all
canonical pairs from `C` (both stones threat-relevant) that create ≥1 hot
window. **M4: additionally, all canonical pairs `{c, f}` where `c` ALONE
raises some attacker window to hot (a "raiser": after placing only `c`,
the attacker owns a hot window) and `f` is ANY empty cell of the rule-5
legal region.** The DEF-PLAN filter is preserved BY CONSTRUCTION on the
new arm (the raiser alone creates the plan, so the pair creates one), and
the v0 arm is kept verbatim — the widened set is a strict superset of v0's.

**Where the widening lands in code, named (m-4)**: `policy.rs` (the
enumeration), `r3.rs` and `r3_zone.rs` (the mirrors), `config.rs` (the
schema and its integer-only grammar grow a string-valued key) and
`SolverParams`/`Solver::new` (the knob rides both), whose five call sites
— solver-selftest, the wp18b_probe example, the oracle gates' config
loader, the config-validate example, and the lib tests — all pass it
through; a `Solver::new` signature change is what reopens the probe
registration's instrument table (M4-5).

**What may not change — the structural invariants the widening must not
touch:**

1. **The AND-node plan assertion** (`policy::NO_PLAN_ASSERT`): every OR
   move creates ≥1 attacker plan. This is why the unfiltered shape
   (M4-1 option b below) is not merely expensive but INCOMPATIBLE with
   the current AND node: steps 2-4 of the defender protocol assume `t ≥
   1`. The raiser-arm preserves the assertion by construction.
2. **Soundness direction**: the attacker stays restricted (the policy
   prunes `generate_turns`' rule-complete set, D-6/D-52 — never extends
   it); a `Win` under the wider policy is still a true rule-2 win; a
   `NoWin` under the wider policy is a STRONGER NoWin than v0's.
3. **The defender protocol is untouched** (steps 1-4 verbatim), as are
   zones (AT-1/DT-1/EP-1: the free stone's cells enter the zone through
   AT-1 like any attacker move cell), the TT, the epoch mechanism, and
   the witness/first-move shape section 2 will consume.
4. **Determinism**: the enumeration's ORDER and DEDUP are stated once,
   in §2 (this section points there and restates nothing). No clock, no
   hash iteration (D-7).

**The free stone is LEGAL BY RULE, not by policy**: `f` ranges over the
rule-5 legal region (the radius-8 union, `LEGAL_RADIUS` in pistol-core —
a named constant, never a knob here).

## 2. The enumeration, stated once (the mirrors point here)

The attacker's OR-node move set under `attacker_policy =
"one_free_stone"` is the deduplicated union of:

- **Arm A (v0, unchanged)**: pairs from `C` (both cells in `C`) that
  create ≥1 hot window — including the joint case where neither cell
  alone raises a window but both land in one live-two's empties.
- **Arm B (the widening)**: pairs `{c, f}`, `c ∈ R`, `f ∈ L ∖ {c}`, where
  `R` = the attacker's raiser cells (place `c` alone ⇒ ≥1 hot window —
  the `cells_raising_to_hot` class over live threes) and `L` = the empty
  cells of the rule-5 legal region.

No pair outside `generate_turns`' set appears (both cells are legal
placements; pairs are canonical; a pair completing six is absorbed by
step 1 before this enumeration, as in v0 — and `R ⊆ L`, so `f` is always
a distinct legal partner). Arm B's pairs qualify for the DEF-PLAN filter
through `c` alone — no per-pair filter check is needed on arm B, which is
what makes its cost `|R|·|L|` enumerations rather than `|R|·|L|`
ThreatState apply/undo filter probes (the child-key apply/undo cost one
layer later is real and counted in §2's cost paragraph).

**THE ORDER AND THE DEDUP, stated here and nowhere else** (M-1's fix;
`dfpn`'s child selection breaks ties by first-in-enumeration, so this is
a load-bearing decision all three implementations must share):

1. Arm A first, in v0's own order (candidates ascending, i<j canonical
   pairs, the existing filter).
2. Arm B appended after, raisers ascending, and for each raiser `c` the
   free cells `f` ascending; each pair spelled canonically by
   `Turn::pair`.
3. **Dedup over the WHOLE union, by canonical pair equality — and the
   duplicate class lives INSIDE arm B**: a pair `{c1, c2}` with both
   cells raisers is emitted twice within arm B itself (at `c=c1,f=c2`
   and `c=c2,f=c1`, because `R ⊆ L`), so dedup is not merely against
   arm A. The implementation keeps a seen-set (or an equivalent
   order-preserving dedup) and every canonical pair appears exactly
   once in the final enumeration.
4. The `dfpn.rs` tie-break doc comment ("least by canonical move order")
   is amended at impl to describe tie-breaking under this order — arm
   B's raiser-major order is not global canonical-pair order, and a doc
   comment that misdescribes the selection order is a finding waiting.

**Cost shape (rule 5)**: per OR node, arm A costs what v0 cost; arm B
adds `|R|·|L|` pair constructions plus the same count of apply/undo
probes one layer later in `dfpn`'s child-key computation (the red
team's correction, adopted — the filter probe is avoided but the child
keys are not). **MEASURED over the 85 anchor positions** (scratch test
tree at `3547fe8`, recorded here, test deleted after the run): `|R|`
mean 5.2, max 20; `|L|` mean 480.1, max 620 — so arm B is ~2,500 pairs
mean, ~12,400 max, per OR node. Late-game anchor regions EXCEED wp18a
§7c's 250-400-cell figure, which is an ESTIMATE there, not a measurement
— as late-game regions should (m-5's correction). The df-pn search
itself grows because OR nodes carry more children. The pre-registered
bracket is in §5.

## 3. Option matrix (each row: options, costs, failure modes, recommendation)

| # | decision | options | recommendation + strongest surviving attack |
|---|---|---|---|
| M4-1 | widened set shape | (a) raiser×free arm ∪ v0 arm, DEF-PLAN preserved by construction; (b) any pair with ≥1 cell in C, NO plan-creation filter; (c) unrestricted attacker (the true game) | **(a)**. BOTH (b) and (c) break the AND-node plan assertion (`NO_PLAN_ASSERT`: every OR move leaves ≥1 attacker plan, and the defender protocol's steps 2-4 assume `t ≥ 1`) — (c) a fortiori, since a fully unrestricted attacker admits plan-free pairs with no C cell at all — so neither is a cost trade but a redesign of the AND protocol itself, out of a one-round widening's scope. ((c) is not dismissed on cost: df-pn would still keep the AND-side shortcuts R3' lacks — LAW-OVERLOAD, the race check, the blocking filter — so it is NOT "the brute force R3' already is"; the recorded R3' intractability receipts (§9a) are R3'-only measurements. The assertion break is the dismissal, and it applies to both.) Strongest surviving attack on (a), RECORDED: *arm B admits pairs whose free stone creates a SECOND plan the AND node then also has to cover — sound, but it can make proofs DEEPER than v0's on positions where the extra plans widen the defender's cover choices; the widening is not guaranteed to prove every v0-provable win at the same depth.* Accepted: value (win/nowin) is monotone in attacker width — a game-tree fact, a strict strategy superset — every v0 win stays a win, while witness depth may grow. AND THE NoWin SIDE IS WHERE THE COST LANDS (the red team's stronger attack, adopted): a disproof must now refute every one of the |R|·|L| arm-B children, each a full AND subtree — multiplicative widening of DISPROOF trees is the shape the probe re-run's wall-caps will take. |
| M4-2 | free-stone range | (a) the full rule-5 legal region; (b) a bounded-radius knob | **(a)**. A bounded `f` would be a NEW attacker-side knob — `free_stone_radius` is the DEFENDER free-stone range, pinned to 8 by validation, and arm B does not interact with it at all (config.rs MEASURED) — and a shrunken attacker range causes INCOMPLETENESS (missed wins), never a false certificate, so the M1 attack does not transfer (the red team's category-error finding, adopted; M1's shrunken defender range was proof invalidation). The config law wants no unmeasured tunable, and cost is answered by the §5 bracket plus the per-call node cap in section 2's design. Strongest surviving attack: *cost — L is the region's full width and arm B is the hot path* (and note `dfpn`'s child-key computation applies every generated move, so arm B's pairs do incur |R|·|L| apply/undo probes one layer later — the §2 cost sentence is corrected accordingly). |
| M4-3 | configuration | (a) an `attacker_policy` enum in the solver config, committed value `"one_free_stone"`, `"both_stones_relevant"` selectable; (b) hard replacement, no knob | **(a)**. Gate (c)'s registered semantics and measured cost are v0's — it must keep running the narrow policy or its σ sweep (118,135 placements MEASURED, ROADMAP's D-437 entry is the claim's home; wp18a §9a m-7's 119,654/25,346 are the pre-dedupe sweep-leg numbers, D-437's are the closure re-run's) rides the widened solver with unmeasured cost; a knob is what lets (c) stay itself while the committed default moves. Also keeps A/B comparability for the probe re-run. The knob is a parser/schema growth (config.rs requires exactly five keys and an integer-only grammar today, MEASURED), with no code-side default (rule 1). **Gate (c)'s pin is a second committed config, `configs/solver_v0_narrow.toml`** (`both_stones_relevant`, complete per rule 1), selected by the oracle test's gate-(c) leg; the test's one-committed-config doc comment is amended with the split at impl, and the widened solver receiving NO gate-(c) coverage is a coverage reduction recorded in §6. Strongest surviving attack: *two policies to keep correct at three sites*; the answer is the state-once rule — §2 is the spec (order and dedup included), `policy.rs`, `r3.rs` and `r3_zone.rs` all point at it, and gate (a)'s differential is exactly the two-implementation agreement test. |
| M4-4 | the gates' story | (a) bounded expectations re-derived, gate (c) pinned narrow, gates (a) (b) (d) green under the committed widened default; (b) re-register every gate under the widened policy including (c) | **(a), with the flip machinery now THEOREM-GROUNDED (m-1) and seated (m-2). THE THEOREM: no bounded case can flip or stop being one-node — a one-node NoWin means `C = ∅` (any C-pair creates a hot window, so v0 had moves), which means `R = ∅` (R is the live-three empties, a subclass of C), which means arm B is EMPTY and the widened solver solves the same single node to the same value; a one-node Win is step 1's policy-independent leaf. The bounded set (61 cases, all one-node, §9a) is therefore immobile under the widening — the probe-side re-solve at impl is a cheap CONFIRMATION of the theorem, and gate (a)'s hang contingency is RETIRED (were observation to contradict the theorem, that is a FINDING about the theorem, not a contingency to route around). THE FLIP SEAT, for any fixture outside the theorem's cover (deep or future): a flip is detected by gate (a) (solver value ≠ registered expectation), adjudicated by a test-tree run of `common::verifier::verify()` — gate (b)'s instrument, invoked by the impl round with the verdict recorded in its receipt — and on `Verified` the expectation line is rewritten and the fixture re-pinned; on `Failed` the widened solver produced a false win, the round FAILS with gate (a) red, and the one fix round is spent on the solver, not the fixture. R3' remains the differential referent for every case it can still adjudicate (one-node cases; MEASURED intractable past those, §9a B-2, and every AND-containing flip is outside it by construction). The σ-sweep number's home is D-437 (118,135/26,865 at closure); §9a m-7's 119,654/25,346 are the pre-dedupe numbers, reconciled here. Strongest surviving attack: *the fixtures were authored to stress v0's shapes; a widened solver may make the bounded set trivially win-heavy, weakening gate (a)'s discriminating power* — dissolved by the theorem (the set cannot change value at all); what stays true is that the bounded set keeps exercising only the leaf, and the deep fixture and gate (b) carry the multi-node burden since WP-1.8a (§9a). |
| M4-5 | probe re-run | (a) same registration, same caps (60 s wall per position), same collapse-turn reading; instrument rebuilt at the M4 revision and the registration's instrument table amended with that revision | **(a)**. The branch is already read (B); the re-run is licensed exactly once and does not re-adjudicate it. Its output is recorded as the widening's own measurement: does arm B prove winner-wins EARLIER than v0 did (v0: g1 t44, g2 t39), and at what node cost. **The wall-cap-on-a-winner-position reading, pinned before the run (the red team's MINOR, adopted): a winner-to-move position that wall-caps under the widened policy is recorded as NO ANSWER — neither "earlier" nor "not earlier" — and the EARLIER question is answered only over positions that returned.** The instrument-table amendment is an amendment to the probe registration under CLAUDE.md's instrument rule; that registration is diagnostic and dispatch-licensed (it has never had a subagent review to reopen), and the amendment says so on its face. The re-run's DRIVER may shard the 85 independent positions across workers — each position is its own process with its own `timeout(1)` cap, so sharding changes wall time only and every recorded number is identical; the amendment records the sharded form. |

## 4. Tests (the impl round's obligations)

- Arm B unit: a position where the winning pair is raiser + far free
  stone, v0-refuted (NoWin under `both_stones_relevant`) and
  M4-proven (Win under `one_free_stone`) — the widening's value in one
  test; plus a position where arm B's free stone creates a second plan
  and the proof still verifies.
- Dedup: ANY canonical pair appears exactly once in the enumeration —
  including a pair emitted TWICE WITHIN arm B itself (two raisers `{c1,
  c2}`: once at `c=c1,f=c2`, once at `c=c2,f=c1`) and a pair reachable
  from both arms (m-1's extension: the count check is over the whole
  union, not just the cross-arm class).
- Legality: every arm B pair passes `Board::is_legal_placement` on both
  cells and is canonical.
- R3'/r3_zone mirrors: the widened differential holds on the full
  fixture (gate (a)); the verifier re-proves arm-B wins (gate (b)).
- Determinism: two runs of the widened selftest byte-identical (the
  solver determinism seat re-runs unchanged — it uses the committed
  config, now widened).
- Mutations: drop the arm-A union (v0 pairs vanish) → the arm-B-value
  unit test dies only if the chosen position also needs arm A (pick one
  that needs both arms' union... the honest form: drop arm B → the
  widening-value test dies; corrupt the dedup (duplicate pairs) → the
  count check dies; skip the raiser-order sort → the determinism
  differential dies.

## 5. The rule-5 bench bracket, registered before measuring

- **Hotspot**: the widened OR's effect on solve cost — the deep
  fixture's 9 decoys at the committed (widened) config.
- **The bounded set**: THE THEOREM (M4-4) says every case stays one-node
  with an unchanged value — the impl's probe-side re-solve CONFIRMS this
  in seconds; no bracket applies to a set that cannot move. (The
  theorem, not an estimate, is what §9a's "expected unchanged" now
  rests on.)
- **Bracket (ESTIMATED, from the MEASURED |R|·|L| arithmetic in §2 and
  v0's measured 40-634-node decoy solves)**: widened decoy solves ≤ 25x
  v0's node counts and inside the gates' registered wall caps; **abort**
  if any decoy exceeds 100x v0's nodes or its wall watchdog fires.
- **The probe re-run's cost** is bounded by its own registration (85 ×
  60 s); more wall-caps than v0's 35 are EXPECTED (stronger policy,
  deeper NoWin searches) and are a finding to record, not a failure.

## 6. What this design does NOT do

No solver-on-search-path wiring (section 2 owns it), no defender-policy
change, no zone-machinery change, no gate (c) re-registration — and the
widened solver receives NO gate-(c) coverage in this round (the σ sweep
stays narrow-policied; (c) is measured tautological on the bounded set,
§9a, so the reduction is recorded here rather than argued away) — no
second widening (the dispatch forbids it), no strength claim. **The
§9a deep-NoWin adjudication licence is NOT consumed by this round**: deep
NoWin is strictly harder under M4 (the multiplicative disproof widening
is this design's own recorded attack), and adjudicating it stays beyond
the v0 instrument complex exactly as §9a licensed-not-scheduled it. The
widening's strength question is section 5's SPRT, judged on the wired
engine.
