<!--
PROVENANCE. The body below is the matrix VERBATIM AS ATTACKED — the text the
fresh-context DECISION-RED-TEAM was dispatched against, landed unedited so that
the ADR line citing the attack cites a retrievable text (CLAUDE.md: an artefact
that produces a registered judgement is named with its revision).

Its own "Status: AUTHORED, NOT SELECTED" line is the state AT ATTACK TIME and is
superseded by docs/experiments/restructure_selection_15b.md, which records the
attack's findings, the per-option survival verdict and the architect's selection
(option D). Nothing in the body is corrected here: a matrix edited after its
attack is a matrix that was never attacked.
-->

# WP-1.5b design restructure: option matrix
Status: AUTHORED, NOT SELECTED. Awaits fresh-context DECISION-RED-TEAM.
Ground: rev 7 REVIEW-design FAIL (7B/7M/9m), report at scratchpad
wp15b_design_rev7_REVIEW.md, SHA 6feb40a. Standing rule: no revision 8.

## Facts the matrix stands on
- Clean on merits after 5 rounds: node protocol (§5, M5-E), solver query
  surface, calculus IDs, scope, hotspot registration, six destructure
  sites, code line citations.
- Broken: M2/M3/M4 matrices absent since rev 2 (B1), pin mechanization
  scans 4-decimal fields only (B7), gate (b) undefined (B3), §8.3(a)
  carries rev-6 defect verbatim (B4), config count inconsistent (B5),
  pwd -P claimed-fixed-not-fixed (B6), M4 no ADR line (B2).
- Recurring defect class: repair in one section, dependent claim in
  another not re-read. Correlates with document size, not content.

## Travelling items (bind to every option, cost is common)
T1. Restore M2/M3/M4 matrices. Recover from ec8f7fb where present,
    reauthor where absent. RULE: a restored matrix is compared against
    the text its decision red team attacked; identical = attack stands,
    differs = fresh DECISION-RED-TEAM before the selection is cited.
T2. M4 ADR line (B2).
T3. Define gate (b) or rewrite "(a)-(d) plus S-E" wiring; kill the S-E
    double-listing (B3).
T4. Fix or demote the pin: scan all restated numerics, or weaken §6.2
    to enumerate what the pin covers (B7). §17 stopping decision
    re-grounded on the honest version.
T5. Revision label bumps on ANY append (6feb40a appended §18 unbumped,
    caused pin ambiguity this round).
T6. B4, B5, B6 land in whichever unit owns their text.

## Options

| Option | Cut | Cost | Failure mode |
|---|---|---|---|
| A | Reviewer's four-way: (1) gate supersession §4, (2) node protocol §5, (3) Tier-T + widening §6-7, (4) soundness instrument §8-9. Integration claims stated per unit, no separate integration doc. | 4 reviews, one sitting each. §5 unit is a fast confirmation pass. | Cross-unit wiring (gate refs span units 1 and 4) drifts again; mitigated by each unit stating its imports/exports as named claims the other unit's review checks. |
| B | Per-matrix: M0..M5 each own doc + integration doc. | 7 reviews. M0, M5 already clean = 2 wasted sittings. | Highest round count, integration doc becomes the new big-doc failure mode. |
| C | Two-way: clean core (§5 + everything the report cleared) fast-passed as unit 1; all broken text as unit 2. | 2 reviews. | Unit 2 is most of the document = the exact size that failed 5 rounds. Does not restructure, relabels. |
| D | Scope narrow: ship stages F + T only, quiet stage + widening deferred to follow-up WP with own design. Restructure remainder per A minus unit 3. | 3 reviews now, one WP later. Faster to IMPL. | SPRT prereg registers a different engine, prereg amends, review reopens. Strength delta smaller; depth_at_500ms movement may not clear noise, closure criterion at risk. Widening matrices (T1) deferred, not escaped. |

## Recommendation
A. Grounds: cut follows section boundaries the review already treats as
coherent units; findings map one-to-one onto units; clean §5 costs one
short sitting and pins the part five rounds could not break; avoids
D's prereg reopen and closure-criterion risk. The recurring defect
class is size-driven and A is the smallest cut that isolates every
blocker in exactly one unit.

## What flips it
- Red team shows the §4/§8-9 gate wiring cannot be stated as checkable
  per-unit import/export claims: flip to D (defer the coupled half).
- Red team shows unit 3 (§6-7) still exceeds one sitting after T1
  restoration: split unit 3 only, A stays otherwise.
