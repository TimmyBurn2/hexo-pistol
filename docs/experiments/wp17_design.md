# WP-1.7 — design: killers, history, countermove on pair moves

**Pinned at the commit this document lands in (see the ADR line that lands with
it).** Governing inputs: the commissioning dispatch ("[GROUNDWORK] WP-1.7"),
CLAUDE.md, `docs/ROADMAP.md` WP-1.7, `docs/research/minimax_report.md` line 83
(the ordering stack), WP-1.6's closure (D-428, `4e2193c`).

---

## 1. What this WP adds, and what it does not

Three move-ordering heuristics in `pistol-search`, each behind its own config
gate, **default OFF** in every committed config until an SPRT says otherwise:

- **killers** — per-PLY killer slots (D-9: two same-side plies per turn; the
  phase bit distinguishes them), two keyings: the completing stone (single
  cell, two slots) and the full canonical pair (D-5, D-51; one slot);
- **history** — a score per (mover, cell), incremented for the stone that
  produced a beta cutoff, aged;
- **countermove** — keyed by the opponent's last placed stone, holding the
  reply cell that refuted it.

They **REORDER the staged candidate set only**: they never add a cell (the
D-52 set is rule-complete and stage F/T per WP-1.5b is the committed policy),
never touch threat generation, the eval, or the TT, and never touch the forced
(Tier F) prefix — `StagedSet::promote_table_move`'s own boundary rule
(`crates/pistol-search/src/staged.rs:139-149`) is theirs too. Under
`CandidatePolicy::Radius` they do not run at all (§4, M1).

**Expected effect, stated honestly.** ESTIMATED, no hex measurement exists:
chess-lineage gains were measured on top of TT ordering WITHOUT threat-first
generation, while here stages F+T already order tactically (WP-1.5b, 92.2%
SPRT-proven). Redundancy is likely; the expected range is null to small
positive at equal nodes. **`h0` is a legitimate outcome and closes the WP as a
measured finding.** No ablation runs on `h0` inside this WP.

## 2. Where they enter — the exact code sites

- **Retrieval and promotion:** `crates/pistol-search/src/pvs.rs:306`
  (`set.promote_table_move(table_move);`, inside the `CandidatePolicy::Staged`
  arm at `pvs.rs:277-308`). The heuristic promotions run immediately after
  that call, on `set.cells[set.forced..]`, with the advancing front starting
  one past the TT move when the TT move was promoted.
- **Update:** the post-loop store block `crates/pistol-search/src/pvs.rs:376-395`
  (`if !self.aborted { self.table.store(...) }`). Recording happens in the same
  `!self.aborted` guard, when the loop ended on a genuine beta cutoff
  (`best_score >= beta`) whose winning candidate index was in the UNFORCED
  range (`best_index >= forced`).
- **State:** the tables live in `Searcher` (`crates/pistol-search/src/search.rs:97-101`),
  begin per search call, and are cleared by `Searcher::clear`
  (`crates/pistol-search/src/search.rs:199-201`), which is what `newgame` is.

**The stage-order position, mapped from the report's stack.** The report's
line 83 stack is `TT move → threat-making/blocking pairs → killers →
history/countermove → quiet pairs by static pattern score`. In the committed
staged scheme the "threat-making/blocking pairs" ARE the forced prefix (Tier F
plus the FILTERED cover union) and the ranked rest is Tier T / the quiet
safety net, delta-ranked by static pattern score. The mapping: the forced
prefix keeps its absolute priority (promotions never cross it), and within the
unforced range the order becomes **TT move → killer slots → pair killer →
countermove → history best → the delta-ranked rest**. The quiet safety net's
cells are ordinary unforced candidates and are promoted like any other.

## 3. The mechanism

### 3.1 Tables and lifecycle

One struct, `HeuristicTables` (new module
`crates/pistol-search/src/heuristics.rs`):

- `killers: Vec<[Option<Coord>; 2]>` — two single-cell slots per ply, indexed
  by PLY (D-9). Slot 0 is the most recent cutoff stone at that ply, slot 1 the
  one before it; a stone already in slot 0 does not shift the slots.
- `pair_killers: Vec<Option<(Coord, Coord)>>` — one canonical-pair slot per
  ply, written at the ply the pair's TURN started at (phase-First ply).
- `history: BTreeMap<(Player, Coord), i32>` — per (mover, cell). `BTreeMap`
  because D-7 forbids randomized hasher order on a choice path; nothing here
  iterates it on a choice path anyway (lookups only), and its `begin_search`
  sweep is over a sorted, hence deterministic, order.
- `countermove: BTreeMap<Coord, Coord>` — opponent's last placed stone →
  reply cell.

Both killer arrays are sized `MAX_PLY` (`crates/pistol-search/src/search.rs:81`),
the same bound the PV table is sized against.

Lifecycle:

- **`begin_search()`** — called at the top of `Searcher::search`: killer and
  pair-killer arrays are reset (ply indices restart at every search); history
  scores are HALVED, floor division, entry per entry (the aging scheme, §4
  M5); countermove is left alone — its key means the same thing throughout a
  game.
- **`clear()`** — called by `Searcher::clear` (newgame): everything to empty.

### 3.2 Update — when a beta cutoff lands

At a node that ended `best_score >= beta` with `best_index >= forced` and was
not aborted, with cutoff cell `c`, mover `m`, ply `p`:

- `killers[p]` shifts (slot 0 ← `c`, slot 1 ← old slot 0), unless `c` is
  already slot 0.
- `history[(m, c)] += 1` (saturating; magnitude is relative — history is only
  ever read as an argmax among candidates).
- `countermove[x] = c`, where `x` is the opponent of `m`'s most recent stone
  on the board (walk `GameState::played()` backwards to the first stone not
  owned by `m`; ≤ 2 steps, since at most our own first stone is newer).
- If the node was at `Phase::Second`, `pair_killers[p - 1] =
  canonical(prev, c)`, where `prev` is the stone at ply `p - 1` — this turn's
  own first stone, the last entry of `GameState::played()`. A cutoff at a
  `Phase::First` node writes no pair (the pair's second stone was never
  placed).

**Why unforced-only.** A cutoff whose cell sits in the forced prefix is a
tactical cell the tiers already front-load; recording it would let a Tier-F
cell masquerade as a quiet-refutation hint. Symmetrically, a `PlyOutcome::Win`
placement can only come from the forced prefix on every row that has an
unforced range at all (Tier F is exactly the win-in-one-ply class, and
`can_win_this_turn` being `None` on BATCHED rows excludes an immediate win
outright), so the index test excludes winning placements without a second
rule.

### 3.3 Retrieval and promotion

At a Staged node after `promote_table_move`, with `front` starting at `forced`
(+1 if the TT move sits at `cells[forced]`), these validated entries are
promoted in order, each by a stable rotate-to-front within `cells[front..]`
(the same rotation `promote_table_move` uses, so unpromoted cells keep their
delta-ranked order and their lexicographic tie-break, D-5/D-7):

1. `killers[ply][0]`, then `killers[ply][1]`;
2. `pair_killers[ply]`'s two cells in canonical order — **phase-First nodes
   only** (a pair is a turn, and a turn starts at phase First);
3. `countermove[x]`, `x` = the opponent's last stone — **phase-First nodes
   only** (mid-turn, the opponent has not just moved);
4. the unforced candidate with the highest history score for the mover, if
   that score is positive.

Each entry is promoted only if it is present in the remaining unforced range;
promotion never crosses the forced boundary and never reorders the forced
prefix. A missing entry is skipped.

### 3.4 Validation — every retrieved entry, before use

- A single cell (killer slots, countermove) is used only if the board says it
  is **empty** (`Board::is_occupied`) and **inside the rule-5 region**
  (`Board::in_legal_region`, `pistol_core::rules::LEGAL_RADIUS`).
- A pair is used only if its two cells are **distinct**, **canonical**
  (`a < b` in the derived `Coord` order, D-5), both pass the single-cell test,
  and it is **legal under rule 4**: placing the FIRST cell must not complete a
  win for the mover — checked with `ThreatState::win_in_one_ply_windows(mover)`
  (empty ⇒ no single placement wins; non-empty ⇒ membership of the first cell
  in `win_in_one_ply_cells`). A pair whose first stone wins is a turn that
  rule 4 never lets exist as a pair.

A stale entry is **skipped, never repaired**. These checks are deliberately
stronger than bare membership in the candidate set (which would already imply
them); they are the named seam the dispatch asks to be able to test, and they
keep the promotion code honest if a future retrieval context relaxes what the
candidate set guarantees. Their cost is bounded: at most six probes per node,
each linear in the stone count — no eval roundtrip, no per-candidate probe
(D-192's finding is about per-candidate cost, and nothing here adds one).

## 4. Option matrix

Every numeric claim is marked **MEASURED** or **ESTIMATED**. Per the
commissioning dispatch's subagent policy, the fresh-context attack on this
matrix is REVIEW-design itself; no separate DECISION-RED-TEAM dispatch is held
(the dispatch names exactly REVIEW-design and REVIEW-impl as the dispatched
reviews).

| # | Decision | Options | Costs / failure modes | Call |
|---|---|---|---|---|
| M1 | Where the config gates live | (a) three keys inside `[search.candidate_policy]` `staged` variant; (b) a top-level `[search.ordering]` section | (b) makes every Radius config state three keys the Radius path never reads — dead keys outside the variant, against the variant-scoped precedent `q_depth_turns`/`q_triggers` set (D-396); (a) costs churn in every `StagedParams` construction site (7 configs, 3 test helpers) — mechanical | **(a)** |
| M2 | How history orders | (a) promote the single best-history unforced candidate; (b) re-sort the unforced range by history; (c) history as a tie-break in the delta sort | (b)/(c) need the delta scores, which live inside `staged_candidates` (`staged.rs:340-348`) — touching that ranking is touching generation, out of scope; (b) also lets history DOMINATE delta, inverting the tactical signal. (a) is one map lookup per unforced candidate plus one rotation | **(a)** |
| M3 | Do pair killers earn a slot | (a) one pair slot per phase-First ply; (b) no pair killers at all | (b) loses the pair's FIRST stone (the completing stone is already covered by the single-cell killer keyed at the phase-Second ply); (a) costs one extra table and the rule-4/canonical validation seam. The report's line 83 says "adapt killers … by keying on the completing stone AND on the pair" | **(a)** — the report names both keyings; the first stone is exactly what (b) drops |
| M4 | Pair promotion shape | (a) promote each present cell of the pair, canonical order; (b) promote only when BOTH cells are candidates | (b) wastes the hint when one cell is occupied/off-set; (a) can promote a pair's second cell as a turn's first stone — a weaker hint, never a wrong move (it reorders legal candidates only) | **(a)** |
| M5 | History aging | (a) halve at `begin_search`; (b) clear at `begin_search`; (c) no aging | (b) discards cross-search signal within a game, which is where history is supposed to help; (c) unbounded growth, and stale-opening scores would dominate late-game argmax. Halving is the standard chess lineage scheme | **(a)** |
| M6 | Killer slots per ply | (a) two; (b) one | Chess lineage: two slots capture the two most recent refutations at negligible cost (two rotations) — **ESTIMATED** benefit, zero measured hex evidence, consistent with §1's honesty | **(a)** |

## 5. Determinism (CLAUDE.md rule 4, D-7)

- Storage is `BTreeMap` and plain arrays — no hasher, no randomized iteration
  on any choice path; lookups only.
- No clock, no node count, no thread is read by any heuristic path
  (`order_deadline` remains the only clock seam, Radius-only, untouched).
- All rotations are stable; equal-history candidates resolve by the argmax
  scanning `cells[front..]` left to right, so the delta-ranked (and behind
  that lexicographic) order breaks every tie.
- `newgame` clears every table: `Searcher::clear` calls `HeuristicTables::clear`.
- The cross-process gate gains a third seat (§6) so D-7 is exercised with the
  heuristics ON as well as OFF; the same-process double-run test
  (`crates/pistol-search/tests/search_determinism_tests.rs`) gains a
  staged-with-heuristics-ON arm including the distraction-then-clear sequence,
  which is what catches state bleed through the new tables.

## 6. Config and gates

- `[search.candidate_policy]` (staged variant) gains three required booleans:
  `killers`, `history`, `countermove` — schema-complete in
  `pistol-engine/src/config.rs`, mapped in `instance.rs`'s `search_policy`,
  carried by `pistol_search::StagedParams` as a new `OrderingHeuristics`
  struct (three bools, `Copy`).
- All seven existing staged configs state the three keys `false`:
  `gate_staged_v0`, `tactical_staged_v0`, `play_staged_v0`, `instrument_v0`,
  `instrument_staged_v0`, `instrument_staged_q_defensive_only_v0`,
  `instrument_staged_q_defensive_and_offensive_v0`. Radius and arena configs
  are untouched (different schema).
- New: `configs/instrument_staged_h_v0.toml` — the SPRT arm A seat, identical
  to `instrument_staged_v0.toml` except the three keys `true`. New:
  `configs/gate_staged_heuristics_v0.toml` — the determinism gate's third
  seat, all three keys `true`, added to `tools/determinism.sh`'s `SEATS`
  (fixture: the staged seat's own `tactical_staged_v0.txt`).
- `q_depth_turns = 0` everywhere in this WP's seats: quiescence stays gated
  off (D-428), and the heuristics do not run inside `crate::quiescence` at any
  `q_depth_turns` — this WP changes `pvs::visit`'s ordering only. That is a
  scope cut, recorded as an ADR line, not a claim that quiescence nodes would
  not benefit.

## 7. Rule-5 bench — pre-registered BEFORE measuring

- **Hotspot:** the heuristics' own per-node overhead — at most six validation
  probes (each linear in stones, `Board::in_legal_region`), at most six
  membership rotations over the unforced range, and one history lookup per
  unforced candidate. **No second eval roundtrip per candidate** (D-192
  measured the first one at 76% of profiled stacks; nothing here adds one).
- **Instrument:** the command block below, run at the design's own revision —
  a fixed-node sweep over `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt`
  (24 positions, two bands: centre-15 and centre-35) under
  `configs/instrument_staged_v0.toml` (OFF) and
  `configs/instrument_staged_h_v0.toml` (ON), 5 repetitions, per-band aggregate
  nps = Σ nodes / Σ median-time-per-position. IQR of the 5 per-position reps
  is printed with the number it gates.
- **Gain bracket (an OVERHEAD bracket — this change is not expected to buy
  nps):** band-aggregate nps ratio ON/OFF **≥ 0.85** in BOTH bands
  (ESTIMATED — the overhead is O(stones) probes plus map lookups per node, and
  a search at 50 000 nodes spends the overwhelmingly larger share of its time
  in eval-delta ranking and subtree recursion; no hex measurement exists).
- **Abort threshold:** ratio **< 0.80** in either band — the heuristics are
  re-scoped (cheaper validation or fewer promotions) and the numbers are
  recorded as a finding, never a threshold move.
- **Time-to-depth:** the per-position completed `depth_turns` at fixed nodes is
  printed for both sides as context — better ordering can buy DEPTH at equal
  nodes, and that is the gain channel this WP actually claims — but it is not
  a gate on this bench.

Command block (run from the repo root, release build current):

```
for CFG in configs/instrument_staged_v0.toml configs/instrument_staged_h_v0.toml; do
  while read -r POS; do
    for REP in 1 2 3 4 5; do
      printf 'newgame\nposition %s\ngo nodes 50000\nquit\n' "$POS" \
        | target/release/pistol --config "$CFG" \
        | sed -n 's/^info totals //p'
    done
  done < <(sed -n 's/^position //p' crates/pistol-cli/tests/fixtures/bench_positions_v1.txt)
done
```

Per-position stones come from the fixture's own `stones` annotations (band
split at ≤ 17 = early). The receipt quotes per-band Σnodes, Σmedian-ms, ratio,
and the IQR of each position's 5 reps.

## 8. Tests and mutation receipts

Unit/integration tests (names are the behavior):

- `a_retrieved_entry_on_an_occupied_cell_is_skipped_not_repaired`
- `a_retrieved_entry_outside_the_rule_5_region_is_skipped`
- `an_uncanonical_pair_entry_is_skipped`
- `a_pair_whose_first_stone_wins_is_skipped_under_rule_4`
- `a_cutoff_in_the_forced_prefix_updates_nothing`
- `killers_reset_history_ages_and_countermove_survives_a_new_search`
- `newgame_clears_every_heuristic_table` (the distraction-then-clear arm of
  the extended same-process determinism test)
- `history_scores_are_halved_at_each_new_search`
- `promotions_never_cross_the_forced_boundary`
- the existing staged suites (perft oracle, differential search oracle,
  staged soundness, tactical fixtures) run unchanged with the gates OFF —
  they are the no-behavior-change proof for the default.

Mutation receipts (each in a separate worktree, mutant dies):

- **validation disabled** (validator returns `true` unconditionally) → the
  four validation tests die;
- **newgame clear skipped** (`Searcher::clear` drops the
  `HeuristicTables::clear` call) → `newgame_clears_every_heuristic_table` and
  the extended double-run test die;
- **history aging removed** (`begin_search` does not halve) →
  `history_scores_are_halved_at_each_new_search` dies.

## 9. ADR lines this design records

One ADR line per non-obvious call, landing with the commits that implement
them: the gate placement (M1), the unforced-only update/promotion boundary,
the aging scheme (M5), the pair-killer slot and its rule-4/canonical
validation seam (M3/M4), the quiescence scope cut, and the bench bracket with
its receipt. The SPRT verdict line lands at closure.

## 10. Out of scope

Stage Q / widening (WP-1.5c), eval changes, perf tuning beyond the §7 bracket,
TT changes, any solver work, quiescence-node heuristics, ablations on an
`h0`, and any change to the arena, `tools/wp16_warm_attribution_check.py`,
`tools/wp15b_attribution_check.py`, or the warm-replay path — the governed
run's instruments are WP-1.6's, unchanged (D-428's `a4d5fbb` state), which is
why RED-TEAM is not dispatched.
