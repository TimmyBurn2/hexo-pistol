# WP-1.6 pre-registration — REVISION 9 GOVERNING REVIEW

**Reviewed revision:** `6c929da0c3b52dd262f01254bc1039e04697cd11`
(`fix(tools,experiments,decisions): D-425 -- close the revision-8 review's seven findings`).

**Matches HEAD:** YES. `git rev-parse HEAD` -> `6c929da0c3b52dd262f01254bc1039e04697cd11`
at the start of this review and at the end.

**`git status --porcelain` at start:** empty (clean tree).

**`git status --porcelain` at end:** empty at the moment of the final check; this
report is then written to `docs/experiments/wp16_prereg_rev9_REVIEW.md` and left
**UNCOMMITTED**, so it shows as `?? docs/experiments/wp16_prereg_rev9_REVIEW.md`.
I wrote nothing else into the repository. Both scratch worktrees I created
(`/home/tom/pistol-testscratch-rev9mut`, `/home/tom/pistol-testscratch-rev9dig`) and
their separate `CARGO_TARGET_DIR`s were removed and `git worktree prune` run. All my
attack harnesses live in `/tmp/rev9atk/`, outside the repository. I explicitly checked
for the stray-`&-` class of harness bug the revision-8 reviewer recorded: none.

**Reviewer:** fresh context. I did not write any part of `6c929da`.
`docs/decisions.md` **D-401 was not read** — not grepped, catted or opened — per the
standing operator prohibition. D-424 and D-425 were read directly, via
`git show 6c929da -- docs/decisions.md` and targeted reads, never a full-file cat.

---

# VERDICT: **FAIL**

| Severity | Count |
|---|---|
| BLOCKING | **0** |
| MAJOR | **4** |
| MINOR | **3** |

**All seven of the revision-8 findings are genuinely closed.** I verified every one,
and the correctness fix (MAJOR 1) survived roughly forty attacks. **M12 KILLED**, and
D-425's unusual mutation story is honest and reproduced exactly.

**The pins: TWO of three held. Pin 2 is FALSE.** §7A.1 item 2 still pins the instrument
at `385631f`, and this very commit moved it. **The header's self-account HELD**,
verified by digesting all fifteen sections at both revisions.

The four MAJORs in one line each: **the instrument pin is false and the document
disagrees with itself about it in three places**; **a THIRD copy of the deleted
exit-2 per-kind reading is still in §7A.1**, in prose that uses none of the terms
D-425's "mechanical" grep searched for; **§7A.1's registered check still describes
only the `/dev/full` legs, which I MEASURED green at the revision where this round's
defect was live**; and **this commit silently added 138 unrelated agent-harness files**
that the revision-8 reviewer had already recorded as not theirs.

---

# 1. WHAT IS CLEAN, STATED FIRST AND SPECIFICALLY

## 1.1 MAJOR 1 (the correctness defect) is genuinely fixed, and it survived a long attack

All four modes the dispatch names, run against the **shipped** script at HEAD, are
**exit 2 with EMPTY stderr**:

```
===A1-closed-honest===       exit=2 stderr_bytes=0
===A2-closed-nonexistent===  exit=2 stderr_bytes=0
===A3-devfull===             exit=2 stderr_bytes=0
===A4-devfull-unbuffered===  exit=2 stderr_bytes=0
```

(`( exec 1>&-; exec python3 tools/wp16_warm_attribution_check.py <report> <replay>
target/release/pistol )`, and the `/dev/full` pair with and without
`PYTHONUNBUFFERED=1`.) The `>&-` cases were run with `exec 1>&-` inside a subshell
rather than through a heredoc, precisely to avoid the mis-escaped-redirection harness
bug the previous reviewer recorded against themselves.

The design is sound: `say()` takes `sys.stdout` into a local, raises `OSError` when it
is `None` instead of letting `print` silently succeed, writes and **flushes per line**,
and guards with `BaseException`; `leave()` does the same at the single exit and
downgrades to `NO_ANSWER` whenever `DELIVERED` is false; `_mute()` falls back to
`sys.stdout = None` when even `os.devnull` cannot be opened, which is what CPython
itself tolerates at shutdown.

**Exit 120 is now structurally unreachable** and I could not produce it: `say()` flushes
after every write and `leave()` forces a final flush, so no buffered bytes survive to
CPython's shutdown flush.

## 1.2 The mutation claim is HONEST and reproduces exactly — M12 KILLED

Re-run by me from scratch in a **separate worktree** at `6c929da`
(`/home/tom/pistol-testscratch-rev9mut`) with its own `CARGO_TARGET_DIR`, never the live
tree, under the login-shell toolchain (`/home/tom/.cargo/bin/rustc`, `rustc 1.97.1`).
Baseline first, so a kill cannot come from an already-red target:

```
===== BASELINE (unmutated, in the mutation worktree)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.38s
```

| Mut | What it reverts | My result |
|---|---|---|
| M10 | `say()` back to `print` (guard stays `BaseException`) | **SURVIVED** — `test result: ok. 15 passed; 0 failed` |
| M11 | `leave()` back to an OSError-only guard, no `None` check | **SURVIVED** — `test result: ok. 15 passed; 0 failed` |
| M12 | **BOTH** | **KILLED** — `test result: FAILED. 14 passed; 1 failed` |

M12's kill, with the panic site:

```
test an_answer_that_cannot_be_delivered_is_no_answer_and_not_a_finding ... FAILED
thread 'an_answer_that_cannot_be_delivered_is_no_answer_and_not_a_finding' (45132) panicked at crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs:1014:9:
test result: FAILED. 14 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s
```

And the raw command under each mutant, on a **pure void** (nonexistent report) with
stdout closed — the exact worst case D-425 names:

```
--- M10: exit=2 stderr_bytes=0
--- M11: exit=2 stderr_bytes=0
--- M12: exit=1 stderr_bytes=1462
    Traceback (most recent call last):
      File ".../tools/wp16_warm_attribution_check.py", line 974, in <module>
        leave(main())
```

Restored afterwards; `git status --porcelain` in the worktree printed nothing.

**Is "redundant by design" an honest account or a rationalisation? It is honest, and I
tested it rather than accepting it.** The two guards are genuinely independent
interceptions of the same condition — `say()` catches a `None` stream at write time,
`leave()` catches it at exit time — and I confirmed each alone converts the closed-stdout
case to exit 2. Reverting both reproduces the pre-fix signature byte for byte. A
rationalisation would have claimed M10 and M11 as kills; D-425 explicitly declines to,
and says so in the commit message and the ADR. **The property is covered; the individual
guards are not individually necessary, and D-425 says exactly that.** The one honest
caveat, which D-425 also states: because either guard alone suffices, CI would stay green
if a future session deleted one. That is the price of defence in depth and is recorded,
not hidden.

## 1.3 Revision-8 MAJOR 2 is closed at every site the dispatch named

```
$ grep -c "^| Exit |" docs/experiments/wp16_sprt_prereg.md
1
```

One exit table only. The seven strings the dispatch told me to grep for yield exactly
**one** hit in the whole document:

```
$ grep -n "CLOSED list\|kind (i)\|kind (ii)\|registered here\|SCOPED to revision\|~50\|no process can catch" docs/experiments/wp16_sprt_prereg.md
102:  CLOSED list it genuinely can be, kind (ii) becomes a REGISTERED CATCH-ALL
```

Line 102 is inside the header's **historical** narrative of revision 6 ("Revision 6 does
not answer that with a longer list … Kind (i) becomes the CLOSED list it genuinely can
be"), past-tense and unambiguous. That is legitimate history. The second table at
old lines 845–851 and the "A NOTE ON DIRECTION" paragraph are both gone.

**But see MAJOR 2 below**: a third copy of the same registered per-kind reading survives
in different words, which is why I do not call this finding fully closed.

## 1.4 Revision-8 MAJOR 3 is closed and consistent

§11's closing paragraph now reads:

```
1378:A fresh review — REOPENED WHOLE, because `385631f` moves the instrument and a
      review of a superseded revision does not transfer — must pass before the
      governed run this document describes may be launched. The cap that governed
      revision 6's review licensed one fix round and one scoped re-review, and
1382: D-423 records BOTH as spent; no scoping survives it.
```

`grep -n "SCOPED to revision"` returns nothing. This agrees with the header (line 5,
"this review reopens WHOLE") and with §11's own row 8 ("**NOT document-only**; this
review reopens WHOLE"). Consistent in all three places. Clean. (The `385631f`
attribution inside it is stale — that is MAJOR 1, not this finding.)

## 1.5 Revision-8 MINOR 1 — the count is deleted, not corrected, and no count remains

§7A.1's exit-2 row now reads "This document does not count, enumerate or partition them
— a count goes stale on every instrument edit and did so twice (D-421 MINOR 3, D-425
MINOR 1)". The only surviving numeral is at line 98, inside the header's account of
**revision 5** ("the instrument had 49 `die()` sites and §7A.1 quoted 7") — past tense,
about a superseded revision, correct as history. For the record the live figure is
unchanged from the revision-8 review:

```
$ grep -oE '\bdie\(' tools/wp16_warm_attribution_check.py | wc -l
57          # 1 `def die(` + 56 call sites
```

Nothing in the document asserts it. Correctly closed.

## 1.6 Revision-8 MINOR 3 — the property is named, and the SIGTERM=143 claim is TRUE

§5's row now reads "any death this process did not choose — `SIGKILL` and the OOM killer,
which cannot be caught, and also `SIGTERM` and its siblings, which CAN be caught and are
not, so they land here at the shell's `128+n` (MEASURED: `SIGTERM` gives 143)". I
measured all three against a genuinely long-lived run (the engine argument pointed at a
`sleep 30` shim so the process was reliably alive; I asserted `kill -0` before signalling
rather than assuming, because my first attempt raced and the process had already exited):

```
===D7 SIGTERM ...  alive, sending SIGTERM   [D7] exit=143 stderr_bytes=0
===D8 SIGKILL ...  alive, sending SIGKILL   [D8] exit=137 stderr_bytes=0
===D9 SIGHUP  ...  alive, sending SIGHUP    [D9] exit=129 stderr_bytes=0
```

143 = 128+15 confirmed. "and its siblings" confirmed by SIGHUP at 129. The claim is true.

## 1.7 Revision-8 MINOR 4 — the control assertion now names the other codes

`crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs:948`:

```rust
"the control must exit 0. Exit 1 would mean the fixture itself has an \
 attribution failure and the cases below would be measuring that instead; \
 exit 2 that it is unreadable; exit 3 a determinism violation. Any of the \
 three makes everything after this line vacuous: {}",
```

That discharges SHELL_CHECKLIST item 12 obligation 3 for the control. Closed.

## 1.8 The header's self-account HELD — verified by section digest, not hunk arithmetic

The header claims: "It amends THE HEADER (this block), §5, §7A.1, §10 and §11 — and
nothing else." I digested **every section at both revisions** and compared bytes
(instrument: `/tmp/rev9atk/sections.py`, splitting on `^## ` and `^### 7A.\d`):

```
15 sections in A, 15 in B
  CHANGED         HEADER   ab1f0547907f -> 39c1a4850d32
  byte-identical  ## 1. What is being judged, and what is not
  byte-identical  ## 2. The hypothesis and the verdict unit
  byte-identical  ## 3. The instrument
  byte-identical  ## 4. What the run reports, and which lines are read
  CHANGED         ## 5. Outcome handling, written before game one   3e90106c0299 -> cc9b1d2a23f4
  byte-identical  ## 6. The honest expectation, and what a negative result means
  byte-identical  ## 7. Costs
  byte-identical  ## 7A. The doubts, their instruments, their agreement criteria and the
  CHANGED         ### 7A.1 DOUBT 1 — the arena between the engines and the verdict   6ad7216f15b3 -> ad6fe4e72832
  byte-identical  ### 7A.2 DOUBT 2 — whether the extension changes what the search compl
  byte-identical  ## 8. The dry run
  byte-identical  ## 9. FILL-IN slots
  CHANGED         ## 10. What flips this document   db92c8e57c83 -> 3f5ab4b5358d
  CHANGED         ## 11. REVIEW STATE   37a01a34ebb0 -> ac31de8f51b7

CHANGED SECTIONS: 5
```

**Exactly the five claimed regions, and nothing else.** The self-account defect
(D-416 MINOR 4, D-419 MINOR G) did not recur a fifth time. This is clean.

Note carefully: the self-account is about WHICH SECTIONS CHANGED, and it is true. A
**different** header sentence — "It MOVES THE INSTRUMENT: `385631f` re-pins §7A.1's
statistics layer" — is false at this revision, and that is MAJOR 1.

## 1.9 Pins 1 and 3 hold, and the binary digests genuinely reproduce

```
$ git diff --stat bfdf933..HEAD -- crates/pistol-arena/
[prints nothing]

$ git diff --stat bfdf933..HEAD -- crates/*/src/ Cargo.toml Cargo.lock
[prints nothing]

$ sha256sum target/release/pistol target/release/arena
b8d0dc963a2453e1eff69823629c37b23bafe419b9225f8af2401df519bc2673  target/release/pistol
3ba8de615d4d708793d72c2f3c2f6c649811996bb331527e64d0f612a13aebc2  target/release/arena
```

Both equal the registered digests. **I did not take the live binaries on trust.** In a
separate worktree at `bfdf933` with a separate `CARGO_TARGET_DIR`, under the login shell:

```
$ which rustc
/home/tom/.cargo/bin/rustc
$ cargo build --release --locked
    Finished `release` profile [optimized] target(s) in 5.93s
$ sha256sum .../release/pistol .../release/arena
b8d0dc963a2453e1eff69823629c37b23bafe419b9225f8af2401df519bc2673  .../release/pistol
3ba8de615d4d708793d72c2f3c2f6c649811996bb331527e64d0f612a13aebc2  .../release/arena
```

Byte-exact from a tree sharing nothing with the live `target/` but the source. Pin 3 is
UNCHANGED and genuinely reproducible. The worktree and its target directory were removed.

## 1.10 Every regression the dispatch named is clean

**Honest control — exit 0:**

```
EXIT=0  stderr=0
warm_attribution_check: W coverage: 8 game(s) accounted for — 8 replayed in full with every node count equal to the run's, 0 halted at a divergence. …
warm_attribution_check: W classification: 0 divergence(s), 0 confirmed inversion(s), 0 unexplained
warm_attribution_check: (b): 0 inert pair(s) excluded by theorem, 4 pair(s) directly attributed at their first differing searched turn, 0 unattributable
warm_attribution_check: cross-check: no inert pairs — the exclusion changed nothing
warm_attribution_check: 1b: 5 decided non-forfeit game(s) adjudicated against the move list
warm_attribution_check: 1c: 8 game(s) and 4 pair(s) rebuilt off the score_a path
warm_attribution_check: PASS — 0 failure(s)
```

**Seeded swap — exit 1 with its inversions:**

```
EXIT=1  stderr=0
warm_attribution_check: W classification: 8 divergence(s), 8 confirmed inversion(s), 0 unexplained
warm_attribution_check: FAIL (a) game 0 turn 5: the report credits `-4,0/-1,0` to `staged`, which answered `-4,0/-3,0` warm, and the other seat (`staged_q_both`, …) answers `-4,0/-1,0` — the seats are the wrong way round
warm_attribution_check: FAIL 1c `counts wins_a 2` against 3 rebuilt from the `game` lines
warm_attribution_check: FAIL 1c `counts losses_a 3` against 2 rebuilt from the `game` lines
warm_attribution_check: FAIL 1c `pair 0 bucket p1 score_a 0.250000000` against p3 / 0.750000000 rebuilt from the `game` lines
```

**The D-422 regular-file guard, on BOTH path arguments, with `timeout 10` bounding
every probe so a reinstated hang would show as 124:**

| Probe | Result |
|---|---|
| FIFO as arg1 | exit **2**, `the report … is not a regular file`, no hang |
| FIFO as arg2 | exit **2**, `the replay document … is not a regular file`, no hang |
| directory as arg1 / arg2 | exit **2**, named, both |
| device (`/dev/zero`) as arg1 / arg2 | exit **2**, named, both |
| nonexistent | exit **2**, `does not exist` |
| **plain copy** | **exit 0, accepted** |
| **symlink -> regular file** | **exit 0, accepted** |
| **`/dev/stdin` redirected from a regular file** | **exit 0, accepted** |

Both directions correct, both arguments guarded.

**The registered gates:**

```
$ cargo test --workspace --locked
[139 targets aggregated] passed=744 failed=0 ignored=9
$ grep -c "^test result: FAILED" <log>
0
```

The instrument's own target:

```
     Running tests/wp16_warm_attribution_check_tests.rs
test an_answer_that_cannot_be_delivered_is_no_answer_and_not_a_finding ... ok
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s
```

```
$ cargo clippy --workspace --all-targets --locked -- -D clippy::all
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
CLIPPY-EXIT=0

$ cargo fmt --all --check
FMT-CLEAN

$ bash tools/artifact_check.sh
artifact_check: ok (567 tracked files, none of them artifacts)      ARTIFACT-EXIT=0

$ bash tools/file_justification_check.sh
file_justification_check: 284 tracked .rs/.sh files, 39 over the cap, all justified
FILEJUST-EXIT=0

$ bash tools/config_check.sh
validate_arena_config: 10 document(s) ok
validate_random_openings_config: 1 document(s) ok                   CONFIG-EXIT=0
```

The commit's own claim of "744 tests pass, 0 failed; clippy and fmt clean" reproduces
exactly.

---

# 2. FINDINGS

## MAJOR 1 — §7A.1 item 2's instrument pin is FALSE, and the document contradicts itself about it in three places

**Where.** `docs/experiments/wp16_sprt_prereg.md` §7A.1 lines 585–590:

> 2. **The statistics layer** — `tools/wp16_warm_attribution_check.py`, **at
>    commit `385631f`, RE-RECORDED IN REVISION 8.** … and `385631f`
>    now: the delivery funnel that closes D-421 MAJOR 1 and the regular-file guard
>    that closes D-422. Each re-record IS the amendment that reopens this
>    document's review.

**It is false.** This commit moved the instrument, in exactly the way item 2's own last
sentence says is an amendment:

```
$ git diff --stat 385631f..HEAD -- tools/wp16_warm_attribution_check.py
 tools/wp16_warm_attribution_check.py | 36 +++++++++++++++++++++++++++++-------
 1 file changed, 29 insertions(+), 7 deletions(-)
```

CLAUDE.md is explicit: "THE INSTRUMENT HAS A GOVERNING REVISION TOO. An artefact that
produces a registered number … is named in the pre-registration WITH ITS REVISION, and a
change to it reopens the review exactly as an amendment to the document does." The
document names the wrong revision. A reviewer dispatched against §7A.1 item 2 would
review `385631f`'s instrument — the one that exits **1** on a pure void.

**And the document disagrees with itself about it, in three further places:**

1. **§10's row contradicts itself inside a single cell** (line 1331):

```
| `tools/wp16_warm_attribution_check.py` | **the D-425 fix round** — `bfdf933` through revision 5, `43e8a86` in revisions 6-7, `385631f` now | §7A.1 |
```

The cell's headline says "the D-425 fix round"; the same cell's tail says "`385631f`
now". Both cannot be true, and §10's "Where" column points at §7A.1, which says
`385631f`.

2. **The header** (line 4): "It MOVES THE INSTRUMENT: `385631f` re-pins §7A.1's
   statistics layer, so this review reopens WHOLE." At revision 9 the instrument is
   moved by *this* commit, not by `385631f`. The conclusion (reopens whole) is right;
   the stated reason is a revision out of date.

3. **§11's closing paragraph** (line 1378): "REOPENED WHOLE, because `385631f` moves the
   instrument". Same staleness.

**Why this is MAJOR and not MINOR.** This is not narrative prose — item 2 is one of the
three PINS, the checkable claims §10 exists to point at, and it is the pin that governs
the artefact this entire fix round changed. It is false by a one-command test. It is also
the same one-of-two-sites mechanism as D-423 MAJOR 1 and revision-8 MAJOR 2, inverted:
this time §10 was brought forward and §7A.1 was left behind. D-425's own headline lesson
is "before a revision lands, the terms it claims to have removed are grepped for" — but
nothing grepped for the term it needed to **add**.

**A note on the chicken-and-egg, so the fix is not over-scoped.** A document cannot cite
the SHA of the commit that contains it. "The D-425 fix round" is a legitimate way to
denote it, and §10 already uses it. The finding is that §7A.1 — the section that owns the
pin — was not given the same treatment, and still asserts a superseded SHA as current.

## MAJOR 2 — a THIRD copy of the deleted exit-2 per-kind reading is still in §7A.1, in prose that D-425's grep could not have found

**Where.** `docs/experiments/wp16_sprt_prereg.md` lines **690–697**, inside §7A.1
(§7A.1 spans 490–903):

> **THE REGISTERED READING OF THESE FOUR, corrected in revision 5.** Revision 4
> registered exit 2 as "the void is fixed and the answer re-taken". **That is the
> wrong reading for a premise refusal, and it is now stated correctly**: a pair
> whose two games declare different openings, or do not swap seats, or diverge
> inside the book, or stop inside it, is a fact about the REPORT. There is no
> void to fix and nothing to re-take. **The report is REFUSED and it is not a
> measurement**, and the thing to investigate is how a report the arena cannot
> write came to exist — never the engines.

**This is the deleted table's kind (ii), almost word for word.** The row D-424 deleted
read: "**Nothing here is 'fixed' and nothing is 're-taken'** — the report is not one the
arena could have written, the run is not a measurement, and what is investigated is the
report's provenance, never the engines". Compare. It is the same claim, in prose, in the
same section, one hundred lines above the rule that forbids it.

**It is operative, not history.** The heading is "**THE REGISTERED READING**"; the verb
is "**it is now stated correctly**"; the "corrected in revision 5" is a provenance tag on
a claim asserted in the present. Contrast line 102, which is genuinely past-tense
narrative and which I passed as clean in §1.3.

**What it falsifies. Three live claims in this same revision:**

1. **§5's single exit-2 row** (line 422) says what to do next "is read off the
   instrument's own message … **That is operational and this document does not register
   it.**" Line 690 registers it, for four named refusals, and forbids one of the two
   options §5 offers: §5 says "re-run, **or** go and find out how a report the arena
   could not have written came to exist"; line 694 says "There is **no** void to fix and
   **nothing** to re-take". A reader with a premise refusal has two registered,
   incompatible instructions and no stated precedence.
2. **§7A.1's own rule** (line 789): "**THE REGISTERED CONSEQUENCE OF EACH EXIT CODE LIVES
   IN §5 AND ONLY IN §5.** … **It does not restate §5's table**, and that is a rule rather
   than a stylistic preference". It restates it.
3. **D-424 and CLAUDE.md's "state it once" rule**, for the third consecutive revision.

**The mechanism is the finding, and it is worse than last round's.** D-425's headline
remedy is: "**THE LESSON IS MECHANICAL AND IS NOW ACTED ON**: before a revision lands, the
terms it claims to have removed are grepped for. Reasoning about whether the second copy
exists has now failed twice; grepping takes a second." That grep is **an enumeration of
terms** — `CLOSED list`, `kind (i)`, `kind (ii)`, `REGISTERED CATCH-ALL` — and this copy
contains none of them. **A guard that enumerates the ways a thing can appear rather than
treating the claim itself as the thing to find** is precisely the defect class D-425's own
MAJOR 1 is about, one level up, and it shipped in the commit that declared the class ended.

**Minimal reproducer.** The dispatch's seven strings find nothing here; a search for the
*claim* rather than its *vocabulary* finds it at once:

```
$ grep -n "no void to fix\|is REFUSED\|never the engines" docs/experiments/wp16_sprt_prereg.md
694:void to fix and nothing to re-take. **The report is REFUSED and it is not a
696:write came to exist — never the engines. A reader who followed revision 4's

$ grep -nE "^## |^### 7A\.[0-9]" docs/experiments/wp16_sprt_prereg.md | awk -F: '$1<690' | tail -1
490:### 7A.1 DOUBT 1 — the arena between the engines and the verdict
```

I then swept the whole document for every other operative restatement of an exit-2
consequence (`grep -n "never the engines\|not a measurement\|no void to fix\|is
REFUSED\|provenance"`). The other hits are lines 32 (header history), 147, 418, 421, 422,
425 (§5's own table, where they belong), 1152, 1269 and 1300 (§8's "what this dry run is
not"). **Lines 690–697 are the only surviving copy.** I am reporting a complete sweep, not
a single hit.

## MAJOR 3 — §7A.1's registered check still describes only the `/dev/full` legs, which I MEASURED green at the revision where this round's defect was live

**Where.** §7A.1 lines 830–838. The registered check:

> `an_answer_that_cannot_be_delivered_is_no_answer_and_not_a_finding`
> (`crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs`), which runs
> the SHIPPED script with stdout on `/dev/full`, buffered and unbuffered, and
> requires exit **2** with no `Traceback` on stderr, against a control on the
> same fixture that must exit 0.

and its receipt:

> **THIS CHECK IS ONE THE DEFECT FALSIFIES, AND THAT IS SHOWN BY MUTATION RATHER
> THAN CLAIMED**: removing `say()`'s guard (M6), making `leave()` ignore the
> delivery flag (M7), or dropping its forced flush (M8) each makes that test FAIL.

**The code added the closed-stdout legs; §7A.1 was not told.** The shipped test at HEAD
has two further legs (`>&-` on an honest report and on a nonexistent one). The document's
description of its own registered check omits them — and those omitted legs are the only
ones that can falsify the D-425 defect.

**MEASURED, which is the whole point.** I checked out the instrument at `385631f` — the
revision §7A.1 item 2 *still pins* — and ran the described legs and the undescribed one:

```
=== the two legs 7A.1 DESCRIBES, run against the 385631f instrument ===
  /dev/full buffered   : exit=2 stderr=0
  /dev/full unbuffered : exit=2 stderr=0
=== the leg 7A.1 does NOT describe, same instrument ===
  stdout closed, pure VOID: exit=1 stderr=1462
```

**The registered check, as the document describes it, is GREEN at the revision where the
defect was live and a pure void exited 1.** That is D-423 MAJOR 2's finding verbatim
— "the check is green at the revision where the defect was live … it is not a
FALSIFIER" — recurring against the replacement that was written to end it.

**And the mutation receipt beside it is now materially misleading.** It names M6, M7 and
M8 as the evidence that the check is falsifiable. Those three mutate `385631f`'s guards.
The guards *this* revision added are each individually survivable — I measured M10 and M11
**SURVIVING** (§1.2) — so a reader taking the receipt at face value would believe the new
guards are covered by killing mutations when the only killing mutation is M12, which the
document does not mention at all. D-425 records M10/M11/M12 honestly **in the ADR and the
commit message**; §7A.1, the section that registers the check, carries none of it.

**Severity.** MAJOR. §7A.1 is the section that registers what the check is; its
description is incomplete about the leg that closes this round's correctness defect, its
receipt cites mutations that do not exercise this round's guards, and the shortfall is
measurable in one command. Per CLAUDE.md's own test, this changes what a reviewer may
conclude: it licenses "the registered check falsifies this defect class" when the
described check demonstrably does not.

## MAJOR 4 — this commit silently added 138 unrelated agent-harness files, including three shell scripts, in a commit whose message accounts for none of them

**Measured:**

```
$ git ls-tree -r --name-only 3696dea | grep -c "^\.agents/\|^\.claude/\|^skills-lock"
0
$ git ls-tree -r --name-only 6c929da | grep -c "^\.agents/\|^\.claude/\|^skills-lock"
138
```

`git show --stat 6c929da` is 143 files, `5632 insertions(+)`. Five of those files are the
work under review (`tools/wp16_warm_attribution_check.py`, the test target,
`docs/decisions.md`, `docs/experiments/wp16_sprt_prereg.md`, and the revision-8 review
report). **The other 138 are third-party agent-harness content with no relation to
pistol** — `.agents/skills/ask-matt/`, `code-review/`, `grill-me/`, `wizard/`,
`writing-beats/`, a `.claude/skills/` symlink farm, and `skills-lock.json`.

**These are the exact files the revision-8 reviewer recorded and declined to touch**, in
that report's own header: "They are not mine and not this review's… I did not remove the
three harness entries, since deleting another process's files is not a reviewer's move."
Revision 9's session did not remove them either — it **committed** them, without a word in
the commit message.

**Three of them are shell scripts** now tracked in a repository whose shell discipline is
a named CI concern:

```
.agents/skills/diagnosing-bugs/scripts/hitl-loop.template.sh
.agents/skills/git-guardrails-claude-code/scripts/block-dangerous-git.sh
.agents/skills/wizard/template.sh
```

They are outside `tools/`, so `tools/SHELL_CHECKLIST.md` does not formally reach them, and
none was reviewed. `tools/file_justification_check.sh` now counts them ("284 tracked
.rs/.sh files") — its denominator moved because of this.

**Why MAJOR.** CLAUDE.md's Workflow says "One feature = one commit,
`type(scope): what changed and why it matters`". CLAUDE.md rule 10 names silent
architecture drift as a breach and requires an ADR line for every non-obvious design
choice; adding 138 files and a lockfile to the repository is a choice, and neither D-425
nor the commit message mentions it. It also degrades the very artefact this review
judges: the diff a reviewer must read to see what revision 9 did is 96% noise. I could not
confirm this was intentional, and I have not removed anything — see §5.

**This is not a correctness finding about the instrument**, and it is not overrulable on
that ground either way; it is a scope-and-record finding, and the fix is a decision, not a
patch.

## MINOR 1 — the digest row now agrees with §7A.1 and both are stale by one revision

§10 (line 1332) reads "unchanged by revisions 6, 7 and 8"; §7A.1 item 3 (line 598) reads
"**UNCHANGED THROUGH REVISIONS 6, 7 AND 8**". **They agree, which is what revision-8's
MINOR 2 asked for, and I record that as closed.** But revision 9 also leaves the digests
unchanged — I reproduced them byte-exactly (§1.9) — and neither site says so. This is an
enumeration of revisions that must be extended by hand at every revision, i.e. the same
defect class D-425 closed by *deleting* the `die()` count and by naming the *property* in
§5's signal row. The same treatment ("unchanged by every revision of this document; the
checkable form is the `git diff --stat` above") would end it. MINOR because the checkable
command beside it is true and does the work.

## MINOR 2 — §5's signal row says it does not enumerate, immediately after enumerating three instances

Line 424: "What remains is any death this process did not choose — `SIGKILL` and the OOM
killer, which cannot be caught, and also `SIGTERM` and its siblings … **This row
deliberately does NOT enumerate them**".

The row names SIGKILL, the OOM killer and SIGTERM, then says it does not enumerate. The
**property** is stated first and is correct and load-bearing, which is why this is MINOR
and why I count revision-8's MINOR 3 as closed. But the closing sentence is false as
written about its own preceding clause, in the row rewritten to retire exactly this defect
class. "Those are examples, not the rule; the rule is the property above" would be true.

## MINOR 3 — §7A.1 item 3's supporting derivation does not cover this commit

Item 3 (lines 598–605) justifies the digests being unchanged: "`43e8a86` and `385631f`
touch only `tools/wp16_warm_attribution_check.py` and
`crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs` — a `tools/` script and a
`tests/` target, neither of which a release binary is built from". `6c929da` touches those
two **and** `docs/`, **and** 138 files under `.agents/`/`.claude/`/`skills-lock.json`. The
derivation names two commits and the set is now three.

**The conclusion still holds and I measured it** — `git diff --stat bfdf933..HEAD --
crates/*/src/ Cargo.toml Cargo.lock` prints nothing, and the digests reproduce — so
nothing is misread. MINOR, and it is the same enumeration-instead-of-property shape as
MINOR 1.

---

# 3. SHELL_CHECKLIST ITEMS, ANSWERED BY NAME

`6c929da` changes `tools/wp16_warm_attribution_check.py`, so the checklist applies. It is
Python, not shell under `set -euo pipefail`, so several items name a hazard class this
diff cannot contain; those are marked N/A **with the reason and with the analogue
checked**, never waved.

**Item 1 — a command substitution whose status is DISCARDED.** N/A in form. The analogue
is unchanged by this diff and re-confirmed: `cold_answer()` takes `subprocess.run(...)`
into a variable and validates the SHAPE of the result (`len(best) != 1`) before using it,
refusing by name and quoting both streams. Nothing in the diff interpolates an unchecked
value into a record. Clean.

**Item 2 — a pipeline in a `then` body is not a pipeline in a condition.** N/A: no shell
pipelines in the diff. **Relevant to my own harness**, and I got it wrong once: my first
attack script read `${PIPESTATUS[0]}` after a `( … | … )` compound, so it reported the
subshell's status, not the instrument's. Corrected and re-run (C1/C2 in §4).

**Item 3 — `grep` under `pipefail`; a substring is not a token.** N/A for `grep`. The
"substring is not a token" half applies to the instrument's regexes and is unchanged and
still discharged: every pattern in `read_report`/`read_replay` is `re.M`-anchored `^…$`,
and `only()` refuses both "no match" and ">1 match" by name. **It also applies to this
review**, and it is why MAJOR 2 exists: D-425's remedial grep matched *substrings of a
vocabulary* rather than the claim, and the claim survived in other words. Item 3's own
lesson, realised against a document instead of a script.

**Item 4 — `LC_ALL`, and which direction it moves a guard.** N/A for locale pinning; the
diff adds none. The item's real subject — a guard whose width depends on a definition — is
unchanged and correct: `read_report` uses `split("\n")` and not `splitlines()`, with the
reason in a comment (two notions of "line" over one document is how a verbatim engine
refusal injects a record). **I did probe the locale direction on the new code**: with
`PYTHONIOENCODING=ascii` the em-dash in the instrument's own output fails to encode, which
is a `UnicodeEncodeError`, a `ValueError`, and the new `BaseException` guard catches it —
exit 2, zero bytes delivered, no partial line (B10/B11 in §4). The new guard makes the
refusal WIDER, which is the direction this item asks for.

**Item 5 — the index is what commits; the working tree is not.** N/A to the instrument,
which reads caller-named artefact paths and makes no claim about tracked bytes. **It is
the item that produced MAJOR 4**: I did not ask what the working tree looked like, I asked
what the *index* now contains (`git ls-tree -r --name-only`), and that is what showed 138
files crossing from untracked to tracked.

**Item 6 — a sweep by prefix must own the prefix.** N/A to the instrument (it deletes
nothing). Applies to the test file, which the diff changes, and is discharged: the new
legs reuse the existing `report_path`/`replay_path` fixtures inside the test's own
`scratch("wp16warm-undelivered")` directory under `SCRATCH_PREFIX = pistol-testscratch-`.
No new sweep. **I adopted that prefix for both of my own worktrees** and removed them.

**Item 7 — traps.** N/A: no `trap`, no EXIT handler. The analogue — "the last thing that
runs decides the status" — is `leave()`, and this is where revision 8's MAJOR 1 lived.
**It is now discharged**: `leave()` guards with `BaseException`, cannot raise anything but
`SystemExit`, and `_mute()`'s fallback (`sys.stdout = None`) is the one value CPython's
own shutdown flush tolerates. I attacked this specifically and could not make the last
statement choose the status (§4). Item 7 moves from BREACHED to discharged.

**Item 8 — one spelling per number, one refusal per reason.** Discharged and unchanged.
`fields()` refuses a repeated key by name rather than last-wins; `slurp()` gives three
distinct refusals (does-not-exist, is-not-a-regular-file, could-not-be-read) where one
behaviour used to stand, verified in §1.10 on both path arguments. **The diff adds a
fourth spelling of one reason and does it right**: `say()` and `leave()` both raise
`OSError("stdout is not open")` for the `None` stream, i.e. one reason gets one refusal
even though two sites detect it.

**Item 9 — what reaches a record is caller-controlled.** Discharged. The diff adds no new
interpolation of caller data; the strings it adds are constants. Refusals still quote the
offending value in backticks, go to stdout for a human, and exit 2 stops anything
downstream reading them as data.

**Item 10 — THE COVERAGE RULE (the binding one).** **DISCHARGED, and it is the strongest
part of the diff.** The script produces a recorded number (the Criterion 1'' verdict) and
carries 15 tests in a suite CI runs, all driving the **SHIPPED** script — `check()` invokes
`python3` on `repo("tools/wp16_warm_attribution_check.py")` resolved from
`CARGO_MANIFEST_DIR`, which is why my mutations in a separate worktree were what that
worktree's tests ran. The new legs have a **control** (the same fixture must exit 0 first),
so a pass cannot come from a checker that refuses everything, and I proved the legs are
load-bearing by mutation: M12 makes them FAIL. `test result: ok. 15 passed; 0 failed`.

**Item 11 — a caller's path that feeds a delete or an overwrite is containment-guarded.**
N/A, and I enumerated rather than assumed, because the item demands "the sweep, not the
instance": the diff contains no `rm`, `mv`, `shutil`, `os.remove`, `os.rename`, and no
write to a caller-named path. The instrument opens its three arguments read-only
(`open(path, "rb")`) and writes only to stdout. `_mute()` opens `os.devnull` — a fixed
script-chosen constant — and the diff's change to it (falling back to `sys.stdout = None`)
introduces no path at all. **No destructive site exists; the enumeration is the evidence.**

**Item 12 — a gate distinguishes RUN VOID from FAIL, by name.** This is the item the whole
round is about.
- *Obligation 1, a code per kind*: **satisfied, and this is the fix**. Revision 8 spelled a
  void (`/nonexistent`) as `1`, the FAIL code — item 12's opening sentence exactly. At
  HEAD that case is exit **2** (A2, §1.1), and M12 proves the tests hold it there.
- *Obligation 2, preflight and void early*: satisfied and unchanged. `slurp()` preflights
  the path kind before opening (D-422), which is what turns a hang into a named early void.
- *Obligation 3, the distinction survives the seam*: **satisfied, including the control**.
  Revision 8's `assert_eq!(code, Some(0))` with an uninformative message was MINOR 4; the
  new message names what 1, 2 and 3 would each have meant (§1.7). The new legs' main
  assertion names what exit 1 would have meant ("a void wearing the exit code this document
  registers as an attribution finding"). Fully discharged.

**The one checklist-adjacent breach is MAJOR 4**, and it belongs to item 5's and item 6's
shared subject — owning what you commit. Three shell scripts entered the repository in this
commit without passing any of the twelve items, because they are not under `tools/`.

---

# 4. ATTACKS ATTEMPTED AND REJECTED

The dispatch told me two previous reviewers each broke the delivery invariant and that a
third break was plausible. **I could not break it.** Everything below is a real run.
Rejected means I tried and the instrument held.

**Rejected — stdout closed AND stderr closed** (`exec 1>&-; exec 2>&-`). `exit=2`. The
handler survives having no channel at all.

**Rejected — stdout closed, stderr on `/dev/full`.** `exit=2`.

**Rejected — stdout closed, stderr closed, `PYTHONUNBUFFERED=1`, on a pure void.**
`exit=2`. The combination that produced revision 8's traceback is fully closed.

**Rejected — both stdout and stderr on `/dev/full`.** `exit=2`.

**Rejected — stdout closed on the SWAPPED (genuine finding) input.** `exit=2,
stderr_bytes=0`. A finding nobody can read is correctly downgraded, not delivered as 1.

**Rejected — stdout on a READ-ONLY descriptor** (`1</dev/null`). `exit=2,
stderr_bytes=0`. `EBADF` is an `OSError` and is caught.

**Rejected — `PYTHONFAULTHANDLER=1` with stdout closed.** `exit=2, stderr_bytes=0`. The
fault handler does not fire and does not alter the code.

**Rejected — SIGPIPE, three ways.** This one is a genuine behaviour CHANGE and I chased
it. Because `say()` now flushes after **every** line, EPIPE is far more reachable than at
revision 8, where the revision-8 reviewer measured `| head -1` giving pipestatus 0. At
HEAD:

```
===C1 SIGPIPE via head -1, swapped input   [C1] instrument_status=2  stderr=0
===C2 SIGPIPE, reader exits immediately    [C2] instrument_status=2  stderr=0
```

(My first attempt read `PIPESTATUS` outside the pipeline and reported 0 — a harness bug,
SHELL_CHECKLIST item 2, corrected above.) So piping the checker's stdout into a
short-lived reader now yields exit 2 where it used to yield 0. **This is correct** — the
answer was not delivered — and `BrokenPipeError` is an `OSError`, caught by design. I then
checked whether it can reach the registered workload: it cannot. No registered invocation
pipes the checker's stdout:

```
$ grep -n "python3 tools/wp16_warm" -A2 docs/experiments/wp16_sprt_prereg.md | grep "|"
[prints nothing]
```

§7A.1's and §8's invocations (lines 1180, 1186) redirect nowhere. **Not a finding**, but
recorded so the change is visible rather than discovered later.

**Rejected — a hostile `sys.stdout` whose `write` and `flush` raise `KeyboardInterrupt`**
(injected via `sitecustomize.py` on `PYTHONPATH`). `exit=2, stderr_bytes=0`. The
`BaseException` guard catches a non-`Exception` failure at the write site, which is
exactly what the narrower form could not.

**Rejected — a hostile `sys.stdout` whose EVERY attribute access raises `SystemExit(77)`.**
`exit=2, stderr_bytes=0`. Notably the injected `SystemExit(77)` does **not** leak into the
process status: `say()`'s guard catches it and `leave()` chooses `NO_ANSWER`. An attacker
who can raise inside the stream cannot choose the exit code.

**Rejected, and it is the most interesting one — a `sys.stdout` that SILENTLY DISCARDS**
(a `write` that returns `len(s)` and a `flush` that does nothing), on the swapped input.
`exit=1`, nothing delivered. **This is not a finding**, and I want to be precise about
why. The registered invariant is "EXIT 1 ARISES ONLY FROM THE NAMED ATTRIBUTION FINDINGS"
— a claim about the CAUSE of exit 1, and here the cause genuinely was 8 confirmed
inversions. A stream that reports success is indistinguishable from a working stream by
any means available to any program; no guard can detect a lying `write`. It also requires
injecting `sitecustomize.py` into the interpreter's own import path, which is not an
invocation of this instrument but a modification of it. Recorded because it is the exact
*shape* of the revision-8 defect (`print` on `None` silently succeeding) and I wanted to
confirm the fix addresses the reachable instance rather than the shape in general.

**Rejected — `ulimit -n` exhausted so `_mute()` cannot open `os.devnull`.** Two variants,
after a first attempt in which the SHELL failed to open `/dev/full` and reported its own
127 — a harness artefact, not the instrument, and discarded:

```
===C4 fd limit 4, /dev/full already open as fd 1   [C4] exit=2 stderr_bytes=0
===C5 fd limit 3, stdout CLOSED                    [C5] exit=2 stderr_bytes=0
```

The `sys.stdout = None` fallback in `_mute()` is doing real work here and it holds.

**Rejected — MemoryError, which was my best remaining candidate.** A 400 MB report under a
300 MB address-space limit makes `slurp()`'s `handle.read()` raise `MemoryError`, which
reaches the top-level handler, whose message must then be *allocated and formatted* under
the same exhausted limit:

```
===C8 exit=2 stderr_bytes=0
   out: warm_attribution_check: CANNOT READ: an unanticipated MemoryError escaped this
        instrument: MemoryError(). Something in one of these documents is malformed in a
        way no read above expected, or this process was interrupted, and either way it is
        a VOID rather than a finding about the engines
===C9 same, stdout CLOSED   exit=2 stderr_bytes=0
===C10 same, stdout on /dev/full   exit=2 stderr_bytes=0
```

The handler formats its `{why!r}` and delivers a named void even out of memory. Also tried
`ulimit -v` at 200 MB, 60 MB, 40 MB and 25 MB on the normal fixture (all `exit=0`) and
`ulimit -s 1000` (`exit=0`).

**Rejected — the `{why!r}` inside the handler raising.** This is the one structural hole I
found by reading: an exception whose `__repr__` raises would escape the `except
BaseException` block itself and exit 1 with a traceback. I could not reach it. Every
exception the instrument can raise comes from builtins or the stdlib, and their reprs do
not raise; C8 shows even `MemoryError` formats under memory pressure. Constructing one
requires injecting a custom exception class, which is modifying the program. **Recorded as
unreproducible, with the attempted reproducer**: I looked for any input-derived path to a
user-defined exception class and there is none — the instrument defines no exception
classes and raises only `OSError` explicitly.

**Rejected — recursion / `RecursionError`.** Established structurally rather than by
guessing, with an AST scan of all 22 functions for self-calls:

```
$ python3 - <<PY   # walk every FunctionDef, look for a Call to its own name
functions: 22 - no self-recursion printed above means none
PY
```

No self-recursive function exists; the parsers are flat loops over lines. Nothing to
report.

**Rejected — SIGINT with stdout genuinely CLOSED.** `exit=2, stderr_bytes=0`, using
`set -m` so the background job gets its own process group (without it, bash sets SIGINT to
`SIG_IGN` and Python inherits it — the trap the revision-8 reviewer recorded).

**Rejected — exit 120.** I could not produce it by any route. `say()` flushes per line and
`leave()` forces a final flush, so no buffered bytes reach CPython's shutdown flush; and
where the stream is broken, `sys.stdout` has already been replaced by devnull or `None`.

**Rejected — `PYTHONIOENCODING=ascii`** on the honest input and on a genuine finding.
Both `exit=2, stderr_bytes=0`, with `out_bytes=0` — no partial line escapes. The em-dash
in the instrument's own output cannot be encoded, `UnicodeEncodeError` is caught, and the
run becomes a clean void rather than a truncated answer. Correct, and no partial delivery.

**Rejected — making the honest control permissive.** I could not get exit 0 out of any
corrupted or hostile input. Structurally, no exit 0 is reachable through `die()`,
`violation()` or the handler; all three are terminal.

**Rejected — the regular-file guard breaking a legitimate invocation.** Plain file,
symlink and `/dev/stdin` redirected from a regular file are all still accepted at exit 0
(§1.10). Unchanged from revision 8.

**Rejected — an incomplete self-account.** I expected a fifth instance, digested all
fifteen sections at both revisions, and did not find one (§1.8). Recorded as rejected.

**Rejected — the `43e8a86` / `385631f` pins being stale in the narrative.** I checked all
eighteen occurrences. The narrative ones are past-tense and correct. The **pin** ones are
not, and that is MAJOR 1 — a different finding, arrived at from the other direction.

---

# 5. ANYTHING I COULD NOT VERIFY

1. **Whether `>&-` occurs in the operator's real Step 6 invocation.** As at revision 8,
   the fix is verified against the invariant as written, which is unconditional. I did not
   establish that the launch path ever closes stdout. This affects urgency, not
   correctness, and MAJOR 1 is closed regardless.

2. **Whether MAJOR 4's 138 files were committed deliberately.** I established they were
   untracked at `3696dea`, tracked at `6c929da`, unmentioned in the commit message and in
   D-425, and recorded by the previous reviewer as not theirs. I could not establish
   intent, and I removed nothing. If the operator added them deliberately, the finding
   reduces to "the commit message does not say so" — still a record defect, but a smaller
   one.

3. **`docs/decisions.md` D-401.** Not read, per the standing prohibition. Where the
   document cites D-401 I took the citation as given and checked nothing about it. No
   claim in this review depends on D-401's content.

4. **The governed run itself.** No governed sample was taken and none should be until this
   FAIL is closed. Nothing here says anything about the SPRT result.

5. **§8's dry-run timing figures and §7's cost table.** Confirmed byte-unchanged by
   section digest only (§1.8). I re-executed §8.6's checker arms (§1.10) but did not re-run
   `arena --replay`, so the `0.997x` family of figures is unverified by me and is
   byte-unchanged at this revision.

6. **The clause-(b) proof, §3's opening slice and §7A.1's agreement criterion.** Confirmed
   byte-unchanged, not re-litigated — but note that the section containing MAJOR 2 (lines
   690–697) IS inside a changed section, and I read it rather than fast-pathing it, which
   is how it was found.

7. **An observation I could not scope, recorded rather than filed as a finding.** D-423
   states `configs/arena_wp16_defensive_only_vs_staged.toml` "IS NOT AUTHORED", but the
   file exists and `git log` shows it authored at `c7132fb`, which
   `git merge-base --is-ancestor c7132fb 003cb18` confirms **predates** D-423.
   `config_check.sh` validates it today. D-423 is not under review this round and revision
   9 neither caused nor could fix this, so I am not counting it. It should be looked at by
   whoever next touches that ADR.

---

# 6. WHAT WOULD MAKE THIS PASS

Not a recommendation between design options — the finding list, closed:

1. **MAJOR 1**: re-record §7A.1 item 2 to denote this fix round (as §10 already tries to),
   and bring the header's and §11's "`385631f` moves the instrument" clauses with it. Make
   §10's cell say one thing.
2. **MAJOR 2**: delete or convert to explicit past tense §7A.1 lines 690–697, and have
   them point at §5 like the rest of the section does.
3. **MAJOR 3**: describe the registered check's closed-stdout legs in §7A.1, and replace
   the M6/M7/M8 receipt with one that names M12 as the mutation the current guards fail
   under — the honest account D-425 already wrote, moved into the section that registers
   the check.
4. **MAJOR 4**: an operator decision — either remove the 138 files, or record them in an
   ADR and split them out of a commit that claims to be a seven-finding fix round.
5. **MINOR 1–3**: state the digest pin as a property rather than a revision list; drop
   §5's "does NOT enumerate" sentence or reword it as "these are examples"; extend or
   de-enumerate item 3's derivation.

MAJOR 2, MAJOR 3 and all three MINORs are document-only. MAJOR 1 is document-only.
**MAJOR 4 is neither — it is a decision about what belongs in the repository.**
**No finding in this review requires touching the instrument**, whose correctness fix I
verified and could not break.
