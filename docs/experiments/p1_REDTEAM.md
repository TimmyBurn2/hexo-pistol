# P1 — RED-TEAM: adversarial inputs on the threat state's rules and data paths

**Date:** 2026-09-04.
**Live tree HEAD:** `ffc5c10f4d16356f574e3221a023f78399d2e3bb` (`dev`), re-read at
the end of this session; the P1 design set is uncommitted there, at the stash
below. The BASELINE worktree is at that same revision, so this report's baseline
IS HEAD.

**BASELINE binary** — worktree `/home/tom/pistol-wt/rt-base`, detached at
`ffc5c10f4d16356f574e3221a023f78399d2e3bb`, which IS the live tree's HEAD.
`target/release/pistol`
sha256 `78a7600adcf099de0b04149535f1f4bffe0b6c945609a3206d73a4e5ee853749`.

**CANDIDATE binary** — worktree `/home/tom/pistol-wt/rt-cand`, detached at
`3ef67068a15880c15e372b22565e0f4f37de8e76`, which is the tip of `p1/impl` and a
direct descendant of the baseline (`git merge-base --is-ancestor ffc5c10
3ef67068a1588…` succeeds), so `git diff ffc5c10 3ef67068a1588… -- crates` is
exactly the package. `target/release/pistol`
sha256 `1413698a22ffbb95fd008b2f27c533c88e3399b8806b62f71b4f827e30271e6d`.

**THE DISPATCH NAMED A REVISION THAT DOES NOT EXIST.** The dispatch to this
session named the candidate `3ef67062`; the worktree it also named is at
`3ef67068a1588…` — the eighth hex digit differs, and `3ef67062` resolves to
nothing in this repository. The binary digest in the dispatch
(`1413698a22ff…`) matches the worktree's binary exactly, so the worktree is the
intended one and every result below is at `3ef67068a1588…`. Recorded because a
review is dispatched against a NAMED REVISION and this report's header is where
that name is checked (CLAUDE.md Process). See MINOR-1.

**Design revision:** `docs/experiments/p1_design.md` at stash
`db4ecd9e86bff8c07507183dc0570ed5e787985f` (revision 3). §5 states the
correctness claim as BYTE-IDENTITY of search output under D-495's two-track
law; §6 states what is not claimed.

**Both digests were re-verified in this session before use:**

```
$ cd /home/tom/pistol-wt/rt-base && git rev-parse HEAD && sha256sum target/release/pistol
ffc5c10f4d16356f574e3221a023f78399d2e3bb
78a7600adcf099de0b04149535f1f4bffe0b6c945609a3206d73a4e5ee853749  target/release/pistol
$ cd /home/tom/pistol-wt/rt-cand && git rev-parse HEAD && sha256sum target/release/pistol
3ef67068a15880c15e372b22565e0f4f37de8e76
1413698a22ffbb95fd008b2f27c533c88e3399b8806b62f71b4f827e30271e6d  target/release/pistol
```

Neither worktree carries a modified tracked file and neither has a branch; the
only additions are the three untracked scratch harness files listed in §7. No
cargo ran in the live tree, in `/tmp`, or in any worktree this session does not
own.

---

## 1. What was attacked, and how

Two independent instruments, because the identity claim and the correctness
claim are different claims:

* **The STATE directly** — `pistol_solver::ThreatState` driven against the test
  crate's own `Reference` (`crates/pistol-solver/tests/common/reference.rs`,
  which reads the BOARD and never the state), comparing every query plus
  `table_snapshot`, `window_count`, `masks` and whole-state equality after
  every apply and every undo. This reaches the `i16` lattice edge, which the
  line protocol cannot (§4, attack E).
* **The BINARIES** — both `target/release/pistol` on identical stdin, full
  transcripts diffed with `sed -E 's/ nps [0-9]+ time [0-9]+//'`. This is the
  identity leg's own instrument, run on inputs the pre-registration's three
  seats do not carry.

A transcript pair is only counted as evidence when **both** runs exited 0. Two
guards were added to the runner after they caught real harness faults:

* **VACUOUS** — a position the engine REFUSES gives both binaries the same
  empty-board answer, so identity there proves nothing. HARNESS FAULT FOUND AND
  FIXED: the first position generator emitted turn pairs in placement order,
  and the protocol requires *"a pair is written smaller cell first,
  lexicographic by (q, then r)"*. **32 of 75 positions were silently refused**
  and their 192 "identical" transcripts were worthless. The generator was
  rewritten (`genpos2.py`) so that both cells of a turn are legal against the
  board as it stood at the START of the turn and the pair is then sorted; it
  validates every tail against those rules before printing, and all 79
  positions were then confirmed accepted by the engine before any comparison
  ran. The runner now records VACUOUS separately; the final counts below carry
  zero.
* **TIMEOUT** — a run either side killed by the wall cap is truncated at an
  arbitrary point, so it is recorded and NEVER compared. Capped pairs are
  excluded from every count of compared transcripts.

---

## 2. The attack log

### A. The state against the reference (1,167,813 compared steps)

`crates/pistol-solver/tests/rt_adversarial.rs` (sha256
`247ac23703fcb39905181e4014f34032c91e828913cf5320d72446eed9573c5c`) and
`crates/pistol-solver/tests/rt_bulk.rs` (sha256
`07e9628561b4d9ad16eb91b640a948cf6f242e7db5ca7a3c868a95d814da6c4b`), untracked
in both worktrees, run in release with each worktree's own `CARGO_TARGET_DIR`.

Every step compares, against `Reference::from_board`: the whole
`table_snapshot` as a `BTreeMap<Window, (u8, u8)>`; `window_count()`; `masks()`
for every window the reference knows AND for every window through every stone;
`hot_windows`, `win_in_one_ply_windows`, `completed_windows` and
`live_windows_at_count` at both counts, per side, AS ORDERED SLICES;
`threat_cells`, `win_in_one_ply_cells`, `cells_raising_to_hot` and
`live_cells_at_count` at both counts, per side. Where the hot set is small it
also compares `min_hitting_set_exceeds`, `blocking_covers` and
`unblockable_double_threat` at all three budgets and `can_win_this_turn` at both
`StonesLeft` values. Every sequence is then unwound LIFO, comparing whole-state
equality against a clone recorded at that ply and re-comparing the full surface
after each undo, and ending on `== ThreatState::new()`, `is_empty()` and
`window_count() == 0`.

| class | what | steps |
|---|---|---|
| every chunk residue | 13-stone chains whose window starts and whose eleven-position `apply` runs sweep **all 64 offsets**, on each axis, at four chunk bases, four colourings — both the `offset + 6 > 64` and the `offset + 11 > 64` arms of the run reader entered from every side | 768 chains |
| boundary chains | 16-stone chains centred on 0, ±64, ±128, ±192, ±512, ±4096, each axis, four colourings, full cover surface | 132 chains |
| full chunk | 64 same-side stones on one line, aligned and straddling (from position 60, −4, 124, −68) | 24 chains |
| two chunks | one line holding stones in chunks 0, 3, −4 and 10 — non-adjacent, so the high-chunk arm must not manufacture a stone between them | 3 |
| three axes | eleven stones on each of the three lines through one boundary cell, at ±64, ±128 | 4 |
| lattice corners/edges | all four `i16` corners; two corners together; walks 12 stones in from each corner along each axis in both directions | 4 + 1 + 24 |
| extreme ConstS | the `q + r` line at and near its bounds (−65536 and 65534 are the two corner cells) | 6 anchors × 3 axes |
| sixes and overlines | a completed six and a nine-stone overline, each axis, at four boundary offsets | 24 |
| spaced-8 chain | 257 stones at spacing 8 on each axis (docs/audit A-01) | 3 |
| dense grids | spacing 1 and 2, 11×11 and 13×13, at five anchors including three chunk boundaries (A-04) | 10 |
| clone | a clone taken mid-sequence and unwound to nothing while the original is checked; a clone driven forward with DIFFERENT stones on each copy, converged, then both unwound | 2 |
| deep LIFO | 400 stones down and back up, whole-state equality at every ply | 1 |
| bulk walks | **20,000** random walks of 30 stones, anchored at 44 awkward places — chunk boundaries on each axis, chunk 8, chunk 64, and ±32700 near the edge — at four spacings | 1,066,652 |

**Total compared steps: 101,161 (`rt_adversarial`) + 1,066,652 (`rt_bulk`) =
1,167,813.** Every one agreed with the reference on the candidate.

The same two files were run against the BASELINE, which agreed with the
reference on the same steps (101,161 and 1,066,652 — identical counts). The one
test that fails on the baseline is `every_refusal_names_threat_desync`, and it
fails on exactly the refusal the design adds (§4, attack A).

### B. Transcripts through the line protocol

79 adversarial positions generated by `genpos2.py` (sha256
`d7a05441d6a9c045e1b1cdff9429f754c22b92f11b84e3ed195d33f4b7027530`, output
`positions2.tsv` sha256
`a387d2fd687e8a1786d0c8d4c68af451436ca3e29cbe372240141d953a87202a`; 25,159
stones over the 79), plus the twenty `tactical_staged_v0.txt` positions and
twenty `phase:1` variants of them, plus one 2,025-stone spacing-2 grid.

Classes, all in the `start moves` grammar and all legal games under rule 5
(placement walks out in hops of at most 4, so both stones of every turn are
legal against the turn-start board): straight chains crossing chunk boundaries
at 0, ±64, ±128, ±192, ±256, ±512, ±1024, ±4096 on each axis; 257-stone
spaced-8 chains on each axis; spacing-2 grids (17×17) and spacing-1 blocks
(9×9) at the origin and at three chunk boundaries; one line holding stones in
four non-adjacent chunks; all three axes through one boundary cell; six
parallel nine-runs straddling a boundary (many hot windows for both sides);
completed sixes and nine-stone overlines on each axis at four offsets.

| pass | inputs | budgets | configs | compared | search / refusal-only | differing | capped |
|---|---|---|---|---|---|---|---|
| F | 79 adversarial | `depth_turns 1`, `nodes 10000` | instrument_v0, gate_staged_v0, gate_staged_solver_v0 | **472** | 472 / 0 | **0** | 2 |
| G + G3 | 79 adversarial (STOPPED, see below) | `depth_turns 2` | the same three | **169** | 169 / 0 | **0** | 23 |
| C | 20 tactical | `depth_turns 2`, `nodes 10000` | the same three | **120** | 120 / 0 | **0** | 0 |
| D | 20 tactical + 20 `phase:1` | `depth_turns 2`, `nodes 10000` | the same three | **240** | 120 / 120 | **0** | 0 |
| DEEP | 20 tactical + 20 `phase:1` (STOPPED) | `depth_turns 3`, `depth_turns 4`, `nodes 300000` | gate_staged_solver_v0, gate_staged_v0, bench_wp18c_solver_on, play_staged_solver_v0, instrument_staged_v0 | **626** | 461 / 165 | **0** | 11 |
| S | 80 MULTI-POSITION SESSIONS | five `go`s per session, revisiting an earlier position | gate_staged_solver_v0, gate_staged_v0 | **80** | 80 / 0 | **0** | 0 |
| H | 28 hostile / malformed tails | `depth_turns 1`, then a known-good position and `depth_turns 2` | instrument_v0, gate_staged_solver_v0 | **56** | 56 / 0 | **0** | 0 |

**PASSES G AND DEEP WERE STOPPED, NOT EXHAUSTED**, and their rows above are what
they had compared when they were stopped. Both were spending their wall on
runs the 90 s / 180 s cap kills — `depth_turns 2` on the twelve `far_*`
positions and the three spaced-8 chains never completes an iteration, because
the root turn's pair enumeration scales with the LEGAL REGION and those
positions spread stones over thousands of cells (a measured 160 s for ONE
`depth_turns 1` iteration on `far_ConstQ_b4096` under the solver-armed seat).
A capped run is not evidence, so continuing would have added capped rows and
nothing else; every one of those positions IS compared at `depth_turns 1` and
`nodes 10000` in pass F, which completed. This is a stop for cost, stated so it
is not read as a completed matrix.

**`phase:1` IS NOT A SEARCHABLE INPUT, and that is the engine refusing loudly.**
All 40 rows of the `phase:1` file are accepted by `position`; `go` then refuses
every one of them, identically on both binaries:

```
$ printf 'newgame\nposition set p1:0,0 1,0 2,0 3,0 4,0 p2:-1,0 1,3 2,3 3,3 1,5 tomove:p2 phase:1\ngo depth_turns 3\nquit\n' \
    | ./target/release/pistol --config configs/gate_staged_solver_v0.toml
error PositionNotSearchable: turn 6 is half played: the search starts at a turn boundary
```

So 285 of the pairs above are pairs where both binaries REFUSED identically —
fail-loud parity (rule 3), not search identity. They are counted separately in
the table's "search / refusal-only" column and are not claimed as searches. The
guard that separates them was added to the runner mid-flight, after it caught
the generator fault described in §1; passes C, D and DET predate it, so their
rows were re-classified afterwards by name (`tacticalph1_*`).

**Total transcript pairs compared: 1,763. Differing: 0. Vacuous (a position
the engine refused that was counted as identical anyway): 0.** Of the 1,763,
**1,478 ran a search** and 285 are the identical `phase:1` refusals above.

Read the table without double-counting: pass D's 120 searching pairs are the
SAME twenty positions at the SAME three configs and two budgets as pass C, so D
contributes no search evidence C does not already carry — its 120 `phase:1`
rows are its only addition. The distinct searching evidence is F (472), G+G3
(169), C (120), DEEP (461), S (80) and H (56).

Pass S is the one that reaches `Position::reset_to` — the seam the design
changed the comment on, where the threat state is REPLACED rather than unwound
because its `undo` is now LIFO-or-panic. Each session visits three positions in
ONE process, goes on each, returns to the first and goes again, in two shapes
(with and without intervening `newgame`); a stale or half-rebuilt state shows
as a different second answer for the same position. None differed.

### C. Determinism (hard rule 4)

* `tools/determinism.sh` at `3ef67068a1588…`, in the candidate worktree with
  its own `CARGO_TARGET_DIR`:

  ```
  determinism: seat radius: ok — 40 searches, 20 positions, no difference outside nps/time
  determinism: seat staged: ok — 40 searches, 20 positions, no difference outside nps/time
  determinism: seat staged-heuristics: ok — 40 searches, 20 positions, no difference outside nps/time
  determinism: seat staged-solver: ok — 40 searches, 20 positions, no difference outside nps/time
  determinism: seat staged-safety-net-cap: ok — 40 searches, 20 positions, no difference outside nps/time
  determinism: ok — 5 seat(s), no difference outside nps/time in any of them
  ```

* The candidate against ITSELF, twice, in SEPARATE PROCESSES, over 119 distinct
  adversarial and tactical positions × 3 configs × 2 budgets:

  ```
  candidate runs compared (two processes each): 834
  differing: 0
  capped: 2
  ```

  832 compared, none differing outside `nps` and `time`; of those, 712 ran a
  search and 120 are the identical `phase:1` refusals.

### D. The solver seat

* `tools/solver_oracle_check.sh` at `3ef67068a1588…`, its own log lines:

  ```
  gate (a) PASS: 61 cases agree with R3'
  gate (b) PASS: 38 proof trees re-verified full-width
  gate (c) PASS: 29 wins, 118135 sigma placements replayed and revalued (26865 refused on collision)
  gate (d) PASS: values agree at both table sizes
  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 298.75s
  solver_oracle_check: all four gates passed
  ```

* `tools/solver_determinism.sh`: `solver_determinism: PASS — 61 cases,
  byte-identical transcripts`.
* `tools/staged_soundness_check.sh`: `staged_soundness_check: all four parts
  passed`.
* The twenty `tactical_staged_v0.txt` positions under
  `configs/gate_staged_solver_v0.toml` at `depth_turns 2` and `nodes 10000`:
  identical on both binaries (pass C above), and again at `depth_turns 3`,
  `depth_turns 4` and `nodes 300000` under four solver-armed configs (pass
  DEEP).

### E. Resource shape

`/usr/bin/time` is NOT INSTALLED on this machine (`command -v time` finds only
the shell builtin; `ls /usr/bin/time` → no such file), so peak RSS was read
with `resource.getrusage(RUSAGE_CHILDREN).ru_maxrss` in a per-run collector
process — the same kernel counter `time -v`'s "Maximum resident set size"
prints. Identical stdin, one fresh process per measurement.

A 2,025-stone spacing-2 grid (`bigrid2`), the shape the dispatch asked for:

| config, budget | baseline | candidate | delta |
|---|---|---|---|
| gate_staged_solver_v0, `depth_turns 1` | 863,296 kB / 878.92 s | 864,260 kB / 802.67 s | **+0.11 % RSS, −8.7 % wall** |
| instrument_v0, `depth_turns 1` | 200,720 kB / 0.09 s | 201,064 kB / 0.09 s | +0.17 % RSS |
| instrument_v0, `nodes 200000` | 201,664 kB / 7.69 s | 201,480 kB / 7.48 s | −0.09 % RSS |
| gate_staged_v0, `depth_turns 1` | 16,500 kB / 0.06 s | 16,604 kB / 0.06 s | +0.63 % RSS |
| gate_staged_v0, `nodes 200000` | 17,392 kB / 7.69 s | 17,348 kB / 7.47 s | −0.25 % RSS |

Both binaries returned the same `bestmove` in every row. The design's own
statement — the log grows with stones applied and not undone, so a root's
stones stay logged for a whole search — is visible and is small: at 2,025
stones under the solver-armed seat the log costs about 1 MB against an 863 MB
peak (0.11 %), and the candidate was FASTER on the same input. The smallest
absolute headroom is the 16 MB staged seat, where the same ~100 kB is 0.6 %.
This is a resource observation, not a finding.

---

## 3. Findings

### BLOCKING

**None. No wrong byte was found.**

### MAJOR

**None.**

### MINOR

**MINOR-1 — the dispatch's candidate revision does not resolve.** The dispatch
named `3ef67062`; the worktree it named is at `3ef67068a15880c15e372b22565e0f4f37de8e76`,
and `git rev-parse 3ef67062` fails in this repository. The binary sha256 the
dispatch also gave matches the worktree's binary exactly, so the subject is not
in doubt, but a review dispatched against a revision that does not exist cannot
have its header checked against HEAD — which is the whole point of naming one.
Reproducer: `cd /home/tom/pistol-wt/rt-cand && git rev-parse HEAD` prints
`3ef67068a15880c15e372b22565e0f4f37de8e76`. Fix: the landing's citations name
the full SHA.

**MINOR-2 — a release build of `pistol-solver` emits an `unused_imports`
warning, at BOTH revisions.** `crates/pistol-solver/src/policy.rs:1` imports
`generate_turns`, whose only use is inside a `#[cfg(debug_assertions)]` block at
`policy.rs:266`, so `cargo build --release -p pistol-solver` warns and
`cargo build -p pistol-solver` does not. Reproducer:

```
$ cd /home/tom/pistol-wt/rt-base && CARGO_TARGET_DIR=$PWD/target cargo build -p pistol-solver --release --locked 2>&1 | grep -c warning
1
$ cd /home/tom/pistol-wt/rt-base && CARGO_TARGET_DIR=$PWD/target cargo build -p pistol-solver --locked 2>&1 | grep -c warning
0
```

**PRE-EXISTING at `ffc5c10`, not introduced by this package** — it is recorded
here only because it is in the diff's own crate and a reader of this report's
build logs will see it.

---

## 4. Attacks that were run to ground and did NOT land

Recorded so they are not re-attempted, each with what was actually run.

**A. `undo` is no longer order-free, and that is the contract change.** The
baseline accepts an out-of-order `undo` silently; the candidate refuses it.
Confirmed by running the SAME harness on both:

```
$ cd /home/tom/pistol-wt/rt-base && cargo test -p pistol-solver --test rt_adversarial --release
thread 'every_refusal_names_threat_desync' panicked at rt_adversarial.rs:584:26:
out-of-order undo must panic: ()
test result: FAILED. 11 passed; 1 failed
$ cd /home/tom/pistol-wt/rt-cand && cargo test -p pistol-solver --test rt_adversarial --release
test result: ok. 18 passed; 0 failed
```

Every `ThreatState::undo` caller at `3ef67068a1588…` was read and every one is
LIFO: `Position::undo` pops a `Vec` (`position.rs:129-146`); `dfpn::undo_turn`
undoes `second` then `first` mirroring `apply_turn` (`dfpn.rs:713-723`);
`policy.rs:155-156` and `policy.rs:398-399` likewise; `Position::reset_to`
REPLACES the state rather than unwinding it (`position.rs:63-65`), which is the
only correct spelling under the new contract and is the comment the design
changed. Nothing undoes out of order, and the 80 multi-position sessions and
the 190 solver-armed deep searches exercise all of them.

Seven refusals were checked to panic with `THREAT_DESYNC` in the message:
`undo` on an empty state, `undo` of a stone never applied, `undo` of the other
player's stone, out-of-order `undo`, double `apply` (both same-side and
cross-side), and double `apply` at the `i16` corner. A `Clone`d state was shown
to undo independently of its original in both directions, and a state that
REFUSED an `apply` was shown byte-equal to its pre-refusal clone.

**B. Key-field overlap in the new packing.** `pack` puts the axis at bit 40,
the line in 24 bits at bit 16, the chunk in 16 bits at bit 0, with biases
`1 << 17` and `1 << 10`. Over the ranges the lattice and the reader actually
reach — line `q + r ∈ [-65536, 65534]`, chunk `∈ [-513, 512]` (the eleven-
position run around a stone at either end keys one chunk past the lattice) —
the biased fields are `[65536, 196606] ⊂ [0, 2²⁴)` and `[511, 1536] ⊂ [0, 2¹⁶)`,
so no field can carry into the next and `pack` is injective. Both extremes are
*exercised*, not only argued: the `i16` corners give `q + r = -65536` and
`65534` exactly, and give chunk indices ∓512 with the run reader reaching ∓513
and 512. Those tests read every class set as well as the store and pass.

**C. The run reader's straddling arm.** `offset + count > 64` is false at
`offset == 0`, so no shift by 64 is ever formed; the sweep in §2A drove all 64
residues on all three axes for both the 6-wide window read and the 11-wide
`apply` read, in four colourings, and compared every class set at each step.

**D. A line holding stones in non-adjacent chunks.** The high-chunk OR could
manufacture a stone across a gap. Driven on each axis with stones in chunks 0,
3, −4 and 10 of one line, full cover surface, apply and undo: no difference.

**E. The cell where an axis carries NO addressable window.** At
`(32767, 32767)` the ConstS axis yields no window from
`windows_through_indexed` (the baseline therefore never touches ConstS there),
while the candidate's per-axis loop still sets a ConstS bit and still checks a
refusal on it. HYPOTHESIS: the candidate refuses a stone the baseline accepts,
or reports a different `is_empty`/`window_count`. FALSIFIED — both binaries
were probed side by side (`rt_probe.rs`, sha256
`59a45bf8d7a6f7ec1ec6f799173788535aaa6bc9662bbd2fac797f5ce8d4cd4a`):

```
BASELINE  PROBE windows_through_indexed(32767,32767) = 2
BASELINE  PROBE after apply: is_empty=false window_count=2 snapshot_len=2
BASELINE  PROBE second apply at the corner: Some("THREAT_DESYNC: p2 stone on 32767,32767 lands on cell 5 of Window { axis: ConstQ, ... }, which already holds one")
BASELINE  PROBE after undo: is_empty=true equals fresh=true
CANDIDATE PROBE windows_through_indexed(32767,32767) = 2
CANDIDATE PROBE after apply: is_empty=false window_count=2 snapshot_len=2
CANDIDATE PROBE second apply at the corner: Some("THREAT_DESYNC: p2 stone on 32767,32767 lands on a cell of its ConstQ line that already holds one")
CANDIDATE PROBE after undo: is_empty=true equals fresh=true
```

Every addressable cell has at least one ConstQ window (`back` can always be
chosen in `[0, 5]`), so the baseline's per-window check always fires on an
occupied cell too; the candidate's per-axis check is strictly stronger and
never *weaker*. The extra ConstS bit is invisible: `snapshot` filters
`windows_through_indexed` by axis and so contributes nothing for it. The
message text differs — it names an axis rather than a window — but the token
`THREAT_DESYNC` is verbatim in both, which is what the contract pins.

**F. The lattice edge through the PROTOCOL.** Unreachable, and this is a rules
fact rather than a gap: the engine refuses any first stone off the origin
(*"the first stone of the game goes on 0,0"*) and any stone outside the
radius-8 region, so a game reaching `i16::MAX` is ~4,096 turns of legal-region
growth away. The edge is attacked through the state API instead (§2A), which is
the only door that reaches it.

**G. Hostile and malformed protocol input.** 28 tails × 2 configs: coordinates
at and beyond the `i16` bounds (`32768,0`, `-32769,0`), a first stone off the
origin, duplicate stones within and across sides, a stone outside the legal
region, wrong turn parity, an unsorted pair, a self-pair, a three-cell "pair",
non-numeric coordinates, a bad `tomove`, `phase:9`, a missing section, sections
out of order, an unknown section, and bare garbage. Every one was refused by
name on BOTH binaries with the SAME message, no panic, no `THREAT_DESYNC`, no
`POSITION_DESYNC`; and the known-good position and search that followed in the
same process answered identically, so no refusal left a corrupted state behind
(48 named `error` lines over the 56 transcripts).

**H. Resource blow-up from the log.** Measured, §2E: +0.11 % peak RSS at 2,025
stones under the solver-armed seat, and the candidate was faster on the same
input. No blow-up.

---

## 5. What this red team did NOT cover

Stated so it is not read as covered.

* **No mutation testing.** That is the implementer's registered set (design §4)
  and it runs in its own worktree.
* **No claim about strength or throughput.** D-495's identity track takes no
  SPRT; the timing numbers in §2E were taken beside other jobs on this machine
  and are a resource SHAPE observation, void as a timing receipt (D-592).
* **No full `tools/ci.sh` run.** Five gates that read this state were run
  individually and are quoted above with their own log lines; the rest is
  REVIEW-impl's and the landing's.
* **The `bench_delta.sh` per-position node identity** named by design §5 is the
  pre-registration's leg, not this one.

---

## 6. VERDICT

VERDICT: NO WRONG BYTE FOUND (120 positions, 1763 transcripts compared, 1167813 state steps against the reference)

Read as: 120 distinct positions (79 generated adversarial games, the 20
committed `tactical_staged_v0` positions, 20 `phase:1` variants of them, and one
2,025-stone spacing-2 grid), plus 28 hostile or malformed protocol tails; 1,763
transcript pairs diffed between the two binaries with `nps` and `time` elided,
of which 1,478 ran a search and 285 are pairs where both binaries refused a
`phase:1` position identically; 1,167,813 apply/undo steps of `ThreatState`
compared against the test crate's independent `Reference` on the CANDIDATE, and
the same 1,167,813 on the BASELINE. Zero differences of any kind.

This does NOT clear passes G and DEEP's unrun rows (§2B) or anything listed in
§5 as not covered. Under D-495 nothing here flips the package to the SPRT
track.

---

## 7. Reproducing this

Scratch harness, untracked, left in place in `/home/tom/pistol-wt/rt-cand/`
(the dispatch's permitted location) and removed from `rt-base` after use:

| file | sha256 |
|---|---|
| `crates/pistol-solver/tests/rt_adversarial.rs` | `247ac23703fcb39905181e4014f34032c91e828913cf5320d72446eed9573c5c` |
| `crates/pistol-solver/tests/rt_bulk.rs` | `07e9628561b4d9ad16eb91b640a948cf6f242e7db5ca7a3c868a95d814da6c4b` |
| `crates/pistol-solver/tests/rt_probe.rs` | `59a45bf8d7a6f7ec1ec6f799173788535aaa6bc9662bbd2fac797f5ce8d4cd4a` |

The state-level attacks are re-run by copying those three into either worktree
and running, with that worktree's own `CARGO_TARGET_DIR` and never in the live
tree:

```
cargo test -p pistol-solver --test rt_adversarial --release --locked -- --test-threads=1 --nocapture
cargo test -p pistol-solver --test rt_bulk --release --locked -- --nocapture
```

The transcript passes were driven by these scripts, whose digests are recorded
because the scripts themselves live in an EPHEMERAL scratchpad and will not
survive the session:

| script | sha256 |
|---|---|
| `genpos2.py` (the position generator and its own validator) | `d7a05441d6a9c045e1b1cdff9429f754c22b92f11b84e3ed195d33f4b7027530` |
| `positions2.tsv` (its 79 positions) | `a387d2fd687e8a1786d0c8d4c68af451436ca3e29cbe372240141d953a87202a` |
| `diff_run.sh` (the two-binary transcript diff) | `aa0e2486fd86d79a0c9a639e3fea49ed411dca6d560b1934ee861f13bb0889ee` |
| `session_diff.sh` (the multi-position sessions) | `7dfdfc2091dda9e9309fc6d1583c90b35db0a9a20cdfeb0cfc5f9d915f250823` |
| `determinism_diff.sh` (candidate against itself, two processes) | `863d0aef9c9fa5a0c4683759195794d9c88a187af6d8faddfb39eb77641eecb0` |
| `hostile.sh` (malformed protocol input) | `47748098eaac0e5eb67e48a7bea06b8ac7dc78c28b497acaa743279bbacba79b` |
| `rss.py` (peak RSS and wall, `getrusage`) | `a755f528377dcddecdead8704a7ecb296cb59a1c13490fd41fefb570385e219f` |

The single-run shape every transcript pass used:

```
printf 'newgame\nposition <tail>\n<budget>\nquit\n' \
  | timeout <cap> ./target/release/pistol --config <config> 2>&1 \
  | sed -E 's/ nps [0-9]+ time [0-9]+//'
```
