# WP-2.0b DESIGN revision 5 — REVIEW-design (fresh context)

## Header

**Revision reviewed:** `fe0570bcf23ef8202d7b19861b9959fadce733ef` — a `git stash create`
object on top of `dev` HEAD `a56449b`. **The tree still matches at the end of this
review**: `git diff fe0570b --stat` is empty on completion, as it was on entry, and the
verification worktree I created was removed (`git worktree list` shows main only).

I did not write this document and I am not any of the four reviewers it answers.

**Read.** `docs/experiments/wp20b_design.md` (949 lines, revision 5) in full;
`wp20b_design_rev2_REVIEW.md`, `wp20b_decision_REDTEAM.md`, `wp20b_design_rev3_REVIEW.md`,
`wp20b_design_rev4_REVIEW.md`; revision 4 recovered as `git show d22b95d:docs/experiments/wp20b_design.md`;
`CLAUDE.md`; `docs/process.md` (§"Criterion and defect class", §"Cost, replication, and
the second instrument"); `docs/decisions.md` D-465, D-508, D-516, **D-517**, D-530,
D-537, D-560, D-563; `docs/experiments/wp21_DISPATCH.md` §4;
`artifacts/wp20b_cap_RECEIPT.txt` and all ten artifacts it indexes, including the two
harness scripts still resident in the producing session's scratchpad.

**Ran.**

- Re-derived every cell of §1.1's table from the exported per-entry lines (Python, sums
  and ratios).
- **Re-ran all six configurations myself** in a detached worktree at `a56449b` under
  `/home/tom/.cache/` (never `/tmp`, `CARGO_TARGET_DIR` never exported), rebuilding
  `crates/pistol-search/examples/trigger_census.rs` from source and re-deriving the two
  fixtures. The fixture derivation reproduces byte-exactly
  (`9cbf3c37469d1a4d…` for corpus, matching `artifacts/wp20b_cap_fx_corpus.txt`).
- Reproduced §9's pre-change digests under the stated extraction rule.
- Computed a distinct-column-signature statistic over the exported census rows that the
  design does not compute and that bears directly on its central inference.
- Verified ~45 source and config citations at their lines with `/usr/bin/grep` and `sed`.

**Could not check.** The 22.99 µs fold cost and the 24 000 (position, symmetry) pairs —
both are the DECISION-RED-TEAM's measurements, cited to that report, and no artifact for
either is in the receipt. The 2.14x duplication and D-560's ~119 800 ceiling are marked
ESTIMATED and inherited. I did not run `tools/ci.sh` or `tools/determinism.sh`; this is a
design review and no code has changed.

---

## VERDICT: **FALLS**

**This is the best revision of this document by a distance, and it falls on one thing
that matters more than everything it fixed.**

What is not in dispute, and I checked all of it: **§1.1's measurement is real and
reproduces bit-exactly.** Every firing count, every proof count, every `solver_nodes`
total in that table is exactly the sum of the exported per-entry lines, and when I
rebuilt the binary and re-ran all six configurations myself, all six count columns came
back **identical to the last digit**. The cuts were made and cost almost nothing. The
citation quality is the highest of any revision — I checked about forty-five sites and
found one off-by-two. Eighteen of the twenty-three carried findings are properly
discharged, several of them by deriving the property rather than by editing the sentence
the reviewer quoted. C2 and T1+T3 survive. F1, F2 and F3 survive, and F3's config claims
(18 configs carrying the key, 3 arming it, the two named files differing in exactly those
two keys) are exact.

It falls on four things.

**W1 — §1.1 measures raw firings and concludes about D-537's clock, and D-537's own text
says "not in firings".** D-537 fixes its minimum in *"WIN-PROVING FIRINGS ON DISJOINT
POSITIONS, not in firings and not in games"*. §1.1 measures the 6.3x–7.2x collapse in
**raw firings** and writes *"D-537's clock runs about six times slower"*, which the ONE
LINE FOR THE MORNING then sells as *"worth about six times as much of D-537's clock"*.
Two substitutions sit between those quantities and neither is marked. I measured one of
them on the design's own exported artifacts: the **distinct column-signature ratio is
1.96x (trigger-rich) and 2.17x (corpus)** against the raw ratio of 7.17x and 6.35x —
because at cap 2048 only **17 %** and **6 %** of firings carry a distinct signature, against
61 % and 19 % at cap 16384. The second substitution runs the other way: the proof rate
moves **1.36 % → 2.44 %** with the larger cap. The design's own §2 argues that in-tree
firings duplicate heavily (that is why C2 is bought), and §10.4 knows the difference
between all firings and rows that could enter D-537's count — so the document contains
both halves of its own correction and does not make it.

**W2 — the "second instrument" claim is false under the rule it invokes.** §1.1's
cap-2048 rows and F3's table are the same example binary, at the same revision, over the
same fixtures, at the same flags, arming the same hand-rolled wiring
(`trigger_census.rs:125-126`, `SolverWiring` built in the example, bypassing
`instance.rs:150`'s config path). `docs/process.md`: *"two instruments blind to the same
stage are one instrument reported twice."* The stage under doubt — what an armed
**config** costs, as against what the example's own wiring costs — is shared, unnamed,
and invariant under the agreement. This is a REPLICATION, which process.md names as a
separate and real thing, mislabelled as the thing it is not. It has already been
propagated into `docs/decisions.md` D-563 in the same uncommitted diff.

**W3 — §9's registered fold cost is wrong on its citation and on its seat, which is what
U2 asked to be verified.** §9 registers the bench over
`crates/pistol-cli/tests/fixtures/bench_solver_positions_v1.txt` — the **trigger-rich**
fixture, as §1.1 itself names it — and prices the fold at *"12.00 firings … (D-516,
MEASURED on the two committed seats at `nodes 50000`)"*. D-516's bands are corpus-15 /
corpus-35 / trigger-rich = **12.00 / 9.73 / 6.72**; 12.00 is the corpus figure. And
D-516's counts are **superseded by name** by D-517 (*"This supersedes D-516's CALL COUNTS
by name"*), whose trigger-rich figure at `nodes 50000`, cap 2048 is **9.05**. The
parenthetical *"MEASURED on the two committed seats at `nodes 50000`"* describes D-517's
measurement while attributing it to D-516 and taking D-516's superseded value.

**W4 — §9 says the bench's instrument is "a command block this document prints", and the
document prints no command block.** There are twenty-two code fences in the document and
none of them is a bench command; the only fence in §9 is the `sed` extraction rule. The
governing revision is therefore attached to nothing, and the dry run at `nodes 2000` whose
*"input and output are recorded with the pre-registration, before its review passes"* is
not recorded. T2 asked exactly this.

**None of the four overturns the cap recommendation's direction.** Every proxy I computed
still points the same way: a small cap yields more census than a large one, at no more
machine cost. What falls is the **magnitude** the operator is being asked to weigh
against D-530's countervailing fact, and the confidence the instrument claim licenses.

---

## §1.1 AUDIT

### The numbers, re-derived

Every count cell verifies exactly against the exported per-entry lines, and every one
came back identical when I rebuilt and re-ran.

| fixture / cap | firings (design) | Σ per-entry (exported) | my re-run | proofs | Σ | mine | `solver_nodes` | Σ | mine |
|---|---|---|---|---|---|---|---|---|---|
| trigger-rich 2048 | 294 | 95+97+102 = **294** | **294** | 4 | 0+0+4 = **4** | **4** | 1 160 027 | **1 160 027** | **1 160 027** |
| trigger-rich 16384 | 41 | 13+13+15 = **41** | **41** | 1 | 0+0+1 = **1** | **1** | 1 233 841 | **1 233 841** | **1 233 841** |
| corpus 2048 | 400 | 134+159+107 = **400** | **400** | 0 | **0** | **0** | 1 148 739 | **1 148 739** | **1 148 739** |
| corpus 16384 | 63 | 23+25+15 = **63** | **63** | 0 | **0** | **0** | 1 236 133 | **1 236 133** | **1 236 133** |
| both, gate OFF | 0 | **0** | **0** | 0 | **0** | **0** | 0 | **0** | **0** |

The emitted `row` lines count 294 / 41 / 400 / 63 in the exported files, matching the
firing sums exactly — so the row count and the firing count are the same quantity, as the
design assumes. Derived ratios: `1233841/1160027 = 1.0636` → **+6.4 %** ✓;
`1236133/1148739 = 1.0761` → **+7.6 %** ✓; `294/41 = 7.17` → **7.2x** ✓;
`400/63 = 6.35` → **6.3x** ✓.

**The sums in the table match the per-entry lines in the exported outputs. Yes, all of
them.** This half of §1.1 is as solid as a measurement in this repository gets.

### The wall-time column is a different matter

| configuration | design §1.1 | exported `wp20b_cap_time_a56449b.log` | **my re-run** |
|---|---|---|---|
| trigger-rich, gate on, 2048 | 149 s | 149 | **147** |
| trigger-rich, gate on, 16384 | 125 s | 125 | **133** |
| trigger-rich, gate off | 4 s | 4 | **4** |
| corpus, gate on, 2048 | 53 s | 53 | **62** |
| corpus, gate on, 16384 | 50 s | 50 | **50** |
| corpus, gate off | 3 s | 3 | **4** |

Three things follow.

1. **The counts are deterministic and the wall times are not.** My corpus cap-2048 run
   came back **62 s against 53 s — a 17 % swing**, which is three times the size of the
   corpus effect the design reports (53 → 50 s, 5.7 %). The claim *"the machine cost is
   flat in the cap"* survives this comfortably. The stronger claim in the same sentence —
   *"and slightly LOWER at 16384, not higher"* — is not resolvable at n = 1 per cell on the
   corpus fixture, though I note the direction did hold in both of my runs and in both of
   theirs.
2. **The ratio column is computed on a one-second denominator when a centisecond one
   exists.** The corpus ratios "18x" and "17x" are `53/3` and `50/3`. F3's own measurement
   of that same gate-off configuration is **3.63 s**, which gives 14.6x and 13.8x. **My
   measurement of it is 4 s**, which gives 15.5x and 12.5x. Three values for one quantity,
   the spread driven entirely by rounding a 3.63-second denominator to an integer.
3. **Wall and counts come from two different runs, and the design does not say so.**
   `cap_measure.sh` produced the outputs with `wall_s=$(echo … | bc)` and `bc` is not
   installed on this machine, so every `RESULT` line in the exported measure log carries
   an **empty** `wall_s=`. `cap_time.sh` was then run separately with `> /dev/null`. The
   table's row is a join across two executions. Harmless given the counts are
   deterministic — but see Y5 for what the exported evidence says on its own face.

### The inference, attacked

**"The price does not move" — SUPPORTED, and by more than the wall clock.** This is the
part of §1.1 I most expected to break and it holds. `solver_nodes` at +6.4 %/+7.6 % is a
better cost proxy than wall time here precisely because it is deterministic, and it is the
right proxy for the claim being made: the argument is that the solver absorbs the node
budget either way, and `solver_nodes` is that quantity directly. The `search_nodes`
columns corroborate it independently — the gate-on search receives 3 752 / 2 123 / 1 097
of its 400 000 nodes at cap 16384. The firing collapse is not a fixture artefact: it
appears on both fixtures, at both stone counts, with the same mechanism visible in the
data (`att_visits` and `def_visits` pinned at the cap in the 16384 rows), and the budget
binds identically because both arms spend the same `--nodes 400000`.

**"A 6–7x lever on census yield" — NOT SUPPORTED for the yield D-537 counts.** D-537:

> *"The minimum is counted in **WIN-PROVING FIRINGS ON DISJOINT POSITIONS**, not in
> firings and not in games."*

§1.1 quotes that sentence correctly and then, in the next clause, substitutes the raw
firing ratio for it. Two gaps:

**(a) Disjointness.** A firing is not a position. I computed the coarsest available proxy
over the exported rows — the tuple `(entry, turns_from_root, mover_hot, opponent_hot,
mover_w1, opponent_w1, mover_l3, opponent_l3, cover, cover_count)`, ten of the eleven
census columns the identity field is being added to disambiguate:

| fixture | cap | rows | distinct signatures | share |
|---|---|---|---|---|
| trigger-rich | 2 048 | 294 | **49** | 17 % |
| trigger-rich | 16 384 | 41 | **25** | 61 % |
| corpus | 2 048 | 400 | **26** | 6 % |
| corpus | 16 384 | 63 | **12** | 19 % |

**Raw firing ratio 7.17x / 6.35x. Distinct-signature ratio 1.96x / 2.17x.** Two firings
on one position necessarily share all ten columns, so this is not a bound in either
direction — but it is a direct measurement that the small cap's extra firings are
overwhelmingly *repetitions of a signature already seen*, exactly as one would expect of
deeper in-tree firings in the same search. The design's own §2 makes this argument in the
opposite direction to justify C2: *"the census population is in-tree, where a search tree
generates symmetric transpositions by construction."* You cannot buy C2 on the premise
that in-tree firings duplicate heavily and then price the cap on the premise that they do
not.

**(b) Win-proving.** Proofs per firing: **4/294 = 1.36 %** at cap 2048 against
**1/41 = 2.44 %** at cap 16384 — the rate roughly doubles with the cap, which is precisely
what D-530 predicts. Applied to the firing ratio this would give a proving-firing lever of
~4x rather than ~7x, consistent with the raw 4-vs-1 proof counts. And win-proofs are a
subset of proofs again: §10.4 records D-530's *"every one is a proven LOSS"*.

**Is the "indicative only" marking sufficient? No.** The MARKED LIMITS paragraph is
genuinely good — it marks the sample size, the bench-versus-corpus fixture class, the
4-vs-1 proof ratio, and it carries D-530's countervailing fact rather than burying it.
But it marks the *proof ratio* and never marks the *substitution*. And the recommendation
does lean on the unmarked substitution: the sentence that carries the operator decision is
*"a sweep armed at 16384 pays the same months and records about a SIXTH of what the census
exists to collect"* — and what the census exists to collect, by the ADR the sentence
cites, is win-proving firings on disjoint positions, of which "about a sixth" is not the
measured ratio of anything. The honest statement §1.1 can support is: **the cap is a
6–7x lever on raw firings; its lever on D-537's own denominator is unmeasured, and both
adjustments that can be estimated from this data point the same way — down.**

### The "second instrument" claim — my verdict

**It is the same instrument run twice, and the claim should be withdrawn.**

`docs/process.md` does not leave room here:

> *"AND IT NAMES THE STAGE UNDER DOUBT, and says how the second instrument does not share
> it: two instruments blind to the same stage are one instrument reported twice, and their
> agreement is invariant under a defect in what they are both blind to."*

§1.1 offers: *"a second instrument agreeing with the first on the quantity F3 hands the
operator, taken by a different session on a different day."* Different session and
different day are the two things the rule does not ask about. What both runs share:

- the same example, `crates/pistol-search/examples/trigger_census.rs`, at the same
  revision `a56449b`;
- the same fixture derivation, the same first-three-entries selection, the same
  `--nodes 400000`;
- **and the stage that is actually under doubt** — the example builds its own
  `SolverWiring` at `trigger_census.rs:125-126` and, in the design's own words, *"arms its
  own wiring independent of any config"*. F3's entire claim is about what happens when a
  **config** seat is armed. Whether arming through `instance.rs:150`'s
  `solver_wiring(section)` costs what the example's hand-rolled wiring costs is invisible
  to both runs and invariant under their agreement.

No stage under doubt is named, and the one that most obviously is under doubt is shared.
What was actually done is a **replication**, which process.md names in the same sentence
as a distinct and legitimate answer to instrument doubt — and which my own third run
strengthens further. The fix is one sentence: call it a replication, say what it does and
does not rule out, and either name a genuine second instrument (a real armed config seat
through the engine, which is the stage under doubt) or record that none was taken. The
same sentence has to come out of D-563.

### The recommendation's scope

**Not a scope violation.** §10.1 keeps the ruling with the operator in terms, §1.1 says
*"it is the operator's to take"* in terms, and CLAUDE.md's Process expects a design that
priced a decision to hand over the price. A design that measured a decision-relevant
quantity and then withheld what it implied would be the worse document. The defect is not
that §1.1 recommends; it is that the magnitude it recommends on is not the magnitude the
decision turns on (W1).

---

## CUT AUDIT — did anything load-bearing go?

**Overwhelmingly, no.** 1130 → 949 lines with §1.1 (67 lines) newly added means roughly
250 lines of history layer went, and I checked each named deletion against revision 4.

| cut | verdict |
|---|---|
| revision-header apparatus, the "revision N said" glosses, the WHAT REVISION 4 CHANGES table | **Clean.** Every finding ID those tables carried is still cited at the place it bites. |
| §10.0 ("whether v2 ruled on F1 or F2") | **Almost clean** — see Y4. |
| the duplicated F3 magnitude in §10.1 | **Clean.** This was Q3's D-423 complaint and it is properly fixed: §10.1 now points at §1.1 instead of restating it. |
| rev-4 invariant 3 ("a firing has exactly one row … unchanged from today") | **Clean.** It was explicitly the invariant that *"was already true before this package"*; rev-4's own 3a is now invariant 3 and is the one that can fail. |
| rev-4 invariant 6 (the cargo-feature refusal) | **Mostly clean, one residue** — see Y3. V6 was right that no crate in this workspace has a feature, so the refusal was over a build that cannot be produced. |
| §4's item-by-item D-551 audit | **Clean, and an improvement.** The audit was U1's home and it expired the moment a fifteenth column was added; §4 now keeps the RULE and drops the enumeration, which is what rev 4 itself said the design owed. |
| §8's D-553 narrative | **Clean.** The seven-row table survives with its "why a direct-call test would not do" column intact, which is the load-bearing part; the motivating-instance story is not. |

**Nothing that constrains an implementer was lost.** Every diff row, every invariant that
can fail, every test, every mutant and every number survived the cut, and the arithmetic
stayed consistent through it (18 tests − 7 call-removed = *"the remaining eleven"*, correct
where rev 4's 17 − 7 = *"ten"* was also correct).

**What should have been cut and was not.** §2's THE STRONGEST SURVIVING ATTACK block and
its answer run to ~30 lines to reach a two-line obligation in §9 that the document could
state directly; the ADR line is the place that quote is owed, and D-563's neighbour will
carry it. Minor, and I would not have raised it if the document had not made size a theme.

---

## DISCHARGE TABLE — T1–T3, U1–U8, V1–V12

| # | status | verified at |
|---|---|---|
| **T1** — arming rule kills `trigger_census.rs` | **DISCHARGED** | `Searcher::clear` is not changed (§6 row says so explicitly). Example verified unaffected: `:167` `engine.collect_trigger_census()`, `:195` `engine.clear()`, `:215` `for row in engine.take_trigger_census()` — all three line numbers exact, and `clear` (`search.rs:230`) still touches table/heuristics/solver only. The determinism obligation transfers cleanly: with the disarm on every exit path including the error path, `self.census` is `None` before `new_game` → `clear` ever runs (`instance.rs:74-76`), so the limb `clear` was carrying is unreachable rather than dropped, and test 16 drives two consecutive `go`s. §5's coldness proof now rests on invariant 8, which the design says out loud. |
| **T2** — bench names no instrument | **NOT DISCHARGED** | The negative half is now excellent and verified: `bench_delta.sh:92` is `CONFIG="configs/instrument_v0.toml"` ✓, `:351` writes the same `$budget` to both sides ✓, the `nps < 1.15` abort is at `:13` and `:452` ✓. The positive half fails: *"a command block this document prints"* — it prints none (22 fences, none a bench command), so no governing revision attaches and the promised `nodes 2000` dry run is unrecorded. See **W4**. |
| **T3** — (i) and (ii) are one comparison | **DISCHARGED** | §9 registers ONE comparison, and argues why rather than asserting it: *"Both costs … are paid under exactly one condition … One ON/OFF comparison at one seat measures their sum and no arm separates them."* The heading matches the body. The split into bench + structural check is real because the two exclude different classes. |
| **U1** — nine-count enumeration | **DISCHARGED** | Enumeration deleted; §4's *"three pairs"* is exact at `census.rs:41-58` — `TriggerColumns` holds `turns_from_root`, three `mover_*`/`opponent_*` pairs, and `cover`. |
| **U2** — §9's firing count / seat | **NOT DISCHARGED** | 12.00 is D-516's **corpus band 15**; §9's registered fixture is `bench_solver_positions_v1.txt` = trigger-rich = **6.72**; and D-517 supersedes D-516's counts by name with **9.05** for that band, seat and budget. The seat does not match. See **W3**. |
| **U3** — §6 row states only an exclusion | **DISCHARGED** | §6's `info.rs` row now reads *"`SearchOutcome` gains the census rows — **not** the per-depth `SearchInfo`"*: the addition first, the exclusion second. `info.rs:237-245` verified. |
| **U4** — the sink's route out of `ask` | **NOT DISCHARGED** | `run` is `pub fn run(transcript, label_nodes) -> Result<Vec<CaptureRecord>, _>` at `capture.rs:242`; its signature must carry the sink and it has no §6 row. `passes.rs` has no §6 row either, though §6.2 names `passes.rs:56`. And the §6 `arena.rs` row says *"the census file written beside the capture"* — but `arena.rs:73` delegates to `passes::capture`, which is what writes the capture file (`passes.rs:43-56`). The false sentence survives verbatim: *"`ask` gains a `&mut Vec<String>` sink … so its … contract and **every existing caller stay as they are**"* — adding a parameter changes its caller. |
| **U5** — the census file is not the arena's idiom | **NOT DISCHARGED** | Re-argued, still false at the site. The idiom is `Fixture` (`crates/pistol-cli/src/corpus/emit.rs:13`): `# <prose>` lines, then `# param <name> <value>`, then `# derived …`, then `# body_sha256 …`, then the payload. `capture_file.rs:63` writes `param capture_format_version`; `labels_file.rs:105` writes `param corpus_schema_version`. **Neither writes a bare kind token first**, and the `# body_sha256` digest — the thing that makes these files self-verifying under rule 8 — is absent from §6.2's spec. |
| **U6** — cap conclusion an unmarked extrapolation | **DISCHARGED** | Superseded by measurement, which is the right answer to D-291. No stale *"lower bound"* framing survives; the withdrawal is explicit. But the withdrawal created a new inconsistency — see **X1**. |
| **U7** — seat rule does not partition | **DISCHARGED, both halves** | Clause (a) added; every one of the eighteen rows now carries its clause letter and I checked the assignment row by row: (a) for 2, 3, 4, 11 (pure fold tests), (b) for 6 (grammar refusal), (d) for 9, (c) for the other twelve. Test 9's cell now names a real mutant — *"a firing site moved outside the wiring guard"* — and test 9 kills it: on a gate-off seat that mutant produces rows where the test asserts none. |
| **U8** — no stated ordering; `clear` limb unreachable | **DISCHARGED** | §6.1 states `collect → search → take → stop`, gives the reason (`take` panics on `None`, verified at `search.rs:216-221`), specifies what `stop` does with untaken rows (*"discards … unreachable rather than lossy"*), and drops the `clear` limb — which is exactly the repair U8 named. |
| **V1** — wrong crate for `position.rs:102` | **DISCHARGED** | `crates/pistol-engine/src/position.rs:102` is *"The stated `to_move` and `phase` are **checked, not trusted**"*. Exact. |
| **V2** — `capture.rs:227`, `exchange.rs:199-205` | **DISCHARGED** | `capture.rs:229` is `Step::Ignore => continue,` ✓; `exchange.rs:198` is *"The word after `key`, matched whole …"* ✓. (A **new** slip appeared two lines away — Y1.) |
| **V3** — invariant 1 states a falsehood | **DISCHARGED** | Rewritten to *"cannot end a search within the radius-8 region — invariant 6 owns the bound"*, which is what V3 asked for almost word for word. |
| **V4** — test 9's mutant cell | **DISCHARGED** | See U7. |
| **V5** — the block's PLACE is unpinned | **DISCHARGED** | Test 18, `the_census_block_is_emitted_after_the_last_depth_and_before_the_totals_line`, with its own row and its own mutant, and invariant 3 cites it. |
| **V6** — cargo-feature refusal over an impossible build | **DISCHARGED** | Both statements deleted (invariant 6 and the §3.1 sentence). Residue at Y3. |
| **V7** — unmarked ~119 800; "MEASURED-free" | **DISCHARGED** | *"D-560's **ESTIMATED** ceiling is ~119 800"*; option A's cell now reads *"free by INSPECTION"* and cites `pvs.rs:249`, which is `let key = self.position.state().key();` — exact. |
| **V8** — D-88's additive-line-kind clause misapplied | **DISCHARGED** | §3 now cites D-88's input-side strictness (*"the `set` form's grammar is as strict as the tokens it carries"*), which is the applicable clause. |
| **V9** — §10.0(a)'s "silent drift by definition" | **DISCHARGED** | §10.0 deleted, and the constructive consequence rev 3 and rev 4 both offered is finally taken: §10.3 says the `key_full` counting rule *"owes an ADR line and this package's closure is where it belongs"*. |
| **V10** — `wp21_DISPATCH.md` §4 omits F3 | **NOT DISCHARGED** | §4 still lists exactly three decisions owed — run WP-2.0b / lift the census gate / re-size the sweep — and not the gate-and-cap ruling §10.1 says WP-2.1 cannot proceed without. Same uncommitted diff as the D-563 that creates it. |
| **V11** — extraction rule off by one line | **DISCHARGED, and I reproduced it** | §9 prints `sed -n '1,/^# timing/p' <record> \| sha256sum` with `# the marker line INCLUDED`. Run: `prechange_gate_v0_run1` → `7f8a6f972c10…`, `run2` → `7f8a6f972c10…`, `instrument_v0_run1`/`run2` → `06490795663c…`. Both digests match §9's citation and run1 == run2 on both configs, so the invariance claim is MEASURED as stated. |
| **V12** — `Step` cited as `:150-158` | **DISCHARGED** | `capture.rs:150` `pub enum Step {` … `:157` `}`. Exact. |

**18 discharged, 5 not** (T2, U2, U4, U5, V10).

---

## NEW FINDINGS

### BLOCKING

#### W1 — §1.1's yield conclusion is about a quantity it did not measure, and the design's own data moves it toward a different answer

Stated in the VERDICT and derived in the §1.1 AUDIT. The measured lever on raw firings is
6.3x–7.2x; the measured lever on distinct column signatures is **1.96x / 2.17x**; the
proof rate moves the opposite way (1.36 % → 2.44 %). D-537's denominator is win-proving
firings on **disjoint positions**, and its own text excludes firings by name.

**Why blocking rather than major.** This is the sentence the operator acts on. The
document's headline offer is *"about six times as much of D-537's clock for the same months
of compute"*, and it is offered against a named cost — D-530's *"a small cap forgoes proofs
on the hardest rows"*. If the true lever on D-537's denominator is nearer 2x than 6x, and
the proof loss is real and measured in the same table, the trade the operator is being
shown is materially closer than the document presents it. That is a decision about 38 to 95
days of single-threaded machine time.

**The repair is small and does not need another run.** The distinct-signature computation
above takes seconds over artifacts already exported. State the raw-firing lever as the raw
lever, state the two adjustments and their measured directions, and let the recommendation
rest on the direction — which survives all of it — rather than on a magnitude that does
not. §10.4 already contains the vocabulary for this.

#### W2 — the "second instrument" claim is false under `docs/process.md`, and it has been propagated into D-563

Derived in the §1.1 AUDIT. Same example, same revision, same fixtures, same flags, same
hand-rolled wiring; no stage under doubt is named, and the obvious one — config-armed
against example-armed — is shared. `docs/decisions.md` D-563 now carries the same sentence
(*"which makes them a second instrument on the figure this line hands the operator"*), so
the fix reaches two files.

#### W3 — §9's registered fold cost cites a superseded ADR and takes the wrong band's number for the fixture §9 itself registers

Derived in the VERDICT. 12.00 is corpus band 15; §9's fixture is trigger-rich; D-516's
counts are superseded by name by D-517; the seat-and-budget-matched current figure is
**9.05**, giving ~0.21 ms rather than ~0.28 ms.

**The direction is conservative** — a higher predicted cost makes the "far under the IQR
gate" argument weaker, not stronger, so §9's conclusion survives the correction. It is
blocking anyway, for two reasons. First, this is a **pre-registration**, reviewed at the
revision that governs the run, and a registered number whose citation and seat are both
wrong is not landable as one. Second, and this is the part I would ask the author to sit
with: rev 4 fell partly for transcribing a reviewer's number instead of deriving it, the
rev-4 review's U2 *supplied* 12.00 in its own text on the assumption the fixture was
`bench_positions_v1.txt`, revision 5 then **changed the registered fixture to the
trigger-rich one** and kept the reviewer's number unchanged. The number was transcribed
across the very edit that invalidated it. Neither the reviewer nor the design noticed
D-517.

#### W4 — §9 asserts a command block it does not contain

*"The instrument is therefore a command block this document prints, and it is named with
its governing revision."* `/usr/bin/grep -n '^```'` over the design returns 22 fences; the
only one in §9 is the `sed` extraction rule. There is no command block, no revision
attached to it, and the dry run at `nodes 2000` — which the same paragraph says is
*"recorded with the pre-registration, before its review passes"* — is not in the document I
am reviewing. `docs/process.md` requires the artefact that produces a registered number to
be named with its revision, and the dry run's input and output to be recorded with the
pre-registration. Mechanical to fix, and it must be fixed before the run this section
governs.

### MAJOR

#### X1 — the document, its own F3 table, and D-563 state three different ranges for one measured quantity

The ONE LINE FOR THE MORNING says arming costs *"a MEASURED 17x-37x"*. F3's table, four
pages later and unchanged, says **14.5x** and **36.0x**. D-563's title says
*"A MEASURED 14.5x-36x"*. The whole difference is the denominator: §1.1 divides by
integer seconds (3 s, 4 s), F3 divides by 3.63 s and 4.11 s. **My replication measured the
corpus gate-off denominator as 4 s**, which gives 15.5x. Nothing in the document
reconciles the two, and §1.1's withdrawal is explicitly of the *"lower bound"* framing,
not of F3's numbers — so both tables stand.

Report the ratios at the precision the coarser instrument supports, or divide by F3's
better denominator and carry one range. Whichever is chosen, the head line, F3's table and
D-563 have to agree. This is D-423's exact shape, introduced by the section that fixed
Q3's instance of it.

#### X2 — see W4

Recorded as blocking; noting here that if the operator prefers to treat a missing command
block as a documentation gap rather than a pre-registration defect, it demotes cleanly to
this tier and the other three do not.

#### X3 — U4: the sink's route out is unspecified and the diff table names the wrong file for the write

`run`'s signature (`capture.rs:242`) and `passes.rs` are both absent from §6, and both must
change for a row to reach a file. §6 credits `arena.rs` with *"the census file written
beside the capture with its own `manifest_row`"*, but `arena.rs:73` delegates to
`passes::capture`, and `passes.rs:43-56` is what calls `run`, `render` and `manifest_row`.
The CLI flag is correctly `arena.rs`'s; the write is not. §3.1 is the section that argues a
correct writer whose call is never made is indistinguishable from a correct system — the
diff table should not leave the call site of its own sink unnamed.

#### X4 — U5: the census file's format is not the arena's idiom, and it omits the digest that makes the idiom auditable

`Fixture` writes `# param`/`# derived` header lines and a `# body_sha256` payload digest.
A bare `census_format 1` first line is neither, and the omitted body digest is exactly what
rule 8 and D-469 lean on when the artifact is 8.7 GB and uncommitted. Either write the
`Fixture` header (one call site, `capture_file.rs:59-72` is the template) or say plainly
that the census file is a wire-format dump and not a fixture, and give it its own reason.

#### X5 — the exported evidence contradicts the table it is the receipt for

`artifacts/wp20b_cap_measure_a56449b.log` is receipt-anchored and its own summary lines say,
for every one of the six configurations:

```
RESULT fixture=trigger-rich gate=on cap=2048 exit=0 wall_s=
  census_rows=0
```

`wall_s` is empty because `bc` is not installed; `census_rows=0` because the harness greps
`^census` where the rows are `trigger_census: row …`. An auditor reading the artifact the
receipt indexes sees *zero census rows in every configuration*, which is the exact opposite
of what §1.1 claims — and only reaches the truth by re-summing the per-entry lines, as I
did. The underlying data is right and reproduces; the receipt's face is wrong. Re-run the
summary extraction, or state in §1.1 that the `RESULT` lines are known-broken and the
per-entry lines are the record. Given that this same package's §3.1 turns on the difference
between a correct writer and a receipt that lies about it, the irony is worth the fix.

#### X6 — §9's [0.98, 1.02] bracket is not a prediction the registered instrument can contradict

The design is admirably honest that the bench *"excludes a GROSS regression and cannot see
the fold"*, with the fold's predicted cost two to three orders below the IQR gate. Given
that, a ratio inside [0.98, 1.02] is a statement about the instrument's noise and not
about the change, and no result outside it distinguishes a census defect from a warm
cache. The only number that does work is the 0.95 abort. Registering a bracket the named
instrument cannot resolve is prose that constrains nothing (D-424), and it invites a
successor to read a 0.97 as a finding. Register the abort, say the bracket is the noise
floor, and let test 17 carry the exclusion — which is what §9's own last sentence already
says.

### MINOR

#### Y1 — a new off-by-two in `capture.rs`, in the same block V2 corrected

§3.1 quotes `capture.rs:170` as the `INFO_PREFIX` test and `:171` as `return Step::Ignore;`.
Actual: `:170` is `return Step::Totals;`, `:171` is `}`; the `INFO_PREFIX` arm is
**`:172-173`**. `:165` and `:229` in the same block are right, and the rev-4 review itself
cited `:172` correctly. Fifth citation slip in this file's lineage; the first one revision 5
introduced.

#### Y2 — V10: `wp21_DISPATCH.md` §4 does not carry the decision §10.1 says blocks it

Not a defect in this design, but it lives in the same uncommitted diff and it is the
document a successor reads first. §10.1's *"WP-2.1 cannot register a census, and D-537's
clock cannot start, until this is ruled"* has no counterpart in the dispatch's own
"Decision owed" list.

#### Y3 — deleting rev-4's invariant 6 leaves the stub engine's behaviour under a census request unstated

V6 was right that the *cargo-feature* refusal was over a build that cannot be produced. But
the invariant also said *"a token the engine cannot honour is a named refusal, never a
silent no-op"*, and §6 lists `crates/pistol-arena/src/bin/stub_engine.rs` — the second
`Engine` implementor, `go_reporting` at `:146` — as changing *"with the required trait
method"* without saying what it does with the request. Under rule 3 a stub that accepts a
census request and silently returns no rows is the shape §3.1 exists to forbid, one layer
down. One clause in the §6 row settles it.

#### Y4 — §10.0's deletion removed the only place the document asked the operator to confirm its two departures from the governing dispatch

F1 dissolves the dispatch's scope item 3 and F2 empties scope item 1's option field. Both
are, in my judgement, right — F2's constraint belongs to a landed and reviewed document
that names this package out loud. But rev 4's §10.0 ended with *"An operator who intended
v2 AS a ruling should say so"*, and revision 5 presents both departures as settled findings
with no open question anywhere. §10 is the list of what the package does not decide;
whether the dispatch's scope survives its own premise being false belongs on it, in a line.

#### Y5 — two small overstatements in §2 and §5 about where the fold's inputs come from

(a) §2: *"C2's fold … allocates nothing beyond `canonical_form`'s own transforms."*
`Board::stones()` (`board.rs:91`) returns an iterator and `canonical_form`
(`symmetry.rs:165`) takes `&[(Coord, Player)]`, so a `collect` precedes the call. The
comparative claim against C1 is unaffected — C1 pays it too — but "allocates nothing
beyond" is not literally true.
(b) §5: *"the root site has the identical shape (`search.rs:304-307`)"*. The **guard** is
identical (`self.census.is_some().then(|| …)`), verified. The **capture** is not: the tree
closure destructures `state` at `pvs.rs:604` and captures it, while the root closure
captures `&mut self.position` and calls `root_census_columns`. `state` is in scope at the
root site so nothing is blocked, but an implementer reading "identical" will look for a
capture that is not there.

---

## THE STRONGEST SURVIVING ATTACK ON REVISION 5

**Revision 5 went and measured the thing nobody had measured, and then read the
measurement as an answer to a question it does not answer — and the document contains the
correction, twice, in its own voice.**

The instruction rev 4 fell on was to derive the property rather than fix the sentence.
Revision 5 obeyed it magnificently on the cost axis: the cap question was extrapolated, it
is now measured, it reproduces bit-exactly under an independent rebuild, and the answer
inverted the prior — the cap turns out not to be a cost lever at all. That is the best work
in this package.

On the yield axis it did the older thing. §1.1 measures **firings**, because firings are
what the instrument prints. D-537 counts **win-proving firings on disjoint positions**, and
says *"not in firings"* in the sentence §1.1 quotes. The document then puts the firing
ratio into the operator's headline as D-537's clock. And both corrections are already
inside the document: §2 buys C2 on the argument that in-tree firings duplicate heavily —
which, if true, is precisely why 294 firings are not 294 positions, and I measured that
they collapse to 49 distinct column signatures against 25 at the larger cap, a 1.96x lever
where the headline claims 6-7x. §10.4 separately distinguishes all firings from *"a census
writing a row only where it could enter D-537's count"*. The design knows the difference
between its numerator and its denominator in two other sections and forgets it in the one
the operator reads.

The same shape produced W2 and W3. The "second instrument" is a replication renamed rather
than a second instrument found, and what it is blind to — the config-armed seat, which is
the entire subject of F3 — is nowhere named. The 12.00 firings survived an edit that moved
the registered fixture out from under it, from a reviewer's text into a pre-registration,
past a supersession recorded in the ADR log at D-517.

**So the attack is not that revision 5 measured badly.** It measured well and it
reproduces. The attack is that **this document is still being written outward from the
instrument's output rather than inward from the claim that has to be supported**, and every
one of the four blocking findings is that one habit at a different site. The prior rounds
diagnosed it as transcription. It is not transcription any more; it is the same reflex with
better sources.

**And the good news is that none of it costs a run.** The distinct-signature statistic is
seconds of Python over artifacts already exported and receipted. The instrument claim is
one withdrawn sentence in two files. The firing count is one lookup in D-517. The command
block is a paste. What has to change is which quantity the headline is denominated in — and
when it changes, the recommendation it supports does not: every proxy I could compute
still says arm small, and none of them says arm at 16384.
