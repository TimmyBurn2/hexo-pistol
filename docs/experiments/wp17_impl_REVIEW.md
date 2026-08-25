# WP-1.7 REVIEW-impl

**Revision reviewed:** `6dcd27e39d6d462daf5f0388a1281f5e27bb19b3` (branch `dev`).
**HEAD matches:** YES (`git rev-parse HEAD` → `6dcd27e…`; working tree clean).
**Diff reviewed:** `git diff c227162..6dcd27e` (code + configs + tools + docs).
**Design of record:** `docs/experiments/wp17_design.md` revision 3 (`c227162`).

---

## What was verified (with receipts)

**Mechanism walk (design §2/§3 vs code).**

- *Promotion order* — `heuristics.rs:190-260` (`HeuristicTables::order_candidates`): TT move (front starts one past it when promoted, `heuristics.rs:196-200`) → killer slot 0 → slot 1 → canonical pair (phase-First only) → countermove (phase-First only) → history argmax → delta-ranked rest. Matches §3.3 exactly, including M9's countermove-before-history. The TT move keeps absolute priority: the front only advances past it, and a killer/pair cell equal to the TT move is not found by the membership scan starting at `front` and is skipped (pinned by `the_table_move_keeps_absolute_priority_over_every_heuristic`). No promotion can cross `cells[..forced]`: `promote` scans `cells[front.min(len)..]` with `front >= forced` always (pinned by `promotions_never_cross_the_forced_boundary`, including the pair-contains-the-forced-cell case).
- *Front bookkeeping safety* — `front` only increments after a found entry (`front ≤ len` invariant); `front.min(cells.len())` clamps make empty-range slicing (`WIN-NOW`/`FILTERED` rows, `forced == len`) no-ops rather than panics. No off-by-one found; no panic path found.
- *Update rule* — `record_cutoff` (`heuristics.rs:117-155`): killer shift correct in both the "already slot 0" (no shift) and "in slot 1" (moves to 0, old slot 0 to 1) cases; pair written at `pair_killers[ply - 1]` only on phase-Second cutoffs with the turn's own first stone (`last` of `played()`) as `prev`, stored canonical; countermove keyed on the opponent's last stone in BOTH phases (`First → last`, `Second → second_last` — verified against the turn structure: at phase Second the last stone is the mover's own first stone). History flat `+1` saturating; nothing decrements; `/= 2` floor on non-negative scores — no overflow/saturation issue.
- *Unforced-only gate* — `pvs.rs:414-421`: `best_index` initialised 0, updated only inside `score > best_score` (first iteration always wins vs `-INFINITY`), compared against `forced_bound` captured AFTER promotions (`set.forced` is never mutated by them, `pvs.rs:334`) — so the index and the boundary describe the same final arrangement. `PlyOutcome::Win` cannot be recorded: on WIN-NOW/FILTERED rows `forced == cells.len()` so `best_index >= forced` is unsatisfiable (`staged.rs:213,256`); on BATCHED rows `can_win_this_turn` is `None`, which excludes every win-in-one-ply cell — and any single placement forming ≥6 is by definition an empty of a count-five own window, i.e. a win-in-one-ply cell (`query.rs:145-166`) — so no unforced candidate can win at all. The design's §3.2 proof holds against the code.
- *Ply bounds* — root mid-turn positions are refused by `check_root` (`search.rs:387-392`, `TurnInProgress`), so `ply == 0` is always phase-First and `pair_killers[ply - 1]` cannot underflow. `killers[ply]`/`pair_killers[ply]` share the exact bound the PvTable already enforces (`PvTable::new(MAX_PLY)` indexed `ply + 1`); a ply that would overflow the killer arrays would already panic the pre-existing PV code.
- *Lifecycle* — `begin_search` (killers/pair-killers reset, history halved floor, countermove kept) called once in `Searcher::search` before the loop (`search.rs:230`); `Searcher::clear` → `HeuristicTables::clear` (`search.rs:205-208`), reached by `Engine::new_game` (`instance.rs:90-93`). `Run` borrows `&mut HeuristicTables` from the `Searcher` for its lifetime; no sharing or outliving path exists.
- *Borrow soundness in pvs.rs* — the second `staged_context()` re-borrow (`pvs.rs:324`) ends the first borrow cleanly; `state`/`threats` (borrowed from `self.position`) and `&self.heuristics` are disjoint fields; promotion runs BEFORE `set.cells` is consumed (`pvs.rs:323-335`) and `forced_bound` is captured before the move. No aliasing hazard.
- *Validation* — `usable_cell` (empty + rule-5 region) and `usable_pair` (distinct via `a < b`, canonical, both cells usable, rule-4 first-stone-wins via `win_in_one_ply_windows` empty ⇒ no single placement wins — the solver's window/cell queries are exactly the winning single placements, `query.rs:145-166`). Stale entries skipped, never repaired.

**Determinism (rule 4 / D-7).** No hasher anywhere (`BTreeMap` + arrays); the only iteration is `begin_search`'s aging sweep over sorted keys — not a choice path. No clock/node/thread read on any heuristic path. History argmax is strictly-`>` left-to-right over `cells[front..]`, so the delta order (and the lexicographic order behind it) breaks every tie. All rotations stable. Verified end-to-end: `tools/determinism.sh` run in full — all THREE seats pass, including the new `staged-heuristics` seat (A/B agree, C/D agree, 20 positions × 2 budgets each).

**Config/schema.** Three required booleans in the staged variant (`config.rs:223-234`), mapped in `instance.rs:178-195`, no `Default` anywhere (rule 1). Missing-key refusal proven by `a_staged_document_missing_an_ordering_heuristic_gate_is_refused` (all three keys, exact key name + "missing field"). All seven committed staged configs state the keys `false`; `instrument_staged_h_v0.toml` vs `instrument_staged_v0.toml` differ in VALUE lines only by the three keys (verified by diff — comments differ, no other value differs). `tools/config_check.sh` validates both new configs (`ok … killers: true, history: true, countermove: true`). No protocol change (`crates/pistol-cli` untouched; `identity_lines` untouched).

**Tests.** `cargo test -p pistol-search --locked`: 103 passed, 0 failed. `cargo test -p pistol-engine --locked`: all green. `cargo clippy -p pistol-search -p pistol-engine --all-targets --locked`: clean. The rule-4 fixture in `a_pair_whose_first_stone_wins_is_skipped_under_rule_4` re-verified by hand: legal game (all stones within hex-distance 8; the farthest, `(0,8)`, is distance 8 from `(0,6)`), 11 stones ⇒ P1 to move, P1 holds (0,0)–(4,0) with (5,0)/(-1,0) empty — the claimed properties hold.

**Mutation receipts (reproduced in a detached worktree at `6dcd27e`; live tree untouched).**
- Delete the `record_cutoff` call in `pvs.rs` → `a_staged_search_with_the_gates_on_records_its_cutoffs` FAILS. ✓ (liveness pin is real)
- `usable_cell` → `true` unconditionally → the two single-cell validation tests die; `usable_pair` → `true` too → all four validation tests die. ✓
- `begin_search` halving removed → `history_scores_are_halved_at_each_new_search` and `killers_reset_history_ages_and_countermove_survives_a_new_search` die. ✓
- `Searcher::clear` drops `HeuristicTables::clear` → `search_with_heuristics_on_double_run_identical` dies. ✓

**Bench receipt (§7b / D-431) recomputed from `artifacts/wp17_bench_v1.txt`** (present on disk; sha256 `e34a6931…` matches D-431 exactly). Independent re-aggregation reproduces every number: Σnodes 602112/602112 (early) and 501914/501914 (late); Σ median ms 2495/2356 and 2514/2362; band nps 241327/255565/199648/212495; **ratios 1.0590 / 1.0644**; depth histograms identical to the receipt's (early-ON has exactly one depth-4 and one fewer depth-3; late-ON loses the depth-4); node identity across reps holds for all 48 blocks (asserted in my script); band split 12/12 matches the fixture's own `stones` annotations (≤17 = early); IQR gate: ZERO violations (linear-interpolation quartiles, 10%-of-median). The WITHIN-THE-BRACKET verdict follows from the registered ≥ 0.85 in both bands.

**tools/determinism.sh vs SHELL_CHECKLIST.** The change is one seat + comments; the mechanism is unchanged. By item: (1) no new command substitutions with discarded status; (2) no new condition-position pipelines; (3) no new greps; (4) no locale-dependent guards added; (5) no git-index reads; (6) the new seat's `$WORK` transcripts are prefixed `staged-heuristics`, distinct from the `staged` seat's — no cross-seat overwrite; (7) trap unchanged, one trap; (8) no new numeric spellings; (9) no caller-controlled strings reaching records; (10) the gate produces a verdict, not a recorded number — the coverage rule's number-producing class is not implicated, and the gate is CI-driven; (11) no new deletions/overwrites beyond the pre-existing `$WORK` (mktemp-created); (12) fail/void distinction unchanged (`fail … exit 1`; environmental failures die by `set -e`). The new seat's config and fixture both exist and pass the preflight checks. **Ran the gate: all three seats pass.**

**Scope discipline.** The diff touches no code in pistol-arena, pistol-solver, pistol-eval, pistol-cli, or the TT; `quiescence.rs`'s changes are entirely inside `#[cfg(test)]` (test-harness `Run::new` arity). Threat generation, eval, TT, and the WP-1.6 instruments are untouched. File-cap rule 9: `heuristics.rs` (656 lines) carries a why-justification comment that does not state a count. ADR lines D-429/D-430/D-431 land with the commits as the design's §9 requires.

---

## Findings

### BLOCKING

None.

### MAJOR

**M-1. The unforced-only recording boundary (design §3.2, matrix M8, D-430 item 2) is pinned by nothing, and the test named for it claims a behaviour it does not exercise — its comment's claim that the behaviour is pinned elsewhere is false.**

- `crates/pistol-search/src/heuristics.rs:440-455` — `a_cutoff_in_the_forced_prefix_updates_nothing` calls `record_cutoff` DIRECTLY (bypassing the `pvs.rs` gate) and asserts the tables ARE updated. It pins "record_cutoff trusts its caller", which is the opposite of what the name claims ("a cutoff in the forced prefix updates nothing").
- Its own comment asserts: "The full-node behaviour is pinned by the integration tests: a WIN-NOW node's cutoffs update nothing, because the whole set is forced there." **Falsified by mutation**: in a detached worktree at `6dcd27e`, replacing `forced_bound.is_some_and(|forced| best_index >= forced)` with `forced_bound.is_some()` in `pvs.rs:417` (i.e. recording every non-aborted beta cutoff, forced or not — exactly what M8 exists to forbid) leaves the ENTIRE pistol-search suite green (19/19 test binaries pass, 103 tests). No integration test inspects the tables after a search whose cutoffs are forced cells; the only table-inspecting search test runs over the quiet BATCHED-only fixture.
- Why this matters and why it is not BLOCKING: the tables only reorder legal, validated candidates, so no wrong move and no determinism violation can result — this is a requirement gap (rule 7: tests are behaviour-named; §8's list promises this pin), not a wrong answer. But a silent regression of a registered ADR mechanism (Tier-F cells entering the killer/history/countermove tables — the exact "Tier-F cell masquerading as a quiet-refutation hint" M8's row names as its failure mode) would ship undetected, and a reader of the test comment would believe it defended.
- Reproducer: worktree at `6dcd27e`, the one-line `pvs.rs` mutation above, `cargo test -p pistol-search --locked` → all green. (All four pre-registered mutations DO die as D-431 claims; this unregistered one survives.)
- Fix direction: an integration-level pin — e.g. a search with the gates ON from a position whose reply rows are WIN-NOW/FILTERED (forced == all cells), asserting the killer/history/countermove entries that a forced cutoff would have written are absent — plus an honest rename/recomment of the unit test (e.g. `record_cutoff_trusts_its_caller_for_the_unforced_test`).

### MINOR

**m-1. Dangling governing-document reference in a committed config.** `configs/instrument_staged_h_v0.toml:5-6` cites `docs/experiments/wp17_sprt_prereg.md §3`; no such file exists in the tree at `6dcd27e` (`ls docs/experiments/` shows only `wp17_design.md` for this WP). The SPRT pre-registration is future work per the design; a committed seat pointing at a document that does not exist is a reference a reader cannot check — either drop the citation until the document lands or land the stub. (Verified: `ls docs/experiments/ | grep wp17` → `wp17_design.md` only.)

---

## Verdict

**FAIL — 0 BLOCKING, 1 MAJOR, 1 MINOR.** The implementation matches the design's mechanism exactly on every point walked (promotion order, update rule, validation, lifecycle, ply bounds, determinism, config surface), the bench receipt's every number recomputes from the artifact, the full three-seat determinism gate passes, and all four pre-registered mutations die. What blocks landing is M-1: the design's M8 boundary — a registered ADR decision — has no defending test, and the test whose name claims to defend it makes a false claim about what does. One small test round (an integration pin + an honest name/comment) closes it; m-1 is a one-line citation fix.
