# WP-2.0b — REVIEW-impl, ROUND 3 (REMEDIES-ONLY). Fresh context, not the implementer, not rounds 1 or 2.

**NAMED REVISION** `81a80746807402a6f1f00976e2ee8832391d49f5` — a `git stash create`
object over the uncommitted work on `dev` (HEAD `a6777f4`).

**DOES IT STILL MATCH THE WORKTREE?** **Yes.** `git diff 81a8074 --stat` is empty; no
untracked file is in scope.

**VERDICT: FAIL.**

**The one line.** The census identity defect round 2 called B1 is **still alive at this
revision**: the remedy moved the exchangeable assignment out of one call site and into
two, registered a mutant at neither, and the code's own doc comment now asserts as a
fact — *"there is no call-site assignment left to exchange"* — a universal that
`pvs.rs:762-764` and `search.rs:812-814` falsify on their face. I applied the exchange at
the in-tree site alone, ran **every suite that can see a census row (73 tests, 8 suites)**,
and **all 73 passed**; the mutant then wrote option-A identities onto four of five census
rows on the wire, byte-for-byte reproducing round 2's own demonstration. That is the
SIXTH vacuous criterion in this package, and the fix for the fifth contains it — which is
the pattern D-567 names in its own words.

Everything else in the package is in better shape than that sentence suggests, and the
disposition table says so: **25 of the 30 prior BLOCKING/MAJOR findings are CLOSED**, most
of them by receipts I re-derived rather than read.

---

## 0. WHAT I RAN, SO EVERY CLAIM BELOW HAS A COMMAND BEHIND IT

Worktree `/home/tom/pistol-runs/r3swap` (`git worktree add --detach … 81a8074`), own
`CARGO_TARGET_DIR=/home/tom/pistol-runs/r3swap-target`. No tracked file in the live tree
was modified by this review except this report.

- The **M-SWAP-INTREE mutant** (§B1) and the eight suites that can see a census row.
- Independent re-derivation of the perf statistic, in my own script, from the raw.
- `sha256sum -c artifacts/wp20b_MANIFEST.txt` from the repository root.
- The byte-identity extraction rule, applied by hand to all eight records.
- `tools/file_justification_check.sh`, `tools/decision_key_check.sh`,
  `tools/label_consistency_check.sh` at the reviewed tree.
- `git diff --stat` between every revision any document names and `81a8074`.
- A wire comparison of the clean and mutant release binaries at the armed seat.

---

## 1. DISPOSITION OF EVERY BLOCKING AND MAJOR FINDING IN THE FOUR PRIOR REPORTS

### 1.1 `wp20b_impl_REVIEW_code.md` (round 1, PASS, 2 MAJOR)

| id | what it claimed | disposition | the check I ran |
|---|---|---|---|
| **M1** | no test anywhere reads an IN-TREE census row's `key` | **NOT CLOSED** | This is the round-2 B1 lineage. The in-tree row's key is still checked against nothing externally derived; the new test pins the FUNCTION and the ROOT row. Live mutant, 73 tests green → **B1** below. |
| **M2** | the arena census sink is O(whole artifact) three times over | **CLOSED** | `passes.rs:73-79` streams through a `BufWriter`; `census_file::write_into` (`census_file.rs:53-68`) walks the rows twice and holds one line; `census_file::render` is gone entirely and no `Fixture` body or rendered `String` survives. One resident copy — the design's own §6.2 sink. |

### 1.2 `wp20b_impl_REVIEW_obligations.md` (round 1, FAIL, 3 BLOCKING + 8 MAJOR)

| id | what it claimed | disposition | the check I ran |
|---|---|---|---|
| **B1** | the arc ledger names a revision that is not the one under review, and says every receipt is against it | **CLOSED, with a residue** | `overnight2_ledger.md:44` names `bf1c3ce`; the reviewed revision is `81a8074`. `git diff --stat bf1c3ce 81a8074 -- crates/ tools/ configs/` is **EMPTY** — the only differences are `overnight2_ledger.md`, `wp20b_artifacts.md` and `wp20b_impl.md` (§3.5, the results write-up, which did not exist when the run was taken). That is exactly the terminating rule this project landed at `4375ad9` (`wp20_pilot_artifacts.md`: *"the receipt names the revision it was TAKEN at, and the commit that records it changes documentation only"*), and it is checkable and checked. Residue at **m2** below: no document invokes the rule, so a reader must derive it. |
| **B2** | the perf guard's amendment governs a run it has no reviewed existence before | **NOT CLOSED — same class, new instance** | §3 is now revision 2 and every one of round 2's ten limbs is discharged (see B4). But **no fresh-context review of revision 2 exists anywhere** — not in `docs/experiments/`, not in `artifacts/` — and the run was taken at 14:52 on 2026-09-02. Three documents assert a review that did not happen → **B2** below. |
| **B3** | §9's `key_pos` obligation undischarged | **CLOSED** | Round 2 re-derived all four cells from the raw and found a bijection on 798 rows. `artifacts/wp20b_keypos_*.txt` are in the manifest and their digests verify. |
| **M1** | the reported statistic is not the registered one | **CLOSED** | §3.3 registers ratio-first-then-median; `wp20b_perf_report.py:75` computes `statistics.median(a / b for a, b in zip(num[n], den[n]))` per position and then `statistics.median(value.values())`. **The instrument computes the statistic §3 registers.** I re-derived it in my own script from `wp20b_perf_guard_v2.txt`: H1 **0.9982**, ON/OFF **0.9994**, per-position spread `[0.9878, 1.0043]`, 0 of 20 outside the band, time_ms both **1.0000** — every figure in the receipt, to the digit. |
| **M2** | every receipt names a revision whose tree does not contain the work | **CLOSED** | Both receipts now name `bf1c3ce`, a stash object whose tree contains the diff, at which the measured binary `7a7a2347…` was built. `a6777f4` appears only inside the records' own `revision` line, which the extraction rule strips. |
| **M3** | `wp21_prereg.md` §3 attributes `347 distinct` to an artifact that does not contain it | **CLOSED** | `wp21_prereg.md:82-89` separates the four inputs and says in terms that the 347 is read from `wp20_CLOSURE.md`, not the run log. I also re-derived every rate and total in §3 (records 256 104, distinct 119 768, capture 62.99 h, per-tranche 3.94/0.52/0.13 h): all reproduce. |
| **M4** | the registering sentence miscounts what it registers | **CLOSED** | §3.1:78-86 states the true form (workload from §9, instrument is not) and the *"unchanged"* is gone; §3.2 lists the third arm, the four counters and the rotation. |
| **M5** | the ledger makes gate and test claims with no cited log output | **CLOSED** | The rows now cite `artifacts/wp20b_ci_closure_v1.txt`, which exists, contains all nineteen `=== gate N/19` lines, `ci: all gates passed` and `EXIT=0`. |
| **M6** | the amendment changed the run's cost by half and did not restate it | **CLOSED** | §3.3:163-166 restates it for REPS=6: ~27 minutes of search. I summed `time_ms` over the governed raw: **1 622 412 ms = 27.04 min**. And the MEASURED input it extrapolates from reproduces too: v1's raw sums to **1 356 330 ms = 22.605 min**. |
| **M7** | a `tools/` change landed without the review `docs/process.md` requires for it | **CLOSED by this report** | §5 below answers `tools/SHELL_CHECKLIST.md` items **1-12 by name** for `tools/wp21_tranche_config.py`. |
| **M8** | the instrument exists only in an ephemeral scratchpad | **CLOSED** | Both files are under `artifacts/`, in the manifest, and their digests **equal the ones §3.1 registers** (`648a96d8…`, `d29222be…`) — verified with `sha256sum`. `wp20b_perf_guard.sh:22` now takes its root from `git rev-parse --show-toplevel` instead of a hardcoded path (round-2 m2r2 closed with it). |

### 1.3 `wp20b_impl_REVIEW_code_r2.md` (round 2, FAIL, 1 BLOCKING + 4 MAJOR)

| id | what it claimed | disposition | the check I ran |
|---|---|---|---|
| **B1** | the in-tree identity column is pinned by nothing; M-SWAP alive | **NOT CLOSED — the remedy relocated the defect** | See **B1** below. 73 tests green on the mutant; the wire output is byte-identical to round 2's. |
| **M1** | `# derived rows` pinned only at ZERO | **CLOSED** | `census_capture_tests.rs:133-157` reads the header's count off a **non-empty** census and compares it to the body's own line count — an externally derived referent. Registered mutant M28 (`fixture.derived("rows", rows.min(0))`) dies at it in `wp20b_mutants_v7.txt`. |
| **M2** | (OVERTAKEN) the registered instrument existed nowhere | **CLOSED** | Both instrument files exist, are manifested, and carry the registered digests. |
| **M3** | §6's MEASURED suite costs false by 45x and 17x | **CLOSED** | §6:377-378 now states 84.4 s and 83.8 s and says why the earlier figures were overtaken. **My own runs at this revision: `census_protocol_tests` 84.67 s, `census_identity_tests` 83.57 s.** Reproduces. |
| **M4** | the byte-identity receipt attests a binary that is not a build of the revision under review | **CLOSED** | The receipt is re-taken at the closure binary `7a7a2347…`, which is `target/release/pistol` today (verified by `sha256sum`), built 13:02 — after the last source edit at 11:53 and before the 14:23 records. I applied the extraction rule by hand to all eight records: four `gate_v0` records → `81e37d42…`, four `instrument_v0` → `c7f155e8…`. **Both referents MATCH, on both binaries, both runs.** |

### 1.4 `wp20b_impl_REVIEW_obligations_r2.md` (round 2, FAIL, 5 BLOCKING + 7 MAJOR)

| id | what it claimed | disposition | the check I ran |
|---|---|---|---|
| **B1** | the committed manifest names a digest the file does not have | **CLOSED** | `sha256sum -c artifacts/wp20b_MANIFEST.txt` from the repository root: **39 of 39 OK, EXIT=0**, no line other than `: OK`. |
| **B2** | the mutation receipt the package stands on is named in no document, and two documents state the opposite | **NOT CLOSED — half-remedied, and the halves now contradict each other** | `overnight2_ledger.md:37` and `wp20b_artifacts.md` both name `wp20b_mutants_v7.txt` (27/27). But `wp20b_impl.md:313` still says *"Twenty-six mutants"*, `:332-336` still says **"THE RECEIPT THIS PACKAGE CLOSES ON IS RUN 5"** at revision `595f004`, and `overnight2_ledger.md:62` still calls run 5 **"THE RECEIPT"** — contradicting its own row 37 eleven lines earlier. `git diff --stat 595f004 81a8074 -- crates/ tools/ configs/` shows **eleven source and test files apart**, including `census.rs` and `census_identity_tests.rs`. → **M2** below. |
| **B3** | the revision table names the wrong revision for run 3; its last row points at a row that does not exist | **CLOSED (limb 1), stale (limb 2)** | Limb 1: the table now reads `1cd3364`, which is `wp20b_mutants_v3.txt`'s own header line. Verified against all seven run headers. Limb 2: the last row still reads *"code pending"* for round 2, whose code report has existed since 11:09 → **m1** below. |
| **B4** | `wp20b_impl.md` §3 MAY NOT GOVERN A RUN, on ten limbs | **CLOSED as to all ten limbs** | (a) one estimator, one decision rule, and the instrument computes it — verified against the source and by re-derivation. (b) the agreement clause is DELETED (§3.3:135-141). (c) both files named with digests that match (§3.1). (d) *"unchanged"* deleted; the exported file is named as the authority (§3.1:66-67). (e) the rotated instrument is dry-run: `wp20b_perf_dryrun_v3.txt`, 360 rows, 0 refusals, 20/20/0 on all three arms, 18/0/0 census rows — §3.4's table reproduces cell for cell. (f) `REPS=6`; the residual is stated and is exactly zero (each arm in each slot twice, read off the 18 `arm` lines). (g) the post binary's digest is in the receipt. **What is NOT closed is whether §3 revision 2 was itself reviewed before the run → B2.** |
| **B5** | the manifest offers, as evidence of reproducibility, an act that never happened | **CLOSED** | `wp20b_artifacts.md:49-59` retracts it in terms: *"NOBODY HAS DONE THAT AND THIS DOCUMENT PREVIOUSLY SAID SOMEONE HAD"*. |
| **M1r2** | the exported instrument is not the one that produced the raw; the receipt's digests name files nobody holds | **CLOSED** | The guard was last written 11:02, the report script 10:58, the dry run 11:05, the governed run 14:52 — so the raw was produced by the files at the registered digests, and those digests are what is on disk now. |
| **M2r2** | the ledger's rule-9 row quotes a tool output that no longer reproduces | **CLOSED** | The row is gone. The quoted output reproduces anyway: I ran `tools/file_justification_check.sh` at the reviewed tree — *"355 tracked .rs/.sh files, 64 over the cap, all registered … (64 entries)"*, EXIT=0. |
| **M3r2** | both receipts name a revision whose tree lacks the work | **CLOSED** | Both now read `CLOSURE REVISION bf1c3ce…`. |
| **M4r2** | the `key_pos` conclusion overstates and forecloses the option its own measurement makes live | **CLOSED** | `overnight2_ledger.md:102-113` retracts (*"AN EARLIER DRAFT OF THIS LINE SAID IT DID"*), keeps F2's a-priori exclusion of option A, and names **D'** as what the measurement makes live, with the matrix's own cost row. |
| **M5r2** | the manifest describes the two failed mutation runs as successes | **CLOSED** | `wp20b_artifacts.md` now names v3's live mutant, v4's abort, and v6's two harness faults. I checked each against the file's own last line. |
| **M6r2** | `SHELL_CHECKLIST.md` unanswered by name; item 12 limb 3 unmet | **CLOSED** | `wp21_tranche_config_tests.rs:28-48` defines `REFUSED = 1` / `VOID = 2` and `refused()` asserts `assert_eq!(code, Some(REFUSED))` with a message naming what 2 and 0 would have meant — item 12 obligation 3, verbatim. All twelve items answered in §5. |
| **M7r2** | the ledger's obligation table shows the perf guard discharged and owes no re-run | **NOT CLOSED** | `overnight2_ledger.md:36` now reads *"perf guard, under a **REVIEWED** registration"*. The registration was not reviewed before the run → **B2**. |

### 1.5 MINORS SAMPLED

Sampled: round-1 code m1-m8, round-2 code m1-m7, round-2 obligations m1r2, m2r2.
**Closed and verified**: round-1 code m1 (`error.rs` refusal is one line — `M20` dies at
its test in v7), m2 (every fallible write precedes every `println!` in
`passes.rs:73-98`), m3 (`census_sha256` takes `format_version` as a parameter and folds
it), m4 (`capture.rs:302-309` refuses a tabbed row by name), m5/M8 (exported), m7
(`CensusNone` seat + `# derived rows 0`), round-2 code m2 (`render` is gone), m3 (§4 now
says *"BY INSPECTION … and did not run an 8 GB capture"*), m4 (`capture.rs:288-295`
refuses an unasked census row by name; `M29` dies at it), m5 (the rule-9 entry now names
the cases that take their own search), round-2 obligations m1r2 (§2's row reads *"the
**DESIGN's** §5"*), m2r2 (`git rev-parse --show-toplevel`).

**Still open and reported below**: round-1 code m6 / round-2 code m6 (WP-2.1 work in the
same tree → **m5**), round-1 obligations m8's ADR half / round-2 code m7 (→ **m4**).

---

## 2. NEW FINDINGS

### **B1 (BLOCKING)** — the identity remedy moved the exchangeable assignment from one call site to two, registered a mutant at neither, and the code's own doc asserts the opposite; the option-A identity still reaches every in-tree census row with 73 tests green

**Claim.** `crates/pistol-search/src/census.rs:8-12` states, as the reason the type
exists:

> *"With the derivation here there is no call-site assignment left to exchange: a site
> hands over a state and takes back both columns, and any exchange has to happen inside
> [`CensusKeys::at`], where a test pins each field against an independently computed
> referent."*

**Both halves are false.** The two firing sites do not "take back both columns" as a
unit — they destructure the pair and assign the fields **one by one**:

```rust
// crates/pistol-search/src/pvs.rs:762-764   (the IN-TREE site)
census.push(crate::census::TriggerObservation {
    key: keys.key,
    key_pos: keys.key_pos,

// crates/pistol-search/src/search.rs:812-814  (the ROOT site)
rows.push(crate::census::TriggerObservation {
    key: keys.key,
    key_pos: keys.key_pos,
```

Those are exactly the call-site assignments the comment says do not exist, and
`artifacts/mutants.py:185-190` registers **M27 at `census.rs` only** — the one of the
three places the defect no longer needs.

**Sites.** `crates/pistol-search/src/census.rs:8-12`, `:29-36`;
`crates/pistol-search/src/pvs.rs:762-768`; `crates/pistol-search/src/search.rs:812-818`;
`crates/pistol-search/tests/census_identity_tests.rs:151-186` (pins the FUNCTION),
`:188-222` (pins the ROOT row's two columns, and for every other row only
`assert_ne!(row.key, row.key_pos)` — which an EXCHANGE preserves, because an exchange
does not make two distinct values equal);
`crates/pistol-cli/tests/census_protocol_tests.rs:143-164` (pins the root row's value
against `fixture_key()`, and for in-tree rows only "32 lower-case hex digits", which
`GameState::key` also is).

**Round 2 said what the remedy had to be**, in its own words: *"assert on the ROWS that
`row.key == canonical_key_of(row's stones) != row.key_pos` for a row with
`turns_from_root > 0` … the criterion must be one the option-A defect cannot satisfy by
relabelling a column"*. **No such assertion exists.** The in-tree row's identity is still
compared to nothing externally derived.

**FAILURE SCENARIO.** Unchanged from round 2's, and now one function further from where
anyone is looking. A production sweep's census rows all carry `GameState::key` except the
one root row per search. Transpositions fold; **symmetries do not** — option A, which
design F2 rules out a priori. D-537's *disjoint positions* denominator is then computed
over the wrong equivalence and **over-counts**, in the direction F2 says the rule exists
to prevent. §9's `key_pos` measurement is corrupted at the same stroke: the two columns
are exchanged, so *"did the in-tree symmetry fold merge anything"* returns its own answer
backwards — and the answer this arc recorded is ZERO on 798 firings, which is precisely
the reading an exchange is invisible under.

**MINIMAL REPRODUCER — RUN.**

```
git worktree add --detach /home/tom/pistol-runs/r3swap 81a8074
# MUTANT M-SWAP-INTREE — crates/pistol-search/src/pvs.rs:763-764, the IN-TREE site ONLY:
-            key: keys.key,
-            key_pos: keys.key_pos,
+            key: keys.key_pos,
+            key_pos: keys.key,
```

The population that can see a census row, enumerated with round 2's own command
(`/usr/bin/grep -rln "TriggerObservation\|take_trigger_census\|info census\|census_file\|census_sha256\|CensusRequest\|--census\|CensusKeys" crates/*/tests/`)
plus `canonical_key_tests`:

| suite | against M-SWAP-INTREE |
|---|---|
| `pistol-search --test census_identity_tests` | **6 passed, 0 failed** (83.57 s) |
| `pistol-cli --test census_protocol_tests` | **14 passed, 0 failed** (84.67 s) |
| `pistol-arena --test census_capture_tests` | **14 passed, 0 failed** (12.65 s) |
| `pistol-core --test canonical_key_tests` | **7 passed, 0 failed** |
| `pistol-cli --test report_tests` | **11 passed, 0 failed** |
| `pistol-cli --test movetime_tests` | **5 passed, 0 failed** |
| `pistol-engine --test engine_tests` | **13 passed, 0 failed** |
| `pistol-search --test trigger_census_cover_tests` | **3 passed, 0 failed** (388.12 s) |

**73 tests across 8 suites, 0 failures. The mutant is alive**, including in both tests written to kill it —
`each_identity_column_is_the_derivation_it_is_named_for` (which tests the function the
mutant does not touch) and
`every_census_row_carries_the_two_columns_that_function_produces` (which tests the root
row the mutant does not touch, and then asserts on the in-tree rows only the inequality
the mutant preserves).

**AND IT IS ON THE WIRE.** Release builds of the reviewed tree and of the mutant, seat
`configs/gate_staged_solver_v0.toml`, the suite's own committed fixture,
`go nodes 4000 census`, printing `key` and `turns_from_root`:

```
clean (7a7a2347…)                     M-SWAP-INTREE
8adc7560f93b697753dc430a63ea6e8e 0    8adc7560f93b697753dc430a63ea6e8e 0   <- root, unchanged
03017106e7fbac162e2edc7afc0b4dfc 2    506195b918c7cef3ee11c283cd5893ac 2
f2b158b9f1f63963e09c19ebd42191fb 2    3036b2ac796d923ad5d05f4814035a10 2
f2b158b9f1f63963e09c19ebd42191fb 2    3036b2ac796d923ad5d05f4814035a10 2
a9197d8867e91dbe1f6ee6ca502c1a28 2    99f3cf90ba3868cc50a398c54598efca 2
```

**These are round 2's own bytes, digit for digit.** Four of five rows carry a different
identity and the root row is untouched — the remedy changed where the exchange is written
and changed nothing about what the suite can see.

**What would close it.** Round 2 already wrote the criterion; it still has not been
implemented. Anything that reads an IN-TREE row's `key` against a referent derived
outside the search — replay the row's position and take `canonical_key` of it — plus a
registered mutant at **each** of the two push sites, since a mutant registered only
inside `CensusKeys::at` certifies nothing about the sites that consume it. And the
`census.rs` doc comment must stop asserting a universal its own callers falsify: it is
the sentence that would have stopped a reader looking.

---

### **B2 (BLOCKING)** — §3 revision 2 governed a run that no fresh-context review preceded, and three documents state that one did

**Claim.** CLAUDE.md's Process: *"A pre-registration is reviewed at the revision that
GOVERNS the run — that revision must itself pass a fresh-context review before the first
run it governs, and reviews of superseded revisions do not transfer."* §3 revision 2 was
written after round 2 (which ruled revision 1 *"MAY NOT GOVERN A RUN"*), and the governed
run `wp20b_perf_guard_v2.txt` was taken at **14:52:31** on 2026-09-02. **No review of
revision 2 exists.** `ls docs/experiments/*REVIEW*` and `ls artifacts/*.md` hold nothing
between round 2 (11:09) and the run; the manifest indexes no such artifact; this report
is the first fresh context to read revision 2, and it is reading it **after** the run.

Three documents say otherwise:

- `artifacts/wp20b_perf_RECEIPT.txt:5-6` — *"§3, REVISION 2, **which passed a
  fresh-context review BEFORE this run**"*
- `docs/experiments/wp20b_impl.md:194` — *"Taken after this registration **passed its
  review**"*
- `docs/experiments/overnight2_ledger.md:36` — *"perf guard, under a **REVIEWED**
  registration"*

**FAILURE SCENARIO.** A successor reads the receipt, concludes the perf verdict is a
governed sample, and closes WP-2.0b's hard-rule-5 obligation on it. It is not: the
registration that decides the verdict was written and applied by the same session, which
is the independence CLAUDE.md's rule exists to buy. The specific hazard is on the record
in round 2's own limb 2 — the decision rule §3 revision 2 chose (aggregate, per-position
values *"reject nothing"*) is the rule under which the **first** run's position 10 at
`1.0215` does not reject H1, and that fact was on disk when revision 2 was written.

**I state the mitigation as plainly as the finding, because it bounds what this costs.**
Revision 2 discharges all ten of round 2's limbs; the instrument computes the registered
statistic; and **the conclusion does not move under any of the alternatives**: H1 is
`0.9982` under the registered estimator, `0.9984` under ratio-of-medians, and 0 of 20
positions fall outside the band, so the aggregate and any-position rules agree. The
finding is not that the number is wrong. It is that **three documents claim a review that
did not happen**, and this project's rule is that a report's claim cites its own output.

**MINIMAL REPRODUCER — RUN.**

```
$ git show -s --format=%ci bf1c3ce      # the revision the receipt names
2026-09-02 14:22:51 +0200
$ ls -la --time-style=full-iso artifacts/wp20b_perf_guard_v2.txt
… 2026-09-02 14:52:31 …                 # the governed run
$ ls -la docs/experiments/*REVIEW*.md | awk '$6=="2026-09-02"'
… 00:45 wp20b_impl_REVIEW_obligations.md
… 00:49 wp20b_impl_REVIEW_code.md
… 10:56 wp20b_impl_REVIEW_obligations_r2.md
… 11:09 wp20b_impl_REVIEW_code_r2.md     # nothing between 11:09 and 14:52
$ /usr/bin/grep -rl "REVISION 2" artifacts/*.md ; echo "exit $?"
exit 1
```

**What would close it.** Either delete the three sentences and record that revision 2's
first fresh-context review is round 3's — in which case the run predates its review and
the honest disposition is that it does not govern, and the 27-minute re-run is owed — or
obtain the review before the run. There is no third reading that leaves the three
sentences standing.

---

### **M1 (MAJOR)** — `wp20b_artifacts.md` is not a sha-index: the committed document carries **zero** digests, and its own sentence about itself cannot be satisfied

**Claim.** `docs/experiments/wp20b_artifacts.md:16-18` reads:

> *"The live digest list is `artifacts/wp20b_MANIFEST.txt`, regenerated at closure; **the
> table below is the committed copy of it**. A file whose digest **disagrees with this
> table** is not the file the closure read."*

The table below it has two columns — file, and what it is. `/usr/bin/grep -cE
'[0-9a-f]{64}' docs/experiments/wp20b_artifacts.md` returns **0**. There is no digest in
the committed file, so the second sentence names a comparison nobody can make, and the
first is false.

**Sites.** `docs/experiments/wp20b_artifacts.md:16-18` and its table;
`.gitignore:19` (`/artifacts/`); `overnight2_ledger.md:39` (*"artifact manifest (rule 8,
D-469) | committed, and every digest verifies"* — the digests that verify are in the
**un**committed list).

**FAILURE SCENARIO.** This is the document's own stated purpose, failing in the exact
manner it names: *"Without it, the evidence for every number this package's closure
states would be one `rm -rf` from not existing"*. `rm -rf artifacts/` removes both the
artifacts **and** `wp20b_MANIFEST.txt`, and the committed file that survives holds not
one digest to check a recovered copy against. The project's own precedent shows the
shape this is meant to have: `docs/experiments/wp20_pilot_artifacts.md` carries a
`| sha256 | file | what it is |` table with the digests in it.

**MINIMAL REPRODUCER — RUN.**

```
$ git check-ignore -v artifacts/wp20b_MANIFEST.txt
.gitignore:19:/artifacts/	artifacts/wp20b_MANIFEST.txt
$ git ls-files docs/experiments/wp20b_artifacts.md
docs/experiments/wp20b_artifacts.md
$ /usr/bin/grep -cE '[0-9a-f]{64}' docs/experiments/wp20b_artifacts.md
0
```

**What would close it.** Put the digests in the committed table — the manifest's own
thirty-nine lines fit — or delete the two sentences and the D-469 row and say the
artifacts are unindexed. Both are honest; the present state is neither.

---

### **M2 (MAJOR)** — the implementation account and the ledger's revision table both name mutation run **5** as the receipt this package closes on, at a revision eleven source files away from the reviewed one, while the same ledger's obligation row names run **7**

**Claim.** `docs/experiments/wp20b_impl.md:313` opens §5 with *"Twenty-six mutants"*;
`:332-336` states **"THE RECEIPT THIS PACKAGE CLOSES ON IS RUN 5,
`artifacts/wp20b_mutants_v5.txt`: 26 registered, 26 dead …, at revision `595f004`, whose
tree is byte-identical to the reviewed `eff179b`"**; and its table at `:341-347` ends at
v5. `overnight2_ledger.md:62` repeats it: *"mutation run 5 … **THE RECEIPT** … the
receipt covers the reviewed code exactly"*. Eleven lines earlier, `:37` says the
obligation is discharged by **`wp20b_mutants_v7.txt`, 27 registered, 27 dead**.

Runs 6 and 7 appear in no table in either document; §5 does not mention them.

**Sites.** `docs/experiments/wp20b_impl.md:311-355`;
`docs/experiments/overnight2_ledger.md:37`, `:55-63`.

**FAILURE SCENARIO.** A successor asked which mutation run certifies this code reads §5 —
the section that owns the mutation account — and takes `595f004`. That tree differs from
the reviewed one in **eleven source and test files**, `crates/pistol-search/src/census.rs`
and `crates/pistol-search/tests/census_identity_tests.rs` among them: the very files the
round-2 remedies rewrote. Run 5 knows nothing about `CensusKeys`, M27, M28 or M29. The
sentence *"the receipt covers the reviewed code exactly"* is false at this revision, and
it is false in the one document a successor is told to continue from.

**MINIMAL REPRODUCER — RUN.**

```
$ git diff --stat 595f004 81a8074 -- crates/ tools/ configs/ | tail -1
 11 files changed, 232 insertions(+), 75 deletions(-)
$ head -1 artifacts/wp20b_mutants_v7.txt
worktree /home/tom/pistol-runs/wp20bmut  revision 7103ad1604cf75e7ee1684ac0bc1fdc3d94493aa
$ git diff --stat 7103ad1 81a8074 -- crates/ tools/ configs/   # empty: v7 DOES cover it
$ tail -2 artifacts/wp20b_mutants_v7.txt | head -1
=== MUTANTS COMPLETE: 27 registered, 27 dead at their registered test, 0 dead elsewhere, 0 alive, 0 harness fault(s) ===
```

**The good news is in the same reproducer**: `7103ad1`'s tree is code-identical to the
reviewed revision, so run 7 genuinely covers this code and the receipt the package needs
exists. What is wrong is that two documents point at the wrong one. (It certifies 27
mutants none of which is the call-site exchange — that is **B1**, not this finding.)

---

### **m1 (MINOR)** — the ledger's revision table still records round 2's code report as *"code pending"*

`docs/experiments/overnight2_ledger.md:63`. The report has existed since 11:09 and
returned FAIL (1 BLOCKING, 4 MAJOR, 7 minor). Round-2 obligations B3's second limb, in a
new spelling.

### **m2 (MINOR)** — the ledger names a closure revision that is not the reviewed one and does not say the difference is documentation only

`docs/experiments/overnight2_ledger.md:44` — *"THE CLOSURE REVISION IS `bf1c3ce…` … Every
receipt above names it"*. True, and `81a8074` is a docs-only descendant of it
(`git diff --stat bf1c3ce 81a8074 -- crates/ tools/ configs/` is empty), which is exactly
the terminating rule landed at `4375ad9`. But no document cites that rule here, so a
reader who notices the mismatch has to derive the check rather than being handed it. One
sentence closes it.

### **m3 (MINOR)** — a registered dry-run criterion is stated in a form the registered instrument can never satisfy, and was reported met

`docs/experiments/wp20b_impl.md:183` registers *"**18 `arm` lines in six distinct
orders**"*; `docs/experiments/wp20b_artifacts.md:44` repeats *"six reps in six distinct
arm orders"*. The rotation has a **three**-cycle, so `REPS=6` produces three distinct
orders, each twice — which is what the raw shows and what the same row's result cell
correctly states (*"each arm in each slot twice"*). The check that was actually made is
the right one and does exclude the named defect; the registered wording is unmeetable.
In a package whose recurring class is a criterion that says something other than what is
checked, this is worth one word.

### **m4 (MINOR)** — no ADR line records anything this package decided (hard rule 10)

`git diff a6777f4 81a8074 -- docs/decisions.md` adds **D-565, D-566, D-567 only** — the
loop grant, the branch deletion, the operator's stop ruling. Nothing records the census
artifact class, `Step::Census`, the `--census` flag, `CENSUS_FORMAT_VERSION`, the `go`-line
token, or `CensusKeys`. Round-1 obligations m8 and round-2 code m7, still open. Mitigated:
`overnight2_ledger.md:42` lists the closure D-line as owed after round 3, which is a
legitimate place for it — but it is now the third round in which it is owed.

### **m5 (MINOR)** — WP-2.1's work is still in the same uncommitted tree as WP-2.0b

`tools/wp21_tranche_config.py`, `crates/pistol-arena/tests/wp21_tranche_config_tests.rs`,
`docs/experiments/wp21_prereg.md`, `docs/book_v2_ledger.md`'s new row. Round-1 code m6,
round-2 code m6. CLAUDE.md's Closure section: *"One feature = one commit"*. The arc
framing (one dispatch, phases 1 and 2) is a reasonable answer; it is not written down as
one.

### **m6 (MINOR)** — `tools/wp21_tranche_config.py`'s VOID exit is driven by no test

`tools/wp21_tranche_config.py:189-191` exits 2 on an `OSError` while writing. The suite
defines `VOID = 2` (`wp21_tranche_config_tests.rs:35`) but uses it only inside a failure
message; no test makes the script take that path. Item 12's own point is that the void
class is what a reader misreads as a regression, and an exit class nothing exercises is a
class nothing defends. A read-only `--out` directory is a two-line test.

### **m7 (MINOR)** — `wp21_prereg.md` cites a section it does not have

`docs/experiments/wp21_prereg.md:192` — *"§0's ruling"*. The document's sections run 1-8;
the census-off ruling is in the unnumbered preamble at `:14-20`. Round-2 obligations
m1r2's class, in the sibling document.

---

## 3. FINDINGS I ATTEMPTED AND **REJECTED**

### **R1 (REJECTED)** — *"the CI receipt does not cover the reviewed tree"*

`artifacts/wp20b_ci_closure_v1.txt` finished at 13:09; `docs/decisions.md` was last
written at 12:39 and gate 18 reports **568** decision keys where the tree now holds
**569**, so D-567 was appended after that gate ran. **It does not survive contact**: the
last change to `crates/`, `tools/` or `configs/` was at 11:53:18, before the run, and
gate 2 builds `git checkout-index`, which equals the fully staged index. The only gates
that read the changed files are 17-19, and I ran all three at the reviewed tree —
`file_justification_check` (355 files, 64 over the cap, 64 entries), `decision_key_check`
(**569** keys, no repeat), `label_consistency_check` (6 documents, all agreeing) — each
EXIT=0. Nothing about the delta could have changed a gate's answer, and the two that
could see it now pass on it.

### **R2 (REJECTED)** — *"the streaming `Sha256` loses bits or depends on where the pieces are cut"*

Read `crates/pistol-cli/src/sha256.rs:53-101` against the padding rule. The zero count is
written as `(55 + 64 - (length % 64)) % 64`, which is the correct `55 − L (mod 64)`
without the underflow the comment names; the padding is fed through `update`'s own
block-splitting path so there is one rule and not two; `update` returns early when a
partial block stays partial, which is the one place a piece could be dropped, and the
comment says so. Round 2 attacked this at every boundary and against an external
referent and found it correct; I found nothing it missed. The one-shot is the streaming
form fed once, so the published vectors that pin one pin the other.

### **R3 (REJECTED)** — *"the `# derived rows` count and the unasked-census-row refusal are the same vacuity in new clothes"*

Both were built as the dispatch asked me to suspect, and both hold.
`the_headers_row_count_is_the_number_of_rows_the_file_holds`
(`census_capture_tests.rs:133-157`) reads the claimed count off a **non-empty** census and
compares it to the body's own line count — a referent derived by counting the artifact,
not by asking the writer — so the constant-zero mutant M28 and any off-by-one die at it.
`a_census_row_on_a_capture_that_asked_for_none_refuses_the_run_by_name` (`:273-289`) drives
a stub that emits a row on a census-off capture and asserts the run refused, that neither
file exists, and that the message names what it refused; M29 (call-removed) dies at it.
Neither criterion is one its named defect preserves.

---

## 4. RECEIPTS, RE-DERIVED RATHER THAN READ

| receipt | claim | what I got |
|---|---|---|
| `wp20b_ci_closure_v1.txt` | CI, 19 gates, `EXIT=0` | 19 `=== gate N/19` lines, one `ci: all gates passed`, `EXIT=0`. **CONFIRMED**, plus R1 above. |
| `wp20b_identity_RECEIPT.txt` | gate-OFF byte identity MATCHES `81e37d42…` / `c7f155e8…` at binary `7a7a2347…` | Applied `sed -n '1,/^# timing/p' \| grep -v '^revision \|^binary_sha256 ' \| sha256sum` to all eight records myself: four → `81e37d42…`, four → `c7f155e8…`. `sha256sum target/release/pistol` = `7a7a2347…`. **CONFIRMED.** |
| `wp20b_determinism_v2.txt` | 5 seats, `EXIT=0` | Five `seat …: ok` lines, `5 seat(s), no difference outside nps/time`, `EXIT=0`. **CONFIRMED.** |
| `wp20b_perf_RECEIPT.txt` + `_guard_v2.txt` | H1 = 0.9982, not rejected; no abort | Re-derived in my own script from the 360 raw rows: H1 **0.9982** `[0.9878, 1.0043]` 0 outside; ON/OFF **0.9994** `[0.9863, 1.0077]`; time_ms **1.0000** / **1.0000**; totals 9 580 300 / 9 579 861 / 9 647 855; 0 refusals; 181/0/0 census rows on 18 arm lines, each arm in each slot twice. **Every figure CONFIRMED to the digit.** (Governance: **B2**.) |
| `wp20b_mutants_v7.txt` | 27 registered, 27 dead at their registered test, 0 alive, 0 faults | Last line states all five numbers; run revision `7103ad1`, whose `crates/ tools/ configs/` is **identical** to the reviewed tree. **CONFIRMED** — and it registers no mutant at either census push site, which is **B1**. |
| `wp20b_MANIFEST.txt` | every digest verifies | `sha256sum -c` from the repository root: **39/39 OK, EXIT=0**. **CONFIRMED.** (What it is *committed* as: **M1**.) |
| `wp20b_perf_dryrun_v3.txt` | §3.4's five criteria | 360 rows, 0 refusals, 20/20 totals and bestmoves on every arm, 0 empty-board answers, 18/0/0 census rows identical across six reps, 18 arm lines. **CONFIRMED** (wording: **m3**.) |
| `wp21_prereg.md` §3 | the arithmetic | Every rate and total re-computed: 57.0769 / 26.6923 / 2.1383 / 0.8854 / 0.8271; 8 974 games, 256 104 records, 119 768 distinct, 62.99 h; per tranche 3.94 / 0.52 / 0.13 h; partition `7×281 + 9×280 = 4487 = 4500 − 13`. **CONFIRMED.** |

---

## 5. `tools/SHELL_CHECKLIST.md`, ANSWERED BY NAME FOR `tools/wp21_tranche_config.py`

Round 2 discharged items 8-11 and found item 12 limb 3 unmet. Here is the whole list, so
`docs/process.md`'s coverage rule is satisfied by a reviewer who read all twelve.

1. **Command substitution whose status is discarded** — **N/A.** Python; no `subprocess`,
   no shell.
2. **Pipeline in a `then` body** — **N/A.**
3. **`grep` under `pipefail`** — **N/A.**
4. **`LC_ALL` and which direction it moves a guard** — **SATISFIED.** The one character
   class is `SHA256 = re.compile(r"\A[0-9a-f]{64}\Z")` (`:60`): an explicit ASCII
   ALLOW-list, anchored with `\A`/`\Z` rather than `^`/`$`, so no trailing newline
   sneaks through and no locale can widen it. The write is `encoding="utf-8"`,
   explicit (`:185`).
5. **The index is what commits** — **N/A.** The script reads no tracked bytes.
6. **A sweep by prefix must own the prefix** — **N/A.** Nothing is deleted.
7. **Traps** — **N/A.**
8. **One spelling per number, one refusal per reason** — **SATISFIED, and it is the item
   most at risk here.** `:163-167` refuses `010`, `+5` and ` 5` explicitly
   (`if str(tranche) != args.tranche`), and each reason gets its own refusal:
   not-an-integer (`:160-162`), wrong spelling (`:163`), outside `1..16` (`:168`), bad
   digest (`:170`). Four reasons, four messages, all naming the input.
9. **What reaches a record is caller-controlled** — **SATISFIED.** `document()` (`:82-149`)
   interpolates only `tranche` (an `int`), `skip`/`take` (derived), and `binary_sha256`
   (regex-validated, so it cannot carry a newline). `--out` never reaches the generated
   document, only the refusal messages.
10. **THE COVERAGE RULE** — **SATISFIED.**
    `crates/pistol-arena/tests/wp21_tranche_config_tests.rs:11-24` drives the **shipped**
    script through `python3` at `repo().join("tools/wp21_tranche_config.py")`, in a
    `Scratch` directory, with four refusal cases and three control cases that must
    succeed — so a pass cannot come from a generator that refuses everything. The suite
    runs under gate 3 (`wp20b_ci_closure_v1.txt:374`).
11. **A caller's path that feeds a delete or an overwrite is containment-guarded** —
    **SATISFIED IN SUBSTANCE.** Enumerating the destructive sites: there are none. There
    is no `rm` and no `mv`, and the single write is `open(out, "x")` (`:185`) —
    create-only, so an existing file is never overwritten, with the readable refusal
    above it (`:174-178`) and the race-free `FileExistsError` arm below it (`:187-188`).
    The script never `cd`s, so a relative `--out` resolves against the caller's own
    directory — which is precisely the direction item 11's `baseline_snapshot.sh`
    paragraph asks for. The residual — an absolute `--out` is not containment-checked —
    cannot reach the named attack, because create-only writing of a caller-named path is
    what the caller asked for and destroys nothing.
12. **A gate distinguishes RUN VOID from FAIL, by name** —
    (1) **A code per kind: SATISFIED.** The usage block (`:28-31`) states 0 / 1 / 2 and
    says *"A void is not a refusal"*; `refuse()` exits 1, the `OSError` arm exits `VOID`.
    (2) **Preflight and void early: SATISFIED IN SUBSTANCE.** The script writes one small
    document rather than scratch, and it voids on the write naming the path and the OS
    error. It does not name available space the way `tools/scratch_preflight.sh` does;
    for a single-file write that is proportionate.
    (3) **The distinction survives the seam: SATISFIED — this is round-2 M6r2's remedy
    and it is real.** `wp21_tranche_config_tests.rs:37-48`: `assert_eq!(code,
    Some(REFUSED))`, with a message that spells out what exit 2 and exit 0 would have
    meant. Not `!status.success()`.
    **One gap: no test drives the exit-2 path — m6.**

---

## 6. COUNT BY CLASS, AND WHETHER THIS PACKAGE IS CLOSABLE

**New findings: 2 BLOCKING, 2 MAJOR, 7 minor. 3 rejected.**

**Prior findings disposed: 30 BLOCKING/MAJOR across the four reports** — 2 (r1 code) +
11 (r1 obligations) + 5 (r2 code) + 12 (r2 obligations).

| disposition | count | which |
|---|---|---|
| **CLOSED** | **25** | r1-code M2; r1-obl B1 (residue → m2), B3, M1, M2, M3, M4, M5, M6, M7, M8; r2-code M1, M2, M3, M4; r2-obl B1, B3 (limb 1; limb 2 → m1), B4, B5, M1r2, M2r2, M3r2, M4r2, M5r2, M6r2 |
| **NOT CLOSED** | **5** | r1-code M1 + r2-code B1 (one defect → **B1**); r1-obl B2 + r2-obl M7r2 (one defect → **B2**); r2-obl B2 (→ **M2**) |
| **RE-OPENED WORSE** | **0** | — |

**Is it closable? Not at this revision, and the reason is one finding rather than the
weight of them.**

Twenty-five of thirty prior findings are closed, most of them by evidence I
re-derived rather than accepted: the manifest verifies whole, the byte-identity digests
reproduce by hand on all eight records, the perf instrument computes the statistic its
registration names and every number in its receipt comes back to the digit, the mutation
receipt covers a tree code-identical to this one, the dry run meets its criteria cell for
cell, and `wp21_prereg.md`'s arithmetic is right to the last figure. Round 2's B4 — ten
limbs — is discharged limb by limb. That is a large amount of real repair.

**B1 is what stops it, and it stops it cleanly.** The defect round 2 demonstrated with a
live mutant is alive at this revision, by the same mutant one function away, surviving
the same suites — and the fix for it introduced the sentence in `census.rs` that asserts
the defect is now impossible. That is the sixth vacuous criterion in this package and the
second time a remedy has contained the defect it was written to close. **B2 is
independent of it**: three documents state that a pre-registration passed a review that
no artifact records, and the run that registration governs was taken before this, its
first, review.

Neither is a large amount of work. B1 needs one assertion over an in-tree row's key
against an externally derived referent, plus a registered mutant at each of the two push
sites, plus the deletion of a false sentence. B2 needs three sentences deleted and either
a re-run or an honest statement of what the run does not carry. But under **D-567** this
is a FAIL, and a FAIL stops the arc where it stands — so what this report is for is the
written account, and the two paragraphs above are the part of it a successor needs first.
