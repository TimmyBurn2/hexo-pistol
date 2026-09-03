# REVIEW of the WP-2.1 N=16 amendment (round 5, remedies-only, LAST GRANTED ROUND)

**REVISION REVIEWED**: `d61e161` (`d61e16164b1d68f8e9f70a51b9c7ff1d9bc2452b`) on
`dev`, HEAD at dispatch. `git rev-parse HEAD` at the end of this review, in this
review's own worktree: **`d61e16164b1d68f8e9f70a51b9c7ff1d9bc2452b`** — still
equals `d61e161`, unchanged throughout.

**REVIEWED ON: Sonnet, per D-597.**

Worktree used: `git -C /home/tom/Projects/HeXO-AlphaBeta worktree add --detach
/home/tom/pistol-wt/amend5 d61e161`, removed at the end of this review (see
closing note). Scratch, if any, stayed under the worktree, never `/tmp`. Nothing
was built; no sweep workload was run; lever A was not re-run.
`tools/governing_citation_check.sh` and `tools/decision_key_check.sh` were both
run and both exit 0.

**SCOPE, and one correction to the dispatch's own count.** The dispatch says
`git diff 0be90de d61e161` "touches two files". It actually touches **three**:
`docs/decisions.md` (+2 lines, D-600), `docs/experiments/arc3_ledger.md` (+21
lines), and `docs/experiments/wp21_prereg_rev10_amendment_REVIEW_round4.md`
(new, 294 lines — round 4's own report, landed in the same commit as the D-600
fix it recommends, exactly the pattern round 3's own report followed one commit
earlier). This is bookkeeping, not a defect: the review report is evidence/
context, not a fourth governing artifact, and — the substantive claim the
dispatch cared about — **no governing document changed**: `git diff 0be90de
d61e161 -- docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md`
is empty, confirmed by my own command.

---

## VERDICT: **FAIL** — 0 BLOCKING, **1 MAJOR**, 2 minor

D-600's re-grounding of the design's exemption (disposition 2) is sound, and its
append-only hygiene (disposition 3) is clean. But **disposition 1 — "is four the
right count" — is itself wrong, the same way D-599 was wrong**: my own search of
the review reports (not D-600's list) finds a **fifth core-class occurrence**
D-600 does not count, sitting inside the very files round 4's search excluded by
construction. This is not a new number a live decision reads to govern the run —
it changes no schedule, no cached-tranche claim, no criterion — so per this
round's own calibration it is **MAJOR, not BLOCKING** (the same rating round 4
gave its own structurally identical finding). But it is squarely a claim the
tree contradicts, so it is not a PASS either: **the remedy is not CLOSED with
nothing new.**

---

## 1. Is D-600's derived history right? — NO, undercounted

I checked each of the four claimed occurrences against the report it names, by
my own commands, before searching for others.

**(1) `wp21_throughput_prereg_rev4_REVIEW.md` MAJOR 1** — design at 7, tree at 10.
```
$ grep -n "^## MAJOR 1" docs/experiments/wp21_throughput_prereg_rev4_REVIEW.md
228:## MAJOR 1 — **The design is cited at revision 7; the tree holds revision 10, and it did before this revision was committed.**
```
Match.

**(2) `wp21_throughput_prereg_rev5_REVIEW.md` MAJOR 1** — sweep (`wp21_prereg.md`)
at 5, tree at 6.
```
$ grep -n "^## MAJOR 1" docs/experiments/wp21_throughput_prereg_rev5_REVIEW.md
214:## MAJOR 1 — **The sibling is cited at revision 5; the tree holds revision 6, landed in the same commit as this revision.**
```
Match.

**(3) `wp21_prereg_rev8_amendment_REVIEW.md` F-5** — sweep at 7, tree at 8, at
`wp21_throughput_prereg.md:13`.
```
$ sed -n '287,296p' docs/experiments/wp21_prereg_rev8_amendment_REVIEW.md
**FINDING (MAJOR), F-5**: `wp21_throughput_prereg.md` line 13 (GOVERNING
section, untouched by the amendment) still reads:
> `D-587, D-588 (the key and what is claimed about it); `wp21_prereg.md` revision 7`
`wp21_prereg.md` is now revision 8. ...
```
Match.

**(4) `wp21_prereg_rev10_amendment_REVIEW.md`** — sweep at 9, tree at 10.
```
$ grep -n "cites `wp21_prereg.md`\|revision 9.*revision 10\|MAJOR" docs/experiments/wp21_prereg_rev10_amendment_REVIEW.md | sed -n '1,6p'
27:## VERDICT: **FAIL** — 0 BLOCKING, 1 MAJOR, 0 minor
187:`wp21_throughput_prereg.md:15`'s citation of `wp21_prereg.md` at "revision 9"
188:does not — the tree holds revision 10**, precisely because this amendment
```
Match.

**(5, one step outside) `wp21_prereg_rev6_REVIEW.md` M5** — book ledger row.
```
$ sed -n '230,235p' docs/experiments/wp21_prereg_rev6_REVIEW.md
### M5 — `docs/book_v2_ledger.md`'s row names revision 5; the document says it names this one.
Preamble :42: ... `/usr/bin/grep -n wp21_prereg docs/book_v2_ledger.md` → `:42 | 13 | 3487 | 13..3499 | … | docs/experiments/wp21_prereg.md revision 5 |`.
```
Match — and `docs/book_v2_ledger.md` is confirmed NOT on gate 20's GOVERNING
list (below), so D-600's "sits one step outside" is correctly grounded.

**Did my own search find more or fewer? MORE — at least a sixth occurrence,
inside the core population D-600 itself defines, that D-600 does not count.**

`wp21_label_cache_design_REVIEW.md` — the *first* review of the label-cache
design, revision 1 — carries:

```
$ grep -n "^## BLOCKING 6" -A3 docs/experiments/wp21_label_cache_design_REVIEW.md
## BLOCKING 6 — THE DESIGN IS GOVERNED BY A SUPERSEDED REVISION, AND CITES SECTION NUMBERS THAT REVISION DID NOT HAVE

Design lines 11-13:
> **GOVERNING REGISTRATION**: `docs/experiments/wp21_throughput_prereg.md` **revision 2**, §2 …
The tree holds **revision 3** …
```
This is a `wp21_label_cache_design.md`-cites-`wp21_throughput_prereg.md`-by-
revision citation (the mirror direction of D-600's own occurrence (1), which is
`wp21_throughput_prereg.md`-cites-the-design), 2-against-3, caught by a fresh
reviewer, and — confirmed against the checker itself, same as D-600's own
mechanical claim:
```
$ sed -n '46,50p' tools/design_citation_check.py
PATH = re.compile(
    r"`(?P<path>(?:crates|tools|configs|docs)/[A-Za-z0-9_./-]+\.(?:rs|sh|py|toml|md|txt))"
    r"(?::(?P<line>\d+)(?:-(?P<end>\d+))?)?`"
)
```
— it matches path existence only, never "revision N" text, so this instance was
**exactly as invisible to gate 20 as the four D-600 counts.** It was found by a
fresh reviewer, and it was FIXED (confirmed CLOSED, both directly in
`wp21_label_cache_design_rev2_REVIEW.md:65` and `wp21_label_cache_design_rev3_REVIEW.md:109`,
both rows: `B6 | ... | CLOSED | header names revision 3, §2.0, §2.1`).

**Why D-600 missed it, mechanically, not by bad luck**: round 4's own report —
the source D-600 builds on — states its search scope explicitly:
`wp21_prereg_rev10_amendment_REVIEW_round4.md:158-159`, *"I searched every
`wp21_prereg_rev*_REVIEW.md` and `wp21_throughput_prereg_rev*_REVIEW.md` file …
and found none."* That glob never reaches `wp21_label_cache_design*_REVIEW.md`,
even though `wp21_label_cache_design.md` is itself one of gate 20's nine
GOVERNING documents (confirmed: `bash tools/governing_citation_check.sh` lists
it as the 7th of nine) and D-600's own occurrence (1) already concedes the
design is a legitimate party to this citation class in one direction. D-600
states its class symmetrically — *"A REVISION NUMBER CITED ACROSS TWO GOVERNING
DOCUMENTS"* — but inherited round 4's asymmetric search, which only ever asked
"does `wp21_throughput_prereg.md` or `wp21_prereg.md` cite someone stale", never
"does the design cite `wp21_throughput_prereg.md` stale". I searched the design's
own five review files (`wp21_label_cache_design_REVIEW.md`, `rev2`, `rev3`,
`rev5`, `rev6`) and `wp21_label_cache_REDTEAM.md`/`_MUTATION.md` for this shape;
BLOCKING 6 above is the only hit — no seventh.

I also checked the five `wp21_prereg_rev{3,4,5,6,7}_REVIEW.md` files against
`wp21_label_cache_design.md`'s own revision citations for a comparable miss and
found none (all rows read ✓, e.g. `wp21_prereg_rev7_REVIEW.md:78`, `wp21_prereg_rev6_REVIEW.md:145`).

**How this is rated.** Per this round's calibration — *"an imprecision that
moves no conclusion is a minor, not a MAJOR"* — I weighed whether this changes
a conclusion. It does not touch the schedule, cached-tranche status, or the
"still owed, still built while the sweep runs, manual rule stands" disposition.
But it directly falsifies D-600's central, load-bearing, specifically-derived
claim — *"Derived from the reports rather than from memory, there are FOUR
occurrences"* — which is the one thing D-600 exists to get right, having been
written to fix exactly this failure mode in D-599. The true count of the core
(governing-document-to-governing-document) class is **at least five**, not
four, before even reaching the ledger example. This is a claim the tree
contradicts, matching round 4's own rubric for its structurally identical
finding (round 4 rated its "wrong first attribution" finding MAJOR, not
BLOCKING, for the same reason: no run-governing number moves). **Rated MAJOR.**

---

## 2. Is D-600's re-grounding of the design's exemption sound? — YES

The design's own text, current tree:
```
$ sed -n '12,20p' docs/experiments/wp21_label_cache_design.md
**GOVERNING**: `docs/experiments/wp21_throughput_prereg.md` revision 3 §2, §2.0,
§4.2, §4.4, §4.5; `docs/experiments/wp21_prereg.md` revision 4 §4 and §5; D-576, …
revision history is in `docs/experiments/arc3_ledger.md`, not here. **`file:line`
citations are at `adb2012`**, the tree before IMPL. **The design gate closed at
revision 6 by D-589**; revision 7 carries round 5's one MAJOR and two minors as the
IMPL obligations D-589 names, and is not reviewed as a design …
```
The pin (`adb2012`) is textually scoped to *"`file:line` citations"* only —
it says nothing about the `revision N` document citations one line above it.
D-599's claim that this pin covers the whole document's citations, including
the stale `revision 3`/`revision 4` GOVERNING numbers, over-reaches exactly as
D-600 says.

D-589, checked directly:
```
$ grep -n "^D-589:" docs/decisions.md
1246:D-589: **THE LABEL CACHE'S DESIGN GATE IS CLOSED BY OPERATOR RULING AT REVISION 6 …**
```
Confirms: gate closed at design-revision 6 by operator ruling; revisions 7-10
are IMPL obligations, not reviewed as a design. This matches the design's own
text above word for word (*"revision 7 carries … IMPL obligations … revision 8
corrects … revision 9 names … revision 10 replaces …"*, lines 16-20). D-600's
re-grounding is sound and precisely stated. (This same correction was already
flagged, independently, as round 4's own **Minor B** — D-600 folds it in
without distortion.)

---

## 3. Did D-600 introduce anything? — NO, clean append

```
$ git diff 0be90de d61e161 -- docs/decisions.md | grep '^-' | grep -v '^---'
(no output)
```
No line of D-599 (or anything else in the log) was removed or altered — pure
append. `tools/decision_key_check.sh`:
```
$ bash tools/decision_key_check.sh
decision_key_check: 602 decision keys in docs/decisions.md, no repeat outside the exemption
```
Exit 0. `grep -c "^D-599:"` and `grep -c "^D-600:"` are each `1`. D-600's own
text explicitly reaffirms D-599's conclusion untouched (*"the check is still
owed, still built while the sweep runs, and the manual rule still stands"*) —
consistent with what I independently verified in §1-2 above; nothing in D-600
contradicts D-599's surviving conclusion, it only corrects the count (which I
find is itself still short) and the exemption's ground.

---

## FINAL SAFETY SWEEP — `wp21_prereg.md` and `wp21_throughput_prereg.md`

Both files are **byte-identical** between `0be90de` and `d61e161` (confirmed by
the three-file diff stat above), so this is a genuine independent pass over
unchanged text, not a re-review of a diff.

**Relying on, not re-deriving**: the N = 16 selection, C1 over 93 files, C2, §3's
wall arithmetic, and the lower-bound sum — settled by rounds 1-3
(`wp21_prereg_rev8_amendment_REVIEW.md`, `_rev9_amendment_REVIEW.md`,
`_rev10_amendment_REVIEW.md`) and re-confirmed unchanged by round 4; the median
rep and the 53-hit cached-grep census and six-hit wave census — taken by round 4
(`wp21_prereg_rev10_amendment_REVIEW_round4.md`, CLOSING SWEEP).

- **Criterion that cannot fail**: relying on round 4's sweep over T-A1, T-A2,
  T-B, T-C, T-D, T-F and C1-C4 — each names a concrete exit code, count, or
  byte comparison; no finding there, not re-walked line by line here.
- **Registered command that does not run**: relying on round 4's path-existence
  sweep (`tools/wp21_tranche_config.py`, `cold_label_check.py`,
  `determinism.sh`, `label_cache_count.py`, `opening_prefix_fold.py`,
  `wp21_assemble.py`, `arena.rs`, `usage.rs` — all present, overlapping
  `design_citation_check.py`'s own 26+16 citation check, which I re-ran myself
  (below) and confirms 0 unreproduced).
- **Self-contradiction about schedule / cached tranches**: spot-checked myself,
  fresh command: `§6.1`'s own header, *"THE CACHE, AND WHY NO TRANCHE OF THIS
  SWEEP RUNS CACHED"* and *"EVERY TRANCHE OF THIS SWEEP THEREFORE RUNS
  UNCACHED"* — consistent, no contradiction found. Full 53-hit and 6-hit
  censuses are round 4's, relied on rather than repeated.
- **Stale cross-revision citation**: this is task 1 above, done exhaustively
  (and the one place this round found something rounds 1-4 did not).

```
$ bash tools/governing_citation_check.sh
governing_citation_check: 9 governing document(s), 0 proposed path(s)
... (all nine) 0 unreproduced
DESIGN_CITATION_CHECK_DONE
$ echo $?
0

$ bash tools/decision_key_check.sh
decision_key_check: 602 decision keys in docs/decisions.md, no repeat outside the exemption
$ echo $?
0
```

---

## minor

**minor 1** — D-600 presents the book-ledger citation defect (item 5, "one step
outside") as a single instance (`wp21_prereg_rev6_REVIEW.md` M5). My own trace
of `docs/book_v2_ledger.md`'s row shows it went stale at least **three** times
across the arc, not once: `wp21_prereg_rev3_REVIEW.md` m6 (names revision 2,
tree at 3), still open at `wp21_prereg_rev4_REVIEW.md` m6/n-m2 ("now two
revisions stale"), closed by rev5, recurred at `wp21_prereg_rev6_REVIEW.md` M5
(the one D-600 cites), closed again by rev7. Does not change D-600's conclusion
— the ledger is still correctly ruled outside gate 20's list, and one example
is enough to make that case — so per this round's own calibration this is a
minor, not a MAJOR.

**minor 2** — the dispatch's "touches two files" underclaims the diff by one
(the round 4 report itself); addressed under SCOPE above. Not a defect in the
tree.

---

## WHAT THIS REVIEW DID NOT LOOK AT AND WHICH ROUND DID

- N = 16's selection, C1's 93-of-93 byte-identical capture files, C2's 93-of-93
  at 152 asked prefixes, §3's wall-block arithmetic, and the lower-bound sum —
  settled by rounds 1-3 (`wp21_prereg_rev8_amendment_REVIEW.md`,
  `_rev9_amendment_REVIEW.md`, `_rev10_amendment_REVIEW.md`), not re-derived
  here.
- The 53-hit cached-tranche grep census and the 6-hit wave census, both taken
  and categorized by round 4 (`wp21_prereg_rev10_amendment_REVIEW_round4.md`,
  CLOSING SWEEP) — relied on, not re-walked hit by hit.
- Whether `wp21_label_cache_design.md`'s own frozen GOVERNING citations
  (`wp21_throughput_prereg.md` revision 3, `wp21_prereg.md` revision 4) still
  correctly describe those documents' content as it stood when the design gate
  closed — round 4 confirmed the structural reason this is not a *live*
  citation but did not re-diff the historical content, and neither did I.
- No sweep workload, no lever A re-run, no engine binary built, no mutation
  testing — nothing in this diff or this round's remit required one.
- I did not attempt the fix for the undercount above (this round's remit is to
  find, per D-600/round-4's own convention); the minimal correction is a sixth
  entry in D-600's enumeration (the design-cites-throughput-prereg,
  2-against-3, `wp21_label_cache_design_REVIEW.md` BLOCKING 6, fixed by
  `wp21_label_cache_design_rev2_REVIEW.md`'s B6) and "at least five" in place of
  "four" in its prose.

---

**Worktree removed after this report was written**: `git -C
/home/tom/Projects/HeXO-AlphaBeta worktree remove /home/tom/pistol-wt/amend5`.
