# MATRIX P1 — the threat state's per-stone maintenance (revision 3, amended under D-598 after round 3)

**Round 3 (`matrix_P1_threat_state_REDTEAM_round3.md`, remedies-only, the
last granted round) returned FAIL on ONE MAJOR that lives in
`p1_bench_prereg.md`'s identity block — a document whose own gate had not
opened — and four MINOR corrections to this document that change no reading.**
The disposition is recorded in `opt_arc_ledger.md` F-P1.4: the prereg's
failing lines go to the prereg's gate as its first finding; this document's
four minors are corrected below and disposed of by a scoped verification pass
over the corrected lines and their defect class (D-598's shape), not by a
fourth round. Every number in this document reproduced under the reviewers'
own commands in all three rounds; the MAJORs of rounds 1 and 2 were record
claims in its own lines, corrected in §0 and §0.1, and round 3's was the
prereg's.

**Revision 3 is the third and last round the dispatch grants, and it is
remedies-only.** Revision 2 (`40b69e3`, a stash commit on `ffc5c10`) was
attacked by a second fresh-context DECISION-RED-TEAM
(`matrix_P1_threat_state_REDTEAM_round2.md`), which executed every round-1
remedy and found all of them holding, then returned *"the recommendation
SURVIVES but the matrix FAILS (revise: R2-M1, R2-M2)"* with eight minors. Both
MAJORs are record claims — one false at the revision and one estimate called a
ceiling — and neither moves the selection. §0's second table carries them.

**Revision 2 was the second round.** Revision 1
(`6099aa7`, a stash commit on `ffc5c10`) was attacked by a fresh-context
DECISION-RED-TEAM (`matrix_P1_threat_state_REDTEAM.md`) and returned *"the
recommendation SURVIVES but the matrix FAILS (revise: M1, M2, M3)"*. Every
number reproduced — most from instruments the red team built and commands it
chose — and none of the three MAJOR findings moves the selection; all three are
holes in the RECORD. What changed is stated in §0 so a reader checks the fix
against the finding rather than re-reading two documents.

**The decision.** How `ThreatState` (`crates/pistol-solver/src/state.rs`) pays
for `apply` and `undo`. At `ffc5c10` every placed stone rewrites eighteen hashed
window records through `ThreatState::touch` (`state.rs:60-91`), and the search
calls `apply` and `undo` on every `Position::place` / `Position::undo`
(`crates/pistol-search/src/position.rs:108-117`, `:126-144`). Owed by the
optimization arc dispatch's P1 (`opt_arc_DISPATCH.md`, tranche 1; the audit's
A-03) and by D-254's own flip clause (§2.3).

**Nothing is selected in this document.** The selection record is written after
a fresh-context DECISION-RED-TEAM has attacked this revision.

**Every number is marked MEASURED or ESTIMATED (D-291)**, and every MEASURED
number names the artifact, its digest, and the revision it was taken at. The
red team's own receipts are exported to `artifacts/p1_rt_round1/` (digest list
`artifacts/p1_rt_round1_digests.txt`, sha256 `a63ee080…`) and are cited below
where they are the better instrument. The
instrument for every whole-engine number is `tools/bench_delta.sh` at
`ffc5c10` in PATH mode (both sides are binaries this session built and digested;
the script prints both digests in the artifact), 5 reps, over the live tree's
`configs/instrument_v0.toml` (`02e2e93b…`) and
`crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` (`931c50b1…`). The
script's own `VERDICT` lines are against D-220's `[1.4, 2.5]` / `1.15`
thresholds, which are not this package's numbers (its prereg registers its own);
they are quoted in the artifacts and read nowhere below.

---

## 0. What revision 1 got wrong, and what replaced it

| finding | what revision 1 said | what this revision does |
|---|---|---|
| **M1** — the counters artifact announces the instrument and does not contain it | *"the edit is printed in `artifacts/p1_counters_ffc5c10_v1.txt`"*; the file ended on the line announcing the patch | **The instrument is in the record twice.** `artifacts/p1_counters_ffc5c10_v2.txt` (sha256 `65a16dee…`) is v1 plus the session's edit script, RECONSTRUCTED after round 1 from the session's own record and verified by re-execution in round 2 — it applies to `position.rs` at `ffc5c10`, builds, and reproduces all three `P1COUNT` lines to the unit (`matrix_P1_threat_state_REDTEAM_round2.md` §1 M1; `/home/tom/pistol-wt/p1-patches/counter_patch_session.py`, `afcc2b97…`); and the red team's INDEPENDENT instrument (`p1_rt_round1/counter_patch.py`, `dd731212…`) reproduced every figure to the unit. §2.2 cites both. |
| **M2** — hand-written open addressing, the row D-254's own red team called *"the faster option on every instrument that reads the corpus correctly"*, was absent from a field that fires D-254's flip clause | no row | **O-I added** (§3), rejected on D-254's own MEASURED figure and on reading 1 |
| **M3** — §1.1's stated grep scope could not produce §1.1's table (which cited `tests/`), and `policy.rs:149-156` is `#[cfg(debug_assertions)]` | a two-crate `src/` scope; a debug-only pair listed as an engine site | **§1.1 re-derived repo-wide with `git grep` pinned to `ffc5c10`**, the command and its hit counts printed; the debug-only pair labelled |
| m1 — *"the zero-pending flushes are the root's own calls"* | asserted | **false and corrected**: 84 / 46 / 5 of 431 / 136 / 52 are root calls; the rest are re-visits of a flushed node after the null-window scan (`pvs.rs:561-579`) — MEASURED by the red team's split |
| m2 — readings 2–3 were derived across runs | ≈ 10 %, ≈ 5 %, ≈ 1.5 % from ratios of different runs | **MEASURED directly, one run each** (§4.1) |
| m3 — the 35.67 % includes query-side `masks` reads | called "maintenance" | called an UPPER bound on maintenance; the prereg's bracket ground says "at most" |
| m4 — O-G cited `window_map.rs`, which does not exist | a false path | the eval's map is `HandcraftedV0`'s field in `crates/pistol-eval/src/handcrafted.rs` (WP-1.9b landed it inline) |
| m5 — the straddling test named the boundary no game reaches | positions 63 / 64 | the three boundaries a game reaches first: −1 / 0, −64 / −65, 63 / 64 (§5) |
| m6 — no idle receipt, and a test link finished seconds before two benches | none | the record is four launch scripts and, for E0, a bench launched 7 s after a test link (round 2, R2-m2), so no idle claim is made for the session's runs; the red team re-measured all four idle within 0.7 % (§4.1) and that is what the table leans on; the prereg registers an idle receipt |
| m7 — §4.3's total | 50–60 KB | 62.4 KB from `MAX_PLY = 162` (§4.3) |
| m8 — O-A measures half of what the seam serves, and O-E's LIFO makes two caches behind one `Position` carry opposite undo contracts | not said | said, in O-A's row and in §5 |
| the axis | stated | the line where each unit is consumed is named (§2.4) |

### 0.1 What revision 2 got wrong, and what replaced it (round 2)

Round 2's receipts are exported to `artifacts/p1_rt_round2/` (digest list
`artifacts/p1_rt_round2_digests.txt`, sha256 `d5b695ee…`).

| finding | what revision 2 said | what this revision does |
|---|---|---|
| **R2-M1** — *"`dfpn.rs`, gated off in every committed config"* is false: three committed configs arm the solver and one is a `tools/determinism.sh` seat | the solver-on seat called uncommitted | **corrected and measured**: the three configs and the gate seat are named in §6; round 2's exploratory idle-receipted run of the same two binaries at the committed solver-on seat is recorded there (about 2 % early, about 0 % late); the package claims its bracket at the instrument seat only, and the prereg's identity leg gains the solver-on seat |
| **R2-M2** — §2.2's "ceiling" is exceeded by O-A in both bands when the counter runs per band | *"at that ceiling"*, *"ceiling met"* | the per-band estimate (1.94 % / 1.82 %) is stated as an ESTIMATE, O-A's measurement (2.8 / 2.1; 2.6 / 2.6 idle) is stated beside it as landing at and somewhat above, the discrepancy is recorded and not explained; the lever's size is unchanged |
| R2-m1 — *"verbatim"* for a script reconstructed two minutes after round 1's report | verbatim | *"reconstructed after round 1 and verified by re-execution"*, with the round-2 execution cited |
| R2-m2 — *"launched in one shell chain after the test suites had returned"* is contradicted by four launch scripts and E0's timing | asserted | deleted; the record is stated; the idle re-measurement is what the table leans on |
| R2-m3 — `reset_to` unwinds the eval *"FIFO"* | FIFO | in ascending `(q, r)` board order (`board.rs:87`), neither FIFO nor LIFO |
| R2-m4 — `quiescence.rs:234` is not a `search_nodes` site | `:234` | `:69` and `:260` |
| R2-m5 — O-I compared against O-E, on mixed axes | one line | compared against O-D1, the row it replaces, and stacked with the logged undo, on the nps axis |
| R2-m6 — the D-254 line-count comparison does no work | one clause | deleted (D-424's test) |
| R2-m7 — §4.3 says MEASURED and cites no artifact | no artifact | round 2's `size_of` receipt cited |
| R2-m8 — `is_empty()` has a caller | *"no caller"* for both | the caller named, `threat_oracle_tests.rs:230` |

---

## 1. The observable surface — derived from the code, two ways

Derived by a grep for the type's name and by the derive list on the struct,
because a grep for the field name alone misses the derived readers (the WP-1.9
matrix's own lesson, `matrix_wp19_storage.md` §1).

### 1.1 Writers — every `apply` / `undo` call site at `ffc5c10`, repo-wide

Derived at the revision, over every `.rs`, `.py` and `.sh` file in the
repository (`tools/sealbot/matchserver` included), with the command printed so
the scope is the reader's to check:

```
git grep -n '\.undo(\|\.apply(' ffc5c10 -- ':(glob)**/*.rs' ':(glob)**/*.py' ':(glob)**/*.sh' | wc -l
177
```

Of the 177 hits, the `ThreatState` ones (the rest are `Board`, `GameState`,
`Symmetry` and eval calls, filtered by reading each line) are: **ten `undo`
sites** — round 1 wrote "eleven" and revisions 2 and 3 transcribed it; the
verification pass counted the table (V-1) — and, beside them, apply-only
builders in tests and one example. The
undo sites, every one, with the order each undoes in:

| site | shape | undo order |
|---|---|---|
| `crates/pistol-search/src/position.rs:142` | one stone per `Position::undo`, popping `placed` (`:126-144`) | LIFO |
| `crates/pistol-solver/src/dfpn.rs:720`, `:722` (`undo_turn`) | a whole turn: applied `first` then `second` (`:706-708`), undone `second` then `first` | LIFO |
| `crates/pistol-solver/src/policy.rs:155`, `:156` | `raiser` then `cell` applied (`:149-150`), undone `cell` then `raiser` — **inside `#[cfg(debug_assertions)]` (`:147`), so absent from every release binary** | LIFO |
| `crates/pistol-solver/src/policy.rs:398`, `:399` (a test module) | `first` then `second` applied (`:395-396`), undone in reverse | LIFO |
| `crates/pistol-solver/tests/threat_oracle_tests.rs:222` | random playouts unwound by popping a recorded stack | LIFO |
| `threat_oracle_tests.rs:336`, `:344` | `undo` of a never-applied stone; `undo` with the wrong player | **must panic `THREAT_DESYNC`** |

Apply-only sites, for completeness: `solver.rs:254`, `:566`; `policy.rs:343`;
`heuristics.rs:273`; `position.rs:67`, `:113`; `examples/value_fixture_recall.rs:69`;
`search/tests/common/mod.rs:133`; `trigger_census_cover_tests.rs:60`;
`wp18b_solver_path_tests.rs:115`; `solver/tests/common/mod.rs:148`;
`defender_precondition_tests.rs:31`; `solver_oracle_tests.rs:528`, `:718`;
`threat_oracle_tests.rs:60`, `:219`, `:253`, `:257`, `:300`, `:327-328`,
`:335`, `:343`; `threat_query_tests.rs:741`; `wp15b_census.rs:244`, `:282`,
`:311`; `wp18b_m4_tests.rs:86`, `:140`, `:271`. No `.py` or `.sh` file calls
the type.

**No call site anywhere in the repository undoes out of order.** The public
doc of `undo` says *"the same stone back up"* and states no order; the two
`should_panic` tests pin what a wrong undo must do, not what a legal one may
be. Whether LIFO becomes the CONTRACT is a design question this matrix raises
in §5 and does not settle.

### 1.2 Readers

Every query answers out of the ten sorted sets or by a point read of one
window (`crate::state::ThreatState::masks`); nothing on a choice path
enumerates the table (D-254's structural licence for hashing).

| # | reader | site | shape |
|---|---|---|---|
| 1 | the nine set queries (`hot_windows` … `can_win_this_turn`) | `crates/pistol-solver/src/query.rs:127-218` | `&[Window]` from a sorted set; `masks(window)` point reads at `:221`, `:240-241`, `:259` |
| 2 | the three cover queries | `crates/pistol-solver/src/cover.rs:114`, `:153`, `:239` | sets plus `masks(window)` at `:247` |
| 3 | `masks(window)` | `state.rs:96` | one window's record |
| 4 | `table_snapshot()` | `state.rs:119`; sole caller `threat_oracle_tests.rs:103` | the whole table, sorted — oracles only |
| 5 | `window_count()` / `is_empty()` | `state.rs:106`, `:111` | `git grep -n 'window_count()' ffc5c10 -- crates tools` → the definition and `lib.rs:41` only; `git grep -n 'threats.is_empty()' ffc5c10 -- crates` → one caller, `threat_oracle_tests.rs:230`, the roundtrip oracle's fully-unwound assertion |
| 6 | derived `PartialEq` / `Eq` | `state.rs:27` | **a correctness surface**: `threat_oracle_tests.rs:223-228` asserts whole-state equality after every undo and against `ThreatState::new()` when fully unwound |
| 7 | derived `Debug`, `Clone` | `state.rs:27` | `Clone` on the oracle's recorded stack; `Debug` in assertion output only |

**The search reads the state at every node.** `pvs::visit` calls
`staged_context()` for every node that generates candidates
(`crates/pistol-search/src/pvs.rs:322`), and at the horizon `Run::quiescence`
runs the free win-now and overload checks through `gate_row()` →
`staged_context()` (`crates/pistol-search/src/quiescence.rs:148-161`) **even at
`q_depth_turns = 0`** (its own doc: *"`0` means the free checks … still run"*).
That is the fact §2.2 measures.

---

## 2. The premise, MEASURED at the package's own SHA

### 2.1 The re-profile (D-477: never inherited)

`perf record -F 2000` on `pistol-ffc5c10` (sha256 `78a7600a…`, built
`--release --locked` in a detached worktree at `ffc5c10`) at the audit's seat —
`configs/instrument_v0.toml`, `bench_positions_v1.txt` line 2 (15 stones),
`go nodes 400000` — `artifacts/p1_profile_ffc5c10_v1.txt` (sha256 `ed5f0cab…`),
2 074 samples:

| symbol | `ffc5c10` | audit at `d83ac01` (A-02, quoted) |
|---|---|---|
| `HandcraftedV0::delta` | 30.64 % | 31.77 % |
| `ThreatState::touch` | 14.04 % | 13.45 % |
| `WindowTable::masks` | 9.86 % | 9.55 % |
| `WindowTable::set` | 7.19 % | 7.98 % |
| `HandcraftedV0::apply` / `undo` | 6.97 % / 6.34 % | 6.29 % / 6.64 % |
| `WindowSets::transition` | 4.58 % | 5.14 % |
| **threat-state maintenance, summed** | **35.67 %** | 36.12 % |

The shape reproduces. The two hashed-table symbols alone are **17.05 %**, which
is the *"bench with `p > 0` naming window lookup as a measured hotspot"* D-254
deferred its recorded option to. **The 35.67 % is an UPPER bound on
maintenance** (m3): `masks` is also the point read the cover and cell queries
make (§1.2 rows 1–2), the profile has no call graph, and the two cannot be split
from this artifact; the bench measures the outcome and needs no split.

### 2.2 How much of it is paid for nothing — the counters

A scratch build at `ffc5c10` with counters in `Position` — the edit script,
reconstructed after round 1 and verified by re-execution in round 2, is
printed in `artifacts/p1_counters_ffc5c10_v2.txt` (sha256 `65a16dee…`; the v1
file, `f7e07c4f…`, holds the same output lines and announced the script
without containing it, M1); never committed — counted,
per seat, the `place` calls, the `staged_context` calls, the undos of a stone
that no `staged_context` had observed since it was placed, and how many stones
were pending at each `staged_context`. **The red team's own instrument
(`artifacts/p1_rt_round1/counter_patch.py`, `dd731212…`; outputs
`count_*.err` beside it), built and run independently, reproduced every figure
below to the unit:**

| seat | places | unqueried undos | share | flushes with 0 / 1 / ≥2 pending |
|---|---|---|---|---|
| 24 positions, `go nodes 50000` | 642 831 | 33 916 | **5.28 %** | 431 / 608 915 / **0** |
| 24 positions, `go depth_turns 2` | 44 166 | 3 780 | **8.56 %** | 136 / 40 386 / **0** |
| line 2, `go nodes 400000` | 240 811 | 16 650 | **6.91 %** | 52 / 224 161 / **0** |

The zero-pending flushes are the root's own calls before any stone is placed —
**84, 46 and 5 of them** — and, for the rest (347, 90, 47), re-visits of a node
already flushed: `Run::child`'s null-window scan followed by the full re-search
(`crates/pistol-search/src/pvs.rs:561-579`), each a `visit` that calls
`staged_context` again with nothing new placed (MEASURED by the red team's
split on `placed.is_empty()`, m1). **Two conclusions, and they are the reason
this matrix has the shape it has:**

- A "lazy" state that skips the stones no query reads saves both touches of
  each such stone, so an ESTIMATE of its yield is the unqueried share times
  the maintenance share. Per band, because the bench reports per band (round
  2 ran the counter binary per band, `artifacts/p1_rt_round2/count_early.err`
  `ff6eb970…` and `count_late.err` `4b6176b0…`): **5.43 % early and 5.10 %
  late** of at most 35.67 % ≈ **1.94 % / 1.82 % of wall**. O-A MEASURES
  **2.8 % / 2.1 %** (§4.1) and **2.6 % / 2.6 %** in round 1's idle re-run — at
  and somewhat above the estimate, by 0.3–0.9 points, over nps IQRs of
  0.16–0.63 % of their medians (`artifacts/p1_mx_bench_A_v1.txt:32,36` and
  `p1_rt_round1/bench_A.txt:30,34`, IQR ÷ median; R3-m4). **The estimate is therefore not a ceiling and is
  not called one** (R2-M2): either the maintenance share at the bench's seats
  is above the 15-stone profile seat's, or deferring the apply to the query
  saves something the two-touch count does not see. Either way the
  count-of-touches lever is bounded near 3 %, and the discrepancy is recorded
  rather than explained. The dispatch's
  *"paid on every place/undo regardless of whether the node queries"* is true
  of the mechanism and nearly empty of yield, because §1.2's last paragraph
  holds: leaves read the state too.
- A "batched" update that merges several pending stones' window touches has
  nothing to merge: **at 0 of 874 081 flushes** across the three seats were two
  stones pending (873 462 had exactly one, 619 had none). Its yield at these seats is exactly zero — MEASURED by the
  histogram, so no prototype was built.

### 2.3 D-254's flip clause, quoted

> AN OPTION NOBODY CONSIDERED IS RECORDED AND NOT PURSUED: per-axis line
> bitboards, where a stone touches 3 lines rather than 18 windows and window
> masks are recovered by shift and popcount … Flips when a bench with `p > 0`
> names window lookup as a measured hotspot, which is WP-1.5b's earliest
> opportunity; the replacement goes behind this same API and is a rule-5 change.

§2.1 is that bench. The option is O-E below, and it was prototyped and measured
rather than argued about, because D-254's own red team recorded that *"every
positive performance ground this line states is refuted by the session's own
second harness"* and WP-1.9's matrix found its structural argument for a dense
store falsified by measurement (`matrix_wp19_storage.md` §4.1, §4.3).

### 2.4 The axis, and where its unit is consumed (D-477)

The axis is *the cost of a touch* against *the count of touches*, and the unit
both arms share is the STONE. A touch is one `ThreatState::apply` / `undo`
(`state.rs:46-59`), consumed at `state.rs:62` — `for (window, index) in
windows_through_indexed(at)` — where one stone becomes eighteen window records.
The count is consumed at `position.rs:113` and `:142`, once per
`Position::place` / `undo`, which is what §2.2's counters count. The bench's
unit is the NODE (`search_nodes` at `pvs.rs:220` and `quiescence.rs:69`, `:260`;
a leaf stone is counted twice, `visit` then `quiescence`), but the script's
per-position node-identity assertion fixes the stones-per-node ratio between
the two sides, so every ratio in §4.1 is a ratio of per-stone costs.

---

## 3. The field

Each option is a MEASUREMENT REVISION on a local branch of the measurement
worktree, committed only so its number names a revision (`bench_delta.sh`'s own
discipline); none is a landing candidate as committed — no tests, no docs, no
rule-9 split, no ADR.

- **O-0 INCUMBENT** — `touch`: per window, `masks` (probe 1), the asserts,
  `ClassSet::of` twice per side, transitions, `set` (probe 2). `ffc5c10`.
- **O-A LAZY AT THE SEAM** — `Position` keeps `threats_synced`, the count of
  `placed` the state has been told about; `place` no longer calls `apply`;
  `staged_context()` applies the pending suffix before handing the state out;
  `undo` calls the state's `undo` only for a stone the state was told about.
  The state itself is untouched. `p1/mx-A` = `cc4153b`, 14 (+) / 4 (−) lines.
  **It defers half of what the seam serves** (m8): at a TT-cut child
  (`pvs.rs:263-274`) or a rule-4 win (`:434-444`) nothing reads the eval
  either, so the eval's `apply` / `undo` (13.31 % in §2.1) is wasted on the
  same 5.3–8.6 % of stones — ESTIMATED 0.7–1.1 % of wall more, behind three
  doors (`value`, `static_score_after`, `staged_context`) rather than one.
  Recorded; not prototyped; not part of any recommendation here.
- **O-B LAZY INSIDE THE TYPE** — the same pending stack inside `ThreatState`,
  flushed by every query through interior mutability. **Not prototyped**: its
  yield is bounded by the same unqueried share as O-A (§2.2) and it adds a
  borrow-flag check to every query, so it cannot exceed O-A; the solver's own
  callers (§1.1) query immediately after every `apply_turn`, so it has no
  second yield there either. ESTIMATED ≤ O-A.
- **O-C BATCHED AT QUERY** — merge the pending stones' window touches so a
  window two pending stones share is rewritten once. MEASURED at zero yield by
  §2.2's histogram; not prototyped.
- **O-D1 SINGLE PROBE** — `WindowTable::update(window, f)` through the `entry`
  API: one probe per window instead of `masks` then `set`. `p1/mx-D1` =
  `5f41dc9`.
- **O-D2 SINGLE PROBE + LOGGED UNDO** — O-D1, plus an undo log: `apply` pushes
  per touched window the record before and the class transitions it caused;
  `undo` pops its stone's frame, reverses the transitions without recomputing
  a class, and restores each record with one probe. `undo` of a stone that is
  not the last applied panics `THREAT_DESYNC`. `PartialEq` is written by hand
  over table and sets, excluding the log. `p1/mx-D2` = `1c23954`.
- **O-E0 LINE BITBOARDS, EAGER UNDO** — D-254's option. The table becomes
  per-axis line bitboards: key `(axis, line, chunk)`, value two `u64` (a bit
  per cell per side over 64 consecutive positions of one line). A stone flips
  one bit on each of its three lines (three probes); the eighteen windows'
  before/after masks are shifted out of an 11-bit run read around the stone;
  class transitions as before. `masks(window)` reads one or two chunks and
  shifts. `undo` recomputes the same way with the bit cleared. `p1/mx-E0` =
  `eecf268`.
- **O-E LINE BITBOARDS + LOGGED UNDO** — O-E0 with O-D2's log: per stone
  three `(key, chunk before)` and per window `(window, was, now)`; `undo`
  reverses the transitions and restores three chunks. `p1/mx-E` = `4298ecd`.
- **O-EA** — O-E plus O-A, the two being orthogonal (one is the state's cost
  per touch, the other the count of touches). `p1/mx-EA` = `e78a094`.
- **O-F DENSE / PAGED DIRECT ADDRESSING** — a `Vec` over a bounding box or
  fixed pages, no hashing. **Not prototyped, on a measured prior**: WP-1.9
  implemented exactly this for the eval's window map and measured it SLOWER
  than the hashed store (1.737 / 1.837 against 1.783 / 1.909,
  `matrix_wp19_storage.md` §4.1) while failing the lattice-edge test class
  (§5 there: `O(R²)` in the played radius on a lattice rule 1 declares
  unbounded). Both grounds transfer unchanged to a map with the same
  eighteen-windows-per-stone access pattern. ESTIMATED ≤ O-D1.
- **O-G ONE WINDOW STORE SHARED WITH THE EVAL** — `HandcraftedV0` maintains its
  own hashed per-window counts over the same eighteen windows per stone (the
  `windows` field of `crates/pistol-eval/src/handcrafted.rs`, the store WP-1.9b
  landed inline; `apply` + `undo` are 13.31 % in §2.1). One store feeding both would pay one traversal instead of two.
  **Not prototyped**: it crosses the `Eval` seam (a Stage-2 backend replaces
  `HandcraftedV0` and has no window map), makes `pistol-solver` depend on
  `pistol-eval` or the reverse, and is a package of its own if it is ever one.
  ESTIMATED upper bound: the eval's 13.31 %; recorded so the next arc does not
  rediscover it.
- **O-I HAND-WRITTEN OPEN ADDRESSING** over the packed window key — D-254's
  ~90-line alternative, which its own DECISION-RED-TEAM found *"ahead by
  4.0–4.8 % with a replication spread under 1 %"* on the state's cost and
  *"the faster option on every instrument that reads the corpus correctly"*
  (D-254, quoted). **Not prototyped, and rejected on that MEASURED figure**:
  4.0–4.8 % of at most 35.67 % is ESTIMATED **1.4–1.7 % of wall**. Set against
  the row it would replace — O-D1, the hashed table at one probe per window,
  1.016 / 1.017 — it is a hashed table one to two points faster; stacked with
  O-D2's logged undo it is ESTIMATED **1.144–1.147 early and 1.129–1.132 late**
  (`1.128 × 1.014–1.017`, `1.113 × 1.014–1.017`) against O-E's measured 1.257 /
  1.240 on the same axis, eleven points under in both bands (R2-m5, R3-m3). And it keeps the
  mechanism O-E removes — eighteen records per stone entering and leaving a
  table (reading 1). It is D-254's
  standing flip, and this is the bench that answers it: the faster hash is not
  where the time went.
- **O-H NO-CHANGE** — the null. The re-recording every landing forces (every
  receipt names a binary digest, D-577's costed limb) against the gain.

---

## 4. The matrix

### 4.1 Whole-engine, all MEASURED, candidate / baseline, larger is better

Baseline is `pistol-ffc5c10` (`78a7600a…`) in every run. Node identity held
per position, both budgets, all reps, in every run (the script's own gate).

| | **O-A** lazy seam | **O-D1** single probe | **O-D2** + logged undo | **O-E0** bitboards, eager undo | **O-E** bitboards + logged undo | **O-EA** |
|---|---|---|---|---|---|---|
| revision | `cc4153b` | `5f41dc9` | `1c23954` | `eecf268` | `4298ecd` | `e78a094` |
| binary sha256 | `ac4988dc…` | `3a5717f9…` | `458724ad…` | `ba532dcf…` | `f333d101…` | `8b33fff2…` |
| **nps ratio, early / late** | **1.028 / 1.021** | **1.016 / 1.017** | **1.128 / 1.113** | **1.190 / 1.188** | **1.257 / 1.240** | **1.273 / 1.259** |
| time-to-depth-2, early / late | 1.026 / 1.035 | 1.000 / 1.009 | 1.113 / 1.104 | 1.176 / 1.182 | 1.212 / 1.184 | 1.197 / 1.245 |
| baseline nps median, early / late | 446 671 / 393 967 | 448 668 / 395 519 | 447 002 / 394 587 | 448 668 / 394 897 | 446 340 / 393 350 | 448 000 / 393 967 |
| candidate nps median, early / late | 459 277 / 402 175 | 455 800 / 402 175 | 504 281 / 439 120 | 533 787 / 469 079 | 561 148 / 487 769 | 570 182 / 495 963 |
| largest nps IQR, % of its median | 0.45 % | 0.81 % | 0.50 % | 0.74 % | 0.78 % | 0.66 % |
| artifact `artifacts/p1_mx_bench_*_v1.txt` | `87a081a5…` | `851478e9…` | `c0b11f4b…` | `8697d659…` | `d0b1b384…` | `4bc15f66…` |

**Five readings, stated plainly.**

1. **The lever is the cost of a touch, not its count.** O-A's 2.8 % / 2.1 %
   lands at and a little above §2.2's per-band estimate; O-D1's 1.6 % says the second probe of a key just probed
   is nearly free, so the hashing cost is per DISTINCT window and per
   insert/remove churn, which is what O-E removes (three in-place bit flips per
   stone instead of eighteen records entering and leaving the table).
2. **The logged undo is worth 1.111 / 1.091 on the hashed table and 1.053 /
   1.043 on the bitboards** — MEASURED directly, one run each, by the red team
   (`p1_rt_round1/bench_D1_to_D2.txt` `903ba0cd…`; `bench_E0_to_E.txt`
   `9df4bd58…`; both with node identity held and exit 0) — because the undo
   half of `touch` stops enumerating windows and recomputing four `ClassSet`s
   per window.
3. **O-E is the largest measured gain and O-A adds 1.018 / 1.014 on top of
   it** (MEASURED directly, `bench_E_to_EA.txt` `bd2a231e…`), the two being
   orthogonal as §3 says.
4. **Time-to-depth tracks nps** within the script's printed deviation in every
   run, as it must for a search-identical change (the harness's own header).
   Its medians are 66–117 ms with IQRs of at most 2 ms, so the ms quantization
   the header warns of is the whole of the deviation.
5. **The four whole-engine numbers that matter reproduce on an idle box**: the
   red team rebuilt every branch in its own worktree, obtained all seven binary
   digests bit-for-bit, and re-benched A, E0, E and EA with `pgrep` empty
   before each run — 1.026 / 1.026, 1.183 / 1.181, 1.256 / 1.241, 1.267 /
   1.251 (`bench_A.txt` `c893a81e…`, `bench_E0.txt` `12e71776…`, `bench_E.txt`
   `e3eaed80…`, `bench_EA.txt` `d142137d…`), within 0.7 % of the table. The
   session's own runs carried no idle receipt (m6), and the measurement
   worktree's timestamps put a test link 26 s and 7 s before the E and E0
   benches, which is why the re-measurement is cited beside them.

### 4.2 Code cost — MEASURED at the measurement revisions

`git diff --shortstat ffc5c10 <rev> -- crates`, and `wc -l` of the touched
files:

| | O-0 | O-A | O-D1 | O-D2 | O-E0 | O-E | O-EA |
|---|---|---|---|---|---|---|---|
| diff | — | +14 / −4 | +47 / −16 | +106 / −34 | +255 / −110 | +276 / −111 | +290 / −115 |
| `state.rs` lines | 122 | 122 | 122 | 163 | 173 | 193 | 193 |
| `table.rs` lines | 214 | 214 | 245 | 245 | **308** | **308** | **308** |
| `position.rs` lines | 165 | 175 | 165 | 165 | 165 | 165 | 175 |

**Every bitboard row puts `table.rs` over rule 9's ~300 soft cap**, and there
is no entry for it in `docs/rule9_justifications.md` (`/usr/bin/grep -c
'pistol-solver/src/table.rs' docs/rule9_justifications.md` → 0). WP-1.9's
matrix met the same finding and the remedy is the same: the line store (chunk,
line position, key packing, hasher) is not the window-mask type or the
`empty_cells` iterator, and the design owes the split. A hasher and a key
packing are not the state.

### 4.3 Memory — the log, MEASURED by `size_of` at `4298ecd`

`Touched` **10 B**, `Applied` **6 B**, `(u64, Chunk)` **24 B** — round 2's
receipt `artifacts/p1_rt_round2/sizeof.log` (sha256 `4024bb86…`), a `#[test]`
printing `size_of` at `4298ecd`; the session's own probe printed the same
figures and kept no artifact (R2-m7). Per applied
stone: `18 × 10 + 3 × 24 + 6 = 258 B`, held only while the stone is on the
board. The log is bounded by the stones applied and not undone — the root's
stones plus the search path — so an 80-stone position searched to `MAX_PLY =
2 × 64 + 2 + 32 = 162` (`crates/pistol-search/src/search.rs:24`, `:35`, `:41`)
holds `(80 + 162) × 258 B` = **62.4 KB** of frames, and the three `Vec`s'
doubling can hold twice that — against a 268 MB transposition table. O-D2's
log is the same shape less the chunk frames.

### 4.4 What each row changes about the type

| | O-A | O-D2 | O-E0 | O-E / O-EA |
|---|---|---|---|---|
| `undo` order contract | none | **LIFO or panic** | none | **LIFO or panic** |
| `PartialEq` | derived | hand-written, excludes the log | derived | hand-written, excludes the log |
| `masks(window)` | 1 probe | 1 probe | 1–2 probes + shifts | 1–2 probes + shifts |
| `window_count()` | O(1) | O(1) | O(table) | O(table) |
| `table_snapshot()` | enumerates records | enumerates records | enumerates chunk bits, 18 window candidates per set bit, filtered by axis | same |
| new reader of `Position`'s stones | `staged_context` is the flush | — | — | as O-A |

---

## 5. Failure modes, per option — what the design must state and the red team should attack

- **O-A / O-EA.** The state is correct only when read through
  `staged_context()`, which is the only accessor of `Position::threats`
  (`/usr/bin/grep -n 'self.threats\|threats' crates/pistol-search/src/position.rs`
  lists the field, `new`, `reset_to`, `place`, `undo`, `staged_context` and
  nothing else at `ffc5c10`). A future accessor that hands out `&threats`
  without flushing reads a stale state and the search is wrong silently. The
  design's answer has to be structural — the field private, the flush inside
  the one accessor, a test that places, does not query, and asserts the state
  after a query equals a freshly built one.
- **O-D2 / O-E.** *The LIFO contract.* A non-LIFO `undo` cannot be served from
  the log — restoring the last frame's "before" would re-place a stone undone
  out of order — so it is refused by name. Every engine call site is LIFO
  (§1.1) and the oracle's `should_panic` tests still pass (a never-applied
  stone and a wrong player are both "not the last applied"), but the public
  doc gets narrower and the design must say so in the `# Panics` section, with
  a test pinning an out-of-order undo. *The equality surface.* A derived
  `PartialEq` would fold the log into state equality, so two states over the
  same stones built by different paths would compare unequal; the hand-written
  one compares table and sets, and the oracle's whole-state assertions are the
  test that keeps it honest. *Frame integrity.* A frame's `touched` count and
  three chunk entries are popped by count; a mutant that pushes one fewer entry
  must die at the roundtrip oracle.
- **O-E0 / O-E.** *Straddling.* A window whose six cells cross a 64-cell chunk
  boundary is read from two chunks; the run reader shifts `high << (64 −
  offset)` and the case `offset = 0` never reaches it (`offset + count > 64` is
  false there). The origin sits at position 0 of every line, so the boundary every
  game crosses on its first turns is **−1 / 0** (`rem_euclid`'s wrap), the
  next **−64 / −65**, and **63 / 64** is one no fixture reaches; the oracle's
  playouts from the origin exercise −1 / 0 constantly and the other two never
  (m5). The design owes a test at all three, reading every window across each
  against the reference. *The lattice edge.*
  Windows that run off the addressable lattice are still enumerated by
  `Coord::checked_step` / `Window::new`, exactly as `windows_through_indexed`
  enumerates them, so the set of windows the state maintains is unchanged; the
  run reader's chunk indices at the edge (`−513`, `512`) are within the packed
  key's biased fields (line `[−65536, 65534]` from `q + r` over `i16`, chunk
  `[−512, 511]` from `pos.div_euclid(64)`, both stated in `pack`'s doc) and
  simply never hold an entry. The eval's lattice-edge test class
  (`eval_windows_stop_at_the_edge_of_the_addressable_lattice`) has no threat
  counterpart at `ffc5c10` and the design owes one. *Pruning.* A chunk is
  removed the moment both sides' bits are zero, so an unwound state equals a
  fresh one — D-62's rule at the chunk rather than the window; a mutant that
  keeps an empty chunk must die at the roundtrip oracle's `ThreatState::new()`
  comparison. *`masks(window)` got dearer.* Every point read now costs one or
  two probes and shifts instead of one probe; the readers are the cover and
  cell queries over the hot and live sets (§1.2 rows 1–2), which are small,
  and the whole-engine number already contains that cost. *The hasher.*
  D-254's collision argument was about a multiply-only mix over a key whose
  low sixteen bits are `r`; the chunk key's low sixteen bits are the chunk
  index (a handful of distinct values) and the line sits above them, and
  SplitMix64 mixes every input bit, so the argument transfers — ESTIMATED, and
  the bench number is the measurement of the outcome rather than of the bucket
  distribution.
- **O-D2 / O-E, and the seam above them** (m8). D-61 makes the eval's unwind
  order-free — `Position::reset_to` (`position.rs:48-59`) unwinds it in ascending `(q, r)` board order (`board.rs:87`), neither FIFO nor LIFO — while
  the logged undo makes the threat state's unwind LIFO-or-panic: two caches
  behind one `Position` with opposite undo contracts. `reset_to`'s replacement
  of the state by `ThreatState::new()` (`:60-62`) therefore stops being an O(1)
  convenience and becomes the only correct spelling, and the design must say
  so where that line is.
- **O-D1.** Nothing new; it is inside O-D2 and O-E.
- **O-H.** The 35.67 % stays. The cost of doing nothing is stated once, here.

---

## 6. Recommendation, and the attacks it invites

**O-E — per-axis line bitboards with the logged undo — as P1's mechanism**, with
`table.rs` split as §4.2 requires, the LIFO contract and hand-written equality
stated in the design and pinned by tests, and the three boundary tests §5 names
(straddling, lattice edge, out-of-order undo).

**O-E over O-E0, and the ground is stated because the red team asked for it.**
The logged undo is worth 1.053 / 1.043 measured directly — a twentieth of the
engine's speed — and it is bought with two contract changes: `undo` becomes
LIFO-or-panic, and equality is written by hand to exclude the log. Against
that: no caller anywhere in the repository undoes out of order (§1.1, repo-wide
at the revision), the solver's own `apply_turn` / `undo_turn` is LIFO by
construction, both changes are pinned by tests the design names, and the
refusal is a `THREAT_DESYNC` panic of the class the type already raises for a
stone it does not hold. A twentieth of the engine for two pinned invariants is
this project's ordinary trade, and the matrix takes it. A reader who weighs it
differently has O-E0 one row up, measured.

**O-A lands as a SECOND, separately benched commit inside the package or not at
all.** Rule 5 is *one change = one commit = one bench*, and O-A is a different
mechanism (the count of touches, in `pistol-search`) from O-E (the cost of a
touch, in `pistol-solver`); O-EA is measured here so the sum is known, not so
that it is landed as one diff. Whether a ≈ 1.5 % gain is worth its own bench,
review surface and the structural risk in §5 is the design's call, and the
matrix's own reading is that it is marginal: the flush-only-through-one-door
invariant is the kind that holds until someone adds a reader.

**The strongest attacks the author can see, listed so the red team starts
there rather than rediscovering them:**

1. *The matrix measured at one seat, and the other seat is committed.* Every
   bench number is `configs/instrument_v0.toml` over `bench_positions_v1.txt`.
   The solver's own use of the state (`dfpn.rs`) is NOT "gated off in every
   committed config", as revision 2 said (R2-M1): `git grep -n
   'on_search_path = true' ffc5c10 -- configs` → `bench_wp18c_solver_on.toml`,
   `gate_staged_solver_v0.toml`, `play_staged_solver_v0.toml`, and
   `gate_staged_solver_v0` is a `tools/determinism.sh` seat (`:76`) and the
   armed seat of two census tests. **Round 2 measured that seat once**,
   exploratory — one hand-rolled instrument, two interleaved reps, idle
   receipt printed, per-position node identity held, 0 error lines
   (`artifacts/p1_rt_round2/solveron_run.log` `61baa09e…`, totals beside it):
   `pistol-ffc5c10` (`78a7600a…`) against `pistol-E` (`f333d101…`) under
   `configs/bench_wp18c_solver_on.toml` (`8414509a…`) at `go nodes 50000`,
   24 positions — **time ratio 1.022 / 1.015 early and 1.005 / 1.001 late**.
   The wall there is the solver's (about 13 k nps against about 450 k), so
   O-E's 24–26 % is the instrument seat's node mix and not a property the
   touch carries to every caller. **What the package claims**: the prereg's
   bracket at the instrument seat; byte-identity at three seats, the solver-on
   one included; nothing about the solver's throughput. **What it does not
   claim**: any gain where the solver is armed. The solver's oracle suites are
   among the 246 tests green on E, E0 and EA, and gates 12 and 13 run at the
   landing.
2. *O-E's snapshot is not the incumbent's snapshot by construction.* Round 1
   found this weaker than stated — `compare` in `threat_oracle_tests.rs:98-200`
   reads both doors at every ply, the snapshot against the reference table and
   every query (through `masks`) against the reference's answers — and
   narrowed it to m5: the playouts reach neither a positive chunk boundary nor
   the lattice edge, so those two have no falsifier today and the design owes
   them.
3. *The log's LIFO refusal narrows a public API on the strength of a grep.*
   Closed at the repo-wide scope in §1.1; the stronger form is m8's two
   caches with opposite contracts, carried into §5.
4. *`bench_delta.sh` in PATH mode measured binaries this session built by hand
   rather than revisions the script built itself.* Closed by instrument in
   round 1: independent builds of `ffc5c10` and all six branches in another
   worktree reproduced all seven digests bit-for-bit. The landing bench in
   `rev:` mode ties them a third way.

**The strongest surviving attack, carried verbatim from round 1's report** and
answered only where this revision answers it: *"O-E is the largest gain among
the rows that were measured, at one seat, and the field it won never held the
row D-254 itself recorded as 'the faster option on every instrument that reads
the corpus correctly' — hand-written open addressing over the same key — which
is dismissed here only after the fact, on D-254's own 4.0–4.8 % of the state's
cost against O-E's measured 24–26 % of wall; the half of O-E that narrows a
public `undo` to LIFO-or-panic and replaces a derived `PartialEq` with a
hand-written one — the logged undo — buys 5.3 % early and 4.3 % late measured
directly against O-E0 in one run, so a design that judged those two contract
changes too dear could take O-E0 at 18 % and forgo a twentieth of the engine's
speed, and nothing in this matrix says which the project prefers; the premise's
three percentages, though reproduced to the unit by a second instrument, were
produced by one whose source the cited artifact announces and does not contain;
and every number here was taken at the seat where the solver is gated off,
while the solver is the one other caller of the contract being narrowed."* This
revision holds O-I in the field, states the O-E-over-O-E0 ground, and puts the
instrument in the record; the one-seat limb stands as a limit of the
measurement, and round 2 sized it.

**And round 2's, carried verbatim**, answered only where this revision answers
it: *"O-E's 24–26 % is the instrument seat's number and only that seat's: at
the committed, CI-gated solver-on seat — the one the matrix wrongly calls
uncommitted, and the one whose caller drives the contract being narrowed
hardest — the same two binaries are about two per cent apart, so the gain the
package banks is a property of one node mix and the LIFO-or-panic narrowing and
hand-written equality are bought, at the solver's seat, for nothing measurable;
the premise that licensed the matrix's shape is not tight either, since O-A's
measured gain exceeds the 'ceiling' the unqueried share puts on it in both
bands, which says the 35.67 % is one seat's share and not a bound; the one
alternative D-254 called faster is still rejected on a figure from another
instrument at another revision, unprototyped, and compared against O-E rather
than against the row it would actually replace; and the chunk-boundary and
lattice-edge behaviour of the store that wins has no falsifier until the
design's tests exist."* This revision names the solver-on seat as committed and
records its number, calls the estimate an estimate, compares O-I against O-D1,
and leaves the last limb to the design's I5 and I6, where it belongs.
