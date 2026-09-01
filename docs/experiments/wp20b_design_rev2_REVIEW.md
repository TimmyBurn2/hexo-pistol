# REVIEW-design — `docs/experiments/wp20b_design.md` revision 2

## Header

**Revision reviewed:** `9c4f702edc25df8c0f5d021bacf188ed10f40374` — a `git stash
create` object (confirmed `git cat-file -t` → `commit`), holding the uncommitted
work on top of HEAD `a56449b`. It does NOT match HEAD.

**Does it still match the working tree?** Yes, at the moment the review finished:
a second `git stash create` returned `585e734`, whose tree is
`37821e2fccd798a0810f55c32062a36235e28276` — byte-identical to `9c4f702`'s tree.
`git status --porcelain` showed exactly the four paths of the diff under review.
The only difference introduced afterwards is **this report file**, which is the
one file this review was permitted to write. No source, config, doc or ADR file
was modified.

**Reviewer:** fresh context, did not author the document.

**What I read.** `CLAUDE.md`; `docs/process.md`; `docs/experiments/wp20b_design.md`
in full; the uncommitted tail of `docs/decisions.md` (D-562, D-563) and the ADR
lines D-6, D-8, D-88, D-291, D-423, D-424, D-441 (via config headers), D-512,
D-527, D-530, D-534, D-535, D-537, D-539, D-542, D-544, D-547, D-551, D-553,
D-554, D-557 through D-561; the whole of `docs/experiments/wp20_dispatches.md`'s
diff, plus the v1 and v2 dispatch texts word by word; `wp20_premise_memo.md`
§P2.1, §P2.4, §P2.5; `wp20s_design.md` §2.1 and §8 in full;
`wp21_DISPATCH.md`. In source: `census.rs` (whole), `pvs.rs` 60–145 and 240–260
and 560–790, `search.rs` 60–100/180–235/240–340/370–390/515–530/700–780,
`info.rs` (whole), `lib.rs` of pistol-search and pistol-engine,
`engine.rs`, `instance.rs` 120–200, `report.rs` 1–45, `protocol.rs` 160–185,
`budget_token.rs` 30–70, `workspace_shape_tests.rs` (whole),
`pistol-core/src/{lib.rs,symmetry.rs,zobrist.rs,state.rs,board.rs}` at the cited
places, `pistol-arena/src/{labels.rs,exchange.rs}`, and every file under
`configs/`. **D-401 was not read.**

**What I could not check.** (1) **Revision 1 is not recoverable.** `git log --all
-- docs/experiments/wp20b_design.md` returns only the index-stash commit
`469153b`; the file is wholly new in `git diff HEAD`. So I could NOT diff rev 1
against rev 2 and cannot confirm the revision header's negative claims ("no
finding, no matrix row, and no recommendation" changed; "no field added, removed
or reordered" in §4). I checked instead that the header's five positive claims
match what §4, §8, §9 and §10 now contain — they do, with the exceptions in N1
and N5. (2) I did not build or run anything: no `cargo`, no worktree, no
`CARGO_TARGET_DIR`. Every claim below is from reading the tree at `9c4f702`.

---

## VERDICT: **FALLS**

One BLOCKING finding (B1), six MAJOR, thirteen MINOR.

**The code half of this design is sound.** F1 is right, F2 is right and survives
both readings of the dispatch's second option, the C2 mechanism is fully
supported by pistol-core as it stands (`Key128::ZERO`, `BitXorAssign`, a
`Display` that emits exactly 32 hex digits, `canonical_form` over
`&[(Coord, Player)]`), and §5's coldness proof holds — I re-derived it from the
closure's own captured bindings rather than trusting it. The design FALLS on
what it *asserts* rather than on what it *builds*: its headline finding F3 is
stated as a universal that three committed configs falsify, and that universal is
being landed into `docs/decisions.md` as D-563, which is hard rule 10.

---

## BLOCKING

### B1 — "the gate is `false` in every committed config" is FALSE, and D-563 lands the falsehood as an ADR line

**The claim.** §1 F3's heading: *"F3 — THE CENSUS CANNOT FIRE AT ALL UNDER ANY
COMMITTED CONFIG"*, and its body: *"**The gate is `false` in every committed
config**"*. D-563 promotes it: *"THE CENSUS CANNOT FIRE UNDER ANY COMMITTED
CONFIG"*, *"verified at four sites and not inferred"*.

**The sites that falsify it.** Three committed configs set the gate `true`:

```
configs/gate_staged_solver_v0.toml:47   on_search_path = true
configs/bench_wp18c_solver_on.toml:45   on_search_path = true
configs/play_staged_solver_v0.toml:75   on_search_path = true
```

`gate_staged_solver_v0.toml` is not incidental — its own header says it is *"the
committed staged seat with the gate flipped, so the CI gate exercises the ON
path's choice plane"*. **This repository has already litigated this exact
overstatement and written the correction into a committed file**:

```
configs/play_staged_solver_v0.toml:8-12
# THIS IS NOT A DEPLOYMENT CONFIG. D-441's "gate OFF in every committed config
# until an SPRT says otherwise" binds what pistol SHIPS; the measurement seats
# configs/gate_staged_solver_v0.toml and configs/bench_wp18c_solver_on.toml are
# committed with the gate on for the same reason this one is, and none of the
# three moves a deployment config.
```

The design inherited the universal from `wp20_premise_memo.md` §P2.5 (*"**The
gate is off in every committed config**, so a census run needs a seat of its
own"*, cited to `configs/instrument_staged_snk_v0.toml:74-78` and that file's own
stale comment) and verified it at exactly one further site,
`configs/instrument_v0.toml:113`. One config is not "every".

**Why it matters, and it is not bookkeeping.**

1. **Hard rule 10.** D-563 is an append-only decision line whose headline
   assertion is false about the tree it cites. Later work will cite D-563 the way
   this design cites D-539 — and D-539's own inherited premise is the second
   thing this design had to correct (M6). A decisions.md line that overstates is
   the breach, not the nit; the prompt's own bar, and CLAUDE.md's.
2. **It changes what the operator may conclude** — D-424's test, which is this
   project's own. §10.1 hands the operator one question: does the production
   labelling seat arm the solver? A reader of F3/D-563 concludes that arming it
   would be unprecedented among committed configs and would have to be invented.
   The truth is the opposite: **the pattern already exists three times over**, is
   documented in `play_staged_solver_v0.toml`'s header as the way to commit a
   non-deployment measurement seat with the gate on, and one instance
   (`bench_wp18c_solver_on.toml`, cap 2048) is the very seat the D-527/D-530 arc
   measured firings on. The ruling F3 asks for is cheaper and better-precedented
   than the finding presents it as.
3. It also touches §9's own determinism obligation ("all seats"): one of the
   standing CI seats **is** a gate-on seat, which is precisely where a census
   would fire if ever requested.

**What survives.** The operative claim survives intact and I verified every limb
of it: `configs/arena_wp20_label_pilot.toml:65-76` names
`config = "configs/instrument_v0.toml"` for **both** seats, and
`configs/instrument_v0.toml:113` is `on_search_path = false`. So *the labelling
seat records nothing*, D-560's cost model is an OFF-seat model, and WP-2.1 cannot
register a census unaltered. That is the finding worth having. The universal is
not.

**Smallest fix.** Replace the universal in both places with the quantifier the
evidence supports: *"the gate is `false` in the labelling seat's config
(`configs/instrument_v0.toml:113`, the config both pilot seats ran) and in every
committed config except the three non-deployment measurement seats
(`gate_staged_solver_v0.toml:47`, `bench_wp18c_solver_on.toml:45`,
`play_staged_solver_v0.toml:75`), whose own headers state why they are not
deployment configs."* Then add one sentence to F3's limb list and to D-563:
**arming a labelling seat does not require touching a deployment config — the
committed measurement-seat pattern already exists and is the cheapest shape for
the ruling §10.1 asks for.**

---

## MAJOR

### M1 — §9's bench guard cannot MEASURE what §9 says it will, on the seat F3 says the sweep uses

**The claim.** §9: *"**Bench guard:** one registered nps spot-check ON-token vs
OFF at 50 000 nodes, direction per convention — **and this is where §2's
ESTIMATED per-firing cost becomes MEASURED**."*

**What falsifies it.** §2's per-firing cost is paid inside the closure at
`crates/pistol-search/src/pvs.rs:623`, which is reached only after
`pvs.rs:602` (`let cap = self.solver.as_ref()?...`) does not return. By the
design's own F3, on any seat with `on_search_path = false` the closure is never
entered — **zero firings**. An ON-token/OFF-token pair on such a seat therefore
compares two runs that both pay the per-firing cost zero times: the comparison is
invariant under a per-firing cost of any size. That is `docs/process.md`'s named
vacuity — *"a criterion that is a property the named defect class PRESERVES …
passes vacuously and is not a criterion"* — arriving in the one obligation hard
rule 5 owns. The seat is never named in §9, and every other seat the document
names (`instrument_v0.toml`, the standing position set, the pilot config) is
gate-off.

**Why it matters.** The document contradicts itself across two sections: F3 says
nothing fires; §9 says the firing cost gets measured. Whichever is implemented,
one of them is wrong, and the ESTIMATED 10² firings/ask that drives §2's whole
artifact-size row would go into the sweep still estimated while the closure
believes it has been measured.

**Smallest fix.** Split the guard and name both seats: (i) the non-census-path
tax — ON-token vs OFF on the standing OFF seat, which is the guard the dispatch's
*"a logging path that taxes the engine"* sentence actually asks for; (ii) the
per-firing cost — the same pair on a committed **solver-ON** seat
(`configs/bench_wp18c_solver_on.toml`, cap 2048, the seat this arc has measured
firings on before). Say which claim each supports, and delete "becomes MEASURED"
from the limb that cannot deliver it.

### M2 — option B's cost row is fabricated, and the quotation offered for it says the opposite of what it is used for

**The claim.** §2's matrix, option B: cost at a firing *"needs a played-turn stack
the search deliberately does not keep — **'walking its history on every take-back
… is a linear cost per ply in a loop that runs millions of times'**
(`crates/pistol-search/src/position.rs:23-24`)"*; cost on the non-census path
*"**a stack on the hot path** — breaks dispatch obligation 4 unless conditionally
maintained, which is a branch in the make/unmake loop"*.

**The sites that falsify it.** The quoted comment is the doc of a field the search
**does** keep, and it explains why that field exists:

```
crates/pistol-search/src/position.rs:18-25
/// The stones this search put down, newest last.
/// Not a second copy of the game: it is what the *evaluation* has to be told
/// to take back … Reading it off the game instead would mean walking its
/// history on every take-back, which is a linear cost per ply in a loop that
/// runs millions of times.
placed: Vec<(Coord, Player)>,
```

And the game itself already publishes the move list, maintained through the
search's own make/unmake (`Position::place` → `state.place`, `Position` line 132
→ `state.undo`, which pushes and pops `history`):

```
crates/pistol-core/src/state.rs:148-151
/// Every stone in the order it was played. This move list is the canonical
/// encoding of a position (docs/decisions.md D-6).
pub fn played(&self) -> impl Iterator<Item = (Coord, Player)> + '_ {
```

So option B needs **no new stack, no branch in the make/unmake loop, and no cost
on the non-census path**: like C1 and C2 it is a read inside the census closure
(`state.played()`, regroup plies into turns, `canonical_sequence`). Its true cost
is of the same order as C1's.

**Why it matters.** CLAUDE.md's Process requires an option matrix whose options
are genuinely distinct and whose costs are attacked, not answered away; a red
team attacking this matrix in parallel will be attacking a row that is not true.
B is still eliminated — by F2, on §8 compliance — so the recommendation does not
move; the matrix's integrity does. This is also the D-544/D-554 class the brief
names: a quotation used to support the negation of what it says.

**Smallest fix.** Replace B's two cost cells with the true ones (a read of
`GameState::played()` inside the closure, regrouping and `canonical_sequence`,
comparable to C1; non-census-path cost **none**) and delete the
"breaks obligation 4" claim. B's elimination then rests where it belongs: §8.

### M3 — the arena limb of §6's diff has no test, no mutant, and no obligation

**The claim.** §6: *"`crates/pistol-arena/src/…` | the `--capture` flag that emits
the token, and the census rows written beside the capture"*. §8 lists twelve
tests and names three call-removed mutants under D-553.

**What falsifies the coverage claim.** Every one of the twelve tests is a
pistol-core, pistol-search or line-protocol test. **Nothing drives the arena.**
Remove the call that appends the token to the capture pass's `go` line, or the
call that writes the rows beside the capture, and all twelve stay green — the
protocol still refuses a bad third word (6), still emits a key on a row (1),
still writes nothing without the token (5). D-553's law is not "guards in
pistol-search get a call-removed mutant"; it is *"for every guard or invariant,
the mutation set includes a call-REMOVED mutant, and it must die at a test that
drives the call site with reachable input"*. The arena limb is the one place in
this diff where a call can vanish with nothing dying, and it is the limb the
production sweep actually runs.

**Why it matters.** F3 already establishes that a sweep can silently record
nothing. A second silent-nothing path — the flag that never reaches the engine —
would be indistinguishable from F3's, and the sweep's receipt (zero rows) is the
same in both cases.

**Smallest fix.** One arena test driving `--capture` with and against the flag
over a fixture report, asserting that the token appears on the emitted `go` line
and that rows land beside the capture; list its call-removed mutant in §8's
second table.

### M4 — the `Engine` seam change is one table cell, and it is the project's named contract

**The claim.** §6: *"`crates/pistol-engine/src/engine.rs`, `instance.rs` | the
per-`go` census request reaches the searcher; **no config field** (§3)"*. §6 also
says `SearchInfo` *"carries the run's census rows when one was collected"*.

**What is unstated.** `crates/pistol-engine/src/engine.rs:14-67` defines the whole
trait: `mode`, `new_game`, `set_position`, `go` (a default forwarding to
`go_reporting`) and `go_reporting`. A per-`go` census request must land somewhere
in that surface, and the design does not say where: a third method? a parameter?
a mode flag set before `go`? Does the defaulted `go` carry it? CLAUDE.md rule 11
makes this trait the contract the API layer will adapt, D-5 makes the line
protocol its spelling, and CLAUDE.md's Map says pistol-cli's I/O *"mirrors
`Engine` 1:1"* — so a protocol verb that has no trait counterpart is a contract
divergence, and a trait change is exactly the non-obvious choice hard rule 10
wants an ADR line for. The ownership of the rows is likewise unstated: today
`crates/pistol-search/src/search.rs:525` hands them back to the `Searcher`
(`self.census = run.census.take();`) and `take_trigger_census`
(`search.rs:216-221`) **panics** if collection was never started. If `SearchInfo`
also carries them, the design must say which owns them after a `go`, and what a
second `go` sees.

**Why it matters.** An implementer has to invent the seam, and whatever they
invent lands in the one place this project has forbidden itself to drift.

**Smallest fix.** One paragraph in §6 (or a §3.1) stating the trait's new shape,
what `go` does with it, that `new_game` clears the rows (§9 asserts this but §6
does not implement it), and which of `Searcher`/`SearchInfo` owns the rows after
a search — plus the ADR line the closure already owes.

### M5 — the dispatches file's six-difference enumeration is wrong twice, and one of the two is the evidence §10.0 turns on

The enumeration exists, in its own words, *"because the whole reason this file
exists is that a successor must be able to tell two dispatch texts apart"*. It is
part of this diff. Two of its six items do not survive a word-by-word comparison
of the two transcribed texts in the same file:

**(a) Difference 1 claims v2 *"adds 'the D-527 cold-seat discipline' as a named
read rather than a bare obligation."*** It does not. v1's own read-first list
(`wp20_dispatches.md:186-190`) already reads: *"census.rs and the D-527 cold-seat
discipline, the workspace-shape test pinning pistol-cli's dependencies"* — the
identical phrase, naming D-527 by number. The real difference in that limb is
narrower: the range `D-535..D-538` becomes `decisions tail (D-521 on…)`, and
D-512 and D-537 are named individually.

**(b) Difference 6 claims the premise clause is *"unchanged"* and *"recorded as a
difference that is NOT there"*. It is changed.** v1 scope 1: *"(the full-turn
128-bit key per D-8, or the canonical move-list prefix)"*. v2 scope 1: *"(full-turn
128-bit key per D-8, or canonical move-list prefix **per D-6**)"*. **The architect
edited the option field itself and still offered both options.** That is the
strongest piece of evidence in the record against §10.0's reading (see the
STRONGEST SURVIVING ATTACK below), and the document that exists to surface
differences buried it.

**Smallest fix.** Correct limb 1; rewrite limb 6 to say what actually happened —
the option field was edited (a citation added) and both options were kept — and
let §10.0 answer that fact rather than the weaker "unchanged" version of it.

### M6 — F1 contradicts a standing ADR line and no ADR line records the correction

**The claim.** §1 F1: *"**The dependency claim is true and the reachability
conclusion is false.**"* The conclusion it declares false is D-539's, quoted
correctly by the design and verbatim in `docs/decisions.md:1146`: *"the census is
unreachable from the line protocol because `crates/pistol-cli` does not depend on
`pistol-search`"*, which D-539 explicitly says *"is not softened by the ruling and
stays on the record"*.

I checked F1 and **F1 is right**: `crates/pistol-engine/src/lib.rs:47-53`
re-exports pistol-search's reporting vocabulary; `crates/pistol-cli/src/report.rs:2`
reads `SearchInfo` through it; and the memo's own §P2.1 limb (c) quotes the
sentence that names the route — `crates/pistol-cli/src/lib.rs:31-33`: *"what this
crate says to an engine, it says through the trait, **and the engine re-exports
the reporting types it hands out**."* The manifest test needs no change; I read
both of its assertions (`workspace_shape_tests.rs:92-96` and `:105-115`) and
neither reads source, only `Cargo.toml`.

**Why it matters.** This session wrote two new ADR lines (D-562, D-563) and gave
one of its three findings a D-line. The finding that corrects a *standing* line is
the one that got none, so D-539's false sentence stays live and citable — the
exact drift hard rule 10 names, with an amendment mechanism the session was
demonstrably willing to use.

**Smallest fix.** One limb on D-563, or one new line: D-539's reachability
conclusion is corrected — the dependency fact is true, the unreachability
conclusion is false, the route is the existing pistol-engine re-export, and the
manifest does not move.

---

## MINOR

**N1 — test 11 is not v2's fourth mutant, and the revision header says it is.**
v2: *"transposition ruling inverted → fixture dies"*. The design's ruling is that
a transposition pair is ONE position; inverting it makes such a pair count as TWO,
which kills **test 4** (`a_known_transposition_pair_counts_as_one_disjoint_position`)
and **test 2**. Test 11 (`two_positions_that_are_not_one_position_carry_different_census_keys`)
pins the opposite direction — over-folding — and its gloss says so: *"a pair the
design rules DISTINCT counted as one"*. It is a good test; it is not the named
mutant, and the header's change-3 (*"§8 gains mutant 11 (the transposition ruling
inverted)"*) mis-describes what was added. No coverage is lost — v2's fourth
mutant was already covered by test 4 before revision 2. Fix: relabel, and say
that v2's fourth mutant dies at test 4.

**N2 — test 4's named mutant has no code in this diff to mutate.** Its mutant is
*"the counting rule's own arithmetic"*, but nothing in §6 counts disjoint
positions: the census emits rows and the count is done later by an analyst.
Either test 4 reduces to test 2 (key equality, whence the count follows
definitionally) or it names a program this package does not ship. Fix: say which,
in one clause — v2's scope 1 asks for a fixture *"pinning a known transposition
pair to the ruled count"*, and a fixture pinning key equality plus a stated
counting rule satisfies it honestly.

**N3 — §4's stated D-512 property is not the property test 12 should assert.**
*"No field on this line is a sum of two others"* is satisfiable by luck and
falsifiable by coincidence (nothing stops `cover_count` equalling
`mover_hot + opponent_hot`). The property the named defect could falsify is the
one the test's own name states: each direction is spelled as its own named field,
asserted on a row whose two sides differ. Fix: state that, and drop the sum
sentence.

**N4 — the `-` spelling's justification is contradicted by the parser D-551 is
about; the better ground is unnamed.** §4(2) says omission is *"a row a parser can
silently mis-split"*. The arena's reader is key-directed —
`crates/pistol-arena/src/exchange.rs:199-205`, *"The word after `key`, matched
whole"* — and a committed test in that same file,
`fields_of_reads_a_captured_line_that_has_no_time_field`, pins that a **missing**
field reads as `None` by design. So omission is not the hazard claimed. The
choice is nevertheless right, on a ground the design does not cite: the corpus
already spells an absent field this way —
`crates/pistol-arena/src/labels.rs:14  pub const EMPTY_FIELD: &str = "-";`, read
back at `crates/pistol-arena/src/labels_file.rs:302`. Fix: swap the reason for
the precedent.

**N5 — "D-54y's form" is an unresolved placeholder.** The revision header cites
*"D-54y's form"*. The dispatches file's own closing note resolves only D-53a,
D-53n and D-55y; D-54y is the paste-block placeholder at
`wp20_dispatches.md:387` and it landed as **D-547** (*"STANDING LAW, THE
PASSED-SECTION FREEZE"*, `docs/decisions.md:1162`). A successor grepping D-54y in
decisions.md finds nothing. Fix: cite D-547.

**N6 — the design quotes v1's wording while declaring v2 governing.** F1 quotes
scope 3 as *"Whatever dependency route gets census output through pistol-cli…"*
(v1's wording; v2 reads *"The dependency route through pistol-cli: …"*); §5 quotes
obligation 4 in v1's longer form (*"the design proves (quoted site) that no extra
hashing or probe is added"*; v2: *"quoted site proving no extra hashing on the
non-census path"*); §2 and F2 quote scope 1's parenthesis without v2's added
*"per D-6"*. The governing file's own header rule is **"A document quoting 'the
dispatch' must say which"**. None of the substitutions changes an obligation, but
the one in §2/F2 hides the evidence M5(b) names. Fix: quote v2, and where v1's
wording is quoted deliberately, label it.

**N7 — §4's placement sentence is ambiguous in a way the plumbing cannot honour
both ways.** *"emitted with the per-depth `info` lines the `go` handler already
streams (`protocol.rs:170-172`), before `info totals` and `bestmove`"* reads
either as interleaved per depth — which the plumbing cannot do, because the rows
live on the `Run` during the search and are moved back to the `Searcher` only at
`search.rs:525`, after every per-depth report has been emitted — or as a block
after the reports and before `info totals`, which is implementable and is what §6
implies. No test in §8 pins **line** order (test 8 pins field order). Fix: say
which, and add the line-order assertion to test 8's fixture.

**N8 — matrix cell mislabel and one unmarked number.** The row *"bytes on the
wire per row"* gives 32 hex (C2) and ~600 B (C1), which are the **key field's**
bytes, not the row's; the arithmetic paragraph then correctly uses ~100 B and
~600 B per row. Separately, *"~119 800 distinct positions"* is used unmarked in
that paragraph although D-560 marks every one of its figures ESTIMATED, and
option A's cost is marked *"MEASURED-free"* for a fact established by reading
code rather than by measuring. D-291 asks for the marking, not for the
measurement.

**N9 — D-291's second clause, on the estimate that drives the size row.** The
volume basis (*"ESTIMATED at order 10² firings per ask"* at `nodes 400000`) is
the input to the 2.5 GB vs 15 GB row, and a committed instrument for it already
exists: `crates/pistol-search/examples/trigger_census.rs`, whose usage block
takes `--fixture --nodes --cap [--gate on|off]` and prints firings per entry.
That is minutes, not seconds, so it is not automatically the D-291 finding — but
a matrix that leaves its one decisive volume number estimated while the
instrument sits committed in the tree should say why it was not run. (It does say
tranche one must measure it; that is after the decision, not before it.)

**N10 — §3's last sentence describes a build configuration this design never
creates.** *"A `go` line carrying the token when the engine was built without
census support is a **named refusal**"* — there is no feature gate, no `cfg`, and
no second build in §6. Either name the mechanism or delete the sentence; as
written it is prose that constrains nothing (D-424's own test).

**N11 — `wp21_DISPATCH.md` §4's "Decision owed" (same diff) omits F3's question.**
It lists three: run WP-2.0b, lift the census gate, re-size the sweep. D-563 says
*"WP-2.1 may not register a census … until the labelling seat's gate is ruled"* —
a fourth decision, owed by the same operator, absent from the document whose whole
job is to enumerate what is owed. (Out of the strict review scope; in the diff.)

**N12 — D-562's handoff to this package is neither answered nor listed.** D-562(2)
says *"which key rules a disagreement is WP-2.0b's transposition question and not
this line's"*, about corpus dedup by three-key agreement. §2 rules the identity
for the **census**; §10 lists four things this package does not decide and this is
not one of them. Either say §2's ruling extends to the dedup disagreement, or add
it to §10.

**N13 — §8's first call-removed row names its site loosely.** *"the `go` handler's
test of the third word, at `crates/pistol-cli/src/protocol.rs`'s `go` arm"* — the
third word is parsed in `budget_token.rs` (`parse_budget`, called at
`protocol.rs:169`); what lives in the `go` arm is the *conditional emission* on
the token. Both calls are removable and only one is named.

---

## THE STRONGEST SURVIVING ATTACK

**That §10.0 has the re-issue exactly backwards, and F1 and F2 were overruled
before the design was written.**

The design's whole load-bearing structure rests on one inference: *"a re-issue
that never mentions a finding has not ruled on it."* Put the case against it at
full strength.

The v2 dispatch was written **after** revision 1 existed — the dispatches file
says so in its own transcription note. The architect had revision 1's F1 and F2
in hand. v2 then changes six things: the reading list, scope 1's fixture
requirement, two new wire constraints in scope 2, a longer and call-site-bound
mutant list, a corrected ROADMAP successor, and a restated STOP protocol. This is
not a re-issue that was pasted without reading; it is a re-issue that was **edited
clause by clause**. And the strongest evidence of all is the one the design and
the dispatches file both missed (M5(b)): **v2 edited scope 1's option field
itself**, adding *"per D-6"* to the second option. An architect who reaches into
the parenthesis that F2 says is empty, adds a citation to it, and leaves both
options standing, has looked at the option field. Likewise scope 3 was rewritten
— *"Whatever dependency route gets census output through pistol-cli"* became
*"The dependency route through pistol-cli:"* — a compression of exactly the clause
F1 says dissolves, retaining the instruction F1 says needs no obeying. On that
reading, v2 **is** the ruling: the architect saw both findings, declined both, and
the design has spent §2 building a matrix over an option field it was told to
choose from, and §1 arguing away a test it was told to update.

**Why I could not make it stick.** Three reasons, in descending strength.

First, F2 is not a preference the architect can decline by silence; it is a
constraint from a **landed, reviewed** design. `wp20s_design.md` §8 does not merely
prefer `key_full` — it names WP-2.0b, quotes the very parenthesis v2 kept, and
says *"neither of those is `key_full`, so this is a constraint that package
inherits and not an observation this one makes."* If v2's silence were a ruling,
it would be a ruling that overrides a landed document by not mentioning it, which
is the definition of silent drift. A dispatch that meant to override §8 would have
to say so, because §8 said WP-2.0b's name out loud first.

Second, F2's conclusion survives **both** readings of the option v2 edited. Under
the design's reading, option 2 is `key_seq` = `canonical_sequence`, which folds
symmetries and not transpositions. Under the reading v2's own new citation
invites — D-6, *"the move list is the canonical position encoding"* — option 2 is
the bare play sequence, which folds **neither**. Either way it is not `key_full`,
and adding "per D-6" makes the option *further* from the constraint, not closer.
So even the strongest evidence that the field was looked at does not make either
option legal.

Third, F1 costs nothing to be wrong about. If the architect did mean scope 3 as
an instruction, obeying it is impossible in the honest direction: there is no new
dependency to add to the ledger, so the "new shape it pins" is the old shape. The
design's answer — the test stays green unchanged, and *that* is the evidence the
package did not reach past the seam — is the only truthful way to satisfy a scope
item written against a false premise.

**What survives the attack anyway, and should be written down.** §10.0's reading
is *defensible* but its ground is stated too weakly. "A text that never names a
finding has not ruled on it" is false in general and the design should not lean on
it. The ground that holds is narrower and stronger: **F2's constraint is not the
dispatch's to waive**, because it belongs to a landed document that named this
package; and **F1's instruction is satisfiable only vacuously**, because the shape
it asks to be re-pinned is the shape already pinned. Swap the reading for those
two sentences and §10.0 stops depending on an inference about the architect's
attention that M5(b) partly refutes.

---

## Quotation audit

Every site the design quotes or cites, checked at `9c4f702`. "OK" = the file says
what the design says it says, at the cited place.

| # | design's citation | verdict |
|---|---|---|
| 1 | `pistol-engine/src/lib.rs:47-53` — the re-export block and its comment | **OK**, verbatim, exact lines |
| 2 | `pistol-cli/src/report.rs:2`, `:39` | **OK**, exact lines |
| 3 | `workspace_shape_tests.rs::pistol_cli_manifest_names_only_core_and_engine` asserts `[pistol-core, pistol-engine, serde, serde_path_to_error, toml]` | **OK** — that is the second assertion (`:105-115`); the first (`:92-96`) pins the `pistol-*` subset. Both read `Cargo.toml`, never source, so the §6 re-exports leave both green |
| 4 | CLAUDE.md rule 11, *"the `Engine` trait + line protocol ARE the contract"* | **OK** |
| 5 | D-539, *"the census is unreachable from the line protocol because `crates/pistol-cli` does not depend on `pistol-search`"* | **OK** verbatim in `decisions.md:1146`. The parenthetical *"from `wp20_premise_memo.md` §P2.1"* is loose: §P2.1 states it in different words (heading + verdict), not in that sentence |
| 6 | `wp20s_design.md` §8, the `key_full` passage | **OK** — I read §8 in full; the ellipses elide *"The coarsest must rule, and BOTH folds have to be in it"*, the symmetry rationale, *"and it loses nothing on an ongoing position"* and the citation. Nothing elided refutes the quotation; §2.1 independently confirms the three keys and that `key_full` **is** `canonical_form` |
| 7 | mapping: dispatch option 1 = `key_pos` (`GameState::key`), option 2 = `key_seq` (`canonical_sequence`) | **OK** per `wp20s_design.md` §2.1. Note v2 adds *"per D-6"* to option 2, which admits a second reading under which it folds neither equivalence — F2's conclusion holds either way |
| 8 | `wp20s_design.md` §8, *"to stop a successor opening round 3 on thin evidence"* / FLOOR | **OK** verbatim |
| 9 | `pvs.rs:602` `let cap = self.solver.as_ref()?.1.per_call_node_cap;` | **OK**, exact line |
| 10 | `search.rs:287-288` `if let Some(wiring) = self.params.solver && self.solver.is_some()` | **OK**, exact lines |
| 11 | `instance.rs:150-152` `solver_wiring` returning `None` when `!on_search_path` | **OK**, exact lines |
| 12 | `configs/instrument_v0.toml:113` `on_search_path = false` | **OK**, exact line — **and it is the only config checked**; see B1 |
| 13 | *"the gate is `false` in every committed config"* | **FALSE** — `gate_staged_solver_v0.toml:47`, `bench_wp18c_solver_on.toml:45`, `play_staged_solver_v0.toml:75` (B1) |
| 14 | premise memo §P2.5, *"a census run needs a seat of its own"* | **OK** — and the memo's own sentence carries the same false universal the design inherited |
| 15 | D-544, *"`Provenance` has **four** variants and the search-path solver accrues `solver_nodes` on answers returning `CompletedDepth`"* | **OK**, verbatim |
| 16 | `pvs.rs:769-770`, *"the ON seat spent a mean 156,313 nodes per position against a 50,000 budget, max 648,192"* | **OK** (the sentence spans 768-771; MEASURED as claimed) |
| 17 | D-534's 725 ms median movetime overshoot, play-config scope | **OK**, and the design's reading (blocks a play config under movetime; silent on a nodes-budget label seat) is faithful |
| 18 | D-530: 26 firings at `nodes 50000`, cap 2048, 13 on two others | **OK**, MEASURED as claimed |
| 19 | `pistol-core/src/lib.rs:86`, `:90` — the two export lines | **OK**, exact lines |
| 20 | `labels.rs:81` — `render_key_full`, the `q,r:p1`-joined canonical stone list | **OK**, exact line |
| 21 | premise memo §P2.4, *"the struct derives `Copy` … so today it cannot hold a `Vec` or a `String` without a shape change"* | **OK**, verbatim |
| 22 | `pvs.rs:249` `let key = self.position.state().key();` | **OK**, exact line (a different method from the firing site, but `GameState::key` is a field read: `board.stones_key() ^ context_key(...)`) |
| 23 | `position.rs:23-24`, *"walking its history on every take-back … a linear cost per ply in a loop that runs millions of times"* | **TEXT OK, USE INVERTED** — it is the doc of `placed`, the stack the search **does** keep, and `GameState::played()` (`state.rs:150`) publishes the move list anyway (M2) |
| 24 | `budget_token.rs:44-51` — the `[kind, amount]` arm and the third-word refusal | **OK**, exact lines |
| 25 | `configs/arena_wp20_label_pilot.toml:47-49`, *"The LABEL budget is not here — it is a command-line argument to `arena --capture`"* | **OK**, exact lines |
| 26 | D-88, *"one `info` line … whose field set is exactly `depth_turns seldepth nodes nps time hashfull score pv` in that order"* | **OK**, verbatim |
| 27 | `protocol.rs:170-172` — the `go` handler's streaming call | **OK**, exact lines (see N7 on what "with" means) |
| 28 | `census.rs:19-21`, *"the attacker proved and the defender was therefore never asked"* | **OK** |
| 29 | `census.rs:79-81` / `:79-89` — `token()`, the three literals `"none" / "impossible" / "minimal"` | **OK** |
| 30 | `census.rs:11-12`, *"It is an observation and never an input"* | **OK** |
| 31 | `pvs.rs:735-736`, *"a firing's row exists whether or not the firing proved"* | **OK** |
| 32 | `pvs.rs:623` and `:635-637` — the `is_some().then(…)` closure and its comment | **OK**, exact lines |
| 33 | `search.rs:304-307` — the root site's identical shape | **OK**, exact lines |
| 34 | D-527, *"a `Searcher` reused across fixture entries without `clear()`"* | **OK** |
| 35 | D-551, *"the score is TWO words after its key"* | **OK**, verbatim |
| 36 | D-553 = v2's `D-55y` | **OK** — D-553 is the only standing law matching both of v2's glosses; the dispatches file's own resolution note agrees |
| 37 | D-537, the denominator *"corrected twice, once for positions and once for direction"* | **OK** (D-537 records it about D-532) |
| 38 | v2's *"Both directions preserved as separate fields (D-512)"* | **v2's gloss, not D-512's words** — `decisions.md:1092` contains no "direction" at all. The standing ruling is **D-535** (*"THE CENSUS GATE RANKS BOTH DIRECTIONS PER D-512 AS REGISTERED"*), which the design does not cite. The design's answer covers the real referent (attacker/defender) in its second bullet |
| 39 | `wp21_DISPATCH.md`: *"Census: ON via the WP-2.0b token"*, *"D-537's clock starts at tranche one"*, *"committed configs untouched"*, *"via the WP-2.0b token in the pipeline's experiment config"* | **OK**, all four verbatim |
| 40 | dispatch scope 3 / obligation 4 / scope-1 parenthesis quoted as "the dispatch's" | **v1's wording under a v2 governing header** (N6) |
| 41 | §2's mechanism: `canonical_form` exported, `cell_key` public and `const`, the fold | **OK and better supported than stated** — `Key128::ZERO`, `BitXorAssign`, and a `Display` emitting exactly 32 hex digits (`zobrist.rs:28,64,70-76`) all exist; `Symmetry::ALL` is 12 (`symmetry.rs:31`), so "12 transforms" is right; `canonical_form(&[(Coord, Player)])` (`symmetry.rs:165`) is reachable at the firing site via `state.board().stones()` (`board.rs:91`), so §5's "inside the closure" claim is **verified, not merely asserted** |
| 42 | §2's sweep arithmetic (119 800 × 2.14 ≈ 256 000; × 10² ≈ 2.6·10⁷; ×100 B ≈ 2.5 GB; ×600 B ≈ 15 GB) | **arithmetic OK**; marking and per-row/per-field labelling defective (N8) |
