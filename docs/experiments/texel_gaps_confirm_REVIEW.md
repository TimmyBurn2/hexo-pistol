# tools/texel instrument gaps — SCOPED CONFIRMATION of the REVIEW-impl fix round

**Pinned revision**: `f73539e575b27599f65ba3fc88be283c2576b0f2`.
**Does it match HEAD?** YES — `git rev-parse HEAD` at the start of this session returned
`f73539e575b27599f65ba3fc88be283c2576b0f2`, and the working tree was clean.

I did not implement this package or its fix round, and I did not write the review I am
confirming. No file was edited except this report. All mutation and re-derivation work ran
in confirmer-owned worktrees under `/home/tom/confirm-wt/` (`texel-confirm` at
`f73539e`, `at-6b85681` at `6b85681`), each removed at the end of the session; neither held
a gitignored `artifacts/` or `sessions/` beyond one throwaway probe file I created and
deleted myself. I did not touch `/home/tom/pistol-wt/texel-gaps`. Two throwaway commits
were made **inside my own disposable worktree only** (`texel-confirm`), to get past a
newly-discovered baseline defect (see BLOCKING-1 below) so the other 20 findings could be
exercised; both were reverted with `git reset --hard f73539e...` before I finished, and
that worktree no longer exists.

## Commands I ran (representative; scope stated inline in each section)

- `python3 tools/texel/test_texel.py` — the shipped suite, run directly in a fresh
  `git worktree add --detach` checkout of `f73539e` (not `tools/texel_tests.sh`, and not
  the pistol-wt CI worktree).
- `cp /home/tom/Projects/HeXO-AlphaBeta/artifacts/texel_gaps/mutants_texel.py` (read-only,
  from the live tree's gitignored `artifacts/`, since that file is not committed) into my
  own worktree's root and ran it there with its own preflight and restore.
- Direct source mutations via `sed`/Python string-replace on files inside my own worktree,
  reverted with `git checkout --` after each, to build the reviewer's own reproducers and
  new probes (case-spelling, padded-allowlist, moved-mutation-site, staged-mutant-restore).
- `git archive <rev> -- docs/ | tar -x` into a scratch directory, then plain
  `/usr/bin/grep -rn`/`-rl` — a method sharing nothing with `git grep`, for the D-665/D-669
  re-derivations.
- `python3 -c` combinatorial triple enumeration (fresh script, not awk) for D-666's
  1140/525.
- `tools/config_check.sh`, `tools/governing_citation_check.sh` run directly in the worktree
  to check gate state.

---

## VERDICT: **FAIL**

Nineteen of the twenty-one findings are genuinely, property-level DISCHARGED — this is a
strong fix round on the merits. But the pinned revision itself does not pass its own
instrument suite in a clean checkout, D-669 (one of the six correction lines this very fix
round added) contains a self-contradicting count, and BLOCKING-1's own disposition line
makes a forward-reference that is false at this revision — the identical defect class
BLOCKING-1 was opened to fix, recreated in the text that claims to have fixed it. Failing
items: **BLOCKING-1** (partial), **MAJOR-9** (a real residual escape), **MAJOR-10** (an
incomplete obligation list), **MINOR-9 / D-669** (a wrong file count), and one **NEW
DEFECT** not on the original 21: gate 18 is RED at `f73539e` in an untouched checkout.

---

## NEW DEFECT — gate 18 is RED at the pinned revision, in a clean checkout

**This is the headline finding.** `f73539e` is the commit that "lands" the fix round and
adds `docs/experiments/texel_gaps_impl_REVIEW.md` (803 lines, new file) to the tree. That
review document is itself a legitimate RECORD that names the three retired candidate
basenames dozens of times (its own reproducers quote `eval_v0_quiet_fit_weights`,
`instrument_quiet_fit_v0`, `arena_wp22_phase1_quiet_dryrun` verbatim, e.g. lines 74, 296,
375-389, 452-454, 474-478). MAJOR-9's fix added a second, list-based check —
`stray = [f for f in naming if f not in RETIRED_MAY_NAME]` — that fails on ANY tracked file
naming a retired candidate unless the file is one of six hardcoded paths. `RETIRED_MAY_NAME`
lists `wp22_phase1_impl_REVIEW.md` (an older, unrelated review) but was never updated to add
`texel_gaps_impl_REVIEW.md` — the new review being landed in this very commit.

**Reproducer**, in a fresh `git worktree add --detach <path> f73539e575b27599f65ba3fc88be283c2576b0f2`:
```
$ python3 tools/texel/test_texel.py 2>&1 | tail -3
test_texel: 3 FAILURE(S): ['and only RECORDS name eval_v0_quiet_fit_weights',
  'and only RECORDS name instrument_quiet_fit_v0',
  'and only RECORDS name arena_wp22_phase1_quiet_dryrun']
$ echo $?
1
$ bash tools/texel_tests.sh; echo GATE18_EXIT=$?
... (same failures) ...
GATE18_EXIT=1
```
Detail: `FAIL and only RECORDS name eval_v0_quiet_fit_weights ['docs/experiments/texel_gaps_impl_REVIEW.md']`
— the check is correctly, not vacuously, catching an unlisted reference; the allowlist is
simply stale by one commit.

**Confirmed this is new, not pre-existing**: a `git worktree add --detach` at `6b85681`
(the code commit, one before `f73539e`, before the review file existed) runs the identical
suite fully GREEN:
```
$ python3 tools/texel/test_texel.py 2>&1 | tail -2
  ok   a classes > keys run is refused by name

test_texel: all checks passed (including census_classes)
$ echo $?
0
```
So `6b85681` → GREEN, `f73539e` → RED, and the only tree change between them affecting this
check is the addition of the un-allowlisted review file. Gates 6 and 21 (`config_check.sh`,
`governing_citation_check.sh`) both remain GREEN at `f73539e`, and no `.rs` file changed
anywhere in `8e7fdf0..f73539e` (verified by `git diff --name-only`), so I did not re-run the
full `cargo test --workspace` — nothing in this diff could touch it.

**Nowhere is this recorded as owed.** `docs/decisions.md` D-665..D-670 and
`texel_gaps_CLOSURE.md` never mention `texel_gaps_impl_REVIEW.md` needing an allowlist
entry. This is not a deferred, judged trade-off; it is an oversight, and it means the
revision this confirmation is pinned to — the revision the package is meant to close on —
fails its own gate 18 (`tools/ci.sh` step 18/21) in a clean checkout. Per CLAUDE.md,
*"A WP is not landable while its reviews are outstanding"*; a landed revision that cannot
pass its own instrument gate is not landable either. **Minimal fix** (not applied by me):
add `"docs/experiments/texel_gaps_impl_REVIEW.md"` to `RETIRED_MAY_NAME` in
`tools/texel/test_texel.py`.

---

## Table: all 21 findings

| finding | verdict | command / mutant that decided it |
|---|---|---|
| **BLOCKING-1** | **MOVED** | §2-§6 do now exist (774 lines, up from 286) with per-item state and a mutation-evidence section — the missing-sections half of the finding is real progress. But the disposition table's own text for this finding says *"gate output quoted in §7"*; `/usr/bin/grep -n '^## §'` on the closure at `f73539e` shows the document ends at `§6` — **no §7 exists**. No gate's raw command+output is quoted anywhere in the closure outside the mutation harness transcript (§3) and the pre-package `beb91b1` CI reference (§0); gates 3/6/18/21's individual states are asserted in prose, not cited from a log. This is the identical "forward-references a section that isn't there" shape BLOCKING-1 was opened to fix, recreated in the very sentence closing it. |
| MAJOR-1 | **DISCHARGED** | `/usr/bin/grep -v '^#' tools/texel/fixtures/fit_rows_v1.txt \| awk -F'\t' '{print NF}' \| sort -nu` → **20**; same on `artifacts/texel_gaps/rows_real.txt` (live tree, read-only) → **20**. Fixture and extractor agree. |
| MAJOR-2 | **DISCHARGED** | Mutant `games.add((index, game))` → `games.add(game)` in `extract.py`, suite re-run: `FAIL and REPORTS the distinct game count... extract: 6 row(s) written from 2 distinct game(s)` — dies exactly as claimed. |
| MAJOR-3 | **DISCHARGED (the crux finding — verified hardest)** | Mutant: `_free_directions`'s `"both"` case `[[1.0,-1.0,0.0]]` → `[]`. Both `bool(directions) and free_ok and intercept_ok` (positive) **and** `bool(directions) and not control_free` (negative control) now FAIL together — `FAIL row 'T...' is a MINIMISER...` and `FAIL and the rescaled free solve FAILS the FREE-DIRECTION condition for 'T...'` both fire. Neither check is vacuous under the mutant that broke this before. |
| MAJOR-4 | **DISCHARGED, with a residual ambiguity noted** | Reproduced two of the three void classes directly: a moved mutation site (harmlessly broke M1's exact-match text) → `RUN VOID: ...the mutation site occurs 0 times...`, exit **2**; a red baseline (the gate-18 defect above, before I patched around it) → `RUN VOID: the baseline suite is RED...`, exit **2**. Residual: "died elsewhere" (WRONG) and "survived" (ALIVE) still share exit code 1 — SHELL_CHECKLIST item 12's 3-way taxonomy is only 2-valued at the exit-code layer for these two outcomes (they are distinguished in the printed label, not the code). Not one of the original four MAJOR-4 outcomes, so not a re-creation of the finding, but a genuine unclosed edge worth naming. |
| MAJOR-5 | **DISCHARGED** | Constructed the exact D-650-class attack: mutate a file, `git add` it (stage it), then compare `git diff --quiet --` (old check, worktree-vs-INDEX) → exit **0** (falsely certifies clean) vs `git diff --quiet HEAD --` (new check) → exit **1** (correctly catches it). The new referent closes the exact hole MAJOR-5 named. |
| MAJOR-6 | **DISCHARGED** | `git grep -n/-l -F 'eval_v0_quiet_fit_weights'` at `beb91b1` → **8 lines / 6 files**; at `6749754` → **9 lines / 7 files**. Matches D-665's corrected numbers exactly. |
| MAJOR-7 | **DISCHARGED** | Independent Python/`tomllib` parse (not `grep -A4`, the tool that caused the original miscount) of all 18 arena configs' `engine_a`/`engine_b` → `config` and all 21 engine configs' `eval.weights_file`: **18 of 18** arena configs share the one committed weight document; **exactly 5** share the same engine config (`arena_smoke_v0`, `arena_wp13_fair_corpus`, `arena_wp13_fair_random`, `arena_wp20_label_pilot`, `arena_wp20_label_pilot_dryrun`) — the identical five D-665 names. |
| MAJOR-8 | **DISCHARGED** | Direct calls to `fit.read_rows` via 19/20/21/18-column probe files: 20 → accepted; 19, 21, 18 → all raise `FitError` naming the width and the file, no `IndexError`. |
| MAJOR-9 | **MOVED — a new escape survives** | The H10 attack (pad `RETIRED_MAY_NAME` with a live path, then re-add a reference under that path) is defeated: the `_live_tree`-backed `live` check fires regardless of list content (confirmed by mutation). The basename-vs-full-path escape is closed (confirmed). **But**: a reference spelled with different case in a live `configs/*.toml` file (`Eval_V0_Quiet_Fit_Weights.toml`) produces **zero** additional failures under either check — `git grep -F` (no `-i`) is case-sensitive and the property check does no normalization. Confirmed by direct mutation, identical failure set before/after. Materiality is low (a functional re-add must match the real, case-sensitive path to matter), but it is a real, demonstrated gap the dispatch specifically asked me to probe. A `sessions/`-path reference (forced past `.gitignore` with `git add -f`) IS caught, by the list-based `stray` check — not an escape. |
| MAJOR-10 | **MOVED — incomplete obligation** | The failure message now names the obligation and four sites: `matrix_wp22_quiet_scale.md:68`, `matrix_wp22_quiet_scale_REDTEAM.md`, `matrix_M4_snapshot_config_seam_rev3.md`, and `decisions.md` D-220/D-627. This genuinely discharges "the message doesn't say why." But D-669 (landed in the same fix round, meant to be the fuller census) finds 9 citing lines in what its own text lists as 7 files (see MINOR-9/D-669 below); three of those — `matrix_wp22_quiet_scale_REDTEAM_round3.md`, `p1_bench_prereg_REVIEW.md`, `wp20b_design_rev3_REVIEW.md` — are absent from the tripwire's own message. That may be a defensible judgment (only GOVERNING docs need re-quoting; the others are frozen records) but the message never says so, so an editor following it verbatim still under-covers what D-669 itself found. |
| MINOR-1 | **DISCHARGED** | `git archive 6749754 -- docs/ \| tar -x` then `/usr/bin/grep -rn 'CTSS' docs/` (independent of `git grep`) → **12** lines, 5 files. Matches the corrected P6. |
| MINOR-2 | **DISCHARGED** | Restored `wp22_phase1_design.md` to its `6749754` text and ran `tools/governing_citation_check.sh` directly: `15 citation(s) checked, 3 unreproduced`, exit 1, one named refusal per retired config — confirms the amendment was gate-FORCED, as D-665 item 3 now states. |
| MINOR-3 | **DISCHARGED (as a recording — the gap is deliberately left open)** | Restored `configs/instrument_quiet_fit_v0.toml` alone → `tools/config_check.sh` exit **1** (`eval.weights_file: cannot read`); restored `configs/arena_wp22_phase1_quiet_dryrun.toml` alone → exit **0** (`validate_arena_config: ... ok`). Matches D-665 item 4 exactly; the underlying gap (an arena config can name a missing engine config with every gate green) is correctly recorded as OWED, not silently fixed. |
| MINOR-4 | **DISCHARGED** | D-666 records the non-reproduction with attempted reproducers, as required. Independently re-derived the replacement numbers with a fresh Python script (not awk, not the document's command): **1140** triples, **525** with `w1+w2>w3`. The supporting "204 of 1140 flatness hits" and the "exactly those 525 / none" set-equality sub-claims were NOT independently re-derived (would require re-running the actual LP fit over all 1140 synthetic pin rows); I neither confirm nor refute those two sub-numbers. |
| MINOR-5 | **DISCHARGED** | `docs/decisions.md` carries D-667; `book_v3_SUMMARY.md:166` now reads "BOTH SINCE TAKEN" in place of the stale owed-item listing. |
| MINOR-6 | **DISCHARGED (as a recording)** | D-670 records the rule-9 `.py` gate gap as OWED, matching the finding's own ask (record, don't fix in a fix round). |
| MINOR-7 | **DISCHARGED** | `fit.py`'s docstring now reads "ALL THREE normals are parallel... skipped count goes from 1 to 3" — verified by reading the source directly. |
| MINOR-8 | **DISCHARGED** | `receipt_lines`' docstring now explains the asymmetry: the rows path is a caller's and is deliberately excluded; the weights path is the module's own constant and is deliberately included. Read directly at the source. |
| MINOR-9 | **MOVED — the correction itself miscounts** | See dedicated section below. D-669 now enumerates (closing "never enumerates"), but its own tally ("six files") does not match its own nine-item list (seven distinct files), and independent re-derivation confirms seven. |
| MINOR-10 | **DISCHARGED** | D-668 records the false premise (`threat_calculus_v1.md` never contained "CTSS") and flags `ADOPT-VC` as an addition made under a correction ruling, exactly as the finding asked. |

---

## D-669 re-derived: "nine lines in six files" is nine lines in **seven** files

D-669 states: *"MEASURED at `6b85681`, `git grep -n -F 41ef5496`: nine lines in six files"*
and then lists nine citations. Counting the distinct paths in **D-669's own sentence**:

1. `docs/decisions.md` (three lines: `:482` D-220, `:1320` D-627, `:1392` D-663)
2. `docs/experiments/matrix_M4_snapshot_config_seam_rev3.md`
3. `matrix_wp22_quiet_scale.md`
4. `matrix_wp22_quiet_scale_REDTEAM.md`
5. `matrix_wp22_quiet_scale_REDTEAM_round3.md`
6. `p1_bench_prereg_REVIEW.md`
7. `wp20b_design_rev3_REVIEW.md`

That is **seven** files, not six, by the ADR line's own listing. I then re-derived
independently at the revision the line names (`6b85681`), by a method sharing nothing with
`git grep`:

```
$ git archive 6b85681ca6dbac3be258be31dfcebbc4d3afd802 -- docs/ | tar -x -C <scratch>
$ cd <scratch> && /usr/bin/grep -rn '41ef5496' . | wc -l ; /usr/bin/grep -rl '41ef5496' . | wc -l
9
7
```

**Nine lines confirmed. Seven files confirmed, not six.** (The three earlier files not in
D-669's own list — `matrix_wp22_quiet_scale_REDTEAM_round3.md:168`,
`p1_bench_prereg_REVIEW.md:136`, `wp20b_design_rev3_REVIEW.md:171` — only cite the
abbreviated `41ef5496…` form, which is why the fuller 16-hex-character substring I first
tried, `41ef549666d787bf`, found only 5 files: it is a stricter search than either D-669's
own or mine, and understates the population for a different reason. Using D-669's own
target string on an independent tool gives the reproducible answer: 9/7.)

This is the exact defect class this whole package has been correcting (D-664 correcting
D-662, D-665 superseding both) — a MEASURED count checked against the wrong population —
now present in D-669, the line that exists specifically to stop it from recurring. Per this
task's framing: **a correction that states a new wrong number is worse than the one it
replaced.** The disposition (which sites are governing vs. records, and the seat-pair
conclusion) is unaffected by the file-count error itself, but the ADR line is not
currently reproducible as written and should be corrected to seven files (or the
`matrix_wp22_quiet_scale_REDTEAM_round3.md` entry re-checked for whether it was meant to be
folded into an existing file rather than counted separately — I found no such folding).

---

## Re-derivation table — every MEASURED number in D-665, D-666, D-669

| # | claim | document | my command (independent of the document's own) | my result | match |
|---|---|---|---|---|---|
| 1 | 8 references, 6 files at `beb91b1` | D-665 (1) | `git grep -n/-l -F 'eval_v0_quiet_fit_weights' beb91b1 --` | 8 lines, 6 files | ✅ |
| 2 | 9 references, 7 files at `6749754` | D-665 (1) | same command at `6749754` | 9 lines, 7 files | ✅ |
| 3 | FALSE of all 18 arena configs (weight-doc reading) | D-665 (2) | Python/`tomllib`: parse every `configs/*.toml`'s `weights_file` and every arena config's two seats' resolved weight doc | 18 of 18 share one weight doc | ✅ |
| 4 | FALSE of exactly 5 (engine-config reading) | D-665 (2) | same script, seat-config identity | exactly 5: `arena_smoke_v0`, `arena_wp13_fair_corpus`, `arena_wp13_fair_random`, `arena_wp20_label_pilot`, `arena_wp20_label_pilot_dryrun` | ✅ |
| 5 | gate 21 exit 1, 3 unreproduced, restoring the old design doc | D-665 (3) | restored `wp22_phase1_design.md` to `6749754`'s text, ran `tools/governing_citation_check.sh` directly | `15 citation(s) checked, 3 unreproduced`, exit 1 | ✅ |
| 6 | gate 6: engine-config restore alone → exit 1; arena-config restore alone → exit 0 | D-665 (4) | restored each config alone from `6749754`, ran `tools/config_check.sh` | exit 1 (`eval.weights_file: cannot read`); exit 0 (`... ok`) | ✅ |
| 7 | 1140 triples, entries ≤ 20 | D-666 | fresh Python triple-loop (not awk, not the shipped test) | 1140 | ✅ |
| 8 | 525 satisfy `w1+w2>w3` | D-666 | same script | 525 | ✅ |
| 9 | 204 of 1140 hit the flatness refusal (old and new constraint sets alike) | D-666 | — | **not independently re-derived** — would require re-running `fit.py`'s LP over all 1140 synthetic rows; out of scope for the time available | ⚠️ unverified |
| 10 | two-constraint set reaches `fit.py:255` on exactly the 525 where three-constraint reaches it on none (set equality) | D-666 | — | **not independently re-derived**, same reason | ⚠️ unverified |
| 11 | nine lines, six files, weights digest at `6b85681` | D-669 | `git archive 6b85681 -- docs/ \| tar -x`, then `/usr/bin/grep -rn`/`-rl` on the extracted tree | **9 lines, 7 files** | ❌ file count wrong — see dedicated section above |
| 12 | 3 sites (not D-663's named 2) recompute the weights digest from bytes | closure item E, cross-checked against D-669 | `git grep -n 'eval_v0_weights' -- crates/`, then filtered to files that actually hash the bytes (not just mention the filename) | exactly 3 call sites in 2 files: `baseline_snapshot_tests.rs:453,:645`, `handshake_identity_tests.rs:15` | ✅ |

---

## What I could not check

- **D-666's "204" and the exact set-equality sub-claim.** Would require re-running
  `fit.py`'s constrained LP over the full synthetic 1140-triple space with both the old and
  new `tempo_constraints`, which was out of scope for the time budget here. The two numbers
  I *could* independently re-derive (1140, 525) both check out.
- **Gate 3 (`cargo test --workspace --locked`) was not re-run by me.** Zero `.rs` files
  changed between `8e7fdf0` and `f73539e` (`git diff --name-only`, verified), and
  REVIEW-impl's own re-derivation table already confirmed gate 3 green at `8e7fdf0` (86
  binaries, 0 real failures). I infer gate 3's status is unaffected by this fix round rather
  than confirming it fresh.
- **Whether `matrix_wp22_quiet_scale_REDTEAM_round3.md`, `p1_bench_prereg_REVIEW.md` and
  `wp20b_design_rev3_REVIEW.md` are intended by D-669 to be RECORDS that never need
  re-quoting** (in which case MAJOR-10's shorter message list would be a defensible,
  if unstated, judgment) — the ADR line does not say so explicitly enough to settle it, and
  I did not attempt a document-status ruling that isn't mine to make.

---

## Summary

19 of 21 original findings are DISCHARGED at the property level, several (MAJOR-3, MAJOR-5,
MAJOR-7, MAJOR-8) verified against attacks harder than the ones that found them. That is a
genuinely strong fix round. But:

1. The pinned revision's own instrument gate (gate 18) is RED in a clean checkout, because
   the docs commit that closes the package added a file the retirement guard's allowlist was
   never updated for — a live, reproducible, currently-true defect, not a hypothetical.
2. BLOCKING-1's own disposition text names a `§7` that does not exist in the document it
   describes.
3. D-669 — a line whose entire purpose is to stop exactly this class of error — states
   "six files" for a list it prints with seven files in it, independently reproduced as
   seven.
4. MAJOR-9 closes two of its three attack surfaces and leaves a real, demonstrated
   case-sensitivity escape; MAJOR-10's fix under-names its own obligation relative to D-669's
   census.

**VERDICT: FAIL**, on the new gate-18 defect (decisive on its own — a currently-red gate at
the revision under confirmation), on BLOCKING-1 (partial), on MINOR-9/D-669 (a wrong
correction), and on the residual gaps in MAJOR-9 and MAJOR-10. The fix for item 1 is a
one-line allowlist addition; item 3 is a one-word ADR correction (or a corrected list); none
of the four is large, but none of them is discharged as claimed either.
