# WP-2.0b — census position identity on the wire, gated. IMPLEMENTATION.

> **ONE LINE FOR THE MORNING.** The census now carries the identity D-537's
> disjointness rule needs, behind a `go`-line token no committed config sets;
> the diff is what the design said it would be, the five carried findings are
> closed inside it, and the two things this document registers BEFORE they run
> are the perf guard's rejection band and its governing revision.

Governing design: `docs/experiments/wp20b_design.md` revision 8, landed at
`a6777f4`. Governing dispatch: **WP-2.0b v2**, transcribed at
`docs/experiments/wp20_dispatches.md`. Carried findings:
`docs/experiments/wp20b_IMPL_FINDINGS.md`.

---

## 1. THE FIVE CARRIED FINDINGS, CLOSED INSIDE THE DIFF

| id | what was owed | what closes it |
|---|---|---|
| **AG2** | H1 is defined twice — §9 registers it as the cross-binary token-OFF comparison and the surrounding prose reads in places as the single-binary ON/OFF ratio. | §3 below states H1 **once**, in the bullet that owns it. Every other mention here points at that bullet (D-423). The single-binary ON/OFF comparison keeps its own name — the **gross-regression abort** — and is never called H1. |
| **AG3** | H1 has no rejection region; the review MEASURED the paired sd at **0.0075**, so ±2 % at REPS=5. | §3 registers **`H1 rejected outside [0.98, 1.02]`** on the cross-binary comparison, BEFORE the run. |
| **AG4** | The fourth-word refusal has no counterpart in the grammar section's own wording. | The exact text is quoted in `crates/pistol-cli/src/budget_token.rs`'s `///` docs on `parse_budget`, and pinned by `a_fourth_word_on_a_go_line_is_refused_naming_the_fourth_word`. |
| **AG5** | The governing dispatch's four mutants are not accounted for by name against §8's rows. | §2 below, one table. |
| **AF2-residual** | The calibration run is unsized — deliberately, and not the implementer's. | Unchanged and not taken here. It is the operator's, and this arc's Phase 3 is where it is registered and run. |

---

## 2. THE DISPATCH'S FOUR MUTANTS, AGAINST THE ROWS THAT KILL THEM (AG5)

The dispatch's obligation reads *"Mutants, per D-55y run green BEFORE REVIEW-impl,
call-removed mutants included: identity column dropped -> schema test dies; token
check call removed -> zero-bytes test dies at the call site; warm-table read
introduced -> D-527 seat dies; transposition ruling inverted -> fixture dies."*

| dispatch mutant | the design's row | the test that must die | where it lives |
|---|---|---|---|
| identity column dropped -> schema test dies | §8 test 1 | `a_census_row_carries_the_canonical_key_of_the_position_it_fired_at` | `crates/pistol-cli/tests/census_protocol_tests.rs` |
| token check call removed -> zero-bytes test dies AT THE CALL SITE | §8 test 5 | `without_the_token_a_go_line_writes_no_census_byte` | `crates/pistol-cli/tests/census_protocol_tests.rs` |
| warm-table read introduced -> D-527 seat dies | **NO §8 ROW EXISTS**, and the DESIGN's §5 says why: this package adds no table and reads none | `a_plain_go_after_a_census_go_computes_no_key_and_emits_no_line`, which is the only state this package can carry across a `go` | `crates/pistol-cli/tests/census_protocol_tests.rs` |
| transposition ruling inverted -> fixture dies | §8 test 4 | `a_known_transposition_pair_counts_as_one_disjoint_position` | `crates/pistol-core/tests/canonical_key_tests.rs` |

**THE THIRD ROW IS THE ONLY ONE §8 HAS NO ROW FOR, and that is the point rather
than an omission.** D-527's defect is a
`Searcher` reused across entries without `clear()`; this package adds no table,
so there is no warm-table read for a mutant to introduce. What it DOES add that
can survive a `go` is the census collector itself, and invariant 8's disarm is
what stops it — so the seat that dies is the one that reads the fold counter
across two consecutive `go`s.

---

## 3. THE PERF GUARD — THE REGISTRATION, WRITTEN BEFORE THE RUN IT GOVERNS

**REVISION 2 of this section. Revision 1 governed a run and should not have**:
REVIEW-impl round 2 read it as a pre-registration and ruled it **MAY NOT GOVERN
A RUN**, on ten limbs. It could not decide its own verdict — it named a
per-position statistic and never said whether one position outside the band
rejected H1 or twenty had to be, it registered an estimator the instrument did
not compute, and it carried an agreement clause with no consequence whose answer
was already on disk. **The first run's numbers stand on the record and are
superseded** (§3.3); this revision governs the re-run.

### 3.1 THE INSTRUMENT, NAMED WITH ITS DIGESTS

`docs/process.md`: *"An artefact that produces a registered number … is named in
the pre-registration WITH ITS REVISION."* The two files ARE the instrument, and
the authority is the exported file rather than any block quoted in prose:

| file | sha256 |
|---|---|
| `artifacts/wp20b_perf_guard.sh` | `648a96d87686dbead588cc1ff3753edabedc2963c1a5f4d101b7a9b7b9be7573` |
| `artifacts/wp20b_perf_report.py` | `d29222bea4a515d9e5a4dd780c317d73d44b9201cd23b55aeec4bea7cf95f985` |

**An edit to either reopens this registration.** Revision 1 said its governing
revision was *"recorded in the receipt beside the numbers"* — after the run,
which is the one place a pre-registration may not put it.

**THE WORKLOAD IS THE DESIGN'S §9 BLOCK; THE INSTRUMENT IS NOT.** Revision 1's
§3 opened *"the instrument is the command block printed in `wp20b_design.md` §9,
unchanged"* and then, twenty lines later, said it was that block plus a third arm
and four counters. The second sentence is the true one. What is unchanged from
§9 is the WORKLOAD — seat `configs/bench_wp18c_solver_on.toml`, fixture
`crates/pistol-cli/tests/fixtures/bench_solver_positions_v1.txt` (20 entries),
`go nodes 50000`, and the two `%%`-filtered lines whose absence §9's own comment
block shows would make the guard pass while measuring nothing. What is added is
below.

### 3.2 WHAT IS ADDED TO THE WORKLOAD, AND WHY EACH ADDITION CANNOT WIDEN A VERDICT

1. **A third arm** — the PRE-change binary at `go nodes 50000`. H1 is defined by
   §9 as a cross-binary comparison and cannot be taken without it. The arm
   implements §9's own definition; it does not extend it.
2. **FOUR per-arm attribution counters** — `totals_lines`, `bestmove_lines`,
   `empty_board_answers`, `census_rows`. **Every one can only VOID a run, never
   pass one.** The first three exclude the defect §9's comment block is about;
   `census_rows > 0` on the ON arm and `= 0` on both OFF arms excludes an ON arm
   that paid none of the cost it is being measured for — the hole a ratio of
   1.000 cannot see from the inside. `= 20` is an EXTERNALLY DERIVED referent:
   the fixture holds exactly twenty entries, counted.
3. **The arm order rotates by rep, and `REPS = 6`.** A fixed order lets any
   drift across a rep land on the arms in the same sequence every time, so it
   does not average out. **Six because the rotation has a three-cycle**: at
   `REPS = 5` the orders used are 0,1,2,0,1 and `post_off` averages slot 2.000
   against `pre_off`'s 2.200 — a −0.200-slot residual, tenfold better than the
   fixed order's −2 and not zero. At `REPS = 6` it is exactly zero.

### 3.3 THE ONE STATISTIC, THE ONE DECISION RULE, AND WHAT REJECTS H1

**H1 = `1.000x`, a no-change hypothesis in D-249's shape**, on the **CROSS-BINARY
token-OFF** comparison: the post-change binary's nps against the pre-change
binary's (`artifacts/pistol_prechange_a56449b`, sha256 `180b4c40…`).

**THE STATISTIC, spelled so the instrument and the document cannot mean two
things by it** — ratio FIRST, then median, because a median of ratios and a
ratio of medians differ at the third decimal and revision 1 registered one while
computing the other:

```
for each position p and rep r:   ratio(p, r) = post_off(p, r) / pre_off(p, r)
for each position p:             value(p)    = median over reps of ratio(p, r)
H1's statistic                             = median over the 20 positions of value(p)
```

**H1 IS REJECTED IF AND ONLY IF THAT ONE NUMBER FALLS OUTSIDE `[0.98, 1.02]`.**
The band is AG3's, from the rev-8 review's MEASURED paired sd of `0.0075`.

**THE PER-POSITION VALUES ARE A DIAGNOSTIC AND REJECT NOTHING, and that is a
decision rather than an omission.** An any-position rule is wrong on the band's
own terms: at `sd = 0.0075`, `P(|x − 1| > 0.02) = 0.00766` per position, so
`P(at least one of twenty outside) = 0.143` — a 14 % false-rejection rate
against this project's registered 0.05 convention, and the band's own derivation
sample already contains a per-position `1.021`. The spread is printed because a
reader should see it; it carries no verdict.

**THE AGREEMENT CLAUSE IS DELETED.** Revision 1 required the verdict to *"hold
under all three estimators or the disagreement is the finding"*. It named no
consequence — `docs/process.md` forbids exactly that — and its answer was
already computable from raw output on disk, which is the same document's *"a run
whose answer is already known before it is taken"*. Under D-424 a clause that
changes no reading is prose and is deleted rather than refined. **One estimator
is registered and the instrument computes that one.**

**THE GROSS-REGRESSION ABORT IS A DIFFERENT COMPARISON AND IS NEVER CALLED H1.**
It is the single-binary token-ON against token-OFF ratio, by the same statistic,
and it **ABORTS below 0.95**. Common-mode cost is paid on both of its arms and
pushes it toward 1.000, which is why it excludes only a large effect and why the
leak question belongs to H1.

**THE FOLD'S OWN PLACEMENT IS EXCLUDED STRUCTURALLY, NOT BY TIMING.**
`the_fold_is_entered_exactly_once_per_firing` asserts exactly that; at a MEASURED
22.99 µs a fold, no nps comparison at this seat resolves a hoist, a per-node call
or a double call.

**A REFUSAL ON ANY FIXTURE LINE VOIDS THE RUN**, per §9's own words: the guard's
whole content is that every arm searched the same positions.

**THE POST BINARY IS IDENTIFIED AT THE RUN.** The pre-change binary is fixed here
by digest; the post-change one cannot be, because it is built at the closure
head. **The run records `sha256sum target/release/pistol` in its receipt**, and a
receipt without it is void — for a cross-binary comparison, which post binary was
measured is the whole attribution.

**COST.** 3 arms x 20 positions x **6** reps = **360 searches**. MEASURED at the
five-rep run: 300 searches took 22.6 minutes of search time (1 356 330 ms
summed), so six reps is **~27 minutes of search** and about 30 of wall. The box
is idle for it: the seat is the one F3 measures as expensive.

### 3.4 THE DRY RUN — TAKEN WITH THE INSTRUMENT §3.1 NAMES

`docs/process.md`: *"A pre-registration's literal commands are exercised before
its review passes, on an input of the SAME KIND as the registered workload."*
Revision 1's dry run was taken **before the rotation existed** and could not
cover the instrument it was cited for. This one is the registered instrument at
its registered digests, differing from the governed workload only in budget:
`REPS=6 BUDGET=2000`, output `artifacts/wp20b_perf_dryrun_v3.txt`.

| criterion | the defect it excludes | result |
|---|---|---|
| refusals `0` | the two filters dropped, every line refused, both arms ~1.000 | **0** |
| `totals_lines` = `bestmove_lines` = **20** on every arm | the fixture read short | **20 / 20** on all three arms |
| `empty_board_answers` `0` | the engine searched the empty board | **0** on all three |
| `census_rows > 0` on ON, `0` on both OFF arms | an ON arm that paid nothing | **18 / 0 / 0**, identical across all six reps |
| **18 `arm` lines in six distinct orders** | the rotation not actually rotating | **6 reps x 3 arms**, each arm in each slot **twice** |

**ERRATUM ON THE LAST ROW, RECORDED RATHER THAN REWRITTEN.** *"six distinct
orders"* is a criterion the registered instrument can never satisfy: the
rotation has a THREE-cycle, so `REPS=6` produces three distinct orders each used
twice, which is what the raw shows and what the result cell already states. **The
registered wording is not edited** — a criterion rewritten after its run is the
post-hoc threshold move `docs/process.md` forbids, whichever direction it moves.
What is recorded instead is the disposition: the check actually made is *"each
arm in each slot equally often"*, it is strictly what the named defect — the
rotation not rotating — requires excluded, and a registration that asked for six
distinct orders asked for something a three-cycle cannot produce. Found by
REVIEW-impl round 3 (m3).

It is not a governed sample and does not consume this registration's run.

**The dry run's own H1 is `1.0056` and is not a result**: at `nodes 2000` a
search is a tenth of the registered budget and per-position noise is
correspondingly larger. What it establishes is ATTRIBUTION — that the commands
reach the intended positions, that every arm ran, and that the ON arm paid.

### 3.5 THE GOVERNED RUN, AND ITS VERDICT

**REVISION 2's FIRST FRESH-CONTEXT REVIEW IS REVIEW-impl ROUND 3**
(`docs/experiments/wp20b_impl_REVIEW_r3.md`, committed `a518cc7`,
`2026-09-02 15:17:35 +0200`), which read this section as a pre-registration,
found round 2's ten limbs discharged and confirmed the instrument computes the
statistic §3.3 registers. **THE RUN THIS SECTION FIRST REPORTED PREDATED THAT
REVIEW AND THEREFORE DID NOT GOVERN**: `artifacts/wp20b_perf_guard_v2.txt` was
taken at `14:52:31`, and this document, the receipt and the ledger each said a
review had preceded it. No such artefact existed. The three sentences are
deleted, that run joins revision 1's on the superseded record
(`artifacts/wp20b_perf_RECEIPT_v2_SUPERSEDED.txt`), and **the governed run is
the RE-RUN below**, taken after round 3's commit with the instrument at §3.1's
digests. Raw `artifacts/wp20b_perf_guard_v3.txt`, receipt
`artifacts/wp20b_perf_RECEIPT.txt`.

RUN_TABLE_PLACEHOLDER

| what | value |
|---|---|
| **H1**, the registered statistic | **0.9982** — inside `[0.98, 1.02]`, **NOT REJECTED** |
| gross-regression comparison | **0.9994** against an abort at 0.95 — **no abort** |
| time-to-depth, both comparisons | **1.0000** |
| refusals | **0** |
| `census_rows` ON / OFF / OFF | **181 / 0 / 0**, identical across all six reps |
| arm order | six reps, each arm in each slot **twice** |
| per-position spread (DIAGNOSTIC, no verdict) | `[0.9878, 1.0043]`, none outside the band |

**THE FIRST RUN IS SUPERSEDED AND ITS NUMBER IS NOT THIS PACKAGE'S.**
`artifacts/wp20b_perf_guard_v1.txt` was taken under revision 1 of this section —
a registration a fresh context later ruled could not govern a run — in a fixed
arm order, and read with an estimator the registration did not name. It is kept
because a superseded measurement stays on the record, and it is not cited for
anything.

### 3.6 ERRATUM AGAINST THE LANDED DESIGN

`wp20b_design.md` §9 registers H1 as *"exactly `1.000x`"* and registers no
rejection region. AG3 is the design's own carried finding that a `1.000x`
hypothesis without a band cannot be rejected, and its remedy is the measured
`[0.98, 1.02]` above. **A design is not edited after it lands**, so where §9 and
this section differ, THIS section governs — AG3 is the design's finding and
`wp20b_IMPL_FINDINGS.md` is the document that carried it. A successor reading §9
alone would find a hypothesis with no way to fail.

---

## 4. WHAT LANDED, AGAINST THE DESIGN'S §6

Every row of §6's diff table, and the two the design owed honesty about.

- `pistol-core`: `canonical_key(stones) -> Key128`, the fold of §2 option C2,
  exported. `# Panics` names the radius-8 bound that makes the overflow panic
  unreachable (invariant 6).
- `pistol-search`: `TriggerObservation` gains `key`; the fold runs inside the
  existing census closure at both firing sites; `Searcher` gains
  `stop_trigger_census` and the fold counter; `SearchOutcome` gains the rows.
- `pistol-engine`: `CensusRequest`, `go_reporting`'s third parameter, the
  arm/take/disarm rule on every exit path of a census `go`, the re-exports F1's
  route needs, and `EngineError::CensusUnsupported`.
- `pistol-cli`: the optional third word and its two refusals; `census_line` in
  §4's field order; the block emitted after the last depth and before totals.
- `pistol-arena`: `Step::Census`, the classifier arm ahead of the `info`
  catch-all, the `ask` sink, the `--census` flag, and the census file with its
  own body digest and manifest row.

**TWO THINGS THE DESIGN SAID WOULD NOT MOVE, AND DID NOT.**
`crates/pistol-cli/tests/workspace_shape_tests.rs::pistol_cli_manifest_names_only_core_and_engine`
is unchanged and green — F1's claim, verified rather than asserted. No committed
config changed.

**ONE THING THE DESIGN SAID WOULD MOVE, AND DID.**
`the_label_go_line_is_the_one_budget_section_spells` moved out of
`capture_tests.rs`; it pinned an equality the token breaks, and its replacement
in `census_capture_tests.rs` pins that equality OFF the token and the token
appended ON it.

**TWO DEPARTURES FROM THE DESIGN'S LETTER, STATED HERE RATHER THAN LEFT TO A
REVIEWER TO FIND.**

1. **`TriggerObservation` carries `key_pos` beside `key`, and the wire does
   not.** §9 registers *"tranche one emits `key_pos` beside the canonical key"*
   as the two-line answer to §2's strongest surviving attack. The overnight
   dispatch's Phase 2 runs its tranches with **the census OFF**, so no tranche
   of this arc can carry the column; and §4's field order is pinned by a report
   test, so the wire may not grow one. The measurement is therefore taken on
   `crates/pistol-search/examples/trigger_census.rs` — the instrument every
   census number in this arc was measured with — which now prints both keys.
   The field is free at the firing: `GameState::key` is carried incrementally
   and reading it is one XOR.
2. **Tests 7, 8 and 12 are pinned twice, once seated and once as pure
   functions.** The seated copies read the engine's own rows on
   `configs/gate_staged_solver_v0.toml`. The pure copies read `census_line`,
   which is what §8's seat rule (a) admits, and they exist because the absent-
   defender spelling is **unreachable from any seat**: the engine's rows at
   every committed armed config ask the defender.

**THE SINK'S MEMORY WAS O(WHOLE ARTIFACT) THREE TIMES OVER, AND IT IS FIXED
RATHER THAN CARRIED.** REVIEW-impl round 1 counted three resident copies
BY INSPECTION — it says so in its own reproducer, *"by inspection rather than
by burning the RAM"*, and did not run an 8 GB capture — before the first byte
reached disk: `ask`'s `Vec<String>` sink, `Fixture`'s body `String`,
and `render`'s output `String`. Against §2's own **ESTIMATED** ~8.7 GB sweep figure that
is an **ESTIMATED** ~26 GB, on a 46 GB box.

**What changed, and it is one copy now — the sink the design specifies (§6.2)
and nothing beside it.**

1. **`pistol_cli::sha256::Sha256`**, a streaming digest. The one-shot
   `sha256_hex` becomes it fed once, so the two cannot disagree and the
   published FIPS vectors that pin one pin the other. A second test pins that
   **where the pieces are cut is not part of the answer** — every split of one
   payload, including at 63, 64 and 65 bytes and one byte at a time.
2. **`census_file::write_into`**, which walks the rows TWICE — once for the
   digest, once for the bytes — through a `BufWriter`, holding one line at a
   time. `Fixture` gains `render_header`, so the header still comes from the one
   builder every sibling artifact uses and the digest line still carries the one
   `BODY_DIGEST` spelling.
3. **`census_file::manifest_row` becomes INFALLIBLE.** It used to read the
   digest back out of a rendered string and could fail to find one; it now
   computes it from the rows by the same expression the writer uses. **A refusal
   that cannot fire is not a guard**, so it is gone rather than left as
   decoration.

**The header's own `# derived rows` line is still the count, and the digest is
still over exactly the bytes the second pass writes** — one expression, called
by the writer and by the manifest row alike, so the row cannot come to name a
digest of something other than what was written.

---

## 5. WHAT THE MUTATION RUN FOUND, INCLUDING THE TEST IT KILLED

Twenty-six mutants, in a worktree on `/home` with its own target directory,
each naming the test that MUST die at it. **Run 1 (`artifacts/wp20b_mutants_v1.txt`,
revision `8efeda0`): nineteen dead at their registered test, ONE ALIVE.**

**M10 — the fold hoisted out of the in-tree census guard — SURVIVED
`the_non_census_path_does_not_compute_a_canonical_key`, and the test was the
defect.** A firing has two sites: the root's, in `search.rs`, and the in-tree
one in `pvs.rs`. At `Stop::Nodes(600)` the root's own two solver calls spend 537
of the budget, so **the in-tree site is never reached at all** — and a fold
hoisted there is invisible to a counter that stays at zero because the code was
never entered. The test passed on a property the defect PRESERVES, which is
`docs/process.md`'s named vacuity in its exact shape.

**The fix is in the test and not in the code**, and it makes the test prove its
own premise: it now runs the workload once WITH a census and asserts some row
carries `turns_from_root > 0` — the in-tree site was reached — before running it
again with no census and asserting the fold count is zero. The budget moved from
600 to 1 500 nodes for the same reason.

**AN EARLIER REVISION OF THIS SECTION NAMED RUN 5 AS THE RECEIPT AND THAT WAS
WRONG** (REVIEW-impl round 3's M2). Run 5's tree at `595f004` differs from the
reviewed one in **eleven source and test files** — `census.rs` and
`census_identity_tests.rs` among them, the very files round 2's remedies rewrote
— so run 5 knows nothing about `CensusKeys`, M27, M28 or M29, and the sentence
*"the receipt covers the reviewed code exactly"* was false at that revision. Run
5 stays on the record below as one of the superseded runs it is.

**THE RECEIPT THIS PACKAGE CLOSES ON IS THE RUN TAKEN AT THE REMEDIED TREE**,
whose count, revision and verdict are in
`docs/experiments/wp20b_VERIFICATION.md` §2 together with the `git grep` receipt
D-568 requires a mutation set to be specified against. **The B1 remedy added four
mutants — M30 to M33, one per defect class per PUSH SITE** — because the set as
it stood registered a mutant only inside `CensusKeys::at`, the one place the
exchange defect no longer needed.

**THE FOUR EARLIER RUNS ARE ON THE RECORD AND TWO OF THEM FAILED**, which is
recorded rather than tidied away:

| run | revision | outcome |
|---|---|---|
| v1 | `8efeda0` | 19 dead, **M10 ALIVE**, EXIT=1 — the vacuity above |
| v2 | `dfba9d7` | 20 dead, 0 alive, EXIT=0 — superseded by the fix round |
| v3 | `1cd3364` | 23 dead, **M24 ALIVE**, EXIT=1 — a FOURTH vacuous test, the census identity's version |
| v4 | `0aa0e2e` | **ABORTED** after 23 mutants on a stale patch anchor, EXIT=1 |
| **v5** | **`595f004`** | **26 of 26 dead at their registered test, 0 alive, 0 faults, EXIT=0** |

**v4's abort is itself a finding and the harness changed for it.** One stale
anchor exited the whole run, so a receipt would have reported fewer mutants than
were registered — the same defect class this set exists to catch, one level up.
A stale anchor is now a named HARNESS FAULT per mutant: the run continues, the
registered count is printed beside the verdicts, and the exit code is non-zero if
any fault occurred. **v5's line states all five numbers**, which is why it can be
read without the file.

---

## 6. THE ARMED SEAT'S COST, AND THE FIXTURE CHOICE THAT PAID IT

A test build multiplies solver time per node by orders, and the fixture the
census's own unit tests use is a 35-stone midgame whose legal region — the union
of radius-8 balls — makes every node expensive. **MEASURED**: the first draft of
`census_protocol_tests.rs` on that fixture ran **173 s for ONE armed `go nodes
2000`**, and the four-test `census_identity_tests.rs` ran **198 s**.

Both suites were moved to `bench_positions_v1.txt`'s first band-15 entry — a
COMMITTED workload, the other fixture the census's own unit tests read.

**AND THEN THE COST WENT BACK UP, WHICH THIS SECTION SAID IT HAD NOT.** An
earlier revision recorded `census_protocol_tests` at **1.88 s** and
`census_identity_tests` at **4.94 s**, and gave the reason as a **depth** budget
rather than a node one. Both statements were overtaken by the fix that closed
REVIEW-impl round 1's M1: the shared armed run had to reach the IN-TREE firing
site as well as the root, and `depth_turns 2` fires once, at the root. The
budget is now `go nodes 4000` — five firings, four of them in-tree — and the
shipped code says so. **MEASURED at the current tree**: `census_protocol_tests`
**84.4 s**, `census_identity_tests` **83.8 s**.

**So the fixture change bought about an order and the coverage fix spent it**,
and that is the honest arithmetic: 173 s for ONE armed search on the long
fixture, against ~84 s for a suite of fourteen on the short one. The numbers are
here because a stale cost in this section is exactly what a reviewer measured and
found false by 45x.
