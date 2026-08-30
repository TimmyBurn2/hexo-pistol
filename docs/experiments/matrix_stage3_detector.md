# Stage-3 detector — OPTION MATRIX

**REVISION 2**, after a fresh-context DECISION-RED-TEAM returned **FALLS** on
revision 1 (`1bc2788`) — 5 BLOCKING, 9 MAJOR, 5 MINOR. It reproduced revision 1's
§5.1 and §5.2 exactly, including an externally derived integrity check the
document had not thought to make (`sum(row visits) == sum(solver_nodes)`, byte-
exact on all three bands). What fell was §5.3's readings and §3's field, and the
two headline findings are that **the REACHES-BUDGET test was in the wrong unit**
and that **the field omitted the mechanisms the census ranks best**. §4's
formulas are AMENDED here, which reopens this document's review (`docs/process.md`).

**Nothing is selected.** Selection follows a fresh-context DECISION-RED-TEAM on
THIS revision, and the selection record quotes its rows (CLAUDE.md, Process).

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
  reduction is **21.7x / 41.1x / 15.1x** (MEASURED, D-517);
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

## 4. HOW THE ROWS ARE RANKED — AMENDED, and the amendment is the point

**The instruments**: `crates/pistol-search/examples/trigger_census.rs` records
one row per firing with the O(1) columns a per-node detector could read there
(`turns`, `mover_hot`, `opp_hot`, `mover_w1`, `opp_w1`, `mover_l3`, `opp_l3`)
and what the solver then answered (`att_visits`, `att_proved`, `def_asked`,
`def_visits`, `def_proved`). `tools/stage3_census_analyse.py` reports the
census's own quantities; **`tools/stage3_census_rank.py` applies the test
below**. A change to any of them reopens this section.

**The workload**: both committed bench fixtures at `nodes 50000`, cap 2048,
`quiet_radius 2` — the seats the bracket was registered against.

### 4.1 THE TEST IS IN VISITS, and revision 1's was not

- **KEPT** = firings a row's predicate admits ÷ all firings.
- **PROOFS KEPT** = proving firings it admits ÷ all proving firings.
- **PRECISION** = proofs kept ÷ firings kept.
- **REACHES BUDGET** = `sum(att_visits + def_visits over KEPT firings) / searches`
  ≤ the band's visit budget (2183.6 / 937.6 / 1799.4 at `T_off`).

**REVISION 1 PRICED A ROW IN FIRINGS AT THE POPULATION MEAN, AND THAT IS THE
WRONG UNIT.** §1 of this document says of the visit: *"what the BUDGET is
denominated in, and the only unit the bracket fixes directly"*. Converting a
visit budget into a firing budget divides by the mean cost of a firing, which is
valid only if a row's kept firings cost the mean — and the census records what
they actually cost. **The two tests disagree on exactly the predicate that
reaches the bracket**: `opp_hot >= 3` keeps firings costing **1085 visits each**
against a population mean of 2678, so the firing test rejects it (1.083 against a
budget of 0.845) and the visit test accepts it (1175.8 against 2183.6). **The
matrix's own registered instrument would have rejected the one row that works.**
That is D-477's class committed by the document whose §1 is a D-477 compliance
section, and it is the same substitution D-508 recorded one level up.

The firing figure is still reported. It is no longer the test.

### 4.2 What is scored, and what is UNSCORED

Rows (a), (g), (j) are predicates over the census columns and are scored by the
test above. Row (i) is scored by re-charging every invocation at `min(visits,
cap)` and counting which proofs remain reachable. Row (h) is scored by
collapsing repeated firing SIGNATURES, and its figure is an **upper bound** for
a reason stated where it is printed. Row (f) has no number by construction.

**Rows (b), (c), (d) and (k) are UNSCORED, and revision 1 scored two of them
anyway.** §4 registered the disposition — *"A row that cannot be scored from the
census is recorded as UNSCORED and its reason given — not scored on something
else"* — and it is applied here rather than ignored:

- **(b)** is an UNANSWERABILITY test over `blocking_covers`. **The census has no
  cover column**, so revision 1 scored `opp_hot > 0 and mover_hot == 0` and
  killed row (b) on a predicate that is not row (b). **Its KILLED verdict is
  withdrawn.** Scoring it needs a cover column, which is one more census run.
- **(c)** is a modifier on a probe cost the census does not contain.
- **(d)** is a composition of rows, and §5.3 scores the compositions directly.
- **(k)** needs parent-child identity, which the rows do not carry.

### 4.3 THE ONE THING THE CENSUS CANNOT ANSWER, unchanged

KEPT is a fraction of TODAY's firings. A detector that gates firings changes the
search, which changes the firing set — the fixed point the premise memo's §3.6
named. The ranking compares rows; it never predicts the post-detector count.

---

## 5. THE MEASURED RANKING

`artifacts/stage3_census_rank_v1.txt` (the visit test) and
`artifacts/stage3_census_analysis_v1.txt` (the census's own quantities).

### 5.1 What the incumbent trigger looks like

| | band 15 | band 35 | trigger-rich |
|---|---|---|---|
| firings per search | 18.33 | 11.75 | 9.05 |
| **visits per search** | **49,100.5** | **39,917.1** | **31,237.1** |
| **visit budget** | **2183.6** | **937.6** | **1799.4** |
| **required cut** | **22.5x** | **42.6x** | **17.4x** |
| invocations per firing | 2.000 | 1.993 | 1.939 |
| PRECISION | 3.64 % | 1.42 % | 13.81 % |
| proving firings | 8 / 220 | 2 / 141 | 25 / 181 |
| **distinct proving positions** | **5** | **1** | **13** |
| root fires in | 3 / 12 searches | 5 / 12 | 12 / 20 |

**THE STRUCTURAL FACT, and its mechanism corrected.** Revision 1 observed that
`mover hot` keeps essentially nothing and attributed it to the staged
generator's threat-first ordering. The red team showed the stronger reason: at a
firing the mover is at `Phase::First` with two stones owed, and *hot* means a
live window holding four or more own stones — **so `mover_hot > 0` at a firing
means the mover completes six THIS TURN**, a theorem of game rules 2, 3 and 5
rather than a property of the ordering. All three mover-hot rows in the entire
census are one-visit root wins. **Four of revision 1's scored rows were empty by
the RULES, not by measurement**, and a detector that changed the ordering would
not change that.

### 5.2 The rows, scored in visits

Per band: KEPT / PROOFS KEPT / visits-per-search / cut / budget verdict.

| row | band 15 | band 35 | trigger-rich |
|---|---|---|---|
| incumbent | 1.000 / 1.000 / 49100 / 1.0x / out | 1.000 / 1.000 / 39917 / 1.0x / out | 1.000 / 1.000 / 31237 / 1.0x / out |
| **(a/g) `opp_hot >= 2`** | 0.227 / **1.000** / 8546 / 5.8x / out | 0.404 / 0.500 / 15895 / 2.5x / out | 0.569 / 0.760 / 16451 / 1.9x / out |
| **(a/g) `opp_hot >= 3`** | 0.059 / **1.000** / **1176** / **41.8x** / **IN** | 0.057 / 0.500 / 2185 / 18.3x / out | 0.204 / 0.560 / 5557 / 5.6x / out |
| **(a/g) `opp_hot >= 4`** | 0.000 / 0.000 / 0 / — / IN (vacuous) | 0.007 / 0.500 / 126 / 317x / IN | 0.011 / 0.080 / 117 / 266x / IN |
| (a) either side ≥ 2 hot | 0.227 / 1.000 / 8546 / 5.8x / out | 0.404 / 0.500 / 15895 / 2.5x / out | 0.569 / 0.760 / 16451 / 1.9x / out |
| (a) a win-in-one-ply | 0.045 / **0.000** / 1803 / 27.2x / IN | 0.078 / 0.000 / 3053 / 13.1x / out | 0.072 / 0.000 / 2736 / 11.4x / out |
| (a) mover hot | 0.000 / 0.000 / 0 / — / IN (theorem, §5.1) | 0.007 / 0.500 / 0.1 / — / IN | 0.011 / 0.080 / 0.1 / — / IN |
| **(j) not the root** | 0.986 / 1.000 / 48655 / 1.0x / out | 0.965 / **0.000** / 38767 / 1.0x / out | 0.934 / 0.800 / 29620 / 1.1x / out |
| **(j) the root only** | 0.014 / **0.000** / 446 / 110x / IN | 0.035 / **1.000** / 1150 / 34.7x / out | 0.066 / 0.200 / 1617 / 19.3x / IN |

**(i) THE CAP**, every invocation re-charged at `min(visits, cap)`, with the
proofs still reachable:

| cap | band 15 | band 35 | trigger-rich |
|---|---|---|---|
| 2048 (incumbent) | 49100 / 8 of 8 | 39917 / 2 of 2 | 31237 / 25 of 25 |
| 1024 | 25828 / 7 of 8 | 20165 / 2 of 2 | 15949 / 23 of 25 |
| 512 | 13451 / 5 of 8 | 10217 / 2 of 2 | 8290 / 21 of 25 |
| 256 | 7165 / 1 of 8 | 5225 / 2 of 2 | 4291 / 8 of 25 |

Nothing reaches a budget on the cap alone. What the sweep says is that
**60–82 % of every invocation's visits buy nothing**: every band-15 proof is
found within 1039 visits against a cap of 2048.

**(h) THE VERDICT CACHE**, repeated firing signatures collapsed — **recall
1.000 by construction**, and an UPPER bound because a signature is evidence of a
repeated position, not proof of one:

| | band 15 | band 35 | trigger-rich |
|---|---|---|---|
| distinct signatures | 94 of 220 | 68 of 141 | 102 of 181 |
| firings per signature | 2.34 | 2.07 | 1.77 |
| visits/search if a repeat is free | 19,977 | 20,061 | 16,014 |
| cut | 2.46x | 1.99x | 1.95x |

### 5.3 THE COMPOSITIONS, which is where the field actually stands

| composition | band 15 | band 35 | trigger-rich |
|---|---|---|---|
| **`opp_hot >= 3` + cache** | **923 / 53.2x / 5 of 5 / IN** | 1844 / 21.7x / 1 of 1 / out | 4008 / 7.8x / 13 of 13 / out |
| cache + cap 512 | 5727 / 8.6x / 3 of 5 / out | 5113 / 7.8x / 2 of 2 / out | 4398 / 7.1x / 20 of 23 / out |
| **`opp_hot >= 3` + cache + cap 512** | **439 / 111.9x / 3 of 5 / IN** | **481 / 83.0x / 1 of 1 / IN** | **1151 / 27.1x / 12 of 13 / IN** |

**`opp_hot >= 3` + the cache preserves EVERY distinct proof on EVERY band and
reaches the bracket on band 15 alone**, missing band 35 by 2.0x and trigger-rich
by 2.2x. Adding cap 512 reaches all three and costs recall — 3 of 5 on band 15.

### 5.4 What §5.3 does and does not license

- **Revision 1's central sentence is WITHDRAWN.** It read *"no single-predicate
  narrowing both reaches the budget and keeps the proofs"*; `opp_hot >= 3` does
  exactly that on band 15, from one comparison, at 41.8x against a required
  22.5x. Row (a)'s KILLED verdict is withdrawn with it: the mechanism §3(a)
  offers — *"fire only above a count of hot windows"* — was tested at one value.
- **Row (b)'s KILLED verdict is WITHDRAWN**; it was scored on a predicate that
  is not its mechanism, and it is UNSCORED (§4.2).
- **THE THRESHOLD 3 WAS CHOSEN AFTER SEEING THE PROOFS**, exactly as revision 1
  chose 2. All eight band-15 proofs sit at `opp_hot == 3`, which is that band's
  maximum; under a null of random placement, eight landing in a nominated 13-of-
  220 subset has probability about 3e-10, so it is not chance — but it is
  in-sample and the threshold may be reading *the busiest positions* rather than
  a proof signal. **That is an argument for scoring the row, not for selecting
  it.**
- **THE DENOMINATORS ARE SMALL AND THE DOCUMENT SAYS SO.** Band 15 has 8 proving
  firings over **5 distinct positions**; band 35 has **2 firings over 1
  position**, both one-visit root proofs. A PROOFS-KEPT of 0.500 on band 35 is
  one proof of two firings of one position. No verdict on that band's rows is a
  measurement of anything, and revision 1 stated no sampling error at all.
- **BAND 15's ENTIRE RECALL DENOMINATOR IS DEFENDER-DIRECTION PROOFS — PROVEN
  LOSSES.** D-512's recall fixture pins WINS. The census's recall and the gate's
  recall are therefore not the same quantity, and §5's ranking does not
  discharge the gate.
- **THE REGISTERED KILL CONDITIONS WERE NOT WHAT REVISION 1 APPLIED.** Rows (a)
  and (e) are killed by §3 *"excluding a VALUE row of the recall fixture"*, which
  is seven named anchor positions the census does not contain. **No row's
  registered kill condition has fired.** What §5 establishes is a RANKING; the
  kill conditions are the recall gate's and the bench's.

---

## 5.5 The rows the field was missing

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

Revision 1 reached its §5.3 verdicts and a red team took four of them apart; the
lesson is not that this revision's readings are better but that a session
holding fresh numbers is exactly where selection is least safe. What this
revision may say is what it measured: **a recall-preserving composition reaches
the bracket on one band of three, and reaches all three at a cost in recall the
gate has not adjudicated.** Whether that is a package worth designing is the
next red team's to attack and the operator's to decide.
