# WP-1.5d — SCOPED RE-REVIEW, design revision 3

## Header

**Artefact reviewed:** `docs/experiments/wp15d_design.md` at commit
`980bbb53d292cc93f03a62494e63e74e362f0573` ("docs(wp15d): design revision 3
closes the four MINORs too — the check is named, the seat's K is not the
calibrated one, and the void threshold says why it can fire"), **revision 3**,
542 lines.

**Does it still match HEAD?** YES.

```
$ git rev-parse HEAD
980bbb53d292cc93f03a62494e63e74e362f0573
$ git status --short
(no output)
$ git branch --show-current
dev
```

**Reviewer:** fresh context. Did not write the design, the matrix, any of the
three DECISION-RED-TEAM rounds, or the revision-2 REVIEW.

### THE SCOPING DIFF — PROVED, NOT ASSERTED

```
$ git diff 13720fc..980bbb5 -- . ':(exclude)docs/experiments/wp15d_design.md'
diff --git a/docs/experiments/wp15d_design_REVIEW.md b/docs/experiments/wp15d_design_REVIEW.md
new file mode 100644
index 0000000..b924882
--- /dev/null
+++ b/docs/experiments/wp15d_design_REVIEW.md
@@ -0,0 +1,850 @@
   [... 850 added lines, the revision-2 REVIEW report, no other file ...]

$ git diff --stat 13720fc..980bbb5
 docs/experiments/wp15d_design.md        | 229 +++++++--
 docs/experiments/wp15d_design_REVIEW.md | 850 ++++++++++++++++++++++++++++++++
 2 files changed, 1031 insertions(+), 48 deletions(-)
```

**The diff is not empty, and the one file outside the design document is the
revision-2 REVIEW report being landed.** That is the report the fix round is
answering; landing it is required (`docs/decisions.md` D-469's class, and the
standing problem that review reports otherwise survive only in ephemeral
scratchpads). **NO ENGINE CODE, NO CONFIG, NO `tools/` SCRIPT AND NO ADR LINE
WAS TOUCHED.** The fix round is properly scoped. This is **not a finding**, and
it is recorded here because the dispatch asked for the diff's output rather than
a claim about it.

Per-commit:

```
$ git show --stat --oneline 14b01c0
14b01c0 docs(wp15d): design revision 3 -- the fix round, ...
 docs/experiments/wp15d_design.md        | 197 ++++++--
 docs/experiments/wp15d_design_REVIEW.md | 850 ++++++++++++++++++++++++++++++++
$ git show --stat --oneline 980bbb5
980bbb5 docs(wp15d): design revision 3 closes the four MINORs too ...
 docs/experiments/wp15d_design.md | 36 ++++++++++++++++++++++++++++--------
```

### WHAT I READ

`docs/experiments/wp15d_design.md` (rev 3, all 542 lines);
`docs/experiments/wp15d_design_REVIEW.md` (rev 2's REVIEW, all 850 lines);
`CLAUDE.md` in full; `docs/process.md` in full;
`docs/experiments/WPQ_seed.md` §7.2; `docs/decisions.md` D-7, D-9, D-22, D-74,
D-95, D-124, D-291, D-356, D-374, D-388, D-395, D-398, D-423, D-424, D-473–D-478;
`tools/bench_block.sh`; `tools/determinism.sh:55-95`; `tools/SHELL_CHECKLIST.md`
via `docs/process.md`'s coverage rule; `artifacts/wp15d_turn_axis_v1.txt`
(header, `SCOPES`, the S1/S2 harness source and every S1/S2 row);
`artifacts/wp15d_worktree_export_receipt_v2.txt`; `docs/experiments/matrix_M2.md`
§3–§4.

**Code read:** `crates/pistol-core/src/rules.rs:20-40`,
`crates/pistol-core/src/movegen.rs:89-115`,
`crates/pistol-search/src/candidates.rs:1-70`,
`crates/pistol-search/src/staged.rs:200-245`,
`crates/pistol-search/src/pvs.rs:140-200,310-350,515-530,600-650,720-745`,
`crates/pistol-search/src/search.rs:225-250,315-380`,
`crates/pistol-search/src/info.rs:1-60`,
`crates/pistol-cli/src/protocol.rs:1-130`,
`crates/pistol-cli/src/report.rs:44-80`,
`crates/pistol-cli/src/bin/pistol.rs:118-176`,
`crates/pistol-engine/src/validate.rs:81-125`.

### WHAT I RAN

**My own worktree, `/home/tom/Projects/pistol-wt-rr`, detached at `980bbb5`**,
own `CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rr-target`, both on `/home`
(`df -h /home` → 793 G available; `/tmp` untouched). Built
`--release --locked -p pistol-cli`.

In it, one probe: an `eprintln!` at the design's own §2.1 truncation site
(`pvs.rs`, immediately after the empty-set check, immediately before
`set.promote_table_move(table_move);`) printing `ply`, `turns_from_root()`,
`state().turn()`, `used_quiet_safety_net` and `set.cells.len()` on every node
that reaches that point. Driven through the shipped line protocol.

**The live tree was never edited except to write this one file.**

**WORKTREES USED AND LEFT IN PLACE.** I created and LEFT
`/home/tom/Projects/pistol-wt-rr` (detached at `980bbb5`, target dir
`/home/tom/Projects/pistol-wt-rr-target`). I did **not** touch, read from, or
remove `/home/tom/Projects/pistol-wt-rvd` (the previous reviewer's, detached at
`13720fc`) — it is untouched and still present. **No worktree was removed.**
`pistol-wt-rr` holds no `artifacts/` or `sessions/` content of its own; its only
uncommitted content is the single probe described above.

---

## CLOSURE LEDGER

| # | Sev | Finding | Status | Evidence |
|---|---|---|---|---|
| 1 | BLOCKING | §8's spread expectation MEASURED-false at the registered budget | **CLOSED BUT INTRODUCED SOMETHING** | The false "expected exactly inert — MEASURED 0 prune events" is gone (`:496-503` states the correct `nodes 50000` counts with the uncapped caveat). I re-took them independently at `980bbb5`: r=2 → **95 / 5 / 152 / 0**, r=3 → **0 / 5 / 130 / 20**, exact. But the replacement introduces **NEW BLOCKING 1** (§8's new claim 1 registers a counter its own registered instrument cannot read), **NEW MAJOR 7** (the `1 / 0 / 0 / 0` vector belongs to r=3, and §8 now names an r=2 seat), and **NEW MAJOR 5** (the anchor has no named instrument) |
| 2 | BLOCKING | The `ply > 1` mutant has no falsifier | **CLOSED BUT INTRODUCED SOMETHING** | Closed **in substance**: I verified by running that at an empty-board root the ply-1 node sits at `turns_from_root() == 1`, is a safety-net row, and is the only node where the two spellings differ — so a test that kills the mutant exists. But the row's pool figure is wrong (**NEW MINOR 8**: 18, not 19) and the assertion it states is not expressible with §4's observables (**NEW MAJOR 4**) |
| 3 | BLOCKING | §4's file list short by six; its greps blind to the class | **CLOSED** | The three greps reproduce at HEAD: 12 configs, 1 embedded TOML, **11 files / 18 hits** for `StagedParams {`. All 11 are on §4's table, including the six revision 2 missed (`quiescence.rs`, `staged_colony_family_tests.rs`, `staged_differential_gate_tests.rs`, `staged_pattern_fixture_tests.rs`, `staged_tier_t_threshold_tests.rs`, `wp18b_solver_path_tests.rs`). `validate.rs:81-92` confirmed: ten fields, no `..` |
| 4 | MAJOR | §8's corpus ratios have no receipt; the node-count claim is not what the instrument records | **NOT CLOSED — and the replacement is worse** | See **NEW BLOCKING 2**. The per-position node-count claim is correctly withdrawn (`:486-487`). But the new paragraph names `artifacts/wp15d_turn_axis_v1.txt` and then misquotes it: the incumbent is **4 776**, not the 4 825 the design states; 4 825 is from `matrix_M2.md:236`, a different run. The fourth "seat" (4 779) is `scope1`, not the selected `scope4`. The derived ratios flip sign on three of four cells |
| 5 | MAJOR | §4 changes the pinned handshake line with no ADR; names `pistol.rs` as a forced site | **CLOSED** | `pistol.rs` is dropped from the list with the reason stated (`:252-259`), and the protocol change is **DROPPED, not made** — so no ADR is owed. Verified: `pistol.rs:154-158` destructures with `..`, so it is genuinely not a forced site. The reviewer's residual point (two SPRT seats with byte-identical handshakes) is answered by pointing at the config path and digest and deferred to the prereg. Sound |
| 6 | MAJOR | §8 names no aggregating instrument and no bench config | **PARTIAL** | The configs are now named for both seats with digests (`:462-468`), and the arithmetic is fully specified in the document — per-position median of five, per-band `Σ nodes / Σ median time` and `Σ median time`, IQR gate per position at 10 % (`:474-479`). That half is closed. **No producer is named for the aggregation**, and `docs/process.md`'s rule is about the artefact that PRODUCES a registered number, not the formula. Registering the formula makes the derivation checkable from the raw record lines, which is most of the value; the gap is narrow but real |
| 7 | MAJOR | §5's flatness anchor measured without §6.3's store rule | **CLOSED** | `:328-338` says exactly that, adds the K = 32 → 490 decay point that makes the curve non-flat across the whole grid, and pre-registers the monotone outcome (grid not extended after the fact, D-374). A clean fix |
| 8 | MAJOR | `lift` folds a mate-terminated search into "did not reach depth 3" | **CLOSED BUT INTRODUCED SOMETHING** | The exclusion is added (`:317-322`). But it is a **per-seat** exclusion, which makes the population K-dependent, and `lift` is now defined twice incompatibly (a COUNT at `:283`, a ratio with a denominator at `:321-322`). **NEW BLOCKING 3** |
| 9 | MINOR | §5's VOID threshold cannot fire, in a document whose §8 refuses such thresholds | **NOT CLOSED — answered with a false claim** | The threshold is retained and justified at `:311-315` by "§6.3's store rule WITHHOLDS records and therefore slows the capped search". Refuted by the code: `search.rs:323` `abortable = depth_turns > 1 \|\| fallback.is_some()`, `search.rs:232-238` `fallback = None` for `Stop::Nodes`, and `pvs.rs:730` asserts `"a non-abortable iteration completes"`. **NEW MAJOR 6** |
| 10 | MINOR | `gate_staged_snk_v0.toml` fixes K = 8 while §9 says no K is chosen | **CLOSED** | `:248` now states the 8 is not §5's calibrated K and does not become it, with the reason (a determinism seat needs the mechanism exercised, not the selected value). Introduced: the supporting figure is mis-transcribed — **NEW MINOR 9** |
| 11 | MINOR | "the bound lives here" names no bound | **CLOSED BUT INTRODUCED SOMETHING** | §3 now names a check (`:196-198`). The check it names can never fire on a 64-bit target — the class §3's very next sentence invokes D-424 to delete. **NEW MINOR 10** |
| 12 | MINOR | §2.1 point 3 is true of the SITE, not the SCOPE; the inherited 92.4 % is a different quantity | **CLOSED** | `:107-111` now says "**It says nothing about the SCOPE** … `turns_from_root() > 0` is measured nowhere in those artifacts." The inherited percentage is gone: `grep -n "92.4\|1 593 643\|1 724 042"` on the design returns nothing. Fully closed |

**Tally: 2 CLOSED of 3 BLOCKING (one in substance only), 3 CLOSED / 1 PARTIAL /
1 NOT CLOSED of 5 MAJOR, 3 CLOSED / 1 NOT CLOSED of 4 MINOR — and five of the
twelve rows introduced something new.**

---

## VERDICT: **FAIL**

Three new BLOCKING findings. **None of them is a correctness finding.** The
mechanism is unchanged from revision 2 and I did not attack it afresh — §2.2's
guard, §2.3's ordering, §2.5's tie-break and §6.3's store rule were attacked at
length by the revision-2 reviewer and held, and revision 3 does not touch them.
Everything below is a REGISTRATION or IMPLEMENTABILITY finding.

The shape of the failure is the one this work package keeps producing, and the
revision-2 REVIEW named it: **a claim that reads correctly in isolation and is
false in the unit, at the budget, or through the instrument the engine actually
works in.** Revision 3 fixed three instances of that shape and committed four
more inside the paragraphs written to fix them.

- **NEW BLOCKING 1** — §8's replacement for the inertness claim registers a
  per-position `safety_net_capped_rows` report. `tools/bench_block.sh`, the
  instrument §8 names, reads only `info totals`, and `StageCounters` is by design
  not on any protocol line. The registered number cannot be produced by the
  registered instrument.
- **NEW BLOCKING 2** — §8's corpus paragraph, written to give MAJOR 4's ratios a
  provenance, names an artifact and misquotes it. The incumbent it divides by is
  from a different run; one of its four "seats" is a different scope; three of
  four derived ratios come out on the wrong side of 1.000 and contradict §8's own
  registered hotspot direction.
- **NEW BLOCKING 3** — §5's mate-terminated exclusion, added to close MAJOR 8,
  makes `lift`'s population K-dependent and leaves `lift` defined twice
  incompatibly. The selection rule that decides K cannot be applied as written,
  and the two readings select different K. D-395, the precedent the design cites
  for the exclusion, did the opposite.

This design has had its one fix round. Per D-423's and D-476's own precedent an
exhausted cap with a returned FAIL is a STOP.

---

## NEW FINDINGS

### NEW BLOCKING 1 — §8 registers a per-position counter that the instrument §8 registers cannot read; and the two horns of the alternative are §7 row 1 and MAJOR 6. **Registration / implementability finding, not correctness.**

**Claim attacked, verbatim (`wp15d_design.md:504-507`):**

> "1. **The cap FIRES on this fixture at this budget.** No inertness is expected or
>    registered. The ON seat REPORTS `safety_net_capped_rows` per position — §4's
>    counter — and that count is the recorded number, taken on the ON seat's own
>    tree rather than inferred from the OFF seat's."

**THE ATTACK.** §8's instrument is named two paragraphs above
(`wp15d_design.md:469-471`): "`tools/bench_block.sh` at its committed revision".
That script drives the shipped binary over the line protocol and parses exactly
one line per rep:

```
$ sed -n '247,266p' tools/bench_block.sh
		printf 'newgame\nposition %s\ngo %s\nquit\n' "$entry" "$BUDGET" |
		...
		totals="$(sed -n 's/^info totals //p' "$WORK/out")"
		printf 'bench_block: record entry %s stones %s rep %s %s\n' \
```

And `StageCounters` is, by explicit design, not on that line:

```
$ sed -n '5,12p' crates/pistol-search/src/info.rs
/// The node protocol's stage-share counters (docs/decisions.md U2-M item 2).
...
/// All zero under `CandidatePolicy::Radius`, where the staged dispatch never
/// runs. **The line protocol does not carry these** — `report.rs` renders an
/// explicit field list, so no protocol output changes; the rates are read
/// through a committed harness in the `pistol-search` test tree that calls
/// `Searcher::search` directly, ...
```

**REPRODUCED** — the field list holds no stage field:

```
$ /usr/bin/grep -n "stages\|StageCounters\|safety_net\|batched" crates/pistol-cli/src/report.rs
(no output)
$ /usr/bin/grep -n "fn totals_line" -A 30 crates/pistol-cli/src/report.rs | /usr/bin/grep -n "depth_turns\|nodes\|hashfull"
71:         "{INFO_PREFIX}{marker} depth_turns {} seldepth {} nodes {}{solver_field} {NPS_FIELD} {} \
72:          {TIME_FIELD} {} hashfull {} score {} pv",
```

and the engine's own output confirms it, at `980bbb5`, release:

```
$ printf 'newgame\ngo depth_turns 2\nquit\n' | pistol --config configs/instrument_staged_v0.toml
info totals depth_turns 2 seldepth 2 nodes 923 nps 482503 time 1 hashfull 0 score cp -76 pv 0,0 -3,1/-2,0
```

**THE TRAP IS CLOSED ON BOTH SIDES, AND THAT IS WHY IT IS BLOCKING.** There are
only two ways to make §8's registered number obtainable, and the design forecloses
each of them in another section:

1. **Put the counter on the protocol.** Then `report.rs`'s field list changes, and
   §7 row 1 — "`the_gate_off_search_is_byte_identical_to_the_pre_change_engine`
   … reproduces a sha-pinned expectation fixture, cross-REVISION"
   (`wp15d_design.md:421`) — goes red on a gate-OFF seat, because the transcript
   gains a field with `safety_net_top_k = 0`. The revision-2 REVIEW cleared row 1
   on precisely the opposite premise: "**Adding counters to `StageCounters` cannot
   move any pinned output.** … §7's row 1 … is therefore not endangered by §4's
   three new counters." Both cannot be true.
2. **Use a different instrument.** Then §8 has an unnamed producer for a
   registered number, which is `docs/process.md`'s instrument rule and is MAJOR 6
   reopened wider than it was:

   > "THE INSTRUMENT HAS A GOVERNING REVISION TOO. An artefact that produces a
   > registered number … is named in the pre-registration WITH ITS REVISION"
   > — `docs/process.md:13-16`

**Why BLOCKING and not MAJOR.** This is the ENTIRE replacement for the claim that
failed revision 2. Revision 2 registered "expected exactly inert"; revision 3
withdraws that and registers, in its place, a per-position count of
`safety_net_capped_rows` as "the recorded number". An implementer cannot record
it. The section that is the fix for the round's first BLOCKING is the section that
cannot be executed, and the choice between the two horns is a decision the design
leaves for the implementer to invent — which is exactly what the dispatch asked
to be treated as a finding.

**What closes it.** Name the harness — a `crates/pistol-search/tests/` instrument
calling `Searcher::search` directly, the shape `info.rs:10-12` already points at
and the shape §5's `wp15d_calibration.rs` already takes — with its governing
revision, and say that `tools/bench_block.sh` supplies the timing legs while that
harness supplies the counter leg. That also keeps §7 row 1 intact.

---

### NEW BLOCKING 2 — §8's corpus paragraph names `wp15d_turn_axis_v1.txt` and then quotes an incumbent that artifact does not contain, a seat from a different scope, and four ratios whose sign is wrong on three cells. **Registration finding, not correctness.**

**Claim attacked, verbatim (`wp15d_design.md:481-489`):**

> "What `artifacts/wp15d_turn_axis_v1.txt`'s `S2/CORPUS` lines actually record, at
> `Stop::Nodes(50_000)`, `quiet_radius 2`, is **Σ nodes = 1 104 026 identical on
> every seat**, **per-position completed depths identical seat for seat** (the
> `depths=` field, 24 values), and **Σ wall 4 828 / 4 795 / 4 790 / 4 779 ms**
> across the four seats against the incumbent's 4 825. Per-position node counts are
> NOT recorded and no claim rests on them. The derived wall ratios are 1.001,
> 0.993, 0.993 and 0.991 — computed here, from those sums, and marked as derived."

**THE ATTACK.** This paragraph exists to give MAJOR 4's ratios a provenance. It
names the artifact, and the artifact refutes three of the sentence's parts.

**REPRODUCED. (a) The incumbent is 4 776, and 4 825 is not in that artifact.**

```
$ /usr/bin/grep -n "S2/CORPUS" artifacts/wp15d_turn_axis_v1.txt | sed 's/depths=.*//'
202:test zz_s2_corpus ... S2/CORPUS/nodes50000/r2/scope0/K0  sum_nodes=1104026 sum_ms=4776 sn_rows=66 capped=0
203:S2/CORPUS/nodes50000/r2/scope4/K4   sum_nodes=1104026 sum_ms=4828 sn_rows=17 capped=9
204:S2/CORPUS/nodes50000/r2/scope4/K8   sum_nodes=1104026 sum_ms=4795 sn_rows=18 capped=10
205:S2/CORPUS/nodes50000/r2/scope4/K16  sum_nodes=1104026 sum_ms=4790 sn_rows=21 capped=13
206:S2/CORPUS/nodes50000/r2/scope1/K8   sum_nodes=1104026 sum_ms=4779 sn_rows=17 capped=17
```

`scope0/K0` is the incumbent (the harness's own first tuple,
`for (cap, scope) in [(0usize, 0usize), (4, 4), (8, 4), (16, 4), (8, 1)]`,
`wp15d_turn_axis_v1.txt:368`), and it reads **4 776**. The 4 825 the design
divides by comes from a different run entirely:

```
$ /usr/bin/grep -rn "4825\|4 825" artifacts/ docs/ | /usr/bin/grep -v llr_
docs/experiments/matrix_M2_REDTEAM.md:899:  4 825 / 4 836 / 4 801 ms and re-summing the node totals to 1 104 026 ...
docs/experiments/matrix_M2.md:236:| 0 | 1 104 026 | `3 2 3 3 3 3 2 3 3 2 3 3 4 1 2 3 1 3 3 2 3 2 2 3` | 4 825 |
docs/experiments/wp15d_design.md:486:across the four seats against the incumbent's 4 825. ...
```

**So §8 divides candidate wall sums from `wp15d_turn_axis_v1.txt` by an incumbent
wall sum from `matrix_M2.md`, in a sentence that says all of it is what
`wp15d_turn_axis_v1.txt` records.** A cross-run ratio presented as one artifact's
record. This is MAJOR 4's own defect — "the two ratios have no provenance" —
recommitted with a provenance that is false rather than absent, which is the
harder version to catch.

**(b) The fourth "seat" is a different scope.** The artifact's own legend:

```
$ sed -n '17,18p' artifacts/wp15d_turn_axis_v1.txt
# and three ply counters. SCOPES: 0 every ply | 1 every ply but 0 | 2 non-PV only |
# 3 ply 0 only | 4 every ply but 0 and 1 (the root turn exempt).
```

Scope 4 is `except-root-turn`, the selected scope. **Scope 1 is `except-ply0` —
a row that caps ply 1, i.e. INSIDE the played turn**, and is one of the rows the
matrix struck for turn-incoherence (D-477). The design folds its 4 779 ms in as
the fourth of "the four seats" of its own scope. There are three scope-4 seats in
that artifact, not four.

**(c) Three of four derived ratios have the wrong sign.** Against the artifact's
own incumbent, every capped seat is SLOWER:

| seat | Σ ms | ratio vs 4 776 (correct) | design's ratio vs 4 825 |
|---|---|---|---|
| scope4/K4 | 4 828 | **1.0109** | 1.001 |
| scope4/K8 | 4 795 | **1.0040** | 0.993 |
| scope4/K16 | 4 790 | **1.0029** | 0.993 |
| scope1/K8 (wrong scope) | 4 779 | **1.0006** | 0.991 |

**And the design's own numbers contradict the design's own registered hotspot.**
§8 opens (`wp15d_design.md:452-457`): "capping the branching factor below the
root turn deepens the tree at a fixed node budget … **This is a cost channel, not
a gain channel**, and it is registered as one." Three of the four ratios the
design derives say the capped seat is FASTER than the incumbent. Corrected, all
four are ≥ 1.000 and the hotspot's direction is consistent. The arithmetic error
is what made the paragraph disagree with its own section.

**Does the registered verdict move?** No — at the registered bracket
(`ttd ≤ 1.10`, ABORT `> 1.25`) 1.0109 passes as comfortably as 1.001 does, and
"the corpus bench is a NO-REGRESSION check" survives. **That is why this is a
registration finding and not a correctness one**, and it is why I record honestly
that D-424's test ("does the disputed claim change what anyone may conclude") is
not met by the ratios alone.

**Why BLOCKING anyway.** Two reasons, and they are about the citation rather than
the conclusion. First, the sentence's grammatical subject is *what the named
artifact records*, and three of its clauses are not what it records — a reader
checking the receipt is told a falsehood about the receipt, which is worse than
an unreceipted number because it defeats the check. Second, the revision-2
reviewer classed the weaker version of this MAJOR and wrote the reason it is not
overrulable: "a registered bracket's stated expectation constrains the reading of
the run." The expectation is now stated with three wrong components in the
paragraph written to fix it, and the class has now recurred four times in this
work package (round 1 MINOR 11, round 3 MINOR 7, revision 2 MAJOR 4, here) — which
is D-424's own escalation ground, "exactly one MAJOR in each of three consecutive
governing reviews, every one of them inside the paragraph written to fix the
previous one."

**What closes it.** Divide by 4 776; drop the scope-1 seat or label it as the
different row it is; state the three scope-4 ratios as 1.011 / 1.004 / 1.003; and
note that they agree with §8's cost-channel hotspot.

---

### NEW BLOCKING 3 — §5's mate-terminated exclusion makes `lift`'s population K-dependent, `lift` is defined twice incompatibly, and D-395 — the precedent cited — did the opposite. **Registration finding, not correctness.**

**Claims attacked, verbatim.** §5 defines the channel once at
`wp15d_design.md:282-283`:

> "**THE CHANNEL, AND ITS DIRECTION.** `lift(K)` = the number of the 2 000 openings
> whose **completed** `depth_turns` is ≥ 3. **It is a COUNT** and **larger is
> better**."

and again, incompatibly, at `wp15d_design.md:317-322`:

> "Such openings are **EXCLUDED from both numerator and denominator**, counted, and
> named in the artifact, which is the same treatment D-395 gave the two positions
> whose candidate found a forced mate early. The `lift` denominator is therefore
> `2 000 − (mate-terminated)` and the artifact prints it."

**THE ATTACK, part 1 — a COUNT has no denominator, and the two readings select
different K.** The selection rule (`wp15d_design.md:290-291`) is:

> "**K is the LARGEST value on the grid whose `lift(K)` is at least 90 % of
> `max_K lift(K)`. Ties break toward the larger K.**"

Applied to which quantity?

- **Reading A (count, per `:283`):** `lift(K) = |{openings : depth ≥ 3, not
  mate-terminated}|`. Excluded openings simply leave the count. More mates ⇒ lower
  `lift`.
- **Reading B (ratio, per `:321-322`):** `lift(K) = |{depth ≥ 3, not mate-term}| /
  (2000 − mate_term(K))`. More mates ⇒ smaller denominator ⇒ HIGHER `lift` for the
  same numerator.

The two move in opposite directions in `mate_term(K)`, and the 90 % rule applied
to them selects different K in general. **An implementer must invent the
definition, and the invention decides K** — the value that governs the bench
candidate seat (§8) and the SPRT seat (§9). D-374 forbids settling it after the
sweep.

**THE ATTACK, part 2 — the exclusion set is per-seat, so the seats are not
comparable, which is the one thing the 90 % rule requires.** `mate_term` is a
function of K: `2 000 − (mate-terminated)` is written with no seat subscript, but
mate-termination is produced by the search, and the search is what K changes. The
design states the mechanism itself in the finding it is closing — the
revision-2 REVIEW's MAJOR 8:

> "the cap removes the OPPONENT's candidate cells at every node below the root
> turn … so a 'mate' found inside a truncated subtree is a mate against a defender
> whose replies were pruned."

That mechanism runs with narrowing: **smaller K prunes more defenders and
manufactures more mates**. Under Reading B the low grid then gets the smallest
denominators and the most flattering `lift`; `max_K lift(K)` sits at a small K,
the 90 % bar rises, and "the largest K within 90 % of the best" selects a SMALLER
K — **which is precisely the degeneracy §5's own "Why that rule and not maximise
lift" paragraph exists to prevent** ("completed depth is monotonically improved by
narrowing, so a rule that maximises it selects the grid minimum by construction —
the mirror image of the defect that failed WP-1.5c's re-review",
`wp15d_design.md:293-296`). Under Reading A the populations differ per seat and
the counts are not comparable at all.

The dispatch asked specifically whether a LARGER K could shrink the denominator
and flatter itself. **I record the honest answer: the mechanism the design itself
cites runs the other way, and the design registers NO measurement of the direction
and no control on it.** Either direction moves K, and the design's silence is the
finding — a denominator that moves with the treatment is not a denominator.

**THE ATTACK, part 3 — D-395, the cited precedent, used a seat-INVARIANT
population.** REPRODUCED:

```
$ /usr/bin/grep -n "^D-395:" docs/decisions.md | cut -c1400-2100
... **THE SAMPLE**: 19 of the 24 fixture positions reach `depth_turns = 3` on
BOTH sides at this budget and are the ones compared; 5 excluded and named rather
than silently dropped — 2 positions are already-decided/near-terminal on both
sides ...; 1 position has the candidate reach only `depth_turns = 2` ...; 2
positions have the candidate find a forced mate and stop at `depth_turns = 1` ...
```

**"on BOTH sides"** is the whole of it. D-395 compared a single common set of 19
positions on both seats. It did not compute a per-seat denominator; it removed
from BOTH seats every position that failed on EITHER. §5 cites D-395 for a
per-seat exclusion, which is the opposite of what D-395 did, and the precedent
therefore refutes the clause it is cited to support.

**Why BLOCKING and not MAJOR.** §5 is the section that DECIDES K, and D-374 pins
the rule before the sweep so it cannot be repaired afterward. A rule whose channel
has two incompatible definitions and whose population moves with the treatment
cannot be applied as written, and both defects were introduced by this fix round.
The `wp19_design_REVIEW_rev2.md` BLOCKING N1(c) question §5 declares itself the
checklist for — "does the selection rule select a grid extreme by construction?" —
is now unanswerable, because the answer depends on which of the two `lift`s and on
an uncontrolled per-seat denominator.

**What closes it, and it is one clause.** Fix the excluded set across seats:
`lift(K)` is a COUNT over the FIXED population `2 000 − |{openings mate-terminated
on ANY seat, incumbent control included}|`, that set named and printed once, the
same at every K — D-395's own shape. Then delete the "numerator and denominator"
sentence, which a count does not have.

---

### NEW MAJOR 4 — §7 row 5's stated assertion is not expressible with §4's observables. **Implementability finding, not correctness.**

**Claim attacked, verbatim (`wp15d_design.md:425`):**

> "**The test asserts the emitted width at that node is K**, and it is the only
> case in which the two spellings differ at all"

**THE ATTACK.** §4 gives §7 four new observables, and `info.rs` states their
grain:

```
$ sed -n '14,18p' crates/pistol-search/src/info.rs
/// Every field is a WHOLE-SEARCH total, like [`SearchInfo::nodes`]: written
/// from the same point, on every [`SearchInfo`] construction path including
/// both salvage ones, so a counter never silently reads zero on a path that
/// visited real nodes.
```

`safety_net_capped_rows`, `safety_net_emitted_cells`, `safety_net_pool_cells` are
whole-search totals. **There is no per-node observable**, and there cannot be one
at `staged.rs` either, because §2.1's whole argument is that the truncation lives
in `pvs.rs` and `staged_candidates` never sees `turns_from_root`. So "the emitted
width **at that node**" is not a quantity any registered observable exposes.

**REPRODUCED — and the mutant IS still killable, which is why this is MAJOR and
not BLOCKING.** My probe at the design's own site, empty-board root,
`configs/instrument_staged_v0.toml`, `depth_turns 2`, at `980bbb5`:

```
$ printf 'newgame\ngo depth_turns 2\nquit\n' | pistol --config configs/instrument_staged_v0.toml 2>&1 >/dev/null \
    | sed -n 's/^SNPROBE //p' | LC_ALL=C sort | uniq -c
      2 ply=0 tfr=0 turn=1 sn=true pool=1
      1 ply=1 tfr=1 turn=2 sn=true pool=18
      6 ply=2 tfr=1 turn=2 sn=true pool=22
      6 ply=2 tfr=1 turn=2 sn=true pool=26
      6 ply=2 tfr=1 turn=2 sn=true pool=27
```

At K = 8 the shipped guard (`tfr > 0 && pool > 8`) fires on **19** rows; the
`ply > 1` mutant fires on **18** (it loses the single ply-1 row). So
`safety_net_capped_rows` reads 19 against 18 and the aggregate DOES discriminate.
A falsifier exists — **BLOCKING 2 is genuinely closed in substance** — but it is
not the one the design writes down, and the derivation from an aggregate to
"the width at that node" is a decision the implementer has to invent.

**The same shape is on rows 2, 3 and 4** ("emitted width == K at a node one turn
from the root"; "the boundary as a set difference"; "the emitted set is identical
gate-on and gate-off at every node of the root turn"). Those rows are unchanged
from revision 2 and the revision-2 reviewer did not raise them, so I record them
as PRE-EXISTING and outside this scoped round; row 5 is new in revision 3 and is
this round's.

**What closes it.** State row 5's assertion in the observable it will actually
use: at an empty-board root, `depth_turns 2`, `safety_net_top_k = 8`,
`safety_net_capped_rows == 19` and the `ply > 1` spelling gives 18.

---

### NEW MAJOR 5 — §8's spread anchor is imported from the revision-2 reviewer's throwaway probe with no instrument, no artifact, no digest, and no K; and one of its two radii rests on a config that exists nowhere in the tree. **Registration finding, not correctness.**

**Claim attacked, verbatim (`wp15d_design.md:496-501`):**

> "At the budget §8 actually registers it is false: MEASURED on the uncapped tree
> at `nodes 50000`, the guard's predicate (`turns_from_root() > 0 && pool > K`) is
> satisfied **95 / 5 / 152 / 0** times at 11 / 21 / 51 / 99 stones at
> `quiet_radius 2`, and **0 / 5 / 130 / 20** at `quiet_radius 3`."

**FIRST, THE GOOD NEWS, BECAUSE IT IS THE LARGER PART.** The dispatch asked
whether quoting a reviewer's number as the design's own anchor is sound and
whether the caveat is stated correctly. **The numbers are right and the caveat is
right.** I re-took them independently, in my own worktree at `980bbb5`, with my
own probe, and they reproduce exactly at both radii:

```
$ bash spread.sh          # probe at the §2.1 site; predicate tfr>0 && pool>K
cfg=instrument_staged_v0.toml K=8  p1 sn_rows=235  PRUNE_EVENTS=95
cfg=instrument_staged_v0.toml K=8  p2 sn_rows=265  PRUNE_EVENTS=5
cfg=instrument_staged_v0.toml K=8  p3 sn_rows=772  PRUNE_EVENTS=152
cfg=instrument_staged_v0.toml K=8  p4 sn_rows=1196 PRUNE_EVENTS=0
cfg=instr_r3.toml             K=8  p1 sn_rows=303  PRUNE_EVENTS=0
cfg=instr_r3.toml             K=8  p2 sn_rows=573  PRUNE_EVENTS=5
cfg=instr_r3.toml             K=8  p3 sn_rows=1493 PRUNE_EVENTS=130
cfg=instr_r3.toml             K=8  p4 sn_rows=2655 PRUNE_EVENTS=20
```

The "MEASURED on the uncapped tree" caveat is **correctly stated** — the cap is
not implemented, the counts are of the predicate's satisfaction on the incumbent's
tree, and §8 says so. And §8's claim 1 is right that the recorded number must come
from the ON seat's own tree, not from this one. **On the substance, revision 3 got
this right.**

**THE ATTACK is about registration, and it has three parts.**

**(a) No instrument, no artifact, no digest.** These cells were produced by an
uncommitted `eprintln!` in `/home/tom/Projects/pistol-wt-rvd`, described in the
revision-2 REVIEW as one of "the two throwaway probes". No artifact carries them:

```
$ /usr/bin/grep -c "wp15d" artifacts/wp15d_worktree_export_receipt_v2.txt   # export head bc003c9, predates rev 2's review
```

The receipt's head is `bc003c9`, before the revision-2 REVIEW existed, so no
digest covers the probe or its output. `docs/process.md:13-16` binds a registered
number to a named artefact WITH ITS REVISION. §8 names none for these cells. My
reproduction is evidence they are TRUE; it is not a substitute for the design
naming a producer, and a second reviewer at a later revision has nothing to
re-run.

**(b) K is not stated, and the predicate is K-dependent.** `pool > K` cannot be
evaluated without K; §5 says explicitly that K is not chosen. **Mitigating, and
measured rather than assumed:** the counts are invariant over K ∈ {4, 8, 16} —
95/5/152/0 at all three at r = 2, per the run above and its K=4 and K=16 rows —
because the safety-net pools on this fixture are far above 16. So the omission
does not make the quoted cells wrong. **But §5's grid runs to 64**, and invariance
there is not established by anything in the document or by me.

**(c) The r = 3 half is not reproducible from the repository.** The revision-2
REVIEW records that its radius-3 seat was "`instrument_staged_v0.toml` with
`quiet_radius = 3`, written to the scratchpad, never to a config document". No
such config is committed:

```
$ /usr/bin/grep -l "quiet_radius = 3" configs/*.toml
(no output)
```

I reproduced it only by writing the same file to my own scratchpad. A registered
MEASURED cell whose config does not exist in the tree cannot be re-taken by anyone
who does not first re-invent the config.

**What closes it.** Name the harness and its revision, state K (or state the
measured K-invariance over the grid), and either commit the r = 3 seat or drop the
r = 3 cells and keep the r = 2 ones, which are the ones §8's named seats use.

---

### NEW MAJOR 6 — §5's answer to MINOR 9 asserts a mechanism the code forbids: under `Stop::Nodes` no amount of slowing can produce completed depth 0. **Registration finding, not correctness.**

**Claim attacked, verbatim (`wp15d_design.md:308-315`):**

> "**That threshold is registered even though the incumbent measures ZERO
> openings at completed depth 0, and the reason is specific** (REVIEW-design
> MINOR 9, which is right that §8 refuses thresholds that cannot fire): §6.3's
> store rule WITHHOLDS records and therefore slows the capped search, so the zero
> measured without it does not transfer to the seats this calibration runs. **The
> threshold can fire under exactly the change being measured**"

**THE ATTACK.** MINOR 9's mechanism was never about speed. It was structural:
depth 1 cannot be aborted under a reproducible stop, so `depth_turns = 0` is
unreachable no matter how slow the search is. Revision 3 answers a different
argument than the one made, and the answer is false.

**REPRODUCED, on the shipped code:**

```
$ sed -n '232,238p' crates/pistol-search/src/search.rs
        let fallback = match stop {
            Stop::Deadline(_) => Some((
                fallback_turn(state, self.params.candidate_policy),
                self.position.value(),
            )),
            Stop::DepthTurns(_) | Stop::Nodes(_) => None,
        };
$ sed -n '321,326p' crates/pistol-search/src/search.rs
            // Every iteration is abortable once a fallback answer is secured;
            // under a reproducible stop the first one still is not (D-74).
            let abortable = depth_turns > 1 || fallback.is_some();
            let Some(score) = run.iterate(depth_plies, abortable) else {
                break;
            };
```

§5's instrument is registered at `Stop::Nodes(50_000)` (`wp15d_design.md:271`).
Under `Stop::Nodes`, `fallback` is `None`, so at `depth_turns == 1` the guard is
`1 > 1 || false` = **false**: the first iteration is not abortable. The stop check
then returns early:

```
$ sed -n '606,610p' crates/pistol-search/src/pvs.rs
        if !self.abortable {
```

and the tree's own committed test pins the consequence by name:

```
$ sed -n '727,731p' crates/pistol-search/src/pvs.rs
        // Iteration 1 is run non-abortable, so it completes over the expired
        ...
        assert!(score.is_some(), "a non-abortable iteration completes");
```

So `iterate` returns `Some` at depth 1, `outcome` is set with
`depth_turns = 1`, and `completed_depth >= 1` **always**. `§6.3`'s store rule
withholds transposition records; withholding records cannot abort an iteration
that is not abortable. **The threshold still cannot fire, and the reason given
that it can is refuted by the code and by D-74's own text** ("the FIRST iteration
is not interruptible").

**This also answers the dispatch's fourth question directly: §3's "no upper bound"
and §5's retained VOID threshold are NOT different — they are governed by the same
test, and §5 fails it.** Both paragraphs apply the same rule, correctly stated:
a clause that cannot fire does no work and is deleted (D-424). §3 applies it and
deletes (`wp15d_design.md:199-203`). §5 claims an exemption from it on a factual
premise, and the premise is false. The symmetry the dispatch suspected is real;
the design's asymmetry is not.

**Severity.** MINOR 9 as filed was a MINOR — dead prose. Revision 3 escalated it
by answering with a false statement about the engine's control flow, and a false
mechanism claim in a registered section is the class D-474 exists for ("any
architect-dispatch claim about a code mechanism is UNVERIFIED until the executing
session quotes it at `file:line`"). The design quotes no site for this claim, and
the sites that exist refute it.

**What closes it.** Delete the clause (D-424's own remedy, and §3's), or keep it
with the honest reason: it is a belt over the structural guarantee at
`search.rs:323` and D-74, named as such.

---

### NEW MAJOR 7 — §8's and §9's `1 / 0 / 0 / 0` depth vector is the r = 3 reading, quoted under the r = 2 seat §8 newly names. **Registration finding, not correctness.**

**Claims attacked, verbatim (`wp15d_design.md:509-512` and `:538-540`):**

> "That is measured at `movetime 500`, the budget at which the debt is DEFINED
> (D-95's own units), where completed depth is **1 / 0 / 0 / 0** with the cap
> armed and without it."

> "**MEASURED untouched by this one in the channel the debt is defined in** —
> completed depth at `movetime 500` on `spread_v1` is **1 / 0 / 0 / 0** with the
> cap armed and without it."

**THE ATTACK.** Revision 2's BLOCKING 1 said: "§8 names no config, so a reader
cannot tell which radius the claim is about — and it is false at one of the two."
Revision 3 fixed the naming — §8 now names `configs/instrument_staged_v0.toml`
**as committed**, `quiet_radius = 2` (`wp15d_design.md:462-464`) — and then
quotes the OTHER radius's vector.

**REPRODUCED from the artifact the design cites.** At r = 2, where §8's seats live:

```
$ sed -n '33,36p;105,108p' artifacts/wp15d_turn_axis_v1.txt
S1/r2/nocap/K0            p00 stones= 11 depth=1 ...
S1/r2/nocap/K0            p01 stones= 21 depth=1 ...      <- ONE, not zero
S1/r2/nocap/K0            p02 stones= 51 depth=0 ...
S1/r2/nocap/K0            p03 stones= 99 depth=0 ...
S1/r2/except-root-turn/K8 p00 stones= 11 depth=1 ... capped=2715
S1/r2/except-root-turn/K8 p01 stones= 21 depth=1 ... capped=150
S1/r2/except-root-turn/K8 p02 stones= 51 depth=0 ... capped=0
S1/r2/except-root-turn/K8 p03 stones= 99 depth=0 ... capped=0
```

At the named seat's radius the vector is **1 / 1 / 0 / 0**, both with the cap
armed and without. `1 / 0 / 0 / 0` is the r = 3 pair
(`wp15d_turn_axis_v1.txt:117-120` and `:189-192`), inherited from D-478, which
states its cell as `quiet_radius 3`.

**The conclusion survives — "untouched" is true at BOTH radii** (1/1/0/0 =
1/1/0/0 at r = 2; 1/0/0/0 = 1/0/0/0 at r = 3), which is why this is MAJOR and not
BLOCKING under D-424's test. What the wrong vector conceals is that D-478's
premise — "the incumbent completes depth 0 at 21 / 51 / 99 stones" — is
radius-dependent and does not hold at 21 stones at the radius this design's bench
actually runs.

**What closes it.** Quote `1 / 1 / 0 / 0` and say it is r = 2, or keep
`1 / 0 / 0 / 0` and label it r = 3 with D-478's citation. One clause either way.

---

### NEW MINOR 8 — §7 row 5's pool figure is 18, not 19.

**Claim attacked, verbatim (`wp15d_design.md:425`):**

> "Its pool is the radius-`r` ball around the origin (**19 cells at r = 2**) and
> the shipped guard truncates it to K"

**THE ATTACK.** `within_radius` filters the ball through
`board.is_legal_placement`, and the origin is occupied by the ply-0 stone:

```
$ sed -n '55,60p' crates/pistol-search/src/candidates.rs
    cells
        .into_iter()
        .filter(|&cell| board.is_legal_placement(cell))
        .collect()
```

The radius-2 hex ball is 19 cells (1 + 6 + 12); one is the stone. **REPRODUCED**,
from the probe output above: `ply=1 tfr=1 turn=2 sn=true pool=18`.

Everything else in that row is correct and I verified each part: the ply-1 node
IS at `turns_from_root() == 1` (`turn=2`, `root_turn=1`); it IS a safety-net row
(`sn=true`); and it IS the only node where the two spellings differ — every ply-2
node is also at `tfr=1` and is capped by both. An implementer who writes the
self-documenting assertion the sentence invites (`assert_eq!(pool, 19)`) ships a
red test.

---

### NEW MINOR 9 — §4's "151 of 153" is a fraction assembled from two different budgets; measured, it is 151 of 151 and 153 of 153.

**Claim attacked, verbatim (`wp15d_design.md:248`):**

> "MEASURED, **151 of 153** safety-net rows on `tactical_staged_v0.txt` under
> `gate_staged_v0.toml` carry a pool of 33–52, so 8 binds on essentially all of
> them"

**THE ATTACK.** The revision-2 REVIEW reported two separate totals at
`tools/determinism.sh`'s two budgets — 151 rows at `depth_turns 4`, 153 at
`nodes 200000` — and "151 of 151 rows have pool > 8". Revision 3 pairs the
numerator from one budget with the denominator from the other, producing a
fraction that implies two rows fail when none do.

**REPRODUCED**, my own probe, 20 positions of `tactical_staged_v0.txt` under
`configs/gate_staged_v0.toml` (`quiet_radius = 1`), at both of
`tools/determinism.sh`'s budgets:

```
budget=depth_turns 4  SAFETY_NET_rows=151 rows_pool_gt_8=151 pool_range=33..52
budget=nodes 200000   SAFETY_NET_rows=153 rows_pool_gt_8=153 pool_range=33..52
```

**100 % at both budgets, not 98.7 %.** The design's conclusion — "8 binds on
essentially all of them", and therefore that `gate_staged_snk_v0.toml` is a real
seat and not a vacuous one — is CORRECT and if anything understated. MINOR
because the conclusion is unaffected; recorded because it is a MEASURED cell that
is false as written, and because it is the third number in this revision
mis-transcribed from the revision-2 REVIEW (with §8's incumbent and §8's depth
vector).

---

### NEW MINOR 10 — §3's newly named check can never fire, in the paragraph that deletes another check for never firing.

**Claims attacked, verbatim (`wp15d_design.md:196-201`):**

> "`safety_net_top_k` is refused by name unless it is **representable as `usize`**,
> which is what `Vec::truncate` takes; `EngineError::config` carries the field path
> and the value. **There is deliberately NO upper bound.** A K above every pool the
> fixture produces is a no-op, so refusing it would refuse a document that spells
> the OFF behaviour a second way — **a refusal doing no work**, which is the class
> D-424 says to delete rather than refine."

**THE ATTACK.** `safety_net_top_k` is a `u64` and `usize` is 64 bits on this
target (`CLAUDE.md`'s Environment names one workstation; no 32-bit target is in
play). Every `u64` is representable as `usize`. **The named check is itself a
refusal doing no work** — the exact class the next sentence invokes D-424 to
delete. §3 deletes one no-op refusal and adds another in the same breath.

A second, smaller inconsistency: §2.2's printed guard does not use a checked
conversion at all —

```
$ sed -n '/let truncated = params.safety_net_top_k/,/^}/p' docs/experiments/wp15d_design.md
    && set.cells.len() > params.safety_net_top_k as usize;
```

— it uses a raw `as` cast, so the code the design prints would not exercise the
check the design registers.

**Not BLOCKING and not MAJOR**: MINOR 11 asked for a bound to be named or for the
document to say the change is a destructure only, and the honest answer is the
latter. The fix is one clause — say there is no bound in either direction and why
— not a new check.

---

## WHAT I CHECKED AND FOUND SOUND

**§7 row 5 CAN exist, and it kills the mutant.** This was the dispatch's
sharpest question and the answer is yes. Verified by running, not by argument.
At an empty-board root under `configs/instrument_staged_v0.toml`:

- `legal_placements` on an empty board is `vec![Coord::ORIGIN]`
  (`movegen.rs:100-104`), so the root's set is one cell and the search reaches
  ply 1 normally — MEASURED `ply=0 tfr=0 turn=1 sn=true pool=1`.
- The ply-1 node sits at `turns_from_root() == 1` — MEASURED
  `ply=1 tfr=1 turn=2`, because `stones_in_turn(1) == 1` (`rules.rs:30-36`)
  completes turn 1 at ply 0.
- It IS a safety-net row — MEASURED `sn=true`.
- It is the ONLY node the two spellings disagree about: at `depth_turns 2` there
  is exactly one `ply=1` row and every `ply=2` row is also at `tfr=1`.
- `safety_net_capped_rows` at K = 8 reads **19** shipped against **18** mutated.

**BLOCKING 3's enumeration reproduces exactly.** All three greps, at HEAD:

```
$ /usr/bin/grep -l 'kind = "staged"' configs/*.toml | wc -l
12
$ /usr/bin/grep -rln 'kind = "staged"' crates/ --include=*.rs
crates/pistol-engine/tests/common/mod.rs
$ /usr/bin/grep -rln 'StagedParams {' crates/ --include=*.rs | wc -l
11
$ /usr/bin/grep -rn 'StagedParams {' crates/ --include=*.rs | wc -l
18
```

Every one of the 11 files is on §4's table, including all six revision 2 missed.
`validate.rs:81-92` destructures ten fields with no `..`, as claimed. One
imprecision, below MINOR: of the 18 grep hits, `params.rs:59` is the struct
definition and `quiescence.rs:540` is a `-> StagedParams {` signature, so 16 hits
and 14 actual literals; the design presents 18 as the grep's own count, which it
is.

**MAJOR 5 is closed cleanly, and I checked the drop rather than the change.**
`pistol.rs:154-158` destructures with `..`, so it is genuinely not a
forced-compile site; the design's decision to leave the handshake alone means
D-356 and `U2_node_protocol.md` §U2-M item 2 are untouched and no ADR is owed.
This is the right resolution of the two the reviewer offered.

**MAJOR 7 is closed cleanly.** §5 now states that the 535/524/514 shape was
measured without the store rule, adds the K = 32 → 490 point that makes the curve
non-flat across the whole grid, and pre-registers the monotone outcome in advance
rather than leaving it to be discovered. This is the model of what a fix round
should look like.

**MINOR 12 is fully closed**, both halves. §2.1 point 3 now says explicitly that
it makes no scope claim, and the inherited 92.4 % is gone:
`/usr/bin/grep -n "92.4\|1 593 643\|1 724 042" docs/experiments/wp15d_design.md`
returns nothing.

**The mechanism is untouched by this round.** `git diff 13720fc..980bbb5` shows no
change to §2.2's guard, §2.3's ordering, §2.4, §2.5 or §6.3's store rule. The
revision-2 reviewer attacked those five ways and they held; I did not re-open
them, and I record that as deliberate scoping rather than as agreement I earned.

**The scoping diff is clean.** No engine code, config, `tools/` script or ADR line
was touched by the fix round.

### Attacks that did NOT reproduce

- **"§8's `1 / 0 / 0 / 0` claim is vacuous because the cap fired 0 times."**
  Partly, and not enough to make a finding of its own. At r = 3 the cap fires
  0 times at 21/51/99 stones (`S1/r3/except-root-turn/K8 ... capped=0`), so
  "untouched" is a tautology at three of four points. **But at r = 2 it fires
  2 715 and 150 times at 11 and 21 stones with the depth unmoved**
  (`wp15d_turn_axis_v1.txt:105-106`), which is genuinely informative. The claim
  is supported; what is wrong with it is the radius, recorded as NEW MAJOR 7
  below. **Recorded as rejected with the attempted reasoning rather than dropped.**

- **"§6.3 narrows `WPQ_seed.md` §7.2 by dropping its root/PV exemption."** Not a
  finding. §7.2 bundles a PV-node exemption with the store rule, and this design
  adopts only the store rule. But the exemption's stated purpose is "no PV node
  ever returns a value it has not proved, so no unsound `Bound::Exact` is ever
  written" — which the store rule alone achieves, since no truncated node stores
  `Exact` under any scope. The residual difference is that capped PV values differ
  from uncapped ones, which §6.3 states outright ("It does not make the capped
  search's values equal to the uncapped search's — nothing does; that is what a
  forward prune is"). Closed.

- **"The three new `StageCounters` fields break §7 row 1's byte-identical
  fixture."** REFUTED on its own terms — `report.rs` renders an explicit field
  list with no stage field, so the protocol is unchanged. It becomes live only
  under NEW BLOCKING 1's horn (1), which is why that finding is stated as a
  choice between two horns rather than as this attack.

---

## SUMMARY FOR THE OPERATOR

Revision 3 is a real fix round, not a re-argument: the revision-2 report's
BLOCKING 3 is fully closed, its BLOCKING 2 is closed in substance (I proved by
running that its falsifier can exist), its MAJOR 5 and MAJOR 7 are closed
cleanly, and its MINOR 10 and MINOR 12 are closed.
The mechanism was not touched and does not need to be.

What fails is the same thing that failed revision 2, in the paragraphs written to
fix it. Three of the four numeric cells revision 3 imports from the revision-2
REVIEW are mis-transcribed (§8's incumbent, §8's depth vector, §4's 151-of-153);
the section that replaces the round's first BLOCKING registers a counter its own
registered instrument cannot read; the clause that closes MAJOR 8 breaks the
comparability its own selection rule depends on and cites a precedent that did the
opposite; and the revision-2 report's MINOR 9 is answered with a claim about the engine's control flow
that `search.rs:323` and `pvs.rs:730` refute.

None of it is a correctness finding. All of it is cheap to fix — every one of the
nine has a one-clause or one-paragraph remedy stated with it, and the commands to
re-take every number are in this report. But the cap is exhausted, and D-423's
and D-476's precedent for that branch is explicit: **STOP, land the report,
collect; the next step is a joint call.**

**Worktrees:** I used and LEFT `/home/tom/Projects/pistol-wt-rr` (detached
`980bbb5`, target `/home/tom/Projects/pistol-wt-rr-target`). I did not touch or
remove `/home/tom/Projects/pistol-wt-rvd`. Neither was removed; neither holds
`artifacts/` or `sessions/` content of its own.
