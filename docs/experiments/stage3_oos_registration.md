# Stage-3 detector — the out-of-sample validation and the value-fixture check

**REGISTERED BEFORE EITHER RUN.** This document fixes the draw rule, the
instruments, the recall definition and the reading of every cell, so that the
numbers the option matrix's revision 3 quotes were constrained before they
existed. It governs two runs and nothing else; the matrix's own in-sample table
is not amended by it and is relabelled rather than replaced.

**Why it exists.** Revision 2 of `docs/experiments/matrix_stage3_detector.md`
states three honesty notes about its own §5 — that the threshold 3 was chosen
after seeing the proofs, that the band-35 denominators are one position, and
that the census's recall and D-512's gate are *"not the same quantity"*. A
document that names the objection to its own headline row and selects on it
anyway has answered nothing. These runs answer the first and third; the second
is a sample width, which no run repairs and which §5 requires be carried on the
face of the row instead.

---

## 1. THE RECALL DEFINITION. One, stated here, in force everywhere after it.

**RECALL IS THE ATTACKER DIRECTION. A row's recall is the fraction of
WIN-direction proofs it keeps, and nothing else is added into it.**

The two directions, quoted where a census row gets them
(`crates/pistol-search/src/pvs.rs`, `solver_verdict`):

| direction | the call | what a proof there means |
|---|---|---|
| **attacker** | `solver.solve(&state_view, cap)` | the SIDE TO MOVE forces a policy-game win. **This is a WIN.** |
| **defender** | `solver.solve_defender(&state_view, cap)` | the OPPONENT forces a win against the mover's best defence. **This is a proven LOSS for the mover.** |

D-512's fixture is stated in the first: its seven rows are positions on which
`wp18b_probe` — which asks `solve`, the attacker direction — returned `win`.
So the gate's denominator is WINS, and a census column that sums the two is not
that denominator.

**What this costs the standing table, stated here rather than discovered later.**
`tools/stage3_census_rank.py` revision 1 scored `att_proved or def_proved`. Under
the split the in-sample census reads:

| band | WIN-direction proving firings | LOSS-direction |
|---|---|---|
| CORPUS band 15 | **0** | 8 |
| CORPUS band 35 | **1** | 1 |
| TRIGGER-RICH | **11** | 14 |

**Band 15 has no win denominator at all**, so no row's recall is measurable
there and a row that reads 1.000 on that band is reading a quantity the gate
does not pin. This is not a finding of these runs — it is a property of the
census already on disk — and it is registered here so that the runs below are
read against a definition fixed before they were taken.

**The consequence for a row that keeps no wins is not softened by its losses.**
A row that ranks out every win-direction proof fails the recall gate however
many proven losses it keeps.

## 2. THE OUT-OF-SAMPLE DRAW. The rule, before the draw.

**The instrument**: `crates/pistol-cli/examples/stage3_oos_positions.rs`.

**The rule.** `corpus-extract` sorts every eligible corpus game by `game_hash`
ascending and, for each band in order, takes the first `PER_BAND = 12` games
that band can supply a position from, skipping any game an earlier band already
used. This continues that loop: **round `r` is the `r`-th consecutive sweep of
the same ordering, carrying ONE used-set across every band of every round.**

- **round 0** is therefore, by construction, `bench_positions_v1.txt`'s own 24
  positions — the IN-SAMPLE set, every number of the matrix's §5 measured on it;
- **round 1** is the next 12 games per band in the same order. It shares no game
  with round 0, so it shares no position with it.

**The corpus is the same one**, identified as `bench_positions_v1.txt` identifies
it and by nothing else: sha256
`b2fe61eb360b91d77873a751446d28287955cad49e331fc32c156b4e1316840c`, 8698 games,
0 excluded. A different digest makes the draw a different draw and the run VOID.

**CRITERION V — the instrument is checked against something outside itself
BEFORE the draw it governs is read.** Run at `--round 0`, the body lines this
instrument prints must equal `bench_positions_v1.txt`'s 24 body lines
**byte-for-byte** — same order, same `start moves` spelling, same `# src … stones
…` commentary. **If they differ in any byte the run is VOID and no round-1
number is quotable**: an instrument that cannot reproduce the draw everything
else was measured on is not drawing from the same population.

**What round 1 is NOT.** It is not a second workload class and not a harder one.
It is the same corpus, the same bands, the same widths, the same selection order
— the only thing that differs is which twelve games each band got, which is the
one thing that has to differ for the word *out-of-sample* to mean anything.

**The trigger-rich fixture has no out-of-sample twin and this document does not
invent one.** `bench_solver_positions_v1.txt` is twenty late-game positions from
the two anchor games; there is no third anchor game to draw from, so the
trigger-rich column of every table below stays IN-SAMPLE and is labelled so.
**A row's out-of-sample evidence is therefore two bands, not three**, and a
selection that needs the third is a selection this arc cannot make.

## 3. THE OUT-OF-SAMPLE CENSUS. Same instrument, same workload.

`crates/pistol-search/examples/trigger_census.rs` at `--nodes 50000 --cap 2048
--quiet-radius 2` — the seats the bracket was registered against
(`docs/experiments/stage3_rulings.md` §1.2), unchanged, because a workload change
would make the comparison a comparison of two things.

**The census gains one column in this arc** — `cover`, the class of
`ThreatState::blocking_covers(mover, HitBudget::from(stones_left))`, with the
count of inclusion-minimal covers beside it. It exists because the matrix's §4.2
records that row **(b)** is an UNANSWERABILITY test which the census could not
express, that revision 1 scored a substitute for it, and that its KILLED verdict
is withdrawn pending *"one more census run"*. This is that run. The column is
read only under a census (`self.census.is_some()`), so no shipped path pays for
it and no bench measures its cost — row (b)'s per-node cost stays what §3(b)
says it is, a bench obligation and not a thing this run has priced.

### 3.1 THE OUT-OF-SAMPLE BUDGET, and why it is not the in-sample one

A band's visit budget is `SHARE x T_off` — the bracket-intrinsic share of the
OFF seat's node total (`artifacts/stage3_call_budget_v3.txt`, quoted through
`docs/experiments/stage3_rulings.md` §1.2). **The SHARE is bracket-intrinsic and
does not move: 4.352 % / 2.242 % / 5.696 %.** `T_off` is a property of the
POSITIONS, and the in-sample bands do not share one — band 15's is 50,176 and
band 35's is 41,826, because a band-35 search can exhaust its tree before the
50,000-node budget. **Carrying an in-sample `T_off` onto out-of-sample positions
would be a borrowed denominator**, which is the substitution D-477 exists to
forbid, one level down from where D-508 found it.

So `T_off` is MEASURED on the drawn positions: `trigger_census --gate off` runs
the same seat with the solver unwired and prints each entry's `search_nodes`,
which is that position's `T_off`. The budget is then `SHARE x mean(T_off)` over
the band, and the required cut is the band's own incumbent visits-per-search
divided by it. The in-sample required cuts — **22.5x / 42.6x / 17.4x** — are
what this arithmetic gives on the in-sample draw, and the out-of-sample table
carries whatever it gives there.

**The in-sample census is RE-TAKEN with the new column**, because a candidate
scored on 220 rows from one build and 220 from another is not scored on one
census. **CRITERION R — the re-take must reproduce the standing artifact on
every column it shares with it**: same firing count per band, same `att_visits`,
`def_visits`, `att_proved`, `def_proved`, same window counts, in the same order.
**A difference outside the two new columns means the column changed the search
and the run is VOID**, which is a finding about the instrument and not about any
row.

## 4. THE VALUE-FIXTURE CHECK. The gate D-512 pins, per position.

**The instrument**: `crates/pistol-search/examples/value_fixture_recall.rs`.

**The rows**, from D-512 verbatim — five VALUE rows and two CALL-RECALL-ONLY:

| row | class | what the record says it costs |
|---|---|---|
| `g001-t44-p2` | VALUE | `win` at 86 visits |
| `g001-t46-p2` | VALUE | `win` at 1 |
| `g002-t12-p2` | VALUE | `win` at 3,904 — **at cap ≥ 4096** |
| `g002-t39-p1` | VALUE | `win` at 714 |
| `g002-t41-p1` | VALUE | `win` at 1 |
| `g001-t42-p2` | CALL-RECALL-ONLY | the M4 flip: `win` at 10,726 with NO cap |
| `g002-t10-p2` | CALL-RECALL-ONLY | a v0-policy proof no M4 cap reproduces |

The two CALL-RECALL-ONLY rows are gated on RANKING and never on proving
(D-512): a gate asking for a proof no deployed configuration produces ships red
on correct code.

**THE UNITS, RECONCILED EXPLICITLY, because this check is the one place the two
words meet.** A probe's `nodes` and a census's `visits` are the SAME quantity:
`SolveResult::nodes` is what `solver_verdict` adds to `self.solver_nodes` at both
call sites, and `self.solver_nodes` is the term `total_nodes = search_nodes +
solver_nodes` reads. The probe artifacts' `nodes` column and the census's
`att_visits`/`def_visits` columns are therefore directly comparable, and the
figures in the table above are visits.

**CRITERION C — THE CAP.** A candidate stack that proposes a per-call cap `C`
**FAILS the recall gate** if any VALUE row's proving invocation spends more than
`C` visits. This is checked per position and it is checked regardless of what
the census says: census recall is a fraction over a workload, and a gate D-512
pins per position is not discharged by a fraction.

**CRITERION A — THE RANKING.** A candidate predicate **FAILS the recall gate**
if it does not admit the firing at a VALUE row's position. Admission is
evaluated on the columns the instrument prints for that position, which are the
columns a detector would read there.

**What a pass does NOT establish**, said here so no reading takes it for more:
seven positions are seven positions. Criterion A passing means the rows are
admitted, not that the surviving fraction is the right fraction — the objection
the premise memo's §5 finding 3 raised against this fixture and which the census
denominator, not this check, is what answers.

## 5. THE SELECTION RULE, registered before the table exists.

A row or composition is **SELECTABLE** only if all four hold:

1. **BUDGET.** On the OUT-OF-SAMPLE table it reaches the visit budget on both
   corpus bands — a cut of at least **22.5x** (band 15) and **42.6x** (band 35)
   against that table's own incumbent — and on the in-sample trigger-rich
   fixture a cut of at least **17.4x**.
2. **RECALL, census.** Its WIN recall is **1.000** on every band that has a
   win-direction denominator. A band with no win denominator scores no recall
   and licenses none.
3. **RECALL, fixture.** It passes Criterion A and Criterion C above, per
   position.
4. **NOT VACUOUS.** It admits at least one firing on each band. A predicate that
   keeps nothing reaches every budget and is row (f) wearing a mechanism.

**IF NO ROW IS SELECTABLE, THAT IS THE KILL POINT.** D-471's clause — *"flips to
Stage 2 immediately if the detector cannot reach the bracket at its registered
kill point"* — fires, the flip is recorded as its own ADR line by this session
under the dispatch's standing delegation, and nothing of Stage 2 is opened.

**The honest expectation, stated before the runs.** The in-sample census already
answers the recall question for the row the matrix headlines: under §1's
definition `opp_hot >= 3` keeps **0 of 1** win-direction proofs on band 35 and
**0 of 11** on the trigger-rich fixture, against band 15's empty denominator. The
mechanism is not subtle — `opp_hot` counts the OPPONENT's hot windows, so it
rises exactly where the DEFENDER direction proves — and no out-of-sample table
is likely to reverse it. This registration is therefore not expected to rescue
that row; it exists so that the row's fall is a measurement rather than an
argument, and so that anything that replaces it is measured on positions its
threshold never saw.
