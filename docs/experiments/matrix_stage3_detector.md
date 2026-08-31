# Stage-3 detector — OPTION MATRIX

**REVISION 4.** Revision 3 (`b6962ac`) took a fresh-context DECISION-RED-TEAM
and **FELL**: 3 BLOCKING, 7 MAJOR, 5 MINOR, twelve enumerated remedies. All
twelve are applied here. **Two of the three BLOCKING findings were defects in
the MEASUREMENT, not in the prose**, and fixing them moved every number in §5:

| the red team found | what revision 4 did |
|---|---|
| **B-1.** The out-of-sample band-35 required cut of **47.25x** appears in no artifact; the instrument says 43.32x. 47.25 is a numerator over 11 firing searches against a denominator over 12 — the borrowed-denominator defect the instrument was *corrected for* and prints a `NOTE:` about | the figure is **gone from all seven sites**. On the corrected seat the cut is **43.20x** and it is transcribed from the artifact |
| **B-2.** The `--gate off` seat carried ONE transposition table across every fixture entry (D-7's own named hazard), so `T_off` was measured **warm** while the SHARE was derived on a cold bench seat. Trigger-rich entries 7/8/17 read 239/186/180 against the bench's 7795/7742/6886 — and §4.2's licensing check passed only on the two bands where the defect **cannot** show | `trigger_census` now **clears between entries**, and the red team's predicted confirming values came back **exactly**. Every census was re-taken. **All three registered `T_off` values now reproduce — 50,176.0 / 41,826.2 / 31,590.7** — where the contaminated seat reproduced two and missed the third |
| **B-3.** §5.1's justification for moving the registered trigger-rich budget 1799.4 → 1737.3 was false: the registered figure was already over 20 searches | **the move is withdrawn and no registered number has moved.** On the corrected seat the budget IS **1799.4** and the incumbent IS **27,504.7 visits/search** — D-516's own recorded figure, reproduced to the last digit |
| **M-6.** §1 declares the INVOCATION a unit and no row touches it; the defender direction is 57–67 % of every band's visits and is free in win-recall by construction | row **(n)** is in the field, scored, and it is the largest single lever in the arc |
| **M-2.** *"DEAD ON BOTH TESTS INDEPENDENTLY"* is false under D-512's registered census instrument | **withdrawn.** §4.2 now says which half of the kill is definition-dependent |
| **M-7.** the four non-firing fixture rows were established at the ROOT only | **measured in-tree**, and the finding is now smaller and sharper (§5.6) |

**Nothing is selected.** The red team's own verdict on the conclusion is quoted
in §6 and is not paraphrased.

**What governs the new numbers.**
`docs/experiments/stage3_oos_registration.md`, written before either run. Its
Criterion V (round 0 reproduces `bench_positions_v1.txt`) passes; the red team
re-checked it byte for byte and by sha256. **Criterion R is now FAILED by name
and that is a finding, not a fault** — see §5.0.

**The field is the operator's** and is a FLOOR, not a closed set: the governing
dispatch reads *"options ranked by precision economics, **at least**: (a)…(f)"*
(`docs/experiments/stage3_overnight_dispatch.md` §3, D-511). Row (n) enters
under that clause, added by the red team before selection.

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
- the incumbent spends **18.42 / 11.75 / 9.05** firings per search and the
  budget is **0.818–0.848 / 0.276–0.286 / 0.592–0.612**, so the required
  reduction is **21.7x / 41.1x / 14.8x** — D-517 registers 18.33, 0.579–0.599
  and 15.1x from the WARM census (§5.1), and these are the same quantities on
  the corrected seat. **Those are FIRING
  figures**; §5's test is in VISITS and its cuts are **22.50x / 42.51x /
  15.29x** in sample and **24.27x / 43.20x** out of it. The two are the same
  constraint in two units and neither replaces the other;
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

### 4.2 THE RECALL DEFINITION — one is USED, and it is CONTESTED

**RECALL IS REPORTED IN BOTH DIRECTIONS AND THE VERDICT COLUMN READS WINS.**

`wp18b_probe`, which produced D-512's fixture, asks `Solver::solve` — *does the
SIDE TO MOVE force a win* — so the fixture's seven rows are attacker-direction
WINS. A census row also records `solve_defender`: *does the OPPONENT force a win
against the mover*, a proven LOSS. `tools/stage3_census_rank.py` revision 1
scored `att_proved or def_proved` and called the sum PROOFS KEPT, so the row
revision 2 headlined was chosen against a denominator that is not the fixture's.

**BUT NARROWING THE CENSUS GATE TO WINS IS THIS DOCUMENT'S READING AND NOT
D-512's, AND REVISION 3 PRESENTED IT AS D-512's.** The red team is right and the
correction is made here. D-512 registers **two** gates and names a different
instrument for each: the per-position **ranking** gate, whose direction the
fixture does fix; and *"the recall denominator measured over a governed search
at the detector's own bench **using D-510's `proofs` counter**"* — and that
counter increments in both directions (`crates/pistol-search/src/pvs.rs`, the
attacker `Win` arm and the defender `Win` arm; the root's two in `search.rs`).
D-512's own next sentence convicts the inference it warns against: a first draft
*"claimed the first supplied the second"*. Revision 3 made that move in the
opposite direction.

**So both columns are printed, and neither is summed into the other.** The
verdict column reads WINS because that is the direction the value fixture is
stated in; **which direction the CENSUS gate should read is ruling §7.1 and is
the operator's.** This document does not take it, and §5 states what changes
under each.

**The denominators, separated, on the corrected seat:**

| band | WIN-direction proving firings | over distinct POSITIONS | LOSS-direction | over positions |
|---|---|---|---|---|
| CORPUS band 15 | **0** | — | 8 | **1** |
| CORPUS band 35 | **1** | 1 | 1 | 1 |
| TRIGGER-RICH | **14** | 4 | 14 | **6** |
| **OOS band 15** | **3** | 1 | 0 | — |
| **OOS band 35** | **1** | 1 | 1 | 1 |

**NO BAND EXCEEDS FOUR DISTINCT POSITIONS, and three of the five are ONE.**
Every recall figure in §5 rests on that; the firing counts are repeated visits
to a handful of positions and are not independent evidence.

**Band 15 in sample has no win-direction proof at all**, so no row's win recall
is measurable there and a row reading 1.000 on that band is reading losses.

**THE TENSION, NAMED RATHER THAN EXPLOITED.** A defender-direction proof is a
real search result — the mover is lost, and the search scores it as a mate. The
incumbent trigger's proofs are half or more of them losses on every band. A row
optimised for win-recall alone is unmeasured on that half, and row (n) below is
the reductio: it scores perfectly under a win-only gate precisely by discarding
every loss. §7.1 is where this is owed.

### 4.3 WHAT IS SCORED, AND WHAT IS UNSCORED

Rows (a), (g), (j) and — since revision 3's census gained a cover column —
**(b)** are predicates over the census columns. Row (i) is scored by re-charging
every invocation at `min(visits, cap)`; row (h) by collapsing repeated firing
SIGNATURES (an UPPER bound); **row (n) by charging the attacker invocation
only**; and **row (e) by `tools/stage3_allocator_bound.py`, separately, because
an allocator is not a predicate** (§5.8). Rows (c), (d) and (k) stay UNSCORED
for the reasons revision 2 gave.

### 4.4 THE TWO THINGS THE CENSUS CANNOT ANSWER

KEPT is a fraction of TODAY's firings, and a detector that gates firings changes
the search, which changes the firing set — the fixed point the premise memo's
§3.6 named. The ranking compares rows; it never predicts the post-detector
count. **And the trigger-rich fixture has no out-of-sample twin** (§5.5), so the
one band carrying a real win denominator cannot be validated at HEAD.

---

## 5. THE MEASURED RANKING

Everything below is from `artifacts/stage3c_census_rank_v2.txt` and
`artifacts/stage3c_allocator_bound_v1.txt`, on censuses that clear between
entries. **The `stage3b_*` artifacts are superseded and are kept because the
round-2 red-team report quotes them.**

### 5.0 The criteria, and the one that now FAILS

- **CRITERION V — the draw instrument reproduces the draw everything else was
  measured on.** `stage3_oos_positions --round 0` prints 24 body lines equal to
  `bench_positions_v1.txt`'s, sha256
  `42d32bd13f9131b151a24f4b404417b37ac2406b6851cdbfc9b7f786f7e8d702`. **PASS**,
  independently re-checked by the red team. Round 1 shares no game hash and no
  position with round 0.
- **CRITERION R — the re-taken census reproduces the standing one on every
  shared column. FAILED, DELIBERATELY, AND THAT IS THE FINDING.** The registered
  consequence of a Criterion R failure is that *"the column changed the search
  and the run is VOID"*. It did not: what changed the search is the seat fix
  B-2 forced. Measured, entry by entry: the corpus census differs on **3 of 24**
  entries (361 → 362 firings), the trigger-rich on **8 of 20** (181 → 181
  firings, 562,268 → 550,093 solver visits), the out-of-sample on **6 of 24**.
  **So the standing censuses — and D-517's `K` with them — were taken on a warm
  table**, and the correct reading is that revision 4 supersedes them rather
  than that this run is void. **The check that replaces Criterion R is
  stronger**: the corrected OFF seat reproduces all three registered `T_off`
  values, 50,176.0 / 41,826.2 / **31,590.7**, and the corrected ON seat
  reproduces D-516's own recorded trigger-rich figure of **27,504.7 solver
  visits per search** — two independent cold-seat anchors the contaminated run
  missed.
- **THE LICENSING CHECK IS NO LONGER VACUOUS.** The red team showed revision 3's
  version could not fail on bands 15 and 35 for a structural reason — their OFF
  entries sit at the 50,176 node ceiling, which a warm table cannot lower — so
  it passed on the two bands that could not test it and failed on the one that
  could. It now passes on the band where it failed, which is the only version of
  the check that is evidence.

### 5.1 What the incumbent trigger looks like

| | band 15 | band 35 | trigger-rich | **OOS band 15** | **OOS band 35** |
|---|---|---|---|---|---|
| searches | 12 | 12 | 20 | 12 | 12 |
| firings per search | 18.42 | 11.75 | 9.05 | 18.00 | 12.83 |
| **visits per search** | **49,137.1** | **39,867.1** | **27,504.7** | **49,135.2** | **36,720.8** |
| `T_off`, measured cold | 50,176.0 | 41,826.2 | 31,590.7 | 46,519.9 | 37,911.5 |
| **visit budget** | **2183.7** | **937.7** | **1799.4** | **2024.5** | **850.0** |
| **required cut** | **22.50x** | **42.51x** | **15.29x** | **24.27x** | **43.20x** |
| **WIN-direction proofs** (positions) | **0** (—) | **1** (1) | **14** (4) | **3** (1) | **1** (1) |
| LOSS-direction proofs (positions) | 8 (**1**) | 1 (1) | 14 (**6**) | 0 (—) | 1 (1) |

**THE TWO REGISTERED FIGURES THIS DOCUMENT'S TEST CONSUMES COME BACK EXACTLY**
— the trigger-rich visit budget **1799.4** and the incumbent **27,504.7 visits
per search**, both D-516's own. Revision 3 reported 1737.3 and 28,113.4 and
justified the move with a denominator claim the round-2 red team falsified; the
true cause was the warm seat, and with the seat fixed they return.

**BUT "NO REGISTERED NUMBER HAS MOVED" WOULD BE FALSE, AND A FIRST DRAFT OF THIS
REVISION WROTE IT.** §5.0 says the standing censuses were warm, and a warm
census moved figures D-517 registers: band 15's firings per search **18.33 →
18.42**, `K` on all three bands (**1339.1/1704.6/1601.9 → 1334.0/1702.5/
1580.7**), invocations per firing on trigger-rich (1.939 → **1.923**), and with
them **the trigger-rich FIRING factor 15.1x → 14.8x** — so D-527's claim that
D-517's factor is untouched holds for bands 15 and 35 and **not** for
trigger-rich. What does not move is what §5's test reads, because the visit
budget and the visit incumbent are the two figures it consumes. The trigger-rich
WIN denominator moved **11 → 14**: the warm table was suppressing three proofs.

**THE STRUCTURAL FACT** of revision 2's §5.1 — that `mover_hot > 0` at a firing
means the mover completes six THIS TURN, a theorem of rules 2, 3 and 5 — stands
unchanged and is not re-argued (D-423).

### 5.2 The rows, scored in visits

Per cell: KEPT / **WINS KEPT** / visits-per-search / cut / verdict. `—` marks a
band with no win denominator, where no recall is measured and none is claimed.

**IN-SAMPLE:**

| row | band 15 | band 35 | trigger-rich |
|---|---|---|---|
| incumbent | 1.000 / — / 49137 / 1.00x / out | 1.000 / 1.000 / 39867 / 1.00x / out | 1.000 / 1.000 / 27505 / 1.00x / out |
| (a/g) `opp_hot >= 2` | 0.226 / — / 8546 / 5.75x / out | 0.404 / 0.000 / 15844 / 2.52x / out | 0.586 / 0.429 / 15168 / 1.81x / out |
| **(a/g) `opp_hot >= 3`** | 0.059 / — / 1175 / 41.80x / IN | 0.057 / 0.000 / 2185 / 18.24x / out | 0.204 / 0.000 / 5001 / 5.50x / out |
| (a/g) `opp_hot >= 4` | 0.000 / — / 0.0 / — / IN (vacuous) | 0.007 / 0.000 / 126 / 316.61x / IN | 0.011 / 0.000 / 106 / 260.21x / IN |
| (a) a win-in-one-ply | 0.045 / — / 1803 / 27.25x / IN | 0.078 / 0.000 / 3053 / 13.06x / out | 0.072 / 0.000 / 2462 / 11.17x / out |
| (a) mover hot | 0.000 / — / 0.0 / — / IN (vacuous) | 0.007 / 1.000 / 0.1 / 478405.00x / IN | 0.011 / 0.143 / 0.1 / 275046.50x / IN |
| (j) the root only | 0.014 / — / 446 / 110.21x / IN | 0.035 / 1.000 / 1150 / 34.67x / out | 0.066 / 0.143 / 1456 / 18.90x / IN |
| **(b) cover impossible** | 0.000 / — / 0.0 / — / IN (vacuous) | 0.007 / 0.000 / 126 / 316.61x / IN | 0.011 / 0.000 / 106 / 260.21x / IN |
| (b′) exactly one cover | 0.045 / — / 1803 / 27.25x / IN | 0.071 / 0.000 / 2712 / 14.70x / out | 0.066 / 0.000 / 2358 / 11.67x / out |
| (m) mover hot or `mover_l3 >= 9` | 0.000 / — / 0.0 / — / IN (vacuous) | 0.007 / 1.000 / 0.1 / 478405.00x / IN | 0.260 / 1.000 / 7069 / 3.89x / out |

The band-15 `(b′)` cell is transcribed from the artifact; revision 3 printed
`0.086 / 3399 / 14.4x / out`, a figure no artifact contains (round-2 M-3).

**OUT-OF-SAMPLE** (round 1; no trigger-rich twin exists):

| row | OOS band 15 | OOS band 35 |
|---|---|---|
| incumbent | 1.000 / 1.000 / 49135 / 1.00x / out | 1.000 / 1.000 / 36721 / 1.00x / out |
| (a/g) `opp_hot >= 2` | 0.343 / 0.000 / 17216 / 2.85x / out | 0.416 / 1.000 / 16376 / 2.24x / out |
| **(a/g) `opp_hot >= 3`** | 0.153 / 0.000 / 7578 / 6.48x / out | 0.117 / 1.000 / 4599 / 7.99x / out |
| (a/g) `opp_hot >= 4` | 0.042 / 0.000 / 1729 / 28.41x / IN | 0.026 / 0.000 / 863 / 42.57x / out |
| (a) a win-in-one-ply | 0.102 / 0.000 / 5123 / 9.59x / out | 0.110 / 0.000 / 4309 / 8.52x / out |
| (a) mover hot | 0.000 / 0.000 / 0.0 / — / IN (vacuous) | 0.006 / 1.000 / 0.1 / 440649.00x / IN |
| (j) the root only | 0.019 / 0.000 / 788 / 62.37x / IN | 0.032 / 1.000 / 996 / 36.86x / out |
| **(b) cover impossible** | 0.000 / 0.000 / 0.0 / — / IN (vacuous) | 0.006 / 0.000 / 171 / 215.06x / IN |
| (b′) exactly one cover | 0.088 / 0.000 / 4597 / 10.69x / out | 0.097 / 0.000 / 3963 / 9.27x / out |
| (m) mover hot or `mover_l3 >= 9` | 0.000 / 0.000 / 0.0 / — / IN (vacuous) | 0.006 / 1.000 / 0.1 / 440649.00x / IN |

**`opp_hot >= 3` FALLS OUT OF SAMPLE BY A FACTOR OF SIX AND A HALF.** In sample
it cuts band 15 by 41.80x against a required 22.50x; out of sample by **6.48x
against 24.27x**. On band 35 it answers 7.99x against 43.20x. **The fall is in
the NUMERATOR — 1,175 visits/search becomes 7,578 — and not in the budget**, so
the round-1 draw's small conditioning effect (§5.5.1) does not explain it.

**(i) THE CAP**, with each direction's recall separate:

| cap | band 15 (0 wins) | band 35 | trigger-rich | OOS b15 |
|---|---|---|---|---|
| 2048 | 49137 / 8 of 8 losses | 39867 / 1 of 1 | 27505 / 14 of 14 | 49135 / 3 of 3 |
| 1024 | 25917 / 7 of 8 losses | 20165 / 1 of 1 | 14118 / 14 of 14 | 25562 / 0 of 3 |
| 512 | 13497 / 5 of 8 losses | 10217 / 1 of 1 | 7374 / 10 of 14 | 13134 / 0 of 3 |
| 256 | 7190 / 1 of 8 losses | 5225 / 1 of 1 | 3825 / 2 of 14 | 6798 / 0 of 3 |

**Cap 512 costs four of fourteen trigger-rich wins and all three out-of-sample
band-15 wins.** No cap reaches a budget alone.

**(h) THE VERDICT CACHE**, recall 1.000 by construction, an UPPER bound:

| | band 15 | band 35 | trigger-rich | OOS b15 | OOS b35 |
|---|---|---|---|---|---|
| distinct signatures | 96/221 | 68/141 | 109/181 | 122/216 | 71/154 |
| visits/search if a repeat is free | 20,188 | 20,018 | 15,350 | 26,249 | 17,278 |
| cut | 2.43x | 1.99x | 1.79x | 1.87x | 2.13x |

**(n) GATE THE DIRECTION — the largest single lever in the arc**, and free in
win recall by construction because `att_proved` is decided before
`solve_defender` is reached:

| band | attacker-only visits/search | cut | defender's share | losses given up |
|---|---|---|---|---|
| band 15 | 16,135 | 3.05x | **67.2 %** | 8 |
| band 35 | 16,401 | 2.43x | **58.9 %** | 1 |
| trigger-rich | 11,690 | 2.35x | **57.5 %** | 14 |
| OOS b15 | 16,378 | 3.00x | **66.7 %** | 0 |
| OOS b35 | 11,962 | 3.07x | **67.4 %** | 1 |

### 5.3 THE COMPOSITIONS

Per cell: visits/search / cut / distinct WINS kept / verdict. `—` marks a band
with no win denominator.

| composition | band 15 | band 35 | trigger-rich | OOS b15 | OOS b35 |
|---|---|---|---|---|---|
| `opp_hot>=3` + cache | 923 / 53.23x / — / IN | 1,844 / 21.62x / 0/1 / out | 3,607 / 7.62x / 0/12 / out | 4,821 / 10.19x / 0/3 / out | 3,566 / 10.30x / 1/1 / out |
| cache + cap 512 | 5,819 / 8.44x / — / out | 5,113 / 7.80x / 1/1 / out | 4,242 / 6.48x / 8/12 / out | 7,282 / 6.75x / 0/3 / out | 4,554 / 8.06x / 1/1 / out |
| **`opp_hot>=3` + cache + cap 512** | 439 / 111.99x / — / IN | 481 / 82.91x / 0/1 / IN | 1,036 / 26.55x / 0/12 / IN | 1,302 / 37.74x / 0/3 / IN | 910 / 40.36x / 1/1 / out |
| cache + cap 4096 | 20,188 / 2.43x / — / out | 20,018 / 1.99x / 1/1 / out | 15,350 / 1.79x / 12/12 / out | 26,249 / 1.87x / 3/3 / out | 17,278 / 2.13x / 1/1 / out |
| (m) + cache | 0.0 / — / vacuous / IN | 0.1 / 478405.00x / 1/1 / IN | 2,308 / 11.92x / 12/12 / out | 0.0 / — / 0/3 / IN | 0.1 / 440649.00x / 1/1 / IN |
| **(n) att-only + cache** | 7,342 / 6.69x / — / out | 8,908 / 4.48x / 1/1 / out | 6,685 / 4.11x / 12/12 / out | 9,252 / 5.31x / 3/3 / out | 6,242 / 5.88x / 1/1 / out |
| (n) att-only + cache + cap 512 | 2,070 / 23.74x / — / IN | 2,296 / 17.36x / 1/1 / out | 1,941 / 14.17x / 8/12 / out | 2,503 / 19.63x / 0/3 / out | 1,696 / 21.65x / 1/1 / out |
| **(m) + (n) att-only + cache** | 0.0 / — / vacuous / IN | 0.1 / 478405.00x / 1/1 / IN | 1,284 / 21.42x / 12/12 / IN | 0.0 / — / 0/3 / IN | 0.1 / 440649.00x / 1/1 / IN |

**REVISION 3's SENTENCE IS WITHDRAWN.** It read *"the only composition that keeps
every win … reaches 12.47x against a required 16.18x … and still short"*.
**`(m) + (n) att-only + cache` reaches 21.42x against a required 15.29x on
trigger-rich while keeping 12 of 12 distinct wins** — the first composition in
this arc to reach a bracket with full win recall. The round-2 red team found it,
in the unit §1 declares and the field had no row on.

**AND IT IS STILL NOT SELECTABLE.** `(m)` is `mover_hot > 0 or mover_l3 >= 9`,
and band 15 holds no firing with `mover_l3 >= 9` in either draw — its maximum is
6 — so the composition **keeps nothing at all** on both band-15 draws, taking
0 of the 3 out-of-sample wins. It reaches every budget by admitting nothing,
which is selection conjunct 4's exact case: row (f) wearing a mechanism.

**EVERY CELL OF §5.2 AND §5.3 IS RENDERED FROM
`artifacts/stage3c_census_rank_v2.txt` BY MACHINE.** Revision 4's first draft
hand-copied them and a scoped re-review found twenty-six wrong, nine of them
figures no artifact contained. A table a human retypes is a table that drifts
from its artifact, so this one is generated.

### 5.4 What §5.2 and §5.3 license

- **`opp_hot >= 3` IS DEAD OUT OF SAMPLE, AND THAT HALF OF THE KILL IS
  DEFINITION-FREE**: 6.48x against 24.27x, 7.99x against 43.20x, under any
  recall definition, with the fall in the numerator.
- **THE OTHER HALF IS DEFINITION-DEPENDENT AND REVISION 3 CLAIMED OTHERWISE.**
  Its *"DEAD ON BOTH TESTS INDEPENDENTLY"* is **WITHDRAWN**: under D-512's own
  registered census instrument (D-510's `proofs`, both directions) the row keeps
  **1.000 of band 15's 8 in-sample proofs at 41.80x, INSIDE the budget**. Which
  instrument the census gate uses is ruling §7.1. **Those eight proofs are ONE
  POSITION** — entry 9 of the band — so the surviving half of that row's case is
  one position's worth of evidence, and this document says so where it is
  quoted rather than only in §4.2's table.
- **ROW (b) IS SCORED ON ITS MECHANISM AND IS DEAD.** `Cover::Impossible` fires
  0 / 1 / 2 times in 542 in-sample firings and 0 / 1 in 370 out-of-sample, and
  keeps 0.000 wins wherever a win exists. It never reaches its registered
  per-node-cost kill condition because it fails the ranking first, so this
  document makes no claim about its cost.
- **ROW (n) IS THE ARC'S LARGEST LEVER AND ITS KILL CONDITION IS A RULING.** It
  is free in `t`, free in win recall by construction, and worth 57–67 % of every
  band's solver visits. It gives up every proven loss — so **row (n) and §7.1
  are the same decision**, which is why it belongs in the field.
- **EVERY WIN DENOMINATOR IS 1 TO 4 DISTINCT POSITIONS.** Trigger-rich's 14
  firings are 4 positions; both band-35 draws and out-of-sample band 15 are ONE
  position each; in-sample band 15 is zero. No recall figure in this document is
  evidence at n > 4.
- **NO ROW'S REGISTERED KILL CONDITION HAS FIRED.** Rows (a) and (e) are killed
  by §3 *"excluding a VALUE row of the recall fixture"*, and §5.6 shows the
  fixture cannot discriminate.

### 5.5 THE SAMPLE THIS ARC CANNOT WIDEN

`bench_solver_positions_v1.txt` is twenty late-game positions from the two
distinct sealbot-anchor games. There is no third anchor game, so **the only band
carrying a win denominator above one position cannot be drawn again**, and any
row fitted on it — `(m)` and `(m)+(n)+cache` included — is quotable in sample
only.

#### 5.5.1 The round-1 draw's own conditioning, acknowledged

The draw rule processes band 15 before band 35 with one carried used-set, so
round-1 band 15 draws from a pool round-0 band 35 has already taken from: **3 of
its 12 games are ones round-0 band 35 skipped**, i.e. games too short to reach
31 stones, where 0 of round-0 band 15's are. That is a plausible partial cause of
`T_off` falling 50,176.0 → 46,519.9 and of the required cut rising 22.50x →
24.27x. The registration's §2 sentence — *"the only thing that differs is which
twelve games each band got"* — is therefore not exactly true, and this is the
acknowledgement it is owed. **It does not rescue `opp_hot >= 3`**, whose fall is
in the numerator.

### 5.6 THE RECALL FIXTURE, RUN DIRECTLY AND THEN CENSUSED

`artifacts/stage3b_value_fixture_v1.txt` (the position itself, five caps) and
`artifacts/stage3c_census_value_fixture_v1.txt` (a full governed search from
each, `nodes 50000`, cap 2048).

| row | class | at the position: mover_hot / opp_hot / cover | fires at the position? | firings in a governed search | proofs found there |
|---|---|---|---|---|---|
| `g001-t44-p2` | VALUE | 0 / 0 / none | no | **0** | **0** |
| `g001-t46-p2` | VALUE | 2 / 2 / minimal(2) | yes | 1 | 1 — `win` at **1 visit** |
| `g002-t12-p2` | VALUE | 0 / 0 / none | no | 13 | 1, in-tree |
| `g002-t39-p1` | VALUE | 0 / 0 / none | no | **0** | **0** |
| `g002-t41-p1` | VALUE | 2 / 2 / minimal(2) | yes | 1 | 1 — `win` at **1 visit** |
| `g001-t42-p2` | CALL-RECALL-ONLY | 0 / 1 / minimal(2) | yes | 26 | 6, all defender-direction |
| `g002-t10-p2` | CALL-RECALL-ONLY | 0 / 0 / none | no | 13 | 1, in-tree |

**REVISION 3'S CLAIM WAS TOO STRONG AND IS NARROWED.** It said four of seven are
not trigger points; that is true **at the position** and false as a claim about
the gate, because **two** of those four — `g002-t12-p2` and `g002-t10-p2` — fire
in-tree and find a proof there. The measured finding is smaller and sharper:

- **TWO of the five VALUE rows — `g001-t44-p2` and `g002-t39-p1` — produce ZERO
  firings in an entire 50,000-node governed search.** The trigger never fires
  anywhere below them, so there is no trigger point to rank at any depth and no
  detector changes anything about them. On those two the gate is unevaluable,
  full stop.
- **The two VALUE rows that do fire at the position prove in ONE VISIT**, both
  because the mover is hot — the rules theorem — so any predicate admitting
  `mover_hot > 0` passes them for one visit. **The cap criterion is therefore
  satisfied at every cap the ladder measured, down to 512**; that it would hold
  lower is arithmetic and not measurement (m-5).
- **`g001-t42-p2`, the M4 flip, is the one row that constrains a cap: `win` at
  10,726 visits, reached only at cap ≥ 16384.** At 4096 both directions return
  `unknown` after spending 8,192 visits. D-512 gates it on ranking and never on
  proving, so it does not bind — but a governed search from it at cap 2048 finds
  **six proofs and every one is a proven LOSS**, which is §4.2's tension in one
  position.

**SO THE FIXTURE'S DISCRIMINATING POWER IS TWO ONE-VISIT ROWS.** The premise
memo's §5 finding 3 objected that *"a gate over seven hot positions cannot tell
whether the surviving fraction is the right fraction"*; measured, they are not
seven hot positions, and the gate is thinner than the objection assumed.

**HOW THE PRECONDITION FINDING AROSE.** Probing `g002-t12-p2` at cap 512
panicked with `SOLVER_NO_PLAN`. Not an engine defect and unreachable by the
engine: `solve_defender`'s root is an AND node needing the opponent hot, and the
trigger guarantees exactly that disjunction. Now documented and pinned by test
with a five-stone reproducer (D-525).

### 5.8 ROW (e), THE ALLOCATOR — scored separately, because it is not a predicate

`artifacts/stage3c_allocator_bound_v1.txt`. **An allocator carries a budget and
stops when it is spent, so it REACHES THE BUDGET BY CONSTRUCTION on every band,
in sample and out.** Its entire question is recall, and ranking it on the same
table as the predicates would report a budget verdict meaning two different
things. This is the form the dispatch's ruling 1 selected (D-516), and neither
earlier revision scored it.

**Two frames, and they are not the same number.**

| band | wins | **per-search ceiling** | **aggregate oracle** | **bound over the columns** |
|---|---|---|---|---|
| band 35 | 1 | 1.000 | 1.000 | 1.000 |
| trigger-rich | 14 | **0.571** (8 of 14) | 1.000 | **0.857** (12 of 14) |
| OOS band 15 | 3 | **0.333** (1 of 3) | 1.000 | 1.000 |
| OOS band 35 | 1 | 1.000 | 1.000 | 1.000 |

- **PER-SEARCH** is the form the dispatch names. It is strictly harder because
  the wins are not spread evenly: one trigger-rich search holds six proofs
  costing more than its own 1,799-visit budget, and one out-of-sample band-15
  search holds three costing 6,014 against 2,024. **No score beats those
  ceilings**, because the constraint is arithmetic and not informational.
- **AGGREGATE** is what the bracket itself fixes — an nps ratio over a whole
  bench. There the oracle keeps every win on every band, spending **6,218 of
  35,988** on trigger-rich and **6,014 of 24,295** out of sample. **The budget
  affords the proofs comfortably** — 17 % and 25 % of it.
- **THE BOUND over the census columns** is what a score fitted with full
  knowledge of which column-classes hold wins could reach: **0.857** on
  trigger-rich (12 of 14) and **1.000** out of sample. It is an upper bound, it
  is IN-SAMPLE by construction, and its arithmetic is exact — a scoped
  re-review re-solved the knapsack unscaled and agreed on every band.

**WHAT THIS SETTLES, AND IT IS THE MOST IMPORTANT THING IN §5.** The obstacle is
**not** that the columns cannot separate proofs from non-proofs. Measured
orderings over those columns reach **0.571** on trigger-rich and **0.333** out
of sample, against a bound of 0.857 and 1.000 — so a real gap exists between
what a fitted score could do and what any ordering this arc could WRITE does.
**Closing it means fitting a score and validating it out of sample, and the
evidence to do that does not exist**: the win denominators are 0, 1, 14 and 3,
over 0, 1, 4 and 1 distinct positions, and the only one worth fitting on has no
second draw.

**AND THE CLAIM IS FRAME-DEPENDENT, WHICH A FIRST DRAFT OF THIS SECTION DID NOT
SAY.** *"The barrier is sample size, not information"* is a statement in the
AGGREGATE frame. In the PER-SEARCH frame — the one the dispatch's own ruling 1
names — the ceiling on trigger-rich is 0.571 and the best measured ordering
already reaches 0.571, so **there the gap is ZERO and the barrier is arithmetic,
not sample size at all.** Both frames are stated because the two answers differ
and a design has to pick a frame before it can know which obstacle it faces.

## 5.9 The rows the field was missing (added at revision 2, extended at revision 4)

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
asymmetric on the corrected seat: **0 of band 15's 8 proofs, 2 of band 35's 2,
4 of trigger-rich's 28**, at **446 / 1150 / 1456** visits per search against
budgets of **2184 / 938 / 1799**. On band 35 the root alone exceeds the whole
budget while carrying every proof that band has.

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

### (n) Gate the DIRECTION, not the firing — ask the attacker only

Added at revision 4 by the round-2 DECISION-RED-TEAM under D-511's flip clause,
in this section's own format. **§1 declares the INVOCATION a unit of this
matrix's axis and no row (a)–(m) touches it**; this is that row.

**Mechanism.** No new predicate and no per-node test. In
`crates/pistol-search/src/pvs.rs`'s `solver_verdict`, the defender invocation
becomes conditional on a new closed config token, with the matching root call in
`search.rs`. The attacker invocation and its early return are untouched.

**What it can rank out.** Every proven LOSS, and **not one WIN**: `att_proved`
is decided before `solve_defender` is reached, so its win recall is 1.000 by
construction on every band — the only row besides (h) of which that is true.

**Cost shape.** Zero per node and zero per firing. It REMOVES work and adds one
branch on a `Copy` config field. **The only row in the field whose `t` is
negative.**

**MEASURED**: the defender direction is **57.5 %–67.4 %** of every band's solver
visits (§5.2). Alone it cuts 2.35x–3.07x, short of every budget. Composed as
`(m) + (n) + cache` it reaches **21.42x on trigger-rich against a required
15.29x with 12 of 12 distinct wins** — and keeps nothing at all on either
band-15 draw, because `(m)` is vacuous there.

**Kill condition.** The recall gate is widened to both directions (§7.1), at
which point this row gives up 8 / 1 / 14 / 0 / 1 proofs and its loss-side recall
is 0.000 by construction. **This row and §7.1's ruling are the same decision**,
which is the reason it belongs in the field rather than outside it. Secondarily,
a proven loss currently seeds `root_restrict` from the proof's zone, so the
root's defender call is not purely a value lookup and a design must price what
dropping that restriction does — no census column answers it.

---

## 6. RECOMMENDATION

REGISTERED SLOT. **Nothing is selected**, and the reason is the same after
revision 4's corrections as before them, which is the one thing about revision 3
the round-2 red team reported it could not break:

> After applying all twelve [remedies], the field still contains no selectable
> row, and D-471's kill point still fires. That is the one thing about
> revision 3 I could not break.
> — `artifacts/matrix_stage3_DECISION_REDTEAM_round2.md`

**The registered selection rule has two readings and the verdict is the same
under both, so this document does not pick one.** The registration's rule 1
hardcodes the in-sample thresholds (22.5x / 42.6x / 17.4x) in one sentence and
says the out-of-sample table *"carries whatever it gives there"* in another. The
corrected table is evaluated against both below; no cell changes side.

| conjunct | where the field stands |
|---|---|
| **1. BUDGET, out of sample, both corpus bands** | The best composition reaching OOS band 15 is `opp_hot>=3 + cache + cap 512` at 39.34x; on OOS band 35 it answers **40.6x against 43.20x** (registered reading: against 42.6x — still out). Nothing else reaches both. |
| **2. RECALL, census** | That composition keeps **0 of 3** OOS band-15 wins and **0 of 12** trigger-rich. `(m)+(n)+cache` keeps 12 of 12 on trigger-rich — and **0 of 3** out of sample, by keeping nothing at all. |
| **3. RECALL, fixture** | Passes vacuously: two of five VALUE rows never fire in a governed search, and the two that fire prove in one visit (§5.6). |
| **4. NOT VACUOUS** | Every row reaching a budget on either band-15 draw does so by admitting nothing, except the one that fails conjunct 1. |

**Conjuncts 1 and 2 fail together, out of sample, for every row and every
composition in the field.** By the registration's own §5, that is the kill
point.

**BUT THE KILL POINT AND D-471's CLAUSE ARE NOT THE SAME SENTENCE, AND §7 IS
WHERE THAT MATTERS.** D-471 flips the roadmap when *"the detector cannot reach
the bracket at its registered kill point"*. Three measured facts sit between
this table and that clause, and none of them is this session's to weigh:

1. **An allocator reaches the bracket by construction** (§5.8). What no row
   reaches is the RECALL bar, and the recall bar's own instrument is measured
   thin (§5.6) and its direction is contested (§4.2).
2. **No row's OPERATOR-registered kill condition has fired.** Rows (a) and (e)
   are killed by *"excluding a VALUE row of the recall fixture"*, and §5.6 shows
   the fixture cannot discriminate. What fired is the rule this session
   registered, which is a stricter and newer instrument.
3. **In the AGGREGATE frame the barrier is sample size, not information**
   (§5.8): a score over the census columns could reach 0.857 / 1.000 against
   the 0.571 / 0.333 any ordering here achieves, and the reason none was fitted
   is that the win denominators are 0, 1, 14 and 3 over 0, 1, 4 and 1 distinct
   positions. **In the PER-SEARCH frame the dispatch names, the gap is zero on
   trigger-rich and the barrier is arithmetic instead.** Which frame a design
   works in is not settled here.

## 7. WHAT IS OWED — three rulings, and the first one can change the verdict

1. **THE CENSUS GATE'S DIRECTION.** §4.2: the fixture is win-direction, but
   D-512 names D-510's `proofs` counter for the census denominator and that
   counter reads both directions. Under both directions `opp_hot >= 3` keeps
   1.000 of band 15's in-sample proofs inside the budget, and row (n) — the
   largest lever measured in this arc — goes from *free* to *forfeiting half of
   what the solver finds*. **Row (n) and this ruling are the same decision.**
   The session that would benefit from narrowing the gate may not take it.
2. **THE FIXTURE.** §5.6: two of five VALUE rows produce zero firings in an
   entire governed search, and the two that fire prove in one visit. Either the
   fixture is restated as what it measurably is — a two-row, one-visit gate —
   or it is re-specified to ask something a non-firing position can answer.
   D-512's flip clause covers a VALUE row moving to CALL-RECALL-ONLY; it does
   not cover a row that never fires.
3. **THE TRIGGER-RICH SAMPLE.** §5.5: the only band with a win denominator above
   one position cannot be drawn again at HEAD. Either a third source of
   trigger-rich positions is commissioned — its own work package — or no row
   fitted there is selectable under any out-of-sample rule.

**WHAT WOULD HAVE TO BE TRUE FOR THE FIELD TO SURVIVE AS IT STANDS.** A row
reaching **24.27x and 43.20x out of sample** while keeping every win. §5.3
contains none. But §5.8 measures that the columns are not the obstacle — a
fitted score reaches 0.857 on trigger-rich and 1.000 out of sample — so the
honest closing statement of this matrix is not *"no detector can reach the
bracket"*. It is:

> **The budget affords the proofs — 17 % and 25 % of it buys every win. Under an
> AGGREGATE budget the columns can find them, to 0.857 and 1.000, and no
> ordering this arc wrote gets past 0.571. What is missing is enough proofs to
> fit a score on and a second draw to check it against** — 14 win-proving
> firings over 4 positions on the one fixture that cannot be drawn twice, and 3
> over 1 position on the one that can. **Under the PER-SEARCH budget the
> dispatch names, that is not the obstacle: there the ceiling is 0.571, a
> written ordering already reaches it, and the limit is how the proofs are
> distributed across searches rather than how few of them there are.**

That is a finding about the EVIDENCE and about the FRAME, and the remedies it
points at are a position corpus and a ruling on the frame — not an abandoned
package. Whether D-471's clause reads any of it as *"cannot reach the bracket"*
is ruling 1's neighbour and is the operator's.
