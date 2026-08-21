<!--
PROVENANCE — this file is a LANDED EPHEMERAL REPORT.

- Report:            REVIEW-design of WP-1.5b design revision 7.
- Dispatching session id: 11e08e37-cb12-4903-9ee6-74ef61d5324f
- Original path:     /tmp/claude-1000/-home-tom-Projects-HeXO-AlphaBeta/
                     11e08e37-cb12-4903-9ee6-74ef61d5324f/scratchpad/
                     wp15b_design_rev7_REVIEW.md
                     (session-scoped tmpfs; does not survive the session — this
                     landing is the only retrievable copy)
- SHA the report examined: 6feb40af1f1c12c1977d7a2030509dd98cbdc8ac
                     (`docs/experiments/wp15b_design.md`, revision 7, 1975 lines;
                     that file is DELETED from the tree at cf74594 by the carve and
                     is retrievable at 6feb40a and nowhere else)
- Landed at:         2026-08-22, tree at cf74594.
- Cited by:          D-309 (closes revision 7 on this report), and the eleven-finding
                     summary in docs/experiments/restructure_selection_15b.md.

THE BODY BELOW IS VERBATIM AND UNEDITED. Nothing in it is corrected, re-scoped or
annotated here: a report edited after the fact is a report that was never written.
Where a finding it raises has since been repaired, the repair is recorded in the
owning unit document and in docs/decisions.md, never in this file.
-->

# REVIEW-design — WP-1.5b staged threat-first generation, revision 7

## Header

- **Revision reviewed:** `6feb40af1f1c12c1977d7a2030509dd98cbdc8ac`
- **Matches HEAD:** YES. Working tree clean at review time.
- **File:** `docs/experiments/wp15b_design.md`, 1975 lines, self-labelled "Revision 7".
- **Candidate confirmation:** `6feb40a` does carry revision 7. Note it is *not* the
  commit that produced revision 7: `d94dc0a` is ("WP-1.5b reaches revision 7"), and
  `6feb40a` appends 69 lines (§18, the OPERATOR-QUEUE) without changing the revision
  label. See finding 20.
- **Reviewer context:** fresh. Revisions 1–6 not seen; read only where a claim in
  revision 7 cites its own history, and then from `git show`.

## VERDICT: **FAIL**

7 BLOCKING, 7 MAJOR, 9 MINOR. No STOP; no matrix selection is reopened on its
merits. The failure is again TRANSMISSION — and, newly, STRUCTURE: three of the six
matrices this review was asked to check no longer contain a matrix.

The primary hunt landed. Revision 7's own repairs (§10's fourth config, §15's
appended items, §17/§18) each left a dependent claim un-re-read, and the
mechanization the revision rests on does not cover the sites it claims to cover —
**verified by running it: the pin is GREEN at this SHA while six census figures are
restated outside the pinned block.** Instances thirteen through nineteen.

---

## BLOCKING

### B1. Matrices M2, M3 and M4 contain no option matrix

- **Claim:** §7 "MATRIX M2 — the widening schedule", §8 "MATRIX M3 — the soundness
  instrument", §9 "MATRIX M4 — the snapshot's config seam".
- **Contradicting text:** the document contains exactly three tables headed
  `| Option |` — §4.2 (M0), §5.6 (M5), §6.3 (M1). §7 has none; §8 has none; §9 has
  none. Option labels `W-B` and `W-C` occur **zero** times in the whole document;
  `S-A`, `S-B`, `S-D` occur once each, in prose; `N-B`/`N-C`/`N-D` occur only inside
  §9's single opening sentence, with no cost and no failure-mode cell.
- **Reproducer:** `grep -c "^| Option |"` → 3. Across revisions:
  `ec8f7fb` 5 tables → `182f389` 2 → `7ad466b`..HEAD 3. The M2/M3/M4 matrices were
  deleted at revision 2 and never restored.
- **Why it breaks:** CLAUDE.md — "A named design decision with more than one viable
  option is settled by an OPTION MATRIX — options, costs, failure modes,
  recommendation … An option adopted without a matrix, or a matrix never attacked,
  is the same breach as silent architecture drift." The record of the attack (§16)
  is not the matrix. This review was instructed to verify that M2 W-E, M3 S-E and
  M4 N-A are "consistent with its own matrix text"; that check is **not performable
  against revision 7**, because the matrix text is not in the artefact under review.
  A reader must fetch `ec8f7fb` to see what W-E was chosen over.

### B2. M4 has no ADR line at all

- **Claim:** §15 preamble — "Registered by name so none is silent."
- **Contradicting text:** §15 contains zero occurrences of `N-A`,
  `baseline_snapshot`, `--config` or `snapshot`. Items 1, 2, 3, 4, 5 cover M0, M1,
  M2, M3, M5. M4 is absent.
- **Why it breaks:** §9 adopts N-A, which changes a shipped `tools/` instrument
  (`tools/baseline_snapshot.sh` gains `--config` with four named guards and eight
  SHELL_CHECKLIST items) and changes the registered quantity of the snapshot. Rule
  10 requires one ADR line per non-obvious choice; the Process section requires the
  surviving option's ADR line to record the strongest surviving attack. Neither
  exists for M4. §9's five amendments are therefore unlanded design with no ADR
  home.

### B3. Soundness gate (b) is never defined, and §8.7 double-counts it

- **Claim:** §8.7 — "(a)–(d) plus S-E become one script,
  `tools/staged_soundness_check.sh`".
- **Contradicting text:** `grep "(b)"` over the whole document returns two hits —
  §4.2's matrix row `(b) INVERT both as declared lists` and §5.2's citation
  `D-257 (a)/(b)`. Neither is a soundness gate. §8.3 is titled "The other three
  parts, re-scoped" and defines **(a)**, **(c)**, **(d)** plus one unlabelled bullet
  ("The five gate_v0 cases need a staged config", which is a config statement, not a
  gate).
- **Reproducer:** revision 1 defined it — `git show ec8f7fb:… | grep "(b)"` line 502:
  "(a) tactical suite at pre-registered thresholds under Staged; **(b) a differential
  gate** against…", and line 507's matrix is headed `| Option | (b)'s instrument |`.
  So **S-E *is* gate (b)'s instrument**. Revision 2 deleted the enumeration and the
  matrix together.
- **Why it breaks:** the gate wired into CI is specified as "(a)–(d) plus S-E", which
  at revision 7 (i) names a component with no definition anywhere in the document and
  (ii) lists S-E twice — once as (b), once as itself. The prompt asks whether gates
  a–d are "defined, checkable"; (b) is neither.

### B4. §8.3(a) still carries verbatim the sentence §10 identifies as revision 6's defect

- **Claim (line 1164):** "**All three staged tactical configs disable the quiet cut**
  (`quiet_top_k` above the whole pool), **not just the two gate ones**."
- **Contradicting text (line 1359, §10):** "Revision 6's §8.3(a) said *'all three
  staged tactical configs disable the quiet cut'* while §10 committed
  `quiet_top_k = 16` for two of them and §15 said 'the two gate configs' — three
  statements of one rule, none agreeing." And §15 item 15: "**The two TACTICAL
  staged configs disable the quiet cut**."
- **Why it breaks:** revision 7 repaired §10 (by adding a fourth config) and §15
  item 15, and left the offending sentence in §8.3(a) untouched. At revision 7 there
  are **four** staged configs of which **two** are tactical, and **one** is a gate
  config — so "all three … not just the two gate ones" is false in both halves and
  has no referent for "the two gate ones". This is the recurring defect reproduced
  inside the very repair that names it, in the section (§8.3(a)) the derivation for
  `require 20` rests on.
- The *substance* of §8.3(a)'s derivation survives — the 20 cases run at
  `tactical_staged_v0.toml` (15) and `gate_staged_v0.toml` (5), both with the cut
  disabled — but the sentence a reader would implement from is wrong.

### B5. The document states the config count three different ways

- §2.2 (line 237): "Staged ships as **three** config-selectable documents (§10)."
- §10 lead-in (line 1349): "**Three complete**, `deny_unknown_fields`, no code-side
  default for any value:" — immediately above a **four-row** table.
- §10 body (line 1358): "**FOUR documents**, because three could not carry the
  requirement."
- §18.3 (line 1941): "Staged would ship as **four** selectable documents."
- **Why it breaks:** rule 1 makes the config set a normative deliverable. Revision 7
  added `tactical_staged_v0.toml` and repaired §10's body and §18.3 while leaving
  §10's own lead-in and §2.2 at three. §2.2 is the section a reader consults for
  scope.

### B6. Option (f)'s precondition: "Fixed here" vs "recorded, not fixed" vs the shipped script

- **Claim (§15 item 14):** "`tools/solver_edge_check.sh:103` uses bash's logical
  `pwd` … **Fixed here** because (f) depends on it."
- **Contradicting text (§18.3):** "**The `tools/` defects this round found were
  recorded, not fixed** (D-302, D-303)."
- **Contradicting text (§15 preamble):** the landed-line corrections "are recorded
  rather than fixed, because they are outside this WP's mandate."
- **Reproducer, decisive:** at this SHA,
  `tools/solver_edge_check.sh:103` is `ROOT_ABS="$(cd "$ROOT" && pwd)"` — bare `pwd`;
  `grep -n "color" tools/solver_edge_check.sh` finds no `--color never` on any of the
  three `cargo tree` invocations (lines 75, 81, 86). D-303's own flip clause —
  "Flips when the gate takes `pwd -P` and `--color never`" — has not fired.
- **Why it breaks:** this is not bookkeeping. §4.2's (f) cost cell and §4.4 both make
  the two `tools/` changes a *condition* of the adopted option: the edge half is a
  byte-compared golden transcript, and §4.4 states it "is not machine-invariant as
  shipped". M0's ADOPTED option therefore rests on a precondition that one section
  asserts is met, two sections say is not, and the tree confirms is not.

### B7. §6.2's no-restatement claim is false, and the pin cannot see it

- **Claim (§6.2, line 826):** "**No other section of this document restates a number
  from it**; they cite it."
- **Contradicting text — six sites outside the pinned block (lines 797–824):**
  - `70.8 %` (census `BATCHED nodes`, corpus roots) at lines **139**, **584**, **1260**.
  - `6.83` (census `option C — Tier T outside the r2 ball` = `6.8333`) at lines
    **853**, **1442**.
  - `23.2` (census `option C — Tier T (threshold, ADOPTED)` = `23.2917`) at line **853**.
- **Reproducer, and it is the load-bearing one:**
  `cargo test -p pistol-solver --release --test wp15b_census` →
  `the_design_document_carries_this_censuss_table_verbatim ... ok` at this SHA. The
  pin is GREEN with all six restatements standing. Its own source says why —
  `wp15b_census.rs:685`: "Only the four-decimal renderings are unambiguous enough to
  grep for", and the scan requires `field.chars().all(|c| c.is_ascii_digit() || c == '.')`
  with exactly four decimals. `70.8 %` carries a space and a `%`; `6.83` and `23.2`
  are rounded, so `outside.contains("6.8333")` is false.
- **Why it breaks:** §17 states the limit honestly ("any four-decimal figure restated
  outside it"); §6.2 states it absolutely, and §6.2 is the section that licenses every
  other section to cite rather than restate. Revision 7's headline claim — the header's
  "All three now fail the build" — is true of the three corruptions the author tried
  and false of the class the document actually contains. The mechanization does not
  bind at four of the sites §6.2 says it covers, and the four-decimal restriction is
  a property the defect class **preserves**, which is the vacuity CLAUDE.md forbids
  and which this document charges against S-C (§8.1) and against its own §12.5 (row 12).

---

## MAJOR

### 8. §8.4's M4 and M6 witnesses are not positions a legal game reaches

- **Claim (§8.4 opening):** "**Each mutation names the position it dies on, and where
  the corpus cannot produce one it is BUILT** (D-260's precedent and its remedy)."
  M4's row: "**VERIFIED on the shipped solver:** P1 at
  (1,0)(2,0)(3,0)(4,0)(0,1)(0,2)(0,3)(0,4) sealed by P2 at (-1,0)(6,0)(0,-1)(0,6)".
  M6's row: "**VERIFIED**: … P2 with three disjoint sealed five-runs at rows 8/16/24".
- **Contradicting text (§17):** "M4's and M6's witnesses must become positions a legal
  game reaches (both are currently `ThreatState`-level constructions with impossible
  stone counts)."
- **Why it breaks, from rule 3 of CLAUDE.md's pinned rules:** turn 1 is ONE stone and
  every later turn is TWO by the mover, so at a turn boundary P1's stone count is odd
  and P2's is even. M4's witness is P1 = 8 (even) — unreachable. M6's is P2 = 15 plus
  seals (odd) — unreachable. §8.4's "VERIFIED on the shipped solver" is true only of
  `ThreatState::apply` driven directly; it is not a verification of the mutation dying
  in the search, which is what the ledger claims. Two of eight mutations are
  undischarged, and §8.4's own opening paragraph quotes D-295's finding that asserting
  an instrument's strength rather than measuring it *is* the defect. §17 records this
  as owed and §8.4 was not annotated.

### 9. Rule 5 is unsatisfied for the change D-263 was written about

- **Claim (§12 item 4):** the rule-5 registration is given for **Tier-T cell
  extraction** — hotspot, baseline, abort threshold 1.05×, IQR-gated bench, one
  change = one commit.
- **Contradicting text (§17):** "rule 5 owes a bracket, an abort threshold and an
  nps/time-to-depth bench for the **NODE PROTOCOL itself**, not only for the Tier-T
  accessor — D-263 registered that hotspot in advance precisely so the first per-node
  caller would not discover it."
- **Why it breaks:** D-263's text is explicit that "a rule-5 verdict needs a
  pre-registered hotspot, an expected gain bracket, an abort threshold and one
  IQR-gated bench reporting nps AND time-to-depth, and NONE of those is in this line",
  and its flip clause is "Flips when WP-1.5b measures it". Putting `can_win_this_turn`
  + `blocking_covers` on every node is the perf-sensitive change; §12 item 4 measures
  their cost (246/71/69 ns, ceiling 10.51 % → 7.45 %) but registers no bracket, no
  abort threshold and no bench for the protocol. The design does **not** smuggle a
  different hotspot — it declares the substitution loudly and takes ADR line 7 for it,
  which is legitimate under D-263's flip clause — but it leaves the original
  registration undischarged and says so.

### 10. §15's preamble is false on both of its clauses

- **Claim:** "Registered by name so none is silent. The first six are this WP's own;
  the **last three** are corrections to LANDED lines that this review round falsified,
  and they are **recorded rather than fixed**, because they are outside this WP's
  mandate."
- **Contradicting text:** the list holds **23** items. The last three in document
  order are 22 (a calculus amendment this WP owes), 23 (this WP's census disposition)
  and 19 (a rule-9 re-read at this WP's own landing commit) — none is a correction to
  a landed line. The corrections to landed lines are items **12** (D-255), **13**
  (D-299(2)), **14** (`solver_edge_check.sh`) and **20** (amends D-207) — four, not
  three, and not last. And item 14 says "**Fixed here**", directly against "recorded
  rather than fixed" (see B6).
- **Why it breaks:** items were appended at revisions 4–7 and the preamble that
  characterises them was never re-read. It is the same defect the list itself records
  at revision 4 ("the un-re-read claim, inside the ADR list itself" — item 4).

### 11. §16 names a revision-6 review SHA the header contradicts, and the two SHAs differ

- **Claim (§16):** "REVIEW-design | revision 6, `9c068a0` | **FAILS** — 4 BLOCKING…".
- **Contradicting text (header):** "revisions 1–6 were `ec8f7fb`, `182f389`,
  `7ad466b`, `f762c9a`, `64af80c`, **`2d07ff6`**."
- **Reproducer:** `git diff 2d07ff6 9c068a0 -- docs/experiments/wp15b_design.md` →
  4 insertions, 4 deletions, in §0 row 8 and §8.1 (the "> 100 s reads the table one
  column left" correction). `9c068a0`'s own subject is the SPRT pre-registration, not
  the design.
- **Why it breaks:** CLAUDE.md — "A review is dispatched against a NAMED REVISION …
  and every reviewer states that revision … Reviews of superseded revisions do not
  transfer — an amendment reopens the review, however small the diff." The document
  gives two different SHAs for the text revision 6's reviewer saw, and they are not
  the same text. Either §16 misnames the reviewed revision, or the header misnames
  revision 6 and the review record is of a text amended after the fact.

### 12. Unmarked numeric claim inside matrix M1, on the adopted option's own failure-mode cell

- **Claim (§6.3, line 853, option C, Failure modes):** "Residual: no cells blocking an
  opponent count-2 window; left to Tier Q's delta ranking, which is a set of **23.2
  cells/node** against a quiet allowance of 16."
- **Why it breaks:** CLAUDE.md — "EVERY NUMERIC CLAIM IN THE MATRIX IS MARKED
  **MEASURED** OR **ESTIMATED**". `23.2` carries neither mark. It is also not a census
  row: the pinned block renders no Tier-Q or quiet-pool quantity at all, and `23.2`
  coincides with `option C — Tier T (threshold, ADOPTED) | 23.2917`, which is Tier T,
  not Tier Q. The cell is the one that states the ADOPTED option's residual risk, and
  the number it states makes the residual pool look comparable to the 16-cell
  allowance; the census's own `radius-2 ball` row (77.9583) does not support that
  reading for Tier Q. Either the figure is mis-attributed or it is an unmarked
  estimate the document has no instrument for — and D-291's clause makes an estimate
  that could have been measured in seconds a finding in its own right, with the
  instrument already committed.

### 13. Three §15 items are listed as owed and have already landed

- **Claim:** §15 is titled "ADR lines this work package **owes**". Item 12 alone
  carries "**LANDED as D-301** at `68a28c8`".
- **Contradicting text:** `docs/decisions.md` carries **D-302** (item 13's D-299(2)
  finding), **D-303** (item 14's `pwd` finding) and **D-304** (item 23's census
  disposition). §18.3 cites "(D-302, D-303)" by number, so the session knew.
- **Why it breaks:** the commit that landed D-301..D-305 (`68a28c8`) updated exactly
  one of the five §15 items that point at it. The list a reader uses to know what is
  still outstanding overstates the debt by three and understates what is settled.

### 14. D-305's recorded count is superseded by this revision and no amendment is registered

- **Claim (§17):** "**twelve instances** of one defect have been found across five
  rounds"; header: "Instances ten, eleven and twelve."
- **Contradicting text:** `D-305` (landed at `68a28c8`, before revision 7) reads
  "Across six revisions … four fresh-context REVIEW-designs found **NINE** instances
  of one shape", and enumerates 3 + 1 + 1 + 4 = 9.
- **Why it breaks:** rule 10 — "Silent architecture drift is a breach; amend the ADR
  instead." §15 takes correction lines for D-255, D-299(2) and D-207 on exactly this
  ground and takes none for D-305, whose count this document's own §17 falsifies. The
  ADR that records the defect class is itself an instance of it.

---

## MINOR

15. **§8.2 carries a duplicated paragraph, spliced mid-sentence.** Lines 1125–1141:
    "It also dissolves three of §8.1's findings at once:" is cut off, the whole
    "**The guard is not decoration, and revision 6 dropped it.**" paragraph is
    repeated verbatim, and the sentence then restarts. An unclosed edit inside
    revision 7's own S-E repair.
16. **§17 misnames the round twice.** "Revision 7 addresses **the fourth round's**
    four BLOCKING findings" and "the fourth round's MAJORs". §16 records the
    4-BLOCKING round as the **fifth** (revision 6's); the fourth round (revision 5's)
    had 3 BLOCKING, 9 MAJOR. "What revision 7 still owes" therefore points at the
    wrong round's MAJOR list.
17. **§18.3 cites the wrong ADR item.** "the ROADMAP changes only by ADR (**§15 item
    6**)" — item 6 is the `Position` seam; the supersession deferral is item 10 (and
    item 9).
18. **§13 undercounts the red teams.** "The five DECISION-RED-TEAM rounds" with five
    timings, against §16's five per-matrix rounds **plus** the M5 round at `7ad466b`
    that supplied M5-E. Six rounds, five costed.
19. **§15's numbering is out of sequence** — 1–14, then 16, 17, 18, **15**, 20, 21,
    22, 23, **19** — so every cross-reference to a §15 item by number has to be
    resolved by search rather than by position.
20. **"Revision 7" names two different documents.** `d94dc0a` and `6feb40a` both carry
    the label; the latter adds 69 lines (§18). Under the Process section's own rule
    that an amendment reopens the review, a review "of revision 7" is ambiguous, and
    §12 item 5 makes exactly this argument against recording a SHA that goes stale.
21. **§0's subsections are out of order** — 0.2, 0.3, **0.6**, **0.5**, 0.4 — and
    there is **no §0.x table for what revision 6 got wrong**, breaking the convention
    every prior revision followed. A reader cannot see what revision 7 changed except
    from the header's prose.
22. **Closing line mis-cites D-305.** "four of the six changed their selection as a
    result — a base rate recorded as **D-305**". D-305 records the un-re-read-repair
    base rate; §5.6 and §16 both say the matrix base rate is recorded *here* rather
    than in an ADR line, and no D-30x carries it.
23. **§12 has no subsections**, yet §0 rows 12 and 34 and §16 cite "§12.5" and
    "§12.4". They resolve to list items 5 and 4.

---

## Rejected, with the attempted reproducer

Recorded so they are not re-found, per the Process section.

- **"§2.1's six destructure sites are wrong."** Reproduced and REJECTED. `grep -rn
  "let CandidatePolicy::Radius" crates/` returns five; the sixth
  (`pistol-cli/src/bin/pistol.rs:131`) is fully qualified as
  `pistol_engine::config::CandidatePolicy::Radius` and is real. All six paths and line
  numbers are accurate at this SHA.
- **"The design invents calculus IDs."** REJECTED. Every one of the 23 IDs the design
  cites exists in `docs/research/threat_calculus_v1.md`.
- **"The solver queries the design consumes do not exist."** REJECTED. All of
  `can_win_this_turn`, `blocking_covers`, `win_in_one_ply_cells`, `hot_windows`,
  `threat_cells`, `cells_raising_to_hot`, `live_windows_at_count`, `masks`,
  `min_hitting_set_exceeds`, `unblockable_double_threat`, `StonesLeft::from_state`,
  `From<StonesLeft> for HitBudget` are public. The one query that does not exist,
  `live_cells_at_count`, is correctly named as owed (§15 item 16).
- **"§5.2's cited source lines are wrong."** REJECTED. `cover.rs:145` is
  `min_hitting_set_exceeds`, `cover.rs:177` is `blocking_covers`,
  `unblockable_double_threat` (`cover.rs:257`) is literally
  `self.min_hitting_set_exceeds(defender_budget, self.hot_windows(side))` — the
  identity claim's ground is exactly as stated. `pvs.rs:134/266/325/378/380`,
  `score.rs:39`, `lib.rs`'s `pub(crate) mod position`, and `pvs.rs`'s 552 lines and
  RULE9-JUSTIFICATION text all check out.
- **"§9's registered quantity is below the marker too."** REJECTED. In
  `tools/baseline_snapshot.sh`, `position … nodes … depth_turns …` (line 494) and
  `ladder %s depth %s nodes %s …` (line 557) are written to the INVARIANT block;
  only `timing position`/`timing ladder`/`timing depth_at_500ms` (496, 560, 605) fall
  below `TIMING_MARKER` (168, emitted at 633). The §9 repair holds.
- **"SHELL_CHECKLIST does not have twelve items."** REJECTED. It has twelve
  (`## 1.`..`## 12.`); §9's "eight of twelve" is arithmetically right.
- **"§10's `1024`/`128` allowances are unsupported."** REJECTED. `18 × 17 = 306` and
  `6 × 17 = 102` are correct hex radius-2/radius-1 union bounds at 17 stones, and the
  census's larger `radius-2 ball` playouts mean (376.47) is a different, denser
  population.
- **"The design smuggles a hotspot other than D-263's."** REJECTED. D-263's flip
  clause is "Flips when WP-1.5b measures it, which replaces these readings with a
  rule-5 bench and this registration with a verdict." §12 item 4 measures D-263's two
  queries at the generator's own counts, states the substitution in bold, and takes
  ADR line 7 for it. That is the registration doing its job. The residual gap is
  finding 9, which is narrower.
- **Scope.** No finding. No `pistol-eval` storage work (§14 non-goal; §5.35 uses the
  existing `&mut dyn Eval` seam only); no quiescence (§5.5 defers to WP-1.6); rules
  truth untouched in `pistol-core` — §5.4 filters every staged cell through
  `Board::is_legal_placement` one cell at a time and cites D-77/D-20 against the
  radius shortcut, and §3 item 1 confirms `canonical_pair` and `phase_key` do not move.
- **Soundness gates as strength claims.** No finding. §8.3(a) states explicitly that a
  green tactical suite under Staged is NOT evidence about the cut, and hands the cut
  to SPRT, the movetime measurement and S-E. §8.6 states `REJ-DEPTHPROOF`. The
  gate/judge boundary is drawn correctly. The defect in gate (b) is B3, an absence,
  not a category error.

---

## STANDING RULE — restated, and it now binds

**This review FAILS, so there is NO revision 8.** The design is to be
RESTRUCTURED — split into smaller reviewable units, or narrowed in scope — not
revised again in place.

The evidence for the restructure, rather than for another revision, is B1 and B7
together:

- **B1** shows the artefact has already lost content it needs to be reviewable at
  all: three of six matrices exist only at `ec8f7fb`, and the sixth review round is
  the first that was explicitly asked to check them and could not.
- **B7** shows the mechanization that revision 7 rests its stopping decision on is
  green while the class it claims to close is present six times. §17 argues the odds
  changed because "the pin refuses … any four-decimal figure restated outside it";
  the document's restatements are percentages and three-significant-figure roundings,
  which is the shape prose actually uses.

A natural cut, offered because the standing rule asks for one rather than for another
pass: **(i)** the gate-supersession item (§4, M0) is independent of the search and is
already a self-contained decision with a live `tools/` precondition (B6) — it can land
or be dropped on its own; **(ii)** the node protocol (§5, M5) plus its tests is the
normative core and is the part the reviews have consistently *not* broken on the
merits; **(iii)** Tier-T qualification and the widening schedule (§6, §7, M1, M2) are
the config-and-population half, and are where the census, the restated figures and the
four-document tension all live; **(iv)** the soundness instrument (§8, §9, M3, M4) is a
gate-and-instrument work package with its own `tools/` change, its own SHELL_CHECKLIST
review, and the two undischarged mutation witnesses (finding 8). Each of the four is
reviewable in one sitting; the present document is not, and five of the six rounds
have failed on transmission between parts that do not need to be in one file.

Whatever the cut, three things travel with it: the M2/M3/M4 matrices must be restored
into whichever unit selects them (B1); M4 must get an ADR line (B2); and gate (b) must
be redefined or the "(a)–(d)" wiring rewritten (B3).
