# WP-1.8b §2 — the solver on the search path (design)

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
| D1 | call sites | (a) root + every node whose mover OR opponent holds an open four-or-better (a hot window, PAT-O4+, by calculus ID); (b) every node; (c) root only | **(a)**. (b) multiplies per-call cost by the node count with no threat to find (the solver's policy game needs a live window to move at all — an OR node with `C = ∅` and `R = ∅` is a one-visit NoWin, so most quiet-node calls are cheap no-ops, but the DEFENDER-direction calls at quiet nodes are not: they enumerate the mover's whole legal turn set at the root); (c) forfeits the interior cutoffs that are the anchor's whole measured story (sealbot's tree collapsed on its own 1-node instant-win probes at interior decisions). Attack on (a), recorded: *the trigger is threat-state-derived, so its cost rides `ThreatState`'s maintenance, and a hot window for EITHER side fires the call — a position where the opponent's hot window is already covered by the mover's forced Tier-F reply pays for a defender call that changes nothing.* Accepted: the call is cap-bounded and the trigger is exactly the position class where a proof can exist. |
| D2 | directions | (a) attacker only; (b) attacker + defender via a new AND-root solver entry; (c) defender only | **(b)**, the dispatch's own default, with the defender entry SPELLED: a new `solve_defender(state)` on `Solver` answering "does the NON-mover force a policy-game win against the mover's best defense, the mover defending with its full legal turn set?" — the policy game with the root at an AND node over the mover's legal turns (the existing `dfpn_and` machinery with the plan assertion relaxed to admit the zero-plan root, where the cover filter is vacuous and every legal mover turn is a child). NOT a null-move/pass formulation: a pass-proof is OPTIMISTIC for the non-mover and unsound as a loss claim. Attack on (b), recorded: *the AND root is the widest node in the game — every legal mover pair — so defender proofs complete only near conversion, and midgame calls burn their cap for `Unknown`.* Accepted and measured by the bench: that is what the per-call cap is for, and near-conversion is where the anchor's games were decided. |
| D3 | root behaviour | (a) root attacker proof answers immediately (proof's first move, mate score); root defender proof restricts the root candidate set to the proof's zone cells for the whole search; (b) no root special-casing | **(a)**, both dispatch defaults. The proof's first move: the root `OrStep`'s witness turn, or the `OrWinLeaf`'s completing stones as a turn — extracted from the emitted tree, so the move is the PROOF's, not the search's ordering. The zone restriction is a VALUE-motivated candidate restriction on a proven loss (the dispatch's own words: "a proven opponent win prunes to the zone cells as the only candidates") — registered here as such so it is never read as an ordering hint. Attack, recorded: *the zone cells might exclude every Tier-F candidate and leave the root with only Tier-Q filler.* Answered: the zone is built from the OPPONENT's plan cells — exactly where the forced defense lives — and an empty intersection leaves the candidate set untouched (fail-open on the intersection, never fail-closed; a test pins this). |
| D4 | unknown results | (a) a cap-exhausted or `NoWinUnderZone` solve returns NO information and the search continues normally; (b) treat as NoWin | **(a)** — (b) is the unsound reading: an unfinished proof is not a refutation. `Unknown` is a third solver outcome, distinct from `NoWin`, and the search treats it as "no verdict here". |

## 3. Node accounting (the fork the dispatch settled)

**Solver nodes COUNT against the per-side node budget**, and are printed
separately every turn. This SUPERSEDES wp18a_design §5/M3's separate-
budget registration — the commissioning dispatch orders the shared budget
in as many words ("Equal per-side compute is the SPRT premise"), and the
print seam lands exactly as 1.8a registered it (both counters printed).
The ADR line at closure records the supersession. Mechanics: the `Run`'s
`nodes` counter absorbs each call's solver nodes the moment the call
returns, so `Stop::Nodes` sees the shared total on its existing
power-of-two mask check (D-74); `SearchInfo` grows `solver_nodes`, the
CLI's `info`/`totals` lines print `solver_nodes` beside `nodes`
(key-positional parsing in the arena reads `nodes` by name and ignores the
new key — verified against `exchange.rs` — so no arena/replay code moves,
and the RED-TEAM trigger the dispatch registered does not fire). The
budget-sum test pins `nodes == search_nodes + solver_nodes` exactly (a
one-node drift fails it).

## 4. Scores and the TT

An attacker proof at interior node N with witness depth `d` (attacker
turns) scores `mate_in(turns_from_root + 2d − 1)` — the mover's `d`-th
turn from N, counting the interleaved defender turns (rule 4's unit); the
parity check (odd distance for a win from the node the score is written
at, `score.rs`'s own doc) is asserted in a test. A defender proof at N
scores `-mate_in(turns_from_root + 2d − 1)` with `d` the defender-proof's
depth. Both store through the table's existing `to_table`/`from_table`
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
flagship t42 flip (10,726) with headroom while bounding a defender call to
a third of the 50 000 budget. `pistol-engine`'s validator grows the
section; `tools/config_check.sh` reads it through the existing seam.

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
no arena/replay/attribution-instrument change (the additive `solver_nodes`
field is ignored by every existing parser by construction), no play-mode
behaviour change beyond the same gate, no strength claim.
