# Threat Calculus for Hex Connect(6,2,1) — Canonical Reference v1.0

Machine-facing consolidation of Phase 1 (pure derivation), Phase 2 (literature sweep), Phase 3
(community-doc audit). Single vocabulary, stable IDs. Cite results by ID (`LAW-FORCE`, `PAT-GAP`, …).
Reader edition with diagrams: `threat_calculus_reader.html`.

**Status tags.**
`[PROVEN]` derivation and/or exact enumeration (script V-P3.0) · `[LIT]` published, citation given ·
`[VERIFIED]` community claim audited exact · `[CONJ]` plausible, unproven · `[OPEN]` solver/arena target ·
`[REJECT]` do not build on.

**Scope rule.** Every published *number* cited below was established on 4-axis square boards unless
explicitly marked hex; algorithms/structures transfer, numbers do not.

---

## 1. Game facts (pinned in pistol-core; restated for closure)

- Unbounded 2-D hex lattice, axial (q,r); axes `d1=(1,0)`, `d2=(0,1)`, `d3=(1,−1)`; 6 neighbours/cell.
- Win: ≥6 own contiguous on one axis; overlines win. Turn 1 = one stone; every later turn = two.
- A win completes on the placing of a single stone; the turn's second stone is not played. Depth in TURNS.
- Legality: hex-distance ≤8 from an existing stone (rule, not a search knob). No captures, no rule draws.

## 2. Objects

| ID | Object | Definition |
|---|---|---|
| DEF-WINDOW | window | 6 consecutive cells on one axis. **Open** for side X iff no opponent stone in it. Overline monotonicity ⇒ 6-windows suffice for all theory. `[PROVEN]` |
| DEF-PLAN | plan π | the empty-cell set (size ≤2) of an open window with ≥4 own stones. Plans of side X = X's immediate winning completions. `[PROVEN]` |
| DEF-T | threat number t(F) | exact **minimum hitting set** over plan family F (plans have size ≤2 ⇒ vertex-cover instance). `[PROVEN]` |
| DEF-SUPPORT | support | max own-stone count over open windows through a region; the LAW-SUPPORT resource. `[PROVEN]` |
| DEF-STAR | star S₅(c) | cells within window-reach of c: 30 cells, 18 windows per cell (square: 40/24). `[PROVEN]` |
| DEF-TEMPO | tempo value τ(p) | for pre-emp p: `t(activated form) − activation cost`. `[VERIFIED]` (community notation, audited exact — §6) |

## 3. Geometry lemmas

- **LEM-AXES** `[PROVEN]` — 3 axes/cell (vs 4 square); crossing-pair types per cell C(3,2)=3 (vs 6).
- **LEM-CROSS** `[PROVEN]` — two lines on distinct axes meet in ≤1 cell. Source of every fork bound and
  of the weight-addition floor (LAW-OVERLOAD note).
- **LEM-MONO** `[PROVEN]` — stones are never removed; an own extra stone never hurts (zugzwang-free).
  Licenses null-move soundness *in principle*; does not license the strategy-steal (REJ-STEAL).

## 4. Laws

### LAW-HIT `[PROVEN]` (was T2)
One defender stone in any empty cell of a window kills that window permanently. Defense against a plan
family is exactly the hitting-set problem. Kill = hit; no other defensive mechanism exists.

### LAW-FORCE `[PROVEN]` (was T3 + D2)
If the opponent has ≥1 plan and the mover cannot win this turn, every non-losing mover move hits **all**
opponent plans. Corollary (No-Counterattack): counter-threats never substitute for hitting, except
win-now. Makes must-block move filtering *sound*, not heuristic. Community "ignore by lower strength"
violates this and loses by its own tempo numbers (Phase 3 C1).

### LAW-OVERLOAD `[PROVEN][LIT]` (was T4/D4)
t ≥ 3 for the attacker + defender cannot win this turn ⇒ defender lost (two stones cannot hit three).
Published as the Connect6 triple-threat theorem (Wu, Huang & Chang, ICGA J. 28(4), 2005); depends only
on p=2 ⇒ transfers exactly. "Three simultaneous threats = mate" is the disjoint-plan special case;
t is the general criterion.
**Addition floor:** for two genuine fours on distinct axes, t ≥ 2+2−1 = 3 (LEM-CROSS); exact t may be
< the sum of parts (enumerated: crossing fours t=3 ≠ 4; same-line double t=3 ≠ 4). Weight addition is an
upper bound only → RULE-EXACT.

### RULE-EXACT `[PROVEN]` (was T5)
t is computed exactly (vertex cover, ≤3-deep branch & bound over ≤2-cell plans), never read from a
pattern table and never derived by weight algebra. Patterns may order moves; only exact t decides truth.

### LAW-SUPPORT `[PROVEN]` (was T9, with T10)
A forced win in k own turns requires an open window already holding ≥ 6−2k own stones.
k=1 ⇒ a plan exists (≥4); at intra-turn phase 2 (one stone left) the win threshold is plan of size ≤1.
k=2 ⇒ attacker candidates restricted to windows with ≥2 own stones — the completeness license for the
Tier-T staged generator. Threat-windows created this turn held ≥2 stones before (T10).

### LAW-RIPOSTE `[PROVEN]` (was D3; community "Pacer")
A forced defensive stone can itself create a plan and flip initiative. Any forcing-line prover must
check every forced reply for new plans; skipping the check is unsound. This is why VCDT-for-null was
incomplete and VCST was built (Wu & Lin 2010) — the literature confirms the danger and the fix.

### LAW-LEDGER `[PROVEN]` (was D5)
Against a plan family with threat number t, the defender's turn is worth **2 − t** free stones.
t=1 chains bank the defender one stone/turn ⇒ a t=1 chain wins only if it terminates in a win before
the bank funds a counter-fork. VCST is the literature's operational analogue (must end in
triple-threat/win).
**INV-MATERIAL** `[PROVEN][LIT]` (was D6): the 1-then-2 rule keeps the material lead at exactly one
stone — the published p=2q fairness mechanism (Wu & Huang, ACG 2005).

### LAW-DECOMP `[PROVEN][LIT]` (was N3)
Regions with disjoint stars: t is additive and proofs decompose independently (license for DBS-style
decomposition; also the mechanism behind community colony spam / location-insensitive threats).
The same additivity is FALSE for static evaluation — never sum regional eval as if independent.

### Fork bounds (was T11/T12)
- **BOUND-CONVERT** `[PROVEN]` — one new stone converts ≤3 pre-threats into threats on hex (≤4 square).
- **BOUND-FORK** `[PROVEN]/[CONJ]` — cheapest double threats are same-line (live four, ~4 stones);
  cross-axis forks cost ~7–8 stones `[CONJ, verify V2]`; 3 crossing-pair types per cell (LEM-AXES).
  Net: hex play is more "linear," plausibly defender-leaning (§8 F-DIR).

## 5. Pattern audit (exact, enumerated; hex axes)

| ID | shape | t | notes |
|---|---|---|---|
| PAT-O5 | open five | **2** | plans include both end singletons; both must be hit |
| PAT-O4 | open four | **2** | 3 plans; a hitting pair exists (e.g. the two inner-end cells) |
| PAT-GAP | X·XXXX, open right | **2** | **Gap Trap** (was T7): singleton must-hit plan at the gap + live right extensions. Absent from named-shape taxonomies; tables mis-score it |
| PAT-C4 | closed four | **1** | single plan |
| PAT-4IFF | contiguous four | **2 iff** | two-deep empty on BOTH sides; else t ≤ 1 (was T6) |
| PAT-RHOMBUS | 4-stone diamond | **0** | ≤2 collinear on every axis ⇒ empty plan family. Menace is multi-turn generation, not weight. Forced-win status: OPEN-RHOMBUS |
| PAT-O3 | open three | **0** | no plan; activates to PAT-O4 with 1 stone ⇒ value is pure tempo (τ=+1) |

## 6. Tempo (community notation, audited)

τ(p) = t(activated) − cost. `[VERIFIED]` exact on all canonical pre-emps:

| pre-emp | activates to | t | cost | τ |
|---|---|---|---|---|
| open three | open four | 2 | 1 | **+1** |
| closed three | closed four | 1 | 1 | **+0** |
| open two | open four | 2 | 2 | **+0** |
| closed two | closed four | 1 | 2 | **−1** |

Audit note: the community's "open three = W3" is wrong under every formalization (all give 2); it hid
because 3−2 and the true 2−1 both print +1. Adopt the tempo layer; never the weight layer (REJ-WSC).
Engine use: incremental ±τ census per side = cheap eval-term candidate (SPRT-gated).

## 7. Search protocol (normative)

**PROTO-NODE** `[PROVEN]` — at every node, in order:
1. **Win-now check.** A completing stone ends the turn instantly (second stone unplayed). Score in turns.
2. **Overload check.** Opponent t ≥ 3 and step 1 failed ⇒ lost; stop (LAW-OVERLOAD).
3. **Survival filter.** Candidate moves must hit all opponent plans (LAW-FORCE). Inside provers, run the
   riposte check on every forced reply (LAW-RIPOSTE).
4. **Staged generation.** Tier F (forced) → Tier T (LAW-SUPPORT-qualified) → Tier Q (quiet).
   Unordered-pair canonicalization; intra-turn phase bit in the hash (PROTO-PAIR, was N1;
   phase bit merges completed turns, never phase-1 nodes).
5. **Quiescence.** Threat-only, zone-bounded (Tier F + Tier T with t ≥ 2), never full-width (was S3).

**ZONE-R** `[LIT]` (supersedes N2's ±5 heuristic) — finite proofs on the unbounded board use relevance
zones defined *combinatorially* by open active segments (RZOP; Wu & Lin, IEEE T-CIAIG 2(3), 2010);
order ≤ 3 suffices for two-stone moves. A bounded-depth search without a zone argument is evidence,
never proof — on either side of any claim (binds us and the community equally; the "horizon exploit"
is this theorem's absence in bots).

**PROOF-ENGINE** `[LIT]` — df-pn with the 1+ε trick (Pawlewicz & Lew 2006) or Deep df-pn
(Connect6-validated) for VCDT/VCST-style forcing proofs; GHI is moot here (no captures; residual
transposition handled by the phase bit).

**Known unsoundness perimeter** (was S4): (i) t=1 free-stone dynamics if the ledger term is ignored,
(ii) Tier-Q interior, (iii) truncated riposte chains. Guard with V4/V5/V6-class tests.

## 8. Eval constraints

- **THM-WINDOW** `[PROVEN]` (was E1) — length-6 occupancy tables cannot classify live vs dead fours
  (needs 8-cell context; fives need 7). Fixes in cost order: exact-t counters as eval terms (free from
  PROTO-NODE), pattern context ≥8–9, learned codebook (Rapfi-style, Stage 2). The "length-11" Gomoku
  convention is k=5 lore — re-derive the minimal sufficient hex length by enumeration.
- **E-PHASE** `[PROVEN]` — win threshold differs by intra-turn phase (phase 2: plan ≤1 wins); tempo
  terms condition on (side, phase).
- **E-INIT** `[CONJ]` — discount positional terms while opponent t ≥ 1 (initiative first). SPRT-gated.
- **F-DIR** `[CONJ, supported]` — 3-axis hex is defender-leaning vs 4-axis square. Support: Sieben's
  hex weak (1,2)-achievement classification (Acta Cybernetica 16(4), 2004; all 5+-cell animals lose;
  caveats: Maker-Breaker, roles reversed, shape ≠ line) and Győrffy et al.: pairing DRAW for
  7-in-a-row on the 3-direction board — via the hex ≅ square-3-directions isomorphism (community
  "hotel.coffee", = LEM-AXES setting) that result applies verbatim to this lattice
  (single-stone Maker-Breaker version). k=6 with pair moves remains open.

## 9. Adopted with sources

| ID | item | source | use |
|---|---|---|---|
| ADOPT-RZOP | relevance zones, order ≤3 | Wu & Lin 2010 | ZONE-R; all proofs |
| ADOPT-DFPN | df-pn + 1+ε / Deep df-pn | Nagai 02; Pawlewicz & Lew 06; Deep df-pn 16/17 | proof engine |
| ADOPT-RAPFI | pattern-codebook incremental eval under α-β | arXiv:2503.13178 | Stage-2 eval template |
| ADOPT-DBS | dependency decomposition where stars disjoint | Allis 94; Wu et al. 2014 | LAW-DECOMP application; hex independence rate unknown |
| ADOPT-SPRT | GSPRT + pentanomial, paired openings | Fishtest practice | all strength claims (arena already implements) |
| ADOPT-TEMPO | tempo census | community, audited §6 | eval-term candidate |
| ADOPT-FINISH | Finisher catalog (V,T,Y,L, Scissors, Is/Towers/Stilts, Quadtri) | HeXOpedia Ch.5 | fixtures AFTER per-shape t-audit (V-P3.8); PI marks = reachability priors |
| ADOPT-7/36 | anticonnective density 7/36 | 2swap (video) | drawishness/contempt calibration, paving prior — AFTER independent verification (V-P3.7) |
| ADOPT-BOOK | opening corpus (Island Gambit A: X crushes O; Fool's Defense refuted; Standard-Pair vs Triangle) | HeXOpedia Ch.6 | arena book seeds; targeted anti-sealbot line |
| ADOPT-LOCSENS | location-(in)sensitivity vocabulary | HeXOpedia 3.4 | = star disjointness; candidate-radius SPRT context (r=2 vs r=3) |

## 10. Rejected

| ID | item | why (proof in Phase 3 doc) |
|---|---|---|
| REJ-WSC | W-S-C weight layer for S ≥ 1 | undefined by its own author; open-2 is 2-2-2 and 1-2-2 in the same document; rhombus "W3" has t=0 and no defined reading (C2/C3/F1) |
| REJ-IGNORE | ignore-by-lower-strength rule | violates LAW-FORCE; loses by its own τ numbers (C1) |
| REJ-ALGEBRA | transform algebra (weight add/subtract) | upper bound presented as identity; counterexamples t=3 ≠ 4 twice; subtraction on inflated W flips verdicts (F3) |
| REJ-STEAL | §7.2 strategy-steal "theorem" | junk-stone pass cannot occupy a seat answering a 1-stone opening; extra OPPONENT stone breaks monotonicity direction; unproven (C4). Conclusion may be true |
| REJ-R110 | Rule-110 ⇒ undecidable outcome | no forcing reduction; lattice hosting a CA ≠ game computing it (F4). Family-version plausible, unproven |
| REJ-DEPTHPROOF | bounded-depth "proofs" without zone argument | ZONE-R; binds us too (F6) |
| REJ-W5 | "open 3 needs 5 moves to block all mates" | no formalization reproduces 5 (candidates give 2, 2, >7); quantity has no measurement procedure (F2) |

## 11. Open questions & verification registry

| ID | question / check | instrument |
|---|---|---|
| OPEN-RHOMBUS | unmolested rhombus a forced win? | df-pn fixture V-P3.3(a), zone-bounded, 3-valued verdict |
| OPEN-O3 | isolated open three a forced win? | V-P3.3(b) |
| OPEN-DISJOINT | reachability: can the defender always prevent two simultaneous open threes? | arena instrumentation V-P3.6 (count positions with ≥2 plan-disjoint live fours vs pistol defense) |
| OPEN-VALUE | game value (draw consensus vs win) | never asserted; measured (self-play first-player rate; pairing evidence defender-ward) |
| V-P3.0 | enumeration script (plans, exact t) | **LANDED, TEST-TREE-ONLY** — `crates/pistol-solver/tests/common/plans.rs`: exact `t`, no ceiling, windows enumerated by position. It VERIFIES and records no number, so it does NOT move to `tools/`; promotion is a future ADR owed the day anything records from it (D-287) |
| V-P3.1 | C1 ledger fixture (rhombus + closed-2 refusal) | tactical fixture |
| V-P3.2 | pattern fixtures = §5 table; mutation-gate the threat oracle (Gap Trap must be caught) | **LANDED** — `tests/fixtures/pattern_v0.txt` (sha-pinned) + `pattern_calculus_tests.rs`; every row checked by the exact reference, the from-scratch reference and the shipped `ThreatState`; §5's numbers hand-typed in `SECTION_FIVE` and compared. Gap Trap gated by two recorded mutations, GT-1 and GT-2 (D-287) |
| V-P3.5 | algebra counterexamples as regression tests (t exact, never additive) | **LANDED** — `threat_number_is_never_additive`: crossing fours `t = 3 != 2+2` split by axis within one position, same-line double `t = 3 != 2+2` against the two halves as records. Mutation GT-3 turns it red (D-287) |
| V-P3.7 | verify 7/36 tiling hits every 6-window; LP lower bound | offline, before ADOPT-7/36 |
| V-P3.8 | Finisher catalog t-audit; any t<3 entry falsifies it; import survivors sha-pinned | offline, before ADOPT-FINISH |
| V1–V9 | Phase-1 verification hooks (support-bound tests, staged-vs-full agreement, mutation gates, riposte fixtures, free-stone ablation, eval SPRT, incremental-t differential) | per Phase-1 doc; unchanged |

## 12. Provenance map (old tag → canonical ID)

G1/G2→LEM-AXES/LEM-CROSS · G3→DEF-WINDOW · G4→DEF-STAR · G5→DEF-WINDOW note · G6→LEM-MONO ·
T1→PROTO-NODE step 1 + LAW-SUPPORT k=1 · T2→LAW-HIT · T3/D2→LAW-FORCE · T4/D4→LAW-OVERLOAD ·
T5→RULE-EXACT · T6→PAT-4IFF · T7→PAT-GAP · T9/T10→LAW-SUPPORT · T11→BOUND-CONVERT ·
T12→BOUND-FORK · D3→LAW-RIPOSTE · D5→LAW-LEDGER · D6→INV-MATERIAL · N1→PROTO-PAIR ·
N2→ZONE-R (superseded mechanism) · N3→LAW-DECOMP · E1→THM-WINDOW · E2→E-PHASE · E4→E-INIT ·
S1–S4→PROTO-NODE + unsoundness perimeter · F4/N4→F-DIR.

---
*v1.0 · changes to this file follow the ADR process; a new law or a status-tag promotion
(CONJ→PROVEN, OPEN→settled) is an ADR line.*
