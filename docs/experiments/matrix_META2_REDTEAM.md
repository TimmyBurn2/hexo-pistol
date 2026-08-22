# DECISION-RED-TEAM — MATRIX META-2 (`matrix_META2.md`)

**SUBJECT REVISION: `908a2f7`.** `docs/experiments/matrix_META2.md`.

**THE DISPATCH'S PIN ARRIVED UNFILLED** — it reads *"matrix_META2.md at <OPERATOR
FILLS SHA>"*. I resolved it myself rather than proceeding on an unpinned subject,
and I state what I resolved it to:

```
$ git log --oneline -- docs/experiments/matrix_META2.md | cut -c1-72
908a2f7 feat(tools): the carve documents' self-state becomes CI gate 15
$ git diff 908a2f7..HEAD -- docs/experiments/matrix_META2.md | wc -l
0
$ git rev-parse HEAD                    # at entry and at exit
75508dbe9b0c0fa14d05a5623b7b37fe68677881
$ git status --porcelain                # at entry
(no output)
```

The subject is byte-identical at `908a2f7` and at HEAD, so the mismatch between the
subject revision and HEAD does not reach any finding below.

**D-337 FREEZE: INTACT. NOT A VOID.**

```
$ git diff 908a2f7..HEAD --stat
 .../tests/label_consistency_check_tests.rs         | 187 +++++++++++--
 crates/pistol-solver/tests/wp15b_census.rs         |  99 ++++++-
 docs/decisions.md                                  |   6 +
 tools/label_consistency_check.sh                   | 305 ++++++++++++++-------
 4 files changed, 468 insertions(+), 129 deletions(-)
$ git diff 908a2f7..HEAD --stat -- docs/experiments/U1_gate_supersession.md \
    docs/experiments/U2_node_protocol.md docs/experiments/U3_tier_t.md \
    docs/experiments/U4_soundness_instrument.md docs/experiments/WPQ_seed.md \
    docs/experiments/section_owner_table.md
(no output)
```

**BUT THREE OF THE SIX SUBJECTS MOVED BETWEEN THE EVIDENCE AND THE MATRIX**, which is
not a freeze breach and is load-bearing for M2:

```
$ git diff 1f834ca..908a2f7 --stat -- docs/experiments/U1_gate_supersession.md \
    docs/experiments/U4_soundness_instrument.md docs/experiments/section_owner_table.md
 docs/experiments/U1_gate_supersession.md    | 7 ++++---
 docs/experiments/U4_soundness_instrument.md | 2 +-
 docs/experiments/section_owner_table.md     | 4 +++-
```

**Context was fresh.** I did not author this matrix, META-1, any unit, any review, any
selection record, gate 15 or any ADR line. Outside this report I edited no repository
file, staged nothing, created no worktree and ran no git write command in the live
tree. Two scratch git repositories were built under this session's scratchpad; nothing
below depends on them surviving, and every command that made them is printed.

---

# VERDICT

**4 KILL, 12 MAJOR, 4 MINOR** — derived, not asserted:

```
$ grep -c '^## K[0-9] (KILL)'   docs/experiments/matrix_META2_REDTEAM.md   # 4
$ grep -c '^## M[0-9]* (MAJOR)' docs/experiments/matrix_META2_REDTEAM.md   # 12
$ grep -c '^\*\*m[0-9] —'        docs/experiments/matrix_META2_REDTEAM.md   # 4
```

**NO SELECTION MAY BE TAKEN FROM THIS FIELD.** Three of the four rows survive wounded;
**the recommendation falls**, and it falls on the row's own registered conditions
rather than on its evidence base — which, unusually for this work package, largely
reproduces.

The recommended row **C-prime is not the C-prime the field inherits its precedent
from**. `D-338` defines C-prime as *"answer every meta finding by striking the sentence
through … reviews stay at full scope"*; this matrix's condition 3 reads *"score
unmarked text only"* and its row reads *"Marked text is never repaired by anyone"*.
Those are the two clauses that carry all of the row's claimed savings, and both are
inversions of the definition V4 prices (K1). Its condition 4 buys a gate that is
invariant under the failure mode the row's own cell names (K2). Applying its own
marking discipline to the surface that produced three of this class's measured
instances **turns CI gate 15 red — reproduced below** (K3). And measured against the
most recent landed review, C-prime converts a FAIL of 1 BLOCKING and 7 MAJOR into a
review with **zero scoreable findings and eight unrepairable defects still in the
document** (K4).

**`D-338`'s C-prime — strike, dispose, replace with nothing, full review scope — is
NOT killed by anything below.** It remains the row a revision 3 owes. What is killed
is this matrix's substitution.

---

# V-CELL REPRODUCTION TABLE (attack surface item 1)

The author's pre-flight declares: *"every source cited at a landed path (SHAs filled by
operator at commit), every flip clause future-only and checked unfired at authoring,
every numeric cell marked. Cells citing the red-team report carry its marks."*
**Three of its five clauses are false**: the SHAs were not filled (M3), the
flip-clause check could not have been run (M4), and two numeric thresholds carry no
mark (M5). The two that hold are that every cited path resolves in-tree, and that the
cells citing the report carry its marks.

| cell | verdict | note |
|---|---|---|
| **V1** eleven restatement instances in normative sections | **REPRODUCES EXACTLY** | The eleven rows are at `U3_tier_t.md:885–897`, the site list compresses faithfully, and all are live at HEAD. This is the field's strongest cell — see M8 for what the recommendation does with it |
| **V2** 54 under a stated 34; 11 failing rows under a stated 6 | **REPRODUCES EXACTLY** | Both counts re-derived below. The compression *"both landed claim inventories carry false self-counts"* is exact: u-rev 8's row count is right and its FAILS count is not |
| **V3** two head/foot instances; the class splits mechanical/semantic; the gate lands unconditionally | **REPRODUCES AS A QUOTATION, FALSE AS A CLAIM** | **M2** — it is THREE, landed at `bb64501` *before* this matrix landed and recorded at D-342. **M9** — the gate covers ZERO of V2's two instances, by the gate's own header. **K3, M10** — "lands unconditionally" landed broken |
| **V4** C-prime-shaped edits executed three times; prescribed by the u-rev 8 reviewer | **REPRODUCES FOR A DIFFERENT OPTION** | **KILL K1.** Both halves are true of `D-338`'s C-prime and neither is true of this matrix's |
| **V5** B and C survive wounded, NULL resurrected, C underrated | **REPRODUCES EXACTLY** | Verbatim against the report's per-option table |
| **V6** direction-only; counts turned upward; zero PASS at close | **REPRODUCES THE DIRECTION; THE PASS CLAUSE IS FALSE** | **M11.** The cited report MEASURED *"zero PASS"* false — three landed PASSes — and this cell transcribes it anyway. U1's LAST landed review is a PASS |

---

# FINDINGS

## K1 (KILL) — THE RECOMMENDED ROW IS NOT THE C-prime IT INHERITS ITS PRECEDENT FROM. `D-338` DEFINES C-prime WITH FULL REVIEW SCOPE AND A STRIKE-AND-DISPOSE MARK; THIS MATRIX INVERTS BOTH CLAUSES AND KEEPS THE PRECEDENT. V4 PRICES A DIFFERENT OPTION.

**The claims attacked, verbatim** (`matrix_META2.md:24–25`, `:43`, `:69–71`):

> V4 MEASURED (report): C-prime-shaped edits already executed three times in-tree;
> prescribed independently by the u-rev 8 reviewer.

> C-prime | WAIVE-AND-MARK: one add-only marking pass over all six documents. …
> **Marked text is never repaired by anyone; reviews score UNMARKED text only** …

> 3. Reviews after the pass: score unmarked text only; a marker is not a finding …

**The definition it cites — `docs/decisions.md` D-338, the stop ADR, landed at
`b223679`:**

```
$ grep -o 'THE DOMINANT MISSING ROW IS C-prime[^*]*' docs/decisions.md | fold -w 78 -s
THE DOMINANT MISSING ROW IS C-prime, WAIVE-AND-MARK
$ grep -o 'keep every document; answer every meta finding[^*]*' docs/decisions.md | fold -w 78 -s
keep every document; answer every meta finding by striking the sentence 
through and appending a fixed `WITHDRAWN AS FALSE AT u-rev N` token, never by 
rewording and never by re-deriving; reviews stay at full scope.
```

**And the prescription V4's second half names —
`wp15b_U4_REVIEW_urev8.md`, MAJOR 5, "Fix scope":**

```
$ sed -n '483,485p' docs/experiments/wp15b_U4_REVIEW_urev8.md
**Fix scope.** Strike it as the three siblings were struck, with the disposition stated
(re-derivation was the instrument, `wp15b_U4_REVIEW_urev7.md` MAJOR 2/3/4 found what it
did not reach), and replace it with nothing.
```

**Four divergences, each on a clause that carries cost:**

| | `D-338` C-prime / the in-tree precedent | this matrix's C-prime |
|---|---|---|
| review scope | *"reviews stay at full scope"* | *"score unmarked text only"* (condition 3) |
| disposition of marked text | struck, disposed, **replaced with nothing** — the mark IS the repair of a scored finding | *"never repaired by anyone"* — the mark PREVENTS the finding |
| trigger | reactive: *"answer every meta finding"*, incremental | proactive: *"one add-only marking pass over all six documents"* |
| predicate | *"known-false"* | *"self-referential, unverifiable, or known-false"* |

**The in-tree precedent, read rather than cited.** Every executed instance is a strike
inside a REPAIRED disposition — the mark records that a reviewer scored the text and
that the round answered it:

```
$ grep -c '~~' docs/experiments/U4_soundness_instrument.md
11
$ sed -n '1953p' docs/experiments/U4_soundness_instrument.md | cut -c1-104
- ~~**B1 / M3 — no matrix, then a matrix in which every option fell.**~~ **CLOSED AT u-rev 6 BY SELECTIO
```

**Why it breaks the row.** V4 is the recommendation's FIRST stated ground — *"already
works in-tree, independently prescribed"*. What works in-tree is a repair convention
executed after a review scored the text. What this matrix proposes is a scope
exclusion executed before a review reads it. They share a typographic act and nothing
else. The row's entire saving — *"shrinks the semantic review surface to unmarked
text"* — comes from the clause the precedent does not contain, and the precedent's own
authority (`D-338`) states the opposite clause by name.

**And it is a D-331 breach on its face.** C-prime's definition has landed in
`docs/decisions.md`; under R15 *"where a claim has landed in docs/decisions.md … THAT
is its home and no unit may restate it"*. This matrix restates it and changes it in the
restatement — which is `D-335`'s generator (1) and (2) together, committed in the row
the matrix recommends.

**Severity: KILL against C-prime as written. `D-338`'s C-prime is untouched.**

---

## K2 (KILL) — REGISTERED CONDITION 4's MARKER INVENTORY IS INVARIANT UNDER THE FAILURE MODE C-prime's OWN CELL NAMES. THAT IS THE CLAUDE.md VACUITY RULE, AND IT IS K2 OF THE PRIOR ROUND REPRODUCED ONE ROUND LATER.

**The two clauses attacked, verbatim** (`matrix_META2.md:43`, `:72–74`):

> Failure mode: **Marking pass misses instances** (it is authored by a session and
> sessions drift) … markers are grep-able, so the marker set itself joins the
> mechanical gate's coverage.

> 4. The mechanical gate extends to marker inventory **(count and placement)** in a
> follow-up commit with its own driving test.

**The rule it fails, verbatim from CLAUDE.md:**

> A criterion that is a property the named defect class PRESERVES — internal agreement
> between components sharing an input, output shape, plausible magnitude, exit status —
> passes vacuously and is not a criterion; it must be one that defect could falsify.

**Count and placement are both preserved by a missed marker.** A marker that was never
written is not in the count, is at no placement, and matches no grep. Detecting a
MISSING marker requires the true set of claims that ought to carry one — which is
exactly the semantic judgment the marking pass makes and the one thing no gate has. The
gate can tell you how many markers exist; the failure mode is about how many do not.

**This is verbatim the prior round's KILL.** `matrix_META1_REDTEAM.md` K2 killed D's
citation gate because *"the recorded defect is MISATTRIBUTION, and existence-checking
is invariant under it"*, and `D-338` records that KILL as *"CLAUDE.md's named failure —
a criterion resting on a property the defect class PRESERVES"*. Condition 4 is the same
sentence with `marker` substituted for `D-nnn`, in the field authored to succeed the
one that fell on it.

**The mitigation clause fails with it.** The row's failure-mode cell says the miss is
*"mitigated by add-only discipline … and by the flip clause below"*. Add-only
discipline constrains what a marker DOES, not whether one EXISTS, so it is invariant
under the miss too; and the flip clause is M5 below.

**Severity: KILL against registered condition 4 and against the row's stated
mitigation.**

---

## K3 (KILL) — MEASURED, REPRODUCED: AN ADD-ONLY MARKER PLACED ON THE SURFACE THAT PRODUCED THREE OF THIS CLASS'S INSTANCES TURNS CI GATE 15 RED. REGISTERED CONDITION 1 IS NOT EXECUTABLE THERE. THAT IS K5's CLASS, ONE ROUND LATER, WITH THE SIGN REVERSED.

The prior round killed condition 1 because *"deletion-only, applied literally to line 3
of each of the six documents, removes the carve marker and turns a CI gate red"*. This
round's condition 1 is add-only, and it reddens a different gate at a different line.

**Setup — a scratch repository, the gate and the six documents both at HEAD:**

```
$ S=<scratchpad>/g15b; mkdir -p $S/tools $S/docs/experiments
$ git show HEAD:tools/label_consistency_check.sh > $S/tools/label_consistency_check.sh
$ for f in U1_gate_supersession U2_node_protocol U3_tier_t U4_soundness_instrument \
           WPQ_seed section_owner_table; do
    git show HEAD:docs/experiments/$f.md > $S/docs/experiments/$f.md; done
$ cd $S && git init -q . && git add -A && git commit -qm base
$ ./tools/label_consistency_check.sh | tail -1; echo "exit=${PIPESTATUS[0]}"
label_consistency_check: 6 documents, 2 summand line(s), 5 summand row(s), 4 group count(s) — every document agrees with itself
exit=0
```

**The marking.** U1's closing line is a self-referential claim about the document's own
u-rev — the exact class C-prime marks, and the surface that produced three of the
class's measured instances (U1, `section_owner_table.md`, U4). Condition 2 requires the
marker to *"point at the truth's home"*; for a u-rev claim the home is a u-rev. The
marker below changes zero existing characters:

```
$ printf '\n*[[SELF-REFERENTIAL · home: `wp15b_U1_REVIEW_urev2.md`, the PASS against u-rev 2]]*\n' \
    >> docs/experiments/U1_gate_supersession.md
$ git add -A && git commit -qm markfoot
$ ./tools/label_consistency_check.sh; echo "exit=$?"
label_consistency_check: docs/experiments/U1_gate_supersession.md: the closing region below the rule at line 325 names 2 u-rev labels; exactly one is resolvable
label_consistency_check: docs/experiments/U1_gate_supersession.md: the fold law (docs/decisions.md D-331) is the repair — the closing line carries the LABEL and points at its home for the rest
…
exit=1
```

**The dilemma, and both horns are registered.** The gate refuses a closing region that
names more than one u-rev label — deliberately, because *"a closing line that also
recounts what each u-rev did names several u-revs, and there is then no fact of the
matter about which one is the document's own"* (`tools/label_consistency_check.sh`
header). A marker naming the home of a u-rev truth necessarily names a u-rev.

- **Mark the foot** → condition 1 reddens a CI gate, and condition 1 says *"Frozen-matrix
  rule and landed-evidence rule bind"* while saying nothing about the gate its own
  sibling condition 4 extends.
- **Do not mark the foot** → the mechanical and semantic classes overlap at the one
  surface where the class is measured three times, so **V3's split is false** and the
  matrix is not deciding "only the semantic remainder".

**Severity: KILL against registered condition 1, and MAJOR against V3's split.**

---

## K4 (KILL) — MEASURED: APPLIED TO THE MOST RECENT LANDED REVIEW, C-prime SCORES ZERO OF ITS EIGHT BLOCKING-AND-MAJOR FINDINGS. THE VERDICT BECOMES PASS, THE EIGHT DEFECTS STAY IN THE DOCUMENT, AND THE RULE FORBIDS REPAIRING THEM.

`wp15b_U4_REVIEW_urev8.md` is the corpus's newest review: FAIL, 1 BLOCKING / 7 MAJOR /
7 MINOR. Each finding's heading names the surface its defective claim sits on:

```
$ grep -n '^### [0-9]\.' docs/experiments/wp15b_U4_REVIEW_urev8.md | cut -c1-118 | head -8
153:### 1. The unit's own closing line still says **"u-rev 7"**, and three further status surfaces are one round behin
229:### 1. The head's change log says **"`D-329`'s relative-base residual"** — the exact misattribution MAJOR 3 charge
282:### 2. U4-Z asserts that D-331 **"does not reach"** and **"is silent about"** this class — D-331's own ground sect
338:### 3. **"Every universal in this unit's REVIEW STATUS table is withdrawn and none is replaced by a narrower one"*
395:### 4. U4-Z's OPEN list asserts **"WHICH CHECKLIST ITEMS N-E's `--config` ENGAGES IS STATED AT §9.1 AND NOWHERE EL
445:### 5. U4-Z's lead-in still asserts that at u-rev 7 **"every claim below whose truth depends on an ADR line was re
489:### 6. The REVIEW STATUS row for MINOR 7 says the second site **"cites rather than restates"** — it restates the f
525:### 7. **"u-rev 8 is one round in several commits"** inverts D-311's own ground — one label now names six texts di
```

**Every one of the eight sits in a closing line, a head change log, U4-Z, or a REVIEW
STATUS row.** Those are self-referential claims about the document's own state on any
reading of the marking predicate — they are the paradigm case, and V3 files two of the
same class under "mechanical". Under condition 3 they are **not scoreable**; under the
row's own *"marked text is never repaired by anyone"* they are **not repairable**.

**The consequence, stated plainly.** Run C-prime's marking pass before that review and
the review returns PASS with no findings, while U4 still tells an IMPL reader that
`D-329` records a residual it does not record, that `D-331` is silent about a class its
own ground section is built on, and that every universal in its REVIEW STATUS table has
been withdrawn when six survive. The first of those is the instance `D-335` fired
D-331's flip on. C-prime would have made it undiscoverable and permanent.

**And the scope shrinks monotonically.** Each round, findings in unmarked text are
answered by marking (never repairing), so the unmarked remainder can only contract. The
terminal state is a fully marked document reviewed at zero scope, PASSing vacuously.
That is C's rot — *"waived-false indistinguishable from true in the document of
record"*, the failure this matrix charges against C on line 42 — reached by rule
instead of by neglect, with a grep-able audit trail as the only difference. `D-338`'s
*"reviews stay at full scope"* is the clause that prevents exactly this, and condition
3 deletes it.

**Severity: KILL against registered condition 3 and against the row's dominance claim
over C.**

---

## M1 (MAJOR) — THE ESTIMATED "WELL UNDER HALF" CELL IS FALSE. MEASURED ON FOUR CONSTRUCTIONS, INCLUDING TWO BUILT TO FAVOUR IT: THE REMAINDER IS 64.0 %, 57.4 %, 58.4 % AND 48.0 %. (attack surface item 2)

**The cell attacked, verbatim** (`matrix_META2.md:43`):

> ESTIMATED remainder well under half of current review surface; attacker asked to
> measure.

**Method, stated because the predicate is not.** The matrix defines no marking
predicate beyond *"self-referential, unverifiable, or known-false"* (M8), so I measured
three readings of increasing generosity to the author and reported all of them. All
six documents are sampled — the whole corpus, not a sample of it.

- **R1, structural:** every head block (line 1 to the first `##`), every lineage
  section (`U*-A`) and every `U*-Z` section marked **wholesale**. Generous already:
  it marks the ADR-lines-owed subsections, which are normative.
- **R2, R1 plus every line anywhere else** matching `u-rev | this unit | this document
  | this table | this section | REVIEW STATUS | carve | WITHDRAWN | SUPERSEDED | OPEN |
  owes | owed`. Deliberately over-inclusive: it marks pointers, which D-331 says are
  not restatements.
- **R3, R2 plus the whole of U4 §8 and §9** (lines 297–1473) — the two selected design
  matrices, marked as "record". Indefensible, included to find the reading that saves
  the cell.

```
$ cat remainder.py
import re
D=["U1_gate_supersession","U2_node_protocol","U3_tier_t",
   "U4_soundness_instrument","WPQ_seed","section_owner_table"]
SR=re.compile(r'u-rev|this unit|this document|this table|this section|REVIEW STATUS'
              r'|carve|WITHDRAWN|SUPERSEDED|OPEN\b|owes|owed', re.I)
tot=r1=r2=r3=tw=w1=w2=0
for d in D:
    L=open(f"docs/experiments/{d}.md").read().split("\n")
    if L and L[-1]=="": L=L[:-1]
    n=len(L); tot+=n
    secs=[(i+1,l) for i,l in enumerate(L) if re.match(r'^## ',l)]
    cov=set(range(1,(secs[0][0] if secs else n+1)))          # head block
    for i,(ln,t) in enumerate(secs):                          # lineage and Z
        if re.match(r'^## (U\d-A\.|.*Lineage)',t) or re.match(r'^## U\d-Z\.',t):
            cov|=set(range(ln,(secs[i+1][0]-1 if i+1<len(secs) else n)+1))
    cov2=cov|{i for i in range(1,n+1) if SR.search(L[i-1])}
    cov3=cov2|(set(range(297,1474)) if d=="U4_soundness_instrument" else set())
    r1+=len(cov); r2+=len(cov2); r3+=len(cov3)
    for i in range(1,n+1):
        w=len(L[i-1].split()); tw+=w
        if i in cov: w1+=w
        if i in cov2: w2+=w
for nm,m in (("R1 structural meta only",r1),
             ("R2 + every self-ref line anywhere",r2),
             ("R3 + all of U4 §8 and §9",r3)):
    print(f"{nm:38s} marked={m:5d} ({100*m/tot:5.1f}%)   REMAINDER={tot-m:5d} ({100*(tot-m)/tot:5.1f}%)")
print(f"corpus = {tot} lines, six documents")
print(f"words total={tw}  R1 marked={w1} ({100*w1/tw:.1f}%) remainder {100*(tw-w1)/tw:.1f}%")
print(f"              R2 marked={w2} ({100*w2/tw:.1f}%) remainder {100*(tw-w2)/tw:.1f}%")

$ python3 remainder.py
R1 structural meta only                marked= 1807 ( 36.0%)   REMAINDER= 3212 ( 64.0%)
R2 + every self-ref line anywhere      marked= 2139 ( 42.6%)   REMAINDER= 2880 ( 57.4%)
R3 + all of U4 §8 and §9               marked= 3171 ( 63.2%)   REMAINDER= 1848 ( 36.8%)
corpus = 5019 lines, six documents
words total=59788  R1 marked=24888 (41.6%) remainder 58.4%
              R2 marked=31079 (52.0%) remainder 48.0%
```

The script is reproduced in full above rather than summarised, so that a reviewer can
re-run it or refute the regions it draws. It is this report's instrument for M1 and it
lives only in this report; there is no `tools/` artefact behind the number.

The corpus:

```
$ for f in U1_gate_supersession U2_node_protocol U3_tier_t U4_soundness_instrument \
           WPQ_seed section_owner_table; do wc -l docs/experiments/$f.md; done
330 docs/experiments/U1_gate_supersession.md
925 docs/experiments/U2_node_protocol.md
990 docs/experiments/U3_tier_t.md
2105 docs/experiments/U4_soundness_instrument.md
227 docs/experiments/WPQ_seed.md
442 docs/experiments/section_owner_table.md
```

**The result.** On the honest readings the remainder is **a clear majority** — 64.0 %
of lines, 58.4 % of words. On the most over-inclusive line reading I could defend it is
still **57.4 %**. Only the words reading of R2 dips below half, and it dips to **48.0
%**, which is "just under" and not "well under". The single construction that produces
"well under half" is R3, and R3 marks U4's adopted soundness instrument and its adopted
config seam — the unit's actual specification — as never-reviewed and never-repairable.

**Charged, as the dispatch directs.** The cell is the author's own ESTIMATED number, it
is the only sizing the recommendation offers, it could have been measured in seconds by
its author, and it is wrong in the direction that favours the recommendation. That is
D-291's finding and D-318's pattern together: an unmeasured estimate that moves toward
its author's row.

**Severity: MAJOR against C-prime's cost cell.**

---

## M2 (MAJOR) — V3's "TWO" WAS ALREADY THREE WHEN THE MATRIX LANDED. THE THIRD INSTANCE LANDED AT `bb64501`, AN ANCESTOR OF THE MATRIX'S OWN COMMIT.

**The cell attacked, verbatim** (`matrix_META2.md:18–19`):

> V3 MEASURED (report): **two** head/foot label instances found by a one-line shell
> loop.

```
$ git log --oneline 1f834ca..908a2f7 -- docs/experiments/U4_soundness_instrument.md
bb64501 docs(experiments): U4's foot label reaches u-rev 8 — a THIRD live instance of the class, missed by the red-team's 3-line-tail loop …
$ git merge-base --is-ancestor bb64501 908a2f7 && echo "bb64501 IS an ancestor of the matrix"
bb64501 IS an ancestor of the matrix
```

D-342 records the correction: *"IT IS THREE. `U4_soundness_instrument.md` was head
`u-rev 8` against foot `u-rev 7` for the whole freeze window and M3 did not report
it … measured against `1f834ca` it refuses three documents where the loop found two."*
D-342 landed after the matrix, but **the instance did not** — it landed two commits
before it, in the same session, and the gate the same commit shipped states the
corrected number in its own header.

**And the report's own body already said three.** `matrix_META1_REDTEAM.md` M3:
*"Three of the six frozen documents fail one line of shell; two of the three failures
are recorded by nobody."* The cell transcribes the heading's "two", which is the count
of UNRECORDED failures, as the count of INSTANCES.

**Why it matters beyond the digit.** The size was load-bearing in `D-338`'s ranking of
R4 against the citation gate, and it is load-bearing here: V3's whole office is to
carve the mechanical subclass out of the matrix's scope. A subclass measured at two
when it is three, by an instrument whose second spelling was itself broken (M10), is
not a settled carve-out.

**Severity: MAJOR against V3, and it is D-336 clause (4) — the summary taken over the
paste, inside the very cell the matrix marks MEASURED.**

---

## M3 (MAJOR) — THE PRE-FLIGHT DECLARES SHAS FILLED AT COMMIT. THE ONLY SHA IN THE DOCUMENT IS STILL THE PLACEHOLDER, AT THE LANDING REVISION AND AT HEAD.

```
$ git show 908a2f7:docs/experiments/matrix_META2.md | grep -n '<SHA>'
11:## Evidence (docs/experiments/matrix_META1_REDTEAM.md at <SHA>, unless noted)
$ grep -n '<SHA>' docs/experiments/matrix_META2.md
11:## Evidence (docs/experiments/matrix_META1_REDTEAM.md at <SHA>, unless noted)
```

The declaration reads *"every source cited at a landed path (SHAs filled by operator at
commit)"*, and the status line reads *"Land at SHA after the slack session, so every
citation below resolves in-tree first"*. The path resolves; the revision was never
supplied, and the matrix landed anyway.

**This is not pedantry, because the cited text moves.** D-336 clause (1): *"a cited
number, quotation or command output is transcribed from a FILE IN THE TREE, and the
citation names that file — and names its revision wherever the cited text can move."*
The cited report is a landed markdown file that a later round may amend, and its
content has already been amended in substance by D-342 — which is precisely M2. Six V
cells hang on one unpinned citation.

**Severity: MAJOR — a D-336 breach on the matrix's own evidence header, declared
discharged by a pre-flight that did not check it.**

---

## M4 (MAJOR) — "EVERY FLIP CLAUSE … CHECKED UNFIRED AT AUTHORING" IS A CHECK THAT COULD NOT HAVE BEEN RUN. FLIP CLAUSE 2 QUANTIFIES OVER "MARKED TEXT", WHICH HAS NO DEFINITION AT AUTHORING — CONDITION 2 DEFERS IT, AND MEASURED, THE DESIGNATED HOME IS EMPTY.

**The clauses attacked, verbatim** (`matrix_META2.md:6–9`, `:60–61`, `:67–68`):

> every flip clause future-only and **checked unfired at authoring**
>
> - If any session repairs **marked text**, the discipline is broken …
>
> 2. **Marker syntax defined once, in the owner table header (its one home)** …

```
$ grep -n 'marker' docs/experiments/section_owner_table.md
154:| 1 | Snapshot before/after, the registered above-marker quantity | **U4** |
```

One hit, unrelated. No marker syntax exists in the designated home, so at authoring
time "marked text" denotes nothing, and a clause quantifying over nothing cannot be
"checked unfired" — it is unfired by vacuity, which is not a check.

**And the vacuity is not stable, because V4 says marked text already exists.** V4
counts three in-tree executions of the marking; `grep -c '~~'` returns 11 lines in U4 alone
(K1). If those are marked text — and V4's whole argument is that they are — then flip
clause 2 quantifies over the repair history of seven existing marks, which no command
in this matrix checks. Either V4 is wrong about the precedent or the pre-flight is
wrong about the check.

**Severity: MAJOR against the pre-flight declaration and against flip clause 2's
testability.** This is K3's class from the prior round — a flip clause whose state at
authoring was asserted rather than established — recorded before it fires rather than
after.

---

## M5 (MAJOR) — FLIP CLAUSE 1's "THREE OR MORE" IS INVENTED, UNMARKED AND DENOMINATOR-FREE, AND ITS TRIGGER IS ROUTED AWAY BY CONDITION 3. IT IS M6 OF THE PRIOR ROUND, SPELLED IN WORDS INSTEAD OF DIGITS.

**The clause attacked, verbatim** (`matrix_META2.md:56–59`):

> If the first post-marking review finds **three or more** unmarked instances in text
> that existed at marking time, the pass was incomplete: a SECOND marking pass by a
> different session, never repairs.

**(a) The number is invented.** It carries no mark. The pre-flight's *"every numeric
cell marked"* is defeated by spelling the number in words:

```
$ grep -n 'three or more\|two consecutive' docs/experiments/matrix_META2.md | cut -c1-96
41:| B | Reviewer claim-inventory institutionalized as the review deliverable, landed in-tree pe
56:- If the first post-marking review finds three or more unmarked
```

Neither threshold is MEASURED or ESTIMATED. The prior round's M6 charged exactly this
against *">20 percent"* and `D-338` recorded *"MEASURED/ESTIMATED marks on every
number"* among the seven things a revision 2 owes.

**(b) The denominator is unstated.** Three per review? Per document? Across the six? A
review of one unit and a review of the corpus have very different base rates: the
newest review found eight blocking-and-major findings in one document.

**(c) It licenses permanent defects.** Two unmarked instances found is BELOW the
threshold, so the pass is declared complete — and the two are then marked, which under
the row's own rule makes them never repairable by anyone. The threshold's practical
content is *"up to two real defects per round become permanent"*.

**(d) Its trigger is routed away by its own sibling condition.** Condition 3:
*"marker-coverage disputes go to the mechanical gate's inventory, not to prose
findings"*. A reviewer's claim that some text should have been marked and was not IS a
marker-coverage dispute. So the finding that fires flip clause 1 is the finding
condition 3 forbids the reviewer to raise, and routes to the gate that K2 measures
invariant under it.

**The honest criterion, offered.** The marking pass asserts a universal — *"every
self-referential, unverifiable, or known-false claim gets an inline marker"* — and a
universal is falsified by one counterexample, not by three. Replace the clause with:

> Before the marking pass lands, a second party who did not author it draws a fixed
> random sample of N lines from each of the six documents at the marking commit,
> classifies each against the marking predicate WITHOUT seeing the markers, and the
> classification is then joined to the marker set. **Registered consequence, before
> either party runs:** any line the blind classifier marks and the pass did not means
> the pass is incomplete and a second pass runs; a disagreement rate the two parties
> cannot reconcile means the PREDICATE is undefined and the row is withdrawn rather
> than re-passed.

That referent is externally derived — the classifier does not share the marking
session's input — and it can falsify the named defect, which "count and placement"
cannot.

**Severity: MAJOR against flip clause 1.**

---

## M6 (MAJOR) — B's "STABILIZE ACROSS TWO CONSECUTIVE ROUNDS" HAS NO DEFINED COMPARISON. MEASURED: THE TWO LANDED INVENTORIES HAVE DISJOINT ROW NAMESPACES, DIFFERENT SCOPES, DIFFERENT AUTHORS, AND THE SECOND STATES IT DID NOT REUSE THE FIRST. (attack surface item 5)

**The criterion attacked, verbatim** (`matrix_META2.md:41`):

> repair until inventories **stabilize across two consecutive rounds**.

**The universe it quantifies over — the entire landed population is two:**

```
$ grep -ln 'CLAIM INVENTORY' docs/experiments/wp15b_*REVIEW*.md
docs/experiments/wp15b_U4_REVIEW_urev7.md
docs/experiments/wp15b_U4_REVIEW_urev8.md
```

Both are against U4, both scoped to U4-Z alone. **No landed inventory exists for U1,
U2, U3, `WPQ_seed.md` or `section_owner_table.md`** — five of the six subjects.

**Their row identities do not intersect:**

```
$ awk 'NR>=342 && NR<=421' docs/experiments/wp15b_U4_REVIEW_urev7.md | grep -o '^| Z[0-9]*' | tr -d '| ' | tr '\n' ' '
Z1 Z2 Z3 … Z54
$ sed -n '769,846p' docs/experiments/wp15b_U4_REVIEW_urev8.md | grep -o '^| Y[0-9]*' | tr -d '| ' | tr '\n' ' '
Y1 Y2 Y3 … Y61
$ grep -n 'did not reuse' docs/experiments/wp15b_U4_REVIEW_urev8.md
764:I did not take the round's account of U4-Z on trust and did not reuse the prior
```

**So there is nothing to compare.** Two hand-built tables, `Z1–Z54` against `Y1–Y61`,
built by two fresh contexts that share no row identity, over a document that grew by
219 lines between them. "Stable" is undefined over that pair on any relation but the
stated headline counts — **and V2 is the finding that both headline counts are false**:

```
$ awk 'NR>=342 && NR<=421' docs/experiments/wp15b_U4_REVIEW_urev7.md | grep -c '^| Z'
54
$ sed -n '344p' docs/experiments/wp15b_U4_REVIEW_urev7.md | grep -o '\*\*Result:.*'
**Result: 34 claims examined, 31 hold, 3 fail** (MAJOR 2, 3, 4 above).
$ sed -n '769,846p' docs/experiments/wp15b_U4_REVIEW_urev8.md | grep '^| Y' | grep -c 'FAILS'
11
$ sed -n '767p' docs/experiments/wp15b_U4_REVIEW_urev8.md
**Result: 61 claims examined, 55 hold, 6 fail.**
```

The only handle the criterion could grasp is the one the criterion's own row records as
corrupt. That is a criterion resting on a property the named defect class does not
merely preserve but produces.

**Severity: MAJOR against row B's registered criterion. B survives — see the survival
table — but not with this criterion.**

---

## M7 (MAJOR) — THE MARKER-AS-TEXT REGRESS IS NOT CLOSED. IT IS RELOCATED, AND IT ACQUIRES A NEW UNREPAIRABLE CLASS. (attack surface item 4)

The dispatch names this the strongest known attack line. It holds, in three parts.

**(a) A marker is either marked text or it is not, and both answers are fatal.**
The row says *"Marked text is never repaired by anyone"*; condition 3 says *"a marker
is not a finding"*.

- If the marker is part of the marked text, then a marker that names the WRONG home is
  frozen by rule. The corpus already contains the instance: `wp15b_U4_REVIEW_urev8.md`
  MINOR 7 — *"§8.7:919's present-tense 'so S-E **is** gate (b)'s instrument' is marked
  only 21 lines below it"* — a finding whose entire content is that an existing marker
  is misplaced.
- If the marker is not marked text, then it is unmarked text, condition 3 says reviews
  score unmarked text, and *"a marker is not a finding"* contradicts it directly.

Either way, the one recorded instance of a defective marker in this corpus is a finding
the C-prime contract cannot accept.

**(b) The home a marker points at drifts, measured.** Condition 2: the marker *"points
at the truth's home"*, one of ADR, report, git.

```
$ grep -o '^D-3[0-9][0-9]: \*\*[^*]*\*\*' docs/decisions.md \
    | grep -ciE "FLIP CLAUSE|CORRECT|SUPERSED|FALSIFIED|DEMOTED|NARROWED|AMENDED"
10
```

Ten ADR lines in the D-3xx range exist to correct or supersede an earlier one —
D-312→D-326, D-316→D-325, D-321→D-322, D-328→D-330, D-331→D-335, D-336→D-339,
D-338→D-342, D-341→D-342, D-343 narrowing itself twice. A marker whose home is `D-331`
points at a line `D-335` records as incomplete. Markers do not escape drift by being
pointers; they inherit the drift of what they point at, and unlike a claim, a marker is
never re-read because reviews are forbidden to score it.

**(c) The syntax's own home is a measured drifter.** Condition 2 puts the marker syntax
in the owner table header. That header's own words:

```
$ sed -n '20,22p' docs/experiments/section_owner_table.md | cut -c1-96
its u-rev. **THIS TABLE IS AT u-rev 6**: the `now` column is DELETED and replaced by
the command that derives it, discharging by deletion the standing duty that had gone
stale three times; §11's question is re-posed on the measured u-rev-6 numbers; and the
```

*"had gone stale three times"* — in the paragraph condition 2 designates as the one
home of the syntax that is supposed to end staleness. The table's own foot label went
stale at `4fd88ec` and was repaired at `8350bbc`.

**The answer to item 4, stated plainly: NO.** Condition 4's mechanical inventory does
not close the regress. It counts markers, which V2's defect class (a false claim about a
document's own state) preserves; it cannot check what a marker says or whether its home
still holds; and it puts the syntax's definition in the document with the corpus's
worst measured record for exactly this class. The regress is reproduced one level up,
and the new level is worse than the old one, because the old level was reviewable.

**Severity: MAJOR — jointly with K2, this is what carries C-prime's failure-mode cell
from "mitigated" to unmitigated.**

---

## M8 (MAJOR) — THE MARKING PREDICATE IS UNDEFINED AND HAS TWO DEFENSIBLE READINGS THAT ANSWER OPPOSITELY ON V1's ELEVEN INSTANCES. ON ONE, C-prime REACHES NONE OF THEM; ON THE OTHER, IT FREEZES ELEVEN LIVE NORMATIVE DESIGN NUMBERS AS UNREPAIRABLE. (attack surface item 3, claim 1)

**The predicate, in full** (`matrix_META2.md:43`): *"Every self-referential,
unverifiable, or known-false claim"*. It is defined nowhere else in the document.

V1's eleven are derived second copies of a pinned census cell — U3 §6.1, §6.2, §6.3
(twice), §6.5, §10 (twice), U3-M item 4, U2 §5.3, U4 §8.4:

```
$ sed -n '890,891p' docs/experiments/U3_tier_t.md | cut -c1-92
  | §6.3, option C's cost cell | `6.83` | `option C — Tier T outside the r2 ball` |
  | §6.3, option C's failure-mode cell | `23.2` | **NOT ONE CELL (MAJOR 1, `wp15b_U3_REVI
$ sed -n '327p' docs/experiments/U3_tier_t.md | cut -c1-150
| **C — ≥2 for us, ≥3 for them** | The lemma in §6.4 | see the census block; **MEASURED 29 % of C's Tier T lies OUTSIDE the radius-2 ball** (6.83 cell
```

- **Reading A — the predicate is read as written.** A derived second copy that is
  currently TRUE is not self-referential (it is about the search, not the document), not
  unverifiable (the census block verifies it), and not known-false. So the marking pass
  **reaches none of the eleven** — which is the identical reach failure K1 scored KILL
  against A and D one round earlier. V1 then supports no row in this field, and the
  recommendation's *"dominates the strip family on the dead options' own grounds"* rests
  on a cell C-prime does not touch.
- **Reading B — "self-referential" is stretched to cover derived copies.** Then the
  option-C cost cell of the matrix U3 ADOPTED, the `29 %` in the same row, U2 §5.3's
  `70.8 %` and U4 §8.4's `70.8 %` are all marked — **never repaired by anyone, and never
  scored by any review**. When the census moves, the adopted option's cost cell in the
  governing options table goes stale by design and no rule permits its correction.

The prior round killed a threshold for returning opposite verdicts on two honest
readings (M6). This is the recommendation's defining predicate doing the same thing, and
neither reading leaves the recommendation standing.

**Severity: MAJOR, and it is the reason C-prime cannot be repaired by amending a
condition — the predicate is upstream of all four conditions.**

---

## M9 (MAJOR) — V3's SPLIT PUTS "SELF-COUNTS" IN THE MECHANICAL SUBCLASS. THE GATE COVERS ZERO OF V2's TWO INSTANCES, BY THE GATE'S OWN HEADER. (attack surface item 3, claim 2)

**The cell attacked, verbatim** (`matrix_META2.md:19–23`):

> The defect class splits: a MECHANICAL subclass (label agreement, **self-counts**) and
> a SEMANTIC remainder. The mechanical gate … lands unconditionally; this matrix decides
> only the semantic remainder.

**The gate's own account of its scope:**

```
$ git show 908a2f7:tools/label_consistency_check.sh | sed -n '34,40p'
# WHAT THE SECOND CHECK IS FOR. `matrix_META1_REDTEAM.md` M2: both landed claim
# inventories ship a headline count of their own table that their own table
# falsifies — fifty-four rows under a stated thirty-four, eleven failing rows
# under a stated six — uncaught by every round including the reviewer who read
# the earlier one closely. Those two live in REVIEW REPORTS, which are outside
# this gate's subject and are a reviewer's own text this project does not edit.
# Inside the subject the same form appears twice over, and it is checked here:
```

The gate says it in its own comment: **V2's two instances are outside its subject.**
The self-counts it does cover are two summand lines and four group counts inside the
owner table — a different population from the one V2 measures. So *"self-counts"* names
a subclass whose measured instances the mechanical half does not reach, and B's
failure-mode hedge *"only partly covered by the mechanical gate"* understates it: the
coverage of V2 is zero.

**This is the third concrete case for item 3.** A marker on a review report's headline
count cannot be placed under this matrix at all: review reports are not among the six
documents, condition 1 scopes the pass to *"all six documents"*, and `D-336` clause (5)
plus this project's convention hold that a reviewer's text is not edited. The instance
that defines V2 is unmarkable, unrepairable and out of scope simultaneously.

**Severity: MAJOR against V3 and against B's failure-mode cell.**

---

## M10 (MAJOR) — "THE MECHANICAL GATE … LANDS UNCONDITIONALLY" AND "TOOLING IS MEASURED TO WORK (V3)" WERE FALSE AT THE MATRIX'S OWN LANDING SHA. REPRODUCED INDEPENDENTLY.

**The claims attacked** (`matrix_META2.md:21–23`, `:51–52`):

> The mechanical gate (tools/label_consistency_check.sh, slack session) lands
> unconditionally …
>
> converts marker bookkeeping into the mechanical class **where tooling is measured to
> work (V3)**.

The gate landed **in the matrix's own commit**, `908a2f7`. `D-344` records the
fresh-context REVIEW-impl against that revision: FAIL, 1 BLOCKING / 3 MAJOR / 1 MINOR,
the BLOCKING being *"the gate printed `OK` and exited 0 on the defect it exists for"*.
I did not take that on trust. **Reproduced, in a scratch repository, against the gate
and the documents both at `908a2f7`:**

```
$ S=<scratchpad>/g15repo; mkdir -p $S/tools $S/docs/experiments
$ git show 908a2f7:tools/label_consistency_check.sh > $S/tools/label_consistency_check.sh
$ for f in U1_gate_supersession U2_node_protocol U3_tier_t U4_soundness_instrument \
           WPQ_seed section_owner_table; do
    git show 908a2f7:docs/experiments/$f.md > $S/docs/experiments/$f.md; done
$ cd $S && git init -q . && git add -A && git commit -qm base
$ ./tools/label_consistency_check.sh | tail -1; echo exit=0     # CONTROL
label_consistency_check: 6 documents, 2 summand line(s), 4 group count(s) — every document agrees with itself

# D-344 BLOCKING-1's shape: the TRUE closing paragraph goes stale, and a trailing
# italic aside naming the HEAD's u-rev follows it.
$ perl -0pi -e 's/\*U3, u-rev 7\./*U3, u-rev 6./' docs/experiments/U3_tier_t.md
$ printf '\n*Filed under u-rev 7.*\n' >> docs/experiments/U3_tier_t.md
$ git add -A && git commit -qm mutate
$ ./tools/label_consistency_check.sh; echo "exit=$?"
label_consistency_check: self-test passed — a clean pair, a stale foot, an ambiguous foot, a WRAPPED foot, a summand line and two group counts
…
label_consistency_check: docs/experiments/U3_tier_t.md                  head=u-rev 7   foot=u-rev 7   OK
…
label_consistency_check: 6 documents, 2 summand line(s), 4 group count(s) — every document agrees with itself
exit=0
```

**`head=u-rev 7 foot=u-rev 7 OK`, exit 0, on a document whose closing paragraph says
u-rev 6** — and the self-test line above it announces that it covers *"a stale foot"*.

**Recorded in the gate's favour: HEAD repairs it.** Same mutated tree, gate at HEAD:

```
$ git show HEAD:tools/label_consistency_check.sh > tools/label_consistency_check.sh
$ ./tools/label_consistency_check.sh; echo "exit=$?"
label_consistency_check: docs/experiments/U3_tier_t.md: the closing region below the rule at line 985 names 2 u-rev labels; exactly one is resolvable
label_consistency_check: FAIL: 1 self-state disagreement(s) above …
exit=1
```

**Why it breaks the recommendation.** *"Tooling is measured to work"* is the fourth of
five grounds. At the matrix's landing SHA the tooling was measured NOT to work, by a
fresh reviewer and now by me. The class has taken three spellings — three-line tail
(missed U4, D-342), last-asterisk-line (exit-0-wrong-answer, D-344), closing region
(current) — and **each of the first two shipped and was caught only by a fresh
adversary**. That is not a class where tooling is measured to work; it is a class where
every spelling so far has shipped a wrong answer, and condition 4 proposes to hang the
marker inventory on the fourth.

**Severity: MAJOR against the recommendation's fourth ground and against condition 4's
premise.**

---

## M11 (MAJOR) — V6 TRANSCRIBES "ZERO PASS AT CLOSE", WHICH THE CITED REPORT MEASURED FALSE. U1's LAST LANDED REVIEW IS A PASS.

**The cell attacked, verbatim** (`matrix_META2.md:28–30`):

> V6 direction-only … finding counts turned upward at the last round, **zero PASS at
> close**.

**The source it cites, `matrix_META1_REDTEAM.md` M4:** *"'Zero PASS at close' is false
across the corpus … Three landed PASSes."* Re-run:

```
$ grep -rn "^\*\*\?PASS\|VERDICT.*PASS" docs/experiments/wp15b_*REVIEW*.md \
    | grep -iv "passed\|passes\|pass a\|does not" | cut -c1-64
docs/experiments/wp15b_trackC_REVIEW_impl.md:709:**VERDICT: PASS
docs/experiments/wp15b_trackC_R19_REVIEW_impl.md:155:**PASS.**
docs/experiments/wp15b_U2_REVIEW_urev2.md:69:- **u-rev labels, r
docs/experiments/wp15b_U1_REVIEW_urev2.md:19:## VERDICT: **PASS*
docs/experiments/wp15b_U2_REVIEW_urev3.md:147:- **Fold-in / stal
$ ls docs/experiments/wp15b_U1_REVIEW*.md
docs/experiments/wp15b_U1_REVIEW.md  docs/experiments/wp15b_U1_REVIEW_urev2.md

Five hits, three of them verdicts — the two `U2` lines are checklist prose, not
verdict blocks. The three verdicts are `wp15b_U1_REVIEW_urev2.md`,
`wp15b_trackC_R19_REVIEW_impl.md` and `wp15b_trackC_REVIEW_impl.md`. (The report's
own paste of this command returned three lines; the corpus has grown since, which is
RJ3's lesson in the prior round and is why I print what I ran.)
```

U1 has two landed reviews and the LATER one is the PASS. On the reading where "at
close" means the corpus's most recent state, one of the four units closes on a PASS.

The cell hedges the SERIES (*"middle terms unreconciled"*) and marks V6 direction-only,
which is right and is what D-341 rules. It then re-asserts the PASS clause, which is
the half D-341 and M4 both record as false rather than unreconciled. Marking a cell
"direction-only" does not license carrying its falsified half forward as content.

**Severity: MAJOR against V6, and it lands on the NULL row's failure-mode cell, which
cites V6 first.**

---

## MINOR

**m1 — the E6 demotion is attributed to the wrong ADR line.** V6 reads *"marked as such
per the stop ADR"*. The stop ADR is D-338; the demotion is D-341's ruling, which D-338
carries by pointer (*"E6 is demoted to direction-only by D-341"*). Defensible as a
pointer-to-a-pointer; recorded because M5 of the prior round was the same shape on the
matrix's second line.

**m2 — the matrix landed as a rider inside a `tools/` feature commit.** `908a2f7`'s
subject is *"feat(tools): the carve documents' self-state becomes CI gate 15"*. Under
D-337 the freeze window opens at the matrix's landing SHA; buried in a tools commit,
the window's start is invisible on the log's face, and CLAUDE.md's *"One feature = one
commit"* is not met. It also means the gate V3 cites as an independent settled fact and
the matrix citing it were the same act.

**m3 — condition 1's "One document per commit" is an unmarked operational number that
collides with a rule the matrix does not name.** A six-commit marking pass writing
cross-document markers is exactly D-332/R17's subject — serial landing, or every
cross-document citation re-checked at commit HEAD, and *the commit states which*.
Condition 1 states neither.

**m4 — the dispatch's subject pin arrived as `<OPERATOR FILLS SHA>`.** Recorded at the
head of this report with the SHA I resolved it to. Not a finding against the matrix.

---

# PER-OPTION SURVIVAL VERDICTS

| row | verdict | reason |
|---|---|---|
| **B INVENTORY** | **SURVIVES WOUNDED — CRITERION MUST BE REPLACED** | Nothing here kills the row. Its registered criterion is not well-defined (M6): two landed inventories, disjoint namespaces, one section of one document, and the only comparable handle is the number V2 measures false. Selectable only with the criterion rewritten and the instrument extended past U4-Z |
| **C RE-SCOPE** | **SURVIVES WOUNDED, UNCHANGED** | Carried faithfully from the prior round; nothing in this matrix improves or worsens it. The prior round's quoted attack still stands verbatim |
| **C-prime (AS WRITTEN HERE)** | **FALLS** | K1 (it is not the C-prime whose precedent it prices), K2 (its gate is vacuous against its own named failure mode), K3 (its condition 1 reddens a CI gate on the class's own surface), K4 (it scores 0 of 8 blocking-and-major findings of the newest review and forbids repairing them), M1 (its only sizing is false on every reading), M7, M8 (its predicate is undefined and answers oppositely on the field's strongest cell) |
| **`D-338`'s C-prime (NOT IN THIS FIELD)** | **UNTOUCHED — STILL OWED** | Strike, dispose, replace with nothing, reviews at full scope. None of K1–K4 reaches it; they all reach the substitutions |
| **NULL** | **SURVIVES WOUNDED — AND THE WOUND IS OVERSTATED** | Confirmed per item 8. Its cell says *"generators 2 and 3 unreached by any clause"*; D-342 is a landed instance of both being reached, recorded and answered with a new practice clause, and gate 15 is a landed mechanism against generator (2). Since the META-1 stop the project has run this row and it produced the field's only mechanism — and the discovery of that mechanism's own defect (D-344) |
| **THE RECOMMENDATION** | **FALLS** | Four of its five grounds fail: V4 prices a different option (K1); *"fixes C's rot"* is inverted by condition 3 (K4); *"shrinks the review surface"* rests on a false estimate (M1); *"tooling is measured to work"* was false at the landing SHA (M10). The fifth — *"dominates the strip family on the dead options' own grounds"* — is `D-338`'s finding about `D-338`'s C-prime, not this one |

---

# MAY A SELECTION BE TAKEN FROM THIS FIELD?

**NO.**

The recommendation falls. B is selectable only with its criterion replaced. C is
unchanged and wounded. NULL survives and is understated. The row the field most needed
to carry — `D-338`'s C-prime — is named in the field under its own name and is not in
it.

**What a revision 3 owes, minimally:**

1. **C-prime restored to its landed definition**, or the divergence declared and priced
   as a NEW row with its own evidence. It may not carry V4 while inverting the two
   clauses V4's precedent turns on.
2. **A defined marking predicate**, with the reading of V1's eleven instances stated
   explicitly (M8), because the row's cost and its reach both hang on it.
3. **The remainder measured, not estimated** — M1 gives four numbers and the script;
   any revision may re-run it or refute it.
4. **Rows R2, R3 and R5**, which `D-338` records as owed and this field carries none of
   (M12 below), plus the item-7 row evaluated below.
5. **Flip clause 1 replaced** by a criterion the named defect can falsify (M5).
6. **The `<SHA>` filled** (M3).
7. **Condition 4 dropped or replaced.** "Count and placement" cannot be repaired into a
   criterion; the defect is that markers are checked against nothing.

## M12 (MAJOR) — MISSING ROWS. `D-338` NAMES FOUR ROWS A REVISION 2 OWES; THIS FIELD CARRIES ONE. R3 IS NOW MISSING FROM TWO CONSECUTIVE FIELDS. (attack surface item 7)

`D-338`: *"FOUR FURTHER MISSING ROWS stand recorded against a revision 2: R2
review-contract-only …, R3 derived-enumeration on the landed `7dfd047` precedent (which
D-334 already owed and which round 2 MEASURED to be the only candidate whose prevention
claim is a mechanism rather than an assertion), R4 the self-state gate …, and R5
retire-don't-edit for `WPQ_seed.md`."*

The field is {B, C, C-prime, NULL}. **R2 absent. R3 absent. R5 absent.** R4 is disposed
of outside the matrix by V3's carve-out, which M2, M9 and M10 each wound.

**R3 is the serious omission.** It is the only candidate two independent rounds have
MEASURED to be a mechanism rather than an assertion, `D-334` owed it, `D-338` owed it
again, and this field is the second consecutive one to omit it. A field that recommends
a marking convention while omitting the derived-enumeration row has not compared its
recommendation against the only mechanism in the record.

---

# THE NAMED CANDIDATE ROW — REVIEWS SCORED BY CLAIM-INVENTORY DIFF (attack surface item 7)

**Verdict: it is a REAL MISSING ROW, distinct from B, not dominated by B, and it must
be carried by a revision 3. It does not rescue the field, and it ships with a vacuity
that must be registered before it runs.**

**Distinct from B, and on the axis that matters.** B institutionalises the inventory as
a deliverable and repairs until successive inventories "stabilize" — a whole-artefact
comparison whose relation M6 shows is undefined. The candidate makes the **diff** the
deliverable: this round's inventory is derived from the prior round's landed one, and
the review is scored on what the diff shows (rows added, rows removed, verdicts
flipped). That is an instrument change, not a cadence change.

**It repairs B's defining defect by construction, and it is the strongest thing in the
field's vicinity.**

- A diff **requires** stable row identity, so `Z1–Z54` against `Y1–Y61` becomes
  impossible: the row set is carried forward and amended rather than rebuilt. M6's
  objection dissolves.
- Every count becomes **derived from the diff** rather than stated by the author. That
  is D-336 clause (4) mechanised — *"a SUMMARY LINE IS NOT THE MEASUREMENT, the pasted
  output is"* — and it is a direct, mechanical answer to **V2**, which neither B nor
  C-prime nor NULL answers at all.
- It is cheap and it is checkable by a third party: two landed tables and `diff`.
- It gives flip clauses a real referent for the first time in this work package. "The
  inventory stabilised" becomes "the diff is empty", which is a fact rather than a
  judgment.

**Its own vacuity, which must be registered with a consequence before it runs.** Seeding
a fresh reviewer with the prior inventory destroys independence at the extraction stage.
A claim that neither the prior reviewer nor this one extracted is **invisible to the
diff** — the diff is invariant under a claim that was never in either table, and the
blindness compounds monotonically across rounds. That is CLAUDE.md's own rule against it:

> two instruments blind to the same stage are one instrument reported twice, and their
> agreement is invariant under a defect in what they are both blind to.

**The registered consequence the row must carry**, stated before either party runs: each
round, a party who has seen NEITHER inventory extracts claims from a fixed random sample
of the section under review; any claim it finds that appears in neither the prior nor the
current inventory means the extraction is incomplete and the round's diff is not a score;
two consecutive rounds in which the blind sampler finds such a claim means seeding is the
defect and the row is withdrawn rather than re-run.

**And the empirical base must be stated as thin, because it is.** Two inventories exist,
both against U4-Z, one 519-line section of one of six documents (M6). The row is a
proposal supported by two data points, and a revision 3 that presents it otherwise
repeats E2's over-specification that M1 of the prior round scored.

---

# FINDINGS ATTEMPTED AND REJECTED, WITH THE REPRODUCER

**RJ1 — "The D-337 freeze was broken."** REJECTED. `git diff 908a2f7..HEAD` over the six
subject documents returns nothing (printed at the head of this report). The four files
that moved are two test files, `tools/label_consistency_check.sh` and `docs/decisions.md`,
all outside the frozen subject per D-337 clause (2). **NOT A VOID.**

**RJ2 — "An add-only marker reddens gate 15's counted forms."** REJECTED, twice, and I
record it because K3 found the collision elsewhere and the difference matters. Markers
appended to a summand line and prefixed to a group-count line left the gate green; so did
a per-item marker placed inside a counted backtick group:

```
$ sed -i '146s/$/ [[SELF-COUNT · home: docs\/decisions.md D-338]]/' docs/experiments/section_owner_table.md
$ sed -i '111s/^\*\*U2 (20):\*\*/**U2 (20):** [[SELF-COUNT · home: docs\/decisions.md D-338]]/' docs/experiments/section_owner_table.md
$ sed -i '112s/^/[[UNVERIFIABLE · home: wp15b_sprt_prereg.md]] /' docs/experiments/section_owner_table.md
$ ./tools/label_consistency_check.sh | tail -1; echo "exit=${PIPESTATUS[0]}"
label_consistency_check: 6 documents, 2 summand line(s), 5 summand row(s), 4 group count(s) — every document agrees with itself
exit=0
```

The counted forms tolerate markers. **Only the closing region refuses them** (K3), and it
refuses them because of the fold law, not by accident.

**RJ3 — "V1 does not reproduce."** REJECTED. `sed -n '885,897p' docs/experiments/U3_tier_t.md`
returns 13 table lines — a header, a separator and the eleven rows
(`… | grep -c '^  |'` → `13`); each site named in the cell appears in it; U3's section map
puts §6.1, §6.2, §6.3, §6.5 and §10 in normative sections. The cell is exact.

**RJ4 — "V4's 'three times in-tree' is false."** REJECTED as stated. The three strikes
`D-338` names are `U4:230`, `U4:234`, `U4:245`, inside the REVIEW STATUS tables, and
`grep -c '~~'` returns 11 lines across U4 plus 1 in `WPQ_seed.md`. The count is conservative
rather than wrong. What is wrong is the option it is attributed to — K1.

**RJ5 — "V2 does not reproduce."** REJECTED. Both counts re-derived in M6's evidence
block: 54 rows under a stated 34, 11 FAILS under a stated 6.

---

*DECISION-RED-TEAM report, matrix META-2. Subject `908a2f7`; HEAD at entry and exit
`75508db`; the six frozen subjects unchanged across the window. Land verbatim.*
