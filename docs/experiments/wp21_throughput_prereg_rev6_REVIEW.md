# REVIEW-design (fresh context) — `wp21_throughput_prereg.md` revision 6, ROUND 5 of 5 (D-585), REMEDIES-ONLY per D-597

**NAMED REVISION.** `docs/experiments/wp21_throughput_prereg.md` as of commit
**`f33593a770cd70ba01bba3266df8fcc4fa291821`**, branch `dev` (byte-identical to
`48a5802`, where this round was first dispatched and died VOID of a harness rate
limit per D-597; this is that same round, not a sixth).

**REVIEWED ON: Sonnet, per D-597.**

**DOES IT STILL MATCH HEAD?**

```
$ git -C /home/tom/Projects/HeXO-AlphaBeta rev-parse HEAD
f33593a770cd70ba01bba3266df8fcc4fa291821
$ git -C /home/tom/Projects/HeXO-AlphaBeta diff --quiet f33593a HEAD -- \
    docs/experiments/wp21_throughput_prereg.md docs/experiments/wp21_prereg.md \
    docs/experiments/wp21_label_cache_design.md crates tools configs; echo $?
0
```

**YES** — HEAD equals the reviewed revision exactly; nothing moved during this
review.

**SCOPE, per D-597.** This round adjudicates only the REMEDIES of round 4
(`wp21_throughput_prereg_rev5_REVIEW.md`'s B1, M1–M3, m1–m13) as applied in the
diff `735fc37..f33593a`. Rounds 3 and 4 already reproduced the soundness
argument, the digests and the record; none of that is re-derived here.

**WHERE.** A detached worktree `/home/tom/pistol-wt/throughput-r5b` at
`f33593a`, `CARGO_TARGET_DIR=/home/tom/pistol-wt/throughput-r5b/target` per
command, scratch under `scratch/` there; nothing in the live tree was written
but this file. The worktree is removed at the end (it held no `artifacts/` or
`sessions/`). Nothing of lever A's registered settings was run: the largest
runs here are the registered harness at N = 1 and N = 2 on the dry-run's own
stand-in report (`/home/tom/pistol-runs/arc3r-dryrun/throughput-v3/report.txt`,
read-only) at `--label-nodes 2000`, each under a second. No `pkill` was issued.

**WHAT I READ.** `git diff 735fc37 f33593a -- docs/experiments/wp21_throughput_prereg.md`;
the document in full; `wp21_throughput_prereg_rev5_REVIEW.md` in full;
`crates/pistol-arena/src/capture.rs`'s `asked_prefixes`;
`/home/tom/pistol-runs/arc3r-leverA/lever_a_harness.sh` and `lever_a.sh`;
`artifacts/arc3r_dryrun_throughput_0c4f3b4_v3.txt` and
`/home/tom/pistol-runs/arc3r-dryrun/throughput-v3/`; `head -1` of
`wp21_prereg.md` and `wp21_label_cache_design.md`; D-597 in `docs/decisions.md`.

---

## VERDICT: **FAIL — 0 BLOCKING, 1 MAJOR (NEW), 0 minor**

Every one of round 4's 1 BLOCKING, 3 MAJOR and 13 minor findings is CLOSED —
B1's fix landed nearly verbatim, M1–M3 and every minor are closed by
re-derivation below. But the remedy that closes B1/m2/m9/m13 together — the
newly-registered lever-A **driver** — states its own file count wrong, twice,
and the number is checkable from the very schedule the document registers two
sections earlier: **NEW-1** (MAJOR). Round 4's own standard for MAJOR (its
MAJOR 2: a restated number that does not match its source) applies identically
here, so this round does not close clean.

---

# DISPOSITION OF ROUND 4'S REMEDIES

| # | round-4 finding | status | command / evidence at `f33593a` |
|---|---|---|---|
| **B1** | §4.5's 5% clause divides tranche one's wave-one wall by its between-the-waves wall; `1/c(N)` reaches the verdict on contention, not bookkeeping | **CLOSED** | `grep -n "seconds=" docs/experiments/wp21_throughput_prereg.md` → only §8's C4-referent row (`capture1 seconds=657`, unrelated to §4.5). §4.5 now reads *"on the play-pass pair alone… Tranche one's pair contributes byte-identity and nothing about its wall: its uncached capture runs in wave one under N-way contention and its re-capture alone between the waves, so their quotient carries `c(N)` and would fail or pass on contention rather than bookkeeping."* No run-log `seconds=` is read as a lever-B wall anywhere in the document. `1/c(N)` cannot reach the verdict: tranche one no longer contributes a wall term to §4.5 at all, only byte-identity (unchanged, §4.4). This is round 4's own unexecuted fix, landed almost word for word. |
| **M1** | sibling cited at revision 5; tree held 6 | **CLOSED** | `head -1 docs/experiments/wp21_prereg.md` → `# WP-2.1 — the production label sweep. RUN REGISTRATION, revision 7.` §GOVERNING cites *"`wp21_prereg.md` revision 7"* — match. (Sibling advanced 6→7 between rounds, so the citation was re-bumped correctly, not merely repaired to the stale number.) Design doc: `head -1 docs/experiments/wp21_label_cache_design.md` → `revision 10`, and the document cites *"revision 10"* — match. |
| **M2** | §1 restates the sibling's tranche addends at superseded seconds, under a ONE LINE saying the wall is not restated | **CLOSED** | `grep -n "11 01[0-9]\|1 4[0-9][0-9] s\|13 927" docs/experiments/wp21_throughput_prereg.md` → **no matches**. §1 now reads *"as fractions of `wp21_prereg.md` §3's tranche (whose seconds are that document's and are not restated): capture **79.1%**; play 10.4%; replay 10.1%; the cold check 0.4%."* No seconds figure of any kind survives, not even the tranche total 13 927 — stricter than round 4's suggested fix (drop four figures or requote); all were dropped. |
| **M3** | §3.4's tie rule has two readings over a field of five, and the 5% base is unstated | **CLOSED** | New rule: *"the SMALLEST N whose median throughput is at least 95% of the highest median throughput over the field… The 5% is of the highest median."* Hand test (`python3`, own script) on round 4's two outcomes: `{4:100, 8:104, 16:108}` → highest=108, threshold=102.6, candidates={8,16}, **selected N=8**, one reading. `{8:100, 16:105.2}` → highest=105.2, threshold=99.94, candidates={8,16}, **selected N=8**, one reading. Base is now always "the highest" by name; no second reading exists. |
| **m1** | `rc` printed and nothing reads it | **CLOSED** | C2's row: *"every `leverA:` line's `rc` is 0 — a setting-rep whose line carries `rc 1`, or no line (the harness's VOID), is VOID under C2's consequence."* `rc` remains directly on the printed line in the receipt (verified live, see m10 below: `... rc 0 wall_s ...`), so the consequence is checkable by direct inspection, not a judgment call. |
| **m2** | C1/C2 have no printed command over the 45 files; C2's referent undated | **CLOSED for the command and the referent** (see NEW-1 for the file count itself) | Driver (`lever_a.sh`) prints both loops verbatim in §7: `for f in "$OUT"/cap-N*-rep*-p*.txt; do … cmp -s …` (C1) and the analogous record-count loop (C2); `ASKED=$(awk '…' "$REPORT")` derives C2's referent off the report with no capture. Re-derived live: `awk` over `/home/tom/pistol-runs/arc3r-dryrun/throughput-v3/report.txt` → `ASKED=32`; independently, `grep -v '^#' pair/cap-N1-rep1-p1.txt \| grep -c .` → `32` and the same for `leverA/cap-N1-rep1-p1.txt` → `32`. Match. |
| **m3** | the divides-16 guard cannot fail | **CLOSED** | `grep -n "must divide\|divide the tranche count" docs/experiments/wp21_throughput_prereg.md` → no matches in §3.4. §3.2 keeps the FACT ("every member of this field divides 16") as a design-element statement, not a §3.4 guard. |
| **m4** | C3 stated at two scopes; "shares no core" names the wrong N | **CLOSED** | `grep -n "REFUSED\|exceeds C3"` → C3's REFUSED clause now appears exactly once (§3.5's table row); §3.4 only points to it (*"C3's refusal (§3.5) removing a setting from the field"*). Reason now reads *"bought nothing over N = 4 at `c = 1`"*. |
| **m5** | C4 cites "the round-2 review's A7"; the number is A3's | **CLOSED** | `grep -n "round-2 review's A"` → *"the round-2 review's A3 re-derivation, 1.0042"*. |
| **m6** | §3.1's transfer evidence is the superseded run's (opening 0, not the registered opening 3) | **CLOSED** | §3.1: *"the registered dry run's opening 3 plays move-for-move identically to the pilot's games 6–7 across the toolchain (§7.1)."* |
| **m7** | the two `bash -c \| sort -u` record lines are not executable as printed | **CLOSED** | Pasted the record's own line (only `<SCRATCH>` substituted) into a live shell with a synthetic 6-field tab corpus: `bash -c /usr/bin/grep\ -v\ \'\^#\'\ $SCRATCH/corpus.txt\ \|\ cut\ -f5\ \|\ LC_ALL=C\ sort\ -u\ \|\ wc\ -l` → correctly counted distinct field 5 values (2) at exit 0; field 6 → 1 at exit 0. It runs the intended `grep\|cut\|sort\|wc` pipeline, not the broken `bash -c /usr/bin/grep` shape round 4 found. The dry-run log itself independently shows the same escaped line printing 16/16 at exit 0, matching `label_cache_count`'s fold counts. |
| **m8** | the record carries an unregistered `config_check.sh` live-tree run | **CLOSED** | `grep -n "config_check"` over the document, the dry-run log, and both scripts → **no matches anywhere**. The command is gone from §7.1's record (the diff shows the two `config_check.sh` lines deleted). |
| **m9** | an unregistered driver at revision-4's shape (ran `config_check.sh`, no digest) sits beside the harness | **CLOSED** | The file at `/home/tom/pistol-runs/arc3r-leverA/lever_a.sh` now IS the registered content: `sha256sum` = `ca07d4a6…`, matching §8's digest and byte-identical (`diff`, exit 0) to §7's second bash block. No `config_check.sh` inside it (grep, above). |
| **m10** | the harness's unregistered invocation shapes (N = 0, bad tail word) were noted but not enforced/registered | **CLOSED** | Built `arena`/`pistol` in-worktree (digests below reproduce §8's exactly). Live runs: `N=0` → `leverA: VOID: N \`0\` is not a count of at least 1`, **exit 2**, no counts line. Tail word `--bogus` → `leverA: VOID: tail word \`--bogus\` is not registered`, **exit 2**, no counts line. Normal `N=1`: `leverA: N 1 rep 1 cache off rc 0 wall_s 0.664 records 32 asks 32 s_per_label 0.020737 s_per_ask 0.020737 throughput 48.224`, exit 0. Normal `N=2`: `leverA: N 2 rep 1 cache off rc 0 wall_s 0.863 records 32 asks 32 s_per_label 0.026967 s_per_ask 0.026967 throughput 74.164`, exit 0. Every field §3.3 (`wall_s`, `records`, throughput) and §4.5 (`s_per_ask`, `s_per_label`) read is present. |
| **m11** | "~2.3 min" should read "~2.2" (`152 × 0.885445 = 134.6 s`) | **CLOSED** | `grep -n "2.2 min\|2.3 min"` → both occurrences (§4.4, §5) now read `~2.2 min`; none read `~2.3`. |
| **m12** | the idle-box rule names lever A only, not the §4.5 play-pass pair | **CLOSED** | §7: *"The box is otherwise idle for lever A **and for the §4.5 play-pass pair**… (the driver prints what it saw)."* |
| **m13** | the fifteen `leverA:` lines have no registered receipt | **CLOSED** | §7: *"the fifteen `leverA:` lines, the C1 and C2 readings and the `ps`/loadavg lines it prints are the receipt, `artifacts/arc3r_leverA_<commit>.txt`, whose digest the amendment filling the sibling's slot carries (hard rules 6 and 8)."* Path registered; consumer of the digest named. |

**Tally: 17 of 17 round-4 findings CLOSED.**

---

# CLOSING CHECK 1 — CITATIONS AND DIGESTS AGAINST THE TREE AT `f33593a`

| claim | command | result | match |
|---|---|---|---|
| sibling cited at revision 7 | `head -1 docs/experiments/wp21_prereg.md` | `RUN REGISTRATION, revision 7` | ✓ |
| design cited at revision 10 | `head -1 docs/experiments/wp21_label_cache_design.md` | `DESIGN, revision 10` | ✓ |
| `tools/wp21_tranche_config.py` digest `586b4e7f…` | `sha256sum tools/wp21_tranche_config.py` (in-worktree) | `586b4e7fad77c7577073e78b2ce313bde84003fa71eb5b507c2c156d62932afb` | ✓ |
| `tools/label_cache_count.py` digest `1a890b53…` | `sha256sum tools/label_cache_count.py` | `1a890b5331c302cf97372603e7ec21a41b08e6131390d92b78b0168bc77c1e18` | ✓ |
| binaries unchanged since the toolchain-anchor commit | `git diff --stat 0c4f3b4 f33593a -- crates configs` | empty | ✓ |
| built `arena`/`pistol` digests reproduce §8's | `cargo build --release --locked -p pistol-cli -p pistol-arena` (own `CARGO_TARGET_DIR`); `sha256sum` | `arena a1a405cb…`, `pistol 78a7600a…` — both exact matches | ✓ |
| lever-A harness digest | `sha256sum /home/tom/pistol-runs/arc3r-leverA/lever_a_harness.sh` | `03e5b7eb…` = §8's, and `cmp` against the §7 block extracted by `awk` → exit 0 | ✓ |
| lever-A driver digest | `sha256sum /home/tom/pistol-runs/arc3r-leverA/lever_a.sh` | `ca07d4a6…` = §8's, and `cmp` against the §7 block extracted by `awk` → exit 0 | ✓ |
| dry-run log digest | `sha256sum artifacts/arc3r_dryrun_throughput_0c4f3b4_v3.txt` | `64573b2e…` = the document's citation | ✓ |
| citation gate | `bash tools/governing_citation_check.sh` | `wp21_throughput_prereg.md: 16 citation(s) checked, 0 unreproduced` | ✓ |
| ADR lines exist | `grep -c '^D-<n>:' docs/decisions.md` for 423, 424, 576, 581, 584, 586, 587, 588, 589, 595 | 1 each | ✓ |

Every revision number and digest this document cites matches the tree at
`f33593a`.

---

# CLOSING CHECK 2 — DOES ANY REMEDY INTRODUCE A NEW DEFECT

## NEW-1 (MAJOR) — **The newly-registered driver states its own file count wrong, in two places, and the correct number is derivable from the schedule the document itself registers two sections earlier.**

§3.5's C1 row (new text): *"§7's driver `cmp`s all **forty-five** against
`cap-N1-rep1-p1.txt`"*. §7's driver-block comment (new text): *"then C1 and C2
over all **forty-five** files."*

§3.2 registers the schedule these files come from: **3 reps**, each a full pass
over the field **{1, 2, 4, 8, 16}** (*"in the fixed order 1, 2, 4, 8, 16 and
then that order twice more"*), and the harness spawns exactly **N** processes
per setting-rep (`for i in $(seq 1 "$N")`), one capture file each. The
document's own driver script's C1 loop globs every one of them:
`for f in "$OUT"/cap-N*-rep*-p*.txt`.

**Reproducer**, arithmetic and an independent empirical simulation of the
driver's exact glob against the exact registered schedule (no `arena` run, no
setting above N = 2 executed anywhere in this review):

```
$ python3 -c "print(3*(1+2+4+8+16))"
93
$ mkdir -p scratch/countcheck && cd scratch/countcheck
$ for rep in 1 2 3; do for N in 1 2 4 8 16; do for i in $(seq 1 "$N"); do \
    touch "cap-N${N}-rep${rep}-p${i}.txt"; done; done; done
$ total=0; for f in cap-N*-rep*-p*.txt; do total=$((total+1)); done; echo "$total"
93
```

The registered schedule produces **93** capture files, not 45 — `3 × (1+2+4+8+16)
= 93`, confirmed by literally recreating the file-naming scheme and running the
driver's own glob over it. `45` does not correspond to any subset of this
schedule I can construct (not `5 settings × 3 reps = 15`, not `N`-only-per-rep
`1+2+4+8+16 = 31`); it appears to be carried over from round 4's own report,
which used "45 files" as an unverified estimate (`wp21_throughput_prereg_rev5_REVIEW.md:76,291`)
when describing what round 4's own partial reproduction (N = 1, N = 2 only)
would need, and round 5's remedy then wrote that number into the document and
into the driver's own comment as if derived, rather than re-deriving it from
§3.2's registered field and rep count — the transcribe-not-derive shape.

**Consequence, and why it is MAJOR and not BLOCKING.** The C1 *criterion* is
unaffected: the driver's loop counts dynamically (`total=$((total+1))`) and
will correctly report `C1: 0 of 93 files differ…` when run, so no criterion is
weakened and no comparison is skipped. But the number is now stated twice in
the document (the C1 row, the §7 comment) and both are wrong the same way —
exactly the shape D-423 names a defect waiting: a reader checking the eventual
receipt against "forty-five" will find "93" with nothing to explain the
mismatch, which is a live source of a false "the driver didn't run as
registered" alarm on the actual run. This is round 4's own MAJOR 2 pattern (a
restated count that does not match its source) recurring in the remedy that
was supposed to close a different finding.

**Fix (unexecuted, two numbers).** Replace "forty-five" with "ninety-three" in
both places, or better, state it symbolically (*"3 × Σ{1,2,4,8,16} = 93"*) so a
future change to the field or rep count cannot leave a second stale digit
behind.

No other new defect was found: the M3 rule was hand-tested edge to edge (a
tie at the maximum, a tie at a non-maximum pair) and returns one answer both
times; the B1 fix removes tranche one's wall contribution to §4.5 without
touching any number the study's cost table (§5) uses; the m10 refusals were
run live and exit 2 with no counts line, matching the registered contract
exactly; the m7 fix was executed, not just read.

---

# HEAD AT THE END OF THIS REVIEW

```
$ git -C /home/tom/Projects/HeXO-AlphaBeta rev-parse HEAD
f33593a770cd70ba01bba3266df8fcc4fa291821
```

Unchanged from the reviewed revision. The worktree
`/home/tom/pistol-wt/throughput-r5b` is removed after this file is written;
its only products were the release build of `pistol-cli`/`pistol-arena`, the
harness/driver probes under `scratch/`, and the extracted §7 blocks — none an
artefact of record, no `artifacts/` or `sessions/` created.
