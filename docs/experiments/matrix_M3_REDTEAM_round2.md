# DECISION-RED-TEAM — MATRIX M3, revision 2, the differential gate's instrument

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
red team. The dispatching session authored the matrix; it did not attack it.
F3's correction to D-321 was independently re-run by the dispatching session
before it was acted on — 174 / 22 / 12.6 % / 85.7 % reproduce — and the ADR
amendment it calls for is landed as its own line.
-->

**Pinned revision:** `d48824f`.
**Matches HEAD:** **NO, at the time of writing.** HEAD was `d48824f` when I started and is `c15529c` now — four commits landed on `dev` during the round (`1d5af10`, `9ce863f`, `0af32fb`, `c15529c`). `git diff --stat d48824f c15529c -- crates/ tools/ configs/` is **empty**, and `docs/experiments/matrix_M3_soundness_instrument_rev2.md` is not among the changed files, so the matrix under attack is unchanged and every measurement below stands. One of those commits matters to this report and is treated as a finding: `0af32fb` landed **D-321**, which adopts fact 7's numbers into the ADR log.

**Fresh context:** I did not author revision 1, revision 2, the round-1 red team, U4, or any ADR line cited here. I read them as inputs. I repaired nothing; every file I wrote lived in a throwaway worktree at `/home/tom/.cache/m3-rt`, now removed. Final `git status --porcelain` in the live tree: **empty**. `git worktree list` no longer contains `m3-rt`.

---

## VERDICT

**All eight facts reproduce.** This is the round's first result and it matters, because D-318's named prior said to expect otherwise. Facts 1, 2, 3, 4, 8 reproduce verbatim; fact 5 reproduces (1703 plies, GREEN, 10.92 s against a claimed 11.03 s); fact 6 rebuilds and answers; and **fact 7 reproduces every printed digit from a probe I wrote myself without seeing the author's** — 1703 / 3406 / 300 / 8.8 % / 84 / 28.0 % / 776 / 2.587 / 608 / 2.027 / 0 of 3406 / 216 of 300 / 72.0 % / 300 of 300 / 100.0 %. The measurement record of this work package is not the pattern D-318 records. It is clean.

**The matrix falls anyway, and it falls on what the numbers MEAN rather than on whether they reproduce.**

| Option | Survival |
|---|---|
| **S-A, S-B, S-C, S-D, S-E, S-F, S-G, S-H, S-I** | **FELL** in round 1; carried, not revived, not re-attacked by me |
| **S-J** | **FALLS** — carried inheritance of S-E's four kills, unchallenged |
| **S-K** (recommended) | **FALLS.** Grounds 1, 2 and 3 all fall, and the row is measured to kill **neither** registered mutation of the class it instruments |
| **S-L** | **FALLS with S-K** — its asserted half IS S-K, so it inherits S-K's ledger vacuity; its recorded half is by its own cell (ii) not a gate |
| **S-M** | **SURVIVES, WOUNDED** — the only row left standing |
| **MISSING ROW S-N** | a rules-derived criterion, immune to R8 *by construction*, excluded by the field's framing |

**Not every option falls, so the "every row died" stop does not apply.** But the field is still incomplete at revision 2 — D-318's third reason, recurring — and the missing row dominates the survivor on the very axis R8 governs. Selection is the architect's; I do not make one.

### The strongest surviving attack on S-M, stated so an ADR line can quote it

> S-M asserts the convention **D-321** records as OPEN: if the project settles toward `DEF-T`'s minimum-cardinality reading, the gate turns RED on a correct engine, and its referent cannot warn of it — R1 is independent of `cover.rs`'s CODE and shares its CONVENTION, so fact 7's `0 of 3406` agreement is invariant under precisely the defect in question, which is CLAUDE.md's two-instruments-blind-to-the-same-stage clause read from inside. What survives the attack is that its advantage over the immune-marked S-K is **not** confined to that open question: MEASURED, S-M kills both registered S-E-class mutations — M4 on its own pinned witness, and M3 on 20 of 20 differing nodes of the registered playout regime — where S-K kills neither, so R8 does not decide between them.

---

## FINDINGS

### F1 — **KILL** (ground 3, and the recommendation). Fact 7's proxy mutant is registered in the owning unit's own ledger as belonging to the OTHER half of the instrument, and U4 §8.2 says in terms that no criterion of this kind can observe it.

**Claim attacked.** Ground 3: *"S-M's extra catching power is 100 % against 72 % (fact 7) … That is the definition of 'beating an immune option on that ground', and R8 forbids it."* And the matrix's own framing promise: *"Every row is scored first on whether a generator that DROPS a needed cell can falsify its criterion."*

**Contradicting evidence.** The mutant fact 7 uses is `if cells.len() > 1 { cells.pop(); }`. U4 §8.4 registers that exact mutant as **M8**, and assigns it a class:

```
$ sed -n '395,414p' docs/experiments/U4_soundness_instrument.md | grep -o 'M8.*the `assert!`'
```
> `| **M8** | **`visit` drops the last candidate after generation** — D-124's own reproducer, `if cells.len() > 1 { cells.pop(); }` | **the `assert!`** |`

Its class is **the `assert!`**, not S-E. The owning unit says why, verbatim (U4 §8.2, line 240):

> *"The test above sees what the generator EMITS; D-124's reproducer (`cells.pop()` after `order`) is a drop AFTER generation, **which no test of the generator can see**."*

And ground 2 of this very recommendation buys S-K's cheapness by giving up exactly that observation:

> *"S-K needs no seam inside `pvs` at all."*

**So the two load-bearing grounds are inconsistent.** Ground 2 wins the field-of-two by placing S-K outside `pvs`; ground 3 then scores S-K on a mutant that only something inside `pvs` can see. Under ground 2's costing, S-K's real firing rate against M8 is **0 %**, not 72 %; the 72 % is an abstract property of a set operation performed on a cover union, and the matrix's fact-7 note ("this measures the criteria, not the generator") concedes the mechanism without drawing the consequence for the ground that rests on it.

This is round 1's F6 recurring in the revision written to not repeat it: F6 killed ground 1 of revision 1 for justifying its criterion with a defect outside the named class, and the "STAGE UNDER DOUBT" section of revision 2 opens by promising *"That asymmetry is not repeated here."*

**Classification: KILL** on ground 3, and on ground 2's compatibility with it.

---

### F2 — **KILL** (S-K, and ground 3's differential claim). S-K kills NEITHER registered mutation of the class it instruments; S-M kills both; and one of them is not the convention question.

**Claim attacked.** Ground 3: *"the 28 percentage points it adds are precisely the nodes where the two conventions disagree … So S-M's advantage over S-K is **exactly co-extensive with the open question**."* And the row's own theory cell: *"IMMUNE to R8's open question."*

**Contradicting evidence.** U4 §8.4 registers exactly two mutations of class **S-E** — the differential gate's class:

- **M3** — "The FILTERED row emits `Cover::cells()` flattened at phase 0 and does not regenerate at phase 1". A **phase-regeneration** defect. It is not the convention.
- **M4** — "Minimum-cardinality covers instead of inclusion-minimal", with a witness *rebuilt at u-rev 3 under MAJOR 8 as a position a legal game reaches*, pinned in `crates/pistol-solver/tests/wp15b_mutation_witnesses.rs`.

**Reproducer** (`crates/pistol-solver/tests/zz_m3_mutants.rs`, written by me, run in the worktree at `d48824f`):

```
$ cargo test -p pistol-solver --test zz_m3_mutants -- --nocapture
running 2 tests
M4 WITNESS  cover            = Minimal([One(Coord { q: -1, r: 0 }), Two { first: Coord { q: -1, r: 5 }, second: Coord { q: 4, r: 0 } }])
M4 WITNESS  correct emitted  = [Coord { q: -1, r: 0 }, Coord { q: -1, r: 5 }, Coord { q: 4, r: 0 }]
M4 WITNESS  DEF-T min union  = [Coord { q: -1, r: 0 }]
M4 WITNESS  M4 mutant emits  = [Coord { q: -1, r: 0 }]
M4 WITNESS  S-K (containment vs DEF-T min) => GREEN — MUTANT SURVIVES
M4 WITNESS  S-M (equality vs inclusion-min) => RED — MUTANT DIES
test m4_survives_s_k_and_dies_to_s_m ... ok
M3 REGIME  phase-1 FILTERED nodes        = 95
M3 REGIME  nodes where the mutant DIFFERS = 20
M3 REGIME  S-M fires (RED) on 20 of 20 differing nodes (100.0 %)
M3 REGIME  S-K fires (RED) on 0 of 20 differing nodes (0.0 %)
test m3_over_the_playout_regime_survives_s_k_far_more_often_than_s_m ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

Two consequences, each sufficient.

**(a) Ground 3's differential claim is false.** M3 is a phase-regeneration bug, not a convention. S-M catches it 100 %; S-K catches it 0 %. R8 shields S-K only on the convention ground, so **S-M beats S-K on a ground R8 does not reach** — which is precisely what the matrix's own "second weakest cell" paragraph nominates as the attack the recommendation does not defend: *"coverage of a defect class S-K is blind to."* The matrix names the attack, concedes it in failure mode (ii) — *"this row is blind to mutation M3's class and does not claim otherwise"* — and then ground 3 asserts the opposite of that concession three paragraphs later.

**(b) S-K is vacuous against its own ledger, by the standard that killed S-C.** U4 §8.1's kill of S-C was, in part, *"M3 and M4 provably cannot fire … Three of seven mutations are killable."* Under S-K, M3 and M4 provably cannot fire **again** — and those two are the *entire* S-E class. The instrument for the differential gate would fire on zero of the differential gate's registered mutations. The matrix pre-empts this in "WHAT FLIPS IT": *"NOT a flip trigger … the mutation ledger's composition. It changes how strongly a criterion is EVIDENCED, not which criterion is right."* That is the same asymmetric application round 1's F4 found (D-115 used to kill S-F and not applied to S-E): a ledger that could not fire was a measured kill against S-C and is declared inadmissible for S-K, in the same document.

**Classification: KILL** on S-K, and on ground 3.

---

### F3 — **KILL** (fact 7's attribution) / and it has already propagated into a landed ADR line.

**Claim attacked.** Fact 7: *"calls a node FILTERED when the first is `None` and the second is `Minimal` (**U2 §5.3's row**)"*, and the headline that follows: *"the two conventions disagree on **28.0 %** of legally reachable FILTERED side-positions. It is not a corner case."*

**Contradicting evidence.** U2 §5.3's row is not that predicate. U2 states the protocol as code (lines 234–239):

```
$ sed -n '233,240p' docs/experiments/U2_node_protocol.md
let left = StonesLeft::from_state(state)
    .unwrap_or_else(|| panic!("{OVERLOAD_ON_A_DECIDED_POSITION}: …"));
match threats.can_win_this_turn(us, left) {
    Some(_) => WIN-NOW ROW,
    None => match threats.blocking_covers(us, HitBudget::from(left)) {
        Cover::NothingToBlock   => BATCHED ROW,
        Cover::Minimal(covers)  => FILTERED ROW,
```

The row is evaluated **once per node, for `us` — the side to move — at the phase-derived `left`**, with `HitBudget::from(left)` (`query.rs:79–86`: `One → One`, `Two → Two`). Fact 7's probe instead evaluates **both sides at every ply** (its own `side-positions = 3406 = 2 × 1703` proves this) and fixes `left = Two`, `budget = Two` regardless of phase. Roughly half its population is a side that is not to move, and its phase-`Second` entries are queried at the wrong stones-left and the wrong budget.

**Reproducer.** My census (`zz_m3_census.rs`) computes both populations in one run:

```
$ cargo test -p pistol-solver --test zz_m3_census -- --nocapture
plies                       = 1703
side-positions              = 3406
FILTERED side-positions     = 300  (8.8 % of side-positions)
conventions DIFFER on       = 84  (28.0 % of FILTERED)
cells: inclusion-minimal    = 776 total, 2.587 per FILTERED node
cells: DEF-T minimum        = 608 total, 2.027 per FILTERED node
R1 referent DISAGREES with shipped on 0 of 3406 side-positions
pop_applicable              = 300 of 300
D-124 pop() mutant [pop LAST]:  S-K fires on 216 of 300 (72.0 %)
D-124 pop() mutant [pop LAST]:  S-M fires on 300 of 300 (100.0 %)
PHASE-CORRECT pop-mutant: S-K fires 132 of 154 (85.7 %), S-M fires 154 of 154 (100.0 %)
PHASE-CORRECT U2 5.3 row (mover only, left/budget by phase): FILTERED = 174 of 1703 mover positions, DIFFER = 22 (12.6 %)
```

On U2 §5.3's actual row the open question's measured size is **12.6 %, not 28.0 %** — less than half — and S-K's proxy rate is **85.7 %, not 72.0 %**. The distortion is not one-directional, which is why I record this as an attribution defect and not as authorial bias: it inflates the number the matrix uses rhetorically and deflates the number the matrix uses against itself.

**This has already left the matrix.** `0af32fb` landed **D-321**, which writes into the append-only ADR log: *"the two conventions give different unions on **84 of 300** legally reachable FILTERED side-positions — **28.0 %** — carrying 2.587 cells per node against `DEF-T`'s 2.027. It is live, not vestigial."* The ADR line now carries a figure attributed to a row it was not measured on. The direction of the erratum does not change (12.6 % is still live, not vestigial), but the number and its attribution do.

**Classification: KILL** on the attribution, **and an owed correction to D-321** — which is the architect's, not mine.

---

### F4 — **WOUND** (S-K's IMMUNE mark). The mark rests on a cardinality→required-set step the row's own cell (iii) concedes is not entailed, and (iii)'s verbatim scenario is the majority case.

**Claim attacked.** *"**IMMUNE to R8's open question.** Its criterion is stated in the convention `DEF-T` uses … It is not immune to (iii) above, which is a **different and smaller** question."*

**Contradicting evidence.** `DEF-T` defines a **NUMBER**: *"threat number t(F) | exact minimum hitting set over plan family F"* (fact 4, reproduced). S-K asserts a **SET**: that the union over *all* minimum-cardinality covers is required. Cell (iii) concedes the step is unlicensed — *"`LAW-FORCE` alone does not entail that every minimum cover's cells are required, since two disjoint size-1 covers make either sufficient"* — and then the theory cell calls the residue "smaller" without measuring it. It is measurable in seconds, which D-291 makes a finding:

```
>=2 ONE-covers (failure mode iii verbatim: two disjoint size-1 covers) = 156 of 300 FILTERED, 116 of 174 phase-correct
>=2 minimum-cardinality covers (failure mode iii is LIVE): 216 of 300 FILTERED [matrix population], 132 of 174 [phase-correct]
```

(iii)'s scenario **as worded** holds on **156 of 300 (52.0 %)** of the matrix's own population and **116 of 174 (66.7 %)** of U2 §5.3's. Generalised to any minimum size it is **216 of 300 (72.0 %)** and **132 of 174 (75.9 %)**. So on a majority of FILTERED nodes S-K asserts more than `DEF-T` says. That is a second convention question — "must a generator carry every minimum cover, or one?" — unmarked, unmeasured in the matrix, and load-bearing for the IMMUNE mark that ground 1 uses to reduce the field to two.

I could not turn this into a RED-on-a-correct-engine reproducer under the U2 §5.3 protocol, and I say so: every minimum-cardinality cover is inclusion-minimal, so `U_MIN ⊆ U_IM` holds by construction and S-K is green on any engine implementing §5.3. The wound is on the *mark*, not on the criterion's correctness.

**Classification: WOUND.** S-K is not immune; it is DEPENDS-OPEN-THEORY on a question the matrix names and declines to size.

---

### F5 — **MISSING ROW.** A criterion on the emitted set derived from the RULES, in which the referent question never arises — excluded by the field's framing for two revisions running.

**Claim attacked.** *"THE FIELD — thirteen rows"*, and ground 1's *"which leaves S-K and S-L as the field this ground admits."*

**Contradicting evidence.** Every one of the thirteen rows either compares the emitted set to a **cover computation** (S-B, S-E, S-F, S-G, S-H, S-J, S-K, S-L, S-M), or is a **root-answer** oracle (S-A, S-C, S-D), or is the null row (S-I). The framing presupposes that an instrument for "does the generator drop a cell a proven tactic needs?" must compute covers — which is what makes R8 dispositive at all. It need not. `LAW-HIT` and `LAW-FORCE` are about *survival*, and survival is decidable from `pistol-core` alone: `movegen::generate_turns` plus `GameState::outcome`. A criterion phrased over non-losing turns mentions no hitting set, no inclusion-minimality, no minimum cardinality, and is therefore **immune to R8 by construction** rather than by picking a side of it.

**Reproducer** (`crates/pistol-solver/tests/zz_m3_missing_row.rs`), on M4's own pinned, legally reachable witness:

```
$ cargo test -p pistol-solver --test zz_m3_missing_row -- --nocapture
M4 mutant emits            = [Coord { q: -1, r: 0 }]
dropped survival move      = [(-1,5), (4,0)]
after it, P1 has a winning turn (RULES-ONLY, movegen + outcome) = false
=> it is a NON-LOSING move whose cells the mutant never emits
S-K verdict on the mutant  = GREEN (containment vs [Coord { q: -1, r: 0 }] holds)
RULES-ONLY row verdict     = RED (a non-losing move's cells are missing)
test the_rules_only_survival_criterion_kills_m4_which_s_k_lets_live ... ok

FILTERED nodes (phase-correct) sampled = 79
mean legal TURNS per FILTERED node     = 1760141
max legal TURNS at one FILTERED node   = 2614337
test cost_of_the_rules_only_row_at_filtered_nodes ... ok
```

The row is expensive — **MEASURED 1.76 M legal turns per FILTERED node, mean, max 2.61 M** — which makes it a sampled or fixture-bound row, not a per-node one. That is a cost cell, and costing a row is what a matrix does; it is not a reason to have no row. I state the criterion's exact wording as **owed to the authoring round and not supplied here**: I did not repair the matrix, and the naive form ("emitted ⊇ every cell in some non-losing turn") is RED on a correct engine, because a non-losing pair may carry a free second stone outside any cover.

What makes this the D-318 shape rather than a wishlist item: the row would be **immune where S-K is only nominally immune**, and it **kills M4**, which S-K does not. A field that admits only S-K and S-L "on that ground" reaches that conclusion by presupposition.

**Classification: MISSING ROW.**

---

### F6 — **WOUND.** The matrix's flagged candour is the move that wins the recommendation, and its "weakest cell" names a softer hole than the one that is there.

**Claim attacked.** S-M's theory cell: *"D-317 records these four rows as 'two of them immune'; **that reading is corrected here for this row, against the recommendation's interest**."*

**Contradicting evidence.** Marking S-M **DEPENDS-OPEN-THEORY** is not against the recommendation's interest — it is the *sole* mechanism by which R8 excludes S-M. Ground 3 says so in its own words: *"That is the definition of 'beating an immune option on that ground', and R8 forbids it."* Had S-M been marked immune, ground 3 would have nothing to say and the 100 %-versus-72 % comparison would decide against S-K. The one classification the recommendation cannot survive without is presented as a concession made at its own expense.

The same shape governs "THE WEAKEST CELL". It names the 72 % proxy's *upper-bound* character and says S-K's real rate "could approach zero" — while the measurable hole beside it is that S-K's rate against the two registered mutations of its own class **is** zero (F2), and that the proxy is registered to the other half of the instrument (F1). Naming the softer version and declaring it unmeasurable — *"nothing in this matrix measures that, and nothing can until fact 1 stops being true"* — is inoculation in effect, because both harder versions were measurable at `d48824f` and one of them takes 0.04 s.

**Classification: WOUND** on the recommendation's self-criticism.

---

### F7 — **WOUND** (R7 / the instrument-revision clause), substance intact.

Fact 7's artefact — `cargo test -p pistol-solver --test zz_m3_census` — **does not exist in the tree at any revision**, and the matrix prints its command and its output but **not its source**, and names no revision for it. CLAUDE.md: *"An artefact that produces a registered number — a `tools/` script, **a scratchpad harness**, or a command block the document prints — is named in the pre-registration WITH ITS REVISION."* Fact 6 at least prints an elided probe body; fact 7, which carries the four numbers ground 3 and D-321 rest on, prints only prose.

I record this as a wound and not a kill because I discharged R7's purpose the hard way: I reconstructed the probe from the prose, and **every printed digit reproduced**. The prose was a sufficient specification of the arithmetic. It was *not* a sufficient specification of the population, which is F3 — and a printed source would have made F3 visible on the document's face in one reading.

**Classification: WOUND.**

---

## WHAT I COULD NOT BREAK — recorded so it is not re-attacked

- **All eight facts reproduce.** 1, 2, 3, 4, 8 verbatim in the live tree at `d48824f`. Fact 3's `awk` predicate is sound: `pvs.rs` has exactly one `#[cfg(test)]`, at line 441 of 552, and the only always-on `assert!` outside it is at 244.
- **Fact 5.** `1703 plies, 805 hot side-positions, 205 unblockable, 266 cross-window`, GREEN, **10.92 s** against the claimed 11.03 s. `reference.rs` is **366 lines**, shares only `pistol-core`, and `threat_oracle_tests.rs:196` does assert `threats.blocking_covers == reference.blocking_covers` at all three budgets, both sides, every ply. The referent's independence-of-code claim is real.
- **Fact 6.** Rebuilt from scratch: the `[dev-dependencies]` line plus a `#[path]` include compiles and answers, with **3 `dead_code` warnings** exactly as claimed.
- **Fact 7's arithmetic**, in full, from an independently written probe. Including `0 of 3406`. Including that S-K's 84 misses are *exactly* the 84 convention-disagreement nodes — that is structural, not coincidence, and I verified it is robust to which cell the mutant drops (removing the FIRST cell instead of the last also gives 216 of 300; removing each cell in turn gives 608 of 776 = 78.4 %, which is the identity `|U_MIN| / |U_IM|`).
- **Item 6, unattacked precedent (D-288's class): NOTHING FOUND.** I checked every ADR the matrix cites. D-115 and D-129 are landed operator rulings, D-119 a specified oracle contract, D-124 *"verified with a reproducer before being written down (D-116)"*, D-266 operator-supplied, D-295 and D-305 themselves red-team findings, D-316 a recorded repair. The matrix's grounds rest on architect ruling R8, on facts it measures, and on round-1 verdicts a fresh-context red team produced. No ground rests on an unattacked selection.
- **Item 7, carried numbers: all five check out and say what the matrix says.** 17.89 s at U4 §8.2:267 ✓. 243 363 538 nodes / 554.2 s at U4 §8.1:174–175, *"one position of thirty-one"* ✓. 28 assertions / 0 RED, 0 of 62, 3-of-7 at U4 §8.1 ✓ (§8.1's phrasing "three of seven mutations are killable" is loose, but its enumeration — M3, M4, M6 — is the three the matrix carries). 3.1 %/25.0 % at REDTEAM F3:68 and 258 ✓. **ESTIMATED 40–90 s** exists at U4:673, in U4-M's Cost table — the matrix cites it as "U4 §13", a section the carve retired; U4 itself carries the same stale pointer at its line 246, so the matrix inherited it rather than invented it. Not a finding.
- **S-K is not RED on a correct engine.** I tried to build one and could not. Every minimum-cardinality cover is inclusion-minimal, so `U_MIN ⊆ U_IM` holds by construction under U2 §5.3. S-K's greenness is safe; its emptiness is the problem.
- **S-M's failure mode (ii) is honestly stated and I could not soften it.** R1 and `cover.rs` genuinely are blind to the convention together, and `0 of 3406` genuinely is what that looks like from inside.
- **The authorship clause.** Revision 2 is by a different session, scheduled under R8. I found nothing suggesting otherwise.

**REJECTED findings, with the attempted reproducer:** I tried to show that fact 7's 72 % is an artefact of the union's sort order (`pop()` takes the lexicographic maximum). It is not — removing the first cell, the last cell, or averaging over every cell all give the same 216 of 300 for the agreement partition, because a cell that hits every hot window lies in every hot window and is therefore not lexicographically extremal. Reproducer: the `VARIANT [remove FIRST]` and `VARIANT [remove EACH cell]` lines above. **Recorded as rejected.**

---

## WHAT THE ARCHITECT IS LEFT WITH

1. **S-K, S-L and the recommendation fall.** Not on a non-reproducing cell — every cell reproduced — but on three things the reproducing cells were used to conclude. Ground 3 scores the criteria on a mutant the ledger registers to the other half of the instrument (F1) and that ground 2's own costing makes S-K blind to; the row kills neither registered mutation of its own class (F2); and its IMMUNE mark rests on a step its cell (iii) concedes is not entailed and that is live on a majority of its own population (F4).

2. **S-M is the only row standing**, wounded exactly where its cell (i) says it is. R8 does not exclude it, because R8 bars a dependent row from beating an immune one *on the convention ground*, and S-M's measured advantage is not confined to that ground — M3 is a phase-regeneration defect and S-M kills it 20 of 20 where S-K kills it 0 of 20.

3. **The field is still incomplete at revision 2** — a fourteenth row, rules-derived and immune to R8 by construction rather than by taking a side in it, is excluded by the framing that every instrument computes covers. This is D-318's third reason recurring in a different matrix. Whether that is enough to stop a second matrix in this work package before selection is a judgement I do not have standing to make; I record that the missing row **kills M4, which the recommended row does not**, and that its measured cost (1.76 M legal turns per FILTERED node) makes it a sampled row rather than an impossible one.

4. **D-321 is owed a correction.** It landed during this round, at `0af32fb`, carrying fact 7's `84 of 300 / 28.0 % / 2.587 against 2.027` as the measured size of the open question and attributing it to "FILTERED side-positions". That population is not U2 §5.3's row. On the row the protocol actually specifies the figure is **22 of 174, 12.6 %**. The erratum's direction survives — the question is live, not vestigial — but an append-only log now carries a number measured on a population the design does not have. Whether that is an amendment or a fresh line is the architect's.

5. **The measurement record of this work package is clean this round**, and that is worth writing down beside D-318. Eight facts, eight reproductions, including one I rebuilt without seeing the author's instrument. The defect this round is not a number that did not reproduce. It is a number that reproduced perfectly and was asked to mean something it does not.
