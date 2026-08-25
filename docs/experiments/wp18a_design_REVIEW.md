# REVIEW-design — `docs/experiments/wp18a_design.md` revision 1 (WP-1.8a)

**Reviewed revision:** `6f13d0c` (git SHA `6f13d0c894feedaf899834522624a5cbf7c681f7`, branch `dev`). HEAD equal at review time — confirmed.

**Verdict: FAIL** — 1 BLOCKING, 8 MAJOR, 6 MINOR. The §2 move policy and the §4 Pawlewicz-Lew quotations survived adversarial checking (details at the end); the §3 zone construction did not: it is not the Wu & Lin construction its soundness hinge depends on, and the design's own justification for the difference is false.

Landed retroactively with revision 2's fix round (the review was returned as a message before the report file existed; its content is unchanged). The findings below are quoted in summary form; revision 2 of the design cites them by number.

## BLOCKING 1 — §3's EP-1/T3-2 are narrowed to hot windows; the constructed `Z_p` is not a relevance-zone sequence, and the hinge "sound iff `Z_p ⊆ Z_s`" is unsupported (a false `Win` is reachable)

The design's words: "**EP-1** (any proof node): for each defender hot window with exactly `k` empty cells (k ∈ {1, 2}; hot means ≥ 4 own), those empties enter `Z_k..Z_3`. A defender hot window is the only defender formation that can win within the proof's horizon; deeper defender builds are the higher-order tolerance."

What Wu & Lin's EP-1 actually says: "For each **active segment of the defender** containing exactly k unoccupied squares, these squares … are all added into Z_k **or higher order zones**". "Active segment" is defined as *any* segment with no attacker stones — a defender **live three** (3 own, 3 unoccupied) contributes its three empties to `Z_3`; a live two / live one contribute at higher orders, which the paper caps into `Z_3` in practice ("we construct zones with size three, and simply use Z_3 for those higher order zones, whenever needed"). A "hot window" in the design's vocabulary is the paper's *threat segment* — a strict subset.

The design's justification sentence inverts the paper's treatment: in the paper, deeper defender builds are **not tolerated** — their squares are placed *into* the zones, which **removes** them from Definition 3's irrelevance class. The design puts them in **no** zone, which **adds** them to the tolerance class. And the sentence's first half is false on its own terms: a defender live three wins within two defender turns, well inside a ≤ 4-turn proof's horizon.

Consequence: the hinge ("a free stone outside Z_s is then outside Z_p, and the RZOP relevance property says defender stones outside the proof zone do not break the proof") has no support — Definition 3's property is guaranteed only for zones built by the paper's full operation set, and the design's `Z_p` is strictly smaller at every order. The solver certifies wins it should not. Reproducer sketch: a defender live-one whose distance-5 extension cell lies outside `Z_s` (radius 4); the defender blocks with the cover stone and spends the free stone on that cell every turn — all free stones outside `Z_s`, none enumerated; the build converts inside the proof's horizon; the solver reports `Win` with `Z_p ⊆ Z_s` holding. False win, certified. Fix direction: restore the paper's grading — all defender active segments, k unoccupied → `Z_k..Z_3` (order-3 cap as the paper does).

## MAJOR 2 — No gate adjudicates the §2 defender-side semantic lemmas: R3' shares them, so gate (a) is vacuous for the policy-semantics defect class

R3' shares the lemmas ("non-blocking pairs are already Win", the t ≥ 3 shortcut, the step-1-before-enumeration ordering) at the LEMMA level: implemented twice but asserted once. A false lemma passes (a) and (b) identically. Fix: make R3' full-expansion — enumerate all legal defender pairs; the attacker side stays policy-restricted because that is the game being solved.

## MAJOR 3 — §4 quotes the first-child threshold formulas but never states how they generalize to the df-pn child loop

The paper's derivation is for the first child under `p1 ≤ p2 ≤ …` at a single visit. A real df-pn implementation loops: after a child returns, the parent re-computes, re-checks termination, re-selects, and recomputes `p2`/`d2` against the new ordering before descending again. Pin it.

## MAJOR 4 — The INF-saturation argument does not cover subtraction, the one operation that can underflow

Both parent formulas subtract (`dt1 = dt − d + d1`, `pt1 = pt − p + p1`); in unsigned arithmetic these underflow whenever `d > dt + d1`. The saving invariant — thresholds are only computed for the child being descended into, after confirming `p < pt ∧ d < dt` — is what the design fails to state.

## MAJOR 5 — Gate (c) is under-specified to the point of being either vacuous or unachievable

Four things open: "the bounded region" never defined; replay semantics unstated (tree defender moves only = vacuous, or adversarial = a second AND-OR verification per σ); the quantifier on "σ's i-th stone lies outside Z_i" ambiguous; the paper's zones are consumed dynamically while the design replays against statically reconstructed zones, and the design cites Definition 3 for semantics it does not reproduce. Given BLOCKING 1, this gate is the only empirical check of the zone construction and it is the least specified thing in §7.

## MAJOR 6 — Gate (d)'s 32-entry TT run has no termination story

df-pn correctness is TT-size-independent, but termination is not: the monotone-progress argument breaks under heavy eviction and the seesaw effect can loop forever. A hang has no node cap, no timeout, and no registered outcome. Register a node-budget abort with defined failure semantics.

## MAJOR 7 — §10 claims the dry-run discipline and then does not perform it

The output is deferred past the review ("recorded here at impl time"); the stand-in input is not recorded; the instrument is unnamed and unrevisioned. CLAUDE.md: the pre-registration records the dry-run input and its output, and names instruments with revisions.

## MAJOR 8 — The matrix breaks the MEASURED/ESTIMATED marking law; both load-bearing numbers are derivable in seconds (D-291 class, twice)

§6's one numeric claim ("~10-50× per AND node") carries no mark. The registered `search_zone_radius = 4` is refuted by the design's own §2 + AT-1: a live-two's far empty sits at distance 4; two chained extensions reach 8 — radius 4 cannot certify the proofs the fixtures permit. Compounding: the under-built `Z_p` (BLOCKING 1) makes the subset check easier to pass while a too-small `Z_s` makes it harder — tuning the radius up widens the false-win window. The strongest surviving attack on M1 (sub-hot builds outside Z_s, invisible to the subset check under the narrowed EP-1) is unrecorded.

## MAJOR 9 — The proof-tree-reconstruction seam is under-specified in three load-bearing places

(a) Two `Z_p` constructions (TT-memoised vs reconstructed), one certificate, no choice stated. (b) Eviction breaks the walk: "take the least policy move whose child entry is proven" assumes an unevicted table — certain under gate (d)'s 32-entry TT. (c) Gate (b)'s leaf criterion ("every leaf is a rule-2 win") is unsatisfiable on §2's own proof trees: t ≥ 3 nodes and non-blocking classifications close subtrees without board-terminal leaves; an implementer who skips such leaves verifies the two shortcut classifications with nothing.

## MINORs

1. D-36 is mis-cited for the monotone/DAG claim (that is D-436's text; D-36 is the win-detection API decision).
2. `NoWinUnderZone` vs gate (a): the comparison is not defined (three-valued solver, two-valued reference).
3. No fixture-composition requirement stresses the zone (all 60 could sit trivially central; ≥ 20 residual positions in an unnamed bucket).
4. The loader cannot pre-verify proof depth (a property of the solution, not the input); post-run filter or drop the claim.
5. M5's recorded attack is answered after selection ("answered at impl"), which the matrix rule does not allow.
6. M-B names "T2", a term the document never defines; §2.4's "(T1-1c)" overclaims inheritance (the paper's T1-1c ranges the free stone over a zone derived from the sub-proof, not a fixed root set).

## Verified clean

- §2 move policy lemmas: C-pairs ⊆ `generate_turns` airtight; every hitting pair contains an inclusion-minimal cover (true as implemented); non-blocking ⇒ attacker wins next turn sound (completion legal by rule-5 arithmetic, one-empty case ends the turn by rule 4); step order correct and complete (LAW-OVERLOAD's precondition is step 1); `t ≥ 1` at every AND node airtight.
- §4 quotations character-exact against the paper (both formula pairs, root thresholds, leaf numbers, TT-miss init, Fig. 1 node numbers, 1+ε quote, ε = 1/4, TwoBig); AND/DN ε form correctly marked DERIVED.
- Calculus citations each used for what the law says.
- Premises verified against the live tree (no search in pistol-solver; no report B; D-numbers cited correctly except MINOR 1).
- Dispatch fit satisfied in substance.
