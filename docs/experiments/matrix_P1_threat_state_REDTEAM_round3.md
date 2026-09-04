# MATRIX P1 — DECISION-RED-TEAM, round 3 (remedies-only, against the diff)

**Target.** `docs/experiments/matrix_P1_threat_state.md` (revision 3) at the
named revision `e13c3e4a61d90e39c20844ad86831c29b2e96621` — a `git stash create`
commit on `dev` = `ffc5c10f4d16356f574e3221a023f78399d2e3bb` (parents `ffc5c10`,
`c234fdc`). Revision 2 was `40b69e390b8fb01d5f2438a9eaddf01182a3d1f0`.

**Does that revision still match the working tree's bytes?** Yes — for all
seven untracked files the stash carries, `git rev-parse <stash>:<path>` equals
`git hash-object <path>` (matrix `ef536a54…`, design `fb7e2531…`, prereg
`078f9e1c…`, ledger `ab13be02…`, the two earlier red-team reports and the
dispatch likewise). **HEAD** at review time: `ffc5c10f4d16356f574e3221a023f78399d2e3bb`.
**Date:** 2026-09-04. **Model:** `claude-fable-5-1` (recorded per D-597).

**What this round is.** The third and last round the dispatch grants
(`opt_arc_DISPATCH.md:16`, *"up to three rounds per review gate, third
remedies-only"*), so a FAIL is STOP-and-split (`:141-142`). Per D-597 its
reading list was the diff
`git diff 40b69e3 e13c3e4 -- matrix_P1_threat_state.md p1_design.md p1_bench_prereg.md opt_arc_ledger.md`
(4 files, +138/−44), round 2's report, and the files the remedies touch. Per
D-591 every remedy naming a fixture, a command, a digest or a line number was
executed, not read. Nothing rounds 1 and 2 reproduced was re-reviewed; the
unchanged sections were not widened into.

**Verdict, stated first.** VERDICT: the recommendation SURVIVES but the matrix
FAILS (revise: R3-M1). Every remedy in the MATRIX's own lines holds on
execution — the three armed configs, the determinism seat, the census tests,
the solver-on run's digests, node identity and ratios, the per-band counter
arithmetic, the `size_of` receipt, the three corrected line citations — and no
number moves. What fails is the one remedy limb that names a command block:
the prereg's identity leg, which R2-M1 asked to be *"where that seat is
checked"*, feeds the keyed `tactical_staged_v0.txt` line by line at both seats
that read it, so the block as spelled sends 508 searches a side (not 128), 460
of them after a `position` line the engine refuses, and the registered
criterion *"128 `bestmove` lines and 0 `error` lines"* cannot be met by any
binary pair. The failing lines are `p1_bench_prereg.md` §2.1–2.2, inside the
diff; the fix is one extraction the tree already spells at
`tools/determinism.sh:204`. The recommendation is untouched.

**Environment.** Everything adjudicated was run with `git grep` pinned to
`ffc5c10` or `/usr/bin/grep`, sorted `LC_ALL=C`. No cargo build was needed:
the one engine run used the session's own digested `pistol-ffc5c10`
(`78a7600a…`, in `/home/tom/pistol-wt/p1-measure`) on four protocol lines, and
the block was executed against a stub in this session's scratchpad. No
worktree was created; nothing in the live tree was built or modified; the
only file written is this report.

---

## 1. Remedy table — each executed or re-derived with my own command

| finding | remedy in the diff | how I executed / re-derived it (scope stated) | holds? |
|---|---|---|---|
| **R2-M1** | §6 attack 1 rewritten: the three configs and the determinism seat named; round 2's solver-on run recorded (digests, 24 positions, two reps, node identity, 0 errors, 1.022 / 1.015 early, 1.005 / 1.001 late, ≈ 13 k vs ≈ 450 k nps); *"the package claims … byte-identity at three seats"*; §0.1 row; design §6 rewritten; prereg gains a "Seat" bullet and a THIRD identity seat | `git grep -n -E 'on_search_path\s*=\s*true' ffc5c10` (WHOLE tree, 42 hits incl. docs) → exactly three configs: `bench_wp18c_solver_on.toml:45`, `gate_staged_solver_v0.toml:47`, `play_staged_solver_v0.toml:75`. `git grep -n 'gate_staged_solver_v0' ffc5c10` (whole tree) → `tools/determinism.sh:76` (`depth_turns-2 nodes-10000`), `census_protocol_tests.rs:10`, `census_identity_tests.rs:14`, rest docs. `sha256sum`: `solveron_run.log` `61baa09e…` ✓, `pistol-ffc5c10` `78a7600a…` ✓, `pistol-E` `f333d101…` ✓, `configs/bench_wp18c_solver_on.toml` `8414509a…` ✓ live and via `git show ffc5c10:`. My python over the four `solveron.*.tot` receipts: node identity across 4 transcripts × 24 positions holds; rep1 early 30598/29943 ms = **1.022**, late 54577/54301 = **1.005**; rep2 **1.015** / **1.001**; pooled base 1 143 309 nodes / 85.2 s = **13.4 k nps**; `bench_E_v1.txt` base medians 446 k / 393 k. `session.nodes`: 24 `go nodes 50000`. `grep -c error` on the four transcripts → 0 each; 24 `bestmove` each; `solveron_run.log` `busy=[]` at start and end. `tools/ci.sh` at `ffc5c10`: gate 12 = solver oracle, gate 13 = solver determinism. Design §6 and the ledger rows say the same. **The prereg block** (extracted from the stash, sha256 `0c1de79c…`) run on a stub: exit 0, the `while read` seat loop and `${b1/-/ }` parse `depth_turns 4`, `nodes 200000`, `depth_turns 2`, `nodes 10000` — **and feed 508 searches a side, not 128** (§2 R3-M1). | three limbs **yes**; the prereg limb **NO — R3-M1** |
| **R2-M2** | §2.2: per-band ESTIMATE 1.94 % / 1.82 % from `count_early.err` / `count_late.err`; O-A's 2.8 / 2.1 and 2.6 / 2.6 *"at and somewhat above … by 0.3–0.9 points with nps IQRs of 0.2–0.5 %"*; *"not a ceiling and is not called one"*; reading 1 reworded | `sha256sum` `count_early.err` `ff6eb970…` ✓, `count_late.err` `4b6176b0…` ✓; their `P1COUNT` lines: 19017/350489 = **5.4258 %**, 14899/292342 = **5.0964 %**; × 35.67 → **1.9354 / 1.8179** → 1.94 / 1.82 ✓; 350489 + 292342 = 642831 = the pooled seat ✓. Excess: 2.8−1.94 = 0.86, 2.1−1.82 = 0.28, 2.6−1.94 = 0.66, 2.6−1.82 = 0.78 → "0.3–0.9" ✓. `grep -n -i ceiling` in revision 3 → lines 10, 73, 220, 610 — every one a negation or a quotation ✓. IQR range re-derived from the two O-A artifacts is 0.16–0.63 %, not 0.2–0.5 % — R3-m4. | **yes** (with R3-m4) |
| R2-m1 | §0 M1 row: *"RECONSTRUCTED after round 1 … verified by re-execution in round 2"* | `grep -n verbatim` in revision 3 → §0 row fixed; **§2.2 line 188 still reads *"the edit script is printed verbatim in `artifacts/p1_counters_ffc5c10_v2.txt`"*** — round 2 named both sites (line 43 and line 161 of revision 2). | **half** — R3-m1 |
| R2-m2 | §0 m6 row rewritten (*"a bench launched 7 s after a test link"*); §0.1 row says the sentence was *"deleted"* | `grep -n 'shell chain'` in revision 3 → line 75 (the §0.1 row claiming deletion) and **line 391: *"they were launched in one shell chain after the test suites had returned, and the measurement worktree's timestamps put a test link 26 s and 7 s before the E and E0 benches"*** — reading 5 still carries it, beside the figures that contradict it. | **no** — R3-m2 |
| R2-m3 | §5: *"in ascending `(q, r)` board order (`board.rs:87`), neither FIFO nor LIFO"* | `git show ffc5c10:crates/pistol-core/src/board.rs \| sed -n 87p` → `/// Every stone, in ascending (q, r) order.` ✓; the design's §1.4 (`:109`) says *"in board order"* ✓; `grep -n FIFO` → only the corrected line and the §0.1 row. | **yes** |
| R2-m4 | §2.4: `quiescence.rs:69`, `:260` | `git grep -n -E 'search_nodes\s*\+=' ffc5c10` (WHOLE tree, any increment form) → `pvs.rs:220`, `quiescence.rs:69`, `quiescence.rs:260` and nothing else; `sed -n 234p` → `if score > best_score {` ✓. | **yes** |
| R2-m5 | §3 O-I: compared against O-D1 (1.016 / 1.017), *"one to two points faster"*; stacked with the logged undo *"≈ 1.14–1.15 nps against O-E's measured 1.257 / 1.240 … ten points under"* | §4.1 table line 361: O-D1 1.016 / 1.017, O-D2 1.128 / 1.113 ✓. 1.4–1.7 % of wall → nps 1.0142–1.0173 → "one to two points" ✓. Early: 1.128 × 1.014–1.017 = **1.144–1.147** ✓; late: 1.113 × 1.014–1.017 = **1.129–1.132**, not stated; gaps 1.257−1.147 = 0.110, 1.240−1.132 = 0.108 → "ten points" holds in both. | **yes** (with R3-m3) |
| R2-m6 | the D-254 line-count clause deleted | `grep -n 'line count'` in revision 3 → 0 hits; §6 now reads *"this project's ordinary trade, and the matrix takes it"* ✓. | **yes** |
| R2-m7 | §4.3 cites `artifacts/p1_rt_round2/sizeof.log` (`4024bb86…`) | `sha256sum` ✓; content `RT2SIZE Touched 10 Applied 6 (u64,Chunk) 24 Window 6 ClassSet 1 Coord 4`, `test result: ok. 1 passed` ✓. The receipt names the worktree, not the revision; `4298ecd` is attested by round 2's §1a and by the types, which exist only on the O-E branch (noted, not rated). | **yes** |
| R2-m8 | §1.2 row 5: `window_count()` → definition + `lib.rs:41`; `threats.is_empty()` → one caller, `threat_oracle_tests.rs:230` | `git grep -n 'window_count' ffc5c10` (whole tree) → `state.rs:106`, `lib.rs:41`, `docs/decisions.md:563` (a doc, not a caller) ✓. `git grep -n 'threats.is_empty' ffc5c10` (whole tree) → `:230` only ✓; `git grep -n -E '\.is_empty\(\)' ffc5c10 -- crates/pistol-solver` → 103 hits, every other one on a `hot_windows(..)` / `Vec` / `str` receiver, none on a `ThreatState` ✓; `sed -n 228-232p` → the roundtrip oracle's fully-unwound `assert!(threats.is_empty(), …)` ✓. | **yes** |
| receipts | `artifacts/p1_rt_round2/` + `p1_rt_round2_digests.txt` `d5b695ee…` | `sha256sum artifacts/p1_rt_round2_digests.txt` → `d5b695ee0975…` ✓; `cd artifacts/p1_rt_round2 && sha256sum -c ../p1_rt_round2_digests.txt` → **33 of 33 `OK`** (the list carries bare names, as round 1's does, so it checks from inside the directory). | **yes** |
| header, ledger | revision 2 = `40b69e3`, *"eight minors"*; ledger rows for rounds 2 and 3 | round 2's report header names `40b69e390b8f…`; its minors are R2-m1..m8 ✓; ledger lines 32-34 carry the round-2 verdict verbatim and the round-3 dispatch ✓. | **yes** |

---

## 2. Findings

### BLOCKING

None.

### MAJOR

#### R3-M1 — the identity block feeds the keyed tactical fixture line by line: 508 searches a side, 460 after a refused `position`, and *"128 `bestmove` lines and 0 `error` lines"* is unsatisfiable as spelled

**Claim attacked.** `p1_bench_prereg.md` §2.1 (the command block, rewritten in
the diff, and its new third seat) and §2.2 (*"128 `bestmove` lines and 0
`error` lines on each side"*, changed from 88 in the diff); the matrix's §0.1
R2-M1 row (*"the prereg's identity leg gains the solver-on seat"*) and §6
attack 1 (*"byte-identity at three seats, the solver-on one included"*), which
rest on it.

**The fixture is keyed, and the block does not know it.**

```
$ git show ffc5c10:crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt | grep -v '^#' | grep -c .
115
$ … | grep -v '^#' | grep . | awk '{print $1}' | LC_ALL=C sort | uniq -c | LC_ALL=C sort -rn
     34 expect
     20 position
     20 config
     20 case
     20 budget
      1 require
$ git show ffc5c10:crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt | sed -n 's/^position //p' | wc -l
20
```

The block's entry filter is `grep -v '^#' "$fixture" | grep .`, and each
surviving line goes out as `position <line>`. That is right for
`bench_positions_v1.txt` (24 bare `start moves …` entries with a trailing
`# src …`, which `${entry%% #*}` strips) and wrong for
`tactical_staged_v0.txt`, whose 20 positions are `position set …` lines among
95 `case` / `config` / `budget` / `expect` / `require` lines.

**Executed** — the block extracted from the stash (sha256 `0c1de79c…`, 24
lines), `BASE` and `CAND` both a stub that echoes each `position` line and
answers every `go` with an `info totals` line and a `bestmove`:

```
BLOCK_EXIT=0
BASE lines 1524 bestmove 508 error 0
CAND lines 1524 bestmove 508 error 0
RESULT: IDENTICAL
position lines: 508 = 460 from tactical_staged_v0.txt (2 seats x 2 budgets x 115) + 48 from bench_positions_v1.txt (2 x 24)
first token after `position`: 136 expect, 80 position, 80 config, 80 case, 80 budget, 48 start, 4 require
```

**And on the engine** — the session's own `pistol-ffc5c10` (`78a7600a…`):

```
$ printf 'newgame\nposition position set p1:0,0 1,0 2,0 3,0 4,0 p2:-1,0 1,3 2,3 3,3 1,5 2,5 tomove:p1 phase:0\ngo depth_turns 1\nquit\n' \
    | pistol-ffc5c10 --config configs/tactical_staged_v0.toml
error Protocol: expected `start` or `set`, got `position` (in: "position position set p1:0,0 …")
info totals depth_turns 1 seldepth 1 nodes 3 … pv 0,0
bestmove 0,0
$ printf 'newgame\nposition case mate_in_1_five_in_a_row_blocked_at_one_end\ngo depth_turns 1\nquit\n' | … --config configs/tactical_staged_v0.toml
error Protocol: expected `start` or `set`, got `case` (in: "position case mate_in_1_five_in_a_row_blocked_at_one_end")
…
bestmove 0,0
$ printf 'newgame\nposition set p1:0,0 1,0 2,0 3,0 4,0 p2:-1,0 1,3 2,3 3,3 1,5 2,5 tomove:p1 phase:0\ngo nodes 10000\nquit\n' \
    | pistol-ffc5c10 --config configs/gate_staged_solver_v0.toml
info totals depth_turns 1 seldepth 1 nodes 1 search_nodes 0 solver_nodes 1 solver_firings 1 solver_invocations 1 solver_proofs 1 …
bestmove -9,0/5,0
```

So the governed leg as spelled prints, per side, 508 `bestmove` lines and 460
`error` lines, and every search at both tactical-fixture seats — the WP-1.9
staged seat and the new solver-on seat — is the empty board after `newgame`,
compared with itself. §2.2's positive-content check refuses it (508 ≠ 128,
460 ≠ 0), so the run cannot pass silently; but a registered run that is
guaranteed to STOP on the instrument's own spelling is what the dry-run
discipline and D-591 exist to catch BEFORE dispatch, and §2.3's falsifier
would not catch it either — a mutant that mismatches at the instrument seat's
48 real searches prints `MISMATCH` while two of three seats compare nothing.

**The count is right; the block is wrong.** `tools/determinism.sh` at
`ffc5c10` defines an entry as a `position` line and cross-checks it:

```
204:	mapfile -t positions < <(sed -n 's/^position //p' "$fixture")
211:	cases="$(grep -c '^case ' "$fixture" || true)"
213:		fail "$name: extracted ${#positions[@]} positions from $fixture but it states $cases cases"
```

With that definition (20 = 20 cases) the prereg's arithmetic is exact:
(20 + 24) × 2 + 20 × 2 = **128**, and §0's own instrument table already says
*"20 positions"* for this fixture. The block's filter says 115. The defect
was latent in revision 2's block at the first seat (same filter, same
fixture, *"44 positions … 88 `bestmove` lines"*), which round 2 did not
execute (its §5 reviewed the drafts for contradictions only); revision 3
rewrote the loop, added a third seat over the same fixture, and derived 128
from a count the block does not produce.

**Remedy.** Extract keyed fixtures the way the gate does (`sed -n 's/^position //p'`,
with the `case`-count cross-check) and keep the bare-line path for
`bench_positions_v1.txt`; run the block on a stub and record `128 / 0` in the
prereg beside the block before the revision carrying it is dispatched
(D-591); add the third seat's config to §0's instrument table (R3-m5). Nothing
in the matrix's numbers or its recommendation changes.

### MINOR

#### R3-m1 — R2-m1's remedy was applied at one of its two sites

`matrix_P1_threat_state.md:188` (§2.2): *"the edit script is printed verbatim
in `artifacts/p1_counters_ffc5c10_v2.txt`"*. Round 2 named §0 M1 AND §2.2;
the §0 row now says *"RECONSTRUCTED after round 1 … verified by re-execution"*
and §2.2 still says *"verbatim"*. Reproducer: `git show e13c3e4:docs/experiments/matrix_P1_threat_state.md | grep -n 'printed verbatim'` → `188`.
D-423's class: one claim, two places, one fixed.

#### R3-m2 — §0.1 says the "shell chain" sentence was deleted, and reading 5 still carries it

`matrix_P1_threat_state.md:390-391`: *"they were launched in one shell chain
after the test suites had returned, and the measurement worktree's timestamps
put a test link 26 s and 7 s before the E and E0 benches"* — the sentence
round 2 asked deleted, standing beside the two figures that contradict it;
and line 75, the §0.1 row, records the remedy as *"deleted; the record is
stated"*. Reproducer: `grep -n 'shell chain'` → `75`, `391`. Nothing leans on
the sentence (the idle re-measurement does the work, as round 2 said), which
is why this stays MINOR; but a remedy table that records a deletion not made
is the record defect the table exists to end.

#### R3-m3 — the O-I stacking estimate is the early band's, stated against both bands

§3 O-I: *"≈ 1.14–1.15 nps against O-E's measured 1.257 / 1.240"*. 1.128 ×
1.014–1.017 = 1.144–1.147 is the EARLY composition; the late one is 1.113 ×
1.014–1.017 = 1.129–1.132. *"Ten points under"* holds in both (0.110 / 0.108)
and the rejection stands; the figure was transcribed from round 2's sentence
rather than derived per band, in a document that reports every other ratio
per band.

#### R3-m4 — "nps IQRs of 0.2–0.5 % of the median" is round 2's phrase, not the artifacts' range

From `artifacts/p1_mx_bench_A_v1.txt:32,36` and `artifacts/p1_rt_round1/bench_A.txt:30,34`
(IQR ÷ median): session A early 1994.1/446670.6 = 0.45 %, 1048.6/459276.9 =
0.23 %, late 927.0/393967.0 = 0.24 %, 644.5/402174.7 = 0.16 %; round-1 idle
early 0.45 %, 0.23 %, late 2497.4/395207.9 = 0.63 %, 1307.8/405423.3 = 0.32 %
— **0.16–0.63 %**. The reading survives: the smallest excess (0.28, the
session's late band) sits over IQRs of 0.24 / 0.16 %, and the idle run's late
excess 0.78 over 0.63 / 0.32 %. State the derived range or cite the lines.

#### R3-m5 — the prereg's §0 instrument table does not list the third seat's config

`p1_bench_prereg.md` §0 names `instrument_v0.toml`, `tactical_staged_v0.toml`,
both fixtures, `bench_delta.sh` and the block, each with its revision;
`configs/gate_staged_solver_v0.toml` (`git log -1 --format=%h ffc5c10 -- configs/gate_staged_solver_v0.toml` → `e4bb5bf`)
is now an instrument of the identity leg and is absent. The table's stated
purpose is that every artefact the run reads is named with its revision.

---

## 3. Round 2's §5 draft contradictions — do they resolve?

| round 2 §5 item | revision 3 | resolved? |
|---|---|---|
| design header cites *"revision 1 and its selection record"*, a stale revision and a file that does not exist | header now: *"at its final revision and the selection record written after its last red-team round (`matrix_P1_threat_state_selection.md`), which takes O-E; this draft precedes that record and is not dispatched for review until it exists"* | **yes** — no revision number is asserted and the record is named as forthcoming; when the design IS dispatched the header must name the matrix's revision (noted, not rated) |
| design §1.4 *"board order"* vs matrix §5 *"FIFO"* | matrix §5 line 502 now *"ascending `(q, r)` board order (`board.rs:87`)"*; design `:109` unchanged | **yes** |
| design §6 *"gated off in every committed config"* | design §6 names the three configs, the determinism seat, the ≈ 2 % / ≈ 0 % measurement, and banks none of it | **yes** |
| prereg registers the bracket at the instrument seat only, worth saying out loud | §1 gains *"**Seat:** the instrument seat only …"* | **yes** |

---

## 4. Re-derivation table — my commands, none the document's

| claim (§) | my command, with scope | my number | document's | agree? |
|---|---|---|---|---|
| armed configs (§6 attack 1, design §6) | `git grep -n -E 'on_search_path\s*=\s*true' ffc5c10`, whole tree | 3 configs (+ 39 doc hits) | three, named | yes |
| determinism seat, census tests | `git grep -n 'gate_staged_solver_v0' ffc5c10`, whole tree | `determinism.sh:76`, `census_protocol_tests.rs:10`, `census_identity_tests.rs:14` | same | yes |
| gate's budgets | `git show ffc5c10:tools/determinism.sh \| sed -n 76p;85p` | `depth_turns-2 nodes-10000`; `BUDGETS=("depth_turns 4" "nodes 200000")` | same | yes |
| solver-on run (§6) | python over `solveron.{base,E}.{1,2}.tot` + `bands`; `grep -c` on transcripts | identity 24 × 4; 1.022 / 1.005, 1.015 / 1.001; 13.4 k nps; 0 errors; 24 `bestmove` | same | yes |
| instrument-seat nps | `bench_E_v1.txt:32,36` | 446 k early / 393 k late | "about 450 k" | yes |
| digests (§0.1, §2.2, §4.3, §6) | `sha256sum` of each; `sha256sum -c` from inside the dir | `d5b695ee`, `61baa09e`, `ff6eb970`, `4b6176b0`, `4024bb86`, `8414509a`, `78a7600a`, `f333d101`; 33/33 OK | same | yes |
| per-band share (§2.2) | arithmetic over the two `.err` receipts | 5.4258 / 5.0964 %; 1.9354 / 1.8179 % | 5.43 / 5.10; 1.94 / 1.82 | yes |
| O-A excess (§2.2) | arithmetic | 0.28–0.86 | 0.3–0.9 | yes |
| O-A IQRs (§2.2) | IQR ÷ median from two artifacts | 0.16–0.63 % | 0.2–0.5 % | **no** — R3-m4 |
| O-I stacking (§3) | arithmetic per band | 1.144–1.147 early, 1.129–1.132 late | 1.14–1.15 | early only — R3-m3 |
| `search_nodes` sites (§2.4) | `git grep -n -E 'search_nodes\s*\+=' ffc5c10`, whole tree | `pvs.rs:220`, `quiescence.rs:69`, `:260` | same | yes |
| `board.rs:87` (§5) | `sed -n 87p` at `ffc5c10` | *"Every stone, in ascending `(q, r)` order."* | same | yes |
| `is_empty()` caller (§1.2) | `git grep -n 'threats.is_empty' ffc5c10`, whole tree; all 103 `.is_empty()` in pistol-solver read by receiver | `threat_oracle_tests.rs:230` only | same | yes |
| `window_count()` (§1.2) | `git grep -n 'window_count' ffc5c10`, whole tree | `state.rs:106`, `lib.rs:41`, `decisions.md:563` (doc) | definition + `lib.rs:41` | yes |
| `size_of` (§4.3) | `cat sizeof.log` | 10 / 6 / 24 | same | yes |
| ci gates (§6) | `git show ffc5c10:tools/ci.sh \| grep 'gate 1[23]/'` | 12 solver oracle, 13 solver determinism | "gates 12 and 13" | yes |
| fixture entry counts (prereg §2) | `grep -v '^#' \| grep -c .` vs `sed -n 's/^position //p' \| wc -l` vs `grep -c '^case '` | 115 vs 20 vs 20 (`tactical_staged_v0.txt`); 24 / 24 (`bench_positions_v1.txt`) | 20; 24 | as `position` lines yes; **as the block reads them, no — R3-M1** |
| searches a side (prereg §2) | stub run of the block | 508 | 128 | **no — R3-M1** |
| seat loop, budget spelling (prereg §2) | stub run | four budgets parsed as spelled | works | yes |
| leftover words | `grep -n -E 'verbatim\|shell chain\|ceiling\|gated off\|FIFO\|:234\|line count\|no caller'` on revision 3 | `verbatim` at 188 (R3-m1); `shell chain` at 391 (R3-m2); the rest only negated, quoted or in the §0 tables | — | see minors |

---

## 5. Out of scope — noted, not rated

- The digest list `p1_rt_round2_digests.txt` carries bare names (round 1's
  does too); `sha256sum -c` from the repository root prints 33 `FAILED open
  or read`. A path prefix would let it check from anywhere. Not in the diff.
- `solveron_run.sh` greps `cargo|rustc|bench_delta` for its idle receipt, not
  `pistol` — right for a run whose own binaries are `pistol-*`, and the
  prereg's registered pattern (§1.3, unchanged) includes `pistol`, which will
  match the harness's own shell line and print it rather than `idle`; the
  receipt is operational, not a criterion, so nothing turns on it.

---

VERDICT: the recommendation SURVIVES but the matrix FAILS (revise: R3-M1 — the prereg's identity block, rewritten in the diff and given a third seat over the same fixture, reads `tactical_staged_v0.txt`'s 115 keyed lines as positions, so as spelled it sends 508 searches a side, 460 of them refused with `error Protocol: expected \`start\` or \`set\``, and the registered *"128 `bestmove` lines and 0 `error` lines"* cannot be met by any binary pair; the count is right as `determinism.sh:204` defines an entry, the block is what is wrong, and R2-M1's fourth limb — *"let the prereg's identity leg be where that seat is checked"* — was carried as a fix without the execution D-591 requires)

**Where the failing lines are, for the operator's ruling.** They are
`p1_bench_prereg.md` §2.1–2.2 — a document whose own review gate
(`opt_arc_ledger.md:42`, *"bench pre-registration … + its review \| not
started"*) has spent no rounds. Every claim in the matrix's own lines holds on
execution; the recommendation and all its numbers are untouched; the fix's
shape is in the tree. Whether a FAIL on the diff's prereg lines is the arc's
STOP-and-split or the prereg gate's first finding is a ruling, not a review
result, and this report does not soften the finding to avoid it.

**Strongest surviving attack, for the selection record verbatim.** O-E's
24–26 % is the instrument seat's number and only that seat's: at the
committed, CI-gated solver-on seat — the one whose caller drives the
LIFO-or-panic contract hardest — the same two binaries are about two per cent
apart early and indistinguishable late, measured once, exploratory, on one
hand-rolled instrument, so the gain the package banks is a property of one
node mix and the narrowing is bought, at the solver's seat, for nothing
measurable; the one check the package offers at that seat, the identity leg's
third seat, cannot run as registered, so byte-identity at the solver's seat
is at this revision a promise and not an instrument; the premise that
licensed the matrix's shape is loose at the edge, since O-A's measured gain
sits 0.3–0.9 points above the per-band estimate the unqueried share gives it
and the discrepancy is recorded rather than explained; the one alternative
D-254 called faster is still rejected on a figure from another instrument at
another revision, unprototyped, its stacked estimate stated for one band; and
the chunk-boundary and lattice-edge behaviour of the store that wins has no
falsifier until the design's I5 and I6 exist.
