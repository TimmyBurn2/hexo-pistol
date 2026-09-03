# REVIEW — `docs/experiments/wp21_prereg.md` revision 5. FRESH CONTEXT. ROUND 3 of 5 (D-585).

**NAMED REVISION.** Commit **`c4963b3908a1ff60da96a229b5ed364ce5a02f52`** on `dev`.

**DOES IT STILL MATCH HEAD?** **YES.** `git rev-parse HEAD` =
`c4963b3908a1ff60da96a229b5ed364ce5a02f52` at the end of this review; `git status
--short` shows no tracked file dirty.

**WHAT I READ.** `CLAUDE.md` (Process; rules 1, 3, 6, 8); `docs/process.md`
whole; the document; `wp21_prereg_rev4_REVIEW.md` whole; `arc3_ledger.md` §1c and
§1f–end; `docs/decisions.md` D-537, D-560, D-562, D-563, D-568, D-576, D-581,
D-584, D-586..D-595; `wp21_throughput_prereg.md` revision 4 (§1, §3.2–3.6, §4.4,
§4.5, §5, §7, §7.1, §8); `wp21_label_cache_design.md` revision 10 whole;
`docs/book_v2_ledger.md`; `tools/wp21_tranche_config.py`, `tools/cold_label_check.py`,
`tools/wp21_assemble.py`, `tools/config_check.sh`; the two dry-run logs; the run
directory `/home/tom/pistol-runs/arc3r-dryrun/sweep/` and its driver
`sweep_dryrun.sh` (read-only); `artifacts/wp20pilot_RUN_2cd4f79_v1.txt`,
`artifacts/arc3_leverB_41_count_v3.txt`, `artifacts/arc3_opening_prefix_fold.txt`,
`docs/experiments/wp20_CLOSURE.md` (all present on disk); `wp21_assemble_REVIEW_round2.md`
header; `wp21_label_cache_impl_REVIEW.md` §7–8; `crates/pistol-arena/src/{outpath,conclusion,usage,capture_file,labels_file}.rs`,
`src/bin/arena.rs`; `configs/instrument_v0.toml`, `configs/arena_wp20_label_pilot.toml`;
`tools/SHELL_CHECKLIST.md` item 10.

**WHAT I RAN, AND WHERE.** Everything under `cargo` or a tool ran in my own
detached worktree `/home/tom/pistol-wt/prereg-r3` at `c4963b3` with
`CARGO_TARGET_DIR=/home/tom/pistol-wt/prereg-r3/target` per command, never
exported, never in the live tree: `cargo build --workspace --release --locked`;
`cargo test -p pistol-arena --locked` over the four coverage-rule suites; the
shipped generator over all sixteen tranches and seven refusal edges; my own
`arena` over the dry run's one-opening report at `--label-nodes 2000` (uncached,
cached, a doubled tail); `cold_label_check.py` with my own `pistol` as the referent
at strides 1 and 2. No tranche, no 218-opening play, no registered workload. Counts
were taken with `/usr/bin/grep`, `git grep`, `git show`, `sed`, `awk`, `python3`,
`sha256sum`, `cmp`, `diff`. The worktree is removed at the end.

## VERDICT: **FAIL**

**1 BLOCKING, 8 MAJOR, 7 minor.**

Revision 5 is the first revision of this document that could honestly be reviewed:
every registered command runs, the dry run is taken and recorded verbatim, the
digests reproduce from a fresh build, the partition reproduces from the shipped
generator, and all twenty of round 2's findings have a disposition that closes
them. What fails is narrower than any earlier round and none of it is arithmetic
of consequence. The one BLOCKING is a criterion row that cannot fail (T-E), whose
remedy is a deletion. The MAJORs are: a run-log rule that, read as written, voids
tranche one by construction; a dry-run limb the document's own record fails; a
resume rule with no branch for the interruption the document itself anticipates;
an anchor §8 claims that the tree does not hold; an ADR limb reversed by a
registration rather than by the log; an instrument digest 64 lines past the review
it cites; a criterion whose text and whose commands test different things; and a
registered step-0 script with no digest, no test and a `cargo run` inside it.

---

# 1. DISPOSITION OF ROUND 2

| # | round-2 finding | status in revision 5 | evidence (my command, scope) |
|---|---|---|---|
| **N-B1** | `--label-cache` refused by the binary; flag exists nowhere | **CLOSED** | `sed -n 111,122p crates/pistol-arena/src/usage.rs` → `capture_tail` accepts `["--label-cache"]` as the sole tail word; my own `arena --capture … --label-nodes 2000 --label-cache` over the dry-run report: exit 0, `label cache on: asks 17 records 34 hits 17`, `cmp` identical to my uncached run |
| **N-B2** | dry run registered, not taken | **CLOSED** | `artifacts/arc3r_dryrun_sweep_0c4f3b4_v2.txt` sha256 `95a77271…` matches §9.1; all 22 `$` commands and all 26 quoted printed lines of §9.1 are in the log verbatim (mechanical `grep -F` over the document's lines 472–515 and 521–546: 0 missing). One residual is new: **M2** (limb 5) |
| **N-B3** | §5 resume contradicts §4 re-run-whole | **CLOSED** | §5:291-296 — *"starts at the first tranche that has no PASS verdict — a VOID tranche is an unfinished one and is re-run whole"*; *"~3 hours"* = §3's 3.06 h. One gap is new: **M3** |
| **N-M1** | limb 3 sums strided samples | **CLOSED** | §9.1 limb 3 now reads *"their class counts sum to the record count"*; 17 + 17 = 34, re-derived by my own `awk` first-seen walk over the capture (misses 17, hits 17, total 34) |
| **N-M2** | "~53 hours" underivable | **CLOSED** | ONE LINE reads `~49`; `199027.23 × 0.8854447 / 3600 = 48.95` |
| **N-M3** | T-F capture cost doubled | **CLOSED** | §3:155 `2 × (20 × 57.0769 × 0.885445) = 2 × 1 011 s = 0.56 h`; mine `20 × 742/13 × 657/742 = 1 010.77` |
| **N-M4** | wall omits the §6.1 gate | **CLOSED** | §3:146-149 carries the gated line `27 141 s = 7.54 h` and `NET … 0.20 h`; mine `(2·13927 − (13927+5152+8062))/3600 = 0.198` |
| **N-M5** | 13 928 vs 13 925 | **CLOSED** | one figure, 13 927, in §3; the sibling's §1 reads `13 927 s` (`sed -n 34,36p wp21_throughput_prereg.md`). (Two of its four addends do not reproduce from the printed rates and cancel — **m1**) |
| **N-M6** | sibling's 85.6 % / 3 % | **CLOSED** | sibling §1: `79.1%`, `10.4%`, `10.1%`, `0.4%`; mine `11017/13927 = 0.791`, `1443/13927 = 0.104`, `1409/13927 = 0.101` |
| **N-M7** | four/six/seven inputs | **CLOSED** | one paragraph, *"THE NINE INPUTS"*; I count nine named and nine consumed, 657 among them |
| **N-M8** | cold rate wrong measurement | **CLOSED** | `671 / 742 = 0.904313`, sourced to `cold seconds=671` (pilot log `:67`) |
| **N-M9** | "tens of gigabytes" | **CLOSED** | `~0.2 GB` from per-record sizes; mine from `ls -l /home/tom/pistol-runs/wp20pilot-artifacts/`: 227828/742 = 307 B, 469783/742 = 633 B, 15418/13 = 1 186 B → 0.191 GB; ground is now the reboot |
| **N-M10** | correction to `overnight2_ledger.md` asserted, not made | **CLOSED by withdrawal** | §preamble: *"is a record, left as written"*; `sed -n 186p overnight2_ledger.md` unchanged; ledger §3 records the withdrawal |
| **N-M11** | T-A2 not in force on uncached tranches | **CLOSED** | T-A2 row: *"In force on all sixteen"*, uncached under T-A1's class |
| **N-M12** | 10-sample floor unreachable for T-F | **CLOSED** | T-A no longer samples T-F's captures (§4:206-208); floor binds a class under 1 801 records, mine `9·200+1 = 1801` < 5 819 |
| **n-m1** | two roundings of one rate | **CLOSED in rule, not in every line** | *"the unrounded quotient, is used in every line"* — two lines are not (**m1**) |
| **n-m2** | ledger row stale | **CLOSED** | `/usr/bin/grep -n 'wp21_prereg' docs/book_v2_ledger.md` → `:42 … revision 5` |
| **n-m3** | `replay.txt` missing from §4.1 | **CLOSED** | §4.1:242 |
| **n-m4** | no commands for `cmp`, cached re-capture, T-F | **CLOSED** | §9:414-424 |
| **n-m5** | "4 hours" | **CLOSED** | §5:294 `~3 hours` |
| structural note | five unlanded amendments | **CLOSED** | `git diff --stat 0c4f3b4 c4963b3 -- crates tools configs Cargo.toml Cargo.lock` is empty; every instrument named exists at the registered digest |

**Score: 20 of 20 CLOSED** (one by withdrawal, one with a residual). Round 2's
class — commands the binary refuses — is gone from this document.

---

# 2. RE-DERIVATION TABLE (my command, its scope, my number, the document's)

| claim | my command (scope) | mine | document's |
|---|---|---|---|
| 13 openings, 26 games, 742 records | `cat -n artifacts/wp20pilot_RUN_2cd4f79_v1.txt` `:17-18`, `:35` (whole log) | `13 openings`, `n 26`, `captured 742` | same |
| capture wall 657, play wall 21 505 ms @4, replay 21, cold 671 | same log `:39`, `:27`, `:72`, `:67` | 657 / 21505 / 21 / 671 | same |
| 347 distinct positions, and NOT in the run log | `/usr/bin/grep -n '347' docs/experiments/wp20_CLOSURE.md` → `:46`, `:50`; `/usr/bin/grep -c 347 artifacts/wp20pilot_RUN_2cd4f79_v1.txt` → 0 | 347; absent from the log | same, *"NOT there"* ✓ |
| hit rate 0.5323 = 395/742 | `cat artifacts/arc3_leverB_41_count_v3.txt`; `sha256sum` → `cbad0786…` | 395 hits, 0.5323, digest matches §8 | same |
| the seven rates | `python3` from the raw integers | 57.07692, 26.69231, 0.8854447, 0.9043127, 0.8271154, 0.8076923, 0.532345 | 57.0769, 26.6923, 0.885445, 0.904313, 0.827115, 0.807692, 0.5323 ✓ |
| games / records / distinct | `3487·2`, `3487·742/13`, `3487·347/13` | 6 974 / 199 027.2 / 93 076.1 | 6 974 / 199 027 / 93 076 ✓ |
| capture per tranche | `12443 · 657/742` | **11 017.59 → 11 018** | **11 017** ✗ (m1) |
| play per tranche | `436 · 21.505/26 · 4` | **1 442.49 → 1 442** | **1 443** ✗ (m1) |
| replay per tranche | `436 · 21/26 · 4` | 1 408.6 → 1 409 | 1 409 ✓ |
| cold per tranche | `ceil(6624/200)+ceil(5819/200)`, `64 · 671/742` | 34 + 30 = 64; 57.9 → 58 | 34 + 30; 58 ✓ |
| tranche total | sum of the four | 11018+1442+1409+58 = **13 927** | 13 927 ✓ (the two errors cancel) |
| hits per tranche | `12443 − 5819` | 6 624 | 6 624 ✓ |
| labels searched | `12443 · (1 − 0.5323)`; `12443 · 347/742` | 5 819.59 → 5 820 at the printed rate; 5 819.03 exact | 5 819 (✓ at the exact quotient only, m1) |
| cached capture, cached tranche | `5819 · 657/742`; `13927 − 11017 + 5152` | 5 152.4; 8 062 | 5 152; 8 062 ✓ |
| the three walls and the net | python | 27 854 s 7.737 h; 21 989 s 6.108 h; 27 141 s 7.539 h; 0.198 h | 7.74 / 6.11 / 7.54 / 0.20 ✓ |
| serial | `199027 · 0.885445`; `3487·57.0769·0.885445`; exact `3487·657/13` | **176 227.46**; 176 227.60; 176 227.62 | line 117 says `199 027 × 0.885445 = 176 228` ✗; line 150 `= 176 228` ✓ (m1) |
| T-F surcharge | `2·20·742/13·657/742`; `2·40·21.505/26·4`; sum | 2 021.5 s = 0.56 h; 264.7 s = 0.07 h; 1.43+0.56+0.07 = 2.06 h; outside 0.635 h | 0.56 / 0.07 / 2.06 / 0.63 ✓ |
| sweep output size | python over the pilot's byte sizes | 0.191 GB | ~0.2 GB ✓ |
| 792 / 93 076 | python | 0.851 % | D-586's 0.85 % ✓ |
| partition, all sixteen | the shipped generator in my worktree, then `awk` over the WRITTEN files (not the script's print) | takes sum 3 487; contiguous; last end 3 500; 15×218 + 217 | §2 ✓ |
| holdout refused | `--tranche 17`, `--tranche 0`, `--skip 3490 --take 20`, `--skip 3499 --take 2` | all REFUSED exit 1; `--skip 3499 --take 1` accepted (the last legal opening) | §2 *"both forms refuse the holdout"* ✓ |
| pilot-range edge | `--skip 12 --take 2 --pilot-range` / `--skip 0 --take 13 --pilot-range` | refused / accepted | generator header ✓ |
| book size | `/usr/bin/grep -vc '^#' crates/pistol-cli/tests/fixtures/random_openings_v2.txt` | 4 500 | 4 500 ✓ |
| §8 tree slot | `git diff --stat 0c4f3b4 c4963b3` (no path filter) | three `docs/` files only | *"docs-only above it"* ✓ |
| §8 the four script digests at `c4963b3` | `git show c4963b3:tools/<f> \| sha256sum`, and at `0c4f3b4`, and on disk | `6386d6bf…`, `707acacc…`, `1a890b53…`, `a367d847…` — identical at both commits and on disk | §8 ✓ |
| §8 the three binaries | **my own** `cargo build --workspace --release --locked` in `/home/tom/pistol-wt/prereg-r3` (fresh `target/`, different path from the live tree) | `pistol 78a7600a…`, `arena a1a405cb…`, `corpus-check efbb76b6…` | §8 ✓ — **byte-identical; the release build is reproducible on this box** |
| `rustc`/`cargo` | `rustc --version; cargo --version` | `1.98.0 (88d9e12ae 2026-08-18)` / `1.98.0 (797e8a9bc 2026-08-05)` | §8 ✓ |
| fold-file digest anchored | `sha256sum artifacts/arc3_opening_prefix_fold.txt` → `b6d4751e…`; `/usr/bin/grep -rn b6d4751e docs/` → **nothing**; `/usr/bin/grep -n -B2 -A2 opening_prefix_fold docs/experiments/arc3_ledger.md \| grep -o '[0-9a-f]{64}'` → **nothing** | not anchored | §8 *"sha-anchored in arc3_ledger.md"* ✗ (M4) |
| `conclusion.rs:81`, `:111` | `sed -n 78,84p;108,114p` | `:81` `"counts n {} … forfeits {} decided {}"`, `:111` `"first_player_wins {} of {} decided_non_forfeit forfeits {}…"`; both lines present in the dry run's `report.txt` (`:41`, `:44`) | T-C ✓ |
| `outpath.rs:6-25` | `sed -n 1,28p` | `:6` doc comment, `:10` `pub fn claim`, `:13` `.create_new(true)`, `:25` `}` | §4.1 ✓ |
| `arena.rs:52-59`, `usage.rs:111` | `sed -n 45,65p`; `sed -n 100,125p` | `:52` `[` … `:59` `] => {` with `tail @ ..`; `:111` `pub fn capture_tail` | §9 ✓ |
| `instrument_v0.toml:113` | `sed -n 110,115p` | `on_search_path = false` | §1 ✓ |
| manifest rows unconditional | `/usr/bin/grep -n 'capture_manifest\|corpus_manifest' src/*.rs` | `capture_file.rs:124`, `labels_file.rs:152`, no branch | (B1) |
| body digest verified elsewhere | `cold_label_check.py` `body_of` (`:131-151`); `labels_file.rs:196-203` | both refuse a body that does not digest to its header | (B1) |
| tests drive the shipped scripts | `git grep -n 'tools/wp21_tranche_config.py\|tools/cold_label_check.py\|tools/wp21_assemble.py\|tools/label_cache_count.py\|tools/config_check.sh' -- crates` | 4 scripts hit via `repo().join(…)`; **`config_check.sh` 0 hits** | coverage ✓ for four, ✗ for the fifth (M8) |
| those suites pass at `c4963b3` | `cargo test -p pistol-arena --locked --test …` (my worktree) | `wp21_tranche_config_tests` 18/18, `cold_label_check_tests` 11/11, `wp21_assemble_tests` 11/11, `label_cache_count_tests` 5/5 | — |
| cross-build capture identity | my `arena` over the dry run's report, records `diff`ed against the recorded `capture.txt` (headers carry the path) | identical; body sha `ec175770…` both | — |
| cold check with an independent referent | `cold_label_check.py … --binary <my pistol> --stride 1 --partition misses/hits` | 17 of 17 / 17 of 17 agree, exit 0; at `--stride 2` (9 samples) **VOID exit 2** naming the floor | §4's floor ✓ |
| the two dry-run logs' digests | `sha256sum` | v2 `95a77271…` ✓; v1 `757cd5fb…` | §9.1 ✓ |
| dry run before the review | log head `06:45:40 UTC`; `git log -1 --date=iso c4963b3` → `08:47:54 +0200` | 2 min before the commit | §9.1 *"before this revision's review was dispatched"* ✓ |
| assembler review revision vs registered digest | `sed -n 1,12p wp21_assemble_REVIEW_round2.md`; `git diff --stat 9c4366c 0c4f3b4 -- tools/wp21_assemble.py` | reviewed at `9c4366c` (script `8dfdb19c…`); **64 lines changed** at `5b17132` → `a367d847…` | §8 *"reviewed twice … PASS with its minors landed"* (M6) |
| "census OFF" has an ADR line | `/usr/bin/grep -n 'census OFF' docs/decisions.md`; `/usr/bin/grep -rln 'census OFF' docs/` | **0** in the log; the phrase exists only in this document, two reviews, and `wp20b_impl.md` | (M5) |
| sibling's field divides 16 | `sed -n 161,187p wp21_throughput_prereg.md` | `{1, 2, 4, 8, 16}` | §1 ✓ |
| sibling §3.4 fills the slot | `sed -n 188,205p` | *"fills wp21_prereg.md §1's slot … reopens … if the answer is not the incumbent 8"* | §1 ✓ |
| §4.1's registered files vs limb 5's listing | `sed -n 237,254p wp21_prereg.md \| grep -o '<SWEEP_DIR>/…'` against the log's listing | listing has `forms/arena_tf.toml`, `forms/arena_tranche-1.toml` (not in §4.1) and lacks `run_log.txt` (in §4.1) | limb 5 *"exactly … and no others"* ✗ (M2) |

---

# 3. FINDINGS

## BLOCKING

### B1 — T-E is a criterion that cannot fail: the row it demands is printed unconditionally by any pass that exits 0, and the binding it names is already verified by T-A's instrument and T-D's loader.

§4, T-E: *"a `capture_manifest` row and a `corpus_manifest` row, each carrying
`body_sha256`"*, defect *"an artifact nothing binds (rule 8, D-469)"*.

`crates/pistol-arena/src/capture_file.rs:124` and `labels_file.rs:152` write the
row with no branch around it; a capture or corpus pass that exits 0 has printed
it. The only way the row is absent is that the pass failed, which T-D and the
exit lines already catch. And the property the row attests — the body digests to
its header — is what `tools/cold_label_check.py`'s `body_of` (`:131-151`) refuses
as a VOID before sampling, and what `labels_file.rs:196-203` refuses before
`corpus-check` says `ok`. So on the data T-E can be neither falsified nor
falsified independently: it is *"output shape"* in `docs/process.md`'s own list
of properties a defect class preserves, and under CLAUDE.md's overrule test its
presence and absence license the same conclusion.

**FIX (executed as a reading; unexecuted as an edit).** Delete the row. The two
manifest rows are what §5's run-log block records — *"its printed manifest row"* —
and that sentence already binds them. Nothing in the sweep's verdicts changes.

## MAJOR

### M1 — §5's run-log rule, read as written, VOIDS tranche one by construction, because the comparison's own input is a command carrying `--label-cache` that necessarily precedes the `cmp -s` line.

§5:286-289: *"The `cmp -s` exit line of §6.1's comparison … appears in the run
log BEFORE the first block whose command carries `--label-cache`; a block
carrying the flag with no such line above it is a VOID tranche."*

§9's *"tranche one only"* block runs `arena --capture … capture-cached.txt
--label-nodes 400000 --label-cache` and THEN `cmp -s capture.txt
capture-cached.txt`. §5 says every pass is logged *"with its command VERBATIM"* in
the tranche's block. Tranche one's block therefore carries the flag, and the
`cmp -s` line cannot be above it — it is computed from that command's output.
Tranche one is VOID under the rule that exists to protect it. The sibling's §4.4
restates the same checker in the same words, so the defect is in both.

**FIX (unexecuted, prose).** Key the rule on the pass that writes `capture.txt`:
*"before the first block whose PASS-2 command carries `--label-cache`; tranche
one's `capture-cached.txt` re-capture is the comparison's own referent and is
named here as the one flagged command that precedes the line."*

### M2 — §9.1 limb 5 fails on the document's own record, and the ledger asserts the opposite.

Limb 5: *"the whole sequence leaves exactly the files §4.1 registers and no
others."* The recorded listing (`:553-566`) contains `./forms/arena_tf.toml` and
`./forms/arena_tranche-1.toml`; §4.1 registers no `forms/` path (my extraction of
every `<SWEEP_DIR>/…` token in `:237-254`). And §4.1 registers `run_log.txt`,
which the listing lacks (the dry run's log lives in `artifacts/`). *"Exactly"*
fails in both directions. `arc3_ledger.md` §3 says *"limb 5's listing exactly
§4.1's files"* — a claim the tree contradicts. The document half-knows it
(`:468-469`, *"the `forms/` pair … never played"*) and registers it anyway.

**FIX (unexecuted, prose).** Either register `<DRY>/forms/` in §9.1 as the dry
run's own validation directory and restate limb 5 as *"exactly the files §4.1
registers plus `forms/`, and no `run_log.txt` because the log is the artifact"*,
or restate limb 5 over the files §9.1's commands name. And correct the ledger.

### M3 — §5's resume rule has no disposition for an INTERRUPTED tranche, the case §4.1 itself anticipates when it puts the sweep on `/home` because it *"spans days"*.

§5: a successor *"starts at the first tranche that has no PASS verdict — a VOID
tranche is an unfinished one and is re-run whole under its next `-run<k>`"*. A
tranche that was killed mid-capture (reboot, OOM, an operator's Ctrl-C) has no
verdict and is not VOID — no criterion failed. Its directory holds `report.txt`
(claimed, complete) and a partial `capture.txt` (claimed under `O_EXCL`,
`outpath.rs:13`). The successor following §5 re-launches it in place; pass 1
refuses at `claim` before any game (*"a run does not overwrite a previous
report"*), and the rule says nothing about what happens next. The refusal is loud
(rule 3), so nothing wrong is written — but the resume point, which §5 says is
the only place per-tranche state lives, does not resolve the one case that a
days-long detached sweep will actually produce.

**FIX (unexecuted, prose).** *"A tranche with no PASS and no VOID whose directory
holds any claimed file is INTERRUPTED and is treated as VOID: re-run whole under
`-run<k>`, the partial directory kept."*

### M4 — §8 says the fold receipt is *"sha-anchored in `arc3_ledger.md`"*. It is not anchored anywhere in the tree.

`sha256sum artifacts/arc3_opening_prefix_fold.txt` → `b6d4751e24e3594d7da827687bc3a35630c3b5052c6e6cd8f3d260076e3e97de`.
`/usr/bin/grep -rn b6d4751e docs/` → no output. The ledger's only mention
(`:595`, *"Receipt `artifacts/arc3_opening_prefix_fold.txt`; matrix revision 3"*)
carries no digest; neither does D-584 nor D-586. The file is gitignored and was
amended after its first text (its own `ADDENDUM` is timestamped 20:45, the head
19:30), so the anchor matters: this is the file §5 says each cached tranche's
`key_full_collisions` is read against, and the reading it feeds is D-586's flip
clause. The sixteen floors themselves ARE in the tree — D-584's text and the
design's §2.4 transcribe `42 51 48 55 47 48 50 44 53 48 51 60 45 56 48 46, sum
792` — so the number a reader needs is recoverable; the claim about where its
digest lives is false.

**FIX (executed as a digest; unexecuted as an edit).** §8's row: *"sha256
`b6d4751e24e3594d7da827687bc3a35630c3b5052c6e6cd8f3d260076e3e97de` (taken at this
revision); the sixteen floors are transcribed in D-584"*. Drop *"sha-anchored in
`arc3_ledger.md`"*.

### M5 — "CENSUS OFF" reverses limb (3) of D-562 on the authority of a dispatch the tree does not hold, and no ADR line records it. Hard rule 10.

Preamble: *"THE CENSUS IS OFF FOR THIS SWEEP, by the dispatch's own words — 'gates
OFF, census OFF'. This supersedes D-562(3)'s 'census ON from game one' for this
run only."* D-562(3) is an operator ruling in the log. The arc III GROUNDWORK
dispatch is not in `docs/` (its title is quoted in the ledger's head; its §6 text
is quoted only here). `/usr/bin/grep -n 'census OFF' docs/decisions.md` → nothing;
the phrase occurs in this document, two reviews and `wp20b_impl.md`. The grounds
exist and are strong — D-563 shows the token on this seat records zero firings,
and the cache's X1 (`usage.rs:116-119`) refuses `--census` with `--label-cache`,
so a cached tranche cannot carry a census at all — but grounds are what an ADR
line carries, not a substitute for one. CLAUDE.md: *"Silent architecture drift is
a breach; amend the ADR instead"*, and *"The operator's answer is an ADR line"*.
R1–R7 of the same dispatch each got a line (D-588..D-594); this ruling did not.

**FIX (unexecuted).** One ADR line: the sweep runs census OFF — D-563's zero yield
on the committed seat and X1's refusal under the cache — superseding D-562(3) for
this registration; flips when a census-capable seat is registered.

### M6 — §8 registers the assembler at a digest 64 lines past the revision its cited PASS reviewed, and calls it *"reviewed twice"*.

`wp21_assemble_REVIEW_round2.md` header: reviewed at `9c4366c`, script sha
`8dfdb19c…`. `git diff --stat 9c4366c 0c4f3b4 -- tools/wp21_assemble.py` → 64
lines changed (commit `5b17132`, *"the assembler's round-2 minors land"*); the
registered digest `a367d847…` is that later file. `docs/process.md`: *"a change
to it reopens the review exactly as an amendment to the document does."* The
ledger §2.4 says *"Round 3 is not owed on a PASS"*, which is a sentence, not a
rule the tree holds. Mitigation, and it is real: the patch is the reviewer's own
executed 30-line remedy plus two tests, the suite is 11 of 11 in my worktree,
and the assembler gates §6's closure numbers, not tranche one.

**FIX (unexecuted).** Either a scoped diff review of `9c4366c..5b17132` over the
script and its suite before §6 runs, or §8's row stated as it is: *"PASS at
`9c4366c`; this digest is `5b17132`'s, the reviewer's own minors applied and run
(ledger §2.4), not re-reviewed as a whole"*.

### M7 — T-F's criterion text and T-F's commands test different things.

§4, T-F: *"a second capture over a registered sub-range of tranche one's report
is byte-identical to the first over the same range"*. §9 runs a SEPARATE play
pass over openings 13..32 (`tf/report.txt`), captures it twice, and compares
`capture-a` with `capture-b`. Neither file is a capture of tranche one's report,
and nothing is compared with *"the first"* (tranche one's `capture.txt`, which
covers 218 openings and could not be `cmp`'d against a 20-opening file anyway —
its header names the source digest). What §9 tests is *"two captures of one
report agree"* at 20 openings, which is a fine criterion for the class named; it
is not the one written.

**FIX (executed on the stand-in; unexecuted as an edit).** Reword T-F to what §9
runs. And the stronger, free check is available: since play is deterministic,
`tf/report.txt`'s games are tranche one's first forty, so T-F's RECORDS should
equal the corresponding records of tranche one's `capture.txt` (headers aside) —
a cross-report referent rather than a self-agreement. On the dry run's stand-in
the two bodies are identical: `tf/capture-a.txt` and `tranche-1/capture.txt`
both digest to `ec175770…` (`grep -v '^#' | sha256sum`, both files).

### M8 — `tools/config_check.sh` is a registered step-0 command and §9.1 limb 2's acceptance, absent from §8, unpinned, untested, and it runs `cargo` from the live tree.

§9 step 0: `tools/config_check.sh <…>/arena_tranche-<n>.toml` — *"ACCEPTED by
the schema gate 6 runs"*. `tools/config_check.sh:113`: `cargo run --quiet --locked
--package pistol-arena --example validate_arena_config` — the verdict is computed
by whatever the live tree's `pistol-arena` is when the command is typed, not by
`0c4f3b4`. §8 lists no digest for it (`4eeba224…` at `c4963b3`, mine). `git grep
config_check.sh -- crates` → no test drives it (checklist item 10). And §3 says
T-F's pair *"runs beside wave one"*, so its step 0 runs `cargo` beside eight
tranches, against §9's *"no cargo … a concurrent job voids the contention
measurement wave one is read for."* The first dry run also showed the two checks
are not the same check: the arena ACCEPTED the config that this script REFUSED
(a basename classification, `config_check.sh:58-60`), so the script's verdict
adds a naming convention and nothing about the schema the arena enforces itself.

**FIX (unexecuted).** Either delete the line from the registered block — the
arena's own `deny_unknown_fields` parse is the acceptance, and D-424 says a
check that changes no reading is prose — keeping it as the operator's preflight;
or pin it (digest in §8, run only before a wave launches, never beside one).

## minor

**m1 — four §3 lines do not reproduce from the rates printed beside them.**
`12 443 × 0.885445 = 11 017` is 11 017.59 (→ 11 018); `436 × 0.827115 × 4 = 1 443`
is 1 442.49 (→ 1 442); the two cancel, so 13 927 stands. `199 027 × 0.885445 =
176 228` (line 117) is 176 227.46; the same figure on line 150,
`3 487 × 57.0769 × 0.885445`, is 176 227.60 → 176 228, so one of the two spellings
reproduces. `12 443 × (1 − 0.5323) = 5 819` is 5 819.59 → 5 820 at the printed
rate; the *"as it must"* equality with 218 × 26.6923 holds only at the exact
395/742. Seconds, all of it; reported because §3's own thesis is that every
printed integer reproduces from the rate beside it.

**m2 — §4 names the wrong function.** *"`records_of()` voids an empty class"* —
`records_of` (`cold_label_check.py:165-180`) voids an empty CAPTURE;
`partitioned()` (`:183-203`) voids an empty class.

**m3 — an instrument VOID voids a 3.87-hour tranche.** *"a void T-A voids the
tranche"* — `cold_label_check.py` exits 2 for an engine timeout (`--timeout-s`,
default 600) or a spawn failure as well as for the floor. The capture is
unchanged by any of these; re-running the check is not repairing the tranche.
Register: a T-A VOID that names the floor voids the tranche; a T-A VOID that
names the engine is re-taken over the same capture.

**m4 — D-423 inside and across the pair.** The gating rule *"no tranche runs
cached until the comparison returns"* is stated in §1's cache row, §5, §6.1
(three clauses, *"repeated here only as a pointer"*) and §9's comment. §6.1's
*"IF THE COMPARISON FAILS"* paragraph restates the sibling §4.4's registered
consequence and its two-uncached-runs diagnosis. The sibling's §4.4 restates §5's
checker. State each once; point from the rest.

**m5 — D-424, prose that constrains nothing, named for deletion.** (a) *"Tranches
are launched detached (`setsid nohup`) and polled with `ps`, never watched"* —
operational guidance, changes no reading. (b) §4's *"the check is not weaker than
the one the pilot passed at any take above thirteen"* — licenses nothing the
first sentence does not. (c) §4.1's size estimate, now that the `/home` ground is
the reboot — the number does no work.

**m6 — the registration schedules the job it forbids.** §3: T-F's pair (0.63 h)
*"runs beside wave one"*. §9: *"THE BOX IS OTHERWISE IDLE DURING A TRANCHE … a
concurrent job voids the contention measurement wave one is read for."* Two
arena processes and (M8) a `cargo run` beside wave one. Say wave one's contention
reading is taken with T-F beside it, or move T-F after wave one (+0.63 h).

**m7 — limb 3 is internal agreement with its external referent one line away.**
Both class counts come from one script over one input; a misclassification
preserves the sum. The arena's own memo prints the same partition independently
(`asks 17 … hits 17` on the cached line) and the criterion does not read it.
Register `MISSES == asks` and `HITS == hits` on cached tranches; on the record
17 = 17 and 17 = 17, and my `awk` first-seen walk agrees.

---

# 4. WHAT SURVIVED ATTACK

**The digests are the build's, and the build is reproducible.** A fresh worktree
at `c4963b3` (docs-only above `0c4f3b4`; `git diff --stat` between them with no
path filter lists three `docs/` files), a different absolute path, its own
`target/`, `cargo build --workspace --release --locked` under the same rustc:
`pistol 78a7600a…`, `arena a1a405cb…`, `corpus-check efbb76b6…` — byte-identical
to §8 and to the live tree's binaries. The only `env!` in non-test code is
`CARGO_PKG_VERSION`, which is path-free. Item 5 of the brief's question — *"is a
release build reproducible on this box"* — is answered yes, by measurement.

**The partition and the holdout reproduce from the shipped generator**, and I
read the WRITTEN files rather than the script's print: sixteen configs, takes sum
3 487, each skip equals the previous end, last end 3 500. Seven refusal edges
behave as §2 says, including the last legal window (`skip 3499 take 1` accepted,
`take 2` refused by name).

**Every quoted line of §9.1 is in the log**, mechanically: 22 of 22 commands
(`<DRY>` substituted) and 26 of 26 printed lines by `grep -F`, the head line
verbatim, the file listing `diff`-identical, the log's digest as stated. 22
`exit=` lines, all `0`.

**The dry run is of the same KIND.** A real opening from the book (opening 0, the
pilot's consumed range, re-read under D-539), the sweep's own seat and game
budget (`nodes 50000`, `configs/instrument_v0.toml`), both seats one engine, the
same three passes, the same checkers, at `--label-nodes 2000` and stride 1. It
exercised attribution, not syntax: the arena's cache produced hits (17 of 34),
the checker's partition classified them, the fresh-process referent was spawned
34 times. Its first failure reads as a registration defect and not as a command
edited until it ran: the arena command was identical in both runs and passed
both; what changed was the registered path's basename and the addition of a
registered step, and the v1 log shows the arena accepting the very file the
validator refused.

**T-A1 and T-A2 are real criteria with an external referent** — one fresh process
per sample, no table shared, and my own independently built `pistol` reproduces
all 34 records of the recorded capture byte for byte. The ten-sample floor fires
(stride 2 over 17 → VOID naming the floor).

**T-C's three readings exist**: `counts … forfeits 0 decided 2` and
`first_player_wins … forfeits 0` in the dry run's `report.txt` at the cited
`conclusion.rs` lines, and `end 1 (normal)` on `corpus_check`'s line.

**The nine inputs are where the document says**, and 347 is NOT in the run log —
I checked for the number in the log and found it only in the closure.

**Every `file:line` citation opened is correct**: `outpath.rs:6-25`,
`arena.rs:52-59`, `usage.rs:111`, `instrument_v0.toml:113`, `conclusion.rs:81`,
`:111`.

**The sibling is consistent with this document where the two touch**: §1's
percentages at 13 927; the field `{1, 2, 4, 8, 16}`; §3.4's slot-filling rule and
its reopening clause; §4.4's uncached-referent, single-byte-abandon and
two-uncached-runs rules match §6.1; §5's *"~0.20 h net"* matches §3's wall.

**The coverage rule holds for the four instruments that produce a registered
number**: each has a suite driving the shipped script via `repo().join(…)`, and
all four suites pass in my worktree (18, 11, 11, 5).

**The void rules are the dispatch's and are consistent with each other**: VOID
whole, re-run whole under `-run<k>`, artifacts kept, shortfall reported against
93 076; two consecutive VOIDs stop, with *"in the run log"* meaning completion
order, which is unambiguous because blocks are appended at completion. Every
second write has its own path (`capture-cached.txt`, `capture-{a,b}.txt`,
`-run<k>/`), which `outpath.rs:13`'s `create_new` makes necessary.

---

# 5. ATTACKS I ATTEMPTED AND REJECTED

1. **"A release build cannot be reproduced, so the digests are unverifiable."**
   Rejected by building it: identical digests from a different path.
2. **"The first dry run was a command edited until it ran."** Rejected — above;
   the edit was to the registered path and added a registered step, and the
   command that had failed was the validator, whose refusal was a basename rule.
3. **"5 819 via 0.5323 is not 5 819, so the cached block's cross-check fails."**
   Rejected as a defect of the cross-check: at 395/742 both routes give 5 819.03.
   Kept as part of m1.
4. **"T-A2 on an uncached tranche is vacuous because a hit's bytes are the
   miss's."** Rejected — the referent is a fresh process, not the miss record; a
   second in-process ask that drifted would disagree with the referent.
5. **"T-F re-reads openings the ledger says are consumed."** Rejected — the same
   registration consumed them, and T-F yields no corpus.
6. **"The ten-sample floor will void a full tranche at stride 200."** Rejected —
   1 801 records suffice and the smaller class is ESTIMATED 5 819.
7. **"`n_workers = 1` is not the pilot's seat."** Rejected — §1 discloses it and
   §3's `×4` corrects the two per-game rates for it.
8. **"The slot clause in §1 can never fire."** Rejected — the sibling's §3.4
   fills it and names the reopening.
9. **"The two counters have no floor for `key_pos`."** Rejected — the fold
   receipt's `k = 2` stone-set classes equal the exact classes (213 = 213), so the
   floor is zero and D-586 reads only `key_full`.
10. **"`hits 6 624` should be sampled at stride 200 giving 33."** Rejected —
    indices 0, 200, …, 6 600 are 34 samples; `floor(6623/200) + 1 = 34`.
11. **"The generator's `--pilot-range` form lets a dry run reach opening 13."**
    Rejected — `skip 12 take 2` refused by name.
12. **"`cmp -s` in §9 is the wrong instrument because the two captures carry
    their own path in the header."** Rejected — the `path` is on the printed
    manifest row, not in the file; the recorded `capture.txt` and
    `capture-cached.txt` are `cmp`-identical and my two are too.

---

**Worktree.** `/home/tom/pistol-wt/prereg-r3` removed at the end of this review;
nothing gitignored in it needed exporting (scratch outputs of a toy run only).
