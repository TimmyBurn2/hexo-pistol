# MICRO-MATRIX U4-R, REVISION 2 — what to do about U4's restatement surface

Status: **AUTHORED, NOT SELECTED.** Awaits fresh-context DECISION-RED-TEAM.

**WHY THERE IS A REVISION 2.** Revision 1 (`docs/experiments/matrix_U4R_restructure.md`,
`4d84694`) was attacked by a fresh-context DECISION-RED-TEAM
(`docs/experiments/matrix_U4R_REDTEAM.md`, at `53c0c0b`, against `4d84694`) and
**THE FIELD FELL**: 6 KILL, 8 MAJOR, 11 MINOR/WOUND, recommendation (b) **FALLS**,
(c) **FALLS AS AUTHORED**, and the report's own verdict is *"NO SELECTION MAY BE
TAKEN FROM THIS MATRIX."* It named two missing rows, one of which dominates the
recommendation, and instructed: *"Re-author with **(e)** and **(c′)** in the field,
replace the line-count cost column with a findings-reached column the data already
supports, and re-attack."* **This revision does exactly that.** Revision 1's rows are
carried below with the verdicts that attack gave them, and are not revived.

**THE AUTHOR'S OWN DEFECT, RECORDED FIRST BECAUSE IT IS THE SHARPEST FACT ABOUT
REVISION 1.** Revision 1 disclosed what it called a discrepancy in its measurer's
report, quoting a summary line reading *"20 live-line occurrences"*. **The red team
established that no such line exists in the instrument at any revision** — the string
occurs in exactly one place in this repository, inside revision 1, as a quotation
attributed to a file that does not contain it. The true figure was never in doubt:
**22 matching lines, 23 occurrences**, and the instrument's own paste says so. The
author had taken the phrase from the measuring agent's transient completion message
rather than from the landed instrument file, and the same mistake produced revision
1's other two transcription defects (F3's `705` for `707`, and three wrong section
counts in F4). **This is D-331's disease occurring inside the document whose declared
governing law is D-331**, in the paragraph announcing the author's fidelity to D-330.
It is recorded here, not smoothed, and it is the ground for revision 2's own sourcing
rule below.

**SOURCING RULE FOR EVERY MEASURED CELL IN THIS REVISION.** Every number below is
transcribed from a **command and its pasted output in a file that is in the tree** —
`docs/experiments/matrix_U4R_measurements.md` (the stakeless measurer, D-328/D-330)
or `docs/experiments/matrix_U4R_REDTEAM.md` (the red team's re-runs, which D-328
expressly permits: *"it does not apply to a red team, whose whole job is to re-run
the author's commands"*). **No cell is transcribed from any agent's summary or
completion message.** Where the two files disagree, the red team's re-run is cited,
because it is the later and the attacked one. Every cell names which file it comes
from. R7: command and output inline.

**PROVENANCE OF THE RECOMMENDATION.** Revision 1's recommendation, (b), came from the
architect's dispatch and fell. **The recommendation below is the author's, derived
from the red team's own findings-by-region measurement, and no option in this
revision was named in any dispatch.** The red team's M8 recorded that a dispatch-
supplied recommendation is *"an unattacked selection wearing a matrix on the field's
face"*; that condition does not recur here, and an attacker should verify rather than
accept it.

**Subject:** `docs/experiments/U4_soundness_instrument.md` at u-rev 7.
**Ground:** `docs/experiments/wp15b_U4_REVIEW_urev7.md` — FAIL, 0 BLOCKING / 4 MAJOR /
4 MINOR — whose reviewer determined, independently of its severity ratings, that
*"the trigger for an architect restructure of **U4's status matter** is met; another
patch-and-re-review round is the wrong instrument."* **The trigger's scope is STATUS
MATTER, and revision 2 treats that as binding on the field rather than as a licence
for any change** (red team M6).
**Governing law: D-331 (R15), landed `c9d4e64`.**

---

## FACTS

**F1. WHERE THE FINDINGS ARE, BY REGION, WITH DEFECT DENSITY.** MEASURED by the red
team, `matrix_U4R_REDTEAM.md` K4, taking each finding's site from
`wp15b_U4_REVIEW_urev7.md` and each region's span from the measurer's section map:

```
region                        lines  MAJ  MIN  all  all/100ln  MAJ/100ln
S8 head fold  205-459           255    0    0    0       0.00       0.00
S9 head fold  865-1257          393    0    2    2       0.51       0.00
both head folds (union)         648    0    2    2       0.31       0.00
U4-Z          1441-1886         446    3    0    3       0.67       0.67
S8.7          814-864            51    1    1    2       3.92       1.96
head          1-172             172    1    1    2       1.16       0.58
U4-M          1362-1440          79    0    1    1       1.27       0.00
rest of unit (not head folds)  1238              6       0.48
findings wholly outside both head folds: ['MAJOR 1', 'MAJOR 2', 'MAJOR 3', 'MAJOR 4', 'MINOR 5', 'MINOR 6']
findings touching a head fold: ['MINOR 7', 'MINOR 8']
```

**THIS IS THE FACT THE FIELD IS NOW RANKED BY, AND IT REVERSES REVISION 1's RANKING.**
The two head folds are the largest targetable mass in the unit and carry **zero
MAJORs at the lowest density measured**. §8.7 is fifty-one lines and carries **12.6×
the fold density**. Ranking by size selected the cleanest region in the document.

**F2. THE 638 "FOLD" LINES ARE TWO DIFFERENT KINDS OF TEXT, AND ONLY 256 ARE MATRIX-
DERIVED.** MEASURED by the red team, K1:

```
$ grep -n "^> ## \|^> ### " docs/experiments/U4_soundness_instrument.md | head -9
207:> ## **SELECTED — S-M, AND IT IS NOT S-E (D-323)**
343:> ## THE RECORD OF THE EARLIER STATES — kept, STAMPED at u-rev 6, and RE-SCOPED at u-rev 7
376:> ### u-rev 1 — SELECTION OPEN — M3 HAS NO MATRIX, AND S-E IS NOT SELECTED HERE
867:> ## **SELECTED — N-E, AND IT IS NOT THE ROW THE MATRIX RECOMMENDED (D-329)**
989:> ## THE RECORD OF THE EARLIER STATES OF §9 — kept, STAMPED, and SUPERSEDED IN PART AT u-rev 7
1002:> ## THE RECOVERY (T1'), THE DIFF, AND WHY THE SELECTION DOES NOT STAND
1010:> ### DIFF 1 — recovered text vs. the text the DECISION-RED-TEAM attacked
1020:> ### DIFF 2 — recovered text vs. an EXTERNALLY DERIVED referent
1037:> ### VERDICT: **DIFFERS. SELECTION OPEN. The carve does not select N-A.**
```

```
$ for r in "207 342" "343 459" "867 988" "989 1257"; do set -- $r; printf '%s..%s bq=%s total=%s\n' "$1" "$2" "$(awk -v a=$1 -v b=$2 'NR>=a&&NR<=b&&/^>/{c++}END{print c+0}' docs/experiments/U4_soundness_instrument.md)" "$(( $2-$1+1 ))"; done
207..342 bq=135 total=136
343..459 bq=115 total=117
867..988 bq=121 total=122
989..1257 bq=267 total=269
```

**135 + 121 = 256 SELECTED-fold lines; 115 + 267 = 382 `THE RECORD OF THE EARLIER
STATES` lines.**

**F3. THE 382 RECORD LINES HAVE NO OTHER HOME IN THE TREE.** MEASURED by the red team,
K1:

```
$ for pat in "5e8c5e4a1e7ad416" "DIFF 2 — recovered" "MECHANICAL RECOVERY"; do echo "--- $pat ---"; grep -rln -- "$pat" docs/; done
--- 5e8c5e4a1e7ad416 ---
docs/experiments/U4_soundness_instrument.md
--- DIFF 2 — recovered ---
docs/experiments/U4_soundness_instrument.md
--- MECHANICAL RECOVERY ---
docs/experiments/U4_soundness_instrument.md
```

**F4. THE TOKEN-OCCURRENCE SURFACE IS 68 % OUTSIDE THE FOLDS, AND IT IS ANTI-CORRELATED
WITH THE DEFECT CLASS ANYWAY.** MEASURED by the red team, K2 — 101 of 314
occurrence-lines (32 %) inside the two fold spans, 213 (68 %) outside; `D-320`'s
twenty occurrences are 100 % outside. **And the row that corresponds to a real defect,
`eight of twelve`, has count 1** (measurer, M6). **This revision therefore does NOT
use token counts to rank options.** They measure pointer density, and D-331 says in
terms that a pointer is not a finding. The fact is carried because revision 1 rested a
ground on it and a reader must be able to see why that ground is gone.

**F5. STATUS MATTER, MEASURED.** From the measurer's M3, M4 and M12 (all reproduced by
the red team):

```
status_start=115  theory_para=167  span=52
U4-A start=173  s8_start=205  span=32
lines naming a u-rev of this unit: 160
lines naming a wp15b_U4_REVIEW report: 31
lines carrying REPAIRED/WITHDRAWN/RE-CHECKED/MARKED AT: 34
```

**F6. CENSUS MEMBERSHIP IS A MARKER, NOT A DIRECTORY** — so a new file need not be a
reviewable carve member. MEASURED by the red team, K6:

```
$ sed -n '656,659p' crates/pistol-solver/tests/wp15b_census.rs
const CARVE_MARKER: &str =
    "<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->";
```

This falsifies revision 1's second ground against option (c) and is why (c′) is in the
field.

**F7. WHAT NO OPTION IN EITHER FIELD REACHES.** From the red team's M4/M5 table,
against `wp15b_U4_REVIEW_urev7.md`: **MAJOR 1's §8.7 blockquote half (U4:833–837),
MINOR 5 (U4-M item 1) and MINOR 7 (§9's unmarked `91 test lines`) are reached by no
row in any field authored so far.** They are REPAIRS, not restructures, and this
matrix says so rather than implying a restructure will collect them.

---

## OPTIONS

**Cost is stated as FINDINGS REACHED and CLASS INSTANCES PREVENTED.** Line counts
appear only as a secondary figure, explicitly marked as a PROXY FOR AN UNDEFINED
PROPERTY — "reviewable in one sitting" has no instrument in this project, which
`section_owner_table.md` §11 hands to the architect and does not answer.

| Option | The cut | Findings reached (of 8) | Class instances prevented | Failure mode |
|---|---|---|---|---|
| **(n) NULL** — repair the eight findings in place, restructure nothing | U4 keeps its shape. | **8 of 8** — a repair reaches every finding by construction. | **NONE.** It is the instrument that produced three consecutive rounds of the class. | **NOT SELECTABLE, and the reason is narrower than revision 1 said.** The reviewer's trigger refuses *"another patch-and-re-review round"*, and it is real. But its scope is **U4's status matter** (red team M6), so it forbids doing nothing about status matter — it does not license a change elsewhere. **(n) is dead as a whole-round answer and alive as the necessary complement of every other row**: every option below still owes the repairs. |
| **(a) INDEX** — U4-Z's body becomes a pointer index | U4-Z (446 lines) replaced by one line per owed ADR line and per OPEN item, each naming its home. | **3 of 8** — MAJOR 2, 3, 4. | **The three stale-dependent MAJORs of this round**, which are the whole recurrence. | **ITS OWN FLIP CLAUSE HAS ALREADY FIRED** (red team m6): U4-Z's item-15 unreconciled-blockage bullet is content that lives nowhere else in the tree, so the index cannot be total and at least one item keeps its home in U4-Z. That is a scoping cost, not a kill — revision 1's flip clause 2 already prescribed exactly this remedy. Reaches nothing in the head. |
| **(c′) STATUS EXTRACTION, HAND-MAINTAINED** — no generator | The head's REVIEW STATUS block (52), U4-A's lineage table (32) and the 34 marker lines move to a hand-maintained STATE section or companion file carrying no `CARVE_MARKER` (F6); the unit's body keeps no status prose. **No `tools/` artefact, no generator, no SHELL_CHECKLIST review, no driving test.** | **~1.5 of 8** — the head halves of MAJOR 1 and MINOR 6, which are the false universals in the REVIEW STATUS table. | **The completeness-universal class** — a status table that asserts *"no live sentence in this unit …"* about a body it does not own cannot assert it. | It reaches roughly one and a half findings and leaves all three of U4-Z's MAJORs where they are. Revision 1 killed this row on two grounds and **both are gone**: the generation costs it was charged are costs of a generator it no longer has, and its census ground is false (F6). It cannot be selected alone. |
| **(e) EXTRACT THE RECORD** — the 382 `RECORD OF THE EARLIER STATES` lines move out | The two record blocks are MOVED to a companion record file and replaced by one citation each. **The two SELECTED folds (256 lines) stay**, being what the unit is bound by and what D-331 calls the preferred instrument. | **0 of 8.** | **NONE measured.** F1 puts zero MAJORs and 0.00 MAJOR/100 lines in this region. | **IT IS A READABILITY CHANGE AND THIS MATRIX WILL NOT PRICE IT AS A DEFECT REMEDY.** It destroys nothing (unlike revision 1's (b), which deleted these same lines) and needs no restatement-versus-normative-use judgement, which is why the red team ranked it above (b). But bought against the eight findings it buys nothing, and the property it does buy has no instrument. **Carried as a live row and NOT recommended**, so the architect can take it on readability grounds if that is the ground they want — stated as such rather than dressed as a defect fix. |
| **(f) = (a) + (c′) — INDEX U4-Z *and* EXTRACT THE HEAD'S STATUS MATTER. RECOMMENDED.** | Both cuts above, and nothing else. No record is deleted or moved; no fold is touched; no `tools/` artefact is created. | **~4.5 of 8** — MAJOR 2, 3, 4, the head half of MAJOR 1, and MINOR 6. **All four MAJORs are touched and three are fully reached.** | **BOTH class surfaces the round produced**: U4-Z's stale-dependent claims and the head's false completeness universals. | It reaches neither MAJOR 1's §8.7 blockquote half nor MINOR 5 nor MINOR 7 (F7) — those are repairs and are owed regardless. It is two changes in one option, so a red team should test whether they are separable and whether either half carries the other. And its own composite is only as good as (a)'s flip clause, which has fired. |
| ~~**(b) UN-FOLD**~~ | ~~(a) + the 638 fold lines replaced by normative use and citation~~ | — | — | **FELL** in round 1 (`matrix_U4R_REDTEAM.md`, K1–K5, M4, M5). Carried with the verdict that attack gave it and NOT revived: 60 % of its cut is D-331-protected record with no other home; its ranking ground reads F4 backwards; its un-fold half reaches no MAJOR. |
| ~~**(c) GENERATED STATE FILE**~~ | ~~(c′) plus a `tools/` generator~~ | — | — | **FELL AS AUTHORED** (K6) — but only the GENERATOR fell. The row that survives it is (c′), above. |

---

## RECOMMENDATION

**(f).** Three grounds, and each is the one the red team's own measurement supports.

1. **It is the only row whose scope is the trigger's scope.** The reviewer's trigger
   names *U4's status matter*. U4-Z's OPEN list and ADR lines, and the head's REVIEW
   STATUS block, are that matter. (e) is not; (n) refuses it; (a) and (c′) each take
   half of it.

2. **It is ranked by findings, not by mass, and it wins on findings.** 4.5 of 8, all
   four MAJORs touched, against (a)'s 3, (c′)'s 1.5 and (e)'s 0. On mass it is the
   *smaller* change than (e) and much smaller than the fallen (b) — which is the point:
   F1 measures that mass and defects point in opposite directions in this unit.

3. **It deletes no record and needs no judgement call.** Both halves relocate claims
   into pointers or into a file; neither requires separating "normative use" from
   "restatement", which is the judgement whose risk killed (b)'s cut cell.

## WHAT THIS MATRIX DOES NOT CLAIM

- **It does not claim a restructure repairs the round's findings.** Five of the eight
  are repairs whichever row is selected, and three of those (F7) are reached by no
  row in any field. **A selection here is about RECURRENCE, not about this round's
  eight defects**, and revision 1's failure to say so is part of why it fell.
- **It does not claim "reviewable in one sitting" is improved.** That property has no
  instrument. Every line count in this document is a proxy and is marked as one.
- **It does not claim the CLAIM-HOME law is followable by care alone.** The evidence
  says otherwise: the session that landed D-331 and declared it governing broke it in
  the next document it wrote. That is recorded at the head of this file and is a
  finding for the architect, not for this matrix to settle.

## WHAT FLIPS IT

Each trigger names a remedy that answers that trigger.

- **If the red team shows (f)'s two halves are not separable — that indexing U4-Z
  requires moving the head's status matter or vice versa — then (f) is not a
  composite but a single change described as two**, and the field is re-opened to
  state it as one row with one cost. Trigger: separability. Remedy: restate the row.
- **If the red team shows the head's REVIEW STATUS block cannot be moved without
  leaving the unit unable to state its own review status** — i.e. that D-311's label
  discipline requires the block to be in the unit — then **(c′) is dropped and (a) is
  selected alone**, because (c′)'s whole content is that move. Trigger and remedy are
  both about (c′).
- **If the red team measures that U4-Z's pointer index cannot name a home for more
  than one owed item** — (a)'s flip clause has fired once, for item 15 — **then the
  index is not the instrument and (c′) is selected alone.** One exception is a
  scoping cost; a pattern of them means U4-Z is the home.
- **If the red team finds a row that reaches more than 4.5 of the eight findings
  without deleting record**, that row is selected over (f) by domination, on the
  definition this project has used twice: satisfying the grounds identically while
  owing less.
- **(e) revives as a selection if the architect states a ground for it that is not a
  defect-rate ground** — readability, or a defined and instrumented "one sitting"
  property. This matrix cannot supply that ground and says so.

---

*Micro-matrix U4-R revision 2. Authored after `matrix_U4R_REDTEAM.md` (at `53c0c0b`)
killed revision 1's field. Every MEASURED cell is transcribed from a command and its
pasted output in a tree file, named per cell; none from any agent's summary message.
D-328/D-330 (R11), R7, D-331 (R15).*
