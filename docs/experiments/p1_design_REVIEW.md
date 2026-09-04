# P1 DESIGN — REVIEW-design, round 1

**Target.** `docs/experiments/p1_design.md` (revision 1) at the named revision
`faf7a54faa55c3405c7ba4c9055524fbbc0c8ecd` — a `git stash create` commit on
`dev` = `ffc5c10f4d16356f574e3221a023f78399d2e3bb`.

**Does that revision still match the working tree's bytes?** Yes — sha256
`e37517352a215e01…` for `git show faf7a54…:docs/experiments/p1_design.md` and
for the untracked file, checked at the start and again at the moment this
report was written. The five governing documents the stash carries (the
matrix, its selection record, the dispatch, the ledger, the prereg) are also
byte-identical to their working-tree copies; `opt_arc_ledger.md` changed on
disk DURING the review (the IMPL and mutation rows were appended — see §5),
and this report reads the stash's copy as the governing one and the live copy
as evidence of what IMPL found.
**HEAD** at review time: `ffc5c10f4d16356f574e3221a023f78399d2e3bb`.
**Date:** 2026-09-04. **Model:** `claude-fable-5-1`.

**Verdict, stated first.** VERDICT: FAIL (D1, D2). The mechanism is sound —
every attack on the premise in §1 of the brief was run to ground and none
found a legal stone sequence on which the store's answer differs from the
incumbent's (§3 below lists them). What fails is two claims the design makes
about its own falsifiers: the registered snapshot mutant M10 is EQUIVALENT
(it survives the whole oracle suite when applied as the design describes it,
and the reason the design gives for its death is false), and the packed key's
"derived" chunk range excludes two keys the design's own reader forms on every
edge stone, so an implementation that takes the design at its word panics at
the lattice edge (reproduced; the design's I6 catches it, which is why this
is MAJOR and not BLOCKING). Nine minors, most of them citations that do not
hold at the governing revision.

**Environment.** Everything adjudicated was run with `git grep` pinned to
`ffc5c10` (or to the branch commit named beside it) or `/usr/bin/grep`,
sorted `LC_ALL=C`. The three executions in §2 ran in
`/home/tom/pistol-wt/review-p1-design` (`git worktree add --detach 5988cbe`,
the head of `p1/impl`, own `CARGO_TARGET_DIR` on `/home`), removed at the end
with no branch left behind (`git worktree list` after: the main tree and the
implementer's two worktrees only); nothing in the live tree was built or
modified, and this report is the only file written. `pgrep -af
'cargo|rustc|bench_delta'` was empty before the build except for the
implementer's own polling loop over `p1_mutants_run2.log`, which was sleeping;
no timing was taken by anyone during the build to my knowledge, but D-592's
rule stands: a timing receipt taken beside this build is void — the build ran
between roughly 00:50 and 00:58 UTC.

The branches were consulted only to check that a mechanism the design
describes is implementable as described (`p1/mx-E` = `4298ecd`, `p1/impl` =
`38fbfb2` and its successor `5988cbe`); every finding below is a finding
against the DOCUMENT, checked against the code at `ffc5c10` and the game
rules.

---

## 1. Findings

### BLOCKING

None.

### MAJOR

#### D1 — M10 as the design describes it is an equivalent mutant, and the reason given for its death is false

**Claim attacked.** §4, row M10: *"snapshot filters no axis (lists windows on
all three axes through each bit) — class: snapshot — dies at I1 (a window with
no stone would be listed)"*.

**Why it cannot die.** §1.6 has the snapshot map each set bit back to its
CELL and take the windows through that cell on the bit's axis. A cell whose
bit is set holds a stone. Every window through a cell — on ANY axis — holds
that cell, so every one of the eighteen windows `windows_through_indexed`
enumerates for it holds a stone, is addressable by construction, and is in the
reference's table (`Reference::from_board` is exactly this enumeration:
`tests/common/reference.rs:56-74`, every stone × every axis × every offset).
Dropping the axis filter therefore lists the SAME window set — it is the
reference's own enumeration — and each listed window's masks are still read
through §1.2. The `BTreeMap` collects duplicates. No window with no stone can
appear, so I1's table comparison cannot see the change, and neither can
anything else.

**Reproducer, executed** (worktree at `5988cbe`, whose `snapshot` is the
design's §1.6 verbatim — `crates/pistol-solver/src/line.rs:251-275`):

```
$ python3 - <<'EOF'      # remove the filter, exactly the design's M10
p='crates/pistol-solver/src/line.rs'; s=open(p).read()
old="""                windows.extend(
                    windows_through_indexed(cell)
                        .filter(|(window, _)| window.axis == axis)
                        .map(|(window, _)| window),
                );"""
new="""                windows.extend(windows_through_indexed(cell).map(|(window, _)| window));"""
assert old in s; open(p,'w').write(s.replace(old,new))
EOF
$ cargo test --release --locked -p pistol-solver --test threat_oracle_tests
test threat_incremental_matches_reference_on_random_playouts ... ok
test every_snapshot_window_reads_back_through_masks ... ok
test threat_windows_stop_at_the_edge_of_the_addressable_lattice ... ok
test windows_straddling_a_chunk_boundary_read_as_the_reference_does ... ok
   (… all eleven …)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The unmutated tree passed the same eleven first (receipt in §2). **The
mutant survives every test in the suite, I1 included.**

**What IMPL did about it, which the design does not know.** The live ledger's
mutation row and `artifacts/p1_mutation_driver.py:22` record the M10 that was
actually run: *"snapshot's cell inverse off by one"* (`pos: index * CHUNK_LEN
+ bit + 1`), dying at I8 — a different mutant from the design's, in the same
class. So the receipt's "12 of 12 DEAD" is true of a mutation set the design
does not describe. D-590 puts the defect classes and the mutation set in the
DESIGN's hands; a class whose registered mutant cannot die has no registered
falsifier.

**Remedy.** Replace M10 with a mutant of the snapshot's ENUMERATION that
changes its output — the cell inverse off by one (IMPL's), or the ConstS
inverse `r = line - pos` mis-spelled as `line + pos` — and name the test it
dies at (I1's table comparison catches both; so does the window-set half of
I8 once the design states it, m3). Delete the sentence *"a window with no stone
would be listed"*.

#### D2 — the packed key's "derived" chunk range omits the two keys the design's own reader forms at every edge stone

**Claim attacked.** §1.1: *"the bias ranges are DERIVED from the lattice —
`q + r` over `i16` spans `[-65536, 65534]`, a chunk index over `i16` positions
spans `[-512, 511]` — and the packing function's doc states them and the
derivation, because a key whose fields can overlap is a store whose two
windows can alias."*

**What the reader forms.** §1.3 step 1 reads *"the eleven positions centred on
the stone (five either side)"* — positions `p - 5 ..= p + 5` — and §1.2's
reader keys the low chunk at `from.div_euclid(64)` and, when straddling, the
next one up. For a stone at position `p ∈ [-32768, -32764]` the low chunk is
`(-32773).div_euclid(64) = -513`; for `p ∈ [32763, 32767]` the high chunk is
`(32772).div_euclid(64) = 512`. Both are outside the range the design states,
both are formed as KEYS (the design says nothing about clipping the run to the
lattice, and the prototype does not clip), and both are reachable from a
legal stone — every corner of the `i16` lattice is a legal placement once
the region grows there, which is exactly the case I6 registers. `masks()`
alone never reaches them (a window's start is at most `32762`); only the
eleven-run does.

**Reproducer, executed.** Bias the chunk field by the design's stated range
(`512` for `[-512, 511]`, the literal reading of *"derived"*) and run the
suite:

```
$ python3 - <<'EOF'      # chunk bias 1<<10 -> 1<<9 in pack AND unpack
…
EOF
$ cargo test --release --locked -p pistol-solver --test threat_oracle_tests
test threat_windows_stop_at_the_edge_of_the_addressable_lattice ... FAILED
thread '…' panicked at crates/pistol-solver/src/line.rs:84:49:
    a chunk index lies within the lattice
test result: FAILED. 10 passed; 1 failed
```

So the design's I6 catches the literal implementation of the design's own
sentence — which is why this is not BLOCKING — and nothing else does: the ten
other tests, the random-playout oracle included, pass with the bias wrong
(the playouts never leave positions `[-31, 35]`, §2 experiment 3). A masking
implementation instead of a checking one would not panic: with a 10-bit
field, chunk `-513` aliases chunk `511` OF THE SAME LINE. I checked whether
that alias can reach a window: the aliased bits land on positions
`< -32768` or `> 32767`, every enumerated window lies wholly inside `i16`, so
no window mask ever contains an aliased bit and the alias is harmless to every
answer — an argument the design would have to make and does not, and one that
is true only because the alias happens to land off-lattice.

**The matrix and the implementation both know this and the design does not.**
Matrix §5: *"the run reader's chunk indices at the edge (−513, 512) are within
the packed key's biased fields"*. `p1/impl`'s `pack` doc (`line.rs:72-76` at
`38fbfb2`): *"a run read one chunk past either end reaches `-513` and `512`;
the biases below keep every field non-negative"*, with a bias of `1 << 10`.
The design is the document meant to stand on its own, and its stated
derivation is the one an implementer would follow.

**Remedy.** One sentence in §1.1: the chunk field must accept `[-513, 512]`
because the eleven-position run around an edge stone keys one chunk past
either end; those two chunks never hold an entry and read as zero. State the
line field the same way if the design wants field widths to be derivable from
it (the line range as stated is right).

### MINOR

#### m1 — I9 and M7 cite a test that does not exist at the governing revision

§3 I9: *"EXISTING `placing_on_an_occupied_cell_is_a_desync` (either player)"*;
§4 M7 dies at I9.

```
$ git grep -n 'placing_on_an_occupied_cell_is_a_desync' ffc5c10 38fbfb2 4298ecd -- crates
(no output)
$ git grep -n 'fn .*_is_a_desync' ffc5c10 -- crates/pistol-solver/tests
ffc5c10:crates/pistol-solver/tests/threat_oracle_tests.rs:322:fn applying_a_stone_twice_is_a_desync() {
ffc5c10:crates/pistol-solver/tests/threat_oracle_tests.rs:333:fn taking_back_a_stone_that_was_never_applied_is_a_desync() {
ffc5c10:crates/pistol-solver/tests/threat_oracle_tests.rs:341:fn taking_back_the_wrong_player_is_a_desync() {
```

The existing test is `applying_a_stone_twice_is_a_desync` (P1 at the origin,
then P2 at the origin). It kills M7 as the design intends, and the live
ledger records that the false name reached the mutation harness (*"run 1 …
M7 … a wrong test name"*). *"Either player"* overstates it: the test covers
P1-then-P2 only; the same player twice is not pinned at `ffc5c10` either, so
nothing regresses, but the parenthesis claims a coverage the test does not
have.

#### m2 — §6 carries a measured number, against D-483 and against the design's own header

Header: *"This document carries no measured number (D-483)"*. §6: *"the matrix
records a measured, exploratory gain of about two per cent early and about
none late at that seat"*. That is a measured number, quoted from the matrix,
in a design. It does no work here — the sentence's job is *"the package banks
none of it"*, which needs no figure — so D-424's test says delete the figure
and point to the matrix's §6 attack 1 (round 3's §3 accepted the paragraph
naming *"the ≈ 2 % / ≈ 0 % measurement"*, but that round reviewed the MATRIX;
D-483 binds the design).

#### m3 — I8 as described is a criterion its defect class preserves; the half that pins something is not in the design

§3 I8: *"`masks(window)` agrees with the snapshot on every window the snapshot
lists — the matrix §6 attack 2, closed by reading both."* Under §1.6 the
snapshot *"reads each window's masks through §1.2"* — the same reader
`masks()` calls — so any defect in the reader changes both sides identically
and I8 cannot fail on it. `docs/process.md`'s criterion clause names exactly
this shape (*"internal agreement between components sharing an input …
passes vacuously and is not a criterion"*). Attack 2 is closed by I1, which
compares the snapshot with the REFERENCE, as round 1 already said.

What would make I8 a criterion is what `p1/impl`'s test of that name adds
beyond the design: the snapshot's WINDOW SET equals the set
`windows_through_indexed` enumerates over the board's stones, and
`window_count()` equals that set's size. Those two are the only pins on the
snapshot's enumeration and on `window_count` (§1.6 says the latter
*"enumerates the same way"* and registers nothing that checks it); they are
also what a real M10 dies at. State them in I8.

#### m4 — M12 names a directory, not the test that drives the call site

§4 M12: *"dies at the search-crate roundtrip and identity tests that drive
`Position` (`crates/pistol-search/tests`), and the two-binary identity leg"*.
D-553's law is that the call-removed mutant *"must die at a test that drives
the call site with reachable input"*, and a receipt names that test. The live
ledger records what an unnamed target costs: *"run 3: 11 dead, M12 aimed at a
suite that never searches through `Position` — retargeted to `staged_tests`"*,
where it dies at
`a_full_search_under_staged_completes_from_the_opening_without_crashing` with
`THREAT_DESYNC`. Name it (and note the mechanism: after an un-mirrored
`Position::undo` the state still holds the stone, and the next `apply` on
that cell is refused — the refusal, not a wrong answer, is what kills it, so
a mutant that ALSO removed the refusal would need the identity leg).

#### m5 — "the crate root doc changes in three places and nowhere else" is short by one, and two other in-crate docs go stale

§2 names three passages of `lib.rs`. A fourth, `lib.rs:45-50` at `ffc5c10`,
says *"the packed key, its hasher, the table type and the class sets are
`pub(crate)`"* and *"the whole ground for the table being its own file is that
a different store replaces exactly that file"* — *"the table type"* names a
type the design removes, and `p1/impl` changed the passage (its diff touches
all four). Outside the root doc, and outside the design's list:
`cover.rs:100-113`'s doc names *"the packed key's `unpack`"* as one of *"the
two doors that could produce a window"* — under the design `unpack` produces
`(axis, line, chunk)` and no window, so the sentence describes a door that no
longer exists; and `state.rs:117`'s `table_snapshot` doc points at
`WindowTable::snapshot`. The design's Scope says `cover.rs` is unchanged.
Also, once `window_count()` *"enumerates the same way"*, the Determinism
paragraph's *"only `table_snapshot` enumerates it"* has a second enumerator
(through the same function, off any choice path — true in spirit, false as
written).

#### m6 — the Scope says `pistol-solver` only; §1.4 amends a comment in `pistol-search`

*"`pistol-solver` only … Unchanged: … every caller (`pistol-search`'s
`Position` …)"* against §1.4's *"the comment at that line says so"* — a
`position.rs` change, which `p1/impl` duly makes (+9/−6 lines). The comment is
right to change (the matrix's m8 asked for it); the Scope should say so, since
its stated purpose is *"so a reviewer can check the diff against it"*.

#### m7 — the quoted sentence is D-261's, not D-254's

§2: *"it is D-254's own sentence — 'a different store replaces exactly that
file and nothing else'"*.

```
$ /usr/bin/grep -n -o 'a different store replaces exactly that file and nothing else' docs/decisions.md
(no output)
$ /usr/bin/grep -n -o 'replaces EXACTLY that file and nothing else' docs/decisions.md
563:replaces EXACTLY that file and nothing else
$ awk 'NR<=563 && /^D-[0-9]+:/ {d=$1} END{print d}' docs/decisions.md
D-261:
```

The quoted wording is `lib.rs:48`'s; the ADR sentence is D-261's (*"the store
is its own file because the container-and-hash decision replaces EXACTLY that
file and nothing else"*), which itself cites D-254's flip clause as the
ground. The substance carries; the attribution does not.

#### m8 — §5's shape of the identity leg is not the leg the prereg registers

§5: *"the two-binary diff over both fixture sets at both determinism
budgets"*. The prereg at the same stash (`p1_bench_prereg.md:181-184`)
registers THREE seats, the third the solver-on seat at the gate's OWN budgets
(`depth_turns 2`, `nodes 10000`, `determinism.sh:76`) — the seat R2-M1 asked
for and the selection record binds the design to. The design's §6 refers to
*"the identity leg's solver-on seat"*, so the document names the leg two ways
(D-423's class). Make §5 the one place, and say three seats.

#### m9 — the defect class the design itself names — a key whose fields overlap — has no mutant, and neither does the line projection

§1.1 gives the ground for stating the ranges: *"a key whose fields can
overlap is a store whose two windows can alias."* No row of §4 mutates the
packing or the projection. Is the class covered by accident? I traced a
16-bit line field through I6's grid: ConstS line `-65536` (the corner
`(MIN, MIN)`) and line `0` (the origin's) collide, `(MIN, MIN)` and
`(MIN+1, MAX)` land in one chunk, no window reads across the alias — and the
SNAPSHOT catches it only because the merged chunk's inverse names the cell
`(MIN, 32768)` and panics on `i16::try_from`. That is a catch by luck of the
grid, not a registered falsifier. Add one mutant per class (a narrowed line
field; a ConstS projection with `pos = r`), each dying at I1 or I6, and the
§1.1 ground has a test.

Observation, not rated: §1.1's *"a chunk is removed the moment both sides'
bits are zero"* reads as D-62's SET-time rule, while under the logged undo the
only path to a zero chunk is `undo`'s restore (§1.4 step 3), and `apply` never
clears a bit. The ledger records that `38fbfb2` carried a cleared-bit branch
in the store that no caller reached and that the first M5 aimed there was
equivalent; `5988cbe` removed the branch. One clause in §1.1 — "`apply` only
ever sets a bit; the pruning lives in restore" — would have forbidden the
dead branch on the design's face.

---

## 2. Executions — receipts

All three in the review worktree at `5988cbe`, `cargo test --release
--locked -p pistol-solver --test threat_oracle_tests` unless stated, tree
restored and `git status --short` empty after each.

| # | what | result |
|---|---|---|
| 0 | unmutated | `test result: ok. 11 passed; 0 failed` (the eleven named in D1's block) |
| 1 | the design's M10: axis filter removed from the snapshot | **`ok. 11 passed; 0 failed` — SURVIVES** (D1) |
| 2 | chunk bias `1 << 9` in `pack` and `unpack` (the design's stated `[-512, 511]`) | `threat_windows_stop_at_the_edge_of_the_addressable_lattice … FAILED`, panic at `line.rs:84:49` *"a chunk index lies within the lattice"*; the other ten pass (D2) |
| 3 | a throwaway test over the twelve oracle playouts (150 plies, seeds 1–12, `random_ply`): position extremes per axis and straddling-window instances at each boundary | `SPREAD stones 1703 pos-min(ConstQ,ConstR,ConstS) [-30, -31, -31] pos-max [35, 32, 32] straddling window instances: 63/64 0, -65/-64 0, -1/0 6041` |

Experiment 3 settles two of the brief's questions by measurement: the
random-playout oracle crosses the `-1 / 0` boundary 6 041 times (so I1 does
kill M2, as §4 says) and never reaches `-65 / -64` or `63 / 64` or the lattice
edge (so I5 and I6 are the only falsifiers there, as the matrix's m5 and the
design's I5 row say).

---

## 3. The premise — attacks that did NOT land

Each was worked through against `ffc5c10`'s `state.rs:61-93` and
`pistol-core`'s `window.rs`, `axis.rs`, `coord.rs`, and against the
prototype only to confirm implementability. Listed so the next round does not
re-run them.

- **Projection table.** `Axis::direction` is `(0,1)`, `(1,0)`, `(1,-1)`
  (`axis.rs:28-30`). Along ConstS `q` rises by one per step and `q + r` is
  constant; position `q`, line `q + r`; inverse `r = line − q`. Correct, and
  the inverse is total over stored bits because only placed cells are stored.
- **Straddling, both directions.** `masks`: offset `> 58` ⇔ `offset + 6 > 64`;
  low chunk `>> offset`, high chunk `<< (64 − offset)` with the shift in
  `[1, 5]`; at offset 0 the second read is never taken. The eleven-run:
  straddles iff offset `> 53`; a stone at position 0 reads chunks `−1` and `0`
  (offset 59); at 63 reads `0` and `1` (offset 58); at 58 reads chunk 0 alone
  (offset 53, `53 + 11 = 64`, not `> 64`) — all correct, both signs.
- **Refusal on the centre bit, per axis.** Read before the flip on each axis;
  an inconsistent store (three lines disagreeing) can only arise from a bug
  and is caught on the first axis that shows it; a partial flip before the
  panic is unobservable because the panic is fatal, and the incumbent's
  per-window assert has the same shape (`state.rs:66-70` after earlier windows
  were already set).
- **Pruning at the chunk vs D-62.** Restore-from-log with vacant-before →
  remove gives a `HashMap` whose content equals the pre-stone map; `HashMap`
  and sorted-`Vec` equality are by content, so "unwound equals fresh" holds
  under the hand-written `PartialEq` (store + sets), and D-62's *"an entry
  exists exactly while its window holds a stone"* becomes *"a chunk exists
  exactly while its line-run holds a stone"* — the same invariant one level
  up.
- **Byte-identity of the class sets.** Per window, `was`/`now` are computed
  from the same six-bit masks the incumbent read (`before.with(player, index,
  true)` is the eleven-run with the centre bit set, shifted by `5 − back`);
  distinct windows never share a record, so the incumbent's interleaving of
  reads and writes within one `touch` changes nothing; transitions are
  order-free on sorted sets. The queries read only `class_windows` and
  `masks` on windows that came from the sets (`query.rs:221, :240-241, :259`;
  `cover.rs:247`), all addressable, all through-a-stone.
- **Undo's reverse transitions.** `transition(window, now, was)` is the exact
  inverse; a window touched by two stones is reversed in LIFO order, so each
  reversal sees the class set the forward step left.
- **LIFO callers, my scope.** `git grep -n -E '\bundo(_turn)?\s*\('
  ffc5c10 -- . ':(exclude)docs/decisions.md'` — every file type, `tools/`
  and docs included: 83 hits, of which the `ThreatState` undo sites are
  `position.rs:142`, `dfpn.rs:720, :722` (via `undo_turn`, called at
  `dfpn.rs:352, :512, :575` and `solver.rs:405, :415` — a turn undone
  `second` then `first`, a one-stone turn undone as one), `policy.rs:155-156`
  (`#[cfg(debug_assertions)]`), `:398-399` (a test), and
  `threat_oracle_tests.rs:222, :336, :344`. Every one is LIFO. No `.py`, `.sh`
  or `.md` code block calls the type; `tests/common/reference_walk.rs`'s
  `undo` is its own struct's. The matrix's §1.1 table is confirmed at a wider
  scope.
- **`reset_to`.** Replaces the state (`position.rs:60-62`) and rebuilds by
  `apply` in board order; `placed.clear()` guarantees no `Position::undo` ever
  reaches a root stone. The eval's board-order unwind (`:56-59`) is D-61's
  contract and never touches the threat state. The root frames live in the
  log until the next `reset_to` drops the whole state — bounded by the matrix
  §4.3's argument, not by anything the design needs to add.
- **`Clone`.** Derived; a clone owns its map and three `Vec`s; undoing on a
  clone and on the original are independent. The oracle's `recorded` stack
  relies on exactly this.
- **Chunk-key collision.** Fields `(axis, line, chunk)` are disjoint in the
  prototype (`axis << 40 | line << 16 | chunk`, line in 24 bits, chunk in
  16); the design states disjointness and leaves widths to `pack`'s doc — see
  D2 for the one range it states wrongly and m9 for the missing mutant.
- **Hand-written `PartialEq`, I7.** Forwards and backwards over the same
  stones produce logs that differ in order; under a derived `PartialEq` the
  assert at `threat_oracle_tests.rs:259` fails, so I7 kills M11; I2 does NOT
  (after a full unwind the logs are empty on both sides), and the design
  correctly names I7 alone.
- **`window_count` / `snapshot` divergence.** §1.6 has `window_count`
  enumerate *"the same way"*; the prototype spells it `snapshot().len()`, so
  the two cannot diverge — but nothing in the design's tests pins either
  count against the board (m3).
- **Rule 9 and the doctest.** `line.rs` at `38fbfb2` is 276 lines, `state.rs`
  206, `table.rs` 54 (`git show 38fbfb2:<file> | wc -l`) — no justification
  entry is owed and the design asserts no count. The retargeted `compile_fail` (`pistol_solver::line::unpack(0)`)
  is non-vacuous under `lib.rs:79-88`'s own argument as long as `line::unpack`
  takes an integer (it takes `u64` on both branches), so the only failure is
  E0603; the design's reasoning that a `table::unpack` path would pass on the
  wrong error is right.
- **D-483 beyond m2.** The design's other numbers — 64, six, eleven, eighteen,
  three, the two ranges — are derived, not measured.
- **Refusal token and message (rule 3).** `THREAT_DESYNC` verbatim; the
  `undo` refusal names the stone asked for and the frame found (`Option`
  `Debug`), the `apply` refusal names stone, player and axis. Adequate.

---

## 4. Re-derivation table — my commands, none the document's

| claim (§) | my command, with scope | my result | document's | agree? |
|---|---|---|---|---|
| axis directions (§1.1) | `git show ffc5c10:crates/pistol-core/src/axis.rs \| sed -n 26-32p` | `(0,1)`, `(1,0)`, `(1,-1)` | same | yes |
| `q + r` over `i16` (§1.1) | arithmetic: `MIN+MIN`, `MAX+MAX` | `[-65536, 65534]` | same | yes |
| chunk index of a position (§1.1) | arithmetic: `(±32768).div_euclid(64)` | `[-512, 511]` for STORED chunks | same | yes, for the store |
| chunk keys the eleven-run forms (§1.2–1.3) | arithmetic: `(-32773).div_euclid(64)`, `(32772).div_euclid(64)` | `-513`, `512` | not stated | **no — D2** |
| straddling test (§1.2) | arithmetic: `offset + 6 > 64` | `offset > 58` | `> 58` | yes |
| `offset = 0` never shifts by 64 (§1.2) | reading `run` at `4298ecd:table.rs:218-221`, `5988cbe:line.rs:175-178` | `0 + count > 64` false for `count ≤ 64` | same | yes |
| windows through a stone, enumeration (§1.3) | `git show ffc5c10:crates/pistol-core/src/window.rs \| sed -n 112-121p` | `checked_step(axis, -back)` then `Window::new` | same | yes |
| three chunk entries per stone (§1.3) | `Axis::ALL` length, `axis.rs:20` | 3 | 3 | yes |
| M10 dies at I1 (§4) | experiment 1 | survives 11/11 | dies at I1 | **no — D1** |
| M2 dies at I1 on a playout crossing a boundary (§4) | experiment 3 | 6 041 windows straddle `-1/0` in the twelve playouts | *"any playout crossing a boundary"* | yes |
| positive boundary and the edge unreached by playouts (§3 I5, I6) | experiment 3 | positions in `[-31, 35]`; 0 instances at `63/64`, `-65/-64` | m5's claim, carried | yes |
| the bias derived from §1.1's range (§1.1) | experiment 2 | panics at I6, passes the other ten | (not claimed) | **D2** |
| `undo` callers all LIFO (§1.4) | `git grep -n -E '\bundo(_turn)?\s*\(' ffc5c10 -- . ':(exclude)docs/decisions.md'`, every file type, 83 hits read | 11 `ThreatState` sites + 5 `undo_turn` wrappers, all LIFO; none in `.py`/`.sh`/`.md` | *"every caller … is LIFO (the matrix §1.1 lists them)"* | yes |
| `position.rs:48-59` unwinds the eval; `:60-62` replaces the state (§1.4) | `git show ffc5c10:crates/pistol-search/src/position.rs \| sed -n 45-71p` | doc `45-54`, loop `56-59`, replacement `60-62` | `48-59`, `60-62` | yes (the first range spans doc and loop) |
| `Reference::from_board` reads the board only (§3) | `git show ffc5c10:crates/pistol-solver/tests/common/reference.rs \| sed -n 56-74p` | `board.stones()`, `Window::new`, `RefWindow::read(window, board)` | same | yes |
| I3's line (§3) | `sed -n 228-232p` of `threat_oracle_tests.rs` at `ffc5c10` | `assert!(threats.is_empty(), …)` at 229–232, receiver on 230 | `:230` | yes |
| I1, I2, I4 (existing), I7 test names (§3) | `git grep -n '^fn ' ffc5c10 -- crates/pistol-solver/tests/threat_oracle_tests.rs` | lines 49, 203, 237, 333, 341 | same names | yes |
| I9's test name (§3) | same grep; whole-tree grep for the name | `applying_a_stone_twice_is_a_desync` at 322; `placing_on_an_occupied_cell_is_a_desync` nowhere | `placing_on_an_occupied_cell_is_a_desync`, *"EXISTING"* | **no — m1** |
| I7 kills M11 (§3, §4) | reading `threat_oracle_tests.rs:251-262`: two states from the same stones, opposite orders, `assert_eq!` on the whole state | logs differ in order → derived `PartialEq` fails | same | yes |
| `window_count()` has no caller (§1.6) | `git grep -n 'window_count' ffc5c10` whole tree | `state.rs:106` (definition), `lib.rs:41` (doc link), `decisions.md:563` (doc) | same | yes |
| the eval's edge test exists under that name (§3 I6) | `git grep -n 'eval_windows_stop_at_the_edge_of_the_addressable_lattice' ffc5c10 -- crates` | `eval_invariant_tests.rs:40` | same | yes |
| three armed configs, one a determinism seat (§6) | `git grep -n -E 'on_search_path\s*=\s*true' ffc5c10 -- configs`; `git show ffc5c10:tools/determinism.sh \| sed -n 76p` | `bench_wp18c_solver_on.toml:45`, `gate_staged_solver_v0.toml:47`, `play_staged_solver_v0.toml:75`; `staged-solver … gate_staged_solver_v0.toml … depth_turns-2 nodes-10000` | same | yes |
| "about two per cent early and about none late" (§6) | matrix §6 attack 1, `1.022 / 1.015` and `1.005 / 1.001` | a measured number in the design | *"no measured number"* (header) | **no — m2** |
| the five CI gates (§5) | `git show ffc5c10:tools/ci.sh \| grep -n 'gate 1[0-3]/\|gate 9/'` | 9 cross-process determinism, 10 differential search oracle, 11 staged generator soundness, 12 solver oracle, 13 solver determinism | same five | yes |
| the identity leg's shape (§5) | `sed -n 181-184p docs/experiments/p1_bench_prereg.md` (stash copy = tree) | three seats, the third at `depth_turns-2 nodes-10000` | *"both fixture sets at both determinism budgets"* | **no — m8** |
| "D-254's own sentence" (§2) | `/usr/bin/grep -n -o 'replaces EXACTLY that file and nothing else' docs/decisions.md` + `awk` for the owning D-line | line 563, **D-261** | D-254 | **no — m7** |
| root-doc changes: three places and nowhere else (§2) | `git grep -n -i -E 'WindowTable\|the table type\|table being its own file\|packed key' ffc5c10 -- crates/pistol-solver`; `git diff ffc5c10 38fbfb2 -- crates/pistol-solver/src/lib.rs` | four root-doc passages (`lib.rs:10, 45-50, 76, 93-95`), plus `cover.rs:108` and `state.rs:117` | three | **no — m5** |
| Scope: `pistol-solver` only (header) | `git diff --stat ffc5c10 38fbfb2` | `position.rs` +9/−6 (the §1.4 comment) | *"`pistol-solver` only"* | **no — m6** |
| `line.rs` under rule 9's cap (§2) | `git show 38fbfb2:crates/pistol-solver/src/line.rs \| wc -l`; `/usr/bin/grep -c 'pistol-solver/src/\(line\|table\|state\).rs' docs/rule9_justifications.md` | 276; 0 entries | (no count asserted — correct under rule 9) | yes |
| the retargeted doctest's type (§2) | `unpack` signatures on `4298ecd` and `38fbfb2` | `fn unpack(key: u64) -> (Axis, i32, i32)` | *"reach for `line::unpack`"* | yes |
| the matrix's prototype matches §1 (implementability) | reading `4298ecd:state.rs`, `table.rs` and `5988cbe:line.rs`, `state.rs` against §1.1–1.6 | eleven-run, per-axis refusal, three chunk entries, `Touched {window, was[2], now[2]}`, frame `(at, player, touched)`, restore-with-prune, hand-written `PartialEq` over store + sets, snapshot by set bit → cell → same-axis windows | same | yes |

---

## 5. "IMPL verifies" rows (D-590)

Rows a design reviewer cannot settle by reading; each with the concern, so
REVIEW-impl runs it rather than reads it. The live ledger already reports a
12/12 mutation receipt at `5988cbe` (`artifacts/p1_mutation_5988cbe_v1.txt`);
these rows say what that receipt must be checked FOR.

| row | concern |
|---|---|
| I4's new test | the design names `taking_back_out_of_order_is_a_desync`; `p1/impl` names it `taking_back_a_stone_that_is_not_the_last_applied_is_a_desync` and the receipt's M6 cites the latter. One of the two documents is wrong about the name; the design governs. Verify the test applies two stones and undoes the FIRST (the impl's does: origin then `(1,0)`, undo origin). |
| I5 | that stones straddle each of the three boundaries on each axis and that EVERY window through them is compared to the reference (the impl compares `windows_through_indexed` per stone plus the snapshot, and unwinds to `new()`; its comment *"so every window through them straddles"* is false for the windows starting at 56–58 and 61–63's lower neighbours — harmless, but a comment that is false). Verify M2 dies at I5 alone (the receipt says it does). |
| I6 | that the grid reaches all four corners and the run reader's `-513` / `512` keys (experiment 2 shows it does at `5988cbe`), that no panic is raised, and that the unwind reaches `new()` — and per m9, whether a narrowed line field is caught only by the snapshot's inverse panic. |
| I8 | that the test carries the window-set and `window_count` checks (m3) — it does at `38fbfb2` — and that the design is amended to say so. |
| M8 | which of the two under-count shapes the harness runs (the count one short with all entries pushed, or one entry fewer pushed); the receipt's M8 is the former (`if back != 0 { touched += 1 }`). Both die at I2 by the argument in §3; verify the receipt's wording matches the mutant applied. |
| M10 | that the receipt's mutant is the one the amended design names (D1), and that its registered test is I1 or the window-set half of I8, not the tautological half. |
| M12 | that the named test is `staged_tests::a_full_search_under_staged_completes_from_the_opening_without_crashing` and that it dies by `THREAT_DESYNC` (m4); note that a mutant removing BOTH the call and the refusal would need the identity leg — out of this set, recorded here so it is not rediscovered. |
| `reset_to`'s comment | that `position.rs`'s comment says the replacement is the only correct spelling (it does at `38fbfb2`: *"REPLACED rather than unwound, and that is the only correct spelling"*). |
| the four docs of m5 | that `lib.rs:45-50`, `cover.rs:108`, `state.rs:117` and the Determinism paragraph are true of the landed tree. |

---

## 6. What this round did not review

The bench pre-registration (`p1_bench_prereg.md`) has its own gate and was
read only for m8's fact. The matrix's selection is not re-litigated: the
three red-team rounds' surviving attack (the one-seat limb) is a limit of the
measurement and this design claims nothing at the solver's seat except
identity, which is the right shape.

---

VERDICT: FAIL (D1 — the registered snapshot mutant M10 is equivalent under the design's own §1.6 and survives the whole oracle suite on execution, so the "snapshot" defect class has no registered falsifier and the death reason *"a window with no stone would be listed"* is false; D2 — the packed key's stated chunk range `[-512, 511]` omits the `-513` and `512` keys the eleven-position run forms at every edge stone, and an implementation biased by the stated range panics at the lattice edge on execution, caught by I6 alone — a gap the matrix and the implementation both state and the design does not)
