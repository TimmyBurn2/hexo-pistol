# MATRIX P1 — scoped verification pass (D-598) after round 3

**Revision reviewed.** `docs/experiments/matrix_P1_threat_state.md` at
`faf7a54faa55c3405c7ba4c9055524fbbc0c8ecd`, a `git stash create` commit on
`dev` = `ffc5c10f4d16356f574e3221a023f78399d2e3bb` (parents `ffc5c10`,
`3154470`). Previous revision `e13c3e4a61d90e39c20844ad86831c29b2e96621`.
**Matches the working tree?** Yes — `git show faf7a54:<path> | sha256sum` and
`sha256sum <path>` both give `f75313f8…`. **HEAD** at review time:
`ffc5c10f4d16356f574e3221a023f78399d2e3bb`. **Date:** 2026-09-04.
**Model:** `claude-fable-5-1` (recorded per D-597).

**What this is.** A scoped verification pass under D-598, dispatched by
`opt_arc_ledger.md` F-P1.4: a fresh context over (1) the four lines the diff
`git diff e13c3e4 faf7a54 -- docs/experiments/matrix_P1_threat_state.md`
corrects for R3-m1..m4, and (2) their defect class over the WHOLE document —
(a) a number transcribed from a reviewer's prose or another run rather than
derived, (b) one claim in two places with one corrected (D-423), (c) a
cross-reference that no longer resolves. R3-M1 and R3-m5 concern
`p1_bench_prereg.md` and are not this pass's subject. Per `docs/process.md`'s
re-derivation clause every count and citation below was re-derived with a
command I chose, printed with its scope; nothing was reproduced by running the
document's own command. No cargo, no timing run; the one execution outside
`git`/`grep`/`sha256sum`/`python3` was `perf script`/`perf report` over an
existing `perf.data`, which is reading an artifact.

**Verdict, stated first.** The four corrected lines hold under my own
re-derivation. The class sweep found three MINOR instances of the class —
a count transcribed as "eleven" where the document's own table and the whole
tree hold ten, a "Four readings" header over five items numbered 1 2 3 5 4,
and a header sentence copied from the ledger's prose with its qualifying
parenthetical dropped — and nothing above MINOR. No number in the matrix
moves; the recommendation is untouched.

---

## 1. The corrected lines — R3-m1..m4

| finding | correction in the diff | my re-derivation (command, scope) | holds? |
|---|---|---|---|
| **R3-m1** — §2.2 still said the counter script is *"printed verbatim"* | line 198-200: *"the edit script, reconstructed after round 1 and verified by re-execution in round 2, is printed in `artifacts/p1_counters_ffc5c10_v2.txt`"* | `/usr/bin/grep -n -F verbatim docs/experiments/matrix_P1_threat_state.md` (whole file) → **85** (the §0.1 row quoting the finding), **595**, **615** (*"carried verbatim"* of the two strongest-attack quotations — a different claim, not the script). No line calls the script the session's verbatim original. `sha256sum` of v2 → `65a16dee…` ✓; `grep -n P1COUNT` on v1 and v2 → the same three output lines at `:3,5,7` of each; v2 additionally holds the script (`grep -c -E 'python\|import\|def \|patch'` → 2 in v2, 1 in v1) ✓. | **yes** |
| **R3-m2** — reading 5 still carried the *"shell chain"* sentence §0.1 said was deleted | lines 404-406: *"The session's own runs carried no idle receipt (m6), and the measurement worktree's timestamps put a test link 26 s and 7 s before the E and E0 benches"* | `/usr/bin/grep -n -F 'shell chain' docs/experiments/matrix_P1_threat_state.md` (whole file) → **86** only, the §0.1 row recording the finding. The sentence is gone from reading 5 and §0.1's *"deleted"* is now true. | **yes** |
| **R3-m3** — the O-I stacking estimate was the early band's, stated against both | line 352-354: *"1.144–1.147 early and 1.129–1.132 late (`1.128 × 1.014–1.017`, `1.113 × 1.014–1.017`) against O-E's measured 1.257 / 1.240 … eleven points under in both bands"* | python: 1.128 × 1.014 = **1.143792**, 1.128 × 1.017 = **1.147176** → 1.144–1.147 ✓; 1.113 × 1.014 = **1.128582**, 1.113 × 1.017 = **1.131921** → 1.129–1.132 ✓. Gaps: 1.257 − {1.147, 1.144} = **0.110–0.113**; 1.240 − {1.132, 1.129} = **0.108–0.111** — all four round to **eleven** points, so the author's change from round 2's *"ten"* to *"eleven"* is a derivation, not a transcription. The factor itself: 4.0 % and 4.8 % of 35.67 % = 1.427 % and 1.712 % of wall → nps 1/(1−f) = **1.0145–1.0174** → *"1.014–1.017"* and *"1.4–1.7 % of wall"* ✓. O-D2's 1.128 / 1.113 re-derived from `artifacts/p1_mx_bench_D2_v1.txt:32,36` (504281.4/447002.2 = 1.1281; 439119.9/394586.5 = 1.1129) ✓. | **yes** |
| **R3-m4** — *"nps IQRs of 0.2–0.5 %"* was round 2's phrase | lines 231-233: *"over nps IQRs of 0.16–0.63 % of their medians (`artifacts/p1_mx_bench_A_v1.txt:32,36` and `p1_rt_round1/bench_A.txt:30,34`, IQR ÷ median; R3-m4)"* | The four lines, printed by `cat -n … \| sed -n`: `p1_mx_bench_A_v1.txt:32` *"nps baseline median 446670.6 (IQR 1994.1), candidate median 459276.9 (IQR 1048.6)"*; `:36` *"393967.0 (IQR 927.0) … 402174.7 (IQR 644.5)"*; `p1_rt_round1/bench_A.txt:30` *"448333.6 (IQR 2006.0) … 459978.6 (IQR 1055.0)"*; `:34` *"395207.9 (IQR 2497.4) … 405423.3 (IQR 1307.8)"*. IQR ÷ median (python): 0.446 %, 0.228 %, 0.235 %, **0.160 %**; 0.447 %, 0.229 %, **0.632 %**, 0.323 % → range **0.16–0.63 %** ✓. Digests `87a081a5…` / `c893a81e…` ✓. The excess *"0.3–0.9 points"*: 2.8−1.94 = 0.86, 2.1−1.82 = 0.28, 2.6−1.94 = 0.66, 2.6−1.82 = 0.78 ✓. | **yes** |

---

## 2. The defect-class sweep — what I looked at

### 2a. Every ratio, percentage and count, re-derived or named unsourced

Each row: my command with its scope, my number, the document's, agreement.
"session" = `artifacts/p1_mx_bench_*_v1.txt`; "idle" = `artifacts/p1_rt_round1/`.

| claim (line) | my command, scope | my number | document's | agree? |
|---|---|---|---|---|
| `.undo(`/`.apply(` hits (`:110`) | `git grep -n -E '\.(undo\|apply)\(' ffc5c10` — WHOLE tree, no pathspec, then bucketed by extension | **189** = 177 `.rs` + 12 `.md`; 0 `.py`, 0 `.sh` | 177 over `.rs`/`.py`/`.sh` | yes (the document's scope is the `.rs` population; no `.py`/`.sh` hit exists) |
| `ThreatState` undo sites (`:114`, table `:118-125`) | `git grep -n -E '\.undo\([^)]+,[^)]+\)' ffc5c10` (whole tree, every two-argument undo) minus `eval.undo` / `self.undo` / `unwound.undo` / `self.0.undo` receivers, each survivor read; `git grep -n -E 'ThreatState::(undo\|apply)\b' ffc5c10` → no code hit | **ten**: `position.rs:142`; `dfpn.rs:720,722`; `policy.rs:155,156,398,399`; `threat_oracle_tests.rs:222,336,344` — exactly the table's ten | *"eleven"* | **no — V-1** (the table is complete; the word is wrong) |
| apply-only sites (`:127-135`) | `git grep -n -E '\.apply\(' ffc5c10` (whole tree) with `threat` on the line or path, and every remaining hit outside `pistol-core`/`pistol-eval`/docs read (`board.apply`, `symmetry.apply`, `eval.apply` only) | 30 sites, the document's list exactly | 30 listed | yes |
| `policy.rs:147` debug-only; `:304` test module | `git show ffc5c10:crates/pistol-solver/src/policy.rs \| sed -n 147p` → `#[cfg(debug_assertions)]` | ✓ | ✓ | yes |
| nine set queries (`:151`) | `git show ffc5c10:…/query.rs \| sed -n 127,218p \| grep -c 'pub fn '` | **9** (`hot_windows` … `can_win_this_turn`) | nine | yes |
| ten sorted sets (`:145`) | `state.rs:30` `sets: [WindowSets; 2]`; `sets.rs:141` `sets: [Vec<Window>; CLASS_COUNT]`; `sets.rs:7` `CLASS_COUNT = 5` | 2 × 5 = **10** | ten | yes |
| `is_empty()` caller; `window_count()` readers (`:155`) | round 3 re-derived whole-tree; I re-ran `git grep -n 'threats.is_empty' ffc5c10` (whole tree) → `:230` only | ✓ | ✓ | yes |
| profile (`:176-187`) | `cat -n artifacts/p1_profile_ffc5c10_v1.txt` → `:21-27`; sums by hand | 30.64 / 14.04 / 9.86 / 7.19 / 6.97 / 6.34 / 4.58; 14.04+9.86+7.19+4.58 = **35.67**; 9.86+7.19 = **17.05**; 6.97+6.34 = **13.31** | same | yes |
| *"2 074 samples"* (`:177`) | the artifact says only *"Samples: 2K"*; `perf script -i /home/tom/pistol-wt/p1-measure/perf_ffc5c10_line2.data \| wc -l` (my command) | **2074**; `perf report -n` gives 622/293/203/148/143/127/94 samples at the same seven symbols and the same printed percentages (period-weighted, as perf reports them) | 2 074 | yes — derived from the `perf.data`, not from the cited artifact (noted, §3) |
| audit column (`:179-187`) | `sed -n 34,35p docs/audit/repo_audit_2026-09.md` (A-02 receipt at `d83ac01`, 2 105 samples) | 31.77 / 13.45 / 9.55 / 7.98 / 6.29 / 6.64 / 5.14; A-03 sums to **36.12** | same | yes |
| counters table (`:212-214`) | `grep -n P1COUNT artifacts/p1_counters_ffc5c10_v2.txt` → `:3,5,7`; ratios by hand | 33916/642831 = **5.276 %**; 3780/44166 = **8.559 %**; 16650/240811 = **6.914 %**; hist 431/608915, 136/40386, 52/224161 | 5.28 / 8.56 / 6.91 | yes |
| root / re-visit split (`:66`, `:217`) | `grep RTCOUNT artifacts/p1_rt_round1/count_{nodes,depth_turns,line2}.err` | `root0 84 revisit0 347`; `46 / 90`; `5 / 47`; 84+347 = 431, 46+90 = 136, 5+47 = 52 | same | yes |
| flush totals (`:243-244`) | arithmetic over the three histograms | 609346+40522+224213 = **874 081**; 608915+40386+224161 = **873 462**; 431+136+52 = **619**; 0 at ≥ 2 | same | yes |
| per-band share and estimate (`:228-229`) | `grep P1COUNT artifacts/p1_rt_round2/count_{early,late}.err` (`ff6eb970…`, `4b6176b0…`) | 19017/350489 = **5.4258 %**, 14899/292342 = **5.0964 %**; × 0.3567 = **1.935 % / 1.818 %**; 350489+292342 = 642831 = the pooled seat | 5.43 / 5.10; 1.94 / 1.82 | yes |
| O-A's *"5.3–8.6 %"*, *"0.7–1.1 % of wall"* (`:293`) | 5.28–8.56 % × 13.31 % | **0.703–1.139 %** | 0.7–1.1 | yes |
| §4.1 table, all 36 numeric cells + 12 digests (`:373-380`) | my parser over the six session artifacts: medians and IQRs from `:32,36`, ttd from `:33,37`, ratios recomputed from the medians, digests from `:3,4`; `sha256sum` of each artifact | every nps ratio, ttd ratio, baseline and candidate median, largest-IQR % (0.446 / 0.814 / 0.503 / 0.743 / 0.780 / 0.664), binary digest and artifact digest reproduces | same | yes |
| reading 2 (`:389-392`) | idle `bench_D1_to_D2.txt:30,34`, `bench_E0_to_E.txt:30,34`; digests | 508111.4/457531.9 = **1.1105**, 439889.6/403143.8 = **1.0911**; 563247.9/534735.3 = **1.0533**, 491590.6/471280.8 = **1.0431**; `903ba0cd…`, `9df4bd58…`; identity line at `:28` in both | 1.111 / 1.091; 1.053 / 1.043 | yes |
| reading 3 (`:395-396`) | idle `bench_E_to_EA.txt:30,34` | **1.0181 / 1.0139**; `bd2a231e…` | 1.018 / 1.014 | yes |
| reading 5 (`:401-403`) | idle `bench_{A,E0,E,EA}.txt:30,34`; deviation from the session table | 1.026/1.026, 1.183/1.181, 1.256/1.241, 1.267/1.251; worst \|deviation\| **0.59 %** (O-EA late); four digests ✓ | same; *"within 0.7 %"* | yes |
| reading 5's *"26 s and 7 s"* (`:405`) | artifact headers: E launched `11:18:56 PM UTC`, E0 `11:22:42 PM UTC`; `find …/p1-measure/target -perm -u+x -newermt … ! -newermt …` in the two windows | the last executables linked before E0 are `release/pistol` and the corpus binaries at 01:22:31.99 local (≈ 10 s before the header line); nothing survives in E's window; the test binaries now carry 02:51 mtimes from a later build | 26 s / 7 s | **not re-derivable today** — sourced to round 2's report (m6 / R2-m2); nothing leans on it (§3) |
| reading 4 (`:409`) | ttd medians and IQRs over all six session artifacts `:33,37` | medians **66–117 ms**, max IQR **2.0 ms** | 66–117 ms, ≤ 2 ms | yes |
| §4.2 diff and line counts (`:419-422`) | `git -C /home/tom/pistol-wt/p1-measure diff --shortstat ffc5c10 <rev> -- crates` for the six revisions; `git show <rev>:<file> \| wc -l` for the three files at seven revisions | +14/−4, +47/−16, +106/−34, +255/−110, +276/−111, +290/−115; `state.rs` 122/122/122/163/173/193/193; `table.rs` 214/214/245/245/308/308/308; `position.rs` 165/175/165/165/165/165/175 | same | yes |
| rule-9 entry (`:425-426`) | `/usr/bin/grep -c 'pistol-solver/src/table.rs' docs/rule9_justifications.md` live and via `git show ffc5c10:` | 0 / 0 | 0 | yes |
| §4.3 sizes and total (`:434-443`) | `sizeof.log` (`4024bb86…`): `Touched 10 Applied 6 (u64,Chunk) 24`; `search.rs:24,35,41` at `ffc5c10`: `64`, `32`, `2 * MAX_DEPTH_TURNS + 2 + MAX_Q_EXTENSION_PLIES` | 18×10 + 3×24 + 6 = **258 B**; 2×64+2+32 = **162**; (80+162)×258 = **62 436 B = 62.4 KB**; `tt_bytes = 268435456` at `instrument_v0.toml:24` = 268 MB | same | yes |
| §6 armed configs, gate seat, CI gates (`:560-563`, `:577`) | `git grep -n -E 'on_search_path\s*=\s*true' ffc5c10 -- ':(glob)configs/**'`; `determinism.sh:76`; `git show ffc5c10:tools/ci.sh \| grep 'gate [0-9]*/'` | three configs; the `staged-solver` seat at `:76`; gate 12 *solver oracle*, gate 13 *solver determinism* | same | yes |
| §6 solver-on run (`:566-571`) | my python over `artifacts/p1_rt_round2/solveron.{base,E}.{1,2}.tot` + `bands` (24 lines each, 12 early / 12 late); `grep -c` on the four transcripts; `solveron_run.log:1,9` | node identity 4 × 24 ✓; rep 1 **1.0219 / 1.0051**, rep 2 **1.0148 / 1.0008**; 24 `bestmove` / 0 `error` each; `busy=[]` at start and end; pooled base 2 286 618 nodes / 170.2 s = **13.4 k nps** | 1.022 / 1.015; 1.005 / 1.001; about 13 k vs about 450 k | yes |
| *"246 tests green on E, E0 and EA"* (`:577`) | `grep '^test result' artifacts/p1_rt_round1/test_{base,E0,E,EA}.log` summed | 38 suites, **246 passed**, each | 246 | yes (uncited in the document; sourced to the round-1 receipts) |
| digests: `a63ee080`, `d5b695ee`, `ed5f0cab`, `f7e07c4f`, `65a16dee`, `dd731212`, `afcc2b97`, `ff6eb970`, `4b6176b0`, `4024bb86`, `61baa09e`, `8414509a`, `02e2e93b`, `931c50b1`, `78a7600a` + six binaries | `sha256sum` of each live file; `git show ffc5c10:` for the config and fixture; `sha256sum -c` from inside each receipt directory | all 27 match; round-1 list **36/36 OK**, round-2 list **33/33 OK** | same | yes |
| D-254 quotations (`:344-348`, `:256-257`) | `grep -o` over the D-254 line of `docs/decisions.md` | *"ahead by 4.0–4.8 % with a replication spread under 1 %"*, *"the faster option on every instrument that reads the corpus correctly"*, *"AN OPTION NOBODY CONSIDERED …"*, *"Flips when a bench with `p > 0` …"*, *"every positive performance ground … second harness"*, *"~90"* — all present | quoted | yes |
| D-220 thresholds (`:53`) | `grep -o` over D-220 | `[1.4, 2.5]` and `1.15` | same | yes |
| wp19 numbers (`:330`) | `grep -n '1\.737\|1\.837\|1\.783\|1\.909' docs/experiments/matrix_wp19_storage.md` → `:127-128` | O-4 dense 1.737 / 1.837; O-2 hashed 1.783 / 1.909 | same | yes |
| O-E edge and straddle claims (`:484-497`) | `git -C …/p1-measure show 4298ecd:…/table.rs` `:60` (`CHUNK_LEN = 64`), `:119-121` (doc: line `[-65536, 65534]`, chunk `[-512, 511]`), `:129-130` (biases `1 << 17`, `1 << 10`, fields `<< 16`, `<< 40`), `:213-221` (`offset + count > 64`, `high << (64 - offset)`) | chunk −513 → 511 and 512 → 1536 both fit the 16-bit chunk field without reaching `try_from`'s panic; at `offset = 0` an 11-bit run gives `0 + 11 > 64` false | as stated | yes (read, not executed) |

### 2b. One claim in two places — the phrases the three rounds asked to change

`/usr/bin/grep -n -F '<phrase>' docs/experiments/matrix_P1_threat_state.md`, whole file, one phrase at a time:

| phrase | hits | each hit is |
|---|---|---|
| `verbatim` | 85, 595, 615 | 85 = §0.1 row quoting R2-m1; 595 / 615 = *"carried verbatim"* of the strongest-attack quotations (a different claim) |
| `shell chain` | 86 | §0.1 row quoting R2-m2 |
| `ceiling` | 21, 84, 233, 623 | 21 = describing R2-M2; 84 = §0.1 row; 233 = *"not a ceiling and is not called one"*; 623 = inside round 2's attack, quoted |
| `gated off` | 83, 559, 609 | 83 = §0.1 row; 559 = *"is NOT 'gated off …', as revision 2 said"*; 609 = inside round 1's attack, quoted |
| `FIFO` | 87, 515 | 87 = §0.1 row; 515 = *"neither FIFO nor LIFO"* |
| `window_map.rs` | 69 | §0 m4 row |
| `:234` | 88 | §0.1 R2-m4 row |
| `no caller` | 92, 537 | 92 = §0.1 R2-m8 row; 537 = *"no caller anywhere in the repository undoes out of order"* — a different claim, supported by §1.1's ten sites (all LIFO) |
| `50–60` | 72 | §0 m7 row |
| `root's own` | 66, 216 | 66 = §0 m1 row; 216 = the corrected sentence (84 / 46 / 5 root, the rest re-visits) |
| `line count` / `line-count` | 0 / 90 | 90 = §0.1 R2-m6 row |

Every phrase survives only where it is quoted as the corrected finding, or in a
different claim that stands on its own.

### 2c. Cross-references

`/usr/bin/grep -o -E '§[0-9]+(\.[0-9]+)?'` over the file → §0, §1, §1.1, §1.2,
§2.1, §2.2, §2.3, §2.4, §3, §4.1, §4.2, §4.3, §5, §6 — every one a heading
that exists (`grep -n -E '^#{1,3} '`), and each read in context: §0's
*"second table"* is §0.1's; *"§2.3"* is the flip clause; *"§1.2 rows 1–2"* are
the set and cover queries with `masks` point reads; *"§1.2's last paragraph"*
is the every-node paragraph; *"§4.2 requires"* the `table.rs` split; *"§5
names"* the three boundary tests. The two external `§1`s are
`matrix_wp19_storage.md` §1 (exists, and says *"plus the derive list on the
struct"*) and round 2's report §1 (its M1 row is at `:46`). `reading 1` (`:64`,
`:356`) and `readings 2–3` (`:67`) resolve to §4.1's items 1, 2 and 3. The
design's *"I5 and I6"* (`:631`) exist at `p1_design.md:175-176`. The audit's
A-02 / A-03 are at `repo_audit_2026-09.md:34-35`; the dispatch's P1 at
`opt_arc_DISPATCH.md:52`; the prereg's *"at most"* at `p1_bench_prereg.md:68`
(*"AT MOST 35.67 %"*). Every `file:line` into the tree (`state.rs`,
`position.rs`, `pvs.rs`, `quiescence.rs`, `query.rs`, `cover.rs`,
`threat_oracle_tests.rs`, `lib.rs`, `search.rs`, `board.rs`, `policy.rs`,
`dfpn.rs`, `determinism.sh`) was printed with `git show ffc5c10:<path> | sed -n`
and holds what the document says it holds. **One list is mis-numbered — V-2.**

### 2d. What I did NOT look at — the edges of this pass

- `p1_bench_prereg.md` (R3-M1, R3-m5), `p1_design.md` beyond I5/I6 and
  §1.4:109, and `opt_arc_ledger.md` beyond F-P1.4. The prereg's identity block
  is the prereg gate's first finding, not this pass's.
- Nothing was built or run: the O-E branch's `pack` and run reader were READ
  at `4298ecd`; the straddle / edge claims are checked against the code's text,
  not executed. The design's I5 and I6 remain the falsifiers.
- The bench artifacts' per-rep raw lines were not re-parsed; the medians,
  IQRs and node-identity lines are the script's own printed results
  (`:30-38`), taken as the instrument's output.
- The profile percentages are perf's period-weighted figures; I did not
  recompute a share from raw sample counts (622/2074 = 30.0 % would differ from
  the printed 30.64 %, which is how `perf report` weights `-F` samples). The
  document quotes what perf printed, and so does the audit.
- Round 2's *"26 s and 7 s"* could not be re-derived: the test binaries in
  the measurement worktree were rebuilt at 02:51 and the evidence is gone.
- Arguments, not numbers: O-B's *"the solver's own callers query immediately
  after every `apply_turn`"*, O-F's `O(R²)` transfer, O-G's seam argument, the
  hasher paragraph, D-61's *"order-free"* beyond `position.rs:48-49`'s doc.
- The figures inside the two verbatim attack quotations (*"18 %"*, *"5.3 %
  early and 4.3 % late"*, *"24–26 %"*) are quotations and were not treated as
  the document's own claims; *"24–26 %"* is 1.240–1.257 in any case.

---

## 3. Findings by severity

### BLOCKING

None.

### MAJOR

None.

### MINOR

#### V-1 — "eleven `undo` sites" where the document's own table, and the whole tree, hold ten (class a: a count transcribed)

`matrix_P1_threat_state.md:114`: *"**eleven `undo` sites**"*. The table at
`:118-125` cites ten: `position.rs:142`; `dfpn.rs:720`, `:722`;
`policy.rs:155`, `:156`; `:398`, `:399`; `threat_oracle_tests.rs:222`; `:336`,
`:344`. Reproducer, my scope:

```
$ git grep -n -E '\.undo\([^)]+,[^)]+\)' ffc5c10 | /usr/bin/grep -v -E 'eval\.undo|self\.undo|unwound\.undo|self\.0\.undo' | wc -l
10
```

(the four `self.undo` hits in `reference_walk.rs` are `ReferenceWalk::undo`,
`:239-246`, which wraps `GameState` + eval; `ThreatState::undo` as a path
appears in no code line). The word was transcribed: round 1 wrote *"11 undo
sites"* (`REDTEAM.md:254`), round 2's M3 row enumerates the same ten line
numbers and calls them *"eleven"* (`REDTEAM_round2.md:48`), and the document
carried it through three revisions. The enumeration is complete and every site
is LIFO, so §1.1's conclusion and §5's contract argument are unchanged. Fix:
*"ten"*.

#### V-2 — "Four readings" heads five items, numbered 1, 2, 3, 5, 4 (class a/c)

`matrix_P1_threat_state.md:382` *"**Four readings, stated plainly.**"*; the
list items begin at `:384` (1), `:389` (2), `:395` (3), `:398` (**5**), `:407`
(**4**). Reproducer: `sed -n 382,411p … | grep -n -E '^[0-9]+\. '` → `1. 2. 3.
5. 4.`. Revision 1 (`6099aa7`) had four; revision 2 (`40b69e3`) inserted the
idle-reproduction reading as *"5."* before *"4."* and left the header. A
Markdown renderer renumbers an ordered list, so round 3's *"reading 5"*
displays as item 4 on a rendered page. No reference inside the document names
reading 4 or 5 (`grep -n -i -E 'readings? [0-9]'` → `:64`, `:67`, `:356`, all
to readings 1–3), so nothing in the document breaks. Fix: *"Five readings"*
and put the items in order.

#### V-3 — the new header sentence is copied from the ledger's prose with its qualifier dropped, and as written the document's own §0 / §0.1 contradict it (class a: transcribed, not derived)

`matrix_P1_threat_state.md:11-12`: *"Every claim in this document's own lines
held on execution in all three rounds."* Rounds 1 and 2 each FAILED the matrix
on claims in its own lines — M1 (*"the edit is printed in
`…_v1.txt`"*, which ended on the line announcing it), M3 (a stated grep scope
that could not produce §1.1's table), R2-M1 (*"gated off in every committed
config"*, false at `ffc5c10`) — and `:63-65`, `:83` record exactly that. What
held in all three rounds is every NUMBER: round 1 *"Every load-bearing number
reproduces"* (`REDTEAM.md:11`), round 2 every round-1 remedy holding, round 3
*"Every remedy in the MATRIX's own lines holds on execution"*
(`REDTEAM_round3.md:26-27`). The sentence
is round 3's own closing prose, `REDTEAM_round3.md:282` *"Every claim in the
matrix's own lines holds on execution"* — said there of round 3's execution,
in the paragraph about where the failing lines are — widened to *"in all three
rounds"*, the same widening `opt_arc_ledger.md:83-84` makes before its
qualifying parenthetical *"(every number reproduced under the reviewers' own
commands three times)"*. A reviewer's sentence copied into the document and
generalized is D-598's class in prose rather than in a figure. Reproducer:
`sed -n 11,12p` against `sed -n 63,65p;83p` of the same file, and
`grep -n -F 'own lines holds on' matrix_P1_threat_state_REDTEAM_round3.md` → `:26`, `:282`. Rated MINOR by this document's
own precedent for a false record claim nothing leans on (R2-m1, R2-m2, R3-m1,
R3-m2): the disposition is the ledger's, and the ledger states the number
ground. Fix: *"Every number in this document's own lines reproduced under the
reviewers' own commands in all three rounds; the MAJORs of rounds 1 and 2 were
record claims, corrected in §0 and §0.1."* Noted, out of subject: the ledger's
F-P1.4 carries the wider phrase too, before its parenthetical.

### Noted, not rated

- `:177` *"2 074 samples"* is not in the cited artifact (which prints
  *"Samples: 2K"*); it is derivable only from the `perf.data` in the
  measurement worktree, which I did (`perf script | wc -l` → 2074) and which
  rounds 1 and 2 did. If the worktree goes, the count is unsourced; the
  percentages, which are the claim, are on the artifact's face.
- `:171` *"(D-477: never inherited)"* — D-477 says a matrix's axis is a
  premise quoted at `file:line` where its unit is consumed (which §2.4 cites
  correctly); *"never inherited"* is the document's gloss on re-taking the
  premise's profile at the package's own SHA, not D-477's words.
- `:34` *"`state.rs:60-91`"* for `touch` — `:60` is blank, `fn touch` begins
  at `:61`.
- `:405` *"26 s and 7 s"* — sourced to round 2, not re-derivable today (§2d).

---

VERIFICATION: PASS — the corrected lines hold and the class sweep found nothing above MINOR
