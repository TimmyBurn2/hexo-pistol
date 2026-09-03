# WP-2.1 lever B — the label cache. MUTATION RECEIPT (D-568's shape).

**Design**: `docs/experiments/wp21_label_cache_design.md` §6, revision 8.
**Revisions**: the full set at **`9c4366c`**; M12 and M13 re-taken at **`610225b`**
after the finding below; every other row is unchanged text and unchanged code
between the two (`git diff 9c4366c 610225b -- crates/pistol-arena/src` is empty;
the diff is one test assertion and the design's row 4 / §6 wording).
**Where**: the detached worktree `/home/tom/pistol-wt/mutation` on `/home`, its own
`target/`, never the live tree; driver `scratch_mutate.py` (untracked, kept in the
worktree), each mutant applied by exact string replacement, the named row run,
the tree restored with `git checkout -- .` before the next.
**Toolchain**: rustc 1.98.0 (88d9e12ae 2026-08-18).
**Logs** (gitignored, digests here): `artifacts/arc3r_mutation_9c4366c.txt`
sha256 `95914dce001308cdf41f61e993717a416a1b256e8c6cba68df41f9fd3535172e`; `artifacts/arc3r_mutation_610225b_X1.txt` sha256
`29784d1bb9af6e4859818e22caf6bd5b217ecb1b85e9aae1fda62096d3d8407c`.

## The call sites, enumerated at `9c4366c`

```
git grep -n 'memo\.\(lookup\|insert\|asked\|recorded\|into_counts\)\|Memo::new\|refuse_census_under_cache\|unsolicited()\|capture_tail(\|MIN_SAMPLED\|stray_armed\|key_pos_collisions +=\|key_full_collisions +=\|counts\.line(' -- crates/pistol-arena/src tools/cold_label_check.py | LC_ALL=C sort
```

| site | where |
|---|---|
| X1, the CLI arm | `crates/pistol-arena/src/usage.rs:111` (`capture_tail`), called at `bin/arena.rs:61` |
| X1b | `capture.rs:322` (defined), `:347` (the first statement of `run`) |
| the memo | `capture.rs:356` (`Memo::new`), `:370` (lookup), `:373` (`asked`), `:384` (insert), `:396` (`recorded`), `:400` (`into_counts`) |
| X3, the hoisted guard | `capture.rs:363`; the play pass's own is `exchange.rs:34` |
| the two counters | `label_cache.rs:132`, `:135` |
| the counts line | `passes.rs:89` |
| the stray | `bin/stub_engine.rs:438`, `:447`, `:583`, `:599` |
| the floor | `tools/cold_label_check.py:85`, `:283`, `:286` |

## The set, and what each died at

Baseline before any mutant: `label_cache_tests` 9 of 9, T8 1 of 1.

| # | design row | mutant as applied | row run | at `9c4366c` | at `610225b` |
|---|---|---|---|---|---|
| M01 | 2c lookup removed | `LabelCache::On => None` | T2 | DIED | — |
| M02 | 2c lookup inverted | a miss takes the first stored pair | T1 | DIED | — |
| M03 | 2c keyed on the stone list | lookup/insert on the sorted stones' `Debug` | T5 | DIED | — |
| M04 | 2c keyed on the canonical form | the same on `canonical_form` | T5 | DIED | — |
| M05 | 2c keyed on the canonical form | the same | T1 | DIED | — |
| M06 | 2d insert removed | the call deleted | T2 | DIED | — |
| M07 | 2d insert stores the raw pair | `(totals, bestmove)` before `normalise` | T1 | DIED | — |
| M08 | 2, mode consulted nowhere | `Memo::new(LabelCache::Off)` always | T2 | DIED | — |
| M09 | `asks` reports `records` | `asks: records` in `into_counts` | T2 (cached arm) | DIED | — |
| M10 | `asks` derived unconditionally | `asks: answers.len()` | T2 (uncached arm) | DIED | — |
| M11 | `asks` derived only when `On` | `asks: if On { answers.len() } else { asks }` | whole suite | **SURVIVED — EQUIVALENT, as the design labels it** | — |
| M12 | X1, the arm removed | the or-pattern arm replaced by an unspellable one | T3 | **SURVIVED** (F-2.1) | **DIED** |
| M13 | X1, one alternative removed | `["--label-cache", "--census"]` dropped | T3 | **SURVIVED** (F-2.1) | **DIED** |
| M14 | X1b removed | the call replaced by `let _ = …` | T3b | DIED | DIED (re-run beside M12/M13) |
| M15 | X3 the guard removed | `if let Some(stray) = None` | T4 | DIED | — |
| M16 | X3 left inside `ask`, not hoisted | the guard restored in `ask`, removed from the loop | T4 | DIED | — |
| M17 | `key_pos_collisions` removed | `+= 0` | T5 | DIED | — |
| M18 | `key_full_collisions` removed | `+= 0` | T5 | DIED | — |
| M19 | counters inverted | `if self.key_pos.insert(…)` without `!` | T5 | DIED | — |
| M20 | row 6, the stray never written | `stray_armed |= … && false` | T4 | DIED | — |
| M21 | row 7, the floor set to zero | `MIN_SAMPLED = 0` | T8 | DIED | — |

**Twenty of twenty non-equivalent mutants die at the row the design names; the
one labelled EQUIVALENT survives the whole suite, as labelled.**

## F-2.1 — why M12 and M13 survived at `9c4366c`, and what changed

The usage catch-all refuses with its own sentence and then appends `USAGE`, which
names every word the program has — `--label-cache` and `--census` included. T3
searched the WHOLE of stderr for both words, so the catch-all satisfied it and
the X1 arm was indistinguishable from its absence. The design's *"names neither
word"* was true of the catch-all's own sentence and false of the process's
stderr. **The mechanism is untouched**: T3 now reads the refusal's first line
(`arena: <why>`), which under the arm names both words and under the catch-all
names neither. Design revision 8 corrects row 4 and §6's two X1 rows to say so.
Recorded in `arc3_ledger.md` F-2.1.

## What the receipt does not claim

- M11's equivalence is a claim about THIS suite: no row distinguishes an `asks`
  derived from the memo under `On` from the counter, because under a live cache
  they are equal. A dead cache is what would separate them, and no test has one.
- T4's cached arm is the row with the design's stated residual (§3, D-593); M15,
  M16 and M20 died deterministically because under each the cached run performs
  no channel read in game 1 and exits 0, which is the arm that has no residual.
