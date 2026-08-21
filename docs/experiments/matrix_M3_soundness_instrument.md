# MATRIX M3 — the differential gate's instrument — **STOPPED, NOTHING SELECTED**

**Status: ATTACKED, EVERY OPTION FELL, NO SELECTION MADE.** Matrix authored at
`f8e73e4`; DECISION-RED-TEAM report at
`docs/experiments/matrix_M3_REDTEAM.md`. Owning unit:
`docs/experiments/U4_soundness_instrument.md` §8.

> **THE MATRIX BODY IS BELOW, VERBATIM AS ATTACKED, AND IS NOT EDITED.** A matrix
> corrected after its attack is a matrix that was never attacked. Everything in
> this STOP RECORD is stated above the body, not inside it.

## THE STOP, and why it is a stop rather than a revision 2

The round returned **EVERY STATED OPTION FALLS** — S-A, S-B, S-C, S-D, S-F, S-H
and S-I on grounds the matrix itself argues and the red team could not break;
**S-E on four independent attacks**, and S-G with it.

The architect's standing rule for this work package is explicit: *if a red team
kills every option, stop that matrix, report to the architect, do not force a
survivor.* **It is followed here.** Nothing is selected. No ADR line may cite S-E,
or any option below, as adopted — which was already true before this round and is
now true for a measured reason instead of a procedural one.

**This is NOT the disposition matrix M4 received**, and the difference is the
rule: M4's round left two options standing, so M4 was re-authored as a revision 2
over a completed field. M3's round left none standing. A revision 2 authored on
the same afternoon by the same session that wrote revision 1, over an option set
the red team supplied, is precisely the pattern D-305 measures and this matrix
was written to stop — the red team supplied the surviving option in four of six
matrices in this WP. **The second authoring round is owed and it is the
architect's to schedule.**

## THE DECISIVE ATTACK, verified independently before this record was written

S-E's criterion asserts that the emitted set EQUALS the union over the
**inclusion-minimal** covers, checked against a referent *"written against the
plan-family definition"*. The plan-family definition D-266 makes canonical is
`docs/research/threat_calculus_v1.md`. MEASURED:

```
$ grep -c 'inclusion' docs/research/threat_calculus_v1.md
0
$ grep -n 'DEF-T ' docs/research/threat_calculus_v1.md
30:| DEF-T | threat number t(F) | exact **minimum hitting set** over plan family F …
```

The canonical reference states the **opposite convention** and never uses the
operative term. So a referent genuinely written from that reference computes the
minimum hitting set, while the criterion needs the inclusion-minimal union — and
MEASURED on a legally reachable FILTERED node the two readings differ,
`{(-1,0),(-1,5),(4,0)}` against `{(-1,0)}`.

**Independence and greenness are therefore in TENSION, not merely unenforced.**
The matrix disclosed a hole here and called the control judged-not-mechanical;
the hole is worse than the disclosure said. A referent that is genuinely
independent makes the gate RED ON A CORRECT ENGINE; a referent that is green has
consulted `cover.rs`, which is D-295's defect wearing S-E's name. **That is the
same shape as mutation M4 in this unit's own ledger** — the ledger already
records that minimum-cardinality and inclusion-minimal differ on a reachable
position, which is the admission the recommendation needed to read and did not.

## THE OTHER THREE ATTACKS ON S-E, and one is the author's own false number

- **D-115 is applied inconsistently.** The matrix kills S-F *"on landed ADR lines,
  not on judgement"* — and S-E's primary mechanism, widening
  `pistol_search::staged` to `pub` for a test's benefit, is what D-115 forbids by
  name. An option adopted over a landed line, in a matrix that killed a rival
  with that same landed line, is adopted by judgement exactly where the matrix
  said judgement was not needed.
- **Fact 5 is a MEASURED number that does not reproduce, and it is the author's.**
  The matrix said *"MEASURED **8** plain `assert!` against **2**
  `debug_assert!`"* in `pvs.rs`, offered as evidence that S-E's always-on
  `assert!` is the house idiom. **VERIFIED against the shipped file: production
  `pvs.rs` carries exactly ONE always-on `assert!`, at line 244.** The 8 counted a
  comment line and six asserts inside `#[cfg(test)]`. The claim it supported —
  "not a novelty" — is the reverse of what the tree says.

  ```
  $ awk '/#\[cfg\(test\)\]/{exit} /assert!/ && !/debug_assert!/ && !/^\s*\/\//{print NR}' \
      crates/pistol-search/src/pvs.rs
  244
  ```

  **THIS IS THE SECOND MATRIX THIS SESSION IN WHICH A NUMBER THE AUTHOR MARKED
  MEASURED DID NOT REPRODUCE** — M4's fact 6b was the first. Two instances, one
  session, one author, both in cells supporting the recommendation. Recorded here
  rather than in a finding list somebody else keeps, because D-291's clause has
  now recurred often enough in this work package that the pattern is the finding.
- **Ground 3's "the two-half split is FORCED by the tree" dies to a one-word
  edit** that builds and clippies clean, and **flip clause 3's remedy flips to a
  population the red team MEASURED firing the criterion at 3.1 % against the
  corpus's 25.0 %** — an 8× dilution in the direction the remedy moves, so the
  clause makes the gate weaker where it promises coverage.

## FOUR MISSING ROWS — where a survivor would live

The field was incomplete as well as dead. None of these is costed or attacked,
and **selecting from them without a round is the pattern D-305 measures**:

1. **The composite the design actually writes as adopted** — "S-E, with the
   reduced S-C beside it". The matrix entered the halves and never the pair.
2. **A calculus-lower-bound containment criterion**, which would be stated in the
   convention `DEF-T` actually uses and so is immune to the decisive attack.
3. **A per-node census** — record and compare rather than assert.
4. **REUSE OF THE LANDED FROM-SCRATCH REFERENT.**
   `crates/pistol-solver/tests/common/reference.rs:223` already implements
   `blocking_covers` *"by the definition: every subset within budget that covers,
   minus every one with a proper subset that also covers"* — the independent
   referent S-E proposes to write **already exists in the tree**. The matrix's own
   fact 4 ran the `ls` that printed that file and did not read it. This row is
   also immune to the decisive attack, because the existing referent is written in
   the inclusion-minimal convention rather than the calculus's.

## WHAT THIS BLOCKS, stated so it is not rediscovered

The DIFFERENTIAL GATE (D-316's second named gate) has **no selected instrument**.
U4-Z item 4 stays blocked, B1 stays open, and `tools/staged_soundness_check.sh`
cannot be specified for that gate. The other three named gates are unaffected.

---
---

# THE MATRIX — VERBATIM AS ATTACKED at `f8e73e4`

*Everything below this line is the text the DECISION-RED-TEAM was dispatched
against, unedited. Its "Status: AUTHORED, NOT SELECTED" is the state at attack
time, and it remains true — nothing was selected. The corrections are above.*

> # MATRIX M3 — the differential gate's instrument
> Status: **AUTHORED, NOT SELECTED.** Awaits fresh-context DECISION-RED-TEAM.
> Subject: the instrument of the DIFFERENTIAL GATE (the second of the four named
> soundness gates, D-316). Owning unit:
> `docs/experiments/U4_soundness_instrument.md` §8, u-rev 2. Authored at `77f7397`.
>
> ## THIS MATRIX HAS NEVER EXISTED. IT IS NOT A RECOVERY.
>
> - **MEASURED, restructure red team F7 and revision-7 review B1:** the superseded
>   design contained exactly three `| Option |` tables, none of them §8's. `S-A`,
>   `S-B` and `S-D` occur once each at `6feb40a`, in prose; **`S-E` occurs ZERO
>   times at `ec8f7fb`**, the revision a recovery would come from.
> - So the adopted-looking option was never in a matrix in any revision. **S-E was
>   supplied by the DECISION-RED-TEAM that killed S-C**, and a red team's
>   replacement option has never itself been attacked. D-305 records that the red
>   team supplied the surviving option in four of six matrices in this work
>   package; that is the base rate this matrix exists to stop resting on.
> - **Nothing below is selected, and the incumbency of S-E is not evidence.** It is
>   entered as a row like every other row.
>
> ## WHAT THE OPTIONS ARE OPTIONS ABOUT — the stage under doubt, named
>
> **Does the staged generator ever drop a cell a proven tactic needs?**
>
> Naming the stage is load-bearing, not throat-clearing. Revision 1 named the
> DEFECT and never the STAGE, and the instrument it then chose (S-C) was blind to
> exactly the stage the defect lives in. CLAUDE.md's own words: *a criterion that is
> a property the named defect class PRESERVES passes vacuously and is not a
> criterion.* Every row below is scored first on whether its criterion can be
> FALSIFIED by a generator that drops a needed cell.
>
> ## FACTS THE MATRIX STANDS ON — MEASURED at `77f7397`, with the command
>
> | # | Fact | Command / source |
> |---|---|---|
> | 1 | **`crates/pistol-search/src/staged.rs` does not exist.** The staged generator is not built; every option here is priced BEFORE any of it exists, which is the only time this matrix can be honest about cost | `ls crates/pistol-search/src/` — 13 entries, no `staged.rs` |
> | 2 | **`pvs` is `pub(crate) mod pvs;`** (`crates/pistol-search/src/lib.rs:40`), so `Run` is unreachable from an integration test. `candidates`, `error`, `fallback`, `info`, `params`, `score`, `search`, `stop`, `tt` are `pub`; `ordering`, `position`, `pv`, `pvs` are `pub(crate)` | `grep -n 'mod ' crates/pistol-search/src/lib.rs` |
> | 3 | **The independent-referent pattern already exists in the crate that would host the test.** `crates/pistol-search/tests/common/` carries **nine** modules including `reference.rs`, `reference_walk.rs`, `reference_invariants.rs`, `agreement.rs`, `ref_score.rs` | `ls crates/pistol-search/tests/common/` |
> | 4 | **`crates/pistol-solver/tests/common/plans.rs` exists and is another crate's integration-test module** — no `src/` can `use` it, which is why revision 3's seam could not be built | `ls crates/pistol-solver/tests/common/` |
> | 5 | **An always-on `assert!` in `pvs.rs` is the house idiom, not a novelty**: MEASURED **8** plain `assert!` against **2** `debug_assert!` in that file today | `grep -oE '(^\|[^_a-z])assert!' crates/pistol-search/src/pvs.rs \| wc -l` → 8; `grep -c debug_assert!` → 2 |
> | 6 | **A legality referee already runs in that crate's test tree.** `crates/pistol-search/tests/common/mod.rs`'s `position` helper interleaves stones into the turn structure rule 3 imposes and replays them through `GameState`, "so a fixture that no legal game could reach fails loudly here" | read `crates/pistol-search/tests/common/mod.rs` |
> | 7 | D-115 permits in-source `#[cfg(test)]` blocks **for private invariant guards only**; D-129 fixes `assert!` = a CORRECTNESS invariant active in every profile, and forbids the quiet demotion of one to `debug_assert!` | `docs/decisions.md` |
>
> **Numbers carried from U4 §8, MEASURED by the sessions that took them and NOT
> re-measured here** — each is cited with its source so a reader can tell my
> measurements from theirs:
>
> | Value | What it measures | Source |
> |---|---|---|
> | **28 class assertions, 0 RED** | S-C's whole class gate under D-124's own reproducer (`if cells.len() > 1 { cells.pop(); }` after `order`), confirmed applied by node counts falling 8794 → 8374, 10482 → 10045, 12260 → 11880 | U4 §8.1 |
> | **0 of 62** position-depth cases | the forced-block class is EMPTY as S-C wrote it (`argmax()` returns `Vec<Turn>`, a `Turn` is `Pair`, the free second stone never hits) | U4 §8.1 |
> | **3 of 7** mutations | provably cannot fire against S-C/S-D | U4 §8.1 |
> | **243 363 538 nodes in 554.2 s** | ONE of 31 positions, full-width reference at radius 2 depth 3 — S-C's registered workload | U4 §8.1 |
> | **17.89 s** | the reduced S-C mate-class regression at radius 1, three fixtures plus one built mate-in-3 | U4 §8.2 |
> | **ESTIMATED 40–90 s** | the whole soundness gate per CI run, dominated by one traversal per fixture | U4 §13 |
>
> ## Options
>
> | Option | The criterion, and where it observes | Cost | Failure modes |
> |---|---|---|---|
> | **S-A — value agreement** (staged root value == full-width r2 root value, depths 1..=3) | Root values only. | **MEASURED void as a cost question** — it never reaches costing. | **FALLS. The criterion is preserved by the defect class.** D-124's defect is interior-node narrowing that never moves a root maximum; a dropped cell that was not the argmax leaves the value identical. Revision 1 rejected S-A on exactly this ground — and then adopted S-C, which the same argument kills. |
> | **S-B — candidate-set containment** (staged set ⊇ the radius policy's set) | The emitted set, per node. | **ESTIMATED** cheap; the radius set is already computable. | **FALLS on two counts.** (i) Containment is preserved by every OVER-generating mutation, which is the same vacuity as S-A one level down. (ii) It is FALSE as a claim under the adopted Tier-T option: U4 §8.3 MEASURES the FILTERED row's set as a subset of the radius-2 set and NOT of the radius-1 set — at `mate_in_3_double_three_becomes_double_four`, **157 of 182** FILTERED descendants have a cover cell outside the radius-1 ball. A gate asserting containment there is RED on a correct engine. |
> | **S-C — class-restricted answer agreement** (root argmax agreement on restricted position classes) | Root argmax. | Registered workload **MEASURED not runnable**: 243 363 538 nodes / 554.2 s for one of 31 positions. | **FELL, and it is the only row here that fell to measurement rather than to argument.** 28 class assertions, 0 RED under D-124's reproducer. The forced-block class is empty (0 of 62). Inside it the argmax restraint is vacuous — `\|argmax\| = 1` on all four members, so membership IS identity, which D-119 refuses by name. |
> | **S-D — S-C plus a mutation gate** | S-C's criterion, plus the eight-mutation ledger as an acceptance test. | S-C's cost plus the ledger's. | **FALLS WITH S-C.** A mutation gate measures the criterion's strength; it does not supply a criterion. MEASURED: 3 of 7 of revision 1's mutations provably cannot fire against S-C at all, so the ledger's own verdict on S-C was inflated by construction. Bolting a strength measurement onto a vacuous criterion measures vacuity precisely. |
> | **S-E — per-node survival-set EQUALITY, in two halves** | **Half one:** at every node where `can_win_this_turn` is `None` **AND** `blocking_covers` answers `Minimal` (U2 §5.3's FILTERED row), assert the emitted set **EQUALS** the union over inclusion-minimal covers, against a referent written from scratch in `crates/pistol-search/tests/common/`. **Half two:** an always-on `assert!` in `visit` that the first `forced` candidates are searched unless a cutoff or abort intervened. | `pistol_search::staged` becomes `pub` (fact 2 makes this necessary, since `pvs` cannot be reached). One integration test walking the existing reference walker (fact 3). One `assert!` in `visit` — **MEASURED** the house idiom there, 8 already (fact 5). Gate cost **ESTIMATED 40–90 s** per CI run, one traversal per fixture. | **Widening the public surface is a real cost and D-115 constrains it** — the module goes public for a test's benefit. The equality criterion is only as good as the independent referent, and an independently-written referent that is WRONG makes the gate red on a correct engine. Half two's `assert!` fires in every profile, so a false positive is a production abort, not a test failure. **Coverage is the corpus, not the population** — see S-G. |
> | **S-F — the in-source `cfg` seam** (revision 3's shape: observe inside `pvs` behind `#[cfg(debug_assertions)]`) | Inside `visit`, no public surface widened. | **ESTIMATED** the cheapest of the observing options. | **FALLS on landed ADR lines, not on judgement.** D-129 fixes `assert!` = a correctness invariant active in every profile and forbids the quiet demotion; a soundness criterion behind `debug_assertions` is that demotion by definition. MEASURED consequence: it would be compiled out of `tools/search_oracle_check.sh`, `tactical_check.sh`, `determinism.sh` and `movetime_check.sh`, **all four of which drive release binaries** — the gate would be green in CI while never having run. Recorded as a row because the design tried this shape and abandoned it, and a rejection no matrix states is a rejection nobody can check. |
> | **S-G — S-E, plus sampled legal positions** | S-E's criterion, evaluated additionally on positions drawn by a playout sampler rather than only on the fixture corpus. | S-E's cost plus a sampler — **partially built already: MEASURED, `playouts.rs` exists in BOTH `crates/pistol-search/tests/common/` and `crates/pistol-solver/tests/common/`**. **ESTIMATED** the marginal work is wiring, not construction. Gate time grows with the sample and would need its own budget. | **The strongest rival, and its cost is a schedule risk rather than an argument.** A sampled gate's runtime is a knob nobody has set, and a per-CI-run gate whose cost is unbounded is how a gate gets disabled. Sampling also needs the referent S-E builds, so it cannot be done FIRST — it is strictly downstream of S-E, which is what makes it a poor rival and a good successor. |
> | **S-H — the generator self-checks** (the staged generator asserts its own output against the cover union it just computed) | Inside the generator. | **ESTIMATED** the cheapest option that observes anything. | **FALLS on CLAUDE.md's own criterion clause.** The check and the suspect share their input: the generator's cover arithmetic is the thing under doubt, and a component agreeing with itself is "internal agreement between components sharing an input", named in CLAUDE.md as a property that passes vacuously. It would catch a wiring slip between computing the union and emitting it, and nothing about whether the union is right. |
> | **S-I — no differential gate** (the null row) | Nothing. The tactical suite, the colony family, the pattern fixtures and SPRT carry the whole load. | Zero. | **The defect class goes unmeasured.** D-124's flip clause reads *"flips when that check lands, which is where the visibility question gets settled"* — under S-I it never lands and D-124 stays open indefinitely. Rejected, but STATED: the restructure red team's F11 found a matrix missing its null row, and a rejection nobody can check is not a rejection. |
>
> ## Recommendation
>
> **S-E.**
>
> Grounds, in the order they bind:
>
> 1. **It is the only surviving row whose criterion the defect class can FALSIFY.**
>    That is the test every row is scored on and it eliminates S-A, S-B and S-H
>    directly, S-D through S-C, and S-I by construction. Equality rather than
>    containment is what does the work: mutation M3's phase-1 cover cells are
>    provably a SUBSET of the phase-0 union, so failing to regenerate can only
>    OVER-generate — and a containment criterion would have let M3 live. This is
>    §8.1's own charge against S-C, applied to S-E before S-E is adopted.
> 2. **It is D-124's named remedy, quoted and then not implemented.** *"What would
>    help is comparing the engine's per-node candidate set with the reference's."*
>    Adopting it discharges a flip clause that has been open across work packages.
> 3. **The two-half split is forced by the tree, not chosen for elegance.** Fact 2:
>    `pvs` is `pub(crate)`, so a test cannot see `Run`. Fact 4: the solver's
>    `plans.rs` is another crate's test module, so `src/` cannot use it. A test of
>    the generator cannot see a drop made AFTER generation, which is precisely
>    D-124's reproducer — so the cheap half is in-source and, by D-129, must be a
>    plain `assert!`. Fact 5 measures that this is the existing idiom in that file.
> 4. **The referent is independent in the sense D-295 requires.** D-295's defect was
>    a shipped oracle checked against a test-side oracle DERIVED FROM THE SAME
>    FIXTURE. Here a shipped generator is checked against a from-scratch
>    plan-family implementation, the pattern rule 7 already names for movegen
>    (`bruteforce.rs`) and the search (`reference.rs`), and fact 3 measures that
>    pattern already present in the hosting crate.
>
> **The weakest cell, named so the red team starts there.** Ground 4 asserts the
> referent will be independent; nothing enforces it. An implementer who writes the
> referent by reading `cover.rs` produces D-295's defect wearing S-E's name, and
> **no criterion in this matrix would detect that** — the gate would be green, the
> mutation ledger would pass, and the instrument would be measuring its own
> reflection. The mitigation available is procedural, not mechanical: the referent
> is written against the plan-family definition and the reviewer of the IMPL commit
> is asked whether it was. **That is a judged control, and this matrix says so
> rather than claiming a check it does not have.**
>
> ## What flips it
>
> Each clause names a remedy its trigger can actually reach — F5's class, where the
> previous matrix's trigger was about units 1 and 4 and its remedy deferred unit 3.
>
> - **The independent referent cannot be written independently.** Trigger: the
>   plan-family definition turns out to be specified nowhere except in `cover.rs`,
>   so any referent is a transcription of the subject. Remedy: **the gate falls back
>   to S-E half two alone** — the in-source `assert!`, which needs no referent and
>   still catches D-124's reproducer — and half one is replaced by the mutation
>   ledger as an explicit strength measurement rather than a correctness criterion.
>   Reachable because the two halves are independent by construction: half two is an
>   `assert!` in `visit` and does not consult the referent at all.
> - **The public `staged` surface is refused.** Trigger: a reviewer holds that
>   making `pistol_search::staged` public for a test's benefit breaches D-115.
>   Remedy: **S-E half one moves into an in-source `#[cfg(test)] mod tests` block**,
>   which D-115 permits for private invariant guards, and the referent moves with
>   it. Reachable, and it costs the behaviour-level pinning that `tests/` gives —
>   which is the trade the flip is choosing, stated rather than hidden. It does NOT
>   flip to S-F: the observation stays in a test, and no correctness invariant is
>   demoted to `debug_assertions`.
> - **The corpus is shown not to exercise the FILTERED row enough to be a gate.**
>   Trigger: a MEASURED count of FILTERED nodes across the gate corpus that is small
>   enough for the criterion to be near-vacuous in practice — the concrete number is
>   the count of nodes where `can_win_this_turn` is `None` AND `blocking_covers`
>   answers `Minimal`, and it is measurable once the generator exists. Remedy:
>   **flip to S-G**, adding sampled legal positions to reach the population the
>   corpus misses. Reachable because S-G is S-E plus a sampler and — MEASURED, S-G's
>   cost cell — `playouts.rs` already exists in the hosting crate's `tests/common/`;
>   nothing in S-E is undone.
> - **The gate's per-CI cost exceeds its budget.** Trigger: the ESTIMATED 40–90 s
>   measures out materially higher once the traversal is real. Remedy: **reduce the
>   gate's corpus, not its criterion** — S-E's criterion is per-node and its cost is
>   linear in nodes walked, so the knob is which fixtures the gate walks. Named
>   explicitly because the tempting remedy is to weaken equality to containment, and
>   that is the one move this matrix's ground 1 forbids.
> - **NOT a flip trigger, stated so it is not mistaken for one:** the mutation
>   ledger's own composition. Whether M1–M8 are the right eight mutations changes
>   how strongly S-E is EVIDENCED, not which option is right. It moves no cell in
>   this table.
>
> ## COST OF THE DECISION THIS MATRIX FEEDS
>
> Selecting costs one DECISION-RED-TEAM dispatch. **IMPLEMENTING S-E** costs, all
> **ESTIMATED** because nothing of it exists (fact 1): one `pub` module and its
> entry point; one integration test in `crates/pistol-search/tests/` plus a
> from-scratch referent in that crate's `tests/common/`; one `assert!` in `visit`;
> and the script wiring under `tools/staged_soundness_check.sh`, a `tools/` change
> carrying a SHELL_CHECKLIST review answered item by item and the coverage rule's
> test driving the shipped script. Per-CI-run cost **ESTIMATED 40–90 s**, carried
> from U4 §13 and not re-derived here. **No number in this matrix was produced by a
> run this session took**; every MEASURED value is either a structural fact of the
> tree with its command beside it, or is cited to the session that measured it.
>
> ---
>
> *Matrix M3, authored fresh at `77f7397`. Never existed at any prior revision. Not
> selected. Awaits DECISION-RED-TEAM.*
