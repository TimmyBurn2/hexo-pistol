# REVIEW-design — `docs/experiments/U4_soundness_instrument.md`, u-rev 8

**Pinned revision reviewed: `a2b50bf` (the unit's own content last changed at `3543a7f`).**

```
$ git rev-parse HEAD          # at entry
a2b50bf30b087f6f1eb9934281ad88b1684ef2d5
$ git rev-parse HEAD          # at exit
4fd88ec01f6accc509f1386e70946a7fc3a5692b
$ git status --porcelain
(no output, at entry and at exit)
```

**Does the pinned revision still match HEAD? NO — but MY SUBJECT DID NOT MOVE.** HEAD
advanced from `a2b50bf` to `4fd88ec` during this review (`4fd88ec` is
*docs(experiments): the owner table reaches u-rev 6 …*, a `section_owner_table.md`-only
commit). The subject is byte-identical across the move:

```
$ git diff a2b50bf HEAD -- docs/experiments/U4_soundness_instrument.md
(no output)
$ git log --oneline -1 -- docs/experiments/U4_soundness_instrument.md
3543a7f docs(experiments): the carve-provenance clause said revision 7 was NEVER REVIEWED …
```

Everything below is read at the subject's `3543a7f` text. Where a finding turns on
another file, I say which revision I read it at.

**Context was fresh.** I did not author this unit, any of its repairs, either matrix,
any red-team round, the U4-R micro-matrix, or any earlier review. Read in this order:
`CLAUDE.md` in full; `docs/decisions.md` D-305, D-309, D-311, D-316, D-320…D-325,
D-328…D-334 in full; `wp15b_U4_REVIEW_urev7.md` in full; the whole of U4;
`matrix_U4R_restructure.md`, `matrix_U4R_REDTEAM.md`,
`matrix_U4R_restructure_rev2.md`, `matrix_U4R_REDTEAM_round2.md`;
`matrix_M4_axisA_selection.md`; `wp15b_U3_REVIEW_urev4.md`, `_urev5.md`, `_urev6.md`;
`wp15b_trackC_R19_REVIEW_impl.md`; `section_owner_table.md`;
`restructure_matrix_15b.md`, `restructure_selection_15b.md`;
`tools/SHELL_CHECKLIST.md` item 11; `tools/baseline_snapshot.sh`.

**Reproducer discipline.** Every finding carries a command and its real output. I
edited no repository file, staged nothing, committed nothing, and ran no git write
command. No worktree was created.

---

## VERDICT: **FAIL**

**1 BLOCKING, 7 MAJOR, 7 MINOR.**

## MEASURED SIZE (dispatch item 7)

```
$ wc -l docs/experiments/U4_soundness_instrument.md
2105 docs/experiments/U4_soundness_instrument.md
```

**2105 lines.** At the reviewed revision `a2b50bf`, `section_owner_table.md` §11's
table recorded U4 at **1886** (fifth column, MEASURED at `161e6d3`) — so the unit grew
**+219 lines** in a round convened to shrink its status matter (`git diff 0f49c90
a2b50bf --stat` → `254 insertions(+), 35 deletions(-)`). During this review the owner
table reached u-rev 6 (`4fd88ec`) and now records 2105 at `a2b50bf`, so its row is
current again.

**Is the unit's own account of that honest? YES, and I credit it.** U4-Z's D-334
bullet states *"the repairs are NET ADDITIVE, because each one discloses what it
replaced"* and *"this round made it sharper rather than softer"*, and it states no
number — which is rule 9 and the unit's own standing-duty design. `grep -n "wc -l\|line
count\|[0-9]\{3,4\} lines"` returns only U4:233 and U4:257, both of which are the
statement that no count is asserted. This is the one status-surface discipline in the
unit that has now worked twice running.

---

# WHAT THE ROUND GOT RIGHT, STATED FIRST

- **MINOR 8 is fully discharged.** The blockquote at U4:1073–1083 is now
  character-identical to `matrix_M4_axisA_selection.md:127–138` including the closing
  sentence, verified by normalised comparison (`IDENTICAL: True`).
- **MINOR 5 is fully discharged.** U4-M item 1's parenthetical is re-taken with the
  complete `grep` output pasted, and it reproduces exactly at HEAD (lines 543/559/592
  for the engine flag, 559 the comment, 182 the literal). The re-take was necessary —
  the script moved at `63eac4c` — and the unit says so.
- **MAJOR 4's substance is discharged.** T2 now has a disposition. (Three factual
  errors were introduced with it; MINOR 1 below.)
- **MAJOR 3's substance is discharged at U4-Z:1956**, correctly re-attributed to
  `matrix_M4_axisA_selection.md` condition 4, and the R19 closure claim is real and
  independently reviewed (`wp15b_trackC_R19_REVIEW_impl.md`: *"**PASS.** No BLOCKING or
  MAJOR finding. Three MINOR findings"*, and it ratifies by name that R19's *"one
  documented, consistent resolution base for every caller-supplied path"* answers the
  residual while `--config` stays out of scope). (The head re-introduces the
  misattribution; MAJOR 1 below.)
- **The six citation conversions are exactly six, and both halves of each are true.**
  ```
  $ git show 0f49c90:docs/experiments/U4_soundness_instrument.md | grep -on "\*\*U[0-9]\*\* (u-rev [0-9])"
  70:**U3** (u-rev 4)
  155:**U3** (u-rev 4)
  259:**U2** (u-rev 3)
  674:**U3** (u-rev 4)
  756:**U3** (u-rev 4)
  1702:**U3** (u-rev 4)
  $ git show 13621d3:docs/experiments/U3_tier_t.md | sed -n '15p'
  **u-rev 6.** Carved from `docs/experiments/wp15b_design.md` §6, §10 and §12 items
  $ git show f0ae14c:docs/experiments/U2_node_protocol.md | sed -n '15p'
  **u-rev 5.** Carved from `docs/experiments/wp15b_design.md` §2, §3, §5 and §14 at
  $ git merge-base --is-ancestor 13621d3 HEAD && git merge-base --is-ancestor f0ae14c HEAD && echo BOTH_ANCESTORS
  BOTH_ANCESTORS
  ```
  Eight sites now carry the landed form; six are conversions and two (U4:247, U4:1933)
  are new text written in the form. The cited sections support the claims (U3 §10 is
  the config table and the *"one place the count is stated"* ruling; U2 §5.3 is the
  phase-derived budget). **The bound is honestly stated and is not the universal U3's
  u-rev 5 was failed for** — this is the round's best single decision, and the
  landed-SHA form has already proved itself: U3 reached u-rev 7 and U2 u-rev 6 after
  this unit landed, and not one of the eight went stale.
- **The record stamp was respected.** No hunk of `git diff 0f49c90 a2b50bf` lands
  inside a block the §8 stamp lists as RECORD. The §8.7 blockquote the round edited is
  **not** in the RECORD enumeration (U4:447–455), and the unit's own u-rev 6 precedent
  (MINOR 9's repair four lines above, made *"because it is carve prose"*) supports the
  justification. **Verified and not a finding**, with one gap charged as MINOR 6.
- **The census gate passes**, its own log output:
  ```
  $ cargo test -p pistol-solver --test wp15b_census
  running 6 tests
  test a_document_quoting_the_carve_marker_is_not_a_carve_member ... ok
  test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
  test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... ok
  test wp15b_census_reproduces_the_registered_populations ... ok
  test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
  test the_census_pin_reads_every_carved_document_it_names ... ok

  test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 3.63s
  ```
- **The D-334 fold's hard facts check out** against both matrices and both red teams:
  `4d84694` / `97aa4d6` are the two authoring commits (`git log --diff-filter=A`), and
  the two attack reports close with *"**Six KILLS, eight MAJOR, eleven MINOR/WOUND.**"*
  and *"**Six KILLS, ten MAJOR, eleven MINOR/WOUND.** … **NO SELECTION MAY BE TAKEN
  FROM THIS FIELD.**"* — matching U4:1905–1907 exactly. The claim that the repairs
  executed the null row while the matrix awaited attack is round 2's own decisive
  finding, correctly attributed and not softened. One understatement is charged as
  MINOR 3.
- **"No review has been dispatched against any intermediate commit of this round" is
  TRUE.** `grep -rln "d328d1d\|75ae04e\|78b4876\|b9f4aea\|823004a" docs/` returns
  `matrix_U4R_REDTEAM_round2.md`, `wp15b_U3_REVIEW_urev6.md`, `wp15b_U2_REVIEW_urev5.md`
  and `docs/decisions.md` — a decision red team of a matrix, and two sibling-unit
  reviews recording that HEAD moved under them. None is a review of U4.

---

# FINDINGS

## BLOCKING

### 1. The unit's own closing line still says **"u-rev 7"**, and three further status surfaces are one round behind — the review history stops at u-rev 6 while u-rev 7 has been reviewed and FAILED

**The claim reviewed — U4:2105, the document's last line:**

> *U4, **u-rev 7**. A carve, not a revision. … **IMPL has not started, and this u-rev
> has not been reviewed; u-rev 6's review FAILED (1 BLOCKING / 2 MAJOR / 4 MINOR)** and
> this u-rev is its answer.*

**Contradicting evidence.** The head, 2104 lines above, reads `**u-rev 8.**`; the same
head says *"u-rev 7 WAS REVIEWED AND FAILED"* with the report and the SHA. The closing
line was re-written at **every** previous bump and was not re-written at this one:

```
$ git log --oneline -3 -L 2105,2105:docs/experiments/U4_soundness_instrument.md
0f49c90 …
-*U4, u-rev 6. …*
+*U4, u-rev 7. … IMPL has not started, and this u-rev has not been reviewed; u-rev 6's review FAILED (1 BLOCKING / 2 MAJOR / 4 MINOR) and this u-rev is its answer.*
7358a07 …
-*U4, u-rev 5. …*
+*U4, u-rev 6. … IMPL has not started, and this u-rev has not been reviewed.*
35aab95 …
-*U4, u-rev 4. …*
+*U4, u-rev 5. …*
$ tail -1 docs/experiments/U4_soundness_instrument.md | cut -c1-24
*U4, u-rev 7. A carve, n
```

**Three further surfaces carry the same staleness.**

*(a)* The OPEN list's review-history bullet, U4:2095–2101:

```
$ sed -n '2095,2099p' docs/experiments/U4_soundness_instrument.md
- **No REVIEW-design has run against this text at THIS u-rev** (U4-A). **u-rev 6 was
  reviewed and FAILED** (`docs/experiments/wp15b_U4_REVIEW_urev6.md`, pinned revision
  `7358a07`, **1 BLOCKING / 2 MAJOR / 4 MINOR**), as u-rev 5 had before it
  (`docs/experiments/wp15b_U4_REVIEW.md`, `35aab95`, 3 BLOCKING / 3 MAJOR / 5 MINOR).
  **u-rev 7 is the repair of the u-rev 6 report plus the M4 axis-A selection fold**, and
```

The bullet enumerates the unit's failed reviews and stops at u-rev 6.
`wp15b_U4_REVIEW_urev7.md` — the report this entire round exists to answer — is absent
from it, and the bullet says u-rev 7 is the current repair.

*(b)* **U4-A, the lineage table, has no row for the u-rev 7 review.** Its last row is
`| REVIEW-design, **this unit** | u-rev 6, `7358a07` | **FAIL** …` (U4:281). U4-A is
the table the u-rev 6 round expressly extended for this purpose — its own BLOCKING 1
disposition reads *"U4-A gains a row for each of the five DECISION-RED-TEAM rounds and
for the u-rev 5 review"* (U4:245). Neither the u-rev 7 review nor either U4-R red-team
round is in it.

*(c)* U4:257, the u-rev 5 table's owner-table row: *"its U4 row was current for u-rev
6. **u-rev 7 supersedes that measurement**"* — at the reviewed revision the owner
table's latest measurement (`161e6d3`) was current for u-rev **7**, and the unit was at
u-rev **8**. Both halves are one u-rev behind.

**Why it breaks.** This is **BLOCKING 1 of `wp15b_U4_REVIEW.md` verbatim** — *"the head
block, U4-A, the change log, §8's stub and **§9's closing paragraph** still describe the
u-rev 1 state; six named sites"* — recurring at the same class of surface, two rounds
after the unit rebuilt those surfaces for it. Concretely: a reader who starts at the
foot is told this is u-rev 7 and that its answer is to the u-rev 6 report; a reader who
consults the OPEN list or U4-A to learn the unit's review history is not told that
u-rev 7 was reviewed and failed 0/4/4. The unit's own binding rule is that *"a citation
of another unit names the unit AND the u-rev cited"* and that reviews of superseded
revisions do not transfer; a document that names itself two different u-revs cannot
support either.

**Fix scope (not mine to apply).** Re-write U4:2105 for u-rev 8; add the u-rev 7 review
to the U4-A lineage table and to the OPEN-list review-history bullet; correct U4:257's
two u-rev numbers. Then read the fix against MAJOR 7 below, because the generator is
that "u-rev 8" is a round rather than a revision.

---

## MAJOR

### 1. The head's change log says **"`D-329`'s relative-base residual"** — the exact misattribution MAJOR 3 charged, re-introduced by a later commit of the same round, 1845 lines from the repair that says the opposite

**The claim reviewed — U4:111–112, the u-rev 8 change-log entry:**

```
$ sed -n '110,112p' docs/experiments/U4_soundness_instrument.md
  **`D-333` (R18)** rules on N-E's unattacked-in-its-own-right residual and is folded
  at that bullet in U4-Z. **`D-329`'s relative-base residual is CLOSED**, not by this
  unit but by architect ruling R19 at `63eac4c`, whose REVIEW-impl PASSED at `d59f0de`.
```

**Contradicting evidence — D-329 records no such residual, at HEAD:**

```
$ L=$(grep -n "^D-329:" docs/decisions.md | cut -d: -f1); echo $L
703
$ sed -n '703p' docs/decisions.md | grep -o "relative\|CALLER_PWD\|relative-base" | sort | uniq -c
(no output)
```

**And the unit's own repair, 1845 lines below, says so in bold:**

```
$ sed -n '1956p' docs/experiments/U4_soundness_instrument.md | grep -o "it is recorded by the SELECTION RECORD and not by D-329"
it is recorded by the SELECTION RECORD and not by D-329
```

The residual's home is `matrix_M4_axisA_selection.md` condition 4 (*"**The relative-base
inconsistency is recorded, not fixed** (F6)"*, `:114–117`), which is exactly what
U4-Z:1956 now says and exactly what U4:111 now denies.

**Timeline — it was written after the repair, not before it.** `78b4876` made the
U4-Z repair; `b9f4aea` wrote the head's change-log entry. `git log --oneline --reverse`
puts `78b4876` before `b9f4aea`. So the round repaired the misattribution and then
re-authored it at a second site in the next commit.

**Why it breaks.** *(i)* It is the **fourth** occurrence of the class D-331 was landed
for, and the second occurrence of this specific defect. D-331's flip clause reads
*"Flips if a document produced under this law nonetheless ships a claim a landed ADR
line falsifies"*. *(ii)* It falsifies the round's own summary claim at U4:1912 (*"the
eight findings are DISCHARGED"*) and, through it, D-334's record that *"U4's eight
findings are DISCHARGED by repair at u-rev 8"* — a landed ADR line now rests on a
statement one of whose eight is not discharged at every site. *(iii)* The change log is
status matter, which D-331 clause (3) says *"may not restate the finding's content or
the repair's content"*; had it pointed rather than restated, the defect could not have
been re-manufactured.

**Fix scope.** One possessive: *"the relative-base residual recorded at
`matrix_M4_axisA_selection.md` condition 4"*, or delete the clause and let the U4-Z
bullet be its one home.

---

### 2. U4-Z asserts that D-331 **"does not reach"** and **"is silent about"** this class — D-331's own ground section names two of the instances U4 cites, by review citation, as instances the law is built on

**The claim reviewed — U4:1927–1934:**

> - **D-331 DOES NOT REACH A NEIGHBOURING CLASS THAT HAS NOW FAILED THREE UNITS, AND
>   THAT IS THE ARCHITECT'S.** The CLAIM-HOME law requires every claim to have one home
>   and every other occurrence to be a pointer. **A UNIVERSAL ABOUT THIS DOCUMENT'S OWN
>   STATE IS AT ITS HOME AND RESTATES NOTHING**, so **the law is silent about it** — and
>   it is exactly what MAJOR 1 and MINOR 6 of `wp15b_U4_REVIEW_urev7.md` are …, and what
>   **MAJOR A, B and C of U3** (u-rev 6, landed `13621d3`) are …

**Contradicting evidence — D-331 at HEAD, in its own ground section:**

```
$ sed -n '707p' docs/decisions.md | grep -o "U3's u-rev-4 repair falsified its own B7 site-table completeness claim on the day it wrote it and simultaneously stated \*\"U2 is at u-rev 2\"\* against a u-rev 3 that was already an ancestor commit and had already failed its own review (\`wp15b_U3_REVIEW_urev4.md\`, MAJOR A and B)"
U3's u-rev-4 repair falsified its own B7 site-table completeness claim on the day it wrote it and simultaneously stated *"U2 is at u-rev 2"* against a u-rev 3 that was already an ancestor commit and had already failed its own review (`wp15b_U3_REVIEW_urev4.md`, MAJOR A and B)
```

D-331 names MAJOR A and MAJOR B **by review citation, in the paragraph that grounds the
law**, and its diagnosis — *"every one of those defects is a SECOND COPY of content
whose FIRST copy is correct and lives somewhere else"* — describes MAJOR B exactly (the
second copy is U3's stale *"U2 is at u-rev 2"*; the first, correct copy is U2's own
head). A law whose text quotes two of three instances is not "silent about" them.

**And the sibling unit was failed for this identical sentence one round earlier.**
`docs/experiments/wp15b_U3_REVIEW_urev6.md` finding **E**, landed at `8c597cd`:

```
$ sed -n '173p' docs/experiments/wp15b_U3_REVIEW_urev6.md
### E. The new OPEN bullet claims D-331 "does not reach" MAJOR A/B/C and "is silent about them" — contradicted by D-331's own text, which names MAJOR A and MAJOR B as grounding instances of the class it is built on
```

**Why it breaks.** *(i)* It is a claim in this unit falsified by a landed ADR line at
HEAD — the exact test the dispatch set, and the exact condition of D-331's own flip
clause. *(ii)* Its consequence is the one that report names: an architect reading
*"D-331 … is silent about"* without re-reading D-331 could amend it redundantly for a
gap D-331 already claims to cover, or miss that D-331's diagnosis itself lumps a
genuine restatement (MAJOR B) with two claims that are not restatements. The accurate
framing is narrower and is available: D-331's *ground section* discusses all three;
what is open is whether its *binding mechanism* (clauses 1–4) reaches an
originally-authored, non-copied completeness claim.

**Secondary, in the same sentence — the pointer names the wrong home.** *"MAJOR A, B
and C of **U3** (u-rev 6, landed `13621d3`)"* attributes three review findings to a
revision of the unit reviewed. MAJOR A and B are findings of
`wp15b_U3_REVIEW_urev4.md` (`grep -n "^### [AB]\."` → lines 32, 59) and MAJOR C is a
finding of `wp15b_U3_REVIEW_urev5.md` (`:59`). Under D-331 a finding's home is its
report; none of the three is at `13621d3`.

**Fix scope.** Narrow the claim to what is genuinely open (the binding mechanism, and
only for the non-copied instances), cite `wp15b_U3_REVIEW_urev4.md` MAJOR A/B and
`wp15b_U3_REVIEW_urev5.md` MAJOR C by report, and state that D-331's ground section
already discusses A and B.

---

### 3. **"Every universal in this unit's REVIEW STATUS table is withdrawn and none is replaced by a narrower one"** — false by enumeration; at least six survive in those tables

**The claim reviewed — U4:1935–1937:**

> **What u-rev 8 does about it is a REMEDY and not a rule:** **every universal in this
> unit's REVIEW STATUS table is withdrawn and none is replaced by a narrower one**, and
> the head's citation claim states its bound instead of asserting a set.

**Contradicting evidence — the enumeration, over U4:184–257 (all three REVIEW STATUS
tables):**

```
$ sed -n '184,257p' docs/experiments/U4_soundness_instrument.md | grep -on "every[^.|]\{0,60\}\|Every[^.|]\{0,60\}\|Nothing[^.|]\{0,50\}\|nothing[^.|]\{0,40\}\|none of[^.|]\{0,40\}\|no live[^.|]\{0,40\}"
…
47:NO LIVE SENTENCE IN THIS UNIT IDENTIFIES THE DIFFERENTIAL GATE AS S
50:every u-rev bump of this unit, including this one) rather than any particul
51:Every abbreviated citation now reads …~~ **THAT UNIVERSAL IS WITHDRAWN AS F
62:all six
63:Nothing is silently re-labelled: each site states what ch
64:at every one
64:no live statement of the count survives**
73:nothing, and the only rule-5 mention is game rule 5 at §8
```

Three universals **are** withdrawn (U4:230 *"NO LIVE SENTENCE…"*, U4:234 *"Every
abbreviated citation…"*, U4:245 *"across the whole unit"*). **These are not**, and each
is a universal about this document's own state asserted in a REVIEW STATUS row:

| line | surviving universal |
|---|---|
| 233 | *"**This unit still asserts no line count of itself** (rule 9)"* |
| 246 | *"**Nothing is silently re-labelled: each site states what changed**"* |
| 247 | *"**REPAIRED at every one.**"* |
| 247 | *"Verified by re-running the reviewer's own line-break-tolerant scan: **no live statement of the count survives**"* |
| 256 | *"`grep -n "23\.2"` returns **nothing**, and **the only** rule-5 mention is game rule 5 at §8.4's M6 row"* |
| 257 | *"**this unit asserts no line count of itself** (rule 9)"* |

I tested the four falsifiable ones and **all four are currently TRUE** (`grep -n "rule
5"` returns only U4:256 and U4:887, §8.4's M6 row; `grep -n "wc -l\|line count\|[0-9]\{3,4\}
lines"` returns only 233 and 257; the only `23.2` in the tree is the row asserting
there is none). **That is the point, not a mitigation:** the claim is that the *shape*
was removed from the table, and it was not — the shape survives six times, in the same
tables, unwithdrawn, and *"REPAIRED at every one"* is the same completeness shape as the
three that were withdrawn.

**Why it breaks.** This is itself a universal about the document's own state, asserted
by its author, false in the commit that asserts it — the class the bullet is about,
committed in the sentence that announces the remedy for it. It is the third time this
unit has done that (U4:234's own row: *"THIS IS THE SECOND UNIVERSAL IN THIS TABLE
WITHDRAWN AS FALSE, AND THE THIRD IN THIS UNIT'S HISTORY"*).

**Fix scope.** State the bound rather than the universal — the shape the same round got
right for citations at U4:154: name the three withdrawn, and say the others were not
examined.

---

### 4. U4-Z's OPEN list asserts **"WHICH CHECKLIST ITEMS N-E's `--config` ENGAGES IS STATED AT §9.1 AND NOWHERE ELSE"** — falsified by the two sentences that follow it in the same bullet

**The claim reviewed — U4:1992–1998, the repair of MAJOR 2:**

```
$ sed -n '1992,1998p' docs/experiments/U4_soundness_instrument.md
  are owed at IMPL, and the coverage rule binds each. **WHICH CHECKLIST ITEMS N-E's
  `--config` ENGAGES IS STATED AT §9.1 AND NOWHERE ELSE (D-331), AND D-329 CHANGES THAT
  SET IN BOTH DIRECTIONS.** §9.1 amendment 2's *"eight of twelve"* was counted for
  **N-A**; D-329 ADDS four conditions and **REMOVES item 11**, whose scope is a binding
  consumed by `rm`, `mv` or a write, while `$CONFIG` is a READ — so item 9 governs it
  and is discharged by the whole-path guard both rows owe. **§9's own head carries that
  measurement (red-team F13) and this bullet cites it rather than re-counting.**
```

**Contradicting evidence, from inside the same bullet.** The sentence after *"NOWHERE
ELSE"* states which item D-329 removes and why — that **is** a statement of which items
N-E's `--config` engages. The sentence after **that** says §9's head carries the
measurement. Both are places other than §9.1, and the bullet names one of them itself:

```
$ sed -n '1426p' docs/experiments/U4_soundness_instrument.md
**SHELL_CHECKLIST items ENGAGED: 1, 3, 4, 8, 9, 10, 11, 12 — eight of twelve**,
$ sed -n '1030,1032p' docs/experiments/U4_soundness_instrument.md
> **N-Q's extra lines are required by no rule in this tree (F13):** item 11's scope is
> *"any binding consumed by `rm`, `mv`, or a write"* and `$CONFIG` is a READ, so item 9
> governs it and is discharged by the whole-path guard **both** rows owe.
```

**And "cites rather than re-counting" is true only of the arithmetic.** The bullet
reproduces §9.1's cardinal (*"eight of twelve"*), `SHELL_CHECKLIST` item 11's scope
(verbatim against `tools/SHELL_CHECKLIST.md:119–120`) and the whole F13 argument. D-331
clause (3) names **OPEN lists** among the artefacts that *"may say that a finding
exists, that it is repaired and where the repair lives, and may not restate the
finding's content"*. The repair replaced a false restatement with a true one; it did not
remove the restatement, which is the remedy D-331 prescribes and which this bullet
claims for itself.

**Why it breaks.** The bullet is the IMPL-facing surface for a `tools/` review that is
owed, its subject is a set that D-329 changed in both directions, and it now carries
both a false locality claim and a second copy of the content whose correctness it
depends on. The next time §9.1 or §9's F13 paragraph moves, this copy goes stale — which
is precisely how MAJOR 2 was manufactured.

**Fix scope.** Drop *"AND NOWHERE ELSE"*; replace the restated cardinal and the restated
F13 reasoning with pointers to §9.1 amendment 2 and §9's F13 paragraph, which the bullet
already names.

---

### 5. U4-Z's lead-in still asserts that at u-rev 7 **"every claim below whose truth depends on an ADR line was re-read … item by item"** — the u-rev 7 review graded that claim FAILS, and u-rev 8 withdrew its three siblings and left this one

**The claim reviewed — U4:1589–1600, the U4-Z lead-in blockquote:**

```
$ sed -n '1592,1594p' docs/experiments/U4_soundness_instrument.md
> **BLOCKING 1** of `wp15b_U4_REVIEW_urev6.md`. So at u-rev 7 every claim below whose
> truth depends on an ADR line was re-read against `docs/decisions.md` as it now
> stands, item by item, rather than the two sentences the report names being edited.
```

**Contradicting evidence — `wp15b_U4_REVIEW_urev7.md`, its own claim-inventory row Z2:**

```
$ grep -n "^| Z2 |" docs/experiments/wp15b_U4_REVIEW_urev7.md
351:| Z2 | *"every claim below whose truth depends on an ADR line was re-read against `docs/decisions.md` as it now stands, item by item"* | **FAILS** — MAJOR 2 and MAJOR 3 |
```

**And u-rev 8 did not touch it.** The round's diff has no hunk between old line 1393 and
old line 1699:

```
$ git diff 0f49c90 a2b50bf -- docs/experiments/U4_soundness_instrument.md | grep "^@@" | sed -n '15,17p'
@@ -1383,10 +1510,29 @@ readable alone; it is a condition, not a datum.)
@@ -1699,7 +1845,7 @@ attacked twice and selected from.)*
@@ -1736,16 +1882,78 @@ list below**.
```

**Why it breaks.** Three universals of exactly this shape were struck through in the
REVIEW STATUS table this round (U4:230, 234, 245), each with the note that a completeness
claim beside an incomplete pass is the defect one level up. This is the largest of them
and it sits at the head of the section where the class recurred three times, telling an
IMPL reader that U4-Z has been re-derived against the ADR log when the review of that
re-derivation found three claims it did not reach. It is the same defect the round's own
head diagnoses at U4:195–199 (*"u-rev 7's answer … was to RE-DERIVE U4-Z in full; the
re-derivation shipped three fresh instances"*) — the diagnosis landed and the sentence it
diagnoses did not move.

**Fix scope.** Strike it as the three siblings were struck, with the disposition stated
(re-derivation was the instrument, `wp15b_U4_REVIEW_urev7.md` MAJOR 2/3/4 found what it
did not reach), and replace it with nothing.

---

### 6. The REVIEW STATUS row for MINOR 7 says the second site **"cites rather than restates"** — it restates the figure, unmarked, so half of MINOR 7 is unrepaired and the table says otherwise

**The claim reviewed — U4:192:**

> | **MINOR 7** — `91 test lines` is unmarked at two sites and measurable in one command
> | §9's fold, MEASURED with its command and output. **The second site is U4-Z's
> four-conditions bullet, which cites rather than restates** |

**Contradicting evidence — what is actually at the second site:**

```
$ grep -on "against a precedent of 91 test lines for one arm at \`b067d47\`" docs/experiments/U4_soundness_instrument.md
1956:against a precedent of 91 test lines for one arm at `b067d47`
$ grep -n "91 test lines" docs/experiments/U4_soundness_instrument.md | cut -c1-70
1055:>    **MEASURED 91 test lines for ONE guard arm at `b067d47`.**
1956:- **THE FOUR CONDITIONS RIDING WITH N-E ARE UNPAID, AND EACH BINDS IMPL** (D-329)
```

U4-Z:1956 restates the number. It carries no **MEASURED** mark, no command, and no
pointer to §9's condition 3 where the measurement now lives. The unit's own head rule
(U4:132–134) is that *"a mark added at u-rev 6 or later names the command that took it
and pastes its complete output"*.

**Why it breaks.** MINOR 7 charged the figure *"unmarked at two sites"*. One site is
repaired well (U4:1044–1055 pastes the full `git show --numstat`, which I re-ran and
which reproduces). The other is unchanged, and the REVIEW STATUS row — the round's
headline instrument, whose lead-in is *"THE ROWS BELOW SAY WHERE EACH FINDING IS
ANSWERED"* — describes it as something it is not. A status row that misdescribes the
state at the site it names is worse than the unmarked number, because it is the surface
a next round reads to decide the finding is closed.

**Fix scope.** Either mark the U4-Z site, or replace its figure with a pointer to §9
condition 3; then correct the row to say which was done.

---

### 7. **"u-rev 8 is one round in several commits"** inverts D-311's own ground — one label now names six texts differing by 289 lines, where D-311 was landed because one label named two texts differing by 69

**The claim reviewed — U4:121–127:**

> **u-rev 8 IS ONE ROUND IN SEVERAL COMMITS, AND THE REVIEWED REVISION IS THE LAST OF
> THEM.** D-311 bumps the label on any append; this round appends more than once, and
> **labelling each commit separately would create exactly the revision-to-label ambiguity
> D-311 exists to remove.** … The revision a review is dispatched against is named in the
> REVIEW STATUS block by its SHA, and that is the binding identifier; `u-rev 8` names the
> round.

**Contradicting evidence — the ground D-311 rests on, in this unit's own words 15 lines
below:**

```
$ sed -n '136,141p' docs/experiments/U4_soundness_instrument.md
**LABEL DISCIPLINE — D-311, travelling item T5.** Any append to this unit bumps
its u-rev, however small the diff. A review is dispatched against a named
revision and reviews of superseded revisions do not transfer; the superseded
document carried the label "Revision 7" at both `d94dc0a` and `6feb40a`, which
differ by 69 lines, and that ambiguity is what this rule removes.
```

The recorded ambiguity is **one label naming more than one text**. Bumping per commit
gives each text its own label — the opposite of the ambiguity — and *not* bumping is what
reproduces it. Measured, "u-rev 8" now names six texts:

```
$ git log --oneline --reverse 0f49c90..a2b50bf -- docs/experiments/U4_soundness_instrument.md | wc -l
6
$ git diff 0f49c90 a2b50bf --stat -- docs/experiments/U4_soundness_instrument.md
 docs/experiments/U4_soundness_instrument.md | 289 ++++++++++++++++++++++++----
 1 file changed, 254 insertions(+), 35 deletions(-)
```

254 + 35 = 289 changed lines across the label's span, against the 69 D-311 was landed for.

**The offered mitigation does not hold.** *"The revision … is named in the REVIEW STATUS
block by its SHA"* is true of the **prior** revision (`0f49c90`) and cannot be true of
this one — the unit cannot name its own SHA, and it names none of the six. Nothing in the
tree enumerates the round's commits; the dispatch that commissioned this review listed
five of the six (it omits `823004a`, the D-334 fold, `38 insertions`), which is the
ambiguity operating in the wild.

**And the concrete harm materialised in this same round:** BLOCKING 1 above. A per-commit
bump forces the label to be re-read on every append, which is the mechanism that caught
the u-rev 4→5→6→7 closing-line updates and which is exactly what did not happen here.

**Why it breaks.** The unit ships, three lines apart, a binding rule (*"Any append to
this unit bumps its u-rev, however small the diff"*) and a self-exemption from it whose
stated justification is the rule's own ground read backwards. Under the project's own
text an exemption of this kind is an architect's to grant, and none has been.

**Fix scope.** Either bump per landed commit (and let the REVIEW STATUS block name the
one under review), or — if the round-label is genuinely wanted — take it to the architect
as an amendment to D-311 rather than as a reading, and meanwhile enumerate the round's
commits in the head so `u-rev 8` resolves to a set of SHAs.

---

## MINOR

### 1. MAJOR 4's repair introduces three factual errors at the site it repairs

**The claim reviewed — U4:1884–1893:**

> T2 is named in the tree at `docs/experiments/restructure_matrix_15b.md:35` and
> `docs/experiments/restructure_selection_15b.md:50`, **both under `## Travelling items
> (bind to every option, cost is common)`** … on the same two documents **from whose
> fiftieth line** this unit already resolves its sibling T5, **at U4:108**.

```
$ grep -n "Travelling items" docs/experiments/restructure_matrix_15b.md docs/experiments/restructure_selection_15b.md
docs/experiments/restructure_matrix_15b.md:30:## Travelling items (bind to every option, cost is common)
docs/experiments/restructure_selection_15b.md:45:## Travelling items, corrected per red team
$ grep -n "T5" docs/experiments/restructure_matrix_15b.md docs/experiments/restructure_selection_15b.md
docs/experiments/restructure_matrix_15b.md:41:T5. Revision label bumps on ANY append (6feb40a appended §18 unbumped,
docs/experiments/restructure_selection_15b.md:51:     double-list killed (B3). T5. Label bump on any append.
$ sed -n '108p' docs/experiments/U4_soundness_instrument.md
  multi-unit repair round lands serially and that this round did — U2, then U3, then
$ grep -n "LABEL DISCIPLINE" docs/experiments/U4_soundness_instrument.md
136:**LABEL DISCIPLINE — D-311, travelling item T5.** Any append to this unit bumps
```

*(a)* Only one of the two documents carries that heading. *(b)* T5 is on line **41** of
one and line **51** of the other — neither is the fiftieth. *(c)* `U4:108` was the
LABEL DISCIPLINE paragraph at u-rev 7; at u-rev 8 it is at 136, so the unit's only
self-referencing line number does not resolve. The three are copied from
`wp15b_U4_REVIEW_urev7.md`'s own text and line numbers without re-checking against the
u-rev 8 tree. The disposition itself (T2 = *"M4 ADR line (B2)"*, DISCHARGED by D-329 and
ruled on by D-333) is correct and the defect MAJOR 4 charged is gone.

**Fix scope.** Drop the heading claim and the "fiftieth line" clause (the two file:line
pointers already carry them), and replace `U4:108` with the §-anchor *"the head's LABEL
DISCIPLINE paragraph"*.

### 2. The head's registered derived command is blind to unbolded sibling-unit references — the same blindness that failed U3's u-rev 6 command and that U3 replaced at u-rev 7

**The claim reviewed — U4:160–164:** *"**The set is DERIVED and is not enumerated
here:**"* followed by
`grep -n '\*\*U[123]\*\*' docs/experiments/U4_soundness_instrument.md | grep -v 'landed'`.

```
$ grep -n '\bU[123]\b' docs/experiments/U4_soundness_instrument.md | grep -v '\*\*U[123]\*\*' | cut -c1-60
108:  multi-unit repair round lands serially and that this round did
423:> FILTERED (U2 5.3, mover only, phase-derived left/budget) = 174
665:generated cell set at both phases (U2-T). D-124's flip clause reads
1476:Carried from the superseded §11. The rows this unit does not own are
1477:U3-T and `WPQ_seed.md`, and no row is in two places.
```

U4:665, 1476 and 1477 cite `U2-T` / `U3-T` — named parts of sibling units, carrying no
u-rev — and the registered command does not surface them. **The class is measured, not
hypothetical:** `wp15b_U3_REVIEW_urev6.md` MAJOR D failed U3's u-rev 6 for a derived
command *"blind to an unbolded citation"*, and U3 replaced that instrument at u-rev 7
(`a2b50bf`). U4 adopted the superseded instrument from U3 (u-rev 6) in the same round.
I looked for the worse case and did **not** find it: `grep -n '[^*]\bU[123] §'` outside
the bolded form returns **0**, so no `§`-citation is missed. This is the milder form.

**Fix scope.** Widen the pattern to cover `U[123]` in any form, or adopt whatever U3's
u-rev 7 replaced it with, and say what the command is and is not blind to.

### 3. The D-334 fold understates what a revision 3 owes — round 2 names four rows no field has carried, not two

**The claim reviewed — U4:1918–1923:** *"round 2's report enumerates six things it must
contain, **of which the two no field has yet carried** are round 1's `(e)` … and the
DERIVED-ENUMERATION row …"*

```
$ sed -n '1000,1020p' docs/experiments/matrix_U4R_REDTEAM_round2.md
**What a revision 3 would have to contain**, …
1. **Round 1's (e) as round 1 wrote it** … It is the row the field has twice claimed to
   carry and twice not carried.
2. **The DERIVED-ENUMERATION row (K5)** …
3. **A §8.7-scoped row.** … **No row in either field touches it** …
4. **A row that changes what a REVIEWER is asked to check** … That instrument … is in
   neither field.
```

Items 3 and 4 are stated by the report as in neither field. The understatement is
inherited from D-334, which says the same, so the accurate repair is a pointer rather
than a correction the unit may make on its own.

**Fix scope.** Point at round 2's §"What a revision 3 would have to contain" and drop
the count.

### 4. D-333 is not folded at the two sites that state the residual as OPEN and the architect's

U4:110–111 states the bound honestly (*"folded at that bullet in U4-Z"*), and D-333 is
also carried at U4:1891. But the two surfaces that tell a reader the residual's status
were not re-read against it:

```
$ sed -n '1093,1096p' docs/experiments/U4_soundness_instrument.md
> **RESIDUAL, OPEN AND THE ARCHITECT'S: N-E HAS NOT BEEN ATTACKED BY A FRESH-CONTEXT
> DECISION-RED-TEAM IN ITS OWN RIGHT.** The matrix law is satisfied for the FIELD — a
> matrix was authored and attacked before selection — and D-329 claims no more than
> that. It is in U4-Z's OPEN list.
$ sed -n '286,288p' docs/experiments/U4_soundness_instrument.md
THIS u-rev; **the differential gate's SEAM decision**, … **a fresh-context attack on N-E
in its own right**, which D-329 records as its own residual because the red team was
dispatched to break N-Q and was never asked to break the row it recommends;
```

The architect **has** now ruled: D-333 holds that the residual does not reopen the
selection, states the two grounds, and states what it does not do. §9's head is where an
IMPL reader meets the residual first, and U4-A's list is the unit's register of what it
owes; neither records that a ruling now governs it. This is D-305's class at a fold the
same round performed.

**Fix scope.** One pointer at each: *"RULED ON by D-333 — the residual stands, the
selection does not reopen."*

### 5. The `3543a7f` carve-provenance correction landed in this round with no change-log entry and no marker at its site, and its new clause is falsified 260 lines below

```
$ git show 3543a7f -- docs/experiments/U4_soundness_instrument.md | tail -6
-item 1 at `6feb40a` (revision 7, never reviewed, CLOSED by D-309) under the
+item 1 at `6feb40a` (revision 7, CLOSED by D-309 — which records the fresh-context REVIEW-design that FAILED it; the counts are D-309's and are not restated here) under the
$ sed -n '272p' docs/experiments/U4_soundness_instrument.md | cut -c1-90
| REVIEW-design | revision 7, `6feb40a` | **FAIL** — 7 BLOCKING, 7 MAJOR, 9 MINOR. …
$ L=$(grep -n "^D-309:" docs/decisions.md | cut -d: -f1); sed -n "${L}p" docs/decisions.md | grep -o "7 BLOCKING, 7 MAJOR, 9 MINOR"
7 BLOCKING, 7 MAJOR, 9 MINOR
```

*(a)* The change log's preamble is *"The text is a verbatim carve apart from
cross-reference retargets and **the following**, each stated where it occurs"* (U4:97–99)
and its u-rev 8 entry (U4:100–119) does not mention this edit; the site carries no
`CORRECTED AT u-rev 8` marker, which every other u-rev 8 repair does. *(b)* The new
clause says *"the counts are D-309's and are not restated here"* while U4-A:272 restates
those exact counts. **What the round got right and I credit:** the correction itself is a
real catch — U4-A already recorded revision 7 as reviewed and FAILED while the head said
"never reviewed", so the unit was internally contradictory since the carve.

**Fix scope.** A change-log line and a site marker; and either delete *"are not restated
here"* or make U4-A:272 a pointer.

### 6. The §8 record stamp does not classify the §8.7 blockquote the round edited, in either direction

The stamp's RECORD enumeration (U4:447–455) covers the u-rev 1 stub, §8.1, §8.2's body
prose, and *"the prose §8.3 and §8.4 carry from the superseded document"*. Its NOT-RECORD
list (U4:457–466) names five blocks, item (v) being *"§8.7's four-name wiring sentence and
the `FOLDED AT u-rev 6` paragraph beneath it"*. §8.7's opening `THE DEFECT` blockquote —
U4:908–942, the block u-rev 8 edited for MAJOR 1 — is in neither list. The ruling that it
is carve prose is recorded **inside** the block (U4:936–938) rather than at the stamp,
which is the stamp's own subject matter and, under D-331, its home. This is how S-E
survived at that site for three revisions: a reader deciding whether to trust or to
correct the paragraph consults the stamp, and the stamp is silent.

**Fix scope.** Add it to the NOT-RECORD list, with the u-rev 8 ruling as its reason.

### 7. §8.7:919's present-tense *"so S-E **is** gate (b)'s instrument"* is marked only 21 lines below it, against the unit's own MINOR 4 precedent

```
$ sed -n '919,920p' docs/experiments/U4_soundness_instrument.md
> and its matrix was headed `| Option | (b)'s instrument |` — **so S-E *is* gate
> (b)'s instrument**, and "(a)–(d) plus S-E" counted it once as (b) and once as
```

MAJOR 1's repair is otherwise **good and I verified it by meaning, not by string**: I
swept all 76 `S-E` occurrences and every other live one is either past-tense, negated,
self-dating (*"S-M since D-323, S-E until u-rev 6"*), or inside stamped record. This one
is present-tense and in the unit's own inferential voice (*"so …"*). The unit disposes of
it at U4:939–942 as *"REPORTED SPEECH about revision 1's own matrix heading"*, which is a
defensible reading — but the disposition sits in a parenthetical below the paragraph and
the paragraph carries no mark at its own site. The unit's own MINOR 4 precedent is
*"REPAIRED AT BOTH ENDS … §9.1 amendment 4 now carries a marked pointer at its own site,
so a reader meeting it first does not read an unqualified …"* (U4:231). A reader meeting
line 919 first meets an unqualified present-tense identification.

*(Cosmetic, noted not charged: the parenthetical's `**` opened at "The paragraph above
it" is never closed, and it terminates `twice.*)` rather than `twice.)*`.)*

**Fix scope.** A three-word inline mark at 919, on the pattern §8.3's table cell uses.

---

# MY OWN CLAIM INVENTORY FOR U4-Z, BUILT INDEPENDENTLY

I did not take the round's account of U4-Z on trust and did not reuse the prior
reviewer's 54 rows. I enumerated every claim in U4-Z (U4:1587–2105) whose truth depends
on an ADR line, on another U4 section, on another document, or on tree state.
**Result: 61 claims examined, 55 hold, 6 fail.**

### Lead-in and B3 section (U4:1589–1783)

| # | Claim | Verdict |
|---|---|---|
| Y1 | *"every claim below whose truth depends on an ADR line was re-read … item by item"* | **FAILS** — MAJOR 5 |
| Y2 | D-320 landed at `0af32fb`, in the same commit as D-321 | HOLDS |
| Y3 | D-325 landed at `81180b8`, after u-rev 6 | HOLDS |
| Y4 | The two-shape comparison is left UNEDITED | HOLDS — no diff hunk in the round touches it |
| Y5 | B3 SETTLED, shape 2 (D-316), residual disposed of by D-320 | HOLDS |
| Y6 | The SELECTION block records the u-rev 2 execution; instrument since D-323 is S-M | HOLDS |
| Y7 | D-320's six parts (breach; waiver on two grounds; proportionality; independent verification; what it does not do; debt PAID by disclosure) | HOLDS — each verbatim at `docs/decisions.md:685` |
| Y8 | D-320's flip clause, and that this unit is where it would fire | HOLDS; I looked for the gate-naming defect and found none — all four names defined (§8.3), all four wired (§8.7), retired letters resolve through the lookup table, the unlabelled bullet is expressly *"A CONFIG STATEMENT, NOT A FIFTH GATE"* |
| Y9 | Shape 2's cost DISCHARGED by §8.3's letter→gate lookup table | HOLDS |
| Y10 | D-325 records the COUNT SIX, no seventh, D-316 untouched | HOLDS |
| Y11 | *"THE SAME FALSE DIAGNOSIS WAS IN THE LANDED D-316"*, quoted | HOLDS — `docs/decisions.md:677` |
| Y12 | The `ec8f7fb:502` quotation is the line verbatim | HOLDS |

### ADR-lines section (U4:1785–1897)

| # | Claim | Verdict |
|---|---|---|
| Y13 | Item 4 blocked on the SEAM, which D-323 records as separate and OPEN | HOLDS |
| Y14 | Item 4's seam ≠ D-329's seam; D-115 bars widening `pistol_search::staged` | HOLDS |
| Y15 | Item 15 blocked on SELECTED-AND-NOT-BUILT plus the missing config document | HOLDS |
| Y16 | Item 15's MAJOR 4 disagreement UNRECONCILED and unchanged by the selection | HOLDS |
| Y17 | Numbered item 4: S-M, R1 by `#[path]`, DEPENDS-OPEN-THEORY, five conditions | HOLDS — matches D-323 |
| Y18 | Numbered item 15: **U3** (u-rev 6, landed `13621d3`) §10 is the one place the count is stated | HOLDS — both halves verified |
| Y19 | B2: revision-7 review found M4 had no ADR line; three rounds stopped; D-329 is now M4's line | HOLDS |
| Y20 | The strongest surviving attack against N-E is ASSEMBLED, not quoted | HOLDS |
| Y21 | T2 is named in the tree, at both file:line pointers, as *"M4 ADR line (B2)"* | HOLDS |
| Y22 | T2 is under `## Travelling items (bind to every option, cost is common)` in **both** | **FAILS** — MINOR 1 |
| Y23 | T5 is resolved from the fiftieth line of the same two documents | **FAILS** — MINOR 1 (lines 41 and 51) |
| Y24 | *"at U4:108"* resolves to the LABEL DISCIPLINE paragraph | **FAILS** — MINOR 1 (it is at 136) |
| Y25 | T2 DISCHARGED by D-329, ruled on by D-333 | HOLDS |

### OPEN list (U4:1899–2101)

| # | Claim | Verdict |
|---|---|---|
| Y26 | Micro-matrix U4-R authored twice at `4d84694` / `97aa4d6` | HOLDS |
| Y27 | Attacked twice at `53c0c0b` (6/8/11) and `1e70f81` (6/10/11) | HOLDS — both reports' own closing lines |
| Y28 | Both fields fell; no row selectable; D-334 forbids citing either recommendation | HOLDS — D-334's own wording |
| Y29 | The repairs landed while round 2's attack ran and took the recommended row's second half | HOLDS — round 2's decisive finding, quoted accurately |
| Y30 | The eight findings are DISCHARGED | **FAILS** — MAJOR 1 (MAJOR 3 is re-manufactured at U4:111) and MAJOR 6 (MINOR 7's second site) |
| Y31 | The repairs are NET ADDITIVE and §11's question is sharper | HOLDS — MEASURED, 1886 → 2105 |
| Y32 | A revision 3 is re-grounded on this review, not on `_urev7.md` | HOLDS — D-334 and round 2 both |
| Y33 | *"the two no field has yet carried"* | **FAILS** — MINOR 3 (four) |
| Y34 | This unit may not restructure itself without a matrix | HOLDS |
| Y35 | D-331 *"does not reach"* / *"the law is silent about"* the class | **FAILS** — MAJOR 2 |
| Y36 | MAJOR A, B and C are *"of **U3** (u-rev 6, landed `13621d3`)"* | **FAILS** — MAJOR 2 (they are of two review reports) |
| Y37 | *"every universal in this unit's REVIEW STATUS table is withdrawn"* | **FAILS** — MAJOR 3 |
| Y38 | The head's citation claim states its bound instead of asserting a set | HOLDS — and it is the round's best decision |
| Y39 | Live carve-prose citations naming no u-rev are NOT converted; OPEN | HOLDS; the derived command that scopes the set is MINOR 2 |
| Y40 | B1/M3 CLOSED by S-M (D-323), with six named residuals each having its own bullet | HOLDS — all six bullets present |
| Y41 | B2/M4 ANSWERED by N-E (D-329); `7866bcf` / `7e0a328` / `d56a898`; N-M eliminated on registered ground; rung (a) silent; rung (b) 22/7 vs 32/12, 4 shared guard lines, 5 containment on top | HOLDS — every figure matches `matrix_M4_axisA_REDTEAM.md`'s own re-derivation |
| Y42 | AXIS B NOT REOPENED; D-324's flip fired toward N-K; no ADR adopts N-K | HOLDS |
| Y43 | D-333 (R18): does not reopen; two stated grounds; flip on IMPL exceeding 22/7 or a fresh attack breaking N-E; does not certify N-E, does not discharge the four conditions | HOLDS — every clause matches `docs/decisions.md:711` |
| Y44 | The four conditions riding with N-E, each stated | HOLDS — all four match D-329 |
| Y45 | *"it is recorded by the SELECTION RECORD and not by D-329"* | HOLDS — and the head contradicts it, MAJOR 1 |
| Y46 | The relative-base residual is CLOSED at `63eac4c` by R19; REVIEW-impl PASSED at `d59f0de` | HOLDS — the R19 review's *"PASS. No BLOCKING or MAJOR finding. Three MINOR"* and its explicit ratification of the reading. **Recorded for the architect, not charged: no ADR line carries R19** — `grep -ci "R19" docs/decisions.md` → `0` — so this closure's only home in the tree is U4, while its subject's home (`matrix_M4_axisA_selection.md` condition 4) still reads *"It is OPEN."* |
| Y47 | B3's residual DISPOSED OF by D-320; debt PAID; flip clause live | HOLDS |
| Y48 | MAJOR 8's residual has two parts; `crates/pistol-search/src/staged.rs` does not exist | HOLDS — `ls` fails at HEAD |
| Y49 | The snapshot's second instrument, agreement criterion, stage under doubt and disagreement consequence are unregistered | HOLDS |
| Y50 | §9.1's *"eight of twelve"* was counted for N-A; D-329 adds four and removes item 11 | HOLDS on the substance — item 11's scope at `tools/SHELL_CHECKLIST.md:119` is *"Any binding consumed by `rm`, `mv`, or a write"*, `$CONFIG` is a READ |
| Y51 | *"STATED AT §9.1 AND NOWHERE ELSE"* | **FAILS** — MAJOR 4 |
| Y52 | *"this bullet cites it rather than re-counting"* | HOLDS of the arithmetic only; the bullet restates the cardinal, item 11's scope and F13's reasoning — MAJOR 4 |
| Y53 | Both shipped-instrument defects CLOSED at `b067d47` / `a102c6a`; REVIEW-impl PASSED at `84ff8d7`, 0/0/3, controls mutation-killed | HOLDS |
| Y54 | F1, F2, F3 of that report as described | HOLDS |
| Y55 | §8.4's M3 witness NOT BUILT; a legal position OWED | HOLDS — U4:884 states it in the cell |
| Y56 | The differential gate's seam is OPEN and is not D-329's seam; no matrix authored | HOLDS |
| Y57 | S-E's second half neither selected nor rejected; no ADR since D-323 carries it | HOLDS |
| Y58 | Per-CI cost UNGROUNDED at its dominant term; D-323 does not re-price | HOLDS |
| Y59 | `configs/instrument_staged_v0.toml` does not exist; no M4 row produces it | HOLDS — `ls` fails |
| Y60 | The seam is SELECTED and NOT BUILT; `tools/baseline_snapshot.sh:182` still the literal, no `--config` of its own | HOLDS — re-measured at HEAD, and the MINOR 5 repair's pasted output reproduces line for line |
| Y61 | *"No REVIEW-design has run against this text at THIS u-rev"* + the review history | **FAILS** — BLOCKING 1 (the history stops at u-rev 6) |

---

# VERIFIED WITH NO FINDING

- **MAJOR 1's repair is correct by MEANING, not by string.** All 76 `S-E` occurrences
  swept. The corrected clause (U4:925–929) now reads *"because **the differential gate is
  named ONCE, in §8.2**"*, matching the U4-Z copy's u-rev 7 repair; the annotation
  (U4:931–942) states what it read, why it is corrected in place rather than annotated,
  and marks the one adjacent survivor. §8.4's ledger is fully retargeted (M3 and M4 both
  read *"the differential gate (**S-M** since D-323; registered as S-E's class until
  u-rev 6)"*); §8.7's `FOLDED AT u-rev 6` paragraph names S-M; §11.6 and U4-T are
  self-dating. **The universal is withdrawn at U4:230 with no narrower one replacing it**,
  and the row says why. Only MINOR 7 above survives, and it is marked.
- **MINOR 6's repair is complete.** `grep -on "af8082a[^)]\{0,45\}"` now returns the
  converted form at 234, 274, 959, 1953 and 2105; the previously-charged §8.7 site
  (u-rev 7's line 854) is now U4:959 and reads *"S-M SELECTED at `af8082a` (taken at
  `809b5db`)"*. The remaining bare hits (29, 309) are *"landed at `af8082a`"*, which is
  self-explaining, and 479/487 are inside stamped u-rev 1 record. The universal at
  U4:234 is withdrawn and not re-asserted.
- **MINOR 7's first site is fully repaired.** U4:1044–1055 pastes the complete
  `git show b067d47 --numstat` output; I re-ran it and it reproduces (`91  0
  crates/pistol-cli/tests/baseline_snapshot_tests.rs`).
- **The record discipline held.** No hunk of the round lands inside a stamped-RECORD
  block; §9.1 amendment 2 (U4:1426) is untouched; the two-shape comparison is untouched.
- **D-332's fold is accurate and is the clause D-332 requires.** The unit's head states
  that the round was **serial** and that this unit is last, which is R17 clause (2)'s
  disclosure obligation. Measured, it is true: `f0ae14c` (U2→5) and `13621d3` (U3→6) both
  precede `d328d1d`. And the landed-SHA form has already outlived the claim's usefulness
  in the right direction — U3 reached u-rev 7 and U2 u-rev 6 after this unit landed, and
  no citation went stale.
- **Rule 9 holds** — output above.
- **The tree is clean at exit.**

---

# REJECTED, WITH THE ATTEMPTED REPRODUCER

- **"The §8.3 sentence at U4:845–846 is stamped RECORD and the round edited it."**
  Attempted: the stamp lists *"the prose §8.3 … carry from the superseded document"* as
  RECORD, the NOT-RECORD list names only **one** live §8.3 **U3** §10 sentence, and the
  round edited two. **Rejected** — `git show 6feb40a:docs/experiments/wp15b_design.md |
  grep -n "Earlier revisions shipped fewer staged"` returns nothing, so the sentence is
  carve-authored, not carried, and the stamp's §8.3 clause is a criterion rather than an
  enumeration. No RECORD was edited.
- **"§8.4's M6 cell identifies S-E as the differential gate in the present tense"** —
  *"Its class is mate and not S-E … S-E **is** blind at an `Impossible` node."*
  **Rejected** — this is a statement about S-E's mechanism, not about which instrument
  the gate carries, and the cell's class column is already retargeted with a mark
  (*"**mate**, not the differential gate's class (\"not S-E\" until u-rev 6)"*).
- **"`grep -n "23\.2"` returns nothing" is now false"** (U4:256). Attempted:
  `grep -c "23\.2"` → `1`. **Rejected** — the single hit is the row asserting there are
  none. Self-referential, not a falsification.
- **"BLOCKING 3's 'no live statement of the count survives' is false — §8.3 enumerates
  the four staged documents."** Attempted: U4:763–767 names `tactical_staged_v0.toml`,
  `gate_staged_v0.toml` and *"the SPRT seat and the play config"*, from which four is
  derivable. **Rejected** — the rule the unit binds itself to is *"states no such number
  **as a live claim**"*, and no cardinal is stated. An enumeration that a reader may
  count is not a statement of the count under this unit's own narrowing, which the u-rev
  6 review accepted.
- **"D-333 has three grounds and U4 says two."** Attempted: `docs/decisions.md:711`
  numbers (1), (2), (3). **Rejected** — (3) is a concession (*"The unquoted attack is a
  WEAKER record … R18 does not upgrade it"*), not a ground for the ruling, and U4 carries
  its substance separately (*"R18 does not certify N-E sound, does not discharge the four
  conditions, and does not close the weakness below"*).
- **"The head overstates by saying D-333 is folded 'at that bullet' when it is at two."**
  Attempted: U4:110–111 against U4:1891 and U4:1955. **Rejected** — a bound that
  understates is not a false claim, and the dispatch's scope guard forbids charging a
  pointer.
- **"The registered derived command is blind to `§`-citations, as U3's was."**
  Attempted: `grep -n '[^*]\bU[123] §' … | grep -v '\*\*U[123]\*\*'` → **0 hits**.
  **Rejected in that form** — the blindness is real but confined to `U2-T`/`U3-T`
  (MINOR 2), not to `§` citations.
- **"D-320's flip has fired."** Attempted: read D-320's three named defect shapes against
  §8.3 and §8.7 at u-rev 8. **Rejected** — all four gate names are defined and wired,
  every retired letter resolves through the U4:741–747 lookup table, no fifth gate is
  appended as a letter, and MAJOR 1's residual (MINOR 7) is an instrument identity, not a
  gate name. The retro-matrix is not owed.
- **"U4 restates D-334 wholesale in an OPEN list, breaching D-331 clause (3)."**
  Attempted: U4:1901–1925 against `docs/decisions.md:713`. **Rejected as a standalone
  charge** — the bullet is the unit's registration that its own restructure is stopped,
  which an OPEN list must carry, and every substantive figure in it is a pointer to a
  named commit or report. The one claim it restates and gets wrong is charged as MINOR 3.
- **"The owner table's row for U4 is stale, so the standing duty is unpaid."**
  Attempted: at `a2b50bf` the table recorded 1886 against a 2105-line unit. **Rejected as
  U4's** — the unit records the STANDING DUTY rather than a size, which is the shape that
  cannot go stale, and the table discharged it at `4fd88ec` during this review. Nothing
  here is U4's.

---

**Cross-unit items noted and not charged to U4:** *(i)* **No ADR line records architect
ruling R19** (`grep -ci "R19" docs/decisions.md` → `0`), while U4 and the R19 REVIEW-impl
both treat it as binding and U4 closes a registered residual on it. Rule 10 asks for one
line. *(ii)* `matrix_M4_axisA_selection.md` condition 4 still reads *"It is OPEN."* for a
residual U4 now records CLOSED; the two records disagree and neither points at the other.
*(iii)* The identical *"D-331 is silent about this class"* sentence is live in **U3-Z**
and was charged there as finding E of `wp15b_U3_REVIEW_urev6.md`; U4's copy is MAJOR 2
here, and the fix should probably be one convention-level decision rather than two
unit-local edits — which is round 2's item 5 for a revision 3.

---

*REVIEW-design of `docs/experiments/U4_soundness_instrument.md` u-rev 8, at pinned
revision `a2b50bf` (unit content unchanged since `3543a7f`). HEAD advanced to `4fd88ec`
during the review; `git diff a2b50bf HEAD -- docs/experiments/U4_soundness_instrument.md`
is empty, so the subject did not move. Fresh context; not the author of the unit, its
repairs, either matrix, any red team, or any earlier review. Every finding reproduced
before reporting; every rejected charge recorded with its attempted reproducer. The one
build command run was the census gate, green with this report present. This report
modifies no other repository file, ran no git write command, and is not committed.*

**VERDICT: FAIL — 1 BLOCKING, 7 MAJOR, 7 MINOR. MEASURED SIZE: 2105 lines.**
