# Stage-3 detector — OPTION MATRIX

**REVISION 3.** Revision 2 (`f62c676`) named three objections to its own §5 and
selected on none of them, because the operator's dispatch put a second
DECISION-RED-TEAM before selection. This revision discharges all three by
measurement before that red team sees it, and **the measurements change the
answer**: the row revision 2 headlines does not survive either of them.

| revision 2 said | revision 3 measured |
|---|---|
| `opp_hot >= 3` keeps **1.000 / 0.500 / 0.560** of the proofs | under the ONE recall definition D-512 pins — WINS, the attacker direction — it keeps **— / 0.000 / 0.000**. Band 15 has **no win-direction proof at all**, so its 1.000 was 1.000 *losses* |
| `opp_hot >= 3` cuts band 15 by **41.8x** against a required 22.5x | **out of sample it cuts by 6.45x against a required 24.33x.** The threshold was fitted to the draw it was measured on |
| `opp_hot >= 3 + cache + cap 512` reaches the bracket on all three bands | out of sample it reaches band 15 (39.17x) and **misses band 35** (40.47x against 47.25x), while keeping **0 of 3** band-15 wins |
| the recall fixture is *"seven named anchor positions"* | **four of the seven are not trigger points**: the incumbent trigger does not fire there, so there is no trigger point to rank and D-512's ranking gate has nothing to evaluate |

**Nothing is selected.** Selection follows a fresh-context DECISION-RED-TEAM on
THIS revision, and the selection record quotes its rows (CLAUDE.md, Process).

**What governs the new numbers.**
`docs/experiments/stage3_oos_registration.md`, written before either run, fixes
the recall definition, the out-of-sample draw rule, the budget derivation, the
value-fixture criteria and the selection rule. Its Criterion V (the draw
instrument must reproduce `bench_positions_v1.txt` exactly at round 0) and
Criterion R (the re-taken census must reproduce the standing one on every shared
column) both PASS — receipts in §5.0.

**The field is the operator's** and is a FLOOR, not a closed set: the governing
dispatch reads *"options ranked by precision economics, **at least**: (a)…(f)"*
(`docs/experiments/stage3_overnight_dispatch.md` §3, D-511). A red team may add
a row before selection.

---

## 1. THE AXIS, and it is a premise (D-477)

D-477 makes a matrix's own axis a premise, quoted at the line where its unit is
**consumed**. This field is arranged along one axis and it has three units that
are routinely confused, so all three are quoted:

| unit | consumed at | what it is there |
|---|---|---|
| **firing** | `crates/pistol-search/src/pvs.rs` — the trigger test in `solver_verdict`, and the root's in `crates/pistol-search/src/search.rs`'s `root_triggers` gate | ONE decision. This is what a detector gates, and it is the unit every row below is ranked in |
| **invocation** | `pvs.rs` — `solver.solve(&state_view, cap)` and `solver.solve_defender(&state_view, cap)` | one DIRECTION of one firing. A firing makes the attacker call and then, unless it proved, the defender call, so a firing is **up to two** invocations |
| **visit** | `pvs.rs` — `self.solver_nodes = …saturating_add(result.nodes)` at both call sites, stopped on where `total_nodes = search_nodes + solver_nodes` is read | what the BUDGET is denominated in, and the only unit the bracket fixes directly |

**A row that changes the number of INVOCATIONS per firing is a different kind
of row from one that changes the number of FIRINGS**, and the matrix keeps them
apart: rows (a), (b), (e) reduce firings; row (c) reduces the visits a firing
costs; row (d) does both. Ranking them on one number without saying which is
the substitution D-477 exists to forbid.

## 2. THE BUDGET the rows are ranked against

Stated once, in `docs/experiments/stage3_rulings.md` §1, and not restated here
(D-423). What this document consumes from it:

- the bracket fixes a **share** of a search's nodes: **4.352 % / 2.242 % /
  5.696 %** (corpus band 15 / band 35 / trigger-rich);
- in the unit a design sets, that is a per-search **firing budget** — and the
  figure and its direction are exactly what the rulings' round-3 revision
  settles from the measured `K`, so this document reads it from there rather
  than carrying its own copy;
- the incumbent spends **18.33 / 11.75 / 9.05** firings per search and the
  budget is **0.815–0.845 / 0.276–0.286 / 0.579–0.599**, so the required
  reduction is **21.7x / 41.1x / 15.1x** (MEASURED, D-517). **Those are FIRING
  figures**; §5's test is in VISITS and its cuts are 22.49x / 42.57x / 16.18x
  in sample and 24.33x / 47.25x out of it. The two are the same constraint in
  two units and neither replaces the other;
- **the ROOT's own firing is inside the budget** and can cost two whole caps.

## 3. THE ROWS

Each row states: the MECHANISM (what code changes and where), what provable
wins its ranking can push OUT of budget, its COST SHAPE (what it adds per node
and per firing), and its KILL CONDITION (the measurement that ends it).

### (a) Tightened calculus-class trigger

**Mechanism.** A new variant of `SolverTrigger`
(`crates/pistol-search/src/params.rs`) and the matching config token
(`crates/pistol-engine/src/config.rs`), evaluated at the same two sites the
present `AnyOpenFour` is — the in-tree predicate and `root_triggers`. The
present trigger fires when EITHER side holds any live window with four or more
own stones. The tightenings available at O(1) from the same slices are:
fire only on a **win-in-one-ply** window (exactly five); fire only when **both**
sides are hot; fire only above a **count** of hot windows (a double threat).

**What it can rank out.** Any proof whose root position holds a single
four-stone window per side. The M4 flip at `g001-t42-p2` is a candidate: it was
proved from a position the record does not describe as a double threat.

**Cost shape.** Unchanged: slice lengths are already maintained on both seats,
so the added per-node cost is a comparison. **This is the only row whose `t` is
free by inspection** (ruling 6, D-514).

**Kill condition.** The tightest variant that reaches the firing budget also
excludes a VALUE row of the recall fixture (D-512).

### (b) Pattern-level must-block / open-four detection

**Mechanism.** Replace the "any hot window" test with a test for an
**unanswerable** threat: the opponent is hot AND no single legal pair covers all
of the opponent's hot windows. `ThreatState::blocking_covers` already answers
the covering question.

**What it can rank out.** Proofs that need a threat the cover test calls
answerable — a mover-hot-only position with one four, where the win comes from
a follow-up the cover test does not look for.

**Cost shape.** **NOT O(1).** The minimal-cover loop is the site D-461's leg (2)
optimised (`legal_placements` hoisted out of `blocking_pairs`' loop), so this
row pays a real per-node cost at EVERY node, not only at firings. Ruling 6's
budget-erosion ceiling is what it must clear, and it is the row most likely to
fail it.

**Kill condition.** Its measured per-node cost exceeds the registered
budget-erosion ceiling — which is a bench measurement it must take before any
SPRT, because at that point the row destroys the budget it was meant to protect.

**Revision 3 adds the column that scores it and the row does not reach that kill
condition**: it fails the ranking first (§5.4). Its cost shape is therefore
never measured, and this document makes no claim about it.

### (c) Bounded VCDT-only probe as a pre-filter

**Mechanism.** At a firing, run a tiny-cap solver call first (a probe) and spend
a full-cap call only where the probe is encouraging. The probe is the same
`Solver::solve` at a much smaller `per_call_node_cap`.

**What it can rank out.** Any proof deeper than the probe's cap that the probe
cannot distinguish from a NoWin.

**Cost shape.** **The probe's visits count against the same budget.** With ~12
firings per search and a visit budget in the low thousands, a probe costing `p`
visits at every firing spends `12p` before any real call is made — so the probe
must be very cheap or the firings must already be few. This row is therefore
**not independent of (a) or (e)**: it is a modifier on whatever reduces
firings first, and the matrix says so rather than ranking it as a rival.

**Kill condition.** `12 × p` exceeds the visit budget on band 15, i.e. the probe
cannot be made cheap enough to leave a real call affordable.

### (d) Two-tier detector → certifier

**Mechanism.** A cheap per-node detector marks candidates; a separate certifier
— the solver at a small cap — confirms before a full call. Structurally (a) or
(b) composed with (c), with state between the tiers.

**What it can rank out.** Whatever the cheap tier misses, plus whatever the
certifier's cap cannot reach.

**Cost shape.** The sum of its tiers, plus per-search STATE — which brings a
determinism seat and a `newgame` clear obligation the other rows do not have.

**Kill condition.** Either tier's kill condition fires; or the state cannot be
cleared observably at `newgame`, which is a determinism-law failure and not a
performance one.

### (e) Precision-ranked budget allocator over the current trigger

**Mechanism.** Keep the present trigger and add an ALLOCATOR: a per-search visit
budget, and at each firing a cheap precision SCORE from the same O(1) columns
the census records. Two shapes, and the design must pick one:

- **online threshold** — admit a firing iff its score clears a threshold and
  the budget remains. One pass, no lookahead, so the threshold is a knob and
  the "rank all firing points" the dispatch names is approximated rather than
  achieved.
- **iterative-deepening two-pass** — the search already visits the tree more
  than once. An early iteration records firing points and their scores; a later
  one calls only at the top-ranked ones within the remaining budget. This is
  the only shape in the field that can literally *rank all firing points and
  call top-budget*, because it is the only one that sees them all before
  choosing.

**What it can rank out.** Whatever the score mis-ranks — and the score is the
row's whole content, so this is the row whose recall gate matters most.

**Cost shape.** O(1) per firing to score, plus a small per-search store and a
budget counter — per-search STATE, so the determinism seat and the `newgame`
clear apply here too.

**Kill condition.** On the census, no score built from the recorded columns
separates proving firings from non-proving ones well enough to keep the recall
fixture inside the budget. **That is measurable from the census alone, before
any code is written**, which is why the census is taken first.

### (f) Null — the field's own answer that the target is unreachable

**Mechanism.** None. The trigger and the gate stay as they are, the gate stays
`false` in every committed config, and the package records that the bracket is
not reachable by a detector at HEAD.

**What it can rank out.** Everything.

**Cost shape.** Zero.

**Kill condition.** Inverted: this row WINS if every other row's kill condition
fires. **It is in the field so that "the target is unreachable" is a result the
field can express** rather than a conclusion someone has to argue for outside
it. D-471's roadmap clause is what it triggers.

---

## 4. HOW THE ROWS ARE RANKED — the unit, the definition, and the draw

**The instruments**: `crates/pistol-search/examples/trigger_census.rs` records
one row per firing with the O(1) columns a per-node detector could read there
(`turns`, `mover_hot`, `opp_hot`, `mover_w1`, `opp_w1`, `mover_l3`, `opp_l3`, and
since this revision **`cover`/`covers`**) and what the solver then answered
(`att_visits`, `att_proved`, `def_asked`, `def_visits`, `def_proved`);
`--gate off` runs the same seat unwired and prints each position's `T_off`.
`tools/stage3_census_rank.py` applies the tests below;
`crates/pistol-cli/examples/stage3_oos_positions.rs` draws the position sets;
`crates/pistol-search/examples/value_fixture_recall.rs` answers the gate D-512
pins per position. A change to any of them reopens this section.

**The workload**: `nodes 50000`, cap 2048, `quiet_radius 2` — the seats the
bracket was registered against, unchanged, because a workload change would make
the comparison a comparison of two things.

### 4.1 THE TEST IS IN VISITS

- **KEPT** = firings a row's predicate admits ÷ all firings.
- **WINS KEPT** = §4.2's recall.
- **REACHES BUDGET** = `sum(att_visits + def_visits over KEPT firings) / searches`
  ≤ the band's visit budget, where the budget is **`SHARE x T_off` measured on
  the same positions** and the search count includes searches that fire nothing.

**REVISION 1 PRICED A ROW IN FIRINGS AT THE POPULATION MEAN, AND THAT WAS THE
WRONG UNIT** — §1 of this document calls the visit *"what the BUDGET is
denominated in"*. That correction stands and is not re-argued (D-423). The
firing figure is still reported; it is not the test.

**AND THE BUDGET IS NOW DERIVED RATHER THAN CARRIED.** `T_off` is a property of
the POSITIONS — 50,176 on the in-sample band 15 and 41,826 on band 35, because a
band-35 search can exhaust its tree inside the 50,000-node budget — so a table
over new positions that reused the old denominator would be borrowing one. The
instrument measures it: `--gate off` on the in-sample draw reproduces
**50,176.0 / 41,826.2**, the figures `artifacts/stage3_call_budget_v3.txt`
records, to the last printed digit. That is the check that licenses reading the
out-of-sample denominators the same way.

### 4.2 THE RECALL DEFINITION — ONE, AND REVISION 2 USED TWO

**RECALL IS THE ATTACKER DIRECTION.** D-512's fixture is seven positions
`wp18b_probe` returned `win` on, and that probe asks `Solver::solve` — *does the
SIDE TO MOVE force a win*. A census row also records `solve_defender`, which
answers *does the OPPONENT force a win against the mover* — **a proven LOSS**.
`tools/stage3_census_rank.py` revision 1 scored `att_proved or def_proved` and
called the sum PROOFS KEPT.

**Separated, the in-sample denominators are:**

| band | WIN-direction proving firings | LOSS-direction |
|---|---|---|
| CORPUS band 15 | **0** | 8 |
| CORPUS band 35 | **1** | 1 |
| TRIGGER-RICH | **11** | 14 |

**Band 15 — the band revision 2's headline row was selected on — has no
win-direction proof at all.** Every recall cell on it is 0/0. And the mechanism
of the conflation is not subtle: `opp_hot` counts the OPPONENT's hot windows, so
it rises exactly where the DEFENDER direction proves. **A threshold fitted to
`opp_hot` is fitted to losses.**

**THE TENSION THIS EXPOSES, NAMED RATHER THAN EXPLOITED.** A defender-direction
proof is a real search result — the mover is lost, and the search scores it as a
mate. D-512 pins the gate on wins because the value fixture is win-direction, so
a row optimised for win-recall alone is optimised against a gate that prices
**none of the majority of what the solver currently finds** (8 of 8 on band 15,
14 of 25 on trigger-rich). This document does not resolve that; it is a ruling,
and it is listed in §6 as owed.

### 4.3 WHAT IS SCORED, AND WHAT IS UNSCORED

Rows (a), (g), (j) and — **for the first time** — **(b)** are predicates over the
census columns and are scored by the test above. Row (b) needed a cover column;
the census now has one, and `tools/stage3_census_rank.py` REFUSES a row (b)
predicate against a census that lacks it rather than scoring a substitute.
Row (i) is scored by re-charging every invocation at `min(visits, cap)`. Row (h)
is scored by collapsing repeated firing SIGNATURES, and its figure is an UPPER
bound. Rows (c), (d) and (k) stay UNSCORED for the reasons revision 2 gave.

### 4.4 THE ONE THING THE CENSUS CANNOT ANSWER, unchanged

KEPT is a fraction of TODAY's firings. A detector that gates firings changes the
search, which changes the firing set — the fixed point the premise memo's §3.6
named. The ranking compares rows; it never predicts the post-detector count.

---

## 5. THE MEASURED RANKING

### 5.0 The two criteria that license reading §5 at all

Both registered in `docs/experiments/stage3_oos_registration.md` before the runs.

- **CRITERION V — the draw instrument reproduces the draw everything else was
  measured on.** `stage3_oos_positions --round 0` prints 24 body lines equal to
  `bench_positions_v1.txt`'s 24, to the same sha256
  `42d32bd13f9131b151a24f4b404417b37ac2406b6851cdbfc9b7f786f7e8d702`. **PASS.**
- **CRITERION R — the re-taken census reproduces the standing one on every
  column they share.** 385 and 201 lines, `diff` empty once the two new columns
  are removed. **PASS**, so the cover column changed no search.

Round 1 shares **no game hash** with round 0 — 24 against 24, intersection 0.

### 5.1 What the incumbent trigger looks like

| | band 15 | band 35 | trigger-rich | **OOS band 15** | **OOS band 35** |
|---|---|---|---|---|---|
| searches | 12 | 12 | 20 | 12 | 12 |
| firings per search | 18.33 | 11.75 | 9.05 | 18.00 | 12.83 |
| **visits per search** | **49,100.5** | **39,917.1** | **28,113.4** | **49,264.2** | **36,817.5** |
| `T_off` measured | 50,176.0 | 41,826.2 | 30,499.8 | 46,519.9 | 37,911.5 |
| **visit budget** | **2183.7** | **937.7** | **1737.3** | **2024.5** | **850.0** |
| **required cut** | **22.49x** | **42.57x** | **16.18x** | **24.33x** | **47.25x** |
| **WIN-direction proofs** | **0** | **1** | **11** | **3** | **1** |
| LOSS-direction proofs | 8 | 1 | 14 | 0 | 1 |

**Where a figure differs from a REGISTERED one, the registered one has not
moved (D-374) — this is a new derivation on a corrected denominator, and both
are on the record.** D-516/D-517 register the trigger-rich budget at 1799.4
visits from the bench artifacts' 18 firing searches; this table derives 1737.3
from the same share and the same seat over **20**, because two of the twenty
searches fire nothing and a search that spends no solver visit is still a search
the budget was taken over. The required cut moves 17.36x → 16.18x with it. Band
15 and band 35 are unaffected: every search there fires. **THE
STRUCTURAL FACT** of revision 2's §5.1 — that `mover_hot > 0` at a firing means
the mover completes six THIS TURN, a theorem of rules 2, 3 and 5 — stands
unchanged and is not re-argued (D-423). It is also, this revision adds, exactly
why `solve_defender` is safe at a mover-hot firing: the race check answers before
its AND-root assert can fire (§5.6).

### 5.2 The rows, scored in visits, under the WIN definition

Per cell: KEPT / **WINS KEPT** / visits-per-search / cut / verdict. `—` is a
band with no win denominator, where recall is not measured and none is claimed.
Full artifact: `artifacts/stage3b_census_rank_v2.txt`.

**IN-SAMPLE** (the draw every threshold saw):

| row | band 15 | band 35 | trigger-rich |
|---|---|---|---|
| incumbent | 1.000 / — / 49100 / 1.0x / out | 1.000 / 1.000 / 39917 / 1.0x / out | 1.000 / 1.000 / 28113 / 1.0x / out |
| **(a/g) `opp_hot >= 2`** | 0.227 / — / 8546 / 5.75x / out | 0.404 / **0.000** / 15895 / 2.51x / out | 0.569 / **0.455** / 14806 / 1.90x / out |
| **(a/g) `opp_hot >= 3`** | 0.059 / — / **1176** / **41.76x** / IN | 0.057 / **0.000** / 2185 / 18.27x / out | 0.204 / **0.000** / 5001 / 5.62x / out |
| (a/g) `opp_hot >= 4` | 0.000 / — / 0 / — / IN (vacuous) | 0.007 / 0.000 / 126 / 317x / IN | 0.011 / 0.000 / 106 / 266x / IN |
| (a) a win-in-one-ply | 0.045 / — / 1803 / 27.23x / IN | 0.078 / 0.000 / 3053 / 13.08x / out | 0.072 / 0.000 / 2463 / 11.42x / out |
| (a) mover hot | 0.000 / — / 0 / — / IN (vacuous) | 0.007 / **1.000** / 0.1 / — / IN | 0.011 / **0.182** / 0.1 / — / IN |
| (j) the root only | 0.014 / — / 446 / 110x / IN | 0.035 / 1.000 / 1150 / 34.71x / out | 0.066 / 0.182 / 1456 / 19.32x / IN |
| **(b) cover impossible** | 0.000 / — / 0 / — / IN (vacuous) | 0.007 / **0.000** / 126 / 317x / IN | 0.011 / **0.000** / 106 / 266x / IN |
| (b′) exactly one cover | 0.086 / — / 3399 / 14.4x / out | 0.071 / 0.000 / 2712 / 14.72x / out | 0.066 / 0.000 / 2358 / 11.92x / out |
| **(m) mover hot or `mover_l3 >= 9`** | 0.000 / — / 0 / — / IN (vacuous) | 0.007 / **1.000** / 0.1 / — / IN | 0.271 / **1.000** / 8014 / 3.51x / out |

**OUT-OF-SAMPLE** (round 1, disjoint by game; no trigger-rich twin exists):

| row | OOS band 15 | OOS band 35 |
|---|---|---|
| incumbent | 1.000 / 1.000 / 49264 / 1.00x / out | 1.000 / 1.000 / 36818 / 1.00x / out |
| (a/g) `opp_hot >= 2` | 0.343 / **0.000** / 17274 / 2.85x / out | 0.416 / 1.000 / 16406 / 2.24x / out |
| **(a/g) `opp_hot >= 3`** | 0.153 / **0.000** / 7636 / **6.45x** / **out** | 0.117 / 1.000 / 4599 / **8.01x** / **out** |
| (a/g) `opp_hot >= 4` | 0.042 / 0.000 / 1746 / 28.21x / IN | 0.026 / 0.000 / 863 / 42.68x / out |
| (a) mover hot | 0.000 / 0.000 / 0 / — / IN (vacuous) | 0.006 / 1.000 / 0.1 / — / IN |
| (j) the root only | 0.019 / 0.000 / 788 / 62.54x / IN | 0.032 / 1.000 / 996 / 36.96x / out |
| **(b) cover impossible** | 0.000 / 0.000 / 0 / — / IN (vacuous) | 0.006 / 0.000 / 171 / 216x / IN |
| (m) mover hot or `mover_l3 >= 9` | 0.000 / 0.000 / 0 / — / IN (vacuous) | 0.006 / 1.000 / 0.1 / — / IN |

**`opp_hot >= 3` FALLS OUT OF SAMPLE BY A FACTOR OF SIX.** In sample it cuts
band 15 by 41.76x against a required 22.49x — comfortably in. Out of sample it
cuts by **6.45x against a required 24.33x**, and on band 35 by 8.01x against
47.25x. It is not close on either. The threshold was chosen after seeing which
firings proved, on the only draw that had been taken; §5.4 of revision 2 said so
and called it *"an argument for scoring the row, not for selecting it"*. Scored,
it does not survive.

**(i) THE CAP**, in-sample, with WIN recall as the test:

| cap | band 15 (0 wins) | band 35 | trigger-rich |
|---|---|---|---|
| 2048 (incumbent) | 49100 / — | 39917 / 1 of 1 | 28113 / **11 of 11** |
| 1024 | 25828 / — | 20165 / 1 of 1 | 14369 / **11 of 11** |
| 512 | 13451 / — | 10217 / 1 of 1 | 7451 / **9 of 11** |
| 256 | 7165 / — | 5225 / 1 of 1 | 3863 / **2 of 11** |

**Cap 512 costs two of the eleven trigger-rich wins**, which revision 2's
conflated column could not see; and no cap reaches a budget on its own.

**(h) THE VERDICT CACHE**, recall 1.000 by construction, an UPPER bound:

| | band 15 | band 35 | trigger-rich | OOS b15 | OOS b35 |
|---|---|---|---|---|---|
| distinct signatures | 94/220 | 68/141 | 103/181 | 120/216 | 69/154 |
| visits/search if a repeat is free | 19,977 | 20,061 | 14,550 | 25,864 | 16,859 |
| cut | 2.46x | 1.99x | 1.93x | 1.90x | 2.18x |

### 5.3 THE COMPOSITIONS

| composition | band 15 | band 35 | trigger-rich | **OOS b15** | **OOS b35** |
|---|---|---|---|---|---|
| `opp_hot >= 3` + cache | 923 / 53.2x / — | 1844 / 21.7x | 3607 / 7.79x / **0 of 10 wins** | 4706 / 10.47x / **0 of 3** / out | 3566 / 10.32x / out |
| cache + cap 512 | 5727 / 8.6x | 5113 / 7.8x | 3986 / 7.05x / 8 of 10 | 7153 / 6.89x / out | 4422 / 8.33x / out |
| **`opp_hot >= 3` + cache + cap 512** | **439 / 111.9x / —** | **481 / 83.0x** | **1036 / 27.14x / 0 of 10 wins** | **1258 / 39.17x / 0 of 3 / IN** | **910 / 40.47x / OUT (47.25x needed)** |
| cache + cap 4096 | — | — | 14,550 / 1.93x / **10 of 10** | 25,864 / 1.90x / out | 16,859 / 2.18x / out |
| (m) mover-side + cache | 0 / — / vacuous | 0.1 / — | 2254 / 12.47x / **10 of 10** | 0 / vacuous | 0.1 / — |

**The composition revision 2 headlined reaches all three IN-SAMPLE bands and
keeps ZERO WINS on every band that has a win to keep. Out of sample it also
misses band 35's budget.** Both halves of its case fail, independently.

**The only composition that keeps every win is `(m) mover-side + cache`, and it
reaches 12.47x against a required 16.18x** — the closest any row comes on the
band where wins actually live, and still short. It is also **in-sample only**:
`(m)` was read off the very firings it is scored against, and the trigger-rich
fixture has no out-of-sample twin to test it on (§5.5.1).

### 5.4 What §5.2 and §5.3 license

- **`opp_hot >= 3` IS DEAD ON BOTH TESTS INDEPENDENTLY** — 6.45x out of sample
  against 24.33x, and 0.000 win recall wherever a win denominator exists. Row
  (a)'s and row (g)'s count-threshold mechanism is not rescued by another value:
  `>= 2` is further out of budget and `>= 4` keeps no win anywhere.
- **ROW (b) IS NOW SCORED, AND IT IS DEAD.** With its own cover column it keeps
  **0.007 / 0.011 / 0.006** of firings and **0.000 wins on every band that has
  one**, in sample and out. `Cover::Impossible` fires 1 / 1 / 2 times in 731
  in-sample firings. Its KILLED verdict, withdrawn by revision 2 because
  revision 1 scored a substitute, is **RE-ENTERED on the mechanism itself** —
  and it never reached its registered per-node-cost kill condition, because it
  fails the ranking first.
- **THE DENOMINATORS ARE STILL SMALL AND THE OUT-OF-SAMPLE DRAW DID NOT FIX
  THAT.** Out-of-sample band 35 has **ONE** win-direction proof. **Every claim
  about band 35's win recall in this document rests on n = 1**, in sample and
  out — one proof of one position, at one visit, found because the mover is hot,
  which is the rules theorem. Band 15 has 0 in sample and 3 out of sample.
  **The only band with a denominator worth the name is trigger-rich, at 11 —
  and it is the one band with no out-of-sample twin.**
- **NO ROW'S REGISTERED KILL CONDITION HAS FIRED ON COST.** Rows (a) and (e) are
  killed by §3 *"excluding a VALUE row of the recall fixture"*; §5.6 shows why
  that condition cannot fire as written.

### 5.5 THE SAMPLE THIS ARC CANNOT WIDEN

`bench_solver_positions_v1.txt` is twenty late-game positions from the two
distinct sealbot-anchor games. There is no third anchor game, so **the only band
carrying a real win denominator cannot be drawn again**, and any row fitted on
it — `(m)` included — is quotable in sample only. That is a property of the
evidence available at HEAD and not a finding about any row; it is stated here
because a selection resting on trigger-rich would be resting on a set that
cannot be validated, which selection rule 1 of the registration forbids.

### 5.6 THE RECALL FIXTURE, RUN DIRECTLY — and FOUR OF SEVEN ROWS ARE NOT
TRIGGER POINTS

`artifacts/stage3b_value_fixture_v1.txt`, the seven rows of D-512 with the
columns a detector would read at each and the solver's answer at five caps.

| row | class | mover_hot | opp_hot | cover | trigger fires? | proof |
|---|---|---|---|---|---|---|
| `g001-t44-p2` | VALUE | 0 | 0 | none | **NO** | — |
| `g001-t46-p2` | VALUE | 2 | 2 | minimal(2) | yes | `win` at **1 visit**, every cap |
| `g002-t12-p2` | VALUE | 0 | 0 | none | **NO** | — |
| `g002-t39-p1` | VALUE | 0 | 0 | none | **NO** | — |
| `g002-t41-p1` | VALUE | 2 | 2 | minimal(2) | yes | `win` at **1 visit**, every cap |
| `g001-t42-p2` | CALL-RECALL-ONLY | 0 | 1 | minimal(2) | yes | `win` at **10,726**, cap ≥ 16384 only |
| `g002-t10-p2` | CALL-RECALL-ONLY | 0 | 0 | none | **NO** | — |

**THE GATE D-512 REGISTERS CANNOT BE EVALUATED ON FOUR OF ITS SEVEN ROWS.** It
asks that *"the proof-bearing trigger point RANK INSIDE the call budget"*, and at
four of them the incumbent trigger does not fire at all: neither side holds a hot
window, so there is no trigger point, and no detector — however tight or however
loose — can change what happens there. Their proofs are real; they were taken by
calling the solver DIRECTLY, which is what `wp18b_probe` does and what the search
never does.

**AND THE TWO EVALUABLE VALUE ROWS DISCRIMINATE ALMOST NOTHING.** Both fire with
the MOVER hot, and both prove in **one visit** — the one-visit root win that
§5.1's theorem explains. Any predicate admitting `mover_hot > 0` passes them, at
a cost of one visit. **CRITERION C (the cap) therefore passes for every cap down
to 1**, and that is not a licence: it is a gate with no discriminating power,
which is the premise memo's §5 finding 3 arriving in a stronger form than it was
stated — the fixture is not seven hot positions, it is three.

**One row is a real cap constraint and it is CALL-RECALL-ONLY.**
`g001-t42-p2` needs **cap ≥ 16384**; at 4096 both directions return `unknown`
after spending 8,192 visits between them. D-512 gates that row on ranking and
never on proving, precisely so a gate does not ship red on correct code — so the
16384 does not bind. **But it does say what a cap-512 stack is buying**: at 512
this position spends 1,024 visits and proves nothing.

**HOW THIS WAS FOUND, because the route matters.** Probing `g002-t12-p2` at cap
512 PANICKED — `SOLVER_NO_PLAN` in `dfpn.rs`. That is not an engine defect and
the engine cannot reach it: `Solver::solve_defender`'s root is an AND node, which
requires the attacker — the opponent — to hold a hot window, and the search's
trigger guarantees exactly that disjunction (a hot MOVER is answered by the race
check first). The instrument was asking a question the engine never asks. **The
precondition was undocumented and is now documented and pinned by a test**, with
a five-stone reproducer and a mutant that dies when the assert is removed
(`crates/pistol-solver/tests/defender_precondition_tests.rs`,
`artifacts/stage3b_mutations_v1.txt`).

## 5.7 The rows the field was missing (added at revision 2)

Added on the DECISION-RED-TEAM's finding that revision 1's field omitted the
mechanisms the census ranks best (D-511's flip clause: a red team may add a row
before selection, and the dispatch's field is a floor — *"at least"*).

### (g) Count-threshold trigger, quantified over the OPPONENT

**Mechanism.** The `SolverTrigger` variant §3(a) already offers, with the count
as a parameter and quantified over `threats.hot_windows(opponent).len()`. One
comparison at the two sites the incumbent is evaluated at. O(1) by inspection.

**What it can rank out.** Positions where the opponent holds fewer than the
threshold's hot windows.

**Cost shape.** A comparison; `t` free by §3(a)'s own argument.

**Kill condition.** No threshold reaches the budget on all three bands while
keeping a VALUE row of D-512's fixture.

### (h) Verdict cache

**Mechanism.** A per-search map from the position key `visit` already probes the
TT with, consulted at the top of `solver_verdict` before either invocation and
written after. Per-search STATE, so the determinism seat and the `newgame` clear
apply.

**What it can rank out.** **Nothing** — a cache returns the answer the solver
would have returned. The only row in or out of the field with recall 1.000 by
construction.

**Cost shape.** One lookup per firing, not per node; a map bounded by the
distinct firing positions (94 / 68 / 102 measured).

**Kill condition.** A run counting DISTINCT `state.key()` values at firings finds
them close to the firing count — i.e. the signature collisions §5.2 counts are
different positions. One counter and one run.

### (i) The per-call cap as a lever

**Mechanism.** No new predicate: `per_call_node_cap` is already config. D-516
records the finding — *"the visit budget is fixed by the bracket and
`count = visits/cap`"* — and revision 1 quoted D-516 for the budget and dropped
this clause.

**What it can rank out.** Proofs deeper than the cap.

**Cost shape.** Zero code.

**Kill condition.** The cap that reaches the budget on the worst band drops a
VALUE row of D-512's fixture — which this row's bench can evaluate, because the
fixture's five VALUE rows have cap-conditioned statuses in the premise memo §5.

### (j) Gate the ROOT separately from the tree

**Mechanism.** A second config token for the root's own firing, independent of
the in-tree predicate; the two sites are already separate functions.

**What it can rank out.** Whatever the root proves — measured sharply
asymmetric: 0 of band 15's 8 proofs, **2 of band 35's 2**, 5 of trigger-rich's
25, at 446 / **1150** / 1617 visits per search against budgets of 2184 / **938**
/ 1799. On band 35 the root alone exceeds the whole budget while carrying every
proof that band has.

**Cost shape.** Zero per node; one config token.

**Kill condition.** Neither setting is dominant on any band — i.e. the asymmetry
does not survive a second workload.

### (k) Subtree / per-turn quota suppression — **UNSCORED**

**Mechanism.** After a firing returns NoWin, suppress firings in that node's
subtree, or for the next N nodes at the same or greater `turns_from_root` — a
decaying quota rather than a per-search cap.

**Why UNSCORED.** The census records `turns_from_root` but not parent-child
identity, so subtree containment is not derivable from these rows. §4.2's
registered disposition applies; it survives to selection unscored.

---

## 6. RECOMMENDATION

REGISTERED SLOT — written after a fresh-context DECISION-RED-TEAM has attacked
**this revision**, and quoting its rows. **Nothing is selected until then.**

What this revision may say is what it measured, and it is a harder statement
than revision 2's: **no row and no composition in the field satisfies the
registered selection rule.** The rule's four conjuncts and where the field
stands against them:

1. **BUDGET, out of sample, both corpus bands.** The only composition that
   reaches OOS band 15 is `opp_hot >= 3 + cache + cap 512` at 39.17x, and it
   misses OOS band 35 at 40.47x against 47.25x. Nothing else is close.
2. **RECALL, census.** That composition keeps **0 of 3** OOS band-15 wins and
   **0 of 10** trigger-rich wins. The only rows keeping every win — `(m)` and
   the cache alone — reach 12.47x and 1.93x against a required 16.18x.
3. **RECALL, fixture.** Passes vacuously: four of seven rows are not trigger
   points and the two evaluable VALUE rows prove in one visit.
4. **NOT VACUOUS.** Every row that reaches a budget on OOS band 15 does so by
   admitting nothing at all, except the one that fails conjunct 1.

**Conjuncts 1 and 2 fail together, out of sample, for every row in the field.**
Whether that is the registered kill point is the red team's to attack and the
operator's to confirm; §7 states what would have to be true for it not to be.

## 7. WHAT IS OWED, and what would have to be true for the field to survive

Three rulings this revision raises and does not take:

1. **THE GATE'S DIRECTION.** §4.2: the recall gate prices wins, the incumbent
   trigger's proofs are majority losses, and a row optimised for one is
   unmeasured on the other. Either the gate widens to both directions — which
   changes every recall cell in §5.2 and is a new measurement, not a re-reading
   — or the arc records that it is selecting on the minority of what the solver
   finds. **This is the one ruling that could change the verdict**, and it
   cannot be taken by the session that would benefit from it.
2. **THE FIXTURE'S FOUR NON-FIRING ROWS.** §5.6: D-512's ranking gate is
   unevaluable on four of seven rows. Either the fixture is restated as the
   three positions the trigger fires at — which makes it a three-row gate whose
   two VALUE rows prove in one visit — or the gate is re-specified to ask
   something a non-firing position can answer. D-512's flip clause covers a
   VALUE row moving to CALL-RECALL-ONLY; it does not cover this.
3. **THE TRIGGER-RICH SAMPLE.** §5.5: the only band with a win denominator
   worth the name cannot be drawn again at HEAD. Either a third source of
   trigger-rich positions is commissioned — its own work package — or no row
   fitted there is selectable under the registered rule.

**What would have to be true for the field to survive as it stands**: a row
reaching **24.33x and 47.25x out of sample** while keeping every win, from a
predicate over the columns the census records. §5.2 contains no such row, and
§5.3 contains no such composition. The oracle bound says one could exist in
principle — keeping exactly the trigger-rich win-proving firings costs 257.5
visits per search against a 1737.3 budget, a 121x cut — so the obstacle measured
here is not that proofs are unaffordable. **It is that nothing a detector can
read at a node separates them**, and this revision looked at every column the
census has, including the one it added.
