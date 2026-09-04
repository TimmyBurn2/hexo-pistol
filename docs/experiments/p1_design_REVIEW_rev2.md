# P1 DESIGN — REVIEW-design, round 2

**Target.** `docs/experiments/p1_design.md` (revision 2) at the named revision
`8a2333bbebfa48eb40c718e0115cad73d2c0810a` — a `git stash create` commit on
`dev` = `ffc5c10f4d16356f574e3221a023f78399d2e3bb`. Revision 1 was stash
`faf7a54faa55c3405c7ba4c9055524fbbc0c8ecd`; round 1's report is
`p1_design_REVIEW.md` (*"VERDICT: FAIL (D1, D2)"*, m1–m9, one observation).

**Does that revision still match the working tree's bytes?** Yes — sha256
`aafa8944de6673e5…` for `git show 8a2333b…:docs/experiments/p1_design.md` and
for the untracked file, checked at the start (01:10 UTC) and again at the
moment this report was written. The other governing documents the stash
carries — the matrix, its selection record, the prereg, the dispatch, round
1's report — are byte-identical to their working-tree copies; `opt_arc_ledger.md`
differs in ONE row (the mutation row now reads *"14 of 14 DEAD at `ddb8a7b`"*
instead of the stash's *"12 of 12 DEAD at `5988cbe`"*), and this report reads
the stash's copy as governing and the live row as evidence of what IMPL ran.
**HEAD** at review time: `ffc5c10f4d16356f574e3221a023f78399d2e3bb`.
**Date:** 2026-09-04. **Round:** 2 of the design's gate (the last FULL round;
round 3 is remedies-only). **Model:** `claude-fable-5-1`.

**Verdict, stated first.** VERDICT: FAIL (N1). Every round-1 remedy holds on
execution or re-derivation (§1) — M10 is now a real mutant dying where the
design says, the axis filter's equivalence is stated for the right reason, the
chunk range `[-513, 512]` is derived correctly and a bias of exactly 513 passes
the suite, M13 and M14 die at I1 and I6, M12 dies by the refusal the design
names. What fails is new, and it is in the one component this design ADDS
rather than replaces: §1.3 step 4 copies `windows_through_indexed`'s loop into
`apply` rather than calling it, and I6 — the invariant that says the lattice
edge is *"handled as `windows_through_indexed` handles it"* — is pinned by a
test that never reads the class sets, which are the only thing that copy
feeds. A mutant that drops every window on an axis for a stone at the high
edge survives the whole oracle suite and both crates' full test suites
(executed, §3 X9). The remedy is executed too: comparing the sets against the
reference kills it on the design's own I6 grid. Eight minors, five of them
introduced or perpetuated by revision 2's own edits.

**Environment.** Everything adjudicated was run with `git grep` pinned to
`ffc5c10` or `ddb8a7b`, `/usr/bin/grep`, `git show`, or arithmetic. The
executions in §3 ran in `/home/tom/pistol-wt/review-p1-design-r2` (`git
worktree add --detach ddb8a7b`, the head of `p1/impl`, own `CARGO_TARGET_DIR`
`/home/tom/pistol-wt/review-p1-design-r2-target`), removed at the end with no
branch left; nothing in the live tree was built or modified, and this report is
the only file written there. `pgrep -af 'cargo|rustc|bench_delta|determinism'`
was empty at 01:10:39 UTC before the build; the build and the test runs
occupied roughly 01:11–01:36 UTC, and per D-592 a timing receipt taken beside
them is void. The branch was consulted only to check that the mechanism as
described is implementable and to execute the registered mutants against real
tests; every finding is a finding against the DOCUMENT.

---

## 1. Round-1 remedies, each verified by execution or re-derivation (D-591)

| finding | remedy in revision 2 | how I executed / re-derived it | holds? |
|---|---|---|---|
| **D1** M10 equivalent | M10 is *"the snapshot's cell inverse off by one (`pos + 1`)"*, dying at I8; the axis filter is named as an equivalent mutant *"found by the session and by round 1"* and not registered | **X1** (§3): `pos: index * CHUNK_LEN + bit + 1` → `every_snapshot_window_reads_back_through_masks` FAILED at `threat_oracle_tests.rs:500` *"seed 1: the snapshot's window set"* (the window-set half, not the tautological half); it also dies at I1 (*"the window table"*), I5 and I6 (the inverse panics `PosOverflow` at `line.rs:66`). **X2**: the filter removed → `11 passed; 0 failed`. The reason given — every window through an occupied cell holds that stone, so the unfiltered union is the reference's own enumeration three times over — is the reason round 1 gave and is correct | **yes** |
| **D2** chunk range omitted the reader's reach | §1.1: *"the eleven-position run around a stone at either end of the lattice keys one chunk past that — `-513` below, `512` above — so the chunk field accepts `[-513, 512]`; those two chunks never hold an entry and read as zero"* | arithmetic: `(-32768 - 5).div_euclid(64) = -513` for stones at positions `[-32768, -32764]`; `(32767 + 5).div_euclid(64) = 512` for `[32763, 32767]`; `masks()` never reaches either (a window start is at most `32762`, offset 58, `58 + 6 = 64` is not `> 64`); `place` keys only the stone's own chunk, so the two are never written. **X6**: bias exactly 513 in `pack` and `unpack` → `11 passed`. **X7**: bias 512 (revision 1's range) → I6 panics at `line.rs:84` *"a chunk index lies within the lattice"*, the other tests pass | **yes** |
| m1 I9's test name | `applying_a_stone_twice_is_a_desync` *"(P1 at the origin, then P2 there)"* | `git grep -n 'fn applying_a_stone_twice_is_a_desync' ffc5c10 -- crates` → `threat_oracle_tests.rs:322`; its body applies `ORIGIN` P1 then `ORIGIN` P2 (`:327-328`) | yes |
| m2 a measured number in §6 | §6 says *"the matrix (§6, attack 1) records an exploratory measurement there"* and no figure | `/usr/bin/grep -n -E '[0-9]+(\.[0-9]+)? ?(%\|per cent)\|[0-9]\.[0-9]{2,}' docs/experiments/p1_design.md` → no hit; the matrix's §6 attack 1 does record that measurement | yes |
| m3 I8 tautological | I8 is *"the snapshot's WINDOW SET is exactly the set `windows_through_indexed` enumerates over the board's stones, and `window_count()` is that set's size"*, with the masks read-back named as pinning nothing on its own | the referent is the BOARD's stones through `pistol-core`'s enumeration — external to the state (docs/process.md's criterion clause); X1 dies at exactly that assert | yes |
| m4 M12 named a directory | `staged_tests.rs`'s `a_full_search_under_staged_completes_from_the_opening_without_crashing`, with the mechanism (the next `apply` on the un-undone cell is refused) | **X8**: `Position::undo`'s call removed → that test FAILED with `THREAT_DESYNC: p1 stone on 0,0 lands on a cell of its ConstQ line that already holds one` (`state.rs:95`). The test drives `Searcher::search` from the opening under a staged config (`staged_tests.rs:288-313` at `ffc5c10`) — reachable input, D-553 | yes |
| m5 root-doc count short, two in-crate docs stale | *"four passages"* listed by content; `cover.rs`'s `min_hitting_set_exceeds` doc and `state.rs`'s `table_snapshot` doc named | `git show ffc5c10:crates/pistol-solver/src/lib.rs \| grep -n -E 'per-window record\|the table type\|window TABLE is hashed\|only \[`ThreatState::table_snapshot`\] enumerates\|table::unpack'` → lines 10, 45, 76, 86, 95 — the four passages the design names are 10, 45-50, 76, 93-95; **line 86 is a fifth** (N4). `git diff ffc5c10 ddb8a7b -- crates/pistol-solver/src/{lib,cover,state}.rs` changes the four, `cover.rs:108` and `state.rs:117` (*"see `LineStore::snapshot`"*) | yes, short by one (N4) |
| m6 Scope said `pistol-solver` only | Scope: *"In `pistol-search`: ONE comment, on `Position::reset_to`'s replacement of the state (§1.4), and no code"* | `git diff --stat ffc5c10 ddb8a7b -- crates/pistol-search` → `position.rs` only, a doc-comment change (+6/−3) | yes |
| m7 D-254 for D-261 | *"the crate root doc's own sentence … D-261's ground with D-254's flip clause behind it"* | `/usr/bin/grep -n -o 'replaces EXACTLY that file and nothing else' docs/decisions.md` → 563 (D-261); the lowercase quote is `lib.rs:48`; D-261 cites D-254's flip clause as its ground | yes |
| m8 §5's identity leg not the prereg's | §5 defers to `p1_bench_prereg.md` §2 as *"the one place"*, says THREE seats (the two WP-1.9 seats at the determinism gate's budgets, the solver-armed seat at its own); §6 points at §5 | prereg §2.1 `SEATS` heredoc: `tactical_staged_v0` and `instrument_v0`/`bench_positions_v1` at `depth_turns-4 nodes-200000`, `gate_staged_solver_v0` at `depth_turns-2 nodes-10000`; `tools/determinism.sh:85` `BUDGETS=("depth_turns 4" "nodes 200000")`, `:76` the solver seat; `artifacts/wp19b_byte_identity_v2.txt:4-6` names the same two fixtures, configs and budgets | yes |
| m9 no mutant for the key or the projection | M13 *"ConstS projection reads the position as `r`"* → I1; M14 *"line field narrowed to eight bits"* → I6 | **X3**: M13 → I1 FAILED *"seed 1 ply 1: the window table"* (also I5, I6, I8). **X4** (IMPL's spelling, `pack` masked `& 0xFF`): M14 → I6 FAILED — by the centre-bit REFUSAL `THREAT_DESYNC: p2 stone on -32768,0 lands on a cell of its ConstR line that already holds one`, the fourth stone; it also dies at I1, I5 and I8 through the snapshot inverse's `i16::try_from` panic (`line.rs:66`, `NegOverflow`) because `pack` and `unpack` stop being inverses. **X5** (a consistent 8-bit field masked in both, bias 128 — pure aliasing): I6 FAILED by the same refusal; I1, I5, I8 PASS — so I6 is the class's only killer, as the design says | *"dies at"* yes; the mechanism parenthetical no (N7) |
| observation: `apply` never clears a bit | §1.1: *"`apply` only ever sets a bit; the pruning lives in `restore` … the store has no bit-clearing operation at all — a branch that cleared one would be unreachable"* | reading; the branch's `place` sets and `restore` writes back or removes | yes, forbidden on its face |

---

## 2. New findings

### BLOCKING

None.

### MAJOR

#### N1 — I6 registers a pin it cannot deliver for the enumeration this design copies into `apply`, and no mutant of that class is registered

**Claim attacked.** §3 I6: *"The lattice edge is handled as
`windows_through_indexed` handles it — NEW
`threat_windows_stop_at_the_edge_of_the_addressable_lattice`: stones at the
four `i16` corners and along each edge, snapshot equals the reference, no
panic."* Against §1.3 step 4: *"For each of the six windows through the stone
on this axis — enumerated exactly as
`pistol_core::window::windows_through_indexed` enumerates them, by
`Coord::checked_step` back and `Window::new` forward, so a window that runs
off the addressable lattice is skipped here iff it is skipped there."*

**Why the test cannot see the class.** The *"iff"* is a claim of equivalence
between two implementations: `apply` does not call `windows_through_indexed`,
it re-runs the loop (`git grep -n windows_through_indexed ddb8a7b --
crates/pistol-solver/src` → `line.rs:4, :254` only — the snapshot; at
`ffc5c10` it was `state.rs:62`, inside `touch`). The only consumer of that
copy is the CLASS SETS: the store's bits are set per cell by `place`,
independent of which windows the loop visits, and the snapshot enumerates
windows through `pistol-core`'s own `windows_through_indexed(cell)`. I6's
registered checks are *"snapshot equals the reference, no panic"* — at
`ddb8a7b` the test's body is `assert_matches_board` (per-window `masks`
against the board, snapshot against `Reference::from_board`) and
`window_count() > 0` (`threat_oracle_tests.rs:460-463`); it reads no class set
(the only set reads in the file are `compare`'s at `:118, :134` for I1 and
I7's at `:267-268`). So I6 compares core's enumeration with itself and with
the board, and never with the copy. I1's playouts, which do compare the sets,
stay within positions `[-31, 35]` (round 1's experiment 3), where
`Window::new` never returns `None` and the copy cannot diverge.

**Reproducer, executed** (worktree at `ddb8a7b`; the mutant is the one edge
branch of the copy — `continue` → `break` on `Window::new`'s `None` in
`apply`, `state.rs:104-106`, which drops every window on that axis for a
stone at position `MAX` or `MAX − 1` and is byte-identical off the edge):

```
=== X9 apply's window loop: break instead of continue on Window::new's None
  whole oracle suite: pass (SURVIVES)  (test result: ok. 11 passed; 0 failed)
  cargo test --release --locked -p pistol-solver: rc=0, no FAILED line
  cargo test --release --locked -p pistol-search: rc=0, no FAILED line
```

**It is not equivalent.** A throwaway test comparing the five class sets
(`hot`, `win_in_one_ply`, `completed`, `live` at two and three) against
`Reference` on the design's own I6 grid, and on a six-stone grid with two
same-side stones within one window at the high edge of each axis:

```
UNMUTATED:  probe_a_the_design_grid_with_the_class_sets_compared ... ok
            probe_b_two_same_side_stones_within_one_window_at_the_high_edge_of_each_axis ... ok
MUTATED:    probe_a ... FAILED  the design's grid p1: live Two
              left:  "ConstS@-32768,32767"
              right: "ConstS@-32768,32767 ConstS@32762,-32763"
            probe_b ... FAILED  after 32766,0 p1: live Two
              left:  "-"       right: "ConstR@32762,0"
```

So under the mutant P1's live-two set lacks a window the reference holds, on
the design's own grid — a wrong answer to `live_windows_at_count`, which
`cells_raising_to_hot` reads — and nothing registered can see it.

**Why MAJOR and not BLOCKING.** The defect is reachable only at the lattice
edge, which `Window::new`'s own doc calls *"unreachable in any game"*; the
incumbent at `ffc5c10` has no copy to drift because `touch` calls the
enumeration. It is MAJOR because D-590 puts invariants and the mutation set's
defect classes in the design's hands, I6 claims a pin for the sets it does not
have, and §4 has no row for the class — the same shape as round 1's D1.

**Remedy, executed.** Either of two, and the second removes the class:

1. I6 compares the class sets against the reference, as `compare` does — the
   probe above IS that comparison and it kills X9 on the design's grid; and
   §4 registers X9 (*"`apply`'s window loop stops at the first window off the
   lattice"*, class: edge enumeration) dying at I6. Note the probe kills it on
   the grid as it stands; no new stones are needed.
2. §1.3 step 4 CALLS `windows_through_indexed(at)` and selects the run by
   `window.axis` (the three runs read up front, in `Axis::ALL` order, which
   is the order the iterator yields) — then there is no second enumeration,
   the *"iff"* sentence goes, and the crate root doc's rule-2 sentence
   *"the window enumeration … all come from there"* (kept at
   `ddb8a7b:lib.rs:11-13`) is true rather than a claim about a copy. If the
   design keeps the copy for a reason, the reason is a rule-10 line and the
   design does not state one.

### MINOR

#### N2 — §1.1 puts M13 in the key-aliasing class; §4 puts it in the projection class

§1.1:61: *"a key whose fields can overlap is a store whose two windows can
alias, and §4's M13 and M14 are the mutants for that class"*. §4 classes M13
*"projection"* and M14 *"key-field overlap"*. A ConstS projection that reads
the position as `r` maps `(q, r) ↦ (q + r, r)`, which is injective — no two
cells share a line position — so M13 is a misread, not an alias (X3 dies on
*"the window table"*, not on a refusal). One mutant, two classes, two sections
(D-423). `/usr/bin/grep -n 'M13' docs/experiments/p1_design.md` → 61, 229.
Fix: *"§4's M14 is the mutant for that class"*.

#### N3 — §1.1's *"the only enumeration is the snapshot (§1.4)"* cites the wrong section

§1.4 is `undo`; the snapshot is §1.6. Present in revision 1 unchanged (the
diff does not touch line 72); round 1 missed it.

#### N4 — the root doc has a fifth passage naming the old path, and it is unchanged on the branch

§2:170: *"The crate root doc changes in four passages"*. `git grep -n
'table::unpack' ffc5c10 ddb8a7b -- crates/pistol-solver/src/lib.rs` →
`ffc5c10:76` (the doctest, listed), `ffc5c10:86` (*"a re-export … `pub use
table::unpack` here at the root — leaves both examples failing"*, not
listed), `ddb8a7b:89` (the same sentence, still `table::unpack`, while the
doctest above it now reaches for `line::unpack`). The sentence names the one
door the guard does not cover, by a path that no longer exists. m5's class,
one short at the design's own scope.

#### N5 — §1.5 excludes *"the two logs"*; §1.3 pushes onto three stacks

§1.3 pushes onto *"the chunk log"*, *"the window log"* and *"the stone's
FRAME"*; §1.4 pops all three. §1.5:139-140: *"written by hand over the store
and the sets and exclude the two logs"*. The positive clause is right and is
what the branch implements (`state.rs:66-70`); the count is off by one, and a
derived `PartialEq` (M11) would fold in three, not two. Pre-existing; missed
by round 1.

#### N6 — I5 attributes to the red team a plural it did not say, and round 1 measured the plural false

§3 I5:205: *"the matrix's round-1 red team (m5) named the negative pairs as
the ones with a live falsifier"*. `sed -n 183,191p
docs/experiments/matrix_P1_threat_state_REDTEAM.md`: *"the negative pair is
the one with a live falsifier today"* — singular, −1 / 0. Round 1's
experiment 3: 0 straddling instances at −65 / −64 in the twelve playouts. The
plural claims a falsifier for −64 / −65 that does not exist until I5 does.
Pre-existing; missed by round 1.

#### N7 — M14's mechanism is not how it dies

§4:230: *"I6 (line `-32768` aliases line `0` on the lattice-edge grid; windows
on both read each other's stones)"*. X4 and X5 (§1, m9): the death is the
centre-bit REFUSAL at the fourth stone — the grid puts the same seven
positions on every aliased pair of lines, so `(−32768, 0)`'s ConstR line
(line 0, aliasing line −32768) already holds the bit at position −32768 from
`(−32768, −32768)` and `apply` refuses before any window is read across the
alias. Under IMPL's spelling the mutant also dies at I1, I5 and I8 through the
snapshot inverse's panic — the *"catch by luck"* round 1's m9 named — because
`pack` and `unpack` stop being inverses, a second class the one mutant
conflates. The design states M12's mechanism with exactly this care (*"the
refusal kills it, not a wrong answer"*); M14's row should say *"dies at I6 by
`THREAT_DESYNC`: the aliased line already holds the bit"*. Fixture-level
under D-590, so MINOR with its execution and never a gate matter.

#### N8 — *"a driver committed as a receipt (`artifacts/p1_mutation_driver.py`)"* cannot be committed

§4:233. `/artifacts/` is gitignored (`.gitignore:19`, *"CLAUDE.md rule 8"*);
the driver is sha-anchored (the ledger names its digest), never committed.
Introduced by revision 2. Say *"sha-anchored"*.

#### N9 — the Scope's *"Unchanged: … the `WindowMasks` type"* is not true of the diff it exists to check

Scope:28. `git diff ffc5c10 ddb8a7b -- crates/pistol-solver/src/table.rs`
removes two `pub(crate)` methods from the type, `is_vacant` and `with` —
D-261's *"the store's own"*, rightly gone with the store. The public surface
(two fields, five accessors) is unchanged, which is what the Scope should
say, since its stated purpose is *"so a reviewer can check the diff against
it"*. Pre-existing; missed by round 1.

**Observations, not rated.** §4 M9 still says *"after the flip"* where §1.3
step 3 now says *"Set"*. M13's *"(every ConstS window misread on the
playouts)"* is a loose universal — a window whose only stone sits at index 0
with nothing on the line behind it reads right. The branch's I5 comment *"so
every window through them straddles"* (`ddb8a7b:threat_oracle_tests.rs:395`)
is still false for the windows round 1 named — carried as an IMPL row.

---

## 3. Executions — receipts

All in the review worktree at `ddb8a7b`, `cargo test --release --locked -p
pistol-solver --test threat_oracle_tests [name -- --exact]` unless stated,
tree restored by `git checkout -- crates` after each and `git status
--porcelain -- crates` empty at the end (`TREE CLEAN`).

| # | what | result |
|---|---|---|
| X0 | unmutated, whole oracle suite | `ok. 11 passed; 0 failed` |
| X1 | design M10: `pos + 1` in the snapshot's inverse | I8 **FAILED** (`:500` *"the snapshot's window set"*); I1 FAILED (*"the window table"*); I6 FAILED (`line.rs:66` `PosOverflow`); I5 FAILED |
| X2 | the axis filter removed from the snapshot (the design's equivalent mutant) | `ok. 11 passed; 0 failed` — **SURVIVES**, as the design says |
| X3 | design M13: ConstS `pos: r` | I1 **FAILED** (*"seed 1 ply 1: the window table"*); I5, I6, I8 FAILED |
| X4 | design M14, IMPL's spelling: `pack` line field `& 0xFF`, `unpack` unchanged | I6 **FAILED** by `THREAT_DESYNC: p2 stone on -32768,0 lands on a cell of its ConstR line that already holds one` (`state.rs:95`); I1, I5, I8 FAILED (`line.rs:66` `NegOverflow`); I2 passed |
| X5 | an 8-bit line field masked in `pack` AND `unpack`, bias 128 (pure aliasing) | I6 **FAILED** by the same refusal; I1, I5, I8 **passed** |
| X6 | D2: chunk bias exactly 513 in `pack` and `unpack` | `ok. 11 passed; 0 failed` |
| X7 | chunk bias 512 (revision 1's range) | I6 FAILED (`line.rs:84` *"a chunk index lies within the lattice"*, `NegOverflow`); I1 passed |
| X8 | design M12: `Position::undo`'s call to the state's `undo` removed | `pistol-search --test staged_tests a_full_search_under_staged_completes_from_the_opening_without_crashing` **FAILED** by `THREAT_DESYNC: p1 stone on 0,0 lands on a cell of its ConstQ line that already holds one` (`state.rs:95`) |
| X9 | `apply`'s window loop: `break` for `continue` on `Window::new`'s `None` | oracle suite `ok. 11 passed`; `cargo test -p pistol-solver` rc 0; `cargo test -p pistol-search` rc 0 — **SURVIVES** (N1) |
| probe | a throwaway `tests/r2_edge_probe.rs` comparing the five class sets against `Reference` on the I6 grid and on a six-stone high-edge grid | unmutated `ok. 2 passed`; under X9 `FAILED. 0 passed; 2 failed` (N1's block); file removed, tree clean |

---

## 4. Re-derivation table — my commands, none the document's

| claim (§) | my command, with scope | my result | document's | agree? |
|---|---|---|---|---|
| stash = tree (header) | `git show 8a2333b…:docs/experiments/<f> \| sha256sum` vs `sha256sum docs/experiments/<f>`, seven files | six SAME; `opt_arc_ledger.md` DIFF in the mutation row only | (implicit) | yes |
| line range (§1.1) | `MIN + MIN`, `MAX + MAX` | `[-65536, 65534]` | same | yes |
| stored chunk range (§1.1) | `(±32768).div_euclid(64)` | `[-512, 511]` | same | yes |
| the reader's reach (§1.1) | `(-32773).div_euclid(64)`, `(32772).div_euclid(64)`; X6, X7 | `-513`, `512`; bias 513 passes, 512 panics at I6 | `[-513, 512]` | yes |
| straddling tests (§1.2, §1.3) | `offset + 6 > 64`, `offset + 11 > 64` | `> 58`, `> 53` | `> 58` (the eleven-run's threshold is not stated, correctly) | yes |
| `undo` reverses by binary search (§1.4 step 2) | `git show ffc5c10:crates/pistol-solver/src/sets.rs \| grep -n -A25 'fn transition'` | `insert`/`remove` by `binary_search` (`:179`, `:190`) | same | yes |
| `position.rs:48-59`, `:60-62` (§1.4) | `git show ffc5c10:crates/pistol-search/src/position.rs \| sed -n 45,63p` | doc 45-54, loop 56-59, replacement 60-62 | `48-59`, `60-62` | yes (as round 1 read it) |
| the round-1 red team's m8 (§1.4) | `sed -n 222,240p …REDTEAM.md` | *"the `ThreatState::new()` replacement at `:60-62` stops being a convenience and becomes the only correct spelling"* | same | yes |
| `window_count()` has no caller (§1.6) | `git grep -n 'window_count' ffc5c10 -- crates tools docs`, less `decisions.md` | `state.rs:106` (definition), `lib.rs:41` (doc link) | same | yes |
| the enumeration is called, not copied (§1.3 step 4, root doc) | `git grep -n 'windows_through_indexed' ffc5c10 ddb8a7b -- crates/pistol-solver/src` | `ffc5c10: state.rs:62` (in `touch`); `ddb8a7b: line.rs:4, :254` (the snapshot only) | *"enumerated exactly as … iff"* | **no — N1** |
| I6 reads the sets (§3 I6) | `git show ddb8a7b:…threat_oracle_tests.rs \| grep -n -E 'hot_windows\|live_windows_at_count'` | `:118, :134` (I1's `compare`), `:267-268` (I7); none in I6 (`:431-470`) | *"handled as `windows_through_indexed` handles it"* | **no — N1** |
| D-261's sentence (§2) | `/usr/bin/grep -n -o 'replaces EXACTLY that file and nothing else' docs/decisions.md`; `lib.rs:48` | 563 = D-261; the lowercase quote at `lib.rs:48` | same | yes |
| root-doc passages, four (§2) | `git show ffc5c10:…lib.rs \| grep -n -E 'per-window record\|the table type\|window TABLE is hashed\|only \[`ThreatState::table_snapshot`\] enumerates\|table::unpack'` | 10, 45, 76, **86**, 95 | four | **no — N4** |
| in-crate docs, two (§2) | `git grep -n -E 'WindowTable\|packed key\|the table type' ffc5c10 -- crates/pistol-solver/src crates/pistol-search/src`, less `table.rs` | `cover.rs:108`, `state.rs:117` (plus `use` lines) | same two | yes |
| rule 9 (§2) | `git show ddb8a7b:…/{line,state,table}.rs \| wc -l`; `/usr/bin/grep -n 'pistol-solver/src/' docs/rule9_justifications.md` | 265 / 206 / 54; entries for cover, dfpn, policy, solver, tt, zone only | no count asserted | yes |
| I3's line (§3) | `sed -n 228,232p` at `ffc5c10` | `is_empty()` assert at 229-232 | `:230` | yes |
| I4's new test name (§3) | `git grep -n -E 'taking_back_out_of_order_is_a_desync\|taking_back_a_stone_that_is_not_the_last_applied_is_a_desync' ddb8a7b -- crates` | only the latter, `:349` | the former | IMPL verifies (carried) |
| the eval's edge test (§3 I6) | `git grep -n 'windows_stop_at_the_edge_of_the_addressable_lattice' ffc5c10 -- crates` | `eval_invariant_tests.rs:40` | same | yes |
| the red team's m5 (§3 I5) | `sed -n 183,191p …REDTEAM.md` | *"the negative pair is the one"* — singular | *"the negative pairs"* | **no — N6** |
| I9's test (§3) | `git grep -n 'fn applying_a_stone_twice_is_a_desync' ffc5c10 -- crates` | `:322`, P1 then P2 at the origin | same | yes |
| M12's test drives the call site (§4) | `git show ffc5c10:crates/pistol-search/tests/staged_tests.rs \| sed -n 288,313p` | `Searcher::search` from `new_game` at `DepthTurns(2)` under a staged config | same | yes |
| the driver is committed (§4) | `git show ffc5c10:.gitignore \| grep -n artifacts` | `:19 /artifacts/` | *"committed as a receipt"* | **no — N8** |
| three seats, the gate's budgets (§5) | `sed -n` prereg §2.1's `SEATS`; `tools/determinism.sh:76, :85`; `artifacts/wp19b_byte_identity_v2.txt:4-6` | three seats; `depth_turns 4` / `nodes 200000`; the solver seat at `depth_turns-2 nodes-10000`; WP-1.9's two are those fixtures at those budgets | same | yes |
| the five CI gates (§5) | `git show ffc5c10:tools/ci.sh \| grep -n 'gate'`; `tools/tactical_check.sh:35`; `configs/gate_v0.toml:61-62` | gates 9–13 as named; gate 8 runs `tactical_v0.txt` under `gate_v0.toml` `kind = "radius"` and reads no threat state, so the prereg's sixth gate is extra, not a contradiction | five | yes |
| three armed configs, one a determinism seat (§6) | `git grep -n -E 'on_search_path\s*=\s*true' ffc5c10 -- configs`; `determinism.sh:76` | `bench_wp18c_solver_on:45`, `gate_staged_solver_v0:47`, `play_staged_solver_v0:75`; the gate seat | same | yes |
| no measured number (header, §6) | the m2 grep above | no hit | same | yes |
| *"nine minors"* (header) | `p1_design_REVIEW.md` §1 | m1–m9 | same | yes |
| matrix citations (§1.1 revision 3 under D-598; §1.4 callers; §2 §4.2; §3 I8 attack 2; §6 attack 1) | reading the stash's matrix | header; §1.1's ten sites; §4.2 *"the design owes the split"*; §6 attacks 1 and 2 | same | yes |
| M10 dies at I8, the filter is equivalent (§4) | X1, X2 | dies at I8's window-set assert; the filter survives 11/11 | same | yes |
| M13 at I1, M14 at I6 (§4) | X3, X4, X5 | as stated; M14 by refusal, not by a read | *"windows on both read each other's stones"* | **no — N7** |
| M12 by the refusal (§4) | X8 | `THREAT_DESYNC` on the next `apply` at `0,0` | same | yes |
| the prototype matches §1 (implementability) | reading `ddb8a7b:line.rs`, `state.rs` against §1.1–1.6 | eleven-run, per-axis refusal, three chunk entries, `Touched`, `Applied`, restore-with-prune, hand-written `PartialEq` over store + sets, snapshot by set bit → cell → same-axis windows | same | yes |

---

## 5. "IMPL verifies" rows (D-590)

Rows a design reviewer settles by execution rather than by reading; the ones
executed here are marked discharged. The live ledger reports *"14 of 14 DEAD
at `ddb8a7b`"* (`artifacts/p1_mutation_ddb8a7b_v1.txt`, sha256 `8bbc2790…`;
driver `fe0722a6…`); these rows say what that receipt must be checked FOR.

| row | concern |
|---|---|
| I4's new test | the design names `taking_back_out_of_order_is_a_desync`; the branch names it `taking_back_a_stone_that_is_not_the_last_applied_is_a_desync` (`:349`) and the receipt's M6 cites the latter. The design governs; one of the two changes. Verify the body applies two stones and undoes the FIRST (it does: origin then `(1,0)`, undo origin). |
| I5 | that stones straddle each of the three boundaries on each axis and every window through them is compared to the reference (the branch does; its comment at `:395` *"so every window through them straddles"* is still false for the windows round 1 named — harmless, but false). Verify M2 dies at I5 alone (the receipt says it does). |
| I6 | that the grid reaches the corners and the `-513` / `512` keys (X7 shows it does), no panic, unwind to `new()`; **and, after N1's remedy, that the class sets are compared against the reference at the edge and X9 dies there** — or that `apply` calls `windows_through_indexed` and X9 is no longer expressible. |
| I8 | that the test carries the window-set and `window_count` checks — it does at `ddb8a7b` (`:493-504`); X1 dies at `:500`. Discharged as a design matter. |
| M8 | the row's label (*"one fewer window entry pushed"*) and parenthetical (*"`touched` under-counted"*) still name two shapes; the driver runs the count shape (`if back != 0 { touched += 1 }`). Both die at I2; the receipt's wording should match the mutant applied. |
| M10 | discharged: X1 is the amended design's mutant and dies at I8's window-set assert. |
| M12 | discharged: X8, `THREAT_DESYNC` on the next `apply`. A mutant removing BOTH the call and the refusal still needs the identity leg — recorded, out of this set. |
| M14 | the receipt records DEAD at I6; verify it names the refusal as the death (N7) and note that under IMPL's spelling it also dies at I1/I5/I8 by the inverse's panic (X4) — a mutant that dies for a reason other than its class is a receipt that counts the wrong kill. |
| `reset_to`'s comment | at `ddb8a7b:position.rs:52-57`: *"REPLACED rather than unwound, and that is the only correct spelling"* — present. |
| the docs of m5, plus N4 | that `lib.rs`'s four passages, `cover.rs:108`, `state.rs:117` are true of the landed tree — and that `lib.rs:89`'s *"`pub use table::unpack`"* is retargeted (it is not at `ddb8a7b`). |
| `pack`'s doc (§1.1) | that it states the derivation — at `ddb8a7b:line.rs:70-75` it states the stored range, the reader's reach to `-513` and `512`, and that the biases keep every field non-negative. Adequate; verify the widths named (24 of line above 16 of chunk) are the ones in the code. |
| the ADR line (§2) | exists at the landing, names the flip clause, the LIFO contract and the hand-written equality. |

---

## 6. What this round did not review

The bench pre-registration has its own gate and was read for m8's fact and
for §5's three seats only. The matrix's selection is not re-litigated. Round
1's §3 (the premise attacks that did not land) was not re-run; nothing in
revision 2 touches the mechanism those attacks were aimed at, and the X-series
above executed every remedy that named a mutant, a range or a test.

---

VERDICT: FAIL (N1 — I6's registered test reads the store and the snapshot and never the class sets, which are the only consumer of the `windows_through_indexed` loop §1.3 step 4 copies into `apply`; a mutant that drops every window on an axis for a stone at the high edge survives the whole oracle suite and both crates' full suites on execution, is non-equivalent on the design's own I6 grid, and belongs to a class §4 does not register — the remedy, comparing the sets against the reference at the edge or calling the enumeration instead of copying it, is named and executed)
