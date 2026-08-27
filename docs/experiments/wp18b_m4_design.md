# WP-1.8b/M4 — the one-free-stone attacker-policy widening (design)

**Revision 2. Closes the DECISION-RED-TEAM's findings (MATRIX FALLS:
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
what selection stands on — hence this revision before selection.**

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
4. **Determinism**: the enumeration is order-fixed (raisers ascending,
   free cells ascending, canonical pair spelling, deduped against the v0
   arm by pair equality), no clock, no hash iteration (D-7).

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
step 1 before this enumeration, as in v0). Arm B's pairs qualify for the
DEF-PLAN filter through `c` alone — no per-pair filter check is needed on
arm B, which is what makes its cost `|R|·|L|` enumerations rather than
`|R|·|L|` ThreatState apply/undo probes.

**Cost shape (rule 5)**: per OR node, arm A costs what v0 cost; arm B
adds `|R|·|L|` pair constructions plus the same count of apply/undo
probes one layer later in `dfpn`'s child-key computation (the red
team's correction, adopted — the filter probe is avoided but the child
keys are not). **MEASURED over the 85 anchor positions** (scratch test
tree at `3547fe8`, recorded here, test deleted after the run): `|R|`
mean 5.2, max 20; `|L|` mean 480.1, max 620 — so arm B is ~2,500 pairs
mean, ~12,400 max, per OR node. L's magnitude agrees with wp18a §7c's
own 250-400-cell measurements. The df-pn search itself grows because OR
nodes carry more children. The pre-registered bracket is in §5.

## 3. Option matrix (each row: options, costs, failure modes, recommendation)

| # | decision | options | recommendation + strongest surviving attack |
|---|---|---|---|
| M4-1 | widened set shape | (a) raiser×free arm ∪ v0 arm, DEF-PLAN preserved by construction; (b) any pair with ≥1 cell in C, NO plan-creation filter; (c) unrestricted attacker (the true game) | **(a)**. BOTH (b) and (c) break the AND-node plan assertion (`NO_PLAN_ASSERT`: every OR move leaves ≥1 attacker plan, and the defender protocol's steps 2-4 assume `t ≥ 1`) — (c) a fortiori, since a fully unrestricted attacker admits plan-free pairs with no C cell at all — so neither is a cost trade but a redesign of the AND protocol itself, out of a one-round widening's scope. ((c) is not dismissed on cost: df-pn would still keep the AND-side shortcuts R3' lacks — LAW-OVERLOAD, the race check, the blocking filter — so it is NOT "the brute force R3' already is"; the recorded R3' intractability receipts (§9a) are R3'-only measurements. The assertion break is the dismissal, and it applies to both.) Strongest surviving attack on (a), RECORDED: *arm B admits pairs whose free stone creates a SECOND plan the AND node then also has to cover — sound, but it can make proofs DEEPER than v0's on positions where the extra plans widen the defender's cover choices; the widening is not guaranteed to prove every v0-provable win at the same depth.* Accepted: value (win/nowin) is monotone in attacker width — a game-tree fact, a strict strategy superset — every v0 win stays a win, while witness depth may grow. AND THE NoWin SIDE IS WHERE THE COST LANDS (the red team's stronger attack, adopted): a disproof must now refute every one of the |R|·|L| arm-B children, each a full AND subtree — multiplicative widening of DISPROOF trees is the shape the probe re-run's wall-caps will take. |
| M4-2 | free-stone range | (a) the full rule-5 legal region; (b) a bounded-radius knob | **(a)**. A bounded `f` would be a NEW attacker-side knob — `free_stone_radius` is the DEFENDER free-stone range, pinned to 8 by validation, and arm B does not interact with it at all (config.rs MEASURED) — and a shrunken attacker range causes INCOMPLETENESS (missed wins), never a false certificate, so the M1 attack does not transfer (the red team's category-error finding, adopted; M1's shrunken defender range was proof invalidation). The config law wants no unmeasured tunable, and cost is answered by the §5 bracket plus the per-call node cap in section 2's design. Strongest surviving attack: *cost — L is the region's full width and arm B is the hot path* (and note `dfpn`'s child-key computation applies every generated move, so arm B's pairs do incur |R|·|L| apply/undo probes one layer later — the §2 cost sentence is corrected accordingly). |
| M4-3 | configuration | (a) an `attacker_policy` enum in the solver config, committed value `"one_free_stone"`, `"both_stones_relevant"` selectable; (b) hard replacement, no knob | **(a)**. Gate (c)'s registered semantics and measured cost are v0's — it must keep running the narrow policy or its σ sweep (118,135 placements MEASURED, ROADMAP's D-437 entry is the claim's home; wp18a §9a m-7's 119,654/25,346 are the pre-dedupe sweep-leg numbers, D-437's are the closure re-run's) rides the widened solver with unmeasured cost; a knob is what lets (c) stay itself while the committed default moves. Also keeps A/B comparability for the probe re-run. The knob is a parser/schema growth (config.rs requires exactly five keys today, MEASURED), with no code-side default (rule 1). THE UNION'S ENUMERATION ORDER, stated once here because three implementations must agree: arm A's v0 order first, arm B appended after (raisers ascending, free cells ascending), deduplicated by canonical pair equality. The solver_oracle_tests' own one-committed-config doc comment is amended with the (c) split at impl. Strongest surviving attack: *two policies to keep correct at three sites*; the answer is the state-once rule — §2 is the spec, `policy.rs`, `r3.rs` and `r3_zone.rs` all point at it, and gate (a)'s differential is exactly the two-implementation agreement test. |
| M4-4 | the gates' story | (a) bounded expectations re-derived from the WIDENED R3' (the independent reference, not the solver), gate (c) pinned to the narrow policy by config, gates (a) (b) (d) green under the committed widened default; (b) re-register every gate under the widened policy including (c) | **(a) AS AMENDED BY THE RED TEAM — the re-derivation REFERENT for flipped cases is gate (b)'s verifier, not R3'.** R3' is independent of `policy.rs` (imports only pistol-core, its own window scan, no ThreatState — MEASURED in r3.rs) but is MEASURED intractable on any position whose solution contains an AND node (§9a B-2), and EVERY nowin→win flip is AND-containing by construction (a one-node win is policy-independent — `can_win_this_turn` precedes and is identical under both policies — so a flip requires an OrStep, hence an AND child). So the re-derivation runs R3' only on cases that remain one-node; every FLIP is adjudicated by the verifier's independent full-width re-proof (gate (b)'s own instrument), and its expectation line is re-registered from that verdict. AND THE HANG CONTINGENCY, registered before impl: gate (a) has no wall cap, so the impl first re-solves the bounded set under the widened policy with a probe-side timeout — if ANY bounded case stops being one-node, gate (a)'s differential is re-registered to exclude multi-node cases from the R3' leg (they move to gate (b)'s verifier leg, the §9a instrument split's own precedent) rather than risking an undetected hang on a required-green gate. The σ-sweep number's home is D-437 (118,135/26,865 at closure); §9a m-7's 119,654/25,346 are the pre-dedupe numbers, reconciled here. Strongest surviving attack: *the fixtures were authored to stress v0's shapes; a widened solver may make the bounded set trivially win-heavy, weakening gate (a)'s discriminating power*; accepted and recorded — the deep fixture and gate (b) carry the multi-node burden since WP-1.8a (§9a), and the re-run keeps R3' as the second implementation on every case it can still adjudicate. |
| M4-5 | probe re-run | (a) same registration, same caps (60 s wall per position), same collapse-turn reading; instrument rebuilt at the M4 revision and the registration's instrument table amended with that revision | **(a)**. The branch is already read (B); the re-run is licensed exactly once and does not re-adjudicate it. Its output is recorded as the widening's own measurement: does arm B prove winner-wins EARLIER than v0 did (v0: g1 t44, g2 t39), and at what node cost. **The wall-cap-on-a-winner-position reading, pinned before the run (the red team's MINOR, adopted): a winner-to-move position that wall-caps under the widened policy is recorded as NO ANSWER — neither "earlier" nor "not earlier" — and the EARLIER question is answered only over positions that returned.** |

## 4. Tests (the impl round's obligations)

- Arm B unit: a position where the winning pair is raiser + far free
  stone, v0-refuted (NoWin under `both_stones_relevant`) and
  M4-proven (Win under `one_free_stone`) — the widening's value in one
  test; plus a position where arm B's free stone creates a second plan
  and the proof still verifies.
- Dedup: a pair reachable from both arms appears once (count check over
  the enumerated set).
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

- **Hotspot**: OR-node move enumeration and the widened OR's effect on
  solve cost — the deep fixture's 9 decoys at the committed config,
  plus the bounded set (**ESTIMATED unchanged: all leaf-level today** —
  and load-bearing, so M4-4 registers the contingency: the impl first
  re-solves the bounded set probe-side; if any bounded case stops being
  one-node, gate (a)'s R3' leg is re-registered to exclude multi-node
  cases rather than risk an undetected hang on a required-green gate).
- **Bracket (ESTIMATED, from the MEASURED |R|·|L| arithmetic in §2 and
  v0's measured 40-634-node decoy solves)**: widened decoy solves ≤ 25x
  v0's node counts and inside the gates' registered wall caps; **abort**
  if any decoy exceeds 100x v0's nodes or its wall watchdog fires.
- **The probe re-run's cost** is bounded by its own registration (85 ×
  60 s); more wall-caps than v0's 35 are EXPECTED (stronger policy,
  deeper NoWin searches) and are a finding to record, not a failure.

## 6. What this design does NOT do

No solver-on-search-path wiring (section 2 owns it), no defender-policy
change, no zone-machinery change, no gate (c) re-registration, no second
widening (the dispatch forbids it), no strength claim. The widening's
strength question is section 5's SPRT, judged on the wired engine.
