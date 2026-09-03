# REVIEW of the WP-2.1 revision-10 amendment remedy (round 4, remedies-only, narrow)

**REVISION REVIEWED**: `0be90de` on `dev` (HEAD at dispatch). `git rev-parse HEAD`
at the end of this review, both the live tree and this review's own worktree:
`0be90de9092893ab68d4f09f3cb718bbcd826416` — **still equals `0be90de`**, unchanged
throughout.

**REVIEWED ON: Sonnet, per D-597.**

Worktree used: `git -C /home/tom/Projects/HeXO-AlphaBeta worktree add --detach
/home/tom/pistol-wt/amend4 0be90de`, removed at the end of this review. Scratch
files (if any) were kept under the worktree, never `/tmp`. Nothing was built; no
sweep workload was run; lever A was not re-run. `tools/governing_citation_check.sh`
was run and exits 0 (see disposition 2).

Scope, per the dispatch: `git diff 4e3d1b6 0be90de` — the sibling's title and its
one cross-citation (`wp21_throughput_prereg.md`), `docs/decisions.md` (D-599), and
`docs/experiments/arc3_ledger.md`. (The diff also adds
`wp21_prereg_rev10_amendment_REVIEW.md` itself — round 3's own report, committed
in the same commit as the fix it recommends — which is context/evidence, not a
fourth thing under review here.) Round 1 (`wp21_prereg_rev8_amendment_REVIEW.md`),
round 2 (`wp21_prereg_rev9_amendment_REVIEW.md`) and round 3
(`wp21_prereg_rev10_amendment_REVIEW.md`) already settled N = 16, C1 over 93
files, C2, §3's wall arithmetic, and the lower-bound sum (`2 909 s = 0.81 h`,
`22 213 − 19 304 = 2 909`); none of that is re-derived here.

---

## VERDICT: **FAIL** — 0 BLOCKING, 1 MAJOR, 2 minor

The remedy itself (disposition 1) is CLOSED and correct: the sibling citation now
matches, and I found no other stale revision-N citation anywhere in either
document, in either direction, against gate 20's full nine-document GOVERNING
list. D-599's account of *why* gate 20 structurally cannot see this class
(disposition 2) is true, verified against the checker's own regex. **The MAJOR
is new, in D-599's own text**: its history of the defect's three occurrences
misattributes the first one — wrong round, wrong finding id in substance, and
wrong numbers — to a review that in fact found a different defect about a
different document. This does not change D-599's bottom line (a check is owed,
gate 20 cannot see it, the manual rule stands), but it is exactly the class of
citation error this whole review chain exists to catch, now inside the ADR that
narrates that chain.

---

## 1. THE REMEDY — does the sibling now cite the tree's own revision?

```
$ sed -n '1p' docs/experiments/wp21_prereg.md
# WP-2.1 — the production label sweep. RUN REGISTRATION, revision 10.
$ sed -n '1p' docs/experiments/wp21_throughput_prereg.md
# WP-2.1 sweep throughput — a scaling study. PRE-REGISTRATION, revision 10.
$ sed -n '15p' docs/experiments/wp21_throughput_prereg.md
D-587, D-588 (the key and what is claimed about it); `wp21_prereg.md` revision 10
```

Match. Full sweep of every `revision N` citation in both documents, both
directions, by my own commands:

```
$ /usr/bin/grep -n "revision [0-9]" docs/experiments/wp21_prereg.md
1:  ... RUN REGISTRATION, revision 10.
56: ... (no revision-N citation here — describes lever A's own numbers)
57: ... `wp21_label_cache_design.md` revision 10.
$ /usr/bin/grep -n "revision [0-9]" docs/experiments/wp21_throughput_prereg.md
1:  ... PRE-REGISTRATION, revision 10.
15: `wp21_prereg.md` revision 10
17: `wp21_label_cache_design.md` revision 10
19: `matrix_label_cache_key.md` revision 3
311: `wp21_label_cache_design.md` revision 10
```

Cross-checked against each cited document's own title line:

```
$ sed -n '1p' docs/experiments/wp21_label_cache_design.md
# WP-2.1 lever B — the label cache. DESIGN, revision 10.
$ sed -n '1p' docs/experiments/matrix_label_cache_key.md
# OPTION MATRIX — the label cache's key. Revision 3.
```

Every citation reproduces: `wp21_prereg.md` → 10 (matches), `wp21_label_cache_design.md`
→ 10 (matches, cited twice), `matrix_label_cache_key.md` → 3 (matches).
`wp21_prereg.md` does not cite `wp21_throughput_prereg.md` by revision anywhere
(checked: `/usr/bin/grep -n "wp21_throughput_prereg" docs/experiments/wp21_prereg.md`
— two hits, both bare prose naming the document, no `revision N`). Neither
document cites `CLAUDE.md`, `docs/ROADMAP.md`, `docs/process.md`,
`anchor_v3_openings_design.md` or `sealbot_anchor_v3_prereg.md` by revision (none
of those five basenames appears in either file at all). **No other stale revision
citation exists anywhere in either document.** **Disposition: CLOSED.**

**On `wp21_label_cache_design.md`'s own citations — the question I was asked to
adjudicate directly.** Its GOVERNING block (lines 12–13) cites
`` `wp21_throughput_prereg.md` revision 3 `` and `` `wp21_prereg.md` revision 4 ``,
both now nine and six revisions behind the tree. Round 3 called this "not the
same defect" because the document "explicitly declares itself pinned" at
`adb2012`. I checked the literal text (line 14): **`` `file:line` citations are
at `adb2012` ``** — and confirmed by grep that the document does carry eight
genuine `path:line` citations (`tools/ci.sh:109-110`, `exchange.rs:70-75`,
`seats.rs:47`, etc.) that sentence plainly covers. It does **not** literally say
anything about `revision N` document citations. So round 3's parenthetical
("declares itself pinned... on its face") slightly over-reaches: the disclaimer
it names is scoped to code line numbers, not to the GOVERNING block's sibling
revisions.

**I still agree with round 3's bottom-line disposition, on a narrower ground.**
`docs/decisions.md` D-589 (confirmed, quoted in full): *"THE DESIGN GATE IS
CLOSED... wp21_label_cache_design.md revision 6 PASSES FOR MECHANISM"* — and the
design's own revision history (lines 20–21 of the document) states plainly that
revisions 7–10 are IMPL-obligation corrections (a test-assertion claim, a site
name, a residual number) that "do not move the mechanism." None of those four
revisions touched the GOVERNING block. A frozen-gate document's citation of a
sibling's revision is a record of what the design was built against, not a live
dependency the way `wp21_throughput_prereg.md`'s citation of `wp21_prereg.md` §3
is (round 3 established that one is live because revision 10 is precisely the
revision that changed the §3 content the citation names). This is a real,
structural distinction — I did not find evidence the design's cited §2/§4.2/§4/§5
content changed in a way the frozen design silently missed, and re-deriving that
fully is outside this round's remit (it predates this amendment's diff entirely
and touches none of the three changed lines). **Verdict on this sub-question:
agree with the disposition, minor imprecision in the stated reason — see minor
finding B below.**

---

## 2. D-599 — is its account of gate 20's blindness true, and is its history true?

**The mechanical claim, checked against the script itself.**
`tools/design_citation_check.py`'s only pattern:

```python
PATH = re.compile(
    r"`(?P<path>(?:crates|tools|configs|docs)/[A-Za-z0-9_./-]+\.(?:rs|sh|py|toml|md|txt))"
    r"(?::(?P<line>\d+)(?:-(?P<end>\d+))?)?`"
)
```

It matches a backticked path under `crates/`, `tools/`, `configs/` or `docs/`
with a recognized extension, optionally followed by `:N` or `:N-M`; `check()`
verifies the path exists and, if a line number is given, that the file has at
least that many lines. Nothing in the script inspects the text "revision N", and
a bare `` `wp21_prereg.md` `` (no directory prefix, as every citation in these
two documents actually is) does not even match `PATH`'s required
`(?:crates|tools|configs|docs)/` prefix — doubly invisible. Confirmed by running
it: `tools/governing_citation_check.sh` exits 0, 9 governing documents, 0
unreproduced citations, with the stale citation this amendment fixes having just
been present in the same tree state one commit ago and never flagged. **D-599's
"gate 20 cannot see it" claim is TRUE, verified rather than taken on faith.**
Its cited backing (D-583, D-584) and the "8-of-9 governing documents on
`tools/SHELL_CHECKLIST.md` item 10" claim also check out: item 10 (line 104,
"THE COVERAGE RULE") does require a test driving the shipped script.

**The historical claim — where I disagree.** D-599: *"That citation has been one
bump stale three times: the sweep's round-4 review found it at 7-against-8 (its
M1), the amendment's round 1 found it at 7-against-8 again (F-5), and the
amendment's round 3 found it at 9-against-10."* I traced all three:

- **Amendment round 3 (9-against-10)**: confirmed, this review chain's own
  finding, disposed of by the remedy above.
- **Amendment round 1, F-5 (7-against-8)**: confirmed —
  `wp21_prereg_rev9_amendment_REVIEW.md:57`: *"F-5 \| sibling cited
  `wp21_prereg.md` at revision 7, which is now revision 8 \| CLOSED"*.
- **"The sweep's round-4 review... its M1... 7-against-8"**: does **not**
  reproduce. `wp21_throughput_prereg_rev4_REVIEW.md`'s actual MAJOR 1 (line 228):
  *"The design is cited at revision 7; the tree holds revision 10"* — this is
  about **`wp21_label_cache_design.md`**, not `wp21_prereg.md`, and the mismatch
  is **7-against-10**, not 7-against-8. Confirmed independently by
  `wp21_throughput_prereg_rev5_REVIEW.md:77`, which closes it: *"M1 \| design
  cited at 7, tree at 10 \| CLOSED"*. The actual **first** occurrence of the
  `wp21_prereg.md`-in-`wp21_throughput_prereg.md` citation going stale is
  `wp21_throughput_prereg_rev5_REVIEW.md`'s own **MAJOR 1** (line 214): *"The
  sibling is cited at revision 5; the tree holds revision 6, landed in the same
  commit as this revision"* — **5-against-6**, at round **5**, not round 4. I
  searched every `wp21_prereg_rev*_REVIEW.md` and `wp21_throughput_prereg_rev*_REVIEW.md`
  file for any other 7-against-8 (or any) finding naming the `wp21_prereg.md`
  citation at round 4 and found none; round 4's only revision-citation finding is
  the design-doc one quoted above.

**This is a MAJOR finding under this round's rubric — a claim the tree
contradicts.** It does not change D-599's conclusion (three genuine occurrences
of this defect class did happen — round 5's 5-vs-6, amendment round 1's 7-vs-8,
amendment round 3's 9-vs-10 — and gate 20 catches none of them), so it is not
BLOCKING: no schedule claim, cached-tranche claim, or run-governing number is
touched. But it is precisely the kind of transcribed-rather-than-derived number
D-598 and this whole arc are about, now sitting inside the ADR that explains that
class. **Fix (not executed — this round's remit is to find, not fix):** replace
*"the sweep's round-4 review found it at 7-against-8 (its M1)"* with *"the
sibling's round 5 review found it at 5-against-6 (its MAJOR 1)"*.

---

## 3. Did the remedy introduce anything, read in context?

The two changed lines in `wp21_throughput_prereg.md` (title, line 1; the
GOVERNING citation, line 15) were read against their surrounding paragraphs.
Neither introduces a new number, a new claim about cached tranches, or a new
schedule claim — the bump is exactly the one-word substitution D-599 says it is
(`revision 9` → `revision 10` in two places). `docs/experiments/arc3_ledger.md`'s
new entry states the same facts I independently re-derived above (round 3's
disposition, the MAJOR's class, D-599's ownership of the mechanism). **Nothing
new from the two changed lines themselves** — the MAJOR above is in the newly
authored D-599 prose, not in the sibling fix.

---

## CLOSING SWEEP (my own choosing, this being the round the run launches on)

Relying on, and not re-deriving: the N = 16 selection, C1 over 93 files, C2, §3's
wall block's own arithmetic, and the lower-bound sum — all settled by rounds 1–3
(`wp21_prereg_rev8_amendment_REVIEW.md`, `wp21_prereg_rev9_amendment_REVIEW.md`,
`wp21_prereg_rev10_amendment_REVIEW.md`) and unchanged by this diff.

**Criterion that cannot fail.** Read T-A1, T-A2, T-B, T-C, T-D, T-F (`wp21_prereg.md:191-198`)
and C1-C4's statement (`:56`): every one names a specific program's exit code,
count, or byte-comparison (`cold_label_check` exit 0 over a named stride,
`arena: replayed G of G... 0 divergences`, `counts n... forfeits N` read off a
named source line, `cmp -s` exit 0). None is a tautology or a threshold set
outside its own possible range. No finding.

**Registered command that does not run.** Checked every `tools/*.py`/`tools/*.sh`
and `crates/.../*.rs` path named in a backtick or a command block in both
documents exists in the tree: `tools/wp21_tranche_config.py`,
`tools/cold_label_check.py`, `tools/determinism.sh`, `tools/label_cache_count.py`,
`tools/opening_prefix_fold.py`, `tools/wp21_assemble.py`,
`crates/pistol-arena/src/bin/arena.rs`, `crates/pistol-arena/src/usage.rs` — all
present. (This overlaps `design_citation_check.py`'s own path-existence check,
which also passed at 0 unreproduced across both documents' 26 and 16 citations
respectively.) No finding.

**Self-contradiction about the schedule or which tranches run cached.** Fresh
sweep, my own command:
`/usr/bin/grep -n -i cached docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md`
returns **53** hits (21 + 32), not the 34 round 3's report states for the
identical command at the prior revision (verified unchanged at `4e3d1b6` too —
the discrepancy is in round 3's count, not a change in the tree). I read all 53
myself: every one is either (a) an explicit negation ("no tranche of this sweep
runs cached", "EVERY TRANCHE OF THIS SWEEP THEREFORE RUNS UNCACHED", "no tranche
of the sweep runs cached"), (b) an after-the-wave capability check on no
tranche's critical path (tranche one's own `capture-cached.txt` re-capture,
§4.4's play-pass pair), (c) a refused command (`--label-cache` with `--census` is
refused, shown as a refusal transcript), or (d) a dry-run/pilot example clearly
scoped as such. **No surviving claim that a live tranche of this sweep runs
cached.** The wave sweep (`grep -n -iE 'wave [0-9]|1st wave|2nd wave|first
wave|second wave|wave one|wave two'`) returns exactly six hits, matching round
3's count and categorization exactly; re-read, same conclusion (T-F's row is an
honest N = 8 counterfactual, nothing else asserts a second wave). No finding
against the registrations from either sweep; see minor A below for the round 3
count itself.

---

## MINOR FINDINGS

**Minor A** — Round 3's report states the cached-sweep grep returns "34 hits
total"; the actual count, both at `0be90de` and re-verified at `4e3d1b6` itself,
is 53. This is an inaccuracy in round 3's report, not in either registration —
I independently re-enumerated all 53 hits and round 3's substantive conclusion
(no live cached-tranche claim) stands regardless. Not counted toward this
round's verdict tally since it is not a claim in the two registrations or in
D-599.

**Minor B** — Round 3 and D-599 attribute `wp21_label_cache_design.md`'s
apparently-stale GOVERNING citations to that document "declaring itself pinned...
on its face (`adb2012`)". The literal disclaimer text is scoped to `path:line`
code citations (confirmed: 8 genuine ones exist) and says nothing about
`revision N` document citations. The exemption's real ground is the design
gate's closure at revision 6 (D-589, confirmed) and revisions 7–10 being
IMPL-only, not design, changes — a sounder but different argument than the one
stated. Does not change the disposition (I agree the design's citations are not
this defect class), only its stated reason.

---

## WHAT THIS REVIEW DID NOT LOOK AT AND WHICH ROUND DID

- N = 16's selection, C1's 93-of-93 byte-identical capture files, C2's 93-of-93
  at 152 asked prefixes, and §3's wall-block arithmetic (`1 442 + 1 409 + 58 =
  2 909`, `22 213 − 19 304 = 2 909`, `86.9 % → 87 %`) — settled by rounds 1–3
  (`wp21_prereg_rev8_amendment_REVIEW.md`, `wp21_prereg_rev9_amendment_REVIEW.md`,
  `wp21_prereg_rev10_amendment_REVIEW.md`), unchanged by this diff, not
  re-derived here.
- Whether `wp21_label_cache_design.md`'s own frozen GOVERNING citations
  (`wp21_throughput_prereg.md` revision 3, `wp21_prereg.md` revision 4) still
  correctly describe those documents' §2/§2.0/§4.2/§4.4/§4.5 and §4/§5 content as
  it stood when the design gate closed at revision 6 — I confirmed the
  structural argument for why this is not a *live* citation (D-589, the
  revision-history disclaimer) but did not re-diff the cited sections' historical
  content against the design's requirements; this predates this amendment's diff
  entirely and touches none of its three changed lines.
- No sweep workload, no lever A re-run, no engine binary built, no mutation
  testing — nothing in this diff required one.
- `docs/book_v2_ledger.md`'s own `wp21_prereg.md` revision-7 row — round 3 ruled
  it out of gate 20's scope (not on the GOVERNING list, a record rather than a
  live reference) and I did not re-open that call.
