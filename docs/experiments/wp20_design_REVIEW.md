# REVIEW-design — `docs/experiments/wp20_design.md`

## Header

**Subject, at its named revision:** `docs/experiments/wp20_design.md` at
**`bb44d6fbd62f2df706989953d7072d513210ce28`** on `dev`
(*"docs(wp20): the design is two passes over an arena report — coldness is a
property of the shape, and the census-minimum rule lands before any corpus
exists"*).

**HEAD match:** `git rev-parse HEAD` → `bb44d6fbd62f2df706989953d7072d513210ce28`.
**The subject IS HEAD.**

**Tree state:** `git status --porcelain` → empty. Clean.

**Fresh context.** This session did not write the design, the selection, the
matrix or the memo.

**What I ran.** `git`, `git grep`, `/usr/bin/grep`, `sed`, `awk`, `wc`, `ls`,
`find`. Every recorded grep is `/usr/bin/grep` or `git grep`, `LC_ALL=C sort`ed
where it enumerates (D-265).

**What I refused.** No `cargo` in any form, and no `tools/ci.sh`, per the
dispatch. Two findings below would be settled faster by a build; each names the
exact run instead.

**Read, in the order the dispatch set:** `CLAUDE.md`, `docs/process.md`,
`wp20_dispatches.md` (WP-2.0), `matrix_wp20_shape_selection.md` (incl. §4),
`wp20_premise_memo.md` (esp. §"WHAT THE DESIGN MUST DECIDE"),
`docs/decisions.md` D-6/8/80/166/200/291/374/424/469/477/483/500/511/518/527 and
D-535…D-543, `artifacts/matrix_wp20_shape_REDTEAM_round2.md`,
`docs/book_v2_ledger.md`, `docs/experiments/book_v2_registration.md` §4/§4.1.

**Code read to verify mechanism claims:** `crates/pistol-arena/src/`
{`bin/arena.rs`, `transcript.rs`, `replay.rs`, `record.rs`, `conclusion.rs`,
`exchange.rs`, `seats.rs`, `validate.rs`, `report.rs`, `config.rs`, `game.rs`},
`crates/pistol-arena/tests/run_tests.rs`, `crates/pistol-cli/src/`
{`corpus/emit.rs`, `corpus/mod.rs`, `report.rs`, `protocol.rs`, `lib.rs`},
`crates/pistol-search/src/` {`search.rs`, `tt/mod.rs`, `position.rs`, `pv.rs`,
`pvs.rs`, `info.rs`}, `crates/pistol-solver/src/solver.rs`,
`crates/pistol-engine/src/instance.rs`, `crates/pistol-core/src/`
{`turn.rs`, `state.rs`}, and every file in `configs/`.

---

## VERDICT: **FAIL**

**2 BLOCKING, 8 MAJOR, 11 MINOR.**

The SHAPE is right and I could not break it. Two passes, coldness as a property
of the shape rather than of a check, the unmodified SPRT path in pass 1, the
`pistol_cli::corpus::emit::Fixture` sink, the branch-B construction with the
three existing lookups load-bearing — I attacked all of these and they hold
(see *the strongest attack that did not land*). Every one of the selection's
five owed items is at least attempted, and D-483 is honoured to the letter.

It fails on **what the record CONTAINS**, not on how it is produced. Two of the
eight columns in §3's table — `label depth`/`label nodes`, and `label score` —
are specified in a way that lets the pipeline write a wrong value into the
corpus with no loader test able to catch it, which is the exact failure mode the
design's own §4 says it exists to prevent. One of those (BLOCKING 1) is the
premise memo's forced decision 9, which the design neither takes nor defers, and
whose escape hatch the memo offered is **false at HEAD**.

**ONE FIX ROUND REMAINS.** Every finding below carries its fix, so the round is
spendable in one pass. A second failure is a STOP and a split.

---

## Requirement coverage — the dispatch's Scope

| # | Requirement (dispatch, `wp20_dispatches.md:70-83`) | Status |
|---|---|---|
| 1 | Plays self-play games: committed config vs committed config, `book_v2` openings, instrument mode, node budgets; GAME and LABEL budgets separate registered values | **DELIVERED, with a gap.** §1 PASS 1 rides the unmodified path; §5 gives the label budget a home. Gap: the label budget's **kind** is unconstrained — MAJOR 7 |
| 2 | One record per position: canonical move list (D-6), side to move, deep label (score + best move + depth + nodes), game outcome; documented versioned schema with a loader test | **GAP.** Schema, version and loader tests are present (§3, §11). Three of the eight columns are unsound or unsourced: BLOCKING 1 (depth/nodes), BLOCKING 2 (score), MAJOR 3 (outcome, capped games). Loader has no named home — MINOR 4 |
| 3 | Census logging on every game per D-53a | **CORRECTLY NOT DELIVERED.** Left the package by D-539; the design cites it at §7 and keeps the census manifest empty. No finding |
| 4 | Deterministic end-to-end given (seed, book range, config, SHA): a re-run receipt proves byte-identical output on a small range | **NOT ADDRESSED.** No test, no invariant, no deferral — MAJOR 2 |
| 5 | Ledgers: book ranges consumed (shared with the SPRT ledger), corpus manifest with digests, census corpus manifest | **DELIVERED.** §7 names all three, append-only, census one empty. Two riders: the run-identity digest is wrong in both directions (MAJOR 1) and the ledger's same-commit rule sits against the preamble (MINOR 11) |

### "Design decides and records" (`wp20_dispatches.md:88-96`)

| item | status |
|---|---|
| Storage format and schema version | **DECIDED** — §3, `Fixture`, version as a `param`, loader refuses an unknown one |
| Label policy (which positions get deep labels — **all, or a registered sampling rule**) | **NOT DECIDED** — §7 defers the *choice*, not merely the fraction — MAJOR 8 |
| Dedup policy for transposed positions, by canonical move list, stated | **DECIDED** — §7, transpositions kept as two with a shared board key. Same-move-list collision unstated — MINOR 10 |
| Census minimum rule per D-53a, the RULE landing now so it cannot be fitted later | **PARTLY DECIDED** — §8 fixes the form and defers what D-518 registered in advance — MAJOR 6 |
| Throughput as a SHAPE, measured in the pilot, never guessed (D-500) | **DECIDED** — §9, and §6 refuses to guess the memset cost. Clean |

### "Development round" §2's test list and mutants (`wp20_dispatches.md:100-107`)

| dispatch item | design |
|---|---|
| schema roundtrip | ✓ `a_record_round_trips_through_the_loader` |
| loader rejects malformed records loudly | ✓ two named tests |
| **determinism re-run receipt** | **ABSENT** — MAJOR 2 |
| ledger append-only behaviour | ✓ `the_ledger_appends_and_never_rewrites` |
| census records carry identity and direction | out of scope (D-539) ✓ |
| mutant: schema field dropped → loader test dies | ✓ |
| **mutant: seed ignored → determinism receipt dies** | **ABSENT.** §11 substitutes the *inverse* mutation ("the seed read on a label-everything policy") against the seed-inertness test — MAJOR 2 |
| mutant: ledger overwrite → append test dies | ✓ |
| mutant: census direction collapsed | out of scope (D-539) ✓ |

---

## The selection's §4 "what the design now owes" — item by item

| # | Owed (`matrix_wp20_shape_selection.md:94-107`) | Verdict |
|---|---|---|
| 1 | **D-540's second clause** — a fresh-process agreement criterion in the pilot's prereg, naming the defect class it excludes | **DISCHARGED, and well.** §6's closing block hands it to the pilot, states it must name the defect class, and cites `docs/process.md`'s "a criterion that is a property the named defect class PRESERVES" with D-527 as the precedent. This is the item the round-2 red team's M5 was about and it is answered in its own terms |
| 2 | **The seed (memo decision 5)** — D-540 fixes seeds to pipeline SAMPLING only; the design says what is sampled | **DISCHARGED.** §7 SEEDS + INVARIANT 5. The seed's one job is which candidates get labelled; inertness under a label-everything policy is pinned rather than asserted. Rider: the seed is outside §5's digest (MAJOR 1) and has no re-run receipt (MAJOR 2) |
| 3 | **The label budget's home (memo decision 4)** — relocated, not dissolved; the design says what identifies a labelling run | **ATTEMPTED, UNSOUND.** §5's `label_sha256` both omits inputs that change the output and includes bytes that cannot — MAJOR 1 |
| 4 | **What `transcript::read` drops** — `result`, `forfeit_by`, `openings_*`; recoverable through pistol-core except for forfeited games; the design says how | **ATTEMPTED, INCOMPLETE.** §2 is accurate about what the reader drops (verified field-by-field against `transcript.rs:29-53`). The recovery is unsound for capped games (MAJOR 3) and never names the field that makes the forfeit skip implementable (MINOR 1) |
| 5 | **The census-minimum rule (D-537)**, landing before any corpus exists | **ATTEMPTED, WEAKER THAN ITS OWN MODEL** — MAJOR 6 |
| — | **Not re-openable**: which crate owns the pipeline | **RESPECTED.** §1 puts it in `arena`; nothing re-opens it |

### Did the design re-introduce anything the round-2 red team corrected?

**No.** Checked item by item.

- **RT MINOR 1** (the `19 files / 4,240 lines` pair that reproduced as neither):
  the design carries **no** tree measurement at all. Clean.
- **RT MAJOR 1** (`replay`'s `walk` listed as *"Public and reusable"*): the
  design says pass 2 *"walk[s] every game's recorded moves"* and never claims
  reuse of the function. Verified still private —
  `/usr/bin/grep -n "fn walk(" crates/pistol-arena/src/replay.rs` → `113:fn walk(`;
  `/usr/bin/grep -c "pub(crate)" crates/pistol-arena/src/replay.rs` → `0`.
- **RT MAJOR 2** ("the only row of which that is true in every sense"): absent.
- **RT MAJOR 3** (the unpriced branch fork): §1 and INVARIANT 7 take branch B by
  name with its construction. Correct.
- **RT MAJOR 4** (what `transcript::read` drops): §2 addresses it.
- **RT MAJOR 5 / 6** (half of D-540; coldness's unmarked cost): §6 discharges
  both, and explicitly refuses to guess the memset.
- **RT MAJOR 7 / 8 / 10** (decision 4; the seed; no sink named): §5, §7, §3.
- **RT MINOR 2** (`transcript.rs:200` is a **second** producer of the `go` line):
  **not re-introduced, but not headed off either** — pass 2 needs a third. See
  MAJOR 7.

---

## The premise memo's twelve forced decisions

| # | Decision | Status |
|---|---|---|
| 1 | Where the census comes from | **TAKEN by reference** — out of scope (D-539), cited at §7 |
| 2 | How a census row acquires position identity | **OUT OF SCOPE** (WP-2.0b). But §8's counting unit depends on it and §8 does not say so — MINOR 8 |
| 3 | Extend `pistol-arena` or sit beside it | **TAKEN** — §1, row (g), D-542 |
| 4 | How GAME and LABEL budgets are both expressed | **TAKEN** — §5. Unsound as written (MAJOR 1), and the budget's kind is unconstrained (MAJOR 7) |
| 5 | Where the seed goes and what it seeds | **TAKEN** — §7, INVARIANT 5 |
| 6 | Which `book_v2` range WP-2.0 consumes | **DEFERRED WITH REASON** — §12, to the pilot prereg, under D-483. Correct |
| 7 | How the pipeline detects game over | **TAKEN** — INVARIANT 1, pistol-core as referee (rule 2). Wrong for capped games — MAJOR 3 |
| 8 | Process isolation across games, given D-7 | **TAKEN IMPLICITLY** — §6 chooses `newgame` per label, which subsumes per-game isolation. The process **lifetime** across pass 2 is never stated, and it is what §9's cost shape and D-540's fresh-process criterion both stand on — MINOR 6 |
| 9 | **What the label's `depth` and `nodes` MEAN, given `Provenance` is off the wire** | **SILENTLY DROPPED** — not taken, not deferred, not in §12's list. **BLOCKING 1** |
| 10 | What the score field is CALLED | **TAKEN** — §4, and it is the best-argued section in the document. Incomplete on the third spelling and on the parse's shape — **BLOCKING 2** |
| 11 | One record per PLY or per TURN | **TAKEN IMPLICITLY**, never stated. §3 says "turn index", §7 says "non-book turn", INVARIANT 6 says "advances along the recorded list" — the answer is per-TURN and is nowhere written down. D-477 is the ADR that exists because an unstated unit axis cost two red-team rounds — MINOR 9 |
| 12 | What the corpus manifest digests | **TAKEN, thinly** — §7 "per-file digests", §5 `label_sha256`. MAJOR 1 |

**One silently dropped decision: 9.** It is BLOCKING 1.

---

# FINDINGS

## BLOCKING

### BLOCKING 1 — `label depth` and `label nodes` silently mean two different things; memo decision 9 is dropped, and the escape the memo offered is FALSE at HEAD

`docs/experiments/wp20_design.md:80-81` — §3's record table:

> | label depth | `depth_turns` from the totals line |
> | label nodes | `nodes` from the totals line |

The premise memo forced this decision (`wp20_premise_memo.md:1508-1512`,
decision 9):

> Under a node budget a `SolverProof` label's depth is a proof depth and its
> nodes are solver nodes; the schema must either record provenance (needing it
> on the wire) or the seat must guarantee the gate is off (**which every
> committed config already does**, `instrument_staged_snk_v0.toml:78`).

The design does neither, and does not defer it — §12's list of five deferrals
does not contain it.

**The mechanism, verified at source.** `crates/pistol-search/src/search.rs:777-806`,
`solver_proof_outcome`:

```
depth_turns: depth,          // = tree.win_depth_turns() — a PROOF depth
nodes: solver_nodes,         // solver nodes, not search nodes
search_nodes: 0,
score: crate::score::mate_in(2 * depth - 1),
provenance: Provenance::SolverProof,
```

`Provenance` never reaches the wire, so a record built from the totals line
cannot tell this apart from a completed-depth answer.

**And the memo's escape hatch does not hold.** RECORDED:

```
$ LC_ALL=C /usr/bin/grep -rn "on_search_path = true" configs/ | LC_ALL=C sort
configs/bench_wp18c_solver_on.toml:45:on_search_path = true
configs/gate_staged_solver_v0.toml:47:on_search_path = true
configs/play_staged_solver_v0.toml:75:on_search_path = true
```

Three committed configs carry the gate ON. `configs/play_staged_solver_v0.toml:71-74`
says so in its own words. §1 requires PASS 1's two engine sections to name "the
same committed config" and does not say **which** — so naming a gate-on config
is a legal use of this design, and the corpus it produces has a `depth` column
that is sometimes a search depth and sometimes a proof depth, with no
discriminator recorded. The memo's own note on this is the reason it matters:
it makes the label's soundness *"depend on a CONFIG property … and a config is
exactly the thing a future SPRT is licensed to change."*

**FIX** (either, one sentence each):

1. State in §1 that pass 2 **refuses by name** a source report whose engine
   config has `solver.on_search_path = true`, and add an invariant; or
2. Record the discriminator, which is already on the wire and costs nothing:
   `crates/pistol-cli/src/report.rs:60-79` emits `search_nodes` /
   `solver_nodes` / `solver_firings` / … **only when the solver ran**, so their
   presence *is* the provenance. Add a `label provenance` column sourced from
   them and widen `totals_of` for `solver_nodes` instead of for `pv`
   (see MINOR 3).

Option 2 is the better one: it survives a config change, where option 1 depends
on nobody editing a `.toml`.

---

### BLOCKING 2 — the score's grammar is three spellings and two words; §4 names two spellings and one word, and INVARIANT 7 makes an absent score silent

`docs/experiments/wp20_design.md:88-103` (§4) and `:220-221` (INVARIANT 7).

§4 is the section whose whole job is that *"a wrong name here makes every label
in the corpus wrong in a way no loader test catches"* (`:102-103`). It names
`cp` (`:93`) and `mate T` (`:97`). It does **not** name `-mate`.

**The wire, verified.** `crates/pistol-cli/src/report.rs:153-159`:

```
pub fn score_token(score: i32) -> String {
    match classify(score) {
        ScoreKind::Eval(value)    => format!("cp {value}"),
        ScoreKind::MateIn(turns)  => format!("mate {turns}"),
        ScoreKind::MatedIn(turns) => format!("-mate {turns}"),
    }
}
```

Three spellings. `-mate` is **the one spelling where the sign is carried by the
token rather than by the number** — which is exactly what INVARIANT 4
(`:215`) pins. The section that fixes the sign convention omits the token that
carries the sign for a loss.

**And the value is two words, which the named parser cannot express.**
`crates/pistol-arena/src/exchange.rs:177-183`:

```
let value = |key: &str| -> Option<&str> {
    words.iter().position(|word| *word == key)
         .and_then(|at| words.get(at + 1)).copied()
};
```

One word after the key. `value("score")` returns the literal `cp`, `mate` or
`-mate` — never a number. The selection's branch B is *"`score` and `pv` come
out of the one parser both clients already share"*; the design repeats it and
never says the score read is a two-word read with a three-way tag. Implemented
as the helper stands, `.parse::<i32>()` fails on **every** line and every label
in the corpus is empty.

**Which brings the third half.** INVARIANT 7 (`:220-221`) makes `score` a
**non-fatal `Option`**. That is exactly right for the SPRT caller — and exactly
wrong for the label caller. A label record written with an absent score is a
record whose one reason for existing is missing, written silently: the
swallowed error CLAUDE.md rule 3 forbids. Nothing in §10 or §11 says a label
record may not carry an absent score, and none of the eleven tests would fail if
every score in the corpus were `None`.

**FIX.** In §4, name all three spellings and state the read as
`(tag, number)` with the tag deciding the sign. In §10, add an invariant: **an
absent or unparseable score is FATAL on the label path and non-fatal on the SPRT
path** — one `Option`, two callers, two obligations. In §11, add
`a_label_record_is_never_written_with_an_absent_score` and a mutant
(*the score's tag ignored and the first word parsed*) that kills it, and extend
`a_score_is_recorded_from_the_movers_point_of_view` to a fixture triple —
`cp`, `mate`, `-mate` — since a `cp`-only fixture leaves both mate spellings
untested.

---

## MAJOR

### MAJOR 1 — `label_sha256` does not identify a labelling run: it omits what changes the output and includes what cannot

`docs/experiments/wp20_design.md:106-121` (§5).

**(a) The `--workers` premise is imprecise.** §5:108 — *"The arena deliberately
excludes `--workers` from `experiment_sha256`"*. What `experiment_digest`
excludes is the **config key** `run.n_workers`
(`crates/pistol-arena/src/report.rs:29-40`, canonical string at `:45-74`). The
`--workers` **flag** exists only in replay mode
(`bin/arena.rs:90-93`), which computes no `experiment_sha256` at all. The
conclusion still lands; the premise as stated does not.

**(b) It omits what changes the output — and the arena's own precedent is one
line below the one the design cites.** `report.rs:48-50`:

> // The skip decides WHICH games are played, so two runs differing only in it
> // are different experiments (docs/decisions.md D-202).
> `let _ = writeln!(canonical, "openings_skip {}", config.run.openings_skip);`

By exactly that reasoning, the **seed** and the **sampling fraction** decide
WHICH POSITIONS are labelled, and both are outside `label_sha256`. The design
says so itself: INVARIANT 5 (`:216-217`) — *"when the label policy labels every
candidate, the seed changes no output byte"* — is a statement that when it
**samples**, the seed **does** change output bytes. Two runs over one report at
one budget with two seeds produce two different corpora carrying **the same**
`label_sha256`. A digest that collides across genuinely different runs is not an
identity. The design cites the arena's `--workers` boundary and not the
`openings_skip` line beside it, which is the one that cuts against it.

**(c) It includes what cannot change the labels.** `source_sha256` is taken over
the **whole file** — `bin/arena.rs:166`,
`let source_sha256 = pistol_cli::sha256::sha256_hex(&bytes);` — which includes
the timing block that `report.rs:187-189` itself introduces as
*"machine- and schedule-dependent; **excluded from every comparison**"*
(`wall_ms`, `timing_engine … time_ms`, `n_workers`). So `label_sha256` moves
between two pass-1 runs that produced byte-identical games on two differently
loaded machines. That is the same class the `--workers` boundary exists to
prevent, arriving from the third side.

**FIX.** Digest `pistol_arena::report::verdict_block(text)`
(`report.rs:231-241`) rather than the whole file — the crate already exports
precisely "the part of a report two worker counts must agree on, byte for byte"
— and add the **seed** and the **label policy's identity** (its registered rule,
by name or digest) to the canonical concatenation. Correct §5:108 to name
`run.n_workers` rather than `--workers`.

---

### MAJOR 2 — requirement 4's determinism re-run receipt is in no test, no invariant and no deferral — and §7's dedup introduces the exact hazard hard rule 4 names

Dispatch requirement 4 (`wp20_dispatches.md:80-82`):

> Is deterministic end-to-end given (seed, book range, config, SHA): **a re-run
> receipt proves byte-identical output on a small range.**

And the Development round's registered mutant (`:106-107`):

> **seed ignored -> determinism receipt dies**

Neither appears in the design. §11's eleven tests contain no re-run comparison,
and §11's mutant table (`:237-246`) substitutes the **inverse** mutation — *"the
seed read on a label-everything policy"* against the seed-**inertness** test.
That is a different property: inertness says a seed changes nothing when nothing
is sampled; the receipt says two runs of the same configuration produce the same
bytes. Neither implies the other. §12 does not defer it either, so it is dropped
rather than delegated.

**This is not a formality.** §7's DEDUP (`:157-162`) introduces a **shared board
key** and *"the count of distinct boards"* as a `derived` header field. That is a
keyed structure over positions, and CLAUDE.md hard rule 4 names its hazard by
name: *"no unseeded hash-iteration order on choice paths (fixed-seed hasher or
sorted iteration)"*. Nothing in the design says the board key's structure is
ordered or fixed-seed, and nothing would notice if it were not — the corpus's
record order (and therefore its `body_sha256`) could differ between two
identical runs. The determinism receipt is the one instrument that would catch
it, and it is the one instrument the design omits.

**FIX.** Add to §11:
`two_labelling_runs_over_one_report_write_the_same_bytes`, and add the
dispatch's own mutant row (*the seed ignored → the re-run receipt dies*). Add to
§7 that the board-key structure is sorted or fixed-seed by hard rule 4. Add to
§10 an invariant that the record ORDER is the walk order (game index, then turn
index) and is not a function of any map's iteration.

---

### MAJOR 3 — INVARIANT 1 is false for a capped game, and capped games are never named

`docs/experiments/wp20_design.md:209-210` (INVARIANT 1):

> **A game's outcome comes from replaying its move list through `pistol-core`
> and from nothing else.** Rule 2 forbids a second win detector.

`crates/pistol-core/src/turn.rs:34-45` — `Outcome` has **two** variants,
`Ongoing` and `Win`. There is no third.

So the referee returns `Ongoing` for a **capped** game *and* for a **forfeited**
game, and cannot tell them apart. The arena's own result type has three values —
`crates/pistol-arena/src/record.rs:5-12`, `P1Win` / `P2Win` / **`Capped`**, with
`Capped` documented as *"The turn cap ended it. An evaluation horizon, never a
game rule."* — and `is_decided` (`record.rs:160-162`) exists precisely because a
capped game has no winner.

The design uses "a forfeited game's outcome cannot be recovered that way"
(`:54-57`) as the ground for skipping forfeits, and then never mentions that the
identical deficiency applies to capped games — which it keeps, and for which §3's
`game outcome` column has no value the referee can supply. Capped games are not
an edge case: `REPORT_SCHEMA` 3 exists (`report.rs:19-21`) because
`first_player_wins` had to be re-defined over decided non-forfeit games.

The information IS recoverable — a **non-forfeited** game that replays to
`Ongoing` was capped, confirmed by the transcript's own `turn_cap`
(`transcript.rs:42`), and `transcript::replays` (`:359-378`) already refuses a
move list with turns recorded after a win, so there is no third case. But that
derivation uses two report facts the referee does not have, which is what
INVARIANT 1 says it does not do.

**FIX.** Name the third outcome value in §3's table. Restate INVARIANT 1 as:
*a WIN and its winner come from the referee and from nothing else; a
non-forfeited game the referee reports `Ongoing` is CAPPED, which the report's
own `turn_cap` confirms.* Add
`a_capped_game_records_the_capped_outcome_and_not_a_win` to §11.

---

### MAJOR 4 — the book exclusion is stated once, pinned by nothing, and its boundary is not fixed

§7:146-147 — *"Every to-move position of every non-forfeited, **non-book** turn is a
candidate."* That is the only occurrence of the exclusion in the document. It is
not in §10's invariant list, it has no test in §11, and it has no mutant. The
forfeit skip — the same kind of exclusion — gets INVARIANT 3, a test
(`a_forfeited_game_contributes_no_records_and_is_counted`) and a header count.
A mutation that labels book positions kills nothing.

The asymmetry matters because the exclusion is **large and load-bearing**: it
removes `opening_turns` positions from every game, and its real ground (the
paired book means the first turns repeat across the whole run, and the turn-1
position is identical in every game) is **never stated** — so a successor cannot
tell whether it is a deliberate policy or a copy of `replay::walk`'s skip.

The boundary is also unfixed. "Non-book turn" does not say whether the position
at the last book turn is in or out. The tree's own answer is
`crates/pistol-arena/src/replay.rs:137-138`:

```
for (at, recorded) in game.moves.iter().enumerate() {
    if at >= opening_turns as usize {
```

— and that answer was written for a **divergence** check, where the ground is
that a book turn was not an engine's choice. A label is a fresh evaluation and
does not inherit that ground. The design borrows the shape without the reason.

**FIX.** Promote the exclusion to §10 as an invariant with its boundary written
as `at >= opening_turns` and its actual ground (duplication across the paired
book) stated in one sentence. Add
`a_book_turn_contributes_no_record_and_the_count_is_in_the_header` to §11 and
the matching mutant row.

---

### MAJOR 5 — the design never says which seat answers a label ask, and never refuses a report whose two seats are different engines

§1:28-29 — *"at each to-move position send `newgame`, then the position, then a
`go`"*. To **which** channel? The document never says.

`replay::walk` answers it for its own purpose —
`crates/pistol-arena/src/replay.rs:139-140`,
`let engine = seat_of(mover_is_p1, game.a_is_p1);` — the seat whose turn it is.
Under §1's self-play PASS 1 that choice is immaterial. But nothing makes PASS 1
self-play *from pass 2's side*: `validate_engines` refuses **only identical
labels** —

```
crates/pistol-arena/src/validate.rs:242-250
if self.engine_a.label == self.engine_b.label { … }
```

— which the design correctly quotes at `:23-24` as the reason self-play is
expressible, and which is equally the reason an **A-vs-B** report is
indistinguishable from a self-play one to `transcript::read`. Handing pass 2 an
existing SPRT report (`configs/arena_wp17_heuristics_vs_staged.toml` produced
one) is legal, produces a corpus whose labels alternate between two different
teachers, and draws no refusal. Rule 3.

**FIX.** State in §1 that pass 2 asks the seat to move
(`seat_of`, `replay.rs:139`), and add to §10: **pass 2 refuses by name a source
report whose two seats do not attest the same `binary_sha256`, `config_sha256`
and `weights_sha256`** — all three are already in `EngineIdentity`
(`transcript.rs:34`, `read_engines` at `:246-251`), so the check is free. Add
`a_report_whose_two_seats_are_different_engines_is_refused` to §11.

---

### MAJOR 6 — §8 defers exactly the components D-518 registered in advance, its timing guard is weaker than its model's, and the split rule is registered nowhere at all

§8 (`:169-197`) cites D-518 as its model: *"the decision rule registered before
the sweep, the sweep before the size"*.

**What D-518 actually fixed before the measurement**
(`docs/experiments/book_v2_registration.md:132-163`):

1. the form, `n_openings = ceil_to_500(P + 500)`;
2. the definition of `P`;
3. **the threshold, `power ≥ 0.90`** — and the document says why it is there:
   *"The threshold is registered here, **before the sweep**, precisely because
   moving it afterwards is the post-hoc threshold move CLAUDE.md forbids."*;
4. **the grid** — nine named pair caps, `500 … 8000`;
5. runs per point and seed;
6. **the off-the-end rule**: *"**If `P` exceeds 8000** … the size is **not**
   extrapolated. The grid is extended in one amendment, reviewed, and re-run."*

**What §8 fixes:** items 1 and 2 only. Items 3 and 4 — *"the grid, the floor and
the confidence level"* — are deferred (`:192-197`). Item 6 is absent entirely:
§8 says nothing about what happens when **no** grid point clears the floor,
which is the precise moment a later session is tempted to lower the floor.

So the design defers the two components its own cited model names as the
anti-post-hoc guard, and drops the third.

**The timing guard is also weaker than the model's.** §8:194-195 — *"before the
corpus reaches any candidate size"*. D-518's is *"before the sweep"*. The
difference is real: the corpus grows monotonically from zero through every
candidate size, and nothing in §8 forbids a session from computing a held-out
recall on the partial corpus, seeing its shape, and **then** writing the
registration. D-518 foreclosed that by registering before anything ran.

**And one component is neither fixed nor deferred.** §8 step 1 says the corpus
is *"split by POSITION into a FIT part and a HELD-OUT part, disjoint by
construction"*. It does not say the **proportion**, or the **mechanism** (random
under a seed? by game? by source report?). §12's deferral list (`:250-252`) names
*"the census minimum's grid, floor and confidence level"* and **not** the split.
A split proportion chosen after the corpus exists is a fitting knob nobody has
registered — which is the defect §8 exists to foreclose, inside §8.

**FIX.** Add the split's **mechanism** to §8's fixed form (it is a mechanism, not
a number — D-483 does not touch it) and its **proportion** to §12's deferral
list. Add D-518's off-the-end clause: *if no grid point clears the floor, the
grid is extended in one reviewed amendment and re-run, never extrapolated, and
the floor does not move.* Strengthen the timing guard to D-518's own wording —
**before any recall is computed on any part of the corpus**, not before a
candidate size.

---

### MAJOR 7 — the label budget's KIND is unconstrained, which would make the corpus nondeterministic and requirement 4 false

§5:119-121 — *"The label budget still arrives on the command line — there is no
config document in this mode and rule 1 forbids a code-side default"*. It says
**where** the budget arrives and never **what kinds** are admissible.

Every other budget path in this crate refuses `movetime` by name, and the crate
treats that refusal as its reason for existing:

- `crates/pistol-arena/src/validate.rs:39-45` — *"The one refusal this crate
  exists to make loudly"*, `ArenaError::MovetimeBudgetRefused`;
- `crates/pistol-arena/src/transcript.rs:164-170` — a source report on a
  non-`nodes` budget is refused, *"the whole premise is that a re-driven engine
  answers what it answered, which wall-clock does not promise (CLAUDE.md rule 4)"*;
- `crates/pistol-arena/src/config.rs:119-133` — `BudgetSection::go_line()`
  returns `None` for `MovetimeMs` and `Some` for the two instrument budgets;
- `bin/arena.rs:49-51` — the usage block says only instrument budgets are
  accepted.

A movetime label budget would put wall-clock inside the label, breaking hard rule
4, D-95 and requirement 4's byte-identical re-run in one stroke. And it is the
new mode's **only** budget input, so nothing else refuses it.

This is also where the round-2 red team's MINOR 2 lands. The `go` line has two
producers today —

```
$ LC_ALL=C git grep -n "protocol::GO" -- crates | LC_ALL=C sort
crates/pistol-arena/src/config.rs:124:  …
crates/pistol-arena/src/config.rs:129:  …
crates/pistol-arena/src/transcript.rs:200: go_line: format!("{} nodes {budget_nodes}", …GO),
crates/pistol-arena/src/bin/stub_engine.rs:291: …
```

— and pass 2 needs a third. Building it from `BudgetSection::go_line()` gives the
movetime refusal for free and keeps the count at two.

**FIX.** One sentence in §5: **the label budget is one of the two instrument
budgets and a `movetime` label budget is refused by name**, its `go` line built
by `BudgetSection::go_line` (`config.rs:119`) rather than formatted a third
time. Add `a_movetime_label_budget_is_refused_by_name` to §11.

---

### MAJOR 8 — the label policy's CHOICE is deferred, and the dispatch names it as the design's to make

`wp20_dispatches.md:89-91` — the design decides and records:

> the label policy (which positions get deep labels — **all, or a registered
> sampling rule**)

§7:146-148 defers the choice itself: *"Whether all candidates are labelled or a
fraction is sampled is a **registered rule in the pilot's pre-registration**, not
a default here."*

D-483 puts the *fraction* in the prereg — correctly. It does not put the
**choice between two policies** there: that is a mechanism, and mechanisms are
what a design is for. The cost of not choosing is visible in the document: it
forces INVARIANT 5 to exist (a pinned statement about a policy that may never be
used), it leaves §5's digest with an input it cannot name (MAJOR 1), and it
leaves two code paths alive where one would do.

**FIX.** Decide it. Either *"the policy is: label every candidate; a sampling
rule may be registered later if the pilot's measured throughput demands one, and
that registration names its fraction"* — which keeps INVARIANT 5 as a genuine
guard on a live path — or *"the policy samples, and the prereg fixes the
fraction"*. One sentence, either way.

---

## MINOR

**MINOR 1 — §2 asserts a gap next to a skip it never says is possible.**
`:47-49` says `transcript::read` *"does not surface `result`, `forfeit_by`"* —
accurate — and `:56-57` then requires pass 2 to SKIP forfeited games. It never
names the field that makes the skip implementable: `RecordedGame.forfeit`
(`transcript.rs:19-20`), set at `transcript.rs:307` from
`value(&fields, "end", record)? == "forfeit"`. A reader who takes §2 at its word
concludes INVARIANT 3 cannot be implemented. **Fix:** cite the field.

**MINOR 2 — the forfeit skip's cost is neither shaped nor traded.**
The skip discards a forfeited game's **pre-forfeit** positions, whose *score*
labels are untouched by the forfeit; only the *outcome* target is unavailable.
The cost's shape is `forfeit_rate × positions_per_game`, visible in the header
per INVARIANT 3, and zero in the pilot (the dispatch requires zero forfeits,
`wp20_dispatches.md:113`) but not in production. The alternative — an outcome
value meaning *"no rules outcome"* — is not considered, and MAJOR 3 shows the
schema will need such a value anyway. **Fix:** one sentence stating the shape of
the loss and why the skip is preferred to a fourth outcome value.

**MINOR 3 — `pv` is widened into `totals_of` with no consumer.**
§3's record table (`:74-83`) has eight columns and none of them is a principal
variation; the best move comes from *"the `bestmove` line's turn"* (`:79`). Yet
INVARIANT 7 (`:220-221`) widens the parser for `score` **and `pv`**. Beyond being
unused, `value()` returns one word (`exchange.rs:177-183`), so the `pv` it yields
is the first turn and not the line — a field that cannot answer what its name
promises. **Fix:** drop `pv` from the widening, or name its consumer. BLOCKING 1
option 2 suggests a better use for the slot: `solver_nodes`.

**MINOR 4 — the loader has no home.** Three of §11's eleven tests are loader
tests and requirement 2 names a loader, but §3 names only the SINK
(`pistol_cli::corpus::emit::Fixture`). The two candidate homes have different
consequences and the design picks neither: `pistol-cli` (beside the sink, where
`corpus/mod.rs`, `corpus/record.rs` and `fixture_loader` already do this work)
breaks the design's own *"Only `pistol-arena` changes"* (`:12`); `pistol-arena`
honours it but puts the training corpus's reader inside the SPRT judge.
**Fix:** name the home and its consequence in one sentence.

**MINOR 5 — `widening_totals_of_leaves_the_sprt_report_byte_identical` is not
the test that proves the property.** Two reasons. (i) No two arena reports are
byte-identical: `report::timing` writes `wall_ms` and `timing_engine … time_ms`
(`report.rs:185-212`). The tree's convention for this exact claim is
`report::verdict_block` (`report.rs:230-241`), and the existing test says so —
`two_worker_run_report_identical_to_single_worker`,
`crates/pistol-arena/tests/run_tests.rs:129-145`, asserts
`verdict_block(one) == verdict_block(two)`, never the report. (ii) A same-tree
test cannot compare against pre-change code; byte-identity across a code change
is a two-revision procedure, which is exactly what the sibling dispatch
prescribes for its own gate (`wp20_dispatches.md:196-197`, *"the two-binary diff
procedure … output digest equal to the pre-change engine's"*).
**The test that would prove it in-tree** is on `totals_of` itself:
`a_totals_line_missing_the_new_fields_still_bills_compute` — feed a totals line
with the three old fields and no `score`/`pv`, assert `Some((nodes, time,
depth))`, so the mutant *"a new lookup made load-bearing with `?`"* fails it.
That is a real test, and it is the one the design's own mutant row implies.
**Fix:** rename to `verdict_block`-scoped, and add the `totals_of` unit test as
the mutant's actual killer.

**MINOR 6 — pass 2's own failure modes are undesigned.** `exchange::ask`
(`exchange.rs:23-89`) classifies four outcomes as forfeits — `EngineExited`,
`ProtocolError`, `BadBestmove`, and an overlong line — plus `ArenaError::Killed`
for a killed child (`:145-148`). "Forfeit" has no meaning in pass 2, which is not
playing a game. The design does not say whether a failed label `go` aborts the
pass (rule 3) or skips the record. It also never states the process **lifetime**
across pass 2 (memo decision 8), which §9's cost shape and D-540's fresh-process
criterion both stand on. **Fix:** one paragraph — a failed label ask aborts by
name, and the process is spawned per *(what)*.

**MINOR 7 — the new mode's CLI grammar is not named.** `bin/arena.rs:82-100`
matches exact argument slices; the design adds a third arm and gives it no
spelling. Rule 1 requires the label budget, the seed and the sampling rule each
to arrive explicitly with no code-side default, so the grammar is a mechanism
and not a detail. **Fix:** write the usage line.

**MINOR 8 — §8's counting unit does not exist yet, and §8 does not say so.**
*"Win-proving firings on DISJOINT POSITIONS"* is countable only once a census row
carries position identity, which is WP-2.0b's scope item 1 and D-539's second
obligation (*"landing only the first would ship a flag that cannot answer the
question it exists to ask"*). §8 reads as though the rule were executable on
landing. **Fix:** one clause naming WP-2.0b as the enabling package.

**MINOR 9 — memo decision 11 (per PLY or per TURN) is answered only by
implication.** "turn index" (`:83`), "non-book turn" (`:146`), "advances along
the recorded list" (`:219`). The answer is per-TURN and is forced by the seam
(one ask per turn), but D-477 is the ADR that exists because an unstated unit
axis cost two DECISION-RED-TEAM rounds, and D-477's own rule is that the unit is
quoted where it is CONSUMED. **Fix:** one sentence, with `replay.rs:137` as the
consumption site.

**MINOR 10 — the dedup rule does not cover two records with the SAME move
list.** §7:157 — *"Records are keyed by the canonical move list (D-6)"* — and
`:158-160` rules only on **transpositions** (different lists, same board). Two
records with an identical list are unaddressed, and "keyed by" reads as dedupe
while §3's `game index and turn index` column reads as retention: if two collapse,
which game does the record point back into? The book exclusion (MAJOR 4) removes
the practical exposure — the shared prefix of a pair — which is another reason
that exclusion needs to be pinned. **Fix:** one sentence.

**MINOR 11 — the ledger's same-commit rule sits against the preamble.**
§7:164-166 asserts *"its own rule that a row lands in the same commit as the
config that consumes it"* — `docs/book_v2_ledger.md:16-19` — while `:12` says
*"No committed config moves"*. The pilot must commit a **new** arena experiment
config alongside its ledger row for the ledger's rule to be satisfiable.
**Fix:** say that `:12` means no EXISTING committed config is edited, and that
the pilot adds one new arena experiment config and its ledger row in one commit.

---

## The strongest attack that did not land

**INVARIANT 2, coldness — and it is the invariant the whole shape rests on.**

If a `Searcher` holds anything `clear` misses, the two-pass shape buys nothing,
D-540's first clause is unmet, and the design is dead rather than amendable. The
selection asserts it, the round-2 red team asserts it, and §6 asserts it — three
documents agreeing is one document repeated, so I traced it myself, from the
verb down, and could not get a byte across a `newgame`.

**The protocol arm.** `crates/pistol-cli/src/protocol.rs:99-102` —
`NEW_GAME => { no_arguments(…)?; self.engine.new_game(); … }`.

**The engine.** `crates/pistol-engine/src/instance.rs:17-20` — `Pistol` has
exactly three fields, `config`, `searcher`, `state`. `new_game` at `:73-76`
resets both mutable ones: `self.state = GameState::new_game(); self.searcher.clear();`
`config` is immutable after `from_config`.

**The searcher.** `crates/pistol-search/src/search.rs:57-81` — six fields, and I
enumerated them mechanically rather than trusting the count:
`params`, `table`, `solver`, `position`, `heuristics`, `census`.
`clear` (`:230-239`) reaches three — `table.clear()`, `heuristics.clear()`,
`solver.reset()`.

The other three, each chased to its floor:

- **`table`** — `crates/pistol-search/src/tt/mod.rs:108-112`,
  `self.buckets.fill([EMPTY; BUCKET_ENTRIES]); self.generation = 0; self.used = 0;`
  A true wipe, not the `new_generation` epoch bump three lines below it (`:116-118`).
  The design's claim is exact.
- **`solver`** — `crates/pistol-solver/src/solver.rs:200-203`,
  `self.table = SolverTT::new(self.tt_entries); self.epoch = 0;`
  The table is **rebuilt**, so the epoch-isolation reasoning at `:169-176` never
  has to be trusted. The design's *"REBUILT rather than merely epoch-bumped"* is
  the right description.
- **`position`** — `crates/pistol-search/src/search.rs:253`,
  `self.position.reset_to(state);` at the top of **every** `search`, before
  `new_generation` and before `heuristics.begin_search()`. And `reset_to` itself
  (`position.rs:55-71`) is where the **evaluation** lives — the field I expected
  to find uncovered, since the design's six-field enumeration never mentions the
  eval. It is inside `Position` (`position.rs:13`), and `reset_to` unwinds it
  stone by stone over the OLD board before applying the new one, exactly as
  D-61/D-62 promise; `threats` (`position.rs:17`) is not unwound but **replaced**
  with `ThreatState::new()` (`:60-62`), which is stronger.
- **`params`** — `SearchParams` is `Copy` and taken by value at `:195-197`;
  nothing writes it after `new`.
- **`census`** — `None` unless `collect_trigger_census` is called, and its own
  doc (`:200-205`) says *"no committed config can ask — the only callers are this
  crate's own tests and the `trigger_census` example"*.

**The field the prompt asked about that is not a field.** The PV table is not
held by `Searcher` at all: `PvTable` is constructed per search inside `Run`
(`crates/pistol-search/src/pvs.rs:110`, `:144`), so it cannot carry across a
position even without a `newgame`.

**So INVARIANT 2 is true, end to end, and I could not manufacture a counterexample.**
The two-pass shape does exactly what §1 claims: it makes coldness a property of
the shape rather than a thing a criterion has to keep re-establishing, which is
D-540's own wording and the thing D-527 was paid for.

**A second attack that did not land: INVARIANT 7's SPRT neutrality.** I expected
branch B to be where this design broke, because row (e)'s `?`-chain hazard is a
real one. It is not. The return value of `totals_of` has exactly one consumer —
`crates/pistol-arena/src/exchange.rs:76-79`:

```
if let Some(totals) = totals_of(&line) {
    compute.add(totals.0, totals.1, totals.2);
    continue;
}
```

— and `Compute::add` (`record.rs:84-89`) is the only thing that touches the
report's node, depth and search counts. Adding fields that the existing three
lookups do not gate cannot reach `compute.add`, cannot reach
`conclusion::games`'s `nodes_a`/`nodes_b`, and cannot reach the verdict block.
**Branch B is output-neutral by construction, and the construction the design
names is the right one.** What it lacks is a proof (MINOR 5) and an account of
what the *other* caller owes (BLOCKING 2) — not soundness.

**A third: the forfeit skip.** I expected to find it throwing away a large
fraction of the corpus. It does throw away pre-forfeit positions (MINOR 2), but
the skip is counted in the header by INVARIANT 3, the pilot requires zero
forfeits, and `RecordedGame.forfeit` makes the skip mechanically exact. It is a
stated cost, not a hidden one.
