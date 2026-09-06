# `random_openings_v3.txt` — the consumed-ranges ledger

**What this file is.** One row per governed pre-registration that draws a slice
of `crates/pistol-cli/tests/fixtures/random_openings_v3.txt`, recording the
range it consumed. A slice appears here **when its pre-registration is
committed**, not when its run finishes — a range reserved by a document that was
never run is still spent, because a later run over it would be a second reading
of a sample someone else chose.

**Why it exists.** The same reason `docs/book_v2_ledger.md` does:
`random_openings_v1.txt` had no ledger, and what its ranges were had to be
recovered by reading `openings_skip`/`openings_take` out of six arena configs
after the fact (D-491, D-492). This file is so that the same question about v3 is
answered by reading one document.

**The rule.** A new pre-registration takes the next unconsumed range, adds its
row here in the same commit that adds its arena config, and never re-reads a
range this table already holds. Reading a consumed range for a CLOSED verdict is
not a new use and needs no row.

**THE BOOK HOLDS 8500 OPENINGS, AND THE NUMBER IS DERIVED.** `pairs_v3 = 7800`
is the smallest pair cap the shipped `crates/pistol-arena/examples/sprt_power.rs`
measures at power ≥ 0.90 for `elo0 0`, `elo1 10`, `alpha = beta = 0.05` on the
WP-2.2 Phase 1 pentanomial (`--buckets 2,3,8,7,4`); 7750 answers 0.8992 and is
below. `ceil_to_500(7800 + 500) = 8500` applies `book_v2_registration.md` §4's
rule. The worst-case closed form `ln(19)²/t1²` floors it at 5233 pairs and does
not bind. Receipt: `artifacts/book_v3/POWER_v3.txt`, sha256
`ea77d4416e8d2f39c89d7a4d7aa7319cfc78e5c8022cb524c49628fe612d3dbf`
(docs/decisions.md D-643).

**WHAT MAKES THIS BOOK DIFFERENT FROM ITS TWO PREDECESSORS, and it is the one
thing a reader must know before drawing from it.** v1 and v2 are drawn and
nothing more; each is reproducible from its own config alone, and both say so in
their own bytes. **v3 is FILTERED.** Its openings are disjoint, under canonical
form, from every opening in v1 and in v2 — not by luck but because 38 candidates
were rejected to make it so. That is a knowing departure from D-518, which
declined exactly this filter for v2 on the ground that it makes a fixture's bytes
depend on another file's; what buys it is D-644, and D-647 records the decision
and the strongest attack that survived it. The cost is that rebuilding v3 needs
three committed files rather than one.

**DISJOINTNESS IS MEASURED, NOT ASSERTED**, by a script that shares no code with
the generator — `tools/book_v3_disjointness.sh`, which reads the finished books
through `book_keys` and prints both terms of every count:

```
book_v3_disjointness: control: corpus5 ^ v2: 3487 of 3487 (the renderings agree)
book_v3_disjointness: v3 vs v1: 0 of 8500
book_v3_disjointness: v3 vs v2: 0 of 8500
book_v3_disjointness: v3 vs corpus: 0 of 8500
book_v3_disjointness: v3 internal: 0 of 8500
```

The **corpus** term needs the 43 MB labelled corpus, which lives outside the
repository (D-636), so this is deliberately NOT a CI gate: a gate depending on it
would be green on one workstation and red on every clone. The control line is
what makes the corpus zero meaningful — the corpus was labelled from `book_v2`
openings `13..3499`, so every five-stone corpus key must be one of them, and a
rendering that had drifted from the arena's `key_full` would report `0` for the
wrong reason.

## Consumed ranges

| `openings_skip` | `openings_take` | range | consumed by | pre-registration |
|---|---|---|---|---|
| — | — | — | **nothing yet** | — |

The book is whole. It was built for the R7 acceptance question (D-638), and the
first pre-registration to draw from it adds the first row.

## Not a consumed range: the loadability check

`book_v3`'s own package played a **four-opening paired smoke match** over
`0..3` to prove the arena loads and plays this book. It is recorded here and
takes no row, because it is not a governed use: it makes no claim, has no
pre-registration, and its verdict is the degenerate one a self-match must give.
A successor may draw `0..3` for a governed run without it being a re-read —
nothing was concluded from those games.

## Standing claims on the book, not yet consumed

| claimant | status | what it will need |
|---|---|---|
| The R7 acceptance SPRT | **THE REASON THIS BOOK EXISTS** (D-638, D-643) | 7800 pairs at the registered bounds; the book covers it with 700 openings to spare |

**AND A NOTE THE v2 LEDGER SHOULD BE READ WITH.** D-568 reserves v2's last 1000
openings for governed runs. R1's measurement is that the R7 question needs 7800
pairs, so that reservation cannot power it, or the WP-1.5d resolution run, at the
bounds either would register. Whether the v2 holdout is retained for some smaller
question or retired is a ruling this package did not take.
