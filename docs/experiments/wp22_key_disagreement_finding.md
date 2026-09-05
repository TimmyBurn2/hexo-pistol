# WP-2.2 §R/R2 — the key-disagreement tabulation D-611 owes, and what it closes

**Run revision** `71fa6f1`. Source
`artifacts/arc3r_sweep_deduped_manifest.txt`, 89 805 deduped rows. The table
itself is `artifacts/wp22_key_disagreement_table.txt`, sha256
`9ecb4ff526f9f6f32fef4e696d4dfcd77d12f84ef8cded031d88cbf31b8cf50c`
(rule 8: the artifact is not committed, this document sha-indexes it).

## The count reproduces independently

The manifest's own header derives `key_disagreements 2369`. Recomputing it from
the body rows — a row disagrees on a key when another distinct row carries the
same value — gives **2 369**. The two agree, and the recomputation does not read
the derived figure.

## The classes

| class | rows |
|---|---|
| shares `key_seq` **and** `key_full` | 2 323 |
| shares `key_full` alone | 46 |
| shares `key_pos` | **0** |
| **any key — the manifest's own quantity** | **2 369** |

**There is no transposition-only class.** Not a small one: zero. The question
D-562(2) left open — *"which key rules a disagreement"* — was framed around a
class this corpus does not contain.

**Every disagreement is a SYMMETRY equivalence**: the same position reached in
a mirror image of the same game (2 323) or of a different one (46).

## What it closes

D-611's flip clause reads: *"Flips if the tabulation shows a class none of the
three consumers' rules handles."* **No such class exists.** All three consumers
address symmetry equivalence directly — strict dedup deliberately keeps both
rows, the trainer folds them by the symmetry-canonical key because symmetric
positions share value by construction, and the census counts them once under
D-570's C2. **The ruling stands, closed by this data rather than by argument**,
which is what D-611 required.

## One collateral finding, because it supersedes a measurement this project leans on

**The symmetry fold's yield at the ROOT population is 2 369 of 89 805 — 2.6 %,
not zero.** D-560 measured zero on the pilot's 347 positions and D-570 measured
zero over 798 in-tree firings.

**The two are not in conflict, and the distinction is the POPULATION.** In
tree, where a search generates its own transpositions, the fold still yields
nothing — this package's own dry-run census returned distinct `key` equal to
distinct `key_pos` at 54 each over 61 firings. At the ROOT population, over a
corpus two orders of magnitude larger than the pilot's, it yields 2.6 %.

A successor reading *"the symmetry fold's yield is zero"* must therefore ask
which population is meant. **D-570's C2 is untouched either way**: its ground
was the definition of disjointness, never the fold's measured yield, and D-570
says so in its own words.
