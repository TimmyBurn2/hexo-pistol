# REVIEW-impl ROUND 2 — WP-2.1 lever B, the label cache package. **VERDICT: PASS — 0 BLOCKING, 0 MAJOR, 2 minor.**

**REVISION REVIEWED**: `4fab9edaefe01d5ae3f772a2eda03479cacee48c` (`4fab9ed`, branch `dev`),
scoped to the seven remedies of round 1 (`wp21_label_cache_impl_REVIEW.md` at `9c4366c`,
FAIL 1B/0M/4m) and the red team (`wp21_label_cache_REDTEAM.md` at `9c4366c`, FAIL 0B/1M/2m),
adjudicated by execution (D-590, D-591, D-592). **HEAD MATCH: NO** — at the end of this review
the live tree is at `036b4a46d13e69ef785e6e7e074f34a06eca9415` (`036b4a4`, one commit past
`4fab9ed`, landed while this review ran; §5 N1 names what it changes). The worktree stayed
at `4fab9ed` throughout.

**WHERE**: a detached worktree `/home/tom/pistol-wt/review-impl2` at `4fab9ed`,
`CARGO_TARGET_DIR=/home/tom/pistol-wt/review-impl2/target` set per command; a second
detached worktree `/home/tom/pistol-wt/review-impl2-load` at the same SHA, its own target
dir, used ONLY as the load source (`cargo test -p pistol-arena --locked` in a loop, three
full iterations, 08:21–08:36). Scratch under `review-impl2/scratch/` (driver `mut.py`, per-run
logs under `scratch/out/`, the by-hand fixture under `scratch/fx/`); nothing written in the
live tree but this file; `/tmp` untouched by me. Every mutant was applied by exact
single-match string replacement, run, and restored with `git checkout -- crates tools`;
`git status --porcelain` (scratch excluded) was empty after each and at the end. The
implementer's worktree `/home/tom/pistol-wt/mutation` was running its own
`scratch_mutate.py X3 stray never` when I began (08:16) and had finished before my first
timed run (`ps` at 08:21: 0 other jobs); no other cargo, arena or python of this project was
on the box during the measurements. All numbers here are functional receipts; the
wall-clock figures in §3.6 are reported as the shape of the race, never as a timing claim
(D-592). Both worktrees are removed at the end (§7).

**COUNTS**: 0 BLOCKING, 0 MAJOR, 2 minor. All seven remedies CLOSED by execution. The
whole crate, clippy, fmt and gate 20 are clean at `4fab9ed`.

**READ, in order**: `CLAUDE.md`; both round-1 reports; `arc3_ledger.md` from §1f to its end;
D-589..D-595; the design at revision 10 (`git diff 9c4366c 4fab9ed` on it, in full); the
mutation receipt; `git diff 9c4366c 4fab9ed` in full (16 files; on the cache path one string
in `capture.rs`, T3 and T4 in `label_cache_tests.rs`); `usage.rs::capture_tail`,
`capture.rs::{ask,run,refuse_census_under_cache}`, `label_cache.rs`, the stub's stray sites,
`cold_label_check.py:85,283-288`, T8; the pre-IMPL `ask` at `adb2012` for the M16 restore.

---

## 1. BASELINE AND GATES AT `4fab9ed`

| gate | command (in the worktree) | output | exit |
|---|---|---|---|
| whole crate | `CARGO_TARGET_DIR=… cargo test -p pistol-arena --locked` (log `scratch/crate_test0.log`) | **36 `test result:` lines, 36 `ok`, 0 `FAILED`/`panicked`**; `label_cache_tests` `9 passed; 0 failed` (35.34 s); `cold_label_check_tests` `11 passed; 0 failed` | 0 |
| clippy | `cargo clippy -p pistol-arena --all-targets --locked -- -D clippy::all` | `Checking pistol-arena v0.0.1 … Finished dev profile … in 5.81s` | 0 |
| fmt | `cargo fmt --all --check` | (no output) | 0 |
| gate 20 | `bash tools/governing_citation_check.sh` | `wp21_label_cache_design.md: 19 citation(s) checked, 0 unreproduced`; `wp21_prereg.md` 25/0; `wp21_throughput_prereg.md` 12/0; `sealbot_anchor_v3_prereg.md` 7/0; `DESIGN_CITATION_CHECK_DONE` | 0 |

The crate run was taken beside no other job of mine; the implementer's mutation job was
finishing on the box (functional receipt only).

---

## 2. THE SEVEN REMEDIES — DISPOSITION

Command shape for every test row: `CARGO_TARGET_DIR=/home/tom/pistol-wt/review-impl2/target
cargo test -p pistol-arena --locked --test <suite> <test> -- --exact` (driver `scratch/mut.py`,
logs `scratch/out/<mutant>_<row>_<i>.log`). Exit 101 = the test panicked; the "output" column
quotes the log's `test result:` line and, for a kill, the panic's own message.

| # | remedy | verdict | what I ran | output |
|---|---|---|---|---|
| 1 | Round-1 BLOCKING: T3 reads the refusal's first line; both X1 mutants must die | **CLOSED** | (a) `usage.rs:116` or-pattern arm → `["\0"] =>` (a NUL cannot be an argv word), T3; (b) `\| ["--label-cache", "--census"]` dropped, T3; (c) correct code, T3 | (a) `FAILED. 0 passed; 1 failed`, exit 101, `label_cache_tests.rs:296:9: census-first: the refusal's own line must name both words: arena: --config and --out are both required, …`; (b) `FAILED. 0 passed; 1 failed`, exit 101, `:296:9: cache-first: the refusal's own line must name both words: arena: --config and --out are both required, …`; (c) `ok. 1 passed; 0 failed` (8.09 s). By execution on the shipped binary over the §3.6 report: `--census --label-cache` and `--label-cache --census` → exit 2, 0 files, stderr ONE line `arena: --label-cache with --census is refused: a cache hit performs no search and emits no census row, …`; `--bogus` → exit 2, 96 stderr lines, first line `arena: --config and --out are both required, …` — the first line separates the arm from the catch-all, which is the property T3 now pins |
| 2 | Round-1 m1: the X1b string's 14-space run | **CLOSED** | `git show 4fab9ed:…/capture.rs \| sed -n 322,330p \| grep -c '[^ ]  \+[^ ]'` vs the same at `9c4366c`; the built binary: `/usr/bin/grep -a -c 'emits no census row, so a cached census capture would under-report firings at exit 0' target/debug/arena`; `/usr/bin/grep -a -o -E '.{0,50}census  +.{0,40}'` | source: **0** at `4fab9ed`, **1** at `9c4366c` (the string is now `… no census row, so a \` + continuation); binary: the single-spaced sentence present (2 hits, the CLI's and the seam's); the only `census` + two-space runs in the binary are the USAGE table's column alignment (`  --census  ask each label at the engine's census to …`, ×3), not the refusal |
| 3 | Round-1 m2: the mutation receipt exists and its rows reproduce | **CLOSED** (one minor beside it, N1) | `docs/experiments/wp21_label_cache_MUTATION.md` read; the receipt's `git grep` enumeration re-run at `4fab9ed` and sorted `LC_ALL=C`; five rows re-taken by its own edit descriptions plus both X1 rows (row 1 above): M03 `lookup`/`insert` keyed on the sorted stones' `Debug` (`stones_of` made `pub(crate)`) → T5, then T1; M07 the raw `(totals, bestmove)` inserted before `normalise` → T1; M09 `asks: self.counts.records` in `into_counts` → T2; M16 the guard block deleted from `run` and the `adb2012` block restored in `ask` → T4 ×3 (row 6b); M21 `MIN_SAMPLED = 0` → T8; then T8, T5, T1, T2 on restored code | The receipt exists (84 lines, 21 rows, two log digests, the enumeration). **M03**: T5 `FAILED`, exit 101, `:513:5: Y's transposed prefix is a stone-set collision: {asks: 22, hits: 32, key_full_collisions: 3, key_pos_collisions: 0, records: 54}` (a wrong-key hit: `asks` 22 < the correct 23); T1 `ok` (the honest stub answers a transposition alike — a row the receipt does not claim). **M07**: T1 `FAILED`, `:185:5: the cached capture is not byte-identical to the uncached one (identity)`. **M09**: T2 `FAILED`, `:248:5: cached, the engine was asked once per record, so the cache never answered: {asks: 54, hits: 31, records: 54}`. **M16**: T4 `FAILED` 3 of 3 (row 6b). **M21**: T8 `FAILED`, `cold_label_check_tests.rs:433:5: nine sampled hits is below the registered floor and must be a VOID (exit 2), not a pass: exit Some(0)`. Controls on restored code: T8, T5, T1, T2 all `ok. 1 passed`. Enumeration at `4fab9ed`: the same 22 sites; `capture.rs`'s eight are one line lower than the receipt's (`:348,:357,:364,:371,:374,:385,:397,:401`) because the X1b string became two lines — the receipt states its lines are at `9c4366c`, so this is consistent by its own header. Every row I re-took agrees with the receipt's verdict |
| 4 | Round-1 m3: the ledger's `cold_label_check_tests` count | **CLOSED** | `grep -c '#\[test\]' crates/pistol-arena/tests/cold_label_check_tests.rs`; the crate log; `/usr/bin/grep -n '12 passed\|holds eleven tests' docs/experiments/arc3_ledger.md` | file: **11**; log: `11 passed; 0 failed`; ledger `:1260` still carries the original *"12 passed"* (an append-only record) and `:1346` records the correction — *"this ledger's `cold_label_check_tests` 12 passed was wrong — the file holds eleven tests and both runs print 11"* |
| 5 | Round-1 m4: design row 4 names the arm's site | **CLOSED** | `/usr/bin/grep -n '^\| 4 \|' docs/experiments/wp21_label_cache_design.md` | row 4 now reads *"… as landed — the capture arm's TAIL parsed by `crates/pistol-arena/src/usage.rs`'s `capture_tail`, where the four command-line vocabulary helpers moved to keep `bin/arena.rs` under rule 9's cap"* and its catch-all reason is corrected to *"whose OWN sentence names neither word — the usage text it appends names every word this program has, so T3 reads the refusal's first line"*. The code: `usage.rs:111` `capture_tail`, `:116` the one or-pattern arm, called from `bin/arena.rs:61` |
| 6 | Red-team F1 (MAJOR): T4's cached arm is five runs, one refusal required | **CLOSED** — (a) 5/5 pass under load; (b) mutant 0/3 pass, deterministic; (c) §3 and D-595 state the numbers and the flip; (d) sound at the measured rates, with the bound stated in §3.6 | (a) T4 ×5 on correct code with the load loop running (loadavg 6.9→8.2 over the five); (b) M16 applied, T4 ×3 under load (loadavg 7.2–8.3); (c) `sed -n 120,160p` of the design + D-595; (d) the by-hand loop of §3.6 | (a) `ok. 1 passed` ×5 — 41.78 s, 36.82, 52.78, 31.82, 31.96 (each ~19 s unloaded at round 1; five captures now). (b) `FAILED. 0 passed; 1 failed` ×3, exit 101 each, every time at `label_cache_tests.rs:474:5: cached, every prefix of game 1 is a hit, and the guard at a hit is what refuses; none of five runs did, which is what the un-hoisted guard produces every time` — i.e. all fifteen mutant captures exited 0 with a file and were accepted by the else-branch, and the final count assertion killed the run. (c) design §3 revision 10: *"the cached run exited 0 in **2 of 20** runs under `cargo test` load and **0 of 20** unloaded"*, *"the cached run completed in 9 of 20 loaded and 8 of 10 unloaded runs where the uncached pass refuses"* (the exit case), *"T4's cached arm is FIVE runs, at least one refusing within game 1 … a flake bound of one in ten to the fifth under load on the correct side (D-595)"*; D-595 records the decision, the architect default (the red team's load read as CI's), and the flip: *"a CI flake of the five-run form, or a wrong byte in any completing run, in which case the drain is designed"*. (d) §3.6 |
| 7 | Red-team F2/F3 (minor): §3 names the exit case; F3 recorded | **CLOSED** | `/usr/bin/grep -o 'EXITS after the last miss' design §3`; `/usr/bin/grep -n 'F3 (minor)' arc3_ledger.md` and `-o 'recorded as unreproduced and pre-existing'` | §3: *"The same window covers an engine that EXITS after the last miss: EOF reaches `try_recv` later than a buffered line, and the cached run completed in 9 of 20 loaded and 8 of 10 unloaded runs …"*; ledger `:1373` F-2.2: *"F3 (minor): the time-of-check loses at a miss too, in both modes; the wrong-record consequence tried ninety times and never produced — recorded as unreproduced and pre-existing"* |

---

## 3. THE T4 RESIDUAL, MEASURED BY ME (remedy 6d)

### 3.6 The by-hand loop

T4's own shape rebuilt from the red team's Appendix B with the worktree's `4fab9ed` binaries
(`scratch/fx/rate.sh`, log `scratch/fx/rate.log`): book `b1` = `start moves 0,0 0,1/1,-1
1,0/4,0 2,0/3,-1`, `turn_cap 8`, `nodes 5000`, one stub on both seats. Honest play, uncached
capture: `captured 18 position(s) from 2 game(s)`, `label cache off: asks 18 records 18` ⇒
`P0 = 9`, `n = 10`. Play with `stray_after_newgame 10`: both games `capped`, no forfeit.
Uncached capture of that report: exit 2, no file, `game 1, turn 0: the engine spoke before it
was asked (Line("bestmove -3,0/-3,1"))` — the same line the red team quotes.

Then the cached capture in a loop, the load loop running (its iteration 3, loadavg 8.2 at the
start, 11.2 at the end, 16 cores):

```
loaded/on x20: exit0=1 refused=19 other=0 :: 2(3463ms) 2(3496ms) 2(3301ms) 2(3291ms) 2(3184ms) 2(3194ms) 2(5901ms) 2(7284ms) 0(6876ms) 2(3540ms) 2(3179ms) 2(3245ms) 2(3731ms) 2(4231ms) 2(4418ms) 2(5412ms) 2(5261ms) 2(4382ms) 2(3191ms) 2(3182ms)
   refusal turns: 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   exit-0 file vs off-mode honest capture bodies: IDENTICAL   (arena: label cache on: asks 9 records 18 hits 9)
```

and after `STOP_LOAD`, the load loop's `LOAD_STOPPED` and 20 s of quiet (`pgrep -c cargo` = 0,
loadavg 5.2 falling):

```
unloaded/on x10: exit0=0 refused=10 other=0 :: 2(3147ms) 2(3168ms) 2(3159ms) 2(3173ms) 2(3173ms) 2(3149ms) 2(3177ms) 2(3176ms) 2(3144ms) 2(3162ms)
   refusal turns: 0 0 0 0 0 0 0 0 0 0
```

Every refusal, loaded or not, landed at **game 1, turn 0** — the first hit's guard, never a
later one — so on this box the race is "queued before the first hit or never": once the
reader thread misses the first check it stays behind for the remaining eight hits
(≈ microseconds of main-thread work), which is why the exit-0 runs are the stretched ones
(6.9 s here; 6.5–13 s in the red team's list). The one exit-0 file's record body is
byte-identical to the honest capture's (`diff` of the non-`#` lines: empty), `hits 9`.

### The bound, from MY numbers

Per-capture exit-0 rate under load: **1/20 = 0.05** measured here (red team 2/20; pooled
3/40 = 0.075). Unloaded: 0/10 here, 0/20 there. Five independent runs all exiting 0, at
the point estimate: **0.05⁵ = 3.1 × 10⁻⁷** (pooled 0.075⁵ = 2.4 × 10⁻⁶; the design's 0.1⁵ =
10⁻⁵). At the one-sided 95 % Clopper–Pearson upper bound on the per-run rate — the honest
reading of a 1-in-20 observation — p ≤ 0.216 and **p⁵ ≤ 4.7 × 10⁻⁴** (pooled 3/40: p ≤
0.183, p⁵ ≤ 2.0 × 10⁻⁴; the red team's 2/20 alone: p ≤ 0.283, p⁵ ≤ 1.8 × 10⁻³).

**Is "at least one of five" sound?** Yes, at the measured rates: the mutant side is
deterministic (0 of 15 mutant captures refused, 0 of 3 mutant T4 runs passed; the red team's
20 of 20 and round 1's 3 of 3 agree — under the un-hoisted guard game 1 performs no channel
operation), and the correct side's flake probability is between 10⁻⁷ (point) and 10⁻³
(conservative upper bound) per T4 execution under a load like this one. Two caveats the
design's *"one in ten to the fifth"* elides, neither a finding: (i) **the five runs are
consecutive and the exit-0 runs coincide with the stretched wall-clocks**, so they are not
independent draws — a sustained load heavier than the one measured (loadavg 8–11 on 16 cores;
`ci.sh` gate 3's parallel test binaries are of this order) would raise the per-run rate and
the bound with it, and p⁵ is the bound only while p stays near 0.1; (ii) the design's figure
is the point estimate at the red team's rate, not an upper confidence bound. D-595's flip
clause — *"a CI flake of the five-run form"* — is exactly the observation that would show the
per-run rate has moved, so the assertion is sound as registered and its failure mode is
already the decision's flip.

---

## 4. WHAT CHANGED ON THE CACHE PATH BETWEEN `9c4366c` AND `4fab9ed`

`git diff 9c4366c 4fab9ed -- crates/pistol-arena/src`: **one file, 2 insertions, 1
deletion** — the X1b string re-wrapped. No mechanism moved: `label_cache.rs`, `usage.rs`,
`passes.rs`, `bin/arena.rs`, `bin/stub_engine.rs`, `channel.rs`, `seats.rs` are
byte-identical. `label_cache_tests.rs`: T3's assertion reads `stderr.lines().next()`; T4's
cached arm is the five-run loop, each run either refusing within game 1 (`!out.exists()`
asserted) or completing (`status.success() && out.exists()` asserted), then `>= 1` refusals.
The docs: design revisions 8–10, D-595, the receipt, the two round-1 reports, the ledger.

---

## 5. NEW FINDINGS

### N1 — minor. At `4fab9ed` the ledger promises a re-take "below" that the revision does not contain, and the receipt's X3 rows are recorded under the one-run T4

`arc3_ledger.md` at `4fab9ed` ends (`:1382-1383`): *"T4 run once here: green. M15/M16/M20
re-taken at this revision: below."* — and nothing follows. `wp21_label_cache_MUTATION.md`
at `4fab9ed` is headed *"revision 8"* and records M15/M16/M20 as DIED at `9c4366c`, under
the ONE-run T4 the design then had; no row is taken under the five-run form at the reviewed
revision. CLAUDE.md Closure: a test claim cites its log. Reproducer: `git show
4fab9ed:docs/experiments/arc3_ledger.md | tail -2`; `git show
4fab9ed:docs/experiments/wp21_label_cache_MUTATION.md | grep -n 'revision\|4fab9ed'` → the
first line's `revision 8`, no `4fab9ed`. **My own re-take of M16 under the five-run T4 is
row 6(b): dead 3 of 3.** Read-only, after the fact: `036b4a4` (the live HEAD at the end of
this review, one commit past `4fab9ed`) edits exactly these two files — *"T4's three mutants
re-taken at `4fab9ed` … M15, M16, M20 all DIED, `artifacts/arc3r_mutation_4fab9ed_X3.txt`"*
— which is the record the sentence points at. Not reviewed here; the finding stands at the
reviewed revision and is discharged by the implementer's own commit if that log's digest
reproduces.

### N2 — minor. T4's else-branch accepts a completing cached run on its exit status alone; the bytes of that run are what D-595 flips on, and no test reads them

`label_cache_tests.rs:462-467`: a cached run that does not refuse is accepted by
`on.status.success() && out.exists()`. D-595's flip clause is *"a wrong byte in any completing
run"*; the design's §3 says of the residual *"every record a real answer, the file
byte-identical"*. Nothing in the suite examines a completing run's bytes — the uncached
capture of the same report refuses, so T4 has no in-test reference file. The property holds
by execution (the red team's two exit-0 files; my one, §3.6: record body identical to the
honest play's uncached capture, which the test already produces as `stray-honest` and could
compare against — the stray stub's play is asserted move-identical to the honest one at
`:407-410`). **Suggested fix, UNEXECUTED**: in the else-branch, read `out` and assert its
record body equals an uncached capture of the honest report taken once before the loop.
Minor because the residual's bytes were verified by hand three times across two reviews and
the flip clause names the wrong byte as a decision, not a test; listed because the assertion
as written would accept a completing run whose bytes were wrong.

---

## 6. ATTACKS ATTEMPTED AND REJECTED

- **T3's first line could be something other than the refusal** (a stub banner, a warning):
  X1 fires in `capture_tail` before any spawn; by execution both orders write exactly one
  stderr line and the catch-all's first line is its own sentence (row 1). Rejected.
- **The five-run T4 passes vacuously under the mutant because the else-branch accepts exit 0**:
  the else-branch accepts the run but does not count it; the `>= 1` assertion at `:474` is what
  killed M16 three times. Rejected.
- **The five-run T4 could pass on correct code with a refusal at a place other than a hit's
  guard** (e.g. `no totals line` in game 1): the cached run never asks in game 1, so the only
  refusing site is the hoisted guard; 29 of 29 refusals across both loops were `game 1, turn 0:
  … spoke before it was asked`. Rejected.
- **The load I applied is lighter than the red team's**: theirs was one `cargo test` in the
  same worktree at loadavg 10–12; mine a looped `cargo test` in a second worktree at 7–11, and
  the measured rate (1/20) sits beside theirs (2/20). Not a distinction the bound turns on
  (§3.6's caveat (i) covers the heavier case). Rejected as a finding.
- **The receipt's line numbers are stale**: they are at `9c4366c` by its own header; the
  22-site enumeration is the same set at `4fab9ed`. Rejected.
- **M03 surviving T1 contradicts the receipt**: the receipt names T5 for M03 and T5 kills it;
  round 1 recorded the same T1 pass and its reason (the honest stub's move is a function of
  the stone set). Rejected.
- **Suite time**: T4 now takes 32–53 s under load (five captures) against ~19 s at round 1; an
  observation, not a correctness or requirement gap; gate 3's wall-clock is not a registered
  quantity. Not a finding.

---

## 7. HEAD MATCH, AND THE WORKTREES

`git -C /home/tom/Projects/HeXO-AlphaBeta rev-parse HEAD` at the end of this review:
**`036b4a46d13e69ef785e6e7e074f34a06eca9415` — the live tree does NOT match `4fab9ed`**; the
one commit between is `036b4a4` *"T4's three mutants re-taken dead under the five-run form;
round 2 of REVIEW-impl dispatched at 4fab9ed"* (two docs, N1's record). Nothing of mine.

Worktrees `/home/tom/pistol-wt/review-impl2` and `/home/tom/pistol-wt/review-impl2-load`
are removed after this file is written (`git worktree remove --force`); neither held
anything under `artifacts/` or beyond the tracked `sessions/`; the implementer's
`/home/tom/pistol-wt/mutation` and the CI worktree were not touched. Scratch evidence (the
driver, 26 per-run logs, the by-hand fixture and its 30-run log) goes with the worktree; the
receipts quoted above are the record.

---

**VERDICT: PASS — 0 BLOCKING, 0 MAJOR, 2 minor** at `4fab9ed`. All seven remedies CLOSED by
execution: both X1 mutants die at T3's first-line assertion; the X1b string is single-spaced
in source and binary; the receipt's rows reproduce (M03, M07, M09, M16, M21 and both X1 rows
re-taken); the ledger's count is corrected; row 4 names `usage.rs::capture_tail`; T4's
five-run arm passes 5 of 5 under load and kills the un-hoisted guard 3 of 3, with the residual
measured here at 1 of 20 loaded and 0 of 10 unloaded and a flake bound between 3 × 10⁻⁷ and
5 × 10⁻⁴ per execution; §3 and the ledger carry the exit case and F3. The two minors are the
ledger's dangling *"below"* at the reviewed revision (filled by `036b4a4`, unreviewed) and the
completing-run bytes no test reads.
