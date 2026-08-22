# REVIEW-design (RE-REVIEW, no dispatch preceded this one against u-rev 6 or u-rev 7) — `docs/experiments/U2_node_protocol.md` u-rev 7

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer (only HTML-escaped angle brackets restored: `&gt;`/`&lt;` to `>`/`<`).
The dispatching session repaired U2; it did not review it. Reports live in the
tree rather than in a scratchpad because finding IDs are cited across units.
-->

## Header

- **Pinned revision:** `1964026` (full: `1964026c8efe89a4fea09f8e5c499cd40b7d9c42`).
- **Match with HEAD at entry:** YES — `git rev-parse HEAD` at entry returned
  `1964026c8efe89a4fea09f8e5c499cd40b7d9c42`.
- **Match with HEAD at exit:** YES — re-checked at the end of this review,
  unchanged.
- **Subject moved from the pin?** NO — `git diff 1964026 HEAD -- docs/experiments/U2_node_protocol.md`
  is empty (0 lines) at both entry and exit checks.
- **Document + u-rev + measured size:** `docs/experiments/U2_node_protocol.md`,
  u-rev 7 (head table row 7: "`| **7** | *this text* | **NOT YET REVIEWED** | — |`"),
  **945 lines** (`wc -l`, MEASURED).
- **Prior reports read:** `wp15b_U2_REVIEW.md` (u-rev 1, FAIL), `wp15b_U2_REVIEW_urev2.md`
  (u-rev 2, FAIL — 1 MAJOR/1 MINOR), `wp15b_U2_REVIEW_urev3.md` (u-rev 3, FAIL — 1
  MAJOR F5/1 MINOR F6), `wp15b_U2_REVIEW_urev4.md` (u-rev 4, FAIL — 0 BLOCKING/0
  MAJOR/2 MINOR, G1/G2), `wp15b_U2_REVIEW_urev5.md` (u-rev 5, FAIL — 1 MAJOR H1/1
  MINOR H2). No report exists for u-rev 6 (`3543a7f`) — confirmed by
  `ls docs/experiments/ | grep -i U2_REVIEW` returning only the five files above —
  and no report exists yet for u-rev 7. This review is u-rev 7's first.
- **Context freshness:** fresh context, no prior turns. Read CLAUDE.md whole;
  `docs/decisions.md` D-305, D-309, D-311, D-331, D-336, D-345, D-346 (via
  `grep -n '^D-3xx:'` then reading the matched line); the whole subject document;
  `wp15b_U2_REVIEW_urev5.md` and `wp15b_U2_REVIEW_urev4.md` in full; and the full
  `git diff 3543a7f ecb0341 -- docs/experiments/U2_node_protocol.md` (u-rev 6 → u-rev
  7, this round's own diff) and `git diff f0ae14c 3543a7f -- docs/experiments/U2_node_protocol.md`
  (u-rev 5 → u-rev 6, the round this review has never seen a dispatched report for).
- **Scope, as given:** confirmation pass under the STRIKE-AND-DISPOSE policy
  (D-346). The absence of struck text is not a finding; a pointer is not a
  finding; gate 15 (head/foot u-rev agreement, self-counts) is not re-derived.
  Chargeable findings are limited to: (a) a false or drifted claim in surviving
  text, including any claim this round's own edits introduced, with the head's
  u-rev table, the REVIEW STATUS blocks and the rewritten OPEN bullet named as
  the highest-risk surfaces; (b) a normative claim that lost its home to a
  strike; (c) a normative error.

---

## VERDICT: FAIL — 1 BLOCKING, 1 MAJOR, 0 MINOR

Derived from this report's own finding headings, once landed, by:
```
$ grep -c '^### Finding.*(BLOCKING)' <this-report>.md   # -> 1
$ grep -c '^### Finding.*(MAJOR)' <this-report>.md      # -> 1
$ grep -c '^### Finding.*(MINOR)' <this-report>.md      # -> 0
```
One BLOCKING heading (Finding 1, below), one MAJOR heading (Finding 2, below), no MINOR heading.

---

# FINDINGS

## BLOCKING

### Finding 1 (BLOCKING) — the strike of "168 030 comparisons" removes the only evidentiary support for a non-vacuity claim the surviving text keeps asserting, and misattributes that claim to §5.2, which does not carry it

**Claim, U2 lines 872–877 (U2-Z, handoff to WP-1.6, this round's own rewrite):**

> The node protocol's shape is settled and attacked even though unimplemented:
> win-now before overload before filter, one `can_win_this_turn` and one
> `blocking_covers` per node, and the identity that makes that possible
> (`blocking_covers == Impossible ⟺ unblockable_double_threat`) **is verified with
> the `Impossible` branch reached at every budget, over the population §5.2 states
> and marks MEASURED — which is that measurement's one home.** *(~~168 030
> comparisons~~ — WITHDRAWN AS UNVERIFIABLE AT u-rev 7 under `D-346`. …)*

**What changed, reproducer:**
```
$ git diff 3543a7f ecb0341 -- docs/experiments/U2_node_protocol.md | sed -n '/168 030/,+10p'
-  (`blocking_covers == Impossible ⟺ unblockable_double_threat`) is verified over
-  168 030 comparisons with the `Impossible` branch reached at every budget.
+  (`blocking_covers == Impossible ⟺ unblockable_double_threat`) is verified with the
+  `Impossible` branch reached at every budget, over the population **§5.2** states and
+  marks MEASURED — which is that measurement's one home. *(~~168 030 comparisons~~ —
+  **WITHDRAWN AS UNVERIFIABLE AT u-rev 7** under `D-346`. …
```
The pre-strike sentence bundled two distinct facts under one number: (i) the
identity holds over a measured population, and (ii) the `Impossible` branch was
specifically *reached* at every budget (a non-vacuity condition — without it,
"the two predicates agree" could be vacuously true at a budget the sample never
exercised the `Impossible` branch at). This round's edit does not merely delete
the disputed number; it **rewrites the surrounding prose to keep asserting fact
(ii)** ("is verified with the Impossible branch reached at every budget") while
re-pointing its evidentiary support to "the population §5.2 states."

**Contradicting evidence — §5.2 does not state fact (ii):**
```
$ sed -n '317,330p' docs/experiments/U2_node_protocol.md
### 5.2 Steps 2 and 3 — ONE match, because they are one predicate
...
`covers` predicate — and MEASURED over **145 158** playout positions and **343 344**
side-and-budget comparisons with **zero** disagreements.
```
§5.2's MEASURED claim is about raw agreement over a stated population size — it
says nothing about the `Impossible` branch being reached at every one of the
three `HitBudget` values. The only place in the whole document that ever stated
a number in connection with "reached at every budget" was the now-struck 168 030
figure:
```
$ grep -n "every budget" docs/experiments/U2_node_protocol.md
711:| `the_two_predicates_agree_everywhere` | ... with a NON-VACUITY assertion that the `Impossible` branch is reached at each budget — M5-E's soundness, and a sweep that never reached a budget would agree there by not testing it |
875:  `Impossible` branch reached at every budget, over the population **§5.2** states and
```
Line 711 is U2-T's registration of a **future, not-yet-run** test
(`the_two_predicates_agree_everywhere`) that is explicitly framed as the
instrument for this exact non-vacuity property — "a sweep that never reached a
budget would agree there by not testing it" states plainly that the property is
not free, does not follow from §5.2's raw count, and needs its own check. That
check has not run (IMPL has not started — line 758: "This unit has no governed
run"). So at u-rev 7, nothing in the tree currently substantiates "the
`Impossible` branch reached at every budget" — the only number that ever did is
struck, the pointer that replaces it (§5.2) does not carry the fact, and the
future test that would re-establish it has not executed.

**Why this is a scope violation of D-346, not merely a citation gap.** D-346
defines STRIKE-AND-DISPOSE narrowly: *"A claim a document makes about **its own
state** — its u-rev history, which of its own revisions were reviewed and with
what verdict, what its own repairs reached, universals about its own text."* A
comparison count backing a non-vacuity property of a solver-level equivalence
proof is not a claim about the document's own state by any reading of that
list — it is exactly the kind of thing D-346 itself carves out as NOT
strike-shaped: *"a measurement IMPL depends on"* is named verbatim among the
things that must be "repaired minimally at its home and never struck." The
head's own framing at line 153–154 calls it *"the unmarked M5-E population
figure"* and folds it into "two claims this unit made about its own state" —
but only the first of those two (the "counts now live only in D-309" sentence)
is actually a document-state claim; the second is a substantive measurement
about solver behaviour that the document's own U2-T row (line 711) treats as
soundness-relevant and its own OPEN bullet (u-rev 5, H2, kept verbatim at lines
913–919) treats as requiring "a measurement act with a registered instrument" to
resolve — the document's own words about the very figure it then struck.

**Why the ambiguity was never resolved before disposal.** `wp15b_U2_REVIEW_urev4.md`
G2 (the finding this strike answers) explicitly left open whether 168 030 "is a
genuinely different, narrower statistic (e.g. only the subset of the 343 344
comparisons where the `Impossible` branch specifically fired… matching the
'reached at every budget' qualifier)" or a plain transcription error, and said
"I could not settle this either way." u-rev 5 registered it OPEN precisely
because choosing requires re-running the probe. u-rev 7 disposes of the number
by strike without ever resolving that disjunction — if the "distinct,
non-vacuity-specific statistic" branch of G2's hypothesis is correct, this
strike removed exactly the kind of normative content D-346 forbids removing,
and D-346's own flip clause anticipates this: *"a strike that removes a
NORMATIVE claim is a defect — restore it and report the instance."* This report
is that instance.

**Does the surviving §5.2 pointer carry what IMPL and WP-1.6 need?** No. §5.2
carries the raw-agreement measurement (which is real, MEASURED, and untouched);
it does not carry, and was never claimed by any prior u-rev to carry, the
non-vacuity-per-budget fact. A WP-1.6 implementer following the handoff
bullet's "over the population §5.2 states… which is that measurement's one
home" to §5.2 would find no confirmation that `Impossible` fired at every
budget — the exact gap the U2-T test row (line 711) exists to close, and has
not yet closed.

**Why it breaks:** the surviving sentence asserts, in the present tense and
without a MEASURED/ESTIMATED hedge or an OPEN marker, that a specific soundness
property (non-vacuity of the `Impossible` branch across all three budgets) "is
verified," while (i) the only number that ever evidenced it has just been
struck as unverifiable in the same edit, (ii) the pointer substituted for it
does not state the property, and (iii) the document's own OPEN-list twin of
this exact material (lines 913–919, kept verbatim from u-rev 6) says
reconciling/re-deriving it "is answerable only by RE-RUNNING the equivalence
probe" — i.e., the document itself says this fact cannot currently be
established, in one place, while asserting it as accomplished fact fourteen
lines above.

**Severity:** BLOCKING. This is not a wording nit on a historical count; it sits
in the WP-1.6 handoff section for the unit's central adopted design decision
(M5-E, matrix M5, §5.6), the property in question is the specific thing that
distinguishes a genuine equivalence proof from a vacuous one, and the document
explicitly hands this debt to a future work package ("A quiescence that reuses
it inherits that debt" — line 883–885, of the neighbouring bullet) without
flagging that its own non-vacuity backing just went missing.

**Fix scope:** at the handoff bullet (lines 872–877) and, correspondingly, at
the OPEN-list entry (lines 904–926). Either (a) restore a number — even
ESTIMATED or explicitly marked OPEN — for the non-vacuity-per-budget property
specifically, distinguishing it from §5.2's raw-agreement count, or (b) strike
the "with the `Impossible` branch reached at every budget" clause itself (not
just the number that once backed it) and replace it with an honest forward
pointer to U2-T's `the_two_predicates_agree_everywhere` test as the instrument
that will establish it once IMPL runs. Option (b) is the smaller, D-346-shaped
repair: it disposes of a now-unverifiable *claim* rather than leaving a claim
standing on a citation that does not support it.

---

## MAJOR

### Finding 2 (MAJOR) — "THIS u-rev — u-rev 5 — HAS NOT BEEN REVIEWED" is false: u-rev 5 was reviewed, by name, by the very report whose findings (H1, H2) are answered fourteen lines below the false sentence, in this same document

**Claim, U2 lines 140–145 (REVIEW STATUS block, unchanged since u-rev 6, `3543a7f`):**

> **REVIEW STATUS — u-rev 4 WAS REVIEWED AND FAILED ON TWO MINORS; u-rev 5 IS THE
> REPAIR.** `docs/experiments/wp15b_U2_REVIEW_urev4.md`, REVIEW-design
> (re-review), fresh context, against `7473a6f` — VERDICT FAIL, 0 BLOCKING, 0
> MAJOR, 2 MINOR. It confirmed F5's substitution structural on the merits,
> having diffed every intermediate revision, and F6 cleanly discharged. **THIS
> u-rev — u-rev 5 — HAS NOT BEEN REVIEWED.**

**Contradicting evidence, reproducer — the very next REVIEW STATUS table, 14
lines later in the same document:**
```
$ sed -n '159,163p' docs/experiments/U2_node_protocol.md
| Finding, `wp15b_U2_REVIEW_urev5.md` | Where it is answered at u-rev 6 |
|---|---|
| **H1** (MAJOR) — the carve-provenance clause says revision 7 was never reviewed; it was reviewed and it failed, and the clause is verbatim in five documents | ...
| **H2** (MINOR) — G2's stated reason misapplies this unit's own CARVED-CONTENT scope | ...
```
`wp15b_U2_REVIEW_urev5.md` exists on disk, is titled "REVIEW-design (CONFIRMATION
RE-REVIEW) — `docs/experiments/U2_node_protocol.md` u-rev 5," is pinned at
`f0ae14c` (u-rev 5's own commit), and reaches `## VERDICT: FAIL — 1 MAJOR, 1
MINOR` — the exact H1/H2 findings the document's own table two paragraphs later
says were "answered at u-rev 6." So u-rev 5 unambiguously **was** reviewed, and
FAILED, and the document's own apparatus fourteen lines below the "HAS NOT BEEN
REVIEWED" sentence proves it by naming the report and its findings.

**When this went stale, reproducer:**
```
$ git diff f0ae14c 3543a7f -- docs/experiments/U2_node_protocol.md | grep -n "Finding, .wp15b_U2_REVIEW_urev5\|HAS NOT"
+| Finding, `wp15b_U2_REVIEW_urev5.md` | Where it is answered at u-rev 6 |
```
The commit that inserted the H1/H2 table (`3543a7f`, "u-rev 6") added the table
documenting that u-rev 5 was reviewed and failed, but left the pre-existing
"THIS u-rev — u-rev 5 — HAS NOT BEEN REVIEWED" sentence (written when it was
still true, at u-rev 4→5) untouched. It has now survived two further commits
(`3543a7f`, `ecb0341`) unrepaired. Every other REVIEW STATUS transition in this
document gets its own header sentence ("REVIEW STATUS — u-rev N WAS REVIEWED
AND FAILED …; u-rev N+1 IS THE REPAIR" — see lines 126 and 140); the u-rev
5→6 transition never received one, and the stale u-rev-4→5 header was left to
speak for a state it no longer describes.

**Why it breaks:** this is precisely the class D-331/D-311 and the prior
review's own H1 finding target — a claim a document makes about its own review
history, false, self-contradicted a few lines below in the same document, and
uncaught because no review has been dispatched against either u-rev 6 or u-rev
7 until now. It is the same shape as H1 (a stale review-status claim
propagating unrepaired past the round that made it stale) but at a different
site than H1 addressed, so it is not H1 recurring — it is a sibling instance
this round's repair did not touch and this review is the first to name.

**Severity:** MAJOR — matching H1's own severity for the same claim-shape (a
false statement about which u-revs were reviewed). It does not touch §5's
design content or the F5/F6/G1/H1/H2 repair chain's soundness, but it misstates
the document's own review status, which is precisely the fact this apparatus
exists to keep accurate.

**Fix scope:** one sentence, one site (lines 144–145). Either delete "THIS
u-rev — u-rev 5 — HAS NOT BEEN REVIEWED" (the H1/H2 table 14 lines below already
states, correctly, that u-rev 5 was reviewed and what its findings were — D-331
already forbids the restatement this sentence amounts to now that it is
outdated) or replace it with an accurate marker for the *current* boundary
("THIS u-rev — u-rev 7 — HAS NOT BEEN REVIEWED"), matching the pattern the
document already uses correctly at line 129 for u-rev 4.

---

# VERIFIED WITH NO FINDING

- **HEAD/pin agreement.** `git rev-parse HEAD` at entry and exit both return
  `1964026c8efe89a4fea09f8e5c499cd40b7d9c42`; `git diff 1964026 HEAD --
  docs/experiments/U2_node_protocol.md` is empty at both checks.
- **The head's u-rev table, row by row against `git log`.** All six historical
  SHAs (`38f21b9`, `56b0bec`, `d85b049`, `7473a6f`, `f0ae14c`, `3543a7f`) and the
  current HEAD (`ecb0341`) resolve as commits (`git cat-file -t`, all return
  `commit`). Row 2's claim ("byte-identical at HEAD `e3f0bc3`") verified:
  `git diff 56b0bec e3f0bc3 -- docs/experiments/U2_node_protocol.md` is empty.
  Row 4's SHA (`7473a6f`) is explained, correctly, by
  `wp15b_U2_REVIEW_urev4.md`'s own header as "a LATER commit that touched a
  different unit (U3, reaching u-rev 5); U2's own content last changed at
  `7dfd047`" — verified: `git diff 7dfd047 7473a6f -- docs/experiments/U2_node_protocol.md`
  is empty, so pinning the review at either SHA is equivalent for this
  document; this is not a defect, it is disclosed and correct. Every report
  path in the table exists on disk and its own `## VERDICT` line matches the
  table's verdict column exactly (all FAIL; u-rev 4's row correctly shows the
  0/0/2 breakdown matching `wp15b_U2_REVIEW_urev4.md`'s own verdict line). Row 6
  (`3543a7f`, "NOT REVIEWED — no round was dispatched against it") is accurate —
  no `wp15b_U2_REVIEW_urev6.md` or equivalent exists
  (`ls docs/experiments/*U2*REVIEW*` lists exactly the five files for u-revs
  1–5). Row 7 ("this text | NOT YET REVIEWED") is this review's own subject.
- **"U2-A's revision-7 row restates them, and has since the carve" (line
  46–47).** `git show cf74594:docs/experiments/U2_node_protocol.md | grep -n
  "7 BLOCKING"` shows the U2-A row already stated "7 BLOCKING, 7 MAJOR, 9
  MINOR" at the original carve commit (`cf74594`, u-rev 1) — the claim that
  this row has carried the counts "since the carve" is true, so the strike of
  "the counts now live only in D-309" is correctly grounded: U2-A's row is a
  second, real, existing home for the counts, contradicting the struck
  sentence's "only" claim. Correctly disposed under D-346 (this claim genuinely
  is about the document's own state — where a set of counts is and is not
  restated — squarely inside D-346's definition, unlike Finding 1's subject).
- **"Nothing else moved" (line 157).** `git diff 3543a7f ecb0341 --
  docs/experiments/U2_node_protocol.md` shows exactly four hunks: the head
  provenance-sentence strike, the u-rev-6 table-row backfill (a pre-existing
  gap, not new content), the G2 status-row rewording plus the new "u-rev 7
  ANSWERS NO REVIEW REPORT" paragraph, the handoff-bullet strike, and the
  matching OPEN-bullet rewrite, plus the footer's u-rev bump. No hunk touches
  §2, §3, §5 or §14 (carved content) or any other site. The claim holds.
- **The two struck spans use the same, consistent strike-and-pointer
  convention** (`~~struck text~~` immediately followed by a bolded
  "WITHDRAWN AS …" disposition clause and a named pointer) at both sites (lines
  45–48 and 876–882), matching `wp15b_U4_REVIEW_urev8.md` MAJOR 5's prescribed
  form quoted in D-346. Formatting itself is not a finding under this review's
  scope (gate 15's job), but the two sites are at least mutually consistent.
- **The OPEN bullet's restated reasoning for why reconciliation is impossible
  (lines 908–919), carried verbatim from u-rev 6/H2.** This text is a pointer
  to, and close paraphrase of, `wp15b_U2_REVIEW_urev5.md` H2's own resolution,
  which that fresh-context review already found sound on the practical
  disposition (register OPEN, do not silently reconcile). Not re-litigated here
  beyond Finding 1's narrower point about the handoff bullet's own prose
  advancing past what this OPEN item concedes is unresolved.

# REJECTED, WITH THE ATTEMPTED REPRODUCER

- **Hypothesis: the u-rev table's row 4 SHA (`7473a6f`) is itself a drifted/
  wrong-unit citation, since `wp15b_U2_REVIEW_urev5.md`'s own check 6 describes
  `7473a6f` as "U3→5" (a U3 commit), not a U2 commit.** Attempted reproducer:
  `git show 7473a6f --stat` shows the commit changes only
  `docs/experiments/U3_tier_t.md` (173 lines changed); `git diff 7dfd047 7473a6f
  -- docs/experiments/U2_node_protocol.md` is empty, confirming the commit's
  tree carries U2's u-rev-4 text unchanged from its parent `7dfd047`. So citing
  `7473a6f` as "the pinned revision reviewed" for U2 u-rev 4 is not wrong — the
  commit's tree does contain U2's u-rev-4 text, byte-identical to `7dfd047`'s —
  and this is exactly what `wp15b_U2_REVIEW_urev4.md`'s own header already
  discloses and reconciles. Rejected: not a finding, the ambiguity is
  pre-resolved in-tree.
- **Hypothesis: the head's "Five fresh-context REVIEW-designs and two
  DECISION-RED-TEAMs" claim (line 108–109, unchanged since u-rev 1) is a false
  self-completeness count, given this document has now been reviewed by six
  REVIEW-design reports (urev1 through urev5, plus the pre-carve
  `wp15b_design_rev7_REVIEW.md`).** Attempted reproducer: re-derived the same
  count `wp15b_U2_REVIEW_urev5.md`'s own "Verified with no finding" section
  already performed (dispatch-round counting, not per-target counting) —
  `ec8f7fb→182f389`, `182f389→7ad466b`, `7ad466b→f762c9a→64af80c→2d07ff6→
  d94dc0a` (the pre-carve rounds this sentence is actually describing — it is
  head apparatus describing the pre-carve design's review history, not this
  unit's own post-carve confirmation passes) plus the M5 DECISION-RED-TEAM and
  the five-matrix DECISION-RED-TEAM round. This lands on 5 REVIEW-design + 2
  DECISION-RED-TEAM exactly as stated, and post-carve confirmation passes
  (urev1–5) are a separate, later-named count elsewhere in the document ("eight
  prior review rounds," OPEN bullet). Already investigated and not charged by
  the prior review for the same reason; I could not falsify it either. Rejected.
- **Hypothesis: the strike at lines 45–48 leaves "D-309 is still where a reader
  is sent" as an inaccurate pointer, since D-309 records only the verdict
  counts and not a full account of the review's content.** Attempted
  reproducer: `sed -n '655,663p' docs/decisions.md` — D-309 states the SHA, the
  verdict (7 BLOCKING/7 MAJOR/9 MINOR), the closing rationale, and names
  `wp15b_design_rev7_REVIEW.md` as the underlying report by implication (it is
  the review D-309 describes). The struck sentence's original claim was
  narrowly about *counts*, not full content, and D-309 is indeed their home
  (alongside U2-A, which is what falsifies "only"). No finding.
