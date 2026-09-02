# WP-2.0b — the SCOPED VERIFICATION PASS. Not a fourth review; a checklist with receipts.

**WHAT THIS IS AND WHAT IT IS NOT.** The resumed dispatch's §1 grants **no fourth
review round**. What it grants instead is a scoped pass over four things, each
named by the dispatch in its own words:

> *"remedies verbatim vs round 3, mutation set vs grep receipt, byte-identity
> referents still MATCH at the new binary, CI green."*

**A FAILURE HERE IS A STOP AND NOT A ROUND 4** — the dispatch says so, and this
document records the verdict either way. It makes no claim about anything round 3
did not raise: the twenty-five findings round 3 closed stay closed on round 3's
own re-derivation, and the two it left open are B1 and B2.

**NAMED REVISION**: `the remedied tree, named by its `git stash create` object in `artifacts/wp20b_MANIFEST.txt`'s header; HEAD `d83ac01``.

---

## 1. REMEDIES VERBATIM AGAINST ROUND 3

Round 3 wrote each remedy in its own words. Each row quotes the requirement and
names where it landed; the right-hand column is what a reader checks, not what
this document asserts.

### B1 — three limbs

| round 3's words | where it landed | how to check it |
|---|---|---|
| *"Anything that reads an IN-TREE row's `key` against a referent derived outside the search — replay the row's position and take `canonical_key` of it"* | `crates/pistol-search/tests/census_identity_tests.rs`, `an_in_tree_rows_identity_is_the_canonical_key_of_the_position_it_fired_on` | the test replays `IN_TREE_FIRING_AT_1500` stone by stone and derives both identities from `pistol-core` alone; `docs/experiments/wp20b_B1_remedy.md` §2 records where the fixture came from and §3 what the assertion does and does not prove |
| *"a registered mutant at **each** of the two push sites"* | `artifacts/mutants.py`, **M30**, **M31**, **M32**, **M33** | §2 of this document, against the receipt |
| *"the `census.rs` doc comment must stop asserting a universal its own callers falsify"* | `crates/pistol-search/src/census.rs:9-13` | the sentence is gone; `wp20b_B1_remedy.md` §5 quotes what was deleted and what replaced it. The same half-claim in `census_identity_tests.rs` — *"there is no call site left to exchange them at"* — is deleted with it |

### B2 — two limbs

| round 3's words | where it landed | how to check it |
|---|---|---|
| *"delete the three sentences"* | `docs/experiments/wp20b_impl.md` §3.5, `artifacts/wp20b_perf_RECEIPT.txt`, `docs/experiments/overnight2_ledger.md`'s obligation row | §3 of this document |
| *"either … the 27-minute re-run is owed — or obtain the review before the run"* | the RE-RUN, `artifacts/wp20b_perf_guard_v3.txt` | §3 of this document, with the review's commit timestamp and the run's own |

---

## 2. THE MUTATION SET AGAINST THE GREP RECEIPT

D-568's standing law: *"a mutation set is specified against CALL SITES ENUMERATED
BY A `git grep` RECEIPT recorded in the mutation document, never against prose."*

### 2.1 THE RECEIPT, TAKEN THREE WAYS

`docs/experiments/wp20b_B1_remedy.md` §1 carries the greps. They are taken three
ways because a receipt over CONSTRUCTIONS alone answers half the question:

1. over the **assignment pattern** `key: keys.` / `key_pos: keys.` — **two push
   sites**, `pvs.rs:763-764` (in-tree) and `search.rs:813-814` (root);
2. over **every construction** of the row type and of the key pair —
   `TriggerObservation {` and `CensusKeys {` — which adds `census.rs:33` (the
   derivation, M27's site), a scripted arena FIXTURE and a test's own builder,
   and no fourth firing site;
3. over every **CONSUMPTION** of the identity column — `\.key\b` across
   `pistol-engine/src`, `pistol-cli/src` and `pistol-arena/src` — which returns
   ONE census consumer, `report.rs:145`, reading `row.key` alone. A
   single-column read has no pair to exchange, and `key_pos` reaches no CLI
   source at all, which is the design's *"NOT on the wire"* clause read off the
   code rather than off the prose.

### 2.2 THE SET AGAINST THE RECEIPT

| receipt site | class | mutant | registered dying test |
|---|---|---|---|
| `census.rs:33` | the DERIVATION | **M27** | `each_identity_column_is_the_derivation_it_is_named_for` |
| `pvs.rs:763-764` | PUSH SITE, in-tree | **M30** exchange | `an_in_tree_rows_identity_is_the_canonical_key_of_the_position_it_fired_on` |
| `pvs.rs:763` | PUSH SITE, in-tree | **M32** call-removed | the same |
| `search.rs:813-814` | PUSH SITE, root | **M31** exchange | `every_census_row_carries_the_two_columns_that_function_produces` |
| `search.rs:813` | PUSH SITE, root | **M33** call-removed | the same |

**EVERY SITE THE RECEIPT ENUMERATES CARRIES A MUTANT, and every mutant names the
test that must die at it.** That is the whole of B1's second limb, and the
failure it closes is on the record: the set as it stood registered M27 at
`census.rs` alone — *"the one of the three places the defect no longer needs"* —
so a mutant could not see the exchange the reviewer then demonstrated live at the
in-tree site with 73 tests green.

### 2.3 THE RUN

`artifacts/wp20b_mutants_v8.txt`, in a worktree on `/home` with its own target
directory — a mutation is a deliberate break, and a break left in the tree the
implementing session is editing is indistinguishable from a regression.

```
=== MUTANTS COMPLETE: 31 registered, 31 dead at their registered test,
    0 dead elsewhere, 0 alive, 0 harness fault(s) ===
```

**THE FOUR NEW ONES EACH DIED AT THE TEST THEY WERE REGISTERED AGAINST**, and
which test each died at is the claim — *"the suite went red"* is not:

| mutant | dies at |
|---|---|
| **M30** in-tree exchange | `an_in_tree_rows_identity_is_the_canonical_key_of_the_position_it_fired_on` |
| **M31** root exchange | `every_census_row_carries_the_two_columns_that_function_produces` |
| **M32** in-tree call-removed | `an_in_tree_rows_identity_is_the_canonical_key_of_the_position_it_fired_on` |
| **M33** root call-removed | `every_census_row_carries_the_two_columns_that_function_produces` |

**M30 IS THE REVIEWER'S OWN LIVE MUTANT.** Round 3 applied exactly that exchange
at exactly that site and ran the population that can see a census row: **73 tests
across 8 suites, 0 failures.** It now dies. That is B1's whole content — not that
the code was wrong, but that nothing could tell.

**AND THE MUTATION RUN COVERS THE REVIEWED CODE, WHICH THE PREVIOUS RECEIPT DID
NOT.** Round 3's M2: two documents named run **5**, at `595f004`, a tree eleven
source and test files away from the reviewed one — *"run 5 knows nothing about
`CensusKeys`, M27, M28 or M29"*. Run 8 is taken at the remedied tree itself.

---

## 3. B2's ORDER, WITH BOTH TIMESTAMPS

| document | what it said | what it says now |
|---|---|---|
| `artifacts/wp20b_perf_RECEIPT.txt` | *"§3, REVISION 2, **which passed a fresh-context review BEFORE this run**"* | the file is renamed `wp20b_perf_RECEIPT_v2_SUPERSEDED.txt`, its false sentence replaced by the disposition, and the name `wp20b_perf_RECEIPT.txt` now holds the GOVERNED run |
| `docs/experiments/wp20b_impl.md` §3.5 | *"Taken after this registration **passed its review**"* | names round 3 as revision 2's first fresh-context review, with its commit and timestamp, and the re-run as the governed one |
| `docs/experiments/overnight2_ledger.md` | *"perf guard, under a **REVIEWED** registration"* | *"perf guard, under the registration round 3 reviewed"*, pointing at the v3 raw |

**THE TWO TIMESTAMPS, AND THEY ARE THE WHOLE FINDING.**

```
round 3's report, committed   a518cc7   2026-09-02 15:17:35 +0200
the SUPERSEDED run                      2026-09-02 14:52:31 +0200   <- BEFORE
the GOVERNED re-run, started            2026-09-02 15:47:54 +0200   <- AFTER
the GOVERNED re-run, finished           2026-09-02 16:16:29 +0200
```

**AND ROUND 3 IS A REVIEW OF THE REGISTRATION AND NOT MERELY OF THE PACKAGE.** It
read §3 revision 2 as a pre-registration, ruled round 2's ten limbs DISCHARGED,
and confirmed *"the instrument computes the registered statistic"* — which was
round 2's central finding and is CLOSED. What it could not do was make a run that
had already happened a governed one.

**THE RE-RUN'S VERDICT**: H1 = **0.9981**, inside `[0.98, 1.02]`, **NOT
REJECTED**; ON/OFF 1.0020 against an abort at 0.95, **no abort**; time-to-depth
1.0000 both ways; **0 refusals**; the ON arm wrote **181** census rows and both
OFF arms none, identically across all six reps. Receipt
`artifacts/wp20b_perf_RECEIPT.txt`, raw `artifacts/wp20b_perf_guard_v3.txt`.

**AGAINST THE SUPERSEDED RUN**, because a reader will want to know whether
anything moved: H1 `0.9982` then, `0.9981` now; ON/OFF `0.9994` then, `1.0020`
now; 181/0/0 both times; 0 refusals both times. **Two runs of one program on one
box — the difference is the box.**

---

## 4. BYTE-IDENTITY REFERENTS AT THE NEW BINARY

The dispatch's words: *"byte-identity referents still MATCH at the new binary"*.
Re-taken with the same rule, on four fresh records:
`artifacts/wp20b_identity_RECEIPT_v2.txt`.

```
THE RULE (wp20b_design.md §9), quoted and then applied
sed -n '1,/^# timing/p' <record> | grep -v '^revision \|^binary_sha256 ' | sha256sum

configs/gate_v0.toml       referent 81e37d42…   MATCH  closure2_gate_v0_run1, run2
configs/instrument_v0.toml referent c7f155e8…   MATCH  closure2_instrument_v0_run1, run2
```

**AND THE STRONGER FACT THE RE-TAKE PRODUCED.** The B1 remedy touches a `///`
comment in `census.rs` and adds tests, so the prediction was that the shipped
binary would not move at all — and it did not:

```
target/release/pistol  7a7a2347a7af56a103d0fa512bb6838361d258e3a146ac71ac9903c2c6ec6a6e
```

the same digest every earlier closure receipt names. **So the determinism run,
the perf guard and both identity referents all attest one program**, and the
remedy is provably a change to what a reader is told rather than to what the
engine does.

---

## 5. CI

`tools/ci.sh` at the remedied tree. **Cited from the gate lines, never from a
wrapper's exit status** (CLAUDE.md's Closure section):
`artifacts/wp20b_ci_closure_v2.txt`.

```
=== gate  1/19: cargo fmt --all --check
=== gate  2/19: build from the git-tracked file set
=== gate  3/19: cargo test --workspace --locked
=== gate  4/19: cargo clippy --workspace --all-targets -- -D clippy::all
=== gate  5/19: artifact rejection
=== gate  6/19: config validation
=== gate  7/19: perft oracle
=== gate  8/19: tactical fixture at its pre-registered threshold
=== gate  9/19: cross-process determinism
=== gate 10/19: differential search oracle
=== gate 11/19: staged generator soundness (four parts)
=== gate 12/19: solver oracle (four gates)
=== gate 13/19: solver determinism
=== gate 14/19: movetime ceiling on the D-95 reproducer class
=== gate 15/19: arena self-match smoke
=== gate 16/19: sealbot anchor platform suite
=== gate 17/19: file-justification check
=== gate 18/19: decision-key uniqueness
=== gate 19/19: carve-document label consistency
ci: all gates passed
EXIT=0
```

**NINETEEN GATE LINES, ONE `all gates passed`, `EXIT=0`.** Wall **30 m 37 s**
(16:18:08 → 16:48:45), MEASURED.

**ONE GATE FAILED FIRST AND IS RECORDED RATHER THAN TIDIED AWAY.** The first
launch returned `ci: FAIL: formatting` at gate 1: the B1 assertion's
`assert_ne!(referent, replayed.key(), …)` wanted its two arguments on separate
lines. `cargo fmt --all` applied it, `cargo fmt --all -- --check` returned clean,
and CI was re-launched from gate 1. **rustfmt is mechanical law** (CLAUDE.md's
Code style), so this is a fix and not a finding — but a run that only reports the
green attempt is a run whose first attempt nobody can see.

**TWO GATES READ THE INDEX AND NOT THE WORKING TREE, WHICH IS HOW A CLAIM ABOUT
THEM CAN BE TRUE OF THE WRONG FILE.** `decision_key_check.sh` resolves
`docs/decisions.md` through `git ls-files -s` and `git cat-file blob`
(`:136-148`), and `file_justification_check.sh` reads tracked blobs the same way.
**Run against unstaged work they answer about HEAD**, which is exactly what
happened here: an intermediate reading of *"569 decision keys"* and *"64 over the
cap"* was a true statement about `d83ac01` quoted as though it were about the
remedied tree. Staged, the same two gates read **574 decision keys, no repeat
outside D-279's exemption** and **66 files over the cap, 66 entries, all
registered**. **A gate's output is evidence about the revision the gate read**,
and naming that revision is the difference between a receipt and a coincidence.

**AND GATE 17 IS NOT A FREE PASS HERE.** Two files crossed hard rule 9's soft cap
with this remedy — `census_identity_tests.rs` and `wp21_tranche_config_tests.rs`
— so both gained an entry in `docs/rule9_justifications.md` saying WHY, neither
of which states a line count (counts are derived, never asserted).
`file_justification_check` reads **355 tracked `.rs`/`.sh` files, 64 over the
cap, all 64 registered**.

---

## 6. VERDICT

**PASS on all four limbs the dispatch named, and the dispatch's STOP is not
reached.**

| limb | verdict |
|---|---|
| remedies verbatim against round 3 | **MET** — §1, five limbs across B1 and B2, each quoted and sited |
| mutation set against the grep receipt | **MET** — §2, every enumerated site carries a mutant; **31 of 31 dead at their registered test, 0 alive** |
| byte-identity referents MATCH at the new binary | **MET** — §4, both referents, both runs; and the binary is unmoved |
| CI green | **MET** — §5, 19 gate lines, `ci: all gates passed`, `EXIT=0` |

**WHAT THIS PASS DOES NOT SAY.** It is not a fourth review and makes no claim
about anything round 3 did not raise. The twenty-five findings round 3 closed
stay closed on ROUND 3's re-derivation, not on this document's. **And two of
round 3's own MAJORs and five of its minors were remedied alongside** — M1 (this
package's artifact index carried zero digests and now carries 48), M2 (two
documents naming the wrong mutation run), m1, m2, m3, m6, m7 — which is more than
the dispatch asked for and is listed so a reader can tell what was scoped from
what was volunteered.

**THE ONE THING A SUCCESSOR SHOULD NOT MISREAD.** Every receipt in this document
attests **one binary**, `7a7a2347…`. The B1 remedy changed a `///` comment and
added tests; it changed no instruction the engine executes. **So this closure is
a repair to what the package CLAIMS, and the code it claims about was already
right** — which is exactly what round 3 said when it opened with *"neither is a
wrong answer in the engine. Both are mine."*
