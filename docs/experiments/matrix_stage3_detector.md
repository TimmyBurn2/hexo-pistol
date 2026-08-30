# Stage-3 detector — OPTION MATRIX

**Status: §1–§4 are written BEFORE the census run that fills §5.** §4 registers
how each row's numbers are computed and what would kill each row, so the
ranking cannot be an argument constructed after the numbers. Nothing is
selected here: selection follows a fresh-context DECISION-RED-TEAM on this
document, and the selection record quotes its rows (CLAUDE.md, Process).

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

## 4. HOW THE ROWS ARE RANKED — registered before the census runs

**The instrument**: `crates/pistol-search/examples/trigger_census.rs`, at the
revision this document is committed at. It prints one row per firing with the
O(1) columns a per-node detector could read there (`turns`, `mover_hot`,
`opp_hot`, `mover_w1`, `opp_w1`, `mover_l3`, `opp_l3`) and what the solver then
answered (`att_visits`, `att_proved`, `def_asked`, `def_visits`, `def_proved`).
A change to it reopens this section (`docs/process.md`).

**The workload**: both committed bench fixtures at `nodes 50000` — the corpus
fixture and the trigger-rich one — at the ON seat's committed cap.

**What each row's number is, stated as a formula over the census so that no row
can be scored on a quantity chosen for it afterwards:**

- **KEPT** = firings a row's predicate admits, ÷ all firings.
- **PROOFS KEPT** = firings the row admits that returned a proof, ÷ all firings
  that returned a proof. **This is the recall the gate is about.**
- **PRECISION** = proofs kept ÷ firings kept. The dispatch's economics: *"the
  expensive calls are the ones that return NoWin"*.
- **REACHES BUDGET** = whether KEPT × 12.00 (band 15's measured firings per
  search) is at or below the firing budget.

**Rows (a), (b) and (e) are scored by evaluating their predicate over the census
columns.** Row (c) is scored differently and the difference is registered here:
it is a modifier, so its number is the visit cost `12 × p` against the visit
budget, not a KEPT fraction. Row (d) is scored as the composition it is: the
KEPT of its cheap tier and the visit cost of its certifier. Row (f) has no
number by construction.

**A row that cannot be scored from the census is recorded as UNSCORED and its
reason given** — not scored on something else. An unscored row survives to the
red team, which is where a field's completeness is judged.

**THE ONE THING THE CENSUS CANNOT ANSWER, said before it is run.** It records
what the CURRENT trigger's firings look like. A detector that gates firings
changes the search, which changes which nodes exist, which changes the firing
set — the fixed-point the premise memo's §3.6 named as a stated assumption. So
KEPT is a fraction of TODAY's firings, and its use is to RANK rows against each
other, never to predict the post-detector firing count. Any row's own bench is
what measures that.

---

## 5. THE MEASURED RANKING

Taken under §4's registered formulas, from
`artifacts/stage3_census_analysis_v1.txt` — the census over both committed bench
fixtures at `nodes 50000`, cap 2048, quiet_radius 2.

### 5.1 What the incumbent trigger actually looks like

| | band 15 | band 35 | trigger-rich |
|---|---|---|---|
| firings per search | 18.33 | 11.75 | 9.05 |
| invocations per firing | 2.000 | 1.993 | 1.939 |
| **PRECISION at the incumbent** | **3.64 %** | **1.42 %** | **13.81 %** |
| proving firings / all firings | 8 / 220 | 2 / 141 | 25 / 181 |
| root fires in | 3 / 12 searches | 5 / 12 | 12 / 20 |

**THE STRUCTURAL FACT THAT SHAPES THE WHOLE FIELD, and it was not predicted by
anything before the census: the trigger fires because the OPPONENT is hot, not
because the mover is.** On band 15 the predicate *mover hot* keeps **0.000** of
the firings and *opponent hot* keeps **1.000**; on band 35, 0.007 and 0.993; on
trigger-rich, 0.011 and 1.000. The staged generator is threat-first, so the
search spends its time at positions where the side to move is under threat, and
the incumbent trigger is — in practice and not by intention — an
opponent-is-hot detector.

### 5.2 The rows, scored

**KEPT** = firings a predicate admits ÷ all firings. **PROOFS KEPT** = proving
firings it admits ÷ all proving firings. **PRECISION** = proofs kept ÷ firings
kept. A row REACHES BUDGET when `KEPT × firings-per-search` ≤ the firing budget,
i.e. when `KEPT ≤ 0.046 / 0.024 / 0.066` on the three bands.

| candidate (row) | KEPT b15 | PROOFS b15 | KEPT b35 | PROOFS b35 | KEPT trig | PROOFS trig |
|---|---|---|---|---|---|---|
| incumbent | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| **(a)** both sides hot | 0.000 | 0.000 | 0.000 | 0.000 | 0.011 | 0.080 |
| **(a)** mover hot | 0.000 | 0.000 | 0.007 | 0.500 | 0.011 | 0.080 |
| **(a)** opponent hot | 1.000 | 1.000 | 0.993 | 0.500 | 1.000 | 1.000 |
| **(a)** a win-in-one-ply, either side | 0.045 | **0.000** | 0.078 | **0.000** | 0.072 | **0.000** |
| **(a)** mover has ≥ 2 hot windows | 0.000 | 0.000 | 0.000 | 0.000 | 0.011 | 0.080 |
| **(a)** either side has ≥ 2 hot windows | 0.227 | **1.000** | 0.404 | 0.500 | 0.569 | 0.760 |
| **(a)** mover hot and ≥ 2 live threes | 0.000 | 0.000 | 0.000 | 0.000 | 0.011 | 0.080 |
| **(b)** must-block shape (opponent hot, mover not) | 1.000 | 1.000 | 0.993 | 0.500 | 0.989 | 0.920 |
| **(e)** root only | 0.014 | 0.000 | 0.035 | 1.000 | 0.066 | 0.200 |
| **(e)** turns ≤ 1 | 0.677 | 1.000 | 0.979 | 1.000 | 0.762 | 0.640 |

### 5.3 What the numbers do to each row

**(a) — the best single predicate cuts 4.4x and the target is 21.7x.** *Either
side has ≥ 2 hot windows* is the only tightening that keeps every band-15 proof
(PROOFS 1.000) while cutting firings, and it cuts them to 0.227 — a **4.4x**
reduction against a required **21.7x**, and on band 35 it keeps 0.404 against a
required 41.1x while dropping half the proofs. **Every predicate in the field
that reaches the budget drops every proof with it**: *win-in-one-ply* keeps
0.045 (band 15, inside budget) and **0.000 of the proofs**; *mover hot* keeps
0.000. **No single-predicate narrowing over these columns both reaches the
budget and keeps the proofs.** Row (a) alone is **KILLED by its registered kill
condition** on band 15 and band 35.

**(b) — measured a no-op.** The must-block shape keeps 0.989–1.000 of the
firings, because §5.1's structural fact makes almost every firing that shape
already. Row (b) buys **nothing** and would pay a non-O(1) per-node cost for it,
which its kill condition names. **KILLED, and killed by measurement rather than
by the cost argument that was expected to kill it.**

**(c) — the probe budget is spent before it starts.** A probe costing `p` visits
at every firing spends `18.33 × p` on band 15 against a visit budget of 2183.6,
so `p` must be under **119 visits** to leave anything for a real call — and a
call is measured at `K` = 1339 visits. A probe under a tenth of a real call's
cost is not a bounded VCDT probe, it is a guess. **KILLED on band 15 by its own
registered kill condition** unless firings are cut first, which makes it what §3
already said it is: a modifier, not a rival.

**(d) — inherits (a)'s or (b)'s kill.** Both cheap tiers available in the field
are killed above, and (d) is their composition with (c). It survives only if a
tier not yet in the field is added — which is what "at least" in the dispatch's
own enumeration leaves open, and what a red team may do before selection.

**(e) — the only row that can reach an arbitrary target, because it CAPS rather
than FILTERS.** A predicate's KEPT is whatever the position mix gives it; an
allocator's is whatever the budget says. The census bears on it in two ways and
both are stated:

- **The columns carry real signal.** *Either side has ≥ 2 hot windows* lifts
  precision from 3.64 % to **16.00 %** on band 15 (4.4x) and from 13.81 % to
  18.45 % on trigger-rich, and *root only* lifts it to **41.67 %** on
  trigger-rich and **40.00 %** on band 35. A score built from these columns
  ranks better than chance, which is what an allocator needs.
- **The signal is not enough on its own, and at a 21.7x cut it does not have
  to be.** An allocator admitting the top 4.6 % of firings by score keeps, at
  the incumbent's 3.64 % precision and a 4.4x-better ranking, an expected
  **0.16 × 0.046 × 220 ≈ 1.6** of band 15's 8 proving firings. **That is a
  recall of about 20 %, and it is the number the recall gate (D-512) will
  adjudicate.**

**(f) — the null is not yet ruled out and the field is honest about that.** Rows
(a) and (b) are killed by measurement; (c) and (d) depend on a row that survives;
(e) is the only row that can reach the target and the census puts its expected
recall near 20 %. **If the recall gate rejects that, (f) wins and D-471's
roadmap clause fires.**

### 5.4 What this section does NOT establish

- **KEPT is a fraction of TODAY's firings.** §4 registered this before the run:
  a detector that gates firings changes the search, which changes the firing
  set. The ranking is a comparison between rows, never a prediction of the
  post-detector count.
- **The recall estimate for (e) is an ESTIMATE**, marked so (D-291): it composes
  a measured precision lift with a budget share, and no allocator has been
  built to check it. The recall GATE is what measures it.
- **The census is one workload at one budget and one cap.** Both committed bench
  fixtures, `nodes 50000`, cap 2048 — the seats the bracket was registered
  against, and no others.

---

## 6. RECOMMENDATION

REGISTERED SLOT — written after a fresh-context DECISION-RED-TEAM has attacked
§5, and quoting its rows. **Nothing is selected until then**, and the shape of
§5.3 is why: four of the six rows are killed by measurements taken after this
document's own §1–§4 were written, which is exactly the position in which a
session most wants to select and least should.
