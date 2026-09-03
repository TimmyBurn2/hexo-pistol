# SCOPED VERIFICATION — the round-5 number-class sweep, checked independently (D-598)

**VERIFIED ON: Sonnet, per D-597/D-598.**

**NAMED REVISION.** `41a8f7b68b56aa2e92128722d3a9a14d1f706cc9` on `dev`. Work done
in a detached worktree at that SHA (`git worktree add --detach
/home/tom/pistol-wt/classcheck 41a8f7b`), `CARGO_TARGET_DIR` set per-command,
never exported, never the live tree. **Does it still match HEAD?** Yes:
`git -C /home/tom/Projects/HeXO-AlphaBeta rev-parse HEAD` = `41a8f7b...` both
before and after this pass; the live tree was touched only to write this file
and to read two gitignored artifacts anchored by sha256 in the registrations
(`artifacts/arc3_leverB_41_count_v3.txt`, and the two scripts under
`/home/tom/pistol-runs/arc3r-leverA/`).

**THIS IS NOT A REVIEW ROUND.** It checks one thing: that every number the
last two revisions added to `docs/experiments/wp21_prereg.md` and
`docs/experiments/wp21_throughput_prereg.md` (the object is
`git diff 735fc37 41a8f7b -- <those two files>`) derives from something the
tree holds, and that the implementer's own sweep (the ten-row table in the
dispatch) missed nothing of the same class.

## VERDICT: PASS

Every number checked below derives from the tree, by a command I ran myself.
No claim in the class contradicts its stated derivation. Three items outside
the implementer's ten rows were found and checked (the "fifteen" setting-rep
count, the opening-3/games-6–7 correction, and the C2 `turns`/`turns + 1`
formula against `capture.rs`) and all derive correctly. One citation
(`round 4's m7`) is confusing but not false — detailed under FOUND BEYOND THE
CHECKLIST, not treated as a FAIL because it names a real string in a real
document about the real topic; a clearer citation exists and is named there.

## THE IMPLEMENTER'S TEN ROWS, EACH VERIFIED

| claim | my command | my result | the document's claim |
|---|---|---|---|
| **93** capture files (C1, and §7's driver comment) | `python3 -c "print(3*(1+2+4+8+16))"`; separately confirmed against the driver script's `for rep in 1 2 3; do for N in 1 2 4 8 16` loop and the harness's per-setting-rep file-naming loop (`cap-N$N-rep$REP-p$i.txt` for `i` in `1..N`), both read from the printed §7 blocks | `93`; loop structure produces exactly `N` files per setting-rep, `31` per rep, `93` over 3 reps — matching the driver's own glob `cap-N*-rep*-p*.txt` | 93 = `3 x (1 + 2 + 4 + 8 + 16)` |
| T-F's sub-range **`13..32`** | Read §3's arithmetic (`3487 = 15 x 218 + 1 x 217`, tranches 1..15 at 218 openings; pilot consumed `0..12`, so the sweep's own range starts at opening 13) and §9's literal command `--skip 13 --take 20` | tranche one = openings `13..230`; first twenty = `13..32` (13 + 20 − 1); 20 openings = 40 games, matching "tranche one's first forty" | tranche one runs at skip 13; the first twenty of its range is `13..32` |
| a tranche's capture **"~3 hours"** | Read `wp21_prereg.md` §3 line 126: `ESTIMATED capture 12 443 x 0.885445 = 11 018 s = 3.06 h` | 3.06 h | "~3 hours" |
| §1's four fractions **79.1 / 10.4 / 10.1 / 0.4 %** | `python3 -c "cap=11018;play=1442;replay=1409;cold=58;t=13927;[print(round(100*v/t,1)) for v in (cap,play,replay,cold)]"` (the four addends read from `wp21_prereg.md` §3 lines 126-131) | `79.1`, `10.4`, `10.1`, `0.4`; sum of seconds = 13 927 exactly | 79.1 %, 10.4 %, 10.1 %, 0.4 % |
| play-pass hit rate **0.5263** | `awk -F'\t' '$1>=0 && $1<=5' /home/tom/pistol-runs/wp20pilot-artifacts/capture_v1.txt \| wc -l` → 152; `... {print $3} ... \| sort -u \| wc -l` → 72; `python3 -c "print((152-72)/152)"` | 152 records, 72 distinct, `0.5263157...` → 0.5263 | `(152 − 72) / 152` = 0.5263 |
| **"~2.2 min"** and **"~1.1 min"** | `python3 -c "print(152*0.885445/60); print(72*0.885445/60)"` | `2.243` min, `1.063` min | ~2.2 min, ~1.1 min (round 5's fix corrected a stale "~2.3 min" — 2.243 rounds to 2.2, not 2.3) |
| C3's **3.54 s** | `python3 -c "print(round(4*0.885445,5))"` | `3.54178` | 3.54 s (`c(N) <= 4`, four times the measured 0.885445) |
| lever A's **0.56–1.12 h** | `python3 -c "s=5*3*152*0.885445; print(s, s/3600, 2*s/3600)"` | `2018.81 s`, `0.5608 h`, `1.1216 h` | 0.56 h at contention 1.0, 1.12 h at 2.0. **Note:** this row is unchanged context in this diff span (no `+`/`-` marker on it) — it was already correct before 735fc37 and the class sweep's inclusion of it is precautionary, not a correction. I re-derived it anyway since the dispatch named it. |
| C4's **0.42 % / 1.0042**, and its citation | `sed -n '705,718p' docs/experiments/wp21_throughput_prereg_rev2_REVIEW.md`; `python3 -c "a=343788;b=342359;print(a/b, (a/b-1)*100)"` | ratio `1.0042`, diff `0.42` — exact; and `grep -n "^### A\|A3\b\|A7\b" wp21_throughput_prereg_rev2_REVIEW.md` shows **A3** is the `search_nodes`/C4 finding and **A7** is an unrelated finding (`position` line vs `key_seq`) | the fix changed "the round-2 review's **A7** re-derivation" to "**A3**" — confirmed A3 is the right citation, A7 was wrong |
| **152** and **72** | `sha256sum artifacts/arc3_leverB_41_count_v3.txt` (live tree, gitignored, read-only); `sha256sum /home/tom/pistol-runs/wp20pilot-artifacts/capture_v1.txt`; then the two `awk`/`sort -u`/`wc -l` commands above | receipt sha256 = `cbad0786...` (matches §3's citation exactly); capture sha256 = `4563f050...` (matches the receipt's own "input sha256"); 152 records and 72 distinct `position` lines over games 0–5, computed directly from the capture body | 152 MEASURED (the receipt states it verbatim: "records for openings 0..2 (corpus games 0-5): 152"); 72 distinct, computed the same way |

## FOUND BEYOND THE CHECKLIST — three more numbers on added lines, and one citation worth a second look

1. **"the fifteen setting-reps"** (§7's driver comment, twice, and §8's new driver
   row) — not in the implementer's ten rows. Derivation: §3.2 registers `3 reps
   each, in the fixed order 1, 2, 4, 8, 16`, i.e. `3 x 5 = 15`; the driver's own
   `for rep in 1 2 3; do for N in 1 2 4 8 16; do` loop (read from the printed §7
   block) runs exactly 15 harness invocations. Correct.

2. **"the registered dry run's opening 3 plays move-for-move identically to the
   pilot's games 6–7"** (§3.1, replacing a prior "opening 0's two games ... "
   claim) — not in the ten rows, and a substantive correction (index changed,
   not just prose). Derivation: §7.1's own play-pass command is
   `tools/wp21_tranche_config.py --skip 3 --take 1 --pilot-range ...` (read from
   the printed dry-run record), i.e. it plays opening index 3, not 0; the
   corpus convention (confirmed by direct inspection of
   `/home/tom/pistol-runs/wp20pilot-artifacts/capture_v1.txt`'s game-index
   field, which runs 0..25 for 13 openings x 2 games) is opening `N` → games
   `2N, 2N+1`, so opening 3 → games 6, 7. The new text is internally consistent
   with the command actually registered; the old text (opening 0) was not.

3. **C2's formula, "a decided game asks `turns` prefixes, a capped one
   `turns + 1`"** (§4's C2 row and §7's driver comment) — checked against
   `crates/pistol-arena/src/capture.rs:29-45`'s `asked_prefixes`:
   `(0..=last).filter(|k| !(decided && *k == last))`. For a decided game this
   drops exactly one of `last + 1` candidates, leaving `last` = `turns`; for an
   undecided ("capped") game nothing is dropped, leaving `last + 1` =
   `turns + 1`. The claim matches the function exactly, and the driver's `awk`
   (`n+=t+(r=="capped")`) implements the same rule.

4. **The "95 %" / "5 %" decision-rule rewrite** (§3.4) — not a new empirically
   derived number; `100 - 5 = 95` is exact, and the rewrite ("smallest N at
   least 95 % of the best") is the same tie-break the prior wording used ("a
   tie inside 5 % goes to the smaller N") stated as one rule instead of two.
   Checked for consistency, not a defect.

5. **"(round 4's m7)"** — the digest-table row for `tools/wp21_tranche_config.py`
   in `wp21_prereg.md` cites this for "its header comment no longer names a
   validator." I traced the "round N" convention: `wp21_prereg_revK_REVIEW.md`
   files are headed by a ROUND number distinct from `K` (`rev4→ROUND 2`,
   `rev5→ROUND 3 of 5`, `rev6→ROUND 4 of 5`, `rev7→ROUND 5 of 5`). So "round 4"
   = `wp21_prereg_rev6_REVIEW.md`. That file's own top-level **m7** row (line
   96) is "limb 3 internal agreement" — unrelated. But the same file's **M8**
   row (line 89) carries a parenthetical, "*Residual: the pinned generator
   still writes "Validate this file with tools/config_check.sh" (**m7**)*" —
   this is the exact topic, informally pre-tagging a finding for the next
   round. The finding was then formally opened, verified and **CLOSED** as its
   own top-level **m7** in `wp21_prereg_rev7_REVIEW.md` ("round 5"), with a
   `grep`, a passing `cargo test` (18/18) and a matching sha256. **Verdict:
   not false** — round 4's document does contain "m7" against this exact
   topic — **but confusing**, since round 4's *own* top-level m7 is a different
   finding, and the citation that actually resolves the claim with evidence is
   round 5's. A clearer citation would read "(round 5's m7)". Not treated as a
   FAIL: the string is real, is in the named document, is about the named
   topic — it is a citation-precision nit, not a fabricated or transcribed
   number.

## THE THREE REQUIRED EXTRA CHECKS

**(a) The two corrected sentences read correctly and agree.** The C1 row
("§7's driver `cmp`s all **93** — `3 x (1 + 2 + 4 + 8 + 16)`, the schedule
§3.2 registers — against `cap-N1-rep1-p1.txt`") and the §7 driver comment
("then C1 and C2 over all 93 capture files — 3 x (1 + 2 + 4 + 8 + 16), the
schedule §3.2 registers") state the identical arithmetic and the identical
source (§3.2), and both match §3.2's own text (`{1,2,4,8,16}`, `3 reps`) and
the driver script's actual loop. Grammatical and factual agreement confirmed.

**(b) `lever_a.sh` unchanged; matches §7 and §8.**
```
$ sha256sum /home/tom/pistol-runs/arc3r-leverA/lever_a.sh /home/tom/pistol-runs/arc3r-leverA/lever_a_harness.sh
ca07d4a65e1ffd2d4ff0d72e401d60af304f1c7b1bd06f2df7fb11e615c99d29  lever_a.sh
03e5b7eb0c5d02627dd459269dc781be5927dbe0a84e575a1186e35ca4afd0d0  lever_a_harness.sh
```
Both match §8's digest table exactly. I also extracted both fenced code blocks
from the document (the two `#!/usr/bin/env bash` blocks in §7) with a small
Python script and `diff`'d each against the live file: both report **no
differences** — byte-identical, confirming the round-5 fix touched the
document's prose, not either script.

**(c) `tools/governing_citation_check.sh` exits 0.**
```
$ CARGO_TARGET_DIR=/home/tom/pistol-wt/classcheck/target tools/governing_citation_check.sh
governing_citation_check: 9 governing document(s), 0 proposed path(s)
...
docs/experiments/wp21_prereg.md: 26 citation(s) checked, 0 unreproduced
docs/experiments/wp21_throughput_prereg.md: 16 citation(s) checked, 0 unreproduced
...
DESIGN_CITATION_CHECK_DONE
EXIT: 0
```
Confirmed exit 0, zero unreproduced citations in either document.

## WHAT THIS PASS DID NOT LOOK AT

- **The transcribed log/output lines.** The re-taken dry-run records in §7.1 of
  both documents (the `leverA:` lines, the `ls` refusal lines, the
  `label_cache_count:` block) changed because a **new** dry run was executed
  (new hashes, new timestamps, new wall-clock numbers like `wall_s 0.545`).
  Per the dispatch's own exclusion ("not a transcribed log line"), I did not
  re-run these dry runs or re-derive their internal arithmetic
  (`s_per_label = wall_s / records`, `throughput = N x records / wall_s`) from
  first principles — I read them as verbatim output, not as claims. A full
  round would plausibly spot-check a couple of these for internal arithmetic
  consistency even though they're excluded from the class; I did not.
- **The prose outside the numeric class.** Wording-only changes (T-A1/T-A2's
  `MISS`→`MISSES`, `HIT`→`HITS`; the removed "two earlier runs are on disk"
  paragraphs; the §6.1 rewrite's restructuring; the C2/C4 defect-class prose)
  were read for context but not adjudicated for clarity, redundancy, or
  D-423/D-424 compliance — that is REVIEW territory, not this pass's.
- **Whether the underlying `wp21_prereg_rev7_REVIEW.md` ("round 5") review
  itself is sound.** I used its m7 entry (grep, cargo test, sha256) as a
  derivation source for the citation check above, and did not re-run its
  `cargo test -p pistol-arena --locked --test wp21_tranche_config_tests`
  myself to reproduce "18 passed; 0 failed" — I took that review's own
  verified claim as tree content, one level removed from a live re-run.
- **Whether revision 7 of `wp21_throughput_prereg.md` (jumping from
  revision 5, skipping 6) and revision 7 of `wp21_prereg.md` (from 6) landing
  on the same number is intentional or coincidental.** Revision numbers are
  explicitly out of scope for this class sweep (not a claim about the world),
  and I did not chase this.
- **A live re-run of lever A or the sweep itself**, at any N, including N = 1
  or N = 2 — not attempted, per the dispatch's own ceiling and because no
  derivation above required it. All verification here is static: reading
  files, hashing, and re-computing arithmetic already MEASURED elsewhere in
  the tree.
- **The full six-test suite the design gate itself is stalled on** (the arc3
  design-gate STOP referenced in `arc3_leverB_41_count_v3.txt`'s sibling
  documents) — orthogonal to this throughput/sweep number sweep, not touched.

## WORKTREE

Removed after this report was written:
`git -C /home/tom/Projects/HeXO-AlphaBeta worktree remove /home/tom/pistol-wt/classcheck`.
