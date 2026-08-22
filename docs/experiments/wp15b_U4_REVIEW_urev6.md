# REVIEW-design — `docs/experiments/U4_soundness_instrument.md`, u-rev 6

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer. The dispatching session dispatched the repair and authored both
matrix outcomes it folds; it did not review it. Per the session's standing rule a
re-review FAIL is COLLECTED AND REPORTED, not looped on in-session: the BLOCKING,
both MAJORs and all four MINORs below are UNREPAIRED and are the architect's.

ONE FINDING MOVED AFTER THE REVIEW RAN. Finding 6 charges U4 with recording the
owner table's sizes as stale; the owner table was re-measured at `a0f241b`, which
is AFTER this review's pinned `7358a07`, so the row it names is stale for the
reason the finding gives and the observation it points at is now discharged.
-->

**Pinned revision reviewed: `7358a07`.**

```
$ git rev-parse HEAD
7358a074b4b508dcddead8c5a4eda7053d90b48d
$ git status --porcelain
(no output)
```

**Does it still match HEAD? YES** — `7358a07` is HEAD, tree clean, at the start and at the end of this review. No `git diff 7358a07 HEAD --stat` is needed. **Live tree at exit: `git status --porcelain` → empty.**

**Subject:** `docs/experiments/U4_soundness_instrument.md`, **u-rev 6**, the repair of `docs/experiments/wp15b_U4_REVIEW.md` (REVIEW-design against u-rev 5, `35aab95`, FAIL, 3 BLOCKING / 3 MAJOR / 5 MINOR) plus the folds of D-323 (M3 SELECTED, S-M) and D-324 (M4 STOPPED).

**Context was fresh.** I did not author this unit, its repair, either matrix, any red-team round, or any earlier review. I read `CLAUDE.md` first, then the prior report in full including "WHAT I COULD NOT BREAK", then `matrix_M3_selection.md`, `matrix_M4_stop_round3.md`, `matrix_M3_soundness_instrument_rev2.md`, `matrix_M3_REDTEAM_round2.md`, `section_owner_table.md` §10/§11, `U3_tier_t.md` §10, and D-316 through D-324 in `docs/decisions.md`.

**Scope taken:** the seven items in the brief. §9's SELECTION-OPEN slot is treated as settled and is not a finding. Whether S-M or the M4 stop were right is out of scope; whether U4 states them accurately is in scope.

**Reproducer discipline.** Every finding below carries a command and its real output. One probe needed a build: a throwaway `git worktree` at `/home/tom/.cache/u4rev-wt` (never `/tmp`; `CARGO_TARGET_DIR` not exported), removed with `--force` and `git worktree prune`, live tree verified clean after. I edited no repository file and ran no git write command in the live tree.

---

## VERDICT: **FAIL**

**1 BLOCKING, 2 MAJOR, 4 MINOR.**

The two folds themselves are **sound**, and this is the part of the round that passes decisively. I checked D-323's fold clause by clause against `matrix_M3_selection.md`, D-323 and D-322: the strongest surviving attack is quoted **byte-identical** to the record (only blockquote nesting differs), all five registered conditions are present and correctly stated including conditions 1 and 2, D-322's `22 of 174 / 12.6 %` is used everywhere and D-321's superseded `84 of 300 / 28.0 %` appears exactly once, explicitly as the figure **not** used. I re-ran the selection record's probe myself in a fresh worktree and it reproduces **to the digit** — 1700 / 174 / 22 / 12.6 % / 132 of 154 / 154 of 154 — so U4's pasted output is complete and verbatim. The D-324 fold matches the stop record and the ADR line on every clause I could check: which stop it is, the two axes, the surviving rows, N-K's fired flip clause and its measured cost, N-Q's absence, the two shipped-instrument defects, the author's-own-defects paragraph. Nothing in U4 reads as though an M4 seam were selected. BLOCKING 2 is genuinely discharged — S-E is **retired, not re-labelled**: the referent changed from "freshly written, in `pistol-search`'s own tree" to "R1, reused by `#[path]`", which is a different mechanism, and every site says so. BLOCKING 3 is discharged: no live statement of the staged-config count survives. MAJOR 4, 5, 6 and MINOR 7, 9, 10, 11 are each repaired in the shape the prior report asked for, and MINOR 9's split treatment is the unit's own discipline applied consistently, not an exception.

The failure is the same class the u-rev exists to close, at the one section the fold-in pass did not enumerate. **`docs/decisions.md` carries a landed line, D-320, that disposes of a residual U4 still records as the architect's open choice — and D-320 landed in the same commit as D-321, which this u-rev does fold.** U4-Z's B3 section was open in the editor at u-rev 6 (MINOR 8's and MINOR 9's corrections landed inside it) and was not re-read against the ADR log.

---

# FINDINGS

## BLOCKING

### 1. D-320 has landed and closes B3's residual; U4-Z's residual paragraph and the OPEN list both still describe the pre-D-320 state, and D-320 is not named anywhere in the unit

**Claim reviewed — U4:1240–1249, "THE RESIDUAL, NAMED, AND IT IS THE ARCHITECT'S":**

> This selection was made by architect ruling on the comparison above. That comparison states both options, both costs MEASURED, and each option's failure mode, but it carries no recommendation and **it was never put to a fresh-context DECISION-RED-TEAM**. … It is recorded here and in D-316 rather than left implicit, because an unattacked selection that nobody writes down is the silent drift the rule exists to refuse, and **an unattacked selection that IS written down is a debt the architect can choose to pay or to accept.** The reviewer of this unit is not asked to ratify it.

**And U4:1325, the OPEN list — the surface whose job is to be current:**

> - ~~**B3 — gate (b), the two shapes above.**~~ **CLOSED at u-rev 2** … **Its RESIDUAL is not closed** and is named there: the selection was not put to a fresh-context DECISION-RED-TEAM.

**Contradicting evidence — the architect has already chosen, in a landed ADR line.**

```
$ git log --oneline -S"D-320:" -- docs/decisions.md
0af32fb docs(decisions): three housekeeping lines land — R5 makes Tier Q's presence in
the node protocol not a scope finding, R6 closes D-316's residual by recording the
breach rather than arguing it away, and R8's errata gives the DEF-T convention question
a measured size
```

D-320's own text (`docs/decisions.md:685`), verbatim in the load-bearing parts:

> D-320: **B3's REPAIR — THE SOUNDNESS GATE'S FOUR NAMED PARTS, SHAPE 2 — WAS ADOPTED BY ARCHITECT RULING WITHOUT AN OPTION MATRIX, AND THAT IS A BREACH OF THE MATRIX LAW, RECORDED HERE RATHER THAN LEFT IMPLICIT.** … **THE RETRO-MATRIX IS WAIVED**, on two grounds stated together because neither carries it alone. … WHAT THIS LINE DOES NOT DO: it does not license adoption-without-matrix as a practice, and it does not convert D-316's residual into a clean record. **It converts an unattacked selection that nobody wrote down into a debt the architect has now paid** in the only currency available after the fact, which is disclosure. **Flips if a GATE-NAMING DEFECT surfaces** — a part of the soundness gate that the four names do not reach, a citation that resolves under the retired letters and not under the names, or a fifth gate appended as a letter — at which point the waiver was wrong, the matrix is owed retroactively, and this line is superseded by the round that runs it.

**Reproducer — D-320 is absent from the unit, and it landed in the same commit as D-321, which u-rev 6 does fold:**

```
$ grep -n "D-320" docs/experiments/U4_soundness_instrument.md
$ echo "exit $?"
exit 1
$ grep -n "\bR6\b" docs/experiments/U4_soundness_instrument.md
$ echo "exit $?"
exit 1
$ git show 0af32fb -- docs/decisions.md | grep -c "^+D-319:\|^+D-320:\|^+D-321:"
3
$ grep -c "D-321" docs/experiments/U4_soundness_instrument.md
9
```

**Why it breaks.** D-320 is *about U4-Z* by name — it quotes the two-shape comparison's own words back at it. It records the breach, **waives** the retro-matrix, states the ground for the waiver, and attaches a flip clause. U4 tells the architect the opposite: that the debt is still theirs to "choose to pay or to accept", and lists the residual as **not closed** in the list an architect reads to learn what is owed. Three consequences, each independent: (i) an architect reading U4's OPEN list is told to decide something already decided; (ii) D-320's flip clause — a gate-naming defect surfacing — is the condition under which the retro-matrix becomes owed again, and U4, the document where such a defect would surface, does not carry the clause; (iii) D-320's waiver *rests on this unit's review history* ("the fresh-context REVIEW-design at `docs/experiments/wp15b_U4_REVIEW.md` verified the six-site retarget … its scope-1 verdict on the named-gate wiring is PASS"), so the two documents are load-bearing on each other and only one of them knows it.

This is not a marginal miss. U4-Z's B3 section **was edited at u-rev 6** — MINOR 8's withdrawal of the false diagnosis (U4:1215–1230) and MINOR 9's recorded correction (U4:1192–1202) both landed inside it. The pass was in this section, three paragraphs from the stale one, and did not re-read it against the ADR log it was simultaneously citing. The head's own REVIEW STATUS row claims *"the fold-in was then re-run across the whole unit"* and then enumerates the sites re-read — *"§8.2's heading, §8.3's gate table, §8.7's wiring, §11.6, U4-T, U4-M, U4-Z's lead-in, items 4 and 15, the OPEN list and the closing line"*. U4-Z's B3 section is not in that enumeration, and the enumeration is what actually happened; "across the whole unit" is not.

**Fix scope (not mine to apply).** Two sites: U4:1240–1249's closing sentences, which must record D-320's ruling (breach acknowledged, retro-matrix WAIVED, debt paid by disclosure) and carry its flip clause; and U4:1325's OPEN-list bullet, whose "Its RESIDUAL is not closed" must state D-320's disposition instead. No ADR edit is needed — D-320 already exists and this is the unit catching up to it.

---

## MAJOR

### 2. The u-rev 6 record stamp at the head of §8 says "EVERYTHING FROM HERE TO THE END OF §8.7 IS RECORD … none of it is the state now" — which is false of four blocks the same u-rev wrote inside that span, and contradicts two other sites in the unit

**Claim reviewed — U4:283–286, added at u-rev 6:**

```
$ sed -n '283,286p' docs/experiments/U4_soundness_instrument.md
> **EVERYTHING FROM HERE TO THE END OF §8.7 IS RECORD.** Each part was true at the
> revision that wrote it and none of it is the state now: the matrix was authored
> (u-rev 4), every option fell (D-317), a second field of thirteen rows was authored and
> attacked, and **S-M IS SELECTED** (D-323 — the block above). Read the stub, the
```

The span it governs is lines 283 to 772 (`§8.7` ends at 772; `## 9.` begins at 774).

**Contradicting evidence — four blocks inside that span are the state now, and three of them were written at u-rev 6:**

```
$ grep -n "FOLDED AT u-rev 6" docs/experiments/U4_soundness_instrument.md
417:> **FOLDED AT u-rev 6.** The instrument for this gate is **S-M**: per-node **EQUALITY**
760:**FOLDED AT u-rev 6 — WHAT THIS SENTENCE SAID AND WHAT IT CAN AND CANNOT SPECIFY
$ sed -n '558p' docs/experiments/U4_soundness_instrument.md | cut -c1-130
| **THE DIFFERENTIAL GATE** — **S-M** (D-323): per-node EQUALITY against the LANDED referent R1, REUSED not rewritte
```

- **417** — §8.2's fold: *"The instrument for this gate is **S-M** … The heading said **SELECTION OPEN** from u-rev 1 to u-rev 5; **it is not open any more**."*
- **558** — §8.3's four-gate table, the row a reader consults to learn what the differential gate's instrument **is**.
- **760** — §8.7's fold: *"**THREE OF THE FOUR PARTS CAN BE SPECIFIED AND ONE CANNOT** … the script's differential part **cannot be written until that decision is made**."* This is the live specification `tools/staged_soundness_check.sh` is taken from.
- **588–590** — §8.3's live sentence *"**U3** (u-rev 3) §10 is the one place the number of staged config documents is stated, and this unit states no such number anywhere"*, which is BLOCKING 3's repair and is current text, not record.

**And two sites in the unit say the opposite.** The head's REVIEW STATUS row for BLOCKING 2 lists *"§8.3's gate table, §8.7's wiring and §8.2's heading are **retargeted** with it"* — retargeted to the current state. And U4:360–367, ninety lines below the stamp, says explicitly: *"**The disclaimer's scope was wrong and the u-rev 5 review found it so (BLOCKING 2):** §8.3's four-gate table and §8.7's wiring paragraph are **carve-authored at u-rev 2 under D-316, not carried verbatim** … both are retargeted at u-rev 6."*

**Why it breaks.** The prior round's BLOCKING 2 found the old disclaimer's scope too *narrow* — it did not reach U4-T and U4-M. The repair widened it to a blanket "everything from here to the end of §8.7", which now over-reaches in the other direction and asserts, in the unit's loudest voice, that the current specification of three of the four gates and the current statement of the selected instrument are **not the state now**. IMPL reads §8.7 for the script's enumeration and §8.3 for what each gate is; a stamp telling it that block is superseded record is a requirement gap, and it is contradicted by the same document twice. The unit demonstrably knows how to scope a stamp correctly — U4-Z's *"Its 'the carve does not choose' is the state AT SELECTION TIME"* is exactly that — and did not do it here.

**Fix scope.** One sentence at U4:283–285: scope the stamp to what it means (the stub, the five-row table, §8.1's and §8.2's S-E body prose, §8.4's ledger narrative), and exclude the u-rev 6 folds and the two carve-authored blocks the unit itself names three paragraphs earlier as retargeted.

---

### 3. U4-Z's u-rev 2 SELECTION block still asserts "S-E **is** the differential gate" in unmarked carve prose, while its two sibling sentences were both retargeted at u-rev 6

**Claim reviewed — U4:1174–1177, inside "#### SELECTION — SHAPE 2, by architect ruling. Landed at u-rev 2. ADR line D-316":**

```
$ sed -n '1174,1177p' docs/experiments/U4_soundness_instrument.md
they are; §8.7's wiring enumerates the four names; the S-E double-list dies with
the letters, because S-E is the differential gate and is named once, in §8.2.
Executed in this unit at §8.2, §8.3 and §8.7, and in **U3** §10 at the two sites
named below.
```

**Contradicting evidence — its two siblings were retargeted at u-rev 6, this one was not:**

- §8.3's table cell (U4:558) now reads *"**THE DIFFERENTIAL GATE** — **S-M** (D-323) … *This cell read "S-E, with the reduced S-C beside it" from u-rev 2 to u-rev 5; S-E fell in M3 round 1…*"*
- §8.7's wiring (U4:747–751) now reads *"the differential gate (§8.2: **S-M** — per-node EQUALITY of the emitted set against the LANDED referent R1…)"*, with the whole "FOLDED AT u-rev 6" paragraph beneath it.
- U4:1175 still says *"S-E **is** the differential gate"*, present tense, with no u-rev marker.

**It is not protected by the unit's unedited-comparison discipline.** The discipline is stated at U4:1129–1136: *"**THE TWO-SHAPE COMPARISON BELOW** IS THE TEXT THE ARCHITECT SELECTED FROM AND IS LEFT UNEDITED … **The selection**, and one MEASURED correction to a cost cell that execution falsified, **are recorded AFTER it, not inside it**."* Line 1175 is in the SELECTION block — the text recorded *after* the comparison — which the unit's own MINOR 9 handling classifies as editable carve prose (*"§8.7's copy IS repaired in place, because that one is carve prose and not the selected-from text"*). The comparison proper, at U4:1141–1165, does carry S-E and is correctly left alone.

**Why it breaks.** Same section, same missed pass as BLOCKING 1. A reader arriving at U4-Z's B3 section — which is where D-316's ground and the retarget table live, and which other documents cite — finds the differential gate identified as S-E in the unit's own voice, six hundred lines after the head says S-M is selected. The three sibling sentences were written to say one thing; two now say S-M and one says S-E.

**Fix scope.** One clause at U4:1175, on the same annotation pattern §8.3's cell already uses: state that this describes the u-rev 2 execution and that the differential gate's instrument since D-323 is S-M.

---

## MINOR

### 4. U4-M item 1 points the wrong way for its own repair: "the amendment is carried **below** … with this note against it" — §9.1 amendment 4 is a hundred lines **above**, and carries no note at its own site

```
$ grep -n "^### 9.1\|^## U4-M" docs/experiments/U4_soundness_instrument.md
968:### 9.1 The five amendments the design made to N-A after that attack
1062:## U4-M. What this unit measures
$ sed -n '1094,1095p' docs/experiments/U4_soundness_instrument.md
     EXIST.** The seconds are real; the attribution is not. The amendment is carried
     below as RECORD with this note against it.
$ sed -n '1019,1024p' docs/experiments/U4_soundness_instrument.md
**4. The instrument is named with its revision, and BEFORE is re-taken.** Revision
1 invoked the instrument clause against N-C without satisfying it for N-A:
`tools/baseline_snapshot.sh` was named twice and never with a revision. And N-A
**is** a change to that instrument, so the BEFORE run — taken under the
pre-`--config` script — is re-taken under the amended one. **MEASURED 34.5 s.**
Not worth an argument.
```

MAJOR 6's repair is otherwise correct and complete — `f317385` is named, the `ls` output is pasted and reproduces at HEAD (`ls: cannot access 'configs/instrument_staged_v0.toml': No such file or directory`), and the AFTER's double blockage is stated. But the note that discharges the misattribution sits after the text it is against, is signposted in the wrong direction, and leaves a reader who meets §9.1 amendment 4 first with an unqualified **MEASURED 34.5 s** attributed to a script that has never existed. Either the direction word is wrong or the note belongs at §9.1.

### 5. §8.3's "this unit states no such number anywhere" is falsified four lines later by the quoted clause it says is deleted

```
$ sed -n '587,595p' docs/experiments/U4_soundness_instrument.md
  `gate_staged_v0.toml` for the five gate cases. The remaining staged documents —
  the SPRT seat and the play config — keep the cut. **U3** (u-rev 3) §10 is the one
  place the number of staged config documents is stated, and this unit states no such
  number anywhere. *The clause that stood here until u-rev 6 — "which is why there are
  four and not three (**U3** §10, the one place that count is stated)" — stated the
  count inside the very clause naming elsewhere as the only place it may be stated.
```

The count is no longer stated *live* — BLOCKING 3 is discharged — but the absolute claim "states no such number anywhere" is contradicted by the record quotation in the same sentence. The prior report's fix shape was *"the 'four and not three' clause can be dropped without touching the derivation"*; it was dropped from the live text and reproduced verbatim beside it. This is the third consecutive revision of this one sentence in which the defect it names reappears inside the repair that names it, now one level up. Either the absolute claim narrows to "states no such number as a live claim", or the quotation is replaced by a description of the deleted clause.

### 6. The REVIEW STATUS row for the prior reviewer's "one observation for the architect" describes a state the same commit fixed

```
$ sed -n '115p' docs/experiments/U4_soundness_instrument.md | cut -c1-190
| the reviewer's **"one observation for the architect"** — `section_owner_table.md` §11 records U4 at a stale size and u-rev | **NOT THIS UNIT'S TO FIX.** The owner table is another document; it is na
$ sed -n '256,262p' docs/experiments/section_owner_table.md | grep "U4 soundness"
| U4 soundness instrument | 701 | 800 | 855 | **1413** | 6 |
$ grep -n "now, MEASURED at" docs/experiments/section_owner_table.md | head -1
252:| Unit | at the carve (u-rev 1) | at u-rev 2 of this table | at u-rev 3 (`0af32fb`) | now, MEASURED at `7358a07` | u-rev now |
$ wc -l docs/experiments/U4_soundness_instrument.md
1413 docs/experiments/U4_soundness_instrument.md
```

At `7358a07` — this unit's own commit — the owner table records U4 at **1413 lines and u-rev 6**, which is current. The **disposition** column, whose whole job is to state the u-rev 6 state, still reports the observation as live. The owner table itself is owned elsewhere and I charge nothing there; the stale claim is U4's. (U4 correctly asserts no line count of itself — `grep -n "wc -l\|line count\|[0-9]\{3\} lines"` returns only the line-115 statement that it asserts none. Rule 9 is clean.)

### 7. The selection SHA is cited two ways, and only one of four sites disambiguates

U4:157–158 discloses it correctly: *"(the selection was taken at `809b5db`, the revision carrying the attack, and landed at **`af8082a`**)"*. But U4-A's round-2 row, the OPEN list's B1 bullet and the closing line each say only **"S-M SELECTED at `af8082a`"**, while `matrix_M3_selection.md`'s own header says **"Selected at `809b5db`"** and the file itself exists only from `af8082a`:

```
$ git log --oneline --diff-filter=A -- docs/experiments/matrix_M3_selection.md
af8082a docs(experiments): MATRIX M3 selects S-M …
$ head -3 docs/experiments/matrix_M3_selection.md | tail -2
**Selected at `809b5db`** by the architect-delegated session, from
```

Both readings are defensible and U4's §8 statement is the right one; the three abbreviated citations conflict with the record's own header at sites that carry no marker. One clause each, or a single convention.

---

# VERIFIED WITH NO FINDING

- **The strongest surviving attack is quoted, not softened.** Normalised for blockquote nesting and emphasis markers, U4:184–192 is **character-identical** to `matrix_M3_selection.md`'s block and to D-323's quotation. Diffed programmatically; the only opcodes returned were inserted `> ` prefixes.
- **All five registered conditions are present and correctly stated.** Condition 1 (R1 reused by `#[path]`, second freshly-written referent **FORBIDDEN** without a registered agreement criterion *and* a registered consequence for disagreement) and condition 2 (`0 of 3406` may not be cited as evidence about the convention) are both complete, including both halves of condition 1's second-instrument requirement. Conditions 3, 4 and 5 match the record, and condition 4 correctly carries S-N's MEASURED `1.76 M / 2.61 M`.
- **`0 of 3406` is genuinely not cited as evidence.** `grep -n "3406"` returns two hits: U4:186, inside the quoted attack that explains why it may not be cited, and U4:215, the prohibition itself.
- **D-322's figure is used and D-321's is not.** `22 of 174 / 12.6 %` at U4:198, 270, 438 and 1057; `84 of 300 / 28.0 %` at U4:199 only, in the clause *"and **NOT** D-321's original …"*, with the population defect correctly diagnosed (both sides at fixed `StonesLeft::Two`/`HitBudget::Two` versus the mover at the phase-derived budget).
- **The re-taken measurement reproduces exactly and the paste is complete.** I extracted the probe verbatim from `matrix_M3_selection.md`, ran it at `7358a07` in a throwaway worktree, and got, MEASURED:
  ```
  running 1 test
  mover positions (PROTO-NODE evaluated) = 1700
  FILTERED (U2 5.3, mover only, phase-derived left/budget) = 174
  conventions DIFFER = 22  (12.6 % of FILTERED)
  pop-mutant applicable = 154; S-K fires 132 (85.7 %), S-M fires 154 (100.0 %)
  test the_protocol_row_as_u2_states_it ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
  ```
  Line for line what U4:260–275 pastes, modulo the worktree path and the test-binary hash. U4's framing — *"It is the SAME instrument as the selecting session's, so it is a REPLICATION and not a second instrument"* — is honest and is the correct reading of CLAUDE.md's clause.
- **Both pasted code blocks in the unit are complete and verbatim.** There are exactly two (`grep -n '```'` → 260/275 and 1086/1089). The second, `ls configs/instrument_staged_v0.toml`, reproduces at HEAD.
- **BLOCKING 2 is discharged and nothing is silently re-labelled.** Every `S-E` outside the §8.1–§8.7 record span (87 occurrences audited by line) is one of: "S-E fell / is superseded / until u-rev 6", the two-shape comparison left unedited by disclosed discipline, or the one site at finding 3. U4-T's differential row registers R1 reused by `#[path]` with the DEPENDS-OPEN-THEORY mark, condition 2's prohibition and the OPEN seam; U4-T's `visit_searches_every_forced_candidate` row is marked **OPEN — not selected and not rejected, and IMPL may not read it as registered**; U4-M's cost row is marked **UNGROUNDED at its dominant term** with the 40–90 s correctly identified as CARRIED and therefore not independent corroboration of itself. The referent really did change, so this is a substitution and not a rename.
- **BLOCKING 3 is discharged.** I re-ran the prior reviewer's line-break-tolerant scan (joined lines, `\b(four|three|two|second|third|fourth)\b` with 160 characters of context filtered on `config|document`). No live statement of the staged-config count survives. The three sub-cardinalities about the *tactical pair* — §8.3's "BOTH staged TACTICAL configs", U4-T's "the **two tactical** staged configs", item 15's "The two TACTICAL staged configs", plus the two MAJOR-4 record sites — state how many of the four are tactical, not the count. The owner table's ruling is specific: *"**The count is FOUR**, and `docs/experiments/U3_tier_t.md` §10 is the only place in the carve that says so"* (`section_owner_table.md:240`), and U3 §10's own text is *"**FOUR** complete documents … This is the one place the count is stated"* (`U3_tier_t.md:325–326`). U3 §10 states the tactical/gate split itself, in its own table's "why" cells. The three sub-cardinalities are **covered**. The quoted false B4 sentence in the change log is a quotation labelled false in both halves and does not state the count.
- **The M4 fold is accurate against `matrix_M4_stop_round3.md` and D-324**, clause by clause: the recorded-tie characterisation, the survivors (N-E, N-J, N-K, N-F, N-L), the two axes and their membership lists, N-K composing rather than rivalling with the red team's own strongest-attack line quoted, N-K's flip clause firing at 8 added lines / 0 removed / ~2× wall (2 × 33 s), axis A's guard ground measured away with three lines at `baseline_snapshot.sh:289`, N-Q as SHELL_CHECKLIST item 11 absent from all three revisions, the two shipped-instrument defects, and the three-authoring-sessions pattern. **Nothing in U4 reads as though an M4 seam were selected**, and the extension of the no-ADR-line prohibition to N-K/N-M/N-L/N-Q is disclosed as an extension.
- **Item 15's UNRECONCILED handling (scope 5a) is correct.** D-324 states *"U4-Z item 15 stays blocked"*; the prior report found no evident dependency on the snapshot's seam. U4 records the ADR as binding, the disagreement as open, and overrules neither, in both U4-Z's lead-in and the OPEN list. That is the only disposition available to a unit that may not amend an ADR, and the false B3 reason and the stale "M3's fresh matrix" are both deleted rather than argued.
- **The D-316 residual handling (scope 5c) is correct.** MINOR 8's false diagnosis is withdrawn in U4's own copy, the COUNT SIX is kept and correctly re-grounded (*"it counted one inside site and missed three others"*), and the OPEN list carries the D-316 residual to the architect with the right remedy: *"in an append-only log, by a new line and not by an edit."*
- **MINOR 9's split treatment is consistent, not convenient.** §8.7's copy is carve prose and is repaired in place; the quotation now matches `ec8f7fb:502` (*"(a) **the** tactical suite **at 100 % of its** pre-registered thresholds under Staged"*), verified against `git show ec8f7fb:docs/experiments/wp15b_design.md | sed -n '501,504p'`. U4-Z's copy is inside the selected-from comparison and the correction is recorded after it, on the same discipline as the cost-cell correction that was already handled that way at u-rev 2. The rule applied is "selected-from text is never edited; carve prose is", and it is applied the same way in both places.
- **MINOR 7, 10 and 11 are repaired as asked.** The stub column now reads *"the differential gate's instrument"* with the retirement noted (U4:312); §9's third conjunct is deleted with the deletion recorded (U4:828–833); MAJOR 8's residual now has two parts, part (ii) being M6's owed PARENT position, correctly stated as not gated on `staged.rs` existing (verified: `ls crates/pistol-search/src/staged.rs` → *No such file or directory*).
- **MAJOR 5 is repaired.** §8.4's M3 cell withdraws "BUILT" as false, restates the `{a,b} {b,d} {d,e}` shape as a required *property*, states that a position a legal game reaches is OWED, and the row is named in the OPEN list rather than left inside a cell that says BUILT.
- **The matrix field's shape checks out.** Revision 2 carries thirteen rows — S-A…S-I carried with round 1's verdicts, plus S-J, S-K, S-L, S-M, which are exactly the four D-317 named as missing. The red team's *"All eight facts reproduce"* is its own words (`matrix_M3_REDTEAM_round2.md:20`). S-J's fall is the red team's verdict (`:27` — *"**S-J** | **FALLS** — carried inheritance of S-E's four kills, unchallenged"*), so §8.2's ruling that the reduced S-C is a costed proposal with no surviving matrix row is grounded and is not a design decision made under cover of the repair.
- **No number added at u-rev 6 lacks its mark**, and no line count of this file is asserted anywhere.
- **U4-A now carries a row for each of the five DECISION-RED-TEAM rounds and for the u-rev 5 review**, and "what this unit owes" is updated off the stale "two fresh DECISION-RED-TEAMs".

---

# REJECTED WITH ATTEMPTED REPRODUCER

- **"The unit still restates the tactical sub-cardinality at five sites, not the three the repair names."** Attempted: the line-break-tolerant scan above returns "two tactical / BOTH tactical" at U4:581, 1056, 1273, 1298, 1396. **Rejected** — the owner table's ruling and U3 §10's claim are both about the count **FOUR** of staged config *documents*, not about how many are tactical, and U3 §10's own table and derivation state the tactical/gate split themselves. Not a restatement of the owned count at any of the five.
- **"'all eight of its facts reproduced' misattributes the matrix's facts to the red team."** Attempted: `grep -n "eight facts" docs/experiments/matrix_M3_REDTEAM_round2.md` → `20:**All eight facts reproduce.**` and `230:- **All eight facts reproduce.**`. **Rejected** — the red team's own report uses that exact phrase in its own voice; the pronoun is loose but the claim is the source's.
- **"§8's item (d) attributes S-E's second half to the selection record's 'what this selection does not decide', which lists only three items."** Attempted: the record's list is the seam, the corpus/per-CI cost, and the convention — three items, not four. **Rejected** — U4's heading is *"WHAT D-323 DOES NOT DECIDE"*, which is a true predicate whether or not the record enumerates it, and U4-T's *"nothing in D-323 carries this half"* is separately true and correctly reasoned from S-M being a criterion over the emitted set alone.
- **"U4 understates the M4 round-3 truncation — the stop record says the paste carried ten lines and the re-run returned twelve, so two lines were omitted, not one."** Attempted: `matrix_M4_stop_round3.md` says both *"the omitted one is `tools/bench_delta.sh:120`"* and *"the same filter returns **12** lines"*. **Rejected as U4's** — D-324 states *"truncated by one line — `tools/bench_delta.sh:120`"*, and U4 follows the landed ADR line exactly. Any tension is between the stop record and D-324 and is owned there, not by U4.
- **"U4-M's cost table carries 34.5 s as MEASURED for a run the same item says was misattributed."** Attempted: U4:1108 gives *"One baseline snapshot | ~35 s | **34.0 / 34.5 s**"*. **Rejected** — item 1 states in terms that *"The seconds are real; the attribution is not"*, and the prior reviewer independently measured 33 s on this machine. The wall time is a sound MEASURED figure; only its attribution to an amended script was wrong, and that is finding 4's siting problem, not a number problem.
- **"The two-shape comparison at U4:1141–1165 still presents S-E as the differential gate's instrument."** Attempted: U4:1151 and 1163 both name S-E. **Rejected** — that block is the text the architect selected from and is protected by the unit's stated, consistently applied discipline (*"a comparison corrected after the decision it fed is a comparison the decision was never made against"*), stamped at U4:1129–1136 with *"Its 'the carve does not choose' is the state AT SELECTION TIME."* Finding 3 concerns the SELECTION block **after** the comparison, which the same discipline classifies as editable.

---

**Cross-unit items noted and not charged to U4:** `section_owner_table.md` §11 is current at `7358a07` (1413 / u-rev 6) — the observation the prior round handed the architect is discharged, and only U4's record of it is stale (finding 6). U2's and U3's own open findings are untouched by this round. `matrix_M4_stop_round3.md`'s internal ten-versus-twelve tension is that record's.

*REVIEW-design of `docs/experiments/U4_soundness_instrument.md` u-rev 6, at pinned revision `7358a07` (matches HEAD, tree clean). Fresh context; not the author of the unit, its repair, either matrix, any red team, or any earlier review. Every finding reproduced before reporting; every numeric claim marked MEASURED with its command. Verification worktree at `/home/tom/.cache/u4rev-wt`, removed and pruned; live tree verified clean (`git status --porcelain` → empty). This report modifies no repository file and is not committed.*
