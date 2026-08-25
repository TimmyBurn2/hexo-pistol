# WP-1.7 — design: killers, history, countermove on pair moves

**Revision 3.** It closes the DECISION-RED-TEAM's wounds on the §4 matrix
(attacked at `232a175` after REVIEW-design's findings were closed in revision
2): M1's site list is re-categorised (one production site, nine test sites)
and its citation corrected; M2 gains the omitted top-K option, the
late-in-stack cap honesty and a by-construction mark; M3/M4 gain the
play-order / first-stone-credit options the dispatch's own keying and credit
contract rule out, recorded as the strongest surviving attacks; M5 and M7 now
state the thin-residue composite honestly; M8 (update/promotion boundary) and
M9 (countermove-before-history order) are added as the rows the red team
found missing; §3.1's countermove exemption carries its true reason. All
seven standing calls from revision 2 are unchanged.

**Revision 2.** It closes all eight findings of the design review
(`038e458`, 1 BLOCKING / 2 MAJOR / 5 MINOR): §7's command block was rewritten
(the registered extraction matched ZERO fixture lines — the wrong-answer class
the dry-run rule exists for) and gained a recorded dry run, a total verdict
space, an IQR gate and a cost statement; §4's M1/M2 cost claims were corrected
to measured ones and M7 added; §3.4 now points at §7 for cost instead of
contradicting it. Sections not named here are revision 1's, unchanged.

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

**Why the killer tables are reset per search** — a premise the review asked to
have stated rather than left implicit: within ONE search, (mover, phase) is a
function of the ply index alone (the placement state machine is deterministic
from the root; turn 1's single stone shifts the parity but keeps it a
function), so a ply-keyed table cannot mix movers. Across searches with
different roots the function changes, which is why `begin_search` resets the
ply-keyed tables rather than carrying them.

Lifecycle:

- **`begin_search()`** — called at the top of `Searcher::search`: killer and
  pair-killer arrays are reset (ply indices restart at every search); history
  scores are HALVED, floor division, entry per entry (the aging scheme, §4
  M5); countermove is left alone — not because its key is stabler than
  history's (both keys mean the same thing throughout a game), but because
  countermove is a single-slot LAST-WRITE-WINS table that overwrites its own
  stale entries, while history is an accumulator that cannot; the residual
  cost — an opening countermove read at full authority in the endgame — is
  the lineage-standard price of that shape, accepted here.
- **`clear()`** — called by `Searcher::clear` (newgame): everything to empty.

### 3.2 Update — when a beta cutoff lands

At a node that ended `best_score >= beta` with `best_index >= forced` and was
not aborted, with cutoff cell `c`, mover `m`, ply `p`:

- `killers[p]` shifts (slot 0 ← `c`, slot 1 ← old slot 0), unless `c` is
  already slot 0.
- `history[(m, c)] += 1` (saturating; the bonus is FLAT — see §4 M7 for why
  not depth-scaled — and magnitude is relative, since history is only ever
  read as an argmax among candidates).
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
unforced range at all (Tier F carries the win-in-one-ply class — plus, at
`StonesLeft::Two`, the count-four pair class — and `can_win_this_turn` being
`None` on BATCHED rows excludes an immediate win
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
each linear in the stone count — no per-candidate EVAL roundtrip; the one
per-candidate cost this WP does add (a single `BTreeMap` lookup per unforced
candidate, for the history argmax) is stated and bounded in §7, which is the
section that owns cost.

## 4. Option matrix

Every numeric claim is marked **MEASURED** or **ESTIMATED** (claims that are
true by construction of the mechanism are marked as such). The matrix has
been attacked twice by fresh contexts: REVIEW-design (`038e458`, whose
findings revision 2 closed) and a DECISION-RED-TEAM against the amended
matrix (`232a175`) — **no recommendation died; every row survived with
wounds, and the strongest surviving attack on each standing call is recorded
in §9's ADR note** per CLAUDE.md's own rule.

| # | Decision | Options | Costs / failure modes | Call |
|---|---|---|---|---|
| M1 | Where the config gates live | (a) three keys inside `[search.candidate_policy]` `staged` variant; (b) a top-level `[search.ordering]` section | (b) makes every Radius config state three keys the Radius path never reads — dead keys outside the variant, against the variant-scoping precedent the committed schema itself sets (`config.rs:189-223`'s `q_depth_turns`/`q_triggers`); (a) costs churn in every `StagedParams` construction site — **MEASURED by grep, 10 sites: one production site (`instance.rs:183`) and nine test sites** (`quiescence.rs:566` is inside its `#[cfg(test)]` module, plus the eight integration-test sites) — all mechanical, rule 1 forbids a code-side default at each | **(a)** |
| M2 | How history orders | (a) promote the single best-history unforced candidate; (b) stable re-sort of the unforced range by history score; (c) history as a tie-break in the delta sort; (d) top-K history promotions, K = 2-3 | (b) needs no delta scores (a stable sort by history alone preserves the delta order among equal-history cells) but lets history DOMINATE delta, inverting the tactical signal the tiers exist to front-load; (c) needs the delta scores, which live inside `staged_candidates` (`staged.rs:340-348`) and are discarded before `pvs::visit` holds the set — touching that ranking is touching generation, out of scope; (d) is the natural first relaxation and costs K-1 extra rotations, but every K beyond one is a tuning axis with no hex evidence. (a) is one map lookup per unforced candidate (by construction) plus one rotation, and caps history's influence at one cell — a cap that lands LATE in the promotion stack (§3.3: behind TT, killers, pair, countermove), which is stated here rather than left for a reader to derive | **(a)**; (d) is licensed-not-scheduled if the SPRT reads null |
| M3 | What earns the pair's first stone a slot | (a) one canonical-pair killer slot per phase-First ply; (b) no pair killers; (c) credit the turn's FIRST stone in history at a phase-Second cutoff | (b) loses the pair's first stone entirely; (c) would recover it through the history table with no new seam, BUT it credits a stone that did NOT produce the cutoff — against the dispatch's own contract ("score per (mover, cell) for the stone that produced a beta cutoff, bonus on cutoff") — and the dispatch names the pair keying itself ("full canonical pair, D-5, D-51"). The report's line 83 says "adapt killers … by keying on the completing stone AND on the pair" | **(a)** — the dispatch's keying; (c) is the strongest surviving attack on it and is recorded, not adopted |
| M4 | Pair promotion shape | (a) promote each present cell of the canonical pair, in canonical order; (b) promote only when BOTH cells are candidates; (c) store and promote in PLAY order | (b) wastes the hint when one cell is occupied/off-set; (c) composes with the phase-Second single-cell killer to reconstruct the whole refuting pair and is free at write time — but the dispatch names the keying "full canonical pair" (D-5, D-51's `make_turn` semantics), so (c) is out of bounds for this WP; canonical order can promote the pair's second cell as a turn's first stone, and inverts play order even when both cells are present — a weaker hint, never a wrong move (it reorders legal candidates only, and the rule-4 check excludes the first-stone-wins class) | **(a)**, with (c) recorded as the strongest surviving attack |
| M5 | History aging | (a) halve at `begin_search`; (b) clear at `begin_search`; (c) no aging | (b) discards cross-search signal; (c) unbounded growth, and stale-opening scores would dominate late-game argmax. **The honest composite, stated rather than oversold**: with floor halving and a flat `+1` (M7), a single-cutoff cell halves to 0 at the next search — history's cross-search memory is a THIN RESIDUE available only to repeatedly-cutting cells, so (a) sits close to (b) in effect, and the choice between them is not load-bearing. Halving is kept as the lineage-standard middle point | **(a)**, with the thin-residue reading on its face |
| M6 | Killer slots per ply | (a) two; (b) one | Chess lineage: two slots capture the two most recent refutations at negligible cost (two rotations) — **ESTIMATED** benefit, zero measured hex evidence, consistent with §1's honesty | **(a)** |
| M7 | History bonus shape | (a) flat `+1` per cutoff; (b) depth-scaled (`+= depth_plies` or `+= depth²`) | (b) is the chess-lineage default and lets deep refutations outrank shallow ones WITHIN a search; across searches it also extends history's memory horizon (a ~50-point bonus survives ~6 agings where a flat 1 dies at the first), which is a memory knob wearing a ranking knob's clothes — an unlicensed tuning axis. Flat +1 keeps history's cross-search footprint at exactly the thin residue M5 admits, and its ties fall back to the delta order, which is weakly safer under M2(a) | **(a)**, recorded as an ADR line; depth-scaled bonuses are licensed-not-scheduled for a future WP that has a reason to expect them to matter |
| M8 | The update/promotion boundary | (a) unforced-only: only cutoffs whose cell sits at or after `forced` update the tables, and promotions never touch `cells[..forced]`; (b) all cutoffs update, promotions may rotate the whole set | (b) lets Tier-F cells masquerade as quiet-refutation hints and would disturb the deterministic internal order of the forced prefix (Tier F's ascending `(q, r)`), changing committed behaviour of forced-cell ordering for no strength claim; (a)'s index test provably excludes winning placements (§3.2) | **(a)** |
| M9 | Order within the report's "history/countermove" tier | (a) countermove promoted before the history cell; (b) history before countermove | The report's line 83 treats them as one tier, so the split is this design's to make: countermove is the MORE SPECIFIC key (the exact opponent stone just placed), so it goes first; the reverse order would let the blunter key displace the sharper one whenever both hit | **(a)** |

**The composite, owned once here rather than row by row:** M2(a) + M5(a) +
M7(a) deliberately select the weakest viable member of each of history's
axes — one cell of influence (landing late in the stack), a thin cross-search
residue, frequency-only signal — on top of a delta-ranked candidate set that
§1 already expects to make the heuristics largely redundant. That is not
timidity, it is the registered expectation: the SPRT judges the trio, and if
it reads null the licensed-not-scheduled relaxations (top-K promotions,
depth-scaled bonuses) are the natural next experiments, each carrying its own
pre-registration.

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
  membership rotations over the unforced range, and one history `BTreeMap`
  lookup per unforced candidate. **No second eval roundtrip per candidate**
  (D-192 measured the first one at 76% of profiled stacks; nothing here adds
  one).
- **Instrument:** the command block below, run at this document's own
  revision — a fixed-node sweep over `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt`
  (24 positions, two bands: ≤ 17 stones = early, else late, the fixture's own
  `EARLY_MAX` convention) under `configs/instrument_staged_v0.toml` (OFF) and
  `configs/instrument_staged_h_v0.toml` (ON), 5 repetitions, per-band aggregate
  nps = Σ nodes / Σ median-time-per-position.
- **Verdict space — TOTAL, so no reading is chosen after the numbers.**
  - band-aggregate nps ratio ON/OFF **≥ 0.85 in BOTH bands**: within the
    bracket — the overhead is accepted and the WP proceeds to SPRT. (The
    bracket's expected band is ≥ 0.85, **ESTIMATED**: the overhead is
    O(stones) probes plus map lookups per node against a search whose time is
    dominated by eval-delta ranking and subtree recursion; no hex measurement
    exists.)
  - ratio **< 0.85 in EITHER band**: OUTSIDE the bracket — the heuristics are
    re-scoped (cheaper validation, fewer promotions) and the numbers are
    recorded as a finding, never a threshold move.
- **IQR gate (rule 5's "IQR-gated bench", the D-215/D-362 convention):** for
  each position, the IQR of its 5 per-rep times must be ≤ 10% of that
  position's median time. A position exceeding it WITHHOLDS the verdict —
  that position is re-measured (all 5 reps, both configs) before any ratio is
  read, and the re-measurement is recorded beside the first.
- **Time-to-depth:** the per-position completed `depth_turns` at fixed nodes is
  printed for both sides as context — better ordering can buy DEPTH at equal
  nodes, and that is the gain channel this WP actually claims — but it is not
  a gate on this bench.
- **Cost, stated on the document's own face:** 24 positions × 5 reps × 2
  configs = 240 sequential engine invocations. **MEASURED on the §7a dry
  run's stand-in (5 and 7 stones): 136-241 ms per 50 000-node search**; the
  fixture's 15- and 35-stone positions are heavier (D-215's radius-2 figures
  ran 185-458 ms at this budget), so **ESTIMATED 3-8 minutes wall on one
  core**, plus ~240 × ~30 ms process startup (D-236's fixed per-invocation
  cost). Operator attention: one launch and one read.

Command block (run from the repo root, release build current; the extraction
is the established idiom `tools/staged_cover_bench.sh:118,137` uses — entries
are the fixture's non-comment lines, the ` # …` commentary stripped per entry,
the band read from the `stones` annotation):

```
for CFG in configs/instrument_staged_v0.toml configs/instrument_staged_h_v0.toml; do
  while IFS= read -r entry; do
    position="${entry%% #*}"
    for REP in 1 2 3 4 5; do
      printf 'newgame\nposition %s\ngo nodes 50000\nquit\n' "$position" \
        | target/release/pistol --config "$CFG" \
        | sed -n 's/^info totals //p'
    done
  done < <(grep -v '^#' crates/pistol-cli/tests/fixtures/bench_positions_v1.txt | grep .)
done
```

### 7a. The dry run — recorded, with its criterion

CLAUDE.md: a pre-registration's literal commands are exercised before its
review passes, on an input of the SAME KIND, never on the registered workload.
**DONE, this session.**

- **Input:** `/tmp/opencode/wp17/stand_in.txt` — two positions in the
  fixture's own line form (`position start moves … # src … stones N`, 5 and 7
  stones), authored for the dry run, not fixture entries. The two configs are
  `instrument_staged_v0.toml` and
  `instrument_staged_q_defensive_only_v0.toml` — the heuristics-ON config
  does not exist yet at design time; what the command block's SYNTAX depends
  on is two instrument configs, and those two are the same kind differing in
  identity.
- **Criterion, and the defect class it excludes:** for each config, the
  command block must print **exactly one `info totals` line per stand-in
  position and zero `error` lines**. The defect class is
  **EXIT-0-WRONG-ANSWER — a sweep that measures nothing and still exits 0**:
  the extraction this section's revision 1 registered
  (`sed -n 's/^position //p'`) matches ZERO fixture lines (fixture data lines
  are position-verb TAILS, not `position`-prefixed lines), so the registered
  loop body never ran — verified: `sed -n 's/^position //p' … | wc -l` prints
  `0`. An extraction failure therefore shows up as ZERO totals lines, which
  this criterion falsifies and a plausible-magnitude check would not. (The
  count is an externally derived referent in the sense that matters here: it
  is derived from the INPUT's own line count, not from anything the command
  under test computes.)
- **Output recorded:** 2 positions × 2 configs → 4 `info totals` lines, zero
  `error` lines, per-position searches of 136-241 ms — the full transcript is
  preserved at `/tmp/opencode/wp17/dryrun_output.txt` and its first lines are:
  `depth_turns 3 seldepth 4 nodes 50176 nps 362895 time 138 …` (config 1,
  position 1) and `depth_turns 3 seldepth 5 nodes 50176 nps 207377 time 240 …`
  (config 2, position 1). The dry run is not a governed sample and consumes
  nothing.

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
them: the gate placement (M1), the unforced-only update/promotion boundary
(M8), the aging scheme and its honest thin-residue reading (M5), the
pair-killer slot with its rule-4/canonical validation seam (M3/M4), the flat
history bonus (M7), the countermove-before-history order (M9), the
quiescence scope cut, and the bench bracket with its receipt. Per CLAUDE.md's
matrix rule, each ADR line records the STRONGEST SURVIVING ATTACK from the
matrix's fresh-context red team: for M3/M4 that attack is the play-order /
first-stone-credit alternative the dispatch's own keying and credit contract
rule out; for M5 it is that floor-halving with a flat bonus preserves almost
none of the cross-search memory its row originally claimed; for M2 it is the
deliberately minimal composite. The SPRT verdict line lands at closure.

## 10. Out of scope

Stage Q / widening (WP-1.5c), eval changes, perf tuning beyond the §7 bracket,
TT changes, any solver work, quiescence-node heuristics, ablations on an
`h0`, and any change to the arena, `tools/wp16_warm_attribution_check.py`,
`tools/wp15b_attribution_check.py`, or the warm-replay path — the governed
run's instruments are WP-1.6's, unchanged (D-428's `a4d5fbb` state), which is
why RED-TEAM is not dispatched.
