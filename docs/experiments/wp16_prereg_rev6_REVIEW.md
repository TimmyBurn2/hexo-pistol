# REVIEW — `docs/experiments/wp16_sprt_prereg.md` revision 6 + `43e8a86` (the instrument)

**Revision reviewed**: `3a198de86447c0d349d41946bfc5a49a23e48684` (`3a198de`,
"docs(experiments,decisions): wp16_sprt_prereg.md revision 6 -- an invariant
where an enumeration was"), together with `43e8a861b3b84e6c30a1d204c3c68df375872715`
(`43e8a86`, "fix(tools): exit 1 only for named attribution findings -- guard the
empty range, catch everything else"). `e2a25c4` (`.gitignore`, +4 lines,
`/sessions/`) was glanced at and is not the subject.

**Still matches HEAD?** YES.

```
$ git rev-parse HEAD          # at the START of this review
3a198de86447c0d349d41946bfc5a49a23e48684
$ git rev-parse HEAD          # at the END of this review
3a198de86447c0d349d41946bfc5a49a23e48684
```

**`git status --porcelain`**: EMPTY at the start; EMPTY at the end. (One
intermediate breach, caused by me and repaired: a shell-escaping accident in an
`>&-` probe created a file literally named `&-` in the repository root. It was
removed the moment `git status` showed it; the porcelain output above is the
post-repair state. Recorded rather than hidden.)

**Mutation testing** was run in a SEPARATE detached worktree (`/tmp/rev6_mut` at
`3a198de`, `CARGO_TARGET_DIR=/tmp/rev6_mut_target`), never in the live tree. The
release-digest rebuild was run in a second separate worktree
(`/home/tom/rev6_check`, `CARGO_TARGET_DIR=/home/tom/rev6_build_target`). Both
worktrees were removed with `git worktree remove --force` and `git worktree
prune`; `git worktree list` at the end shows only the main tree and two
pre-existing worktrees I did not create. `target/release/pistol` and
`target/release/arena` were never rebuilt in the main tree.

**Toolchain**: every remote command ran under `bash -lc`.

```
$ which rustc && rustc --version
/home/tom/.cargo/bin/rustc
rustc 1.97.1 (8bab26f4f 2026-07-14)
```

**Reviewer**: fresh-context subagent, Claude Opus 5. **Date**: 2026-08-24.
**Standing prohibition honoured**: `docs/decisions.md` D-401 was never read,
grepped or printed. The one command that would have printed it
(`grep -n "^D-419:" -A 14`) was filtered with `grep -v "D-401"`, and D-420 was
extracted by an exact `startswith("D-420:")` line selector.

---

## VERDICT: **FAIL** — 0 BLOCKING, 1 MAJOR, 2 MINOR

**The governed run must not be launched at this revision.**

This is a close FAIL and the margin matters, so it is stated plainly: revision 6
and `43e8a86` are the strongest work this document has carried. Every one of
D-419's nine findings is genuinely closed, verified individually and by
execution rather than by reading. The instrument fix is real, reproduced,
tested, and mutation-verified — I re-ran all five mutations myself and all five
died. The two release-binary digests reproduce byte-exactly from a fresh
worktree build. The fast-path territory is byte-unchanged.

The FAIL rests on ONE MAJOR, and it is the same defect class one level down
again: **the INVARIANT that revision 6 installs in place of revision 5's
enumeration is itself false as stated, and the four checks the document
registers to make it "checkable rather than believed" cannot detect that.** I
reproduced two falsifications, one of them in the operator's default
environment, and the instrument's own comment concedes a third.

Direction, stated so the severity is not over-read: **no exit 0 is reachable
through any finding here, so no false PASS is available.** D-417's direction
still holds.

---

## What is CLEAN — stated FIRST and specifically

### C1. The mutation table: all five KILLED, re-run by me

Run in `/tmp/rev6_mut` at `3a198de`, pristine baseline first:

```
$ cd /tmp/rev6_mut && git rev-parse HEAD
3a198de86447c0d349d41946bfc5a49a23e48684
$ sha256sum tools/wp16_warm_attribution_check.py
5a6865dc7475752a466bb8b9adefa24b5289dd4517c2521819f41ea2e0b37ff4  tools/...
$ CARGO_TARGET_DIR=/tmp/rev6_mut_target cargo test -p pistol-cli \
      --test wp16_warm_attribution_check_tests
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s
```

| Mutant | What I changed | Result |
|---|---|---|
| **M1** | `clause_b`'s guard removed — lines 614-641 collapsed back to the pristine one-line `spot = next(at for at in …)` | **KILLED** |
| **M2** | `except Exception` → `except (KeyError, ValueError, IndexError)` | **KILLED** |
| **M3** | guard fires but drops `fewer than the {book}-turn book` | **KILLED** |
| **M4** | catch-all message drops `{type(why).__name__}` and `{why!r}` | **KILLED** |
| **M5** | `main()` returns `NOT_A_MEASUREMENT` unconditionally | **KILLED** |

**M1** — this is the mutation the dispatch singled out as BLOCKING if it did not
fire. It fires:

```
---- a_pair_mate_shorter_than_the_book_is_a_refusal_and_not_a_crash stdout ----
panicked at crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs:970:5:
the refusal must name the short game rather than a turn index that does not exist: exit Some(2)
stdout: warm_attribution_check: CANNOT READ: an unanticipated StopIteration escaped
this instrument: StopIteration(). …
test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

Note the informative detail: with M1 alone the run exits **2**, not 1 — the
catch-all absorbs the `StopIteration`. The test still fails, on the
name-the-short-game assertion. **The test covers the fix.** Not a blocking
finding.

**M2** — reverts the catch-all and reproduces D-419's MAJOR B exit code exactly:

```
---- an_unanticipated_exception_is_a_refusal_and_not_a_finding ----
  File "/tmp/rev6_mut/tools/wp16_warm_attribution_check.py", line 158, in recompute_verdict
    h0 = math.log(beta / (1.0 - alpha))
ZeroDivisionError: division by zero
  left: Some(1)
 right: Some(2)
test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

**The independence claim is CORRECT, and I judged it rather than accepted it.**
Under M2 the short-mate test stayed GREEN (it is inside the 12 that passed).
That is an independence result and not a coverage gap, and the reason is
demonstrable rather than asserted: M1 kills the guard limb *without* touching
the handler, and M2 kills the handler limb *without* touching the guard. Each
limb has a mutant that only it catches. A coverage gap would look like one
mutant killed by neither test, or both limbs killed only by the same test.
Neither is the case.

**M5** — the control probe:

```
test a_clean_replay_of_an_honest_report_is_attributable ... FAILED
test an_unanticipated_exception_is_a_refusal_and_not_a_finding ... FAILED
test an_inert_pair_is_excluded_by_the_theorem_and_its_cross_check_is_a_no_op ... FAILED
test a_forfeit_containing_pair_that_differs_at_a_searched_turn_is_attributed ... FAILED
test documents_that_are_not_about_each_other_are_a_void_and_not_a_finding ... FAILED
test result: FAILED. 8 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s
```

The controls are load-bearing. (The short-mate test PASSED under M5 — it has no
in-test control of its own; see "Observations" below. The document does not
claim it has one.)

### C2. The two new tests drive the SHIPPED script, and the suite is green

`check()` at `crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs:327`
invokes `repo("tools/wp16_warm_attribution_check.py")`, and `repo()` resolves
`CARGO_MANIFEST_DIR/../..` — the real file in the tree under test, not a copy.
Confirmed by the mutants: editing `tools/…py` in the worktree changed the test
outcomes, which is only possible if the shipped file is what runs.

```
$ cargo test -p pistol-cli --test wp16_warm_attribution_check_tests
running 13 tests
…
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s
```

### C3. Workspace gates green at `3a198de`

```
targets: 139 TOTAL passed: 742 failed: 0 ignored: 9
cargo exit status: 0
```

(computed by re-parsing every `test result:` line of `cargo test --workspace
--locked`, not by trusting the wrapper's status — the count matches `43e8a86`'s
commit message and D-420 exactly.)

```
$ cargo clippy --workspace --all-targets --locked -- -D clippy::all
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
clippy_rc=0
$ cargo fmt --all --check
fmt_rc=0
```

### C4. The handler's boundary is the right one, and `SystemExit` genuinely passes through

`except Exception` (`:875`) and not `BaseException`. `SystemExit` is a
`BaseException`, so `die()` → 2 and `violation()` → 3 travel through untouched.
Verified by execution, not by reading:

- exit 2 through the handler's own `die()`:
  `an_unanticipated_exception_is_a_refusal_and_not_a_finding` asserts `Some(2)`
  and passes;
- exit 2 through `main()`'s own `die()`: the usage refusal below exits 2;
- exit 3 survives: the three determinism tests assert `Some(3)` (`:619`, `:902`)
  and all pass.

If `Exception` had accidentally swallowed `SystemExit`, exit 3 would have become
exit 2 and those tests would be red. They are green.

### C5. §7A.1's four registered checks are each TRUE as stated

```
$ grep -n "raise SystemExit" tools/wp16_warm_attribution_check.py
169:    raise SystemExit(NO_ANSWER)
182:    raise SystemExit(DETERMINISM_VIOLATION)
874:        raise SystemExit(main())
$ grep -n "NOT_A_MEASUREMENT" tools/wp16_warm_attribution_check.py
133:NOT_A_MEASUREMENT = 1
838:    return ATTRIBUTABLE if not failures else NOT_A_MEASUREMENT
846:    #   NOT_A_MEASUREMENT`, so exit 1 means the `failures` list is non-empty and
$ grep -n "except " tools/wp16_warm_attribution_check.py
215:    except OSError as why:        -> die()
235:    except OSError as reason:     -> die()
252:    except UnicodeDecodeError:    -> die()
332:    except UnicodeDecodeError:    -> die()
875:    except Exception as why:      -> die()   [the boundary]
```

Three `raise SystemExit` sites, exactly one `NOT_A_MEASUREMENT` *use* site (`:838`
— `:846` is the comment), all four inner `except` clauses routing to `die()`, and
the boundary is a catch-all. Every one of the document's four bullets is
literally true. **They are true and they are not sufficient — see MAJOR 1.**

### C6. Every quoted instrument string is CHARACTER-EXACT, including the new fourth

Mechanical check: the instrument's string literals were extracted by `ast`,
adjacent literals and `Call`-argument runs joined, f-string placeholders and the
document's `{…}` placeholders wildcarded, backticks stripped, whitespace
normalized.

```
premise 1 opening          doc=True  source_exact=True
premise 2 seats            doc=True  source_exact=True
premise 3 differ-at-turn   doc=True  source_exact=True
premise 4 NEW short-mate   doc=True  source_exact=True
catch-all message          doc=True  source_exact=True
status word                doc=*     source_exact=True
halt invariant             doc=*     source_exact=True
cross-check self-check     doc=*     source_exact=True
CANNOT READ prefix         doc=True  source_exact=True
MISMATCHES (source side): 0
```

`*` = my hand-transcription failed a raw `in doc` because the document wraps
those three quotes across lines; a second pass against the whitespace-normalized
document returned `True` for all three. **All nine exact, both sides.** The
fourth premise message — the one revision 5's whole receipts rule turns on — is
exact, and the other three are still exact.

### C7. The pins

```
$ git diff --stat bfdf933..HEAD -- crates/pistol-arena/          # item 1
                                        # prints nothing
$ git diff --stat bfdf933..HEAD -- crates/*/src/ Cargo.toml Cargo.lock   # item 3
                                        # prints nothing
$ sha256sum target/release/pistol target/release/arena
b8d0dc963a2453e1eff69823629c37b23bafe419b9225f8af2401df519bc2673  target/release/pistol
3ba8de615d4d708793d72c2f3c2f6c649811996bb331527e64d0f612a13aebc2  target/release/arena
```

Both match §7A.1 item 3 exactly. **And they REPRODUCE** — built fresh in a
separate worktree with a separate target dir under the login-shell toolchain:

```
$ git worktree add -q --detach /home/tom/rev6_check 3a198de
$ cd /home/tom/rev6_check && which rustc
/home/tom/.cargo/bin/rustc
$ CARGO_TARGET_DIR=/home/tom/rev6_build_target cargo build --release --locked \
      --bin arena --bin pistol
    Finished `release` profile [optimized] target(s) in 5.60s
$ sha256sum /home/tom/rev6_build_target/release/{pistol,arena}
b8d0dc963a2453e1eff69823629c37b23bafe419b9225f8af2401df519bc2673  …/pistol
3ba8de615d4d708793d72c2f3c2f6c649811996bb331527e64d0f612a13aebc2  …/arena
```

Item 2's move to `43e8a86` is right, and §10's table agrees with §7A.1 item 2.

**The retired DOCUMENT-ONLY sentence is genuinely gone**, and the historical
instance is rewritten against `de53f5d` as claimed:

```
$ grep -n "DOCUMENT-ONLY" docs/experiments/wp16_sprt_prereg.md
8:   **REVISION 6 IS NOT DOCUMENT-ONLY, AND THAT IS ITS HEADLINE.**
15:  … is RETIRED rather than quietly left standing where it has become false.
547: … Revisions 3, 4 and 5 each said "Commits after `bfdf933` … DOCUMENT-ONLY …"
1286: … the "commits after `bfdf933` are DOCUMENT-ONLY" sentence is RETIRED as false
```

Line 65-70 carries the rewritten historical form: `git diff --stat
bfdf933..de53f5d -- crates/ tools/`, with the parenthetical explaining why the
`HEAD` form was abandoned. No live instance of the false claim survives.

### C8. FAST PATH — verified BYTE-UNCHANGED, not skimmed

Rather than read hunk headers, I split both revisions of the document by heading
and compared per-section `sha256`:

```
CHANGED   HEADER-PREAMBLE
same      1. What is being judged, and what is not
same      2. The hypothesis and the verdict unit
same      3. The instrument
same      4. What the run reports, and which lines are read
CHANGED   5. Outcome handling, written before game one
same      6. The honest expectation, and what a negative result means
CHANGED   7. Costs
same      7A. The doubts, their instruments …
CHANGED   7A.1 DOUBT 1 …
CHANGED     What the instrument refuses BEFORE it applies the criterion at all
same        What the instrument checks about the REPLAY DOCUMENT itself
CHANGED     Clause (b)'s satisfaction condition — EXHAUSTIVE over four cases
CHANGED     The inert cross-check — restored, with its consequence registered
same      7A.2 DOUBT 2 …
same      8. The dry run   (and 8.1, 8.2, 8.3, Criterion 1'/2/3, 8.4, 8.5, 8.6 — all same)
same      9. FILL-IN slots
CHANGED   10. What flips this document
CHANGED   11. REVIEW STATE
```

**§8.6's re-execution, the clause-(b) proof section, the agreement criterion, §3's
fresh opening slice, and §8's twelve pre-existing quoted strings are all in
`same` sections and are byte-identical.** The diff does not disturb the fast
path. This also discharges MINOR G: the header's claim ("HEADER, §5, §7, §7A.1,
§10 and §11 — and nothing else") is COMPLETE and TRUE against this table.

### C9. The seven MINORs, each verified individually

| | Finding | Verdict | Evidence |
|---|---|---|---|
| **C** | §7 said "three times", §8.6 said FOUR | **CLOSED** | §7:419 now reads `FOUR times: 0.997x, 1.003x, 0.994x and 1.003x`; §8.6:1181-1192 lists exactly those four (`0.997x` from the artefacts, `1.003x` pre-revision, `0.994x` D-413's reviewer, `1.003x` again). Same four, same order-set |
| **D** | Step 6 / Step 7 self-contradiction | **CLOSED** | §7:423 says `Step 6` in the column and quotes §7A.2's own words back; §7A.2:905 (untouched) says `TO BE RUN AT STEP 6`. Three statements, one step number |
| **E** | `~2 min` estimate ~8x the document's own MEASURED figures | **CLOSED** | §7:423 replaces it with `≤14 s`, marked **DERIVED FROM MEASURED**. Arithmetic checked: `2 × 24 × 0.291 = 13.968 ≤ 14`. §9.5:1233-1236 (untouched) is the source and does say "MEASURED worst single search (291 ms) … over two independent 24-position sweeps". Honest upper bound: it charges every position the worst search's time, and charges the cheaper plain-staged seat at the `defensive_only` seat's rate |
| **F** | `wp15b_attribution_check.py` pinned twice | **CLOSED** | §10:1261 now records BOTH pins with the identical-content proof. Proof verified: `git diff --stat bfdf933..HEAD -- tools/wp15b_attribution_check.py` prints nothing; `git diff --stat 8ca4063..bfdf933 -- tools/wp15b_attribution_check.py` prints nothing; `git log -1 -- tools/wp15b_attribution_check.py` → `a80a864` |
| **G** | header's revision account incomplete | **CLOSED** | See C8 — verified against per-section digests, not against the hunk list |
| **H** | inert row 1 over-claimed the replay | **CLOSED** | §7A.1:675 now grounds row 1 on the theorem alone ("identical move lists force a 1-1 split whatever the replay saw"); :686-695 records the correction and D-419's reproducer. The over-claiming clause is gone |
| **I** | three unreceipted claims | **CLOSED** | §7A.1:476-498 supplies them. All three verified to exist: `crates/pistol-arena/tests/replay_tests.rs:316` (`the_replay_path_sends_newgame_on_every_fresh_spawn_too`), `crates/pistol-arena/tests/seat_setup_identity_tests.rs:232` (`every_fresh_spawn_is_sent_newgame_before_it_is_given_a_position`), `crates/pistol-arena/tests/replay_chain_tests.rs` (exists, 7638 bytes). The second-instrument receipt checks out: `grep -c "subprocess.run" tools/wp15b_attribution_check.py` → `1` |

### C10. MAJOR B is genuinely CLOSED IN CODE, and the receipt is now TRUE

I verified the receipt "Each prints under `CANNOT READ:` and exits **2**" for all
four premise refusals **by testing, not by reading**:

- premises 1-3: `a_pair_that_does_not_satisfy_the_proofs_premise_is_a_void_and_not_an_attribution`
  seeds all three and asserts `Some(2)` on each — PASSES;
- premise 4: `a_pair_mate_shorter_than_the_book_is_a_refusal_and_not_a_crash`
  asserts `Some(2)`, the `warm_attribution_check: CANNOT READ:` prefix, the
  words `fewer than the 2-turn book`, and NO `StopIteration` on stderr — PASSES;
- and M1 proves that fourth assertion is load-bearing rather than vacuous.

### C11. §5 and §7A.1 say the SAME thing about exit 2

§5:362 kind (i) — "a missing or unrunnable engine, an unreadable or non-UTF-8
document, an incomplete or abandoned replay pass, or a budget this cannot
replay". §7A.1:778 kind (i) — the same four, same order. §5:363 kind (ii) —
"EVERY OTHER exit 2, by rule and not by list"; §7A.1:778 kind (ii) — "every
other exit 2, named or not". No contradiction. (They also share MAJOR 1's and
MINOR 2's defects identically, which is consistent rather than contradictory.)

---

## Findings

### MAJOR 1 — the registered INVARIANT is FALSE as stated, and its four registered checks cannot detect that

§7A.1:733-761 installs, as the thing that replaces revision 5's enumeration:

> **EXIT 1 ARISES ONLY FROM THE NAMED ATTRIBUTION FINDINGS. Every other
> termination of the instrument is exit 0, exit 2, or exit 3.**

and then says: "**IT IS CHECKABLE, AND THESE ARE THE CHECKS**, so a reviewer
confirms it rather than believing it", listing four. `43e8a86`'s commit message
calls the invariant "true by construction"; D-420 calls it "Checkable and
checked, not asserted".

**It is not true, and the four checks are not sufficient for it.** The gap is
that all four checks look at *which exception classes reach the handler* and
none looks at *what happens when the handler's own reporting fails*, or at
*terminations that do not pass through `SystemExit` at all*.

**(a) Reproduced in the operator's DEFAULT environment — a termination that is
none of 0, 2, 3.** `PYTHONUNBUFFERED` is unset on this machine (verified).

```
$ echo "PYTHONUNBUFFERED=[${PYTHONUNBUFFERED:-unset}]"
PYTHONUNBUFFERED=[unset]

$ python3 tools/wp16_warm_attribution_check.py > /dev/full ; echo "exit=$?"
Exception ignored while flushing sys.stdout:
OSError: [Errno 28] No space left on device
exit=120

$ python3 tools/wp16_warm_attribution_check.py | head -0 ; echo "exit=${PIPESTATUS[0]}"
Exception ignored while flushing sys.stdout:
BrokenPipeError: [Errno 32] Broken pipe
exit=120
```

Exit **120** is not 0, not 1, not 2 and not 3. The invariant's second sentence is
false, one command, no special environment. §5's table and §7A.1's exit table
have no row for it, so a reader who sees 120 has no registered reading at all —
which is the condition MAJOR A existed to abolish.

**(b) Reproduced — exit 1 reached with NO named attribution finding.** With
stdout unbuffered (`PYTHONUNBUFFERED=1`, or `python3 -u`), the write failure
surfaces *inside* `die()`, propagates to `except Exception`, whose handler calls
`die()` again, whose `print` fails again — and **that second exception is
uncaught**:

```
$ PYTHONUNBUFFERED=1 python3 tools/wp16_warm_attribution_check.py > /dev/full ; echo "exit=$?"
Traceback (most recent call last):
  File ".../tools/wp16_warm_attribution_check.py", line 874, in <module>
    raise SystemExit(main())
  File ".../tools/wp16_warm_attribution_check.py", line 793, in main
    die("usage: wp16_warm_attribution_check.py <report> <replay> <engine-binary>")
  File ".../tools/wp16_warm_attribution_check.py", line 168, in die
    print(f"warm_attribution_check: CANNOT READ: {why}")
OSError: [Errno 28] No space left on device

During handling of the above exception, another exception occurred:

Traceback (most recent call last):
  File ".../tools/wp16_warm_attribution_check.py", line 876, in <module>
    die(
  File ".../tools/wp16_warm_attribution_check.py", line 168, in die
    print(f"warm_attribution_check: CANNOT READ: {why}")
OSError: [Errno 28] No space left on device
exit=1
```

and identically with a closed pipe:

```
$ PYTHONUNBUFFERED=1 python3 tools/wp16_warm_attribution_check.py 2>/tmp/e.err | head -0
$ echo "exit=${PIPESTATUS[0]}"
exit=1
        # /tmp/e.err: BrokenPipeError … During handling of the above exception,
        # another exception occurred … BrokenPipeError
```

**Exit 1, a traceback, and no `CANNOT READ:` line** — which is, verbatim, the
signature D-419's MAJOR B was written about and the signature `43e8a86` exists
to abolish. The document registers exit 1 as "THE RUN IS NOT A MEASUREMENT …
a confirmed inversion; an unattributable pair; …", i.e. a finding about the
engines. The handler has no protection against its own reporting failing, and
the four registered checks cannot see this because they only enumerate what
reaches the handler, never what leaves it.

**(c) The instrument's own comment concedes a third case the document does not.**
`tools/wp16_warm_attribution_check.py:870-872`:

> A `KeyboardInterrupt` is not an answer this file computed and dies by its
> signal rather than by exit 1, so it does not breach the invariant either.

A signal death is not exit 0, 2 or 3. The instrument's comment carves it out;
the document states the invariant with no carve-out at all. The two accounts do
not match, which is scope item 9's question answered in the negative.

**Direction and reachability, so this is not over-read.** No exit 0 is reachable
through any of (a), (b) or (c) — **no false PASS**. (b) requires a stdout write
failure AND unbuffered stdout; (a) requires only a stdout write failure. §8.2's
registered commands do not pipe the checker's output, so the governed run does
not obviously trip either. This is MAJOR and not BLOCKING for that reason. It is
MAJOR and not MINOR because the invariant is the entire load-bearing content of
revision 6's MAJOR-A fix, it is asserted unconditionally and as "checkable", and
one of its two falsifications is exactly the defect class the commit was written
to close, surviving inside the fix.

**What would close it.** Either (i) narrow the invariant to what is actually
true and provable — e.g. "no exception raised by the instrument's own analysis
can reach exit 1; a failure of the instrument's *output channel* is outside the
invariant and terminates on CPython's own status", and give §5 a row for it; or
(ii) make it true — wrap the handler's `die()` so a second reporting failure
still lands on a registered code, and add the check that would have caught this
("no `print` outside a path that can survive its own failure") to the four. A
registered check list that cannot falsify the registered invariant is, by this
document's own standard, "a criterion nothing can fail".

**SHELL_CHECKLIST item this lands under**: item 12, obligation 1 ("a code per
kind" — 120 and a signal death have no kind).

---

### MINOR 2 — kind (i) is NOT closed: a reachable exit-2 VOID that is none of the four named categories

§5:362 says of kind (i): "These four categories are enumerable and **this is the
enumeration**". §7A.1:778 says: "kind (i) is closed above, so an exit 2 that is
not one of those four **IS kind (ii) by rule**."

**Minimal reproducer** — `main()`:793, reachable on any wrong argument count:

```
$ python3 tools/wp16_warm_attribution_check.py ; echo "exit=$?"
warm_attribution_check: CANNOT READ: usage: wp16_warm_attribution_check.py <report> <replay> <engine-binary>
exit=2
```

This is a **VOID**: the void is fixed (re-issue the command) and the answer is
re-taken. It is none of the four — not "a missing or unrunnable engine" (no
engine was named or run), not "an unreadable or non-UTF-8 document" (no document
was opened), not "an incomplete or abandoned replay pass", not "a budget this
cannot replay". By the document's own rule it is therefore kind (ii), whose
registered reading is:

> **NOTHING IS FIXED AND NOTHING IS RE-TAKEN.** The report is not one the arena
> could have written. The run is not a measurement, and what is investigated is
> the REPORT's provenance — never the engines.

That reading is wrong for a mistyped command line. Kind (i) is not closed, so
the rule that replaces the enumeration does not partition the whole space after
all.

**A second, larger instance argued but NOT reproduced** (recorded as unverified
rather than claimed): §7A.1:778 explicitly assigns the handler's own message
`an unanticipated {Class} escaped this instrument` to kind (ii). But `43e8a86`
made that handler a catch-all, so it now absorbs *environmental* failures too —
`MemoryError`, or the `UnicodeDecodeError` that `subprocess.run(..., text=True)`
raises if the probed engine writes invalid UTF-8. Those are VOIDs about the
machine or the binary, and routing them to "investigate the report's provenance,
never the engines" is `SHELL_CHECKLIST` item 12's own failure mode with the sign
flipped. I did not reproduce either: `MemoryError` would require exhausting a
24 GiB tmpfs the checklist itself records as a past hazard, and the
`UnicodeDecodeError` route needs a divergence fixture plus a shim emitting
invalid UTF-8, which I judged not worth the machine time for a MINOR.

**Why MINOR and not MAJOR, stated so the grading can be attacked.** D-419's
MAJOR A had two limbs. The dangerous one — a report-internal contradiction
landing on "fix the void and re-take", inviting a governed run to be re-taken on
a report the arena could not have written — is **genuinely closed**; I looked
for a survivor of that limb and found none. What remains is the *opposite*
direction: a void landing on the over-strict reading. Over-strict cannot license
a bad re-take and cannot reach exit 0. That is a real difference in consequence
and it is why I did not grade this MAJOR.

**What would close it**: add the fifth category ("an invocation this tool cannot
act on") to kind (i), or state kind (i) as a *rule* too — e.g. "kind (i) is any
exit 2 in which no `game`/`replay`/`pair` record was read" — rather than as a
list of four.

---

### MINOR 3 — a stale, unmarked number in new revision-6 text

§7A.1:741 (inside the new hunk):

> That promise was an ENUMERATION — **the instrument has 49 `die()` call sites**
> and §7A.1 quoted 7 — and an enumeration is only as good as the imagination of
> whoever wrote it.

Present tense, at a governing revision where it is **50**:

```
$ grep -o "die(" tools/wp16_warm_attribution_check.py | wc -l    # at 43e8a86
52
$ git show bfdf933:tools/wp16_warm_attribution_check.py | grep -o "die(" | wc -l
50
```

52 occurrences minus `def die(why):` (`:167`) minus one *comment* mention
(`:867`, "`SystemExit` is what `die()`, `violation()` …") = **50 call sites**;
at `bfdf933` it was 50 − 1 = 49, which is what D-419 measured. `43e8a86` adds
the guard's `die()`, so the number moved and the sentence did not.

The header's line 38 states the same fact in the **past** tense ("while the
instrument **had** 49 `die()` sites") and is therefore correct; only §7A.1's
present-tense instance is wrong. The number also carries no MEASURED / ESTIMATED
/ DERIVED marking, against CLAUDE.md's numeric rule.

**SHELL_CHECKLIST item this lands under**: item 3's second half — "a substring is
not a token". `grep -c "die("` counts the definition and, now, a comment. The
document's own check list has the same shape of exposure and survives it only
because a human filters: `grep -n "NOT_A_MEASUREMENT"` returns three lines and
the document correctly describes two of them.

---

## `tools/SHELL_CHECKLIST.md` — answered ITEM BY NAME against `43e8a86`'s diff

The diff is Python plus a Rust test file, so several items have no shell
construct to bite. Each is answered anyway, because the checklist binds "a change
under `tools/`", not "a change to a shell script".

**1. A command substitution whose status is DISCARDED.** N/A to the diff — it
adds no command substitution and no subshell. The Python analogue (a subprocess
whose return status is dropped into a string) does not appear: the file's only
`subprocess.run` is at `:229`, pre-existing, and its result is bound and its
`stdout` shape-checked at `:237-242` before use. **Not breached.**

**2. A pipeline in a `then` body is not a pipeline in a condition.** N/A — no
shell control flow in the diff. **Not breached.**

**3. `grep` under `pipefail`.** N/A to the diff's code, **but it binds the
document**, which registers four `grep` commands as the invariant's proof
(§7A.1:748-755) and quotes a `grep`-derived count at :741. The "a substring is
not a token" half is **BREACHED at :741** — this is MINOR 3. The first half
(`grep` exiting 1 on no match) is not in play: the registered greps are
reviewer-run, not scripted, and each is expected to match.

**4. `LC_ALL`, and which direction it moves a guard.** N/A to the diff — it adds
no character class and no locale-sensitive comparison. Noted in passing that the
file's pre-existing design already reasons in this direction (`:290-293`
deliberately uses `split("\n")` rather than `splitlines()` so that its notion of
"line" is the *narrower* one that `re.M` shares). Undisturbed. **Not breached.**

**5. The index is what commits; the working tree is not.** N/A — nothing in the
diff reads git objects or enumerates tracked paths. **Not breached.**

**6. A sweep by prefix must own the prefix.** **APPLIES, and is discharged.** The
two new tests create scratch directories via `scratch("wp16warm-shortmate")` and
`scratch("wp16warm-catchall")` (`:928`, `:993`), and `common::scratch` prefixes
with `SCRATCH_PREFIX = "pistol-testscratch-"`
(`crates/pistol-cli/tests/common/mod.rs:48`) — the owned prefix, not
`pistol-`. **Not breached.**

**7. Traps.** N/A — no `trap`, no EXIT handler in the diff. The Python analogue
(a `finally` whose last statement decides the status) does not appear. **Not
breached.**

**8. One spelling per number, one refusal per reason.** **APPLIES to the new
guard, and is discharged — by proof, since a wrong diagnosis here is exactly the
item's subject.** The guard computes
`shorter, longer = sorted((len(one), len(two)))` and
`short = first if len(one) < len(two) else second` (`:630-632`). If
`len(one) == len(two)` could reach it, `short` would silently be `second` and
`shorter == longer` — a refusal naming the wrong game. It cannot: with
`len(one) == len(two) == L`, the slices `one[:book]` and `two[:book]` both have
length `min(L, book)`, so they can be unequal only by differing at some index
`i < min(L, book)`, and `range(min(L, L, book))` is exactly `range(min(L, book))`
— so `spot` is found and the guard is not reached. The guard is entered only on
strictly unequal lengths. **Not breached.**
The item's other half — "`command -v` … ACCEPTS a FIFO that then blocks every
read" — is discussed under Observations; it is pre-existing and outside the diff.

**9. What reaches a record is caller-controlled.** **APPLIES, and is
discharged.** The new refusal interpolates `{pair}`, `{first['game']}`,
`{second['game']}`, `{short['game']}`, `{other['game']}`, `{shorter}`,
`{longer}` and `{book}` into a printed line. `game` values come from `fields()`
(`:191-198`), which builds them by `line.split()` — so a field value cannot
contain a newline, and no line can be injected into the instrument's stdout
through them. `{book}` is `int(...)`-parsed. **Not breached.** (I did drive a
537 KB single refusal line through the pre-existing `fields()` duplicate-key
refusal at `:196`, which echoes a whole record; that record itself came from
`text.split("\n")` and so contains no newline. No injection.)

**10. THE COVERAGE RULE — the binding one.** **APPLIES, and is DISCHARGED, and
this is the item I spent the most effort on.** Both new behaviours carry tests in
a suite CI runs (`cargo test --workspace` builds and runs this target — it is
among the 139 targets, 742 passing), driving the **SHIPPED** script via
`repo("tools/wp16_warm_attribution_check.py")` rather than a copy, in scratch
directories, with a control:
- `an_unanticipated_exception_is_a_refusal_and_not_a_finding` carries its **own**
  in-test control (`:1075-1088`) — the same fixture with `alpha` untouched,
  asserted `Some(0)`;
- `a_pair_mate_shorter_than_the_book_is_a_refusal_and_not_a_crash` has no in-test
  control; its control is the suite-level
  `a_clean_replay_of_an_honest_report_is_attributable`.
M5 proves both controls are load-bearing: under a checker that refuses
everything, five tests including both controls go red. **Not breached.**

**11. A caller's path that feeds a delete or an overwrite is
containment-guarded.** **APPLIES; discharged BY ENUMERATION, not by memory**, as
the item demands. Every destructive or writing site in the instrument was
enumerated:

```
$ grep -n "open(" tools/wp16_warm_attribution_check.py
213:        with open(path, "rb") as handle:
$ grep -c "shutil" tools/wp16_warm_attribution_check.py   ;  0
$ grep -c "os.remove" tools/wp16_warm_attribution_check.py ;  0
```

**Exactly one `open()`, mode `"rb"`, read-only. Zero deletes, zero renames, zero
writes.** The instrument's only outputs are `print` to stdout and its exit code.
There is nothing for absolute-value escape to escape into. **Not breached, and
the diff does not add a site.**

**12. A gate distinguishes RUN VOID from FAIL, by name.** **APPLIES — this item
IS the commit's subject, and it is where both of my findings land.**
- *Obligation 1, a code per kind*: the file declares `ATTRIBUTABLE = 0`,
  `NOT_A_MEASUREMENT = 1`, `NO_ANSWER = 2`, `DETERMINISM_VIOLATION = 3`
  (`:132-135`) and the whole commit is about routing voids to 2 rather than 1 —
  a real and correct improvement. **But BREACHED in two ways**: exit **120** and
  a signal death have no kind (MAJOR 1), and a genuine void — the usage refusal,
  and the catch-all's environmental residue — is registered as kind (ii),
  "investigate the report's provenance, never the engines" (MINOR 2), which is
  this item's failure mode with the sign flipped.
- *Obligation 2, PREFLIGHT what the run needs and void early*: **not discharged.**
  There is no preflight of any kind. The demonstration is under Observations: the
  instrument accepts a FIFO as its report path and blocks forever in `slurp`.
  Pre-existing, not introduced by the diff.
- *Obligation 3, the distinction survives the seam*: **DISCHARGED, and well.**
  Both new tests assert on the code they expect AND say in the failure message
  what the other code would have meant — "Exit 1 would read as a finding about
  the engines, which is exactly what the uncaught StopIteration produced"
  (`:945-948`) and "an exception this file never anticipated is a VOID … Exit 1
  would be that void read as an attribution failure" (`:1053-1057`). This is
  exactly what the item asks for and it is done better here than the item's own
  example.

---

## Attacks ATTEMPTED and REJECTED

- **Does removing `clause_b`'s guard leave the short-mate test green?** No — M1
  fails it. The dispatch's BLOCKING condition does not fire. **Rejected.**
- **Is M2's green short-mate test a coverage gap dressed up as independence?**
  No. M1 kills the guard limb without touching the handler; M2 kills the handler
  limb without touching the guard. Each limb has a mutant only it catches, which
  is what independence means and what a gap would contradict. **Rejected.**
- **Does `except Exception` swallow `SystemExit` and collapse exits 2 and 3 into
  2?** No — `SystemExit` is a `BaseException`. Verified by execution: the usage
  refusal exits 2, the catch-all test exits 2, and all three determinism tests
  assert `Some(3)` and pass. **Rejected.**
- **Is the exit-1 row's named list ("a confirmed inversion; an unattributable
  pair; an inert pair whose bucket contradicts the theorem; a cross-check that
  moves the verdict; a broken link 1b or 1c") short of what actually appends to
  `failures`?** I enumerated every `failures.append`/`extend` site: `(a)`
  inversions (`:818`), `(b)` inert-bucket (`:653`), `(b)` unattributable
  (`:673`, `:681`), cross-check verdict move (`:727`), `link_1b` (`:743`),
  `link_1b` VACUOUS (`:749`), `link_1c` (`:766`, `:774`, `:784`). The only
  candidate for a gap is `"1b is VACUOUS on this input"`, which is not obviously
  "a broken link 1b" — but §7A.1:496-498 defines that phrase by receipt as
  "`link_1b`/`link_1c` appending to `failures`", which covers it. **Rejected.**
- **Is `NOT_A_MEASUREMENT` reachable from anywhere but `main()`'s return?** No —
  one use site, verified by grep and by reading. **Rejected.**
- **Can the new guard misname which game is short (equal lengths)?** No — proved
  unreachable; see SHELL_CHECKLIST item 8 above. **Rejected.**
- **Can a caller-controlled `game` label inject a line into the instrument's
  stdout through the new refusal?** No — `fields()` splits on whitespace.
  Attempted with a 537 806-byte crafted record; the refusal printed as one line.
  **Rejected.**
- **Is §7A.1 item 3's pathspec (`crates/*/src/ Cargo.toml Cargo.lock`) too narrow
  to prove the binaries unmoved?** In general yes — it misses per-crate
  `Cargo.toml`, `build.rs`, and non-`src` includes, which is CLAUDE.md's own
  "a build script READ" hazard. Verified INAPPLICABLE here:
  `find crates -name build.rs` → nothing; `grep -rn "include_str!|include_bytes!|include!" crates/*/src/`
  → nothing; `git diff --stat bfdf933..HEAD --` each crate's `Cargo.toml` →
  nothing; and `git diff --stat bfdf933..HEAD -- crates/` shows the ONLY change
  is the test file. The `sha256sum` check registered alongside is the stronger
  one and it reproduces from a clean build. **Rejected as a finding**, recorded
  as a latent narrowness in the command.
- **Does the diff disturb the fast-path territory?** No — verified by per-section
  digest, not by hunk-reading. **Rejected.**
- **Do §5 and §7A.1 contradict each other on the exit-2 split, as an earlier
  revision did?** No — same four categories, same rule, same consequences.
  **Rejected.**
- **Is the `≤14 s` derivation wrong or dishonestly marked?** No —
  `2 × 24 × 0.291 = 13.968`, sourced to §9.5's own MEASURED figures, marked
  **DERIVED FROM MEASURED**, and conservative in the disclosed direction.
  **Rejected.**
- **Did the digests move, or were they built with the wrong toolchain?** No —
  reproduced byte-exactly from a fresh worktree under `~/.cargo/bin/rustc`.
  **Rejected.**
- **Can a large refusal message force the write failure into `die()` without
  `PYTHONUNBUFFERED`?** ATTEMPTED and could not: I drove 12 421-byte and
  537 806-byte refusals into `/dev/full` and into a closed pipe, and both still
  surfaced at interpreter shutdown as exit 120, not inside `die()`. So MAJOR 1's
  limb (b) stands only under unbuffered stdout, and I say so. **Partially
  rejected** — limb (a) survives in the default environment.

---

## Anything I could not verify

Recorded rather than assumed.

1. **A `KeyboardInterrupt` termination's exit status, driven through the shipped
   script.** ATTEMPTED: `mkfifo`, pass the FIFO as the report path so `slurp`
   blocks, then `kill -INT`. **The process did not die** — SIGINT did not
   interrupt `open()` on the FIFO and I had to `kill -9` it. So MAJOR 1's limb
   (c) rests on the instrument's own comment (`:870-872`) and on CPython
   semantics, not on a reproducer of mine. The comment is the document's own
   evidence, so the *contradiction* between comment and document is established;
   the exact status is not.
2. **The catch-all's environmental residue** (`MemoryError`, a
   `UnicodeDecodeError` out of `subprocess.run(text=True)`). Argued in MINOR 2,
   **not reproduced**, for the reasons stated there.
3. **§8.6's `0.994x` and `1.003x` samples**, taken on other machines' state.
   Inherited unresolved from D-416/D-419; §8.6 is byte-unchanged by this
   revision and is fast-path territory.
4. **§7's governed-run ESTIMATE** (`~2-3 core-hours, ~35-50 min wall`).
   Unverifiable before the run, correctly marked, derivation stated.
5. **§7's `MEASURED: 14.254 s` dry-run figure.** No committed artefact carries
   `14254` (rule 8). Inherited from D-419's list; §7's row is in the diff but
   that cell is not.
6. **§9.4's "no further amendment to `docs/wp16_quiescence_design.md` between
   D-394 and the run's launch revision"** — a Step-6 slot by construction.
7. **D-401's contents.** Not read, by operator instruction. Everywhere the
   document cites D-401 I took the citation on trust; I could not check those
   three or four sentences and I say so rather than implying I did.

---

## Observations — recorded for the architect, explicitly NOT graded as findings

**The instrument accepts a FIFO as its report path and blocks forever,
un-interruptibly.**

```
$ mkfifo /tmp/rev6_scratch/fifo
$ python3 tools/wp16_warm_attribution_check.py /tmp/rev6_scratch/fifo /dev/null /bin/true &
$ kill -INT $!        # no effect; the process survived and needed kill -9
```

`slurp` (`:211-216`) calls `open(path, "rb")` with no preflight on what `path`
is. This is `SHELL_CHECKLIST` item 8's named class ("ACCEPTS a FIFO that then
blocks every read") and item 12 obligation 2 (preflight and void early).

**Why this does NOT fire the CAP's stop-immediately branch.** It is not a defect
in the diff — the code is pre-existing at `bfdf933` and `43e8a86` does not touch
it. And it is not "territory the last two reviews verified clean": the revision-5
review states in terms that "`tools/SHELL_CHECKLIST.md` was NOT applied as a
review target: this revision touches no file under `tools/`". No prior review
covered this file under the checklist. It is newly-in-scope territory that no
review has cleared, not previously-cleared territory now failing. I record it and
leave the disposition to the architect.

**Second observation, minor and not a finding.** `43e8a86`'s commit message and
D-420 both say "two driving tests against the SHIPPED script, **each with a
control**". Under M5 the short-mate test PASSED, which shows it has no in-test
control; its control is the suite-level `a_clean_replay_of_an_honest_report_is_attributable`.
That reading satisfies `SHELL_CHECKLIST` item 10, which asks for "a control run"
in the suite, not in the test. **The pre-registration itself does not claim an
in-test control** (§7A.1:603-606 lists only the four assertions), so there is no
over-claim in the governing document. Recorded because the commit message's
phrasing is looser than the evidence.

---

## Which CAP branch fired

**THE "FAIL ON THE DIFF" BRANCH.** All three findings are against text or code
that `43e8a86` and `3a198de` introduced:

- MAJOR 1 — the invariant is new in revision 6 (§7A.1:733-761, inside the
  `@@ +733,41 @@` hunk) and the catch-all handler is new in `43e8a86`;
- MINOR 2 — the closed-list *rule* is new in revision 6 (§5:362-363,
  §7A.1:778);
- MINOR 3 — the sentence is new revision-6 text (§7A.1:741).

Nothing failed outside the diff, and nothing failed in territory the revision-4
or revision-5 reviewers verified clean. **The stop-immediately branch did NOT
fire.**

**Per the architect's pre-registered cap, the rule is therefore: ONE fix round,
then a re-review SCOPED TO THE FIXES, then STOP regardless of outcome.**

---

## What would unstick it

- **MAJOR 1** — either narrow the invariant to what is provable (the
  instrument's *analysis* cannot reach exit 1 unnamed; a failure of its *output
  channel* is outside it) and give §5 a row for a termination that is none of
  0/1/2/3; or harden the handler so its own reporting failure still lands on a
  registered code. Either way, add to the four registered checks one that this
  defect could have falsified — the current four cannot, and that is the part of
  the finding that is about the document rather than the code.
- **MINOR 2** — add the fifth kind-(i) category, or state kind (i) as a rule
  rather than a list of four.
- **MINOR 3** — `49` → `50`, or put the sentence in the past tense as the header
  already does, and mark it.

All three are small. None reopens the instrument's test coverage, none touches
the mutation table, and none makes a false PASS reachable.
