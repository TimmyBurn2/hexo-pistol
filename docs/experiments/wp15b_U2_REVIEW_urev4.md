# REVIEW-design (CONFIRMATION RE-REVIEW) — `docs/experiments/U2_node_protocol.md` u-rev 4

**Pinned revision: `7473a6f`.** This is a LATER commit that touched a different
unit (U3, reaching u-rev 5); U2's own content last changed at `7dfd047`
("U2 reaches u-rev 4"). `git diff 7dfd047 7473a6f -- docs/experiments/U2_node_protocol.md`
is empty — the two SHAs name byte-identical U2 text, so reviewing at either is
equivalent for this document.

**At review finish:** `git rev-parse HEAD` → `d59f0def7c1c7d9f37a27ebc257f738f355b09fd`.
HEAD advanced three commits past the pin during this review (`0a07395`, `53c0c0b`,
`d59f0de` — D-332/R17, a U4-R red-team round, and a Track-C REVIEW-impl, none
touching U2). `git diff 7473a6f HEAD -- docs/experiments/U2_node_protocol.md` is
**empty** — U2's text is unchanged at current HEAD. `git status --porcelain` at
review start showed one untracked file, `docs/experiments/matrix_U4R_REDTEAM.md`
(unrelated work, not part of this unit or its carve); it is unchanged and back in
place at review finish (see GATE section — it was moved aside and restored during
this review to isolate a gate failure, never edited).

**Fresh context:** no prior turns this conversation; read CLAUDE.md whole,
`docs/decisions.md` D-331 (D-305/D-310/D-311 also read), then the whole of
`docs/experiments/U2_node_protocol.md`, then `wp15b_U2_REVIEW_urev3.md` and
`wp15b_U2_REVIEW_urev2.md`.

**Scope:** judge u-rev 4's substitution (delete the hand-enumerated "exceptions"
list, replace with a `CARVE-EXCEPTION` marker rule + derived grep) on its merits
against F5/F6 of `wp15b_U2_REVIEW_urev3.md`, plus the seven named checks in the
dispatch brief, plus a general sweep for D-331 violations. Per this prompt's
REVIEWER SCOPE GUARD, a pointer is not charged as a finding; only text carrying
an independent, falsifiable claim is.

---

## VERDICT: FAIL — 0 BLOCKING, 0 MAJOR, 2 MINOR

---

## Finding G1 (MINOR) — the "four [markers] already existed" fact is asserted twice, with no pointer between the two sites

**Claim 1, U2 lines 35–38 (head):** "It rewrites the head's exceptions paragraph
and adds `CARVE-EXCEPTION` markers at the five sites the exceptions occur at —
**four of those markers already existed in another wording and one is new.**"

**Claim 2, U2 line 118 (REVIEW STATUS table, F5 row):** "the missing disclosure
is added at its own site, `§5.4`'s architect-ruling paragraph, as a
`CARVE-EXCEPTION` marker. **The four disclosures that already existed carry the
same token,** so one command enumerates every one of them."

**Reproducer:**
```
$ grep -n "five sites\|four of those markers\|four disclosures\|one is new" docs/experiments/U2_node_protocol.md
36:the five sites the exceptions occur at — four of those markers already existed
37:in another wording and one is new. **The markers sit inside §2.2, §5.3, §5.4 and
118:| **F5** (MAJOR) | ... The four disclosures that already existed carry the same token, so one command enumerates every one of them |
```
Both statements independently assert the same fact — that 4 of the 5
`CARVE-EXCEPTION` sites pre-date u-rev 4 in some form (B5/§2.2, F1/§5.3,
F2/§5.3, the D-310 seam/U2-M) and 1 (R5/§5.4) is new. I verified the fact itself
is TRUE against the diff (`git diff d85b049 7dfd047 -- docs/experiments/U2_node_protocol.md`):
§2.2's marker text and the U2-M/§12 marker text did not exist AT THOSE SITES
before u-rev 4, but equivalent prose existed in the now-deleted head "Four
exceptions" paragraph, which is a defensible reading of "in another wording";
§5.3's two markers (F1, F2) existed at-site already, gaining only the token.
So neither copy is FALSE. The finding is the duplication itself, not an error
in either copy.

**Why it breaks:** this is a live instance of the shape D-331 names — "every
claim has exactly ONE HOME... every other occurrence is a POINTER" — inside the
very paragraph pair that argues the OLD hand-enumerated list was replaced
*because* a hand-maintained fact drifts. Neither instance here is a `§n`
reference, a file reference, an ADR reference, or a marked quotation (D-331's
three exempted forms), so this is not a pointer under the prompt's scope guard —
it is two independent assertions of the same specific historical count. If a
later session discovers the "four already existed / one new" breakdown was
itself wrong (e.g. a marker's provenance gets re-characterized), only one copy
is likely to be corrected — exactly D-331's diagnosis ("a repair that restates
manufactures a stale claim... because the restatement is true only at the
revision it is written and nothing re-reads it").

**Severity:** MINOR. Both copies currently agree and are historically pinned to
one already-closed transition (u-rev 3 → 4), so the practical drift risk is
lower than the class D-331 was written against (a claim about another
document's *live, still-changing* state). It does not affect the soundness of
the CARVE-EXCEPTION substitution itself.

**Fix scope:** one sentence. Either drop line 118's restatement and point at the
head paragraph ("see the count above"), or drop the head paragraph's number and
let the REVIEW STATUS table carry it (the table is where D-331's "STATUS AND
SUMMARY MATTER CARRIES POINTERS ONLY" carve-out most naturally lives). Confined
to two sentences; no design content changes.

---

## Finding G2 (MINOR) — an unmarked, unreconciled numeric claim ("168 030 comparisons") sits beside a thrice-repeated, MEASURED-tagged claim of a different number ("343 344") for what reads as the same identity check

**Claim, U2 lines 828–832 (U2-Z, WP-1.6 handoff section, unchanged by u-rev 4):**
"the identity that makes that possible (`blocking_covers == Impossible ⟺
unblockable_double_threat`) is verified over **168 030 comparisons** with the
`Impossible` branch reached at every budget."

**Contradicting/inconsistent evidence, reproducer:**
```
$ grep -n "145 158\|343 344\|168 030" docs/experiments/U2_node_protocol.md
285:`covers` predicate — and MEASURED over **145 158** playout positions and **343 344**
348-349: (455 177/455 201, a different figure, unrelated to this pair)
606:| M5-E ... MEASURED equivalence over 145 158 positions and 343 344 comparisons, 0 disagreements. ...
618-619 (verbatim, marked quote): "...MEASURED to agree on 145 158 playout positions and on 343 344 side-and-budget comparisons with zero disagreements..."
832:  168 030 comparisons with the `Impossible` branch reached at every budget.
```
§5.2 (the load-bearing derivation, the natural HOME of this fact), the §5.6
M5-E matrix row, and the §5.6 verbatim "strongest surviving attack" quote all
independently state **343 344** comparisons over **145 158** positions for the
*same* identity (`blocking_covers(us,b) == Impossible ⟺
unblockable_double_threat(them,b)`), each tagged **MEASURED**. The U2-Z handoff
bullet states **168 030** comparisons for what its own prose describes as "the
identity that makes that possible" — the same identity, described with the
same predicate. It carries no MEASURED/ESTIMATED tag, and gives no derivation
relating 168 030 to 343 344 (it is not an obvious subset arithmetic: neither
`343344 − 168030 = 175314` nor any simple per-budget division of 343 344
by 2 or 3 lands on 168 030 exactly). I traced this line back through every
revision:
```
$ git diff 38f21b9 7dfd047 -- docs/experiments/U2_node_protocol.md | grep -n "168 030"
(no output — the line is byte-identical at u-rev 1 and u-rev 4)
$ git show 6feb40a:docs/experiments/wp15b_design.md | grep -n "168 030"
1960:  168 030 comparisons with the `Impossible` branch reached at every budget.
```
The figure predates the carve entirely — it was already in the superseded
`wp15b_design.md` at `6feb40a` (revision 7) and has been carried unchanged
through the carve and all three repairs. None of the five original
REVIEW-designs, two DECISION-RED-TEAMs, or three post-carve confirmation-pass
reviews caught it.

**Why it breaks:** the project's own convention (visible at every other
quantitative claim in this document, and CLAUDE.md's numeric-marking rule)
is that a measured count gets a **MEASURED** tag; this one does not, and it
disagrees with an established, triply-repeated, explicitly MEASURED figure for
what reads as the same fact with no stated reconciliation. I cannot rule out
that 168 030 is a genuinely different, narrower statistic (e.g. only the
subset of the 343 344 comparisons where the `Impossible` branch specifically
fired, across all three budgets, matching the "reached at every budget"
qualifier and echoing U2-T's `the_two_predicates_agree_everywhere` test
description) rather than a plain transcription error — I could not settle this
either way since IMPL has not started and no code exists yet to re-run either
count. Either way (error, or an unexplained distinct statistic), the text as
it stands does not let a reader tell which, and that ambiguity is itself the
defect this project's MEASURED/ESTIMATED discipline exists to prevent.

**Severity:** MINOR, not MAJOR/BLOCKING — it predates u-rev 4 by every
revision on record, is untouched by this repair's diff, and does not bear on
whether the F5/F6 CARVE-EXCEPTION substitution under review here is sound. I
flag it because named check 1's brief calls for a full-document sweep for
exactly this shape of claim, not because it is chargeable against this u-rev's
repair.

**Fix scope:** outside this repair's diff. Either mark 168 030 MEASURED and
state what subset it counts (distinguishing it from 343 344), or replace it
with a pointer to §5.2's figure if it is the same fact restated with a stale
number. Not a fix this round owes; recorded for a future repair.

---

## Verified with no finding

- **F5 answered on the merits (named checks 1, 2).** `git diff d85b049 7dfd047 --
  docs/experiments/U2_node_protocol.md` shows exactly five site-level changes
  beyond head apparatus: §2.2 (B5 marker added), §5.3 ×2 (F1 and F2 markers gain
  the `CARVE-EXCEPTION` token), §5.4 (R5's marker — the genuinely NEW one,
  closing F5), and U2-M/§12 (the D-310 stage-Q seam marker added). I swept the
  full diff hunk-by-hunk (`git diff d85b049 7dfd049 -- ...` above, plus the
  earlier `38f21b9→56b0bec` and `56b0bec→d85b049` diffs for context) and found
  no change to CARVED CONTENT (§2/§3/§5/§14 body prose) that is neither a
  `§n`-retarget nor carries one of the five markers. Every marker names both a
  u-rev and a cause (B5, F1, F2, R5/D-311, D-310), satisfying the stated rule.
  The substitution genuinely closes F5's defect class rather than adding a
  fifth list item that would itself go stale on the next repair — deleting the
  hand-maintained set and replacing it with a rule + derived enumeration is a
  real structural fix, not a rename of the same failure mode. This is the
  strongest part of the repair and I could not falsify it.
- **The self-matching grep (named check 3).** `grep -n "(CARVE-EXCEPTION"
  docs/experiments/U2_node_protocol.md` (the document's own registered command,
  exact string) returns **6** matches: the 5 real markers (lines 191, 430, 453,
  543, 710) plus **exactly 1** self-match — the command's own quoted invocation
  at line 62, because the literal string `"(CARVE-EXCEPTION` inside the code
  fence contains the same substring. The head's characterisation ("One line of
  this head matches the pattern too... the command above matches itself") is
  exact — singular, and correct. Cross-check: `grep -c "CARVE-EXCEPTION"`
  (no leading paren) returns **9** — the 6 above plus 3 genuinely bare mentions
  (lines 35, 55, 118, all describing the token/rule in prose, none of them a
  marker) — confirming the head's claim that "the parenthesis is what
  separates a marker from a bare mention of it" holds exactly: every bare
  mention lacks the leading `(`, every marker has it.
- **F6 (named check 4).** `grep -n "new lines\|line count\|lines of\|roughly
  [a-z]* lines\|~[0-9]* lines" docs/experiments/U2_node_protocol.md` returns
  only line 119, F6's own status-row description ("No line count of the diff
  is stated anywhere in this unit") — a true statement about itself. No other
  line-count or diff-count claim, marked or unmarked, exists anywhere in the
  unit. F6 is cleanly discharged by deletion, as claimed.
- **The u-rev table (named check 5).** All four SHAs resolve
  (`git cat-file -t` on `38f21b9`, `56b0bec`, `d85b049` — all `commit`); all
  three report paths exist on disk; each report's own `Pinned revision:` line
  matches the table's SHA exactly, and each report's own verdict line
  (`## VERDICT: FAIL` in all three) matches the table's "FAIL" column. Row 4
  ("this text | NOT YET REVIEWED | —") is accurate — this review is that row's
  first entry.
- **Cross-unit citations (named check 6).** U2 cites other units by
  section only (`**U1** §4`, `**U4** §9`, `**U3** §6`, `**U3** §12 item 4`,
  `**U4** §8.2`, `**U3** §7`, `**U3** §10`, `**U3** §6.2`, `**U4** §8.6`) —
  never by another unit's u-rev number. Verified live: U1 is at u-rev 2, U3 at
  u-rev 5, U4 at u-rev 7 at `7473a6f` (`grep -n "^\*\*u-rev [0-9]\."` on each
  unit file), all matching what the dispatch prompt stated. Since U2 makes no
  cross-unit u-rev claim at all, D-311/D-332's stale-citation defect class
  (which sank U3's own u-rev 4) cannot occur here — there is nothing to go
  stale.
- **B5/F1/F2/D-310-seam marker content (spot-check).** Each of the 4
  pre-existing markers' text matches, near-verbatim, the corresponding item
  that was in the now-deleted "Four exceptions" paragraph at u-rev 3 — I
  diffed the old paragraph's four clauses against the four new site markers
  and found no content drift introduced by the relocation, only the addition
  of the `CARVE-EXCEPTION` token and (for F1) a note about the now-removed
  ordinal self-reference.
- **WPQ_seed.md's sibling R5 paragraph** ("Its sibling paragraph in
  `WPQ_seed.md` carried its marker from the start," line 547–548) — confirmed:
  `docs/experiments/WPQ_seed.md:24` carries "(u-rev 2, one sentence, per
  D-311.)" attached to its own R5 paragraph, present since that seed's own
  u-rev 2. Not the literal `CARVE-EXCEPTION` token (WPQ_seed.md is a different
  unit, not bound by this unit's local convention), but the claim only says
  "carried its marker," which is true of WPQ_seed's own disclosure practice.

## Rejected, with the attempted reproducer

- **"The gate fails" as a finding against this document.** `cargo test -p
  pistol-solver --test wp15b_census` FAILS in the live tree (see GATE below),
  but the cause is `docs/experiments/matrix_U4R_REDTEAM.md`, an untracked file
  from unrelated concurrent work that happens to *quote* the census test's own
  `CARVE_MARKER` string verbatim inside a code block, tripping the test's naive
  substring-containment scan. I moved that one file aside
  (`mv docs/experiments/matrix_U4R_REDTEAM.md /tmp/.../holding/`), re-ran the
  gate (passed clean, 4/4), and moved the file back
  (`git status --porcelain` confirms the tree is exactly as found). U2's own
  content is read correctly by the pin in both runs
  (`the_census_pin_reads_every_carved_document_it_names ... ok` in both).
  Rejected as a finding against U2_node_protocol.md; recorded here because the
  gate's raw exit code is currently red and CLAUDE.md requires citing the
  gate's own log, not a wrapper's exit status.

---

## GATE

```
$ cargo test -p pistol-solver --test wp15b_census
running 5 tests
test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... FAILED
test wp15b_census_reproduces_the_registered_populations ... ok
test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
test the_census_pin_reads_every_carved_document_it_names ... ok

failures:
---- the_pins_document_list_is_the_set_of_carved_documents_on_disk stdout ----
thread 'the_pins_document_list_is_the_set_of_carved_documents_on_disk' panicked at
crates/pistol-solver/tests/wp15b_census.rs:878:5:
assertion `left == right` failed: this pin's CARVE_DOCS list and the carved
documents on disk disagree. Files carrying the marker but not read by the pin
are green-over-unread; files listed but not on disk are already a panic in
carve_documents().
  left: ["U1_gate_supersession.md", "U2_node_protocol.md", "U3_tier_t.md",
         "U4_soundness_instrument.md", "WPQ_seed.md", "section_owner_table.md"]
 right: ["U1_gate_supersession.md", "U2_node_protocol.md", "U3_tier_t.md",
         "U4_soundness_instrument.md", "WPQ_seed.md",
         "matrix_U4R_REDTEAM.md", "section_owner_table.md"]

test result: FAILED. 3 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
```

Isolated re-run with the untracked, unrelated file moved out of the tree
(restored immediately after; `git status --porcelain` clean before and after):
```
$ mv docs/experiments/matrix_U4R_REDTEAM.md /tmp/.../holding/
$ cargo test -p pistol-solver --test wp15b_census
running 5 tests
test wp15b_census ... ignored, a measurement, not a gate; run with --ignored --nocapture
test the_pins_document_list_is_the_set_of_carved_documents_on_disk ... ok
test wp15b_census_reproduces_the_registered_populations ... ok
test the_carved_design_units_carry_this_censuss_table_verbatim ... ok
test the_census_pin_reads_every_carved_document_it_names ... ok

test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
$ mv /tmp/.../holding/matrix_U4R_REDTEAM.md docs/experiments/
$ git status --porcelain
(clean — matches the state at review start)
```
The failure is real, live, and reproducible right now against HEAD — but it is
caused by an untracked file outside this unit's carve, containing a *quotation*
of the census test's marker string, not by anything in
`docs/experiments/U2_node_protocol.md`. This is worth flagging to the operator
independent of U2's own verdict: the census pin's substring-containment check
(`.contains(CARVE_MARKER)`) is not robust to a document that merely *quotes*
the marker while explaining the census mechanism (as `matrix_U4R_REDTEAM.md`
does, legitimately, in a red-team report about a different unit) — this is a
pre-existing brittleness in `wp15b_census.rs`, not introduced by this review.

---

## Summary

u-rev 4's substitution — deleting U2's hand-enumerated "exceptions" list and
replacing it with a stated rule (`CARVE-EXCEPTION` marker at point of
occurrence) plus a derived `grep` — is sound and does not lose what the list
gave. The list's only real function was enumeration for a reader; the rule
plus grep reproduces that (the command's own output IS the enumeration, and I
verified it produces the same 5-item set, self-match disclosed honestly) while
removing the specific failure mode (a second, hand-maintained copy of a count
that drifts on the next repair) that sank the list twice (F3, then F5). This
is a genuine structural fix, not a relabeling. F6 is cleanly discharged by
deletion. The u-rev table, SHAs, report paths, and cross-unit citations all
check out exactly.

Two MINOR findings remain, both narrow: G1, a small residual duplication
(a 5-item marker-provenance count asserted in two places) inside the very
paragraph that argues against hand-maintained counts; G2, a stale,
inconsistency-shaped, unmarked numeric claim inherited unchanged from the
pre-carve document (`6feb40a`) that eight prior review rounds (5
REVIEW-design + 2 DECISION-RED-TEAM + this document's own history) never
caught, and which is outside this repair's diff. Neither blocks landability on
the strength of F5/F6's repair; both are recorded per this project's zero-
outstanding-finding convention (every prior confirmation-pass round — u-rev 1,
2, 3 — was scored FAIL on 1 MAJOR + 1 MINOR each, never PASS-with-notes), so
the verdict here follows the same discipline.
