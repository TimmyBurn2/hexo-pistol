# P2 — REGISTRATION AND RESULTS: the legal-region probe's bounded scan

**Registered before the runs it governs.** Lean by D-604's ruling: what follows
is the expectation, the criterion and the abort, and nothing that does not
change a reading. Numbers never move after the fact (D-374).

## What changed and why it is byte-identical

`Board::in_legal_region` asked every stone on the board whether it was within
`LEGAL_RADIUS`. It now asks only the stones whose `q` lies within that radius,
taken as one `BTreeMap` range. **The bound is exact, not an approximation**: hex
distance is `(|dq| + |dr| + |dq+dr|) / 2`, and `|dr| + |dq+dr| >= |dq|` by the
triangle inequality, so `distance >= |dq|` and a stone further than
`LEGAL_RADIUS` in `q` alone can never be within that distance. `Coord` orders
lexicographically by `q` then `r`, which its own doc calls load-bearing, so the
band is a contiguous range. Skipping the rest changes no answer, which is why
search output must be byte-identical and why no strength run is owed (D-495).

Pinned by `the_region_probe_agrees_with_an_unbounded_scan_over_every_stone`
(`crates/pistol-core/tests/legal_region_tests.rs`), which compares the banded
probe against a full sweep over every stone on boards built to straddle the
band's edges at spreads of 1, 7, 8, 9, 16 and 40, over ~70 000 cells.

## The premise, MEASURED at the package's own revision (D-477)

`perf record -F 2000` at `45df2c8`, `configs/instrument_v0.toml`:

| seat | `Board::check_placement` |
|---|---|
| spacing-2 grid, 1 025 stones, `go nodes 2000` | **23.25 %**, the top symbol |
| `bench_positions_v1.txt` line 2, 15 stones, `go nodes 400000` | **below 1.5 %**, absent from the profile |

**AND THE AUDIT NAMED THE WRONG CALL SITE.** A-04 says the linear probe is
*"called per candidate cell from `crates/pistol-search/src/candidates.rs:56`"*.
That site exists, but removing its cost alone was measured at **4 %** on the
1 025-stone position (a prototype that special-cased the filter when the
candidate radius is no wider than the region's, kept only long enough to
measure). The dominant caller is `GameState::place` at `state.rs:163`, the
rules' own fail-loud validation of every stone the SEARCH PLACES — millions of
calls per search, not per candidate.

## What is registered, before the runs

- **Instrument:** `tools/bench_delta.sh` at `ab369b0`, `rev:45df2c8` against
  `rev:7731d71a77624baa12cee5d0dfbb54bde303c313`, 5 reps, on an idle box with a
  receipt at both ends.
- **EXPECTATION, and it is a NULL:** at the bench seat the probe is under 1.5 %
  of wall, so the registered bands cannot show this package. **Expected nps
  ratio 1.00 to 1.02 in both bands.** A package whose value does not appear at
  its own registered seat is a finding, not a failure (rule 5).
- **ABORT:** an nps ratio **< 0.98** in either band. The change would then be
  costing time at the seat that governs, which the range's own arithmetic says
  it cannot, and something other than the registered mechanism moved.
- **WHERE THE VALUE IS CLAIMED INSTEAD:** the growth series below, which is the
  receipt A-04 registered and which the bench's fixture cannot reach.
- **IDENTITY:** the same three-seat, 128-search block P1 registered
  (`p1_bench_prereg.md` §2.1), run at this revision. **Any mismatch = STOP.**

## Results, and the verdict is DO NOT LAND

**The bench at the registered seat, `artifacts/p2_bench_v1.txt`** — idle both
ends, node identity holding per position at both budgets in all five reps,
exit 0:

| band | nps | time-to-depth | registered expectation |
|---|---|---|---|
| early | **0.994** | 0.972 | 1.00–1.02 — **missed** |
| late | **0.995** | 1.000 | 1.00–1.02 — **missed** |

Above the registered abort of 0.98 and below the registered expectation. **The
identity leg is IDENTICAL** (`artifacts/p2_identity_v1.txt`: 128 `bestmove`, 0
`error` per side, exit 0), so the change is what it claims to be; it is simply
not worth what it costs here.

**WHERE IT STARTS PAYING, which is the number that decides the package.** Same
two binaries, same positions, `go nodes 20000`, three reps each:

| stones | baseline ms | banded ms | ratio |
|---|---|---|---|
| 39 | 2.0 | 2.0 | 1.000 |
| **79** | 67.0 | 68.0 | **0.985** |
| 159 | 228.0 | 226.3 | 1.007 |
| 319 | 34.7 | 32.3 | 1.072 |
| 641 | 86.0 | 71.3 | 1.206 |
| 1 025 | 7 571.0 | 6 111.3 | 1.239 |

**THE CROSSOVER IS NEAR 160 STONES AND NO GAME REACHES IT.** Rule 3 gives one
stone on turn 1 and two on every turn after, so a match under the 40-turn cap
the arena imposes ends at **79 stones**; `bench_positions_v1.txt`'s two bands
centre on 15 and 35. Every size this engine plays sits on the losing side of the
crossover, and every size that wins sits beyond any game. The `BTreeMap` range
seek costs `O(log n)` before the first comparison, which a short scan never
recovers.

**VERDICT: the change is NOT landed.** Rule 5's own words — *"a measured
structural floor is a finding, not a failure"*. Trading half a per cent at every
seat the engine plays for a quarter on positions it never reaches is a bad
trade, and the measurement is what says so rather than a preference. The branch
`p2/impl` at `7731d71a77624baa12cee5d0dfbb54bde303c313` keeps the implementation
and its differential test for whoever revisits it.

## What P2 leaves behind, and it is worth more than the change was

1. **A-04's call site is wrong in the audit and right here.** The linear probe
   is paid by `GameState::place` (`crates/pistol-core/src/state.rs:163`), the
   rules' fail-loud validation of every stone the search plays, not by candidate
   generation. A successor optimising this looks there.
2. **The remedy A-04 proposes — a maintained region — is not obviously better.**
   A radius-8 hex ball is 217 cells, so a per-stone-ball occupancy count pays 217
   updates per apply and per undo to make the query `O(1)`. Against a scan that
   this measurement shows costs nothing at play sizes, that is a worse trade
   than the one rejected above. Anyone building it should measure at 79 stones
   first.
3. **The bound itself is exact and is written down**: hex distance dominates
   each axial component, so the `q`-band is a sound prune. If a future position
   size makes it worth taking, the argument and its differential test already
   exist on the branch.
4. **The false comment A-04 named is still false.** `board.rs`'s
   `in_legal_region` doc says candidate generation *"never pays this per
   candidate"* — it does, at `candidates.rs`, though that cost is the small half.
   Correcting the comment is a docs-only change this package did not make
   because it did not land; it is owed.
