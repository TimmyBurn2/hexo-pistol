# P1 — REVIEW-impl ROUND 2: the discharge of round 1's findings, and a full re-check

**Implementation revision reviewed:** `d42d33d0c8ad640d7138a14caa5a1176d49fd2c0`,
which IS `p1/impl`'s tip at the time of this review:

```
$ git rev-parse p1/impl
d42d33d0c8ad640d7138a14caa5a1176d49fd2c0
```

Seven commits on `dev`; the first (`4298ecd`) says of itself that it is a
measurement revision, so the cumulative diff `ffc5c10 → d42d33d` is what was read.
Round 1 reviewed `3ef67068a15880c15e372b22565e0f4f37de8e76`; the code half of the
discharge is `git diff 3ef6706 d42d33d -- crates`.

**Design revision it is checked against:** `289f184615c97abf8385d90258d356705a6783aa`
(a `git stash create` commit on `dev`) — `p1_design.md` revision 3 **as amended
after REVIEW-impl round 1**. The four governing documents at that revision hash
the same as the working tree's untracked copies, at the start of this review and
again at its end:

```
$ for f in p1_design.md p1_design_REVIEW.md p1_design_REVIEW_rev2.md p1_design_REVIEW_rev3.md; do
    a=$(git show 289f184615c97abf8385d90258d356705a6783aa:docs/experiments/$f | sha256sum | cut -c1-16)
    b=$(sha256sum docs/experiments/$f | cut -c1-16); echo "$f rev=$a wt=$b"; done
p1_design.md             rev=2515304fddd13cc7 wt=2515304fddd13cc7 SAME
p1_design_REVIEW.md      rev=b35e4c29ce24040d wt=b35e4c29ce24040d SAME
p1_design_REVIEW_rev2.md rev=785b43a52fff9296 wt=785b43a52fff9296 SAME
p1_design_REVIEW_rev3.md rev=cab1411c9d9f0256 wt=cab1411c9d9f0256 SAME
```

The design moved between rounds (`p1_design.md` `5126a44d…` → `2515304f…`); the
diff is the header's amendment note plus the Scope and §2 changes F5 asked for.

**HEAD:** `ffc5c10f4d16356f574e3221a023f78399d2e3bb` on `dev` (= the branch point;
`dev` has not moved). **Date:** 2026-09-04. **Round 2**, a FULL round.

> ### THE DOCUMENT HALF OF THE SUBJECT MOVED DURING THIS REVIEW — READ THIS FIRST
>
> The brief pinned the implementation by commit and the design by stash, but
> named `docs/decisions.md` and `docs/experiments/p1_bench_prereg.md` by path
> only. Both are uncommitted in the live tree (`M docs/decisions.md`; the prereg
> untracked) and **both were rewritten while this review was running**:
>
> | file | as I first read it | as adjudicated below |
> |---|---|---|
> | `docs/decisions.md` (D-603) | bracket `[1.20, 1.30]`; verification pinned `at \`3ef6706\``; *"the five gates"* | mtime **11:28:15**, sha256 `31d1b485811aec67…`: per-band brackets, full 40-char SHAs, a landing-revision paragraph, *"the six gates"* |
> | `p1_bench_prereg.md` | **revision 2** | mtime **11:27:45**, sha256 `ebbfead197562d7d…`: **revision 3** |
>
> Two of the findings this round raised (F10, F12) were closed by those edits
> mid-review; they are recorded below with both texts, and they are NOT counted
> in the verdict. **Everything this report says about those two files is pinned to
> the two sha256 values above and to nothing a reader can check out** — which is
> D-602's own rule (*"a working tree is not a revision"*) failing in the review
> that adjudicates the line D-602 immediately precedes. That is **F16**.
>
> **The implementation itself did not move**: `p1/impl` is still `d42d33d`, both
> worktrees are still detached at it, and §4 shows its release binary is
> byte-identical to `3ef6706`'s.

**Where everything ran:** `/home/tom/pistol-wt/review-p1-impl` (detached at
`d42d33d`; the full `cargo test --workspace --locked` with NO exported
`CARGO_TARGET_DIR`) and `/home/tom/pistol-wt/review-p1-impl-mut` (detached at
`d42d33d`, `CARGO_TARGET_DIR` = its own `target`, for the mutation re-run, the six
gates, fmt/clippy and the binary-identity experiment). Never the live tree, never
`/tmp`, never `p1-measure`, `p1-mutants`, `rt-base` or `rt-cand`. Both worktrees
are left in place, detached at `d42d33d`, clean, no branch created:

```
review-p1-impl:     HEAD=d42d33d branch=detached status=''
review-p1-impl-mut: HEAD=d42d33d branch=detached status=''
```

---

## 1. Round 1's findings — the discharge, BY EXECUTION (D-591)

| round-1 finding | discharge offered | what I ran | holds? |
|---|---|---|---|
| **F1 (BLOCKING)** — the ADR line §2 requires does not exist | `D-603` appended to `docs/decisions.md` | read D-603 in full; `bash tools/decision_key_check.sh`; `sha256sum` on every artifact it cites; every number re-derived from those artifacts; cross-read against `p1_bench_prereg.md` | **YES on §2's requirement** — all three required things are recorded and D-254's flip clause is answered as written. The line carries defects of its own: F11 (MAJOR), F13, F17 standing; F10 and F12 raised and closed mid-review |
| **F2 (MINOR)** — the mutation receipt records no death REASON | driver gains a first-panic-line capture; receipt re-taken as `artifacts/p1_mutation_d42d33d_v1.txt` | re-ran the driver myself in `review-p1-impl-mut` at `d42d33d`; diffed against the receipt; checked every reason against its registered class | **YES.** 15/15 DEAD, byte-identical to the receipt, and **M14's reason is the refusal, not the key inverse's panic** (§2) |
| **F3 (MINOR)** — the carried row *"M2 dies at I5 alone"* is false | recorded in the ledger and carried into no governing document | `/usr/bin/grep -n "I5 alone\|dies at I5\|M2 dies" docs/experiments/p1_design.md docs/experiments/p1_bench_prereg.md docs/decisions.md docs/experiments/opt_arc_ledger.md` → **one hit, `opt_arc_ledger.md:132`, the finding's own record** | **YES.** Nothing carried it into the design, the prereg or the ADR; the design's §4 M2 row still reads *"I5 (and I1 on any playout crossing a boundary)"*, which is what execution supports |
| **F4 (MINOR)** — I5's comment is false as written | comment rewritten with its arithmetic (`threat_oracle_tests.rs:419-427`) | re-derived the arithmetic against `LineStore::run`'s own condition (`line.rs:175`) and against `windows_through_indexed` (`crates/pistol-core/src/window.rs:112-121`) | **YES on the substance** — the false sentence is gone and the numbers printed are right (`(-6).rem_euclid(64) = 58`, `58 + 6 = 64`, not `> 64`; position `0` has offset `0`). Three residues remain: **F14** |
| **F5 (MINOR)** — `sets.rs`'s docs still name *"the table"*; a DESIGN gap | design Scope + §2 amended to name `sets.rs` as a third in-crate doc; both sentences corrected in the code | `git grep -n -iE "the (window )?table\|table (underneath\|type\|is)" -- crates/pistol-solver/src` at `d42d33d` | **YES.** The only surviving `table` nouns in the solver are the df-pn transposition table (`config.rs:79`, `dfpn.rs:117/259/522/583`, `solver.rs:195`, `tt.rs:38/63/217`) and `sets.rs:26`, the module doc's markdown table. But §6 was not amended with Scope: **F15** |

### F1's discharge, clause by clause

**§2's three required things** — *"one D-line recording that D-254's flip clause
fired on the P1 re-profile and what replaced the store, with the LIFO contract and
the hand-written equality named as the two things a caller can observe"*:

1. **The flip clause fired** — yes, and quantified: *"named `WindowTable::masks` +
   `::set` at 17.05 % of wall and the state's maintenance at most 35.67 %, so the
   clause fired."*
2. **What replaced the store** — yes: per-axis line bitboards with a logged undo,
   with the matrix's O-E named and its measured ratio.
3. **The two observable things** — yes, under its own heading *"THE TWO THINGS A
   CALLER CAN OBSERVE"*: `undo` takes back the last stone applied and refuses any
   other with `THREAT_DESYNC`; `PartialEq` is hand-written over the store and the
   sets, excluding the three stacks.

**D-254's flip clause as written** — *"Flips when a bench with `p > 0` names window
lookup as a measured hotspot… the replacement goes behind this same API and is a
rule-5 change."* D-603 names the hotspot with a digested profile, states the
replacement, registers a rule-5 bracket, and records the one way the API is *not*
identical (the LIFO refusal) rather than glossing it. **Answered.**

**`tools/decision_key_check.sh`:**

```
decision_key_check: self-test passed — a clean seed, a seeded repeat, and the anchor
decision_key_check: grandfathered by D-279 (ruled, not deleted): D-276 2 D-277 2
decision_key_check: 604 decision keys in docs/decisions.md, no repeat outside the exemption
```

**Every measured number in D-603, traced.** Digests first:

```
ed5f0cab52ee69fe…  artifacts/p1_profile_ffc5c10_v1.txt      cited ed5f0cab…  MATCH
65a16dee49f63389…  artifacts/p1_counters_ffc5c10_v2.txt     cited 65a16dee…  MATCH
d0b1b38480b740ce…  artifacts/p1_mx_bench_E_v1.txt           cited d0b1b384…  MATCH
1c831daaf2644c4f…  artifacts/p1_mutation_d42d33d_v1.txt     cited 1c831daa…  MATCH (added mid-review)
61baa09e2dd3136e…  artifacts/p1_rt_round2/solveron_run.log  cited 61baa09e…  MATCH (but see F13)
6c0a601a69048741…  artifacts/p1_mutation_3ef6706_v1.txt     cited 6c0a601a…  MATCH
87a081a5bfd07c29…  artifacts/p1_mx_bench_A_v1.txt           cited 87a081a5…  MATCH
```

Then the arithmetic, re-derived from those files rather than read off the line:

| D-603's number | derivation I ran | verdict |
|---|---|---|
| `masks` + `::set` at **17.05 %** | profile rows `WindowTable::masks` 9.86 % + `WindowTable::set` 7.19 % = 17.05 | exact |
| maintenance **at most 35.67 %** | 14.04 (`ThreatState::touch`) + 9.86 + 7.19 + 4.58 (`WindowSets::transition`) = 35.67 | exact |
| `WindowSets::transition` **4.58 %** | the profile row itself | exact |
| **5.3–8.6 %** of stones undone unread | `unqueried_undos / places` over the three `P1COUNT` rows: 33916/642831 = 5.28 %, 3780/44166 = 8.56 %, 16650/240811 = 6.91 % | exact |
| **zero flushes with two stones pending in 874 081** | `pending_hist[2] == 0` in all three rows; 609346 + 40522 + 224213 = 874081 | exact |
| lazy seam **1.028 / 1.021** | `p1_mx_bench_A_v1.txt:34,38` | exact |
| O-E **1.257 / 1.240** | `p1_mx_bench_E_v1.txt:34,38` | exact |
| O-E0 **1.190 / 1.188** | `artifacts/p1_mx_bench_E0_v1.txt:34,38` | exact |
| *"about two per cent early and about none late"* at the solver-armed seat | **not in the cited file.** Derived from the four `.tot` files beside it, with `bands`: early (30598+30487)/(29943+30042) = **1.018**, late (54577+54559)/(54301+54513) = **1.003**; node counts are identical per position, so a time ratio is an nps ratio | number TRUE, citation WRONG — **F13** |
| *"ten undo sites, all read"* | `matrix_P1_threat_state.md:116-129` at `ffc5c10` lists exactly ten (`position.rs:142`; `dfpn.rs:720`,`:722`; `policy.rs:155`,`:156`; `policy.rs:398`,`:399`; `threat_oracle_tests.rs:222`,`:336`,`:344`) | **correct** for the baseline the past tense names. (At `d42d33d` there are thirteen, the three new ones being I4's third test, I5 and I6.) Derived, not transcribed |
| 15/15 mutants, 1 106 passed, the gates | re-run here at `d42d33d`: §2 and §3 | true; the gate COUNT at `3ef6706` is **F17** |

**The three `<landed>` slots against `p1_bench_prereg.md`** (revision 3,
sha256 `ebbfead197562d7d…`):

| D-603's slot | the prereg | consistent? |
|---|---|---|
| `tools/bench_delta.sh rev:ffc5c10 rev:<landed> 5` | §1 *"**Instrument:** `tools/bench_delta.sh rev:ffc5c10 rev:<landed> 5`"* | YES |
| *"against the per-band brackets [1.207, 1.307] early and [1.190, 1.290] late"* | §1.1 (`:104-109`) registers exactly those two | **YES — as amended at 11:28. It read `[1.20, 1.30]` when this round began: F10** |
| *"the identity leg over 128 searches at three seats → `<RESULT>`"* | §2.1 *"`(20 + 24 + 20) × 2` = **128 searches a side**"*, three seats | YES for the slot; the HEADLINE is **F11** |
| *"CI's twenty gates at `<landed>`"* | §0 *"`tools/ci.sh` … `GATE_TOTAL=20`"*; §3 *"twenty gates"* | YES |

---

## 2. The mutation re-run (F2's discharge)

Driver read first — `artifacts/p1_mutation_driver.py`, sha256
`7143a133ce972516954bff10345a9c2ec688fa67e91e8083420fd3f5073332b2` (round 1 read
`ecb3925b…`; the diff is the first-panic-line capture, F2's remedy, and nothing
else). Then re-run by me:

```
$ python3 /home/tom/Projects/HeXO-AlphaBeta/artifacts/p1_mutation_driver.py \
      /home/tom/pistol-wt/review-p1-impl-mut
REV d42d33d
… 15 UNMUTATED rows, all "-> pass" …
SUMMARY 15/15 dead, 0 alive
TREE CLEAN

$ diff artifacts/p1_mutation_d42d33d_v1.txt rerun.log ; echo $?
0        # byte-identical to the receipt
$ git -C /home/tom/pistol-wt/review-p1-impl-mut status --porcelain    # empty
```

**Every reason against its registered class** — the check F2 exists for:

| mutant | recorded reason | design §4's class | consistent? |
|---|---|---|---|
| M1 shift off by one | `threat_oracle_tests.rs:133` `live at Two` | misread masks, I1 | yes |
| M2 high-chunk OR dropped | `:401` `ConstQ -1/0 one side after 0,-3: Window { axis: ConstR … }` | straddling, I5 | yes |
| M3, M4, M5, M8, M9 | `:223` `undoing … did not restore the state` | undo / pruning / frame integrity, I2 | yes |
| M6 frame check removed | `should panic … FAILED` at I4 | LIFO refusal | yes |
| M7 centre-bit refusal removed | `should panic … FAILED` at I9 | desync guard | yes |
| M10 cell inverse off by one | `:546` `the snapshot's window set` | snapshot enumeration, I8 — the design names this assert | yes |
| M11 `PartialEq` includes the log | `:259` `insertion order changed the state itself` | equality, I7 | yes |
| M13 ConstS projection | `:112` `ply 1: the window table` | projection, I1 | yes |
| **M14 line field narrowed to 8 bits** | **`crates/pistol-solver/src/state.rs:108:13: THREAT_DESYNC: p2 stone on -32768,0 lands on a cell of its ConstR line that already holds one`** | §4: *"I6, by `THREAT_DESYNC` … `apply` refuses it (N7 — the refusal kills it, not a misread window)"* | **yes — the refusal at `state.rs:108`, and NOT `line.rs:66`, the key inverse's `expect` that round 1 measured as M14's death at I1/I5/I8. This is exactly what F2 asked for** |
| M15 transitions skipped at `index == 5` | `:133` `live at Two` | class sets, I1 | yes |
| M12 CALL-SITE | `state.rs:108:13: THREAT_DESYNC: p1 stone on 0,0 …` | §4: *"the state still holds the undone stone, and the next `apply` on that cell is refused with `THREAT_DESYNC`"* | **yes — verbatim the mechanism the design describes** |

The mutant→test mapping is unchanged from `3ef6706`:

```
$ diff <(sed -E 's/ BY .*//' artifacts/p1_mutation_d42d33d_v1.txt | sed 's/^REV.*/REV/') \
       <(sed -E 's/ BY .*//' artifacts/p1_mutation_3ef6706_v1.txt | sed 's/^REV.*/REV/')
       # no output
```

Round 1's eight self-designed mutants were not re-run: the binary is identical
(§4), so their result transfers.

---

## 3. The suite and the SIX gates at `d42d33d`, by their own log lines

**Full suite**, `/home/tom/pistol-wt/review-p1-impl`, launched with
`env -u CARGO_TARGET_DIR` so nothing was exported into the scratch-workspace
fixtures:

```
$ cargo test --workspace --locked
$ grep -E '^test result:' full.log | awk '{p+=$4;f+=$6;i+=$8} END {print NR,p,f,i}'
suites=173  passed=1106  failed=0  ignored=21
$ grep -c FAILED full.log             -> 0
$ grep -c '^test result: ok' full.log -> 173
$ grep -cE '^error' full.log          -> 0
```

Identical to round 1's counts at `3ef6706`, which is what a comment-only diff
should produce.

**The gates.** The brief named five; the pre-registration's §3 names **six**, and
D-603 (as amended mid-review) claims six *"re-run at that revision by REVIEW-impl
round 2 and cited in its report"*, so I ran the sixth as well. All in
`review-p1-impl-mut` at `d42d33d`, each cited by the script's own line:

| gate | script | its own log line |
|---|---|---|
| 8 | `tools/tactical_check.sh` | `selftest: 20 of 20 cases solved (required 20), 0 failed to reproduce` |
| 9 | `tools/determinism.sh` | `determinism: ok — 5 seat(s), no difference outside nps/time in any of them` (seats `radius`, `staged`, `staged-heuristics`, `staged-solver`, `staged-safety-net-cap`, each *"ok — 40 searches, 20 positions, no difference outside nps/time"*, run C one process per position) |
| 10 | `tools/search_oracle_check.sh` | `search_oracle_check: the always-on tier, in release` / `: the depths a debug build cannot afford` / `: the gated seat spends the budget it is given` / `: the solver call counters count what was asked`; final suite `test result: ok. 6 passed; 0 failed` |
| 11 | `tools/staged_soundness_check.sh` | `staged_soundness_check: all four parts passed` (last part `test result: ok. 1 passed; 0 failed`) |
| 12 | `tools/solver_oracle_check.sh` | `gate (a) PASS: 61 cases agree with R3'` / `gate (b) PASS: 38 proof trees re-verified full-width` / `gate (c) PASS: 29 wins, 118135 sigma placements replayed and revalued (26865 refused on collision)` / `gate (d) PASS: values agree at both table sizes` / `solver_oracle_check: all four gates passed` |
| 13 | `tools/solver_determinism.sh` | `solver_determinism: PASS — 61 cases, byte-identical transcripts` |

```
$ cargo fmt --all --check                                          # no output
$ cargo clippy --workspace --all-targets --locked -- -D clippy::all
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.83s
```

Rule 9, re-run because the test file grew four lines:

```
$ bash tools/file_justification_check.sh
file_justification_check: 363 tracked .rs/.sh files, 69 over the cap, all registered
                          in docs/rule9_justifications.md (69 entries)
```

`threat_oracle_tests.rs` is now 553 lines and already registered; `line.rs` is 265,
under the cap; `sets.rs` is 197.

---

## 4. Did the discharge introduce anything? — NO, and the instrument is the binary

**(a) Every changed line is a comment line:**

```
$ git diff 3ef6706 d42d33d -- crates | grep -E '^[+-]' | grep -vE '^(\+\+\+|---)' \
    | grep -vE '^[+-][[:space:]]*(///|//)'
       # no output
```

**(b) The shipped program does not move** — D-573's own instrument, applied here.
In `review-p1-impl-mut`, one `CARGO_TARGET_DIR`, each build after
`rm -f target/release/pistol`:

```
REV d42d33d  1413698a22ffbb95fd008b2f27c533c88e3399b8806b62f71b4f827e30271e6d
REV 3ef6706  1413698a22ffbb95fd008b2f27c533c88e3399b8806b62f71b4f827e30271e6d
REV d42d33d  1413698a22ffbb95fd008b2f27c533c88e3399b8806b62f71b4f827e30271e6d   # control, reproduces
```

**Not vacuous, verified both ways.** The `3ef6706` leg genuinely recompiles the
whole dependent chain:

```
   Compiling pistol-solver / pistol-search / pistol-engine / pistol-cli
    Finished `release` profile [optimized] target(s) in 2.95s
SHA 1413698a22ffbb95fd008b2f27c533c88e3399b8806b62f71b4f827e30271e6d
```

and the instrument has positive content — a revision that DOES differ gives a
different digest:

```
REV ffc5c10  78a7600adcf099de0b04149535f1f4bffe0b6c945609a3206d73a4e5ee853749
```

Both digests are exactly the ones the RED-TEAM report and the prereg's §1.2 dry
run recorded independently (`78a7600a…` baseline, `1413698a…` candidate), now
reproduced in a third worktree. **Consequence:** the RED-TEAM's whole result at
`3ef6706` — 1 167 813 state steps, 1 763 transcript pairs, 0 differing — is a
result about `d42d33d`'s program too, and so is round 1's mutation and gate work,
without re-running any of it.

**The design's amended Scope against the real diff, file for file:**

```
$ git diff --name-only ffc5c10 d42d33d
crates/pistol-search/src/position.rs                one doc comment, no code        Scope
crates/pistol-solver/src/cover.rs                   in-crate doc 1                  §2
crates/pistol-solver/src/lib.rs                     the five root-doc passages      §2
crates/pistol-solver/src/line.rs                    the new store                   Scope
crates/pistol-solver/src/sets.rs                    two doc sentences, no code      §2 (amended)
crates/pistol-solver/src/state.rs                   apply/undo                      Scope
crates/pistol-solver/src/table.rs                   reduced to the mask vocabulary  Scope
crates/pistol-solver/tests/threat_oracle_tests.rs   §3's own deliverable            §3
```

Eight files, all inside the amended Scope, nothing outside it. Both `sets.rs`
hunks are `///` lines, so §2's *"No code in `sets.rs` changes"* is true.
**Scope matches the diff file for file. §6 does not — F15.**

---

## 5. What round 1 said it did not reach

* **The identity leg of §5** — still NOT reachable: the governed run is over
  `<landed>`, which does not exist, and `p1_bench_prereg.md` is now at revision 3,
  whose own fresh-context gate has not run (its round 2 returned
  *"FAIL (B3; MAJOR M4, M5, M6)"*). CLAUDE.md makes that review a precondition of
  the first run it governs. What IS settled is that the leg's answer at `d42d33d`
  cannot differ from its answer at `3ef6706`, by §4(b).
* **The matrix, the selection record, the bench pre-registration** — round 1 did
  not re-litigate them. This round read `p1_bench_prereg.md` because D-603 cites
  it, and that is where F10 came from. It was the one unreached item worth
  reaching, and it produced the round's sharpest finding — independently of, and
  simultaneously with, the prereg gate's own round-2 M6, which found the same
  thing at 11:19.
* **`pistol-core`'s window enumeration and `sets.rs`'s class algebra** — `sets.rs`
  is now inside the diff, so I checked its change is doc-only (§4);
  `windows_through_indexed` was read at `crates/pistol-core/src/window.rs:112-121`
  for F14's arithmetic. The class algebra is left as given.
* **The design's own premises** — settled by three design rounds; not re-attacked.

---

## 6. Findings

### BLOCKING

None standing.

### MAJOR

**F11 — D-603's headline is false under both of its readings: it asserts as
MEASURED a result its own body records as an unfilled slot, and scopes byte
identity to one seat where the design, the prereg and its own body all say three.**

The headline, unchanged by the mid-review rewrite:

> *"…PER-AXIS LINE BITBOARDS WITH A LOGGED, LIFO-OR-PANIC UNDO — **BYTE-IDENTICAL
> SEARCH OUTPUT, MEASURED AT THE INSTRUMENT SEAT AND CLAIMED NOWHERE ELSE.**"*

*Minimal reproducer,* four quotations, two of them from the same D-line:

```
D-603 headline : "BYTE-IDENTICAL SEARCH OUTPUT, MEASURED AT THE INSTRUMENT SEAT
                  AND CLAIMED NOWHERE ELSE"
D-603 body     : "the identity leg over 128 searches at three seats -> <RESULT>"
                 …and of the solver-armed seat: "the package banks nothing there
                 and checks byte-identity there instead"
p1_design.md §5: "the two-binary diff at THREE seats"
p1_bench_prereg.md §2 : "the claim is BIT-IDENTITY of search output over the 128
                 searches this section registers, at the three seats it names —
                 that is the ground D-495's no-SPRT exemption rests on here, and
                 neither the results document nor the ADR widens it to search
                 output simpliciter (m8)"
```

The clause `MEASURED … AND CLAIMED NOWHERE ELSE` has two possible antecedents and
is wrong on each:

* **read as byte-identity** (the noun it actually follows) — it is not measured at
  all: `<RESULT>` is one of the three slots, filled at the landing commit and
  nowhere else; and it is registered at THREE seats, not at the instrument seat,
  in the exact place the prereg's `m8` registered a guard against the ADR
  re-scoping the claim. The identity ground is what makes hard rule 6's SPRT
  unnecessary under D-495, so an ADR recording it as a one-seat measurement
  records the wrong ground for the exemption in this project's permanent log.
* **read as the GAIN** (whose seat the prereg's §1 does restrict to the instrument
  seat) — it is still not measured: the landing bench is the other unfilled slot,
  `<nps early> / <nps late>`.

This is F-P1.8's own class — *a registered run is not taken by registering it* —
moved up into the ADR headline, which is the sentence a later reader quotes.

*Discharge:* the headline states the gain's seat and leaves both results to their
slots (*"byte-identity registered at three seats, `<RESULT>`; the bench at the
instrument seat only, `<nps>`"*), or drops `MEASURED`. One sentence; no code; no
review of §1, §3 or §4 reopens.

**F16 — this round was dispatched against two governing documents that are pinned
to no revision, and both were rewritten while it ran.**

*Minimal reproducer:*

```
$ git status --porcelain | grep -E 'decisions|prereg'
 M docs/decisions.md
?? docs/experiments/p1_bench_prereg.md
$ stat -c '%y %n' docs/decisions.md docs/experiments/p1_bench_prereg.md
2026-09-04 11:28:15 docs/decisions.md              # D-603 rewritten mid-review
2026-09-04 11:27:45 docs/experiments/p1_bench_prereg.md   # revision 2 -> revision 3
```

The brief pinned the implementation by commit and the design by `git stash
create`, and named these two by path. During the review D-603's bracket changed
from `[1.20, 1.30]` to the per-band pair, its verification pin gained the landing
revision and the full 40-character SHAs, and its gate count changed from *"five"*
to *"six"*; the pre-registration went from revision 2 to revision 3. Two of this
round's findings (F10, F12) are therefore closed against a file state that no
commit holds, and **no reader can reproduce either the finding or its closure**:
CLAUDE.md requires a review to be dispatched against a NAMED REVISION, and D-602's
own rule is that *"a working tree is not a revision… output taken from it belongs
to no commit at all"*. That rule was written three D-lines before D-603 and this
review is the first thing to break it.

This is a finding about the round's dispatch, not about the code, and it does not
by itself block the landing. *Discharge:* a review of uncommitted documents is
dispatched against a `git stash create` SHA covering **all** of them, exactly as
this brief did for the design set; and the implementing session does not edit the
subject while a review of it is outstanding (CLAUDE.md: *"A WP is not landable
while its reviews are outstanding"* — the corollary is that its documents do not
move under the reviewer either).

### MINOR

**F13 — D-603 cites `p1_rt_round2/solveron_run.log` for a number that file does not
contain.**

*Reproducer:*

```
$ wc -l artifacts/p1_rt_round2/solveron_run.log                            -> 9
$ grep -cE "per cent|ratio|nps|%" artifacts/p1_rt_round2/solveron_run.log  -> 0
```

The nine lines are the run's receipt: idle window, three sha256 lines, four
`… exit 0 … errors 0` lines, `end`. The claim cited to it is *"AT THE SOLVER-ARMED
SEAT THE GAIN IS ABOUT TWO PER CENT EARLY AND NONE LATE"*. The number is real and
lives one file over: from `solveron.{base,E}.{1,2}.tot` with `bands`, node counts
identical per position so a time ratio is an nps ratio, I derive **early 1.018,
late 1.003**, which is what the sentence says. This is *"an artifact not holding
what it was cited for"* — the first of the three RECORD failures D-603 itself
reports the matrix's red team finding — recurring inside D-603. *Discharge:* cite
the `.tot` files or the directory, or print the two ratios.

**F14 — I5's comment is no longer false, but three residues remain, and the fix is
deletion rather than a fourth refinement.**

`crates/pistol-solver/tests/threat_oracle_tests.rs:419-427`:

> *"on the stones' own axis most windows through them straddle the boundary, and
> one window of each innermost stone does not — the window starting six back from
> position -1 has offset 58 and 58 + 6 == 64, which `LineStore::run`'s
> `offset + count > 64` does not straddle, and the one starting at position 0 has
> offset 0. Every window is compared either way; the sentence is here because two
> review rounds read it as a claim."*

*Minimal reproducer, arithmetic against the enumeration (`window.rs:112-121`: per
axis, windows start at `at - back` for `back ∈ 0..6`) and against `run`'s own
condition (`line.rs:175`).* At the `-1/0` boundary the fixture places six stones at
positions `-3..=2` (`threat_oracle_tests.rs:437`). The distinct own-axis windows
through them start at `-8..=2`; one straddles iff `start.rem_euclid(64) + 6 > 64`,
i.e. iff `start ∈ {-5,…,-1}`.

1. **`most`** — 5 of the 11 DISTINCT windows straddle, a minority; 24 of the 36
   stone-window pairs do, a majority. True on one reading, false on the other,
   which is the state F4 was raised to end.
2. **`one window of each innermost stone`** — true, but read as exclusive it is
   false: the stone at `-3` has THREE non-straddling own-axis windows (starts
   `-8`, `-7`, `-6`; offsets 56, 57, 58).
3. **`starting six back from position -1`** — the window it names starts at `-6`,
   which is `back = 5`, the last index the enumeration yields; six back is `-7`.

None of this can change a reading of the test, which compares every window either
way — so by CLAUDE.md's own test the clause is prose that constrains nothing, and
the discharge is to DELETE it (keeping: the fixture crosses each boundary, and
every window through every stone is compared) rather than restate it a fourth
time. Two further notes on the same comment, both CLAUDE.md Code style, which the
brief puts in scope: it is now a nine-line paragraph (*"if a comment needs a
paragraph, the code or the design doc is the wrong shape"*), and its last clause is
about the review history rather than about the code.

**F15 — the design's §6 still says *"No change to `sets.rs`"* while its own amended
Scope and §2 license the change the diff makes.**

*Reproducer,* three lines of the same design revision (`289f184`, `p1_design.md`
sha256 `2515304fddd13cc7…`):

```
:51  Scope : "the class-set CODE in `sets.rs` (two of its doc sentences change, §2)"
:217 §2    : "`sets.rs`'s two sentences about *the table* … say *the store* …
              No code in `sets.rs` changes."
:293 §6    : "- No change to `sets.rs`, to any query, to any refusal message's
              token, to any config or fixture."
```

The amendment reached Scope and §2 and not §6, so the design now both licenses and
forbids the same two lines, and the implementation contradicts §6 as written. A
DESIGN gap, not an implementation one — the code did what Scope and §2 say — and
the same shape as round 1's F5, which is what produced the amendment.
*Discharge:* §6's bullet takes the word Scope already uses — *"No change to
`sets.rs`'s CODE"*.

**F17 — D-603's *"six gates … green by their own log lines"* at `3ef6706` is
supported by no log for the sixth, and its `d42d33d` sentence cited this report's
result before this report existed.**

*Reproducer:*

```
$ grep -n "the five gates that read this state" docs/experiments/p1_impl_REVIEW.md
363:## 8. Check 6 — the five gates that read this state
      (its table lists gates 9, 10, 11, 12, 13 — gate 8, the tactical fixture,
       is not among them and was not run at 3ef6706)
$ awk '/^D-603/' docs/decisions.md | grep -o "the six gates that read this state green by their own log lines"
the six gates that read this state green by their own log lines
```

The count was corrected from *"five"* to *"six"* at 11:28 to satisfy the prereg's
§3 (the prereg gate's own M6), but the only report that ran gates at `3ef6706` ran
five, so the sixth has no cited log at that revision. The same sentence's second
half — *"the suite and the six gates re-run at that revision by REVIEW-impl round 2
and cited in its report"* — was written before this report existed; it is TRUE only
because this round ran gate 8 as well (§3 above, `selftest: 20 of 20 cases solved`).
Had a gate been red, the ADR would have shipped a false claim about a review that
had not reported. *Discharge:* say *"five"* at `3ef6706`, which is what round 1
cites, and leave the six-gate claim to `d42d33d`, where this report cites all six;
and no ADR states a review's result before the review returns it.

**F18 — the mutation driver is not sha-anchored, though §4 requires it to be.**
The design's §4: *"a driver sha-anchored beside its receipt
(`artifacts/p1_mutation_driver.py`; `artifacts/` is never committed, rule 8)"*.
`/usr/bin/grep -rln "7143a133" docs/` returns only this report; the ledger's run-9
row names the driver by path with no digest where every earlier run's row carries
one. The
RECEIPT's digest did land in D-603 mid-review (`1c831daa…`), so only the driver is
outstanding. Since `artifacts/` is never committed, the digest is the only thing
tying the claim to the file. Its value, read this round:
`7143a133ce972516954bff10345a9c2ec688fa67e91e8083420fd3f5073332b2`.

### Raised, and closed by an edit made during this review — NOT counted in the verdict

**F10 (was BLOCKING) — D-603 named the bracket the pre-registration WITHDREW.**
As first read, its landing-bench slot said *"against the bracket **[1.20, 1.30]**
registered in `p1_bench_prereg.md` before any run"*, while the prereg registered
early `[1.207, 1.307]` and late `[1.190, 1.290]` and recorded `[1.20, 1.30]` as
*"neither ground's arithmetic … in the late band admitted 1.295 — outside what the
ground produces — while calling 1.195 a miss, which the ground includes."* The two
documents therefore accepted and rejected the same landing bench: a late-band
1.295 was inside the ADR's bracket and outside the prereg's, and 1.195 the
reverse. That is a way to reach a wrong answer out of a governing document, which
D-424 says is never overruled, only fixed — hence BLOCKING when raised.
**Closed at 11:28** (`decisions.md` sha256 `31d1b485811aec67…`): the slot now reads
*"against the per-band brackets [1.207, 1.307] early and [1.190, 1.290] late"*.
The prereg's own round-2 gate found the same defect independently at 11:19 (its
M6), which is why it was already being fixed.

**F12 (was MINOR) — D-603 pinned its verification at the superseded `3ef6706` and
cited only the superseded receipt.** As first read, all of *"15 of 15 … 1 106
passed … the gates green"* were pinned `at \`3ef6706\``, and
`artifacts/p1_mutation_d42d33d_v1.txt` (`1c831daa…`), the receipt at the tip and
the one carrying F2's death reasons, was cited nowhere. **Closed at 11:28**: the
line now reads *"THE VERIFICATION THAT IS TAKEN, EACH CLAIM AT THE REVISION IT WAS
TAKEN AT"*, splits `3ef67068a158…` from
*"the landing revision `d42d33d0c8ad640d7138a14caa5a1176d49fd2c0`, which adds two
doc comments and a test comment and no behaviour"*, and cites the `d42d33d`
receipt with the digest this round reproduced byte for byte.

### Observations, not findings

* **F19** — round 1's F7 stands unchanged: `warning: unused import: 'generate_turns'`
  at `crates/pistol-solver/src/policy.rs:1:44` in every release gate log. Re-confirmed
  pre-existing: `git show ffc5c10:crates/pistol-solver/src/policy.rs | head -1` is
  the same line, and `git diff --name-only ffc5c10 d42d33d -- crates/pistol-solver/src/policy.rs`
  is empty.
* **F20** — round 1's F9 stands, at seven commits now rather than five. The landing
  shape is an operator act; it is named, not judged.
* **F21** — round 1's F8 (the third privacy doctest needs BOTH locks opened to flip)
  is unaffected: `lib.rs` is untouched by the discharge, and it was not re-run.

---

## 7. What this round did not do

The identity leg's governed run and the landing bench belong to
`p1_bench_prereg.md` and cannot be taken before `<landed>` exists and before that
document's revision 3 passes a fresh-context gate; neither was run here, and §4(b)
is the reason the leg's answer cannot have moved since `3ef6706`. Round 1's eight
self-designed mutants were not re-designed or re-run — the binary is identical, so
their result transfers. The matrix and the selection record were not re-litigated;
`p1_bench_prereg.md` was read only where D-603 cites it, and its revision 3 is not
adjudicated here — that is its own gate's round. The design's premises were settled
by three design rounds and are not re-attacked.

---

VERDICT: FAIL (F11, MAJOR — D-603's headline, *"BYTE-IDENTICAL SEARCH OUTPUT,
MEASURED AT THE INSTRUMENT SEAT AND CLAIMED NOWHERE ELSE"*, is false under both of
its readings: read as identity it names one seat where the design's §5, the
prereg's §2 and the same D-line's own body all say three, and it says MEASURED of a
result the same line records as the unfilled slot `<RESULT>`; read as the gain it
says MEASURED of the other unfilled slot, `<nps early> / <nps late>`. F16, MAJOR —
the round was dispatched against `docs/decisions.md` and `p1_bench_prereg.md`
pinned to no revision, and both were rewritten while it ran (11:28:15 and 11:27:45,
D-603's bracket, revision pin and gate count; the prereg revision 2 → 3), so two
findings closed mid-review against a file state no commit holds, which is D-602's
own rule failing in the review of the line D-602 precedes. Four MINOR: F13, the
ADR cites `solveron_run.log` for a ratio that nine-line file does not contain
(derived 1.018 / 1.003 from the `.tot` files beside it); F14, I5's comment is no
longer false but is true on only one of two readings, names the wrong window index,
and should be deleted rather than refined a fourth time; F15, the design's §6 still
forbids the `sets.rs` change its own amended Scope licenses; F17, *"the six gates …
green"* at `3ef6706` has no log for the sixth and the `d42d33d` half cited this
report's result before this report existed. F10 (BLOCKING when raised — the ADR
named the bracket the prereg withdrew) and F12 are recorded as closed by edits made
during the review and are not counted.
**Round 1's five findings are discharged and THE CODE IS NOT WRONG ON ANY CHECK IN
THIS REPORT**: D-603 records all three things §2 requires and answers D-254's flip
clause as written; every measured number in it traces to a digested artifact except
F13's citation; `decision_key_check.sh` passes at 604 keys; the re-run mutation
receipt is byte-identical at 15/15 with M14 dying by the refusal the design's N7
names and M12 by the mechanism §4 describes; `sets.rs` carries no stale store noun;
1 106 tests pass and 0 fail across 173 suites; all SIX gates the prereg names are
green by their own log lines; fmt and clippy are clean; rule 9 is met; the amended
Scope matches the diff file for file; and the release binary at `d42d33d` is
byte-identical to `3ef6706`'s — `1413698a22ff…`, the digest the RED-TEAM recorded,
against `78a7600a…` at the baseline — so the discharge moved no behaviour at all
and every result taken at `3ef6706` is a result about this revision.)
