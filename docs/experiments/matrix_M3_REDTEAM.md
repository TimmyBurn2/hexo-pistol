# DECISION-RED-TEAM — MATRIX M3, the differential gate's instrument

**Pinned revision attacked: `f8e73e4`.** Reachable and confirmed
(`git rev-parse f8e73e4` → `f8e73e45e1291ea08416e71c6658e5cfe061343d`).

**Does it still match HEAD? NO.** HEAD was `1b645ac` when this round opened and
moved to `dab170b` during it (a concurrent session landed the M4/M6 witness
test). **`docs/experiments/matrix_M3_soundness_instrument.md` is byte-identical
at `f8e73e4` and at HEAD** — `git diff f8e73e4 HEAD -- <matrix>` is empty — and
the only `crates/` change since `f8e73e4` is the ADDITION of
`crates/pistol-solver/tests/wp15b_mutation_witnesses.rs` (150 lines, no existing
file touched). Every structural fact below therefore holds at the pinned
revision; where I used the new witness file I say so and give the pinned-revision
reproducer.

**Fresh context.** I did not author this matrix, the design it belongs to, U4, or
any review behind them. I read `CLAUDE.md` first.

**Reproducer discipline.** Every finding below carries a command I actually ran
and its real output. Mutation work ran in a throwaway git worktree at
`/home/tom/pistol-m3-rt` (checked out at `f8e73e4`, removed at the end); the live
tree was never modified. My own numeric claims are marked **MEASURED** or
**ESTIMATED**.

---

## 1. VERDICT

**EVERY STATED OPTION FALLS.** S-A, S-B, S-C, S-D, S-F, S-H and S-I fall on
grounds the matrix itself argues and I could not break; S-E falls on four
independent attacks, and S-G falls with it. The recommendation's decisive cell —
"the referent is written against the plan-family definition" — selects, when
followed literally against the project's own ADOPTED canonical reference (D-266,
`DEF-T`: *exact **minimum** hitting set*), a referent that I MEASURED disagreeing
with a correct engine on a legally reachable FILTERED node: `{(-1,0),(-1,5),(4,0)}`
against `{(-1,0)}`. So S-E's EQUALITY criterion is RED on a correct engine when
the referent is genuinely independent, and green only when the referent consults
`cover.rs` — D-295's defect wearing S-E's name. That is worse than the
self-disclosed hole admits: independence and greenness are in TENSION, not merely
unenforced. Beside it: the matrix kills S-F "on landed ADR lines, not on
judgement" and then demotes the *same* test to a cost cell for S-E, whose primary
mechanism is the act D-115 forbids by name ("no item is made public … to let a
test reach it"); its S-E cost cell's one MEASURED number does not reproduce (the
"8 plain `assert!`" is one comment plus six `#[cfg(test)]` asserts — production
has exactly ONE); its ground 3's "forced by the tree" is falsified by a one-word
edit that builds and clippies clean; and its flip clause 3 flips to a population
that I MEASURED, in 0.23 s with a landed instrument, firing the criterion **8×
less often** than the corpus it is escaping. Four viable rows are missing,
including the composite the design actually writes as adopted ("S-E, with the
reduced S-C beside it", U4 §8.2 / §8.3 / §8.7) and a landed from-scratch cover
referent that the matrix's own fact-4 command PRINTS and it did not read. The
architect has pre-committed to stopping rather than forcing a survivor; that is
the honest outcome here. The survivors are all in the MISSING-ROW list, and none
of them has been costed or attacked.

---

## 2. PER-OPTION SURVIVAL

| Option | Verdict | The one attack that did it |
|---|---|---|
| **S-A** — value agreement | **FALLS** | The matrix's own ground, and I could not break it: an interior drop that never moves a root maximum leaves the root value identical. Criterion preserved by the defect class. |
| **S-B** — candidate-set containment (staged ⊇ radius) | **FALLS** | Falls on the matrix's count (i). But its count (ii)'s MEASURED evidence proves the WRONG DIRECTION — "157 of 182 have a cover cell OUTSIDE the radius-1 ball" falsifies staged ⊆ radius, not staged ⊇ radius (**F11**). It still falls; the matrix cited the datum that does not show it. |
| **S-C** — class-restricted answer agreement | **FALLS** | Measurement, as the matrix says. But the row prices the SUPERSEDED 554.2 s workload; the reduced S-C (radius 1, MEASURED 17.89 s), which U4 §8.2 puts INSIDE the adopted gate, is not a row at all (**F5**). |
| **S-D** — S-C plus a mutation gate | **FALLS** | With S-C. A strength measurement is not a criterion; I could not break this. |
| **S-E** — per-node survival-set EQUALITY, two halves | **FALLS** *(repairable)* | **F2.** A genuinely independent referent, written from the mitigation's own named source (D-266's canonical calculus, `DEF-T` = minimum hitting set), is RED on a correct engine at a legally reachable FILTERED node — REPRODUCED. The only referent that is green is one that consults `cover.rs`, which is D-295's defect. Reinforced by **F4** (D-115 breach by name, on the matrix's own kill-test for S-F), **F1** (its cost cell's MEASURED number does not reproduce), **F9** (its own fallback is vacuous on the 70.8 % BATCHED population) and **F10** (ground 3's "forced by the tree" is false). |
| **S-F** — the in-source `cfg` seam | **FALLS** | The matrix's ground, verified: all four named scripts exist and drive release binaries (`grep -c -- '--release\|release-checked'` → 5 / 1 / 1 / 5). D-129 is on point. |
| **S-G** — S-E plus sampled positions | **FALLS** | Inherits every S-E defect (it IS S-E plus a sampler) and adds **F3**: MEASURED by me in 0.23 s, the sampled population fires the FILTERED row at **3.1 %** against the corpus's **25.0 %**. The sampler dilutes the criterion 8×. |
| **S-H** — the generator self-checks | **FALLS** | The matrix's ground is CLAUDE.md's own clause and is correct. I could not break it. |
| **S-I** — no differential gate | **FALLS** as a positive choice | Leaves the named stage unmeasured, as the matrix says. Recorded here because **F9** shows the RECOMMENDED option's own fallback clause silently flips to exactly this, for exactly the named stage, under S-E's name. |

**No stated option survives.** The four MISSING rows in §4 (F5, F6, F7, F8) are
where survivors, if any, live. They are not mine to select and none has been
costed.

---

## 3. THE STRONGEST SURVIVING ATTACK — per surviving option

**There is no surviving option.** Per the brief's Rules, that is stated plainly
rather than forced. For the architect's benefit, the strongest attack that any
REPAIRED form of the two live candidates must answer, in ADR-quotable form:

**Against a repaired S-E** (mitigation re-pointed at U2 §5.3's convention rather
than at the calculus's `DEF-T`): *the criterion's operative term —* inclusion-minimal
*cover — is specified nowhere in the ADOPTED canonical threat reference (MEASURED:
`grep -c inclusion docs/research/threat_calculus_v1.md` → 0), whose `DEF-T` states
the opposite convention (minimum hitting set), so the referent's independence rests
on a five-word design sentence rather than on the reference D-266 makes canonical,
and mutation M4 is the ledger's own admission that the two conventions differ on a
position a legal game reaches.*

**Against any option that widens `pistol_search::staged`:** *D-115 forbids by name
the act the option requires — "no item is made public, no `pub(crate)` is widened
and no signature is altered to let a test reach it" — and the matrix applies that
same landed-ADR test to kill S-F, so an option adopted over it is adopted by
judgement in exactly the place the matrix said judgement was not needed.*

---

## 4. FINDINGS

### F1 — **KILL.** Fact 5's MEASURED "8 plain `assert!`" does not reproduce: production `pvs.rs` holds ONE

**Claim attacked.** Facts table row 5: *"An always-on `assert!` in `pvs.rs` is the
house idiom, not a novelty: MEASURED **8** plain `assert!` against **2**
`debug_assert!` in that file today."* Load-bearing twice — S-E's cost cell
("**MEASURED** the house idiom there, 8 already (fact 5)") and recommendation
ground 3 ("Fact 5 measures that this is the existing idiom in that file").

**Contradicting evidence.** The matrix's own command counts a COMMENT and six
asserts inside `#[cfg(test)] mod tests`. `#[cfg(test)]` opens at line 441 of a
552-line file.

**Reproducer and real output:**

```
$ grep -nE '(^|[^_a-z])assert!' crates/pistol-search/src/pvs.rs | awk -F: '{print $1}' | while read l; do
    if [ "$l" -ge 441 ]; then tag="IN #[cfg(test)]"; else tag="production"; fi
    printf "line %-4s %-16s %s\n" "$l" "$tag" "$(sed -n "${l}p" crates/pistol-search/src/pvs.rs | sed 's/^ *//' | cut -c1-60)"
  done
line 199  production       // carries the same statement as an always-on `assert!` unde
line 244  production       assert!(
line 500  IN #[cfg(test)]  assert!(score.is_some(), "a non-abortable iteration complete
line 501  IN #[cfg(test)]  assert!(!run.line().is_empty(), "a completed iteration has a
line 502  IN #[cfg(test)]  assert!(
line 510  IN #[cfg(test)]  assert!(score.is_none(), "the abortable iteration aborts");
line 511  IN #[cfg(test)]  assert!(run.aborted, "the run records the abort");
line 512  IN #[cfg(test)]  assert!(

$ sed -n '1,440p' crates/pistol-search/src/pvs.rs | grep -cE '^\s*assert!'
1
$ sed -n '1,440p' crates/pistol-search/src/pvs.rs | grep -cE '^\s*debug_assert!'
1
$ grep -n 'cfg(test)' crates/pistol-search/src/pvs.rs
441:#[cfg(test)]
```

**MEASURED, mine:** production `pvs.rs` carries **1** always-on `assert!`
(`NO_CANDIDATES_MID_TURN`, line 244) against **1** `debug_assert!` (line 201).
The ratio is 1:1, not 8:2 — and the file's own lines 195–199 are a written-out
D-129 taxonomy argument distinguishing the two cases, which is evidence that the
idiom in that file is *justify which macro you are in*, not *always-on asserting
is routine*. This is precisely CLAUDE.md's named class: **a command that counted
the wrong symbols**.

**KILL** — a MEASURED number in a cost cell that does not reproduce, in a matrix
whose header promises "every MEASURED value is either a structural fact of the
tree with its command beside it, or is cited to the session that measured it".

---

### F2 — **KILL.** S-E's criterion is RED on a correct engine when the referent is genuinely independent

**Claim attacked.** Ground 4 ("The referent is independent in the sense D-295
requires") and the self-disclosed hole's mitigation: *"the referent is written
against the plan-family definition and the reviewer of the IMPL commit is asked
whether it was."*

**Contradicting evidence.** The plan-family definition lives in
`docs/research/threat_calculus_v1.md`, which D-266 adopts as **the canonical
threat reference**. Its `DEF-T` reads: *"threat number t(F) | exact **minimum
hitting set** over plan family F"*. S-E's criterion needs the union over
**inclusion-minimal** covers — a term that occurs **ZERO** times in that
canonical reference and whose only normative statements in the project are U2
§5.3's five words and `cover.rs` itself, which is the subject. U4 §8.4's mutation
**M4 is literally "minimum-cardinality covers instead of inclusion-minimal"** —
the ledger treats this as a live defect axis.

**Reproducer and real output** (run at `f8e73e4` in the worktree; `cover.rs` is
byte-identical at HEAD, only a test file was added):

```
$ grep -c 'inclusion' docs/research/threat_calculus_v1.md
0
$ grep -n '^| DEF-T' docs/research/threat_calculus_v1.md
30:| DEF-T | threat number t(F) | exact **minimum hitting set** over plan family F ... `[PROVEN]` |
```

Then, on the M4 witness — a position PINNED as one a legal game reaches, replayed
ply by ply through `GameState`, at which `can_win_this_turn` is `None` and
`blocking_covers` answers `Minimal` (i.e. a genuine FILTERED node):

```
$ cargo test -p pistol-solver --test zz_probe probe_calculus -- --nocapture
running 1 test
INCLUSION-MINIMAL union (cover.rs, the subject): [Coord { q: -1, r: 0 }, Coord { q: -1, r: 5 }, Coord { q: 4, r: 0 }]
MINIMUM-CARDINALITY union (DEF-T, the calculus):  [Coord { q: -1, r: 0 }]
EQUAL? false
test probe_calculus_vs_cover_union ... ok
```

(The probe replays P1 `(0,0)(1,0)(2,0)(3,0)(-1,1)(-1,2)(-1,3)(-1,4)(0,7)` and P2
`(-2,0)(5,0)(-1,-1)(-1,6)(4,-4)(5,-4)(-4,4)(-5,5)` through `GameState`, takes
`blocking_covers(P2, Two)`, and forms both unions from the same `Cover`.)

**Why this is a KILL and not a wound.** S-E half one asserts EQUALITY. So:

- A referent written from the mitigation's own named source — the plan-family
  definition, `DEF-T` — emits `{(-1,0)}` where a **correct** engine emits
  `{(-1,0),(-1,5),(4,0)}`. The gate is RED on a correct engine.
- The only way to make it green is a referent that adopts `cover.rs`'s
  minimality convention. Adopting the subject's convention for the exact
  arithmetic under doubt is D-295's defect — "checking a shipped oracle against
  a test-side oracle derived from the same" — which is what ground 4 claims S-E
  escapes.

The matrix's disclosure — *"nothing enforces that the independent referent is
actually independent"* — understates this. The hole is not that independence is
unenforced. It is that **independence and greenness are in tension**, and the
matrix's own mitigation sentence points at the branch that is red. The concession
is therefore also **inoculation in effect**: it names a softer version of the hole
and offers a judged control, and a reader who credits the honesty does not run the
`grep -c inclusion` that takes two seconds and shows the control pointing the wrong
way.

**Repair, named because it is cheap:** the mitigation must name **U2 §5.3's**
convention (which does say inclusion-minimal) rather than "the plan-family
definition", and the `DEF-T` conflict needs an ADR line, because D-266 makes the
calculus canonical and the calculus states the other convention.

---

### F3 — **KILL.** Flip clause 3's remedy moves the criterion into a population where it fires 8× less often, and the matrix declares unmeasurable a number that is already measured

**Claim attacked.** Flip clause 3: *"Trigger: a MEASURED count of FILTERED nodes
across the gate corpus that is small enough for the criterion to be near-vacuous
in practice — … it is measurable once the generator exists. Remedy: **flip to
S-G**, adding sampled legal positions to reach the population the corpus misses.
Reachable because S-G is S-E plus a sampler."*

**Contradicting evidence.** A landed instrument —
`crates/pistol-solver/tests/wp15b_census.rs`, committed at `7941775` and running
in under a second — already MEASURES how often the FILTERED row fires across four
populations, including a uniform-playout population of the exact kind S-G's
sampler would draw. The sampled population is the **worst** of the four.

**Reproducer and real output** (0.23 s, at the live tree; the harness is
unchanged since `f8e73e4`):

```
$ cargo test -p pistol-solver --release --test wp15b_census -- --ignored --nocapture
== corpus roots  (n = 24)
   radius-2 ball 77.96 | filter fires 25.0% | impossible 4.2%
   batched nodes: 17 of 24 (70.8%), their ball 82.18
== +1..3 turns, radius-2 draw (REPORTED)  (n = 576)
   radius-2 ball 94.50 | filter fires 18.4% | impossible 1.4%
   batched nodes: 354 of 576 (61.5%), their ball 99.08
== +1..3 turns, radius-8 draw (SUPERSEDED)  (n = 576)
   radius-2 ball 123.66 | filter fires 13.7% | impossible 1.2%
== uniform playouts to 80 plies  (n = 960)
   radius-2 ball 376.47 | filter fires 3.1% | impossible 1.7%
   batched nodes: 888 of 960 (92.5%), their ball 364.49
test wp15b_census ... ok
test result: ok. 1 passed; ... finished in 0.23s
```

**MEASURED, mine:** FILTERED fires on **25.0 %** of corpus roots and on **3.1 %**
of uniform playouts — an **8.1×** dilution in the direction the remedy moves.

Two separate defects follow, and both are F5's class:

1. **The remedy moves away from the trigger.** The trigger is "the corpus does not
   exercise the FILTERED row enough to be a gate". The remedy is to add a sampled
   population that exercises it **eight times less densely per node walked** — and
   the matrix's own S-G failure-mode cell says "a per-CI-run gate whose cost is
   unbounded is how a gate gets disabled". The two compound: S-G buys FILTERED
   observations at roughly 8× the corpus's cost per observation, MEASURED, and the
   matrix cites the sampler's mere existence (`playouts.rs`) as what makes the flip
   cheap, having never measured its yield.
2. **The matrix declares a measured quantity unmeasurable.** "It is measurable once
   the generator exists" is true of the exact per-search-node count and false of the
   quantity the trigger is *about*. A MEASURED prior — 3.1 % to 25.0 % across four
   populations — sat in a sibling unit at the pinned revision (`U3_tier_t.md`, the
   census block) and in a runnable instrument, and the matrix carries it neither as
   MEASURED nor as ESTIMATED, writing instead "**Coverage is the corpus, not the
   population**" with no number at all. D-291's clause is on point: an estimate
   settleable in seconds is a finding, and this one was settleable in 0.23 s.

**KILL** on flip clause 3, and a wound on S-G independent of S-E's fate.

---

### F4 — **KILL.** D-115 forbids S-E's primary mechanism BY NAME, and the matrix applies that same test asymmetrically

**Claim attacked.** S-E's cost cell (`pistol_search::staged` becomes `pub` …
"the module goes public for a test's benefit"), scored as a **cost**; against
S-F's failure mode, which reads **"FALLS on landed ADR lines, not on judgement."**

**Contradicting evidence.** D-115's text is not a constraint, it is a
prohibition, and it names the act:

```
$ grep -n '^D-115:' docs/decisions.md | cut -c1-420
260:D-115: In-source `#[cfg(test)] mod tests` blocks are PERMITTED, for pinning private
invariant guards only — behaviour-level pinning stays in the behaviour-named integration
suites under `tests/` (CLAUDE.md rule 7), and no item is made public, no `pub(crate)` is
widened and no signature is altered to let a test reach it.
```

The matrix's own words for what S-E does: *"the module goes public for a test's
benefit"*, and *"fact 2 makes this necessary, since `pvs` cannot be reached"*.
That is the clause's subject, verbatim.

So the matrix runs one test — *does a landed ADR line forbid this?* — and returns
FALLS for S-F and "a real cost" for S-E. Ground 1 claims S-E is "the only
surviving row whose criterion the defect class can FALSIFY"; survival was decided
before this test was applied to it.

**Not a full KILL of the option, only of its scoring as written:** flip clause 2
pre-empts the trigger and offers the in-source `#[cfg(test)]` remedy, which D-115
does permit — *for private invariant guards*. What the matrix does not say is
that the remedy also **retires a registered test row**:
`staged_filtered_set_equals_the_minimal_cover_union` is one of the five §11 rows
`section_owner_table.md` assigns to U4, and it is a behaviour-named integration
test, which D-115 routes to `tests/` and not into the module. The flip changes
U4's registered test plan and the clause does not say so.

**KILL** on the matrix's scoring symmetry; **WOUND** on the option.

---

### F5 — **KILL (MISSING ROW).** The option the design writes as adopted is not a row

**Claim attacked.** The header: *"Nothing below is selected, and the incumbency of
S-E is not evidence. It is entered as a row like every other row."*

**Contradicting evidence.** The thing U4 writes as adopted is **not S-E**. It is
a COMPOSITE, and U4 says so in three places:

```
$ grep -n 'reduced S-C beside it\|with a reduced S-C\|S-E with the reduced S-C' docs/experiments/U4_soundness_instrument.md
158:### 8.2 THE DIFFERENTIAL GATE — S-E, with a reduced S-C beside it — **SELECTION OPEN**
252:| **THE DIFFERENTIAL GATE** — S-E, with the reduced S-C beside it | §8.2 |
418:**THESE FOUR — the tactical suite under Staged (§8.3), the differential gate
     (§8.2: S-E with the reduced S-C beside it), ...
```

No row in the matrix is that composite, and no row is the reduced S-C alone. The
matrix's S-C row instead prices the **superseded** workload —
`243 363 538 nodes / 554.2 s` — and calls it "MEASURED not runnable", when U4
§8.2 already replaced it: *"Reduced S-C — a cheap mate-class regression only.
Radius 1, the three fixtures plus one built mate-in-3 that gate 10 already
affords at **MEASURED 17.89 s**."* The matrix even CARRIES the 17.89 s in its
numbers table and then uses it in no row.

This is F11's class applied to the adopted option itself: the restructure red
team's F11 found a matrix missing two rows and that was a finding *even though
neither was selected*. Here the missing row is the one the design says is
adopted, and its absence lets the matrix kill "S-C" at 554.2 s while the live
proposal runs it at 17.89 s.

**KILL.**

---

### F6 — **KILL (MISSING ROW).** Containment against an externally derived LOWER bound is absent, and ground 1 justifies EQUALITY with a defect outside the named class

**Claim attacked.** Ground 1: *"Equality rather than containment is what does the
work: mutation M3's phase-1 cover cells are provably a SUBSET of the phase-0
union, so failing to regenerate can only OVER-generate — and a containment
criterion would have let M3 live."*

**Contradicting evidence.** The matrix's own organising principle, stated in its
own §"WHAT THE OPTIONS ARE OPTIONS ABOUT":

> **Does the staged generator ever drop a cell a proven tactic needs?** … Every
> row below is scored first on whether its criterion can be FALSIFIED by a
> generator that **drops a needed cell**.

M3 **over-generates**. Over-generation is not a member of the named defect class.
So ground 1 — the ground that selects S-E's distinguishing feature — is argued
from a defect the matrix's own scoring rule excludes. The matrix moves its
goalposts between the scoring rule and the recommendation.

And the row the scoring rule actually favours is absent: **containment against a
calculus-derived LOWER bound** — assert that every cell `LAW-HIT`/`LAW-FORCE`
makes required is present in the emitted set. `LAW-FORCE` is `[PROVEN]` in the
adopted reference:

```
$ sed -n '/^### LAW-FORCE/,+3p' docs/research/threat_calculus_v1.md
### LAW-FORCE `[PROVEN]` (was T3 + D2)
If the opponent has ≥1 plan and the mover cannot win this turn, every non-losing mover
move hits **all** opponent plans.
```

This row is materially different from S-B, whose referent is the RADIUS policy (a
relation over-generation preserves). A required-set lower bound is the literal
negation of the named defect — a dropped needed cell falsifies it directly — and,
decisively, it needs **no** `cover.rs`-specific minimality convention, so **F2
does not touch it**. Its cost is the strength question (extra cells go
unmeasured), which CLAUDE.md hard rule 6 already assigns to SPRT.

The matrix's framing question is the right question. Its NINE ROWS are not the
right nine: they are all set-comparison or answer-comparison instruments plus a
null, and the framing's own criterion picks out a shape that is not among them.

**KILL** on ground 1; **MISSING ROW** that is not mine to select.

---

### F7 — **MISSING ROW.** A recorded-and-compared per-node census is absent, and it dissolves S-E's own named failure mode

**Claim attacked.** The two-half architecture as the only shape available, and
S-E's failure-mode cell: *"Half two's `assert!` fires in every profile, so a false
positive is a production abort, not a test failure."*

**Contradicting evidence.** A recorder seam on `Run` — the searched candidate set
recorded per node, compared against the referent **offline** in the test — covers
**both** halves with one criterion and one referent: it sees the generator's
output *and* any drop made after `order`, because it records what was actually
searched rather than what was generated. It needs no always-on production
`assert!` at all, so the production-abort failure mode the matrix names for half
two does not arise. Its real cost — a nullable branch per node, and the ordering
question of whether "searched" is observable without also observing cutoffs — is
a genuine cost cell the matrix never writes.

This shape was named in my brief as a candidate and occurs zero times in the
matrix:

```
$ grep -ci 'census\|record.*compare\|recorder' docs/experiments/matrix_M3_soundness_instrument.md
0
```

**MISSING ROW**, not a kill on any stated option.

---

### F8 — **KILL.** A landed, from-scratch, independent cover referent already exists, and the matrix's own fact-4 command printed it

**Claim attacked.** S-E's cost cell ("a from-scratch referent in that crate's
`tests/common/`", priced as new work), ground 4's independence argument, and the
self-disclosed hole's "the mitigation available is procedural, not mechanical".

**Contradicting evidence.** `crates/pistol-solver/tests/common/reference.rs` is a
**366-line landed from-scratch reference** implementing exactly the arithmetic
S-E's referent needs, sharing only `pistol-core`, never touching `ThreatState` —
D-68's pattern, third instance (D-106). Its `blocking_covers` doc comment reads:

```
$ sed -n '223,226p' crates/pistol-solver/tests/common/reference.rs
    /// The inclusion-minimal covers of the attacker's hot windows, by the
    /// definition: every subset within budget that covers, minus every one with
    /// a proper subset that also covers.
    pub fn blocking_covers(&self, defender: Player, budget: HitBudget) -> Cover {
```

and its module header states the independence argument the matrix says nothing
enforces:

```
$ sed -n '10,18p' crates/pistol-solver/tests/common/reference.rs
//! **Its scope is deliberately the whole query surface and not the table.** A
//! reference scoped to the table, with the shipped queries then run over it,
//! would compare the cover enumeration, the phase conditioning and the witness
//! selection AGAINST THEMSELVES — and those are exactly the arithmetic that has
//! been wrong before. So the covers here are enumerated by a different
//! algorithm (all subsets within budget, then keep the ones no proper subset
//! covers), ...
```

**The matrix ran the command that prints this file and read the output for one
thing only.** Its fact 4 is `ls crates/pistol-solver/tests/common/`:

```
$ ls crates/pistol-solver/tests/common/
fixtures.rs  mod.rs  patterns.rs  plans.rs  playouts.rs  reference.rs  region.rs  sha256.rs
```

The matrix reports `plans.rs` from that output (to argue revision 3's seam could
not be built) and does not report `reference.rs`, on the very question its
recommendation calls its weakest cell.

Three consequences:

1. **Inconsistent accounting.** The matrix credits existing code to S-G ("MEASURED,
   `playouts.rs` exists in BOTH …") and declines to credit existing code on the
   referent question. Same tree, same commit, opposite treatment.
2. **The mitigation is not the only one available.** An already-written and
   already-reviewed independent referent turns a judged procedural control into a
   landed one. Whether it is *reachable* (it is another crate's test module —
   `#[path]` inclusion, or promotion to a shared dev-dependency test crate, are the
   candidates) is a costing question the matrix never asks; note also that
   `pistol-search` does not yet depend on `pistol-solver` at all
   (`crates/pistol-search/Cargo.toml` lists only `pistol-core` and `pistol-eval`).
3. **A second referent needs a registered agreement criterion.** Writing a fresh
   pistol-search-side referent for the same definition creates a SECOND instrument
   for arithmetic a first already covers, with nothing in the matrix registering
   what their disagreement means. CLAUDE.md: *"two instruments blind to the same
   stage are one instrument reported twice."*

**KILL** on the completeness of the option set and on the honesty of the cost
column; **MISSING ROW** (differential against the landed R1 referent).

---

### F9 — **KILL.** Flip clause 1's remedy falls back to a criterion the owning unit MEASURES as vacuous on 70.8 % of the population, and blind to the named stage

**Claim attacked.** Flip clause 1: *"Remedy: **the gate falls back to S-E half two
alone** — the in-source `assert!`, which needs no referent and still catches
D-124's reproducer … **Reachable because the two halves are independent by
construction**: half two is an `assert!` in `visit` and does not consult the
referent at all."*

**The structural ground is TRUE and I verified it.** Half two is an assertion in
`visit` over the `forced` prefix; it does not touch the referent. Mechanical
separability holds.

**Remedial adequacy does not.** Two facts, both already in the owning unit:

```
$ grep -n 'VACUOUS there' docs/experiments/U4_soundness_instrument.md | cut -c1-260
383:| **M8** | ... Registered because the honest reading is that on the 70.8 % BATCHED
population `forced == 0` and the assertion is VACUOUS there — it earns its place only on
forced rows, and the mutation is what shows which |

$ grep -n 'S-E is blind at an' docs/experiments/U4_soundness_instrument.md | cut -c1-200
381:| M6 | ... Its class is mate and not S-E, because the mutant RETURNS rather than emitting
and S-E is blind at an `Impossible` node ...
```

And MEASURED by me (F3's census run): BATCHED nodes are **70.8 / 61.5 / 65.5 /
92.5 %** of the four populations; on every one of them `forced == 0` and half two
holds trivially.

So the fallback is:

- **vacuous on the majority of nodes** — 70.8 % at corpus roots, 92.5 % on
  playouts, MEASURED; and
- **structurally blind to the stage under doubt.** The matrix's own architecture
  assigns generation defects to half one and post-generation drops to half two.
  Half two asserts that the search consumes what the generator emitted as
  `forced`; a generator that never emits a needed cell as `forced` preserves that
  assertion perfectly. It is the same "components sharing an input" vacuity the
  matrix uses to kill S-H, one level up.

The trigger is "the referent cannot be written independently" — i.e. half one is
unsound. The remedy deletes half one. For the stage the matrix went to the trouble
of naming — *does the staged generator ever drop a cell a proven tactic needs?* —
the remedy is **S-I**, a row this matrix rejects, arriving under S-E's name. F5's
class exactly: the remedy is mechanically reachable from the trigger and does not
address the harm.

Note also what this does to the whole document's structure: **F2 fires clause 1's
trigger at the pinned revision** (the operative convention is specified nowhere
that is not the subject or a five-word design sentence), so this is not a
hypothetical branch — it is the branch the matrix is on.

**KILL.**

---

### F10 — **KILL.** Ground 3's "forced by the tree" is false; it is one word, and the matrix already pays that word for `staged`

**Claim attacked.** Ground 3: *"**The two-half split is forced by the tree, not
chosen for elegance.** Fact 2: `pvs` is `pub(crate)`, so a test cannot see `Run`."*

**Fact 2 itself reproduces.** At `f8e73e4`, in the worktree:

```
$ cat > crates/pistol-search/tests/zz_rt_probe.rs <<'EOF'
#[test]
fn probe() { let _ = std::any::type_name::<pistol_search::pvs::Run>(); }
EOF
$ cargo build --tests -p pistol-search
error[E0603]: module `pvs` is private
  --> crates/pistol-search/tests/zz_rt_probe.rs:4:50
  --> crates/pistol-search/src/lib.rs:40:1
```

**The inference from it does not.** `Run` and all its methods are already `pub`
INSIDE a `pub(crate)` module. The whole of fact 2 is one keyword on one line:

```
$ sed -i 's/^pub(crate) mod pvs;$/pub mod pvs;/' crates/pistol-search/src/lib.rs
$ grep -n 'mod pvs' crates/pistol-search/src/lib.rs
40:pub mod pvs;
$ cargo build --tests -p pistol-search 2>&1 | grep -E '^(error|warning)' ; echo "exit: $?"
exit: 0                       # no errors, no warnings
$ cargo clippy --tests -p pistol-search -- -D clippy::all 2>&1 | tail -1
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.12s
```

and with it a test calling `Run::iterate` compiles:

```
$ cat > crates/pistol-search/tests/zz_rt_probe2.rs <<'EOF'
#[test]
fn probe2() { fn _f(r: &mut pistol_search::pvs::Run<'_>) -> Option<i32> { r.iterate(2, false) } }
EOF
$ cargo build --tests -p pistol-search 2>&1 | grep -E '^(error|E0)|-->' ; echo "BUILD RESULT: $?"
BUILD RESULT: 0
```

(Worktree reverted and removed; the live tree was never touched.)

**MEASURED, mine:** widening `pvs` is a one-word edit that builds and clippies
clean at the pinned revision. The matrix's S-E cost cell already pays exactly this
word for a sibling module (`pistol_search::staged` becomes `pub`), so the tree does
not forbid the move — the matrix simply declines it in one place and buys it in the
other, unargued. What is genuinely absent is a **per-node observation seam** (`visit`
is private and there is no recorder), and that absence is a design choice nobody has
matrixed — see F7. Ground 3 dresses a choice as a structural necessity, and it is one
of the four grounds the recommendation rests on.

**KILL** on ground 3.

---

### F11 — **NOT A KILL.** S-B falls, but its cited MEASURED evidence proves the opposite direction

**Claim attacked.** S-B's failure mode (ii): *"It is FALSE as a claim under the
adopted Tier-T option: U4 §8.3 MEASURES the FILTERED row's set as a subset of the
radius-2 set and NOT of the radius-1 set — at
`mate_in_3_double_three_becomes_double_four`, **157 of 182** FILTERED descendants
have a cover cell **outside** the radius-1 ball. A gate asserting containment
there is RED on a correct engine."*

**Contradicting evidence.** S-B is defined in the same row as *"staged set **⊇**
the radius policy's set"*, and count (i) confirms that reading ("Containment is
preserved by every OVER-generating mutation"). To falsify `staged ⊇ radius` you
need a RADIUS cell missing from the staged set. "157 of 182 have a cover cell
outside the ball" is the other direction: it falsifies `staged ⊆ radius`.

```
$ sed -n '318,325p' docs/experiments/U4_soundness_instrument.md
  3. **FILTERED row** — the emitted set is the cover union. **It is a SUBSET of
     the radius policy's set at radius 2 and NOT at radius 1** ... MEASURED at the gate case
     `mate_in_3_double_three_becomes_double_four` (radius 1): of **182** FILTERED
     descendants, **157** have a cover cell outside the ball. MEASURED at
     `must_block_p2_five_in_a_row` (radius 2): **0 of 10**.
```

The datum that DOES support the claim is in the same paragraph — the radius-2
statement, where the FILTERED set is a strict subset. The matrix cited the other
one. S-B still FALLS, on count (i) and on §8.3's prose; but "the number the
matrix reached for does not show the thing the sentence claims" is the same class
as F1, and D-305's "a repair is not done until every claim resting on the repaired
thing has been re-read" is what makes it worth recording.

**NOT A KILL** — the option falls anyway.

---

### F12 — **NOT A KILL.** Fact 1's MEASURED count is wrong (13 vs 14)

**Claim attacked.** Fact 1: *"`ls crates/pistol-search/src/` — **13** entries, no
`staged.rs`"*, under a header reading "FACTS THE MATRIX STANDS ON — **MEASURED**
at `77f7397`".

**Reproducer and real output:**

```
$ git ls-tree --name-only 77f7397 crates/pistol-search/src/ | wc -l
14
$ git ls-tree --name-only f8e73e4 crates/pistol-search/src/ | wc -l
14
$ ls crates/pistol-search/src/ | wc -l
14
```

(The fourteenth is the `tt` directory, which `ls` prints.) The substantive half —
`staged.rs` does not exist, so every option is priced before the subject is built
— reproduces and is correct.

Related soft note, same row family: fact 3 says the hosting crate's
`tests/common/` "carries **nine** modules" with the command `ls …` → **10**
entries. Nine is defensible (`mod.rs` declares nine `pub mod`s, verified) but the
printed command does not yield it, which is the same reading gap.

**NOT A KILL** — but it is the second MEASURED count in this matrix that its own
command does not produce, and F1 is the first.

---

### F13 — **NOT A KILL.** The cost column does not measure what the options differ in

**Claim attacked.** The cost cells: S-E "Gate cost **ESTIMATED 40–90 s** per CI
run"; S-I "Zero"; the closing section "Per-CI-run cost **ESTIMATED 40–90 s**,
carried from U4 §13".

**Contradicting evidence.** Per U4's own cost row, 40–90 s is the whole FOUR-gate
soundness gate, and it explicitly includes the reduced S-C:

```
$ grep -n 'soundness gate per CI run' docs/experiments/U4_soundness_instrument.md | cut -c1-300
618:| The soundness gate per CI run | **ESTIMATED 40–90 s**, dominated by S-E's one traversal
per fixture plus the reduced S-C's **MEASURED 17.89 s**. Revision 1's 60–180 s priced a
workload that is days | to be MEASURED when it lands, and reconciled here |
```

So the same figure is entered as S-E's cost while covering three other gates plus
the reduced S-C, and S-I is entered at "Zero" although under S-I the tactical
suite, the colony family and the pattern fixtures all still run. The column
therefore cannot discriminate between the options, which is what a cost column is
for. The marginal question — *what does the differential gate add over the other
three?* — is nowhere costed.

Two smaller consistency notes in the same column. (a) The citation "U4 §13" does
resolve, but only through `section_owner_table.md` §6, which splits the superseded
§13 by row and assigns this one to U4; U4 itself contains no `§13`, and the matrix
promises "each is cited with its source so a reader can tell my measurements from
theirs". (b) `tools/SHELL_CHECKLIST.md` item 10 (the coverage rule — verified,
`grep -n 'produces a recorded number' tools/SHELL_CHECKLIST.md` → line 106) is
charged only in S-E's cost. S-A, S-B, S-C, S-D and S-H would each need a
`tools/` script wired into `tools/ci.sh` too, and none is charged for the
checklist review or the coverage-rule test.

**NOT A KILL** — no option's survival turns on it.

---

### F14 — **KILL (on the recommendation, not on a row).** The four grounds are the incumbent's four arguments, re-worded

**Claim attacked.** The matrix's reason for existing: *"S-E was supplied by the
DECISION-RED-TEAM that killed S-C, and a red team's replacement option has never
itself been attacked. … that is the base rate this matrix exists to stop resting
on. **Nothing below is selected, and the incumbency of S-E is not evidence.**"*

**Contradicting evidence.** The recommendation's four grounds are U4 §8.2's four
arguments for S-E, in §8.2's order, with §8.2's evidence. Every distinctive
evidentiary phrase in the grounds appears verbatim in §8.2:

```
$ # §8.2 extracted to u4_82.txt (lines 158-243), newlines flattened
$ for p in "phase-1 cover cells are provably a SUBSET of the phase-0 union" \
           "comparing the engine's per-node candidate set with the reference's" \
           "DERIVED FROM THE SAME FIXTURE" "plan-family implementation" \
           "another crate's integration-test" "all of which drive release binaries" \
           "the pattern rule 7 already names for movegen" "which is §8.1's own" \
           "the first \`forced\` candidates are searched unless a cutoff"; do
      printf "[%s] %s\n" "$(grep -c -F "$p" u4_82_flat.txt)" "$p"; done
[1] phase-1 cover cells are provably a SUBSET of the phase-0 union
[1] comparing the engine's per-node candidate set with the reference's
[1] DERIVED FROM THE SAME FIXTURE
[1] plan-family implementation
[1] another crate's integration-test
[1] all of which drive release binaries
[1] the pattern rule 7 already names for movegen
[1] which is §8.1's own
[1] the first `forced` candidates are searched unless a cutoff
```

**MEASURED, mine: 11 of 11** distinctive phrases I sampled from the grounds and
from S-E's and S-F's cells occur verbatim in §8.2. (An 8-word shingle overlap
puts literal reuse at 7.7 % — the prose is re-written; the ARGUMENT and the
EVIDENCE are not.)

The matrix's genuinely new content is real and should be credited: S-E's
failure-mode cell adds the false-positive-referent hazard, the production-abort
hazard and the corpus-vs-population hazard, none of which §8.2 states, and the
five rows S-F–S-I are new. But the four grounds the recommendation *stands on*
are the incumbent's brief. A matrix written to stop S-E resting on itself rests
its recommendation on the text that advocated S-E — and the four attacks above
(F2 on ground 4, F10 on ground 3, F6 on ground 1, F4 on the scoring test never
applied to S-E) are all attacks §8.2 was never subjected to and that this matrix
inherited un-run.

**KILL** on the recommendation's independence.

---

## 5. WHAT I COULD NOT BREAK — recorded so it is not re-attacked

- **The "never existed" header reproduces.** MEASURED: `S-E` occurs **0** times at
  `ec8f7fb`; `S-A`, `S-B`, `S-D` occur **1** each at `6feb40a`; `grep -c '^| Option |'`
  at `6feb40a` returns **3**. Commands:
  `git show ec8f7fb:docs/experiments/wp15b_design.md | grep -o 'S-E' | wc -l` → `0`;
  `git show 6feb40a:… | grep -c '^| Option |'` → `3`.
- **Fact 2 reproduces** (see F10's first block) — the inference from it does not.
- **Fact 4 reproduces**: `plans.rs` is in `crates/pistol-solver/tests/common/`, and
  no `src/` can `use` another crate's integration-test module. (What the same
  command also prints is F8.)
- **Fact 6 reproduces verbatim.** `crates/pistol-search/tests/common/mod.rs:8`:
  *"replays them through `GameState`, so a fixture that no legal game could reach
  fails loudly here"*, and `position()` asserts rule 3's parity by name.
- **S-F's four scripts exist and drive release binaries.** MEASURED:
  `grep -c -- '--release\|release-checked'` → `search_oracle_check.sh` 5,
  `tactical_check.sh` 1, `determinism.sh` 1, `movetime_check.sh` 5. S-F falls as
  the matrix says.
- **Every number cited to U4 §8 is in U4 §8 saying what the matrix says it says.**
  MEASURED: "28 class assertions, 0 RED" (§8.1 L121–122), "0 of 62" (§8.1 L134),
  "Three of seven mutations are killable" (§8.1 L141), "243 363 538 nodes in
  554.2 s" (§8.1 L145–146), "17.89 s" (§8.2), "157 of 182" (§8.3 L321–322),
  "ESTIMATED 40–90 s" (U4 Cost L618). The separation between the matrix's own
  measurements and cited ones is honestly drawn — **F1 and F12 are failures of
  the matrix's OWN measurements, not of its citations.**
- **S-B's failure mode (ii) cites "the adopted Tier-T option" legitimately.**
  U3's census block labels option A `(threshold, ADOPTED)`. The matrix CITES U2
  §5.3 and U3's Tier-T selection; it does not DECIDE either. **No cut-boundary
  breach found** — the matrix's subject sits inside U4 §8 as
  `section_owner_table.md` §1 draws it (`§8 | 1007–1286 | U4`), and its two S-E
  test rows are exactly two of the five §11 rows that table assigns to U4.
- **Flip clause 1's structural ground is true** (the halves are independent) and
  **flip clause 3's structural ground is true** (`playouts.rs` exists in both
  `tests/common/` trees, and `playout(seed, turns) -> GameState` is a real
  sampler). Both clauses fail on adequacy, not on reachability — F9 and F3.
- **Flip clause 5's self-declaration is honest.** Whether M1–M8 are the right
  eight mutations does change how strongly S-E is evidenced rather than which
  option is right, and I found no cell it moves. It is not hiding a trigger.

---

## 6. WHAT THE ARCHITECT IS LEFT WITH

Nothing in this matrix is selectable as written. The four missing rows — F5's
composite (S-E + reduced S-C, which is what the design actually says), F6's
calculus-lower-bound containment, F7's per-node census, F8's reuse of the landed
R1 referent — are untested and uncosted, and two of them (F6, F8) are immune to
the attack that kills S-E. Selecting from them would be selecting from rows no
matrix has stated and no red team has attacked, which is the breach D-305
measures at four in six for this work package. The proportionate move is a second
authoring round over an option set that includes them, not a selection from this
one.

---

*DECISION-RED-TEAM against `docs/experiments/matrix_M3_soundness_instrument.md`
at `f8e73e4` (matrix text identical at HEAD `dab170b`). Fresh context; not the
author. Verdict: every stated option falls. Nothing selected — selection is the
architect's.*
