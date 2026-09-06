# D-616 retirement sweep receipt (R9 / D-625)

Revision: `0998a4410ce5a27900b0bcc49b747fea86d3efcb` (`dev`). Command, with its scope:

```
git grep -n 'D-616' -- . | LC_ALL=C sort
```

Occurrence totals, derived rather than transcribed (`git grep -c` counts LINES,
so the occurrence count is taken with `-o`):

```
docs/ROADMAP.md                                       2 occurrence(s) on  2 line(s)
docs/decisions.md                                    14 occurrence(s) on  3 line(s)
docs/experiments/wp22_FINAL_SUMMARY.md                4 occurrence(s) on  4 line(s)
docs/experiments/wp22_SESSION_SUMMARY.md              3 occurrence(s) on  3 line(s)
docs/experiments/wp22_STOP_SUMMARY.md                 2 occurrence(s) on  2 line(s)
docs/experiments/wp22_phase1_design_REVIEW.md         6 occurrence(s) on  6 line(s)
```

**These totals are LARGER than the ones D-625 states**, and deliberately so:
D-625 counts at `ae211f0`, before the sweep ran, and the sweep's own ADR line
names the key it retires seven times. A receipt that agreed with the count it
was taken to check would be a receipt taken before the work.

## Classification, and why the sweep is not uniform

| file | kind | action | why |
|---|---|---|---|
| `docs/decisions.md` | append-only ADR log | **unchanged** | D-616 stays as written and D-621 corrects it downstream. Editing it would falsify the log's own rule. |
| `docs/experiments/wp22_phase1_design_REVIEW.md` | review REPORT | **unchanged** | a reviewer's words are the record that produced the finding; editing them to agree with the finding destroys the evidence. |
| `docs/experiments/wp22_SESSION_SUMMARY.md` | record | **unchanged** | its citations were true when written, which is why the citation gate names its list rather than globbing it. |
| `docs/experiments/wp22_STOP_SUMMARY.md` | record | **unchanged** | same. |
| `docs/ROADMAP.md` | **governing** | **REWRITTEN** | its Stage-2 paragraph corrected the mechanism to non-identifiability in one sentence and still named the censored likelihood as the successor two sentences later. One paragraph, two answers. |
| `docs/experiments/wp22_FINAL_SUMMARY.md` | record carrying **forward instructions** | **two items rewritten** | "Phase 2's premise memo should/must quote D-616" is an instruction, not a record, and it pointed a successor at a mechanism measured backwards. |
| `docs/experiments/wp22_phase1_fit_finding.md` | superseded finding | **unchanged** | it already declares itself superseded in its first ten lines, which is where a reader meets it. |

## What the sweep does NOT prove

It proves no document CARRIES D-616 as a live instruction. It does not prove
no document restates D-616's mechanism WITHOUT naming the key — a claim in
bare prose is invisible to this instrument, which matches `tools/governing_citation_check.sh`'s
own disclaimer about itself.
