# REVIEW-design (RE-REVIEW, confirmation pass against u-rev 8) — `docs/experiments/U2_node_protocol.md` u-rev 8

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer (only HTML-escaped angle brackets and ampersands restored:
`&gt;`/`&lt;`/`&amp;` to `>`/`<`/`&`). The dispatching session repaired U2; it
did not review it. Reports live in the tree rather than in a scratchpad
because finding IDs are cited across units.
-->

## Header

- **Pinned revision:** `2bc4170a96ec03a04fa34837a958dd386f56d268` (full SHA; confirmed by `git rev-parse HEAD` = `2bc4170a96ec03a04fa34837a958dd386f56d268`, matching the SHA given in the dispatch prompt).
- **Match with HEAD at entry:** YES.
- **Match with HEAD at exit:** YES — re-checked; unchanged.
- **Subject moved from the pin?** NO — `git diff 2bc4170 HEAD -- docs/experiments/U2_node_protocol.md` is empty at both entry and exit checks.
- **Document + u-rev + measured size:** `docs/experiments/U2_node_protocol.md`, u-rev 8 (head table row 8: `| **8** | *this text* | **NOT YET REVIEWED** | — |`), **946 lines** (`wc -l`, MEASURED).
- **This round's own commit:** `8eef276` (landed on `dev`, three commits behind HEAD `2bc4170`; the three intervening commits touch U1/U3/U4/`docs/decisions.md` only — confirmed by `git diff 8eef276 2bc4170 -- docs/experiments/U2_node_protocol.md` returning empty). The earlier BLOCKING-discharging restore is `ec84bc6`, landed before `8eef276` and before this repair round started.
- **Prior reports read:** `wp15b_U2_REVIEW.md` (u-rev 1, FAIL), `wp15b_U2_REVIEW_urev2.md` (u-rev 2, FAIL), `wp15b_U2_REVIEW_urev3.md` (u-rev 3, FAIL — F5 MAJOR/F6 MINOR), `wp15b_U2_REVIEW_urev4.md` (u-rev 4, FAIL — G1/G2 MINOR — not re-read in full, cited via the subject document's own pointers), `wp15b_U2_REVIEW_urev5.md` (u-rev 5, FAIL — H1 MAJOR/H2 MINOR — cited via pointers), and **`wp15b_U2_REVIEW_urev7.md` in full** (u-rev 7, FAIL — 1 BLOCKING/1 MAJOR, the review this u-rev-8 repair answers). No report exists for u-rev 6. No report exists yet for u-rev 8 — this review is u-rev 8's first.
- **Context freshness:** fresh context, no prior turns. Read `CLAUDE.md` whole; `docs/decisions.md` D-331 (R15), D-332 (R17), D-336, D-346 and D-349 in full (via `grep -n`); the whole subject document (both halves, offset 0 and offset 784); the full `git diff ec84bc6 8eef276 -- docs/experiments/U2_node_protocol.md` (this session's own repair, isolated from the earlier `ec84bc6` restore); `git log --oneline -12 -- docs/experiments/U2_node_protocol.md`; and targeted `git diff`/`grep` reproducers for every claim checked below.
- **Scope, as given:** confirmation pass under STRIKE-AND-DISPOSE (D-346). The absence of struck text is not a finding; a pointer is not a finding; gate 15 (head/foot u-rev agreement, self-counts) is not re-derived — `tools/label_consistency_check.sh`'s territory. Chargeable findings limited to: (a) a false or drifted claim in surviving text, including any claim this round's own edits introduced; (b) a normative claim that lost its home to a strike (D-346's flip clause); (c) a normative error.

---

## VERDICT: FAIL — 0 BLOCKING, 1 MAJOR, 0 MINOR

Derived from this report's own finding headings, once landed, by:
```
$ grep -c '^### Finding.*(BLOCKING)' <this-report>.md   # -> 0
$ grep -c '^### Finding.*(MAJOR)' <this-report>.md      # -> 1
$ grep -c '^### Finding.*(MINOR)' <this-report>.md      # -> 0
```
One MAJOR heading (Finding 1, below), no BLOCKING, no MINOR.

---

# FINDINGS

## MAJOR

### Finding 1 (MAJOR) — a sibling of the exact defect this round repaired survives, unrepaired: "THIS u-rev — u-rev 4 — HAS NOT BEEN REVIEWED" is false, and the document's own next REVIEW STATUS block, 11 lines below, proves it

**Claim, U2 line 130 (REVIEW STATUS block for the u-rev-3→4 transition, unchanged since u-rev 4, `7dfd047`):**

> **REVIEW STATUS — u-rev 3 WAS REVIEWED AND FAILED; u-rev 4 IS THE REPAIR.**
> `docs/experiments/wp15b_U2_REVIEW_urev3.md`, REVIEW-design, fresh context,
> against `d85b049` — VERDICT FAIL, 1 MAJOR (F5), 1 MINOR (F6). **THIS
> u-rev — u-rev 4 — HAS NOT BEEN REVIEWED**, and u-rev 3's review does not
> transfer to it: an amendment reopens the review, however small the diff
> (D-311).

**Contradicting evidence, reproducer — the very next REVIEW STATUS block, 11 lines later, in the same document:**
```
$ sed -n '141,146p' docs/experiments/U2_node_protocol.md
**REVIEW STATUS — u-rev 4 WAS REVIEWED AND FAILED ON TWO MINORS; u-rev 5 IS THE
REPAIR.** `docs/experiments/wp15b_U2_REVIEW_urev4.md`, REVIEW-design (re-review),
fresh context, against **`7473a6f`** — **VERDICT FAIL, 0 BLOCKING, 0 MAJOR, 2 MINOR**.
It confirmed F5's substitution structural on the merits, having diffed every
intermediate revision, and F6 cleanly discharged.
```
`wp15b_U2_REVIEW_urev4.md` exists on disk, is pinned at `7473a6f` (u-rev 4's own reviewed revision), and reaches VERDICT FAIL. So u-rev 4 unambiguously **was** reviewed — the document's own apparatus 11 lines below the "HAS NOT BEEN REVIEWED" sentence names the report and its verdict.

**When this went stale, reproducer:**
```
$ git diff d85b049 7dfd047 -- docs/experiments/U2_node_protocol.md | grep -n "HAS NOT BEEN REVIEWED"
141:+u-rev — u-rev 4 — HAS NOT BEEN REVIEWED**, and u-rev 3's review does not transfer

$ git diff 7dfd047 f0ae14c -- docs/experiments/U2_node_protocol.md | sed -n '/REVIEW STATUS — u-rev 4/,+6p'
+**REVIEW STATUS — u-rev 4 WAS REVIEWED AND FAILED ON TWO MINORS; u-rev 5 IS THE
+REPAIR.** `docs/experiments/wp15b_U2_REVIEW_urev4.md`, REVIEW-design (re-review),
+fresh context, against **`7473a6f`** — **VERDICT FAIL, 0 BLOCKING, 0 MAJOR, 2 MINOR**.
```
The sentence was written true, at u-rev 4 (`7dfd047`). It went false the moment u-rev 5 (`f0ae14c`) landed the "u-rev 4 WAS REVIEWED AND FAILED" block immediately below it — that commit's diff *adds* the new block but never touches the older sentence. It has since survived four more repair rounds (`f0ae14c`, `3543a7f`, `ecb0341`, `ec84bc6`, `8eef276`) unrepaired, none of which is HEAD's u-rev 8, which is itself the round with a direct mandate to sweep for exactly this claim-shape.

**Why this round should have caught it.** This round's own commit (`8eef276`) fixed the structurally identical sibling — "THIS u-rev — u-rev 5 — HAS NOT BEEN REVIEWED" — by deletion, in direct answer to `wp15b_U2_REVIEW_urev7.md` Finding 2. Finding 2 itself named the general pattern: *"a stale review-status claim propagating unrepaired past the round that made it stale."* That is precisely this instance, at an earlier site the repair round did not visit. `grep -n "HAS NOT BEEN REVIEWED" docs/experiments/U2_node_protocol.md` at authoring time would have surfaced both.

**Why it breaks:** this is the same class D-331/D-311 and the u-rev-7 review's own Finding 2 target — a claim a document makes about its own review history, false, self-contradicted a few lines below in the same document. A reader (or WP-1.6 implementer) hitting line 130 first is told u-rev 4 is an open, unreviewed repair when the document's own next paragraph shows it was reviewed and failed on two MINORs three u-revs ago.

**Severity:** MAJOR — matching Finding 2's own severity for the identical claim-shape (a false statement about which u-revs were reviewed). It does not touch §5's design content or the F5/F6/G1/G2/H1/H2 repair chain's soundness; it misstates the document's own review status, which is precisely the fact this REVIEW STATUS apparatus exists to keep accurate.

**Fix scope:** one sentence, one site (line 130). Delete "**THIS u-rev — u-rev 4 — HAS NOT BEEN REVIEWED**" (the block 11 lines below already states, correctly, that u-rev 4 was reviewed and what its findings were — D-331 already forbids the restatement this sentence amounts to now that it is outdated), matching exactly the fix this round already applied to the sibling u-rev-5 sentence.

---

# VERIFIED WITH NO FINDING

- **HEAD/pin agreement.** `git rev-parse HEAD` returns `2bc4170a96ec03a04fa34837a958dd386f56d268`, matching the SHA named in the dispatch prompt; `git diff 2bc4170 HEAD -- docs/experiments/U2_node_protocol.md` is empty.
- **Diff shape of this round's own repair (`ec84bc6` → `8eef276`), isolated from the earlier `ec84bc6` restore.** `git diff ec84bc6 8eef276 -- docs/experiments/U2_node_protocol.md` shows exactly four hunks: (1) the head provenance-sentence label bump "u-rev 7" → "u-rev 8" (LABEL DISCIPLINE apparatus, expected to change every u-rev, not carved content, carries no marker requirement); (2) the head u-rev table's row 7 completion (SHA `1964026`, report path, verdict `FAIL`) plus a fresh row-8 placeholder, in the exact tabular pattern the document has used at every prior u-rev bump (rows 1–6 were filled in identically as each review landed) — mechanical bookkeeping, not new prose/analysis; (3) deletion of "THIS u-rev — u-rev 5 — HAS NOT BEEN REVIEWED" (answers Finding 2, verified below); (4) the footer u-rev label bump "u-rev 7" → "u-rev 8". No hunk touches §2, §3, §5 or §14 (carved content), and no hunk adds a new normative or analytical sentence anywhere in the document. The NO-AUTHORING constraint (deletions and minimal correction only) holds for this round's own diff.
- **Finding 2 of `wp15b_U2_REVIEW_urev7.md` is correctly discharged.** The exact sentence quoted in that finding ("THIS u-rev — u-rev 5 — HAS NOT BEEN REVIEWED") is absent from the current file (`grep -n "u-rev 5 — HAS NOT" docs/experiments/U2_node_protocol.md` returns nothing), and the fix taken (deletion) matches option (a) of that finding's own stated fix scope verbatim.
- **The `ec84bc6` restore (Finding 1 of `wp15b_U2_REVIEW_urev7.md`, predates this repair round) is intact and matches D-349's description.** `sed -n '871,883p'` and `sed -n '904,927p'` of the current file show the handoff bullet restoring "168 030 comparisons … with the `Impossible` branch reached at every budget" verbatim from `3543a7f`, annotated `*(RESTORED VERBATIM FROM 3543a7f under D-346's flip clause: … a NORMATIVE measurement IMPL depends on, not a document-state claim, so D-346 never licensed striking it. `wp15b_U2_REVIEW_urev7.md` Finding 1 (BLOCKING) is the instance; this is its restore. …)*`, and the OPEN bullet correspondingly restored and annotated the same way, still registering the 168 030/343 344 disagreement as OPEN, resolvable "only by RE-RUNNING the equivalence probe." This matches `docs/decisions.md` D-349's description of the disposition exactly (`grep -n '^D-349:' docs/decisions.md`, MEASURED): restored verbatim from the pre-strike SHA, unmarked, unreconciled, problem left OPEN rather than resolved. Finding 1 (BLOCKING) is fully discharged: the surviving text no longer silently asserts the non-vacuity property as settled fact — the restore's own annotation and the paired OPEN bullet now flag exactly the gap Finding 1 identified, satisfying option (a) of that finding's fix scope ("restore a number … explicitly marked OPEN"). This restore predates u-rev 8 and is out of this round's own diff, but its continued presence and correctness at HEAD is confirmed.
- **No other stale "HAS NOT BEEN REVIEWED" / "NOT YET REVIEWED" / "ANSWERS NO REVIEW REPORT" self-referential claim about u-rev 6, 7 or 8 is false.** `grep -n "HAS NOT BEEN REVIEWED\|NOT YET REVIEWED\|NOT REVIEWED\|ANSWERS NO REVIEW\|dispatched against\|UNREVIEWED"` over the current file returns: line 34 (u-rev 6, "NOT REVIEWED — no round was dispatched against it" — still true, no `wp15b_U2_REVIEW_urev6.md` exists); line 36 (u-rev 8 row, "NOT YET REVIEWED" — true, this review is its first); line 130 (u-rev 4 — **charged above as Finding 1**); line 152 ("u-rev 7 ANSWERS NO REVIEW REPORT. No round has been dispatched against u-rev 6." — a historical statement about u-rev 7's own basis, still accurate, not stale); line 888 ("this text is UNREVIEWED at this u-rev" — true, u-rev 8 unreviewed). Only line 130 is false; all others hold.
- **`docs/decisions.md` D-346 and D-349 text, MEASURED via `grep -n`, matches the subject document's citations of them** (the flip-clause wording "a strike that removes a NORMATIVE claim is a defect — restore it and report the instance," and D-349's instance description) — no drift between the ADR and the document's paraphrase/quotation of it.

# REJECTED, WITH THE ATTEMPTED REPRODUCER

- **Hypothesis: the head table's row-7 completion and row-8 addition (line 32–36) are themselves "new sentences" and violate the NO-AUTHORING constraint.** Attempted reproducer: compared against `git diff d85b049 7dfd047`, `git diff 7dfd047 f0ae14c`, and `git diff f0ae14c 3543a7f` (all prior u-rev bumps) — each shows the identical mechanical pattern of converting the previous u-rev's "*this text* | NOT YET REVIEWED" placeholder row into a completed historical row once a review report exists, plus a fresh placeholder row for the new u-rev. This pattern was never challenged by any prior review (`wp15b_U2_REVIEW_urev2.md` through `_urev7.md` all post-date at least one such row-completion and none charged it). It is factual tabular bookkeeping (a SHA, a file path, a verdict word) in a pre-existing table structure, not authored analytical or normative prose. Rejected: not a finding.
- **Hypothesis: `docs/decisions.md` D-349's restore is itself out of scope for this review since it landed at `ec84bc6`, before this repair round.** Attempted reproducer: `git log --oneline -12 -- docs/experiments/U2_node_protocol.md` confirms `ec84bc6` precedes `8eef276` (the u-rev-8 landing commit) in history, and the dispatch prompt explicitly directs confirming this restore's presence and adequacy as part of this review's task, distinct from crediting or charging it against this round's own diff. Not treated as a chargeable finding against u-rev 8's own repair; only verified as intact and adequate, per the dispatch instructions. Rejected as a finding, confirmed as a verification item instead.
- **Hypothesis: the U2-A lineage table (lines 184–195) or the "Five fresh-context REVIEW-designs and two DECISION-RED-TEAMs" self-completeness claim (line 109) drifted at u-rev 8.** Attempted reproducer: `git diff ec84bc6 8eef276 -- docs/experiments/U2_node_protocol.md` shows no hunk touching lines 109 or 184–195; these were already investigated and rejected by `wp15b_U2_REVIEW_urev7.md`'s own "REJECTED" section and are untouched by this round. Rejected: no new drift, out of this round's diff.

---

**Summary for the dispatching session:** the BLOCKING finding from `wp15b_U2_REVIEW_urev7.md` (the 168 030 non-vacuity figure) is fully and correctly discharged — restored verbatim under D-346's flip clause at `ec84bc6`, matching D-349, predating this repair round. The MAJOR finding from that review (the stale "u-rev 5 HAS NOT BEEN REVIEWED" sentence) is correctly fixed by deletion in this round's own commit `8eef276`. This round's diff is pure deletion plus mechanical label/table bookkeeping — no new authored prose. However, an unflagged sibling of the exact same defect class survives at line 130 ("THIS u-rev — u-rev 4 — HAS NOT BEEN REVIEWED," false since u-rev 5 landed, contradicted 11 lines below in the same document) — this is a fresh MAJOR finding this review is the first to name. **VERDICT: FAIL — 0 BLOCKING, 1 MAJOR, 0 MINOR.**
