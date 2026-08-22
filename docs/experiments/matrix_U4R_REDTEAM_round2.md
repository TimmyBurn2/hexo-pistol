# MICRO-MATRIX U4-R, REVISION 2 — FRESH-CONTEXT DECISION-RED-TEAM, ROUND 2

**SUBJECT REVISION:** `docs/experiments/matrix_U4R_restructure_rev2.md` at **`97aa4d6`**.

## DOES IT STILL MATCH HEAD? **NO — THE TREE MOVED NINE COMMITS UNDER THIS ATTACK, AND THREE OF THEM REPAIRED ALL EIGHT FINDINGS THE MATRIX'S NEW COST COLUMN SCORES. I STATE IT LOUDLY RATHER THAN SILENTLY RE-RUNNING.**

HEAD was `97aa4d6` and the tree was clean when this attack began. At exit:

```
$ git rev-parse HEAD
78b4876bc9abb74534738f34a031703e0ec4dcb9
$ git status --porcelain
?? docs/experiments/matrix_U4R_REDTEAM_round2.md
$ git log --oneline 97aa4d6..HEAD
78b4876 docs(experiments): U4's three stale-dependent MAJORs are repaired at their sites — the SHELL_CHECKLIST set is cited to its one home instead of re-counted, the relative-base residual is re-attributed to the selection record AND recorded CLOSED by R19, and travelling item T2 finally gets its disposition
75ae04e docs(experiments): U4 adopts the landed-SHA citation form and converts the six citations this session's own serial repairs had just made stale, stating the bound rather than the universal because the sibling unit's universal over the same set was false at nine sites
d328d1d docs(experiments): U4's MAJOR 1, MINOR 5, 7 and 8 are repaired at the sites the restructure matrix MEASURED that no option reaches — the semantic survival at 8.7 is corrected by meaning and both false universals in the REVIEW STATUS table are withdrawn without a narrower one replacing them
f0ae14c docs(experiments): U2 reaches u-rev 5 — the "four already existed, one is new" count is deleted from both sites rather than pointed at, and the inherited 168 030 / 343 344 disagreement is registered OPEN because reconciling it is a design act the carve may not take
13621d3 docs(experiments): U3 reaches u-rev 6 — all nine inherited bare citations are converted and the citation universal becomes a DERIVED command, because the asserted universal that answered MAJOR B was itself the third completeness claim this document falsified in the commit that wrote it
b1df309 docs(experiments): U3's u-rev 5 re-review FAILS on a THIRD completeness claim false in the commit that wrote it — the citation-form fix reached 7 of 16 sites and the head claims it reached all of them
f0edcfe docs(experiments): U2's u-rev 4 re-review returns 0 BLOCKING 0 MAJOR 2 MINOR — the exceptions rule and its derived grep are confirmed structural, and the one residual is the same duplication pattern surviving twice in the head
2ace2a9 fix(tests): the carve-membership pin matched the marker as a SUBSTRING, so a red-team report that pasted the marker's own definition as evidence was declared a carved design unit and turned the gate red
a1d425c docs(decisions): D-333 lands R18 — N-E was recommended by a red team dispatched to break a different row, and the selection stands because it was taken at a registered ladder rung whose twelve cells the attack itself re-derived, flipping if IMPL exceeds the 22/7 ground
```

**THE SUBJECT AND ITS SOURCES HAVE NOT MOVED, so every finding below stands at HEAD:**

```
$ git diff 97aa4d6 HEAD -- docs/experiments/matrix_U4R_restructure_rev2.md docs/experiments/matrix_U4R_REDTEAM.md docs/experiments/matrix_U4R_measurements.md docs/experiments/wp15b_U4_REVIEW_urev7.md | wc -l
0
```

**U4 ITSELF HAS MOVED — TO u-rev 8 — AND EVERY MEASUREMENT I TAKE OF IT IS PINNED.**
All U4 outputs below were taken at `97aa4d6`, which is byte-identical to `0f49c90`, the
revision the u-rev-7 review and the whole matrix lineage describe:

```
$ git diff 0f49c90 97aa4d6 -- docs/experiments/U4_soundness_instrument.md | wc -l
0
$ git show 97aa4d6:docs/experiments/U4_soundness_instrument.md > $S/U4_at_97aa4d6.md
$ wc -l $S/U4_at_97aa4d6.md
1886 …/U4_at_97aa4d6.md
```

Every U4 command in this report was re-run against that pinned blob after the drift was
discovered and returned exactly what is pasted. Where a command reads `$U4` it is that
blob. U4 at HEAD is a different document — **u-rev 8, 2022 lines** — and where I quote
it as it now stands I say so.

**THREE OF THE NINE NEW COMMITS BEAR DIRECTLY ON THE FIELD, AND ALL THREE TELL AGAINST
IT.**

1. **`d328d1d` and `78b4876` repair ALL EIGHT of the u-rev-7 findings in place** —
   including the three F7 registers as *"reached by no row in any field authored so
   far"* — **with no restructure, no matrix and no selection, while this matrix was
   awaiting its attack.** The entire evidential sample the new cost column scores was
   consumed by the null row's method during the red-team round. See **K6**, **M10**,
   **m11**.
2. **`2ace2a9` falsifies the mechanism F6 describes** and records that round 1's own
   report turned the census gate **RED** by pasting the marker. See **m10**.

I edited no repository file except this report, staged nothing, committed nothing,
created no worktree, and ran no git write command. The one build command I ran was
`cargo test -p pistol-solver --test wp15b_census`, to confirm this report does not turn
that gate red; it passes with this report present (**m10**).

**GROUND:** every command below was run by me, in the live repository. Every output
block is my own, pasted complete. Every U4 command is pinned to the `97aa4d6` blob as
stated above; every other command was re-run at exit and is stated at the state it was
taken.

**SCOPE, AS DISPATCHED:** obligations A–H. Round 1
(`docs/experiments/matrix_U4R_REDTEAM.md`, at `53c0c0b`) is prior art, not law, and
**this round charges revision 2 for errors it inherited from round 1 as well as for
its own.** Most of revision 2's facts are transcribed from round 1; three of the
inherited transcriptions are wrong.

**HEADLINE.** Revision 2 was instructed to author **(e)** and **(c′)**. It authored a
row it calls (e) that is **not** the row round 1 named — round 1's (e) *contains (a)*
and reaches three findings; revision 2's (e) is the extraction half alone and is
scored **0 of 8**. The row round 1 said *dominates the recommendation* was scored out
of the field by relabelling. That, and not any single number, is why this field falls
again.

---

# 1. PER-FACT REPRODUCTION VERDICT (obligation A, D-330's third part)

Revision 2's sourcing rule, tested cell by cell:

> **SOURCING RULE FOR EVERY MEASURED CELL IN THIS REVISION.** Every number below is
> transcribed from a **command and its pasted output in a file that is in the tree**

| fact | verdict |
|---|---|
| **F1** (findings by region, the fact the field is ranked by) | **NOT SOURCED TO ANY COMMAND, IN ANY FILE, ANYWHERE IN THE TREE** — and its site attribution is wrong for MINOR 7. The region spans re-derive; the finding counts are hand-assigned. See **K2**, **m7** |
| **F2** (256 vs 382) | **REPRODUCES** — both commands, every number, exact |
| **F3** (the record has no other home) | **DOES NOT REPRODUCE.** The pasted output names one file per pattern; the command returns **three**. It was already false when revision 2 was written. See **M2** |
| **F4** (68 % outside the folds) | **REPRODUCES** — 101 / 314 / 213, `D-320` 20 at 0 %, `eight of twelve` = 1, all exact |
| **F5** (status matter) | **REPRODUCES** — 52 / 32 / 160 / 31 / 34 exact. **But no command is pasted with it**, and the pasted label under-describes the command that produced `34`. See **m2** |
| **F6** (census membership) | **REPRODUCES WITH ELIDED OUTPUT** — `sed -n '656,659p'` returns four lines; revision 2 pastes two. See **m1** |
| **F7** (what no option reaches) | **MIS-TRANSCRIBED FROM ITS CITED SOURCE, AND FALSIFIED BY THE TREE.** It cites round 1's M4/M5 table; that table records MINOR 7 as reached by (b)'s un-fold. And MINOR 7 has a **second site, U4:1748, inside U4-Z**, which (a) and (f) both cut. See **K2** |
| **the confession** (head, ¶3) | **CONTAINS A CLAIM THAT IS FALSE AT THE REVISION IT LANDS** — *"the string occurs in exactly one place in this repository"*. It occurs in three files, one of them this document. See **M1** |
| **the confession's causal claim** | **DOES NOT REPRODUCE AGAINST ITS OWN SOURCE** — round 1's m1 records F3's `705` as taken from the instrument's own pasted output, not from any agent's summary. See **M3** |

**Two of seven facts do not reproduce as stated, one is unsourced, one is elided, and
the self-disclosure contains a false census.** The practice was not fixed; it was
re-described.

## The commands, re-run

```
$ U4=docs/experiments/U4_soundness_instrument.md
$ grep -n "^> ## \|^> ### " $U4 | head -9
207:> ## **SELECTED — S-M, AND IT IS NOT S-E (D-323)**
343:> ## THE RECORD OF THE EARLIER STATES — kept, STAMPED at u-rev 6, and RE-SCOPED at u-rev 7
376:> ### u-rev 1 — SELECTION OPEN — M3 HAS NO MATRIX, AND S-E IS NOT SELECTED HERE
867:> ## **SELECTED — N-E, AND IT IS NOT THE ROW THE MATRIX RECOMMENDED (D-329)**
989:> ## THE RECORD OF THE EARLIER STATES OF §9 — kept, STAMPED, and SUPERSEDED IN PART AT u-rev 7
1002:> ## THE RECOVERY (T1'), THE DIFF, AND WHY THE SELECTION DOES NOT STAND
1010:> ### DIFF 1 — recovered text vs. the text the DECISION-RED-TEAM attacked
1020:> ### DIFF 2 — recovered text vs. an EXTERNALLY DERIVED referent
1037:> ### VERDICT: **DIFFERS. SELECTION OPEN. The carve does not select N-A.**
$ for r in "207 342" "343 459" "867 988" "989 1257"; do set -- $r; printf '%s..%s bq=%s total=%s\n' "$1" "$2" "$(awk -v a=$1 -v b=$2 'NR>=a&&NR<=b&&/^>/{c++}END{print c+0}' $U4)" "$(( $2-$1+1 ))"; done
207..342 bq=135 total=136
343..459 bq=115 total=117
867..988 bq=121 total=122
989..1257 bq=267 total=269
```

```
$ printf '%-22s %7s %10s %10s %8s\n' token total in_folds outside_folds pct_in
$ for t in D-323 S-M D-329 D-316 N-E DEPENDS-OPEN-THEORY 7e0a328 af8082a D-320 "four conditions" "SELECTED AND NOT BUILT"; do tot=$(grep -Fc -- "$t" $U4); inf=$(awk 'NR>=205&&NR<=459 || NR>=865&&NR<=1257' $U4 | grep -Fc -- "$t"); printf '%-22s %7s %10s %10s %7s%%\n' "$t" "$tot" "$inf" "$((tot-inf))" "$((inf*100/tot))"; done
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
TOTAL=314 IN=101 OUT=213
$ grep -Fc -- "eight of twelve" $U4
1
```

```
$ awk '/^\*\*REVIEW STATUS/{s=NR} /^Theory citations are calculus IDs/{e=NR} END{print "status_start="s"  theory_para="e"  span="(e-s)}' $U4
status_start=115  theory_para=167  span=52
$ awk '/^## U4-A\./{s=NR} /^## 8\. MATRIX M3/{e=NR} END{print "U4-A start="s"  s8_start="e"  span="(e-s)}' $U4
U4-A start=173  s8_start=205  span=32
$ printf 'lines naming a u-rev of this unit: '; grep -c "u-rev [0-9]" $U4
lines naming a u-rev of this unit: 160
$ printf 'lines naming a wp15b_U4_REVIEW report: '; grep -c "wp15b_U4_REVIEW" $U4
lines naming a wp15b_U4_REVIEW report: 31
$ printf 'lines carrying REPAIRED/WITHDRAWN/RE-CHECKED/MARKED AT: '; grep -cE "REPAIRED|WITHDRAWN|RE-CHECKED|MARKED AT u-rev|RE-READ AT|FOLDED AT" $U4
lines carrying REPAIRED/WITHDRAWN/RE-CHECKED/MARKED AT: 34
```

---

# 2. KILLS

## K1 (KILL) — **REVISION 2'S (e) IS NOT ROUND 1'S (e).** The row that dominates the recommendation was re-scoped out of the field under its own name, and then scored 0.

**The claim (revision 2, head):**

> It named two missing rows, one of which dominates the recommendation, and
> instructed: *"Re-author with **(e)** and **(c′)** in the field, replace the
> line-count cost column with a findings-reached column, and re-attack."* **This
> revision does exactly that.**

**What round 1's (e) actually is (`matrix_U4R_REDTEAM.md:385–392`):**

```
$ sed -n '385,392p' docs/experiments/matrix_U4R_REDTEAM.md
**Row (e) — INDEX + EXTRACT, not INDEX + DELETE.**

> U4-Z becomes the pointer index of (a). **In addition, the two
> `THE RECORD OF THE EARLIER STATES` blocks — U4:343–459 and U4:989–1257, 382
> blockquote lines, MEASURED — are MOVED to a companion record file** (or appended to
> the existing selection records) and replaced in U4 by one citation each. **The two
> `SELECTED` folds (207–342 and 867–988, 256 lines) stay**, because they are what the
> unit is bound by and D-331 calls marked quotation the preferred instrument.
```

and round 1's own scoring of it:

```
$ grep -n "(e) contains (a)" docs/experiments/matrix_U4R_REDTEAM.md
409:* **Ground 3** — identical: (e) contains (a), so it reaches MAJOR 2, 3, 4.
```

**What revision 2 calls (e):**

> **(e) EXTRACT THE RECORD** — the 382 `RECORD OF THE EARLIER STATES` lines move out
> … **Findings reached: 0 of 8.** … **IT IS A READABILITY CHANGE AND THIS MATRIX WILL
> NOT PRICE IT AS A DEFECT REMEDY.**

**The first clause of round 1's (e) — *"U4-Z becomes the pointer index of (a)"* — is
gone, and with it the three MAJORs the row reached.** Round 1's (e) scores **3 of 8**
on revision 2's own metric (3.5 once MINOR 7's U4-Z site is counted, K2); revision 2
prints **0**.

**Why it breaks.** Round 1's field verdict rests on (e) dominating: *"A row that
dominates the recommendation — satisfying its grounds identically while owing less —
is a KILL."* Revision 2 says it has authored that row, prints a row of the same
letter with the dominating half removed, scores the remainder zero, and declares it
*"carried as a live row and NOT recommended"*. The dominating row is therefore
**still not in the field**, two rounds running, while the document asserts it is.
Every comparison in the RECOMMENDATION section — *"4.5 of 8 … against (a)'s 3, (c′)'s
1.5 and (e)'s 0"* — is taken against a row that does not exist in round 1's report.
This is not a transcription slip; the deleted clause is the first sentence of the
block revision 2 cites. Severity: **KILL against the field.**

## K2 (KILL) — the two facts that carry the new cost column, F1 and F7, are the only two with **no command anywhere in the tree**; F7 is falsified by the table it cites and by the file it describes.

**The claim (sourcing rule):** *"Every number below is transcribed from a command and
its pasted output in a file that is in the tree."*

**F1 has no command.** Round 1's K4, which revision 2 names as F1's source, introduces
the block with prose and pastes a bare table:

```
$ grep -n '^\$' docs/experiments/matrix_U4R_REDTEAM.md | awk -F: '$1>330 && $1<360'
(no output)
$ sed -n '344,347p' docs/experiments/matrix_U4R_REDTEAM.md
So I measured defect density by region, taking each finding's site from
`wp15b_U4_REVIEW_urev7.md` and each region's span from F1:

```
```

There is no command, no script, and no instrument for the assignment of eight
findings to seven regions. **The region spans re-derive; the finding counts are a
hand-reading of a review report, taken by a party that then prescribed the metric.**

**F7 is falsified by its own cited source.** F7 reads:

> **F7. WHAT NO OPTION IN EITHER FIELD REACHES.** From the red team's M4/M5 table …
> **MAJOR 1's §8.7 blockquote half (U4:833–837), MINOR 5 (U4-M item 1) and MINOR 7
> (§9's unmarked `91 test lines`) are reached by no row in any field authored so
> far.**

The cited table says the opposite for MINOR 7:

```
$ sed -n '554p' docs/experiments/matrix_U4R_REDTEAM.md
| **MINOR 7** | 937–939 | §9 SELECTED fold (867–988) | no | yes |
```

The last column is *"reached by the UN-FOLD"*. Round 1's (b) is `(a) + the un-fold`,
so a row in round 1's field reaches MINOR 7.

**And MINOR 7 has a second site that both round 1 and revision 2 missed, inside
U4-Z:**

```
$ grep -n "91 test lines" $U4
939:>    paid 91 test lines for ONE guard arm.
1748:- **THE FOUR CONDITIONS RIDING WITH N-E ARE UNPAID, AND EACH BINDS IMPL** (D-329). … **(3)** An **item-10 driving test** is owed for both new refusal classes, in two halves with a control, against a precedent of 91 test lines for one arm at `b067d47`; no row of round 4 was costed for it. …
```

The review states it in terms — *"`91 test lines` is added at u-rev 7 at **two
sites** with no MEASURED / ESTIMATED mark"* (`wp15b_U4_REVIEW_urev7.md`, MINOR 7) —
and pastes exactly this grep. **U4:1748 is inside U4-Z (1441–1886), which is precisely
what (a) replaces with an index.** So on the matrix's own half-convention (a) reaches
**3.5** and (f) reaches **5.0**, F1's U4-Z row should read `MIN 1` not `MIN 0`, and
F7's list of unreachable findings has two members, not three.

**Why it breaks.** The column the whole field is now ranked by is built from one fact
that no command produces and one fact that contradicts the source it cites. D-330's
third part makes a fact whose pasted output an attacker cannot reproduce a finding
against the round *even where its conclusion survives*; here there is no output to
reproduce at all for the load-bearing one. Severity: **KILL against the cost column.**

## K3 (KILL) — (c′)'s `~1.5` is inherited from an argument about a GENERATOR, and (c′) is defined by deleting the generator. Nothing in (c′) reaches either universal.

**The claim ((c′)'s cells):**

> **~1.5 of 8** — the head halves of MAJOR 1 and MINOR 6, which are the false
> universals in the REVIEW STATUS table.
> **Class instances prevented: the completeness-universal class** — a status table
> that asserts *"no live sentence in this unit …"* about a body it does not own
> **cannot assert it**.

**Where the 1.5 comes from — round 1's m7, in full:**

```
$ sed -n '758,766p' docs/experiments/matrix_U4R_REDTEAM.md
**m7 (MINOR).** Option (c)'s absolute claim is false: *"A generator would faithfully
regenerate a STATE file while **every one of the eight findings stayed exactly where
it is**."* MINOR 6's finding is *"the universal, not the citation"*
(`wp15b_U4_REVIEW_urev7.md:302`) and lives at U4:142, inside the REVIEW STATUS table
(c) moves; MAJOR 1's fix scope names *"the REVIEW STATUS row's universal at U4:138
either verified or narrowed"* as half the repair, also inside it. **A generator cannot
emit a hand-written completeness universal — which is why moving the table removes
both**, rather than regenerating them. (c) reaches roughly 1.5 of 8 …
```

**The whole mechanism is the generator's inability to emit a hand-written sentence.**
Revision 2's (c′) is defined as *"**No `tools/` artefact, no generator**, no
SHELL_CHECKLIST review, no driving test"* — it keeps the table hand-maintained and
moves the file it lives in. **A hand-maintained table carries its sentences with it.**
Moving `U4:138`'s *"NO LIVE SENTENCE IN THIS UNIT IDENTIFIES THE DIFFERENTIAL GATE AS
S-E"* into `U4_STATE.md` leaves a false universal, in a new file, about U4:835 —
which is still there:

```
$ sed -n '833,837p' $U4
> **REPAIRED by shape 2 (D-316): the letters are dropped and the four gates are
> named.** The double-list dies with the letters — S-E is not listed beside the
> letters it was one of, because it IS the differential gate and is named once, in
> §8.2. The two-shape comparison the selection was made from stands unedited in
> U4-Z, with the selection recorded beneath it.
```

The matrix's replacement mechanism — *"a status table … about a body it does not own
**cannot** assert it"* — is a **norm**, not a consequence. Nothing in (c′) enforces
it; there is no generator, no rule, no derived command and no test. The unit's own
author writes such universals freely in exactly this register (`U4:141`, `U4:165`).
D-331's own diagnosis forecloses the hope: *"Care does not fix it either — three
sessions applied care."*

**Why it breaks.** `1.5` of `(f)`'s `4.5` — one third of the recommendation's whole
margin over (a) — is the reach of a mechanism the recommended row does not contain.
Corrected, **(f) reaches what (a) reaches**, and the second half buys a relocation.
Severity: **KILL against (c′), and against (f)'s margin.**

## K4 (KILL) — (c′)'s cut cell, executed literally, moves lines out of the §8 RECORD fold and out of §8.2, §8.4 and §8.7 — so (f)'s *"no record is deleted or moved; no fold is touched"* is false of its own half.

**The claims:**

> **(c′)** … The head's REVIEW STATUS block (52), U4-A's lineage table (32) **and the
> 34 marker lines** move to a hand-maintained STATE section or companion file …
> **the unit's body keeps no status prose.**
>
> **(f)** … Both cuts above, and nothing else. **No record is deleted or moved; no
> fold is touched**; no `tools/` artefact is created.

**Where the 34 lines actually are — MEASURED, with the measurer's own pattern:**

```
$ grep -nE "REPAIRED|WITHDRAWN|RE-CHECKED|MARKED AT u-rev|RE-READ AT|FOLDED AT" $U4 | cut -d: -f1 | awk '{n=$1; if(n<=172)h++; else if(n<=204)a++; else if(n<=459)f8++; else if(n<=813)b8++; else if(n<=864)s87++; else if(n<=1257)f9++; else if(n<=1346)s91++; else if(n<=1361)t++; else if(n<=1440)m++; else z++; tot++} END{printf "head=%d U4-A=%d S8fold=%d S8body=%d S8.7=%d S9fold=%d S9.1=%d U4-T=%d U4-M=%d U4-Z=%d total=%d\n",h+0,a+0,f8+0,b8+0,s87+0,f9+0,s91+0,t+0,m+0,z+0,tot}'
head=19 U4-A=0 S8fold=3 S8body=4 S8.7=3 S9fold=0 S9.1=1 U4-T=0 U4-M=0 U4-Z=4 total=34
$ grep -nE "REPAIRED|WITHDRAWN|RE-CHECKED|MARKED AT u-rev|RE-READ AT|FOLDED AT" $U4 | awk -F: '$1>204' | cut -c1-100
366:> **(i)** §8.2's opening `FOLDED AT u-rev 6` block, which states the selected
371:> `RE-READ AT u-rev 6` block and the u-rev 6 marks inside the ledger's M3, M4 and M6
372:> cells; **(v)** §8.7's four-name wiring sentence and the `FOLDED AT u-rev 6` paragraph
504:> **FOLDED AT u-rev 6.** The instrument for this gate is **S-M**: per-node **EQUALITY**
637:**THE LETTERS ARE GONE. B3 IS REPAIRED HERE, BY SHAPE 2 (D-316).** The soundness
778:**RE-READ AT u-rev 6, AND THAT CLAIM IS NOT TRUE OF EVERY ROW.** **M3's witness is
792:| M3 | The FILTERED row emits `Cover::cells()` flattened at phase 0 and does not re…
814:### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
833:> **REPAIRED by shape 2 (D-316): the letters are dropped and the four gates are
851:**FOLDED AT u-rev 6 — WHAT THIS SENTENCE SAID AND WHAT IT CAN AND CANNOT SPECIFY
1316:> **MARKED AT u-rev 7, AT THIS SITE AND NOT ONLY ELSEWHERE (MINOR 4 of
1507:**MARKED AT u-rev 7.** This block records the u-rev 2 EXECUTION, and at that execution
1555:**THE DIAGNOSIS THIS PARAGRAPH CARRIED UNTIL u-rev 6 IS FALSE AND IS WITHDRAWN**
1657:  **RE-CHECKED AT u-rev 7, AND THIS SEAM IS NOT THE ONE D-329 SELECTS.** Two
1652:```

Three separate defects fall out of one measurement:

1. **Lines 366, 371 and 372 are inside the §8 head fold (205–459)** — blockquote lines
   of the two-sided RECORD stamp, the D-331-protected matter (f) promises not to
   touch. `(f)`'s *"no fold is touched"* is false of `(c′)`'s own cut cell.
2. **Line 814 is a SECTION HEADING**; line 792 is §8.4's M3 ledger cell; line 504 is
   §8.2's live instrument statement; line 833 is **MAJOR 1's own defect site**. These
   are live specification, and *"the unit's body keeps no status prose"* removes
   them. The cut cannot be executed as written without gutting the specification the
   trigger was never about.
3. **Only 19 of the 34 are in the head.** The row's scope figure is inflated by 79 %
   over the status matter it actually names, and the inflation runs into (f)'s
   findings column via (c′).

**Why it breaks.** A cut cell that, taken literally, moves the recommendation's own
defect site and three lines of protected record is not a cut cell; and the number that
sizes it counts fifteen lines that are somewhere else. Severity: **KILL against (c′)
as scoped, and against (f)'s third ground.**

## K5 (KILL, by domination) — the row with a **landed in-tree precedent** — replace a hand-enumerated set with a RULE AND A DERIVED COMMAND — is in neither field, and round 1 named the precedent in a paragraph revision 2 did not transcribe.

**The precedent, landed in this work package before the matrix was authored:**

```
$ git show 7dfd047 --stat | head -8
commit 7dfd047b365576379fcc612c53eedd83f0a548d5
Author: Timmy Burn <148332956+seeligto@users.noreply.github.com>
Date:   Sat Aug 22 13:51:09 2026 +0200

    docs(experiments): U2 reaches u-rev 4 — the exceptions list that went stale twice is REPLACED by a rule and a derived grep, because a hand-enumerated set inside a document that keeps being repaired is the generator D-331 names

 docs/experiments/U2_node_protocol.md | 173 ++++++++++++++++++++++-------------
$ sed -n '62p;80p' docs/experiments/U2_node_protocol.md
$ grep -n "(CARVE-EXCEPTION" docs/experiments/U2_node_protocol.md
and that is the generator D-331 names**; a rule plus a derived enumeration
```

**Round 1 named it, at the head of its own report:**

> **A SIBLING PRECEDENT LANDED WHILE THIS ATTACK RAN, AND IT BEARS ON THE FIELD.**
> … U2 did not un-fold a quotation; it replaced a hand-maintained enumeration with a
> derived one. `7473a6f` is U3's, and it removed a completeness claim rather than
> correcting it. **Both landed remedies act on hand-maintained status and completeness
> matter … and neither cuts record or quotation.**

**Revision 2 transcribes F1–F7 from round 1 and does not carry this paragraph.** It
is the one paragraph in round 1 that tells against relocation rows and toward a
derived-enumeration row, and it is the only paragraph of round 1's substance that
revision 2 drops.

**Why the missing row dominates.** MAJOR 2 — the round's own headline finding — **is
a hand-enumerated set that went stale**: *"eight of twelve items ENGAGED … plus
D-329's four conditions on top"*, falsified by D-329 removing item 11. A rule plus a
derived command reaches it **by construction**, not by relocation; it reaches MAJOR 3
and MAJOR 4 (both are hand-held claims about ADR and tree state that a derived
citation cannot hold); it reaches the two completeness universals behind MAJOR 1's
head half and MINOR 6 the way U3's u-rev 5 reached its own — *"the completeness claim
… removed rather than corrected"*; **it deletes no record, moves no fold, creates no
new file, and asks for no "normative use vs restatement" judgement.** On the field's
own domination test — *"satisfying the grounds identically while owing less"* — it
satisfies all three of the recommendation's grounds and owes strictly less than a
composite that relocates two blocks and a table.

**And (c′) is that precedent inverted.** U2's landed answer to D-331 was to *remove*
a hand-maintained enumeration; (c′) is defined as **keeping** one, hand-maintained,
in a new file. The field contains the anti-precedent and not the precedent.
Severity: **KILL by domination.**

## K6 (KILL) — the ranking column measures the thing the matrix's own text says the selection is not about, and it ranks the null row first.

**The two claims, both in revision 2:**

> **Cost is stated as FINDINGS REACHED** … (the column the options are ordered by)
>
> **It does not claim a restructure repairs the round's findings.** … **A selection
> here is about RECURRENCE, not about this round's eight defects**, and revision 1's
> failure to say so is part of why it fell.

**And the column's own top row:**

> **(n) NULL** … **8 of 8** — a repair reaches every finding by construction.

**Why it breaks.** The field is ordered by a metric scored over a **sample of eight
past defects**, in a document that states in terms that the decision is not about
those eight. The row that maximises the metric is the row the matrix refuses. So the
metric decides nothing, and the criterion that *does* decide — *"class instances
prevented"* — is a prose column with no instrument, no derivation and no command
anywhere (obligation G, **M9**). Recommendation ground 2, *"It is ranked by findings,
not by mass, and it wins on findings"*, is therefore an argument from the column that
the matrix has disqualified, offered in place of the column that carries the weight.

**And the sample was exhausted during this attack.** `d328d1d` repaired MAJOR 1 and
MINOR 5, 7, 8 — three of them F7's *"reached by no row"* set — and `78b4876` repaired
MAJOR 2, 3 and 4. **All eight, by ordinary repair, with no restructure, while the
matrix sat awaiting its red team** (**M10**, **m11**). A field ordered by *"findings
reached (of 8)"* is ordered by a denominator that the null row's own method drove to
zero inside one working day.

Round 1's m11 prescribed *"findings reached"* as the honest replacement for line
counts. It is a better column than line counts and still not a criterion for
recurrence: **it scores deletion and repair identically** — (a) "reaches" MAJOR 2, 3
and 4 by removing the paragraphs that carry them, (n) reaches all eight by rewriting
them, and neither operation is measured against the class's next instance. A column
under which erasing text and fixing text score alike cannot separate a restructure
from a repair, which is the exact separation this matrix exists to make. Severity:
**KILL against the field's ranking.**

---

# 3. MAJOR FINDINGS

## M1 (MAJOR) — the confession's own census is false at the revision it lands. The class recurs a third time, inside the paragraph confessing its second occurrence.

**The claim (revision 2:18–21):**

> **The red team established that no such line exists in the instrument at any
> revision** — the string occurs in **exactly one place in this repository**, inside
> revision 1, as a quotation attributed to a file that does not contain it.

**Re-run at `97aa4d6`:**

```
$ grep -rn "20 live-line occurrences" --include=*.md . | cut -c1-95
docs/experiments/matrix_U4R_restructure.md:187:reads *"20 live-line occurrences"*; the complete o
docs/experiments/matrix_U4R_restructure_rev2.md:18:report, quoting a summary line reading *"20 li
docs/experiments/matrix_U4R_REDTEAM.md:97:> reads *"20 live-line occurrences"*; the complete outp
docs/experiments/matrix_U4R_REDTEAM.md:107:docs/experiments/matrix_U4R_restructure.md:187:reads *
```

**Three files, four lines. One of them is the sentence making the claim.**

Round 1's version of this sentence was true when round 1 ran the command. Revision 2
copied the *conclusion* into a new document at a later revision, where it is false —
which is D-331's disease stated in D-331's own words: *"a repair that restates
manufactures a stale claim at the moment it lands, because the restatement is true
only at the revision it is written and nothing re-reads it."*

**Why it breaks, and why it is the sharpest fact about revision 2.** The paragraph
exists to confess a false quotation and to found a rule against restating from
elsewhere. It restates from elsewhere, and the restatement is false. Severity:
**MAJOR**, and it is the answer to obligation H: **the confession is not accurate.**

## M2 (MAJOR) — F3's pasted output does not reproduce, and it was already stale when revision 2 was written.

**The claim (F3):**

> **F3. THE 382 RECORD LINES HAVE NO OTHER HOME IN THE TREE.** MEASURED by the red
> team, K1:
> ```
> $ for pat in "5e8c5e4a1e7ad416" "DIFF 2 — recovered" "MECHANICAL RECOVERY"; do echo "--- $pat ---"; grep -rln -- "$pat" docs/; done
> --- 5e8c5e4a1e7ad416 ---
> docs/experiments/U4_soundness_instrument.md
> …
> ```

**Re-run, verbatim:**

```
$ for pat in "5e8c5e4a1e7ad416" "DIFF 2 — recovered" "MECHANICAL RECOVERY"; do echo "--- $pat ---"; grep -rln -- "$pat" docs/; done
--- 5e8c5e4a1e7ad416 ---
docs/experiments/matrix_U4R_restructure_rev2.md
docs/experiments/matrix_U4R_REDTEAM.md
docs/experiments/U4_soundness_instrument.md
--- DIFF 2 — recovered ---
docs/experiments/matrix_U4R_restructure_rev2.md
docs/experiments/matrix_U4R_REDTEAM.md
docs/experiments/U4_soundness_instrument.md
--- MECHANICAL RECOVERY ---
docs/experiments/matrix_U4R_restructure_rev2.md
docs/experiments/matrix_U4R_REDTEAM.md
docs/experiments/U4_soundness_instrument.md
```

**Round 1's report landed at `53c0c0b`, an ancestor of `97aa4d6`**, so the command
already returned two files before revision 2 was written; publishing it added a
third. The *conclusion* survives — the other two hits are this dispute's own
quotations of U4 — but the pasted evidence does not, and D-330's third part is
explicit that this is a finding *"even when the fact's conclusion survives"*.

**Why it breaks.** F3 is the fact that makes the record irremovable, and it is one of
the three facts revision 2 leans on to distinguish (e) from the fallen (b). **This is
the same cell that was stale in revision 1** (round 1's m1, `705` for `707`): the
matrix has now shipped a stale F3 twice, under two different sourcing rules.
Severity: **MAJOR.**

## M3 (MAJOR) — the confession's diagnosis is false for two of the three defects it explains, and the rule built on that diagnosis does not prevent the defect revision 2 then repeats.

**The claim (revision 2:22–26):**

> The author had taken the phrase from the measuring agent's transient completion
> message rather than from the landed instrument file, and **the same mistake produced
> revision 1's other two transcription defects (F3's `705` for `707`, and three wrong
> section counts in F4).**

**Round 1's actual finding about F3 (m1):**

> The instrument disclosed this drift in a LOUD NOTE and named the exact cell; **the
> matrix pasted the stale output and dropped the disclosure** …

F3's `705` came **from the instrument file**, correctly, and went stale. It was not
taken from any completion message. And round 1's M2 attributes F4's three wrong
counts to a counting rule that *"changes inside one table"* — a reading defect over
the instrument's pasted output, again not a completion message.

**Why it breaks, and it is not pedantry.** The remedy revision 2 adopts follows from
the false diagnosis: *"**No cell is transcribed from any agent's summary or completion
message**."* That rule is orthogonal to two of the three defects it is offered
against, and **revision 2's F3 fails in exactly the way revision 1's F3 failed** (M2)
while complying with it perfectly. A sourcing rule that is satisfied by copying a
stale command output out of a tree file is a rule against the wrong thing; D-331's
rule — *point, do not restate* — is the one that would have caught it, and it is the
matrix's declared governing law. Severity: **MAJOR**, and it is the second half of
obligation H's answer.

## M4 (MAJOR) — the class's measured history is wider than (f)'s two surfaces, and one of this round's own findings sits on a surface the class has struck before that no row touches.

**The claim ((f)'s class column):** *"**BOTH class surfaces the round produced**: U4-Z's
stale-dependent claims and the head's false completeness universals."*

**The class's history, from the reviews themselves:**

```
$ sed -n '437p' docs/experiments/wp15b_U4_REVIEW.md | cut -c1-190
### 1. The head block, U4-A and §9's closing paragraph were not re-read when the u-rev 4 and u-rev 5 folds landed: the unit's own status surfaces still describe the u-rev 1 sta
$ sed -n '46,49p' docs/experiments/wp15b_U4_REVIEW.md
The u-rev 4 and u-rev 5 folds were appended without re-reading the head block, the
lineage table, U4-T, U4-M or the U4-Z lead-in, so the document's most-read surfaces
still describe the u-rev 1 state, and its two registered *test rows* and its *cost
row* still carry S-E as adopted with no caveat at all.
```

So the class has landed in: the **head block**, the **lineage table**, **§9's closing
paragraph**, **U4-T**, **U4-M**, a **cost row** (u-rev 5 review, BLOCKING 1); **U4-Z**
(u-rev 6 review, BLOCKING 1, and u-rev 7's MAJOR 2/3/4); and **§8.7** (u-rev 7's
MAJOR 1 live half, MINOR 6's live citation).

**(f) reaches two of those seven surfaces.** And this round's **MINOR 5 is in U4-M** —
a surface the class struck at u-rev 5 — which the matrix classifies as *"a repair,
owed regardless"* rather than as evidence that its scoping is fitted to one round.

**Why it breaks.** *"Both class surfaces **the round produced**"* is the tell: the
option's scope is defined by where the class landed **this time**. D-331 rests
explicitly on *"the recurrence rather than any instance"*; a remedy scoped to the
latest instance is the instrument the ADR says does not work. Severity: **MAJOR**, and
it is the concrete form of obligation B's *"fitted to the answer"* question: not the
metric's choice — round 1 prescribed the metric — but **its scoping**, which is the
author's and was taken after the data.

## M5 (MAJOR) — D-331 clause (3) names four artefacts; the field's enumeration substitutes a grep for one of them, and the substituted-away artefact is the CHANGE LOG, where the class landed at u-rev 5.

**D-331 clause (3), verbatim (`docs/decisions.md`, D-331):**

> (3) **STATUS AND SUMMARY MATTER CARRIES POINTERS ONLY** — REVIEW STATUS tables,
> **change logs**, lineage tables and OPEN lists may say that a finding exists, that
> it is repaired and where the repair lives, and may not restate the finding's content
> or the repair's content

**Round 1's enumeration, which revision 2 inherits (K3):**

> Clause (3) names four artefacts. U4 has all four: the REVIEW STATUS table …, U4-A's
> lineage table …, the 34 REPAIRED / WITHDRAWN / RE-CHECKED markers, and U4-Z's OPEN
> list.

**"Change logs" has been replaced by "the 34 markers".** U4's change log is a distinct,
named artefact:

```
$ sed -n '66,70p' $U4
**WHAT IS NOT A VERBATIM CARVE, u-rev BY u-rev — the change log, RE-READ at u-rev 6
rather than appended to.** The text is a verbatim carve apart from cross-reference
retargets and the following, each stated where it occurs.

- **u-rev 1** — **B4** (the tactical-suite gate's "all three staged tactical configs …
```

It runs U4:66–104, it is **outside** (c′)'s cut (which takes 115–166, 173–204 and the
34 grep hits — none of them in 66–104), and it is where the class landed once already:

```
$ sed -n '105,107p' $U4
**The sentence this block replaces named three repairs and omitted the u-rev 4 and
u-rev 5 folds entirely — the two largest non-verbatim additions in the unit.** That is
BLOCKING 1 of the u-rev 5 review, at the surface a reader meets first.
```

**Why it breaks.** (f) is recommended on the ground that *"it is the only row whose
scope is the trigger's scope"* — the unit's status matter. On the governing law's own
enumeration it reaches three of the four clause-(3) artefacts and misses the one with
a recorded prior instance. Severity: **MAJOR.**

## M6 (MAJOR, flip-clause coherence, obligation E) — flip clause 3's remedy selects an option the same document declares unselectable.

**Flip clause 3:**

> **If the red team measures that U4-Z's pointer index cannot name a home for more
> than one owed item** … **then the index is not the instrument and (c′) is selected
> alone.**

**(c′)'s own failure-mode cell:**

> **It cannot be selected alone.**

**Why it breaks.** If clause 3 fires, the matrix's instruction is to select a row it
has itself ruled out, and the field has no selectable member — the identical shape
round 1 charged as m4 against revision 1's clause 1 (*"the remedy names an option the
matrix has already condemned"*), in the revision whose flip-clause preamble asserts
*"Each trigger names a remedy that answers that trigger."* Severity: **MAJOR.**

**And the trigger cannot be operated.** *"Cannot name a home"* is undefined. Round 1
fired it once on U4-Z item 15. Testing further OPEN items, at least two more carry
determinations that exist only in U4:

```
$ grep -rlin "PER-CI COST IS UNGROUNDED" docs/ crates/ tools/
docs/experiments/U4_soundness_instrument.md
docs/experiments/wp15b_U4_REVIEW_urev7.md
$ grep -rln "SECOND HALF" docs/
docs/experiments/U4_soundness_instrument.md
docs/decisions.md
```

U4:1834's disposition — *"S-E's SECOND HALF … IS NOT SELECTED AND NOT REJECTED … no
attacked matrix row carries it at u-rev 7 either, and IMPL may not read it as
registered"* — and U4:1841's per-CI-cost ungrounding are U4's own determinations; the
only other hits are a review verifying them and an ADR deciding something else.
**Whether that counts as "cannot name a home" is a judgement the clause does not
supply**, so the clause's firing condition is unadjudicable — and if it does fire, the
remedy is the contradiction above.

## M7 (MAJOR) — D-328's red-team exclusion is quoted with its operative half cut off, and the cut does the work: revision 2 has **no stakeless measurer**, and its measurer prescribed its field, its metric and its endorsed row.

**The claim (sourcing rule):**

> `docs/experiments/matrix_U4R_REDTEAM.md` (the red team's re-runs, **which D-328
> expressly permits**: *"it does not apply to a red team, whose whole job is to re-run
> the author's commands"*)

**D-328's actual sentence:**

> it does not apply to a red team, **whose whole job is to re-run the author's commands
> against the author's text**

The dropped clause is what scopes the exclusion. D-328 exempts a red team from the
split **when it re-runs an author's commands against that author's text** — it does
not appoint a red team as the measurer of a *later* matrix. There is no elision mark.

**And the party revision 2 uses as its measurer is not stakeless in it.** Round 1
named the two rows revision 2 authors, prescribed the cost column it adopts, supplied
F1 and F4 whole, endorsed (c′) as *"the only row whose scope matches the fired
trigger"*, and declared round-1-(e) dominant. D-328(2) requires *"a measurer with no
stake in the recommendation"*. **Revision 2 registered no commands of its own and
dispatched no measurer**; its MEASURED cells are one round's attack transcribed by the
next round's author.

The head's disclaimer — *"The recommendation below is the author's … and no option in
this revision was named in any dispatch"* — is true of the *dispatch* and false of the
substance: every component of (f) was supplied by an earlier party, and the sentence
says so itself two clauses later (*"derived from the red team's own findings-by-region
measurement"*). D-305's base rate (*"FOUR of the six option matrices recommended an
option that a fresh-context DECISION-RED-TEAM then dominated"*) is the reason this
dimension is the one that matters, and round 1's M8 verdict — *"an unattacked
selection wearing a matrix on the field dimension"* — recurs with the supplier
changed rather than removed. Severity: **MAJOR.**

## M8 (MAJOR) — not one number in the cost column carries a MEASURED or ESTIMATED mark, and the half-convention that produces `4.5` is undefined and used inconsistently.

**CLAUDE.md:** *"EVERY NUMERIC CLAIM IN THE MATRIX IS MARKED **MEASURED** OR
**ESTIMATED**"*. **D-291:** *"what the sentence forbids is a number that LOOKS
measured and is not"*. **Precedent, same work package:** `restructure_selection_15b.md`
F1 — *"no numeric claim in the matrix was marked MEASURED or ESTIMATED … D-291's
precedent, third occurrence"*.

**The column:** `8 of 8`, `3 of 8`, `~1.5 of 8`, `0 of 8`, `~4.5 of 8`. **No mark on
any of them.** The FACTS section marks scrupulously; the column the field is ranked by
does not. These are derivable in seconds from the review report — D-291's own test —
and when I derived them, two were wrong (K2).

**And the half is undefined.** The same content is counted two ways inside one table:

> **(c′)** … **~1.5 of 8** — the head halves of **MAJOR 1 and MINOR 6**
> **(f)** … **~4.5 of 8** — MAJOR 2, 3, 4, the **head half of MAJOR 1**, and **MINOR 6**

(c′)'s cell enumerates two halves (1.0) and prints 1.5; (f)'s cell reads MINOR 6 whole
(1.5). The review settles it — MINOR 6's *"finding is the universal, not the
citation"* — so (f)'s reading is the right one and (c′)'s own prose is wrong; but the
convention is nowhere stated, and **flip clause 4's trigger is a threshold on this
number** (*"more than 4.5 of the eight findings"*). A registered threshold on an
unmarked, undefined, hand-assigned quantity is not a criterion. Severity: **MAJOR.**

## M9 (MAJOR, obligation G) — "class instances prevented" is the column that decides and it is pure assertion; three further undefined properties are load-bearing.

The matrix concedes one undefined property (*"reviewable in one sitting"*) and marks
its line counts as proxies for it. It concedes nothing about the column that actually
carries the recommendation:

| row | "class instances prevented" | what derives it |
|---|---|---|
| (n) | **NONE** | nothing |
| (a) | *"the three stale-dependent MAJORs of this round, **which are the whole recurrence**"* | falsified by M4 |
| (c′) | *"the completeness-universal class"* | falsified by K3 |
| (e) | *"NONE measured"* | F1, which has no command (K2) |
| (f) | *"BOTH class surfaces the round produced"* | falsified by M4 |

Three further undefined terms do load-bearing work: **"reached"** (K6 — deletion and
repair score alike); **"status matter"** (the trigger's scope, and the thing (c′) is
built to move — measured at K4 to be 19 lines or 34 depending on the reading);
**"cannot name a home"** (M6). **Obligation G's answer: yes — the column that decides
the field is an assertion, and it is the one column the matrix does not mark as one.**
Severity: **MAJOR.**

## M10 (MAJOR) — the one ground for excluding the column's top-scoring row is a determination the project did not follow: the patch-and-re-review round the trigger refuses has now happened, three times, and closed all eight findings.

**The claim ((n)'s cell, the field's only exclusion argument):**

> **NOT SELECTABLE, and the reason is narrower than revision 1 said.** The reviewer's
> trigger refuses *"another patch-and-re-review round"*, and it is real.

**What happened while this matrix awaited its attack:**

```
$ git log --oneline 97aa4d6..HEAD -- docs/experiments/U4_soundness_instrument.md
78b4876 docs(experiments): U4's three stale-dependent MAJORs are repaired at their sites — …
75ae04e docs(experiments): U4 adopts the landed-SHA citation form and converts the six citations this session's own serial repairs had just made stale, …
d328d1d docs(experiments): U4's MAJOR 1, MINOR 5, 7 and 8 are repaired at the sites the restructure matrix MEASURED that no option reaches — …
$ grep -n "^\*\*u-rev [0-9]" docs/experiments/U4_soundness_instrument.md | head -1
15:**u-rev 8.** Carved from `docs/experiments/wp15b_design.md` §8, §9, §11.6 and §12
$ wc -l docs/experiments/U4_soundness_instrument.md
2022 docs/experiments/U4_soundness_instrument.md
```

**Why it breaks.** The matrix's field has exactly one exclusion mechanism, and it is a
reviewer's determination quoted from `wp15b_U4_REVIEW_urev7.md`. That determination
has been overtaken: u-rev 8 is the patch-and-re-review round, it landed unattacked and
unmatrixed, and it discharged every finding the cost column scores. Whatever the
architect now selects, **it will not be selected against the state the matrix
describes**, and the row the matrix calls *"dead as a whole-round answer"* is the row
the project executed.

Two consequences the field cannot absorb:

* **The findings-reached column is now empty of subject matter.** Its numbers score
  reach against eight defects that no longer exist at HEAD. Nothing in the document
  survives that comparison, because the column *is* the comparison.
* **The size proxy moved the wrong way, unopposed.** U4 went 1886 → **2022** lines
  doing the repairs — the direction `section_owner_table.md` §11 hands to the
  architect as its open question — while the matrix authored to answer that question
  was waiting to be attacked.

**AND THE RECOMMENDATION'S OWN OBJECTIVE WAS THEN TAKEN WITHOUT IT.** One further
commit landed as this report was being written:

```
$ git log --oneline -1
b9f4aea docs(experiments): U4 reaches u-rev 8 — the REVIEW STATUS block says WHERE each finding is answered instead of restating it, and the round records that D-331 is silent about the self-completeness class that has now failed three units in consecutive rounds
```

That is D-331 clause (3) applied to the REVIEW STATUS block — **the substance of
(c′)** — achieved **inside the unit, by rewriting the block to pointers, without
extraction, without a companion file, without a matrix and without a selection from
this field.** (c′)'s premise is that the block must MOVE for the clause to bind; the
project has just demonstrated the block can be made to comply where it stands. That
is the strongest possible evidence for **K3**: the reach (c′) claims comes from
rewriting the sentences, not from relocating the file, and rewriting was available all
along.

Severity: **MAJOR**, and it is the answer to obligation F in its sharpest form: the
metric ranked (n) first, the matrix excluded it on a trigger, and the project then did
(n) anyway — and did (c′)'s job with it.

---

# 4. MINOR FINDINGS AND WOUNDS

**m1 (MINOR).** F6's paste is short against R7's complete-output rule:

```
$ sed -n '656,659p' crates/pistol-solver/tests/wp15b_census.rs
/// tree rather than from the constant above.
const CARVE_MARKER: &str =
    "<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->";

```

Four lines; the matrix prints two. Inherited from round 1's K6, which elided the same
way. Nothing turns on it — the conclusion (membership is the marker) is right — but a
paste presented as a command's output is either complete or marked.

**m2 (MINOR).** F5 pastes five output lines with **no command**, and the labels
under-describe the commands in the measurer's file. `34` is produced by
`grep -cE "REPAIRED|WITHDRAWN|RE-CHECKED|MARKED AT u-rev|RE-READ AT|FOLDED AT"`, which
matches two patterns the label does not name; the label taken literally returns 28:

```
$ grep -cE "REPAIRED|WITHDRAWN|RE-CHECKED|MARKED AT" $U4
28
```

A reader of the matrix alone cannot check the attribution of any F5 number — which is
what CLAUDE.md means by a dry run exercising ATTRIBUTION rather than syntax.

**m3 (WOUND, obligation E).** Flip clause 5 — *"(e) revives as a selection if the
architect states a ground for it that is not a defect-rate ground"* — states a revival
condition and **no consequence for the selected row**. This is round 1's m5 class
(*"A clause that cannot change what was selected is not a flip clause"*), recurring in
the revision that says its clauses were written against the F5 family.

**m4 (MINOR, obligation E).** Flip clause 2's gloss mis-cites its law: *"i.e. that
**D-311's label discipline** requires the block to be in the unit"*. D-311 is about
**revision labels bumping on append** and nothing else; U4 implements it in a separate
paragraph (`U4:108`), not in the REVIEW STATUS block. The clause's general trigger
stands; the authority it names does not support it.

**m5 (MINOR, obligation E).** Flip clause 4 is the F5 class in miniature: the
**trigger** is a count (*"reaches more than 4.5 of the eight findings"*) and the
**remedy** invokes a different test (*"selected over (f) **by domination**, on the
definition this project has used twice: satisfying the grounds identically while owing
less"*). A row reaching more findings need not satisfy the grounds identically nor owe
less; the clause licenses a selection its own cited definition would refuse.

**m6 (MINOR).** Flip clause 1's registered consequence covers only one direction. If
the halves **are** separable — which they are — nothing follows, even though
separability is what makes a composite a scoring device rather than an option
(obligation C). The clause is written so that only the finding favouring the composite
has a consequence.

**m7 (MINOR).** F1's U4-Z row reads `MAJ 3 MIN 0`. MINOR 7's second site is U4:1748,
inside U4-Z, so it is `MAJ 3 MIN 1`, and the derived line *"findings touching a head
fold: ['MINOR 7', 'MINOR 8']"* should record MINOR 7 as straddling. Inherited from
round 1's K4; both rounds read the review's MINOR 7 as one site when it states two.

**m8 (MINOR, obligation C).** (f)'s halves are not disjoint: four of (c′)'s 34 marker
lines (1507, 1555, 1657, 1829) are inside U4-Z, which (a) replaces wholesale. The
composite's cost is therefore not the sum of its parts, in the direction the matrix
does not disclose, and the row that *"is two changes in one option"* is two changes
that overlap.

**m9 (WOUND).** *"AUTHORED, NOT SELECTED"* plus a new filename is correct under D-311
and is noted as compliant; but the matrix nowhere states its own governing revision or
that of the review report it grounds itself on, in a project whose Process section
requires the instrument that produces a registered number to be *"named in the
pre-registration WITH ITS REVISION"*. F1's producer is a report at `53c0c0b`, cited by
path and not by revision.

**m10 (MINOR, and it went RED in CI).** F6's *conclusion* survives — a file without
the marker is not a member — but the mechanism it states was not the mechanism the pin
applied at `97aa4d6`. The pin asked `text.contains(CARVE_MARKER)`, so **any document
that merely QUOTED the marker was declared a carve member**, which is what round 1's
own report did:

```
$ git show 2ace2a9 --oneline --stat | head -3
2ace2a9 fix(tests): the carve-membership pin matched the marker as a SUBSTRING, so a red-team report that pasted the marker's own definition as evidence was declared a carved design unit and turned the gate red
$ git show 2ace2a9 -- crates/pistol-solver/tests/wp15b_census.rs | grep -A3 "fn carries_marker"
fn carries_marker(text: &str) -> bool {
    text.lines().any(|line| line.trim() == CARVE_MARKER)
}
```

and the fix's own doc comment records the consequence: *"the pin went RED declaring two
argument documents to be carved design units. The reviewer of a sibling unit hit the
same red gate and had to isolate it before it could report on its own subject."*

**Why it matters to this field and not only to the test.** F6 is the fact that revives
(c′) — *"CENSUS MEMBERSHIP IS A MARKER, NOT A DIRECTORY — so a new file need not be a
reviewable carve member"* — and the matrix presents it as settled by reading four lines
of the pin. Reading four lines of a test is not measuring what the test does; the
behaviour it actually had was wide enough to red the gate on the report making the
argument. The conclusion holds at HEAD and held then; **the ground was verified by
inspection where a run was available in seconds**, which is D-291's own test. I ran
the gate with this report present and it is green:

```
$ cargo test -p pistol-solver --test wp15b_census
running 6 tests
test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
test a_document_quoting_the_carve_marker_is_not_a_carve_member ... ok
test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... ok
test wp15b_census_reproduces_the_registered_populations ... ok
test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
test the_census_pin_reads_every_carved_document_it_names ... ok

test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 3.59s
```

**m11 (MINOR — the mechanical half of M10, recorded separately because it is a
reproduction check).** The repair reached **both** of MINOR 7's sites, which is
independent confirmation of **K2**: the second site exists, it is inside U4-Z, and
both F1 and F7 miss it.

```
$ grep -n "91 test lines" docs/experiments/U4_soundness_instrument.md | cut -c1-72
995:>    **MEASURED 91 test lines for ONE guard arm at `b067d47`.**
1846:- **THE FOUR CONDITIONS RIDING WITH N-E ARE UNPAID, AND EACH BIND
```

(at HEAD; the pinned `97aa4d6` blob has the same two sites at 939 and 1748, both
unmarked, which is what the review charged.)

---

# 5. PER-OPTION SURVIVAL VERDICT

| option | verdict | reason |
|---|---|---|
| **(n) NULL** | **SURVIVES — AND IT HAS BEEN EXECUTED** | Revision 2's narrowing of its death (dead as a whole-round answer, alive as every other row's complement) is coherent and is an improvement on revision 1, and its quotation of the trigger is accurate. But the exclusion has been overtaken by events: `d328d1d` and `78b4876` repaired all eight findings in place and U4 is now u-rev 8 (**M10**). **Obligation F's answer: the incoherence is not in the null row, it is in the column and in the exclusion.** (n) tops the ranking column at 8 of 8, is excluded on a criterion the column does not measure, and is then what the project did. Wound: its *"class instances prevented: **NONE**"* is asserted with nothing behind it, and it is the one cell that would have to be true for any other row to be worth taking. |
| **(a) INDEX** | **SURVIVES WOUNDED** | Still the only row scoped to the region measured to carry every MAJOR in the unit's body, and its reach is **3.5**, not 3 (K2). Wounds: its flip clause has fired at least once and the trigger for a second is unadjudicable (M6); its scope is fitted to the surfaces this round produced while the class's measured history spans seven (M4); "reached" for its three MAJORs means "deleted", which is not shown to prevent the next instance (K6). |
| **(c′) HAND-MAINTAINED STATUS EXTRACTION** | **FALLS** | Its entire findings-reached figure is inherited from an argument whose mechanism is the **generator it is defined by deleting** (K3), so nothing in it reaches either universal; its cut cell, executed as written, moves three lines of D-331-protected record and four lines of live specification including the recommendation's own defect site (K4); and the sibling unit's landed answer to the same law, in the same work package, was to **remove** a hand-maintained enumeration rather than relocate one (K5). |
| **(e) EXTRACT THE RECORD** | **FALLS AS AUTHORED — and the row round 1 named is still not in the field** | This is not round 1's (e) (K1). As authored it is a readability change scored 0, honestly labelled; it falls not for what it says about itself but because it occupies the letter of the row that dominates, so the field reads as having tested that row when it has not. |
| **(f) = (a) + (c′)** | **FALLS** | It is a composite of a wounded row and a fallen one. Its margin over (a) is entirely (c′)'s 1.5, which K3 removes; corrected, **(f) reaches exactly what (a) reaches** and adds a relocation. Its cut cell asserts *"no fold is touched"* against its own half's measured contents (K4). It is ranked by a column its own text disqualifies (K6) and scoped to one round's surfaces (M4). It misses the fourth clause-(3) artefact its governing law names (M5). And its numbers carry no marks and rest on an undefined half (M8). |

---

# 6. MAY A SELECTION BE TAKEN FROM THIS FIELD?

## **NO. NO SELECTION MAY BE TAKEN FROM THIS FIELD, AND NO ROW IN IT IS SELECTABLE.**

The recommendation falls. Its second half falls. The row round 1 identified as
dominating is **still absent** after a revision authored to add it. The column the
field is ranked by has no instrument for its load-bearing fact, contradicts the
document's own statement of what the selection is about, and ranks first a row the
matrix refuses.

**One row survives wounded — (a) — and it is not selectable from this document**,
because the only argument offered for it here is a comparison against (c′), (e) and
(f) as revision 2 scores them, and all three scores are wrong.

**AND THE FIELD NO LONGER DESCRIBES THE DOCUMENT IT GOVERNS.** While this attack ran,
U4 reached **u-rev 8** and all eight findings the cost column scores were repaired in
place, with no restructure (**M10**). A selection taken from this field now would be
taken against a state that does not exist, on a column whose denominator is empty, in
a unit that grew 1886 → 2022 lines while the matrix waited. **Any revision 3 must be
re-grounded on the u-rev-8 review that is now owed, not on `wp15b_U4_REVIEW_urev7.md`.**

**What a revision 3 would have to contain**, stated so the next round is not a fourth
re-author:

1. **Round 1's (e) as round 1 wrote it** — `(a) + extraction of the 382 record lines`
   — scored against (f) honestly. It is the row the field has twice claimed to carry
   and twice not carried.
2. **The DERIVED-ENUMERATION row (K5)**, on the landed `7dfd047` precedent: a rule plus
   a derived command replacing every hand-held enumeration and completeness universal
   in the unit. It reaches MAJOR 2 by construction, deletes no record, touches no fold,
   creates no file, and is the only candidate whose prevention claim is a mechanism
   rather than an assertion.
3. **A §8.7-scoped row.** §8.7 is fifty-one lines at 3.92 findings / 100 lines — the
   highest density in the unit, 12.6× the folds — and carries MAJOR 1's live half and
   MINOR 6's live citation. **No row in either field touches it**, and both fields say
   so while ranking by defect density.
4. **A row that changes what a REVIEWER is asked to check, and nothing structural.**
   Every instance of this class in three rounds was found by a review, never by a
   structure; the u-rev 7 reviewer found MAJOR 2, 3 and 4 by building a 54-row claim
   inventory by hand. That instrument is the one thing measured to work, and it is in
   neither field.
5. **A convention-level row** covering all four units' review-status surfaces rather
   than U4's copy of one, which is the scope D-331 clause (4) actually binds and the
   question `section_owner_table.md` §11 hands to the architect.
6. **Marks on every number in the cost column**, a stated half-convention, and a
   command — anywhere in the tree — for the findings-by-region table.

---

# 7. THE STRONGEST ATTACK SURVIVING AGAINST EACH SURVIVING OPTION

*Written to be quoted VERBATIM in an ADR line. Assembling one of these from parts is a
residual under D-329.*

**Against (n) NULL:**

> The null row is the only row in the field that scores full marks on the field's own
> ranking column — eight of eight — and it is excluded by a criterion that column does
> not measure, so what the exclusion convicts is the column and not the row; the one
> cell that would justify running any other option, the null row's "class instances
> prevented: NONE", is an assertion with no derivation in a document that marks its
> every other number; and while the matrix waited for its attack the project repaired
> all eight findings in place and shipped u-rev 8, so the row the field calls dead is
> the row the field's own subject actually took, at a cost of a hundred and thirty-six
> further lines and no matrix at all.

**Against (a) INDEX:**

> Option (a) is scoped to the region measured to hold every MAJOR the review found in
> the unit's body, and it is the only row of two fields whose reach survives being
> re-checked — but "reached" here means the paragraph is deleted rather than the claim
> corrected, which scores identically to a repair the matrix says is not what the
> selection is about; its own flip clause has fired at least once with no stated way to
> tell whether it has fired again; and its scope is drawn around the two surfaces this
> round's findings happened to land on, while the same class has been recorded in the
> head block, the change log, the lineage table, §9's closing paragraph, U4-T, U4-M and
> §8.7 across the unit's three failed reviews.

---

# 8. FINDINGS ATTEMPTED AND REJECTED, WITH THE REPRODUCER

Recorded so a later round does not re-discover them.

- **"F1's region spans do not reproduce."** Attempted: re-derived every span from the
  section map (`awk '/^#{2,3} /{print NR"\t"$0}'`) — §8 fold 205–459 (§8.1 at 460),
  §9 fold 865–1257 (§9.1 at 1258), U4-Z 1441–1886, §8.7 814–864, head 1–172, U4-M
  1362–1440. **REJECTED** — all six exact, and the line totals 255 / 393 / 446 / 51 /
  172 / 79 are right.
- **"F4 does not reproduce."** Attempted: the whole eleven-token loop. **REJECTED** —
  314 / 101 / 213 and every per-token row identical, `D-320` 20 at 0 %,
  `eight of twelve` = 1.
- **"F2 does not reproduce."** Attempted: both commands. **REJECTED** — 135 / 115 /
  121 / 267 and the nine heading lines, exact.
- **"F5's numbers do not reproduce."** Attempted: all five, from the measurer's
  registered script. **REJECTED** — 52 / 32 / 160 / 31 / 34, exact. Charged only as
  m2, for pasting output without its command.
- **"The confession's `22 matching lines, 23 occurrences` is wrong."** Attempted:
  `grep -c "differential gate" $U4` → **22**; `grep -o … | wc -l` → **23**; and the
  measurer's M10 paste carries 22 rows and no summary line
  (`grep -n "SUMMARY\|summary" docs/experiments/matrix_U4R_measurements.md` → exit 1).
  **REJECTED** — that part of the confession is exactly true, and the measurer's report
  is clean at this cell as round 1 found.
- **"Prior instances of the class were outside U4-Z, so (a)'s prevention claim is
  false."** Attempted: read u-rev 6's BLOCKING 1 and MAJOR 1. **REJECTED as stated** —
  both are in U4-Z (`wp15b_U4_REVIEW_urev6.md:1`, the residual paragraph and the OPEN
  bullet), and D-325's instance is U4-Z's B3 section. The narrower charge that
  survives is M4: the class is *also* recorded at six other surfaces, so "the whole
  recurrence" overstates.
- **"(e) as revision 2 scores it is wrong — the record blocks contain findings."**
  Attempted: MINOR 7 (939) and MINOR 8 (957) against the RECORD spans 343–459 and
  989–1257. **REJECTED** — both sit in the §9 **SELECTED** fold (867–988), which (e)
  keeps, so revision 2's `0 of 8` is right *for the row it authored*. The finding is
  K1: that row is not the (e) round 1 named.
- **"Revision 2 breaches D-311 by not bumping a label."** Attempted: it is a new file
  titled REVISION 2, and D-311 binds *"designs, pre-registrations, option matrices and
  selection records"* to bump on append. **REJECTED** — the label moved with the file.
- **"F6's conclusion is false — a new STATE file would be a carve member."**
  Attempted: read `wp15b_census.rs` around the marker and the disk filter.
  **REJECTED** — membership is `CARVE_MARKER`, a filter over file contents, exactly as
  stated; only the paste is short (m1).
- **"The matrix restates D-329's conditions and thereby breaches its own governing
  law."** Attempted: read (f) and (c′) against D-331 clause (1). **REJECTED** — the
  matrix cites D-329 by reference and does not restate its conditions; its D-331
  breaches are the two found (M1, M2) and are about the tree, not about an ADR.
- **"Flip clauses 1–5 carry the F5 class throughout."** Attempted: read each trigger
  against each remedy. **REJECTED as a blanket charge** — clauses 1 and 2 are
  trigger-and-remedy coherent. Three separate defects are charged instead: M6
  (condemned remedy), m3 (no consequence), m4 (mis-cited authority), m5 (trigger and
  remedy test different things), m6 (one-directional consequence).

---

# 9. CLOSING STATE

```
$ git rev-parse HEAD
b9f4aead811b4f603fc7e9044b655d2670ab60d4
$ git status --porcelain
?? docs/experiments/matrix_U4R_REDTEAM_round2.md
```

The `??` is this report. **No other repository file was created, edited, staged or
committed; no git write command was run; no worktree was created.** **HEAD MOVED TEN
COMMITS DURING THIS ATTACK AND WAS STILL MOVING AT EXIT** — U4 reached u-rev 8 in four
of them, and the last of the four rewrote the REVIEW STATUS block to pointers, which is
(c′)'s objective taken without (c′); **the subject, round 1's report, the
measurer's log and the u-rev-7 review are byte-identical across all of them**, and every
U4 measurement above is pinned to the `97aa4d6` blob and was re-run against it after the
drift was found. The one build command run was the census gate, green with this report
present.

*Fresh-context DECISION-RED-TEAM round 2 of `docs/experiments/matrix_U4R_restructure_rev2.md`
at `97aa4d6`. Seven MEASURED facts re-run: four reproduce, one does not, one is
mis-transcribed from its cited source and falsified by the tree, one is sourced to no
command in any file. The self-disclosure of revision 1's fabricated quotation contains a
false census of the tree and a false diagnosis of two of the three defects it explains.
**Six KILLS, ten MAJOR, eleven MINOR/WOUND.** (c′), (e) and the recommendation (f) FALL;
(a) survives wounded; (n) survives and has already been executed, together with the
substance of (c′). The row round 1 said
dominates the recommendation is still not in the field, and every finding the new cost
column scores was repaired by the null row's method while this matrix awaited its
attack. **NO SELECTION MAY BE TAKEN FROM THIS FIELD.***
