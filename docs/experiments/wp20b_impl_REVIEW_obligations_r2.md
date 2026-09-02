# WP-2.0b REVIEW-impl (OBLIGATIONS) — ROUND 2. The documents, the registrations, the receipts.

**NAMED REVISION:** `eff179bc5b9b77ad03fbab312e31ea87ed9be0f1`, a `git stash create`
object holding the uncommitted work on `dev` at HEAD `a6777f4`.

**DOES IT MATCH THE WORKTREE?** **YES.** `git add -A && git stash create` produced
`5ede9feb302c107f57adcd4a9583649647989b93` and
`git diff eff179b 5ede9fe --stat` is EMPTY. It has not moved. (Writing this report is
the first change; after it lands the worktree differs from `eff179b` by this file
alone.)

**VERDICT: FAIL.** Five BLOCKING, seven MAJOR, two minor, two REJECTED.

**MAY `wp20b_impl.md` §3 GOVERN THE PERF GUARD'S RE-RUN? NO — NOT AS WRITTEN.** The
registered estimator is not the one the instrument computes; the decision rule that
turns twenty per-position ratios into one verdict is not fixed; §3's own
"hold under all three" agreement clause carries no registered consequence, which
`docs/process.md` forbids by name; and that clause's outcome is already computable
from the raw output on disk. Every defect is an edit plus one 25-minute re-run — none
needs a new measurement. The full ruling, with its ten limbs and the shortest list
that would fix it, is **B4** below.

**SCOPE.** Documents, registrations and receipts. A second reviewer holds the code.
Where a test's NAME and FILE discharge a documented obligation I checked that the name
exists where the document says; I did not review what it asserts.

---

## 0. THE HEADLINE, IN THREE SENTENCES

The measurements are right and I re-derived all of them myself — both byte-identity
referents over eight records, all sixteen perf statistics, the four `key_pos` cells and
their 798 firings, every line of `wp21_prereg.md` §3's arithmetic, the sixteen-tranche
partition, and D-537's `n = 28, c = 21` from first principles. The mutation set is in
the best state it has been in: **26 registered, 26 dead at their registered test, 0
alive, 0 harness faults, EXIT=0**, at a revision whose **tree is byte-identical** to the
one under review. What fails is that **no document in the tree knows this** — the
receipt it stands on is named nowhere, three documents name superseded runs instead,
the committed manifest carries a digest for an instrument that does not match, and §3
still cannot govern the run it was written to govern.

---

## 1. ROUND-1 DISPOSITION TABLE

| id | round-1 claim | disposition | the check I ran |
|---|---|---|---|
| **B1** | the arc ledger named a revision not under review and said every receipt was against it | **RE-OPENED WORSE** | The `8efeda0` sentence is gone and a four-revision table with an explicit disclaimer replaces it — that instance is closed. But the new table names **`293ab1f`** for mutation run 3 where the receipt's own header names **`1cd3364`**; `git diff --stat 293ab1f 1cd3364` shows **six source and test files** apart (`stub_engine.rs`, `capture.rs`, `census_file.rs`, `passes.rs`, `census_capture_tests.rs`, `error.rs`). See **B3**. |
| **B2** | the perf guard's amendment governed a run it had no reviewed existence before | **NOT CLOSED** — and this round is the review it was owed | §3 read in full against `docs/process.md` in full. Ruling at **B4**: it may not govern a run as written. The governed run `wp20b_perf_guard_v1.txt` demonstrably predates the amendment — its fifteen `arm` lines run `post_off, post_on, pre_off` in that fixed order every rep, and §3 now registers a rotation. |
| **B3** | §9's `key_pos` obligation undischarged | **CLOSED** | Taken, and I re-derived it. All four cells of the ledger's table reproduce exactly from the raw (below). The map from `key` to `key_pos` is a **bijection** on all 798 rows — not merely equal counts. One overstatement in the conclusion is **M4**, and it does not touch the operative half. |
| **M1** | the reported statistic is not the registered one | **NOT CLOSED, partly re-opened** | §3 now names an estimator — *"the MEDIAN OVER REPS of the per-position paired ratio"* — but `artifacts/wp20b_perf_report.py:54` computes `statistics.median(num[n]) / statistics.median(den[n])`, the **ratio of medians**. I computed both: median-of-ratios gives H1 `median 0.9968 min 0.9883 max 1.0078`; ratio-of-medians gives `0.9979 / 0.9882 / 1.0048`, which is what the receipt and the ledger print. The decision rule over the twenty positions is still unfixed. **B4** limbs 1-2. |
| **M2** | every receipt names a revision whose tree lacks the work | **NOT CLOSED** | `wp20b_identity_RECEIPT.txt:10` still reads *"post-change tree revision (uncommitted work on) a6777f47…"*; `wp20b_perf_RECEIPT.txt:11` still reads *"git HEAD a6777f47…, work UNCOMMITTED"*. Neither receipt changed. Partly mitigated by the ledger's new revision table. See **M3**. |
| **M3** | `wp21_prereg.md` §3 attributed `347 distinct` to an artifact lacking it | **CLOSED** | §3 now separates the four inputs explicitly: *"the 347 distinct positions are NOT [in the run log] — that log reports `distinct-n 13`, which is distinct GAMES"* — and cites `wp20_CLOSURE.md`. Exactly the right fix. |
| **M4** | the registering sentence miscounted the counters ("two", not four) | **CLOSED** | §3:81 now reads *"THE DESIGN'S BLOCK PLUS A THIRD ARM AND FOUR COUNTERS"*, and the bullet lists four. But §3:54's *"unchanged"* survives and now contradicts it — **B4** limb 7. |
| **M5** | the ledger made gate and test claims with no cited log output | **CLOSED as to the unsupported claim** | The rows no longer assert green: they read *"cited at the closure HEAD's CI run, gate 3"* / *"gate 4"*. That is a forward citation to an artifact still owed, which asserts nothing — acceptable. The rule-9 row DOES still quote an output, and that quote no longer reproduces: **M2r2**. |
| **M6** | the amendment changed the run's cost and did not restate it | **CLOSED** | §3:110-115 restates it: 3 arms × 20 × 5 = 300 searches, **MEASURED 22.6 minutes** of search time, ~25 min wall. I summed `time_ms` over all 300 rows: **1 356 330 ms = 22.605 min**. Exact. |
| **M7** | a `tools/` change landed without the `SHELL_CHECKLIST.md` review | **NOT CLOSED**, partially discharged here | My round-2 dispatch also does not cite the checklist. I answered what an obligations reviewer can: items **8, 9, 10, 11** are satisfied; **item 12 limb 3 is NOT** — see **M6r2**. |
| **M8** | the instrument existed only in an ephemeral scratchpad | **RE-OPENED WORSE** (limb 1); **CLOSED** (limb 2) | Limb 2 closed: I ran the exported `wp20b_perf_report.py` on the raw and it reproduces **every number in the receipt**, including the two the old script did not print. Limb 1 worse: the exported guard is a **later revision** than the one that produced the raw, and the receipt's two instrument digests now name files nobody holds — **M1r2**. |
| **m1** | `wp20b_impl.md` had no §5 | **CLOSED** | Headings now run 1, 2, 3, 4, 5, 6 with no gap. A new consequence is **m1r2**. |
| **m2** | the governing design's H1 spelling was uncorrected | **CLOSED** | §3:160-168 is an explicit erratum: *"where they differ THIS document governs"*, with the reason. The right instrument for a landed document. |
| **m3** | AG5's fourth row answered with a §5 pointer | **CLOSED** | The row now reads *"**NO §8 ROW EXISTS**, and §5 says why"* and names the test in its own column. Saying no row exists is the honest answer. The bare `§5` is **m1r2**. |
| **m4** | the ledger's mutation row was stale in both cells | **RE-OPENED WORSE** | It still cites `artifacts/wp20b_mutants_v{1,2,3}.txt` and presents run 3 as *"adds the two REVIEW-impl asked for"*. Runs 4 and 5 exist; run 5 is the one the obligation stands on. **B2r2**, **B3**. |
| **m5** | no D-469 row and no artifact manifest | **CLOSED** | The row exists and `docs/experiments/wp20b_artifacts.md` + `artifacts/wp20b_MANIFEST.txt` are the manifest. Their own defects are **B1**, **B5**, **M5r2**. |
| **m6** | fixed arm order confounds within-rep drift with H1 | **NOT CLOSED** | The exported instrument now rotates (`artifacts/wp20b_perf_guard.sh:48-53`), but the governed run has not been re-taken, and at REPS=5 over a 3-cycle the rotation leaves a residual — **B4** limb 9. |
| **R1** | (rejected) the measured binary is stale | stands rejected | Not re-litigated. |
| **R2** | (rejected) the D-537 minimum could be argued down | stands rejected | I re-derived `n = 28, c = 21` independently; no `n` in 20..27 admits any critical value. |

---

## 2. NUMBERS I RE-DERIVED AND FOUND **CORRECT**

Stated because a verified number is worth as much as a finding. All arithmetic is mine,
from the raw files, not read out of a receipt.

1. **Both byte-identity referents, all eight records.** Applying `wp20b_design.md` §9's
   rule verbatim — `sed -n '1,/^# timing/p' | grep -v '^revision \|^binary_sha256 ' |
   sha256sum` — gives `81e37d420d32e6d9…a98b` for `gate_v0` run 1 and run 2 on **both**
   binaries, and `c7f155e8a6702e5b…b20e` for `instrument_v0` run 1 and run 2 on **both**
   binaries. Exactly the two registered referents. Run 1 = run 2 on each binary, which is
   the check that the rule did not buy satisfiability.
2. **The `key_pos` measurement, every cell.** Parsed all four
   `artifacts/wp20b_keypos_*.txt` myself:

   | fixture / cap | rows | `firings` field | distinct `key` | distinct `key_pos` | keys over >1 pos | pos under >1 key |
   |---|---|---|---|---|---|---|
   | corpus, 2048 | 400 | 400 | 320 | 320 | 0 | 0 |
   | corpus, 16384 | 63 | 63 | 57 | 57 | 0 | 0 |
   | trigger-rich, 2048 | 294 | 294 | 287 | 287 | 0 | 0 |
   | trigger-rich, 16384 | 41 | 41 | 40 | 40 | 0 | 0 |

   Every number in `overnight2_ledger.md` §1's table reproduces, and `400 + 63 + 294 +
   41 = 798` is the stated firing total. I checked the stronger property the ledger
   asserts and it holds: the `key` → `key_pos` relation is a **bijection** on all 798
   rows in both directions, so *"the in-tree symmetry fold merged nothing"* is exactly
   what the data shows and not an artefact of equal counts. **The semantics are the
   right ones**: `crates/pistol-search/src/search.rs:766` and `pvs.rs:639` set
   `key_pos = state.key()` (the incremental position key) while `census.rs:17-20`
   documents `key` as the **both-folded** canonical key — so the comparison measures the
   symmetry fold's yield and nothing else.
3. **All sixteen perf statistics**, recomputed from the 300 raw rows under all three
   estimators. The exported `wp20b_perf_report.py` reproduces the receipt line for line
   (H1 nps `0.9979 / 0.9882 / 1.0048`; ON/OFF nps `1.0012 / 0.9789 / 1.0093`; H1 time
   `1.0004 / 0.9952 / 1.0909`; ON/OFF time `1.0000 / 0.9908 / 1.0078`). Attribution:
   `totals_lines` and `bestmove_lines` 20 on every arm and rep, `empty_board_answers` 0,
   `census_rows` **181 / 0 / 0** identical across all five reps, `REFUSAL` count **0**.
   `depth_turns` is identical across all three arms at every one of the 100 (rep,
   position) cells — the arms really did the same work. Summed `time_ms` = **1 356 330**.
4. **The dry run**, all four criteria from its own raw: 0 refusals; `20 / 20` totals and
   bestmoves on all three arms; 0 empty-board answers; `census_rows` **18 / 0 / 0**.
   The `= 20` referent is **externally derived** — the fixture
   `crates/pistol-cli/tests/fixtures/bench_solver_positions_v1.txt` holds exactly 20
   non-comment non-blank entries, which I counted.
5. **Mutation run 5, and its provenance.** `artifacts/wp20b_mutants_v5.txt` reports
   `26 registered, 26 dead at their registered test, 0 dead elsewhere, 0 alive, 0 harness
   fault(s)`, `EXIT=0`. I tallied it independently: **26** `=== mutant:` headers, **26**
   `-> DIES` lines, **26** `dies AT ITS REGISTERED TEST` lines, **0** `ALIVE` occurrences
   outside the summary, ids covering M1..M26 exactly once each, **9 CALL-REMOVED**
   against §8's registered 7. Its worktree revision `595f004153d9…` resolves to
   **tree `5972488e11f7…` — byte-identical to `eff179b`'s tree**, so the receipt covers
   the code under review exactly, with no argument needed. It completed at **10:36**,
   before this review began, so D-553's *"green BEFORE REVIEW-impl"* is met in fact.
6. **`wp21_prereg.md` §3, every line.** `742/13 = 57.0769`; `347/13 = 26.6923`;
   `742/347 = 2.1383`; `657/742 = 0.8854`; `21.505/26 = 0.8271`; games `8974`; records
   `256 104`; distinct `119 768`; capture `226 766 s = 62.99 h`; per tranche
   `562 / 16 039 / 7 501`, `14 201 s = 3.94 h`, `1 859 s = 0.52 h`, `455 s = 0.13 h`,
   `73 s`. Every total uses the exact fraction rather than the rounded rate printed above
   it, which is what makes them reproduce. The per-tranche sum is **16 589 s = 4.61 h**,
   so *"two waves × ~4.6 h = ~9.2 hours"* is right to both digits.
7. **The sixteen-tranche partition**, by importing the shipped
   `tools/wp21_tranche_config.py` and calling `slice_of` over 1..16: skips
   `13, 294, 575, 856, 1137, 1418, 1699, 1980, 2260, 2540, 2820, 3100, 3380, 3660, 3940,
   4220`, takes `281 × 7` then `280 × 9`, sum **4487**, first **13**, last **4499**.
   I built the covered set and compared it to `set(range(13, 4500))`: **contiguous,
   disjoint and exhaustive**. `crates/pistol-cli/tests/fixtures/random_openings_v2.txt`
   holds **4 500** non-comment non-blank lines, so `0..12` plus `13..4499` exhausts it
   and the ledger's *"the book is now fully claimed"* is TRUE.
8. **D-537's minimum, from first principles.** With `p0 = 8/14`, `p1 = 12/14`, exact
   rational binomial tails, alpha 0.05, power 0.95: the smallest `n` admitting a critical
   `c` with `P(X≥c|p0) ≤ 0.05` and `P(X≥c|p1) ≥ 0.95` is **n = 28, c = 21**, size
   **0.040184**, power **0.962225** — every digit of `overnight2_ledger.md` §4
   reproduced, `c = 21` unique at `n = 28`, and **no `n` in 20..27 admits any `c`**. The
   input cells are the right ones: `matrix_stage3_detector.md:613` reads
   `| trigger-rich | 14 | **0.571** (8 of 14) | 1.000 | **0.857** (12 of 14) |` under
   headers *per-search ceiling* / *aggregate oracle* / *bound over the columns*, and
   `wp20s_design.md` §8 items 2, 3 and 4 name exactly those two. §8's
   *"**WHAT COUNTS AS DISJOINT IS `key_full`**"* ruling, which the ledger's `key_pos`
   paragraph leans on, is verbatim in the landed text.
9. **The manifest, 26 of 27 rows.** `sha256sum -c` over every digest line of
   `artifacts/wp20b_MANIFEST.txt`: **26 OK, 1 FAILED** (`mutants.py` — **B1**). Every
   file named in the committed table `docs/experiments/wp20b_artifacts.md` exists on
   disk.
10. **The band's provenance.** `[0.98, 1.02]` is genuinely pre-run: it is in
    `wp20b_IMPL_FINDINGS.md:13`, committed at `a6777f4` (22:05), and its derivation
    *"sd 0.0075, range 0.987–1.021, so a ±2 % paired band at REPS=5"* is at
    `wp20b_design_rev8_REVIEW.md:598`. Not invented to fit an outcome.
11. **Rules 8 and 9.** `git ls-files artifacts/` → **0** files; no untracked
    non-ignored files anywhere. `tools/file_justification_check.sh` run by me:
    `355 tracked .rs/.sh files, 64 over the cap, all registered in
    docs/rule9_justifications.md (64 entries)`, **exit 0** — the obligation is met (the
    ledger's *quoted* figures are stale, **M2r2**).
12. **D-565 and D-566** are appended to `docs/decisions.md` in the diff, both with
    flip clauses. No closure D-line yet, correctly marked owed.

---

## 3. FINDINGS

### **B1** — the committed manifest names a digest that the file it names does not have

**Claim.** `artifacts/wp20b_MANIFEST.txt:9` —
`882c0f66bdee3eec1913f1bd0a424b9a90863743dea21f39cb1e111de32636be  artifacts/mutants.py`,
under the heading `## the instruments`. And
`docs/experiments/wp20b_artifacts.md:18`: *"A file whose digest disagrees with this
table is not the file the closure read."*

**Where.** `artifacts/wp20b_MANIFEST.txt` line 9.

**FAILURE SCENARIO.** `mutants.py` is the driver that produced **every** mutation
receipt this package cites, including the closing one. A successor verifying the
manifest gets one FAILED line and cannot tell whether the mutation driver was tampered
with, was silently upgraded, or whether the closing receipt was produced by something
else entirely. This is the exact defect the manifest exists to prevent, on the exact
row where it matters most: the manifest names bytes nobody holds. Worse, the true
history is benign and recoverable — the file was fixed at 10:05 for the stale patch
anchor that aborted run 4 — and the manifest destroys that reading by asserting the
old digest as current.

**Check I ran.** `sha256sum -c` over the 27 digest lines: 26 OK, `artifacts/mutants.py:
FAILED`. `sha256sum artifacts/mutants.py` → **`4fdf0edab891315956cba6c7bd8da37119e49b64456e3eb265477927dcbf4962`**. Mtimes settle
the direction: the manifest was taken at **09:37:02**, `mutants.py` was rewritten at
**10:05:47** — the same second as the timestamp of `595f004`, the revision run 5 names —
and run 5 completed at 10:36. The pre-fix driver is **not recoverable**: the scratchpad
copy at
`/tmp/.../scratchpad/mutants.py` is the same 17 134 bytes and the same 10:05 mtime, so
`882c0f66…` names no file on this machine.

---

### **B2** — the mutation receipt the package actually stands on is named in no document, and two documents state the opposite

**Claim.** `docs/experiments/wp20b_impl.md` §5:256 — *"**Twenty mutants**, in a worktree
on `/home`…"*; and §5:275 — *"**Run 2 (`artifacts/wp20b_mutants_v2.txt`, revision
`dfba9d7`) is the receipt this package closes on**"*. Both are FALSE.

**Where.** `docs/experiments/wp20b_impl.md` §5, lines 256 and 275. And by omission:
`overnight2_ledger.md:39` (cites `wp20b_mutants_v{1,2,3}.txt`),
`docs/experiments/wp20b_artifacts.md:35-37` (names v1..v4), and
`artifacts/wp20b_MANIFEST.txt` (digests v1, v2, v3 only).

**FAILURE SCENARIO.** The set is **26** mutants, not twenty, and the receipt is **v5**,
not v2. `grep -rn 'wp20b_mutants_v5\|595f004' docs/` returns **nothing** — the run with
`26 registered, 26 dead at their registered test, 0 alive, 0 harness fault(s), EXIT=0`,
taken at a revision whose tree is byte-identical to the one under review, exists only as
a file on disk. Meanwhile v2 covers a 20-mutant set at `dfba9d7`, a tree six source and
test files short of the current one. A successor closing from these documents cites v2,
claims twenty mutants, and thereby claims **nothing at all** about M21-M26 — which
include the two options-A mutants (`M21`, `M22`, *"the IN-TREE site emits the POSITION
key — the matrix's option A, which F2 forbids"*), the TAB guard, the grammar version,
the streaming digest and the manifest row. The strongest evidence this package has is
the evidence it does not cite. The ledger's own first rule is *"Anything not written
here did not happen."*

**Check I ran.** `/usr/bin/grep -n "M24\|M25\|M26\|v3\|v4\|v5\|Twenty\|mutants_v"` over
`wp20b_impl.md`, `overnight2_ledger.md` and `wp20b_artifacts.md`: the only run numbers
any of them carries are 1, 2, 3 and 4. Tally of v5 given at §2.5 above.
`git rev-parse 595f004^{tree}` = `git rev-parse eff179b^{tree}` = `5972488e11f7…`, and
`git diff --stat 595f004 eff179b` is empty.

---

### **B3** — the ledger's revision table names the wrong revision for run 3, and its last row's verdict points at a row that does not exist

**Claim.** `docs/experiments/overnight2_ledger.md:59` — *"| `293ab1f` | mutation run 3,
`artifacts/wp20b_mutants_v3.txt` … |"*, and `:60` — *"| `0aa0e2e` | mutation run 4,
`artifacts/wp20b_mutants_v4.txt` — M24's test repaired, plus the streaming digest and the
manifest row | **see the row below** |"*.

**Where.** `docs/experiments/overnight2_ledger.md` §1, the four-revision table at lines
54-60 — the table built in this fix round **to close B1**.

**FAILURE SCENARIO.** Two distinct failures in the same table.
*(a)* `artifacts/wp20b_mutants_v3.txt`'s own first line reads `worktree
/home/tom/pistol-runs/wp20bmut  revision 1cd3364dcd1da253bbdb5a23341a122c1b2d56af`. A
successor reproducing run 3 checks out `293ab1f` and gets a tree **six source and test
files** away from the one the receipt was taken at — `stub_engine.rs`, `capture.rs`,
`census_file.rs`, `passes.rs`, `census_capture_tests.rs`, `error.rs`. This is B1's defect
recreated inside B1's own remedy.
*(b)* There **is no row below** `0aa0e2e` — the table ends there, and the paragraph that
follows is about byte-identity. So the verdict of run 4 is recorded nowhere, and the
verdict is not benign: run 4 **ABORTED** at M24 with `PATCH ANCHOR NOT UNIQUE (0):
crates/pistol-arena/src/census_file.rs`, `EXIT=1`, after only 23 of its mutants. A reader
following the pointer finds nothing and is left to assume run 4 succeeded — and run 5,
which is what actually rescued it, has no row at all.

**Check I ran.** `git show -s --format=%ci` and `git rev-parse …^{tree}` on all seven
named revisions: `293ab1f` (01:00:41, tree `7bc27b2ccaca`) and `1cd3364` (01:07:59, tree
`1059f7a7c6ca`) are different commits with different trees;
`git diff --stat 293ab1f 1cd3364` lists the six code paths plus four documents. `tail -20
artifacts/wp20b_mutants_v4.txt` shows the abort and `EXIT=1`; `grep -c '^=== mutant:'`
gives **23**. `grep -n 'see the row below' -A6` on the ledger shows the table ends at
line 60.

---

### **B4** — `wp20b_impl.md` §3 MAY NOT GOVERN A RUN

This is the round-2 review of §3 as a pre-registration, judged against
`docs/process.md` in full, taken **before** the run it would govern. I answer the
dispatch's five questions in order and then state the ruling.

**Is every value fixed?** Almost. Seat, fixture, `REPS=5`, `BUDGET=50000`, the band
`[0.98, 1.02]`, the `0.95` abort and the pre-change binary (by digest `180b4c40…`) are
all fixed. **The post-change binary is not** — §3:124 fixes it as a path,
`POST=target/release/pistol`, with no digest, in a registration whose whole content is a
cross-binary comparison (limb 10).

**Is the estimator named?** Named, but **wrongly** (limbs 1-2).

**Is the rejection band applied to the comparison it was registered for?** Yes — §3
separates H1 (cross-binary `post_off/pre_off`) from the gross-regression abort
(single-binary `post_on/post_off`) cleanly, names each once, and applies the band only to
H1. AG2 and AG3 are properly discharged.

**Can any registered criterion pass vacuously?** **No — and this part is good.** Each of
the four attribution counters names the defect class it excludes, and each could be
falsified by it: `totals_lines`/`bestmove_lines = 20` against the dropped-filter defect,
`empty_board_answers = 0` against the empty-board search, and `census_rows > 0` on the ON
arm against *"an ON arm that paid nothing"* — which is the one that closes the hole from
the side a ratio of 1.000 cannot see. The `= 20` referent is **externally derived** (the
fixture holds exactly 20 entries), which is what `docs/process.md`'s criterion rule looks
for first.

**Is the cost stated?** **Yes**, and MEASURED: 300 searches, 22.6 minutes of search time,
~25 minutes of wall. M6 is properly closed.

**Is the instrument named with its revision?** **No** (limb 5).

#### The ten limbs

1. **The registered estimator is not the one the instrument computes.** §3:105-106
   registers *"the **MEDIAN OVER REPS of the per-position paired ratio**"* — ratio first,
   then median. `artifacts/wp20b_perf_report.py:54` computes
   `statistics.median(num[n]) / statistics.median(den[n])` — median first, then ratio.
   These are different estimators and they do not agree: median-of-ratios gives H1
   `median 0.9968, min 0.9883, max 1.0078`; ratio-of-medians gives `0.9979 / 0.9882 /
   1.0048`, which is what `wp20b_perf_RECEIPT.txt:30` and `overnight2_ledger.md:69`
   print. A run governed by §3 as written would again publish an estimator nobody
   registered — round 1's M1 in a new costume.
2. **The decision rule over the twenty positions is not fixed, and the two candidates
   disagree.** §3 fixes a **per-position** statistic (twenty numbers) and says the band
   *"applies to it"*. It never says whether H1 is rejected when **any** position falls
   outside or when the **aggregate** of the twenty does. The receipt and the ledger both
   took the any-position reading (*"positions OUTSIDE H1's registered band: 0 of 20"*,
   *"all 20 positions sit inside the registered band"*). Under that reading the three
   estimators §3 requires agreement across **do not agree**: 0 of 20 outside
   (median-over-reps), **1 of 20** (mean-over-reps, position 10 at **1.0215**), **7 of
   100** (per-rep unpooled). Under the aggregate reading they all agree (0.9979, 0.9981,
   0.9972 — all inside). §3 does not pick, so its own agreement clause cannot be
   evaluated. Note also that the any-position rule is the wrong one on the band's own
   terms: at the registered `sd = 0.0075`, `P(|x−1| > 0.02) = 0.00766` per position, so
   **P(at least one of 20 outside) = 0.143** — a 14 % false-rejection rate against the
   project's 0.05 convention, and the band's own derivation sample already produced a
   per-position 1.021 (`wp20b_design_rev8_REVIEW.md:598`).
3. **The agreement criterion carries no registered consequence.** §3:108: *"the verdict
   must hold under all three or the disagreement is the finding"*. `docs/process.md`,
   *Cost, replication, and the second instrument*: *"A registered agreement criterion
   carries a REGISTERED CONSEQUENCE: the pre-registration states, before either
   instrument runs, what DISAGREEMENT does to the verdict, or the criterion leaves
   standing the after-the-numbers decision it exists to forbid."* *"The disagreement is
   the finding"* does not say whether H1 is rejected, whether the run is VOID, or whether
   closure is blocked. It is the after-the-numbers decision, named as if it were a rule.
4. **That criterion's outcome is already known.** The disagreement in limb 2 is
   computable today from `artifacts/wp20b_perf_guard_v1.txt`, which is on disk and in the
   manifest. `docs/process.md`'s closing sentence: *"neither catches a run whose answer is
   already known before it is taken — that defect is judged, not checked."* I judge it:
   §3 as written would register a criterion whose failure is already recorded.
5. **The instrument is not named with its revision.** `docs/process.md`: *"An artefact
   that produces a registered number … is named in the pre-registration WITH ITS
   REVISION, and a change to it reopens the review exactly as an amendment to the
   document does."* §3 names no file and no digest. §3:55-57 says its governing revision
   *"is recorded **in the receipt** beside the numbers"* — i.e. after the run, which is
   the one place a pre-registration may not put it. Both files exist and could be named
   now: `artifacts/wp20b_perf_guard.sh` = `4a380ea57e3611961e5bd8c35329fd5a34819367436c81820f0e25515269b423`,
   `artifacts/wp20b_perf_report.py` = `8bf64a3706a1f4b536be638d6c336dfa2128587733c3a473c5881fd39ced6b70`.
6. **The printed "literal" block is not the instrument and cannot run.** §3:118 says
   *"THE INSTRUMENT, literally"*, and gives its reason: *"printed here so it survives a
   scratchpad"*. But the block elides the entire parser as `awk ...`; it omits the arm →
   binary and arm → `go`-line mapping, which is precisely what H1 depends on; and its
   loop body is `for label in $order; do run_arm "$label"; done` against a function its
   own comment declares as `run_arm() { # $1 binary $2 go line $3 label $4 rep`. Run as
   printed, `run_arm post_off` binds `bin=post_off`. The shipped script dispatches through
   a `case` (`wp20b_perf_guard.sh:54-60`). The block therefore defeats the purpose §3
   gives for printing it.
7. **§3 contradicts itself about the instrument's extent.** §3:54: *"The instrument is
   the command block printed in `wp20b_design.md` §9, **unchanged**."* §3:81: *"THE
   INSTRUMENT IS THE DESIGN'S BLOCK **PLUS A THIRD ARM AND FOUR COUNTERS**"* — and it is
   also plus a rotation and plus an estimator. Round 1's M4 fixed the count and left the
   *"unchanged"* standing, so the sentence a reviewer of the amendment reads first still
   denies the amendment.
8. **The dry run does not cover the amended instrument.** `docs/process.md`: *"A
   pre-registration's literal commands are exercised before its review passes."* The dry
   run §3:147-158 relies on is `artifacts/wp20b_perf_dryrun_v2.txt`, whose three `arm`
   lines are `post_off`, `post_on`, `pre_off` at rep 1 — the **fixed** order. The rotation
   is new since. The instrument §3 now registers has never been dry-run.
9. **The rotation does not do what §3 says it does.** §3:97-101: *"THE ARM ORDER ROTATES
   BY REP … which is what makes the pairing a pairing"*; the script's comment adds
   *"gives each arm each position in the sequence"*. At `REPS=5` over a 3-cycle the orders
   used are 0, 1, 2, 0, 1 — so `post_off` occupies slots `[1,3,2,1,3]` (mean **2.000**)
   and `pre_off` slots `[3,2,1,3,2]` (mean **2.200**). H1 keeps a **−0.200-slot** order
   imbalance; it is reduced tenfold from the fixed order's −2, not removed. `REPS=6`
   makes it exactly **0.000**. The claim is true over three reps and false over five, and
   the fix is one character.
10. **The post binary is not identified.** §3 fixes `PRE` by digest and `POST` by path.
    For a re-run at a closure head, which post binary was measured is the whole
    attribution of a cross-binary comparison — and it is round 1's M2 in the one document
    that could still fix it before the run.

#### What is right about §3, said so this is not read as bigger than it is

The band is genuinely pre-run and measured. H1 and the abort are separated and each
named once, which discharges AG2 and AG3. The cost is stated and MEASURED. The four
attribution counters are real criteria against named defect classes, one of them with an
externally derived referent, and none of them can pass vacuously. The refusal-VOIDs rule
is stated. The erratum against the landed design's §9 (§3:160-168) is the correct
instrument for a document that may not be edited after it lands. **Nothing in §3 is
wrong about the engine.** What is wrong is that it cannot decide its own verdict.

#### The shortest list that would make §3 governable

(a) name **one** estimator **and one decision rule** over the twenty positions, and make
the instrument compute that one; (b) give the agreement clause a registered consequence,
or delete it — under D-424 a clause that changes no reading is prose, not a rule; (c)
name both instrument files with the two digests above; (d) delete *"unchanged"* at §3:54
and either print the block truly literally or make the exported file the named authority;
(e) dry-run the rotated instrument; (f) `REPS=6`, or state the residual imbalance; (g)
record the post binary's digest at the run. None of these needs a measurement. Then the
25-minute re-run.

---

### **B5** — the committed manifest offers, as evidence of reproducibility, an act that never happened

**Claim.** `docs/experiments/wp20b_artifacts.md:44-49` — *"`artifacts/pistol_prechange_a56449b`
is a BINARY … It is reproducible from `a56449b` by a release build — **REVIEW-impl round 1
did exactly that for the post-change binary and got `15c94598…` back** — but that is a
rebuild and not a recovery."*

**Where.** `docs/experiments/wp20b_artifacts.md`, the closing section *"The one thing a
successor cannot reproduce from this table"*.

**FAILURE SCENARIO.** Round 1 performed no build of any kind. Its R1 says so in
terms — *"the binary was never rebuilt"* — and its whole argument is that a rebuild was
**not needed**, because the only source change since the 23:55 build was `///` doc
comments, which do not reach codegen. It read the digest of the binary already on disk;
it did not produce `15c94598…` from a checkout. So the one paragraph in the manifest
whose job is to tell a successor how to recover the single irrecoverable artifact cites a
demonstration that does not exist — and cites it for the **post**-change binary while the
paragraph is about the **pre**-change one (`180b4c40…`). A successor who later finds the
rebuild does not reproduce (a toolchain move, a `Cargo.lock` resolution) has been told the
path was already walked, and will look for the defect in the wrong place.

**Check I ran.** `/usr/bin/grep -n -i 'rebuil\|cargo build\|release build\|15c94598'` over
`docs/experiments/wp20b_impl_REVIEW_obligations.md`: five hits, none of them a build. Line
585 reads *"…and the binary was never rebuilt"*; line 598 is the rejection —
*"Doc comments do not reach codegen."* The claim is contradicted by the report it cites.
The remedy is deleting one clause; the surrounding paragraph's point (the digest is what
binds it) survives intact.

---

### **M1r2** — the exported instrument is not the one that produced the raw output the manifest pairs it with, and the receipt's instrument digests name files nobody holds

**Claim.** `docs/experiments/wp20b_artifacts.md:25` — *"`wp20b_perf_guard.sh` | **THE
INSTRUMENT** — the design §9 block plus the third arm and the four counters
`wp20b_impl.md` §3 registers"* — listed eight rows above *"`wp20b_perf_guard_v1.txt` | the
guard's raw per-position output, 300 rows"*. And `wp20b_perf_RECEIPT.txt:8-9`, which names
the instrument as two `/tmp/.../scratchpad/` paths with digests `9ec50053…` and
`0a6a0f87…`.

**Where.** `docs/experiments/wp20b_artifacts.md:25,33`; `artifacts/wp20b_perf_RECEIPT.txt`
lines 8-9.

**FAILURE SCENARIO.** The exported `wp20b_perf_guard.sh` **rotates** the arm order; the raw
output it is listed beside has a **fixed** order in all five reps. So the exported script
could not have produced that file, and a successor who re-runs the manifest's "THE
INSTRUMENT" against the manifest's raw output gets a different arm sequence and cannot
reconcile them. Meanwhile the receipt's own two digests are unrecoverable: both scratchpad
paths were **overwritten in place** (guard at 01:03, report at 00:48) and now hold
`4a380ea5…` and `8bf64a37…`. The instrument that produced the only perf numbers this
package has stated is gone, and M8's remedy — export it — exported its successor under its
name.

**Check I ran.** `grep '^arm ' artifacts/wp20b_perf_guard_v1.txt` → fifteen lines, order
`post_off, post_on, pre_off` in every one of the five reps; `artifacts/wp20b_perf_guard.sh:48-53`
is the `case $(( (rep - 1) % 3 ))` rotation. `sha256sum` on both scratchpad paths returns
`4a380ea5…` and `8bf64a37…`, matching the manifest and **not** the receipt.

**Mitigation, stated because it bounds the finding.** The *analysis* is fully reproducible:
I ran the exported `wp20b_perf_report.py` on the raw and it returns the receipt's numbers
line for line, including the two the old script did not print. This closes round 1's M8
second limb. What is unrecoverable is the *generator*, and since B4 requires the guard to be
re-run anyway, the fix is a sentence saying so rather than an archaeology.

---

### **M2r2** — the ledger's rule-9 row quotes a tool output that no longer reproduces

**Claim.** `overnight2_ledger.md:35` — *"rule 9 | green: `355 tracked .rs/.sh files, **63**
over the cap, all registered … (**63** entries)`, exit 0 | `tools/file_justification_check.sh`'s
own output, reproduced by the obligations reviewer"*.

**Where.** `docs/experiments/overnight2_ledger.md` §1, the rule-9 row.

**FAILURE SCENARIO.** CLAUDE.md's Closure section requires a gate claim to cite the gate's
own log output. This row does exactly that — and the quoted output is not what the gate
prints at the revision under review. A successor re-running it to confirm the ledger gets
different numbers and must decide whether a file lost its justification or the ledger is
stale. The row also names *"the obligations reviewer"* as having reproduced it, which was
true at `f7606cc` and is not true at `eff179b`.

**Check I ran.** `bash tools/file_justification_check.sh` at the reviewed revision:
`355 tracked .rs/.sh files, **64** over the cap, all registered in
docs/rule9_justifications.md (**64** entries)`, exit 0. The obligation is **met** — one
more file crossed the cap and its justification landed with it (the four new entries are
in the diff to `docs/rule9_justifications.md`). Only the quoted figures are wrong.

---

### **M3r2** — both receipts still name a revision whose tree does not contain the work they measured

**Claim.** `artifacts/wp20b_identity_RECEIPT.txt:10` — *"post-change tree revision
(uncommitted work on) a6777f47…"*. `artifacts/wp20b_perf_RECEIPT.txt:10-11` — *"GOVERNING
REVISION of the instrument and of the tree it measured: git HEAD a6777f47…, work
UNCOMMITTED"*.

**Where.** both receipts, header blocks. Round 1's M2, carried unchanged.

**FAILURE SCENARIO.** `a6777f4` is the commit **without** the WP-2.0b diff. CLAUDE.md is
explicit that a named revision is *"a commit SHA, or a `git stash create` SHA where the
work is uncommitted"*; these give the former where only the latter identifies anything, so
a reader wanting to know which tree `15c94598…` was built from is handed a revision at
which it cannot be built. §9's own instruction was *"INSTRUMENT REVISION: the closure HEAD
at which the guard runs, restated in the closure receipt"*.

**Check I ran.** `grep -n -i 'revision' ` on both receipts; neither changed between
`f7606cc` and `eff179b`. **Partly mitigated**: the ledger's new four-revision table now
carries the stash objects, and its paragraph at :62-65 explains which tree the identity,
determinism and perf receipts were taken at. The receipts themselves — the artifacts the
manifest points a successor at — still do not.

---

### **M4r2** — the `key_pos` conclusion overstates what the landed ruling requires, and forecloses the option its own measurement makes live

**Claim.** `overnight2_ledger.md:99-101` — *"**It does not make C2 wrong** — `wp20s_design.md`
§8 rules the denominator is `key_full` and F2 shows both forms the dispatch named are
forbidden by it, so **C2 is the identity the landed ruling requires whatever the fold
yields**"*.

**Where.** `docs/experiments/overnight2_ledger.md` §1, the `key_pos` paragraph.

**FAILURE SCENARIO.** The design's own option matrix (`wp20b_design.md:373`) marks §8
compliance as *"**FAILS** (F2) | **FAILS** (F2) | passes | passes | passes"* — **three** of
five options are §8-compliant: C1, C2 and **D'**. What the landed ruling requires is a
`key_full`-consistent identity, not C2 specifically; C2 was selected among the compliant
three on bytes (8.7 GB against C1's 25.5 GB) and on meaning (D' *"NO — a fourth notion of
sameness"*). That distinction matters precisely now: the matrix's own row calls D'
*"12 transforms, **zero sorts, zero allocations** — cheaper than C2"*, and the measurement
just taken says C2's **22.99 µs a firing** buys a fold that merged nothing on 798 firings.
The one sentence a successor would read before deciding whether that cost is still worth
paying tells them there was never a choice. There was, and it is one row away.

**FAILURE SCENARIO, bounded.** The operative half survives untouched: the red team's
alternative was **option A** (`key_pos`), and F2 forbids A a priori because §8 defines
disjointness rather than measuring it — a zero yield on this sample does not license an
identity that folds only one of the two equivalences. So *"it does not make C2 wrong"* is
**right**; only *"the landed ruling requires C2"* is wrong.

**Check I ran.** Read `wp20b_design.md:348-400` (the matrix, including the §8-compliance
row and the cost/bytes rows) and `:460-490` (the strongest surviving attack and its
answer), and `wp20s_design.md` §8's *"WHAT COUNTS AS DISJOINT IS `key_full`"* paragraph in
full.

---

### **M5r2** — the manifest describes the two failed mutation runs as successes

**Claim.** `docs/experiments/wp20b_artifacts.md:37` — *"`wp20b_mutants_v3.txt`,
`wp20b_mutants_v4.txt` | **the fix round's runs, carrying the mutants REVIEW-impl asked
for**"*.

**Where.** `docs/experiments/wp20b_artifacts.md`, the artifact table.

**FAILURE SCENARIO.** The table's stated job is *"what each artifact is, **and the claim it
carries**"*, and it does this well elsewhere — v1's row says outright *"the one that found a
vacuous test: 19 dead, M10 ALIVE"*. These two rows carry no claim and hide two outcomes: v3
had **M24 ALIVE** (`23 dead, 1 alive, EXIT=1`) and v4 **ABORTED** at M24 after 23 mutants
(`PATCH ANCHOR NOT UNIQUE (0)`, `EXIT=1`). A successor auditing the mutation obligation from
this table sees four artifacts described as if two of them discharged it, and the one that
did (v5) is not in the table at all.

**Check I ran.** `tail`/`grep -c '^=== mutant:'` on v3 (24 mutants, M24 alive, EXIT=1) and
v4 (23 mutants, abort, EXIT=1); the v1 row's own wording read for contrast.

---

### **M6r2** — `tools/SHELL_CHECKLIST.md` is still unanswered by name, and item 12's third limb is unmet

**Claim.** The diff adds `tools/wp21_tranche_config.py` (208 lines, new), which writes the
sixteen production sweep configs and prints each one's digest into the run log.
`docs/process.md`: *"A change under tools/ is reviewed against tools/SHELL_CHECKLIST.md —
the review prompt cites it and the reviewer answers its items by name."*

**Where.** `docs/process.md`, *tools/ review coverage rule*; `tools/wp21_tranche_config.py`;
`crates/pistol-arena/tests/wp21_tranche_config_tests.rs`.

**FAILURE SCENARIO.** Round 1 raised this and my round-2 dispatch also does not cite the
checklist, so the generator that writes sixteen production configs would ship having had
its items answered by nobody. The failure class the checklist exists for is
EXIT-0-WRONG-ANSWER.

**What I discharge here, by item.** **Item 8** (one spelling per number, one refusal per
reason): satisfied — `--tranche 01` is refused by name (*"a tranche number spelled a way
this program will not echo back"*), the range is checked separately, and the digest shape is
its own refusal. **Item 9** (caller-controlled values reaching a record): satisfied for the
digest (`SHA256.match`) and the tranche (int round-trip). **Item 10, THE COVERAGE RULE**:
satisfied, and well — `wp21_tranche_config_tests.rs` drives the **shipped** script via
`Command::new("python3")` in a scratch directory, with control runs beside every refusal, and
`the_printed_digest_is_the_digest_of_the_bytes_that_were_written` compares against an
independently computed `sha256_hex`, which is an externally derived referent. **Item 11**
(caller path feeding an overwrite): satisfied — `open(out, "x")` is race-free and cannot
clobber, the script never `cd`s so a relative `--out` resolves against the caller's cwd, and
there is no `rm` or `mv` anywhere.

**What is NOT satisfied — item 12, limb 3.** *"A test that drives a gate asserts on the code
it expects AND says, in the failure message, what the other codes would have meant."* The
script correctly defines three codes (`0` yes, `1` no, `VOID = 2` no answer taken), but
**every refusal test asserts `assert!(!output.status.success())`** —
`a_tranche_outside_the_partition_is_refused_by_name:121`,
`a_tranche_number_spelled_a_way_this_program_will_not_echo_back_is_refused:135`,
`a_binary_digest_that_is_not_one_is_refused_before_anything_is_written:149`,
`an_existing_out_path_is_refused_rather_than_rewritten:163`. That predicate accepts **2**
exactly as it accepts **1**, so a script that VOIDed — an unwritable `--out`, a full
filesystem — would pass a test written to prove it said "the answer is no". This is the
distinction not surviving the seam, in the direction opposite to D-281/D-285's but with the
same consequence.

**Check I ran.** Read the checklist in full and the script's `main()` and `slice_of()`; read
all seven tests. `slice_of` itself does not validate its argument (`slice_of(0)` returns
`(-268, 281)` and `slice_of(17)` returns `(4500, 280)`), but `main()` gates it at
`1 <= tranche <= TRANCHES` before ever calling it, and the test covers `0`, `17` and `-1`.

---

### **M7r2** — the ledger's obligation table shows the perf guard discharged and owes no re-run

**Claim.** `overnight2_ledger.md:38` — *"| **perf guard, registered first** | **H1 NOT
REJECTED, no abort** | `artifacts/wp20b_perf_RECEIPT.txt`, raw
`artifacts/wp20b_perf_guard_v1.txt` |"*, with no row anywhere saying a re-run is owed.

**Where.** `docs/experiments/overnight2_ledger.md` §1, the obligation table (rows 38 and
44-46).

**FAILURE SCENARIO.** The table is the checklist a successor closes from — the ledger says so
— and it lists as **owed** only *"REVIEW-impl, fresh context"*, *"CI at closure HEAD"* and
*"closure D-line, commit, tag"*. The perf guard reads as done. But the run it names was taken
against a registration no fresh context had passed (round 1's B2), the registration has since
been amended twice more (the rotation and the estimator), and the raw output demonstrably
predates both. A successor closing from this table ships a perf verdict produced by an
instrument two revisions behind its own registration, and nothing in the ledger tells them
to look. The ledger's own §1 heading is **IN PROGRESS**, which is the only thing currently
preventing it.

**Check I ran.** Read the §1 table in full; compared its "owed" rows against §9's obligation
list; confirmed from the fifteen `arm` lines that `wp20b_perf_guard_v1.txt` has the
pre-rotation order.

---

### **m1r2** — bare `§5` cross-references now collide with `wp20b_impl.md`'s own §5

`wp20b_impl.md` §2's third row reads *"**NO §8 ROW EXISTS**, and **§5** says why: this
package adds no table and reads none"*, and §3's fold bullet leans on the same document. The
§5 meant is `wp20b_design.md` §5, *"COLDNESS, AND THE NON-CENSUS PATH"*. Round 1's m1 was
closed by inserting a §5 into `wp20b_impl.md` — *"WHAT THE MUTATION RUN FOUND"* — so the
unqualified reference now resolves, in the reader's own document, to a section about
something else. Qualifying it (*"the design's §5"*, as the same table does for §8) closes it.
Verified by listing both documents' headers.

### **m2r2** — the exported instrument carries its scratchpad life with it

`artifacts/wp20b_perf_guard.sh:10-12` runs `cd "$(dirname "$0")/../../../../.." 2>/dev/null
|| true` and then immediately overwrites it with a hardcoded
`ROOT=/home/tom/Projects/HeXO-AlphaBeta`. The `cd` is dead on the exported copy (five levels
up from `artifacts/` is not the repository) and is silenced by `|| true`, and the hardcoded
absolute root means the manifest's "THE INSTRUMENT" runs only on this machine at this path.
Neither affects any number taken; both belong in the file that B4(c) would name as the
registered instrument. Also `set -uo pipefail` without `-e`, which is deliberate and correct
here — the four attribution counters are what catch a dead arm — but it is worth one comment
saying so, since a reader checking the script against `SHELL_CHECKLIST.md` item 2 will ask.

---

## 4. FINDINGS I ATTEMPTED AND **REJECTED**

### **R1r2 (REJECTED)** — "the dry run was taken on the registered workload, which `docs/process.md` forbids"

**The suspicion.** `docs/process.md`'s dry-run rule: the literal commands are exercised *"on
an input of the SAME KIND as the registered workload — the same sort of artefact, differing
only in identity — and **never on the registered workload itself**"*. The dry run used the
**same** fixture and the **same** seat as the governed run, changing only `REPS=1` and
`BUDGET=2000`.

**Reproducer attempted.** Compared `wp20b_perf_dryrun_v2.txt`'s argv-equivalent inputs
against §3's registered ones; confirmed the fixture and config are identical and the budget
and reps are not.

**REJECTED.** The workload a pre-registration governs is the fixture **at** its registered
budget and reps; a 2 000-node single-rep pass produces none of the governed numbers and
cannot preview the verdict, so it does not consume the first run — which is what the
prohibition protects. And the rule's stated purpose runs the other way: *"A synthetic
stand-in exercises syntax; only a real instance of the kind exercises ATTRIBUTION."* Using the
real fixture is the strongest available attribution test, and it is what produced the honest
`census_rows 18 / 0 / 0`. Finding a defect here would punish the implementer for doing the
harder thing. **The genuine dry-run defect is a different one and is recorded at B4 limb 8:
the dry run predates the rotation, so the instrument now registered has never been exercised.**

### **R2r2 (REJECTED)** — "the determinism receipt records a compiler warning, so the tree is not clean at the closure head"

**The suspicion.** `artifacts/wp20b_determinism_v1.txt:2-6` carries
`warning: unused import: 'generate_turns' --> crates/pistol-solver/src/policy.rs:1:44`. If
that is live at the reviewed revision, the ledger's forward citation of gate 4 as green is at
risk.

**Reproducer attempted.** `git show eff179b:crates/pistol-solver/src/policy.rs` — the symbol
appears **three** times in the file, so it is used on some configuration; the file is
byte-identical at `a6777f4` and `eff179b`; and `git diff --stat a6777f4 eff179b --
crates/pistol-solver/` is **empty** — WP-2.0b does not touch that crate. `tools/ci.sh:87` runs
`cargo clippy … -- -D clippy::all`, which does not deny rustc's warn-level `unused_imports`.

**REJECTED.** Pre-existing, configuration-conditional, not this package's, and not a gate
failure. Recorded so a successor does not re-find it in the same artifact.

---

## 5. COUNT BY CLASS

| class | count | ids |
|---|---|---|
| **BLOCKING** | **5** | B1, B2, B3, B4, B5 |
| **MAJOR** | **7** | M1r2, M2r2, M3r2, M4r2, M5r2, M6r2, M7r2 |
| **minor** | **2** | m1r2, m2r2 |
| **REJECTED** | **2** | R1r2, R2r2 |

**Round-1 disposition:** of 3 BLOCKING — 1 CLOSED (B3), 1 NOT CLOSED and now ruled on (B2 →
B4), 1 RE-OPENED WORSE (B1 → B3). Of 8 MAJOR — 4 CLOSED (M3, M4, M5, M6), 3 NOT CLOSED (M1,
M2, M7), 1 split (M8: second limb CLOSED, first RE-OPENED WORSE). Of 6 minor — 4 CLOSED (m1,
m2, m3, m5), 1 RE-OPENED WORSE (m4), 1 NOT CLOSED (m6). Both REJECTED findings stand rejected.

**Verdict: FAIL**, with the reason stated narrowly, and it is the same shape as round 1's.
**Nothing here says a measurement is wrong.** I re-derived nine groups of numbers myself and
every one reproduced: both byte-identity referents over eight records, all sixteen perf
statistics under three estimators, all four `key_pos` cells and the bijection under them,
every line of `wp21_prereg.md` §3, the sixteen-tranche partition against
`set(range(13,4500))`, D-537's `n = 28, c = 21` from exact binomial tails, 26 of the 27
manifest digests, the rule-9 gate, and the mutation tally. The code's evidence is in the best
state this package has had it — **26 mutants, 26 dead at their registered test, 0 alive, at a
tree byte-identical to the one under review**.

What fails is that the documents do not carry it. The receipt that discharges the mutation
obligation is named in no document and no manifest; three documents name superseded runs
instead, and one of them says in terms that a twenty-mutant run at a six-file-older tree is
*"the receipt this package closes on"*. The manifest built to stop exactly this carries a
digest that does not match its own mutation driver, and offers as proof of reproducibility a
rebuild that round 1 explicitly says it never performed. And §3, the section this round exists
to pass or fail before its run, registers an estimator its instrument does not compute, leaves
the rule that turns twenty numbers into one verdict unwritten, and attaches to that gap an
agreement clause with no registered consequence whose failure is already sitting in the raw
output on disk.

**ONE LINE FOR THE MORNING.** The mutation set is green and the arithmetic is all correct — I
checked every number myself — but v5, the receipt that proves it, is written down nowhere; the
manifest's digest for the mutation driver is wrong; and `wp20b_impl.md` §3 may not govern the
perf guard's re-run until it names one estimator, one decision rule, and its own instrument
by digest.
