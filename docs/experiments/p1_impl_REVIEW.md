# P1 — REVIEW-impl: the threat state's per-axis line bitboards with a logged undo

**Implementation revision reviewed:** `3ef67062` (`3ef6706`), which IS `p1/impl`'s
tip at the time of this review (`git rev-parse p1/impl` → `3ef6706…`).
**Design revision it is checked against:** `db4ecd9e86bff8c07507183dc0570ed5e787985f`
(a `git stash create` commit on `dev`) — `p1_design.md` revision 3 as amended after
its PASS. The four governing documents at that revision hash the same as the
working tree's untracked copies:

```
$ for f in p1_design.md p1_design_REVIEW.md p1_design_REVIEW_rev2.md p1_design_REVIEW_rev3.md; do
    a=$(git show db4ecd9e…:docs/experiments/$f | sha256sum | cut -c1-16)
    b=$(sha256sum docs/experiments/$f | cut -c1-16); echo "$f rev=$a wt=$b"; done
p1_design.md            rev=5126a44d382c51b1 wt=5126a44d382c51b1 SAME
p1_design_REVIEW.md     rev=b35e4c29ce24040d wt=b35e4c29ce24040d SAME
p1_design_REVIEW_rev2.md rev=785b43a52fff9296 wt=785b43a52fff9296 SAME
p1_design_REVIEW_rev3.md rev=cab1411c9d9f0256 wt=cab1411c9d9f0256 SAME
```

**HEAD:** `ffc5c10` on `dev` (= the branch point; `dev` has not moved).
**Date:** 2026-09-04.
**Where everything ran:** `/home/tom/pistol-wt/review-p1-impl` (detached at
`3ef6706`, the full `cargo test --workspace --locked` with NO exported
`CARGO_TARGET_DIR`) and `/home/tom/pistol-wt/review-p1-impl-mut` (detached at
`3ef6706`, `CARGO_TARGET_DIR` = its own `target`, for the mutation re-run, the
reviewer's own mutants, the doctest experiment, fmt/clippy and the five gates).
Never the live tree, never `/tmp`, never the implementer's worktrees. Both
worktrees are left in place, clean, with no branch created
(`git status --porcelain` empty in both at the end).

**Reviewer's note on the brief.** REVIEW-impl checks the implementation against
the design — correctness and requirement gaps. `rustfmt` and `clippy` are
mechanical law and are reported as run, not discussed.

---

## 1. The "IMPL verifies" rows (D-590), discharged BY EXECUTION

Rows consolidated across the three design reviews (`R1` = `p1_design_REVIEW.md`
§5, `R2` = `…_rev2.md` §5, `R3` = `…_rev3.md` §5). Every row below was RUN, not
read.

| row (rounds) | what I ran | holds? |
|---|---|---|
| **I4's new test name** (R1, R2, R3 "still open") | `/usr/bin/grep -n "taking_back" docs/experiments/p1_design.md` → §3 I4 names `taking_back_a_stone_that_is_not_the_last_applied_is_a_desync`; `git grep -n "fn taking_back" p1/impl -- crates` → `threat_oracle_tests.rs:333, :341, :349`, the last being that name | **YES — now closed.** The design at the governing revision (rev 3) carries the LONG name; the divergence R3 left open was resolved on the design side by the amendment. The body applies `ORIGIN` then `(1,0)` and undoes `ORIGIN` (`:349-355`), i.e. the FIRST, as I4 requires |
| **I4's body drives the LIFO refusal** (R1, R2) | read `:349-355`; mutation M6 (frame check removed) run against it | YES — M6 DEAD at that test |
| **I5: three boundaries × three axes, every window compared** (R1, R2, R3) | read `:418-468`; `assert_matches_board` (`:371-406`) compares five class sets per side, every `windows_through_indexed` window through every stone, and the snapshot — after every stone AND after every undo; boundaries `-1/0` (`-3..=2`), `-64/-65` (`-67..=-62`), `63/64` (`61..=66`), each on `Axis::ALL` | YES |
| **I5: "M2 dies at I5 alone"** (R1, R2, R3) | M2 (high-chunk OR dropped) applied and the WHOLE oracle file run, not just the registered test | **NO — see F3.** M2 kills **8 of 11** tests: I1, I2, I5, I6, I7, I8, the legality test and `applying_a_stone_twice`. "Alone" is false; the design's own §4 M2 row already says *"(and I1 on any playout crossing a boundary)"*, so the DESIGN is right and only the review row's word is wrong. Not a defect |
| **I5: R1 — the set clause is empty-against-empty** (R3, new) | M15 (transitions skipped for `index == 5`) applied and run FILTERED to I5 | **YES, and now CLOSED by `3ef6706`.** M15 is DEAD at I5: `assertion left == right failed: ConstQ -1/0 one side after 0,-2 p1: live Two`. The one-side colouring the last commit added makes I5's class-set clause read POPULATED sets, which is exactly what R3's R1 said it did not |
| **I5's comment** (R2, R3 — *"still false"*) | arithmetic against `LineStore::run`'s own straddle condition | **NO — see F4.** Still false at `3ef6706` |
| **I6: the grid reaches the corners and the `-513`/`512` keys, no panic, unwind to `new()`** (R1, R2, R3) | four probes asserting each stated bound inside `pack`, each run against the whole oracle file (§4 below) | **YES, by execution.** `CHUNK_RANGE_PROBE -513` and `CHUNK_RANGE_PROBE_HIGH 512` both FIRE, and `LINE_PROBE_LOW -65536` / `LINE_PROBE_HIGH 65534` both FIRE — **all four at `threat_windows_stop_at_the_edge_of_the_addressable_lattice` and at no other test**. The complementary probe asserting the design's READER ranges (`chunk ∈ [-513,512]`, `line ∈ [-65536,65534]`) is SILENT on the whole suite: the code forms exactly the range §1.1 states, no more |
| **I6: the class sets are compared at the edge and X9-shaped mutants die there** (R2 after N1, R3 "discharged") | read `:471-516` — it calls `assert_matches_board`, which reads all five class sets per side; M15 run | YES — M15 DEAD at I6 (`:390`, the `live {count:?}` assert) as well as at I1 |
| **I8 carries the window-set and `window_count` checks** (R1, R2/R3 "discharged") | read `:518-548`: `:542` `assert_eq!(listed, expected, "the snapshot's window set")`, `:544` `assert_eq!(threats.window_count(), expected.len())`; M10 run | YES — M10 DEAD there |
| **M8: which under-count shape the harness runs; the receipt's wording must match** (R1, R2, R3) | read `artifacts/p1_mutation_driver.py`: the applied mutant is `if index != 0 { touched += 1 }` — the COUNT shape; the receipt's label reads *"one fewer window entry counted per stone"* | **YES** — the receipt's wording matches the mutant applied. The DESIGN's §4 label still names two shapes (*"one fewer window entry pushed per stone (`touched` under-counted)"*) — carried, minor, F-list item |
| **M10 is the amended design's mutant and dies at the window-set half of I8** (R1, R2, R3) | driver's M10 = `pos: index * CHUNK_LEN + bit + 1` (the snapshot's cell inverse off by one) — the design's D1 remedy exactly; run | YES — DEAD at `every_snapshot_window_reads_back_through_masks` |
| **M12: the named test drives the call site and it dies by `THREAT_DESYNC`** (R1, R2, R3) | driver's M12 removes the `threats.undo` CALL in `Position::undo`; run | YES — DEAD at `pistol-search/staged_tests::a_full_search_under_staged_completes_from_the_opening_without_crashing`, and (from RX6, the mirror mutant) the death is at `state.rs:164`, `undo`'s frame refusal |
| **M14 dies at I6 BY THE REFUSAL (N7), and at I1/I5/I8 by the inverse's panic; the receipt must say which is the class's kill** (R2, R3) | M14 applied and the WHOLE oracle file run, with the panic messages captured | **Fact YES, receipt NO — see F2.** At I6 the death is `THREAT_DESYNC: p2 stone on -32768,0 lands on a cell of its ConstR line that already holds one` (`state.rs:108` — the refusal, per N7). At I1, I5 and I8 the death is `line.rs:66:18`, `LinePos::cell`'s `expect` — the inverse's panic, exactly as R2/R3 predicted. The receipt records only *"DEAD at …"* and names no reason |
| **M15 dies at I1 and the receipt does not credit I5** (R3) | the driver registers M15 at `threat_incremental_matches_reference_on_random_playouts`; receipt says so | YES — and (see the I5 row) at `3ef6706` M15 now ALSO dies at I5, which is the improvement the last commit was for. The receipt credits I1, which is correct |
| **`reset_to`'s comment says the replacement is the only correct spelling** (R1, R2, R3) | `git diff ffc5c10 p1/impl -- crates/pistol-search/src/position.rs` | YES — *"The threat state is REPLACED rather than unwound, and that is the only correct spelling: its `undo` takes back the last stone applied and refuses any other…"*. The same diff, filtered to non-doc lines, is EMPTY: `pistol-search` gets ONE comment and no code, as Scope requires |
| **the docs of m5 plus N4 (`lib.rs`'s passages, `cover.rs`, `state.rs`) are true of the tree** (R1, R2, R3 "discharged") | read all five root-doc passages and both in-crate docs at `3ef6706`; `git grep -nE 'WindowTable\|table::unpack\|table::pack' -- crates` → no hit | YES — all five §2 passages changed, `lib.rs:89` retargeted to `pub use line::unpack`, `cover.rs:108-109` names *"the line store's snapshot enumeration and `empty_cells`"*, `state.rs`'s `table_snapshot` doc points at `LineStore::snapshot` |
| **`pack`'s doc states the derivation and its widths match the code** (R2, R3) | read `line.rs:70-84` against `line << 16`, `& 0xFF_FFFF` (24 bits), `& 0xFFFF` (16 bits) | YES — 24 bits of line above 16 of chunk, axis at bit 40; the doc names the stored range, the reader's reach to `-513`/`512`, and that the biases keep every field non-negative |
| **the ADR line exists at the landing, naming the flip clause, the LIFO contract and the hand-written equality** (R2, R3) | `git diff --name-only ffc5c10 p1/impl -- docs \| wc -l` → **0**; `/usr/bin/grep -oE '^D-[0-9]+' docs/decisions.md \| tail -1` → **D-602** | **NO — F1, BLOCKING.** No D-line exists at `3ef6706`; `docs/decisions.md` is untouched by all five commits |
| **`policy.rs:1` release warning, outside scope** (R3, observation) | it appears in every gate log; `git show ffc5c10:crates/pistol-solver/src/policy.rs \| head -1` shows the same import on `dev`, and `policy.rs` is not in the diff | YES — pre-existing, not this package's (F7) |

---

## 2. The mutation re-run

Read `artifacts/p1_mutation_driver.py` (sha256 `ecb3925bf434e2c4…`), then re-ran it
myself in my own worktree:

```
$ python3 /home/tom/Projects/HeXO-AlphaBeta/artifacts/p1_mutation_driver.py \
      /home/tom/pistol-wt/review-p1-impl-mut
REV 3ef6706
… 15 UNMUTATED rows, all "-> pass" …
MUTANT M1 …: DEAD at pistol-solver/threat_oracle_tests::threat_incremental_matches_reference_on_random_playouts
MUTANT M2 …: DEAD at …::windows_straddling_a_chunk_boundary_read_as_the_reference_does
MUTANT M3/M4/M5/M8/M9 …: DEAD at …::threat_apply_undo_roundtrips
MUTANT M6 …: DEAD at …::taking_back_a_stone_that_is_not_the_last_applied_is_a_desync
MUTANT M7 …: DEAD at …::applying_a_stone_twice_is_a_desync
MUTANT M10 …: DEAD at …::every_snapshot_window_reads_back_through_masks
MUTANT M11 …: DEAD at …::window_map_ordering_is_unobservable
MUTANT M13/M15 …: DEAD at …::threat_incremental_matches_reference_on_random_playouts
MUTANT M14 …: DEAD at …::threat_windows_stop_at_the_edge_of_the_addressable_lattice
MUTANT M12 CALL-SITE …: DEAD at pistol-search/staged_tests::a_full_search_under_staged_completes_from_the_opening_without_crashing
SUMMARY 15/15 dead, 0 alive
TREE CLEAN
```

Compared with the implementer's receipt:

```
$ diff artifacts/p1_mutation_3ef6706_v1.txt <(grep -v '^EXIT=' rerun.log)
$ echo $?   # → no output, exit 0
IDENTICAL to the receipt, byte for byte
```

**The receipt reproduces exactly.** Its one gap is that it records the test each
mutant died at and not the death REASON, which R2/R3 asked for on M14 (F2).
The driver itself is sound on the two points that matter: it verifies the
UNMUTATED tree passes each registered test first, it aborts if an anchor is not
unique, and it restores each file with `git checkout` after every mutant
(`TREE CLEAN` at the end, confirmed independently by `git status --porcelain`).

---

## 3. Check 1 — §1 mechanism IS what the code does

**§1.1 projection table against `LinePos::of`** (`line.rs:44-55`). `ConstQ →
{line: q, pos: r}`, `ConstR → {line: r, pos: q}`, `ConstS → {line: q+r, pos: q}`
— identical to the design's table, and consistent with `Axis::direction`
`(0,1)`, `(1,0)`, `(1,-1)`. The inverse `LinePos::cell` (`:58-68`) is the exact
inverse, including `ConstS → (pos, line - pos)`. M13 (ConstS position read as
`r`) is DEAD at I1.

**§1.1 chunk / key / bias ranges, by arithmetic.** `pack` (`line.rs:76-84`)
biases line by `1<<17` and chunk by `1<<10`, then `(axis << 40) | (line << 16) |
chunk`. Line spans `[-65536, 65534]` → biased `[65536, 196606]`, 18 bits, inside
the 24-bit field at bits 16..40. Chunk spans `[-513, 512]` → biased `[511,
1536]`, 11 bits, inside the 16-bit field at bits 0..16. Axis at bit 40. The
fields cannot overlap and `unpack`'s masks (`0xFF_FFFF`, `0xFFFF`) are the same
widths. The reader's reach is real: `pos ∈ [-32768, 32767]`, the eleven-run
starts at `pos - 5`, and `(-32773).div_euclid(64) = -513` while
`32762.rem_euclid(64) = 58` with `58 + 11 = 69 > 64` forces the read of
`index + 1 = 512`.

**§1.1 ranges, by execution** (this is the row R1/R2/R3 left to IMPL). Four
probes inserted at the head of `pack`, each run against the whole oracle file in
`review-p1-impl-mut` and restored after (`TREE CLEAN` each time):

```
PROBE-A  assert!((-512..=511).contains(&chunk))   -> FIRED   CHUNK_RANGE_PROBE -513
         only test threat_windows_stop_at_the_edge_of_the_addressable_lattice FAILED (10 passed; 1 failed)
PROBE-C  assert!(chunk <= 511)                    -> FIRED   CHUNK_RANGE_PROBE_HIGH 512
         same single test (10 passed; 1 failed)
PROBE-D  assert!(line > -65536)                   -> FIRED   LINE_PROBE_LOW -65536
PROBE-E  assert!(line < 65534)                    -> FIRED   LINE_PROBE_HIGH 65534
PROBE-B  assert!((-513..=512).contains(&chunk))
         && assert!((-65536..=65534).contains(&line))  -> SILENT  (11 passed; 0 failed)
```

So a stone at each `i16` extreme DOES form `-513`, `512`, `-65536` and `65534`;
the test that forms them is I6; and nothing anywhere in the suite exceeds the
range §1.1 states. The design's `[-513, 512]` is exactly right, and it is
exercised.

**§1.1 "apply only ever sets a bit; pruning lives in restore".** `place`
(`line.rs:199-208`) does `*after.side_mut(side) |= bit` and nothing else;
`restore` (`:214-220`) removes-or-writes-back. A search for any bit-clearing
operation in the whole solver source finds ONE hit and it is not a store
mutation:

```
$ /usr/bin/grep -rn -E '&=[^=]|!\(1|\^=' crates/pistol-solver/src/ | LC_ALL=C sort
crates/pistol-solver/src/line.rs:247:                bits &= bits - 1;      # the snapshot's set-bit walk
```

The old `WindowMasks::with`'s `*slot &= !bit` branch is deleted with the rest of
the old store (N9). Confirmed: there is no bit-clearing operation in the store.

**§1.2 the window reader.** `run` (`line.rs:169-190`) reads `low`, shifts down by
`offset`, and reads `high` only when `offset + count > 64`. At `count = 6` that is
`offset > 58`, the design's condition; at `offset == 0` it is false for every
`count ≤ 64`, so `64 - offset` is never a shift by 64. `masks` (`:195-203`) is
this read at `WINDOW_LEN`, masked by `keep` to six bits per side, returning the
unchanged `WindowMasks`.

**§1.3 `apply`, steps in order** (`state.rs:89-146`). Per axis in `Axis::ALL`:
(1) `run(axis, here.line, here.pos - REACH, 2*REACH + 1)` — the eleven positions;
(2) `assert!(((p1|p2) >> REACH) & 1 == 0, "{THREAT_DESYNC}: …")` — the centre-bit
refusal, both sides, BEFORE any mutation; (3) `place`, then
`self.chunks.push((key, before))`; the before-run and after-run kept in `runs`,
the after-run being the before-run with bit `REACH` set on the mover's side.
Then `for (window, index) in windows_through_indexed(at)` — **called, not
copied**: `git grep -n windows_through_indexed -- crates` shows the enumeration
defined once in `pistol-core` and reached from `pistol-solver` at exactly two
sites, `state.rs:122` (`apply`) and `line.rs:254` (the snapshot). Masks are taken
by `shift = REACH - index` = `5 - index`, per side, and each side's transition
applied only where the class set changed; `Touched { window, was, now }` pushed.
Then the frame `Applied { at, player, touched }`. The chunk log receives exactly
three entries per stone (one per `Axis::ALL`); the window log one per enumerated
window.

*Considered and rejected as a finding:* the refusal is inside the per-axis loop,
so in principle a later axis could refuse after an earlier axis has already
placed. It is unreachable — the centre bit is the SAME CELL on all three lines,
so if it is set anywhere it is set on `ConstQ`, which is checked first, before any
mutation. Not reproducible; recorded so it is not rediscovered.

**§1.4 `undo`** (`state.rs:158-186`). (1) pops the frame and panics
`THREAT_DESYNC` naming both the stone asked for and the frame found unless cell
AND player match; (2) pops `frame.touched` entries and applies `transition(now →
was)` per side that changed; (3) pops three chunk entries (`for _ in Axis::ALL`)
and `restore`s each. No class is recomputed and no window enumerated. The three
chunk keys differ by construction (the axis is in the key), so pop order among
them cannot matter.

**§1.5 equality.** `#[derive(Debug, Clone, Default)]` on `ThreatState`; hand-written
`impl PartialEq` over `self.table` and `self.sets` only (`state.rs:74-80`) and
`impl Eq`. The three stacks are excluded. `git diff … | grep -E '^[-+].*(pub |fn )'`
over `state.rs` shows the only added items are the private `eq` and `axis_slot`
and the only removed one the private `touch` — **no public item was added,
removed or renamed**, so D-261's enumeration of the public surface still holds.

**§1.6 snapshot / `window_count` / `is_empty`.** `LineStore::snapshot`
(`line.rs:238-262`) walks set bits, maps each back through `LinePos::cell`, takes
`windows_through_indexed(cell)` **filtered to that axis**, collects into a
`BTreeSet` (so the `HashMap` walk order cannot be observed) and reads each window
through `masks`. `window_count()` is `self.table.snapshot().len()` with a doc
saying it enumerates the store and is diagnostics only; `is_empty()` is
`self.chunks.is_empty()`.

---

## 4. Check 2 — §2 files

| §2 requirement | verified |
|---|---|
| the store leaves `table.rs` for a private `line.rs` | `line.rs` is new (265 lines) and holds `Chunk`, `LinePos`, `pack`/`unpack`, `SplitMix64`, `LineStore`; `lib.rs:124` declares `mod line;` (private) |
| `table.rs` reduced to the mask vocabulary | 54 lines: `FULL_MASK`, `WindowMasks` + its five accessors and two public fields, `empty_cells`. `is_vacant` and `with` are gone with the old store (N9) |
| root doc, passage 1 — *"a per-window record"* | → *"an incremental record of what each side holds along every line — per-axis line bitboards, read window by window; apply and undo one stone at a time, the undo in reverse order of the apply"* |
| passage 2 — the "STORE is not exported" paragraph naming *"the table type"* | → *"the packed key, its hasher, the line store and the class sets"*, and *"the whole ground for the store being its own file"* |
| passage 3 — Determinism's *"The window TABLE is hashed"* + *"only `table_snapshot` enumerates it"* | → *"The line STORE is hashed by chunk … only `table_snapshot` and, through the same enumeration, `window_count` walk it, both diagnostics, and the snapshot sorts"* |
| passage 4 — the third privacy doctest | `lib.rs:78-80` now reads `pistol_solver::line::unpack(0)` |
| passage 5 — *"`pub use table::unpack` here at the root"* | → *"`pub use line::unpack` here at the root"* (`lib.rs:89`) |
| `cover.rs`'s `min_hitting_set_exceeds` doc | `:108-109` → *"the line store's snapshot enumeration and `empty_cells` itself, are both crate-private"* |
| `state.rs`'s `table_snapshot` doc | → *"see `LineStore::snapshot`, whose doc says why the store underneath may be hashed at all"* |
| `Position::reset_to`'s comment in `pistol-search` | present, and the non-doc part of that file's diff is EMPTY |
| the ADR D-line | **ABSENT — F1** |

**The third doctest is NOT vacuous — verified both ways** (in
`review-p1-impl-mut`, restored after; `git status --porcelain -- crates` empty):

```
1. as shipped:                       lib.rs - (line 78) - compile fail ... ok   (2 passed; 0 failed)
2. `mod line;` → `pub mod line;`:    lib.rs - (line 78) - compile fail ... ok   (2 passed; 0 failed)   ← does NOT flip
3. …and `pub(crate) fn unpack` → `pub fn unpack`:
                                     lib.rs - (line 78) - compile fail ... FAILED  (1 passed; 1 failed)
```

Step 3 is the decisive one: once the door is genuinely open the example COMPILES,
so the path `pistol_solver::line::unpack` is real, correctly spelled, and the
`compile_fail` is failing on privacy rather than on a typo or a stale path — the
exact defect §2 says it was retargeted to avoid. Step 2 shows the brief's
single-step check is not sufficient on its own: the item is `pub(crate)`, so the
module is only one of two locks, and both must be opened. This matches D-261's
own recorded recipe, whose reviewer re-published *"`pack`, `unpack`, `empty_cells`
and `pub mod table`"* together. Recorded as observation F8, not a finding.

---

## 5. Check 3 — §3 invariants, and the full suite

Every named test exists under the design's name and pins what its row says (bodies
read):

| # | test | line | what it actually asserts |
|---|---|---|---|
| I1 | `threat_incremental_matches_reference_on_random_playouts` | `:49` | per ply, `compare` (`:98`) asserts the snapshot == the REFERENCE's table (`:112`) and all ten class sets == the reference's, plus the cell answers and the census floors |
| I2 | `threat_apply_undo_roundtrips` | `:203` | whole-state equality against a clone recorded at every ply, then `== ThreatState::new()` |
| I3 | (same test) | `:230` | `assert!(threats.is_empty(), "no entry survives the unwind")` beside the equality |
| I4 | `taking_back_a_stone_that_was_never_applied…` / `…the_wrong_player…` / `taking_back_a_stone_that_is_not_the_last_applied…` | `:333`, `:341`, `:349` | the third applies ORIGIN then `(1,0)` and undoes ORIGIN; all three `#[should_panic(expected = "THREAT_DESYNC")]` |
| I5 | `windows_straddling_a_chunk_boundary_read_as_the_reference_does` | `:418` | 3 axes × 3 boundaries × 2 colourings; `assert_matches_board` after every stone and every undo; `== ThreatState::new()` and `is_empty()` at `:459-464` |
| I6 | `threat_windows_stop_at_the_edge_of_the_addressable_lattice` | `:471` | 7×7 grid over `{MIN, MIN+1, MIN+7, 0, MAX-7, MAX-1, MAX}²`; `assert_matches_board`; `window_count() > 0`; unwind to `new()` |
| I7 | `window_map_ordering_is_unobservable` | `:237` | same stones forwards and backwards, whole-state equality |
| I8 | `every_snapshot_window_reads_back_through_masks` | `:518` | `:542` the snapshot's key set == the enumerated set; `:544` `window_count()` == its size |
| I9 | `applying_a_stone_twice_is_a_desync` | `:322` | P1 at ORIGIN then P2 at ORIGIN — the OPPOSITE-side case, which is what makes a refusal narrowed to the mover's own side die (RX5) |

`assert_matches_board` (`:371`) is the shared clause I5 and I6 both use, and it
reads all FIVE class sets per side (`hot`, `win_in_one_ply`, `completed`, `live`
at `Two` and at `Three`) against `Reference::from_board`, which reads the board
and never the state.

**The full suite**, in `/home/tom/pistol-wt/review-p1-impl`, with NO exported
`CARGO_TARGET_DIR` (`env | grep -i cargo_target` empty before the run):

```
$ cargo test --workspace --locked
… EXIT=0
$ grep -E '^test result:' full_test.log | awk '{p+=$4;f+=$6;i+=$8} END {print NR,p,f,i}'
suites=173  passed=1106  failed=0  ignored=21
$ grep -c FAILED full_test.log
0
$ grep -c '^test result: ok' full_test.log
173
```

173 result lines, every one `ok`; 1106 passed, 0 failed, 21 ignored.

---

## 6. Check 4 — §4 mutants, plus mutants of my own

The re-run is §2 above: 15/15 DEAD, byte-identical to the receipt.

I then designed **six further mutants and one call-site mutant of my own**, for
defect classes I judged the registered set does not name, and ran each against
the WHOLE oracle test file (not a single registered test), restoring after each
(`TREE CLEAN` between every one):

| mine | class the registered set does not name | result |
|---|---|---|
| RX1 `run`'s `keep` mask removed (`u64::MAX`) | the window read is not masked to its own length, so bits 6–7 of the line leak into the `u8` mask (M1 is a shift, M2 is the high chunk; neither is the mask) | **DEAD** — I1 (`:112`), I5, I6 (8 passed; 3 failed) |
| RX2 `run` indexes by truncating division (`from / 64`) | negative-position chunk indexing — the whole reason the `-1/0` and `-64/-65` boundaries are in I5, and no registered mutant touches it | **DEAD** — 6 tests, incl. `sets.rs:194` `THREAT_DESYNC: … is not in LiveTwo to remove` |
| RX3b `place` keys its chunk by truncating division | the same class on the WRITE side | **DEAD** — 7 of 11 tests |
| RX7 pruning predicate narrowed to `p1 == 0` | a chunk holding only P2 stones is pruned on restore (M5 removes pruning entirely; nothing narrows it) | **DEAD** — I2, I5 |
| RX8 the window log records `was`/`now` swapped | `undo` replays the transition FORWARD | **DEAD** — I2, I5, I6 |
| RX9 the snapshot enumerates `chunk.p1` only | the snapshot is blind to one side | **DEAD** — I1, I5, I6, I8 |
| RX5 the centre-bit refusal narrowed to the mover's own side | a narrower M7 | **DEAD** — I9, and only I9 (10 passed; 1 failed) |
| RX6 CALL-SITE: `Position::place` no longer calls `threats.apply` | D-553's companion to M12 on the APPLY side, which §4 does not register | **DEAD** — `staged_tests`, 2 tests, panicking at `state.rs:164` (`undo`'s frame refusal) |

**Nothing I designed survived.** One mutant I first wrote, RX3
(`(at.pos % 64).rem_euclid(64)` for `at.pos.rem_euclid(64)`), came back ALIVE and
is recorded here as **rejected, an EQUIVALENT mutant and my own harness error**:
`x % 64 ≡ x (mod 64)`, and `rem_euclid` returns the canonical non-negative
residue, so `(x % 64).rem_euclid(64) == x.rem_euclid(64)` for every `x`. It
computes the same bit and is not a defect. RX3b above is the same class written
so that it actually breaks, and it dies.

So the registered set's coverage holds under eight additional independent defect
classes, including the one D-553 would have asked for on the apply side.

---

## 7. Check 5 — rule 9, code style, fmt, clippy

```
$ wc -l <every touched file>
  168 crates/pistol-search/src/position.rs
  357 crates/pistol-solver/src/cover.rs          ← over, registered (rule9_justifications.md:60)
  142 crates/pistol-solver/src/lib.rs
  265 crates/pistol-solver/src/line.rs
  216 crates/pistol-solver/src/state.rs
   54 crates/pistol-solver/src/table.rs
  549 crates/pistol-solver/tests/threat_oracle_tests.rs  ← over, registered (:71)
$ bash tools/file_justification_check.sh
file_justification_check: 363 tracked .rs/.sh files, 69 over the cap, all registered
                          in docs/rule9_justifications.md (69 entries)
```

`line.rs`, the one new file, is 265 — under the cap, no entry owed. The two files
over the cap already carry entries and neither states a line count.

Style, read rather than argued: no file-top narrative header in `line.rs` (it opens
on `use`); no `//!` outside the crate root; every public item carries `///`, with
`# Panics` on `ThreatState::apply`, on `ThreatState::undo` — *"unless `player`'s
stone at `at` is the most recently applied stone still down"*, which is the
contract §1.4 requires it to state — and on `line::unpack`. The comments I read
say WHY (why the store may be hashed, why `restore` prunes rather than clears,
why the reader never shifts by 64, why the eleven-run exists) and not WHAT. One
comment is false; that is F4.

```
$ cargo fmt --all --check ; echo $?
0
$ cargo clippy --workspace --all-targets --locked -- -D clippy::all
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.47s     (no warning, no error)
```

---

## 8. Check 6 — the five gates that read this state

All run at `3ef6706` in `review-p1-impl-mut`, each cited by the script's own log
lines (never a wrapper's exit status):

| gate | script | its own log line |
|---|---|---|
| 9 | `tools/determinism.sh` | `determinism: ok — 5 seat(s), no difference outside nps/time in any of them` — seats `radius`, `staged`, `staged-heuristics`, `staged-solver`, `staged-safety-net-cap`, each *"ok — 40 searches, 20 positions, no difference outside nps/time"*, incl. run C, one process per position |
| 10 | `tools/search_oracle_check.sh` | `search_oracle_check: the gated seat spends the budget it is given`; final suite `test result: ok. 6 passed; 0 failed` |
| 11 | `tools/staged_soundness_check.sh` | `staged_soundness_check: all four parts passed` (`4/4: THE PATTERN FIXTURES UNDER STAGED`, `test result: ok. 1 passed; 0 failed`) |
| 12 | `tools/solver_oracle_check.sh` | `gate (a) PASS: 61 cases agree with R3'` / `gate (b) PASS: 38 proof trees re-verified full-width` / `gate (c) PASS: 29 wins, 118135 sigma placements replayed and revalued (26865 refused on collision)` / `gate (d) PASS: values agree at both table sizes` / `solver_oracle_check: all four gates passed` |
| 13 | `tools/solver_determinism.sh` | `solver_determinism: PASS — 61 cases, byte-identical transcripts` |

Byte-identity of search output is the pre-registration's leg (§5) and is NOT
claimed here.

---

## 9. Check 7 — anything outside the design's Scope paragraph

```
$ git diff --stat ffc5c10 p1/impl        # identical to `-- crates`
 crates/pistol-search/src/position.rs              |   9 +-
 crates/pistol-solver/src/cover.rs                 |   4 +-
 crates/pistol-solver/src/lib.rs                   |  28 ++-
 crates/pistol-solver/src/line.rs                  | 265 +++++++++++
 crates/pistol-solver/src/state.rs                 | 192 ++++++---
 crates/pistol-solver/src/table.rs                 | 162 +------
 crates/pistol-solver/tests/threat_oracle_tests.rs | 212 +++++++-
```

Nothing outside Scope in the CODE. `query.rs`, `sets.rs`, `zone.rs`, `dfpn.rs`,
`policy.rs`, `solver.rs`, `tt.rs`, `config.rs`, every config and every fixture are
untouched; `pistol-search` gets one comment and no code. The test file is §3's
own deliverable. Two things are worth naming rather than passing over:

- **`docs/decisions.md` is NOT in the diff**, and §2's last paragraph puts it
  there. That is F1.
- **`lib.rs` gains one sentence §2's list of five passages does not name** — *"It
  has happened once: the per-window hashed table D-254 adopted became per-axis
  line bitboards behind the same queries, and `table.rs` kept only the
  window-mask vocabulary."* It sits inside the "STORE is not exported" paragraph,
  which §2 DOES list, so it is inside scope by paragraph and outside it by
  sentence. Recorded as F6, not a defect: it is true of the tree and it is the
  sentence D-261's flip clause earns.

I also checked all nine `ThreatState::undo` call sites against the new
LIFO-or-panic contract, because §1.4 asserts *"Every caller at the governing
revision is LIFO"* and a wrong answer would be a production panic:

```
$ git grep -n -E '\.apply\(|\.undo\(' -- crates/pistol-solver/src ; and Position::place/undo
dfpn.rs:706/708 apply first, second        → :720/722 undo second, first     LIFO
policy.rs:149/150 apply raiser, cell       → :155/156 undo cell, raiser      LIFO (cfg(debug_assertions))
policy.rs:395-399 (test)                                                     LIFO
position.rs:116 apply                      → :145 undo, driven by the `placed` stack   LIFO
policy.rs:343, solver.rs:254, :566                                    apply-only builds
```

`dfpn::apply_turn`/`undo_turn` are exact mirrors under the same
`turn.stone_count() == 2` test, with no early return between them. The contract
holds at every caller.

---

## 10. Findings

### BLOCKING

**F1 — the ADR line §2 requires does not exist.** The design's §2 closes with
*"Docs/ADR: one D-line recording that D-254's flip clause fired on the P1
re-profile and what replaced the store, with the LIFO contract and the
hand-written equality named as the two things a caller can observe."* Rounds 2
and 3 both carried it as an "IMPL verifies" row. It is absent.

*Minimal reproducer:*
```
$ git diff --name-only ffc5c10 p1/impl -- docs | wc -l
0
$ /usr/bin/grep -oE '^D-[0-9]+' docs/decisions.md | tail -1
D-602
$ /usr/bin/grep -n -iE 'line bitboard|per-axis line' docs/decisions.md
549:D-254: …  (only D-254's own "AN OPTION NOBODY CONSIDERED IS RECORDED AND NOT PURSUED" clause)
```
D-254 still reads as the governing decision for a store that no longer exists,
and its flip clause (*"Flips when a bench with `p > 0` names window lookup as a
measured hotspot"*) is unanswered in the log. `ThreatState::undo`'s contract also
changed observably — it now refuses any stone but the last — and hard rule 10
makes that a D-line, not a doc comment.

*Severity:* BLOCKING under the brief's rubric (a design requirement missed), not
because the code is wrong. **The code is not wrong on any check in this report.**
The discharge is one appended D-line naming the three things §2 names; it touches
no code and re-opens no review of §1, §3 or §4.

### MAJOR

None.

### MINOR

**F2 — the mutation receipt records no death REASON, which R2 and R3 asked for on
M14.** `artifacts/p1_mutation_3ef6706_v1.txt` says only *"MUTANT M14 …: DEAD at
…::threat_windows_stop_at_the_edge_of_the_addressable_lattice"*. R2's row: *"verify
it names the refusal as the death … a mutant that dies for a reason other than
its class is a receipt that counts the wrong kill"*; R3's: *"the receipt must
count the I6 kill as the class's and name the other three as the inverse's"*.

*Minimal reproducer* (M14 applied, whole oracle file run):
```
I6:  panicked at crates/pistol-solver/src/state.rs:108:13:
     THREAT_DESYNC: p2 stone on -32768,0 lands on a cell of its ConstR line that already holds one
I1, I5, I8: panicked at crates/pistol-solver/src/line.rs:66:18   (LinePos::cell's expect — the inverse)
```
The FACT R2/R3 wanted is true and is now on the record here; the receipt's wording
is what is owed. A driver that captured the first panic line per mutant would
close this class permanently.

**F3 — "M2 dies at I5 alone" is false.** Carried as a row through all three
rounds. On execution M2 kills 8 of 11 oracle tests (I1, I2, I5, I6, I7, I8, the
legality test, and `applying_a_stone_twice`, the last three via
`sets.rs:194  THREAT_DESYNC: Window { axis: ConstS, start: … } is not in LiveTwo
to remove`). No defect follows — the DESIGN's own §4 M2 row already says *"I5 (and
I1 on any playout crossing a boundary)"*, and dying more widely is a stronger
result. What is wrong is only the review row's word "alone", which a
single-test-per-mutant driver can never establish either way. Recorded so it is
not carried into a fourth document.

**F4 — I5's comment is still false, as R2 and R3 both said.**
`threat_oracle_tests.rs:421-422`: *"on the stones' own axis the windows through
the inner stones straddle the boundary and the outer ones reach across it from
one side."*

*Minimal reproducer, against `LineStore::run`'s own condition
(`offset + count > 64`):* at the `-1/0` boundary the innermost stone is at
position `-1`; `windows_through_indexed` gives it six own-axis windows, starting
at positions `-6 … -1`. The window starting at `-6` has
`offset = (-6).rem_euclid(64) = 58` and `58 + 6 = 64`, which is not `> 64` — it
lies wholly inside chunk `-1` and does not straddle. The same holds for the stone
at position `0` and its window starting at `0` (`offset = 0`). So one own-axis
window of each innermost stone does not straddle, and the sentence is false as
written. Harmless to the test, which compares every window either way; the fix is
one word ("most of the windows", or naming the two that do not).

**F5 — `sets.rs`'s docs still name "the table" as the thing underneath.**
`sets.rs:133` *"there is no shared position field carried in the table"* and
`:156` *"it is what lets the table underneath be hashed"*. Both sentences remain
TRUE of the line store; only the noun is stale. This is a **design gap rather than
an implementation one**: §2 lists `cover.rs` and `state.rs` as the two in-crate
docs to correct and Scope says `sets.rs` is unchanged, so the implementer did what
the design said. (`sets.rs:26`'s *"the table above"* is the module doc's markdown
table and is not stale.)

### Observations, not findings

**F6** — `lib.rs` gains one sentence beyond §2's five named passages, inside a
paragraph §2 names (§9 above).

**F7** — a release-only `warning: unused import: 'generate_turns'` at
`crates/pistol-solver/src/policy.rs:1:44` appears in every gate log. Pre-existing:
`git show ffc5c10:crates/pistol-solver/src/policy.rs | head -1` has the same
line, and `policy.rs` is not in the diff. Out of scope, as R3 recorded.

**F8** — the third privacy doctest does not flip on `pub mod line;` alone,
because `unpack` is `pub(crate)` and the module is only one of two locks; it flips
when both are published (§4 above). Non-vacuous by D-261's own recipe. Recorded so
a future reviewer running the one-step check does not read a pass as a failure of
the guard.

**F9** — the branch is five commits for one feature, and the first
(`4298ecd`) says of itself *"measurement revision only, never a landing candidate
as committed"*. Its 2-file content is fully superseded by the cumulative diff,
which is what this review read. CLAUDE.md Closure wants one feature = one commit;
the landing shape is an operator act and is named here, not judged.

**Rejected finding** — RX3, my own equivalent mutant, recorded with its arithmetic
in §6.

---

## 11. What this review did not do

The identity leg of §5 (byte-identical search output at the three seats) belongs to
`p1_bench_prereg.md` and was not run here. The matrix, its selection record and
the bench pre-registration were not re-litigated. `pistol-core`'s window
enumeration and `sets.rs`'s class algebra were read as given, not re-derived —
they are outside the diff and I1 compares against an independent reference in any
case. The design's §1 and §3 were checked against the code and by execution; the
design's own premises were settled by three prior rounds and are not re-attacked.

---

VERDICT: FAIL (F1 — the D-line §2 requires, recording that D-254's flip clause
fired and naming the LIFO contract and the hand-written equality, does not exist
at `3ef6706`; `docs/decisions.md` is untouched by all five commits, so D-254 still
reads as the governing decision for a store the branch deletes. Four MINOR: F2 the
receipt names no death reason where R2/R3 asked for M14's; F3 the carried row "M2
dies at I5 alone" is false on execution; F4 I5's comment is still false, as R2 and
R3 both said; F5 `sets.rs`'s docs still name "the table" — a design gap, since §2
does not list that file. No BLOCKING defect in the code: §1's mechanism is what
the code does, §2's files and all five root-doc passages are as specified, all
nine §3 invariants are pinned by tests under their design names, the 15-mutant
receipt reproduces byte for byte, and eight further defect classes I designed
myself all die.)
