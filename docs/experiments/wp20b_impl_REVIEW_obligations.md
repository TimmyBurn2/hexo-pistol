# WP-2.0b REVIEW-impl (OBLIGATIONS) — the documents, the registrations, the receipts

**NAMED REVISION:** `f7606cc7809cc19e9b481a57af7f0a662cddd831`, a `git stash create`
object holding the uncommitted work on `dev` at HEAD `a6777f4`.

**DOES IT MATCH THE WORKTREE?** YES, at the moment this review began.
`git add -A && git stash create` produced `7569445bbf2c2db8675a1a05d8147c9c97626fa5`
and `git diff f7606cc 7569445 --stat` is EMPTY. It has not moved. (Writing this
report is the first change; after it lands the worktree differs from `f7606cc` by
this file alone.)

**VERDICT: FAIL.** Three BLOCKING. The code's measurements are sound — I re-derived
every number the receipts state and found them all correct — but the perf guard's
own amendment was registered in a document that has no reviewed existence before the
run it governs, one governing §9 obligation is undischarged and now has no home, and
the arc ledger names the wrong revision as the one under review.

**SCOPE.** Documents, registrations and receipts. A second reviewer has the code.
Where a test's NAME and FILE discharge a documented obligation I checked that the
name exists where the document says; I did not review what it asserts.

---

## 1. THE OBLIGATIONS TABLE

### 1.1 `wp20b_design.md` §9 — OBLIGATIONS BEFORE CLOSURE

| # | obligation, in §9's words | artifact that discharges it | state | did I verify it myself |
|---|---|---|---|---|
| 1 | **BYTE-IDENTITY, GATE OFF** — two-binary diff, extraction rule, re-taken from the main tree | `artifacts/wp20b_identity_RECEIPT.txt` over `artifacts/prechange2_*` and `artifacts/postchange_*` | **DISCHARGED** | **YES** — all 8 digests re-derived from the raw records by applying §9's rule myself; both match the registered referents |
| 2 | **DETERMINISM** — `tools/determinism.sh` green on all seats | `artifacts/wp20b_determinism_v1.txt` | **DISCHARGED** | read the artifact (5 seats, `EXIT=0`, each seat's own `ok —` line); did not re-run |
| 3 | **PERF GUARD** — one bench comparison, registered instrument, paired per position, nps AND time | `artifacts/wp20b_perf_RECEIPT.txt`, raw `artifacts/wp20b_perf_guard_v1.txt` | **DISCHARGED as a measurement**, **UNMET as a registration** (B2) | **YES** — every statistic recomputed from the 300 raw rows |
| 4 | **DRY RUN — taken, input and output recorded** | `artifacts/wp20b_perf_dryrun_v2.txt` | **DISCHARGED as a recording** | **YES** — all four criterion values re-derived from the raw (0 refusals; 20/20/20; 0 empty-board; 18/0/0 census rows) |
| 5 | **H1 = 1.000x with a rejection region** | band in `wp20b_IMPL_FINDINGS.md` (committed `a6777f4`); arms/counters in `wp20b_impl.md` §3 | **PARTIAL** — the band is genuinely pre-run; the rep-aggregation rule is unregistered (M1) | **YES** |
| 6 | **COST stated on the document's own face** | design §9 (~16 min, 200 searches) | **UNMET for the amended instrument** (M6) | **YES** — the run is 300 searches, 22.6 min of search time alone |
| 7 | **THE STRUCTURAL CHECK (test 17)** | `the_fold_is_entered_exactly_once_per_firing` (`crates/pistol-search/tests/census_identity_tests.rs:69`); M17 dies at it in `wp20b_mutants_v2.txt` | **DISCHARGED** | **YES** — name located, mutant kill line read |
| 8 | **TRANCHE ONE EMITS `key_pos` BESIDE THE CANONICAL KEY, and the two distinct counts are compared** | none | **UNDISCHARGED, AND NOW HOMELESS** (B3) | **YES** — no artifact anywhere carries the comparison |
| 9 | **REVIEW-impl**, fresh context, one fix round | this report + the code reviewer's | in progress | — |
| 10 | **ARTIFACTS exported with digests (D-469)** | partial: `wp20b_perf_RECEIPT.txt` digests the raw; `wp20b_identity_RECEIPT.txt` digests the two binaries | **PARTIAL** (M8); closure-time, not yet due | **YES** — no export receipt or artifact manifest for WP-2.0b exists |
| 11 | **CI all 19 gates at closure HEAD** | only `artifacts/ovn2_ci_a6777f4_v1.txt`, which is the PRE-work base | **OWED**, and correctly labelled owed in the ledger | **YES** — 19 `gate n/19` lines, `ci: all gates passed`, `EXIT=0`, at `a6777f4` |
| 12 | tree clean; closure summary; ROADMAP | — | owed | — |

### 1.2 The WP-2.0b **v2** dispatch's Obligations list

| obligation, verbatim | artifact | state | verified |
|---|---|---|---|
| *"Gate off = byte-identical: two-binary diff over the standing position set, output digest equal to pre-change."* | `wp20b_identity_RECEIPT.txt` | **DISCHARGED** | **YES**, re-derived |
| *"Determinism all seats; census state newgame-cleared and seated if it exists."* | `wp20b_determinism_v1.txt`; seated by `a_plain_go_after_a_census_go_computes_no_key_and_emits_no_line` (`crates/pistol-cli/tests/census_protocol_tests.rs:370`) | **DISCHARGED** | artifact read; test name located |
| *"Tests: identity on every census row (schema test); disjointness fixture counts exactly as ruled; token absent = zero census bytes, pinned by a loud test."* | tests 1, 4, 5 of §8 | **DISCHARGED** | names located at `census_protocol_tests.rs:116`, `canonical_key_tests.rs:82`, `census_protocol_tests.rs:179` |
| *"Mutants, per D-55y run green BEFORE REVIEW-impl, call-removed mutants included"* + the four named mutants | `artifacts/wp20b_mutants_v2.txt` at `dfba9d7` | **DISCHARGED** | **YES** — `20 dead at their registered test, 0 alive`, `EXIT=0`; 8 CALL-REMOVED against §8's registered 7; `git diff dfba9d7 f7606cc --stat` = `wp20b_impl.md` only, so the receipt covers the code under review exactly |
| *"REVIEW-impl: fresh, strongest, one fix round"* | this + the code review | in progress | — |
| *"Bench guard: registered nps spot-check ON-token vs OFF at 50 000 nodes"* | `wp20b_perf_RECEIPT.txt` | **DISCHARGED** | **YES** — ON/OFF median 1.0012, min 0.9789, abort 0.95 not approached |

### 1.3 The dispatch's Closure list (not yet due; state recorded so it is not lost)

`D-line recording the identity form and transposition ruling` — **not written**
(D-565/D-566 are the arc's, not this package's). `protocol doc updated in its one
home` — **DISCHARGED**: this repository has no `docs/protocol.md`; the protocol's one
home is `crates/pistol-cli` (D-5, D-167), and `protocol.rs`'s `go` docs and
`report.rs`'s `census_line` docs both carry the addition. `artifacts exported with
digests` — partial (M8). `CI at closure HEAD` — owed. `ROADMAP` — registered in
`overnight2_ledger.md` §2 and `wp21_prereg.md`.

### 1.4 The five carried findings

| id | closed? | the check I ran |
|---|---|---|
| **AG2** — H1 stated once | **closed inside `wp20b_impl.md`**, with a residual in the governing design (m2) | read both documents; `wp20b_impl.md` §3 names the second comparison "the gross-regression abort" and never "H1" |
| **AG3** — the rejection band | **closed, and genuinely pre-run** | `wp20b_IMPL_FINDINGS.md` carries *"Register `H1 rejected outside [0.98, 1.02]`"* and is COMMITTED at `a6777f4` (2026-09-01 22:05), ~2 h before the run. The derivation (`sd 0.0075`) is at `wp20b_design_rev8_REVIEW.md:598`. Realised per-position sd is 0.00451, tighter than registered. **But which statistic the band applies to is not fixed — see M1.** |
| **AG4** — fourth-word refusal pinned AND quoted | **closed** | the `///` on `parse_budget` (`crates/pistol-cli/src/budget_token.rs`) quotes ``` `go` takes one budget and at most the `census` token, and `<fourth>` follows them ```, character-for-character the `format!` two screens below it; `a_fourth_word_on_a_go_line_is_refused_naming_the_fourth_word` (`census_protocol_tests.rs:225`) asserts the exact string with `extra` substituted; M7 dies at it |
| **AG5** — four dispatch mutants against §8's rows | **closed, 3 of 4 cleanly** (m3) | all four test names located in the files the table names; §8 rows 1, 5 and 4 confirmed by reading §8's table. The third row points at "§5" rather than at §8 test 16, although the test it names IS §8 test 16 |
| **AF2-residual** | correctly not taken | — |

---

## 2. NUMBERS I RE-DERIVED AND FOUND **CORRECT**

Stated because a verified number is worth as much as a finding.

1. **Both byte-identity referents.** Applying §9's rule
   (`sed -n '1,/^# timing/p' | grep -v '^revision \|^binary_sha256 ' | sha256sum`)
   to all eight records gives `81e37d42…` for both `gate_v0` runs on **both**
   binaries and `c7f155e8…` for both `instrument_v0` runs on **both** binaries.
   Exactly the registered referents. Run 1 = run 2 on each binary, which is the
   check that the rule did not buy satisfiability. Both binaries' sha256 confirmed
   (`180b4c40…`, `15c94598…`).
2. **The perf guard, all of it**, recomputed from the 300 raw rows under
   per-position median-over-reps: H1 nps median **0.9979**, min **0.9882**, max
   **1.0048**, **0 of 20** outside `[0.98, 1.02]`; ON/OFF nps median **1.0012**, min
   **0.9789**, max **1.0093**, **0 of 20** below 0.95 and **0 of 100** per-rep;
   H1 time median **1.0004** min **0.9952** max **1.0909**; ON/OFF time median
   **1.0000** min **0.9908** max **1.0078**. Attribution: 20/20/0 on every arm and
   rep, `census_rows` 181/0/0 identical across all five reps, `REFUSAL` lines **0**.
   Depth reached is identical across all three arms at every position — the arms
   really did the same work.
3. **The dry run v2**, all four criteria, from its own raw: 0 refusals, 20/20 on
   three arms, 0 empty-board answers, 18/0/0 census rows.
4. **The instrument digests.** `wp20b_perf_guard.sh` = `9ec50053…` and
   `wp20b_perf_report.py` = `0a6a0f87…` on disk today, matching the receipt;
   raw = `bdfc1ea0…`, matching.
5. **`wp21_prereg.md` §3, every line.** `742/13 = 57.0769`; `347/13 = 26.6923`;
   `742/347 = 2.1383`; `657/742 = 0.8854`; `21.505/26 = 0.8271`; `4487x2 = 8974`;
   records `256 104`; distinct `119 768`; capture `226 766 s = 62.99 h`; per tranche
   `562 / 16 039 / 7 501`, `14 201 s = 3.94 h`, `1 859 s = 0.52 h`,
   `455 s = 0.13 h`, `73 s`; wall `2 x ~4.6 h = ~9.2 h`. Each total uses the EXACT
   fraction rather than the rounded rate printed above it, which is the right
   choice and is what makes them reproduce. `657`, `742`, `26`, `13` and the
   `wall 21505 ms at 4 workers` all appear in `artifacts/wp20pilot_RUN_2cd4f79_v1.txt`.
   **`347` does not** — see M3.
6. **The partition.** `tools/wp21_tranche_config.py`'s `slice_of` over tranches
   1..16 yields skips `13, 294, 575, 856, 1137, 1418, 1699, 1980, 2260, 2540, 2820,
   3100, 3380, 3660, 3940, 4220`, takes `281 x 7` then `280 x 9`, sum **4487**,
   first **13**, last **4499**. **Contiguous, disjoint and exhaustive over
   `13..4499`** — I built the set and compared it to `set(range(13,4500))`. The
   book holds **4500** non-comment lines, so `0..12` plus `13..4499` exhausts it and
   the ledger's *"the book is now fully claimed"* is true.
7. **The D-537 minimum.** With `p0 = 8/14`, `p1 = 12/14`, alpha 0.05, power 0.95,
   the smallest `n` admitting a critical `c` with `P(X>=c|p0) <= 0.05` and
   `P(X>=c|p1) >= 0.95` is **n = 28, c = 21**, size **0.040184**, power
   **0.962225** — every digit of `overnight2_ledger.md` §4 reproduced, and `c = 21`
   is the unique admissible critical value at n = 28. `n = 20..27` admit none.
   The inputs are the right cells: `matrix_stage3_detector.md:613` reads
   `| trigger-rich | 14 | **0.571** (8 of 14) | 1.000 | **0.857** (12 of 14) |`
   under headers `per-search ceiling` / `aggregate oracle` / `bound over the
   columns`, and `wp20s_design.md` §8 items 2, 3 and 4 name exactly those two.
8. **D-566, every limb.** `git merge-base --is-ancestor d22b95d HEAD` exits 1 (NOT
   an ancestor); `git merge-base HEAD d22b95d` = `a56449b`; the eight differing
   paths are exactly the eight the receipt enumerates; `docs/decisions.md` differs
   in **exactly one line** and it is **D-563**; the branch's design line 1 reads
   *revision 4* and `dev`'s reads *revision 8*; `git rev-parse --verify
   wp20b-stopped` fails. **What can still be checked, contrary to the receipt's own
   closing note:** the commit `d22b95d` is still in the object database (reachable
   from the reflog and unpruned), so every one of the receipt's claims is
   independently reproducible today. What cannot be checked is the branch REF, and
   what will stop being checkable is the commit itself after a `git gc` prunes it —
   at which point the receipt is the only record, which is what it says.
9. **CI at `a6777f4`**: 19 `gate n/19:` lines, `ci: all gates passed`, `EXIT=0`.
   Gate 18's `566 decision keys` is consistent with a maximum of D-564 plus the two
   D-279-grandfathered repeats, i.e. this run predates D-565/D-566 as the ledger
   says.
10. **Rule 9**, run by me: `tools/file_justification_check.sh` →
    `355 tracked .rs/.sh files, 63 over the cap, all registered in
    docs/rule9_justifications.md (63 entries)`, exit 0.
11. **Rule 8**: `git ls-files artifacts/` returns **0** files; `/artifacts/`,
    `/sessions/`, `__pycache__/` and `*.pyc` are all in `.gitignore`; `f7606cc`
    contains no `__pycache__` or `.pyc` path; there are no untracked non-ignored
    files. **Nothing is committed that should not be.**
12. **The wire format.** `census_line`'s `format!` in
    `crates/pistol-cli/src/report.rs:140-144` emits the field order §4 prints,
    field for field, including the `-` spelling for the unasked defender.

---

## 3. FINDINGS

### **B1** — the arc ledger names a revision that is not the one under review, and says every receipt is against it

**Claim.** `docs/experiments/overnight2_ledger.md` §1: *"**THE NAMED REVISION under
review is `8efeda0`** — a `git stash create` object holding the uncommitted work on
`dev` at `a6777f4`. Every review and every mutation receipt of this phase is against
it."* Both sentences are FALSE.

**Where.** `docs/experiments/overnight2_ledger.md`, §1, the paragraph after the
obligation table.

**Failure scenario.** The ledger's own first line is *"A successor continues from
THIS, never from memory"*, and CLAUDE.md's Process requires each review to be
dispatched against a NAMED REVISION stated in the reviewer's header. A successor
reading this line dispatches the closure — or a re-review, or a re-run of the
mutation set — against `8efeda0`, a tree in which the M10 test fix does **not**
exist. At `8efeda0` the mutation set is `19 dead, 1 alive, EXIT=1`. The successor
would be re-running the package against the revision whose receipt FAILED, and
would find M10 alive and conclude a regression.

**Check I ran.** `git show -s --format='%ci' 8efeda0` → `00:23:22`; `dfba9d7` →
`00:25:51`. `artifacts/wp20b_mutants_v1.txt` line 1 names worktree revision
`8efeda0…` and ends `19 dead …, 1 alive`, `EXIT=1`;
`artifacts/wp20b_mutants_v2.txt` line 1 names `dfba9d7…` and ends `20 dead …,
0 alive`, `EXIT=0`. `wp20b_impl.md` §6 says run 2 *"is the receipt this package
closes on"*. This REVIEW-impl is dispatched against `f7606cc`.
`git diff 8efeda0 dfba9d7 --stat` shows three files changed, two of them tests.
So neither the closing mutation receipt nor this review is against `8efeda0`.

**Related, same paragraph:** the obligation table's row *"mutation receipts (D-553) |
running, in /home/tom/pistol-runs/wp20bmut | artifacts/wp20b_mutants_v1.txt"* is
stale in both cells (see m4).

---

### **B2** — the perf guard's amendment governs a run it has no reviewed existence before

**Claim under attack.** `wp20b_impl.md` §3: *"**THE INSTRUMENT IS THE DESIGN'S BLOCK
PLUS TWO COUNTERS AND A THIRD ARM, AND THE ADDITIONS ARE REGISTERED HERE BEFORE THE
RUN.**"* and, in §1's AG3 row, *"§3 registers … BEFORE the run."*

**Where.** `docs/experiments/wp20b_impl.md` §3, the bolded sentence and the
dry-run criteria table beneath it.

**Failure scenario.** A reader takes §3 as a pre-registration and reads the guard's
result as adjudicated against criteria fixed in advance. It was not adjudicated
against anything that had passed a review. `docs/process.md`'s *"Instrument
governing revision"* section: *"An artefact that produces a registered number … is
named in the pre-registration WITH ITS REVISION, and a change to it reopens the
review exactly as an amendment to the document does. Without this, a run stands on
an instrument whose own review had failed and is licensed by argument rather than by
this text."* CLAUDE.md: *"A pre-registration is reviewed at the revision that GOVERNS
the run — that revision must itself pass a fresh-context review before the first run
it governs … and an amendment reopens the review however small the diff."* The
amendment — a third arm, four attribution counters, four dry-run criteria and (by
omission, M1) an estimator — was never reviewed by anyone before the run. **This
review is the first fresh context to see it, and it sees it after the numbers.**

**What the evidence establishes, and what it does not.** I am splitting this because
the brief asks me to say plainly where I cannot settle the order.

*Established:*
- The instrument script `wp20b_perf_guard.sh` and `wp20b_perf_report.py` were written
  at **23:58:35**. The dry run's output landed **23:58:57**. The governed run's raw
  output completed **00:22:06** and contains **22.6 minutes** of summed search time,
  so it started at approximately **23:59:2x**. The measurement window is
  23:59 → 00:22.
- `docs/experiments/wp20b_impl.md`'s mtime is **00:26:20** and its earliest
  appearance in ANY git object is `8efeda0` at **00:23:22** — 76 s after the perf
  receipt was written and 76 s after the run ended.
- **No version of the amendment was ever committed, stashed or otherwise made
  reviewable before the run**, and no review of it exists. That obligation is unmet
  regardless of when the text was typed.
- The band `[0.98, 1.02]` itself IS pre-run and is not implicated: it is in
  `wp20b_IMPL_FINDINGS.md`, committed at `a6777f4` at **22:05**.

*Not established, and I say so:* whether the third arm, the counters and the
criteria table were WRITTEN DOWN somewhere before 23:59. The git object graph
**cannot** settle it, and the tempting inference is wrong: `git stash create`
without `-u` captures only tracked files, and I confirmed two untracked files that
demonstrably existed on disk before `ac58c2b` (00:22:36) — `wp21_prereg.md` (mtime
00:06:08) and `tools/wp21_tranche_config.py` (00:04:12) — are absent from it. So
`wp20b_impl.md`'s absence from `ac58c2b` proves nothing about the disk. What *is*
evidence in the implementer's favour is that the instrument implementing the three
arms and four counters existed at 23:58:35 and the dry run exercised them at
23:58:57 — so the amendment's CONTENT preceded the run even if its DOCUMENT did not.

**The narrow, unarguable defect** is therefore the review, not the typing: the
governed run was taken against a registration that no fresh context had attacked,
and §9's own *"an edit to the block reopens this registration"* was not honoured.

**Remedy shape.** Either the operator rules that a same-session amendment whose
content is embodied in a dry-run-exercised instrument satisfies the rule (an ADR
line), or the guard is re-run under the now-reviewed §3. The run is 22.6 minutes.

---

### **B3** — §9's `key_pos` obligation is undischarged, and after this package it has nowhere left to be discharged

**Claim.** `wp20b_design.md` §9: *"**TRANCHE ONE EMITS `key_pos` BESIDE THE CANONICAL
KEY**, and the two distinct counts are compared. This is the two-line answer to §2's
strongest surviving attack — the symmetry fold's only measurement is zero (D-560's
`key_seq = key_pos = key_full = 347`) and this settles whether the in-tree fold's
yield is above it."*

**Where.** `wp20b_design.md` §9 (the obligation); `wp20b_impl.md` §4 departure 1
(the relocation); `wp21_prereg.md` §0 (the closure of the original home);
`overnight2_ledger.md` §1 (the omission from the checklist).

**Failure scenario.** §2 selected the symmetry fold (option C2) over `key_pos`. The
DECISION-RED-TEAM's strongest surviving attack is that the fold's only measurement is
**zero yield** — D-560's three keys all agreed at 347. §9's answer was this
comparison. It has not been taken, and the three places it could live are now all
closed: `wp21_prereg.md` §0 puts the census **OFF for the whole sweep** by the
dispatch's words, so no tranche of this arc can carry the column; `wp20b_impl.md` §4
relocates the measurement to `crates/pistol-search/examples/trigger_census.rs` and
cites **no artifact**; `overnight2_ledger.md` §1's obligation table has **no row for
it at all**, and §4 (the census run) is *"NOT REGISTERED"*. A reader of the closure
concludes the identity decision's strongest attack was answered. It was deferred and
then dropped out of every checklist.

**Check I ran.** `key_pos` is printed by the example — `trigger_census.rs:227`
formats `key {} key_pos {}` and reads `row.key_pos` — and the field exists at
`crates/pistol-search/src/census.rs:39`. So the INSTRUMENT is built. But
`grep -rln key_pos artifacts/` returns only five `wp20pilot_*` files, which carry
the CORPUS's three keys and not the census's. No artifact anywhere holds a
census key-vs-`key_pos` distinct-count comparison. `wp20b_impl.md` §4's *"The
measurement is therefore taken on … `trigger_census.rs`"* names a place and cites
no receipt, in a document that cites a receipt for everything else it claims.

**Remedy shape.** The instrument exists and the run is one `cargo run --example`.
Either take it and cite it, or the operator rules the obligation deleted with an ADR
line saying so — but it may not simply vanish from the ledger.

---

### **M1** — the reported statistic is not the registered one, and the choice changes what the receipt says

**Claim.** `wp20b_perf_RECEIPT.txt`: *"positions OUTSIDE H1's registered band
`[0.98, 1.02]`: **0 of 20**"*, echoed in `overnight2_ledger.md` §1 as *"**all 20
positions sit inside the registered band**"*.

**Where.** `wp20b_impl.md` §3 (the registration: *"paired per position, REPS=5"* —
and nothing about how five reps collapse); `wp20b_perf_RECEIPT.txt` §"THE TWO
COMPARISONS".

**Failure scenario.** A reader takes "0 of 20" as a fact about the engine. It is a
fact about an estimator nobody registered. The instrument
(`wp20b_perf_report.py`, `med()`) takes the **median** over the five reps and then
the ratio. Under the **mean** — equally admissible under "paired per position,
REPS=5" — position 10's H1 nps ratio is **1.0215**, outside `[0.98, 1.02]`, and the
receipt would read "1 of 20". Under raw per-rep pairing, **7 of 100** H1 ratios fall
outside the band. Three defensible readings, three different sentences, and the
registration picks none of them.

**Second limb, same defect.** The band's derivation
(`wp20b_design_rev8_REVIEW.md:598`) reads *"paired per-position nps over three reps
of one binary gives **sd 0.0075, range 0.987–1.021**, so a **±2 % paired band at
REPS=5**"* — the review's own NOISE sample already produced a per-position value of
**1.021**, outside the band it then recommended. So a per-position application of
the band is expected to produce out-of-band positions from noise alone, and "0 of
20" is a stronger-sounding statement than the derivation licenses.

**What is NOT endangered, said so this is not read as bigger than it is.** H1's
verdict survives every reading: the median of the per-position ratios is 0.9979
under medians and 0.9981 under means, and the 0.95 abort is untouched under all
three (0 of 100 per-rep ON/OFF ratios below 0.95). **H1 is not rejected on any
reading.** What is wrong is the receipt's per-position sentence.

**Check I ran.** Recomputed H1 and ON/OFF under median-over-reps, mean-over-reps and
raw per-rep from the 300 rows of `wp20b_perf_guard_v1.txt`; read `med()` in
`wp20b_perf_report.py`.

---

### **M2** — every receipt names a revision whose tree does not contain the work it measured

**Claim.** `wp20b_identity_RECEIPT.txt`: *"post-change tree revision (uncommitted
work on) a6777f47…"*. `wp20b_perf_RECEIPT.txt`: *"GOVERNING REVISION of the
instrument and of the tree it measured: git HEAD a6777f47…, work UNCOMMITTED"*.

**Where.** both receipts, the header block.

**Failure scenario.** `a6777f4` is the commit **without** the WP-2.0b diff. A reader
who wants to know whether `15c94598…` is the binary of the code under review has
been handed a revision at which that binary cannot be built. CLAUDE.md's Process is
explicit that a named revision is *"a commit SHA, or a `git stash create` SHA where
the work is uncommitted"* — the receipts give the former where only the latter
identifies anything. §9's own instruction was *"INSTRUMENT REVISION: the closure HEAD
at which the guard runs, restated in the closure receipt"*, and what was restated is
the parent commit.

**Check I ran, and what it cost.** I could only establish that the binary matches the
reviewed code by reconstructing it from **dangling** stash objects: `e14bf06`
(23:54:23, immediately before the 23:55 build) is the last stash object predating the
build, and `git diff e14bf06 f7606cc -- crates/` shows the only non-test source
change since is **`///` doc comments in `protocol.rs`**. Those objects are
unreferenced and a `git gc` removes them. A receipt should not require that.

**This finding is about traceability only.** The measurement itself is sound — see
R1.

---

### **M3** — `wp21_prereg.md` §3 attributes `347 distinct` to an artifact that does not contain it

**Claim.** *"Every per-unit rate is **MEASURED** in the pilot
(`artifacts/wp20pilot_RUN_2cd4f79_v1.txt`, 13 openings / 26 games / 742 records /
**347 distinct**)"*.

**Where.** `docs/experiments/wp21_prereg.md` §3, first sentence.

**Failure scenario.** `347` is the numerator of `distinct per opening = 26.6923`,
which produces the headline `~119 800 distinct positions` that sizes the entire
sweep and, through it, the ledger's *"the book is now fully claimed"* consequence for
two other standing claimants. A reviewer checking the sweep's size against the cited
artifact **cannot derive it** and would have to either accept it unchecked or
conclude the arithmetic is wrong.

**Check I ran.** `/usr/bin/grep -n "347" artifacts/wp20pilot_RUN_2cd4f79_v1.txt`
returns two lines, neither of them the count: `binary … 180b4c40…` and
`n 26  distinct-n 13`, which is distinct GAMES. The figure is real and lives at
`docs/experiments/wp20_CLOSURE.md:46,50` (*"742 records -> 347 distinct positions"*,
*"key_seq = key_pos = key_full = 347"*) and
`docs/experiments/wp20_pilot_artifacts.md:67`. **D-560 does this correctly** — it
closes the parenthetical at *"742 records"* and states the 347 separately outside it.
`wp21_prereg.md` pulled it inside.

---

### **M4** — the sentence that registers the amendment miscounts what it registers

**Claim.** `wp20b_impl.md` §3: *"**THE INSTRUMENT IS THE DESIGN'S BLOCK PLUS TWO
COUNTERS AND A THIRD ARM**"*. The bullet three lines below registers **four**:
`totals_lines`, `bestmove_lines`, `empty_board_answers`, `census_rows`.
`wp20b_perf_RECEIPT.txt` says *"the four attribution counters registered in
docs/experiments/wp20b_impl.md §3"*.

**Where.** `wp20b_impl.md` §3, the bolded registering sentence.

**Failure scenario.** The registering sentence is the one a reviewer of the
amendment reads to know its extent. A reader who takes "two counters" as the
registered set and then meets four in the receipt has to decide which two were
registered and which two arrived later — the exact question B2 is about, made
unanswerable by the document's own text.

**Check I ran.** Counted the names in the bullet; read the receipt's counter-parity
line; counted the `arm … totals_lines … bestmove_lines … empty_board_answers …
census_rows` fields in the raw output (four per arm per rep, 15 such lines).

---

### **M5** — the ledger makes gate and test claims with no cited log output

**Claim.** `overnight2_ledger.md` §1 obligation table: *"workspace tests | green |
re-run per suite after the two late test edits"*; *"clippy `-D clippy::all` | green |
—"*; *"rule 9 | green, 63 over cap all registered | `tools/file_justification_check.sh`"*.

**Where.** `docs/experiments/overnight2_ledger.md` §1, the table's receipt column.

**Failure scenario.** CLAUDE.md's Closure section: *"A gate or test claim in any
report cites the gate's own log output, never a wrapper's exit status."* Two of these
three rows cite nothing at all and the third cites a tool name rather than its
output. A closure assembled from this table would carry three green claims that no
artifact supports, and the two test edits the first row mentions are exactly the ones
that landed AFTER the byte-identity and determinism artifacts were taken — the case
where an unrecorded re-run is least safe to assume.

**Check I ran.** I ran `tools/file_justification_check.sh` myself:
`355 tracked .rs/.sh files, 63 over the cap, all registered … (63 entries)`,
exit 0 — so the rule-9 row is TRUE, and now has a cited output. I did not run
`cargo test` or `clippy` (CLAUDE.md forbids me the live tree's target directory, and
a second reviewer holds a worktree at `f7606cc`), so those two rows remain claims
with no receipt.

---

### **M6** — the amendment changed the run's cost by half and did not restate it

**Claim.** `wp20b_design.md` §9 COST: *"2 arms x **20 positions** x 5 reps … which is
**~16 minutes** for 200 searches"*. `wp20b_impl.md` §3 adds a third arm and says
nothing about cost.

**Where.** `wp20b_impl.md` §3, addition 1.

**Failure scenario.** `docs/process.md`: *"A pre-registration states what its governed
run COSTS — wall time, operator attention, machine hours — so the proportion between
the document and the run is visible on the document's own face."* The amendment's
face carries the design's 200-search figure; the run was **300 searches** and
**22.6 minutes of search time alone** (summed `time_ms` over the 300 raw rows,
1 356 330 ms). An operator sizing the fix round from the document under-budgets by
about half, and the same document's §7 shows this session already lost a suite to
exactly that class of under-estimate.

**Check I ran.** Summed `time_ms` across all 300 rows of the raw output.

---

### **M7** — a `tools/` change landed without the review `docs/process.md` requires for it

**Claim.** The diff adds `tools/wp21_tranche_config.py` (202 lines, new).

**Where.** `docs/process.md`, *"tools/ review coverage rule"*: *"A change under tools/
is reviewed against `tools/SHELL_CHECKLIST.md` — the review prompt cites it and the
reviewer answers its items by name."*

**Failure scenario.** The rule exists because *"three consecutive rounds found ONE
class in those scripts"*. This reviewer's dispatch does not cite the checklist and
does not ask for its items by name; if the code reviewer's does not either, the
generator that writes sixteen production sweep configs ships without the review its
own methodology mandates, and the failure class is EXIT-0-WRONG-ANSWER — a config
silently written with the wrong slice.

**What I could check without the prompt.** The checklist's own **item 10, THE
COVERAGE RULE** — *"any tools/ script that produces a recorded number carries at
least one test driving the shipped script"* — **is satisfied**:
`crates/pistol-arena/tests/wp21_tranche_config_tests.rs:13-22` runs
`Command::new("python3").arg(repo().join("tools/wp21_tranche_config.py"))`, the
shipped script. **Item 12** (VOID distinguished from FAIL by name) is satisfied: the
script defines `VOID = 2`, refuses with exit 1 and voids with exit 2, and its header
enumerates all three. **Item 8** (one spelling per number, one refusal per reason) is
satisfied: `--tranche 01` is refused by name. I have not judged the remaining nine
items and they are not mine to sign off.

---

### **M8** — the instrument that produced the registered numbers exists only in an ephemeral scratchpad

**Claim.** `wp20b_perf_RECEIPT.txt` names its instrument as
`/tmp/claude-1000/…/scratchpad/wp20b_perf_guard.sh` and
`…/wp20b_perf_report.py`, each with a sha256.

**Where.** `wp20b_perf_RECEIPT.txt`, the INSTRUMENT block.

**Failure scenario.** Neither file is under `artifacts/`, and `/tmp` on this machine
is a 24 GiB **tmpfs** (CLAUDE.md, Environment) inside a per-session scratchpad
directory. After a reboot or a session change the registered instrument is gone and
only its digest survives — a digest of nothing anyone can obtain. Rule 8 permits a
manifest to sha-index an uncommitted artifact, but the artifact has to exist
somewhere; here nothing is exported. §9's own requirement is *"ARTIFACTS exported
with digests (D-469)"*, and `docs/decisions.md`'s memory note records that finding IDs
pointing at `/tmp` paths have already been lost once on this project.

**Second limb.** The named report script does **not** produce two of the numbers the
receipt reports. `wp20b_perf_report.py`'s `time_ms` branch prints the median only,
yet the receipt states `time_ms … min 0.9952 max 1.0909`, and the line *"positions
OUTSIDE H1's registered band … 0 of 20"* is produced by neither script. A successor
who re-runs the named instrument on the raw gets a strict subset of what the receipt
shows and cannot tell which of the extra numbers were computed how. (I recomputed
them independently and they are all correct — see §2.2 — so this is a
reproducibility defect and not an accuracy one.)

**Check I ran.** `sha256sum` on both scratchpad files (both match the receipt);
`ls artifacts/ | grep perf_guard.sh` → nothing; read the report script's `else`
branch.

---

### **m1** — `wp20b_impl.md` has no §5

Headings run **1, 2, 3, 4, 6, 7**. Introduced by the mutation section: at `8efeda0`
the document ran `… 4, 5` and the fix round inserted a new §6 and renumbered the old
§5 to §7 without closing the gap. A reader hunting a cross-reference to §5 — and this
project's documents cross-reference by section constantly — finds a hole and has to
decide whether a section was deleted. Verified with `git diff 8efeda0 f7606cc --
docs/experiments/wp20b_impl.md`.

### **m2** — AG2's residual: the governing design still spells H1 the way AG3's band contradicts

AG2's remedy was *"State H1 once, in the bullet that owns it, and have the other
mentions point there"*. `wp20b_impl.md` does this **within itself** and its §1 row
honestly scopes the claim to *"Every other mention **here**"*. But the governing
design is committed and unedited, and `wp20b_design.md` §9 still opens its H1 bullet
with *"**H1: the ON/OFF nps ratio is `1.000x`.**"* before the next bullet corrects
it, and **no document says that sentence is superseded**. Applying the design's
spelling with AG3's band to this very receipt gives: ON/OFF, position 20, **0.9789**
— outside `[0.98, 1.02]` — **H1 rejected**. The opposite verdict from the same data,
reachable by reading the two governing documents in the wrong order. One sentence in
`wp20b_impl.md` §3 naming the design's line as superseded closes it.

### **m3** — AG5's table answers three of four mutants with a §8 row and the fourth with a §5 pointer

`wp20b_impl.md` §2 maps *"warm-table read introduced -> D-527 seat dies"* to *"§5,
and the design's answer is that this package adds no table"*. The substitution is
declared and is right, but the test it then names —
`a_plain_go_after_a_census_go_computes_no_key_and_emits_no_line` — **is** §8 test 16.
AG5's ask was *"the four dispatch mutants against §8's rows in one table"*; naming
the row would have completed it with no argument.

### **m4** — the ledger's mutation row is stale in both cells

*"mutation receipts (D-553) | **running**, in `/home/tom/pistol-runs/wp20bmut` |
`artifacts/wp20b_mutants_v1.txt`"*. Run 2 completed at `dfba9d7` and
`wp20b_impl.md` §6 says v2 *"is the receipt this package closes on"*. The ledger's
mtime (00:24:34) predates the v2 export (00:27:43), so this is an ordering artifact
— but the ledger is the document that says *"Anything not written here did not
happen"*, and what is written here is that the receipt is the failed one.

### **m5** — the ledger's obligation table has no row for D-469 and no artifact manifest exists

`wp20b_design.md` §9's closure list names *"ARTIFACTS exported with digests
(D-469)"*; `overnight2_ledger.md` §1's table has rows for byte-identity, determinism,
perf, mutants, review, CI and the closure D-line, and **none** for the export. WP-2.0
shipped `docs/experiments/wp20_pilot_artifacts.md` as its manifest; WP-2.0b has no
counterpart. Closure is legitimately owed, so this is not yet a breach — it is an
obligation missing from the checklist a successor closes from, which is how B3
happened.

### **m6** — the arms run in a fixed order every rep, so within-rep drift is confounded with H1

`wp20b_perf_guard.sh`'s loop is `post_off`, `post_on`, `pre_off`, in that order, five
times. H1 is `post_off / pre_off` — always the FIRST arm of a rep over the LAST,
about eight minutes apart, never counterbalanced. A monotone thermal or boost drift
across a rep is therefore indistinguishable from the effect H1 measures, and the
measured H1 of **0.9979** is the same sign and the same order of magnitude as such a
drift. The registration says nothing about arm order. This does not endanger the
verdict — the effect is a tenth of the band — but "0.9979" should not be read as an
engine property. Noted rather than escalated because the design registered the block
this script implements and the block has the same order.

---

## 4. FINDINGS I ATTEMPTED AND **REJECTED**

### **R1 (REJECTED)** — "the measured binary is stale relative to the reviewed code"

**The suspicion.** `target/release/pistol` (`15c94598…`) was built at **23:55**, and
it is the post-change binary for the byte-identity records (23:55), the determinism
run (23:57), the dry run (23:58) and the governed perf guard (00:22). But
`crates/pistol-cli/src/protocol.rs` — a **source** file, not a test — has mtime
**00:07:25**, after the build, and the binary was never rebuilt. If that edit changed
code, every measurement in this package is against a binary that is not the reviewed
one.

**Reproducer attempted.** I found the last stash object predating the build,
`e14bf06` (23:54:23), and diffed it against the reviewed revision:
`git diff e14bf06 f7606cc -- crates/pistol-cli/src/protocol.rs`. The **entire**
difference is `///` doc-comment lines — the `go <budget> <amount> [census]` summary
and two doc paragraphs. `git diff e14bf06 f7606cc --stat -- crates/` shows only three
files: that one, plus `census_identity_tests.rs` (the M10 test fix) and
`wp21_tranche_config_tests.rs` (new) — both integration tests, neither linked into
the `pistol` binary.

**REJECTED.** Doc comments do not reach codegen. `15c94598…` **is** the correct
binary for `f7606cc`'s compiled code, and the byte-identity, determinism and perf
results all hold for the revision under review. (What the exercise did expose is
M2: nothing in any receipt let me establish this, and I had to use objects a `git gc`
would delete.)

### **R2 (REJECTED)** — "the D-537 caveat leaves room to recompute a smaller minimum"

**The suspicion.** `overnight2_ledger.md` §4 records that its two inputs sit in
different columns of `matrix_stage3_detector.md` §5.4 — `0.571` under *per-search
ceiling*, `0.857` under *bound over the columns*. If a different pairing were equally
licensed, the minimum could be argued down, and the ledger's protection would be
nominal.

**Reproducer attempted.** I enumerated every pairing legal under `wp20s_design.md`
§8's item 4 (*"Both figures are read from the trigger-rich band"*). Within that band
the table offers `0.571`, `1.000` and `0.857`. `(0.571, 0.857)` → **n = 28**.
`(0.571, 1.000)` → **n = 6**. There is no legal pairing that yields a larger minimum.

**REJECTED, and the caveat is both RIGHT and SUFFICIENT.** The frame observation is
factually correct (I read the table's headers and its row). The reading the ledger
takes is the **most conservative** admissible one, so the caveat's operative clause —
*"may never simply recompute a smaller minimum"* — is guarding in the right
direction and the number cannot be gamed downward by re-reading the frames. §5.8's
own text also supports the pairing: it says in so many words *"Measured orderings
over those columns reach **0.571** on trigger-rich … against a bound of **0.857**"*,
which is exactly §8 items 2 and 3. **`n = 28, c = 21` stands.**

---

## 5. COUNT BY CLASS

| class | count | ids |
|---|---|---|
| **BLOCKING** | **3** | B1, B2, B3 |
| **MAJOR** | **8** | M1, M2, M3, M4, M5, M6, M7, M8 |
| **minor** | **6** | m1, m2, m3, m4, m5, m6 |
| **REJECTED** | **2** | R1, R2 |

**Verdict: FAIL**, with the reason stated narrowly. Nothing here says a number is
wrong — I re-derived twelve groups of them and every one reproduced, including both
byte-identity referents, all sixteen perf statistics, all fourteen lines of
`wp21_prereg.md` §3's arithmetic, the sixteen-tranche partition and D-537's `n = 28`.
What fails is the registration discipline around a run that was itself clean (B2),
one obligation that fell between three documents (B3), and a ledger line that would
send the next session to the wrong tree (B1). B1 and B3 are edits. B2 is either an
operator ruling or a 23-minute re-run.

**One line for the morning.** The measurements are right and I checked them myself;
the paperwork around them is not — the perf guard's amendment was never reviewed
before the run it governs, the `key_pos` comparison §9 owed has quietly lost every
home it could have had, and the arc ledger points a successor at `8efeda0`, the
revision whose mutation receipt says one mutant is still alive.
