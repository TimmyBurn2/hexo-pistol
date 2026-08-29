# MATRIX M2 — the safety-net candidate cap

**REVISION 3. THE AXIS MOVED.** Revision 2 (`f8754b8`) was attacked before
selection and **FELL** — `docs/experiments/matrix_M2_REDTEAM_round2.md`
(`27b1c3d`) — on a fact neither revision had modelled: **a turn in this game is
TWO stones and the engine chooses both inside the search**, so a field whose rows
are identified by PLY scope is not a field of the options the engine actually
has. Every row below is re-derived on the TURN axis. Neither red-team report is
edited by this revision and nothing in either is softened.

**THE FACT THAT MOVED IT**, quoted at `file:line` because D-477 now requires an
axis to be quoted like any other premise, at the line where its unit is SPENT:
`ply` increments once per STONE — `PlyOutcome::TurnContinues => self.child(
depth_plies - 1, alpha, beta, ply + 1, index == 0, true)` with `same_side = true`
(`crates/pistol-search/src/pvs.rs:397-399`) — and the answer the engine returns is
the first `Turn` of the ply-indexed principal variation,
`let pv = turns_from_plies(state, run.line()); let best = *pv.first()`
(`crates/pistol-search/src/search.rs:328-329`). **Plies 0 AND 1 are the move that
gets played.** So the scope that prunes nothing inside the played turn is
`ply > 1`, not `ply > 0`, and revision 2 selected a row that caps the second stone
of its own answer.

**WHAT THE SELECTION IS, AND ON WHOSE AUTHORITY.** `docs/decisions.md` **D-478**
is an OPERATOR RULING that re-scopes this work package: the selected option is the
turn-coherent **`ply > 1`** scope, its value case is the BOOK-OPENING class and is
to be earned at SPRT, and **the D-95 spread-class debt stays OPEN**, re-pointed at
a licensed-not-scheduled package of its own. This revision is the field that
ruling is checked against — not a record written to agree with it. Where the
measurement disagrees with the ruling's grounds it says so, and §5 states plainly
what the selection gives up.

**WHY THE D-95 OBJECTIVE IS DROPPED RATHER THAN DEFERRED QUIETLY. MEASURED**
(`artifacts/wp15d_turn_axis_v1.txt`, sha256 `5a64034e…`), `movetime 500`,
`quiet_radius 3`, K = 8, completed `depth_turns` at 11 / 21 / 51 / 99 stones, with
a breakdown of which ply each scope's cap actually bound at:

| scope | depth | bound at ply 0 | ply 1 | deeper |
|---|---|---|---|---|
| W-N incumbent | 1 / 0 / 0 / 0 | — | — | — |
| cap every ply | 4 / 4 / 3 / 3 | 4 | 0 | 157 |
| cap every ply but 0 *(rev 2's pick)* | 1 / 1 / 0 / 0 | 0 | **170** | **0** |
| cap ply 0 only | **2 / 2 / 2 / 2, COMPLETED** | 3 | 0 | 0 |
| **cap every ply but the ROOT TURN** | **1 / 0 / 0 / 0** | 0 | 0 | **0** |

**The selected row prunes NOTHING on this fixture** — zero prune events at 21, 51
and 99 stones — and completed depth is the incumbent's exactly. The class is
reachable only by pruning inside the played turn, which is what makes dropping it
a finding rather than a concession: no option protects the played turn AND pays
the debt, and D-478 chooses the protection.

**Every numeric claim is marked MEASURED or ESTIMATED (D-291).** Revision 1's and
revision 2's receipts stand and are not re-taken. This revision adds
`artifacts/wp15d_turn_axis_v1.txt` (sha256 `5a64034e…`) and
`artifacts/wp15d_turn_axis_book_v1.txt` (sha256 `43fa71ce…`), taken in a detached
worktree at `4540540` with its own `CARGO_TARGET_DIR`, the committed
revision-2 instrument re-applied with `git apply` and no rejects — which is itself
a check that the earlier artifact is complete — plus one scope value
(`4 => ply > 1`) and three ply counters. **THE MOVETIME CELLS ARE SCOPING-ONLY**
(D-22, D-478): instrument mode refuses `MovetimeMs`, so a registered calibration
or bench re-takes them under `Stop::Nodes`.

---

## 0. THE FACTS THE FIELD IS SCORED AGAINST

**F1 — the safety net is a branch, and both tiers are empty inside it.**
`crates/pistol-search/src/staged.rs:222-236`, quoted **whole** this time —
revision 1 elided the comment, which is the only place in the tree that names
the knob F12 is about (RED-TEAM M6):

```rust
    let mut tier_t = tier_t_union(threats, us, params);
    if tier_t.is_empty() {
        // The safety net. See the module doc: at the game's earliest plies no
        // window anywhere has reached a live count, so Tier T is provably
        // empty too, and this is the branch that keeps the search from
        // reporting no move at all. The same ball `candidate_cells`'s `Staged`
        // arm answers with, uncapped — `quiet_top_k` is stage Q's own knob and
        // this D-scope does not arm stage Q.
        tier_t = within_radius(board, params.quiet_radius);
        out.used_quiet_safety_net = true;
    }
    delta_rank(&mut tier_t, eval, us);
    out.cells = tier_t;
    out.forced = 0;
```

Tier F is provably empty on every row `batched` is called from (that function's
doc, `staged.rs:209-213`) and Tier T is empty by the branch condition. **No cap
placed inside this branch can exclude a cell Tier F or Tier T would have emitted
AT THAT NODE**, and that is a GUARD (the `if`), not a construction. The
node-local scope of the claim is stated here because revision 1 spent it as a
subtree claim and RED-TEAM M8 is right that it is not one: a cut cell is a move
never played, so the whole subtree under it — including a forced win three turns
later — is never searched. That is what a forward prune IS, and it is priced in
each row rather than argued away.

**F2 — CORRECTED. The per-node cost of this row is independent of the cap only
where the cap is placed after the ranking.** `candidates.rs:42-61` builds the
ball as a `BTreeSet` over `stones × ball_offsets` and filters by legality;
`staged.rs:287-295`'s `delta_rank` calls `Eval::delta` on **every cell of the
vector it is handed**, and `staged.rs:230, 233` hands it the whole ball. A
truncation placed BETWEEN those two lines removes the eval term. **MEASURED**
(RED-TEAM R0, 200 reps per cell, `staged_candidates` timed directly): a
safety-net node costs

| | width | whole | pool truncated to 8 first | ratio |
|---|---|---|---|---|
| 99 stones, r = 3 | 3 564 | 2 657.9 µs | 118.9 µs | **22.4×** |
| 51 stones, r = 3 | 1 836 | 1 202.8 µs | 43.2 µs | 27.8× |
| 21 stones, r = 3 | 756 | 434.7 µs | 13.4 µs | 32.5× |
| 99 stones, r = 2 | 1 782 | 1 414.9 µs | 119.4 µs | 11.9× |

So ~95 % of a wide safety-net node is the eval pass. **This is what makes the
pool-prefilter class a real option and not a footnote**, and revision 1 priced
that class by ESTIMATE when the measurement took 1.45 s of test time — a D-291
finding against revision 1, recorded rather than absorbed.

**F3 — the root's zone restriction and its fail-open run at ply 0 only.**
`pvs.rs:336-347`:

```rust
if let Some(zone) = self.root_restrict.as_ref().filter(|_| ply == 0) {
    let forced_intact = set.cells[..set.forced]
        .iter()
        .all(|cell| zone.binary_search(cell).is_ok());
    if forced_intact {
        let unrestricted = set.cells.clone();
        set.cells.retain(|cell| zone.binary_search(cell).is_ok());
        if set.cells.is_empty() {
            set.cells = unrestricted;
        }
    }
}
```

On a safety-net row `set.forced == 0`, so `forced_intact` is vacuously true.
**An option that caps at ply 0 makes the comment at `pvs.rs:329-335` false as
written** — "FAILS OPEN to the unrestricted set" becomes "fails open to the
capped set". **This is not an open choice, it is the correctness hole the
immediately preceding package named and fixed** (RED-TEAM M10):
`sessions/WP-1.9/2026-08-29-WP19-STOPPED.md` records it, and revision 2 of that
design excluded ply 0 under `root_restrict` with the re-review confirming the fix
"right and sufficient". `configs/gate_staged_solver_v0.toml` and
`configs/bench_wp18c_solver_on.toml` arm the solver, so the seat is not
hypothetical. Every ply-0-capping row below carries this as a cost, not as a
question.

**F4 — a fail-low widening trigger cannot bind on this recursion.** Re-verified
at HEAD: the root opens at `visit(depth_plies, -INFINITY, INFINITY, 0)`
(`pvs.rs:153`) and `best_score` starts at `-INFINITY` (`pvs.rs:368`), so
`best_score <= original_alpha` is unsatisfiable once any child returns a score;
a non-PV child opens at `(alpha, alpha + 1)` (`pvs.rs:498, 500`), so any
`score > alpha` sets `alpha >= beta` and breaks (`pvs.rs:427-429`). MEASURED by
the seed's own instrument: 0 root widenings in 101 root iterations, 0 of
2 022 904 non-PV interior nodes truncated. Independently confirmed sound by the
red team.

**F5 — the class's rate and width. MEASURED**
(`artifacts/wp19_design_census_v1.txt`):

| corpus | safety-net rows / batched rows | rate | mean width | share of all batched cells |
|---|---|---|---|---|
| `bench_positions_v1` (24 pos, 15/35 stones) | 66 / 108 662 | **0.000607** | **78.12** | 0.0015 |
| `spread_v1` (4 pos, 11/21/51/99 stones) | 2 468 / 4 015 | **0.6147** | **1 218.33** | **0.9966** |

**F6 — CORRECTED, and the correction matters more than the number.** The debt is
live at HEAD: MEASURED at `fbc8e62` through the shipped `target/release/pistol`
and `tools/bench_block.sh`, `go movetime 500` on `spread_v1` completes
`depth_turns` **1 / 0 / 0 / 0** at 11 / 21 / 51 / 99 stones under BOTH
`configs/play_v0.toml` and `configs/play_staged_v0.toml`. **But
completed-depth-0 is not an unsearched answer.** `Provenance::Fallback` is this
engine's unsearched answer (`search.rs:410-416, 434`); what these positions
return is `Provenance::PartialRoot`, and `Run::salvage`'s own doc
(`pvs.rs:174-186`) says a ply-0 promotion only ever happens on a COMPLETED child
subtree, so **the line is turn-whole and the score exact**. MEASURED this
revision (`RA/SPREAD/mt500/*/nocap`, root counters added to `pvs.rs`):

| position | root width | root children completed | what the answer is |
|---|---|---|---|
| 99 stones, r = 3 | 3 564 | **957** | the exact depth-1 argmax over 26.9 % of the root ball |
| 51 stones, r = 3 | 1 836 | 530 | over 28.9 % |
| 21 stones, r = 3 | 756 | 337 | over 44.6 % |
| 99 stones, r = 2 | 1 782 | 642 | over 36.0 % |

(RED-TEAM B3 measured 956 at the first of these; the ±1 is the deadline's own
granularity, and the two runs are independent.) **So the debt is "one turn deep
over a fraction of the root", not "no search at all"**, and any row that
narrows the root is trading 957 candidates at depth 1 against K candidates at
greater depth. Which side of that is stronger is exactly what D-124 says no
oracle answers.

**F7 — CORRECTED. What moves F6 is not "a root-inclusive cap"; it is either
removing the root's fanout or cheapening the root's children.** MEASURED,
`movetime 500`, completed `depth_turns` at 11 / 21 / 51 / 99 stones:

| row | r = 3 | r = 2 |
|---|---|---|
| W-N incumbent | 1 / 0 / 0 / 0 | 1 / 1 / 0 / 0 |
| W-K2 `except-ply0`, K = 8 | 1 / 1 / 0 / 0 | 2 / 1 / 1 / 0 |
| W-K3 `non-pv-only`, K = 8 | 1 / 1 / 0 / 0 | — |
| **W-K0 `ply0-only`, K = 8** | **2 / 2 / 2 / 2** | **2 / 2 / 2 / 2** |
| **W-K1 `every-ply`, K = 8** | **4 / 4 / 3 / 3** | 4 / 4 / 4 / 3 |
| **W-PFR recency M = 2, every ply** | — | **2 / 2 / 2 / 2** |
| **W-PFR recency M = 2, `except-ply0`** | — | **2 / 2 / 1 / 1** |
| **W-PFO ring-order M = 2, every ply** | — | **5 / 6 / 5 / 5** |

Two rows refute revision 1's headline. **W-K0** removes the root's fanout and
nothing else — MEASURED, it caps 3 of 49–76 safety-net rows per search — and
pays the debt at all four stone counts. **W-PFR at `except-ply0`** leaves the
root's set whole and still lifts 51 and 99 stones off zero, because F2's eval
term is what its children were paying. Revision 1's sentence was true of the
rows it had and false of the field.

**F8 — at the reproducible budget the same ordering holds, and nps is not the
unit. MEASURED**, `Stop::Nodes(50_000)`, `quiet_radius 2`, `spread_v1`:

| seat | depth 11/21/51/99 | nodes | wall ms (99 stones) |
|---|---|---|---|
| no cap | 1 / 1 / 1 / 1 | 58 368 / 201 728 / 1 149 952 / **4 283 392** | 10 648 |
| every ply, K = 4 | 5 / 4 / 5 / 5 | 50 176 each | **907** |
| every ply, K = 8 | 4 / 4 / 4 / 4 | 50 176 each | 869 |
| every ply but 0, K = 4 | 2 / 2 / 2 / 1 | 50 176 each | **16 441** |

The uncapped seat's node counts are D-74's own behaviour. **nps is not a
like-for-like unit across these seats** (D-374's standing lesson): 4.28 M mostly
leaf nodes against 50 k mostly interior ones. Replicated by the red team at the
committed `tt_bytes = 268 435 456` — **every seat byte-identical** to the
receipt's `1<<20` — so the table size is not carrying any of this.

**F9 — the cap is inert where the class is rare. MEASURED**,
`bench_positions_v1` at `Stop::Nodes(50_000)`, `quiet_radius 2`:

| K | Σ nodes | completed depth, 24 positions | Σ wall ms |
|---|---|---|---|
| 0 | 1 104 026 | `3 2 3 3 3 3 2 3 3 2 3 3 4 1 2 3 1 3 3 2 3 2 2 3` | 4 825 |
| 8, every ply but 0 | 1 104 026 | identical, position for position | 4 836 |
| 32, every ply but 0 | 1 104 026 | identical, position for position | 4 801 |

and it transfers to **every** scope, which revision 1 asserted and the red team
verified: at `every-ply` the corpus has **zero** safety-net rows at ply 0
(`sn_rows − sn_capped = 0`), so the scopes walk the same tree (their R2). The
corpus bench is therefore a no-regression check and never a gain channel.

**F10 — NEW, AND IT DISQUALIFIES THE CHANNEL REVISION 1 DECIDED ON.** Completed
depth on `spread_v1` is monotonically improved by narrowing, without bound, so it
ranks options by how hard they narrow. MEASURED, and the demonstration is the
whole field's own worst row: **W-PFO — the ring-order prefilter, whose key is
ascending `(q, r)` — is the DEEPEST seat measured anywhere in this document (5 /
6 / 5 / 5 at M = 2)** and it plays `-4,1/-2,1` at 11, 51 and 99 stones alike,
i.e. beside the origin regardless of where the game is. Depth cannot decide this
field. What the channel CAN carry is F6's **binary** defect — completed depth 0,
no iteration finished — and that is the only way it is read below.

**F11 — NEW. The transposition store is sound under every UNIFORM ply scope and
unsound under the non-PV one, MEASURED.** A capped node stores a `Record`
(`pvs.rs:449-467`) ranging over the capped set, and the probe (`pvs.rs:245-256`)
takes a cutoff on `!is_pv` alone; the bound is sound for a later visitor iff the
same key always emits the same set. A probe hashing set MEMBERSHIP at the point
`visit` will store over it — after the cap, after `promote_table_move`, after the
root restriction — with all three ordering heuristics ON:

| scope | observations | disagreements |
|---|---|---|
| 0 `every-ply`, 1 `except-ply0`, 3 `ply0-only` (K ∈ {8, 32}, three fixtures) | **3 329 995** | **0** |
| 2 `non-pv-only` (same) | 1 254 229 | **821** |

The reason is structural and worth stating: the stone count is a function of the
key and only increases down the tree, so ply 0's key can never recur below it,
and a scope that is a function of ply alone therefore emits one set per key. A
scope that is a function of `is_pv` does not — the same key is visited PV (full
ball) and non-PV (top-K) inside one search, and `Bound::Lower` from the superset
is not a valid lower bound for the subset. **That is `WPQ_seed.md` §7.1(3)'s
poisoned-entry class arriving exactly as it describes**, and it is the ground
that kills W-K3, replacing revision 1's weaker one. (Independently measured by
the red team at scopes 0 and 2; extended to 1 and 3 here. Revision 1's own first
attempt at this probe was VOID — an order-sensitive digest reporting every
table-move promotion as a disagreement — and the void run is kept in the receipt
so the correction is visible.)

**F12 — NEW. The knob this decision would use already exists, in twelve
committed documents, meaning the opposite thing.** `/usr/bin/grep -rn
"quiet_top_k" configs/ | LC_ALL=C sort` returns twelve documents;
`crates/pistol-engine/src/validate.rs:94-97` refuses `quiet_top_k == 0`, so **the
schema has no off-value**; `validate.rs:104-121` cross-validates
`widen_schedule` against it; and `docs/experiments/U3_tier_t.md` §10 fixes the
semantics as *"the first batch is `quiet_top_k` quiet cells; Tier F and Tier T
are always emitted whole and are not counted against it"* — **a quiet tier ADDED
beyond Tier T, the opposite direction from every row below**, which is the
finding that stopped WP-1.5c and which D-474 names by name.
`configs/instrument_staged_v0.toml:10-15` further asserts that "the cut BINDS
here". **Any selected option owes an answer**: a third knob is two schema places
for one default (hard rule 1, D-423), and re-purposing these two silently changes
what twelve committed headers assert. §5 answers it.

---

## 1. THE FIELD, ON THE TURN AXIS

A scope is named by which nodes of the PLAYED TURN it prunes, because that is the
distinction `WPQ_seed.md` §7.1 draws — *"the root and the PV, **where the move is
chosen**"* — and it is the one revision 2 could not draw while counting stones.
Ply 0 is the played turn's first stone, ply 1 its second, and everything from ply
2 down is the opponent's reply and beyond.

Eleven rows carry forward from revision 2 with their kill conditions intact:
**W-N** (no cap), **W-CFG** (shrink `quiet_radius` — cannot reach the debt at its
own floor, §2), **W-2R** (near ring unbounded), **W-W** (F4: the schedule cannot
bind on this recursion), **W-A** (a fixed K already spans 11–99 stones),
**W-PFO** (coordinate-degenerate, F10), **W-PFR** (licensed-not-selected: a
selection key that is not the eval's judgement, and its advantage is concentrated
where the SPRT cannot see it), and **W-K3** (F11: 821 membership disagreements —
`WPQ_seed.md` §7.1(3)'s poisoned-entry class, a correctness kill). None of those
is re-argued here.

**What the axis change does is re-cut the three capping rows into four**, of which
two are turn-coherent and two are not:

| row | scope | prunes the played turn's… | turn-coherent |
|---|---|---|---|
| **T-ALL** | every ply | both stones, and below | yes |
| **T-ROOT** | `ply <= 1` | both stones, and nothing below | **yes** |
| **T-BELOW** | `ply > 1` | neither stone | **yes** |
| ~~W-K2~~ | `ply > 0` | the SECOND stone only | **no** |
| ~~W-K0~~ | `ply == 0` | the FIRST stone only | **no** |

The last two are struck as options rather than killed as ideas: a scope that
prunes one stone of a two-stone move and not the other is not a choice anyone
would make on purpose, and neither was ever proposed as one — they are what a
ply-indexed field produced. Their measured cells stand and are cited below where
they price something.

### T-BELOW — cap every ply but the root turn *(`ply > 1`, SELECTED by D-478)*

- **Mechanism.** The truncation under a `ply > 1` guard, inside F1's
  `tier_t.is_empty()` branch, behind a gate that is `false` in every committed
  config.
- **Prunes.** Nothing in the move the engine returns. MEASURED, and this is a
  measurement rather than a definition: on `spread_v1` at `movetime 500`, K = 8,
  `ply0 = 0` and `ply1 = 0` on all four positions; on the book, `ply0 = 0` and
  `ply1 = 0` over 1 593 643 capped rows.
- **What it can wrongly exclude.** The opponent's live-count-two development
  cells at every node below the played turn — the only threat-shaped class a
  safety-net row can hold. MEASURED on the book at K = 8: **5 179 074 of
  7 475 936 in-pool cells, 69.3 %**.
- **Cost.** MEASURED, the book: **121 → 524 of 2 000 openings at completed depth
  ≥ 3** (K = 4: 535; K = 16: 514 — flat in K, which no other row is). MEASURED:
  **1.131× wall time** at fixed nodes (426 503 ms against 376 953). MEASURED F9
  and S2: inert on the corpus — identical node counts and identical completed
  depths position for position. MEASURED F11/S3: TT-sound, 0 membership
  disagreements in 1 280 000+ observations at this scope. MEASURED: **it does
  nothing at all on `spread_v1`** — 0 prune events at 21 / 51 / 99 stones.
- **Debt.** D-310's excised-axis debt on the SPRT's own sample. **Not** the D-95
  depth debt, which D-478 re-points elsewhere.
- **Kill.** A corpus bench outside its registered bracket, or an SPRT that reads
  `h0`.

### T-ROOT — cap the root turn only *(`ply <= 1`)*

- **Mechanism.** The truncation under `ply <= 1`.
- **Prunes.** Both stones of the played turn, and nothing below it. This is where
  D-124 says no oracle catches a mistake, and it is the package D-478 re-points
  the D-95 debt at.
- **Cost.** MEASURED, `movetime 500`, r = 3: **2 / 2 / 2 / 2, from COMPLETED
  iterations**, at every K on the grid, capping **3 rows per search**. MEASURED,
  the book at K = 8: **677 of 2 000 openings at completed depth ≥ 3**, capping
  **6 783 rows of 461 602 (1.47 %)**, at **0.770× wall time** — 23 % FASTER than
  the incumbent. Its ply split is `ply0 = 5 455, ply1 = 1 328`.
- **Kill.** Not killed. **Deferred by D-478**, and §5 states what deferring it
  costs rather than implying it costs nothing.

### T-ALL — cap every ply

- **Prunes.** Everything, the played turn included.
- **Cost.** MEASURED, the book at K = 8: **2 000 of 2 000** openings at depth
  ≥ 3, 61.5 % of count-two cells cut (79.5 % against the pool denominator), at
  1.174× wall time. MEASURED, spread: 4 / 4 / 3 / 3.
- **Kill.** It is T-ROOT ∪ T-BELOW exactly, so neither an `h1` nor an `h0` on it
  can say which half earned or lost the verdict — revision 2's §4 argument, which
  survives the axis change because the partition survives it.

---

## 2. THE RADIUS AXIS

MEASURED, `movetime 500`, completed `depth_turns` at 11 / 21 / 51 / 99 stones:

| `quiet_radius` | no cap | every ply, K = 8 | every ply, K = 32 |
|---|---|---|---|
| 1 | **2 / 1 / 1 / 0** | 4 / 4 / 4 / 4 | 3 / 3 / 3 / 3 |
| 2 | **1 / 1 / 0 / 0** | 4 / 4 / 4 / 3 | 3 / 3 / 3 / 2 |
| 3 | **1 / 0 / 0 / 0** | 4 / 4 / 3 / 3 | 3 / 3 / 2 / 1 |

**W-CFG's kill condition fires**: at `quiet_radius = 1`, the smallest value
`Searcher::new` accepts, the 99-stone position still completes zero iterations.
Mean widths at 99 stones: 596.0 (r = 1), 1 789.2 (r = 2), 3 579.3 (r = 3).

---

## 3. WHAT THE GOVERNED RUN WOULD SEE

MEASURED, `random_openings_v1.txt`, all **2 000** openings, `Stop::Nodes(50_000)`,
`quiet_radius 2`, ordering heuristics off. **This is the whole value case for the
selected row**, since D-478 drops the D-95 objective:

| row | openings at depth ≥ 3 | rows capped | count-two cells cut, pool denominator | wall ratio |
|---|---|---|---|---|
| **W-N incumbent** | **121 (6.1 %)** | 0 / 1 936 431 | 0 | 1.000 |
| **T-ROOT**, K = 8 | **677 (33.9 %)** | 6 783 / 461 602 (**1.5 %**) | — | **0.770** |
| **T-BELOW**, K = 4 | 535 (26.8 %) | 1 593 877 / 1 725 067 | 79.6 % | 1.129 |
| **T-BELOW**, K = 8 | **524 (26.2 %)** | 1 593 643 / 1 724 042 (92.4 %) | **69.3 %** | **1.131** |
| **T-BELOW**, K = 16 | 514 (25.7 %) | 1 600 062 / 1 729 079 | 57.7 % | 1.139 |
| ~~W-K2~~, K = 8 | 1 083 (54.2 %) | 611 121 / 616 627 | 73.2 % | 1.005 |
| **T-ALL**, K = 8 | **2 000 (100 %)** | 263 291 / 263 291 | 79.5 % | 1.174 |

**The divergence rate, on BOTH trajectories**, 25 games from the book, turn cap
40, the other seat asked the same question from the same position at every turn:

| row | trajectory | searches | safety-net-bearing | turn diverged |
|---|---|---|---|---|
| T-BELOW K = 8 | incumbent's moves | 631 | 120 (19.0 %) | **24 (3.80 %)** |
| T-BELOW K = 8 | the capped engine's moves | 670 | 126 (18.8 %) | **21 (3.13 %)** |

**The SPRT can see this row.** 3–4 % of a governed game's turns change, on either
trajectory. That is a statement about SENSITIVITY, not direction.

---

## 4. WHAT THE SELECTION COSTS, MEASURED AND NOT DISCOUNTED

D-478 selects T-BELOW on a principle — the prune never touches the move the
engine returns. The principle is sound and it is `WPQ_seed.md` §7.1's own. **This
section prices it, because a matrix that recorded only the reasons for a ruling
would not be a matrix.**

Against T-ROOT, the row D-478 defers, on the SPRT's own sample at the same K:

| | T-ROOT | T-BELOW *(selected)* |
|---|---|---|
| openings at depth ≥ 3 | **677** | 524 |
| rows pruned | **6 783 (1.5 %)** | 1 593 643 (92.4 %) |
| wall time at fixed nodes | **0.770×** | 1.131× |
| pays the D-95 debt | **yes, 2/2/2/2 completed** | no — 0 prune events there |
| prunes the played turn | **yes, both stones** | **no** |

**T-ROOT leads on four of five cells and the fifth is the whole reason it is not
selected.** It reaches deeper on the very sample the SPRT walks, at a
twenty-fifth of the pruning and a third less wall time, and it pays a debt the
selected row abandons. What it does is prune the node where the move is chosen,
which D-124 says no oracle catches and which the incumbent is MEASURED to be
searching properly — 337 / 530 / 957 root children completed at 21 / 51 / 99
stones, not the "unsearched top-1 play" a dispatched hypothesis described
(D-478's own correction).

**So the selection is a judgement that a 153-opening depth advantage and a 1.47×
wall-time swing are not worth buying with a prune at the choosing node.** That is
a defensible judgement and it is the operator's to make; it is recorded here as a
judgement with a price rather than as a dominance the numbers do not show.

---

## 5. RECOMMENDATION — **T-BELOW**, as D-478 rules, with its price on the record

**T-BELOW: score-ranked top-K of the quiet ball, at every ply but the root
turn's two, inside F1's `tier_t.is_empty()` branch, behind a gate that is
`false` in every committed config.**

Grounds, each a measured cell:

1. **It prunes nothing in the move the engine returns** — MEASURED, not defined:
   `ply0 = 0` and `ply1 = 0` across 1 593 643 capped rows on the book and all
   four spread positions. `WPQ_seed.md` §7.1's rule is satisfied exactly, and
   D-124 is not engaged.
2. **Its effect on the SPRT's own sample is large**: 121 → 524 of 2 000 openings
   at completed depth ≥ 3, and flat across K = 4/8/16 (535 / 524 / 514), so the
   verdict will not hinge on the calibration landing on one grid point.
3. **The SPRT can see it**: 3.80 % and 3.13 % turn divergence on the two
   trajectories.
4. **It is inert where the class is rare**: identical node counts and identical
   completed depths, position for position, on the corpus.
5. **It is sound in the transposition table**, measured at this scope: 0
   membership disagreements, against W-K3's 821.
6. **`quiet_top_k` is not re-purposed.** F12: those two keys carry
   `U3_tier_t.md` §10's committed semantics in twelve documents and have no
   off-value; the design introduces its own key with its own name and off-value
   and states in one place that the existing two remain validated-and-unread.

**What it costs, stated once and not softened**: 153 fewer openings at depth ≥ 3
than the row it is chosen over, a 1.47× wall-time swing against that row, 69.3 %
of the in-pool count-two cells excluded below the played turn, **and the D-95
depth debt left entirely unpaid** — 0 prune events on the class, completed depth
unchanged from the incumbent's 1 / 0 / 0 / 0.

**THREE THINGS THIS DOES NOT SETTLE**, listed so they are attacked as open:

- **K itself.** Registered before the calibration runs, with the DIRECTION of
  every ratio stated and the treatment of undefined positions stated;
  `sessions/WP-1.9/wp19_design_REVIEW_rev2.md` BLOCKING N1 (a)(b)(c) is the
  checklist. **F10 is the trap** — completed depth is monotone in narrowing — but
  §3's book column is nearly FLAT in K for this row (535 / 524 / 514), which is
  what makes a non-degenerate rule possible here and is stated before any rule is
  written. The wall-time column is the one that moves.
- **Whether the table move is promoted before or after the truncation.**
  `pvs.rs:328` promotes within `cells[forced..]`; which side of the truncation it
  falls on decides whether the emitted set is a pure function of the position,
  and F11's soundness measurement was taken with the promotion AFTER.
- **The gate's surface.** F12 says a new key is owed; where it lives, its
  off-value, and which of the twelve documents gain it are the design's.

---

*M2, revision 3, on the turn axis. The selection is D-478's operator ruling; this
document is the field it is checked against and the price it carries. The
DECISION-RED-TEAM attacks this revision; neither earlier red-team report is
edited by it.*
