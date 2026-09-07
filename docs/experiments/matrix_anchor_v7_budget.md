# OPTION MATRIX — what v7 does about the unequal budget. Revision 2.

> **FELL. THIS MATRIX HAS NOT SETTLED ITS DECISION AND NO OPTION IS SELECTED
> (D-699).** Its round-2 DECISION-RED-TEAM landed **16 of 18**. The three that
> decide it: it never cites **D-695**, the ADR that governs this run and pins
> "equal measured movetime per side", so its recommendation is forbidden by
> standing law it did not name; §4 claim 4 is wrong **in kind** — the
> precondition takes the non-search bound from 4.50 % to **3.53 %**, not to zero,
> because `engine_time_ms` is null for every sealbot answer; and §2.1's "load,
> not mystery" is the wrong explanation, the right one being **42 ms against
> 1 ms** by whether sealbot's first answer follows its own spawn or pistol's,
> measured inside one run at one load.
>
> **The surviving option is the round-2 reviewer's O7** — the series budgets
> 500/300 unchanged, after BOTH halves of the harness fix (revision 2 folded in
> the `ready` line and silently dropped "have the shim report its own elapsed
> time", which is the larger half), with the handicap reported as a MEASURED
> search-time ratio. **A revision 3 selecting it must cite and amend D-695.**
>
> Read the rest for its measured ground, which BOTH red-team rounds re-derived
> independently and every row of which reproduced — and not for its reasoning,
> which is what fell.

**Revision 1 is `matrix_anchor_v7_budget_rev1_SUPERSEDED.md`.** Its
DECISION-RED-TEAM landed **14 of 15** attacks and its recommendation FELL. This
revision is written against those attacks; where one of them changed the answer,
it is named at the place it changed it.

**Revision** `f27d233` for the measured ground, re-derived at the closing
revision before publication (D-692). **Instrument, named with its revision**
(`docs/process.md`, "Instrument governing revision"): `tools/anchor_overshoot.py`
at the revision this document is read at, tested by
`crates/pistol-arena/tests/anchor_overshoot_tests.rs` under gate 3. *(Revision 1
attributed its numbers to "a script written for this matrix" with no path and no
revision — A12, and the script now exists.)*

---

## §1 THE PREMISE OF REVISION 1 WAS FALSE, AND THAT IS THE LARGEST CHANGE

Revision 1 opened by asserting that D-438 — *"An opening-policy change, a budget
change, or any engine change is a NEW anchor, not a re-reading"* — forbids v7
from equalising the clock **and** printing a comparison table against v5 and v6.

**It forbids no such thing (A1).** D-438's dichotomy is *new anchor* versus
*re-reading*: it says take a fresh measurement rather than reinterpret an old
one. It says nothing about comparison or about series numbering. And this project
has already applied it in the opposite direction, on the sibling clause of the
same sentence: `sealbot_anchor_v3_prereg.md:23-25` invokes D-438 for an
opening-policy change — *"THIS IS NOT A RE-READING OF v1 OR v2"* — and then
`docs/decisions.md:1282` (D-608) compares the resulting anchor to its predecessor
and calls it an **overturning**: *"THE FIRST ANCHOR WITH A REAL DENOMINATOR
OVERTURNS D-606: 40 W / 60 L OVER FIFTY PAIRED OPENINGS, NOT 0 W / 100 L."*

A clause that lists opening-policy change and budget change in one breath cannot
forbid for the second what it licensed for the first. **What D-438 requires is
that the change be NAMED and the number re-taken — not that the series be
abandoned.** CLAUDE.md's own test applies: where both sides of a distinction
license the same conclusion it is not a distinction.

**So the decision this matrix settles is narrower than revision 1 thought.** It
is not "new series or not". It is: **which clock does v7 run at, and what must be
true of the harness before it runs.**

## §2 The measured ground

From the 200 per-game transcripts of v5 and v6 via `tools/anchor_overshoot.py`.
Receipts `artifacts/fast_wins/anchor_overshoot_v5.txt` and `…_v6.txt`.
**Independently re-derived by the DECISION-RED-TEAM with its own parser and its
own scope, and by the harness's own `report.json` as a second instrument: every
row agreed.**

| quantity | v5 | v6 | label |
|---|---|---|---|
| games / answers | 100 / 2090 | 100 / 1964 | **MEASURED** |
| answer-wall pistol / sealbot / total | 403.3 / 227.1 / **630.4 s** | 369.8 / 206.4 / **576.2 s** | **MEASURED** |
| pistol result | 40 W / 60 L | 42 W / 58 L | **MEASURED** |
| pistol max excess over budget | 15 ms | 14 ms | **MEASURED** |
| sealbot max excess over budget | **106 ms** | 15 ms | **MEASURED** |
| **pistol** first-of-game excess, median / max | 6 / 15 ms | 4 / 14 ms | **MEASURED** |
| **sealbot** first-of-game excess, median / max | 15 / 106 ms | 6 / 15 ms | **MEASURED** |
| pistol later-answer excess, median / max | 0 / 6 ms | 0 / 0 ms | **MEASURED** |
| sealbot later-answer excess, median / max | 0 / 11 ms | 0 / 4 ms | **MEASURED** |
| `engine_time_ms` reported, sealbot | 0 of 1050 | 0 of 986 | **MEASURED** |
| `engine_time_ms` reported, pistol | 1040 of 1040 | 978 of 978 | **MEASURED** |

**REVISION 1 SAID THE OVERSHOOT RESIDUAL WAS "ENTIRELY ON THE OPPONENT'S SIDE"
AND PRINTED THE SPLIT FOR ONE ENGINE ONLY (A7).** Both rows are here now, and in
v6 the two engines' first-answer overheads are indistinguishable (4/14 against
6/15). Pistol has the same shape, from its own per-game handshake. "Entirely" was
wrong and the omitted row is the one that showed it.

### §2.1 The 106 ms is process start-up, it is ~26 ms, and it is removable

**MEASURED, twice, independently.** The DECISION-RED-TEAM found that all 40 v5
sealbot answers with excess over 30 ms are the first answer of their game, and
measured the spawn charge at 326 ms against 300 ms when the client does not wait.
Re-measured here by driving the shim directly, five spawns each, load 3.13:

```
first answer of a game, budget 300 ms
  matchserver today (does NOT consume `ready`): 26, 25, 26, 26, 25 ms
  consuming `ready` first:                       0,  0,  0,  0,  0 ms
```

The mechanism is in two files this repository owns:
`tools/sealbot/sealbot_shim.py:38-49` imports the extension, imports `game` and
constructs `MinimaxBot` **before** printing `sealbot_shim: ready`, and
`tools/sealbot/matchserver/src/sealbot_client.rs:113-122` spawns the process and
returns from `new_game` without consuming that line — so the whole prefix is
charged to the first measured answer. `pistol_client.rs` completes its handshake
inside `new_game`.

**AND THE 7x DIFFERENCE BETWEEN TWO RUNS OF A BYTE-IDENTICAL CONFIG IS LOAD, NOT
MYSTERY.** v5's transcripts are stamped midday and v6's at 02:53; the medians are
15 ms and 6 ms. Revision 1 called this row "why this matrix exists"; it is a
load-dependent, once-per-game constant of about 26 ms, and §A4's `ps` clause is
what it argues for.

### §2.2 The size of the unfairness, bounded rather than asserted

Revision 1 argued that equal wall is not equal search. True, and now bounded.
The DECISION-RED-TEAM computed a rigorous upper bound on sealbot's non-search
wall — for an answer that reached the deadline, non-search ≤ wall − budget; for
one that returned early, non-search ≤ wall — of **4.50 % (v5)** and **3.50 %
(v6)** of its total wall, with the per-answer Python replay measured at **1–2 ms
on a 300 ms clock**.

**THE PROPORTION IS THE ARGUMENT (A8).** The imperfection in an equalised clock
would be about 1 %, worst-cased at 4.5 %. The inequality actually in the series
is **67 %**. Revision 1 rejected an option for the 1 % while recommending one
that preserved the 67 %.

## §3 The options

O1 and O2 are revision 1's. **O5 and O6 are new and are the red team's (A2, A3);
their absence is what made revision 1's recommendation look forced.** O3 is
withdrawn and O4 is folded in as a precondition rather than an option, for the
reasons given under each.

### O1 — run at 500/300, the series budgets, with the inequality labelled

- **Cost**: **ESTIMATED** ~10.5 min of answer-wall per 100 games, from v5's
  MEASURED 630.4 s and v6's 576.2 s, which differ by 9 %. *(Revision 1 labelled
  this MEASURED. A forecast of a future run derived from a past one is an
  estimate — A10's corollary.)*
- **What it buys**: everything except pistol is held, so a v6→v7 difference is
  attributable to the only thing that changed, which is what a series is for.
- **REVISION 1'S REJECTION OF THIS OPTION WAS MEASURED AGAINST THE WRONG
  POPULATION (A5)** — the defect class `docs/process.md` names, committed inside
  this matrix. It rejected O1 because "the label has measurably already failed to
  travel", citing six unqualified sites. But **at that revision no label existed
  anywhere**: neither finding document carried a budget qualifier, and D-697 was
  the first time one was written. The measurement was of a period in which there
  was nothing to travel.
- **Its real failure mode**: a reader takes 40 W / 60 L as a fair head-to-head.
  The remedy is the label, and the label is now **written** (D-697) at both
  finding documents and all downstream records — free, and orthogonal to every
  option (A6).

### O2 — equalise upward at 500/500, as a new series

- **Cost**: **MEASURED** ~12.9 min per 100 games. *(Revision 1 said the scaling
  "could be measured only by running it" and the red team measured it in about
  twenty seconds — sealbot's deadline-bound answers go 301 → 501 ms — which is
  D-291's finding exactly, A10.)*
- **What it fails at**: it raises **sealbot's** budget 1.67x, and sealbot
  consumes it — **MEASURED**, 68.9 % of its v5 answers reach the deadline. **The
  opponent is the ruler**, and §A5 of the protocol already says a stronger
  sealbot is "a second series, never a replacement". This makes the ruler
  stronger while keeping the series name.

### O5 — equalise, stay in the series, print the table with the change named

The red team's A2. Under §1 this is legitimate where revision 1 thought it was
not, and it is what revision 1's O2 should have been.

- **Cost**: as O2, **MEASURED** ~12.9 min.
- **What it fails at**: it inherits O2's defect. Naming the change does not stop
  the ruler from moving; it only stops a reader from being misled about it.

### O6 — equalise downward at 300/300

The red team's A3, and the one it would select.

- **Cost**: **ESTIMATED** ~7.8 min — pistol's 403.3 s scaled by 300/500 plus
  sealbot's 227.1 s unchanged. The cheapest option on the page.
- **What it buys**: the ruler is held at its standing value, and only pistol —
  the thing the series tracks — moves. 300 ms is inside CLAUDE.md's design point
  ("0.5 s, stretch 0.1-0.3 s").
- **WHAT THE RED TEAM'S OWN ARGUMENT DOES TO IT.** A3 says an anchor's opponent
  must be held fixed. The same sentence applies to the SUBJECT's measurement
  conditions: moving pistol from 500 ms to 300 ms makes a v6→v7 difference
  confound "pistol changed" with "pistol got 40 % less time", which is the exact
  confound the series exists to prevent. **There is no clock change that
  preserves comparability**, because both engines' budgets are part of the
  measurement setup.
- And it measures pistol at a budget it is not deployed at: CLAUDE.md's
  deployment budget is 0.5 s and D-534's forfeit analysis is at 500 ms.

### O3 — WITHDRAWN. It was the union of the two options it had just rejected

Revision 1 recommended running both 500/300 and 500/500. **A4 is unanswerable**:
the 500/300 arm is O1's run and the 500/500 arm is O2's, so O3 inherits O1's
label-dependence and O2's n=1, and its only defence against its own named failure
mode was the instruction "never subtracted" — a label, on a page whose siblings
dropped the last label at six sites.

**And the instrument cannot carry it (A14).** Two independent 100-game arms at
p ≈ 0.4 give SE(difference) = 6.9 pp, a 95 % interval of **±13.6 pp** on the
difference — while the reproducibility floor is already visible in 40 → 42 on a
byte-identical config. §A7 registers "direction only", which is a ONE-NUMBER
vocabulary; two arms give the reader two candidate values for "the series" and no
registered way to choose.

### O4 — folded in: the harness fix is a PRECONDITION, not an alternative

Revision 1 listed "do not run; fix the harness first" as an option and rejected
it as "right about the mechanism and wrong about the schedule". **A11 lands**:
there is no schedule. v7's trigger is the first Phase 2 `h1`, and
`docs/experiments/wp22_FINAL_SUMMARY.md` records Phase 2 as not started. A fix
measured at 26 ms → 0 ms, in two files this repository owns, with no run waiting
on it, is not deferred by a schedule argument.

It is therefore not an option here but a **precondition** on all of the above.

## §4 Recommendation: **O1, after the O4 precondition**

**v7 runs at the series budgets — pistol 500 ms, sealbot 300 ms — with the
inequality labelled, after the client is fixed to consume the shim's `ready`
line.**

The reasoning is what survived the attacks and is not what either revision 1 or
the red team proposed:

1. **§1 removes the thing that forced a change at all.** No rule requires v7 to
   equalise. Revision 1 changed the budget because it believed D-438 made
   equalising and comparing incompatible, and then had to choose which to give
   up. The premise was false, so the choice was never forced.
2. **An unequal clock held CONSTANT across a series is not a defect in the
   series.** It is a fixed property of the ruler. It is only a defect where a
   number is quoted as a fair head-to-head — and the remedy for that is the
   label, which is now written and cost nothing (A5, A6).
3. **Every option that moves a clock breaks comparability**, O6 included, and
   comparability is the one thing the series has. §2.2's proportion cuts this way
   too: the 67 % is the CONSTANT, and a constant is what a series can see past.
4. **The 4.5 % that made equalisation look urgent is 26 ms of process start-up**
   and goes to 0 with the precondition (§2.1).

**WHAT THIS OWES, and it is registered rather than assumed**: the run reports
both sides' per-answer distributions from the transcripts, names the three
overshoot terms, and states in the same breath as every W/L number that pistol
had 1.67x the clock. If a fair head-to-head is ever wanted, it is a SEPARATE
question from the anchor series and gets its own document — at which point O5's
shape, with the harness fix already landed, is the one to take.

**THE STRONGEST SURVIVING ATTACK against this recommendation**, and it is A8's
turned around: a series whose ruler gives one side 1.67x the clock can be
internally consistent for ever and still never answer the question a reader
actually brings to it, so the label is doing load-bearing work that a single
sentence has now failed to do at six sites once already.
