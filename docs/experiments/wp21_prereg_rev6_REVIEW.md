# REVIEW — `docs/experiments/wp21_prereg.md` revision 6. FRESH CONTEXT. ROUND 4 of 5 (D-585).

**NAMED REVISION.** Commit **`735fc370c7018ce1e77ddd8a1befc960e3397891`** on `dev`.

**DOES IT STILL MATCH HEAD?** **NO.** `git rev-parse HEAD` in the live tree was
`735fc37` when this review began and is **`73979e7675696022b81e429f87b5437f8ac47db3`**
at its end (`docs(arc3): the assembler's scoped round 3 passes over its own diff …`,
09:18:49 +02:00). `git diff --stat 735fc37 HEAD` is two files, both `docs/`
(`arc3_ledger.md` +22, `wp21_assemble_REVIEW_round3.md` +228); the document under
review, every instrument and every binary are byte-identical at both. Every finding
below is against `735fc37`; where HEAD closes one, it says so.

**WHAT I READ.** `CLAUDE.md` (Process; hard rules 1, 3, 6, 8, 10); `docs/process.md`
whole; the document whole; `wp21_prereg_rev5_REVIEW.md` whole; `arc3_ledger.md` from
"§3 — ROUND 3 OF BOTH REGISTRATIONS" to the end; `docs/decisions.md` D-423, D-424,
D-539, D-540, D-562, D-563, D-568, D-572, D-576, D-577, D-581, D-584, D-586, D-596;
`wp21_throughput_prereg.md` revision 5 §3.3–3.6, §4.4, §4.5, §5, §7, §8;
`tools/wp21_tranche_config.py`, `tools/cold_label_check.py` (whole), `tools/wp21_assemble.py`
(CLI); `crates/pistol-arena/src/{capture,capture_file,openings,schedule,outpath,usage,config,validate,conclusion}.rs`,
`src/bin/arena.rs`; `crates/pistol-arena/tests/wp21_tranche_config_tests.rs`;
`configs/instrument_v0.toml:113`, `configs/arena_wp20_label_pilot.toml`;
`docs/book_v2_ledger.md:42`; the dry-run log `artifacts/arc3r_dryrun_sweep_0c4f3b4_v3.txt`
and its driver `/home/tom/pistol-runs/arc3r-dryrun/sweep_dryrun.sh`; the run directory
`/home/tom/pistol-runs/arc3r-dryrun/sweep/` (read-only); `artifacts/wp20pilot_RUN_2cd4f79_v1.txt`,
`artifacts/arc3_leverB_41_count_v3.txt`, `artifacts/arc3_opening_prefix_fold.txt`,
`docs/experiments/wp20_CLOSURE.md:46`; `wp21_assemble_REVIEW_round2.md` header.

**WHAT I RAN, AND WHERE.** Everything under `cargo`, every tool and every arena
binary ran in my own detached worktree `/home/tom/pistol-wt/prereg-r4` at `735fc37`
with `CARGO_TARGET_DIR=/home/tom/pistol-wt/prereg-r4/target` per command, never
exported, never in the live tree: `cargo build --workspace --release --locked` (a
fresh `target/`, a different absolute path from the live tree); `cargo test -p
pistol-arena --locked` over the four coverage-rule suites; the shipped generator over
all sixteen tranches, the T-F window, the stand-in and four refusal edges; my own
`pistol` as the cold referent over the dry run's capture at strides 1 and 2 and with
two deliberately broken referents; the registered T-F body compare (with its `head
-n`) on the stand-in and on a doubled body; the recorded `bash -c` lines copied as
printed. The `artifacts/` files are gitignored and so absent from a worktree; they
were read from the live tree's copies by path, never modified. No tranche, no
218-opening play, no registered workload. Counts were taken with `/usr/bin/grep`,
`git grep`, `git show`, `sed`, `awk`, `diff`, `cmp`, `sha256sum`, `python3`. The
worktree is removed at the end.

## VERDICT: **FAIL**

**0 BLOCKING, 5 MAJOR, 8 minor.**

Revision 6 closes all sixteen of round 3's findings by my own reading and, where a
run was named, by my re-taking it: T-E is gone; §5's rule is keyed on pass 2; limb 5
holds on its own record and the ledger's §3 now says so; an INTERRUPTED tranche has
a disposition; the fold receipt's digest is in §8 and in the ledger; D-596 exists and
says what the preamble says; the assembler's digest is stated as `5b17132`'s past
its `9c4366c` PASS; T-F tests what §9 runs and has a cross-report referent;
`config_check.sh` is out of the registered block; the four addends reproduce;
`partitioned()`; the T-A VOID is read by what it names, and the names are the
script's; the gate is stated in §5 once; the three deletions are taken; T-F runs
before wave one; limb 3 reads the cached line. The record of §9.1 is mechanically
the log's — 20 of 20 commands, 21 of 21 printed lines, the head line, the listing
`diff`-identical doc = log = disk — and the log's digest, the fold receipt's, the
four scripts' at `735fc37` and the three binaries' from my own fresh release build
all reproduce; `git diff --stat 0c4f3b4 735fc37 -- crates tools configs Cargo.toml
Cargo.lock` is empty.

What fails is in the text that revision 6 wrote or that round 3 did not open: the
T-A criterion rows quote a line the instrument never prints (`MISS record(s)` for
`MISSES record(s)`); §9's registered T-F body compare carries a `head -n` clause the
dry run did not exercise and the record does not hold, although the clause is what
makes the compare correct on a real tranche; §5's checker reads the presence of the
`cmp -s` line and not its exit status, so a cached tranche after a FAILED comparison
has no disposition; T-F, an agreement criterion, carries no registered consequence;
and `docs/book_v2_ledger.md`'s row names revision 5 while the document says it names
this one. None of these is a number a decision reads, and every registered command
runs — which is why nothing is BLOCKING.

---

# 1. DISPOSITION OF ROUND 3

| # | round-3 finding | status in revision 6 | evidence (my command, scope) |
|---|---|---|---|
| **B1** | T-E cannot fail | **CLOSED** | `/usr/bin/grep -n 'T-E' wp21_prereg.md` → nothing; §4's table has T-A1, T-A2, T-B, T-C, T-D, T-F; §5:277-278 still binds the manifest rows (*"its printed manifest row"*) |
| **M1** | §5's `cmp` rule voids tranche one | **CLOSED, with a residual** | §5:290-296: keyed on *"the PASS-2 command — the one that writes `capture.txt`"*; tranche one's `capture-cached.txt` named as *"the one flagged command that may"*. Residual: the rule reads the line's presence and not its exit (**M3** below) |
| **M2** | limb 5 fails on its own record; ledger says otherwise | **CLOSED** | §9.1:474-476 names `tranche-1/`, `tf/`, `assembly/` and *"no `run_log.txt`"*; my `diff` of the doc's listing (:562-573) against the log's `## limb 5` block and against `find . -type f \| LC_ALL=C sort` in `/home/tom/pistol-runs/arc3r-dryrun/sweep/`: identical both ways, 12 files; my extraction of every `<SWEEP_DIR>/…` token in §4.1:243-261 gives exactly those 12 for the three directories. Ledger §3:1470-1473 records the correction |
| **M3** | no disposition for an INTERRUPTED tranche | **CLOSED, with a residual** | §5:300-304, *"whose directory holds any claimed file is INTERRUPTED … treated as VOID: re-run whole under `-run<k>`, the partial directory kept"*. Decidable from the directory alone (`ls`). Residual: whether it counts toward TWO CONSECUTIVE VOIDS (**m5**) |
| **M4** | fold receipt "sha-anchored in the ledger" was false | **CLOSED** | `sha256sum /home/tom/Projects/HeXO-AlphaBeta/artifacts/arc3_opening_prefix_fold.txt` → `b6d4751e24e3594d7da827687bc3a35630c3b5052c6e6cd8f3d260076e3e97de` = §8:381; `/usr/bin/grep -c b6d4751e docs/experiments/arc3_ledger.md` → 1; `/usr/bin/grep -n sha-anchored wp21_prereg.md` → nothing; the sixteen floors in the receipt's addendum (`:55-70`) are `42 51 48 55 47 48 50 44 53 48 51 60 45 56 48 46`, my sum 792, = D-584 |
| **M5** | census OFF without an ADR line | **CLOSED** | `/usr/bin/grep -n '^D-596:' docs/decisions.md` → `:1260`, *"THE WP-2.1 SWEEP RUNS WITH THE CENSUS OFF, SUPERSEDING D-562(3) … FOR THIS REGISTRATION ONLY"*; its two grounds are D-563 and X1 (`usage.rs:116-119`, read); the preamble :25-30 quotes it |
| **M6** | assembler digest 64 lines past its PASS | **CLOSED as stated; the review it names is outside this revision** | §8:380 states `9c4366c` PASS, digest `5b17132`'s, a scoped round 3 named, *"§6 does not run before that round passes"*. Mine: `git show 9c4366c:tools/wp21_assemble.py \| sha256sum` → `8dfdb19c…`; at `5b17132` and `0c4f3b4` → `a367d847…` = §8; `git diff --stat 9c4366c 5b17132 -- tools/wp21_assemble.py` → 52+12 = 64. The named file is absent at `735fc37` (**m2**); HEAD `73979e7` lands it |
| **M7** | T-F's text and commands test different things | **CLOSED, with a residual** | T-F row :184 now states both halves as §9 runs them; §9:425-434 has the pair and the body compare. The record :522 exercises a body compare — but not the registered one (**M2** below) |
| **M8** | `config_check.sh` registered, unpinned, untested, `cargo run` | **CLOSED** | `/usr/bin/grep -n config_check wp21_prereg.md` → `:578` only (the history paragraph); §9 step 0 :392-395 names the arena's parse. The parse IS strict: `#[serde(deny_unknown_fields)]` at `config.rs:29,47,92,151,169`; `ArenaConfig::load` (`config.rs:197-202`) = `parse_unvalidated` + `validate`; `bin/arena.rs:184` calls `load` for pass 1. Residual: the pinned generator still writes *"Validate this file with tools/config_check.sh"* (**m7**) |
| **m1** | four §3 addends | **CLOSED** | re-derivation table: 11 018, 1 442, 1 409, 58, 13 927; 5 819 at 395/742 and *"5 820 at the rounded 0.5323"* stated; 176 227 (:119) and 176 228 (:152) each reproduce from the expression beside it and :152 says which is which |
| **m2** | `records_of()` named for `partitioned()` | **CLOSED** | §4:202 `partitioned()`; `cold_label_check.py:183` `def partitioned`, `:198-202` the empty-class VOID |
| **m3** | an engine VOID voids a tranche | **CLOSED** | §4:211-215. The texts ARE distinguishable by name, executed: stride 2 over 17 → `RUN VOID: 9 sampled HITS record(s) at stride 2 is below the registered floor of 10` (script `:285-287`); `--binary /usr/bin/false` → `RUN VOID: record 17 (game 1, turn 0): the engine exited 1` (`:225-229`); a 3-s stub under `--timeout-s 1` → `the engine did not answer inside 1 s` (`:221-222`); a spawn failure → `the engine could not be spawned` (`:223-224`) |
| **m4** | the gate stated four times | **CLOSED in §1 and §5; §6.1 still restates** | §1:57 points; §5:290-296 states the checker; sibling §4.4 says the checker is *"stated there and nowhere else"* ✓; §6.1:340-344 calls itself *"a pointer"* and then carries all three clauses (**m3** below) |
| **m5** | three D-424 deletions | **CLOSED** | `/usr/bin/grep -n 'setsid\|not weaker\| GB' wp21_prereg.md` → nothing |
| **m6** | T-F scheduled beside wave one | **CLOSED** | §3:158-161 *"BEFORE wave one, the box otherwise idle"*; §9:424 `# T-F, BEFORE wave one`; §9:445-446 *"no T-F"* during a tranche |
| **m7** | limb 3 internal agreement | **CLOSED** | §9.1:467-471 registers the checker's class sizes against the cached line's `asks`/`hits`; record 17 = 17, 17 = 17; my own `awk` first-seen walk over the stand-in's capture body: `misses 17 hits 17` |

**Score: 16 of 16 CLOSED** (four with a residual named below).

---

# 2. RE-DERIVATION TABLE (my command, its scope, my number, the document's)

| claim | my command (scope) | mine | document's |
|---|---|---|---|
| 13 openings, 26 games, 742 records, 657, 21 505 ms @4, 21, 671 | `/usr/bin/grep -n 'n 26\|captured 742\|capture1 seconds=\|wall 21505\|replay seconds=\|cold seconds=\|13 openings'` over the pilot log (whole file) | `:17` 13 openings, `:18` n 26, `:35` captured 742, `:39` 657, `:27` 21505 @4, `:72` 21, `:67` 671 | §3:97-100 ✓ |
| 347 NOT in the pilot log | `/usr/bin/grep -n 347` over the pilot log | one hit, `:12`, inside the binary's sha `…d20253476` — not a count; round 3's `-c` → 0 was a substring miss the other way | §3:100-102 *"NOT there"* ✓; `wp20_CLOSURE.md:46` *"742 records -> 347 distinct positions"* ✓ |
| 395 / 0.5323 | `/usr/bin/grep -n '395\|0.5323'` over the count receipt; `sha256sum` | `:24` hits 395, `:26` 0.5323; `cbad0786…` | §3:103, §8:379 ✓ |
| the seven rates | `python3` from the integers | 57.0769, 26.6923, 0.885445, 0.904313, 0.827115, 0.807692, 0.5323 | §3:108-114 ✓ |
| 6 974 / 199 027 / 93 076 | `3487·2`, `3487·742/13`, `3487·347/13` | 6 974 / 199 027.23 / 93 076.08 | ✓ |
| serial | `199027·0.885445`; `3487·57.0769·0.885445`; `/3600` | 176 227.46; 176 227.60; 48.95 h | :119 `176 227` ✓; :152 `176 228` with both spellings named ✓ |
| per tranche at 218 | `218·742/13`, `218·347/13`, difference | 12 442.77 / 5 818.92 / 6 623.85 | 12 443 / 5 819 / 6 624 ✓ |
| the four addends | `12443·0.885445`; `436·0.827115·4`; `436·0.807692·4`; `(ceil(6624/200)+ceil(5819/200))·0.904313` | 11 017.59; 1 442.49; 1 408.61; (34+30)·0.904313 = 57.88 | 11 018 / 1 442 / 1 409 / 58 ✓; sum 13 927 = 3.87 h ✓ |
| labels searched | `12443·(1−395/742)`; `12443·(1−0.5323)`; `218·26.6923` | 5 819.03; 5 819.59; 5 818.92 | 5 819, *"5 820 at the rounded"* ✓ |
| cached capture / cached tranche | `5819·0.885445`; `13927−11018+5152` | 5 152.40 = 1.43 h; 8 061 = 2.24 h | ✓ |
| the three walls and the net | python | 27 854 s 7.74 h; 21 988 s 6.11 h; 27 140 s 7.54 h; net 0.198 h | ✓ |
| T-F surcharge | `2·20·57.0769·0.885445`; `2·40·0.827115·4`; sums | 2·1 010.77 = 0.56 h; 264.7 = 0.07 h; 2.06 h; 0.63 h | ✓ |
| floor binds below 1 801; 34 and 30 samples | `9·200+1`; `floor(6623/200)+1`; `floor(5818/200)+1` | 1 801; 34; 30 | ✓ |
| 792 / 93 076 | python | 0.851 % | D-586's 0.85 % ✓ |
| partition, all sixteen | the shipped generator in my worktree, skips/takes read off the WRITTEN files with `grep -E '^openings_(skip\|take)'`, `awk` sum | `13/218 231/218 … 3065/218 3283/217`; sum 3 487; contiguous; last end 3 500 | §2 ✓; the fold receipt's sixteen skips agree |
| refusals | `--tranche 17`; `--skip 3490 --take 20`; `--skip 12 --take 2 --pilot-range`; `--skip 13 --take 20 --pilot-range` | all REFUSED by name, exit 1 | §2 *"Both forms refuse the holdout"* ✓ |
| the three forms differ only in the integers and header comments | `diff` of the written files, non-`#` lines and whole | tranche 1 vs window 13/20: `openings_take` only; tranche 1 vs stand-in: `openings_take`, `openings_skip`; whole-file: plus the header block and the range comment. The stand-in I wrote is `cmp`-identical to the dry run's file (`772958a9…`) | limb 2 ✓ as fact; the suite does not pin it (**m8**) |
| book size | `/usr/bin/grep -vc '^#' crates/pistol-cli/tests/fixtures/random_openings_v2.txt` | 4 500 | ✓ |
| §8 tree slot | `git diff --stat 0c4f3b4 735fc37 -- crates tools configs Cargo.toml Cargo.lock`; no path filter | empty; six `docs/` files | ✓ |
| §8 the four script digests | `git show 735fc37:tools/<f> \| sha256sum`, and on disk in the worktree | `6386d6bf…`, `707acacc…`, `1a890b53…`, `a367d847…`, identical | ✓ |
| §8 `MIN_SAMPLED = 10` | `/usr/bin/grep -n MIN_SAMPLED tools/cold_label_check.py` | `:85 MIN_SAMPLED = 10`, `:283` the comparison | ✓ |
| §8 the three binaries | **my own** `cargo build --workspace --release --locked` in `/home/tom/pistol-wt/prereg-r4` (fresh `target/`, different path) | `pistol 78a7600a…`, `arena a1a405cb…`, `corpus-check efbb76b6…` | ✓ byte-identical, a second independent reproduction after round 3's |
| `rustc`/`cargo` | `rustc --version; cargo --version` | `1.98.0 (88d9e12ae 2026-08-18)` / `1.98.0 (797e8a9bc 2026-08-05)` | ✓ |
| dry-run log digest, head line | `sha256sum`; `head -1` vs the doc's quoted line, `diff` | `2654f002…`; identical | ✓ |
| dry run before this revision | log head `07:08:25 UTC`; `git log -1 --format=%cI 735fc37` → `09:15:23 +02:00` = 07:15:23 UTC | 7 min before the commit | §9.1 *"before this revision's review was dispatched"* ✓ |
| every `$` command of §9.1 in the log | `grep -F -x` per line, `<DRY>` substituted, over the log | 20 of 20 present; 20 `exit=` lines, all `0` | ✓; ledger §3 *"20 commands, 20 at exit 0"* ✓ |
| every printed line of §9.1 in the log | `grep -F -x` per line (:536-556) | 21 of 21 present | ✓ |
| stand-in config digest | `sha256sum` of both generated files on disk | `772958a9…` both; 3 occurrences in the log | ✓ |
| records are written in game order | `capture.rs:359-360` `for game in &transcript.games { for k in asked_prefixes(game)? {`, one seat, one channel; `capture_file.rs` `render_records` in vector order; the stand-in's body: game 0 turns 0..16 then game 1 turns 0..16 | serial, game order | T-F's *"the records coincide"* ✓ |
| T-F's forty games ARE tranche one's first forty | `openings.rs:109-114` the window is `parsed.drain(skip..skip+take)` re-indexed `0..take`; `schedule.rs:131-134` `opening = taken[index/2]`, `a_is_p1 = index % 2 == 0`; `schedule.rs:14` records *"in index order"* | under skip 13, games 0..39 are openings 13..32 in both windows, same side assignment | ✓ |
| the registered body compare runs and is load-bearing | §9:433-434 literal on the stand-in; then over a doubled tranche body | exit 0; exit 0 with `head -n`, exit 1 without | runs ✓ — but was not dry-run (**M2**) |
| `conclusion.rs:81`, `:111` | `sed -n 79,83p;109,113p` | `:81` `"counts n {} … forfeits {} decided {}"`, `:111` `"first_player_wins {} of {} decided_non_forfeit forfeits {}…"`; both in the dry run's `report.txt` `:41`, `:44` | T-C ✓ |
| `outpath.rs:6-25`, `arena.rs:52-59`, `usage.rs:111`, `instrument_v0.toml:113` | `cat -n` / `sed -n` | `:10 pub fn claim`, `:13 create_new(true)`; `:52 [` … `:59 ] => {` with `tail @ ..`; `:111 pub fn capture_tail`; `on_search_path = false` | ✓ |
| the seat is the pilot's | `/usr/bin/grep -nE '^(turn_cap\|n_workers\|hang_timeout_ms\|kind\|value)' configs/arena_wp20_label_pilot.toml`; `[sprt]` | 40 / **4** / 120000 / nodes / 50000; elo0 0.0 elo1 15.0 alpha 0.05 beta 0.05 | §1 ✓ (`n_workers 1` disclosed at :55; SPRT block the pilot's ✓) |
| the coverage suites at `735fc37` | `cargo test -p pistol-arena --locked --test …` (my worktree) | `wp21_tranche_config_tests` 18/18, `cold_label_check_tests` 11/11, `wp21_assemble_tests` 11/11, `label_cache_count_tests` 5/5 | — |
| the citation gate | `bash tools/governing_citation_check.sh` (my worktree) | green, `DESIGN_CITATION_CHECK_DONE`, exit 0 | — (it checks `path:line`, not bare `.md` names — **m2** is invisible to it, as D-584 says) |
| assembler `--corpus` repeats | `/usr/bin/grep -n add_argument tools/wp21_assemble.py` | `:281 --corpus … action="append"` | §9:442 ✓ |
| `docs/book_v2_ledger.md`'s row | `/usr/bin/grep -n wp21_prereg docs/book_v2_ledger.md` | `:42 … revision 5` | preamble :42 *"names this revision"* ✗ (**M5**) |
| the T-A printed words | `cold_label_check.py:290-293`, `:317-320`; my own run | `MISSES record(s)` / `HITS record(s)`; the log's `:541`, `:543` the same | §4:179, :180, :199 `MISS record(s)` / `HIT record(s)` ✗ (**M1**) |
| the design's revision | `head -1 wp21_label_cache_design.md` | revision 10 | §1:57 ✓ |
| `no_tranche_reaches_the_reserved_holdout` | `/usr/bin/grep -n 'fn no_tranche' wp21_tranche_config_tests.rs` | `:129` | preamble :36-37 ✓ |
| `arc3_ledger.md` F-0.4 | `/usr/bin/grep -n 'F-0.4'` | `:192` | :22 ✓ |

---

# 3. FINDINGS

## BLOCKING

None.

## MAJOR

### M1 — T-A1's and T-A2's *"criterion, as it must read"* quotes a line the shipped instrument never prints.

§4:179: `cold_label_check: N of N sampled MISS record(s) agree byte for byte`; :180
the same with `HIT`; :199 the worked example `34 of 34 sampled HIT record(s)`. The
instrument at the registered digest prints `{want.upper()}` (`cold_label_check.py:317-320`),
which is `MISSES` or `HITS` — the record's own lines :541 and :543 read `17 of 17
sampled MISSES record(s)` and `… HITS record(s)`, and my own run prints the same.
A closure reader matching the log against the criterion *"as it must read"* finds no
line that reads so; the document's §4 and its §9.1 disagree on the criterion's
spelling. The number, the exit status and the class are right, so the criterion can
fail and no decision reads a wrong figure — which is why this is not BLOCKING — but
a quoted criterion string that the instrument cannot produce is a claim the tree
contradicts.

**FIX (unexecuted as an edit; the true text is executed above).** `MISSES` and
`HITS` at :179, :180, :199.

### M2 — §9's registered T-F body compare carries a `head -n` clause the dry run did not exercise and the record does not hold; limb 1 says every command above ran.

§9:433-434 registers `cmp -s <(grep -v '^#' tf/capture-a.txt) <(grep -v '^#'
tranche-1/capture.txt | head -n $(grep -v '^#' tf/capture-a.txt | wc -l))`. The
record's line :522 (and the driver's `:47`) ran `cmp -s <(… capture-a) <(…
capture.txt)` with no `head -n`. On the stand-in both bodies are 34 lines, so the
clause is a no-op there; on a real tranche one it is the whole compare — tranche
one's body is ~12 400 lines against T-F's ~1 140. Executed: the registered literal
on the stand-in exits 0; over a doubled tranche body it exits 0 WITH `head -n` and 1
WITHOUT. So the registered command runs and is right, and the clause that makes it
right is exactly the part `docs/process.md`'s *"literal commands are exercised"*
never saw. §9.1 limb 1 (*"every command above … exits 0"*) and *"Every command and
its exit line, verbatim"* are contradicted for this one command by the log.

**FIX (executed here on the stand-in; unexecuted as a record).** Re-take that one
line of the dry run with the registered form, or register the form the dry run took
and state that the prefix-`head` is what a real tranche needs — the former, since
the latter registers a command that fails on the workload.

### M3 — §5's checker of §6.1's gate reads the presence of the `cmp -s` line and not its exit status, so a cached tranche after a FAILED comparison has no disposition.

§5:290-296: *"the `cmp -s` exit line of the comparison … appears BEFORE the first
block whose PASS-2 command … carries `--label-cache`; a block whose pass 2 carries
the flag with no such line above it is a VOID tranche"*. A line reading `exit=1` is
*"such a line"*. §6.1:340-344 registers *"no tranche runs cached until the comparison
… returns byte-identity"* and :346-350 hands the failure branch to the sibling
(*"every remaining tranche uncached"*). A cached tranche run after `exit=1` violates
§6.1 and passes §5's checker; nothing in either document says what it is. Round 3's
M1 was fixed by re-keying the rule on pass 2 and the exit status was dropped on the
way — the second shape `fix-rounds-must-derive` warns of.

**FIX (unexecuted, one word).** *"the `cmp -s` `exit=0` line"*; and *"a block whose
pass 2 carries the flag with no such line above it — or with a non-zero one — is a
VOID tranche"*.

### M4 — T-F is an agreement criterion with no registered consequence.

`docs/process.md`: *"A registered agreement criterion carries a REGISTERED
CONSEQUENCE: the pre-registration states, before either instrument runs, what
DISAGREEMENT does to the verdict."* T-F (§4:184) compares two captures with each
other and then with tranche one's records. What a failure does is not stated
anywhere I can find: §4's void rule (:223-229) is per tranche and T-F's first half
runs *"before tranche one"*, when there is no tranche to void; its second half fails
after tranche one exists, and *"tranche one is VOID, re-run whole"* would re-run the
tranche to cure a defect in the capture INSTRUMENT (*"a capture that is not a
function of its inputs"*), which no re-run cures. The sibling's C1 (§3.6) registers
**THE SWEEP STOPS** for the same defect class in its concurrent form; T-F does not
point at it. As it stands, the after-the-numbers decision the rule exists to forbid
is left standing.

**FIX (unexecuted, prose).** One sentence in the T-F row or under the void rule:
*"a T-F failure is the sibling's C1 class: THE SWEEP STOPS before wave one (first
half) or before wave two (second half), and no tranche is re-run to cure it"*.

### M5 — `docs/book_v2_ledger.md`'s row names revision 5; the document says it names this one.

Preamble :42: *"`docs/book_v2_ledger.md`'s row for this sweep names this revision"*.
`/usr/bin/grep -n wp21_prereg docs/book_v2_ledger.md` → `:42 | 13 | 3487 | 13..3499 |
… | docs/experiments/wp21_prereg.md revision 5 |`. Round 2's n-m2 was this defect at
revision 4 → 5 and it recurred at 5 → 6. A claim the tree contradicts; rated by the
scale and not by its cost, which is one digit.

**FIX (unexecuted).** `revision 6` at `docs/book_v2_ledger.md:42`, and the sentence
in the preamble is then true.

## minor

**m1 — three recorded lines do not run as printed.** §9.1:522, :526, :528 read `$
bash -c cmp -s <(…) <(…)` and `$ bash -c tools/cold_label_check.py … | /usr/bin/grep
-o …`. The driver (`sweep_dryrun.sh:47,53,54`) ran them QUOTED; its `run()` prints
`$*`, which drops the quotes. Copied as printed, :522 is `bash -c cmp` with `-s` as
`$0` → `cmp: missing operand`, exit 2 (executed); :526 likewise exit 2. The log is
verbatim; the commands it shows are not the ones that ran. The registered forms in
§9 are unaffected. Print them quoted, or note that the driver's rendering is a shape
(CLAUDE.md's *"a captured transcript is evidence of shape"*).

**m2 — §8 cites a file the tree does not hold at this revision.** :380
`wp21_assemble_REVIEW_round3.md` — absent at `735fc37` (`ls
docs/experiments/wp21_assemble_REVIEW_round*.md` → round 2 only); the ledger says
*"dispatched"*. The citation gate is green because it checks `path:line`, not bare
names (D-584's own limitation). **Closed by HEAD `73979e7`**, which lands the file;
recorded because the review is against `735fc37` and the sentence was written as a
present fact.

**m3 — D-423, four restatements.** (a) The `arena_` basename / gate-6 courtesy at
§9:394-395 and again at :583-584. (b) §6.1:340-344 calls itself *"a pointer"* and
carries the sibling §4.4's three clauses whole; a pointer is one line. (c) :576-582
is revision history — two earlier runs, a validator registered and deleted — under a
preamble (:16-17) that says the history is *"in `arc3_ledger.md` and nowhere here"*;
the ledger §3 M8 paragraph tells it. (d) :217-221 (*"THE SUB-RANGE FOR T-F IS
REGISTERED HERE"*) restates the T-F row's skip, take, pair, `cmp -s` and body
compare; the row owns them.

**m4 — D-424, prose that constrains nothing.** :221 *"Twenty is a round number above
the pilot's thirteen, fixed before the tranche"* and :583 *"it costs nothing"*.
Neither changes a reading.

**m5 — INTERRUPTED and the two-voids STOP.** §5:300-304 treats an interruption as a
VOID; §4:231-234 stops the sweep at two consecutive VOIDs *"because the fault is in
the seat or the instrument"*. A reboot is neither. Whether an INTERRUPTED tranche
counts toward the two is unstated, and the two readings license different
conclusions (STOP or continue) after one reboot and one genuine void. Say which.

**m6 — a VOID tranche one and the referent pair's paths.** §4.1:252-253 and §9:437-439
register `tranche-1/capture.txt` and `tranche-1/capture-cached.txt` as the sibling's
§4.4 pair; §6.1 says the referent is tranche one's *"corpus of record"*. If tranche
one is VOID and re-run under `tranche-1-run2/`, the corpus of record is `run2`'s and
the registered paths name the void run. One clause: *"or its passing `-run<k>`"*.

**m7 — the pinned generator writes a pointer to the deleted step.**
`wp21_tranche_config.py:172` → every generated config's `:25` (the stand-in on disk
included): *"Validate this file with tools/config_check.sh."* §9 step 0 says *"no
second validator is registered"*. A config that tells its reader to run a validator
the registration deleted is the two-documents-one-claim defect on the config's own
face. The generator is digested in §8, so the one-line change reopens this document —
which is the honest cost, and why it is named now rather than found in the run log.

**m8 — limb 2's *"which the generator's own suite pins"* over-reads the suite.**
§9.1:463-466: the forms *"differ … in the two integers and the header comment only,
which the generator's own suite pins over all sixteen tranches and the T-F window"*.
The suite pins the INTEGERS (contiguity and sum over sixteen; skip/take and the first
line for the window and the stand-in) and the full seat for the tranche form only
(`:159-186`); no test compares two forms' outputs, and the stand-in's `turn_cap`,
`n_workers`, `hang_timeout_ms` and budget are pinned by nothing but the template.
The fact holds — one `document()` template (`:135-223`), and my `diff` of the three
written files shows the integers and comment lines and nothing else — so the license
not to play the tranche form is sound; the sentence should cite the template, or the
suite should gain the diff.

---

# 4. WHAT SURVIVED ATTACK

**The record is the log's, mechanically and in both directions.** 20 of 20 `$` lines
(with `<DRY>` substituted) and 21 of 21 printed lines are in the log by `grep -F -x`;
the head line is `diff`-identical; the listing is identical doc = log = disk; 20
`exit=` lines, all `0`; the log's digest, the stand-in config's digest (in the log
three times, on disk twice) as stated.

**The digests are the build's, twice over.** A second fresh worktree at a second
path, its own `target/`, `--release --locked`: `78a7600a…`, `a1a405cb…`, `efbb76b6…`.
`git diff --stat 0c4f3b4 735fc37 -- crates tools configs Cargo.toml Cargo.lock` is
empty; the four script digests at `735fc37` are §8's; the fold receipt's is §8's and
the ledger's.

**T-F's second half is well-defined on the real workload.** Records are rendered in
the order asked (`capture_file.rs` `render_records`), asked in `transcript.games`
order over one serial channel (`capture.rs:359-360`), and games are indexed
`2·opening + side` over a window re-indexed from zero (`schedule.rs:131-134`,
`openings.rs:109-114`) — so under one book and one skip the first forty games of a
218-take are the twenty-take's forty, and their records are the first K body lines.
`grep -v '^#'` leaves exactly the record lines (the stand-in: 49 lines, 15 `#`, 34
records, no blanks); the headers differ only in `source_sha256` (my `diff`). The
`head -n` form is correct and load-bearing — which is M2's point, not a rejection.

**The T-A VOID rule is decidable by the instrument's own words**: `floor` in the
floor VOID, `the engine` in all four engine VOIDs, executed. The empty-class,
digest-mismatch and field-count VOIDs name neither and fall under the general rule
(*"a void T-A voids the tranche"*), which is the right default for a capture defect.

**The INTERRUPTED rule is decidable from the directory alone.** *"Any claimed file"*
is any file — the generator's `open(…, "x")` and `outpath::claim`'s `create_new` are
the same `O_EXCL` — and a re-launch in place would refuse at the first claim
(`outpath.rs:13`), which is what the rule says.

**Step 0 needs no second validator.** `deny_unknown_fields` on every arena section
(`config.rs:29,47,92,151,169`), `load` = parse + `validate` (`:197-202`), called at
pass 1 (`bin/arena.rs:184`); the first dry run showed the arena accepting the very
file the validator refused.

**The pass-2 key resolves round 3's M1.** Tranche one's re-capture writes
`capture-cached.txt`, is not pass 2, and is named as the one flagged command that
precedes the line; wave-two blocks are appended at completion, after it. (M3 is the
residual, and it is the exit status, not the key.)

**Every §3 figure reproduces from the rate beside it**, the four addends included,
and the two serial spellings are each right for their own expression. Nothing a
decision reads is wrong.

**The partition, the holdout and the pilot-range wall reproduce from the shipped
generator**, read off the written files; the fold receipt's sixteen skips are the
generator's.

**T-C's three readings exist** at the cited lines and in the dry run's report.

**The coverage rule holds for the four instruments**: 18, 11, 11, 5, all green at
`735fc37` in my worktree.

**D-596 is the ADR line M5 asked for**, with the grounds the preamble quotes and a
flip clause.

---

# 5. ATTACKS I ATTEMPTED AND REJECTED

1. **"The SPRT block can stop tranche one early, so its first forty games are not
   T-F's."** Rejected — both seats are one engine, every pair is `p2`, `llr_pair
   none` (the dry run's report), and `first_crossing_pairs` never fires on a
   degenerate sample; §1:59 says so.
2. **"`cmp -s` over the two T-F captures fails on the header's path."** Rejected —
   the `path` is on the printed manifest row, not in the file; `tf/capture-a.txt`
   and `capture-b.txt` are `cmp`-identical on disk and their bodies digest alike.
3. **"The dry run's `== ps before` printed nothing, so the idle check did not run."**
   Rejected — `sweep_dryrun.sh:14` greps `ps` for `cargo|rustc|arena |pistol` and
   prints matches; nothing matched. Empty is the pass.
4. **"The log says `at c4963b3`, so it is not this revision's dry run."** Rejected —
   `735fc37` is docs-only above `c4963b3` (six `docs/` files, no path filter), every
   instrument and binary is byte-identical, and §9.1 quotes the head line honestly.
5. **"347 IS in the pilot log."** Rejected — the one hit is four characters inside a
   sha256 at `:12`; the count is only in the closure, as §3 says.
6. **"The ten-sample floor voids T-A on a real tranche."** Rejected — 1 801 records
   per class suffice and the smaller class is ESTIMATED 5 819.
7. **"`asks` is not the miss count, so limb 3 compares unlike things."** Rejected —
   under the cache `memo.asked()` increments only on a miss (`capture.rs:371-374`),
   and on the record `asks 17` = the checker's `17 are MISSES` = my `awk` walk.
8. **"T-A2 on an uncached tranche is vacuous."** Rejected as round 3 did — the
   referent is a fresh process, not the miss record.
9. **"`head -n` over `wc -l` miscounts when the body lacks a final newline."**
   Rejected — `render_records` uses `writeln!`, and the stand-in's body ends in a
   newline (49 lines by `wc -l`, 49 by `grep -n`).
10. **"The `arena_` basename is a registered dependency on `config_check.sh`."**
    Rejected — §9:394-395 says the preflight is not a registered step; the basename
    constrains nothing (m3/m4 name the restatement, not a dependency).
11. **"§8 lacks a digest for the §9 shell pipelines."** Rejected — a command block the
    document prints is its own revision (`docs/process.md`), and §9 is printed whole.

---

**Worktree.** `/home/tom/pistol-wt/prereg-r4` removed at the end of this review;
nothing gitignored in it needed exporting (my scratch outputs of a toy run and the
regenerated configs, whose digests are in the table above).
