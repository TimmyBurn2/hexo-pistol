# WP-2.0 — CLOSURE. The label pipeline exists, ran, and produced this project's first corpus.

> **ONE LINE FOR THE MORNING.** The pilot ran and passed every criterion — 742
> labelled positions, all 742 re-checked cold in fresh processes and byte-equal —
> so WP-2.0 is closed; next is WP-2.0b, which still blocks production runs.

## 1. In plain language

The pipeline plays self-play games, then walks the report position by position and
asks the engine again at a deeper budget, then turns that capture into a corpus.
It ran end to end on thirteen registered openings from `book_v2` and every check
passed. **The one thing worth knowing beyond "it passed": the corpus has outcome
signal after all.** A four-game dry run had every game hit the turn cap, which
looked like a structural problem — a corpus that could never say who won. At
thirteen openings, eighteen of twenty-six games decided, and 56 % of records carry
a result. That was a fact about four games, not about the design.

**The pipeline itself was never what failed in this arc.** What failed, five
times, was the document governing the run; it was landed by your conditional take
(D-557) and the run then passed on the first attempt.

## 2. The criteria, each quoted from the run's own output

Everything below is from `artifacts/wp20pilot_RUN_2cd4f79_v1.txt`, whose first
lines are the provenance receipt: the revision, a clean tree, and a digest for
each of the four instruments.

| criterion | registered | measured |
|---|---|---|
| **C-A** cold-label agreement | every sampled record byte-equal, re-asked in a fresh process; any mismatch STOPS the arc | `cold_label_check: 742 of 742 sampled record(s) agree byte for byte`, `exit 0`, **stride 1 — every record, not a sample** |
| **C-B** determinism re-run | both passes byte-identical over the whole range | `capture-determinism exit=0`, `labels-determinism exit=0`, read from `cmp -s` |
| **C-C** replay, zero forfeits | exit 0, zero divergences, `covered == total` | `replayed 26 of 26 game(s) … 0 divergence(s)`, `exit 0`; forfeits **zero** |
| **C-D** throughput | MEASURED, no target registered | **0.885 s per label**, 4 066 labels/hour serial; 26 games in 21.5 s at 4 workers |
| **C-E** schema and loader | control loads; two injections refused on different guards | `ok, 742 record(s)`; grammar injection refused naming `key_pos`; digest injection refused naming the digest; exits 0 / 1 / 1 |

**Verdict V1 — PASS.** Zero forfeits, so not even V2.

**Zero forfeits is read from two receipts and not from one exit code**, because
`arena --config` returns `1` from two disjoint branches: pass 1 printed its
`VERDICT` line (so it completed) and exited `0` (so the tally is zero), and the
corpus's own `end 1 (normal)` across all 742 records corroborates.

## 3. What the run measured

```
13 openings -> 26 games -> 742 records -> 347 distinct positions
   n 26, distinct-n 13          the paired seats play each opening identically
   9 W / 9 L / 8 capped         18 of 26 games decided
   result spread                p1_win 258, p2_win 156, capped 328
   key_seq = key_pos = key_full = 347   all three keys agree on distinctness
   whole pilot                  2 029 s (34 min), against 3 297 s estimated
```

**It finished early because `p = 41` was an upper bound**, as §6.3 said: a decided
game contributes fewer asked positions than a capped one.

**AND IT CAME IN UNDER ITS OWN FLOOR, reported rather than repaired.** RULE-1's
floor (b) asks for at least 1 000 asked positions; the run produced **742**, for
the same reason it finished early. §6.1 registered no consequence for this and
said so — floor (b) fixes a size before the run and cannot be re-read after it —
so the number is reported beside the rate and the rate is a rate over 742.

## 4. The corpus-size plan — ESTIMATED (D-560)

Every input measured in the pilot; the arithmetic is shown so it can be re-derived
rather than trusted.

```
per label ask                  0.885 s   (742 asks in 657 s, SERIAL)
duplication factor             2.14x     (742 records / 347 distinct)
                                         = 2x paired seats + 0.14x transposition
so N distinct positions cost   2.14 N x 0.885 s  =  ~1.89 N seconds
per 100 000 distinct           ~52.6 hours, single-threaded
```

**The book is the binding ceiling, and that is the plan's most useful number.**
The pilot yielded **26.7 distinct positions per opening**. `random_openings_v2.txt`
holds 4 500 openings, 13 now consumed, so the remaining book affords about
**119 800 distinct positions at about 63 hours**. A corpus larger than that needs a
new book, not more machine time.

**Labelled ESTIMATED because it extrapolates from one 13-opening run**, and it
carries that run's shape: change `turn_cap`, the seat, or the label budget and
positions-per-opening and seconds-per-label move together, so the plan is
re-derived rather than scaled.

## 5. What is next, and what still blocks it

**WP-2.0b — census position identity on the wire, gated.** Production label runs
begin only after it lands (D-539): the pilot carried no census and is not corpus,
and its games count toward no minimum. That package has **two** obligations — the
census behind a token no committed config sets, AND the position identity that
makes D-537's *"win-proving firings on DISJOINT positions"* countable. Landing
only the first ships a flag that cannot answer the question it exists to ask.

**Then the Stage-2 eval design package**, consuming §4's plan and the Research-A
findings, bound by three inputs (D-558, amended by D-559): the corpus *does* carry
outcome signal, so state how outcome is USED; every position appears ~2.14 times
and cannot be configured away, so state the dedup or weighting rule; the coldness
overhead is a measured ~12 ms per ask and enters the throughput arithmetic.

**Standing, and independent of all of the above (D-534):** the 725 ms median
movetime overshoot blocks any play-config arming of the solver, and no SPRT
discharges it — it is abort-responsiveness, not strength.

## 6. What this closure does not claim

No strength claim is made and none can be: both seats are one engine, which is the
only shape a capture can be taken from. The pre-registration carries an
**append-only ERRATA block** of eleven acknowledged findings left unfixed by design
under D-557; none touches a registered value, and a reader relying on that document
should read it. The pilot's corpus is a shakedown artifact, not training data.
