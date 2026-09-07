# CONFIRMATION — pre-Phase-2 sweep, GROUP C (comments and documentation only)

**Revision confirmed: `90a77a8`** (`git rev-parse HEAD` → `90a77a89b90310e8fa365fb2baf050b17a52927f`).
**HEAD still matches `90a77a8`** — no new commit has landed since dispatch. The
working tree carries uncommitted changes, none of them mine and none touching
group C's files: the dispatcher's own two-ADR-line hunk to `docs/decisions.md`
(D-686, D-687, as pre-announced), plus concurrent sibling confirmation sessions'
own uncommitted edits to `docs/experiments/U3_tier_t.md` and
`tools/SHELL_CHECKLIST.md` and their own new report files
(`pre_phase2_sweep_{E,T}_CONFIRM.md`). All analysis below reads committed content
(`git show <rev>:<path>`, `git grep <rev>`) or a detached worktree checked out at
`90a77a8`, so none of that live-tree churn touches this report's findings.

**Fix-round commit read: `9ce9a7c`** ("fix(sweep): the three reviews' findings —
gate 17 was red and nothing re-ran it, five configs kept a deleted key's comment,
and a wrong-partner mutant was surviving"), diffed against the reviewed revision
`cb6e853`. Group C's cited files in that commit: `crates/pistol-cli/src/corpus/{bench,mod}.rs`,
`crates/pistol-cli/src/random_openings/error.rs`, `crates/pistol-core/src/symmetry.rs`,
`crates/pistol-solver/src/{solver,state,config,bin/solver-cost}.rs`,
`crates/pistol-search/src/position.rs`, `crates/pistol-engine/src/instance.rs`, and
`docs/experiments/pre_phase2_sweep_CLOSURE.md`. No group-C file was touched by any
commit after `9ce9a7c` (checked with `git log --oneline 9ce9a7c..90a77a8 -- <each file>`,
all empty) — only `docs/experiments/pre_phase2_sweep_CLOSURE.md` continued to change,
which is where this confirmation's one FAIL comes from.

**Worktree used: `/home/tom/pistol-wt/confirm-c`**, `git worktree add --detach
90a77a8`, removed after this report's evidence was exported (see the log list at
the end). `CARGO_TARGET_DIR` was **not** exported around any `cargo test` run in
it — the first attempt exported it and reproduced D-672's documented failure
mode (`solver_link_check_tests`, 7 of 19 red on scratch-workspace dep-info
collision) exactly as CLAUDE.md's Environment section and the closure's own §3
warn; the export was undone and the suite reran green. That failed run's log was
not kept (D-469 only requires exporting what is cited as evidence; it is
mentioned here for the record, not cited as a finding).

The Group C review's finding list (`docs/experiments/pre_phase2_sweep_C_REVIEW.md`)
has 9 findings and no BLOCKING: MAJOR-1, MAJOR-2, MAJOR-3, MINOR-1 … MINOR-6.
Every number below was re-derived independently from the tree at the revision
named — not read off the closure's corrected sentence — per the dispatch's
explicit instruction that reading the corrected sentence is the defect these
findings are about.

## Per-finding table

| id | severity | verdict | command (re-derived, not read off the doc) | quoted output | reasoning |
|---|---|---|---|---|---|
| MAJOR-1 | MAJOR | **CONFIRMED** | `git grep -h '/// # Errors' <rev> -- crates \| wc -l` at `c586837`/`cb6e853`/`90a77a8`, plus `git show f4f4a5c \| grep -c '^+.*# Errors'` | `c586837: 25`, `cb6e853: 103`, `90a77a8: 103`; diff-added count `78`. `103 − 25 = 78`, exact. | The closure's row now reads "25 `# Errors` sections at `c586837`, 103 at `cb6e853` — 78 added, none removed" — reproduces exactly, at the population the commit's own diff supports (unlike the refuted "46 without"). Unchanged since `cb6e853`, so still true at HEAD. |
| MAJOR-2 | MAJOR | **CONFIRMED** | Flipped the two `compile_fail` fences to plain fences in `crates/pistol-solver/src/state.rs` inside the worktree, `cargo test --doc -p pistol-solver --locked`, then restored (`git checkout --`, `git diff --quiet` asserted) | Third example (was line 75/76): `error[E0603]: module \`line\` is private … function \`unpack\` is not publicly re-exported`. Second example: `error[E0603]: module \`table\` is private`. First example: `ok`. | The restored sentence claims "the THIRD is one line reaching `line::unpack`, which is `pub(crate)` behind a private `mod line`, so E0603 is the only error it can produce" — measured, and it is exactly E0603, not a type error or an unresolved path. Non-vacuity restored, worded differently from the finding's suggested text but discharging the same property. |
| MAJOR-3 | MAJOR | **CONFIRMED** | `RUSTFLAGS="-W missing_docs" cargo build --workspace --locked` after `touch crates/*/src/lib.rs`, at `c586837` and `90a77a8`, in the worktree, no `CARGO_TARGET_DIR` export | `c586837`: `1 function, 1 method, 108 module, 23 struct field`. `90a77a8`: `1 function, 1 method, 109 module, 23 struct field` — **no `associated constant` line at either revision boundary I compare against the baseline; the +1 seen at `cb6e853` is gone.** | At `cb6e853` this same build gave `109 struct field` (+6) and `1 associated constant` (+1) over the `c586837` baseline — the seven MAJOR-3 items. At `90a77a8` both deltas are back to baseline (struct field 23=23, no assoc-const line): all seven now have a `///`. Spot-read all seven (`BenchPosition::position`, `CorpusError::path`, `RandomOpeningsError::{RadiusPastCeiling::ceiling, CountPastCeiling::ceiling, Write::path}`, `Symmetry::IDENTITY`, `SolveResult::outcome`) — each carries a substantive, non-restating doc line. The +1 module (`pistol_search::params`) is the reviewer's own noted non-finding (A-11's by-design uniformity), still present and still not a finding. |
| MINOR-1 | MINOR | **CONFIRMED** | `sed -n '75,79p' crates/pistol-search/src/position.rs`; `sed -n '100p' docs/decisions.md`; `git grep -n "D-42[^0-9]" -- crates` | `position.rs`: `/// deciding what is true asks here.` (no `D-42` citation). `docs/decisions.md:100`: D-42 is entirely about `GameState::from_plies` replaying a move list. The four remaining `D-42` hits in the tree are fixtures/tests citing D-42 for exactly that replay claim, not the authority-caches claim. | The citation removed from `position.rs::state()` was borrowed from a different file's deleted line and asserted a proposition D-42 does not carry. It is now gone and not replaced by another miscitation. |
| MINOR-2 | MINOR | **FAIL** | Independent re-implementation of the audit's Detector A (maximal runs of plain `//` line comments ≥ 8, `///`/`//!` excluded, a bare `//` breaks the run) over `crates/*/src/**/*.rs`, run at `d83ac01`, `c586837`, `cb6e853`, `90a77a8` (script: `c_minor2_detectorA_script.py`) | `d83ac01: 16` (matches the audit's own published number and its six named `file:line(length)` pairs exactly — calibrated), `c586837: 17`, `cb6e853: 4`, **`90a77a8: 6`**, new blocks at `crates/pistol-search/src/census.rs:166` (8 lines) and `crates/pistol-search/src/quiescence.rs:418` (9 lines). | The closure's row (`docs/experiments/pre_phase2_sweep_CLOSURE.md:619`) says "17 at `c586837` under the AUDIT's OWN detector, **4 now**" — the detector-attribution fix (MINOR-2's actual ask) is right and reproduces at `cb6e853`, but the number is stale AT THE GOVERNING REVISION: re-deriving under the *same, correctly-identified* detector at `90a77a8` gives **6**, not 4. `census.rs`'s new block was added in `9ce9a7c` itself (bundled group-E `site` parameter comment, landed in the same commit as this fix round); `quiescence.rs`'s was added by the later commit `0fedfa8`. Per the dispatch's rule, a later commit reopening a finding's property is a FAIL — the row's own re-derivation instruction ("the AUDIT's OWN detector") now produces a different number than the row states. |
| MINOR-3 | MINOR | **CONFIRMED** | `git grep -n "Instant::now" 90a77a8 -- crates/pistol-engine/src` | Exactly one hit: `crates/pistol-engine/src/instance.rs:304`, now preceded by `// THE ONLY CLOCK THIS CRATE READS, and instrument mode refuses to be given a budget that needs one (docs/decisions.md D-22, D-73). Nothing else here holds nondeterministic state (CLAUDE.md rule 4).` | The deleted crate-wide `# Determinism` block's claim ("reads a clock in exactly one place … holds no other nondeterministic state") is now stated at the one call site it is about, and mechanically there is only one `Instant::now()` call in the crate to falsify it. |
| MINOR-4 | MINOR | **CONFIRMED** | `git show <rev>:crates/pistol-solver/src/bin/solver-cost.rs \| grep -c '^//!'` at `cb6e853` and `90a77a8` | `cb6e853: 16`, `90a77a8: 9`. | Matches the closure's own stated "carried 16; it is cut to 9 in the same spirit" exactly, closing the one root the standard had not reached. |
| MINOR-5 | MINOR | **CONFIRMED** | Audit's own pattern `git grep -nE '[^ ][[:space:]]+// [A-Za-z]' <rev> \| grep -E '^crates/[^/]+/src/'`, bucketed by file, at `c586837` (pre-fix) and `90a77a8` | `c586837`: `zone.rs` 30, `tt.rs` 1, `cover.rs` 4, `pistol-search/src/lib.rs` 1 — **total 36 = 31 (test) + 4 (doc table) + 1 (non-test)**. `90a77a8`: `zone.rs` 30, `tt.rs` 1, `cover.rs` 4 — **35, zero non-test hits**, `lib.rs`'s hit gone (moved above the item). | The closure's corrected paragraph now reads "31 are inside `#[cfg(test)] mod tests` … 31 + 4 + 1 = 36, which is the total" — reproduces exactly at the pre-fix revision the paragraph is describing, correcting the "30 + 4 + 1 = 35" arithmetic error the finding named. The post-fix state (0 non-test hits) is unchanged since `cb6e853` and still holds at HEAD. |
| MINOR-6 | MINOR | **CONFIRMED** | `sed -n '85,116p' crates/pistol-solver/src/config.rs` | `# Errors` section: `` [`SolverConfigError::Epsilon`], [`SolverConfigError::ZoneOrders`], [`SolverConfigError::FreeStoneRadius`] or [`SolverConfigError::TtEntries`], whichever refuses first. `` Body returns all four constructors by name (`Epsilon`, `ZoneOrders`, `FreeStoneRadius`, `TtEntries`). | All four variants the body can return are now named in the doc; previously only two of four were named with a hand-wave for the rest. |

## Machine checks re-run at `90a77a8` (worktree, no `CARGO_TARGET_DIR` export)

| check | result |
|---|---|
| `cargo fmt --all --check` | clean, exit 0 (`c_fmt.log`) |
| `cargo clippy --workspace --all-targets --locked -- -D clippy::all -D warnings` | clean, exit 0 (`c_clippy.log`) |
| `cargo doc --workspace --no-deps` (all `lib.rs` touched) | 0 warnings, exit 0 (`c_cargo_doc.log`) |
| `cargo test --workspace --locked` | 184 suite result lines, **1160 passed, 0 failed**, exit 0 (`c_test.log`) |
| `cargo test --workspace --doc --locked` | 10 doctests, all pass including the four group-C `compile_fail`/moved doctests (`c_doctest.log`) |

## NEW FINDINGS (introduced within `9ce9a7c..90a77a8`, group-C-relevant)

### NEW-1 — MINOR — dangling cross-reference in the closure's A-13 row

`docs/experiments/pre_phase2_sweep_CLOSURE.md:619` reads: `A-13 | 16 blocks of
≥ 8 consecutive \`//\` | **17 at \`c586837\` under the AUDIT's OWN detector, 4
now** — see the detector note below`. No "detector note" exists anywhere below
it, or anywhere in the document.

Reproducer: `git show 9ce9a7c -- docs/experiments/pre_phase2_sweep_CLOSURE.md`
shows the elaborating paragraph ("A-13's '16 → 19, grown' mixed a real change
with a DETECTOR SWAP … `d83ac01` → 16 … `c586837` → 17 … `cb6e853` → 4") was
added by the fix round; `git show a2bfdbf -- docs/experiments/pre_phase2_sweep_CLOSURE.md`
shows it deleted two commits later with no successor; `git grep -n "detector
note" docs/experiments/pre_phase2_sweep_CLOSURE.md` at `90a77a8` returns nothing.
Log: `c_new1_dangling_ref.log`.

This is independent of MINOR-2's numeric staleness above (the row's *number* is
wrong; separately, its *promise of elaboration* points at nothing). Comment-only
severity: it misleads a reader into looking for a justification that was written
and then deleted, but asserts nothing false on its own.

## Group verdict

**FAIL** — on MINOR-2 alone. Eight of nine findings (MAJOR-1, MAJOR-2, MAJOR-3,
MINOR-1, MINOR-3, MINOR-4, MINOR-5, MINOR-6) discharge their property at `90a77a8`
and were each re-derived independently rather than read off the corrected
sentence. MINOR-2's *diagnosis and remedy* (attribute the count to the audit's
own detector) are correct and reproduce exactly at the revision the fix round
closed (`cb6e853`), but two later comment edits — one bundled into the fix-round
commit itself (`census.rs`, `9ce9a7c`), one from a subsequent commit
(`quiescence.rs`, `0fedfa8`) — pushed the true count from 4 to 6 under the exact
detector the row now (correctly) says to use, and the row was never re-run. None
of the nine findings is in D-424's never-overruled class (no wrong-answer path in
code), and NEW-1 is a MINOR documentation defect, not a code defect.

## Exported logs (`artifacts/pre_phase2_confirm/`, gitignored, `c_` prefix)

- `c_fmt.log`, `c_clippy.log`, `c_cargo_doc.log`, `c_test.log`, `c_doctest.log`
- `c_compile_fail_flip.log` (MAJOR-2 reproducer)
- `c_missing_docs.log` (MAJOR-3 reproducer)
- `c_major1_errors_count.log` (MAJOR-1 reproducer)
- `c_major2_sentence.log` (MAJOR-2 restored text, read)
- `c_minor1_d42.log` (MINOR-1 reproducer)
- `c_minor2_detector.log`, `c_minor2_detectorA_reproduction.log`, `c_minor2_detectorA_script.py` (MINOR-2 reproducer — the FAIL)
- `c_minor3_clock.log` (MINOR-3 reproducer)
- `c_minor4_bin_root.log` (MINOR-4 reproducer)
- `c_a15_hits_c586837.txt`, `c_a15_hits_90a77a8.txt` (MINOR-5 reproducer)
- `c_minor6_errors.log` (MINOR-6, read)
- `c_new1_dangling_ref.log` (NEW-1 reproducer)

Worktree `/home/tom/pistol-wt/confirm-c` removed after export.
