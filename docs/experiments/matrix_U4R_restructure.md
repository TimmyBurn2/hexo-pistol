# MICRO-MATRIX U4-R — what to do about U4's restatement surface

Status: **AUTHORED, NOT SELECTED.** Awaits fresh-context DECISION-RED-TEAM.

**WHO AUTHORED THIS, AND THE R11 SPLIT DECLARED ON ITS FACE.** Authored by the
session dispatching WP-1.5b's CLAIM-HOME round. **Under D-328 as amended by D-330
(architect ruling R11) this author wrote NO MEASURED CELL.** Every `MEASURED`
number below was produced by a separate stakeless measurer that was told nothing
about the options or the recommendation; its complete log, the registered command
script it ran verbatim, and the revision it ran at are landed beside this file at
`docs/experiments/matrix_U4R_measurements.md`. **THAT FILE IS THIS MATRIX'S
INSTRUMENT AND IT HAS A REVISION** — it lands in the same commit as this matrix,
and a change to it reopens this matrix's review exactly as an amendment to this
matrix would. Every MEASURED cell below carries its command and its complete
output inline (R7), including the zeros.

**THE RECOMMENDATION'S PROVENANCE IS DISCLOSED, BECAUSE A RED TEAM SHOULD KNOW IT.**
Option **(b)** was marked RECOMMENDED **in the architect's dispatch**, before this
author read a line of the measurements. This matrix carries that recommendation
and states its own grounds for it below, and it also records — under *What the
author could not settle* — the one place where the measurements tell against the
dispatch's framing. An option recommended by the party who commissioned the matrix
is exactly the shape D-328 exists to distrust, and the attacker is told so here
rather than having to find it.

**Subject:** `docs/experiments/U4_soundness_instrument.md` at u-rev 7, revision
`871e678`. **Ground:** `docs/experiments/wp15b_U4_REVIEW_urev7.md` — VERDICT FAIL,
0 BLOCKING / 4 MAJOR / 4 MINOR — whose reviewer recorded, as a determination
independent of its own severity ratings, that *"the trigger for an architect
restructure of U4's status matter is met; another patch-and-re-review round is the
wrong instrument."* **Governing law: D-331 (R15), the CLAIM-HOME law, landed at
`c9d4e64` before this matrix was authored.**

---

## FACTS — every one MEASURED by the stakeless measurer, command and complete output inline (R7)

**F1. The unit is 1886 lines, and its two largest sections are the two matrix folds.**

```
$ wc -l docs/experiments/U4_soundness_instrument.md
1886 docs/experiments/U4_soundness_instrument.md
```

Per-heading spans (measurer follow-up (a); the awk is in the instrument file):

```
173	## U4-A. Lineage …                                      span=32
205	## 8. MATRIX M3 — the soundness instrument …            span=660
460	### 8.1 Why S-C fell                                    span=42
502	### 8.2 THE DIFFERENTIAL GATE …                         span=133
635	### 8.3 The other three gates, re-scoped …              span=135
770	### 8.4 The mutation ledger, with witnesses             span=29
799	### 8.5 Floors, not printed counts                      span=8
807	### 8.6 REJ-DEPTHPROOF, stated where it belongs         span=7
814	### 8.7 Gate wiring — B3 REPAIRED, shape 2, D-316       span=51
865	## 9. MATRIX M4 — the snapshot's config seam …          span=482
1258	### 9.1 The five amendments … after that attack         span=77
1335	### 11.6 One thing this WP does NOT close               span=12
1347	## U4-T. The tests this unit registers                  span=15
1362	## U4-M. What this unit measures                        span=79
1422	### Cost                                                span=19
1441	## U4-Z. ADR lines this unit owes …                     span=446
1456	### B3, gate (b) — SETTLED …                            span=183
1639	### ADR lines                                           span=104
1743	### OPEN — carried forward, not closed by the carve     span=144
```

**F2. The matrix-derived fold blocks at the heads of §8 and §9 are 638 blockquote
lines, and they are 88 % of every blockquoted line in the unit.**

```
$ awk '/^## 8\. MATRIX M3/{f=1;s=NR} f&&/^### 8\.1/{print "S8 fold span "s".."NR-1" = "(NR-s)" lines"; f=0}' docs/experiments/U4_soundness_instrument.md
S8 fold span 205..459 = 255 lines
$ awk '/^## 8\. MATRIX M3/{f=1} f&&/^### 8\.1/{f=0} f&&/^>/{c++} END{print "S8 blockquote lines="c+0}' docs/experiments/U4_soundness_instrument.md
S8 blockquote lines=250
$ awk '/^## 9\. MATRIX M4/{f=1;s=NR} f&&/^### 9\.1/{print "S9 fold span "s".."NR-1" = "(NR-s)" lines"; f=0}' docs/experiments/U4_soundness_instrument.md
S9 fold span 865..1257 = 393 lines
$ awk '/^## 9\. MATRIX M4/{f=1} f&&/^### 9\.1/{f=0} f&&/^>/{c++} END{print "S9 blockquote lines="c+0}' docs/experiments/U4_soundness_instrument.md
S9 blockquote lines=388
```

```
$ for f in docs/experiments/U4_soundness_instrument.md docs/experiments/matrix_M3_selection.md docs/experiments/matrix_M4_axisA_selection.md; do
    total=$(wc -l < "$f"); bq=$(grep -c "^>" "$f"); echo "$f  total_lines=$total  blockquote_lines=$bq"; done
docs/experiments/U4_soundness_instrument.md  total_lines=1886  blockquote_lines=727
docs/experiments/matrix_M3_selection.md  total_lines=216  blockquote_lines=15
docs/experiments/matrix_M4_axisA_selection.md  total_lines=149  blockquote_lines=15
```

250 + 388 = **638**, against 727 blockquote lines in the whole unit — **87.8 %**.
638 of 1886 is **33.8 % of the unit**.

**F3. The content those folds restate is landed in the tree, in four files, and is
therefore retrievable without them.**

```
$ wc -l docs/experiments/matrix_M3_selection.md docs/experiments/matrix_M4_axisA_selection.md \
        docs/experiments/matrix_M4_axisA_round4.md docs/experiments/matrix_M3_soundness_instrument_rev2.md docs/decisions.md
  216 docs/experiments/matrix_M3_selection.md
  149 docs/experiments/matrix_M4_axisA_selection.md
  305 docs/experiments/matrix_M4_axisA_round4.md
  451 docs/experiments/matrix_M3_soundness_instrument_rev2.md
  705 docs/decisions.md
 1826 total
```

**F4. THE DUPLICATION SURFACE, WHICH IS WHAT D-331 CALLS A CLAIM WITH MORE THAN ONE
HOME.** For each registered claim token, the number of DISTINCT sections of U4 that
state it (full per-section listing in the instrument file, M6):

| token | occurrences (lines / total) | distinct sections |
|---|---|---|
| `D-323` | 46 / 50 | **12** |
| `S-M` | 54 / 61 | **12** |
| `DEPENDS-OPEN-THEORY` | 12 / 12 | **9** |
| `D-329` | 57 / 62 | **8** |
| `D-316` | 27 / 29 | **8** |
| `N-E` | 64 / 72 | **7** |
| `7e0a328` | 12 / 14 | **6** |
| `af8082a` | 9 / 11 | **5** |
| `D-320` | 20 / 28 | **4** |
| `four conditions` | 8 / 8 | **4** |
| `SELECTED AND NOT BUILT` | 5 / 5 | **4** |
| `eight of twelve` | 1 / 1 | **1** |

The last row is the one to read first: the u-rev-7 review's MAJOR 2 — the
`eight of twelve` claim D-329 falsifies — occurs **once**. **A defect of this class
does not need a duplicated STRING; it needs a duplicated CLAIM.** A grep for
repeated strings would have found nothing at that site, which is why the repair
instruction for MAJOR 1 is by meaning and not by literal grep.

**F5. Self-referential status prose is spread across the unit, not confined to one
section.**

```
$ printf 'lines naming a u-rev of this unit: '; grep -c "u-rev [0-9]" docs/experiments/U4_soundness_instrument.md
lines naming a u-rev of this unit: 160
$ printf 'lines naming a wp15b_U4_REVIEW report: '; grep -c "wp15b_U4_REVIEW" docs/experiments/U4_soundness_instrument.md
lines naming a wp15b_U4_REVIEW report: 31
$ printf 'lines carrying REPAIRED/WITHDRAWN/RE-CHECKED/MARKED AT: '; grep -cE "REPAIRED|WITHDRAWN|RE-CHECKED|MARKED AT u-rev|RE-READ AT|FOLDED AT" docs/experiments/U4_soundness_instrument.md
lines carrying REPAIRED/WITHDRAWN/RE-CHECKED/MARKED AT: 34
```

```
$ grep -E "D-3" docs/experiments/U4_soundness_instrument.md | grep -cE "RECORD|record|REPAIRED|CLOSED|OPEN|SELECTED"
79
```

The head's REVIEW STATUS block is 52 lines and U4-A's lineage table is 32:

```
$ awk '/^\*\*REVIEW STATUS/{s=NR} /^Theory citations are calculus IDs/{e=NR} END{print "status_start="s"  theory_para="e"  span="(e-s)}' docs/experiments/U4_soundness_instrument.md
status_start=115  theory_para=167  span=52
$ awk '/^## U4-A\./{s=NR} /^## 8\. MATRIX M3/{e=NR} END{print "U4-A start="s"  s8_start="e"  span="(e-s)}' docs/experiments/U4_soundness_instrument.md
U4-A start=173  s8_start=205  span=32
```

**F6. THE FINDING THAT DECIDES BETWEEN THE OPTIONS, AND IT IS MEASURED: FIVE OF THE
EIGHT LIVE FINDINGS SIT OUTSIDE U4-Z.** Taking each finding's site from
`wp15b_U4_REVIEW_urev7.md` and reading it against the span map in F1:

| finding | site in U4 | section | inside U4-Z (1441–1886)? |
|---|---|---|---|
| MAJOR 1 | 833–837 (and the status row at 138) | §8.7 | **NO** |
| MAJOR 2 | 1781–1785 | U4-Z OPEN | yes |
| MAJOR 3 | 1748 | U4-Z OPEN | yes |
| MAJOR 4 | 1738–1741 | U4-Z ADR lines | yes |
| MINOR 5 | 1386–1389 | U4-M item 1 | **NO** |
| MINOR 6 | 854 (the universal at 142/182) | §8.7 | **NO** |
| MINOR 7 | 937–939 | §9 | **NO** |
| MINOR 8 | 957–964 | §9 | **NO** |

The spread is confirmed by the registered command M10, whose complete output is in
the instrument file and whose hits fall in §8's fold, §8.2, §8.3, §8.4, §8.7, §9,
U4-M, U4-T and U4-Z — nine sections:

```
$ grep -n "differential gate" docs/experiments/U4_soundness_instrument.md
(complete output at matrix_U4R_measurements.md, M10 — line numbers 138, 159, 192,
 399, 453, 651, 747, 792, 793, 795, 826, 835, 839, 852, 857, 1470, 1504, 1508,
 1512, 1678, 1708, 1820)
```

**NO COUNT IS ASSERTED FOR THIS CELL, AND THE REASON IS A DISCREPANCY THE AUTHOR
FOUND IN THE MEASURER'S OWN REPORT.** The measurer's closing SUMMARY line for M10
reads *"20 live-line occurrences"*; the complete output it pasted immediately above
that summary carries **more rows than that**. Under R7 the pasted output is the
measurement and the summary is not, so this matrix cites the output and states no
number. **This is registered as the first thing for the red team to re-run**, under
D-330's third part — the attacker re-runs the inline evidence rather than reading
it — and it is disclosed here rather than smoothed, because a summary that does not
match its own paste is D-328's exact class appearing inside the very split that
line prescribes. Nothing else in this matrix depends on this cell: the claim it
supports is SPREAD ACROSS SECTIONS, which the line numbers establish without a
count.

**F7. Option (c)'s generator would be a new `tools/` artefact, and the coverage
rule's most recent price for one is MEASURED.**

```
$ git show b067d47 --numstat -- crates/pistol-cli/tests/baseline_snapshot_tests.rs tools/baseline_snapshot.sh
91	0	crates/pistol-cli/tests/baseline_snapshot_tests.rs
54	4	tools/baseline_snapshot.sh
$ ls tools/ | wc -l
18
```

**F8. The recurrence this matrix answers, as the three reports state it.**

```
$ for f in docs/experiments/wp15b_U4_REVIEW.md docs/experiments/wp15b_U4_REVIEW_urev6.md docs/experiments/wp15b_U4_REVIEW_urev7.md; do echo "---- $f"; grep -n "BLOCKING," "$f" | head -4; done
---- docs/experiments/wp15b_U4_REVIEW.md
32:**3 BLOCKING, 3 MAJOR, 5 MINOR.**
---- docs/experiments/wp15b_U4_REVIEW_urev6.md
39:**1 BLOCKING, 2 MAJOR, 4 MINOR.**
---- docs/experiments/wp15b_U4_REVIEW_urev7.md
24:**0 BLOCKING, 4 MAJOR, 4 MINOR.**
```

---

## OPTIONS

| Option | The cut | Cost | Failure mode |
|---|---|---|---|
| **(n) NULL — change nothing; repair the eight findings in place and re-review** | U4 keeps its shape. The eight findings are patched at the eight sites F6 enumerates. | **MEASURED zero restructure lines.** One repair commit set plus one re-review, which is what the last three rounds cost. | **DEAD, AND IT IS DEAD BY A TRIGGER THAT HAS ALREADY FIRED — stated rather than omitted.** The u-rev-7 reviewer recorded, independently of its severity ratings, that the standing architect trigger for a restructure of U4's status matter is met and that *"another patch-and-re-review round is the wrong instrument."* F8 is the measured recurrence: three rounds, and the round that RE-DERIVED the section in full shipped three fresh instances of the class it was rewritten to prevent. This row exists because a field without its null row is a field that never asked whether the change is worth making; it is not selectable. |
| **(a) U4-Z becomes a pointer index; its body is deleted** | U4-Z (446 lines, 23.6 % of the unit) is replaced by an index: one line per owed ADR line and per OPEN item, each naming its home — a §n of this unit, an ADR id, or a record file — and stating nothing else. §8, §9, U4-T, U4-M and the head are untouched. | **ESTIMATED −300 to −380 lines**, bounded below by the three sub-spans an index replaces (B3 183 + ADR lines 104 + OPEN 144 = 431 MEASURED) minus the index itself. Marked ESTIMATED because the number exists only after the cut is executed; every input to the estimate is MEASURED in F1. One commit, no new artefact, no test. | **IT REACHES THREE OF THE EIGHT FINDINGS AND F6 MEASURES WHICH FIVE IT MISSES.** MAJOR 1, MINOR 5, 6, 7 and 8 are outside U4-Z and survive this cut untouched — including MAJOR 1, the semantic survival at §8.7 that the dispatch names as the hardest of the eight. Worse for the option's own premise: the restatement GENERATOR is the two fold blocks (F2: 638 lines, 88 % of the unit's blockquoted matter), and (a) leaves all of it. The three findings it does reach are real, so this is a partial answer wearing the shape of a complete one. |
| **(b) (a) + UN-FOLD: the matrix-derived text at the heads of §8 and §9 is replaced by NORMATIVE USE plus a citation to the landed selection records** — RECOMMENDED (provenance disclosed above) | (a), and in addition: the 638 blockquote lines of F2 are replaced by, for each of M3 and M4, (i) the statement of what IMPL is bound to do, in this unit's own voice, and (ii) a citation to the ADR line and the selection record that decided it. What the unit RESTATES about the selections goes; what the unit USES them for stays. | **ESTIMATED −800 to −1000 lines total** (the (a) cut plus 638 fold lines replaced by roughly 40–80 lines of normative statement and citation). Marked ESTIMATED for the same reason; its inputs are F1, F2 and F3, all MEASURED. Two to four commits, one concern each. No new artefact, no test. | **THE CUT BETWEEN "NORMATIVE USE" AND "RESTATEMENT" IS A JUDGEMENT, AND A WRONG CUT DELETES A BINDING CONDITION.** D-323's five registered conditions and D-329's four ride inside the fold blocks; each is simultaneously a restatement (it is in the ADR line) and a normative use (it binds this unit's IMPL). Second: the unit's own stated property is that *"a unit is readable alone"*, and after (b) a reader must open `matrix_M3_selection.md`, `matrix_M4_axisA_selection.md` and `docs/decisions.md` (F3: 216 + 149 + 705 lines) to recover what the fold said. F3 measures that those files EXIST; it does not measure that a reader will follow the pointer. |
| **(c) Status matter is extracted to a per-u-rev STATE file, GENERATED and not hand-edited; U4's body keeps no status prose** | The head's REVIEW STATUS block (52), U4-A's lineage table (32) and every `REPAIRED`/`WITHDRAWN`/`RE-CHECKED` marker in the body (34 lines) move to `U4_STATE.md`, emitted by a new `tools/` generator from the review reports and the ADR log. The unit body carries design content only. | **MEASURED price of the precedent artefact: 91 test lines and 54 script lines for ONE guard arm** (F7, `b067d47`). A generator of this kind is not one arm. Plus a `tools/SHELL_CHECKLIST.md` review it does not have, item 10 binding by name. | **IT DOES NOT REACH THE DEFECT CLASS, AND F4's LAST ROW IS THE PROOF.** The three stale-dependent statements were PROSE CLAIMS — *"eight of twelve … plus D-329's four conditions on top"*, *"a fifth thing is recorded by D-329"*, *"named in no document now in the tree"* — none of which any generator can emit, because none is derivable from a review report or an ADR id. `eight of twelve` occurs ONCE in the whole unit (F4). A generator would faithfully regenerate a STATE file while every one of the eight findings stayed exactly where it is. Second, and independently: "non-reviewable" is not available to it — `crates/pistol-solver/tests/wp15b_census.rs` reads the carve members, and a generated member is a member. |

---

## RECOMMENDATION

**(b).** Three grounds, in the order they bear.

1. **It is the only option in the field that reaches the generator.** F4 measures
   what D-331 calls the disease: `D-323` and `S-M` each stated in **twelve**
   distinct sections, `D-329` in eight, `N-E` in seven. F2 measures where the bulk
   of it lives: 638 lines, 88 % of the unit's blockquoted matter, in two blocks
   whose whole content is a second telling of decisions that landed elsewhere.
   (a) leaves all 638. (c) leaves all 638 and buys a `tools/` artefact besides.

2. **The first telling is landed, in the tree, and retrievable** (F3). This is what
   makes citation a pointer rather than a deletion: the un-fold destroys no record,
   because the record is not in U4 and never was. The fold blocks are the unit's
   own prose ABOUT four files, not a carry of them.

3. **F6 rules out (a) on the field's own terms.** Five of eight findings sit
   outside U4-Z. An option whose scope is U4-Z is scoped to 3/8 of the evidence
   that produced it.

## WHAT THE AUTHOR COULD NOT SETTLE, AND WHERE THE MEASUREMENTS TELL AGAINST THE DISPATCH

The dispatch's framing is that (b) = (a) + un-fold, i.e. that the U4-Z cut is the
base and the un-fold the addition. **F6 measures the opposite priority**: U4-Z
holds 3 of 8 findings and §8/§9/U4-M hold 5. If the two halves of (b) had to be
taken in sequence, the measurements say the un-fold goes first. This matrix does
not split the option on that ground — both halves are cheap and neither blocks the
other — but it records that the recommendation's own internal ordering is not the
one the measurements support, and an attacker should press it.

**Nothing here is measured about the property the whole restructure was bought
for.** "Reviewable in one sitting" has no instrument in this project.
`section_owner_table.md` §11 hands that question to the architect and does not
answer it, and neither does this matrix. Every line count above is a proxy, and it
is marked as one here rather than in a footnote.

## WHAT FLIPS IT

Each trigger below names a remedy that answers that trigger. (F5 of the
restructure red team killed a flip clause whose trigger and remedy were about
different units; the clauses here are written to be checked against that class.)

- **If the red team shows that a specific registered condition of D-323 or D-329
  cannot be stated as normative use without restating it** — i.e. that the
  restatement and the binding are the same sentence for at least one of the nine
  conditions — then **(b)'s un-fold half is dropped and (a) is selected**, because
  the un-fold's whole claim is that the two are separable. The trigger is about the
  un-fold and the remedy removes the un-fold.
- **If the red team shows that U4-Z's index cannot name a home for some owed item**
  — an OPEN item whose content lives nowhere but U4-Z — then **that item's content
  stays in U4-Z as its home under D-331** and the index covers the rest. The
  trigger is about one item and the remedy is scoped to that item; it does not
  unseat the option.
- **If the red team measures that the un-fold's replacement text reproduces more
  than half the lines it removes**, the option is not doing what it claims and the
  field is re-opened with a fourth option authored — extraction of §8/§9's fold to
  their own record files, which no row here states.
- **(c) revives only if a generator is shown to emit a claim of the kind that
  failed** — that is, if the defect class turns out to be generable after all. F4's
  `eight of twelve` row is the fact to break.

---

*Micro-matrix U4-R, authored under D-328/D-330 (R11) with a stakeless measurer, R7
inline evidence, and D-331 (R15) as its governing law. Instrument:
`docs/experiments/matrix_U4R_measurements.md`, landed in the same commit.*
