# REVIEW-design (fresh context) — `wp21_throughput_prereg.md` revision 5, ROUND 4 of 5 (D-585)

**NAMED REVISION.** `docs/experiments/wp21_throughput_prereg.md` as of commit
**`735fc370c7018ce1e77ddd8a1befc960e3397891`**, branch `dev`.

**DOES IT STILL MATCH HEAD?** **NO — and the difference is not in anything this
review read.** `git rev-parse HEAD` returned `735fc37` at the start and
`73979e7675696022b81e429f87b5437f8ac47db3` at the end. The one commit between
them is docs-only (`wp21_assemble_REVIEW_round3.md` new, `arc3_ledger.md` +22);
`git diff --quiet 735fc37 HEAD -- docs/experiments/wp21_throughput_prereg.md`
exits 0 (byte-identical), so does the same over `wp21_prereg.md` and over
`crates tools configs`. Every finding below is against `735fc37` and holds at
HEAD.

**WHERE.** A detached worktree `/home/tom/pistol-wt/throughput-r4` at `735fc37`,
`CARGO_TARGET_DIR=/home/tom/pistol-wt/throughput-r4/target` per command, scratch
under `scratch/` there; nothing in the live tree was written but this file. The
worktree is removed at the end (it held no `artifacts/` or `sessions/`). Nothing
of lever A's registered settings was run: the largest run here is the registered
harness at N = 2 on the dry-run stand-in at `--label-nodes 2000`, under a second.
The other reviewer's worktree (`/home/tom/pistol-wt/prereg-r4`) was not touched
and no `pkill` was issued.

**WHAT I READ.** CLAUDE.md (Process; hard rules 1, 4, 5, 6); `docs/process.md`
in full; the document; `wp21_throughput_prereg_rev4_REVIEW.md` in full;
`arc3_ledger.md` from "§3 — ROUND 3 OF BOTH REGISTRATIONS" to the end;
`docs/decisions.md` D-576, D-581, D-586, D-587, D-588, D-595, D-596 (and D-423,
D-424 for the rules applied); `wp21_prereg.md` revision 6 in full;
`wp21_label_cache_design.md` revision 10 in full; `tools/determinism.sh`;
`crates/pistol-arena/src/{capture.rs,label_cache.rs,passes.rs}`, `usage.rs:105-125`,
`bin/arena.rs:50-60`; `/home/tom/pistol-runs/arc3r-leverA/lever_a_harness.sh`
and the unregistered `lever_a.sh` beside it; `artifacts/arc3r_dryrun_throughput_0c4f3b4_v2.txt`
in full; `/home/tom/pistol-runs/arc3r-dryrun/throughput-v2/` and
`throughput_dryrun_v2.sh`; `/home/tom/pistol-runs/wp20pilot-artifacts/{capture_v1,report_v1}.txt`;
`wp21_throughput_prereg_rev2_REVIEW.md:705-722`; `artifacts/arc3_opening_prefix_fold.txt`;
`artifacts/wp20pilot_RUN_2cd4f79_v1.txt`.

---

## VERDICT: **FAIL — 1 BLOCKING, 3 MAJOR, 13 minor**

Revision 5 closes sixteen of round 3's eighteen findings and leaves two PARTIAL,
and the closures are real: the harness is printed, digested, byte-identical to
the file, and it reproduces on my own build (every printed field is what §3.3 and
§4.5 read, `records` off the file equals the counts line's, all captures
byte-identical including the dry run's); the design is cited at 10; the pair is
costed on the report the binary will accept; C3 and C4 are read at the median
rep; the stand-in is opening 3 and disjoint; every §8 digest re-takes; all 40
command/exit lines and 20 printed lines of §7.1 are in the log in order.

It fails on one clause revision 5 added. **§4.5 applies its 5 % bookkeeping
test to tranche one by dividing two walls taken at different concurrencies**:
tranche one's uncached pass 2 runs in wave one, N processes on the box, and its
cached re-capture runs alone between the waves (the sibling's §3 puts it on the
serial critical path). The contention factor lever A exists to measure sits
inside that quotient. Read two-sided, the clause abandons lever B whenever
`c(N) > 1.05` — the dry run's own two lines already show `c(2) = 1.41` at a toy
budget; read one-sided, it is blind to bookkeeping up to `c(N) − 1.05` of a
search. Neither reading is the criterion §4.5 names, and which one applies is
decided after the numbers. The play-pass pair has no such defect: both walls are
one harness at N = 1.

The three MAJORs are the class round 3 named as MAJOR 1 recurring in the same
commit: the sibling is cited at revision 5 while revision 6 landed in `735fc37`
itself; §1 restates the sibling's tranche addends at the values revision 6
corrected (11 017 / 1 443 against 11 018 / 1 442) under a ONE LINE saying the wall
is not restated; and §3.4's tie rule, stated for a pair, has two readings over a
field of five that select different N on a plausible outcome.

---

# DISPOSITION OF ROUND 3 (revision 4 → revision 5)

| # | round-3 finding | status | evidence at `735fc37` |
|---|---|---|---|
| **B1** | lever A's wall had no instrument; N ≥ 2 never dry-run; §4.5's walls the same hole | **CLOSED** | §7 prints the harness whole; `sha256sum` of `/home/tom/pistol-runs/arc3r-leverA/lever_a_harness.sh` = `baf4a6d9…` = §8's = my `awk`-extracted §7 block's (`cmp` exit 0). Run on my build (table below): N = 1, N = 2, the pair with and without `--label-cache`; `wall_s`, `records`, `asks`, `s_per_label`, `s_per_ask`, `throughput` all printed; `records 32` = my `awk '!/^#/ && NF'` over the file = the counts line's. §3.3 reads those fields; §4.5 names `s_per_ask`/`s_per_label`; the log has `leverA:` at N = 1 and N = 2. Residue: **minor 1** (`rc` unread), **minor 2** (C1/C2 over the 45 files uncommanded), **minor 13** (no receipt for the fifteen lines) |
| **M1** | design cited at 7, tree at 10 | **CLOSED** | §GOVERNING and §4.2 read *"revision 10"*, with *"8–10 changed test rows and §3's measured residual only — D-589, D-595"*; `head -1` of the design says 10. **The class recurs on the sibling — MAJOR 1** |
| **M2** | §4.4/§5 costed a pilot-report pair the binary refuses | **CLOSED** | §4.4: *"the §3.1 play-pass report (152 records, 72 distinct `position` lines, hit rate 0.5263 …; the pilot's own 13-opening report is not capturable under this revision's binary"*; §5 `~2.3 min + ~1.1 min`. My Python over `capture_v1.txt` games 0–5: **152 / 72 / 0.5263** (multiplicities {2: 70, 6: 2}). 152 × 0.885445 = 134.6 s = **2.24 min** (minor 11) |
| **M3** | C3/C4 aggregation over reps unregistered | **CLOSED** | §3.3: *"C3 and C4 are read at that median rep and at no other"*; §3.4 and §3.5 say *"at its median rep"* |
| minor 1 | SMT note cannot fire | **CLOSED** | `grep -i SMT` → 0 |
| minor 2 | `TR mod N` rule and the prohibition both vacuous | **PARTIAL** | §3.2's rule is gone; §3.4 now carries *"The selected N must divide the tranche count"* as a GUARD, which no member of {1, 2, 4, 8, 16} can fail — **minor 3** |
| minor 3 | 7.74 h restated | **CLOSED** for the wall (`grep -c '7\.74'` → 0) — **but the addends are now restated at superseded values, MAJOR 2** |
| minor 4 | §4.1 block a trimmed transcript | **CLOSED** | the eleven `label_cache_count:` lines of `artifacts/arc3_leverB_41_count_v3.txt` `diff` clean against §4.1's block |
| minor 5 | agreement criterion blind to under-fold | **CLOSED** | §4.1 says so, names the test (`label_cache_count_tests.rs:107`, my grep) and the 792 |
| minor 6 | `wp20b_perf_guard.sh` uncheckable | **CLOSED** | cited through `wp20b_artifacts.md`, whose lines 105/113 name the arm-order rotation |
| minor 7 | §3.4 → §3.5 cross-reference | **CLOSED** | *"§3.5 C3's 120 s watchdog"* |
| minor 8 | `sort -u` lines echoed, not executable | **PARTIAL** | now printed as `bash -c /usr/bin/grep -v '^#' <SCRATCH>/corpus.txt \| …`; the script ran `bash -c "<whole pipeline>"` and echoed `$*` unquoted. Pasting the printed line runs `bash -c /usr/bin/grep` with `-v` as `$0` and prints `0` at **exit 0** (executed) — **minor 7** |
| minor 9 | stand-in inside the registered range | **CLOSED** | `openings_skip = 3`, `openings_take = 1` in the stand-in config (sha `9644fc9f…`, re-taken) |
| minor 10 | lever-A cost as a bracket; C4's twenty reasoned | **CLOSED** | `0.56–1.12 h` (mine 0.561 / 1.122); C4 cites the 1.0042 — re-derived from `capture_v1.txt` (mean `nodes` 342 359 all / 343 788 games 0–5, ratio **1.0042**) — but attributed to *"A7"*; it is round 2's **A3** (`rev2_REVIEW.md:711-718`) — **minor 5** |
| minor 11 | counts line has two shapes | **CLOSED** | §4.2 |
| minor 12 | "produce a corpus" vs the second instrument | **CLOSED** | §6.3 *"corpus OF RECORD"*; `arena --labels` printed in §7 |
| minor 13 | `~6 600`, `~53 %` unmarked | **CLOSED** | both ESTIMATED |
| minor 14 | 152 measured on the pilot's report | **CLOSED** | §3.1 attributes it and says C2 reads the new report. Residue: §3.1's evidence is *"opening 0's two games"* — the superseded run's; the registered record is opening 3 — **minor 6** |

**Tally: 16 CLOSED, 2 PARTIAL.**

---

# RE-DERIVATION TABLE

Every count was taken with a command I chose, in my worktree at `735fc37` unless
the scope says otherwise. "Doc" is what revision 5 states.

| claim | my command (scope) | mine | doc | match |
|---|---|---|---|---|
| tree diff binaries' commit → this one | `git diff --stat 0c4f3b4 735fc37 -- crates tools configs \| wc -l` | **0** (full diff: four `docs/` files) | §8 | ✓ |
| `pistol`, `arena` digests | `cargo build --release --locked -p pistol-cli -p pistol-arena`, own `CARGO_TARGET_DIR`, `rustc 1.98.0 (88d9e12ae 2026-08-18)`; `sha256sum` | `78a7600a…`, `a1a405cb…` | same | ✓ |
| `tools/label_cache_count.py`, `tools/wp21_tranche_config.py`, receipt, dry-run log, stand-in config | `sha256sum` (five files) | `1a890b53…`, `707acacc…`, `cbad0786…`, `947f73e8…`, `9644fc9f…` | same five | ✓ |
| harness digest; §7 block = file | `sha256sum` the file; `awk '/^```bash$/{f=1;next} f&&/^```$/{exit} f'` over the doc → `sha256sum`, `cmp` | `baf4a6d9…` both; `cmp` exit 0 | `baf4a6d9…`, "byte-identical" | ✓ |
| harness at N = 1 (my build, my worktree, the read-only stand-in report) | `lever_a_harness.sh <report> 1 1 2000 scratch/hr/leverA` | `wall_s 0.548 records 32 asks 32 s_per_label 0.017129 s_per_ask 0.017129 throughput 58.380`, exit 0 | log: `0.544 / 32 / 32 / 0.017008 / 58.794` | ✓ shape and counts; walls within 1 % |
| harness at N = 2 | `… 2 1 2000 scratch/hr/leverA` | `wall_s 0.755 records 32 asks 32 s_per_label 0.023591 throughput 84.780`, exit 0 | log `0.766 / 0.023930 / 83.576` | ✓ |
| the pair, off / on | `… 1 1 2000 scratch/hr/pair` and `… --label-cache` | off `0.555 / 32 / 32 / 0.017351`; on `0.326 / 32 / 16 / s_per_label 0.010186 s_per_ask 0.020372` | log `0.561 / 0.017516`; `0.320 / 16 / 0.009992 / 0.019984` | ✓ |
| `records` off the file, independently | `awk '!/^#/ && NF' <file> \| wc -l`; `awk -F'\t' … {print NF}` | 32 rows, 5 fields, for N1-p1, N2-p2 and the cached file | harness `records 32`; counts line `records 32` | ✓ |
| C1's shape | `cmp` N1-p1 = N2-p1 = N2-p2 = pair-p1 = pair-cached = the dry run's `pair/cap-N1-rep1-p1.txt` | all identical | limb 1, limb 3 | ✓ |
| §7.1 record vs log | Python: both fenced blocks, `<SCRATCH>`/`<HARNESS>` substituted, searched line-exact, in order | **40/40** command+exit lines, in order; **20/20** printed lines; the log's 20 `$` lines = the doc's 20; head line present | "verbatim" | ✓ (minor 7 on the two `bash -c` echoes) |
| stand-in disjoint from `0..2` | `grep openings_ <toml>`; report header | `openings_skip = 3`, `take = 1`; `openings_skip 3` | opening 3 | ✓ |
| toolchain transfer, **opening 3** (the registered record) | Python: `moves` lines of the v2 `report.txt` games 0–1 vs `report_v1.txt` games 6–7 | identical, 16 turns each; pilot records for games 6+7 = **32** | (not claimed; §3.1 claims opening 0) | ✓ — minor 6 |
| toolchain transfer, opening 0 (the superseded run) | same over `/home/tom/pistol-runs/arc3r-dryrun/throughput/report.txt` vs games 0–1 | identical | §3.1 | ✓ |
| 152 / 72 / 0.5263; 742 / 347 / 0.5323 | Python over `capture_v1.txt` (sha `4563f050…`), `game < 6` and all | 152 / 72 / 0.5263, {2: 70, 6: 2}; 742 / 347 / 0.5323 | same | ✓ |
| asked prefixes per game | `capture.rs:29-44`: `0..=last` minus `last` if decided; pilot per-game counts 17,17,18,18,41,41 | 152 = Σ; a decided game gives `turns`, a capped one `turns + 1` | (C2's referent, unstated) | ✓ — minor 2 |
| C4's 1.0042 | Python: mean of `nodes` in field 4 over 742 / over games 0–5 | 342 359 / 343 788 = **1.0042** | 0.42 %, "A7" | ✓ number; ✗ citation (A3) — minor 5 |
| C3, C4 band, §5 lever-A cost | `4 × 0.885445`; `0.885445 × {0.8, 1.2}`; `15 × 152 × 0.885445` | 3.5418; [0.708, 1.063]; 2 018.8 s = 0.561 h / 1.122 h | 3.54; 20 %; 0.56–1.12 h | ✓ |
| §4.4/§5 pair cost | `152 × 0.885445`, `72 × 0.885445` | 134.6 s = **2.24 min**; 63.8 s = 1.06 min | ~2.3, ~1.1 | ≈ (minor 11) |
| §1's tranche addends vs the sibling | `grep -n '11 01[78]\|1 44[23]'` both docs | sibling rev 6 §3: **11 018**, **1 442**, 27 140; this doc §1: **11 017**, **1 443** | — | ✗ — MAJOR 2 |
| §1's percentages | `11018/13927`, `1442/13927`, `1409/13927`, `58/13927` | 0.7911, 0.1035, 0.1012, 0.0042 | 79.1 / 10.4 / 10.1 / 0.4 | ✓ (10.35 rounds either way) |
| §5 net and gate-free saving | sibling §3: `27 854 − 27 140`, `27 854 − 21 988` | 714 s = 0.198 h; 5 866 s = 1.63 h | ~0.20 h; ~1.6 h | ✓ |
| the sibling's revision | `head -1 docs/experiments/wp21_prereg.md`; `git log -- wp21_prereg.md` | **revision 6**, landed in `735fc37` | "revision 5" (line 13) | ✗ — MAJOR 1 |
| gate 9 seats, budgets, absences | `awk '/^SEATS=\(/,/^\)/' … \| grep -o '"[^"]*"'`; `grep -n '^BUDGETS=\|LAYOUT_BUDGET='`; `grep -c 400000`; `grep -c instrument_v0` | five (radius, staged, staged-heuristics, staged-solver, staged-safety-net-cap); `depth_turns 4`, `nodes 200000`; solver `depth_turns-2 nodes-10000`; 0; 0 | §2 | ✓ |
| memo scope, `go` once, `asks` at the call, refusal first | `capture.rs:342-401`; `label_cache.rs:100-138` | `refuse_census_under_cache` first; `go` before `with_seats`; `Memo::new` inside the closure; `memo.asked()` before `ask`; `insert` returns under `Off` | §2.0, §4.2 | ✓ |
| counts line two shapes; no elapsed time printed | `label_cache.rs:52-69`; `passes.rs:84-99` | off/on shapes; `passes.rs` prints no wall | §4.2; §3.3's wall is the harness's | ✓ |
| the tail parser and the capture arm | `usage.rs:111` `pub fn capture_tail`, `:116` both orders; `bin/arena.rs:52-59` `tail @ ..` | as cited | §3.2, §4.2 | ✓ |
| the under-fold test; the 47-line stage; the fold receipt | `grep -n a_mirrored… label_cache_count_tests.rs` → `:107`; `grep -n 'def ' label_cache_count.py` → `stones_of :101`, `images :138`, `return out :147`; `tail` of `arc3_opening_prefix_fold.txt` | present; 101–147; SUM 792, MIN 42, MAX 60 | §4.1, §2 | ✓ |
| pilot referents | `grep -n 'capture1 seconds=\|742\|n 26' wp20pilot_RUN_2cd4f79_v1.txt` | `n 26 distinct-n 13`, `captured 742`, `capture1 seconds=657` | §8, §1 | ✓ |
| ADR lines exist | `grep -c '^D-<n>:'` for 137, 291, 423, 424, 539, 558, 560, 562, 576, 581, 584, 585, 586, 587, 588, 589, 595, 596 | 1 each | cited | ✓ |
| citation gate | `bash tools/governing_citation_check.sh` | `wp21_throughput_prereg.md: 15 citation(s) checked, 0 unreproduced` | — | ✓ |
| box shape | `lscpu` | 3700X, 8 cores, 16 threads | 8 physical, 16 threads | ✓ |
| **harness: refused run** | `lever_a_harness.sh <pilot report_v1.txt> 1 1 2000 …` (digest mismatch, exit 2 from `arena`) | `grep: … No such file`; Python `ValueError: invalid literal for int() … ''`; **no `leverA:` line; exit 1** | (unstated) | voids loudly — minor 1 |
| harness: N = 0 / empty | `… 0 1 2000 …` | traceback, no line, **exit 0** | (unregistered N) | minor 10 |
| harness: unquoted `$TAG` | `"--label-cache --census"` as one word → refused, no line, exit 1; `--census` alone → line reads `cache off`, files named `-cached`, a census file written; `--bogus` → no line, exit 1; a third word ignored | as stated | registered use is one word | minor 10 |
| harness: timing bracket | read: `t0` after `mkdir`, before the loop; `t1` after the `wait` loop, which returns only after every pid has exited | first spawn → last exit | §3.3 | ✓ |
| the printed `bash -c` line, verbatim | `timeout 5 bash -c "bash -c /usr/bin/grep -v '^#' <path> \| cut -f5 \| LC_ALL=C sort -u \| wc -l"` | grep usage error, prints **0**, **exit 0** | "verbatim" | ✗ — minor 7 |
| what the dry-run script ran | `cat throughput_dryrun_v2.sh` | `cd /home/tom/Projects/HeXO-AlphaBeta`; `run tools/config_check.sh …` (which `cargo run`s three validators, `config_check.sh:103-113`); `run bash -c "<pipeline>"` | §7 registers no `config_check.sh` | minor 8 |

---

# BLOCKING

## BLOCKING 1 — **§4.5's 5 % clause, applied to tranche one, divides a wave-one wall by a between-the-waves wall; the contention factor is inside the quotient, and the clause either fails on contention alone or is blind to what it names — which, is chosen after the numbers.**

§4.5: *"the cached capture's measured seconds per MISS … is within 5% of the
uncached capture's `s_per_label` … the last excluding a cache whose bookkeeping
ate its own saving. Over the play-pass report both are §7's commands; **over
tranche one's report the two walls are the run log's `seconds=` on its two
pass-2 commands** (`wp21_prereg.md` §5), divided by that block's `asks` and
`records`."* And: *"a failure of either clause abandons the lever per §4.4."*

Where the two walls come from, read off the sibling (revision 6):

- tranche one's uncached pass 2 (`capture.txt`) runs in **wave one**, N tranches
  at once — §1's slot row, §3 *"wave one's realised seconds-per-label is read for
  contention"*, §9's idle rule *"before each wave"*;
- tranche one's cached re-capture (`capture-cached.txt`) runs **between the
  waves, alone**: §3's gate line adds it serially (`13 927 + 5 152 + 8 061`), and
  §3's last paragraph says *"the referent re-capture sits between the waves as
  the gate line says"*; §9 lists it under *"tranche one only, between the waves"*.

So the quotient §4.5 registers for tranche one is
`(seconds_cached / asks) / (seconds_uncached / records)` ≈ `t_miss / (c(N) × t_miss)`
for a perfect cache, i.e. **`1 / c(N)`**, where `c(N)` is the number lever A
exists to measure and §3.5 C3 tolerates up to 4.

**Reproducer, arithmetic on the document's own numbers.** The registered dry
run's two lines give `c(2) = 0.023930 / 0.017008 = 1.41` at a toy budget (my
re-take: 1.38). Take any `c(N) ≥ 1.06` at the selected N:

- two-sided *"within 5 %"*: the cached quotient is `1/c(N) ≤ 0.94` of the
  uncached one → the clause FAILS → *"lever B is abandoned and the sweep runs
  uncached"* — on contention, not on bookkeeping. At `c(8) = 1.3` (uncached
  1.151 s per label, cached ≈ 0.885 s per miss) the cache is 23 % "too fast";
- one-sided (cached not more than 5 % slower): a cache whose bookkeeping adds
  up to `(c(N) − 1.05) × 0.885 s` per miss — 0.22 s at `c = 1.3`, a quarter of a
  search — passes. That is *"a property the named defect class PRESERVES"*
  (`docs/process.md`, Criterion and defect class): the criterion cannot be
  falsified by the bookkeeping it names until the bookkeeping exceeds the
  contention.

Neither reading is the clause as written, and the document does not say which
applies, so the day the run log's two `seconds=` are divided, a reader chooses.
Under the dispatch's scale that is *"a decision rule leaving an after-the-numbers
choice"*, and under the two-sided reading it is also a criterion that fails for a
reason outside its defect class, which is the mirror of one that cannot fail.
Round 3 could not have seen this: the tranche-one form of the clause is
revision 5's (ledger §3: *"§4.5 gets … the run log's `seconds=` for tranche
one"*), and round 3's A5 rejected the attack on the clause only for the
same-harness, same-N pair.

**The play-pass half is sound**: both walls are §7's harness at N = 1 on the same
box minutes apart, and A5's algebra applies (the clause fails at
`h + fold/asks > 44 ms`).

**Fix (unexecuted, one deletion).** Strike *"over tranche one's report the two
walls are the run log's `seconds=` on its two pass-2 commands (`wp21_prereg.md`
§5), divided by that block's `asks` and `records`"* and say instead that the 5 %
clause is decided on the play-pass pair alone, and that tranche one's referent
pair contributes byte-identity and nothing about its wall. No number in the
document moves; §4.4's *"taken twice"* stays for byte-identity. (Correcting by
lever A's measured `c(N)` instead would be *"a margin derived to defend a single
sample"* — the wrong instrument by the same page.)

---

# MAJOR

## MAJOR 1 — **The sibling is cited at revision 5; the tree holds revision 6, landed in the same commit as this revision.**

§GOVERNING (line 13): *"`wp21_prereg.md` revision 5 §1 (the slot lever A fills),
§3 (the wall), §4 …, §6.1"*.

```
$ head -1 docs/experiments/wp21_prereg.md
# WP-2.1 — the production label sweep. RUN REGISTRATION, revision 6.
$ git log --format='%h %ad %s' --date=iso -3 -- docs/experiments/wp21_prereg.md
735fc37 2026-09-03 09:15:23 +0200 docs(arc3): wp21_prereg.md revision 6 and wp21_throughput_prereg.md revision 5 …
```

This is round 3's MAJOR 1 — *"a registration naming a superseded revision of
the design of the thing it verifies"* — closed for the design and re-opened for
the sibling in the commit that closed it. It is not cosmetic here: revision 6
changed the sections this document points at — §5's run-log rule is now keyed on
the PASS-2 command (the checker §4.4 names *"and is stated there and nowhere
else"*), §3's addends moved (MAJOR 2), and §1's slot row now names *"the design …
revision 10"*. `docs/process.md`: the instrument is named *"WITH ITS REVISION"*.
**Fix (unexecuted, one number):** *"revision 6"*.

## MAJOR 2 — **§1 restates the sibling's tranche addends, at the values revision 6 corrected, under a ONE LINE that says the wall is not restated.**

ONE LINE: *"`wp21_prereg.md` §3 owns the sweep's wall and this document does not
restate it."* §1: *"Per tranche, from `wp21_prereg.md` §3's 13 927 s: capture
**11 017 s — 79.1%**; play 1 443 s, 10.4%; replay 1 409 s, 10.1%; the cold check
58 s, 0.4%."*

```
$ grep -n '11 01[78]\|1 44[23]' docs/experiments/wp21_throughput_prereg.md docs/experiments/wp21_prereg.md
wp21_throughput_prereg.md:35:  … capture **11 017 s — 79.1%**; play 1 443 s, 10.4% …
wp21_prereg.md:126:            ESTIMATED  capture  12 443 x 0.885445 = 11 018 s = 3.06 h
wp21_prereg.md:127:            ESTIMATED  play     436 x 0.827115 x 4 =  1 442 s = 0.40 h
```

The sibling's round-3 m1 corrected the four addends (ledger §3: *"11 018,
1 442, 8 061, 21 988, 27 140"*); this document carries the pre-correction pair.
The percentages survive (11 018/13 927 = 0.7911; 1 442/13 927 = 0.1035), so no
conclusion moves — which is exactly D-423's point: a claim stated in two
documents was fixed in one. The addends are the sibling's; §1 needs only the
percentages and a pointer. **Fix (unexecuted):** drop the four seconds figures
from §1 and keep the shares, or quote the sibling's current values.

## MAJOR 3 — **§3.4's tie rule is stated for a pair; over a field of five it has two readings that select different N on a plausible outcome, and the base of the 5 % is unstated.**

§3.4: *"The sweep runs at the N whose median THROUGHPUT is highest, with guards
that fire before it: A tie inside 5% goes to the SMALLER N."*

Take medians `T(4) = 100`, `T(8) = 104`, `T(16) = 108` (an SMT gain under 5 %
and a bandwidth-bound step from 4 to 8 — both ordinary on an 8-core/16-thread
box; the dry run already shows `c(2) = 1.4` at a toy budget).

- **against the maximum**: candidates within 5 % of 108 are 16 and 8 (3.7 %);
  4 is 7.4 % under → **N = 8**;
- **pairwise, descending**: 16 ties 8 → 8; 8 ties 4 (3.8 %) → **N = 4**.

And the base: `T(8) = 100`, `T(16) = 105.2` is a tie at 4.9 % of 105.2 and not
one at 5.2 % of 100. Both are after-the-numbers choices of the kind §3.4 was
written to forbid. I rate it MAJOR rather than BLOCKING by round 3's own
reasoning for M3: the against-the-maximum reading is the one the sentence's
grammar supports (*"a tie"* is with *"the highest"*), so a reader would infer
it — but inference is what the rule says not to rely on, and here the two
readings differ by a factor of two in the sweep's concurrency.
**Fix (unexecuted, one clause):** *"the SMALLEST N whose median throughput is at
least 95 % of the highest median throughput"*.

---

# minor

**minor 1 — the harness prints `rc` and nothing reads it.** With p1 intact and
p2 dead the line carries `rc 1` and full numbers; §3.3, §3.5 and §3.6 never
mention `rc`. C2 catches the dead process only through *"no file"*, which no
registered command checks for p2..pN (minor 2). Register *"a line with `rc`
other than 0 is a VOID of that setting-rep under C2's consequence"*, or drop the
field (D-424).

**minor 2 — C1 and C2 have no printed command over lever A's 45 files, and C2's
referent has no derivation.** §7 prints `cmp -s` for the pair only; §8 registers
`cmp` as *"POSIX, no revision"* but the loop and its reference are not stated
(harmless for C1 — byte-identity is an equivalence — but unstated). C2's *"the
report's own asked-prefix count"* is computable from the report with no capture:
`turns` per decided game, `turns + 1` per capped game (`capture.rs:29-44`; pilot
games 0–5: 17, 17, 18, 18, 41, 41 = 152). One sentence makes C2's referent
external to the process it checks.

**minor 3 (D-424) — §3.4's *"The selected N must divide the tranche count"* is a
guard no member of the field can fail** (round 3 minor 2, PARTIAL). §3.2 already
says every member divides 16 and §3.4 forbids changing the partition. Delete the
guard.

**minor 4 (D-423) — C3 is stated at two scopes, and its reason's phrase is
wrong.** §3.4: *"Any N whose realised seconds-per-label … exceeds C3's threshold
is REFUSED"*; §3.5: *"at the selected N's median rep"*. The outcome coincides
(a refused maximum falls to the next, which must pass too) but one criterion
has two statements. And *"bought nothing over the smallest N that shares no
core"*: every N ≤ 8 shares no core on this box; the arithmetic in the same row
names N = 4 (`16/4 = 4`). Say *"over N = 4 at `c = 1`"*.

**minor 5 — C4's reason cites *"the round-2 review's A7"*; the 1.0042 is round
2's A3** (`wp21_throughput_prereg_rev2_REVIEW.md:711-718`; `grep -n 1.0042` finds
no other line). The number re-derives (table). One letter.

**minor 6 — §3.1's transfer evidence is the superseded run's.** *"the dry run
shows opening 0's two games move-for-move identical across the toolchain"* —
the registered record (`…_v2.txt`) is opening 3, which §7.1 says is *"this
record"* and the earlier run *"is not"*. Opening 3 transfers too (my `moves`
comparison against `report_v1.txt` games 6–7, identical, 16 turns each, 32
records), and opening 0 is on disk in the superseded run and transfers. Cite the
registered record's opening, or both.

**minor 7 — §7.1's two `bash -c` lines are still not executable as printed**
(round 3 minor 8, PARTIAL). Pasting `bash -c /usr/bin/grep -v '^#' <SCRATCH>/corpus.txt | cut -f5 | LC_ALL=C sort -u | wc -l`
runs `bash -c /usr/bin/grep` with `-v` as `$0`, prints `0` and exits 0 — the
EXIT-0-WRONG-ANSWER shape, in a record line. The script ran
`bash -c "<pipeline>"` and echoed `$*` unquoted. Print the quotes, or print §7's
plain pipeline (which is right) and say the record's form is the script's echo.

**minor 8 — §7.1's record carries a command §7 does not register:
`tools/config_check.sh`**, which `cargo run`s three validators
(`config_check.sh:103-113`) in the LIVE tree (`throughput_dryrun_v2.sh`:
`cd /home/tom/Projects/HeXO-AlphaBeta`). The sibling's round 3 M8 deleted that
step as unpinned and untested; limb 1's *"every command above"* does not cover
it; a `cargo run` in the live tree during a record taken *"on the box otherwise
idle"* is at least worth a sentence. Drop it from the next dry run.

**minor 9 — an unregistered driver at revision 4's shape sits beside the
registered harness.** `/home/tom/pistol-runs/arc3r-leverA/lever_a.sh` (08:49,
before the harness at 09:06) runs the whole lever-A schedule — `config_check.sh`,
its own clock, `--label-nodes 400000`, 15 settings, its own C1/C2 loops — from
the live tree. It is not §7's instrument and has no digest. An operator told
*"run lever A"* in that directory has two scripts to choose from. Delete or
rename it before the run.

**minor 10 — the harness's unregistered shapes, noted so nobody rediscovers
them**: N = 0 or an empty file yields a Python traceback, no line and **exit 0**
(`exit $rc` ignores Python's failure); `--census` as the tail word is accepted,
names the files `-cached`, writes a census file and prints `cache off`; any
other tail word yields no line and exit 1. None is a registered invocation. A
refused or dead p1 yields no line and exit 1, which is the right shape.

**minor 11 — §4.4/§5's *"~2.3 min"* is 2.24 min** (`152 × 0.885445 = 134.6 s`);
*"~2.2"*. The cached *"~1.1"* is right (63.8 s).

**minor 12 — §7's idle rule covers lever A only**; the §4.5 play-pass pair is a
wall the 5 % clause reads and is not under it. Extend the sentence.

**minor 13 — the fifteen `leverA:` lines have no registered receipt.** §3.3
reads *"the harness's one printed line"*; nothing says where the fifteen go, and
the amendment that fills the sibling's slot is the only consumer. Hard rule 6
ships the instrument and n; hard rule 8 sha-indexes match logs. Register a log
under `artifacts/` (the dry run's own shape) whose digest the amendment carries.

---

# WHAT SURVIVED ATTACK

**S1 — The harness is the instrument §3.3 describes, and it reproduces.** Clock
before the first spawn and after the last `wait`; N processes started together;
`records` off p1's file (my independent count agrees), `asks` off p1's counts
line; every field §3.3 and §4.5 read is on the line; byte-identical to §7's block
and §8's digest; my four runs on my own build agree with the log's to within a
percent of wall and exactly on every count. A refused run produces no line.

**S2 — Every §8 digest re-takes**, the two binaries built in a fresh worktree
digest to §8's, the tree diff under `crates tools configs` is empty, and the
citation gate reports 15/0.

**S3 — §7.1's record is faithful**: 40/40 command and exit lines present in
order, 20/20 printed lines, the log's `$` set equals the document's, the head
line and the config sha quoted exactly; the stand-in is opening 3, disjoint, and
its games transfer across the toolchain move-for-move.

**S4 — §2's soundness argument still reads off the code**: `newgame` before every
ask, the memo inside the closure, `go` once, `asks` at the call, `insert` a no-op
under `Off`, five gate-9 seats and two budgets with none at the sweep's, D-588's
correction imported.

**S5 — §4.1 re-derives whole**: 742/347/0.5323, {2: 345, 26: 2}, the block equals
the receipt's eleven lines, the 47-line stage is `:101-147`, the 792/42/60.

**S6 — C1, C2, C3, C4 can each fail on the play-pass workload at lever A** (C1 by
any load-dependent byte, C2 by an absent or short file, C3 by `c > 4`, C4 by an
external referent whose band I re-took as [0.708, 1.063] s), and each has a
registered consequence.

**S7 — §4.4's byte-identity criterion and its consequences are unchanged and
right**; §4.5's clause over the PLAY-PASS pair is falsifiable by the bookkeeping
it names (round 3's A5 algebra), with the harness now behind it.

---

# ATTACKS I ATTEMPTED AND REJECTED

**A1 — "the unquoted `$TAG` lets a multi-word tail reach `arena` as two words and
run something unregistered."** Executed: `"--label-cache --census"` reaches
`arena` as two words and is REFUSED by name (exit 2), no file, no line. The
registered invocations pass one word. Not a finding; noted in minor 10.

**A2 — "`asks` absent from the log divides by zero and prints a number."**
Executed: a refused run leaves no `.txt`, `records` and `asks` are empty
strings, `int('')` raises, no line, exit 1. It voids by accident rather than by
design, but it voids loudly. Not a finding.

**A3 — "the harness's `t1` is taken before the last process exits."** Rejected:
the `wait` loop returns only when every pid has been reaped; `t1` follows it.

**A4 — "C4 will fire because openings `0..2` are a different workload."**
Rejected again by re-derivation: mean `nodes` ratio 1.0042; the band is 20 %.

**A5 — "the toolchain moved the play, so 152 is not this study's count."**
Rejected for opening 3 (the registered record) and opening 0 (the superseded
one) by `moves`-line comparison; games 6+7 give exactly the dry run's 32.

**A6 — "the `arena` binary is verified by nothing at run time."** True — the
report attests `pistol` only, and the harness runs whatever `target/release/arena`
is in the cwd — but the sibling's §5 records the digest in the run log for
tranches, and for lever A the receipt minor 13 asks for is where it belongs.
Folded into minor 13.

**A7 — "§3.3's `N × records / wall_s` over-states throughput where processes
finish unevenly."** The document says so itself and that it cancels in the
selection (a monotone function of `wall_s` at fixed N and records). Rejected.

**A8 — "the second instrument in §4.1 is still in bijection with the first."**
Rejected: §4.1 now states the blindness (over-fold only) and where the under-fold
half is pinned; both re-derived.

**A9 — "HEAD moved, so the review is against a stale revision."** Rejected: the
one commit above `735fc37` touches the assembler's review and the ledger; the
reviewed file, the sibling and the code are byte-identical at HEAD.

---

# HEAD AT THE END OF THIS REVIEW

```
$ git -C /home/tom/Projects/HeXO-AlphaBeta rev-parse HEAD
73979e7675696022b81e429f87b5437f8ac47db3
$ git diff --quiet 735fc37 HEAD -- docs/experiments/wp21_throughput_prereg.md docs/experiments/wp21_prereg.md crates tools configs; echo $?
0
```

**Does not equal the reviewed revision `735fc37`; the reviewed document and
everything it governs are byte-identical between the two.** The worktree
`/home/tom/pistol-wt/throughput-r4` is removed after this file is written; its
only products were the release build of `pistol-cli`/`pistol-arena`, the harness
runs and failure probes under `scratch/`, and the extracted §7 block — none an
artefact of record, no `artifacts/` or `sessions/` created.
