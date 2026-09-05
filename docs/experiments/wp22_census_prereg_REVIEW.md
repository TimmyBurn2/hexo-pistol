# WP-2.2 §B census pre-registration — fresh-context REVIEW, revision 2. Verdict FAIL.

**Reviewed**: `docs/experiments/wp22_census_prereg.md`, revision 2 (its first
line states the revision), at named revision `8153369` on `dev`.

**Does it match HEAD?** **No — HEAD moved during this review.** HEAD is now
`ce6cfd1` (`docs(wp22): the final summary …`), which adds
`docs/experiments/wp22_FINAL_SUMMARY.md` and nothing else:
`git diff --stat 8153369..ce6cfd1` is one file, 279 insertions. The document
under review is **byte-identical at `8153369` and at `ce6cfd1`**, and
`git diff --stat 84c1af2..ce6cfd1 -- crates tools configs Cargo.toml Cargo.lock`
is **empty**, so every instrument this review exercised is the same code at the
document's stated governing revision and at HEAD. The finding is recorded
anyway, because D-617 is the record of exactly this: a session editing while a
gate reads the working tree.

**VERDICT: FAIL.** Six MAJOR, nine MINOR, three QUESTIONs.

**The one-line reason.** I ran the registered command on a fresh, disjoint,
500-position sample of the same corpus. **The registered currency —
distinguishable trials — reaches 12 at n=500 and its own asymptote is 15–18.
The floor of 28 is not reachable at n=800, and not at n=89 205 either.** The
saturation the document checked for and reported absent is present and large; it
was invisible at n=100 because the check was a linear extrapolation of a
coupon-collector quantity. Compounding it, the trial count the document sizes on
is computed over the **wrong partition**: `p1 = 12/14` is defined over
`stage3_allocator_bound.py`'s nine-column classes, which give **26** at cap 2048
and cross 28 at **n=268** — the document's own justification, applied to the
partition it actually cites, yields a criterion that clears comfortably.

---

## 0. What I ran, and what it consumed

Everything below was run at `ce6cfd1` with the prebuilt
`target/release/examples/{trigger_census,fixture_key_full}`. No `cargo` was run
in the live tree.

**The load-bearing new measurement**: the registered command,

```
trigger_census --fixture <calibration slice> --nodes 50000 --cap 2048 --gate on
```

over the **calibration slice (rows 100..599, 500 positions)** — the sample
**D-618 retired UNRUN**, so nothing governed was consumed. **The governed census
slice (rows 600 onward) is untouched by this review.** A successor should know
that the calibration slice now has a cap-2048 / 50 000-node census over it;
`artifacts/` was not written, the output sits in this session's scratchpad.

**Cost**: 1 656 s wall for 500 positions = **3.312 s/position**, which
independently reproduces the registered 3.41 s/position within 3 %.

**Marked limit** (`docs/process.md`, "Cost, replication, and the second
instrument"): this is a **REPLICATION**, not a second instrument — same binary,
same revision, same seat. That is the correct instrument for the claim it
falsifies, which is a claim about how a count grows with sample size, but it is
blind to anything the census instrument itself gets wrong.

---

## 1. Re-derivation ledger

Commands are mine, not the document's. Scripts written from the row format
rather than copied from `signatures.py`; the parser asserts all seventeen row
fields are present before it counts anything.

### 1.1 The sample (§3) — reproduces exactly

| claim | my command | result | ? |
|---|---|---|---|
| census slice = rows 600 onward, **89 205** positions | `python3 tools/texel/draw_census_samples.py census …` | 89 205 lines | **YES** |
| dry-run slice rows 0..99, calibration rows 100..599 | `draw_census_samples.py dryrun/calibration …` | 100 / 500 | **YES** |
| the drawn fixtures are the ones the pilot used | `diff` against `d3.txt`, `calib3.txt`, `census_all.txt` | **byte-identical, all three** | **YES** |
| manifest digest vs `arc3_ledger.md` §5 | `sha256sum artifacts/arc3r_sweep_deduped_manifest.txt` | `00f61780…` = ledger §5 line 62 | **YES** |
| §8's pre-run check, on the **whole** census fixture | `fixture_key_full rv_census.txt` vs `.expected_key_full` | **0 of 89 205 disagree, 2.3 s** | **YES** |

### 1.2 The §5 table and the collapse — every stated number reproduces

All on `artifacts/wp22_cap_dryrun_v2/d3_c2048.txt` (n=100, cap 2048).

| claim | result | ? |
|---|---|---|
| 86 win-proving keys | 86 (from 97 win-proving rows) | **YES** |
| from 8 roots | 8 — `[4, 7, 60, 65, 67, 72, 82, 98]` | **YES** |
| distributed `[35, 33, 11, 3, 1, 1, 1, 1]` | identical | **YES** |
| **10 distinguishable trials** against `stage3_census_rank.py`'s field | 10 | **YES** |
| "28 keys are reachable from ONE search tree" | top root supplies 35 ≥ 28 | **YES** |
| rates 0.86 / 0.08 / 0.10 per position | 0.86 / 0.08 / 0.10 | **YES** |
| `n` for 28 = 33 / 350 / 280 | arithmetic correct | **YES** (but see M3) |
| curve sat at 2 to sixty positions, 10 by a hundred | `10:2 20:2 … 60:2 70:7 80:8 90:9 100:10` | **YES** |
| trial rate 0.10 / 0.07 / 0.08, spread 1.4x | 0.10 / 0.07 / 0.08; 0.10/0.07 = 1.43x | **YES** |
| 3.41 s/position; 800 positions ≈ 45 min | 341 s/100 in the receipt; **3.31 s/pos measured fresh** | **YES** |
| `signatures.py`'s CANDIDATES faithful to `stage3_census_rank.py`'s | 13 of 14; the omitted one is `("incumbent", lambda r: True)`, a **constant** that cannot split any class — I verified the 14-predicate and 13-predicate partitions give **identical** counts 10 / 7 / 8 | **YES — faithful** |

### 1.3 §4, §7, §9 — reproduce

| claim | result | ? |
|---|---|---|
| cap 16384 at `--nodes 200000` recovers two of the three roots cap 2048 alone proves | roots **65** (12 win rows) and **98** (16) prove; root **7** proves 0. `rt/r65_200000.txt`, `rt/disc_16384_200000.txt` | **YES** |
| truncation 71.2 % at cap 2048 | 828 of 1 163 unproved invocations at the cap = 71.2 % | **YES** |
| symmetry fold's yield zero on the dry run | `distinct_key 1038 == distinct_key_pos 1038` | **YES** |
| D-537's floor: `p0=8/14`, `p1=12/14`, α=β=0.05, `n=28`, `c=21` | `overnight2_ledger.md` §4 as quoted | **YES** |

### 1.4 What does NOT reproduce

| claim | what I got | where |
|---|---|---|
| `p1 = 12/14` is defined in `matrix_stage3_detector.md` **§5.4** | §5.4 is "What §5.2 and §5.3 license" and contains no such figure. The definition is **§5.8**, line 613 ("bound over the columns", 0.857, 12 of 14) | m1 |
| §8: the dry-run criterion "passed **0 of 20**" | `wp22_cap_prereg.md` §9 and `artifacts/wp22_cap_dryrun_v2/RECEIPT.md` both record **0 of 100**; the "19 of 19" shift test is the 20-row one | m2 |
| "two proving rows every candidate scores identically are one trial **by the alternative's own definition**" | the alternative's own partition is `stage3_allocator_bound.py`'s nine `COLUMNS`, applied **atomically** in `knapsack_bound`. That partition gives **26** classes among the same 86 keys, not 10 | **M2** |
| "most of the apparent cap complementarity is a NODE-BUDGET artifact" | 2 of the 7 discordant roots are node artifacts; the other 4 (16, 52, 74, 91) are D-563's structural cap foreclosure and are unaffected by node budget | m3 |

### 1.5 THE NEW MEASUREMENT — the registered command on 500 fresh, disjoint positions

Calibration slice (rows 100..599), cap 2048, `--nodes 50000`, `--gate on`.
5 935 census rows, 117 win-proving rows.

| quantity | dry-run slice, n=100 (what §5 registers from) | **calibration slice, n=500 (mine)** | §5's linear model at n=500 |
|---|---|---|---|
| distinct win keys | 86 (0.86/pos) | **104** (0.21/pos) | 430 |
| distinct proving roots | 8 (0.08/pos) | **36** (0.072/pos) | 40 |
| **distinguishable trials (§5's currency)** | 10 (0.10/pos) | **12** (0.024/pos) | **50** |
| nine-column classes (`p1`'s own partition) | 26 (0.26/pos) | **62** (0.124/pos) | 130 |

**Cumulative trials over the 500** — this is the whole finding:

```
100:8  200:9  300:10  400:11  500:12
```

**New classes per proving root, in order** (36 proving roots):

```
1 1 1 1 1 1 2 1 0 0 0 0 1 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0
  the first 8 proving roots gave 9 classes; the last 28 gave 3.
```

**Union over the two disjoint samples (600 positions): 13 classes.**
Intersection 9; the 500-position sample found **3** classes the 100-position
sample had not.

**Where each count crosses 28, measured within the 500:**

| count | crosses 28 at |
|---|---|
| distinct win keys | **n = 216** |
| nine-column classes (`p1`'s partition) | **n = 268** |
| distinct proving roots | **n = 420** |
| **distinguishable trials (§5's currency)** | **never — 12 at n = 500** |

**Chao2 incidence extrapolation from my n=500 data, positions as sampling
units** (S_obs=12, Q1=5, Q2=2):

| target | trials, bias-corrected | trials, classic | §5's linear model |
|---|---|---|---|
| n = 800 (**the registered n**) | **14.0** | 14.4 | 19 |
| n = 2 000 | 15.3 | 17.7 | 48 |
| n = 89 205 (the whole slice) | **15.3** | **18.2** | 2 141 |

I recorded a prediction from the n=100 data **before** this run finished
(`extrap.py`): 22.5–30.1 trials at n=500. The measured answer, **12**, is below
even that conservative extrapolation.

---

## 2. MAJOR findings

### M1 — §5's sizing extrapolates a coupon-collector quantity linearly, and the measured answer is that the registered floor is UNREACHABLE, not merely at risk

**What is wrong.** §5 registers `n = 800` on a MEASURED rate of 0.10
trials/position and the claim that "the cumulative curve is still climbing over
the second fifty positions at both caps examined, so **no ceiling is visible**
and 28 is reachable." Both halves fail on more data from the same corpus and the
same instrument. On 500 fresh disjoint positions the count reaches **12**, the
per-position rate falls from 0.10 to **0.024**, the marginal contribution of a
new proving root falls from 1.25 classes to **3 classes across the last 28
roots**, and Chao2 puts the asymptote at **15.3–18.2 over the entire 89 205-position
slice**. The union of the two disjoint samples — 600 positions — is **13**.

**Why it matters.** The registered margin is not a margin. §5 says `n = 800` is
"well above the 280 the point estimate needs" and that "a sample sized at the
point estimate would be a coin toss"; measured, n=800 returns ~14 and no `n`
this corpus can supply returns 28. §5 also says "if the governed run's curve
flattens instead, that is the finding §7 reports and not a failure of the run" —
but that clause registers a contingency whose probability the document states as
low, when it is in fact the outcome. Spending 45 minutes to learn a number that
600 positions already fix is the proportionality failure CLAUDE.md asks a
registration to show on its own face, and it is the same failure D-618 charged
against the calibration it retired.

**Minimal fix.** Replace §5's linear table with the measurement above (or take
it again — it costs 28 minutes), and re-decide. Three coherent successors, and
the document must pick one: (a) adjudicate on the nine-column partition, where
28 crosses at n=268 and `n = 800` is a genuine margin (see M2); (b) keep the
candidate-field currency, state that its asymptote is 15–18, and take the
shortfall to the operator as an OVERRULE or a round-3 pre-registration question
rather than spending a run to observe it; (c) keep D-537's letter as the sole
adjudicated count and demote the trial figure to a reported diagnostic — which
is what §7 already does, and which M3 says the document must say out loud.

### M2 — the collapse is measured over the NULL's family and attributed to the ALTERNATIVE's; on the partition `p1` is actually defined over, the count is 26 and the criterion clears

**What is wrong.** §5 argues: *"A candidate detector is a boolean predicate over
the census columns, and `p1 = 12/14` is defined as a bound OVER THOSE COLUMNS
(`matrix_stage3_detector.md` §5.4), so two proving rows every candidate scores
identically are one trial by the alternative's own definition."* Two independent
errors:

1. **`p1` does not belong to a predicate.** `matrix_stage3_detector.md`'s
   section that defines 0.857 is **§5.8**, whose heading is *"ROW (e), THE
   ALLOCATOR — scored separately, **because it is not a predicate**"*. `p1` is
   the allocator's bound.
2. **The alternative's partition is not the candidate field.** The bound is
   produced by `tools/stage3_allocator_bound.py::knapsack_bound`, whose classes
   are `COLUMNS = ("turns", "mover_hot", "opp_hot", "mover_w1", "opp_w1",
   "mover_l3", "opp_l3", "cover", "covers")` and which treats each class as **one
   knapsack item** — so within a column-class the verdict is constant and the
   collapse argument is *sound, for that partition*. Measured on the same file,
   that partition gives **26** classes among the 86 win keys (0.26/position), and
   **62** on my 500 (crossing 28 at **n = 268**).

   The **10** comes from `stage3_census_rank.py`'s CANDIDATES — the family of
   *written orderings*, which is what `p0 = 8/14` is measured over
   (`overnight2_ledger.md` §4: *"the best WRITTEN ordering's win recall"*). The
   document uses the null's family and cites the alternative's definition for it.

**Why it matters.** It is not a labelling quibble: it is the difference between a
criterion that crosses 28 at n=268 and one that never crosses at all. The
sentence is the sole ground offered for the number the whole of §5 is built on,
and for D-618's "no operator ruling is needed". Choosing the coarsest available
partition is conservative for *sizing* — and I say so — but the moment 28 trials
is read as a threshold, the coarsest partition makes the threshold ~2.6x harder
than "the alternative's own definition" licenses.

**Minimal fix.** Fix the citation to §5.8, and either (a) adopt the nine-column
partition as the registered trial unit, naming `stage3_allocator_bound.py`'s
`COLUMNS` and `knapsack_bound`'s atomicity as the ground — this is the reading
the quoted sentence actually supports — or (b) keep the candidate-field count and
delete the "by the alternative's own definition" clause, replacing it with the
honest ground: *this is the partition the written-ordering field can distinguish,
it is coarser than the alternative's, and it is chosen because it is
conservative.* (b) is only viable together with M1's answer, because the
conservative choice is the one that cannot be met.

### M3 — the document never says which count licenses detector round 3, and §6, the section whose heading owns the question, does not answer it

**What is wrong.** §6 is headed *"What the three counts are for, and which one
licenses what."* Its body says nothing about licensing. Elsewhere the document
points two ways: §1 and §7 adjudicate on `w`, the distinct-key count
(*"`w` against the floor of **28**, and whether the floor is cleared"*), while §5
sizes the sample on trials against a target of 28 and calls 28 trials the thing
`n` must reach. §7's closing bullet — *"Detector round 3 licensed, or the
shortfall stated"* — does not say which count decides.

**Why it matters.** This is the one thing a pre-registration exists to fix. On
the measurement in §1.5 the governed run will report roughly **150–200 keys,
~60–65 roots, ~14 trials**: D-537's letter cleared by 5–7x and the trial count at half
the floor. That is not a tail case, it is the expected outcome, and the document
as written licenses both readings of it. It also leaves D-618's own justification
incoherent: D-618 says licensing is *"on the letter"* **and** that no operator
ruling is needed because *"28 distinguishable trials is strictly larger than 28
distinct keys … in the tightening direction"*. Both cannot hold — if licensing is
on the letter, no larger minimum is registered and the "larger minimum" licence
is not being exercised; if a larger minimum is registered, licensing is not on
the letter.

**Minimal fix.** One sentence in §6 naming the count that decides, and what the
other two are for. If it is `w`, say that the trial count is reported and does
**not** gate — and then D-618's "why no operator ruling is needed" paragraph
needs amending, because the tightening it invokes is not performed.

### M4 — §6 registers an output that cannot be produced: it turns on a calibration D-618 retired unrun

**What is wrong.** §6 reads, in the present tense: *"**The calibration measures
the proving-rows-to-distinct-keys ratio** and this run applies no correction to
`n` for it … **The ratio is reported for both samples** so a successor can see
whether the margin was doing that work."* Revision 2's own opening states that
D-618 retired the cap calibration **UNRUN**. There is no calibration sample and
there never will be, so "reported for both samples" is a registered reporting
obligation with no possible discharge, and the sentence about applying no
correction answers a question nobody can now ask.

**Why it matters.** A registered output that cannot exist is not a harmless
leftover: §7 is the closure line a successor reads, and §6 is the section that
tells them how to read it. This is revision-1 text that revision 2 did not
revisit — the same class as M1's stale saturation claim.

**Minimal fix.** Rewrite §6 to answer its own heading (M3) and delete the
calibration clause. If the ratio is still wanted, it is computable from the
governed run alone (win-proving rows ÷ distinct win keys) and should be listed in
§7 as one figure over one sample.

### M5 — no committed, revision-named, tested instrument produces the registered distinguishable-trial count

**What is wrong.** `docs/process.md`, "Instrument governing revision": *"An
artefact that produces a registered number — a `tools/` script, a scratchpad
harness, or a command block the document prints — is named in the
pre-registration WITH ITS REVISION."* The trial count is the number this revision
exists to introduce, and:

- `tools/stage3_census_rank.py` does **not** compute it. Its `signature()` is a
  twelve-field tuple including `index`, `att_visits` and `def_visits`, used for
  the *cache* figure — a different quantity. Nothing in `tools/` partitions win
  rows by the CANDIDATES verdict vector.
- The only implementation is `artifacts/wp22_cap_decision/signatures.py`, which
  is **gitignored**, carries **no test**, and hard-codes
  `artifacts/wp22_cap_dryrun_v2/d3_c{cap}.txt` paths.
- `docs/process.md`, "tools/ review coverage rule": *"any tools/ script that
  produces a recorded number carries at least one test driving the shipped
  script."* There is no shipped script to test.

§4 names `trigger_census` without a revision; §3 names
`tools/texel/draw_census_samples.py` without one; §8's pre-run check names no
instrument at all (it is `crates/pistol-core/examples/fixture_key_full.rs`,
committed at `42967e0` — after `b876d1d`, the revision the dry run it inherits
was taken at).

**Why it matters.** The registered number has no instrument the tree holds, so
nothing can be reviewed against it and D-582's "green forever" class applies
directly. It also means the §5 numbers a reviewer must check live only in a
gitignored directory (see m5).

**Minimal fix.** Land the trial-count computation as a `tools/` script with one
test driving the shipped script, and name it in §4/§7 with its revision, together
with `trigger_census`, `draw_census_samples.py` and `fixture_key_full`.

### M6 — two of the three MEASURED rates in §5's table do not transfer to a second sample of the same population, and the document extrapolates all three

**What is wrong.** §5's table marks 0.86 keys/position and 0.10 trials/position
**MEASURED** and converts each to an `n` by division. On a disjoint 500-position
draw from the same sha256-ordered corpus the key rate is **0.21** (4.1x
optimistic) and the trial rate **0.024** (4.2x optimistic). Only the root rate
transfers (0.08 → 0.072). The consequence for the table's third column: `n` for
28 keys is **216**, not 33; `n` for 28 roots is **420**, close to the registered
350; `n` for 28 trials does not exist.

**Why it matters.** The slices are contiguous blocks of a `sha256(key_full)`
ordering — a pseudo-random permutation — so they are exchangeable by
construction and the disagreement is not a slice property. It is (a) sampling
noise on a heavy-tailed key count and (b) saturation on the trial count. A
registration that divides a 100-position rate by a target and reports the
quotient as an ESTIMATE is making a modelling assumption it does not state, and
it is wrong for two of three rows.

**Minimal fix.** State the model. For a saturating count, quote a rarefaction or
an extrapolator and its assumption, not a quotient; for the key count, quote an
interval (the red team's own bootstrap at n=500 gave [233, 660] — my measured 104
is far below its lower end, which is itself worth a line).

---

## 3. MINOR findings

**m1 — `p1 = 12/14` is cited to §5.4; it is defined in §5.8.** §5.4 ("What §5.2
and §5.3 license") contains neither 0.857 nor 12/14. The definition is §5.8's
"bound over the columns" table. The mis-citation is inherited from
`overnight2_ledger.md` §4 (which additionally says "§5.4's table" — §5.4 has no
table) and from the red team's B1, so three documents now carry it. **Fix**: cite
§5.8 here and note the inheritance so the ledger and the red team can be
corrected in the same commit.

**m2 — §8 understates its own dry run and contradicts §3.** §8: the criterion
*"passed 0 of 20"*. `wp22_cap_prereg.md` §9 and
`artifacts/wp22_cap_dryrun_v2/RECEIPT.md` both record **0 of 100**; the "19 of 19"
shift test is the separate 20-row one. §3 of this same document says revision 4
*"enlarged the dry-run slice from 20 rows to 100"*. **Fix**: `0 of 100`.

**m3 — §4 over-generalises the node-budget result.** *"…so most of the apparent
cap complementarity is a NODE-BUDGET artifact."* Measured, the two caps disagree
on **7** roots: `{7, 65, 98}` proved only at 2048 and `{16, 52, 74, 91}` only at
16384. Raising nodes recovers **2** of the 7. The other 4 are D-563's structural
foreclosure — at cap 2048 those roots fire 28/12/12/14 times and every attempt
truncates at the cap, which no node budget changes. 2 of 7 is a minority. **Fix**:
say "two of the three roots cap 2048 alone proves", and drop "most of the
apparent cap complementarity". The decision to decline a second arm does not
depend on this sentence — the 1.4x trial-rate spread carries it, and carries it
on the nine-column partition too (0.26/0.14/0.15 = 1.86x, still far from the key
count's 4.5x).

**m4 — §4's command block contains an undefined placeholder.**
`--fixture <census slice prefix>` — nothing in the document or the tree produces
an 800-row prefix or fixes its digest; `draw_census_samples.py census` emits all
89 205. Given this package's own recorded history of a registered command that
does not run (`wp22_FINAL_SUMMARY.md` §9: *"a search command written into a
registration that does not run (dropped dash-escapes)"*), a placeholder in the
one command block the run executes is worth closing. **Fix**: print the literal
two-command block, and register the prefix's `sha256`.

**m5 — every number in §5 rests on gitignored artifacts that no committed
document sha-indexes.** `artifacts/` is gitignored (correct, rule 8), but neither
`artifacts/wp22_cap_decision/RECEIPT.sha256` nor
`artifacts/wp22_cap_dryrun_v2/RECEIPT.sha256` has any digest appearing anywhere
under `docs/` or `tools/` — `git grep` for all four `wp22_cap_decision` digests
returns nothing. Rule 8's own escape is *"a committed manifest may sha-index
them"*, and §9 of this document already applies that discipline to the corpus
manifest. **Fix**: name `d3_c2048.txt`'s digest
(`343429341c3f9b7160c04436eb3173650977b8ba2d5fb1de573855b57bac0d3d`) in §5, since
every §5 number is derived from that one file.

**m6 — the slice boundaries are now stated in three places, one of them
retired.** They are `DRY_RUN = 100` / `CALIBRATION = 600` in
`tools/texel/draw_census_samples.py`, revision 4 of the **retired**
`wp22_cap_prereg.md`, and §3 here. §3 binds a successor to change "BOTH
documents in one commit (D-599)" — but one of the two is retired and will not be
maintained, and neither is the artefact that actually decides the boundaries.
**Fix**: point §3 at the draw script's two constants as the single source, and
drop the tie to the retired document.

**m7 — §7's "`w` by the position's turn count" is not computable from the run's
output as registered.** The census row's `turns` column is depth **from the
search root**, not the root position's turn count; the per-entry line carries no
turn count either. The root's `depth_turns` lives in the manifest (column 5) and
reaching it needs a join on the fixture's line number. **Fix**: say which is
meant and, if it is the root's, register the join.

**m8 — line 3's "Governing revision: `84c1af2`" names a tree that does not
contain this revision of this document.** Revision 2 landed at `8153369`; HEAD is
`ce6cfd1`. Code is byte-identical across all three, so nothing is materially
wrong, but D-617 exists because a run is only valid over a frozen tree and this
document's own stated revision is already two commits stale. **Fix**: restate the
governing revision at the commit the run is taken over, immediately before the
run, and freeze.

**m9 — §9's first VOID condition cannot fire.** *"VOID AND RE-RUN: the box busy
with a timed run, transient I/O."* This instrument is node-budgeted and
deterministic (D-615 confirmed a byte-identical repeat), so a busy box changes
the wall clock and not one census row or one count. The condition protects the
45-minute estimate and nothing that is reported. **Fix**: say so, or delete it —
a void condition that cannot change a reading is prose a reviewer must still
attack (D-424).

---

## 4. QUESTIONS — flagged as unsubstantiated, not asserted

**Q1 — does a count of 28 distinguishable classes actually restore the
binomial?** The classes are wildly unequal: at cap 2048 the ten classes hold
`[71, 14, 4, 2, 1, 1, 1, 1, 1, 1]` rows. Reaching 28 classes gives 28 independent
trials only if the test is then run on **one representative per class**, and
nothing in this document or in D-618 registers that. If round 3 runs on all rows,
28 classes does not fix the correlation the red team's B1 measured; if it runs on
representatives, the census must record which representative and why. I have not
established which round 3 intends, and it may legitimately be round 3's to
register — but then M3's answer should say so.

**Q2 — is a replication owed?** `docs/process.md`: *"Where the run is cheap,
doubt about the instrument is answered by REPLICATION and by a SECOND INSTRUMENT
whose agreement criterion is registered before either runs."* The run is 45
minutes and the instrument is deterministic, so a replication is nearly free but
also nearly vacuous. The document neither registers one nor says why none is
owed. I do not claim one is required; I claim the document should say.

**Q3 — does §8's inherited dry run still discharge the obligation?** I lean yes,
and I checked the two things that could have broken it: the draw reproduces
byte-identically at HEAD, and the criterion passes on the **whole** governed
fixture (0 of 89 205, 2.3 s) — so the property the dry run establishes is
verified directly on the governed sample, which is stronger than inheritance.
But `docs/process.md`'s dry-run rule says *"the pre-registration records the
dry-run input and its output"*, and this one records another document's, at a
revision (`b876d1d`) that predates the commit (`42967e0`) where the criterion's
instrument entered the tree. Whether inheriting across a retirement satisfies the
rule is a reading I do not think a reviewer can settle alone.

---

## 5. What I attacked and could not break

Stated explicitly, because a FAIL is not a verdict on everything.

1. **The `signatures.py` transcription is faithful.** I re-transcribed all
   fourteen CANDIDATES from `tools/stage3_census_rank.py` and partitioned with
   both. The omitted entry is `("incumbent (any hot, either side)", lambda r:
   True)` — a constant, which cannot split any class — and the 14-predicate and
   13-predicate partitions give **identical** counts, 10 / 7 / 8. The
   registration's "against the candidate field of `tools/stage3_census_rank.py`"
   is accurate.

2. **The collapse is real, and it is a correctness finding, not prose.** 86 keys
   from 8 roots with 35 in the largest is exactly as stated, 28 keys really are
   reachable from one search tree, and on any partition the trials are far fewer
   than the keys. D-618 was right to treat B1 as a correctness finding. My
   quarrel is with which partition, not with whether one is needed.

3. **The tightening is legitimate and is not a loosening.** A class is a
   non-empty set of keys, so trials ≤ keys **always**; requiring 28 trials never
   admits a sample the 28-key rule rejects. D-537's condition (1) — counted in
   positions, not firings, not games — is not breached by a coarsening of
   positions, and reusing `n = 28` in the trial currency is arguably the *correct*
   application of a power calculation whose `n` means independent trials, not a
   re-derivation. Attack 3 survives: the defect is that the tightening is **not
   performed** (M3), not that it would be illegitimate.

4. **Retiring the cap calibration unrun is defensible.** The 1.4x spread
   reproduces (0.10 / 0.07 / 0.08), and it survives M2: on the nine-column
   partition the spread is 0.26 / 0.14 / 0.15 = 1.86x, still nothing like the key
   count's 4.5x. Nothing in this registration depends on a measured cap choice —
   the cap is fixed by D-618, the run is single-arm, and no reported quantity is
   a function of an unmade cap decision. The node-budget measurement that
   demotes the second arm reproduces exactly.

5. **Everything mechanical in §3 and §8 works.** The draw reproduces
   byte-identically for all three slices; rows 600 onward really is 89 205
   positions; the manifest digest matches `arc3_ledger.md` §5, so §9's VOID AND
   REPAIR condition is well-founded and currently satisfied; the pre-run replay
   check runs on the whole census fixture in 2.3 s and returns 0 of 89 205; the
   identity is `pistol_core::canonical_key` computed in one place
   (`crates/pistol-search/src/census.rs:30`), as §2 and D-570 say.

6. **The cost estimate is right.** 3.31 s/position measured on 500 fresh
   positions against the registered 3.41; `n = 800` really is about 45 minutes.
   Of the four MEASURED figures §5 extrapolates, this is the one that transfers.

7. **"The run does NOT stop when any count reaches 28"** is correct discipline
   and I have no attack on it.

---

## 6. What would turn this into a PASS

M1 and M3 are the two that must move; M2 decides which way M1 can go. Concretely:

1. Answer §6's own heading: name the count that licenses round 3 (M3), and
   reconcile D-618's justification with the answer.
2. Fix the partition question (M2) — cite §5.8, and either adopt
   `stage3_allocator_bound.py`'s `COLUMNS` as the trial unit or drop the
   "alternative's own definition" ground.
3. Re-size §5 on the measurement in §1.5 rather than on a linear rate (M1). If
   the candidate-field currency is kept, the honest registration is that its
   asymptote is 15–18 and the run cannot clear 28 — which is a finding available
   now, without the 45 minutes.
4. Repair §6's retired-calibration clause (M4) and land a tested instrument for
   the trial count (M5).
5. The MINORs are one commit between them.
