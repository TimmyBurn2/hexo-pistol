# WP-P1d — provenance response

Revision: over `a3fa5c8` (the WP-P1b commit). Step-1 review pin `76a98666`.
Human corpus `b2fe61eb…` verified before and after; **never modified**.

## Step 0 — close WP-P1b: DONE

T3 converted. It no longer fails because a witness has not landed; it asserts
the STATE of the evidence — `mapping_status: UNVERIFIED`, POSITION-GRADE
declared, and no golden text fixture derived from the puzzle corpus. T5 asserts
the D-447 horizon criterion is the one the gate registered and applies. **The
suite is green end to end; nothing ships red.**

Both are real tests, not decorations: against a mutant that returns VERIFIED
with no witness, three assertions fire and the suite goes red. The gate's own
exit code now means one thing — non-zero only if the BASIS leg fails, which
would mean the payload is not axial in pistol's lattice at all. A witness
shortfall is a state of the evidence, not a broken instrument.

WP-P1b committed as `a3fa5c8`. The RULE9 marker restoration is a separate
session and separate commit per the standing note; it is NOT in this commit, and
gate 17/19 still fails at HEAD on the 30 pre-existing files.

## Step 1 — byte-level filter audit: BOTH AUDITABLE CONJUNCTS HOLD

`crates/pistol-cli/src/corpus/audit.rs` + `bin/corpus-audit.rs`. Win detection
is pistol-core's `wins_at` and the turn structure is `stones_in_turn`; this
module computes neither (rule 2).

    $ corpus-audit --corpus <corpus> --expect-sha b2fe61eb…
      games audited           8698
      move count  min 20  median 49  max 715
      percentile  move count
      0           20        60          57
      10          27        70          67
      20          31        80          83
      30          37        90          113
      40          41        100         715
      50          49
      >= 20 moves           8698 of 8698 (0 short)
      last stone completes a run  8698 of 8698 (0 not)
      continued past a win        0
      rating present both sides   8698 of 8698
    source_filter auditable conjuncts: BOTH HOLD          EXIT=0

**The minimum being exactly 20 is the informative part.** The dispatch asked for
distributions precisely because a floor of exactly the claimed threshold and a
floor of 47 mean different things: a sharp boundary at 20 is what a deliberately
applied cut looks like, where 47 would have meant the floor was incidental to
some other selection.

**The `rated` conjunct is reported as UNAUDITABLE in substance.** `record.rs`
REQUIRES `elo` (`ELO_KEY_REQUIRED`) and refuses a corpus without it, so
"8698 of 8698" is forced by the reader — a tautology for any corpus this reader
accepts, not a measurement. The tool says so rather than presenting it as a
result.

Negative controls (7 tests, all green), because a clean pass from an audit that
cannot fail is not a pass: a 19-move game is flagged short and makes
`filter_holds()` false; a game whose last stone completes nothing is flagged
indecisive; a game continuing past a win is flagged; a missing rating is not
counted as rated; the rendered distribution and deciles are bound to the games.

**D-456's residual is untouched and is now larger, not smaller** — see step 3:
this establishes the stated filter WAS applied, never that it was the ONLY
filter applied.

### Reviews: the numbers were right, the audit was not yet an audit

Both fresh-context reviews reproduced every figure with their own independent
implementations — 8698 games, min 20 / median 49 / max 715, all eleven deciles,
0 short, 0 indecisive, 0 continued-past-a-win — and confirmed the corpus is
unmodified. **No number in this report moved.** What they found is that the
result was PRINTED rather than ESTABLISHED, which is the dispatch's own bar:

- **BLOCKING.** The corpus-level counts were unbound. Deleting one line — the
  `indecisive.push` in `Audit::of` — left all seven tests green while the binary
  printed the exact reported block over a corpus with **500 deliberately
  non-decisive games**. Every per-game flag was bound; the aggregation and the
  rendered block the numbers are transcribed from were not. Third occurrence of
  D-219's defect class in this module family.
- **BLOCKING.** The move floor turned on a `>=` no test could distinguish from
  `>` (the only fixture was 19 moves, which fails both). **The corpus has
  exactly two games at exactly 20 moves**, so that one character flips the run
  from BOTH HOLD to a D-456 STOP.
- The decile mutant `seen > want` -> `seen >= want` survived and moves the real
  corpus's 100th percentile **from 715 to 657** — a live channel, not a
  hypothetical one.
- `filter_holds` called a game that ran PAST its win a false `source_filter`. It
  is decisive — the six is on the board — and its defect is rule-4 conformance.
  A false STOP is the expensive direction, so `decided_early` is now its own
  class and only a game with no run anywhere fails the conjunct.
- The occupied-cell path announced "the metadata is wrong" for a record that
  simply does not replay. Wrong-shape input now has its own name (rule 3).
- `winner` was parsed, range-checked and never read, so a record contradicting
  its own outcome passed. Now checked: **0 of 8698 disagree**, and the check is
  sharp because both winner values are well represented.
- The binary had no test at all, though its exit code IS the machine-readable
  verdict.

Everything above is fixed. The suite went **7 -> 25 tests** (19 library, 6 CLI),
and **all 16 mutants the reviews found alive now die**, verified in a throwaway
worktree. The audit was re-run after every fix and the numbers did not move.

**One finding is reported rather than fixed** (RED-TEAM F6): four sites now walk
the turn structure with the same `owed.min(remaining)` shape. They cannot
diverge arithmetically today and none re-implements geometry or win detection,
so this is not a rule-2 breach on its face — but `audit.rs` and `replay.rs`
reach OPPOSITE verdicts on one input class (a turn whose recorded first stone
completes a line and is followed by a second): one calls the metadata wrong, the
other calls the game eligible. Zero such games exist in this corpus. A shared
`turns()` helper touches `replay.rs`, a Stage-1 instrument, and is out of this
package's scope; it is recorded here and in D-458 rather than done quietly.

Two facts the reviews established that the tool now reports:

- **157 of 8698 games end on an OVERLINE (7+), not exactly six** — 149 sevens,
  7 eights, one eleven. Game rule 2 scores overlines as wins, so all 157 are
  decisive, but "six-in-a-row" is literally false for them. Counting them is
  independent evidence the SOURCE platform scores overlines the same way, which
  corroborates rule 2 against an exact-six variant.
- The `rated` count is a HALF-tautology and the tool now says which half: a
  record with no `elo` key is refused by the reader and can never be counted,
  while the `null` case is real and reachable (the corpus has 0 nulls).

## Step 2 — LIVE RADIUS PROBE: IMPOSSIBLE, not deferred

Two independent blocks, both operator-confirmed: **there is no active server**,
and no credential exists anywhere (`HEXO_BRIDGE_TOKEN` unset, no bridge config
on this machine, and the bridge resolves its token only from that variable).
Nothing was faked around either, and no credential was invented, reused from
another surface, or committed.

**The radius question is settled by OPERATOR RULING: radius 8 confirmed.** Two
things must be said plainly about what that is and is not:

- It is the *same footing D-101 already rested on* — an operator confirmation
  against the platform — and **not** the source measurement D-455 specified.
- **D-455's stated benefit is NOT obtained.** The corpus shows max observed 8
  with zero beyond, which is consistent with radius 8 and equally consistent
  with any larger radius human play never exercised. A rejection at 9 would have
  pinned the UPPER bound for the first time. No probe ran, so **the upper bound
  remains unmeasured** and D-455's open question stays open.

What this does achieve is D-455's main purpose: D-218's radius conclusion no
longer needs the orphan dataset to be representative. It rests on the operator's
platform ruling, with the corpus reduced to a consistency check.

## Step 3 — credential question: HISTORY IS UNAUTHENTICATED

One authorized request, one redirect followed to the same resource, and stop.

    $ curl -sS -D - -o - "https://hexo.did.science/games"
    HTTP/2 302
    location: /games?at=1787936967396
    $ curl -sS -L --max-redirs 2 "https://hexo.did.science/games"
    HTTP/2 302
    HTTP/2 200          57918 bytes, hydration payload present

The payload's `account` query carries `{"user": null}` — anonymous — and its
`["finished-games","all","all",1,20,…]` query carries 20 real games. **Match
history serves data without authentication.**

    "pagination": {"page":1, "pageSize":20, "totalGames":101257,
                   "totalMoves":4686965, "totalPages":5063, …}

Two facts the successor needs:

- **101257 finished games against this corpus's 8698 — roughly 8.6%.** The
  corpus is far more selective than "rated, >=20 moves, decisive" alone
  suggests. That does not contradict step 1, and it makes D-456's residual
  BIGGER: the stated filter is corroborated, and something else also selected.
- The listing carries **no `moves` field** (`id, sessionId, startedAt,
  finishedAt, players, playerTiles, gameOptions, moveCount, gameResult,
  tournament`). A move list needs a per-game fetch, so a snapshot is a two-level
  crawl of ~5063 index pages plus one request per game.

Nothing further was fetched. No second page, no enumeration, no snapshot begun.

**Two server-side corroborations fell out of the same payload, neither sought:**

- `"playerTiles": {"gmm744": {"color": "#fbbf24"}, "e43w82": {"color": "#38bdf8"}}`
  — players[0] yellow, players[1] blue, **confirming D-445 from the SERVER**
  rather than from the client bundle it was derived in.
- `"gameOptions": {… "firstPlayer": "host"}` — the opener is a game OPTION,
  confirming D-444's claim that play order is not implied by seat index.

## Step 4 — documents re-graded

- `docs/decisions.md`: D-444 … D-456 appended verbatim as dictated (the log
  ended at D-443 and none of them had been written down, so step 4's citations
  had nothing to point at). D-457 appended recording this package's results.
- `docs/research/sealbot_deep_dive.md:1167`: the corpus citation under D-218's
  refutation is marked ARTIFACT-GRADE, with the radius ruling restated as
  resting on the operator's confirmation rather than on that histogram.
- `docs/ROADMAP.md`: one section stating plainly that the corpus cannot be
  re-acquired, what that leaves standing (D-446, D-447, D-218) and what it
  blocks — **D-434's Stage-2 calibration and holdout, BLOCKED**, not "preferred"
  and not "ideally".

## What is still NOT established

The mapping stays UNVERIFIED and the puzzle corpus POSITION-GRADE: one
unambiguous witness at 11 stones, below the turn-14 horizon, so it closes
nothing. The upper bound on the legal radius has never been measured and the
operator ruling does not measure it. `source_filter`'s `rated` conjunct is
unauditable from these bytes, and the two that were audited establish only that
the stated filter was applied — the 8.6% coverage says something else selected
too, and no byte-level audit can say what. No snapshot was taken, so nothing was
re-censused on a new population. `b2fe61eb…` remains ARTIFACT-GRADE and cannot
be promoted, only superseded. Nothing here is a strength claim.
