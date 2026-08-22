# Matrix META-2: successor to R15, corrected field
Status: AUTHORED BY ARCHITECT, NOT SELECTED. Awaits fresh-context
DECISION-RED-TEAM. Land at SHA after the slack session, so every
citation below resolves in-tree first.

Author pre-flight, declared: every source cited at a landed path
(SHAs filled by operator at commit), every flip clause future-only and
checked unfired at authoring, every numeric cell marked. Cells citing
the red-team report carry its marks; cells of mine are ESTIMATED.

## Evidence (docs/experiments/matrix_META1_REDTEAM.md at <SHA>, unless noted)
- V1 MEASURED (report): eleven restatement instances in normative
  sections (U3 §6.1/§6.2/§6.3/§6.5/§10/§12-4, U2 §5.3, U4 §8.4), live
  at HEAD. Strips cannot reach them; A and D fell on this.
- V2 MEASURED (report): both landed claim inventories carry false
  self-counts (54 under a stated 34; 11 failing rows under a stated
  6). The detector drifts by generator 2.
- V3 MEASURED (report): two head/foot label instances found by a
  one-line shell loop. The defect class splits: a MECHANICAL subclass
  (label agreement, self-counts) and a SEMANTIC remainder. The
  mechanical gate (tools/label_consistency_check.sh, slack session)
  lands unconditionally; this matrix decides only the semantic
  remainder.
- V4 MEASURED (report): C-prime-shaped edits already executed three
  times in-tree; prescribed independently by the u-rev 8 reviewer.
- V5 (report verdicts): B and C survive wounded, NULL resurrected
  wounded, C underrated per the attacker.
- V6 direction-only, marked as such per the stop ADR: finding counts
  turned upward at the last round, zero PASS at close. Middle terms
  unreconciled; no death sentence rests on V6 alone.

## The decision
How reviews and repairs treat self-referential and drift-prone claims
in the six design documents, given strips are dead and repair
manufactures instances.

## Options

| Option | What | Cost | Failure mode |
|---|---|---|---|
| B | Reviewer claim-inventory institutionalized as the review deliverable, landed in-tree per round; repair until inventories stabilize across two consecutive rounds. | ESTIMATED 2-4 full rounds at current burn. | V2: the instrument itself drifts, and its self-counts are only partly covered by the mechanical gate; repairs remain the generator per this whole record. |
| C | Waive meta-class findings silently; reviews score normative claims only. | Cheapest per round. | Waived-false indistinguishable from true in the document of record; the U1 falsehood (carried five documents, nine rounds) is the measured instance of the rot. |
| C-prime | WAIVE-AND-MARK: one add-only marking pass over all six documents. Every self-referential, unverifiable, or known-false claim gets an inline marker naming the home of the truth (ADR, report, git). Marked text is never repaired by anyone; reviews score UNMARKED text only; markers are grep-able, so the marker set itself joins the mechanical gate's coverage. | One marking pass + re-reviews against the unmarked remainder. ESTIMATED remainder well under half of current review surface; attacker asked to measure. | Marking pass misses instances (it is authored by a session and sessions drift); mitigated by add-only discipline (zero text changed, only markers added) and by the flip clause below, which routes misses to a second marking pass, never to repairs. |
| NULL | D-331 stands as amended; keep repairing under existing law. | Per round as today. | V6 direction plus generators 2 and 3 unreached by any clause; resurrected on procedure, wounded on the record. |

## Recommendation
C-prime. Grounds: V4 (already works in-tree, independently
prescribed), dominates the strip family on the dead options' own
grounds per the report, fixes C's rot by making waived-false visible
and machine-inventoriable, shrinks the semantic review surface to
unmarked text, and converts marker bookkeeping into the mechanical
class where tooling is measured to work (V3). B keeps paying the
repair generator; NULL is the measured status quo that produced V6.

## Flip clauses (future-only, unfired at authoring)
- If the first post-marking review finds three or more unmarked
  instances in text that existed at marking time, the pass was
  incomplete: a SECOND marking pass by a different session, never
  repairs. A second incomplete pass stops the option, architect.
- If any session repairs marked text, the discipline is broken:
  escalate to operator as a process decision, not a new law.

## Registered conditions on C-prime, if selected
1. Marking pass is add-only: zero existing characters change, markers
   only. One document per commit. Frozen-matrix rule and
   landed-evidence rule bind.
2. Marker syntax defined once, in the owner table header (its one
   home): marker names the claim class and points at the truth's home.
3. Reviews after the pass: score unmarked text only; a marker is not
   a finding; marker-coverage disputes go to the mechanical gate's
   inventory, not to prose findings.
4. The mechanical gate extends to marker inventory (count and
   placement) in a follow-up commit with its own driving test.
