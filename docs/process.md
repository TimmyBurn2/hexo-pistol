# Process — pre-registration methodology detail

Pointed to from CLAUDE.md's Process section. This file holds the parts of
the pre-registration methodology whose wording is detailed enough that
inlining it would sink CLAUDE.md's line budget (D-424's "state once, and
point" applied to this document itself). Nothing here is optional because
it lives in a second file — a pre-registration is bound by every rule
below exactly as it would be bound by CLAUDE.md text, and an amendment to
this file reopens review the same as an amendment to CLAUDE.md would.

## Instrument governing revision

THE INSTRUMENT HAS A GOVERNING REVISION TOO. An artefact that produces a
registered number — a `tools/` script, a scratchpad harness, or a
command block the document prints — is named in the pre-registration
WITH ITS REVISION, and a change to it reopens the review exactly as an
amendment to the document does. `tools/` is where such artefacts usually
live; living there is not what makes the rule apply. Without this, a run
stands on an instrument whose own review had failed and is licensed by
argument rather than by this text.

## tools/ review coverage rule

A change under tools/ is reviewed against tools/SHELL_CHECKLIST.md — the
review prompt cites it and the reviewer answers its items by name —
because three consecutive rounds found ONE class in those scripts: shell
under `set -euo pipefail` parsing unvalidated output and failing as
EXIT-0-WRONG-ANSWER. Its coverage rule is the binding one: any tools/
script that produces a recorded number carries at least one test driving
the shipped script. The checklist is judged, not mechanized.

## Dry-run discipline

A pre-registration's literal commands are exercised before its review
passes, on an input of the SAME KIND as the registered workload — the
same sort of artefact, differing only in identity — and never on the
registered workload itself. A synthetic stand-in exercises syntax; only a
real instance of the kind exercises ATTRIBUTION, which is where a command
that counted the wrong symbols passed a synthetic dry run and still
shipped. The dry run is not a governed sample and does not consume the
pre-registration's first run. The pre-registration records the dry-run
input and its output. This constrains the dry run's input; it constrains
no reviewer, who may run anything, the registered workload included.

## Criterion and defect class

AND IT RECORDS WHAT THAT OUTPUT MUST SHOW, together with the DEFECT CLASS
the criterion is meant to exclude. Recording without a criterion is a dry
run nothing can fail. A criterion that is a property the named defect
class PRESERVES — internal agreement between components sharing an
input, output shape, plausible magnitude, exit status — passes vacuously
and is not a criterion; it must be one that defect could falsify. An
externally derived referent, a value computed by something that does not
share the suspect input, is the operationalisation that reliably achieves
this and is what a reviewer looks for first: sufficient, not necessary.
This binds ANY registered criterion, dry-run or governed alike.

## Cost, replication, and the second instrument

A pre-registration states what its governed run COSTS — wall time,
operator attention, machine hours — so the proportion between the
document and the run is visible on the document's own face. Where the
run is cheap, doubt about the instrument is answered by REPLICATION and
by a SECOND INSTRUMENT whose agreement criterion is registered before
either runs, never by a margin derived to defend a single sample. A
registered agreement criterion carries a REGISTERED CONSEQUENCE: the
pre-registration states, before either instrument runs, what DISAGREEMENT
does to the verdict, or the criterion leaves standing the after-the-
numbers decision it exists to forbid. AND IT NAMES THE STAGE UNDER DOUBT,
and says how the second instrument does not share it: two instruments
blind to the same stage are one instrument reported twice, and their
agreement is invariant under a defect in what they are both blind to —
measured, a dependency-graph check and a two-build digest comparison both
missed what a build script READ, agreed, and confirmed a tree where the
subject reached the binary. A derived margin is the instrument of a
measurement that cannot be taken again, and it is the wrong instrument
for a workload measured in seconds. Neither this rule nor the dry-run
rule is mechanized, and neither catches a run whose answer is already
known before it is taken — that defect is judged, not checked.

## Re-derivation, and it is addressed to the reviewer

**A reviewer of a governing document RE-DERIVES its load-bearing counts and
citations with commands THE REVIEWER CHOOSES, never the document's**, prints each
command with its scope beside the number it returns, and reports the comparison.
**A count the reviewer reproduces only by running the document's own command is
NOT reproduced.**

**WHY THE LAST SENTENCE IS THE WHOLE CLAUSE.** The recurring defect this project
pays for is not an unchecked claim; it is a claim CHECKED AGAINST THE WRONG
POPULATION — a `git grep` scoped to `src` that missed its answer in `tests`, a
line count scoped to `src` that counted `src/bin`. In every one of those the
command was run and its output transcribed faithfully. **Re-running the
document's own command re-derives nothing**, and any mechanism that executes it —
including a CI gate — is green forever on exactly this class (D-582).
Independence of SCOPE is the property that catches it, and only a second party
choosing differently has that property.

**WHY IT IS ADDRESSED TO THE REVIEWER AND NOT THE AUTHOR.** D-568 and D-574 wrote
the same instruction for authors twice; the arc that wrote them broke them four
more times, and the next arc broke them at least nine, three of those inside the
document restating the law. The same arc briefed three fresh contexts to
re-derive and they returned 57 findings between them. **The difference is not
diligence** — the author and the reviewer are the same kind of agent, minutes
apart, holding the same rule. The author is reaching for a label for something
already believed; the reviewer is paid to disbelieve it.

**WHAT A DISPATCHER OWES**: nothing. This clause is standing, so a review no
longer depends on whoever wrote the prompt remembering to ask for it.

**WHAT IS MECHANIZED, AND IT IS ONLY THE CHEAP HALF.** CI's
governing-document citation gate (`tools/governing_citation_check.sh`) refuses a
citation, in a document on its own named list, that the tree does not hold. It
catches ROT — a path or line that moved — and it catches nothing about scope. The
list is named rather than globbed because most documents in this repository are
RECORDS, whose citations were true when written, and demanding a record match
today's tree is demanding that history be falsified.
