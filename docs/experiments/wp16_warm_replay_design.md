# WP-1.6 Criterion 1'' — warm-replay attribution, closing D-401's blind spot

**Status: DESIGN, revision 3. Revision 2 (`d9a2852`) FAILED its second,
final fresh-context review — 1 BLOCKING, 1 MAJOR, both in the Option
Matrix's own grounding claims, per the dispatch's own "second FAIL -> STOP"
rule (D-404, a hard stop). This revision proceeds only because two architect
rulings license a THIRD attempt under a structurally different discipline —
quoted verbatim below — not because this session reopened the question on
its own initiative.**

**RULING A (receipts), quoted verbatim from the dispatch commissioning this
revision**: "every arena/engine behavior claim carries its receipt inline:
command + quoted output, or file:line the reviewer can open. A described
behavior without a receipt is a finding by definition." **Applied as a
standing rule for this document from here on**: every claim in this
revision about what arena or engine code actually does names the exact
file and line(s), or quotes the exact command and its output, at the point
the claim is made — not as a separate bibliography, not asserted first and
sourced later.

**RULING B (code identity), quoted verbatim**: "STRIKE every wire-protocol
description paragraph. The mechanism section now says: the replay driver
reuses `schedule.rs::one_game`'s own spawn-and-drive path inside a
pistol-arena replay mode (subcommand or flag), feeding the report's
recorded moves and comparing the engine's move at each of its turns;
protocol behavior (NEW_GAME included) is inherited by code identity, pinned
by driving tests, not described. The Python checker consumes the replay's
divergence summary only."

**Why this structurally resolves D-404, not merely patches it.** Both
review rounds' BLOCKING findings were the SAME defect class wearing
different clothes: revision 1 asserted "one persistent process per seat,
feed the game's own move list, send `position`/`go`" without grounding it
against `exchange.rs`; revision 2 grounded MOST of that correctly but still
asserted "no explicit `new_game` verb is sent or needed" — a claim about
protocol LIFECYCLE that was false, and false in a paragraph written
specifically to fix the PRIOR round's ungrounded-claims defect. Two rounds,
two independently-wrong hand-written descriptions of the same real code.
Ruling B removes the class by removing the hand-written description
entirely: the replay driver's protocol behavior is not a paragraph in this
document to get right or wrong, it is `crates/pistol-arena/src/exchange.rs`
and `crates/pistol-arena/src/schedule.rs`'s own code, invoked directly. A
third mis-description is not possible if there is no third description.

**The buildability check Ruling B's own text requires before proceeding**
("If a concrete blocker to Ruling B is found... STOP and report; no silent
reversion to protocol re-description") **was performed this session, before
writing the rest of this revision, and found no blocker** — see §3's
grounding for the receipts.

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
the same two queries `crates/pistol-search/src/quiescence.rs::gate_row`
calls (lines 182 and 186 of that file at the revision this line was
written) — rather than inferring from CLI output. **Result: 16/16 positions
show `win_now=false`, `overload=false`; 15/16 show `defensive_t_le_1=false`
outright, and the one exception (opening 133, turn 5) finds only
`Cover::Minimal` entries with `MinimalCover::Two` (t=2), never
`MinimalCover::One` (t<=1) — so trigger (b) still does not fire.** At 5-7
stones on the board, no opponent plan has reached the threshold DEF-PLAN
names — `docs/research/threat_calculus_v1.md:29`: "DEF-PLAN | plan π | the
empty-cell set (size ≤2) of an open window with ≥4 own stones... `[PROVEN]`"
— so `blocking_covers` has nothing to find and the quiescence gate falls
straight to `StandPat` in every sampled case — `staged_q` and plain
`staged` are byte-identical at the horizon because the mechanism this WP
built has, correctly, nothing to extend yet. **This is not a defect in the
extension or in the checker: it is this specific matchup's own early-game
behavior, and it is exactly the shape D-383's diagnostic already found for
a completely different matchup** — "early, low-stone-count positions are
where the two candidate policies' shortlists most overlap"
(`wp15b_attribution_window_design.md` §3, quoting
`wp15b_vacuity_diagnostics.md` Result 1). WP-1.6's own version of that
effect is simply stronger, because trigger (b)'s activation condition is a
rarer event this early than a generation-policy's shortlist overlap.

## 2. What was already tried and killed for exactly this failure mode — not repeated here

D-383 (`wp15b_attribution_window_design_REVIEW.md`, 2 BLOCKING) killed
"widen the replay window to more turns, one cold subprocess per query"
outright: MEASURED on the actual governed-run data, a cold-subprocess replay
of any turn past an engine's *first* free search disagrees with what that
same engine, played live, actually answered — because the live engine's
transposition table warms continuously across every search of its own game.
**Receipt**: `crates/pistol-engine/src/instance.rs:95-101` —
`fn set_position(&mut self, spec: &PositionSpec) -> Result<(), EngineError>`
replays `self.state` only and never references `self.searcher` anywhere in
its body; `crates/pistol-engine/src/instance.rs:90-93` —
`fn new_game(&mut self)` is the one method that calls
`self.searcher.clear()`. A fresh subprocess-per-query is always cold. Turn
`opening_turns` and `opening_turns+1` agree only because they are each
engine's absolute first search of the game — nothing later is trustworthy
under that architecture. **A wider window without warm state is not a
smaller version of this design; it is the design D-383 already falsified.**

## 3. Option matrix — how warm replay is realized, not merely that it is

**The decision**: what actually speaks the live per-turn protocol during
replay. Attacked below with receipts, per Ruling A, rather than described
and hoped correct, per Ruling B.

**The buildability check.** Before recommending code-identity reuse, this
session read the actual functions Ruling B names, to confirm reuse is
possible without a behavior-changing modification to the existing
report-generating path (Ruling B's own STOP condition).

- **Receipt — `crates/pistol-arena/src/game.rs:42-131`,
  `pub fn play(opening, a_is_p1, index, channels, rules)`.** Its loop
  (lines 70-130) determines the mover from `state.to_move()` (line 82),
  calls `ask(&mut channels[engine], &moves, rules, ...)` (line 86,
  `crate::exchange::{Answer, ask}` imported at line 24) to GENERATE a move,
  then applies whatever the engine answered via `state.make_turn(turn)`
  (line 109). It never compares an answer to a pre-existing recorded move —
  there is no such comparison to reuse, only the shape of the loop and its
  use of `ask`.
- **Receipt — `crates/pistol-arena/src/schedule.rs:158-196`,
  `fn one_game(config, openings, identities, rules, index)`** (private, no
  `pub`/`pub(crate)`, confirmed by `grep -n "^fn one_game" schedule.rs` ->
  line 158 with no visibility modifier). Its body: build `channels` from
  `Channel::start(&sides[0].label, &sides[0].binary, &sides[0].config)`
  (lines 171-174); per channel, `handshake::shake(channel,
  config.run.hang_timeout_ms)` (line 176), `identity::verify_respawn(...)`
  (line 182), then `channel.send(pistol_cli::protocol::NEW_GAME)` (line
  183) — **unconditionally, on every fresh spawn**, per the module's own
  documented reason (`schedule.rs:12-17`): "Fresh subprocesses per game
  rather than `newgame` on a reused pair: reuse would be faster and D-7's
  gate already certifies that `newgame` clears everything, but under N
  workers the assignment of games to processes changes with N, so any
  residue would make the report depend on the worker count — the one thing
  this module must guarantee it does not (docs/decisions.md D-164)." Then
  `game::play(opening, a_is_p1, index, &mut channels, rules)` (line 191),
  then `channel.shutdown()` per channel (lines 192-194).
- **Receipt on reusability of the pieces `one_game` itself calls**:
  `Channel::start` is `pub fn` (`channel.rs:96`); `handshake::shake` is
  `pub fn` (`handshake.rs:51`); `identity::verify_respawn` is `pub fn`
  (`identity.rs:135`); `pistol_cli::protocol::NEW_GAME` is a public
  constant the existing code already imports. All FOUR of `one_game`'s own
  building blocks are already crate-visible or better — nothing here is
  blocked by `one_game`'s own lack of `pub`, because a NEW function does
  not need to call `one_game`; it needs to call what `one_game` calls, in
  the same order, which is already possible without touching `one_game` at
  all.

**No concrete blocker found.** The setup sequence `one_game` runs
(spawn, handshake, verify_respawn, `NEW_GAME`) is buildable as a second,
parallel call site using already-`pub` pieces, with zero modification to
`one_game` or `game::play`'s existing behavior for the report-generating
path — a pure additive change, not a behavior-changing one. This is what
licenses proceeding under Ruling B rather than invoking the STOP clause.

**Option A — hand-rolled Python description of the wire protocol.** This is
the option BOTH prior review rounds found broken in its grounding, not in
its underlying idea. **Receipted history, not re-argued**: round 1
(D-403) found no option matrix existed at all; round 2 (D-404) found this
document's own attempt to ground Option A asserted "no explicit `new_game`
verb is sent or needed... the same fact the arena's own per-game channel
spawn relies on" — directly contradicted by `schedule.rs:183`'s
unconditional `NEW_GAME` send, quoted above. **Kept in this matrix, marked
REJECTED**, specifically because its failure mode is now itself evidence:
a hand-written description of live process lifecycle is exactly the kind of
claim this project's own reviewers have twice caught wrong, on the same
document, at increasing levels of care. The defect class is protocol
RE-DESCRIPTION in prose a human can get subtly wrong twice in a row on the
same document; it is not fixed by writing it more carefully a third time.

**Option B — the Ruling-B replay mode: a new function pair in
`pistol-arena` that reuses the setup sequence and `exchange::ask` directly,
wired to a new CLI mode on `arena`.** Concretely, additive to the receipted
functions above:
1. A new `pub fn replay(...)` alongside `game::play` in `game.rs`, sharing
   its `Rules` type and calling `exchange::ask` — THE SAME function
   `game::play:86` already calls — at each of ONE credited engine's own
   turns (determined the same way `play`'s own loop determines a mover,
   `game.rs:82`), advancing the walked position by the RECORDED move (not
   necessarily the engine's own answer), and reporting the first turn, if
   any, where the engine's answer disagrees with the recorded move.
2. A new function alongside `one_game` in `schedule.rs` that runs the SAME
   four-step setup `one_game:171-183` runs (`Channel::start`,
   `handshake::shake`, `identity::verify_respawn`, `NEW_GAME`) — literally
   the same calls, in the same order, on the credited engine's own config —
   then calls the new `game::replay(...)` instead of `game::play(...)`.
   Protocol behavior, including whether/when `NEW_GAME` is sent, is
   INHERITED because this is the identical code path `one_game` already
   runs for generation, not a re-description of it.
3. A new mode on `crates/pistol-arena/src/bin/arena.rs` (currently 171
   lines, `--config`/`--out` only — receipt: `arena.rs:29-30`, `USAGE`'s own
   text), e.g. `arena --replay <report-path> --out <replay-report-path>`,
   mirroring the existing explicit-path, no-default style (rule 1) and
   reusing `schedule.rs`'s existing parallel-worker scheduling for the
   SAME reason the generation path does (worker-invariant results,
   `schedule.rs:1-9`'s own documented invariance argument) — replaying
   every game in the named report and writing a structured
   divergence-per-game result.
- *Cost*: ESTIMATED MODERATE. New Rust surface (a replay function, a setup
  reuse, a CLI mode) inside a crate every closed WP's own SPRT reports
  already depend on for GENERATION — but additive, not modifying, the
  generation path, per the buildability check above.
- *Failure mode*: eliminates the protocol-re-description defect class by
  construction — there is no prose about `position`/`go`/`NEW_GAME` left to
  get wrong, because the driver runs the actual functions rather than a
  description of them. Residual risk moves to ordinary Rust correctness
  (does the new `game::replay` function correctly walk the recorded moves,
  correctly detect a divergence) — checkable by the driving tests in §5,
  not by re-reading a paragraph against a source file a third time.

**Recommendation: Option B, per Ruling B's own instruction and the
buildability check above finding no blocker.** Option A is not merely
more expensive; two independent fresh-context reviews of THIS document
already demonstrated its actual failure mode in practice, on real content
this session wrote. Option B is not a speculative alternative — every piece
it needs is already `pub`, already exercised by the generation path this
project's own closed reviews already trust, and the additive change needed
to reach it was read and confirmed buildable, with receipts, above.

## 4. The selected design: the Ruling-B replay mode

1. **Mechanism.** The replay driver reuses `schedule.rs::one_game`'s own
   spawn-and-drive path (§3's Option B, concretely: a new setup-reuse
   function calling the same `Channel::start` / `handshake::shake` /
   `identity::verify_respawn` / `NEW_GAME` sequence `one_game` runs) inside
   a new pistol-arena replay mode, feeding the report's recorded moves and
   comparing the engine's move at each of its own turns via `exchange::ask`
   — the same function the generation path calls. **Protocol behavior,
   NEW_GAME included, is inherited by code identity and pinned by the
   driving tests in §5, not described in prose here** (Ruling B). **Refuses
   any budget kind other than `nodes`, by name, before replaying anything**
   — the same restriction `tools/wp15b_attribution_check.py` already
   carries, for the same reason: only a `nodes` budget is reproducible under
   instrument-mode determinism (CLAUDE.md rule 4). **On ANY divergence, that
   game's replay HALTS IMMEDIATELY** — no further recorded moves are fed to
   it, and no further comparison is drawn from it for that game (continuing
   past a divergence would feed the process a move it did not itself
   choose, desyncing its TT from what the live game actually had, making
   any later "divergence" in the same replay meaningless).
2. **On divergence: classify with BOTH engines, not one.** On a divergence
   at turn T, take ONE additional COLD query (a fresh one-shot process is
   sufficient here — this is a diagnostic probe, not part of the warm chain
   being verified) of the OTHER (non-credited) engine's config, at the SAME
   prefix. Reuses the same dual-engine-query pattern
   `tools/wp15b_attribution_check.py`'s existing link 1a already runs at its
   two checked turns, extended here to whichever turn the divergence
   actually falls on:
   - **The other engine's answer equals the recorded move** → the report's
     label assignment for this seat is the wrong one — a CONFIRMED
     INVERSION under clause (a) below.
   - **Neither engine's fresh answer equals the recorded move** → nothing
     currently known explains what was actually played. Instrument mode
     guarantees determinism (rule 4); this is that guarantee failing, not
     an attribution question — a DETERMINISM VIOLATION, its own loud,
     distinct exit code, blocking everything past it (bigger than this WP,
     reported as such, never folded into an attribution-FAIL count).
3. **Inert pairs, a theorem, not a tolerance — decisive, non-forfeit games
   only.** A pair's two games share an identical opening-book prefix with
   the seat assignment reversed (already true of every pair in this arena).
   **A pair is eligible for this exclusion only if BOTH its games end
   `normal` (non-forfeit).** A forfeit's outcome-deciding event has no
   recorded MOVE to warm-replay against — "zero divergence across the entire
   game" would be vacuously true at exactly the ply that decided the result,
   not proven there. For a pair of two `normal`-ending games: if BOTH games'
   own credited engines warm-replay with ZERO divergence across every
   recorded move, AND the two games' actual recorded move lists are
   IDENTICAL turn for turn, then the two engines are behaviorally
   indistinguishable at every position either game ever reached — confirmed
   independently by each game's own replay, not assumed. Swapping which
   label sat in which seat could not have produced a different board at any
   ply, hence not a different result: whichever PLAYER-INDEX (not label)
   wins one game necessarily wins the other too (same board, same rules),
   and since the two games swap which LABEL occupies that index, this is a
   FORCED 1-1 split — a `p2` pentanomial bucket the pair could not have
   avoided regardless of which engine is actually stronger. (A pair of two
   CAPPED games is already forced to `p2` by `score_a`'s own unconditional
   0.5-both scoring, independent of this theorem — the theorem's own work
   is for ordinary decisive games. A pair cannot mix one capped and one
   decisive game under this theorem's own precondition of identical move
   lists: result is a pure function of the move sequence under pistol-core's
   pinned rules plus the shared `turn_cap`, so identical move lists force
   identical outcome type at the identical turn — verified this session by
   inspection of `crates/pistol-arena/src/record.rs:40-45`'s `End` type, whose
   only two variants are `Normal` — covering both `Capped` and decisive
   results — and `Forfeit`.) This is provable from the replay data itself,
   not an adversarial worst-case assumption over an unknown alternative.
   Such a pair is EXCLUDED from adversarial flipping by this proof, not by
   tolerating an unresolved ambiguity.
4. **Criterion 1''.** A report is a measurement iff (a) zero
   divergence-confirmed inversions — every divergence found in point 2 above
   resolves to either "no divergence" or "confirmed inversion" (the
   other-engine match case), never left unclassified — and (b) every
   NON-INERT pair (point 3's exclusion, forfeits always non-inert) is
   directly attributed by first divergence. A DETERMINISM VIOLATION (point
   2's other branch) is checked FIRST and, if found anywhere, stops the
   whole evaluation before (a)/(b) are even asked, per its own exit code.
   The old clause (b)'s adversarial-reassignment machinery is KEPT, but only
   as a cross-check run over the INERT pairs alone (expected to be a no-op,
   since point 3's theorem already fixes their bucket) — its result is
   cited in the report as confirming evidence, not as the thing the verdict
   depends on.
5. **Cost.** Warm-replay re-runs, in full, every turn every seat actually
   searched in the original run — structurally ~1x the original run's own
   compute (a second full pass, not a handful of extra queries the way
   window-widening was), plus point 2's own single extra cold query per
   divergence (expected rare — zero on a clean report). DECLARED here; the
   first execution's actual `time_ms` is what makes it MEASURED
   (§9-equivalent slot, prereg rev 3). The OLD cold-2-turn check is not
   deleted — the tests and historical reports that cite
   `tools/wp15b_attribution_check.py` at its own pinned revision keep
   working exactly as before; this design adds a NEW, separately-named
   instrument rather than mutating the one WP-1.5b's own closed reviews
   already govern.

## 5. What changes, concretely

1. **New Rust code in `pistol-arena`** (§3 Option B): `game::replay`
   alongside `game::play`; a setup-reuse function alongside
   `schedule::one_game`; a new `arena --replay <report-path> --out
   <replay-report-path>` mode alongside the existing `--config`/`--out`
   entry point in `crates/pistol-arena/src/bin/arena.rs`.
2. **New file, successor-named** — `tools/wp16_warm_attribution_check.py`
   (the successor name the dispatch licenses; `tools/wp15b_attribution_check.py`
   is untouched, kept exactly as WP-1.5b's own reviews left it). **Consumes
   the Rust replay mode's divergence summary only** (Ruling B) — no
   protocol-speaking code of its own. Implements §4's statistics layer: the
   dual-engine cold cross-check on a reported divergence (point 2), the
   inert-pair theorem and its forfeit exclusion (point 3), Criterion 1''
   (point 4).
3. **`docs/experiments/wp16_sprt_prereg.md` §7A.1**, revision 3: Criterion
   1'' replaces Criterion 1' verbatim, the replay cost stated (DECLARED
   ~1x, MEASURED after the first real execution), every other section
   unchanged. An amendment, reopening this document's own already-passed
   review (D-400) exactly as CLAUDE.md's rule requires.
4. **Driving tests**, named into this design per the commissioning dispatch
   (`tools/SHELL_CHECKLIST.md` item 10 — a shipped behavior with no test
   driving it is the coverage hole this whole instrument class exists to
   close; these tests are what pin the facts the struck protocol-description
   paragraphs used to assert, per Ruling B):
   - **(i) Replay a real dry-run report under correct seats -> zero
     divergence.** The determinism premise itself, as a test, over REAL
     search behavior rather than a synthetic fixture — reusing
     `configs/arena_wp16_dryrun.toml`'s own matchup (`defensive_and_offensive`
     vs plain `staged`, `openings_v1.txt` take 4, NOT D-401's own governed
     run or its book/matchup, which stays unread forever per this WP's own
     rule).
   - **(ii) Synthetic seat swap -> divergence at first differing move**,
     correctly classified as a CONFIRMED INVERSION via point 2's dual-engine
     check (not merely flagged and left unclassified, closing what round 1
     found as BLOCKING 1).
   - **(iii) Inert-pair fixture -> recognized, excluded, flip cross-check
     no-op**, AND a sibling fixture identical except one game FORFEITS,
     confirmed NOT excluded (closing what round 1 found as BLOCKING 2).
   - **(iv) Forfeit-containing pair handled per the round-1 fix** —
     covered jointly with (iii)'s sibling fixture; listed separately here
     because the commissioning dispatch names it as its own item.
   - **(v) Replay mismatch under CORRECT seat assignment -> the distinct
     determinism-violation exit**, confirmed to exit loudly and distinctly
     from an attribution FAIL, AND a genuinely ambiguous case built so that,
     under the OLD single-engine-query mechanism, it would have been
     indistinguishable from an attribution failure — confirming the dual-
     engine cross-check actually discriminates it correctly (closing what
     round 1 found as its second MINOR finding).

## 6. What is explicitly NOT decided here

Whether D-401's own 44 vacuous pairs, if warm-replayed, would resolve to
"all inert" or "some genuinely misattributed" is an empirical question for
whoever next inspects that report — **moot regardless of the answer, since
this dispatch's own instruction is that D-401's run is never read under any
criterion, ever.** §1's mechanistic verification (8/8 openings, trigger (b)
inactive) is offered as WHY building this is worth it, not as a
re-litigation of that run's verdict. Whether Step 5's fresh run lands `h0`
or `h1` is this design's to make measurable, not to predict. The exact
on-disk shape of the replay mode's report format (§4's "structured
divergence-per-game result") is Step 3's own IMPL detail, not fixed here —
this design fixes what it must classify (divergence, at which turn, for
which game), not its exact serialization.

## 7. Review record

*(filled in after dispatch)*
