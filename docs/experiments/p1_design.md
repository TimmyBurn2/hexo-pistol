# P1 — DESIGN: the threat state's per-axis line bitboards with a logged undo (revision 3, amended after REVIEW-impl round 1)

**Amendments after the gate closed, listed because Scope, §2 and §3 are frozen
sections (D-54y).** REVIEW-impl round 1 (`p1_impl_REVIEW.md`) returned
*"VERDICT: FAIL (F1 …)"* on one BLOCKING finding that is documentary — the ADR
line §2 requires was never appended, now `D-603` — and four minors. Two are the
design's: **F5**, §2's list of in-crate docs to correct is one file short
(`sets.rs` says *"the table"* twice of a store that is now line bitboards), so
§2 names it and Scope no longer calls that file's docs unchanged; and **F4**,
I5's own comment overstated which windows straddle, corrected in the test. The
mechanism, the invariants and the mutation set are untouched: the review
reproduced the receipt byte for byte, killed eight further defect classes of
its own, and found no wrong answer on any check.

**Revision 3 is remedies-only, the third and last round the grant holds.**
Round 2 (`p1_design_REVIEW_rev2.md`, at stash `8a2333b`) executed every
round-1 remedy and found all holding, then returned *"VERDICT: FAIL (N1)"*
with eight minors. N1: §1.3 copied `pistol-core`'s window enumeration into
`apply`, and a mutant truncating that copy at the lattice edge survived every
test because I6 read the store and never a class set. The remedy is both
halves the review named: `apply` takes its windows from
`windows_through_indexed` itself (§1.3), and I5/I6 read every class set
against the reference (§3), with M15 registered for the class (§4). N2–N9 are
one-line corrections, each at the line the review cites.

**Revision 2 answered REVIEW-design round 1** (`p1_design_REVIEW.md`, at stash
`faf7a54`: *"VERDICT: FAIL (D1, D2)"*, nine minors; every premise attack on
the mechanism was run to ground and none landed). What changed: §4's M10 is
the snapshot's cell inverse rather than an equivalent mutant (D1); §1.1
states the run reader's reach one chunk past the lattice (D2); the test names,
the scope, the attribution, the identity leg's shape, the root-doc list and
the two mutants for the key and projection classes (m1–m9). This is round 2
of the design's gate; round 3 is remedies-only.

**Governing.** The optimization arc dispatch's P1 (`opt_arc_DISPATCH.md`,
tranche 1), `matrix_P1_threat_state.md` revision 3 (amended under D-598 after
its third round) and its selection record
(`matrix_P1_threat_state_selection.md`), which takes **O-E**. This document
carries no measured number (D-483): what it says is mechanism, invariants,
refusals, and the tests and mutants that pin them. The numbers a reader wants
are in the matrix (the option's measurement) and will be in the package's bench
pre-registration and its results (the landing's).

**Scope, stated so a reviewer can check the diff against it.** In
`pistol-solver`: the store under `ThreatState` changes representation,
`apply`/`undo` change how they maintain the sets, the crate's root doc is
amended where it describes the store (§2 lists every passage), and three
in-crate doc comments that name the old store are corrected (§2). In
`pistol-search`: ONE comment, on `Position::reset_to`'s replacement of the
state (§1.4), and no code. **Unchanged**: every query's code in `query.rs` and
`cover.rs`, the class-set CODE in `sets.rs` (two of its doc sentences change,
§2), the `WindowMasks` type and `empty_cells`, the
`THREAT_DESYNC` token, every caller's code (`Position`, `dfpn.rs`, `policy.rs`,
`solver.rs`), every config, every fixture. `WindowMasks`'s PUBLIC surface —
two fields, five accessors — is unchanged; its two `pub(crate)` helpers that
were the old store's own (`is_vacant`, `with`) go with it (N9). O-A (laziness at the
`Position` seam) is NOT in this design; the selection record says why.

---

## 1. What the state holds

### 1.1 The line store

A stone lies on three lines, one per axis. Along an axis the cell's POSITION is
the coordinate that grows in the axis direction and its LINE is the one that
does not (`Axis::direction` is `(0,1)`, `(1,0)`, `(1,-1)`):

| axis | line | position |
|---|---|---|
| `ConstQ` | `q` | `r` |
| `ConstR` | `r` | `q` |
| `ConstS` | `q + r` | `q` |

A line is stored in CHUNKS of 64 consecutive positions: chunk index
`pos.div_euclid(64)`, bit `pos.rem_euclid(64)`, one `u64` per side. The store is
`HashMap<u64, Chunk>` under the same seedless SplitMix64 hasher D-254 adopted
(CLAUDE.md rule 4; the hasher and its ground do not move). The key packs
`(axis, line, chunk)` into disjoint biased fields, and the ranges each field
must accept are DERIVED from the lattice AND from the reader: `q + r` over
`i16` spans `[-65536, 65534]`; a chunk index over `i16` positions spans
`[-512, 511]`, **and the eleven-position run around a stone at either end of
the lattice (§1.3 step 1) keys one chunk past that — `-513` below, `512`
above — so the chunk field accepts `[-513, 512]`**; those two chunks never
hold an entry and read as zero. The packing function's doc states the ranges
and this derivation, because a key whose fields can overlap is a store whose
two windows can alias, and §4's M14 is the mutant for that class (N2).

**`apply` only ever sets a bit; the pruning lives in `restore`.** The only path
to a cleared bit is `undo`'s restore of the chunk as it was before the stone
(§1.4 step 3), so the store has no bit-clearing operation at all — a branch
that cleared one would be unreachable. **The pruning rule is D-62's at the
chunk, applied where the clearing happens**: `restore` removes a chunk whose
record before the stone was vacant rather than writing back zeroes, so a fully
unwound state holds no chunk and is EQUAL to a fresh one, not merely equal in
every answer. D-254's ground for hashing at all —
the table is never enumerated on a choice path — is unchanged: the only
enumeration is the snapshot (§1.6).

### 1.2 Reading a window

A window is six consecutive positions of one line from its start. Reading it is
one chunk read, or two when the six positions cross a chunk boundary
(`start.rem_euclid(64) > 58`); the reader shifts the low chunk down by the
offset and, only in the straddling case, ORs the high chunk shifted up by
`64 - offset`. The `offset = 0` case never reaches the second read (the
straddling test is `offset + count > 64`, false at zero), so no shift by 64 is
ever formed. `masks(window)` is this read, masked to six bits per side, and
returns the same `WindowMasks` it always did.

### 1.3 `apply`, one stone

For each axis, in `Axis::ALL` order:

1. Read the eleven positions centred on the stone (five either side) for both
   sides, one or two chunk reads.
2. **Refuse a stone the line already holds**, either side, with
   `THREAT_DESYNC` — the same drift the incumbent's per-window assert names,
   now checked once per axis on the centre bit.
3. Set the stone's bit on this line and push the chunk's key and its record
   BEFORE the stone onto the chunk log.
   Keep each axis's before-run and after-run (the after-run is the
   before-run with the centre bit set on the mover's side).

Then, for every `(window, index)` that `pistol_core::window::windows_through_indexed(at)`
yields — THE enumeration, called, not copied (N1): a window that runs off the
addressable lattice is absent here because it is absent there — select the
window's axis's two runs, take its before-masks and after-masks by shifting
them by `5 - index`, derive each side's class set before and after, apply the
transitions to the sorted sets as today, and push `(window, was, now)` onto
the window log.

Then push the stone's FRAME: its cell, its player, and how many window entries
it pushed. The chunk log always receives exactly three entries per stone; the
window log receives one per window the enumeration yielded.

### 1.4 `undo`, the last stone

1. Pop the frame. **If there is no frame, or its cell or player differ from
   the arguments, panic `THREAT_DESYNC`**, naming the stone asked for and the
   frame found. This is the refusal §3 I4 pins, and it is the contract change
   this design makes: `undo` takes back THE LAST STONE APPLIED, not any stone.
   Every caller at the governing revision is LIFO (the matrix §1.1 lists them),
   and the doc's `# Panics` section says so.
2. Pop the frame's window entries and, for each, apply the REVERSE transition
   (`now → was`) to each side whose class set changed. The sets are sorted by
   construction and insertion/removal is by binary search, so the order in
   which entries are reversed does not affect the result.
3. Pop three chunk entries and restore each: a record that was vacant before
   is removed, any other is written back. Restoring, rather than clearing the
   bit, is what makes the undone chunk byte-equal to the one that existed
   before the stone, including the pruning.

No class is recomputed and no window is enumerated on this path. That is the
whole of what the log buys, and the matrix measured it.

**Two caches, two contracts, one `Position`.** D-61 makes the eval's unwind
order-free, and `Position::reset_to` unwinds the eval stone by stone in board
order (`crates/pistol-search/src/position.rs:48-59`) before rebuilding; the
threat state under this design is LIFO-or-panic, so that unwind cannot be
applied to it. `reset_to` already REPLACES the state with `ThreatState::new()`
(`:60-62`) and rebuilds it by `apply` over the board — under this design that
replacement is the only correct spelling, not an O(1) convenience, and the
comment at that line says so (the matrix's round-1 red team, m8).

### 1.5 Equality

`PartialEq` and `Eq` are written by hand over the store and the sets and exclude
the three stacks `apply` pushes onto — the window log, the chunk log and the
frames (N5). The log is bookkeeping for `undo`, not state: two states holding
the same windows in the same classes are the same state whatever path built
them (§3 I7 pins this). `Clone`, `Debug` and `Default` stay derived; a clone
carries its logs and undoes independently.

### 1.6 The snapshot, `window_count`, `is_empty`

`table_snapshot()` enumerates chunk bits, maps each set bit back to its cell,
takes the windows through that cell ON THAT AXIS, and reads each window's masks
through §1.2 — a `BTreeMap<Window, WindowMasks>` holding exactly the addressable
windows with at least one stone, which is what the incumbent's snapshot held.
It is for oracles and diagnostics and allocates; nothing on a choice path calls
it (unchanged). `window_count()` now enumerates the same way and its doc says it
costs the table; it has no caller outside its definition at the governing
revision. `is_empty()` is whether the store holds a chunk, which by the pruning
rule is whether any window holds a stone.

---

## 2. Files

The store leaves `table.rs` for a private module of its own, `line.rs`: the
chunk type, the line projection and its inverse, the key packing, the hasher,
and the store (`LineStore`). `table.rs` keeps `WindowMasks`, `FULL_MASK` and
`empty_cells`, which are the window-mask vocabulary the queries speak and not
the store. This is the split the matrix §4.2 owes (rule 9) and it is the crate
root doc's own sentence — *"a different store replaces exactly that file and
nothing else"*, D-261's ground with D-254's flip clause behind it — made true of
the file that holds the store.

The crate root doc changes in five passages: the sentence calling the state
*"a per-window record"*; the "STORE is not exported" paragraph, which names
*"the table type"* and must name the line store; the Determinism paragraph's
*"The window TABLE is hashed"* (it is the line store that is hashed, on the
same ground) and its *"only `table_snapshot` enumerates it"*, which gains
`window_count` as a second enumerator through the same function, both off any
choice path; the third privacy doctest, which reaches for `table::unpack` and
must reach for `line::unpack` instead — a `compile_fail` on a path that no
longer exists passes on the wrong error and pins nothing (the doc's own
paragraph on vacuous `compile_fail` says so); and the sentence after it that
names the one door the doctests do not cover, *"`pub use table::unpack` here at
the root"*, which names the same moved path (N4). Three in-crate doc comments name
the old store and are corrected: `cover.rs`'s `min_hitting_set_exceeds` doc
names *"the packed key's `unpack`"* as a door that could produce a window —
under this design `unpack` produces a chunk key and no window, and the doors
are `empty_cells` and the store's own snapshot enumeration, both
crate-private; `state.rs`'s `table_snapshot` doc points at the store's snapshot
by its new name; and `sets.rs`'s two sentences about *"the table"* underneath
the sorted sets say *"the store"*, both being true of either (REVIEW-impl F5).
No code in `sets.rs` changes.

Docs/ADR: one D-line recording that D-254's flip clause fired on the P1
re-profile and what replaced the store, with the LIFO contract and the
hand-written equality named as the two things a caller can observe.

---

## 3. Invariants, each with the test that pins it

Behaviour-named tests in `crates/pistol-solver/tests/threat_oracle_tests.rs`
unless stated. "Reference" is the test crate's existing independent
`Reference::from_board`, which reads the board and never the state.

| # | invariant | pinned by |
|---|---|---|
| I1 | For every stone sequence the snapshot equals the reference's table and every set equals the reference's set | EXISTING `compare` over the random playouts (the oracle) — unchanged, and it is the whole-representation oracle |
| I2 | `apply` then `undo` restores whole-state equality at every ply, and a fully unwound state equals `ThreatState::new()` | EXISTING `threat_apply_undo_roundtrips` — unchanged |
| I3 | An unwound state holds no chunk | EXISTING: `threat_apply_undo_roundtrips` already asserts `is_empty()` after the unwind (`threat_oracle_tests.rs:230`), beside the equality |
| I4 | `undo` of a stone that is not the last applied panics `THREAT_DESYNC` | EXISTING `taking_back_a_stone_that_was_never_applied_is_a_desync` and `..._the_wrong_player_...`; NEW `taking_back_a_stone_that_is_not_the_last_applied_is_a_desync` (two stones applied, the first undone first) |
| I5 | Windows crossing a chunk boundary read correctly, in the store AND in every class set | NEW `windows_straddling_a_chunk_boundary_read_as_the_reference_does`: six stones across each of the three boundaries a game reaches first — −1 / 0 (the one every game crosses from the origin, and the only one the random playouts reach: 63 / 64 never, per the matrix's round-1 red team m5, and −64 / −65 never, per that round's experiment 3), −64 / −65 and 63 / 64 — on each axis, in TWO colourings: one side only, so the windows climb through every live class to a completed six and the class sets are read populated (round 3's R1: alternating sides leave every window dead and the set clause empty-against-empty), then alternating; after every stone and every undo, every window through the stones reads as the reference, the snapshot equals the reference's table, and all five class sets per side equal the reference's |
| I6 | The lattice edge is handled as `windows_through_indexed` handles it, in the store AND in every class set | NEW `threat_windows_stop_at_the_edge_of_the_addressable_lattice`: stones at the four `i16` corners and along each edge; the snapshot equals the reference's table and all five class sets per side equal the reference's, no panic — the threat counterpart of the eval's test of the same name, reading the sets because round 2 (N1) showed a store maintained at the edge while a set is not passes every other check |
| I7 | Equality ignores the path that built the state | EXISTING: `window_map_ordering_is_unobservable` applies the same stones forwards and backwards and asserts whole-state equality — under a derived `PartialEq` the three stacks differ and it FAILS, so it is M11's falsifier as it stands |
| I8 | The snapshot's WINDOW SET is exactly the set `windows_through_indexed` enumerates over the board's stones, and `window_count()` is that set's size | NEW `every_snapshot_window_reads_back_through_masks`: over random playouts, the snapshot's keys equal the enumerated set and `window_count()` equals its size; the test also reads every listed window back through `masks`, which shares the snapshot's reader and so pins nothing on its own (the matrix's §6 attack 2 is closed by I1, which compares the snapshot with the REFERENCE) |
| I9 | A stone the line already holds is refused | EXISTING `applying_a_stone_twice_is_a_desync` (P1 at the origin, then P2 there) |

---

## 4. The mutation set — defect classes, each dying at a named test (D-553)

| mutant | class | dies at |
|---|---|---|
| M1 shift off by one in the window read (`REACH - back` → `REACH - back + 1`) | misread masks | I1 |
| M2 high-chunk OR dropped in the run reader | straddling | I5 (and I1 on any playout crossing a boundary) |
| M3 a chunk not restored on `undo` | undo | I2 |
| M4 a transition not reversed on `undo` (skip one side) | undo | I2 |
| M5 empty chunk kept (pruning removed) | pruning | I2's `new()` equality, I3 |
| M6 frame check removed (`undo` pops without comparing) | LIFO refusal | I4 |
| M7 centre-bit refusal removed in `apply` | desync guard | I9 |
| M8 one fewer window entry pushed per stone (`touched` under-counted) | frame integrity | I2 |
| M9 chunk log records the record AFTER the stone | undo | I2 |
| M10 the snapshot's cell inverse off by one (`pos + 1`) | snapshot enumeration | I8 (the listed window set is not the enumerated one). The snapshot's axis FILTER is not a mutant: every window through an occupied cell holds that stone, so removing the filter lists the same set three times over — an equivalent mutant, found by the session and by round 1 (D1), and not registered |
| M11 `PartialEq` includes the log (derived) | equality | I7 (`window_map_ordering_is_unobservable`) |
| M12 CALL-SITE: `Position::undo` no longer calls the state's `undo` | call site | `crates/pistol-search/tests/staged_tests.rs`'s `a_full_search_under_staged_completes_from_the_opening_without_crashing`: the state still holds the undone stone, and the next `apply` on that cell is refused with `THREAT_DESYNC` — the refusal kills it, not a wrong answer, so a mutant that also removed the refusal would be caught by the identity leg (§5) |
| M13 the ConstS projection reads the position as `r` instead of `q` | projection | I1 (ConstS windows with a stone behind them on the line are misread on the playouts) |
| M14 the key's line field narrowed to eight bits | key-field overlap | I6, by `THREAT_DESYNC`: line `-32768` aliases line `0` on the lattice-edge grid, so the aliased line already holds a bit at the fourth stone's position and `apply` refuses it (N7 — the refusal kills it, not a misread window) |
| M15 the class-set transitions are skipped for the last window of each axis (`index == 5`) | a window the store holds and a set does not | I1, and I5's one-side colouring and I6, which read the sets — the class round 2's X9 found unpinned |

Mutants run in a separate worktree, never the live tree, green before
REVIEW-impl (CLAUDE.md Process), by a driver sha-anchored beside its receipt
(`artifacts/p1_mutation_driver.py`; `artifacts/` is never committed, rule 8). The receipt lists each mutant, the test it
died at, and that the unmutated tree passes every registered test.

---

## 5. What proves the landing correct

The state is a cache of the board, so the package's correctness claim is
BYTE-IDENTITY of search output (D-495's two-track law: identity takes no SPRT,
one mismatch flips it to the SPRT track). The package's bench pre-registration
(`p1_bench_prereg.md` §2) is the one place the identity leg is specified; this
design names only what it must contain: the two-binary diff at THREE seats —
the two WP-1.9 seats at the determinism gate's budgets and the committed
solver-armed seat at its own — with the identity digests recorded,
`bench_delta.sh`'s per-position node identity, and the CI gates that read this
state — determinism, search oracle, staged soundness, solver oracle, solver
determinism — green at the landed SHA with their own log lines cited.

---

## 6. What this design does not do, stated so it is not read in

- No laziness at the `Position` seam: the state is told about every stone the
  moment it is placed, as today.
- No change to any query, to any refusal message's token, to any config or
  fixture, and **no CODE change in `sets.rs`** — two of its doc sentences say
  "store" where they said "table", which Scope and §2 license and which
  REVIEW-impl round 2 found this bullet still forbidding (its F15).
- No change to how many windows a stone maintains: eighteen, less those off the
  lattice, exactly as enumerated by `pistol-core`.
- No claim about the solver's own throughput. The solver IS armed in three
  committed configs (`bench_wp18c_solver_on.toml`, `gate_staged_solver_v0.toml`,
  `play_staged_solver_v0.toml`), one of them a `tools/determinism.sh` seat; the
  matrix (§6, attack 1) records an exploratory measurement there and the
  package banks none of it. The oracle gates and the identity leg's third seat
  (§5) cover its correctness.
