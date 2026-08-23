# REVIEW-design: `docs/experiments/wp15b_attribution_window_design.md`

**Pinned target.** `docs/experiments/wp15b_attribution_window_design.md`, authored at commit `6039ad9`. HEAD at review time is `6039ad98e940a2ed6f4bd0aa6076acd3af57a23b` — **identical to the pinned SHA** (this commit is HEAD; `git log 6039ad9..HEAD -- docs/experiments/wp15b_attribution_window_design.md` is empty, and `git diff 6039ad9..HEAD` on the file is empty). No amendment exists to reopen.

Fresh context. This session authored no prior revision of this document and touched none of its instrument chain before this review began, other than read-only inspection and (see below) live, reproducer-grade measurement against the actual governed-run artifact and the actual registered binaries/configs, which this review's job requires.

## VERDICT: **FAIL** — 2 BLOCKING, 2 MAJOR further findings, 1 MINOR

The design's central technical premise — that walking the replay past each engine's first free search is "doing more of the same valid thing, not a different, shakier thing" (§3) — is **empirically false**, demonstrated on the actual governed-run data using the exact registered binary, configs, and budget. Separately, the document was never built as the OPTION MATRIX CLAUDE.md's process requires for a decision with more than one viable option, and it omits an option (tolerate a bounded rate) that the diagnostic document it is built on explicitly raised. Both are BLOCKING. IMPL must not proceed until both are resolved and the document is re-reviewed at its next revision.

---

## BLOCKING 1 — the walk's core premise is false beyond each engine's first free search, and this is not hypothetical: it already breaks on one of the 5 openings this design exists to fix

**§3's claim, quoted:** *"Link 1a's referent has always been 'a fresh process handed the actual recorded prefix reproduces the recorded move exactly' (§8.3, prereg). That premise does not weaken as the prefix grows longer — it is still the real, already-decided history, not a hypothetical continuation. ... Walking further is doing more of the same valid thing, not a different, shakier thing."*

**This is wrong, and the diagnostics document the design is built on already named why it might be wrong**, in language the design doesn't engage with: `wp15b_vacuity_diagnostics.md` Result 5 states turn 4 and turn 5 are replayable "via a simple fresh-process query ... because each is that engine's FIRST search of the game," and explicitly flags: *"Widening to a 3rd turn checks each engine's SECOND search, which is a different claim needing its own justification."* The design's §3 answers this by assertion ("does not weaken") rather than by testing it.

**Root cause, read from source, three independent confirmations:**

1. `crates/pistol-engine/src/instance.rs:90-101` — `Engine::new_game` clears the searcher (`self.searcher.clear()`); `Engine::set_position` does **not** touch `self.searcher` at all, only replays `self.state`. The transposition table is never cleared by a `position`/`go` cycle, only by an explicit `new_game`.
2. `crates/pistol-arena/src/game.rs:1` (module doc) — *"one game, two subprocesses"* — confirmed by `crates/pistol-arena/src/schedule.rs:172-173`, which calls `Channel::start` (spawning a fresh `Command::new`, `channel.rs:96-103`) immediately before `game::play` **per game**. `crates/pistol-arena/src/exchange.rs:60-62` sends exactly `[position, go]` per turn, with no `new_game` in the per-turn path. So: one live process serves **every turn of one engine's one game**, and its TT warms continuously across all of that engine's own searches in that game.
3. `tools/wp15b_attribution_check.py`'s replay (both the current 2-turn check and the proposed walk) spawns a **brand-new subprocess per single query** (`subprocess.run([engine, "--config", config], ...)`, line 167) — always a cold TT.

Turn `opening_turns+1` and `opening_turns+2` (today's fixed window) are, respectively, each engine's absolute *first* free search of the game — cold TT live, cold TT replayed, hence they agree. Any *later* turn is a live search with a TT warmed by that same engine's own earlier searches *in that game*, which a fresh replay subprocess cannot reproduce. This is exactly the gap Result 5 named and the design dismissed.

**Empirical reproduction, on the actual governed-run artifact (`artifacts/wp15b_governed_run.txt`), using the exact registered binary (`target/release/pistol`, sha256 `9acd23554…`, matching the report's own `binary_sha256`) and the exact registered configs/budget (`configs/instrument_staged_v0.toml` / `configs/instrument_v0.toml`, `go nodes 50000`):**

The reviewer's replay harness first reproduces `wp15b_vacuity_diagnostics.md` Result 1's published turn-4/turn-5 numbers for opening 44 (games 88/89) exactly (`1,3/2,3` and `-3,3/3,3`, both configs, matching the doc verbatim) — establishing the harness is sound and matches the registered process. Walking the *same opening* forward turn by turn:

```
turn 6..10 (free=5..9): staged and r2 agree with each other and with the recorded move — clean.
turn 11 (free=10): staged answers -1,4/4,-1   r2 answers -2,5/4,-1   (DISCRIMINATES)
```

Game 88's and game 89's actual `moves` lines are byte-identical through index 11 (confirmed by direct diff of the report's `moves 88 ...` / `moves 89 ...` lines — true divergence doesn't occur until index 12, turn 13). At turn 11, game 89's credited mover is `r2` (game 89's `p1`), and its **actual recorded move** is `-1,4/4,-1` — identical to game 88's, because the pair hadn't really diverged yet. But `r2`, replayed fresh on that exact prefix (verified deterministic, repeated 3×, same answer every time), answers `-2,5/4,-1` — **not** the move it actually played live.

**If the walk design is implemented as written, it will report this as a link-1a attribution failure** — *"1a game 89 turn 11: the report attributes `-1,4/4,-1` to `r2`, and `r2` (configs/instrument_v0.toml) answers `-2,5/4,-1`"* — indistinguishable in its printed form from a genuine seat/label swap. Per the prereg's §5, a non-zero Criterion-1 exit means *"the run is not a measurement ... the next step is finding the defect the link names"* — sending an operator hunting for a labeling bug that does not exist, on exactly the opening this design was written to clear. This also means the walk discriminates (turn 11) *earlier* than the design's own move-list-derived "first divergence" (turn 13) for this opening — the two methods (replay vs. move-list comparison) are not equivalent past each engine's first search, contrary to what §2 assumes when it says the divergence table needed "no engine replay."

**What this requires before IMPL:** either (a) the replay must reproduce the *live* TT state — i.e., replay the full sequence of that engine's own prior `go` calls within the same subprocess, not a single isolated query per checked turn — a materially different and more expensive implementation than §4 describes, or (b) some other argument establishing why later-search replay is trustworthy that survives contact with this counterexample. Neither exists in this document.

---

## BLOCKING 2 — no option matrix, and a third live option (Result 5's "tolerate") is never mentioned

CLAUDE.md: *"A named design decision with more than one viable option is settled by an OPTION MATRIX — options, costs, failure modes, recommendation — attacked by a fresh-context DECISION-RED-TEAM subagent BEFORE selection ... An option adopted without a matrix, or a matrix never attacked, is the same breach as silent architecture drift."*

This document is a narrative "one alternative tried and rejected, one selected" (§2 vs. §3). But `wp15b_vacuity_diagnostics.md` Result 5 — the document this design is explicitly built on — already put a **third** option on the table: *"registering a tolerated vacuity rate while leaving 1b/1c universal — changes no code and reopens nothing, since Result 4 shows 1b and 1c already hold on all 116 games including the 10 in question; it would need only a §5-adjacent sentence."* That option is **cheaper** (no code change, no re-review of the prereg, no new tests) and **safer** (it doesn't touch the instrument chain a governed run depends on). The design never names it, never explains why it's inferior to walking, and is not formatted as a matrix with costs/failure-modes side by side, let alone red-teamed.

This is not merely a style gap: given BLOCKING 1, the "tolerate" option would in fact have been the correct call, since walking demonstrably introduces a *new* failure mode (spurious link-1a failures from TT-state mismatch) that tolerating never would have. A matrix, attacked, is exactly the process step that would plausibly have caught this before a review cycle was spent on it.

---

## MAJOR — §3's "cost unchanged for the 106" claim is measurably wrong (though benignly: cost drops, not rises)

§3: *"For the 106 of 116 games that already discriminate within the current 2-turn window, behavior and cost are unchanged (the walk exits on the same first turn it always did, with an early exit — no re-check of turns already passed)."*

This isn't true for the majority of that population. The **current** code (`tools/wp15b_attribution_check.py:157`, `for free in (opening_turns, opening_turns + 1):`) has no break — it always queries **both** checked turns (4 subprocess calls: 2 turns × 2 engines) regardless of whether the first one already discriminates. The **new** walk explicitly breaks "the first time a game's `here` count reaches 1" (§4 item 1) — i.e., as soon as the first checked turn discriminates, it stops, at half the query count.

Measured directly from the governed-run artifact (`opening_turns=3`, so the first checked turn is index 3 / turn 4): **33 of 58 openings (66 of 116 games) already discriminate at that very first checked turn** — reproducing the diagnostics doc's own Result 2 figure exactly ("Turn 4 agreement: 25 of 58 openings," so disagreement is 33 of 58). For every one of those 66 games, the new design would issue 2 replay searches instead of 4 — a real, unacknowledged **behavior and cost change**, not an "unchanged" case. Only the remaining 40 games (the 20 openings that agree at turn 4 but discriminate at turn 5) genuinely see unchanged cost.

This doesn't threaten the "a few tens of seconds" bound (the true effect is cheaper than claimed, not more expensive), and §4 item 1 already anticipates that the `checked`/`discriminating` note wording needs updating for run-to-run variability — but the specific factual claim in §3 should be corrected to state a decrease for the 66-game subset, not "unchanged," given this project's explicit discipline about measured claims.

## MAJOR — no test plan for a walked-to `Turn::Single` move

`pistol_core::Turn` (`crates/pistol-core/src/turn.rs:117-121`) has two variants, `Single(Coord)` and `Pair(Coord, Coord)`, with different `Display` output (`turn.rs:196-202`: `Single` prints one coordinate, `Pair` prints `a/b`) — the encoding of rule 4's "the turn's second stone is then not played." `movegen.rs` confirms the search can legitimately choose a `Turn::Single` whenever it completes a line.

The current 2-turn window (turns `opening_turns+1`/`+2`, i.e. turns 4-5 in this run) never reaches deep enough into a game to encounter this format (empirically, 0 of 116 games in the actual governed run end on a `Turn::Single` — every game's win completes on the *second* of its final turn's two stones in this dataset, which is a property of this run's data, not a guarantee). The walk is the first design to make the replay loop routinely reach turns near or at a game's end, where a `Turn::Single` genuinely can appear (any future run's data, or even the tail of a "walk to exhaustion" fully-vacuous game). Neither existing fixtures (`crates/pistol-cli/tests/wp15b_attribution_check_tests.rs`, whose shim/`honest_report` builder always emits `Turn::Pair` strings for every non-opening turn) nor the two new test cases §4 item 3 proposes ("agree then diverge," "never diverge") exercise a walked-to single-stone turn. Given this project's explicit coverage rule (`tools/SHELL_CHECKLIST.md` item 10: *"any tools/ script that produces a recorded number carries at least one test driving the shipped script"*) and its stated history of exactly this class of parsing gap, a third fixture covering this case is needed before landing.

## MAJOR — missing ADR line (CLAUDE.md rule 10)

§4 ("What changes, concretely") lists exactly three files. It never proposes a `docs/decisions.md` entry, despite this being precisely the kind of *"non-obvious design choice"* CLAUDE.md rule 10 requires one for, and despite this WP's own dense precedent of filing ADR lines for comparably-sized decisions in this exact instrument chain (D-307, D-308, D-329, etc., all cited throughout the prereg and diagnostics docs this design descends from). A design that reopens a five-times-reviewed prereg and rewrites a load-bearing instrument's core loop, without registering the decision on the ADR log, is a process gap that should be closed before IMPL, not after.

## MINOR — §2's stated range doesn't match its own table

§2: *"The first-divergence point varies from 2 to 10 turns past the current window, per opening."* The table's own "turns past current window" column reads `+3, +2, +3, +6, +9` — range **2 to 9**, not 2 to 10. (Independently recomputed all five first-divergence turns directly from `artifacts/wp15b_governed_run.txt`'s `moves` lines and confirmed the table's turn numbers — 7, 6, 7, 10, 13 — are exactly right; only the prose's summary range is off by one.) Doesn't change the conclusion (turn 13 is correctly the deepest reach needed), but is exactly the class of arithmetic slip this project's discipline exists to catch.

---

## Answering the six scrutiny points directly

1. **Divergence-point table**: verified correct against the actual governed-run artifact, turn-for-turn (7, 6, 7, 10, 13) — see MINOR above for the one prose/table mismatch. The underlying reasoning ("shared prefix ⇒ first index of actual-move divergence is the first point of underlying disagreement") is sound as far as it goes, but is a claim about **move-list divergence**, not about **replay fidelity** — and those two came apart empirically at turn 11 of opening 44 (BLOCKING 1).
2. **"Walk until discrimination or exhaustion" soundness**: unsound as designed, for the reason in BLOCKING 1. Capped games are fine (a cap never completes a line — `game.rs`'s `if state.turn() > rules.turn_cap { return Capped }` fires before any win, so a capped game's final `moves` entry is an ordinary `Turn::Pair`). Forfeited games: link 1b already excludes them (`result == "capped" or end != "normal"` skip), and nothing in the proposed change appears to special-case them in link 1a either, but no fixture exercises `end forfeit` through the walk — flag for RED-TEAM, not a confirmed defect. The "cost unchanged for the 106" claim is quantifiably wrong (MAJOR above), though the true direction is cheaper, not more expensive.
3. **File scope**: the three files named are the mechanically correct ones, but the scope omits an ADR entry (MAJOR above) and — more importantly — omits any file/plan addressing the TT-replay-fidelity defect BLOCKING 1 exposes, which is squarely a game-legality/data-parsing-adjacent path (`pistol-engine`'s `set_position`/`new_game` contract) that this design's premise leans on without checking.
4. **Subtler alternative**: yes — Result 5's untouched "tolerate" option (BLOCKING 2), and, once BLOCKING 1 is known, a "walk that replays the full turn sequence in one persistent subprocess per checked game" (to match live TT state) as a fourth option, neither considered.
5. **Section 5's punt**: this was exactly the wrong thing to leave to IMPL time, and not just on principle — the reviewer ran the check now, in minutes, using data already sitting in the repo (`artifacts/wp15b_governed_run.txt`) and the already-built binary, and it fails on one of the five openings. This is the textbook case CLAUDE.md names: *"an estimate that could have been measured in seconds is a finding"* (and here it wasn't even estimated — it was asserted as not needing measurement, which is worse).
6. **Other gaps**: the two MAJOR findings on cost-claim accuracy and `Turn::Single` coverage, plus the ADR omission and the MINOR arithmetic slip, above.

---

## Reproducer summary (for the record, per this project's review-header convention)

- Divergence table verification: `python3` script reading `artifacts/wp15b_governed_run.txt`'s `game`/`moves` lines, index-comparing the 5 vacuous pairs — reproduces (7, 6, 7, 10, 13) exactly.
- Turn-4/5 harness sanity check against `wp15b_vacuity_diagnostics.md` Result 1: exact match (`1,3/2,3`, `-3,3/3,3`).
- Turn-4 agreement count: 25/58 openings agree, 33/58 disagree — matches diagnostics Result 2 exactly, and directly falsifies the "106 games unchanged" claim for the 33-opening/66-game subset.
- BLOCKING 1 reproducer: walking opening 44 (games 88/89) from turn 6 via `target/release/pistol --config configs/instrument_staged_v0.toml|instrument_v0.toml`, `go nodes 50000`, against prefix `moves("88")[:10]` — `r2` answers `-2,5/4,-1`, three repeated runs, all identical; the actual recorded move at that exact (shared) prefix, played live by `r2` as game 89's `p1`, is `-1,4/4,-1`. Root cause traced to `crates/pistol-engine/src/instance.rs:90-101` (`set_position` never clears `self.searcher`) and `crates/pistol-arena/src/schedule.rs:172-173`/`game.rs:1` (one persistent subprocess per engine per game).

## Files relevant to this review

- `/home/tom/Projects/HeXO-AlphaBeta/docs/experiments/wp15b_attribution_window_design.md` (target, rev `6039ad9` = HEAD)
- `/home/tom/Projects/HeXO-AlphaBeta/docs/experiments/wp15b_vacuity_diagnostics.md`
- `/home/tom/Projects/HeXO-AlphaBeta/docs/experiments/wp15b_sprt_prereg.md` (§7A.1, §8.3)
- `/home/tom/Projects/HeXO-AlphaBeta/tools/wp15b_attribution_check.py`
- `/home/tom/Projects/HeXO-AlphaBeta/crates/pistol-cli/tests/wp15b_attribution_check_tests.rs`
- `/home/tom/Projects/HeXO-AlphaBeta/crates/pistol-engine/src/instance.rs` (lines 90-101, the TT-clearing gap)
- `/home/tom/Projects/HeXO-AlphaBeta/crates/pistol-arena/src/game.rs`, `schedule.rs` (lines 172-173, 191), `exchange.rs` (lines 60-62), `channel.rs` (lines 96-103) — the one-process-per-game structure
- `/home/tom/Projects/HeXO-AlphaBeta/crates/pistol-core/src/turn.rs` (lines 117-202, `Turn::Single`/`Turn::Pair`)
- `/home/tom/Projects/HeXO-AlphaBeta/artifacts/wp15b_governed_run.txt` (the governed-run artifact used for all live verification above)
