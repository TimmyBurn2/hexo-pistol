# REVIEW-design (re-review) — WP-1.5b unit U3, `docs/experiments/U3_tier_t.md` u-rev 9

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer. The dispatching session repaired U3; it did not review it. Reports
live in the tree rather than in a scratchpad because finding IDs are cited
across units.
-->

## Header

- **Pinned revision:** `2bc4170a96ec03a04fa34837a958dd386f56d268` (`git rev-parse HEAD` confirmed this exact SHA, as instructed).
- **Document:** `docs/experiments/U3_tier_t.md`, **u-rev 9**, landed at `a10314f93989a98a5113af070d166e729b9aa61d`. MEASURED, `git log --oneline -1 -- docs/experiments/U3_tier_t.md` → `a10314f`, an ancestor of HEAD with no further changes to the file (`git diff a10314f HEAD -- docs/experiments/U3_tier_t.md` → empty).
- **Prior report read:** `docs/experiments/wp15b_U3_REVIEW_urev8.md` (u-rev 8, `173397d`, **FAIL — 0 BLOCKING, 2 MAJOR (F, G), 0 MINOR**) — the review this u-rev-9 repair claims to answer.
- **Diff examined:** `git diff 173397d a10314f -- docs/experiments/U3_tier_t.md`, full hunk-by-hunk (three hunks, reproduced below where load-bearing).
- **Context freshness:** FRESH. I did not write this unit, its u-rev 8/9 repairs, or the u-rev-8 review, and had not read any of them before this dispatch.
- **Scope, as given:** (a) a false or drifted claim in surviving text, including any claim this round's own edits introduced; (b) a normative claim that lost its home to a strike; (c) a normative error. The absence of struck text is not a finding; a pointer is not a finding; Gate 15's territory (`tools/label_consistency_check.sh` — head/foot u-rev agreement, the two *anchored* self-count forms: summand lines and backtick group-count lines) is out of scope.
- **Diff-shape check:** pure deletion plus the mechanical u-rev-label bump (8→9, three sites: head, REVIEW STATUS `**u-rev N.**` isn't present but the two content deletions and the footer bump are). MEASURED, `git diff 173397d a10314f -- docs/experiments/U3_tier_t.md | grep -c '^+'` → 3 (the two label-bump lines plus one edited sentence-fragment line), `grep -c '^-'` → 4; every `+` line is either a shortened version of the immediately preceding `-` line (a clause removed) or a bare u-rev-label bump. No new clause, sentence, or claim not already present in u-rev 8 was added. The commit message's "All three fixes are deletions, no new sentences authored" is TRUE.

## VERDICT

**FAIL — 0 BLOCKING, 1 MAJOR, 0 MINOR.**

(Counts by section: `## BLOCKING` has zero `### ` entries; `## MAJOR` has one, `### H`; `## MINOR` has zero.)

u-rev 9's three deletions are each individually correct and answer u-rev 8's review exactly: (1) the head's false "and is that review's home" clause (MAJOR F) is deleted with nothing added, leaving a complete sentence — no navigation stranded; (2) the fourth unstruck instance of "it asserts no universal about its own citations" in U3-Z (MAJOR G) is deleted, leaving a grammatical, complete sentence; (3) the stale "NEITHER u-rev 7 NOR u-rev 8 HAS BEEN REVIEWED" claim — false the moment `wp15b_U3_REVIEW_urev8.md` landed, which the dispatch context flagged for checking — is also correctly identified and deleted. All three are minimal deletions under `D-346`, consistent with the hard NO-AUTHORING constraint.

But the round caught one instance of the "review-of-u-rev-8 landed and changed the facts on the ground" staleness (the `NEITHER…HAS BEEN REVIEWED` sentence) and missed a second, closely related instance triggered by the identical event: the REVIEW STATUS block's narrative — "This document has now failed FOUR consecutive re-reviews," the MAJOR-D/E-only disposition table, and "the review they owe is the architect's to schedule" — was never updated to reflect that the owed review *did* run (against u-rev 8), *did* fail, and is what u-rev 9 itself is repairing. U3-A's lineage table, which the document itself repeatedly calls "where the round record lives," has no row for the u-rev-8 review. This is the same shape as MAJOR G (one instance of a class fixed, a sibling instance of the same class left standing) — occurring again, one level up, inside the very round that fixed the first case of it.

---

# FINDINGS

## BLOCKING

None.

## MAJOR

### H. The REVIEW STATUS block, its disposition table, and the U3-A lineage table were never updated to record that the u-rev-8 review ran and FAILED — the same "review landed, tree not updated" staleness this very round fixed once (the deleted `NEITHER…HAS BEEN REVIEWED` sentence) but missed a second time

**Claim reviewed** (REVIEW STATUS block, lines 113–128, unchanged by the u-rev 8→9 diff except for the one deleted sentence):

> **REVIEW STATUS — u-rev 6 WAS REVIEWED AND FAILED; u-rev 7 IS THE REPAIR, AND THE ROUND STOPPED THERE RATHER THAN DISPATCHING A SIXTH REVIEW.** `docs/experiments/wp15b_U3_REVIEW_urev6.md`… **VERDICT FAIL**, **0 BLOCKING, 2 MAJOR, 0 MINOR**.
>
> **WHY NO SIXTH REVIEW WAS DISPATCHED…** This document has now failed **FOUR** consecutive re-reviews, and **every one of the four MAJORs was manufactured by the previous round's repair** — MAJOR A … MAJOR B … MAJOR C … MAJOR D… **The two findings are repaired because they are FALSEHOODS in the tree and leaving them is not an option; the review they owe is the architect's to schedule.**

Followed by a disposition table (lines 136–139) with exactly two rows, `MAJOR D` and `MAJOR E`, both from `wp15b_U3_REVIEW_urev6.md`, "answered at u-rev 7."

**What is now true and unrecorded.** MEASURED, `wp15b_U3_REVIEW_urev8.md` exists in the tree (`git log --oneline -1 -- docs/experiments/wp15b_U3_REVIEW_urev8.md` → `a10314f93989a98a5113af070d166e729b9aa61d` — the same commit that lands u-rev 9), carries `## VERDICT` **FAIL — 0 BLOCKING, 2 MAJOR (F, G), 0 MINOR**, and its own text states the repair it reviews "manufactured two fresh defects of the exact shape this document has now failed on for four consecutive prior rounds" — i.e., this is now the **FIFTH** consecutive re-review to fail on the "self-completeness claim false in the commit that wrote it" class, not four, and both its findings were manufactured by u-rev 8's own repair, exactly matching the pattern the block's own sentence describes for A–D. u-rev 9 answers F and G by deletion (confirmed above), which is the disposition this document's own established convention (see every prior u-rev's commit, e.g. `a2b50bf` reproduced below) requires be recorded as a new REVIEW STATUS header, a new disposition-table row, and a new U3-A lineage row — and none of the three happened:

- **The header is two generations stale.** It still reads "u-rev 6 WAS REVIEWED AND FAILED; u-rev 7 IS THE REPAIR" instead of recording that u-rev 8 was *also* reviewed and failed, and u-rev 9 is *that* repair.
- **"FOUR consecutive re-reviews" undercounts by (at least) one**, on a document whose review history is precisely the object this sentence is about, in a class (MINOR 8/MINOR 11's "four sites" undercounting eight; B7's "5" vs. six counter-examples; MAJOR-D's blind grep) this project's own decisions.md D-331/D-305 lines exist to name.
- **"The review they owe is the architect's to schedule" is now false as a description of present state** — that review was scheduled, ran, and delivered a FAIL verdict (F, G) already answered by this very commit. The obligation described is discharged, not outstanding, and the sentence gives no indication of this.
- **The disposition table has no row for MAJOR F or MAJOR G.** MEASURED, `grep -n "MAJOR F\|MAJOR G"` and `grep -n "wp15b_U3_REVIEW_urev8"` against `docs/experiments/U3_tier_t.md` both return **zero** hits — the finding IDs and the review file that u-rev 9's own commit message says it is answering are named nowhere in the document itself.
- **U3-A's lineage table has no row for the u-rev-8 review.** MEASURED, `grep -n "REVIEW-design, this unit"` returns exactly five rows (u-rev 2, 3, 4, 6, 5) — none for u-rev 8 — even though the document calls U3-A "where the round record lives" in two places (the u-rev-6→7 disposition-table preamble, line 130–134, and U3-Z's closing bullet, lines 996–998: *"No REVIEW-design has run against this text at THIS u-rev (U3-A, which is where the round record lives…)"*). That second sentence, read at u-rev 9, is accurate about u-rev 9 itself but sits directly beside a table that is missing an entire round's record — the home it points at does not, in fact, hold what the document elsewhere claims it holds.

**Why this is a repeat of the exact defect this round already caught once.** The single event that made the deleted `NEITHER…HAS BEEN REVIEWED` sentence false — `wp15b_U3_REVIEW_urev8.md` landing with a FAIL verdict — is the *same* event that makes "FOUR consecutive re-reviews," the MAJOR-D/E-only disposition table, "the review they owe is the architect's to schedule," and U3-A's missing row all stale. u-rev 9 found and deleted one casualty of that landing and left the rest of the same blast radius untouched, inside the same commit whose whole charter is repairing exactly this class of defect (D-346/D-331) — this is MAJOR G's shape recurring one paragraph away from where it was just fixed.

**Established convention this breaks, for contrast** (`git show a2b50bf -- docs/experiments/U3_tier_t.md`, the u-rev 6→7 transition): that round rewrote the REVIEW STATUS header ("u-rev 5…" → "u-rev 6 WAS REVIEWED AND FAILED; u-rev 7 IS THE REPAIR"), replaced the disposition table's rows wholesale, and added a new U3-A row for the u-rev-6 review, all in the same commit that answered the findings. u-rev 8→9 did none of this for the u-rev-8 review.

**Fix scope.** U3-local, and larger than a pure deletion (this is new content, which may be why the NO-AUTHORING round deferred it): the REVIEW STATUS header needs "u-rev 8 WAS REVIEWED AND FAILED (2 MAJOR: F, G); u-rev 9 IS THE REPAIR"; the disposition table needs an F/G row stating "answered by deletion"; the "FOUR consecutive" count needs correcting; "the review they owe is the architect's to schedule" needs removing or updating (the review already ran); and U3-A needs a new lineage row for `wp15b_U3_REVIEW_urev8.md`. If the round's NO-AUTHORING constraint forbids adding this in the current commit, that constraint itself is what should be named as the reason the update is deferred — silently leaving the block stale is not a neutral option under this document's own D-331/D-346 standard.

---

## MINOR

None.

---

# VERIFIED WITH NO FINDING

- **The three u-rev-8→9 deletions are each individually correct.** (1) Head: "and is that review's home" removed, sentence complete without it, no navigation stranded (D-309 is still named). (2) U3-Z line ~853–854: "it asserts no universal about its own citations" removed; remaining text — "the B7 list is explicitly not a boundary, and the instrument it offers is labelled a finding aid rather than a proof" — is grammatical and complete. (3) REVIEW STATUS: "NEITHER u-rev 7 NOR u-rev 8 HAS BEEN REVIEWED, and u-rev 6's review does not transfer to either" removed; the preceding sentence ("VERDICT FAIL, 0 BLOCKING, 2 MAJOR, 0 MINOR.") stands alone and is unaffected.
- **No new sentence was authored anywhere in the diff.** MEASURED, `git diff 173397d a10314f -- docs/experiments/U3_tier_t.md`: every `+` line is either a shortened remnant of the corresponding `-` line or a bare u-rev-number bump (8→9, two sites: head and footer). No clause appears in a `+` line that was not already present, word-for-word, somewhere in u-rev 8.
- **The struck-text disposal notes from u-rev 8 (`WITHDRAWN AT u-rev 8 under D-346`, `STRUCK AT u-rev 8 under D-346`, at lines 138 and 403) are untouched and remain internally consistent** — they describe u-rev 8's own action and are not affected by u-rev 9's further deletions.
- **The removed "u-rev 6's review does not transfer to either" clause did not strand a normative claim.** The rule it restated (reviews of superseded revisions do not transfer) has its home in CLAUDE.md and is independently and correctly restated in U3-Z's closing bullet ("a review of a superseded revision does not transfer," line 999), so no normative claim lost its only home to this strike.
- **No live occurrence of the unscoped "asserts no universal about its own citations" survives anywhere in the tree.** MEASURED, `grep -n "asserts no universal" docs/experiments/U3_tier_t.md` → three hits: line 103 (a quotation, inside the u-rev-8 preamble, of the claim *being struck* — historical, not a live assertion), line 107 (the deliberately-kept, narrower, true claim scoped to "HOW MANY SITES OBEY THE RULE" — out of scope per the dispatch), and line 138 (struck-through with `~~…~~` markup, historical). No fourth live instance remains.
- **The footer's u-rev bump is self-consistent with the head.** MEASURED, footer reads "*U3, u-rev 9…*" matching the head's "**u-rev 9.**" (Gate 15's territory, not re-derived, but confirmed on its face.)
- **The u-rev 8 review's "VERIFIED WITH NO FINDING" items are unaffected by this diff** — none of them concerned text this diff touches (the census block, U3-T/U3-M, the finding-aid grep's seven hits, the two-citation exception list) — spot-checked: MEASURED, `git diff 173397d a10314f -- docs/experiments/U3_tier_t.md` touches only the three hunks already covered above; the `BEGIN CENSUS TABLE`/`END CENSUS TABLE` block and `U3-M`/`U3-T` sections remain byte-unchanged from u-rev 8.
- **R17/D-332 compliance for this serial round.** MEASURED, `git diff 173397d a10314f -- docs/experiments/U3_tier_t.md` touches no `**U1**`, `**U2**`, or `**U4**` citation line; the commit message's "no cross-unit citations touched" is accurate, and the sibling units' u-rev bumps this round (U1→5 at `8d3641f`, U2→8 at `8eef276`, U4→10 at `56523b2`) landed in a serial sequence around this commit with no citation in U3 pointing at a stale sibling state.

# REJECTED, WITH THE ATTEMPTED REPRODUCER

- **"The head's `u-rev 8 ANSWERS NO REVIEW REPORT. No round has been dispatched against u-rev 7.` sentence (lines 98–99, unchanged by this diff) is now stale, since a review (of u-rev 8) has since run."** Attempted: checked whether this reads as a live universal about current review status. REJECTED: it is a historical claim about u-rev 8's own authoring circumstances ("at the time u-rev 8 was written, it answered no review report, because none existed against u-rev 7") — true then and still true as history; it makes no claim about anything after u-rev 8 was authored, unlike the REVIEW STATUS block's forward-looking "the review they owe is the architect's to schedule."
- **"D-346's flip clause is triggered here because the strike of `NEITHER…HAS BEEN REVIEWED` removed a normative claim."** Attempted: checked whether "u-rev 6's review does not transfer to either" (part of the same struck sentence) is normative rather than self-referential-status. REJECTED: D-346 explicitly classes "which of its own revisions were reviewed and with what verdict" as struck-shaped, and the transfer-rule clause is a restatement of a rule whose home is CLAUDE.md and which U3-Z restates independently at line 999 — not stranded, and not the kind of "binding rule" D-346's flip clause protects since it carries no unique content.
- **"MAJOR H should be BLOCKING, not MAJOR, since it concerns the document's core self-tracking claim-home mechanism (D-331)."** Attempted: compared against this document's own severity precedent for the identical defect class. REJECTED: every prior instance of "a self-referential completeness/history claim in this document went stale and was not caught" (MAJOR A, B, C, D, and u-rev 8's F, G) was scored MAJOR, never BLOCKING, across five prior reviews of this same document; nothing distinguishes this instance's severity from that established pattern.
- **"U3-A's lineage table is itself in violation of D-331 by restating review verdicts a second time (rather than the ADR/report being their only home)."** Attempted: checked whether the existing five rows (and the missing sixth) breach the claim-home law on their own terms. REJECTED: `wp15b_U3_REVIEW_urev6.md`'s own review already examined this exact question (per `wp15b_U3_REVIEW_urev8.md`'s equivalent rejection for U3-A's revision-7 row) and D-331 explicitly carves out "lineage tables" as pointer-carrying status matter; the defect charged in MAJOR H is that a row is *missing*, not that the existing rows are non-compliant restatements.
