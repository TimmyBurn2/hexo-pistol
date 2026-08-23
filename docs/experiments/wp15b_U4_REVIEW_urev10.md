# REVIEW-design (re-review) — `docs/experiments/U4_soundness_instrument.md`, u-rev 10

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer. The dispatching session repaired U4; it did not review it. Reports
live in the tree rather than in a scratchpad because finding IDs are cited
across units.
-->

## Header

- **Pinned revision:** `2bc4170`. **At entry:** `git rev-parse HEAD` → `2bc4170a96ec03a04fa34837a958dd386f56d268`. **At exit:** identical. **The pin matches HEAD**, and it matches the revision the dispatch named.
- **Subject moved?** No. `git diff 2bc4170 HEAD -- docs/experiments/U4_soundness_instrument.md` → 0 lines. `git status --porcelain` → 0 lines; the tree is clean.
- **Document + u-rev:** `docs/experiments/U4_soundness_instrument.md`, u-rev 10, landed at `56523b2`.
- **MEASURED size:** `wc -l` → **2192** lines; `wc -c` → **189731** bytes.
- **Prior report answered:** `docs/experiments/wp15b_U4_REVIEW_urev9.md`, against `1964026` — FAIL, 0 BLOCKING / 6 MAJOR / 4 MINOR.
- **Context freshness:** fresh. I did not author this document, any of its u-revs, any ADR line, or any prior report. Every number below was measured in this session and is marked MEASURED.
- **Scope as given, and I held to it.** Three finding kinds only: (a) a false or drifted claim in **surviving** text, including text this round introduced; (b) a normative claim orphaned by a strike (D-346's flip clause); (c) a normative error. The absence of struck text is not a finding; a pointer is not a finding. Gate 15's subject is out of scope, and I established MEASURED what gate 15 actually covers so that nothing I charge falls inside it:

```
$ sed -n '1,60p' tools/label_consistency_check.sh | grep -n "closing line that carries\|THE SUMMAND LINE\|THE GROUP-COUNT LINE\|THE SUMMANDS AGAINST"
37:# closing line that carries the u-rev LABEL and nothing else cannot go stale
52:#   1. THE SUMMAND LINE — `20 + 4 + 5 + 3 = **32**`. The arithmetic must hold,
55:#   2. THE GROUP-COUNT LINE — `**U2 (20):**` introducing a backtick-quoted list.
58:#   3. THE SUMMANDS AGAINST THE TABLE ABOVE THEM. Where a summand line sits under
```

Gate 15 covers head/foot label agreement and summand/group self-counts. It does **not** cover whether a label bumped when the bytes moved, and it does not read review-status prose. Every finding below is outside it.

## VERDICT

**FAIL — 0 BLOCKING, 4 MAJOR, 2 MINOR**

Counts derived mechanically from this report's own finding headings: four `###` headings under `## MAJOR`, two under `## MINOR`, none under `## BLOCKING`.

**The diff-shape check first, because the round's constraint was NO-AUTHORING and it held it.** MEASURED, `git show 56523b2 --numstat` → **13 insertions, 14 deletions**, one file. I classified all twelve hunks: two are mechanical label bumps (head, foot); four are **pure deletions** (the `"and is that review's home"` clause; the `| grep -v 'landed'` filter; `"and it is the fourth of its shape struck from this unit"`; the `"THIS u-rev — u-rev 9 — HAS NOT BEEN REVIEWED"` sentence); five are **minimal word-substitutions**, four of which paste the prior reviewer's own suggested fix text verbatim; one is the record-stamp header widening, which is the prior reviewer's suggested fix option 1, quoted. **No hunk adds a new sentence.** The no-authoring constraint was honoured, and I found no drift introduced by the substitutions themselves. Six of the ten prior findings are cleanly discharged and I record each below.

**The one-line diagnosis.** The repair is well-executed at every site it touched and manufactured no new false claim — which is a first for this unit. It fails on the same axis every prior round failed on, one site removed: **the round deleted the newest instance of a false self-state sentence and left two verbatim siblings of it standing 32 and 72 lines below**, one of which contradicts a sentence 40 lines above it. Separately, and worse for the process than for the text: a commit that landed **between** the reviewed revision and this one (`e42ca88`) appended 14 lines to this unit **without bumping its label**, so `u-rev 9` now names two different texts — the exact ambiguity D-311 exists to remove, produced by the commit that amends D-311 to reaffirm it — and the two claims in this document that rest on that commit's edit were left un-re-read. That is this unit's signature defect class, reproduced by the repair chain itself rather than by the repair.

---

# FINDINGS

## BLOCKING

None.

---

## MAJOR

### 1. The round deleted `"THIS u-rev — u-rev 9 — HAS NOT BEEN REVIEWED"` and left two verbatim siblings of the same sentence standing — one of them contradicting the head 40 lines above it

**MEASURED — every occurrence of the shape in the subject at HEAD:**

```
$ grep -n "HAS NOT" docs/experiments/U4_soundness_instrument.md | grep -i "REVIEWED"
233:**VERDICT FAIL**, **0 BLOCKING, 4 MAJOR, 4 MINOR**. **THIS u-rev — u-rev 8 — HAS NOT
265:**VERDICT FAIL**, **1 BLOCKING, 2 MAJOR, 4 MINOR**. **THIS u-rev — u-rev 7 — HAS NOT
```

**The two sentences, verbatim, with their blocks:**

```
$ sed -n '230,234p' docs/experiments/U4_soundness_instrument.md
**THE PRIOR ROUND — u-rev 7's review; u-rev 8 was its repair.**
`docs/experiments/wp15b_U4_REVIEW_urev7.md`, REVIEW-design, fresh context, dispatched
against the named revision **`0f49c90`** (which was HEAD, tree clean, when it ran) —
**VERDICT FAIL**, **0 BLOCKING, 4 MAJOR, 4 MINOR**. **THIS u-rev — u-rev 8 — HAS NOT
BEEN REVIEWED.**
$ sed -n '262,266p' docs/experiments/U4_soundness_instrument.md
**THE PRIOR ROUND — u-rev 6's review; u-rev 7 was its repair.**
`docs/experiments/wp15b_U4_REVIEW_urev6.md`, REVIEW-design, fresh context, dispatched
against the named revision **`7358a07`** (which was HEAD, tree clean, when it ran) —
**VERDICT FAIL**, **1 BLOCKING, 2 MAJOR, 4 MINOR**. **THIS u-rev — u-rev 7 — HAS NOT
BEEN REVIEWED**, and u-rev 6's review does not transfer to it: an amendment reopens
```

**Contradicting evidence — both are false, twice over each, and the falsifiers are inside this same document and this same directory:**

```
$ ls docs/experiments/ | grep 'U4_REVIEW'
wp15b_U4_REVIEW.md
wp15b_U4_REVIEW_urev6.md
wp15b_U4_REVIEW_urev7.md
wp15b_U4_REVIEW_urev8.md
wp15b_U4_REVIEW_urev9.md
$ sed -n '193,196p' docs/experiments/U4_soundness_instrument.md
**REVIEW STATUS — u-rev 8 WAS REVIEWED AND FAILED; u-rev 9 IS THE REPAIR.**
`docs/experiments/wp15b_U4_REVIEW_urev8.md`, REVIEW-design, fresh context, dispatched
against the named revision **`a2b50bf`** — **VERDICT FAIL**, **1 BLOCKING, 7 MAJOR,
7 MINOR**.
```

U4:233 says u-rev 8 **has not been reviewed**. U4:193–196 says u-rev 8 **was reviewed and failed**, and names the report, its pinned revision and its counts. Both are live, unstamped, present-tense text in the same head apparatus, 40 lines apart. U4:265 says u-rev 7 has not been reviewed; U4:231 names `wp15b_U4_REVIEW_urev7.md` and its verdict. And "THIS u-rev" is, at HEAD, u-rev 10 in both sentences:

```
$ sed -n '15p' docs/experiments/U4_soundness_instrument.md | cut -c1-14
**u-rev 10.**
```

**And this round deleted exactly this sentence, one block up:**

```
$ git show 56523b2 -- docs/experiments/U4_soundness_instrument.md | grep '^-.*HAS NOT BEEN REVIEWED'
-7 MINOR**. **THIS u-rev — u-rev 9 — HAS NOT BEEN REVIEWED**, and u-rev 8's review does
```

**Why it breaks.** This is the defect class in its purest form, and it is not a judgement call: D-346 names it by name. Its own text lists, as the class it governs, *"a claim a document makes about **its own state** — its u-rev history, **which of its own revisions were reviewed and with what verdict**…"*:

```
$ sed -n '737p' docs/decisions.md | grep -o "which of its own revisions were reviewed and with what verdict"
which of its own revisions were reviewed and with what verdict
```

The round identified the shape correctly, named it in its own commit subject (*"plus the same self-referential 'u-rev 9 HAS NOT BEEN REVIEWED' staleness U2/U3 hit"*), applied D-346's remedy — deletion, replaced with nothing — to one instance, and did not run the two-second grep that would have found the other two. The sentences are three lines apart in structure and identical in wording; the only search needed is the phrase the commit subject itself quotes. This is the fifth consecutive round in which a repair to one claim left a sibling instance of the same claim standing elsewhere in this file, and it is the first in which the round's own commit message names the search string that would have found the siblings.

The consequence is not decorative. The head apparatus is the surface a next repair round, an IMPL session or the architect reads to learn what this unit's review debt is, and it now states both that u-rev 8 was reviewed and failed and that u-rev 8 has not been reviewed. A reader who resolves the contradiction the wrong way concludes that two of this unit's four landed review reports do not exist.

**Fix scope.** Delete both sentences — `"**THIS u-rev — u-rev 8 — HAS NOT BEEN REVIEWED.**"` at U4:233–234 and `"**THIS u-rev — u-rev 7 — HAS NOT BEEN REVIEWED**, "` at U4:265–266 — replaced with nothing, per D-346. Both are pure deletions and neither strands anything: the normative half of the second (*"an amendment reopens the review, however small the diff"*) survives in the same sentence after the comma, and again at U4:143–148 and U4:2181–2183. I verified that retention before proposing the deletion; see VERIFIED WITH NO FINDING, item 2.

---

### 2. The head's `REVIEW STATUS` block is one round behind: it asserts in the present tense that u-rev 9 **is** the repair, at u-rev 10, with the u-rev 9 review landed in the tree, FAILED, and named nowhere in this document

**The claim reviewed — U4:193, verbatim:**

```
$ sed -n '193p' docs/experiments/U4_soundness_instrument.md
**REVIEW STATUS — u-rev 8 WAS REVIEWED AND FAILED; u-rev 9 IS THE REPAIR.**
```

**Contradicting evidence — the u-rev 9 review is landed, tracked at HEAD, and FAILED:**

```
$ git ls-files --error-unmatch docs/experiments/wp15b_U4_REVIEW_urev9.md && echo TRACKED
docs/experiments/wp15b_U4_REVIEW_urev9.md
TRACKED
$ sed -n '21,23p' docs/experiments/wp15b_U4_REVIEW_urev9.md
## VERDICT

**FAIL — 0 BLOCKING, 6 MAJOR, 4 MINOR**
```

**And it appears nowhere in the subject:**

```
$ grep -n "urev9\|u-rev 9 review\|u-rev 9's review" docs/experiments/U4_soundness_instrument.md
$ echo "exit=$?"
exit=1
```

**Why it breaks.** The document defines its own convention for this apparatus, three times over, and the convention makes the top slot a live claim about which round is current rather than a historical note. The three blocks below it are all labelled with the same past-tense form — `THE PRIOR ROUND — u-rev 7's review; u-rev 8 was its repair.` (U4:230), `THE PRIOR ROUND — u-rev 6's review; u-rev 7 was its repair.` (U4:262), `THE PRIOR ROUND — u-rev 5's review…` (U4:291) — while the top block alone is labelled `REVIEW STATUS` and alone uses the present tense (`IS THE REPAIR`). At u-rev 10 the review status is that u-rev 9 was reviewed and failed with 6 MAJOR and 4 MINOR and that u-rev 10 is its repair; the block says the status is one round earlier.

This is BLOCKING 1 of `wp15b_U4_REVIEW_urev8.md` recurring by its own description — *"the closing line and three further status surfaces are one round behind"* — and it is the fourth time a status surface of this unit has gone one round behind. The round's own head acknowledges the pattern at U4:335 (*"it went one round behind at u-rev 8 and was BLOCKING 1 of `wp15b_U4_REVIEW_urev8.md`"*) while the surface immediately above it does it again.

**I am not charging the absence of a u-rev-10 status block.** Under the round's no-authoring constraint no such block could be written, and an absence is out of scope. What I charge is the surviving present-tense label on a block that is no longer the status — a drifted claim, correctable by word-substitution alone.

**Fix scope.** Two in-policy options, neither of which authors a sentence. **(a)** Relabel to the form the three blocks below already use: `**THE PRIOR ROUND — u-rev 8's review; u-rev 9 was its repair.**` — pure word-substitution, and it makes the top block consistent with its siblings. **(b)** The stronger option, and the one D-346 points at: **strike the block's verdict-and-counts line entirely.** The document has already ruled twice that this content's home is the report and not this unit — U4:335 (*"Each round's pinned revision, verdict and counts live in that round's report, `docs/experiments/wp15b_U4_REVIEW*.md`, which is their home under D-331"*) and U4:2183–2185 (*"WHICH EARLIER u-revs WERE REVIEWED, WITH WHAT VERDICT AND WHAT COUNTS, IS NOT RESTATED HERE"*). The head restates precisely pinned revision, verdict and counts for four rounds, against both rulings. Striking is what stops this surface going stale a fifth time; relabelling only postpones it.

---

### 3. Two sites still tell the architect that the round-label question is open in the OPEN list — `e42ca88` closed it and struck that bullet, which now reads `CLOSED`

**The two claims reviewed, verbatim:**

```
$ sed -n '132,134p' docs/experiments/U4_soundness_instrument.md
  running. **THE LABEL BUMPS PER LANDED COMMIT AGAIN**, which is D-311 as written; the
  round-label reading u-rev 8 asserted is struck, and see the OPEN list for what that
  leaves the architect.
$ sed -n '219p' docs/experiments/U4_soundness_instrument.md | grep -o "whether a round-label is wanted instead is an amendment to D-311 and is in the OPEN list for the architect"
whether a round-label is wanted instead is an amendment to D-311 and is in the OPEN list for the architect
```

**Contradicting evidence — the OPEN-list bullet those two sites point at, at HEAD:**

```
$ sed -n '2175,2179p' docs/experiments/U4_soundness_instrument.md
- ~~**THE u-rev LABEL BUMPS PER LANDED COMMIT, AND WHETHER A ROUND-LABEL MAY REPLACE
  THAT IS THE ARCHITECT'S.**~~ **CLOSED BY `D-311`'s APPENDED AMENDMENT.** The
  round-label reading is rejected as an AGGREGATION — the self-state-claim class
  D-346 and D-338 already narrow — and the per-commit bump stands unchanged. This
  bullet's own request is answered there and is not restated here.
```

**And D-311's amendment closes it in its own words:**

```
$ sed -n '667p' docs/decisions.md | grep -o "A ROUND-LABEL — ONE u-rev SPANNING A MULTI-COMMIT REPAIR ROUND — IS REJECTED, AND THE PER-COMMIT BUMP STANDS UNCHANGED"
A ROUND-LABEL — ONE u-rev SPANNING A MULTI-COMMIT REPAIR ROUND — IS REJECTED, AND THE PER-COMMIT BUMP STANDS UNCHANGED
$ sed -n '667p' docs/decisions.md | grep -o "This closes the OPEN request; it is not reopened by a future round finding a multi-commit label convenient"
This closes the OPEN request; it is not reopened by a future round finding a multi-commit label convenient
```

**Where the staleness came from — the commit that closed the question edited the target bullet and not the two claims resting on it:**

```
$ git show e42ca88 --stat --format='%s' | head -5
docs(decisions): D-311 amended to reject round-labels as aggregation (the self-state-claim class D-346/D-338 already narrow) and confirm the per-commit bump stands — closing U4's u-rev 9 OPEN request by pointer

 docs/decisions.md                           |  2 +-
 docs/experiments/U4_soundness_instrument.md | 14 +++++---------
```

**Why it breaks.** Nothing is left to the architect there, and nothing is in the OPEN list for the architect. Both sentences instruct a reader to go and find an open architect decision, and both send them to a bullet that says the opposite of what they promise. These are not pointers whose target moved — they are claims about what is owed and to whom, and both are false.

This is the u-rev 5 reviewer's one-line diagnosis of this unit reproduced exactly: *"a change landed in one place with the claims resting on it left un-re-read elsewhere"* (U4:294–295). `e42ca88` landed the closure at the bullet; the u-rev 10 round then re-read this unit against a six-MAJOR report, edited the change-log entry at U4:127 — **five lines above** the first stale sentence — and edited the REVIEW STATUS row at U4:216, **three rows above** the second, and re-read neither.

**Fix scope.** At U4:133–134 delete `"and see the OPEN list for what that leaves the architect"`. At U4:219 delete `"and is in the OPEN list for the architect"`. Both are pure deletions; both sentences close cleanly without them, and the surviving `"whether a round-label is wanted instead is an amendment to D-311"` remains true.

---

### 4. `"THE LABEL BUMPS PER LANDED COMMIT AGAIN"` is falsified by the very next commit to touch this unit: `e42ca88` appended 14 lines under the unbumped label `u-rev 9`, so `u-rev 9` names two texts and the second was never reviewed

**The claim reviewed — U4:132, and its normative twin at U4:143–145:**

```
$ sed -n '132p' docs/experiments/U4_soundness_instrument.md | grep -o "\*\*THE LABEL BUMPS PER LANDED COMMIT AGAIN\*\*, which is D-311 as written"
**THE LABEL BUMPS PER LANDED COMMIT AGAIN**, which is D-311 as written
$ sed -n '143,147p' docs/experiments/U4_soundness_instrument.md
**LABEL DISCIPLINE — D-311, travelling item T5.** Any append to this unit bumps
its u-rev, however small the diff. A review is dispatched against a named
revision and reviews of superseded revisions do not transfer; the superseded
document carried the label "Revision 7" at both `d94dc0a` and `6feb40a`, which
differ by 69 lines, and that ambiguity is what this rule removes.
```

**Contradicting evidence — MEASURED, `u-rev 9` names two distinct texts:**

```
$ git log --oneline 3ced890..HEAD -- docs/experiments/U4_soundness_instrument.md
56523b2 docs(experiments): U4 reaches u-rev 10 — ...
e42ca88 docs(decisions): D-311 amended to reject round-labels as aggregation ...
$ git diff 3ced890 e42ca88 --stat -- docs/experiments/U4_soundness_instrument.md
 docs/experiments/U4_soundness_instrument.md | 14 +++++---------
 1 file changed, 5 insertions(+), 9 deletions(-)
$ for c in 3ced890 e42ca88 56523b2; do printf "%s  head=" "$c"; git show $c:docs/experiments/U4_soundness_instrument.md | sed -n '15p' | cut -c1-14; done
3ced890  head=**u-rev 9.** C
e42ca88  head=**u-rev 9.** C
56523b2  head=**u-rev 10.**
$ for c in 3ced890 e42ca88; do printf "%s  blob=" "$c"; git rev-parse $c:docs/experiments/U4_soundness_instrument.md; done
3ced890  blob=4caaacd53e2fc6c3d09daf1c40a72daf523a3822
e42ca88  blob=24dc8a72e6654840935b1d6404090a24910aefa8
```

**And the text `e42ca88` produced was never reviewed — the u-rev 9 review ran before it, against the other blob:**

```
$ git rev-parse 1964026:docs/experiments/U4_soundness_instrument.md
4caaacd53e2fc6c3d09daf1c40a72daf523a3822
$ git merge-base --is-ancestor 1964026 e42ca88 && echo "the urev9 review's pin PRECEDES the unbumped append"
the urev9 review's pin PRECEDES the unbumped append
```

`wp15b_U4_REVIEW_urev9.md`'s header pins `1964026`, whose U4 blob is `4caaacd` — `3ced890`'s text. `e42ca88` then produced blob `24dc8a7` under the same label, after the review, and no review has ever been dispatched against it.

**Why it breaks.** *(i)* The claim is false as written. `"THE LABEL BUMPS PER LANDED COMMIT AGAIN"` asserts the practice resumed at u-rev 9; the next landed commit to touch this unit appended to it and did not bump. *(ii)* The LABEL DISCIPLINE paragraph's closing clause — *"that ambiguity is what this rule removes"* — is falsified by this unit's own most recent history: it cites two texts 69 lines apart under one label as the ambiguity, and `u-rev 9` now names two texts 14 lines apart under one label. *(iii)* The consequence is normative and reaches CLAUDE.md directly: *"an amendment reopens the review, however small the diff"*, and D-311's own ground is that *"an unbumped append makes «a review of revision N» name two different texts"*. There is a text of this unit that has never been reviewed and that no label distinguishes.

*(iv)* And the process irony is load-bearing rather than decorative: the commit that broke the rule is the commit that **amends D-311 to reaffirm it** — `"docs(decisions): D-311 amended to reject round-labels … and confirm the per-commit bump stands"`. Its amendment argues that a round-label is rejected because *"an aggregate label is a wider instance of the same claim than a single-commit one"*, and the same commit made `u-rev 9` an aggregate over two commits.

*(v)* I record explicitly that the prior reviewer considered this charge and rejected it, correctly for its revision. `wp15b_U4_REVIEW_urev9.md` RJ2 rejected *"`THE LABEL BUMPS PER LANDED COMMIT` is falsified by `bb64501`"* on the ground *"u-rev 9 itself is one commit (`3ced890`) and honours the rule. The claim is normative, not a history claim, and nothing at HEAD falsifies it."* That ground was true at `1964026` and was falsified by `e42ca88`, which landed after the review. The rejection does not transfer, for the same reason a review does not.

**Fix scope.** This one is not fully repairable inside the document, and I state that rather than pretending otherwise. The text-side fix is a deletion: strike `"AGAIN"` from U4:132, or strike the clause, leaving the rule stated once at U4:143–145 where it is normative and cannot go stale. The remainder is the architect's: `u-rev 9` names two texts, one of them unreviewed, and D-311 gives no retroactive remedy. Recording that in the OPEN list is the honest disposition; re-labelling history is not.

---

## MINOR

### 1. The change log's u-rev 9 entry names its ADR folds as `D-345`–`D-348`; the text under that label also folds D-311's appended amendment, which the entry does not name

**The claim reviewed — U4:123–125, verbatim:**

```
$ sed -n '123,125p' docs/experiments/U4_soundness_instrument.md
- **u-rev 9** — **the repair of `docs/experiments/wp15b_U4_REVIEW_urev8.md` under the
  STRIKE-AND-DISPOSE policy (`D-346`), and the fold of `D-345`–`D-348`.** The report is
  the home of its own findings and this entry restates none of them; the REVIEW STATUS
```

**Contradicting evidence — a fifth ADR fold landed under the same label:**

```
$ sed -n '2176p' docs/experiments/U4_soundness_instrument.md | grep -o "CLOSED BY \`D-311\`'s APPENDED AMENDMENT"
CLOSED BY `D-311`'s APPENDED AMENDMENT
$ git show e42ca88 -- docs/experiments/U4_soundness_instrument.md | grep -c '^+.*D-311'
1
```

The fold of D-311's amendment landed at `e42ca88`, under head label `u-rev 9` (MEASURED in MAJOR 4 above), and the change log's u-rev 9 entry names `D-345`–`D-348` and not D-311. The block's own header states the enumeration is exhaustive:

```
$ sed -n '65,67p' docs/experiments/U4_soundness_instrument.md
**WHAT IS NOT A VERBATIM CARVE, u-rev BY u-rev — the change log, RE-READ at u-rev 6
rather than appended to.** The text is a verbatim carve apart from cross-reference
retargets and the following, each stated where it occurs.
```

**Why it breaks.** It is graded MINOR and not MAJOR because it is a downstream symptom of MAJOR 4 rather than an independent defect — the entry is complete for the commit it was written in, and it is the unbumped append that put a fifth fold under its label. It is charged rather than waived because the header quantifies (*"apart from … the following"*) and the enumeration is now short by one, and because the change log is where a next round looks to learn what a given u-rev did.

**Fix scope.** Add D-311 to the entry's fold list — a two-token substitution, `` `D-345`–`D-348` `` → `` `D-311`, `D-345`–`D-348` `` — or, if MAJOR 4's label question is resolved by giving `e42ca88`'s text its own u-rev, the entry needs no change and the fix belongs there instead.

---

### 2. The §8 record stamp's own heading says it was `RE-SCOPED at u-rev 7` — this round re-scoped it again, one line below, and left the heading naming u-rev 7

**The two texts, one line apart, verbatim:**

```
$ sed -n '490p' docs/experiments/U4_soundness_instrument.md
> ## THE RECORD OF THE EARLIER STATES — kept, STAMPED at u-rev 6, and RE-SCOPED at u-rev 7
$ sed -n '512,513p' docs/experiments/U4_soundness_instrument.md
> **WHAT IS NOT RECORD — editable by this unit; items (i)–(v) are additionally
> the state now, and IMPL reads those as such:**
```

**Contradicting evidence — the second of those is this round's own edit, and it is a re-scoping:**

```
$ git show 56523b2 -- docs/experiments/U4_soundness_instrument.md | grep '^[-+].*WHAT IS NOT RECORD'
-> **WHAT IS NOT RECORD — these are the state now, and IMPL reads them as such:**
+> **WHAT IS NOT RECORD — editable by this unit; items (i)–(v) are additionally
```

The stamp's NOT-RECORD half went from a one-criterion header covering six items to a two-criterion header that removes item (vi) from the "state now" set. That is a re-scoping of exactly the kind the heading records for u-rev 7 — and item (vi) itself was added at u-rev 9, a second unrecorded scoping event.

**Why it breaks.** The heading is the block's own currency stamp, and it is the surface a reader checks to learn how current the classification below it is. It now names u-rev 7 as the last time the classification moved, when it moved at u-rev 9 and again at u-rev 10. I record honestly that the clause as written is not *false* — the stamp **was** scoped at u-rev 6 and re-scoped at u-rev 7 — and that the charge is **drift**: it is a self-state claim about the block's own revision history, two events behind, one of them made by this round's own edit. It is MINOR because no reader is misled about the classification itself, only about its vintage. It is charged rather than waived because it is precisely a claim this round's own edit made stale, one line from the edit, in a document whose entire failure history is that shape.

**Fix scope.** Substitute `u-rev 7` → `u-rev 10` in the heading, or delete the trailing `", and RE-SCOPED at u-rev 7"` so the heading stops carrying a scoping history it cannot keep current. The deletion is the D-346-shaped remedy and cannot go stale again.

---

# VERIFIED WITH NO FINDING

**The six MAJOR / four MINOR chain from `wp15b_U4_REVIEW_urev9.md` is genuinely discharged, and I checked each one at its site rather than reading the round's account of it.**

1. **MAJOR 1 (`"and is that review's home"`) — discharged by pure deletion, and the ADR side was fixed too.** `sed -n '15,16p'` now reads `(revision 7, CLOSED by D-309 — which records the fresh-context REVIEW-design that FAILED it)` with the home claim gone. The 432-line report the reviewer identified as the real home is landed and tracked (`wc -l docs/experiments/wp15b_design_rev7_REVIEW.md` → `432`), and `d0f36b3` independently retargeted D-309's citation to it (*"D-309's 'session-scoped' premise is falsified now that the revision-7 review report is landed at 7d0418e"*). Nothing normative was orphaned — the review's failure is still stated in the surviving clause.

2. **MAJOR 2 (`"six texts 289 lines apart"`) — INDEPENDENTLY VERIFIED GONE, not relocated and not rephrased, as the dispatch asked.** MEASURED: `grep -n "289\|six texts\|291 " docs/experiments/U4_soundness_instrument.md` returns six lines, and **every one of them is a reference to `tools/baseline_snapshot.sh:289`** (U4:1102, 1122, 1123, 1323, 1340, 1342 — the line-289 basename-loop guard, an unrelated subject). There is no occurrence of `six texts`, of `289 lines`, or of `291`. The bullet that carried the figure was struck whole at `e42ca88` and replaced by a pointer (`git show e42ca88 -U1 -- <subject>` shows a 9-line deletion and a 5-line pointer). I then checked the pointer for a new false claim, since a replacement is where this unit manufactures them: the bullet says the round-label reading is *"rejected as an AGGREGATION — the self-state-claim class D-346 and D-338 already narrow — and the per-commit bump stands unchanged"*, and D-311's amendment reads *"A round-label is an AGGREGATE … which is the self-state-claim shape D-346 and D-338 already name and narrow"* and *"THE PER-COMMIT BUMP STANDS UNCHANGED"*. **Accurate on every clause.** The claim is discharged; what `e42ca88` left behind is charged at MAJOR 3 and MAJOR 4, which are about the two sites it did not re-read, not about this one.

3. **MAJOR 3 (`"Eleven such claims"`) — discharged by the reviewer's own suggested substitution.** U4:127 now reads `"Claims of that shape went at this u-rev"`. No cardinal survives; I re-read the sentence and it carries its argument without one.

4. **MAJOR 4 (the MAJOR 3 row reporting a reword as a strike) — discharged, and I verified BOTH halves of the replacement rather than taking the row's word.** The row now reads *"The six enumerated universals STRUCK; the claim itself narrowed at its home to the enumerated set."* I tested all six universals `wp15b_U4_REVIEW_urev8.md` MAJOR 3 enumerated:

```
$ for p in "asserts no line count of itself" "Nothing is silently re-labelled" "no live statement of the count survives" "REPAIRED at every one"; do printf '%-45s => ' "$p"; grep -c "$p" docs/experiments/U4_soundness_instrument.md; done
asserts no line count of itself               => 0
Nothing is silently re-labelled               => 0
no live statement of the count survives       => 0
REPAIRED at every one                         => 1
$ grep -n '23\.2' docs/experiments/U4_soundness_instrument.md | cut -c1-95
311:| revision-7 review **MAJOR 12** (the unmarked `23.2`) and **MAJOR 9** (rule 5 / D-263) | **NOT R
```

Five are gone outright; the sixth survives only inside `~~REPAIRED at every one.~~` (struck through, at U4:302); the `23.2`/rule-5 universal was replaced by the pointer row at U4:311. **Six for six.** And half one is now correctly described as a narrowing at its home rather than a strike (U4:2012–2016). The row is accurate on both halves for the first time.

5. **MAJOR 5 (the wrong home for the relative-base residual) — discharged, and the new home resolves.** The row now names `matrix_M4_axisA_selection.md` condition 4, and that condition exists and is the right one:

```
$ grep -n "condition 4\|^4\.\|CALLER_PWD" docs/experiments/matrix_M4_axisA_selection.md | head -3
114:4. **The relative-base inconsistency is recorded, not fixed** (F6): a relative
116:   `$CALLER_PWD`. N-E inherits it; unlike N-Q it does not make it load-bearing
```

This is the fourth consecutive round in which this residual's attribution was contested; it is correct now.

6. **MAJOR 6 (the record-stamp header) — discharged, and I checked the widened header against all six items rather than accepting it.** The new header separates two criteria: *editable* (all six) and *the state now* (items i–v). I read each item at its own site. All six are unit-editable — (ii) is *"carve-authored at u-rev 2 under D-316 and retargeted at u-rev 6"*, (vi) is *"CARVE PROSE this unit may edit and did edit"*. Items (i)–(v) are each justified at their site by currency (*"states the selected instrument"*, *"states what that gate's instrument IS"*, *"the live specification `tools/staged_soundness_check.sh` is taken from"*). Item (vi) — the block containing the present-tense `"so S-E *is* gate (b)'s instrument"` that the prior reviewer charged — is now correctly outside the "state now" set while staying editable, which is exactly the third classification the prior reviewer said it needed. **The header's universal is no longer false of its own newest member.** The stamp's closing sentence *"Each of the six says at its own site which it is"* still holds: the six item markers are present (`grep -o '\*\*(\(i\|ii\|iii\|iv\|v\|vi\))\*\*'` returns exactly six), and §8.7's block carries its own u-rev 8 disposition.

7. **MINOR 1 (the `"fourth of its shape"` ordinal) — discharged by pure deletion.** The clause is gone; the sentence before it carries the argument, as the reviewer said it would.

8. **MINOR 2 (the wrong u-rev for item (vi)'s ruling) — discharged, and I checked the substituted u-rev rather than trusting it.** The row now reads *"with the u-rev 8 carve-prose ruling as its reason"*. The ruling is stated at U4:996–999 inside a parenthetical headed `*(CORRECTED AT u-rev 8 — MAJOR 1 of wp15b_U4_REVIEW_urev7.md`, and its words are *"It is corrected IN PLACE, not annotated, because this block is carve prose"*. u-rev 8 is right, and the circularity the prior reviewer flagged is gone.

9. **MINOR 3 (the finding aid's `landed` filter) — discharged, AND THE FIX ACTUALLY WORKS, which I verified against the specific line the prior reviewer used as its falsifier.** The round took the reviewer's second option and dropped the filter. The falsifying line has moved from 303 to 302 (the round is net −1 line), and it is now surfaced:

```
$ grep -n 'BLOCKING 3\*\* — the staged-config count' docs/experiments/U4_soundness_instrument.md | cut -c1-40
302:| **BLOCKING 3** — the staged-config c
$ grep -nE '(^|[^A-Za-z])U[123]\b' docs/experiments/U4_soundness_instrument.md | grep -c '^302:'
1
$ grep -nE '(^|[^A-Za-z])U[123]\b' docs/experiments/U4_soundness_instrument.md | wc -l
36
```

The aid returns 36 lines where the filtered form returned 29, and the bare `U3 §10` on the BLOCKING 3 row is among them. The surviving disclaimer at U4:179–182 — *"A reference shape it does not match would be invisible to it as well"* — is now the **only** blindness the aid has, so the disclaimer became accurate as a side effect of the fix rather than needing its own repair.

10. **MINOR 4 (`"IS NOT COUNTED OR RESTATED HERE"`) — discharged by deleting the falsified half.** The bullet now reads `"AND IS NOT COUNTED HERE"`, and it states no count of the enumeration; naming one item is not counting it. The reviewer's suggested trailing clause was not added, correctly, under the no-authoring constraint.

**D-346's flip clause does not fire on this round.** I checked each of the round's four deletions for an orphaned normative claim, which is the failure mode a strike-based policy has:

- **`"and is that review's home"`** — a home attribution, not normative. Nothing depends on it.
- **`"THIS u-rev — u-rev 9 — HAS NOT BEEN REVIEWED, and u-rev 8's review does not transfer to it."`** — the second clause **is** normative, and it is my strongest flip candidate. It survives in three places: U4:143–147 (*"A review is dispatched against a named revision and reviews of superseded revisions do not transfer"*), U4:266–268 (*"an amendment reopens the review, however small the diff (D-311, and CLAUDE.md's own words)"*), and U4:2181–2183 (*"a review of a superseded revision does not transfer — an amendment reopens the review, however small the diff"*). **Triply retained. No loss** — and this is what licenses the deletion I ask for in MAJOR 1.
- **`"and it is the fourth of its shape struck from this unit"`** — a self-state ordinal. Nothing normative.
- **`| grep -v 'landed'`** — the removal only widens the aid's output. Nothing depends on the narrower set.

**Further verifications that produced no finding:**

- **The diff introduces no new sentence.** MEASURED, `git show 56523b2 --numstat` → 13 insertions / 14 deletions. I classified every hunk (see VERDICT). The one hunk that adds words — the record-stamp header — is the prior reviewer's own suggested replacement text, quoted, and I verified it against all six stamp items above. **The NO-AUTHORING constraint was honoured**, and unlike the last four rounds, no repair manufactured a new false claim at its own site. That is the round's real achievement and it should be recorded as such: every one of my four MAJORs is a claim the round **left standing**, not one it wrote.
- **Head and foot labels agree at u-rev 10** (`sed -n '15p'` → `**u-rev 10.**`; `sed -n '2192p'` → `*U4, u-rev 10.*`). Reported as context only; label agreement is gate 15's and is out of scope.
- **The unit's load-bearing MEASURED claims about the tree still hold at HEAD**, so no restated measurement has gone stale under this round: `ls configs/instrument_staged_v0.toml` → `No such file or directory`; `ls crates/pistol-search/src/staged.rs` → `No such file or directory`; `sed -n '182p' tools/baseline_snapshot.sh` → `CONFIG="configs/instrument_v0.toml"`; `grep -c -- "--config" tools/baseline_snapshot.sh` → `4`. The foot's *"IMPL has not started"* is true.
- **U4:154–162's bounded citation claim holds.** It states that u-rev 8 converted *six* citations that named the wrong u-rev — five naming U3 at u-rev 4, one naming U2 at u-rev 3. MEASURED, `U3** (u-rev 6, landed 13621d3)` occurs at U4:70, 151, 826, 908, 1914 (five) and `U2** (u-rev 5, landed f0ae14c)` at U4:406 (one). Six, and the bound is stated rather than a universal asserted.
- **`wp15b_U3_REVIEW_urev6.md` MAJOR D exists**, as U4:172–173 claims (`grep -n '^### '` → `64:### D. "So every cross-unit citation in this unit now reads (u-rev N, landed <sha>)…`).
- **§9's record stamp carries no sibling of the §8 stamp's MAJOR-6 defect.** Its NOT-RECORD half classifies by supersession, not by currency, and its universal *"Each such sentence carries a u-rev 7 mark at its own site"* holds — I checked the six candidate sentences (U4:1229, 1263, 1272, 1333, 1359, 1377) and each carries its mark.
- **The class is live in a sibling unit but that unit is not my subject.** `grep -n "HAS NOT" docs/experiments/U2_node_protocol.md` → `130:u-rev — u-rev 4 — HAS NOT BEEN REVIEWED`. Recorded for the dispatching session, not charged here.

---

# REJECTED, WITH THE ATTEMPTED REPRODUCER

### RJ1. *"U4-A's row asserting `which is the third time a status surface of this unit has gone stale by being kept in two places` is a non-derivable self-state ordinal of exactly the shape this round deleted at U4:1661"* — REJECTED

**Attempted reproducer:**

```
$ sed -n '335p' docs/experiments/U4_soundness_instrument.md | grep -o "which is the third time a status surface of this unit has gone stale by being kept in two places"
which is the third time a status surface of this unit has gone stale by being kept in two places
$ grep -n "three times running\|the last three times" docs/experiments/U4_soundness_instrument.md | cut -c1-80
132:  running. **THE LABEL BUMPS PER LANDED COMMIT AGAIN**, which is D-311 as written; the
2188:  times.
```

The shape is identical to MINOR 1 of the prior report — a self-state ordinal, non-derivable, asserted in the disposition of a strike — and it is restated at three sites. **But I could not falsify it.** Enumerating the candidate instances from the reports in the tree gives three (`wp15b_U4_REVIEW.md` BLOCKING 1's six sites describing the u-rev 1 state; `wp15b_U4_REVIEW_urev6.md` MINOR 6's stale owner-table row; `wp15b_U4_REVIEW_urev8.md` BLOCKING 1's four surfaces one round behind), which is exactly three and makes the ordinal true rather than false. My scope is a false or drifted claim, and a finding is verified with a minimal reproducer before it is charged. I could not produce one. Rejected — and recorded so the next round does not spend the search again. *(I note without charging it that the ordinal will become false at the next instance, which MAJOR 1 and MAJOR 2 above supply.)*

### RJ2. *"The change-log header's `The text is a verbatim carve apart from cross-reference retargets and the following` is falsified by u-rev 10, which has no change-log entry"* — REJECTED

**Attempted reproducer:**

```
$ sed -n '65,67p' docs/experiments/U4_soundness_instrument.md
**WHAT IS NOT A VERBATIM CARVE, u-rev BY u-rev — the change log, RE-READ at u-rev 6
rather than appended to.** The text is a verbatim carve apart from cross-reference
retargets and the following, each stated where it occurs.
$ grep -n "^- \*\*u-rev 10\*\*\|^- \*\*u-rev 9\*\*" docs/experiments/U4_soundness_instrument.md
123:- **u-rev 9** — **the repair of `docs/experiments/wp15b_U4_REVIEW_urev8.md` under the
```

There is no u-rev 10 entry. **But the universal quantifies over departures from the VERBATIM CARVE, and u-rev 10 touched no carried text.** I checked every hunk's target: the head apparatus, the finding-aid command, three REVIEW STATUS rows, the §8 record stamp (unit-authored at u-rev 6/7, not carved), the U4-Z lead-in and an OPEN-list bullet. None is text carried from `wp15b_design.md` §8, §9, §11.6 or §12 item 1. The universal survives its literal reading, and charging it would charge the absence of an entry the round's no-authoring constraint forbade. Rejected. *(The narrower version of this concern that I could substantiate — a fold that landed under label u-rev 9 and is missing from the u-rev 9 entry — is charged as MINOR 1, where the enumeration is genuinely short.)*

### RJ3. *"The head's four REVIEW STATUS blocks restate pinned revisions, verdicts and counts, which U4-A:335 and U4:2183 both declare belong to the reports — a D-331 second copy"* — REJECTED as a finding of its own

**Attempted reproducer:**

```
$ sed -n '335p' docs/experiments/U4_soundness_instrument.md | grep -o "Each round's pinned revision, verdict and counts live in that round's report, \`docs/experiments/wp15b_U4_REVIEW\*\.md\`, which is their home under D-331"
Each round's pinned revision, verdict and counts live in that round's report, `docs/experiments/wp15b_U4_REVIEW*.md`, which is their home under D-331
$ grep -c "VERDICT FAIL" docs/experiments/U4_soundness_instrument.md
3
```

The head does restate pinned revision, verdict and counts for four rounds, and the unit twice declares that content's home to be the reports. **But both declarations are explicitly scoped to their own site** — U4:335 says *"THIS TABLE no longer restates"* (the lineage table) and U4:2183 says *"IS NOT RESTATED HERE"* (that OPEN bullet) — so neither is a universal the head blocks falsify, and the prior reviewer verified both strikes as sound. Charging the head blocks as a D-331 breach would be re-litigating a restructure question `D-334` records as STOPPED with no selection, which is not a reviewer's to settle. Rejected as an independent finding, and folded instead into MAJOR 2's fix scope, where it bears: it is the reason option (b), striking the block, is the better repair than relabelling it.
