<!--
PROVENANCE.

- Matrix attacked: docs/experiments/restructure_matrix_15b.md, landed verbatim
  as attacked in this same commit.
- DECISION-RED-TEAM report: `wp15b_restructure_REDTEAM.md`, written to the
  dispatching session's scratchpad
  (`/tmp/claude-1000/-home-tom-Projects-HeXO-AlphaBeta/28856979-30d2-48d6-b1c8-5dc584ab6c03/scratchpad/`).
  THAT PATH IS SESSION-SCOPED AND WILL NOT SURVIVE. Per this project's practice
  a review is cited by the revision it attacked and its substance is carried in
  the ADR line, not by file path (D-308 cites `5ed1e84`/`e9a5145`, not a report
  file) — the report's eleven findings are summarised in the record below and
  the load-bearing measurement is reproduced in the ADR line that cites it.
- Subject revisions: design `6feb40a`, review report `wp15b_design_rev7_REVIEW.md`
  against that SHA, SPRT pre-registration `ca0d331` (revision 4).
- The body below is the architect's selection record verbatim.
-->

# WP-1.5b restructure: SELECTION RECORD — option D
Selected by architect after DECISION-RED-TEAM
(wp15b_restructure_REDTEAM.md, design 6feb40a, prereg ca0d331).
Survival: A wounded, B falls, C falls, D survives.

## PREREQUISITE, blocks the ADR
The matrix and this record land at a SHA before any ADR line cites the
attack. Operator: commit restructure_matrix_15b.md + this file under
docs/, note the red-team report location. ADR cites that SHA.

## The selection, corrected for F5
D = WP-1.5b ships stages F + T only. Quiet stage + widening (stage Q)
deferred to follow-up WP with its own design, own matrix (M2 W-E,
fresh, never authored), own SPRT.

Unit cut under D, F5 incoherence fixed:
- U1 gate supersession (§4 lineage)
- U2 node protocol (§5, M5-E) — fast confirmation pass, clean 5 rounds
- U3 Tier-T ONLY (§6-7 minus widening text; widening text moves to the
  deferred WP's design seed, not deleted, not reviewed now)
- U4 soundness instrument (§8-9)
Every one of the 1975 lines gets exactly one owner (F3): the carve
produces a section-owner table covering §1-§18 including §2.2, §10,
§18.3 (B5's four sites each named to an owner). Unowned line = carve
FAIL.

## Travelling items, corrected per red team
T1'. M4: RECOVERY from ec8f7fb, then diff vs red-teamed text; identical
     = attack stands, differs = fresh round. M3 S-E: FRESH matrix +
     fresh DECISION-RED-TEAM, never existed. M2 W-E: deferred with
     stage Q, debt recorded in the follow-up WP stub, not escaped.
T2. M4 ADR line (B2). T3. Gate (b) defined or wiring rewritten, S-E
     double-list killed (B3). T5. Label bump on any append.
T4'. The pin (F8, B7): hard-coded path at wp15b_census.rs:638 will stay
     green over files it no longer reads. Pin either re-targeted to the
     carved files with per-file paths, or RETIRED with an honest note.
     A green pin over unread files is banned.
T6'. B4 -> U4 (§8.3(a) sentence). B5 -> per the owner table. B6
     (pwd -P, solver_edge_check.sh:103) -> U1, it gates M0's adopted
     option.
NEW T7 (F4): build-order edge. pistol-solver absent from
     [workspace.dependencies]; U2's IMPL creates the edge U1's gates
     fire on. Review order free, IMPL order binding: U2 lands before
     U1's gates are armed. Recorded in the carve and in U1's design.

## ADR draft lines (operator appends, SHAs filled)
D-3xx: WP-1.5b design rev 7 REVIEW-design FAIL at 6feb40a, 7B/7M/9m —
  standing no-rev-8 rule fires, restructure per matrix at <SHA> — flips
  on nothing, rev 7 is closed.
D-3xx: Restructure option D selected (F+T ship, stage Q + widening
  deferred to follow-up WP) after DECISION-RED-TEAM at <SHA> — strongest
  attack surviving against the field: A's size-driven premise measured
  false, 0.86 vs 7.35 defects per 100 added lines across rev 1-6 vs
  rev 7, the moved variable is repairs not lines, and A multiplies the
  boundaries each repair crosses while its one mechanized check resolves
  a hard-coded path that stays green over files it no longer reads —
  strongest attack surviving against D itself: the M2 matrix round and
  stage-Q strength are deferred debt, not escaped, and 1.5b's SPRT
  delta shrinks accordingly — flips if the follow-up WP is never
  scheduled or the F+T SPRT delta fails to clear its registered bound.
D-3xx: Rev label bumps on any append to a reviewed document (6feb40a
  appended §18 unbumped, caused pin ambiguity) — flips on nothing.

## Prereg consequence (F9, simplifies my earlier hold)
Prereg rev 4 is a draft, governs nothing, never reviewed. Editing it to
D scope is a draft edit, not an amendment reopen. Order: carve fixes
D's exact shipped surface -> prereg draft edited to register F+T engine
and drop the removed confounded axis -> FIRST review per the held
dispatch (dispatch_prereg_rev4_review.md), target updated to the
D-scope revision. Held dispatch otherwise unchanged.

---

## The attack this record rests on — findings, for the reader who cannot fetch the report

Eleven findings; four kills. Reproduced here because the report file is
session-scoped (see PROVENANCE).

- **F0** the matrix had no revision and no path — discharged by this commit.
- **F1** no numeric claim in the matrix was marked MEASURED or ESTIMATED
  (CLAUDE.md's matrix clause; D-291's precedent, third occurrence).
- **F2 (KILL)** the founding premise "correlates with document size, not content"
  is false against the document's own history. MEASURED, `git show <rev>:… | wc -l`:
  revisions 1–6 (`ec8f7fb`→`2d07ff6`) added 1044 lines for 9 recorded instances
  (D-305) = **0.86 per 100 lines**; revision 7 (`2d07ff6`→`d94dc0a`) added 136 for
  10 = **7.35 per 100 lines**. 8.5× the rate at one-eighth the growth. The moved
  variable is repairs, not lines.
- **F3** option A's four units left 923 of 1975 lines (46.7 %) unowned, and T6 had
  no referent for B5, whose four sites are §2.2 (237), §10 (1349), §10 (1358),
  §18.3 (1941) — none in any unit. Answered by this record's owner-table requirement.
- **F4** the named suspect was the wrong coupling: `pistol-solver` is absent from
  `[workspace.dependencies]` and from `crates/pistol-search/Cargo.toml`, so U2 is the
  commit that creates the edge U1's gates fire on (§4.1 measures both at exit 1).
  Answered by T7.
- **F5 (KILL)** flip clause 1 was incoherent — trigger about units 1 and 4, remedy
  (D) defers unit 3 and retains both. Answered by this record's corrected cut.
- **F6 (KILL)** flip clause 2 had already fired: unit 3 measures ~325 lines after T1
  restoration and ~425 once §10's config shape is owned, against unit 2's 355.
- **F7** T1's "cost is common" was false and false toward the recommendation.
  MEASURED: `W-E` and `S-E` occur **zero** times at `ec8f7fb`. M4 is a mechanical
  recovery; M2 and M3 are fresh matrices needing fresh rounds. Answered by T1'.
- **F8** the pin (`crates/pistol-solver/tests/wp15b_census.rs:638`) resolves one
  hard-coded path and stays green over files it no longer reads. Answered by T4'.
- **F9 (KILL)** both grounds cited against D were void: the prereg is a draft that
  governs nothing and owes a fresh-context review under every option, and
  `depth_at_500ms` is "demoted to below-marker CONTEXT" (design:1313), ADVISORY
  (design:1500), and absent from ROADMAP's exit criterion (`docs/ROADMAP.md:140`).
  Answered by the Prereg consequence section above.
- **F10** option B's cost cell called M0 clean against the matrix's own Facts block;
  B6 is M0's and is live.
- **F11** two rows missing — cut-by-artefact, and the null row. Not selected; recorded
  so a later round does not re-discover them. Every splitting option also owes a
  ROADMAP-by-ADR line, §15 items 9 and 10 being unlanded already.

**Strongest attack surviving against D**, recorded for the ADR line: the M2 matrix
round and stage-Q strength are deferred debt, not escaped; WP-1.5b's own SPRT delta
shrinks by exactly the axis D removes, and the follow-up WP is the only place that
debt can be paid.
