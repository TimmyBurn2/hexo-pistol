<!--
PROVENANCE — this file is a LANDED DISPATCH.

- Document:          Dispatch for the fresh-context REVIEW-design of the WP-1.5b
                     SPRT pre-registration (`docs/experiments/wp15b_sprt_prereg.md`).
- Authored by:        The architect, in-conversation, supplied verbatim to this
                     session for landing.
- Supersedes:        `dispatch_prereg_rev4_review.md` — cited at
                     `restructure_selection_15b.md:87` as the held dispatch
                     governing a first review, and recorded by `docs/decisions.md`
                     D-339 as never added at any revision and unrecoverable. That
                     ruling stands; this file does not reconstruct it. It is a
                     freshly authored dispatch, content updated for WP-1.5b's
                     shipped D scope (F+T engine, stage Q deferred), that resolves
                     the citation going forward.
- Launch condition:  met at `cd70944` (`wp15b_sprt_prereg.md` revision 5) — the
                     prereg's two launch-condition defects (the dangling
                     `wp15b_design.md` citation, the false "MATRIX M4 ADOPTS
                     --config" sentence) are fixed at that commit.
- Landed at:          2026-08-22, tree at `cd70944` (the prereg-edit commit this
                     dispatch's launch condition names; this file's own landing
                     commit is necessarily one commit later and is what D-339's
                     appended line cites).
- Cited by:          D-339 (appended line), and this dispatch's own resulting
                     review report.

THE BODY BELOW IS VERBATIM AND UNEDITED. Nothing in it is corrected, re-scoped or
annotated here: a dispatch edited after the fact is a dispatch that was never
issued.
-->

# [GROUNDWORK] REVIEW: SPRT prereg — HELD, launch condition inside

LAUNCH CONDITION: after the prereg draft is edited to WP-1.5b's
shipped D scope (F+T engine, stage Q deferred), its known defects
fixed in the same edit: the "MATRIX M4 ADOPTS adding --config"
sentence (no matrix supports it; N-E was selected by D-329's ladder),
the dangling reference at prereg:50 to the deleted pre-carve design
file, and the two items formerly labeled R10 (identical to the two
fixes above). The edit is a draft edit, not an amendment reopen: the
document has never been reviewed and governs nothing yet. This review
targets the edited revision. CONDITION MET at cd70944, revision 5.

Fresh context. You are not the session that authored the prereg, not
any session that edited it, not any session that touched its
instrument chain. This document has NEVER passed review at any
revision; you are its first reviewer. It governs an operator-hardware
strength run; review at the revision that governs, per CLAUDE.md.

## Pinned target
- Prereg at revision 5, repo SHA cd70944.
- Header: SHA, HEAD match, prereg revision label.

## Read
- The prereg, all sections. Its own amendment/reopen terms.
- CLAUDE.md: dry-run rule, proportionality rule, hard rule 6.
- tools/SHELL_CHECKLIST.md: the instrument chain is tools/ scripts;
  answer checklist items BY NAME for every script the prereg's
  verdict reads through.
- The ADR lines recording instrument findings and residuals
  (per-game inversion guard, exit-code taxonomy, budget reading,
  tmpfs field-collapse residual, one-base path rule) and the
  accepted-residue list (Track C F1-F3 plus later MINORs). Verify
  the prereg text claims nothing stronger than these lines concede.

## Verify, minimum
1. Dry-run record: input same-kind, not the registered workload;
   input and output recorded; dry run does not consume the first
   governed run.
2. Inversion guard: per-game attribution. The confined-inversion
   residual stated honestly; run the honest-twin test yourself.
3. Exit-code taxonomy: precondition failures cannot read as
   attribution defects; a skipped build is loud.
4. Budget line: replay reads the report's budget, refuses any kind
   but nodes, loudly.
5. Registered engine scope matches shipped D scope: F+T, stage Q
   absent, N-E's config selection consistent with D-329.
6. Cost statement on the document's face (wall time, operator
   attention, machine hours).
7. Verdict criteria pre-registered, GSPRT bounds and n-accounting
   per hard rule 6, distinct-n dedupe stated, no post-hoc freedom.
8. Engine seats digest-bound at the revision the run will use.
9. N-E implementation-debt visibility: the debt recorded at
   selection is either discharged by the IMPL revision the run binds
   or named in the prereg as a known cost.

## Verdict
PASS or FAIL, numbered findings, reproducer or text cite each.
PASS = operator confirms slots and schedules the run; any later
touch of the instrument or the prereg reopens this review.

## DONE
Report landed in-tree and committed, with header, verdict, findings,
checklist items answered by name.
