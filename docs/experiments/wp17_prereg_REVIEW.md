# REVIEW — WP-1.7 SPRT pre-registration (`docs/experiments/wp17_sprt_prereg.md`)

**Revision reviewed: `be5cbdbe86d4f2b66e6ce5a998b36ee1dffec2c9`. HEAD matches: YES**
(`git rev-parse HEAD` → `be5cbdb…`; the document was read at HEAD, not `git show`).

Reviewer: fresh context; authored nothing reviewed here. Every receipt below was
executed, not read. Scratch work under `/tmp/opencode/wp17rev/`; the repository
was not modified (the seeded-arm reproduction ran on copies).

---

## 1. Receipts tested — all of them hold

| Claim (site) | Test | Result |
|---|---|---|
| §7A.1 pin 1: warm-replay path unchanged since `bfdf933` | `git diff --stat bfdf933..HEAD -- crates/pistol-arena/` | **prints nothing** ✓ |
| §7A.1 pin 2: `6c929da` last touched `tools/wp16_warm_attribution_check.py` | `git log --oneline -1 HEAD -- tools/wp16_warm_attribution_check.py` | **`6c929da`** ✓ (also `git log --oneline -1 6c929da -- …` names it, as registered) |
| §7A.1 pin 4: `a80a864` last touched `tools/wp15b_attribution_check.py` | `git log --oneline -1 HEAD -- tools/wp15b_attribution_check.py` | **`a80a864`** ✓ |
| §7A.1 pin 3: binary digests | fresh `sha256sum target/release/pistol target/release/arena` | `665d2815ddba28e7889ebea661a10b15352036ab46bfc6f1758d72813cad4184` / `3e5c114fee9b1d8018b733075b2eaaeb7625ea2d14d387123f53c727173e5851` — **both exact** ✓ |
| §7A.1 pin 3 corollary: nothing a release binary is built from moved since `fc4bc69` | `git diff --stat fc4bc69..HEAD -- crates/ Cargo.toml Cargo.lock` | **prints nothing** ✓ (`be5cbdb` adds only the dry-run config and this document) |
| §7A.1 Criterion 1'' quotation | diffed against `wp16_warm_replay_design.md` §4 point 4 (current revision `b6afd66`, confirmed via `git log`) | **character-for-character identical** modulo the design's 3-space list continuation indent vs the blockquote's re-wrap — the same treatment WP-1.6's passed reviews accepted ✓ |
| §7A.1 agreement-criterion quotation | diffed against WP-1.6 §7A.1 | **identical** modulo blockquote re-wrap ✓ |
| §8.2 artifact digests | `sha256sum artifacts/wp17_dryrun_{run,replay,swapped,swapped_replay}.txt artifacts/wp17_bench_v1.txt` | all five **exact**, including `3818f464…`, `95650062…`, `ffeab3da…`, `f7c9a56e…`, `e34a6931…` ✓ |
| §8.2 quoted checker lines (W-1, W-3) | re-ran `python3 tools/wp16_warm_attribution_check.py artifacts/wp17_dryrun_run.txt artifacts/wp17_dryrun_replay.txt target/release/pistol` and `python3 tools/wp15b_attribution_check.py artifacts/wp17_dryrun_run.txt target/release/pistol` | both **exit 0**; every quoted line (`W coverage: 8 game(s)…`, `W classification: 0 divergence(s)…`, `(b): 0 inert pair(s)… 4 pair(s) directly attributed…`, `1b: 5 decided…`, `1c: 8 game(s) and 4 pair(s)…`, `PASS — 0 failure(s)`; `1a: 16 turns replayed, 2 of them discriminating, 2 of 8 games…`, `PASS — 0 failure(s)`) **character-exact** ✓ |
| §8.2 timing receipts | `timing n_workers 4 wall_ms 17367` (run), `wall_ms 17379` (replay) | both read off the artifacts ✓; `17379/17367 = 1.0007` ≈ the registered `1.00x` ✓ |
| W-1's node-equality referent | compared `nodes_a`/`nodes_b` of all 8 `game` records against all 8 `replay` records | **equal in every game** ✓ |
| §1 "CONFIRMED by diff … exactly THREE value lines" | `diff configs/instrument_staged_h_v0.toml configs/instrument_staged_v0.toml` | only value differences are `killers`/`history`/`countermove` (`true`/`false`); everything else differs in comments only ✓ |
| §3 / §9.5 hang-timeout calibration (D-431: worst single search 430 ms, ~279x) | recomputed from `artifacts/wp17_bench_v1.txt`: 240 searches attributed to configs by the artifact's own `# config` headers; ON-seat max `time` = **430 ms** (OFF-seat max 517 ms) | `120000/430 = 279.07` ✓ |
| §5 h1 row: `instrument_v0.toml` "identical in policy" to `instrument_staged_v0.toml` | extracted both `[search.candidate_policy]` blocks | **identical, key for key** (both staged, `q_depth_turns = 0`, gates `false`) ✓ |
| Provenance: "the full three-seat determinism gate is green" | ran `tools/determinism.sh` | `determinism: ok — 3 seat(s), no difference outside nps/time in any of them`, exit 0 — the third seat `gate_staged_heuristics_v0.toml` is in `SEATS` ✓ |
| §8 literal command `tools/config_check.sh configs/arena_wp17_dryrun.toml` | ran it | `validate_arena_config: 1 document(s) ok`, exit 0 ✓ |

## 2. The W-2′ registration (§8.1) — reproduced, and the mechanism story is TRUE

I reproduced the seeded arm independently on copies under `/tmp/opencode/wp17rev/`:

1. Transposed `p1`/`p2` in every `game` record of a copy of
   `artifacts/wp17_dryrun_run.txt` (8 records) — **the result is byte-identical
   to the recorded `artifacts/wp17_dryrun_swapped.txt`** (`diff` clean), so the
   recorded transformation is exactly the registered one.
2. `arena --replay` on my copy: **exit 1**, `arena: replayed 8 of 8 game(s) …,
   8 divergence(s)`; my replay differs from the recorded
   `wp17_dryrun_swapped_replay.txt` only in the `timing … wall_ms` line
   (machine-dependent, excluded from every comparison by §4).
3. The checker on my pair: **exit 3** with
   `DETERMINISM VIOLATION: game 5 turn 14: the report records `-2,-1/5,-1`; the
   credited seat `staged`, replayed WARM at exactly the state the run had,
   answers `4,-1/5,-1`, and the other seat, asked cold at the same prefix and
   the same budget, answers `4,-1/5,-1`. Nothing known explains what was
   played.` followed by the two-possible-causes message — the exact shape §8.1
   registers.

**The mechanism story is verified, not merely plausible.** The swapped replay
document carries eight `divergence` records at turns **5, 5, 11, 11, 14, 14,
11, 11**: the early ones (games 1–4) were probe-confirmable inversions, and the
checker's scan reached game 5 turn 14 as the first divergence whose dual probe
fails — both probes answer `4,-1/5,-1`, the recorded move is `-2,-1/5,-1`, and
by turn 14 the cold subprocess no longer reproduces the warm engine's answer
(D-383's own measured mechanism). Contrast WP-1.6's dry run, whose swapped arm
diverged at turn 5 in every game — the first searched turn, where cold = warm —
and confirmed all eight as inversions. The registration's load-bearing content
(the cold probe's confirmability expires as the game warms; the swap is still
refused; nothing downstream is read) is exactly what the artifacts show.

**The registration leaves no after-the-numbers choice standing.** The criterion
as registered ("checker exits NON-ZERO with a named finding") is falsifiable —
an instrument blind to a seat swap exits 0 on the swapped report and fails
W-2′, while W-1 independently excludes the refuse-everything instrument; only a
discriminating chain satisfies both. The observed exit-3 shape is registered
*as observed* with its mechanism, and the governed-run consequence of exit 3 is
pinned to WP-1.6 §5's row (hard stop) — see MINOR 4 for the one wording defect
in this story, and §3 below for the one loosening worth recording: a future
re-take of this dry run that exits **1 or 2** would also satisfy the registered
criterion. On this transformation no premise refusal can fire (transposing
*both* games of each pair preserves the swapped-seating premise), so an exit 2
would itself be an instrument anomaly; the looseness is defensible, but it is
the document's, not the reviewer's, to have said so.

## 3. Findings

### MAJOR 1 — The verdict space is NOT total: `inconclusive_at_game_cap`, `arena_report_aborted`, and the pre-game-refusal/no-report case have no registered consequence reachable from this document

§5's pointer sentence scopes WP-1.6's table to the **checker's exit taxonomy**:
"The exit-code taxonomy of the Criterion 1'' chain — what exit 0/1/2/3 of
`tools/wp16_warm_attribution_check.py` may be concluded to mean — is WP-1.6
§5's table". The last row points at WP-1.6 §5 only for "Criterion 1'' fails /
instruments disagree / any exit not 0". WP-1.7's own rows cover `h1` (two),
`inconclusive_degenerate`, `h0`, and forfeits. What remains uncovered:

- **`verdict inconclusive_at_game_cap`** — reachable by construction (the dry
  run itself printed it; `game_cap` = 2 × take = 1000 games for the governed
  run), and *likely*: §6's own registered expectation is "null to small
  positive", i.e. a true elo near zero, exactly the region where the LLR drifts
  slowest and a boundary is least likely to be crossed before the cap.
  WP-1.6 §5 has a row ("Reported as inconclusive. No action. The sample is
  reported with its LLR and its distance from both bounds"); WP-1.7 has none,
  and no pointer reaches it. The choice it leaves standing is real: §6 says
  "`h0` … closes the WP as a measured finding" — a cap-inconclusive run could
  be read after the numbers either as h0-like closure or as WP-1.6's
  "no action", and those license different conclusions about the same numbers
  (WP closed as a measured null vs open with no conclusion).
- **`arena_report_aborted`** (`crates/pistol-arena/src/report.rs:51`) — the
  hang-timeout/liveness abort path. WP-1.6 §5 has a row ("No verdict exists.
  The games are a diagnostic and explicitly not a sample"); WP-1.7 has nothing.
- **A pre-game refusal with NO REPORT AT ALL** (digest mismatch, config
  refusal, `--out` exists) — WP-1.6 §5 has a row; WP-1.7 has nothing.

The whole-table reading of the pointer is not available either: WP-1.7's own
h1/h0 rows *replace* WP-1.6's h1/h0 actions (flip the three gates vs accept the
quiescence extension), so the pointer cannot import WP-1.6's table wholesale —
which leaves the three rows above in a dead zone. Under the
registered-consequence rule this is precisely "the after-the-numbers decision
[the document] exists to forbid", in the outcome region §6 itself predicts.
**Fix: one row each (or one pointer that names them), stating the consequence
before the run.**

### MAJOR 2 — The `inconclusive_degenerate` row points at "WP-1.6 §5's own degenerate row" (singular) and prescribes the WRONG action for the `distinct_n == n/2` case — which is more plausible in THIS matchup than in any before it

WP-1.6 §5 has **two** degenerate rows: `distinct_n == n` (read the direction
off the pentanomial — the row WP-1.7's pointer describes) and `distinct_n ==
n/2` ("The two seats played identical games — a document or digest error …
Investigate the instrument, not the engine"). WP-1.7's row mentions
`distinct_n` not at all and instructs "Read the direction off the pentanomial
exactly as WP-1.6 §5's own degenerate row prescribes".

Two defects compound here:

1. **Misrouting.** For this WP, `distinct_n == n/2` is the signature of the
   seat-swap/config-load failure class this very document's W-2′ exists to
   catch (engine A silently running engine B's document). Following WP-1.7's
   pointer literally sends that case to k=2 → the h0 row → "closes the WP as a
   measured finding" — an instrument defect read as a strength result.
2. **The case is genuinely live.** Two deterministic seats differing only in
   move ordering can play identical games *legitimately* — and this matchup is
   the closest pair of seats this project has ever run: the dry run's own
   record (§8.2) is that only **2 of the cold checker's 16 replayed turns
   discriminated**, and WP-1.6's governed run — a *larger* behavioural
   difference — already produced 2 identical-game pairs in 450. If the three
   gates never change a chosen move, all 500 pairs come back identical, all
   buckets are p2, and the verdict is `inconclusive_degenerate` with
   `distinct_n == n/2`. Note that for THIS matchup that outcome is not even
   clearly WP-1.6's "investigate the instrument" case — genuinely identical
   games would be a *strong, legitimate null* — which is exactly why this WP
   needs to register its own consequence for it rather than inherit a pointer
   that misdescribes it. **Fix: name `distinct_n` in the row and register the
   n/2 consequence explicitly (instrument investigation vs genuine-null
   discrimination — e.g. via the three-key diff / config digests, which the
   report already carries).**

### MINOR 1 — §7's governed-run cost estimate misreads D-398's node tax as a fixed-nodes time tax, and is not reconciled with the document's own dry-run measurement (D-291 shape)

The row's derivation ("WP-1.6's 450-game run cost 707 s wall with a seat
carrying a 2.48x node tax … both seats plain-staged prices") treats the 2.48x
fixed-depth *node* ratio as a fixed-nodes *time* ratio, which is what producing
"8-15 min" from 707 s requires (a 1.74x per-game reduction). At the arena's
fixed 50 000-node budget both seats spend the same nodes, and D-398's own
figures price the quiescence seat's per-search time at only 2.80/2.48 ≈
**1.13x** (ttd-ratio over node-ratio). The honest anchor is D-428's measured
707 s / 450 games → **~26 min at the full 1000-game cap**, above the bracket;
and the document's own §7/§8.2 dry-run measurement (17.4 s for 8 games) sits on
the same face implying a similar full-cap figure and is nowhere reconciled with
the estimate. Cost constrains no reading, but CLAUDE.md puts the run's cost on
the document's face precisely so the proportion is visible, and this bracket
understates the cap case by ~2x using arithmetic derivable in seconds from the
document's own numbers — the exact D-291/D-419-MINOR-E shape. Suggested fix:
restate as "ESTIMATED ~12 min if the SPRT early-stops near WP-1.6's 450-game
crossing; ~26-36 min at the full 1000-game cap".

### MINOR 2 — §8.1's mechanism sentence "the first divergence lands late (game 5, turn 14)" is falsified by the artifact it describes

The swapped report's first divergence is **game 1, turn 5** (the first searched
turn, `opening_turns 4`), and games 3, 4, 6, 7, 8 diverge at turn 11; what is
true — and what the registration needs — is that the first divergence the dual
probe **cannot confirm** is game 5 turn 14 (divergence records 0–3, at turns
5/5/11/11, were confirmed inversions). The verified mechanism (§2 above) is
unaffected; the sentence as written would not survive being checked against
`artifacts/wp17_dryrun_swapped_replay.txt`. One-clause fix.

### MINOR 3 — The dispatch quotations that fix §2's `elo1` and the header's launch delegation are unverifiable from the repository

§2 fixes `elo1 = 15.0` by quoting the commissioning dispatch ("elo0/elo1,
budget, book stay as registered in WP-1.6; not re-read") and the header quotes
the delegation sentence ("Delegation for the governed run granted in-dispatch
(D-382 pattern)"). Unlike WP-1.6 — whose dispatch's delegation was recorded
verbatim in the ADR log (decisions.md D-396/D-402, "this line is that record")
— no WP-1.7 ADR line records either sentence (D-429/D-430 record the dispatch's
subagent policy and keying/credit contract only). The *values* are consistent
with WP-1.6's registrations, so nothing turns on it today, but the fixity of a
fill-in slot and the launch authority both rest on quotes only the operator can
check. Fix: record both sentences in the launch/closure ADR line (the D-382
pattern WP-1.6 itself established), or have the operator confirm them at the
slot pass.

### MINOR 4 — §7A.1/§8.1 restate WP-1.6-owned claims while asserting they are "pointed at rather than restated" — the D-419-MINOR-F shape, three times in one document

§7A.1's closing sentences: "The registered consequence of disagreement is
WP-1.6's own: the run is not a measurement, the verdict is not read, and the
disagreement is investigated as an INSTRUMENT defect before anything is
concluded about either engine. The stage the second instrument does not share
is WP-1.6 §7A.1's own naming (the WARM DRIVE); what both are blind to is the
report WRITER. **Both rows are pointed at rather than restated.**" — every one
of those three items IS restated (verbatim, in the case of the consequence) in
the immediately preceding sentences, and the paragraph then claims it did not
happen. §8.1 does the same for exit 3's consequence ("WP-1.6 §5's row (hard
stop, bigger than the WP, investigate), pointed at and not restated") and §5's
intro flirts with it. The copies are identical to WP-1.6's text today, so this
changes no reading (and could be overruled as prose under D-424's test) — but
it is a false self-description in a receipts-governed document, and it plants
exactly the second-site-fix mechanism that D-423 codified the state-it-once
rule to abolish. Fix: delete the restatements or delete the words "pointed at
rather than restated" wherever they are false.

## 4. Attacks that did not hold (recorded so the next round knows they were run)

- **Claim-made-twice hunt (duty 2).** The exit taxonomy is genuinely NOT
  restated: §7A.1 quotes the instrument's four exit-code *constants* (as
  WP-1.6 §7A.1 itself does) and points for meanings; §2 and §4 point at WP-1.6
  §2/§4 without restating (verified: no pair-floor derivation, no field list in
  this document). WP-1.7's own §5 rows do not contradict the WP-1.6 rows they
  sit beside (different WPs' actions, same shapes). The one genuine restatement
  cluster is MINOR 4.
- **Fresh-slice arithmetic (duty 5).** Verified end to end: D-402 retired
  `skip 0, take 500` (0..499); WP-1.6's governed run consumed `skip 500, take
  500` (500..999) — `configs/arena_wp16_defensive_only_vs_staged.toml` reads
  `openings_skip = 500`, D-427 records the slot-pass correction that made it
  so, and D-428 records the run under it; WP-1.7 registers `skip 1000, take
  500` (1000..1499) in the document itself (§9.2a, D-427's lesson applied).
  `random_openings_v1.txt` holds **2000 openings** (61 comment lines, 2000
  data lines) — ≥1500, so the slice exists whole. Disjoint from both prior
  draws by construction.
- **Dry-run rule, same arm (duty 6).** Attacked as instructed: WP-1.6's dry
  run reached for a different *arm*; WP-1.7's runs the registered matchup. The
  attack fails, on the rule's own text: the registered workload is the SAMPLE —
  500 openings of `random_openings_v1` at skip 1000 — and the dry run touches
  none of it (different book, take 4, skip 0); it is a real instance of the
  kind exercising exactly what the rule wants exercised (attribution on a real
  arena report), and the same-arm choice is what surfaced the W-2′ exit-3
  shape — the hard case of seats that agree, i.e. THIS matchup's own risk. The
  document fences the dry run's game content from strength readings
  ("Nothing in this file may be quoted as a measurement"; §8.2's
  "not read as anything"), and its one strength-adjacent observation (2 of 16
  turns discriminating) is recorded as instrument behaviour consistent with
  §6's pre-registered expectation, changing no registered consequence. Legitimate.
- **Scope (duty 8).** Nothing WP-1.6 closed is reopened: `elo1 = 15.0`,
  `nodes 50000`, `random_openings_v1`, `turn_cap 40`, 100-pair floor, alpha/
  beta — all as registered; the h1-below-floor confirmatory procedure is
  WP-1.6's own row (D-190's instrument, `openings_v1.txt` confirmed 1591
  openings); the h0 row's "planning finding, never a threshold move" and the
  licensed-not-scheduled relaxations match the design's §4 record. No post-hoc
  reading is licensed — except where MAJORs 1 and 2 leave gaps.
- **Numbers (duty 7).** Every numeric claim checked is marked and sourced:
  430 ms / ~279x MEASURED (recomputed from the bench artifact, §1 above);
  17.4 s / 17.4 s / 1.00x MEASURED (read off the artifacts); 0.029 s and
  6.485 s correctly attributed to WP-1.6 §7; §6's expectation marked ESTIMATED
  with its honesty stated. The one D-291 finding is MINOR 1.

## 5. Observations (not counted as findings)

- WP-1.6's prereg revision 10 header still reads "UNREVIEWED … does NOT govern
  a run until a fresh review passes it", while D-428 records the governed run
  as "governed by … revision 10" under the operator's D-426 overrule. That is a
  WP-1.6 internal inconsistency (the header predates the overrule's use), noted
  only because WP-1.7's provenance leans on WP-1.6's run having been governed —
  the ADR record is unambiguous that it was, and by this revision's instruments,
  which are pinned here byte-for-byte and re-verified above.
- §8.2's W-2′/W-3 quotes use marked `…` elisions (e.g. "1a robustness: 3
  vacuous pair(s) … verdict inconclusive_at_game_cap unchanged" drops the
  openings list and "adversarially reassigned"). Every quoted fragment is
  character-exact and the elision is marked — unlike the unmarked truncation
  that was D-416's BLOCKING — and WP-1.6 §8.6 used the same idiom in a passed
  round. Acceptable; noted for completeness.

---

## Verdict

**FAIL — 0 BLOCKING, 2 MAJOR, 4 MINOR.**

The instrument chain is everything the document claims for it: every pin,
digest, quotation and dry-run receipt tested exact, the seeded arm reproduces
byte-for-byte to exit 3 with the registered message, the mechanism story is
true, the slice is genuinely fresh, and the dry run satisfies the dry-run rule
as written. What fails is the document's own headline discipline: the verdict
space is not total (MAJOR 1 — a dead zone in the outcome region §6 itself
predicts), and the one outcome made *more* likely by this matchup's own
registered redundancy expectation — all-identical games — is misrouted by a
singular pointer (MAJOR 2). Both are one-row fixes; neither may be fixed after
game one, which is why they fail the review rather than defer it.
