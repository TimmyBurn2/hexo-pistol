# MATRIX M3 — the differential gate's instrument — **REVISION 2, AUTHORED, NOT SELECTED**

Status: **AUTHORED. AWAITS FRESH-CONTEXT DECISION-RED-TEAM.** Nothing below is
selected. Subject: the instrument of the DIFFERENTIAL GATE, the second of the
four named soundness gates (D-316). Owning unit:
`docs/experiments/U4_soundness_instrument.md` §8.

Revision 1 and its attack are `docs/experiments/matrix_M3_soundness_instrument.md`
and `docs/experiments/matrix_M3_REDTEAM.md`; the stop is D-317. Revision 1's body
is not edited by this file and is not re-attacked by it — every row it carries is
carried here with the verdict its own red team gave it.

## WHO AUTHORED THIS, AND WHY IT IS NOT THE PATTERN D-305 MEASURES

D-317 closes with: *"a revision 2 written by the session that wrote revision 1,
over options the red team supplied, is the pattern D-305 measures … It is the
architect's round to schedule and explicitly not this session's to author."*

Both conditions are met, and stating them is not a formality — it is the reason
this file may exist at all. **The session authoring this revision is not the
session that authored revision 1**; it read revision 1, its red team and D-317 as
inputs, and holds no position it authored. **The round is scheduled by the
architect**, ruling R8, which also fixes what the field must contain and how the
open theory is marked. The four rows the red team supplied are entered here as
ROWS — costed, given failure modes, and open to attack — and not as findings
promoted to conclusions.

## WHAT BINDS THIS ROUND

- **R7 — MEASURED cells carry their command and their output.** Every MEASURED
  claim below names a fact number, and that fact prints its command and its
  verbatim output IN THIS FILE, above the table. A red team re-runs every one. **A
  cell that does not reproduce kills the OPTION'S GROUND, not just the cell** —
  which is the disposition D-318 records after three MEASURED cells in this work
  package failed reproduction, all three favouring their author's recommendation.
  That record is the named prior for this round: *re-run everything*.
- **R8 — INCLUSION-MINIMALITY IS OPEN THEORY.** Any option whose criterion depends
  on it is marked **DEPENDS-OPEN-THEORY** and **cannot beat an immune option on
  that ground**. `docs/research/threat_calculus_v1.md` is NOT amended by this
  round; one errata ADR line records the question.
- The standing stop rule: if the attack kills every row, the matrix stops and the
  architect is told. No survivor is forced.

## THE STAGE UNDER DOUBT, named before the rows

**Does the staged generator ever drop a cell a proven tactic needs?**

Revision 1 named this and then scored its recommendation on a different defect.
Red team F6: ground 1 of revision 1 justified EQUALITY by mutation M3, which
**over-generates** — and over-generation is not a member of the named class. That
asymmetry is not repeated here. Every row is scored first on whether a generator
that DROPS a needed cell can falsify its criterion, and a row's strength against
over-generation is recorded separately and never used to choose between rows.

## THE OPEN THEORY, STATED PRECISELY — because revision 1's decisive kill was a
## convention question mistaken for a settled one

The canonical threat reference is `docs/research/threat_calculus_v1.md` (D-266).
What it does and does not say:

- `DEF-T` defines the threat NUMBER `t(F)` — *"exact **minimum hitting set** over
  plan family F (plans have size ≤2 ⇒ vertex-cover instance)"*. It is a
  **CARDINALITY**, the quantity tempo arithmetic consumes. It is not a statement
  about which covers a defender's candidate set must contain.
- `LAW-HIT` `[PROVEN]`: *"Defense against a plan family is exactly the hitting-set
  problem. Kill = hit; no other defensive mechanism exists."*
- `LAW-FORCE` `[PROVEN]`: *"If the opponent has ≥1 plan and the mover cannot win
  this turn, every non-losing mover move hits all opponent plans."*
- The word `inclusion` occurs **ZERO** times in the file (fact 4).

So the calculus fixes the hitting-set FRAME and a cardinality, and fixes **no
enumeration convention** for the set of covers a sound candidate generator must
carry. `cover.rs` enumerates the INCLUSION-MINIMAL covers; `DEF-T` counts the
MINIMUM-CARDINALITY ones; U4 §8.4's mutation M4 is literally *"minimum-cardinality
covers instead of inclusion-minimal"*, i.e. the ledger already treats the choice
as a live axis. **Which of the two a soundness criterion must assert is OPEN
THEORY, and R8 says so.** It is not a defect in `cover.rs` and this matrix does
not propose to change it.

Revision 1's red team called this the decisive attack on S-E and it stands. What
this revision adds is that the question has a SIZE, and the size is measured
(fact 7): the two conventions disagree on **28.0 %** of legally reachable FILTERED
side-positions. It is not a corner case, and an option resting on it is resting on
something live.

---

# FACTS — every one MEASURED HERE, with its command and its verbatim output (R7)

Measurements were taken over the tree at `4a23677`..`e3f0bc3` — docs-only commits,
no crate source changed between them (`git diff --stat 4a23677 e3f0bc3 -- crates/`
is empty). Numbers CARRIED from other documents are in a separate table at the end
of this block and are marked as carried, not as measured here.

**Fact 1 — the staged generator does not exist. Every cost below is priced before
any of it is built.** Revision 1 said "13 entries"; red team F12 found 14. MEASURED
here:

```
$ ls crates/pistol-search/src/ | tr '\n' ' '
candidates.rs error.rs fallback.rs info.rs lib.rs ordering.rs params.rs position.rs pv.rs pvs.rs score.rs search.rs stop.rs tt
```

14 entries, 13 files plus the `tt` directory. No `staged.rs`.

**Fact 2 — `pvs` is `pub(crate)`, so an integration test cannot see `Run`.**

```
$ grep -n "^pub mod \|^pub(crate) mod " crates/pistol-search/src/lib.rs
24:pub mod candidates;
25:pub mod error;
26:pub mod fallback;
27:pub mod info;
28:pub mod params;
29:pub mod score;
30:pub mod search;
31:pub mod stop;
32:pub mod tt;
37:pub(crate) mod ordering;
38:pub(crate) mod position;
39:pub(crate) mod pv;
40:pub(crate) mod pvs;
```

**Fact 3 — an always-on `assert!` in `pvs.rs` is NOT the house idiom. There is
exactly ONE, and revision 1's fact 5 said eight.** This is the cell red team F1
killed; it is re-measured here rather than restated:

```
$ awk '/#\[cfg\(test\)\]/{exit} /assert!/ && !/debug_assert!/ && !/^\s*\/\//{print NR": "$0}' \
    crates/pistol-search/src/pvs.rs
244:             assert!(
```

One, at line 244. Any ground of the form "the always-on assert is already the
idiom there" is void for this round.

**Fact 4 — the canonical reference never uses the operative word.**

```
$ grep -c 'inclusion' docs/research/threat_calculus_v1.md
0
$ grep -n '^| DEF-T' docs/research/threat_calculus_v1.md
30:| DEF-T | threat number t(F) | exact **minimum hitting set** over plan family F (plans have size ≤2 ⇒ vertex-cover instance). `[PROVEN]` |
```

**Fact 5 — AN INDEPENDENT COVER REFERENT IS ALREADY LANDED, AND ITS AGREEMENT
WITH THE SHIPPED QUERIES IS ALREADY A GATE.** `crates/pistol-solver/tests/common/
reference.rs` — 366 lines, D-68's pattern, third instance (D-106) — implements the
whole query surface by a different algorithm:

```
$ sed -n '223,226p' crates/pistol-solver/tests/common/reference.rs
    /// The inclusion-minimal covers of the attacker's hot windows, by the
    /// definition: every subset within budget that covers, minus every one with
    /// a proper subset that also covers.
    pub fn blocking_covers(&self, defender: Player, budget: HitBudget) -> Cover {
```

and `crates/pistol-solver/tests/threat_oracle_tests.rs` already asserts
`threats.blocking_covers(side, budget) == reference.blocking_covers(side, budget)`
at all three budgets, both sides, at every ply of a seeded playout regime:

```
$ cargo test -p pistol-solver --test threat_oracle_tests \
    threat_incremental_matches_reference_on_random_playouts -- --nocapture 2>&1 | tail -5
oracle census: 1703 plies, 805 hot side-positions, 205 unblockable, 266 cross-window
test threat_incremental_matches_reference_on_random_playouts ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 11.03s
```

**MEASURED: 1703 plies, 11.03 s, GREEN.** So the referent revision 1 priced as
new work exists, is reviewed, and is continuously checked against the subject.

**Fact 6 — that referent REACHES `pistol-search`'s test tree, and the cost of
reaching it is one `#[path]` line and one dev-dependency.** Measured in a
throwaway worktree on `/home` (`git worktree add --detach /home/tom/.cache/m3-author
HEAD`), never in the live tree; nothing from it is committed:

```
$ cat >> crates/pistol-search/Cargo.toml <<'EOF'
[dev-dependencies]
pistol-solver = { path = "../pistol-solver" }
EOF
$ cat crates/pistol-search/tests/m3_referent_reuse_probe.rs
#[path = "../../pistol-solver/tests/common/reference.rs"]
mod reference;
... let r = reference::Reference::from_board(&board);
    let cover = r.blocking_covers(Player::P2, HitBudget::Two);
$ cargo test -p pistol-search --test m3_referent_reuse_probe -- --nocapture 2>&1 | tail -6
PROBE cover = Minimal([Two { first: Coord { q: -1, r: 0 }, second: Coord { q: 5, r: 0 } }])
test the_landed_referent_answers_blocking_covers_from_this_crates_test_tree ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

**It compiles and answers.** The measured cost is three `dead_code` warnings —
the module's own `#![allow(dead_code)]` lives in `tests/common/mod.rs`, which a
`#[path]` include does not bring along, so the including test carries the allow
itself. That is the whole friction; it is not a redesign.

**Fact 7 — THE CONVENTION HAS A SIZE, AND SO DOES THE STRENGTH GAP BETWEEN THE
TWO CANDIDATE CRITERIA.** Same worktree, same seeded playout regime as fact 5
(seeds 1..=12, cap 150 plies). At every side-position the probe takes
`can_win_this_turn(defender, Two)` and `blocking_covers(defender, Two)`, calls a
node FILTERED when the first is `None` and the second is `Minimal` (U2 §5.3's
row), and forms both unions from the same `Cover`:

```
$ cargo test -p pistol-solver --test zz_m3_census -- --nocapture 2>&1 | tail -10
plies                       = 1703
side-positions              = 3406
FILTERED side-positions     = 300  (8.8 % of side-positions)
conventions DIFFER on       = 84  (28.0 % of FILTERED)
cells: inclusion-minimal    = 776 total, 2.587 per FILTERED node
cells: DEF-T minimum        = 608 total, 2.027 per FILTERED node
R1 referent DISAGREES with shipped on 0 of 3406 side-positions
D-124 pop() mutant: S-K (DEF-T containment) fires on 216 of 300 applicable FILTERED nodes (72.0 %)
D-124 pop() mutant: S-M (inclusion-minimal equality) fires on 300 of 300 (100.0 %)
```

Four numbers this matrix uses and one it does not:

1. **8.8 %** — the FILTERED row's share of side-positions in this regime. It is a
   population figure for THIS regime and is not the corpus figure U4 §8 carries;
   the two are not compared here and neither is derived from the other.
2. **28.0 %** — the two conventions give different unions on 84 of 300 FILTERED
   nodes. The open theory is live, not vestigial.
3. **776 vs 608 cells** — inclusion-minimal carries 0.56 more cells per FILTERED
   node than DEF-T's minimum, a **27.6 %** larger emitted set under the convention
   `cover.rs` uses.
4. **72.0 % against 100.0 %** — the strength gap, under a PROXY that must be read
   with its limit stated: the staged generator does not exist (fact 1), so the
   mutant is D-124's own reproducer shape (`if cells.len() > 1 { cells.pop(); }`)
   applied to the FILTERED row's COVER UNION rather than to a generator's output.
   **This measures the criteria, not the generator**, and a red team should treat
   it as an upper bound on what either criterion catches in the real thing.
   `pop_applicable` was 300 of 300 — every FILTERED node in the regime carried a
   union of more than one cell.
5. **0 of 3406** — the landed referent disagreed with the shipped queries nowhere
   in the regime. This is fact 5's gate restated as a number, and §"the weakest
   cell" below reads it in the direction that is against the recommendation, not
   for it.

**Fact 8 — `pistol-search` does not depend on `pistol-solver` today, and U2's IMPL
is the commit that creates the edge (travelling item T7).**

```
$ sed -n '/\[dependencies\]/,$p' crates/pistol-search/Cargo.toml | grep -v '^#'
[dependencies]
pistol-core = { workspace = true }
pistol-eval = { workspace = true }
```

Consequence for costing: at the time the differential gate is armed, that edge
exists, so fact 6's dev-dependency line is redundant with a dependency the WP
already lands. Before it, this matrix's rows are not buildable at all — which is
an ordering fact, not a cost.

**CARRIED NUMBERS — measured by other sessions, cited, NOT re-measured here.**

| Value | What it measures | Source |
|---|---|---|
| **17.89 s** | the REDUCED S-C mate-class regression, radius 1, three fixtures plus one built mate-in-3 | U4 §8.2 |
| **243 363 538 nodes / 554.2 s** | ONE of 31 positions at S-C's SUPERSEDED registered workload (radius 2, depth 3) | U4 §8.1 |
| **28 class assertions, 0 RED**; **0 of 62** forced-block cases; **3 of 7** mutations unable to fire | S-C's measured vacuity | U4 §8.1 |
| **ESTIMATED 40–90 s** | the whole soundness gate per CI run | U4 §13 |
| **3.1 % against 25.0 %** | revision 1's flip-clause-3 dilution | red team F3 |

---

# THE FIELD — thirteen rows

Nine from revision 1, carried with the verdict its red team gave them, and the
four D-317 names as missing. **R7 COMPLIANCE:** every MEASURED cell names the
fact whose command and verbatim output are printed above, in this file. A red team
re-running a cell re-runs the printed command; nothing points at a scratchpad.

## The nine carried rows

| Option | Criterion | Standing after round 1 | Theory |
|---|---|---|---|
| **S-A** value agreement | staged root value == full-width root value | **FELL.** Preserved by the defect class: interior narrowing that never moves a root maximum leaves the value identical | — |
| **S-B** containment vs the RADIUS set | staged ⊇ radius policy's set | **FELL** twice over: preserved by over-generation, and MEASURED false on a correct engine (U4 §8.3: 157 of 182 FILTERED descendants have a cover cell outside the radius-1 ball) | — |
| **S-C** class-restricted argmax agreement | root argmax on restricted classes | **FELL to measurement** — 28 class assertions 0 RED, the forced-block class empty at 0 of 62, `\|argmax\| = 1` making membership identity (D-119) | — |
| **S-D** S-C + mutation ledger | S-C's criterion, ledger as acceptance | **FELL with S-C.** A ledger measures a criterion's strength; it does not supply one | — |
| **S-E** per-node equality, two halves | emitted == union over inclusion-minimal covers, referent written from scratch; plus an always-on `assert!` in `visit` | **FELL on four independent attacks** (F1–F4): the convention tension, D-115 applied asymmetrically, "forced by the tree" false, and fact 5's own MEASURED number not reproducing (fact 3 here) | **DEPENDS-OPEN-THEORY** |
| **S-F** in-source `cfg(debug_assertions)` seam | same criterion, inside `pvs` | **FELL on landed lines** — D-129 forbids demoting a correctness invariant to `debug_assertions`; it would compile out of four release-driving `tools/` gates | **DEPENDS-OPEN-THEORY** |
| **S-G** S-E + sampled positions | S-E's criterion over playout-sampled nodes | **FALLS WITH S-E**, and is strictly downstream of whatever referent S-E lands | **DEPENDS-OPEN-THEORY** |
| **S-H** generator self-check | the generator asserts against the union it just computed | **FELL on CLAUDE.md's own clause** — the check and the suspect share their input | **DEPENDS-OPEN-THEORY** |
| **S-I** no differential gate (null row) | nothing | **REJECTED, and stated so it is checkable.** D-124's flip clause never fires and the class goes unmeasured. Kept because a matrix without its null row is F11's finding | — |

**None of the nine is revived by this revision**, and the four new rows below do
not inherit anything from them.

## The four rows D-317 names — costed and given failure modes for the first time

### S-J — the composite the design actually writes as adopted: S-E half one, with the REDUCED S-C beside it

| | |
|---|---|
| **Criterion** | S-E's per-node equality on the FILTERED row, PLUS the reduced S-C mate-class regression as a second, cheap, answer-level check |
| **Cost** | **CARRIED 17.89 s** for the reduced half (U4 §8.2, not re-measured here); the equality half's cost is S-E's, **ESTIMATED** because fact 1 |
| **Where it observes** | the emitted set per node, and root answers on a mate class |
| **Failure modes** | **It inherits every one of S-E's four kills** — the composite does not repair the convention tension, and pairing a wounded criterion with a cheap regression does not make the wounded half sound. Its ONE genuine contribution is accounting honesty: revision 1 priced "S-C" at the SUPERSEDED 554.2 s workload while the live proposal runs at 17.89 s, so the matrix killed a row the design had already replaced (red team F5). That correction is recorded; it is not a ground |
| **Theory** | **DEPENDS-OPEN-THEORY** (via S-E's half) |

### S-K — CALCULUS LOWER-BOUND CONTAINMENT, stated in `DEF-T`'s own convention

| | |
|---|---|
| **Criterion** | At every FILTERED node: the emitted set **CONTAINS** the union over the MINIMUM-CARDINALITY covers, computed by the independent referent R1 (fact 5), never by `cover.rs`. Containment, not equality: the gate says the generator did not drop what the calculus's own cardinality convention makes required, and says nothing about what else it kept |
| **Cost** | The referent is **MEASURED already landed and already gated** (fact 5: 1703 plies, 0 disagreements, 11.03 s) and **MEASURED reachable from the consuming crate for one `#[path]` line plus a dev-dependency the WP lands anyway** (facts 6, 8). What remains is the per-node walk, **ESTIMATED** and unmeasurable until fact 1 stops being true |
| **Strength, MEASURED and stated against itself** | fact 7: fires on **216 of 300** proxy-mutant instances, **72.0 %**, against S-M's 100 %. The 28 % it misses are exactly the nodes where the two conventions disagree — i.e. the cells whose requiredness is the open question. **It is the weaker criterion, and it is weaker precisely where the theory is open** |
| **Failure modes** | (i) A generator that keeps the minimum-cardinality cells and drops every other inclusion-minimal cover's cells passes. Under `LAW-FORCE` that generator still leaves a non-losing move available at the node; whether it costs STRENGTH is an SPRT question by hard rule 6, and this row says so rather than asserting it costs nothing. (ii) Containment is preserved by over-generation — the same vacuity S-B has one level down — so this row is blind to mutation M3's class and does not claim otherwise. (iii) The minimum-cardinality union is itself a CHOICE within the calculus's frame: `LAW-FORCE` alone does not entail that every minimum cover's cells are required, since two disjoint size-1 covers make either sufficient. **The row's honest claim is that it is stated in the canonical convention, not that it is entailed by it** |
| **Theory** | **IMMUNE to R8's open question.** Its criterion is stated in the convention `DEF-T` uses and never mentions inclusion-minimality. It is not immune to (iii) above, which is a different and smaller question and is named here rather than left for the attack |

### S-L — the per-node CENSUS: record and compare, assert only the immune bar

| | |
|---|---|
| **Criterion** | A recorder seam records, per node, the emitted set and both referent unions; the CI gate asserts only S-K's containment, and the inclusion-minimal DELTA is **RECORDED, not asserted**. The recording is what would settle R8's open theory with data instead of argument |
| **Cost** | A nullable recorder branch per node inside `Run` — **fact 2 makes this in-source**, since `pvs` is `pub(crate)`. Volume: **MEASURED at 2.587 cells per FILTERED node** over 300 nodes in fact 7's regime, which is the recording's shape, not its search-time volume; the search-time volume is **ESTIMATED** and unmeasurable until fact 1 stops being true |
| **Failure modes** | (i) A recorder that fires only in instrument mode records a population that is not the shipped one, and one that fires always is a per-node branch in the hot path — the matrix does not know which, and that is a real open cost, not a rhetorical one. (ii) **A census is not a gate.** Recording a delta nobody has registered a criterion over is precisely the "criterion nothing can fail" CLAUDE.md forbids — so this row is only ever S-K PLUS a record, never a record alone. (iii) It needs the determinism law honoured on the recording path (rule 4) |
| **Theory** | **IMMUNE** — its asserted half is S-K's. What it adds about the open convention it records rather than asserts |

### S-M — EQUALITY against the LANDED referent R1

| | |
|---|---|
| **Criterion** | At every FILTERED node the emitted set **EQUALS** R1's inclusion-minimal cover union — S-E's criterion, with the referent question answered by reuse rather than by a promise |
| **Cost** | **MEASURED cheapest of the observing rows**: the referent exists (fact 5), reaches the consuming crate for one `#[path]` line (fact 6), needs no new arithmetic and no `pub` widening of `staged` for its referent (the emitted-set seam is a separate question S-L's cost cell prices) |
| **Strength** | **MEASURED 100 % on fact 7's proxy mutant, 300 of 300** — it catches every instance S-K catches and the 84 it does not |
| **Failure modes** | (i) **It asserts the convention R8 declares open**, so a decision that the calculus's convention is the right one turns this gate RED on a correct engine — the same tension that killed S-E, moved but not removed. (ii) **R1 AND `cover.rs` ARE BLIND TO THAT QUESTION TOGETHER, AND THE MEASUREMENT SHOWS IT**: fact 7's `0 of 3406` is what shared blindness looks like from inside. CLAUDE.md: *"two instruments blind to the same stage are one instrument reported twice, and their agreement is invariant under a defect in what they are both blind to."* R1 is independent of `cover.rs`'s CODE and shares its CONVENTION, so it is a real oracle for the arithmetic and not an oracle at all for the convention. (iii) It remains a strictly stronger criterion than S-K wherever the convention is not the question, which is 72 % of the proxy instances |
| **Theory** | **DEPENDS-OPEN-THEORY.** Immune to the INDEPENDENCE prong of the attack that killed S-E — R1 predates this matrix, was written by a different algorithm and is itself gated — and NOT immune to the CONVENTION prong, which is the prong R8 names. D-317 records these four rows as "two of them immune"; **that reading is corrected here for this row, against the recommendation's interest**, because the row's independence and its convention are separate questions and only the first is settled |

---

# RECOMMENDATION

**S-K.** Containment against the minimum-cardinality union, computed by the landed
referent R1.

Grounds, in the order they bind. Each is differential — it separates S-K from a
named rival — and each names the fact carrying its evidence.

1. **It is the only row that both observes the named defect and rests on no open
   question.** S-A, S-B, S-C, S-D, S-H and S-I fail the first half on round 1's
   own attack; S-E, S-F, S-G, S-J and S-M satisfy it while resting on the second.
   R8 is explicit that a DEPENDS-OPEN-THEORY row cannot beat an immune row on that
   ground, and S-L's asserted half IS S-K, which leaves S-K and S-L as the field
   this ground admits.
2. **Against S-L, and this is the only place the two separate: cost, and it is a
   cost nobody has measured.** S-L's recorder lives inside `Run` (fact 2), on the
   search's per-node path, and fact 1 makes its search-time volume unmeasurable
   today. S-K needs no seam inside `pvs` at all. **S-L is not rejected — it is
   SEQUENCED**: it is S-K plus a record, and the record is the instrument that
   would settle R8's question. The flip clause below promotes it the moment its
   cost is measurable.
3. **Against S-M, on the ground R8 fixes and with the number that shows it is not
   academic.** S-M's extra catching power is 100 % against 72 % (fact 7), and the
   28 percentage points it adds are precisely the nodes where the two conventions
   disagree (84 of 300, fact 7). So S-M's advantage over S-K is exactly co-extensive
   with the open question. That is the definition of "beating an immune option on
   that ground", and R8 forbids it.
4. **Its referent is landed, reviewed, and continuously checked — measured, not
   promised.** Revision 1's weakest cell was that nothing enforces the referent's
   independence. Facts 5 and 6 replace that promise with a file: 366 lines, a
   different enumeration algorithm, agreement with the shipped queries asserted at
   every ply of a seeded regime (0 of 3406), and reachable from the consuming
   crate for one `#[path]` line. **This ground belongs to S-K, S-L and S-M
   equally** and is stated here because it is what makes any of the three cheap —
   it is not used to separate them.

## THE WEAKEST CELL, named so the red team starts there

**S-K is the weaker instrument, and ground 3 is doing work that a measurement
could overturn.** Its 72 % is a proxy figure (fact 7's own limit statement): the
mutant is applied to a cover union because the generator does not exist. If the
real staged generator's drops land preferentially on cells outside the
minimum-cardinality union — which is not absurd, since ordering pushes them
last — S-K's real-world catch rate is BELOW 72 % and could approach zero, while
S-M's stays at 100 %. **Nothing in this matrix measures that, and nothing can
until fact 1 stops being true.**

Second weakest: ground 3 treats R8's ruling as decisive between S-K and S-M. R8
forbids a dependent row beating an immune one ON THAT GROUND; it does not say an
immune row wins on every ground. A red team that shows S-M beats S-K on a ground
independent of the convention — CI cost, implementation risk, coverage of a defect
class S-K is blind to — is attacking something this recommendation does not
defend, and the honest answer would be that the two are separated only by R8.

## WHAT FLIPS IT

- **The convention question is SETTLED toward inclusion-minimality** — by an
  amendment to `threat_calculus_v1.md` under D-266's own amendment rule, or by an
  ADR line that adopts `cover.rs`'s convention as the project's. Remedy: **flip to
  S-M**, whose criterion is then immune too and strictly stronger (100 % against
  72 %, fact 7). Reachable: S-M's referent IS S-K's referent, so the flip changes
  one comparison operator and no infrastructure.
- **The recorder's per-node cost measures out negligible once the generator
  exists.** Remedy: **flip to S-L** — S-K's assertion, plus the record. Reachable
  by construction, since S-L's asserted half is this option.
- **S-K's real-world catch rate measures materially below its 72 % proxy.**
  Trigger: once `staged.rs` exists, the same mutant applied to the GENERATOR's
  output rather than to the cover union. Remedy: **the gate takes S-M's criterion
  and the convention question moves to an ADR line that must be answered before
  the gate is armed** — i.e. the flip forces the theory to be settled rather than
  weakening the gate.
- **The FILTERED row is too rare in the GATE's corpus to gate anything.** Trigger:
  a corpus-side count, the analogue of fact 7's 8.8 % regime figure, measured
  against the gate corpus rather than against playouts. Remedy: **add sampled
  legal positions** — the S-G shape, which is orthogonal to which criterion is
  asserted and applies to S-K exactly as it applied to S-E.
- **NOT a flip trigger, stated so it is not mistaken for one:** the mutation
  ledger's composition. It changes how strongly a criterion is EVIDENCED, not
  which criterion is right. It moves no cell here.

## WHAT THIS MATRIX DOES NOT DECIDE

- **It does not amend the calculus** (R8). The errata ADR line records the
  question; `docs/research/threat_calculus_v1.md` is untouched by this round.
- **It does not settle whether the emitted-set seam is `pub`, `#[cfg(test)]`, or a
  recorder.** Every row here needs to observe the emitted set somehow, and D-115's
  constraint on widening `pistol_search::staged` — red team F4 — applies to
  whichever row is selected. That is a separate named decision and it is not
  smuggled into this one.
- **It does not price the gate per CI run.** The 40–90 s figure is CARRIED from
  U4 §13 and is not this matrix's measurement.

## COST OF THE DECISION THIS MATRIX FEEDS

Selecting costs one fresh-context DECISION-RED-TEAM dispatch — one subagent, no
machine time beyond re-running the eight facts above, which together take under a
minute of CPU (fact 5's 11.03 s is the longest single command; fact 7's probe runs
in 4.52 s). **The matrix's own governed run is that re-run, and it is cheap enough
that doubt about any cell is answered by REPLICATION rather than by argument.**
IMPLEMENTING the selected row costs, all **ESTIMATED** because fact 1: one
observation seam for the emitted set, one `#[path]` include plus the `[dev-dependencies]`
line (fact 6, subsumed by fact 8's edge once U2's IMPL lands), one integration test,
and the `tools/staged_soundness_check.sh` wiring — a `tools/` change carrying a
SHELL_CHECKLIST review answered item by item and the coverage rule's test driving
the shipped script.

---

*Matrix M3, revision 2. Authored by a session that did not author revision 1,
scheduled by the architect under R8. Thirteen rows: nine carried with their
round-1 verdicts, four entered for the first time. NOT SELECTED. Awaits
fresh-context DECISION-RED-TEAM.*
