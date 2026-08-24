# REVIEW — `docs/experiments/wp16_sprt_prereg.md` revision 5

**Revision reviewed**: `de53f5d2da4e1a9adb12439f44fb07e0f601eb73` (`de53f5d`,
"docs(experiments,decisions): wp16_sprt_prereg.md revision 5 -- quote the
instrument").
**Still matches HEAD?** YES. `git rev-parse HEAD` was `de53f5d2da…` at the start
of this review and `de53f5d2da…` at the end; `git status --porcelain` was empty
at both points (this report is the only file this review creates).
**Boundary command**:

```
$ git diff --stat bfdf933..de53f5d -- crates/ tools/
$                                        # prints nothing
```

**The scoping therefore HOLDS.** The six items the dispatch named out of scope
were not re-litigated: the mutation table, the two release-binary digests,
§8.6's re-execution, the clause-(b) proof, the agreement criterion's
false-disagreement mode, and §3's fresh opening slice.
**Reviewer**: fresh-context subagent, Claude Opus 5 (1M context).
**Date**: 2026-08-24.
**Scope**: (1) the eight D-416 findings, (2) the receipt audit of §7A.1 against
the shipped instrument at `bfdf933`, (3) the document-only diff and its internal
consistency.

---

## VERDICT: **FAIL** — 0 BLOCKING, 2 MAJOR, 7 MINOR

**The governed run must not be launched at this revision.**

Eight of eight D-416 findings are closed or substantially closed, and the
receipt audit is overwhelmingly clean: **every message string §7A.1 quotes
matches the shipped instrument exactly**, every driving test it names exists,
passes, and drives the path claimed, and every exit code it cites matches the
instrument's own constants. That is a real improvement and it is recorded first.

The FAIL rests on two MAJORs, both of which are the *same defect class this
revision exists to close*, surviving one level down:

- §7A.1 declares "**the reader distinguishes them by the message text, which is
  why every one of those messages is quoted verbatim above**" about its exit-2
  partition. The instrument has **49 `die()` call sites**; §7A.1 quotes **7**.
  Neither the kind-(i) nor the kind-(ii) enumeration is exhaustive, and a
  reachable refusal on a self-contradictory report matches neither.
- §7A.1's registered receipt for the third premise refusal — "Each prints under
  `warm_attribution_check: CANNOT READ:` and exits **2**" — is **false** for a
  reachable input of that very refusal: the instrument raises an uncaught
  `StopIteration`, prints no `CANNOT READ:` line at all, and exits **1**.

Both are DOCUMENT findings in the same direction D-417 records — the instrument
is stricter, never looser, and **no false PASS is reachable through either**.
Neither requires an instrument change to fix (MAJOR B is closable by narrowing
the document's receipt; whether the instrument should also be hardened is an
operator call and outside this review's remit, since the boundary forbids
touching `tools/`).

---

## Duty 1 — the eight D-416 findings

| # | Finding | Verdict |
|---|---|---|
| BLOCKING 1 | §7A.1 registered a criterion the instrument no longer implemented (6 limbs) | **CLOSED**, 6 of 6 limbs — but see MAJOR A and MAJOR B, which are new defects inside the fix |
| MAJOR 2 | exit 3 blamed the ENGINE | **CLOSED** |
| MAJOR 3 | §11/§7A.1 silent on D-413/D-414/D-415 | **CLOSED** |
| MINOR 4 | header's account of the revisions wrong twice | **CLOSED** for revisions 3 and 4; **recurs for revision 5** — see MINOR G |
| MINOR 5 | Criterion 1'' quote's provenance wrong, quote truncated | **CLOSED** |
| MINOR 6 | §8.6's wall figures unsourceable | **CLOSED** |
| MINOR 7 | §7's cost table stale, §7A.2's sweep unpriced | **PARTIALLY CLOSED** — 3 of 4 limbs; see MINOR D and MINOR E |
| MINOR 8 | §10's pinning claim looser than §10 said | **CLOSED** — see MINOR F for a residual |

### BLOCKING 1 — CLOSED, limb by limb

Each limb was checked against the shipped instrument, not against the prose.

**(a) The three premise refusals.** §7A.1 gains a table of all three
(`clause_b`, `tools/wp16_warm_attribution_check.py:598-620`), each with the
instrument's own message. All three quotations verified **character-exact**
modulo f-string placeholders (`{X}`, `{p1}`, `{t}`, `{n}`) and line-wrapping:

```
$ python3 - <<'PY'   # adjacent-literal-joined source vs the document's quotes
… OK  its two games declare openings {X} and {Y}, so they are not one opening played…
… OK  its two games seat `{p1}`/`{p2}` and `{p1}`/`{p2}`, which is not one seating…
… OK  its two games differ at turn {t}, which is inside the {n}-turn book, so they…
PY
```

The `CANNOT READ:` prefix is `die()`'s (`:167-169`) and `NO_ANSWER = 2`
(`:134`). Driving test named and verified: it seeds all three and asserts
`Some(2)` plus a `must_say` substring on each
(`crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs:744-796`).

**(b) The `status` cross-check and the halt invariant.** §7A.1 gains a
subsection. Both quoted messages verified exact against `:457` and `:460-463`,
and the halt message against `:470-474`. `compared_turns == at_turn -
opening_turns` (`:476`) and the clean-record derivations (`:484`, `:491`) are
stated as invariants rather than quoted, which the receipt rule permits since
the section names the driving test. Driving test verified: three seeded
documents in a `Vec` plus a fourth halt-invariant document, each asserted
`Some(2)` (`:803-869`).

**(c) The truncated inert-cross-check sentence.** Restored. The quote is now
verbatim against design revision 3 IN FULL, ending exactly where design §4
point 4 ends ("…not as the thing the verdict depends on."):

```
$ python3 …  # normalized substring test
b6afd66 VERBATIM SUBSTRING: True
d9a2852 VERBATIM SUBSTRING: False
```

and §7A.1 registers the cross-check's real consequence. All four cross-check
strings verified exact against `cross_check` (`:659-703`): the no-op note, the
forfeit skip (`skipped, not silently passed`), the exit-2 self-check refusal,
and the exit-1 verdict-moving failure.

**(d) The enumeration is now exhaustive over four cases.** The partition
(equal/unequal × forfeit/no-forfeit) is exhaustive by construction and matches
`clause_b`'s control flow exactly. Both `(b) pair …` failure messages verified
exact. The identical-plus-forfeited case that matched no revision-4 bullet is
row 2, with its own test. One MINOR over-claim in row 1's justification — see
MINOR H.

**(e) The exit-2 registered reading for a premise refusal.** Corrected: §5
gains a second exit-2 row and §7A.1's exit table splits kind (i) from kind (ii).
The two sections describe the **same** partition, word for word. The correction
itself is right; its *coverage* is MAJOR A.

### MAJOR 2 — CLOSED

Both §5's row and §7A.1's exit-3 row now name both causes. §7A.1 quotes the
instrument's own second printed line, and the quotation is a **complete
sentence** in the source (`violation()`, `:172-183`) — no truncation this time.
All three exit-3 driving tests exist and pass.

```
$ grep -n "instrument-mode guarantee (CLAUDE.md rule 4) is failing" \
    docs/experiments/wp16_sprt_prereg.md
$                                        # gone from both sections
```

### MAJOR 3 — CLOSED

§7A.1 gains "THE INSTRUMENT'S OWN REVIEW HISTORY" (D-413's FAIL with its
counts, D-414's fix round, D-415's waiver and its two transferred duties) and
§11 gains a matching paragraph. `D-413`, `D-414`, `D-415`, `D-416` all now
appear; all four ADR lines exist in `docs/decisions.md` and the citations are
accurate against them.

### MINOR 4 — CLOSED for revisions 3–4, RECURS for revision 5

The corrected claim was checked against the actual diff:

```
$ git diff -U0 731150a..de53f5d -- docs/experiments/wp16_sprt_prereg.md | grep "^@@"
@@ -3 …          header
@@ -158,2 …      §3, TWO rows
@@ -247,2 …      §5
@@ -301 / -303 … §7
@@ -316,9 / -326,5 / -332,5 …  §7A.1
@@ -588,0 …      §8.5 + §8.6
@@ -603,0 …      §9 (§9.2a)
@@ -633,3 …      §10
@@ -650,5 …      §11
```

§1, §2, §4, §6, §7A.2 and §8.1–§8.4 are genuinely untouched; §3 is two rows;
§8.5 is touched. The header now says all three correctly. See MINOR G for the
new instance.

### MINOR 5 — CLOSED

Provenance corrected to design revision 3 (`b6afd66`), with revision 2's single
wording difference named. Verified: `git diff d9a2852 b6afd66` on the design
document shows the point-4 paragraph differing **only** in `§4 point 3's` →
`point 3's`, twice, exactly as the document states.

### MINOR 6 — CLOSED

§8.6 now cites the artefacts' own lines:

```
$ grep wall_ms artifacts/wp16_warmreplay_dryrun_run.txt
timing n_workers 4 wall_ms 14341 discarded_in_flight 0 hang_timeout_ms 120000
$ grep wall_ms artifacts/wp16_warmreplay_dryrun_replay.txt
timing n_workers 4 wall_ms 14305 hang_timeout_ms 120000
```

`14305 / 14341 = 0.99749…` → `0.997x`, as registered. The shell-`time` figures
are kept and explicitly demoted ("the artefact figures are the registered
ones"), which is the right disposal. §7's row is now stale against it — MINOR C.

### MINOR 7 — PARTIALLY CLOSED (3 of 4 limbs)

- "one Criterion 1' run" → the actual Criterion 1'' chain. **CLOSED.**
- §7A.2's sweep unpriced → it has a row now. **CLOSED**, but the row carries two
  new defects (MINOR D, MINOR E).
- the attention row omitted the second instrument → now named. **CLOSED.**
- the second-instrument row's "minutes at `openings_take = 500`" — an unmarked
  extrapolation sitting in the **MEASURED** column — is **unchanged, verbatim**.
  Recorded as an open residual rather than re-raised, since D-416 itself flagged
  it with a mitigation.

### MINOR 8 — CLOSED

§10's blanket claim is replaced by a five-row table with a `Where` column, and
every row's `Where` is true. `tools/baseline_snapshot.sh` is correctly shown as
pinned in **§7A.2**, not §7A.1; §7A.2 pins it at `9282dd0`, and that pin is
valid — `271cdf1` (the file's last modification) is an ancestor of `9282dd0`,
and `git diff --stat 9282dd0..de53f5d -- tools/baseline_snapshot.sh` is empty.
`§10`'s "Pinned at: its own revision" for that row is vaguer than §7A.2's
`9282dd0` but is not false. See MINOR F for the `wp15b` row.

---

## Duty 2 — the receipt audit against the SHIPPED instrument

### What is CLEAN, stated first

**Message quotations.** Every inline-code span in §7A.1 of ≥ 25 characters was
mechanically extracted and matched against the instrument's string literals
(adjacent literals joined, `\"` unescaped, whitespace normalized, `{…}`
placeholders wildcarded). **Every quotation of an instrument string matched.**
Twelve distinct strings: the three premise refusals, the `CANNOT READ:` prefix,
the two `status` refusals, the halt refusal, the two `(b) pair …` messages, the
three cross-check messages, and the exit-3 second line. **No paraphrase was
found presented as a quotation.** The two `(b) pair …` quotes use `…` and
`{o}`-style placeholders — both honestly marked; the underlying text matches
character for character.

**Driving tests.** All ten named tests exist in
`crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs`, and the suite is
green at HEAD:

```
$ cargo test -p pistol-cli --test wp16_warm_attribution_check_tests
running 11 tests
… test result: ok. 11 passed; 0 failed
```

Each was read and drives the path §7A.1 claims:
`a_pair_that_does_not_satisfy_the_proofs_premise_…` seeds all three premises and
requires exit 2 on each (`:744`); `a_replay_record_cannot_skip_…` seeds four
documents and requires exit 2 on each (`:803`);
`an_inert_pair_is_excluded_…_no_op` asserts the exact words
``leaves the verdict `inconclusive_degenerate` unchanged`` (`:466`);
`a_forfeit_sibling_of_an_inert_pair_is_not_excluded` asserts `Some(1)` and the
forfeit clause (`:497`). One small imprecision: §7A.1 says the coverage test
runs "against a control that must pass", and that test builds an `honest`
document but never asserts a pass on it — the control is the *suite-level*
`a_clean_replay_of_an_honest_report_is_attributable` over the same fixture
family. Not raised as a finding: the control exists and the file's own docstring
frames it that way (SHELL_CHECKLIST item 10).

**Exit codes.** `ATTRIBUTABLE = 0`, `NOT_A_MEASUREMENT = 1`, `NO_ANSWER = 2`,
`DETERMINISM_VIOLATION = 3` verified at `:132-135`, and every code §7A.1 cites
was traced to the path that reaches it (`die` → 2, `violation` → 3, `main`'s
`return ATTRIBUTABLE if not failures else NOT_A_MEASUREMENT` at `:811`).

**Clause (b)'s four-case table is exhaustive** over the partition it names, and
the four cases map one-to-one onto `clause_b`'s branches. The four edge cases
the dispatch asked about were each driven against the shipped instrument:
a capped pair (covered by rows 1/3), `opening_turns` equal to the full move list
(row 1), both games pure book (row 1 — see MINOR H), and a game with fewer
recorded moves than the book (**MAJOR B**).

### MAJOR A — §7A.1's exit-2 partition is not exhaustive, and its "every one of those messages is quoted verbatim above" is false

§7A.1's exit table, exit-2 row, closes:

> The instrument prints the same `CANNOT READ:` prefix for both, so **the reader
> distinguishes them by the message text, which is why every one of those
> messages is quoted verbatim above**

The two kinds are closed lists:

- **(i) a VOID** — "a missing or unrunnable engine, an unreadable document, an
  incomplete replay pass, a budget this cannot replay"
- **(ii) a REFUSED REPORT** — "any of the three premise refusals, the
  `status`/halt refusals, or the cross-check's self-check failing"

Two things are wrong.

**1. The claim about the quotations is false.** The instrument has 49 `die()`
call sites; §7A.1 quotes 7 of them. **Not one kind-(i) message is quoted
anywhere in the document** — not the unrunnable-engine refusal (`:236`), not the
non-UTF-8 / not-a-report refusals (`:253`, `:259`), not the abandoned-pass
refusal (`:336`), not the incomplete-coverage refusals (`:419`, `:424`, `:437`),
not the non-`nodes` budget refusal (`:284`). So the reader cannot do the thing
the sentence tells them to do — distinguish (i) from (ii) by text — for kind (i)
at all.

```
$ grep -c "die(" tools/wp16_warm_attribution_check.py
50                        # 49 call sites + the def
```

**2. A large residue belongs to neither list, and lands on the reading revision
5 itself declares wrong.** Roughly a dozen refusals are report-internal
contradictions — facts about the report, with no void to fix and nothing to
re-take: `both seats carry the label X` (`:278`), `N games is not an even
number` (`:303`), `the two documents disagree on X` (`:410`), `the replay
document records game i twice` (`:364`), a divergence reported at a book turn
(`:539`), a divergence past the end of the game (`:544`), the replay's
`recorded` move disagreeing with the report's move list (`:548`), and the
`turns`-vs-`moves` contradiction below. By the document's own closed lists these
are not kind (ii), so a reader falls back on kind (i) — "the void is fixed and
the answer re-taken" — which is precisely the reading D-416's BLOCKING 1 named
as wrong and D-417 records as corrected.

**Minimal reproducer.** The pinned §8.6 honest artefacts, with game 0's `turns`
field changed from `40` to `39` and nothing else (the replay's
`source_report_sha256` re-pointed so `bind` passes):

```
$ python3 tools/wp16_warm_attribution_check.py <scratch>/c2_run.txt \
      <scratch>/c2_replay.txt target/release/pistol
warm_attribution_check: CANNOT READ: game 0: `turns 39` against 40 recorded turns
$ echo $?
2
```

That message is quoted nowhere in the document, and matches neither closed
enumeration. It is unambiguously a report the arena could not have written — the
kind-(ii) consequence — but the document routes it to kind (i).

**What would close it**: either make kind (ii) the *default* for any refusal
naming an internal contradiction of the two documents and kind (i) the closed
list (which it can be — kind (i)'s four categories genuinely are enumerable), or
state that the enumeration is illustrative and give the reader a rule that
partitions the whole space. Deleting the "every one of those messages is quoted
verbatim above" clause is required either way.

### MAJOR B — the registered receipt "exits 2" is false for a reachable input of the book-prefix premise refusal: the instrument crashes and exits 1

§7A.1 registers, for all three premise refusals: "Each prints under
`warm_attribution_check: CANNOT READ:` and exits **2**."

The book-prefix refusal's guard is `if one[:book] != two[:book]:` (`:613`), and
its first act is

```python
spot = next(at for at in range(min(len(one), len(two), book)) if one[at] != two[at])
```

Two Python slices can be unequal because they differ in **length**, in which case
no index in `range(min(len(one), len(two), book))` differs and `next()` raises
`StopIteration`. `__main__` catches `(KeyError, ValueError, IndexError)`
(`:822`) — **`StopIteration` is not among them**, so it escapes as a traceback.

**Minimal reproducer.** The pinned §8.6 honest artefacts, with game 0's move
list truncated to its first 2 turns (`opening_turns` is 4), `turns 40 → turns 2`,
and replay record 0 adjusted to `recorded_turns 2 replayed_turns 2
compared_turns 0` (all coverage checks still pass; the replay's
`source_report_sha256` re-pointed so `bind` passes):

```
$ python3 tools/wp16_warm_attribution_check.py <scratch>/craft_run.txt \
      <scratch>/craft_replay.txt target/release/pistol
Traceback (most recent call last):
  File ".../tools/wp16_warm_attribution_check.py", line 821, in <module>
    raise SystemExit(main())
  File ".../tools/wp16_warm_attribution_check.py", line 793, in main
    inert = clause_b(report, replay, buckets, notes, failures)
  File ".../tools/wp16_warm_attribution_check.py", line 614, in clause_b
    spot = next(at for at in range(min(len(one), len(two), book)) if one[at] != two[at])
StopIteration
$ echo $?
1
```

No `CANNOT READ:` line is printed. Exit **1**, whose registered consequence in
§7A.1 is "THE RUN IS NOT A MEASUREMENT", reached by "a confirmed inversion; an
unattributable pair; an inert pair whose bucket contradicts the theorem; a
cross-check that moves the verdict; a broken link 1b or 1c" — none of which is
what happened. This is the exact misreading the instrument's own comment says the
`try/except` exists to prevent: "*it catches them into exit 2 rather than letting
a traceback exit 1 and read as 'the run's seats are mis-attributed'*" (`:815-819`).

**Direction and reachability, stated so the severity is not over-read.** No exit
0 is reachable through this — no false PASS. The input requires a game whose
recorded move list is shorter than `opening_turns`, which the arena does not
write; but `clause_b`'s own docstring is explicit that the premise exists
because "*the reports it exists to judge are exactly the ones that might not be
what they say they are*", so a corrupted report is this check's declared threat
model, and this is the one premise arm that answers it with a traceback.

**What would close it as a DOCUMENT finding** (the boundary forbids touching
`tools/`): §7A.1 narrows the receipt to the case the instrument actually refuses
at exit 2 and registers the crash path, or the operator licenses a one-line
instrument fix (`next(…, None)` with a fallback message, or adding
`StopIteration` to the caught tuple) — which would be an instrument change and
would reopen this review.

---

## Duty 3 — the diff is document-only, and internally consistent

**Files changed** (`git diff 20f9b26..de53f5d --stat`):

```
 docs/decisions.md                           |   6 +
 docs/experiments/wp16_prereg_rev4_REVIEW.md | 593 ++++++
 docs/experiments/wp16_sprt_prereg.md        | 403 ++++---
```

Document-only, as claimed. `docs/decisions.md` gains D-417 (the eight closures)
and D-418 (the standing receipts rule); both were read and their accounts of
revision 5 are accurate.

**§5 vs §7A.1**: they describe the **same** partition. §5's two new exit-2 rows
and §7A.1's exit-2 row use the same two enumerations verbatim. They also share
MAJOR A's non-exhaustiveness identically, so the defect is consistent rather
than contradictory. §5's exit-3 row and §7A.1's exit-3 row also agree, and both
now match the instrument.

**§11's revision table** is honest about all five revisions, including
revision 3's withdrawn review and revision 4's FAIL with D-416's counts, and the
added paragraph states that the *instrument* failed its REVIEW-impl and that its
second review was waived. Verified against D-413/D-414/D-415/D-416.

**§10's new table**: every `Where` claim is true (checked row by row against
§7A.1 and §7A.2). One residual — MINOR F.

**Numeric marking**: every numeric claim in §7 and §8.6 carries MEASURED,
ESTIMATED or DECLARED. Two problems with the new ESTIMATED row (MINOR D, MINOR E)
and one stale MEASURED count (MINOR C).

---

## New findings, most severe first

*(MAJOR A and MAJOR B are stated in full under Duty 2 above.)*

### MINOR C — §7 says the replay cost was MEASURED "three times"; §8.6, rewritten in the same revision, says FOUR

```
$ grep -n "three times\|FOUR TIMES\|Four samples" docs/experiments/wp16_sprt_prereg.md
369:… | **MEASURED, §8.6, three times: `0.997x`, `1.003x` and `0.994x`** — …
1006:**THE REPLAY'S COST IS MEASURED, AND IT HAS NOW BEEN MEASURED FOUR TIMES.**
1017:Four samples straddling 1.0 is the honest reading, …
```

Revision 5 added the fourth sample (revision 4's reviewer's `1.003x`, D-416) to
§8.6 and did not update §7's row, which still cites three and omits the new one.
`git diff 20f9b26..de53f5d` shows the §8.6 edit (`-Three samples` / `+Four
samples`) with no corresponding §7 edit. The registered reading ("about one
run") is unaffected.

### MINOR D — §7's new sweep row contradicts itself and §7A.2 about which step the sweep runs at

The row title says "**§7A.2's own Step-6 sweep**"; the same row's right-hand
column says "the sweep itself, **Step 7**"; and §7A.2 (untouched) says "**TO BE
RUN AT STEP 6**, alongside the governed SPRT run". The governed-run row directly
above says "the run itself, **Step 6**". Three statements, two step numbers, one
table.

```
$ grep -n "Step-6 sweep\|the sweep itself\|TO BE RUN AT STEP" \
      docs/experiments/wp16_sprt_prereg.md
375:| §7A.2's own Step-6 sweep (the completed-depth comparison) | ESTIMATED **~2 min**, … | the sweep itself, Step 7 |
711:**TO BE RUN AT STEP 6**, alongside the governed SPRT run, …
```

### MINOR E — the sweep's `~2 min` ESTIMATE is ~8x what the document's own MEASURED figures imply, and was measurable in seconds

The row anchors "~2 min" to §9.5's calibration probe — but to that probe's
**DECLARED** figure, not to its measurement. §9.5's probe MEASURED "two
independent 24-position sweeps at `go nodes 50000`: worst single search **291 ms**
(run 1) / 289 ms (run 2)". 24 × 0.291 s ≈ **7 s** per seat, ≈ 14 s for the
two-seat comparison. The probe ran twice **this session** at exactly the
registered shape and budget, and its wall time is recorded nowhere in the
document. CLAUDE.md: "an estimate that could have been measured in seconds is a
finding" (D-291 precedent). The direction is conservative (over-priced, so the
cost disclosure is not understated), which is why this is MINOR rather than
MAJOR.

### MINOR F — `tools/wp15b_attribution_check.py` is pinned at two different revisions in one document

§7A.1's second-instrument paragraph and §10's table both say `bfdf933`. §8.2
(revision 2's text, untouched) says "**NAMED HERE WITH ITS REVISION** — the
commit this document lands at", which now resolves to `de53f5d`. §10's new table
— which exists to make exactly this bookkeeping true — names only the `bfdf933`
pin and does not mention §8.2's.

Materially harmless and verified so: `git diff --stat 8ca4063..bfdf933 --
tools/wp15b_attribution_check.py` is empty, the boundary command shows `tools/`
unchanged from `bfdf933` to `de53f5d`, and the file was last modified at
`a80a864`, long before this WP. Both pins therefore denote identical content.

### MINOR G — the header's "NEW IN REVISION 5" list omits §11, which revision 5 rewrote

The header reads: "…and, **NEW IN REVISION 5**, a rewritten §7A.1 and
corrections in §5, §7, §8.6 and §10." §11 is not in that list; it appears only
in the *preceding* list, which the header's own appositive labels "revisions 3
and 4's own amendments". But revision 5 rewrote §11 substantially — two table
rows and the entire instrument-review-record paragraph, which **is** MAJOR 3's
fix:

```
$ git diff -U0 20f9b26..de53f5d -- docs/experiments/wp16_sprt_prereg.md | grep "^@@" | tail -2
@@ -911,9 +1102 @@ …      # §11
@@ -921,9 +1104,23 @@ …    # §11
```

The header itself is also rewritten and unlisted. Same class as D-416's
MINOR 4 — a section the document's own account of the revision leaves out.

### MINOR H — the inert row's stated ground over-claims for a pair whose games are pure book

§7A.1's clause-(b) row 1 grounds the exclusion on "**both credited engines
warm-replayed every move**, so the two seats are indistinguishable at every
position either game reached". For a pair whose two games are the book and
nothing more, `compared_turns` is 0 — the replay compared nothing — yet
`clause_b` excludes the pair as inert on the strength of `one == two` alone.

**Reproducer** (the §8.6 artefacts with games 0 and 1 truncated to the 4-turn
book, both `capped`, replay records set to `recorded_turns 4 replayed_turns 4
compared_turns 0`): the run reaches `cross_check`'s self-check, which is only
reachable when `inert` is **non-empty** (`:668` returns early on an empty
`inert`) —

```
$ python3 tools/wp16_warm_attribution_check.py <scratch>/pb_run.txt \
      <scratch>/pb_replay.txt target/release/pistol
warm_attribution_check: CANNOT READ: this tool's ported sprt.rs/score.rs arithmetic
recomputes `inconclusive_degenerate` off the report's own unmodified pentanomial,
against its printed `verdict inconclusive_at_game_cap` — …
$ echo $?
2
```

— so the pure-book pair *was* accepted as inert with zero compared turns. **The
conclusion survives**: the theorem's real content is that identical move lists
force a 1-1 split whatever the replay saw, and the row says that too ("swapping
the labels could not have changed a board at any ply"). Only the first clause
over-claims, and the arena cannot write such a report. MINOR.

### MINOR I — §7A.1's most load-bearing "what the instrument does" sentence carries no receipt, against the rule the section states about itself

§7A.1's rule: "A sentence here that cannot be traced to a quoted string, an exit
code or a test name is a defect by construction." The warm-drive premise — the
sentence the entire criterion and the second instrument's stage-separation rest
on — carries none of the three:

> `arena --replay` spawns BOTH seats of every game through the same
> `seats::with_seats` the generation path calls, feeds the report's own recorded
> move list, and asks each seat at every one of its own turns THAT HAS A
> RECORDED MOVE, through the same `exchange::ask` the referee calls

The receipts exist and are named nowhere in the document: D-414 records
`the_replay_path_sends_newgame_on_every_fresh_spawn_too` and
`every_fresh_spawn_is_sent_newgame_before_it_is_given_a_position` as the tests
that pin the spawn sequence, and `crates/pistol-arena/tests/replay_chain_tests.rs`
as the test that drives the shipped chain end to end. The same gap applies to
the second-instrument bullet ("the cold checker spawns one fresh process per
query and never drives a game at all") and to the exit-1 row's "a broken link 1b
or 1c". Five much smaller claims in the same section carry full receipts; this
one does not.

---

## Attacks ATTEMPTED and REJECTED

- **Does §5 contradict §7A.1 on the exit-2 split?** No. Word-for-word the same
  two enumerations, same order, same consequences. Rejected.
- **Is the four-case clause-(b) table missing a fifth case?** No — the partition
  (lists equal or not) × (a forfeit or not, when equal) is complete, and every
  branch of `clause_b` is covered. The edge cases I probed produced MAJOR B (a
  crash inside an existing case) and MINOR H (an over-claimed justification
  inside an existing case), not a missing case. Rejected as a table defect.
- **Does the §7A.2 sweep's estimate hide a cost the run cannot afford?** No — the
  error is in the conservative direction (MINOR E). Rejected as a cost-disclosure
  failure.
- **Is the Criterion 1'' quote truncated again?** No. It ends exactly where
  design §4 point 4 ends; point 5 begins on the next line. Verified by reading
  `b6afd66:docs/experiments/wp16_warm_replay_design.md:297-310`. Rejected.
- **Is any named driving test absent or non-driving?** No. All ten exist, all 11
  tests in the target pass, and each was read against the path it is cited for.
  Rejected.

---

## Is there anything this document claims that I could not verify?

Yes — six things, recorded rather than assumed.

1. **§8.6's `0.994x` third sample**, attributed to D-413's reviewer on its own
   machine state. Another machine's state cannot be reproduced. Inherited
   unresolved from D-416.
2. **§8.6's `1.003x` fourth sample**, revision 4's reviewer's independent
   re-execution (D-416). Recorded in that report; not re-taken here (out of
   scope by the dispatch).
3. **§7's governed-run ESTIMATE** (`~2-3 core-hours, ~35-50 min wall at 4
   workers`). Unverifiable before the run, correctly marked ESTIMATE, derivation
   stated.
4. **§7's `~2 min` sweep ESTIMATE** — see MINOR E. Not measured, and the
   document offers no measurement to check it against.
5. **§7's dry-run figure "MEASURED: 14.254 s of arena wall time"** (§8.4's run,
   revision-2 text). §8.4's artefact is named by digest but not committed
   (rule 8), and I have no artefact carrying `14254`. Unlike §8.6's figures
   (MINOR 6, now fixed), this one has no pinned artefact to be sourced to.
6. **§9.4's "no further amendment to `docs/wp16_quiescence_design.md` between
   D-394 and the run's launch revision"** — a Step-6 slot by construction,
   dischargeable only at launch.

Out-of-scope items were not re-verified and are **not** listed above as
unverified: the mutation table, the two binary digests, §8.6's re-execution, the
clause-(b) proof, the agreement criterion's false-disagreement mode, and §3's
fresh slice all stand on `docs/experiments/wp16_prereg_rev4_REVIEW.md` at the
same instrument revision, which the boundary command confirms is undisturbed.

Rule 8 was re-checked and holds: `git status --porcelain` empty, `/artifacts/`
gitignored, no artefact added by this revision.

`tools/SHELL_CHECKLIST.md` was NOT applied as a review target: this revision
touches no file under `tools/`, which the boundary command establishes.

---

## What would unstick it

Every finding is a DOCUMENT change. A revision 6 would:

- **MAJOR A** — delete "every one of those messages is quoted verbatim above",
  make kind (i) the closed list and kind (ii) the residue (or vice versa with a
  stated rule), so that every one of the 49 refusals has a registered reading.
- **MAJOR B** — narrow the "exits 2" receipt to what the instrument actually
  does, and register the crash path; or obtain a licence to harden `clause_b`,
  which would move the instrument and reopen this review.
- **MINOR C–I** — a stale count, a step number, an over-priced estimate, a
  double pin, an incomplete header list, an over-claimed justification clause,
  and three missing receipts.

None of them requires re-opening the instrument, and none of them makes a false
PASS reachable — the direction D-417 records still holds: the instrument is
stricter than the document says, never looser.
