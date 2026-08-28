# WP-1.8b §2 — the solver on the search path (design)

**Revision 3. Closes REVIEW-design's 1 BLOCKING and 5 MAJORs. B-1: the
per-call node cap is DESIGNED (§2a) instead of presumed — it does not
exist in the code, every cost claim leaned on it, and `solve_root`'s stall
guard would panic on a truncated re-search unless the spent flag is
checked first; the cap bounds VISITS and one visit's own enumeration is
O(region²) pair checks (MEASURED ~480-cell regions → ~10⁵ checks ≈
milliseconds), so the wall bound per call is cap × enumeration, stated.
M-2: call sites are TURN-BOUNDARY nodes only (`Phase::First`, two owed) —
mid-turn nodes never call (the policy game is defined at turn boundaries;
`Solver::solve` refuses mid-turn positions by panic, and the trigger is
evaluated only where a call is legal). M-3: the bench's fixture is the
WP-1.3 CORPUS fixture (the "anchor positions" attribution deleted), the
trigger frequency over it is now MEASURED — 8 of 24 positions hold a hot
window at the mover's boundary, so the bench exercises the hotspot, not
vacuously — and a second, committed, sha-pinned TRIGGER-RICH fixture
(`bench_solver_positions_v1.txt`, late-game threat shapes) benches the
stress class the corpus under-represents. M-4: `solver_nodes` prints ONLY
on gate-ON configs — the field set of every committed (gate-off) config
is unchanged, D-88's pinned order stands, and byte-identity holds; the
print order and the sealbot word-boundary fix are unchanged. M-5: TWO
INDEPENDENT COUNTERS — `search_nodes` incremented at every visit
(+quiescence), `solver_nodes` incremented per call, `nodes` DERIVED as
their sum at report time; the sum test compares two independent writers
and the drop-accounting mutation kills it. M-6: the stop contract's full
amendment (a call after the last check can let the final iteration
complete over budget with the overshoot absorbed — deterministic,
overshoot ≤ one capped call per intervening visit between checks OR the
completion overshoot; `stop.rs`'s module doc and D-74's wording are
amended at impl/closure, named here). And the MINORs: the reset's
unobservability stated once in the honest form (epoch isolation makes the
skip unobservable until 2³² solves; the mutation is registered as
PREDICTED-UNKILLABLE with the honest receipt the process allows), the
parity test asserts the t-relative invariant (attacker ≡ t+1, defender ≡
t mod 2) rather than an unconditional ODD/EVEN, SCHEMA_VERSION moves 2 →
3 (the D-16 class, named), the dependency sentence corrected (the edge
exists since WP-1.5b; what is new is CALLING the solver), and D1's
quiet-node cost sentence now matches the code (the zero-plan defender
call refuses by assert, not by enumerating).**

**Revision 2 closed the DECISION-RED-TEAM's findings (MATRIX FALLS on
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

`pistol-search` has DEPENDED on `pistol-solver` since WP-1.5b (the
manifest edge is old; `staged.rs` already consumes `ThreatState`); what
this WP adds is the first CALL of the df-pn solver from the search path.
The WP-1.5a `p = 0` posture (no binary LINKS the solver's proof
machinery) is superseded BY THIS WP, which exists to wire it, and the
WP-1.8a edge-check tool remains the adjudicator for any future claim
about who links whom.
`pistol-search` re-implements no rule (rule 2): the solver is called with
the search's own `GameState`, and every verdict flows from pistol-core's
rules through the solver's policy game.

**Where the solver lives**: in `Searcher`, constructed from the same
config seam as everything else, ONE instance per engine — its epoch TT is
reused across every call in a game, the epoch mechanism isolates solves,
and `Searcher::clear` — what a new game is — calls `Solver::reset()`
(wholesale table-and-epoch clear, wiring code §5 names). **The reset's
observability, stated once and honestly**: epoch isolation already makes
every earlier-solve entry read as absent, so a SKIPPED reset changes no
observable until the epoch counter wraps — the C-vs-D seat cannot see it,
the registered mutation is PREDICTED-UNKILLABLE, and if the receipts
confirm that, the honest record is the process's own form for it (the
attempted reproducer, the unobservability argument) — the reset stands as
memory hygiene and defence-in-depth, which is what it is. The
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

## 2a. The per-call node cap — designed, not presumed (REVIEW-design B-1)

The cap does not exist in the shipped solver; it is wiring code, and this
section is its design. `Solver::solve` grows a `node_cap: u64` argument
(explicit, no default — rule 1; the engine passes the config value, the
selftest and the probe example pass an explicit unbounded constant, and
the probe registration's instrument table is re-pinned for the signature
change). Mechanics: `Search` carries `node_cap` and a `spent` flag; at
`dfpn` entry, after the node count increments, `nodes >= node_cap` sets
`spent` and returns the node's CURRENT `(pn, dn)` unchanged — a
truncation, never a fabricated convergence — so proof numbers do not
reach 0/INF and the parents unwind on unchanged values.
**`solve_root` checks `spent` BEFORE the stall guard, every pass**: a
truncated re-descent leaves the root's `(pn, dn)` exactly where the last
pass left it, which is byte-for-byte the stall signature, and the
distinction is the flag, not the numbers — the check order is a
correctness requirement, not a style choice. The spent solve returns a
new `SolveOutcome::Unknown` (distinct from `NoWin`; D4's semantics), with
its nodes still counted. TT hygiene: a truncated solve's partial entries
are mid-search state exactly like any aborted pass, the solve ENDS (no
continuation), and the epoch isolates every later solve — no poisoning
path. **What the cap bounds, stated**: VISITS. One visit's own
enumeration is O(region²) pair checks (MEASURED ~480-cell regions → ~10⁵
checks ≈ milliseconds) for an AND node's cover filter, or |R|·|L| for an
OR node's arm B — so one call's wall bound is cap × enumeration, not cap
alone; at cap 16384 that is seconds-worst-case, and the search-path
aggregate is bounded by the shared node budget (§3).

## 2b. IMPL-TIME AMENDMENTS (REVIEW-impl's W-2/S-1/S-3/S-4, and the bench)

- **§2a AS SHIPPED (W-2)**: the truncation returns `(INF, INF)`, not "the
  node's current numbers", and the shipped safety argument is
  **spent-before-stall plus spent-means-store-nothing plus epoch
  quarantine**: the unwind never merges into a stored entry (both descent
  loops return without storing once `spent` latches — the mechanism of the
  determinism seat's `SOLVER_CHILD_ZONE` panic, reproduced and closed),
  and a spent solve returns `Unknown` regardless of its unwound numbers.
  §2a's original "numbers do not move" sentence is retired by this
  paragraph.
- **§4 AS SHIPPED (S-1)**: solver verdicts RETURN directly and store
  nothing — the safer shape (no re-basing path exists to get wrong);
  §4's "store through to_table/from_table" sentence is retired. Re-visits
  re-solve, cap-bounded, deterministic.
- **The AND-children enumeration order changed with the fast path (S-3)**:
  canonical-sorted where `generate_turns` emission order stood before —
  deterministic, value-neutral, node counts shift; df-pn's tie-break is
  enumeration-order-sensitive and the registered order spec covers it.
- **§2 D1's quiet row (S-4)**: the shipped root block calls both
  directions whenever the gate is on and the root is a two-stone boundary
  — including quiet roots, where both calls are real capped searches.
  Registered here rather than left as design silence; the bench's abort
  finding is the cost story that covers it.
- **THE §7 BENCH ABORT (the registered instrument's outcome)**:
  `artifacts/wp18b_bench_v1.txt` — OFF 223,668 nps; ON searches 9-240+ s
  at cap 16384 (2 of 5 calibration positions hit a 240 s timeout; one
  position ran >10 min), ratio ≤ 0.02 against the ≥ 0.5 corpus bound. The
  registered consequence applies verbatim: the ON seat is not a candidate
  for h1 REGARDLESS of the SPRT, and every committed config keeps the gate
  false. **The SPRT's own status is the operator's overrule record
  (docs/decisions.md, the closure lines)**.

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
block so a Deadline-salvaged answer reports it too), **and the counters
are TWO INDEPENDENT WRITERS** (REVIEW-design M-5): `search_nodes`
increments at every visit and quiescence node, `solver_nodes` increments
per solver call, and `nodes` is DERIVED as their sum at report time —
the budget-sum test then compares two independent counters, and the
drop-accounting mutation genuinely kills it. The CLI prints
`solver_nodes` STRICTLY AFTER `nodes` on both the `info` and `totals`
lines, **ONLY WHEN THE GATE IS ON** — every committed config is gate-off,
so their field sets are byte-unchanged (D-88's pinned order stands, the
golden transcripts stand, gate-off byte-identity holds) and only an ON
seat's line grows the field. The print order and a word-boundary fix for
`tools/sealbot`'s substring parser (`field_after(rest, "nodes ")` —
`"solver_nodes 300"` contains `"nodes "`) are both PINNED BY REPORT
TESTS: field order is load-bearing for that parser even though it only
ever reads gate-off seats today. The `tools/sealbot` edit is a tools/
change reviewed under the SHELL_CHECKLIST; it is NOT an arena/replay/
Criterion-1'' change, so the dispatch's RED-TEAM trigger does not fire.
The budget-sum test pins `nodes == search_nodes + solver_nodes` exactly
against the two independent writers (a one-node drift fails it).

## 4. Scores and the TT

An attacker proof at interior node N with witness depth `d` (attacker
turns) scores `mate_in(turns_from_root + 2d − 1)` — the mover's `d`-th
turn from N sits at root offset `t + 2d − 1`, ODD, as `score.rs`'s law
demands for a win read at the node it is written at; the parity is asserted in a test — as the t-RELATIVE invariant (attacker
distance ≡ t+1 mod 2, defender distance ≡ t mod 2), not an unconditional
odd/even, because the same formulas yield both parities across call sites
of either ply parity and both are correct in the root frame (D-98). A
defender proof at N — the OPPONENT is the
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

## 7. The rule-5 bench, registered before measuring (REVISION 3's form)

- **Hotspot**: the per-node cost shift at trigger nodes — measured on TWO
  committed, sha-pinned fixtures: the WP-1.3 corpus fixture
  `bench_positions_v1.txt` (24 midgame positions, both bands, MEASURED 8
  of 24 holding a hot window at the mover's boundary — the bench exercises
  trigger nodes, not vacuously) and a NEW trigger-rich fixture
  `bench_solver_positions_v1.txt` (late-game threat shapes, the class the
  corpus under-represents; committed and sha-pinned per rule 7 —
  positions, not run artifacts).
- **Bracket**: band-aggregate nps ratio ON/OFF ≥ 0.5 in both bands on the
  CORPUS fixture (the regression axis: gate-on must not halve ordinary
  throughput), and ≥ 0.25 on the TRIGGER-RICH fixture (the stress axis:
  late-game trigger-heavy searching spends solver budget by design —
  ESTIMATED, and the whole point of measuring it); **abort** if the
  corpus bracket fires (< 0.5) or the trigger-rich ratio is below 0.1 —
  a config-default-on engine that spends nine tenths of equal budget
  inside solver calls is not a candidate for h1 regardless of what the
  SPRT says about the gated seat.
- IQR gate at the D-215/D-362 convention.

## 8. What this design does NOT do

No move-ordering use of solver output (values only), no quiescence change,
no arena/replay/attribution-instrument change (the `solver_nodes` field is
additive for the arena's key-positional parser; the ONE substring parser
in the tree — `tools/sealbot`'s — is made word-boundary-safe at impl and
the print order is test-pinned, per §3), no play-mode behaviour change
beyond the same gate, no strength claim. `NoWinUnderZone` at a call site
counts into `SearchInfo`'s `solver_refusals` (loud, never swallowed).
