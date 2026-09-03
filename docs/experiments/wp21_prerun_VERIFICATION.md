# WP-2.1 pre-run VERIFICATION (D-598's scoped-pass disposition of round 5's FAIL)

**REVISION VERIFIED**: `6ddd959` (`6ddd95946fb5a213df2e06c86b555a504d5276a3`) on
`dev`, HEAD at dispatch. `git rev-parse HEAD` at the end of this pass, in the live
tree: `6ddd95946fb5a213df2e06c86b555a504d5276a3` — **still equals `6ddd959`**,
unchanged throughout. `git status --short` in the live tree is clean at both ends.

**VERIFIED ON: Sonnet, per D-597/D-598.**

This is a scoped verification pass, not a review round (D-598's shape): it does not
re-derive N = 16, C1–C4, the wall arithmetic, or anything rounds 1–4 already settled.
It checks the five things the dispatch named, each with the command that produced
the result, and nothing else is asserted.

Worktree used: `git -C /home/tom/Projects/HeXO-AlphaBeta worktree add --detach
/home/tom/pistol-wt/predawn 6ddd959`, removed at the end of this pass. Nothing was
built; the sweep workload was not run; lever A was not re-run. This report is the
only file written in the live tree.

---

## VERDICT: **PASS**, with one non-blocking finding under item 3

Items 1, 2, 4 and 5 hold exactly as the dispatch states, reproduced independently
below. Item 3's mechanism (a search replacing a transcribed count) is sound and its
invariance argument is correct, but running the dispatch's own search against the
tree **as it actually stood at the revision D-601 names** returns 24, not 25 — the
25th match, `wp21_amendment_REVIEW_round5.md`, was first committed in `6ddd959`
itself, the very commit that adds D-601. D-601's stated count (25) is correct only
for `6ddd959`, not for `d61e161` as it says. This is a **sixth instance of the exact
defect class D-599/D-600/D-601 exist to fix** — a citation naming a revision the
cited fact does not hold at — sitting self-referentially inside D-601's own sentence
about re-runnability. It changes no governing document, no criterion, no schedule,
no cached-tranche claim, and D-599's surviving conclusion reads identically whether
the count is 24 or 25 — so by this same ADR chain's own established precedent
(D-600 correcting D-599's history without blocking anything), it is recorded here
as a finding for a follow-up ADR line, not as a reason to withhold the PASS or to
touch `docs/decisions.md` myself (out of this pass's scope; see the dispatch's
"that file is the only thing you write in the live tree").

---

## 1. THE LOAD-BEARING CLAIM: no governing document changed while the ADR history was being corrected

Command:
```
git diff 4e3d1b6 6ddd959 --stat -- docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md tools crates configs
```
Output:
```
 docs/experiments/wp21_throughput_prereg.md | 4 ++--
 1 file changed, 2 insertions(+), 2 deletions(-)
```
Full content of that diff:
```diff
-# WP-2.1 sweep throughput — a scaling study. PRE-REGISTRATION, revision 9.
+# WP-2.1 sweep throughput — a scaling study. PRE-REGISTRATION, revision 10.
...
-D-587, D-588 (the key and what is claimed about it); `wp21_prereg.md` revision 9
+D-587, D-588 (the key and what is claimed about it); `wp21_prereg.md` revision 10
```
Confirmed: the ONLY change in the scoped paths is the throughput document's title
line and its one cross-citation of the sweep, revision 9 to 10 — round 3's own
named remedy (landed at `0be90de`, one commit after `4e3d1b6`). Nothing in
`wp21_prereg.md`, `tools`, `crates`, or `configs` moved.

For context, the repo-wide diff over the same range (`git diff 4e3d1b6 6ddd959
--stat`, unscoped) touches only `docs/decisions.md` (+6 lines: D-599, D-600,
D-601), `docs/experiments/arc3_ledger.md` (+72), and three REVIEW report files
(`wp21_amendment_REVIEW_round5.md`, `wp21_prereg_rev10_amendment_REVIEW.md`,
`wp21_prereg_rev10_amendment_REVIEW_round4.md`) — ADR/history/evidence artifacts,
not governing documents. **HOLDS.**

## 2. The two registrations at HEAD are the ones the safety sweeps saw

Round 3 reviewed `4e3d1b6`, round 4 reviewed `0be90de`, round 5 reviewed `d61e161`
(taken from each report's own header). Ancestry check confirms all three are
ancestors of `6ddd959`. Diffs of the two registration files across every interval:

```
git diff 0be90de d61e161 -- docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md   -> empty
git diff d61e161 6ddd959 -- docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md   -> empty
```

sha256 of both files at each named revision and at HEAD:

| revision | `wp21_prereg.md` | `wp21_throughput_prereg.md` |
|---|---|---|
| `4e3d1b6` (round 3) | `f45786a1...15f69` | `498314bb...4e4887` |
| `0be90de` (round 4) | `f45786a1...15f69` | `dfe83bbe...3ef885` |
| `d61e161` (round 5) | `f45786a1...15f69` | `dfe83bbe...3ef885` |
| `6ddd959` (HEAD)    | `f45786a1...15f69` | `dfe83bbe...3ef885` |

`wp21_prereg.md` has been byte-identical since round 3 (`4e3d1b6`); it is the exact
text rounds 3, 4 and 5 each swept. `wp21_throughput_prereg.md` changed once, at
`0be90de` — round 4's own remedy, which round 4 itself reviewed — and has been
byte-identical since, matching what round 5 saw and what is at HEAD now. No
unreviewed edit slipped in. **HOLDS.**

## 3. D-601's withdrawal is sound and its replacement re-runs

Command (run at HEAD, `6ddd959`):
```
/usr/bin/grep -lE "cite[sd]? .*revision [0-9]+|revision [0-9]+; the tree|names revision [0-9]+" docs/experiments/*REVIEW*.md | LC_ALL=C sort | wc -l
```
Output: `25` — matching the number D-601 states.

**But the revision D-601 attributes that count to does not hold it.** D-601 says the
search "returns 25 review reports at `d61e161`". Re-running the identical pattern
against the tree exactly as committed at `d61e161` (`git ls-tree -r --name-only
d61e161 -- docs/experiments/` filtered to the same glob shape, then `git show
d61e161:<path> | grep -qE <pattern>` per file, since `d61e161` is not the current
worktree's checkout) returns **24**, not 25:

```
docs/experiments/derivation_gate_design_REVIEW.md
docs/experiments/wp15b_U1_REVIEW.md
docs/experiments/wp15b_U3_REVIEW_urev4.md
docs/experiments/wp15d_b_prereg_REVIEW_rev3.md
docs/experiments/wp20_pilot_prereg_REVIEW_rev3.md
docs/experiments/wp20_pilot_prereg_REVIEW_rev4.md
docs/experiments/wp20b_design_rev5_REVIEW.md
docs/experiments/wp20b_design_rev6_REVIEW.md
docs/experiments/wp20b_design_rev7_REVIEW.md
docs/experiments/wp20s_design_REVIEW_rev2.md
docs/experiments/wp20s_design_REVIEW_rev3.md
docs/experiments/wp20s_design_REVIEW_rev4.md
docs/experiments/wp20s_design_REVIEW_rev5.md
docs/experiments/wp21_label_cache_design_REVIEW.md
docs/experiments/wp21_label_cache_design_rev2_REVIEW.md
docs/experiments/wp21_label_cache_design_rev3_REVIEW.md
docs/experiments/wp21_prereg_rev10_amendment_REVIEW_round4.md
docs/experiments/wp21_prereg_rev6_REVIEW.md
docs/experiments/wp21_prereg_rev7_REVIEW.md
docs/experiments/wp21_prereg_rev9_amendment_REVIEW.md
docs/experiments/wp21_throughput_prereg_rev2_REVIEW.md
docs/experiments/wp21_throughput_prereg_rev4_REVIEW.md
docs/experiments/wp21_throughput_prereg_rev5_REVIEW.md
docs/experiments/wp21_throughput_prereg_rev6_REVIEW.md
```

The 25th file the HEAD-run finds, `docs/experiments/wp21_amendment_REVIEW_round5.md`
(round 5's own FAIL report), does not exist at `d61e161`:
```
$ git cat-file -e d61e161:docs/experiments/wp21_amendment_REVIEW_round5.md
fatal: path '...' exists on disk, but not in 'd61e161'
$ git log --diff-filter=A --oneline -- docs/experiments/wp21_amendment_REVIEW_round5.md
6ddd959 docs(arc3): D-601 withdraws the defect-class count ...
```
It was first committed in `6ddd959` — the same commit that writes D-601. So 25 is
the true count **at `6ddd959`**, not at `d61e161`; D-601 names the wrong revision for
its own number, by exactly one commit, in the same style as the three (now
arguably six) occurrences it is busy cataloguing.

**Is this blocking?** No, by this ADR chain's own repeatedly-applied test: it is a
citation error inside history/self-description prose, not a change to any governing
document, criterion, schedule, or cached-tranche claim, and `governing_citation_check.sh`
already states plainly that it "cannot tell whether a true quotation supports the
claim built on it" — a `revision N` claim like this one is invisible to it by
construction (D-599's own diagnosis). D-599's surviving conclusion — the class is
real, gate 20 structurally cannot see it, the check is owed and built while the
sweep runs, and until then a revision bump carries its sibling's citation in the
same commit — reads identically whether the search's true count is 24 or 25 or 26.
**D-601's invariance argument is correct.** The mechanism it installs (print the
command, scope and output at a named revision, assert no closed integer) is the
right fix for the class; this finding shows the mechanism was misapplied once on
its own first use, not that the mechanism is wrong. Recommend a follow-up ADR line
(next in sequence, D-602) correcting "at `d61e161`" to "at `6ddd959`" — not made
here, as this pass writes only this report.

**Verdict for item 3: HOLDS with a flagged, non-blocking finding.**

## 4. Append-only hygiene

```
git diff 4e3d1b6 6ddd959 -- docs/decisions.md | grep -E '^-' | grep -v '^---'
```
Output: empty — additions only (`+9` lines net: D-599, D-600, D-601, each a single
paragraph).

```
grep -c "^D-599:" docs/decisions.md   -> 1
grep -c "^D-600:" docs/decisions.md   -> 1
grep -c "^D-601:" docs/decisions.md   -> 1
```

```
$ bash tools/decision_key_check.sh
decision_key_check: self-test passed — a clean seed, a seeded repeat, and the anchor
decision_key_check: grandfathered by D-279 (ruled, not deleted): D-276 2 D-277 2
decision_key_check: 603 decision keys in docs/decisions.md, no repeat outside the exemption
EXIT CODE: 0

$ bash tools/governing_citation_check.sh
governing_citation_check: 9 governing document(s), 0 proposed path(s)
CLAUDE.md: 5 citation(s) checked, 0 unreproduced
docs/ROADMAP.md: 11 citation(s) checked, 0 unreproduced
docs/process.md: 1 citation(s) checked, 0 unreproduced
docs/experiments/anchor_v3_openings_design.md: 10 citation(s) checked, 0 unreproduced
docs/experiments/matrix_label_cache_key.md: 7 citation(s) checked, 0 unreproduced
docs/experiments/sealbot_anchor_v3_prereg.md: 7 citation(s) checked, 0 unreproduced
docs/experiments/wp21_label_cache_design.md: 19 citation(s) checked, 0 unreproduced
docs/experiments/wp21_prereg.md: 26 citation(s) checked, 0 unreproduced
docs/experiments/wp21_throughput_prereg.md: 16 citation(s) checked, 0 unreproduced
DESIGN_CITATION_CHECK_DONE
EXIT CODE: 0
```
**HOLDS.**

## 5. The launch is safe on its own terms

```
$ git -C /home/tom/Projects/HeXO-AlphaBeta status --short
(empty)
$ git -C /home/tom/Projects/HeXO-AlphaBeta rev-parse HEAD
6ddd95946fb5a213df2e06c86b555a504d5276a3
$ ps aux | grep -iE "pistol|arena|corpus-check|wp21|cargo|rustc" | grep -v grep
(empty — nothing running)
```

Binary digests (live tree, `target/release/`) against `wp21_prereg.md` §8:

| binary | on disk | registered |
|---|---|---|
| `pistol` | `78a7600adcf099de0b04149535f1f4bffe0b6c945609a3206d73a4e5ee853749` | same |
| `arena` | `a1a405cb44d21f1a70918f44b15553f0a90d23f02e9f959458545707a69614c3` | same |
| `corpus-check` | `efbb76b643fb72fc4024168a278542836a94d1470ebd831cdb707890593e3abf` | same |

All three match exactly. Bonus (not requested, checked for corroboration): installed
`rustc 1.98.0 (88d9e12ae 2026-08-18)` / `cargo 1.98.0 (797e8a9bc 2026-08-05)` match
§8's registered toolchain exactly. **HOLDS.**

---

## WHAT THIS PASS DID NOT LOOK AT

Honestly, what a sixth review round would have covered and this pass did not:

- **No fresh derivation of anything rounds 1–4 already settled**: N = 16, C1 over
  93 files, C2, §3's wall arithmetic, the lower-bound sum. This pass took those as
  given, per the dispatch's own framing.
- **No independent widening of D-601's search** beyond re-running its exact command
  at its exact stated scope. A full round might have tried alternate patterns, or
  swept `docs/` more broadly than `docs/experiments/*REVIEW*.md`, to see whether a
  seventh occurrence of the stale-revision class exists outside that glob (e.g. in
  `docs/decisions.md`'s own prose, or in non-`*REVIEW*` experiment documents).
- **`wp21_label_cache_design.md` was not content-checked against its own prior
  review revisions** the way the two registrations were in item 2 — it is cited as
  a governing document and its citation shape was exercised by
  `governing_citation_check.sh`, but this pass did not diff it against the revision
  its own last design review (`wp21_label_cache_design_rev6_REVIEW.md` / D-589)
  actually read.
- **Only the three release binaries' digests were checked against §8.** The same
  table also registers digests for `tools/cold_label_check.py`,
  `tools/wp21_tranche_config.py`, `tools/label_cache_count.py`,
  `tools/wp21_assemble.py`, and `artifacts/arc3_opening_prefix_fold.txt` (plus a
  receipt file). None of those five were re-digested here; the dispatch named only
  "the three release binaries."
- **No re-verification of D-590 through D-598** (the standing-law lines governing
  review process itself) beyond using them as background for why this pass exists
  and is shaped as it is.
- **`tools/design_citation_check.py` / gate 20's own source was not read** — only
  its behavior was exercised via `governing_citation_check.sh`'s exit code and
  printed counts, per the dispatch's own framing of what gate 20 can and cannot see.
- **The sweep workload and lever A were not run**, per explicit instruction — so
  nothing about actual runtime behavior, dry-run output, or the cache mechanism
  itself was checked here; that is exactly the "verified capability" §4.4 is
  reserved to take after the wave.
- The one finding this pass did surface (item 3, D-601's own revision mislabel) was
  found by the specific reproduction the dispatch asked for, not by an open-ended
  hunt; a full round's own "closing sweep of its own choosing" is exactly the kind
  of unscoped look this pass, by design, did not do.
