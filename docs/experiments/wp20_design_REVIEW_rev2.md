# SCOPED RE-REVIEW — `docs/experiments/wp20_design.md` revision 2

## Header

**Subject, at its named revision:** `docs/experiments/wp20_design.md` at
**`7689714399b84ab9ce8ca1ed7879ea8790b4eb6d`** on `dev`
(*"docs(wp20): the design's record half is rebuilt — provenance is on the wire
and now in the schema, the third score spelling is named, and an absent score is
refused rather than blanked"*).

**HEAD match:** `git rev-parse HEAD` → `7689714399b84ab9ce8ca1ed7879ea8790b4eb6d`.
**The subject IS HEAD.**

**Tree state:** `git status --porcelain` → empty. Clean.

**Prior revision:** `bb44d6fbd62f2df706989953d7072d513210ce28`. I diffed them
myself (`git diff bb44d6f 7689714 -- docs/experiments/wp20_design.md`) before
reading either in full, so every disposition below is against what actually
changed rather than against what revision 2 says it changed.

**Fresh context.** This session did not write the design, revision 2, the prior
review, the selection, the matrix or the memo.

**Scope.** Per the dispatch: whether each enumerated finding was remedied, and
whether a remedy broke something. I did not re-open the shape, the selection, or
anything the prior review passed.

**What I ran.** `git`, `git grep`, `/usr/bin/grep`, `sed`, `wc`. Every recorded
enumeration is `/usr/bin/grep` or `git grep`, `LC_ALL=C sort`ed (D-265).

**What I refused.** No `cargo` in any form, and no `tools/ci.sh`, per the
dispatch. No finding below needs a build to stand; where a build would add
confirmation I name the exact run.

**Code read to verify the remedies:** `crates/pistol-cli/src/report.rs`,
`crates/pistol-search/src/` {`info.rs`, `search.rs`, `pvs.rs`},
`crates/pistol-arena/src/` {`report.rs`, `transcript.rs`, `exchange.rs`,
`record.rs`, `Cargo.toml`}, `crates/pistol-core/src/turn.rs`,
`crates/pistol-cli/src/corpus/openings.rs`, `configs/`,
`docs/decisions.md` (D-6, D-483, D-518, D-537, D-539–D-543),
`docs/experiments/wp20_dispatches.md`, `docs/process.md`.

---

## VERDICT: **FAIL**

**2 new BLOCKING, 6 new MAJOR, 3 new MINOR. Of the prior review's 21 findings:
4 APPLIED, 6 PARTIALLY APPLIED, 9 NOT APPLIED, 2 APPLIED BUT INTRODUCED A NEW
DEFECT.**

Revision 2 did real work and some of it is good. §2's forfeit reversal is
**safe** — I attacked it exactly where the dispatch pointed and it holds, for a
reason revision 2 did not state and I had to find (see *the strongest attack
that did not land*). §4's three spellings are exact against `score_token`. §5's
`openings_skip` precedent is exact. The INVARIANT 7 / INVARIANT 10 pair the
dispatch asked about is **not** a contradiction. D-483 holds, and §4a states the
command behind its one tree measurement, which is D-543's own remedy.

It fails for two reasons, both of them consequences of the fix round itself.

**First, the BLOCKING 1 remedy is built on a predicate that is false.**
`info.solver_nodes > 0` is not "this answer came from a solver proof" — it is
"the solver was consulted". A search that consulted the solver at interior nodes
and did not prove at the root returns `Provenance::CompletedDepth` with the
solver fields on the wire. INVARIANT 8 therefore marks ordinary search answers
as solver-provenance, and the very corpus column the finding existed to make
trustworthy is wrong for exactly the configs §4a establishes are legal. The
finer discriminator *is* on the wire and revision 2 did not use it. Worse, §3's
`label nodes` column is still sourced from the totals line's `nodes`, which the
engine defines as `search_nodes + solver_nodes` — so the record **does** sum the
two quantities that INVARIANT 9 says are never summed.

**Second, the forfeit reversal was applied in two places and not in the other
two.** §7 still restricts candidates to *non-forfeited* turns, and §11 still
registers `a_forfeited_game_contributes_no_records_and_is_counted` — the test for
the invariant revision 2 deleted. The document now says both things, and its
test list pins the negation of its own INVARIANT 3.

Four of the prior review's eight MAJORs (4, 5, 7, 8) were not touched at all, and
ten of eleven MINORs were not touched. That alone would be PASS WITH FINDINGS
territory for a first round; it is not what decides this one. The two BLOCKING
items above are.

**The fix round is spent. By the standing caps this is a STOP and a split.** I
note without recommending it that every finding below carries a one-to-few-line
fix and that the *shape* has now survived two independent attacks — that is the
operator's call, not mine.

---

## PART 1 — the prior review's findings, every one

### BLOCKING

#### BLOCKING 1 — `label depth`/`label nodes` mean two things; memo decision 9 dropped

**Disposition: APPLIED BUT INTRODUCED A NEW DEFECT.**

Revision 2 took the decision (§4a, lines 103–131), added a `label provenance`
column (line 96), added INVARIANT 8 and 9, a test and two mutant rows, and took
the review's option 2 (record the discriminator) over option 1 (refuse gate-on
configs). The escape-hatch refutation is **verified**:

```
$ LC_ALL=C /usr/bin/grep -rln "on_search_path = true" configs/ | LC_ALL=C sort
configs/bench_wp18c_solver_on.toml
configs/gate_staged_solver_v0.toml
configs/play_staged_solver_v0.toml
```

Three, exactly as §4a says, and §4a states the command that produced the count —
D-543's remedy, honoured.

The conditional is also **verified**. `crates/pistol-cli/src/report.rs:62`:

```
let solver_field = if info.solver_nodes > 0 {
```

and the six fields (`search_nodes`, `solver_nodes`, `solver_firings`,
`solver_invocations`, `solver_proofs`, `solver_root_nodes`) ride inside it. A
gate-off line prints none of them. So far §4a is exact, and its *"it needs
nothing new on the wire"* claim is true — the no-engine-diff license survives.

**But the predicate is the wrong one, and this is NEW BLOCKING 1 below.** See
that finding for the mechanism. In short: `Provenance` has four variants
(`crates/pistol-search/src/info.rs:250–266`), `SolverProof` is one, and
`solver_nodes > 0` is true for `CompletedDepth` answers too.

#### BLOCKING 2 — the score's grammar; INVARIANT 7 makes an absent score silent

**Disposition: PARTIALLY APPLIED.** The substantive half is right; two of the
review's four named sub-fixes are missing.

**Applied and verified.**

- *Three spellings.* §4:145–148 names `cp <value>`, `mate <turns>` and
  `-mate <turns>`, and identifies the third as the one where the token carries
  the sign. Verified against `crates/pistol-cli/src/report.rs:153–159`
  (`ScoreKind::Eval` → `cp`, `MateIn` → `mate`, `MatedIn` → `-mate`, at `:157`).
  **Exact, all three, correctly attributed.**
- *The two-word read.* §4:150–155 states the value is a keyword plus a number and
  that branch B's widening parses the score as a PAIR. Verified against
  `crates/pistol-arena/src/exchange.rs:177–183`, whose `value()` returns the
  single word after a key. **Correct.**
- *The refusal.* §4:157–164 and INVARIANT 10 make an absent score fatal on the
  label path while INVARIANT 7 keeps it non-fatal for SPRT.

**The dispatch asked whether INVARIANT 10 contradicts INVARIANT 7. It does
not.** They are about different objects. INVARIANT 7 constrains the **parser**:
`totals_of` returns `Option`s so the three existing lookups stay load-bearing and
the SPRT path's output cannot move. INVARIANT 10 constrains a **consumer**: pass
2 refuses a position whose score did not parse. One `Option`, two callers, two
obligations — §4:162 says so in those words. **The one place the decision is
actually made is pass 2's record-writing consumer of `totals_of`'s return
value** — the call site that turns a `None` into a refusal instead of a blank
column. That is named, and it is the right place. This item is sound.

**Not applied (two of the review's four sub-fixes):**

1. *The fixture triple.* The review asked that
   `a_score_is_recorded_from_the_movers_point_of_view` be extended to a `cp` /
   `mate` / `-mate` triple, because a `cp`-only fixture leaves both mate
   spellings untested. Revision 2 instead added a separate
   `a_negative_mate_token_is_read_as_the_mover_being_mated`. That covers `-mate`
   but leaves plain `mate` untested by anything — the middle spelling now has no
   fixture at all, in a section whose entire argument is that the spellings differ.
2. *`pv`.* MINOR 3 asked that `pv` be dropped from the widening or its consumer
   named. INVARIANT 7 (line 324) still widens for `score` **and `pv`**, §3's
   table still has no principal-variation column, and no consumer is named. See
   NEW MAJOR 6 — the slot revision 2 now genuinely needs is `solver_nodes`, and
   it did not take it.

**And the refusal's blast radius is unstated.** INVARIANT 10 says pass 2
*"refuses a position ... by name, and writes no record for it."* Refuses the
POSITION or the RUN? "Writes no record for it" reads as skip-and-continue, which
is the skip-with-default rule 3 forbids; "refuses by name" reads as abort. The
review's own words were *"an absent or unparseable score is **FATAL** on the
label path"*. This is NEW MAJOR 3.

### MAJOR

#### MAJOR 1 — `label_sha256` does not identify a labelling run

**Disposition: PARTIALLY APPLIED, and one limb introduced a new defect.**

- **(a) the `--workers` premise — NOT APPLIED.** §5:172 still reads *"The arena
  deliberately excludes `--workers` from `experiment_sha256`"*. What
  `experiment_digest` excludes is the config key `run.n_workers`
  (`crates/pistol-arena/src/report.rs:41`ff); the `--workers` flag exists only in
  replay mode, which computes no `experiment_sha256`. The review asked for the
  one-word correction and it was not made.
- **(b) seed and sampling rule — APPLIED.** §5:181–187 adds both to the canonical
  concatenation and cites the right precedent. Verified:
  `crates/pistol-arena/src/report.rs:50` writes `openings_skip` into
  `experiment_digest` under the comment *"The skip decides WHICH games are
  played…"*. The reasoning transfers exactly.
- **(c) `source_sha256` → `experiment_sha256` — APPLIED BUT INTRODUCED A NEW
  DEFECT.** It removes the timing block from the identity, which was the
  complaint. But `experiment_digest` closes over config-level parameters only —
  openings body digest, take, skip, turn_cap, budget, SPRT bounds, and both
  engines' binary/config/weights digests — and **not over the games**. See NEW
  MAJOR 1: the review's prescribed `verdict_block` had neither problem.

#### MAJOR 2 — determinism re-run receipt absent; dedup's hash-order hazard

**Disposition: PARTIALLY APPLIED.** Three of four sub-fixes landed.

- INVARIANT 11 (byte-identical re-run) — **added**.
- `a_rerun_over_one_report_is_byte_identical` — **added**.
- The board key's ordering rule (§7:241–248) — **added**, but underspecified in a
  way that is now load-bearing. See NEW MAJOR 4.
- **The record-ORDER invariant — NOT ADDED.** The review asked for an invariant
  that the record order is the walk order (game index, then turn index) and is
  not a function of any map's iteration. Nothing in §10 says it. The board-key
  rule covers the key's *value*, not the corpus's *order*, and the order is what
  `body_sha256` is over.
- **The dispatch's registered mutant — SUBSTITUTED WITHOUT SAYING SO.**
  `wp20_dispatches.md` registers *"seed ignored -> determinism receipt dies"*.
  Revision 2's new mutant row is *"the seed dropped from `label_sha256`"* against
  the identity test. Those are different mutations. In revision 2's defence, the
  dispatch's mutant looks ill-posed — a same-seed re-run receipt survives a
  build that ignores the seed entirely — but a design that departs from a
  registered mutant says so and says why. This one is silent.

#### MAJOR 3 — INVARIANT 1 false for a capped game

**Disposition: PARTIALLY APPLIED, and the remedy discards information the
transcript carries.**

**Verified as the dispatch asked:** `crates/pistol-core/src/turn.rs:34–45` —
`Outcome` has exactly two variants, `Ongoing` and `Win`. Revision 2's claim is
exact. INVARIANT 1 is correctly restated (decided outcome from the referee;
undecided recorded as undecided), and
`an_undecided_replay_is_recorded_undecided_and_not_guessed` is added.

**But §2:61–65 says the design *cannot* tell a capped game from a forfeited one,
and that is false of the transcript.** `crates/pistol-arena/src/transcript.rs:20`
— `RecordedGame` carries `pub forfeit: bool`, set at `:307` from the report's own
`end` field. So the derivation is complete and available: a game the referee
reports `Ongoing` is **capped** if `forfeit == false` and **forfeited** if
`forfeit == true`. The arena's own result type has the third value already —
`crates/pistol-arena/src/record.rs:5–12`, `GameResult::{P1Win, P2Win, Capped}`.

Revision 2 collapses two mechanically distinguishable outcomes into one
`undecided` value, and makes the header count conflate them, while reading the
discriminator on the same struct. This is the same class as BLOCKING 1 — a
discriminator that exists and is not recorded — and it is NEW MAJOR 2.

Also not applied: the review asked that §3's table name the outcome column's
values. §3:97 still reads only *"from INVARIANT 1's referee"*.

#### MAJOR 4 — the book exclusion is stated once and pinned by nothing

**Disposition: NOT APPLIED, and revision 2 made it worse.** §7:223 remains the
only occurrence of *"non-book"*. It is in no invariant, no test, no mutant. And
the new INVARIANT 3 (*"Every position of every game is a label candidate"*) now
states the exclusion's negation. See NEW BLOCKING 2.

#### MAJOR 5 — which seat answers a label ask; no refusal of a two-engine report

**Disposition: NOT APPLIED.** `/usr/bin/grep -n "seat" docs/experiments/wp20_design.md`
returns lines 32, 116, 117, 118, 210 — none of them about which channel pass 2
asks, and none an invariant refusing a source report whose two seats attest
different `binary_sha256` / `config_sha256` / `weights_sha256`.

#### MAJOR 6 — §8 defers what D-518 registered in advance

**Disposition: PARTIALLY APPLIED.**

- **The split rule — APPLIED, and this is the item the dispatch asked about.**
  §8:286–291 moves it in as a mechanism: partition by POSITION, never by firing,
  a stated function of the position's own board key and of nothing about its
  outcome, held-out never read while the score is fitted. That is the right
  shape and it is correctly argued as a mechanism rather than a number.
  **However** the split is now a function of the board key, and the board key is
  underspecified in a way that decides whether the split leaks — NEW MAJOR 4.
- **The grid, floor and confidence — STILL DEFERRED**, with a new deadline.
- **D-518's off-the-end clause — STILL ABSENT.** §8 still says nothing about what
  happens when no grid point clears the floor, which is the precise moment a
  later session is tempted to lower it. The review named this as D-518 item 6 and
  revision 2 did not add it.
- **The deadline — the dispatch asked whether it is enforceable. Half of it is;
  half of it is prose.** §8:293–298: *"before the corpus is first counted against
  any candidate minimum, and the pipeline records the count in its manifest, so
  a registration arriving later is visibly later."* The **timing** is defensible
  — the grid ranges over candidate minima, so a sweep necessarily counts the
  corpus against them, which puts the deadline at roughly D-518's *"before the
  sweep"*. The **enforcement** is not: the quantity §8 counts is *win-proving
  firings on disjoint positions* (§8:261–262), which is countable only from
  census rows carrying position identity — WP-2.0b's scope by D-539 — and §7:253
  says the census corpus manifest *"stays EMPTY until WP-2.0b lands"*. So the
  pipeline this design specifies **cannot record the count its own deadline is
  enforced by**. NEW MAJOR 5.

#### MAJOR 7 — the label budget's KIND is unconstrained

**Disposition: NOT APPLIED.**
`/usr/bin/grep -n "movetime" docs/experiments/wp20_design.md` → no match. Nothing
in §5 says the label budget is one of the two instrument budgets, nothing refuses
a `movetime` label budget by name, and `BudgetSection::go_line` is not named as
the `go` line's builder. This remains a path to a wall-clock quantity inside a
label, against hard rule 4 and INVARIANT 11's own byte-identity claim.

#### MAJOR 8 — the label policy's CHOICE is deferred; the dispatch names it as the design's

**Disposition: NOT APPLIED.** §7:223–225 is unchanged word for word, and §12
still lists *"the label policy's sampling fraction"*. The dispatch's
"design decides and records" item is *"the label policy (which positions get deep
labels — **all, or a registered sampling rule**)"*; that choice is a mechanism
and is still not made.

### MINOR — compact

| # | finding | disposition |
|---|---|---|
| 1 | §2 never names `RecordedGame.forfeit`, the field that makes the skip implementable | **NOT APPLIED.** `/usr/bin/grep -n "RecordedGame" …` → no match. The reversal did not dissolve this: §7 still excludes forfeited turns, so the field is still needed, and MAJOR 3's remedy needs it too |
| 2 | the forfeit skip's cost neither shaped nor traded | **APPLIED** in §2:66–71 — the reversal is argued rather than asserted. Contradicted elsewhere (NEW BLOCKING 2) |
| 3 | `pv` widened into `totals_of` with no consumer | **NOT APPLIED.** INVARIANT 7:324 still widens for `pv`; §3 has no PV column; no consumer named. The review's suggested better use of the slot — `solver_nodes` — is now required by §4a and still absent (NEW MAJOR 6) |
| 4 | the loader has no home | **NOT APPLIED.** §3 still names only the sink (`pistol_cli::corpus::emit::Fixture`); three §11 tests are loader tests with no crate |
| 5 | `widening_totals_of_leaves_the_sprt_report_byte_identical` is not the test that proves it | **NOT APPLIED.** Test name unchanged at line 346; `verdict_block` appears nowhere in the document; the suggested `totals_of` unit test was not added |
| 6 | pass 2's own failure modes and process lifetime undesigned | **NOT APPLIED**, and now sharper: INVARIANT 10 adds a refusal without saying whether it aborts (NEW MAJOR 3). `/usr/bin/grep -n "abort" …` → no match |
| 7 | the new mode's CLI grammar is not named | **NOT APPLIED.** No usage line anywhere |
| 8 | §8's counting unit does not exist until WP-2.0b, and §8 does not say so | **NOT APPLIED**, and now load-bearing because §8's new deadline is enforced by that count (NEW MAJOR 5) |
| 9 | per PLY or per TURN answered only by implication | **NOT APPLIED.** No sentence states the unit; `replay.rs:137` is not cited |
| 10 | dedup silent on two records with the SAME move list | **NOT APPLIED.** §7:234 unchanged |
| 11 | the ledger's same-commit rule sits against the preamble | **NOT APPLIED.** Line 19 (*"No committed config moves"*) and §7:250–251 both unchanged |

**Tally: 1 applied, 10 not applied.** A MINOR left unfixed is a MINOR, and none
of these alone would fail the revision — but MINOR 3, 6 and 8 have each been
promoted into a MAJOR by a remedy that landed on top of them.

---

## PART 2 — new defects introduced by the remedies

### NEW BLOCKING 1 — `solver_nodes > 0` means "the solver ran", not "the answer is a proof": INVARIANT 8 is false, and INVARIANT 9 is contradicted by §3's own sourcing

This is the dispatch's item 1, and it is the harder question it invited. The
answer is that the mark is mis-named.

**`Provenance` has four variants**, not two —
`crates/pistol-search/src/info.rs:250–266`: `CompletedDepth`, `PartialRoot`,
`SolverProof`, `Fallback`. Only `SolverProof` is the hazard §4a describes.

**`SolverProof` is produced at exactly one site**, and it is an early return.
`crates/pistol-search/src/search.rs:319` returns `solver_proof_outcome(...)` when
the ROOT attacker call proves a win, before any deepening. That function
(`:796`) sets `search_nodes: 0` and `nodes: solver_nodes`.

**The ordinary path sets the same solver fields non-zero.** When the root call
does *not* prove, the search proceeds and the search-path solver fires at
interior nodes — `crates/pistol-search/src/pvs.rs:674` (`solver.solve`) and
`:705` (`solver.solve_defender`) — accruing `run.solver_nodes`. The completed
iteration then constructs its outcome at `search.rs:404–420` with
`search_nodes: run.search_nodes`, `solver_nodes: run.solver_nodes`, and
`provenance: Provenance::CompletedDepth`.

**So under any of the three gate-on configs §4a itself names as legal**, a plain
search answer prints the solver fields, and INVARIANT 8 (*"A record whose totals
line carried the solver fields is marked as solver-provenance"*) marks it
solver-provenance. Its `depth_turns` is a search depth. Its score is a search
score. Nothing about it is a proof. **The column §4a exists to make trustworthy
is wrong for the majority of gate-on positions**, and the registered test
(`a_solver_provenance_answer_is_marked_and_its_nodes_kept_separate`), written
against a solver-proof fixture, passes anyway. That is the prior BLOCKING 1's
own criterion — a wrong value in the corpus that no test catches — reproduced by
its remedy.

**The second limb: the record sums what INVARIANT 9 says it never sums.**
`crates/pistol-search/src/pvs.rs:150–151`:

```
pub fn total_nodes(&self) -> u64 {
    self.search_nodes + self.solver_nodes
```

and `search.rs:404` / `:513` set `info.nodes = run.total_nodes()`. `info.rs:169`
says it in the field's own doc: *"`nodes` is their derived sum at report time"*.
§3:95 sources `label nodes` from *"`nodes` from the totals line"*. **So the
record's node column IS `search_nodes + solver_nodes`**, while INVARIANT 9 states
*"Search nodes and solver nodes are never summed into one column."* One of the
two is false. §3's table has one node column; INVARIANT 9 presupposes two.

**FIX, and it is small.** (1) The discriminator is `search_nodes == 0`, not
"the solver fields appeared" — it is on the same wire, inside the same
conditional, and under an instrument budget only `CompletedDepth` and
`SolverProof` are reachable (the salvage arm is `Stop::Deadline`-only,
`search.rs:437–444`), so `search_nodes == 0` ⟺ `SolverProof` exactly. State
INVARIANT 8 in those terms. (2) Give §3 two node columns — `label search nodes`
and `label solver nodes` — sourced from the fields of those names, and let
INVARIANT 9 be about the schema it describes. (3) Note the one residual: a proof
costing zero solver nodes would print no fields at all; say whether that is
possible or refused. **The exact run that would settle (3):**
`cargo test -p pistol-search --test wp18b_solver_path_tests` in a detached
worktree — `wp18b_solver_path_tests.rs:130` already asserts
`outcome.info.search_nodes == 0` on the proof path, which is the discriminator
this design should be reading.

### NEW BLOCKING 2 — the forfeit reversal was applied in two places out of four; §11 registers a test that pins the negation of INVARIANT 3, and INVARIANT 3 silently deletes the book exclusion

The reversal itself is sound and safe (see *the strongest attack that did not
land*). It was not carried through the document.

RECORDED — `LC_ALL=C /usr/bin/grep -n "forfeit\|non-book\|candidate" docs/experiments/wp20_design.md`,
the load-bearing rows:

```
223:**LABEL POLICY.** Every to-move position of every non-forfeited, non-book turn
316:3. **Every position of every game is a label candidate**, forfeited games
340:- `a_forfeited_game_contributes_no_records_and_is_counted`
```

Three statements, and no reading reconciles them:

- **§7:223 vs INVARIANT 3** — one excludes forfeited games, the other includes
  them by name. Revision 2 rewrote INVARIANT 3 and left the section that owns
  the candidate rule saying the opposite.
- **§11:340 vs INVARIANT 3** — the test list still registers the rev-1 test whose
  behaviour INVARIANT 3 now forbids. An implementer working from §11 and an
  implementer working from §10 produce different corpora, and the first one's
  test suite fails against the second one's code.
- **INVARIANT 3 vs the book exclusion** — *"Every position of every game"* drops
  *non-book* as well as *non-forfeited*. So the one invariant that now states the
  candidate rule states it without the exclusion that removes `opening_turns`
  turns from every game. MAJOR 4 asked for that exclusion to be pinned; the
  revision instead wrote an invariant that contradicts it, and the exclusion is
  still in no test and no mutant. **That half is silent**, not loud: a build
  faithful to INVARIANT 3 labels the paired book's shared prefix in every game,
  duplicating it across the whole run, and nothing in §11 notices.

I record that this is loud rather than silent in its forfeit half — an
implementer hits it on the first read — and that the prior review's BLOCKING bar
was silence. It is BLOCKING regardless, because *which positions are in the
corpus* is dispatch requirement 2 and a "design decides and records" item, and
after the fix round the design does not say.

**FIX.** Three edits, all mechanical: §7:223 → *"Every to-move position of every
non-book turn is a candidate, forfeited games included"*; §11:340 → delete, or
replace with `a_forfeited_games_positions_are_labelled_and_its_outcome_marked`;
INVARIANT 3 → restore the book exclusion with the boundary written as
`at >= opening_turns` (`crates/pistol-arena/src/replay.rs:137`).

### NEW MAJOR 1 — `label_sha256` no longer closes over the games

`experiment_digest` (`crates/pistol-arena/src/report.rs:41–74`) hashes:
`openings_body`, `openings_take`, `openings_skip`, `turn_cap`, `budget`, the four
SPRT bounds, and per slot the engine's label, `binary_sha256`, `config_sha256`
and `weights_sha256`. It is a digest of the experiment's **parameters**, not of
its **outcome**.

Revision 1's `source_sha256` covered the games but also the timing block; §5:189–194
correctly says so and swaps it out. But the review's prescribed fix was neither
— it was `pistol_arena::report::verdict_block` (`report.rs:231`), and
**`verdict_block` has neither problem**. `render` (`report.rs:106–124`) writes
`instrument` → `conclusion::games` → `found` → `timing`, and `verdict_block`
returns everything before the `# timing` marker. So it excludes exactly the
machine-dependent block revision 2 wanted gone **and** closes over every `game`
record, every `moves` line, and `arena_version`.

Under revision 2's choice, two source reports of one experiment whose games
differ — because the arena binary that played them changed, with the engine
binaries' digests unmoved — carry one `experiment_sha256`, and two different
corpora carry one `label_sha256`. That is MAJOR 1(b)'s own criterion (*"a digest
that collides across genuinely different runs is not an identity"*) surviving in
narrowed form, in the limb that was supposed to fix it.

**FIX.** Digest `verdict_block(source_text)` in place of `experiment_sha256`, or
alongside it. The crate already exports it, and its doc line is *"The part of a
report two worker counts must agree on, byte for byte."*

### NEW MAJOR 2 — the `undecided` outcome discards a discriminator the transcript already carries

See MAJOR 3's disposition above for the mechanism (`transcript.rs:20`, `:307`;
`record.rs:5–12`). §2:61–65 states a limitation that is true of the replay and
false of the reader, and the design then declines to record what it can
establish. The count of `undecided` games conflates caps with forfeits in a
`derived` header field a consumer is told to filter on.

**FIX.** Record three outcome values — decided (with winner), capped, forfeited —
deriving the last two from `RecordedGame.forfeit` beside the referee's `Ongoing`,
and say in INVARIANT 1 that the referee supplies the WIN and the report supplies
the kind of non-win. That is the review's original MAJOR 3 fix, unchanged.

### NEW MAJOR 3 — INVARIANT 10's refusal does not say whether the run survives it

*"Pass 2 refuses a position … by name, and writes no record for it"* admits both
"abort the run" and "skip this position and continue". The second is the
skip-with-default hard rule 3 forbids, and the review's wording was FATAL. MINOR
6 asked for the same paragraph for a failed label `go`
(`exchange::ask`'s four forfeit classes have no meaning in a pass that is not
playing a game) and was not applied, so the document now has two refusal paths
and says of neither whether it ends the run.

**FIX.** One sentence: a position whose score does not parse, and a label `go`
that fails, both **abort pass 2 by name**; no record is written and no partial
corpus is emitted.

### NEW MAJOR 4 — the board key's canonicalisation is unspecified, and §8's split now depends on it

§7:246–248: *"The key is derived from the board's own contents in a sorted,
canonical order."* The tree has two different things that phrase could name, and
they give different answers:

- `sorted_stones` (`crates/pistol-cli/src/corpus/openings.rs:231–235`) — stones
  sorted, no symmetry folding;
- `canonical_of` (`:238–240`) — `canonical_form(&sorted_stones(state))`,
  `pistol_core::canonical_form` (`crates/pistol-core/src/symmetry.rs:165`), folded
  to the twelve lattice symmetries.

In revision 1 the ambiguity cost only dedup precision. In revision 2 it is
load-bearing twice over. §8:286–289 makes the FIT/HELD-OUT partition *"a stated
function of the position's own board key"*. **Under the unfolded key, a position
and its mirror image get different keys and can land in different halves — and
they are the same position, with the same value.** That is a leak of exactly the
kind §8 step 1 exists to forbid (*"never by firing, because firings repeat
positions and a split by firing would leak"*), arriving through the door the
remedy opened.

`pistol-arena` already depends on `pistol-core`
(`crates/pistol-arena/Cargo.toml`), so `canonical_form` is one import away.

**FIX.** Name it: either *"the board key is `pistol_core::canonical_form` over
the sorted stones"* — one clause, implementable, and
`a_board_key_does_not_depend_on_iteration_order` becomes testable against a
transposed AND a reflected fixture — or state that the key is deliberately not
symmetry-folded and say why the split does not leak.

### NEW MAJOR 5 — §8's deadline is enforced by a count this package cannot produce

Detailed under MAJOR 6's disposition. The timing clause is defensible; the
enforcement clause (*"the pipeline records the count in its manifest, so a
registration arriving later is visibly later"*) names win-proving firings on
disjoint positions, which need WP-2.0b's census position identity (D-539's second
obligation), and §7:253 says this package's census manifest stays empty. The
sentence's second half is prose.

**FIX.** Either name WP-2.0b as the enabling package and bind the deadline to
*that* pipeline's manifest, or — better, and the review's own wording — set the
deadline at *before any recall is computed on any part of the corpus*, which
needs no manifest to be checkable because it is a fact about what the
registration's own document may cite.

### NEW MAJOR 6 — §4a's mechanism needs fields off the totals line that INVARIANT 7 does not widen for

§4a:125–127 has pass 2 read the provenance and *"the solver node count"* from the
totals line. INVARIANT 7:323–324 widens `totals_of` for `score` and `pv` and
nothing else. So the design's own parser statement does not admit the two fields
its new column depends on, while it does admit a field (`pv`) with no consumer —
which is MINOR 3, left unfixed, now blocking the BLOCKING 1 remedy.

**FIX.** Widen for `search_nodes` and `solver_nodes`; drop `pv`. This is the swap
MINOR 3 proposed, and it costs one line of INVARIANT 7.

### NEW MINOR

- **NM-1.** `a_labelling_run_is_identified_by_its_source_report_and_its_budget`
  (§11:344) keeps its revision-1 name while §5 now hashes five things including
  the seed, and it is the registered killer for the *"seed dropped from
  `label_sha256`"* mutant — a test whose name does not mention the property it is
  the sole guard for. Tests are named for the behaviour pinned.
- **NM-2.** INVARIANT 6 (*"Pass 2 never plays a move"*) still has no test, and the
  new board-key rule (§7:241–248) is stated as prose while being strong enough
  that revision 2 says violating it would make INVARIANT 11 fail intermittently.
  One is an invariant with no test; the other is a test with no invariant.
- **NM-3.** The reversal makes the walk's end boundary matter where it did not
  before: a forfeited game's *last* position — the one at which a mover forfeited
  — is a genuine to-move position that pass 1 never answered, and §1's walk over
  recorded moves yields prefixes `0..n-1`, so it is excluded silently. Worth one
  clause, since it is the position a forfeited game's information sits at.

**D-483 (dispatch item 8): no number crept in.** Revision 2's new figures are
`three committed configs` (a tree measurement, stated with the command that
produced it — D-543's remedy), and D-518's `nine-point grid` / `0.90` /
`ceil_to_500(P + 500)`, quoted as history about another decision while §8:293
explicitly refuses to adopt any of them for this one. No number a WP-2.0
pre-registration or gate would consume is fixed here. **Compliant.**

**Internal consistency (dispatch item 9).** Eleven invariants, seventeen tests.
Every invariant has a test except **INVARIANT 3** (its listed test pins the
negation — NEW BLOCKING 2) and **INVARIANT 6** (none — NM-2). Six tests pin
things no invariant states (three loader tests, the transposition test, the
ledger test, the board-key test) — acceptable, since §3 and §7 own those in
prose. Invariants contradicted: **8** (by the code), **9** (by §3's own
sourcing), **3** (by §7 and §11). **INVARIANT 10 vs 7 is NOT a contradiction** —
see BLOCKING 2's disposition.

---

## PART 3 — is the design now implementable?

**No.** An implementer starting from revision 2 must decide, unaided, at least
these ten things the design was supposed to decide — four of them named by the
dispatch's own "design decides and records" block or by requirement 2:

1. **Whether forfeited games are labelled.** §7, §10 and §11 disagree. *(dispatch requirement 2, "which positions")*
2. **Whether book turns are labelled.** §7 says no; INVARIANT 3 says every position.
3. **Whether the policy labels everything or samples.** Still deferred. *(dispatch "design decides and records")*
4. **How many node columns the record has**, and what each holds. §3 says one, INVARIANT 9 presupposes two.
5. **How provenance is determined.** The stated predicate is false; the implementer must find the right one, which is the design's job.
6. **Which seat pass 2 asks**, and whether a two-engine source report is refused.
7. **Which budget kinds the label budget admits**, and whether `movetime` is refused.
8. **Whether a refusal aborts pass 2 or skips the position**, and what a failed label `go` does.
9. **Whether the board key folds the twelve lattice symmetries** — which decides whether §8's split leaks.
10. **Which crate owns the loader**, and the new mode's CLI grammar.

Items 1, 2, 4 and 5 are worse than open: the document answers them **twice, and
differently**. An implementer who resolves them by picking one side is making a
design decision under a document that appears to have made it, which is the
condition this review exists to prevent.

The **shape** remains implementable and remains, in my reading, right. The record
half — which is what revision 1 failed on and what revision 2 set out to rebuild
— is not.

---

## The strongest attack that did not land

**The forfeit reversal, attacked exactly where the dispatch aimed me: can a
forfeited game's recorded move list be truncated or illegal in a way that makes
pass 2 panic or refuse?**

This was the right thing to be worried about. A forfeit is a fact about the
protocol, not about the position — `exchange::ask` classifies an exited engine, a
protocol error, a bad bestmove and an overlong line as forfeits, none of which
says anything about whether the moves recorded so far form a legal game. Revision
1 skipped these games; revision 2 walks them. That is a reversal into precisely
the territory where a reversal breaks something, and the design offers no
argument that the walk is safe. I expected to find the finding here.

**It is safe, and the reason is upstream of the design, which is why revision 2
could not have known to say it.** Pass 2 does not reach a game's moves through
any path of its own: it reads a `Transcript`, and `transcript::read` legality-
checks every game **at read time, on the document**, before pass 2 exists.
`read_moves` (`crates/pistol-arena/src/transcript.rs:329`) ends with
`replays(index, &moves)?` at `:344`, and `replays` (`:359–378`) walks the list
through `GameState::make_turn` — pistol-core, the only judge of legality in this
workspace — refusing the whole report if any recorded turn is illegal in the
position its predecessors reach, and refusing again if a turn wins with further
turns recorded after it. Its own doc says why the check lives there rather than
part-way through a replay: *"a report it cannot answer about costs exit 2 and no
document, and a legality refusal discovered at turn nine … would have already
produced part of an answer."*

So the three failure modes I went looking for are all closed before pass 2 runs.
A forfeited game's move list is a **legal prefix** of a game, guaranteed, or the
report never becomes a `Transcript`. It cannot contain an illegal turn. It cannot
contain moves after a win. An empty one (`turns 0`) replays trivially and
contributes zero candidates rather than crashing. **Pass 2 cannot panic on a
forfeited game, and the positions revision 2 reclaims are exactly as ordinary as
§2:66–71 claims they are.**

The reversal is the best thing in revision 2. It recovers real corpus, it is
argued rather than asserted, and it stands on a guarantee the tree already makes.
That the document then failed to carry it into §7 and §11 is NEW BLOCKING 2 — but
that is a clerical failure of the edit, not a defect in the decision, and the
decision was correct.

**A second attack that did not land: INVARIANT 10 against INVARIANT 7.** The
dispatch invited me to find a contradiction between a non-fatal `Option` in
`totals_of` and a fatal absent score on the label path, and there is none. They
constrain different objects — a parser and one of its two consumers — and §4
says so in its own words before I could. `totals_of`'s three existing lookups
stay load-bearing, `Compute::add` is its only existing consumer, and pass 2's
record-writing step is the one place the label path's obligation is discharged.
This is the best-argued paragraph in the document and I could not break it.

**A third: whether §4a's remedy needs an engine diff.** If the provenance were
genuinely off the wire, the whole remedy would collapse into option 1 (refuse
gate-on configs) and the *"nothing new reaches the wire"* license would be in
question. It does not: `search_nodes` is printed inside the same conditional as
`solver_nodes`, and `search_nodes == 0` separates a root proof from a completed
depth exactly. **The discriminator is on the wire, and revision 2 was right that
its remedy costs no engine diff — it simply read the wrong field.** That is why
NEW BLOCKING 1's fix is one predicate and not a re-scope.
