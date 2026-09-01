# WP-2.0b — census position identity on the wire, gated. DESIGN, revision 8.

> **ONE LINE FOR THE MORNING.** The package is designable and its diff is small; the
> sweep it unblocks is not, because the census cannot fire on the seat the sweep runs
> and arming that seat costs a MEASURED 14.5x-36x — but the decision that matters is
> the solver CAP, not the gate: measured at both, the cap does not change the price,
> and it splits D-537's own quantity — **~2x more distinct positions at the small cap,
> and the sample's only win-proof at the large one, which the small cap could not have
> found** — so the cap is NOT settled here, and §1.1 names the dedicated run that would
> settle it and why it cannot be a corpus tranche.

Governing revision `a56449b`. Governing dispatch: **WP-2.0b v2**, transcribed at
`docs/experiments/wp20_dispatches.md`. **EIGHT fresh-context reviews** stand behind
this document and are in the repository beside it: `wp20b_decision_REDTEAM.md` (the
DECISION-RED-TEAM on §2 and §3) and `wp20b_design_rev{2,3,4,5,6,7,8}_REVIEW.md`.
Findings are cited where a claim here turns on one; the reports are the record of the
arc and this document does not restate them. **Revision 8 took the rev-8 review, which
returned FALLS on two findings both OUTSIDE §§2-8 — a byte-identity extraction rule that
could not be satisfied by any correct build, and an unsound sizing for the calibration
run — and both are repaired above (§9, §1.1). Five findings are carried into
implementation rather than into a ninth revision:
`docs/experiments/wp20b_IMPL_FINDINGS.md`.**

---

## 0. What this package is

D-539 split the census logging flag out of WP-2.0 and gave this package **two**
obligations: put census output behind a token no committed config sets, AND give a
census row the position identity D-537's *"win-proving firings on DISJOINT
POSITIONS"* needs. *"Landing only the first would ship a flag that cannot answer the
question it exists to ask."*

§1 adds three findings about the ground the dispatch stands on, each verified at a
quoted site, because two of them delete work the dispatch asks for and the third
prices a decision no document in this repository had priced.

**Out of scope:** any detector logic, any ranking, any label work, any
committed-config change, any strength claim.

---

## 1. THREE PREMISE FINDINGS

### F1 — the dependency route needs no manifest change, and the dispatch's scope item 3 dissolves

The dispatch directs that *"the workspace-shape test is UPDATED deliberately, not
deleted"*, descending from D-539's *"the census is unreachable from the line protocol
because `crates/pistol-cli` does not depend on `pistol-search`"*.

**The dependency claim is true and the reachability conclusion is false.** The route
exists, is in daily use, and carries a `pistol-search` type to the protocol printer
today:

```
crates/pistol-engine/src/lib.rs:47   // The search's reporting vocabulary, re-exported because `go` hands it out.
crates/pistol-engine/src/lib.rs:48   // A caller of this seam should not have to depend on pistol-search to read what
crates/pistol-engine/src/lib.rs:49   // the seam returns (CLAUDE.md rule 11).
crates/pistol-engine/src/lib.rs:51   pub use pistol_search::{
crates/pistol-cli/src/report.rs:2    use pistol_engine::{EngineError, ScoreKind, SearchInfo, classify};
```

`SearchInfo` is defined in `crates/pistol-search/src/info.rs`. pistol-cli reads it by
name, through pistol-engine, and takes no pistol-search dependency to do it. **A
census row travels the same seam for the same reason, and the manifest does not move.**

`crates/pistol-cli/tests/workspace_shape_tests.rs::pistol_cli_manifest_names_only_core_and_engine`
is **not updated**, and that is the finding rather than an omission: both of its
assertions read `Cargo.toml` and never source, so it stays green unchanged — which is
the evidence that this package did not reach past the seam. Rule 11 is honoured in its
own words, *"the `Engine` trait + line protocol ARE the contract"*.

**The correction to D-539 is D-564**, because a false sentence left in the ADR log
would send a successor to widen a dependency ledger that nothing needs widened.

### F2 — both identity forms the dispatch names are already forbidden

The dispatch's scope 1 offers *"full-turn 128-bit key per D-8, or canonical move-list
prefix per D-6"*. `docs/experiments/wp20s_design.md` §8 — landed, reviewed, and the
document that registers D-537's counting rule — has already ruled the denominator and
named this package:

> *"**WHAT COUNTS AS DISJOINT IS `key_full`.** … Two firings on one position reached
> by two move orders are one position, which `key_pos` folds; two firings on a
> position and its mirror image are also one position for a detector's purposes …
> and that fold is `key_seq`'s alone. **`key_full` is the only key with both** …
> **WP-2.0b's identity form must be consistent with it** — that package chooses
> *"the full-turn 128-bit key per D-8, or the canonical move-list prefix"* …, and
> **neither of those is `key_full`**, so this is a constraint that package inherits
> and not an observation this one makes."*

Mapped onto `wp20s_design.md` §2.1's three keys: option 1 **is** `key_pos`
(`GameState::key`, folds transpositions, not symmetries); option 2 **is** `key_seq`
(`canonical_sequence`, folds symmetries, not transpositions). **The option field is
empty under the constraint it inherits**, and the real decision is *how `key_full` is
carried*. That is §2.

**v2's added citation makes its option 2 worse, not better.** Under D-6 — the move
list as the canonical position encoding — option 2 reads as the bare play sequence,
which folds **neither** equivalence, where the `canonical_sequence` reading at least
folds symmetries. F2's conclusion holds under both readings.

**Why the bias runs the wrong way, which is what makes this blocking rather than
tidy.** Either named option counts one position as two whenever the other fold would
have merged them, so the census over-reports disjoint positions. D-537's minimum is a
FLOOR existing *"to stop a successor opening round 3 on thin evidence"*. An identity
that over-counts clears that floor early — the failure in the direction the rule was
written to prevent.

### F3 — the census cannot fire on the seat the sweep runs, and arming that seat is expensive

A census row is pushed at exactly two sites, and both are unreachable unless the
solver is armed on the search path:

```
crates/pistol-search/src/pvs.rs:602     let cap = self.solver.as_ref()?.1.per_call_node_cap;
crates/pistol-search/src/search.rs:287  if let Some(wiring) = self.params.solver
crates/pistol-search/src/search.rs:288      && self.solver.is_some()
crates/pistol-engine/src/instance.rs:150 fn solver_wiring(section: &crate::config::SolverSection) -> Option<pistol_search::SolverWiring> {
crates/pistol-engine/src/instance.rs:151     if !section.on_search_path {
```

**The gate is `false` on the seat the sweep runs** — `configs/instrument_v0.toml:113`,
the config **both pilot seats ran** and the one D-560's whole cost model is
extrapolated from. **It is NOT false in every committed config**: three of the
eighteen carrying the key arm it —
`configs/bench_wp18c_solver_on.toml:45`, `configs/gate_staged_solver_v0.toml:47`,
`configs/play_staged_solver_v0.toml:75` — the last of which already carries the
correction in a committed comment, that D-441's *"gate OFF in every committed config"*
*"binds what pistol SHIPS"* and the measurement seats are exempt.

**So the token is the second of two switches.** WP-2.1's registration says *"Census: ON
via the WP-2.0b token"* and *"D-537's clock starts at tranche one"*. With the labelling
seat's gate off, the token turns on a recorder with nothing to record: **zero firings,
zero rows, zero win-proving firings, for the whole sweep.** The clock would not start
late; it would not start.

**WHY THIS PACKAGE CANNOT FIX IT AND MUST NOT TRY.** Arming the labelling seat is not
a logging decision:

1. **It changes the labels, and by more than a provenance column.** MEASURED at
   `nodes 400000` on identical positions, the gate-on search receives **0.8 %–10 %**
   of its own node budget while the solver absorbs the rest, so the label's DEPTH
   collapses. The provenance point is also true — D-544's fix round broke on it:
   *"`Provenance` has **four** variants and the search-path solver accrues
   `solver_nodes` on answers returning `CompletedDepth`"*.
2. **It breaks D-560's arithmetic by a MEASURED factor.** Same binary, same fixtures,
   `nodes 400000`, cap 2048, gate on against gate off — reproducible through
   `crates/pistol-search/examples/trigger_census.rs`, which arms its own wiring
   independent of any config:

   | fixture | gate off | gate on | ratio |
   |---|---|---|---|
   | corpus/regression, 3 entries | 3.63 s | 52.75 s | **14.5x** |
   | trigger-rich, 3 entries | 4.11 s | 147.99 s | **36.0x** |

   D-560's ~63 h book ceiling becomes **~900 h to ~2 300 h — 38 to 95 days,
   single-threaded**. That is what makes this a STOP rather than a preference.
3. **D-534 is adjacent and settles nothing.** Its 725 ms overshoot blocks a
   **play-config** arming under a **movetime** budget; the label budget is
   `nodes 400000`. It neither forbids nor permits this.

**THE OPERATOR IS CHOOSING A CAP AS WELL AS A GATE, AND THE TWO ARE NOT THE SAME
DECISION.** The ratios above were measured at **cap 2048**. The seats differ:

```
configs/instrument_v0.toml:113-114        on_search_path = false   per_call_node_cap = 16384
configs/bench_wp18c_solver_on.toml:45-46  on_search_path = true    per_call_node_cap = 2048
configs/gate_staged_solver_v0.toml:47-48  on_search_path = true    per_call_node_cap = 512
```

`instrument_v0.toml` and `gate_staged_solver_v0.toml` differ in **exactly those two
keys**, which is what makes arming a choice among existing shapes. But a one-key
arming of the labelling seat leaves cap **16384**, eight times the measured cap, and
the cap is not free in either direction: D-530 records that *"THE ONE ROW THAT
CONSTRAINS A CAP IS `g001-t42-p2` AT cap >= 16384 — at 4096 both directions return
`unknown` after 8,192 visits"*, so a larger cap buys proofs and costs time. **The
magnitude at 16384 is MEASURED in §1.1 rather than extrapolated**, because the same
two commands that produced the table above answer it and D-291 makes an unmeasured
estimate of a measurable quantity a finding.

**What this design does with F3:** records it, builds the identity and the token so
they are correct whichever way it is ruled, and names it in §10.1 as the question that
must be answered before WP-2.1's registration can claim a census. **The code is the
same under either ruling**, which is why the finding does not block this package's
implementation and does block the sweep's claim.

**And F3 reaches this document's own obligations.** If the census cannot fire on a
gate-off seat, every check registered on a gate-off seat is a check that cannot see a
census row — `docs/process.md`'s named vacuity, *"a criterion that is a property the
named defect class PRESERVES"*, and D-527's defect class besides. §8 names the seat for
every test that needs a firing; §9 registers its bench on the armed seat.

### 1.1 THE CAP IS NOT A LEVER ON COST, AND ON THE QUANTITY D-537 COUNTS IT IS WORTH ABOUT 2x — MEASURED

F3's ratios were taken at **cap 2048**. A one-key arming of the labelling seat leaves
cap **16384**. D-465 measured the cap *"dead as a lever"* on the cost ratio and
**nobody had asked what it does to the thing the census exists to collect**, so this
revision measured it rather than extrapolating (D-291).

**METHOD.** `crates/pistol-search/examples/trigger_census.rs` at `a56449b`, release
build, `--nodes 400000 --gate on`, the first three entries of each of the two hardest
committed fixtures (`bench_solver_positions_v1.txt`, `bench_positions_v1.txt`), cap
2048 against cap 16384, everything else identical. Raw outputs exported at
`artifacts/wp20b_cap_*` with a digest receipt; the corrected summary and its derivation
are `artifacts/wp20b_cap_SUMMARY.txt`.

| fixture | cap | wall | raw firings | **distinct** | **win-proofs** | loss-proofs | solver_nodes |
|---|---|---|---|---|---|---|---|
| trigger-rich, 3 entries | OFF | 4 s | 0 | 0 | 0 | 0 | 0 |
| trigger-rich, 3 entries | 2 048 | 149 s | 294 | **49** | **0** | 4 | 1 160 027 |
| trigger-rich, 3 entries | 16 384 | 125 s | 41 | **25** | **1** | 0 | 1 233 841 |
| corpus, 3 entries | OFF | 3 s | 0 | 0 | 0 | 0 | 0 |
| corpus, 3 entries | 2 048 | 53 s | 400 | **26** | **0** | 0 | 1 148 739 |
| corpus, 3 entries | 16 384 | 50 s | 63 | **12** | **0** | 0 | 1 236 133 |

**THE TWO PROOF COLUMNS ARE NEVER SUMMED.** The rule is D-522's and **D-535 supersedes
D-522 while keeping exactly this part of it**: *"What survives from D-522 is the
distinction itself: the two directions are different quantities, they are reported
separately, and a row's LOSS-side recall is not evidence about its WIN-side recall or
the reverse."* **What D-535 retired is "wins-only" as the GATE's direction** — the
census gate ranks both — **and that is a different question from this one**: D-537's
minimum is stated in its own words as *"WIN-PROVING firings on disjoint positions"*, so
for the CLOCK the attacker direction is what counts, while for the detector's RANKING
both do. **Win-proof** is therefore the ATTACKER direction, by the shipped definition
`tools/stage3_allocator_bound.py:133`, `won(row) = att_proved == "true"`.

**`distinct`** counts distinct `TriggerColumns` signatures — the row with the
`attacker_*`/`defender_*` answer fields removed, because those are cap-dependent
OUTCOMES and not descriptors of the position at the decision, a split
`crates/pistol-search/src/census.rs:41-58` already makes.

**THREE LIMBS, AND THE THIRD IS THE ONE THAT MATTERS.**

1. **The price is flat in the cap, and slightly lower at 16384.** Wall time 149 s ->
   125 s and 53 s -> 50 s; `solver_nodes` +6.4 % and +7.6 % — the solver absorbs the
   budget either way. This CONFIRMS D-465 on its own quantity and settles the question
   revision 4 wanted to extrapolate: **F3's ratios are not an under-statement at the
   cap a one-key arming produces.** The cap-2048 rows also come back at F3's own
   figures (149 s against 147.99 s, 53 s against 52.75 s). **That is a REPLICATION and
   not a second instrument, and the difference is not cosmetic**: it is the same
   example binary at the same revision over the same fixtures with the same
   hand-rolled `SolverWiring` (`trigger_census.rs:125-126`), which bypasses the config
   path — the stage actually under doubt. `docs/process.md` is explicit that *"two
   instruments blind to the same stage are one instrument reported twice"*, and this is
   that. It raises confidence in the arithmetic and none in the wiring.
2. **Raw firings collapse — 294 -> 41 (7.17x) and 400 -> 63 (6.35x)** — because a call
   permitted eight times the visits fits into the budget eight times less often.
3. **BUT D-537 DOES NOT COUNT FIRINGS.** Its minimum is counted in *"WIN-PROVING
   FIRINGS ON DISJOINT POSITIONS, **not in firings** and not in games"*, and on the
   closest quantity this data can reach the collapse is **about 2x, not about 7x**:
   distinct signatures fall **49 -> 25 (1.96x)** and **26 -> 12 (2.17x)**. At cap 2048
   only **16.7 %** and **6.5 %** of firings carry a distinct signature — which is the
   in-tree duplication §2 buys C2 to fold, appearing here as a measurement.
4. **AND THE NUMERATOR RUNS THE OTHER WAY.** D-537 counts win-**proving** firings, and
   split by direction as D-522 requires, **the sample's only win-direction proof is at
   the LARGE cap**: 0 win-proofs in 694 firings at cap 2048, 1 in 104 at cap 16384.
   That is n = 1 and carries no rate — but it points AWAY from the small cap, and it
   agrees with D-530's independent mechanism: *"THE ONE ROW THAT CONSTRAINS A CAP IS
   `g001-t42-p2` AT cap >= 16384 — at 4096 both directions return `unknown` after 8,192
   visits"*. A larger cap buys proofs; a smaller one buys distinct positions.

5. **AND THE CAP MOVES A THIRD AXIS THIS SECTION PRICED ON TWO — IT CHANGES THE
   LABEL.** `search_nodes` sits one field left of the `firings` summed above, in the
   same six exported files, and it is the share of its own budget the SEARCH received:

   | fixture | cap 2 048 | cap 16 384 |
   |---|---|---|
   | trigger-rich, per entry | 3.90 %, 1.36 %, 6.15 % | 3.72 %, 1.18 %, 1.73 % |
   | corpus, per entry | 3.47 %, **10.03 %**, 0.78 % | 0.94 %, 0.53 %, 0.27 % |

   **F3's headline *"the search gets 0.8 %-10 %"* is the cap-2048 range**, and at 16384
   it is **0.27 %-3.72 %** — one position moves 40 162 search nodes to 2 123, a factor
   of **18.9**. So the cap does not only change the price and the yield: **it changes
   what a label IS**, because the label is the answer of a search that saw that share of
   its budget. F3's limb 1 already makes the depth collapse the operator's concern; this
   is the same axis, unread until now, and it is read here in the artifacts this section
   already exported.

**AND THE ONE WIN-PROOF COULD NOT HAVE BEEN FOUND AT THE SMALL CAP**, which is the
strongest single fact against the withdrawn recommendation and is stated rather than
left in the file: the winning row carries **`att_visits 11040`**, above cap 2048. It is
not that the small cap searched and missed; it is that the small cap forecloses that
proof structurally — D-530's mechanism, appearing in this arc's own data.

**SO THIS MEASUREMENT DOES NOT SETTLE THE CAP, AND SAYING SO IS THE FINDING.** D-537's
quantity is a **numerator over a denominator** — win-proving firings, on disjoint
positions — and the two limbs point in opposite directions:

- **the DENOMINATOR favours the small cap**, by a MEASURED ~2x on distinct signatures,
  at no extra price;
- **the NUMERATOR's only observation favours the large cap**, at n = 1, corroborated by
  D-530's mechanism but not by a rate.

**An earlier revision of this section recommended the small cap. That recommendation is
WITHDRAWN**, because it rested on a proof count that summed the two solver directions —
the sum D-522 forbids — and under the one definition that counts, the small cap's
proof column is **zero**.

**WHAT WOULD SETTLE IT — A DEDICATED CALIBRATION RUN, WHOSE SIZE THIS SECTION DOES NOT
REGISTER.** An earlier revision proposed that *"tranche one runs both caps"* and then
sized a calibration run at *"about 40 minutes"*. **Both are withdrawn.** The tranche
proposal dies on limb 5: a tranche split across two caps ships a production corpus
**labelled by two instruments**, whose searches saw 0.27 %-3.72 % and 0.78 %-10.03 % of
their budgets. **So the comparison must be a DEDICATED run whose records are excluded
from the corpus by construction** — that part stands.

**The 40-minute figure was wrong and is not replaced with another number here**, because
it was wrong in a way worth naming rather than patching: it applied §2's **116 firings
per ask** — a cap-2048 measurement — to **both** arms, two paragraphs after limb 2
measured that very rate collapsing 6.3x-7.2x at the larger cap. The large-cap arm needs
several times the asks the small-cap arm does for the same firing count, and no position
source was named for either.

**WHAT THIS SECTION REGISTERS INSTEAD IS THE INPUTS AND THE SHAPE, and leaves the sizing
to whoever runs it**, which is the honest division given that this design has no
governed budget for a run of its own:

- **the arms**: caps **2048 and 16384**, the pair measured here;
- **the quantity**: win-direction proofs (`att_proved`) on distinct keys, once §2's
  identity exists — which is why this run belongs AFTER this package and not before it;
- **the inputs, all MEASURED and all in this section**: the pooled win-proof rate of
  **1 in 798 firings**, and firings per ask of **~116 at cap 2048** against **~17 at cap
  16384** (694 and 104 firings over six searches each);
- **the constraint**: each arm sized on **its own** firing rate, and the records
  excluded from the corpus.

**And the sizing must be honest about what it can buy**: at a pooled rate near 0.1 %,
a run large enough to separate two rates differing by less than about a factor of two is
a substantial one, and a small-cap arm that returns no events bounds its rate from above
rather than estimating it. **That is still the answer the operator currently lacks, and
it is cheap relative to a 900-2 300 hour sweep** — which is the comparison that matters.

**WHY THE PROXY IS COARSE, AND THE REASON IS THIS PACKAGE'S OWN SUBJECT.** A
`TriggerColumns` signature is a tuple of aggregate counts: two genuinely different
positions can share one, so **distinct signatures are a LOWER bound on distinct
positions** and the ratio's bias is not established in either direction. The reason a
better measurement is not available is exactly the gap this package exists to close —
**census rows carry no position identity today**. Once §2's key ships, the same two
commands measure disjointness properly, and §9 registers tranche one to do it.

**MARKED LIMITS.** Three entries per fixture, and these are BENCH fixtures rather than
the sweep's own corpus positions, so every ratio here **brackets** the sweep's rate
rather than being it.

---

## 2. DECISION 1 — the identity form. OPTION MATRIX

**The question.** What identity does a census row carry, such that D-537's
*"win-proving firings on disjoint positions"* is countable mechanically and the count
is consistent with `wp20s_design.md` §8's `key_full` denominator (F2)?

Every number is marked **MEASURED** or **ESTIMATED** (D-291).

**THE VOLUME BASIS, MEASURED.** `crates/pistol-search/examples/trigger_census.rs`
takes `--nodes --cap --gate` and arms its own wiring, so the sweep-budget firing rate
is two commands rather than a tranche:

```
nodes 400000, cap 2048, gate on
  trigger-rich fixture:   95,  97, 102 firings
  corpus fixture:        134, 159, 107 firings
```

**MEASURED 95-159 firings per ask, mean ~116**, over 694 firings in six searches.

| | **A — `key_pos`** | **B — canonical move prefix** | **C1 — `key_full`, rendered** | **C2 — `key_full`, keyed** ← recommended | **D' — minimum image key** |
|---|---|---|---|---|---|
| what it is | `GameState::key()`, a `Key128` | `canonical_sequence` over `GameState::played()` | the `q,r:p1`-joined canonical stone list, as `crates/pistol-arena/src/labels.rs:81` renders it | a `Key128` folded over `canonical_form(stones)` via `cell_key` | `min` over `Symmetry::ALL` of the key of the transformed board |
| folds transpositions | yes | **no** | yes | yes | yes |
| folds symmetries | **no** | yes | yes | yes | yes |
| §8 compliance | **FAILS** (F2) | **FAILS** (F2) | passes | passes | passes |
| cost at a firing | free by INSPECTION — already computed one method above the site (`crates/pistol-search/src/pvs.rs:249`) | a read of `GameState::played()` inside the closure, regrouped into turns, then `canonical_sequence` — same order as C1 | 12 transforms, each allocating and sorting <= 80 stones | **MEASURED 22.99 us** at 80 stones, dominated by C1's 12 sorts and 12 allocations, plus one XOR fold | 12 transforms, **zero sorts, zero allocations** — cheaper than C2 |
| cost on the **non-census** path | none | none | none | none | none |
| bytes the IDENTITY FIELD adds | 32 | variable | ~570 | 32 | 32 |
| **bytes of a whole row** | ~291 | variable | ~859 | **~291** | ~291 |
| census artifact, full sweep | ~8.7 GB | — | **~25.5 GB** | **~8.7 GB** | ~8.7 GB |
| joins the corpus's `key_full` textually | no | no | **yes** | no — same equivalence, second spelling | no |
| inherits `canonical_form`'s pinned meaning | n/a | n/a | yes | **yes** | **NO — a fourth notion of sameness** |

**SWEEP-VOLUME ARITHMETIC, shown because a bare total is not checkable.** The
identity field's bytes are not the row's bytes, and every option pays §4's other
~250 B of names and counts identically. Counted from §4's field order:

```
minimum C2 row (single-digit counts, cover none, defender '-')   281 B
typical C2 row                                                    291 B
C1 row = the same line with 32 hex replaced by a ~600 B stone list  859 B
```

D-560's **ESTIMATED** ceiling is ~119 800 distinct positions at a MEASURED 2.14x
duplication, so ~256 000 label asks; at the MEASURED mean of ~116 firings per ask,
**~3.0.10^7 census rows**. At 291 B that is **~8.7 GB (C2)**; at 859 B, **~25.5 GB
(C1)** — a ratio of **2.95x**. Both are artifacts and neither is committed (rule 8).

**THE COROLLARY THIS MATRIX OWNS RATHER THAN HIDES.** The identity is 32 B of a 291 B
row: **89 % of the axis this matrix decides on is set by a variable the matrix does not
vary** — the row FORMAT. §4 pins field order with a report test, so a
header-plus-positional-values line is compatible with the D-88 precedent and would cut
the sweep artifact to ~2.5 GB. That is not this decision; naming it is what keeps this
decision's cost axis readable (§10.5).

**Recommendation: C2**, and the mechanism is one function in pistol-core, not in this
crate:

```
crates/pistol-core/src/lib.rs:86   pub use symmetry::{Symmetry, canonical_form, canonical_sequence};
crates/pistol-core/src/lib.rs:90   pub use zobrist::{Key128, ZOBRIST_SEED, cell_key, from_scratch_key, phase_key, side_key};
```

so `canonical_key(stones) = canonical_form(stones).fold(ZERO, ^ cell_key(cell, player))`
is a fold over an existing canonical form and **not a fourth notion of sameness** —
`wp20s_design.md` §2.1's stated hazard, and the reason the function goes in pistol-core
(rule 2) rather than in pistol-search.

**C2's CORRECTNESS IS MEASURED**: the fold was exercised over **24 000
(position, symmetry) pairs** — invariant under all twelve symmetries, invariant under
move order, distinct under colour swap.

**AND IT IS CORRECT ON SIDE-TO-MOVE.** The objection is obvious and a successor will
raise it again: `GameState::key` carries side-to-move and intra-turn phase, and
`canonical_form(stones)` cannot, so a key over stones alone looks as though it folds
two different decision points into one row and **under**-counts — the opposite
direction to F2's complaint. It does not, and the reason is a property of this game
rather than of the fold: **turn 1 is one stone and every later turn is two, so the
stone count determines both the mover and the intra-turn phase**
(`crates/pistol-core/src/state.rs:129-133`), and a `Set` position's declared `to_move`
is checked against that count rather than trusted —
`crates/pistol-engine/src/position.rs:102`, *"The stated `to_move` and `phase` are
**checked, not trusted**"*. Two positions with the same stone multiset therefore have
the same mover and the same phase.

**WHY C2 OVER C1.** Two reasons, both about cost at the firing rather than on the wire:

- **C1 builds a `String` and allocates inside the census closure, per firing, on the
  search path** — ~570 B of formatting at a MEASURED ~116 firings per ask. C2's fold
  returns a `Key128` and adds no allocation of its own: it pays `canonical_form`'s
  transforms and the `collect` that feeds them — `Board::stones()` (`board.rs:91`)
  yields an iterator and `canonical_form` (`symmetry.rs:165`) takes a slice — both of
  which C1 pays too, and then C1 builds the string on top.
- **C1's one advantage does not apply.** Its textual join to the corpus's `key_full`
  column is something D-537's count does not need: the count is census rows against
  each other, and an in-tree firing (`turns_from_root > 0`) has no corpus record to
  join to.

**WHY NOT D', THE CHEAPEST OPTION ON THE ROW THIS MATRIX PRICES.** D' reaches the same
equivalence and the same count with no sorts and no allocations, and could carry
side-to-move for free by XORing `context_key`. **It is rejected because its
representative — *"the position whose key is least"* — is not `canonical_form`'s
representative, so it IS the fourth notion of sameness named as the hazard, and nothing
it produces can be pinned against already-reviewed semantics.** C2 inherits a meaning
two reviews have passed; that is worth more than 23 us a firing.

Three further shapes fail and are recorded so a successor does not re-open them:
**canonicalise once per game** — impossible, the minimising symmetry is not stable
under adding stones and the key of `g.board` is not derivable from the key of `board`;
**carry `key_pos` and fold symmetry offline** — impossible, a 128-bit hash cannot be
un-hashed into stones, and adding the stones back is C1; **carry `key_pos` plus a
symmetry tag** — strictly costlier, since producing the tag needs `canonical_form`
anyway.

**THE STRONGEST SURVIVING ATTACK ON C2**, from the fresh-context DECISION-RED-TEAM
that could not overturn the selection, quoted because the ADR line carries it:

> **C2 pays for a fold whose only measurement is zero.** The single number this
> project has on the symmetry fold's yield is the pilot's, and it is
> `key_seq = key_pos = key_full = 347` over 742 records (D-560) — symmetry merged
> **nothing** — and `canonical_sequence`'s own doc explains why a deterministic engine
> rarely produces mirrored lines: *"D-7's final tie-break is lexicographic by `(q, r)`
> and is therefore not symmetry-invariant"* (`crates/pistol-core/src/symmetry.rs:213-217`,
> D-137). Against that measurement, option A — which F2 eliminates a priori — is free
> at the site, keeps 32 hex, needs no new pistol-core function, carries side-to-move
> and phase, and reaches the same count. C2's answer is that the census population is
> **in-tree**, where a search tree generates symmetric transpositions by construction,
> and not the root population the 347 was measured over — which is correct, is nowhere
> in the document, and is itself a claim about a magnitude nobody has measured. **The
> recommendation therefore rests on an unmeasured belief that the in-tree fold's yield
> is materially above the root fold's measured zero.**

**Why it does not overturn the selection, and what it buys.** F2's asymmetry argument
is sound in direction — over-counting clears a FLOOR early, the failure D-537 exists to
prevent, so a fold of uncertain yield is taken rather than skipped — and §10.4 shows
the count's numerator is under 1 % of the rows, so a fold merging even a few positions
moves a small number by a large fraction. **What the attack buys is a two-line
addition this design adopts**: tranche one emits `key_pos` beside the canonical key and
the two distinct counts are compared (§9). That is the measurement that settles it, at
32 B a row on one tranche.

---

## 3. DECISION 2 — where the token lives. OPTION MATRIX

**The question.** The census goes on the wire *"behind a token NO committed config
sets"*. Where is that token?

**T2 IS TWO OPTIONS AND THEY HAVE DIFFERENT BLAST RADII.** WP-2.1's words are
*"Census: ON via the WP-2.0b token in **the pipeline's experiment config**"* — the
**arena** config (`crates/pistol-arena/src/config.rs`, which
`configs/arena_wp20_label_pilot.toml` instantiates), not the engine's TOML. Both are
priced, because the row that answers WP-2.1 has to be the row WP-2.1 named.

| | **T1 — a `go`-line token** | **T2a — an ENGINE config field** | **T2b — an ARENA config field** | **T3 — an `arena --capture` argument** ← recommended with T1 |
|---|---|---|---|---|
| shape | `go nodes 400000 census` | `[census] on = true` in the engine's TOML | the same in the arena's experiment TOML | a flag on the capture pass, which appends T1's token |
| committed configs touched | none | **18** | **14** | none |
| rule 1 collision | none | **direct** | **direct** | none |
| the mechanism, named rather than asserted | — | `crates/pistol-engine/tests/config_schema_tests.rs:21` forbids the string `serde(default` in the config module, and `schema_version` is refused on mismatch | `crates/pistol-arena/tests/config_tests.rs:186` forbids `serde(default`, `impl Default for` and `#[derive(Default`, same `schema_version` discipline | — |
| grammar cost | widens `parse_budget`'s `[kind, amount]` arm, which today refuses a third word (`crates/pistol-cli/src/budget_token.rs:44-51`) | none | none | inherits T1's |
| precedent | — | — | — | **the label budget itself**: *"The LABEL budget is not here — it is a command-line argument to `arena --capture`"* (`configs/arena_wp20_label_pilot.toml:47-49`) |

**BOTH T2 VARIANTS ARE ELIMINATED, BY A MECHANISM RATHER THAN BY ASSERTION.** Hard
rule 1's letter — *"a default lives in exactly one schema place"* — would on its face
permit a schema-side `#[serde(default)]`. What forecloses it is landed and testable:
**both config crates carry a test forbidding `serde(default` in the config module**,
and both carry a `schema_version` their validator refuses on mismatch. A new field is
therefore a schema-version bump **plus an edit to every committed file of that kind**.

**T1 and T3 are not rivals — T3 is T1 plus a caller AND A SINK** — and the
recommendation is both.

**Grammar, stated exactly** (rule 3, and D-88's strictness about the input side —
*"the `set` form's grammar is as strict as the tokens it carries"*): `go <kind>
<amount>` gains an optional third word which must be the census token. Any other third
word keeps today's refusal, quoted back. **A FOURTH word is refused naming the FOURTH
word, not the token** — a refusal that names the wrong token is one a driver cannot act
on. **No handshake line is added; §4 records why.**

### 3.1 T3's SINK, without which the token is inert

The arena throws every `info` line away today:

```
crates/pistol-arena/src/capture.rs:165   pub fn classify(line: &str) -> Step {
crates/pistol-arena/src/capture.rs:172       if line.starts_with(&format!("{} ", pistol_cli::report::INFO_PREFIX)) {
crates/pistol-arena/src/capture.rs:173           return Step::Ignore;
crates/pistol-arena/src/capture.rs:229                   Step::Ignore => continue,
```

The capture loop reads until `bestmove` and keeps **only** the totals line. `info
census …` begins with `info `, so a `--capture` run could set the token, have the
engine honour it, and write **zero census bytes at exit 0**.

**This is F3's failure mode by a second route, from inside the package written to
prevent it** — and the two are indistinguishable from the receipt. It is D-553's law in
its exact shape: the census writer can be correct and its call from the arena never
made. §6.2 specifies the sink, invariant 7 states it, test 14 pins it, and it carries a
call-removed mutant.

**T3 changes `capture_sha256`, which this design states rather than lets a successor
discover.** `crates/pistol-arena/src/capture.rs:103-109` digests `capture_format` +
`experiment_sha256` + `label_go <go_line>`, and the token is part of the `go` line — so
**a census-on capture has a different `capture_sha256` from the otherwise identical
census-off capture.** That is correct (it is a different instrument) and it matters
because WP-2.1 registers tranches against these digests.

**THE STRONGEST SURVIVING ATTACK ON T1+T3**, from the same red team:

> **T1 spends the protocol's `go` grammar on a switch that, by the design's own F3,
> cannot produce a byte on the seat it was built for.** The census already has a
> working off-wire seam — `Searcher::collect_trigger_census` / `take_trigger_census`
> (`crates/pistol-search/src/search.rs:206-220`), driven by
> `crates/pistol-search/examples/trigger_census.rs`, which arms its own solver
> independent of any config. So the wire is a **second** output path for rows that
> already have one, and it widens a line kind D-88 pins and every future driver reads.

**The answer, which the matrix owes.** D-562(3) registers *"census ON from game one"*,
so the census must ride the corpus's **own** games — and the off-wire route would cost
a second full sweep at F3's measured multiple, months of machine time to collect rows
the first sweep could have carried for 8.7 GB. **That is an argument from cost, and it
is the argument that decides the question.** Separately, the governing dispatch's scope
2 mandates *"census rows on the wire"*, so the wire is not this package's choice to
unmake.

---

## 4. WHAT GOES ON THE WIRE

**ONE `info census …` LINE PER FIRING PER SEARCH — A BLOCK, NOT A PER-DEPTH STREAM.**
A per-depth `SearchInfo` is constructed fresh inside the deepening loop and reported
once per completed depth (`crates/pistol-search/src/search.rs:401-416`,
`crates/pistol-cli/src/protocol.rs:169-172`), while the census `Vec` accumulates across
the WHOLE search and is drained once (`search.rs:216`). If each per-depth report
carried the run's rows, every row would print again at every later depth and the wire
volume would be multiplied by the iteration count — and §2's byte arithmetic assumes it
is not. The rows are moved back to the `Searcher` at `search.rs:525`, after every
per-depth report has been emitted, so **the block goes after the last depth's `info`,
before `info totals`, before `bestmove`**. Test 13 pins the count of lines; **test 18
pins their place**.

**Field order, pinned by a report test** (the D-88 precedent, whose own words are
*"one `info` line … whose field set is exactly `depth_turns seldepth nodes nps time
hashfull score pv` in that order"*):

```
info census key <32 hex> turns_from_root <n> mover_hot <n> opponent_hot <n>
  mover_win_in_one_ply <n> opponent_win_in_one_ply <n> mover_live_three <n>
  opponent_live_three <n> cover <token> cover_count <n>
  attacker_visits <n> attacker_proved <0|1>
  defender_visits <n|-> defender_proved <0|1|->
```

**Three properties this shape has on purpose.**

1. **`key` leads**, because it is the only new field and a reader that keys on names
   finds it without counting.
2. **The `defender` pair spells absence with `-` rather than omitting the fields.**
   The `Option<TriggerAnswer>` is `None` exactly when *"the attacker proved and the
   defender was therefore never asked"* (`crates/pistol-search/src/census.rs:19-21`).
   The ground is precedent, not a parsing hazard — the arena's reader is key-directed
   (`crates/pistol-arena/src/exchange.rs:198`, *"The word after `key`, matched whole"*)
   and a committed test there pins that a missing field reads as `None` by design.
   **The corpus already spells an absent field this way**:
   `crates/pistol-arena/src/labels.rs:14  pub const EMPTY_FIELD: &str = "-";`, read
   back at `crates/pistol-arena/src/labels_file.rs:302`. One spelling for absence
   across two files this project already ships.
3. **`cover` and `cover_count` are separate** because `CoverClass::token()` and
   `CoverClass::count()` already are, and *"the reader and the writer share one
   spelling rather than two that drift"* (`census.rs:79-81`).

**Off the token, the line does not exist** — not an empty line, not a zero-row header.
The byte-identity obligation is over the whole output and this is what makes it
satisfiable.

**NO HANDSHAKE LINE, AND THE DELETION IS A DECISION.** An `id` line advertising the
census would break the obligation this package cannot break:
`tools/baseline_snapshot.sh:578,598` reads the engine's handshake into the record's
INVARIANT BLOCK (`sed 's/^id /engine_id /'`, inside the `>>"$INVARIANT"` group), and
that block is the referent §9 names for byte-identity. The pre-change record carries
**ten `engine_id` lines** in it. An eleventh would change the digest **on every seat,
with the token off, before a single `go`**. D-88 supplies a second reason on its own:
*"the budget kinds the handshake advertises are **derived** by asking
`Budget::check_supported`, never restated"*, and a census advertisement has no
derivation source. The transcript question is answered by the `go` line the transcript
already contains, so the advertisement changes no reading (D-424).

**BOTH DIRECTIONS STAY SEPARATE FIELDS, IN BOTH SENSES OF *DIRECTION*.** The governing
dispatch requires it citing D-512; the standing ruling that says it is **D-535**
(*"THE CENSUS GATE RANKS BOTH DIRECTIONS PER D-512 AS REGISTERED"*), and D-512 is what
D-535 registers against.

- the **detector's** directions — `mover_*` against `opponent_*`, **three pairs**
  (`crates/pistol-search/src/census.rs:41-58`: `*_hot`, `*_win_in_one_ply`,
  `*_live_three`) — each a field of its own, never a sum. This is the denominator
  D-537 records as having had to be *"corrected twice, once for positions and once for
  direction"*, and a row shipping `hot 3` instead of `mover_hot 2 opponent_hot 1` would
  put that correction back;
- the **solver's** directions — `attacker_*` against `defender_*` — likewise, and the
  `-` spelling exists so the defender pair keeps its own two fields when it was never
  asked.

Test 12 asserts each direction is spelled as its own named field, on a row whose two
sides differ.

**THE D-551 RULE.** D-551's lesson is that *"the score is TWO words after its key"*
(`cp <n>`, `mate <t>`), so a reader taking the word after a key got the tag and lost
the number. **There is no score field on a census row and no multi-word value on it**,
and the rule that keeps it that way as columns are added is: *a census column whose
value can contain a space is spelled with its word count named in the schema; a column
that cannot is what §4's grammar assumes.* Test 8's field-order pin makes it
mechanical — a value that grew a space would split into an extra word, move every field
after it, and fail that test.

---

## 5. COLDNESS, AND THE NON-CENSUS PATH

The dispatch's scope 4: the census stays cold-table by D-527's discipline, and the
identity is read at firing time from state the search already holds, with a quoted site
proving no extra hashing on the non-census path.

**The columns are already computed inside a closure that runs only under a census:**

```
crates/pistol-search/src/pvs.rs:623   let observed = self.census.is_some().then(|| {
crates/pistol-search/src/pvs.rs:635       // The one column that costs more than a slice length. It is paid
crates/pistol-search/src/pvs.rs:636       // ONLY under a census — a run that collects no census never
crates/pistol-search/src/pvs.rs:637       // reaches this closure
```

The canonical key is computed **inside that same closure**, beside the cover column
that already costs more than a slice length. `canonical_form` needs the stone list and
gets it from the closure's own captured `state` via `state.board().stones()`; the root
site has the identical shape (`crates/pistol-search/src/search.rs:304-307`). On the
non-census path `self.census` is `None`, the closure is never entered, and the added
cost is the `is_some()` test that is already there.

**THE PROOF DEPENDS ON INVARIANT 8, AND SAYING SO IS THE POINT.** *"`self.census` is
`None` on the non-census path"* holds of the tree today because nothing arms it. Under
this design an engine arms it, and the existing seam **never disarms**:
`take_trigger_census` leaves collection ON by its own documented contract, and
`Searcher::clear` does not touch the field. §6.1's arming rule is what restores the
antecedent; without it every `go` after the first census `go` in a session would enter
this closure. **The quoted sites above are necessary and not sufficient, and invariant
8 supplies the rest.**

**Cold-table discipline is untouched** because this package adds no table and reads
none: `canonical_form` reads the board's stones and `cell_key` is a pure function of
`(q, r, colour, FIXED_SEED)` (D-8). D-527's defect was a `Searcher` reused across
fixture entries without `clear()`; nothing here is carried between games.

---

## 6. THE DIFF

| file | change |
|---|---|
| `crates/pistol-core/src/symmetry.rs` | `canonical_key(stones) -> Key128`, the fold of §2. Public, `///`-documented with a `# Panics` section (invariant 6) |
| `crates/pistol-core/src/lib.rs` | export it |
| `crates/pistol-search/src/census.rs` | `TriggerObservation` gains `key: Key128` |
| `crates/pistol-search/src/pvs.rs` | the key computed inside the existing census closure; passed through `observe`; the fold-entry counter of test 17 |
| `crates/pistol-search/src/search.rs` | the same at the root site; **`stop_trigger_census`** beside the existing `collect`/`take` pair (§6.1). **`Searcher::clear` is NOT changed** |
| `crates/pistol-search/src/info.rs` | `SearchOutcome` gains the census rows — **not** the per-depth `SearchInfo` (§4) |
| `crates/pistol-engine/src/lib.rs` | re-export `TriggerObservation`, `TriggerAnswer`, `CoverClass` — the F1 route, beside the existing `SearchInfo` re-export |
| `crates/pistol-engine/src/engine.rs` | `go_reporting` gains the census request; `go`'s default forwards "no census" (§6.1) |
| `crates/pistol-engine/src/instance.rs` | the request reaches the searcher, and the arm/disarm pair around a census `go`; **no config field** (§3) |
| `crates/pistol-arena/src/bin/stub_engine.rs` | **the second `Engine` implementor** — it changes with the required trait method, and **it REFUSES a census request by name rather than accepting one and returning no rows** (rule 3). A stub that silently honours a request it cannot serve is §3.1's defect one layer down |
| `crates/pistol-cli/src/budget_token.rs` | the optional third word, its named refusal, and the fourth-word refusal naming the fourth word |
| `crates/pistol-cli/src/report.rs` | `census_line`, field order per §4 |
| `crates/pistol-cli/src/protocol.rs` | emit the block after the last depth's `info`, before `info totals`. **No handshake change** |
| `crates/pistol-cli/tests/movetime_tests.rs`, `crates/pistol-engine/tests/engine_tests.rs` | the two committed call sites of `go_reporting` |
| `crates/pistol-arena/src/capture.rs` | **THE SINK** — `Step` gains a public `Census(String)` variant; `classify` gains its arm ahead of the `info` catch-all; `ask` gains a `&mut Vec<String>` sink (§6.2). The token appended in **`label_go_line`**, not `BudgetSection::go_line()` |
| `crates/pistol-arena/tests/capture_tests.rs` | `the_label_go_line_is_the_one_budget_section_spells` **moves** — it pins the equality the token breaks. Listed because F1 makes a point of the workspace-shape test staying green unchanged, and the same honesty is owed about the test that does not |
| `crates/pistol-arena/src/bin/arena.rs` | the `--capture` census flag only — this file delegates at `:73` |
| `crates/pistol-arena/src/passes.rs` | **the write's actual call site**: `:43-56` calls `run`, `render` and `manifest_row`, so the census file is written and indexed here |
| `crates/pistol-arena/src/capture.rs` (`run`) | `run`'s signature at `:242` carries the sink through from `ask` |
| — | **the protocol has no separate markdown home**: its verbs and answers are the public items of `crates/pistol-cli/src/protocol.rs`, pinned as prose by D-88. The dispatch's *"documented in the protocol's one home"* resolves to the `///` docs in that file plus §8's report test |

**No manifest changes** (F1). No committed config changes (§3). `pistol-api` untouched.

### 6.1 THE `Engine` SEAM AND ITS ARMING RULE

`crates/pistol-engine/src/engine.rs:53` defines `go` with a default body forwarding to
`go_reporting`; `:63` defines `go_reporting` as a **required** method.

- **The request is a parameter on `go_reporting`.** Because it is required, **both
  implementors change** — `crates/pistol-engine/src/instance.rs:86` and
  `crates/pistol-arena/src/bin/stub_engine.rs:146` — and **all three call sites**:
  `crates/pistol-cli/src/protocol.rs:172`, `crates/pistol-cli/tests/movetime_tests.rs:99`,
  `crates/pistol-engine/tests/engine_tests.rs:119`. Callers of `go` are unchanged,
  because its default forwards "no census".
- **The rows come back on `SearchOutcome`**, the return type —
  `{ best, info, provenance }` at `crates/pistol-search/src/info.rs:237-245`, a
  pistol-search struct re-exported by pistol-engine. The field is added there, which is
  why §6's `search.rs`/`info.rs` rows carry it and the per-depth `SearchInfo` does not.

**THE ARMING RULE.** The existing seam does not disarm, and every limb of that is
documented at the site:

```
crates/pistol-search/src/search.rs:206   pub fn collect_trigger_census(&mut self) { self.census = Some(Vec::new()); }
crates/pistol-search/src/search.rs:210   /// The rows collected since the last take, leaving collection ON.
crates/pistol-search/src/search.rs:216   pub fn take_trigger_census(...)   // panics if self.census is None
crates/pistol-search/src/search.rs:230   pub fn clear(&mut self)           // table, heuristics, solver — NOT self.census
```

> **THE RULE. The engine's census lifetime is exactly one `go`. The order is
> `collect_trigger_census` -> search -> `take_trigger_census` -> `stop_trigger_census`,
> and the disarm runs on EVERY exit path of a census `go`, the error path included.**

Three things that ordering settles, each because getting it wrong is a live failure:

1. **`take` comes before `stop`**, because `take_trigger_census` panics when
   `self.census` is `None`. A design whose §6.1 exists because a seam's contract was
   not read owes that line explicitly.
2. **`stop_trigger_census` discards whatever rows remain**, and under the stated
   ordering there are none. The discard is unreachable rather than lossy, and test 15
   pins the ordering that makes it so.
3. **`Searcher::clear` is NOT changed**, for two independent reasons. It would change
   no reading — with the disarm on every exit path, `self.census` is already `None`
   when `Pistol::new_game` (`instance.rs:74-76`) calls `clear` — and it would **break
   `crates/pistol-search/examples/trigger_census.rs`**, which calls
   `collect_trigger_census()` at `:167`, `engine.clear()` inside its per-entry loop at
   `:195`, and `take_trigger_census()` at `:215`; clearing the census in `clear` makes
   that example panic at entry 0. **That example is the instrument that produced §2's
   firing counts, F3's ratios and D-563's numbers**, three committed
   `tools/stage3_*.py` scripts parse its output, and no CI gate runs it. The
   determinism obligation hard rule 4 asks for is discharged by the disarm plus test
   16, which drives two consecutive `go`s.

### 6.2 THE SINK'S ARTIFACT

A new artifact class with no format and no manifest row is a rule-8 and D-469
obligation deferred onto the closure, so it is decided here.

- **`Step` gains a public `Census(String)` variant.** `Step` is a public enum
  (`crates/pistol-arena/src/capture.rs:150-157`) matched exhaustively at `:224-231` and
  asserted on in `capture_tests.rs`; adding a variant is a seam change and is listed
  rather than discovered.
- **`ask` gains a `&mut Vec<String>` sink** (`capture.rs:181`) rather than a changed
  return type, so its `Result<(String, String), _>` contract and every existing caller
  stay as they are.
- **The file, written in the arena's ACTUAL idiom rather than a gesture at it.** One
  census file per capture run, named from the capture's own stem, carrying the
  `Fixture` header the sibling writers already produce — `# param` / `# derived` lines
  and a **`# body_sha256` payload digest** (`crates/pistol-arena/src/capture_file.rs:59-72`
  is the template and the one call site) — then one row per line in §4's field order,
  **the same bytes the wire carried**, so a consumer that disagrees can re-parse without
  re-running the engine (D-551's own reason for splitting the passes). **The body digest
  is not decoration here**: rule 8 and D-469 lean on exactly it when the artifact is
  ~8.7 GB, uncommitted, and sha-indexed by a committed manifest. It gets a
  `manifest_row` beside the capture's own (`capture_file.rs:106`, called at
  `passes.rs:56`).
- **Test 14 synthesises its armed seat**, because every committed arena config points
  both engine seats at a gate-off engine config (`arena_wp20_label_pilot.toml` names
  `configs/instrument_v0.toml` for both) and committed-config changes are out of scope.
  **It runs at a small budget on purpose**: by F3's measurement an armed seat is
  expensive, and a CI gate is not the place to spend that.

---

## 7. INVARIANTS

1. **The census cannot move a move.** Nothing added here is read back by the search;
   the identity is written, never probed (rule 4, and `census.rs:11-12`).
   `canonical_key` cannot end a search within the radius-8 region — invariant 6 owns
   the bound.
2. **Off the token, the engine's bytes are the pre-change engine's bytes**, over the
   standing position set, under §9's stated extraction rule.
3. **A firing produces exactly one LINE**, emitted once as a block after the last
   depth's report and never re-emitted per depth. Tests 13 and 18.
4. **Two firings on positions with the same `canonical_form` carry the same key**, and
   two on different canonical forms do not. MEASURED over 24 000
   (position, symmetry) pairs.
5. **The key is a pistol-core function**; no crate outside it decides sameness (rule 2).
6. **THE FOLD IS BEING PUT ON THE SEARCH PATH AND ITS OVERFLOW BEHAVIOUR IS PART OF
   THE CONTRACT.** `canonical_form`'s transform arithmetic panics on coordinate
   overflow. Today it runs at the root, off the search path; under this design it runs
   at every firing. **The radius-8 legal region (game rule 5) bounds the reachable
   coordinates far below the overflow point** — a position would need on the order of
   a thousand turns to approach it — so the panic is unreachable in play, and that is
   why it is written down rather than left implicit: `canonical_key` carries a
   `# Panics` section naming the bound that makes it unreachable, so a successor who
   widens the region reads the consequence at the function rather than in a crash.
   `Symmetry::apply` already documents its own panic, and this is the same bound.
7. **The census request reaches the arena's output or the run fails loudly** — a token
   honoured by the engine whose rows the capture discards is §3.1's defect. Test 14.
8. **COLLECTION IS ARMED PER CENSUS `go` AND DISARMED AT ITS END, ON EVERY EXIT PATH.**
   §5's proof is false without it after the first census `go` in a session, and rows
   would cross `go` boundaries. Tests 15 and 16 pin its two calls.

---

## 8. TESTS AND MUTANTS

**THE SEAT RULE, which every row below is placed by.** By F3, a test that needs a
census row cannot get one on a gate-off seat, so it would pass vacuously against an
empty row set. Hence:

> *(a) a test whose subject is a **pure function** has no seat; (b) a test whose
> subject is **refused before a search starts** has no seat; (c) a test whose mutant
> can only manifest **at a firing** is seated solver-ON; (d) exactly one test is seated
> solver-OFF, because the absence of firings is what it pins.*

The armed seat is `configs/gate_staged_solver_v0.toml`, committed, already a CI gate
seat, and existing for this reason.

| # | test (behaviour-named) | mutant it kills | seat |
|---|---|---|---|
| 1 | `a_census_row_carries_the_canonical_key_of_the_position_it_fired_at` | identity column dropped | ON (c) |
| 2 | `two_move_orders_reaching_one_position_share_a_census_key` | the fold loses transposition-invariance | none (a) |
| 3 | `a_position_and_its_mirror_image_share_a_census_key` | the fold loses symmetry-invariance — which `key_pos` would fail (F2) | none (a) |
| 4 | `a_known_transposition_pair_counts_as_one_disjoint_position` | the transposition ruling inverted — the pair counted as two | none (a) |
| 5 | `without_the_token_a_go_line_writes_no_census_byte` | token check removed | ON (c) |
| 6 | `a_third_word_that_is_not_the_census_token_is_refused_naming_it` | the grammar widened to accept anything | none (b) |
| 7 | `a_census_row_spells_an_unasked_defender_rather_than_omitting_it` | the absent-field spelling | ON (c) |
| 8 | `the_census_line_field_order_is_the_documented_one` | field reordering (D-88's precedent) | ON (c) |
| 9 | `a_search_with_the_solver_gate_off_produces_no_census_row_under_the_token` | **the census made reachable without the solver** — a firing site moved outside the wiring guard, which would silently change what the sweep measures | OFF (d) |
| 10 | `the_non_census_path_does_not_compute_a_canonical_key` | the key hoisted out of the closure (§5) | ON (c) |
| 11 | `two_positions_that_are_not_one_position_carry_different_census_keys` | the fold OVER-folding | none (a) |
| 12 | `a_census_row_spells_each_direction_as_its_own_field` | either direction pair collapsed (D-535/D-512) | ON (c) |
| 13 | `a_firing_produces_one_census_line_however_many_depths_completed` | the per-depth re-emission (invariant 3) | ON (c) |
| 14 | `a_capture_run_with_the_census_flag_writes_the_rows_it_was_sent` | the sink removed or never called (invariant 7) | ON (c), through `arena --capture` |
| 15 | `a_census_go_takes_its_rows_before_it_disarms` | `stop` ordered before `take`, which panics (§6.1) | ON (c) |
| 16 | `a_plain_go_after_a_census_go_computes_no_key_and_emits_no_line` | the disarm never called (invariant 8) | ON (c) |
| 17 | `the_fold_is_entered_exactly_once_per_firing` | **the fold hoisted, called per node, or called twice** — the structural check §9 registers in place of a timing bound it cannot resolve | ON (c) |
| 18 | `the_census_block_is_emitted_after_the_last_depth_and_before_the_totals_line` | the block's PLACE moved (invariant 3) | ON (c) |

Test 9 pins a **limitation**, not a feature, and is how a successor discovers F3 from
the test suite instead of from a months-long run.

**Test 4 pins key equality plus the stated counting rule, not a program.** Nothing in
§6 counts disjoint positions — the census emits rows and an analyst counts them — so
the fixture pins that the transposition pair shares one key, and the count follows from
the rule §2 states. The dispatch's scope 1 asks for *"a fixture pinning a known
transposition pair to the ruled count"*, and this satisfies it without naming a program
this package does not ship.

**CALL-REMOVED MUTANTS (D-553).** A guard's mutation set includes a call-removed
mutant, and it must die at a test that drives the call site with reachable input; a
test invoking the guarded function directly pins the function and leaves *"the call was
never made"* alive. Seven of the eighteen guard a CALL:

| mutant | the call removed | the test that must die | why a direct-call test would NOT do |
|---|---|---|---|
| token check removed | the `go` handler's test of the third word | **test 5** | a test calling the token parser directly proves the parser classifies a word, not that the printer consulted it |
| identity column dropped | `census_line`'s emission of `key` | **test 1** | a test calling `canonical_key` directly proves the fold and leaves a row that never carries it |
| the key hoisted out of the closure | the `is_some().then(…)` guard at `pvs.rs:623` and `search.rs:304` | **test 10** | a test asserting the key's VALUE cannot see where it was computed |
| the arena's sink never called | the census arm in `classify`, or the write that follows it | **test 14** | every other test drives the ENGINE; the engine is correct in this mutant and the rows are dropped downstream |
| the emission moved inside the per-depth report | the block emission after the last depth | **test 13** | a test asserting one row per firing passes — the internal `Vec` is right; only a test counting LINES over a multi-depth search sees it |
| the disarm never called | `stop_trigger_census` at the end of a census `go` | **test 16** | a test asserting the first `go`'s rows are right cannot see that the SECOND `go` is still paying the fold |
| the fold's guard bypassed | the `is_some()` test in front of the fold | **test 17** | the counter is the only witness that distinguishes "computed once per firing" from "computed per node" |

The remaining eleven are value mutants: they change what a computation returns, and a
test that calls the computation kills them.

**Mutation receipts run green BEFORE REVIEW-impl is dispatched** (D-553's corollary,
restated in the dispatch's obligations), and **mutation testing runs in a separate
worktree** (CLAUDE.md, Process) on `/home`, never in the live tree.

---

## 9. OBLIGATIONS BEFORE CLOSURE

**BYTE-IDENTITY, GATE OFF.** Two-binary diff over the standing position set, using
`tools/baseline_snapshot.sh`. **THE EXTRACTION RULE EXCLUDES THE TWO LINES THAT MOVE
FOR EVERY CORRECT BUILD, AND AN EARLIER REVISION DID NOT — WHICH MADE THE OBLIGATION
UNSATISFIABLE.** The invariant block's own first lines are:

```
artifacts/prechange2_gate_v0_run1.txt:1-5
baseline_snapshot 1
schema 1
revision a56449baeebc3519385b32059d2dea76612d1554
binary_sha256 180b4c406b225fc81342bb8218b8546dda1ffac1a99f7eb91cdaf73d20253476
config /home/tom/Projects/HeXO-AlphaBeta/configs/gate_v0.toml 4af71088…
```

`revision` and `binary_sha256` **necessarily differ** between the pre-change and
post-change records — that is what makes them two binaries — so a digest over the whole
block fails on every correct build, and *"byte-identity mismatch"* is the first item on
this section's own STOP list. The rule is therefore:

```
sed -n '1,/^# timing/p' <record> | grep -v '^revision \|^binary_sha256 ' | sha256sum
```

**AND THE RECORD IS RE-TAKEN FROM THE MAIN TREE, because the `config` line carries a
WHOLE PATH** (D-232's named residual, and the guard `tools/baseline_snapshot.sh` grew
for it): the first pre-change record was taken inside a verification worktree and its
`config` line read `/home/tom/.cache/wp20b-baseline/configs/gate_v0.toml`, so its
digest was partly an artefact of where it was taken. Re-taken from the main tree with
`--binary artifacts/pistol_prechange_a56449b`, the path is the one the closure will
also produce.

**THE PRE-CHANGE REFERENTS, MEASURED, each equal across two runs:**

| config | referent |
|---|---|
| `configs/gate_v0.toml` | `81e37d420d32e6d973c27bcda03ab32da7059cbf5ad35b430aa50c213b00a98b` |
| `configs/instrument_v0.toml` | `c7f155e8a6702e5bbbeaeba0ddcb53f3c01f7c7ae0dd847fe6de74bcab3eb20e` |

24 positions at 50 000 nodes, pre-change binary `180b4c40…`, records
`artifacts/prechange2_*`, receipt `artifacts/wp20b_prechange_RECEIPT.txt`.
**Run 1 equals run 2 under this rule for both configs**, which is the check that the
rule did not buy satisfiability by excluding something that varies.

**DETERMINISM.** `tools/determinism.sh` green on all seats. The census carries no state
across games: invariant 8's disarm runs on every exit path of a census `go`, so
`new_game` finds nothing to clear (§6.1), and test 16 pins it over two consecutive
`go`s.

**THE PERF GUARD IS ONE BENCH COMPARISON AND ONE STRUCTURAL CHECK, AND THEY EXCLUDE
DIFFERENT DEFECT CLASSES.**

Both costs this package can add — formatting and printing the rows, and computing
`canonical_key` — are paid under exactly one condition: the token on, at a firing. **One
ON/OFF comparison at one seat measures their sum and no arm separates them**, so the
bench is ONE comparison rather than two labels over one measurement.

**THE INSTRUMENT IS NOT `tools/bench_delta.sh`, and that is registered rather than
assumed.** That script pins `CONFIG="configs/instrument_v0.toml"` at `:92` — the
gate-OFF seat — varies the **binary** rather than the `go` line (`:351` writes the same
`$budget` to both sides), and carries the `Eval::delta` bench's own abort clause
(`nps < 1.15`), which a census change with an honest ratio near 1.000 would trip. The
instrument is therefore the command block below, **named with its governing revision**
per `docs/process.md`'s *"an artefact that produces a registered number … is named in
the pre-registration WITH ITS REVISION"*. **INSTRUMENT REVISION: the closure HEAD at
which the guard runs**, restated in the closure receipt; an edit to the block reopens
this registration.

```
# WP-2.0b perf guard. Seat: configs/bench_wp18c_solver_on.toml (on_search_path = true,
# per_call_node_cap = 2048). Fixture: crates/pistol-cli/tests/fixtures/bench_solver_positions_v1.txt
# (20 entries). One binary, two go lines. REPS=5.
#
# THE TWO FILTERS ARE LOAD-BEARING AND ARE tools/baseline_snapshot.sh:484-485's:
#   entries() { grep -v '^#' "$1" | grep . || true; }   # drop comment and blank lines
#   tail_of() { printf '%s' "${1%% #*}"; }              # drop the trailing " # anchor ..."
# Without them EVERY line is refused and the engine searches the empty board, both arms
# return ~1.000, and the guard passes while measuring nothing. MEASURED: 8 of 8 sampled
# lines refused without the filters, 0 of 20 with them. The parameter expansion is
# `%%` — the longest-match suffix operator. An earlier revision printed `%%%%` here and
# a reviewer ran it: 20/20 refused, 20 x `bestmove 0,0`, exit 0, which is this block's
# own failure mode reproduced inside its own repair.
for rep in $(seq 1 "$REPS"); do
  for arm in "go nodes 50000" "go nodes 50000 census"; do
    grep -v '^#' "$FIXTURE" | grep . | while read -r line; do
      position="${line%% #*}"
      printf 'newgame\nposition %s\n%s\n' "$position" "$arm"
    done | "$BIN" --config configs/bench_wp18c_solver_on.toml
  done
done
# Report per position, paired across arms: nps AND time-to-depth (hard rule 5).
# A refusal on any line VOIDS the run: the guard's whole content is that both arms
# searched the same positions.
```

`newgame` precedes every position for the reason `tools/baseline_snapshot.sh` gives:
a table carried across positions lets one search's node count depend on another's (D-7).

**DRY RUN — TAKEN, NOT PROMISED, AND ITS INPUT AND OUTPUT RECORDED.** The block's
literal commands were run on the same KIND of input — the same seat and the same
fixture at `nodes 2000`, differing from the registered workload only in budget — with
the pre-change binary `artifacts/pistol_prechange_a56449b`
(sha256 `180b4c40…`). **Only the OFF arm can run**, because the token does not exist
yet; what the dry run exercises is therefore the harness's ATTRIBUTION — that the
commands reach the intended positions — and not the comparison.

```
refusals                     0
bestmove lines              20     (= the fixture's 20 entries)
`bestmove 0,0` (empty board)  0
solver_firings              1 per search at nodes 2000
```

Output at `artifacts/wp20b_perf_dry_run.txt`, digested in
`artifacts/wp20b_cap_RECEIPT.txt`. **The same block WITHOUT the filters was run first
and refused 8 of 8 sampled lines**, which is the defect the dry run exists to catch and
the reason it is recorded rather than asserted (`docs/process.md`, dry-run discipline).
It is not a governed sample and does not consume the pre-registration's first run.

**REGISTERED HYPOTHESIS H1 = `1.000x`, A NO-CHANGE HYPOTHESIS, AND AN ABORT AT 0.95.**
Hard rule 5 asks for a hotspot, an expected-gain bracket AND an abort threshold, and an
earlier revision registered only the abort on the ground that the instrument cannot
resolve a finer bracket. **D-249 is the precedent that shows the third element is still
owed and how to state it**: it registers *"H1 AS EXACTLY `1.000x`, A NO-CHANGE
HYPOTHESIS"*, and says in its own words *"That is falsifiable and it is not a
placeholder … because a reader meeting `1.000x` in a rule-5 bracket will otherwise read
it as a number someone forgot to fill in."*

- **H1: the ON/OFF nps ratio is `1.000x`.** The fold's predicted cost at this seat and
  budget is **9.05 firings** — D-517's trigger-rich figure, which **supersedes D-516's
  6.72 by name** — times a MEASURED **22.99 us**, or **~0.21 ms per search**, which is
  below what this comparison resolves. So no-change is the prediction, not a gap in the
  form.
- **H1's REFERENT IS THE PRE-CHANGE BINARY, NOT THE OTHER ARM, AND THAT CORRECTION IS
  WHAT MAKES IT FALSIFIABLE.** An earlier revision claimed a 1.000x ON/OFF ratio would
  catch *"a cost that leaks outside the guard"*. **It would not**: a key computed before
  the `is_some()` test, a hoisted formatting call, a per-search allocation are all paid
  on **both** arms of a single-binary ON/OFF comparison, so they are COMMON-MODE and
  push the ratio TOWARD 1.000 — `docs/process.md`'s named vacuity, *"a criterion that is
  a property the named defect class PRESERVES"*, which is the same trap this obligation
  has fallen into at three earlier revisions. **D-249's 1.000x works because its
  instrument is a CROSS-REVISION comparison**, and this one is corrected to match:
  **H1 is the post-change binary's token-OFF nps against the PRE-CHANGE binary's, on the
  same seat and fixture** — `artifacts/pistol_prechange_a56449b`, sha256 `180b4c40…`,
  already built and digested. A leak outside the guard is paid by the post-change binary
  and not by the pre-change one, so it moves THAT ratio and nothing common-mode does.
  The single-binary ON/OFF comparison is retained for the 0.95 gross-regression abort,
  where common mode is not a problem because the question there is a large effect.
- **ABORT below 0.95**, which is the coarse gross-regression floor.
- **The comparison is PAIRED per position across arms**, which resolves finer than a
  band IQR; the 10 % IQR gate is a property of the band summary, not of the paired
  difference, and the two must not be confused when reading the result.

The bench therefore excludes a gross regression and a leak outside the guard; **the
fold's own placement is excluded structurally instead**, below.

**COST.** 2 arms x **20 positions** x 5 reps on an armed seat — the fixture has 20
entries, counted — at a MEASURED **4.75 s per search at `nodes 50000` on this
seat** (4 searches in 19 s, this revision, with `artifacts/pistol_prechange_a56449b`),
which is **~16 minutes** for 200 searches. **The rate is not the dry run's**: the dry
run ran at `nodes 2000` and implies about a minute, which would be wrong by an order.
Stated because
this guard runs on the seat F3 measures as expensive and the proportion between the
document and the run belongs on the document's face (`docs/process.md`).

**THE STRUCTURAL CHECK (test 17) IS WHAT ACTUALLY EXCLUDES THE DEFECT CLASS.** A
counter asserts `canonical_key` is entered **exactly once per firing per search**,
killing the hoist, the accidental per-node call and a double call outright and
falsifiably, at any budget, with no timing. **What it does NOT exclude, said so the
criterion is not read as bigger than it is**: a fold correctly placed and merely slower
than 22.99 us. The 0.95 abort covers that, coarsely, and nothing covers it finely —
which is the honest position for a change whose measured cost is three orders below the
instrument's floor.

**TRANCHE ONE EMITS `key_pos` BESIDE THE CANONICAL KEY**, and the two distinct counts
are compared. This is the two-line answer to §2's strongest surviving attack — the
symmetry fold's only measurement is zero (D-560's `key_seq = key_pos = key_full = 347`)
and this settles whether the in-tree fold's yield is above it. It costs 32 B a row on
one tranche.

**REVIEW-impl**: fresh context, not the implementer, one fix round; a second failure is
STOP and split.

**ARTIFACTS** exported with digests (D-469); CI all 19 gates at closure HEAD; tree
clean; the closure summary in the standing format, ONE LINE FOR THE MORNING first.

**ON STOP** (byte-identity mismatch, determinism exit 3, CI red after the one fix
round, a failure outside the diff, or any cap exhausted): tree clean or WIP on
**`wp20b-stopped`**, never `dev`; no detached processes, with a receipt; exports
complete; the summary names the decision owed, plain language first.

**ROADMAP AT CLOSURE**: production label runs UNBLOCKED, and the successor is the
**production corpus package** — label runs at D-562(3)'s registered size, census on
from game one, D-537's clock starting — then the Stage-2 eval design package. **F3
(§10.1) is what stands between that successor and its census.**

---

## 10. WHAT THIS PACKAGE DOES NOT DECIDE

1. **F3's question, and it is the operator's**: whether the production labelling seat
   arms the solver on the search path, **and at what cap**. Without arming, the census
   records nothing; with it, both the corpus's labels and D-560's cost model change.
   **WP-2.1 cannot register a census, and D-537's clock cannot start, until this is
   ruled.** Three committed armed seats already exist, so the ruling is a choice among
   existing shapes; §1.1 gives the cap's own measurement so the choice is made on
   numbers rather than on the one ratio F3 happens to have measured.
2. **Whether the corpus's `key_full` column is re-spelled as §2's `Key128`** — a
   WP-2.0-S schema question.
3. **D-537's registered minimum itself**, which `wp20s_design.md` §8 owns. **But its
   counting rule owes an ADR line and this package's closure is where it belongs**: §8
   of a design document is what F2 treats as binding on this package, and hard rule
   10's drift clause is about ADR lines. If the `key_full` disjointness rule is
   load-bearing for D-537's count — and F2 makes it so — it should be citable as a
   decision and not only as a section.
4. **THE ROW-SET QUESTION, which dominates the cost axis §2 decides on.** D-537 counts
   win-proving firings on disjoint positions; §2 prices **all** firings. The two differ
   by the proof rate and again by the fold, and MEASURED the proof rate is tiny: across
   the six searches of §2's own basis, `proofs` came back **0, 0, 4, 0, 0, 0 — four in
   694 firings, 0.58 %** — and win-proofs are a subset of that again, since D-530
   records a search finding six proofs *"and every one is a proven LOSS"*. **A census
   writing a row only where it could enter D-537's count would reach the registered
   criterion identically and cut ~8.7 GB to well under 100 MB.** It is not adopted, and
   the reason is recorded so a successor does not adopt it by accident: the census's
   older and equally load-bearing purpose is D-516's *"what FRACTION of the present
   trigger's firings does a predicate keep"*, and a fraction needs its denominator.
   **The unfiltered row set is dead weight for D-537's clock and load-bearing for the
   detector's option field**, and this design keeps it for the second reason.
5. **The row FORMAT**, which §2's corollary shows sets 89 % of the bytes the identity
   decision was priced on. A positional line would cut the sweep artifact from ~8.7 GB
   to ~2.5 GB. It is a D-88 question about the protocol's shape, not an identity
   question, and it is named here so the cost axis is readable rather than silently
   dominated.
6. **Which key rules a disagreement between the corpus's three keys.** D-562(2) assigns
   this to WP-2.0b by name — *"which key rules a disagreement is WP-2.0b's transposition
   question and not this line's"* — and answers it only for the corpus's own dedup.
   **This design answers it for the CENSUS and nowhere else**: a census row is
   identified by §2's canonical key, which is `key_full`'s equivalence in a second
   spelling, so for census purposes `key_full` rules. The corpus's dedup rule, where
   the three keys can disagree at sweep scale, is the Stage-2 assembly's question and
   stays D-562(2)'s.
7. **Any detector logic.** Round 3 re-opens on the count, not on this package.
8. **WHETHER THE DISPATCH'S SCOPE SURVIVES ITS OWN PREMISE BEING FALSE — and this is a
   question for the operator, not a finding.** F1 dissolves scope item 3 (the
   workspace-shape test needs no update, because the route it assumes absent already
   exists) and F2 empties scope item 1's option field (both named identity forms are
   forbidden by a landed, reviewed document that names this package out loud). This
   design proceeds on both, and every reviewer so far has upheld both. **But a design
   does not get to retire two of its dispatch's four scope items on its own authority**,
   and the departure belongs on this list rather than presented as settled. An operator
   who intended the v2 re-issue as a ruling on either — it edited scope 1's parenthesis,
   adding *"per D-6"*, while leaving both options standing — should say so, and §2
   re-opens rather than being amended.
