<!--
PROVENANCE — this file is a LANDED EPHEMERAL REPORT.

- Report:            DECISION-RED-TEAM against the WP-1.5b design restructure matrix.
- Dispatching session id: 28856979-30d2-48d6-b1c8-5dc584ab6c03
- Original path:     /tmp/claude-1000/-home-tom-Projects-HeXO-AlphaBeta/
                     28856979-30d2-48d6-b1c8-5dc584ab6c03/scratchpad/
                     wp15b_restructure_REDTEAM.md
                     (session-scoped tmpfs; does not survive the session — this
                     landing is the only retrievable copy)
- SHAs the report examined: the matrix was attacked AS IN-SESSION CHAT TEXT, at no
                     revision — that absence is the report's own F0. The text it
                     attacked was landed verbatim afterwards as
                     `docs/experiments/restructure_matrix_15b.md` at eea480b.
                     Its other three subjects were at a revision and are named in its
                     header: design 6feb40af1f1c12c1977d7a2030509dd98cbdc8ac,
                     review report `wp15b_design_rev7_REVIEW.md` against that SHA
                     (landed alongside this file), SPRT pre-registration ca0d331
                     (revision 4).
- Landed at:         2026-08-22, tree at cf74594.
- Cited by:          D-310 (selects option D; carries this report's strongest
                     surviving attack), and the eleven-finding summary in
                     docs/experiments/restructure_selection_15b.md, which was written
                     because this file was not yet retrievable.

THE BODY BELOW IS VERBATIM AND UNEDITED. Nothing in it is corrected, re-scoped or
annotated here: a report edited after the fact is a report that was never written.
F0 in particular is left standing as written even though this landing discharges it.
-->

# DECISION-RED-TEAM — WP-1.5b design restructure matrix

- **Revision attacked:** matrix as supplied in-session (unversioned, not on disk — see F0).
  Design `6feb40a`; review report `wp15b_design_rev7_REVIEW.md`; SPRT prereg at `ca0d331` (rev 4).
- **Matches HEAD:** YES for design, review and prereg. Working tree clean.
- **Context:** fresh. Did not author the matrix, the design, or the review.

## VERDICT

**The recommendation does not survive in its recommended form.** Option A survives as a
cut; **A-as-recommended does not**, because its own flip clause 2 already fires on a
measurement available before this red team ran (F6), and both grounds it cites against
its nearest rival are void against the primary sources (F9). The matrix's founding
premise is falsified by the document's own history (F2). Selection is the architect's.

Eleven findings. F2, F5, F6, F9 are the kills.

---

## F0 — The matrix has no revision and no path

It exists only as chat text. CLAUDE.md binds the instrument as tightly as the document:
"an artefact that produces a registered number … is named in the pre-registration WITH
ITS REVISION". A matrix whose surviving option takes an ADR line is such an artefact —
the ADR must cite what was attacked. `find / -iname "restructure_matrix*"` returns
nothing. **Land the matrix at a SHA before citing this attack**, or the ADR line records
an attack on a text nobody can retrieve. Not a merits finding; a prerequisite.

## F1 — Not one numeric claim in the matrix is marked MEASURED or ESTIMATED

Unmarked numerics: "4 reviews, one sitting each", "7 reviews", "2 wasted sittings",
"2 reviews", "3 reviews now, one WP later", "5 rounds", and the Facts block's causal
claim "**Correlates with document size, not content**". CLAUDE.md: "EVERY NUMERIC CLAIM
IN THE MATRIX IS MARKED **MEASURED** OR **ESTIMATED**, and an estimate that could have
been measured in seconds is a finding — twice in one round a matrix argued that
unmeasured claims are the failure mode while resting on one (D-291)."

This matrix's subject is a document that failed six rounds partly for unmarked figures
(review MAJOR 12), and its own load-bearing cell — "one sitting each" — was one `awk`
away from measurement. This is D-291's precedent recurring for the third time.

## F2 — KILL: the founding premise is false against the document's own history

"Recurring defect class … **Correlates with document size, not content.**" That premise
is the whole of the recommendation's closing sentence ("The recurring defect class is
size-driven and A is the smallest cut"). **MEASURED**, `git show <rev>:… | wc -l`:

| revisions | size | growth | recorded instances |
|---|---|---|---|
| 1–6 (`ec8f7fb`→`2d07ff6`) | 726 → 1770 | **+1044** | **9** (D-305, landed before rev 7) |
| 7 (`2d07ff6`→`d94dc0a`) | 1770 → 1906 | **+136** | **10** (three self-declared, seven found by the review: "Instances thirteen through nineteen") |

**0.86 instances per 100 added lines across revisions 1–6; 7.35 across revision 7 —
8.5× the rate at one-eighth the growth.** Revision 7 is the smallest-but-one growth step
and the largest instance harvest in the work package's life. Size is not the variable
that moved. What moved was the **number of repairs**: revision 7 is the revision that
repaired §10, §15, §17 and §18 at once, and the review's own diagnosis of every one of
its seven blockers is a repair whose dependent claim was not re-read.

This inverts A's argument. A does not reduce repairs. It multiplies the boundaries each
repair must propagate across and relocates them **between files** — and B4 and B5, the
two blockers of exactly this class, were both found by grepping one file
(`grep "(b)"`, the four config-count sites). After A, that grep needs four paths and a
reviewer who knows to use them.

## F3 — A's four units leave 923 of 1975 lines unassigned, and T6 is unsatisfiable for B5

Section spans at `6feb40a` (`awk '/^## /{print NR}'`):

| unit | sections | lines | count |
|---|---|---|---|
| 1 | §4 | 295–405 | 111 |
| 2 | §5 | 406–760 | 355 |
| 3 | §6–§7 | 761–1006 | 246 |
| 4 | §8–§9 | 1007–1346 | 340 |
| — | header + §0–§3 | 1–294 | 294 |
| — | §10–§18 | 1347–1975 | 629 |

Assigned 1052. **Unassigned 923 (46.7 %)** — the config shape (§10), the test plan
(§11), the measurement register (§12), the 23 owed ADR lines (§15), the review record
(§16), what the WP still owes (§17). "No separate integration doc" does not assign them.

**T6 has no referent for B5.** T6: "B4, B5, B6 land in whichever unit owns their text."
B5's four contradicting sites are §2.2 (line 237), §10 lead-in (1349), §10 body (1358),
§18.3 (1941). **No unit owns any of them.** Same for two of B7's six restatement sites:
line 139 (§0) and line 1442 (§10). T6 is unexecutable as written for the blocker whose
text is most scattered.

## F4 — The named suspect is the wrong coupling; the real one is a build-order prerequisite

A's failure-mode cell names "gate refs span units 1 and 4" — a documentation-reference
coupling. **MEASURED**, the load-bearing coupling is units 1 and 2, and it is a Cargo
dependency-graph prerequisite:

- `Cargo.toml` `[workspace.dependencies]` lists core/eval/search/engine/cli.
  **`pistol-solver` is absent.** `crates/pistol-search/Cargo.toml` takes core and eval only.
- §5.1–§5.3 realise the node protocol out of `win_in_one_ply_cells`, `can_win_this_turn`,
  `blocking_covers` — all `pistol-solver`, called inside `visit`. **Unit 2 is the commit
  that creates the edge.**
- §4.1 measures unit 1's two gates firing on exactly that edge: `solver_edge_check.sh`
  → exit 1; `solver_link_check.sh` → exit 1, 30 hits over 5 binaries.

Land unit 2 without unit 1 → CI red. Drop unit 1 → unit 2 cannot land. The units are
**ordered**, and the order is the reverse of A's numbering: unit 1 is a precondition
commit. A's cost cell ("4 reviews, one sitting each") prices four independent sittings;
neither flip clause can fire on this coupling, because flip clause 1 is scoped to
§4/§8-9.

## F5 — KILL: flip clause 1 is incoherent — its remedy does not address its own trigger

> "Red team shows the §4/§8-9 gate wiring cannot be stated as checkable per-unit
> import/export claims: **flip to D** (defer the coupled half)."

D is "Restructure remainder **per A minus unit 3**" — D defers §6-7. **Units 1 and 4 are
both retained under D.** A finding that units 1 and 4 cannot be decoupled is answered by
deferring a third unit that is not party to the finding. The parenthetical "(defer the
coupled half)" describes something D does not do. The clause is unfireable as written:
no observation about §4/§8-9 makes D the remedy.

## F6 — KILL: flip clause 2 has already fired, and it was measurable before this attack

> "Red team shows unit 3 (§6-7) still exceeds one sitting after T1 restoration: split
> unit 3 only, A stays otherwise."

**MEASURED:**
- Unit 3 today: 246 lines.
- T1 restoration: M2's `ec8f7fb` table is 32 lines; a W-E row + recommendation + a
  recorded surviving attack, sized off M5-E's own precedent in §5.6, is ~45 more.
  → **~325 lines.**
- Plus §10's config shape (100 lines: `widen_schedule`, `quiet_top_k`, the tier
  thresholds), which F3 says must be owned and which no other unit can own.
  → **~425 lines**, before §12's widening-rate and stage-share registrations.
- Unit 2, the largest as drawn: **355 lines**, needing no restoration.

So after T1, **unit 3 is the largest unit in the cut**, and it is simultaneously the
owner of B5 (four sites), B7 (the pin block plus two of six restatements) and — per F7 —
one of the two open selections. The condition the matrix wrote as a red-team hypothesis
is satisfied by arithmetic on its own inputs. **A-as-recommended is not the surviving
form of A;** the five-unit variant is, and the matrix's Cost cell of "4 reviews" is
wrong before any option is chosen.

## F7 — T1's "cost is common" is false, and it is false in the direction of the recommendation

T1 is placed above the options with "cost is common", which excludes it from the Cost
column. **MEASURED**, `git show <rev>:… | grep -c`:

```
ec8f7fb : W-E 0   S-E 0
```

- **M2** at `ec8f7fb` holds W-A..W-D and recommends **W-A**. Adopted: **W-E**. Not there.
- **M3** at `ec8f7fb` holds S-A..S-D and recommends **S-D**. Adopted: **S-E**. Not there.
- **M4** at `ec8f7fb` holds N-A..N-D and recommends **N-A** — the adopted option.
  **M4 alone is a mechanical recovery.**

The design says why, in §5.6: "the fourth time in this work package's review round that
the surviving option was one the matrix did not contain (M0 (f), M2 W-E, M3 S-E, M5 E)."

T1's rule handles this — a reauthored row "differs", so a fresh DECISION-RED-TEAM is
required before the selection is cited. **But that makes T1 two red-team rounds, and
they are not common cost:** under A they land in units 3 (M2) and 4 (M3); under D, unit
3 is deferred and the matrix itself says so ("Widening matrices (T1) deferred, not
escaped"). T1 is therefore **differential** between A and D, and pricing it as common
understates A relative to D — the direction that favours the recommendation. T1's own
wording ("Recover from ec8f7fb where present, reauthor where absent") conceals the split
by not saying which is which: it is one of three, not a general case.

## F8 — A's mitigation is unsound under the project's own review-revision rule, and it discards the only mechanization

A's mitigation: "each unit stating its imports/exports as named claims **the other
unit's review checks**." Combined with T5 (revision label bumps on any append), this is a
mutual claim graph across four documents with **independent revision lifecycles**.
CLAUDE.md: "Reviews of superseded revisions do not transfer — an amendment reopens the
review, however small the diff." Unit 2's reviewer checking unit 3's export claim at
unit 3's revision *n* is checking a superseded text the moment unit 3 bumps. The matrix
specifies no pinning discipline for cross-unit claims, so the mitigation cannot be
executed without one — and a reviewer who must read the other unit to check it is not
having "one sitting each".

**And the one mechanized check does not survive the cut.**
`crates/pistol-solver/tests/wp15b_census.rs:638` builds its scan target as a literal:

```rust
"/../../docs/experiments/wp15b_design.md"
```

with "outside" defined (line 673) as that one file minus the BEGIN/END block. Split into
four files and the pin **stays green while blind to three of them** — EXIT-0-WRONG-ANSWER,
the class `tools/SHELL_CHECKLIST.md` exists for. T4 says "scan all restated numerics **or**
weaken §6.2"; it does not say the pin becomes multi-path, and its second branch (weaken
§6.2) leaves the split with no mechanization at all. **T4 must name the path list as a
travelling artefact with its own revision**, or A trades the only automated transmission
check for four manual ones.

## F9 — KILL: both grounds the recommendation cites against D are void

> "avoids **D's prereg reopen** and **closure-criterion risk**"

**(a) The prereg reopen is sunk cost, not a differential.** `docs/experiments/wp15b_sprt_prereg.md`
at `ca0d331`, line 3: "**Revision 4. DRAFT. THIS DOCUMENT GOVERNS NOTHING YET.**" Line 34:
"It names `configs/instrument_staged_v0.toml`, which **does not exist at this revision**,
so it cannot govern a run and **no run has been taken** under it. It becomes governing
when three things hold together: the config exists, every OPERATOR-CONFIRM slot in §9 is
filled, **and the document passes a fresh-context review AT THE REVISION THAT GOVERNS THE
RUN**." That review is owed under **every** option, D included and excluded. D's
differential is one edit to §1's axis (1) and one to §3's engine-A row.

**And D's cell reports only one sign.** The prereg's §1 registers three confounded axes
and states: "A verdict here is a verdict on the three together. Attributing it to any one
of them would need an experiment this document does not describe." Dropping the quiet cut
removes one axis. **D produces a better instrument for the same rule-6 claim**, and the
failure-mode cell does not say so.

**(b) The closure-criterion risk rests on a number the design demoted.**
D's cell: "`depth_at_500ms` movement may not clear noise, closure criterion at risk."
Design line 1313: "**`depth_at_500ms` is demoted to below-marker CONTEXT** and its dead
band is stated so an unmoved triple is not read as a null result." Line 1500: "All
ADVISORY on this machine." Line 1504 makes the registered quantity `depth_turns` and
`nodes` at 50 000 — `depth_at_500ms` is "reported as context". `docs/ROADMAP.md:140`
states the exit: "engine refutes the tactical fixture class at pre-registered thresholds;
every landed change SPRT-positive." **`depth_at_500ms` is not in it.** An advisory,
demoted, dead-banded context number cannot put the closure criterion at risk.

The residue of D's failure mode — "strength delta smaller" — is real, unmarked, and
unmeasurable before the implementation exists, which is the same standing W-A's own cell
declared honestly at `ec8f7fb`. **The A-vs-D comparison was decided on two grounds that do
not exist and one that cannot be evaluated yet.**

## F10 — B's cost cell contradicts the matrix's own Facts block

"M0, M5 already clean = 2 wasted sittings." The Facts block's clean list is: "node
protocol (§5, M5-E), solver query surface, calculus IDs, scope, hotspot registration, six
destructure sites, code line citations." **M0 is not on it** — and B6, a BLOCKING, is
M0's: §4.2's option (f) cost cell and §4.4 make the two `tools/` changes a *condition* of
the adopted option, and at this SHA `tools/solver_edge_check.sh:103` is still bare `pwd`
with no `--color never` on any of the three `cargo tree` calls. One wasted sitting, not
two. B loses anyway on round count; the cell is still wrong, and it is wrong in the
direction that flatters the recommendation.

## F11 — Two options missing; the standing rule's other disjunct is *not* missing

D is the narrowing branch, so the matrix does answer "split **or** narrow" in full.
Two rows are still absent:

- **Cut by ARTEFACT, not by section.** F3's 923-line remainder exists because the unit of
  division is "a `##` heading containing a matrix". The document's real seams are the
  design prose, the config set (§10 — four `.toml` files that must validate together
  under `deny_unknown_fields`), the `tools/` instruments (§4's golden transcript, §8.7's
  `staged_soundness_check.sh`, §9's `baseline_snapshot.sh --config` — three
  SHELL_CHECKLIST reviews), and the test tree (§11, the census, the mutation ledger).
  That cut has **no unassigned remainder** and puts three SHELL_CHECKLIST reviews in one
  reviewer's hands. It may lose; it must be on the page to lose.
- **The null row.** Six revisions, six FAILs, nineteen instances of one class. Every row
  in this matrix is a continuation. The baseline they must beat — withdraw WP-1.5b to its
  calculus, or defer it behind WP-1.6 — is not costed. The cheapest row on the page
  already carries 4 reviews + 2 fresh red-teams (F7) + 3 SHELL_CHECKLIST reviews + a
  ROADMAP ADR (below). Continuation being the cheap branch is assumed, not shown.

**Also uncosted by every splitting option:** a four-way split *is* a work-package cut,
and CLAUDE.md puts those in `docs/ROADMAP.md`, "changed only by ADR". §15 already owes
two unlanded ROADMAP-by-ADR lines (items 9 and 10). Splits add a third; D adds one too
(the follow-up WP). C and the null row add none.

---

## Per-option survival

| Option | Verdict |
|---|---|
| **A** | **SURVIVES WOUNDED, not in its recommended form.** Flip clause 2 already fires (F6) → the surviving variant is five units, not four. Cost understated by two red-team rounds (F7) and by the §5↔§4 ordering (F4). Its mitigation needs a cross-unit revision-pinning discipline and a multi-path pin it does not specify (F8). Must assign 923 lines (F3). Its stated advantage over D is void (F9). |
| **B** | **FALLS**, on round count, as the matrix says — but its cell is wrong (F10). Note B is the only option that gives each restored matrix its own reviewable home, which is what T1 actually needs; if F7's two red-team rounds are priced honestly, B's gap to A narrows. |
| **C** | **FALLS.** The "relabels" charge holds: unit 2 would be ~1500 lines. Its own cost cell shares A's omission of T1's red teams. |
| **D** | **SURVIVES, and is cheaper than the matrix shows** (F9). Not clean: it retains units 1, 2 and 4 and therefore F4's prerequisite ordering, and it defers M2's open selection rather than closing it — which the matrix states honestly. D's real cost is the follow-up WP's own ROADMAP ADR and the fact that the widening schedule's soundness argument (§7.1's `REJ-DEPTHPROOF` perimeter) is what unit 3 was going to carry. |

## STRONGEST SURVIVING ATTACK — for the ADR line

> A's central ground is that the recurring defect class is size-driven, so the smallest
> cut that isolates every blocker wins. **MEASURED against the document's own history,
> the premise is false:** revisions 1–6 added 1044 lines and produced nine recorded
> instances (0.86 per 100 lines); revision 7 added 136 and produced ten (7.35 per 100
> lines) — 8.5× the rate at one-eighth the growth. The variable that moved was the number
> of repairs, not the number of lines. A does not reduce repairs; it multiplies the
> boundaries each repair must cross and relocates them between files, where the two
> blockers of exactly this class (B4's "(b)" and B5's four config-count sites) were both
> found by grepping one file, and where the single mechanized check that could have caught
> one resolves a hard-coded path (`wp15b_census.rs:638`) and will stay green over the three
> files it no longer reads. A is the remedy for the diagnosis the matrix asserted, not for
> the one its own evidence supports.
