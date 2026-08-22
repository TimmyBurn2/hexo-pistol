# MICRO-MATRIX U4-R — measurement report (stakeless measurer, D-328/D-330)

This report is produced by a STAKELESS MEASURER under architect ruling R11 and
ADR D-328 as amended by D-330. Every command below was run VERBATIM from the
registered script; every output block is COMPLETE and unedited. The measurer
has no stake in and did not interpret the recommendation of the matrix this
report feeds.

## Revision measured

**LOUD NOTE — REVISION DRIFTED MID-TASK.** At dispatch time (step 1 of this task)
`git rev-parse HEAD` = `871e6789fcefebe5d275cb36224b08315c9fbe6b`, matching the
expected `871e678...`, and `git status --porcelain` was empty (clean tree). The
registered script (M0..M12) and follow-ups (a), (b), (c) were ALL run against that
revision. While this report was being assembled — after the script and follow-ups
had already completed — a NEW commit landed on `dev`:

```
c9d4e64  docs(decisions): D-331 lands the CLAIM-HOME law — ...
```

Re-running `git rev-parse HEAD` at report-write time returns the new commit:

```
git rev-parse HEAD
c9d4e643c35924785fabebca9bd9cbe826b76e2c

git status --porcelain
?? docs/experiments/matrix_U4R_measurements.md
```

`git diff --stat 871e678 c9d4e64` shows exactly one file touched by that commit:
`docs/decisions.md | 2 ++` (1 file changed, 2 insertions(+), 0 deletions(-)). No file
this task measured content from — `U4_soundness_instrument.md`, the M3/M4 selection
records, the review reports, the sibling units, or `tools/` — was touched. The ONE
number in this report that is now stale relative to current HEAD is M7's
`docs/decisions.md` line count: measured as **705** against `871e678`; at current
HEAD `c9d4e64` it is **707** (D-331 added 2 lines). Every other measurement below is
unaffected by the drift because its source file did not change between the two
revisions. This report records the M0..M12 and (a)/(b)/(c) outputs exactly as
produced against `871e678`, per the task's instruction to continue and report loudly
rather than to silently re-run against a moving target.

## Registered script (verbatim, as read before execution)

```bash
#!/usr/bin/env bash
# REGISTERED MEASUREMENT COMMANDS for MICRO-MATRIX U4-R.
# Registered by the authoring session BEFORE any cell was written.
# Run from the repository root. Every command's COMPLETE output is returned.
set +e
cd /home/tom/Projects/HeXO-AlphaBeta || exit 1
U4=docs/experiments/U4_soundness_instrument.md

echo "===== M0 revision ====="
git rev-parse HEAD
git status --porcelain
echo "(end M0)"

echo "===== M1 unit size and section map ====="
wc -l "$U4"
awk '/^#{2,3} /{print NR"\t"$0}' "$U4"
echo "(end M1)"

echo "===== M2 U4-Z span ====="
awk '/^## U4-Z\./{s=NR} END{print "U4-Z start="s"  file_end="NR"  span="(NR-s+1)}' "$U4"
echo "(end M2)"

echo "===== M3 head status-matter span (REVIEW STATUS .. Theory-citations paragraph) ====="
awk '/^\*\*REVIEW STATUS/{s=NR} /^Theory citations are calculus IDs/{e=NR} END{print "status_start="s"  theory_para="e"  span="(e-s)}' "$U4"
echo "(end M3)"

echo "===== M4 U4-A lineage span ====="
awk '/^## U4-A\./{s=NR} /^## 8\. MATRIX M3/{e=NR} END{print "U4-A start="s"  s8_start="e"  span="(e-s)}' "$U4"
echo "(end M4)"

echo "===== M5 matrix-derived fold blocks at the heads of section 8 and section 9 ====="
awk '/^## 8\. MATRIX M3/{f=1;s=NR} f&&/^### 8\.1/{print "S8 fold span "s".."NR-1" = "(NR-s)" lines"; f=0}' "$U4"
awk '/^## 8\. MATRIX M3/{f=1} f&&/^### 8\.1/{f=0} f&&/^>/{c++} END{print "S8 blockquote lines="c+0}' "$U4"
awk '/^## 9\. MATRIX M4/{f=1;s=NR} f&&/^### 9\.1/{print "S9 fold span "s".."NR-1" = "(NR-s)" lines"; f=0}' "$U4"
awk '/^## 9\. MATRIX M4/{f=1} f&&/^### 9\.1/{f=0} f&&/^>/{c++} END{print "S9 blockquote lines="c+0}' "$U4"
echo "(end M5)"

echo "===== M6 CLAIM-HOME duplication surface: distinct sections each registered claim token is stated in ====="
awk '/^#{1,3} /{sec=$0} {print sec"\t"$0}' "$U4" > /tmp/u4r_tagged.txt
while IFS= read -r t; do
  echo "---- token: $t"
  printf 'occurrences(lines): '; grep -Fc -- "$t" "$U4"
  printf 'occurrences(total): '; grep -Fo -- "$t" "$U4" | wc -l
  echo 'distinct sections:'
  grep -F -- "$t" /tmp/u4r_tagged.txt | cut -f1 | sort -u
done <<'TOK'
D-323
D-329
D-316
D-320
S-M
N-E
four conditions
SELECTED AND NOT BUILT
eight of twelve
af8082a
7e0a328
DEPENDS-OPEN-THEORY
TOK
echo "(end M6)"

echo "===== M7 landed record sizes the folds restate ====="
wc -l docs/experiments/matrix_M3_selection.md docs/experiments/matrix_M4_axisA_selection.md docs/experiments/matrix_M4_axisA_round4.md docs/experiments/matrix_M3_soundness_instrument_rev2.md docs/decisions.md
echo "(end M7)"

echo "===== M8 option (c) generator precedent and current tools/ inventory ====="
git show b067d47 --numstat -- crates/pistol-cli/tests/baseline_snapshot_tests.rs tools/baseline_snapshot.sh
echo "--- tools/ script count:"
ls tools/ | wc -l
echo "--- tools/ artefacts that read or write anything under docs/:"
grep -ln "docs/" tools/* 2>/dev/null
echo "(end M8)"

echo "===== M9 the three U4 review verdicts, as the reports state them ====="
for f in docs/experiments/wp15b_U4_REVIEW.md docs/experiments/wp15b_U4_REVIEW_urev6.md docs/experiments/wp15b_U4_REVIEW_urev7.md; do
  echo "---- $f"
  grep -n "BLOCKING," "$f" | head -4
done
echo "(end M9)"

echo "===== M10 every live occurrence of the phrase 'differential gate' in U4 ====="
grep -n "differential gate" "$U4"
echo "(end M10)"

echo "===== M11 sibling unit sizes and declared u-revs ====="
wc -l docs/experiments/U1_gate_supersession.md docs/experiments/U2_node_protocol.md docs/experiments/U3_tier_t.md docs/experiments/WPQ_seed.md docs/experiments/section_owner_table.md
grep -n "^\*\*u-rev [0-9]" docs/experiments/U1_gate_supersession.md docs/experiments/U2_node_protocol.md docs/experiments/U3_tier_t.md docs/experiments/section_owner_table.md
echo "(end M11)"

echo "===== M12 U4 lines that are self-referential status prose markers ====="
printf 'lines naming a u-rev of this unit: '; grep -c "u-rev [0-9]" "$U4"
printf 'lines naming a wp15b_U4_REVIEW report: '; grep -c "wp15b_U4_REVIEW" "$U4"
printf 'lines carrying REPAIRED/WITHDRAWN/RE-CHECKED/MARKED AT: '; grep -cE "REPAIRED|WITHDRAWN|RE-CHECKED|MARKED AT u-rev|RE-READ AT|FOLDED AT" "$U4"
echo "(end M12)"
```

## M0..M12 — script output (verbatim, via `bash U4R_registered_commands.sh 2>&1`)

```
===== M0 revision =====
871e6789fcefebe5d275cb36224b08315c9fbe6b
(end M0)
===== M1 unit size and section map =====
1886 docs/experiments/U4_soundness_instrument.md
173	## U4-A. Lineage — what has attacked this unit's content, and at which revision
205	## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u-rev 6: S-M (D-323)**
460	### 8.1 Why S-C fell
502	### 8.2 THE DIFFERENTIAL GATE — **S-M IS SELECTED (D-323)**; the text below is the RECORD of S-E
635	### 8.3 The other three gates, re-scoped — NAMED, NOT LETTERED
770	### 8.4 The mutation ledger, with witnesses
799	### 8.5 Floors, not printed counts
807	### 8.6 REJ-DEPTHPROOF, stated where it belongs
814	### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
865	## 9. MATRIX M4 — the snapshot's config seam — **AXIS A SELECTED AT u-rev 7: N-E (D-329)**
1258	### 9.1 The five amendments the design made to N-A after that attack
1335	### 11.6 One thing this WP does NOT close
1347	## U4-T. The tests this unit registers
1362	## U4-M. What this unit measures
1422	### Cost
1441	## U4-Z. ADR lines this unit owes, the OPEN decision it stops on, and what is OPEN
1456	### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316); ITS RESIDUAL IS DISPOSED OF BY D-320.
1639	### ADR lines
1743	### OPEN — carried forward, not closed by the carve
(end M1)
===== M2 U4-Z span =====
U4-Z start=1441  file_end=1886  span=446
(end M2)
===== M3 head status-matter span (REVIEW STATUS .. Theory-citations paragraph) =====
status_start=115  theory_para=167  span=52
(end M3)
===== M4 U4-A lineage span =====
U4-A start=173  s8_start=205  span=32
(end M4)
===== M5 matrix-derived fold blocks at the heads of section 8 and section 9 =====
S8 fold span 205..459 = 255 lines
S8 blockquote lines=250
S9 fold span 865..1257 = 393 lines
S9 blockquote lines=388
(end M5)
===== M6 CLAIM-HOME duplication surface: distinct sections each registered claim token is stated in =====
---- token: D-323
occurrences(lines): 46
occurrences(total): 50
distinct sections:
### 11.6 One thing this WP does NOT close
### 8.2 THE DIFFERENTIAL GATE — **S-M IS SELECTED (D-323)**; the text below is the RECORD of S-E
### 8.3 The other three gates, re-scoped — NAMED, NOT LETTERED
### 8.4 The mutation ledger, with witnesses
### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u-rev 6: S-M (D-323)**
### ADR lines
### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316); ITS RESIDUAL IS DISPOSED OF BY D-320.
### Cost
### OPEN — carried forward, not closed by the carve
## U4-A. Lineage — what has attacked this unit's content, and at which revision
## U4-T. The tests this unit registers
# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT
---- token: D-329
occurrences(lines): 57
occurrences(total): 62
distinct sections:
## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u-rev 6: S-M (D-323)**
## 9. MATRIX M4 — the snapshot's config seam — **AXIS A SELECTED AT u-rev 7: N-E (D-329)**
### ADR lines
### OPEN — carried forward, not closed by the carve
## U4-A. Lineage — what has attacked this unit's content, and at which revision
## U4-M. What this unit measures
## U4-T. The tests this unit registers
# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT
---- token: D-316
occurrences(lines): 27
occurrences(total): 29
distinct sections:
### 8.3 The other three gates, re-scoped — NAMED, NOT LETTERED
### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u-rev 6: S-M (D-323)**
## 9. MATRIX M4 — the snapshot's config seam — **AXIS A SELECTED AT u-rev 7: N-E (D-329)**
### ADR lines
### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316); ITS RESIDUAL IS DISPOSED OF BY D-320.
### OPEN — carried forward, not closed by the carve
## U4-Z. ADR lines this unit owes, the OPEN decision it stops on, and what is OPEN
# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT
---- token: D-320
occurrences(lines): 20
occurrences(total): 28
distinct sections:
### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316); ITS RESIDUAL IS DISPOSED OF BY D-320.
### OPEN — carried forward, not closed by the carve
## U4-Z. ADR lines this unit owes, the OPEN decision it stops on, and what is OPEN
# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT
---- token: S-M
occurrences(lines): 54
occurrences(total): 61
distinct sections:
### 11.6 One thing this WP does NOT close
### 8.2 THE DIFFERENTIAL GATE — **S-M IS SELECTED (D-323)**; the text below is the RECORD of S-E
### 8.3 The other three gates, re-scoped — NAMED, NOT LETTERED
### 8.4 The mutation ledger, with witnesses
### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u-rev 6: S-M (D-323)**
### ADR lines
### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316); ITS RESIDUAL IS DISPOSED OF BY D-320.
### Cost
### OPEN — carried forward, not closed by the carve
## U4-A. Lineage — what has attacked this unit's content, and at which revision
## U4-T. The tests this unit registers
# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT
---- token: N-E
occurrences(lines): 64
occurrences(total): 72
distinct sections:
## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u-rev 6: S-M (D-323)**
## 9. MATRIX M4 — the snapshot's config seam — **AXIS A SELECTED AT u-rev 7: N-E (D-329)**
### ADR lines
### OPEN — carried forward, not closed by the carve
## U4-A. Lineage — what has attacked this unit's content, and at which revision
## U4-M. What this unit measures
# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT
---- token: four conditions
occurrences(lines): 8
occurrences(total): 8
distinct sections:
## 9. MATRIX M4 — the snapshot's config seam — **AXIS A SELECTED AT u-rev 7: N-E (D-329)**
### OPEN — carried forward, not closed by the carve
## U4-A. Lineage — what has attacked this unit's content, and at which revision
# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT
---- token: SELECTED AND NOT BUILT
occurrences(lines): 5
occurrences(total): 5
distinct sections:
## 9. MATRIX M4 — the snapshot's config seam — **AXIS A SELECTED AT u-rev 7: N-E (D-329)**
### ADR lines
### OPEN — carried forward, not closed by the carve
# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT
---- token: eight of twelve
occurrences(lines): 1
occurrences(total): 1
distinct sections:
### 9.1 The five amendments the design made to N-A after that attack
---- token: af8082a
occurrences(lines): 9
occurrences(total): 11
distinct sections:
### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u-rev 6: S-M (D-323)**
### OPEN — carried forward, not closed by the carve
## U4-A. Lineage — what has attacked this unit's content, and at which revision
# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT
---- token: 7e0a328
occurrences(lines): 12
occurrences(total): 14
distinct sections:
## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u-rev 6: S-M (D-323)**
## 9. MATRIX M4 — the snapshot's config seam — **AXIS A SELECTED AT u-rev 7: N-E (D-329)**
### ADR lines
### OPEN — carried forward, not closed by the carve
## U4-A. Lineage — what has attacked this unit's content, and at which revision
# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT
---- token: DEPENDS-OPEN-THEORY
occurrences(lines): 12
occurrences(total): 12
distinct sections:
### 8.2 THE DIFFERENTIAL GATE — **S-M IS SELECTED (D-323)**; the text below is the RECORD of S-E
### 8.3 The other three gates, re-scoped — NAMED, NOT LETTERED
### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**
## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u-rev 6: S-M (D-323)**
### ADR lines
### OPEN — carried forward, not closed by the carve
## U4-A. Lineage — what has attacked this unit's content, and at which revision
## U4-T. The tests this unit registers
# WP-1.5b U4 — the soundness instrument and the snapshot seam: DESIGN UNIT
(end M6)
===== M7 landed record sizes the folds restate =====
   216 docs/experiments/matrix_M3_selection.md
   149 docs/experiments/matrix_M4_axisA_selection.md
   305 docs/experiments/matrix_M4_axisA_round4.md
   451 docs/experiments/matrix_M3_soundness_instrument_rev2.md
   705 docs/decisions.md
  1826 total
(end M7)
===== M8 option (c) generator precedent and current tools/ inventory =====
commit b067d47083282ab7b66fb053a5b644d6a8487a26
Author: Timmy Burn <148332956+seeligto@users.noreply.github.com>
Date:   Sat Aug 22 10:53:06 2026 +0200

    fix(tools): a SPACE in a caller-named --corpus name is refused instead of shifting every field after it in the record, and the guarded expression and the recorded expression become the same one
    
    D-324 and `matrix_M4_stop_round3.md` record this OPEN: a space in a caller-named
    `--corpus` path reached the record unescaped at exit 0 under the COMPLETE kind
    token. REPRODUCED at `369d43a` on the shipped, unpatched script before anything
    changed — the record line read `corpus mini corpus.txt sha256 <hex> positions 2`,
    so a reader taking the digest from the line's fourth token got the literal string
    `sha256`. The printable-ASCII allow-list admitted it BY CONSTRUCTION: `[:print:]`
    includes the space, in C and in every locale, so the guard let it through by
    definition rather than by oversight.
    
    REFUSED, NOT SUPPORTED. Making a spaced name work means quoting or delimiting the
    line, which changes the record SCHEMA and every reader of it, against a format
    the header pins to a three-run byte-identical result. The record's leading-tokens
    rule is the same rule that lets the engine's handshake carry multi-token VALUES —
    so a value may hold a space and a field before one may not. Refusing is one
    guard and costs the caller an `mv`. The named cost is stated in the script beside
    the existing refusal's, and the printable-ASCII arm's own cost is marked as
    standing unchanged: the new arm is an addition and narrows nothing.
    
    A SECOND DEFECT WAS FOUND AND FIXED IN THE SAME BOUNDARY, and it is why the guard
    could drift from the record at all: the record wrote `$(basename "$CORPUS")` while
    the guard checked `${CORPUS##*/}`. Two spellings of "the basename" is how a guard
    comes to guard a value the record does not write, and the substitution also strips
    the trailing newline the control-character refusal exists for and discards its
    status inside an `echo` argument (SHELL_CHECKLIST items 1 and 9). Both emit lines
    now use `${x##*/}` — the same expression the guard checked, character for
    character. Verified this changed no recorded byte on an unspaced corpus.
    
    ONLY THE BASENAME IS GUARDED, because only the basename reaches the record. A
    corpus inside a spaced DIRECTORY still runs, and the test's control half pins
    that so the refusal cannot quietly widen into one that refuses everything.
    
    DRIVING TEST (SHELL_CHECKLIST item 10, the coverage rule):
    `a_corpus_name_carrying_a_space_is_refused_rather_than_shift_the_records_fields`
    drives the SHIPPED script through the existing harness, in two halves — the
    refusal (exit code asserted as exactly 1, stderr naming the space) and the
    control (spaced directory, unspaced name, asserting field 2 is the basename,
    field 3 is `sha256` and field 4 is 64 hex digits). AGAINST THE UNPATCHED SCRIPT
    IT FAILS, `left: Some(0) / right: Some(1)`, with the mangled record in the
    failure message. It asserts the CODE and not `!success`, because this script has
    no void class — now stated in the usage block rather than inferred from silence
    (item 12 obligation 1).
    
    `$CONFIG` is NOT guarded and a comment says why: no flag sets it, so it is not
    caller-named. That comment also binds the next change — if a `--config` flag is
    ever added, this line joins the guard in the same commit.
    
    `cargo test -p pistol-cli --test baseline_snapshot_tests`: 30 passed, 0 failed.
    `tools/ci.sh`: all 14 gates, `ci: all gates passed`, exit 0.
    Independently re-driven by the dispatching session: spaced basename -> exit 1
    with the named refusal; spaced directory -> not caught by this guard.
    
    RESIDUAL, unchanged and still open: D-232's named residual — two byte-identical
    corpora under two names still give differing invariant blocks. This narrows which
    names are admissible; it does not take the name out of the block.
    
    Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>

91	0	crates/pistol-cli/tests/baseline_snapshot_tests.rs
54	4	tools/baseline_snapshot.sh
--- tools/ script count:
18
--- tools/ artefacts that read or write anything under docs/:
tools/arena_smoke.sh
tools/artifact_check.sh
tools/baseline_snapshot.sh
tools/bench_delta.sh
tools/ci.sh
tools/config_check.sh
tools/decision_key_check.sh
tools/determinism.sh
tools/file_justification_check.sh
tools/movetime_check.sh
tools/perft_check.sh
tools/scratch_preflight.sh
tools/search_oracle_check.sh
tools/SHELL_CHECKLIST.md
tools/solver_edge_check.sh
tools/solver_link_check.sh
tools/tactical_check.sh
tools/wp15b_attribution_check.py
(end M8)
===== M9 the three U4 review verdicts, as the reports state them =====
---- docs/experiments/wp15b_U4_REVIEW.md
32:**3 BLOCKING, 3 MAJOR, 5 MINOR.**
---- docs/experiments/wp15b_U4_REVIEW_urev6.md
7:re-review FAIL is COLLECTED AND REPORTED, not looped on in-session: the BLOCKING,
39:**1 BLOCKING, 2 MAJOR, 4 MINOR.**
---- docs/experiments/wp15b_U4_REVIEW_urev7.md
24:**0 BLOCKING, 4 MAJOR, 4 MINOR.**
408:| Z44 | Both shipped-instrument defects CLOSED at `b067d47` and `a102c6a`; REVIEW-impl PASSED at `84ff8d7`, 0/0/3, on mutation-checked controls | **HOLDS** — `wp15b_trackC_REVIEW_impl.md:709–711` reads **VERDICT: PASS**, *"0 BLOCKING, 0 MAJOR, 3 MINOR (F1, F2, F3)"* |
473:**VERDICT: FAIL — 0 BLOCKING, 4 MAJOR, 4 MINOR.**
(end M9)
===== M10 every live occurrence of the phrase 'differential gate' in U4 =====
138:| **MAJOR 3** — U4-Z's u-rev 2 SELECTION block still asserts "S-E **is** the differential gate" in present tense while its two siblings were retargeted at u-rev 6 | **REPAIRED.** The SELECTION block — which is carve prose recorded AFTER the comparison, not the selected-from text — now states that it records the u-rev 2 EXECUTION and that **the differential gate's instrument since D-323 is S-M**. **NO LIVE SENTENCE IN THIS UNIT IDENTIFIES THE DIFFERENTIAL GATE AS S-E.** The two-shape comparison at the SHAPE 1 / SHAPE 2 cells is left unedited, on the unit's stated discipline, and the report agrees that is right |
159:| **MINOR 7** — the stubbed matrix's column header is a live letter-address in the slot a future author copies | **REPAIRED** — the column reads "the differential gate's instrument", with the retirement noted |
192:THIS u-rev; **the differential gate's SEAM decision**, which D-323 records as separate
399:> | Option | the differential gate's instrument *(this column head read `(b)'s instrument` until u-rev 6 — MINOR 7: a live letter-address in the one slot a future author copies its frame from, and the authored matrices at `f8e73e4` and `d48824f` sensibly did not use it)* | Cost | Failure modes |
453:> differential gate as "S-E, with the reduced S-C beside it"; both are retargeted at
651:`(b)` is the differential gate and is §8.2's subject, `(c)` is the colony family,
747:  and by the differential gate (§8.2 — **S-M** since D-323, S-E until u-rev 6).
792:| M3 | The FILTERED row emits `Cover::cells()` flattened at phase 0 and does not regenerate at phase 1 | the differential gate (**S-M** since D-323; registered as S-E's class until u-rev 6) | **NOT BUILT. THE "BUILT" THIS CELL CARRIED UNTIL u-rev 6 IS WITHDRAWN AS FALSE** — MAJOR 5 of `wp15b_U4_REVIEW.md`. What follows is a REQUIRED PROPERTY of a witness and not a witness, and §8.4's lead-in promises a named position or a built one. Revision 6's witness was inert under EQUALITY too: with a single two-cell cover the stale union minus the played cell EQUALS the correct phase-1 set, so nothing separates them. **The property a witness must have:** a phase-0 union of **three or more** cells — `cover.rs`'s own `{a,b} {b,d} {d,e}` shape, whose union is `{a,b,d,e}` while the phase-1 set after any one cell is strictly smaller. **That shape is not a position.** It is the abstract window-empties example in `crates/pistol-solver/src/cover.rs`'s module doc comment: no coordinates, no stone counts, no parity, no legality, no pin. **A POSITION A LEGAL GAME REACHES IS OWED**, on the `wp15b_mutation_witnesses.rs` pattern MAJOR 8's repair established for M4 and M6 — a row MAJOR 8's literal scope (§17 named M4 and M6) did not reach. It is in U4-Z's OPEN list rather than left inside this cell. *Separately, and it does NOT discharge the witness: M3 round 2 MEASURED the mutation's class on a proxy — S-M fires on 20 of 20 differing nodes of the registered playout regime and the matrix's recommended row S-K on 0 of 20 (D-323), which is what killed S-K.* |
793:| M4 | Minimum-cardinality covers instead of inclusion-minimal | the differential gate (**S-M** since D-323; registered as S-E's class until u-rev 6) | **BUILT, and revision 4's witness was inert.** The shape must have a 1-cover COEXISTING with a minimal 2-cover; `cover.rs`'s flat-list counterexample has no 1-cover, so the two notions coincide there and the mutant is an identity. **REBUILT AT u-rev 3 AS A POSITION A LEGAL GAME REACHES (MAJOR 8).** The witness this row carried until u-rev 2 held P1 = 8 stones with no stone on the origin — MEASURED refused by the rules on three counts at once, so it was a `ThreatState::apply` construction and never a position the SEARCH could be at. The rebuilt witness, with P2 to move: **P1** `(0,0)(1,0)(2,0)(3,0)` and `(-1,1)(-1,2)(-1,3)(-1,4)` and `(0,7)`, **P2** `(-2,0)(5,0)(-1,-1)(-1,6)` and `(4,-4)(5,-4)(-4,4)(-5,5)`. Nine P1 stones and eight P2 stones is rule 3's parity; the two arms share the empty corner `(-1,0)` and each is sealed at both far ends, so exactly one window per arm is hot. **MEASURED by replaying every ply through `GameState` and then querying the shipped solver:** `can_win_this_turn(P2,Two) = None` and `blocking_covers(P2,Two) = Minimal([One((-1,0)), Two{(-1,5),(4,0)}])` — the 1-cover coexisting with the minimal 2-cover, and minimum-cardinality drops the pair. Pinned by `crates/pistol-solver/tests/wp15b_mutation_witnesses.rs` |
795:| M6 | The overload return drops its `can_win_this_turn` guard | **mate**, not the differential gate's class ("not S-E" until u-rev 6) | **BUILT, AND REBUILT AT u-rev 3 AS A POSITION A LEGAL GAME REACHES (MAJOR 8).** The shape is unchanged and was never the defect: P1 holds one five-run sealed at one end, so exactly one cell completes it, and P2 holds three disjoint five-runs at rows 8 / 16 / 24 — 8 apart keeps every placement legal under rule 5 and 8 > 5 guarantees no shared window. What was wrong was the COUNT: P2 held 15 stones, and rule 3 gives P2 an even number at every turn boundary. The rebuilt witness, with P1 to move: **P1** `(0,0)(1,0)(2,0)(3,0)(4,0)`, the three seals `(-1,8)(-1,16)(-1,24)`, and seven further stones `(0,4)(3,4)(0,12)(3,12)(0,20)(3,20)(7,4)` placed where no window reaches four — fifteen in all; **P2** the seal `(-1,0)` and the three runs `(q,8)(q,16)(q,24)` for `q` in `0..5` — sixteen. The seven fillers are not decoration: P2's sixteen stones force P1's fifteen, and a witness that cannot be counted to cannot be replayed. **MEASURED by replaying every ply through `GameState` and then querying the shipped solver:** `can_win_this_turn(P1,Two) = Some(OnePly{ at: (5,0) })` while `unblockable_double_threat(P2,Two) = true`. Pinned by `crates/pistol-solver/tests/wp15b_mutation_witnesses.rs`. Its class is mate and not S-E, because the mutant RETURNS rather than emitting and S-E is blind at an `Impossible` node. **The witness is driven as a NON-PV DESCENDANT, never as a root**: the overload return is `!is_pv`-gated and ply 0 is always a PV node, so as a root the mutant does not fire at all and survives. Revision 5 changed this mutation's class and did not re-read the gate it then leaned on |
826:> thresholds under Staged; **(b) a differential gate** against …",
835:> letters it was one of, because it IS the differential gate and is named once, in
839:**THESE FOUR — the tactical suite under Staged (§8.3), the differential gate
852:NOW.** From u-rev 2 to u-rev 5 the differential gate above read *"S-E with the reduced
857:CAN BE SPECIFIED AND ONE CANNOT:** the differential gate's CRITERION is selected, but
1470:suite at pre-registered thresholds under Staged; **(b) a differential gate**
1504:letters, because the differential gate is named ONCE, in §8.2.** Executed in this unit
1508:the gate named once in §8.2 was **S-E**. It read "because S-E **is** the differential gate" in the present
1512:**S-E FELL in M3 round 1 and the differential gate's instrument since D-323 is S-M.**
1678:4. **The differential gate's instrument, and D-124's flip clause discharged.
1708:    differential gate (§8.2 — **S-M** since D-323, S-E until u-rev 6). The line records
1820:  the ledger classes to the differential gate, so this is the half MAJOR 8 raised for M4
(end M10)
===== M11 sibling unit sizes and declared u-revs =====
   329 docs/experiments/U1_gate_supersession.md
   827 docs/experiments/U2_node_protocol.md
   846 docs/experiments/U3_tier_t.md
   227 docs/experiments/WPQ_seed.md
   371 docs/experiments/section_owner_table.md
  2600 total
docs/experiments/U1_gate_supersession.md:15:**u-rev 2.** Carved from `docs/experiments/wp15b_design.md` §4 at `6feb40a`
docs/experiments/U2_node_protocol.md:15:**u-rev 3.** Carved from `docs/experiments/wp15b_design.md` §2, §3, §5 and §14 at
docs/experiments/U2_node_protocol.md:21:**u-rev 2 was a REPAIR, not a new carve.** It answered
docs/experiments/U2_node_protocol.md:27:**u-rev 3 is a REPAIR of u-rev 2's own review, not a new carve.** It answers
docs/experiments/U3_tier_t.md:15:**u-rev 4.** Carved from `docs/experiments/wp15b_design.md` §6, §10 and §12 items
docs/experiments/U3_tier_t.md:41:**u-rev 4**: the repairs answering `docs/experiments/wp15b_U3_REVIEW_urev3.md`,
docs/experiments/section_owner_table.md:5:**u-rev 5.** The map from `docs/experiments/wp15b_design.md` at `6feb40a`
(end M11)
===== M12 U4 lines that are self-referential status prose markers =====
lines naming a u-rev of this unit: 160
lines naming a wp15b_U4_REVIEW report: 31
lines carrying REPAIRED/WITHDRAWN/RE-CHECKED/MARKED AT: 34
(end M12)
```

## (a) Section span map — every top-level (##) and second-level (###) heading in U4, span to next heading of same-or-shallower depth (plus tail)

Command:
```bash
awk '
/^#{2,3} / {
  match($0,/^#+/); d=RLENGTH;
  n++; ln[n]=NR; dep[n]=d; txt[n]=$0
}
END{
  total=NR;
  for(i=1;i<=n;i++){
    e=total+1;
    for(j=i+1;j<=n;j++){
      if(dep[j]<=dep[i]){ e=ln[j]; break }
    }
    printf "%d\t%s\tspan=%d\n", ln[i], txt[i], (e-ln[i]);
  }
}' docs/experiments/U4_soundness_instrument.md
```

Output:
```
173	## U4-A. Lineage — what has attacked this unit's content, and at which revision	span=32
205	## 8. MATRIX M3 — the soundness instrument — **SELECTED AT u-rev 6: S-M (D-323)**	span=660
460	### 8.1 Why S-C fell	span=42
502	### 8.2 THE DIFFERENTIAL GATE — **S-M IS SELECTED (D-323)**; the text below is the RECORD of S-E	span=133
635	### 8.3 The other three gates, re-scoped — NAMED, NOT LETTERED	span=135
770	### 8.4 The mutation ledger, with witnesses	span=29
799	### 8.5 Floors, not printed counts	span=8
807	### 8.6 REJ-DEPTHPROOF, stated where it belongs	span=7
814	### 8.7 Gate wiring — **B3 REPAIRED, shape 2, D-316**	span=51
865	## 9. MATRIX M4 — the snapshot's config seam — **AXIS A SELECTED AT u-rev 7: N-E (D-329)**	span=482
1258	### 9.1 The five amendments the design made to N-A after that attack	span=77
1335	### 11.6 One thing this WP does NOT close	span=12
1347	## U4-T. The tests this unit registers	span=15
1362	## U4-M. What this unit measures	span=79
1422	### Cost	span=19
1441	## U4-Z. ADR lines this unit owes, the OPEN decision it stops on, and what is OPEN	span=446
1456	### B3, gate (b) — SETTLED. SHAPE 2 SELECTED (D-316); ITS RESIDUAL IS DISPOSED OF BY D-320.	span=183
1639	### ADR lines	span=104
1743	### OPEN — carried forward, not closed by the carve	span=144
```

## (b) Line counts and blockquote-line counts, three files

Command:
```bash
for f in docs/experiments/U4_soundness_instrument.md docs/experiments/matrix_M3_selection.md docs/experiments/matrix_M4_axisA_selection.md; do
  total=$(wc -l < "$f")
  bq=$(grep -c "^>" "$f")
  echo "$f  total_lines=$total  blockquote_lines=$bq"
done
```

Output:
```
docs/experiments/U4_soundness_instrument.md  total_lines=1886  blockquote_lines=727
docs/experiments/matrix_M3_selection.md  total_lines=216  blockquote_lines=15
docs/experiments/matrix_M4_axisA_selection.md  total_lines=149  blockquote_lines=15
```

## (c) Lines in U4 containing BOTH a D-3 ADR reference AND one of RECORD/record/REPAIRED/CLOSED/OPEN/SELECTED

Two forms run, both registered as equivalent readings of "a D-3 ADR reference":

Form 1 — D-3 followed by one or more digits (i.e. any D-3xx ADR id), matched with the status words on the same line via a single alternation regex:

Command:
```bash
grep -cE "D-3[0-9]+.*(RECORD|record|REPAIRED|CLOSED|OPEN|SELECTED)|(RECORD|record|REPAIRED|CLOSED|OPEN|SELECTED).*D-3[0-9]+" docs/experiments/U4_soundness_instrument.md
```

Output:
```
79
```

Form 2 — literal substring "D-3" AND one of the status words, same line, via a two-stage grep pipeline (AND, not OR):

Command:
```bash
grep -E "D-3" docs/experiments/U4_soundness_instrument.md | grep -cE "RECORD|record|REPAIRED|CLOSED|OPEN|SELECTED"
```

Output:
```
79
```

Both forms agree: 79.
