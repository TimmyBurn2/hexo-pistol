# WP-2.0-M — DESIGN STOP. Five reviews, and the failure is the authoring method.

> **Audience: the operator, first read.** §0 is the diagnosis, and it is about
> how I write these documents rather than about the package. §1 is the evidence.
> §2 is the remedy, which is built and in the tree. §3 is the decision owed.

## 0. THE DIAGNOSIS

**WP-2.0-M's design failed twice** (`5064b05`, then `7af62e7`), so the standing
caps fire and it STOPs. It is the second STOP in this package: WP-2.0's design
also failed twice and split into WP-2.0-M and WP-2.0-S (D-544).

**FIVE DESIGN REVIEWS IN THIS ARC, FIVE FAILURES, AND THEY ARE NOT
DISAGREEMENTS ABOUT JUDGEMENT.** Every one turned on a claim about the code that
the code does not make, or on a true claim that a later revision destroyed:

| revision | what failed |
|---|---|
| WP-2.0 rev 1 | a forced decision dropped without appearing in the deferral list |
| WP-2.0 rev 2 | the fix round **introduced two new defects** — the provenance remedy reproduced the defect it was fixing, and a reversal was applied to two sites of four, leaving a test that pinned the negation of the invariant above it |
| WP-2.0-M rev 1 | the central claim was false — a "verbatim" capture cannot be reproducible, because the totals line carries wall-clock fields that CI gate 9 exists to strip |
| WP-2.0-M rev 2 | **the rewrite deleted the true sentence and built a mechanism on its negation**, and lost seven things including a whole section the previous review had PASSED |

**THE LAST ONE IS THE CLEAREST AND THE WORST.** Revision 1 of WP-2.0-M said, at
its line 28, *"`validate_engines` refuses identical **labels**, not identical
binaries or configs."* That is true. Revision 2 deleted that clause and asserted
that a self-play report has identical engine sections — and built a refusal on
it. Both `crates/pistol-arena/src/validate.rs` and
`crates/pistol-arena/src/transcript.rs` **refuse identical labels**, and `label`
is a field of `EngineSection`, so **the mechanism would have refused one hundred
per cent of its inputs.** The correct comparison is one type to the left:
`EngineIdentity` carries `id_lines`, `binary_sha256`, `config_sha256` and
`weights_sha256` — and no label — so two seats on one binary and config do
compare equal.

**THIS IS NOT A SCOPE FINDING AND I WILL NOT PRETEND IT IS.** The reviewer
offered that reading — a second split would mean the mechanism cannot be designed
as scoped. It can. The two hardest mechanisms in revision 2 were **verified
correct against the tree**: the wall-clock normalisation holds because the solver
fields interpolate between `nodes` and `nps`, leaving `nps` and `time` adjacent
for gate 9's own expression; and the asked set is right at all three boundaries,
with INVARIANT 2 implementable because only a game's final prefix can ever be
decided. **The content was reachable. The authoring lost it.**

## 1. THE PATTERN, AND THIS ARC ALREADY NAMED IT ONCE

**D-543** recorded that a matrix's `MEASURED` block was wrong in three
consecutive revisions — a count taken from a `tail`-truncated listing, then
**re-asserted under the label `REPRODUCED` without being re-run**. It named the
remedy: render the tables from the artifact **by machine**, so a retyped number
cannot drift from its run. That remedy worked; the tables have been right since.

**It was never generalised to prose, and prose is where the next four failures
happened.** The same defect in a different medium: a claim about the tree,
asserted from memory or from a previous draft, carried across a revision without
being re-checked against the thing it describes.

## 2. THE REMEDY, BUILT

`tools/design_citation_check.py`. It extracts every `path` and `path:line` a
document attributes to the tree and checks that the tree holds it — that the file
exists, and that a cited line is inside it. A design must declare the files it
proposes to create with `--proposes`, **which is a discipline for the author
rather than a courtesy to the checker: a design that must list the files it
invents cannot invent one by accident in a rewrite.**

**Run over this arc's documents it is green**, and its own output says what that
is worth:

> A GREEN RUN MEANS THE CITATIONS ARE REAL, NOT THAT THE DOCUMENT IS RIGHT: this
> instrument cannot tell whether a true quotation supports the claim built on it,
> and a claim stated as bare prose is not checked at all.

**IT WOULD NOT HAVE CAUGHT THE ENGINE-LABEL DEFECT**, because revision 2 stated
that claim as bare prose with no citation, and that is the honest measure of it.
What it forecloses is the cheaper half — a rotted path, a line past the end of a
file, an invented module — and what it teaches is the habit the expensive half
needs: **a claim about the code carries the citation that makes it checkable.**
The premise memo's author did this to themselves by hand and found seven wrong
citations in three hundred and ninety-six; this is that pass, mechanised.

## 3. THE DECISION OWED

**One question, and it is not about WP-2.0.**

Five reviews have been spent on two design documents and neither landed. The
package is not blocked on knowledge — every mechanism it needs is verified
somewhere in this arc's reviews, most of them twice. **It is blocked on my
producing a document whose every claim survives a reader who checks.**

- **A — re-attempt WP-2.0-M under the new discipline**: every code claim carries
  a citation, `design_citation_check.py` green before any review is dispatched,
  and the two verified mechanisms (§4's normalisation, §2's asked set) lifted
  from revision 2 **by quotation rather than by rewriting**, since rewriting is
  what lost them.
- **B — hand WP-2.0-M's design to a fresh session** with this arc's five reviews
  as its input. The reviews between them contain nearly every answer; what they
  do not contain is a document.
- **C — something else you see that I do not.**

**I recommend B**, and I recommend it against my own interest in finishing what I
started. The evidence that I lose true sentences across revisions of these
documents is four reviews deep, and the last failure happened in the very act of
fixing the one before it.

## 4. STATE AT THE STOP

- **`dev` is clean.** Both WP-2.0-M design revisions and both their reviews are
  committed as the record of a failed round. No implementation was started, no
  engine file was touched, no committed config moved.
- **`tools/design_citation_check.py` is new and on no gate path.** It is an
  instrument, not a gate; making it one is `tools/` hardening's business.
- No worktree, no detached process.
- What WP-2.0-S is owed is unchanged and is listed in
  `docs/experiments/wp20_DESIGN_STOP_SPLIT.md`.

## 5. Gates

`tools/ci.sh` at `7af62e7` — the revision the second review adjudicated — run in
a detached worktree with its own `target/`, never with `CARGO_TARGET_DIR`
exported. Log `artifacts/wp20m_ci_stop_7af62e7_v1.txt`, read from the gate log's
own lines: all nineteen `=== gate N/19:` lines, final line **`ci: all gates
passed`**, `/usr/bin/grep -cE "^ci: FAIL|^ci: RUN VOID|test result: FAILED"`
returns **0**, and gate 9 closes `determinism: ok — 5 seat(s), no difference
outside nps/time in any of them`.

**The commit that lands this document is documentation plus one `tools/` script
on no gate path**, so it is not re-adjudicated — the cut this project makes and
states rather than implies.
