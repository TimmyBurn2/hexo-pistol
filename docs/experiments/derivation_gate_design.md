# The derivation gate — DESIGN, revision 1.

> **ONE LINE.** Across three fresh-context reports on this arc's own documents,
> **37 of 57 findings — 65% — are one defect**: a document asserting something
> about the content of a tracked file that a read-only command would have
> refuted. The project already wrote the remedy down twice (D-568, D-574) and the
> defect kept happening, **four more times after the second writing**. This
> package stops asking authors to check and makes CI check instead: a document
> may state a tree-claim as a **derivation block** — a command and its expected
> output — and a gate runs every one of them.

---

## 1. THE DEFECT, ENUMERATED RATHER THAN CHARACTERISED

**THE POPULATION.** Every finding in the three round-1 reports this arc
commissioned:

```
$ /usr/bin/grep -cE "^## (B|M|FATAL|MAJOR)[0-9]* " docs/experiments/wp21_prereg_rev3_REVIEW.md
16
$ /usr/bin/grep -cE "^## (B|M|FATAL|MAJOR)[0-9]* " docs/experiments/wp21_throughput_prereg_rev2_REVIEW.md
14
$ /usr/bin/grep -cE "^## (B|M|FATAL|MAJOR)[0-9]* " docs/experiments/matrix_label_cache_key_REDTEAM.md
12
```

plus the minors each report lists under its own `# minor` heading: 7, 8 and 5.
**57 findings.**

**THE CRITERION, stated before the classification and applied to every one**: a
finding is in the class if it says *a document asserts something about the
content of a tracked file — a line, a string, a count, an existence — that a
read-only command over the tree would have refuted.*

| report | in the class | findings | share |
|---|---|---|---|
| sweep registration review | 16 | 23 | 70% |
| throughput study review | 12 | 22 | 55% |
| cache-key DECISION-RED-TEAM | 9 | 12 | 75% |
| **total** | **37** | **57** | **65%** |

**A SAMPLE, so the class is recognisable rather than asserted.** Each of these
was refuted by one command:

| the claim | the command that refuted it |
|---|---|
| *"`tools/determinism.sh` is CI gate 6"* | `/usr/bin/grep -n 'gate [0-9]*/19' tools/ci.sh` — it is gate 9 |
| *"four seats"* in the determinism gate | `/usr/bin/grep -n '^\t"' tools/determinism.sh` — five |
| *"`:89` walks play order under `gates.killers`"* | reading `pvs.rs:491-498` — the call site is gated by `params.ordering.any()` |
| *"THE DISPATCH SAYS 3,500"* | `/usr/bin/grep -n '3,500\|3500' docs/experiments/wp21_DISPATCH.md` — no hits; the number is D-568's |
| *"`outpath.rs:9-24`"* | reading the file — the function is `10-25`, its doc `6-25` |
| *"D-576"*, cited four times and quoted once | `/usr/bin/grep -c '^D-576' docs/decisions.md` — 0 at that revision |
| a four-row block quoted *"Receipt `arc3_leverB_41_count.txt`"* | `cat` — the receipt carried three rows, under a different key |
| *"Twenty is the smallest take that exceeds thirteen"* | arithmetic — fourteen is |

**AND THE PRIOR ARC COUNTED FIVE OF THE SAME THING** and wrote the remedy down:
D-568's standing law (*a mutation set is specified against call sites enumerated
by a `git grep` receipt, never against prose*) and D-574's check (*print the
command WITH ITS SCOPE beside the claim, and compare its hit count to the prose's
before believing either*). **FOUR OF THAT ARC'S FIVE HAPPENED AFTER THE LAW WAS
WRITTEN.** This arc adds at least nine more, three of them in the very document
that restated the law.

**SO THE FINDING IS NOT "PEOPLE SHOULD CHECK". IT IS THAT THE CHECK CANNOT FIRE.**

---

## 2. WHY THE RULE FAILS, AND IT IS NOT CARELESSNESS

D-574's check is addressed to the author at the moment of writing, and asks them
to doubt a claim. **But the author does not experience these as claims.** *"gate
6"*, *"four seats"*, *"the dispatch says 3,500"* are reached for as LABELS for
things already believed — they are by-products of saying something else, not the
subject of the sentence. A rule that says *check before believing* cannot fire in
someone who does not notice they are believing anything.

**THE THREE PROPERTIES THAT MAKE THIS CLASS INVISIBLE**, each visible in the
sample above:

1. **The claim is subordinate.** Nobody wrote a document in order to say which
   gate number the determinism script is. It arrived while saying something else.
2. **It was true once.** Several were correct when first written and rotted when
   the tree moved — the seat count, the line ranges, the revision pointers. A
   rule aimed at the writing moment cannot reach a claim that decayed later.
3. **Restating multiplies it.** D-423 already says a claim made twice is a defect
   waiting; the copies do not get re-derived, and the copy nobody re-derived is
   the wrong one. Two of this arc's instances are exactly that.

**WHAT WOULD ACTUALLY WORK, DERIVED FROM THOSE THREE**: a mechanism that (1) makes
the claim the subject of its own sentence, (2) re-evaluates it on every CI run
rather than once at writing, and (3) makes a restatement cost something. **A
command with its expected output, executed by a gate, is all three at once.**

---

## 3. THE MECHANISM — A DERIVATION BLOCK

A fenced block whose info string is `derive`. First line is the command, prefixed
`$ `. The remaining lines are its expected stdout, byte for byte.

````
```derive
$ /usr/bin/grep -c '^gate_step "' tools/ci.sh
20
```
````

**THAT EXAMPLE IS DELIBERATELY NOT A LIVE BLOCK IN THIS DOCUMENT**, and the reason
is a rule the gate needs: **a `derive` block is a live claim from the moment it is
written.** The command above returns `0` until §5's implementation lands, so
writing it as a real block here would land a red gate to illustrate a form. An
illustration is shown inside an outer fence; a claim is a block.

The gate runs the command from the repository root and compares. A mismatch is a
**named refusal** quoting both, exit 1.

**THE FORM IS THE ARGUMENT.** A block cannot be written without naming the scope,
because the scope is the command's arguments. *"src is not the repository"* stops
being advice and becomes something a reader sees on the line.

### 3.1 WHAT A BLOCK MAY DO — AN ALLOWLIST, AND EVERYTHING ELSE IS REFUSED BY NAME

CI executing text out of Markdown is a code-execution surface, so the grammar is
closed (hard rule 3: wrong-shape input raises a named error, never a skip).

- **One command.** No `;`, `&&`, `||`, `` ` ``, `$(`, `>`, `>>`, `<`, `&`.
- **Pipes are allowed between allowlisted programs only**, so a count can be a
  count: `… | wc -l`, `… | LC_ALL=C sort -u | wc -l`.
- **The allowlist**, by program name: `/usr/bin/grep`, `git grep`, `git log`,
  `git rev-parse`, `git show`, `sed`, `awk`, `cut`, `sort`, `uniq`, `wc`, `head`,
  `tail`, `cat`, `sha256sum`, `python3 tools/…`, `tools/…`. Anything else is
  refused **naming the program**.
- **`grep` must be `/usr/bin/grep` or `git grep`**, never bare `grep`
  (CLAUDE.md's Environment section, D-265: the agent shell's `grep` is wrapped,
  multithreaded and order-nondeterministic).
- **Every path argument must be tracked.** `git ls-files --error-unmatch` on each
  path-looking argument; an untracked or absent path is a refusal naming it. This
  is what keeps the gate runnable on a fresh clone.
- **`LC_ALL=C`, cwd = repository root, no network, no environment inherited
  beyond `PATH` and `LC_ALL`.**

### 3.2 WHAT THE GATE DOES NOT DO, AND THIS IS THE LOAD-BEARING RESTRAINT

**IT DOES NOT POLICE FREE PROSE.** It checks blocks a document volunteers and
nothing else. Two reasons, and the second is decisive:

1. A regex over prose cannot tell a claim from a quotation from a record.
2. **This repository's documents are largely RECORDS.** A review report saying
   *"CI, 19 gates"* is a true statement about a run that happened. There are

```derive
$ git grep -ohE "gate [0-9]+/19|gate [0-9]+ of 19|19 gates|all 19" -- docs tools | wc -l
88
```

   such strings in the tree, and almost all of them are history.

   **THAT NUMBER WAS 84 IN THIS DOCUMENT'S FIRST DRAFT, AND THE BLOCK IS WHY IT IS
   NOW 88.** The 84 came from a shell command run four paragraphs earlier with the
   pathspec `'docs/**/*.md' 'tools/*'`; the block's own pathspec is `-- docs
   tools`, which reaches `docs/audit/`, `docs/research/` and `tools/`
   subdirectories the first one missed. **The author of a gate against scope drift
   drifted the scope between two runs of the same claim, minutes apart, and did
   not notice until the block was executed.** It is left in the document as the
   best evidence the design has: this is not a discipline problem. **A gate that
   went red on them would be demanding that records be falsified**, which is the
   opposite of the discipline it exists to serve.

**IT DOES NOT CHECK CLAIMS ABOUT RUNS.** A capture's record count, a wall time, a
mutation score — those live in artifacts, which are gitignored (hard rule 8), and
a gate that read them would pass or fail depending on what happened to be on the
disk. **The boundary is exact and it is the package's main design decision:
derivation blocks are for claims about the TREE; receipts are for claims about
RUNS**, and a receipt is bound by its sha256 in a committed manifest (D-469).

**IT DOES NOT MAKE A DOCUMENT CORRECT.** A block proves its command produced its
output. It cannot prove the command was the right one to run — that is what
review is for, and a reviewer's first move becomes *read the command*, which is a
far cheaper act than reconstructing the claim from scratch.

### 3.3 WHERE BLOCKS ARE OWED

**A rule, and it is narrow on purpose.** A document that GOVERNS something — a
pre-registration, a design, an option matrix, a dispatch — states its
load-bearing tree-claims as derivation blocks. Records (review reports, closures,
ledgers, session summaries) may use them and are never required to.

**AND A CLAIM WITHOUT A BLOCK IS NOT FORBIDDEN — IT IS VISIBLE.** That is the
whole enforcement model: after this gate, a governing document's tree-claim that
carries no block is a claim the author chose not to derive, in a document where
its neighbours are derived. A reviewer can see the difference on the page.

---

## 4. THE SECOND PART — CITATION EXISTENCE AND RANGE

Cheap, zero authoring cost, and it catches the rot that no block can: a
`` `path/to/file:NN` `` or `` `path:NN-MM` `` in a **governing** document whose
path is untracked or whose line number is past the end of the file.

It would not have caught `outpath.rs:9-24` (in range, wrong function). It would
catch every citation to a file that moved or vanished, which is the failure mode
that grows with the tree rather than with the author.

**SCOPED TO GOVERNING DOCUMENTS**, by the same argument as §3.2: a review report
citing a line that has since moved is a true record of what was there.

---

## 5. THE GATE ITSELF, AND THE NUMBER THAT IS ASSERTED NINETEEN TIMES

The check ships as `tools/derivation_check.sh` and runs as a new gate.

**AND ADDING IT EXPOSES THE SAME DEFECT IN `tools/ci.sh`.** The script asserts its
own gate total **nineteen separate times**:

```derive
$ /usr/bin/grep -cE '^step "gate [0-9]+/19' tools/ci.sh
19
```

Nineteen copies of one count, none derived — D-423's *"a claim the document makes
twice is a defect waiting"*, at nineteen. Adding a twentieth gate means editing
all nineteen, and the twenty-first means editing twenty.

**SO THE PACKAGE DERIVES IT.** `step` becomes `gate_step`, which takes only the
gate's NAME, counts itself from the script, and numbers by order:

```
GATE=0
TOTAL=$(/usr/bin/grep -cE '^gate_step "' "${BASH_SOURCE[0]}")
gate_step() { GATE=$((GATE + 1)); printf '\n=== gate %d/%d: %s\n' "$GATE" "$TOTAL" "$*"; }
```

**THE PRINTED LINE SHAPE IS UNCHANGED** — `=== gate N/M: <name>` — so every
consumer of it still reads. What changes is that `M` becomes 20 and stops being a
literal. **CLAUDE.md's own sentence** — *"it prints `gate N/19: <name>` as each
one runs"* — becomes false on landing and is corrected to `gate N/M` in the same
diff. **That correction is flagged to the operator rather than made silently**:
CLAUDE.md is the operator's file.

**AND EVERY HISTORICAL "19 gates" STAYS AS IT IS.** Those are records of runs
that had nineteen.

---

## 6. WHAT THIS PACKAGE OWES

| obligation | discharged by |
|---|---|
| REVIEW-design, fresh context, attacking the premise | a subagent, against this document |
| IMPL | `tools/derivation_check.sh`, the `ci.sh` numbering change, blocks retro-fitted to this arc's governing documents |
| REVIEW-impl, fresh context, not the implementer | a subagent, against this document |
| **RED-TEAM on the execution path** — the adversarial input here is a MARKDOWN BLOCK THAT RUNS AS A COMMAND, which is the most dangerous thing this repository has ever put in CI | a subagent, with a fixture per refusal |
| `docs/process.md`'s tools/ coverage rule: a test driving the SHIPPED script | `crates/pistol-arena/tests/derivation_check_tests.rs`, in the pattern `wp21_tranche_config_tests.rs` and `label_cache_count_tests.rs` set |
| a mutation set specified against call sites enumerated by a `git grep` receipt recorded in the mutation document (D-568) | the IMPL package; the sites do not exist yet and this line is not the receipt |
| an ADR line, recording the strongest surviving attack | at closure |

**THE RED-TEAM IS THE ONE THAT MATTERS AND ITS BRIEF IS WRITTEN HERE**: try to
execute something outside the allowlist from inside a Markdown file — through
quoting, through a path that escapes the repository, through a program name that
matches an allowlisted prefix, through a pipe to something unlisted, through an
argument that looks like a path and is not, and through a `python3 tools/…` that
names a script the block itself supplies. **A refusal that fires is a pass; a
command that runs is a FATAL.**

---

## 7. WHAT THIS DOES NOT CLAIM TO FIX

1. **The FATAL of the cache-key matrix is not in this class.** *"MEASURED"*
   applied to the wrong population is a true measurement with a false quantifier,
   and no command refutes it. What helps is adjacent and is worth stating: **a
   number that comes out of a derivation block carries its population in the
   command** — `--capture <the pilot's capture>` says thirteen openings on its
   face — so the population stops being prose. That is a mitigation, not a fix.
2. **It does not reduce the volume of prose**, which is the class's other
   multiplier. It makes restatement cost a block, which pushes the right way.
3. **It cannot make a wrong command look wrong.** Review still does that.
