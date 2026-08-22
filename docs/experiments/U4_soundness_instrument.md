# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT

<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->

**HOW TO RESOLVE A `§n` IN THIS FILE.** Every `§n` is the SUPERSEDED document's
own numbering, kept unchanged so an existing citation still resolves. A `§n` that
names a section this unit does not own is prefixed with the unit that does
(**U1**–**U4**, or `WPQ_seed.md`) wherever it appears in prose written or
retargeted by the carve. Inside text carried VERBATIM — matrix cells, quoted
sentences, the seed — a bare `§n` may still name a section that now lives
elsewhere; `docs/experiments/section_owner_table.md` maps every one of them to
its owner, and that is what it is for.


**u-rev 5.** Carved from `docs/experiments/wp15b_design.md` §8, §9, §11.6 and §12
item 1 at `6feb40a` (revision 7, never reviewed, CLOSED by D-309) under the
restructure selected as option D by D-310. The carve's section-to-owner map is
`docs/experiments/section_owner_table.md`. The superseded document is not
in the tree: it is retrievable at `6feb40a` and nowhere else.

**TWO OF THIS UNIT'S SELECTIONS ARE OPEN, AND THE CARVE DID NOT MAKE THEM OPEN —
IT FOUND THEM SO.**

- **M3 (the soundness instrument).** No matrix exists at any revision, and the
  adopted option S-E occurs **zero** times at `ec8f7fb`. FRESH matrix, never
  authored, never attacked. The slot is stubbed at the head of §8.
- **M4 (the snapshot's config seam).** The matrix exists at `ec8f7fb` and is
  recovered verbatim at the head of §9, but the diff against an externally
  derived referent is **non-empty** — three MEASURED falsifications of its cells
  by the design's own later §9 amendments. T1' says a diff that differs means a
  fresh round.

Both fresh DECISION-RED-TEAMs are the architect's dispatch. **The carve selects
nothing.** §8 and §9 below are carried as the RECORD of what was argued, and
U4-Z's ADR lines are written on that footing.

**THE TEXT IS OTHERWISE A VERBATIM CARVE** apart from cross-reference retargets
and three named repairs, each stated where it occurs: **B4** (the tactical-suite
gate's "all three staged tactical configs … not just the two gate ones", false in
both halves, corrected to the four-config reality), **MINOR 15** (§8.2 carried a
paragraph spliced in twice mid-sentence; the duplicate is removed and the
sentence closes), and — **since u-rev 2** — **B3**, repaired by shape 2 under
D-316: the gate letters are dropped, the four gates are named, and the S-E
double-list dies with them. The two-shape comparison the architect selected from
stands unedited in U4-Z beneath its selection record. Every **MEASURED**
and **ESTIMATED** mark is the mark the superseded text carried.

**LABEL DISCIPLINE — D-311, travelling item T5.** Any append to this unit bumps
its u-rev, however small the diff. A review is dispatched against a named
revision and reviews of superseded revisions do not transfer; the superseded
document carried the label "Revision 7" at both `d94dc0a` and `6feb40a`, which
differ by 69 lines, and that ambiguity is what this rule removes. A citation of
another unit names the unit AND the u-rev cited.

**THIS UNIT HAS NOT BEEN REVIEWED** at this u-rev.

Theory citations are calculus IDs from `docs/research/threat_calculus_v1.md`
(D-266). This unit restates no theory; where it appears to, the calculus wins and
the disagreement is an ADR line.

---

## U4-A. Lineage — what has attacked this unit's content, and at which revision

| Round | Against | Verdict reaching M3 / M4 / §8 / §9 |
|---|---|---|
| DECISION-RED-TEAM, matrix M3 | revision 1, `ec8f7fb` | **M3 FELL.** S-C was blind to D-124's own reproducer — `cells.pop()` after `order` leaves the class gate at 28 assertions, 0 RED. **S-E was supplied by that red team and has never itself been in a matrix** |
| DECISION-RED-TEAM, matrix M4 | revision 1, `ec8f7fb` | **M4 SURVIVES AMENDED** — at a text three of whose cells the design has since MEASURED false. See §9 |
| REVIEW-design | revisions 2–6 | all FAIL. The tactical-suite gate's derivation (then §8.3(a)) was redesigned **three times** (rows 17, 32 and 27 of the superseded §0), and §8.4's ledger was rebuilt once |
| REVIEW-design | revision 7, `6feb40a` | **FAIL** — 7 BLOCKING, 7 MAJOR, 9 MINOR. **B1, B3, B4 and MINOR 15 are this unit's**, and MAJOR 8 (M4's and M6's witnesses are not positions a legal game reaches) is this unit's and is OPEN |

**What this unit owes that no round has given it:** a REVIEW-design of THIS text
at THIS u-rev, two fresh DECISION-RED-TEAMs, and a SHELL_CHECKLIST review of the
`tools/` change §8.7 and §9 each carry.

---

## 8. MATRIX M3 — the soundness instrument — FELL

> ## SELECTION OPEN — M3 HAS NO MATRIX, AND S-E IS NOT SELECTED HERE
>
> **B1 of the revision-7 review, and F7 of the restructure red team.** MEASURED,
> `grep -c "^| Option |"` over the superseded document returns **3** — **U1** §4.2 (M0),
> **U2** §5.6 (M5), **U3** §6.3 (M1). §8 has none. `S-A`, `S-B` and `S-D` occur once each in
> prose; the matrix that held them was deleted at revision 2 and never restored.
> **The adopted option S-E is not in it either:** MEASURED, `S-E` occurs **zero**
> times at `ec8f7fb`. S-E was supplied by the DECISION-RED-TEAM that killed S-C,
> so there is nothing to recover — M3 is a FRESH matrix that has never been
> authored and has never been attacked in the form it would be selected in.
>
> **CLAUDE.md:** "An option adopted without a matrix, or a matrix never attacked,
> is the same breach as silent architecture drift."
>
> **THE MATRIX SLOT, STUBBED. The carve does not fill it and does not select.**
>
> | Option | (b)'s instrument | Cost | Failure modes |
> |---|---|---|---|
> | S-A — value agreement, as WP-1.1 | *to be authored* | *to be authored* | *to be authored* |
> | S-B — candidate-set containment | *to be authored* | *to be authored* | *to be authored* |
> | S-C — class-restricted answer agreement | *to be authored* | *to be authored* | *to be authored* |
> | S-D — S-C plus a mutation gate | *to be authored* | *to be authored* | *to be authored* |
> | S-E — per-node survival-set containment | *to be authored* | *to be authored* | *to be authored* |
> | *further options* | *the red team supplied the surviving option in four of six matrices in this WP (D-305); the slot is not closed at five rows* | | |
>
> **Every numeric claim entered in that matrix is marked MEASURED or ESTIMATED,
> and an estimate that could have been measured in seconds is a finding** —
> D-291, third occurrence in this work package (restructure red team F1).
>
> **THE MATRIX WAS AUTHORED AND ATTACKED AT u-rev 4, AND EVERY OPTION FELL.**
> `docs/experiments/matrix_M3_soundness_instrument.md` (authored `f8e73e4`) put
> nine options — the five above plus S-F, S-G, S-H and a null row — to a
> fresh-context DECISION-RED-TEAM, whose report is
> `docs/experiments/matrix_M3_REDTEAM.md`. **VERDICT: every stated option falls.**
> Under the architect's standing rule the matrix is STOPPED and no survivor is
> forced; the second authoring round is owed and is the architect's to schedule.
> D-317 records it.
>
> **THE DECISIVE ATTACK, and it lands on the text below.** S-E's criterion is an
> EQUALITY against the union over **inclusion-minimal** covers, checked by a
> referent "written against the plan-family definition". MEASURED: the plan-family
> definition D-266 makes canonical, `docs/research/threat_calculus_v1.md`,
> contains `inclusion` **zero** times, and its `DEF-T` states the OPPOSITE
> convention — *"exact **minimum hitting set**"*. On a legally reachable FILTERED
> node the two readings give `{(-1,0),(-1,5),(4,0)}` against `{(-1,0)}`. **So a
> referent that is genuinely independent makes the gate RED ON A CORRECT ENGINE,
> and one that is green has read `cover.rs` — D-295's defect.** Independence and
> greenness are in TENSION. §8.4's own mutation M4 is the ledger's admission that
> the two conventions differ on a reachable position, which is what the S-E text
> below needed to read and did not.
>
> **AND ONE ROW WAS NEVER NEEDED TO BE WRITTEN.**
> `crates/pistol-solver/tests/common/reference.rs:223` already implements
> `blocking_covers` *"by the definition: every subset within budget that covers,
> minus every one with a proper subset that also covers"* — the independent
> referent S-E proposes to build EXISTS, in the inclusion-minimal convention, and
> the matrix's own fact 4 ran the `ls` that printed the file without reading it.
>
> §8.1 through §8.7 below are carried verbatim as the RECORD of why S-C fell and
> of what S-E was written to do. **They are not a selection, and after this round
> they are not a recommendation either.** **No ADR line may cite S-E as adopted** —
> item 4 in U4-Z is written on that footing, and D-317 now gives that footing a
> measured ground rather than a procedural one.


### 8.1 Why S-C fell

**MEASURED, not argued.** D-124's own reproducer applied verbatim to
`pvs::visit` — `if cells.len() > 1 { cells.pop(); }` immediately after `order`,
confirmed applied by node counts falling 8794 → 8374, 10482 → 10045, 12260 →
11880 at depth 2 radius 2 — leaves S-C's entire class gate at **28 class
assertions, 0 RED**, identical to the pristine baseline.

D-124's defect is INTERIOR-node narrowing that never moves a ROOT maximum. S-C is
still a ROOT oracle; restricting *which roots* you assert on buys a root oracle no
interior visibility. **Revision 1 rejected S-A as "the criterion the defect class
preserves" and then adopted a criterion the same defect class preserves.**

Four more findings, each measured:

- **The forced-block class is EMPTY as written.** `ReferenceRun::argmax()` returns
  `Vec<Turn>` and a `Turn` is `Pair(Coord, Coord)`; the free second stone is never
  a hitting cell, so "the argmax set is a subset of the cells that hit every
  opponent plan" is unsatisfiable — 0 of 62 position-depth cases. The loose
  reading gives 4 positions at one depth. The reading that would admit `t = 2`
  positions IS `Cover::cells()`, i.e. mutation M3 itself.
- **M3 and M4 provably cannot fire.** Every forced-block member is
  `Minimal([One(c)])` with phase 1 `NothingToBlock` after the block, so
  inclusion-minimal and minimum-cardinality coincide and flattening changes
  nothing. M6 has no root witness: 0 corpus positions with `t ≥ 3` AND win-now.
  Three of seven mutations are killable.
- **The registered workload is not runnable.** `configs/gate_v0.toml`'s own
  committed table measures radius 2 at **depth 4** as "> 100 s" — depth 3 is 9.7 s
  there, and revisions 2–5 of this document read that table one column left; the full-width reference on the
  corpus's cheapest branching position at radius 2 depth 3 is **243 363 538 nodes
  in 554.2 s** — one position of thirty-one. Revision 1 attributed the cost to
  gate 10, which runs depth 3 at radius **1** on three fixtures.
- **Inside the forced-block class the argmax restraint is vacuous.** `|argmax| = 1`
  on all four members, so membership IS identity — which D-119 refuses by name —
  and the top entries differ only in the FREE second stone by 2 to 24 eval units,
  the quantity **U2** §5.3 explicitly licenses narrowing to move.

One half of the brief's own suspicion did **not** land, and the reviewer credits
it: `tactical_v0.txt`'s expectations are game facts, not generator output
(`expect cell` over `expect move`, deliberately), so a staged fixture carrying the
same rows is not the D-287/D-295 shape.

### 8.2 THE DIFFERENTIAL GATE — S-E, with a reduced S-C beside it — **SELECTION OPEN** (see the block above)

**The stage under doubt, named** — revision 1 named the defect and never the
stage: **does the staged generator ever drop a cell a proven tactic needs?**

**S-E — per-node survival-set containment. The primary instrument.** At every node where **`can_win_this_turn` is `None` AND** `blocking_covers`
answers `Minimal` — which is **U2** §5.3's FILTERED row and not `blocking_covers` alone —
assert the emitted set **EQUALS** the union of cells over the inclusion-minimal
covers, against a hitting set computed
**independently of the generator**. **Equality, not containment** — **U2** §5.3 says the
filtered row emits the cover union "and nothing below it", and a containment
criterion is preserved by every mutation that OVER-generates, which is §8.1's own
charge against S-C arriving at S-E. Mutation M3 is exactly such a mutation:
phase-1 cover cells are provably a SUBSET of the phase-0 union, so failing to
regenerate can only over-generate, and containment would have let it live — the test-side
exact reference, not `ThreatState`. At every node, assert the emitted set is
non-empty and its tiers disjoint.

This is D-124's own named remedy, quoted in revision 1 and then not implemented:
*"What would help is comparing the engine's per-node candidate set with the
reference's"*. Revision 1 rejected its nearest relative (S-B) as a superset claim
D-124 never made, and priced it at a visibility cost that belonged to **WP-1.1's**
non-goal list, not this WP's — whose test plan already registers a test watching the
generated cell set at both phases (U2-T). D-124's flip clause reads *"flips when that
check lands, which is where the visibility question gets settled"*; S-E settles
it.

**Where S-E observes, and against what — respecified, because revision 3's seam
cannot be built.** A REVIEW-design closed all three of its doors: `pvs` is
`pub(crate)` so `Run` is unreachable from an integration test;
`crates/pistol-solver/tests/common/plans.rs` is another crate's integration-test
module and no `src/` can `use` it; and D-115 forbids widening either to let a test
reach it, while D-129 forbids putting a CORRECTNESS invariant behind
`debug_assertions` — "never a quiet demotion". Revision 4 splits the claim in two
and gives each the mechanism its own class calls for.

- **The expensive half is a test over a PUBLIC generator.** `pistol_search::staged`
  becomes public with one entry point returning the ordered candidate vector and
  its forced count. An integration test in `crates/pistol-search/tests/` walks
  positions with the existing reference walker and, at every one, compares the
  forced prefix against a hitting set computed by an **independent from-scratch
  plan-family implementation in that crate's own `tests/common/`** — the pattern
  rule 7 already names for movegen (`bruteforce.rs`) and the search
  (`reference.rs`). That is NOT D-295's defect: D-295's defect was checking a
  shipped oracle against a test-side oracle DERIVED FROM THE SAME FIXTURE; this
  checks a shipped generator against an independently written referent.
- **The cheap half is an always-on `assert!` in `visit`.** The test above sees what
  the generator EMITS; D-124's reproducer (`cells.pop()` after `order`) is a drop
  AFTER generation, which no test of the generator can see. So `visit` carries a
  named invariant — the first `forced` candidates are searched unless a cutoff or
  an abort intervened — as an `assert!`, not a `debug_assert!`: its violation
  makes the answer wrong, which is D-129's own criterion for the macro. Cost is
  one comparison per node. It is a PRIVATE invariant guard, which is precisely
  what D-115 permits in-source.

Two consequences worth stating. The gate then runs in whatever profile CI uses
without a `release-checked` argument, because an `assert!` survives release —
revision 3's `debug_assertions` seam would have been compiled out of
`tools/search_oracle_check.sh`, `tactical_check.sh`, `determinism.sh` and
`movetime_check.sh`, all of which drive release binaries. And §13's estimate is
re-based: the traversal cost is the test's, once per fixture, not "at every node".

**The guard is not decoration, and revision 6 dropped it.** **U2** §5.3 selects the row
on `can_win_this_turn` FIRST; `blocking_covers` is consulted only in the `None`
arm. A node where the mover can win now AND the opponent holds a coverable hot
window takes the WIN-NOW row and emits the win class — while `blocking_covers`
still answers `Minimal`. MEASURED on the shipped fixture case
`mate_in_1_own_win_beats_blocking_the_opponent`: `can_win_this_turn = Some(OnePly
{ at: (5,0) })` and `blocking_covers = Minimal([One((5,3))])`, so revision 6's S-E
would assert `{(5,0)} == {(5,3)}` and fail a CORRECT implementation on a corpus
position — while passing mutation M5, whose mutant emits exactly `{(5,3)}`. The
instrument was inverted on that class. Revision 5's wording was red on the same
node for the same reason; revision 6 removed the win-now half of the criterion and
did not add the guard that made it well-posed.

It also dissolves three of §8.1's findings at once: no argmax type confusion, no
verdict decided by a free stone's 2–24 eval units, and no comparison across two
universes the adopted Tier-T option makes non-comparable.

**Reduced S-C — a cheap mate-class regression only.** Radius 1, the three fixtures
plus one built mate-in-3 that gate 10 already affords at **MEASURED 17.89 s**. Not
a corpus sweep, not radius 2, not depth 3 at radius 2. Its mate class must state
whether `LossInTurns` roots are included or excluded **by name** — `compact_mated_in_2`
falls in neither class today.

### 8.3 The other three gates, re-scoped — NAMED, NOT LETTERED

**THE LETTERS ARE GONE. B3 IS REPAIRED HERE, BY SHAPE 2 (D-316).** The soundness
gate has FOUR parts and each is named by what it is, so that no part can lose its
definition the way `(b)` did — deleted with revision 2's enumeration and then
wired into CI for five revisions as a letter naming nothing:

| The four gates | Where it is specified |
|---|---|
| **THE TACTICAL SUITE UNDER STAGED** | §8.3 below, first bullet |
| **THE DIFFERENTIAL GATE** — S-E, with the reduced S-C beside it | §8.2 |
| **THE COLONY FAMILY**, ≥ 6 built cases | §8.3 below |
| **THE PATTERN FIXTURES UNDER STAGED** | §8.3 below |

**LEGACY CITATIONS RESOLVE THROUGH THIS TABLE AND ARE NOT ORPHANED**, which is
the one cost shape 2 was charged with: `§8.3(a)` is the tactical suite,
`(b)` is the differential gate and is §8.2's subject, `(c)` is the colony family,
`(d)` is the pattern fixtures. The letters are retired as an ADDRESSING SCHEME;
they are kept here as a lookup so an existing "§8.3(a)" in another unit, in the
superseded document's history or in a landed ADR line still lands on the right
gate. Nothing below is addressed by letter.

- **THE TACTICAL SUITE UNDER STAGED** *(the superseded `(a)`)*. **Revision 5's derivation rested on set
  containment; a negamax value is not monotone in the candidate set, so it is
  redesigned again — this time on where the two searches can differ at all.**

  Revision 5 argued: with the cut disabled the staged set is a SUPERSET of the
  radius policy's, so anything the radius search finds Staged finds. The
  containment half is true and survived attack. The inference is not: enlarging
  the set at the OPPONENT's nodes LOWERS the value, so a superset can refute a
  mate as easily as find one. Revision 5 also claimed none of the fifteen
  `instrument_v0` cases takes a batched row; MEASURED, the phase-1 node
  immediately below each `must_block` root answers `NothingToBlock` with a 48–56
  cell quiet pool against `quiet_top_k = 16`, so the cut binds inside all four.

  **The derivation that does work, and what it costs.** **BOTH staged TACTICAL
  configs disable the quiet cut** (`quiet_top_k` above the whole pool):
  `tactical_staged_v0.toml` for the fifteen `instrument_v0` cases and
  `gate_staged_v0.toml` for the five gate cases. The other two staged documents —
  the SPRT seat and the play config — keep the cut, which is why there are four
  and not three (**U3** §10, the one place that count is stated). *Revision 7
  repaired §10 and §15 item 15 and left this sentence saying "all three staged
  tactical configs … not just the two gate ones", which was false in both halves
  and had no referent for "the two gate ones" — B4 of the revision-7 review, the
  recurring defect reproduced inside the repair that names it. The DERIVATION
  below is unaffected: the twenty cases run at `tactical_staged_v0.toml` (15) and
  `gate_staged_v0.toml` (5), both with the cut disabled, which is what it needs.* Then Staged and the radius policy can differ at a
  node in exactly three ways, and each is a `[PROVEN]` law rather than a
  heuristic:

  0. **The `!is_pv` OVERLOAD RETURN** — a fourth way, and revision 6's "exactly
     three" was wrong to omit it: it GENERATES NOTHING and returns
     `-mate_in(k+2)`, so it is not a generation row at all. **U3** §7 names it as the SPRT's third axis, and **U2** §5.2 MEASURES it changing the
     bestmove on 2 of 24 corpus roots and a non-mate score on 3 of 24. It fires
     inside a `require 20` case: enumerating every P1 turn from the radius-1 ball
     at `mate_in_3_double_three_becomes_double_four` gives **8** resulting P2
     nodes answering `Impossible`, all reached at a null window and therefore
     non-PV. **Its direction argument**: the return replaces a search with a
     verdict that is exact when its premises hold, so it can only shorten a mate
     distance on a position that is already lost — never invent one, since the
     premise includes `can_win_this_turn` being `None` for the side it condemns.
     **Its licence is not yet landed**: 455 177 of 455 201 firings are the
     `t >= 2` / one-stone form, which `LAW-OVERLOAD` does not state, and **U2** item
     22 records the calculus amendment as OWED. Until that lands, `require 20`
     rests partly on theory this project has derived and not yet adopted, and
     this document says so rather than counting it among the `[PROVEN]` three.
  1. **WIN-NOW row** — the emitted set is the argmax set, because nothing beats
     `mate_in(k+1)`. The value is identical to full-width's.
  2. **BATCHED row** — the emitted set is a SUPERSET (Tier T ∪ the whole ball).
     The value can differ, and the direction is not fixed — but it differs only by
     the search seeing MORE, which is the direction the fixture's expectations are
     written in: they are game facts (`expect cell`, `expect mate n`), and its
     header says "A case states only what the game decides".
  3. **FILTERED row** — the emitted set is the cover union. **It is a SUBSET of
     the radius policy's set at radius 2 and NOT at radius 1**, which revision 6
     asserted unconditionally. A cover cell is an empty of an opponent hot window,
     hence within distance 2 of an own stone — inside a radius-2 ball, but able to
     fall outside a radius-1 one. MEASURED at the gate case
     `mate_in_3_double_three_becomes_double_four` (radius 1): of **182** FILTERED
     descendants, **157** have a cover cell outside the ball. MEASURED at
     `must_block_p2_five_in_a_row` (radius 2): **0 of 10**. So at the gate configs
     the two sets INTERSECT rather than nest — and the direction still holds, for
     a different reason: the cells Staged adds are exactly the blocks `LAW-HIT`
     says are the only defence, which the radius-1 policy never offered. Staged
     sees more of the truth there, not less. `LAW-FORCE`
     is `[PROVEN]` that every excluded move LOSES. So a value difference here can
     only be Staged declining a move that loses, and if full-width's answer rested
     on such a move, full-width was wrong about the game and right only about its
     horizon.

  **`require 20` is pre-registered on that**, and on nothing weaker: every case's
  root and every node of its proof line is one of those three, and in each the
  divergence is either nil or in the direction of the game fact the case asserts.
  A failure is a red gate to investigate — and, specifically, evidence that one of
  the three laws is being composed wrongly, which is what makes it worth gating.

  **What this deliberately gives up.** With the cut disabled the tactical suite is
  silent about the quiet prune. That is the right division — the prune is a
  strength knob and rule 6 makes SPRT its judge — but it must be said, because a
  green tactical suite under Staged is then NOT evidence about the cut. The cut is
  judged by the movetime measurement (`WPQ_seed.md`, §12 item 3), by the SPRT,
  and by S-E.

  **And it is an honest weakening from revision 5**, which claimed the suite ran
  fifteen of twenty cases at the committed `quiet_top_k = 16`. It does not.

- **The five gate_v0 cases need a staged config — A CONFIG STATEMENT, NOT A FIFTH GATE**, and it was the unlabelled bullet whose presence beside three lettered ones helped hide that a fourth letter had gone missing. MEASURED: `tactical_v0.txt`
  is 15 cases at `configs/instrument_v0.toml` and **5 at `configs/gate_v0.toml`**
  (radius 1, the `depth_turns 3` cases, because gate_v0's table measures radius 2
  at depth 4 as > 100 s, and depth 3 at 9.7 s). Revision 1 shipped one staged config. Revision 2 shipped three; **U3** §10 states the
  number that ships and is the only place that states it (B5).
- **THE COLONY FAMILY, ≥ 6 built cases** *(the superseded `(c)`)*, distant-cluster attack and defence,
  where `LAW-DECOMP`'s star-disjointness puts the right answer in a cluster the
  delta ranking does not favour.
- **THE PATTERN FIXTURES UNDER STAGED** *(the superseded `(d)`)*, **re-scoped so it is about the stage.** As written it never ran the search:
  D-295 measured the pattern pack's whole contact with `crates/pistol-solver/src`
  as 33 booleans plus four `hot_windows` assertions. Revision 2 runs the **U2** §5
  pattern positions **through the staged generator** and asserts `PAT-GAP`'s
  singleton gap cell is in Tier F — `LAW-HIT`, the singleton plan must be hit.
  Confirmed available from the fixture's own data: PAT-GAP's plans are
  `{-1,0 1,0} {1,0} {1,0 6,0} {6,0 7,0}`, so `{1,0}` is a singleton and every
  minimal cover contains it.

### 8.4 The mutation ledger, with witnesses

S-D's discipline is kept — asserting an instrument's strength rather than
measuring it is D-295's finding one WP earlier — but the ledger is rebuilt,
because three of revision 1's seven mutations could not fire and one had no
witness. **Each mutation names the position it dies on, and where the corpus
cannot produce one it is BUILT** (D-260's precedent and its remedy).

| # | Mutation | Class | Witness |
|---|---|---|---|
| M1 | Tier F drops the pair-completion class | mate | `mate_in_1_two_stones_complete_a_row` (corpus) |
| M2 | Tier F drops `win_in_one_ply_cells` | mate | the **nine** single-stone `mate_in_1` cases (corpus; eleven `mate_in_1` cases in all, two of which are two-stone and belong to M1) |
| M3 | The FILTERED row emits `Cover::cells()` flattened at phase 0 and does not regenerate at phase 1 | S-E | **BUILT, and revision 6's witness was inert under EQUALITY too.** With a single two-cell cover the stale union minus the played cell EQUALS the correct phase-1 set, so nothing separates them. The witness must have a phase-0 union of **three or more** cells: `cover.rs`'s own `{a,b} {b,d} {d,e}` shape, whose union is `{a,b,d,e}` while the phase-1 set after any one cell is strictly smaller |
| M4 | Minimum-cardinality covers instead of inclusion-minimal | S-E | **BUILT, and revision 4's witness was inert.** The shape must have a 1-cover COEXISTING with a minimal 2-cover; `cover.rs`'s flat-list counterexample has no 1-cover, so the two notions coincide there and the mutant is an identity. **REBUILT AT u-rev 3 AS A POSITION A LEGAL GAME REACHES (MAJOR 8).** The witness this row carried until u-rev 2 held P1 = 8 stones with no stone on the origin — MEASURED refused by the rules on three counts at once, so it was a `ThreatState::apply` construction and never a position the SEARCH could be at. The rebuilt witness, with P2 to move: **P1** `(0,0)(1,0)(2,0)(3,0)` and `(-1,1)(-1,2)(-1,3)(-1,4)` and `(0,7)`, **P2** `(-2,0)(5,0)(-1,-1)(-1,6)` and `(4,-4)(5,-4)(-4,4)(-5,5)`. Nine P1 stones and eight P2 stones is rule 3's parity; the two arms share the empty corner `(-1,0)` and each is sealed at both far ends, so exactly one window per arm is hot. **MEASURED by replaying every ply through `GameState` and then querying the shipped solver:** `can_win_this_turn(P2,Two) = None` and `blocking_covers(P2,Two) = Minimal([One((-1,0)), Two{(-1,5),(4,0)}])` — the 1-cover coexisting with the minimal 2-cover, and minimum-cardinality drops the pair. Pinned by `crates/pistol-solver/tests/wp15b_mutation_witnesses.rs` |
| M5 | The WIN-NOW row emits the cover union instead of the win class | **mate** | `mate_in_1_own_win_beats_blocking_the_opponent`. Revision 4 named "own win-now cells dropped from the FILTERED set", a path **U2** §5.3 deleted — on that position `can_win_this_turn` is `Some`, so the node takes the WIN-NOW row and `blocking_covers` is never called |
| M6 | The overload return drops its `can_win_this_turn` guard | **mate**, not S-E | **BUILT, AND REBUILT AT u-rev 3 AS A POSITION A LEGAL GAME REACHES (MAJOR 8).** The shape is unchanged and was never the defect: P1 holds one five-run sealed at one end, so exactly one cell completes it, and P2 holds three disjoint five-runs at rows 8 / 16 / 24 — 8 apart keeps every placement legal under rule 5 and 8 > 5 guarantees no shared window. What was wrong was the COUNT: P2 held 15 stones, and rule 3 gives P2 an even number at every turn boundary. The rebuilt witness, with P1 to move: **P1** `(0,0)(1,0)(2,0)(3,0)(4,0)`, the three seals `(-1,8)(-1,16)(-1,24)`, and seven further stones `(0,4)(3,4)(0,12)(3,12)(0,20)(3,20)(7,4)` placed where no window reaches four — fifteen in all; **P2** the seal `(-1,0)` and the three runs `(q,8)(q,16)(q,24)` for `q` in `0..5` — sixteen. The seven fillers are not decoration: P2's sixteen stones force P1's fifteen, and a witness that cannot be counted to cannot be replayed. **MEASURED by replaying every ply through `GameState` and then querying the shipped solver:** `can_win_this_turn(P1,Two) = Some(OnePly{ at: (5,0) })` while `unblockable_double_threat(P2,Two) = true`. Pinned by `crates/pistol-solver/tests/wp15b_mutation_witnesses.rs`. Its class is mate and not S-E, because the mutant RETURNS rather than emitting and S-E is blind at an `Impossible` node. **The witness is driven as a NON-PV DESCENDANT, never as a root**: the overload return is `!is_pv`-gated and ply 0 is always a PV node, so as a root the mutant does not fire at all and survives. Revision 5 changed this mutation's class and did not re-read the gate it then leaned on |
| M7 | Tier T qualifies at ≥3 for the mover (option A) | informative | survival is a recorded finding under **U3** §6.5's second branch, with a diagnosis, per D-281 |
| **M8** | **`visit` drops the last candidate after generation** — D-124's own reproducer, `if cells.len() > 1 { cells.pop(); }` | **the `assert!`** | **A FILTERED root**, where `forced` is the whole set and `beta = INFINITY` guarantees the loop exhausts. Revision 4 registered no mutation for the `assert!` half at all, while §8.4 opened by quoting D-295's finding that asserting an instrument's strength rather than measuring it is the defect. Registered because the honest reading is that on the 70.8 % BATCHED population `forced == 0` and the assertion is VACUOUS there — it earns its place only on forced rows, and the mutation is what shows which |

### 8.5 Floors, not printed counts

Revision 1 printed the count of positions in neither class "so the gate cannot
silently become vacuous". Printing is not a criterion: the defect it names —
classifying nothing — **preserves** it. Revision 2 registers a per-class floor
that names its witness positions, so a class whose members all disappear turns
the gate RED rather than green with a large number beside it.

### 8.6 REJ-DEPTHPROOF, stated where it belongs

Every claim this instrument makes is bounded-depth with no zone argument and is
therefore EVIDENCE and never PROOF. `REJ-DEPTHPROOF` binds us as it binds the
community. Revision 1 asserted the test plan said this; it contained no occurrence of
`proof`, `evidence` or `DEPTHPROOF`, and §8 cited neither.

### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**

> **THE DEFECT, kept because the wiring below is only legible against it.** B3 of
> the revision-7 review: the superseded wiring sentence read "(a)–(d) plus S-E
> become one script", which named a component with no definition anywhere and
> listed S-E twice. MEASURED: `grep "(b)"` over the superseded document returns
> two hits, §4.2's matrix row `(b) INVERT both as declared lists` and **U2** §5.2's
> citation `D-257 (a)/(b)` — neither is a soundness gate. §8.3 was titled "The
> other three parts" and defined **(a)**, **(c)**, **(d)** plus one unlabelled
> config bullet. Revision 1 DID define it (`ec8f7fb:502`): "(a) tactical suite at
> pre-registered thresholds under Staged; **(b) a differential gate** against …",
> and its matrix was headed `| Option | (b)'s instrument |` — **so S-E *is* gate
> (b)'s instrument**, and "(a)–(d) plus S-E" counted it once as (b) and once as
> itself. Revision 2 deleted the enumeration and the matrix together, and five
> revisions then shipped a CI wiring sentence addressing a gate by a letter that
> named nothing.
>
> **REPAIRED by shape 2 (D-316): the letters are dropped and the four gates are
> named.** The double-list dies with the letters — S-E is not listed beside the
> letters it was one of, because it IS the differential gate and is named once, in
> §8.2. The two-shape comparison the selection was made from stands unedited in
> U4-Z, with the selection recorded beneath it.

**THESE FOUR — the tactical suite under Staged (§8.3), the differential gate
(§8.2: S-E with the reduced S-C beside it), the colony family (§8.3) and the
pattern fixtures under Staged (§8.3) — become one script**,
`tools/staged_soundness_check.sh`, added to `tools/ci.sh`. **Four parts, four
names, each defined in exactly one place, and the script's own enumeration is
this sentence's.** A `tools/` change: reviewed against `tools/SHELL_CHECKLIST.md`
with every item answered by name, carrying the coverage rule's test driving the
shipped script, and distinguishing RUN VOID from FAIL by name (item 12) with a
scratch preflight.

---

## 9. MATRIX M4 — the snapshot's config seam — **SELECTION OPEN**

> ## THE RECOVERY (T1'), THE DIFF, AND WHY THE SELECTION DOES NOT STAND
>
> **The recovery.** B1 found §9 contains no matrix. Unlike M2 and M3, M4's does
> exist and holds the adopted option — MEASURED, `N-A` occurs at `ec8f7fb` and
> the matrix there recommends it — so T1' makes M4 a MECHANICAL RECOVERY. The
> text below the fold is `ec8f7fb:558–578`, recovered verbatim, sha256 prefix
> `5e8c5e4a1e7ad416`.
>
> ### DIFF 1 — recovered text vs. the text the DECISION-RED-TEAM attacked
>
> **IDENTICAL — and identical BY CONSTRUCTION, which is why it is reported and
> not relied on.** The five-matrix DECISION-RED-TEAM round ran against revision 1,
> `ec8f7fb` (superseded §16, row 1). Recovering from `ec8f7fb` and comparing to
> what was attacked at `ec8f7fb` reads one blob twice. **A criterion the defect
> class preserves is not a criterion** — CLAUDE.md's own words — and "did the
> recovery corrupt the bytes" is preserved by every defect that matters here.
> Stated so a reader does not mistake it for a check that passed.
>
> ### DIFF 2 — recovered text vs. an EXTERNALLY DERIVED referent
>
> The referent that does not share the suspect input: **the design's own §9 at
> `6feb40a`**, five amendments produced by measurements taken AFTER the attack, by
> a process that is not the matrix. `diff -u` is non-empty, and three of the four
> differences are MEASURED falsifications of matrix cells:
>
> | # | Recovered cell | The design's §9 says | Mark |
> |---|---|---|---|
> | 1 | N-A failure modes: "the flag is the **fourth of its exact kind**", and "the `argument` helper already refuses an empty value" | Amendment 2: *"The fourth flag of its exact kind" is withdrawn* — `--corpus` reaches the record through `$(basename …)` while `--config` would reach it as a whole path on TWO invariant lines; **four guards are owed and the `argument` helper is none of them** | MEASURED |
> | 2 | N-B failure modes: "**breaks the D-209 instrument golden transcripts**" | Amendment 3: **false** — `grep -c instrument_v0` on that fixture is **0**; the golden is taken at `configs/gate_v0.toml` | MEASURED |
> | 3 | N-A cost: "**MEASURED** one snapshot run costs **34.0 s**" | Amendment 4: N-A **is** a change to the instrument, so BEFORE is re-taken under the amended script — **MEASURED 34.5 s** | MEASURED |
> | 4 | Preamble: the pinned triple `depth_at_500ms` 2 / 2 / 1 "reproduces exactly" — the quantity the matrix is about | Amendment 1: `depth_at_500ms` sits **32 lines below the `# timing` marker** whose text reads *excluded from every comparison*, is demoted to CONTEXT, and the registered quantity becomes per-position `depth_turns` and `nodes` | MEASURED |
>
> Difference 1 removes N-A's mitigation; difference 2 removes a ground on which
> the matrix REJECTED a rival; difference 4 moves the matrix's SUBJECT.
>
> ### VERDICT: **DIFFERS. SELECTION OPEN. The carve does not select N-A.**
>
> **UPDATE AT u-rev 5 — THE FRESH ROUNDS RAN, TWICE, AND STILL SELECT NOTHING
> (D-318).** `docs/experiments/matrix_M4_snapshot_config_seam.md` was authored
> fresh (revision 1, `77f7397`), attacked, re-authored over a ten-option field
> (revision 2, `cb16f7c`) and attacked again. Options SURVIVED both rounds, so
> this is not M3's "every option fell" stop — it stops for three other reasons,
> each sufficient. **(i)** Revision 2's stated reason for existing is FALSE: it
> claimed D-252's matrix selected the document seam and D-283 landed it, and
> **D-288 exists in this tree solely to relabel D-252's option (c) "DEFERRED …
> NO OPTION SELECTED"**, warning against a successor who reads it as adopted and
> finds "the matrix already spent, and no red-team owed, on a choice that had
> never been attacked". **(ii)** Every precedent ground is therefore void — the
> tree holds NO attacked selection for how an instrument binds a per-run input,
> since D-252 selected nothing, D-283 states its own choices were never attacked,
> and D-316's residual says the same of B3's shape comparison — leaving one ground
> that argues equally for five options and discriminates between none.
> **(iii)** The field is STILL incomplete at revision 2: a required
> `--config {instrument|staged}` **closed-enum selector** — rule 1's own mechanism,
> which the recommendation invokes — dominates the recommended option on the
> matrix's own guard trigger and was excluded by the framing "lets a caller name
> the path" for two revisions running.
>
> **So B2 is not discharged and the reason is now measured rather than
> procedural.** No ADR line may cite N-A, N-A′ or N-E as adopted. Item 15 in U4-Z
> stays blocked, and `tools/baseline_snapshot.sh` has no config seam, so the
> registered above-marker quantity has a BEFORE and no AFTER.
>
> T1' is explicit — "identical = attack stands, differs = fresh round". It
> differs. **The fresh DECISION-RED-TEAM is the architect's dispatch, not the
> carve's**, and until it runs no ADR line may cite N-A as adopted — item 15 in
> U4-Z is written on that footing, and B2 (M4 has no ADR line at all) stays open
> beneath it.
>
> <details><summary>THE RECOVERED MATRIX, verbatim from <code>ec8f7fb:558–578</code>
> — a record of what was attacked, NOT a selection</summary>
>
> The brief requires `tools/baseline_snapshot.sh` before and after, against pinned
> operator numbers (`depth_at_500ms` opening 2 / early_mid 2 / late_mid 1 at
> `050961d`). **MEASURED at `f317385` on this machine, BEFORE run: opening 2,
> early_mid 2, late_mid 1 — the pinned triple reproduces exactly.** The script's
> `CONFIG` is a literal, `configs/instrument_v0.toml`, with no flag.
>
> | Option | What it does | Cost | Failure modes |
> |---|---|---|---|
> | **N-A — add `--config PATH`** | A workload-scope flag beside `--corpus`, `--ladder-depth` and `--binary`. The budget line still says `registered`; the record already carries `config <path> <sha>` and `engine_id config <path>` ABOVE the timing marker, so two records taken under two configs are already distinguishable and cannot be diffed as one. | A `tools/` change: SHELL_CHECKLIST answered item by item, plus at least one test driving the shipped script (item 10). **MEASURED** one snapshot run costs 34.0 s wall on this machine. | It reopens a script whose review round closed recently. Mitigated by the shape: the flag is the fourth of its exact kind, the record's invariance claim is untouched because the config path was already inside the invariant block, and the argument parser's `argument` helper already refuses an empty value. |
> | **N-B — flip `configs/instrument_v0.toml` to staged** | No tools change; the standing instrument measures Staged by construction. | Zero. | Lands the strength claim before its judge, against rule 6 and against D-190/D-194's own precedent; breaks the D-209 instrument golden transcripts; fires D-204's flip clause on this session's authority rather than the operator's. Rejected. |
> | **N-C — a scratchpad harness** | Measure Staged with a session-local script. | Zero repository change. | The number would come from an instrument with no governing revision, which CLAUDE.md's instrument clause exists to forbid, and it would not be comparable with the pinned operator triple because it is a different instrument. Rejected. |
> | **N-D — take no Staged snapshot** | Report the radius numbers only. | Zero. | The brief's required measurement is not taken and the WP's whole depth claim goes unmeasured. Rejected. |
>
> **RECOMMENDATION: N-A.** It is the only option that produces the required number
> from the registered instrument. The deltas it yields are ADVISORY per the session
> policy; the operator re-runs on their own hardware for the record.
>
> ---
>
> </details>

### 9.1 The five amendments the design made to N-A after that attack

Carried verbatim. They are what DIFF 2 is measured against, and they are the
reason the selection is open rather than the reason it is closed.


N-A (add `--config PATH`) remains the only option that produces a Staged number
from the registered instrument; N-B (flip the committed config), N-C (a scratchpad
harness) and N-D (measure nothing) are rejected. Five amendments.

**1. The registered quantity changes.** `timing depth_at_500ms` sits at lines
89/93/97 of a 97-line record — **32 lines below the `# timing` marker** whose own
emitted text reads *excluded from every comparison*. Its resolution, MEASURED
from this session's own BEFORE ladder:

| rung | to move UP one unit | to move DOWN one unit |
|---|---|---|
| opening (d2 102 ms, d3 9339 ms) | 18.7× faster | 4.9× slower |
| early_mid (d2 118 ms, d3 1340 ms) | 2.68× faster | 4.24× slower |
| late_mid (d1 30 ms, d2 982 ms) | **1.96× faster** | 16.7× slower |

and the reviewer measured the triple **unchanged at 2 / 2 / 1** under a deliberate
16-way load that stretched the same run from 34.5 s to 66.3 s. The agreement
revision 1 reported as a reproduction is invariant under a ~2× defect in the
quantity it is made of — a criterion its own defect class preserves.

**Revision 2 registers the ABOVE-MARKER quantity**: per-position `depth_turns`
and `nodes` at the registered 50 000-node budget, plus the `ladder … nodes`
counts. That is D-190's own mechanism statistic ("radius 2 completes a second
turn-iteration in 17 of the 24 bench positions"), it is inside the invariant
block, and it is byte-invariant by construction. `depth_at_500ms` is demoted to
below-marker CONTEXT and its dead band is stated so an unmoved triple is not read
as a null result.

**2. "The fourth flag of its exact kind" is withdrawn.** `--corpus` reaches the
record through `$(basename …)`; `--config` would reach it as a whole path on TWO
invariant lines. Four guards are owed and the `argument` helper is none of them:
caller-relative resolution (as `--out` and `--binary` each got), the printable
allow-list extended to the whole `$CONFIG` path, three named refusals
(directory / missing / not a regular file), and an assertion that the script's
`config` line and the engine's `engine_id config` line name the same document.
**SHELL_CHECKLIST items ENGAGED: 1, 3, 4, 8, 9, 10, 11, 12 — eight of twelve**,
answered by name in the IMPL commit.

**3. N-B's rejection loses a cost that does not exist.** "Breaks the D-209
instrument golden transcripts" is **false**: `grep -c instrument_v0` on that
fixture is **0**; the golden is taken at `configs/gate_v0.toml`. The real
exposure is `tactical_v0.txt`'s **15** `instrument_v0`-bound cases under D-204.
The rejection stands on its three surviving grounds — rule 6's judge, the
D-190/D-194 precedent, and D-204's flip being the operator's to fire.

**4. The instrument is named with its revision, and BEFORE is re-taken.** Revision
1 invoked the instrument clause against N-C without satisfying it for N-A:
`tools/baseline_snapshot.sh` was named twice and never with a revision. And N-A
**is** a change to that instrument, so the BEFORE run — taken under the
pre-`--config` script — is re-taken under the amended one. **MEASURED 34.5 s.**
Not worth an argument.

**5. Replicate.** The run is 34.5 s and CLAUDE.md says a cheap doubt is answered
by replication, never by a margin defending a single sample. The below-marker
triple is taken three times.

---


---

### 11.6 One thing this WP does NOT close

D-295's residual — `RULE-EXACT`'s "never derived by weight algebra" is unpinned in
`src`, because no `HitBudget`-shaped fixture separates `t = 3` from `t = 4` — is
not closed here. D-295 names `blocking_covers` as the differently-shaped surface
that could close it, and this WP puts `blocking_covers` on the per-node path, so a
reader will ask. S-E exercises `blocking_covers` for its ANSWERS, not for its
arithmetic's exactness. Registered for WP-1.10 (item 8, **U2**).

---

## U4-T. The tests this unit registers

Carried from the superseded §11. The rows this unit does not own are in U2-T,
U3-T and `WPQ_seed.md`, and no row is in two places.

| Test | Watches |
|---|---|
| `gap_trap_answered_in_tier_f` | that `PAT-GAP`'s singleton gap cell is in the FORCED prefix from the defender's side |
| `colony_family_passes_under_staged` | the move played on ≥ 6 built distant-cluster positions |
| `tactical_suite_holds_at_its_rederived_thresholds_under_staged` | the `require` count of `tactical_staged_v0.txt`, at the **two tactical** staged configs — `tactical_staged_v0.toml` for fifteen cases and `gate_staged_v0.toml` for five |
| `staged_filtered_set_equals_the_minimal_cover_union` | **S-E, half one**, and it asserts EQUALITY, not containment — revision 6 established in §8.2 that containment is exactly what an over-generating mutation preserves, and then left this row registering containment: the public generator's forced prefix against an independently written plan-family referent in pistol-search's own test tree |
| `visit_searches_every_forced_candidate` | **S-E, half two**: the always-on `assert!` in `visit`, which is what sees a drop made AFTER generation — D-124's own reproducer |

---

## U4-M. What this unit measures

ADVISORY on this machine; the operator re-runs for the record. (A standing
condition of every measurement in every unit, stated per unit so a unit is
readable alone; it is a condition, not a datum.)

1. **Snapshot before / after**, both under the amended script. **Registered
   quantity: per-position `depth_turns` and `nodes` at 50 000 nodes** (above the
   marker). `depth_at_500ms` reported as context with its dead band.

The BEFORE numbers taken at `f317385` reproduce the pinned operator triple
(`depth_at_500ms` 2 / 2 / 1) — **and §9 amendment 1 establishes that the triple
sits BELOW the record's own "excluded from every comparison" marker with a dead
band of about 2×, so it is context and not the registered quantity.** No AFTER
exists, because no engine code was written.

### Cost

| Item | DECLARED | MEASURED |
|---|---|---|
| One baseline snapshot | ~35 s | **34.0 / 34.5 s** |
| The soundness gate per CI run | **ESTIMATED 40–90 s**, dominated by S-E's one traversal per fixture plus the reduced S-C's **MEASURED 17.89 s**. Revision 1's 60–180 s priced a workload that is days | to be MEASURED when it lands, and reconciled here |

**The proportionality clause, and what it now says here.** The snapshot run is
34.5 s, so a doubt about it is answered by REPLICATION and by a SECOND INSTRUMENT
whose agreement criterion is registered before either runs — never by a margin
derived to defend one sample. §9 amendment 5 registers the replication (the
below-marker triple is taken three times). **It registers no second instrument,
no agreement criterion, no stage under doubt and no consequence for
disagreement** — U4-Z, OPEN. (D-307 rules WP-1.5b's SPRT run EXPENSIVE and lapses
the replication clause *for that run*; it does not reach this 34.5 s one, and
D-307 keeps the second-instrument duty in either case.)

---

## U4-Z. ADR lines this unit owes, the OPEN decision it stops on, and what is OPEN

### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316).

**THE TWO-SHAPE COMPARISON BELOW IS THE TEXT THE ARCHITECT SELECTED FROM AND IS
LEFT UNEDITED**, on the same discipline the restructure matrix landed under: a
comparison corrected after the decision it fed is a comparison the decision was
never made against. Its "the carve does not choose" is the state AT SELECTION
TIME. The selection, and one MEASURED correction to a cost cell that execution
falsified, are recorded AFTER it, not inside it.

*(At selection time:)* The carve does not choose because both shapes are
coherent, and choosing between them is a design act.

**SHAPE 1 — RESTORE THE ENUMERATION, and put S-E inside (b).**
Reinstate revision 1's four-part bar verbatim from `ec8f7fb:502` — "(a) tactical
suite at pre-registered thresholds under Staged; **(b) a differential gate**
against full-width r2 at depths 1..=3 for mates and forced blocks; (c) a colony
fixture family of ≥ 6 cases; (d) the INTEG **U2** §5 pattern fixtures under Staged" —
and state that S-E, with the reduced S-C beside it, **is (b)'s instrument**, which
is what revision 1's own matrix heading said (`| Option | (b)'s instrument |`).
§8.7's wiring then reads "(a)–(d) become one script", which is revision 1's
sentence unchanged, and the double-list is gone because S-E is no longer listed
beside the letters it is one of.
*Cost, MEASURED:* four sentences reinstated; **zero** cross-references outside §8
change; §8.3's title "The other three parts" becomes true again, since (a), (c)
and (d) are the other three and (b) is §8.2's subject.
*What it keeps:* a lettering scheme that has already lost its own definition once,
in a document that then shipped four revisions without noticing.

**SHAPE 2 — DROP THE LETTERS, name the four gates by what they are.**
The gate is: the tactical suite under Staged; the differential half (S-E plus the
reduced S-C); the colony family; the pattern fixtures under Staged. §8.7 reads
"these four become one script". No component can go undefined, because none is
named by a letter that a later edit can delete.
*Cost, MEASURED:* **three** cross-references outside §8 retarget — `configs/tactical_staged_v0.toml`'s
"why" cell and §10's B5 paragraph, both **U3** §10, and item 15 in this unit's
list, which says "gate (a)". Inside §8, `(a)`, `(c)` and `(d)` become named
bullets.
*What it costs:* "§8.3(a)" is cited from three units and from the superseded
document's history; after shape 2 those citations name nothing.

**Neither shape is a repair the carve may make**, because each decides how the
gate is addressed from the other units, and the count of retargets is not the
argument — the argument is whether a lettered enumeration is worth having at all.

#### SELECTION — SHAPE 2, by architect ruling. Landed at u-rev 2. ADR line D-316.

**Selected: SHAPE 2.** The letters are dropped; the four gates are named by what
they are; §8.7's wiring enumerates the four names; the S-E double-list dies with
the letters, because S-E is the differential gate and is named once, in §8.2.
Executed in this unit at §8.2, §8.3 and §8.7, and in **U3** §10 at the two sites
named below.

**The ground, which is the option statement's own last clause read the way it
asks to be read:** the argument is not the retarget count, it is whether a
lettered enumeration is worth having. It measurably was not. `(b)` lost its
definition when revision 2 deleted the enumeration, and the document then shipped
FIVE further revisions wiring `(b)` into CI as a letter naming nothing, through
five REVIEW-design rounds that did not catch it. Shape 1 reinstates exactly the
scheme that failed that way and its own cost cell concedes the point — *"a
lettering scheme that has already lost its own definition once, in a document
that then shipped four revisions without noticing."* A name cannot go undefined
while remaining in the sentence; a letter can, and did.

**MEASURED CORRECTION TO SHAPE 2's COST CELL, recorded because the cell was wrong
in the direction that favoured the option selected.** The cell says *three*
cross-references outside §8 retarget. Executing it, the count is **SIX**:

| # | Site | Was |
|---|---|---|
| 1 | **U3** §10, `configs/tactical_staged_v0.toml`'s "why" cell | "what **U4** §8.3(a)'s derivation requires" |
| 2 | **U3** §10, the B5 paragraph | "Revision 6's **U4** §8.3(a) said …" |
| 3 | This unit's §15 item 15 | "so gate (a) tests the threat mechanisms" |
| 4 | This unit's head, the B4 sentence | "**B4** (§8.3(a)'s …)" |
| 5 | This unit's U4-A lineage row | "§8.3(a)'s derivation was redesigned three times" |
| 6 | This unit's U4-Z lead-in | "item 15 on §8.3(a)'s derivation" |

Sites 4–6 are outside §8 and inside this unit, and the cell counted only what was
outside the unit. **The correction does not move the selection** — the cell's own
sentence is that the count is not the argument — but it is recorded rather than
quietly fixed, because a cost cell understating the selected option's cost is the
defect class this work package has been failing on, and the reviewer of this unit
should see it stated rather than discover it.

**The stated cost of shape 2 is DISCHARGED, not paid.** The cell's charge was
that "§8.3(a)" cited from elsewhere would after shape 2 "name nothing". §8.3 now
opens with a lookup table mapping each retired letter to its gate, so every
legacy citation — in another unit, in the superseded document's history, or in a
landed ADR line — still resolves. The letters are retired as an ADDRESSING
SCHEME, not erased as a HISTORY.

**THE RESIDUAL, NAMED, AND IT IS THE ARCHITECT'S.** This selection was made by
architect ruling on the comparison above. That comparison states both options,
both costs MEASURED, and each option's failure mode, but it carries no
recommendation and **it was never put to a fresh-context DECISION-RED-TEAM**.
CLAUDE.md's Process section wants a named design decision with more than one
viable option settled by an attacked matrix, and this one was not attacked. It is
recorded here and in D-316 rather than left implicit, because an unattacked
selection that nobody writes down is the silent drift the rule exists to refuse,
and an unattacked selection that IS written down is a debt the architect can
choose to pay or to accept. The reviewer of this unit is not asked to ratify it.

### ADR lines

Carried from the superseded §15. Its item numbers are retained exactly so an
existing cross-reference to "§15 item n" still resolves; this unit invents none
and renumbers none. The superseded §15's preamble does not travel (MAJOR 10
measured it false on both clauses); this is U4's lead-in instead: **both items
below are this unit's own, neither has landed, and BOTH are blocked on a
selection that is OPEN** — item 4 on M3's fresh matrix, item 15 on the tactical-suite gate's
derivation (§8.3, the superseded `(a)`), which stands on §10's four configs and on B3's unresolved wiring.

4. S-E, and D-124's flip clause discharged. Its seam is the PAIR of §8.2 — a
   public generator driven by a test in pistol-search's own tree against an
   independently written referent, plus an always-on `assert!` in `visit` for the
   drop a generator test cannot see. Revision 4's line still registered the
   `#[cfg(debug_assertions)]` observer that §8.2 had withdrawn two sections
   earlier — the un-re-read claim, inside the ADR list itself.

15. **The two TACTICAL staged configs disable the quiet cut** — which needs a
    FOURTH config, `tactical_staged_v0.toml`, because the SPRT seat must keep it —
    so THE TACTICAL SUITE gate (the superseded `(a)`) tests the threat mechanisms rather than the prune, and the prune is judged by SPRT, by the
    movetime measurement and by S-E. The line records what a green tactical suite
    under Staged does NOT evidence.

**AND B2 IS NOT DISCHARGED HERE.** The revision-7 review found **M4 has no ADR
line at all**: §15 contains zero occurrences of `N-A`, `baseline_snapshot`,
`--config` or `snapshot`, while §9 adopts N-A, changes a shipped `tools/`
instrument and changes the registered quantity of the snapshot. Rule 10 requires
one line; the Process section requires it to record the strongest surviving
attack. **The carve cannot write that line, because §9's selection is OPEN and a
line citing an unattacked selection is the breach it exists to prevent.** B2 is
answered by the fresh round, not by the carve. Travelling item T2 is therefore
carried, not closed.

### OPEN — carried forward, not closed by the carve

- **B1 / M3 — the matrix was authored and attacked, and EVERY OPTION FELL** (the stub at the head of §8; D-317). No instrument is selected for the DIFFERENTIAL GATE, so `tools/staged_soundness_check.sh` cannot be specified for that gate and the other three named gates are unaffected. A SECOND AUTHORING ROUND is owed, over a field that must include the four rows the red team named as missing — the S-E-plus-reduced-S-C composite the design actually writes as adopted, a calculus-lower-bound containment criterion, a per-node census, and reuse of the landed referent at `crates/pistol-solver/tests/common/reference.rs:223`. Two of those four are immune to the attack that killed S-E. **It is the architect's to schedule, not this session's to author**: a revision 2 written by the session that wrote revision 1, over options the red team supplied, is the pattern D-305 measures.
- **B2 / M4 — no ADR line, and after TWO authored revisions and TWO fresh-context DECISION-RED-TEAMs the selection is STILL OPEN** (the block at the head of §9; D-318). It is not open for want of a round; it is open because every precedent ground the recommendation rested on turned out to be unattacked or misread, and because the field was measurably incomplete at both revisions. A THIRD round is owed and is the architect's: it must carry the closed-enum selector and the corpus-fixture binding, drop the "caller names a path" framing that excluded them twice, and rest on no precedent that has not itself been attacked.
- ~~**B3 — gate (b), the two shapes above.**~~ **CLOSED at u-rev 2** by the architect's selection of shape 2, recorded above and in D-316. Its RESIDUAL is not closed and is named there: the selection was not put to a fresh-context DECISION-RED-TEAM.
- ~~**MAJOR 8 — M4's and M6's mutation witnesses are not positions a legal game
  reaches.**~~ **CLOSED at u-rev 3.** Both witnesses are rebuilt in §8.4 as
  positions reached by replaying every ply through `GameState`, which is the
  referee rule 2 names, and both are pinned by
  `crates/pistol-solver/tests/wp15b_mutation_witnesses.rs`. The pin is not
  vacuous: MEASURED, the superseded M4 witness is REFUSED by that replay on three
  independent counts — P1 holds an even 8, its first stone is not the origin, and
  P2's 4 is neither one more nor one fewer than 8. §8.4's old "VERIFIED on the
  shipped solver" is replaced by a verification that goes through the rules
  first, which is the distinction MAJOR 8 drew and the reason the old claim was
  true and worthless. **THE RESIDUAL IS NAMED AND IS NOT CLOSED:** a legal
  position is not yet a position the mutation DIES on in the SEARCH — that needs
  the search, and the search is not built (`crates/pistol-search/src/staged.rs`
  does not exist). What is discharged is the reachability half, which was the
  half MAJOR 8 raised; the ledger's "dies here" claim stays owed to IMPL.
- **The snapshot's SECOND INSTRUMENT is unregistered** (U4-M). Replication is
  registered; the second instrument, its agreement criterion, the stage under
  doubt, how the second instrument does not share that stage, and the registered
  consequence of disagreement are not.
- **The `tools/` changes this unit implies have had no SHELL_CHECKLIST review.**
  §8.7's `staged_soundness_check.sh` is a new script; §9's `--config` reopens
  `tools/baseline_snapshot.sh` with **eight of twelve** items ENGAGED by its own
  count. Both reviews are owed at IMPL, and the coverage rule binds each.
- **No REVIEW-design has run against this text at this u-rev** (U4-A).

---

*U4, u-rev 5. A carve, not a revision. BOTH selections remain OPEN and both are now STOPPED rather than pending: M3 attacked once, every option fell (D-317); M4 attacked twice over two authored fields, options survived but no ground discriminates and the field is still incomplete (D-318). B3 CLOSED at u-rev 2, MAJOR 8's reachability half CLOSED at u-rev 3. IMPL has not started.*
