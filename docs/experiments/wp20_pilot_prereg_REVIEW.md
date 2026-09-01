# REVIEW-prereg — `docs/experiments/wp20_pilot_prereg.md` revision 2

## Header

- **Revision adjudicated**: `6e1fea3bd993004cf7e0c9ae4f2de671f248c3c4` (branch `dev`).
- **Matches HEAD**: YES. `git rev-parse HEAD` returns `6e1fea3bd993004cf7e0c9ae4f2de671f248c3c4`.
- **Tree state**: `git status --porcelain` is EMPTY.
- **Reviewer**: fresh context. I wrote none of this and owe it no deference.

**What I read.** `docs/experiments/wp20_pilot_prereg.md` (all 792 lines);
`configs/arena_wp20_label_pilot.toml`; `configs/arena_wp20_label_pilot_dryrun.toml`;
`docs/book_v2_ledger.md`; `CLAUDE.md`; `docs/process.md` (all 79 lines);
`docs/experiments/wp20_dispatches.md` (Development round, §4, §5, DONE, closing note);
`tools/SHELL_CHECKLIST.md` items 8–12; `docs/decisions.md` D-427, D-483, D-500, D-518,
D-534, D-539, D-540, D-542, D-543, D-544, D-547, D-549, D-550, D-551, D-552, D-553;
`tools/cold_label_check.py`; `crates/pistol-arena/src/bin/corpus-check.rs`;
`crates/pistol-arena/src/capture.rs`; `crates/pistol-arena/src/bin/arena.rs`;
`crates/pistol-arena/src/usage.rs`; `crates/pistol-arena/src/labels_file.rs`;
`crates/pistol-arena/src/labels.rs`; `crates/pistol-arena/src/transcript.rs`;
`crates/pistol-arena/src/schedule.rs`; `crates/pistol-arena/src/summary.rs`;
`crates/pistol-arena/src/identity.rs`; `crates/pistol-cli/src/report.rs` (totals format);
`configs/instrument_v0.toml`; `crates/pistol-arena/tests/cold_label_check_tests.rs`;
the four `corpus_check`-driving tests and the digest test in
`crates/pistol-arena/tests/labels_tests.rs`; `crates/pistol-arena/tests/capture_tests.rs`
(decided-branch coverage); `artifacts/wp20pilot_dryrun_f297eab_v1.txt`.

**What I ran** (read-only; **no `cargo` in any form and no `tools/ci.sh`**, per the
in-flight CI run):

- `git rev-parse HEAD`, `git status --porcelain`, `git log --oneline`
- `git diff --stat f297eab 1311454 -- tools/ crates/ configs/`
- `git diff f297eab 1311454 -- configs/arena_wp20_label_pilot_dryrun.toml`
- `git diff --stat 1311454 6e1fea3`, `git show --stat 1311454`
- `git cat-file -e f297eab:<path>` for the two arena configs and both instruments
- `python3 tools/design_citation_check.py docs/experiments/wp20_pilot_prereg.md`
  → `52 citation(s) checked, 0 unreproduced`, exit 0
- `sha256sum target/release/pistol` → `180b4c406b225fc8…`, matching §1's SLOT R2
- a read-only `python3` script over the four surviving dry-run captures in
  `/home/tom/pistol-runs/wp20pilot-dryrun/`, re-deriving §6.3's RULE-2 medians and
  means and re-checking each file's body digest against the cited artifact
- `/usr/bin/grep` throughout (D-265)

**Where a build would be needed.** No finding below rests on one. Had I needed to
settle whether a claimed refusal fires, the run would have been
`cargo test -p pistol-arena --locked --test labels_tests --test cold_label_check_tests`
in a detached worktree with its own `CARGO_TARGET_DIR`.

---

## VERDICT: **FAIL**

| severity | count |
|---|---|
| **BLOCKING** | **1** |
| **MAJOR** | **7** |
| **MINOR** | **6** |

The document is far stronger than revision 1 and it survives the attack it asks for
by name on its own hardest question (§4A's non-vacuity — see the closing section).
It fails on one defect that converts a broken run into a registered PASS, and on
seven claims the tree does not support.

---

# BLOCKING

## B1 — a pass-1 run FAILURE is registered as `V2 — PASS WITH A FINDING`

**Against**: `docs/experiments/wp20_pilot_prereg.md` §4C ("ZERO FORFEITS", the
paragraph beginning *"The pilot's own pass-1 exit code is the receipt"*) and §5's
verdict table, row `V2`.

**What is wrong.** §4C writes:

> `--config` returns `0` only where `score::tally(&played.records).forfeits == 0`,
> and `1` **otherwise** with the report still written.

and §5's V2 reads a nonzero pass-1 exit as *"a nonzero forfeit count"* → **PASS WITH
A FINDING**.

The first half is true; the second is false. `crates/pistol-arena/src/bin/arena.rs`
returns `RUN_FAILED` (1) from **two** disjoint branches:

- lines 225–233 — `failure` is `Some(error)`: the run itself died. The program prints
  `arena: {error}` and then, in its own words, that the finished games are in the
  report *"as a diagnostic, not as a sample"*.
- lines 234–243 — the run completed and `score::tally(...).forfeits != 0`.

`crates/pistol-arena/src/usage.rs:76` says so on the program's own face:
`1 abandoned **or** forfeited (report still written)` — **and §4C quotes that exact
block, verbatim, three paragraphs above the sentence that contradicts it.**

**Why it matters.** A pass-1 death is reachable at these settings: the
`hang_timeout_ms = 120000` watchdog firing (D-159 — it *"can end a run and it can
never produce a game result"*), a seat closing its pipe, an engine line the driver
refuses. In that event the pilot writes a truncated report; the registered reading
maps exit 1 to V2 and the arc proceeds; and **no downstream criterion catches it** —
`--capture` walks whatever games the report holds, C-B compares two captures of the
same truncated report, C-C replays the games present so `covered == total` and it
exits 0, and C-E loads the corpus that results. C-D's MEASURED throughput is then a
rate over a run that aborted. The pilot's registered outcome is PASS.

**How I reproduced it.** `sed -n '14,17p;225,243p' crates/pistol-arena/src/bin/arena.rs`
and `/usr/bin/grep -n "abandoned\|forfeited" crates/pistol-arena/src/usage.rs`, read
against §4C and §5's V2 row.

**Minimal remedy.** Two edits, both inside §4/§5:

1. §4C: the forfeit receipt is the **count printed in the report's own summary**, not
   the exit code. `crates/pistol-arena/src/summary.rs:63-66,119-121` prints
   `— CONDITIONAL: <n> forfeited game(s) excluded` and
   `*** <n> game(s) were forfeited`, so the count is already on the face of the
   artifact the pilot keeps.
2. §5 gains a row: **pass 1 exits `1` with `arena: <error>` on stderr and the
   `"as a diagnostic, not as a sample"` line — the run DIED. STOP; the partial report
   is preserved and is not captured from.** V2 keeps only the clean-run-with-forfeits
   case.

---

# MAJOR

## M1 — RULE-2's depth table, which fixes the single most expensive value in the document, has no cited artifact and no instrument

**Against**: §6.3, the table headed *"RULE-2, APPLIED AS WRITTEN"* (median
`depth_turns` 3.0 / 3.0 / 3.0 / 4.0, means 2.72 / 3.04 / 3.30 / 3.63).

**What is wrong.** §6.3 opens *"**THE FOUR MEASURED COSTS**, every one from that
artifact's own printed lines"* — which correctly scopes the citation to `g`, `p`, `l`
and `c`. The RULE-2 table then follows with **no source at all**.
`artifacts/wp20pilot_dryrun_f297eab_v1.txt` contains no depth or median figure
anywhere; I read all 84 lines. No row of §1's instrument table produces a median
`depth_turns`, so under `docs/process.md`'s *"Instrument governing revision"* the
artefact that produced these numbers is un-named and un-revisioned. D-483 is explicit:
*"every number a prereg or gate consumes is produced post-implementation by a
registered instrument in one run and cited from that run's artifact by digest."*

**Why it matters.** This table selects **SLOT S2 = `400000`**, which sets `l = 1.006 s`
and therefore `165.0 T` of the arithmetic's `254.5 T` — **65 % of the pilot's whole
wall**. It is also the document's own headline defence of RULE-2's integrity (*"it
selected `400000`, the most expensive of its three candidates"*). The evidence for it
is four gitignored files in `/home/tom/pistol-runs/wp20pilot-dryrun/`, outside the
repository, **sha-indexed by no committed manifest** (CLAUDE.md rule 8). Deleted
tomorrow, this number becomes unreproducible. D-543 records this class — a number a
human retyped out of an artifact — at three consecutive revisions of this package's
own matrix.

**How I reproduced it.** `/usr/bin/grep -in "depth\|median\|mean" artifacts/wp20pilot_dryrun_f297eab_v1.txt`
returns nothing. I then recomputed the table from the surviving captures:

```
50000 : n=164 median=3.0 mean=2.7195 digest_ok=True body_sha256=2a640284ce2b65fc
100000: n=164 median=3.0 mean=3.0366 digest_ok=True body_sha256=d95b71687539774f
200000: n=164 median=3.0 mean=3.3049 digest_ok=True body_sha256=33f67f788ca919ef
400000: n=164 median=4.0 mean=3.6341 digest_ok=True body_sha256=49eea2712d757507
```

**The numbers are TRUE and they reproduce exactly**, and each file's body digest
matches the one the cited artifact prints. This is a citation defect, not an
arithmetic one — but the document is one `rm -rf` away from having no evidence for its
most expensive decision.

**Minimal remedy.** Cheap, because the digests are already in the cited artifact.
§6.3 names the four captures by the `body_sha256` values the artifact printed
(`2a640284…`, `d95b7168…`, `33f67f78…`, `49eea271…`), and gives the one-line
derivation (`depth_turns` out of field 4 of each record, median over 164). §1 gains a
row for that extraction. Better still, the four captures get a committed manifest row
under rule 8.

## M2 — §5's void class is not total: a mid-pipeline `--capture` / `--labels` refusal falls outside it

**Against**: §5, *"THE VOID CLASS, DEFINED"*, first bullet and the closing
*"**and nothing else**"*.

**What is wrong.** §5 enumerates arena's exit `2` as *"`arena` refusing a document
**before any game**"*. But `crates/pistol-arena/src/bin/arena.rs:76-88` funnels
**every** `ArenaError` from the `--capture` and `--labels` arms into
`Err(error.to_string())` → `main` → exit `2`. That includes failures raised **during**
the capture walk, from `capture::ask` (lines 190–234): *"the engine spoke before it
was asked"*, *"the engine closed its pipe"*, *"the engine wrote `<line>`, which is not
a line this protocol has"*, and the receive timeout. None of these is a refusal
*before any game* — the games are already played and the walk is partway through.

**Why it matters.** The document claims totality by name (*"Every way this pilot can
end, with no residue. A run whose outcome is not one of these rows is itself a finding
about this document"*) and closes the enumeration with *"and nothing else"*. Here is a
reachable ending that is neither in the enumeration nor in V1–V6. Read strictly, it
has no verdict. Read loosely as *"an instrument exiting 2"*, it becomes a VOID and
earns **one re-run** — 55 minutes — for what is a deterministic pipeline defect that
will reproduce identically. (The document's own *"A void with no receipt is a STOP"*
clause happens to rescue it, since no filesystem, process or signal can be receipted
for a protocol violation. That is the right answer arrived at by accident, through a
clause written for something else.)

**How I reproduced it.** `sed -n '66,90p' crates/pistol-arena/src/bin/arena.rs` against
`sed -n '180,235p' crates/pistol-arena/src/capture.rs`, read against §5's bullet.

**Minimal remedy.** Split the first void bullet: `arena` exiting `2` **before any
work** is a void; `arena` exiting `2` **from inside a pass** is a new STOP row (the
pipeline refused its own input mid-walk, and the refusal names the reason).

## M3 — RULE-1's amendment: the two defences do not answer the suspicion, one floor's stated ground is falsified by the dry run, and "leaves no free choice" is false

**Against**: §6.1, *"RULE-1 — the SLICE (SLOT S1). AMENDED AFTER THE DRY RUN…"*, and
§6.1's opening claim *"**Both rules are total: applied to §7's measurements they leave
no free choice.**"*

**My adjudication, stated plainly, because the document asks for it by name.** The
**correction is legitimate in kind**. Maximising a draw against a wall-clock ceiling
is a genuinely wrong rule for a run D-539 declares *"carries no census and is not
corpus"*; spending a session's remaining capacity on 4 592 records that *"count toward
no minimum"* is real waste, and re-shaping the rule as a minimum against floors is the
right shape. The disclosure is prominent and the *"I did not notice this until the
numbers made it concrete"* sentence is worth more than a smoothed-over rule would be.

**But the specific value 13 is not derived, and the document says it is.** Three
things:

1. **The two defences answer a suspicion nobody has.** *"It cannot be tuned by moving
   the ceiling"* is true and irrelevant — the ceiling is not the free parameter. The
   free parameter is **floor (b)**, and it was set *after* `p = 41` was known, at which
   point the author could compute which floor returns which `T`. The document is
   silent about exactly the parameter under suspicion. For the record, at the measured
   costs: **floor 500 → T = 7** (wall ≈ 30 min), **floor 1000 → T = 13** (≈ 55 min),
   **floor 2000 → T = 25** (≈ 1 h 46). All three sit under the four-hour ceiling, so
   the ceiling constrains nothing and the floor is doing all the work.
2. **Floor (a)'s stated ground is falsified by the dry run's own output.** §6.1
   defends *"at least **4** openings"* as *"the floor at which the report's own pairing
   is exercised over more than one pair"*. The dry run drew **2** openings and
   `artifacts/wp20pilot_dryrun_f297eab_v1.txt:5` prints
   `pair outcomes  p0 0 p1 0 p2 2 p3 0 p4 0  (**2 pairs**)`. More than one pair is
   exercised at 2, not 4. The ground does not support the value. (Floor (a) is
   non-binding at `T = 13`, so this changes no number — but it is direct evidence that
   the floors were written down rather than derived, which is the whole question.)
3. **A larger draw buys something the rule gives zero weight, and the document itself
   names it.** §7.2 finding 1 records that all dry-run games capped, so `result` is
   `capped` on every record and `end` is `normal` on every record, and that *"C-E's
   control is real but narrower than the schema"*. Whether the pilot's own games
   decide is, by §6.3's own admission, *"not known from the dry run"*. A bigger draw
   is the only lever the pilot has on that, and RULE-1 as amended counts wall time and
   position count and **does not count game-shape coverage at all**. I hold this
   honestly: the `decided` branch of `capture::asked_prefixes` (capture.rs:39–42) **is**
   unit-tested (`a_decided_terminal_position_is_never_asked`, capture_tests.rs:643),
   so what a larger draw buys is end-to-end token coverage and a data answer to
   §10(ii), not an untested mechanism. It is a real purchase, and it is uncounted.

RULE-2's expensive answer is evidence about RULE-2. It is not evidence about RULE-1,
and there is a coupling the document does not acknowledge: RULE-2 selecting the most
expensive candidate is precisely what made the un-amended RULE-1 return 56 rather than
something modest, i.e. it created the pressure the amendment relieved.

**Verdict on the section**: not fatal post-hoc convenience, but the claim *"Both rules
are total … they leave no free choice"* is **false** for RULE-1 as amended. The free
choice moved from `T` to the floor and the document does not disclose that it moved.

**How I reproduced it.** The three floor-to-`T` figures come from §6.3's own
`254.5 T + 2` and `2T × 41 ≥ floor`; the two-pairs fact is line 5 of the cited
artifact.

**Minimal remedy.** §6.1 states the floor's sensitivity on its own face — the three
`(floor, T, wall)` triples above — so a reader sees what the free parameter was and
that the author did not move it; fixes floor (a)'s ground (2 openings, not 4, is where
pairing is exercised, so state the real reason for 4 or drop the floor); and either
adds game-shape coverage as a fourth condition or records in one sentence that it was
considered and deliberately given no weight, with grounds.

## M4 — §7.2's footer disclaims finding 1 for this pilot, but finding 1 binds a pilot criterion

**Against**: §7.2, the closing sentence *"**FINDINGS 1 AND 2 ARE FOR THE CLOSURE AND
NOT FOR THIS PILOT.** Neither is a criterion, neither can fail, and neither changes a
value above"*; and §4E's *"WHAT IT MUST SHOW, IN THREE RUNS — 1. THE CONTROL"*.

**What is wrong.** Finding 1 says two of the loader's four token-set columns were
exercised at one value each in the dry run. `crates/pistol-arena/src/labels_file.rs:258-261`
gives the four sets: `to_move` ∈ {p1,p2}, `book` ∈ {yes,no}, `result` ∈
{p1_win,p2_win,capped}, `end` ∈ {normal,forfeit}. The pilot runs the same engine at the
same budget under the same `turn_cap = 40` over random openings, so its corpus is very
likely `capped`/`normal` throughout as well. **C-E is a criterion of THIS pilot**, and
its control's reach over `result` and `end` is then 1-of-3 and 1-of-2. A writer defect
in the `p1_win`, `p2_win` or `forfeit` spelling passes C-E's control silently.

The document handles this correctly for the dry run — *"the closure says so rather
than letting a green run imply otherwise"* — and then the footer explicitly says the
finding does not touch this pilot. It does.

**Why it matters.** C-E's registered defect class is *"a transform that writes a corpus
its own reader refuses"*. The criterion excludes that class **only over the token
values the corpus happens to contain**, and §4E does not say so. A reader of the
closure would take C-E green as covering the grammar.

**How I reproduced it.** `/usr/bin/grep -n "result\|end" crates/pistol-arena/src/labels_file.rs`
(lines 258–261) against §7.2's finding 1 and §4E.

**Minimal remedy.** §4E's run 1 additionally records the observed value-counts of the
four token columns, and states that C-E's reach over `result` and `end` is whatever
that run exercised. Delete finding 1 from the footer's "not for this pilot" list.

## M5 — §8 claims both injections are pinned against the shipped binary; the digest injection is not

**Against**: §8, the paragraph after the command block: *"The same two injections are
pinned as tests against the shipped binary at
`crates/pistol-arena/tests/labels_tests.rs`; the runs above are a **second instance**
over a corpus the pilot itself wrote."*

**What is wrong.** Only the **grammar** injection is driven through the binary.
`crates/pistol-arena/tests/labels_tests.rs:568` —
`a_corpus_carrying_an_injected_malformed_record_is_refused_loudly` — calls `checked()`,
which runs `env!("CARGO_BIN_EXE_corpus-check")`. The **digest** injection's test,
`a_corpus_whose_body_digest_is_wrong_is_refused_by_name` at line 325, calls the library
function directly:

```rust
let error = pistol_arena::labels_file::read(&tampered).expect_err("a tampered body is refused");
```

The tampered line it builds is byte-identical to the one §8's run 3 appends, so it is
plainly the same injection — but it never reaches `corpus-check`.

**Why it matters.** This is **D-553's own class**, which this package invented the law
for: *"a test invoking the guarded function directly pins the function and leaves 'the
call was never made' alive."* Nothing in the suite dies if `corpus-check`'s call into
`labels_file::read` stops reaching the digest guard. And the document's stated ground
for confidence — that C-E run 3 is a *second* instance — is wrong: **it is the first**,
and it runs once, at the pilot, unrepeatable.

**How I reproduced it.** `sed -n '324,335p;531,548p;567,588p' crates/pistol-arena/tests/labels_tests.rs`.

**Minimal remedy.** Either correct the sentence to say only the grammar injection is
pinned against the shipped binary and that run 3 is C-E's sole binary-level exercise of
the digest guard, or (better, and outside this document) add the four-line
`checked()`-driven digest test to `labels_tests.rs`.

## M6 — the dry run ran at `f297eab`, where the committed dry-run config was un-runnable; §1 and §7.1 name `1311454`

**Against**: §1's instrument table, row *"the dry-run arena config |
`configs/arena_wp20_label_pilot_dryrun.toml` | `1311454`"*; and §7.1's *"**THE RECORDED
INPUT.** `configs/arena_wp20_label_pilot_dryrun.toml` **at the revision this document
lands in**"*.

**What is wrong.** The cited artifact is `artifacts/wp20pilot_dryrun_f297eab_v1.txt` —
the run was taken at `f297eab`. The document lands at `1311454`. The config **changed
between them**:

```
$ git diff f297eab 1311454 -- configs/arena_wp20_label_pilot_dryrun.toml
-binary_sha256 = "0000000000000000000000000000000000000000000000000000000000000000"
+binary_sha256 = "180b4c406b225fc81342bb8218b8546dda1ffac1a99f7eb91cdaf73d20253476"
```

(both seats). `crates/pistol-arena/src/identity.rs:65-71` refuses a seat whose binary
does not digest to the declared value, so **the committed `f297eab` file could not have
produced this run** — the dry run was taken against a working-tree edit, i.e. an
uncommitted instrument. §7.1's enumeration of the recorded input lists every field
*except* `binary_sha256`, which is the only one that differed.

**Why it matters.** This is `docs/process.md`'s *"Instrument governing revision"* in
miniature: *"a change to it reopens the review exactly as an amendment to the document
does."* The dry-run config produced §6.3's four registered costs, it changed between
the run and the document, and the document names the wrong revision for it. It is
checkable in one command, which is what makes it a finding rather than a quibble.

**Substantively there is no drift**: every value that governs behaviour
(`openings_v1.txt`, take 2, skip 0, cap 40, workers 4, `nodes 50000`, both seats on
`instrument_v0.toml`) is identical, and the digest the run must have used is
`180b4c40…`, which is the one §1's SLOT R2 and the live `sha256sum target/release/pistol`
both name. The defect is the record, not the run.

**Minimal remedy.** One sentence in §7.1: the run was taken at `f297eab`; the config's
only delta to `1311454` is `binary_sha256`, filled from the `0000…` placeholder to
`180b4c40…`, which is the digest the run used and the one §1 names. §1's row for the
dry-run config reads `f297eab (+ the binary_sha256 fill at 1311454)`.

## M7 — the ledger asserts RULE-1 was registered before the costs existed; §6.1, which it cites, says the opposite

**Against**: `docs/book_v2_ledger.md:43-46`.

**What is wrong.** The ledger's first row is glossed:

> Thirteen openings is not a chosen number: the pre-registration's RULE-1 returns it
> from the per-unit costs its dry run measured, **and the rule was registered before
> those costs existed (§6.1, §6.3)**.

§6.1 — the very section cited — is headed *"**RULE-1 — the SLICE (SLOT S1). AMENDED
AFTER THE DRY RUN, AND THE AMENDMENT IS DISCLOSED HERE RATHER THAN SMOOTHED OVER**"*.
The rule that returns 13 is the **amended** one, written after the costs existed. The
sentence is true of RULE-2 and false of RULE-1.

**Why it matters.** `docs/book_v2_ledger.md` is the durable record — its own opening
says it exists so *"the same question about v2 is answered by reading one document"*.
A successor reading it learns the opposite of what happened, and the amendment §6.1
went to some trouble to disclose does not survive into the file that outlives the
pre-registration. This is precisely the drift D-547 exists against, one document
downstream.

**How I reproduced it.** `sed -n '43,50p' docs/book_v2_ledger.md` against
`/usr/bin/grep -n "AMENDED AFTER THE DRY RUN" docs/experiments/wp20_pilot_prereg.md`.

**Minimal remedy.** Replace the clause with: *"RULE-1's floors were registered after
the dry run's costs arrived, as an amendment §6.1 discloses; RULE-2, which fixes the
label budget, was registered before."*

---

# MINOR

## m1 — two terms of the wall arithmetic are asserted, not measured, in a section headed "derived from a cited timing artifact"

**Against**: §6.3's arithmetic block — `replay (C-C) … = 3.0 T` and
`two corpus transforms  measured under a second each = 2 s`.

The cited artifact times pass 1, four captures, the capture re-run and the cold check.
It does **not** time the replay pass or either `--labels` pass — lines 55, 59, 71 print
exit codes with no `seconds=` companion, and §8's block brackets no `t=$SECONDS` around
them. The `3.0 T` replay term is a sound *inference* from the measured `g` (replay
re-drives the same turns at the same budget and worker count), and the transforms are
almost certainly sub-second (`corpus_v1.txt` and `corpus_v2.txt` carry the same mtime
minute). Both are true. Neither is measured, and the word *"measured"* is used for the
transforms. Together they are 5 s of 3 310 s. Remedy: label both **ESTIMATED**, or add
`t=$SECONDS` brackets around the replay and the two `--labels` calls in §8 so the pilot
measures what the next document will want.

## m2 — §7's defence of the extrapolation is false about §6's own arithmetic

**Against**: §7, *"§6's arithmetic uses per-TURN and per-POSITION costs and never a
per-game constant, which is what makes the extrapolation survive that difference."*

§6.2 defines `g` as *"the measured seconds per pass-1 GAME"* and `p` as *"the measured
POSITIONS per game"*. Both are per-game constants; they are two of the four measured
costs. The real reason the v1/v2 opening-length difference is harmless is better than
the one given: `p = turn_cap + 1 = 41` for **any** capped game regardless of opening
length, because `asked_prefixes` returns `0..=len(moves)` and `len(moves)` is the cap.
I confirmed the mechanism from the artifact: 144 searches over 4 games = 36
engine-chosen turns per game = `40 − 4` book turns, and `164 = 4 × 41`. The residual
error is on `g` alone: a pilot game has 37 engine turns to a dry-run game's 36, so `g`
is understated by ~2.8 %, i.e. 0.08 T of 254.5 T. Remedy: replace the sentence with the
`p = turn_cap + 1` argument and note the 2.8 % direction on `g`.

## m3 — C-B departs from both dispatches' "sub-range" wording without naming the text

**Against**: §4B, *"**THE REGISTERED RANGE IS THE WHOLE PILOT AND NOT A SUB-RANGE**"*.

Both governing dispatches say *"the determinism re-run receipt on a **sub-range**"*
(`docs/experiments/wp20_dispatches.md`, Development round item 4 and §4). §4B's
justification is sound — neither pass has a subsetting flag, and editing a report
changes `source_sha256` and therefore the capture identity — and the whole range is
strictly stronger than a sub-range, so the requirement is over-satisfied. But the
document argues only from the code and never names the dispatch clause it is reading
past, and it is not a free change: the second capture pass is the `165.0 T` term's
other half, i.e. ~46 % of the pilot's wall. Remedy: one clause naming the dispatch's
*"sub-range"* and stating it is read as a floor, not a ceiling.

## m4 — "zero forfeits" is a dispatch DONE condition; §4C converts it to a non-stopping finding without naming that text

**Against**: §4C's *"A nonzero forfeit count does not stop the arc"* and §5's V2.

`docs/experiments/wp20_dispatches.md` DONE list: *"§4 pilot receipts: cold-label
agreement green, throughput MEASURED, replay_check green, **zero forfeits**,
determinism receipt"* — every neighbour in that list is a pass/fail condition. §4C's
grounds are real (D-544's recorded decision, and `transcript::read`'s legality check),
and reading it as a receipt is defensible. The document should say it is doing so, and
against which sentence. Remedy: name the DONE clause and record the reading.

## m5 — C-B is the one STOP-bearing criterion whose answer is read from output, in a block whose own rule forbids that

**Against**: §8's `sha256sum "$ART/capture_v1.txt" "$ART/capture_v2.txt"` and the
matching corpus line, read against §8's own stated rule: *"**EVERY EXIT CODE IS TAKEN
INTO A VARIABLE AND PRINTED** … because a criterion that STOPS the arc is read from a
code and not from the absence of output."*

`sha256sum` with two paths prints two digests and exits 0 whether or not they match.
C-B's answer is therefore read by eye. Every other STOP-bearing criterion in the
document carries an exit code. The dry run's operator did read it correctly (artifact
lines 49–50 and 60–61 show matching digests) and the two lines are adjacent, so the
practical risk is low. Remedy: `cmp -s a b; echo "capture_identical exit=$?"`.

## m6 — `corpus-check` prints a caller path unguarded (SHELL_CHECKLIST item 9); its sibling guards

**Against**: `crates/pistol-arena/src/bin/corpus-check.rs:50` and `:56`.

Both the ok line and the REFUSED line interpolate `path.display()` into a receipt
somebody parses, with no control-character guard. `tools/cold_label_check.py`'s
`readable()` (lines 88–99) rejects exactly that for all three of its caller paths, and
cites item 9 for doing so. In §8's block every path is author-fixed, so nothing is
reachable here; the asymmetry between two instruments introduced by one document is
what makes it worth a line. Remedy: reject control characters in `paths` before the
first `println!`, or state in the usage block that the caller owns the path.

---

# Instruments answered against `tools/SHELL_CHECKLIST.md`, by number

Both instruments produce a recorded number, so item 10's coverage rule binds both —
including `corpus-check`, which does not live under `tools/`, because `docs/process.md`
says *"living there is not what makes the rule apply."*

### `tools/cold_label_check.py` (at `1311454`; unchanged at HEAD)

| item | answer |
|---|---|
| **8** — one spelling per number, one refusal per reason | **PASS.** `spelled()` (lines 67–85) rejects `+4`, ` 4`, `04` by round-tripping `str(int(w)) != w`, for `--stride` and `--timeout-s`. `readable()` gives *separate* refusals for a control character and for a non-regular file. Driven by `a_stride_spelled_a_way_this_program_will_not_echo_back_is_a_void`. |
| **9** — what reaches a record is caller-controlled | **PASS.** All three caller paths pass `readable()` (lines 88–99), which rejects any byte `< 0x20` or `0x7F` and quotes the input back with `{word!r}` before it can reach a printed line. Not directly driven by a test (no test feeds a newline path) — a gap, not a defect. |
| **10** — THE COVERAGE RULE | **PASS.** Seven tests in `cold_label_check_tests.rs` drive the **shipped** script via `repo().join("tools/cold_label_check.py")` (the helper's own comment cites item 10), each in a `Scratch` directory. **A control run is present and named**: `a_capture_a_fresh_process_reproduces_is_reported_as_agreeing` asserts exit `0` *and* that stdout contains `agree byte for byte`, so a checker that refused everything fails it. |
| **11** — destructive sites | **PASS by enumeration: none.** The script's header states it (lines 26–28), and I confirmed by reading: no `open(..., "w")`, no `unlink`, no `shutil`. |
| **12** — void vs fail, by name | **PASS.** Obligation 1: three codes, `VOID = 2` with a dedicated `Void` exception and a stderr line saying *"this is NOT a disagreement"*. Obligation 2 (preflight) is vacuous — it writes no scratch. Obligation 3: the `meaning()` helper (lines 80–88) spells all three codes into every failure message; four tests assert `Some(2)` distinctly from the `Some(1)` disagreement tests. |

### `crates/pistol-arena/src/bin/corpus-check.rs` (at `1311454`; unchanged at HEAD)

| item | answer |
|---|---|
| **8** | **Vacuous** — the program takes no numeric argument. Nothing states this; harmless. |
| **9** | **PARTIAL — see m6.** `path.display()` reaches both the ok line and the REFUSED line unguarded. |
| **10** | **PASS.** Four tests drive the shipped binary through `env!("CARGO_BIN_EXE_corpus-check")` (labels_tests.rs:531–620). **A control run is present and named**: `a_corpus_this_build_wrote_loads_through_the_reader_that_ships_with_the_writer`, whose doc comment reads *"THE CONTROL. A loader that refused everything would pass every refusal case below and answer nothing"*, asserts exit `0` and the record count. |
| **11** | **PASS by enumeration: none.** Usage: *"It reads and prints. It writes nothing and removes nothing."* Confirmed by reading — the only filesystem call is `std::fs::read_to_string`. |
| **12** | **PASS.** Three codes; `VOID = 2` for "nothing named" and "could not be read", with *"this is NOT a refusal"* on stderr; both void paths and both answer paths have tests whose failure messages spell what the other codes would have meant. |

**So: yes — both suites carry a control run that would fail if the instrument refused
everything, and both name item 10 while doing it.**

### The re-derived normalisation, and whether the error direction is safe (§4A)

The document claims: *"an independent strip can manufacture a mismatch and cannot hide
one."* **I verified this against both implementations and the claim HOLDS.**

`tools/cold_label_check.py:50,195-203` strips ` nps <digits> time <digits>` with one
regex, `count=1`, and **voids** if the pair is not found.
`crates/pistol-arena/src/capture.rs:65-95` hand-parses the same shape — find ` nps `,
require digits, require ` time ` immediately after, require digits, splice out
`[at..end]` — and errors if any step fails. On the engine's real output
(`crates/pistol-cli/src/report.rs:83-84` writes the two fields adjacent and in that
order) the two remove the identical substring; I confirmed against the live capture
data, where the surrounding text joins as `nodes 400384 hashfull 2`.

The direction is safe because each side's strip only **deletes** a matched substring
from **its own** line, and the script normalises **only the cold line**, comparing it
against the capture's already-stored field:

- capture's strip removes **more** than the script's → the stored field is missing text
  the cold line keeps → **mismatch**;
- capture's strip removes **less**, or (D-553's call-removed mutant) is **never
  called** → the stored field keeps ` nps … time …` and the cold line does not →
  **mismatch**;
- the script's strip is wrong → it either voids (`count != 1`) or mismatches.

There is no combination in which a strip defect produces byte-equality between two
lines that genuinely differ. **One honest caveat the document should not be read
past**: this says nothing about the normalisation *policy*. A warm-table effect
appearing **only** in `nps`/`time` is removed on both sides and is invisible to C-A by
construction. That is gate 9's rule, not a defect — but C-A does not see everything,
and §4A's confident phrasing invites the opposite reading.

---

# What §7.2's findings survive

**Finding 2 — "a self-match yields each position twice, and it cannot be configured
away" — VERIFIED, both limbs.**
`crates/pistol-arena/src/capture.rs:128-146` (`one_engine`) refuses a report whose two
seats differ at `binary_sha256`, `config_sha256`, `weights_sha256` or `id_lines`, so a
capture can only be taken from a self-match. And the duplication is structural, not a
setting: `crates/pistol-arena/src/schedule.rs:32` fixes `total = openings.taken.len() * 2`
and line 131 fixes `opening = &openings.taken[index / 2]` with the seats swapped on
parity — there is no unpaired mode and no config key that reaches it. With one
deterministic engine in both seats the two games of a pair are identical, which is what
the artifact's `n 4  distinct-n 2  (2 duplicate games)` records. The finding stands as
written.

**Finding 1 — see M4.** The finding itself is correct and well-measured; the footer
that exempts this pilot from it is what fails.

**Finding 3 — the 2.4 % coldness cost — VERIFIED.** `85/164 = 0.5183` cold against
`83/164 = 0.5061` in-process = `0.0122 s`, i.e. `2.4 %`. `tt_bytes = 268435456` in
`configs/instrument_v0.toml:24` confirms the 256 MiB the document attributes the
overhead to. Taking `c(400000) = l + 0.012` is right physics — a memset's cost does not
scale with a node budget — and it is labelled DERIVED.

# Other tree claims I checked and found sound

- §1's SLOT R1 = `1311454` still governs at HEAD: `git diff --stat 1311454 6e1fea3`
  touches **only** `docs/experiments/wp20_pilot_prereg.md`. No instrument drifted.
- §1's SLOT R2: `sha256sum target/release/pistol` = `180b4c406b225fc8…`, matching both
  configs and §9.1.
- §1's seat claims: `configs/instrument_v0.toml` reads `kind = "staged"` (line 65) and
  `on_search_path = false` (line 113), as §1 says and as D-441/D-534 require.
- §3's label-budget-kind claim: `crates/pistol-arena/src/transcript.rs:172-176` refuses
  a non-`nodes` source budget with *"the run used a `{kind}` budget and only a `nodes`
  budget replays"*; `usage.rs:54` carries the quoted wall-clock sentence.
- §4C's replay quotes are byte-accurate against `usage.rs:76-77` and the
  *"a criterion over some of a report's games is not one anybody registered"* stderr
  against `bin/arena.rs:155-160`.
- §4A's and §4E's usage-block quotes are byte-accurate against
  `tools/cold_label_check.py:35-38` and `bin/corpus-check.rs:23-26`; §4A's required
  output line matches the script's line 251 exactly.
- §4D's *"pass 2 is serial by construction"*: `capture::run` builds a one-element
  `seats` array (capture.rs:246-249).
- §6.3's four measured costs all re-derive from the cited artifact:
  `6/4 = 1.5`, `164/4 = 41`, `165/164 = 1.006`, `85/164 = 0.518`; and the arithmetic
  sums correctly (`3.0 + 165.0 + 83.5 + 3.0 = 254.5`; `2×41 ≥ 1000 ⇒ T ≥ 12.2 ⇒ 13`;
  `254.5 × 13 + 2 = 3 310`; and the pre-amendment `T ≤ 56.57 ⇒ 56`).
- §2 and the ledger row agree with the config: `openings_skip = 0`, `openings_take = 13`,
  range `0..12`, added in the same commit as the arena config (`git show --stat 1311454`).
  The consumed-range receipt is present and correct; only its *gloss* fails (M7).
- `tools/design_citation_check.py` returns `52 citation(s) checked, 0 unreproduced`.
  As the instrument's own output says, that means the citations are real, not that the
  claims built on them are right — which is why M1, M5, M6 and M7 are all claims whose
  cited paths exist and whose content says something else.

---

# THE STRONGEST ATTACK THE DOCUMENT SURVIVED

*(Recorded in full, because this is what the ADR line should carry.)*

**The attack.** C-A is vacuous, and it is the pilot's most expensive criterion.

The argument runs: `arena --capture` sends `newgame` before **every** ask —
`crates/pistol-arena/src/capture.rs:196` iterates
`[pistol_cli::protocol::NEW_GAME, position, go]` for each position. And
`tools/cold_label_check.py:160` builds its referent script as
`"newgame\n{position}\n{go}\nquit\n"` — **the referent sends `newgame` too**. So both
sides of the comparison are "a `newgame` followed by this position at this budget."
D-542 and D-549 record the coldness chain as already verified by reading, twice, in
detail: `newgame` → `Pistol`'s three fields → `Searcher`'s six, `clear` reaching three,
`Table::clear` a true `fill(EMPTY)` and not the epoch bump beside it, `Solver::reset`
rebuilding, `Position::reset_to` unwinding the eval, the `PvTable` not being a
`Searcher` field at all. If `newgame` is a true table wipe, the two sides are the same
computation by construction, and `docs/process.md` is explicit that *"a criterion that
is a property the named defect class PRESERVES … passes vacuously and is not a
criterion."* The pilot then spends 1 066 process spawns × 1.018 s ≈ **18 minutes, a
third of its whole wall**, confirming a theorem.

**Why the document survives it — and this is the reviewer's answer, not the
document's.** The attack smuggles in its own conclusion. *"`newgame` is a true wipe"*
is not a premise C-A may assume; it is the **proposition under test**, and D-542
establishes it by **reading, with no build** — its own words are *"FOUR CLAIMS VERIFIED
BY READING, NO BUILD."* C-A's defect class is not "a warm table" in the abstract; it is
**"the reading was wrong"**, and a reading is falsified by execution and by nothing
else. The two sides are *not* symmetric: the referent's tables were never touched by
any other `go` in that process's life, while the subject's have had every prior
position of the walk pass through them. Symmetry of the `newgame` **request** is not
symmetry of the **state**.

And it is falsifiable *in the compared bytes*, which I checked rather than assumed.
`crates/pistol-cli/src/report.rs:83-84` writes the totals line as
`depth_turns … seldepth … nodes …{solver} nps … time … hashfull … score … pv …`, and
normalisation removes only ` nps <n> time <n>`. So the compared field carries
**`hashfull`** — a direct readout of transposition-table occupancy. Over the dry run's
own 164 records at `nodes 400000` I measured `hashfull` taking **14 distinct values,
0 through 13**, while `nodes` takes only 5 (it is pinned near 400 384 by the budget).
A `clear` that missed the table — or missed the killer/history tables, which is the
classic miss — makes the subject's `hashfull` climb across the walk while the
referent's cannot, and 164 of 164 records would not have agreed. `depth_turns`,
`seldepth`, `score` and `pv` all move on table state too.

So C-A is exactly what `docs/process.md` says a reviewer *"looks for first"*: **an
externally derived referent, a value computed by something that does not share the
suspect input.** The stage under doubt is named (the capture pass's table state), the
way the second instrument fails to share it is named (a process per position against
one process for the whole walk), the agreement criterion is registered before either
runs, and its disagreement consequence — **exit 1 STOPS THE ARC**, not re-run, not
averaged, not attributed to noise — is registered with it. The document has satisfied
every clause of *"Cost, replication, and the second instrument"* on the one criterion
that carries D-540's obligation, and it did so in the shadow of D-527, where the check
meant to catch this exact defect *"passed vacuously on the two bands a warm table
cannot move."*

**C-A is not vacuous. It is the best-constructed thing in this document, and the
18 minutes are earned.**
