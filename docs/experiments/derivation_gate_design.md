# The derivation gate — DESIGN, revision 2. **WITHDRAWN.**

> **ONE LINE.** Revision 1 proposed a CI gate that executes commands written
> inside Markdown. Its fresh-context REVIEW-design returned **FAIL — 11 BLOCKING,
> 10 MAJOR, 3 minor** and its verdict was *"do not build this package"*. **THE
> VERDICT IS ACCEPTED IN FULL.** Revision 1's headline number was wrong (25%, not
> 65%), its criterion was a selection artefact, its allowlist was open by
> construction, its own live block was red at the commit that landed it, and its
> "best evidence" — a self-catch — had a fabricated explanation. **And the
> argument that settles it is none of those**: the dominant sub-class is a
> command run faithfully over the WRONG SCOPE, and a gate that executes the
> author's own command is green forever on exactly that.

**THE REVIEW**: `docs/experiments/derivation_gate_design_REVIEW.md`. Revision 1 is
superseded by this document rather than deleted; what it proposed is not built.

---

## 1. WHAT THE REVIEW ESTABLISHED, AND WHAT I VERIFIED MYSELF

Three of its findings decide the outcome. **Each was re-run here before being
accepted** — a reviewer's finding is verified with a minimal reproducer before its
fix lands, and a withdrawal is a fix.

### 1.1 The gate cannot reach the dominant sub-class (the review's B3)

The prior arc's five instances name their own class:
*"a claim asserted at the scope where it was convenient rather than derived at
the scope where it is true"*. Two of the five — a `git grep` scoped to
`crates/pistol-core/src` that missed its answer in `tests`, and a line count
scoped to `src` that counted `src/bin` — are **commands that were run, and whose
output was transcribed faithfully.**

**A derivation block holding such a command is GREEN FOREVER AND STILL WRONG.**
The gate certifies output-given-command. It certifies nothing about the command,
and the command is where the defect lives. Revision 1 conceded this in its §3.2
and then, four pages earlier, offered those same five instances as its motivating
precedent.

### 1.2 Revision 1's own self-catch was a scope error, and its explanation was false

Revision 1 presented an incident as *"the best evidence the design has"*: a count
it stated as 84 and its own block returned 88. **The explanation it gave — that
the block's wider pathspec reached `docs/audit/`, `docs/research/` and `tools/`
subdirectories — is false, and one command shows it:**

```
$ git grep -ohE "gate [0-9]+/19|gate [0-9]+ of 19|19 gates|all 19" -- docs/audit docs/research | wc -l
0
```

Those two directories contribute **nothing**. The difference came from somewhere
revision 1 never identified, and it wrote a mechanism for the discrepancy without
checking the mechanism — **which is the defect, in the paragraph offered as
evidence for its cure.**

**AND THE BLOCK WAS ALREADY RED WHEN IT LANDED.** It claims 88; at the commit that
committed it the command returns **98**, and 89 at the commit before. Revision 1
ran it against a dirty working tree, wrote the number, and the tree moved twice
within the hour.

**BOTH FACTS POINT THE SAME WAY**: had the block existed under revision 1's own
§3.3, the author would have written it with the narrow pathspec, it would have
printed 84, and it would have been permanently green and permanently wrong.

### 1.3 The premise number does not reproduce

Revision 1 claimed 37 of 57 findings — 65%. The reviewer re-classified all 57 and
got **14 strictly (25%)**, 24 at the outer limit. **The criterion was two criteria
in one sentence**: *(a) the document asserts something about a tracked file's
content*, and *(b) a read-only command would have refuted it*. Under (b) alone the
answer is 57 of 57 — **because all three reviewers were forbidden `cargo`**, so
every finding they raised was necessarily established by read-only means. A
criterion the population satisfies universally because of how the population was
collected is a selection artefact.

**Revision 1's own population block was wrong too**: it asserted 12 findings for
the red-team report where its printed command returns 7.

---

## 2. WHAT IS TRUE AFTER ALL THAT, AND IT IS THE USEFUL PART

**THE DEFECT IS NOT "THE CLAIM WAS NOT CHECKED". IT IS "THE CLAIM WAS CHECKED
AGAINST THE WRONG POPULATION."** That reframing is what revision 1 missed, and it
changes what can possibly fix it: no mechanism that executes the author's own
chosen command can detect that the population was wrong, because the population
IS the author's judgement.

**WHAT DETECTS IT IS A SECOND PARTY CHOOSING A DIFFERENT SCOPE.** Every instance
in this arc was caught that way. The reviewer ran a different command and got a
different number; revision 1's own 84-vs-88 was the author accidentally running a
different pathspec.

**AND THERE IS AN ASYMMETRY IN THE EVIDENCE THAT SETTLES WHERE TO PUT THE RULE.**
D-568 and D-574 addressed the rule to AUTHORS, and the arc that wrote them broke
them four more times; this arc broke them at least nine, three of those inside the
document restating them. The same arc dispatched three fresh-context reviewers
with a brief to re-derive, and they returned **57 findings between them**.

> **A rule addressed to the author of a claim does not fire. The same rule
> addressed to a fresh context does.** That is not a statement about diligence —
> the author and the reviewer here are the same kind of agent, holding the same
> rule, minutes apart. It is a statement about what the two are doing: the author
> is reaching for a label for something already believed, and the reviewer is
> being paid to disbelieve it.

---

## 3. WHAT REPLACES THIS PACKAGE

Two changes, both small, and neither is a new execution surface.

### 3.1 GATE THE CITATION CHECKER THAT ALREADY EXISTS (the review's B9)

`tools/design_citation_check.py` — 117 lines, committed, tested, carrying its own
`--proposes` discipline — already does everything revision 1's §4 proposed, and
does it better. **It is on no gate path**, and
`docs/experiments/wp20m_DESIGN_STOP.md:111` recorded that gap and nothing acted on
it.

**REVISION 1 PROPOSED BUILDING A TOOL THAT WAS ALREADY IN THE TREE, BECAUSE ITS
AUTHOR DID NOT GREP FOR ONE.** That is the same defect once more, at the level of
the package rather than the sentence.

**ITS YIELD IS NOT ZERO, AND THAT IS MEASURED RATHER THAN ARGUED.** The reviewer
established it would have caught none of this arc's 57 historical findings. Run
today over this arc's three governing documents it catches one immediately:

```
$ python3 tools/design_citation_check.py docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md docs/experiments/matrix_label_cache_key.md
docs/experiments/wp21_prereg.md: 34 citation(s) checked, 1 unreproduced
  `tools/wp21_assemble.py` names no file in the tree, and is not --proposes'd
```

That is a real finding about revision 4 of a registration — an instrument it
names and does not yet have — and the tool found it in under a second. **The
value is prospective**: citations rot as the tree moves, and this catches the rot
on the run after it happens rather than in a review round.

**WHAT IT COSTS**: one `gate` line in `tools/ci.sh`, a `--proposes` list, and a
test driving the shipped script (`docs/process.md`'s tools/ coverage rule). No
execution of document text, no allowlist, no new failure mode. It is its own
package with its own review; this document only names it.

**AND ONE WRINKLE THAT PACKAGE MUST ANSWER, FOUND BY RUNNING THE TOOL OVER THIS
DOCUMENT**: the block above QUOTES the tool's own output, so this document now
contains the string `` `tools/wp21_assemble.py` `` and the tool flags itself
reading itself. A document that quotes a refusal inherits the refusal. Whether
that is answered by `--proposes`, by a quoting convention, or by scoping the gate
to non-quoted spans is the package's decision and not this document's — but it is
the kind of thing that turns a gate into churn, and it was found in one run.

### 3.2 MOVE THE RE-DERIVATION BRIEF FROM THE DISPATCH TO `docs/process.md`

The three reviews that caught the 57 were briefed, by hand, in three separate
dispatch prompts, to *re-derive every number, print the command with its scope,
and compare its hit count to the prose's*. **That brief is currently a property of
whoever writes the prompt.** §2's asymmetry says it is the single highest-yield
instruction in this project, and it should not depend on a dispatcher remembering
it.

**THE STANDING FORM**, for `docs/process.md`, and it is deliberately addressed to
the reviewer and not to the author:

> A reviewer of a governing document RE-DERIVES its load-bearing counts and
> citations with commands **the reviewer chooses**, never the document's, prints
> each command with its scope beside the number it returns, and reports the
> comparison. A count the reviewer reproduces only by running the document's own
> command is **not** reproduced.

**THE LAST SENTENCE IS THE WHOLE POINT** and is what revision 1 could not deliver:
independence of scope is the property that catches this class, and it is exactly
the property a gate executing the document's own command destroys.

---

## 4. WHAT IS NOT CLAIMED

1. **This does not make the class go away.** It moves the catching from a review
   round that a dispatcher had to brief into one that every reviewer owes, and it
   stops citation rot mechanically. The rest is paid for in review, and §2 says
   why that is the only place it can be paid.
2. **No number in §1 is this document's own work** except the three re-runs it
   prints; the classification is the reviewer's and the disagreement with revision
   1 is recorded rather than adjudicated by the party that lost it.
3. **The `tools/ci.sh` gate-total change of revision 1 §5 goes with it.** It was a
   separate change riding in this package's commit — the review said so — and if
   it is wanted it comes back on its own, with its own reason.
