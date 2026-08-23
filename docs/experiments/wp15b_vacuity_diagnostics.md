# WP-1.5b governed run — Criterion 1 vacuity diagnostics

**Read-only.** Nothing here repairs, re-runs, or amends the instrument or the
prereg. Five numbered results, no recommendation — the instrument-amendment
decision, the prereg re-review, and the §5 confirmatory run are the
architect's, made on this session's output (per the GROUNDWORK dispatch
"WP-1.5b run diagnostics: preserve, record, diagnose, stop").

**Input.** `artifacts/wp15b_governed_run.txt` (this WP's own artifact
convention — gitignored, gated by `tools/artifact_check.sh`, never tracked).
Whole-file sha256 `8c04e917a175d6b2cf58513c88fe025b618942b536d7ff76c90fa4e62cd4a064`,
byte-identical to the session scratchpad original it was copied from
(`cmp` confirmed). Its internal `experiment_sha256` field reads
`228fa48f13aaf6a71bac4f0635b29a7788c48f66ba6274b71f7d5f9fad6fedc6`, matching
D-381's citation exactly — that field, not the file's own hash, is what
D-381 and the GROUNDWORK dispatch's "recorded 228fa48f…" referred to; the
distinction is stated here because the prior session's summary conflated the
two, which is corrected in D-382's neighborhood by this document existing.

All work below re-derives facts already present in
`artifacts/wp15b_governed_run.txt` and by direct engine invocation at HEAD
`3bbcbe5` (unchanged through this session — confirmed via `git status
--short`/`git rev-parse HEAD` before and after every diagnostic command).
No commit in this document's own preparation touched engine source.

---

## Result 1 — the 10 vacuous games, individually, and whether the replayed
turns are forced-class

The 10 games collapse to **5 openings** (8, 22, 34, 44, 57), each contributing
its paired colour-reversed games (16/17, 44/45, 68/69, 88/89, 114/115). Game
lengths: 15/16, 15/16, 33/20, 17/14, 22/16 turns respectively.

Replaying both checked positions (turn 4 — the book alone; turn 5 — book +
the actual recorded turn 4) against **both** `configs/instrument_staged_v0.toml`
and `configs/instrument_v0.toml` at the registered budget (`go nodes 50000`):

| opening | turn 4: bestmove (both agree) | turn 4 score (staged / r2) | turn 5: bestmove (both agree) | turn 5 score (staged / r2) |
|---|---|---|---|---|
| 8  | `-2,0/0,2`  | −238 / −238  | `-2,4/-2,6` | −266 / −266 |
| 22 | `1,-4/1,-1` | −246 / −246  | `-1,-1/1,-3` | **+962 / −192** |
| 34 | `3,0/3,1`   | −148 / −148  | `3,-3/3,3`  | **+382 / −268** |
| 44 | `1,3/2,3`   | −48 / −48    | `-3,3/3,3`  | **+66 / −1052** |
| 57 | `-5,1/-4,0` | −60 / −60    | `-6,2/0,-4` | **+124 / −1010** |

**This does not fit a "forced-class move" (must-block / only-threat-answer)
hypothesis as originally framed.** No candidate-count instrument exists to
count legal moves directly (building one would be new instrument-creation
work, out of this diagnostic's read-only scope), so forced-ness is read off
score magnitude instead: at turn 4, both engines report near-symmetric,
small-magnitude scores (−48 to −238 cp) — consistent with an early (5-stone),
still-open position, not a forced tactic. What the data actually shows is a
**different, milder mechanism**: at turn 4, staged and r2 converge on an
identical top move with near-identical *evaluations* too (not just the same
move — literally the same score, to the reported cp), which is the strongest
form of agreement and suggests the two candidate policies' shortlists
genuinely overlap this early (few stones on the board, so the staged "quiet
cut" excludes little that radius-2 wouldn't already reach). At turn 5, 4 of
5 openings show the SAME chosen move but wildly different scores (up to
opposite sign, up to ~1000 cp apart) — the two policies agree on *what to
play* while disagreeing sharply on *how good the position is*, plausible if
one dominant "best development" move exists that both find independently
even though staged is reading further ahead. Opening 8 is the outlier: its
turn 5 scores agree exactly too (−266 both), the only one of the 5 where
both checked positions show full staged/r2 convergence on both move and
evaluation.

## Result 2 — base rate, exact (not sampled)

The dispatch asked for a ≥30-game sample; a stronger measurement was
available directly from the existing report data with **zero new engine
calls and zero sampling error**, covering the full population.

**Naive (flawed) framing first, since it was the initial approach and its
flaw is instructive.** Treating the 232 replayed turns as 232 independent
trials: 162 discriminating, 70 agreeing; restricting to the 106 attributed
games' 212 turns (subtracting the 10 vacuous games' 20 trivially-agreeing
turns) gives a naive per-turn agreement rate **p = 50/212 ≈ 0.2358**.
Modeling each of the 116 games as an independent Bernoulli(p²) trial for
full vacuity predicts **6.45 expected vacuous games** (sd 2.47); observed
10 is z ≈ 1.44, P(X≥10) ≈ 11.2% — not anomalous under this model, but the
model is wrong.

**It is wrong because pairing makes turn 4 checks structurally
non-independent.** Both games of a pair share the identical opening book, so
turn 4's query (book-only prefix) is the *literal same question* asked
twice — its discriminating-or-not status is therefore provably identical
across a pair's two games (confirmed: for every one of the 58 pairs, if the
two games' actual turn-4 moves differ, that IS turn-4 discriminating for
that pair, and if they're equal, that IS turn-4 non-discriminating — this
follows deterministically from the run's own determinism guarantee, gate 9
of `tools/ci.sh`, with no engine replay needed to establish it). Turn 5's
independence depends on whether turn 4 already agreed: if it did, turn 5's
prefix is also identical across the pair (again testing the same question
twice); if turn 4 disagreed, the pair's two turn-5 prefixes genuinely
diverge.

**Exact, deduplicated measurement, derived directly from the report's
`moves` lines (all 58 openings, the complete population):**

- Turn 4 agreement: **25 of 58 openings (43.1%)**
- Of those 25, turn 5 *also* agreeing (→ fully vacuous pair): **5 of 25 (20.0%)**
- Of those 25, turn 5 disagreeing (→ attributed via turn 5 alone): 20 of 25

`5/58 = 8.62%` is not a sampled estimate compared against a model — it **is**
the exact, full-population vacuity rate. The two component rates (43% turn-4
agreement at 5 stones on the board; 20% turn-5-given-turn-4-agreement) are
each independently plausible on the mechanism Result 1 describes, and
compose exactly to the observed count. There is no discrepancy between
"expected" and "observed" left to explain once the correct unit (opening,
not game) is used.

## Result 3 — pairing structure, confirmed

Every one of the 5 vacuous outcomes affects **both** games of its opening
simultaneously (16 AND 17; 44 AND 45; 68 AND 69; 88 AND 89; 114 AND 115) —
zero counterexamples of a "half-vacuous" pair, across the full 58-pair
population. This is not a coincidence but the structural consequence proven
in Result 2: turn 4's discriminating status is identical across a pair by
construction, and the 5 vacuous pairs are exactly the ones where that shared
turn-4 answer, and the resulting shared turn-5 answer, both happen to
coincide between staged and r2. **The correct unit of vacuity is the
opening, and the rate restated per-opening is 5/58 ≈ 8.62%** (not 10/116).

## Result 4 — end-to-end sanity via pistol-core as an independent referee

Replayed the single staged loss (the pair-57 split — see below) and 3 p4
pairs (openings 0, 1, 2 — chosen by index order, not a hardware RNG, per
this project's own prohibition on unseeded nondeterminism on any
verification path) by feeding each game's **complete, verbatim move list**
from the report into `target/release/pistol --config
configs/instrument_v0.toml` and issuing `go`. Every one of the 8 games
produced the engine's own independent win-detection refusal
(`IllegalPosition: pN completed a line on turn T`), and every T and N
matched the report's `turns`/`result` fields exactly:

| game | report: result / turns | pistol-core: winner / turn |
|---|---|---|
| 114 | p2_win / 22 | p2 / 22 |
| 115 | p2_win / 16 | p2 / 16 |
| 0 | p1_win / 15 | p1 / 15 |
| 1 | p2_win / 32 | p2 / 32 |
| 2 | p1_win / 13 | p1 / 13 |
| 3 | p2_win / 14 | p2 / 14 |
| 4 | p1_win / 19 | p1 / 19 |
| 5 | p2_win / 18 | p2 / 18 |

This is a **third, independent code path** — pistol-core's own legality/win
detection, sharing no code with the arena's scoring (`conclusion.rs`,
`score.rs`) or with the attribution script's rule-3 parity check (1b). All 8
games, including the pair containing the run's only staged loss, are
confirmed legal, six-in-a-row completions, by the exact player and turn the
report claims. **This says nothing about link 1a's open question** (whether
the *labels* staged/r2 are correctly assigned to the right seat in the 10
vacuous games) — it confirms the *games themselves*, and the winner/turn
bookkeeping downstream of them, are real and correctly scored.

**The depth figures reproduce exactly.** `compute staged: … deepest 6
turns` / `compute r2: … deepest 2 turns` (arena stdout summary) are exactly
`max(depth_a)` = 6 and `max(depth_b)` = 2 across all 116 `game` lines in the
report — re-derived directly, not approximated. The full distribution is
notable beyond the two extremes: staged reaches depth_turns 4–6 in 96 of 116
games (79 at depth 4, 16 at depth 5, 1 at depth 6) while r2 is at depth 2 in
112 of 116 (4 at depth 1). This is a **systematic, consistent** advantage
across nearly the whole run, not an artifact of one or two games — offered
as context for the architect on whether the run's extreme `nelo_pair`
(1854.8) is mechanistically plausible independent of the attribution
question: a 2–4 ply depth edge sustained across ~83% of games, in a sharp
forcing-line game, is a real candidate mechanism for a lopsided score, not
obviously a measurement artifact.

## Result 5 — where the replay-window size lives, and what widening it touches

**Instrument code**: `tools/wp15b_attribution_check.py:157`,
`for free in (opening_turns, opening_turns + 1):` — a hardcoded 2-element
tuple. `opening_turns` itself is read from the report (reflecting the book's
own depth), but the decision to check exactly that turn and the one after it
— a window of size 2 — is this literal line.

**Prereg text**: `docs/experiments/wp15b_sprt_prereg.md` §8.3, link 1a
(around line 526–528), independently commits to the same fact in prose:
"The two free turns after the book are each some engine's **FIRST search**
of the game" — the justification for why exactly 2 turns are replayable via
a simple fresh-process query is that each is that engine's *first* call.
Widening to a 3rd turn checks each engine's *second* search, which is a
different claim needing its own justification (a fresh process can still be
handed the actual recorded prefix and asked, so it's not un-replayable — but
the prose as written does not cover it, and CLAUDE.md's rule is that an
amendment to the prereg's text — however small — reopens its review).

**Driving test**: `crates/pistol-cli/tests/wp15b_attribution_check_tests.rs`
carries a fixture built for exactly this window (`OPENING_TURNS` constant,
comment "ra moves at turn 3, rb at turn 4, and the game ends there").
Widening the window without extending this fixture would violate
`tools/SHELL_CHECKLIST.md` item 10 (the shipped script's actual behavior
must be driven by a test) the moment the new turn is added to the check.

**What this means for the architect's choice.** Widening the window is not
an isolated one-line change: it touches instrument code (line 157), the
prereg's own prose (§8.3), and the driving test's fixtures — three files,
and by CLAUDE.md's own rule, a full fresh-context re-review of the prereg
regardless of which piece is edited first. The alternative — registering a
tolerated vacuity rate while leaving 1b/1c universal — changes no code and
reopens nothing, since Result 4 shows 1b and 1c already hold on all 116
games including the 10 in question; it would need only a §5-adjacent
sentence stating the tolerance and its ground (Result 2's exact 8.62%
figure, or a bound above it).

---

**No recommendation is made.** The five results above are the complete
diagnostic; the amendment-vs-tolerate ruling, the resulting re-review scope,
and the §5 confirmatory run are the architect's.
