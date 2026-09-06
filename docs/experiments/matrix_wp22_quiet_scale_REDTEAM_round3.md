# DECISION-RED-TEAM — `matrix_wp22_quiet_scale.md` revision 3 (round 3 of three)

**Named revision**: `0220d83` (`dev`), matching HEAD at the start and the end, live tree clean at
both. The reviewer built one detached worktree under `/home` with its own `CARGO_TARGET_DIR` and
left it in place because its play-change logs were its only engine record; it is exported to
`artifacts/wp22_redteam3_worktree_export/` (19 files, receipt clean) and removed.

**Instruments**, none of them the document's: an extractor taking the position from the manifest's
`key_full` stone list and enumerating windows by grouping stones onto lines and sliding a length-6
two-pointer per cluster — a different data path from `extract.py` and a different algorithm from
`features.py`; an exact-rational active-set solver for every fit including four pins; an exact
minimum-hitting-set blockability probe; and a play-change probe drawing a **seeded uniform sample
over all 74 671 eval rows** rather than a stride.

**Self-checks before any number**: the reviewer's extractor agrees with `features.py` on 3 000
random positions (0 disagreements) and its `key_full` path agrees with the `moves`-replay path on a
1-in-11 stride of all 89 805 (0 disagreements). The shipped `fit.py` at `0220d83`, fed the
reviewer's own rows, prints `[5, 34, 60, 300, 1500]`.

## Discharge table — round 2's six MAJOR and ten MINOR

| # | round 2's finding | status |
|---|---|---|
| M-1 | instruments absent at the named revision; no design §9 | **DISCHARGED** for the tracked instruments; **re-created one step to the left** for the untracked ones → MAJOR-2 |
| M-2 | registered table ≠ what the instrument computes | **DISCHARGED**, verified by running the shipped fit on the reviewer's own rows |
| M-3 | "steps 1-4 admit exactly one table" is false | withdrawn, then **re-created one step to the left** as "the only normalisation that holds the balance" → MAJOR-1 |
| M-4 | diagnostics are scale statistics | **DISCHARGED**; two of six replacement anchors misreported → m-2 |
| M-5 | J's play-change unreceipted; the stride is the scope | **DISCHARGED** |
| M-6 | D-623's clause forbids J | confronted, **not soundly discharged** → MAJOR-3 |
| m-1 | `round_to_schema` clamps silently | clamp deleted **on a theorem that is false** → m-1 |
| m-2 | guard tests degeneracy not one-sidedness | **DISCHARGED** |
| m-3 | optimality check vacuous | **DISCHARGED** — deleted; the grid oracle's optimum has a bound BINDING |
| m-4 | "exact constrained minimiser" false for B and C | **DISCHARGED for B and C**; still false for H → m-3 |
| m-5 | rounding table omits B and C | **DISCHARGED** |
| m-6 | dominance check never reported | **DISCHARGED** |
| m-7 | "90 s" mismarked | **DISCHARGED** |
| m-8 | D-626's distances | **DISCHARGED** — 5 307 `mate_in` at {3: 2 428, 5: 2 737, 7: 142} plus 32 `mated_in` at {4: 3, 6: 29} |
| m-9 | D-483: tables cited from no artifact by digest | **NOT DISCHARGED — re-created one step to the left** → MAJOR-2 |
| m-10 | governing revision was the parent | **DISCHARGED** |
| Q-1..Q-4 | | all answered |

## MAJOR findings

### MAJOR-1. §5's ground is not sound, and §4's own adjacent column refutes it. Round 2's M-3, one step to the left.

§5 rests the selection on *"`w3` is the coordinate ADJACENT to the pinned tactical block, so pinning
it at the committed value is the only normalisation of the family that leaves the quiet-to-tactical
ratio exactly where the committed table has it… J-vs-committed is a contrast in the shape the corpus
determined **and in nothing else**."*

**(a) The uniqueness holds only of the ratio the sentence stipulates, and the rival was dropped
between revisions.** Round 2's M-3 listed three alternatives; revision 3's §5 table carries two and
omits `Σ = 74`. Pinned INSIDE the intercept solve it gives `[4.4076, 31.7903, 37.8021]`, c = +301.82
→ **`[4, 32, 38, 300, 1500]`**, holding `Σ(quiet)/w4` at **0.2467 — exactly the committed value** —
where J moves it to 0.3300. Schema-feasible, a minimiser and not a rescale, and a materially
different engine (57 % play-change).

**(b) "In nothing else" is false, and §4 prints the refutation one column right.** The filtered rows
are silent about `w4` and `w5` entirely, hence about EVERY quiet-to-tactical exchange rate:

| | `w1/w4` | `w2/w4` | `w3/w4` | `Σ/w4` |
|---|---|---|---|---|
| committed | 0.0067 | 0.0400 | **0.2000** | 0.2467 |
| **J** | 0.0167 (**×2.5**) | 0.1133 (**×2.8**) | **0.2000** | 0.3300 (**+34 %**) |

§4's own dominance column says it: committed `300 > 74`, J `300 > 99`. Applied consistently the
stated principle selects **nothing** — only the committed table holds all three.

**(c) The family is not a family of normalisations.** Pinning inside the solve is a different
constrained estimator, which is what makes J a minimiser rather than a rescale — and what stops it
being a normalisation. Its shape is not "the shape the corpus determined" but the shape conditional
on `w3 = 60`; the freely determined shape is `[4.9404, 32.8870, 66.9980]`.

**Why this blocks selection.** Not the choice of J — round 2 said "if forced to select, still J",
and §6 already records the pin as a limit. What fails is what the registered run may CONCLUDE: §5
tells a reader an h1 is attributable to shape and nothing else, and the arithmetic says it is
equally attributable to a 2.5× move in `w1/w4`, a 2.8× move in `w2/w4`, or a 34 % move in the
aggregate. A registration whose statement of what the contrast varies is contradicted by its own
table licenses a wrong conclusion.

### MAJOR-2. §7's named instruments cannot produce §4's and §5's tables — one aborts, the other computes the WITHDRAWN table. Third consecutive revision of the same class.

Run against `fit.py` at `0220d83`: `measure8.py` dies with `IndexError: tuple index out of range`
(`constrained_min` now returns a pair), and `measure9.py` dies with the new `FitError` while
computing `rescale_to(noint, 60.0)` — **the rescale that produces the withdrawn `[4, 29, 60]`**,
precisely the construction round 2's M-2 convicted. Nothing in the tree produces §5's three tables.
And `RECEIPT.sha256` verifies 12 of 12 files, every one a script or probe output: there is **no
`measure8`/`measure9` output in it**, so "whose outputs are receipted there" is false against the
receipt's own contents. Revision 1 pointed at a §7 that carried nothing; revision 2 at a commit that
did not exist; revision 3 at instruments that exist, are unversioned, and cannot run.

### MAJOR-3. D-627's deletion is not sound under D-424, and its sharper half rests on an option set that omits the row defeating it.

Verified first: D-623 does carry the clause outside the operator's quoted R7, and D-627 quotes R7
accurately, so it is amending the session's own gloss exactly as it says.

**(a)** D-424 deletes a distinction where *"both sides license the same conclusion"*. D-627 itself
records that the clause *"FORBIDS the option the same session then selected"* — which is a statement
that it changes what may be concluded. Its operative predicate, *"re-asserts a committed quantity"*,
cleanly separates A and the free solve from B, F, H and J.

**(b)** R7's own adjective — *"the SPRT on **data-derived** quiet weights"* — survives the deletion
unaddressed. If "data-derived" is empty here, R7 does not describe the registered run; if it has
content, the deleted clause was operationalising the operator's own word.

**(c) The sharper half is false once the missing row is on the matrix.** D-627 argues the clause
would have selected A, whose gain is offset absorption. Measured on the validation rows:

| | table | bias | resid var | quiet ρ | play-change |
|---|---|---|---|---|---|
| committed | `[2,12,60,…]` | −248.3 | 663 078 | 0.3873 | — |
| A | `[1,19,20,…]` | −189.7 | 668 947 | 0.3850 | 49.6 % |
| **free intercept solve** | **`[5,33,67,…]`** | **−365.8** | 636 034 | **0.4045** | 44.5 % |
| J | `[5,34,60,…]` | −356.4 | 635 707 | 0.4015 | 48.5 % |

`[5, 33, 67, 300, 1500]` is data-derived in the clause's sense, is the exact minimiser of the model
design §4 registers, imports no committed number, and does **not** absorb the offset. The clause
would have selected it, not A.

## MINOR findings

- **m-1. The clamp was deleted on a FALSE theorem, pinned by a test that cannot see the
  counterexample.** Python rounds half to even: `[1.5, 2.5, 3.5]` is input-feasible and rounds to
  `[2, 2, 4]`; `[3.5, 4.5, 60.0]` → `[4, 4, 60]`. The 2 000-draw test increments by multiples of
  1/7, so a `.5` tie is unreachable by construction — the vacuity round 2's m-3 convicted, one
  function to the left. It cannot change the registered answer and the loader would refuse a flat
  table loudly, so it is not a correctness blocker; it is a false invariant asserted in a governing
  design document.
- **m-2. Two of §5's six diagnostic anchors are not rescales.** "×0.05" is the table `[1, 2, 3]`,
  supplying both `719 668` and the endpoint `0.1236`; the genuine ×0.05 rescale gives `728 346` and
  ALL-ρ `0.1555`. The bullet's conclusion survives on the ×1.75 and ×3.0 anchors, which reproduce.
- **m-3. §4's header is still false for H**, which §5 itself calls a rescale-of-nothing.
- **m-4. §4's H quiet-ρ is 0.3867, not 0.3868** — the document's own `measure8.py` prints 0.3867.
- **m-5. The option set omits the unpinned intercept solve** `[5, 33, 67, 300, 1500]`. Design §4
  argues against it, but an argument in the design is not a matrix row with a cost and a failure
  mode — and §6 concedes the symmetric charge against J.
- **m-6. §5's alternative tables silently disagree with the report they discharge**: round 2 gave
  `w1 = 2 → [2, 13, 27]` (a rescale), revision 3 gives `[2, 35, 64]` (a pin). Both right for their
  own construction; the change is not stated.
- **m-7. The dry run's registered "lean" records nothing** — `DRYRUN.md`'s lean section is an EMPTY
  fenced block, and the receipt digests that emptiness. Everything else in it verifies.
- **m-8. "quiet ρ is flat across every rescale" is a theorem stated as a measurement** (Spearman is
  invariant under a positive rescale and nothing saturates).

## QUESTIONS

- **Q-1.** Why is `w3/w4` "the balance" rather than `Σ(quiet)/w4`? If the answer is the exchange
  rate at the split's own boundary, say that — "adjacent" asserts position, not relevance.
- **Q-2.** `w4 = 300` is an untuned Stage-0 number. Holding a ratio to it is a CONTROL, not a
  calibration. Does §5 mean to claim more than control?
- **Q-3.** D-627 landed in the same commit as the matrix whose ground it installs, so this reviewer
  was asked to attack a ground already in the append-only log — round 1's own observation about
  D-622, D-617's class. Is that the intended order?
- **Q-4.** If `[5, 33, 67, 300, 1500]` is genuinely excluded, what excludes it?

## What the reviewer attacked and it SURVIVED

1. **Every population count, sign count, solve and the blockability probe** reproduce at a scope the
   document never took — a different data path, a different window algorithm, exact rationals, an
   independent minimum-hitting-set. 29 401 / 0 / 15 135 / 14 266 / **0 needing ≥3** is exact, and so
   is 5 307 + 32.
2. **Round 2's M-2 is completely discharged**: the shipped fit on the reviewer's own rows prints
   `[5, 34, 60, 300, 1500]`, `c = +355.00` against an exact `+355.00443`.
3. **J is not a self-match**: 48.50 % at a seeded uniform draw over all eval rows against the
   registered 48.0 %; H 64.00 % against 65.7 %.
4. **§2's coextension survives a check it did not run**: both predicates keep the identical 45 271
   rows, because **0 of 74 672** eval rows have both sides holding a 4+ window.
5. **§3(a) reproduces** — `41ef5496…` in all sixteen tranches' `report.txt` AND `replay.txt`.
6. **§3(b)'s withdrawal and decomposition are right to the digit** in exact rationals.
7. **The tempo term's inertness is sound as MECHANISM, not only as measurement**: with the committed
   config's knobs all zero and a full root window, adding a mover-relative constant to every leaf
   makes the whole negamax recursion equivariant — every internal score and every window shifts by
   the same ±c — which is why round 2 measured identical node counts in all 448 records.
   `MATE_THRESHOLD 29 000` against `EVAL_MAX 16 000` closes the only remaining route.
8. **The grid oracle is a real oracle**: its optimum sits on a BINDING bound (`w1 = 1.000`, slack
   2e-15), so it exercises the bounds-as-members fix rather than passing interior.
9. **The sign-variation guard is not vacuous** and tests the property D-621 is about.
10. **The receipt is honest as far as it goes** — 12 of 12 verify, and `PLAYCHANGE.txt` carries the
    command, the draw and the withdrawn table beside the surviving one.
11. **Option D's kill is sound** — the gradient is exactly zero, not approximately.

## VERDICT

**FAIL.** The selection may not proceed on J as §5 argues it.

**What blocks it**: §5 stakes the recommendation on one sentence that is false against measurement
the document itself prints. J moves `w1/w4` by 2.5×, `w2/w4` by 2.8× and `Σ/w4` by 34 %; the
filtered rows are silent about all four ratios, not one; and a rival pin holding the aggregate at
exactly the committed value was in round 2's own list and is absent from revision 3's table. Round
2's M-3 re-created one step to the left, and not cosmetically: it is the statement of what the
registered SPRT varies.

**What the reviewer is NOT saying**: that J is the wrong table. Everything the corpus can say about
it reproduces exactly, the shipped instrument computes it, and it is not a self-match. Round 2's
judgement — "if forced to select, still J" — the reviewer would repeat. **Option S is still not the
answer.**

**The shortest route that is a deletion and not a fourth argument**, if the operator prefers ruling
to splitting: strike §5's uniqueness — the word "only", the "in nothing else", and the claim that
`w3/w4` is *the* balance — and let the pin stand as what §6 and design §10 already call it twice,
*"a choice no evidence in this corpus can make"*, registered as a limit on what an h1 may be
attributed to. That deletion is D-424's own remedy, it changes no number, it needs no new
measurement, and it leaves J selected on the ground that survives: the pin has to go somewhere,
`w3` is the entry the split's own boundary runs through, and which entry it is remains unevidenced.
