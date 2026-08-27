# WP-1.8b §2 — the solver on the search path (design)

**Revision 2. Closes the DECISION-RED-TEAM's findings (MATRIX FALLS on
three code-grounded claims; every RECOMMENDATION survived). BLOCKING 1:
the defender score was parity-violating — the opponent's d-th turn from N
sits at root-offset `t+2d` (EVEN, `score.rs`'s law; the code's own
OverloadReturn row is the d=1 instance), so the formula is
`-mate_in(t + 2d)`, not `-mate_in(t + 2d - 1)`. BLOCKING 2: the
additive-parser claim was false for `tools/sealbot`'s substring parser
(`"solver_nodes 300"` contains `"nodes "`) — safe only by field-order
luck; the design now pins the print order (`solver_nodes` strictly after
`nodes`, test-pinned) AND word-boundaries the sealbot parser at impl.
BLOCKING 3: the zero-plan AND root is UNREACHABLE under D1's own trigger
(hot ⟺ completable-this-turn, so mover-hot ends the defender call in one
visit via the race check, and opponent-hot means the AND assert passes
UNRELAXED) — the relaxation machinery is deleted and the reachable
mechanism spelled: `solve_defender` is a thin wrapper (attacker =
opponent, the SAME `solve_root`, whose to-move dispatch lands in the
existing AND path), zero df-pn changes, with the zero-plan class refused
loudly by the existing assert if a future trigger ever admits it. Plus
the MINORs: the Staged-only wiring with a named refusal for
`on_search_path` under Radius (4 of the 12 configs), the stop-contract
sentence amended (between checks the counter can absorb up to one capped
call per visit; deterministic still), the two-validator sentence
re-spelled, the 16384 ESTIMATE's caveat (attacker-direction data only),
the root extraction keyed by `tree.root` (the emission is post-order —
solver.rs's own doc comment corrected at impl), the turn-1 guard, the
wholesale solver reset named as wiring code, and the `NoWinUnderZone`
counter's home named (`SearchInfo`).**

**What this wires**: the M4-widened policy-game df-pn solver
(`pistol-solver`, `attacker_policy = "one_free_stone"`, the committed
default since the M4 round) into `pistol-search`'s PVS, behind a config
gate that is FALSE in every committed config until an SPRT says otherwise.
The solver's verdict is a VALUE, never a move-ordering hint. The wiring is
judged by section 5's SPRT; this design is the soundness and accounting
story the SPRT presumes.

The commissioning dispatch's defaults are the floor this design decides
against; every deviation is an option-matrix row with the strongest
surviving attack recorded.

## 1. The seam

`pistol-search` gains a dependency on `pistol-solver` (the normal
direction — search USES solver; the WP-1.5a `p = 0` posture is superseded
BY THIS WP, which exists to wire it, and the WP-1.8a edge-check tool
remains the adjudicator for any future claim about who links whom).
`pistol-search` re-implements no rule (rule 2): the solver is called with
the search's own `GameState`, and every verdict flows from pistol-core's
rules through the solver's policy game.

**Where the solver lives**: in `Searcher`, constructed from the same
config seam as everything else, ONE instance per engine — its epoch TT is
reused across every call in a game (the epoch mechanism isolates solves;
`Searcher::clear` — what a new game is — resets it wholesale, the
belt-and-braces the determinism seat's C-vs-D comparison can see). The
per-call node cap and trigger live in the engine config (rule 1); no
code-side default.

## 2. Call sites, trigger, and directions (matrix D1)

The dispatch's default, taken with one sharpening:

| # | decision | options | recommendation + strongest surviving attack |
|---|---|---|---|
| D1 | call sites | (a) root + every node whose mover OR opponent holds an open four-or-better (a hot window, PAT-O4+, by calculus ID); (b) every node; (c) root only | **(a)**. (b) multiplies per-call cost by the node count with no threat to find (the solver's policy game needs a live window to move at all — an OR node with `C = ∅` and `R = ∅` is a one-visit NoWin, so most quiet-node calls are cheap no-ops, but the DEFENDER-direction calls at quiet nodes are not: they enumerate the mover's whole legal turn set at the root); (c) forfeits the interior cutoffs that are the anchor's whole measured story (sealbot's tree collapsed on its own 1-node instant-win probes at interior decisions). **THE DIRECTION DISPATCH, stated (the red team's gap (i))**: mover-hot ⇒ the ATTACKER call (a win this turn is a one-visit leaf) and the defender call answers in one visit by the race check — both cheap; opponent-hot-only ⇒ the DEFENDER call (the meaningful direction) and the attacker call still legal (the mover may force a deeper win through the check — LAW-FORCE admits it) and cap-bounded; both-hot ⇒ attacker first, defender only if the attacker call returned no win. **STAGED-ONLY (gap (ii))**: `ThreatState` exists only under `CandidatePolicy::Staged`, so `on_search_path = true` under a Radius-kind config is REFUSED BY NAME at validation (rule 3), never a silent no-op — 4 of the 12 committed engine configs are Radius-kind and their `[solver]` sections carry the gate `false` with the same refusal standing guard. Attack on (a), recorded: *the trigger is threat-state-derived, so its cost rides `ThreatState`'s maintenance, and a hot window for EITHER side fires the call — a position where the opponent's hot window is already covered by the mover's forced Tier-F reply pays for a defender call that changes nothing.* Accepted: the call is cap-bounded and the trigger is exactly the position class where a proof can exist. |
| D2 | directions | (a) attacker only; (b) attacker + defender; (c) defender only | **(b)**, the dispatch's own default, with the defender entry spelled AS THE CODE ALREADY ADMITS IT (the red team's correction, adopted wholesale): `Solver::solve_defender(state)` is a THIN WRAPPER — the attacker is `state.to_move().opponent()`, the root is the SAME `solve_root`, and the df-pn's own to-move dispatch lands it in the existing AND path (`dfpn_and`), ZERO df-pn changes. At every D1-reachable input the AND assert passes UNRELAXED (opponent-hot means the opponent-of-the-policy-attacker — the mover — faces plans... precisely: at a defender call the policy-attacker is O and O's hot windows ARE the plans the assert wants); the zero-plan AND root is UNREACHABLE under the registered trigger, and if a future trigger ever admits it the existing assert refuses it LOUDLY — the refusal semantics come free. NOT a null-move/pass formulation: a pass-proof is OPTIMISTIC for the non-mover and unsound as a loss claim. Attack on (b), recorded: *the AND root is the widest node in the game — every legal mover pair that covers — so defender proofs complete only near conversion, and midgame calls burn their cap for `Unknown`.* Accepted and measured by the bench: that is what the per-call cap is for, and near-conversion is where the anchor's games were decided. |
| D3 | root behaviour | (a) root attacker proof answers immediately (proof's first move, mate score); root defender proof restricts the root candidate set to the proof's zone cells for the whole search; (b) no root special-casing | **(a)**, both dispatch defaults. The proof's first move: the root `OrStep`'s witness turn, or the `OrWinLeaf`'s completing stones as a turn — extracted from the emitted tree, so the move is the PROOF's, not the search's ordering. The zone restriction is a VALUE-motivated candidate restriction on a proven loss (the dispatch's own words: "a proven opponent win prunes to the zone cells as the only candidates") — registered here as such so it is never read as an ordering hint. **THE ZONE ORDER IS Z2** (`zone.order(1)`, pinned: Z1 is too tight to hold a two-stone defense's cells, Z3 absorbs every k≥3 EP-1 segment and nears the whole legal region — the paper's own two-stone cap suggests the middle order; a test pins the choice so it cannot drift silently). The operative property is CONTAINMENT, not exactness: the zone is a strict superset of the opponent's plan cells (all children's win trees union EP-1), and the Tier-F cover cells sit inside it by construction — the red team's wording correction, adopted. The move extraction keys by `tree.root` against the emitted node map — the emission is POST-ORDER (the module's own doc comment says root-first and is corrected at impl), so an implementer reading nodes[0] would read a leaf. Attack, recorded: *the zone cells might exclude every Tier-F candidate and leave the root with only Tier-Q filler.* Answered: the zone is built from the OPPONENT's plan cells — exactly where the forced defense lives — and an empty intersection leaves the candidate set untouched (fail-open on the intersection, never fail-closed; a test pins this). |
| D4 | unknown results | (a) a cap-exhausted or `NoWinUnderZone` solve returns NO information and the search continues normally; (b) treat as NoWin | **(a)** — (b) is the unsound reading: an unfinished proof is not a refutation. `Unknown` is a third solver outcome, distinct from `NoWin`, and the search treats it as "no verdict here". |

## 3. Node accounting (the fork the dispatch settled)

**Solver nodes COUNT against the per-side node budget**, and are printed
separately every turn. This SUPERSEDES wp18a_design §5/M3's separate-
budget registration — the commissioning dispatch orders the shared budget
in as many words ("Equal per-side compute is the SPRT premise"), and the
print seam lands exactly as 1.8a registered it (both counters printed).
The ADR line at closure records the supersession. Mechanics: the `Run`'s
`nodes` counter absorbs each call's solver nodes the moment the call
returns, so `Stop::Nodes` sees the shared total; the stop-contract
sentence is AMENDED, not silently changed (the red team's MINOR, adopted):
the mask check still fires only at visit entries, and between two checks
the counter can absorb up to one capped call per intervening visit, so the
budget can overshoot by that amount before the next check — deterministic
(identical on every run; D-74's exactness wording applied to the mask
mechanism, with the overshoot now bounded by the cap, not by the interval
alone). **The PER-CALL cap bounds ONE call; the aggregate solver spend is
bounded by the shared budget itself (solver nodes count against it) —
"equal per-side compute" is enforced as equal BUDGET, with the ON seat's
spend split reported per turn.** `SearchInfo` grows `solver_nodes`
(written on the `Run`, threaded through `search.rs`'s final overwrite
block so a Deadline-salvaged answer reports it too), and the CLI prints
`solver_nodes` STRICTLY AFTER `nodes` on both the `info` and `totals`
lines — PINNED BY A REPORT TEST, because `tools/sealbot`'s parser reads
`nodes` by SUBSTRING (`field_after(rest, "nodes ")`), and `"solver_nodes
300"` contains `"nodes "`: field order is load-bearing for that parser,
and the impl word-boundaries it as well (a `tools/sealbot` edit, reviewed
under the SHELL_CHECKLIST as a tools/ change; NOT an arena/replay/
Criterion-1'' change, so the dispatch's RED-TEAM trigger does not fire).
The budget-sum test pins `nodes == search_nodes + solver_nodes` exactly (a
one-node drift fails it).

## 4. Scores and the TT

An attacker proof at interior node N with witness depth `d` (attacker
turns) scores `mate_in(turns_from_root + 2d − 1)` — the mover's `d`-th
turn from N sits at root offset `t + 2d − 1`, ODD, as `score.rs`'s law
demands for a win read at the node it is written at; the parity is
asserted in a test. A defender proof at N — the OPPONENT is the
policy-game attacker, whose turns from N sit at offsets `t+2, t+4, …` —
scores `−mate_in(turns_from_root + 2d)`, EVEN, exactly the shape of the
staged generator's own OverloadReturn row (`−mate_in(turns_from_root +
2)`, the d=1 instance). **The formula's first draft was `-mate_in(t +
2d − 1)`, parity-violating (the red team's BLOCKING 1); this is the
corrected form, and the parity test checks BOTH directions.** Both store through the table's existing `to_table`/`from_table`
re-basing — no new bound kind. `NoWinUnderZone` at a call site is
`Unknown` for accounting purposes but LOUD in the report (a counter, not
a silent swallow).

## 5. Config (rule 1: complete, explicit, no code default)

Every committed engine config gains a `[solver]` section:

```toml
[solver]
on_search_path = false          # the gate; FALSE in every committed config until h1
per_call_node_cap = 16384       # ESTIMATED, from the probe's own numbers
trigger = "any_open_four"       # calculus ID: PAT-O4+ on either side
epsilon_num = 1                 # the solver's own knobs ride the same
epsilon_den = 4                 # section so one document configures one
zone_orders = 3                 # engine (SolverParams::validate is the
free_stone_radius = 8           # single validator, as in configs/solver_v0.toml)
tt_entries = 1048576
attacker_policy = "one_free_stone"
```

`per_call_node_cap = 16384` is ESTIMATED from the anchor probe's measured
win costs (1-10,726 nodes, `wp18b_probe_v1/v2_results.txt`): it clears the
flagship t42 flip (10,726) with headroom while bounding one call to a
third of the 50 000 budget. **CAVEAT, registered with the estimate
(the red team's MINOR)**: the probe measured ATTACKER-direction costs
only — no defender-direction cost measurement exists anywhere, and the
defender AND root is the widest node class; the cap's fitness for that
class is an assumption the bench's ON seat measures. **TWO VALIDATORS, one
table** (the red team's correction): `pistol-engine`'s own schema parses
the WHOLE `[solver]` section (all nine keys, `deny_unknown_fields`,
refusing `on_search_path = true` under Radius by name), and derives the
solver-internal six keys into `SolverParams` through
`pistol-solver`'s unchanged `SolverSection` validation — the engine
validator owns the wiring knobs, the solver validator owns the solver
knobs, and neither re-reads the other's literals (rule 1). **Two wiring
guards named**: the root call is guarded against the game's first turn
(`Phase::First` owing ONE stone — `Solver::solve` refuses it by panic;
the search never calls there), and `Solver` gains the wholesale reset
(`reset()` — table and epoch; `Searcher::clear` calls it) that §1's
determinism story needs as wiring code. `tools/config_check.sh` reads the
section through the engine's existing seam.

## 6. Tests (§3's obligations, restated as design)

- Root proof → the proof's first move, mate score, `solver_nodes` counted.
- Defender proof → root candidates restricted to zone cells ∩ candidates,
  nothing outside; empty intersection → candidates untouched (D3's
  fail-open, pinned).
- Budget accounting sums exactly (the one-node-drift test).
- Gate off = byte-identical to committed behaviour (determinism fixtures,
  both configs).
- Determinism gains an ON seat (D-7): `configs/gate_staged_solver_v0.toml`
  (gate on, cap and trigger as registered), run by `tools/determinism.sh`
  beside the three existing seats; newgame's solver reset is exercised by
  the C-vs-D layout comparison.
- Mutations (registered): drop the solver-node accounting → the sum test
  dies; let a defender zone leak a non-zone candidate → the restriction
  test dies; skip the newgame solver reset → the determinism seat dies
  (the reset's observable is the C-vs-D byte-identity: with the reset
  skipped, D's second game inherits solver-table state C never had — the
  receipts must demonstrate the mutant dies or record why the clear is
  unobservable, honestly).

## 7. The rule-5 bench, registered before measuring

- **Hotspot**: the per-node cost shift at trigger nodes (the anchor's own
  positions are the stress class: `bench_positions_v1.txt`, 50 000 nodes,
  both bands ON vs OFF).
- **Bracket**: band-aggregate nps ratio ON/OFF ≥ 0.5 in both bands
  (ESTIMATED: a trigger-node call costs up to the cap in solver nodes
  against 50 000 budget, but triggers are rare in the bench positions —
  the bracket is on WALL PER REPORTED NODE, which is exactly where solver
  calls land their cost); **abort** < 0.5 — a config-default-on engine
  that halves throughput at equal budget is not a candidate for h1
  regardless of what the SPRT says about the gated seat.
- IQR gate at the D-215/D-362 convention.

## 8. What this design does NOT do

No move-ordering use of solver output (values only), no quiescence change,
no arena/replay/attribution-instrument change (the `solver_nodes` field is
additive for the arena's key-positional parser; the ONE substring parser
in the tree — `tools/sealbot`'s — is made word-boundary-safe at impl and
the print order is test-pinned, per §3), no play-mode behaviour change
beyond the same gate, no strength claim. `NoWinUnderZone` at a call site
counts into `SearchInfo`'s `solver_refusals` (loud, never swallowed).
