# WP-1.5b CARVE — the section-owner table

<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->

The map from `docs/experiments/wp15b_design.md` at `6feb40a`
(revision 7, CLOSED by D-309 — which records the fresh-context REVIEW-design that FAILED it; the counts are D-309's and are not restated here) to the units the restructure
selected as option D by D-310 produced. **This table is the carve's own gate: an
unowned line or a double-owned line is a FAILED carve, not a finding.**

**The superseded document is deleted from the tree by the same commit that adds
this table.** It is retrievable at `6feb40a` and nowhere else. It is deleted
rather than kept as a superseded copy for a reason that is mechanical, not
tidiness: leaving it would double-own all 1975 lines and would put a second
census `BEGIN…END` block in the tree, which the pin
(`the_carved_design_units_carry_this_censuss_table_verbatim`) refuses by name —
the exact corruption a reviewer used against the old pin and which it now fails
the build on.

**LABEL DISCIPLINE — RETIRED BY `D-311`'s APPENDED AMENDMENT.** Revision identity for this table is its commit SHA; travelling item T5 is resolved by that retirement, as at every sibling document. The append history below is kept as the record of this table's own measurements, not as a label.

**THIS TABLE IS AT u-rev 6**: the `now` column is DELETED and replaced by
the command that derives it, discharging by deletion the standing duty that had gone
stale three times; §11's question is re-posed on the measured u-rev-6 numbers; and the
carve-provenance sentence above is corrected (it said revision 7 was *never reviewed*;
D-309 records that it was reviewed and FAILED). **At u-rev 5**: §11's unit-size table gains a FIFTH
column, re-measured at `161e6d3` after the design-closure round moved three of the
five units — U2 to 827, U3 to 846 and U4 to 1886. The u-rev 4 column, re-measured
at `7358a07` after U4's own repair moved it from 855 to 1413,
which is the re-measurement the u-rev 3 append promised in writing. At u-rev 3 it
gained a third column, re-measured at `0af32fb` after the repair round moved every
unit, and it recorded that the discipline of correcting a size in the commit that
falsifies it was not kept that round. At u-rev 2 that same table gained a
re-measured `now` column, because the sizes it recorded had been made false by
the B3 and MAJOR 8 repairs to U4.

---

## 1. THE GRANULARITY, STATED BECAUSE IT IS NOT UNIFORM

The brief asks for one owner per section §1–§18. Fifteen sections take exactly
one owner and are listed that way. **Four do not, because their contents are
per-item lists whose items belong to different units** — §11 (32 test rows), §12
(5 measurement items), §13 (7 cost rows) and §15 (23 ADR items). For those, the
row granularity is the ITEM, every item has exactly one owner, and the
enumeration is below. That satisfies the binding rule — F3's "every one of the
1975 lines gets exactly one owner" — which a coarser table would satisfy only by
putting U1's gate tests inside U4's document.

**MEASURED span of every section**, `awk '/^## /{print NR}'` at `6feb40a`:

| § | lines | count | owner |
|---|---|---|---|
| header | 1–74 | 74 | **DROPPED** |
| §0 | 75–172 | 98 | **DROPPED** |
| §1 | 173–204 | 32 | **U1** |
| §2 | 205–256 | 52 | **U2** |
| §3 | 257–294 | 38 | **U2** |
| §4 | 295–405 | 111 | **U1** |
| §5 | 406–760 | 355 | **U2** |
| §6 | 761–904 | 144 | **U3** |
| §7 | 905–1006 | 102 | **SEED** (88 lines) + **U3** (14 lines) — see §3 below |
| §8 | 1007–1286 | 280 | **U4** |
| §9 | 1287–1346 | 60 | **U4** |
| §10 | 1347–1446 | 100 | **U3** |
| §11 | 1447–1497 | 51 | *split by row* — see §4 |
| §12 | 1498–1659 | 162 | *split by item* — see §5 |
| §13 | 1660–1675 | 16 | *split by row* — see §6 |
| §14 | 1676–1684 | 9 | **U2** |
| §15 | 1685–1811 | 127 | *split by item* — see §7 |
| §16 | 1812–1853 | 42 | **DROPPED** |
| §17 | 1854–1902 | 49 | **DROPPED** (its owed-list travels — see §8) |
| §18 | 1903–1975 | 73 | *split by subsection* — see §9 |

74 + 98 + 32 + 52 + 38 + 111 + 355 + 144 + 102 + 280 + 60 + 100 + 51 + 162 + 16
+ 9 + 127 + 42 + 49 + 73 = **1975**. No gap, no overlap.

---

## 2. THE DROPS, each with its one-line reason

| § | Reason |
|---|---|
| header | Revision preamble and the revision-1-to-7 history of a document that no longer exists. Its one normative sentence restates D-266, which binds every unit without being restated; each unit cites D-266 in its own header |
| §0 | The superseded document's own defect history (rows 1–38). Every substantive correction it records is stated in the section it corrects, and those sections are owned. Retrievable at `6feb40a`. **B7 restatement site line 139 (`70.8 %`) is dropped with it** |
| §16 | The review record. Its round rows go to the unit whose matrix each round attacked, as that unit's lineage block (U1-A, U2-A, U3-A, U4-A) — one round-and-matrix pair, one owner, no row twice. Its two WP-wide paragraphs (the four-matrices base rate; "a repair is not done until every claim resting on the repaired thing has been re-read") are **LANDED as D-305** and are cited, not restated; restating a landed ADR is the defect D-305 itself records |
| §17 | The session's stopping verdict on revision 7. Superseded by D-309 (rev 7 closed on its failed review) and D-310 (restructure option D). Its "what revision 7 still owes" list is NOT dropped — see §8 |

---

## 3. §7 — the only section split by SENTENCE, and why

D-310 excises the widening text; the brief says "§6-7 MINUS widening text" to U3
and "the excised widening text, verbatim" to the seed. Measured:

| Part | lines | owner |
|---|---|---|
| §7.1 (why W-A fell), §7.2's schedule bullets, the mechanism, the struck `Impossible` bullet, the withdrawn `[16, 0]`, the registered reading and the root-argmax record | 88 | **SEED** |
| §7.2's bullet "THE CUT BINDS UNDER `CandidatePolicy::Staged` ONLY", and §7.2's bullet "the two SPRT seats therefore differ on a THIRD axis" | 14 | **U3** (§7) |

88 + 14 = 102. **Neither of the two bullets is about the schedule.** The first
scopes the whole `Staged` policy against `Radius` and is what U2's
`a_radius_policy_search_is_byte_identical_to_the_committed_engine` watches; the
second warns a reader of the SPRT verdict, and both of its named causes (the
overload return, §5.3's licensed shortening) are U2's. Leaving them in the seed
would put a claim a registered test rests on inside a file that is not
reviewable.

---

## 4. §11 — the 32 test rows

**U2 (20):** `overload_at_entry_scores_loss_without_expansion`,
`overload_check_is_not_taken_at_a_pv_node_or_the_root`,
`overload_check_guarded_by_own_win_now`,
`overload_composition_handles_completed_window_reading`,
`survival_filter_hits_all_plans_across_both_plies`,
`defensive_union_covers_nonminimum_two_stone_splits`,
`mate_in_1_by_pair_generated_in_tier_f_not_ranked_in`,
`the_table_move_ordering_under_staged_is_within_tier`,
`new_plan_creation_gets_no_forced_slot`,
`staged_ordering_deterministic_within_and_across_tiers`,
`stage_counters_reported_in_search_info`,
`a_win_now_node_generates_only_the_win_now_class`,
`cover_impossible_at_phase_zero_still_generates_the_win_now_class`,
`cover_impossible_at_phase_one_with_a_win_in_one_ply_cell`,
`stones_left_and_hit_budget_are_read_from_core_at_both_phases`,
`the_protocol_runs_after_the_horizon_return_and_the_table_cutoff`,
`a_forced_row_emits_no_tier_t_or_tier_q_cell`,
`the_two_predicates_agree_everywhere`,
`a_radius_policy_search_is_byte_identical_to_the_committed_engine`,
`the_threat_state_stays_in_step_with_the_game_and_the_eval`.

**U3 (4):** `tier_t_qualification_matches_adopted_matrix_option`,
`the_fallback_under_staged_answers_from_the_quiet_radius_ball`,
`no_candidates_under_staged_is_refused_by_a_policy_agnostic_error`,
`tier_t_cells_match_an_independent_window_walk`.

**U4 (5):** `gap_trap_answered_in_tier_f`, `colony_family_passes_under_staged`,
`tactical_suite_holds_at_its_rederived_thresholds_under_staged`,
`staged_filtered_set_equals_the_minimal_cover_union`,
`visit_searches_every_forced_candidate`.

**SEED (3):** `widening_schedule_fires_on_fail_low_and_is_deterministic`,
`the_root_and_every_pv_node_search_the_full_staged_universe`,
`a_truncated_fail_low_stores_no_transposition_record`.

20 + 4 + 5 + 3 = **32**. **§11.6** (D-295's `RULE-EXACT` residual) → **U4**.

---

## 5. §12 — the 5 measurement items

| Item | Subject | Owner |
|---|---|---|
| 1 | Snapshot before/after, the registered above-marker quantity | **U4** |
| 2 | Stage-share counters, the `SearchInfo` seam, the handshake line | **U2** — *except* the widening rate per node class and the declined-TT-entry count, which are stage Q's quantities and go to the **SEED**; the counter SEAM stays in U2, because it is what a later WP reads them through |
| 3 | The WP-1.4 adversarial-spread debt at `movetime 500` | **SEED** — on that class Tier F and Tier T are both EMPTY, so it measures the cut alone |
| 4 | D-263's hotspot, and Tier-T extraction registered rule-5-shaped | **U3** |
| 5 | The census pin, and what happened to the second instrument | **U3** |

§12's preamble sentence ("All ADVISORY on this machine; the operator re-runs for
the record") is a standing CONDITION on every measurement in every unit, not a
datum. Each unit that owns measurements states it once, and says that it is doing
so. It carries no number, so it cannot disagree with itself the way B5's count
did.

---

## 6. §13 — the 7 cost rows

| Row | Owner |
|---|---|
| `tools/ci.sh`, once — **5 m 20 s** | **U1** (which also owns §1.1's evidence cell for the same number; stating it once there removes a latent duplicate) |
| One baseline snapshot — **34.0 / 34.5 s** | **U4** |
| The census harness — **< 1 s** | **U3** |
| The gate-expiry re-measurement — **~2 min** | **U1** |
| The five DECISION-RED-TEAM rounds | **DROPPED** — it costs the superseded document's own rounds, not any unit's run, and MINOR 18 measured the count wrong (six rounds ran, five are costed). Retrievable at `6feb40a` |
| The soundness gate per CI run — **ESTIMATED 40–90 s** | **U4** |
| The operator's SPRT run | **DROPPED** — the pre-registration's, and the pre-registration is a separate step |

---

## 7. §15 — the 23 ADR items

| Owner | Items |
|---|---|
| **U1** | 1, 13, 14 |
| **U2** | 5, 6, 8, 10, 11, 17, 18, 19, 20, 21, 22 |
| **U3** | 2, 7, 12, 16, 23 |
| **U4** | 4, 15 |
| **SEED** | 3, 9 |

3 + 11 + 5 + 2 + 2 = **23**. Item numbers are retained unchanged in every unit so
an existing "§15 item n" citation still resolves; listing each unit's items in
ascending order dissolves **MINOR 19** (the superseded list ran 1–14, 16, 17, 18,
15, 20, 21, 22, 23, 19).

**§15's PREAMBLE is DROPPED.** MAJOR 10 measured it false on both clauses. Each
unit writes its own lead-in, and each states which of its items are the unit's
own and which are corrections to landed lines. **MAJOR 13's repair rides here:**
items 13, 14 and 23 have LANDED (D-302, D-303, D-304) and the superseded list
said so of item 12 alone.

Two items name a clause another unit consumes. Item 8's Tier-Q ball-scan clause
(WP-1.5c) is CITED by the seed and item 8's D-295 clause is CITED by U4; a
citation is not ownership, and item 8 is U2's.

---

## 8. §17's owed-list, which does not drop with §17

| Owed item | Owner |
|---|---|
| M4's and M6's mutation witnesses must become positions a legal game reaches | **U4** (OPEN) |
| The census owes a registered replication and a second instrument | **U3** (OPEN) |
| Rule 5 owes a bracket, an abort threshold and an nps/time-to-depth bench for the NODE PROTOCOL itself | **U2** (OPEN) |

---

## 9. §18 — the OPERATOR-QUEUE, by subsection

| §18.x | Owner |
|---|---|
| §18.1 the SPRT pre-registration | **DROPPED** — its content is `docs/experiments/wp15b_sprt_prereg.md`'s own §11 and its OPERATOR-CONFIRM slots. The pre-registration is a separate step (D-310's prereg consequence) and no unit owns it |
| §18.2 the ADVISORY measurements | **U4** (the snapshot triple) and **SEED** (the WP-1.4 spread baseline) |
| §18.3 the conservative branches | **U3** (the committed configs; D-263's remedies), **U2** (the ROADMAP supersession), **U1** (the `tools/` defects, CORRECTED: D-303 is now fixed at `8af9c5b`, D-302 is not) |
| §18.4 the handoff to WP-1.6 | **U2** (`LAW-RIPOSTE`/`LAW-LEDGER`, the node protocol's settled shape, the generalised overload verdict, D-111's stand-pat rule, and "what WP-1.6 must not inherit") and **U3** (the census is yours to extend) |

**MINOR 17 rides here:** §18.3 cited "§15 item 6" for the ROADMAP deferral, which
is the `Position` seam. It is items 10 (U2) and 9 (SEED). Retargeted.

---

## 10. B5's FOUR SITES, NAMED, AND WHERE THE COUNT IS STATED NOW

The superseded document stated the number of staged config documents **three
different ways across four sites**, and §2.2 is the section a reader consults for
scope. MEASURED at `6feb40a`:

| Site | line | Said | Now |
|---|---|---|---|
| §2.2 | 237 | "Staged ships as **three** config-selectable documents (§10)" | **U2** §2.2 — CITES U3 §10 and states no number |
| §10 lead-in | 1349 | "**Three complete**, `deny_unknown_fields`…" — immediately above a **four-row** table | **U3** §10 — **THE ONE PLACE THE COUNT IS STATED: FOUR**, matching the four rows of its own table |
| §10 body | 1358 | "**FOUR documents**, because three could not carry the requirement" | **U3** §10 — kept as the DERIVATION ("the fourth document exists because three could not carry the requirement"); it explains the count, it does not restate it |
| §18.3 | 1941 | "Staged would ship as **four** selectable documents" | **U3** (U3-Q) — CITES §10 and states no number |

**The count is FOUR**, and `docs/experiments/U3_tier_t.md` §10 is the only place
in the carve that says so.

---

## 11. CARVE VERDICT

**PASS.** 1975 lines, twenty sections, four units, one seed, one table. Every
section, every test row, every measurement item, every cost row and every ADR
item has exactly one owner or an explicit DROP with its reason. Nothing is
unowned; nothing is owned twice.

**MEASURED unit sizes after the carve** — recorded because F6's flip clause was
about a unit exceeding one sitting, and after the carve the largest unit is not
the one F6 named:

| Unit | at the carve (u-rev 1) | at u-rev 2 of this table | at u-rev 3 (`0af32fb`) | at u-rev 4 (`7358a07`) | at u-rev 5 (`161e6d3`) |
|---|---|---|---|---|---|
| U1 gate supersession | 274 | 274 | 329 | 329 | 329 |
| U2 node protocol | 754 | 754 | 799 | 799 | 827 |
| U3 Tier-T and the config shape | 603 | 603 | 818 | 818 | 846 |
| U4 soundness instrument | 701 | 800 | 855 | 1413 | 1886 |
| WPQ seed (not reviewable) | 222 | 222 | 227 | 227 | 227 |
| this table | 268 | *not measured* | 287 | 321 | 337 |

**THERE IS NO `now` COLUMN AT u-rev 6, AND ITS REMOVAL IS THE POINT OF THIS APPEND.**
Every column above is a MEASUREMENT PINNED TO A REVISION and is true forever at that
revision. A `now` column is a claim about the working tree, it is false the moment any
unit is appended to, and **this table has had to disclose it going stale three
times** — at u-rev 3, at u-rev 4, and at u-rev 5, each time as a batch correction taken
after the fact rather than in the commit that falsified it, which the table itself
recorded as the weaker discipline. **The standing duty that produced those corrections
is DISCHARGED BY DELETION rather than by another correction.** What replaces it is the
command:

```
$ wc -l docs/experiments/U{1_gate_supersession,2_node_protocol,3_tier_t,4_soundness_instrument}.md \
        docs/experiments/WPQ_seed.md docs/experiments/section_owner_table.md
$ for f in U1_gate_supersession U2_node_protocol U3_tier_t U4_soundness_instrument WPQ_seed section_owner_table; do
    printf '%-28s ' "$f"; grep -m1 -o '^\*\*u-rev [0-9]*' docs/experiments/$f.md; done
```

This is the same substitution `docs/experiments/U2_node_protocol.md` (u-rev 6, landed
`3543a7f`) made for its exceptions list at `7dfd047` — a hand-maintained set replaced
by a rule and a derived enumeration — which is the precedent
`docs/experiments/matrix_U4R_REDTEAM_round2.md` identifies as *"the only candidate
whose prevention claim is a mechanism rather than an assertion"* (D-334). **AND THIS
TABLE CLAIMS NO MORE FOR IT THAN THAT.** The command is an instrument, not a proof:
the sibling unit registered a derived command for a different claim at its u-rev 6 and
the command turned out to be BLIND to a case it was about
(`docs/experiments/wp15b_U3_REVIEW_urev6.md`, MAJOR D). What makes this one safer is
narrower than a general argument — `wc -l` over an explicit file list cannot miss a
file the list names, and the list is the same one `CARVE_DOCS` pins — and that is the
whole of the claim.

**THE THIRD COLUMN IS RE-MEASURED AT u-rev 3 OF THIS TABLE, AND THE SECOND HAD
GONE FALSE AGAIN BEFORE IT WAS READ.** Command, at `0af32fb`:

```
$ wc -l docs/experiments/U{1_gate_supersession,2_node_protocol,3_tier_t,4_soundness_instrument}.md \
        docs/experiments/WPQ_seed.md docs/experiments/section_owner_table.md
   329 docs/experiments/U1_gate_supersession.md
   799 docs/experiments/U2_node_protocol.md
   818 docs/experiments/U3_tier_t.md
   855 docs/experiments/U4_soundness_instrument.md
   227 docs/experiments/WPQ_seed.md
   287 docs/experiments/section_owner_table.md
```

**Every row moved, and one of them moved before this table was even read**: the
`800` recorded for U4 at u-rev 2 was already `855` at `35aab95`, found by the
fresh-context reviewer of U3 rather than by this table. The rest moved in the
repair round that answered the four unit reviews — U1 to u-rev 2 (`5baea10`), U2
and the seed to u-rev 2 (`56b0bec`), U3 to u-rev 3 (`7d5d39c`).

**U4's ROW MOVED AGAIN, EXACTLY AS THE u-rev 3 APPEND SAID IT WOULD, AND THIS IS
THE RE-MEASUREMENT IT PROMISED.** Command, at `7358a07`:

```
$ wc -l docs/experiments/U4_soundness_instrument.md
1413 docs/experiments/U4_soundness_instrument.md
```

**855 → 1413 at u-rev 6, `7358a07`** — the repair of `wp15b_U4_REVIEW.md`'s
FAIL (3 BLOCKING, 3 MAJOR, 5 MINOR) plus two matrix folds landing in one pass:
M3's SELECTION of S-M (D-323) and M4's third STOP (D-324). No other unit moved.

**AND IT MAKES §11's QUESTION SHARPER RATHER THAN ANSWERING IT.** The property
the whole restructure was bought for is "reviewable in one sitting". At the carve
the largest unit was U2 at 754; at u-rev 4 the largest was U4 at 1413, which was
87 % larger than anything the four-way cut was judged against and 72 % of the 1975-line
document the carve replaced. **That is a finding for the architect and not a
change this table may make** — it is the same question §11 already hands over,
posed on a number that has since nearly doubled.

**THE RULE THIS ROUND OWED AND DID NOT KEEP, recorded because the alternative is
the defect itself:** the standing discipline is that the owner table's MEASURED
sizes are corrected IN THE SAME COMMIT that falsifies them. The three repair
commits above each falsified a row and none of them corrected it; this append is
a batch correction taken afterwards, which is weaker, and it is disclosed here
rather than presented as the discipline working. The earlier note's own history:
`wc -l` at the commit that added the second column. U4 grew by **99 lines** across two commits — the B3 repair (D-316: the
gate letters dropped for four named gates, its selection record, and the MEASURED
correction that six cross-references retarget where the cost cell predicted
three) and MAJOR 8's repair (both mutation witnesses rebuilt as positions a legal
game reaches, pinned by
`crates/pistol-solver/tests/wp15b_mutation_witnesses.rs`). U3 grew by 0 net.
**Neither commit appended to THIS table, so D-311's bump was not owed then and is
owed now**, by the append you are reading.

**Recorded rather than quietly corrected**, because a stale MEASURED number in
the document that calls itself the carve's own gate is the defect class this work
package has failed on six times, and because the number was made stale by the
repairs and not by the carve. The `at the carve` column is kept so the figure
F6's flip clause was judged against stays retrievable.

**THE FIFTH COLUMN IS THIS RE-MEASUREMENT, AND U4's OWN u-rev 7 RECORDS THE DUTY
THAT PRODUCED IT.** Command, at `161e6d3`:

```
$ wc -l docs/experiments/U{1_gate_supersession,2_node_protocol,3_tier_t,4_soundness_instrument}.md \
        docs/experiments/WPQ_seed.md docs/experiments/section_owner_table.md
   329 docs/experiments/U1_gate_supersession.md
   827 docs/experiments/U2_node_protocol.md
   846 docs/experiments/U3_tier_t.md
  1886 docs/experiments/U4_soundness_instrument.md
   227 docs/experiments/WPQ_seed.md
   337 docs/experiments/section_owner_table.md
```

**Three of the five units moved in one round** — U2 to u-rev 3 (`d85b049`), U3 to
u-rev 4 (`6f2dfe6`), U4 to u-rev 7 (`0f49c90`) — and U1 and the seed did not move
at all. The row that matters is U4's: **1413 → 1886**, the repair of a FAIL that
carried a BLOCKING plus the fold of M4's axis-A selection (D-329) and D-330.

**AND §11's QUESTION IS NOW POSED ON A NUMBER THAT HAS MORE THAN DOUBLED SINCE
THE CUT WAS JUDGED.** U4 is **1886** against the largest-at-the-carve U2 at
**754** — **150 % larger**, and **95 %** of the 1975-line document the four-way
carve replaced. The unit that the restructure existed to make reviewable in one
sitting is now within ninety lines of the document it was carved out of. This
table states that and does not resolve it; it is the architect's.

**AT u-rev 6 THE ANSWER IS WORSE, AND THE ROUND THAT WAS SUPPOSED TO FIX IT IS THE
ROUND THAT MADE IT WORSE.** MEASURED at `a2b50bf`:

```
$ wc -l docs/experiments/U{1_gate_supersession,2_node_protocol,3_tier_t,4_soundness_instrument}.md \
        docs/experiments/WPQ_seed.md docs/experiments/section_owner_table.md
   329 docs/experiments/U1_gate_supersession.md
   925 docs/experiments/U2_node_protocol.md
   990 docs/experiments/U3_tier_t.md
  2105 docs/experiments/U4_soundness_instrument.md
   227 docs/experiments/WPQ_seed.md
   371 docs/experiments/section_owner_table.md
  4947 total
```

**U4 IS 2105 — it PASSED the 1975-line document it was carved out of**, and it is
**179 % larger** than the largest unit the four-way cut was judged against. U2 and U3
grew too, by roughly a hundred and fifty each. **THE ROUND THAT PRODUCED THESE NUMBERS
WAS THE ROUND CONVENED TO SHRINK U4's STATUS MATTER**, under the CLAIM-HOME law
(D-331): a micro-matrix was authored twice to restructure it, **both fields were killed
by fresh-context red teams, and D-334 stopped it with no selection**. What happened
instead is that every unit's findings were repaired in place — and **every repair is
net additive, because each one discloses what it replaced.**

**THAT IS THE FINDING, AND IT IS NOT A COMPLAINT ABOUT DISCLOSURE.** The disclosures
are why four consecutive rounds could each name the previous round's defect. But it
means the property the restructure was bought for — *reviewable in one sitting* — has
now moved against the project in every round since the carve, **and it still has no
instrument**: nothing in this project defines it, measures it, or can say whether 2105
lines fails it while 1886 passed. Every number above is a PROXY for an undefined
property, which `docs/experiments/matrix_U4R_REDTEAM_round2.md` charged against the
matrix that tried to use them to rank options. **This table states the numbers, states
that they are proxies, and states that the question is unanswerable as posed. Defining
the property is the architect's, and it is the prior question to any revision 3.**

**THIS RE-MEASUREMENT WAS OWED BY A RULE AND NOT BY A REMINDER**, which is the
difference from the two rounds that went stale. U4's u-rev 7 replaced its own
stale owner-table row with a STANDING DUTY — the row re-opens on every bump of
that unit rather than recording a size that will be wrong at the next one — and
this append is the first discharge of it.

**This is a finding for the architect, not a change the carve may make.** D-310
selected a four-unit cut; F6's arithmetic predicted unit 3 at ~425 and unit 2 at
355. Measured after the carve, with the remainder F3 found unowned now assigned,
**U2 was 754 lines at the carve and U3 was 603 — and at `161e6d3` they are 827
and 846, with U4 at 1886** — the remainder was 923 lines and it had to go
somewhere. Whether "reviewable in one sitting", which is the property the whole
restructure was bought for, survives at those sizes is a selection question and
is the architect's.

---

*Carve table.*
