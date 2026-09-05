# DECISION-RED-TEAM on `matrix_wp22_cap_decision.md`, revision 1.

Fresh context. Target: `docs/experiments/matrix_wp22_cap_decision.md` revision 1,
which recommends **option E**. Tree at `84c1af269646e177d0125452f8124c17adfac40b`,
matching HEAD at dispatch.

**VERDICT IN ONE LINE.** Every MEASURED figure in the matrix reproduces — all of
them, to the digit. **E is not killed by any attack on its own arithmetic.** It is
wounded in three places, and it stands on a premise that a measurement taken here
falsifies: **the floor E is built to clear does not deliver the test D-537 asks
for at any cap, and it delivers it worst at the cap E makes primary.** I recommend
a sixth option the matrix does not list.

---

## (a) Re-derivation ledger

All parses use the archived arms directly; `d` is the whitespace key/value split
`tally.py` uses. Reproduction script bodies are in this section's commands.

### A.1 — the matrix's own MEASURED figures

| # | Claim (matrix §) | Value claimed | Value I get | Reproduced |
|---|---|---|---|---|
| 1 | keys/position (F1) | 0.86 / 0.17 / 0.19 | 86 / 17 / 19 distinct `att_proved=true` keys over 100 positions | **YES** |
| 2 | positions for 28 (F1) | 33 / 165 / 147 | 32.6 / 164.7 / 147.4, ceil | **YES** |
| 3 | wall at 3x margin (F1) | 5.6 / 33.2 / 35.4 min | 337 s / 1995 s / 2122 s from 341/403/480 s per 100 | **YES** |
| 4 | calibration cost (F1 quote) | 1 h 40 m | (341+403+480)/100 × 500 = 6120 s = 102 min | **YES** |
| 5 | proving roots (F2) | 8 / 8 / 9 | 8 / 8 / 9 | **YES** |
| 6 | union (F2) | 12 | 12 | **YES** |
| 7 | 2048-only vs both larger (F2) | [7, 98] | [7, 98] | **YES** |
| 8 | 16384's cell "roots it **alone** finds" (F2) | [16, 52, 74, 91] | **[74, 91]** — 8192 also proves 16 and 52 | **NO — see B5** |
| 9 | clustering (F3) | 10.75 vs 2.1 | 86/8 = 10.75, 17/8 = 2.125, 19/9 = 2.111 | **YES** |
| 10 | `w/s` (F3) | 0.2522 / 0.0422 / 0.0396 | 86/341, 17/403, 19/480 | **YES** |
| 11 | truncation (F4) | 71.2 / 48.7 / 38.6 % | 828/1163, 190/390, 103/267 = 71.195 / 48.718 / 38.577 % | **YES** |
| 12 | option D/E wall (§2) | 5.6 / 11 / 27 min | 334 s / 682 s / 1642 s | **YES** |
| 13 | nominal test (ledger §4) | size 0.040184, power 0.962225 | identical to six places | **YES** |

The proving-root sets, since the matrix prints only their sizes:

```
2048  [4, 7, 60, 65, 67, 72, 82, 98]
8192  [4, 16, 52, 60, 65, 67, 72, 82]
16384 [4, 16, 52, 60, 67, 72, 74, 82, 91]
union [4, 7, 16, 52, 60, 65, 67, 72, 74, 82, 91, 98]  (12)
```

E's own pair `{2048, 16384}` reaches the full union of all three arms; 8192 adds
nothing to it. That is a fact **in E's favour** the matrix does not claim.

### A.2 — is `entry` comparable across arms? (dispatch question 1)

**YES, and this sub-attack fails outright.** All three arms name the same fixture
(`--fixture .../d3.txt`, 100 lines) in their own argv line, and each emits exactly
100 `entry` lines. I re-ran the instrument today on the first 20 fixture lines:

```
./target/release/examples/trigger_census --fixture f20.txt --nodes 50000 --cap 2048 --gate on
```

236 rows, and every row is **byte-identical to the archived `d3_c2048.txt` rows
for entries 0-19**. A repeat run is byte-identical to itself (hard rule 4 holds).
So `entry k` denotes fixture line `k+1` in every arm, and root identity is
comparable. The matrix is entitled to compare the sets.

### A.3 — does the cost model reproduce on this box today?

| arm | archive s/position | measured today (20 positions) |
|---|---|---|
| cap 2048, nodes 50k | 3.41 | 71 s → **3.55** |
| cap 16384, nodes 50k | 4.80 | 84 s → **4.20** |

Within 4 % and 13 % on a non-random 20-position slice. **The matrix's own worry
that `w/s`'s denominator was measured on a loaded box does not bite** — the wall
clock reproduces, and no option's cost claim moves.

### A.4 — new measurements this red team took

| # | Quantity | Result |
|---|---|---|
| 14 | keys-per-root profile at 2048 | **[35, 33, 11, 3, 1, 1, 1, 1]** — two roots supply 68 of 86 keys (79 %) |
| 15 | distinct full-column signatures among win keys | 26 / 14 / 15 (2048 / 8192 / 16384) |
| 16 | distinct signatures over the **detector's own feature columns** | **17 / 11 / 12** |
| 17 | signatures shared across two roots | **0** at both caps (`f1 = S_obs`, `f2 = 0`) — clustering is entirely *within* root |
| 18 | Chao1 richness of win signatures | 153 (2048) / 78 (16384) — the signature space is **not** exhausted |
| 19 | P(census of n positions yields < 28 keys), 2048 | n=98 → **0.111**; n=200 → 0.005; n=300 → 0.0003 |
| 20 | same, 16384 | n=98 → 0.861; **n=200 → 0.191**; n=300 → 0.012 |
| 21 | bootstrap CI at n=500, cap 2048 | [233, 660] — the rev-4 review's independent bootstrap gave **[231, 657]**; reproduces |
| 22 | `search_nodes` at the six discordant roots, 2048 vs 16384 | moves by **≤ 5 %** while firings collapse 7-21x |
| 23 | cap 16384 at `--nodes 200000` on roots {7, 65, 98} | **65 and 98 now prove** (12 and 16 win rows); 7 still proves nothing |
| 24 | detector-signature rate per position | **0.17 / 0.11 / 0.12** — 2048's lead is 1.4x, not the 4.5x its key rate shows |
| 25 | cost of cap 16384 at nodes 200k | 286 s / 20 positions = **14.3 s/position** |

Bootstraps are 20 000 replicates, positions drawn i.i.d. from the 100 observed.
**Marked limit**: that empirical distribution's tail rests on three heavy
positions, so every interval above is a *lower* bound on true spread.

---

## (b) Attacks

### B1 — THE FLOOR OF 28 DOES NOT BUY THE TEST IT WAS SIZED FOR, AND BUYS IT WORST AT E'S PRIMARY CAP. **WOUNDS E** (and kills a premise A, D and E all share)

This is the dispatch's question 3 and it is the strongest thing in this report.

`overnight2_ledger.md` §4 fixes n = 28, c = 21 from a **one-sample binomial**
power calculation: `P(X>=21 | p0=8/14) = 0.0402`, `P(X>=21 | p1=12/14) = 0.9622`.
A binomial assumes 28 **independent** trials. The 28 units are win-proving
firings on disjoint positions, and D-570 fixes the identity as the canonical key.

**The trials are not independent, and this is provable rather than assumed.**
`matrix_stage3_detector.md` §5.4 defines the alternative `p1 = 0.857` as *"what a
score fitted with full knowledge of which column-classes hold wins could
reach"* — **the bound OVER THE CENSUS COLUMNS**. Every candidate and every score
in `tools/stage3_census_rank.py` and `tools/stage3_allocator_bound.py` is a pure
function of those columns (`opp_hot`, `mover_hot`, `mover_l3`, `cover`, `covers`,
`turns`). So two census rows agreeing on those columns receive the **same score
and the same verdict from any hypothesis the test can entertain**: within a
column signature the correlation is not estimated at ~1, it *is* 1, by the
alternative's own definition.

MEASURED on the arms in hand:

| cap | win keys | roots | full-column sigs | **detector-feature sigs** |
|---|---|---|---|---|
| 2048 | 86 | 8 | 26 | **17** |
| 8192 | 17 | 8 | 14 | 11 |
| 16384 | 19 | 9 | 15 | **12** |

Simulating a census that stops at exactly 28 distinct keys, then running the
registered test with perfect within-signature correlation:

| grouping | cap | distinct groups among the 28 (median, 95 % CI) | **actual size** (registered 0.05) | **actual power** (registered 0.95) |
|---|---|---|---|---|
| full columns | 2048 | 9 [7, 15] | **0.263** | 0.791 |
| full columns | 16384 | 12 [8, 15] | 0.171 | 0.852 |
| **detector features** | **2048** | **6 [4, 11]** | **0.479** | 0.837 |
| **detector features** | 16384 | 10 [6, 12] | 0.222 | 0.820 |

**A detector round 3 opened on 28 keys censused at cap 2048 would declare
significance against a null it should reject 5 % of the time at a measured
48 %.** The floor is met in the letter D-570 fixes and defeated in the
"power-style rule" D-537 demands in its own words.

The matrix **found this fact and did not follow it home**. F3 says *"the quantity
the registered rule maximises is clustering, not coverage… the rule reads the
inflated quantity"* — and uses it only to discredit `w/s` as a *selection*
statistic. Then §3 makes **the same inflated quantity, at the arm that maximises
the inflation, the primary count against D-537's floor**. F3 is an argument
against cap 2048 for the count, and the matrix deploys it as an argument against
the calibration only.

Two honest limits. (i) Distinct signatures are **necessary, not sufficient**, for
independence — two different signatures can still be correlated, so 28 signatures
does not restore the nominal 0.05, it only removes the ρ = 1 groups, which is the
part that is provable. (ii) My between-signature ρ = 0 assumption is generous, so
the actual sizes above are **lower bounds on the inflation**.

**Why this is not overrule-able prose (D-424).** It names a way the instrument
produces a wrong answer: a registered α of 0.05 realised at 0.48. It is fixed,
not deleted.

### B2 — F2's COMPLEMENTARITY IS TWO-THIRDS A NODE-BUDGET ARTIFACT, NOT A CAP PROPERTY. **WOUNDS E**

F2 explains the small-cap-only roots as: *"at a large cap the 50 000-node budget
is consumed by fewer, deeper calls and the search never reaches positions the
small cap fires at."* That sentence is **asserted, unmarked, and measurable in
seconds from the files the matrix already cites** (D-291's own finding class).

MEASURED at the six discordant roots — `search_nodes` barely moves while firings
collapse:

```
root  7:  search_nodes 1945 (2048) vs 1863 (16384),  firings 15 vs 2
root 98:  search_nodes 7868 (2048) vs 7796 (16384),  firings 42 vs 2
```

≤ 5 % on the tree size, 7-21x on the firings. **The search reaches a
near-identical amount of tree; what changes is how many firings get a solver call
before the budget is gone.** So I tested the implication directly — cap 16384 at
`--nodes 200000` on the roots 2048 proves and 16384 does not:

| root | 16384 @ 50k | **16384 @ 200k** |
|---|---|---|
| 98 | 0 win rows | **16 win rows** |
| 65 | 0 win rows | **12 win rows** |
| 7 | 0 win rows | 0 win rows |

(And note: against E's actual pair the 2048-only set is **{7, 65, 98}**, three
roots, not the [7, 98] the matrix's table shows — 65 is proved by 8192, so it
falls out of the "neither larger cap" column while remaining a 2048-vs-16384
discordance.)

**Two of the three roots that justify E's second arm are recovered by turning one
other knob on a single instrument.** F2's *"no cap dominates"* survives — root 7
is a genuine structural foreclosure in the small cap's favour, mirroring D-530's
in the large cap's — but its **magnitude collapses to one root in a hundred
positions**, 95 % Poisson interval roughly [0.03, 5.6] per 100. E buys its second
arm on a fact that is real and one-third the size the matrix prices it at.

### B3 — E's SECOND ARM AT n = 200 DOES NOT DELIVER A SECOND ATTRIBUTABLE COUNT. **WOUNDS E**

E's defensible purpose (see B4) is a second single-instrument count at the other
cap. MEASURED, at cap 16384 the census clears 28 keys with probability:

```
n = 200 -> 0.809      n = 300 -> 0.988      n = 500 -> 1.000
```

**E as recommended — n = 200 — leaves a 19 % chance its second arm produces a
count that clears nothing.** The matrix sizes both arms at one n because it is
thinking of the second arm as coverage, where any yield is a gain. Under the only
reading that makes the second arm do work, n = 200 is undersized by ~50 %.

Related, smaller: F1's headline *"the floor is cleared in 5.6 minutes"* is the
3x-margin figure at n ≈ 98, where MEASURED **P(fewer than 28 keys) = 0.111**. The
recommendation registers n = 200 (P = 0.005), so E is safe; the *headline* is not.
This is the dispatch's question 2, and the answer is that a clustered rate does
extrapolate in the **mean** (the bootstrap means reproduce the matrix's linear
projections exactly, and the rev-4 review's independent bootstrap agrees) while
its **spread** is enormous: positions needed to reach 28 keys has a 95 % interval
of **[2, 154]** around a median of 35.

### B4 — THE "UNION" IS THE ONE FRAMING D-537 CANNOT USE, AND E'S BEST ARGUMENT IS NOT THE ONE IT MAKES. **WOUNDS E's framing; the option survives**

Dispatch question 4. E is **not forbidden** by D-537 — that ADR fixes a counted
quantity and a power-style rule, not an instrument count, and D-563 confirms
census records are excluded from the label corpus by construction, so two arms
contaminate nothing. E's own guard (primary count single-instrument, union never
summed) is coherent as stated.

But ask what the union *licenses*. It cannot be the floor's count. It cannot be
round 3's test sample either: a row proved at 16384 does not fire at 2048 at all,
and the shipped engine runs **one** cap — a detector tested on a two-cap union is
fitted to a tree that does not exist, which is precisely D-538's third ground for
the D-471 flip, transposed from the eval to the cap. Under CLAUDE.md's own test —
*"whether the disputed claim changes what anyone may conclude"* — the union
changes nothing anyone may conclude, and it is exactly the number a successor
will misread. E names that risk and then prints the number anyway.

**E's real purchase is the one it never states**: two *separately attributable,
single-instrument* counts, one per cap, so that whichever cap the eventual
production config takes, round 3 has a sample it can use. That is strictly better
than "coverage evidence", it needs no union, and — see B3 — it needs n ≈ 300.

I also note that E does not so much *retire* the calibration as **perform it under
another name**: D-563 says what would settle the cap is *"a DEDICATED calibration
run at caps 2048 and 16384, counting win-direction proofs on distinct keys…
whose records are excluded from the corpus by construction"*. That is E, at
n = 200 instead of n = 500. This is a point in E's favour that the matrix's
"RETIRE" framing hides from itself.

### B5 — A WRONG TABLE CELL. **MINOR, wounds nothing**

F2's column is headed *"roots it alone finds"* and 16384's cell reads
[16, 52, 74, 91]. Roots 16 and 52 are also proved by 8192. Truly alone, 16384
finds **[74, 91]**. The cell's annotation (*"2048 proves none of these"*) is
correct and is what the recommendation actually leans on, so the conclusion is
undamaged — but the header does not say what the cell means.

### B6 — "12 vs 9 IS NOISE". **FAILS — this attack does not land**

Dispatch question 1's last limb. It does not land, and I say so plainly.
Complementarity here is not a sampling question: roots 7, 65 and 98 **are**
proved at 2048 and **are not** at 16384, on the same 100 positions, with a
deterministic instrument I re-ran and confirmed byte-identical. Nesting is
falsified by demonstration, not tested. A McNemar exact on the discordant pairs
(b = 2, c = 4 against both larger caps) gives p = 0.69, which is the correct
reading of *"no cap dominates"* — the arms are not distinguishable in **which**
is better, which is what the matrix claims. What is imprecise is the **magnitude**
of the union's advantage, and B2 shows the magnitude is smaller than priced.

### B7 — CITATION. **MINOR, not a finding**

§3 attributes the proportionality quote to CLAUDE.md; it is at `docs/process.md:60`
(D-228 makes the same loose attribution). CLAUDE.md's Process section makes
process.md bind "exactly as this file would", so the citation is loose, not false.

### B8 — WHAT I ATTACKED AND IT SURVIVED, SAID EXPLICITLY

- **Every MEASURED number in the matrix reproduces**, including all four facts'
  headline figures and all three options' wall-clock estimates. I found no
  fabricated or mislabelled measurement; every ESTIMATED figure is a division or
  multiplication of a MEASURED one and is labelled as such.
- **F1 kills B and C, and it is right to.** A 1 h 40 m instrument selecting
  between runs of 6-35 minutes fails the proportionality rule on its face, and the
  rev-4 review's bootstrap (2048 in 20 000 of 20 000; I reproduce its interval)
  means the confirmation's information content really is near zero. **B and C stay
  dead**, and nothing in B1-B4 revives them — B1 in particular is an argument
  against the *floor's currency*, not for spending 100 minutes choosing a cap.
- **A stays dead.** F2's root 7 is a real, measured foreclosure.
- **E is not forbidden by D-537** (B4), and its arithmetic is sound.

---

## (c) THE STRONGEST SURVIVING ATTACK ON OPTION E

Quotable verbatim into the ADR line:

> **E takes its primary D-537 count at cap 2048 — the arm its own F3 identifies
> as the most clustering-inflated — and the inflation is not cosmetic: MEASURED,
> the 86 win-proving keys at cap 2048 come from 8 roots, two of which supply 68 of
> them, and collapse to 17 distinct values of the columns every candidate detector
> score reads. Since `p1 = 12/14` is defined as the bound OVER THOSE COLUMNS, rows
> sharing a signature return one verdict to every hypothesis the test can
> entertain, so a sample of exactly 28 distinct keys holds a median of 6
> distinguishable trials and the registered one-sample binomial runs at an actual
> size of 0.48 against its registered 0.05 and a power of 0.84 against 0.95. The
> floor is met in the letter D-570 fixes and defeated in the power-style rule
> D-537 demands. The same measurement prices E's second arm lower than E does:
> cap 16384 at `--nodes 200000` recovers two of the three roots (65, 98) that cap
> 2048 alone proves at `--nodes 50000`, so two-thirds of the complementarity the
> second arm is bought for is a node-budget artifact rather than a cap property,
> and only root 7 survives as a structural small-cap foreclosure.**

---

## (d) My recommendation — it differs

**Adopt option F, which the matrix does not list.**

> **F — RETIRE the calibration (as E and D do). Register ONE census: cap 2048,
> `--nodes 50000`, single instrument. Size n against the count of distinct
> win-proving DETECTOR-FEATURE SIGNATURES reaching 28, and report the distinct-key
> count beside it as D-537's literal figure.**

**Cost.** MEASURED signature rate 0.17/position at cap 2048 ⇒ ESTIMATED 165
positions for 28; register **n = 350** for margin ⇒ ESTIMATED **20 minutes**
(350 × 3.41 s). That is **less than E's 27 minutes**, on one instrument, with no
union to misread. The distinct-key count at n = 350 is ESTIMATED ~300, so D-537's
literal floor is cleared with two orders of headroom and is not the binding
quantity — which is F1's insight, kept.

**Why F is licensed without an operator ruling.** D-537 says a successor *"may
register a larger one with grounds and may never register a smaller one"*.
Requiring 28 **signature-distinct** win-proving keys is strictly larger than
requiring 28 keys (17 ≤ 86 on the arm in hand) — it is a tightening in the same
currency, with the grounds measured in B1. The unit change still deserves its own
ADR line, since D-570 fixed the identity and this adds a second, coarser tally
beside it rather than replacing it.

**Why F beats E on E's own terms.** Cheaper; one instrument; no union; and it
targets the quantity that decides whether round 3 may open. MEASURED (#24), the
signature rate per position is 0.17 / 0.11 / 0.12 across the three caps — the cap
moves the quantity that matters by **1.4x**, against the 4.5x it moves the
inflated key count. **On the currency that governs the downstream consumer, the
cap barely binds at all**, which is a stronger and better-grounded version of the
matrix's own F1 than the wall-clock argument F1 actually makes.

**If the operator prefers E anyway**, two amendments are owed, and both are cheap:

1. **n = 300, not 200** — otherwise the second arm has a MEASURED 19 % chance of
   producing a count that clears nothing (B3).
2. **Report two independent single-instrument counts, never a union** (B4). The
   union licenses no conclusion and is the number a successor will misread; two
   attributable per-cap counts are E's actual purchase.

**And a cheaper coverage lever than a second cap, if coverage is wanted**:
MEASURED (#23), `--nodes` recovers two of the three discordant roots at one cap.
The matrix's option field never considers varying the node budget; on this
evidence it dominates varying the cap for coverage purposes, at a MEASURED
14.3 s/position at cap 16384 / 200k.

**What would flip my recommendation.** If the operator rules that D-537's counted
unit is the canonical key and may not be supplemented — i.e. that B1 is a defect
in the *floor* to be fixed when round 3 registers, not in the *census* that feeds
it — then F's tally is premature and **E-with-both-amendments is the right
option**, since B1 then binds round 3's own pre-registration rather than this one.
That is an operator call about where the fix lands, not about whether B1 is true.
