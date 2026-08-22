# DECISION-RED-TEAM — MATRIX META-1 (`matrix_META1_successor_R15.md`)

**SUBJECT REVISION: `b683d48`.** `docs/experiments/matrix_META1_successor_R15.md`.

```
$ git rev-parse HEAD                     # at exit
1f834ca80e53bb6e3cdccd4412dad9e38157d5fc
$ git status --porcelain
(no output, at entry and at exit)
```

**Does the subject revision still match HEAD? NO — and that does not matter here,
because the matrix and its D-337 subject are byte-identical across the move.** HEAD
advanced from `b683d48` to `1f834ca` (the dispatch file landing, 154 lines added, no
other file touched).

**D-337 FREEZE: INTACT. NOT A VOID.**

```
$ git diff b683d48..HEAD --stat
 docs/experiments/matrix_META1_REDTEAM_dispatch.md | 154 ++++++++++++++++++++++
 1 file changed, 154 insertions(+)
$ git diff b683d48..HEAD -- docs/experiments/U1_gate_supersession.md \
    docs/experiments/U2_node_protocol.md docs/experiments/U3_tier_t.md \
    docs/experiments/U4_soundness_instrument.md docs/experiments/WPQ_seed.md \
    docs/experiments/section_owner_table.md
(no output)
```

**Context was fresh.** I did not author the matrix, any unit, any review, either
selection record, or either U4-R round. I edited no repository file other than this
report, staged nothing, committed nothing, created no worktree, and ran no git write
command. Scratch work is under this session's scratchpad and nothing in this report
depends on it surviving.

---

# VERDICT

**5 KILL, 11 MAJOR, 4 MINOR.**

**NO SELECTION MAY BE TAKEN FROM THIS FIELD. NO ROW IN IT IS SELECTABLE.**

The recommendation D falls on its load-bearing premise. **E3 is false** — eleven
recorded restatement instances sit in NORMATIVE sections, they are enumerated in a
landed table inside the matrix's own frozen subject, they are live at HEAD, and A/D's
strip does not reach one of them (K1). The matrix's flip clause 2 therefore **had
already fired when the matrix was authored**, which is `restructure_selection_15b.md`
F6's class and was scored KILL in this same work package (K3). D's citation gate is
vacuous against every recorded instance of the defect it is bought for (K2). E1's
cited source does not exist in the tree, which fires **D-336's flip clause on the
round that landed D-336** (K4). And registered condition 1, read literally, turns a
CI gate red on line 3 of every one of the six subject documents (K5).

---

# PER-E-ITEM REPRODUCTION TABLE (obligation A)

| item | verdict | note |
|---|---|---|
| **E1** three generators measured; D-331 reaches one | **REPRODUCES WITH A DIFFERENT NUMBER, AND ITS SOURCE DOES NOT EXIST** | The taxonomy reproduces from D-335. *"Source: closure session report"* names **no file in the tree** — K4. *"failed U3 four rounds"* is four findings across **three** reports — m1. It mis-sorts U3 MAJOR B, a restatement, under self-completeness — m1 |
| **E2** sole working detector, 54/61-row inventories | **REPRODUCES WITH A DIFFERENT MEANING** | The two row counts reproduce exactly. The universal does not: the instrument exists in **two** documents, covers **U4-Z only**, and reached 9 of u-rev 8's 15 findings — M1. The round-2 attribution reproduces verbatim |
| **E3** every recorded generator instance sits in meta-text | **DOES NOT REPRODUCE** | **KILL K1.** Eleven recorded instances in normative sections. Second half also false — M9, M11 |
| **E4** U4 MEASURED 2105, +219, larger than 1975 | **REPRODUCES EXACTLY** | 1886 → 2105 = +219; `git show 6feb40a:…wp15b_design.md \| wc -l` = 1975. The only E-item that reproduces to the digit |
| **E5** external systems of record exist for every meta-claim; none can drift | **DOES NOT REPRODUCE** | **M7.** Five ADR lines exist solely to correct earlier ADR lines; two landed review reports carry false summaries of their own tables; R19's closure has no ADR line at all |
| **E6** convergence reversed, 23 → 20 → 12 → 15+, zero PASS | **DOES NOT REPRODUCE; THE DIRECTION DOES** | **M4.** Seven denominators tried, none yields the series. The tail reversal reproduces on **all seven**. *"zero PASS"* is false across the corpus — three landed PASSes |

**No E-item carries a MEASURED or ESTIMATED mark** (m2). The provenance comment
discloses this rather than repairing it; disclosure does not discharge D-291, and
three of the six items are numeric.

---

# FINDINGS

## K1 (KILL) — E3 IS FALSE. ELEVEN RECORDED RESTATEMENT INSTANCES SIT IN NORMATIVE SECTIONS, THEY ARE ENUMERATED IN A LANDED TABLE INSIDE THE MATRIX'S OWN FROZEN SUBJECT, AND A/D's STRIP REACHES NONE OF THEM.

**The claim attacked, verbatim:**

> E3. Every recorded generator instance sits in meta-text: heads, change logs,
> lineage prose, status blocks (U4-Z, REVIEW STATUS), self-completeness claims.

**Contradicting evidence — U3's own B7 residual table, `U3_tier_t.md:886–898`, live at
HEAD, LISTED-NOT-REPAIRED by the unit's own words:**

```
$ sed -n '885,897p' docs/experiments/U3_tier_t.md | cut -c1-96
  | site | rendering | census cell it derives from |
  |---|---|---|
  | **U2** (u-rev 4, landed `7dfd047`) §5.3 | `70.8 %` | `BATCHED nodes`, corpus roots |
  | **U4** (u-rev 7, landed `0f49c90`) §8.4 | `70.8 %` | `BATCHED nodes`, corpus roots |
  | **§6.5, the STRONGEST SURVIVING ATTACK — MAJOR A, `wp15b_U3_REVIEW_urev4.md`** | `2.7
  | §6.3, option C's cost cell | `6.83` | `option C — Tier T outside the r2 ball` |
  | §6.3, option C's failure-mode cell | `23.2` | **NOT ONE CELL (MAJOR 1, `wp15b_U3_REVI
  | §10, the withdrawn config comment | `6.83` | `option C — Tier T outside the r2 ball` |
  | **§6.2, the sampler sentence — u-rev 2 omitted it** | `78.0 → 123.7` | `radius-2 ball`
  | **§6.1, the threshold-repair cost — u-rev 2 omitted it** | `+0.17`, `+0.04` | `option
  | **§6.3 and §10, the outside-the-ball share — u-rev 2 omitted it** | `29 %` | `option C
  | **§12 item 4, the forced-row share — u-rev 2 omitted it** | `29.2 %` | 100 minus `BATC
  | **§10, the `1024` tactical config derivation — MINOR 3** | `under 400` | `radius-2 bal
```

**Every site named is a NORMATIVE section.** U3's own section map:

```
$ grep -n '^#\{2,4\} ' docs/experiments/U3_tier_t.md | awk -F: '$1<700' | cut -c1-64
171:## U3-A. Lineage — what has attacked this unit's content, and at whi
202:## 6. MATRIX M1 — Tier-T qualification — SURVIVES AMENDED
204:### 6.1 The reading, corrected
227:### 6.2 The measurements, with their sampling regime
321:### 6.3 The options
330:### 6.4 The asymmetry, re-grounded
368:### 6.5 ADOPTED: C at the threshold reading
398:## 7. What survives here of §7, and what does not
425:## 10. The config shape
560:## U3-T. The tests this unit registers
574:## U3-M. What this unit measures
```

§6.1, §6.2, §6.3, §6.5, §10, §12 item 4 (in U3-M), U2 §5.3 and U4 §8.4. Not one of
them is a head, a change log, lineage prose, a status block or a self-completeness
universal. **They are live at HEAD:**

```
$ awk 'NR<860' docs/experiments/U3_tier_t.md \
    | grep -n "78\.0 → 123\.7\|\*\*+0\.17\*\*\|29 % of\|\*\*29\.2 %\*\*" | cut -c1-84
223:the repair: **+0.17** cells/node for B, **+0.04** for C at corpus roots. There is
236:which inflated the ball 78.0 → 123.7 by the sampler rather than by depth.
327:| **C — ≥2 for us, ≥3 for them** | The lemma in §6.4 | see the census block; **MEA
551:SEE". MEASURED, **29 % of option C's Tier T lies outside the radius-2 ball**
627:   **29.2 %** of them that take a forced row, so the registered figure is a
```

**These are the generator by D-331's own definition** — *"every one of those defects is
a SECOND COPY of content whose FIRST copy is correct and lives somewhere else"*. The
first copy is the pinned census block; each of these is a rounded or derived second
copy that goes stale silently when a cell moves, which is exactly what
`wp15b_U3_REVIEW.md` MAJOR 4 charged and what the unit records as OPEN.

**Why it breaks the matrix.** D's whole case, in the recommendation's own first clause,
is *"E3 says the defect surface is meta-text"*. It is not. The strip in A and D —
*"no heads beyond title+u-rev, no change logs, no lineage, no status blocks, no
self-completeness universals"* — removes **zero** of these eleven. D executes at the
cost of a strip session, a citation gate and four re-reviews, and the recorded
generator survives in eleven places across three of the four units.

**Severity: KILL against A and against D.**

---

## K2 (KILL) — D's CITATION GATE IS VACUOUS. MEASURED: 576 REFERENCES, 67 DISTINCT KEYS, ZERO DANGLING. THE RECORDED DEFECT IS MISATTRIBUTION, AND EXISTENCE-CHECKING IS INVARIANT UNDER IT — WHICH IS A CLAUDE.md BREACH ON ITS FACE.

**The claim attacked, verbatim (option D, and registered condition 4):**

> a citation gate (tools/) mechanically checks every D-nnn reference in unit docs
> resolves to an existing key … **4.** Citation gate before the re-reviews, so wrong
> pointers are caught mechanically and reviews spend zero rows on reference existence.

**Contradicting evidence.** I built the gate's own criterion and ran it over the six
frozen documents:

```
$ docs="docs/experiments/U1_gate_supersession.md docs/experiments/U2_node_protocol.md \
docs/experiments/U3_tier_t.md docs/experiments/U4_soundness_instrument.md \
docs/experiments/WPQ_seed.md docs/experiments/section_owner_table.md"
$ grep -oh 'D-[0-9]\{1,3\}' $docs | wc -l
576
$ grep -oh 'D-[0-9]\{1,3\}' $docs | sort -u | wc -l
67
$ for k in $(grep -oh 'D-[0-9]\{1,3\}' $docs | sort -u); do \
    grep -q "^$k:" docs/decisions.md || echo "DANGLING: $k"; done
(no output)
```

**Zero dangling references.** Reviews already spend zero rows on reference existence,
because there is nothing to spend them on. The gate's stated purchase is already owned.

**And it is blind to every recorded instance of the class it is named for.** The two
strongest are:

```
$ sed -n '110,112p' docs/experiments/U4_soundness_instrument.md | cut -c1-88
  **`D-333` (R18)** rules on N-E's unattacked-in-its-own-right residual and is fold
  at that bullet in U4-Z. **`D-329`'s relative-base residual is CLOSED**, not by th
  unit but by architect ruling R19 at `63eac4c`, whose REVIEW-impl PASSED at `d59fe
$ grep -q "^D-329:" docs/decisions.md && echo "D-329 RESOLVES — the gate is GREEN here"
D-329 RESOLVES — the gate is GREEN here
```

`D-329` exists. `D-331` exists. Both of the u-rev 8 flip-firing instances — MAJOR 1's
misattributed residual and MAJOR 2's *"D-331 does not reach"* — are references that
**resolve** and **misdescribe**. So is D-335's instance (i), so is `wp15b_U4_REVIEW_urev7.md`
MAJOR 2 and MAJOR 3, and so is this matrix's own Status line (M5).

**Why it breaks.** CLAUDE.md, in the paragraph binding registered criteria: *"A
criterion that is a property the named defect class PRESERVES … passes vacuously and is
not a criterion; it must be one that defect could falsify."* Reference-resolution is
preserved by **every** recorded instance of the generator. The gate is not weak
evidence; it is a criterion the defect cannot falsify, registered as the round's one
mechanical safeguard.

**Compounding — the gate cannot be scoped without the meta-text D deletes.**

```
$ grep '^>' docs/experiments/U4_soundness_instrument.md | grep -o 'D-[0-9]\{1,3\}' | wc -l
95
$ grep -o 'D-[0-9]\{1,3\}' docs/experiments/U4_soundness_instrument.md | wc -l
375
```

95 of U4's 375 references sit inside blockquoted RECORD, which the unit *may not edit*
— *"a text an architect selected from is carried unedited"* (D-331). A gate that flags
one of those flags an unrepairable line, so it must exclude record; and the only
artefact in the tree that says which lines are record is the pair of §8/§9 record
stamps, which the strip deletes (M8). **The gate D buys depends on the meta-text D
demolishes.**

**Severity: KILL against D's registered condition 4, and against the composition that
distinguishes D from A.**

---

## K3 (KILL) — FLIP CLAUSE 2 HAD ALREADY FIRED WHEN THE MATRIX WAS AUTHORED. THAT IS `restructure_selection_15b.md` F6's CLASS, SCORED KILL IN THIS WORK PACKAGE.

**The clause attacked, verbatim:**

> If a drift instance appears in NORMATIVE text **after D executes**, the meta-text
> diagnosis (E3) is falsified: STOP, architect, no successor law authored in-session.

**Contradicting evidence.** K1 establishes eleven drift instances in normative text
**before** D executes. The clause is armed only after the cost is paid, so on the
evidence that already exists it cannot fire, and the round's own falsification test is
scheduled for the point at which falsification is most expensive.

**The precedent is this project's and it is a KILL:**

```
$ sed -n '115,116p' docs/experiments/restructure_selection_15b.md
- **F6 (KILL)** flip clause 2 had already fired: unit 3 measures ~325 lines after T1
  restoration and ~425 once §10's config shape is owned, against unit 2's 355.
```

**Why it breaks.** A flip clause whose trigger condition is already satisfied at
authoring time is not a safeguard; it is a deferral of the finding that kills the row.
Here the deferral has a price attached — a strip session, a citation gate, a
SHELL_CHECKLIST review and four re-reviews of four rewritten documents — and E3 is
false today, measurably, for free.

**Severity: KILL against D.**

---

## K4 (KILL) — E1's CITED SOURCE IS NOT IN THE TREE. D-336's FLIP CLAUSE FIRES ON THE ROUND THAT LANDED D-336, AND THE MATRIX BINDS D-336 BY NAME.

**The claim attacked, verbatim:**

> E1. Three drift generators measured … **Source: closure session report, D-335.**

**Contradicting evidence.**

```
$ ls docs/experiments/ | grep -i clos
(no output)
$ grep -rn "closure session" docs/ tools/ crates/ | cut -c1-96
docs/process_readings.md:255:closure session was dispatched to land T1 and T2 and this reading
docs/experiments/wp15a_prereg.md:589:sat untracked, and the previous session was told it would
docs/experiments/wp15a_prereg.md:648:closure session landed both halves in one commit per D-245'
```

**No file named or describable as a "closure session report" exists.** The three
strings that match belong to a different work package (WP-1.5a) and to the T-bucket.
D-335 is in the tree and is real; the other half of the citation is not.

**Why it breaks.** D-336 clause (1): *"a cited number, quotation or command output is
transcribed from a FILE IN THE TREE, and the citation names that file"*. Clause (3):
*"an agent's completion message, a subagent's summary, a scratchpad path and the
author's memory of any of them are NOT citable"*. The matrix's **registered condition
2 binds D-336 by name**, and D-336 landed in the same commit as the matrix, so this is
a document written under the rule. D-336's own flip clause:

> **Flips if a document written under this rule nonetheless ships a fabricated or
> mis-attributed citation**, at which point the transient-source diagnosis is wrong,
> the defect is something else, and this line is superseded by the round that finds it.

**It fires here.** This is the third consecutive architect line in this work package to
flip on the round that landed or first governed it — D-328 → D-330, D-331 → D-335, and
now D-336. The E-item whose source cannot be retrieved is **E1, the taxonomy the whole
matrix is built on**.

**Severity: KILL against the evidence base as a whole, and a landed flip the architect
must record independently of what happens to this field.**

---

## K5 (KILL) — REGISTERED CONDITION 1 IS NOT EXECUTABLE ON LINE 3 OF ANY OF THE SIX DOCUMENTS: DELETION-ONLY TURNS A CI GATE RED.

**The claim attacked, verbatim:** option A, *"no heads beyond title+u-rev"*; condition
1, *"deletion-only … A sentence that cannot be deleted whole is deleted anyway"*.

**What is on line 3 of every subject document:**

```
$ sed -n '3p' docs/experiments/U4_soundness_instrument.md
<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->
```

**It has a machine consumer that refuses its absence by name:**

```
$ sed -n '891,905p' crates/pistol-solver/tests/wp15b_census.rs | cut -c1-84
    assert!(
        on_disk.len() >= 3,
        "only {} file(s) in {dir} carry the carve marker; the marker itself has dri
         an equality between two empty sets certifies nothing: {on_disk:?}",
        on_disk.len()
    );

    let mut named: Vec<String> = CARVE_DOCS.iter().map(|name| (*name).to_owned()).c
    named.sort();
    assert_eq!(
        named, on_disk,
        "this pin's CARVE_DOCS list and the carved documents on disk disagree. File
         the marker but not read by the pin are green-over-unread; files listed but
         disk are already a panic in carve_documents()."
    );
```

**Demonstrated on a scratch copy, using the pin's own membership rule (`carries_marker`
— the marker as a whole trimmed line), with no repository file touched:**

```
$ S=<this session's scratchpad>; mkdir -p $S/stripdocs
$ cp docs/experiments/*.md $S/stripdocs/ && M='<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->'
$ for f in $S/stripdocs/*.md; do awk -v m="$M" '{l=$0; gsub(/^[ \t]+|[ \t]+$/,"",l); if(l==m) found=1} END{if(found) print FILENAME}' $f; done | xargs -n1 basename
section_owner_table.md
U1_gate_supersession.md
U2_node_protocol.md
U3_tier_t.md
U4_soundness_instrument.md
WPQ_seed.md
$ sed -i '3d' $S/stripdocs/U4_soundness_instrument.md     # A's rule, applied literally
$ for f in $S/stripdocs/*.md; do awk -v m="$M" '{l=$0; gsub(/^[ \t]+|[ \t]+$/,"",l); if(l==m) found=1} END{if(found) print FILENAME}' $f; done | xargs -n1 basename
section_owner_table.md
U1_gate_supersession.md
U2_node_protocol.md
U3_tier_t.md
WPQ_seed.md
```

Five carriers against a six-name `CARVE_DOCS`, so `assert_eq!(named, on_disk)` fails.
The gate is green today and I ran it:

```
$ cargo test -p pistol-solver --test wp15b_census 2>&1 | tail -3
test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 3.76s
```

**Why it breaks.** Condition 1 says a sentence that cannot be deleted whole *"is
deleted anyway and its normative content, if any, gets a pointer to its home"*. This
line's content is not normative and has no home — it **is** the mechanism. Its "home"
is a Rust constant that reads the document. The rule as registered has no carve-out,
and the first line the strip session meets in each of six files is one where following
it turns CI red. A registered condition that fails on line 3 of every document it
governs has not been tried against the tree.

**Severity: KILL against condition 1 as registered; repairable in one sentence, which
is why it is disqualifying only for *this* text and not for the idea.**

---

## M1 (MAJOR) — E2's "SOLE WORKING DETECTOR" OVER-SPECIFIES THE INSTRUMENT. THE HAND INVENTORY EXISTS IN TWO DOCUMENTS, COVERS ONE 519-LINE SECTION OF ONE UNIT, AND REACHED 9 OF THE 15 FINDINGS IN THE ROUND E2 CITES.

**The claim attacked, verbatim:**

> E2. Sole working detector across ~10 rounds: fresh-context reviewer hand-building a
> claim inventory (54 rows u-rev 7, 61 rows u-rev 8). … No law, grep, structure, or
> matrix has found one instance.

**The row counts reproduce exactly** and I credit that:

```
$ awk 'NR>=342 && NR<=421' docs/experiments/wp15b_U4_REVIEW_urev7.md | grep -c '^| Z'
54
$ sed -n '769,846p' docs/experiments/wp15b_U4_REVIEW_urev8.md | grep -c '^| Y'
61
```

**Contradicting evidence — the instrument's whole population:**

```
$ grep -rn "^# MY OWN.*CLAIM INVENTORY" docs/experiments/*.md
docs/experiments/wp15b_U4_REVIEW_urev7.md:342:# MY OWN U4-Z CLAIM INVENTORY, BUILT INDEPENDENTLY
docs/experiments/wp15b_U4_REVIEW_urev8.md:762:# MY OWN CLAIM INVENTORY FOR U4-Z, BUILT INDEPENDENTLY
```

**Two documents, both scoped to U4-Z.** The u-rev 8 inventory states its own span:
*"I enumerated every claim in U4-Z (U4:1587–2105)"* — 519 lines of one 2105-line unit,
**24.7 %** of U4 and **10.3 %** of the four units. The other ~fifteen rounds cited by
*"~10 rounds"* used no inventory at all, and they are where most of the recorded
instances come from: U1 BLOCKING 1, U2 F1–F6/G1–G2/H1–H2, U3 MAJOR 4/5/A/B/C/D/E.

**And it did not reach most of its own round's findings.** Mapping the 11 FAILS to the
15 findings, the inventory reaches BLOCKING 1, MAJOR 1, 2, 3, 4, 5, 6 and MINOR 1, 3.
It does **not** reach MAJOR 7 (the head's D-311 self-exemption), MINOR 2 (the blind
derived command), MINOR 5 (the unlogged carve-provenance correction), MINOR 6 (the §8
record stamp) or MINOR 7 (§8.7:919) — all of them outside U4-Z, all found by ordinary
fresh-context reading.

**Why it breaks.** The detector measured to work is **a fresh context reading the whole
document**. The inventory is one technique that context used on one section. D's
registered condition 3 institutionalises the technique — *"reviewer inventories land
in-tree … fresh-built each round"* — and D's strip is justified by *"the inventory is
the instrument … against a shrunken claim set"*. If the instrument is the fresh
context rather than the table, the shrinkage argument does not transfer, and condition
3 mandates the narrower thing.

**Severity: MAJOR against E2, and against D's ground clause *"E2 says the inventory is
the instrument"*.**

---

## M2 (MAJOR) — BOTH LANDED INVENTORIES SHIP A SELF-COMPLETENESS UNIVERSAL ABOUT THEIR OWN TABLE THAT THEIR OWN TABLE FALSIFIES. THE INSTRUMENT E2 CALLS THE SOLE WORKING DETECTOR IS PRODUCING GENERATOR (2), UNCAUGHT, AND D's CONDITION 3 LANDS MORE OF IT.

**The claims attacked, verbatim:**

> **Result: 34 claims examined, 31 hold, 3 fail** (`wp15b_U4_REVIEW_urev7.md:344`)
>
> **Result: 61 claims examined, 55 hold, 6 fail.** (`wp15b_U4_REVIEW_urev8.md:767`)

**Contradicting evidence — each report's own table:**

```
$ awk 'NR>=342 && NR<=421' docs/experiments/wp15b_U4_REVIEW_urev7.md | grep -c '^| Z'
54
$ awk 'NR>=342 && NR<=421' docs/experiments/wp15b_U4_REVIEW_urev7.md | grep '^| Z' | grep -c 'FAILS'
4
$ sed -n '769,846p' docs/experiments/wp15b_U4_REVIEW_urev8.md | grep -c '^| Y'
61
$ sed -n '769,846p' docs/experiments/wp15b_U4_REVIEW_urev8.md | grep '^| Y' | grep -c 'FAILS'
11
$ sed -n '769,846p' docs/experiments/wp15b_U4_REVIEW_urev8.md | grep '^| Y' | grep 'FAILS' | cut -d'|' -f2 | tr -d ' ' | tr '\n' ' '
Y1 Y22 Y23 Y24 Y30 Y33 Y35 Y36 Y37 Y51 Y61
```

u-rev 7: **54 rows, 4 fail, 50 hold** against a stated 34/31/3.
u-rev 8: **61 rows, 11 fail, 50 hold** against a stated 61/55/6. No reading of the
table gives 6 — the eleven FAILS carry nine distinct findings, not six.

**Why it breaks.** D-335's generator (2) is *"a universal about the document's own
state, asserted by its author, false in the commit that asserts it"*. Both sentences
are exactly that, in the one artefact E2 names as the only thing measured to find the
class, and **no round has recorded either** — including the u-rev 8 reviewer, who read
the u-rev 7 inventory closely enough to say *"I did not reuse the prior reviewer's 54
rows"* and took the row count over the summary without noticing they disagree.

Under D-336 clause (4) — *"inside a landed file, a SUMMARY LINE IS NOT THE MEASUREMENT
— the pasted output is, and where the two disagree the paste governs and the
disagreement is a finding against the summary"* — the matrix took the paste, which is
right, and did not record the disagreement, which is the finding.

**The consequence for D is direct.** Registered condition 3 makes the inventory a
landed review deliverable, and review reports are the one document class in this
project **nothing reviews**. D would institutionalise, as the round's primary
instrument, an artefact that has produced the generator twice out of two and has no
reviewer above it.

**Severity: MAJOR against E2, and against D's registered condition 3.**

---

## M3 (MAJOR) — TWO UNRECORDED LIVE INSTANCES OF BLOCKING 1's CLASS IN FROZEN SUBJECT DOCUMENTS, FOUND BY A ONE-LINE SHELL LOOP. U1 NAMES ITSELF TWO u-revs AND CITES ITS ONLY PASS NOWHERE.

**A mechanical sweep, run in under a second:**

```
$ for f in U1_gate_supersession U2_node_protocol U3_tier_t U4_soundness_instrument \
           WPQ_seed section_owner_table; do p=docs/experiments/$f.md; \
    printf "%-26s head=%-3s foot=%s\n" "$f" \
      "$(grep -m1 -o '^\*\*u-rev [0-9]*' $p | grep -o '[0-9]*')" \
      "$(tail -3 $p | grep -o 'u-rev [0-9]*' | tail -1)"; done
U1_gate_supersession       head=3   foot=u-rev 1
U2_node_protocol           head=6   foot=u-rev 6
U3_tier_t                  head=7   foot=
U4_soundness_instrument    head=8   foot=u-rev
section_owner_table        head=6   foot=u-rev 5
WPQ_seed                   head=2   foot=u-rev 2
```

*(U1's `foot=u-rev 1` is the loop taking the last match on a three-line tail; the
closing line reads `u-rev 2` and the `u-rev 1` is its own next clause — both are one
behind the head.)*

**U1 — head `u-rev 3`, closing line `u-rev 2`, and its only PASS is cited nowhere:**

```
$ grep -n '^\*\*u-rev' docs/experiments/U1_gate_supersession.md
15:**u-rev 3.** Carved from `docs/experiments/wp15b_design.md` §4 at `6feb40a`
$ tail -3 docs/experiments/U1_gate_supersession.md
*U1, u-rev 2. u-rev 1 was a carve, not a revision; u-rev 2 is a repair of
u-rev 1's REVIEW-design FAIL (`docs/experiments/wp15b_U1_REVIEW.md`, pinned
`38f21b9`). IMPL has not started.*
$ grep -c "wp15b_U1_REVIEW_urev2" docs/experiments/U1_gate_supersession.md
0
$ sed -n '19,21p' docs/experiments/wp15b_U1_REVIEW_urev2.md
## VERDICT: **PASS**

0 BLOCKING, 0 MAJOR, 0 MINOR.
```

U1's head still reads *"**THIS UNIT HAS BEEN REVIEWED, AT u-rev 1, AND FAILED** … the
repaired text is itself unreviewed until a fresh REVIEW-design runs against u-rev 2"*
(`U1:28–38`) — u-rev 2 **was** reviewed and it is the only PASS in the work package.
U1-A has no row for it, and U1's owed-list says *"a REVIEW-design of THIS text at THIS
u-rev"* without ever recording that the previous one passed.

**`section_owner_table.md` — head `u-rev 6`, closing line `u-rev 5`:**

```
$ sed -n '24p' docs/experiments/section_owner_table.md | cut -c1-64
its u-rev. **THIS TABLE IS AT u-rev 6**: the `now` column is DELETED
$ tail -1 docs/experiments/section_owner_table.md
*Carve table, u-rev 5.*
```

This one landed at `4fd88ec`, **after** `wp15b_U4_REVIEW_urev8.md` reported the
identical defect in U4 as its BLOCKING finding.

**Why it breaks.** Three of the six frozen documents fail one line of shell; two of the
three failures are recorded by nobody; and one was manufactured in the commit *after*
the class was reported as BLOCKING in a sibling. This is generator (1)/(2) at the
surface a reader meets last, and it says three things at once: the class is not
converging, a **structure** finds it in one second (against E2's universal), and the
gate D actually needs is not the one D registers.

**Severity: MAJOR against E2's universal, MAJOR against E6's convergence framing, and
the ground for missing row R4.**

---

## M4 (MAJOR) — E6's SERIES REPRODUCES ON NO DENOMINATOR I COULD CONSTRUCT. "ZERO PASS AT CLOSE" IS FALSE ACROSS THE CORPUS. THE NULL ROW IS NOT DEAD ON THE GROUND GIVEN.

**The claim attacked, verbatim:**

> E6. Convergence reversed: findings 23 -> 20 -> 12 -> 15+ (U4 alone), zero PASS at
> close.

**Every landed report's own verdict block, transcribed** (`grep -A3 '^## VERDICT'`):
design rev 7 7/7/9; U1 1/0/1 then **PASS 0/0/0**; U2 0/1/1, 0/1/1, 0/1/1, 0/0/2,
0/1/1; U3 2/5/4, 0/2/3, 0/2/0, 0/1/0, 0/2/0; U4 3/3/5, 1/2/4, 0/4/4, 1/7/7.

**Seven denominators, computed:**

```
A) U4 alone, total:       [23, 11, 7, 8, 15]
B) U4 alone, B+M:         [14, 6, 3, 4, 8]
C) all units per round:   [23, 26, 14, 12, 22]
D) all units, B+M:        [14, 15, 6, 7, 12]
E) three units (no U1):   [23, 24, 14, 12, 22]
F) U3+U4 only:            [23, 22, 12, 10, 18]
G) cumulative U4:         [23, 34, 41]
TARGET                  : [23, 20, 12, '15+']
```

**None reproduces.** (F) carries 23 and 12 but pairs them with 22 and a tail of 18;
(C) carries 23 and 12 in the wrong positions. The implementing session's provenance
comment reached the same wall on reading (A) and said so; I confirm it on six more.

**"Zero PASS at close" is false across the corpus:**

```
$ grep -rn "^\*\*\?PASS\|VERDICT.*PASS" docs/experiments/wp15b_*REVIEW*.md \
    | grep -iv "passed\|passes\|pass a\|does not" | cut -c1-64
docs/experiments/wp15b_U1_REVIEW_urev2.md:19:## VERDICT: **PASS**
docs/experiments/wp15b_trackC_R19_REVIEW_impl.md:155:**PASS.**
docs/experiments/wp15b_trackC_REVIEW_impl.md:709:**VERDICT: PASS**
```

Three landed PASSes. The clause is true only if scoped to U4 alone,
which the sentence's parenthetical scopes to the **series**, not to the PASS clause.

**What DOES reproduce, and I record it in the matrix's favour:** the tail reverses on
**all seven** denominators — 8→15, 4→8, 12→22, 7→12, 12→22, 10→18. The direction is
robust. The series is not, and the number *"20"* appears nowhere.

**Why it breaks.** The null row is declared *"Dead on the evidence"* and E6 is the
evidence. A row killed by a series that reproduces on no reading is killed by a
criterion the field's own column does not measure — **which is verbatim the finding
that stopped the last matrix in this work package** (D-334: *"it is excluded by a
criterion that column does not measure, so what the exclusion convicts is the column
and not the row"*), and the project then executed the null row anyway.

**Severity: MAJOR. The NULL row is resurrected to SURVIVES WOUNDED.**

---

## M5 (MAJOR) — THE MATRIX'S STATUS LINE MISATTRIBUTES ITS OWN LANDING PREREQUISITE TO D-310, WHICH RECORDS NOTHING OF THE KIND. THAT IS THE EXACT CLASS D-335 FIRED D-331's FLIP ON, COMMITTED ON THE MATRIX'S SECOND LINE.

**The claim attacked, verbatim:**

> Status: AUTHORED BY ARCHITECT, NOT SELECTED. Awaits fresh-context
> DECISION-RED-TEAM. **Land at SHA before dispatch (D-310 prerequisite).**

**Contradicting evidence — D-310 in full, at HEAD.** D-310 selects restructure option
D. It carries five registered conditions: (1) the section-owner table, (2) the split
matrix-restoration item, (3) the pin re-targeted or retired, (4) binding IMPL order,
(5) the corrected four-way cut. It has one flip clause, about the follow-up work
package and the F+T SPRT delta.

```
$ sed -n '665p' docs/decisions.md | grep -c "dispatch"
0
$ sed -n '665p' docs/decisions.md | grep -o "Land at SHA\|before dispatch\|matrix.*land"
(no output)
```

**Nothing in D-310 concerns landing a matrix at a SHA before dispatch.** The real home
of that requirement is CLAUDE.md's Process section — *"A review is dispatched against
a NAMED REVISION — a commit SHA, or a `git stash create` SHA where the work is
uncommitted"* — composed with D-311's label discipline.

**Why it breaks.** This is a reference that resolves and misdescribes: D-329's
relative-base residual, one document over. It is committed in the matrix's own status
matter — the surface E3 says the class lives on — by a matrix arguing that the class
is confined to that surface and that D fixes it. It also demonstrates K2 from the
inside: the citation gate would have passed this line, because `D-310` exists.

**Severity: MAJOR, and evidentially it is the matrix's own strongest self-refutation.**

---

## M6 (MAJOR) — FLIP CLAUSE 1's ">20 PERCENT" IS INVENTED, UNMARKED AND DENOMINATOR-FREE. IT RETURNS OPPOSITE VERDICTS ON TWO DEFENSIBLE READINGS OF THE SAME TEXT, AND ITS REMEDY CONTRADICTS THE MATRIX's OWN E6.

**The clause attacked, verbatim:**

> If the red team shows >20 percent of unit text is meta-load-bearing (reviewers cannot
> navigate without it), flip to B: the meta-text is functional and the instrument, not
> the surface, is the fix.

**(a) The number is invented.** It carries no MEASURED or ESTIMATED mark, and nothing
in the evidence base or the tree produces it:

```
$ grep -rn "20 percent\|20 %\|20%" docs/experiments/matrix_U4R_*.md docs/experiments/restructure_*.md \
    docs/experiments/section_owner_table.md | grep -i "meta\|load-bear" | wc -l
0
```

CLAUDE.md: *"EVERY NUMERIC CLAIM IN THE MATRIX IS MARKED **MEASURED** OR **ESTIMATED**,
and an estimate that could have been measured in seconds is a finding"* (D-291).

**(b) The denominator is unstated.** *"unit text"* — one unit or four? Per-unit or
aggregate? 20 % of U4 is 421 lines; 20 % of the four units is 870.

**(c) The property is undefined, and this project has recorded that three times.**

```
$ grep -rn "one sitting" docs/experiments/matrix_U4R_*.md docs/experiments/section_owner_table.md | cut -c1-92
docs/experiments/matrix_U4R_REDTEAM.md:849:> matrix's undefined "reviewable in one sitting" 
docs/experiments/matrix_U4R_restructure.md:265:for.** "Reviewable in one sitting" has no ins
docs/experiments/matrix_U4R_restructure_rev2.md:169:PROPERTY — "reviewable in one sitting" h
docs/experiments/matrix_U4R_restructure_rev2.md:208:- **It does not claim "reviewable in one
docs/experiments/matrix_U4R_restructure_rev2.md:237:  defect-rate ground** — readability, or
docs/experiments/matrix_U4R_REDTEAM_round2.md:758:The matrix concedes one undefined property
docs/experiments/section_owner_table.md:259:about a unit exceeding one sitting, and after th
docs/experiments/section_owner_table.md:334:the whole restructure was bought for is "reviewa
docs/experiments/section_owner_table.md:414:means the property the restructure was bought fo
docs/experiments/section_owner_table.md:434:somewhere. Whether "reviewable in one sitting", 
```

*"reviewers cannot navigate without it"* is the same shape and has no more instrument
than *"reviewable in one sitting"* does.

**(d) MEASURED, it answers both ways on U4.** Taking the navigation-critical set
narrowly — the `§n` resolution rule (8), the two record stamps (32 + 13), and the two
selection folds (135 + 141) — gives **329 lines = 15.6 %**, and the clause does not
fire. Taking it to include the surfaces a reviewer actually consults to locate a claim
— adding the head's selection summary (43), the three REVIEW STATUS tables (86) and
U4-A (32) — gives **490 lines = 23.3 %**, and it does. A trigger that returns opposite
verdicts on two honest readings of one document cannot discharge its office.

**(e) F5's class: the trigger is about one thing and the remedy about another.** The
trigger is NAVIGABILITY. The remedy is B, whose defining property is a repair loop —
and the matrix's own E6 says *"current trajectory does not converge"* and B's own
failure-mode cell says *"E6 says current trajectory does not converge"*. So if >20 %
of the text is load-bearing, the sound conclusion is *the surface may not be deleted*,
not *the repair loop converges*. Flip clause 1 routes to a row its own author scored
non-convergent.

**The honest criterion, offered.** Replace the percentage with a per-block homing test
that the defect class can falsify: *"Before any deletion lands, the strip session
lists every block it proposes to delete with the file and line of its home, and a
second party OPENS each home and confirms the claim is there. A block with no home in
`git log`, `docs/decisions.md`, a selection record or a landed report is NOT deleted
and is counted. Registered consequence: any homeless-block count above zero means
condition 1 is not executable as registered and the round STOPS rather than reworking
the block."* That is measurable before the strip, its referent is externally derived
(the home is read, not asserted), and it fires on the failure mode D itself names.

**Severity: MAJOR against flip clause 1.**

---

## M7 (MAJOR) — E5 IS FALSE. `docs/decisions.md` DRIFTS, LANDED REVIEW REPORTS DRIFT, AND ONE CLAIM D's STRIP WOULD DELETE HAS NO ADR HOME AT ALL WHILE ITS OWN HOME SAYS THE OPPOSITE.

**The claim attacked, verbatim:**

> E5. External systems of record already exist for every meta-claim: git log (change
> history), docs/decisions.md (lineage, rulings), landed review reports (review
> status). **None can drift**; the in-document copies are the drift.

**(a) The ADR log drifts — five lines exist solely to correct earlier ADR lines:**

```
$ for k in D-301 D-322 D-325 D-326 D-330; do sed -n "$(grep -n "^$k:" docs/decisions.md \
    | cut -d: -f1)p" docs/decisions.md | cut -c1-64; done
D-301: D-255 IS WRONG ON A NUMBER IT STATES, AND THE DECISION IT
D-322: **D-321's MEASURED SIZE FOR THE OPEN CONVENTION QUESTION 
D-325: **D-316's DIAGNOSIS OF ITS OWN MEASURED CORRECTION IS FAL
D-326: **D-312 IS AMENDED — IT NAMES THE WRONG REFERENT FOR THE 
D-330: **D-328's FLIP CLAUSE FIRED ON THE FIRST ROUND IT GOVERNE
```

It is append-only, which is **worse** than mutable for this purpose: the false line
stays and a reader who stops at it is misled.

**(b) Landed review reports drift** — M2, two of two.

**(c) One meta-claim has no external home and its nominal home contradicts it.**

```
$ grep -c "R19" docs/decisions.md
0
$ sed -n '114,117p' docs/experiments/matrix_M4_axisA_selection.md
4. **The relative-base inconsistency is recorded, not fixed** (F6): a relative
   `--config` resolves against `$ROOT` while a relative `--out` resolves against
   `$CALLER_PWD`. N-E inherits it; unlike N-Q it does not make it load-bearing
   for a refusal. It is OPEN.
```

Architect ruling **R19 has no ADR line**, and the selection record — the residual's own
home under D-331 — still reads *"It is OPEN."* U4 carries the closure at four sites, two
of which (the head change log at `:112` and the REVIEW STATUS row at `:188`) the strip
deletes. E5's *"a pointer to its home"* has nothing to point at for this claim; the only
pointer that would survive is to a review report, which is not a system of record for
architect rulings.

**Severity: MAJOR against E5, and against condition 1's re-homing clause.**

---

## M8 (MAJOR) — THE STRIP IS NOT A DETERMINATE OPERATION. MEASURED, IT REMOVES BETWEEN 265 AND 1047 LINES OF U4 DEPENDING ON WHICH READING OF "STATUS BLOCK" IS TAKEN, AND 383 PROTECTED RECORD LINES SURVIVE BOTH WITH THEIR ONLY LIVE/RECORD BOUNDARY DELETED.

**The measurement (obligation B).** I hand-classified U4 block by block against A's
five categories, keeping the title, the u-rev token, the carve marker, the `§n`
resolution rule, the theory-citation paragraph, and one pointer line where the content
has a real home.

**NARROW reading — only text whose sole subject is the document's own state:**

| block | lines removed |
|---|---|
| head: carve provenance (`15–19`, u-rev token kept) | 4 |
| head: the two selection-state bullets (`21–63`) | 43 |
| head: change log (`65–119`) | 55 |
| head: *"u-rev 8 is one round in several commits"* (`121–135`) | 15 |
| head: LABEL DISCIPLINE → pointer to D-311 (`136–141`) | 5 |
| head: citation-form prose + derived command (`143–171`) | 25 |
| head: REVIEW STATUS ×3 (`173–258`) | 86 |
| U4-A lineage table (`265–283`) | 19 |
| U4-A *"what this unit owes"* (`284–296`) | 13 |
| **NARROW TOTAL** | **−265 → remainder 1840** |

**BROAD reading — adding every block A's five categories can be read to reach:** the
§8 SELECTED-S-M fold (135), the §8 record stamp (32), §8.2's `FOLDED AT u-rev 6` (46),
§8.7's `FOLDED` paragraph (16), the §9 SELECTED-N-E fold (141), the §9 record stamp
(13), U4-Z's lead-in (12), U4-Z's B3 status prose (182), U4-Z's OPEN list (203), the
closing line (2) — **−782 more, total −1047 → remainder 1058.**

```
$ awk '{ if ($0 ~ /^>/) { if (start==0) start=NR; last=NR } else { if (start>0 && NR-last>1) \
  { if (last-start+1 >= 8) printf "%d-%d (%d)\n", start, last, last-start+1; start=0 } } }' \
  docs/experiments/U4_soundness_instrument.md
299-549 (251)
596-641 (46)
908-942 (35)
974-1383 (410)
1443-1451 (9)
1589-1600 (12)
```

**Why it breaks.**

*(i)* **The rule is a judgment, not a mechanical deletion.** The 782-line gap between
two honest readings is 37 % of the document. D's own failure mode is *"the strip
session itself edits documents and could drift mid-strip; mitigated: deletion-only
commits"* — the mitigation assumes the deletion set is determined by the rule. It is
not, and the session choosing among readings is doing exactly the interpretive work the
mitigation claims to have removed.

*(ii)* **On the narrow reading D does not answer the size question.** 2105 → 1840 is
−12.6 %. U4 is still 2.2× U2 and U3, still 87 % of the pre-carve document's mass in the
one unit, and *"reviewable in one sitting"* still has no instrument. The
recommendation's clause *"E4 says it also answers the size question"* is not supported
at the narrow reading, and at the broad reading it is bought by (iii).

*(iii)* **383 lines of protected record survive both readings and lose their label.**

```
$ printf "%d\n" $(( (549-435+1) + (1383-1116+1) ))
383
```

These are `THE RECORD OF THE EARLIER STATES` and `… OF §9`, which `matrix_U4R_REDTEAM.md`
K1 killed a whole option for proposing to cut — *"382 of the 638 lines … are NOT matrix
restatement. They are U4's own record, they have no other home, and D-331 protects them
by name."* They stay. What the broad strip removes is the two **record stamps** that tell
a reader which of the surviving lines are superseded. Nothing outside U4 can supply
that: `git log` gives commits, not the live/record status of a paragraph. `wp15b_U4_REVIEW_urev8.md`
MINOR 6 turns on precisely this dependence — *"a reader deciding whether to trust or to
correct the paragraph consults the stamp, and the stamp is silent"* — and *"this is how
S-E survived at that site for three revisions."* **On the broad reading, 36 % of the
stripped unit is record a reader can no longer identify as record.**

**Severity: MAJOR against A and D, and the measured ground for flip clause 1's failure
(M6d).**

---

## M9 (MAJOR) — E3's SECOND HALF IS FALSE. THE NODE PROTOCOL WAS NOT CLEAN: ITS FIRST POST-CARVE REVIEW FOUND A MAJOR IN NORMATIVE §5.3, A GAME-RULES DEFECT CARRIED VERBATIM THROUGH THE CARVE AND MISSED BY EVERY PRIOR ROUND.

**The claim attacked, verbatim:**

> Normative content: node protocol clean five rounds; S-M and N-E selections ADR'd
> (D-323, D-329); gates defined post-B3.

**Contradicting evidence — `wp15b_U2_REVIEW.md`, the first of those five rounds:**

```
$ sed -n '226p' docs/experiments/wp15b_U2_REVIEW.md | cut -c1-260
| F2 | 2 | MAJOR | §5.3's D-257 pairing fixture ("two disjoint sealed five-stone P1 rows")
has P1 = 10, an even count unreachable at any turn boundary per rule 3 — same defect class
as MAJOR 8, not covered by the owner table's M4/M6 OPEN item, pre-existing (carried verbatim,
not introduced by the carve) | Verified (pistol-core reproducer, run and removed) |
```

§5.3 is *"The four generation rows"* — the normative protocol itself, not a head, a
change log or a status block. The defect is a position no legal game reaches, in a
fixture the protocol registers, verified with a `pistol-core` reproducer.

**And a second normative-text instance is still OPEN at HEAD** — `wp15b_U2_REVIEW_urev4.md`
G2:

```
$ grep -n "168 030\|343 344\|145 158" docs/experiments/U2_node_protocol.md \
    | awk -F: '$1>300 && $1<870' | cut -c1-76
317:`covers` predicate — and MEASURED over **145 158** playout positions and
638:| **M5-E — ONE `can_win_this_turn` and ONE `blocking_covers`; step 2 rea
650:> `cover.rs:145` and `cover.rs:177`, and MEASURED to agree on 145 158 pl
651:> positions and on 343 344 side-and-budget comparisons with zero disagre
864:  168 030 comparisons with the `Impossible` branch reached at every budg
```

The same identity's comparison count is stated four times — three MEASURED at
**343 344** in §5.2 and §5.6, once unmarked at **168 030** in U2-Z's *"handoff this unit
carries to WP-1.6"*, a substantive design section that is not a head, change log,
lineage, status block or self-completeness universal. **A/D's strip does not touch it.**

**Severity: MAJOR against E3's second half.**

---

## M10 (MAJOR) — MISSING ROWS. AT LEAST FIVE, ONE OF WHICH DOMINATES D ON THE MATRIX'S OWN GROUNDS; AND D-334 ENUMERATES FOUR ROWS A SUCCESSOR FIELD OWES, OF WHICH THIS FIELD CARRIES NONE.

**D-334's enumeration, at HEAD:**

> round 2's report enumerates six things it must contain, of which two are rows no
> field has yet carried: round 1's `(e)` as round 1 actually wrote it, and **the
> DERIVED-ENUMERATION row on the landed `7dfd047` precedent — a rule plus a derived
> command replacing every hand-held enumeration and completeness universal in the
> unit, which round 2 measured to be the only candidate whose prevention claim is a
> mechanism rather than an assertion.**

```
$ sed -n '1003,1019p' docs/experiments/matrix_U4R_REDTEAM_round2.md | grep -n "^[0-9]\." | cut -c1-84
1:1. **Round 1's (e) as round 1 wrote it** — `(a) + extraction of the 382 record lin
4:2. **The DERIVED-ENUMERATION row (K5)**, on the landed `7dfd047` precedent: a rule
9:3. **A §8.7-scoped row.** §8.7 is fifty-one lines at 3.92 findings / 100 lines — t
13:4. **A row that changes what a REVIEWER is asked to check, and nothing structural
```

META-1 carries **none of 1, 2 or 3**, and carries 4 only bundled into D's composite.
Item 2 is the row round 2 **measured** to be the only mechanism in the field, and the
owner table's own u-rev 6 append is an executed instance of it (a hand-maintained
`now` column deleted and replaced by a derived command). It should be the first row of
any successor field and it is absent.

**Five further rows the field does not carry:**

**R1 — U4-ONLY PILOT.** Strip U4 alone, re-review, then decide for the other three.
Dominates D on risk at strictly lower cost: D-332 (R17) already requires multi-unit
repairs to land serially, and D-337 exists because a matrix's subject moved under it.
One unit is one subject.

**R2 — REVIEW-CONTRACT-ONLY, no repair.** Round 2's item 4. The reviewer's inventory
becomes the deliverable, findings are recorded and **batched to the architect** rather
than repaired in-session. This is distinct from B, which repairs until stable and which
E6 says does not converge. It is the only row that attacks generator (3),
intra-round re-authoring, by construction: a round that authors nothing cannot
re-author.

**R3 — DERIVED-ENUMERATION** (D-334's item 2, above).

**R4 — SELF-STATE GATE.** A `tools/` check that each carve document's head u-rev equals
its foot u-rev and that its REVIEW STATUS names the latest landed report for it.
**MEASURED: it finds two unrecorded live instances in six documents in under a second
(M3), against the citation gate's zero (K2).** It costs less than the citation gate,
its criterion is one the defect class can falsify, and it is not in the field.

**R5 — RETIRE, don't edit.** `WPQ_seed.md` is marked *"not reviewable"* by the owner
table's own §11 table, and U1 has a landed PASS. Removing 556 lines from the review
surface by retirement costs no authoring at all. The dispatch names this row; the
field omits it.

**R6 — C′ WAIVE-AND-MARK, and it DOMINATES D.** Keep every document. Answer every meta
finding by **striking the sentence through and appending a fixed `WITHDRAWN AS FALSE AT
u-rev N` token** — never by rewording, never by re-deriving. Reviews stay at full
scope.

*It is not hypothetical: the project has executed it three times and a fresh reviewer
prescribes it as the fix for a fourth.*

```
$ sed -n '230p' docs/experiments/U4_soundness_instrument.md | grep -o "THAT UNIVERSAL IS WITHDRAWN AS FALSE AT u-rev 8"
THAT UNIVERSAL IS WITHDRAWN AS FALSE AT u-rev 8
$ sed -n '483,485p' docs/experiments/wp15b_U4_REVIEW_urev8.md
**Fix scope.** Strike it as the three siblings were struck, with the disposition stated
(re-derivation was the instrument, `wp15b_U4_REVIEW_urev7.md` MAJOR 2/3/4 found what it
did not reach), and replace it with nothing.
```

**Against D's own grounds:** it is deletion-shaped (D's condition 1's whole point); it
removes the false claim (C's failure mode answered); it needs no strip session, no
citation gate, no SHELL_CHECKLIST round and no re-review of four rewritten documents;
it preserves the record a future reader needs, which is what K1 of the U4-R red team
killed an option to protect; and it is incremental, so it cannot manufacture a
mid-strip drift across four units at once. **It satisfies D's grounds while owing less,
which is this project's stated definition of a KILL by missing row** (M3's four rows,
M4's closed-enum row, U4-R's `(e)` and `(c′)`).

*The one thing C′ does not do is shrink U4, and E4 is the only E-item that reproduces.
That is the honest argument for D over C′, and the matrix does not make it.*

**Severity: MAJOR — KILL-grade on R6 alone.**

---

## M11 (MAJOR) — §8's AND §9's FOLDS RESTATE THE TWO SELECTION RECORDS IN NORMATIVE SECTIONS, 276 LINES, UNCHARGED BY TEN ROUNDS. EITHER THEY ARE META-TEXT, IN WHICH CASE D DELETES THE UNIT'S ONLY SPECIFICATION OF ITS TWO GATES; OR THEY ARE NOT, IN WHICH CASE E3 IS FALSE AGAIN.

**The evidence.** `matrix_M3_selection.md` has its `## FIVE REGISTERED CONDITIONS` and
`## WHAT THIS SELECTION DOES NOT DECIDE`; U4:354–402 carries both, reworded, inside
§8's fold. `matrix_M4_axisA_selection.md` has `## CONDITIONS THAT RIDE WITH THIS
SELECTION`, four items; U4:1037–1060 carries all four, reworded, inside §9's fold.

```
$ grep -n "FIVE REGISTERED CONDITIONS\|CONDITIONS THAT RIDE" \
    docs/experiments/matrix_M3_selection.md docs/experiments/matrix_M4_axisA_selection.md
docs/experiments/matrix_M4_axisA_selection.md:102:## CONDITIONS THAT RIDE WITH THIS SELECTION
docs/experiments/matrix_M3_selection.md:88:## FIVE REGISTERED CONDITIONS, each a red-team finding that would otherwise ride free
$ sed -n '354p;1037,1038p' docs/experiments/U4_soundness_instrument.md | cut -c1-80
> **THE FIVE REGISTERED CONDITIONS BIND. Each is a red-team finding that would o
> 1. **THE `config` LINE'S DIGEST IS `$3`, NOT `$4`.** That line is `config <pat
>    <sha>` — three fields. The four-token reasoning belongs to the differently 
```

They are **reworded copies, not marked quotations** — U4's condition 1 appends *"This
retires S-E's … as an IMPL instruction"*, its condition 2 appends *"This unit cites it
nowhere and may not start"*, its condition 5 says the probe *"is committed in full
inside the selection record"* where the record says *"reproduced in full below"*. D-331
clause (1): *"where a claim has landed in `docs/decisions.md` or in a selection record,
THAT is its home and no unit may restate it"*; clause (2): *"every other occurrence is
a POINTER that names WHERE and does not repeat WHAT"*. D-331 exempts marked quotation;
these are not marked quotations.

**276 lines** (135 + 141), in §8 and §9, the unit's two principal normative sections,
and **no round in ten has charged them** — including the two rounds that built claim
inventories, because both inventories are scoped to U4-Z.

**The dilemma.** If the folds are *"status blocks"* under A/D, the strip deletes the
only place in the design of record that says **what the differential gate is** and
**what the config seam is**, and every IMPL constraint riding with them; the reader is
sent to two matrix records that D-331 makes the home but that no unit then points at
from its own gate section. If they are not status blocks, they are 276 lines of the
recorded generator sitting in normative text and **E3 is false a third way**.

**Severity: MAJOR, and it is the cleanest single demonstration that the matrix's
meta/normative partition does not exist in the document it governs.**

---

## MINOR

**m1.** E1 says self-completeness *"failed U3 four rounds"*. Measured, it is four
findings across **three** reports — `wp15b_U3_REVIEW_urev4.md` A and B,
`_urev5.md` C, `_urev6.md` D (`grep -o '^### [A-Z]\.'`). And MAJOR B is a
restatement, not a self-completeness claim — `wp15b_U4_REVIEW_urev8.md` MAJOR 2 already
says so: *"D-331's diagnosis itself lumps a genuine restatement (MAJOR B) with two
claims that are not restatements."* E1 inherits the mis-sort from D-335 without the
correction that was landed one round earlier.

**m2.** No E-item and no option cell carries **MEASURED** or **ESTIMATED**. The
provenance comment discloses this. Disclosure is not the remedy CLAUDE.md's matrix
clause names, and D-291 makes an unmarked estimate that could have been measured in
seconds a finding in its own right. E4 and E6 are numeric; E4 measures in one command
and E6 in seven.

**m3.** *"Reviewable in one sitting"* carries load in the recommendation through E4
(*"E4 says it also answers the size question"*) and has no instrument — recorded as
undefined by `matrix_U4R_restructure.md:265`, `matrix_U4R_restructure_rev2.md:169` and
`section_owner_table.md:414`, which says in terms that *"the question is unanswerable
as posed"* and that *"Defining the property is the architect's, and it is the prior
question to any revision 3."* This matrix is that revision and does not define it.

**m4.** *"~10 rounds"* is unmarked and low. Measured: 17 landed review reports plus 2
DECISION-RED-TEAM rounds against the U4-R micro-matrix.

---

# PER-OPTION SURVIVAL VERDICTS

| row | verdict | reason |
|---|---|---|
| **A META-BAN** | **FALLS** | Its whole ground is E3 (K1, M9, M11) and E5 (M7). Its literal rule turns a CI gate red on line 3 of every document (K5), and it is not a determinate operation (M8) |
| **B INVENTORY** | **SURVIVES WOUNDED** | Its cost cell is the field's most honest. But E2 over-states the instrument (M1) and the instrument itself has produced the generator twice out of two with nothing above it (M2). Selectable only paired with a check on the inventory's own summary |
| **C RE-SCOPE** | **SURVIVES WOUNDED, AND IS UNDERRATED** | Its rot argument is real but is already answered in-tree by the strike-through convention this project has executed three times and which a fresh reviewer prescribes as a fix (M10-R6). As written it waives without marking, and that does rot; marked, it becomes C′, which dominates D |
| **D = A + review contract** | **FALLS** | K1, K2, K3, K4, K5, plus M8 and M11. Its premise is false, its one mechanism is vacuous against the recorded defect, its safety clause had already fired, its taxonomy's source is not in the tree, and its condition 1 breaks a gate |
| **NULL** | **RESURRECTED — SURVIVES WOUNDED** | Its death certificate is E6, which reproduces on no denominator (M4). The divergence *direction* reproduces on all seven, so it is wounded, not dead; and D-334 records the standing attack against exactly this form of exclusion |

---

# MAY A SELECTION BE TAKEN FROM THIS FIELD?

**NO. AND NO ROW IN IT IS SELECTABLE.**

The recommended row falls on five KILLs. A falls with it. The two rows that survive
wounded — B and NULL — survive only in the sense that nothing here kills them, and
neither is the answer the round was convened to find: B is scored non-convergent by the
matrix's own E6, and NULL is what the project has already been doing. C is the one row
whose repair is cheap, and the repair (C′) is a row the field does not carry.

**What a revision 2 owes, minimally:** a re-grounded E3 that states the measured
normative-text instances rather than denying them; E1's source named as a file in the
tree or the item withdrawn; MEASURED/ESTIMATED marks on every number; C′ and R2, R3,
R4 authored as rows; the citation gate either dropped or replaced by a criterion the
defect class can falsify; condition 1 amended to exempt the carve marker by name; and
flip clause 1 replaced by the homing test in M6.

---

# THE STRONGEST ATTACK SURVIVING AGAINST EACH SURVIVING OPTION

*Written to be quoted VERBATIM in an ADR line. Assembling one of these from parts is a
recorded residual under D-329/D-333, so each stands as one paragraph.*

**AGAINST B INVENTORY:**

> B is the only row whose instrument has ever caught anything, and it is also the only
> row that puts an unreviewed artefact at the centre of the process. Both landed claim
> inventories ship a headline count of their own table that their own table falsifies —
> fifty-four rows under a stated thirty-four, and eleven failing rows under a stated six
> — which is D-335's second generator committed inside the one thing measured to detect
> it, by two independent fresh contexts, uncaught by anyone including the reviewer who
> read the earlier inventory closely enough to decline to reuse it. B pays two to four
> more full rounds at 2105 lines to run an instrument that has never been checked, in a
> document class this project reviews last and never re-reads, and its own matrix cell
> concedes the trajectory does not converge.

**AGAINST C RE-SCOPE:**

> C's failure mode is stated against the wrong version of C. Waiving a false sentence
> silently does rot, and the U1 lesson is real — a falsehood rode five documents and
> nine rounds because nothing marked it. But this project has already executed the
> marked form three times inside U4's own REVIEW STATUS tables, and the fresh reviewer
> of u-rev 8 prescribes it by name as the fix for MAJOR 5: strike the sentence, state
> the disposition, replace it with nothing. What survives against C is narrower and
> harder: marking is still an edit, every edit is an occasion for generator three, and
> C shrinks nothing — U4 stays at 2105 lines against a property this project has
> recorded three times as having no instrument, so C answers the falsehoods and leaves
> the only question the matrix's own E4 measures.

**AGAINST THE NULL ROW:**

> The null row is not dead, because the number that kills it does not exist: seven
> denominators over the landed verdict blocks — U4 alone, U4 by blocking-plus-major,
> all units per round, all units by blocking-plus-major, the three units without U1,
> U3 and U4 together, and cumulative — yield no series matching twenty-three, twenty,
> twelve, fifteen, and the word "zero PASS" is false of a corpus holding three landed
> PASSes. What survives is the direction and not the series: on all seven readings the
> tail turns back up, so the null row stands convicted of divergence and acquitted of
> the arithmetic, which is the same shape D-334 recorded when the last field in this
> work package excluded its null row on a criterion its own column did not measure and
> the project then executed that row anyway.

---

# FINDINGS ATTEMPTED AND REJECTED, WITH THE REPRODUCER

**RJ1 — "The D-337 freeze was broken."** REJECTED. `git diff b683d48..HEAD` over the
six subject documents returns nothing; the only change is the dispatch file.

**RJ2 — "The strip strands revision-7 MAJOR 9's IMPL-gate disposition."** REJECTED.
U4:256's REVIEW STATUS row is the only U4 site, but the claim has a home in normative
text elsewhere: `sed -n '908,909p' docs/experiments/U2_node_protocol.md` → *"**RULE 5 IS
UNDISCHARGED FOR THE NODE PROTOCOL ITSELF** (revision-7 review MAJOR 9, and the
superseded §17's own list)."* A pointer works.

**RJ3 — "D-336 misdescribes its own instance."** REJECTED, and I record the check
because obligation H asks for it. `grep -rn "20 live-line occurrences"` now returns five
files, so D-336's *"the string occurs in the repository only as the quotation attributing
it"* is false at HEAD — but D-336 attributes that census to what round 1 established and
then **records the compounding itself**: *"true when round 1 ran the command, false once
round 1's report landed."* The account is exact and self-aware. `705` vs `707` also
checks out against `matrix_U4R_measurements.md:37–38`, which states both with the
reconciliation.

**RJ4 — "D-337 misdescribes its own instance."** REJECTED. All five named commits exist
and every one touches U4:
`for c in d328d1d 75ae04e 78b4876 b9f4aea 823004a; do git show --stat --format= $c | grep -c U4_soundness; done` → `1 1 1 1 1`.

**RJ5 — "E4 does not reproduce."** REJECTED, emphatically. `git show
0f49c90:…U4_soundness_instrument.md | wc -l` → 1886; `wc -l` at HEAD → 2105; difference
+219; `git show 6feb40a:…wp15b_design.md | wc -l` → 1975. Every figure in E4 is exact.
It is the one item in the evidence base that survives contact with the tree unchanged,
and the matrix should be read as resting on it and on nothing else.

**RJ6 — "The census pin's restatement scan is a counter-example to E2 in its own
right."** REJECTED as stated. `restatements_outside` in
`crates/pistol-solver/tests/wp15b_census.rs:717` is a real mechanical restatement
detector and it is GREEN, but its own doc comment concedes it sees four-decimal figures
only — *"narrower than 'no number is restated'"* — and I found no landed record of it
turning red on a real restatement rather than on a reviewer's deliberate corruption. It
is evidence that a mechanism is possible (and it grounds R3 and R4), not evidence that
one has caught an instance.
