# WP-2.0 pilot — STOP. The pre-registration spent its grant without passing.

> **ONE LINE FOR THE MORNING.** The pilot did not run: its pre-registration failed
> four fresh-context reviews and D-552's grant is spent, so the package is back
> with you — but every number in it is verified correct, nothing is broken, `dev`
> is clean and green, and what is left to fix is about eight sentences.

## 1. What happened, in plain language

The pipeline itself is fine. Both halves are landed, tested and green; the dry run
executed the pre-registration's literal commands end to end four times and every
criterion answered as registered. **What failed is the document that governs the
run**, four times, and by the fourth round the reason had stopped being about the
pilot at all: **fixing one sentence kept breaking another.**

Round one was a real defect of understanding — the document read `arena --config`'s
exit `1` as "forfeits" when the program returns `1` from two disjoint branches, so
an ABORTED pilot would have been recorded as a pass with a finding. That was fixed
and never came back. Rounds two, three and four failed almost entirely on
**remedies that broke something adjacent**: twelve of thirty in one round, six of
eleven in the last. The final round applied all eleven findings it was given and
introduced six new defects doing so.

## 2. The decision owed — and it is small

**Either** grant one further revision round (the remaining work is listed in §4 and
is entirely sentence-level), **or** take the document as it stands on condition
that the two findings in §3 are fixed first.

**And a second, smaller ruling if you want it.** D-552 says *"up to FOUR revision
rounds"* and also *"a fifth failure returns the package to the architect"*, and
those two can be counted to give three rounds spent or four. **I took the stricter
reading**, because revision 5's own header — committed before its review ran —
says *"THIS IS THE FOURTH AND LAST ROUND THE GRANT ALLOWS"*, and re-counting after
seeing the verdict is the move this arc's whole discipline forbids. One sentence
reverses it if you meant the other.

## 3. The two findings that could make a recorded conclusion WRONG

Everything else on the list makes the document worse without endangering a verdict.
These two do not, and a run governed by the document as it stands could report
something false because of them.

**(a) §5 misclassifies a reachable launch failure as a STOP.** The section says an
exit `2` from *"any instrument of this pilot"* is one of three things, then
enumerates only the capture, labels and checker refusals. **Pass 1's own pre-game
refusals are in no limb** — a bad arena config, an unreadable openings file, and
above all a stale `binary_sha256`, which is the one slot §9.1 says can only be
true at the run's own launch. By the document's complement rule those become
V7-B, a STOP of the whole arc, when they are plainly "no answer was taken". The
error direction is conservative — it stops when it should void — but the verdict
is wrong and the trigger is likely.

**(b) §7.2's coldness bound is off by two and cites a disqualified artifact.** It
says two quantisation intervals *"both contain everything from 0 to about 4 s"*.
That is false of `0 ± 2`; the intersection of `[0,4]` and `[-2,2]` is `[0,2]`, so
the bound is **12 ms per ask, not 24**. Worse, its two inputs (85 s and 83 s) exist
only in the dry-run artifact that §7.1 itself declares superseded. This is a
number the closure would have carried into the corpus plan.

## 4. What else is owed, none of it verdict-endangering

Four MAJORs and seven MINORs, every one a sentence or a table cell:

- §6.3 still says the coldness cost is "below one-second resolution" — the claim
  §7.2 was rewritten to retract — and its "SLOT A's predecessor" now points at the
  wrong artifact.
- §6.3 and §7.1 cite `captured 164 position(s)` as the source for `p = 41`, and
  that line is **not in the current artifact**: the dry-run script suppressed the
  arena's stdout, which is an undeclared fourth difference between the registered
  block and the one that ran.
- §9's watchdog check is unsatisfiable as written: `hang_timeout_ms = 120000` is
  120 s and does not exceed the 165 s bound the same row names, yet §9.1 records
  "yes, no correction needed". At pilot scale the pass is ~1 073 s.
- §7.1 cites `5fe1f1a3…` for the C-B digest; the current artifact and the manifest
  both say `807d5656…`. True at revision 4, spent by the re-take.
- Seven MINORs listed in `docs/experiments/wp20_pilot_prereg_REVIEW_rev5.md`.

## 5. What is NOT in doubt

The final reviewer reproduced, with its own code from the raw artifacts, **every
number the document registers**: all four RULE-2 medians and means, `T = 13`, every
row of the sensitivity table, the `253.5 T` wall arithmetic, the 32.5 %/65 % pair,
and **all seventeen digests** of `docs/experiments/wp20_pilot_artifacts.md`. Its
own words on fitness: *"the gap is narrow and not in the instrument. Every remedy
is a sentence or a cell; none needs a re-run or re-measurement, and none touches
SLOT S1/S2/S3/W/R1, all verified correct."*

**The registered values, all derived and none chosen**: 13 openings (26 games,
1 066 asked positions); label budget `nodes 400000`, selected by a rule that named
the median before any number existed and picked the most expensive of its three
candidates; cold-label stride 1, every record.

## 6. Three measured findings the dry run produced that nobody asked for

These bind the corpus plan whenever it is written, and they are the pilot's real
yield so far.

1. **A corpus built this way has no outcome signal.** Every dry-run game reached
   the turn cap undecided, so `result` is `capped` on every record and `end` is
   `normal` on every record — two of the loader's four token-set columns exercised
   at one value each.
2. **Every position appears twice.** A self-match of one deterministic engine plays
   each opening identically in both seats. This cannot be configured away:
   `arena --capture` refuses a report whose two seats attest different engines, so
   a self-match is the only shape a capture can be taken from at all.
3. **The coldness overhead is ~12 ms per ask** — the cost
   `docs/experiments/wp20m_design.md` §12 declined to guess, measured at last, and
   negligible beside the search.

## 7. State

`dev` is clean, everything committed, CI cited at the stop head. No worktrees, no
detached processes. Artifacts exported with digests in
`docs/experiments/wp20_pilot_artifacts.md`. The book_v2 ledger row for `0..12`
**stays**: a range a committed pre-registration reserved is spent whether or not
its run happened, which is that file's own rule.

**The pilot has not run. No corpus exists. WP-2.0 is not closed.**
