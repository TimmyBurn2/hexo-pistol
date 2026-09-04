# MATRIX P1 — DECISION-RED-TEAM, round 1

**Target.** `docs/experiments/matrix_P1_threat_state.md` (revision 1) at the
named revision `6099aa74a03f8a66cdfcfdd2c1b961de9c297fc5` — a `git stash create`
commit on `dev` = `ffc5c10f4d16356f574e3221a023f78399d2e3bb`.

**Does that revision still match the working tree's bytes?** yes — byte-identical (sha256 20c9ce1b… both ways, checked again at the moment this report was written).
**HEAD** at review time: `ffc5c10f4d16356f574e3221a023f78399d2e3bb`. **Date:** 2026-09-04.

**Verdict, stated first.** VERDICT: the recommendation SURVIVES but the matrix
FAILS (revise: M1, M2, M3). Every load-bearing number reproduces — the profile,
the counters, all six benches, the digests, the line counts — most of them from
instruments I built and commands I chose. What fails is the record: the matrix
cites an artifact for content it does not hold (M1), its field omits the one
alternative the ADR it invokes recorded as faster (M2), and the section that
narrows a public contract to LIFO cites a command that cannot produce its own
table (M3). None of the three moves the selection; all three are the defect
class `docs/process.md`'s re-derivation clause exists for.

**Environment.** Everything adjudicated was run with `git grep` pinned to
`ffc5c10` or `/usr/bin/grep`, sorted `LC_ALL=C`. Builds, tests and benches ran in
`/home/tom/pistol-wt/redteam-p1` (`git worktree add --detach ffc5c10`, own
`CARGO_TARGET_DIR`), removed at the end with no branch left behind; nothing in
the live tree was built or modified. `pgrep -af 'cargo|rustc|bench_delta'` was
empty before every timing run and is printed in each bench log. My receipts are
in this session's scratchpad (`…/scratchpad/rt/`), digests below; every
load-bearing line is quoted here so the report stands without them.

---

## BLOCKING

None.

## MAJOR

### M1 — The counters artifact does not contain the instrument the matrix says it prints

**Claim attacked.** §2.2: *"A scratch build at `ffc5c10` with counters in
`Position` (the edit is printed in `artifacts/p1_counters_ffc5c10_v1.txt`,
sha256 `f7e07c4f…`; never committed)"*.

```
$ wc -l -c artifacts/p1_counters_ffc5c10_v1.txt
9 1123 artifacts/p1_counters_ffc5c10_v1.txt
$ tail -1 artifacts/p1_counters_ffc5c10_v1.txt
# counter patch (python edit script) follows
```

Nothing follows. The file the digest `f7e07c4f…` names ends on the line that
announces the patch. The patch is also absent from the session's own patch
directory (`ls /home/tom/pistol-wt/p1-patches/` holds `proto_{A,D1,D2}.py`,
`proto_E.diff`, `proto_E_table.rs` and the three `count_*.err` outputs — no
counter script) and from the measurement worktree (`git -C
/home/tom/pistol-wt/p1-measure status --short` lists only untracked binaries and
outputs; `crates/` is clean). **The instrument behind the premise's three
percentages exists nowhere in the record.** D-479/D-483: a number a document
consumes is cited from an artifact by digest — here the digest is of a file that
lacks what the citation says it holds.

**The numbers themselves stand, re-derived with an instrument I wrote** (a
`Vec<bool>` beside `placed` marking each stone observed-since-placed, flushed in
`staged_context`; `counter_patch.py`, sha256 `dd731212…`), built at `ffc5c10`
and run over sessions I generated from the fixture:

```
seat 1  RTCOUNT places 642831 undos 642831 flushes 609346 unqueried_undos 33916 resets 24 reset_stones 596 root0 84  revisit0 347 pending_hist [431, 608915, 0, 0, 0, 0, 0, 0]
seat 2  RTCOUNT places 44166  undos 44166  flushes 40522  unqueried_undos 3780  resets 24 reset_stones 596 root0 46  revisit0 90  pending_hist [136, 40386, 0, 0, 0, 0, 0, 0]
seat 3  RTCOUNT places 240811 undos 240811 flushes 224213 unqueried_undos 16650 resets 1  reset_stones 15  root0 5   revisit0 47  pending_hist [52, 224161, 0, 0, 0, 0, 0, 0]
```

Every figure equals the artifact's, to the unit. **Remedy:** append the
session's counter patch to the artifact and re-digest, or cite the re-derivation
above; either way the matrix's sentence must describe a file that exists.

### M2 — The field omits the alternative D-254 itself recorded as the faster one

**Claim attacked.** §2.3 and §3 build the field on D-254's flip clause and quote
its red team (*"every positive performance ground this line states is refuted by
the session's own second harness"*). The same D-254 paragraph, one sentence on:

```
$ /usr/bin/grep -o "the flip is that open addressing is the faster option on every instrument that reads the corpus correctly" docs/decisions.md
the flip is that open addressing is the faster option on every instrument that reads the corpus correctly
$ /usr/bin/grep -o "open addressing is ahead by 4.0–4.8 %" docs/decisions.md
open addressing is ahead by 4.0–4.8 %
$ /usr/bin/grep -c "open addressing" docs/experiments/matrix_P1_threat_state.md
0
```

Hand-written open addressing over the packed key — D-254's ~90-line
alternative, the row WP-1.9's red team also left *"not disposed of"* — is not in
this field. O-F (dense direct addressing) is a different structure and its
dismissal does not reach it. **The conclusion survives on D-254's own numbers:**
4.0–4.8 % of the state's cost is, over the 35.67 % share, **ESTIMATED 1.4–1.7 %
of wall**, a fifteenth of O-E's measured 24–26 %; and O-E removes the per-window
churn that any hashed store, hand-written or not, still pays (reading 1). But a
matrix that fires an ADR's flip clause and quotes its red team owes a row for
the option that red team said was faster, rejected on that ground — one line.

### M3 — §1.1's stated command cannot produce §1.1's table, and one engine row is debug-only

**Claim attacked.** §1.1: *"Writers — every `apply` / `undo` call site at
`ffc5c10`"*, derived by
`/usr/bin/grep -rn '\.undo(\|\.apply(' crates/pistol-solver/src crates/pistol-search/src …`.

The table's fifth and sixth rows cite `crates/pistol-solver/tests/threat_oracle_tests.rs`,
which that scope cannot visit — so the table was assembled from a population
other than the one the document names, which is the process-clause defect
exactly. Re-derived at my own scope (every `.rs`, `.py` and `.sh` in the
repository, `tools/sealbot/matchserver` included):

```
$ git grep -n '\.undo(\|\.apply(' ffc5c10 -- ':(glob)**/*.rs' ':(glob)**/*.py' ':(glob)**/*.sh' | grep -v 'eval\.undo\|eval\.apply\|self\.eval' | LC_ALL=C sort
  … 121 lines; the ThreatState UNDO sites among them are exactly:
  crates/pistol-search/src/position.rs:142
  crates/pistol-solver/src/dfpn.rs:720, :722
  crates/pistol-solver/src/policy.rs:155, :156, :398, :399
  crates/pistol-solver/tests/threat_oracle_tests.rs:222, :336, :344
```

So **the conclusion holds at the wider scope: no call site anywhere in the
repository undoes out of LIFO order** (the eleven undo sites are the matrix's
own list; the other 110 hits are `Board`/`GameState`/`Symmetry`/eval calls or
apply-only `ThreatState` builders in tests and examples — `heuristics.rs:273`,
`examples/value_fixture_recall.rs:69`, `search/tests/common/mod.rs:133`,
`solver/tests/common/mod.rs:148`, `wp15b_census.rs:244/282/311`,
`wp18b_m4_tests.rs:86/140/271`, `threat_query_tests.rs:741`, …). What the
table gets wrong beyond scope: `policy.rs:149-156` sits inside
`#[cfg(debug_assertions)]` (`sed -n 147p crates/pistol-solver/src/policy.rs` →
`            #[cfg(debug_assertions)]`), so that apply/undo pair does not exist
in any release binary — the table presents it as an engine call site without
saying so. The LIFO contract §5 proposes is therefore supported by a grep the
matrix did not run, over a population it does not name; it is still true.

## MINOR

### m1 — "The zero-pending flushes are the root's own calls" is false

§2.2. My instrument splits the zero-pending bucket by whether `placed` is
empty at the flush: seat 1 **84 of 431** are root calls, seat 2 **46 of 136**,
seat 3 **5 of 52** (five iterations of one search). The remainder (347 / 90 / 47)
are re-visits of an already-flushed node — `Run::child`'s null-window scan
followed by the full re-search (`pvs.rs:561-579`), each a `visit` that calls
`staged_context` again with nothing new placed. Nothing in the matrix turns on
this; it is a statement of fact about the data that the data refutes.

### m2 — Readings 2 and 3 are differences between ratios of different runs, when the one-run measurement costs 33 s

§4.1 readings 2–3 derive *"≈ 10 %"*, *"≈ 5 %"* and *"≈ 1.5 %"* by dividing
ratios taken in separate runs against separate baseline samples (baseline
medians across the six runs spread 446 340–448 668 early, 0.52 %, and
393 350–395 519 late, 0.55 % — a third of the smallest quantity read). D-479's
shape, and D-291's: `bench_delta.sh` takes any two binaries, so each reading is
one 33-second run. Taken (idle machine, node identity held, all exit 0):

```
E0 -> E   band early: nps ratio 1.053   band late: nps ratio 1.043   (bench_E0_to_E.txt  9df4bd58…)
E  -> EA  band early: nps ratio 1.018   band late: nps ratio 1.014   (bench_E_to_EA.txt  bd2a231e…)
D1 -> D2  band early: nps ratio 1.111   band late: nps ratio 1.091   (bench_D1_to_D2.txt 903ba0cd…)
```

The readings hold. The matrix should quote these shapes of number, MEASURED,
rather than derive them.

### m3 — The 35.67 % "maintenance" sum includes query-side reads of `WindowTable::masks`

§2.1 attributes all 9.86 % of `masks` to maintenance. `masks` is also the point
read every cover and cell query makes (`query.rs:221, :240-241, :259`;
`cover.rs:247`), and the profile was taken without a call graph
(`perf record -F 2000`, no `-g`), so the two cannot be split from the artifact.
The maintenance share is an UPPER bound, and O-E makes exactly those query reads
dearer (§4.4). Not load-bearing here — the bench measured the outcome — but the
prereg's bracket will be derived from this number and should say "at most".

### m4 — O-G cites a file that does not exist

§3 O-G: `crates/pistol-eval/src/window_map.rs`. `ls crates/pistol-eval/src/`
→ `error.rs eval.rs handcrafted.rs lib.rs weights.rs window.rs`. The eval's
window map is `HandcraftedV0`'s field in `handcrafted.rs` (WP-1.9's matrix,
§1). A false path in a governing document, on a row that is not selected.

### m5 — The straddling test the matrix names is the boundary no game reaches

§5/§6: *"places stones at positions 63 and 64 of a line"*. The origin sits at
position 0 of every line, so the chunk boundary every real game crosses on its
first turns is **−1 / 0** (`rem_euclid`'s wrap, `table.rs` at `4298ecd`), and
the next is −64 / −65; the fixture's positions never reach ±63. The oracle's
random playouts (`threat_oracle_tests.rs`, 150 plies from the origin) exercise
−1/0 constantly and 63/64 never. The design owes the three boundaries, not one,
and the negative pair is the one with a live falsifier today.

### m6 — No idle receipt, and the measurement worktree's own timestamps show cargo linking seconds before two benches

The artifacts carry no `ps`/`pgrep` line. In `/home/tom/pistol-wt/p1-measure/target`,
`find -newermt` shows a `cargo test --release -p pistol-solver -p pistol-search`
linking 41 targets at **01:18:30** local (the E bench header is 23:18:56 UTC =
01:18:56) and a `-p pistol-solver` test link at **01:22:33–35** (E0 bench
01:22:42). The suites take 13–16 s wall on this machine (`TESTTIME E 16s`,
`E0 15s`), so E was clear by ~10 s and E0's first baseline rep may have shared
its first seconds with a test tail. **Re-measured idle, no number moves:**

```
E0  early 1.183 late 1.181   (artifact 1.190 / 1.188)   bench_E0.txt 12e71776…
E   early 1.256 late 1.241   (artifact 1.257 / 1.240)   bench_E.txt  e3eaed80…
EA  early 1.267 late 1.251   (artifact 1.273 / 1.259)   bench_EA.txt d142137d…
A   early 1.026 late 1.026   (artifact 1.028 / 1.021)   bench_A.txt  c893a81e…
```

The landing prereg should register the idle check as part of the receipt
(D-592's principle; the dispatcher's brief states it), because next time the
tail may be longer than the margin.

### m7 — §4.3's arithmetic is a little under

`MAX_PLY = 2 * MAX_DEPTH_TURNS + 2 + MAX_Q_EXTENSION_PLIES = 2*64 + 2 + 32 = 162`
(`search.rs:25, :36, :41`), so an 80-stone root searched to `MAX_PLY` holds
(80 + 162) × 258 B = **62.4 KB** of frames, and the three `Vec`s' doubling can
hold twice that. "50–60 KB" is the wrong side of the order; the conclusion
(negligible against 268 MB) is untouched.

### m8 — The lazy seam is measured over half of what it serves

O-A defers only the threat state. At a TT-cut child (`pvs.rs:263-274`) or a
rule-4 win (`:434-444`) nothing reads the eval either — `delta` runs inside
candidate generation, which such a node never reaches — so the eval's
`apply`/`undo` (13.31 %, §2.1) is wasted on the same 5.3–8.6 % of stones.
ESTIMATED 0.7–1.1 % of wall on top of O-A's measured 2.1–2.8 %, behind three
doors (`value`, `static_score_after`, `staged_context`) rather than one. Recorded
so §6's *"marginal"* judgement on O-A is made over the whole seam, and with the
design cost named: D-61 makes the eval's unwind order-free (`position.rs:48-59`
unwinds it FIFO in `reset_to`) while O-E's log makes the threat state's unwind
LIFO-or-panic — two caches behind one `Position` with opposite undo contracts,
and the `ThreatState::new()` replacement at `:60-62` stops being a convenience
and becomes the only correct spelling.

---

## Re-derivation table

Every command is mine; none is the document's. Scope is stated beside each.

| claim (§) | my command, with scope | my number | document's | agree? |
|---|---|---|---|---|
| profile shares (§2.1) | `perf report -i p1-measure/perf_ffc5c10_line2.data --stdio -n --no-children --percent-limit 1.0` (the session's raw perf data, read directly) | delta 30.64 (622), touch 14.04 (293), masks 9.86 (203), set 7.19 (148), apply 6.97, undo 6.34, transition 4.58 (94); sum 14.04+9.86+7.19+4.58 = 35.67; masks+set 17.05 | same | yes |
| sample count (§2.1) | `perf script -i … \| wc -l` | 2074 | 2 074 | yes (the artifact itself says only "2K") |
| audit row (§2.1) | `/usr/bin/grep -n "A-03" docs/audit/repo_audit_2026-09.md` | 13.45+9.55+7.98+5.14 = 36.12 | 36.12 | yes |
| counters, three seats (§2.2) | my own instrument (`counter_patch.py`), sessions generated from the fixture by me | 642831/33916 (5.276 %), 44166/3780 (8.559 %), 240811/16650 (6.914 %) | 5.28 / 8.56 / 6.91 % | yes, to the unit |
| flush totals (§2.2) | same | 609346+40522+224213 = 874 081; ≥2 pending = 0; =1: 873 462; =0: 619 | same | yes |
| zero-pending = root calls (§2.2) | same, split on `placed.is_empty()` | 84/431, 46/136, 5/52 | "the root's own calls" | **no** (m1) |
| O-A ceiling arithmetic (§2.2) | `python3 -c` | 0.0528×35.67 = 1.88; 0.0856×35.67 = 3.05 | 1.9–3.1 | yes |
| leaves read the state at q=0 (§1.2) | read `pvs.rs:226-255` → `quiescence.rs:68-92` → `gate_row` `:148` | `visit` at depth 0 → `quiescence` → `gate_row` → `staged_context` unconditionally | same | yes |
| `staged_context` is the only accessor of `threats` (§5) | `git grep -n '\.threats\b' ffc5c10 -- crates` | position.rs:60, 61, 66, 112, 141, 157 only | same | yes |
| writers (§1.1) | repo-wide grep, M3 | 11 undo sites, all LIFO; policy.rs pair is debug-only | 6 rows | conclusion yes; table scope no (M3) |
| `table_snapshot` sole caller (§1.2) | `git grep -n table_snapshot ffc5c10 -- crates tools` | threat_oracle_tests.rs:103 (+ the definition and two doc links) | same | yes |
| `window_count()` callers (§1.2) | `git grep -n 'window_count()' ffc5c10 -- crates tools` | definition only | same | yes |
| binary digests (§2.1, §4.1) | independent build of `ffc5c10` and each `p1/mx-*` in my worktree at a different path | 78a7600a, ac4988dc, 3a5717f9, 458724ad, ba532dcf, f333d101, 8b33fff2 | same seven | yes — bit-for-bit |
| artifact digests (§2, §4.1) | `sha256sum artifacts/p1_*` | ed5f0cab, f7e07c4f, 87a081a5, 851478e9, c0b11f4b, 8697d659, d0b1b384, 4bc15f66 | same | yes |
| instrument digests (header) | `sha256sum configs/instrument_v0.toml …/bench_positions_v1.txt` | 02e2e93b…, 931c50b1… | same | yes |
| nps ratios, six runs (§4.1) | recomputed from each artifact's median lines | 1.028/1.021, 1.016/1.017, 1.128/1.113, 1.190/1.188, 1.257/1.240, 1.273/1.259 | same | yes |
| nps ratios, re-measured (§4.1) | `tools/bench_delta.sh` PATH mode on my builds, idle | A 1.026/1.026, E0 1.183/1.181, E 1.256/1.241, EA 1.267/1.251 | within 0.7 % | yes |
| largest IQR % (§4.1) | `python3` over each artifact's IQR/median | 0.45, 0.81, 0.50, 0.74, 0.78, 0.66 | same | yes |
| reading 2, 3 (§4.1) | direct pairs, one run each | E0→E 1.053/1.043; E→EA 1.018/1.014; D1→D2 1.111/1.091 | ≈5 %, ≈1.5 %, ≈10 % | yes (m2) |
| node identity, all runs | the script's own line in every artifact and every re-run | holds | holds | yes |
| diff stats (§4.2) | `git diff --shortstat ffc5c10 <branch> -- crates` and without the path filter | +14/−4, +47/−16, +106/−34, +255/−110, +276/−111, +290/−115; the branches touch nothing outside `crates` | same | yes |
| line counts (§4.2) | `git show <branch>:<file> \| wc -l` | state 122/122/122/163/173/193/193; table 214/214/245/245/308/308/308; position 165/175/165/165/165/165/175 | same | yes |
| rule-9 entry (§4.2) | `/usr/bin/grep -n 'pistol-solver' docs/rule9_justifications.md` | cover, dfpn, policy, solver, tt, zone, three test files — no `table.rs` | 0 | yes |
| `size_of` (§4.3) | by layout from the diff (`Window` 6 = `Axis` 1 + `Coord` 4 padded to align 2, per WP-1.9's measured sizes; `ClassSet` 1; `Player` 1) | Touched 10, Applied 6, (u64, Chunk) 24; 18×10+3×24+6 = 258 | same | yes |
| §4.3 total | `MAX_PLY` from `search.rs:41` | 62.4 KB (+ Vec doubling) | 50–60 KB | roughly (m7) |
| `pack` bias ranges (§5) | arithmetic over `i16` | q+r ∈ [−65536, 65534]; pos.div_euclid(64) ∈ [−512, 511]; `run` reaches −513 and 512; all biased fields non-negative and < 2^24 / 2^16 | same | yes |
| straddling guard (§5) | read `run` at `4298ecd` | `offset + count > 64` false at offset 0 for count ≤ 64; shift `64 − offset` ∈ [1, 63] | same | yes |
| hasher (§5, ESTIMATED) | `python3` SplitMix64 over the 24 fixture positions' chunk keys and window keys, hashbrown's power-of-two bucket count at 7/8 load, low bits as index | chunk keys 21–53 per position in 32–64 buckets, distinct/keys ≥ 0.62, max bucket 6; window keys 120–386 in 256–512, ≥ 0.70, max 5 | "transfers — ESTIMATED" | yes, now MEASURED: no collapse |
| O-F prior (§3) | `matrix_wp19_storage.md` §4.1, §5 | O-4 1.737/1.837 vs O-2 1.783/1.909; lattice-edge test fails; O(R²) | same | yes |
| O-G's file (§3) | `ls crates/pistol-eval/src/` | no `window_map.rs` | `window_map.rs` | **no** (m4) |
| prototypes pass the suite (§6 attack 2) | `cargo test --release --locked -p pistol-solver -p pistol-search` at `4298ecd`, `e78a094`, `eecf268`, `ffc5c10` | 246 passed / 0 failed each, incl. `threat_incremental_matches_reference_on_random_playouts`, `threat_apply_undo_roundtrips`, `window_map_ordering_is_unobservable` and the three `should_panic` desync tests | "passes on all playouts" | yes |
| E's mechanism matches §3 | `git diff ffc5c10 p1/mx-E -- crates`; `diff` against `p1-patches/proto_E.diff` | identical modulo `index` lines; key `(axis, line, chunk)`, two `u64` per chunk, three `flip`s, 11-bit run, per-window `Touched {window, was, now}`, per-stone three `(key, Chunk)`, LIFO refusal, hand-written `PartialEq` over table and sets, prune-on-vacant chunk, `len()` via snapshot | same | yes |

---

## The axis (D-477)

The matrix's axis is *the cost of a touch against the count of touches*. The
unit both arms share is the STONE: a touch is one `ThreatState::apply`/`undo`
(`state.rs:46-59`), consumed at `state.rs:62` — `for (window, index) in
windows_through_indexed(at)` — where one stone becomes eighteen window records;
the count is consumed at `position.rs:113` and `:142`, once per
`Position::place`/`undo`, and that is what my counters and the session's count.
The bench's unit is the NODE (`search_nodes` at `pvs.rs:220`, `quiescence.rs:234`
and `:260`), and a leaf stone is counted twice there (`visit` then
`quiescence`) — nodes are not stones — but the stones-per-node ratio is fixed by
the script's per-position node-identity assertion, so every ratio in §4.1 is a
ratio of per-stone costs. The axis survives; the matrix should say where the
unit is consumed rather than leave it to the reader.

## §6's four self-listed attacks, answered

1. **One seat.** True and unanswered by this round, which re-measured the same
   seat. What the round adds: the solver (`dfpn.rs:700-724`) drives the same
   type through `apply_turn`/`undo_turn`, strictly LIFO, and the solver's oracle
   suites are among the 246 tests that pass on E, EA and E0 — so the OFF seat's
   correctness is checked even where its speed is not.
2. **The snapshot is not the incumbent's by construction.** Weaker than the
   author fears. `compare` in `threat_oracle_tests.rs:98-200` reads BOTH doors
   at every ply of every playout — the snapshot against the reference table
   (`:102-112`) and every query, which goes through `masks`, against the
   reference's answers — so a window the snapshot lists and `masks` disagrees on
   is caught at the same ply. The residual is m5: the playouts never reach a
   positive chunk boundary or the lattice edge, so those two are owed a test and
   have no falsifier today.
3. **LIFO on the strength of a grep.** The wider grep (M3) finds no non-LIFO
   caller anywhere — tests, tools, the sealbot matchserver included. The
   stronger form of this attack is m8's: LIFO makes the threat state the one
   cache behind `Position` whose unwind is order-dependent while D-61 makes the
   eval's order-free three lines above it, and the design must pin the
   `reset_to` replacement as load-bearing rather than as an O(1) convenience.
4. **Binaries tied to sources by the session's word.** Answered by instrument:
   rebuilding `ffc5c10` and all six branches in an independent worktree at a
   different path reproduces all seven digests bit-for-bit, so the sources on
   the branches are the bytes the benches measured — no longer the session's
   word. (And `bench_delta.sh` in `rev:` mode on a `p1/mx-*` commit would tie
   them a third way at landing.)

**A stronger attack the author did not list** is M2 with m2 behind it: the row
D-254 itself said was faster is not in the field, and the one number that
justifies O-E over O-E0 — the logged undo, which is also the whole of the LIFO
narrowing and the hand-written equality — is 4.3–5.3 % measured directly. That
is a real gain and it is bought with the two contract changes the design will
spend its review surface on.

---

## §R — what reproduced exactly

Stated so the length above is read in proportion: the profile to the sample; the
three counter seats to the unit, from an independent instrument; all seven
binary digests from independent builds; all eight artifact digests; every nps
ratio, IQR and time-to-depth median in §4.1; all four re-measured ratios within
0.7 %; the three direct-pair readings; every line count and diff stat; the
`size_of` figures; the `pack` ranges; the `run` boundary argument; the hasher
claim (upgraded from ESTIMATED to MEASURED); the rule-9 gap; the D-254 quotation;
the WP-1.9 prior; the `staged_context`-only access; the leaf-reads-at-q=0
premise; and 246 green tests on each of E, EA and E0. This is a matrix whose
numbers are true and whose record has three holes.

---

VERDICT: the recommendation SURVIVES but the matrix FAILS (revise: M1 — the counters artifact does not contain the counter patch it is cited for and the instrument exists nowhere in the record; M2 — hand-written open addressing, the alternative D-254's own strongest surviving attack calls the faster option, is absent from a field that fires D-254's flip clause; M3 — §1.1's stated command cannot produce §1.1's table and lists a `#[cfg(debug_assertions)]` pair as an engine call site)

**Strongest surviving attack, for the selection record verbatim.** O-E is the
largest gain among the rows that were measured, at one seat, and the field it
won never held the row D-254 itself recorded as *"the faster option on every
instrument that reads the corpus correctly"* — hand-written open addressing over
the same key — which is dismissed here only after the fact, on D-254's own
4.0–4.8 % of the state's cost against O-E's measured 24–26 % of wall; the half of
O-E that narrows a public `undo` to LIFO-or-panic and replaces a derived
`PartialEq` with a hand-written one — the logged undo — buys 5.3 % early and
4.3 % late measured directly against O-E0 in one run, so a design that judged
those two contract changes too dear could take O-E0 at 18 % and forgo a
twentieth of the engine's speed, and nothing in this matrix says which the
project prefers; the premise's three percentages, though reproduced to the unit
by a second instrument, were produced by one whose source the cited artifact
announces and does not contain; and every number here was taken at the seat
where the solver is gated off, while the solver is the one other caller of the
contract being narrowed.
