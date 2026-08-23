# WP-1.5b Criterion 1 — replay-window design: adaptive walk, not a fixed count

**Status: DESIGN. Not yet reviewed, not implemented. Queued for a
fresh-context REVIEW-design before any IMPL.**

## 1. The problem this replaces

`tools/wp15b_attribution_check.py`'s link 1a currently checks exactly
`(opening_turns, opening_turns + 1)` — each engine's first free search, two
turns per game — and refuses a game where both replayed turns agree
("vacuous"). `docs/experiments/wp15b_vacuity_diagnostics.md` measured this
against the governed run (D-381): an exact, deduplicated 5/58 openings
(8.62%) hit full 2-turn vacuity, with no evidence of a labeling defect (1b
and 1c hold universally; 8 independently-replayed games via pistol-core all
match; the mechanism is plausible convergent early-game play, not a bug) —
but the 2-turn check cannot distinguish "these two policies happen to agree
briefly" from "these two policies are mislabeled," because it never looks
far enough to find out.

## 2. What was tried and rejected: a wider FIXED window

The natural first idea — extend the checked window from 2 turns to some
larger fixed N — was tested empirically before being adopted, per this
project's own rule against un-measured design claims. For each of the 5
vacuous openings, the paired games' actual recorded move lists were
compared index-by-index (no engine replay needed: since both games of a
pair share an identical prefix by construction, the first index at which
their actual moves differ is *exactly* the first turn at which the
underlying engines disagree — proven in
`wp15b_vacuity_diagnostics.md` Result 2/3's pairing argument, reused here).
**The first-divergence point varies from 2 to 10 turns past the current
window, per opening:**

| opening | games (lengths) | first divergence | turns past current window |
|---|---|---|---|
| 8  | 16/17 (15/16t) | turn 7  | +3 |
| 22 | 44/45 (15/16t) | turn 6  | +2 |
| 34 | 68/69 (33/20t) | turn 7  | +3 |
| 57 | 114/115 (22/16t) | turn 10 | +6 |
| 44 | 88/89 (17/14t) | turn 13 | +9 |

**A fixed window sized to catch all 5 would need to reach turn 13** — on
opening 44's pair, that is 13 of the shorter game's own 14 turns, i.e.
almost the entire game. A fixed window small enough to be cheap (say, +2 or
+3) resolves only 1–3 of the 5 cases. There is no principled stopping point
for a fixed N: the next run's book could produce an opening that agrees for
15 or 20 turns, and nothing about the mechanism (Result 1's observation that
early, low-stone-count positions are where the two candidate policies'
shortlists most overlap) bounds how long that can persist. A fixed window is
therefore either too narrow to be trustworthy or an arbitrary, unjustified
guess at how wide is "enough" — the exact failure mode CLAUDE.md's
estimate-vs-measured discipline exists to catch.

## 3. The selected design: walk the game's own real history until it speaks

Replace the fixed two-element tuple with a walk over the game's *actual
recorded turns*, starting at `opening_turns`, continuing turn by turn using
the real prefix at each step (exactly the technique already used for the
first two turns — nothing about replaying turn 6 is less valid than
replaying turn 4; both hand a fresh process the real recorded prefix and
ask), **stopping at the first turn where the two configs disagree**. If no
turn in the entire game discriminates, that game is genuinely vacuous — a
categorically stronger finding than today's, since it now means "two
different search policies produced completely parallel play for this
game's *entire real length*," not merely "for two arbitrarily-chosen early
turns."

**Why this is not a bigger technical claim than the current check makes.**
Link 1a's referent has always been "a fresh process handed the actual
recorded prefix reproduces the recorded move exactly" (§8.3, prereg). That
premise does not weaken as the prefix grows longer — it is still the real,
already-decided history, not a hypothetical continuation. The "two free
turns, each engine's FIRST search" framing in the current prereg text is a
description of what was checked, not a boundary the underlying technique
requires. Walking further is doing more of the same valid thing, not a
different, shakier thing.

**Cost.** For the 106 of 116 games that already discriminate within the
current 2-turn window, behavior and cost are unchanged (the walk exits on
the same first turn it always did, with an early exit — no re-check of
turns already passed). Only the games that don't discriminate quickly need
extra replays: this run's own 5 vacuous openings would have cost at most
13, 10, 7, 7, and 6 replays respectively (worst case ~43 extra searches
total across 10 games) instead of 2 each — a few tens of seconds at this
run's ~0.3-0.7s/search, not a material change to the check's economics.

**What this design does NOT do.** It does not touch the arena, the SPRT
statistics, `score.rs`, `conclusion.rs`, or any engine crate — it is
confined to `tools/wp15b_attribution_check.py`'s link-1a loop, the prereg's
own §8.3 prose describing that loop, and that script's driving test
fixtures. It changes no committed config and reopens no review outside this
document's own chain (the prereg's, per its standing rule that any
amendment — however small — reopens it).

## 4. What changes, concretely

1. `tools/wp15b_attribution_check.py`, link 1a's loop: replace
   `for free in (opening_turns, opening_turns + 1):` with a walk over
   `range(opening_turns, len(played))`, breaking out of the inner iteration
   (not `continue`-ing past a fixed pair) the first time a game's `here`
   count reaches 1 — i.e. keep querying only until that game is attributed,
   then move to the next game. `checked`/`discriminating` notes must still
   report meaningfully (e.g. total turns actually queried this run, which
   will now vary run to run — the note's wording needs to say so rather
   than implying a fixed `2 * n_games` count).
2. `docs/experiments/wp15b_sprt_prereg.md` §8.3, link 1a's prose: replace
   "the two free turns after the book are each some engine's FIRST search"
   with a description of the walk and its stopping conditions (discriminate,
   or exhaust the game). This is a prereg text amendment and reopens its
   review per CLAUDE.md's own binding rule — no way around that, and this
   design does not try to.
3. `crates/pistol-cli/tests/wp15b_attribution_check_tests.rs`: the existing
   fixture ends the synthetic game exactly at the old window's boundary, so
   it exercises only "discriminate immediately" and (via corruption) the
   three link-specific failure injections. Two new cases are needed to
   drive the new behavior (`tools/SHELL_CHECKLIST.md` item 10 — a shipped
   behavior with no driving test is the coverage hole this whole instrument
   exists to close): (a) a synthetic game where the two engines agree for
   several turns before diverging, confirming the walk finds the later
   discriminating turn rather than stopping early or missing it; (b) a
   synthetic game where the two engines never diverge for the whole game,
   confirming the walk correctly reports genuine full-game vacuity rather
   than silently passing or hanging.

## 5. What is explicitly NOT decided here

Whether the 5 openings this run found are re-attributed cleanly by the new
walk is an IMPL-time empirical question (Result-1-style spot check,
expected to resolve all 5 given §2's table), not asserted here as a given.
Whether the confirmatory run (prereg §5, the separate below-100-pair-floor
blocker) proceeds on `openings_v1.txt` is a decision downstream of this
design landing and being re-reviewed — not this document's to make.

## 6. Review record

*(filled in after dispatch)*
