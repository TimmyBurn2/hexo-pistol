# SCOPED RE-REVIEW — `docs/experiments/wp16_sprt_prereg.md` revision 7

**Revision reviewed**: `1618467b57514382d041a4b46fa4bcbf565d6292` (`1618467`,
"docs(experiments,decisions): wp16_sprt_prereg.md revision 7 -- narrow the
invariant to what is provable").

**Still matches HEAD?** YES.

```
$ git rev-parse HEAD          # at the START of this review
1618467b57514382d041a4b46fa4bcbf565d6292
$ git rev-parse HEAD          # at the END of this review
1618467b57514382d041a4b46fa4bcbf565d6292
```

**`git status --porcelain`**: EMPTY at the start; EMPTY at the end except this
report, which is left UNCOMMITTED as instructed. No worktree was created, no
build was run, `target/release/pistol` and `target/release/arena` were never
rebuilt or touched.

**Toolchain**: every remote command ran under `bash -lc`.

```
$ which rustc && rustc --version
/home/tom/.cargo/bin/rustc
rustc 1.97.1 (8bab26f4f 2026-07-14)
```

**Scope**: this is the ONE scoped re-review the cap licenses, against the rev-6
report at `docs/experiments/wp16_prereg_rev6_REVIEW.md`, which was read first and
used as the specification. **Reviewer**: fresh-context subagent. **Date**:
2026-08-24.

**Standing prohibition honoured**: `docs/decisions.md` D-401 was never read,
grepped or printed. D-421 and D-422 were extracted with an exact
`str.startswith(("D-421:", "D-422:"))` line selector (`/tmp/adr.py`), which
cannot emit any other ADR line.

**Not reopened, per the dispatch**: whether MAJOR 1 should have been fixed in
code (settled by architect ruling), and the FIFO issue itself (deferred by
D-422 — its *reasoning* is judged below, the issue is not).

---

## THE SCOPING GATE — RUN FIRST, AND IT HELD

```
$ git diff --stat 43e8a86..HEAD -- crates/ tools/
                                        # prints nothing
$ git diff --stat 3a198de..1618467 -- . ':(exclude)docs/'
                                        # prints nothing
$ git diff --name-only 3a198de..1618467
docs/decisions.md
docs/experiments/wp16_prereg_rev6_REVIEW.md
docs/experiments/wp16_sprt_prereg.md
```

**No diff outside `docs/`. The scoping is VALID and the re-review proceeds
scoped.** The header's own claim that
`git diff --stat 43e8a86..HEAD -- crates/ tools/` prints nothing is TRUE,
verified above rather than taken.

---

## VERDICT: **FAIL** — 0 BLOCKING, 2 MAJOR, 2 MINOR

**The governed run must not be launched at this revision. Per the cap, this
round now STOPS regardless.**

Direction, stated so the severity is not over-read, exactly as the rev-6
reviewer did: **no finding here makes exit 0 reachable, so no false PASS is
available.** Both MAJORs are about registered *readings* and registered *claims
about a check*, not about the instrument's behaviour, which is byte-identical to
its reviewed pin and was not touched.

The FAIL is not close on MAJOR 1 and is arguable on MAJOR 2; I say so, and I
give the strongest steelman for MAJOR 2 in its own section so the architect can
overrule it with full information rather than having to reconstruct it.

---

## What is CLEAN — stated FIRST and specifically

### C1. MINOR 3 is FULLY CLOSED, and the receipt survives independent re-measurement

The document now says the instrument "**had 50 `die()` call sites at the
governing revision `43e8a86`**", past tense, MEASURED, with a receipt. I
re-measured all of it.

The instrument is byte-identical at its pin and at HEAD, so measuring at HEAD
*is* measuring at `43e8a86`:

```
$ git diff --stat 43e8a86..HEAD -- tools/wp16_warm_attribution_check.py
                                        # prints nothing
$ sha256sum tools/wp16_warm_attribution_check.py
5a6865dc7475752a466bb8b9adefa24b5289dd4517c2521819f41ea2e0b37ff4  tools/wp16_warm_attribution_check.py
```

The raw count, and the fact that `grep -c` coincides only because no line
carries two occurrences:

```
$ grep -oE '\bdie\(' tools/wp16_warm_attribution_check.py | wc -l
52
$ grep -c "die(" tools/wp16_warm_attribution_check.py
52
$ awk '{t+=gsub(/die\(/,"")} END{print t}' tools/wp16_warm_attribution_check.py
52
```

The document's own cross-check, re-run by me, and it is a genuinely
**externally derived referent** — `ast` does not share the substring-counting
defect that the subtraction exists to repair:

```
$ python3 -c '...ast.walk / isinstance(n, ast.Call) and n.func.id == "die"...'
ast.Call die() nodes : 50
ast FunctionDef die  : 1 [167]
call linenos count   : 50
867 among calls?     : False
876 among calls?     : True
167 among calls?     : False
raw grep linenos     : 52
raw minus calls      : [167, 867]
```

**`50` is right. The arithmetic `52 − 1 − 1` is right. And the two subtracted
lines are exactly the two the receipt names**, not two other lines that happen
to make the sum work — `:167` is the definition and `:867` is a prose mention,
neither is an `ast.Call`, and nothing else is excluded.

`:867` really is a prose mention inside a comment:

```
$ awk 'NR==867' tools/wp16_warm_attribution_check.py
    # `SystemExit` is what `die()`, `violation()` and `main()`'s own return travel
```

The tense fix is real too: the figure is now stated against a NAMED revision,
which is the durable form. **MINOR 3 is closed with no residue.**

### C2. The `:870-872` citation is ACCURATE

The document quotes the handler comment as: "A `KeyboardInterrupt` is not an
answer this file computed and dies by its signal rather than by exit 1, so it
does not breach the invariant either."

```
$ awk 'NR>=870 && NR<=872 {printf "%d: %s\n", NR, $0}' tools/wp16_warm_attribution_check.py
870:     # hole being closed. A `KeyboardInterrupt` is not an answer this file computed
871:     # and dies by its signal rather than by exit 1, so it does not breach the
872:     # invariant either.
```

The sentence begins mid-`:870` and ends at `:872`. The line range is correct and
the quoted words are character-exact modulo the comment markers. **Clean.**

### C3. The header's account of itself is COMPLETE AND TRUE — the third instance did NOT occur

This was a finding twice (D-416 MINOR 4, D-419 MINOR G), so I verified it by
per-section `sha256` rather than by reading hunk headers, splitting both
revisions on every `^#{2,4} ` heading:

```
section name lists identical: True
CHANGED  HEADER-PREAMBLE
same     ## 1. What is being judged, and what is not
same     ## 2. The hypothesis and the verdict unit
same     ## 3. The instrument
same     ## 4. What the run reports, and which lines are read
CHANGED  ## 5. Outcome handling, written before game one
same     ## 6. The honest expectation, and what a negative result means
same     ## 7. Costs
same     ## 7A. The doubts, their instruments, their agreement criteria …
same     ### 7A.1 DOUBT 1 — the arena between the engines and the verdict
same     #### What the instrument refuses BEFORE it applies the criterion at all
same     #### What the instrument checks about the REPLAY DOCUMENT itself
same     #### Clause (b)'s satisfaction condition — EXHAUSTIVE over four cases
CHANGED  #### The inert cross-check — restored, with its consequence registered
same     ### 7A.2 DOUBT 2 — whether the extension changes what the search completes
same     ## 8. The dry run   (and 8.1–8.6, all Criteria — all same)
same     ## 9. FILL-IN slots
same     ## 10. What flips this document
CHANGED  ## 11. REVIEW STATE
TOTAL CHANGED: 4 of 28
```

Exactly four sections moved, and they are exactly the four the header names
(the changed `#### The inert cross-check` subsection lies inside §7A.1). The
independent hunk-to-section mapping agrees:

```
$ git diff -U0 3a198de..1618467 -- docs/experiments/wp16_sprt_prereg.md | grep '^@@'
@@ -3,4 +3,6 @@        -> HEADER
@@ -8 +10,31 @@        -> HEADER
@@ -362,2 +394,2 @@    -> §5   (§5 spans old 345-373)
@@ -364,0 +397 @@      -> §5
@@ -733,2 +766,3 @@    -> §7A.1
@@ -736,2 +770,3 @@    -> §7A.1
@@ -739,5 +774,4 @@    -> §7A.1
@@ -745,2 +779,55 @@    -> §7A.1
@@ -757,5 +844,12 @@    -> §7A.1
@@ -771,2 +865,5 @@    -> §7A.1
@@ -1277 +1374 @@      -> §11  (§11 starts old 1275)
@@ -1286 +1383,2 @@    -> §11
@@ -1303,4 +1401,6 @@    -> §11
```

**"THE HEADER (this block), §5, §7A.1 and §11 — and nothing else" is accurate.**

### C4. The out-of-scope territory is BYTE-UNCHANGED — the scoping is not merely nominal

Every item the dispatch forbade me to re-litigate sits in a `same` section
above: the five-mutation table and the two binary digests and the statistics-layer
pin (`### 7A.1 DOUBT 1`, `same`), the nine quoted instrument strings
(`#### What the instrument refuses BEFORE …`, `same`), the clause-(b) proof
(`same`), §8.6's re-execution (`same`), §3's opening slice (`same`), §10's flip
table (`same`). **The diff disturbs none of it.** Since the non-docs diff is
empty, the workspace suite's inputs are bit-identical to those the rev-6 review
recorded green; I did not re-run it and I do not claim it, per scope.

### C5. The narrowed invariant's operative sentence is TRUE where it has to be

With a healthy output channel, an analysis exception cannot reach exit 1
unnamed. The route is closed structurally and I checked every limb of it:

- exit 1 has exactly one source in the analysis path — `grep` finds
  `NOT_A_MEASUREMENT` at `:133` (definition), `:838` (the single use,
  `return ATTRIBUTABLE if not failures else NOT_A_MEASUREMENT`) and `:846`
  (a comment);
- the only `raise SystemExit` sites are `:169` (`die()` → 2), `:182`
  (`violation()` → 3) and `:874` (`main()`'s return);
- the handler at `:875` catches `Exception` — a catch-all — and its body is a
  `die()` call (`:876-880`), i.e. exit 2;
- `die()` reaching exit 2 with a working channel is not argued, it is measured:

```
$ python3 tools/wp16_warm_attribution_check.py ; echo "exit=$?"
warm_attribution_check: CANNOT READ: usage: wp16_warm_attribution_check.py <report> <replay> <engine-binary>
exit=2
```

The only exceptions that bypass `except Exception` are `BaseException`s: the
intended `SystemExit`, and `KeyboardInterrupt`, which the second sentence carves
out explicitly. **I could not break the first sentence with a healthy output
channel, and I tried — see Attacks.** The carve-out for the output channel is
honest and it is the reviewer's own proposed wording.

### C6. The new §5 row exists, and its registered consequence is RIGHT

```
| **A TERMINATION THAT IS NONE OF 0, 1, 2 OR 3** — new in revision 7. The
instrument did not choose this status; CPython did, because the instrument's
OUTPUT CHANNEL failed or the process was signalled. MEASURED instances: exit
**120** with stdout on a full device, and a signal death's own status | **A VOID
ABOUT THE INVOCATION ENVIRONMENT, never about the engines and never about the
report.** … **The invocation is re-issued** … |
```

The consequence — a void about the environment, re-issue, conclude nothing —
is the correct reading for exit 120 and for a signal death, and the row's
"MEASURED instances" label is honest: I reproduced 120 myself (below). The
remedy limb the rev-6 reviewer asked for ("give §5 a row for a termination that
is none of 0/1/2/3") **landed, and landed correctly.**

### C7. The three registered invocations ARE the complete set — there is no fourth

I did not take the document's word for "all three". I enumerated every mention
of either checker in the whole file and every `python3` line in it:

```
$ grep -n "python3" docs/experiments/wp16_sprt_prereg.md
780:$ python3 tools/wp16_warm_attribution_check.py <run> <replay> target/release/pistol >/dev/full
783:$ PYTHONUNBUFFERED=1 python3 tools/wp16_warm_attribution_check.py … >/dev/full
849:  … §8.2's `python3 tools/wp15b_attribution_check.py …`          <- the check's own quote
850:  … §8.6's two `python3 tools/wp16_warm_attribution_check.py …`  <- the check's own quote
1045:python3 tools/wp15b_attribution_check.py <scratch>/wp16_dryrun_report.txt target/release/pistol
1205:    python3 tools/wp16_warm_attribution_check.py <scratch>/run.txt <scratch>/replay.txt target/release/pistol
1211:    python3 tools/wp16_warm_attribution_check.py <scratch>/swapped.txt <scratch>/swapped_replay.txt target/release/pistol
```

`:1045`, `:1205`, `:1211` are the only registered invocations, and none carries
a redirection operator. `:780` and `:783` DO redirect, and I considered them
carefully — they are the rev-6 reviewer's falsification reproducers, introduced
by the words "Its governing reviewer TESTED the claim … and MEASURED it false
two ways", not registered workload. **No fourth registered invocation exists,
and the check's factual content is TRUE.** (Its *self-description* is MAJOR 2
and its *wording* is MINOR 2; the fact it asserts is sound.)

### C8. D-422's deferral grounds all HOLD

Each ground checked rather than accepted:

- **"PRE-EXISTING at `bfdf933`"** — TRUE:
  ```
  $ git show bfdf933:tools/wp16_warm_attribution_check.py | grep -n "open(path"
  213:        with open(path, "rb") as handle:
  $ git diff bfdf933..43e8a86 -- tools/wp16_warm_attribution_check.py | grep -E "^[+-].*(def slurp|open\(path)"
                                        # prints nothing — the instrument fix never touched it
  ```
- **"outside the diff the cap licenses a fix round for"** — follows from the
  above, and revision 7's diff touches no file under `tools/` at all.
- **"does not fire the cap's stop-immediately branch … revision 5's review
  states in terms that `SHELL_CHECKLIST` was NOT applied"** — consistent with
  the rev-6 report's own Observations section, which reasons identically and
  reaches the same disposition.
- **"the registered commands pass ordinary file paths written by `arena --out`,
  never a FIFO"** — TRUE and verified at both call sites:
  ```
  1041: target/release/arena --config configs/arena_wp16_dryrun.toml --out <scratch>/wp16_dryrun_report.txt
  1045: python3 tools/wp15b_attribution_check.py <scratch>/wp16_dryrun_report.txt target/release/pistol
  1203: target/release/arena --config configs/arena_wp16_dryrun.toml --out <scratch>/run.txt
  1204: target/release/arena --replay <scratch>/run.txt --out <scratch>/replay.txt --workers 4
  1205: python3 tools/wp16_warm_attribution_check.py <scratch>/run.txt <scratch>/replay.txt target/release/pistol
  ```
  Every checker input is an `arena --out` product, i.e. a regular file the
  operator did not hand-construct.

**D-422 is accurate and its deferral is a decision, not an oversight. No
finding.**

### C9. D-421 is substantially accurate

Its account of the rev-6 review, the cap branch, the architect's ruling, the
reverted `say()`/`leave()` hardening, and MINOR 3's correction all match the
rev-6 report and the tree. Two claims inside it are carried defects rather than
separate findings, and are folded into MAJOR 1 and MAJOR 2 below rather than
counted twice.

### C10. `tools/SHELL_CHECKLIST.md` does NOT apply as a review target this round

```
$ git diff --stat 3a198de..1618467 -- tools/
                                        # prints nothing
$ git diff --name-only 3a198de..1618467
docs/decisions.md
docs/experiments/wp16_prereg_rev6_REVIEW.md
docs/experiments/wp16_sprt_prereg.md
```

Revision 7 touches no file under `tools/`, so the checklist is not a review
target and its items are not answered by name. Its *substance* still informed
two findings below, and I name the items there (item 3, item 12) because the
checklist binds the document's reasoning even when it does not bind the diff.

---

## Findings

### MAJOR 1 — MINOR 2's fix is HALF-LANDED: §5 and §7A.1 now state DIFFERENT rules, and §5's new rule contradicts its own examples

The dispatch asks two things of MINOR 2's fix: that the rule classify the
shipped instrument's refusals correctly, and that "§5 and §7A.1 state the SAME
rule". **Neither holds.** Three reproduced limbs, all in the section under
review.

#### Limb (a) — §7A.1 was never updated; it still registers the CLOSED LIST OF FOUR

§5:394 now reads:

> **Exit 2, kind (i): a VOID — defined by a RULE** — an exit 2 taken before any
> `game`, `replay` or `pair` record was read … **Examples, not the
> enumeration**: a missing or unrunnable engine, … , the instrument invoked with
> the wrong arguments

§7A.1's exit table at `:875` — the row the rev-6 review cited as `§7A.1:778` —
still reads:

```
$ sed -n '875p' docs/experiments/wp16_sprt_prereg.md
| 2 | **two kinds, partitioned by RULE rather than by list.** **(i) a VOID — the
CLOSED list**: a missing or unrunnable engine, an unreadable or non-UTF-8
document, an incomplete or abandoned replay pass, a budget this cannot replay.
… **The reader does not need to match a message against a list**: kind (i) is
closed above, so an exit 2 that is not one of those four IS kind (ii) by rule.
```

And it is **byte-identical to revision 6's**:

```
$ git show 3a198de:docs/experiments/wp16_sprt_prereg.md | sed -n '778p' > /tmp/old778.txt
$ sed -n '875p' docs/experiments/wp16_sprt_prereg.md            > /tmp/new875.txt
$ diff /tmp/old778.txt /tmp/new875.txt && echo "IDENTICAL: rev7 did NOT touch this row"
IDENTICAL: rev7 did NOT touch this row
```

So the document simultaneously registers, for the same exit code, "kind (i) is
defined by a RULE, these four are examples, not the enumeration" (§5) and "kind
(i) is the CLOSED list, an exit 2 that is not one of those four IS kind (ii)"
(§7A.1). The rev-6 review's C11 verified §5 and §7A.1 *agreed* at revision 6;
**revision 7 broke that agreement and did not notice.** §7A.1's own narrative
three lines earlier even announces the change it then fails to make: "revision 7
has now had to do it a third time, for kind (i) … an enumeration standing in for
a rule is this work package's most repeated defect."

#### Limb (b) — MINOR 2's ORIGINAL counterexample is still misclassified by §7A.1

The usage refusal is live:

```
$ python3 tools/wp16_warm_attribution_check.py ; echo "exit=$?"
warm_attribution_check: CANNOT READ: usage: wp16_warm_attribution_check.py <report> <replay> <engine-binary>
exit=2
```

Under §5: named as an example of kind (i) — void, fix and re-take. **Correct.**
Under §7A.1:875: not one of the four, therefore kind (ii) — "NOTHING IS FIXED
AND NOTHING IS RE-TAKEN … what is investigated is the report's provenance, never
the engines". **That is the exact wrong reading D-421 MINOR 2 was written
about, still standing in the section that carried it.**

#### Limb (c) — the NEW RULE misclassifies a case §5's own example list names

The rule's discriminator is *when* the refusal fires: kind (i) is "an exit 2
taken before any `game`, `replay` or `pair` record was read". §5's very first
example of kind (i) is **"a missing or unrunnable engine"**. That refusal fires
at `:236`:

```
$ awk 'NR==236' tools/wp16_warm_attribution_check.py
        die(f"`{engine}` could not be run: {reason}")
```

`:236` is inside `cold_answer` (`:219`), whose only call site is `:553`, inside
`classify` (`:530`), whose only call site is `main()`:816 — **after**
`read_report` (`:794`) and `read_replay` (`:795`):

```
$ awk 'NR>=791 && NR<=816' tools/wp16_warm_attribution_check.py
791: def main():
792:     if len(sys.argv) != 4:
793:         die("usage: …")
794:     report = read_report(sys.argv[1])
795:     replay = read_replay(sys.argv[2])
796:     engine = sys.argv[3]
797:     bind(report, replay)
…
816:     inversions = classify(report, replay, engine, notes)
$ grep -n "cold_answer" tools/wp16_warm_attribution_check.py
219:def cold_answer(engine, config, budget, prefix, why):
553:        answer = cold_answer(
```

`read_report` parses `game` records and `read_replay` parses `replay` records
(`:362  index = record["replay"]`), so both have certainly been read by `:816`.
The ordering is confirmed by execution — hand the tool a nonexistent engine and
it refuses about the *report*, proving the engine is not preflighted:

```
$ python3 tools/wp16_warm_attribution_check.py /dev/null /dev/null /nonexistent/engine
warm_attribution_check: CANNOT READ: /dev/null is not a report carrying a verdict (its first token is not arena_report)
exit=2
```

**So "a missing or unrunnable engine" — a genuine VOID, fix the path and
re-take — fires AFTER `game` and `replay` records were read, and §5's own rule
therefore assigns it kind (ii): "NOTHING IS FIXED AND NOTHING IS RE-TAKEN …
never the engines."** §5's rule and §5's first example contradict each other
inside a single table cell, and the consequence attached to the wrong side is
plainly wrong for a mistyped binary path.

This is `tools/SHELL_CHECKLIST.md` item 12 obligation 1 again — a void
registered as a refusal — and it is the same shape as MINOR 2 itself: the fix
swapped one imperfect partition for another without testing the new one against
the shipped instrument's actual call order.

**Grading.** MAJOR, not MINOR, because unlike D-421 MINOR 2 this is no longer a
single missing category on an otherwise coherent partition: it is two mutually
contradictory registered readings of the *same observed exit code* in the *same
document*, which is the D-416/D-419 MAJOR-A class exactly, and because the
dispatch's explicit acceptance test for this fix ("Check §5 and §7A.1 state the
SAME rule") fails outright. It reaches no exit 0 and licenses no bad re-take of
a governed run — kind (ii) is the over-strict side — so it is not BLOCKING.

**What would close it**: bring §7A.1:875 into line with §5:394 (the same rule,
the same examples), and fix the rule's discriminator so a void detected late
stays a void — e.g. partition on *what the refusal is about* (the invocation and
its environment vs. a fact about the two documents' content) rather than on
*when it fired*. The "about" formulation is already half-present in §5's kind
(ii) ("a refusal that names a FACT ABOUT THE CONTENT") and classifies all four
of my cases correctly; it is the "before/after a record was read" clause that
mis-sorts.

---

### MAJOR 2 — the FIFTH registered check does not meet the criterion it claims to meet: the defect class PRESERVES it

§7A.1 introduces the fifth check under this heading:

> **THE REGISTERED CHECKS — FIVE, AND THE FIFTH IS ONE THIS DEFECT CLASS COULD
> HAVE FALSIFIED.** Revision 6's four could not: every one of them inspects the
> ANALYSIS path, and the defect was in the OUTPUT path, so four green checks and
> a false invariant were perfectly consistent.

D-421 repeats it: "**A FIFTH REGISTERED CHECK IS ADDED, CHOSEN BECAUSE THIS
DEFECT CLASS COULD FALSIFY IT**".

CLAUDE.md's operationalisation is the test: "A criterion that is a property the
named defect class PRESERVES … passes vacuously and is not a criterion; it must
be one that defect could falsify." Applied to check 5 the answer is decidable,
and it is decidable by execution, because the instrument is byte-identical at
revision 6 and revision 7:

```
$ git diff --stat 3a198de..HEAD -- tools/
                                        # prints nothing — identical
```

**Step 1 — the defect was live at revision 6.** Reproduced here at HEAD, which
is the same code:

```
$ python3 tools/wp16_warm_attribution_check.py > /dev/full ; echo "exit=$?"
Exception ignored while flushing sys.stdout:
OSError: [Errno 28] No space left on device
default-buffered exit=120

$ PYTHONUNBUFFERED=1 python3 tools/wp16_warm_attribution_check.py > /dev/full ; echo "exit=$?"
    print(f"warm_attribution_check: CANNOT READ: {why}")
    ~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
OSError: [Errno 28] No space left on device
unbuffered exit=1
```

**Step 2 — apply check 5's own procedure to revision 6's document**, i.e. to the
revision where that defect stood unconceded and the invariant was false:

```
$ for R in 3a198de 1618467; do echo "-- $R --"; \
    git show $R:docs/experiments/wp16_sprt_prereg.md \
    | grep -E "^ *python3 tools/wp1(5b|6_warm)_attribution_check.py" \
    | sed "s/<scratch>/SCR/g" | grep -cE "[|]|>|tee"; done
-- 3a198de --
0
-- 1618467 --
0
```

Revision 6 already had three registered invocations and not one of them piped or
redirected the checker's stdout:

```
$ git show 3a198de:docs/experiments/wp16_sprt_prereg.md | grep -nE "python3 tools/wp1(5b|6_warm)_attribution_check.py" | grep -v dev/full
948:python3 tools/wp15b_attribution_check.py <scratch>/wp16_dryrun_report.txt target/release/pistol
1108:    python3 tools/wp16_warm_attribution_check.py <scratch>/run.txt <scratch>/replay.txt target/release/pistol
1114:    python3 tools/wp16_warm_attribution_check.py <scratch>/swapped.txt <scratch>/swapped_replay.txt target/release/pistol
```

**Conclusion: check 5 would have been GREEN at revision 6, alongside the very
defect it is advertised as able to falsify.** The document's own sentence,
turned on its own fifth check, reads: *five* green checks and a false invariant
would have been perfectly consistent. The claim "THE FIFTH IS ONE THIS DEFECT
CLASS COULD HAVE FALSIFIED" is false by the document's own test, and it is the
sentence that discharges the rev-6 reviewer's explicit remedy ("Either way, add
to the four registered checks one that this defect could have falsified").

**The strongest steelman, stated because it is strong and the architect should
weigh it.** Once the document *concedes* the output-path defect and carves it
out, there is nothing left about the code for a check to falsify — you cannot
falsify a conceded fact. What remains worth checking is whether the carve-out is
*reachable in the governed workload*, and check 5 does exactly that job, does it
correctly, and is genuinely falsifiable by a `tee` added to §8.2. On that
reading check 5 is the right check and only its label is wrong.

**I accept every word of that steelman, and it is why this finding is about a
sentence rather than about the check.** But the document does not say it. It
says the opposite: it asserts compliance with the vacuity rule on the one check
added to satisfy that rule, and the assertion is measurably untrue. Under
CLAUDE.md a criterion whose falsifiability is asserted rather than established is
the named breach, and this project's own history — D-419 MAJOR A, D-420's
exception tuple, D-421 MINOR 2 — is a chain of exactly this: a guarantee
asserted one level above where it was tested.

**Grading.** MAJOR because it is the load-bearing discharge of MAJOR 1's
document-limb under a cap that permits no second attempt, and because it
reproduces, at the meta-level, the defect class the round exists to close. It has
**no operational consequence**: the check is true, useful, and correctly guards
the carve-out, so nothing about the governed run is unsafe on account of it. If
the architect prefers to grade a false self-label MINOR and ship, that is a
defensible call and I have given the evidence to make it.

**What would close it**: one honest sentence. Drop "ONE THIS DEFECT CLASS COULD
HAVE FALSIFIED" and say what the check actually does — "the four above establish
the ANALYSIS-path claim; the fifth establishes that the OUTPUT-path carve-out is
UNREACHABLE IN THE REGISTERED WORKLOAD, which is what a narrowed invariant needs
in place of a check the concession has made impossible" — and amend D-421 to
match.

---

### MINOR 1 — "stated in a comment at its handler" is FALSE of the narrowed sentence; the comment still carries the retracted one

§7A.1 introduces the narrowed invariant with:

> **THE INVARIANT THIS TABLE RESTS ON, introduced in revision 6, NARROWED IN
> REVISION 7 TO WHAT IS ACTUALLY PROVABLE, enforced in the instrument and
> stated in a comment at its handler:**
>
> > **No exception raised by the instrument's own ANALYSIS can reach exit 1
> > unnamed.** A failure of the instrument's OUTPUT CHANNEL, or a signal death,
> > terminates on CPython's own status and is OUTSIDE this invariant.

The handler's comment says something materially different — and says the thing
revision 7 exists to retract:

```
$ awk 'NR>=844 && NR<=849 {printf "%d: %s\n", NR, $0}' tools/wp16_warm_attribution_check.py
844:     #   EXIT 1 ARISES ONLY FROM THE NAMED ATTRIBUTION FINDINGS. The single site that
845:     #   reaches it is `main()`'s `return ATTRIBUTABLE if not failures else
846:     #   NOT_A_MEASUREMENT`, so exit 1 means the `failures` list is non-empty and
847:     #   every entry in it was printed by name. EVERY OTHER TERMINATION OF THIS FILE
848:     #   IS EXIT 0 (a measurement), EXIT 2 (no answer could be taken) OR EXIT 3
849:     #   (a determinism violation, on `violation()`'s own code).
```

`:847-849` is revision 6's unnarrowed claim, which the rev-6 reviewer MEASURED
false (exit 120) and which revision 7's own §5 row now contradicts in terms. The
narrowed sentence appears nowhere in the instrument.

In revision 6 the phrase was accurate — document and comment said the same
thing. Revision 7 changed the document, could not change the instrument (by
ruling, correctly), and **carried the provenance phrase across unexamined**. The
phrase is now a receipt that fails on inspection. It also makes the section's
rhetorical move — "**THE INSTRUMENT ALREADY SAID SO AND THE DOCUMENT DID NOT
LISTEN**", citing the `KeyboardInterrupt` sub-clause at `:870-872` — a selective
quotation: the instrument's sub-clause concedes one carve-out while the same
comment's headline sentence six lines earlier denies all of them.

No operational consequence, and the architect's ruling makes the comment
unfixable this round, so the honest fix is textual: say that the instrument's
comment states the BROADER revision-6 form, that it is superseded by this
document at the same pin, and that the discrepancy is standing debt scheduled
with D-422's. Recorded as MINOR for that reason.

---

### MINOR 2 — the fifth check's criterion is a SUBSTRING test its own quoted commands fail, and one of its "three quotes" is not a quote

Two defects of precision in the same bullet, both of the kind the adjacent
paragraph explicitly warns about.

**(i) The literal criterion is false of the commands it is applied to.** The
check says the three registered invocations are "all three carrying no `>`,
`>>`, `|` or `tee`". They all carry a literal `>`, inside the `<scratch>`
placeholder:

```
$ sed -n '1045p;1205p;1211p' docs/experiments/wp16_sprt_prereg.md | grep -o "<scratch>"
<scratch>
<scratch>
<scratch>
<scratch>
<scratch>
$ git show 3a198de:… | grep -E "python3 tools/wp1(5b|6_warm)_attribution_check.py" | grep -cE ">|\||tee"
3
```

A reviewer mechanising the check as written gets three hits and must know to
discount them. The same imprecision bites on `|`: §8.2's registered block
contains two pipes at `:1043-1044` (`printf … | target/release/pistol`), which
are nothing to do with the checker's stdout but would trip a naive scan of "the
registered commands". The intended criterion — *no redirection operator applied
to the checker's stdout* — is true, and I verified it holds (C7); it is the
stated one that is wrong.

This is `tools/SHELL_CHECKLIST.md` item 3's "a substring is not a token" — named
by the document itself **two paragraphs above**, in the `die()`-count receipt,
where it was handled correctly by an `ast` cross-check. The fifth check gets no
such treatment.

**(ii) "all three quoted from their own code blocks" — one of the three is not
quoted.** Only §8.2's is character-exact; §8.6's two commands are represented by
a single elided composite containing U+2026:

```
QUOTES INSIDE CHECK 5 (lines 849-850):
   [python3 tools/wp15b_attribution_check.py <scratch>/wp16_dryrun_report.txt target/release/pistol]
   [python3 tools/wp16_warm_attribution_check.py <scratch>/… target/release/pistol]
REGISTERED CODE-BLOCK COMMANDS:
   1045 [python3 tools/wp15b_attribution_check.py <scratch>/wp16_dryrun_report.txt target/release/pistol]
   1205 [python3 tools/wp16_warm_attribution_check.py <scratch>/run.txt <scratch>/replay.txt target/release/pistol]
   1211 [python3 tools/wp16_warm_attribution_check.py <scratch>/swapped.txt <scratch>/swapped_replay.txt target/release/pistol]

CHARACTER-EXACT?  [1045] <- python3 tools/wp15b_attribution_check.py …
CHARACTER-EXACT?  NONE   <- python3 tools/wp16_warm_attribution_check.py <scratch>/… target/release/pistol
ellipsis char in quote 2: True
```

The elision spans precisely the region where a redirect could hide, so the quote
does not itself establish what it is offered to establish — the reader must go
to §8.6. This document's own receipts standard is character-exact quotation
(D-419 verified twelve instrument strings that way, and the rev-6 review nine).
**MINOR**, because the underlying fact is TRUE — I checked `:1205` and `:1211`
directly and neither carries a redirect — and because the line references are
given, so the claim is at least traceable.

---

## Attacks ATTEMPTED and REJECTED

- **Can an exception from the instrument's own ANALYSIS reach exit 1 unnamed
  with a HEALTHY output channel?** This is the claim that now matters and I
  attacked it from five directions and could not break it. (1) Exit 1 has one
  and only one source in the analysis path — `NOT_A_MEASUREMENT` is used at
  `:838` alone, `:133` being its definition and `:846` a comment. (2) The only
  `raise SystemExit` sites are `:169`, `:182`, `:874`. (3) The handler at `:875`
  catches `Exception`, a catch-all, and its body is `die()` → exit 2. (4) The
  only escapes are `BaseException`s, of which the reachable ones are the
  intended `SystemExit` and `KeyboardInterrupt`, explicitly carved out by the
  second sentence. (5) `die()` reaching exit 2 with a working channel is
  measured, not argued (the usage refusal, C5). **Rejected — the narrowed
  sentence stands.**
- **Can the COMPOUND case break it — an analysis exception AND a failing output
  channel, so that an analysis exception does reach exit 1 unnamed?** This is
  the sharpest reading available against the sentence, and I pushed it hard. It
  is real: with `PYTHONUNBUFFERED=1` and stdout on `/dev/full` the handler's own
  `die()` fails and exit 1 results with no `CANNOT READ:` line. But the second
  sentence carves out termination *caused by the output channel*, and in the
  compound case the output channel is what terminates the process — the analysis
  exception was caught and handled. **Rejected as a falsification**, and I note
  the architecture that makes it coherent: check 5 exists precisely to make the
  compound case unreachable in the governed workload, which is why §5 needs no
  row for it. That reading is also why MAJOR 2 is about check 5's *label* and
  not its *substance*.
- **Does §5's new row have a gap, given the compound case exits 1 and 1 is
  inside 0/1/2/3?** Considered as a candidate finding and **rejected**: the
  rev-6 reviewer's remedy asked only for a row covering a status outside
  0/1/2/3, revision 7 supplied exactly that, and the compound case is closed by
  reachability (check 5) rather than by a §5 row. Reported as an observation
  below, not as a finding.
- **Is there a FOURTH registered invocation that redirects, which would make
  check 5 simply false?** No. I enumerated every `python3` line and every
  `attribution_check.py` mention in the file (C7). Three registered invocations,
  no redirect operator on any. The two redirecting lines at `:780`/`:783` are the
  rev-6 reviewer's reproducers, explicitly introduced as such. **Rejected.**
- **Does `grep -oE '\bdie\('` overcount by matching inside string literals, so
  that `50` is right by luck?** No — `ast` agrees at exactly 50 and the set
  difference between the raw grep hits and the `ast.Call` linenos is exactly
  `[167, 867]`, the two lines the receipt names. The receipt is not merely
  arithmetically consistent, it subtracts the right two lines. **Rejected.**
- **Is `:867` actually a code line the receipt is wrongly discounting?** No —
  printed it, it is a comment, and `ast` confirms no `Call` there. **Rejected.**
- **Is the `:870-872` line range off by one, as a hand-transcribed citation
  easily is?** No — the quoted sentence starts mid-`:870` and ends at `:872`.
  **Rejected.**
- **Did the header's self-account miss a section, as it did twice before
  (D-416 MINOR 4, D-419 MINOR G)?** No. Verified by per-section `sha256` over
  all 28 sections AND independently by mapping all 13 hunk headers to section
  line ranges. Both agree on exactly HEADER, §5, §7A.1, §11. **Rejected — and
  this is the one place the document's history predicted a finding and did not
  produce one.**
- **Did the diff disturb any territory the rev-6 review verified clean, which
  would void the scoping?** No — every such section digests `same` (C4).
  **Rejected.**
- **Is D-422's "pre-existing at `bfdf933`" ground actually true, or is the FIFO
  exposure something `43e8a86` introduced?** True — `open(path, "rb")` is at
  `bfdf933:213` and the instrument fix's diff contains no `slurp`/`open(path`
  line at all. **Rejected.**
- **Is D-422's "the registered commands pass ordinary `arena --out` paths"
  ground true?** True at all three call sites (C8). **Rejected.**
- **Could the FIFO exposure reach the governed run through §8.6's
  `<scratch>` placeholder, if `<scratch>` were a directory containing a FIFO
  named `run.txt`?** Only if the operator created it, and `:1203-1204` show
  `run.txt` and `replay.txt` are written by `arena --out` in the same block.
  **Rejected** — and in any case the issue itself is out of scope by D-422.
- **Does the rule in §5 misclassify the `documents_that_are_not_about_each_other`
  refusal (`bind`, `:797`) — a named VOID that fires after `read_report` and
  `read_replay`?** I could not settle this without building fixtures, and it is
  the same shape as limb (c) which I did reproduce, so I did not press it and I
  do not count it. Recorded under "could not verify".

---

## Anything I could not verify

Recorded rather than assumed.

1. **The analysis-exception path with a healthy channel, driven end-to-end on a
   hand-built fixture.** I did not reconstruct the `alpha 1.0` report/replay
   pair outside the Rust test harness, so my C5 conclusion rests on the code
   path (five independently checked limbs) plus the usage-refusal reproducer,
   not on an independent end-to-end fixture. The shipped test
   `an_unanticipated_exception_is_a_refusal_and_not_a_finding` asserts exactly
   this and was verified green and mutation-load-bearing by the rev-6 review —
   territory this dispatch places out of scope, so I relied on it rather than
   re-running it.
2. **Whether `bind`'s "not about each other" refusal is classified correctly by
   §5's new rule.** Argued above, not reproduced; it needs a fixture pair. It
   would if anything strengthen MAJOR 1 limb (c), not weaken it.
3. **The workspace test suite at `1618467`.** Not run: the non-docs diff is
   empty, so its inputs are bit-identical to the rev-6 review's green run, and
   the dispatch places the suite out of scope. I make no claim that it passes;
   I claim only that nothing it exercises changed.
4. **D-401's contents.** Not read, by operator instruction. Where the document
   cites D-401 (`:486`, `:527`, `:1197`) I took the citation on trust and could
   not check it, and I say so rather than implying otherwise.
5. **Whether a signal death's exact status matches §5's new row.** The row says
   "a signal death's own status", which is unfalsifiably general; the rev-6
   reviewer could not drive a `KeyboardInterrupt` through the shipped script
   either (its item 1). Inherited unverified.

---

## Observations — recorded for the architect, explicitly NOT graded as findings

**A missing engine may never be detected at all.** `cold_answer` is reached only
from inside `classify`'s loop over `replay["divergences"]` (`:534`), so on a run
with zero divergences the engine binary is never executed and an unrunnable or
wrong engine path is silently never exercised. That is not a misclassification —
it is the absence of a preflight, which is D-422's territory (`SHELL_CHECKLIST`
item 12 obligation 2, recorded by the rev-6 review as not discharged). Noted
because it is the same root cause as MAJOR 1 limb (c): the document reasons about
*when* the engine check fires without the instrument having a defined point at
which it fires.

**The invariant and the exit table live under a `#### The inert cross-check`
heading.** Structurally odd — the exit-code table is not about the inert
cross-check — and it is very likely why §7A.1:875 was missed when §5 was
amended. Pre-existing, not introduced by revision 7, not graded.

---

## Which CAP branch fires

Both MAJORs and both MINORs are against text revision 7 itself introduced or
carried across unexamined into new text:

- MAJOR 1 limb (a) is the *absence* of a change inside the section the header
  claims to have amended, and limbs (b)/(c) are against §5's new rule text;
- MAJOR 2 is against the fifth check's new heading and D-421's new ADR line;
- MINOR 1 is against a phrase revision 7 rewrote and left pointing at something
  it no longer matches;
- MINOR 2 is against the fifth check's new bullet.

Nothing failed in territory the rev-6 review verified clean; the scoping held
and the stop-immediately branch did NOT fire.

**Per the cap, this round STOPS regardless of this outcome.** There is no second
fix round. The disposition of these four findings is the architect's, not a
further review's.
