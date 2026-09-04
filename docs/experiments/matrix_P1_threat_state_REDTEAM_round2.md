# MATRIX P1 — DECISION-RED-TEAM, round 2

**Target.** `docs/experiments/matrix_P1_threat_state.md` (revision 2) at the
named revision `40b69e390b8fb01d5f2438a9eaddf01182a3d1f0` — a `git stash create`
commit on `dev` = `ffc5c10f4d16356f574e3221a023f78399d2e3bb` (parents
`ffc5c10`, `52ef6bc`). Revision 1 was `6099aa74a03f8a66cdfcfdd2c1b961de9c297fc5`.

**Does that revision still match the working tree's bytes?** Yes — byte-identical
(sha256 `3eb1059d0edca509…` for `git show 40b69e39…:docs/experiments/matrix_P1_threat_state.md`
and for the untracked file; the round-1 report and the two drafts also match
their stash copies). **HEAD** at review time: `ffc5c10f4d16356f574e3221a023f78399d2e3bb`.
**Date:** 2026-09-04.

**Verdict, stated first.** VERDICT: the recommendation SURVIVES but the matrix
FAILS (revise: R2-M1, R2-M2). Every remedy revision 2 claims for M1–M3 and
m1–m8 was executed and holds — the counters instrument now in the record
rebuilds and reproduces all three seats to the unit from sessions I generated
myself; the O-I row's quotations are D-254's and its arithmetic is right; §1.1
reproduces at three scopes of my own choosing. Every load-bearing number
re-derives: the profile from the session's raw `perf.data`, the six benches and
the round-1 receipts from their artifacts with my own parser, all thirty-six
round-1 digests by `sha256sum -c`, the diff stats and line counts from the
branches. What fails is two claims revision 2 makes at a convenient scope: the
seat it says is uncommitted is a committed, CI-gated one — and at that seat the
same two binaries differ by about two per cent, not twenty-five — and the
"ceiling" it says O-A meets is one O-A exceeds in both bands when the counter
is run per band. Neither moves the selection; both are the defect class
`docs/process.md`'s re-derivation clause names.

**Environment.** Everything adjudicated was run with `git grep` pinned to
`ffc5c10` or `/usr/bin/grep`, sorted `LC_ALL=C`. Builds and runs happened in
`/home/tom/pistol-wt/redteam-p1-r2` (`git worktree add --detach … ffc5c10` on
`/home`, `CARGO_TARGET_DIR=$WT/target-rt2`), removed at the end with no branch
left behind; nothing in the live tree was built or modified. The one timing run
was preceded by `pgrep -af 'cargo|rustc|bench_delta|pistol'` printing nothing
but the harness's own shell (quoted in §N-1), and my `size_of` build ran only
after it had finished. Receipts are in this session's scratchpad (`…/scratchpad/rt2/`);
every load-bearing line is quoted here so the report stands without them.

---

## 1. Remedy verification — each executed, not read (D-591)

| finding | remedy claimed (§0) | how I executed it | holds? |
|---|---|---|---|
| **M1** | `artifacts/p1_counters_ffc5c10_v2.txt` (`65a16dee…`) holds the session's edit script verbatim; the red team's independent instrument reproduced every figure | `sha256sum` → `65a16dee49f6…` (v2), `f7e07c4fd3ae…` (v1), `afcc2b97cb8c…` (`p1-patches/counter_patch_session.py`) — all three as cited. Extracted the 37 lines after the marker `# counter patch (python edit script) follows` (sha256 `eabf28898b52…`), ran it against `crates/pistol-search/src/position.rs` at `ffc5c10` in my worktree: `1 file changed, 34 insertions(+)`, seven `fetch_add` sites, so all eight `str.replace` targets hit (a miss would have left the count short). `cargo build --release --locked -p pistol-cli` from an empty target dir: 21 `Compiling` lines, `BUILD_EXIT=0`, binary sha256 `5ae44a253cf8…`. Sessions built by me from the fixture in `bench_delta.sh`'s shape (`newgame / position <entry sans comment> / go <budget>`, `quit`) — `cmp` says byte-identical to the round-1 red team's `session.{nodes,depth_turns,line2}`. Three runs, exit 0, zero `error` lines: `P1COUNT places 642831 … unqueried_undos 33916 … pending_hist [431, 608915, 0, …]`, `44166 … 3780 … [136, 40386, 0, …]`, `240811 … 16650 … [52, 224161, 0, …]` — identical to the artifact's three lines to the unit. | **yes** (with R2-m1 on the word "verbatim") |
| **M2** | O-I row added, rejected on D-254's own MEASURED figure and reading 1 | `/usr/bin/grep -c -F` in `docs/decisions.md`: `ahead by 4.0–4.8 % with a replication spread under 1 %` → 1 (line 549); `the faster option on every instrument that reads the corpus correctly` → 1 (line 549); both inside D-254's carried red-team paragraph, which attributes the figure to the corrected `statebench` — a whole-state apply/undo instrument, so "on the state's cost" is a fair paraphrase. Arithmetic: 0.040 × 35.67 = 1.427, 0.048 × 35.67 = 1.712 → "1.4–1.7 %" ✓; against O-E's 1.257 / 1.240: 1.43/25.7 = 1/18, 1.71/24.0 = 1/14 → "a fifteenth" ✓. | **yes** (with R2-m5 on the comparator) |
| **M3** | §1.1 re-derived repo-wide with `git grep` pinned to `ffc5c10`, hit count printed; the debug-only pair labelled | My scopes, none the document's: (i) whole tree, every path, `git grep -n -E '\.(undo\|apply)\(' ffc5c10` → **189** = 177 `.rs` + 12 `.md` (so the document's 177 is the `.rs` population and no `.py`/`.sh` hit exists at all); (ii) `\b(undo\|apply)\(` whole tree → 216 (adds free-function and definition sites); (iii) every `.undo(` site printed with its receiver (60 lines): the `ThreatState` receivers are exactly `position.rs:142`, `dfpn.rs:720,722`, `policy.rs:155,156,398,399`, `threat_oracle_tests.rs:222,336,344` — eleven; `reference_walk.rs`'s `self.undo` (4 hits) wraps `GameState` + eval, `pistol_client.rs:188` is `GameState`. `sed -n 147p` at `ffc5c10` → `#[cfg(debug_assertions)]`; `#[cfg(test)] mod tests` at `policy.rs:304`, so `:398/:399` are test-module as labelled. The apply-only list (30 sites) matches my own `.apply(` grep filtered to threat receivers exactly. `tools/determinism.sh:50` mentions the type in a comment only. | **yes** |
| m1 | 84 / 46 / 5 of 431 / 136 / 52 are root calls, the rest re-visits | `sha256sum -c artifacts/p1_rt_round1_digests.txt` → 36 `OK`; the list itself digests `a63ee080330a…` as the header cites. `count_{nodes,depth_turns,line2}.err`: `root0 84 revisit0 347`, `root0 46 revisit0 90`, `root0 5 revisit0 47`; 84+347 = 431, 46+90 = 136, 5+47 = 52 ✓. `pvs.rs:561-579` at `ffc5c10` is the null-window scan and `full(self)` re-search ✓. | **yes** |
| m2 | readings 2–3 MEASURED directly, one run each | `bench_E0_to_E.txt` (`9df4bd58…`) `1.053 / 1.043`; `bench_E_to_EA.txt` (`bd2a231e…`) `1.018 / 1.014`; `bench_D1_to_D2.txt` (`903ba0cd…`) `1.111 / 1.091`; each carries `node identity holds` and `BENCH_EXIT=0`; digests verified above. | **yes** |
| m3 | 35.67 % called an upper bound; the prereg says "at most" | §2.1 text; `p1_bench_prereg.md` §1.1: *"AT MOST 35.67 % of wall"*. | **yes** |
| m4 | the eval's map is `HandcraftedV0`'s field in `handcrafted.rs` | `git grep -n` at `ffc5c10`: `handcrafted.rs:152: windows: WindowMap,` inside `pub struct HandcraftedV0` (`:147`). | **yes** |
| m5 | the three boundaries −1/0, −64/−65, 63/64 | §5 text; `table.rs` at `4298ecd`: `from.rem_euclid(CHUNK_LEN)` (`:214`), `if offset + count > 64` (`:218`), `high.p1 << (64 - offset)` (`:220`) — the guard and shift as described. The design's I5 names the same three. | **yes** |
| m6 | the red team re-measured all four idle within 0.7 %; the prereg registers an idle receipt | `phase2.log`: `busy=[]` before each of A, E0, E, EA; `phase3.log`: `busy=[]` before the three pairs. Re-measured against the table (my recomputation): A 1.028→1.026 / 1.021→1.026, E0 1.190→1.183 / 1.188→1.181, E 1.257→1.256 / 1.240→1.241, EA 1.273→1.267 / 1.259→1.251 — largest gap 0.6 %. Prereg §1.3 registers `pgrep` into the artifact. | **yes** for the re-measurement; the NEW sentence about the session's own runs does not — R2-m2 |
| m7 | 62.4 KB from `MAX_PLY = 162` | `search.rs:24` `MAX_DEPTH_TURNS = 64`, `:35` `MAX_Q_EXTENSION_PLIES = 32`, `:41` `2 * 64 + 2 + 32` = 162 (the matrix's line numbers are right; round 1's `:25/:36` were off by one); (80 + 162) × 258 = 62 436 B. `Touched {window, was: [ClassSet; 2], now: [ClassSet; 2]}`, `Applied {at, player, touched: u8}`, `Chunk {p1: u64, p2: u64}` at `4298ecd` give 10 / 6 / 24 by layout; measured in §1a below. | **yes** |
| m8 | said in O-A's row and in §5 | Present at §3 O-A and §5's fourth bullet; `pvs.rs:263-274` is the TT cut, `:434-444` the rule-4 win ✓. The sentence carries one wrong word — R2-m3. | **yes** (with R2-m3) |
| the axis | §2.4 names where each unit is consumed | `state.rs:62` `for (window, index) in windows_through_indexed(at)` ✓; `position.rs:113`, `:142` ✓; `pvs.rs:220` `search_nodes += 1` ✓; `quiescence.rs:260` ✓; **`quiescence.rs:234` is `if score > best_score {`** — the increment is at `:69` (R2-m4). | **yes** (with R2-m4) |

### 1a. `size_of` at `4298ecd`

Measured in my worktree at `4298ecd` (a throwaway `#[cfg(test)]` module appended to
`state.rs`, `cargo test --release --locked -p pistol-solver --lib rt2_print_sizes
-- --nocapture`, exit 0, then the file restored):

```
RT2SIZE Touched 10 Applied 6 (u64,Chunk) 24 Window 6 ClassSet 1 Coord 4
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 42 filtered out
```

The three figures §4.3 calls MEASURED are what the compiler says; the matrix
cites no artifact for them (R2-m7).

---

## 2. Findings

### BLOCKING

None.

### MAJOR

#### R2-M1 — "gated off in every committed config" is false, and at the committed solver-on seat the same two binaries differ by about two per cent

**Claim attacked.** §6 attack 1 (line 504): *"the solver's own use of the state
(`dfpn.rs`, gated off in every committed config) is not in any number"*; the
carried paragraph (line 541) and the design's §6 repeat it.

```
$ git grep -n 'on_search_path = true' ffc5c10 -- configs
ffc5c10:configs/bench_wp18c_solver_on.toml:45:on_search_path = true
ffc5c10:configs/gate_staged_solver_v0.toml:47:on_search_path = true
ffc5c10:configs/play_staged_solver_v0.toml:75:on_search_path = true
$ git grep -n 'gate_staged_solver_v0' ffc5c10 -- tools crates | sed 's/^ffc5c10://'
crates/pistol-cli/tests/census_protocol_tests.rs:10:const ARMED: &str = "configs/gate_staged_solver_v0.toml";
crates/pistol-search/tests/census_identity_tests.rs:14:/// The armed seat's shape, cap and all: `configs/gate_staged_solver_v0.toml`'s
tools/determinism.sh:76:	"staged-solver configs/gate_staged_solver_v0.toml crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt depth_turns-2 nodes-10000"
```

Three committed configs put the solver on the search path, and one of them is
a seat of `tools/determinism.sh` — the hard-rule-4 CI gate — and of two census
tests. The limb the matrix lists as its strongest is therefore not "a seat no
committed config reaches" but a committed, gated seat family the package never
measured. **What the seat says, measured** (exploratory, ONE hand-rolled
instrument, not a registered number; its purpose is to size the limb, not to
replace the bench): the session's own digested binaries `pistol-ffc5c10`
(`78a7600a…`) and `pistol-E` (`f333d101…`) from `/home/tom/pistol-wt/p1-measure`,
`configs/bench_wp18c_solver_on.toml` (`8414509a…`, the WP-1.8c pair's ON seat),
my `session.nodes` (24 positions, `go nodes 50000`), idle-receipted:

```
$ pgrep -af 'cargo|rustc|bench_delta|pistol' | grep -v -E 'pgrep|snapshot-bash'   (before launch)
busy=[]  load 0.51 0.48 0.43
                       early band (12 pos.)          late band (12 pos.)
rep1 base   623808 nodes 30598 ms (20387 nps)   519501 nodes 54577 ms ( 9519 nps)
rep1 E      623808 nodes 29943 ms (20833 nps)   519501 nodes 54301 ms ( 9567 nps)
rep2 base   623808 nodes 30487 ms (20461 nps)   519501 nodes 54559 ms ( 9522 nps)
rep2 E      623808 nodes 30042 ms (20765 nps)   519501 nodes 54513 ms ( 9530 nps)
time ratio base/E:  rep1 early 1.022 late 1.005   rep2 early 1.015 late 1.001
node counts identical per position, both reps, 24 positions; 0 error lines; end busy=[]
```

Two interleaved reps, base then E, ~85 s a side; the whole-engine gain of the
same binary that measured 1.257 / 1.240 at the instrument seat is **≈ 1.5–2 %
early and ≈ 0–0.5 % late** here.

The wall at that seat is the solver's (≈ 13 k nps against ≈ 450 k), and the
threat state's share of it is evidently small: the 24–26 % is the instrument
seat's node mix, not a property the touch carries to every caller. **The
recommendation survives this** — O-E is not slower there, node identity holds,
and the oracle gates cover the solver's correctness — but the matrix's
characterisation of its own strongest limb is wrong at the revision, and the
prereg's bracket is registered at the instrument seat only. **Remedy:** delete
"gated off in every committed config"; name the three configs and the
determinism-gate seat; either record a measured number at the solver-on seat
(the session's binaries make it a two-minute run) or state that the package
claims nothing about it, and let the prereg's identity leg — which already runs
`tactical_staged_v0` — be where that seat is checked.

#### R2-M2 — §2.2's "ceiling" is one O-A exceeds in both bands

**Claim attacked.** §2.2 (line 189): *"O-A in §4 measures it directly at 2.8 % /
2.1 %, at that ceiling"*; reading 1 (line 331): *"O-A's 2.8 % / 2.1 % is §2.2's
ceiling met"*. The ceiling is `unqueried share × at most 35.67 %`, computed
from the pooled 24-position seat (5.28 %) and the 15-stone profile seat.

The bench reports per band, so I ran the counter binary from §1 M1 per band
(fixture split at the script's own `EARLY_MAX=17`, 12 + 12 positions,
`go nodes 50000`):

```
early: places 350489 unqueried 19017 share 5.43%  x 35.67% = 1.94% of wall
late:  places 292342 unqueried 14899 share 5.10%  x 35.67% = 1.82% of wall
(350489 + 292342 = 642831, the pooled seat's places)
```

Against those ceilings O-A measured **2.8 % / 2.1 %** in the session's run and
**2.6 % / 2.6 %** in round 1's idle re-run — above the ceiling in both bands by
0.3–0.9 points, with nps IQRs of 0.2–0.5 % of the median. And m3 makes the true
ceiling LOWER still, because 35.67 % includes the query-side `masks` reads O-A
does not save. A bound the measurement exceeds is not a bound: either the
maintenance share at the bench's seats is above the 400 k-node line-2 profile's
(then "at most 35.67 %" is one seat's), or O-A saves more than the two touches
of each unqueried stone (deferring the apply to just before the query, say).
The conclusion — the count-of-touches lever is worth about two to three per
cent and the cost-of-a-touch lever ten times that — is untouched. **Remedy:**
replace "at that ceiling" / "ceiling met" with the per-band numbers and the
statement that the measurement lands at and somewhat above the estimate, so
the lever is bounded near 3 % whichever way the discrepancy resolves; do not
call an estimate that is exceeded a ceiling.

### MINOR

#### R2-m1 — "the session's edit script verbatim" is a reconstruction, and the record says so elsewhere

§0 M1 (line 43) and §2.2 (line 161). `ls --time-style=full-iso`:
`count_{nodes,depth,line2}.err` 01:12:43; `matrix_P1_threat_state_REDTEAM.md`
01:52:22; `counter_patch_session.py` **01:54:42**; `p1_counters_ffc5c10_v2.txt`
01:54:42. Round 1 found no counter script anywhere in the record at 01:52; the
file cited as "verbatim" was written two minutes after that report and 42
minutes after the outputs it is said to have produced, and the ledger's own
word for it is *"restored"* (`opt_arc_ledger.md:31`). It IS the instrument —
§1 M1 shows it rebuilds and reproduces all three seats to the unit, which is
what M1 asked for — but "verbatim" is a provenance claim the timestamps do not
support. Say "reconstructed after round 1 and verified by re-execution".

#### R2-m2 — "launched in one shell chain after the test suites had returned" is contradicted by the record

§0 m6 (line 51) and reading 5 (lines 350-353). `/home/tom/pistol-wt/p1-patches/`
holds FOUR launch scripts: `run_mx_bench.sh` (01:12:00, a loop over A, D1, D2)
and one each for E (01:18:56), EA (01:19:52), E0 (01:21:56); the bench headers
carry the same minutes (23:12:00, :12:33, :13:06, :18:56, :19:52, :22:42 UTC).
`pistol-EA` was linked at 01:19:52.27, its script written at 01:19:52 and its
bench header stamped 01:19:52 — build, copy and launch in the same second. For
E0, the test binaries in the measurement worktree were linked at
01:22:33–35 and the bench launched at 01:22:42, while round 1 timed that suite
at 15 s (`TESTTIME E0 15s`) — so if the suite ran to completion it had not
returned. Nothing is concluded from this sentence (the idle re-measurement is
what the table leans on) and the fix is to delete it; but a governing document
must not describe its own runs as something the record shows they were not.

#### R2-m3 — `reset_to` does not unwind the eval "FIFO"

§5 (line 459), §0 m8. `position.rs:56` iterates `self.state.board().stones()`,
whose doc at `board.rs:87` is *"Every stone, in ascending `(q, r)` order"* — a
`BTreeMap` walk, neither FIFO nor LIFO. The design (`p1_design.md` §1.4) has it
right: *"in board order"*. The point survives and is stronger for it: the
eval's unwind order is arbitrary with respect to placement, which is exactly
what D-61 licenses and what a LIFO-or-panic state cannot accept.

#### R2-m4 — `quiescence.rs:234` is not a `search_nodes` site

§2.4 (line 220). At `ffc5c10` the increments are `quiescence.rs:69` and `:260`
(`grep -n 'search_nodes += 1'`); `:234` is `if score > best_score {`. Round 1's
axis paragraph carried the same wrong line and the matrix transcribed it.

#### R2-m5 — O-I is compared against the wrong row, on mixed axes

§3 O-I. Open addressing replaces the hashed TABLE, so it is an alternative to
O-D1 (1.016 / 1.017), and it stacks with O-D2's logged undo: the honest
comparator is O-I + log ≈ 1.128 × 1.014–1.017 ≈ 1.14–1.15 against O-E's
1.257 / 1.240 — it still loses by ten points, so the rejection stands, but the
row compares O-I alone with O-E. Separately, "1.4–1.7 % of wall" is set against
a 24–26 % nps ratio; on one axis that is 1.4–1.7 % against 19–20 % of wall
(1 − 1/1.257) or ≈ 1.5 % nps against 24–26 % nps — a twelfth to a fifteenth
either way, so the word survives and the units should match.

#### R2-m6 — "smaller than the one D-254 made the other way on line count" compares incommensurables

§6 (line 486). D-254 traded 4.0–4.8 % of the state's cost (≈ 1.4–1.7 % of wall)
for ~36 fewer lines; O-E over O-E0 takes 4.3–5.3 % nps for `+63/−43` lines in
one file (`git diff --shortstat eecf268 4298ecd -- crates`) and two contract
changes. On the speed axis this trade is three times D-254's; on the line axis
it is smaller. The sentence does no work either way; the paragraph's ground
(no non-LIFO caller, tests pin both changes, the panic class already exists,
O-E0 stands one row up) carries the choice without it. Delete rather than
refine (D-424's test).

#### R2-m7 — §4.3 says MEASURED and cites no artifact

The header promises *"every MEASURED number names the artifact, its digest, and
the revision it was taken at"*; §4.3's `size_of` figures name the revision and
nothing else, and round 1 verified them by layout, not by running anything.
§1a above is the measurement; cite it or an artifact of the session's own.

#### R2-m8 — §1.2 row 5 asserts "no caller" for `is_empty()` on a grep of `window_count()`

Line 117. `git grep -n 'threats.is_empty()' ffc5c10 -- crates` →
`threat_oracle_tests.rs:230` — the roundtrip oracle's fully-unwound assertion,
i.e. the caller the design's I3 relies on. True for `window_count()` (definition
and `lib.rs:41`'s crate doc only), not for the row's second item. Under O-E
`is_empty()` is chunk-map emptiness, O(1), so nothing turns on it.

---

## 3. Re-derivation table

Every command is mine; none is the document's. Scope is stated beside each.

| claim (§) | my command, with scope | my number | document's | agree? |
|---|---|---|---|---|
| profile shares (§2.1) | `perf report -i p1-measure/perf_ffc5c10_line2.data --stdio -n --no-children --percent-limit 1.0` on the session's raw data | delta 30.64 (622 samples), touch 14.04 (293), masks 9.86 (203), set 7.19 (148), apply 6.97, undo 6.34, transition 4.58 (94); sums 35.67 / 17.05 / 13.31 | same | yes |
| sample count (§2.1) | `perf script -i … \| wc -l` | 2074 | 2 074 | yes |
| audit row (§2.1) | `/usr/bin/grep -n 'A-03' docs/audit/repo_audit_2026-09.md` | `13.45 + 9.55 + 7.98 + 5.14 = 36.12` at 2 105 samples | 36.12 | yes |
| counters, three seats (§2.2) | the v2 script rebuilt and run by me (§1 M1), sessions generated by me | 642831/33916, 44166/3780, 240811/16650; 0 flushes with ≥ 2 pending | same | yes, to the unit |
| per-band unqueried share (§2.2) | same binary, fixture split at `EARLY_MAX=17` | 5.43 % early, 5.10 % late; ceilings 1.94 / 1.82 | pooled 5.28 %, "ceiling" 1.9–3.1 | **no** — R2-M2 |
| flush totals (§2.2) | arithmetic over the three P1COUNT lines | 609346+40522+224213 = 874 081; 608915+40386+224161 = 873 462; 431+136+52 = 619 | same | yes |
| writers (§1.1) | three scopes, §1 M3 | 189 / 216 / 177; eleven undo sites; `policy.rs:147` debug-only | 177; eleven | yes |
| readers (§1.2) | `git grep -n 'window_count\|table_snapshot\|threats.is_empty()' ffc5c10 -- crates tools` | `window_count`: definition + `lib.rs:41`; `table_snapshot`: definition + `threat_oracle_tests.rs:103` + docs; `is_empty()`: `threat_oracle_tests.rs:230` | row 4 same; row 5 "no caller" | rows 4 yes; row 5 no for `is_empty` — R2-m8 |
| `staged_context` the only accessor (§5) | `git grep -n 'self.threats\|threats' ffc5c10 -- crates/pistol-search/src/position.rs` | field, `new`, `reset_to`, `place`, `undo`, `staged_context`, and their doc lines | same | yes |
| leaf reads at q = 0 (§1.2) | read `quiescence.rs:62-92`, `:147-163` at `ffc5c10` | `visit` → `quiescence` → `gate_row` → `staged_context` unconditionally; doc `:65-67` says `0` still runs the free checks | same | yes |
| axis line cites (§2.4) | `grep -n 'search_nodes += 1'` | `pvs.rs:220`, `quiescence.rs:69`, `:260` | `pvs.rs:220`, `quiescence.rs:234`, `:260` | one wrong — R2-m4 |
| D-254 quotations (§2.3, §3 O-I) | `/usr/bin/grep -c -F` of each fragment in `docs/decisions.md` | every fragment 1 hit at line 549 ("every" is "Every" in the source) | same | yes |
| solver LIFO by construction (§6) | read `dfpn.rs:700-723` | `apply` first, second; `undo` second, first | same | yes |
| branch SHAs (§3) | `git branch --list 'p1/mx-*' --format='%(refname:short) %(objectname:short)'` | A `cc4153b`, D1 `5f41dc9`, D2 `1c23954`, E0 `eecf268`, E `4298ecd`, EA `e78a094` | same | yes |
| binary digests (§4.1) | `sha256sum` of the session's binaries in `p1-measure`; round 1's independent rebuilds already matched bit-for-bit | `78a7600a`, `f333d101`, `ba532dcf` re-checked | same | yes |
| artifact digests (§2, §4.1, header) | `sha256sum artifacts/p1_*` and `sha256sum -c artifacts/p1_rt_round1_digests.txt` | six bench, profile, v1, v2, digest list `a63ee080…`; 36 of 36 `OK` | same | yes |
| instrument digests (header) | `sha256sum configs/instrument_v0.toml …/bench_positions_v1.txt` (live and worktree) | `02e2e93b…`, `931c50b1…` | same | yes |
| nps ratios, six runs (§4.1) | python over each artifact's median lines | 1.028/1.021, 1.016/1.017, 1.128/1.113, 1.190/1.188, 1.257/1.240, 1.273/1.259 | same | yes |
| largest nps IQR % (§4.1) | python, max over both sides and bands | 0.45, 0.81, 0.50, 0.74, 0.78, 0.66 | same | yes |
| ttd medians and IQRs (§4.1 reading 4) | same | 66–117 ms, IQR ≤ 2 ms | same | yes |
| node identity, all runs | `grep -c 'node identity holds'` in all 13 artifacts | 1 each; `BENCH_EXIT=0` each | holds | yes |
| round-1 re-bench and pairs (§4.1 readings 2, 3, 5) | grep of the seven receipts | 1.026/1.026, 1.183/1.181, 1.256/1.241, 1.267/1.251; 1.053/1.043, 1.018/1.014, 1.111/1.091 | same | yes |
| idle receipts of round 1 (§4.1 reading 5) | `grep '^== bench' phase2.log phase3.log` | `busy=[]` × 7 (E0's 1-min load 4.18, a test tail's decay) | "pgrep empty" | yes |
| 246 tests (§6 attack 1) | `grep '^test result' test_{base,E,E0,EA}.log` summed | 246 passed / 0 failed, 38 suites, each | same | yes |
| diff stats (§4.2) | `git diff --shortstat ffc5c10 p1/mx-* -- crates` and unfiltered | +14/−4, +47/−16, +106/−34, +255/−110, +276/−111, +290/−115; nothing outside `crates`; E0→E `+63/−43`, one file | same | yes |
| line counts (§4.2) | `git show <rev>:<file> \| wc -l` | state 122/122/122/163/173/193/193; table 214/214/245/245/308/308/308; position 165/175/165/165/165/165/175 | same | yes |
| rule-9 entry (§4.2) | `/usr/bin/grep -c 'pistol-solver/src/table.rs' docs/rule9_justifications.md` | 0 | 0 | yes |
| §4.3 arithmetic | `search.rs:24,35,41`; `(80+162)*258` | 162; 62 436 B | 162; 62.4 KB | yes |
| `size_of` (§4.3) | §1a | `Touched` 10, `Applied` 6, `(u64, Chunk)` 24 (`Window` 6, `ClassSet` 1, `Coord` 4) | 10 / 6 / 24 | yes, now measured |
| `pack` ranges, `run` guard (§5) | `table.rs` at `4298ecd`, lines 119-122, 213-221 | doc states `[-65536, 65534]`, `[-512, 511]`; `offset + count > 64`; shift `64 - offset` | same | yes |
| O-E's mechanism (§3) | `state.rs`/`table.rs` at `4298ecd` | `log: Vec<Touched>`, `chunks: Vec<(u64, Chunk)>`, `applied: Vec<Applied>`; frame check at `:141-144` panics `THREAT_DESYNC`; `impl PartialEq` by hand `:62`, `impl Eq :68`; `remove` on vacant chunk `:255,:265`; `len()` via `snapshot()` `:272-273` | same | yes |
| no choice-path clone of the state | `git grep -n -E 'threats?\.clone\(\)\|ThreatState.*clone' ffc5c10 -- crates/*/src` | none | (not claimed) | — |
| O-F prior (§3) | `matrix_wp19_storage.md:127-128, :183, :212-220` | 1.737/1.837 vs 1.783/1.909; lattice-edge test fails; `O(R^2)` | same | yes |
| O-G's file (§3) | `git grep` in `handcrafted.rs` | `windows: WindowMap` at `:152` | same | yes |
| committed solver-on configs (§6 attack 1) | `git grep -n 'on_search_path = true' ffc5c10 -- configs` | three; one a determinism-gate seat | "every committed config" gated off | **no** — R2-M1 |
| session-run provenance (§0 m6, reading 5) | `ls --time-style=full-iso p1-patches p1-measure`, bench headers | four launch scripts; EA build/launch same second; E0 launched 7–9 s after a 15 s suite's link | "one shell chain after the test suites had returned" | **no** — R2-m2 |
| counter-script provenance (§0 M1) | same | script 01:54:42, outputs 01:12:43, round-1 report 01:52:22 | "verbatim" | **no** — R2-m1 |

---

## 4. The recommendation, attacked with fresh eyes

**The O-E-over-O-E0 paragraph.** Its ground is the four facts it states, and
all four re-derive: no non-LIFO caller at any scope I chose (§1 M3); `dfpn.rs`
LIFO by construction (`:700-723`); the refusal is the `THREAT_DESYNC` class
already raised at `state.rs:15,41-59`; and the two contract changes have named
tests in the design (I4, I7). The 1.053 / 1.043 it prices them at is round 1's
direct pair, idle. The one sentence in it that does no work is R2-m6. The
paragraph is sound, and it answers the "nothing says which the project
prefers" limb of round 1's attack.

**The carried paragraph.** Of its four limbs, three are answered by this
revision and one is not: O-I is in the field (rejected on D-254's own figure,
R2-m5 notwithstanding); the O-E/O-E0 preference is stated with its ground; the
counters instrument is in the record and runs (§1 M1). The fourth limb — every
number taken where the solver is gated off — stands, and R2-M1 shows it stands
larger than the matrix says: the solver-on seat is committed and CI-gated, and
at it the two binaries are within about two per cent of each other. What round
1 could only assert, this round measured once.

**Things that were attacked and held.** The histogram's zero ≥ 2-pending
flushes is structural (every `place` is followed by a `visit` that flushes
before the next `place`), and the matrix reads it as "nothing to merge", which
is the right reading for O-C. The "leaves read the state too" premise holds at
`quiescence.rs:76,148`. The hasher argument is round 1's MEASURED transfer. No
choice path clones the state, so the log's extra bytes ride only on the
oracle's recorded stack. Every §4.4 row matches `4298ecd`.

---

## 5. The two drafts that follow — contradictions only (not reviewed)

- `p1_design.md` header (lines 4-5) is governed by *"`matrix_P1_threat_state.md`
  revision 1 and its selection record (`matrix_P1_threat_state_selection.md`),
  which took O-E"*. The matrix is at revision 2, it says *"Nothing is selected
  in this document"*, and `ls docs/experiments/ | grep -i p1` finds no selection
  record. A stale revision citation of the class D-602 corrected yesterday, plus
  a citation of a file that does not exist.
- `p1_design.md` §1.4 says `reset_to` unwinds *"in board order"*; the matrix §5
  says *"FIFO"*. The design is right (R2-m3).
- `p1_design.md` §6 repeats *"`dfpn.rs` is gated off in every committed
  config"* — false at `ffc5c10` (R2-M1).
- `p1_bench_prereg.md`: no contradiction found. Its instrument revisions check
  (`git log -1 --format=%h ffc5c10 -- <file>`: `bench_delta.sh` `ab369b0`,
  `instrument_v0.toml` `e4bb5bf`, `bench_positions_v1.txt` `70cc465`,
  `tactical_staged_v0.toml` `e4bb5bf`, `tactical_staged_v0.txt` `538b3e5`); its
  "AT MOST 35.67 %" matches m3; its referent 1.257 / 1.240 is the matrix's. It
  registers the bracket at the instrument seat only, which R2-M1 makes worth
  saying out loud in it.

---

VERDICT: the recommendation SURVIVES but the matrix FAILS (revise: R2-M1 — "`dfpn.rs`, gated off in every committed config" is false at `ffc5c10`, three committed configs arm it and one is a `tools/determinism.sh` seat, and at that seat the two binaries differ by about two per cent; R2-M2 — the §2.2 "ceiling" O-A is said to meet is exceeded in both bands when the counter is run per band)

**Strongest surviving attack, for the selection record verbatim.** O-E's
24–26 % is the instrument seat's number and only that seat's: at the committed,
CI-gated solver-on seat — the one the matrix wrongly calls uncommitted, and the
one whose caller drives the contract being narrowed hardest — the same two
binaries are about two per cent apart, so the gain the package banks is a
property of one node mix and the LIFO-or-panic narrowing and hand-written
equality are bought, at the solver's seat, for nothing measurable; the premise
that licensed the matrix's shape is not tight either, since O-A's measured gain
exceeds the "ceiling" the unqueried share puts on it in both bands, which says
the 35.67 % is one seat's share and not a bound; the one alternative D-254
called faster is still rejected on a figure from another instrument at another
revision, unprototyped, and compared against O-E rather than against the row
it would actually replace; and the chunk-boundary and lattice-edge behaviour of
the store that wins has no falsifier until the design's tests exist.
