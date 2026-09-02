# WP-2.0b — REVIEW-impl (CODE), ROUND 2. Fresh context, not the implementer, not round 1.

**NAMED REVISION:** `eff179bc5b9b77ad03fbab312e31ea87ed9be0f1` — a `git stash create`
object over the uncommitted work on `dev` (HEAD `a6777f4`).

**DOES IT STILL MATCH THE WORKTREE?** **For the CODE under review, YES, throughout. For
the tree as a whole, NO — it moved twice while this review ran, in `docs/` and
`artifacts/` only, and it was still moving as this was written.** Taken independently at
the start, `git add -A && git stash create` returned
`01741ee18590fdcd27ac4b18c105a557761b2e89` and `git diff eff179b 01741ee --stat` was
**empty** — the revision under review was the tree on disk. Taken again at the end it
returned `6023492c5ed2ad794d95b60d1be85335aa93cc64`, whose diff against `eff179b` names
exactly two paths: `docs/experiments/wp20b_impl_REVIEW_obligations_r2.md` (new, 745
lines) and `docs/experiments/wp20b_impl.md` (§3 rewritten as a "revision 2" superseding
the first perf run). While the report was being written, `docs/experiments/wp20b_artifacts.md`,
`artifacts/wp20b_MANIFEST.txt` and `artifacts/wp20b_perf_guard.sh` all changed again.

**`git diff eff179b <current> -- crates/ tools/` is EMPTY at every one of those points**,
so every code finding below is a finding about the tree on disk. The artifact- and
document-side findings are stated **against `eff179b`**, which is what a review at a
named revision states, and the ones the concurrent work has already overtaken are marked
**OVERTAKEN IN FLIGHT** and listed again in *State of the tree at writing* near the end.

**VERDICT: FAIL.** One BLOCKING, and it is round-1 **M1 NOT CLOSED**. The fix round
wrote `the_identity_column_is_never_the_position_key` to close it and installed the
**fifth vacuous criterion of this package's standing defect class**: the test asserts a
property (`key != key_pos`) that the defect it names — the in-tree identity column
carrying `GameState::key`, the matrix's option A — **preserves**. I have that mutant
alive across **70 tests, the whole population that can see a trigger-census row**, and
its effect **on the wire**. A **second** instance of the same class is in the newly
written `census_file.rs`: the header's `# derived rows` count is pinned **only at zero**,
and a mutant that writes `rows 0` over a 108-row census survives 51 tests.

**THE CODE ITSELF IS CORRECT AT BOTH FIRING SITES AND IN THE FILE WRITER.** Nothing
below says the shipped build produces a wrong answer. What is unmet is coverage the
design's own §8 test 1 and D-553's call-removed law claim, at the call site whose output
D-537's denominator will be counted over. **Both remedies are tests, not code changes**,
and both are given as verified one-liners.

**SCOPE.** The WP-2.0b diff: `crates/pistol-{core,search,engine,cli,arena}`, plus the
artifacts and documents §9 registers. `tools/wp21_tranche_config.py` and its test are
out of scope for the second time (m6 below).

**WHAT I RAN.** A detached worktree at `eff179b` under `/home/tom/pistol-runs/` with its
own `CARGO_TARGET_DIR` (never the live tree, never `/tmp`), removed at the end with
nothing gitignored in it to export. Clean-tree: `clippy --workspace --all-targets -D
clippy::all` (clean); `census_identity_tests` 5/5 (83.75 s), `census_protocol_tests`
14/14 (84.42 s), `census_capture_tests` 12/12 (10.27 s), `capture_tests` 39/39,
`canonical_key_tests` 7/7, `report_tests` 11/11, `movetime_tests` 5/5, `engine_tests`
13/13, `trigger_census_cover_tests` 3/3 (386 s). Plus: an independent SHA-256 conformance
probe against `python3 hashlib`, a from-source release build of `eff179b` and a **re-take
of the gate-off byte-identity obligation**, and two live mutants.

---

## THE THINGS THE DISPATCH ASKED ME TO ATTACK, AND WHAT THEY RETURNED

### 1. The streaming SHA-256 — **CORRECT. Attacked at every boundary and against an external referent.**

`crates/pistol-cli/src/sha256.rs`. I re-derived the padding rule rather than reading it:
with `L = length % 64`, the code's `zeros = (55 + 64 - L) % 64` is `(119 - L) % 64`, which
for `L ∈ [0,63]` equals `55 - L (mod 64)` — so `L + 1 + zeros ≡ 56 (mod 64)` and
`L + 1 + zeros + 8 ≡ 0`, exactly the spec. The `usize` underflow the author fixed is
genuinely gone for every `L`, including 56–63 where naive `56 - L` breaks. The
invariant `buffered == total_fed % 64` holds on all three paths of `update` (early
return, exact fill, no carry), which is what makes `finish_hex`'s `zeros` right, and
`update(&[])` with a partial block held is a no-op rather than a truncation.

Then I ran it. `crates/pistol-cli/examples/sha_probe.rs` (written in the worktree,
removed with it):

```
for n in 0..=300:  one-shot digest, EVERY 2-way split (0..=n), strided 3-way splits,
                   one byte at a time, and empty updates interleaved between every byte
for piece in [1,63,64,65,127,128,129,1000]: a 100 003-byte payload fed in that piece size
```

**`disagreements 0`**, and all 301 one-shot digests compared equal to
`hashlib.sha256(payload).hexdigest()`: `checked 301 mismatches 0`. Lengths 0, 55, 56, 57,
63, 64, 65, 119, 120, 127, 128 are all inside that sweep. **`sha256_hex` is unchanged in
behaviour**, which is the load-bearing fact — it computes every fixture pin, corpus
digest, capture identity and baseline record line in this workspace. No finding.

### 2. `census_file::write_into` — **the three walks cannot disagree, and I could not make them.**

`header(transcript, label_go, rows.len())`, `body_digest(rows)` and the `for row in rows`
write loop take **the same slice, in the same order, by one expression each**
(`crates/pistol-arena/src/census_file.rs:61-66`). A mutant that digests a different
subset is caught by `the_census_file_carries_a_body_digest_over_the_rows_it_holds`, which
recomputes the digest from the bytes on disk. **What is NOT caught is the header's own
count** — see **M1** below; that is the one of the three that no test reads for a
non-empty file.

### 3. `emit::render_header` — **cannot drift from `render`.**

`render` emits `header` then `writeln!("{BODY_DIGEST}{digest}")` then `body`;
`write_into` emits `render_header()` then the same `writeln!` then the rows. One
`BODY_DIGEST` constant, one line shape. The census file is read back in
`census_capture_tests` through `emit::body_of` and `emit::claimed_body_digest` — the same
readers every capture fixture uses — so a grammar change breaks both writers at once.
No finding.

### 4. Gate-off byte-identity — **RE-TAKEN AT `eff179b`, AND IT HOLDS. The committed receipt does not cover this revision (M4).**

The fix round rewrote `sha256.rs`, which computes the `config`, `weights_sha256`,
`corpus` and `openings` digest lines **inside the record the obligation is taken over**,
so this could not be inherited from round 1. I built `eff179b` release from source in the
worktree (`cd972a42d0329128a7f5202836a8a92914b70396364a2b9df7239a51f5031eb3`, against the
receipt's `15c94598…`, which is still what `target/release/pistol` in the live tree holds
— i.e. the round-1 binary), ran `tools/baseline_snapshot.sh` for both configs with BOTH
binaries from one directory with one `--config` spelling, and applied the receipt's own
rule:

```
sed -n '1,/^# timing/p' <record> | grep -v '^revision \|^binary_sha256 ' | sha256sum
```

| record | digest | registered referent |
|---|---|---|
| `OLD_gate_v0.txt` (binary `15c94598…`) | `81e37d42…` | `81e37d42…` |
| **`NEW_gate_v0.txt` (binary `cd972a42…`, a build of `eff179b`)** | **`81e37d42…`** | `81e37d42…` |
| `OLD_instrument_v0.txt` | `c7f155e8…` | `c7f155e8…` |
| **`NEW_instrument_v0.txt`** | **`c7f155e8…`** | `c7f155e8…` |

**The obligation is satisfied at the revision under review.** The receipt that attests it
is not — see **M4**.

### 5. The arming rule, the seat rule, the arena sink, rule 3

Unchanged from round 1 in every limb round 1 traced, and the r1→r2 diff does not touch
`instance.rs`, `search.rs` or `pvs.rs` at all. Re-checked at source: `Budget::resolve` and
`stop_for` are refused **before** `collect_trigger_census`, so a refused budget never
arms; `take` then `stop` both run on the `Err` path; `self.census_folds` takes the root's
increment directly and `run.census_folds` once at `search.rs:563`, so a solver-proof early
return counts one fold for one row. The new TAB guard (`capture.rs:269-281`) is the same
arity check `no_tab` makes of the three other captured fields, driven by a real stub that
can emit one. The sink now writes **every byte before any stdout claim**
(`passes.rs:60-90`), which is m2's fix and the right order.

---

## ROUND-1 CODE FINDINGS, ADJUDICATED

| id | round-1 claim | status | how I decided it |
|---|---|---|---|
| **M1** | no test anywhere reads an IN-TREE census row's `key`; live mutant | **NOT CLOSED** | the new test pins `key != key_pos`, a correlate the defect preserves; mutant **M-SWAP** alive across 70 tests and visible on the wire → **B1** |
| **M2** | the arena's census sink is O(artifact) three times over | **CLOSED** | `passes.rs:64-75` streams through a `BufWriter`; `census_file::write_into` holds one line; `Fixture` body and the rendered `String` are both gone. Three copies → one, and the remaining copy is the design's own §6.2 sink, registered in `wp20b_impl.md` §4. (Its citation of round 1 is wrong — **m3**.) |
| **m1** | `CensusUnsupported` goes out with eighteen spaces | **CLOSED** | `error.rs:174-176` line-continuation restored; `the_census_refusal_reaches_the_wire_as_one_readable_line` pins the exact text AND `!refusal.contains("  ")`. Ran it: passes. Registered mutant M20 dies at it. |
| **m2** | stdout claims a capture a census failure then deletes | **CLOSED** | the census write and its manifest row now precede every `println!` (`passes.rs:60-90`), with the reason in a comment. Read the ordering; no `println!` remains ahead of a fallible write. |
| **m3** | `CENSUS_FORMAT_VERSION` is a promise nothing binds | **CLOSED** | `census_sha256(experiment, go, format_version)` folds it into the digest and takes it as a PARAMETER so a test can vary it; `a_census_file_of_a_later_grammar_is_not_the_same_run_as_this_one` now asserts `at(1) != at(2)`, which the version-dropping mutant M24 cannot survive. |
| **m4** | census rows bypass the `no_tab` guard | **CLOSED** | `capture.rs:269-281` refuses a tabbed row by name; `Behave::CensusTab` makes the call site reachable; `a_census_row_carrying_a_tab_refuses_the_run_by_name` ran green and M23 (call-removed) dies at it. |
| **m5** | the perf instrument survives only on a RAM-backed tmpfs | **RE-OPENED WORSE** | the files ARE exported to `artifacts/` and manifested — **under digests that are not the ones the perf receipt registers**. `9ec50053…`/`0a6a0f87…` (receipt, and what BOTH round-1 reviewers verified on disk) against `4a380ea5…`/`8bf64a37…` (exported, manifested). → **M2** |
| **m6** | the revision carries work that is not WP-2.0b | **NOT CLOSED** | `tools/wp21_tranche_config.py`, `wp21_tranche_config_tests.rs`, `wp21_prereg.md`, `overnight2_ledger.md`, `book_v2_ledger.md` are all still in the same uncommitted tree. → **m6** |
| **m7** | a `--census` capture on a gate-off seat writes `rows 0`, untested | **CLOSED** | `Behave::CensusNone` + `an_engine_that_honours_the_token_and_fires_nothing_writes_a_census_of_zero_rows`, which asserts `# derived rows 0`, an empty body and the stdout line. Ran it: passes. **This is also the test M1 exploits** — see **M1**. |
| **m8** | §5 missing; ADR line for D-537's counting rule owed | **HALF CLOSED** | §5 exists (`wp20b_impl.md:254`). The ADR half does not: `git diff a6777f4 eff179b -- docs/decisions.md` adds **D-565 and D-566 only**, neither of which records the census artifact class, the `Step::Census` seam or the `--census` flag. → **m7** |

**RE-OPENED WORSE: one (m5).** **NOT CLOSED: three (M1, m6, m8-second-half).**

---

## NEW FINDINGS

### B1 (BLOCKING) — the in-tree identity column is pinned by NOTHING, and its own test passes on a mutant that emits the position key there

**Claim.** `crates/pistol-search/tests/census_identity_tests.rs:151-188`
(`the_identity_column_is_never_the_position_key`) exists to close round-1 M1 and asserts
`assert_ne!(row.key, row.key_pos)`. That is a **one-bit correlate, not the property**.
`row.key_pos` is `state.key()` = `stones_key ^ context_key(to_move, phase)` and
`context_key` is never zero, so **`key != key_pos` is true for essentially every possible
value of `key` other than literally `state.key()`**. The registered mutant M21 happens to
be that literal form (`artifacts/mutants.py:160-165` leaves `key_pos = state.key()`
standing, so the two columns collide and the assertion fires). **Move the canonical key
into `key_pos` at the same time and the identity column is option A, the assertion is
true, and every test passes.**

**Sites.** `crates/pistol-search/src/pvs.rs:638-639`;
`crates/pistol-search/tests/census_identity_tests.rs:181-187`;
`crates/pistol-cli/tests/census_protocol_tests.rs:143-164` (which pins the key VALUE for
`turns_from_root == 0` only, and 32 lower-case hex digits for everything else — which
`GameState::key` also is).

**Governing obligation.** Design §8 **test 1** — *"`a_census_row_carries_the_canonical_key_of_the_position_it_fired_at`"*,
seat ON (c), *"a test whose mutant can only manifest AT A FIRING"* — and **D-553's
call-removed law**, which §8's own table applies to *"identity column dropped"* at test 1
with the reason *"a test calling `canonical_key` directly proves the fold and leaves a row
that never carries it"*. The in-tree site **is** a firing, and it is where the census's
production population comes from (§2). Test 1 as implemented reads the value at the root
only. The obligation is met at one of the two call sites.

**Failure scenario.** A production sweep's rows all carry `GameState::key` except one per
search. Transpositions still fold (that key folds them), **symmetries do not** — F2's
stated reason option A *"FAILS"* §8 compliance — so D-537's *disjoint positions*
denominator is computed over the wrong equivalence, **over-counting**, which F2 names as
*"the failure in the direction the rule was written to prevent"*. §9's `key_pos`
obligation is corrupted at the same stroke: the two columns' distinct-value counts are
exchanged, so the measurement of whether the in-tree symmetry fold's yield is above zero
returns its own answer backwards.

**MINIMAL REPRODUCER — RUN.**

```
git worktree add --detach /home/tom/pistol-runs/wp20b-r2 eff179b
cd /home/tom/pistol-runs/wp20b-r2
# MUTANT M-SWAP, crates/pistol-search/src/pvs.rs, inside the census closure:
-            let key = pistol_core::canonical_key(&stones);
-            let key_pos = state.key();
+            let key = state.key();
+            let key_pos = pistol_core::canonical_key(&stones);
```

**The whole population that can see a trigger-census row** — established with
`/usr/bin/grep -rln "TriggerObservation\|take_trigger_census\|info census\|census_file\|census_sha256\|CensusRequest\|--census" crates/*/tests/`,
which names exactly seven files:

| suite | against M-SWAP |
|---|---|
| `pistol-search --test census_identity_tests` | **5 passed, 0 failed** (83.69 s) |
| `pistol-cli --test census_protocol_tests` | **14 passed, 0 failed** (84.38 s) |
| `pistol-search --test trigger_census_cover_tests` | **3 passed, 0 failed** (386.52 s) |
| `pistol-arena --test census_capture_tests` | **12 passed, 0 failed** |
| `pistol-core --test canonical_key_tests` | **7 passed, 0 failed** |
| `pistol-cli --test report_tests` | **11 passed, 0 failed** |
| `pistol-cli --test movetime_tests` | **5 passed, 0 failed** |
| `pistol-engine --test engine_tests` | **13 passed, 0 failed** |

**70 tests, 0 failures. The mutant is alive**, including in the test written to kill it.

**AND IT IS VISIBLE ON THE WIRE, not only in a row set.** Release builds of the clean
tree and of M-SWAP, same seat, same fixture, `go nodes 4000 census`, printing `key` and
`turns_from_root`:

```
clean                                 M-SWAP
8adc7560f93b697753dc430a63ea6e8e 0    8adc7560f93b697753dc430a63ea6e8e 0   <- root, unchanged
03017106e7fbac162e2edc7afc0b4dfc 2    506195b918c7cef3ee11c283cd5893ac 2
f2b158b9f1f63963e09c19ebd42191fb 2    3036b2ac796d923ad5d05f4814035a10 2
f2b158b9f1f63963e09c19ebd42191fb 2    3036b2ac796d923ad5d05f4814035a10 2
a9197d8867e91dbe1f6ee6ca502c1a28 2    99f3cf90ba3868cc50a398c54598efca 2
```

Four of five rows carry a different identity, the root row is untouched — which is
exactly why the seated suite, which pins the root, does not notice.

**THE REMEDY IS A TEST.** The shipped code is correct. What the suite needs is the
in-tree row's key checked against something **externally derived**, the way
`census_protocol_tests::fixture_key()` does for the root — e.g. replay the in-tree row's
position from `played()` and assert `canonical_key` of it, or (cheaper, and it kills
M-SWAP outright) assert on the ROWS that
`row.key == canonical_key_of(row's stones) != row.key_pos` for a row with
`turns_from_root > 0`. Whatever the shape, **the criterion must be one the option-A
defect cannot satisfy by relabelling a column**, and `artifacts/mutants.py` should carry
M-SWAP as a registered mutant beside M21, since M21 alone certifies nothing M-SWAP does
not defeat.

---

### M1 (MAJOR) — the census file's `# derived rows` count is pinned only at ZERO; a 108-row census ships claiming `rows 0`, at exit 0

**Claim.** `crates/pistol-arena/src/census_file.rs:119` writes the header's
`# derived rows`. The **only** test in the workspace that reads that line is
`census_capture_tests.rs:287`, and it reads it on the **empty** census:
`text.contains("# derived rows 0")`. `a_capture_run_with_the_census_flag_writes_the_rows_it_was_sent`
counts the body's rows and never looks at the header. So a defect that writes `0`
unconditionally is preserved by every criterion in the suite — the same shape as the four
already found.

**Why it matters, in the code's own words.** `census_capture_tests.rs:272-274`: *"the
design asks for no refusal, so the `rows 0` line is the ONLY signal a tranche has."*
`wp20b_impl.md` §4: *"The header's own `# derived rows` line is still the count."* A
tranche reader who believes that line discards a completed sweep as "the trigger never
fired" — the F3 failure mode, wearing a receipt that is now wrong.

**MINIMAL REPRODUCER — RUN.**

```
# MUTANT M-ROWS, crates/pistol-arena/src/census_file.rs:119
-    fixture.derived("rows", rows);
+    fixture.derived("rows", rows.min(0));
```

| suite | against M-ROWS |
|---|---|
| `pistol-arena --test census_capture_tests` | **12 passed, 0 failed** |
| `pistol-arena --test capture_tests` | **39 passed, 0 failed** |

**51 tests, 0 failures.**

**THE FALSIFIER, WRITTEN AND RUN BOTH WAYS.** Four lines appended to
`a_capture_run_with_the_census_flag_writes_the_rows_it_was_sent`:

```rust
assert!(
    text.contains(&format!("# derived rows {}", rows.len())),
    "the header's count is not the number of rows the file holds: header says {:?}, body has {}",
    text.lines().find(|l| l.starts_with("# derived rows ")),
    rows.len()
);
```

Against M-ROWS it fails with

```
the header's count is not the number of rows the file holds:
header says Some("# derived rows 0"), body has 108
```

and on the clean tree `census_capture_tests` is **12 passed, 0 failed**. The remedy is
those four lines.

---

### M2 (MAJOR, **OVERTAKEN IN FLIGHT**) — at `eff179b` the perf receipt's registered instrument no longer existed anywhere (round-1 m5, RE-OPENED WORSE)

**Claim.** `artifacts/wp20b_perf_RECEIPT.txt:8-9` names the instrument the perf number was
taken with by digest: `wp20b_perf_guard.sh` = `9ec50053…`, `wp20b_perf_report.py` =
`0a6a0f87…`. **Both round-1 reviewers independently verified those digests on disk**
(`wp20b_impl_REVIEW_code.md:253`, `wp20b_impl_REVIEW_obligations.md:98-99`). Today the
same two scratchpad paths hold `4a380ea5…` and `8bf64a37…` — the files were **edited after
the receipt and after round 1** — and it is the **edited** versions that were exported to
`artifacts/` and indexed by `artifacts/wp20b_MANIFEST.txt:7-8`. The export answered m5's
letter and destroyed what m5 was protecting: the instrument the number stands on is now
unrecoverable, and the manifest names a **different** file under the receipt's claim.

**Reproducer — run.**

```
$ sha256sum artifacts/wp20b_perf_guard.sh artifacts/wp20b_perf_report.py
4a380ea57e3611961e5bd8c35329fd5a34819367436c81820f0e25515269b423  artifacts/wp20b_perf_guard.sh
8bf64a3706a1f4b536be638d6c336dfa2128587733c3a473c5881fd39ced6b70  artifacts/wp20b_perf_report.py
$ /usr/bin/grep -n 'sha256 9ec50053\|sha256 0a6a0f87' artifacts/wp20b_perf_RECEIPT.txt
8:script  .../wp20b_perf_guard.sh   sha256 9ec500538646468519fb10d3438799a9b8cc1376c9fc7577a25af1dca8602ccf
9:report  .../wp20b_perf_report.py  sha256 0a6a0f870a75d0ffb91c9852b45d30111a5e42ca1e5286a0cf3ab6610cc4adfa
```

**OVERTAKEN, and recorded rather than hidden.** While this report was being written,
`wp20b_impl.md` §3 became a "revision 2" that **rules the first perf registration MAY NOT
GOVERN A RUN, names a new instrument by digest before the run, and declares the first
run's numbers superseded**; the instrument was rewritten again at 11:02 (now
`648a96d8…`/`d29222be…`) and `wp20b_MANIFEST.txt` regenerated at 11:06, which now verifies
clean under its own `sha256sum -c`. `artifacts/wp20b_perf_RECEIPT.txt` is still the 00:23
file and still names `9ec50053…`/`0a6a0f87…`, but it is now openly a superseded receipt
awaiting a re-run rather than a standing claim. **The finding is therefore true of
`eff179b` and is being disposed of by OBLIGATIONS round 2**; it is recorded because m5 is
a round-1 CODE finding and adjudicating it is this report's job, and because the shape —
*an export that answers a finding's letter while destroying what the finding protected* —
is worth having on the record. **No action is asked of the implementer here beyond the
re-run already registered.**

---

### M3 (MAJOR) — `wp20b_impl.md` §6's MEASURED suite costs are false by 45x and 17x, and its stated reason contradicts the shipped code

**Claim.** `docs/experiments/wp20b_impl.md:304-308` (unchanged by the concurrent edit):

> *"the shared armed run was given a **depth** budget rather than a node one … **MEASURED
> after**: `census_protocol_tests` **1.88 s**, `census_identity_tests` **4.94 s**."*

The shipped budget is `const BUDGET: &str = "go nodes 4000"`
(`census_protocol_tests.rs:29`) — **a node budget**, changed in this very fix round for
B1's reason, and its own doc comment says so. And the costs are not those numbers.

**MEASURED, clean tree at `eff179b`, `cargo test -p CRATE --test FILE`:**

| suite | §6 claims | measured |
|---|---|---|
| `census_protocol_tests` | 1.88 s | **84.42 s** |
| `census_identity_tests` | 4.94 s | **83.75 s** |

**Why this is MAJOR and not prose.** §6 is a MEASURED claim in the package's governing
implementation document, and CLAUDE.md's matrix rule and D-291 make an unmarked or false
MEASURED number a finding. It is also the paragraph whose closing sentence is *"the
numbers are here because the alternative was a gate nobody runs"* — an argument that, at
the true numbers, cuts against the budget that shipped. **The CI cost this WP adds to
`cargo test --workspace` is ~168 s, not the ~6.8 s §6 registers**, and that is a fact the
operator is entitled to see stated correctly before closure. The fix is to restate §6
against the shipped budget with the measured numbers; the budget itself is defensible
(B1 is why it exists) and I am not asking for it to be reverted.

---

### M4 (MAJOR) — the byte-identity receipt attests a binary that is not a build of the revision under review

**Claim.** `artifacts/wp20b_identity_RECEIPT.txt` records `post-change target/release/pistol
15c94598…` and `post-change tree revision (uncommitted work on) a6777f4`. That binary is a
build of **`f7606cc`** — round 1's revision, and still exactly what
`/home/tom/Projects/HeXO-AlphaBeta/target/release/pistol` holds. The fix round then
rewrote `crates/pistol-cli/src/sha256.rs`, **the module that computes the `config`,
`weights_sha256`, `corpus` and `openings` digest lines inside the record the obligation is
taken over**, plus `error.rs`. A build of `eff179b` is `cd972a42…`, a different binary, and
the receipt says nothing about it.

**Not a correctness defect — I checked.** See "Gate-off byte-identity" above: I built
`eff179b` from source and re-took both configs, and both match the registered referents
exactly. **The property holds; the receipt does not attest it.** `docs/process.md`'s rule
that *"reviews of superseded revisions do not transfer"* is about pre-registrations, and
the same reasoning is what makes a receipt a receipt: it must name the revision it was
taken at, and this one names a superseded one.

**Reproducer.**
```
sha256sum /home/tom/Projects/HeXO-AlphaBeta/target/release/pistol   # 15c94598… == the receipt
git worktree add --detach /home/tom/pistol-runs/X eff179b && cd /home/tom/pistol-runs/X
CARGO_TARGET_DIR=/home/tom/pistol-runs/X-rel cargo build --release --locked --bin pistol
sha256sum /home/tom/pistol-runs/X-rel/release/pistol                # cd972a42… != the receipt
./tools/baseline_snapshot.sh --config /home/tom/Projects/HeXO-AlphaBeta/configs/gate_v0.toml \
    --binary /home/tom/pistol-runs/X-rel/release/pistol --out /tmp-less/NEW_gate_v0.txt
sed -n '1,/^# timing/p' NEW_gate_v0.txt | /usr/bin/grep -v '^revision \|^binary_sha256 ' | sha256sum
# 81e37d42…  == the registered gate_v0 referent
```
(The same, run for `instrument_v0`, returns `c7f155e8…`. Records left under
`/home/tom/pistol-runs/bi/`.)

---

### m1 (MINOR, **OVERTAKEN IN FLIGHT**) — at `eff179b`, `wp20b_MANIFEST.txt` failed its own `sha256sum -c` and the mutation run at the reviewed revision was in no manifest

```
$ /usr/bin/grep -E '^[0-9a-f]{64}  ' artifacts/wp20b_MANIFEST.txt | sha256sum -c - | grep -v ': OK$'
artifacts/mutants.py: FAILED
```
`mutants.py` is `4fdf0eda…` on disk against the manifest's `882c0f66…` (the driver was
extended after the manifest was taken). Separately, the manifest lists
`wp20b_mutants_v{1,2,3}.txt` and **not v4 or v5** — and **`wp20b_mutants_v5.txt` is the run
that governs**: 26 mutants, 26 dead, 0 alive, at worktree revision `595f004`, whose
`crates/` and `tools/` I verified are **byte-identical to `eff179b`**
(`git diff 595f004 eff179b -- crates/ tools/` is empty). `docs/experiments/wp20b_artifacts.md:37`
named v3 and v4 as *"the fix round's runs"* and never mentioned v5. So the artifact
carrying this package's mutation verdict was indexed by nothing.

**OVERTAKEN.** As of writing, `wp20b_MANIFEST.txt` has been regenerated (11:06), lists
`wp20b_mutants_v4.txt` and `v5.txt` with `v5` marked as the receipt the obligation stands
on, carries the current `mutants.py` digest, and **passes its own `sha256sum -c` with no
FAILED line**; `wp20b_artifacts.md` now names v5 as *"THE RECEIPT THE MUTATION OBLIGATION
STANDS ON"* and v4 as ABORTED. Re-verified by me at 11:0x. No action asked.

### m2 (MINOR) — `census_file::render` is dead, and its doc claims a purpose nothing fulfils

`crates/pistol-arena/src/census_file.rs:70-86`. Its own doc says *"this exists so a test
can read the file's text without a file"*, and no test calls it:
`/usr/bin/grep -rn "census_file::" --include=*.rs crates/` names only `write_into`,
`manifest_row`, `census_sha256` and `CENSUS_FORMAT_VERSION`. `pub` in a lib crate, so
`dead_code` does not fire. It carries two `unreachable!` arms that nothing exercises.
Either give it the caller its doc describes or delete it.

### m3 (MINOR) — `wp20b_impl.md` §4 attributes to round 1 a measurement round 1 explicitly declined to make

`wp20b_impl.md:223`: *"REVIEW-impl **measured** three resident copies before the first byte
reached disk."* Round 1 said the opposite in the finding's own reproducer heading:
*"Reproducer (bounded, **by inspection rather than by burning the RAM**) … I did not run an
8 GB capture"* (`wp20b_impl_REVIEW_code.md`, M2). The count of three is right; the word
MEASURED is not the source's. The adjacent *"~26 GB, on a 46 GB box"* carries no
MEASURED/ESTIMATED mark either.

### m4 (MINOR) — an `info census …` line emitted OFF the token is silently collected and dropped

`crates/pistol-arena/src/passes.rs:53-54` passes `&mut rows` to `capture::run` on **every**
capture, census or not, and `capture.rs:268-283` classifies and pushes any `info census …`
line the engine writes. With `census: None` the whole `Vec` is dropped and nothing is said.
The design's §4 is explicit that off the token *"the block does not exist"*, so a row
arriving there is a protocol deviation — and it is swallowed rather than named, which is
the shape rule 3 is about. Impact is low (the capture itself is unaffected and no `pistol`
build can produce one), and `Step::Ignore` already exists for `info` lines, which is why
this is minor rather than a rule-3 breach.

### m5 (MINOR) — the new rule-9 justification says the suite makes one search; it makes at least five

`docs/rule9_justifications.md`, entry for `crates/pistol-cli/tests/census_protocol_tests.rs`:
*"the suite makes a single search and each case asserts one property of its answer."* The
suite makes the shared `armed_census()` run, two `CHEAP` runs
(`two_move_orders_reaching_one_position_share_a_census_key_on_the_wire`,
`a_plain_go_after_a_census_go_computes_no_key_and_emits_no_line`, the latter twice), an
`UNARMED` run, an off-token `BUDGET` run and two refusal runs. The justification's argument
— that the seated cases are only worth anything because they read ONE run — is sound and
survives the correction; the sentence as written is not true of the file.

### m6 (MINOR) — round-1 m6 unclosed: WP-2.1 work is still in the same uncommitted tree

`tools/wp21_tranche_config.py`, `crates/pistol-arena/tests/wp21_tranche_config_tests.rs`,
`docs/experiments/wp21_prereg.md`, `docs/experiments/overnight2_ledger.md` and
`docs/book_v2_ledger.md` are unchanged since round 1 raised this. **Not reviewed here**, for
the second time; in particular `tools/wp21_tranche_config.py` still has no
`tools/SHELL_CHECKLIST.md` review, which `docs/process.md`'s coverage rule requires of a
`tools/` artefact producing a recorded number. CLAUDE.md's Closure section wants one feature
per commit.

### m7 (MINOR) — round-1 m8's ADR half unclosed

`git diff a6777f4 eff179b -- docs/decisions.md` adds **D-565** (the loop grant) and
**D-566** (the `wp20b-stopped` deletion) and nothing else. Hard rule 10 wants a line for the
census artifact class, the `Step::Census` seam change and the `--census` flag; design §10.3
books one for D-537's counting rule *"at this package's closure"*. None exists.

---

## REJECTED

### R1 (REJECTED) — "the streaming digest's `wrapping_mul(8)` loses the top three length bits"

**The suspicion.** `finish_hex` computes `bit_length = self.length.wrapping_mul(8)`, which
discards bits 61-63 of the byte count.

**Why it rejects.** FIPS 180-4 specifies the length field as the message length in bits
**mod 2^64**, so the wrap is the spec and not a loss; and a payload above 2^61 bytes is
2 exabytes. I confirmed the arithmetic is right where it can be checked — every one of the
301 lengths in the probe matched `hashlib` — and there is no reachable input that
distinguishes the two.

### R2 (REJECTED) — "`census_path` can collide with another run's `--out` and overwrite it"

**The suspicion.** `outpath::census_path` derives `cap.txt` → `cap.census.txt`, so a
concurrent run started with `--out cap.census.txt` names the same file.

**Reproducer attempted, and it cannot lose a byte.** Both paths go through
`outpath::claim`, which is one `O_EXCL` open (D-200). Whichever run reaches the name second
is refused by name and exits 2, and `bin/arena.rs:72-76` gives the `--out` claim back. I
re-ran round 1's collision case: with `cap.census.txt` pre-existing,
`arena --capture … --out cap.txt --label-nodes 5000 --census` refuses, exits 2, leaves the
pre-existing file untouched and does not leave `cap.txt` behind. No silent overwrite
exists.

---

## STATE OF THE TREE AT WRITING

`crates/` and `tools/` have not moved from `eff179b`. `docs/` and `artifacts/` have, and
a concurrent OBLIGATIONS round-2 pass is visibly mid-flight (a perf registration rewritten
as revision 2, a new instrument, a regenerated manifest, a re-run pending). Against the
tree as it stands **right now**:

| finding | still open? |
|---|---|
| **B1** (in-tree identity unpinned; M-SWAP alive) | **OPEN** — `crates/` unchanged |
| **M1** (`# derived rows` pinned only at zero; M-ROWS alive) | **OPEN** — `crates/` unchanged |
| **M3** (§6's MEASURED costs false by 45x/17x; contradicts `BUDGET`) | **OPEN** — `wp20b_impl.md:344-346` unchanged |
| **M4** (identity receipt names a superseded binary) | **OPEN** — `wp20b_identity_RECEIPT.txt` untouched since 23:55 |
| M2 (perf instrument vs receipt) | overtaken; a re-run is registered |
| m1 (manifest) | overtaken; the manifest now verifies |
| m2, m3, m4, m5, m6, m7 | OPEN |

**The two that must not be lost in the churn are B1 and M1.** Both are test-side and both
have a live mutant.

**One late movement in `crates/`, and it is B1's.** After the code sections above were
finished, `crates/pistol-search/tests/census_identity_tests.rs` gained a `probe_mirror_tree`
function: twelve searches, one per `Symmetry::ALL`, printing each run's sorted row keys and
**asserting nothing**. It is exploratory scaffolding, not a fix — it must not land as it
stands (no assertion, and a name that is not a behaviour, against CLAUDE.md's test-naming
rule) — but its DIRECTION is the right one and worth saying so: **a mirrored game's in-tree
rows sharing keys with the unmirrored game's is a criterion option A cannot satisfy**,
because `GameState::key` folds no symmetry. Turned into an assertion over rows with
`turns_from_root > 0`, that closes B1 and kills M-SWAP. `crates/pistol-arena/tests/wp21_tranche_config_tests.rs`
also moved (+29 lines); still out of scope, see m6.

---

## COUNT BY CLASS

Against the named revision `eff179b`.

| class | n |
|---|---|
| **BLOCKING** | **1** (B1) |
| MAJOR | 4 (M1, M2, M3, M4) |
| MINOR | 7 (m1–m7) |
| REJECTED | 2 (R1, R2) |

**Round-1 CODE findings: 5 CLOSED, 1 HALF CLOSED, 3 NOT CLOSED, 1 RE-OPENED WORSE.**

**The one finding that changes what anyone may conclude is B1.** It is round-1 M1
un-closed, it is the **fifth** vacuous criterion this package has produced and the second
in as many rounds at the same call site, and the criterion that replaced the fourth is
defeated by relabelling one column. **The remedy is a test, not a code change** — and the
mutant that proves the point should be registered beside M21 in `artifacts/mutants.py`, or
the next round will certify the same gap a third time.
