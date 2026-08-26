# WP-1.8a design — relevance zones + df-pn in pistol-solver, correctness only

Revision 3. Governing dispatch: `[GROUNDWORK] WP-1.8a` (operator). D-436 governs
the GHI retirement; D-434/D-435 landed with this WP's opening commit and bind
nothing here beyond the teacher question they settle.

Revision 1 (`6f13d0c`) FAILED its REVIEW-design (report at
`docs/experiments/wp18a_design_REVIEW.md`): 1 BLOCKING, 8 MAJOR, 6 MINOR —
closed in revision 2 (`34c6838`), which the review's own words overturn in one
place: v0 no longer prunes the defender's free stone by a zone, because the
radius-4 zone certified false wins and overflowed legitimate wins; the zone is
now the derived, independently re-verified artifact, and the pruning knob is
carried at its no-prune value.

Revision 2 FAILED its scoped re-review (report at
`docs/experiments/wp18a_design_REVIEW_rev2.md`): 12 of 15 rev-1 findings
closed; 1 new BLOCKING (gate (c)'s P+σ replay mis-specified — the "turn set is
a subset of P's" claim is false under rule 5, σ stones extend the legal
region), 2 MAJOR (verifier defender-no-win at expanded AND nodes; dry-run
criteria without externally-derived referents), 4 MINOR. All closed in
revision 3 (`6dcd3ae`), whose two-leg gate (c) the round-3 review verified
sound in structure.

Revision 3 FAILED its third scoped review by ONE MAJOR plus eight one-edit
MINORs — every round-2 finding closed, no false certificate reachable, and
the MAJOR sat in one word of the registered σ class ("legal region" admits
occupied cells; root stones are never zone cells, so the literal class
included every root stone of every fixture). All nine are closed in this
revision 4; the σ class now reads `legal_placements(P)`, the σ-invariant is
restated at the strength its proof carries (at most k−1 of k cells), the σ
tuple order is pinned, the (c2)/(b) cost figures carry their own registered
caps,
the dry-run criteria are scoped to the pinned revision, and (c2)'s
independence note now states the fact that carries it (the v0 value path
never consults zones).

## 0. Premises, corrected against the live tree

Two premises of the dispatch do not hold and are corrected here rather than
silently worked around:

1. **"the existing AND-OR threat solver" does not exist.** `pistol-solver`
   contains `ThreatState` + queries + cover arithmetic only; no search of any
   kind. WP-1.8a therefore BUILDS the AND-OR solver. This matches the ROADMAP
   ("WP-1.8 — AND-OR solver, upgraded to relevance-zone Deep df-pn"), whose
   WP-1.5a cut put only the threat generator in the crate.
2. **"research report B (RZ / df-pn / GHI, filed in docs/research/)" is not
   filed.** `docs/research/` holds four files, none of them report B. The
   primary sources are used instead, both obtained and read this session:
   - Pawlewicz & Lew, *Improving Depth-first PN-Search: 1 + ε Trick*,
     CG 2006, pp. 160-171 (`mimuw.edu.pl/~pan/papers/lm-pns.pdf`), the source
     the dispatch's "Pawlewicz-Lew threshold formulas" name.
   - Wu & Lin, *Relevance-Zone-Oriented Proof Search for Connect6*, IEEE
     TCIAIG 2(3), 2010, pp. 191-207 (operator-provided PDF), the source of
     AT-1/DT-1/EP-1/T1/T2/T3 and of Definition 3 (the relevance property).
   The formulas below are quoted from the papers, not from a report. If report
   B lands later and disagrees, that is an amendment reopening this design.

Everything else in the dispatch's premise set checked out: D-8 (128-bit solver
key), D-9 (phase bit), D-6/D-52 (unordered canonical pairs, rule-complete
generation), D-346/D-424 process rules, the calculus IDs cited below.

## 1. What the solver decides

A position `P` (attacker to move, `Phase::First`, `stones_owed == 2`, game
ongoing — asserted, not coerced: a wrong-kind position is a named error, rule 3)
is solved for the value of the **policy game**: the attacker wins iff it can
force a rule-2 win playing only the §2 attacker policy, against every defender
turn. Restricting the attacker makes the value a *lower* bound on the true game
value (sound, incomplete); the defender is NOT restricted — v0 enumerates the
defender's full legal turn set filtered only by the §2 blocking predicate. The
value is `Win` or `NoWin` — df-pn is run to completion, no node budget; the
solver is not on the search path and makes no strength claim. Turns, not plies,
are the depth unit throughout (rule 4). All three implementations below apply
moves through `GameState::place`, so rules 1-5 (legality, win detection, sudden
death) are pistol-core's and re-implemented nowhere (rule 2).

## 2. Move policy

**Attacker (OR node), in order:**
1. Win this turn: `ThreatState::can_win_this_turn(attacker, StonesLeft::Two)`
   — a leaf `Win` (rule 4: the completing stone ends the turn).
2. Otherwise, threat pairs: all canonical pairs `{a, b}`, both cells in `C`,
   where `C` = empty cells of the attacker's live windows with own ≥ 2
   (LAW-SUPPORT k=2, the same qualification Tier T uses), such that after
   placing both, the attacker owns ≥ 1 hot window (DEF-PLAN). `C`-cells all lie
   in windows that hold attacker stones, so every `C`-pair is a legal turn
   under rule 5 without partner-reachability; the win check in 1 has already
   absorbed every pair that completes six, so no policy pair is missing from
   `generate_turns`' set (D-6/D-52: the policy prunes the rule-complete set,
   never extends it).
3. No policy move and no win: leaf `NoWin`.

The v0 policy has NO free attacker stone (both stones must be threat-relevant).
That is the deliberate narrowing M4 records; the one-free-stone widening is
licensed-not-scheduled.

**Defender (AND node), solver-side enumeration, in order:**
1. Defender wins this turn (`can_win_this_turn(defender, Two)`): the node is
   `NoWin` — rule 4's race is decided by the side to move and the defender
   moves now.
2. Attacker plans = attacker hot windows (DEF-PLAN); exact minimum hitting set
   size `t` over their empty cells (RULE-EXACT). `t ≥ 1` always here: the
   attacker's last move created a hot window by §2.2 — asserted, fail loud if
   false.
3. `t ≥ 3` and step 1 false: the node is `Win` without expansion — LAW-OVERLOAD
   (two defender stones cannot hit three).
4. `t ∈ {1, 2}`: the children are `generate_turns(state)` filtered to pairs
   forming a COVER of the plan family — a hitting set of the plans'
   empty-cell sets: at least one pair stone in every plan's empty set
   (re-review MINOR G; every such pair contains an
   inclusion-minimal cover; `blocking_covers(defender, Two)` exists as the fast
   path and the per-pair predicate as the specification — the filter and the
   cover arithmetic must agree, and a unit test pins that they do). The
   defender's second stone is the LAW-RIPOSTE danger; enumerating it over the
   full legal region is the seminull mechanism of RZOP's T1-1c, adapted: the
   paper derives the second stone's range from the sub-proof's zones, v0 ranges
   it over the whole legal turn set (marked deviation; the paper-faithful
   derived-range version is what the `free_stone_radius` knob licenses later).

**Every defender-side step above is a SOLVER-ONLY shortcut.** The reference
(§7a) and the verifier (§7b) implement none of them: they enumerate every legal
defender turn. Steps 1, 3 and the blocking filter are therefore falsifiable by
gate (a), which is the point of building them that way (review MAJOR 2).

## 3. Zones

**The proof zone `Z_p`** is a sequence `Z_1 ⊆ Z_2 ⊆ Z_3` (`zone_orders` config,
registered start 3, ESTIMATED — ZONE-R/ADOPT-RZOP: "order ≤ 3 suffices for
two-stone moves", square-board), constructed per Wu & Lin §IV:

- **AT-1** (OR node, proven by move m): `Z_p(node) = Z_p(child) ∪ cells(m)`,
  at EVERY order `Z_i` (the attacker's own move cells enter all orders, so no
  tolerated defender stone can ever sit on one).
- **DT-1** (AND node, proven): `Z_p(node) = ⋃` over ALL legal defender pairs of
  `Z_p(child(pair))`, elementwise, plus the EP-1/T3-1 contributions below. A
  non-blocking pair's child is the plan completion (§2.4's classification): its
  contribution is the completion's cells — the least surviving plan by
  `(axis, start)`, its empty cells, the same deterministic rule in both the
  solver and the verifier (this is a zone-cell bookkeeping choice, not a move
  choice; no strength implication).
- **EP-1** (every proof node): for each **defender active segment** — a 6-window
  with no attacker stones, ANY defender count including zero (Wu & Lin §III-A:
  "A segment is called an active segment of one player, if none of the squares
  are occupied by the opponent's stones") — with exactly `k` unoccupied
  squares: those squares enter `Z_k..Z_3`, i.e. `k = 1` → `Z_1..Z_3`, `k = 2`
  → `Z_2..Z_3`, `k ≥ 3` → `Z_3` (the paper's order-3 cap: "we construct zones
  with size three, and simply use [Z_3] for those higher order zones"). This
  is the paper-faithful grading revision 1 narrowed to hot windows, which the
  review's BLOCKING 1 showed certified false wins (a defender live-one's
  extension cells are order-3 relevant; leaving them in no zone put them in the
  tolerance class).
- **T3-1**: the attacker's hot-window empties enter ALL of `Z_1..Z_3`.

**One marked deviation from the paper.** The paper's boards are finite; ours is
unbounded. The active-segment scan at a proof node is bounded to windows
intersecting that node's own legal region (the radius-8 union around the
node's stones — the R2 region-scan pattern). Grounds: cells beyond the legal
region are unreachable by any legal stone at that node (rule 5), and the scan
re-runs at every node as the board grows, so a defender build marching outward
enters the scan one step behind the march, which is exactly when it becomes
legally reachable. The paper's zone machinery is dynamic (promotion, shifting,
Lemma 3) in ways this static per-node construction does not reproduce; **no
inheritance from the paper's Theorem 2 is claimed** — the property our
construction actually has is whatever gate (c) falsifies or confirms on the
fixture class, and nothing beyond it (review MAJOR 5(iv)).

**Storage and the certificate.** The solver computes `Z_p` incrementally on
its proof DAG as nodes are proven and stores it in the TT entry (memoising
zone construction across transpositions; proven entries carry zones). The
certificate is the verifier's (§7b) from-scratch `Z_p` over the reconstructed
witness tree, and gate (b) asserts the solver's memoised zones and the
verifier's agree — two independent constructions of the same specification
(review MAJOR 9(a): one certificate, the verifier's; the TT copy is a
cross-check, never the authority). A fail-loud invariant, not a soundness
claim: every cell of the verifier's `Z_p` must lie within the legal region of
some proof node grown by 5 (a window's reach past its last stone). By
construction it cannot fail; if it does, the zone construction has a defect
and the solve refuses (review BLOCKING 1's blast radius: v0 has no zone-based
pruning, so this invariant is the only thing the zone can still get wrong
loudly).

**No zone pruning in v0.** The defender's second stone ranges over the full
legal turn set. The `free_stone_radius` knob (registered start 8 = the full
legal region = no prune) exists so a later WP can shrink it; any value below 8
makes the solver report `NoWinUnderZone` whenever the proof's zone exceeds the
radius-`r` balls around the root stones — a loud outcome, never a silent
false win (the review's MAJOR 8(b) showed radius 4 overflows legitimate
2-turn extension chains; that arithmetic is why v0 does not prune).

## 4. df-pn

Nagai's df-pn with the Pawlewicz-Lew thresholds and 1+ε. Quoted verbatim from
the paper (§2.2): for an OR node's first child (children ordered `p1 ≤ p2 ≤ …`):

> pt1 = min(pt, p2 + 1), dt1 = dt − d + d1.

For an AND node's first child (`d1 ≤ d2 ≤ …`):

> pt1 = pt − p + p1, dt1 = min(dt, d2 + 1).

Root thresholds (§2.2): "For the root we set the thresholds to +∞." Leaf
numbers (§2.1): proved `0/+∞`, disproved `+∞/0`, unsolved leaf `1/1`; TT miss
initialises `1/1` (§2.2). Node numbers (Fig. 1): OR takes min of child PNs and
sum of child DNs; AND the dual. The 1+ε form (§3.2), quoted verbatim:

> we change the constraint to pt1 ≤ ⌈p2 (1 + ε)⌉ and the new formula for the
> child's PN threshold in an OR node is pt1 = min(pt, ⌈p2 (1 + ε)⌉).

The AND/DN form `dt1 = min(dt, ⌈d2 (1 + ε)⌉)` is the paper's own §2.2 symmetry
applied to §3.2 — marked DERIVED, not quoted. ε multiplies THRESHOLDS ONLY;
stored pn/dn never see it. ε = 1/4 is the registered starting value (paper
§4.1: "ε was set empirically to 1/4", Atari-Go/LOA numbers, ESTIMATED,
non-transferable), a config knob as an exact rational (numerator/denominator,
u32, `ceil` in u128 arithmetic).

**The loop semantics the quotes do not state, pinned (review MAJOR 3):** the
paper's derivation is for the first child at one visit; a df-pn node LOOPS. At
each iteration the parent (i) re-derives `p`/`d` from current child numbers,
(ii) terminates if `p ≥ pt` or `d ≥ dt`, (iii) re-selects the minimum child
(least by canonical move order on ties — deterministic), (iv) recomputes
`p2`/`d2` against the CURRENT ordering, and (v) descends with the quoted
formulas evaluated against current values. The formulas are applied per
descent, never cached from a previous iteration.

**INF sentinel**: `INF = 1 << 62` (u64). All pn/dn/threshold arithmetic is
saturating: `INF + x = INF`, `INF + INF = INF`, `min`/`max` saturate at INF,
no operation can wrap (u64::MAX is unreachable by construction; the ε multiply
is done in u128 and clamped to INF). No negative values exist (unsigned).
**Subtraction, the one operation that can underflow (review MAJOR 4):** the
subtractions `dt − d` and `pt − p` occur only inside the threshold formulas,
which are evaluated only in step (v) for the selected child, only after step
(ii) confirmed `p < pt ∧ d < dt` at the parent — so `dt − d ≥ 1` and the
result is ≥ `d1 ≥ 0`. The order evaluate → terminate-check → select →
threshold is pinned by a unit test that asserts no underflow path exists
(debug builds panic on overflow; the release build is checked by the same
test).

**Solver TT.** Key: pistol-core's full 128-bit key (D-8) — side-to-move and
intra-turn phase bit included (D-9) — so half-move positions key correctly and
no GHI machinery is needed (D-436: the game is monotone, no repetition, the
state graph is a DAG; the review's MINOR 1 mis-citation is corrected here —
D-36 is the win-detection API decision and says nothing about this). Entry:
`{pn, dn, value ∈ {Proven, Disproven, Unknown}, zone: Option<ZoneP>,
generation}`. Two-level bucket (Breuker's TwoBig, the scheme the paper itself
used): slot pair per index — generation-preferred + always-replace; PROVEN
entries are never replaced by unproven ones (dispatch). Replacement is by key
hash into a fixed array — deterministic, no hasher iteration on any choice
path (D-7).

**Proof-tree reconstruction.** The solver records its witness move per node
into a proof DAG owned by the solve (separate from the TT; eviction cannot
lose it — review MAJOR 9(b)). After a root `Win`, the claimed tree is emitted
by walking the proof DAG: OR nodes emit the recorded witness move; AND nodes
emit the §2.4 child set (re-derived deterministically). Gate (b)'s verifier
then re-proves that tree full-width and independently (§7b).

**Seesaw counter.** One counter per solve. A seesaw event: a df-pn recursive
call returns on a threshold miss, and the parent's next descent selects a
different child. Printed per solve, and aggregated over the fixture set at
the gates — a to-be-measured number with no threshold
registered — that number is WP-1.8c's licence input.

## 5. Node accounting (the decision the dispatch asks for)

**Separate registered budget.** Solver nodes never count against the search's
per-side node budget; a `solver_nodes` budget is its own registered quantity,
and the instrument prints BOTH (`nodes` and `solver_nodes`). Grounds: every
Stage-1 strength claim is node-matched (`nodes 50000` is the matched axis,
D-374's never-move rule); a solver firing inside that budget would silently
unmatch the seats exactly where the comparison is made. Recorded as an ADR line
at closure; the print seam lands with 1.8b's wiring.

## 6. Option matrix (each row: options, costs, failure modes, recommendation)

| # | decision | options | recommendation + strongest surviving attack |
|---|---|---|---|
| M1 | defender second-stone range | (a) full legal turn set, no zone prune; (b) fixed root radius `Z_s`; (c) RZOP dynamic seminull sets | **(a)** in v0, knob `free_stone_radius = 8` carrying (b) as licensed-not-scheduled; (c) needs the strategy-first verifier architecture, incompatible with df-pn search. Strongest surviving attack on (b)/(c), recorded after the review broke rev 1's answer: *sub-hot defender builds outside a shrunken range invalidate the found proof, and the `Z_p ⊆ Z_s` check sees them only if EP-1 is paper-faithful — with rev 2's EP-1 it does see them, but then a shrunken range converts legitimate outward-walking wins into `NoWinUnderZone` at radius scales the rev-1 arithmetic already refuted (ESTIMATED: a live-two's far empty sits at distance 4, two chained extensions reach 8)*. The knob's trade is certification rate for speed, v0 takes the no-prune end, and any shrinking is a future pre-registration. |
| M2 | zone representation | (a) sequence `Z_1⊆Z_2⊆Z_3`; (b) flat set | **(a)**. EP-1's grading is per-order and gate (c)'s tolerance class is order-structured (Definition 3); a flat set collapses the tolerance claim to the weakest order. Attack: three small sorted vecs per TT entry — an ESTIMATED few-hundred-bytes cost, measured at impl. |
| M3 | node accounting | (a) inside per-side budget; (b) separate, both printed | **(b)**, §5. Attack: two knobs to configure per seat — real, and cheaper than a silently unmatched SPRT. |
| M4 | attacker policy width v0 | (a) both stones threat-relevant; (b) one free stone | **(a)**. (b) multiplies OR branching by the legal region and is a width claim needing its own gate story. Attack: (a) proves strictly less — recorded as the licensed-not-scheduled widening, not hidden. |
| M5 | harness home | (a) bin target inside pistol-solver; (b) bin in pistol-cli; (c) tests only | **(a)**. (b) creates the normal reverse edge WP-1.5a's `p = 0` claim adjudicates against; (c) cannot give the determinism gate a two-process seat. The strongest attack ("a bin inside the crate is still a binary in this workspace on a literal reading") is answered BEFORE selection, by receipt: the shipped `tools/solver_edge_check.sh` was driven against a scratch workspace containing exactly this shape — bin target inside the solver crate, `--locked` graph — and printed `NO normal reverse-dependency on pistol-solver anywhere in the workspace`, exit 0; the control run (a normal `pistol-solver` dependency added to a sibling crate) printed the edge and exited 1. The adjudicator of `p = 0` answers "no edge" to this shape. |

## 7. Oracles — the gate of this WP

Fixture: `crates/pistol-solver/tests/fixtures/solver_v0.txt`, sha-pinned,
machine-checkable loader (the `pattern_v0.txt` discipline: unknown line or
missing expectation = panic). **Bounded means, as numbers**: ≤ 10 stones; proof
depth ≤ 2 turns for the bulk of the differential set, with ≥ 6 positions at
depth 3-4; the depth cap is a POST-RUN check — a fixture whose proof runs
deeper than its registered band FAILS the gate, it is not excluded (review
MINOR 4). Sample size: ≥ 60 positions — ≥ 20 `Win`, of which ≥ 8 walk outward
(their zone reaches ≥ 2 cells beyond the initial live-window cluster) and ≥ 8
carry a defender live-three or better near the action (riposte-stressing: the
positions where a naive free-stone treatment goes wrong); ≥ 20 `NoWin`; ≥ 20
shallow (≤ 1 turn) wins and refutations pinning rules 2/4 at the leaves
(review MINOR 3).

**R3' — the brute-force reference** (test tree only, alongside R1/R2/R3): a
memoised AND-OR with short-circuiting (value-preserving: OR stops at the first
`Win` child, AND at the first `NoWin`), written against `Board` with its own
window scans — sharing nothing with the solver but pistol-core: no df-pn, no
thresholds, no TT, no zones, and **no §2 defender-side shortcuts** — every
legal defender turn from `generate_turns` is applied and the child evaluated;
step-1/step-3/blocking classifications are never used (review MAJOR 2). The
attacker side stays policy-restricted: that restriction IS the game being
solved, implemented independently (its own live-window scan, not
`ThreatState`).

- **(a) Differential**: for every fixture position, solver value == R3' value.
  A `NoWinUnderZone` from the solver is a MISMATCH and fails the gate (it is
  unreachable in v0 by construction — `free_stone_radius = 8` prunes nothing —
  so it can only mean a defect; review MINOR 2). Because R3' enumerates every
  defender turn and the solver prunes to blocking pairs with LAW-OVERLOAD and
  race shortcuts, this gate adjudicates the df-pn machinery AND every
  defender-side lemma.
- **(b) Proof-tree re-verification**: an independent full-width verifier
  re-proves every `Win` from the emitted claimed tree: every attacker node's
  move is regenerated by the verifier's own policy code; every defender node
  applies EVERY legal turn through `GameState::place` and a defender-win
  `PlyOutcome` is a verification FAILURE at every AND node, not only at
  `t ≥ 3` shortcut nodes (re-review MAJOR B) — a tree edge recurses, a
  non-edge must concretely resolve as an attacker win (apply the least
  surviving plan's completion, rule-2 via pistol-core); every leaf is a rule-2
  win via pistol-core; every `t ≥ 3` shortcut node is re-derived from the
  verifier's own hitting-set enumeration plus a concrete defender-no-win
  check (review MAJOR 9(c)). The verifier computes `Z_p` from scratch over
  the witness tree; gate (b) also asserts solver-memoised `Z_p` == verifier
  `Z_p`. **Cost, priced (round-3 review MINOR N-8)**: applying every legal
  turn at every AND node is on the order of 10⁵-10⁶ `place`+win-check
  applications per AND node across the tree, ESTIMATED — terminating by
  construction (finite tree × finite turns), but slow rather than hung; gate
  (b) runs detached under its own 30-minute wall cap, with exceedance the
  named failure `VERIFIER-OVERRUN` (distinct from (c2)'s 60-minute cap —
  the two legs are separate wall budgets).
- **(c) RZ property, two legs** (re-review BLOCKING A re-specified; σ class
  narrowed to LEGAL pre-placements — cells of `P`'s own legal region — the
  natural game reading of "place defender stones", and a bounded sample):

  **(c1) Sequence replay.** For every sampled σ: walk the proof tree in
  `P+σ`. At attacker nodes the tree's move is played and must (i) be legal
  and (ii) still create its hot window — both checked concretely; AT-1 puts
  attacker move cells in every zone order, so σ can never occupy one, and
  every empty cell of every window a proof move activates is in `Z_1` (the
  move cells by AT-1, the future plan cells by T3-1 at the child), so σ
  cannot kill the activation either — but the replay CHECKS rather than
  assumes. At defender nodes the tree's pairs are applied via
  `GameState::place`: a defender-win outcome is a named failure
  (`DEFENDER-WIN-UNDER-SIGMA`); a tree pair made illegal by σ (σ on a
  free-stone cell) is SKIPPED and counted — its line is adjudicated by (c2),
  not by this walk. Leaves are core-checked rule-2 wins.

  **(c2) Property evaluation.** For every sampled σ: the SOLVER's value on
  `P+σ` must be `Win` — the paper's Definition 3 property ("the attacker
  wins in `P+σ` for all irrelevant σ"), with the defender playing
  adversarially through the solver's own full enumeration on the `P+σ`
  board, whose legal region σ stones EXTEND (rule 5 unions around every
  existing stone, σ's included — the re-review's superset correction: the
  defender's `P+σ` turn set is a SUPERSET of `P`'s, and (c2) is the leg that
  adjudicates the additions, which (c1) cannot). Independence note: (c2)
  uses the solver as its win-oracle, and the fact that actually carries it
  is stronger than vouching — at the registered `free_stone_radius = 8` the
  solver's VALUE PATH NEVER CONSULTS ZONES (no pruning happens), so the
  certificate under test and the oracle cannot contaminate each other; a
  too-small `Z_p` therefore lands as `NoWin`, the right direction. This
  independence is CONDITIONAL on the no-prune value: the licensed future
  shrinking of the knob would erode it, and that erosion is part of what a
  future pre-registration must re-argue.

  **The σ class and sample, registered**: σ's i-th stone lies outside `Z_i`
  (all i), and all σ cells lie in the EMPTY cells of `P`'s legal region
  (`legal_placements(P)`, region less occupied cells — round-3 review
  MAJOR N-1: the region itself holds stones, and root stones are never
  zone cells, so the wider reading admitted every root stone of every
  fixture). A σ is held as a tuple of cells ascending by `(q, r)`; its
  i-th stone is the i-th element of that tuple; the sample enumerates
  ascending tuples in lexicographic order. Sample: |σ| = 1 — ALL such
  cells; |σ| = 2 — all such tuples, capped at the first 5 000; |σ| = 3 —
  all such tuples, capped at the first 2 000. One structural invariant,
  stated because it is the paper's grading working: σ alone can never
  complete a defender six — a k-empty active segment's empties lie in
  `Z_k..`, σ_i can sit in `Z_k` only for i < k, so at most k − 1 of the k
  cells can carry σ stones (k = 1: zero of one; k = 2: one of two; k = 3:
  two of three; k ≥ 4: |σ| ≤ 3 < k), and no k is ever filled.

  **Cost, priced (re-review MINOR F)**: (c1) is a tree walk —
  O(proof-tree size) per σ, µs-scale; (c2) is one solver call per σ.
  Registered caps, stated before the run: (c2) runs on ALL `Win` fixtures
  under a 60-minute wall cap for the whole gate leg (detached execution);
  if the cap is exceeded the gate FAILS with `SIGMA-SAMPLE-OVERRUN` — the
  sample does not shrink post-hoc. Expected order of magnitude, ESTIMATED:
  ≈ 7 300 solver calls per fixture when the caps bind (~300 singles +
  5 000 pairs + 2 000 triples on a ~250-400-cell region), ≥ ~145 000 over
  ≥ 20 `Win` fixtures, each call a small solve; the dry run MEASURES the
  real figure (and its stand-in measures the near-best-case σ-solve — the
  depth-3-4 cost driver is measured by the gate's own first run, not the
  dry run's) and both are recorded in §10.
- **(d) TT cross-check**: full TT vs a 32-entry TT, identical VALUES on the
  fixture set (node counts and seesaw may differ; values may not). The 32-entry
  leg runs under a node cap of 50× the full-TT node count (registered);
  exceeding it is a named failure `TT-NONTERMINATION`, never a hang (review
  MAJOR 6). Runs on the differential fixture set only.

Exact commands (release, as the tactical gate precedent dictates):

```
cargo test --release -p pistol-solver --test solver_oracle_tests
cargo run --release -p pistol-solver --bin solver-selftest -- \
    crates/pistol-solver/tests/fixtures/solver_v0.txt
```

The first is gates (a)+(d) plus the unit tests; the second prints per-position
value, nodes, seesaw, proof digest and zone status (gates (b)/(c) consume its
output shape; they are asserted inside the test target too, so the script and
the test agree by construction). Both are wrapped by
`tools/solver_oracle_check.sh` (exit 0/1/2 per the taxonomy; SHELL_CHECKLIST
reviewed; carries a driving test against the shipped script, per D-289's rule).

**Mutation receipts** (each in its own worktree, each must die):
- M-A: drop AT-1's attacker-move-cell union at every order → gate (a) or (c)
  dies.
- M-B: drop one blocking pair (skip the last minimal cover) at an AND node →
  a gate dies.
- M-C: INF as raw `i64::MAX` with plain `+` (overflow/wrap) → a gate dies.
- M-D: ε applied to stored pn → gate (a) or (d) dies.

**Determinism seat**: `tools/solver_determinism.sh` — the selftest binary,
built once, run twice in separate processes over the fixture, full transcript
diffed (value, node count, seesaw, proof digest — everything reproducible; no
wall clock in any choice path). Wired into `tools/ci.sh`; D-7's law gains its
solver seat.

## 8. Config

`configs/solver_v0.toml`, schema in pistol-solver (`serde`,
`deny_unknown_fields`, every key required, no code-side default):

```
schema_version = 1
[solver]
epsilon_num = 1
epsilon_den = 4
zone_orders = 3
free_stone_radius = 8
tt_entries = 1048576
```

All four tunables are ESTIMATED imports or engineering starts (ε=1/4 and
order 3 from square-board papers; `free_stone_radius = 8` is the no-prune
value §3 argues for; TT size unmeasured until §10). Validated by
`tools/config_check.sh` once its schema table learns the file.

## 9. Out of scope (licensed-not-scheduled unless D-numbered)

Search→solver calls, Deep df-pn, df-pn(r), BTA, twin nodes, TCA, SNDA (all
retired by D-436), perf tuning, strength claims, attacker free-stone widening,
free-stone range shrinking below the legal region, 12-fold TT canonicalization.
The seesaw number is measured at the gates and NOT read as a licence.

## 9a. IMPL-TIME AMENDMENT: the deep fixture and the instrument split

Taken at impl, recorded here rather than discovered later by a reviewer:

**R3' is measured intractable on deep positions.** The eight decoy wins
(the canonical-first policy pair is a failing decoy; the search
re-selects onto the overload pair) cost the SOLVER 40-92 nodes each — and
cost R3' **25+ minutes with no verdict on ONE position** (a standalone
timing probe, `/tmp` scratch, never committed). R3' has no LAW-OVERLOAD
shortcut and enumerates every legal defender reply at every AND node, so
its cost explodes with region size, not depth. The design's own §7b cost
row anticipated this shape for the VERIFIER; it applies to R3' a
fortiori.

**The split, therefore**: the fixture becomes two files. `solver_v0.txt`
(61 bounded positions, R3'-differentialable — gate (a) covers it in
under a second) and `solver_deep_v0.txt` (8 decoy wins, excluded from
gates (a) AND (c), covered by gate (b)'s independent full-width verifier
— the design's own instrument for deep positions). The deep positions'
`expect win` is verified by the verifier's re-proof, not by R3'.
Gate (c) also left the deep set, MEASURED: with the decoys attached the
σ-sweep leg exceeded its registered 60-minute wall cap (62+ min CPU,
killed, SIGMA-SAMPLE-OVERRUN by name). The coverage reduction is real
and named — the deep trees get no σ replay, keeping (b)'s solver-zone ==
verifier-zone cross-check as their zone instrument.

**Why the decoys exist at all**: the mutation receipts found the
original fixture vacuous — all 61 positions solved in one node, so
M-A/M-B/M-D touched code the fixture never executed and all four
mutants SURVIVED. The decoys put OR steps, AND steps, re-selection
(seesaw 38-90 events per solve), threshold recomputation and INF
summation on the fixture's execution path.

**A second measured finding, recorded**: the human corpus
(timmyburn/hexo-bootstrap-corpus, 8.7k games) contains NO deep
policy-solver wins under the v0 both-stones-threat-relevant policy —
every winner-to-move position near the end either wins now (one node) or
searches unboundedly (87+ timeouts at 8s, zero deep wins found, even
restricted to games with both players rated 1200+). That is a real
measurement about the v0 policy's narrowness, and it is the concrete
argument for the M4 widening (licensed-not-scheduled).

**The REVIEW-impl amendments** (the review's findings, each closed in code
or recorded here; the report lands with this WP's closure):

- **σ-class narrowing, recorded (M-1)**: the implemented class is |σ| = 2
  ONLY. The registered |σ| = 1 and |σ| = 3 require pre-placement stones
  beyond σ itself (a turn places exactly two stones), and the design never
  specified those stones' zone class — the gap the implementation hit at
  perturb() and closed by refusing to guess. The 17.5% of pairs refused on
  region collision (25,346 of 145,000) is announced in the gate's output
  and is a third narrowing the design also never specified; both are now
  stated here rather than in code comments alone.
- **Fixture composition actuals, recorded (M-2)**: the bounded set's
  registered composition was authored without measuring (the D-291 class).
  What shipped: stone counts 7-21 (53/61 over the registered ≤ 10); depth
  3-4 positions ZERO (the depth-win family degenerated to one-node wins);
  the riposte family was eight byte-identical duplicates until the
  review caught it — now eight distinct positions, re-pinned, re-verified
  by R3' in gate (a). The deep set (8 decoys, 40-92 nodes) is the search-
  exercising half; registered "depth 3-4" positions are licensed-not-
  scheduled — neither hand construction nor the 8.7k-game corpus produced
  one inside the session.
- **Mutation venues, recorded (m-1)**: M-A died at gate (b); M-B, M-C and
  M-D died in the LIB suite and at compile time, not at the four oracle
  gates. The design's receipt table said "a gate dies" for all four; the
  honest reading is "the CI gate complex catches each" (the lib tests and
  the oracle gates run in the same `tools/ci.sh`), and the receipts are
  the four logs in `artifacts/`.
- **Verifier independence is partial, recorded (m-2)**: the verifier
  shares `ZoneP::add_graded`/`union_with` with the solver, so a grading-
  index defect passes the zone cross-check identically in both. The
  verifier's plan families, threat moves, blocking pairs and EP-1 scan
  ARE independent (board reads, no ThreatState); the zone arithmetic is
  shared and the design's "two independent constructions" overstates it.
- **(c1) scope, recorded (m-3)**: the replay checks move legality, not
  that the move still creates its hot window; (c2)'s revaluation is the
  value-axis compensation.
- **Measured figures, now with homes (m-7)**: the σ sweep's 119,654
  placements / 25,346 refused (gate (c) stdout, `artifacts/
  wp18a_selftest_v1.txt` context); the deep fixture's 571 total seesaw
  events (`artifacts/wp18a_selftest_deep_v1.txt`); the corpus measurement
  (87+ unbounded searches, 0 deep wins, both players 1200+ Elo — the
  probe was a scratch binary over
  `timmyburn/hexo-bootstrap-corpus`@`1a82e15`, run 2026-08-25, never
  committed).

**The RED-TEAM closure** (verdict: YES, a false proof could pass —
every named route closed in code or recorded here; the report lands with
this WP's closure):

- **The tripwire's order dependence (B-1, REACHABLE)**: the red team
  constructed a legal position where a legitimate 9917-node win was
  refused as `NoWinUnderZone` — the check ran against the stones
  accumulated *so far* in the DFS, so the root was checked against root
  stones only while its zone carries cells propagated from deep
  descendants. FIXED: the tripwire now runs after the walk completes,
  against the full stone union (order-independent by construction), and
  `emit_node` no longer refuses mid-walk.
- **The NoWinUnderZone laundering path (B-1's second half)**: gate (a)
  mapped `NoWinUnderZone` to "nowin", so a FALSE win whose zone
  overflows would print "nowin", agree with R3' and the expectation on a
  nowin-registered case, and pass. FIXED: the mapping is now the
  registered §7(a) semantics — `NoWinUnderZone` is a MISMATCH that
  fails the gate.
- **The bounded differential is leaf-only (B-2, MEASURED, strengthened)**:
  all 61 bounded cases solve in one node, so gates (a), (c) and (d) never
  executed a search — the red team's own M-A receipt is the evidence
  (gates (a), (c) and (d) all PASSED under that known zone defect; only
  gate (b) died, on the decoys). The stronger finding, MEASURED after
  the review: R3' is intractable on ANY position whose solution contains
  an AND node — the minimal 15-ply overload shape ran 12+ minutes
  without answering (a standalone probe, killed with the session's
  crash; not re-attempted after the operator's restart). The instrument
  split is therefore not "bounded vs deep" but "leaf-only vs
  everything": gate (a)'s differential can never exercise the df-pn
  loop on this game's board size. What ships: gate (b) is the ONLY
  multi-node instrument (a genuine full-width re-proof, the only gate
  that has ever killed a mutant); gate (d)'s deep extension was
  attempted and WITHDRAWN, MEASURED at closure — no deep case returns
  at a 32-entry table in bounded time (the 8 original decoys had no
  verdict in 300 s; decoy-m0 none in 120 s at every size up to 512
  entries, against 0.1 s and ~1 s at the full table), so the 50x node
  cap, which can only fire on a solve that RETURNS, was an unreachable
  detector over a hang; (d) stays bounded, carries a named 10-minute
  wall watchdog (TT-CROSS-OVERRUN) like (b)/(c)'s, and its bounded
  vacuity is recorded here alongside (c)'s tautology; gate (c) remains
  bounded-only and is TAUTOLOGOUS on one-node trees by the same
  measurement (σ cannot touch witness cells that AT-1 puts in every
  order) — its live check is the pre-placement defender-six tripwire
  alone.
- **Deep diversity (B-2's construction route)**: the deep set was one
  base geometry with different anchors; it now carries the 180-degree
  mirror of decoy-0 (634 nodes, 624 seesaw events — a genuinely
  different search shape because canonical-first ordering differs under
  mirroring). Deep NoWin adjudication remains beyond the v0 instrument
  complex (R3' intractable; the verifier re-proves claimed wins only)
  — licensed-not-scheduled with the M4 widening.
- **The filler construction (M-1, recorded)**: leg (c2) measures
  P+σ+fillers, and the red team demonstrated filler-sensitivity on the
  class gate (c) was excluded from (fillers rescuing a σ-broken proof).
  Unobservable on the shipped leaf-only set; the finding binds the
  licensed deep re-inclusion of gate (c), which must re-argue the
  filler policy before its first non-trivial run.
- **The zone certificate's semantic backstop (M-2, recorded)**: with
  gate (c) tautological on the shipped set, no gate can currently
  falsify a consistent-but-wrong zone (the §9a m-2 shared-arithmetic
  risk). The value path never consults zones (verified by the red team:
  `Solver::new` takes only ε and tt_entries), so no false VALUE proof
  is reachable through it — but the zone is the artifact future
  zone-pruning WPs consume, and its certificate is unfalsifiable by the
  v0 gates. That is the concrete debt the M4/1.8c line inherits.

**The TT knee probe (registered as a probe, not a governed run; run at
closure, receipts in `artifacts/wp18a_tt_knee_v1.txt`)**: the gate (d)
withdrawal left the question "does ANY reduced table size terminate the
deep solves in bounded time" open, and that question is WP-1.8c's
licence input. The probe ladders the table size upward per case family
(the 8 decoys at 64/256/1024/4096 entries, 600 s per rung; decoy-m0 at
1024/2048/4096/8192, 900 s per rung) with the shipped selftest binary
in release, stopping at the first size that returns a verdict. MEASURED
at closure (`artifacts/wp18a_tt_knee_v1.txt`): the top rung (8192)
terminated decoy-m0's ladder in practice; the decoy family's knee
lies between 1024 and 4096 entries, decoy-m0's between 4096 and 8192,
and every rung that returned did so with digests, node counts and
seesaw IDENTICAL to the full table — values are stable across a
128-256x table reduction, so a future gate (d) re-extension at a
knee-sized table is possible in principle (1.8b/1.8c's call, with its
own pre-registration). The same receipts falsify the sizing assumption
this paragraph first carried — that a table exceeding the PROOF size
provably suffices: the TT working set measures 11-40x the proof DAG
(92-node proofs needed >1024 entries), and 8192 sufficed for decoy-m0
in practice, not by the proof-size argument.

## 10. Dry run and receipts

**M5's receipt is above (§6), taken before selection.**

**THE DRY RUN, TAKEN** at the first commit where the `solver-selftest` bin
actually solves positions (the df-pn core in, gates not yet), against the
stand-in fixture registered above — two positions differing from the
registered workload only in identity. The stand-in fixture's exact bytes are
preserved at `artifacts/wp18a_dryrun_fixture.txt` (gitignored, sha-anchored
in the ADR; its content is the two cases below):

```
case open-five-win
plies 0,0 7,0 7,2 1,0 2,0 7,4 7,6 3,0 4,0 8,2 8,4
expect win

case no-live-window
plies 0,0 0,8 0,-8 8,0 -7,7 1,8 1,-8
expect nowin
```

Command and output, verbatim:

```
$ cargo run --release -p pistol-solver --bin solver-selftest -- \
    crates/pistol-solver/tests/fixtures/solver_v0.txt   [stand-in path]
case open-five-win value win nodes 1 seesaw 0 digest 09610c19a17a73c6 zone ok
case no-live-window value nowin nodes 1 seesaw 0 digest 0000000000000000 zone -
summary 2 cases 1 wins 0 failures
EXIT=0
```

**The criteria, with their externally-derived referents, all shown**: the
open five prints `win` (a five-own window completes in one stone — rule 2 +
DEF-PLAN, known from the definitions, not from the instrument), with a
nonzero digest and one node (the leaf); the scattered position prints
`nowin` (no live window at own ≥ 2 exists — LAW-SUPPORT k=2 admits no
candidate — again from the definitions) at one node; the whole run costs
0.26 s wall (MEASURED). A malformed fixture (`bogus`) refuses by name at
exit 2 (`CANNOT READ: ... unknown directive "bogus"`), MEASURED. The
per-gate-assertion PASS/FAIL lines are criteria for the GATE revisions, not
this pre-gate dry run. The σ-sample and (c2) costs are measured by the
gates' first run on the registered fixture; the dry run's stand-in measures
the near-best-case σ-solve, and that extrapolation weakness is stated above.
