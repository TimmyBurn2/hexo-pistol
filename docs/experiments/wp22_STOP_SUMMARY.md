# WP-2.2 — STOP, taken at §B's cap registration

## Plain language first

**I stopped because §B's cap-calibration registration failed its third
fresh-context review, and the dispatch grants three rounds per gate with the
third remedies-only. That grant is spent. The STOP protocol says a document
failing after the granted rounds is a STOP — "split, no self-granted round" —
so I did not write a fifth revision.**

**The decision owed to the operator is one question**, and it is not "was the
reviewer right". It is:

> **Is the cap calibration worth running at all, given that every branch of its
> own registered rule already returns 2048?**

The third review demonstrated that, and I reproduced enough of it to believe it.
Three options, and the choice is the operator's:

1. **Do not run it.** Adopt **cap 2048** on the evidence already in hand, record
   it as an ADR line, and go straight to the census registration. Saves ~1 h 40 m
   and asserts nothing the data does not already say.
2. **Run it as a CONFIRMATION**, with the discriminating condition registered
   first — the review supplies it: 2048 is overturned only if
   `w(2048) < 0.76 · w(8192)` or `< 0.64 · w(16384)`, against a dry run where
   `w(2048) = 5.1 · w(8192)`. This is a two-sentence amendment, not a redesign.
3. **Redesign** the calibration around a question whose answer is not foreknown.

**Nothing is broken and nothing is half-finished.** CI is green at the closure
HEAD, the tree is clean, no processes of mine are running, and every artifact is
exported with a digest receipt.

## Why the third review is a STOP and not just another fix round

Rounds used, all fresh-context, none by the implementing session:

| round | revision | verdict |
|---|---|---|
| 1 | revision 2 | FAIL — 13 MAJOR, 8 MINOR |
| 2 | revision 3 | FAIL — 7 findings, 3 measured |
| 3 (remedies-only) | revision 4 | FAIL — 5 MAJOR, 8 MINOR |

Sixteen of round 2's seventeen findings landed in revision 4, and the reviewer
says so. **The document got materially better each round and still cannot govern
a run**, which is exactly the condition the STOP protocol describes.

## The finding that matters, in the reviewer's terms and my own

**B4 — the run's answer is known before it is taken.** §5 selects on `w/s`. On
§6's own dry run:

| cap | `w` | `s` | `w/s` |
|---|---|---|---|
| **2048** | 86 | 341 | **0.2522** |
| 8192 | 17 | 403 | 0.0422 |
| 16384 | 19 | 480 | 0.0396 |

The rule as registered picks the smallest cap within 0.9 of the maximum ratio:
**2048**, already, from the dry run. A 20 000-replicate bootstrap at n = 500
selects 2048 in **100.00 %** of replicates, and `P(w < 10) = 0` at every arm so
the UNDERPOWERED branch never fires — **and its fallback is also 2048.**

`docs/process.md` names this class in its own words: *"neither catches a run
whose answer is already known before it is taken — that defect is judged, not
checked."*

**And §6 asserted the opposite of what its own numbers show** — *"it does not
separate the arms"* — which is true of the root rate (8, 8, 9 of 100) and false
by 6x of the only quantity the selection reads. That sentence is mine and it is
the one that kept the foreknown answer invisible.

## The other four MAJOR findings, and which I verified myself

| id | what it says | verified |
|---|---|---|
| **B5** | §10.2's registered search **does not run** — the pattern begins `--` and is parsed as an option — and its "run at `b876d1d`" label is impossible because `artifacts/` is gitignored, so no revision holds the population | **YES, reproduced.** The command I actually ran escaped the dashes; I dropped the escapes when transcribing it into the document. A transcription defect inside the paragraph invoking D-601/D-602 against transcription defects, which is D-600's shape exactly |
| **B5b** | §2's "the tree holds 25" is scope-dependent and unstated | **YES.** `artifacts/*.txt` gives 25; recursive gives 31, 21 at 50 000. **The extra six are my own dry-run outputs** — the population moved because I measured it |
| **B1** | §1 and §4 still quote numbers from populations D-615 retired | not re-derived; the reviewer's live values are 45 806 / 49 079 / 53 398 |
| **B2** | §8 names `b876d1d` as two instruments' governing revision; they were first added at `42967e0` | not re-derived; mechanically checkable |
| **B3** | §9's "19 of 19" is a 20-row figure; on the registered 100-row input the shift gives 96 of 99, three rows agreeing because one `key_full` occurs four times | not re-derived |

## One caveat the reviews did not raise and I am raising against myself

**Every `s(c)` in this package is wall-clock, and the box was not exclusively
mine.** Another project's arena runs have been executing throughout — 3.1 hours
and 2.2 hours at the time of writing. Load average sat near 2 on 16 cores, so
CPU contention for a single-threaded instrument is unlikely to be material, but
**that is an argument and not a measurement**, and D-592 voids a timing receipt
taken beside a reviewer's build for precisely this reason. A selection rule
whose denominator is wall clock inherits that exposure. Option 1 above is immune
to it; options 2 and 3 are not.

## What is green, and unaffected by this STOP

- **CI at the closure HEAD**: 20 of 20 gates, `ci: all gates passed`, `EXIT=0`,
  over a tree clean for the run's whole length.
- **§0 and §R** complete, with receipts; thirteen worktrees removed.
- **D-611 closed** by R2's tabulation.
- **Phase 1**: the linearity oracle (0 of 500), the loader-inertness pair, the
  fit, D-616's censoring finding, and a validated SPRT candidate that changes
  play. **The SPRT has not run** and the committed weights have not moved.
- **D-615, D-616, D-617** recorded.

## What is owed, beyond the one decision

1. The cap decision above, then the census registration (drafted, unreviewed).
2. **Phase 1's SPRT** — candidate prepared, expectation registered (it loses).
3. **R3's seat-swap anchor** — configured, needs a quiet box.
4. **Phase 2** — not started.

## State on disk

Tree clean at `dev`. No WIP anywhere, so nothing is parked on `wp22-stopped`;
the branch was not created because there is nothing to put on it. `git worktree
list` holds the main tree alone. All artifacts under `artifacts/wp22_*` carry
`RECEIPT.sha256` or a named receipt.
