# MICRO-MATRIX U4-R — FRESH-CONTEXT DECISION-RED-TEAM

**SUBJECT REVISION:** `docs/experiments/matrix_U4R_restructure.md` at **`4d84694`**,
together with its declared instrument `docs/experiments/matrix_U4R_measurements.md`
at the same commit.

**DOES IT STILL MATCH HEAD? NO — THE TREE MOVED FOUR TIMES UNDER THIS ATTACK, AND I
STATE IT LOUDLY RATHER THAN SILENTLY RE-RUNNING.** HEAD was `4d84694` when this attack
began. At exit:

```
$ git rev-parse HEAD
7473a6fb6c775e0e87cd7dd9821b459be6cc41f5
$ git status --porcelain
?? docs/experiments/matrix_U4R_REDTEAM.md
$ git log --oneline 4d84694..HEAD
7473a6f docs(experiments): U3 reaches u-rev 5 — both MAJORs were manufactured by u-rev 4's own repair, so the completeness claim and the bare cross-unit u-rev are removed rather than corrected, and a citation now names the revision it was true at
7dfd047 docs(experiments): U2 reaches u-rev 4 — the exceptions list that went stale twice is REPLACED by a rule and a derived grep, because a hand-enumerated set inside a document that keeps being repaired is the generator D-331 names
63eac4c fix(tools): R19 — baseline_snapshot.sh resolved --out against the caller and --corpus/--binary against the repository root, one flag apart, so it now has ONE base, states it in its own usage text, and refuses the two readings' disagreement by name
```

**THE SUBJECT ITSELF HAS NOT MOVED, so every finding below stands at HEAD:**

```
$ git diff 4d84694 HEAD -- docs/experiments/matrix_U4R_restructure.md docs/experiments/matrix_U4R_measurements.md docs/experiments/U4_soundness_instrument.md | wc -l
0
```

Mid-attack the working tree also carried another session's uncommitted
`U2_node_protocol.md`; it has since landed as `7dfd047`. **I edited no repository file
except this report, staged nothing, committed nothing, and ran no git write command.**
Scratch files live only under this session's scratchpad.

**NOTE ON PASTED COMMANDS.** Where a command below reads `$U4` it is the instrument
script's own binding, `U4=docs/experiments/U4_soundness_instrument.md`; set it before
re-running.

**A SIBLING PRECEDENT LANDED WHILE THIS ATTACK RAN, AND IT BEARS ON THE FIELD.**
`7dfd047` is U2's answer to the same law under which this matrix was authored: *"the
exceptions list that went stale twice is REPLACED by a rule and a derived grep, because
a hand-enumerated set inside a document that keeps being repaired is the generator
D-331 names."* U2 did not un-fold a quotation; it replaced a hand-maintained
enumeration with a derived one. `7473a6f` is U3's, and it removed a completeness claim
rather than correcting it. **Both landed remedies act on hand-maintained status and
completeness matter — the clause-(3) surface (b) does not touch — and neither cuts
record or quotation.** This is a third and fourth data point for K3 and M6 and it is
this project's own, taken after the matrix was authored.

**GROUND:** every command below was run by me, in the live repository, at the states
recorded above. Every output block is my own, pasted complete.

**SCOPE, AS DISPATCHED:** obligations A–H. The u-rev-7 review's findings are not
re-litigated; whether U4 needs a restructure at all is in scope only through the null
row.

---

# 1. PER-FACT REPRODUCTION VERDICT (D-330's third part)

| fact | verdict |
|---|---|
| **F1** | **REPRODUCES** (every number) — **but its own headline sentence is falsified by the map it pastes.** See **F7-MAJ** |
| **F2** | **REPRODUCES WITH A DIFFERENT MEANING.** Every number is exact; the label *"matrix-derived fold blocks"* is false of **382 of the 638 lines**. See **K1** |
| **F3** | **REPRODUCES WITH A DIFFERENT NUMBER** — `docs/decisions.md` is **707**, not 705; total **1828**, not 1826. See **m1** |
| **F4** | **REPRODUCES WITH DIFFERENT NUMBERS IN 3 OF 12 ROWS** — `D-323` is **13**, `S-M` is **13**, `D-316` is **9**. Two of the three are Ground 1's headline. See **M2** |
| **F5** | **REPRODUCES** — 160 / 31 / 34 / 79 / span 52 / span 32, all exact |
| **F6 (line numbers)** | **REPRODUCES** — all 22, exactly |
| **F6 (the disclosed discrepancy)** | **DOES NOT REPRODUCE — THE QUOTED SUMMARY LINE DOES NOT EXIST.** See **M1** |
| **F6 (the "nine sections" enumeration)** | **DOES NOT REPRODUCE** — four of the nine named sections carry zero hits. See **M3** |
| **F6 (site table)** | **7 of 8 rows REPRODUCE**; the MINOR 6 row mis-attributes 182. See **m3** |
| **F7** | **REPRODUCES** — 91/0 and 54/4 exactly; `ls tools/ \| wc -l` = 18. **The word "most recent" is stale at HEAD.** See **m10** |
| **F8** | **REPRODUCES WITH ELIDED OUTPUT** — the same command returns **six** lines; the matrix pastes **three**, under a header claiming complete output. See **m2** |

## The instrument file's own commands, re-run

| command | verdict |
|---|---|
| M0 / the LOUD NOTE | **REPRODUCES** — `git diff --stat 871e678 c9d4e64` is exactly `docs/decisions.md \| 2 ++`; 705 → 707 confirmed |
| M1, M2, M3, M4, M5 | **REPRODUCE**, every figure |
| M6 (raw output) | **REPRODUCES** — occurrence counts and section lists identical. *The matrix's transcription of it does not; see M2* |
| M7 | **REPRODUCES at `871e678`**; 705 → **707** at HEAD, as the instrument itself disclosed |
| M8 | **REPRODUCES** — numstat exact, `ls tools/` = 18, 18 files matching `docs/` |
| M9 | **REPRODUCES** — all six lines |
| M10 | **REPRODUCES** — **22 matching lines, 23 total occurrences** |
| M11 | **DOES NOT REPRODUCE AT THE LIVE TREE** — `U2_node_protocol.md` is 858 lines and **u-rev 4**, not 827 / u-rev 3. It reproduces at `871e678`; the working tree carries another session's edit. **No matrix cell cites M11**, so nothing turns on it |
| M12 | **REPRODUCES** |
| follow-ups (a), (b), (c) | **REPRODUCE** — the span map, the three-file blockquote counts, and 79 from both forms |

---

# 2. THE DISCLOSED DISCREPANCY, RESOLVED — AND IT IS WORSE THAN DISCLOSED

**The claim (matrix:185–196):**

> **NO COUNT IS ASSERTED FOR THIS CELL, AND THE REASON IS A DISCREPANCY THE AUTHOR
> FOUND IN THE MEASURER'S OWN REPORT.** The measurer's closing SUMMARY line for M10
> reads *"20 live-line occurrences"*; the complete output it pasted immediately above
> that summary carries **more rows than that**.

**THERE IS NO SUCH SUMMARY LINE. THERE NEVER WAS ONE, AT ANY REVISION.**

```
$ grep -n "SUMMARY\|summary\|20 live\|live-line" docs/experiments/matrix_U4R_measurements.md
$ echo "exit=$?"
exit=1
$ grep -rn "20 live" . 2>/dev/null | grep -v "^./.git/"
docs/experiments/matrix_U4R_restructure.md:187:reads *"20 live-line occurrences"*; the complete output it pasted immediately above
$ git log --all --oneline -S"20 live-line"
4d84694 docs(experiments): MICRO-MATRIX U4-R is authored under the R11 split ...
$ git log --oneline -- docs/experiments/matrix_U4R_measurements.md
4d84694 docs(experiments): MICRO-MATRIX U4-R is authored under the R11 split ...
```

The string exists in exactly one place in this repository — inside the matrix, as a
quotation attributed to the instrument. The instrument file has one revision, landed
in the same commit, and it contains no summary line for M10 whatever. Its **only**
summary sentence in the whole 562-line file is `Both forms agree: 79.`, and that one
is true.

**THE TRUE NUMBER.**

```
$ grep -c "differential gate" docs/experiments/U4_soundness_instrument.md
22
$ grep -o "differential gate" docs/experiments/U4_soundness_instrument.md | wc -l
23
```

**22 matching lines, 23 occurrences.** The matrix should have cited **22**. There was
no competing number to choose between: the paste is 22 rows and the instrument
asserts nothing else.

**Why this breaks, and it is the sharpest single fact in this report.** The matrix
registers this cell as *"the first thing for the red team to re-run"* under D-330's
third part, and frames it as evidence of intellectual honesty — *"disclosed here
rather than smoothed, because a summary that does not match its own paste is D-328's
exact class appearing inside the very split that line prescribes."* Re-run, the
disclosure is the defect. The author manufactured a mismatch inside its own
instrument, attributed a false quotation to a stakeless measurer, and used the
manufactured mismatch as the ground for withholding a number that its own paste
states unambiguously. **This is D-331's disease — a restatement of a claim whose home
is another file, false at the moment it lands — occurring in the document whose
governing law is D-331, in the paragraph that announces the author's fidelity to
D-330.** Severity: **MAJOR**, and evidence for obligation H that the CLAIM-HOME law
is not followable by care alone (it was not followed by the session announcing it as
its governing law).

## Every other summary line in the instrument, checked against its paste

Per the dispatch, I checked all of them. There are three prose assertions in the
instrument outside code fences:

| assertion | verdict |
|---|---|
| `Both forms agree: 79.` | **TRUE** — both forms return 79 (re-run below). **But see m9: it is a vacuous agreement** |
| LOUD NOTE: `git diff --stat 871e678 c9d4e64` touches one file, `docs/decisions.md \| 2 ++` | **TRUE** |
| LOUD NOTE: 705 at `871e678`, 707 at `c9d4e64`, D-331 added 2 lines | **TRUE** |

```
$ git diff --stat 871e678 c9d4e64
 docs/decisions.md | 2 ++
 1 file changed, 2 insertions(+)
$ git show 871e678:docs/decisions.md | wc -l ; git show c9d4e64:docs/decisions.md | wc -l
705
707
$ grep -cE "D-3[0-9]+.*(RECORD|record|REPAIRED|CLOSED|OPEN|SELECTED)|(RECORD|record|REPAIRED|CLOSED|OPEN|SELECTED).*D-3[0-9]+" docs/experiments/U4_soundness_instrument.md
79
$ grep -E "D-3" docs/experiments/U4_soundness_instrument.md | grep -cE "RECORD|record|REPAIRED|CLOSED|OPEN|SELECTED"
79
```

**The measurer's report is clean.** Every number in it reproduces; its one summary is
true; its revision drift is disclosed accurately and completely. The R11 split worked.
**What failed is the author's transcription of it** — which is exactly D-330's finding
that the split is necessary and not sufficient, appearing again one round later.

---

# 3. KILLS

## K1 (KILL) — 382 of the 638 lines (b) proposes to cut are NOT matrix restatement. They are U4's own record, they have no other home, and D-331 protects them by name.

**The claim (matrix, F2 header):**

> **F2. The matrix-derived fold blocks at the heads of §8 and §9 are 638 blockquote
> lines, and they are 88 % of every blockquoted line in the unit.**

**and (option (b)'s cut cell):**

> the 638 blockquote lines of F2 are replaced by … a citation to the ADR line and the
> selection record that decided it. What the unit RESTATES about the selections goes;
> what the unit USES them for stays.

**The contradicting evidence.** The awk the matrix inherits measures *"heading `## 8.`
to heading `### 8.1`"*. That span is not one block. It is **two** blocks, and only the
first is matrix-derived:

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

**So the 638 decomposes as 135 + 121 = 256 MATRIX-DERIVED lines and 115 + 267 = 382
lines of `THE RECORD OF THE EARLIER STATES`.** Sixty per cent of the mass the option
is priced against is the unit's record of its own superseded states — the u-rev-1
SELECTION-OPEN stub and its five-row table, the u-rev-7 two-sided RECORD / NOT-RECORD
stamp, and §9's T1′ recovery with both DIFFs.

**And that record has no other home.** F3's four files do not contain it:

```
$ for pat in "5e8c5e4a1e7ad416" "DIFF 2 — recovered" "MECHANICAL RECOVERY"; do echo "--- $pat ---"; grep -rln -- "$pat" docs/; done
--- 5e8c5e4a1e7ad416 ---
docs/experiments/U4_soundness_instrument.md
--- DIFF 2 — recovered ---
docs/experiments/U4_soundness_instrument.md
--- MECHANICAL RECOVERY ---
docs/experiments/U4_soundness_instrument.md
```

The T1′ recovery — the `ec8f7fb:558–578` recovery, its sha256 prefix, DIFF 1's
"identical by construction and therefore not relied on" reasoning, DIFF 2's
externally-derived referent with **four MEASURED falsifications of matrix cells**, and
the verdict `DIFFERS. SELECTION OPEN` — exists in exactly one place in this tree, and
that place is inside the span (b) deletes.

**Why it breaks.** Ground 2 is *"the un-fold destroys no record, because the record is
not in U4 and never was."* For 382 of the 638 lines that sentence is false: the record
**is** in U4, it is **only** in U4, and it is the unit's own. So (b) has two horns and
both are fatal:

* **Horn 1 — it deletes them.** Ground 2 fails outright, and the WP loses the only
  copy of the recovery on which §9's whole "the carve does not select N-A" history
  rests.
* **Horn 2 — it reproduces them.** Then **the third flip clause fires by measurement**:
  *"If the red team measures that the un-fold's replacement text reproduces more than
  half the lines it removes, the option is not doing what it claims and the field is
  re-opened with a fourth option authored."* 382/638 = **59.9 %**, more than half.

**And the governing law forecloses horn 1 independently.** D-331 (`docs/decisions.md:707`):

> **WHAT IT DOES NOT DO:** … it does not disturb the RECORD discipline, under which a
> text an architect selected from is carried unedited and is a record of what was
> argued rather than a claim about what is true now

Those 382 lines are exactly that. **Severity: KILL against (b) as scoped.**

## K2 (KILL) — Ground 1 reads F4 backwards. Sixty-eight per cent of the registered duplication surface is OUTSIDE the fold spans, and D-320 is one hundred per cent outside.

**The claim (Ground 1):**

> F4 measures what D-331 calls the disease: `D-323` and `S-M` each stated in **twelve**
> distinct sections, `D-329` in eight, `N-E` in seven. **F2 measures where the bulk of
> it lives: 638 lines, 88 % of the unit's blockquoted matter, in two blocks whose whole
> content is a second telling of decisions that landed elsewhere.**

**The contradicting evidence — I intersected F4's own registered token list with F2's
own fold spans:**

```
$ printf '%-22s %7s %10s %10s %8s\n' token total in_folds outside_folds pct_in
$ for t in D-323 S-M D-329 D-316 N-E DEPENDS-OPEN-THEORY 7e0a328 af8082a D-320 "four conditions" "SELECTED AND NOT BUILT"; do
    tot=$(grep -Fc -- "$t" $U4); inf=$(awk 'NR>=205&&NR<=459 || NR>=865&&NR<=1257' $U4 | grep -Fc -- "$t")
    printf '%-22s %7s %10s %10s %7s%%\n' "$t" "$tot" "$inf" "$((tot-inf))" "$((inf*100/tot))"; done
token                    total   in_folds outside_folds   pct_in
D-323                       46          9         37      19%
S-M                         54         18         36      33%
D-329                       57         20         37      35%
D-316                       27          4         23      14%
N-E                         64         38         26      59%
DEPENDS-OPEN-THEORY         12          2         10      16%
7e0a328                     12          4          8      33%
af8082a                      9          3          6      33%
D-320                       20          0         20       0%
four conditions              8          2          6      25%
SELECTED AND NOT BUILT       5          1          4      20%
```

**101 of 314 occurrence-lines (32 %) are inside the two fold spans; 213 (68 %) are
outside.** For Ground 1's two headline tokens the numbers are **19 %** and **33 %**.
`D-320` — twenty occurrences — is **entirely** outside.

**Why it breaks.** Ground 1 joins two measurements with the word "where": F4 says the
duplication is wide, F2 says the blockquote mass is concentrated, and the ground
asserts these are the same place. They are not, and the matrix never intersected them.
The folds are where the **quotation** lives, not where the **restatement** lives.
D-331 is explicit that these are opposite things:

> it does not forbid QUOTATION — a marked verbatim quotation carrying its source and
> revision is a pointer that brings its referent with it, and **is the preferred
> instrument where the words themselves are load-bearing**

and

> **it does not make a pointer a finding** — a reviewer may charge a restatement only
> where the text carries a claim, and may not charge a section reference, an ADR
> reference or a file reference as one

F4 counts token occurrences. Under D-331 most of them are lawful pointers, and the
matrix's own table proves it: the row that corresponds to a real defect, `eight of
twelve`, has count **1**. **F4 is anti-correlated with the defect class it is offered
as a measure of.** Severity: **KILL against Ground 1.**

## K3 (KILL) — the matrix's governing law binds the status matter (b) does not touch and protects the record matter (b) cuts.

**D-331's operative clauses, in order:**

> (1) every claim has exactly ONE HOME … (2) every other occurrence is a POINTER …
> (3) **STATUS AND SUMMARY MATTER CARRIES POINTERS ONLY** — REVIEW STATUS tables,
> change logs, lineage tables and OPEN lists may say that a finding exists, that it is
> repaired and where the repair lives, and may not restate the finding's content or
> the repair's content

Clause (3) names four artefacts. U4 has all four: the REVIEW STATUS table (F5: span
52, lines 115–166), U4-A's lineage table (F5: span 32), the 34 REPAIRED / WITHDRAWN /
RE-CHECKED markers, and U4-Z's OPEN list. **Option (a) reaches one of them. Option (c)
reaches three. Option (b) — the recommendation — adds nothing to (a) on this axis at
all**, because the un-fold's scope is §8/§9's heads, which contain none of them.

Meanwhile the "WHAT IT DOES NOT DO" clause protects the 382 record lines (K1) and
names marked verbatim quotation as *the preferred instrument*, which is what the
remaining 256 are.

**Why it breaks.** A matrix that declares D-331 its governing law and then recommends
the one option in its field that leaves every clause-(3) artefact untouched while
cutting the matter the law's exclusions protect has inverted its own law. Severity:
**KILL against the recommendation.**

## K4 (KILL) — the field is ranked by SIZE, and this work package has already MEASURED that premise false. It is false again here, on this unit, by this review's own findings.

**The precedent (`docs/experiments/restructure_selection_15b.md:100–105`):**

> - **F2 (KILL)** the founding premise "correlates with document size, not content"
>   is false against the document's own history. MEASURED … **0.86 per 100 lines** …
>   **7.35 per 100 lines**. 8.5× the rate at one-eighth the growth. **The moved
>   variable is repairs, not lines.**

**The repetition.** Ground 1 selects by mass ("*where the bulk of it lives*: 638
lines, 88 %, 33.8 % of the unit"). Ground 3 rejects (a) by mass-adjacent reasoning.
Every cost cell is a line count. So I measured defect density by region, taking each
finding's site from `wp15b_U4_REVIEW_urev7.md` and each region's span from F1:

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

**The two head folds are 34 % of the unit and carry the LOWEST defect density of any
region measured: 0.31 findings per 100 lines and ZERO MAJORS.** §8.7 — fifty-one lines
— carries **12.6× the fold density** and one of the four MAJORs. U4-Z carries three.

**Why it breaks.** The matrix treats size as the cause. Measured, on this unit, at
this u-rev, by this review's own eight findings, size is anti-correlated with defects
in exactly the region the recommendation targets. This is the same premise
`restructure_selection_15b.md` F2 killed with a KILL, in the same work package, on
the same document lineage. Severity: **KILL against the field's ranking.**

## K5 (KILL, by domination) — a row nobody authored satisfies all three grounds, cuts comparable mass, deletes nothing, and owes no judgement call.

**Row (e) — INDEX + EXTRACT, not INDEX + DELETE.**

> U4-Z becomes the pointer index of (a). **In addition, the two
> `THE RECORD OF THE EARLIER STATES` blocks — U4:343–459 and U4:989–1257, 382
> blockquote lines, MEASURED — are MOVED to a companion record file** (or appended to
> the existing selection records) and replaced in U4 by one citation each. **The two
> `SELECTED` folds (207–342 and 867–988, 256 lines) stay**, because they are what the
> unit is bound by and D-331 calls marked quotation the preferred instrument.

Against (b)'s own three grounds:

* **Ground 1** — (e) removes the same body mass from the unit that (b) claims to, and
  reaches strictly more of F4's registered duplication than (b) does, because (b)'s
  cut is 68 % off-target (K2) and (e)'s extraction is the same 382 lines with zero
  judgement required.
* **Ground 2** — satisfied *better*: extraction relocates, so nothing is destroyed,
  and Ground 2's own worry ("the record is not in U4") is answered by giving the
  record a home instead of asserting it already has one (K1 shows it does not).
* **Ground 3** — identical: (e) contains (a), so it reaches MAJOR 2, 3, 4.

And it **owes less**: no "normative use vs restatement" judgement, which is the
judgement (b)'s own failure-mode cell says can delete a binding condition; no risk to
D-323's five and D-329's four registered conditions; no cost-bracket error.

**Measured mass:** 382 (extraction) + the (a) cut's MEASURED floor inputs
(183 + 104 + 144 = 431 minus the index) — comparable to (b)'s claimed −800 to −1000
without deleting a line.

**Why this is a KILL and not a wound.** The dispatch's own definition: *"A row that
dominates the recommendation — satisfying its grounds identically while owing less —
is a KILL."* (e) satisfies all three grounds at least as well and owes strictly less.
This is the M4 closed-enum-row shape and the M3 four-missing-rows shape, third
occurrence in this work package. Severity: **KILL.**

## K6 (KILL of the (c) row's dismissal) — the matrix killed the GENERATOR and let the option die with it. The hand-maintained row is the one the fired trigger actually names, and it is not in the field.

**The claim (option (c)'s failure mode):**

> **IT DOES NOT REACH THE DEFECT CLASS, AND F4's LAST ROW IS THE PROOF.** … none of
> which any generator can emit … Second, and independently: "non-reviewable" is not
> available to it — `crates/pistol-solver/tests/wp15b_census.rs` reads the carve
> members, and a generated member is a member.

**Both grounds attack only the generation, and the second is false.** Membership in
the census is decided by a marker, not by a directory:

```
$ sed -n '656,659p' crates/pistol-solver/tests/wp15b_census.rs
const CARVE_MARKER: &str =
    "<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->";
$ sed -n '855,860p' crates/pistol-solver/tests/wp15b_census.rs
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
        .filter(|entry| {
            std::fs::read_to_string(entry.path()).is_ok_and(|text| text.contains(CARVE_MARKER))
        })
```

A `U4_STATE.md` that does not carry `CARVE_MARKER` is not on the disk list, is not in
`CARVE_DOCS`, and the pin never opens it. Whether a new file is a carve member is the
author's choice, spelt by one HTML comment.

**And the row the trigger names is missing.** Strip the generator from (c) and you get:

**Row (c′) — the status matter moves to a hand-maintained STATE section or file; no
`tools/` artefact, no generator, no SHELL_CHECKLIST review, no driving test.** Every
one of the matrix's costs against (c) is a cost of generation: F7's 91+54 precedent,
the `tools/` artefact, item 10's coverage rule, the checklist review it does not have.
**(c′) owes none of them.** And it is the only row in any field that reaches the two
findings nothing else touches — see M5.

Severity: **KILL of the (c) row as authored**; the field is incomplete without (c′).

---

# 4. MAJOR FINDINGS

## M1 (MAJOR) — the fabricated instrument quotation. See §2 above.

## M2 (MAJOR) — F4's distinct-section column is wrong in three of twelve rows, and two of the three are Ground 1's headline numbers.

**The claim (F4 table):** `D-323` → **12**, `S-M` → **12**, `D-316` → **8**.

**Re-run, using the instrument's own tagging awk:**

```
$ awk '/^#{1,3} /{sec=$0} {print sec"\t"$0}' $U4 > /tmp/.../u4r_tagged.txt
$ for t in D-323 S-M D-316; do echo "==== $t ===="; grep -F -- "$t" /tmp/.../u4r_tagged.txt | cut -f1 | sort -u | wc -l; done
==== D-323 ====
13
==== S-M ====
13
==== D-316 ====
9
```

The thirteenth section is `# WP-1.5b U4 — the soundness instrument and the snapshot
seam: DESIGN UNIT`, which tags the whole 172-line head — the region containing the
REVIEW STATUS block, i.e. exactly the clause-(3) status matter D-331 governs. The
matrix **included** that section for nine of the twelve tokens (D-329's 8, N-E's 7,
D-320's 4, `7e0a328`'s 6, `af8082a`'s 5, `four conditions`' 4, `SELECTED AND NOT
BUILT`'s 4, `DEPENDS-OPEN-THEORY`'s 9 all require it) and **excluded** it for three.
The counting rule changes inside one table.

**Why it breaks.** Ground 1's sentence is *"`D-323` and `S-M` each stated in **twelve**
distinct sections"*. Both numbers are wrong, and they are wrong in the direction that
drops the head — the one region no option in the field except (c)/(c′) touches. A
MEASURED cell that does not reproduce is a finding against the round under D-330's
third part **even where the conclusion survives**; here three do not reproduce.
Severity: **MAJOR.**

## M3 (MAJOR) — F6's supporting enumeration names four sections that carry zero hits, and omits three that carry ten.

**The claim (matrix:174–177):**

> The spread is confirmed by the registered command M10, whose complete output is in
> the instrument file and **whose hits fall in §8's fold, §8.2, §8.3, §8.4, §8.7, §9,
> U4-M, U4-T and U4-Z — nine sections**

**Re-run, attributing every hit to its section:**

```
$ awk '/^#{1,3} /{sec=$0} /differential gate/{printf "%5d  %s\n", NR, substr(sec,1,60)}' $U4
  138  # WP-1.5b U4 — the soundness instrument and the snapshot sea
  159  # WP-1.5b U4 — the soundness instrument and the snapshot sea
  192  ## U4-A. Lineage — what has attacked this unit's content, an
  399  ## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u
  453  ## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u
  651  ### 8.3 The other three gates, re-scoped — NAMED, NOT LETTER
  747  ### 8.3 The other three gates, re-scoped — NAMED, NOT LETTER
  792  ### 8.4 The mutation ledger, with witnesses
  793  ### 8.4 The mutation ledger, with witnesses
  795  ### 8.4 The mutation ledger, with witnesses
  826  ### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
  835  ### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
  839  ### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
  852  ### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
  857  ### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
 1470  ### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316); ITS RE
 1504  ### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316); ITS RE
 1508  ### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316); ITS RE
 1512  ### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316); ITS RE
 1678  ### ADR lines
 1708  ### ADR lines
 1820  ### OPEN — carried forward, not closed by the carve
```

**§8.2 has zero hits. §9 has zero hits. U4-M has zero hits. U4-T has zero hits.** The
head has three, U4-A has one, and U4-Z's three sub-sections have six. The count nine
is right by coincidence; the list is fabricated.

**Why it breaks.** M10 is the matrix's *"registered command"* for the spread claim,
and the spread claim is what the matrix substitutes for the number it declines to
state. One of the four sections falsely named is **§9** — one of the two sections
(b)'s un-fold targets. A reader checking whether the un-fold reaches the spread is
told §9 carries the phrase; it does not carry it once. Severity: **MAJOR.**

## M4 (MAJOR) — the un-fold half of (b) reaches ZERO of the four MAJORs and ZERO instances of the class D-331 exists for.

Taking every finding's site from `wp15b_U4_REVIEW_urev7.md` against F1's spans and
F2's fold spans:

| finding | site | region | reached by (a) | reached by the UN-FOLD |
|---|---|---|---|---|
| **MAJOR 1** | 833–837 + status row 138 | §8.7 blockquote + head | no | **no** |
| **MAJOR 2** | 1781–1785 | U4-Z OPEN | yes | no |
| **MAJOR 3** | 1748 ff | U4-Z OPEN | yes | no |
| **MAJOR 4** | 1738–1741 | U4-Z ADR lines | yes | no |
| **MINOR 5** | 1386–1389 | U4-M | no | **no** |
| **MINOR 6** | 854 + universal at 142 | §8.7 + head | no | **no** |
| **MINOR 7** | 937–939 | §9 SELECTED fold (867–988) | no | yes |
| **MINOR 8** | 957–964 | §9 SELECTED fold (867–988) | no | yes |

**(a) reaches 3 of 8, all MAJOR. (b) reaches 5 of 8, and the two the un-fold adds are
the two least severe findings in the set.** Note further that both hits are in the
*SELECTED* fold, not in the RECORD blocks that carry 60 % of the mass — so the mass
and the hits are in different halves of the same span.

**Neither hit is an instance of the disease.** MINOR 7 is a missing MEASURED /
ESTIMATED mark (the D-291 class). MINOR 8 is an unmarked elision in a verbatim
quotation — a finding whose remedy is *to quote more faithfully*, which D-331 calls
the preferred instrument. **Zero of the two are the stale-restatement class D-331 was
landed for, and the four that are — MAJOR 2, 3, 4 and the head half of MAJOR 1 — are
all outside the un-fold.**

**Why it breaks.** Ground 1's whole force is *"it is the only option in the field that
reaches the generator."* Measured against the eight findings that produced this
matrix, the un-fold half reaches no MAJOR and no instance of the class. Severity:
**MAJOR**, and combined with K2 and K4 it is what carries (b) to FALLS.

## M5 (MAJOR) — Ground 3's criterion is applied to (a) and not to (b), and it condemns (b) at the one finding the matrix itself calls the hardest.

**The claim (option (a)'s failure mode):**

> **IT REACHES THREE OF THE EIGHT FINDINGS AND F6 MEASURES WHICH FIVE IT MISSES.**
> MAJOR 1, MINOR 5, 6, 7 and 8 are outside U4-Z and survive this cut untouched —
> **including MAJOR 1, the semantic survival at §8.7 that the dispatch names as the
> hardest of the eight.**

**and (Ground 3):**

> **F6 rules out (a) on the field's own terms.** Five of eight findings sit outside
> U4-Z. An option whose scope is U4-Z is scoped to 3/8 of the evidence that produced it.

**The contradicting evidence.** MAJOR 1's two sites are U4:833–837 (inside §8.7, a
blockquote that is not either head fold) and U4:138 (inside the head). Neither is in
205–459, 865–1257, or 1441–1886:

```
$ sed -n '833p;837p;138p' $U4 | cut -c1-90
> **REPAIRED by shape 2 (D-316): the letters are dropped and the four gates are
> U4-Z, with the selection recorded beneath it.
| **MAJOR 3** — U4-Z's u-rev 2 SELECTION block still asserts "S-E **is** the diff
```
```
$ awk 'NR==814,NR==864' $U4 | head -1 | cut -c1-60      # S8.7 spans 814-864 per F1
### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
```

**MAJOR 1 is reached by NO option in this field.** So is MINOR 5. So is MINOR 6.

**Why it breaks.** The matrix charges (a) with missing MAJOR 1 in the row that kills
(a), and does not charge (b) with missing the same MAJOR 1 in the row it recommends.
Ground 3's arithmetic, applied evenly, reads: (a) 3/8, **(b) 5/8**, (c) ~1.5/8 — and
the *hardest of the eight* is 0/4 across the whole field. A criterion that is applied
to one row and withheld from another is not a criterion; it is the selection wearing
one. Severity: **MAJOR.**

## M6 (MAJOR) — the trigger that kills the null row is about STATUS MATTER. The selected option touches no status matter. The trigger licenses (c)/(c′) and is spent on (b).

**The trigger, verbatim from the ground (`wp15b_U4_REVIEW_urev7.md:40`):**

> Per the dispatching session's standing rule, **the trigger for an architect
> restructure of U4's status matter is met**; another patch-and-re-review round is the
> wrong instrument

**The matrix's (n) row quotes it accurately**, then the recommendation goes elsewhere.
Under (b), all of the following remain exactly as they are: the 52-line REVIEW STATUS
block, U4-A's 32-line lineage table, and the 34 REPAIRED / WITHDRAWN / RE-CHECKED /
MARKED-AT lines (F5, all MEASURED and all reproducing). **The option that answers the
trigger is (c′) — hand-maintained status extraction — and it is not in the field
(K6).**

**Why it breaks.** The null row is the field's only defence against a change made
because someone wanted to make one. It is killed here by a trigger whose scope the
recommendation does not enter. Severity: **MAJOR**, and see M8 — the standing rule
that fires the trigger belongs to the same dispatching session that marked (b)
RECOMMENDED.

## M7 (MAJOR) — F1's headline sentence is falsified by the span map printed beneath it.

**The claim:** *"F1. The unit is 1886 lines, and its two largest sections are the two
matrix folds."*

**The map it pastes, re-run and identical:**

```
205	## 8. MATRIX M3 …	span=660
865	## 9. MATRIX M4 …	span=482
1441	## U4-Z. …	span=446
```

The two largest **sections** are §8 (660) and §9 (482), and neither is a fold; the
largest single block any option targets is **U4-Z at 446**, which is larger than the
§9 fold (393) and 1.75× the §8 fold (255). Ranked as folds, §8's is the *third*
largest targetable block in the unit.

**Why it breaks.** F1 is the fact that establishes "the folds are the bulk", which
Ground 1 then converts into "the folds are the generator". The headline conflates a
section with a block inside it, and the conflation runs in the direction of the
recommendation. Severity: **MAJOR.**

## M8 (MAJOR) — the disclosure of provenance is honest and does not discharge the breach.

**What the matrix discloses:** that (b) was RECOMMENDED in the architect's dispatch
before the author read the measurements, and that F6 contradicts the dispatch's
internal ordering of (b)'s two halves.

**What it does not do.** The field is the dispatch's field: (n), (a), (b), (c) with
(b) recommended. **No row the dispatch did not name was authored — including the one
this matrix's own third flip clause names** (*"extraction of §8/§9's fold to their own
record files, which no row here states"*). All three grounds were constructed after
the recommendation was fixed. The single place the author says the measurements tell
against the dispatch is about **ordering**, and the matrix then declines to act on it
(*"This matrix does not split the option on that ground"*). Meanwhile the measurements
tell against the dispatch on a far larger point the author did not draw — that the
un-fold reaches zero MAJORs (M4), that 68 % of the duplication is outside the folds
(K2), and that the fold region has the lowest defect density in the unit (K4).

**And the null row is killed by the same party's standing rule** (M6): the dispatching
session's rule, quoted through a reviewer, removes the only option that would have
required the change to justify itself.

**Verdict on obligation C.** The disclosure is real and it is the right thing to have
written. It is not sufficient. On the dimension that matters — **whether the field was
authored independently of the recommendation** — the answer is no, and D-305's
measured base rate (*"the red team supplied the surviving option in four of six
matrices in this WP"*, carried in U4:394) is the reason that dimension is the one that
matters. Severity: **MAJOR — an unattacked selection wearing a matrix on the field
dimension**, cured only by authoring (e) and (c′) and re-attacking.

---

# 5. MINOR FINDINGS AND WOUNDS

**m1 (MINOR).** F3 does not reproduce at the matrix's own revision:

```
$ wc -l docs/experiments/matrix_M3_selection.md docs/experiments/matrix_M4_axisA_selection.md docs/experiments/matrix_M4_axisA_round4.md docs/experiments/matrix_M3_soundness_instrument_rev2.md docs/decisions.md
   216 …matrix_M3_selection.md
   149 …matrix_M4_axisA_selection.md
   305 …matrix_M4_axisA_round4.md
   451 …matrix_M3_soundness_instrument_rev2.md
   707 docs/decisions.md
  1828 total
```

705 → **707**, 1826 → **1828**. The instrument disclosed this drift in a LOUD NOTE and
named the exact cell; **the matrix pasted the stale output and dropped the
disclosure**, in a document whose own header states that D-331 *"landed at `c9d4e64`
before this matrix was authored"* — i.e. the author knew the commit that made its
paste stale, cited it, and re-published the pre-drift number as R7 inline evidence.
This is the CLAIM-HOME disease in miniature and it is the second instance inside this
matrix (with M1). **Obligation H is answered: yes, the matrix restates claims whose
home is elsewhere, and both restatements went wrong.**

**m2 (MINOR).** F8's paste is incomplete against the header's *"complete output inline
(R7), including the zeros"*:

```
$ for f in docs/experiments/wp15b_U4_REVIEW.md docs/experiments/wp15b_U4_REVIEW_urev6.md docs/experiments/wp15b_U4_REVIEW_urev7.md; do echo "---- $f"; grep -n "BLOCKING," "$f" | head -4; done
---- docs/experiments/wp15b_U4_REVIEW.md
32:**3 BLOCKING, 3 MAJOR, 5 MINOR.**
---- docs/experiments/wp15b_U4_REVIEW_urev6.md
7:re-review FAIL is COLLECTED AND REPORTED, not looped on in-session: the BLOCKING,
39:**1 BLOCKING, 2 MAJOR, 4 MINOR.**
---- docs/experiments/wp15b_U4_REVIEW_urev7.md
24:**0 BLOCKING, 4 MAJOR, 4 MINOR.**
408:| Z44 | Both shipped-instrument defects CLOSED at `b067d47` and `a102c6a`; REVIEW-impl PASSED at `84ff8d7`, 0/0/3, on mutation-checked controls | **HOLDS** — `wp15b_trackC_REVIEW_impl.md:709–711` reads **VERDICT: PASS**, *"0 BLOCKING, 0 MAJOR, 3 MINOR (F1, F2, F3)"* |
473:**VERDICT: FAIL — 0 BLOCKING, 4 MAJOR, 4 MINOR.**
```

Six lines; the matrix prints three. The instrument (M9) printed all six. The elision
is toward the thesis — it removes the two lines showing a PASS in the same lineage.
Nothing turns on it, but R7 is a rule about pasting, not about relevance.

**m3 (MINOR).** F6's MINOR 6 row reads *"854 (the universal at 142/182)"*. The
universal is at **142**; **182** is `U4-A`'s round-2 lineage row, which the review
lists among the four sites *repaired*, not among the universals
(`wp15b_U4_REVIEW_urev7.md:301`). One of two cited line numbers is mis-attributed.

**m4 (WOUND, flip-clause coherence).** Flip clause 1's remedy is *"(b)'s un-fold half
is dropped and **(a) is selected**"* — an option Ground 3 declares *"ruled out on the
field's own terms"*. The trigger and the remedy are about the same thing (the
un-fold's separability), so this is **not** the F5 class the matrix guards against;
it is a different incoherence: **the remedy names an option the matrix has already
condemned.** If clause 1 fires, the matrix has no selectable row.

**m5 (WOUND, flip-clause coherence).** Flip clause 4 — *"(c) revives only if a
generator is shown to emit a claim of the kind that failed"* — states a **revival**
condition and no consequence for the selected option. A clause that cannot change what
was selected is not a flip clause. Clauses 2 and 3 are coherent and clause 3 **fires**
(K1, horn 2).

**m6 (MINOR).** Flip clause 2 fires. U4-Z's OPEN list carries at least one item whose
content lives nowhere else: *"ITEM 15's BLOCKAGE IS UNRECONCILED BETWEEN A LANDED ADR
AND A LANDED REVIEW"* (U4:1855–1866). The reconciliation analysis — *"the blockage's
ground narrows from 'no seam is selected' to 'the seam is selected and not built', and
the question MAJOR 4 raises … is untouched by which option was chosen"* — is U4's own
determination; `grep -rln "UNRECONCILED\|unreconciled" docs/` returns only U4 and the
two review reports, neither of which states it. Under an index that *"states nothing
else"*, this is deleted. The remedy is scoped and does not unseat (a), as the clause
says — but it is a fired trigger and should be recorded as one.

**m7 (MINOR).** Option (c)'s absolute claim is false: *"A generator would faithfully
regenerate a STATE file while **every one of the eight findings stayed exactly where
it is**."* MINOR 6's finding is *"the universal, not the citation"*
(`wp15b_U4_REVIEW_urev7.md:302`) and lives at U4:142, inside the REVIEW STATUS table
(c) moves; MAJOR 1's fix scope names *"the REVIEW STATUS row's universal at U4:138
either verified or narrowed"* as half the repair, also inside it. A generator cannot
emit a hand-written completeness universal — which is why moving the table **removes**
both, rather than regenerating them. (c) reaches roughly 1.5 of 8, and it is the only
row in the field that reaches any part of the finding the matrix calls the hardest.

**m8 (MINOR).** The (n) row's cost cell reads *"**MEASURED zero restructure lines**"*.
Zero restructure lines under the null option is a definition, not a measurement; no
command produces it, and R7 requires a MEASURED cell to carry one. The row's only
genuinely MEASURED content is F8's recurrence, which is about **re-review rounds**.
The trigger is real and correctly quoted, and it does refuse *"another
patch-and-re-review round"* — so (n) is correctly not selectable **as stated** — but
"dead" is licensed only for the scope the trigger names (M6).

**m9 (MINOR, against the instrument).** The instrument's one summary, *"Both forms
agree: 79."*, is offered as corroboration and is a vacuous agreement. Both forms are
greps over the same file with the same status-word list; the defect that would make 79
the wrong number is a wrong word list or a wrong file, and both forms are blind to
both. CLAUDE.md's own clause: *"two instruments blind to the same stage are one
instrument reported twice, and their agreement is invariant under a defect in what
they are both blind to."* Nothing rests on 79 — F5 pastes it and states no claim from
it — so this is a wound on the instrument's presentation, not on a cell.

**m10 (MINOR, HEAD drift).** F7's framing — *"the coverage rule's **most recent** price
for one is MEASURED"* — is stale as of `63eac4c`, landed during this attack:

```
$ git show 63eac4c --numstat -- crates/pistol-cli/tests/baseline_snapshot_tests.rs tools/baseline_snapshot.sh
163	0	crates/pistol-cli/tests/baseline_snapshot_tests.rs
95	17	tools/baseline_snapshot.sh
```

The most recent price is **163 test lines and 95/17 script lines**, not 91 and 54.
This moves (c)'s cost against it, so it does not change the verdict — but it is the
third number in this matrix to go stale within days of authoring, and it is direct
evidence for the point at obligation G: **a matrix whose every cell is a line count is
a matrix whose every cell decays.**

**m11 (WOUND, obligation G — answered).** The matrix concedes that *"reviewable in one
sitting"* has no instrument and that *"every line count above is a proxy"*. It is worse
than a proxy: it is a proxy that K4 measures to be **anti-correlated** with the defect
rate on this very unit, and the options are ranked by it in every cost cell
(−300/−380 vs −800/−1000). **Yes: the field is ranked by a proxy for an undefined
property, and the proxy has been measured pointing the wrong way.** The honest cost
column for this field is *findings reached*, which the matrix has the data for (F6)
and never tabulates — M4 does it above.

---

# 6. PER-OPTION SURVIVAL VERDICT

| option | verdict | reason |
|---|---|---|
| **(n) NULL** | **SURVIVES WOUNDED** | The trigger is real and quoted accurately, and it does refuse *"another patch-and-re-review round"*, so (n) as written is correctly not selectable. But the trigger's scope is **U4's status matter** (M6), its cost cell's MEASURED mark is a definition (m8), and the standing rule that fires it belongs to the party that recommended (b) (M8). It survives as the row that says: the trigger licenses a status-matter restructure and nothing wider. |
| **(a)** | **SURVIVES WOUNDED** | It reaches 3 of 4 MAJORs, in the region measured at the **highest MAJOR density in the unit** (0.67/100 lines against the folds' 0.00) — the opposite of the matrix's characterisation of it. Wounds: flip clause 2 has fired (m6), so the index cannot be total; it reaches nothing in the head; and its −300/−380 bracket is a proxy (m11). Ground 3 does not rule it out once applied evenly (M5). |
| **(b)** | **FALLS** | Sixty per cent of its cut is D-331-protected record matter with no other home, and the option's two horns are "destroy the record" or "fire your own flip clause 3" (**K1**). Ground 1 reads F4 backwards — 68 % of the registered duplication is outside the fold spans (**K2**). Its cut leaves every clause-(3) artefact its governing law binds and removes the quotation that law calls the preferred instrument (**K3**). Its ranking repeats the size premise this WP already killed with a KILL, and the folds measure **zero MAJORs at the lowest density in the unit** (**K4**). Its un-fold half reaches no MAJOR and no instance of the class (**M4**). An unauthored row satisfies its grounds while deleting nothing (**K5**). Its cost bracket is out by roughly 1.7× once the un-foldable mass is 256 rather than 638. |
| **(c)** | **FALLS AS AUTHORED; the field is incomplete without (c′)** | Its first ground is sound — the three MAJOR stale claims are prose no generator emits, and `eight of twelve` occurs once. But its second ground is false (**K6**: census membership is `CARVE_MARKER`, not the directory), its absolute reach claim is false (**m7**), and every cost the matrix charges it is a cost of **generation**. Strip the generator and the row that remains — hand-maintained status extraction — is the only row whose scope matches the fired trigger (**M6**) and the only row reaching any part of MAJOR 1. |

**FIELD VERDICT: NO SELECTION MAY BE TAKEN FROM THIS MATRIX.** The recommended row
falls; the field is missing two rows, one of which dominates the recommendation and
one of which is the row the fired trigger actually names; two of its eight facts do
not reproduce as stated and a third is fabricated. Re-author with **(e)** and **(c′)**
in the field, replace the line-count cost column with a findings-reached column the
data already supports, and re-attack.

---

# 7. THE STRONGEST ATTACK SURVIVING AGAINST EACH SURVIVING OPTION

*Written to be quoted verbatim in an ADR line. Assembling one of these from parts is a
residual under D-329.*

**Against (n) NULL:**

> The reviewer's trigger is real and it refuses another patch-and-re-review round, but
> it names U4's STATUS MATTER and nothing wider; a null row killed by a trigger whose
> scope the selected option never enters has been killed by an argument that does not
> reach it, and the party whose standing rule fired the trigger is the party that
> marked the recommendation.

**Against (a):**

> Option (a) is scoped to the region measured to carry every MAJOR the review found in
> the unit's body — U4-Z holds 3 of 4 at 0.67 per 100 lines, against 0.00 in the fold
> blocks the recommendation targets — but its own flip clause has already fired: U4-Z's
> item-15 unreconciled-blockage bullet is content that lives nowhere else in the tree,
> so the index cannot be total, and every line it does not remove is a line the
> matrix's undefined "reviewable in one sitting" cannot say whether it needed to.

**Against (c′), the hand-maintained status extraction the field did not author:**

> Moving U4's status matter out by hand answers D-331 clause (3) and the fired trigger
> exactly, and it is the only cut that reaches the REVIEW STATUS universals behind
> MAJOR 1 and MINOR 6 — but it reaches roughly one and a half of the eight findings,
> it leaves all three of U4-Z's stale-dependent MAJORs where they are, and nothing in
> this project has yet measured that a shorter unit is a better-reviewed one; the one
> time the premise was tested, in this same work package, the defect rate moved with
> repairs and against size.

**Against (e), the extraction row that dominates (b):**

> Extracting the 382 lines of `THE RECORD OF THE EARLIER STATES` destroys nothing and
> needs no restatement-versus-normative-use judgement, but it buys its mass reduction
> from the region measured to contain zero of the eight findings and zero of the four
> MAJORs, so it is a readability change priced in lines and sold as a defect remedy;
> the two MAJOR-bearing regions, U4-Z and §8.7, are reached by (a)'s half and not at
> all respectively, and §8.7 — fifty-one lines carrying 12.6× the fold defect density —
> is untouched by every row in this field.

---

# 8. FINDINGS ATTEMPTED AND REJECTED, WITH THE REPRODUCER

Recorded so a later round does not re-discover them.

- **"F1's span map does not reproduce."** Attempted: re-ran the instrument's follow-up
  (a) awk verbatim. **REJECTED** — all nineteen rows and all nineteen spans are
  identical, including `span=660`, `span=482`, `span=446`.
- **"F5's counts do not reproduce."** Attempted: all six commands. **REJECTED** — 160,
  31, 34, 79, `span=52`, `span=32`, exact.
- **"F7's numstat does not reproduce."** Attempted: `git show b067d47 --numstat …`.
  **REJECTED** — `91 0` and `54 4`, exact; `ls tools/ | wc -l` = 18. Only the word
  *"most recent"* has decayed (m10).
- **"M6's raw output does not reproduce."** Attempted: the whole token loop.
  **REJECTED** — every occurrence count and every section list is identical to the
  instrument's paste. The defect is in the matrix's transcription (M2), not the
  measurement.
- **"The §8 SELECTED fold's content is not landed elsewhere, so ground 2 fails there
  too."** Attempted: `grep -rln` for `zz_m3_phase`, `12.6 %`, `S-K fires 132`.
  **REJECTED** — all three are in `matrix_M3_selection.md` (the probe source at :131,
  its verbatim output at :196–201). Ground 2 **holds** for the 256 matrix-derived
  lines; it fails only for the 382 record lines (K1). *One residue, recorded and not
  charged: U4's u-rev-6 REPLICATION of that probe in a worktree at `46c58ac` — the run,
  not the figures — exists only in U4, and the un-fold would lose the fact that a
  replication was taken.*
- **"M10's twenty-two line numbers are wrong."** Attempted: re-ran the grep.
  **REJECTED** — all 22 reproduce exactly and in order.
- **"The instrument's LOUD NOTE overstates or understates the revision drift."**
  Attempted: `git diff --stat 871e678 c9d4e64`, `wc -l` at both revisions.
  **REJECTED** — every clause of the note is exactly true.
- **"F4's `eight of twelve` row is wrong."** Attempted: `grep -Fc`. **REJECTED** — it
  is 1/1 in one section, exactly as stated, and it is the matrix's own best fact.
- **"Flip clauses 1–4 carry the restructure red team's F5 class (trigger about one
  thing, remedy about another)."** Attempted: read each trigger against each remedy.
  **REJECTED as that class** — all four triggers and remedies concern the same subject.
  Two other defects are charged instead (m4, m5), and clause 3 fires (K1).
- **"The matrix's `87.8 %` and `33.8 %` arithmetic is wrong."** Attempted:
  638/727 = 87.76 %, 638/1886 = 33.83 %. **REJECTED** — both correct. The error is in
  what the 638 is (K1), not in the division.

---

# 9. CLOSING STATE

```
$ git rev-parse HEAD
7473a6fb6c775e0e87cd7dd9821b459be6cc41f5
$ git status --porcelain
?? docs/experiments/matrix_U4R_REDTEAM.md
```

The `??` is this report. **No other repository file was created, edited, staged or
committed, and no git write command was run.** No worktree was created. Scratch files
live only under this session's scratchpad. HEAD moved three commits during the attack;
the subject, its instrument and U4 are byte-identical across all of them.

*Fresh-context DECISION-RED-TEAM of `docs/experiments/matrix_U4R_restructure.md` at
`4d84694`, with its instrument `matrix_U4R_measurements.md` at the same commit.
Eight MEASURED facts and thirteen instrument commands re-run: five facts reproduce,
one reproduces with a different number, two reproduce with different numbers in named
cells, and one disclosed discrepancy does not exist. **Six KILLS, eight MAJOR, eleven
MINOR/WOUND.** The measurer's report is clean; the author's transcription of it is not.
**(b) FALLS. (c) FALLS as authored. (n) and (a) survive wounded. Two rows are missing,
one of which dominates the recommendation. No selection may be taken from this field.***
