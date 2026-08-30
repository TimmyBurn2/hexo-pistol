# WP-1.9 — CLOSURE HANDOFF (tracked copy)

**Why this file is tracked.** The operator-facing summary lives at
`sessions/WP-1.9/2026-08-30-WP19-CLOSED.md`, and `/sessions/` is gitignored
(`.gitignore:25`) — so a successor pinned to a revision cannot resolve it.
That is the same defect this package hit with its own dispatch, which the
stage-Q package had archived to gitignored `sessions/` for a reviewer who
then could not read it (see `wp19_storage_DISPATCH.md`). This copy is
byte-identical below the header and lives where a review can cite it.

---


> **Audience: the next agent.** This file is written to be worked from, not just
> read. Every claim names the artifact or command that establishes it, so a
> successor verifies rather than trusts. Section 9 is the environment hazards
> this session actually hit — read it before running anything.

## 0. ONE LINE FOR THE MORNING

**The eval's window map is now a hashed store and the engine is ~1.5x faster
whole-engine with its output proven byte-for-byte unchanged; it missed its own
registered bracket of [1.60, 2.10] at 1.508/1.579, the bracket was NOT moved, and
the cause is measured rather than guessed — the module split costs 0.844/0.828,
worth ~18% and recoverable in a small follow-up for the price of one rule-9
entry. All 19 gates green at `3c9e28b`, tree clean, nothing running.**

---

## 1. Orientation — what changed, and where

`crates/pistol-eval` only. Three files. No search, no solver, no config, no
committed document that shipped behaviour.

| File | What it holds now |
|---|---|
| `crates/pistol-eval/src/window_map.rs` | **NEW.** The packed key, the hasher, the container, and their six in-source invariant guards. |
| `crates/pistol-eval/src/handcrafted.rs` | Unchanged in shape. Three call sites now go through `WindowMap`; the `windows` field and `delta`'s equivalence doc were retargeted. |
| `crates/pistol-eval/src/lib.rs` | `mod window_map;` plus a corrected Determinism paragraph — it used to claim "no hasher: the window bookkeeping is a `BTreeMap`". |

A fourth doc site was corrected in `crates/pistol-core/src/window.rs` (its
`Window` doc asserted the bookkeeping lived "in an ordered map with no hasher").
**REVIEW-impl found that one; the design had named only two.**

### The API, which is the whole surface

`WindowMap<V>` exposes exactly the operations the eval performs, and no more:

```
get(window) -> V                                  // delta:  one probe, read-only
entry_or_default(window) -> &mut V                // apply:  one probe, insert-or-update
update(window, edit) -> Option<(V, V)>            // undo:   one probe, edit-and-maybe-remove
len() / capacity()                                // #[cfg(test)] only
```

**`update` exists for a measured reason.** A `get` then a `set` answers the same
thing and hashes twice; on take-back that is `WINDOWS_PER_CELL` = 18 extra probes
for every stone a search unwinds. The first implementation did exactly that and
both the rule-5 bench and REVIEW-impl caught it independently. **Do not
reintroduce a `set`.**

`Counts::is_empty` was deleted: `update` owns the emptied-entry rule now, and an
absent window and an emptied one are ONE observation to the map. That equivalence
is what makes `undo`'s desync check still fire on exactly the window it used to.

---

## 2. Revisions and tags — reproduce, do not re-measure

Every measured revision is anchored by a tag. `git show <tag>:<path>` works from
the main repo; the worktrees they were built in are gone.

| Tag | What it is |
|---|---|
| `wp19/mx-base` (`a5c5661`) | Pre-implementation engine. Baseline for every matrix number. |
| `wp19/mx-O1` (`abf3d5d`) | Packed `u64` key, `BTreeMap` retained — **the option D-225 recommended first**. |
| `wp19/mx-O2` (`9a986c6`) | The selected shape, written INLINE in `handcrafted.rs`. **Faster than what shipped.** |
| `wp19/mx-O4` (`22bbd96`) | Dense direct-addressed grid. Fails the lattice-edge test by construction. |
| `wp19/mx-O1-prefmt`, `wp19/mx-O2-prefmt` | Superseded, un-`rustfmt`ed. Do not cite. |

Shipped: `07f518b` (implementation) -> `16c6b70` (fix round) -> `3c9e28b` (closure).
`723758b` is the last docs-only commit before the implementation and is
engine-identical to `a5c5661` — both build a `pistol` with sha256 `8dc2f922…`,
verified, which is why either serves as a baseline.

---

## 3. The verdicts, each with its command

### Track E — proven, and no SPRT owed (D-495)

- **Agreement:** `eval_incremental_matches_from_scratch_on_random_playouts`
  (`crates/pistol-eval/tests/eval_incremental_tests.rs:29`). Random legal
  sequences coloured through `GameState` (never `i % 2`, per D-499), turn-1
  single stones and rule-4 truncation included, checked against
  `value_from_scratch` forward AND on the way back. **It already existed** — the
  design's first revision wrongly registered it as new.
- **Byte-identity: IDENTICAL, twice** — at `07f518b` and again at `16c6b70` after
  `undo` was rewritten. 44 positions (`tactical_staged_v0.txt` 20 +
  `bench_positions_v1.txt` 24), both determinism budgets (`depth_turns 4`,
  `nodes 200000`), **422 output lines, 88 bestmoves, 0 error lines, digest
  `0b1cb805…` in every run including the baseline's.** Only `nps` and `time` are
  elided, both wall-clock.
  Artifacts: `artifacts/wp19_byte_identity_v1.txt`, `_v2.txt`.

### The rule-5 bench — BELOW BRACKET, and the bracket did not move (D-374)

Registered at `562c8eb` **before** the run (`wp19_storage_bench_prereg.md`):
hotspot D-192's H1, bracket **[1.60, 2.10]** both bands, abort below 1.00.

| run | what | early | late |
|---|---|---|---|
| 1 (`wp19_bench_shipped_v1.txt`) | shipped, two-probe `undo` | 1.505 | 1.555 |
| 2 (`wp19_bench_shipped_v2.txt`) | shipped, one-probe `undo` | **1.508** | **1.579** |
| 3 (`wp19_bench_inline_vs_module_v1.txt`) | inline vs module, diagnostic | 0.844 | 0.828 |

Run 2 is the closing number. Above the abort line, so a **FINDING**, exactly as
the registration said a miss would read.

**Run 3's `VERDICT ABORT` wording does not apply** — it is a diagnostic between
two candidates, not a change proposed against a baseline, and nothing was
reverted on it. It is quoted because the instrument's own output is what gets
cited (D-327).

### Mutation receipts — six mutants, all dead, three SINGLETONS

`artifacts/wp19_mutations_v2.txt`, re-run at the shipped revision. The three
singletons are the valuable ones: each kills exactly one test, which is what
proves that test is the unique pin.

| mutation | kills |
|---|---|
| `undo` leaves the emptied entry behind | `eval_apply_undo_roundtrip` + 4 others |
| `apply` skips one axis | `eval_incremental_matches_from_scratch_on_random_playouts` + many |
| **swap `q`/`r` in the key** | **only** `a_packed_key_orders_windows_the_way_the_window_type_does` |
| drop the `as u16` narrowing | `a_packed_key_never_collides_for_two_distinct_windows` + many |
| **reseed the hasher** | **only** `the_window_hasher_answers_a_fixed_digest_for_a_fixed_key` |
| **`shrink_to_fit` on removal** | **only** `the_window_map_holds_its_peak_capacity_after_every_entry_is_gone` |

The reseed singleton matters most: **a reseeded hasher moves no search output**,
so `tools/determinism.sh` cannot see it. The golden digest is its only pin. An
earlier "two fresh hashers agree" test would have caught nothing — it passes for
a hasher seeded once per process.

### CI

`tools/ci.sh` at `3c9e28b`: **19/19**, all nineteen `gate N/19:` lines present,
final line `ci: all gates passed`, and
`/usr/bin/grep -cE "^ci: FAIL|^ci: RUN VOID|test result: FAILED"` returns **0**.
Artifact `artifacts/wp19_ci_closure_final_v2.txt`.

---

## 4. Invariants and the test that pins each

| # | Invariant | Pinned by |
|---|---|---|
| I1 | `window_key` is **injective** (the operative property — a collision merges two windows silently) | `a_packed_key_never_collides_for_two_distinct_windows` |
| I2 | `window_key` is order-preserving (not relied on; implies I1; kept for reuse) | `a_packed_key_orders_windows_the_way_the_window_type_does` |
| I3 | Equality is **canonical** — order- and history-independent | `eval_apply_undo_roundtrip`, `delta_leaves_the_eval_indistinguishable` |
| I4 | The hasher is **seedless by construction** | `the_window_hasher_answers_a_fixed_digest_for_a_fixed_key`, `..._refuses_a_key_that_is_not_a_u64` |
| I5 | An emptied window leaves no entry behind | `an_emptied_window_leaves_no_entry_behind` |
| I6 | The footprint is bounded by **peak**, not live entries | `the_window_map_holds_its_peak_capacity_after_every_entry_is_gone` |
| I7 | A game boundary leaves the eval indistinguishable from fresh | `new_game_forgets_the_position_and_everything_learned` (`pistol-engine`) |

**I7 has no mechanism of its own — do not go looking for one.** `Searcher::clear()`
(`crates/pistol-search/src/search.rs:194-203`) never touches the `Position` that
owns the eval. What empties it is `Position::reset_to`
(`crates/pistol-search/src/position.rs:55-70`), which **unwinds** at the head of
every search. So I7 is a consequence of I3 and I5. The design's first revision
claimed `newgame` clears the map; it does not, and REVIEW-design caught it.

**Why that matters beyond bookkeeping:** because the eval is unwound rather than
rebuilt, cross-position correctness rests *directly* on canonical equality. That
is why D-498 is load-bearing and not ceremony.

---

## 5. Two hypotheses were falsified before the cause was found

Recorded because the process is what caught them.

1. **The pre-registration's own hypothesis — that the module split was free — is
   false.** Written before the run, refuted by it.
2. **Run 1 blamed an extra hash probe in `undo`.** Real, found independently by
   REVIEW-impl, and fixed — but removing it moved the ratio by **0.003 early and
   0.024 late**, so it explained nothing.

Only then was the cause measured: inline against module is **0.844 / 0.828**.
The three runs compose to within 0.002 — `1.783 x 0.844 = 1.505` against a
measured 1.505, `1.909 x 0.828 = 1.581` against a measured 1.579 — which is the
check that they measure one thing.

**The module version shipped anyway, and the reason is process, not preference:**
it is the version REVIEW-impl reviewed, every mutant was re-run against, and
Track E proved twice. Restructuring at closure would ship code no reviewer had
seen.

---

## 6. Debts leaving this package — named, costed, with procedures

### 6.1 O-3, the registered flip trigger (D-501) — HIGHEST VALUE

The hand-rolled open-addressing probing table **D-225 actually named**, and the
only row in the field never implemented. The selection is PROVISIONAL on it.

**Terms are fixed and may not move (D-374):**
- Implement a probing table over the same packed key (~120-160 lines + tests).
- `tools/bench_delta.sh rev:a5c5661 rev:<O-3> 5`, config `configs/instrument_v0.toml`.
- Comparand: **O-2 at 1.783 / 1.909** (`wp19/mx-O2`), which does not move.
- **Flips only if O-3 exceeds those in BOTH bands** by more than the within-run
  IQR. One band is a finding, not a flip.
- Never run: the debt stays open. Silence does not discharge it.

### 6.2 The module-split cost

Inlining the storage back into `handcrafted.rs` buys **1.18x-1.21x**. It costs
~15 lines over rule 9's soft cap plus one justification entry — and rule 9's cap
is soft precisely so a measured reason can override it. **This needs its own
REVIEW-impl and its own bench**; it is not an unreviewed edit.

Note 6.1 and 6.2 interact: if O-3 wins, it should land inline from the start.

### 6.3 Stage-1 residue carried by name (D-496, D-503)

- **WP-1.4** — D-95 movetime ceiling. On HeXO the server owns the clock and
  hard-clamps the call, so this is a **forfeit risk**, not a limitation.
- **WP-1.10** — `tools/` gate hardening: five undriven gate scripts, the
  `command -v` sweep (16 sites / 11 files), the `mktemp -d` preflight debt. Its
  precedence over WP-1.9 was displaced BY NAME, not silently.

---

## 7. Where everything lives

| Document | What it is |
|---|---|
| `docs/experiments/wp19_storage_DISPATCH.md` | The operator dispatch, **archived to a tracked path** so its requirements are citable. |
| `docs/experiments/wp19_storage_scope_memo.md` | The premise gate. Four conflicts that produced D-496..D-499. |
| `docs/experiments/matrix_wp19_storage.md` | The option matrix, revision 2. |
| `..._REDTEAM.md`, `..._REDTEAM_rev2.md` | Both DECISION-RED-TEAM rounds. Both FAIL. |
| `docs/experiments/matrix_wp19_storage_selection.md` | The selection, its one ground, and the flip trigger. |
| `docs/experiments/wp19_storage_design.md` | Design revision 2. |
| `docs/experiments/wp19_storage_design_REVIEW.md` | REVIEW-design. FAIL, 6 MAJOR. |
| `docs/experiments/wp19_storage_impl_REVIEW.md` | REVIEW-impl. FAIL, 3 MAJOR, Track E confirmed independently. |
| `docs/experiments/wp19_storage_bench_prereg.md` | The bracket, registered before the run. |
| `docs/experiments/wp19_storage_bench_results.md` | All three runs and the falsifications. |

**`docs/experiments/wp19_design.md` is NOT this package** — it is the stage-Q
design that D-473 re-designated WP-1.5c. Do not read it as WP-1.9's.

ADR lines: **D-494..D-503**. D-502 closes the package, D-503 closes Stage 1.

---

## 8. The road here, since it was not straight

Six gates rejected work before anything landed, and **every rejection reproduced
when checked against the tree**:

| gate | outcome |
|---|---|
| Premise memo | **STOP** — 4 conflicts, incl. D-225's licence clause being false at HEAD |
| Matrix rev 1 | **FAIL** — 3 BLOCKING, incl. a requirement quoted that existed nowhere in the tree |
| Matrix rev 2 | **FAIL** — 1 BLOCKING; cap fired, package returned to the architect |
| Design rev 1 | **FAIL** — 6 MAJOR; three tests it named as new already existed |
| REVIEW-impl | **FAIL** — 3 MAJOR, no correctness defect |
| CI at closure | **FAIL** — gate 17, rule 9: `window_map.rs` over the cap, unregistered |

---

## 9. Environment hazards this session actually hit — READ BEFORE RUNNING

1. **Concurrent `cargo` on the shared target dir corrupts a workspace run.** A
   `cargo test --workspace` racing a `cargo build -p …` produced
   `error[E0463]: can't find crate for pistol_engine` in a doctest. It is not a
   real failure. Run workspace tests with nothing else building.
2. **`tools/file_justification_check.sh` reads TRACKED bytes, not the working
   tree** (D-233). A rule-9 entry you have not `git add`ed does not exist to it.
3. **The harness `grep` is order-nondeterministic** (D-265). Use `/usr/bin/grep`
   or `git grep` for anything recorded — and note that `until grep -q …` poller
   loops built on it **can fail to exit**, leaving you waiting on work that
   finished. Check `ps` before assuming something is still running.
4. **`/tmp` is a 24 GiB tmpfs.** Worktrees and build dirs go under `/home`.
5. **`cargo fmt` before quoting any line count.** Two candidate revisions were
   benched and counted before formatting; the true counts were 277 and 315.
6. **Removing a worktree can orphan its commits.** Tag them first — that is why
   `wp19/mx-*` exists.

---

## 10. State at closure

- On `dev` at `3c9e28b`. **Tree clean, one worktree (the repo), zero detached
  processes.**
- Engine diff vs `wp19/mx-base`: 3 files in `pistol-eval` (+1 doc line in
  `pistol-core`), 352 insertions, 43 deletions.
- Artifacts exported with digests: `artifacts/wp19_CLOSURE_export_receipt_v1.txt`
  (13 entries) and `artifacts/wp19_rt_export_receipt_v1.txt` (a removed
  reviewer worktree's logs).
- **Next per D-471 as amended by D-494: the Stage-3 scoped detector** — cheap
  VCDT/TSS detection gating solver calls, targeting the measured ~6x call-count
  cut the WP-1.8c bracket demands, DBS decomposition only after the detector
  earns its own SPRT.
