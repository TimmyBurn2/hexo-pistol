# REVIEW-design (re-review) — WP-1.5b unit U1, `docs/experiments/U1_gate_supersession.md` u-rev 5

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
re-reviewer. The dispatching session repaired U1; it did not review it. Reports
live in the tree rather than in a scratchpad because finding IDs are cited
across units.
-->

## Header

- **Pinned revision:** `2bc4170a96ec03a04fa34837a958dd386f56d268` (matches the SHA given in the dispatch).
- **Matches HEAD:** YES at entry and at exit — `git rev-parse HEAD` returned `2bc4170a96ec03a04fa34837a958dd386f56d268` both times; `git diff 2bc4170a96ec03a04fa34837a958dd386f56d268 HEAD -- docs/experiments/U1_gate_supersession.md` is empty at both checks. The subject did not move during this review.
- **Document:** `docs/experiments/U1_gate_supersession.md`, u-rev 5, 335 lines (`wc -l`, MEASURED — unchanged from u-rev 4's 335).
- **Prior report:** `docs/experiments/wp15b_U1_REVIEW_urev4.md`, pinned `1964026`, **FAIL — 0 BLOCKING, 1 MAJOR, 0 MINOR**, against u-rev 4. MAJOR 1 charged a false cardinality claim ("the one PASS in this work package") restated twice, when at least three PASSes were landed in WP-1.5b before that text was written.
- **Reviewer context:** fresh — no prior turns in this conversation touched the pistol repo before this review.
- **Scope, as given:** (a) a false or drifted claim in surviving text, including any claim this round's own edits introduced; (b) a normative claim that lost its home to a strike (D-346's flip clause); (c) a normative error. The absence of struck text is not a finding; a pointer is not a finding; gate 15's territory (head/foot u-rev label agreement, self-counts) is not re-derived (covered by `tools/label_consistency_check.sh`).

## VERDICT: **PASS**

Derivation: this report's own `# FINDINGS` section is empty and contains zero `### BLOCKING` / `### MAJOR` / `### MINOR` headings.

---

# FINDINGS

(none)

---

# VERIFIED WITH NO FINDING

**1. The u-rev-4→5 diff is exactly what the NO-AUTHORING constraint requires: word-level scoping corrections, zero new sentences.**

```
$ git show 8d3641f -- docs/experiments/U1_gate_supersession.md
```
shows four hunks, all mechanical:
- head label `**u-rev 4.**` → `**u-rev 5.**`
- site 1 (struck-block explanation, ~line 40): `named the one\nPASS in this work package nowhere at all` → `named\nU1's own PASS nowhere at all`
- site 2 (U1-A table row, line 58): `which is how the one PASS in this work package went uncited` → `which is how U1's own PASS went uncited`
- foot label `*U1, u-rev 4.*` → `*U1, u-rev 5.*`

No line was added or removed (`wc -l` is 335 before and after); every changed line is a substitution inside an existing sentence. This matches the commit message's own claim ("No new sentences authored, only a scoping word-substitution at each site") and satisfies the dispatch's diff-shape check.

**2. "U1's own PASS" is true and unambiguous — exactly one PASS report exists for U1.**

```
$ ls docs/experiments/ | grep -i 'U1_REVIEW'
wp15b_U1_REVIEW.md
wp15b_U1_REVIEW_urev2.md
wp15b_U1_REVIEW_urev4.md
```
`wp15b_U1_REVIEW.md` (u-rev 1) is FAIL, `wp15b_U1_REVIEW_urev4.md` (u-rev 4) is FAIL 1 MAJOR (the report this round answers), and `wp15b_U1_REVIEW_urev2.md` is the sole PASS. So "U1's own PASS," unlike the struck "the one PASS in this work package," refers to exactly one verified, unambiguous artefact and carries no cross-unit cardinality claim at all — it fully discharges MAJOR 1's fix scope, which explicitly offered "U1's own PASS" as an acceptable replacement.

**3. Both repaired sentences remain true under the new wording, checked in context.**

Site 1 ("...a head describing this unit as awaiting that review was false from `f81706a` onward and named U1's own PASS nowhere at all"): the struck u-rev-1/2 text it describes (`~~This unit has been reviewed, at u-rev 1, and failed … unreviewed until a fresh REVIEW-design runs against u-rev 2.~~`) indeed never mentions any PASS verdict — it only asserts an "unreviewed" status. True.

Site 2 ("It carried no row for them at all until u-rev 4, which is how U1's own PASS went uncited by the unit that earned it"): already independently confirmed in the u-rev-4 review (its VERIFIED item 5, checking `f81706a`'s copy of U1-A) that the table carried no row for U1's own u-rev 1/2 reviews before u-rev 4. Unchanged this round. "The unit that earned it" = U1 itself, the document's own subject — consistent and non-circular.

**4. D-332 (R17) self-disclosure is accurate.** Commit message: "R17 (D-332): second unit of a serial round, no cross-unit citations touched."

```
$ git log --oneline -4 -- docs/experiments/
56523b2 ... U4 reaches u-rev 10 ...
a10314f ... U3 reaches u-rev 9 ...
8d3641f ... U1 reaches u-rev 5 ...
8eef276 ... U2 reaches u-rev 8 ...
```
Order confirmed: U2 (`8eef276`) → U1 (`8d3641f`, this unit, second) → U3 (`a10314f`) → U4 (`56523b2`). "Second unit of a serial round" is accurate. And the diff (finding 1 above) touches no cross-unit citation — U1_gate_supersession.md's only mentions of another unit are in U1-B ("U2's IMPL," "U2's node protocol"), which is prose about build order, not a D-311-form citation of a specific claim at a specific u-rev, and is byte-unchanged by this round.

**5. No other stale self-referential claim about this document's own review status exists at HEAD.**

```
$ grep -n "REVIEWED\|awaiting\|has been reviewed\|NOT BEEN\|outstanding\|not landable" docs/experiments/U1_gate_supersession.md
```
returns: the struck quotation itself (inert, inside `~~...~~`), "NO REVIEW-design HAS RUN AGAINST THIS u-rev" (line 31), and "That review is outstanding and this unit is not landable while it is" (line 73) — both current and true, since no `wp15b_U1_REVIEW_urev5.md` exists in the tree yet:
```
$ ls docs/experiments/ | grep -i 'U1_REVIEW'
wp15b_U1_REVIEW.md
wp15b_U1_REVIEW_urev2.md
wp15b_U1_REVIEW_urev4.md
```
This review is the first one dispatched against u-rev 5, so these claims are not yet stale as of the pinned revision.

**6. D-309 citation (line 16) remains accurate.** `grep -n '^D-309:' docs/decisions.md` confirms D-309 records revision 7 FAILED with 7 BLOCKING / 7 MAJOR / 9 MINOR, matching U1-A's table row 3 verbatim and the head's characterization of D-309 as "that review's home." Unchanged this round; re-checked for drift, none found.

**7. §4.1/§4.2 MEASURED test counts re-taken independently against HEAD** (unchanged by this round, carried from u-rev 4's already-passed content):
```
$ cargo test -p pistol-cli --test solver_edge_check_tests -- --list 2>&1 | tail -3
the_shipped_workspace_has_no_normal_edge_on_the_solver: test

11 tests, 0 benchmarks

$ cargo test -p pistol-cli --test solver_link_check_tests -- --list 2>&1 | tail -3
no_solver_source_reaches_any_shipped_binary_of_this_workspace: test

19 tests, 0 benchmarks
```
11 and 19 confirmed, matching the document's claims and the u-rev-4 review's own re-measurement. No drift.

**8. No strike touched by this round, so D-346's flip clause (scope b) has no new instance to check.** The only strike in the document (the struck u-rev-1/2 paragraph) was executed at u-rev 4 and independently verified against D-346's shape by the prior review (its VERIFIED item 3); this round's diff does not touch the struck span itself, only the unstruck explanatory sentence around it.

---

# REJECTED, WITH THE ATTEMPTED REPRODUCER

**Candidate: "false from `f81706a` onward" (site 1, unchanged surviving text) understates when the paragraph actually became false — it was already false at `e3f0bc3`, two commits earlier.**

This exact candidate was raised and rejected by the u-rev-4 review (its "REJECTED" section, first entry), with the same reproducer:
```
$ git log --oneline --graph 1964026 -- docs/experiments/U1_gate_supersession.md docs/experiments/wp15b_U1_REVIEW_urev2.md
* 1964026 ... U1 reaches u-rev 4 ...
* f81706a ... U1's foot label reaches u-rev 3 ...
* 3543a7f ... the carve-provenance clause said revision 7 was NEVER REVIEWED ...
* e3f0bc3 ... U1's u-rev 2 re-review lands and it is the session's first PASS ...
* 5baea10 ... U1 reaches u-rev 2 ...
```
Re-run against current HEAD and unchanged: `e3f0bc3` precedes `f81706a`, and the sentence is unaffected by this round's edits (the substitution only touches the clause after "and named"). Re-rejected on the same grounds the prior review gave: "false from `f81706a` onward" is literally true (it asserts falsity held from that point on, not that `f81706a` was the origin), and no downstream claim in the document depends on `f81706a` being the first point of falsity. Not a new defect introduced by u-rev 5's edit, and not re-chargeable as a repeat of an already-adjudicated non-finding.

**Candidate: does "U1's own PASS" silently drop the broader point the struck cardinality claim was reaching for (that U1's PASS specifically went uncited among several project PASSes)?**

Attempted reproducer: compared the corrected sentences' remaining content against the fix scope suggested by the u-rev-4 review ("drop the cardinality claim... or replace it with a verified one ('U1's own PASS,'...) "). Both sentences retain their full original point — that the document's own head/table failed to cite U1's PASS — and only the false global-cardinality qualifier is removed. Nothing referenced by other text in the document (e.g., D-342's citation, U1-A's structure) depends on the dropped "in this work package" scoping. Rejected: no loss of load-bearing content, and the exact repair the prior review offered as sufficient was the one applied.
