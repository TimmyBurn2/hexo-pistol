# WP-1.6 Criterion 1'' — warm-replay attribution, closing D-401's blind spot

**Status: DESIGN. Not yet reviewed, not implemented. Queued for a
fresh-context REVIEW-design before any IMPL.**

## 1. The problem this replaces

D-401: the governed run (`staged_q(defensive_only)` vs plain `staged`, 500
openings) raised a raw `verdict h0` at 141 pairs, but Criterion 1' clause (b)
FAILED — 44 of 141 pairs (31.2%) are link-1a-vacuous, and adversarially
reassigning them flips the verdict to `inconclusive_at_game_cap`. **The run
is not a measurement. That run is never read, under any criterion, ever —
this line and the ADR both state it verbatim.**

31.2% is not a marginal rate. WP-1.5b's own comparable check (staged vs
radius-2, a much larger behavioral difference) measured 8.62% at the same
2-turn-window check. **Verified this session, mechanistically, not just by
rate comparison**: sampled 8 of the 44 vacuous openings (2, 20, 40, 58, 70,
91, 108, 133), reconstructed the exact board position at each engine's two
replayed free turns (16 positions total) via a throwaway diagnostic calling
`pistol_solver::ThreatState::{can_win_this_turn, blocking_covers}` directly —
the same two queries `quiescence.rs::gate_row` calls — rather than inferring
from CLI output. **Result: 16/16 positions show `win_now=false`,
`overload=false`; 15/16 show `defensive_t_le_1=false` outright, and the one
exception (opening 133, turn 5) finds only `Cover::Minimal` entries with
`MinimalCover::Two` (t=2), never `MinimalCover::One` (t<=1) — so trigger (b)
still does not fire.** At 5-7 stones on the board, no opponent plan has
reached `own >= 4` yet (`DEF-PLAN`'s own threshold), so `blocking_covers`
has nothing to find and the quiescence gate falls straight to `StandPat` in
every sampled case — `staged_q` and plain `staged` are byte-identical at the
horizon because the mechanism this WP built has, correctly, nothing to
extend yet. **This is not a defect in the extension or in the checker: it is
this specific matchup's own early-game behavior, and it is exactly the shape
D-383's diagnostic already found for a completely different matchup** —
"early, low-stone-count positions are where the two candidate policies'
shortlists most overlap" (`wp15b_attribution_window_design.md` §3, quoting
`wp15b_vacuity_diagnostics.md` Result 1). WP-1.6's own version of that effect
is simply stronger, because trigger (b)'s activation condition is a rarer
event this early than a generation-policy's shortlist overlap.

## 2. What was already tried and killed for exactly this failure mode — not repeated here

D-383 (`wp15b_attribution_window_design_REVIEW.md`, 2 BLOCKING) killed
"widen the replay window to more turns, one cold subprocess per query"
outright: MEASURED on the actual governed-run data, a cold-subprocess replay
of any turn past an engine's *first* free search disagrees with what that
same engine, played live, actually answered — because the live engine's
transposition table warms continuously across every search of its own game
(`instance.rs`'s `set_position` never clears `self.searcher`; only
`new_game` does), while a fresh subprocess-per-query is always cold. Turn
`opening_turns` and `opening_turns+1` agree only because they are each
engine's absolute first search of the game — nothing later is trustworthy
under that architecture. **A wider window without warm state is not a
smaller version of this design; it is the design D-383 already falsified.**

## 3. The selected design: one persistent process per engine per game, warm state matched to the live search that actually happened

1. **Mechanism.** Per game, spawn one persistent engine process per seat —
   exactly matching the arena's own architecture
   (`crates/pistol-arena/src/game.rs`'s "one game, two subprocesses",
   `schedule.rs`'s per-game `Channel::start`). Feed that seat's own recorded
   move list in order, turn by turn: at each of ITS turns, send
   `position`/`go` under the report's own recorded budget and compare the
   answer to the move the report credits it with. This reproduces the exact
   TT-warming sequence the live game had — instrument-mode determinism (a CI
   gate, CLAUDE.md rule 4) is what makes the replay BIT-FAITHFUL to the
   original search, not merely plausible.
2. **Attribution.** A pair is ATTRIBUTED at its first divergent move —
   direct evidence a fresh cold-subprocess replay could never produce past
   turn one, now available for the WHOLE game because warm state is matched.
3. **Inert pairs, a theorem, not a tolerance.** A pair's two games share an
   identical opening-book prefix with the seat assignment reversed (already
   true of every pair in this arena). If BOTH games' own credited engines
   warm-replay with ZERO divergence across the ENTIRE game, AND the two
   games' actual recorded move lists are IDENTICAL turn for turn, then the
   two engines are behaviorally indistinguishable at every position either
   game ever reached: swapping which label sat in which seat could not have
   produced a different board at any ply, hence not a different result.
   Whichever PLAYER-INDEX (not label) wins one game necessarily wins the
   other too (same board, same rules), and since the two games swap which
   LABEL occupies that index, this is a FORCED 1-1 split — a `p2` pentanomial
   bucket the pair could not have avoided regardless of which engine is
   actually stronger. This is provable from the replay data itself, not an
   adversarial worst-case assumption over an unknown alternative. Such a
   pair is EXCLUDED from adversarial flipping by this proof, not by
   tolerating an unresolved ambiguity.
4. **Criterion 1''.** A report is a measurement iff (a) zero
   divergence-confirmed inversions — the credited engine, warm-replayed
   under CORRECT seat assignment, matches its own recorded move at every
   turn of every game — and (b) every NON-INERT pair is directly attributed
   by first divergence. The old clause (b)'s adversarial-reassignment
   machinery is KEPT, but only as a cross-check run over the INERT pairs
   alone (expected to be a no-op, since §3's theorem already fixes their
   bucket) — its result is cited in the report as confirming evidence, not
   as the thing the verdict depends on.
5. **Determinism violations are a different, bigger finding.** If a game's
   credited engine, warm-replayed under the CORRECT seat assignment it was
   actually given, does NOT reproduce its own recorded move, that is not an
   attribution failure — instrument mode guarantees determinism (rule 4), so
   this is that guarantee failing, somewhere in the engine, independent of
   which WP is running. This gets its OWN loud, distinct exit code and
   blocks everything past it — it is bigger than WP-1.6 and is reported as
   such, never folded into an attribution-FAIL count.
6. **Cost.** Warm-replay re-runs, in full, every turn every seat actually
   searched in the original run — structurally ~1x the original run's own
   compute (a second full pass, not a handful of extra queries the way
   window-widening was). DECLARED here; the first execution's actual
   `time_ms` is what makes it MEASURED (§9-equivalent slot, prereg rev 3).
   The OLD cold-2-turn check is not deleted — the tests and historical
   reports that cite `tools/wp15b_attribution_check.py` at its own pinned
   revision keep working exactly as before; this design adds a NEW,
   separately-named instrument rather than mutating the one WP-1.5b's own
   closed reviews already govern.

## 4. What changes, concretely

1. **New file**, successor-named — `tools/wp16_warm_attribution_check.py`
   (the successor name the dispatch licenses; `tools/wp15b_attribution_check.py`
   is untouched, kept exactly as WP-1.5b's own reviews left it, since
   retroactively changing what an already-cited instrument does would reopen
   reviews this WP has no standing to touch). Implements §3's mechanism:
   per-game persistent subprocess pairs, first-divergence attribution, the
   inert-pair theorem, the adversarial cross-check demoted to inert pairs
   only, and the determinism-violation exit path.
2. **`docs/experiments/wp16_sprt_prereg.md` §7A.1**, revision 3: Criterion
   1'' replaces Criterion 1' verbatim, the replay cost stated (DECLARED
   ~1x, MEASURED after the first real execution), every other section
   unchanged. An amendment, reopening this document's own already-passed
   review (D-400) exactly as CLAUDE.md's rule requires — no way around that,
   and this design does not try to.
3. **Driving tests** (`tools/SHELL_CHECKLIST.md` item 10 — a shipped
   behavior with no test driving it is the coverage hole this whole
   instrument class exists to close): (i) a synthetic report with a swapped
   seat, caught at first divergence; (ii) a synthetic inert-pair fixture
   (identical move lists, zero divergence both games) recognized, excluded
   from the load-bearing criterion, its no-op cross-check confirmed green;
   (iii) a synthetic genuinely-divergent pair, correctly attributed by first
   divergence rather than by the old 2-turn window; (iv) a synthetic
   determinism-mismatch case (correct seat assignment, replay disagrees with
   the recorded move anyway), confirmed to exit loudly and distinctly from
   an attribution FAIL.

## 5. What is explicitly NOT decided here

Whether D-401's own 44 vacuous pairs, if warm-replayed, would resolve to
"all inert" or "some genuinely misattributed" is an empirical question for
whoever next inspects that report — **moot regardless of the answer, since
this dispatch's own instruction is that D-401's run is never read under any
criterion, ever.** §1's mechanistic verification (8/8 openings, trigger (b)
inactive) is offered as WHY building this is worth it, not as a
re-litigation of that run's verdict. Whether Step 5's fresh run lands `h0`
or `h1` is this design's to make measurable, not to predict.

## 6. Review record

*(filled in after dispatch)*
