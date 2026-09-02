# Sealbot study — 2026-09 (read-only, design input under D-569(b))

**Scope.** sealbot's LOCAL source as design input, never as code to copy
(docs/research/sealbot_notes.md "What NOT to copy" stands). Every citation is
`<path>:<line>` in the clone at `/home/tom/Work/sealbot-scope/sealbot`, branch
`master`, tip `c94749c` ("Split engine.h into engine/ directory"), tree
`current/` — the engine the anchors ran (`local/sealbot_anchor_v2_seat2.toml`
names that directory). Paths below are relative to `current/`.

**Labels.** CODE = read from the cited line. MEASURED = a number this study
produced by running sealbot's own instrument (§5, command quoted). UNVERIFIED =
a claim from sealbot's own docs or from `docs/research/sealbot_deep_dive.md`
(78 entries, **0 VERIFIED/REFUTED** at d83ac01 — `grep -c "STATUS: VERIFIED"`
returns 0) that this study did not check. No strength claim anywhere (D-197).

**What "current/" is NOT.** No VCF solver, no NNUE, no SMP: those live on the
`nnue-eval`/`mixnet-repro` branches the deep dive indexes (its SB-01, SB-08,
SB-21). `current/engine/` is 1 361 lines of header-only C++ (`wc -l`), plus a
2 239-line vendored hash map; the register's branch claims are UNVERIFIED
here and out of scope.

## 1. Candidate policy

| question | answer | CODE |
|---|---|---|
| cell pool | every empty cell within hex distance ≤ 2 of any stone (`NEIGHBOR_DIST = 2`); maintained incrementally by reference counts on make/undo | `engine/constants.h:47`; offsets `engine/tables.h:48-51`; `engine/board.h:67-79,143-157`; init `engine/search.h:117-127` |
| per-node scoring | every pool cell scored by `_move_delta` = Σ over the 18 eval windows through the cell of `pv[new] - pv[old]` (3 axes × 6 offsets), sign-flipped for the minimizing side | `engine/board.h:185-196`; `engine/search.h:556-565` |
| sort / tie-break | descending delta, then ascending packed coordinate — deterministic, history deliberately excluded ("keeps candidate selection deterministic and independent of history heuristic") | `engine/search.h:559-570` |
| interior cap | top **15** cells (`CANDIDATE_CAP = 15 // 11`) | `engine/constants.h:44`; `engine/search.h:573-575` |
| pair rule | pairs `(i, j)`, `i < j`, `i + j <= PAIR_SUM_CAP = 14` over the ranked cells — **56 pairs** at full width (computed from the loop bounds) | `engine/tables.h:60-62`; `engine/search.h:584-589`; `engine/constants.h:46` |
| root cap | top **20** (`ROOT_CANDIDATE_CAP`) + one "colony" cell → all 210 (+20) pairs, then the must-block filter; the root list is re-sorted by the previous iteration's scores | `engine/constants.h:45`; `engine/movegen.h:138-168`; `engine/search.h:171-177` |
| colony cell | one hash-directed cell at centroid distance `max_r + 3`, root only — the escape the register's SB-37 describes | `engine/movegen.h:143-161`; `engine/tables.h:13-14` |
| widening | none; `no_cand_cap` (all pairs) exists only as a diagnostic flag | `engine/bot.h:23`; `engine/search.h:573-582` |
| where "~15" comes from | a literal beside its previous value (`// 11`); the register's claim that root 20 was measured optimal is UNVERIFIED (SB-06) | `engine/constants.h:44-45` |
| `DELTA_WEIGHT = 15` | multiplies every candidate's score equally — only its sign reaches the sort (SB-69 confirmed by reading) | `engine/constants.h:48`; `engine/search.h:557` |

Per-node ordering cost, CODE: `|pool| × 18` table lookups + one `std::sort` +
two vector allocations (`scored`, `cands`) + a `turns` vector of ≤ 56.

## 2. Threat layer

| mechanism | what it does | trigger / cost / cut | CODE |
|---|---|---|---|
| window counts | `(countA, countB)` per 6-window, 3 axes, flat `[3][140][140]`; updated on every make/undo (18 windows per stone) | O(18) per stone | `engine/bot.h:115`; `engine/board.h:36-54,115-130` |
| hot sets | a window enters `_hot_a/_hot_b` when its own count reaches 4 (opponent count NOT checked at insertion; every consumer re-checks `opp == 0`) | O(1) insert, O(\|hot\|) erase (linear scan) | `engine/board.h:43,51,121,128`; `engine/containers.h:26-35` |
| instant-win probe | first hot window with own ≥ 4 and opp == 0: one empty → pair it with the first other pool cell; two empties → that pair | every `_minimax` node and every quiescence node; O(\|hot\|) | `engine/movegen.h:13-47`; `engine/search.h:338-347,478-491` |
| must-block filter | opponent hot windows with own ≥ 4, opp == 0 become "must-hit" empty sets; only turns hitting every set survive; **an empty survivor set returns the UNFILTERED turns** (SB-59 confirmed by reading) | every node after pair generation; O(\|turns\| × \|sets\|) with a `flat_set` allocated per set per node | `engine/movegen.h:71-109` (unfilter at `:108`) |
| unblockable double | if the opponent has an instant win and > 1 must-hit set and no cell pair covers all sets → return a mate score without expanding | every node where the opponent is one stone from six; O(\|cells\|² × \|sets\|) | `engine/search.h:493-539` |
| threat cells (quiescence) | union of empties over hot windows with opp == 0 | per quiescence node, allocates a `flat_set` | `engine/movegen.h:49-69` |
| VCF / VCT / VCDT probes | **none on master.** The one-step VCDT test above is the whole of it; no forcing-line solver exists in `current/` | — | absence: `engine/*.h` contain no `vcf`, `solve`, `proof` symbol (`grep -il`) |
| threat depth seen | only windows at own ≥ 4 are "hot"; a three is invisible to every defensive path (SB-75 consistent with the `>= 4` at `board.h:43`) | — | `engine/board.h:43`; `engine/bot.h:84-94` (`has_near_threats` counts threes but has no caller in `engine/`) |

Where the layer cuts the tree, CODE: (a) a mover win-in-one ends the node
(`search.h:481-490`); (b) an unblockable opponent double ends it
(`:530-536`); (c) the must-block filter replaces ≤ 56 pairs by the covering
subset — in a FILTERED-like position that is often 1–3 pairs; (d) in quiescence
only threat turns are generated (`search.h:368`).

## 3. Evaluation

| term | shape | CODE |
|---|---|---|
| one term: Σ over all occupied-touching 6-windows of `PATTERN_VALUES[pattern]` | 729 = 3^6 doubles, one per ternary window pattern, player-relative (`_cell_a/_cell_b` remap so the table reads from the root mover's side) | `pattern_data.h:3-5`; `engine/search.h:62-64,90-115` |
| incremental | per placed stone: 18 window pattern indices updated, `_eval_score += pv[new] - pv[old]`; undo symmetric | `engine/board.h:56-65,132-141` |
| initialisation | full rebuild at every `get_move` (`memset` of five `[3][140][140]` arrays, then O(stones × 18)) — an UNTIMED prefix before the deadline loop | `engine/search.h:17-127` |
| numeric type | `double` throughout; scores are not integer | `engine/bot.h:98,133` |
| weight provenance | README says CMA-ES tuned (UNVERIFIED); the deep dive's SB-41..SB-53 index the pipeline on other branches (UNVERIFIED) | `../README.md:9`; `../experiments/cma/optimize.py` |

Cost shape, CODE: make + undo = 2 × (18 window-count updates + 18 pattern
updates + 18 neighbour-reference updates + one hash xor). No multi-window
interaction, no "two independent threats" term (SB-63's claim is consistent
with a single additive table; UNVERIFIED as to magnitude).

## 4. Search

| feature | sealbot master | CODE |
|---|---|---|
| algorithm | minimax alpha-beta (max/min branches, not negamax), full window at every node; **no PVS / null-window scan** | `engine/search.h:407-437,442-673` |
| iterative deepening | depth 1..`max_depth = 200`, stops on a mate score | `engine/bot.h:29`; `engine/search.h:157,178` |
| TT | direct-mapped 2^20 entries, upper-32-bit verify, depth-preferred replace on same key else always replace; probed and CUT at every node including the PV; phase (`moves_left`) and side in the key | `engine/bot.h:33,148-172,214-217`; `engine/search.h:455-466` |
| entry size | key u32 + depth i16 + flag i8 + score double + move 2×int64 + bool ≈ 40 B → ~40 MiB | `engine/engine_types.h:37-44` |
| ordering | TT move first, then two killers per ply, then delta order | `engine/search.h:600-620`; `engine/bot.h:177-186` |
| history | written on every cutoff, **never read** (grep: `_history` at `search.h:46,639-640,659-660`, `bot.h:175` — no read site) — SB-62 confirmed by reading | as cited |
| extensions / reductions / pruning | none: no LMR, no futility, no razoring, no null move, no aspiration, no IID, no singular. `todo.md` lists "Null move pruning" and "Aspiration windows" as TODO (UNVERIFIED intent) | `engine/search.h` whole; `../todo.md:5-6` |
| quiescence | threat-only, stand-pat, `MAX_QDEPTH = 16` (in TURNS: each recursion is one `_make_turn`), turns from the hot-window empties; opponent threats take priority over own | `engine/search.h:329-402`; `engine/constants.h:49`; `engine/movegen.h:173-221` |
| mate scores | `WIN_SCORE ∓ _ply`, `_ply` incremented once per TURN (`_ply++` after each `_make_turn`) — mate distance is in turns on master; SB-71's "plies" claim is NOT what `current/` does | `engine/search.h:420,630`; `engine/bot.h:137-146` |
| time management | deadline = now + `time_limit` set AFTER the untimed setup; `_check_time` every 1024 nodes throws `TimeUp`; the catch restores five flat arrays by `memcpy` (board 19.6 KB, wc 117.6 KB, wp 235.2 KB, cand_rc 19.6 KB + bit tables) and keeps the LAST COMPLETED depth's move — **no partial-iteration harvest on master** (`best_move` is assigned only after `_search_root` returns; SB-33's harvest is not in `current/`) | `engine/search.h:40-42,139-155,157-200`; `engine/bot.h:208-212` |
| overshoot bound | ≤ 1024 nodes of search past the deadline + the untimed setup + the rollback; SB-28 consistent | as above |
| cross-move state | killers and history cleared only when the side to move changes; TT never cleared (SB-67 consistent) | `engine/search.h:44-48` |
| determinism | no RNG on any search path (`_rng` constructed at `bot.h:32,36,68`, never used); order depends only on history and insertion order of the pool vector | `engine/bot.h:189` |
| board bound | flat 140×140, coordinates in [-70, 69], unchecked on input (SB-40 consistent; not exercised here) | `engine/constants.h:55-58` |

## 5. Per-node cost decomposition

**MEASURED** with sealbot's own fixed-depth instrument, single thread, this
workstation, `current/` built `-O3 -march=native` (`setup.py:14`), after the
arc's perf-guard bench had finished (no other engine process running):

```
cd /home/tom/Work/sealbot-scope/sealbot && .venv/bin/python benchmark.py -n 10 -d 4 --seed 42
```

| quantity | `current/` | note |
|---|---|---|
| positions | 10, seeds 42, 9–19 stones, all reach depth 4 (one reaches 3) | the tool's own random mid-game generator |
| nodes per position at depth 4 | 39 841 – 161 738 (turn-nodes: `_check_time` increments once per `_minimax`, `_quiescence` and root-move call, `bot.h:209`) | |
| wall per position | 31.9 – 167.1 ms; 92.4 ms/pos mean | single core |
| throughput | **867 575 nodes/s** (`best/`: 839 970) | a second run under `perf` read 840 164 / 819 333 |
| `best/` vs `current/` move agreement | 7/10 | the tool's own column; not a strength number |

A turn-node here is one two-stone turn; pistol's node is one ply
(`docs/research/sealbot_notes.md`), so nps is not comparable and depth 4 turns
in ~90 ms is the figure to carry (direction only).

**Profile** (`perf record -g` over the same command, user-space only,
`perf_event_paranoid = 2`; symbols are the `.so`'s own local `t` symbols —
`_move_delta`, `_filter_turns_by_threats` and `_find_instant_win` are partly
inlined into `_minimax`, so their share is a LOWER bound):

```
perf record -F 2000 -o sealbot.perf.data -- .venv/bin/python benchmark.py -n 10 -d 4 --seed 42
perf report --stdio --no-children --percent-limit 1.5     # 3 727 samples
  29.47%  opt::MinimaxBot::_minimax                (node body: pool scoring, TT, pair build, filter — inlined)
  20.96%  opt::MinimaxBot::_undo_turn
  17.97%  opt::MinimaxBot::_make
  10.94%  std::__introsort_loop<pair<double,long>…> (the per-node candidate sort, search.h:566)
   3.38%  std::__insertion_sort<…_generate_threat_turns lambda>  (quiescence pair sort, movegen.h:199)
   2.27%  _generate_threat_turns lambda (delta pairs)
   2.18%  opt::MinimaxBot::_filter_turns_by_threats
```

Read: make + undo ≈ **39 %** of the wall (the 3 × 18 array updates per stone,
§3), candidate scoring + sort ≈ 11 % visible plus an unmeasured part of the
29 % in `_minimax`, quiescence's own generation ≈ 6 %. No single term is a
hotspot; the engine is cheap per node because every per-node structure is a
flat array indexed by coordinate.

**Decomposition, CODE-derived operation counts per interior node** (labelled
ESTIMATED where the profile cannot separate them): pool scoring `|pool| × 18`
lookups (pool ≈ 2–3× the stone count at distance ≤ 2 mid-game) — the dominant
term by count; sort of `|pool|`; ≤ 56 pair constructions; must-block filter
`|turns| × |sets|`; per visited turn 2 × (make + undo) at 3 × 18 array updates
each; one TT probe. Nothing per node is O(board).

## 6. What pistol lacks, ranked by this study's own measurements

Cross-referenced to `docs/audit/search_gap_2026-09.md` (C-n rows). "Lacks" is
about MECHANISM; whether any of it is worth Elo here is an SPRT's answer.

1. **A hard per-node width** (C-2). sealbot searches ≤ 56 pairs per interior
   node from 15 cells (§1); pistol's BATCHED row emits Tier T uncapped and the
   quiet ball uncapped (`crates/pistol-search/src/staged.rs:222-235`). The
   register's colony-blindness entries (SB-58, SB-64) are the cost of doing
   this without widening — the design constraint, not an argument against.
2. **A cheap must-block pass that never widens** (C-7 context). pistol has the
   sound version (`blocking_covers`, FILTERED row); sealbot's is cheaper and
   unsound at `movegen.h:108`. Nothing to take but the cost figure in §5.
3. **Root re-ordering by last iteration's scores** (C-4 context,
   `search.h:171-177`): pistol orders the root by TT move + delta rank only.
   Cheap; a small-diff SPRT candidate on its own.
4. **Nothing else.** sealbot master has no PVS, no aspiration, no reductions,
   no extensions, a dead history table and a TT that cuts at PV nodes — on
   every one of these pistol already has the stronger mechanism. The
   Elo-bearing branches (VCF solver, NNUE) are not in the local tree and stay
   UNVERIFIED.

**What the two anchors say about the comparison, for calibration only** (D-438,
`docs/experiments/sealbot_anchor_v2_prereg.md` §10): at `go nodes 50000`
sealbot 40–0; at `go movetime 500` each side converts its own p1 seat
(20–20, 2 distinct games). Neither is a strength claim; both are direction.
