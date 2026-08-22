# WP-1.5b U3 — Tier-T qualification and the config shape: DESIGN UNIT

<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->

**HOW TO RESOLVE A `§n` IN THIS FILE.** Every `§n` is the SUPERSEDED document's
own numbering, kept unchanged so an existing citation still resolves. A `§n` that
names a section this unit does not own is prefixed with the unit that does
(**U1**–**U4**, or `WPQ_seed.md`) wherever it appears in prose written or
retargeted by the carve. Inside text carried VERBATIM — matrix cells, quoted
sentences, the seed — a bare `§n` may still name a section that now lives
elsewhere; `docs/experiments/section_owner_table.md` maps every one of them to
its owner, and that is what it is for.


**u-rev 7.** Carved from `docs/experiments/wp15b_design.md` §6, §10 and §12 items
4 and 5 at `6feb40a` (revision 7, CLOSED by D-309 — which records the fresh-context REVIEW-design that FAILED it; the counts are D-309's and are not restated here), plus the two
bullets of §7.2 that are not widening text, under the restructure selected as
option D by D-310. The carve's section-to-owner map is
`docs/experiments/section_owner_table.md`. The superseded document is not
in the tree: it is retrievable at `6feb40a` and nowhere else.

**WHAT IS NOT HERE.** MATRIX M2 — the widening schedule — and every sentence of
§7 that is about it are EXCISED to `WPQ_seed.md` with stage Q, per D-310. This
unit is Tier T only. `quiet_top_k` and `widen_schedule` still appear in §10's
config documents because a `deny_unknown_fields` document is complete or it is
nothing; **whether the D-scope shipped surface keeps those two keys at all is
OPEN and is the architect's, not the carve's** — see U3-Z. **And that OPEN
question is not confined to the config documents (MAJOR 3):** MATRIX M1's ADOPTED
option mitigates its own residual with the deferred stage, and the matrix's whole
cost column is computed with the deferred stage inside it. U3-Z lists what the
decision moves; the RANKING that selects C is what survives it.

**THE TEXT IS A VERBATIM CARVE** apart from cross-reference retargets and the
named repairs, each stated where it occurs. At u-rev 1: **B5** (§10's lead-in —
the config count, which the superseded document stated three different ways
across four sites, is stated once here and cited everywhere else) and **B7**
(§6.2's no-restatement claim, which was absolute and false, is stated at the
strength the pin actually has). At **u-rev 3**: the repairs answering
`docs/experiments/wp15b_U3_REVIEW.md`, finding by finding (see U3-A for the
one-line record now that u-rev 4 has superseded that fold-in table). At
**u-rev 4**: the repairs answering `docs/experiments/wp15b_U3_REVIEW_urev3.md`.
At **u-rev 5**: the repairs answering `docs/experiments/wp15b_U3_REVIEW_urev4.md`,
and the removal of two restating surfaces those findings came out of. At **u-rev 6**:
the repairs answering `docs/experiments/wp15b_U3_REVIEW_urev5.md`. **Which
round is which is stated once, in `U3-A`; the REVIEW STATUS block below covers the
round being answered and no other (D-331).** Every **MEASURED** and **ESTIMATED** mark is the mark the superseded
text carried; no number carried from the superseded text moved, and none gained
or lost a mark.

**LABEL DISCIPLINE — D-311, travelling item T5.** Any append to this unit bumps
its u-rev, however small the diff. A review is dispatched against a named
revision and reviews of superseded revisions do not transfer; the superseded
document carried the label "Revision 7" at both `d94dc0a` and `6feb40a`, which
differ by 69 lines, and that ambiguity is what this rule removes. A citation of
another unit names the unit AND the u-rev cited.

**AND AT u-rev 5 IT NAMES THE REVISION AT WHICH THAT u-rev WAS CURRENT.** D-311's
rule as written manufactures the staleness it exists to prevent: a bare
`**U2** (u-rev 2)` is a live claim about another document's present state, it goes
false the moment that document is bumped, and nothing in this unit re-reads it.
That is exactly what happened — this unit asserted `**U2** (u-rev 2)` at four
sites while U2 had already reached u-rev 3 in an ancestor commit and had already
FAILED its review (`docs/experiments/wp15b_U3_REVIEW_urev4.md`, MAJOR B). **THE
RULE, and it is a rule rather than a claim about how many sites obey it:** a live
cross-unit citation in this unit names the unit, the u-rev, and the revision that
u-rev was current at — `(u-rev N, landed <sha>)` — which is a historical fact and
cannot go stale. Under D-331 the cited unit's CURRENT u-rev has exactly one home,
that unit's own head, and this unit points at it rather than keeping a second copy.

**THIS HEAD ASSERTS NO UNIVERSAL ABOUT HOW MANY SITES OBEY THE RULE, AND THE REASON
IS THAT BOTH PREVIOUS ATTEMPTS TO STATE ONE WERE FALSE.** u-rev 5 asserted it
directly, and it was false at nine sites (`wp15b_U3_REVIEW_urev5.md`, MAJOR C).
u-rev 6 replaced the assertion with a DERIVED COMMAND and said *"the rule and the tree
now agree"* — **and that was false too, because the command was blind**: its pattern
required bold markdown, and a tenth live citation at §10 had none
(`wp15b_U3_REVIEW_urev6.md`, MAJOR D). **A universal laundered through a grep is still
a universal, and a grep narrowed until it cannot see the case it is about is the
failure this project has already recorded once** (`matrix_M4_REDTEAM_round2.md` R11).

**WHAT IS OFFERED INSTEAD IS A FINDING AID, LABELLED AS ONE.** The command below is
WIDER than the citation form — it deliberately returns head apparatus, quotations of
the old form, and prose that names a unit without citing it, all of which a reader
triages. It is offered because a wide instrument that returns noise can be checked,
and a narrow one that returns nothing cannot:

```
$ grep -nE '\*\*U[1234]\*\*|(^|[^A-Za-z])U[1234] §' docs/experiments/U3_tier_t.md | grep -v 'landed'
```

**THIS UNIT DOES NOT CLAIM THAT COMMAND IS COMPLETE EITHER.** A citation shape neither
the bold form nor the `§` form matches would be invisible to it as well, and nothing
here has established that no such shape exists. What the head states is the RULE; what
the tree contains is what the reader checks. **Two citations are deliberately NOT in the form and are not
exceptions to it:** the two in the paragraph above quote the OLD bare form as the
defect being described, and §10's *"Revision 6's §8.3"* names the SUPERSEDED document
at `6feb40a`, which has no u-rev and is not **U4**.

**REVIEW STATUS — u-rev 5 WAS REVIEWED AND FAILED; u-rev 6 IS THE REPAIR.**
`docs/experiments/wp15b_U3_REVIEW_urev5.md`, REVIEW-design (re-review), fresh
context, dispatched against the named revision **`7473a6f`** — **VERDICT FAIL**,
**0 BLOCKING, 1 MAJOR, 0 MINOR**. **THIS u-rev — u-rev 6 — HAS NOT BEEN
REVIEWED**, and u-rev 5's review does not transfer to it: an amendment reopens
the review, however small the diff (D-311, and CLAUDE.md's own words).

**THE ROWS BELOW SAY WHERE EACH FINDING IS ANSWERED AND DO NOT RESTATE WHAT IT
SAID (D-331).** The report is in the tree; a reader who wants a finding's content
reads it there, where it has its home. The dispositions of the u-rev 3 round are
not repeated here either — `U3-A` carries the one-line record of every round, and
this block is about the round that is being answered.

| Finding, `wp15b_U3_REVIEW_urev5.md` | Where it is answered at u-rev 6 |
|---|---|
| **MAJOR C** — the head's *"every cross-unit citation in this unit now reads `(u-rev N, landed <sha>)`"* was false at nine sites, the third completeness claim in this document falsified in the commit that wrote it | **ANSWERED BY THE REVIEWER'S OWN RECOMMENDED FIX (a), PLUS THE ONE THING (a) DOES NOT DO.** All nine sites are converted, so the debt three rounds had scoped out is gone rather than re-disclosed. **AND THE UNIVERSAL IS REPLACED BY A RULE AND A DERIVED COMMAND** — see the paragraph beside D-311's label discipline above. Converting the nine makes today's universal true; deriving it is what stops the next append making it false again, which is what (a) alone would not have done |

**WHY THE UNIVERSAL WAS THE DEFECT AND NOT ONLY THE NINE SITES.** The reviewer
offered two fixes: convert the nine, or re-scope the claim honestly as u-rev 4's
disposition table had. **Both are half-measures on their own.** Re-scoping keeps a
hand-maintained count of inherited debt, which is the shape that went stale in U2
twice and in this unit's B7 table three times. Converting alone leaves an asserted
universal that the next citation added without the form falsifies silently. The
answer taken is: convert all nine, and make the universal derivable, so the claim
has an instrument instead of an author.

**AND THIS IS THE THIRD RECURRENCE, WHICH IS RECORDED RATHER THAN ABSORBED.** MAJOR
A, MAJOR B and MAJOR C are one class — *a completeness claim false in the commit that
writes it* — and MAJOR C was written by the repair answering MAJOR A and MAJOR B,
under D-331, by a session that had just landed D-331. **D-331 forbids restating a
claim whose home is elsewhere; it does not forbid asserting a universal about this
document's own state, which is what all three of these were.** That gap is a finding
for the architect and is in the OPEN list; it is not something this unit may settle.

**THE u-rev 4 ROUND'S OWN DISPOSITION TABLE IS REMOVED AT u-rev 5.** It restated
the content of seven findings of `docs/experiments/wp15b_U3_REVIEW_urev3.md`
alongside what was done about each — and one of its rows is where MAJOR B's false
claim was written. Under D-331 that report is the home of its own findings and
`U3-A` is the home of the round record; a third copy here is the surface that
manufactured the defect. **Two dispositions from that round are NOT closed and so
are not removed with the table** — they are carried below as live items, not as
history:

- **Revision-7 review MAJOR 12, the unmarked `23.2` — STILL OPEN.** Its
  ATTRIBUTION was repaired at u-rev 4 from one candidate cell that does not
  reproduce to three that are named and left undecided; **deciding between them is
  a design act and has not been taken.** The reason is in U3-Z, where it lives.
- **Revision-7 review MAJOR 9, rule 5 undischarged — AN IMPL GATE, not discharged
  by this unit.** The disposition rests on **U2** (u-rev 4, landed `7dfd047`),
  which carries the same item OPEN for the node protocol itself, and on U3-M item
  4's declared hotspot substitution under D-263's flip clause. `wp15b_U4_REVIEW.md`'s
  own sentence about it is scoped to that report and is not a project ruling.

Theory citations are calculus IDs from `docs/research/threat_calculus_v1.md`
(D-266). This unit restates no theory; where it appears to, the calculus wins and
the disagreement is an ADR line.

---

## U3-A. Lineage — what has attacked this unit's content, and at which revision

| Round | Against | Verdict reaching M1 / §6 / §10 |
|---|---|---|
| DECISION-RED-TEAM, matrix M1 | revision 1, `ec8f7fb` | **M1 SURVIVES AMENDED.** The reviewer's own re-derivation, sharing no code with the census, disagreed on the READING — threshold against exact — and that disagreement is what killed revision 1's Tier-T option (§12 item 5) |
| REVIEW-design | revisions 2–6 | all FAIL; **M1 was never reopened on its merits.** §6.3's cost column was re-derived twice for transmission defects, and §6.2 became the instrument's output rather than a typed table |
| REVIEW-design | revision 7, `6feb40a` | **FAIL** — 7 BLOCKING, 7 MAJOR, 9 MINOR. **B5 and B7 are this unit's** and are repaired here. **MAJOR 12 is this unit's and is NOT repaired here** — it is a design question, recorded OPEN in U3-Z |
| DECISION-RED-TEAM, restructure | matrix at `eea480b` | F6: after M2's restoration unit 3 would have been the largest unit in option A's cut. Under D, M2 is not restored here at all |
| REVIEW-design, this unit | u-rev 2, `1b645ac` | **FAIL** — 2 BLOCKING, 5 MAJOR, 4 MINOR (`docs/experiments/wp15b_U3_REVIEW.md`). Both BLOCKINGs are D-305's class inside the carve that exists to stop it: B5 recurring at a site the carve itself wrote, and an instrument-clause discharge naming a test D-312 deleted. **Every finding was dispositioned finding-by-finding at u-rev 3, in the head's REVIEW STATUS block at the time (since superseded there by the u-rev 3 review below); u-rev 3 is the answer to it, and the u-rev 3 review confirmed nine of eleven repairs holding.** M1 itself is again NOT reopened on its merits — see the owed-list below |
| REVIEW-design, this unit | u-rev 3, `7d5d39c` | **FAIL** — 0 BLOCKING, 2 MAJOR, 3 MINOR (`docs/experiments/wp15b_U3_REVIEW_urev3.md`). u-rev 4 was the answer to it. M1 itself is again NOT reopened on its merits — see the owed-list below |
| REVIEW-design, this unit | u-rev 4, `6f2dfe6` | **FAIL** — 0 BLOCKING, 2 MAJOR, 0 MINOR (`docs/experiments/wp15b_U3_REVIEW_urev4.md`). **Both MAJORs were created by the u-rev 4 repair itself.** u-rev 5 was the answer to it. M1 itself is again NOT reopened on its merits — see the owed-list below |
| REVIEW-design, this unit | u-rev 5, `7473a6f` | **FAIL** — 0 BLOCKING, 1 MAJOR, 0 MINOR (`docs/experiments/wp15b_U3_REVIEW_urev5.md`). **THE THIRD CONSECUTIVE ROUND WHOSE MAJOR WAS MANUFACTURED BY THE PREVIOUS ROUND'S REPAIR**, and all three are one class: a completeness claim about this document's own state, false in the commit that wrote it. The B7 site table's completeness claim (MAJOR A), the cross-unit u-rev correction (MAJOR B), and the citation-form universal that answered MAJOR B (MAJOR C). u-rev 6 is the answer, and it is the first of the three to replace an asserted universal with a DERIVED one. The reviewer confirmed the B7 sweep clean at u-rev 5, breaking that half of the pattern |

**What this unit owes that no round has given it:**

- a REVIEW-design of THIS text at THIS u-rev — u-rev 5's ran, FAILED, and does
  not transfer to u-rev 6;
- **a fresh DECISION-RED-TEAM against MATRIX M1 AS AMENDED (MAJOR 6).** The
  attack in row 1 ran against revision 1 at `ec8f7fb`. §6.1 has since flipped the
  ADOPTED reading from exact to threshold — changing what the config commits and
  re-deriving every option row — and §6.5 SELECTED C under the new reading, which
  postdates the attack. Row 2 records that M1 "was never reopened on its merits"
  across revisions 2–6. U3-Z item 2's ADR line is GATED on this attack, because
  CLAUDE.md requires that line to record the strongest attack surviving against
  the option AS ADOPTED;
- and the census's registered replication and second instrument (U3-Z, OPEN).

---

## 6. MATRIX M1 — Tier-T qualification — SURVIVES AMENDED

### 6.1 The reading, corrected

Revision 1's §10 said `tier_t_own_count = 2` was "mapped to the closed
`LiveCount`", whose `LiveTwo` is `own == 2`. Its §6.1 table was produced by a
census that unioned `LiveTwo ∪ LiveThree`. Re-derived independently over the same
24 corpus roots:

The census block renders both readings as adjacent rows — `option B — Tier T
(threshold, ADOPTED)` against `option B — Tier T (exact, NOT adopted)` — and
revision 1 printed the threshold figure while §10 committed the exact one.

**The option committed was not the option measured.** An implementer following
§10 literally would have shipped a generator the matrix never evaluated — and
under it the reviewer constructed a position where the mover has a forced win in
two own turns that option A finds and exact-C does not, with the pre-registered
fallback to B repairing nothing because exact-B has the same own half.

**ADOPTED: the THRESHOLD reading.** `tier_t_own_count = 2` means own windows at
count **≥ 2**; `tier_t_opponent_count = 3` means **≥ 3**. **MEASURED** cost of
the repair: **+0.17** cells/node for B, **+0.04** for C at corpus roots. There is
no cost argument for the exact spelling, and under the threshold reading B ⊇ C,
so the pre-registered fallback is coherent for the first time.

### 6.2 The measurements, with their sampling regime

**MEASURED** at `f317385`, release, by the census harness
`crates/pistol-solver/tests/wp15b_census.rs`, **committed at `7941775`** rather than deleting with its worktree — CLAUDE.md's
instrument clause, and D-287's rule that an artefact recording numbers is not
test-tree-only.

Three regimes. The middle one is **re-sampled** in revision 2: revision 1 deepened
by uniform draws from the radius-**8** legal ball while the policy is radius **2**,
which inflated the ball 78.0 → 123.7 by the sampler rather than by depth.

<!-- BEGIN CENSUS TABLE — rendered by crates/pistol-solver/tests/wp15b_census.rs -->
| quantity | corpus roots | +1..3 turns, r2 draw (REPORTED) | +1..3 turns, r8 draw (SUPERSEDED) | playouts |
|---|---|---|---|---|
| own hot, mean | 0.0417 | 0.3559 | 0.3299 | 0.0833 |
| opponent hot, mean | 0.4583 | 0.2951 | 0.2101 | 0.0958 |
| live-2 own | 7.2083 | 11.1771 | 11.0694 | 23.7792 |
| live-2 opponent | 12.1667 | 12.4497 | 10.8976 | 25.4302 |
| live-3 own | 0.7500 | 1.7760 | 1.6059 | 1.7063 |
| live-3 opponent | 1.8750 | 1.8733 | 1.4253 | 1.8698 |
| radius-2 ball | 77.9583 | 94.4965 | 123.6615 | 376.4708 |
| cover union when FILTERED | 2.1667 | 2.1698 | 2.1899 | 2.2667 |
| WIN-NOW row | 4.2 % | 23.3 % | 21.7 % | 4.4 % |
| FILTERED row (`Cover::Minimal`) | 25.0 % | 18.4 % | 13.7 % | 3.1 % |
| `Cover::Impossible` | 4.2 % | 1.4 % | 1.2 % | 1.7 % |
| BATCHED nodes | 70.8 % | 61.5 % | 65.5 % | 92.5 % |
| option A — Tier T (threshold, ADOPTED) | 6.1250 | 8.2448 | 7.0382 | 6.6510 |
| option A — Tier T (exact, NOT adopted) | 6.1250 | 8.2205 | 7.0330 | 6.6510 |
| option A — staged, BATCHED only | 21.65 = 3.80x | 23.20 = 4.27x | 21.92 = 5.80x | 21.44 = 17.00x |
| option A — Tier T outside the r2 ball | 1.1250 | 1.0069 | 0.9236 | 0.0167 |
| option B — Tier T (threshold, ADOPTED) | 46.5000 | 54.6250 | 51.6649 | 88.1271 |
| option B — Tier T (exact, NOT adopted) | 46.3333 | 54.3854 | 51.4288 | 87.8708 |
| option B — staged, BATCHED only | 62.82 = 1.31x | 70.36 = 1.41x | 66.77 = 1.90x | 98.17 = 3.71x |
| option B — Tier T outside the r2 ball | 14.8750 | 14.5747 | 12.5851 | 6.1323 |
| option C — Tier T (threshold, ADOPTED) | 23.2917 | 31.4965 | 30.2622 | 48.7344 |
| option C — Tier T (exact, NOT adopted) | 23.2500 | 31.3194 | 30.0938 | 48.5812 |
| option C — staged, BATCHED only | 37.82 = 2.17x | 47.34 = 2.09x | 45.82 = 2.78x | 60.82 = 5.99x |
| option C — Tier T outside the r2 ball | 6.8333 | 7.4549 | 6.9392 | 2.9740 |
<!-- END CENSUS TABLE -->

**This block is the instrument's output and is not typed by hand.** It is
rendered by `crates/pistol-solver/tests/wp15b_census.rs` and pinned by
`the_carved_design_units_carry_this_censuss_table_verbatim`, which fails the
build if the two drift, and which reads — **MEASURED**, `CARVE_DOCS` in that file
— the **SIX** documents of the carve by an enumerated path list rather than by
one hard-coded path (travelling item T4'): the four units, `WPQ_seed.md` **and
`docs/experiments/section_owner_table.md`**, which u-rev 2's description here
omitted (MINOR 11). D-312 landed two companion gates beside it, which u-rev 2
also did not name: `the_census_pin_reads_every_carved_document_it_names`, which
plants a census figure in one document at a time and requires the scan to name
that document, so a file whose bytes never reach the scan fails rather than
passing silently; and `the_pins_document_list_is_the_set_of_carved_documents_on_disk`,
which compares the path list against the set of files on disk carrying
`CARVE_MARKER` — a referent the constant does not share, and the answer to "the
list is not self-certifying".

**What the pin refuses, stated at the strength it actually has.** No FOUR-DECIMAL
figure from the block is restated anywhere outside it, in any carved unit or in
the seed, and the pin fails the build if one is. **That is narrower than "no
section restates a number from it" — and the superseded §6.2 made the wider claim
while the document contained SIX counter-examples to it, at five distinct lines**
(revision-7 review B7, whose own heading says six, as does D-309; u-rev 2 said
"four" here and so undercounted the finding it repairs — MINOR 8). **MEASURED**
at `6feb40a`, pinned block at lines 797–824: `grep -n "70\.8\|6\.83\|23\.2"`
outside the block returns lines 139, 584, 853 (twice on that line), 1260 and
1442. The wider claim is not made here.

**And what the pin cannot see is a CLASS, not a list (MAJOR 4):** *any rounded,
percentage or otherwise derived rendering of a census cell*. `70.8 %`, `6.83` and
`23.2` are the three shapes u-rev 2 named here, and further sites inside THIS unit
carry renderings none of those three covers. **THE SITES HAVE ONE HOME AND IT IS
THE TABLE IN U3-Z (D-331); THIS PARAGRAPH POINTS AT IT AND STATES NO COUNT.** It
counted four, and was short by one; it counted five, and was short by the one
§6.5's own MAJOR-2 repair created in the same commit
(`wp15b_U3_REVIEW_urev4.md`, MAJOR A). A count restated one section away from the
list it counts is stale as soon as the list grows, and the list has grown in each
of the last three rounds. **The class is what cannot go stale when a site is
added**; everything else cites the block.

Why it exists as a mechanism rather than a resolution: across four revisions this
document moved a number in one section nine times and left a copy of it in
another — §6.2 repaired while §6.3 was not, the instrument extended while its
registered SHA was not, the sampler figures replaced while §12 kept the withdrawn
ones. Writing the lesson down did not stop it. This is D-259's discipline, which
the project already applies to a derived fixture, applied to a design table for
the same reason: an edited number becomes a red test rather than a reviewer's
finding.

**How to read it.** The FILTERED and BATCHED rows are separate because **U2** (u-rev 4, landed `7dfd047`) §5.3 emits
different sets on them: a filtered node emits the cover union alone, a batched one
emits Tier T plus the quiet cut. `quiet_top_k` and `widen_schedule` govern only
the batched population, which is why the staged rows report it rather than a
blended mean — a blend flattered option B by half.

### 6.3 The options

| Option | Theory standing | Cost | Failure modes |
|---|---|---|---|
| A — count ≥3 both sides | **No completeness licence.** `LAW-SUPPORT` k=2 licences windows at ≥2, and T10 adds that a window made hot this turn held ≥2 before — so count 3 misses every plan a PAIR creates from a count-2 window, which is the two-stone move this game is about | The largest reduction of the three — see the census block's `option A` rows | Provably k=2-incomplete. The reviewer built the position: P1 (0,0)(1,0)(2,1)(1,2)(0,3), pair {(2,0),(3,0)}, `t = 4`, `(2,0)` in own count-2 windows only |
| B — count ≥2 both sides | Full licence both sides | The smallest reduction of the three — see the census block's `option B` rows, whose BATCHED figure is the one `quiet_top_k` governs | Its opponent half buys the least, per §6.4's lemma |
| **C — ≥2 for us, ≥3 for them** | The lemma in §6.4 | see the census block; **MEASURED 29 % of C's Tier T lies OUTSIDE the radius-2 ball** (6.83 cells/node at corpus roots) | Asymmetric, so argued in §6.4. Residual: no cells blocking an opponent count-2 window; left to Tier Q's delta ranking, which is a set of 23.2 cells/node against a quiet allowance of 16 |
| D — a config knob instead of a choice | — | — | Rejected as a matrix answer. The knob exists (§10); what the matrix decides is what the config COMMITS |

### 6.4 The asymmetry, re-grounded

Revision 1's ground was "a defence against the opponent's two-turn win is what
SEARCH DEPTH and the filter are for". That is falsified by **U4** (u-rev 7, landed `0f49c90`) §12
item 1's **MEASURED `depth_at_500ms` = 2 / 2 / 1**: the opponent's second turn is
depth 4, and the engine reaches 2. The sentence is deleted.

**Where that measurement lives, and how U4 treats it (MAJOR 5).** Until u-rev 3
this paragraph said "this document's own", which the carve made false — the
measurement went to **U4** and U3 carries no other occurrence of it — and that
phrase is exactly what licenses a reader of U3 alone not to look elsewhere. It is
**U4**'s, and **U4** (u-rev 7, landed `0f49c90`) §9 amendment 1 records that the triple sits BELOW the
snapshot record's own "excluded from every comparison" marker with a dead band of
about 2×, so it is CONTEXT and not that unit's registered quantity; D-310 records
the same demotion. The ground here uses the depth NUMBER — 2 against the 4 the
refuted sentence needs — which is not a difference inside that dead band; a
reviewer checking it must read it in **U4** and read it as context.

The replacement is the reviewer's **count-3-leg lemma**, marked as a DERIVATION
and not a measurement: every k=2 win through `LAW-OVERLOAD` requires at least one
own window at count **3**. If every leg came from count 2, each leg contains both
new stones; by `LEM-CROSS` two windows on distinct axes share at most one cell,
so all legs lie on one axis — a same-line four, `PAT-4IFF`, `t ≤ 2`, not an
overload. Hence ≥1 leg at count 3. The attacker must generate **all** legs of its
own fork, so its half needs count ≥2; the defender need only break **one** leg,
and every fork has a count-3 leg, which C's opponent half carries.

**Its gap is named:** the lemma covers the `t ≥ 3` route only, not the
`LAW-LEDGER` t=2 forcing chain (four → forced blocks → win), whose pre-emption is
exactly the opponent count-2 cell C omits and whose refutation needs depth 4.
Both of the reviewer's constructed positions exhibit the lemma; it is not
exhaustively enumerated.

**Also stated, because revision 1 implied more than the law gives:** `LAW-SUPPORT`
at k=3 requires ≥0 own stones, i.e. no licence for any option. The licence
discriminates only inside a two-own-turn horizon, which is a horizon the engine
currently searches at depth 2.

### 6.5 ADOPTED: C at the threshold reading

**Pre-registered consequence, fixed before any gate runs.** If the soundness
instrument (**U4** (u-rev 7, landed `0f49c90`) §8) shows C dropping a cell a proven tactic needs, C is replaced by
B — which under the threshold reading is strictly wider — and the exchange is an
amendment with its own review, never a threshold move. **And the branch revision
1 omitted:** if the instrument is GREEN while mutation M7 (**U4** (u-rev 7, landed `0f49c90`) §8.4; Tier T at ≥3 for the
mover — option A) also SURVIVES, then the instrument has demonstrated it cannot
tell A from C, C's entire ground is unmeasured, and that is recorded as such in
the results rather than read as a confirmation of C.

**STRONGEST SURVIVING ATTACK** (abridged for the ADR line; the reviewer's full
paragraph is in the round record): *the matrix's MEASURED Tier-T column was
produced by a census reading count ≥2 while its config clause spelled count ==2 —
the threshold reading against the exact one — so the option committed was not the option measured;
and the reduction it is bought with shrinks the moment the depth stand-in is
re-sampled from the radius-2 ball the search actually uses — see the census
block's own `option C — staged, BATCHED only` row, `45.82 = 2.78x` on the r8
draw against `47.34 = 2.09x` on the r2 draw, a one-second run the document did
not take at the time. Revision 1's own figures for this, `3.1× to 2.4×`, are
WITHDRAWN (the superseded §0 row 34, `6feb40a`) and are not restated here, which
is the rule the block exists to enforce (§12 item 5; MAJOR 2,
`wp15b_U3_REVIEW_urev3.md`).* Both halves are repaired in revision
2; what survives is that neither repair was found by the author.

---


---

## 7. What survives here of §7, and what does not

MATRIX M2 and the widening schedule are `WPQ_seed.md`'s. Two bullets of §7.2 are
**not** about the schedule — one scopes the whole `Staged` policy against
`Radius`, the other warns a reader of the SPRT verdict — and both are carried
here verbatim because a unit needs them and the seed is not reviewable.

- **THE CUT BINDS UNDER `CandidatePolicy::Staged` ONLY.** Stated because revision
  2 left it implied and an implementer would have had to invent it. Under
  `Radius` the candidate loop is byte-for-byte what ships today: no batching, no
  node protocol, no threat state (`Position::threats` is `None`). Three things
  depend on it — the D-209 golden transcripts are taken at
  `configs/gate_v0.toml`, which is `kind = "radius"`; `tools/determinism.sh`
  runs the same radius configs; and the SPRT's incumbent seat must be the
  committed engine, or the match measures two changes instead of one.
- **And the two SPRT seats therefore differ on a THIRD axis**, named here beside
  the other two (§10 withdrew one such claim already): not only in what they
  SELECT and in what they can SEE, but in SEARCH VALUE — the overload return and
  **U2** (u-rev 4, landed `7dfd047`) §5.3's licensed shortening of mate distances on lost positions both change what
  a node reports. A reader of the SPRT verdict must not read it as a pure
  generation experiment.

**The first of the two is what U2's
`a_radius_policy_search_is_byte_identical_to_the_committed_engine` watches**, and
it is why that test's claim has a reviewable home rather than a home in the seed.

---
## 10. The config shape

**FOUR** complete documents, `deny_unknown_fields`, no code-side default for
any value. **This is the one place the count is stated; **U2** (u-rev 5, landed
`f0ae14c`) §2.2 and U3-Q cite it and do not restate it** (B5, which found it stated
three different ways across four sites). *(This citation was UNBOLDED and carried no
u-rev from u-rev 2 until u-rev 7. It survived every sweep of the nine known bare sites
and was structurally invisible to the derived command u-rev 6 registered, whose
pattern required the bold markdown this one lacks —
`docs/experiments/wp15b_U3_REVIEW_urev6.md`, MAJOR D.)*

| document | mode | `quiet_radius` | `quiet_top_k` | `widen_schedule` | why |
|---|---|---|---|---|---|
| `configs/instrument_staged_v0.toml` | instrument | 2 | 16 | `[32]` | **the SPRT seat and the snapshot's AFTER.** The cut BINDS here, because a seat with the cut disabled would make the SPRT measure nothing about the prune (rule 6, `WPQ_seed.md` §7.2) |
| `configs/tactical_staged_v0.toml` | instrument | 2 | **1024** | `[2048]` | **NEW in revision 7.** The 15 `instrument_v0` tactical cases. The cut is DISABLED, which is what **U4** (u-rev 7, landed `0f49c90`) §8.3's TACTICAL SUITE gate derivation requires and what revision 6 asserted while committing `quiet_top_k = 16` for these cases |
| `configs/gate_staged_v0.toml` | instrument | 1 | **128** | `[256]` | the five `depth_turns 3` cases, at radius 1. Cut disabled — MEASURED balls 22/22/22/18/15 at 11 stones, bounded by 6 × 17 = 102 three turns deeper |
| `configs/play_staged_v0.toml` | play | 3 | 16 | `[32]` | the movetime measurement, whose incumbent is `play_v0.toml` at radius 3. Cut binds |

**The fourth document exists because three could not carry the requirement.**
Revision 6's §8.3 TACTICAL SUITE gate — the SUPERSEDED document at `6feb40a`, NOT **U4** as it now stands — said "all three staged tactical configs disable the quiet cut" while §10
committed `quiet_top_k = 16` for two of them and §15 said "the two gate configs" —
three statements of one rule, none agreeing. The tension is real and needs a
document rather than a sentence: `instrument_staged_v0.toml` cannot be both the
tactical config (cut off, so `require 20`'s derivation holds) and the SPRT seat
(cut on, or the match measures nothing about the prune). `tactical_staged_v0.toml`
is that fourth document. The `1024` is not a guess: the radius-2 ball is MEASURED
in the census block's own `radius-2 ball` row, whose largest regime mean is under
400, and a bounded ball at 17 stones cannot exceed `6 × 17 = 102` at radius 1 or
`18 × 17 = 306` at radius 2.

Every other key is identical to the radius document it is the counterpart of, so
each is complete under rule 1 without restating the whole schema here; revision 4
promised "three complete documents" and printed the policy block of one.

**`widen_schedule` is defined against `quiet_top_k`, in QUIET CELLS, and both
ends are named.** Revision 2 left four questions an implementer would have had to
answer by invention.

**WHAT THE FIVE BULLETS BELOW ARE, AND ARE NOT — MAJOR 7.** They **RECORD** the
semantics `WPQ_seed.md` carries for a stage D-310 DEFERS and D-315 schedules into
WP-1.5c. **They do not state settled design, and nothing may be adopted from
them.** The seed's own header says nothing in it is selected and nothing in it may
be cited as adopted, and D-315 records W-E — naming the non-PV cut as a forward
prune — as never having been attacked as an option among options. Every citation
in this section that reaches into the seed rests on that unselected text: these
bullets, the `instrument_staged_v0.toml` row's `WPQ_seed.md` §7.2 clause and the
TOML comment's `WPQ_seed.md` §7 clause, both carried verbatim. u-rev 2 asserted
the bullets as settled ("Correct, and now stated"); at u-rev 3 that is scoped, not
withdrawn — the carve does not get to decide them either way. Moving the semantics
into a unit where they CAN be attacked is a design act belonging to whoever
decides the D-scope (U3-Z, OPEN).

- The **first** batch is `quiet_top_k` quiet cells. Tier F and Tier T are always
  emitted whole and are not counted against it (**U2** (u-rev 4, landed `7dfd047`) §5.4).
- The schedule's entries are **cumulative counts of QUIET cells**, not indices
  into the whole vector.
- A pool **shorter** than the first boundary never truncates, so the node is not
  counted in the widening schedule's registered denominator (`WPQ_seed.md`
  §7.2). Correct, and now stated.
- A pool **longer** than the last boundary is cut there permanently. That is what
  a finite last entry is FOR, and it is the forward prune **the ADR line the seed
  records as OWED** would name (`WPQ_seed.md`, item 3, which that file records as
  unwritable while M2 is an open selection — so there is no such line to name in
  the present tense, as u-rev 2 did).
- Cross-field validation, which revision 2's validator lacked: every entry must
  exceed `quiet_top_k`. `quiet_top_k = 64` with `widen_schedule = [32]` passes
  revision 2's "non-empty and strictly increasing" and describes a widening that
  NARROWS — a named refusal under rules 1 and 3.

`schema_version` stays **2**: adding a `kind` to a tagged enum leaves every
existing document valid, and D-16's bump is for a change that invalidates one.
Recorded rather than left silent.

```toml
[search.candidate_policy]
kind = "staged"
quiet_radius = 2
quiet_top_k = 16
# Batch boundaries after the first. The LAST ENTRY IS FINITE: "all remaining"
# is what makes a widening schedule a rename of full width (WPQ_seed.md §7).
widen_schedule = [32]
# LAW-SUPPORT qualification, THRESHOLD reading: >= 2 for the mover, >= 3 for
# the opponent (§6).
tier_t_own_count = 2
tier_t_opponent_count = 3
```

Validation, in `pistol-engine`'s validator and again in `Searcher::new` (a
`SearchParams` can be built in code and never passes through a document):
`quiet_radius` in `1..=MAX_CANDIDATE_RADIUS` and representable as `i16`;
`quiet_top_k >= 1`; `widen_schedule` non-empty, strictly increasing, **every entry
greater than `quiet_top_k`**, and **no sentinel admitted**; `tier_t_own_count` and `tier_t_opponent_count` in `{2, 3}`; and **every
`widen_schedule` entry strictly greater than `quiet_top_k`**, which revision 3's
validator did not check — `quiet_top_k = 64` with `[32]` passed "non-empty and
strictly increasing" and described a widening that NARROWS. The cross-field rule
is the last bullet's and carries the same status the lead-in gives it: it
validates two keys whose D-scope is OPEN, against semantics that are the seed's
unselected text (MAJOR 7, U3-Z).

**And the threshold is NOT "over `LiveCount`", which cannot express it.**
`LiveCount` is closed at `{Two, Three}` (D-255, a compile error otherwise), so it
cannot name `>= 4`; the `>= 4` windows are `hot_windows`, a different set. A count
of `n` therefore means the UNION:

```
n = 2  ->  live_cells_at_count(side, Two) ∪ live_cells_at_count(side, Three) ∪ threat_cells(side)
n = 3  ->                                   live_cells_at_count(side, Three) ∪ threat_cells(side)
```

Reachable, not pedantic: at `Phase::Second` with an own hot-4 window and no
win-in-one-ply, `can_win_this_turn` is `None`, the node takes a BATCHED row, and
that window's empties are in Tier T under the union reading and absent under the
`LiveCount`-only one. Revision 3 said "threshold over `LiveCount`" in §10 and
spelled the EXACT-count union in the test plan's referent (U3-T) — two different sets in one
document, which is **the superseded §0**'s row 4 class recurring in the opposite
direction — §0 is DROPPED by the carve, is owned by no unit, and is retrievable
only at `6feb40a` (MINOR 10; the header's `§n` resolution rule prefixes the OWNING
unit and there is none here, so it is said in words, as with "the superseded §11"
and "the superseded §15" below). The
committed census implements the union reading, so §6.3's numbers are the union's.

`instrument_r2_v0.toml` is value-identical to the committed `instrument_v0.toml`
(D-194) and is the SPRT's incumbent seat.

**Revision 1's config comment is withdrawn.** It read "`quiet_radius = 2` so …
the SPRT's two seats differ in what they SELECT rather than in what they can
SEE". MEASURED, **29 % of option C's Tier T lies outside the radius-2 ball**
(6.83 cells/node at corpus roots), so the seats also differ in what they can see.
The comment now says so.

---


---

## U3-T. The tests this unit registers

Carried from the superseded §11. The rows this unit does not own are in U2-T,
U4-T and `WPQ_seed.md`, and no row is in two places.

| Test | Watches |
|---|---|
| `tier_t_qualification_matches_adopted_matrix_option` | the Tier T set against an independent **`us@{2,3} ∪ threat_cells(us) ∪ them@{3} ∪ threat_cells(them)`** — the UNION reading §10 establishes, since `LiveCount` cannot express ≥ 4. On a position where exact-2 and ≥2 DIFFER, and one where the `LiveCount`-only and union readings differ. Revision 4 spelled the `LiveCount`-only referent here while §10 corrected it two sections earlier |
| `the_fallback_under_staged_answers_from_the_quiet_radius_ball` | the turn `fallback_turn` returns under a Staged policy, and that it reads no threat state — the bounded, pure property WP-1.4's movetime ceiling rests on |
| `no_candidates_under_staged_is_refused_by_a_policy_agnostic_error` | the error variant at a root the policy cannot serve; `SearchError::NoCandidates { turn, radius }` names a `radius` a Staged policy has three of |
| `tier_t_cells_match_an_independent_window_walk` | the emitted Tier T against a from-scratch enumeration, on a position where the `LiveCount`-only reading and the union reading DIFFER |

---

## U3-M. What this unit measures

ADVISORY on this machine; the operator re-runs for the record. (A standing
condition of every measurement in every unit, stated per unit so a unit is
readable alone; it is a condition, not a datum.)

4. **D-263's registered hotspot — the bracket is recomputed, and the pre-registered
   hotspot turns out not to be the dominant one.** **MEASURED**, release:
   `blocking_covers` 246 / 71 / 69 ns mean (max 1513 / 1252 / 2665 ns),
   `unblockable_double_threat` 101 / 50 / 49 ns; a deliberately built family of 16
   disjoint hot windows costs 1479 ns/call, and the maximum hot count observed
   anywhere is **5**.

   **The bracket, corrected.** Revision 2 printed "0.6 %–3.7 % of a node" and
   declined all three of D-263's remedies on it; the bracket omitted its own worst
   cell. Per-regime sums are 347 / 121 / 118 ns against node times of 21 277 ns
   (47 knps) and 3 300 ns (303 knps), so the true ceiling is **10.51 %** — about
   3× what was printed — and one worst-case call is **81 %** of a fast node.

   **The remedies stay unimplemented, and revision 3's adoption of the first one is
   WITHDRAWN.** Revision 3 implemented the three-pairwise-disjoint-families
   early-out. Two measurements retire it: it needs three families and **1 of 24**
   corpus roots has them (17 roots have none, five have one, one has two); and it
   accelerates `min_hitting_set_exceeds`, which under M5-E (**U2** (u-rev 4, landed `7dfd047`) §5.2) is **no longer
   called per node at all**. M5-E delivers **−29.1 % / −41.3 % / −41.5 %** of the
   registered per-node threat cost by deleting the redundant query — a larger cut
   in the same hotspot than any remedy D-263 names, and it needs no new code in
   `pistol-solver`.

   **THE REAL HOTSPOT IS TIER-T EXTRACTION, and it is registered here BEFORE the
   change that touches it**, which is what rule 5 asks. **MEASURED** on one
   harness over the 24 corpus roots (see the population caveat below): extracting
   Tier T's cells costs about **6×**
   both threat queries combined (533 ns with a reused buffer, 662 ns fresh,
   against 86 ns for the pair on the same harness). D-263 named the cover
   arithmetic and the measurement says otherwise — which is a pre-registration
   doing its job, not failing it. **Registered rule-5-shaped**, which revision 4's registration was not — it named a
   mechanism where a bracket belongs. HOTSPOT: Tier-T cell extraction on the
   per-node path. EXPECTED GAIN BRACKET: the honest answer is that **no bracket can be
   derived before the IMPL measures it**, and revision 5's `[1.10×, 1.35×]` was
   anchored on the wrong comparison — 662/533 = 1.24× is the cost of NOT reusing a
   buffer, a saving the search gets free with one scratch `Vec` on `Run` and
   without any accessor, since `query.rs`'s cell queries already fill a
   caller-supplied `&mut Vec<Coord>`. So the accessor's own gain is the per-window
   public-boundary crossing alone, which nothing has measured. The registration is
   therefore: **BASELINE = the in-search mask walk with a reused buffer, MEASURED
   first, in its own commit**; the accessor is then a second commit whose bracket
   is set from that baseline before it is written. ABORT THRESHOLD: below 1.05×, or any
   regression in whole-search nps. INSTRUMENT: one IQR-gated bench reporting
   **nps AND time-to-depth**, per rule 5, not the snapshot — which reports
   `depth_turns` and `nodes` only. ONE CHANGE = ONE COMMIT.
   **And the number is re-taken on the right population**: 533/662 ns were
   measured over all 24 corpus roots, but **U2** (u-rev 4, landed `7dfd047`) §5.3 does not extract Tier T on the
   **29.2 %** of them that take a forced row, so the registered figure is a
   blended mean over two populations — the same mixture defect §6.3 was corrected
   for. The IMPL re-takes it on BATCHED nodes only.

   **The surface gap behind it.** Tier T needs the empty cells of live-2 and live-3
   windows, and after D-261 `pistol-solver` offers **no convenience accessor** for
   them — `live_windows_at_count`, `masks()` and `Window::cells()` are all public
   and are the route the committed census takes, so the claim is about ergonomics
   and per-node cost rather than reachability:
   `threat_cells` covers hot only, `cells_raising_to_hot` is closed at
   `NearHot::Three`, and `empty_cells` is crate-private. The committed census had
   to walk `masks()` bits against `Window::cells()` per window, which is exactly
   what the search would have to do per node. D-261's flip clause — "Flips when a
   consumer outside this crate needs one of these names — additive, one `pub use`
   each. WP-1.9's instrument is the nearest candidate and is NOT one" — names this
   WP as that consumer, and item 16 (U3-Z) takes the line.


5. **The census is `crates/pistol-solver/tests/wp15b_census.rs`, coupled to the
   CARVED DOCUMENTS — all six of them, not this one — by a TEST rather than by a
   SHA**, and `tools/baseline_snapshot.sh` is at **`e889b5b`**.

   The instrument clause asks that a change to the instrument reopen the review.
   A recorded SHA does that only if someone re-reads it: revision 5 named the
   census at `7941775`, a revision emitting THREE regimes, while a whole column of
   its own tables came from a fourth added in the same commit as the document —
   the SHA went stale in the commit that wrote it.
   **`the_carved_design_units_carry_this_censuss_table_verbatim`** does it
   mechanically instead: change the instrument and the build fails until the
   block is re-rendered — and the gate is at the CARVE's scope, not one path's.
   It reads the six documents `CARVE_DOCS` names (the four units, `WPQ_seed.md`
   and `section_owner_table.md`), it requires exactly ONE `BEGIN`/`END` pair
   across the whole set, and it fails on any FOUR-DECIMAL figure from the block
   restated outside it in any of them. D-312 landed two companion gates with it:
   `the_census_pin_reads_every_carved_document_it_names`, which plants a census
   figure in one document at a time and requires the scan to name that document,
   so a file the scan cannot see fails here rather than passing silently there;
   and `the_pins_document_list_is_the_set_of_carved_documents_on_disk`, which
   compares the path list against the `CARVE_MARKER` set on disk — a referent the
   list does not share, and the answer to "the list is not self-certifying". That
   is a stronger discharge than a SHA, not a weaker one, and it is the same
   substitution D-284 made for this log's own integrity — a property nobody was
   checking became a gate.

   **BLOCKING 2, repaired at u-rev 3, and recorded because it is this WP's
   standing class.** Until u-rev 3 this paragraph named
   `the_design_document_carries_this_censuss_table_verbatim` — the ONE-PATH gate
   D-312 renamed — and kept its singular scope ("until *the document* is
   re-rendered"), which is the framing D-312 calls the defect. §6.2 had been
   repaired to the live name; this claim, resting on it and living in the same
   file, was not re-read. The command that would have "verified" the old name
   exits 0 while running nothing, which is `tools/SHELL_CHECKLIST.md`'s
   EXIT-0-WRONG-ANSWER: a claim that a test discharges something must name a test
   that RUNS. So it is verified here, **MEASURED at u-rev 3** — the gate's own
   output, not a wrapper's exit status:

   ```
   $ grep -n "fn the_carved_design_units_carry\|fn the_census_pin_reads_every\|fn the_pins_document_list" crates/pistol-solver/tests/wp15b_census.rs
   738:fn the_carved_design_units_carry_this_censuss_table_verbatim() {
   799:fn the_census_pin_reads_every_carved_document_it_names() {
   851:fn the_pins_document_list_is_the_set_of_carved_documents_on_disk() {

   $ cargo test -p pistol-solver --test wp15b_census
   running 5 tests
   test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
   test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... ok
   test wp15b_census_reproduces_the_registered_populations ... ok
   test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
   test the_census_pin_reads_every_carved_document_it_names ... ok

   test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
   ```

   The residual, stated: the pin covers the TABLE. Prose claims about the census —
   its sampling regimes, what a column means — remain judged, and so does
   `tools/baseline_snapshot.sh`, whose output this document does not carry; and so
   does every rounded, percentage or otherwise derived rendering of a cell, which
   is a CLASS the four-decimal scan cannot see (§6.2, and U3-Z, whose table is a
   finding aid for the class and not its boundary).

   **The second-instrument framing of revision 2 is WITHDRAWN, and what actually
   happened is recorded instead.** Revision 2 registered "the two regimes must
   agree on the RANKING of options A, B and C by staged-set size". Under the
   adopted threshold reading `own≥3 ⊆ own≥2` and `them≥3 ⊆ them≥2`, so
   **A ⊆ C ⊆ B as SETS** and `|A| ≤ |C| ≤ |B|` holds in every position under every
   sampler — verified over all 24 corpus roots, strict on 23 and 24 of them. The
   ranking is a set-inclusion identity, so the criterion could not have been
   falsified by the sampler defect it named, nor by anything else. That is the
   vacuity CLAUDE.md forbids, registered as the answer to a doubt.

   **The real second instrument already ran, and it already disagreed.** The
   fresh-context DECISION-RED-TEAM on M1 independently re-derived §6.2's
   population columns from the same corpus, sharing no code with the census. Its
   agreement was exact on every population number — and its DISAGREEMENT was
   the exact reading against the threshold one, which is what exposed that defect that killed revision 1's Tier-T option. The stage
   under doubt was the census's READING of `tier_t_own_count`, the two instruments
   did not share it, and the consequence was that M1 reopened. Nothing needs
   registering after the fact; what needs recording is that the criterion which
   worked was independent re-derivation by a fresh context, and the criterion this
   document invented was an identity.

   **What is registered forward**, for the deepening sampler: the radius-2 regime
   is the reported one, and the radius-8 regime's numbers are retained as
   SUPERSEDED with the delta stated — both regimes' figures are in the census
   block, and **§6.5's STRONGEST SURVIVING ATTACK is where the sensitivity is
   read from it**: it cites the block's own `option C — staged, BATCHED only` row
   rather than restating revision 1's WITHDRAWN `3.1× to 2.4×` pair, which is the
   rule the block exists to enforce. Until u-rev 4 §6.5 restated that withdrawn
   pair instead of following this rule (MAJOR 2, `wp15b_U3_REVIEW_urev3.md`); it
   now carries the rule rather than breaking it. No verdict in this document
   turns on a quantity that moves by less than the sampler does between those two
   columns.

---

### Cost

| Item | DECLARED | MEASURED |
|---|---|---|
| The census harness | ~1 min | **< 1 s** per run after a 1.3 s build |

The census is the reason CLAUDE.md's proportionality clause bites here rather than
being argued around: a run measured in seconds is answered by REPLICATION and a
SECOND INSTRUMENT, never by a margin. Neither is registered — U3-Z.

---

## U3-Q. The conservative branches this unit records

- **The committed configs do not move.** `instrument_v0.toml` and `play_v0.toml`
  stay at `kind = "radius"`; Staged ships as the selectable documents §10 lists,
  and §10 is the one place their number is stated (B5). The SPRT is the judge and
  it is the operator's run — D-190/D-194's own order.
- **D-263's three remedies stay unimplemented**, on a corrected bracket and a
  measured firing rate — and the measurement says D-263 named the wrong hotspot
  (§12 item 4).
- **The census is WP-1.6's to extend**: it renders population figures for any
  regime added to it, and the pin makes a unit that cites them unable to drift.

---

## U3-Z. ADR lines this unit owes, and what is OPEN

### ADR lines

Carried from the superseded §15. Its item numbers are retained exactly so an
existing cross-reference to "§15 item n" still resolves; this unit invents none
and renumbers none. The superseded §15's preamble does not travel (MAJOR 10
measured it false on both clauses); this is U3's lead-in instead: **items 2, 7
and 16 are this unit's own and have not landed; items 12 and 23 are corrections
to LANDED lines and have themselves LANDED, which the superseded list said of
only one of them** (MAJOR 13).

2. Tier-T option C at the threshold reading, with §6.5's surviving attack and the
   count-3-leg lemma's two named gaps. **GATED — MAJOR 6: this line may not be
   written until a fresh DECISION-RED-TEAM has attacked M1 AS AMENDED.** CLAUDE.md
   requires the surviving option's ADR line to record the strongest attack
   surviving against the option **as adopted**; §6.5 records the attack against
   the option as it stood at revision 1, `ec8f7fb`, before §6.1 flipped the reading
   from exact to threshold and before C was selected under the flipped reading.

7. **D-263's three remedies stay unimplemented, and revision 3's adoption of the
   first is WITHDRAWN** — it needs three disjoint families and 1 of 24 corpus
   roots has them, and it accelerates a query M5-E deletes. The larger cut in the
   same hotspot comes from removing the duplicated query (−29 % to −42 %).
   **AND D-263 NAMED THE WRONG HOTSPOT**: Tier-T cell extraction is MEASURED at
   about 6× both threat queries combined. Registered here with its own bracket and
   abort threshold, before the change that touches it (rule 5).

12. **D-255 is wrong on a number it states.** It says "the corpus shows own-side
    hot = 0.0 mean / 0 max at both stone counts". The census block's `own hot,
    mean` row at corpus roots refutes it, at index 16 / 31 stones, which sits in
    the 35-band. **LANDED as D-301** at `68a28c8`.

16. **D-261 gains a query.** Tier T needs the empty cells of live-2 and live-3
    windows and the public surface has no route to them; D-261's flip clause names
    this WP as the consumer. `ThreatState::live_cells_at_count(side, LiveCount,
    &mut Vec<Coord>)`, additive, with the map entry D-267 requires of a new query
    naming its calculus ID (`LAW-SUPPORT`'s k=2 qualification), and the recorded
    coincidence that it equals `cells_raising_to_hot(side, NearHot::Three)` at
    count 3 while MEANING something different.

23. **The census stays in the test tree while recording numbers**, which is the
    case D-287's clause reserves for a future ADR ("promotion is a FUTURE ADR,
    owed the day anything records a number from it"). The line records why it does
    not move to `tools/`: `tools/` membership pulls in SHELL_CHECKLIST's coverage
    rule and the shell instrument rules, and this artefact is a Rust test driven by
    `cargo test` with its own pinning test — the coverage rule's intent, met by a
    different mechanism.
    **LANDED as D-304**

### OPEN — carried forward, not closed by the carve

- **A SELF-COMPLETENESS CLAIM IS A NEIGHBOURING CLASS TO THE ONE D-331 FORBIDS, AND
  D-331 NAMES IT AS GROUND WITHOUT BINDING IT. THAT GAP IS THE ARCHITECT'S.** This
  unit has now failed FOUR consecutive rounds on it: the B7 table's *"here is every
  site"* (MAJOR A), the *"U2 is at u-rev 2"* correction (MAJOR B), the *"every
  cross-unit citation now reads …"* universal that answered it (MAJOR C), and the
  derived command registered to replace THAT, which was blind to a tenth site
  (MAJOR D). **THE THREE ARE NOT ALL THE SAME THING, AND u-rev 6 SAID THEY WERE**
  (`wp15b_U3_REVIEW_urev6.md`, MAJOR E, which is what this bullet is being corrected
  by). **MAJOR B IS A TEXTBOOK D-331 RESTATEMENT**: U2's current u-rev has its home in
  U2's own head, and this unit held a second copy that went false — D-331's diagnosis
  in D-331's own words. **MAJOR A and MAJOR C are the neighbouring class**: assertions
  about a set the author had not enumerated, made at their own home, restating
  nothing. **AND D-331 IS NOT SILENT ABOUT ANY OF THEM** — u-rev 6 said it was, and
  that was wrong. D-331 cites `wp15b_U3_REVIEW_urev4.md` MAJOR A and MAJOR B BY NAME
  as grounding instances of the recurrence it rests on. What it does not do is state a
  RULE that a self-completeness claim owes a derivation, the way the Process section
  states that a matrix cell owes a command. **What this unit does about it is a remedy
  and not a rule:** it asserts no universal about its own citations, the B7 list is
  explicitly not a boundary, and the instrument it offers is labelled a finding aid
  rather than a proof — because u-rev 6 tried the derivation and the derivation was
  blind. **Whether a self-completeness claim owes an instrument, and what makes an
  instrument for one adequate, is a project-level question this unit may not settle.**

- **MAJOR 12 — the unmarked `23.2` in §6.3's option-C failure-mode cell.** It
  carries neither **MEASURED** nor **ESTIMATED**, on the cell that states the
  ADOPTED option's residual risk, and the review found it is not a census row at
  all: the block renders no Tier-Q quantity. **Its provenance is not settled to
  one cell (MAJOR 1, `wp15b_U3_REVIEW_urev3.md`):** `23.2` is the ROUNDING of the
  block's own `option C — Tier T (exact, NOT adopted)` row at corpus roots, the
  LITERAL `option A — staged, BATCHED only` figure at the r2-draw column (23.20),
  or the TRUNCATION — not the rounding, which is 23.3 — of `option C — Tier T
  (threshold, ADOPTED)` at corpus roots; all three are cited by name rather than
  restated in decimals, which is what the pin
  refuses, and under every one of them the figure is Tier T and not Tier Q. **The
  carve preserves marks and does not add them** — adding one would decide which
  of the three cells the figure comes from, which is a design act, not a carve
  act. Either the figure is wrong or it is an unmarked estimate with a committed
  instrument standing beside it, which is D-291's clause. **A repair here is a
  design act, not a carve act.**
- **B7's residual is a CLASS, AND THE CLASS IS THE CLAIM — THE TABLE BELOW IS A
  FINDING AID AND NOT A BOUNDARY (MAJOR 4).** The pin refuses FOUR-DECIMAL
  restatements only. What survives it is **any rounded, percentage or otherwise
  derived rendering of a census cell** — stated as a class because u-rev 2 stated
  it as an ENUMERATION ("four sites"), a reader takes an enumeration for the
  boundary of the question, and that enumeration was short by four sites inside
  this unit alone.

  **THIS BULLET NO LONGER CLAIMS THE TABLE IS COMPLETE, AND THE REASON IS THAT
  THE CLAIM HAS BEEN FALSE EVERY TIME IT WAS MADE.** Until u-rev 5 it read *"here
  is every site of it known at this u-rev"*. At u-rev 3 that was false by four
  sites; at u-rev 4 it was false on the day it was written, over a site that
  u-rev's OWN MAJOR-2 repair had just created in §6.5, two lines from where the
  new row belonged (`wp15b_U3_REVIEW_urev4.md`, MAJOR A). **A site absent from
  this table is still in the class**, and the test a reader applies is the class
  sentence above, not this list. That is not a weakening: it is the same thing
  this bullet already argued four paragraphs down — *"the CLASS statement, not the
  table, is what stops this list going stale when a site is added"* — finally
  stated where the table is, instead of being contradicted by a completeness
  claim in the same bullet (D-331).

  Each site below is
  **LISTED, not repaired**: deciding whether `70.8 %` is the same quantity as the
  block's `BATCHED nodes` row is a design question and not a transcription one, and
  the carve repairs transcription only.

  | site | rendering | census cell it derives from |
  |---|---|---|
  | **U2** (u-rev 4, landed `7dfd047`) §5.3 | `70.8 %` | `BATCHED nodes`, corpus roots |
  | **U4** (u-rev 7, landed `0f49c90`) §8.4 | `70.8 %` | `BATCHED nodes`, corpus roots |
  | **§6.5, the STRONGEST SURVIVING ATTACK — MAJOR A, `wp15b_U3_REVIEW_urev4.md`** | `2.78x`, `2.09x` | `option C — staged, BATCHED only`, r8 draw and r2 draw |
  | §6.3, option C's cost cell | `6.83` | `option C — Tier T outside the r2 ball` |
  | §6.3, option C's failure-mode cell | `23.2` | **NOT ONE CELL (MAJOR 1, `wp15b_U3_REVIEW_urev3.md`).** Reproduces from `option C — Tier T (exact, NOT adopted)` at corpus roots (rounds) and from `option A — staged, BATCHED only` at the r2-draw column (23.20, literal); `option C — Tier T (threshold, ADOPTED)` only TRUNCATES to it and does not round — see the MAJOR-12 bullet above |
  | §10, the withdrawn config comment | `6.83` | `option C — Tier T outside the r2 ball` |
  | **§6.2, the sampler sentence — u-rev 2 omitted it** | `78.0 → 123.7` | `radius-2 ball`, corpus roots and the r8 draw |
  | **§6.1, the threshold-repair cost — u-rev 2 omitted it** | `+0.17`, `+0.04` | `option B — Tier T` threshold minus exact; `option C — Tier T` threshold minus exact |
  | **§6.3 and §10, the outside-the-ball share — u-rev 2 omitted it** | `29 %` | `option C — Tier T outside the r2 ball` over `option C — Tier T (threshold, ADOPTED)` |
  | **§12 item 4, the forced-row share — u-rev 2 omitted it** | `29.2 %` | 100 minus `BATCHED nodes`; equally `WIN-NOW` plus `FILTERED` |
  | **§10, the `1024` tactical config derivation — MINOR 3, `wp15b_U3_REVIEW_urev3.md`** | `under 400` | `radius-2 ball`, playouts |

  The derivations reproduce — **MEASURED at u-rev 4, and the new §6.5 row's two
  values are read from the same `option C — staged, BATCHED only` row the census
  block prints** — from the block's own cells,
  and the values are read from the block rather than restated here because that is
  what the pin exists to enforce, **with one correction (MAJOR 1,
  `wp15b_U3_REVIEW_urev3.md`): at u-rev 3 this sentence covered a table that
  attributed `23.2` solely to `option C — Tier T (threshold, ADOPTED)`
  and stamped the whole row MEASURED, but that cell's four-decimal value has no
  rounding equal to 23.2 — it only TRUNCATES to it, and rounds to 23.3. The table above now
  cites the two cells that DO reproduce it exactly; deciding which of the three
  candidate cells is the true provenance remains MAJOR 12's open design act, and
  every candidate is a Tier-T quantity, so "which is Tier T and not Tier Q"
  stands regardless.** The first omitted site sits **two lines above the
  `BEGIN CENSUS TABLE` marker**, inside the very paragraph that repairs B7, which
  is how an enumeration goes stale. Widening the pin's scan past four decimals is
  an instrument change, not a re-target, and T4' does not license it — so the CLASS
  statement, not the table, is what stops this list going stale when a site is
  added. **MINOR 9, repaired:** u-rev 2 also named a `70.8 %` in **U4** (u-rev 7, landed `0f49c90`) §8.5.
  There is none — MEASURED, `grep -n "70\.8" docs/experiments/U4_soundness_instrument.md`
  returns one line, and it is inside §8.4; the §8.5 claim is deleted.
  **RE-TAKEN at u-rev 5 against U4** (u-rev 7, landed `0f49c90`): the same command
  still returns one line, still inside §8.4.
- **The census owes a registered REPLICATION and a SECOND INSTRUMENT** (the
  superseded §17's own list). It runs in under a second, so CLAUDE.md's clause
  gives it no room to argue: the agreement criterion is registered before either
  runs, it names the stage under doubt and how the second instrument does not
  share it, and it carries a registered consequence for disagreement. §12 item 5
  records that the criterion which WORKED was independent re-derivation by a
  fresh context and the one this document invented was a set-inclusion identity —
  that is a record of what happened, not a registration for next time.
- **The D-scope of `quiet_top_k` and `widen_schedule`.** **§10's config documents
  each commit both keys** — §10 is the one place their number is stated and this
  bullet cites §10 rather than restating it (**B5**; u-rev 2 asserted the
  cardinality here, which is the same defect as **U2** (u-rev 4, landed `7dfd047`) §2.2's "three" going stale when
  §10's list grew, and it was introduced by the repair, not inherited — BLOCKING
  1). D-310 defers the stage those keys govern. Whether the shipped
  `Staged` surface keeps them (validated, inert, and set wide), narrows to Tier F
  ∪ Tier T with no quiet tier at all, or something else, changes the config
  documents, the validator and the SPRT seat. **The carve does not choose.**

  **AND THE DECISION REACHES INTO M1 ITSELF, which u-rev 2's bullet did not say
  (MAJOR 3).** The list, so that a reader takes the whole question and not part of
  it:
  - **The ADOPTED option's own stated mitigation.** §6.3's option C answers its
    residual — no cells blocking an opponent count-2 window — by leaving them to
    **Tier Q's delta ranking against a quiet allowance of 16**. D-315 schedules
    Tier Q into WP-1.5c. Under this bullet's own second branch there is no quiet
    tier at all; then C's stated mitigation does not exist in the shipped engine
    and the cells C omits are generated by nothing. That moves the B-against-C
    comparison, not only the config documents.
  - **Option B's cost cell** points the reader at the census row "whose BATCHED
    figure is the one `quiet_top_k` governs".
  - **The matrix's cost evidence itself.** **MEASURED**,
    `crates/pistol-solver/tests/wp15b_census.rs`: it holds
    `const QUIET_TOP_K: usize = 16`, commented *"The design's committed quiet cut,
    so the staged-set column is the set the config actually produces"*, and each
    `option X — staged, BATCHED only` row is Tier T whole plus the quiet cells
    capped at that constant. **Every staged figure and every `= N.NNx` multiplier
    in the pinned block therefore has the deferred stage inside it**, and under
    D-310 the shipped engine's candidate count is not that figure.
  - **What survives, recorded because it errs toward the document:** the RANKING.
    §12 item 5 establishes `A ⊆ C ⊆ B` as a set-inclusion identity under the
    threshold reading, so the Tier-T rows order the options exactly as the staged
    rows do, and the SELECTION of C over A and B does not turn on the quiet cut.
    What does not survive the deferral is the MAGNITUDE of the reduction the
    matrix reports, and C's stated mitigation.
  - **This is also where MAJOR 12's scoping belongs.** The bullet above states the
    MARK question on `23.2`; the other half of that cell — "against a quiet
    allowance of 16" — is a deferred-stage quantity, so the D-scope decision moves
    it too.
- **M1 AS AMENDED HAS NEVER BEEN ATTACKED (MAJOR 6).** The DECISION-RED-TEAM in
  U3-A's first row ran against revision 1 at `ec8f7fb`. §6.1 has since flipped the
  ADOPTED reading from exact to threshold — which changed what the config commits
  and re-derived every option row — and §6.5 selected C under the flipped reading;
  U3-A's second row records that M1 "was never reopened on its merits" across
  revisions 2–6. CLAUDE.md: reviews of superseded revisions do not transfer, and a
  matrix never attacked is the same breach as silent architecture drift. A fresh
  DECISION-RED-TEAM against §6.3 as it now stands is therefore owed, item 2 above
  is GATED on it, and the carve does not run it. **U4** (u-rev 7, landed `0f49c90`) states the
  parallel duty for its own matrices, which is why this reads as an omission at
  u-rev 2 rather than as a scope decision.
- **No REVIEW-design has run against this text at THIS u-rev** (U3-A, which is
  where the round record lives; this bullet points at it and keeps no second
  copy). The previous round ran and FAILED, and a review of a superseded revision
  does not transfer.

---

*U3, u-rev 7. A carve, plus the repairs answering its five review rounds — see the
REVIEW STATUS block and U3-A, which are where those rounds are recorded. **IT IS
UNREVIEWED, AND THE ROUND THAT PRODUCED IT DID NOT DISPATCH A SIXTH REVIEW: see the
REVIEW STATUS block for why.** IMPL has not started.*
