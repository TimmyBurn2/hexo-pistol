# P1 — REVIEW-impl ROUND 3: remedies-only, against the diff

**Implementation revision reviewed:** `e32f8c4bf51ec99ec3ffcb3f898910de809bc7cd`,
which IS `p1/impl`'s tip:

```
$ git rev-parse p1/impl
e32f8c4bf51ec99ec3ffcb3f898910de809bc7cd
```

Round 2 reviewed `d42d33d0c8ad640d7138a14caa5a1176d49fd2c0`. **The diff
adjudicated** is `git diff d42d33d e32f8c4b -- crates` — one test comment, three
lines for six, in `crates/pistol-solver/tests/threat_oracle_tests.rs`, and
nothing else in the whole tree:

```
$ git diff --stat d42d33d0c8ad640d7138a14caa5a1176d49fd2c0 e32f8c4bf51ec99ec3ffcb3f898910de809bc7cd
 crates/pistol-solver/tests/threat_oracle_tests.rs | 9 +++------
 1 file changed, 3 insertions(+), 6 deletions(-)
```

**Pinned document revision:** `189c6d6e73e17deab45cc36ffb8aa929f4a8f7a7` (a
`git stash create` commit on `dev`). **Every document below was read with
`git show <rev>:<path>` and never from the working tree**, which is the only
reason the paragraph after this one does not void anything.
`docs/decisions.md`'s tail at that revision is `D-603`.

**At the START of this review all five matched the working tree. At its END,
TWO NO LONGER DO** — they were rewritten while the round ran:

```
$ for f in decisions.md experiments/p1_design.md experiments/p1_bench_prereg.md \
           experiments/opt_arc_ledger.md experiments/p1_impl_REVIEW_rev2.md; do … done

           START (before any check)                 END (after every check)
docs/decisions.md                        MATCH                 MATCH
docs/experiments/p1_design.md            MATCH                 MATCH
docs/experiments/p1_bench_prereg.md      MATCH        →      DIFFER   (mtime 12:00:43)
docs/experiments/opt_arc_ledger.md       MATCH        →      DIFFER   (mtime 12:01:21)
docs/experiments/p1_impl_REVIEW_rev2.md  MATCH                 MATCH
```

**Nothing in this report closed or moved underneath it**, and that is checked
rather than asserted — see F25. Every line this round adjudicates is quoted from
`189c6d6e…`, and the four things I read the pre-registration FOR (§1's instrument
line, §1.1's two brackets, §2.1's 128 searches, §0/§3's twenty gates) are
byte-identical in the rewritten copy as well, so §1.1's slot table below holds at
both.

**Two documents outside the pin were read**, and this says so because the brief
requires it. `docs/experiments/p1_REDTEAM.md` and
`docs/experiments/p1_impl_REVIEW.md` (round 1's report) are untracked in the live
tree and were read from it, because D-603 makes claims ABOUT them — *"a RED-TEAM
that found no wrong byte … 1 763 two-binary transcript pairs over six committed
configs"* and *"the five gates REVIEW-impl round 1 ran"* — and F17 asks whether
those attributions are true, which cannot be answered without them. Neither is a
subject of this round; both are cited only for what they say about themselves.

**HEAD:** `ffc5c10f4d16356f574e3221a023f78399d2e3bb` on `dev` (the branch point;
`dev` has not moved). **Date:** 2026-09-04. **Round 3 — REMEDIES-ONLY, the last
round the loop grant holds.**

**Where everything ran.** `/home/tom/pistol-wt/review-p1-impl`, detached at
`e32f8c4b`, for the one `cargo test --workspace --locked` launched with
`env -u CARGO_TARGET_DIR`; `/home/tom/pistol-wt/review-p1-impl-mut`, detached at
`e32f8c4b` with `CARGO_TARGET_DIR` = its own `target`, for the mutation re-run,
the six gates, fmt/clippy/rule 9 and the binary-identity experiment. Never the
live tree, never `/tmp`, never `p1-measure`, `p1-mutants`, `rt-base`, `rt-cand`.
Both worktrees are left in place, detached at `e32f8c4b`, clean, no branch
created. No worktree was created and none removed.

---

## 1. The remedy table — BY EXECUTION (D-591)

| round-2 finding | remedy offered | what I ran | holds? |
|---|---|---|---|
| **F11 (MAJOR)** — the ADR headline said MEASURED of two unfilled slots and named ONE seat where three are registered | headline rewritten: *"THE SEARCH'S OUTPUT UNCHANGED AND CHECKED SO AT THREE SEATS, THE SPEED CLAIMED AT THE INSTRUMENT SEAT AND NOWHERE ELSE"* | read D-603 at the pin; split it on `.` and grepped every `measured`/`checked`/slot token; checked its three slots against `p1_bench_prereg.md` §1, §1.1, §2.1, §3 and §0 at the same pin; read the design's §5 | **YES on both prongs.** Three seats now, matching design §5, prereg §2 and the line's own body; and **the body says MEASURED of nothing unfilled** — every `measured` traces to a digested artifact (§1.1 below). One residue, recorded as an observation and not a finding: §1.1 |
| **F13 (MINOR)** — the ADR cited `solveron_run.log` for a ratio that file does not contain | the sentence now names `artifacts/p1_rt_round2/solveron.{base,E}.{1,2}.tot` as the source and says the log carries what it carries | derived the ratio myself from the four `.tot` files with `bands`; checked node identity across all four; re-read the nine-line log and `solveron_run.sh` | **YES on the ratio and the citation** — my independent derivation is **early 1.0183, late 1.0030**, so *"about two per cent early and none late"* is honest (§1.2). One clause of the replacement is wrong: **F23** |
| **F14 (MINOR)** — I5's comment: delete the straddle clause, do not refine it a fourth time | the clause is deleted; the comment now says what the fixture exercises | read the comment at `e32f8c4b`; checked each surviving claim against `line.rs` (`CHUNK_LEN`, `project`, `div_euclid`) and against `assert_matches_board` (`threat_oracle_tests.rs:397-403`) | **YES on the deletion.** The straddle claim and the review-history clause are both gone, and what replaced them is true and verified in the code. One SURVIVING sentence of the same comment is false: **F24** |
| **F15 (MINOR)** — the design's §6 forbade the `sets.rs` change its own Scope licenses | §6's bullet takes the word Scope uses | read §6, Scope (`:51`) and §2 (`:217-219`) at the pin; diffed `sets.rs` `ffc5c10 → e32f8c4b` | **YES.** §6 now reads *"no CODE change in `sets.rs`"*, and the diff is two `///` lines and nothing else — exactly the two sentences §2 names |
| **F17 (MINOR)** — *"the six gates … green"* at `3ef6706` had no log for the sixth; the `d42d33d` half cited this report's result before it existed | the line now says *"the five gates REVIEW-impl round 1 ran"* at `3ef6706` and leaves the six-gate claim to `d42d33d`/round 2 | read round 1's §8 (`p1_impl_REVIEW.md:363`) and its gate table; compared mtimes; **re-ran all six gates myself at `e32f8c4b`** (§3) | **YES on both halves.** Round 1's §8 is headed *"the five gates"* and lists gates 9-13 — gate 8 is not among them, so *"five"* is what round 1 cites. Round 2's report existed (mtime 11:34:34) before D-603 was written (11:36:00), so no result is cited before its review returned. The revision the second half is pinned to is now stale: **F22** |

### 1.1 F11, clause by clause

**The headline against the three registrations.**

| headline clause | what the pinned documents register | true? |
|---|---|---|
| *"CHECKED SO AT THREE SEATS"* | `p1_design.md` §5: *"the two-binary diff at THREE seats"*; `p1_bench_prereg.md` §2: *"at the three seats it names"*, §2.1's `SEATS` heredoc lists `tactical_staged_v0.toml`, `instrument_v0.toml`, `gate_staged_solver_v0.toml` | **YES on the count.** Round 2's *"one seat where three are registered"* is gone |
| *"THE SPEED CLAIMED AT THE INSTRUMENT SEAT AND NOWHERE ELSE"* | prereg §1: *"**Seat:** the instrument seat only … this document registers no bracket at any solver-armed seat and the package claims no gain there"*; design §6: *"No claim about the solver's own throughput … the package banks none of it"* | **YES.** This is the clause F11 asked for, and it is now attached to the gain rather than to the identity |

**Every `measured` in the body, against what it is measured from:**

```
$ git show 189c6d6e…:docs/decisions.md | awk '/^D-603/' | tr '.' '\n' \
    | /usr/bin/grep -inE "measured|<nps|<RESULT>|<landed>"
```

| the body's `measured` | its artifact | unfilled? |
|---|---|---|
| *"window lookup as a measured hotspot"* | a quotation of D-254's own flip clause | no — it is the clause, not a result |
| *"THE PREMISE … WAS MEASURED"*, 5.3-8.6 % | `p1_counters_ffc5c10_v2.txt` `65a16dee…` | no |
| lazy seam *"measured 1.028 / 1.021"* | `p1_mx_bench_A_v1.txt` `87a081a5…` | no |
| O-E *"measured 1.257 / 1.240"* | `p1_mx_bench_E_v1.txt` `d0b1b384…` | no |
| O-E0 *"measured 1.190 / 1.188"* | `p1_mx_bench_E0_v1.txt` | no |

**No `measured` in D-603 stands against an unfilled slot.** The two slots
(`<nps early> / <nps late>` and `<RESULT>`) appear once each, inside the
`THE THREE SLOTS BELOW` sentence, and carry no verb of measurement.

**The three slots against `p1_bench_prereg.md` revision 3 at the same pin:**

| D-603's slot | the prereg | consistent? |
|---|---|---|
| `tools/bench_delta.sh rev:ffc5c10 rev:<landed> 5` → `<nps early> / <nps late>` | §1 *"**Instrument:** `tools/bench_delta.sh rev:ffc5c10 rev:<landed> 5`"* | YES |
| per-band brackets **[1.207, 1.307]** early, **[1.190, 1.290]** late | §1.1's table registers exactly those two | YES — round 2's F10 stays closed |
| the identity leg over **128 searches at three seats** → `<RESULT>` | §2.1: *"`(20 + 24 + 20) × 2` = **128 searches a side**"*, three seats named in the `SEATS` heredoc | YES |
| CI's **twenty** gates at `<landed>` | §0 *"`tools/ci.sh` … its `GATE_TOTAL=20`"*; §3 *"twenty gates"*; and at `e32f8c4b` itself `tools/ci.sh:21` is `readonly GATE_TOTAL=20` | YES |

### 1.2 F13, the ratio derived rather than read off

`bands` is 24 rows, twelve `early` then twelve `late`; each `.tot` row is
`nodes time`, written by `solveron_run.sh`'s own `awk` from the engine's
`totals` lines. Node identity first, because it is what makes a time ratio an
nps ratio:

```
node columns identical across all four files: True
early: base=61085 cand=59985 ratio=1.018338  (gain 1.834 %)
       per-rep: rep1 1.021875  rep2 1.014813
late : base=109136 cand=108814 ratio=1.002959  (gain 0.296 %)
       per-rep: rep1 1.005083  rep2 1.000844
```

**The ADR's characterisation is honest.** *"About two per cent early and none
late"* against 1.8 % and 0.3 % is a fair reading of the numbers, in the direction
that under-claims rather than over-claims; the run's own log records `busy=[]` at
both ends and `errors 0` on all four legs; the seat is
`configs/bench_wp18c_solver_on.toml` (`8414509a…` in the log), which is one of
the three committed solver-armed configs the design's §6 names. The citation now
points at the files that hold the number.

---

## 2. The mutation re-run — the remedies moved nothing

Driver read first: `artifacts/p1_mutation_driver.py`, sha256
`7143a133ce972516954bff10345a9c2ec688fa67e91e8083420fd3f5073332b2` — **the same
digest round 2 read**, so the instrument did not move between the rounds. Then
re-run by me in `review-p1-impl-mut` at `e32f8c4b`:

```
$ python3 /home/tom/Projects/HeXO-AlphaBeta/artifacts/p1_mutation_driver.py \
      /home/tom/pistol-wt/review-p1-impl-mut
REV e32f8c4
… 15 UNMUTATED rows, all "-> pass" …
SUMMARY 15/15 dead, 0 alive
TREE CLEAN

$ diff artifacts/p1_mutation_e32f8c4_v1.txt rerun.log
33a34
> EXIT=0        # my own marker; the 33 receipt lines are byte-identical
$ sed '$d' rerun.log | sha256sum
c8f9e849940eb63abf11a0a50e75fe61b866d9e550bc51cec59c947563883d71
$ sha256sum artifacts/p1_mutation_e32f8c4_v1.txt
c8f9e849940eb63abf11a0a50e75fe61b866d9e550bc51cec59c947563883d71
$ git -C /home/tom/pistol-wt/review-p1-impl-mut status --porcelain   # empty
```

**15 of 15 DEAD, byte-identical to the receipt.** Every death reason is the one
round 2 checked against its registered class; M14 still dies by the refusal
(`state.rs:108:13: THREAT_DESYNC: p2 stone on -32768,0 …`) and not by the key
inverse's panic, and M12 by the same refusal at the origin.

**The receipt at this revision differs from `d42d33d`'s in exactly one place, and
it is the comment's line count:**

```
$ diff artifacts/p1_mutation_d42d33d_v1.txt artifacts/p1_mutation_e32f8c4_v1.txt
1c1
< REV d42d33d
> REV e32f8c4
26c26
< MUTANT M10 … BY crates/pistol-solver/tests/threat_oracle_tests.rs:546:9: …
> MUTANT M10 … BY crates/pistol-solver/tests/threat_oracle_tests.rs:543:9: …
$ git show e32f8c4b:crates/pistol-solver/tests/threat_oracle_tests.rs | sed -n 543p
        assert_eq!(listed, expected, "seed {seed}: the snapshot's window set");
```

The comment lost three lines, so M10's assert moved from 546 to 543. That is the
whole behavioural footprint of this diff.

---

## 3. The suite and the SIX gates at `e32f8c4b`, by their own log lines

**Full suite**, `/home/tom/pistol-wt/review-p1-impl`, `env -u CARGO_TARGET_DIR`:

```
$ cargo test --workspace --locked
$ grep -E '^test result:' full.log | awk '{p+=$4;f+=$6;i+=$8} END {print NR,p,f,i}'
suites=173  passed=1106  failed=0  ignored=21
$ grep -c FAILED full.log              -> 0
$ grep -c '^test result: ok' full.log  -> 173
$ grep -cE '^error' full.log           -> 0
$ tail -1 full.log                     -> EXIT=0
```

**1 106 passed, 0 failed, over 173 suites** — the same counts rounds 1 and 2
report at `3ef6706` and `d42d33d`, which is what a comment-only diff must
produce.

**The six gates the pre-registration's §3 names**, all run by me in
`review-p1-impl-mut` at `e32f8c4b`, each cited by the script's own line and never
by a wrapper's exit status:

| gate | script | its own log line |
|---|---|---|
| 8 | `tools/tactical_check.sh` | `selftest: 20 of 20 cases solved (required 20), 0 failed to reproduce` (`configs/instrument_v0.toml, configs/gate_v0.toml`) |
| 9 | `tools/determinism.sh` | `determinism: ok — 5 seat(s), no difference outside nps/time in any of them` — seats `radius`, `staged`, `staged-heuristics`, `staged-solver`, `staged-safety-net-cap`, each `ok — 40 searches, 20 positions, no difference outside nps/time`, run C one process per position |
| 10 | `tools/search_oracle_check.sh` | `search_oracle_check: the always-on tier, in release` / `: the depths a debug build cannot afford` / `: the gated seat spends the budget it is given` / `: the solver call counters count what was asked`; suites `ok. 6 passed`, `5 passed`, `2 passed`, `1 passed`, `5 passed`, `9 passed`, `6 passed`; `0 failed` throughout |
| 11 | `tools/staged_soundness_check.sh` | `staged_soundness_check: all four parts passed` (`selftest: 20 of 20 cases solved`; `test result: ok. 1 passed; 0 failed`) |
| 12 | `tools/solver_oracle_check.sh` | `gate (a) PASS: 61 cases agree with R3'` / `gate (b) PASS: 38 proof trees re-verified full-width` / `gate (c) PASS: 29 wins, 118135 sigma placements replayed and revalued (26865 refused on collision)` / `gate (d) PASS: values agree at both table sizes` / `solver_oracle_check: all four gates passed` |
| 13 | `tools/solver_determinism.sh` | `solver_determinism: PASS — 61 cases, byte-identical transcripts` |

Mechanical law and rule 9, in the same worktree at the same revision:

```
$ cargo fmt --all --check                                            # no output
$ cargo clippy --workspace --all-targets --locked -- -D clippy::all
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.70s
$ bash tools/file_justification_check.sh
file_justification_check: 363 tracked .rs/.sh files, 69 over the cap, all registered
                          in docs/rule9_justifications.md (69 entries)
$ wc -l crates/pistol-solver/tests/threat_oracle_tests.rs crates/pistol-solver/src/line.rs \
        crates/pistol-solver/src/sets.rs
550  265  197
```

`threat_oracle_tests.rs` is three lines shorter than at `d42d33d` and still
registered; nothing crossed the cap in either direction.

---

## 4. Did the diff introduce anything? — NO, and the binary is the instrument

**(a) Every changed line is a comment line**, both against the round-2 revision
and against the revision the RED-TEAM measured:

```
$ git diff d42d33d e32f8c4b -- crates | grep -E '^[+-]' | grep -vE '^(\+\+\+|---)' \
    | grep -vE '^[+-][[:space:]]*(///|//)'
       # no output
$ git diff 3ef6706 e32f8c4b -- crates | grep -E '^[+-]' | grep -vE '^(\+\+\+|---)' \
    | grep -vE '^[+-][[:space:]]*(///|//)'
       # no output   (two files: sets.rs 4 lines, threat_oracle_tests.rs 5)
```

**(b) The shipped program does not move.** In `review-p1-impl-mut`, one
`CARGO_TARGET_DIR`, each build after `rm -f target/release/pistol`:

```
REV e32f8c4  1413698a22ffbb95fd008b2f27c533c88e3399b8806b62f71b4f827e30271e6d
REV ffc5c10  78a7600adcf099de0b04149535f1f4bffe0b6c945609a3206d73a4e5ee853749
REV e32f8c4  1413698a22ffbb95fd008b2f27c533c88e3399b8806b62f71b4f827e30271e6d   # control
   Compiling pistol-solver v0.0.1 (…/review-p1-impl-mut/crates/pistol-solver)
    Finished `release` profile [optimized] target(s) in 3.49s
```

**Not vacuous, verified both ways**: the `ffc5c10` leg recompiles the dependent
chain and produces a DIFFERENT digest, so the instrument has positive content;
and the control at `e32f8c4b` reproduces. `1413698a22ff…` is the digest
`d42d33d`'s and `3ef6706`'s binaries carry and the one the RED-TEAM recorded as
its subject; `78a7600a…` is the RED-TEAM's baseline.

**What that means.** Round 2 established the same identity at `d42d33d`, and it
**still holds at `e32f8c4b`**: the release binary is byte-for-byte the program
the RED-TEAM adversarially exercised (1 167 813 state steps, 1 763 two-binary
transcript pairs, 0 differing) and the one round 1 and round 2 measured. So every
result taken at `3ef6706` and at `d42d33d` is a result about THIS revision,
without re-running any of it. Had the digest moved, the opposite would follow:
the RED-TEAM's verdict and the earlier rounds' gate work would be results about a
program that no longer exists, and D-603's *"the discharge moved nothing"* would
be false — which is why the experiment is run rather than argued.

---

## 5. Findings

### BLOCKING

None.

### MAJOR

None.

### MINOR

**F22 — D-603 calls `d42d33d` *"the landing revision"* and cites its receipt,
but `p1/impl`'s tip is `e32f8c4b` and the receipt at the tip exists; the ADR and
the arc ledger, edited five minutes apart, now disagree about which revision
lands.** This is round 2's F12 class recurring one revision later, and it is the
F14 remedy's own side effect.

*Minimal reproducer:*

```
$ git rev-parse p1/impl
e32f8c4bf51ec99ec3ffcb3f898910de809bc7cd
$ git show 189c6d6e…:docs/decisions.md | /usr/bin/grep -c \
    "At the landing revision \`d42d33d0c8ad640d7138a14caa5a1176d49fd2c0\`"
1
$ git show 189c6d6e…:docs/experiments/opt_arc_ledger.md | sed -n 45p | cut -c1-80
| IMPL | `p1/impl` = `e32f8c4bf51ec99ec3ffcb3f898910de809bc7cd` (round 2's F14; …
$ ls artifacts/p1_mutation_e32f8c4_v1.txt      # exists, sha256 c8f9e849…
$ stat -c '%y' docs/decisions.md docs/experiments/opt_arc_ledger.md
2026-09-04 11:36:00   # D-603 written
2026-09-04 11:41:35   # ledger updated with e32f8c4b
$ git log -1 --format=%ci e32f8c4b
2026-09-04 11:36:31   # the remedy commit, 31 s AFTER D-603 was written
```

The paragraph headed **THE VERIFICATION THAT IS TAKEN, EACH CLAIM AT THE REVISION
IT WAS TAKEN AT** is the one that names the wrong revision, which is this arc's
recurring shape — the defect inside the sentence promising the property (D-582,
D-600). Concretely wrong, three ways: `d42d33d` is not the landing revision;
`artifacts/p1_mutation_d42d33d_v1.txt` is not the receipt at the tip, and the
receipt it cites records M10 dying at `threat_oracle_tests.rs:546`, a line the
landing tree does not hold (§2); and *"the suite … and all six gates … re-run at
that revision by REVIEW-impl round 2"* leaves this round's own re-run at the
actual tip uncited.

**Nothing this licenses is a wrong conclusion about the CODE** — §4(b) proves the
binary is unchanged, and §2 and §3 re-take the receipt, the suite and the six
gates at `e32f8c4b` — but a reader who checks out the ADR's named revision gets a
tree whose test file differs from the receipt it is pointed at. *Discharge:* the
paragraph names `e32f8c4bf51ec99ec3ffcb3f898910de809bc7cd` and cites
`artifacts/p1_mutation_e32f8c4_v1.txt` `c8f9e849…`, with round 3 as the round
that took it; one sentence, no code.

**F23 — F13's replacement citation credits `solveron_run.log` with the node
identity, which its nine lines do not carry either.** The ratio is now cited
correctly; this is the same sentence over-reaching by one item.

*Minimal reproducer:*

```
$ wc -l artifacts/p1_rt_round2/solveron_run.log
9
$ /usr/bin/grep -ci node artifacts/p1_rt_round2/solveron_run.log
0
```

D-603: *"the run's own log `solveron_run.log` `61baa09e…` carries the seat, the
digests and **the node identity** but not the ratio"*. The nine lines are
`start … busy=[]`, three `sha256sum` lines (two binaries and the config), four
`<side> rep<n> exit 0 <time> errors 0` lines, and `end … busy=[]`. Node counts
appear nowhere in it; node identity is a property of the `.tot` files' FIRST
column, which is identical across all four (§1.2) — the same files the sentence
correctly credits with the ratio. *Discharge:* move *"the node identity"* into
the `.tot` clause, or drop it — the log carries the seat, the digests, the exit
codes and the idle receipts.

**F24 — the surviving half of I5's comment makes the same over-general claim the
deleted half made: *"then alternating, so every window is dead for both sides"*
is false, and the reference's own predicate is the reproducer.**

*Minimal reproducer,* against the reference the test compares to
(`crates/pistol-solver/tests/common/reference.rs:42-44`):

```rust
fn is_live(self, side: Player) -> bool {
    self.count(side) >= 1 && self.count(side.opponent()) == 0
}
```

A window is LIVE for a side when it holds at least one of that side's stones and
none of the opponent's — so a window holding exactly ONE stone is live, not dead.
Under the alternating colouring the fixture places six stones on ONE line of
`axis`; the other two axes' lines through each stone meet that line in exactly
one cell, so **each of the 12 other-axis windows through each of the 6 stones
holds exactly one stone and is LIVE for that stone's side** — 72 windows per
boundary per axis that are not dead for both sides. Two own-axis windows per boundary are in the same
position: of the eleven distinct own-axis windows through the six stones, the two
extreme ones reach only the outermost stone and hold nothing else. `assert_matches_board` (`threat_oracle_tests.rs:397-403`)
compares every one of them, so they are not out of scope of the sentence either.

The sentence is TRUE of the thing it is for — under the alternating colouring no
window lands in any maintained CLASS set, because `sets.rs`'s lowest maintained
class is `LiveTwo` and any window holding two of these stones holds two adjacent
positions and therefore both colours — and FALSE as written. This is round 1's F4
and round 2's F14 one clause to the right: the remedy deleted the sentence two
rounds read as a claim and left its neighbour making the same kind of claim.
*Discharge:* the clause says what it is for — *"then alternating, so no window
reaches a class set and every set is read empty"* — or the pair of colourings is
named without the justification, which the test's own assertions carry.

### About the round's own dispatch, not the implementation

**F25 — for the THIRD time in this package a governing document was rewritten
under a running review; unlike round 2, nothing closed underneath this one,
because the brief pinned every document and this round read only the pin.**
Recorded because F-P1.14 registered a practice against exactly this two hours
ago, and because a reader must be able to tell a held pin from a lucky one.

*Minimal reproducer:*

```
$ stat -c '%y %n' docs/experiments/p1_bench_prereg.md docs/experiments/opt_arc_ledger.md
2026-09-04 12:00:43  docs/experiments/p1_bench_prereg.md
2026-09-04 12:01:21  docs/experiments/opt_arc_ledger.md
$ sha256sum docs/experiments/p1_bench_prereg.md
7c72785a5604f9dfb1707c17c178a226181e3fb06c480aa86d9951b006fbf508   # pin: ebbfead1…
```

The prereg's §0, its replication paragraph, its disposition rule and its
harness-verdict paragraph were rewritten (its own round-3 gate's B4 and M7); the
ledger gained three rows, including its round-3 FAIL and a
*"**§P1 STOPS HERE, at the pre-registration's gate**"* row. **Why it voids
nothing here, checked rather than assumed:** this round read the pre-registration
for four things only, and all four are byte-identical in the rewritten copy —
§1's `tools/bench_delta.sh rev:ffc5c10 rev:<landed> 5`, §1.1's `[1.207, 1.307]`
and `[1.190, 1.290]`, §2.1's *"128 searches a side"*, and §0's `GATE_TOTAL=20`
with §3's *"twenty gates"*. Every quotation in §1.1's slot table therefore reads
the same at the pin and in the tree. The ledger is read here only for context and
nothing is adjudicated against it. **This is F-P1.14's practice breached and the
pin doing its job anyway**, which is the whole argument for pinning: round 2 had
two findings close under it and could reproduce neither closure; this round can
point at one revision for every word it judges. *Discharge:* none owed of the
implementation. The practice is the ledger's.

*One thing the rewrite leaves behind, outside this round's remit and noted so it
is not lost:* the ledger's new row calls the rewritten document **revision 4**
while its own title line still reads `(revision 3)` — the stale-revision class
D-599 through D-602 are about, in the document those lines were written beside.
It belongs to the pre-registration's own gate, not to REVIEW-impl.

### Observations, not findings

* **The headline's tense.** *"CHECKED SO AT THREE SEATS"* is past-participle
  about a leg whose `<RESULT>` slot the same D-line records as unfilled. I do not
  raise it, for three reasons that hold together: round 2's own discharge offered
  *"or drops `MEASURED`"* as sufficient and the word is dropped; the clause is
  parallel to *"THE SPEED CLAIMED AT THE INSTRUMENT SEAT"*, which is
  uncontroversially a scope statement, and D-427's slot convention is what makes
  the pair readable; and the proposition itself is very well supported — the
  RED-TEAM ran THIS binary against `ffc5c10`'s over 1 478 searching transcript
  pairs at six committed configs, two of them among the three registered seats,
  with 0 differing, and the prereg's §2.4 ran all three seats' 128 searches
  against the prototype. What has not been run is the registered leg at the
  landing, and the same sentence says so three clauses later, with
  *"A slot still unfilled at the commit is a breach of that prereg, not a licence
  to land."* A reader cannot take the headline as the leg's result without
  contradicting the body.
* **The comment is still a nine-line paragraph** (round 2's style note under
  F14). It lost three lines and the review-history clause; the paragraph shape
  did not change. Named, not rated — the brief's remedy is the deletion, which
  landed.
* **F19 stands, still pre-existing.** `warning: unused import: 'generate_turns'`
  at `crates/pistol-solver/src/policy.rs:1:44` in five of the six gate logs.
  `git diff --name-only ffc5c10 e32f8c4b -- crates/pistol-solver/src/policy.rs`
  is empty, so the package did not introduce it and does not touch it.
* **F16 does not recur.** This round was dispatched with all five documents
  pinned to `189c6d6e…`, and they still match the working tree byte for byte at
  the end of it (header). F-P1.14's practice held.
* **F18 stands unchanged** — the driver's digest
  (`7143a133ce972516954bff10345a9c2ec688fa67e91e8083420fd3f5073332b2`) is in no
  governing document, though the design's §4 asks for it sha-anchored beside its
  receipt; the ledger's run-10 row still names it by path only. Outside the
  remedies, so noted and not rated.

---

## 6. What this round did not do

The identity leg's governed run and the landing bench belong to
`p1_bench_prereg.md` and cannot be taken before `<landed>` exists; neither was
run here, and §4(b) is the reason the leg's answer cannot have moved since
`3ef6706`. The design's premises, the option matrix, the selection record and
`p1_bench_prereg.md` revision 3's own gate were not re-litigated — the prereg was
read only where D-603 cites it. Round 1's eight self-designed mutants were not
re-run: the binary is identical, so their result transfers. `pistol-core`'s
window enumeration and `sets.rs`'s class algebra were read only where F24's
arithmetic needed them. Round 2's F16, F18, F19, F20 and F21 sit outside this
round's remedies and are recorded above rather than adjudicated.

---

VERDICT: FAIL (F22, MINOR — D-603 calls `d42d33d0c8ad640d7138a14caa5a1176d49fd2c0`
*"the landing revision"* and cites `artifacts/p1_mutation_d42d33d_v1.txt`, but
`p1/impl`'s tip is `e32f8c4bf51ec99ec3ffcb3f898910de809bc7cd`, the arc ledger
edited five minutes later says so, `artifacts/p1_mutation_e32f8c4_v1.txt`
`c8f9e849…` exists, and the cited receipt records M10 dying at
`threat_oracle_tests.rs:546`, a line the landing tree does not hold — round 2's
F12 class recurring in the paragraph headed *"EACH CLAIM AT THE REVISION IT WAS
TAKEN AT"*, and the F14 remedy commit's own side effect, since D-603 was written
31 seconds before it. F23, MINOR — F13's replacement sentence fixes the ratio's
citation but credits `solveron_run.log` with *"the node identity"*, which its nine
lines do not carry (`grep -ci node` returns 0); node identity is the `.tot` files'
first column, the files the same sentence correctly credits with the ratio. F24,
MINOR — the boundary comment's surviving clause *"then alternating, so every
window is dead for both sides"* is false under the reference's own
`is_live = count(side) >= 1 && count(opponent) == 0`: each of the 12 other-axis
windows through each of the 6 stones holds exactly one stone and is LIVE for that
stone's side, and `assert_matches_board` compares every one of them — round 1's
F4 and round 2's F14 one clause to the right, where the remedy deleted the
sentence two rounds read as a claim and left its neighbour making the same kind.)

**NO BLOCKING AND NO MAJOR STANDS, AND ALL FIVE REMEDIES HOLD BY EXECUTION.**
F11's headline names three seats where it named one and the body says MEASURED of
nothing unfilled; F13's ratio, derived here independently as 1.0183 early and
1.0030 late from the `.tot` files with node identity holding across all four, is
honestly characterised and now cited to the files that hold it; F14's straddle
claim and its review-history clause are deleted and everything that replaced them
is true; F15's §6 bullet says CODE and the `sets.rs` diff is two `///` lines;
F17's *"five gates"* at `3ef6706` is what round 1's §8 cites and round 2's report
existed before D-603 quoted it. **AND THE REMEDIES MOVED NO BEHAVIOUR**: every
changed line from `3ef6706` to here is a comment; the mutation re-run is
byte-identical to the receipt at 15 of 15 dead with M14 dying by the refusal and
M12 by the mechanism the design names; 1 106 tests pass and 0 fail over 173
suites; all six gates the pre-registration names are green by their own log
lines; fmt, clippy and rule 9 are clean; and the release binary at `e32f8c4b`
hashes `1413698a22ff…` — the RED-TEAM's own subject — against `78a7600a…` at
`ffc5c10` built in the same worktree as a control, so the RED-TEAM's *no wrong
byte* over 1 167 813 state steps and 1 763 transcript pairs is a result about
THIS revision. **THE WHOLE FINDING SET IS CORRECTIONS OF RECORD** — a stale SHA,
a clause crediting a log with one item it does not hold, and a false sentence in
a test comment — none of which changes any reading of the code, which is D-598's
shape and not a defect of the implementation.
