# WP-1.6 pre-registration — REVISION 8 GOVERNING REVIEW

**Reviewed revision:** `3696dea102ba6e5af38e7605c398b93412d0056c`
(`docs(claude,experiments,decisions): D-424 -- delete the paragraph that failed three reviews`),
together with its parent `385631f`
(`fix(tools): an undelivered answer is no answer, and a FIFO is refused rather than hung on`).
The review was dispatched against both commits as one unit, and it reopened WHOLE
because `385631f` moves the instrument.

**Matches HEAD:** YES. `git rev-parse HEAD` → `3696dea102ba6e5af38e7605c398b93412d0056c`
at the start of this review and at the end.

**`git status --porcelain` at start:** empty (clean tree).

**`git status --porcelain` at end:**

```
?? .agents/
?? .claude/
?? docs/experiments/wp16_prereg_rev8_REVIEW.md
?? skills-lock.json
```

`docs/experiments/wp16_prereg_rev8_REVIEW.md` is this report, left **UNCOMMITTED** as
instructed. The other three appeared mid-session, all timestamped `Aug 24 17:47`, and contain agent
harness material (`.agents/skills/ask-matt`, `code-review`, `grill-me`, …) unrelated
to pistol. **They are not mine and not this review's**: I wrote nothing into the
repository. One stray file *was* mine — a `&-` created by a mis-escaped redirection in
an attack harness — and it is recorded under "Attacks attempted" below and was removed
(`rm -f -- '&-'`); the tree is otherwise as I found it. I did not remove the three
harness entries, since deleting another process's files is not a reviewer's move. Both
scratch worktrees I created were removed and `git worktree prune` run; `git worktree
list` is back to the three entries that were present at start.

**Reviewer:** fresh context. I did not write any part of `385631f` or `3696dea`.
`docs/decisions.md` **D-401 was not read** — not grepped, catted or opened — per the
standing operator prohibition. All other ADR lines cited below were read directly.

---

# VERDICT: **FAIL**

| Severity | Count |
|---|---|
| BLOCKING | **0** |
| MAJOR | **3** |
| MINOR | **4** |

**The pins HELD — all three, verified independently.**
**The header's self-account HELD — verified exactly, by section digest, not by hunk arithmetic.**

The two MAJORs that matter are, in one line each: **the registered exit-1 invariant is
still false** (a closed stdout makes the instrument exit **1 with a traceback**, and it
does so for pure VOIDs, which is the very thing exit 1 is registered to mean it is not);
and **the exit-2 partition D-424 deleted is still in the document**, standing in §7A.1
sixty lines below the paragraph that says it was deleted. The third is that **§11 still
tells this reviewer the review is "SCOPED to revision 7's three fixes"** — had I obeyed
it, I would have found neither of the other two.

---

# 1. WHAT IS CLEAN, STATED FIRST AND SPECIFICALLY

This revision is substantially better than the two that preceded it, and a great deal of
it is verified-clean rather than merely un-attacked.

### 1.1 The header's self-account is EXACTLY right — the fourth time is not the charm's failure

An incomplete self-account has been a finding three times (D-416 MINOR 4, D-419 MINOR G,
revision 7's near-miss). Revision 8's claim is:

> It amends THE HEADER (this block), §5, §7A.1, §10 and §11 — and nothing else.

I did not check this by mapping hunks; I digested **every section at both revisions** and
compared bytes, which is a referent the hunk arithmetic does not share:

```
section                                                                verdict
----------------------------------------------------------------------------------------
HEADER                                                                 CHANGED  484f29533abd -> ab1f0547907f
## 1. What is being judged, and what is not                            byte-identical
## 2. The hypothesis and the verdict unit                              byte-identical
## 3. The instrument                                                   byte-identical
## 4. What the run reports, and which lines are read                   byte-identical
## 5. Outcome handling, written before game one                        CHANGED  2b98d48e84d5 -> ac5688b129da
## 6. The honest expectation, and what a negative result means         byte-identical
## 7. Costs                                                            byte-identical
## 7A. The doubts, their instruments, their agreement criteria and th  byte-identical
### 7A.1 DOUBT 1 — the arena between the engines and the verdict       CHANGED  443c9d30be8b -> 78938ecbd4e2
### 7A.2 DOUBT 2 — whether the extension changes what the search comp  byte-identical
## 8. The dry run                                                      byte-identical
## 9. FILL-IN slots                                                    byte-identical
## 10. What flips this document                                        CHANGED  a01063be77a0 -> 4e4c3e336082
## 11. REVIEW STATE                                                    CHANGED  08cab1d7099f -> cd63a1637c9d
```

Exactly the five claimed regions changed and nothing else. **§1–§4, §6, §7, §7A.2, §8 and
§9 are byte-identical**, which is the fast-path confirmation the dispatch asked for. The
instrument is `/home/tom/pistol-testscratch-rev8attack/sections.py`, written for this
review; it splits on `^## ` and `^### 7A.\d` and digests each slice.

This also independently confirms the hunk mapping: §7A.1 spans lines 463–928 at
`1618467`, and every one of the thirteen §7A hunks (`-559`, `-571`, `-762`, `-766`,
`-770`, `-774`, `-779`, `-789`, `-798`, `-805`, `-814`, `-819`, `-861`) falls inside it,
none in §7A.2 (929–1001).

### 1.2 All three pins hold, and the binary digests genuinely reproduce

**Pin 1 — the warm-replay pass at `bfdf933`:**

```
$ git diff --stat bfdf933..HEAD -- crates/pistol-arena/
[prints nothing]
```

**Pin 2 — the statistics layer at `385631f`.** §7A.1 item 2 reads "**at commit
`385631f`, RE-RECORDED IN REVISION 8**", and §10's table and §11's row 8 agree. Correct.

**Pin 3 — the two binary digests, claimed UNCHANGED.** The document's own checkable form:

```
$ git diff --stat bfdf933..HEAD -- crates/*/src/ Cargo.toml Cargo.lock
[prints nothing]

$ sha256sum target/release/pistol target/release/arena
b8d0dc963a2453e1eff69823629c37b23bafe419b9225f8af2401df519bc2673  target/release/pistol
3ba8de615d4d708793d72c2f3c2f6c649811996bb331527e64d0f612a13aebc2  target/release/arena
```

Both equal the registered digests. And the supporting claim — that `43e8a86` and
`385631f` touch only the instrument and its test file — is true of both:

```
$ git show --stat --oneline 385631f
 .../tests/wp16_warm_attribution_check_tests.rs     | 128 +++++++++++++++++++
 tools/wp16_warm_attribution_check.py               | 137 +++++++++++++++----
$ git show --stat --oneline 43e8a86
 .../tests/wp16_warm_attribution_check_tests.rs     | 196 +++++++++++++++++++++
 tools/wp16_warm_attribution_check.py               |  73 +++++++-
```

**I did not take the live binaries on trust.** In a separate worktree at `bfdf933` with a
separate `CARGO_TARGET_DIR`, under the login shell (`~/.cargo/bin/rustc`, the rustup
toolchain — I confirmed `which rustc` inside the build shell, since the Arch
`/usr/bin/rustc` reports the same version and different bytes):

```
$ cd /home/tom/pistol-testscratch-rev8digest && CARGO_TARGET_DIR=... cargo build --release --locked
    Finished `release` profile [optimized] target(s) in 6.09s
$ sha256sum /home/tom/pistol-testscratch-rev8target/release/{pistol,arena}
b8d0dc963a2453e1eff69823629c37b23bafe419b9225f8af2401df519bc2673  .../release/pistol
3ba8de615d4d708793d72c2f3c2f6c649811996bb331527e64d0f612a13aebc2  .../release/arena
$ rustc --version && which rustc
rustc 1.97.1 (8bab26f4f 2026-07-14)
/home/tom/.cargo/bin/rustc
```

Byte-exact, from a tree that shares nothing with the live `target/` but the source. The
worktree and its target directory were removed afterwards.

### 1.3 The instrument still discriminates — the honest control passes and the seeded swap fails

Neither has gone permissive. Both arms reproduce §8.6's registered figures verbatim.

Honest arm (exit **0**):

```
warm_attribution_check: W coverage: 8 game(s) accounted for — 8 replayed in full with every node count equal to the run's, 0 halted at a divergence. …
warm_attribution_check: W classification: 0 divergence(s), 0 confirmed inversion(s), 0 unexplained
warm_attribution_check: (b): 0 inert pair(s) excluded by theorem, 4 pair(s) directly attributed at their first differing searched turn, 0 unattributable
warm_attribution_check: cross-check: no inert pairs — the exclusion changed nothing
warm_attribution_check: 1b: 5 decided non-forfeit game(s) adjudicated against the move list
warm_attribution_check: 1c: 8 game(s) and 4 pair(s) rebuilt off the score_a path
warm_attribution_check: PASS — 0 failure(s)
EXIT=0
```

Seeded swap (exit **1**, `8 confirmed inversion(s)`, `FAIL — 13 failure(s)`), including
link 1c catching the same corruption from the other direction:

```
warm_attribution_check: W classification: 8 divergence(s), 8 confirmed inversion(s), 0 unexplained
warm_attribution_check: FAIL (a) game 0 turn 5: the report credits `-4,0/-1,0` to `staged`, … the seats are the wrong way round
warm_attribution_check: FAIL 1c `counts wins_a 2` against 3 rebuilt from the `game` lines
warm_attribution_check: FAIL — 13 failure(s)
EXIT=1
```

All four dry-run artefacts carry the digests §8.6 records:

```
6e2a531c8e346b23a661fd96abef15f847e7c6f60cc0d8ac4a8813e7e007c793  artifacts/wp16_warmreplay_dryrun_run.txt
cf91e3fa9484d1ffcd7e0573ef2f349452e8065fa14c5f45d9214d1e31ad6170  artifacts/wp16_warmreplay_dryrun_replay.txt
377521bfd08408c395402d37e238ce9bdfeaebe5b26579358f0afb0001595882  artifacts/wp16_warmreplay_dryrun_swapped.txt
b63395e2b8c2d6f1d467920b6edcf5e167626ae07a19e3b252c86925901b4eca  artifacts/wp16_warmreplay_dryrun_swapped_replay.txt
```

### 1.4 The regular-file guard (D-422) is correct in both directions

This is a clean, complete fix. It refuses what cannot carry a report, **by name and
instantly**, and it does not break any legitimate path I could find:

| Path kind | Result |
|---|---|
| FIFO (as report) | exit **2**, `… is not a regular file. A directory, device or FIFO cannot carry a report…`, no hang |
| FIFO (as replay doc — the *second* argument) | exit **2**, same refusal, no hang |
| directory | exit **2**, named |
| device (`/dev/zero`) | exit **2**, named |
| nonexistent | exit **2**, `… does not exist` |
| **symlink → regular file** | **exit 0, accepted** (`PASS — 0 failure(s)`) |
| **ordinary copy** | **exit 0, accepted** |
| **`/dev/stdin` (redirected from a regular file)** | **exit 0, accepted** |
| process substitution `<(cat …)` → `/dev/fd/63` | exit 2, refused |

The second argument being guarded too is the point that matters: the D-422 fix is not
scoped to one call site, which is the `SHELL_CHECKLIST` item 11 "sweep, not the instance"
failure mode. `slurp()` is the single choke point and both callers go through it.

Process substitution is refused, and I decided this is **not** a finding: no registered
command in §7A.1, §8.2 or §8.6 uses `<(…)`; the refusal is exit 2 (a void, named), never
a wrong answer; and a pipe genuinely can hang, which is the whole reason for the guard.
Recorded here so the behaviour change is visible rather than discovered later.

### 1.5 The mutation table: 8 of 9 killed as claimed, M9's 10 s is real

Re-run by me, from scratch, in a **separate worktree** at `3696dea`
(`/home/tom/pistol-testscratch-rev8mut`) with its own `CARGO_TARGET_DIR` — never the live
tree. Baseline first, so a kill cannot come from an already-red target:

```
===== BASELINE (unmutated, in the mutation worktree)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s
```

| Mut | My result | Killed by |
|---|---|---|
| M1 remove `clause_b`'s empty-range guard | **KILLED** `FAILED. 14 passed; 1 failed` | `a_pair_mate_shorter_than_the_book_is_a_refusal_and_not_a_crash` |
| M2 revert handler to `(KeyError, ValueError, IndexError)` | **KILLED** `FAILED. 14 passed; 1 failed` | `an_unanticipated_exception_is_a_refusal_and_not_a_finding` |
| M3 guard that does not name the short game | **KILLED** `FAILED. 14 passed; 1 failed` | `a_pair_mate_shorter_than_the_book_is_a_refusal_and_not_a_crash` |
| M4 catch-all that does not name the class | **KILLED, but only in its full form** — see note | `an_unanticipated_exception_is_a_refusal_and_not_a_finding` |
| M5 `main()` always returns `NOT_A_MEASUREMENT` | **KILLED** `FAILED. 9 passed; 6 failed` | six tests incl. every control |
| M6 `say()` lets the OSError escape | **KILLED** `FAILED. 14 passed; 1 failed` | `an_answer_that_cannot_be_delivered_is_no_answer_and_not_a_finding` |
| M7 `leave()` ignores `DELIVERED` | **KILLED** `FAILED. 14 passed; 1 failed` | `an_answer_that_cannot_be_delivered_is_no_answer_and_not_a_finding` |
| M8 `leave()` does not force the flush | **KILLED** `FAILED. 14 passed; 1 failed` | `an_answer_that_cannot_be_delivered_is_no_answer_and_not_a_finding` |
| M9 the regular-file guard removed | **KILLED**, `finished in 10.01s` | `a_path_that_is_not_a_regular_file_is_refused_rather_than_blocked_on` |

Restored afterwards: `test result: ok. 15 passed; 0 failed; …`.

**M6/M7/M8 confirm §7A.1's central new claim** (line 825–826): the registered check *is*
one the defect falsifies. That claim is TRUE for the defect class it names.

**M9's ~10 s is real and is `timeout 10` firing**, exactly as claimed — `10.01s` against a
`0.28s` baseline. The dispatch asked me to check that a test for a hang cannot itself
hang: it cannot. `Command::new("timeout").arg("10")` bounds it, and the test asserts
`assert_ne!(code, Some(124))` *before* asserting `Some(2)`, so a reinstated hang is
reported as a named failure rather than as a stuck suite. This is properly built.

**M4 note, recorded because it is a real property of the table, not a finding.** My first
M4 (drop `{type(why).__name__}`) **SURVIVED**. So did the mirror (drop `{why!r}`). The
class name is delivered **twice in one message**:

```
f"warm_attribution_check: CANNOT READ: an unanticipated {type(why).__name__} "
f"escaped this instrument: {why!r}. Something in one of these documents is "
```

`repr(ZeroDivisionError(...))` contains `ZeroDivisionError`, so either site alone
satisfies `printed.contains("ZeroDivisionError")`. Removing **both** kills it:

```
===== M4a drop only type(why).__name__     => SURVIVED
===== M4b drop only {why!r}                => SURVIVED
===== M4c drop BOTH namings                => KILLED  (an_unanticipated_exception_is_a_refusal_and_not_a_finding)
```

M4a/M4b are **equivalent mutants**: the observable behaviour the test pins — "the refusal
NAMES the exception" — is preserved by the redundancy. The property is genuinely covered.
I record it because "M4 killed" is true of the behaviour and false of either single edit,
and a future session re-running this table should not be surprised.

### 1.6 The registered gates are green, and the commit's own numbers are honest

```
$ cargo test --workspace --locked
[139 targets aggregated] passed=744 failed=0 ignored=9
DONE-0
$ grep -c "^test result: FAILED" <log>
0

$ cargo clippy --workspace --all-targets --locked -- -D clippy::all
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
CLIPPY-EXIT=0

$ cargo fmt --all --check
FMT-CLEAN
```

`385631f`'s commit message claims "742 -> 744 passed, 0 failed, 9 ignored" and "15 tests
in the target, was 13". Both reproduce exactly. The target itself:

```
running 15 tests
…
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s
```

### 1.7 The delivery fix does work — for the defect it was written for

The registered check passes, both legs, with empty stderr:

```
===== A1 devfull default buffering
exit=2
stderr_bytes=0
===== A2 devfull PYTHONUNBUFFERED=1
exit=2
stderr_bytes=0
```

D-421 MAJOR 1's two measured signatures (exit 120; exit 1 + traceback) are genuinely
gone on `/dev/full`. The funnel through `say()`/`leave()` is sound design and I attacked
it on many other axes without success (§4).

### 1.8 D-424 is an accurate ADR

I read D-424 in full and checked its historical claims against D-419, D-421, D-422 and
D-423. Its account of the three MAJORs is correct; its "WHAT WAS NOT OVERRULED" paragraph
correctly records that D-421 MAJOR 1, D-422 and D-423 MAJOR 2 were all closed **in code**
rather than waived, which is the discipline the rule claims for itself. Its final general
lesson — "a registered check over a document's TEXT cannot falsify a defect in an
instrument's RUNTIME" — is exactly right and is the reason this revision is better than
the last two.

---

# 2. FINDINGS

## MAJOR 1 — §7A.1's registered exit-1 invariant is STILL FALSE. A closed stdout makes the instrument exit **1 with a traceback**, and it does so for pure VOIDs

**Where.** `docs/experiments/wp16_sprt_prereg.md` §7A.1 line 795–798, stated with the
narrowing revision 7 used deliberately removed:

> **THE INVARIANT, and it is now TRUE AS WRITTEN rather than narrowed to fit:**
> **EXIT 1 ARISES ONLY FROM THE NAMED ATTRIBUTION FINDINGS.** Every other
> termination of the instrument is exit 0, exit 2, or exit 3.

**It is not true as written.** Minimal reproducer — one shell redirection, no exotic
tooling, `>&-` closes fd 1:

```
$ cd /home/tom/Projects/HeXO-AlphaBeta
$ python3 tools/wp16_warm_attribution_check.py \
    artifacts/wp16_warmreplay_dryrun_run.txt \
    artifacts/wp16_warmreplay_dryrun_replay.txt \
    target/release/pistol >&- 2>/tmp/e3
$ echo $?
1
$ head -c 830 /tmp/e3
Traceback (most recent call last):
  File ".../tools/wp16_warm_attribution_check.py", line 959, in <module>
    leave(main())
    ~~~~~^^^^^^^^
  File ".../tools/wp16_warm_attribution_check.py", line 224, in leave
    sys.stdout.flush()
    ^^^^^^^^^^^^^^^^
AttributeError: 'NoneType' object has no attribute 'flush'

During handling of the above exception, another exception occurred:

Traceback (most recent call last):
  File ".../tools/wp16_warm_attribution_check.py", line 969, in <module>
    leave(NO_ANSWER)
    ~~~~~^^^^^^^^^^^
  File ".../tools/wp16_warm_attribution_check.py", line 224, in leave
    sys.stdout.flush()
    ^^^^^^^^^^^^^^^^
AttributeError: 'NoneType' object has no attribute 'flush'
```

This is, word for word, the signature §7A.1 and the handler's own comment say was
abolished: **"EXIT 1 WITH A TRACEBACK AND NO `CANNOT READ:` LINE, the exact signature
that handler exists to abolish, surviving inside it."** It survives inside it again.

**The mechanism**, measured rather than argued. When fd 1 is closed at interpreter
startup CPython sets `sys.stdout` to `None`, and `print()` **silently returns** when
`sys.stdout is None`:

```
$ python3 -c "import sys; sys.stderr.write(repr(sys.stdout)+chr(10))" >&-
None
$ python3 -c "
import sys
print(chr(65))
sys.stderr.write('print returned normally, delivered nothing'+chr(10))
" >&-
print returned normally, delivered nothing
```

So the guards do not fire. `say()` catches only `OSError`; `print()` raises nothing, so
**`DELIVERED` stays `True` while nothing is delivered** — the "an undelivered answer is no
answer" rule is bypassed entirely, not triggered. Then `leave()`'s `sys.stdout.flush()`
raises `AttributeError`, which is neither `OSError` (so `leave`'s guard misses it) nor
`SystemExit` (so the top-level re-raise misses it). The `BaseException` handler catches
it, calls `say()` (silent no-op), then calls `leave(NO_ANSWER)` — **which raises
`AttributeError` again, this time inside the handler**, escaping the module. CPython
prints the chained traceback and exits **1**.

**Why this is worse than exit 120 was.** The failure is not confined to the honest case.
Both other exit classes are dragged into exit 1:

```
===== B7 stdout closed with a CANNOT READ path (nonexistent report)
exit=1
stderr_bytes=1438
Traceback (most recent call last):
  ...
  File ".../wp16_warm_attribution_check.py", line 870, in main
    report = read_report(sys...
```

A **nonexistent report file** — the purest possible VOID, `die("the report … does not
exist")` — exits **1**. §5 registers exit 1 as:

> **The run is not a measurement, and it is not an `h0` either.** The verdict is not read…

and §7A.1's table registers exit 1 as "**the NAMED attribution findings, and nothing
else**". So a missing file is reported in the vocabulary of a finding about the engines.
That is **precisely D-419 MAJOR B** — "A refusal wearing a finding's exit code" — which
§7A.1 line 657 declares "CLOSED IN CODE AND NOT BY NARROWING THIS RECEIPT".

It also reaches the genuine-finding path (the finding is never delivered):

```
===== B6 stdout closed on the SWAPPED (genuine finding) input
exit=1
stderr_bytes=830
```

and the interrupt path, with stdout closed:

```
===== G1 SIGINT with stdout genuinely CLOSED
exit=1
stderr_bytes=1836
Traceback (most recent call last):
  File ".../wp16_warm_attribution_check.py", line 959, in <module>
    leave(main())
  File ".../wp16_warm_attribution_check.py", line 892, in main
    inversions = classify(report, replay, engine, notes)
```

Consolidated reproducer, the two lines that matter:

```
===== D4 MINIMAL REPRODUCER of the closed-stdout defect
-- honest input, stdout closed:
   exit=1 (registered invariant says 0, 2 or 3)
-- nonexistent report (a pure VOID), stdout closed:
   exit=1 (registered consequence of 1 = NOT A MEASUREMENT, about the ENGINES)
```

**Scope item 4, discharged — every way the process can terminate:**

| # | Termination | Exit | Invariant holds? |
|---|---|---|---|
| 1 | `leave(main())`, delivered | 0 or 1 | ✅ 1 only from `failures` |
| 2 | `die()` → `leave(NO_ANSWER)` | 2 | ✅ |
| 3 | `violation()` → `leave(3)` | 3 | ✅ |
| 4 | `BaseException` handler → `leave(NO_ANSWER)` | 2 | ✅ |
| 5 | Write/flush `OSError` (`/dev/full`, read-only fd, EPIPE) → downgrade | 2 | ✅ measured |
| 6 | **`sys.stdout is None` (fd 1 closed) → `AttributeError` escapes the handler** | **1 + traceback** | ❌ **BREACH** |
| 7 | Uncatchable/uncaught signal (SIGKILL, SIGTERM=143) | 137/143 | ✅ §5's "none of 0,1,2,3" row |
| 8 | SIGINT → `KeyboardInterrupt` → handler | 2 | ✅ measured (F1) |

Row 6 is the finding. Rows 1–5, 7 and 8 I verified individually and they hold.

**Why the registered check did not catch it.** The test opens `/dev/full` — an fd that is
**open and unwritable**. The defect lives in the disjoint case, an fd that is **not open
at all**. The test is a good check that could not falsify this particular defect, which is
the same shape of gap D-423 MAJOR 2 raised against the fifth grep — one level up.

**The fix is small** (guard `sys.stdout is None` in `say()`/`leave()`/`_mute()`, or catch
`(OSError, AttributeError, ValueError)`, or re-point `sys.stdout` at `os.devnull` at
start-up when it is `None`), and it wants a test leg with a genuinely closed descriptor
beside the `/dev/full` leg. **I make no recommendation between those; that is the
implementer's call.**

**Severity note.** I grade this MAJOR and not BLOCKING for consistency with precedent:
D-421 MAJOR 1 was this same claim, in this same section, found false by the same method,
and was graded MAJOR. It fails the document on its own regardless of the label. Per
CLAUDE.md, this is a finding that "names a way the code can produce a wrong answer" and
is therefore **not overrulable — only fixable**.

---

## MAJOR 2 — the exit-2 partition D-424 DELETED is still in §7A.1, with its per-kind consequences, contradicting §5 and §7A.1's own rule

**Where.** `docs/experiments/wp16_sprt_prereg.md` lines **846–851**, a *second* exit table
in §7A.1, sixty lines below the first one.

§7A.1 line 777 states the rule revision 8 is built on:

> **THE REGISTERED CONSEQUENCE OF EACH EXIT CODE LIVES IN §5 AND ONLY IN §5.** This
> section states what the INSTRUMENT does … and points at §5 for what may be CONCLUDED.
> **It does not restate §5's table**, and that is a rule rather than a stylistic
> preference…

Its own table at lines 788–794 obeys that: every Consequence cell reads `§5`. Correct.

Then at line 845 a **second table** begins, whose third column is headed
"**Consequence, registered here**", and whose exit-2 row is the deleted partition,
verbatim:

```
$ sed -n '845,851p' docs/experiments/wp16_sprt_prereg.md
| Exit | What reaches it | Consequence, registered here |
|---|---|---|
| 0 | Criterion 1'' holds | §5's table is read, and only then |
| 1 | **the NAMED attribution findings, and nothing else** — …
| 2 | **two kinds, partitioned by RULE rather than by list.** **(i) a VOID — the CLOSED list**: a missing or unrunnable engine, an unreadable or non-UTF-8 document, an incomplete or abandoned replay pass, a budget this cannot replay. Not a finding, not evidence about any engine; the void is fixed and the answer re-taken. **(ii) a REFUSED REPORT — the REGISTERED CATCH-ALL**: … **Nothing here is "fixed" and nothing is "re-taken"** … | as stated per kind. **The reader does not need to match a message against a list**: kind (i) is closed above, so an exit 2 that is not one of those four IS kind (ii) by rule …
| 3 | …
```

The dispatch asked me to grep for exactly these strings. They are all present:

```
$ grep -n "CLOSED list\|kind (i)\|kind (ii)\|REGISTERED CATCH-ALL" docs/experiments/wp16_sprt_prereg.md
90:  CLOSED list it genuinely can be, kind (ii) becomes a REGISTERED CATCH-ALL
850:| 2 | **two kinds, partitioned by RULE rather than by list.** **(i) a VOID — the CLOSED list**: …
```

(Line 90 is inside the header's *historical* narrative of revision 6 and is legitimately
past-tense. **Line 850 is not history — it is a live registered-consequence table.**)

**What this falsifies.** Four separate claims in this same revision:

1. **The header, lines 25–26:** "§5 now carries **one** exit-2 row" — true of §5, but the
   partition it says was deleted is still registered in §7A.1.
2. **§7A.1 line 777:** "does not restate §5's table" — it restates it, and the restatement
   is not even the same table.
3. **D-424 itself:** "§5 now carries ONE exit-2 row and the operator reads the
   instrument's own printed message for what to do next."
4. **CLAUDE.md's brand-new rule**, added by this very commit: "A CLAIM THE DOCUMENT MAKES
   TWICE IS A DEFECT WAITING. State it once, in the section that owns it…" — the rule is
   violated in the commit that introduces it.

**And the two texts give materially different instructions.** §5's single row says what to
do next is *operational* and explicitly **not registered**:

> What to do next … is read off the instrument's own message… **That is operational and
> this document does not register it.**

Line 850 registers it, per kind, and in opposite directions: kind (i) "the void is fixed
and the answer re-taken"; kind (ii) "**Nothing here is 'fixed' and nothing is
're-taken'**". So at revision 8 the document contains two registered, conflicting readings
of the same exit code, with no stated precedence — which is the after-the-numbers
ambiguity that CLAUDE.md's registered-consequence rule exists to forbid, and which was
revision 1's own BLOCKING.

**This is D-423 MAJOR 1 exactly repeated**: "revision 7 corrected the exit taxonomy in §5
and left §7A.1's copy byte-identical, and the document shipped self-contradicting."
Revision 8 corrected §5, added the *rule* against doing this, and left a copy in §7A.1
anyway. The fix is deletion of lines 845–851 (and the surrounding paragraph at 853–858,
"A NOTE ON DIRECTION, so the exit-2 catch-all is not over-read as laxity", which is about
the deleted catch-all).

---

## MAJOR 3 — §11 still directs this review to be "SCOPED to revision 7's three fixes", under a cap D-423 declared exhausted

**Where.** `docs/experiments/wp16_sprt_prereg.md` line **1377**, the closing paragraph of
§11 REVIEW STATE — a section revision 8 *did* amend:

```
$ grep -n "SCOPED to revision 7" docs/experiments/wp16_sprt_prereg.md
1377:A fresh review — SCOPED to revision 7's three fixes, because revision 6's whole
```

In full:

> A fresh review — SCOPED to revision 7's three fixes, because revision 6's whole review
> has already been taken and its cap licenses exactly one fix round and one scoped
> re-review — must pass before the governed run this document describes may be launched.
> **Any diff outside this document voids that scoping.**

Every clause of this is false at revision 8:

- **"SCOPED to revision 7's three fixes"** contradicts the header (lines 4–6): "It MOVES
  THE INSTRUMENT: `385631f` re-pins §7A.1's statistics layer, so **this review reopens
  WHOLE**", and contradicts §11's *own* row 8 twelve lines above: "**NOT document-only**;
  this review reopens WHOLE".
- **"revision 7's three fixes"** are not the change set under review; revision 8 is a
  deletion plus an instrument move.
- **"its cap licenses exactly one fix round and one scoped re-review"** — D-423 records
  that cap as **EXHAUSTED**: "THE CAP IS EXHAUSTED — ITS ONE LICENSED FIX ROUND HAS BEEN
  SPENT AND ITS RE-REVIEW HAS RETURNED — SO THIS SESSION STOPS."
- **"Any diff outside this document voids that scoping"** — `385631f` *is* a diff outside
  this document, so by this sentence's own terms the scoping is void; the sentence
  refutes itself and still instructs.

**Why this is MAJOR and not MINOR.** It is not stale prose in a narrative section — it is
the operative instruction to the reviewer, in the section that owns review state, and it
is the last thing §11 says. A reviewer who obeyed it would scope to three fixes in a
superseded revision and would **not** re-run the mutation table, **not** re-verify the
binary digests, and **not** attack the delivery invariant. Concretely: had I obeyed line
1377, I would have found neither MAJOR 1 nor MAJOR 2. This is a document instruction that
actively degrades the gate protecting an expensive run.

---

## MINOR 1 — §7A.1 says "~50" `die()` sites; there are 56

```
$ grep -oE '\bdie\(' tools/wp16_warm_attribution_check.py | wc -l
57
```

57 matches = 1 `def die(` + **56 call sites**. §7A.1 line 792 reads "There are ~50 of
them". D-424's own history quotes the earlier figure ("49 `die()` sites, 7 quoted"), so
the number has moved twice and the document rounds the wrong way. Measurable in seconds.

It is a MINOR and not more because the same row explicitly declines to enumerate or
partition them, so the count does no work — by CLAUDE.md's own test ("whether the disputed
claim changes what anyone may conclude") nothing turns on it. But it is a stated number in
a governing document that is wrong.

## MINOR 2 — §10's binary-digest row is stale relative to §7A.1

§10's pin table:

> | `target/release/arena` and `target/release/pistol`, BY CONTENT | the two `sha256`
> digests, **unchanged by revision 6** | §7A.1 |

§7A.1 item 3 says "**UNCHANGED THROUGH REVISIONS 6, 7 AND 8**". §10 was amended by
revision 8 and this cell was not brought along. Not false (they *were* unchanged by
revision 6) but incomplete in a table whose purpose is to say what is pinned *now*.

## MINOR 3 — §5's "none of 0, 1, 2, 3" row asserts a complete enumeration that is not complete

The row, rewritten by revision 8, ends:

> What remains is what no process can catch: `SIGKILL`, the OOM killer, a machine that
> stops. Those are facts about the box and this row is where they are read

**SIGTERM is catchable**, the instrument installs no handler for it, and it lands in this
row — measured:

```
===== D3 SIGTERM during the swapped run
exit=143
stderr_bytes=0
```

143 = 128+15. SIGHUP and SIGQUIT behave the same way. So the set of terminations reaching
this row is strictly larger than "what no process can catch". The **consequence** the row
registers is right for 143 (a void about the invocation environment), so nothing is
misread — hence MINOR. But it is an enumeration presented as complete that is not, in a
row revision 8 rewrote, which is the exact defect class (D-419 MAJOR A) that D-424 says it
has ended. "What remains is whatever this process does not catch, including uncaught
signals" would be true and would cost nothing.

## MINOR 4 — the delivery test's control assertion does not satisfy SHELL_CHECKLIST item 12 obligation 3

`crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs:948`:

```rust
assert_eq!(
    control.status.code(),
    Some(0),
    "the control is refused, so nothing below is about delivery: {}",
    said(&control)
);
```

Item 12 obligation 3 asks that a test driving a gate "asserts on the code it expects AND
says, in the failure message, what the other codes would have meant" — and names
`assert_eq!(code, Some(0))` as the anti-pattern that reports a void as a regression. The
message here does not distinguish 1 (a finding) from 2 (a void) from 3 (a determinism
violation); all three read as "the control is refused". The *main* assertion in the same
test discharges the obligation very well (it names what exit 1 and exit 120 would have
meant), which is why this is MINOR and confined to the control.

---

# 3. SHELL_CHECKLIST ITEMS, ANSWERED BY NAME

`385631f` changes `tools/wp16_warm_attribution_check.py`. It is Python, not shell under
`set -euo pipefail`, so several items are about a hazard class the diff cannot contain;
those are marked N/A **with the reason**, not waved.

**Item 1 — a command substitution whose status is DISCARDED.** N/A in form (no shell
substitution), but the analogous hazard is present and is **discharged**: `cold_answer()`
takes `subprocess.run(...)` into a variable, then validates the *shape* of what came back
(`len(best) != 1`) and refuses by name, quoting both streams — rather than interpolating
an unchecked value into a record. `subprocess.run` without `check=True` cannot die
silently because the return value is inspected. Clean.

**Item 2 — a pipeline in a `then` body is not a pipeline in a condition.** N/A: no shell
pipelines in the diff.

**Item 3 — `grep` under `pipefail`; a substring is not a token.** N/A for `grep`, but the
"substring is not a token" half **applies to the regexes** and is discharged: every
pattern in `read_report`/`read_replay` is `re.M`-anchored with `^…$`
(`r"^budget (\S+) (\d+)$"`, `r"^engine {slot} label (\S+) …$"`), and `only()` refuses both
"no match" and ">1 match" by name — "more than one …, so there is no one answer to read".
That is stronger than anchoring alone. Clean.

**Item 4 — `LC_ALL`, and which direction it moves a guard.** N/A for locale pinning, but
the item's real subject — a guard whose width depends on how "line" is defined — is
**explicitly discharged**, and well. `read_report` uses `split("\n")` and **not**
`splitlines()`, with the reason in a comment: `splitlines()` also breaks on `\r`, `\x0b`,
`\x0c`, U+2028 and U+0085 while `re.M` breaks on `\n` alone, and two notions of "line"
over one document is how an engine's verbatim refusal injects a record. This is the
correct direction (the narrower, agreeing definition). Unchanged by this diff but re-read
and confirmed still correct.

**Item 5 — the index is what commits; the working tree is not.** N/A: the instrument reads
caller-named artefact paths, never `git ls-files`, and makes no claim about tracked bytes.

**Item 6 — a sweep by prefix must own the prefix.** N/A to the instrument (it deletes
nothing). It **applies to the test file**, which `385631f` changes, and is discharged: the
new tests use `scratch("wp16warm-undelivered")` / `scratch("wp16warm-fifo")`, and
`scratch()` builds under `SCRATCH_PREFIX` = `pistol-testscratch-`, documented at
`crates/pistol-cli/tests/common/mod.rs:95` as "a name nothing but these suites writes",
with the `pistol-` incident recorded. Correct prefix, no new sweep. I also adopted that
prefix for my own scratch directories.

**Item 7 — traps.** N/A: no `trap`, no EXIT handler. The analogue — "the last thing that
runs decides the status" — is exactly what `leave()` is, and it is where **MAJOR 1**
lives: `leave()`'s final act raises an uncaught `AttributeError` and lets CPython choose
the status. **This item is BREACHED in substance**, which is MAJOR 1.

**Item 8 — one spelling per number, one refusal per reason.** **Discharged, and it is the
item that produced D-422's fix.** "One spelling per number": `fields()` refuses a repeated
key by name — "the key `{key}` appears twice on one record" — rather than last-wins, so a
human reading the first `result` and the instrument reading the second cannot diverge.
"One refusal per reason": `slurp()` now gives **three** distinct refusals where it
previously had one behaviour — does-not-exist, is-not-a-regular-file, and could-not-be-read
(`OSError`) — which is precisely this item's "`command -v` … ACCEPTS a FIFO that then
blocks every read — three reasons, three refusals". Verified by test (C1–C4 in §1.4).

**Item 9 — what reaches a record is caller-controlled.** Discharged. Refusals quote the
offending value back in backticks and the document is not re-parsed by another tool. The
`split("\n")` choice under item 4 is the load-bearing guard against line injection from an
engine's free-text refusal. No `basename` is used. Note that the *new* refusal message
interpolates `path` — a caller-supplied string — but it goes to stdout for a human, and
exit 2 stops anything downstream from reading it as data.

**Item 10 — THE COVERAGE RULE (the binding one).** **DISCHARGED, and this is the strongest
part of `385631f`.** The script produces a recorded number (the Criterion 1'' verdict) and
carries **15 tests in a suite CI runs**, all driving the **shipped** script — `check()`
invokes `python3` on `repo("tools/wp16_warm_attribution_check.py")`, and `repo()` resolves
from `CARGO_MANIFEST_DIR` two levels up, so a mutation in a worktree is what that
worktree's tests run (which is why my mutation table works at all). Both new tests have
**controls**: the delivery test asserts a plain run exits 0 before testing `/dev/full`, and
the catch-all test asserts the un-edited fixture exits 0. So a pass cannot come from a
checker that refuses everything. `test result: ok. 15 passed; 0 failed`.

**Item 11 — a caller's path that feeds a delete or an overwrite is containment-guarded.**
N/A, and I enumerated rather than assumed: `385631f`'s diff contains **no** `rm`, `mv`,
`shutil`, `os.remove`, `os.rename` or write to a caller-named path. The instrument opens
its three arguments read-only (`open(path, "rb")`) and writes only to stdout. `_mute()`
opens `os.devnull` for writing — a fixed, script-chosen constant, not caller-supplied. The
new tests write only under `scratch()`. No destructive site exists, so there is nothing to
guard. This enumeration is the evidence the item asks for.

**Item 12 — a gate distinguishes RUN VOID from FAIL, by name.** This is the item the whole
commit is about, and it is **partly discharged and partly breached**.
- *Obligation 1, a code per kind*: satisfied in the constants and named as such —
  `ATTRIBUTABLE = 0`, `NOT_A_MEASUREMENT = 1`, `NO_ANSWER = 2`, `DETERMINISM_VIOLATION = 3`,
  with the comment citing this item. The docstring's closing paragraph states all four.
  **But MAJOR 1 is a direct breach**: a void (missing file) is spelled as `1`, the FAIL
  code — "a gate that spells them the same way turns every environmental accident into a
  regression report", which is this item's opening sentence.
- *Obligation 2, preflight and void early*: satisfied and improved. `slurp()` now preflights
  the path kind *before* opening, which is what turns D-422's hang into an early named
  void. The `read_report` budget check ("this checks only a `nodes` budget…") is another.
- *Obligation 3, the distinction survives the seam*: mostly satisfied — the FIFO test
  asserts `!= Some(124)` **and** `== Some(2)` with distinct messages, and the delivery
  test's main assertion names what 1 and 120 would have meant. **MINOR 4** is the one
  control assertion that does not.

---

# 4. ATTACKS ATTEMPTED AND REJECTED

Substantial effort went at the delivery invariant, since the last two reviewers each broke
it. Everything below is a real run; only the closed-descriptor family succeeded.

**Rejected — `/dev/full`, default buffering.** `exit=2, stderr_bytes=0`. The registered
check holds.

**Rejected — `/dev/full` under `PYTHONUNBUFFERED=1`.** `exit=2, stderr_bytes=0`. D-421
MAJOR 1's second signature is genuinely closed.

**Rejected — stdout and stderr *both* on `/dev/full`.** `exit=2`. `_mute()` prevents the
shutdown flush from re-raising even with no working stderr.

**Rejected — stderr closed, stdout on `/dev/full`** (`>/dev/full 2>&-`). `exit=2`.

**Rejected — `PYTHONFAULTHANDLER=1` with `/dev/full`.** `exit=2, stderr_bytes=0`. The
fault handler does not fire and does not alter the code.

**Rejected — stdout on a READ-ONLY descriptor** (`1</dev/null`). `exit=2,
stderr_bytes=0`. The `EBADF` on write is an `OSError` and is caught.

**Rejected — SIGPIPE via `| head -1`.** `pipestatus=0, stderr_bytes=0`, no traceback. The
output is seven short lines and fits the 64 KiB pipe buffer, so every write succeeds
before `head` exits; the reader discarding delivered bytes is not a transport failure. I
could not construct a payload large enough to force EPIPE from this instrument, and if I
could, `BrokenPipeError` is an `OSError` and would be caught. Rejected as unreproducible.

**Rejected — SIGINT / `KeyboardInterrupt` with a working stdout.** First attempt was
**inconclusive, not a finding**: I sent SIGINT to a background job in a non-interactive
shell, where bash sets SIGINT to `SIG_IGN` and Python inherits it, so the signal was
ignored and the run completed normally (`exit=3` from a genuine determinism violation
against my slow shim). Redone with `set -m` so the job gets its own process group, and
with a 5-second shim so the process is reliably alive:

```
===== F1 SIGINT, stdout OPEN
exit=2
warm_attribution_check: CANNOT READ: an unanticipated KeyboardInterrupt escaped this instrument: KeyboardInterrupt(). … it is a VOID rather than a finding about the engines
stderr_bytes=0
```

The `BaseException` change works exactly as §7A.1 and the handler comment claim. Rejected.

**Rejected — SIGINT with stdout on `/dev/full`.** `exit=2, stderr_bytes=0`.

**Rejected — SIGTERM.** `exit=143`, no traceback. Correctly lands in §5's "none of
0, 1, 2, 3" row; produced MINOR 3 about that row's wording, not a breach of the exit-1
invariant.

**Rejected — the regular-file guard breaking a legitimate invocation.** Symlink-to-file,
an ordinary copy, and `/dev/stdin` redirected from a regular file are all still accepted
(exit 0, `PASS — 0 failure(s)`). Only process substitution is newly refused, and it is
refused as a named exit 2, is used by no registered command, and is a genuine hang risk.
Not a finding — see §1.4.

**Rejected — recursion / `RecursionError`.** No recursive call exists in the instrument
(`say` → `_mute` → `open` terminates; `leave` is not re-entrant except through the handler,
and that path is MAJOR 1's `AttributeError`, already reported). I could find no input that
grows the stack; the parsers are flat loops over lines. Nothing to report.

**Rejected — making the honest control permissive.** I could not get an exit 0 out of any
corrupted input. Note the structural reason the catch-all cannot create a false PASS, which
the document states at line 853 and which I confirm: no exit 0 is reachable through `die()`,
`violation()` or the handler; they are all terminal.

**Rejected — attacking the overrule's premise (scope item 6).** I went looking for an
exit-2 refusal whose *correct* consequence differs from "not a measurement, verdict not
read, nothing in evidence about either engine", which would have made the overrule wrong.
I examined the refusal families: `bind()`'s "not about each other"; `read_report`'s
non-`nodes` budget; the duplicate-key and duplicate-record refusals; `check_coverage`'s
turn/halt refusals; `clause_b`'s three premise refusals; `cross_check`'s self-check on the
ported arithmetic; `link_1c`'s missing-field refusals; `cold_answer`'s "could not be run"
and "answered no single bestmove"; and the `BaseException` residue. **Every one licenses
the same reading: the numbers may not be read.** The closest case is "`{engine}` answered
no single bestmove", which *is* arguably evidence that a binary is broken — but it is not
evidence about the *extension*, which is what §5's row actually restricts ("never evidence
about the extension"), and it changes no conclusion this document may draw. **The
overrule's premise survives.** D-424 is right on the merits. What is wrong is that the
document did not carry the deletion out — MAJOR 2.

**Rejected — an incomplete self-account (scope item 9).** I expected a fourth instance
and did not find one; the section-digest table in §1.1 is the evidence. Recorded as
rejected.

**Rejected — `43e8a86` still standing as the pin.** It appears eight times, all in
past-tense narrative ("`43e8a86` in revisions 6 and 7", "MAJOR B is closed IN CODE by
`43e8a86`"). §7A.1 item 2, §10's table and §11's row 8 all name `385631f`. Not stale.

**Harness bug of my own, recorded for honesty.** In one attack script a heredoc quoted
with `<<"EOS"` left `>\&-` literal, so bash created a file named `&-` in the repository
root instead of closing fd 1 — which silently made two "closed stdout" cases pass. I
caught it with an explicit stray-file check, redid those cases with
`( exec 1>&-; exec python3 … )`, and removed the file (`rm -f -- '&-'`). The corrected run
is G1/G2 in MAJOR 1. This is exactly the SHELL_CHECKLIST's own subject matter, arriving in
the reviewer's tooling.

---

# 5. ANYTHING I COULD NOT VERIFY

1. **Whether `>&-` occurs in the actual launch path.** I showed the invariant is false as
   written and that the failure reaches VOIDs. I did **not** establish that the operator's
   real Step 6 invocation ever closes stdout. The invariant is stated unconditionally, and
   revision 8 deliberately removed revision 7's narrowing, so one counterexample falsifies
   it — but a reader weighing urgency should know the trigger is a non-default redirection.
   The equivalent is true of D-421 MAJOR 1's `/dev/full`, which was graded MAJOR.

2. **The governed run itself.** Nothing here says anything about the SPRT result; no
   governed sample was taken and none should be until this FAIL is closed.

3. **`docs/decisions.md` D-401.** Not read, per the standing prohibition. Where the
   document cites D-401 (the retired run, the 31.2% vacuous figure) I took the citation as
   given and checked nothing about it. If any claim in this review depends on D-401's
   content, I did not check that claim.

4. **§8's dry-run cost figures and §7's cost table.** Spot-checked as byte-unchanged only
   (§1.1). I re-executed §8.6's *checker* arms but did **not** re-run `arena --replay`, so
   the `14305 / 14341 = 0.997x` figure is unverified by me; it was verified by earlier
   reviewers and is byte-unchanged at this revision.

5. **The clause-(b) proof, §3's opening slice, and §7A.1's agreement criterion.** Fast-path
   only, per the dispatch: confirmed byte-unchanged, not re-litigated.

6. **The three untracked entries** (`.agents/`, `.claude/`, `skills-lock.json`). I
   established they appeared mid-session at `17:47` and contain agent-harness skills
   unrelated to pistol. I could not establish which process created them, and I did not
   remove them.

---

# 6. WHAT WOULD MAKE THIS PASS

Not a recommendation between design options — just the finding list, closed:

1. **MAJOR 1**: make `sys.stdout is None` a delivery failure rather than an unguarded
   attribute error, and add a genuinely-closed-descriptor leg to
   `an_answer_that_cannot_be_delivered_is_no_answer_and_not_a_finding` (a mutation that
   removes the new guard must make it fail). This moves the instrument again, and so
   re-pins §7A.1 item 2 and reopens this review a fourth time. That cost is real and is
   the operator's to weigh.
2. **MAJOR 2**: delete §7A.1 lines 845–851 and the "A NOTE ON DIRECTION" paragraph that
   depends on them.
3. **MAJOR 3**: rewrite §11's closing paragraph to say the review reopens WHOLE, matching
   the header and §11's own row 8.
4. **MINOR 1–4**: correct "~50" to 56; bring §10's digest row to "revisions 6, 7 and 8";
   drop the "what no process can catch" enumeration in §5's last row; give the delivery
   test's control assertion a message that names what the other codes would have meant.

MAJOR 2 and MAJOR 3 are document-only and do not move the instrument.
