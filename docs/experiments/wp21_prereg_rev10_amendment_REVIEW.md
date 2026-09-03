# REVIEW of the WP-2.1 revision-10 amendment remedy (round 3, remedies-only)

**REVISION REVIEWED**: `4e3d1b6` on `dev` (HEAD at dispatch). `git rev-parse HEAD`
at the end of this review, both the live tree and the review's own worktree:
`4e3d1b6f6340953884dbaef9c09cabd709c1e8ed` — **still equals `4e3d1b6`**, unchanged
throughout.

**REVIEWED ON: Sonnet, per D-597.**

Scope, per the dispatch: `git diff 8a73bec 4e3d1b6 -- docs/experiments/wp21_prereg.md`
— three changed lines (title, the lower-bound paragraph, one shell comment) —
whether they introduced anything, and one closing sweep of my own choosing over
`wp21_prereg.md` and `wp21_throughput_prereg.md`. Round 1
(`wp21_prereg_rev8_amendment_REVIEW.md`) and round 2
(`wp21_prereg_rev9_amendment_REVIEW.md`, at `8a73bec`) already settled N = 16, C1–C4,
the wall block's own arithmetic, and F-1–F-5; none of that is re-derived here
except where re-checking round 2's F-6 remedy required touching the same numbers.

Worktree used: `git -C /home/tom/Projects/HeXO-AlphaBeta worktree add --detach
/home/tom/pistol-wt/amend3 4e3d1b6`, no `CARGO_TARGET_DIR` needed (nothing built),
removed at the end of this review. No sweep workload was run, lever A was not
re-run. `tools/governing_citation_check.sh` was run (text-only) and exits 0 — 9
governing documents, 0 unreproduced path/identifier citations.

---

## VERDICT: **FAIL** — 0 BLOCKING, 1 MAJOR, 0 minor

Both of round 2's named remedies are cleanly CLOSED, independently re-derived
below: the F-6 arithmetic now checks out under both routes it offers, and the
sole surviving "wave" mention outside the fixed shell comment is T-F's
deliberate N = 8 counterfactual, which I agree is honest. The three changed
lines introduce nothing wrong on their own face. **The FAIL is from my own
closing sweep**: `wp21_throughput_prereg.md:15` still cites `` `wp21_prereg.md`
revision 9 `` as the document governing §1, §3, §4 and §6.1, but revision 10's
own title bump (this amendment) means the tree's `wp21_prereg.md` is now
revision 10, and revision 10 is precisely the revision that changed the §3
content this citation names. This is the exact defect class round 1's F-5 and
round 6 of the throughput document's own review (M1) already found and fixed
once each ("sibling cited at revision N; tree held N+1") — both times by
bumping the cross-reference in step with the title. Revision 10's amendment
touched only `wp21_prereg.md` and did not re-bump the sibling, so the same
defect has recurred. It is MAJOR rather than BLOCKING under this round's rubric:
it is a claim the tree contradicts (the cited document's own header no longer
reads "revision 9"), not a wrong number a decision reads or a self-contradiction
about the schedule or which tranches run cached — the substance the citation
points at (the 6.17 h wall, the criteria, §6.1's uncached-every-tranche
conclusion) is unchanged between the two revisions.

---

## THE TWO DISPOSITIONS

### 1. Round 2's MAJOR (F-6 PARTIAL) — the lower-bound paragraph's arithmetic

`wp21_prereg.md:172–182` now reads, after the amendment:

> Play, replay and the cold check keep their uncontended rates … and are
> therefore lower bounds, worth `1 442 + 1 409 + 58 = 2 909 s = 0.81 h` of the
> 6.17 h, which is also 6.17 less the capture term's 5.36

Re-derived independently from §3's wall block (`sed -n '140,152p'
docs/experiments/wp21_prereg.md`), which gives the four line items and the two
totals in seconds before any hour-rounding:

```
play      436 x 0.827115 x 4  = 1 442 s
replay    436 x 0.807692 x 4  = 1 409 s
cold      (34 + 30) x 0.904313 =   58 s
capture   12 443 x 1.551364    = 19 304 s
tranche total                  = 22 213 s
```

**Route A (direct sum)**: `1 442 + 1 409 + 58 = 2 909` s — matches the document's
addition exactly. `2 909 / 3 600 = 0.808056` h, rounds to `0.81` h — matches.

**Route B (subtraction)**: `22 213 − 19 304 = 2 909` s exactly, the same number —
the wall block's own four line items sum to the tranche total with no residue,
so the two routes are not independently-rounded coincidences agreeing by luck at
the seconds level. In the hour-rounded figures the document actually prints,
`6.17 − 5.36 = 0.81`, which also matches. **Both routes reproduce; the new text
is correct.**

**The capture-share claim, re-checked**: "Capture is 87 % of the tranche above."
`19 304 / 22 213 = 0.869041…` → 86.9 %, which rounds to 87 % at the nearest
integer. **Stands**, unchanged by the remedy (this sentence was not part of the
diff, but the task asked it be re-checked against the amendment's neighbourhood
and it holds).

Command that decides it: `sed -n '140,182p' docs/experiments/wp21_prereg.md`,
arithmetic re-run by hand above (no script exists for this — the same
re-derivation round 2's report used).

**Disposition: CLOSED.**

### 2. Round 2's minor — the wave-ordinal sweep

`docs/experiments/wp21_prereg.md:445` now reads `# T-F, BEFORE the wave, the box
otherwise idle`, replacing `BEFORE wave one`. Full sweep for any ordinal or
plural-wave language surviving elsewhere:

```
/usr/bin/grep -n -iE 'wave [0-9]|1st wave|2nd wave|first wave|second wave|wave one|wave two' \
  docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md | LC_ALL=C sort
```

Six hits, all checked:

- `wp21_prereg.md:155,157,365` and `wp21_throughput_prereg.md:393,399` all
  affirmatively state this sweep has **no** second wave ("one wave has no second
  wave to serve", "a wall that no longer has a second wave", "no second wave for
  it to shorten") or describe, in the past tense, a pricing block that assumed
  one and is now deleted ("went with the second wave"). None asserts this sweep
  runs more than one wave.
- `wp21_prereg.md:198` (T-F's row) is the one place "second wave" appears as
  live content rather than a negation: *"At the incumbent N = 8 the second half
  would have stopped the sweep between the waves and saved half of it; that
  saving is gone with the second wave."* This is explicitly scoped to "the
  incumbent N = 8" as a counterfactual — it states what a different, unselected
  N would have cost this sweep, not a claim about the N = 16 sweep's own
  schedule. **I agree with round 2's reading that this is honest**: it is the
  record F-2 originally asked for (what the selected N gives up), and reading
  it as a claim that the actual sweep has two waves requires ignoring its own
  "at the incumbent N = 8" clause.

No ordinal ("wave one", "wave two", "wave 1/2") survives anywhere in either
document.

**Disposition: CLOSED.**

### 3. Did revision 10 introduce anything, read in context?

The three changed lines (`git diff 8a73bec 4e3d1b6 -- docs/experiments/wp21_prereg.md`):
title `revision 9` → `revision 10`; the lower-bound paragraph (checked above);
the `T-F` shell comment (checked above). Read against their surrounding
paragraphs (`sed -n '134–184p'` and `sed -n '438–452p'`), neither introduces a
new number, a new claim about cached tranches, or a new contradiction with §3's
wall block, §6.1, or the T-F row. `docs/experiments/arc3_ledger.md`'s own new
entry for this remedy states the same 2 909 s / 0.81 h derivation and the same
86.9 %-stands-as-87 % conclusion I reached independently before reading it.

**Nothing new from the three lines themselves.**

---

## CLOSING SWEEP (my own choosing)

Checked, across both documents, for: (a) any surviving claim that this sweep
has more than one wave — covered above, none found; (b) any surviving claim
that a tranche of this sweep runs cached; (c) any citation of a sibling
document's revision the tree does not hold.

**(b) — cached tranches.** `/usr/bin/grep -n -i cached docs/experiments/wp21_prereg.md
docs/experiments/wp21_throughput_prereg.md` (34 hits total). Every hit is one of:
tranche one's own re-capture verification, explicitly taken AFTER the wave and
on no tranche's critical path as a capability check (§6.1, §4.1's row);
§4.4's play-pass and tranche-sized comparison pairs in the sibling, same
after-the-wave framing; or an explicit negation ("no tranche runs cached",
"EVERY TRANCHE OF THIS SWEEP THEREFORE RUNS UNCACHED", "at the N = 16 lever A
selected there is one wave, no tranche of the sweep runs cached"). **No surviving
claim that a tranche of THIS sweep runs cached.**

**(c) — stale sibling-revision citations. This is where the MAJOR finding is.**
`/usr/bin/grep -n "wp21_prereg.md\` revision\|wp21_throughput_prereg.md\` revision\|wp21_label_cache_design.md\` revision\|matrix_label_cache_key.md\` revision" docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md`:

```
docs/experiments/wp21_prereg.md:57:            wp21_label_cache_design.md` revision 10.
docs/experiments/wp21_throughput_prereg.md:15: wp21_prereg.md` revision 9
docs/experiments/wp21_throughput_prereg.md:17: wp21_label_cache_design.md` revision 10
docs/experiments/wp21_throughput_prereg.md:19: matrix_label_cache_key.md` revision 3
```

Cross-checked against each cited document's own title line:

```
$ sed -n '1p' docs/experiments/wp21_prereg.md
# WP-2.1 — the production label sweep. RUN REGISTRATION, revision 10.
$ sed -n '1p' docs/experiments/wp21_label_cache_design.md
# WP-2.1 lever B — the label cache. DESIGN, revision 10.
$ sed -n '1p' docs/experiments/matrix_label_cache_key.md
# OPTION MATRIX — the label cache's key. Revision 3.
```

`wp21_label_cache_design.md` revision 10 and `matrix_label_cache_key.md`
revision 3 both match. **`wp21_prereg.md`'s own citation (line 57, within
itself) of the label-cache design at "revision 10" matches. But
`wp21_throughput_prereg.md:15`'s citation of `wp21_prereg.md` at "revision 9"
does not — the tree holds revision 10**, precisely because this amendment
bumped the title without touching the sibling. `wp21_label_cache_design.md`
also carries an old-looking citation ("`wp21_prereg.md` revision 4 §4 and §5",
its own line 12) but that document explicitly declares itself pinned — "`
file:line` citations are at `adb2012`, the tree before IMPL" — and its design
gate is stated closed at revision 6, so its citations are a declared historical
anchor rather than a live cross-reference; it is not the same defect class.
`docs/book_v2_ledger.md`'s row naming `wp21_prereg.md` revision 7 is likewise
not in scope: it is not on `tools/governing_citation_check.sh`'s `GOVERNING`
list, and per that script's own stated philosophy ledgers are records whose
citations were true when written and are not required to track the tree.
`wp21_throughput_prereg.md` IS on the `GOVERNING` list, carries no such pin
disclaimer, and this exact citation has twice before been a live one that
tracks the tree (round 1's F-5, fixed revision 7→8; the throughput document's
own round 6 review M1, fixed revision 5→6) — so its going stale here is a
recurrence of a known, previously-fixed defect class, not a record rotting
honestly.

`tools/governing_citation_check.sh` was run and exits 0 (9 governing documents,
0 unreproduced citations) — **it does not catch this class**: it verifies
`path` and `path:line` citations and backticked identifiers exist, never text
of the form "revision N", so a green run here is compatible with the stale
citation above. This is stated in the tool's own docstring ("it cannot tell
whether a true quotation supports the claim built on it").

---

## WHAT THIS REVIEW DID NOT LOOK AT

- No sweep workload, no lever A re-run, no engine binary built — nothing in
  this amendment's three lines or the closing sweep required one.
- Round 1 and round 2's own re-derivations (N = 16 selection, C1–C4, F-1
  through F-5, the wall block's four-line-item arithmetic itself) were not
  re-run from scratch; only the parts round 2's F-6 remedy touched were
  re-checked here.
- The closing sweep covered "more than one wave", "runs cached" and "stale
  sibling revision" as directed, plus my own read of the cache-design
  document's citations to confirm they are a declared pin rather than the same
  defect. It did not re-audit every other cross-reference in either document
  (D-numbers, `docs/decisions.md` lines, `tools/*.py` paths) beyond what
  `tools/governing_citation_check.sh` already covers.
- I did not attempt to fix the MAJOR finding or draft the one-line remedy; per
  this review's remit, that is the next amendment's job.
