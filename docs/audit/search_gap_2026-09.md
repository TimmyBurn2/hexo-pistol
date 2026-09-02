# Search-enhancement gap — 2026-09 (roadmap input under D-569(c); nothing here is a decision)

**Revision audited.** `d83ac01` (worktree `/home/tom/pistol-runs/audit-2026-09`).
Every "present/absent" cell cites `crates/pistol-search/src/<file>:<line>` at
that revision.

**Literature, and a citation correction.** The dispatch cites "Rapfi
arXiv:2503.03178". That identifier resolves to *Active operator learning with
predictive uncertainty quantification for partial differential equations*
(Winovich et al.) — not Rapfi. The Rapfi paper is **arXiv:2503.13178**, *Rapfi:
Distilling Efficient Neural Network for the Game of Gomoku* (Jin, Duan, Hang,
2025-03-17), the paper `docs/research/minimax_report.md:4` already quotes.
Every number from it below is **15×15 square-board gomoku, NON-TRANSFERABLE**;
every chessprogramming.org number is **8×8 chess, NON-TRANSFERABLE**. Rapfi
names its stack twice — §2.2 "futility pruning, razoring, null move pruning,
late move reduction" and Appendix A.4 "futility pruning, late move reduction,
singular extension and null move pruning" — and gives **no per-technique
ablation**; its headline numbers are throughput (Table 1: 428K / 257K / 104K
nps for Mixnet small/medium/large vs 1 758 → 32 nps for ResNets) and "≈ 400 Elo
above the SOTA agent" CPU-only (§5.3). So the literature supports "these are in
the stack of the strongest engine of the family", never "this one buys N Elo".

**pistol's own priors, which outrank the literature here.** Two chess-lineage
heuristics have already been measured on this game at eval v0: WP-1.7's
killers/history/countermove returned `h0` (D-433, `llr_pair -2.9911`) and
WP-1.6's threat quiescence returned `h0` (D-428, `llr_pair -2.9787`), both at
50 000 nodes over `random_openings_v1`. The report's own verdicts
(`minimax_report.md:142-143`) put LMR/futility/razoring/null-move at PROTOTYPE
with "margins change" as the stated risk. D-465's WP-1.8c bracket abort and
D-534's 725 ms overshoot are standing facts any extension package inherits.

**Two measured facts the ranking rests on** (MEASURED in this audit,
`docs/audit/repo_audit_2026-09.md` A-02 for the receipt). (i) At the
committed instrument seat on a 15-stone fixture position, iteration node
counts were `837 / 2 986 / 50 453 / 90 024` for depths 1–4 turns (cumulative
`837 / 3 823 / 54 276 / 144 300`), so **50 000 nodes completes depth 2 and
400 000 completes depth 4**; the odd/even alternation says the per-turn
branching is dominated by the BATCHED row's width at one side's turn. (ii) The
per-node profile is `delta` 31.8 % + threat-state maintenance 36.1 %: width,
not evaluation depth, is where a node's cost goes. Both facts point the same
way as sealbot's shape (§B.5): the lever is the candidate set's WIDTH, and every
reduction below is a way of not paying for it.

## Inventory

| id | enhancement | pistol at d83ac01 | where |
|---|---|---|---|
| C-1 | aspiration windows | **absent** — every iteration opens `(-INFINITY, INFINITY)` | `pvs.rs:171`; `search.rs:419-426` |
| C-2 | Tier-T width cap / widening | **absent on Tier T**; the only cap is the safety net on the QUIET ball, gated `0` in every committed config; `quiet_top_k`/`widen_schedule` are validated and advertised but consumed by nothing | `staged.rs:222-235`; `pvs.rs:363-373`; `crates/pistol-engine/src/instance.rs:235-236`; `configs/instrument_v0.toml:67-68,72` |
| C-3 | LMR | **absent** — every non-first child gets the full remaining depth behind a null window | `pvs.rs:542-580` |
| C-4 | internal iterative deepening | **absent**; ordering never lacks a signal (delta rank on every row), so the classic trigger does not arise | `pvs.rs:297,377`; `staged.rs:283-295` |
| C-5 | futility pruning at the frontier | **absent**; the frontier is a turn boundary (D-111) and evaluates or enters the gated quiescence | `pvs.rs:226-255` |
| C-6 | razoring | **absent** | `pvs.rs:226-255` |
| C-7 | null move / monotone analogue | **absent** as a pruning device; the sound analogue that exists is `LAW-OVERLOAD`'s early return (the opponent's unstoppable win ends a non-PV node without expansion) and the solver-on-path verdict (gated off) | `staged.rs:148`; `pvs.rs:338-340,283-290` |
| C-8 | singular / threat extensions | **absent** as extensions; the forced prefix is ordered first, never extended; quiescence (a threat extension at the horizon) exists and is gated `q_depth_turns = 0` (D-428) | `staged.rs:156-204`; `quiescence.rs:68-92`; `configs/instrument_v0.toml:75` |
| C-9 | lazy SMP | **absent by rule**; `instrument.threads != 1` is refused; Stage 4 | `crates/pistol-engine/src/instance.rs:149-158`; `docs/ROADMAP.md:466-472` |
| — | present, for the record | PVS null-window scan `pvs.rs:558-579`; iterative deepening `search.rs:419`; TT with generation ageing `tt/mod.rs:116-195`; threat-first staged generation `staged.rs:115-154`; killers/history/countermove gated `heuristics.rs`; solver-on-path gated `search.rs:320-396` | — |

## Per-enhancement analysis

Each row: what the literature says it buys (NON-TRANSFERABLE), the specific
risk on THIS game (two-stone turns, overline wins, threat-first staging), the
package shape, and an honest expectation. "Warm-replay attribution" means the
WP-1.5b/1.6 instrument (`tools/wp15b_attribution_check.py`,
`tools/wp16_warm_attribution_check.py`) re-run on the SPRT's own games.

### C-1 aspiration windows
- Literature: "typical window sizes are 1/2 to 1/4 of a pawn on either side of
  the guess", widen the failing bound, exponentially in Stockfish
  (chessprogramming.org/Aspiration_Windows); no Elo or node figure given.
  Universal in Yixin/Embryo/Rapfi (`minimax_report.md:35`, "BUILD; table
  stakes").
- Risk here: the score band is `EVAL_MAX < MATE_THRESHOLD` (`score.rs:26-29`)
  with mate scores common in sharp positions — a window that fails on a mate
  must widen straight to `±INFINITY` or it re-searches every iteration; a
  fail-low/high loop under a node budget changes which iteration completes,
  so the byte-identity of `instrument_golden_v1.txt` is lost by design.
- Package: ~30-line diff in `search.rs:419-426` (window from the last
  iteration's score, ±δ in eval units, mate ⇒ full width), SPRT at 50 000
  nodes, fresh `book_v2` slice, warm-replay attribution; a node-matched
  time-to-depth bench as the second instrument (rule 5).
- Expectation: small positive or null on node count (a re-search on a window
  the PVS null-window scan already exploits buys less than in engines that lack
  PVS); the win, if any, is time-to-depth, which an SPRT at fixed nodes cannot
  see. Cheap enough to run first.

### C-2 Tier-T width cap and widening (WP-1.5c's owner)
- Literature: Rapfi/gomoku engines cap candidate counts (Rapfi lists "move
  count" style limits only implicitly); sealbot's own measured shape is 15
  cells → 56 pairs per node (`docs/audit/sealbot_study_2026-09.md` §1) and its
  register claims root 26 is a blowout (SB-06, UNVERIFIED).
- Risk here: uncompensated capping is what makes sealbot colony-blind
  (`sealbot_notes.md` "What NOT to copy"); on this board a cap that drops a
  distant defensive cell loses to an overline-capable double threat the
  FILTERED row would have caught one turn later. `safety_net_top_k` at K = 16
  was `inconclusive_at_game_cap` with `nelo_pair +16.9 ci95 21.5` (D-491/492)
  — the only width measurement on record, and it is the QUIET ball, not Tier T.
- Package: WP-1.5c as the ROADMAP already cuts it (`ROADMAP.md:124-138`): a
  width HISTOGRAM instrument first (Tier T set sizes per row class — no such
  counter exists; `StageCounters` counts rows, not widths, `info.rs:25-93`),
  then a delta-ranked top-K with fail-low re-widening, matrix M2, SPRT.
- Expectation: the largest lever in the field — sealbot reaches 1–3 turns
  deeper at equal wall (`sealbot_notes.md` "Measured performance") on width
  alone — and the one with a measured failure mode. The instrument is worth
  landing even if the cap is not.

### C-3 late move reductions
- Literature: reduces "the effective branching factor to less than 2"; not
  applied at depth < 3, to killers, checks, captures, PV nodes
  (chessprogramming.org/Late_Move_Reductions); formulas of the shape
  `a + ln(depth)·ln(moves)/b`. Rapfi lists it (§2.2, A.4). No Elo figure.
- Risk here: (i) a reduction of one PLY lands mid-turn — D-111 forbids a
  horizon there — so reductions must be in whole turns (2 plies), which is a
  coarse step at depth 3–5 turns; (ii) the "late" moves are the unforced range
  of a BATCHED row, exactly where WP-1.7 measured that chess ordering priors
  did not survive (D-433); (iii) a reduced quiet move that misses a double
  threat is a sudden-death loss the re-search may not reach under a node cap.
- Package: reduce only `cells[forced..]` beyond index k, only at
  `depth_plies >= 6`, never on the FILTERED/WIN-NOW rows, re-search on
  `scan > alpha` (the PVS hook at `pvs.rs:575` already exists); SPRT; the
  warm-replay attribution must show the reduced subtree was the changed one.
- Expectation: the EBF measured in this audit (see the fact block) says what is
  on the table; with Tier T uncapped the "late" range is where the width lives,
  so LMR is a cheaper cousin of C-2 and should be measured after it, not
  before.

### C-8 forced-reply / singular extension
- Literature: singular extensions "extend … if one move seems to be a lot
  better than all of the alternatives", via a reduced-depth exclusion search
  (chessprogramming.org/Singular_Extensions; Stockfish 1.6's "large jump" is
  unquantified there). Rapfi A.4 lists it.
- Risk here: the cheap version needs no exclusion search — a FILTERED row with
  exactly one cover cell (`forced == cells.len() == 1`) IS singular by
  construction. Extending it by a whole turn is branching 1; the risk is
  chain length in threat sequences, bounded today only by `MAX_PLY`
  (`search.rs:41`) and the ply-indexed tables sized to it.
- Package: extend one turn on a one-cell FILTERED row, cap the extension budget
  per line like `q_depth_turns`; SPRT. This overlaps WP-1.6's quiescence, which
  was `h0` at eval v0 — the extension only helps if the horizon misread is at
  the forced reply and not in the static eval, which is the same question.
- Expectation: null-to-small at eval v0 (D-428's reasoning transfers); cheap
  enough to bundle with C-1's run as a second arm.

### C-5 futility pruning / C-6 razoring
- Literature: frontier futility drops moves that "have no potential of raising
  alpha" with a margin; disabled in check and near mate; extended futility at
  depth 2 (chessprogramming.org/Futility_Pruning). Razoring: "search it to a
  reduced depth" at depth ≤ 3, Stockfish's 2022 reintroduction "~1 Elo"
  (chessprogramming.org/Razoring).
- Risk here: both need a static eval trusted to ±margin. WP-1.6 measured that
  eval v0 misreads horizons badly enough that threat quiescence was net
  negative (D-428); a futility margin on that eval prunes the wrong moves.
  Overline wins mean a "quiet" stone can complete a seven; only the FILTERED
  row protects against that and it must be exempt.
- Package: defer until the Stage-2 eval lands (ROADMAP Stage 2 acceptance
  ≥ +150 Elo); then SPRT with the margin as a config key, never a literal.
- Expectation: negative or null at eval v0; the report already parks these at
  PROTOTYPE with "margins change" (`minimax_report.md:143`).

### C-7 null move, and its sound analogue
- Literature: reduction R = 3–4, "verified null move" against zugzwang,
  disabled in check (chessprogramming.org/Null_Move_Pruning). The report:
  "PROTOTYPE null-move in the quiet regime with a threat-verification guard
  (never null-move when the opponent has an open double-threat)"
  (`minimax_report.md:37`).
- Risk here: a pass hands the opponent TWO stones — in sudden death that is
  the whole game, and the guard the report demands is exactly the FILTERED /
  OVERLOAD classification the staged row already computes. So the cheap
  sound analogue is not a new prune but a widening of `LAW-OVERLOAD`: a
  non-PV BATCHED node whose static eval is ≥ beta by a margin AND where the
  opponent holds no live three could return the bound. That is futility by
  another name and inherits C-5's eval dependence.
- Package: none now; re-open with Stage 2.
- Expectation: null at eval v0.

### C-4 internal iterative deepening
- Literature: "pretty much a washout on average", an insurance against
  10× time blow-ups when no hash move exists (chessprogramming.org/IID).
- Risk / package: pistol never lacks an ordering signal (delta rank on every
  row; TT move promoted within tier), so the trigger condition is empty.
  No package. Expectation: nothing to measure.

### C-9 lazy SMP (note only)
- Literature: "scales surprisingly well up to 8 cores and beyond" in nps and
  strength, worse than YBW in time-to-depth, nondeterministic by construction
  (chessprogramming.org/Lazy_SMP). The report: BUILD in Stage 4
  (`minimax_report.md:85,144`).
- Here: `instrument.threads` is refused unless 1 (`instance.rs:149-158`);
  hard rule 4 keeps the instrument single-threaded whatever lands. Stage 4;
  no package from this analysis.

## Ranking by expected payoff over cost

| rank | id | expected payoff | cost | why here |
|---|---|---|---|---|
| 1 | C-1 aspiration | small, time-to-depth | ~30 lines + one SPRT | cheapest positive-expectation change; also the prerequisite for any later fail-soft work |
| 2 | C-2 Tier-T width instrument, then cap | largest in the field | WP-1.5c: matrix, red team, SPRT | the only lever with a measured 1–3-turn depth gap behind it; the histogram is cheap and blocks nothing |
| 3 | C-8 one-cell forced-reply extension | small | ~20 lines, bundle with C-1 | branching-1 extension; overlaps WP-1.6's `h0` |
| 4 | C-3 LMR on the unforced range | medium if C-2 finds width | ~40 lines + SPRT, after C-2 | same mechanism as a cap seen from the depth side; whole-turn reductions only |
| 5 | C-5/C-6 futility, razoring | null at eval v0 | after Stage 2 | eval-margin techniques on an eval measured to misread horizons |
| 6 | C-7 null-move analogue | null at eval v0 | after Stage 2 | reduces to C-5 once the sound guard is applied |
| 7 | C-4 IID | none | — | trigger never fires here |
| 8 | C-9 lazy SMP | — | Stage 4 | out of scope by rule |

Every row above carries the SPRT gate its package will face (rule 6): paired
`book_v2` openings, fixed nodes, `elo1`/`alpha`/`beta` as the committed arena
configs state them, distinct-n reported, and no committed config moves on
anything but `verdict h1`.
