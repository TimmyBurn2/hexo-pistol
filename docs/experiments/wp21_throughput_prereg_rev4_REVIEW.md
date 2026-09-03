# REVIEW-design (fresh context) — `wp21_throughput_prereg.md` revision 4, ROUND 3 of 5 (D-585)

**NAMED REVISION.** `docs/experiments/wp21_throughput_prereg.md` as of commit
**`c4963b3908a1ff60da96a229b5ed364ce5a02f52`**, branch `dev`.

**DOES IT STILL MATCH HEAD?** **YES.** `git rev-parse HEAD` returned
`c4963b3908a1ff60da96a229b5ed364ce5a02f52` at the start and at the end of this
review (see the last section), and `git status` was clean.

**WHERE.** A detached worktree `/home/tom/pistol-wt/throughput-r3` at `c4963b3`,
`CARGO_TARGET_DIR=/home/tom/pistol-wt/throughput-r3/target` per command, scratch
under `scratch/` there; nothing in the live tree was written but this file. The
worktree is removed at the end. Nothing of lever A's registered settings was run:
the largest run here is two concurrent captures of the dry-run stand-in at
`--label-nodes 2000`, a fraction of a second. The other reviewer's worktree
(`/home/tom/pistol-wt/prereg-r3`) was not touched and no `pkill` was issued.

**WHAT I READ.** CLAUDE.md (Process, hard rules 1, 4, 5, 6); `docs/process.md` in
full; the document; `wp21_throughput_prereg_rev3_REVIEW.md` in full;
`arc3_ledger.md` §1c and §1f to the end; `docs/decisions.md` D-576, D-581, D-584,
D-586..D-595 in full; `wp21_prereg.md` revision 5 §1, §3, §4, §5, §6.1, §8, §9;
`wp21_label_cache_design.md` revision 10 §2, §3, §7, §8; the headers and verdicts
of `wp21_label_cache_impl_REVIEW.md`, `_round2.md`, `wp21_label_cache_REDTEAM.md`;
`matrix_label_cache_key.md` §2.2–§4; `tools/determinism.sh` in full;
`crates/pistol-arena/src/{capture.rs,label_cache.rs,usage.rs,passes.rs,capture_file.rs,replay.rs,bin/arena.rs}`,
`labels.rs`/`labels_file.rs` (column indices), `crates/pistol-engine/src/instance.rs`,
`crates/pistol-search/src/{search.rs,heuristics.rs}`; `tools/label_cache_count.py`,
`tools/wp21_tranche_config.py`, `tools/governing_citation_check.sh`;
`artifacts/arc3r_dryrun_throughput_0c4f3b4.txt`, `artifacts/arc3_leverB_41_count_v3.txt`,
`artifacts/wp20pilot_RUN_2cd4f79_v1.txt`, `artifacts/arc3_opening_prefix_fold.txt`;
`/home/tom/pistol-runs/arc3r-dryrun/throughput/` and `throughput_dryrun.sh`
(read-only); `/home/tom/pistol-runs/wp20pilot-artifacts/{capture_v1,corpus_v1,report_v1}.txt`.

---

## VERDICT: **FAIL — 1 BLOCKING, 3 MAJOR, 14 minor**

Revision 4 closes twenty of round 2's twenty-two findings, and every one I mark
CLOSED below I verified against the tree or the log and not against the sentence
that claims it. The soundness argument for the cache (§2) is now derived where it
is true: every claim I re-derived about `newgame`, gate 9's five seats, its two
budgets, its `budgets x positions` session, the countermove path and the counters
reproduces from the script and the code under commands I chose. The digests all
re-take, the binaries I built at `0c4f3b4` in my own worktree digest identically to
§8's, and all forty-five quoted lines of §7.1 are in the log verbatim.

It fails on one thing, and the thing is the study's own statistic. **Lever A's
registered number is a wall clock, and no registered instrument produces it.** §7
prints one process's `arena --capture` line; nothing registered starts N processes
together, nothing registered reads a clock, `arena` prints no wall, and the
dry-run log carries not one elapsed second. The concurrent launch was never
dry-run at all — limb 1 was taken at N = 1. `docs/process.md` names "a scratchpad
harness" as an artefact that must be named with its revision, and the harness that
would produce the number this document exists to register is not named, printed,
digested or exercised. §4.5's 5% clause needs two walls the same way and has the
same hole. I wrote and ran the missing harness (BLOCKING 1) so the fix is a
paste, not a claim.

The three MAJORs are claims the tree contradicts: the design is cited at revision
7 and stands at 10; §4.4 and §5 describe a lever-B pair "over the pilot's report
(742 records, ~11 min uncached and ~5 min cached)" that the closure binary REFUSES
at exit 2 — the runnable pair is §7's, over the 152-record play-pass report, at a
fifth of that cost; and C3/C4 are read at a setting with three reps and no
registered aggregation over them.

---

# DISPOSITION OF ROUND 2 (revision 3 → revision 4)

| # | round-2 finding | status | evidence at `c4963b3` |
|---|---|---|---|
| **B1** | §1/§5 computed on superseded arithmetic; wall restated three ways | **CLOSED** | §1 reads `13 927 s`, `11 017 s — 79.1%`, play `1 443 s, 10.4%`, replay `1 409 s, 10.1%`, cold `58 s, 0.4%`; `wp21_prereg.md` revision 5 §3 (lines 126–131) prints the same five lines. `11017/13927 = 0.7910`, `1443/13927 = 0.1036`, `1409/13927 = 0.1012`, `58/13927 = 0.0042` (my arithmetic). §5 reads `7.54 h against 7.74 h — ~0.20 h net`; sibling §3: `27 141 s = 7.54 h` vs `27 854 s = 7.74 h`, difference `713 s = 0.198 h`. Replay is now excluded by criterion (*"T-B mandates `--workers 1`"*), which sibling §4 T-B says. Residue: **minor 3** (7.74 h is still stated here, twice, under a sentence saying it is not) |
| **B2** | a-priori `k <= opening_turns` false; flip clause pre-answered | **CLOSED** | §2: *"over the sweep's own range the book's prefixes fold at `k = 2` — 42 to 60 merges per tranche, 792 in all … at `k = 3` the book's whole-opening dedupe holds (D-584)"*. My `awk` over `artifacts/arc3_opening_prefix_fold.txt`'s addendum: sum **792**, min **42**, max **60**. Flip clause now D-586's per-tranche floor + one percent + the `countermove` conjunct; §2 states it that way |
| **B3** | dry run claimed, not recorded; limb 1 false of `--label-cache` | **CLOSED** | §7.1 RECORD carries log path, sha256 (`1e929644…` — re-taken, matches), head line, config sha, every command with its exit line, every printed line the limbs read. Mechanical check: **30 of 30** command/exit lines and **15 of 15** printed lines found verbatim in the log (with `<SCRATCH>` substituted). `--label-cache` is `usage.rs:115`; the dry run exercised it at exit 0. Residue: **minor 8** (the two `sort -u` lines are echoed, not executed, as printed) |
| **M1** | four seats; array has five | **CLOSED** | §2 names five: `radius`, `staged`, `staged-heuristics`, `staged-solver`, `staged-safety-net-cap` — my `awk` over the `SEATS=(…)` block returns those five names; solver overrides `depth_turns-2 nodes-10000` named; `nodes 400000` absent from the script |
| **M2** | "strongest attack" superseded | **CLOSED** | §2 carries D-581/D-588's attack (*"the identical ask, which is a hit, is the repeat it never takes"*), the channel hunt demoted to supporting evidence, D-581 and D-588 cited by number |
| **M3** | second instrument in bijection; wrong stage; asked-prefix limb unevaluable; registered after; never run | **CLOSED** | §4.1 names the stage (the 47-line derivation, `label_cache_count.py:101-147` — my `sed -n '101p;147p'` shows `def stones_of` and `images`'s `return out`), second instrument = corpus `key_pos`/`key_full` from `pistol-core`, agreement on both columns, consequence registered, asked-prefix limb dropped, *"a REPLICATION, not a pre-registered second instrument"* conceded, taken **347 and 347**. Residue: **minor 5** |
| **M4** | sha anchors false | **CLOSED** | §8 holds `cbad0786…` (receipt) and `1a890b53…` (script) in the committed document; both re-taken by `sha256sum`, match |
| **M5** | §5 withdraws an appeal and re-makes it | **CLOSED** | §5 is one paragraph; *"There is no later sweep to point at"* stated once |
| **M6** | play-pass instrument uncommitted, unregistered, wrong binary pin | **CLOSED** | §3.1/§7 use the shipped `tools/wp21_tranche_config.py --skip 0 --take 3 --pilot-range --binary-sha256 <digest>`; §8 digests it (`707acacc…`, re-taken, matches) and names the closure binaries; C4's referent binary (`180b4c40…`, 1.97.1) stated against the closure binary (`78a7600a…`, 1.98.0) |
| **M7** | C3 at 12 s cannot fire | **CLOSED** | C3 is `3.54 s`, `c(N) <= 4`, with a reason tied to §3.4's SMT note; `4 x 0.885445 = 3.5418` (mine). A factor of four is crossable only by pathology, which is the defect class named |
| **M8** | D-581 too strong about gate 9 | **CLOSED** | D-588 appended; §2 says the A/B session repeats every position once per budget after `newgame` and never the identical `(position, go)`. Re-derived: the session builder is `for budget … for position … printf 'newgame\nposition %s\ngo %s\n'` (`tools/determinism.sh:222-225`), consumed by ONE process per run A/B; `BUDGETS` holds two entries → every position twice per process, at two `go` lines |
| **minor 1** | §4.1's block a trimmed transcript | **OPEN** | §4.1 still prints eight unprefixed lines; the receipt has ten `label_cache_count:` lines plus the `capture …, body sha256 …` line. §7.1 pastes the full form, §4.1 does not — **minor 4** |
| **minor 2** | D-576 cites the `_v1` receipt | **OPEN, outside this document** | `/usr/bin/grep -n 'arc3_leverB_41_count' docs/decisions.md` → only `:1220` (D-576, `_v1`). No later line amends it. Not a finding against this document |
| **minor 3** | ONE LINE vs §5 on the trade | **CLOSED** | one figure, `~0.20 h`, ONE LINE pointing at §5 |
| **minor 4** | `~1.0 h` is the top of its bracket | **PARTIAL** | now *"~1.0 h at contention 1.0–2.0"*; the bracket is `0.56–1.12 h` (mine: `15 x 152 x 0.885445 = 2 019 s`) — **minor 10** |
| **minor 5** | §7 lacks limb 2's command | **CLOSED** | §7.1's record prints both orders with their refusals |
| **minor 6** | dry-run input inside the registered range | **OPEN** | still `--skip 0 --take 1` ⊂ `0..2`. The generator ADMITS `--skip 3 --take 1 --pilot-range` (executed: exit 0, sha `9644fc9f…`) — **minor 9** |
| **minor 7** | §8 "revision" is prose | **CLOSED** | digests |
| **minor 8** | D-581 uncited | **CLOSED** | cited five times |

**Tally: 17 CLOSED, 1 PARTIAL, 3 OPEN (one outside this document).**

---

# RE-DERIVATION TABLE

Every count below was taken with a command I chose, in my worktree at `c4963b3`
unless the scope says otherwise. "Doc" is what revision 4 states.

| claim | my command (scope) | mine | doc | match |
|---|---|---|---|---|
| tree diff between the binaries' commit and this one | `git diff --stat 0c4f3b4 c4963b3 -- crates tools configs` | empty (three `docs/` files only in the full diff) | §8: build at `0c4f3b4` is this revision's | ✓ |
| `pistol`, `arena`, `corpus-check` digests | `cargo build --workspace --release --locked` in `/home/tom/pistol-wt/throughput-r3`, own `CARGO_TARGET_DIR`, `rustc 1.98.0 (88d9e12ae 2026-08-18)`; `sha256sum target/release/{pistol,arena,corpus-check}` | `78a7600a…`, `a1a405cb…`, `efbb76b6…` | same three | ✓ (build reproducible across paths) |
| `tools/label_cache_count.py`, `tools/wp21_tranche_config.py` digests | `sha256sum` (live tree, same bytes at `c4963b3`) | `1a890b53…`, `707acacc…` | same | ✓ |
| receipt and dry-run log digests | `sha256sum artifacts/arc3_leverB_41_count_v3.txt artifacts/arc3r_dryrun_throughput_0c4f3b4.txt` | `cbad0786…`, `1e929644…` | same | ✓ |
| gate 9 seat count and names | `awk '/^SEATS=\(/,/^\)/' tools/determinism.sh \| grep -c '^\s*"'`; names by `sed` | **5**: radius, staged, staged-heuristics, staged-solver, staged-safety-net-cap | five, those names | ✓ |
| gate 9 budgets | `sed -n 's/^BUDGETS=…'` + `LAYOUT_BUDGET`/`SOLVER_LAYOUT_BUDGET` + override words in the array | `depth_turns 4`, `nodes 200000`; layout `nodes 200000`; solver `depth_turns-2 nodes-10000`, layout `nodes 10000` | same | ✓ |
| none of the five configs is the sweep's seat | `cmp -s configs/<seat>.toml configs/instrument_v0.toml` x5 | all five differ | none is `instrument_v0.toml` | ✓ |
| A/B session shape | `grep -n 'for budget in\|for position in'` → `:222`, `:223`, nested; `grep -c "printf 'newgame"` → 2 (session and layout builders); one `$ENGINE … <"$script"` per run | `budgets x positions`, `newgame` before each, one process | same | ✓ |
| gate 9 is 9 of 20 | `grep -n 'gate 9/' tools/ci.sh` → `:109`; `GATE_TOTAL=20` at `:21` | 9/20 | (not restated in rev 4; D-584) | ✓ |
| `newgame` clears TT, heuristics, solver | read `instance.rs:85-88` (`new_game` → `searcher.clear()`), `search.rs:263-273` (`table.clear()`, `heuristics.clear()`, `solver.reset()`); collector: `instance.rs:112-119` comment | as stated | §2 | ✓ |
| the one play-order read path | `heuristics.rs:153-159`: `if gates.countermove && … last_stone(state)`; `configs/instrument_v0.toml:79-81` all three `false` | one gate, `countermove` | §2 | ✓ |
| `ask` sends `newgame` first | `capture.rs`: `for line in [NEW_GAME, position, go]` | yes | §2 | ✓ |
| `go` computed once outside both loops; memo built inside `run` | `capture.rs`: `let go = label_go_line(…)` before `with_seats`; `Memo::new(cache)` inside the closure | yes | §2.0 | ✓ |
| `asks` counted at the call; counters one sort + `canonical_form` per miss, Off pays nothing | `label_cache.rs`: `memo.asked()` before `ask`; `insert` returns early under `Off`, else `sort_unstable` + `canonical_form` | yes | §4.2, §2 | ✓ |
| `Symmetry::ALL` is twelve | `grep 'pub const ALL' crates/pistol-core/src/symmetry.rs` → `[Symmetry; 12]` | 12 | "twelve images" | ✓ |
| capture arm and tail parser | `bin/arena.rs:52-59` is the `--capture … tail @ ..` pattern; `usage.rs:111` is `pub fn capture_tail`; `:116` refuses both orders | yes | §3.2, §4.2 | ✓ |
| capture file carries no path and no timestamp | `cmp` of the dry run's `cap-N1-rep1-p1.txt`, `uncached.txt`, `cached.txt` (different `--out`, seconds apart) → all identical; `capture_file::render` writes params, derived, records only | C1 is satisfiable | §3.5 C1 | ✓ |
| §4.1 count block | Python over `capture_v1.txt` (sha `4563f050…`): rows, distinct `position`, `Counter` of multiplicities | 742 / 347 / 2.1383 / 0.5323 / {2: 345, 26: 2} | same | ✓ |
| 47-line derivation | `sed -n '101p;147p'` | `def stones_of` … `return out` | `:101-147` | ✓ |
| lever A's per-process records | Python over `capture_v1.txt`, `game < 6` | **152**; distinct 72; hit rate 0.5263 | 152 | ✓ (72/0.5263 is new — MAJOR 2) |
| 0..2 workload transfers across the toolchain | `diff` of the `moves` lines of `report_v1.txt` games 0–1 (binary `180b4c40…`) and the dry run's `report.txt` (binary `78a7600a…`), opening 0 | identical | (assumed) | ✓ for opening 0 — **minor 14** |
| corpus columns for the second instrument | `labels_file.rs:277-278`: `key_pos: f[4]`, `key_full: f[5]` → `cut -f5`/`-f6` | correct columns | §4.1, §7 | ✓ |
| dry-run second-instrument counts | Python `set()` over columns 5 and 6 of `/home/tom/pistol-runs/arc3r-dryrun/throughput/corpus.txt` (34 records, 16 fields) | 17 and 17 | 17 and 17 | ✓ |
| C3 threshold, C4 referent | `4 x 0.885445`; `657/742` | 3.5418; 0.885445 | 3.54 s; 0.885445 | ✓ |
| §5 lever-A cost | `5 x 3 x 152 x 0.885445` | 2 019 s = **0.56 h** at c = 1, **1.12 h** at c = 2 | ~1.0 h at 1.0–2.0 | ≈ (minor 10) |
| §5 net saving, gate-free saving | sibling §3: `27 854 − 27 141`, `27 854 − 21 989` | 713 s = 0.20 h; 5 865 s = 1.63 h | 0.20 h; 1.63 h | ✓ |
| ~6 600 hits a tranche | sibling §3 `hits 6 624` = `12 443 x 0.5323` | 6 623 | ~6 600 | ✓ |
| fold floors | `awk` over the addendum of `arc3_opening_prefix_fold.txt` | sum 792, min 42, max 60 | 42 to 60, 792 | ✓ |
| §7.1's record vs the log | Python: every line of the two fenced blocks, `<SCRATCH>` substituted, searched in the log | 30/30 and 15/15 present | "verbatim" | ✓ (minor 8 on how one pair was produced) |
| §7.1's `ls` exit and refusals | log: `exit=2` after four `cannot access` lines; both refusal lines identical | as quoted | as quoted | ✓ |
| box shape | `lscpu` | 8 cores, 16 threads (3700X) | 8 physical, 16 threads | ✓ |
| ADR references exist | `grep -c '^D-<n>:'` for 137, 291, 423, 424, 539, 558, 560, 562, 576, 581, 584, 586, 587, 588 | 1 each | cited | ✓ |
| citation gate on this document | `bash tools/governing_citation_check.sh` in my worktree | `wp21_throughput_prereg.md: 12 citation(s) checked, 0 unreproduced` | (ledger: 12, 0) | ✓ |
| **the pilot's report is capturable by the closure binary** | `./target/release/arena --capture /home/tom/pistol-runs/wp20pilot-artifacts/report_v1.txt --out scratch/refused.txt --label-nodes 2000` | **REFUSED, exit 2**: `EngineBinaryDigestMismatch: … hashes to 78a7600a… and this document binds it to binary_sha256 180b4c40…` | §4.4: "over the pilot's report (742 records …)" | **✗ — MAJOR 2** |
| **the design's revision** | `head -1 docs/experiments/wp21_label_cache_design.md`; `git log` on it | **revision 10**, at `4fab9ed` (08:13), before `c4963b3` (08:47) | "revision 7" (§GOVERNING, §4.2) | **✗ — MAJOR 1** |
| **an instrument that prints lever A's wall** | `grep -c 'seconds\|wall' artifacts/arc3r_dryrun_throughput_0c4f3b4.txt` → 0 of either outside the play pass's own `wall 2340 ms`; `passes.rs::capture` prints no elapsed time; §7 prints one process's command | **none** | §3.3 "wall from first start to last exit" | **✗ — BLOCKING 1** |

---

# BLOCKING

## BLOCKING 1 — **Lever A's statistic is a wall clock no registered instrument produces; the concurrent launch was never dry-run; §4.5's two walls have the same hole.**

§3.3: *"Realised seconds per label at concurrency N = (wall from first start to
last exit) / `records`"*. §3.2: *"N processes each run `arena --capture …`
concurrently, started together; the setting ends when the last exits."* §4.5:
*"the cached capture's measured seconds per MISS — its wall over its `asks` — is
within 5% of the uncached capture's seconds per record"*.

Scope: everything the document registers as a command, everything the binary
prints, everything the dry run recorded.

- §7's lever-A block is one line — `arena --capture <SCRATCH>/report.txt --out
  <SCRATCH>/cap-N<n>-rep<r>-p<i>.txt --label-nodes 400000` — *"one of the N
  concurrent processes"*. Nothing registered starts N of them together, waits for
  the last, or reads a clock before and after.
- `passes.rs::capture` prints four lines — `captured …`, the counts line, the
  manifest row, `capture written to …` — and no elapsed time. `arena --capture`
  has no `seconds=` output.
- `artifacts/arc3r_dryrun_throughput_0c4f3b4.txt` contains no elapsed-seconds
  line for any capture. The only wall in it is the PLAY pass's own `wall 2340 ms`.
- The dry run's lever-A limb is titled *"N = 1, rep 1"*: one process. No setting
  with N ≥ 2 was ever launched, so "started together" and "last exit" were never
  exercised.

`docs/process.md`, **Instrument governing revision**: *"An artefact that produces
a registered number — a `tools/` script, **a scratchpad harness**, or a command
block the document prints — is named in the pre-registration WITH ITS REVISION …
Without this, a run stands on an instrument whose own review had failed and is
licensed by argument rather than by this text."* The wall IS the registered number
— it is the one thing this document exists to measure, and §3.4's whole decision
rule is a function of it. **Dry-run discipline**: *"A pre-registration's literal
commands are exercised before its review passes"* — the launch-N-and-time-them
command has no literal form to exercise, and its N = 1 stand-in exercised neither
concurrency nor timing. The same applies to §4.5: two walls, no instrument, no
line in the log where either would be read.

This is the severity scale's *"an instrument without its digest"* and *"a
registered command that does not run"* at once, on the statistic itself. It is
not a wording defect: the day the study runs, somebody writes a loop, and the
loop's shape (does it `wait` for the last or for all; does it time from before the
first `fork` or after; does it count records from the file or from the counts
line) is decided after the numbers start coming out.

**FIX, EXECUTED.** I wrote the harness and ran it on the dry-run stand-in in my
worktree at `--label-nodes 2000`, N = 1 and N = 2 (well under the box's noise
floor for the other reviewer):

```
scratch/leverA_harness.sh   sha256 ea1172a2860d7f57cb28babe0e32fa4c888f628f7e8e5ff66500296e1d4098ce
  REPORT=$1; N=$2; REP=$3; NODES=$4; OUT=$5
  t0=$(date +%s.%N)
  for i in $(seq 1 "$N"); do
    ./target/release/arena --capture "$REPORT" --out "$OUT/cap-N$N-rep$REP-p$i.txt" \
        --label-nodes "$NODES" > "$OUT/cap-N$N-rep$REP-p$i.log" 2>&1 &
    pids+=($!)
  done
  rc=0; for p in "${pids[@]}"; do wait "$p" || rc=1; done
  t1=$(date +%s.%N)
  records=$(/usr/bin/grep -v '^#' "$OUT/cap-N$N-rep$REP-p1.txt" | /usr/bin/grep -c .)
  … prints: leverA: N <n> rep <r> rc <rc> wall_s <w> records <r> s_per_label <w/r> throughput <n*r/w>

$ ./scratch/leverA_harness.sh /home/tom/pistol-runs/arc3r-dryrun/throughput/report.txt 1 1 2000 scratch/leverA
leverA: N 1 rep 1 rc 0 wall_s 0.511 records 34 s_per_label 0.015029 throughput 66.536
$ ./scratch/leverA_harness.sh /home/tom/pistol-runs/arc3r-dryrun/throughput/report.txt 2 1 2000 scratch/leverA
leverA: N 2 rep 1 rc 0 wall_s 0.737 records 34 s_per_label 0.021676 throughput 92.266
$ cmp cap-N2-rep1-p1.txt cap-N2-rep1-p2.txt && cmp cap-N1-rep1-p1.txt cap-N2-rep1-p2.txt \
     && cmp cap-N1-rep1-p1.txt /home/tom/pistol-runs/arc3r-dryrun/throughput/uncached.txt
C1 shape: all four byte-identical
```

(The N = 2 numbers are the stand-in's at a toy budget on a box with another
reviewer's work on it — a shape, not a measurement.) What revision 5 must do:
print the harness in §7 (or land it under `tools/` with a test driving it, per
the tools/ coverage rule) and digest it in §8; state in §3.3 that `records` is
read off the capture file and the wall off the harness's own two clock reads;
give §4.5 its instrument (the same harness at N = 1 with and without
`--label-cache`, or the run log's per-command seconds as `wp21_prereg.md` §5
already registers for the sweep); and re-take the dry run with one setting at
N ≥ 2 so the `leverA:` line is in the record.

---

# MAJOR

## MAJOR 1 — **The design is cited at revision 7; the tree holds revision 10, and it did before this revision was committed.**

§GOVERNING: *"`wp21_label_cache_design.md` revision 7 (what lever B builds)"*;
§4.2: *"`wp21_label_cache_design.md` revision 7, in one paragraph"*.

```
$ head -1 docs/experiments/wp21_label_cache_design.md
# WP-2.1 lever B — the label cache. DESIGN, revision 10.
$ git log --format='%h %ad %s' --date=iso -- docs/experiments/wp21_label_cache_design.md | head -3
4fab9ed 2026-09-03 08:13:23 +0200 … design revision 10, D-595
5b17132 2026-09-03 07:47:18 +0200 … design revision 9 names the arm's site …
610225b 2026-09-03 07:35:00 +0200 … design revision 8 says so
$ git log -1 --format='%h %ad' c4963b3
c4963b3 2026-09-03 08:47:54 +0200
```

Revisions 8, 9 and 10 all landed before `c4963b3`. The ledger's §3 says the
document *"names the design's revision 7"* at landing (`eb3dc6c`, 07:28) — true
then, stale by 07:35, and not updated when the slots were filled at 08:47.
`docs/process.md`: the instrument is named *"WITH ITS REVISION, and a change to it
reopens the review"*. A registration naming a superseded revision of the design
of the thing it verifies is the ROT class gate 20 exists for, in the one form
(a revision number, not a path) the gate cannot see.

What changed in 8–10 does not move the mechanism — T3's first-line assertion,
row 4's site, T4's five-run form and the measured residual (D-595) — so this is a
citation defect, not a soundness one. **Fix (unexecuted, one number twice):**
cite revision 10 and, since §4.2's paragraph is *"in one paragraph"* of the design,
say that 8–10 changed test rows and §3's residual only (D-589, D-595).

## MAJOR 2 — **§4.4 and §5 describe a lever-B pair "over the pilot's report (742 records, ~11 min uncached and ~5 min cached)" that the closure binary refuses; the pair §7 registers is over the 152-record play-pass report, at a fifth of that cost.**

§4.4: *"Taken twice: over the pilot's report (742 records, ~11 min uncached and
~5 min cached) and over tranche one's own report"*. §5: *"lever B §4.4's pilot
pair | ~11 min + ~5 min"*. §7: *"# lever B §4.4 — the pilot pair"* over
`<SCRATCH>/report.txt` — the §3.1 play-pass report, openings `0..2`.

The pilot's report binds its engines to `180b4c40…` and `arena --capture` verifies
that identity before any game (`replay::verify_engines`, called second in
`capture::run`). Reproducer, my worktree, closure binary:

```
$ ./target/release/arena --capture /home/tom/pistol-runs/wp20pilot-artifacts/report_v1.txt \
      --out scratch/refused.txt --label-nodes 2000
arena: EngineBinaryDigestMismatch: engine a: `target/release/pistol` hashes to
78a7600adcf099de0b04149535f1f4bffe0b6c945609a3206d73a4e5ee853749 and this document binds it to
binary_sha256 180b4c406b225fc81342bb8218b8546dda1ffac1a99f7eb91cdaf73d20253476; the file at that
path is not the build this run is written for, so no game was played
exit=2
```

So "the pilot's report" cannot be the referent pair's input under §8's binary,
and 742 / 11 min / 5 min describe a run that does not exist. The runnable pair is
§7's, whose report is 3 openings, 6 games. Measured on the pilot's capture
(games 0–5, which the dry run shows reproduce move-for-move across the toolchain
at opening 0): **152 records, 72 distinct `position` lines, hit rate 0.5263** —
so ~135 s uncached and ~64 s cached (`72 x 0.885445`), about 2.3 min + 1.1 min.
§5's row is high by ~5x, and the ONE LINE's *"the lever is net negative … counting
lever A's hour and the pilot pair"* rests on the wrong number (the conclusion
survives: 0.56–1.12 h + 0.06 h still exceeds 0.20 h).

Also: §4.4 calls the tranche-sized referent "tranche one's own report" and §5 costs
the cached re-capture at 1.43 h — both consistent with the sibling; only the
first pair is wrong.

**Fix (unexecuted, prose):** §4.4 and §5 say *"over the §3.1 play-pass report
(152 records, 72 distinct, ~2.3 min uncached and ~1.1 min cached)"*; drop
"pilot's report" everywhere it means the 13-opening one, since that report is
not capturable at this revision's binary.

## MAJOR 3 — **C3 and C4 are read at a setting with three reps, and the aggregation over reps is not registered.**

§3.5 C3: *"the realised MEAN seconds-per-label at the selected N is at most
3.54 s"*. C4: *"the N = 1 realised seconds-per-label is within 20% of … 0.885445"*.
§3.3 defines "realised seconds per label" per setting-rep (one wall over one
record count) and says the DECISION statistic is the *median throughput over the
three reps*; it never says which of the three per-rep values C3 and C4 read.
"MEAN" in C3 is the mean over labels (wall / records — round 2 M7's fix), not over
reps.

At N = 1 three deterministic captures of one report will land within a percent or
two of each other, so the choice will rarely matter — but "rarely" is the
after-the-numbers choice `docs/process.md` forbids in as many words: if rep 1
reads 1.10 s (24% over) and reps 2–3 read 0.95 s, whether the whole study is VOID
under C4 depends on a rule nobody wrote down. Under the dispatch's scale this is
*"a decision rule that leaves an after-the-numbers choice"*; I rate it MAJOR
rather than BLOCKING because §3.3's median is the only aggregation the document
names and a reader would infer it — but inference is what the rule says not to
rely on.

**Fix (unexecuted, one clause):** *"C3 and C4 are read at the median rep — the
one whose throughput §3.3 reports — and at no other."*

---

# minor

**minor 1 (D-424) — §3.4's SMT note cannot fire when its trigger fires.** *"If
the highest-throughput N is 16, its wall figure carries the note … a per-tranche
time at 16 more than 2x the time at 8 means the second wave was cheaper than the
sharing."* With equal records per process, `throughput(16) = 16 R / t16` and
`throughput(8) = 8 R / t8`; `t16 > 2 t8` implies `throughput(16) < throughput(8)`,
so 16 could not be the highest-throughput N. The note's condition and its trigger
are mutually exclusive. It constrains nothing; delete it. (C3's *"four because
§3.4's own SMT note …"* then needs its reason restated on its own: at `c = 4`,
N = 16 matches N = 4 at `c = 1`, which the row already says.)

**minor 2 (D-424) — §3.2's rule and §3.4's prohibition are both vacuous given the
field.** §3.2: *"if the sweep's concurrency changes, the tranche count changes
with it, so that `TR mod N == 0`"*; §3.4: *"It may not change the tranche count"*.
Every member of {1, 2, 4, 8, 16} divides 16, so neither sentence can ever act.
Keep one — the prohibition — and delete the rule.

**minor 3 (D-423) — 7.74 h is stated twice here under a sentence saying it is
not.** ONE LINE: *"`wp21_prereg.md` §3 owns the sweep's wall — 7.74 h … — and this
document does not restate it"*; §5: *"7.54 h against 7.74 h"*. `grep -c '7.74'` →
2. Likewise 742 five times, 347 nine times, 0.885445 four times. The referent
figures may be quoted; the wall the document promises not to restate should be
pointed at.

**minor 4 — §4.1's block is still a trimmed transcript** (round 2 minor 1,
OPEN): eight lines without the `label_cache_count:` prefix, without the
`capture …, body sha256 …` line, without `cache hits 395` / `cache misses 347`.
§7.1 pastes the ten-line form; §4.1 should too.

**minor 5 — §4.1's agreement criterion is blind to an under-folding derivation
on every population it has been taken on.** On the pilot corpus the three keys
all give 347 and on the stand-in 17/17/17: the exact count equals the folded
counts, so a `stones_of`/`images` that folded NOTHING would return the same 347
and agree with the corpus's 347. The criterion can be falsified by a derivation
that over-folds, not by one that under-folds — half of the defect class. The stage
under doubt is in fact exercised by
`crates/pistol-arena/tests/label_cache_count_tests.rs::a_mirrored_position_folds_only_under_the_symmetry_column`
(a fixture whose two coarser columns differ by one) and by
`tools/opening_prefix_fold.py`'s 792 at `k = 2`; §4.1 should say that is what
pins the derivation, and that the corpus agreement certifies no over-fold only.

**minor 6 — `wp20b_perf_guard.sh` is cited with no path and is not in the tree.**
§3.2: *"(`wp20b_perf_guard.sh`'s rotation lesson)"*. `ls tools/ | grep guard` →
nothing; `git grep wp20b_perf_guard` → only `docs/experiments/wp20b_artifacts.md`
(it lives under gitignored `artifacts/`, sha-indexed there) and
`docs/audit/repo_audit_2026-09.md`. Cite the indexed record
(`wp20b_artifacts.md:94,105`, which names the arm-order rotation) so the citation
is checkable.

**minor 7 — §3.1's cross-reference is wrong.** *"§3.4's 120 s guard reads
`hang_timeout_ms` off that config"* — §3.4 has no 120 s guard; it is §3.5 C3's
row (*"the 120 s `hang_timeout_ms` watchdog stays the backstop"*).

**minor 8 — §7.1's two `sort -u` lines are echoed, not executed, as printed.**
`throughput_dryrun.sh` runs `/usr/bin/grep -v '^#' $S/corpus.txt | …` and echoes
`… corpus.txt | …`; the log and the document print the echo. A reader
re-running the printed line from the repository root gets `grep: corpus.txt: No
such file` and `0`. The counts are right (re-derived above); the record's
*"verbatim"* is one word short of true for these two lines. Print the path as
`<SCRATCH>/corpus.txt` like every other command.

**minor 9 — the dry-run input is still inside the registered range** (round 2
minor 6, OPEN): `--skip 0 --take 1` ⊂ `0..2`. `docs/process.md`: *"never on the
registered workload itself"*. The report is a different artefact, so this is
arguable — but the generator admits the disjoint form today: executed,
`tools/wp21_tranche_config.py --skip 3 --take 1 --pilot-range --out
scratch/arena_dry3.toml --binary-sha256 78a7600a…` → exit 0, sha `9644fc9f…`.
When the dry run is re-taken for BLOCKING 1, take it at `--skip 3`.

**minor 10 — §5's lever-A figure is the top of a bracket it does not print, and
C4's 20% has no stated reason.** `5 x 3 x 152 x 0.885445 = 2 019 s = 0.56 h` at
`c = 1`, `1.12 h` at `c = 2`; *"~1.0 h at contention 1.0–2.0"* should read
*"0.56–1.12 h"*. C4's row says what the 20% tolerates (a toolchain and a workload
difference) and not why 20 rather than 10 or 30; one sentence (e.g. round 2 A7's
measured `search_nodes` ratio of 1.0042 between the two workloads, leaving the
compiler as the only unmeasured term) would make it a reason.

**minor 11 — §4.2 describes the counts line as one shape; the Off line has two
fields.** *"one counts line — the mode, `asks`, `records`, `hits`, the two
collision counters and `fold_ms`"* is the On line; `label_cache.rs::line` prints
`arena: label cache off: asks A records R` for Off. §7.1 shows both; §4.2 should
say "two shapes".

**minor 12 — §6.3 "produce a corpus" against the second instrument and the dry
run.** §4.1's second instrument reads a corpus, §7 does not print the `arena
--labels` command that makes one, §7.1's dry run ran it and its `corpus.txt` is
still on disk under `<SCRATCH>` beside every other dry-run file (§6.3: *"scratch on
`/home` and is deleted"*). Say "no corpus OF RECORD" in §6.3, and either print
`arena --labels` in §7's §4.1 block or say the second instrument was taken on the
pilot's existing `corpus_v1.txt` and is not re-run.

**minor 13 — §2's `~6 600` and §4.2's `~53%` are ESTIMATED and unmarked** (the
sibling marks `hits 6 624` ESTIMATED; `~53%` is the pilot's measured 0.5323 applied
to the sweep). Mark them.

**minor 14 — "152 records MEASURED per process" is measured on the pilot's report
under the pilot's binary; the study's report does not exist yet.** The play pass
is re-run under `78a7600a…`. I checked opening 0: its two games' `moves` lines in
the dry run's `report.txt` are identical to `report_v1.txt`'s games 0–1, so the
count almost certainly transfers, but the document should say the 152 is the
pilot's and that C2 reads the new report's own asked-prefix count (which it does
in substance). One sentence.

---

# WHAT SURVIVED ATTACK

**S1 — §2's soundness argument is derived where it is true, at every limb I
could check.** `newgame` → `clear` → TT, heuristics, solver (`instance.rs:85-88`,
`search.rs:263-273`); the census collector is the stated exception and is armed
only under a census `go`; `ask` sends `newgame` first; the one play-order path is
`countermove` at `heuristics.rs:153-159`; the five seats, the two budgets, the
solver's own two, the `budgets x positions` session in one process with `newgame`
before each ask, the C-vs-D layout — all read off the script by my own commands,
all as §2 states. The D-588 correction is imported exactly.

**S2 — The counters are described as the code has them.** One `sort_unstable`
and one `canonical_form` (twelve images) per miss, nothing under Off, `asks`
counted at the call, `hits = records − asks`, `key_full >= key_pos` by
construction. D-586/D-587's three corrections all appear in §2.

**S3 — §4.1's count block re-derives exactly under my own code**, including the
multiplicity structure and the two 26x lines; the 47-line stage is `:101-147`;
the corpus columns are the right ones (`f[4]`, `f[5]`); the second instrument's
17/17 on the stand-in re-derives by Python sets.

**S4 — Every digest in §8 re-takes**, and the two binaries plus `corpus-check`
built in a different directory under the same toolchain digest identically to
§8's — the `--locked` release build is reproducible across paths, which is more
than §8 needed.

**S5 — §7.1's record is faithful**: 45 of 45 quoted lines are in the log
verbatim; the `ls` exit 2 is the four absences; the refusal names both words in
both orders; the cached counts line reads `asks 17 < records 34`; the capture
files carry no path or timestamp, so C1 is satisfiable — confirmed by `cmp` across
my own N = 1 and N = 2 runs and the dry run's file.

**S6 — C1–C4 each name a defect that can falsify them**: C1 by any load-dependent
byte; C2 by a missing or short file (and `arena`'s all-or-nothing write means an
early exit leaves no file, so a short count cannot hide); C3 by `c > 4`, pathology
but the named pathology; C4 by an external referent with a stated provenance. §3.6
registers a consequence for each, and C1's escalation to the sweep is the right
one.

**S7 — §4.4 is an external referent with a mechanism, a named checker and a
registered consequence**, and "a single differing byte voids lever B" is registered
with its reason (no repair into passing). §4.5's clause, once it has an instrument
(BLOCKING 1), is falsifiable by the bookkeeping it names.

**S8 — Nothing in §7 contradicts §1's "may not touch" or §6.** The generator
writes `instrument_v0.toml`, `nodes 50000`, cap 40, `hang_timeout_ms 120000`;
label budget 400000 on every registered capture; no seat touched; no internal
parallelism.

---

# ATTACKS I ATTEMPTED AND REJECTED

**A1 — "the capture file carries its own path or a timestamp, so C1 can never
pass."** Rejected: `capture_file::render` writes params, derived digests and
records only; the dry run's three files at three `--out` paths, seconds apart, are
byte-identical, and so are my N = 2 pair.

**A2 — "a gate-9 seat config is `instrument_v0.toml` under another name."**
Rejected: all five `cmp` differ.

**A3 — "the corpus `key_pos` is a zobrist over play order, so it is not the
stone-set fold the script counts."** Rejected: `GameState::key` is over the placed
stones plus side-to-move and phase, both of which are functions of the stone count
for whole-turn prefixes; distinct `key_pos` = distinct sorted `(cell, player)`
lists up to a 128-bit collision.

**A4 — "the toolchain change moved the games, so 152 and 0.885445 are about a
different workload."** Rejected for opening 0 by a `diff` of the `moves` lines;
the residue is minor 14.

**A5 — "§4.5's 5% clause cannot fail."** Rejected: seconds-per-miss cached is
`t + (hits x h + fold) / asks`; with `hits ≈ asks` the clause fails at `h + fold/asks
> 44 ms`, which is exactly a cache whose bookkeeping ate its saving. It needs an
instrument (BLOCKING 1), not a different threshold.

**A6 — "C2 cannot fire because `arena` never writes a short file."** Rejected as
a finding: a process that dies leaves no file, and "no file" fails C2's equality
as surely as a short count would; the defect class named — *"exited early and
looked fast"* — is caught either way.

**A7 — "the `BTreeMap` claim 'one string compare' is false."** True (a lookup is
~log2(347) ≈ 8 compares) and not load-bearing; not recorded as a finding.

**A8 — "§3.2's field should hold 12."** Rejected: the divides-16 argument is
right, and the sibling's §1 slot says any N other than 8 reopens it, so 12 would
have to reopen the partition as well.

**A9 — "the second instrument was registered after the first ran, so §4.1 is
the same defect as round 2 M3(d)."** Rejected: §4.1 now SAYS so — *"a REPLICATION,
not a pre-registered second instrument … recorded as what it is"* — which is what
`docs/process.md` asks of an order that does not count.

**A10 — "the dry run is not the same KIND as the workload."** Rejected for the
capture (a generator-written report at the sweep's seat, captured by the same
binary); accepted for the concurrency limb, which is BLOCKING 1's second half, and
noted for the range overlap as minor 9.

---

# HEAD AT THE END OF THIS REVIEW

```
$ git -C /home/tom/Projects/HeXO-AlphaBeta rev-parse HEAD
c4963b3908a1ff60da96a229b5ed364ce5a02f52
```

**Still equals the reviewed revision.** The worktree
`/home/tom/pistol-wt/throughput-r3` is removed after this file is written; its
only products were the release build, the refusal reproducer, the harness and
its two toy runs, and `scratch/arena_dry3.toml`, none of them artefacts of record.
