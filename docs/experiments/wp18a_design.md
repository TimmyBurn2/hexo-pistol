# WP-1.8a design — relevance zones + df-pn in pistol-solver, correctness only

Revision 1. Governing dispatch: `[GROUNDWORK] WP-1.8a` (operator). D-436 governs
the GHI retirement; D-434/D-435 landed with this WP's opening commit and bind
nothing here beyond the teacher question they settle.

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
     AT-1/DT-1/EP-1/T1/T2/T3.
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
value (sound, incomplete); the defender is NOT restricted except as §3 states.
The value is `Win` or `NoWin` — df-pn is run to completion, no node budget; the
solver is not on the search path and makes no strength claim (dispatch DONE
list). Turns, not plies, are the depth unit throughout (rule 4).

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

**Defender (AND node), in order — every step is a law, not a heuristic:**
1. Defender wins this turn (`can_win_this_turn(defender, Two)`): the node is
   `NoWin` — rule 4's race is decided by the side to move and the defender
   moves now.
2. Attacker plans = attacker hot windows (DEF-PLAN); exact minimum hitting set
   size `t` over their empty cells (RULE-EXACT; `Cover`/`min_hitting_set`
   arithmetic already in the crate). `t ≥ 1` always here: the attacker's last
   move created a hot window by §2.2 — asserted, fail loud if false.
3. `t ≥ 3` and step 1 false: the node is `Win` without expansion — LAW-OVERLOAD
   (two defender stones cannot hit three).
4. `t ∈ {1, 2}`: blocking pairs = pairs covering all plan-empty cells = pairs
   containing some inclusion-minimal cover (`blocking_covers(defender, Two)`).
   Each is enumerated as: the cover itself (size 2), or cover cell + free stone
   `f` (size-1 cover). The free stone ranges over the §3 search zone. The
   defender's free stone is the LAW-RIPOSTE danger, and enumerating it at every
   zone cell is RZOP's seminull verification in the df-pn setting (T1-1c).
   Non-blocking pairs are NOT enumerated: a surviving plan completes next turn
   by DEF-PLAN + rule 3 (≤ 2 empties, 2 stones), so they are already `Win` for
   the attacker; the oracle reference (§7a) reaches the same classification by
   its own code.

## 3. Zones

**Search zone `Z_s` (top-down, fixed at the root).** Every empty cell within
hex-distance `search_zone_radius` (config, registered start 4, ESTIMATED —
the RZOP paper's radius is square-board and non-transferable) of any stone.
Defender free stones range over `Z_s`. This is the only pruning the zone does.

**Proof zone `Z_p` (bottom-up over the found proof).** A sequence
`Z_1 ⊆ Z_2 ⊆ Z_3` (`zone_orders` config, registered start 3, ESTIMATED —
ZONE-R/ADOPT-RZOP: "order ≤ 3 suffices for two-stone moves", square-board),
constructed per Wu & Lin §IV, adapted to 3 axes and this formulation:

- **AT-1** (OR node, proven by move m): `Z_p(node) = Z_p(child) ∪ cells(m)`,
  elementwise across the sequence.
- **DT-1** (AND node, proven): `Z_p(node) = ⋃` over all enumerated defender
  pairs of `Z_p(child(pair))`, elementwise, plus the T-contributions below.
- **EP-1** (any proof node): for each defender hot window with exactly `k`
  empty cells (k ∈ {1, 2}; hot means ≥ 4 own), those empties enter `Z_k..Z_3`.
  A defender hot window is the only defender formation that can win within the
  proof's horizon; deeper defender builds are the higher-order tolerance.
- **T3-1**: the attacker's hot-window empties enter ALL of `Z_1..Z_3`.
- **T3-2**: EP-1's contributions (the paper's own pairing of the two).

`Z_p` is stored in the solver TT entry when a node is proven (memoising zone
construction across transpositions) and recomputed bottom-up during proof-tree
reconstruction.

**The soundness hinge, stated once.** Pruning free stones to `Z_s` is sound for
a found proof iff `Z_p ⊆ Z_s`: a free stone outside `Z_s` is then outside
`Z_p`, and the RZOP relevance property (Wu & Lin Definition 3, the property
gate (c) replays) says defender stones outside the proof zone do not break the
proof. The solver CHECKS `Z_p ⊆ Z_s` after every solve and reports a zone
overflow as a named outcome (`NoWinUnderZone` — fail loud, rule 3) rather than
claiming the win.

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

**INF sentinel**: `INF = 1 << 62` (u64). All pn/dn/threshold arithmetic is
saturating: `INF + x = INF`, `INF + INF = INF`, `min`/`max` saturate at INF,
no operation can wrap (u64::MAX is unreachable by construction; the ε multiply
is done in u128 and clamped to INF). No negative values exist (unsigned).

**Solver TT.** Key: pistol-core's full 128-bit key (D-8) — side-to-move and
intra-turn phase bit included (D-9), so half-move positions key correctly and
no GHI machinery is needed (D-36: the game is monotone, the state graph is a
DAG). Entry: `{pn, dn, value ∈ {Proven, Disproven, Unknown}, zone: Option<ZoneP>,
generation}`. Two-level bucket (Breuker's TwoBig, the scheme the paper itself
used): slot pair per index — depth/generation-preferred + always-replace;
PROVEN entries are never replaced by unproven ones (dispatch). Replacement is
by key hash into a fixed array — deterministic, no hasher iteration on any
choice path (D-7).

**Proof-tree reconstruction.** After a root `Win`: walk from the root — OR
nodes take the least policy move (canonical order, stable tie-break) whose
child entry is proven; AND nodes take every enumerated defender pair; leaves
re-verify rule 2 through pistol-core's own win detection. Output: the proof
tree (moves per node) + `Z_p` + node count + seesaw count. Gate (b) consumes
this output.

**Seesaw counter.** One counter per solve. A seesaw event: a df-pn recursive
call returns on a threshold miss, and the parent's next descent selects a
different child. Printed per solve. MEASURED on the fixture set, no threshold
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

| # | decision | options | recommendation + strongest attack |
|---|---|---|---|
| M1 | defender free-stone range | (a) full legal region; (b) fixed root `Z_s`; (c) RZOP dynamic seminull sets | **(b)**. (a) forfeits the zone entirely (~10-50× per AND node, and the dispatch requires zone pruning); (c) needs the strategy-first verifier architecture, incompatible with df-pn search. Attack: a root-fixed zone ignores what the proof discovers — answered by the `Z_p ⊆ Z_s` check, which converts a too-small zone into a loud `NoWinUnderZone`, never a false win. |
| M2 | zone representation | (a) sequence `Z_1⊆Z_2⊆Z_3`; (b) flat set | **(a)**. EP-1's classification is per-order and gate (c)'s tolerance class is order-structured (Wu & Lin Definition 3); a flat set collapses the tolerance claim to the weakest order. Attack: more state in the TT entry — cost is three small sorted vecs, measured at impl. |
| M3 | node accounting | (a) inside per-side budget; (b) separate, both printed | **(b)**, §5. Attack: two knobs to configure per seat — real, and cheaper than a silently unmatched SPRT. |
| M4 | attacker policy width v0 | (a) both stones threat-relevant; (b) one free stone | **(a)**. (b) multiplies OR branching by the legal region and is a width claim needing its own gate story. Attack: (a) proves strictly less — recorded as the licensed-not-scheduled widening, not hidden. |
| M5 | harness home | (a) bin target inside pistol-solver; (b) bin in pistol-cli; (c) tests only | **(a)**. (b) creates the normal reverse edge that WP-1.5a's `p = 0` claim and `tools/solver_edge_check.sh` adjudicate against; (c) cannot give the determinism gate a two-process seat. Attack: a bin inside the crate is still "a binary in this workspace" on a literal reading — answered at impl by driving the SHIPPED edge-check script against a scratch workspace containing exactly this shape and recording its verdict. |

## 7. Oracles — the gate of this WP

Fixture: `crates/pistol-solver/tests/fixtures/solver_v0.txt`, sha-pinned,
machine-checkable loader (the `pattern_v0.txt` discipline: unknown line or
missing expectation = panic). Positions are hand-built small forcing setups and
near-misses; **bounded means: ≤ 10 stones, proofs ≤ 4 turns deep** (caps are
design parameters, stated as numbers; wall times MEASURED at §10 and recorded
here when the oracle first runs). Sample size: ≥ 60 positions, ≥ 20 `Win`,
≥ 20 `NoWin`. The loader refuses a fixture whose caps it cannot verify.

**R3' — the brute-force reference** (new, test tree only, alongside R1/R2/R3):
a memoised AND-OR over the SAME policy semantics, written against `Board` and
its own hitting-set enumeration, sharing nothing with the solver but
pistol-core: no df-pn, no thresholds, no TT, no zones, and its defender
free-stone range is the FULL legal region. Semantics shared, machinery not.

- **(a) Differential**: for every fixture position, solver value == R3' value.
  Because R3' enumerates free stones over the full legal region and the solver
  over `Z_s`, this gate adjudicates BOTH the df-pn machinery AND zone-pruning
  sufficiency on the fixture class.
- **(b) Proof-tree re-verification**: every attacker node's claimed move is in
  the policy set (recomputed independently); every defender node's enumerated
  pair set is complete per §2 (recomputed independently); every leaf is a
  rule-2 win via pistol-core. Runs over every `Win` fixture.
- **(c) RZ property**: for every `Win` fixture, for every defender
  pre-placement σ with |σ| ≤ `zone_orders` where σ's i-th stone lies outside
  `Z_i` (Wu & Lin Definition 3's irrelevance, restricted to the bounded region
  around the stones — sample = ALL such σ), replay the proof tree's attacker
  strategy; it must still win. Sample size stated as the count of σ per
  fixture.
- **(d) TT cross-check**: full TT vs a 32-entry TT, identical VALUES on the
  fixture set (node counts and seesaw may differ; values may not).

Exact commands (release, as the tactical gate precedent dictates):

```
cargo test --release -p pistol-solver --test solver_oracle_tests
cargo run --release -p pistol-solver --bin solver-selftest -- \
    crates/pistol-solver/tests/fixtures/solver_v0.txt
```

The first is gate (a)+(d) plus the unit tests; the second prints per-position
value, nodes, seesaw, proof digest and zone overflow status (gates (b)/(c)
consume its output shape; they are asserted inside the test target too, so the
script and the test agree by construction). Both are wrapped by
`tools/solver_oracle_check.sh` (exit 0/1/2 per the taxonomy; SHELL_CHECKLIST
reviewed; carries a driving test against the shipped script, per D-289's rule).

**Mutation receipts** (each in its own worktree, each must die):
- M-A: drop AT-1's attacker-move-cell union → gate (a) or (c) dies.
- M-B: drop one T2 blocking pair (skip the last minimal cover) → a gate dies.
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
search_zone_radius = 4
tt_entries = 1048576
```

All four tunables are ESTIMATED imports or engineering starts (ε=1/4 and
order 3 from square-board papers; radius and TT size unmeasured until §10).
Validated by `tools/config_check.sh` once its schema table learns the file.

## 9. Out of scope (licensed-not-scheduled unless D-numbered)

Search→solver calls, Deep df-pn, df-pn(r), BTA, twin nodes, TCA, SNDA (all
retired by D-436), perf tuning, strength claims, attacker free-stone widening,
12-fold TT canonicalization. The seesaw number is measured and NOT read as a
licence.

## 10. Dry run (registered commands exercised before review)

The §7 command shapes were exercised on a stand-in fixture of the same kind
(two positions, one trivial win, one trivial refutation, thrown away after the
dry run — not the registered workload). Output and wall times recorded here at
impl time; the criterion each command's output must show: the test target
prints one PASS/FAIL line per gate assertion, the selftest prints one line per
position with all six fields, and neither exits 0 on an empty run.
