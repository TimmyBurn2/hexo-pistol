<!--
PROVENANCE. The body below is the ARCHITECT'S OWN TEXT, landed VERBATIM AND
UNEDITED so that the DECISION-RED-TEAM is dispatched against a retrievable
revision and so that the ADR line citing the attack cites a text that exists
(CLAUDE.md: an artefact that produces a registered judgement is named with its
revision).

The implementing session added this comment and nothing else. No sentence, cell,
number or clause of the matrix was written, corrected or reworded by it. Its
"Status: AUTHORED BY ARCHITECT, NOT SELECTED" line is the state at landing.

WHAT AN ATTACKER SHOULD KNOW BEFORE STARTING, stated here rather than inside the
body because it is not the architect's text:

- D-328/D-330 (ruling R11) splits author from measurer so that no author writes
  its own MEASURED cells. THIS MATRIX PRODUCES NO NEW MEASUREMENT. Its evidence
  base E1-E6 is entirely CITATION to artefacts already landed in this tree and
  already attacked: `docs/decisions.md` D-331/D-334/D-335, the four
  `wp15b_U*_REVIEW*.md` reports, `matrix_U4R_REDTEAM.md` and
  `matrix_U4R_REDTEAM_round2.md`. There is therefore no measurer to separate,
  and equally no measurement that has not already been through a fresh context.
  Whether every E-item's citation actually supports it is exactly what the
  attack is for.

- NO CELL IN THE OPTION TABLE OR THE EVIDENCE BASE CARRIES A **MEASURED** OR
  **ESTIMATED** MARK. CLAUDE.md's matrix clause requires every numeric claim in
  a matrix to carry one, and D-291 records that an estimate which could have
  been measured in seconds is itself a finding. The implementing session did not
  add the marks, because adding them would be editing the matrix. This is
  disclosed rather than repaired.

- THE IMPLEMENTING SESSION COULD NOT RECONCILE E6's SERIES against its own count
  of the landed reports, and states that here rather than changing the number.
  E6 reads "23 -> 20 -> 12 -> 15+ (U4 alone)". Taking each landed report's own
  verdict line for U4: `wp15b_design_rev7_REVIEW.md` 7B/7M/9m = 23 (that report
  covers the WHOLE pre-carve document, not U4 alone), `wp15b_U4_REVIEW.md`
  3B/3M/5m = 11, `wp15b_U4_REVIEW_urev6.md` 1B/2M/4m = 7,
  `wp15b_U4_REVIEW_urev7.md` 0B/4M/4m = 8, `wp15b_U4_REVIEW_urev8.md` 1B/7M/7m
  = 15. The direction E6 asserts at its tail — that the series turns back up and
  ends at 15 with zero PASS — reproduces. The middle terms do not, on that
  reading. A different denominator (all units per round rather than U4 per
  round) may reconcile them, and the attacker is the right party to settle it.

- D-336 (landed-evidence) and D-337 (frozen-matrix) land in the SAME COMMIT as
  this file, because the matrix's registered condition 2 binds them by name and
  a condition citing a line that does not exist is the defect this project has
  recorded more than once.
-->

# Matrix META-1: successor to R15, the meta-text decision
Status: AUTHORED BY ARCHITECT, NOT SELECTED. Awaits fresh-context
DECISION-RED-TEAM. Land at SHA before dispatch (D-310 prerequisite).

## Evidence base (all from landed session reports and ADRs, cited)
- E1. Three drift generators measured: restatement (D-331 reaches),
  self-completeness universals (no clause reaches; failed U3 four
  rounds, U4 twice), intra-round re-authoring (D-335). Source: closure
  session report, D-335.
- E2. Sole working detector across ~10 rounds: fresh-context reviewer
  hand-building a claim inventory (54 rows u-rev 7, 61 rows u-rev 8).
  Round-2 attacker stated this independently. No law, grep, structure,
  or matrix has found one instance.
- E3. Every recorded generator instance sits in meta-text: heads,
  change logs, lineage prose, status blocks (U4-Z, REVIEW STATUS),
  self-completeness claims. Normative content: node protocol clean
  five rounds; S-M and N-E selections ADR'd (D-323, D-329); gates
  defined post-B3.
- E4. U4 MEASURED 2105 lines, +219 in the round convened to shrink it,
  now larger than the 1975-line document the carve replaced.
- E5. External systems of record already exist for every meta-claim:
  git log (change history), docs/decisions.md (lineage, rulings),
  landed review reports (review status). None can drift; the
  in-document copies are the drift.
- E6. Convergence reversed: findings 23 -> 20 -> 12 -> 15+ (U4 alone),
  zero PASS at close.

## Options

| Option | What | Cost | Failure mode |
|---|---|---|---|
| A META-BAN | Strip ALL document-state claims from all four units: no heads beyond title+u-rev, no change logs, no lineage, no status blocks, no self-completeness universals (never assertable in-document; the reviewer inventory establishes completeness, the document never claims it). Homes: git, ADR log, review reports. Findings against deleted text die with the text. | One mechanical strip session + four re-reviews. | Some meta-sentence is load-bearing (a reviewer needs it to locate normative claims); mitigated by the owner table, which survives as the navigation artifact. |
| B INVENTORY | Keep documents as-is. Institutionalize E2: reviewer claim-inventory is the review deliverable, landed in-tree, diffed between rounds. Repair until inventories stabilize. | 2-4 more full rounds at current burn, U4 at 2105 lines each round. | Pays per-round for a surface A deletes for free; E6 says current trajectory does not converge; generators keep producing. |
| C RE-SCOPE | Documents as-is, but reviews re-scoped to NORMATIVE claims only; meta findings recorded, waived, never repaired (repair is the generator per E1). PASS = normative-clean. | Cheapest immediate path to IMPL. | Known-false meta-text ships in the design of record; a future reader cannot tell waived-false from true; the U1 lesson (falsehood carried five documents, nine rounds) argues this rots. |
| D = A + review contract | A's strip, PLUS: reviews target normative claims; reviewer inventory (normative claims only, now a small set) lands in-tree as the review deliverable per E2; a citation gate (tools/) mechanically checks every D-nnn reference in unit docs resolves to an existing key. | Strip session + citation gate (small, SHELL_CHECKLIST) + four re-reviews against much smaller, meta-free documents. | Strip session itself edits documents and could drift mid-strip; mitigated: deletion-only commits, no rewording, any sentence needing rewording is instead deleted and its claim re-homed by pointer; reviewer diffs deletions. |
| NULL | Keep D-331 as amended, keep repairing. | Per E6 and D-335: measured divergence. | Dead on the evidence; row present per law. |

## Recommendation
D. Grounds: E3 says the defect surface is meta-text, E5 says the
surface is redundant, E4 says it also answers the size question, E2
says the inventory is the instrument and D institutionalizes it at
minimum cost against a shrunken claim set. A alone leaves the review
contract vague; C alone ships falsehoods; B pays rent on a surface D
demolishes.

## Flip clauses
- If the red team shows >20 percent of unit text is meta-load-bearing
  (reviewers cannot navigate without it), flip to B: the meta-text is
  functional and the instrument, not the surface, is the fix.
- If a drift instance appears in NORMATIVE text after D executes, the
  meta-text diagnosis (E3) is falsified: STOP, architect, no successor
  law authored in-session.

## Registered conditions on D, if selected
1. Strip session: deletion-only. A sentence that cannot be deleted
   whole is deleted anyway and its normative content, if any, gets a
   pointer to its home. Zero rewording. One unit per commit.
2. Frozen-matrix rule and landed-evidence rule (this round's D-lines)
   bind.
3. Reviewer inventories land in-tree, reviewer-authored, fresh-built
   each round, never touched by an authoring session.
4. Citation gate before the re-reviews, so wrong pointers are caught
   mechanically and reviews spend zero rows on reference existence.
