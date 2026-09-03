# REVIEW of the WP-2.1 revision-8 amendment (lever A fills the concurrency slot at N = 16)

**REVISION REVIEWED**: `e3497b9` on `dev` (HEAD at dispatch). `git rev-parse HEAD`
at the end of this review: `e3497b95884060d341632b5907ab1ab481915dbe` — **still
equals `e3497b9`**, unchanged throughout.

**REVIEWED ON: Sonnet, per D-597.**

Scope, per the dispatch: the amendment only — `git diff c68e69e e3497b9 --
docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md`.
Rounds 3–5 already reproduced everything else in both documents and are not
re-litigated here (in particular, the `2.06 h in all` T-F-plus-recapture surcharge
figure at `wp21_prereg.md` line 166 is byte-identical to revision 7 and is out of
scope).

Worktree used: `git -C /home/tom/Projects/HeXO-AlphaBeta worktree add --detach
/home/tom/pistol-wt/amend e3497b9`, removed at the end of this review. No build was
required; all checks are text derivation, arithmetic, and file-content checks
against artifacts already on disk.

---

## VERDICT: **FAIL** — 4 BLOCKING, 2 MAJOR, 0 minor

The sweep does not launch on this revision. Four findings are BLOCKING: a
self-contradiction about which tranches run cached (§1 vs the amended §6.1), an
unaddressed regression in T-F's early-stop guarantee caused by the move from two
waves to one (compounded by three leftover "between the waves" / "wave two"
references the amendment should have removed), a cross-document contradiction
about whether the cached re-capture sits on the sweep's critical path (the
sibling cites `wp21_prereg.md` §3 for a claim §3 no longer makes), and a
misapplication of the registered "median rep" rule at N = 1 that puts a wrong
number in a criterion the amendment quotes as PASS evidence.

---

## ITEM 1 — Was the rule applied as registered, and only once?

**Command** (my own, not the document's):

```
python3 /home/tom/pistol-wt/amend/scratch/lever_a_check.py
```

reading `artifacts/arc3r_leverA_c68e69e.txt`'s fifteen `leverA:` lines, computing
exact throughput `= N * records / wall_s` per line (not the printed 3-decimal
field), sorting each N's three reps by that exact value, and taking the middle
one as the median rep.

**Output** (median throughput per N, and the median rep found):

```
N=1  reps [1.098107, 1.09933, 1.09802]  -> median 1.098107  rep 1  s_per_label 0.910660
N=2  reps [2.183359, 2.172794, 2.177494] -> median 2.177494  rep 3  s_per_label 0.918487
N=4  reps [4.360111, 4.338333, 4.352619] -> median 4.352619  rep 3  s_per_label 0.918989
N=8  reps [7.974659, 7.984609, 8.024496] -> median 7.984609  rep 2  s_per_label 1.001925
N=16 reps [10.310808, 10.313519, 10.331307] -> median 10.313519 rep 2 s_per_label 1.551364
```

These reproduce the document's stated median throughputs 1.098, 2.177, 4.353,
7.985, 10.314 exactly.

**§3.4's rule, applied**: highest median = 10.313519; 95 % of it = 9.797843.
N = 1, 2, 4, 8 are all below that bar; N = 16 (10.313519) is at/above it. **The
smallest N at or above the bar is 16.** No other reading of the registered
sentence ("the smallest N whose median throughput is at least 95 % of the
highest median throughput over the field") gives a different N — the field is
monotone increasing in this data, so there is no tie or crossing to dispute.
Incumbent 8 sits at `7.984609 / 10.313519 = 77.42 %`, matching the document's
"77.4 %".

**C3, every setting**: `c(N) = median-rep s_per_label / 0.885445`:

```
N=1  c=1.0285   N=2 c=1.0373   N=4 c=1.0379   N=8 c=1.1315   N=16 c=1.7521
```

All ≤ 4 (bar 3.54178 s); C3 PASSES at every setting, including N = 16
(`c(16) = 1.7521`, matching the document exactly).

**C4 at N = 1 — FINDING (BLOCKING), see F-4 below.** The median rep for N = 1,
by the rule's own definition (median-by-throughput, ties broken by the actual
sorted order, not by the rounded 3-decimal display), is **rep 1** (s_per_label
0.910660), not rep 3 (0.910729) as the document's percentage implies. C4 still
PASSES either way (both are inside the 20 % bar), but the printed percentage is
wrong. Full detail in F-4.

## ITEM 2 — Do C1 and C2 hold as the receipt claims?

**93 derived from the registered schedule**: §3.2 registers reps `{1,2,4,8,16}`
× 3 reps = `3 × (1+2+4+8+16) = 93`, matching C1's own stated derivation.

**Commands**, run directly against the read-only capture directory:

```
$ cd /home/tom/pistol-runs/arc3r-leverA/run/leverA/
$ ls cap-*.txt | wc -l
93
$ for n in 1 2 4 8 16; do ls cap-N${n}-rep*-p*.txt | wc -l; done
3   6   12   24   48        # matches 3×N exactly for each setting
$ sha256sum cap-*.txt | awk '{print $1}' | sort -u | wc -l
1
$ sha256sum cap-N1-rep1-p1.txt
83c706ac4fa634e9357971530c8bc26168b39df39d9f74949f83f30b57100e27  cap-N1-rep1-p1.txt
```

All 93 files hash identically — **not a spot check, a full sweep** — and the
single hash matches the receipt's claimed `83c706ac…`. **C1 fully confirmed: 0 of
93 differ.**

**C2 spot-check**, record counts against the report's 152:

```
$ for f in cap-N1-rep1-p1.txt cap-N2-rep2-p2.txt cap-N4-rep3-p4.txt \
           cap-N8-rep1-p8.txt cap-N16-rep2-p16.txt cap-N16-rep3-p1.txt; do
    grep -vc '^#' "$f"
  done
152  152  152  152  152  152
```

Every file checked holds exactly 152 data records, matching C2's claim (and the
receipt's own `asks 152` on every one of the fifteen `leverA:` lines). **C2
confirmed** on this sample; combined with the receipt's own `rc 0` on all fifteen
lines (visually confirmed by reading the file), nothing contradicts "0 of 93
short of 152."

## ITEM 3 — Is §3's new wall arithmetic right?

**Re-derivation** (my own script):

```
capture16 = 12443 * 1.551364 = 19303.62  -> 19304 s = 5.3622 h -> "5.36 h"  OK
play      =   436 * 0.827115 * 4 = 1442.49 -> 1442 s = 0.4007 h -> "0.40 h" OK
replay    =   436 * 0.807692 * 4 = 1408.61 -> 1409 s = 0.3913 h -> "0.39 h" OK
cold      =    64 * 0.904313 = 57.88 -> 58 s                               OK
total     = 22212.60 -> 22213 s = 6.1702 h -> "6.17 h"                     OK

N=8 comparison:
capture8  = 12443 * 1.001925 = 12466.95 -> 12467 s                        OK
tranche8  = 12467+1442+1409+58 = 15376 s                                  OK
2 waves   = 30751.86 -> 30752 s = 8.5422 h -> "8.54 h"                    OK

SERIAL uncached = 3487 * 57.0769 * 0.885445 = 176227.60 -> 176228 s
                = 48.9521 h -> "48.95 h"                                  OK
```

Every line of the N = 16 block and the N = 8 comparison reproduces exactly.

**MEASURED/ESTIMATED marking**: the *rate* `1.551364 s/label` is correctly marked
MEASURED (it is lever A's own median-rep reading, not extrapolated); the
*capture total* (`12 443 × 1.551364`) is correctly marked ESTIMATED (the 12 443
records figure is itself an estimate). Play, replay and cold correctly stay
ESTIMATED off the pilot's idle rates.

**FINDING (MAJOR), F-6**: The paragraph immediately below this block —
"**EVERY FIGURE IS A LOWER BOUND.** The per-label rate was measured with the box
otherwise idle…" (lines 171–176, untouched by the amendment) — is now
**inaccurate for the dominant term**. The amendment's own preceding sentence
(lines 134–138) says the capture term (≈79 % of a tranche, §1 of the sibling) "is
no longer estimated at the pilot's serial rate but MEASURED at the concurrency
the sweep runs at." A rate measured *at* the contention it will run under is not
an idle-derived lower bound the way play/replay/cold still are (those still use
the pilot's uncontended rates). The blanket claim "every figure is a lower bound"
directly contradicts the amendment's own characterization of the capture term two
paragraphs above it, and the amendment did not narrow the claim when it changed
the capture term's basis.

## ITEM 4 — Is §6.1's conclusion sound?

**Attack**: is there a schedule, including a re-run of a VOID tranche, under
which a tranche could legitimately run cached? The amendment anticipates exactly
this and forecloses it explicitly: "EVERY TRANCHE OF THIS SWEEP THEREFORE RUNS
UNCACHED — a re-run of a VOID tranche included, because the gate would have
returned by then and putting one tranche's corpus on a different instrument from
the other fifteen is not worth a saving §3 prices at 0.20 h." This is a
**policy** choice (uniformity of instrument across all sixteen corpora), stated
honestly as a choice rather than as a technical necessity — I find no gap in this
argument. No schedule survives the attack: even a VOID tranche's re-run, occurring
after the gate has resolved, is explicitly and correctly closed off. **This part
of §6.1 is sound.**

**Self-contradiction check — §1's cache row — FINDING (BLOCKING), F-1**: `wp21_prereg.md`
line 57, untouched by the amendment:

> `| **label cache** | **ON** from the tranche §6.1 names, keyed on the
> position line's exact bytes | D-576; … which tranches run cached, and on what
> condition, is §6.1's and nothing else's |`

This asserts the cache reaches an **ON** state "from" a tranche §6.1 **names**.
§6.1, as amended, names no such tranche — it concludes "EVERY TRANCHE OF THIS
SWEEP THEREFORE RUNS UNCACHED." In revision 7 (before N was known) this row was
a coherent forward reference to whatever tranche a not-yet-run gate would
identify; the amendment filled the slot with a concrete answer (no tranche ever
switches on) and updated §6.1 to say so, but did not update §1's row to match.
The result is a live self-contradiction in the governing document about which
tranches run cached — exactly the class of defect this review is instructed to
treat as BLOCKING.

**§5's run-log rule, §9's commands**: checked, no contradiction. §5's gate-line
mechanism ("a block whose pass 2 carries the flag with no such line above it… is
a VOID tranche") is now a dead-but-harmless safety net, since no pass-2 command
in §9 ever carries `--label-cache` for a real tranche; it would still catch an
operator error. Not a defect.

**The sibling's §4.4 — FINDING (BLOCKING), F-3**: see below, filed under item 5
since it is a "did the amendment break what it touched" defect, but it directly
answers item 4's question about the cache's self-consistency: yes, the
registration now contradicts itself about the cached re-capture's critical-path
status, between `wp21_prereg.md` §3 (amended: "on no tranche's critical path")
and `wp21_throughput_prereg.md` §4.4/§5 (unamended: "on the critical path,
`wp21_prereg.md` §3" — citing the very section that now says the opposite).

## ITEM 5 — Did the amendment break anything it touched?

**§4's criteria still falsifiable**: T-A, T-A2, T-B, T-C, T-D are untouched and
unaffected. T-F's core test (`cmp -s` byte-identity, both halves) remains
well-defined and falsifiable. But see F-2 below — T-F's **registered
consequence** no longer describes an achievable schedule.

**FINDING (BLOCKING), F-2 — the "between the waves" / "wave two" leftovers, and
the T-F stop this breaks**. The amendment's whole premise is "sixteen tranches at
once is ONE wave" (line 135), yet the following text, none of it touched by the
diff, still speaks of two:

- `wp21_prereg.md` line 169 (end of the very paragraph the amendment rewrote):
  "…the referent re-capture sits **between the waves** as the gate line says" —
  contradicting the same sentence's own opening clause, four lines up, that the
  recapture is "taken AFTER the wave… and on **no** tranche's critical path."
- `wp21_prereg.md` line 192, §4's own T-F criteria table, REGISTERED CONSEQUENCE
  column (untouched): "THE SWEEP STOPS, before wave one on the first half and
  **before wave two on the second**." There is no wave two at N = 16.
- `wp21_prereg.md` line 448, §9's command comment (untouched): "# tranche one
  only, **between the waves**: the sibling's §4.4 referent pair."
- `wp21_throughput_prereg.md` lines 370–371, §4.5 (untouched): "its uncached
  capture runs in wave one under N-way contention and its re-capture alone
  **between the waves**."

Beyond the wording, this is a **substantive regression the amendment does not
acknowledge**: T-F's registered purpose is to stop the sweep *before wasting
compute* on tranches that would inherit a broken capture instrument — in the old
two-wave design, a T-F failure detected after wave one would cancel wave two
before it started. At N = 16 all sixteen tranches run **concurrently as one
wave**; T-F's second half needs tranche one's own capture, which becomes
available only once tranche one — running alongside the other fifteen at the
same contended rate — finishes its own capture pass, i.e. at essentially the same
wall-clock moment the other fifteen tranches finish theirs too. By the time T-F's
second half can even be evaluated, there is no "wave two" left to cancel: the
compute T-F exists to protect is already spent. The amendment reduces the wall
estimate and declares the cache moot for this sweep, but never revisits whether
T-F's stop-before-further-compute guarantee still holds at the concurrency it
just selected. It does not.

**§9's pass-2 command vs. tranche one's referent — checked, correct**: the
pass-2 command template (lines 417–418) carries no `--label-cache`; tranche one's
`§4.4` referent pair (lines 449–451) still does. This is exactly the split the
document intends and it is implemented consistently. No defect.

**The sibling's corrected "second wave" sentence — true in itself, but its
neighbours are now false — FINDING (BLOCKING), F-3**: The amended sentence itself
("at the N = 16 lever A selected there is no second wave for it to shorten") is
true and consistent with `wp21_prereg.md` §6.1. But the surrounding,
**untouched** text in the same document is now false or stale:

- `wp21_throughput_prereg.md` §4.4 (lines ~344–347): "…the only added cost is
  its cached re-capture (1.43 h ESTIMATED, **on the critical path**,
  `wp21_prereg.md` §3)." `wp21_prereg.md` §3, as amended, says the opposite in
  so many words: the re-capture is "taken AFTER the wave… and **on no tranche's
  critical path**." The sibling cites §3 for a claim §3 no longer makes.
- `wp21_throughput_prereg.md` §5's cost table, untouched row: "lever B §4.4's
  tranche-sized pair | the cached re-capture alone, ~1.43 h, **on the sweep's
  critical path**" — same contradiction.
- §5's prose immediately above the corrected sentence still leans on "the
  difference between its cached-with-gate line and its uncached line is ~0.20 h
  net on this sweep" — but `wp21_prereg.md` §3 no longer has a "cached-with-gate
  line": the amendment deleted that two-wave arithmetic block outright ("The
  arithmetic that priced a cached second wave at a net 0.20 h **is gone** with
  the second wave"). §5 states a comparison against a computation that no longer
  exists in the cited document, then reaches "the decision to take it is the
  architect's (D-576), taken against this number" — the number the architect's
  decision is described as resting on is stale.

## ITEM 6 — Citations and digests

**Both documents at revision 8**: confirmed — `wp21_prereg.md` line 1 and
`wp21_throughput_prereg.md` line 1 both read "revision 8."

**FINDING (MAJOR), F-5**: `wp21_throughput_prereg.md` line 13 (GOVERNING
section, untouched by the amendment) still reads:

> `D-587, D-588 (the key and what is claimed about it); `wp21_prereg.md` revision 7`

`wp21_prereg.md` is now revision 8. Checked against c68e69e: at revision 7 of
both documents this citation correctly said "revision 7" (self-consistent then);
the amendment bumped both headers to 8 but did not bump this cross-reference,
so the tree now directly contradicts this citation.

**Receipt digest**: `sha256sum artifacts/arc3r_leverA_c68e69e.txt` =
`5eb62d6c909699d76078b96f71801741d544fda98c3a58cbb07a83209eeeef01`, matching §1's
cited digest exactly.

**`tools/governing_citation_check.sh`**:

```
$ CARGO_TARGET_DIR=/home/tom/pistol-wt/amend/target bash tools/governing_citation_check.sh
governing_citation_check: 9 governing document(s), 0 proposed path(s)
... (all nine documents) 0 unreproduced
DESIGN_CITATION_CHECK_DONE
$ echo $?
0
```

Exits 0. Note its own printed caveat: it verifies that quoted text reproduces
byte-for-byte, not that a document's claims are logically consistent with each
other or with a sibling's revision number — it is the wrong instrument for F-1,
F-2, F-3, F-5 and F-6, all of which are prose-level contradictions rather than
misquotations, and it does not claim to catch them.

---

## FINDINGS, WITH REPRODUCERS

**F-1 (BLOCKING)** — `wp21_prereg.md` §1's "label cache" row ("ON from the
tranche §6.1 names") contradicts §6.1's amended conclusion that every tranche of
this sweep runs uncached. Reproducer: `sed -n '57p' docs/experiments/wp21_prereg.md`
vs `sed -n '345,362p' docs/experiments/wp21_prereg.md`.

**F-2 (BLOCKING)** — Four leftover "between the waves" / "wave two" references
(`wp21_prereg.md:169,192,448`; `wp21_throughput_prereg.md:370-371`) contradict the
amendment's own "one wave" premise, and T-F's registered stop-before-wave-two
consequence no longer describes an achievable schedule at N = 16 — a compute-cost
regression the amendment introduces without discussion. Reproducer:
`grep -n "wave" docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md`.

**F-3 (BLOCKING)** — `wp21_throughput_prereg.md` §4.4 and §5 still assert the
cached re-capture is "on the critical path," citing `wp21_prereg.md` §3 for it;
§3 (amended) says "on no tranche's critical path." §5's "~0.20 h net" narrative
also still leans on a two-wave arithmetic block `wp21_prereg.md` §3 explicitly
says is "gone." Reproducer: `grep -n "critical path" docs/experiments/wp21_prereg.md
docs/experiments/wp21_throughput_prereg.md`.

**F-4 (BLOCKING)** — §3.3's rule ("the median rep by throughput… C3 and C4 are
read at that median rep and at no other") is misapplied at N = 1. The true
median rep, by exact throughput (`152/wall_s` to full precision) or equivalently
by `s_per_label`, is **rep 1** (`wall_s 138.420`, `s_per_label 0.910660`); the
document's `+2.86 %` figure corresponds to **rep 3** (`wall_s 138.431`,
`s_per_label 0.910729`), the wrong rep — the three reps' rounded 3-decimal
throughputs (1.098, 1.099, 1.098) tie at that precision, but the exact values do
not: rep 3 (1.098020) < rep 1 (1.098107) < rep 2 (1.099330), so rep 1 is the
middle value. Correct figure: `(0.910660-0.885445)/0.885445 = +2.85 %`, not
`+2.86 %`. Does not flip C4's PASS (both are inside the 20 % bar) or change the
selected N. The arc3_ledger.md §4 table (line 1718, "its rep" = 3 for N = 1)
carries the identical error; every other row's "its rep" (N=2,4,8,16) is
correct. Reproducer:

```
python3 -c "
records=152
for rep,wall in [(1,138.420),(2,138.266),(3,138.431)]:
    print(rep, records/wall)
"
```
sorted ascending gives rep3 < rep1 < rep2, median = rep1.

**F-5 (MAJOR)** — `wp21_throughput_prereg.md:13` cites "`wp21_prereg.md`
revision 7"; the tree's `wp21_prereg.md` is revision 8. Reproducer:
`sed -n '1p' docs/experiments/wp21_prereg.md; sed -n '13p'
docs/experiments/wp21_throughput_prereg.md`.

**F-6 (MAJOR)** — `wp21_prereg.md:171-176` ("EVERY FIGURE IS A LOWER BOUND")
is stale relative to the amendment's own claim two paragraphs above (`:134-138`)
that the capture term is now measured directly at N = 16 contention rather than
extrapolated from an idle rate. Only play/replay/cold remain idle-rate lower
bounds; the capture term, ≈79 % of the wall, is not. Reproducer:
`sed -n '134,176p' docs/experiments/wp21_prereg.md`.

---

## WHAT THIS REVIEW DID NOT LOOK AT

- Rounds 3–5's own territory: T-A/T-A2/T-B/T-C/T-D's substance, the partition
  arithmetic (§2), the assembly instrument, the toolchain/digest table (§8), the
  dry run's own record (§9.1), D-586's flip clause, and every line of §3 that
  the diff did not touch (including the `2.06 h in all` T-F surcharge total,
  confirmed byte-identical to revision 7 and therefore out of scope here).
- `wp21_label_cache_design.md` and `matrix_label_cache_key.md` themselves —
  cited but not re-opened by this amendment.
- Anything about whether N = 16 is a *good* engineering choice beyond the
  registered rule's mechanical application — that was settled by §3.4 before
  the run and is not this review's business.
- The dry run's own record (`artifacts/arc3r_dryrun_sweep_0c4f3b4_v4.txt`) was
  read but not independently re-verified byte-for-byte against a fresh dry run;
  it predates this amendment and was not itself amended.
- Whether F-2's T-F regression is severe enough that the architect should
  reconsider N = 16 itself, versus simply rewording T-F's registered
  consequence to match a one-wave schedule — that is a design decision this
  review surfaces but does not adjudicate.
