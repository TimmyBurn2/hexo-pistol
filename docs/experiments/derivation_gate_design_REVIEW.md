# REVIEW-design (fresh context) — `docs/experiments/derivation_gate_design.md` revision 1

**WHAT I READ, AND AT WHICH REVISION.** The working-tree file, sha256
`2106db3f8887b385329a61e3941261bc8eadbbaf528cf5b3a8dbb650a2005d82`, 280 lines.
When I began, HEAD was `0321cc1` and the file was untracked; my `git stash create`
object was `a06d87b5c586e623b7a2346a1d5052bde22a9e63`. **THE TREE MOVED UNDER ME**:
another session committed twice while I read, and HEAD is now
`64dbd6ca8a8774fcd7d78de42633df50a9782ed7`. The file is committed there as blob
`b9e36ab1779327aa941dcabcef0567a86cdcc109` and
`git diff a06d87b5… -- docs/experiments/derivation_gate_design.md` shows only the
new-file hunk — **the bytes I judged are HEAD's bytes**, and the working tree is
otherwise clean (`git status --porcelain` empty). Every count below that is
sensitive to the move is reported at both revisions.

**ALSO READ.** `CLAUDE.md`; `docs/process.md`; `docs/decisions.md` (D-118, D-131,
D-231, D-234, D-265, D-272, D-285, D-423, D-424, D-443, D-467, D-469, D-543,
D-545, D-546, D-549, D-568, D-574, D-575, D-576, D-581); `tools/ci.sh` in full;
`tools/SHELL_CHECKLIST.md` in full; `tools/decision_key_check.sh`;
`tools/file_justification_check.sh`; `tools/design_citation_check.py`;
`tools/config_check.sh`; `tools/determinism.sh`; `docs/experiments/arc3_ledger.md`;
`docs/experiments/overnight2_RESUMED_SUMMARY.md`; and **all 57 findings** in
`wp21_prereg_rev3_REVIEW.md`, `wp21_throughput_prereg_rev2_REVIEW.md` and
`matrix_label_cache_key_REDTEAM.md`.

**REVIEWER CONSTRAINT HONOURED.** No `cargo` of any kind. Everything below is
`/usr/bin/grep`, `git grep`, `git log`, `git show`, `sed`, `awk`, `sort`, `comm`,
`wc`, `sha256sum`, `python3` over text, and reading files. Every command is
printed with its scope.

---

## VERDICT: **FAIL**

**11 BLOCKING, 10 MAJOR, 3 minor.**

The premise does not survive. **The 65% does not reproduce**: applying the
document's own stated criterion I get **14 of 57 (25%)** strictly, and **24 of 57
(42%)** at the outer limit of a charitable reading. The criterion is two criteria
wearing one sentence, and under its loose half the answer is **57 of 57**, because
all three reports were written under a no-`cargo` constraint and every finding in
them is therefore supported by read-only evidence — which makes the loose reading
vacuous.

**And the remedy does not reach even the class that is real.** Finding by finding
across all 57, a derivation block that someone would plausibly have written
catches **5**. Two of the design's own named claim kinds — *existence* and *a
count that is arithmetic* — **cannot be expressed under its own §3.1 allowlist at
all**. Its §4 half, the one it calls "cheap, zero authoring cost", **already exists
as `tools/design_citation_check.py`**, is not gated, is not mentioned, and would
have caught **zero** of the 57.

**The security surface is worse than the document estimates and worse than a
red-team finding: it is open by construction.** I ran eight escapes from the §3.1
allowlist on this machine, using only allowlisted program names and none of the
banned metacharacters. `awk 'BEGIN{system("…")}'`, `sed -n '1e …'`, and the
allowlist entry `tools/…` — which matches `tools/../../../anywhere` — each give
arbitrary code execution. Exact block text is in BLOCKING-5.

**And the document's own live derivation block is RED at the commit that landed
it.** It claims 88; the command returns **98** at `64dbd6c` (89 at `0321cc1`, 95
at `709ad32`, 96 at `114b562`). Its explanation of its own self-catch — the "best
evidence the design has" — is false: the three directories it blames contribute
**zero** hits between them.

**Do not build this package.** BLOCKING-9 names the one-line change that gets most
of the real value.

---

# BLOCKING

## B1 — **THE HEADLINE NUMBER DOES NOT REPRODUCE. I get 14 of 57 (25%), not 37 of 57 (65%).**

> **37 of 57 findings — 65% — are one defect**: a document asserting something
> about the content of a tracked file that a read-only command would have refuted.

My full re-classification is in **THE PREMISE** below, finding by finding. The
totals:

| report | design's count | **my strict count** | my outer bound | findings |
|---|---|---|---|---|
| sweep registration review | 16 | **7** | 10 | 23 |
| throughput study review | 12 | **4** | 8 | 22 |
| cache-key DECISION-RED-TEAM | 9 | **3** | 6 | 12 |
| **total** | **37 (65%)** | **14 (25%)** | **24 (42%)** | **57** |

The design's per-report share for the cache-key report is **9 of 12 = 75%**. I read
that report's twelve findings and can identify **three** — MAJOR 2, MAJOR 3,
MAJOR 6 — that are a document asserting tracked-file content refutable by a
command. The FATAL is excluded by the design's own §7. MAJOR 4 is a semantic
misattribution of a measurement's provenance; MAJOR 5 required tracing a call site
through three files; MAJOR 7 is a judgement about which attack is strongest; minors
8, 9 and 10 are about what the matrix omits. **To reach 9 of 12 you must count
findings whose refutation merely used a command, not findings whose claim was
about a file.** That is the criterion problem B2 is about.

**FIX.** Re-classify, publish the per-finding table (not a per-report share), and
re-derive every downstream sentence — including the ONE LINE, §2's "three
properties", and the arc3 ledger entry that repeats 65%.

## B2 — **THE CRITERION IS TWO CRITERIA, AND UNDER ITS LOOSE HALF THE ANSWER IS 57 OF 57.**

> a finding is in the class if it says *a document asserts something about the
> content of a tracked file — a line, a string, a count, an existence — that a
> read-only command over the tree would have refuted.*

Two conditions are conjoined by an em-dash and never separated:

- **(a)** the document asserts something about a **tracked file's content**;
- **(b)** a **read-only command** would have refuted it.

They do very different work. Under **(a) ∧ (b)** the class is 14. Under **(b)
alone** the class is **everything**, and demonstrably so:

```
$ /usr/bin/grep -n "cargo" docs/experiments/wp21_throughput_prereg_rev2_REVIEW.md | head -2
19:**REVIEWER CONSTRAINT HONOURED.** No `cargo` invocation of any kind. Everything
$ /usr/bin/grep -n "No .cargo. was run" docs/experiments/wp21_prereg_rev3_REVIEW.md
28:No `cargo` was run. Every count below was taken with `/usr/bin/grep`, `git show`,
```

**All three reviewers were forbidden `cargo`.** Every one of the 57 findings was
therefore established by read-only means. Condition (b) is satisfied by 57/57 **by
construction of the reviewers' brief**, not by any property of the defect. A
criterion that the population satisfies universally because of how the population
was collected is not a criterion; it is a selection artefact.

The document then slides between the two readings **inside its own eight-item
sample** (§1). Two of the eight rows put the refutation in the "reading" column,
not the command column:

> | *"`:89` walks play order under `gates.killers`"* | **reading** `pvs.rs:491-498` — the call site is gated by `params.ordering.any()` |
> | *"Twenty is the smallest take that exceeds thirteen"* | **arithmetic** — fourteen is |

Neither is a read-only command, and "Twenty is the smallest…" is not a claim about
a tracked file's content at all. A third row is refuted by `cat` of
`artifacts/arc3_leverB_41_count.txt` — **a gitignored artifact**, which fails (a)'s
"tracked" and which §3.2 then explicitly refuses to check. **Three of the design's
eight recognisability examples fail its own criterion.**

**FIX.** State (a) and (b) as separate conditions, classify against the
conjunction, and drop from the sample every row the conjunction excludes.

## B3 — **§1's CLAIM THAT THE PRIOR ARC COUNTED "FIVE OF THE SAME THING" IS FALSE. The prior arc's five are a class this gate provably cannot catch.**

> **AND THE PRIOR ARC COUNTED FIVE OF THE SAME THING** and wrote the remedy down

`docs/experiments/overnight2_RESUMED_SUMMARY.md:165-179`, the section the dispatch
points at, names the class in its own words and it is **not** the design's class:

> **Every one was the same defect**: a claim asserted at the scope where it was
> convenient rather than **derived at the scope where it is true**.

And the five instances (`:171-176`):

| # | what |
|---|---|
| 3 | *a grep scoped to `crates/pistol-core/src` that missed its answer in `crates/pistol-core/tests`* |
| 4 | *a line count scoped to `src` that counted `src/bin`* |
| 5 | *a sentence that said "nine" and listed eight* |

**Instances 3 and 4 are SCOPE errors: the command WAS run and its output WAS
transcribed faithfully.** A derivation block containing
`git grep -c foo -- crates/pistol-core/src` → `8` is **green forever** and still
wrong. The gate certifies output-given-command; it certifies nothing about the
command. The design concedes exactly this in §3.2 —

> **IT DOES NOT MAKE A DOCUMENT CORRECT.** … It cannot prove the command was the
> right one to run

— and then, four pages earlier, counts as its motivating precedent five instances
of the defect it has just said it cannot fix. Instance 5 is arithmetic (no command
exists for it, and §3.1 forbids the one you would reach for — see B10). Instances 1
and 2 are transcriptions from prose, which **D-568's standing law already requires
a `git grep` receipt for**; §1's own sentence says four of the five happened after
that law was written, which is an argument that authors do not write receipts they
are told to write — and §3.3 asks authors to write blocks they are told to write.

**AND THE DESIGN'S OWN SELF-CATCH IS A SCOPE ERROR TOO** (§3.2's 84-vs-88 story).
It was caught by re-running with a different scope, not by a gate. Under §3.3 the
author would have written the block with the narrow pathspec, it would have printed
84, and it would have been **permanently green and permanently wrong**.

**FIX.** Delete the appeal to the prior arc's five, or state plainly that the gate
does not reach them and that D-574's printed-scope check remains the only thing
that does.

## B4 — **§2's DIAGNOSIS AND §3.3's REMEDY ADDRESS DIFFERENT SUBCLASSES. §3.3 re-introduces the exact failure §2 says is impossible.**

§2's whole argument:

> A rule that says *check before believing* cannot fire in someone who does not
> notice they are believing anything.

§3.3's rule:

> A document that GOVERNS something … **states its load-bearing tree-claims as
> derivation blocks.**

**To write the block the author must first identify the claim as a load-bearing
tree-claim.** That is the same act of noticing D-574 asks for and §2 declares
unavailable. The mechanism is not "a command with its expected output"; the
mechanism is "an author who noticed, then a command". §2 removes the first half
and the design does not replace it.

The design's own defence is property (2), rot:

> 2. **It was true once.** Several were correct when first written and rotted when
>    the tree moved

**Measured against the design's own eight-item sample, rot is the minority.** Six
of the eight were wrong the moment they were written: `D-576` never existed;
*"THE DISPATCH SAYS 3,500"* was never in the dispatch; the receipt's fourth row was
never in the receipt; *"Twenty is the smallest take"* was never true; `:89`'s gate
was never `gates.killers`; and *"four seats"* was **transcribed from
`tools/determinism.sh`'s own stale header comment** (`matrix_label_cake_key_REDTEAM.md`
MAJOR 6), i.e. copied from a second copy, not rotted. At most the seat count and
`outpath.rs:9-24` are rot.

So the property that the gate's re-evaluation genuinely addresses covers ≈2 of 8;
the property that dominates is the one §2 says no rule can reach, and §3.3 is a
rule.

**FIX.** If the package proceeds, it must stop claiming to fix property (1). The
honest claim is narrower: *a block, once written, does not rot.* That is real and
it is small.

## B5 — **FATAL, EIGHT TIMES OVER: I EXECUTED ARBITRARY CODE THROUGH §3.1's ALLOWLIST. Every construction below uses only allowlisted program names and none of the banned characters `; && || ` $( > >> < &`.**

The brief in §6 says *"a command that runs is a FATAL"*. Here are eight. All were
run on this machine, in this repository, at `64dbd6c`. `PWNED-tom` / `tom` is the
output of `id -un` reached by the payload.

**F1 — `awk`'s `system()`.** `awk` is a general-purpose language and is on the
allowlist.

````
```derive
$ awk 'BEGIN{system("id -un")}' CLAUDE.md
tom
```
````
Ran. Output `tom`. Substitute any command. No banned character appears.

**F2 — `awk`'s pipe-to-shell.** The `|` here is inside an awk string, so a
tokenizer that splits the line on `|` to check "pipes between allowlisted programs"
sees `"sh"` as a pipeline stage, and one that does not, runs it anyway.

````
```derive
$ awk 'BEGIN{print "id -un" | "sh"}' CLAUDE.md
tom
```
````
Ran. Output `tom`.

**F3 — GNU `sed`'s `e` command.**

````
```derive
$ sed -n '1e id -un' CLAUDE.md
tom
```
````
Ran. Output `tom`. `sed 's/^/ /e'` executes the pattern space and also ran.

**F4 — GNU `sed`'s `w` command writes anywhere.**

````
```derive
$ sed -n '1w /home/tom/.profile' CLAUDE.md
```
````
Ran (tested against a scratch path, which was created and contained CLAUDE.md's
first line). `/home/tom/.profile` is not a path argument the grammar would inspect
as a *file to read*; it is part of the `sed` script.

**F5 — `sort --output`.**
````
```derive
$ sort -o /home/tom/.bashrc CLAUDE.md
```
````
Ran (scratch target). `sort` is allowlisted; `-o` truncates and writes.

**F6 — `git grep`'s pager option runs an arbitrary program.** `git grep` is
allowlisted **as two tokens**, so a subcommand check passes.

````
```derive
$ git grep -O/path/to/tracked/payload.sh -e pistol -- CLAUDE.md
```
````
Ran with a payload script; output `PWNED-tom`. `git grep -Oid -e pistol -- CLAUDE.md`
also ran `id`. `git log`'s pager is the same primitive.

**F7 — THE ALLOWLIST ENTRY `tools/…` IS A PATH PREFIX, AND `tools/../` LEAVES THE
REPOSITORY.**

````
```derive
$ tools/../../../../../tmp/anywhere/payload.sh
PWNED-tom
```
````
Ran. Output `PWNED-tom`. `python3 tools/../…` is the same hole. **And this is not
even an escape:** `tools/…` and `python3 tools/…` admit **any tracked file under
`tools/`**, so a diff that adds `tools/x.sh` and a document with
```` ```derive $ tools/x.sh ```` executes arbitrary code with no traversal at all.
The allowlist constrains a program's *location*, never its *behaviour*.

**F8 — A TRACKED SYMLINK PASSES `git ls-files --error-unmatch` AND READS OUTSIDE
THE TREE.** Git tracks symlinks (mode 120000). `cat docs/x` where `docs/x` is a
tracked symlink to `/etc/shadow` satisfies §3.1's tracked-path rule and prints the
target into the CI log. (There are no tracked symlinks today —
`git ls-files -s | awk '$1=="120000"' ` is empty — which is why this is a PR-shaped
attack, not a live one.)

**AND ONE AVAILABILITY FATAL.** §3.1 registers no timeout.
````
```derive
$ wc -l docs/somefifo
```
````
`timeout 3 wc -l <fifo>` returned exit **124** — it hangs forever. A tracked FIFO,
or `sort` on a huge tracked file, is an unkillable CI run. `tools/SHELL_CHECKLIST.md`
item 8 names the FIFO case by name: *"ACCEPTS a FIFO that then blocks every read"*.

**WHY THIS IS BLOCKING AND NOT A RED-TEAM ITEM.** §6 defers this to a RED-TEAM
subagent. It cannot be deferred: the design *specifies the allowlist*, and the
allowlist as specified is not securable by implementation. `sed` and `awk` are
interpreters; `git grep`/`git log` carry a pager option; `tools/…` is a location,
not a capability. **A design whose stated grammar admits arbitrary execution has
made the wrong decision, and no implementer can fix it inside the design.** The
only shapes that could work are (i) a closed set of *parameterised* checks with no
free-form command, or (ii) execution in a sandbox with no filesystem write and no
`exec`, which this repository has no facility for.

**FIX.** Drop free-form commands. See BLOCKING-9.

## B6 — **THE DOCUMENT'S OWN LIVE `derive` BLOCK IS RED AT THE COMMIT THAT LANDED IT, AND ITS EXPLANATION OF ITS OWN SELF-CATCH IS FALSE.**

The block, `derivation_gate_design.md:152-155`:

> ```derive
> $ git grep -ohE "gate [0-9]+/19|gate [0-9]+ of 19|19 gates|all 19" -- docs tools | wc -l
> 88
> ```

I ran it. Scope: the whole tracked tree at each named revision.

```
$ for r in 5dbff8e 0321cc1 709ad32 114b562 64dbd6c; do git grep -ohE "gate [0-9]+/19|gate [0-9]+ of 19|19 gates|all 19" "$r" -- docs tools | wc -l; done
82
89
95
96
98
```

**The stated output 88 matches no revision in the arc.** At HEAD — the commit whose
message is *"the recurring defect is 65% of every finding three reviews raised"* —
it is **98**. §3's own rule is *"a `derive` block is a live claim from the moment it
is written"*, and this one was false on arrival.

**THE SELF-CATCH NARRATIVE IS ALSO FALSE.** §3.2:

> The 84 came from a shell command run four paragraphs earlier with the pathspec
> `'docs/**/*.md' 'tools/*'`; the block's own pathspec is `-- docs tools`, which
> reaches **`docs/audit/`, `docs/research/` and `tools/` subdirectories** the first
> one missed.

Measured. Scope printed:

```
$ git grep -ohE "gate [0-9]+/19|gate [0-9]+ of 19|19 gates|all 19" -- docs/audit docs/research | wc -l
0
$ comm -23 <(git grep -lE "gate [0-9]+/19|gate [0-9]+ of 19|19 gates|all 19" -- tools | LC_ALL=C sort) \
           <(git grep -lE "gate [0-9]+/19|gate [0-9]+ of 19|19 gates|all 19" -- 'tools/*' | LC_ALL=C sort)
(no output)
$ comm -23 <(git grep -lE "gate [0-9]+/19|gate [0-9]+ of 19|19 gates|all 19" -- docs | LC_ALL=C sort) \
           <(git grep -lE "gate [0-9]+/19|gate [0-9]+ of 19|19 gates|all 19" -- 'docs/**/*.md' | LC_ALL=C sort)
docs/decisions.md
```

**All three named causes contribute exactly zero.** `docs/audit/` and
`docs/research/` hold no matching string at all, and `tools/*` and `tools` match
an identical file set. The entire difference is **`docs/decisions.md`** — a
**top-level** file, which `docs/**/*.md` misses because git's default wildmatch
still requires the literal `/` the pattern spells. The delta at `0321cc1` was 89−85
= 4 (decisions.md's four lines), not 88−84.

So: a document arguing that scope drift is the defect **got the direction of its
own scope drift backwards**, in the paragraph it calls *"the best evidence the
design has"*, and shipped a live block asserting a number that is wrong by ten.
This is not a rhetorical point. It is the second-strongest empirical result in the
review: **the block did not prevent the defect in the one document written by an
author maximally primed against it.**

**FIX.** Re-derive both numbers, correct the mechanism sentence to name
`docs/decisions.md`, and either make the block stable or delete it.

## B7 — **§5's LIVE BLOCK IS GUARANTEED TO GO RED WHEN §5 LANDS. The document is self-refuting on its own change.**

`derivation_gate_design.md:218-221`:

> ```derive
> $ /usr/bin/grep -cE '^step "gate [0-9]+/19' tools/ci.sh
> 19
> ```

Correct today (I ran it: `19`). §5 then replaces every `step "gate N/19: …"` with
`gate_step "…"`. **After the package lands, the command returns 0 and the block is
red.** §3 shows the design thinking about exactly this hazard in the other
direction —

> The command above returns `0` until §5's implementation lands, so writing it as a
> real block here would land a red gate to illustrate a form.

— and misses the symmetric case sitting two pages later in its own file. The 88
block dies too: 19 of those 98 hits are `tools/ci.sh`'s own `gate N/19` literals
(`git grep -ohE … -- tools/ci.sh | wc -l` → **19**), so §5 drops the count to ~79.

**Both of this document's live blocks are red on landing.** The design has no
answer for what a contributor does then, which is B12's finding at revision 1.

## B8 — **THE PACKAGE REVERSES TWO ADR'd DECISIONS WITHOUT CITING EITHER. Rule 10 breach.**

```
$ /usr/bin/grep -nE 'D-272|D-231|design_citation|label_consistency|decision_key|file_justification' docs/experiments/derivation_gate_design.md
(no output — exit 1)
```

The design mentions **none** of the following:

**D-272** (`docs/decisions.md:585`), which decided the opposite question in this
repository:

> **H1's INSTRUMENT LEAVES THE PRE-REGISTRATION AND BECOMES TWO TESTED SCRIPTS** …
> EVERY ONE WAS FOUND BY AN AGENT RUNNING THE DOCUMENT BY HAND, **because a command
> block printed inside a pre-registration is a thing only a reader can execute.**

`tools/solver_edge_check.sh:12-16` carries the same conclusion in shipped code:

> it is a SEPARATE FILE because the previous two adjudicators were both wrong and
> neither was testable: **they lived inside a pre-registration's prose** … (D-231's
> lesson, and `tools/SHELL_CHECKLIST.md` item 10's rule).

D-272 and D-231 record that commands living in documents produced **four
consecutive rounds of BLOCKING defects** — a `git grep` under `pipefail` that
adjudicated evidence as its opposite, a SIGPIPE, an EXIT trap that collapsed a
three-way verdict, a probe scoped to one package of a workspace. The remedy the
project adopted was **move the command out of the document into a tested script**.
§3.3 moves commands **into** documents, in bulk, across every governing document,
and does not argue against D-272 — it does not know D-272 exists.

CLAUDE.md hard rule 10: *"Silent architecture drift is a breach; amend the ADR
instead."* This is a direct reversal presented as a novelty.

**FIX.** The design must quote D-272, say why the failure mode it records does not
apply (the honest answer is that it does — every §3.1 block is an untested command,
and the gate that runs it is one script for all of them), and either amend D-272 or
withdraw.

## B9 — **§4 ALREADY EXISTS AS A SHIPPED TOOL, AND IT WOULD HAVE CAUGHT ZERO OF THE 57.**

§4 proposes:

> Cheap, zero authoring cost … a `` `path/to/file:NN` `` or `` `path:NN-MM` `` in a
> **governing** document whose path is untracked or whose line number is past the
> end of the file.

`tools/design_citation_check.py` (117 lines, committed at `b96eb15`) does exactly
this, and says so in its own header:

> Check that a document's claims about the tree are claims the tree makes. …
> It checks that **every `path`, every `path:line` and every backticked identifier
> attributed to a file is one the tree actually holds**

Its regex `PATH` (`:47-51`) matches `` `crates|tools|configs|docs/…` `` optionally
with `:line` or `:line-end`; `:80-84` refuses `end > count`. It even carries the
`--proposes` discipline the design does not think of, and D-549 records that it
"foreclosed the cheap half completely". **It is on no gate path**:

```
$ /usr/bin/grep -n 'design_citation\|decision_key\|file_justification' tools/ci.sh
177:gate "file justification" tools/file_justification_check.sh
185:gate "decision key check" tools/decision_key_check.sh
```

`docs/experiments/wp20m_DESIGN_STOP.md:111` already recorded the gap —
*"`tools/design_citation_check.py` is new and on no gate path."*

**AND ITS YIELD ON THIS ARC'S 57 FINDINGS IS ZERO.** I checked every finding for a
citation that names a missing path or a line past end-of-file. There is none.
`outpath.rs:9-24` is in range (§4 admits it: *"It would not have caught…"*);
`heuristics.rs:89` is in range; `determinism.sh:20-24` is in range;
`instrument_v0.toml:113` was correct. The one nonexistent artefact — `count_key.py`
— is named as *"scratchpad `count_key.py`"* and matches no `path:line` form.

So the "cheap half" is (a) built, (b) ungated, (c) unmentioned, and (d) worth
nothing on the motivating population. **THE SMALLER CHANGE THIS PACKAGE SHOULD
HAVE BEEN**: wire `tools/design_citation_check.py` into `tools/ci.sh` as gate 20
over `docs/experiments/*.md` and `docs/*.md`, with the `--proposes` list. That is
one `gate` line plus a coverage test, it costs no new execution surface, and it is
the change D-549's own text is waiting for.

## B10 — **THE ALLOWLIST CANNOT EXPRESS TWO OF THE FOUR CLAIM KINDS THE CRITERION NAMES.**

The criterion names *"a line, a string, a count, an existence"*. §3.1's allowlist is
`/usr/bin/grep`, `git grep`, `git log`, `git rev-parse`, `git show`, `sed`, `awk`,
`cut`, `sort`, `uniq`, `wc`, `head`, `tail`, `cat`, `sha256sum`, `python3 tools/…`,
`tools/…`.

- **EXISTENCE (and non-existence) is not expressible.** `git ls-files` is not on the
  list; neither is `ls`, `test`, or `find`. The design's own §1 sample uses
  `/usr/bin/find . -name 'count_key*'` (quoted from `wp21_prereg_rev3_REVIEW.md`
  M10) — a command its own gate would refuse. `cat` of a missing path errors rather
  than producing comparable stdout, and §3.1's tracked-path precheck would refuse
  the block before it ran.
- **ARITHMETIC is not expressible.** `python3` is admitted **only** as
  `python3 tools/…`, so `python3 -c '…'` — the natural derivation for *"Twenty is
  the smallest take that exceeds thirteen"*, for `199027 * 0.8854`, for
  `436 × 0.81 × 4`, for `ceil(6624/200) + ceil(5819/200)` — is refused. By my count
  **at least 5 of the 57** are arithmetic errors over registered numbers
  (rev3 m1, m2, m4; throughput M12; cache-key FATAL 1's rule-of-three), including
  one of the design's own eight sample rows.

**FIX.** Either widen the allowlist (which widens B5's surface) or stop claiming
the four kinds.

## B11 — **§1's OWN POPULATION BLOCK ASSERTS 12 WHERE THE COMMAND RETURNS 7 — the defect, uncaught, in the paragraph that enumerates the defect.**

`derivation_gate_design.md:24-25`:

> ```
> $ /usr/bin/grep -cE "^## (B|M|FATAL|MAJOR)[0-9]* " docs/experiments/matrix_label_cache_key_REDTEAM.md
> 12
> ```

I ran it, scope as printed:

```
$ /usr/bin/grep -cE "^## (B|M|FATAL|MAJOR)[0-9]* " docs/experiments/matrix_label_cache_key_REDTEAM.md
7
```

The report's `##` finding headings are `## FATAL 1`, `## MAJOR 2` … `## MAJOR 7`
(seven), then `## minor 8` … `## minor 12` (five, lowercase, not matched). **7, not
12.** The design's own arithmetic confirms 7 is the intended value: 16 + 14 + 7 +
(7 + 8 + 5) = **57**, the stated population; 16 + 14 + **12** + 20 = 62, which
appears nowhere. The `12` is the report's *total*, transcribed into the *command
output* slot — a claim about a file's content, refuted by the command printed
directly above it.

**Two things make this decisive rather than embarrassing.**

1. **The design did not write these as `derive` blocks.** They are plain fences.
   §3.3's rule — *"a document that GOVERNS … states its load-bearing tree-claims as
   derivation blocks"* — was not applied by its author to the single most
   load-bearing tree-claim in the package. If the rule cannot be self-applied here,
   B4 is not a theoretical objection.
2. **It could not have been a `derive` block.** At the revision it was written,
   `docs/experiments/matrix_label_cache_key_REDTEAM.md` was untracked
   (`git status --porcelain` showed `??` when I began), and §3.1 refuses any block
   whose path argument is untracked. **A design document's own population census is
   structurally underivable under its own gate**, because a review report is
   written before it is committed.

---

# MAJOR

## M1 — **NO VOID CLASS. `tools/SHELL_CHECKLIST.md` item 12, cited by number in five ADR lines, is not answered.**

> The gate runs the command from the repository root and compares. A mismatch is a
> **named refusal** quoting both, exit 1.

That is the only exit the design defines. The checklist's item 12 (`:173-187`),
whose obligations D-285 landed:

> **A code per kind.** `0` the answer is yes, `1` the answer is no, `2` no answer
> was taken. **A gate with no void class says so in its usage block** rather than
> leaving a reader to infer it from silence.

A gate that *executes* commands has obvious void conditions and the design names
none: `/usr/bin/grep` absent (`tools/ci.sh:37-38` preflights `cargo` and `git`
only), a command that times out, `git` unavailable, an unreadable blob, a scratch
shortage. Collapsing "the tree moved" (a finding) with "the gate could not take an
answer" (a void) is the exact defect D-285 was written for, and
`tools/file_justification_check.sh` is already carried in D-467 as an open instance
of it. **Do not ship a twentieth gate with the nineteenth's known defect.**

## M2 — **`grep -c` PRINTS `0` AND EXITS `1`, AND THE DESIGN'S OWN PARADIGM CASE IS SUCH A COMMAND. The gate's exit-status semantics are unspecified.**

`tools/SHELL_CHECKLIST.md:44`, verbatim: *"`grep -c` prints `0` and STILL exits
1."* Measured:

```
$ /usr/bin/grep -c 'zzzznomatch' CLAUDE.md
0
$ echo $?
1
```

§1's canonical example is exactly this shape:

> | *"D-576"*, cited four times and quoted once | `/usr/bin/grep -c '^D-576' docs/decisions.md` — **0** at that revision |

The design never says whether the gate compares **stdout only** or **stdout and
exit status**. If status, the paradigm block is unwritable. If stdout only, then
a command that died halfway (SIGPIPE from `head`, a `git` error) and printed a
truncated prefix can still match a short expected output — an EXIT-0-WRONG-ANSWER,
which the checklist's preamble calls the class it exists to prevent. And the gate
itself will run under `set -euo pipefail` like every sibling, where an un-guarded
`grep` in statement position takes the run down.

**FIX.** Register the comparison contract: stdout bytes **and** exit status, both
declared in the block, with the status line part of the grammar.

## M3 — **THE GATE'S OUTPUT IS NOT DETERMINISTIC ACROSS MACHINES. Hard rule 4, measured.**

§3.1 closes the environment — *"no environment inherited beyond `PATH` and
`LC_ALL`"* — and leaves the larger channel open: **git configuration is not
environment.** Measured, same pattern, two configs:

```
$ git -c grep.patternType=basic    grep -c 'gate [0-9]+/19' -- tools/ci.sh
(no output, exit 1)
$ git -c grep.patternType=extended grep -c 'gate [0-9]+/19' -- tools/ci.sh
tools/ci.sh:19
```

A developer with `grep.patternType = extended` in `~/.gitconfig` gets 19 where the
default gets 0. Every `git grep` block without an explicit `-E`/`-F`/`-G` is
machine-dependent. `core.quotepath`, `.gitattributes` textconv and `diff.external`
are further channels.

Second channel: **`/usr/bin/grep` is hardcoded and unpreflighted.** `tools/ci.sh:37-38`
preflights `cargo` and `git` and not `grep`. On a host whose GNU grep is not at
`/usr/bin/grep` — or is BSD grep, where `-E` and `--include` differ — every block
in the tree fails at once. §3.1 says the tracked-path rule *"is what keeps the gate
runnable on a fresh clone"*; the program path is the part that does not.

**And the rationale for the rule is imported from the wrong context.** §3.1 cites
D-265 for banning bare `grep`. D-265's finding is about *the agent tool shell's*
injected wrapper — `bash -lic 'type grep'` reports `/usr/bin/grep`. Inside
`tools/ci.sh` there is no wrapper, so the ban solves a problem the gate does not
have while creating a portability problem it does.

**FIX.** `git -c grep.patternType=…`-pin every git invocation, preflight the grep
binary as a void, or use bare `grep` in CI (which is what the sibling gates do).

## M4 — **THE GATE READS THE WORKING TREE; THE PRECEDENT GATES READ THE INDEX. Checklist item 5.**

`tools/SHELL_CHECKLIST.md` item 5 (`:60`): *"The index is what commits; the working
tree is not."* Both precedent gates obey it:
`tools/decision_key_check.sh:136-143` and `tools/file_justification_check.sh:218-245`
read tracked **blobs** via `git ls-files -s -z` → `git cat-file blob`, and D-467
records a test whose whole subject is *"the registry's own index-versus-worktree
decoy"*.

§3.1's tracked check is `git ls-files --error-unmatch` (the **index**) while the
commands then read the **working tree** (`/usr/bin/grep`, `cat`, `sed`). A block is
therefore green on uncommitted edits and red in CI, or the reverse — and this is not
hypothetical here: arc3's F-1.7 records a CI run that overlapped untracked file
creation, and F-1.5 records the tree being edited under two live reviewers.

**FIX.** Read blobs, or state that the gate deliberately checks the working tree
and why.

## M5 — **CHURN IS UNBOUNDED, AND FIXING A RED BLOCK IN A PRE-REGISTRATION IS AN AMENDMENT THAT REOPENS ITS REVIEW. The design does not say what a contributor does.**

§3.3 puts blocks in *"a pre-registration, a design, an option matrix, a dispatch"*.
CLAUDE.md's Process section:

> A pre-registration is reviewed at the revision that GOVERNS the run … and **an
> amendment reopens the review however small the diff.**

So when an unrelated refactor moves a line and a sealed pre-registration's block
goes red, the contributor's choices are: (a) edit the registration — an amendment,
reopening a fresh-context review, for a whitespace-class change; (b) leave CI red;
(c) revert the refactor. The design offers no fourth option and does not mention
the collision. F-1.5 already records what happens when a governing document is
edited mid-review: *"moving the tree under a review is an instrument fault, and a
review of a revision that no longer exists adjudicates nothing."* §3.3 makes that
collision routine.

The same problem hits records the other way. `D-576` did not exist when the reports
were written and **now does** (`docs/decisions.md:1220`), amended by D-581. Had the
reviewers been required to derive it, their blocks would now be red, and F-1.8's
own rule — *"D-576 is committed and append-only, so the correction is a new D-line
rather than an edit"* — would force falsifying a record, which §3.2 correctly
forbids.

**Volume, measured.** `git grep -ohE "gate [0-9]+/19|…" -- docs tools | wc -l`
returns 98 hits for **one** claim shape across 35 files. §3.3 scoped to governing
documents is narrower, but the design gives no estimate of how many blocks it
creates, how often the tree moves under them, or what the expected red-per-month
rate is. **A gate whose false-red rate is unestimated is a gate whose cost is
unknown.**

**FIX.** Estimate the block population and the churn rate on this arc's history;
register the amendment-vs-block-repair rule explicitly.

## M6 — **§5 IS A SEPARATE CHANGE, ITS DERIVATION HAS FOUR SILENT-MISCOUNT MODES, AND IT DOES NOT FIX THE ONE GATE NUMBER THAT IS ACTUALLY STALE.**

> ```
> TOTAL=$(/usr/bin/grep -cE '^gate_step "' "${BASH_SOURCE[0]}")
> ```

**(a) It is safe today and fragile by construction.** All nineteen `step "gate …"`
calls are at column 0, unconditional, with no heredocs in the file
(`/usr/bin/grep -nE '^step "' tools/ci.sh` → 19 hits; zero `<<` in the file). Four
ways it silently miscounts, none guarded:

1. an **indented** `gate_step` call (inside an `if`/`for`) runs and is not counted →
   prints `gate 20/19`; the file's own style already indents inside blocks
   (`tools/ci.sh:31-33`);
2. `grep -c` counts **lines**, so two calls on one line count 1 and run 2;
3. a call in a loop counts ≤1 and runs N;
4. `GATE` increments in **execution** order and `TOTAL` counts in **source** order —
   they agree only under 1–3 holding.

There is no shell linter or formatter gate to keep column 0 stable: gate 1 is
`cargo fmt` and the only `shellcheck` string in the tree is a `# shellcheck disable`
comment.

**(b) It breaks a working invocation, loudly but with the wrong code.**
`tools/ci.sh:14-15` is `ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"; cd "$ROOT"`.
`BASH_SOURCE[0]` holds the path **as typed**, so after the `cd` it re-resolves
against the new cwd. Reproduced with the exact pattern:

```
$ (cd tools; S=ci.sh; R="$(cd "$(dirname "$S")/.." && pwd)"; cd "$R"; /usr/bin/grep -cE '^step "' "$S")
/usr/bin/grep: ci.sh: No such file or directory   (rc=2)
```

`cd tools && ./ci.sh` works today and would stop working. Under `set -e` the run
dies with **exit 1** — which `tools/ci.sh:7-8` defines as *"a gate failed"* — when
no gate was reached. That is item 12's misreading exactly.

**(c) No named refusal.** Checklist item 1 (`:24-25`): *"`VAR="$(cmd)"` does
propagate under `set -e` — but **a bare `set -e` death prints nothing**, which is
rule 3's other failure: name the refusal."* Rename `gate_step` and forget the
pattern and the run dies silently with rc 1.

**(d) The cost it removes is not the cost that has bitten, and the cost that has
bitten is not fixed.** There is **no machine consumer** of the gate line — no test
drives `ci.sh`, no `.github/`, no Makefile; the only in-code references are three
comments. So nineteen literals cost nineteen one-character edits, once per gate
added. Meanwhile the tree's one **actually stale** gate number is in another file:

```
$ git grep -nE 'gate [0-9]+( of |/)[0-9]+' -- tools | /usr/bin/grep -v '^tools/ci.sh'
tools/config_check.sh:84:		echo "config_check: tools/arena_smoke.sh runs a committed one (CI gate 13 of 16)" >&2
```

Arena smoke is **gate 15/19** (`tools/ci.sh:157-158`). `TOTAL` does not reach it.
**§5 derives the copies that never went stale and leaves the one that did.**

**(e) It is smuggled.** §5 changes the file CLAUDE.md calls *"the sole definition of
the gates"*, falsifies a CLAUDE.md sentence, and requires an operator edit — for a
benefit unrelated to derivation blocks. It belongs in its own package with its own
matrix, if at all.

**One correction owed either way**: CLAUDE.md:43 says ci.sh *"prints `gate N/19:
<name>`"*. `tools/ci.sh:17` is `step() { printf '\n=== %s\n' "$*"; }`, so the
emitted line is `=== gate 1/19: cargo fmt --all --check`. The sentence is already a
substring claim, not the printed line; the repo's own closures quote the fuller
`=== gate N/19:` form.

## M7 — **THE §3.2 BOUNDARY IS NOT EXACT. Three load-bearing claim classes fall in neither half.**

> **The boundary is exact and it is the package's main design decision: derivation
> blocks are for claims about the TREE; receipts are for claims about RUNS**

Three classes fall between, and together they account for **at least 12 of the 57**:

1. **ARITHMETIC OVER REGISTERED NUMBERS.** Not tree, not run, and B10 shows §3.1
   cannot express it. rev3 m1 (`199027 × 0.8854` does not reproduce), m2, m4
   (*"Twenty is the smallest take"*), throughput M12 (a record count printed as
   seconds), rev3 M3 (`436 × 0.81` missing its `×4`). **Five findings, unprotected,
   and one of them is the design's own sample.**
2. **CLAIMS ABOUT AN ARTIFACT THAT IS NOT A SHA-ANCHORED RECEIPT.** §3.2 hands these
   to D-469's receipt manifest. `artifacts/arc3_leverB_41_count.txt` is gitignored
   and **nothing sha-indexes it** (throughput M10(c) says so explicitly), so the
   quoted-four-rows defect — the design's own sample row 7, and arc3's F-1.6, *"the
   arc's named defect in its purest form"* — is in neither half. Two findings
   (throughput M3, rev3 M1) plus F-1.6.
3. **CLAIMS ABOUT AN INSTRUMENT'S BEHAVIOUR.** *"`arena --capture … --label-nodes
   … --out …` is the command every setting runs"* (throughput B2) is a claim the
   binary refutes, not the tree. rev3 M8 (the criterion quotes an output line the
   shipped tool cannot print) is the same shape. The remedy for this class already
   exists and is not a gate: `docs/process.md`'s **Dry-run discipline**, which the
   throughput registration simply did not follow. **A new gate adds nothing here; an
   unfollowed rule is not fixed by a different unfollowed rule.**

**FIX.** Name the three classes in §3.2 and say what covers them (nothing;
receipts-if-anchored; the dry-run rule).

## M8 — **THE FENCE GRAMMAR IS UNDERSPECIFIED, AND A NAIVE PARSER EXECUTES THE ONE BLOCK THE DOCUMENT SAYS MUST NOT RUN.**

```
$ /usr/bin/grep -nE '^`{3,}' docs/experiments/derivation_gate_design.md
102:````
103:```derive
106:```
107:````
152:```derive
155:```
218:```derive
221:```
$ /usr/bin/grep -cE '^```derive$' docs/experiments/derivation_gate_design.md
3
```

Three `^```derive` fences; **two are live and one is the illustration**, nested
inside a four-backtick fence at 102–107. §3 states the convention in prose —
*"An illustration is shown inside an outer fence; a claim is a block"* — and §3.1's
grammar section says nothing about parsing, nesting, or the CommonMark rule that a
fence closes only on a run of the same character at least as long. A `grep`-shaped
scanner runs the illustration, whose command
`/usr/bin/grep -c '^gate_step "' tools/ci.sh` returns **0** against an expected
`20`, and **the gate goes red on its own design document on day one**.

Also unspecified: how a trailing blank line in expected stdout is represented; how
a block asserting empty output is written (the `D-576` case); whether `$ ` inside
expected output is re-read as a command; encoding and trailing-newline handling.

**FIX.** Specify the parser against a named Markdown grammar, with a fixture per
ambiguity, and make the illustration syntactically impossible to mistake (a
non-`derive` info string).

## M9 — **HARD RULE 9 IS NOT ADDRESSED, AND THE SCRIPT WILL EXCEED THE CAP.**

`tools/decision_key_check.sh` is 197 lines and does one regex over one file;
`tools/file_justification_check.sh` is 266 lines and does four checks over two file
sets. `tools/derivation_check.sh` must do: fence parsing with nesting, command
tokenization, a metacharacter ban, an allowlist with pipeline stages, per-argument
tracked-path validation, environment scrubbing, timeout handling, execution,
byte-comparison, a named refusal, a void class, and a seeded self-test (both
precedents self-test before running). **It will cross 300 lines.** Neither §5 nor
§6's obligations table mentions `docs/rule9_justifications.md`. Under D-467 the
registry is the only place the why may live, and gate 17 goes red without an entry.

**FIX.** Add the registry entry to §6's table, or split the script.

## M10 — **§6's OBLIGATION TABLE MISPLACES THE COVERAGE TEST AND UNDER-SPECIFIES THE MUTATION SET.**

> `crates/pistol-arena/tests/derivation_check_tests.rs`, in the pattern
> `wp21_tranche_config_tests.rs` and `label_cache_count_tests.rs`

Those two drive `tools/` **Python** scripts that belong to the arena's workload.
The precedent for a `tools/` **gate** is
`crates/pistol-cli/tests/decision_key_check_tests.rs` and
`file_justification_gate_tests.rs`. Placing a gate's test in `pistol-arena` splits
the gate-test suite across crates for no reason.

More seriously, `docs/process.md:28-30` requires *"at least one test driving the
shipped script"* **with a control run so a pass cannot come from a gate that
refuses everything** (checklist item 10). §6 names the file and not the control.
For a gate whose failure mode is *executing something it should have refused*, the
control is the load-bearing half: a fixture per allowlist escape in B5, each
asserting a refusal **naming the program**, plus a fixture asserting a legitimate
block still passes.

---

# minor

**m1 — D-265's rationale is cited for a context it does not describe.** §3.1 bans
bare `grep` *"(CLAUDE.md's Environment section, D-265: the agent shell's `grep` is
wrapped…)"*. D-265's own text says the wrapper *"is injected by the tool harness
into the agent's own shell and by nothing else"* and that
`bash -lic 'type grep'` reports `/usr/bin/grep`. `tools/ci.sh` is not an agent
shell. The ban is right for a document an agent runs by hand and wrong for a gate,
and M3 shows the cost.

**m2 — §5's quoted CLAUDE.md sentence is already inexact.** *"it prints
`gate N/19: <name>`"* — `tools/ci.sh:17` emits `=== gate N/19: <name>` preceded by a
blank line. If the sentence is being corrected anyway, correct it fully.

**m3 — §6's RED-TEAM brief is written as if the allowlist were the design's
strongest part.** *"A refusal that fires is a pass; a command that runs is a
FATAL."* B5 shows eight commands that run, constructed in under an hour from the
design's own text. A brief that expects the red team to *fail* mis-sets the
reviewer's prior and risks a shallow round.

---

# THE PREMISE

## My re-classification, finding by finding

**Criterion applied, conjunctively as B2 asks:** the finding's core is a document
assertion about **tracked-file content** — a line, a string, a count, an existence
— that a **read-only command** would have refuted. I mark **IN**, **out**, or
**~** (in only under the loose, command-was-used reading).

### `wp21_prereg_rev3_REVIEW.md` — 7 IN of 23 (design: 16, 70%)

| # | finding | verdict | why |
|---|---|---|---|
| B1 | D-576 does not exist | **IN** | `grep -c '^D-576' docs/decisions.md` |
| B2 | no registered command block, no dry run | out | an omission in the reviewed document + a process rule |
| B3 | run log cannot distinguish cached/uncached | out | design gap; nothing asserted about a file |
| B4 | void rule contradicts the dispatch | out | semantic contradiction between two texts, not a false content-claim |
| M1 | 0.5323 measured under the wrong key | ~ | the refuting `cat` is over a **gitignored** artifact; §3.2 refuses it |
| M2 | quotes as MEASURED a figure its source calls ungoverned | ~ | a dropped qualification, not a false content-claim |
| M3 | 4-worker rate applied to a 1-worker criterion | out | arithmetic + inference |
| M4 | §6.1 attributes 7.15 h to a document that does not state it | **IN** | `grep -n '7\.15' wp21_prereg.md` |
| M5 | ONE LINE asserts what rev 3 unsettled | ~ | internal contradiction in one file |
| M6 | re-registration trigger can never fire | out | logic over {1,2,4,8,16} |
| M7 | T-A's defect column is vacuous on uncached tranches | out | vacuity argument |
| M8 | criterion quotes an output the shipped tool cannot print | **IN** | `cold_label_check.py`'s single `--stride` is tracked content |
| M9 | empty-class behaviour unregistered | out | omission |
| M10 | §8 empty; `count_key.py` not in the tree | **IN** | existence — **but see B10: not expressible** |
| M11 | three output paths unregistered | out | design gap |
| M12 | *"THE DISPATCH SAYS 3,500"* — it does not | **IN** | `grep -n '3,500\|3500' wp21_DISPATCH.md` |
| m1 | headline figure does not reproduce | out | arithmetic |
| m2 | cold-check line not re-derived, `0.9` unsourced | out | arithmetic |
| m3 | "four inputs" then uses six | out | internal |
| m4 | *"Twenty is the smallest take"* | out | arithmetic (design counts it IN) |
| m5 | `outpath.rs:9-24` off at both ends | **IN** | line range in a tracked file |
| m6 | ledger row still names revision 2 | **IN** | `book_v2_ledger.md` content |
| m7 | T-C's limb does not say where to look | out | omission |

### `wp21_throughput_prereg_rev2_REVIEW.md` — 4 IN of 22 (design: 12, 55%)

| # | finding | verdict | why |
|---|---|---|---|
| B1 | D-576 does not exist | **IN** | same command |
| B2 | registered command refused by the shipped binary | ~ | a claim about the binary's behaviour, not the tree (M7 class 3) |
| M1 | determinism.sh is gate 9, not gate 6 | **IN** | `grep -n 'gate [0-9]*/19' tools/ci.sh` |
| M2 | §1 and §2 contradict on transpositions | ~ | refuted over `corpus_v1.txt`, an **untracked** artifact |
| M3 | receipt lacks the rows attributed to it | ~ | gitignored artifact; §3.2 refuses |
| M4 | key is one of four arguments; invariant unregistered | out | design gap |
| M5 | no cache OFF switch registered | out | omission |
| M6 | protective rule has no mechanism | out | omission |
| M7 | C3 cannot be evaluated from any output | out | unevaluability argument |
| M8 | consequence pre-attributes the diagnosis | out | judgement |
| M9 | no registered sweep after this one | ~ | absence in a tracked ledger, refuting a claim about the future |
| M10 | replication is not a second instrument | out | process |
| M11 | key selected without an OPTION MATRIX | out | process |
| M12 | "~171 s per process" is a units error | out | arithmetic |
| m1 | C1/C2 name corpus where the tool writes a capture | ~ | tracked source, but the claim is about an output kind |
| m2 | *"no state crosses an ask"* is false | **IN** | `instance.rs:112-119`'s own comment |
| m3 | BTreeMap rationale misreads rule 4 | ~ | refuted by CLAUDE.md's text |
| m4 | canonicality is `Turn`'s, not `position_line`'s | **IN** | `exchange.rs:154-161` is four lines calling `to_string` |
| m5 | machine-seconds algebra over-states | out | derivation |
| m6 | `search_nodes` no longer sums | out | forward-looking |
| m7 | play pass has no registered instrument | out | omission |
| m8 | D-423, the block stated five times | out | meta |

### `matrix_label_cache_key_REDTEAM.md` — 3 IN of 12 (design: 9, 75%)

| # | finding | verdict | why |
|---|---|---|---|
| FATAL 1 | MEASURED over a population 10⁵ too small | out | design's §7 agrees |
| MAJOR 2 | the checker row is false about the shipped tool | **IN** | `grep -n "sampled = " cold_label_check.py`; the word "partition" is absent |
| MAJOR 3 | both citations unverifiable at the named revision | **IN** | `git log --oneline`, `git show HEAD:… | head -1` |
| MAJOR 4 | the MEASURED cost cell measures another program | out | provenance/semantics |
| MAJOR 5 | `:89` names the wrong gate | ~ | required tracing `pvs.rs:491-498` and `params.rs:114-116`; the design's own sample column says *"reading"* |
| MAJOR 6 | four seats, not five | **IN** | `grep -n '^\t"' tools/determinism.sh` |
| MAJOR 7 | the recorded strongest attack is not the strongest | out | judgement |
| minor 8 | the baseline is not a row | out | omission |
| minor 9 | three options missing from the field | out | omission |
| minor 10 | the hit rate is the pilot's | out | population marking |
| minor 11 | K4 over-costs zobrist | ~ | rests on D-8 and a probability |
| minor 12 | ships at a revision whose CI failed | ~ | ledger content |

**TOTAL: 14 IN (25%). With every `~` admitted: 24 (42%). Neither is 37.**

## Would a block have caught it, and would anyone have written it?

For each of my 14 I asked both halves. **Expressible under §3.1** and **plausibly
written by an author who did not doubt the claim**:

| finding | expressible? | plausibly written? |
|---|---|---|
| rev3 B1 / throughput B1 (D-576) | yes, but the command exits 1 (M2) | **no** — an ADR citation is not experienced as a tree-claim |
| rev3 M4 (7.15 h) | self-referential; circular | **no** |
| rev3 M8 (tool cannot print the line) | partly | **no** |
| rev3 M10 (`count_key.py` absent) | **NO** — no `ls-files`/`find`/`test` on the allowlist | n/a |
| rev3 M12 (dispatch 3,500) | yes | **yes** — the paragraph's whole job is reconciliation |
| rev3 m5 (`outpath.rs:9-24`) | yes | **no** — §4 concedes it |
| rev3 m6 (ledger revision) | yes | **no** |
| throughput M1 (gate 6→9) | yes | **borderline yes** — the sentence cites the gate as evidence |
| throughput m2 (state crosses an ask) | no — a negative over all state | n/a |
| throughput m4 (canonicality attribution) | yes | **no** |
| cache MAJOR 2 (checker row) | yes | **yes** — one of the recommendation's three stated reasons |
| cache MAJOR 3 (both citations) | yes | **yes** — "why it exists at all" is load-bearing |
| cache MAJOR 6 (four seats) | yes | **yes** — the single most load-bearing paragraph |

**FIVE of 57 — 8.8%.** Even granting that every strictly-in-class claim would have
carried a block, the ceiling is 12 of 57 (21%), because two are not expressible at
all. And two of the five plausible blocks (MAJOR 3's `git log --oneline` and
`git show | head -1`) carry a commit **subject line** as expected output and go red
on the next commit to that file — M5's churn, at maximum.

**AND THE FIVE HAVE A CHEAPER REMEDY THAT IS ALREADY LAW.** All five are
"transcribed a fact instead of deriving it" — precisely what D-568's standing law
(*"a mutation set is specified against call sites enumerated by a `git grep`
receipt … never against prose"*) and D-574's check (*"print the command WITH ITS
SCOPE beside the claim"*) already require. The design's §1 says the law kept being
broken. **It is not obvious that a gate fixes a rule-following problem when the
gate's trigger is the same act of rule-following.**

## Is the package worth its cost?

**No.** Stated plainly:

- The class it answers is **25%**, not 65%, and the design's own remedy plausibly
  reaches **9%**.
- Its "cheap half" (§4) is **already built** (`tools/design_citation_check.py`),
  ungated, unmentioned, and would have caught **0 of 57**.
- Its expensive half puts **arbitrary code execution** in CI. I demonstrated eight
  escapes from the specified grammar in one session; the grammar is not securable,
  because `sed` and `awk` are interpreters, `git grep`/`git log` carry a pager
  option, and `tools/…` is a filesystem prefix.
- It **reverses D-272 and D-231 without citing them**, and those two ADRs record
  four consecutive BLOCKING rounds caused by commands living in documents.
- Its own two live blocks are red — one **now**, one **on landing** — and its
  self-catch narrative is false in all three of its named causes.
- §5 is an unrelated change to the file CLAUDE.md calls the sole definition of the
  gates, it introduces four silent-miscount modes and one loud break of a working
  invocation, and it does not fix the one gate number in `tools/` that is actually
  stale.

## What should be done instead

**One line, then stop.** Wire the existing checker into `tools/ci.sh` as a
twentieth gate:

```
step "gate 20/19 → 20/20: design citation check"
gate "design citations" python3 tools/design_citation_check.py <governing documents>
```

plus the coverage test `docs/process.md:28-30` requires and a void class per
checklist item 12. That is D-549's own unfinished business
(`wp20m_DESIGN_STOP.md:111`: *"on no gate path"*), it costs no execution surface,
and it forecloses the rotted-citation half permanently.

**Then, and separately, consider the smallest version of the derivation idea that
survives B5**: not a command, but a **parameterised assertion** with a closed set of
verbs the gate implements itself — `count <ERE> in <tracked pathspec> = N`,
`line <tracked path>:<n> matches <ERE>`, `adr <D-nnn> exists`. No shell, no `sed`,
no `awk`, no `tools/…`, no pager, no traversal. That form keeps §3's real insight —
*the scope is on the line where a reader sees it* — and gives up nothing my
classification says is reachable, because all five plausible blocks are counts,
line contents or existence checks. If it is worth building at all, it is worth
building without an interpreter.

**And take the free finding regardless**: `tools/config_check.sh:84` says *"CI gate
13 of 16"* and arena smoke is gate 15/19. That is a one-line fix and it is the only
stale gate number in `tools/`.

---

# WHAT SURVIVED ATTACK

**S1 — §2's mechanism analysis is the best thing in the document and I could not
break it.** *"The claim is subordinate … reached for as LABELS for things already
believed"* is a real and well-observed account of why D-574 does not fire, and the
three properties are individually defensible. My attack (B4) is that the remedy
does not follow from them, not that they are wrong. If the package is withdrawn,
§2 should be preserved — an ADR line or a `docs/process.md` paragraph — because it
is the most useful sentence anyone has written about this failure class in the arc.

**S2 — §3.2's refusal to police free prose is correct, and its reason is the right
one.** *"This repository's documents are largely RECORDS"* is true and the
consequence — *"a gate that went red on them would be demanding that records be
falsified"* — is exactly right. I attacked this from three directions (could a
governing/record split be inferred mechanically? could a "claims" section be
required? could records be frozen by digest instead?) and every alternative is
worse. The 98 measured occurrences of one claim shape across 35 files, almost all
historical, settle it.

**S3 — §7's honesty about what the package does not fix is real and unusual.**
*"'MEASURED' applied to the wrong population is a true measurement with a false
quantifier, and no command refutes it"* correctly excludes the cache-key report's
FATAL, which is the single most serious finding in the 57. A design that excluded
its population's worst finding on principle rather than quietly counting it is
arguing in good faith, and I want that on the record beside the FAIL.

**S4 — the population count of 57 is right.** I re-derived it: 16 + 7, 14 + 8,
7 + 5 = 23 + 22 + 12 = 57. The `12` in the third code block is wrong (B11) and the
57 is not.

**S5 — "the check cannot fire" is a correct diagnosis of D-574.** §1's *"FOUR OF
THAT ARC'S FIVE HAPPENED AFTER THE LAW WAS WRITTEN"* reproduces against
`overnight2_RESUMED_SUMMARY.md:178`, and *"three of them in the very document that
restated the law"* is consistent with what I read. The problem is real. The
proposed solution is the wrong one.

**S6 — the illustrative-block rule is a genuinely good instinct.** *"A `derive`
block is a live claim from the moment it is written"* is the right invariant, and
noticing it in §3 is a sign of careful thinking. M8 and B7 are failures to apply it,
not failures to have it.

---

# ATTACKS I ATTEMPTED AND REJECTED

**A1 — "the gate's blocks would leak secrets into CI logs."** Attempted via
`cat`/`head` on paths outside the tree. **Rejected as a standalone finding**:
§3.1's `git ls-files --error-unmatch` genuinely refuses a plain
`cat /home/tom/.ssh/id_rsa`, and I could not construct a bare path escape. The
symlink route (B5 F8) survives, but as a component of B5 rather than its own
finding, and there are no tracked symlinks today.

**A2 — "`git grep -f <file outside the repo>` reads arbitrary files."** Tested:
```
$ git grep -f /tmp/…/pat.txt -- /etc/passwd
fatal: /etc/passwd: '/etc/passwd' is outside repository
```
Git refuses pathspecs outside the worktree even under `--no-index`. **Rejected.**

**A3 — "`grep.patternType` is inherited environment and §3.1 already excludes it."**
I checked: `git config --show-origin --get-regexp '^grep\.'` returns nothing on this
machine, and §3.1's exclusion is of *environment*, which git config is not.
**Kept as M3, but downgraded from BLOCKING** — it needs a developer to have set the
config, and the `/usr/bin/grep` portability half of M3 is the stronger limb.

**A4 — "hard rule 1 is breached: the allowlist is a code-side default."** Attempted
and **rejected**. Rule 1 governs *tunables* and `Budget`; a gate's refusal set is
not a tunable, and both precedent gates hardcode theirs (`SOFT_CAP=300`,
`GRANDFATHERED='D-276 2\nD-277 2'`) with D-131 and D-467 approving. The design is
consistent with precedent here.

**A5 — "hard rule 8 is breached: the gate would read artifacts."** **Rejected** —
§3.2 explicitly refuses run-claims and §3.1 requires tracked paths. This is the one
place the boundary does exactly what it says.

**A6 — "hard rule 11 is breached."** **Rejected.** Nothing here touches
`pistol-api` or adds a transport dependency.

**A7 — "the 65% is right and I mis-scoped the criterion."** I re-read the criterion
four times and tried the loose reading in good faith. It yields 57/57 (B2), which
cannot be what was meant, and the strict reading yields 14. I could find no third
reading that lands on 37. **The attack on my own count failed.**

**A8 — "§3.3's visibility model is enough on its own."** *"A claim without a block
is not forbidden — it is visible."* I took this seriously: perhaps the value is
social, not mechanical. **Rejected on the arc's own evidence.** The 57 findings were
raised by fresh-context reviewers who read the documents *without* any block-vs-noblock
signal and found them anyway; the signal would tell a reviewer what they already
learn by reading. And the design's own document is the counterexample: its
population census carries no block, in a document whose author was maximally primed,
and nobody noticed until a reviewer ran the command.

**A9 — "the ci.sh renumbering will break a test or a CI parser."** Searched the
whole tracked tree for machine consumers of `gate N/19`. **None exist** — no
`.github/`, no Makefile, no test drives `ci.sh`, and the only in-code references are
three comments. **Rejected as a break risk**, and folded into M6 as the reason the
change buys nothing.

**A10 — "the design fabricated the 84-vs-88 self-catch to argue for itself."** This
was the sharpest version of the dispatch's item 7 and I tested it directly. **The
story is NOT fabricated**: a real pathspec difference exists, in the direction
claimed, and `docs/decisions.md` really is missed by `'docs/**/*.md'`. What is wrong
is the *mechanism sentence* (the three directories contribute zero) and the
*numbers* (84 and 88 reproduce at no revision; the pair at `0321cc1` was 85/89).
**So: an honest self-report, inaccurately derived** — which is, exactly and
unintentionally, another instance of the class. Recorded as B6, not as fabrication.

