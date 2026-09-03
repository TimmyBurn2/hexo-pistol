# REVIEW — `docs/experiments/wp21_prereg.md` revision 7. FRESH CONTEXT. ROUND 5 of 5 (D-585, D-597). REMEDIES-ONLY.

**NAMED REVISION.** `f33593a770cd70ba01bba3266df8fcc4fa291821` on `dev`. The
document under review is byte-identical at `48a5802` (where this round was
first dispatched and died VOID under D-597) and at `f33593a`; the diff between
them (`git diff --stat 48a5802 f33593a`) touches only `docs/decisions.md` (+2,
D-597 itself) and `docs/experiments/arc3_ledger.md` (+23).

**DOES IT STILL MATCH HEAD?** **YES**, at the end of this review as at the
start: `git -C /home/tom/Projects/HeXO-AlphaBeta rev-parse HEAD` = `f33593a`
throughout, working tree clean.

**REVIEWED ON: Sonnet, per D-597.**

**SCOPE, per D-597.** Rounds 3 and 4 reproduced this document's arithmetic,
partition, digests and record; not re-derived here. This round adjudicates only
round 4's five MAJOR and eight minor findings against `docs/experiments/wp21_prereg_rev6_REVIEW.md`,
and whether the remedy diff (`git diff 735fc37 f33593a -- docs/experiments/wp21_prereg.md`,
93 changed lines) introduces anything new.

**WHAT I READ.** The named diff; the document whole; `wp21_prereg_rev6_REVIEW.md`
whole; `tools/wp21_tranche_config.py` whole; `tools/cold_label_check.py` (the
printing lines); `docs/book_v2_ledger.md:38-44`; `docs/experiments/wp21_throughput_prereg.md`
§3.5–3.6 (C1's criterion and consequence rows); `docs/experiments/wp21_label_cache_design.md:1`;
`docs/decisions.md` D-596, D-597; `docs/experiments/arc3_ledger.md` (the D-597
paragraph); `crates/pistol-arena/tests/wp21_tranche_config_tests.rs` (via the
suite run).

**WHAT I RAN, AND WHERE.** A detached worktree at `f33593a`,
`/home/tom/pistol-wt/prereg-r5b`, `CARGO_TARGET_DIR=/home/tom/pistol-wt/prereg-r5b/target`
per command, never exported, never in the live tree: `cargo build --workspace
--release --locked` (fresh `target/`); `cargo test -p pistol-arena --locked`
over `wp21_tranche_config_tests` (required) and, for due diligence, `cold_label_check_tests`,
`wp21_assemble_tests`, `label_cache_count_tests`; the shipped `cold_label_check.py`
against the existing dry-run capture at `/home/tom/pistol-runs/arc3r-dryrun/sweep/tranche-1/capture.txt`
(read-only, not regenerated); the two recorded `bash -c` lines pasted back
verbatim; `bash tools/governing_citation_check.sh`; `sha256sum` on every
digest the document cites and on the on-disk dry-run artifacts; `git diff --stat`,
`git log`, `grep`, `head`, `python3` for decode/compare. The `artifacts/`
directory is gitignored and absent from the worktree; its files were read from
the live tree's copies by path, never modified. **No tranche, no 218-opening
play, no registered workload was run.** Worktree removed at the end.

## VERDICT: **PASS**

**0 BLOCKING, 0 MAJOR, 0 minor.** All thirteen of round 4's findings (M1–M5,
m1–m8) are CLOSED by this round's own re-derivation; no new defect found.

---

## 1. DISPOSITION OF ROUND 4's FINDINGS

| id | round-4 finding | disposition | command / evidence |
|---|---|---|---|
| **M1** | T-A1/T-A2 quote a line the instrument never prints (`MISS`/`HIT` singular) | **CLOSED** | Doc §4:179-180,199 now read `MISSES record(s)` / `HITS record(s)`. `tools/cold_label_check.py:285,291,311,318` prints `{want.upper()} record(s)` = `MISSES`/`HITS`. My own run: `tools/cold_label_check.py --capture /home/tom/pistol-runs/arc3r-dryrun/sweep/tranche-1/capture.txt --binary target/release/pistol --engine-config configs/instrument_v0.toml --stride 1 --partition hits` → `cold_label_check: 17 of 17 sampled HITS record(s) agree byte for byte`, exit 0; `--partition misses` → `17 of 17 sampled MISSES record(s) agree byte for byte`, exit 0. Printed words match the quoted criterion exactly. `grep -n 'MISS record\|HIT record'` (singular) over the whole document → no hits |
| **M2** | §9's registered T-F body compare (`head -n` clause) is not what the record shows | **CLOSED** | §9 registers `cmp -s <(grep -v '^#' <SWEEP_DIR>/tf/capture-a.txt) <(grep -v '^#' <SWEEP_DIR>/tranche-1/capture.txt \| head -n $(grep -v '^#' <SWEEP_DIR>/tf/capture-a.txt \| wc -l))`. `grep -n head artifacts/arc3r_dryrun_sweep_0c4f3b4_v4.txt` (live-tree copy) → the recorded `bash -c` line, decoded (backslash-unescaped, `<DRY>` for the real path), is byte-for-byte the same command. Digest of that log file: `c91dd08d00ecd319cc52460fb387ec39c851e33fb7f554d2bee92f1814d9ebda` = §9.1's cited sha |
| **M3** | §5's checker reads the `cmp -s` line's presence, not its exit status | **CLOSED** | §5 now reads: *"the `cmp -s` `exit=0` line of the comparison … appears BEFORE the first block … a block whose pass 2 carries the flag with no such line above it, or with only a non-zero one, is a VOID tranche"*. A FAILED comparison (only an `exit=1` line, or none) now has an explicit disposition |
| **M4** | T-F carries no registered consequence | **CLOSED** | T-F's row (§4:184) now ends: *"REGISTERED CONSEQUENCE: a T-F failure is the sibling's C1 class — THE SWEEP STOPS, before wave one on the first half and before wave two on the second, and no tranche is re-run to cure it"*. `wp21_throughput_prereg.md:233` C1's consequence: *"THE SWEEP STOPS, not this study … `wp21_prereg.md` may not run until it is closed"* — same defect class (a capture instrument whose output is not a pure function of its inputs), same disposition (stop, not re-run) |
| **M5** | `docs/book_v2_ledger.md`'s row names revision 5 | **CLOSED** | `grep -n wp21_prereg docs/book_v2_ledger.md` → `:42 \| 13 \| 3487 \| 13..3499 \| … \| docs/experiments/wp21_prereg.md revision 7 \|`. Matches the preamble's *"names this revision"* |
| **m1** | three recorded `bash -c` lines do not run as printed (unquoted in the log) | **CLOSED** | Pasted the two recorded lines back verbatim into a shell in the worktree (paths substituted for `<DRY>`): `bash -c cmp\ -s\ \<\(grep\ -v\ \'\^#\'\ …\)…` → exit 0; `bash -c tools/cold_label_check.py\ --capture\ …\ \|\ /usr/bin/grep\ -o\ \'of\ which\ \[0-9\]\*\ are\ MISSES\'` → prints `of which 17 are MISSES`, exit 0. Both run as printed |
| **m3** | four D-423 restatements (arena_ basename/gate-6 courtesy; §6.1 "a pointer" carrying the sibling's clauses whole; the revision-history paragraph under a "nowhere here" preamble; the duplicated T-F sub-range paragraph) | **CLOSED** | `grep -n 'basename\|gate 6\|pointer\|SUB-RANGE FOR T-F IS REGISTERED HERE'` over the document → no hits. §9 step 0's comment now ends at *"no second validator is registered"*; §6.1 is four sentences deferring everything but one fact to the sibling; the trailing revision-history paragraph is deleted from the file (confirmed: file ends at limb 5's file listing) |
| **m4** | two D-424 constraint-free sentences (*"Twenty is a round number…"*, *"it costs nothing"*) | **CLOSED** | Both sentences lived inside the two paragraphs m3 confirms deleted; `grep -n 'round number\|costs nothing'` → no hits |
| **m5** | INTERRUPTED vs. two-consecutive-VOIDs is unstated | **CLOSED** | §5 now reads: *"An INTERRUPTED tranche does not count toward §4's two consecutive VOIDs, which counts criterion VOIDs only — the fault a reboot names is not the seat's or the instrument's."* Unambiguous |
| **m6** | a VOID tranche one leaves the §4.4 referent pair's registered paths naming the void run | **CLOSED** | §4.1 now reads: *"…the sibling's §4.4 referent half; if tranche one is VOID, the pair's paths are its passing `-run<k>`'s"* |
| **m7** | the pinned generator writes a pointer to the deleted `config_check.sh` step | **CLOSED** | `grep -n config_check tools/wp21_tranche_config.py` (worktree) → no hits; the generated-config docstring now ends *"no other validator is registered for it"*. `cargo test -p pistol-arena --locked --test wp21_tranche_config_tests` → **18 passed; 0 failed**. `sha256sum tools/wp21_tranche_config.py` → `586b4e7fad77c7577073e78b2ce313bde84003fa71eb5b507c2c156d62932afb` = §8's cited digest |
| **m8** | limb 2's *"which the generator's own suite pins"* over-reads the suite | **CLOSED** | §9.1 limb 2 now reads: *"The `--tranche` and `--skip/--take` forms come from the same `document()` template (`tools/wp21_tranche_config.py`) and differ from it in the two integers and the header comment only — a `diff` of any two written forms shows nothing else"* — cites the template, no longer claims suite coverage it lacks |

**Score: 13 of 13 CLOSED, no residuals.**

---

## 2. CITATION-AND-DIGEST CHECK

| citation | document's claim | tree at `f33593a` | match |
|---|---|---|---|
| self revision | revision 7 (title, `docs/book_v2_ledger.md:42`) | file title reads revision 7 | ✓ |
| sibling revision | `wp21_throughput_prereg.md` (named, no number inline in this doc) | `head -1` → *"PRE-REGISTRATION, revision 6"* | ✓ (no stale number cited) |
| design revision | `wp21_label_cache_design.md` revision 10 (§1, §8) | `head -1` → *"DESIGN, revision 10"* | ✓ |
| tree slot | `0c4f3b4` for every binary/instrument; `git diff --stat 0c4f3b4 f33593a -- crates configs` empty | ran it: empty | ✓ |
| `target/release/pistol` | `78a7600adcf0…` | my own fresh `--release --locked` build: `78a7600adcf0…` | ✓ |
| `target/release/arena` | `a1a405cb44d2…` | mine: `a1a405cb44d2…` | ✓ |
| `target/release/corpus-check` | `efbb76b643fb…` | mine: `efbb76b643fb…` | ✓ |
| `tools/cold_label_check.py` | `6386d6bfe2ea…` | `sha256sum`: `6386d6bfe2ea…` | ✓ |
| `tools/wp21_tranche_config.py` | `586b4e7fad77…` | `sha256sum`: `586b4e7fad77…` | ✓ |
| `tools/label_cache_count.py` | `1a890b5331c3…` | `sha256sum`: `1a890b5331c3…` | ✓ |
| `tools/wp21_assemble.py` | `a367d8475416…` | `sha256sum`: `a367d8475416…` | ✓ |
| `arc3_opening_prefix_fold.txt` | `b6d4751e2459…` | `sha256sum` (live tree copy): `b6d4751e2459…` | ✓ |
| dry-run log | `arc3r_dryrun_sweep_0c4f3b4_v4.txt`, sha `c91dd08d00ec…` | `sha256sum` (live tree copy): `c91dd08d00ec…` | ✓ |
| stand-in config | sha `00b95f90c5bc…` | `sha256sum` of both on-disk generated files: `00b95f90c5bc…` (both) | ✓ |
| dry-run head line | `== dry run at 73979e767569…, Thu Sep 3 07:34:12 AM UTC 2026, rustc 1.98.0…, cargo 1.98.0…` | `head -1` of the log: identical | ✓ |
| `73979e7`'s tree | docs-only above `0c4f3b4` | `git diff --stat 0c4f3b4 73979e7 -- crates tools configs` empty | ✓ |
| citation gate | — | `bash tools/governing_citation_check.sh` → `docs/experiments/wp21_prereg.md: 26 citation(s) checked, 0 unreproduced`, all nine governing docs green, `DESIGN_CITATION_CHECK_DONE` | ✓ |
| coverage suites | four instruments' digests are pinned by their tests | `wp21_tranche_config_tests` 18/18, `cold_label_check_tests` 11/11, `wp21_assemble_tests` 11/11, `label_cache_count_tests` 5/5, all green in worktree | ✓ |

Every revision number and digest this document cites matches the tree at
`f33593a`.

---

## 3. NEW DEFECTS INTRODUCED BY THE REMEDIES

**None found.** Specifically checked and rejected:

- The `MISSES`/`HITS` fix is applied everywhere the criterion is quoted or
  worked (§4:179,180,199; §9.1's printed-lines block); no stray singular form
  remains anywhere in the document (`grep -n 'MISS record\|HIT record'` → empty).
- The `head -n` clause added to §9's T-F body compare is exercised by the
  re-taken dry run (the log's `bash -c` line decodes to the identical command)
  and by my own paste-back, both exit 0.
- M3's new wording (*"exit=0"* / *"or with only a non-zero one"*) is decidable
  from the log alone and does not create a new ambiguous case: a `cmp -s`
  `exit=0` line anywhere before the flagged pass-2 block still satisfies it,
  which is the same reading round 4 approved for the un-residualed half of
  the rule.
- M4's new consequence sentence is consistent with the sibling's own C1
  consequence row (§3.6) in disposition (stop, no cure-by-rerun) and does not
  contradict T-F's void-rule carve-out (T-F's failure is explicitly excluded
  from the ordinary per-tranche VOID/re-run rule).
- m3/m4's deletions removed exactly the four restatements and two
  constraint-free sentences named, and nothing else load-bearing: the
  T-F sub-range parameters (skip 13, take 20) remain stated once, in the T-F
  criterion row itself.
- m7's one-line generator change is scoped as promised (`git diff 735fc37
  f33593a -- tools/wp21_tranche_config.py` is 3 lines, the docstring line
  only) and re-digested correctly in §8; the coverage suite still passes in
  full.
- m8's rewording is a citation change only; the underlying fact (one
  `document()` template, forms differ in two integers and the header/range
  comment) is unchanged and was already reproduced in round 4's re-derivation
  table.

No criterion in the remedied text can pass vacuously, no registered command
failed to run, and no remaining claim in the diff is contradicted by the tree.

---

**Worktree.** `/home/tom/pistol-wt/prereg-r5b`, detached at `f33593a`, removed
at the end of this review. Nothing gitignored in it needed exporting.
