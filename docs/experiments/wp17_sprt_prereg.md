# WP-1.7 — SPRT pre-registration: the three ordering heuristics (killers, history, countermove, all ON) vs the committed staged policy

**Revision 1. UNREVIEWED. It governs no run until a fresh-context review
passes it.**

**Provenance, so a reviewer does not have to reconstruct it from the ADR
log.** The design (`docs/experiments/wp17_design.md`, revision 3 at
`c227162`) passed a fresh-context REVIEW-design and a fresh-context
DECISION-RED-TEAM on its option matrix (D-429). The implementation landed at
`f745d90` with its rule-5 bench WITHIN BRACKET (D-431: nps ratio 1.059/1.064,
IQR gate clean, all four pre-registered mutations dead), passed a
fresh-context REVIEW-impl (1 MAJOR, 1 MINOR — the M8 boundary pin and a
dangling citation, both closed at `fc4bc69`, with the boundary mutant now
dying), and the full three-seat determinism gate is green. **Delegation**: the
commissioning dispatch states "Delegation for the governed run granted
in-dispatch (D-382 pattern)" — launch authority for Steps 5-7 of this WP
(slot pass, governed run, closure) is this session's, by that sentence.

**This document deliberately reuses WP-1.6's prereg structure
(`docs/experiments/wp16_sprt_prereg.md`, revision 10) and its instruments
UNCHANGED** — the arena, the warm-replay pass, `tools/wp16_warm_attribution_check.py`
and `tools/wp15b_attribution_check.py` are WP-1.6's, byte-for-byte (checkable
below), which is why this WP's dispatch does not dispatch a RED-TEAM: the
instruments and data paths are unchanged from WP-1.6's governed run
(`a4d5fbb`).

---

## 1. What is being judged, and what is not

**The claim.** The staged policy with all three WP-1.7 ordering-heuristic
gates ON (`configs/instrument_staged_h_v0.toml`: `killers = true`,
`history = true`, `countermove = true`) is stronger than the committed plain
staged policy (`configs/instrument_staged_v0.toml`, all three `false`) at a
fixed node budget.

**One axis separates the seats, not several.** **CONFIRMED by diff, this
session**: the two documents differ in exactly THREE value lines — the gates.
Every other field (`quiet_radius`, `quiet_top_k`, `widen_schedule`,
`tier_t_own_count`, `tier_t_opponent_count`, `q_depth_turns = 0`,
`q_triggers = "defensive_only"`, `tt_bytes`, eval, instrument, play) is
identical. A verdict here is a verdict on the three gates together — never on
any one alone, and never on quiescence (gated off in both seats, D-428).

**What this run cannot judge.** SPRT judges STRENGTH, not soundness.
Soundness is the design's own gate (five validation unit tests, the M8
boundary pin, the three-seat determinism gate, the mutation table at D-431,
REVIEW-impl at `6dcd27e` closed at `fc4bc69`). A green SPRT over an unsound
mechanism is a measurement of nothing.

## 2. The hypothesis and the verdict unit

- **H0**: `elo0 = 0.0` — the heuristics-ON seat is no stronger.
- **H1**: `elo1 = 15.0` — **FIXED by this WP's dispatch**: "elo0/elo1,
  budget, book stay as registered in WP-1.6; not re-read." The pair-floor
  derivation at this `elo1` is WP-1.6 §2's own (computed there against
  `sprt.rs`'s LLR formula) and is not restated here.
- `alpha = 0.05`, `beta = 0.05` — the values every prior SPRT run used.
- **Verdict unit: the PAIR** — both games of an opening, colours reversed,
  pentanomial pair outcome, `sprt.rs`'s own unit.
- **The pair floor: 100 pairs**, the standing D-190 convention every SPRT
  document in this project uses.

## 3. The instrument

| | |
|---|---|
| arena | `target/release/arena`, built `--release --locked` at the run's revision |
| engine A | `configs/instrument_staged_h_v0.toml` — the heuristics-ON seat, label `staged_h`. **In slot A because the arena's statistic is slot A's score and §1's H1 is "the heuristics seat is stronger"** — the one mistake a pre-registration exists to prevent is reading that sign backwards |
| engine B | `configs/instrument_staged_v0.toml` — the committed plain staged policy, unchanged, label `staged` |
| binaries | both seats run `target/release/pistol`, bound by `binary_sha256` (D-283 as qualified by D-294); recorded at §7A.1 and re-confirmed at launch (§9.2) |
| book | `crates/pistol-cli/tests/fixtures/random_openings_v1.txt` — the PRIMARY SPRT book, 2000 openings, sha-pinned |
| `openings_take` | 500 — clears §2's 100-pair floor with wide margin |
| `openings_skip` | **1000 — THE FRESH SLICE, FIXED HERE AND NOT AT LAUNCH** (D-427's lesson, learned the hard way). D-402 retired `0..499` as evidence for anything; WP-1.6's governed run consumed `500..999`; this draws `1000..1499`, disjoint from both by construction. The skip is inside `experiment_sha256`, so this is formally a different experiment from every prior one |
| budget | `kind = "nodes"`, `value = 50000` — the registered snapshot budget every SPRT document in this project uses |
| `turn_cap` | 40, as every prior run |
| `n_workers` | 4 — WP-1.3's red-team clearance, unchanged |
| `hang_timeout_ms` | 120000 — liveness only, never an adjudication (D-159). Calibrated NO-CHANGE: D-431's bench measured the heuristics seat's worst single search at **430 ms** over 24 positions × 5 reps at this exact budget — a **~279x** margin |

**Engine identity closes over the config and the weights** (D-198, D-199,
D-283 as qualified by D-294): the arena re-digests each seat's document
before every spawn and refuses a mismatch by name.

**The document that IS the experiment** is
`configs/arena_wp17_heuristics_vs_staged.toml`, authored and committed at
Step 6's own launch, once §9.2/§9.6 are known — not authored here, since it
needs the launch-time binary digest.

## 4. What the run reports, and which lines are read

The same arena, the same `conclusion.rs`/`report.rs`, unchanged by this WP.
The field list is WP-1.6 §4's own and is not restated here — **nothing
differs**; below `# timing` is machine-dependent and excluded from every
comparison. **Per-side compute is a reporting requirement** (rule 6), read
from the two `timing_engine` lines and reported beside the verdict.

## 5. Outcome handling, written before game one

The exit-code taxonomy of the Criterion 1'' chain — what exit 0/1/2/3 of
`tools/wp16_warm_attribution_check.py` may be concluded to mean — is
**WP-1.6 §5's table, pointed at and not restated** (state-it-once; the
partition there was deleted by D-424 after failing three reviews and must not
be reborn here). What follows are the rows THIS WP adds on top of it:

| Outcome | Action |
|---|---|
| `verdict h1` **at or above 100 pairs** | The three gates are accepted as stronger AT THIS BUDGET AND BOOK. The committed instrument configs — `configs/instrument_staged_v0.toml` and `configs/instrument_v0.toml`, identical in policy — flip `killers`/`history`/`countermove` to `true` in a separate commit after the run, subject to the closure gate. The gate and play configs do not move: they are not strength seats, and the determinism gate already exercises the ON path through its own third seat (`configs/gate_staged_heuristics_v0.toml`) |
| `verdict h1` **below 100 pairs** | WITHHELD. A confirmatory run on the DISJOINT book decides it — the same document with `openings_file` changed to `crates/pistol-cli/tests/fixtures/openings_v1.txt` (1591 openings) and `openings_take` re-stated, D-190's own instrument, exactly as D-385/D-386 executed for WP-1.5b |
| `verdict inconclusive_degenerate` | Read the direction off the pentanomial exactly as WP-1.6 §5's own degenerate row prescribes (its reading is generic in the occupied slot and is not restated here) |
| `verdict h0` | The gates are NOT accepted. **A planning finding, never a threshold move**: the committed configs keep all three gates `false`, no re-read of `elo1`, no budget change, no book change, no run-until-favourable. The licensed-not-scheduled relaxations (top-K history promotions, depth-scaled bonuses, play-order pair keying — the design §4's recorded strongest surviving attacks) are future pre-registrations, not this WP's business |
| `invalid_forfeit`, or `forfeits > 0` at any verdict | **The run is not a measurement.** Investigate the forfeit's cause (D-158) |
| Criterion 1'' fails / instruments disagree / any exit not 0 | **WP-1.6 §5's rows govern, pointed at and not restated** |

**The turn cap and the book are part of the claim.** A verdict is about
`random_openings_v1` at 50 000 nodes with a 40-turn cap, `openings_skip 1000`.

## 6. The honest expectation, and what a negative result means

**ESTIMATED, no hex measurement exists for the pairing**: chess-lineage gains
for these heuristics were measured on top of TT ordering WITHOUT threat-first
generation; here stages F+T already order tactically (WP-1.5b, 92.2%
SPRT-proven), so redundancy is likely and the expected range is **null to
small positive at equal nodes**. The design's own bench (D-431) measured a
~6% nps gain and one position gaining a completed depth — an ordering-quality
signal, not a strength claim. **`h0` is a legitimate outcome and closes the
WP as a measured finding**: three heuristics whose entire prior is
chess-lineage analogy do not clear +15 normalised Elo over a threat-first
ordering that already captures most of their value. Recording that in advance
is what stops it being said afterwards as an excuse.

## 7. Costs, stated on the document's own face

| Item | DECLARED | MEASURED |
|---|---|---|
| The dry run (§8) | minutes | **17.4 s** run (`timing n_workers 4 wall_ms 17367`) + **17.4 s** replay (`wall_ms 17379`) for 8 games at 4 workers, plus both checkers |
| The governed run | **ESTIMATED 8-15 min wall at 4 workers**: WP-1.6's 450-game run cost 707 s wall with a seat carrying a 2.48x node tax (D-398); this matchup's two seats both cost plain-staged prices (D-431 measured the ON seat ~6% FASTER, not slower) | the run itself, Step 6 |
| The warm replay | ~1x the run (WP-1.6 §7's four samples straddling 1.0; this WP's dry run: `17379/17367 = 1.00x`) | the pass itself |
| `tools/wp16_warm_attribution_check.py` | seconds | WP-1.6 §7: `0.029 s` per 8 games |
| The second instrument | minutes at 500 openings | WP-1.6 §7: `6.485 s` per 8 games |
| Operator attention | one launch, one slot pass, one read of the chain | — |

## 7A. The doubts and their instruments

### 7A.1 DOUBT 1 — the arena between the engines and the verdict

**THE STAGE UNDER DOUBT**: everything between the two engine processes and
the printed verdict — the arena's seat bookkeeping, pairing, referee and
scoring. Identical stage to WP-1.6's Doubt 1: this WP changes which engine
sits in which seat, not the arena code that scores them.

**THE INSTRUMENT, AND ITS GOVERNING REVISION — every artefact named WITH the
revision that governs this run** (CLAUDE.md's instrument rule; a change to
any of them reopens this document's review exactly as an amendment would):

1. **The warm-replay pass** — `crates/pistol-arena/src/seats.rs`,
   `transcript.rs`, `replay.rs`, `replay_report.rs` and `bin/arena.rs`'s
   `--replay` mode, at commit `bfdf933`. **Unchanged by this WP and by
   everything since**: `git diff --stat bfdf933..HEAD -- crates/pistol-arena/`
   prints nothing.
2. **The statistics layer** — `tools/wp16_warm_attribution_check.py`, at
   commit **`6c929da`** — the last commit that touched it, through WP-1.6's
   revision-10 fix round. `git log --oneline -1 6c929da --
   tools/wp16_warm_attribution_check.py` names it.
3. **The binaries those two actually run** — `target/release/pistol`
   `sha256 665d2815ddba28e7889ebea661a10b15352036ab46bfc6f1758d72813cad4184`
   and `target/release/arena`
   `sha256 3e5c114fee9b1d8018b733075b2eaaeb7625ea2d14d387123f53c727173e5851`,
   built `--release --locked` at `fc4bc69`. Rebuild means re-record, and
   re-recording is an amendment; §9.2 re-confirms at launch.
4. **The second instrument** — `tools/wp15b_attribution_check.py`, at commit
   `a80a864`, the last commit that touched it, unchanged through every
   revision since (WP-1.6 §10 records the same pin).

**CRITERION 1'', quoted verbatim from `docs/experiments/wp16_warm_replay_design.md`
§4 point 4 at its revision 3 (`b6afd66`, the current text), in full and
without ellipsis:**

> **Criterion 1''.** A report is a measurement iff (a) zero
> divergence-confirmed inversions — every divergence found in point 2 above
> resolves to either "no divergence" or "confirmed inversion" (the
> other-engine match case), never left unclassified — and (b) every
> NON-INERT pair (point 3's exclusion, forfeits always non-inert) is
> directly attributed by first divergence. A DETERMINISM VIOLATION (point
> 2's other branch) is checked FIRST and, if found anywhere, stops the
> whole evaluation before (a)/(b) are even asked, per its own exit code.
> The old clause (b)'s adversarial-reassignment machinery is KEPT, but only
> as a cross-check run over the INERT pairs alone (expected to be a no-op,
> since point 3's theorem already fixes their bucket) — its result is
> cited in the report as confirming evidence, not as the thing the verdict
> depends on.

**The instrument's own exit codes are its constants**: `ATTRIBUTABLE = 0`,
`NOT_A_MEASUREMENT = 1`, `NO_ANSWER = 2`, `DETERMINISM_VIOLATION = 3`. What
each may be concluded to mean is **WP-1.6 §5's table, pointed at and not
restated here** — that partition was deleted once already (D-424) and
reborn copies are how it failed three reviews (D-423).

**THE SECOND INSTRUMENT'S AGREEMENT CRITERION — as registered in WP-1.6
§7A.1, checked as written, quoted:**

> **THE AGREEMENT CRITERION**: for every game the cold checker attributes by
> a discriminating replayed turn, the warm pass must record that game `status
> clean` — and for every game it calls a confirmed inversion, the warm pass
> must record a `divergence`.

**The registered consequence of disagreement** is WP-1.6's own: the run is
not a measurement, the verdict is not read, and the disagreement is
investigated as an INSTRUMENT defect before anything is concluded about
either engine. **The stage the second instrument does not share** is
WP-1.6 §7A.1's own naming (the WARM DRIVE); **what both are blind to** is the
report WRITER. Both rows are pointed at rather than restated.

## 8. The dry run — recorded, with its criteria

CLAUDE.md: a pre-registration's literal commands are exercised before its
review passes, on an input of the SAME KIND as the registered workload — the
same sort of artefact, differing only in identity — and never on the
registered workload itself. **DONE, this session.**

**The input.** `configs/arena_wp17_dryrun.toml` — the registered matchup
(`staged_h` vs `staged`; WP-1.7 has no wider arm to reach for, and the dry
run's job is the commands, not a strength sample) over
`crates/pistol-cli/tests/fixtures/openings_v1.txt` (NOT the primary book),
`openings_take = 4`, the governed run's budget/cap/workers/timeout. The
registered workload is 500 openings of `random_openings_v1` at skip 1000;
four openings of a different book are a different sample of the same kind,
and nothing is consumed.

**The literal commands.** `<scratch>` is outside the repository; `--out`
paths did not exist:

```
cargo build --release --locked --bin arena --bin pistol
sha256sum target/release/pistol target/release/arena
tools/config_check.sh configs/arena_wp17_dryrun.toml
target/release/arena --config configs/arena_wp17_dryrun.toml --out <scratch>/run.txt
target/release/arena --replay <scratch>/run.txt --out <scratch>/replay.txt --workers 4
python3 tools/wp16_warm_attribution_check.py <scratch>/run.txt <scratch>/replay.txt target/release/pistol
```

and the seeded-defect arm — the same report with every `game` record's
`p1`/`p2` labels transposed and NOTHING else changed:

```
target/release/arena --replay <scratch>/swapped.txt --out <scratch>/swapped_replay.txt --workers 4
python3 tools/wp16_warm_attribution_check.py <scratch>/swapped.txt <scratch>/swapped_replay.txt target/release/pistol
```

and the second instrument on the honest report:

```
python3 tools/wp15b_attribution_check.py <scratch>/run.txt target/release/pistol
```

### 8.1 What the output must show, and the defect class each criterion excludes

**Criterion W-1 — the honest arm.** The replay reports `8 of 8 game(s)` and
`0 divergence(s)`; every game's `nodes_a`/`nodes_b` in the replay document
EQUALS the report's; the checker exits **0**. *Defect class excluded: a
replay that is not actually warm* — re-driving the engines cold, feeding a
desynchronised sequence, silently skipping searches. Node equality is the
externally derived referent that class cannot preserve (`0 divergence(s)`
alone would not be a criterion: a replay that never asked anything reports
zero too).

**Criterion W-2′ — the seeded-defect arm.** The label-transposed copy must
NOT pass: `arena --replay` exits **1** with a divergence in every game, and
the checker exits **NON-ZERO with a named finding**. *Defect class excluded:
an attribution chain that cannot see a seat swap.* **REGISTERED AS OBSERVED —
the exit is 3, not WP-1.6's exit-1-confirmed-inversion shape, and the reason
is load-bearing**: this matchup's two seats run the same engine with the same
budget and agree on the opening turns, so the first divergence lands late
(game 5, turn 14), where the other seat's COLD probe no longer reproduces
what the WARM seat recorded — the checker's dual probe cannot confirm an
inversion and names the only thing it can, `DETERMINISM VIOLATION`, with its
own two-possible-causes message. The corrupted report is still refused and
nothing downstream of it is read; on the GOVERNED run, exit 3's consequence
is WP-1.6 §5's row (hard stop, bigger than the WP, investigate), pointed at
and not restated.

**Criterion W-3 — the second instrument.** `tools/wp15b_attribution_check.py`
exits **0** on the honest report, and the §7A.1 agreement criterion HOLDS on
both clauses.

### 8.2 What the dry run recorded

Every artefact is gitignored (rule 8) and named by content:

| | |
|---|---|
| run | `artifacts/wp17_dryrun_run.txt`, sha256 `3818f464743f967d16438b55afac1592fb3b2093c3e27ee988cfd5ebafe18c64`, `timing n_workers 4 wall_ms 17367`, `verdict inconclusive_at_game_cap` (4 openings cannot cross a boundary; not read as anything) |
| warm replay | `artifacts/wp17_dryrun_replay.txt`, sha256 `956500624859d35df1ea31de4161446516ab6dca6131891a006edb7a1dc04141`, `replayed 8 of 8 game(s) … 0 divergence(s)`, `wall_ms 17379` |
| **W-1** | **MET.** `W coverage: 8 game(s) accounted for — 8 replayed in full with every node count equal to the run's, 0 halted at a divergence`; `W classification: 0 divergence(s), 0 confirmed inversion(s), 0 unexplained`; `(b): 0 inert pair(s) excluded by theorem, 4 pair(s) directly attributed at their first differing searched turn, 0 unattributable`; `1b: 5 decided non-forfeit game(s)`; `1c: 8 game(s) and 4 pair(s) rebuilt off the score_a path`; `PASS — 0 failure(s)`, exit 0 |
| seeded swap | `artifacts/wp17_dryrun_swapped.txt` (`ffeab3da…`) and its replay `artifacts/wp17_dryrun_swapped_replay.txt` (`f7c9a56e…`) |
| **W-2′** | **MET.** `arena: replayed 8 of 8 game(s) … 8 divergence(s)`, replay exit 1; checker exit **3** with `DETERMINISM VIOLATION: game 5 turn 14 …` and the two-possible-causes message, exactly as §8.1 registers it |
| **W-3** | **MET.** `1a: 16 turns replayed, 2 of them discriminating, 2 of 8 games directly attributed by replay`; `1a robustness: 3 vacuous pair(s) … verdict inconclusive_at_game_cap unchanged`; `PASS — 0 failure(s)`, exit 0. The agreement criterion HOLDS: the 2 games it attributes by discriminating turns are `status clean` in the warm pass (all 8 are), and its 0 confirmed inversions match the warm pass's 0 divergences |

**The dry run is not a governed sample and consumes this document's first
run.** It found one real thing worth recording: the two seats of THIS
matchup agree often enough that only 2 of the cold checker's 16 replayed
turns discriminate — consistent with §6's registered redundancy expectation,
and irrelevant to the chain's validity (the WARM instrument attributed all 4
pairs at their first differing searched turn, 0 unattributable).

## 9. FILL-IN slots

**9.1 `elo1`.** FIXED at `15.0` (§2), by this WP's dispatch. Not re-opened.

**9.2 `binary_sha256`.** Recorded from `sha256sum target/release/pistol`
after `cargo build --release --locked --bin pistol` at Step 6's own launch
revision, for BOTH seats; §7A.1's pins re-confirmed or re-recorded then.
Rebuild means re-record.

**9.2a `openings_skip`.** FIXED at `1000` (§3) — decided HERE, in the
document, before the run, per D-427's lesson: which games are played is part
of the experiment and choosing a window after seeing anything is the
after-the-numbers move this document exists to forbid.

**9.3 `openings_take`.** FIXED at `500` (§3).

**9.4 The design and implementation gates.** GREEN as of `fc4bc69`:
REVIEW-impl closed (its one MAJOR — the M8 boundary pin — closed with the
mutant dying), the mutation table dead, the bench within bracket (D-431),
`tools/ci.sh` to be run once more at the launch revision before the run.

**9.5 The hang timeout.** Discharged NO-CHANGE (§3): D-431's bench measured
the heuristics seat's worst single search at 430 ms at the registered budget,
a ~279x margin under 120000 ms.

**9.6 The run's revision.** The commit Step 6's games are played at,
recorded before the first game, so the report's `experiment_sha256` has
something to be compared with.

**9.7 `configs/arena_wp17_heuristics_vs_staged.toml`.** Authored and
committed at Step 6's launch, once §9.2/§9.6 are known — "the document that
IS the experiment" (§3).

## 10. What flips this document

An amendment to any section reopens its review, however small the diff. It
binds the instruments too, each pinned with WHERE (§7A.1's four items). The
claim itself flips on the run (§5). The DOCUMENT flips if the committed
staged policy or its three gates move before the run (which would change
what engine B is), if the arena's verdict vocabulary changes, or if the
three-key diff of §1 stops being the only difference between the seats.

## 11. REVIEW STATE

**REVISION 1 (this text) IS UNREVIEWED AND GOVERNS NOTHING YET.** A
fresh-context review must pass it before the governed run may be launched.
This WP's review ledger so far: design REVIEW-design + DECISION-RED-TEAM
(D-429), REVIEW-impl (closed at `fc4bc69`), and this document's own review —
the last gate before the run.
