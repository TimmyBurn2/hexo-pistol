# WP-1.8c — the solver call at viable cost: design (revision 2)

**Provenance.** Revision 1 (`005395c2`) went to a fresh-context REVIEW-design
and a fresh-context DECISION-RED-TEAM over its matrices. REVIEW-design returned
**FAIL** — 1 BLOCKING, 8 MAJOR, 10 MINOR — while independently CONFIRMING every
substantive claim it could check: leg 1 correct at every reachable node
(184.6 M child keys, 0 wrong), leg 3's exhaustiveness argument sound (23.3 M
filter decisions, 0 disagreements), leg 4 differentially verified against the
committed implementation at the `i16` lattice edges (1,000+ boards, 0
mismatches), §4b's loser-win finding right and re-reproduced, §4c's TT
diagnosis right in every number. The red team felled **Rule C** outright and
holed three rows of M6. This revision is the ONE fix round the dispatch
licenses. It fixes all nineteen findings, and it carries three things revision 1
did not have: a leg ladder with a named revision per row (§2.2), and two
INHERITED DEFECTS the reviews turned up (§4d, §4e).

**What changed in substance, not only in prose.** Revision 1 registered a cap
and expected the bench to decide. Two independent measurements — the red team's
and then this session's own, on a separately written driver — have since put
the ON/OFF corpus ratio at **0.044** against the registered ≥ 0.50 bound (§6b).
The bracket is unchanged and the bench is still run; what changed is that this
design no longer pretends not to know which way it points.

## 1. The question, and the one thing this WP is allowed to move

WP-1.8b's bench measured the ON seat at nps ratio ≤ 0.02 against a ≥ 0.5 bound.
Nothing about the solver's VALUES is in dispute — four oracle gates are green
(D-437), the wiring's own gates are green (D-441), the determinism seat is
byte-identical. What is in dispute is the WALL COST OF ONE df-pn VISIT.

The four legs of §3 move that number and nothing else: **each is required to
leave the solver's answers, node counts, seesaw counts and proof digests
bit-identical**, and §5 is how that is checked rather than asserted.

**Three further changes are in this WP and do NOT carry that property**, each
because the reviews found a defect that would make the bench unreadable or the
seat illegitimate. They are named here so the identity property's scope is not
mistaken for the WP's:

| § | what | why it is in scope |
| --- | --- | --- |
| §4c | the TT replacement law is not epoch-aware | a solver whose answers decay with call count is not a solver this WP can bench |
| §4d | the node budget is not enforced once solver nodes are absorbed | a seat that spends 3× its budget is not an SPRT arm, and rule 6's premise is equal per-side compute |
| §4e | the trigger-rich bench fixture is neither sha-pinned nor loadable | §6's trigger-rich band has never been runnable, so half the registered bracket does not exist |

None of the three was in the commissioning dispatch's scope. All three were
found by instruments this WP built, and each blocks the WP's own measurement.

## 2. The measured cost picture

### 2.1 The instruments — five trees, five revisions, and which produced what

**This is BLOCKING B-1's fix, and it is the reason §2.2 exists in this form.**
Revision 1 named ONE revision for numbers produced by three different trees, and
the review proved it: the registered revision contained §4c's TT change while
every endpoint quoted had been taken by a tree that predates it. A revision that
cannot reproduce the number it governs is not a governing revision.

The instrument is `crates/pistol-solver/src/bin/solver-cost.rs` plus four
scratch `std::time::Instant` timers in `dfpn.rs` and `policy.rs`
(`Search::child_keys`, `policy::threat_pairs`, `policy::blocking_pairs`, and the
child-selection scan in both descent loops), which print the `ATTRIB`, `BP` and
`TP` lines. `docs/experiments/wp18c_instruments.md` carries every command block verbatim.
**The timers are SCRATCH and come out before the IMPL commit; the instrument
itself does NOT** (m-9): §6's REPRODUCTION bracket has to be readable at the
IMPL revision, so `solver-cost` ships as a binary beside `solver-selftest`, and
it carries a driving test — one `case` line per fixture case and a `TOTAL`
whose node count is their sum, so a binary that printed a total without the
cases would fail rather than exit 0 with a number the bracket would read.

The ladder is five trees, each snapshotted by `git stash create` in a DETACHED
worktree (`/home/tom/wp18c-ladder`) built from the WP's base commit `4569c1a`,
each built into its own `CARGO_TARGET_DIR`, each measured over both fixture sets
at cap 4096 under `configs/solver_v0.toml`:

| tree | what it is | `git stash create` revision |
| --- | --- | --- |
| **T0** | the committed sources at `4569c1a` + the instrument + the timers | `18b3d8a21bd735b9062f2d474493853a2ba47d92` |
| **T1** | T0 + leg 1 (child keys) + leg 2 (hoist) | `277ae0cdfe2bacf7ba3ada43469006b86194e256` |
| **T2** | T1 + leg 3 (arm A predicate) | `e0c99ce4ad16e2b88c608e9f22020e3e8289616e` |
| **T3** | T2 + leg 4 (region row sweep) — **the four legs, complete** | `654abd4362560e7469ae48e42c390253b8bc31dc` |
| **T4** | T3 + §4c option B (epoch-aware replacement law) | `9eb1245ba9482dde24980b9c34eb71e7209fcf83` |

The timers cost what they cost and it is not hidden — but only ONE side of it
was ever measured, and revision 2 said otherwise before its own re-review
caught it. T0 measures **2,904.08** µs/visit on CORPUS with the timers and an
earlier scratch run measured **2,630.11** without them, so on the committed
sources they cost about **a tenth**. **No untimered run of the FAST side was
ever taken**: `artifacts/wp18c_cost_corpus_fastpath_v1.txt` prints `ATTRIB`
lines, which a build without the timers cannot do. Every ratio in §2.2 is
therefore taken WITHIN the instrumented ladder, where the overhead cancels and
every row has a revision, and **§2.2's T0-to-T3 rows are the endpoints this
document stands on and the referent §6's reproduction bracket is written
against** — stated here once, and pointed at from §6.

Two fixture sets, both derived from committed material:

- **CORPUS** — the 24 positions of `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt`,
  the rule-5 bench's own regression fixture.
- **ANCHOR** — the 85 positions of the WP-1.8b anchor probe, regenerated by
  `tools/wp18b_probe_extract.py` from `artifacts/sealbot_anchor_v1/g00{1,2}.jsonl`.
  **Provenance checked rather than assumed** (m-7): `MANIFEST.sha256` reproduces
  D-438's registered digest `e2821749147a909294a1fa93c012896de76127f23678664705ebab4623f0d3a6`,
  and `g001.jsonl` / `g002.jsonl` reproduce their manifest entries
  `6b7651e1…` / `9a318760…`.

A third set, **TRIGGER-RICH**, is the 20 positions of
`bench_solver_positions_v1.txt` — see §4e for what is wrong with it and what
this WP does about it.

### 2.2 The leg ladder — one row per leg, one revision per row

All rows: cap 4096, `configs/solver_v0.toml`, both fixture sets, the same
detached worktree, one build per row.

| stage | corpus µs/visit | anchor µs/visit | corpus nodes | anchor nodes | leg's own ratio (corpus / anchor) |
| --- | --- | --- | --- | --- | --- |
| T0 | 2,904.08 | 2,200.84 | 72,794 | 263,640 | — |
| T1 | 481.84 | 343.10 | 72,794 | 263,640 | **6.03× / 6.41×** |
| T2 | 191.72 | 215.01 | 72,794 | 263,640 | **2.51× / 1.60×** |
| T3 | 91.80 | 85.02 | 72,794 | 263,640 | **2.09× / 2.53×** |
| T3 vs T0 | | | | | **31.63× / 25.89×** |
| T4 | 95.25 | 92.36 | 72,794 | **256,292** | 0.96× / 0.92× |

**The node counts are the identity property, measured.** 72,794 and 263,640
appear on every row T0 through T3 — the four legs do not move a single visit.
They move at T4 because §4c's change is a VALUE change and says so.

**T4 is slower per visit and that is reported, not buried** (the red team's
finding against revision 1): option B costs 3.8 % on CORPUS and 8.6 % on
ANCHOR, while cutting ANCHOR's node count by 2.8 % and turning three
non-answers into answers (§4c). It buys strength with wall.

**What each leg removes, read off the same runs' `ATTRIB` / `BP` / `TP` lines**
(all **MEASURED**, all at the revision in the row above):

| stage | fixture | `child_keys` | `threat_pairs` | `blocking_pairs` | selection scan |
| --- | --- | --- | --- | --- | --- |
| T0 | CORPUS | **79.6 %** | 11.0 % | 6.6 % | 2.6 % |
| T0 | ANCHOR | **78.3 %** | 7.2 % | 10.4 % | 3.8 % |
| T1 | CORPUS | 1.1 % | **64.9 %** | 21.9 % | 11.0 % |
| T1 | ANCHOR | 1.1 % | **45.5 %** | 37.5 % | 14.6 % |
| T2 | CORPUS | 2.6 % | 12.5 % | **55.8 %** | 26.7 % |
| T2 | ANCHOR | 1.8 % | 9.1 % | **63.0 %** | 24.2 % |
| T3 | CORPUS | 5.6 % | 5.3 % | 27.0 % | **57.3 %** |
| T3 | ANCHOR | 4.6 % | 3.3 % | 26.1 % | **61.3 %** |

Read down the diagonal: leg 1 takes `child_keys` from ~79 % to ~1 %; leg 3
takes `threat_pairs` from 65 %/46 % to ~10 %; leg 4 takes `blocking_pairs` from
56 %/63 % to ~27 %. What is left at T3 is the child-selection scan, which is
M6 row C's subject.

Two sub-terms worth naming because they answer the commissioning dispatch
directly. **Arm B's own construction is 0.15 % of the T1 CORPUS wall** — the
registered hotspot's `|R|·|L|` count is real but it is paid in `child_keys` one
layer later, exactly as `wp18b_m4_design.md` §2 said it would be. And **arm A's
apply/undo filter is 93.0 % of `threat_pairs` at T1**, which is what leg 3
removes.

**Why the ladder is the per-leg instrument and the nps bench is not** (M-6): the
node count is held IDENTICAL across every row by construction, so µs/visit is a
clean per-leg measure. An nps ratio over a changing node mix is not. §6 registers
a bracket per leg from this ladder, which is what rule 5 asks for and what
revision 1 left as a receipt rather than a gate.

### 2.3 The endpoints, and the identity receipts

**The endpoints are §2.2's own T0 and T3 rows**, because those are the two
measurements taken by the same instrument, on the same machine, at NAMED
revisions, with the timers on both sides:

| fixture set | T0 (`18b3d8a2`) | T3 (`654abd43`) | ratio |
| --- | --- | --- | --- |
| CORPUS (24 positions, 72,794 visits) | **2,904.08 µs/visit** | **91.80 µs/visit** | **31.63×** |
| ANCHOR (85 positions, 263,640 visits) | **2,200.84 µs/visit** | **85.02 µs/visit** | **25.89×** |

**A second pair exists and is NOT the headline, for a reason worth recording.**
Earlier scratch runs measured the committed sources at **2,630.11** (CORPUS)
and **1,975.53** (ANCHOR) µs/visit against the four legs at **88.53** and
**81.70** — 29.71× and 24.18×. Those four receipts are in `artifacts/`, but the
two committed-side files carry no `ATTRIB` lines and the two fast-side files
do, so the pair mixes an UNTIMERED numerator with a TIMERED denominator: it is a
conservative LOWER BOUND on the gain and not a like-for-like measurement, and
the trees that produced it were scratch build directories that were never
snapshotted. **Revision 2 called that pair "timer-free" on both sides and made
it the headline; its own re-review falsified the label from the receipts, and
this paragraph is the correction.** Nothing this document concludes now rests
on those four numbers.

**And the answers did not move.** Every case of both fixture sets returned the
identical `value`, `nodes` and `seesaw` at T0 and at T3 — 109 cases, **0
differing** — visible in §2.2's own node columns and checked by the join in
`wp18c_instruments.md` §3 rather than by inspection. Two further receipts, one
internal and one not:

- REVIEW-design drove the committed apply/undo filter ALONGSIDE leg 3's
  predicate over CORPUS, TRIGGER-RICH, ANCHOR and both pinned fixtures:
  **23.3 M filter decisions, 0 disagreements**; and compared every constructed
  child key against the apply/undo key: **184.6 M children, 0 wrong keys**.
- The externally derived one: under `configs/solver_v0_narrow.toml` the fast
  path reproduces `wp18b_probe_v1_results.txt`'s
  `g002-t10-p2 value win nodes 1599 seesaw 1543` and
  `g002-t12-p2 value win nodes 397 seesaw 387` — two lines written down by a
  build that predates every leg here, so no leg can have produced them.

## 3. The fast path — four legs, one property

Every leg is OUTPUT-IDENTICAL, not merely equivalent in intent. The property,
stated once here and pointed at from §5: *for every position, the move sets
emitted, their ORDER, the child keys, and therefore the node counts, seesaw
counts, verdicts and proof digests are bit-identical to the committed
implementation's.* §4c, §4d and §4e do NOT carry it and are gated as the value
changes they are.

### Leg 0 — arm B is NOT touched, and the rule-5 subtlety on the OR side

The commissioning dispatch asks for arm B to be "constructed from minimal
covers, mirroring the AND-side precedent", and warns to watch the AND side's own
rule-5 subtlety there — "a pair with one cell outside `R` reachable through the
cell inside it". Both are answered rather than silently declined.

**Arm B has no scan to replace.** The AND-side precedent replaced a
FILTER-A-SCAN (~10⁵ legal pairs, one window check each) with a construction. Arm
B is already a construction, and §2.2 measures what it costs. What the `|R|·|L|`
count actually buys is paid by legs 1 and 4, exactly as `wp18b_m4_design.md` §2
registered ("the child-key apply/undo cost one layer later is real and
counted"). There is no covers-shaped rewrite of arm B that removes work.

**The subtlety is real, it is on the AND side only, and the asymmetry is by
kind.** A first stone OPENS ITS OWN BALL, so `{a, x}` with `x` outside the
CURRENT region but within `LEGAL_RADIUS` of `a` is a legal turn — the 12-pair
divergence WP-1.8b's inline equivalence check caught live. The AND side MUST
emit those: a defense the solver never enumerates is a defense the proof never
beats, and missing one makes a WIN claim unsound. The OR side must not and does
not: `L` is defined by `wp18b_m4_design.md` §2 as "the empty cells of the rule-5
legal region", the shipped arm B reads exactly that, and a narrower ATTACKER
move set weakens only the policy-relative `NoWin` the solver already claims — it
can never manufacture a win. **This design changes neither side's set.**

### Leg 1 — child keys by zobrist delta (`dfpn.rs`)

`child_keys` learns each child's key by playing the turn on BOTH the `GameState`
and the `ThreatState` and taking it back. pistol-core's key is a pure XOR of
`cell_key` per stone, `side_key(to_move)` and `phase_key(phase)`, so:

```
child_key = parent_key
          ^ side_key(mover) ^ side_key(mover.opponent())
          ^ cell_key(first, mover) ^ cell_key(second, mover)
```

**THE PRECONDITION IS THAT THE TURN DOES NOT COMPLETE A WIN — not that both
stones are placed** (M-1: revision 1 named the wrong one, and named it wrong at
the one place where a wrong name is a silent wrong key). `GameState::place`
sets `phase = Phase::First` on a win and **returns before flipping
`to_move`**, so a pair whose SECOND stone completes six places both stones,
lands at `Phase::First`, and does not flip the side — the formula is then wrong
by exactly `side_key(mover) ^ side_key(opponent)`. REVIEW-design constructed
that position and confirmed the divergence. "Both stones are placed" is
therefore necessary and NOT sufficient.

The sufficient condition holds at every node the solver reaches, and it holds
because of the guards above the call sites: a turn completes six only through a
live window at `own ≥ 4` containing one of its cells; at an OR node
`can_win_this_turn(attacker, StonesLeft::Two)` has already answered and returned
a leaf proof, and at an AND node `can_win_this_turn(defender, StonesLeft::Two)`
has already answered and returned a leaf disproof. `query.rs`'s implementation is
complete for two-stone completions — it fires on `own == 5` through the
win-in-one-ply set and on the least hot window at `own_count == 4`, and `own ≥ 6`
means the position is already decided, which the solver's own precondition
refuses. So `None` from that call means no turn of that mover completes six.
MEASURED over CORPUS, TRIGGER-RICH and ANCHOR: **184.6 M children, 0 wrong keys,
0 decided children, and `hot_already` fired at 0 OR nodes** — the same fact from
the other side.

**A second, unstated precondition, recorded rather than left latent** (m-10):
`policy::turn_cells(Turn::Single(at))` returns `[at, at]`, so the two `cell_key`
XORs cancel and a single-stone turn would key as the parent with the side
flipped. No policy emits one — `blocking_pairs` says so in as many words and
`threat_pairs` builds only pairs — but the formula's silence about it is the
kind that becomes a defect when a later policy does.

Legality is not checked by this leg and does not need to be: `apply_turn` still
plays the child that is actually descended into, and its `SOLVER_ILLEGAL_TURN`
panic still fires there.

### Leg 2 — `legal_placements` hoisted out of the cover loop (`policy.rs`)

`blocking_pairs` calls `pistol_core::legal_placements(state.board())` INSIDE the
minimal-cover loop, so a node with `k` one-cell covers pays the region
construction `k` times. Hoisted, it happens once. Its check is the existing
`blocking_pairs` `debug_assert` against the spec form, which §5.2 keeps
untouched (m-2).

### Leg 3 — arm A's filter becomes a predicate (`policy.rs`)

At an OR node past `can_win_this_turn`, the attacker holds NO live window at
`own ≥ 4`, so every live attacker window has `own ≤ 3`. Placing two attacker
stones reaches `own ≥ 4` two ways and no others:

- a live `own == 3` window gains one of the cells — that cell is then in
  `cells_raising_to_hot(attacker, NearHot::Three)`, call it `R`;
- a live `own == 2` window gains BOTH cells — the pair is then two of that
  window's four empties.

`own ≤ 1` plus two stones is at most three; a window dead for the attacker
cannot be revived by attacker stones; own counts never decrease. `sets.rs`'s
classes NEST — win-in-one-ply and completed are SUBSETS of hot, and
`hot = live && own ≥ 4` — so "hot" means exactly `own ≥ 4 && live` and the two
routes are exhaustive. The filter becomes, for candidates `c1 < c2`:

```
hot_already  ||  c1 ∈ R  ||  c2 ∈ R  ||  {c1,c2} ⊆ empties(w) for some live own-2 window w
```

`R` and the own-2 windows come from existing public queries
(`cells_raising_to_hot`, `live_windows_at_count`); the joint pairs are collected
once per node into a sorted vector and searched by binary search. **The
iteration order is unchanged** — the same `i < j` walk over the same
`candidates` vector — so df-pn's first-minimum tie-break sees the same sequence.

`hot_already` is the general statement of the predicate for the case the
argument above rules out at a search node. It is NOT dead: `threat_pairs` is
public and the three-site agreement test drives it directly, where 58 of the 61
bounded fixture cases DO hold a hot window for the policy attacker (REVIEW-design
measured this). Removing it changes those 58 drives.

Leg 3 also leaves **no residue**: the committed filter's transient
`threat.apply`/`undo` round-trips exactly — `WindowTable::set` removes vacant
entries and the class sets are sorted vectors — so deleting it cannot shift any
later query's order.

### Leg 4 — `region_cells` by row sweep (`movegen.rs`)

The union of radius-8 balls is built by inserting `|stones| × 217` cells into a
`BTreeSet`. It is instead swept by row: for each row `q` from `min_q − 8` to
`max_q + 8`, each stone within 8 rows contributes the r-interval
`[r + max(−8, −8 − dq), r + min(8, 8 − dq)]` (the axial ball's own arithmetic,
`dq = q − stone.q`); the intervals are sorted, merged, and emitted ascending.
Stones arrive from `Board::stones()` already ascending by `(q, r)`, so the row
window is a two-pointer walk. Cells off the addressable lattice are excluded by
clamping the interval to `i16`'s range and the row range by saturating
arithmetic — the same exclusion `checked_offset` performs (D-47), reached by the
same reasoning rather than by the same call.

Emission is rows ascending, `r` ascending within a row, which is `Coord`'s own
`(q, r)` lexicographic order, and merged intervals are duplicate-free — so the
result is the `BTreeSet`'s sequence, element for element. REVIEW-design verified
this differentially over 1,000+ boards including single stones and stone pairs
at every combination of `q, r ∈ {i16::MIN, MIN+1, MIN+7, MIN+8, −1, 0, 1,
MAX−8, MAX−7, MAX−1, MAX}` and rows jammed against both `r` edges: **0
mismatches, strictly ascending everywhere.**

**This is the one leg outside `pistol-solver`, and the blast radius is larger
than revision 1 said** (m-4). `region_cells` serves `generate_turns` and
`legal_placements`; downstream of those are the movegen perft oracle, the
`legal_placements`-vs-`is_legal_placement` agreement test,
`pistol-solver/src/zone.rs`'s `ep1_contribution` — **per proof node and
VALUE-BEARING**, since it builds the zone the containment tripwire and D3's root
restriction read — `policy.rs`'s arm-B free set, `policy.rs`'s debug spec scan,
`pistol-arena`'s stub engine, and TWO partner-lookup sites in `search.rs`
(`:588`, `:647`), not one. Every one of them is covered by the identity
property, which is why the enumeration is a coverage argument and not a risk
list. It is NOT on the search's hot path: `pistol-search` reaches
`legal_placements` only through `candidates::within_radius`'s empty-board arm
and those two partner lookups, and calls `generate_turns` not at all.


## 4. Matrix M6 — what the fast path is allowed to be

Every numeric claim is marked. The rows the red team holed are repaired with
measurement, not with argument.

| # | option | cost | failure mode | verdict |
| --- | --- | --- | --- | --- |
| A | Legs 1+2 only | `dfpn.rs` and `policy.rs`, ~30 lines, no `pistol-core` change | leaves arm A and `region_cells` standing; **MEASURED** at T1 (§2.2) | REJECTED — one order of magnitude short |
| A′ | Legs 1+2+3 | `pistol-solver` only — the whole sweep stays inside the crate the WP owns | **MEASURED** at T2 (§2.2). This is the row revision 1 omitted, and it is the one that prices leg 4: the difference between this row and B is exactly what touching the rules crate buys | REJECTED, and the price is printed rather than argued |
| B | Legs 1+2+3+4 (§3) | two crates, four sites, one `pistol-core` change | leg 4's blast radius is movegen (§3 leg 4 enumerates it); a wrong region is a RULES defect, not a perf defect. Mitigated by the perft oracle, the `legal_placements` agreement test, and REVIEW-design's own differential over 1,000+ boards at the lattice edge (0 mismatches) | **RECOMMENDED** |
| C | B, plus a compact TT entry (zone boxed out of line, the two slots bucketed adjacently) | `tt.rs` layout change | the child-selection scan is **MEASURED** 57.3 % (CORPUS) / 61.3 % (ANCHOR) of the post-B wall — §2.2's T3 row, recomputed from the `ATTRIB` lines this row cites; revision 2 printed 56.4/60.4 here, which is revision 1's `legal_placements` share at a different stage, relabelled. This is the only lever on that term. **What revision 1 got wrong**: it defended the deferral with an unsourced "~1.4×" marked MEASURED — a D-291 breach, withdrawn. The honest bound from the same `ATTRIB` lines is that removing the scan ENTIRELY would give **2.34×** (CORPUS) / **2.58×** (ANCHOR), and a layout change gets some fraction of that — **ESTIMATED**, and deliberately not measured, because §6b has since made the question moot: at `f = 0.976` a 2.5× on the whole visit moves the ON/OFF ratio from 0.044 to 0.11 against a 0.50 bound | DEFERRED — not because it is small, but because it cannot reach the bracket either |
| D | Narrow arm B's free-cell set (a policy narrowing) | small | it changes the move SET, so policy-relative `NoWin` answers change. **Revision 1 cited D-6/D-52 for this and that citation was wrong** — those govern `pistol-core` movegen, not the solver's `attacker_policy`, which WP-1.8b itself widened through M4. The real reason is scope and gates: it is a VALUE change needing its own matrix, its own red team and a full oracle re-run, exactly as §4c's TT change needed and got | DEFERRED with the correct authority, not refused "by kind" |
| E′ | A solver-local region builder, so the sweep stays inside `pistol-solver` | small | it would remove row B's own stated failure mode — no `pistol-core` change, no movegen blast radius — and it is REFUSED by hard rule 2, not by cost: game geometry and legality live in `pistol-core` only and no other crate re-implements them. **Named because the rule is the reason, and a matrix that omits the option omits the rule** (the red team's finding) | REFUSED by rule 2 |
| F | `blocking_pairs`' internal sort, the second post-B hotspot | small | **MEASURED** in this WP's own receipts: 1,051,868 of 6,444,406 µs (CORPUS) and 2,930,797 of 21,540,318 (ANCHOR) — 16.3 % / 13.6 %, second only to the selection scan. A merge of the already-sorted per-cover runs would take most of it | DEFERRED for row C's reason: at `f = 0.976` it cannot reach the bracket either |
| E | Cache `legal_placements` per position key inside `Search` | small | df-pn re-visits positions heavily (**MEASURED** seesaw 4,090 of 4,096 visits), so the hit rate would be high, and the red team is right that a cache on that workload is worth more than leg 4's **MEASURED** 16.3× on the same term. **Revision 1's "leg 4 is the same win" was wrong by roughly an order of magnitude.** What survives: rule 2 puts the legal region in `pistol-core`, so the cache would still need pistol-core's answer to cache; it adds search-carried state to the determinism surface; and §6b makes the extra factor moot for the same reason as row C | DEFERRED, with revision 1's dismissal withdrawn |

## 4b. Matrix M7 — the per-call visit cap, and why Rule C fell

**Rule C as revision 1 wrote it is WITHDRAWN.** Its algebra was right and its
conclusion did not follow from it. The algebra: both seats spend the same
50,000-node budget, solver nodes count against it (D-441), so with a fraction
`f` of the budget inside the solver the nps ratio is `s / ((1−f)s + f·c)` with
`s = 4.47 µs` the OFF seat's wall per search node, and `≥ 0.5 ⟺ f ≤ s/(c − s)`.
At `c = 88.53` that is `f ≤ 0.0532`, i.e. 2,658 solver visits per search.

Three things break the step from there to a cap of 2048:

1. **`f · 50,000` bounds the WHOLE SEARCH's solver spend, not one call.** It
   forbids caps above 2,658 and selects nothing below. Revision 1's sentence —
   "a single call that spends more than the bracket permits the entire search is
   a call the bracket cannot survive even once" — is a necessary condition
   dressed as a selection rule, and §4b expressly forbade the ladder's receipts
   from selecting. Nothing selected 2048 except "the largest admissible".
2. **The wiring makes TWO capped calls per triggered node**, attacker and
   defender (`pvs.rs::solver_verdict`), and the root does the same. One
   triggered node can therefore spend `2 × cap` — 4,096 visits at the registered
   cap, more than the 2,658 the bracket allows the entire search. Revision 1
   knew about both directions and filed the fact under "makes `c` an
   under-estimate", which is a per-visit term; it is a per-node COUNT term.
3. **A one-visit solve is not one search node.** The ladder's own receipts show
   one-visit calls at 1.4–2.0 ms on late-game positions — the leaf's zone
   construction, `ep1_contribution`'s region scan and the emission walk — while
   the `f · c` model charges it as a single 4.47 µs node. A ~450× mischarge with
   no term in the model.

**What replaces it.** The cap is not the lever, and §6b's measurement settles
that empirically rather than by argument: at cap 2048 the corpus ratio is 0.044
with `f = 0.976`, and the cap sweep in §6b shows what happens across
`{32, 128, 512, 2048}`. **The registered cap for the bench seat is therefore
chosen for a different job than reaching the bracket** — it is the largest cap
whose per-call worst case a 0.5 s deployment turn could absorb twice, which at
the MEASURED in-search per-solver-node wall of 102 µs is
`0.5 s / (2 × 102 µs) ≈ 2,450` visits, i.e. **2048**, unchanged in value and
honest in derivation. Its receipts are §4b's ladder table below and §6b's sweep.
**Where it lands** (m-1): `per_call_node_cap` is 16384 in twelve committed
configs and 512 in `gate_staged_solver_v0.toml`, the determinism seat. The
twelve are gate-off, where the value is inert; the registered 2048 lands in the
BENCH seat config this WP authors and nowhere else, and the determinism seat's
512 does not move — it is sized for CI wall, not for strength, and D-441 says so.

**The ladder's receipts** (`artifacts/wp18c_cap_ladder_committedTT_v1.txt`,
**MEASURED** at the committed replacement law — see §4c for why that
qualification is load-bearing):

| cap | ANCHOR win/nowin/unknown | CORPUS | TRIGGER-RICH | ANCHOR µs/visit |
| --- | --- | --- | --- | --- |
| 128 | 3 / 14 / 68 | 1 / 4 / 19 | 3 / 3 / 14 | 91.21 |
| 256 | 3 / 14 / 68 | 1 / 4 / 19 | 3 / 3 / 14 | 87.67 |
| 512 | 3 / 15 / 67 | 1 / 4 / 19 | 3 / 3 / 14 | 88.68 |
| 1024 | 4 / 16 / 65 | 1 / 4 / 19 | 4 / 3 / 13 | 86.72 |
| **2048** | **4 / 17 / 64** | **2 / 5 / 17** | **4 / 3 / 13** | **83.69** |
| 4096 | 3 / 23 / 59 | 2 / 5 / 17 | 4 / 4 / 12 | 82.59 |
| 16384 | 3 / 24 / 58 | 2 / 4 / 18 | 4 / 6 / 10 | 84.50 |

Two things the table says that are worth reading rather than skimming. The
per-visit wall is FLAT across the ladder (82.6–91.2 µs on ANCHOR), so `c` is not
a number that moves with the cap it is being used to choose. And the verdict
counts are NOT monotone in the cap — ANCHOR loses a win between 2048 and 4096,
CORPUS loses a nowin between 4096 and 16384 — which is impossible for a
per-position quantity and is §4c's subject.

**The loser-win proofs do not fit, and the design says so.** The commissioning
dispatch names them at 1,599 and 397 nodes. **Those are v0's numbers, not the
committed policy's**: `wp18b_probe_v1_results.txt` is the `both_stones_relevant`
run, and under M4 — `one_free_stone`, the committed `attacker_policy` — the same
two positions are `wall-cap` in `wp18b_probe_v2_results.txt`, which the anchor
probe recorded at the time ("t10/t12 became wall-caps (value-monotone: still
winnable, unproven within 60 s)"). **MEASURED this session**, fast path, no wall
cap: g002-t10-p2 wins at **14,673 nodes** and g002-t12-p2 at **3,904** — 9.2× and
9.8× their v0 counts, independently re-reproduced by REVIEW-design. No cap this
WP registers reaches either. The dispatch's instruction is discharged by saying
it plainly rather than by moving the cap: **under the committed attacker policy,
the search path cannot see the two loser-wins that motivated this WP.**


## 4c. INHERITED DEFECT 1 — the TT replacement law is not epoch-aware

Running the cap ladder produced a result the ladder was not looking for: on
ANCHOR, `g002-t39-p1` returns `win` at 714 nodes under caps 1024 and 2048, and
`unknown` at 4,096 visits under cap 4096. **A larger cap lost a proof that costs
714 nodes — a third of the smaller cap that found it.** That is impossible if a
solve depends only on its position and its cap.

**Reproducer, minimal, and it exonerates every leg in §3.** The same position,
the same cap 4096, the same config, on BOTH the committed binary and the
prototype:

| how it is run | verdict |
| --- | --- |
| alone in its own fixture directory | `win`, **714 nodes**, seesaw 712 |
| as the 61st case of the 85-position ANCHOR set | `unknown`, **4,096 nodes**, seesaw 497 |

**The mechanism, read off `tt.rs`.** One `Solver` keeps ONE table across solves
and isolates them by EPOCH: a stale entry reads as absent to `lookup`, which is
what makes the reuse safe and what made gate (c)'s 145k solves affordable
(D-437). `store`'s replacement law does not share that reading. `may_replace`
refuses when the occupying entry `is_proven()` and the newcomer is not — with no
epoch guard on that clause — and refuses again when the newcomer's `generation`
is lower. Both treat a STALE entry as worth protecting. A stale proven entry is
therefore invisible to every lookup and unevictable by every DISPLACEMENT — only
a later store of its own key can dislodge it — and the live table shrinks with
every solve that precedes the current one. The module's own doc anticipates
staleness for the same-key REFRESH path and not for the DISPLACEMENT path.

**The scope is worse than a single search.** `Searcher` builds one `Solver` and
resets it only at `newgame`, so the silting is across MOVES within a game, not
merely within a search. The determinism gate cannot see it (a fresh process per
seat); the arena would.

**What it is and what it is not.** Not unsoundness: `Unknown` is not `NoWin` and
never launders as one (D-441). Not a D-7 violation: the outcome is a
deterministic function of the solve SEQUENCE and the determinism gate replays
the same sequence. It is a strength defect.

**Option B was tried.** With `may_replace` made epoch-aware — a stale occupant
is always replaceable, two lines, the proven-retention law untouched WITHIN an
epoch — the 85-position ANCHOR set at cap 4096 goes from **3 win / 23 nowin / 59
unknown** to **5 win / 24 nowin / 56 unknown**, `g002-t39-p1` returns to `win` at
**714 nodes**, and the total node count falls 263,640 → 256,292. **And the wall
goes UP**: 21.54 s → 22.86 s, µs/visit 81.70 → 89.18 (+9.2 %), `select_us`
+13.8 %. Revision 1 quoted only the node reduction; the red team was right that
this is the WP's own subject and had to be reported. The direction is not
mysterious — a table that evicts stale entries holds more LIVE entries, so
lookups hit more often and the search does more work per surviving visit — but
it means the change is a strength improvement bought with wall, not a free one.

**AND THE FIX DOES NOT BUY CROSS-SOLVE INDEPENDENCE, which is a limit worth
stating because the obvious test for it FAILS.** A shared table of finite size
is a shared resource whether or not stale entries can be evicted: entries from
earlier solves still OCCUPY slots, so the eviction pattern a later solve meets
is not the one a fresh table would give it. **MEASURED**: with the epoch clause
in place and a deliberately small 256-entry table, `decoy-2` of the deep fixture
still answers `Win` alone and `Unknown` as the third case of a shared `Solver`.
The claim the fix supports is the narrower one — a stale entry no longer holds a
slot against a live one — and it is the mechanism, not the symptom, that the
registered gate asserts (§5.7's M-9). A test that asserted independence would be
asserting something a finite shared table cannot provide, and it is recorded here
rather than quietly dropped.

**Rule C's input, RE-MEASURED rather than asserted** (M-7): CORPUS `c` under
option B is **91.44 µs** against 88.53 without it. §4b's cap is unchanged at that
value, and this paragraph is the recomputation revision 1 replaced with a
sentence.

**Matrix M8.**

| # | option | cost | failure mode | verdict |
| --- | --- | --- | --- | --- |
| A | Leave it | zero | the SPRT would judge a solver whose answers decay with call count. **Revision 1 said the same of the cost BENCH and that was wrong**: silting makes solves longer, so it biases the ratio CONSERVATIVELY, and the bench is readable through it. The unreadable measurement is the SPRT, which §6b now forecloses anyway | REJECTED, on the SPRT and not on the bench |
| B | Epoch-aware replacement law (two lines) | small in code, LARGE in gates — it CHANGES VALUES, so §3's identity property does not cover it | **RECOMMENDED**, in its own commit, landing BEFORE the bench so §1's bit-identity premise stays true of the four legs |
| C | Clear the table per solve | O(entries) per solve — the cost D-437 rejected it for; gate (c)'s 145k solves become infeasible again | REJECTED, and it is why the epoch scheme exists |
| D | Hand it up and stop | none | the WP would then bench a solver it has just measured as degrading | REJECTED — but the red team is right that revision 1 strawed this row as "stop": the real D is "fix it in its own slice, before the bench", which is what row B now says |

**The four oracle gates were RUN with option B in the tree, and none of their
numbers moved.** `tools/solver_oracle_check.sh` (CI gate 12) at the prototype
revision, release, all four gates with `--ignored`, 274 s:
`gate (a) PASS: 61 cases agree with R3'`, `gate (b) PASS: 38 proof trees
re-verified full-width`, `gate (c) PASS: 29 wins, 118135 sigma placements
replayed and revalued (26865 refused on collision)`, `gate (d) PASS: values
agree at both table sizes` — every count identical to the ones `docs/ROADMAP.md`
records for D-437's WP-1.8a closure (61 / 38 / 29 / 118,135 / 26,865). Receipt
`artifacts/wp18c_oracle_gates_v1.txt`. This is evidence and not yet the gate:
the run was taken on the prototype, and §5.4 requires it again at the IMPL SHA.

**What option B does to the pinned fixtures, registered rather than left open**
(M-8): `solver-selftest` constructs ONE `Solver` for all cases, so it is exposed
to exactly this staleness. The fixtures pin VALUES (`expect win|nowin`) and
nothing else, so the gate that adjudicates option B is the VALUE set: every
fixture case must keep its registered `expect`, and gate (b) must re-verify every
`Win`'s tree full-width. Node counts and digests MAY move and are re-recorded,
not defended; `tools/solver_determinism.sh` is a run-twice diff and is unaffected
by a change that is deterministic. If any fixture case's VALUE moves, that is a
finding and this WP stops on it. The determinism seat's own cap (512) does not
move; D-442's closure pin is superseded by this WP's own pin at closure.

## 4d. INHERITED DEFECT 2 — the node budget is not enforced on the ON seat

`should_stop` tests `self.total_nodes().is_multiple_of(NODE_CHECK_INTERVAL)`
with `NODE_CHECK_INTERVAL = 1024`. `total_nodes()` is `search_nodes +
solver_nodes`; `search_nodes` moves by ONE per visit, but a solver call absorbs
its whole node count at once. **An exact-multiple test on a counter that jumps
by thousands steps OVER the multiples and does not fire.** `total mod 1024`
performs a walk that lands on 0 with probability ~1/1024 per sampled node, so
the expected number of nodes past the budget is not one capped call — it is
hundreds of them.

**MEASURED** (`artifacts/wp18c_onoff_verify_v1.txt`, this session's own driver,
corpus fixture, 50,000-node budget, cap 2048): the ON seat spent **3,751,507
nodes over 24 positions — a mean of 156,313 per position against a budget of
50,000, and a maximum of 648,192**. The OFF seat spent 50,176 per position, its
registered 176-node overshoot.

D-441 registered the overshoot as bounded: "between two checks the counter can
absorb up to one capped call per intervening visit, so the budget can overshoot
by that amount before the next check — deterministic (identical on every run;
D-74's exactness wording applied to the mask mechanism, with the overshoot now
bounded by the cap, not by the interval alone)." **That claim is false**, and it
is false in the direction that matters: rule 6's premise for every SPRT is equal
per-side compute, and a seat spending 3× its budget is not an arm.

**Gate-off is unaffected**, which is why no committed config ever showed it: with
`solver_nodes == 0`, `total_nodes() == search_nodes` and the mask lands on every
multiple exactly as before. Every committed config is gate-off.

**The fix, and why this spelling.** Keep the mask — its exactness is a registered
contract (`stop.rs`: "a search given `n` nodes stops on node
`n.next_multiple_of(NODE_CHECK_INTERVAL)`"), and changing it would move every
recorded node count in the project. Add ONE disjunct:

```
Stop::DepthTurns(_) | Stop::Nodes(_) => {
    self.search_nodes.is_multiple_of(NODE_CHECK_INTERVAL) || self.solver_nodes > 0
}
```

`search_nodes` moves by one, so it lands on every multiple; the SPENT test still
reads the derived total, so the budget stopped at is still the shared one. The
second disjunct is `false` for the whole life of every gate-off search, so
gate-off behaviour is **byte-identical** — the pinned golden transcripts, D-88's
field order and every recorded node count stand. On an ON seat every visit after
the first solver call checks, so the overshoot is bounded by one visit's own
calls: `2 × per_call_node_cap`, which is what D-441 claimed and did not have.

**Its own test**: an ON-seat search reports
`nodes ≤ budget + NODE_CHECK_INTERVAL + 2·cap`, driven at BOTH cap 1024 and the
registered cap 2048 and asserting `nodes ≥ budget` FIRST so a search that
answers early fails rather than satisfying the bound vacuously; gate-off
byte-identity is carried by `search_budget_tests.rs`, which pins the stopping
node at several ragged budgets.

**THE BOUND HAS AN EXCEPTION, AND IT IS OLDER THAN THIS WP** (REVIEW-impl's
I-4). The FIRST iteration of iterative deepening is non-abortable —
`search.rs` sets `abortable = depth_turns > 1 || fallback.is_some()`, and a
reproducible stop has no fallback — so `should_stop` returns `false` for the
whole of depth 1 and no budget check of any kind is consulted there. The bound
above therefore holds for depth ≥ 2 and not unconditionally. **MEASURED** on the
twenty repaired trigger-rich positions at cap 2048: at a 1,000-node budget FIVE
of twenty exceed it, worst by 5,772 nodes — and two of those five have
`solver_nodes == 0`, which is what shows the cause is D-74's non-abortable
iteration and not §4d; at the registered **50,000-node budget, none of the
twenty exceeds it** (max 53,971 against a bound of 55,120). So rule 6's
equal-compute premise for §6's bench is not in question, and this paragraph
corrects §4d's claim to be about what it fixed rather than about a bound the
ungated seat never had either.

## 4e. INHERITED DEFECT 3 — the trigger-rich bench fixture is neither pinned nor loadable

`wp18b_design.md` §7 registers "a NEW trigger-rich fixture
`bench_solver_positions_v1.txt` (late-game threat shapes, the class the corpus
under-represents; committed and sha-pinned per rule 7 — positions, not run
artifacts)", and §6 of revision 1 inherited that sentence. **Both halves are
false.**

- **Not pinned.** `crates/pistol-cli/tests/corpus_document_tests.rs` pins
  `openings_v1.txt` and `bench_positions_v1.txt` and nothing else. `git grep
  bench_solver_positions` finds the file, its own header, and no consumer: no
  test, no `tools/` script, no binary reads it.
- **Not loadable.** Its header claims "Same line form as
  `bench_positions_v1.txt`". It is not: `bench_positions_v1.txt`'s tails are
  TURNS (`0,0 -1,1/1,0 0,1/0,2 …`), and this file's are CELLS
  (`0,0 2,-2 1,0 0,1 …`). Fed to the shipped engine, the first position is
  refused: `error IllegalMove: turn 2: 2,-2: turn 2,-2 is not legal: a turn of
  one stone completes only when that stone completes a line (rule 4); this turn
  owes two`.

So the trigger-rich half of the ≥ 0.25 / abort-below-0.1 bracket has **never
been runnable**, in WP-1.8b or in revision 1 of this design.

**The repair, and it is a repair rather than a replacement**: the cells are the
plies of real anchor games in play order, so pairing them (`cells[0]` alone,
then `cells[2i-1]/cells[2i]`) recovers the turns, and each pair is written
canonically (D-56 refuses an uncanonical pair rather than reordering it).
**MEASURED**: all 20 positions then load through the engine's own
`PositionSpec` parser, 0 refusals. The repaired file is sha-pinned in
`corpus_document_tests.rs` beside the other two, and a new test drives EVERY
line through `PositionSpec::from_str` — which is the check whose absence let the
original ship. **The pinning test alone would not have caught this** (a pin
attests bytes, not meaning), and that is the point of the second test.

**AND A THIRD CLAIM ABOUT THE SAME FILE IS FALSE, found by asking the repaired
bytes rather than the header.** `wp18b_design.md` §7 registered as MEASURED
that "every position below holds a hot window (an open four or better) for at
least one side at the mover's turn boundary" — the property §6's trigger-rich
band is a gate ON. **MEASURED here, through the same `ThreatState` the trigger
predicate reads: TWELVE of twenty.** The repair cannot have caused it — it
reordered cells only WITHIN each pair, and both stones of a turn belong to the
same mover, so every board is the one WP-1.8b committed; the original
measurement was taken over the EXTRACTOR's positions, which is what a file
nobody could load leaves you doing. The fixture still earns its name (12/20
against the corpus fixture's own registered 8/24) and §6's band still means
something, but the proportion is now PINNED by
`the_trigger_rich_fixture_is_trigger_rich_in_the_measured_proportion` so a
regeneration is a deliberate act. **Three registered claims about one committed
fixture, all three false, none of them checkable until something read the
file** — which is the argument for the coverage rule and not merely for a pin.


## 5. Correctness — how the identity property is checked

The property is §3's opening paragraph; this section is only the checking.
§4c/§4d/§4e do not carry it and name their own gates in their own sections.

1. **Leg 3 is checked against an INDEPENDENT implementation, not against
   itself.** `crates/pistol-solver/tests/common/r3.rs`'s `threat_moves` computes
   arm A by its own board scan — clone the state, play the pair, look for a live
   window at `own >= 4` — touching neither `ThreatState` nor the predicate under
   suspicion; `r3_zone.rs` is a third site.
   `the_three_policy_sites_agree_elementwise_under_both_policies` drives all
   three and today drives them on ONE position (`g001-t42`). **It is extended to
   every case of the two pinned fixtures**, under both `attacker_policy` values,
   elementwise and in order. **What that coverage actually is, stated rather than
   implied** (m-6): REVIEW-design measured it — of 61 bounded drives, 58
   short-circuit on `hot_already` (so arm A emits every pair and the predicate's
   other clauses are untested there) and 0 exercise the joint or raiser clauses;
   ALL of the discriminating coverage comes from the 9 deep cases, whose 18
   drives exercise the joint clause on every one. Enough to kill the mutants
   below; not the breadth the wording alone suggests.
2. **Agreement in-tree for the legs R3' does not see.** `blocking_pairs`'
   existing `debug_assert` against the `generate_turns`-and-filter spec form
   stays untouched — it is leg 2's whole check, since leg 2 changes only WHERE
   the region is computed. `Search::child_keys` gains the same shape: under
   `debug_assertions`, every constructed key is compared against the apply/undo
   key. **Named risk with its own criterion**: that assert is O(children)
   apply/undo per node in DEBUG, which is what `cargo test` runs. The registered
   criterion was that if `cargo test --workspace --locked` grew past twice its
   pre-change wall the blanket assert would move into a dedicated test and item
   3 would carry the rest. **MEASURED, and the assert stays**: the candidate
   tree runs the workspace suite in **12m52.9s** (146 results, 0 failed) against
   the base commit's **13m18.7s** (145 results, 0 failed) — a ratio of **0.97**,
   taken on the same machine with the two runs contending against each other so
   neither had it to itself. It is FASTER despite the added assert because leg 4
   speeds up `region_cells`, which the pre-existing `blocking_pairs` spec-scan
   assert calls through `generate_turns` on every AND node of every debug run.
3. **An always-on check on the load-bearing path.** In both descent loops, after
   `apply_turn`, `state.key()` is asserted equal to the `child_key` the parent
   constructed. `GameState::key` is O(1) — the key is carried incrementally — so
   this is one 128-bit comparison per descent, on the child whose key is about to
   be STORED under.
4. **The four oracle gates re-run green at the impl SHA**, through
   `tools/solver_oracle_check.sh` (CI gate 12), plus the determinism gate on all
   four seats (CI gate 13) byte-identical, plus `tools/ci.sh` entire. **Two
   gates move BY DESIGN and are named here so a reviewer does not read the
   movement as drift**: `solver_link_check_tests`' shipped-binary count goes
   from six to seven, because §2.1's `solver-cost` is a seventh shipped binary
   and that gate exists to make a new one a deliberate act; and
   `file_justification_check` gains
   `crates/pistol-search/tests/wp18b_solver_path_tests.rs`, which §4d's two
   tests push over the soft cap and which now carries its why.
5. **The fixtures keep their registered values.** `solver-selftest` on
   `solver_v0.txt` and `solver_deep_v0.txt` reproduces every registered
   `expect`. **What is NOT registered, stated so the claim is not inflated**
   (m-5): the fixtures carry `expect win|nowin` and nothing else — node counts
   and proof digests are a before/after this WP holds, not a committed golden,
   and `tools/solver_determinism.sh` is a run-twice diff. For the four legs they
   must not move (identity); under §4c's change they may, and §4c registers what
   happens then.
6. **Node-identity over both fixture sets.** The T0-vs-T3 per-case
   `value`/`nodes`/`seesaw` triples over 109 positions, compared by the join in
   `wp18c_instruments.md` §3, must be identical.
7. **Mutants, in a separate git worktree (never the live tree).** Each must be
   killed by a NAMED gate, and the naming is the receipt:
   - **M-1** drop `side_key` from leg 1's delta → every child key wrong →
     the descent assert (§5.3) and gate (b).
   - **M-2** drop the joint own-2 case from leg 3's predicate → arm A emits a
     subset → the three-site agreement test, on the deep fixture's 18 drives.
   - **M-3** drop the raiser clause from leg 3's predicate → decisive for
     596,768 of 6,975,203 CORPUS pairs (**MEASURED** by REVIEW-design) → the
     three-site agreement test on the deep fixture. *Registered here because
     revision 1 omitted the predicate's largest clause* (m-3).
   - **M-4** drop `hot_already` → `policy::tests::a_hot_position_emits_every_candidate_pair`,
     a unit test built for it. **NOT the three-site agreement test, and the
     reason is a finding this WP owes**: a hot attacker window IS a win this
     turn (hot means live at `own ≥ 4`, and `can_win_this_turn` answers every
     one of those), so every position that would exercise `hot_already` is a
     position `threat_pairs`' own precondition excludes — and on those the two
     implementations legitimately diverge, because `policy.rs` never asks about
     legality while R3' plays each pair and drops what rule 4 refuses. MEASURED
     at `shallow-win-0`: `policy.rs` emits `(-1,0)/(5,0)`, which `make_turn`
     refuses because either cell completes a line on its own. **That divergence
     is in the COMMITTED code too and is not leg 3's**; the agreement gate is
     scoped to the contract and says so, and the arm gets its own gate instead.
   - **M-5** widen leg 4's r-interval to `[r − 8, r + 8]` (the square, not the
     hex ball) → cells outside the region enter it →
     `legal_placements_are_the_cells_the_cell_probe_calls_legal` and the perft
     oracle.
   - **M-6** narrow leg 4's row range to `min_q ..= max_q` → cells inside the
     region go missing → the same agreement test and the perft oracle.
   - **M-7** drop leg 4's `i16` clamp → `r as i16` wraps and lands cells at
     `r ≈ −32,763` in the region →
     `legal_placements_stop_at_the_edge_of_the_addressable_lattice`.
   - **M-8** drop §4d's `|| self.solver_nodes > 0` disjunct → the ON seat
     overshoots again → §4d's own budget test.
   - **M-9** drop §4c's epoch guard →
     `tt::tests::a_stale_occupant_never_blocks_a_live_entry`, which asserts the
     MECHANISM (a stale proven entry at the freshest generation does not hold a
     slot against a live unproven one) rather than the symptom, for the reason
     §4c gives: the symptom test would be asserting an independence a finite
     shared table cannot provide.
   - **Leg 2 has no mutant of its own and does not need one**: it moves a call
     out of a loop, and the `debug_assert` in item 2 compares the whole emitted
     set against the spec form on every AND node of every debug run.
   **THE ROUND WAS RUN AND TWO GATES FAILED IT.** All nine mutants are dead at
   their named gates in the receipt (`artifacts/wp18c_mutations_v1.txt`), but
   two of them survived the first round and the reason each survived is a
   finding this section keeps rather than a step it quietly re-took:

   - **M-9's gate was VACUOUS.** `SolverTT::new(2)` is two buckets of two
     slots, and the test's occupants left one of the newcomer's slots EMPTY —
     `store` fills an empty slot before it consults `may_replace` at all, so the
     replacement law was never reached and the test passed with the epoch clause
     deleted. It now fills every slot and ASSERTS the table is full before the
     displacement it is about. (REVIEW-design found the same thing independently
     and went one further: with `may_replace` hardwired to `false`, all seven
     `tt` tests stayed green.)
   - **M-8's gate was vacuous twice over, and the second reason is a fact about
     the DEFECT.** First the position: `G001_T45` answers through a root proof
     in a few hundred nodes against a 20,000-node budget, so no budget check of
     any kind was reached — the gate now runs a midgame corpus position and
     asserts `nodes >= BUDGET` FIRST, so a search that finishes early fails
     loudly instead of satisfying every upper bound. Then the cap: **the
     registered cap 2048 is the one value at which the broken check hides.** Two
     capped calls at 2048 move the derived total by close to four
     `NODE_CHECK_INTERVAL`s, which PRESERVES the residue, so the exact-multiple
     test keeps landing; at 1024 the same broken check spends **11,264 nodes
     against a 4,000-node budget** (MEASURED). The gate drives BOTH caps, and
     the pair is the gate — a gate run only at the cap that ships would have
     passed through the defect it was written for.

   **A mutation that does NOT kill is a finding, not a pass**: leg 4's interval
   merge is the case in point — `low <= run_high` instead of `low <= run_high + 1`
   stops ADJACENT runs merging and still emits the same cells, in the same order,
   without duplicates. It is not a defect and not a mutant; it is recorded so a
   reviewer does not look for the gate that kills it.

## 6. The rule-5 bench, registered before measuring

The hotspot is §2's: the wall cost of one df-pn visit, paid at trigger nodes.

**Inherited verbatim from `wp18b_design.md` §7, and this time quoted rather than
paraphrased** (M-4: revision 1 dropped "in both bands" while claiming to inherit
the section unchanged):

> **Bracket**: band-aggregate nps ratio ON/OFF ≥ 0.5 in both bands on the
> CORPUS fixture (the regression axis: gate-on must not halve ordinary
> throughput), and ≥ 0.25 on the TRIGGER-RICH fixture (the stress axis…);
> **abort** if the corpus bracket fires (< 0.5) or the trigger-rich ratio is
> below 0.1 — a config-default-on engine that spends nine tenths of equal budget
> inside solver calls is not a candidate for h1 regardless of what the SPRT says
> about the gated seat.

`bench_positions_v1.txt` carries two bands (15-stone and 35-stone); **both must
clear 0.5**, and a single aggregate that clears while the late band does not is
an ABORT. The trigger-rich fixture becomes runnable for the first time under
§4e's repair.

- **The GATE bracket** is the quoted one, unchanged. Its consequence is
  WP-1.8b's, verbatim: *the ON seat is not a candidate for h1 REGARDLESS of what
  the SPRT says about the gated seat.*
- **Both units are reported** (rule 5's "report nps AND time-to-depth", which
  revision 1 registered as nps only): time-to-depth is reported per position
  beside nps. It is NOT independent evidence here — the ON seat's `nodes` is
  `search_nodes + solver_nodes`, a mixture of two units, so ttd and nps are two
  readings of one wall, and both are printed with that said.
- **The rep counts, and why they differ between the two fixtures.** The
  D-215/D-362 convention is **five repetitions with a 10 %-of-median IQR gate**
  (`tools/bench_delta.sh`'s own header and `stats()`), and the CORPUS fixture
  gets it: that fixture carries the abort's own axis, and both its bands are
  reported with medians and IQRs. **The TRIGGER-RICH fixture is run ONCE and
  recorded as a DIAGNOSTIC, not as a verdict input**, and the reason is stated
  before the run rather than after the numbers: the abort clause is
  DISJUNCTIVE — "abort if the corpus bracket fires (< 0.5) **or** the
  trigger-rich ratio is below 0.1" — so a corpus ratio under 0.5 fires the
  abort whatever the trigger-rich band does, and no rep count on that fixture
  could change the verdict. If the CORPUS band clears 0.5, the trigger-rich
  fixture is re-run at the full five reps before anything is concluded from it.
  Its single-rep number is reported as what it is: the first measurement that
  fixture has ever admitted (§4e).
- **The REPRODUCTION bracket**, per leg, from §2.2's ladder — which §2.1 names
  as this document's one referent, and which this bullet POINTS AT rather than
  restating (M-6, and D-423's rule about a claim made twice). Each leg's commit
  carries its own row re-run at that commit. The partition is total AND
  disjoint, read top to bottom, first match wins:

  | measured ratio at the commit | verdict |
  | --- | --- |
  | composite T0→T3 outside [20×, 35×] | the IMPL is not the prototype: **STOP** and find out why. Not a WP abort |
  | else, every leg within its ladder row ±15 % | PASS |
  | else | PASS with the deviation RECORDED in the commit message |

  **THE IMPL IS NOT ANY LADDER ROW, and the three differences are named rather
  than absorbed into the ±15 %**: the timers come out (their cost is MEASURED
  only on the committed side, ~10 %, and not on any other row — §2.1); §5.3's
  always-on `assert_eq!(state.key(), child_key)` is in the IMPL and in no ladder
  tree; and the IMPL reuses arm A's sorted, deduplicated raiser set for arm B
  where T2 and T3 recomputed it — behaviour-identical (same query, same sort,
  same dedup, verified by REVIEW-design) but not wall-identical. The bracket is
  wide because of these three, not in spite of them.

## 6b. WHAT IS ALREADY KNOWN ABOUT THE ANSWER, AND WHY THE BENCH IS STILL RUN

Pre-registration exists to stop a READING being chosen after the numbers are
seen. It does not license pretending the numbers are unseen. Two independent
measurements have already been taken at the review revision — the DECISION-RED-
TEAM's, and then this session's own on a separately written driver — and they
agree:

CORPUS fixture, 24 positions, 50,000-node budget, cap 2048, the SAME binary
gate-off vs gate-on (`sessions/WP-1.8c/redteam_seat_{off,on_cap2048}.toml`,
identical but for `on_search_path`), all four legs and §4c option B in the
build. Receipt `artifacts/wp18c_onoff_verify_v1.txt`:

| seat | nodes | of which solver | engine time | nps |
| --- | --- | --- | --- | --- |
| OFF | 1,104,026 | — | 4,857 ms | **227,306** |
| ON | 3,751,507 | 3,662,801 (**f = 0.976**) | 374,182 ms | **10,026** |

The OFF seat's 1,104,026 is the same integer `artifacts/wp18b_bench_v1.txt`
records, so the seat is the one WP-1.8b measured.

**0.044 against a ≥ 0.50 bound.** `f = 0.976`: the ON seat spends 97.6 % of its
(overshot) budget inside solver calls. The registered bracket is unchanged and
no threshold moves.

**The bench is still run, and here is what makes it a different measurement**
rather than a re-run of the same one: it is taken at the IMPL revision, after
§4d's budget fix (so the seat spends its budget rather than 3× it), after §4c's
TT change (so the solver's answers do not decay through the run), on the
repaired trigger-rich fixture (§4e, whose band has never been measured at all),
IQR-gated at the D-215/D-362 convention, with both bands reported separately. If
it lands near 0.044 the abort is the verdict; if the three fixes move it, that
is the measurement this WP owes.

**What the sweep says about WHERE the cost is**, because the ABORT branch's
value is entirely in that answer:

The ON seat's own cap, swept over the CORPUS fixture at the same 50,000-node
budget, same binary, same driver (`artifacts/wp18c_cap_sweep_onseat_v1.txt`):

| cap | solver share `f` | nps | ratio vs OFF's 227,306 |
| --- | --- | --- | --- |
| 32 | 0.482 | 23,341 | **0.103** |
| 128 | 0.743 | 13,772 | 0.061 |
| 512 | 0.931 | 10,429 | 0.046 |
| 2048 | 0.976 | 10,267 | 0.045 |

**Sixty-four times less cap buys 2.3× of ratio, and the bracket needs eleven.**
The curve flattens rather than climbing, and the mechanism is visible in the
`search` column of the receipt: a smaller cap leaves more of the budget for
SEARCH nodes, which produce MORE trigger nodes, which make more calls — so the
solver's share falls far more slowly than the cap does. At cap 32, a cap too
small to prove anything deeper than an immediate win, the solver still takes
48 % of the budget. **The binding term is the number of solver CALLS times the
per-call FIXED cost, and neither is the cap** — **INFERRED**, and marked as
such: it follows from the sweep's own `search`/`solver` columns together with
the per-call fixed costs §4b measures, but there is no `solver_calls` counter
anywhere in the workspace, so the call count itself is derived and not read.
Adding that counter is the first thing the next package should do, and this
sentence is what it would be measuring.

**Caveat, stated because it cuts the other way and is still not enough**: the
sweep was taken before §4d's budget fix, so every ON row overshoots its budget.
`nps` is a rate and the comparison survives that, but the registered bench is
run after the fix, on seats that spend what they are given.

## 7. What this design does NOT do

- It does not touch the solver's policy or its zones. M6 rows C, D and E name
  three further levers with their measured or bounded worth; all three are
  deferred, and §4b/§6b say why none of them reaches the bracket.
- It does not re-open `attacker_policy`. §4b records that the committed M4
  policy costs ~9× the nodes on the two loser-win proofs and that no viable cap
  reaches them; whether the SEARCH PATH should run `both_stones_relevant` while
  the ORACLE keeps M4 is a decision with its own matrix, its own red team and
  its own gates.
- It does not schedule Deep df-pn. The post-fast-path seesaw share is recorded
  as one MEASURED number at closure and the licence stays unscheduled.
- It does not build a `tools/` bench script. The bench is a command block the
  design prints and `wp18c_instruments.md` carries, named with its revision —
  the anchor probe's §4 precedent. **The alternative is named rather than
  ignored**: a `tools/` script would carry a driving test under
  `SHELL_CHECKLIST.md`'s coverage rule and would be the better artefact if this
  bench were going to be re-run across revisions. §6b is why it is not.

## 8. Cost of the governed work this design licenses

**MEASURED, not estimated** (revision 1's "~10 minutes on the OFF seat" was
wrong by two orders — `artifacts/wp18b_bench_v1.txt` already records 4,935 ms):
the OFF seat is **4.9 s** over 24 positions; the ON seat at cap 2048 is **374 s**
(this session's own run); the cap sweep is a few minutes; the leg ladder is
about 20 minutes of machine time and was taken. Everything this design licenses
is cheap enough that doubt about any of it is answered by re-running it, not by
a margin derived to defend one sample — which is why §6b could take the
feasibility measurement twice, on two independently written drivers, before
registering anything about it.

## 9. Instrument register (governing revisions)

Every command block is written out verbatim in
`docs/experiments/wp18c_instruments.md`, which is part of the record rather than
a session note. **Five trees produced the numbers in this document and each is
named** — this is BLOCKING B-1's fix:

| artefact | revision | what it produced |
| --- | --- | --- |
| `solver-cost` + timers at T0 | `18b3d8a2` | §2.2's T0 row and T0 attribution |
| `solver-cost` + timers at T1 | `277ae0cd` | §2.2's T1 row and T1 attribution |
| `solver-cost` + timers at T2 | `e0c99ce4` | §2.2's T2 row and T2 attribution |
| `solver-cost` + timers at T3 | `654abd43` | §2.2's T3 row and T3 attribution |
| `solver-cost` + timers at T4 | `9eb1245b` | §2.2's T4 row; §4c's option-B receipt |
| the `BASE` / `LEGS` scratch build directories | **NOT SNAPSHOTTED, and nothing load-bearing rests on them** (§2.3): the withdrawn 29.71× / 24.18× pair only | the four `wp18c_cost_*_v1.txt` receipts |
| the `LEGS` tree, as the cap ladder ran it | code-identical to T3 (`654abd43`) plus the timer edits already in it; the ladder was driven over one-case fixture DIRECTORIES, which the shipped `solver-cost` does not take | §4b's cap ladder (`wp18c_cap_ladder_committedTT_v1.txt`) and §4c's reproducer |
| the `LEGS` tree + `configs/bench_wp18c_solver_{on,off}.toml`, with `per_call_node_cap` rewritten per run | the §6b driver and sweep (`wp18c_instruments.md` §7, §8) | §6b's ON/OFF table (`wp18c_onoff_verify_v1.txt`) and its cap sweep (`wp18c_cap_sweep_onseat_v1.txt`) |
| `tools/wp18b_probe_extract.py` | unchanged since D-441, transcripts digest-checked against D-438's `MANIFEST.sha256` | the ANCHOR fixture set |
| the CORPUS / TRIGGER-RICH conversion block (`wp18c_instruments.md` §2) | with the ladder | those two fixture sets |
| the ON/OFF verification driver (`wp18c_instruments.md` §7) | with the ladder | §6b's table |
| the bench command block | authored and named at the bench | §6's ratios |

**Where the trees live**: the five ladder revisions are `git stash create`
objects in the detached worktree `/home/tom/wp18c-ladder` on the WP's base
commit `4569c1a`; the WP's own work is uncommitted on branch `wp18c` in
`/home/tom/Projects/HeXO-AlphaBeta-wp18c`. `git cat-file -p <revision>` reaches
any of them from the shared object store.

Receipts land under `artifacts/` (rule 8, never committed) and are named in the
closure ADR by digest.
