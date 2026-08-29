# WP-1.5d (B) PRE-REGISTRATION REVISION 4 — SCOPED RE-REVIEW

## Header

| field | value |
|---|---|
| artefact | `docs/experiments/wp15d_b_prereg.md` revision 4 |
| named revision | **`d205a5f`** |
| matches HEAD? | **YES** — `git rev-parse HEAD` = `d205a5f02ee4496268547180cb1c71f6f4145999`, working tree clean, branch `dev` |
| prior revision | `a62ffca` (revision 3) — FAILED, 0 BLOCKING / 6 MAJOR / 7 MINOR |
| prior verdicts | rev 1 (`0dcd0db`) FAIL 3B/7MAJ/4MIN; rev 2 FAIL 1B/3MAJ; rev 3 FAIL 0B/6MAJ/7MIN |
| freshness | fresh context; I authored no revision of this document and no prior review of it |
| scope | D-489 as amended by D-490: the rev-3 review §7's twelve mechanical remedies + the D-488 row replacement. Nothing else. |
| worktree | `/home/tom/Projects/pistol-wt-rev4` (detached at `d205a5f`), own `CARGO_TARGET_DIR=…/target-rev4`, on `/home` |
| digests verified | **9 of 9 cited digests recomputed and matching** (see part (c)) |
| evidence exported | `artifacts/wp15d_b_rev4_verification_v1.txt` `3daf7b52c39330442577cf5e2c33296d81339915146b69ad22a5937118c9f94f`; `artifacts/wp15d_b_rev4_timeout_probe_execution_v1.txt` `26808d6cd7589df521e878e87a56402e93864fe465c6cbb13f2368426c61d7e9` |

---

# VERDICT: **PASS**

**0 BLOCKING, 0 MAJOR, 0 MINOR.**

All thirteen scoped items landed as the rev-3 reviewer stated them — including
every part of the multi-part ones, which is where revision 3 failed. Nothing
moved outside the list. Every new assertion I could check is true, and the two I
was told to distrust most — the file-line arithmetic and the registered probe
command — reproduce exactly.

**The document as it stands would let a launching session run the experiment and
read its verdict without inventing a decision.** Every arena verdict token, every
arena exit code, every attribution-checker exit code and every registered gate is
routed to a decision, with a genuine import-by-reference catch-all for the
remainder.

One redundancy I considered and deliberately did **not** raise as a finding is
recorded in full at the end of part (c), with the reasoning, so that the decision
is auditable rather than silent.

---

# PART (a) — THE THIRTEEN-ROW CLOSURE LEDGER

Line numbers are revision 4's (`d205a5f`). Each row is judged against the rev-3
review's own wording of the remedy, not revision 4's restatement of it.

| # | remedy (rev-3 review §7) | status |
|---|---|---|
| 1 | **D-489 row replacement** — mislabeled `§4B` row → the two D-488 receipts by digest | **CLOSED** |
| 2 | NEW 1 — delete the `(100, 120]` clause | **CLOSED** |
| 3 | NEW 2 — `invalid_forfeit` + arena exit 1 + import-by-reference row + "fourth token" | **CLOSED (all four parts)** |
| 4 | NEW 3 — restate §5's attribution row over (i)/(ii)/(iii) | **CLOSED** |
| 5 | NEW 4 — restate `:369-370` **and** `:390-392` | **CLOSED (both parts)** |
| 6 | NEW 6 — "openings `1500..1549`, i.e. file lines `1562..1611`" | **CLOSED** |
| 7 | NEW 7 — delete "against §3's own worst single search", point at §7.7 | **CLOSED** |
| 8 | NEW 8 — delete the duplicate second-instrument paragraph | **CLOSED** |
| 9 | NEW 9 — derive the wall estimate from `wall_ms 1049514`; separate the two checker passes | **CLOSED (both parts)** |
| 10 | NEW 10 — name `wp15d_b_dryrun_v1.txt` with digest **and** add the falsification digest | **CLOSED (both parts)** |
| 11 | NEW 11 — "15-, 31- and 35-stone" | **CLOSED** |
| 12 | NEW 12 — cite `tools/wp15b_attribution_check.py:284` | **CLOSED** |
| 13 | NEW 13 — register the verbatim command and the replication procedure | **CLOSED (both parts, and exceeded)** |

## Item 1 — the D-489 row replacement — CLOSED

The mislabeled row (rev 3: *"game 2's `result` flipped `p1_win`→`p2_win`"*) is
gone. `:384` now reads:

> | **a self-consistent REFEREE INVERSION, reproduced TWICE INDEPENDENTLY and
> exported re-runnable** — the launching session's
> (`artifacts/wp15d_b_referee_inversion_repro_v1.txt` sha256 `b587b8c6…`, its
> mutated report `c532be4f…` and replay `1c2d7967…`, instrument `d14b0136…`) and
> the rev-3 reviewer's, built without knowledge of it
> (`artifacts/wp15d_b_rev3_criterion_reproduction_v1.txt` sha256 `978bb06c…`,
> instrument `b59f7b49…`) | … | **FAILS (ii)** |

Both receipts are cited **by digest**, as D-489 requires, and all six digests
verify (part (c)). The narrative that rested on the mislabeled row is withdrawn
at `:389-402`, and the lost `/tmp` scratch is **not** reconstructed — exactly
D-489's ruling.

I checked the document's own claim that **"No claim in this document now rests on
the withdrawn account"**:

```
$ /usr/bin/grep -n "attempted reproduction\|did not reach the term\|flipping .result. alone\|partial mutation" \
    docs/experiments/wp15d_b_prereg.md
391:reproduction that "did not reach the term it aimed at", and diagnosed the failure
392:as a partial mutation.
```

The only two hits are **inside the withdrawal paragraph itself**, describing what
is being withdrawn. The claim is true.

**Coverage was not reduced by the swap.** The replaced row demonstrated term (i);
the new row demonstrates term (ii). Term (i) is still demonstrated by the
surviving move-list-reordering row at `:385` ("warm exits **2** on a structural
invariant … **FAILS (i)**"). Both terms retain a demonstration.

## Item 2 — NEW 1 — CLOSED

```
$ /usr/bin/grep -c '100, 120' docs/experiments/wp15d_b_prereg.md
0
$ /usr/bin/grep -c 'wp15d_b_rev1_reviewer_probe' docs/experiments/wp15d_b_prereg.md
0
```

The figure is deleted, not re-sourced — the remedy's own parenthesis ("Keeping the
figure instead is substantive") is respected. `:602-608` now carries the remainder
on the registered artifact alone:

> every counter, so the whole region above it is an inert shelf. **The registered
> artifact is what says so and nothing else is cited for it**: `capped_rows=0` at
> K = 128.

## Item 3 — NEW 2 — CLOSED IN ALL FOUR PARTS

This is the remedy whose predecessor slipped through revision 3 by being closed in
one of three parts, so I checked each part separately.

- **`invalid_forfeit` row** — `:530`, present.
- **arena exit 1 row** — `:531`: "arena exit 1 (`RUN_FAILED`, `src/bin/arena.rs`) | the run is VOID, exactly as exit 2 is."
- **import-by-reference row** — `:533`: "`arena_report_aborted`; a pre-game refusal with NO REPORT AT ALL; any attribution-checker exit other than the ones routed above | **WP-1.6 §5's rows govern, imported by reference and not restated here**".
- **"fourth token" → "one of five"** — `:529`: "**one of the arena's five tokens** (`crates/pistol-arena/src/sprt.rs`)".

```
$ /usr/bin/grep -c 'invalid_forfeit' docs/experiments/wp15d_b_prereg.md   # 2
$ /usr/bin/grep -c 'RUN_FAILED' docs/experiments/wp15d_b_prereg.md        # 1
$ /usr/bin/grep -c "arena's fourth token" docs/experiments/wp15d_b_prereg.md  # 0
```

And "five" is the true count, verified at source:

```
$ /usr/bin/grep -n 'Verdict::[A-Z].* => "' crates/pistol-arena/src/sprt.rs
189: Verdict::H0 => "h0",
190: Verdict::H1 => "h1",
191: Verdict::InconclusiveAtGameCap => "inconclusive_at_game_cap",
192: Verdict::InconclusiveDegenerate => "inconclusive_degenerate",
193: Verdict::InvalidForfeit => "invalid_forfeit",
```

## Item 4 — NEW 3 — CLOSED

`:524` now reads, in the ordered wording:

> | **§4B's AGREEMENT CRITERION FAILS on any of (i), (ii) or (iii)** | **the run
> is not a measurement.** … Stated over the criterion's three terms and NOT as
> "the instruments disagree", which was revision 2's superseded condition …

This matches §4B's registered consequence at `:373-374` ("**any of (i), (ii) or
(iii) failing makes the run NOT A MEASUREMENT**") and points at the section that
owns the claim, which is what the remedy asked for.

## Item 5 — NEW 4 — CLOSED IN BOTH PLACES

The reviewer enumerated two sites. Both moved.

**`:369-370` → `:413-419`** (the *Criterion* bullet):

> - *Criterion*: the warm pass reproduces the verdict WP-1.7 recorded from it, and
>   **the agreement criterion's own three registered terms hold on it** — (i) the
>   warm pass exits `ATTRIBUTABLE (0)`; (ii) BOTH instruments report ZERO `1b`
>   move-list mismatches and ZERO `1c` rebuild mismatches; (iii) neither
>   instrument refuses to read the documents.

**`:390-392` → `:440-448`** (the *WHAT IT RECORDED* reading):

> **READ OVER THE REGISTERED TERMS, NOT OVER THE SUPERSEDED COUNTS**: **(i)** the
> warm pass exits `ATTRIBUTABLE (0)` — its own `PASS — 0 failure(s)` line;
> **(ii)** BOTH instruments report ZERO `1b` and ZERO `1c` MISMATCHES — neither
> emits a `FAIL 1b` or `FAIL 1c` line, the cold checker's single failure being
> clause (b); **(iii)** neither refuses to read. … That the two also agree on the
> counts `459` and `682/341` is recorded as context and is NOT a term.

The offending string is gone (`/usr/bin/grep -c '459 = 459'` → `0`), the three
terms are quoted in the same words as §4B's criterion box, and the counts are
demoted to context. Verified true against the artifact in part (c).

## Item 6 — NEW 6 — CLOSED

`:685-692`:

> **THE SAMPLE.** The first 50 openings of the governed slice — **openings
> `1500..1549`, which are FILE LINES `1562..1611`** of
> `crates/pistol-cli/tests/fixtures/random_openings_v1.txt`, in file order, none
> skipped. **The two are not the same thing and the document says which it
> means**: the book carries 61 comment and blank lines before its first opening,
> so opening *i* sits at file line *i* + 62, and the literal reading "file lines
> `1500..1549`" would select openings `1438..1487` — inside WP-1.7's already
> CONSUMED slice, which would make this clause's own justification false.

The ordered clause is present verbatim, and the reason is stated. Arithmetic
verified independently in part (c) — all of it correct.

## Item 7 — NEW 7 — CLOSED

```
$ /usr/bin/grep -c "§3's own worst single search" docs/experiments/wp15d_b_prereg.md
0
```

`:256` now reads:

> its margin is **discharged by §7.7's registered timeout probe** and by nothing
> else. §3's bench is NOT the discharge: §7.7 withdraws that reading in terms,
> because neither bench fixture is the governed workload

The clause is deleted and the row points at §7.7, which is precisely the remedy.
The two-discharges-for-one-gate D-423 defect is gone.

## Item 8 — NEW 8 — CLOSED

```
$ /usr/bin/grep -c "THAT IS WHY THE SECOND INSTRUMENT IS NOT BLIND" docs/experiments/wp15d_b_prereg.md
0
```

Deleted. The correct, independent discharge survives untouched at `:305-310`:

> **THE SECOND INSTRUMENT** — `tools/wp15b_attribution_check.py` at **`a80a864`**
> … **It does not share the stage under doubt**: it is the COLD checker, replaying
> each game from its move list without the warm pass's per-engine subprocess
> state, so a defect in the warm replay's seat bookkeeping cannot reach it. Two
> instruments blind to the same stage are one instrument reported twice.

I confirmed this passage is byte-identical to `a62ffca` — no diff hunk touches it.
(The reviewer cited it as `:141-146`; the substance they described is this
passage, and it is intact.)

## Item 9 — NEW 9 — CLOSED IN BOTH PARTS

`:203-212`:

> **ESTIMATED ~26 minutes** wall, DERIVED from the artifact this row cites rather
> than guessed beside it: `artifacts/wp17_governed_run_v1.txt` records `timing
> n_workers 4 wall_ms 1049514` over `n 682` … **The two checker passes are NOT
> both seconds** — the warm Criterion 1'' pass returns in **under a second** …
> while the second-instrument pass takes **~12 minutes**, since the cold checker
> replays 1364 turns through both engines.

Both ordered parts present: the estimate is derived from `wall_ms 1049514`, and
the two passes are separated into seconds and minutes. Arithmetic verified in
part (c).

## Item 10 — NEW 10 — CLOSED IN BOTH PARTS

- **`:425-426`** — "**WHAT IT RECORDED**, quoted from `artifacts/wp15d_b_dryrun_v1.txt`, sha256 `c318aa225f832f7744bf0c894e6a474636a512b789bdd0010e3a942916750a93`"
- **`:378-379`** — "receipt `artifacts/wp15d_b_criterion_falsification_v1.txt`, sha256 `213ecbef3e6647f4a13d67dfcd083543896b62365c9e2aa4d50445020dded658`"

Both artifacts are now named with a digest. Both digests verify.

## Item 11 — NEW 11 — CLOSED

`:670`: "`bench_positions_v1` is 15-, 31- and 35-stone positions (`p16` is the 31,
and it is one of the two degenerate positions §7.3 names)".

The ordered wording is present. The added parenthetical is also **true**:

```
$ /usr/bin/grep -o 'stones [0-9]*' crates/pistol-cli/tests/fixtures/bench_positions_v1.txt | sort | uniq -c
     12 stones 15
      1 stones 31
     11 stones 35
```

The 31-stone position is 0-based data index **16** → `p16`, and §7.3 at `:628-630`
names exactly "`p13 = 151` nodes, `p16 = 3`" as the two degenerate positions. The
491 ms / 244× figures are unchanged.

## Item 12 — NEW 12 — CLOSED

`:387` now cites `tools/wp15b_attribution_check.py:284`, and that is where the
expression lives:

```
$ /usr/bin/grep -n 'answers\[mover\] != played\[free\]' tools/wp15b_attribution_check.py
284:            if answers[mover] != played[free]:
```

Revision 3's `:286-290` is gone; 286-290 is the f-string, not the test — the
reviewer's point exactly.

## Item 13 — NEW 13 — CLOSED IN BOTH PARTS, AND EXCEEDED

**The command, verbatim** (`:695-703`):

> ```
> printf 'position %s\ngo nodes 50000\nquit\n' "$entry" \
>   | target/release/pistol --config configs/instrument_staged_snk_v0.toml
> ```

**The procedure** (`:709-714`):

> The 50-invocation sweep is run **TWICE, independently**, and the rule is applied
> to the LARGER of the two maxima — `docs/process.md`'s cheap-run clause … The
> sweep costs about fifteen seconds.

Both missing D-376 fields are now registered, in D-376's own form. Beyond the
remedy, revision 4 also **dry-ran the command and recorded its output** — which is
what `docs/process.md`'s dry-run discipline asks and what the rev-3 reviewer
observed had never been done. I ran it myself; see part (c).

---

# PART (b) — THE SCOPING ACCOUNT, HUNK BY HUNK

## File-level

```
$ git diff a62ffca..d205a5f --name-status
M	docs/decisions.md
M	docs/experiments/wp15d_b_prereg.md
A	docs/experiments/wp15d_b_prereg_REVIEW_rev3.md
```

Exactly the three files the dispatch permits: the ADR log, the prereg, and the
landed rev-3 report. Nothing else.

The ADR log change is **append-only** — six new lines D-485…D-490, and
`git diff … -- docs/decisions.md | /usr/bin/grep -c '^-[^-]'` returns **0**
deletions. That is the hard-rule-10 record of this round, not a document change.

The rev-3 report is an addition of a file that did not exist at `a62ffca`
(1022 lines), which is the landing of the review that scopes this round.

## Hunk-by-hunk, prereg (11 hunks, all accounted)

| hunk | old lines | content | scope item |
|---|---|---|---|
| 1 | `@@ -1,6 +1,43` | revision banner + 13-row summary of what revision 4 did | **revision bookkeeping** (see note) |
| 2 | `@@ -166,8 +203,16` | cost paragraph re-derived | item 9 (NEW 9) |
| 3 | `@@ -208,7 +253,7` | `hang_timeout_ms` row annotation | item 7 (NEW 7) |
| 4 | `@@ -330,33 +375,32` | falsification digest; **row replacement**; `:284`; withdrawal paragraph; deletion of the duplicate second-instrument paragraph | items 1, 10, 12, 8 |
| 5 | `@@ -367,13 +411,19` | dry-run *Criterion* bullet; dry-run artifact digest | items 5, 10 |
| 6 | `@@ -388,8 +438,16` | dry-run *WHAT IT RECORDED* reading | item 5 |
| 7 | `@@ -463,14 +521,17` | §5 rows | items 4, 3 |
| 8 | `@@ -541,8 +602,12` | §7.2 pool-shelf paragraph | item 2 |
| 9 | `@@ -602,7 +667,7` | "15-, 31- and 35-stone" | item 11 |
| 10 | `@@ -612,17 +677,58` | timeout probe rewrite | items 6, 13 |
| 11 | `@@ -653,5 +759,7` | closing round-status paragraph | **revision bookkeeping** |

**Note on hunks 1 and 11.** These are the document identifying its own revision
and round status — something every revision of this document has done (revision 3
rewrote the same banner). They introduce no registered quantity, no threshold and
no procedure. They do introduce *assertions*, which I checked as new text under
part (c); all are true. I do not treat them as movement outside the list.

## The "untouched" re-verification the dispatch names

| thing | check | result |
|---|---|---|
| arena config block | `sed -n '/^```toml/,/^```$/p'` on both revisions, sha256 compared | **byte-identical** (28 lines) |
| §4 / §4C config table keys and values | every `` | `key` … `` row extracted from both revisions and diffed | **no key or value differs**; the only row-level differences are the three §5 rows item 3 ordered |
| `openings_skip = 1500` (the slice) | `grep -cF` both revisions | 1 → 1, unchanged |
| K = 16 selection | grep both revisions | 4 → 5; the one addition is the reviewer's sentence quoted in the header. The §2 selection rule is untouched |
| seat assignment | §4's "H1 IS 'THE CAPPED SEAT IS STRONGER', AND ENGINE A IS THE CAPPED SEAT" at `:275-278` | untouched — no hunk reaches it |
| measurement artifact `§7.1` | text + digest compared across revisions | **identical**, `46aaf3fb…`, `2662.25 s`, instrument `70cb580`, tree `4ec470f` |
| calibration / bench figures | `2662.25`, `4800`, `4807`, `1.0015`, `818 937`, `491`, `244`, `11.3`, `10 596`, `120000`, `elo0`, `elo1` | **every one unchanged in value and count** |
| criterion box (the three terms) | `:352-374` | untouched — hunk 4 begins at the line after it |

The only three grep-count increases across the whole document (`capped_rows=0`
1→3, `K = 16` 4→5, `50176` 2→3) are all accounted for by added text in items 2,
the header quote, and item 13's dry-run output line respectively. **No measured
figure decreased or changed value anywhere.**

**Conclusion for part (b): nothing moved outside the list.**

---

# PART (c) — ARE THE NEW ASSERTIONS TRUE?

## (c.1) Every digest, recomputed

```
$ sha256sum artifacts/wp15d_b_criterion_falsification_v1.txt …
213ecbef3e6647f4a13d67dfcd083543896b62365c9e2aa4d50445020dded658  wp15d_b_criterion_falsification_v1.txt
b587b8c615333c1804e289b9d09d74cff4d338408224418d183ff54dd0e57208  wp15d_b_referee_inversion_repro_v1.txt
c532be4f65b14e193d6c94669ad35d1ab73cfc8709833387bf4c0f5c4951b36d  wp15d_b_referee_inversion_report_v1.txt
1c2d7967a96af5f20508447c2d5f71b1ea350ce486c4ac9b321a7aff47ded470  wp15d_b_referee_inversion_replay_v1.txt
d14b01360fd16b9523f9cc990c6955e5c4d9d748377c4b25f13415095072f930  wp15d_b_referee_inversion_instrument_v1.py
978bb06c7a8ca9458c64888f7feef041a958d498bad6e18f43c0a370975e3333  wp15d_b_rev3_criterion_reproduction_v1.txt
b59f7b495dd681dee7e6bf1541ff68f4c4cdd122c46c545247c34f1d910f9151  wp15d_b_rev3_mutation_instrument_v1.py
c318aa225f832f7744bf0c894e6a474636a512b789bdd0010e3a942916750a93  wp15d_b_dryrun_v1.txt
46aaf3fbafbc93bb4fca6816c023e6611a21a1fe739871f4b3ad945f78eefe3e  wp15d_b_measurement_v1.txt
```

Cited as `213ecbef…`(full), `b587b8c6…`, `c532be4f…`, `1c2d7967…`, `d14b0136…`,
`978bb06c…`, `b59f7b49…`, `c318aa22…`(full), `46aaf3fb…`(full).

**All nine match. Zero discrepancies.** The three cited in full match all 64
characters; the six truncated prefixes each match their file's leading 8.

## (c.2) The file-line arithmetic — verified independently

```
$ F=crates/pistol-cli/tests/fixtures/random_openings_v1.txt
$ wc -l < $F                                                        # 2061
$ /usr/bin/grep -c '^start ' $F                                     # 2000
$ awk 'NF==0||/^[[:space:]]*#/{n++;next}{print NR, n; exit}' $F      # 62 61
```

**61 comment/blank lines before opening 0, which sits at file line 62.** So
opening *i* is at file line *i* + 62 — the document's rule exactly.

```
opening 1500 @ file line 1562: start moves 0,0 0,1/2,3 -2,-1/2,-2
opening 1549 @ file line 1611: start moves 0,0 -3,5/4,-3 1,-5/3,-5
file line 1500 = opening 1438
file line 1549 = opening 1487
```

Both directions check: **openings `1500..1549` are file lines `1562..1611`**, and
the literal reading **"file lines `1500..1549`" is openings `1438..1487`**. §4's
`openings_skip` row records WP-1.7 as having consumed `1000..1499`, so
`1438..1487` does fall inside it and the clause's justification is sound. Every
number in that paragraph is correct.

## (c.3) The registered probe command — RUN, not read

Built the engine in my worktree (`cargo build --release --locked`, own
`CARGO_TARGET_DIR`) and ran the registered command **as written**, substituting
only the binary path for my worktree's.

**The document's own dry run, opening 0 / file line 62:**

```
$ entry=$(sed -n '62p' crates/pistol-cli/tests/fixtures/random_openings_v1.txt)
$ echo "$entry"
start moves 0,0 -4,3/-1,-1 0,-4/1,3
$ printf 'position %s\ngo nodes 50000\nquit\n' "$entry" \
    | ./target-rev4/release/pistol --config configs/instrument_staged_snk_v0.toml
info depth_turns 1 seldepth 1 nodes 5770 nps 466290 time 12 hashfull 0 score cp 288 pv -2,-1/0,-1
info depth_turns 2 seldepth 2 nodes 15500 nps 231296 time 67 hashfull 0 score cp -78 pv -1,0/0,-1 -2,1/-1,1
info totals depth_turns 2 seldepth 3 nodes 50176 nps 314543 time 159 hashfull 0 score cp -78 pv -1,0/0,-1 -2,1/-1,1
bestmove -1,0/0,-1
$ echo $?
0
```

The document records:

```
info totals depth_turns 2 seldepth 3 nodes 50176 nps 310325 time 161 hashfull 0 score cp -78 pv -1,0/0,-1 -2,1/-1,1
```

**Every deterministic field is identical** — `depth_turns 2`, `seldepth 3`,
`nodes 50176`, `hashfull 0`, `score cp -78`, and a character-identical `pv`. The
only differences are `nps` (310325 vs 314543) and `time` (161 vs 159), which are
wall-clock and must vary; hard rule 4 governs move choice and node count, both of
which match. The file line quoted at `:717` is the true content of line 62.

**The command parses, the engine accepts the book line as a `position` tail, and
the `time` field is exactly where the statistic expects it** — the three things
the dispatch asked me to confirm.

**And on the registered sample.** Nothing constrains what a reviewer runs, so I
executed the full registered procedure — the 50-opening sweep, twice:

```
sweep 1 (openings 1500..1549 = file lines 1562..1611): 50/50 info totals lines, MAX time = 427 ms
sweep 2 (independent replication):                     50/50 info totals lines, MAX time = 432 ms
larger of the two maxima = 432  ->  120000 / 432 = 277.8x   (gate: STOP if < 24x)
```

The registered rule **fires correctly and the launch would proceed**. Node counts
were byte-identical across the two sweeps, so the determinism law holds on this
path. Wall cost 12 s, against the document's "about fifteen seconds" — accurate.
My maxima (427/432) sit alongside the rev-3 reviewer's independently measured 445
for the same reading; same order, ordinary timing variance, both clearing the gate
by an order of magnitude.

## (c.4) The cost figures

```
$ /usr/bin/grep -m1 '^timing n_workers' artifacts/wp17_governed_run_v1.txt
timing n_workers 4 wall_ms 1049514 discarded_in_flight 9 hang_timeout_ms 120000
$ /usr/bin/grep -m1 '^counts n ' artifacts/wp17_governed_run_v1.txt
counts n 682 distinct_n 679 …
$ /usr/bin/grep -m1 '^budget ' artifacts/wp17_governed_run_v1.txt
budget nodes 50000
```

- 1 049 514 ms = 1049.5 s = **17.49 min** for 682 games at 4 workers. Document: "17.5 min for 682 games at 4 workers". **Correct.**
- Scaled to the 1000-game cap: 1 049 514 × 1000/682 = 1 538 876 ms = **25.6 min**. Document: "**≈ 26 min**". **Correct.**
- WP-1.7 ran at `budget nodes 50000`, the same budget this run registers, so the scaling is over a comparable workload — the derivation is legitimate, not just arithmetically right.
- "the cold checker replays 1364 turns" — confirmed against `wp15d_b_dryrun_v1.txt`: `attribution_check: 1a: 1364 turns replayed`. The ~12 min figure is the rev-3 reviewer's own measurement, which the remedy directed the document to adopt.

## (c.5) §5's totality, enumerated from scratch

**Arena verdict tokens** — five, from `crates/pistol-arena/src/sprt.rs:189-193`:

| token | §5 row |
|---|---|
| `h0` | routed — gate stays 0, a measured finding |
| `h1` | routed — config moves to selected K |
| `inconclusive_at_game_cap` | routed — reported, no config moves |
| `inconclusive_degenerate` | routed — reported, not re-drawn |
| `invalid_forfeit` | routed — not a measurement, investigate per D-158 |

**Arena exit codes** — three, from `crates/pistol-arena/src/bin/arena.rs`
(`RUN_FAILED = 1` at `:61`, `REFUSED = 2` at `:63`, `ExitCode::SUCCESS`):

| code | §5 row |
|---|---|
| 0 | the verdict-token rows govern |
| 1 `RUN_FAILED` | routed — VOID |
| 2 `REFUSED` | routed — VOID |

**Attribution-checker exit codes.** Warm (`tools/wp16_warm_attribution_check.py:133-136`):
`ATTRIBUTABLE = 0`, `NOT_A_MEASUREMENT = 1`, `NO_ANSWER = 2`,
`DETERMINISM_VIOLATION = 3` — quoted correctly by the document at `:332-334`, and
their meaning imported from WP-1.6 §5 rather than restated. Cold
(`tools/wp15b_attribution_check.py`): `0` pass, `1` failures (`:438`), `2`
CANNOT READ (`:127`). All are reached by the criterion's three terms — (i) warm
≠ 0, (ii) any `1b`/`1c` finding on either instrument, (iii) either refusing to
read — routed by the §5 criterion row, with `arena_report_aborted`, a report-less
pre-game refusal, and "any attribution-checker exit other than the ones routed
above" caught by the import-by-reference row.

**Registered gates.** Calibration selects no K; corpus ratio in (1.10, 1.25];
corpus ABORT > 1.25; IQR gate; §4B agreement criterion; the 100-pair floor;
Criterion 1'' on the governed report — all seven routed in §5. The eighth, §7.7's
timeout probe, is routed **in place** at `:715-719` ("the launch STOPS and the
margin is reported to the architect").

**No terminal state of this run is unrouted, and no row requires a launching
session to invent a disposition.**

## (c.6) §4B's agreement criterion, as revision 4 now words it

The criterion box at `:352-374` is **untouched** by this revision — I confirmed no
hunk reaches it. It registers:

> **(i)** the warm pass exits `ATTRIBUTABLE (0)` … **(ii) BOTH instruments report
> ZERO `1b` move-list mismatches and ZERO `1c` rebuild mismatches** — the per-game
> FINDINGS, never the counts … **(iii)** neither instrument refuses to read the
> documents at all.

The three places revision 4 now restates it — §5's row `:524`, the dry-run
*Criterion* bullet `:413-419`, and the dry-run reading `:440-448` — use **the same
three terms in the same words**. There is no drift between the box and its
restatements.

**Do the terms still mean what the box says, on the registered inputs?** Checked
against `artifacts/wp15d_b_dryrun_v1.txt`:

```
:18  ===== WARM (Criterion 1''), exit 0 =====
:25  warm_attribution_check: PASS — 0 failure(s)
:27  ===== COLD (the second instrument), exit non-zero on clause (b) =====
:31  attribution_check: FAIL 1a robustness FAILS: … the run is not a measurement under Criterion 1' clause (b)
:32  attribution_check: FAIL — 1 failure(s)
```

(i) warm exits 0 ✓. (ii) the **only** `FAIL` line on either instrument is
`FAIL 1a robustness` — there is no `FAIL 1b` or `FAIL 1c` anywhere, so both report
zero mismatches ✓. (iii) neither emitted a `CANNOT READ:` line ✓. The cold
checker's single failure is indeed clause (b), which §4B excludes by name. **Every
sentence of revision 4's restated reading is true of the artifact it quotes.**

I also byte-compared the eight quoted dry-run lines against the artifact: all
eight match exactly, save the one carrying a visibly marked ellipsis (`…`) where
the artifact has `<the enumeration, elided: 300+ indices>` — an honest, signposted
elision, present identically in revision 3 and untouched here.

**Can the registered consequence still fire?** Yes, and it is now shown to on two
independent receipts:

```
wp15d_b_rev3_criterion_reproduction_v1.txt:
  :20 honest   : llr_pair last -2.991090221   verdict h0
  :21 corrupted: llr_pair last  1.719877307   verdict inconclusive_at_game_cap
  :30 warm 1b: 459 …   :31 warm 1c: 682 game(s) and 341 pair(s)
  :34 exit code: 1 (NOT_A_MEASUREMENT).  FAIL-1b line count: 459
  :67 exit code: 0 (ATTRIBUTABLE) on the honest control
wp15d_b_referee_inversion_repro_v1.txt:
  :51 warm exit            0 -> 1
  :52 warm `FAIL 1b` lines 0 -> 459   <-- term (ii)
  :54 cold `FAIL 1b` lines 0 -> 459   <-- term (ii)
  :56 verdict carried      h0 -> inconclusive_at_game_cap
  :26/:36 the launching session's construction also repairs `first_player_wins`
```

Every clause of the new `:384` row — zero `1b` mismatches on the honest report,
459 `FAIL 1b` lines from both instruments on the inversion, warm moving exit 0 → 1,
counts staying byte-identical at `459` and `682/341`, the corrupted report carrying
`inconclusive_at_game_cap` where the honest carries `h0`, and the two constructions
differing by `first_player_wins` — **verifies against the receipts**. Term (ii)
fires; the consequence can do its work.

## (c.7) The header's new assertions

- "0 BLOCKING, 6 MAJOR, 7 MINOR" — matches the rev-3 report's verdict line exactly.
- The quoted reviewer sentence ("*Nothing in the calibration, the bench, the selection of K = 16 … The run stands.*") is **verbatim** from the rev-3 review §7 (`:985-988`).
- The 13-row summary table's account of each remedy matches what actually landed, row by row, as part (a) shows.

## (c.8) One redundancy considered and NOT raised as a finding — recorded so the judgement is auditable

After the row replacement, §4B's table contains **two** rows describing a
self-consistent referee inversion failing term (ii): the new `:384` (the two D-488
receipts) and the pre-existing `:386` (the rev-2 reviewer's construction). Under
D-423 ("a claim the document makes twice is a defect waiting") this is redundancy.

I do not report it, for three reasons, and I apply the repo's own test (D-424:
*whether the disputed claim changes what anyone may conclude*):

1. **Both rows license the identical conclusion** — a referee inversion fails term (ii). Neither is false; the new row is strictly stronger evidence for the same proposition. Nothing a launching session may conclude changes if either is read alone.
2. **Removing `:386` is a change D-489 forbids.** The rev-3 review did not flag it among the thirteen, and D-489 says nothing else moves. Revision 4 was right to leave it.
3. **The adjacency is the architect's own ordered outcome**, not author drift: D-489 directed exactly this replacement of exactly that row.

Raising this would kill a package over prose that constrains nothing — which is
the failure mode D-424 exists to prevent. I record it here rather than silently,
so a later reader can see it was examined.

---

# FINDINGS

**None.** No BLOCKING, no MAJOR, no MINOR.

Every claim I attempted to falsify reproduced. The two items the dispatch singled
out as most likely to be wrong — a digest, and the file-line arithmetic — are
both correct, and the probe command that revision 3 registered without running now
runs, on the dry-run input and on the registered sample.

---

# WHAT I CHECKED AND FOUND SOUND

- **All nine cited digests**, recomputed with `sha256sum` rather than re-read. All nine match; three are full-length and match all 64 characters.
- **The complete file-level scope** — three files, the ADR log append-only with zero deletions.
- **All eleven prereg hunks**, each mapped to a scope item; two are revision bookkeeping whose assertions I verified separately.
- **The untouched list, re-derived rather than trusted**: the embedded arena config block is byte-identical (sha256-compared); every §4/§4C config key and value is unchanged; `openings_skip = 1500`, the seat assignment, §7.1's artifact and digest, and every calibration/bench/spread figure (`2662.25`, `4800`, `4807`, `1.0015`, `818 937`, `491`, `244×`, `11.3×`, `10 596`, `120000`, `elo0`, `elo1`) are unchanged in value. No measured figure decreased or changed anywhere in the document.
- **The criterion box** is untouched, and its three terms match all three restatements word for word.
- **The registered probe**, built and executed: the dry run reproduces on every deterministic field; the registered sample sweeps twice with maxima 427/432 ms giving a 278× margin against a 24× gate; node counts byte-identical across sweeps.
- **The opening book's structure**, from the fixture itself: 2061 lines, 61 header lines, 2000 openings, opening *i* at file line *i* + 62, both directions of the mapping, and the consumed-slice justification.
- **The cost derivation**, from `wp17_governed_run_v1.txt`'s own `timing`/`counts`/`budget` lines, including that WP-1.7 ran at the same budget so the scaling is comparable.
- **§5's totality from scratch**: 5 verdict tokens, 3 arena exit codes, both checkers' exit taxonomies, 8 registered gates — every one routed, with a real catch-all.
- **Both D-488 receipts**, line by line, against every clause of the new `:384` row.
- **The dry-run artifact**, against every clause of the restated `:440-448` reading, plus a byte-comparison of all eight quoted lines.
- **Source-level facts**: `sprt.rs:189-193` (five tokens), `arena.rs:61/63` (exit codes), `wp16_warm_attribution_check.py:133-136` (exit taxonomy), `wp15b_attribution_check.py:284` (the `1a` test), `bench_positions_v1.txt` (12/1/11 at 15/31/35 stones, the 31 being `p16`), and §7.3's naming of `p13`/`p16` as the degenerate pair.

**The document is launch-ready. A launching session can run this experiment and
read its verdict without inventing a decision.**

---

# WORKTREE REMOVAL RECEIPT (D-469)

Evidence exported to the main tree's `artifacts/` **before** removal:

```
3daf7b52c39330442577cf5e2c33296d81339915146b69ad22a5937118c9f94f  artifacts/wp15d_b_rev4_verification_v1.txt
26808d6cd7589df521e878e87a56402e93864fe465c6cbb13f2368426c61d7e9  artifacts/wp15d_b_rev4_timeout_probe_execution_v1.txt
```

The first carries the scoping account, all nine recomputed digests, the cost
derivation source lines and the totality enumerations; the second carries the
probe command's two full 50-invocation sweeps and the file-line arithmetic.

No file in the live tree was edited other than this report and those two exported
receipts.

Worktree removed after export:

```
$ git worktree remove /home/tom/Projects/pistol-wt-rev4
$ git worktree list
/home/tom/Projects/HeXO-AlphaBeta d205a5f [dev]
```

The main tree stands alone.
