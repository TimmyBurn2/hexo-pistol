# WP-2.1 label cache — RED-TEAM report

**Revision under review**: `9c4366c030d0c48f85f3d5a09066e149ba978f9b` (short `9c4366c`), branch
`dev`. **Live-tree HEAD when this report was written**: `5b17132f41228b1dcf1a20056a9bd5f210a75a4b`
on `dev` — NOT the reviewed revision; it moved twice during this run (`037b196` when the
work began, `5b17132` at the end). `9c4366c` is its ancestor (four commits between:
`eb3dc6c`, `610225b`, `037b196`, `5b17132`). `git diff --stat 9c4366c..5b17132` touches one
source file on the cache path — `crates/pistol-arena/src/capture.rs`, 3 lines, the X1b
refusal STRING re-wrapped (`"… emits no census              row …"` → `"… emits no census
row …"`), no mechanism — plus `label_cache_tests.rs`, `wp21_assemble_tests.rs`,
`tools/wp21_assemble.py` and ten documents. Everything else this report exercised
(`label_cache.rs`, `passes.rs`, `bin/arena.rs`, `usage.rs`, `channel.rs`, `seats.rs`,
`bin/stub_engine.rs`, the rest of `capture.rs`) is byte-identical at `5b17132`.

**Where it ran**: detached worktree `/home/tom/pistol-wt/redteam` at `9c4366c`,
`CARGO_TARGET_DIR=/home/tom/pistol-wt/redteam/target`, debug build, `--locked`. A second
detached worktree `/home/tom/pistol-wt/redteam-mutant` at the same SHA carried the one X3
mutant (§10) in its own target dir. Both were removed afterwards; neither held anything under
`artifacts/`, and `sessions/` in each was the tracked directory as committed, nothing added. Scratch lived at `/home/tom/pistol-wt/redteam/scratch` and is
gone with the worktree; Appendix B rebuilds it. `cargo test -p pistol-arena --locked` in
the red-team worktree (with Appendix A's stub): 36 suites, 242 passed, 0 failed — cited as
the load source for §10, not as a gate claim (the stub was augmented).

**Two stub builds**. Classes 1, 2, 3 and 5 ran against the stub AS COMMITTED (binary sha256
`7b2fbe09592f8f6764a094164eed9f97b9a4a79ed1ede5c56545d8e9b9580eff`). Classes 4, 6, 7, 9
and 10 ran against the stub with six behaviours added in the worktree (Appendix A; sha256
`13f3dae66c08ee4351f0c0cf3c0a98ae6b3b9f79fa0cb9272304f82977d4ff53`). The arena binary was
the committed source throughout except the mutant arm of §10. The added behaviours are
adversarial ENGINES, not edits to the code under review: design §3 requires the capture's
two seats to attest ONE engine, not to be `pistol`, so an engine that misbehaves once, on
the answer after its n-th `newgame`, is a legitimate input — it is exactly the shape the
shipped `stray_after_newgame` already has.

## VERDICT

**FAIL — 0 BLOCKING / 1 MAJOR / 2 minor.**

BLOCKING was defined by the dispatch as a wrong answer, a wrong byte, a silent skip, or an
exit-0 that should have been a refusal. **None was found.** Every cached capture that exited
0 was byte-identical to the uncached capture of the same report (`cmp` on the files, or
`diff` on the record bodies where a hand-edited report changed only a header), every
refusal the uncached pass made before writing, the cached pass made too at the same prefix
or earlier, and no run of either mode wrote a partial file or printed a counts line before
its file. The MAJOR is a MEASURED gap between design §3's residual sentence and the run —
it changes what the mutation receipt and T4 may conclude, which is D-291's test — and it
is the operator's to weigh, not this report's to soften. If the operator reads F1 as the
documented residual it is, the verdict is PASS at 0/0/3.

## Findings

### F1 — MAJOR. The X3 residual is not "tens of microseconds": under load the cached T4 arm exits 0 in 2 of 20 runs, and T4 is the only row that kills the X3 mutant.

Design §3, revision 7: *"T4's cached assertion holds unless the reader thread is preempted
between its two sends for the whole of game 1, a window ESTIMATED at tens of microseconds"*.
Measured with the committed arena and the committed stub behaviour (`stray_after_newgame
10`, `n = P0 + 1`, T4's own shape), under a concurrent `cargo test -p pistol-arena` in the
same worktree (load average 10–12 on this box):

```
T4/on x20: exit0=2 refused=18 other=0 ::  2(3287ms) 2(3216ms) 2(3669ms) 2(7254ms) 0(6557ms) 2(7441ms) 2(8332ms) 2(8014ms) 2(13461ms) 0(6590ms) 2(6057ms) 2(4666ms) 2(3998ms) 2(3306ms) 2(3217ms) 2(3215ms) 2(3311ms) 2(3236ms) 2(3198ms) 2(3213ms)
   last stderr: arena: Config: `capture`: game 1, turn 0: the engine spoke before it was asked (Line("bestmove -3,0/-3,1"))
```

and with no load (load average ≈ 2, nothing else of mine running):

```
T4-noload/on x20: exit0=0 refused=20
```

The two exit-0 runs wrote a capture whose records are identical to the honest capture's
(`diff` of the bodies: empty), with `arena: label cache on: asks 9 records 18 hits 9`; the
stray `bestmove` was still in the pipe when `with_seats` shut the engine down. The same
window closed an `info`-shaped stray at exit 0 once in the first eleven cached runs of that
shape (§4) — the window is the reader thread's, whatever the line says.

Why MAJOR and not BLOCKING: the design names the residual, D-593 declines the drain that
would close it, and the file is correct — this is the documented non-refusal, not a new
one. Why MAJOR and not minor: (a) the estimate is wrong by three to four orders of magnitude
under the load `tools/ci.sh` produces itself (its test binaries run in parallel), and D-291
makes an estimate that could have been measured a finding; (b) T4
(`a_stray_line_in_the_pipe_while_the_cached_run_serves_hits_is_refused_in_both_runs`)
asserts `!out.exists()` on the cached arm and is therefore FLAKY at about one in ten under
load; (c) the mutation set's X3 row — *"the guard left inside `ask` and not hoisted … dies
at T4"* — dies with probability ≈ 0.9 per run under load, and the receipt landed at
`037b196` reads that row as a kill.

Reproducer (Appendix B's harness; the committed stub suffices):

```
cd /home/tom/pistol-wt/redteam/scratch && source ./h.sh
book b1.txt "start moves 0,0 0,1/1,-1 1,0/4,0 2,0/3,-1"
play s10 "stray_after_newgame 10" $PWD/b1.txt          # P0 = 9 asked prefixes; n = P0 + 1
(CARGO_TARGET_DIR=/home/tom/pistol-wt/redteam/target cargo test -p pistol-arena --locked >/dev/null 2>&1 &)
./loop.sh $ARENA $S/s10/report.txt on 20 T4             # exit0 vs refused, under load
```

What would settle it is IMPL's and the operator's, not this report's: T4 makes the stray
unmissable, or §3 carries the measured rate in place of the estimate and the receipt's X3
row says "dies at T4 with probability p under load" and the operator says whether that is a
kill.

### F2 — minor. The same window covers an engine that DIES after the last miss; §3's residual sentence names only a line.

`exit_after_newgame 10` (Appendix A): the engine exits right after the answer to game 0's
last ask. The uncached pass refuses deterministically — its next ask blocks in `receive`
and gets `Closed` — while the cached pass sees `Closed` only if the reader thread observed
EOF before its nine `try_recv` guards ran:

```
exit10/off x10:        exit0=0 refused=10     (game 1, turn 0: the engine closed its pipe)
exit10/on  x20 (load): exit0=9 refused=11     (refusals at game 1, turn 0..4: "spoke before it was asked (Closed)")
exit10-noload/on x10:  exit0=8 refused=2
```

Not BLOCKING: every record is a real answer, the file is byte-identical to the honest
capture, and the engine died AFTER the last position the run needed — the uncached pass
refuses only because it needs one more ask. Listed because a death is caught by the same
`try_recv` with the same race as a line, at a higher rate (EOF is delivered after process
teardown, later than a buffered line), and §3 does not say so.

### F3 — minor. The guard's time-of-check is lost at a MISS too, in both modes; the wrong-record consequence was NOT reproduced.

`stray_after_newgame 1` (stray after game 0, turn 0's answer): the uncached run refused
with *"game 0, turn 1: the search closed with no totals line this driver recognised"* —
the guard at turn 1 ran before the reader thread had queued the stray, and `ask` read the
stray as the answer. The cached run of the same report refused with *"game 0, turn 1: the
engine spoke before it was asked (Line("bestmove 0,0"))"*. Both refuse; which sentence
refuses is the race.

The consequence the design does not state: a stray shaped as a WHOLE answer (`info totals`
then `bestmove`; Appendix A's `double_answer_after_newgame`) that loses the race at a miss
is read as that miss's answer — a wrong record — and is refused only if a LATER guard sees
the real answer. At the final miss of the run there is no later guard. Measured at the
final miss of each mode (`n = 18` uncached, `n = 9` both) and at game 0's last miss:

```
da18/off x30: exit0=0 refused=30   (game 1, turn 8: spoke before it was asked (Line("info totals …")))
da9/on  x20:  exit0=0 refused=20   (game 0, turn 8)
da9/off x20:  exit0=0 refused=20   (game 0, turn 8)
da10/on x20:  exit0=0 refused=20   (game 1, turn 0)
```

The guard won 90 of 90 at a miss; the wrong-byte path exists in the code and was observed
to be enterable (the `s1` sentence) but no wrong byte was produced in any run. Recorded as
unreproduced at the wrong-byte level, and as a pre-existing property of the capture pass
that the cache neither adds to nor removes — the shape is identical in both modes.

## The classes

Notation: `off` = `arena --capture <report> --out <out> --label-nodes 5000`; `on` = the same
with `--label-cache` last. `P0` is game 0's asked-prefix count: 9 for book `b1` (four-turn
opening, `turn_cap 8`). Reports named `sN` were played by `stray_after_newgame N` on both
seats, where `n >= 2` never fires (one `newgame` per spawn) and the report is an honest one.

### 1. A forfeit report — PASS

Book `b2` (the first two committed openings), each behaviour on BOTH seats, `turn_cap 8`,
`hang_timeout_ms 30000` (1500 for `hang`):

| behaviour | play | uncached | cached | same? |
|---|---|---|---|---|
| `demands_newgame_per_ask` | every game forfeits at turn 6, exit 1 | exit 0, `asks 28 records 28` | exit 0, `asks 12 records 28 hits 16 key_pos_collisions 0 key_full_collisions 0` | `cmp` identical |
| `refuses_go` | forfeits at turn 4 | exit 2, no file: `game 0, turn 0: the engine refused: error Refused: …` | identical line, exit 2, no file | yes |
| `illegal` | forfeits at turn 4 | exit 2: `game 0, turn 0: the search closed with no totals line` (the stub writes `bestmove 0,0` alone) | identical | yes |
| `exit` | forfeits at turn 4 | exit 2: `game 0, turn 0: the engine closed its pipe` | identical | yes |
| `hang` (1500 ms) | the RUN aborts, report kind `arena_report_aborted` | exit 2: *its first token is `arena_report_aborted` …* | identical | yes |

A hang UNDER the capture needs an engine that hangs only when asked an n-th time
(`hang_after_newgame 10`): both modes `Hung: engine a answered nothing for 1500 ms on
opening 0, turn 8`, exit 2, no file — turn 8 is a miss in both.

### 2. A rule-4 win — PASS

Book `bwin` = the design's T6(b) opening (`start moves 0,0 0,5/2,5 -4,0/-3,0 4,5/6,5
-2,0/-1,0`); committed stub `honest`; both games end `p1_win` at turn 7 with the single
stone `-5,0`:

```
cap win/off: exit 0, captured 14 position(s) from 2 game(s), label cache off: asks 14 records 14
cap win/on : exit 0, label cache on: asks 7 records 14 hits 7 key_pos_collisions 0 key_full_collisions 0 fold_ms 0
game 0: records 7, max turns_played 6 ; game 1: records 7, max 6 ; records at turns_played 7: 0 ; cmp: IDENTICAL
```

The terminal position is asked in neither mode; game 1's seven prefixes are all hits.

### 3. The T5 fixture, late-differing positions, an uncanonical pair — PASS

X, Y, Z exactly as `label_cache_tests.rs` spells them, committed stub:

```
cap xyz/off: asks 54 records 54
cap xyz/on : asks 23 records 54 hits 31 key_pos_collisions 1 key_full_collisions 4 fold_ms 0 ; cmp IDENTICAL
k=4:  game 0 (X)  position start moves 0,0 0,5/2,5 1,0/2,0 4,5/6,5        bestmove -1,0/-1,1
      game 2 (Y)  position start moves 0,0 0,5/4,5 1,0/2,0 2,5/6,5        bestmove -1,0/-1,1
      game 4 (Z)  position start moves 0,0 -5,5/-5,7 0,1/0,2 -5,9/-5,11   bestmove -1,0/-1,1
```

`asks 23` = 9 (X) + 7 (Y, misses from k = 2) + 7 (Z, misses from k = 2): the transposed and
imaged prefixes were ASKED, not served. A wrong-key hit would read `asks` lower and, for Z,
put a `bestmove` in X's frame. (The honest stub answers Y's k = 4 with X's bytes because its
move is a function of the stone set; that is the engine, not the memo — Y's record carries
Y's own `position` line.)

Two openings differing only in the LAST turn's second stone (X and `… 0,1/1,2`): `asks 13
records 36 hits 23` = 9 + 4 hits + 1 miss at k = 5 for the second; `cmp` identical; the
two k = 5 records carry their own distinct `position` lines.

The honest xyz report with `1,0/2,0` rewritten `2,0/1,0` (`sed`, four occurrences): both
modes refuse at the loader, exit 2, no file — *`2,0/1,0` is not a turn: bad turn token
"2,0/1,0": a pair is written smaller cell first, lexicographic by (q, then r)*. There is no
second spelling of a turn for the memo to mis-key on.

### 4. A stray line — PASS on the class; F1, F2, F3 were measured here

Committed `stray_after_newgame n` over `b1` (P0 = 9):

| n | the stray lands | uncached | cached |
|---|---|---|---|
| 1 | after game 0 turn 0's answer (play: every game forfeits, `protocol_error`) | exit 2 `game 0, turn 1: … no totals line` (race lost — F3) | exit 2 `game 0, turn 1: the engine spoke before it was asked (Line("bestmove 0,0"))` |
| 2 | after game 0 turn 1's answer | exit 2 `game 0, turn 1: spoke before it was asked` | identical |
| 10 = P0+1 | after game 0's LAST answer (T4) | exit 2 `game 1, turn 0: spoke before it was asked` | same line — 18/20 under load, 20/20 without (F1) |
| 11 = P0+2 | after game 1 turn 0's answer | exit 2 `game 1, turn 1: … no totals line` | **exit 0**, `asks 9 records 18 hits 9`; record bodies identical to the honest capture |

n = 11 is not a finding: the cached run never asks an eleventh time, so the engine never
deviates. An engine whose misbehaviour is a function of how often it is asked is not an
input the cache promises parity over, and its file is the honest file.

Added shapes at n = 10 (Appendix A):

| shape | uncached | cached |
|---|---|---|
| `info depth …` stray in the answer's own write | exit 2 `game 1, turn 0: spoke before it was asked (Line("info depth_turns …"))` | same, 20/20 in the loop; 1 exit 0 in an earlier single run (F1's window) |
| whole answer doubled (`info totals` + `bestmove`, twice) | exit 2 `game 1, turn 0: spoke before it was asked (Line("info totals …"))` | identical, 20/20 |
| `bestmove` BEFORE its totals | exit 2 `game 0, turn 8: … no totals line` | identical (turn 8 is a miss in both) |
| `bestmove` stray written 300 ms late by a second thread | exit 0, 10/10 (the run ends first) | exit 0, 10/10, identical file — the last-prefix residual again |
| engine exits after the answer | exit 2 `(Closed)`, 10/10 | F2 |
| engine hangs at its next `go` | `Hung … turn 8`, exit 2 | identical |

`classify` never sees a stray at a hit: the hoisted guard refuses ANY queued line, `info`
included, before the lookup. `classify`'s `Ignore` arm is reachable only inside `ask`, at a
miss, in both modes — an `info` stray that arrives during an ask is dropped there in both.

### 5. `--census` — PASS

Over a `census_rows` report (committed stub):

```
--census                         exit 0; capture + census file; "captured 54 census row(s)"; label cache off: asks 18 records 18
(no tail)                        exit 0; asks 18 records 18
--label-cache                    exit 0; asks 9 records 18 hits 9
--census --label-cache           exit 2, no capture, no census file: "--label-cache with --census is refused: a cache hit performs no search and emits no census row, so a cached census capture would under-report firings at exit 0"
--label-cache --census           exit 2, the same line
--census --census                exit 2, the usage refusal
--census --label-cache --census  exit 2, the usage refusal
```

The library seam (`capture::run` with a sink whose `request` is `On` under
`LabelCache::On`) is T3b, which passed in the worktree's `cargo test`; not repeated.

### 6. A doubled or misspelled word — PASS

Each of these exited 2 with the usage refusal and left no file: `--label-cache
--label-cache`; `--label-nodes 5000 --label-nodes 5000 --label-cache`; `--label-cache`
before `--out`; `--label-cache` between `--out` and `--label-nodes`; `--label-cache=1`;
`--label-cache --`; `--label-cache ''`; `‐‐label-cache` (U+2010); `--label‐cache`;
`—label-cache` (em dash); `--Label-cache`; `--LABEL-CACHE`; `--label_cache`;
`-label-cache`; a leading space; a trailing space; a trailing TAB; a trailing newline.
`capture_tail` matches whole words against three literal patterns and the rest falls to
`usage_error()`; the `--out` claim comes after the parse, so there is nothing to abandon.

### 7. The memo itself — PASS

- Two openings sharing only `position start` and `position start moves 0,0` (§3's X/X′
  and the xyz fixture): the second opening's k = 0, 1 hits reproduce game 0's bytes —
  `cmp` identical.
- Game 1 a PREFIX of game 0 (the honest `b1` report hand-edited: `game 1 … result p2_win
  end forfeit … turns 5`, `moves 1` cut to five turns): `asks 9 records 15 hits 6`, `cmp`
  identical; game 1's six records equal game 0's first six with only the game index
  changed.
- The reverse — game 0 the SHORT forfeit, game 1 full, the memo built from the short game
  first: `asks 9 records 15 hits 6`, identical; game 1's k = 6..8 are the misses.
- Game 0 with ZERO moves (`turns 0`, `moves 0 ` with a trailing space — `moves 0` alone is
  refused as *"carries no `moves 0` record"*): `asks 9 records 10 hits 1`, identical.
- "One game, no hits at all": UNREACHABLE. The loader refuses an odd game count, and every
  game shares `position start`, so any readable report yields `hits >= 1` cached; the
  zero-move report above is the minimum.
- Two seats, different engines (`honest` vs `honest_last`, one binary): both modes exit 2
  before anything — *its two seats attest different engines: they differ at
  `config_sha256`* — no file.

### 8. The counters — PASS

`key_full_collisions >= key_pos_collisions` by construction: a sorted stone list already in
`key_pos` was inserted by an earlier miss, whose `canonical_form` went into `key_full` at
the same time. Observed (1, 4) on xyz and (0, 0) on every other report. Under `Off`,
`Memo::insert` returns before either set is touched and the line is `arena: label cache
off: asks A records R` — no such field, as every `off` line above shows. `fold_ms` is
printed only and read 0 on every run.

### 9. The counts line and the file — PASS

`passes::capture` writes and flushes the capture BEFORE its first `println!`; a refused
`run` returns `Err` before the write and `bin/arena.rs` abandons the O_EXCL claim. Every
refused run above: stdout 0 bytes (`wc -c s10/cap-on.out s10/cap-off.out` → 0, 0), no file.
A refusal after nine records have been built (`s10`) leaves nothing: the records live in
`out: Vec<CaptureRecord>` until `run` returns.

### 10. Timing — the residual, measured

Committed arena, cached, the `s10` report: 20 runs under `cargo test -p pistol-arena` → 18
refused, 2 exit 0; 20 runs with no load → 20 refused (F1). Each capture takes ≈ 3.2 s wall
in either mode on this box (measured on the honest report: off 3231 ms, on 3253 ms) — the
runs under load stretched to 3–13 s and the two exit-0 runs were among the stretched ones.

Mutant arm — `capture.rs` with the guard moved back inside `ask` (the un-hoisted X3 mutant,
applied by hand in `/home/tom/pistol-wt/redteam-mutant`, its own target dir, run against
the SAME report and the same stub binary):

```
MUT-T4/on x20 (under load): exit0=20 refused=0
MUT-T4/off x3:              exit0=0 refused=3      (game 1, turn 0: spoke before it was asked)
MUT-info10/on x5:           exit0=5 refused=0
MUT-honest/on x2:           exit0=2 refused=0
```

The mutant never refused on the cached arm, so per the dispatch the count is not a finding
against T4's premise; what F1 records is the committed arm's rate.

## Attacks attempted and rejected

- **A wrong-key hit**: not constructible. The memo is `BTreeMap<String, _>` on the exact
  `position` bytes, and `Turn`'s parser refuses the only re-spelling a pair has. Verified
  by the xyz `asks` arithmetic and the uncanonical-pair refusal (§3).
- **A hit that skips `no_tab`**: `no_tab` runs on every record after the `match`; the pair a
  hit returns already passed it at its miss. A TAB-writing engine's first TAB lands on a
  miss (`tab_totals` refuses at game 0, turn 0 in both modes — `capture_tests.rs` pins it).
- **`asks > records`**: unreachable at exit 0 — every failed ask refuses the run before
  `recorded()`; `saturating_sub` never saturates.
- **A partial file**: none possible — see §9.
- **A 1 ms `hang_timeout_ms`**: not run. A wall-clock refusal is a fact about the machine in
  both modes, and the cached run's fewer asks give it fewer chances to time out — an
  asymmetry the design already owns under D-159 and rule 4.
- **`late_stray` at 5–20 ms**: not run separately; at 300 ms both modes finish first, which
  is the last-prefix residual under another name.
- **An `info` stray consumed silently at a miss**: reachable in both modes only inside
  `ask` (`classify` → `Ignore`), identical in both; not cache-specific, not executed.

## Appendix A — the six added stub behaviours

`git diff` over `crates/pistol-arena/src/bin/stub_engine.rs` in the red-team worktree,
applied on top of `9c4366c`. They count `newgame`s exactly as the shipped
`stray_after_newgame` does and deviate once, on the answer that follows the n-th. (The
late-stray thread writes through `/proc/self/fd/1` because the crate forbids `unsafe`.)

```diff
diff --git i/crates/pistol-arena/src/bin/stub_engine.rs w/crates/pistol-arena/src/bin/stub_engine.rs
index 0b471d2..3795a2f 100644
--- i/crates/pistol-arena/src/bin/stub_engine.rs
+++ w/crates/pistol-arena/src/bin/stub_engine.rs
@@ -140,6 +140,18 @@ enum Behave {
     /// already doubles its `bestmove` does so at its FIRST `go`, a miss in any
     /// run, which is why this is a behaviour and not that script.
     StrayAfterNewGame(u32),
+    /// RED-TEAM: the stray is an `info depth` line rather than a `bestmove`.
+    StrayInfoAfterNewGame(u32),
+    /// RED-TEAM: the whole answer (totals + bestmove) is written twice.
+    DoubleAnswerAfterNewGame(u32),
+    /// RED-TEAM: the answer's `bestmove` precedes its totals line.
+    BestmoveFirstAfterNewGame(u32),
+    /// RED-TEAM: a `bestmove` stray written by another thread 300 ms after the answer.
+    LateStrayAfterNewGame(u32),
+    /// RED-TEAM: the process exits right after the answer.
+    ExitAfterNewGame(u32),
+    /// RED-TEAM: the `go` following the n-th `newgame` is never answered.
+    HangAfterNewGame(u32),
 }
 
 impl Behave {
@@ -164,14 +176,21 @@ impl Behave {
             "census_tab" => Behave::CensusTab,
             "census_rows_unasked" => Behave::CensusRowsUnasked,
             _ => {
-                let count = word.strip_prefix("stray_after_newgame ")?;
-                // The spelling is validated, not only the value: a count a
-                // receipt could not copy back is not a count (SHELL_CHECKLIST 8).
+                let (kind, count) = word.split_once(' ')?;
                 let n: u32 = count.parse().ok()?;
                 if n.to_string() != count || n == 0 {
                     return None;
                 }
-                Behave::StrayAfterNewGame(n)
+                match kind {
+                    "stray_after_newgame" => Behave::StrayAfterNewGame(n),
+                    "stray_info_after_newgame" => Behave::StrayInfoAfterNewGame(n),
+                    "double_answer_after_newgame" => Behave::DoubleAnswerAfterNewGame(n),
+                    "bestmove_first_after_newgame" => Behave::BestmoveFirstAfterNewGame(n),
+                    "late_stray_after_newgame" => Behave::LateStrayAfterNewGame(n),
+                    "exit_after_newgame" => Behave::ExitAfterNewGame(n),
+                    "hang_after_newgame" => Behave::HangAfterNewGame(n),
+                    _ => return None,
+                }
             }
         })
     }
@@ -438,7 +457,7 @@ fn serve(
     let mut stray_armed = false;
     for line in stdin.lock().lines() {
         let mut line = line.map_err(|io| format!("stdin: {io}"))?;
-        if let Behave::StrayAfterNewGame(n) = behave
+        if let Some(n) = after_newgame(behave)
             && line
                 .trim_start()
                 .starts_with(pistol_cli::protocol::NEW_GAME)
@@ -446,6 +465,14 @@ fn serve(
             new_games += 1;
             stray_armed |= new_games == n;
         }
+        if stray_armed
+            && behave_is(behave, "hang")
+            && line.trim_start().starts_with(pistol_cli::protocol::GO)
+        {
+            loop {
+                std::thread::sleep(std::time::Duration::from_secs(3600));
+            }
+        }
         if behave == Behave::CensusRowsUnasked
             && line.trim_start().starts_with(pistol_cli::protocol::GO)
         {
@@ -582,6 +609,47 @@ fn serve(
             .iter()
             .find(|answer| stray_armed && answer.starts_with(&bestmove_prefix))
             .cloned();
+        let armed_now = stray_armed && answers.iter().any(|a| a.starts_with(&bestmove_prefix));
+        if armed_now && !matches!(behave, Behave::StrayAfterNewGame(_)) {
+            let mut buffer = String::new();
+            let totals: Vec<&String> = answers.iter().filter(|a| a.starts_with(TOTALS_PREFIX)).collect();
+            let bests: Vec<&String> = answers.iter().filter(|a| a.starts_with(&bestmove_prefix)).collect();
+            match behave {
+                Behave::BestmoveFirstAfterNewGame(_) => {
+                    for b in &bests { buffer.push_str(b); buffer.push('\n'); }
+                    for t in &totals { buffer.push_str(t); buffer.push('\n'); }
+                }
+                _ => {
+                    for answer in &answers { buffer.push_str(answer); buffer.push('\n'); }
+                }
+            }
+            match behave {
+                Behave::StrayInfoAfterNewGame(_) => buffer.push_str("info depth_turns 1 seldepth 1 nodes 1 nps 1 time 0 score cp 0 pv 0,0\n"),
+                Behave::DoubleAnswerAfterNewGame(_) => {
+                    for t in &totals { buffer.push_str(t); buffer.push('\n'); }
+                    for b in &bests { buffer.push_str(b); buffer.push('\n'); }
+                }
+                _ => {}
+            }
+            out.write_all(buffer.as_bytes()).map_err(io_error)?;
+            out.flush().map_err(io_error)?;
+            stray_armed = false;
+            match behave {
+                Behave::LateStrayAfterNewGame(_) => {
+                    let stray = format!("{}\n", bests[0]);
+                    std::thread::spawn(move || {
+                        std::thread::sleep(std::time::Duration::from_millis(300));
+                        if let Ok(mut raw) = std::fs::OpenOptions::new().write(true).open("/proc/self/fd/1") {
+                            let _ = raw.write_all(stray.as_bytes());
+                            let _ = raw.flush();
+                        }
+                    });
+                }
+                Behave::ExitAfterNewGame(_) => return Ok(ExitCode::from(ENGINE_EXIT_CODE)),
+                _ => {}
+            }
+            continue;
+        }
         match stray {
             Some(stray) => {
                 // ONE buffer, ONE `write_all`: the locked stdout is line-buffered
@@ -612,6 +680,23 @@ fn serve(
     Ok(ExitCode::SUCCESS)
 }
 
+fn after_newgame(behave: Behave) -> Option<u32> {
+    match behave {
+        Behave::StrayAfterNewGame(n)
+        | Behave::StrayInfoAfterNewGame(n)
+        | Behave::DoubleAnswerAfterNewGame(n)
+        | Behave::BestmoveFirstAfterNewGame(n)
+        | Behave::LateStrayAfterNewGame(n)
+        | Behave::ExitAfterNewGame(n)
+        | Behave::HangAfterNewGame(n) => Some(n),
+        _ => None,
+    }
+}
+
+fn behave_is(behave: Behave, word: &str) -> bool {
+    matches!((behave, word), (Behave::HangAfterNewGame(_), "hang"))
+}
+
 /// The handshake deviations, which are line rewrites rather than control flow.
 fn deviate(answer: &str, behave: Behave) -> String {
     match behave {
```

## Appendix B — the harness

`scratch/h.sh` — write it, `chmod +x`, `source ./h.sh` from `scratch/`. `book`, `play`,
`cap` are the three verbs every class above uses; `$S` is the scratch root, `$ARENA` and
`$STUB` the worktree's debug binaries.

```bash
#!/usr/bin/env bash
# red-team harness: play a stub self-match and capture it, cached or not.
set -u
WT=/home/tom/pistol-wt/redteam
S=$WT/scratch
ARENA=$WT/target/debug/arena
STUB=$WT/target/debug/arena-stub-engine
STUB_SHA=$(sha256sum "$STUB" | cut -c1-64)

# book <out> <line>... : an openings fixture with a correct body digest
book() {
  local out=$1; shift
  local body=""
  for l in "$@"; do body+="$l"$'\n'; done
  local d; d=$(printf '%s' "$body" | sha256sum | cut -c1-64)
  printf '# a test fixture\n# body_sha256 %s\n%s' "$d" "$body" > "$out"
}

# play <tag> <behave> <book> [hang_ms] [turn_cap] [behave_b]
play() {
  local tag=$1 behave=$2 bk=$3 hang=${4:-30000} cap=${5:-8} behave_b=${6:-}
  local d=$S/$tag; rm -rf "$d"; mkdir -p "$d"
  printf '# a test instrument\nbehave %s\n' "$behave" > "$d/engine-a.toml"
  local cfg_b="$d/engine-a.toml"
  if [ -n "$behave_b" ]; then
    printf '# a test instrument\nbehave %s\n' "$behave_b" > "$d/engine-b.toml"; cfg_b="$d/engine-b.toml"
  fi
  local take; take=$(grep -vc '^#' "$bk")
  cat > "$d/arena.toml" <<CFG
schema_version = 2
[run]
openings_file = "$bk"
openings_take = $take
openings_skip = 0
turn_cap = $cap
n_workers = 1
hang_timeout_ms = $hang
[budget]
kind = "nodes"
value = 5000
[sprt]
elo0 = 0.0
elo1 = 4.0
alpha = 0.05
beta = 0.05
[engine_a]
label = "a"
binary = "$STUB"
binary_sha256 = "$STUB_SHA"
config = "$d/engine-a.toml"
[engine_b]
label = "b"
binary = "$STUB"
binary_sha256 = "$STUB_SHA"
config = "$cfg_b"
CFG
  "$ARENA" --config "$d/arena.toml" --out "$d/report.txt" > "$d/play.out" 2> "$d/play.err"
  echo "play $tag: exit $?  $(grep -E '^(game|verdict)' "$d/report.txt" 2>/dev/null | tr '\n' ';' | cut -c1-400)"
}

# cap <tag> <report> <name> [tail...]
cap() {
  local tag=$1 rep=$2 name=$3; shift 3
  local d=$S/$tag; mkdir -p "$d"
  local out=$d/cap-$name.txt
  rm -f "$out" "$d/cap-$name.census.txt"
  "$ARENA" --capture "$rep" --out "$out" --label-nodes 5000 "$@" > "$d/cap-$name.out" 2> "$d/cap-$name.err"
  local code=$?
  local ex=absent; [ -e "$out" ] && ex="present sha=$(sha256sum "$out" | cut -c1-16)"
  local cen=""; [ -e "$d/cap-$name.census.txt" ] && cen=" census-file=present"
  echo "cap $tag/$name [$*]: exit $code out=$ex$cen"
  sed 's/^/  stdout: /' "$d/cap-$name.out" | grep -v 'manifest' | head -6
  grep -v 'TEST INSTRUMENT' "$d/cap-$name.err" | sed 's/^/  stderr: /' | head -4
}
```

`scratch/loop.sh <arena-binary> <report> <on|off> <n> <label>`:

```bash
#!/usr/bin/env bash
# loop.sh <arena-binary> <report> <mode:on|off> <n> <label>
A=$1; R=$2; M=$3; N=$4; L=$5
S=/home/tom/pistol-wt/redteam/scratch
tail=""; [ "$M" = on ] && tail=--label-cache
e0=0; e2=0; other=0; codes=""
for i in $(seq 1 $N); do
  out=$S/loop-$L-$M.txt; rm -f $out
  t0=$(date +%s%N)
  timeout 60 $A --capture $R --out $out --label-nodes 5000 $tail > $S/loop-$L-$M.out 2> $S/loop-$L-$M.err
  c=$?; t1=$(date +%s%N); ms=$(( (t1-t0)/1000000 ))
  codes="$codes $c(${ms}ms)"
  case $c in 0) e0=$((e0+1)); [ -e $out ] && cp $out $S/loop-$L-$M.last0.txt;; 2) e2=$((e2+1));; *) other=$((other+1));; esac
done
echo "$L/$M x$N: exit0=$e0 refused=$e2 other=$other :: $codes"
echo "   last stderr: $(grep -v INSTRUMENT $S/loop-$L-$M.err | head -1 | cut -c1-140)"
```

## Closing

`git rev-parse HEAD` in the live tree at the end of this run: `5b17132f41228b1dcf1a20056a9bd5f210a75a4b` — not
`9c4366c`, which is its ancestor; the only source change between them on the cache path is
the one refusal string named in the header.

**VERDICT: FAIL — 0 BLOCKING / 1 MAJOR / 2 minor.** No wrong answer, wrong byte, silent skip
or undocumented exit-0 was found; the MAJOR is design §3's residual estimate, measured at
two exit-0 runs in twenty under load where the design estimated tens of microseconds, with
T4 and the X3 mutation row resting on it.
