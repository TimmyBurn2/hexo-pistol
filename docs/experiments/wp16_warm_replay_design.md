# WP-1.6 Criterion 1'' — warm-replay attribution, closing D-401's blind spot

**Status: DESIGN, revision 2. Revision 1 (`4eb13e4`) FAILED its first
fresh-context review — 3 BLOCKING, 2 MAJOR, 2 MINOR. Queued for a second,
final fresh-context REVIEW-design before any IMPL; per this WP's own
dispatch, a second FAIL is a hard stop for the whole effort.**

**What changed and why, so a reviewer knows what not to re-litigate.**
Revision 1's reviewer found the design technically sound in its core
mechanism (persistent, warm-state-matched replay correctly targets D-383's
root cause; the report format carries everything a replay needs; the
inert-pair deduction is valid for ordinary decisive games) but broken or
underspecified in three load-bearing ways, plus two narrower gaps:

- **BLOCKING 1 — no mechanism distinguished an attribution failure from a
  determinism violation.** Both look identical from "the credited engine's
  replay doesn't match the recorded move." Fixed in §4 point 4 below with a
  concrete dual-engine cross-check, reusing the pattern
  `tools/wp15b_attribution_check.py`'s own link 1a already uses (query BOTH
  configs, not only the credited one).
- **BLOCKING 2 — the inert-pair theorem didn't handle forfeits.** A
  forfeit-ending game has no recorded MOVE at its outcome-deciding ply, so
  "zero divergence across the entire game" was vacuously true there, not
  proven. Fixed in §4 point 3: any pair with a forfeited game is excluded
  from inert-pair eligibility outright.
- **BLOCKING 3 — no option matrix for the core mechanism**, in a project
  where CLAUDE.md makes that a hard rule for any named decision with more
  than one viable option. Fixed with new §3 below — a real matrix, weighing
  a hand-rolled Python persistent-process protocol against reusing
  `pistol-arena`'s own Rust replay machinery, grounded in reading the actual
  wire protocol (`exchange.rs::ask`/`position_line`) rather than asserted.
- **MAJOR — what happens to the rest of a game's replay after a divergence
  was unstated.** Fixed: §4 point 1 now states explicitly that a diverging
  process is halted immediately, never fed further recorded moves.
- **MAJOR — the `nodes`-only budget restriction was implicit.** Fixed: §4
  point 1 now states it explicitly, inheriting the same restriction
  `tools/wp15b_attribution_check.py` already carries and for the same reason
  (determinism reproducibility, CLAUDE.md rule 4).
- **MINOR — "whichever player-index wins" didn't fit capped-capped pairs**
  (already forced to `p2` by `score_a`'s own unconditional scoring,
  independent of the theorem). Tightened in §4 point 3.
- **MINOR — the determinism-mismatch test (§5 item 3(iv)) could only check
  routing, not the classification itself.** Fixed: §5 now adds a case built
  on genuinely ambiguous replay data, exercising the new cross-check the
  BLOCKING-1 fix introduces.

**On dispatching this review as this effort's SUBAGENT 1, twice.** The
dispatch commissioning this design licenses exactly two subagent slots for
the whole effort (design review, prereg review); it does not license a
separate DECISION-RED-TEAM dispatch for §3's new option matrix. Rather than
spawn a third subagent outside that license, the SAME design-review slot's
second dispatch is asked to review this revision's technical fixes AND
attack §3's matrix and recommendation — both are "fresh-context, attacks the
premise" work over one document, and this is recorded here rather than done
silently.

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

## 3. Option matrix — how warm replay is realized, not merely that it is

**The decision**: what actually speaks the live per-turn protocol during
replay. Three options, attacked below rather than the first one adopted by
assertion (revision 1's BLOCKING 3).

**Grounding, read from the source rather than assumed.** The arena's own
per-turn exchange (`crates/pistol-arena/src/exchange.rs::ask`) sends exactly
two lines — `position_line(moves)` (`position start moves <every turn
so far, restated cumulatively>`, `position_line`, line 177-184) then
`rules.go_line` — and reads until a `bestmove`/forfeit line, checking for a
stray unsolicited line both before sending (`Channel::unsolicited`) and
after receiving (`Channel::receive`). **This is byte-identical to what
`tools/wp15b_attribution_check.py`'s existing one-shot `subprocess.run`
calls already send** (`position start moves <prefix>\n<budget line>\nquit\n`,
read the `bestmove` line) — the wire grammar is not new or unproven; only
the process LIFECYCLE differs (one query per fresh process, vs. many queries
against one persistent process). `schedule.rs`'s `one_game` spawns one fresh
`Channel` pair per GAME (not shared across a match), matching what a warm
replay needs to mirror.

**Option A — hand-rolled Python, one persistent `subprocess.Popen` per seat
per game, mirroring `exchange.rs::ask`'s exact two-line shape.**
- *Cost*: ESTIMATED LOW. The grammar is proven (already used one-shot); the
  new surface is `Popen` lifecycle management (spawn once per game, keep
  stdin/stdout pipes open, write `position`/`go` per turn, read to
  `bestmove`) — a standard, well-understood Python idiom, not a new
  protocol. Stays in the same language as the checker it extends and the
  report-parsing/statistics layer it must share code with either way.
- *Failure mode*: a Python reimplementation is a SECOND place the protocol
  is spoken, so a future protocol change (D-2's verb set) could update
  `exchange.rs` and silently leave this driver stale — this is real and is
  why item 3(iii) below is specified as a drift check, not merely a
  functionality test. Revision 1's BLOCKING 3 named this risk in the
  abstract; grounded against the actual protocol text above, the risk is
  specifically about LIFECYCLE drift (does a fresh process really start
  "new game" for free, does the queue-draining discipline hold), not about
  GRAMMAR drift, since the grammar this design must speak is exactly what
  the existing, already-reviewed one-shot tool already speaks correctly.
- *Mitigation this revision adds*: §4 point 1 now states the process
  lifecycle precisely (spawn once per game; no explicit `new_game` verb
  needed, since a freshly spawned `Pistol` starts empty by construction,
  the same fact `schedule.rs`'s own one-fresh-channel-per-game pattern
  relies on; drain `unsolicited()`-equivalent state before each `position`
  send, mirroring `exchange.rs`'s own discipline).

**Option B — a new Rust replay mode, reusing `pistol-arena`'s own
`game.rs`/`schedule.rs`/`exchange.rs`/`channel.rs` directly.**
- *Cost*: ESTIMATED MODERATE-HIGH. `game::play`'s loop generates NEW moves
  and records them; replay needs to instead FEED a fixed, already-recorded
  move list and compare — not a drop-in reuse, but a new function/mode
  alongside it, plus visibility changes (several of the modules above are
  presently `pub(crate)` to `pistol-arena`). Touches a crate every other
  closed WP's own SPRT reports already depend on for report GENERATION,
  even if the new code path is additive.
- *Failure mode*: ELIMINATES protocol-drift risk entirely by construction —
  same code generates and replays. The cost is schedule and surface area:
  a new binary or subcommand, its own tests, and review scope on
  `pistol-arena` rather than a standalone script.

**Option C — hybrid: Python owns report-parsing/statistics (unchanged from
today's tool), a new minimal Rust replay subcommand (reusing Option B's
internals) owns the live protocol exchange, invoked as a subprocess exactly
as `target/release/pistol` already is.**
- *Cost*: ESTIMATED HIGH — carries both Option A's Python surface (for
  everything upstream of replay) and Option B's Rust surface (for replay
  itself), for the smallest possible protocol-drift window.

**Recommendation: Option A, specified precisely rather than left to
implementer discretion.** Revision 1's actual defect was not "Python is the
wrong language" — the reviewer's own confirmed findings show the grammar
this design needs is already correctly spoken by the existing one-shot tool,
and `schedule.rs`/`exchange.rs` confirm a fresh process already gets a
correct "new game" state without any extra verb. Revision 1 earned BLOCKING
3 because §3 (old) described the mechanism ("feed that seat's own recorded
move list in order... send `position`/`go`") without grounding it against
the actual protocol text, leaving an implementer to invent the lifecycle
details rather than mirror them precisely. §4 below now pins those details
against `exchange.rs` directly. **This is not free of Option A's structural
risk (protocol drift over time) — that risk is real and is why the design
does not delete `tools/wp15b_attribution_check.py`'s own protocol-speaking
code, so a future divergence between the two is at least detectable by
diffing them, and why item 3(iii)'s test below is a drift check and not
merely a green light.** If a future REVIEW-impl or RED-TEAM finds this
Python replica has drifted from the arena's real protocol, Option B/C is the
named escalation — recorded here as a live path, not dismissed.

## 4. The selected design: one persistent process per engine per game, warm state matched to the live search that actually happened

1. **Mechanism, pinned against `exchange.rs`.** Per game, spawn one
   persistent engine process per seat (`subprocess.Popen`, matching
   `schedule.rs`'s one-fresh-channel-per-game pattern) — no explicit
   `new_game` verb is sent or needed, since a freshly spawned process starts
   with an empty board and a cold TT by construction, the same fact the
   arena's own per-game channel spawn relies on. At each of that seat's own
   turns, in order, send EXACTLY the two lines `exchange.rs::ask` sends —
   `position start moves <every turn so far, restated cumulatively>` then
   the recorded `go` line under the run's own recorded `budget` — and read
   to `bestmove`, draining any stray unsolicited line first (mirroring
   `Channel::unsolicited`'s discipline). **Refuses any budget kind other
   than `nodes`, by name, before replaying anything** — the same restriction
   `tools/wp15b_attribution_check.py` already carries, for the same reason:
   only a `nodes` budget is reproducible under instrument-mode determinism
   (CLAUDE.md rule 4); a `movetime_ms`-budgeted report cannot be
   bit-faithfully replayed and this design does not pretend otherwise.
   Compare the answer to the move the report credits that seat with.
   **On ANY divergence, that process's replay HALTS IMMEDIATELY** — no
   further recorded moves are fed to it, and no further comparison is drawn
   from it for that game. Continuing past a divergence would feed the
   process an ORIGINAL move it did not itself choose, which would desync
   its TT from what the live game actually had at that point, making any
   later "divergence" in the same replay meaningless (revision 1's MAJOR
   finding).
2. **On divergence: classify with BOTH engines, not one.** The single
   biggest gap in revision 1 (BLOCKING 1): querying only the credited engine
   cannot distinguish "wrong label" from "nondeterminism," because both
   produce the identical symptom (credited engine's replay disagrees with
   the recorded move). On a divergence at turn T, take ONE additional COLD
   query (a fresh one-shot process is sufficient here — this is a diagnostic
   probe, not part of the warm chain being verified) of the OTHER
   (non-credited) engine's config, at the SAME prefix (`position start moves
   <prefix up to T>`, same budget):
   - **The other engine's answer equals the recorded move** → the report's
     label assignment for this seat is the wrong one — a CONFIRMED
     INVERSION under clause (a) below, exactly the class link 1a's own name
     always meant. Reuses the same dual-engine-query pattern
     `tools/wp15b_attribution_check.py`'s existing link 1a already runs at
     its two checked turns — extended here to whichever turn the divergence
     actually falls on, not merely the first two.
   - **Neither engine's fresh answer equals the recorded move** → nothing
     currently known explains what was actually played. Instrument mode
     guarantees determinism (rule 4); this is that guarantee failing, not an
     attribution question — a DETERMINISM VIOLATION, its own loud, distinct
     exit code, blocking everything past it (bigger than this WP, reported
     as such, never folded into an attribution-FAIL count).
3. **Inert pairs, a theorem, not a tolerance — decisive, non-forfeit games
   only.** A pair's two games share an identical opening-book prefix with
   the seat assignment reversed (already true of every pair in this arena).
   **A pair is eligible for this exclusion only if BOTH its games end
   `normal` (non-forfeit).** A forfeit's outcome-deciding event has no
   recorded MOVE to warm-replay against — "zero divergence across the entire
   game" would be vacuously true at exactly the ply that decided the result,
   not proven there, which is precisely revision 1's BLOCKING 2. For a pair
   of two `normal`-ending games: if BOTH games' own credited engines
   warm-replay with ZERO divergence across every recorded move, AND the two
   games' actual recorded move lists are IDENTICAL turn for turn, then the
   two engines are behaviorally indistinguishable at every position either
   game ever reached — confirmed independently by each game's own replay,
   not assumed. Swapping which label sat in which seat could not have
   produced a different board at any ply, hence not a different result:
   whichever PLAYER-INDEX (not label) wins one game necessarily wins the
   other too (same board, same rules), and since the two games swap which
   LABEL occupies that index, this is a FORCED 1-1 split — a `p2`
   pentanomial bucket the pair could not have avoided regardless of which
   engine is actually stronger. (A pair of two CAPPED games is already
   forced to `p2` by `score_a`'s own unconditional 0.5-both scoring,
   independent of this theorem — the theorem's own work is for ordinary
   decisive games, and capped-capped pairs are noted here only so the
   language above, which presumes a winner, is not misread as claiming
   something new about them.) This is provable from the replay data itself,
   not an adversarial worst-case assumption over an unknown alternative.
   Such a pair is EXCLUDED from adversarial flipping by this proof, not by
   tolerating an unresolved ambiguity.
4. **Criterion 1''.** A report is a measurement iff (a) zero
   divergence-confirmed inversions — every divergence found in point 2 above
   resolves to either "no divergence" or "confirmed inversion" (the
   other-engine match case), never left unclassified — and (b) every
   NON-INERT pair (§4 point 3's exclusion, forfeits always non-inert) is
   directly attributed by first divergence. A DETERMINISM VIOLATION (point 2's
   other branch) is checked FIRST and, if found anywhere, stops the whole
   evaluation before (a)/(b) are even asked, per its own exit code. The old
   clause (b)'s adversarial-reassignment machinery is KEPT, but only as a
   cross-check run over the INERT pairs alone (expected to be a no-op, since
   §4 point 3's theorem already fixes their bucket) — its result is cited in
   the report as confirming evidence, not as the thing the verdict depends
   on.
5. **Cost.** Warm-replay re-runs, in full, every turn every seat actually
   searched in the original run — structurally ~1x the original run's own
   compute (a second full pass, not a handful of extra queries the way
   window-widening was), plus the diagnostic branch's own single extra cold
   query per divergence (expected rare — zero on a clean report). DECLARED
   here; the first execution's actual `time_ms` is what makes it MEASURED
   (§9-equivalent slot, prereg rev 3). The OLD cold-2-turn check is not
   deleted — the tests and historical reports that cite
   `tools/wp15b_attribution_check.py` at its own pinned revision keep
   working exactly as before; this design adds a NEW, separately-named
   instrument rather than mutating the one WP-1.5b's own closed reviews
   already govern.

## 5. What changes, concretely

1. **New file**, successor-named — `tools/wp16_warm_attribution_check.py`
   (the successor name the dispatch licenses; `tools/wp15b_attribution_check.py`
   is untouched, kept exactly as WP-1.5b's own reviews left it, since
   retroactively changing what an already-cited instrument does would reopen
   reviews this WP has no standing to touch). Implements §4's mechanism:
   per-game persistent subprocess pairs pinned against `exchange.rs`'s own
   two-line shape, halt-on-divergence, the dual-engine classification
   branch, the inert-pair theorem (forfeits excluded), the adversarial
   cross-check demoted to inert pairs only, and the determinism-violation
   exit path checked first.
2. **`docs/experiments/wp16_sprt_prereg.md` §7A.1**, revision 3: Criterion
   1'' replaces Criterion 1' verbatim, the replay cost stated (DECLARED
   ~1x, MEASURED after the first real execution), every other section
   unchanged. An amendment, reopening this document's own already-passed
   review (D-400) exactly as CLAUDE.md's rule requires — no way around that,
   and this design does not try to.
3. **Driving tests** (`tools/SHELL_CHECKLIST.md` item 10 — a shipped
   behavior with no test driving it is the coverage hole this whole
   instrument class exists to close): (i) a synthetic report with a swapped
   seat, caught at first divergence, classified as a CONFIRMED INVERSION via
   the dual-engine check (not merely flagged and left unclassified); (ii) a
   synthetic inert-pair fixture (identical move lists, zero divergence both
   games, both `normal`-ending) recognized, excluded from the load-bearing
   criterion, its no-op cross-check confirmed green, AND a sibling fixture
   identical except one game FORFEITS, confirmed NOT excluded (BLOCKING 2's
   own fix, driven); (iii) a synthetic genuinely-divergent pair, correctly
   attributed by first divergence rather than by the old 2-turn window, AND
   a drift check comparing this driver's own two sent lines byte-for-byte
   against `exchange.rs::ask`/`position_line`'s own output on the same
   input, so a future protocol change that edits one without the other is
   caught here rather than silently; (iv) TWO determinism-mismatch cases —
   the original labeled case (correct seat assignment, replay disagrees,
   confirmed exit loudly and distinctly from an attribution FAIL) AND a
   genuinely ambiguous case built so that, under the OLD single-engine-query
   mechanism, it would have been indistinguishable from an attribution
   failure — confirming the NEW dual-engine cross-check (§4 point 2)
   actually discriminates it correctly, not merely that a pre-labeled input
   reaches its intended branch (revision 1's second MINOR finding).

## 6. What is explicitly NOT decided here

Whether D-401's own 44 vacuous pairs, if warm-replayed, would resolve to
"all inert" or "some genuinely misattributed" is an empirical question for
whoever next inspects that report — **moot regardless of the answer, since
this dispatch's own instruction is that D-401's run is never read under any
criterion, ever.** §1's mechanistic verification (8/8 openings, trigger (b)
inactive) is offered as WHY building this is worth it, not as a
re-litigation of that run's verdict. Whether Step 5's fresh run lands `h0`
or `h1` is this design's to make measurable, not to predict.

## 7. Review record

*(filled in after dispatch)*
