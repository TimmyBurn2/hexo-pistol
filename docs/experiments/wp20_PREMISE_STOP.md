# WP-2.0 — PREMISE STOP. One decision is owed, and it is a reading of the dispatch's own scope line.

> **Audience: the operator, first read.** §0 is the decision. §1 is why it is
> yours and not mine. Everything else is the evidence. The premise memo is
> `docs/experiments/wp20_premise_memo.md`; this document does not restate it
> (D-423).

## 0. THE DECISION, and it is one sentence

**May WP-2.0 add a gated census-logging path to `crates/` — or must requirement
3 be dropped from this package?**

The dispatch's scope line says both of these:

> Out of scope: any training, any codebook, any eval change, **any engine diff
> at all**, any strength claim, **detector work beyond the logging flag**.

*"Any engine diff at all"* forbids the change. *"Detector work beyond the logging
flag"* is a carve-out that only means something if **the logging flag itself is
in scope**. The two clauses cannot both be applied to the same change, and
requirement 3 — *"the census logging on every game per D-53a"* — is exactly that
change.

**Answer A — the flag is in scope.** I add the census behind a config token no
committed config sets, put its rows on the line protocol, and add position
identity to the census row. Requirement 3 is met. This is a diff to
`pistol-search`, `pistol-cli` and `pistol-engine`.

**Answer B — no diff.** Requirement 3 is dropped from WP-2.0 and becomes its own
package; the pipeline ships with labels and ledgers only, and D-537's corpus
starts accumulating one package later.

I have not chosen. Nothing of the design or the implementation has been written.

## 1. Why this is not mine to take

The dispatch's own STOP protocol reads **"STOP on: premise failure"**, with no
default-taking clause — unlike the previous arc's dispatch, which said *"every
fork below has a default: take it, record 'architect default applied', continue"*.
The premise verification returned **FAILS** on P2. A registered trigger that
fires is honoured, not reasoned around; that discipline is most of what this
project is.

And the reading is not neutral between the two answers. **Answer A is the one
that lets me do more work**, which is precisely the reason a session should not
pick it on its own authority.

## 2. WHAT FAILED, verified twice

The premise memo returned P2 **FAILS**. I re-verified both halves myself rather
than taking a subagent's word for a STOP:

**(a) The census is not reachable from the line protocol, because the CLI cannot
see the crate that owns it.** `crates/pistol-cli/Cargo.toml`'s `[dependencies]`
are `pistol-core`, `pistol-engine`, `serde`, `toml`, `serde_path_to_error` —
**`pistol-search` is absent**, and the manifest's own comment says why: *"The
`Engine` seam, its config, its budgets and its errors. Everything this crate says
to an engine, it says through that trait (CLAUDE.md rule 11)."* The census is a
`Searcher` method (`crates/pistol-search/src/search.rs`,
`collect_trigger_census`), whose doc says *"no committed config can ask"*. The
absence is pinned by `crates/pistol-cli/tests/workspace_shape_tests.rs`, so it
is a designed boundary and not an oversight.

**(b) A census row carries no position identity.**
`crates/pistol-search/src/census.rs`'s `TriggerObservation` holds `columns`,
`attacker`, `defender`; `TriggerColumns` holds nine numbers and a cover class.
`/usr/bin/grep -cE "key|zobrist|Key"` over that file returns **0**.

**Why (b) matters as much as (a).** D-537 fixes the round-3 re-open condition in
*"win-proving firings on DISJOINT POSITIONS"*. **That quantity is not countable
from census output today** — the arc that just closed had to recover position
identity by joining rows back to their fixture entry, which works for a 24-entry
fixture and does not work for a self-play corpus. So requirement 3 is not
"switch on the existing census": the census must gain a column before it can
answer the question it is being run to answer.

## 3. WHAT HOLDS, so the pause is costed honestly

**P1 — the protocol seam — HOLDS outright, and it is the premise most likely to
have failed.** `crates/pistol-cli/src/report.rs` renders one explicit field list
carrying `depth_turns seldepth nodes nps time hashfull score pv`, with
`bestmove` on its own line. **Score, best move, depth and nodes are all four on
the wire today**, `nodes` is in the budget grammar, and `newgame` exists and
clears the table, heuristics and solver. **The labelling half of WP-2.0 needs no
engine change at all.**

**P5 — the arena — HOLDS for the seam and FAILS for the record, and the failure
is one function deep.** `pistol-arena` already drives engines as child processes
over the line protocol, pinned by a cross-crate manifest test and CI gate 15/19
— WP-2.0's hardest-sounding constraint is met by shipped code. But
`crates/pistol-arena/src/exchange.rs` reads `nodes`, `time` and `depth_turns`
off the `info totals` line and **discards the `score` and `pv` sitting on that
same line**. The label this package exists to collect is already on a wire the
arena is already reading and already throwing away.

**So Answer B is not a stalled package.** Under it, requirements 1, 2, 4 and 5
proceed on shipped seams, and only requirement 3 moves out.

## 4. Two gaps the design must close under EITHER answer

- **`pistol-arena` has no seed.** `/usr/bin/grep` for seed/rng/rand across
  `crates/pistol-arena/src/` returns nothing, and requirement 4 asks for
  determinism given `(seed, book range, config, SHA)`.
- **No committed config points at `book_v2`**, which D-518 committed and whose
  ledger `docs/book_v2_ledger.md` is still empty.

## 5. The premise memo's own adversarial note, which I am not discounting

Its author names **P1** as the premise most likely to be wrong — not because a
quotation is off, but because the seam was verified **one `go` at a time and
never over a game**. The composite WP-2.0 needs — play at the game budget,
re-ask the same position at the label budget, advance — **has no precedent
anywhere in the tree**. The open risk is whether a label `go` on a table the
game `go` just warmed is the label that budget would produce cold. **This arc
has already paid for that exact confusion once**: D-527 records a warm
transposition table deflating a census and costing a matrix revision. Settling
it needs a run, and the run belongs in the design's dry-run, whichever answer
you give.

**The memo also reports a defect in itself**: a mechanical audit of its own 396
citations found 7 wrong, all corrected, re-run clean. It recorded that rather
than fixing it quietly, which is the behaviour the process asks for.

## 6. State at the STOP

- **`dev` is clean** and carries §0's deliverables: D-535–D-538 (the operator's
  three rulings, registered) and `docs/trigger_coverage_ledger.md` with TC-1 and
  TC-2 as its first rows.
- **No design was written, no implementation started, no engine file touched.**
- No worktree but the CI one, no detached process but its run.
- §0.2's green citation and this document's own gate line are in §7.

## 7. Gates

`tools/ci.sh` at `2434e32` — the commit carrying the rulings — run in a detached
worktree with its own `target/`, never with `CARGO_TARGET_DIR` exported. Log
`artifacts/wp20_ci_base_2434e32_v1.txt`, read from the gate log's own lines: all
nineteen `=== gate N/19:` lines, final line **`ci: all gates passed`**,
`/usr/bin/grep -cE "^ci: FAIL|^ci: RUN VOID|test result: FAILED"` returns **0**,
and gate 9 closes `determinism: ok — 5 seat(s), no difference outside nps/time in
any of them`. **That is §0.2's green confirmation** — `dev` is green at the
detector-arc closure HEAD as this package inherits it.

**The commit that lands this document is documentation only** — this file, the
premise memo, the trigger-coverage ledger and one artifact, none on a gate path.
It is not re-adjudicated, which is the same cut this arc's predecessor made and
stated rather than implied.
