# REVIEW-impl — WP-2.1 lever B, the label cache package. **VERDICT: FAIL — 1 BLOCKING, 0 MAJOR, 4 minor.**

**REVISION REVIEWED**: `9c4366c030d0c48f85f3d5a09066e149ba978f9b` (`9c4366c`, branch `dev`),
checked against `docs/experiments/wp21_label_cache_design.md` revision 7 and the IMPL
obligations of D-589. **HEAD MATCH: NO** — at the end of this review the live tree is at `037b196` (§12); the
worktree stayed at `9c4366c` throughout.

**WHERE**: a detached worktree `/home/tom/pistol-wt/review-impl` at `9c4366c`, its own
`CARGO_TARGET_DIR=/home/tom/pistol-wt/review-impl/target`, set per command; nothing in the live
tree was edited except this file. Every mutant was hand-applied there, run, and restored with
`git checkout --`; `git status --porcelain` was empty after each. Another project's
`cargo test` was on the box throughout (the ledger names it); every number here is a
functional receipt, none a timing one (D-592). The worktree is removed at the end.

**COUNTS**: 1 BLOCKING (two §6 mutant rows SURVIVE their named test — T3 cannot tell the X1
refusal from the usage refusal), 0 MAJOR, 4 minor. Every §1 row is implemented; every test row
T1–T8 passes and pins what it says; 19 of the 21 mutants I applied die at the named test; the
EQUIVALENT row is confirmed equivalent; all three D-589 obligations are discharged by execution.
The code produces no wrong answer that I could construct — the BLOCKING is a test-strength
defect under the prompt's own rule ("a mutant that SURVIVES is BLOCKING"), with an executed fix.

**READ, in order**: `CLAUDE.md`; `docs/process.md`; the design (rev 7); D-576, D-581,
D-586..D-594; `wp21_label_cache_design_rev6_REVIEW.md`; `label_cache.rs`, `capture.rs`,
`passes.rs`, `bin/arena.rs`, `usage.rs`, `bin/stub_engine.rs`, `channel.rs`, `seats.rs`,
`exchange.rs`; `tests/label_cache_tests.rs`, `tests/cold_label_check_tests.rs`,
`tests/common/mod.rs`, `tests/protocol_abuse_tests.rs:170-200`; `tools/cold_label_check.py`;
`docs/rule9_justifications.md`; `git diff 02c1601 9c4366c`.

---

## 0. BASELINE

`CARGO_TARGET_DIR=/home/tom/pistol-wt/review-impl/target cargo test -p pistol-arena --locked`
(whole crate, log `/home/tom/pistol-wt/review-impl-test0.log`): **36 `test result:` lines, 36
`ok`, `exit=0`**; among them `label_cache_tests` **`9 passed; 0 failed`**,
`cold_label_check_tests` **`11 passed; 0 failed`**, `capture_tests` 39, `census_capture_tests`
14, `wp21_tranche_config_tests` 18. The nine named rows, each `... ok` in that log:

```
test a_cached_capture_is_byte_identical_to_an_uncached_one_over_the_collision_fixture ... ok
test the_counts_line_reads_fewer_asks_than_records_cached_and_equal_uncached ... ok
test label_cache_with_census_in_either_order_is_refused_naming_both_and_leaves_no_file ... ok
test capture_run_refuses_a_census_sink_under_the_cache_before_spawning_any_engine ... ok
test label_cache_twice_or_anywhere_but_last_is_refused ... ok
test a_stray_line_in_the_pipe_while_the_cached_run_serves_hits_is_refused_in_both_runs ... ok
test the_collision_counters_are_zero_over_one_opening_and_positive_over_the_fixture ... ok
test a_cached_capture_is_byte_identical_over_a_report_whose_every_game_forfeits ... ok
test a_cached_capture_is_byte_identical_over_a_report_holding_a_rule_4_win ... ok
test a_class_with_nine_sampled_records_is_a_void_and_with_ten_it_passes ... ok
```

`cargo fmt --all --check`: exit 0. `cargo clippy -p pistol-arena --all-targets --locked --
-D clippy::all`: `Finished ... in 3.47s`, exit 0. `tools/file_justification_check.sh` (reads
the index = the commit): `362 tracked .rs/.sh files, 69 over the cap, all registered in
docs/rule9_justifications.md (69 entries)`, exit 0.

---

## 1. THE DESIGN's §1 ROWS, AGAINST THE CODE AT `9c4366c`

| # | row says | code | verdict |
|---|---|---|---|
| 1 | `LabelCache { Off, On }`, `CaptureCounts { asks, records, hits, key_pos_collisions, key_full_collisions, fold_ms }`, crate-private `Memo` with the `BTreeMap<String,(String,String)>`, two coarser sets, counts | `label_cache.rs:13-19`, `:27-44`, `:76-83` (`pub(crate) struct Memo`, `answers: BTreeMap<String, (String, String)>`, `key_pos`/`key_full: BTreeSet<Vec<(Coord, Player)>>`) | ✓ |
| 2 | `run` takes a MODE, builds the `Memo` inside the `with_seats` closure, returns `(Vec<CaptureRecord>, CaptureCounts)` | `capture.rs:341-346` signature; `:355-356` `with_seats(..., \|channels\| { let mut memo = Memo::new(cache);` — the map's type crosses no signature | ✓ |
| 2a | X1b as `run`'s first statement | `capture.rs:347` `refuse_census_under_cache(census.request, cache)?;` precedes `one_engine` (`:348`) and `verify_engines` (`:349`) | ✓ |
| 2b | X3 at every prefix, before the lookup, moved out of `ask` | `capture.rs:363-368` inside `for k in asked_prefixes(game)?`, before `position_line` (`:369`); `git diff 02c1601 9c4366c` removes the block from `ask` (old `:241-246`); `git grep 'unsolicited()'` at HEAD → `capture.rs:363` and `exchange.rs:34` only | ✓ |
| 2c | the LOOKUP: hit takes the pair, makes no ask; miss increments `asks` then asks | `capture.rs:370-387`: `match memo.lookup(&position) { Some(pair) => pair, None => { memo.asked(); let (totals, bestmove) = ask(...)?; ... } }`; `Memo::lookup` (`label_cache.rs:100-105`) returns `None` under `Off` | ✓ |
| 2d | the INSERT after `normalise`; the two coarser keys into their sets, a counter per key already present | `capture.rs:383-384` `let pair = (normalise(&totals)?, bestmove); memo.insert(&position, &game.moves[..k], pair.clone());`; `label_cache.rs:123-138` (`if !self.key_pos.insert(..) { += 1 }`, same for `key_full` over `canonical_form(&stones)`) | ✓ |
| 3 | `passes.rs` threads the mode, prints the counts line | `passes.rs:47` parameter, `:61` `run(&transcript, label_nodes, &mut sink, cache)?`, `:89` `println!("{}", counts.line(cache))` | ✓ |
| 4 | five reachable spellings; `--label-cache` legal; both orders ONE or-pattern arm (X1); everything else the catch-all | the arm is not in `bin/arena.rs`'s `match words` (row 4's site) but in `usage.rs:111-122` `capture_tail`, called from `arena.rs:61` before `outpath::claim` (`:80`); `:116` is the one or-pattern `["--census", "--label-cache"] \| ["--label-cache", "--census"]`; `:120` `_ => Err(usage_error())`. The ledger records the move (rule 9: `arena.rs` came out at 308). **The row's reason — the catch-all "names neither word" — is FALSE at HEAD**: `usage_error()` (`:124-129`) appends `USAGE`, which names `--census` 3× and `--label-cache` 2× (`git show HEAD:…usage.rs \| sed -n 10,104p \| grep -c`). Mechanism ✓; the test that leans on that reason is the BLOCKING below | ✓ mechanism / see B1 |
| 5 | `usage.rs`: the word, absence = off, what the counts line means | `usage.rs:16` synopsis `[--census \| --label-cache]`; `:67-81` the paragraph: "absent means OFF", both counts-line shapes spelled, `asks`/`hits` defined, refused with `--census` by name | ✓ |
| 6 | stub `Behave::StrayAfterNewGame(n)` spelled `stray_after_newgame <n>`; second `bestmove` in the SAME write syscall; counts `newgame`s; a `bestmove`-shaped stray | `stub_engine.rs:142` variant; `:166-175` parse (spelling validated, `n == 0` refused); `:183` in `ALL`; `:441-448` count + arm at `new_games == n`; `:580-600` one `String` buffer, `out.write_all(buffer.as_bytes())` on the `StdoutLock` (`:425`), `stray_armed = false` after. Syscall shape verified by execution in §3(b) | ✓ |
| 7 | `cold_label_check.py`: fewer than ten SAMPLED in a class is VOID (exit 2), a named constant printed in the void | `:85` `MIN_SAMPLED = 10`; `:283-288` `raise Void(f"{len(sampled)} sampled {want.upper()} record(s) ... below the registered floor of {MIN_SAMPLED} ...")`; `:327-333` `Void` → exit `VOID = 2` (`:80`) | ✓ |
| 8 | `label_cache_tests.rs` new with §5's rows; T8 beside the checker's cases | `label_cache_tests.rs` 541 lines, 9 `#[test]` (T1, T2, T3, T3b, T7, T4, T5, T6a, T6b); `cold_label_check_tests.rs:413-459` T8 | ✓ |
| 9 | rule-9 entries; `PROPOSES` entries go | `rule9_justifications.md:82` `capture.rs` entry rewritten naming the lookup/insert; `:89` new `label_cache_tests.rs` entry (two-stage fixture, builder checks geometry with `pistol-core`); `governing_citation_check.sh` `PROPOSES=( )` now empty (diff removes both lines) | ✓ |

**NOTHING ELSE**: `capture_file.rs:53` `render(transcript, label_go, records)` takes no counts;
`git grep -n 'fold_ms\|CaptureCounts\|into_counts\|counts\.line'` over `crates/pistol-arena/src`
hits only `label_cache.rs`, `capture.rs:9,346,400`, `passes.rs:89`, `usage.rs:76` — the counts
reach stdout and nothing else. `CAPTURE_FORMAT_VERSION` untouched (`git diff` shows no
`capture_file.rs` change).

---

## 2. THE EIGHT TEST ROWS, RUN

All in the worktree, `cargo test -p pistol-arena --locked --test <suite> <name> -- --exact`
(the baseline whole-crate run above is the log cited; each row was also run alone during the
mutation sweep's restore cycles and in §5's probes).

| row | test | passes | pins what the design says? |
|---|---|---|---|
| T1 | `a_cached_capture_is_byte_identical_to_an_uncached_one_over_the_collision_fixture` | ok | `identical()` (`:176-190`) asserts both files exist and `std::fs::read` equal, over `collision_fixture()` whose three geometric properties are asserted with `pistol-core` (`:58-95`) before the book is written. ✓ |
| T2 | `the_counts_line_reads_fewer_asks_than_records_cached_and_equal_uncached` | ok | `records` derived independently from `asked_prefixes` over the transcript (`:222-233`); uncached `asks == records` (`:239`); cached `asks < records` (`:248`), `hits == records - asks` (`:252`), `hits >= records/2` (`:258`). ✓ — and it is the row that killed M01, M05, M07, M08, M09 |
| T3 | `label_cache_with_census_in_either_order_is_refused_naming_both_and_leaves_no_file` | ok | exit 2 both orders, no `--out` and no `.census.txt` file, stderr contains both words (`:280-294`). **The word check is VACUOUS**: the usage refusal's stderr also carries both words (B1). Exit-2 and no-file are pinned; "named" is not |
| T3b | `capture_run_refuses_a_census_sink_under_the_cache_before_spawning_any_engine` | ok | over a report whose binary path is edited to a missing file, `run(..., On)` errs naming both words; the `Off` control errs WITHOUT naming `--label-cache` (`:318-337`) — proves the refusal precedes the spawn. ✓ (M13 dies here with `Io: reading .../no-such-engine`, the control's message) |
| T4 | `a_stray_line_in_the_pipe_while_the_cached_run_serves_hits_is_refused_in_both_runs` | ok | `n = P0 + 1` read off `asked_prefixes` of an honest play (`:391-394`); play with the stray stub has no forfeit and identical games (`:407-414`); uncached: no file, stderr `game 1, turn 0` and (`spoke before it was asked` or `no totals line`) (`:426-435`); cached: no file, stderr `game 1,` and `spoke before it was asked` (`:437-447`). ✓ matches §5 T4 word for word |
| T5 | `the_collision_counters_are_zero_over_one_opening_and_positive_over_the_fixture` | ok | one opening → `(0, 0)` (`:465`); fixture → `key_pos >= 1`, `key_full >= 2`, `key_full >= key_pos` (`:482-493`). ✓ |
| T6 | `..._over_a_report_whose_every_game_forfeits` (stub `demands_newgame_per_ask`, `:507`; asserts every game forfeits, `:512`) and `..._over_a_report_holding_a_rule_4_win` (asserts a 7-turn game, `moves[6] == Turn::single((-5,0))`, `P1Win`, `:525-530`) | ok, ok | ✓ both; this is D-589 obligation (a), see §3 |
| T7 | `label_cache_twice_or_anywhere_but_last_is_refused` | ok | exit 2 and no file for `--label-cache --label-cache` and for the word before `--out` (`:351-374`). ✓ |
| T8 | `a_class_with_nine_sampled_records_is_a_void_and_with_ten_it_passes` (`cold_label_check_tests.rs:414`) | ok | cuts the capture to game 0 + 9 hits → exit 2, stderr `RUN VOID`, `9 sampled HIT`, `10` (`:433-444`); + 10 hits → exit 0, `10 of 10 sampled HIT` (`:448-458`). Guarded by `rows.len() >= misses + 10 && rows[misses][0] == "1"` (`:426`) so a fixture too small fails loudly rather than vacuously. ✓ |

**Row 11 of the prompt — the floor against the existing suite**: all 11 cases in
`cold_label_check_tests.rs` pass at HEAD (`11 passed`), so no existing fixture fell under the
floor; the two-opening honest fixture yields ≥ 10 sampled records at stride 1 and at stride 3
(`the_sample_is_every_stride_th_record...` passes with `total.div_ceil(3)`).

---

## 3. THE THREE IMPL OBLIGATIONS OF D-589

**(a) T6's forfeit report uses `demands_newgame_per_ask` and the identity holds over it.**
`label_cache_tests.rs:507` names the word; `:512-515` asserts `games` non-empty and every
`game.forfeit`; `identical()` then compares the two captures. Run: `... ok` (baseline log). The
mechanism the row leans on re-derived by me: `seats.rs:47` sends one `newgame` per spawn;
`stub_engine.rs:491-516` sets the latch on `newgame`, clears it on `go` under
`DemandsNewGamePerAsk` (`:495-500`), and answers a `position` without it with an `error` line
(`:505-514`); `exchange.rs:70-75` forfeits `ProtocolError` on that line; `capture.rs:242` sends
`newgame` before every ask so every position is answered. ✓ DISCHARGED.

**(b) The stub's stray shares the answer's SINGLE write syscall.** From the code:
`stub_engine.rs:591-598` builds one `String` (every answer, then the stray, each `\n`-terminated)
and calls `out.write_all` once on `stdout.lock()` (`:425`); a `LineWriter` given one buffer
ending in `\n` with nothing pending hands everything up to the last newline to ONE inner
`write`. By execution — no `strace` on this box, so an `LD_PRELOAD` shim that logs every
`write(1, …)` to stderr (`scratchpad/shim/wshim.c`, built with `gcc -shared -fPIC`), the stub
fed exactly the prompt's script:

```
$ printf 'newgame\nnewgame\nposition start\ngo nodes 5\nquit\n' | LD_PRELOAD=wshim.so \
    target/debug/arena-stub-engine --config stray2.toml      # behave stray_after_newgame 2
[wshim] write(fd=1, len=193, newlines=4)
info depth_turns 1 seldepth 1 nodes 1 nps 1 time 0 hashfull 0 score cp 0 pv 0,0
info totals depth_turns 1 seldepth 1 nodes 1 nps 1 time 0 hashfull 0 score cp 0 pv 0,0
bestmove 0,0
bestmove 0,0
```

ONE `write` of 193 bytes carrying four lines, the stray the fourth. Controls: `behave honest`
on the same feed → three writes (`len=80`, `len=87`, `len=13`, one line each); the stray stub
fed ONE `newgame` → the same three writes and no stray (the count is honoured; `n = 2` does not
fire on one). ✓ DISCHARGED. **The residual §3 records** is present in the design verbatim
("the cached arm has no check after game 1's last hit; T4's cached assertion holds unless the
reader thread is preempted between its two sends for the whole of game 1 … the mutant arm has
no such residual") and is true of the code: `channel.rs:86-110` is one `read_until` and one
`send` per line, so the answer's `bestmove` and the stray are two `send`s from one 193-byte
read; a hit (`capture.rs:370-371`) touches the channel only through `unsolicited()`'s
`try_recv` (`:363`, `channel.rs:198-205`); `with_seats` → `shutdown` (`channel.rs:227-232`)
receives nothing. So T4's cached-arm assertion on correct code is NOT deterministic (as the
design says); §5 below probes it. **The mutant arm IS deterministic**: under M14a/M14b game 1
performs no channel operation at all, so the run exits 0 with a file — killed 3 of 3 runs, the
same assertion (`:439`) each time (§5).

**(c) Row 6's reason about `doubled.sh` is true of `tests/protocol_abuse_tests.rs`.**
`:180-199`: `first=1`, and the `go*)` arm prints the doubled `bestmove` only `if [ "$first" = 1 ]`
then sets `first=0` — it doubles at its FIRST `go`, game 0 turn 0, a miss in any run; a later
`go` prints one `bestmove`. The row's sentence ("doubles at its FIRST `go` — game 0, turn 0, a
miss in any run — so X3's test needs a behaviour that deviates after a counted `newgame`") is
exact. ✓ DISCHARGED.

---

## 4. THE MUTATION SET — 21 MUTANTS OVER THE 19 ROWS, EACH APPLIED, RUN, RESTORED

Driver: `scratchpad/mut/mutants.py` (each edit asserted to match exactly once; `git checkout --`
after each; `git status --porcelain` asserted empty). Per-mutant diffs and full logs under
`scratchpad/mut/out/`. Command per row: `CARGO_TARGET_DIR=… cargo test -p pistol-arena --locked
--test <suite> <test> -- --exact`. Exit 101 = the test panicked; the "killed by" column is the
assertion's own line and message from the log.

**My call-site receipt** (`git grep -n -E 'memo\.(lookup|insert|asked|recorded|into_counts)|Memo::new|refuse_census_under_cache\(|capture_tail\(|stray_armed|MIN_SAMPLED' 9c4366c -- crates/pistol-arena/src tools/cold_label_check.py`, sorted `LC_ALL=C`):
`arena.rs:61` `capture_tail`; `stub_engine.rs:438,447,583,599` `stray_armed`;
`capture.rs:322,347` `refuse_census_under_cache`; `capture.rs:356` `Memo::new`; `:370` `lookup`;
`:373` `asked`; `:384` `insert`; `:396` `recorded`; `:400` `into_counts`; `usage.rs:111`
`capture_tail`; `cold_label_check.py:85,283,286` `MIN_SAMPLED`. One site each; no second
lookup or insert exists.

| § 6 row | mutant as applied (file) | test | result | killed by |
|---|---|---|---|---|
| 2c lookup removed | M01: `memo.lookup(&position).and(None)` (capture.rs) | T2 | **DIES** exit 101 | `:248` "cached, the engine was asked once per record… {asks: 54, hits: 0, records: 54}" |
| 2c lookup inverted | M02: `Memo::lookup` returns `answers.values().next_back()` on a miss (label_cache.rs) | T1 | **DIES** | `:185` "the cached capture is not byte-identical" |
| 2c keyed on the stone list | M03: key = `format!("{sorted stones:?}")` for lookup and insert (capture.rs; `stones_of` made `pub(crate)`) | T5; T1 | **DIES** at T5; T1 passes (the honest stub answers a transposition alike — not a row the design claims) | `:482` "Y's transposed prefix is a stone-set collision: {key_pos_collisions: 0, …}" |
| 2c keyed on the canonical form | M04: key = `format!("{canonical_form(&sorted):?}")` | T5; T1 | **DIES** at both | T5 `:482` (`key_full 0, key_pos 0`); T1 `:185` — a `bestmove` in the wrong frame |
| 2d insert removed | M05: `self.answers.insert(..)` → `let _ = (position, pair)` (label_cache.rs; counters kept) | T2 | **DIES** | `:248` asks 54 = records 54, hits 0 |
| 2d stores the raw pair | M06: `memo.insert(&position, .., (totals, pair.1.clone()))` — un-normalised totals (capture.rs) | T1 | **DIES** | `:185` byte-identity (` nps` in a hit's record) |
| 2 mode consulted nowhere | M07: `Memo::new` stores `LabelCache::Off` whatever it is given | T2 | **DIES** | `:248` asks 54, hits 0, counters 0 |
| `asks` reports `records` | M08: `into_counts` sets `asks: self.counts.records` | T2 cached arm | **DIES** | `:248` asks 54 = records 54 (hits 31 — the derived field still read) |
| `asks` derived unconditionally | M09: `asks: self.answers.len()` | T2 **uncached** arm | **DIES** | `:239` "uncached, every record is an ask: {asks: 0, records: 54}" |
| `asks` derived only when `On` | M10: `asks: match mode { On => answers.len(), Off => counts.asks }` | whole `label_cache_tests` | **SURVIVES — EQUIVALENT as the design says** | `9 passed; 0 failed` (45 s); no row distinguishes it while the cache is live |
| X1 the arm removed | M11: the or-pattern arm deleted from `capture_tail` (usage.rs) — both orders fall to `usage_error()` | T3 | **SURVIVES** `1 passed` | — (B1) |
| X1 one alternative removed | M12: `["--census", "--label-cache"] =>` only | T3 | **SURVIVES** `1 passed` | — (B1) |
| X1b removed | M13: `refuse_census_under_cache(..)?` → `let _ = (..)` (capture.rs) | T3b | **DIES** | `:321` "the refusal must name both words: Io: reading …/no-such-engine" |
| X3 removed | M14a: the guard block deleted from `run` | T4 | **DIES** | `:439` "the cached run served game 1 from the memo and never looked at the pipe" (file written, exit 0) |
| X3 left inside `ask`, not hoisted | M14b: block deleted from `run`, the pre-IMPL block restored in `ask` | T4 | **DIES** | `:439` same |
| `key_pos_collisions` removed | M15: `self.key_pos.insert(..)` with no count | T5 | **DIES** | `:482` `key_pos_collisions: 0` |
| `key_full_collisions` removed | M16: same for `key_full` | T5 | **DIES** | `:486` "Z's image prefixes and Y's transposed one are canonical collisions: {key_full_collisions: 0, key_pos_collisions: 1}" |
| counters inverted | M17: `if self.key_pos.insert(..)` / `if self.key_full.insert(..)` | T5 one-opening arm | **DIES** | `:465` "over one opening every miss has a distinct stone count: {key_full: 9, key_pos: 9, records: 18}" |
| row 6 stray never written | M18: `buffer.push_str(&stray); buffer.push('\n')` → `let _ = &stray` (stub_engine.rs) | T4 | **DIES** | `:426` "the uncached run wrote a capture with a stray in the pipe" |
| row 7 floor set to zero | M19a: `MIN_SAMPLED = 0` (cold_label_check.py) | T8 | **DIES** | `cold_label_check_tests.rs:433` "nine sampled hits … must be a VOID (exit 2), not a pass: exit Some(0)" |
| row 7 floor removed | M19b: the `if len(sampled) < MIN_SAMPLED: raise Void(..)` block deleted | T8 | **DIES** | `:433` same |

**Tally**: 19 die at the named row, 1 equivalent as registered, **2 survive** (M11, M12).

---

## 5. PROBES: T4's TWO ARMS, THE FIXTURE AT HEAD, X1 BY EXECUTION, GATE 20

All in the worktree at `9c4366c`, binaries rebuilt at HEAD first (`cargo build -p pistol-arena
--locked --bins`, tree clean by `git status --porcelain`). Script `scratchpad/post.sh`, log
`scratchpad/post.log`; the X1 execution re-run without `set -e` (the first pass stopped at the
refusal's exit 2).

**1. The §5 fixture at HEAD, by hand** (`scratchpad/fx/run_fixture.sh`: the three openings X, Y, Z
written with a `body_sha256` header, the honest stub on both seats, `turn_cap 8`, `nodes 5000`,
`take 3`, `workers 1`; then `arena --capture … --label-nodes 5000` without and with
`--label-cache`):

```
arena: captured 54 position(s) from 6 game(s) at go nodes 5000
arena: label cache off: asks 54 records 54
arena: captured 54 position(s) from 6 game(s) at go nodes 5000
arena: label cache on: asks 23 records 54 hits 31 key_pos_collisions 1 key_full_collisions 4 fold_ms 1
cmp cap-off.txt cap-on.txt → IDENTICAL   (sha256 698ad4472b9ac960… both)
```

`hits 31 = 54 − 23`; `key_full 4 ≥ key_pos 1 ≥ 1`; `key_full ≥ 2` — the T5 fixture's numbers on
correct code (compare M15/M16/M17's readings in §4). Y's transposed four-turn prefix is the one
stone-set collision; Z's images plus Y's account for the four canonical ones.

**2. X1 by execution, on that report, both orders** (stderr verbatim, one line each; exit 2;
`ls cap-x1*` empty; `grep -c 'usage:'` 0):

```
[--census --label-cache] arena: --label-cache with --census is refused: a cache hit performs no search and emits no census row, so a cached census capture would under-report firings at exit 0
[--label-cache --census] arena: --label-cache with --census is refused: a cache hit performs no search and emits no census row, so a cached census capture would under-report firings at exit 0
```

and the catch-all beside it: `… --bogus` → exit 2, **96 stderr lines**, `usage:` present,
`--label-cache` named 2×, `--census` 3× — the population T3's assertion cannot separate (B1).

**3. The executed T3 fix.** In the worktree, T3's word check replaced by
`stderr.contains("--label-cache with --census is refused") && !stderr.contains("usage:")`
(`git diff --stat`: 1 file, 2 insertions, 2 deletions). Correct code:
`test label_cache_with_census_in_either_order_is_refused_naming_both_and_leaves_no_file ... ok`
(`1 passed; 0 failed`, 6.35 s). Then M11 and M12 re-applied under the patched test
(`scratchpad/mut/out_fix/`):

```
M11_X1_arm_removed           -> exit 101, FAILED (0 passed, 1 failed) | :291 census-first: the refusal must be the named one, not the usage text: arena: --config and --out are both required, …
M12_X1_one_alternative_removed -> exit 101, FAILED (0 passed, 1 failed) | :291 cache-first: the refusal must be the named one, not the usage text: arena: --config and --out are both required, …
```

Both die; test file restored (`git status --porcelain` → 0 entries).

**4. T4's cached arm on correct code, 5 runs** (`--test label_cache_tests a_stray_line… -- --exact`):
`ok. 1 passed` ×5 (18.95 s, 19.08, 19.08, 19.09, 19.09). No flake observed in five, beside
another project's `cargo test`; the residual §3 states remains a residual, not a measurement of
zero. **The mutant arm, M14a, 3 runs**: `FAILED (0 passed, 1 failed)` ×3, each at `:439`
"the cached run served game 1 from the memo and never looked at the pipe" — deterministic as
the design and D-593 say.

**5. Gate 20** (`tools/governing_citation_check.sh` in the worktree): `DESIGN_CITATION_CHECK_DONE`,
exit 0.

---

## 6. §3 REFUSALS — REACHABLE, NAMED, CALLED (D-553)

| refusal | site | reachable from | named | test exercising the CALL |
|---|---|---|---|---|
| X1 | `usage.rs:116-119` | `arena --capture … --label-nodes n --census --label-cache` (either order), `arena.rs:61`, before `outpath::claim` (`:80`) | "--label-cache with --census is refused: a cache hit performs no search and emits no census row, …" | T3 drives the binary — but see B1: the assertion cannot tell this sentence from the usage text |
| X1b | `capture.rs:322-329`, called `:347` | `capture::run(.., sink{request: On}, LabelCache::On)` from any crate | "`--label-cache` with `--census`: a cache hit performs no search …" (see m1 for the 14-space run inside it) | T3b calls `run` directly; M13 dies |
| X3 | `capture.rs:363-368` | any stray in the pipe at any prefix, hit or miss | "game G, turn K: the engine spoke before it was asked (…)" | T4 both arms; M14a/M14b die |

---

## 7. BYTE-IDENTITY, `asks`, DETERMINISM, RULE 9, COVERAGE

**Byte-identity (prompt 6).** A record is built at ONE place (`capture.rs:388-394`) from
`(totals, bestmove)`; on a miss that pair is `(normalise(&totals)?, bestmove)` (`:383`) and the
same `pair.clone()` goes into the memo (`:384`); on a hit it is the stored clone (`:371`).
`no_tab` (`:395`) runs on every record either way. Under `Off`, `lookup` is `None` by
construction (`label_cache.rs:102`), so the uncached pass is the pre-IMPL pass with `normalise`
moved one statement earlier. I found no path by which a hit's bytes could differ from the miss
that filled it, and no path by which `Off` differs from `02c1601`. T1/T6 confirm over 54, 2-game
forfeit, and rule-4-win records; the §5 fixture run shows `cmp` identical.

**`asks` (prompt 7).** `memo.asked()` at `capture.rs:373` is the statement before `ask(`
(`:374`); `Memo::asked` is `self.counts.asks += 1` (`label_cache.rs:108-110`); nothing reads the
records or the memo to produce it (M09 shows what would happen). `hits` is
`records.saturating_sub(asks)` (`:143`); records ≥ asks always (`recorded()` runs once per
record, `asked()` only on the miss path), so it is `records - asks`. T2 asserts the identity.

**Determinism (prompt 8).** The cache path holds a `BTreeMap`/two `BTreeSet`s that are never
iterated (only `get`/`insert`), an `Instant` whose only sink is `fold: Duration` →
`fold_ms` → `CaptureCounts::line` → `println!` (`passes.rs:89`); `render` (`capture_file.rs:53`)
and `manifest_row` take `records`/`rendered` only. No hash iteration, no time-based choice, no
thread beyond the reader threads that exist today. The capture chooses no move; a hit
substitutes stored bytes for an identical question. Nothing here can influence move choice or
captured bytes.

**Rule 9 (prompt 9).** `git show 9c4366c:… | wc -l`: `bin/arena.rs` 249 (under the cap, no
entry needed), `usage.rs` 167, `label_cache.rs` 169, `capture.rs` 426, `stub_engine.rs` 635,
`label_cache_tests.rs` 541. `rule9_justifications.md:82` (`capture.rs`) now names the lookup and
insert as "the decision not to ask and the record of what was asked" and points the memo's type
and folds at `label_cache.rs` — describes the file as it is. `:22` (`stub_engine.rs`: "every
deviation this instrument makes lives in one file") is unchanged and still true of the new
variant. `:89` (`label_cache_tests.rs`) matches the file's shape. Gate `file_justification_check`
exit 0, quoted in §0.

**Coverage rule (prompt 12).** `cold_label_check_tests.rs:75-76` runs `python3
repo()/tools/cold_label_check.py` — the shipped script, never a copy — with a control run
(`a_capture_a_fresh_process_reproduces_is_reported_as_agreeing`, exit 0) beside the refusals;
T8 drives the floor through it in both directions. ✓.

---

## 8. RE-DERIVATION (my commands, their scope)

| claim | my command (scope) | mine | document's |
|---|---|---|---|
| suite counts | `grep -cE '^test result: ok' review-impl-test0.log`; `grep -c '#\[test\]'` on the three files | 36/36 ok; `label_cache_tests` 9, `cold_label_check_tests` **11**, `wp21_tranche_config_tests` 18 | ledger §2.1 says `cold_label_check_tests` **12 passed** — m3 |
| gate 9 / total | `git show 9c4366c:tools/ci.sh \| grep -n 'GATE_TOTAL=\|gate 9/\|"determinism"'` | `:21` `GATE_TOTAL=20`; `:109-110` | §1 ✓ |
| 742 / 347 | `/usr/bin/grep -n -E '742\|347' artifacts/arc3_leverB_41_count_v3.txt` (live tree, read-only; not in git) | `:19` 742 asked, `:20-22` 347 under all three keys | §1 ✓ |
| ten-record floor is registered | `git show 9c4366c:docs/experiments/wp21_prereg.md \| grep -n '10 sampled records'` | `:345` | row 7 ✓ |
| `newgame` senders | `git grep -n NEW_GAME 9c4366c -- crates/pistol-arena/src` | `seats.rs:47` (spawn), `capture.rs:242` (per ask); stub reads only | row 6 ✓ (design's `:247` is now `:242`) |
| `unsolicited` callers | `git grep -n 'unsolicited()' 9c4366c -- crates/pistol-arena` | `capture.rs:363`, `exchange.rs:34` — none left in `ask` | 2b ✓ |
| `demands_newgame_per_ask` captured already | `git grep -n demands_newgame_per_ask 9c4366c -- crates/pistol-arena/tests` | `capture_tests.rs:252`; `label_cache_tests.rs:502,507` | T6 ✓ |
| usage text names the words | `git show 9c4366c:…usage.rs \| sed -n 10,104p \| grep -c -- '--census'` / `'--label-cache'`; same at `adb2012` | HEAD 3 / 2; `adb2012` 2 / 0 | row 4 "names neither word" ✗ — false for `--census` even at `adb2012`, false for both once row 5 added the word |
| call sites, one each | the receipt in §4 | 1 lookup, 1 insert, 1 asked, 1 recorded, 1 X1b call, 1 X1 arm, 1 floor | §6 ✓ |
| `classify` ignores `info` | `git show 9c4366c:…capture.rs \| grep -n 'Step::Ignore;'` | `:199` | row 6 ✓ (`:197-199` → `:198-199`) |
| line counts | `git show 9c4366c:<f> \| wc -l` | above | ledger's 249/167 ✓ |

---

## 9. FINDINGS

### B1 (BLOCKING by the prompt's rule; the code is correct) — §6's two X1 mutants SURVIVE T3: the "names both words" assertion is satisfied by the usage refusal

**Reproducer**: apply M11 (delete `usage.rs:116-119`) or M12 (drop the second or-alternative);
run T3 → `test result: ok. 1 passed` (§4, logs `scratchpad/mut/out/M11_*`, `M12_*`). Why: under
either mutant the combination reaches `_ => Err(usage_error())` (`usage.rs:120`), `main` prints
`arena: {why}` to stderr and exits 2 (`arena.rs:26-28`), no file is claimed, and `usage_error()`
(`:124-129`) appends `USAGE`, whose text contains `--census` and `--label-cache`. T3's three
assertions (exit 2, no file, both words in stderr) are all true. By execution on the shipped
binary: `arena --capture <report> --out <p> --label-nodes 5000 --bogus` → exit 2, stderr
names `--label-cache` 2× and `--census` 3× (§5). §6's row says the catch-all "names neither
word"; it never did for `--census` (`adb2012` USAGE: 2 hits) and row 5 made it name both.

**What it means**: X1 itself is implemented and reachable (`usage.rs:116`, executed in §5 with
the named sentence and no usage text), so no wrong answer is produced — but T3 pins only "exit 2
and no file", which the usage arm also delivers, and D-553's "named" is unpinned. Under the
prompt's severity rule a surviving mutant is BLOCKING.

**Fix, EXECUTED in the worktree** (§5, item 2): replace T3's word check with the refusal's own
sentence and the absence of the usage text —
`stderr.contains("--label-cache with --census is refused") && !stderr.contains("usage:")`.
Correct code: `1 passed`. M11 under the patched T3: **dies**; M12: **dies** (the `cache-first`
order falls to usage). Test file restored afterwards. The design's §6 row should read "the
catch-all refuses with the USAGE text, which T3 distinguishes by the refusal's own sentence".

### m1 — X1b's refusal string carries a 14-space run

`capture.rs:325`: `"… emits no census              row, so …"` — a line continuation lost its
`\`. The refusal still names both words (T3b passes); the message as printed has the gap.
Reproducer: `sed -n 325p capture.rs | grep -c '              '` → 1. Fix: the `\` continuation
(unexecuted claim — trivial, but not run).

### m2 — the mutation receipt document §7 owes at IMPL does not exist

§6: "the receipt is owed at IMPL"; §7: "the mutation receipt with its `git grep` enumeration —
at IMPL, its own document". `ls docs/experiments | grep -i mutation` → nothing; the ledger §2.1
does not mention one. This review's §4 is an executed set with its own receipt, so the mutants
have now been run; the IMPL-side document is still owed by the design's own table.

### m3 — ledger §2.1's test claim does not match the log

"`cold_label_check_tests` **12 passed** (T8 among them)". The file has 11 `#[test]`
(`grep -c`), and both the implementer's crate-wide run and mine print `11 passed`. CLAUDE.md
Closure: a test claim cites the log. The other counts in that sentence (9, 18, 39, 14) match.

### m4 — design row 4's site and reason are not the code's

Row 4 places the five spellings in `bin/arena.rs`'s `match words` with the or-pattern as an arm
there; the code has one `tail @ ..` arm (`arena.rs:52-71`) and the five spellings in
`usage.rs::capture_tail`. The ledger records the move as an "architect default" (rule 9).
Mechanism identical; the row's *reason* for the catch-all is the false claim behind B1. An ADR
line or a row-4 amendment should say where the arm lives and that the catch-all names the words.

---

## 10. WHAT SURVIVED ATTACK

- **The key**: exact `position` bytes; M03/M04 show a coarser key is caught by T5 (and T1 for
  the symmetry fold). M02 shows a wrong hit is caught by T1 at the second record.
- **The hoist**: M14a and M14b both exit 0 with a file under the cached run, 3 of 3 times for
  M14a; T4's cached arm on correct code refused within game 1 in 5 of 5 runs (§5).
- **The counter at the call**: M08/M09 die at the two different arms the design names.
- **The single syscall**: one 193-byte `write` holding four lines, by shim.
- **Byte-identity** over the collision fixture, a forfeit-every-game report, and a rule-4 win;
  the §5 fixture run `cmp`s identical with the counts line reading hits.
- **The floor**: both removal shapes die at the nine-record case.
- **The seam refusal**: precedes the spawn (M13's failure message is the missing-binary error).
- **rustfmt, clippy `-D clippy::all`, gates file_justification and governing_citation**: clean.

## 11. ATTACKS ATTEMPTED AND REJECTED

- **A hit can reach `normalise` twice or skip `no_tab`**: no — one record site, `no_tab` after
  it; rejected by reading `capture.rs:388-396`.
- **`records < asks` makes `hits` wrap**: `saturating_sub`, and `asked()` is only on the miss
  path where `recorded()` follows unless the run refuses (then no counts line). Rejected.
- **`Off` folds and pays `fold_ms`**: `insert` returns before the fold under `Off`
  (`label_cache.rs:124-126`); the `Off` line prints no such field. Rejected.
- **`canonical_form` of a sorted list is not set-invariant**: the input is sorted first
  (`:130`), so equal sets → equal input → equal output; `key_full >= key_pos` holds by
  construction and T5 asserts it. Rejected.
- **The stray fires in play**: one `newgame` per spawn, `n >= 2` asserted by T4 (`:395`); by
  shim, one `newgame` → no stray. Rejected.
- **`stray_after_newgame` accepts `02` or `0`**: `:169-172` refuse both. Rejected.
- **T3b's control is vacuous**: it asserts the `Off` error does NOT name `--label-cache` and
  M13's kill message is exactly that error. Rejected.
- **T8 passes vacuously when game 1 is short**: guarded at `:426`; a short game 1 would fail
  the `9 sampled HIT` substring, not pass. Rejected.
- **The counts line could land in the capture file**: `render` signature takes no counts;
  `grep` receipt in §1. Rejected.
- **`tools/wp21_tranche_config.py --pilot-range`** (in the diff, not in this design): out of
  this review's scope; its 18 tests pass; not audited here.

---

## 12. HEAD MATCH, AND THE WORKTREE

`git -C /home/tom/Projects/HeXO-AlphaBeta rev-parse HEAD` at the end of this review:
**`037b196ff69f56607b98acb52ea70a3b70e6ebb4` — the live tree NO LONGER matches `9c4366c`.**
Three commits landed on `dev` while this review ran (`eb3dc6c`, `610225b`, `037b196`; read-only
`git log`/`git show`, nothing of mine): `610225b` *"T3 reads the refusal's own line — the usage
text the catch-all appends names every word, and both X1 mutants survived a whole-of-stderr
search; design revision 8 says so"* changes T3 to assert on `stderr.lines().next()`, and
`037b196` adds `docs/experiments/wp21_label_cache_MUTATION.md`. Those are the implementer's own
discovery and remedy of B1 and m2, taken AFTER the dispatched revision; they were not reviewed
here and do not change the verdict on `9c4366c`. The first-line assertion is a different fix
from the one I executed (mine checks the refusal's sentence and the absence of `usage:`); both
shapes kill M11/M12 by the same mechanism — the catch-all's own line does not name the words —
but I have run only mine. The remaining `diff --stat` is the two registrations landing from
their drafts, outside this package.

The worktree `/home/tom/pistol-wt/review-impl` is removed after this file is written
(`git worktree remove --force`); nothing under `artifacts/` or `sessions/` was written there.
Scratch evidence (mutant diffs and logs, the shim, the fixture) lives under this session's
scratchpad and will not survive it — the receipts quoted above are the record.

---

**VERDICT: FAIL — 1 BLOCKING, 0 MAJOR, 4 minor** at `9c4366c`. The BLOCKING is §6's two X1
mutants surviving T3 (an assertion the usage text satisfies), fixed by an executed one-line
assertion change; the mechanism, every refusal, every §1 row, all three D-589 obligations and
the other 19 mutant rows hold by execution. Reviewed at `9c4366c`; HEAD is now `037b196`.
