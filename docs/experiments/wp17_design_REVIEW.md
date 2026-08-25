# REVIEW-design — WP-1.7 design (killers, history, countermove on pair moves)

**Reviewed revision:** `038e458d830ca1327b9568f8419e34395de4fc8f` (`docs/experiments/wp17_design.md`).
**HEAD at review time:** `038e458d830ca1327b9568f8419e34395de4fc8f` — **matches**. Working tree clean.
**Reviewer:** fresh-context REVIEW-design subagent; did not author anything reviewed. No repository files were modified; all experiments ran under `/tmp/opencode` or read-only over the tree.

---

## 0. Premise verification (the dispatch's priority-1 checks)

### 0.1 "A `PlyOutcome::Win` placement can only arise from the forced prefix" — **SOUND**

Rows with an unforced range are exactly BATCHED and BATCHED-lost (`staged.rs:193-205`); on both,
`can_win_this_turn(us, left)` returned `None` (`staged.rs:186`). `None` requires the win-in-one-ply
window set to be empty (`query.rs:231-244`: the `OnePly` scan runs first and unconditionally), and a
single placement wins iff it fills the last empty of a live window holding exactly five own stones —
i.e. iff a win-in-one-ply window exists (`query.rs:144-147`; any ≥6 completion, overlines included,
passes through such a 6-window). So on BATCHED rows **no candidate placement wins at all**, forced or
unforced — the quiet safety net included, since it is a subset of the board, not an exemption from
`can_win_this_turn`. On WIN-NOW and FILTERED rows `forced == cells.len()`
(`staged.rs:213`, `staged.rs:256`), so the unforced range is empty by construction. The
`best_index >= forced` test therefore excludes winning placements without a second rule, exactly as
§3.2 claims. (Nit, not a finding: §3.2's parenthetical "Tier F is exactly the win-in-one-ply class"
is loose — at `StonesLeft::Two` Tier F also carries the count-four pair class, `staged.rs:227-236` —
but the conclusion never leans on that wording.)

### 0.2 "Beta cutoffs never happen at PV nodes / the root" — **FALSE as relayed; the design never states it and needs no `is_pv` gate**

- **Root:** true — `iterate` calls `visit(depth, -INFINITY, INFINITY, 0)` (`pvs.rs:151`), and every
  score is bounded inside the mate band (`search.rs:83-86` asserts `2 * MAX_DEPTH_TURNS <
  MAX_MATE_TURNS`; `INFINITY` sits above it), so `alpha >= beta` is unreachable at ply 0.
- **PV-entry nodes:** false. A node whose *entry* window is wide (`beta - alpha > 1`, `pvs.rs:242`)
  can still fail high: a non-first child is scouted at a null window (`pvs.rs:424-428`), the scout is
  fail-soft and can return `scan >= beta` (e.g. a mate score), `child` then returns it without a
  re-search (`pvs.rs:438-442`), and `visit` breaks on `alpha >= beta` (`pvs.rs:371`).

This is harmless here: the design's update condition (`best_score >= beta`, `!self.aborted`) is the
*same* condition the TT store already uses to write `Bound::Lower` (`pvs.rs:387`), and the change is
reordering-only. Recording the refuting cell of a genuine fail-high is correct killer/history
semantics whether the node's entry window was wide or not. **No finding** — but the design should not
pick up the relayed claim in prose, because it is false.

### 0.3 File:line citations — **all eight exact**

| Citation | Points at | Check |
|---|---|---|
| `pvs.rs:306` | `set.promote_table_move(table_move);` | ✓ |
| `pvs.rs:277-308` | the `CandidatePolicy::Staged` arm | ✓ |
| `pvs.rs:376-395` | the `!self.aborted` TT store block | ✓ |
| `search.rs:97-101` | `pub struct Searcher` fields | ✓ |
| `search.rs:199-201` | `Searcher::clear` | ✓ |
| `search.rs:81` | `MAX_PLY` | ✓ |
| `staged.rs:139-149` | `StagedSet::promote_table_move` boundary rule | ✓ |
| `staged.rs:340-348` | `delta_rank` | ✓ |

### 0.4 Ply-indexing claims — **SOUND**

- `GameState::played()` is an iterator over the `history` `Vec`, newest last (`state.rs:176-178`) —
  no allocation, no walk of a linked structure; cheap.
- At a `Phase::Second` node the last entry is the turn's own first stone (placed at ply `p-1`,
  `state.rs:193`), so `pair_killers[p-1] = canonical(prev, c)` reads the right `prev`, and `p >= 1`
  always because `check_root` refuses mid-turn roots (`search.rs:374`) — no underflow.
- The walk to the opponent's last stone is ≤ 2 steps at any update node (phase-Second: own first
  stone then the opponent's; phase-First: 1 step), and the "no opponent stone at all" case exists
  only at a turn-1 root, which cannot cut (0.2).
- An unstated premise the design nevertheless satisfies: ply-keyed tables are mover/phase-consistent
  *because they are reset per search* — within one search, (mover, phase) is a function of ply alone
  (the placement state machine is deterministic from the root; the turn-1 one-stone wrinkle shifts
  the parity but keeps it a function). If killers ever survived across searches with different
  roots, the keying would mix movers. Worth one sentence in §3.1 so the coupling is not lost.

### 0.5 Rule-4 validation semantics — **TRUE as stated**

`ThreatState::win_in_one_ply_windows(mover)` empty ⇒ no single placement wins for that side
(0.1's window argument). The check runs at the current node against the same incrementally-kept
`ThreatState` the search itself uses (`position.rs:187-195`), so it cannot disagree with generation.

### 0.6 Determinism (D-7) — **SOUND**

Storage is arrays + `BTreeMap` (no hasher anywhere on a choice path); the history argmax scans
`cells[front..]` left-to-right so the delta-ranked/lexicographic order breaks ties; the `begin_search`
halving sweep iterates a sorted map; no heuristic path reads a clock, a thread, or a node count —
`order_deadline` remains Radius-only (`pvs.rs:486-491`). Lifecycle verified end-to-end:
`Pistol::new_game` → `searcher.clear()` (`instance.rs:90-93`); `set_position` does **not** clear
(`instance.rs:95-101`) — the same warm-state semantics the TT already has, which is what
`replay.rs`'s warm replay relies on (one engine process per game, spawned with `newgame` via
`seats::with_seats`, then one ask per own turn; replay re-drives exactly that sequence, so the
history-halving and countermove state sequences reproduce identically). The forfeit-game exclusion
(forfeit asks are not replayed) cannot desynchronize anything because the game ends at that ask. The
extended same-process determinism test's two comparison arms (two fresh searchers; distraction →
`clear` → search) are both cold-table arms, so heuristics-ON does not break them.

### 0.7 Config/coverage completeness — **COMPLETE**

`rg -l 'kind = "staged"' configs/` returns exactly the seven files §6 lists (gate_staged_v0,
tactical_staged_v0, play_staged_v0, instrument_v0, instrument_staged_v0,
instrument_staged_q_defensive_only_v0, instrument_staged_q_defensive_and_offensive_v0); arena configs
carry no `candidate_policy` schema. The enum is `#[serde(tag = "kind", deny_unknown_fields)]`
(`config.rs:177-178`), so three non-`Option` booleans in the `Staged` variant make a missing key a
serde "missing field" error — the required-key claim holds. `Board::is_occupied` and
`Board::in_legal_region` exist (`board.rs:90`, `board.rs:169`, the latter linear in stones as §3.4
says).

### 0.8 Scope discipline — **CLEAN**

No threat-generation, eval, TT, solver, or arena changes; the two attribution-checker scripts and the
warm-replay path are untouched. The `tools/determinism.sh` seat addition is in-scope test
infrastructure (its fixture `tactical_staged_v0.txt` exists; the current SEATS list is
`tools/determinism.sh:54-57`) — note for IMPL: a change under `tools/` is reviewed against
`tools/SHELL_CHECKLIST.md` per CLAUDE.md.

### 0.9 §7 band split — **matches the fixture**

Non-comment fixture lines: 12 × `stones 15`, 1 × `stones 31`, 11 × `stones 35` → split at ≤ 17 gives
12 early / 12 late, matching the design's "two bands" and the existing tools' `EARLY_MAX=17`.

---

## 1. Findings

### BLOCKING-1 — §7's registered command block measures NOTHING and exits 0

The extraction

```
sed -n 's/^position //p' crates/pistol-cli/tests/fixtures/bench_positions_v1.txt
```

matches **zero lines**. The fixture's data lines are `position`-verb *tails*: they begin
`start moves 0,0 -1,1/1,0 … # src … stones 15` (the fixture's own header: "Same line form as
openings_v1.txt: a `position` verb tail"), and the protocol verb is `position start [moves …]`
(`protocol.rs:12`, `protocol.rs:181-186`). Reproducer (run at HEAD):

```
$ sed -n 's/^position //p' crates/pistol-cli/tests/fixtures/bench_positions_v1.txt | wc -l
0
```

so the registered loop body never executes; the whole 24 × 5 × 2 sweep prints nothing and the block
exits 0 — precisely the EXIT-0-WRONG-ANSWER class `tools/SHELL_CHECKLIST.md` exists for. This is
compounded by a second defect hiding behind the first: even with the prefix handled, the
` # src … stones N` commentary must be stripped before the line is a valid position tail —
`PositionSpec` refuses it by name, and worse, the engine *stays alive on the previous state*
(`newgame`'s empty board), so `go` still prints plausible `info totals` lines for the wrong position.
Demonstrated on a two-line stand-in of the same kind (`/tmp/opencode/wp17/stand_in.txt`, positions
authored for this review, not fixture entries):

```
$ printf 'newgame\nposition start moves 0,0 0,1/1,0 # src deadbeef stones 5\ngo nodes 2000\nquit\n' \
    | target/release/pistol --config configs/instrument_staged_v0.toml | head -1
error Protocol: expected `q,r` (in: "position start moves 0,0 0,1/1,0 # src deadbeef stones 5")
```

The correct extraction already exists in the repo the design cites: `entry="${entry%% #*}"` with
`grep -v '^#' | grep .` (`tools/staged_cover_bench.sh:118,137`, same for `tools/bench_delta.sh:339`).
With that extraction the rest of the block's shape is verified working on the stand-in: the engine
invocation, `--config`, `go nodes <n>`, and `sed -n 's/^info totals //p'` all behave (two
`info totals` lines produced, one per stand-in position).

Also folded here: **§7 records no dry run.** CLAUDE.md requires a pre-registration's literal
commands be exercised before its review passes, on same-kind input, **with a recorded criterion
naming the defect class it excludes**. §7 has no dry-run record at all — and the missing dry run is
exactly what let an extraction that produces zero measurements reach review. The registered criterion
should be external and count-based (e.g. "the dry run prints exactly one `info totals` line per
stand-in position; zero lines or any `error` line is the wrong-answer defect class"), which the
broken command fails and the corrected one passes — demonstrated above.

### MAJOR-1 — §7's verdict space is not total: the dead zone [0.80, 0.85) has no registered reading, and the IQR is printed, not gated

Registered: pass = ratio ≥ 0.85 in BOTH bands; abort = ratio < 0.80 in EITHER band. A run landing at,
say, 0.83 in one band is neither inside the bracket nor at the abort threshold, and §7 says nothing
about what it means — the accept/re-scope decision would be made after the numbers are seen, which is
what pre-registration exists to forbid. Relatedly, "IQR of the 5 per-position reps is printed with
the number it gates" registers no NOISY threshold and no consequence — but CLAUDE.md rule 5's own
words are "one change = one commit = one **IQR-gated** bench", and this repo's two prior benches both
registered a 10%-of-median IQR gate with a withheld verdict (D-215, D-362). Amend §7 to cover the
whole outcome space (e.g. 0.80–0.85 = recorded finding + operator ruling pre-registered as the
default) and to gate on IQR with a stated threshold and consequence.

### MAJOR-2 — §7 states no run COST

CLAUDE.md: "A pre-registration states what its governed run COSTS — wall time, operator attention,
machine hours — so the proportion between the document and the run is visible on the document's own
face." §7 has no cost statement. For scale (MEASURED on the stand-in, this machine, release build):
a 5 000-node search reports ~11-13 ms engine time; at 50 000 nodes the repo's own prior numbers put a
position at ~185-458 ms (D-215/D-236), so 240 invocations is roughly 1.5-4 minutes of one core plus
~240 × ~28 ms process setup (D-236's P_fixed + C). The run is cheap — which makes the missing dry run
of BLOCKING-1 inexcusable rather than understandable.

### MINOR-1 — M2's cost claim for option (b) is false as stated

"(b)/(c) need the delta scores, which live inside `staged_candidates`" — true for (c) (the tie-break
would live inside `delta_rank`, `staged.rs:340-348`) but false for (b): a *stable* re-sort of
`cells[forced..]` by history score alone, in `pvs.rs`, needs no delta scores — equal-history cells
keep the delta order by stability, with no change to `staged.rs`. The recommendation (a) still
survives on the row's other, correct failure mode ("history DOMINATEs delta, inverting the tactical
signal") — make that the stated reason and delete the false one, or the next reader will "fix" (b)'s
cost and re-litigate the call.

### MINOR-2 — M1's "(7 configs, 3 test helpers)" is an unmarked, wrong numeric claim (D-291 class)

`StagedParams {` construction sites: `instance.rs:183`, `quiescence.rs:566`, and in tests alone
`staged_differential_gate_tests.rs:126`, `staged_pattern_fixture_tests.rs:51`,
`staged_tier_t_threshold_tests.rs:96`, `staged_colony_family_tests.rs:122` and `:153`,
`staged_tests.rs:88` and `:401`, `common/mod.rs:76` — nine test sites, not three, ~11 total. Every
site must state the new `OrderingHeuristics` (rule 1 forbids a code-side default), so the churn is
~3× the stated size. Derivable in seconds by grep; per D-291 an estimate that could have been
measured in seconds is a finding. The call itself is unaffected (mechanical churn either way).

### MINOR-3 — §3.4 and §7 contradict each other on per-candidate cost (D-423 class)

§3.4: "no eval roundtrip, no per-candidate probe (D-192's finding is about per-candidate cost, and
nothing here adds one)." §7's hotspot, honestly: "one history lookup per unforced candidate." The
argmax over unforced candidates **is** a per-candidate probe — a `BTreeMap` lookup, vastly cheaper
than D-192's eval roundtrip, but a per-candidate cost the node path did not have before. §7's account
is the accurate one; §3.4's sentence should say "no per-candidate *eval roundtrip*" and stop. State
the cost once, in the section that owns it (§7), and have §3.4 point there.

### MINOR-4 — The history bonus shape is a non-obvious multi-option choice absent from the matrix and from §9's ADR list

§3.2 fixes the bonus at flat `+1` per cutoff. The chess lineage the document leans on everywhere else
(M5, M6) weights the bonus by depth precisely so a deep refutation outranks a shallow one; with a
flat bonus and argmax-only reading they rank equal. Flat-vs-depth-scaled is exactly the kind of
"more than one viable option" call CLAUDE.md routes through a matrix row and an ADR line. Low stakes
(the WP pre-registers h0 as legitimate and SPRT judges), but it should be a row or at least an ADR
line, not silent.

### MINOR-5 — The §4 DECISION-RED-TEAM waiver is unverifiable from the dispatch this review received

§4: "the dispatch names exactly REVIEW-design and REVIEW-impl as the dispatched reviews," so no
separate DECISION-RED-TEAM is held. The commissioning dispatch available to this reviewer does not
state that waiver. CLAUDE.md's default is that the option matrix is attacked by a fresh-context
DECISION-RED-TEAM *before* selection; the design has already selected. This review attacks the matrix
with a fresh context and before IMPL, which is the substance of the rule, and none of its findings
flip a call — but the operator should confirm the waiver is actually in the dispatch, or hold the
subagent, before the design is treated as matrix-attacked in the CLAUDE.md sense.

---

## 2. Verdict

**FAIL — 1 BLOCKING, 2 MAJOR, 5 MINOR.**

The mechanism itself is sound and well-grounded: the unforced-only update rule is provably
win-excluding (0.1), the ply keying is consistent (0.4), validation is correctly stronger than
membership (0.5), determinism and the warm-replay lifecycle hold (0.6), config coverage is complete
(0.7), and scope is clean (0.8). The pre-registration is what fails: the registered bench command
measures nothing and exits 0 (BLOCKING-1), and the bracket's verdict space and cost statement are
incomplete (MAJOR-1, MAJOR-2). Fix §7 — the command block, the dead zone, the IQR gate, the cost, and
a recorded dry run with an external count-based criterion — plus the four prose/matrix repairs, and
this design is landable; nothing found here reaches the mechanism.
