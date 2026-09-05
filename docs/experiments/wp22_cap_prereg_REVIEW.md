# REVIEW — `docs/experiments/wp22_cap_prereg.md` revision 2

**Revision reviewed.** Revision 2, as it stood in the working tree while `git log -1`
read `71fa6f1` (`dev`).

**Does it match HEAD?** No, and not in the ordinary way. The file is **UNTRACKED**
(`git status` shows `?? docs/experiments/wp22_cap_prereg.md`); `71fa6f1` does not
contain it and neither does any commit. There is therefore no revision of this
document, and none of its predecessor "revision 1", that a reviewer or a successor
can name, diff, or verify. See F13.

**VERDICT: FAIL.**

Thirteen MAJOR findings. The three that on their own require a re-registration:
the ladder's measured anchor (§4) was taken at `--nodes 400000` and the registered
seat is `--nodes 50000` (F1), the tree's own 50 000-node censuses **contradict**
§4's registered expectation (F2), and the top rung of the ladder is not a cap at
all at this seat because solver nodes are inside the node budget and 131072
exceeds it (F3).

---

## Re-derivation ledger

Per `docs/process.md` *Re-derivation*, every load-bearing count below was produced
by a command I chose, not one the document prints. Scope is stated beside each.

| # | claim under test | my command (scope) | result |
|---|---|---|---|
| R1 | instrument revision `0f58533`, "unchanged at `71fa6f1`" | `git diff --stat 0f58533 71fa6f1 -- crates/pistol-search/examples/trigger_census.rs` | empty diff — **claim holds** |
| R2 | corpus is 89 805 distinct positions | `/usr/bin/grep -vc '^#' artifacts/arc3r_sweep_deduped_manifest.txt` | `89805` — matches the manifest's own `# derived distinct_positions` |
| R3 | manifest identity | `sha256sum artifacts/arc3r_sweep_deduped_manifest.txt` | `00f61780cc1654958696786051dbd8de7d1bbab3f5d49153b1694770caf35968`, **equal to the committed index at `docs/experiments/arc3_ledger.md:1969`** |
| R4 | manifest body identity | `grep -v '^#' … \| sha256sum` | `f3135fc1f88459c8e954dbe2b3574cf15538c043a78f0929a2308b317f9a03b0`, equal to the file's own `# body_sha256` |
| R5 | `r[4]` is `key_full` | `head -c 4000` on the manifest, reading its `# columns:` line | columns are `corpus_index, record_number, key_seq, key_pos, key_full, depth_turns, result, end` → index 4 **is** `key_full`; the draw block's index is right |
| R6 | `corpus_index` → `tranche-<n>` | `sha256sum tranche-{1,9,16}/corpus.txt` vs `arc3_ledger.md:1973,1981,1988` | all three match; mapping **verified**, including the 1-vs-9 case that record counts alone cannot separate (both 12 362) |
| R7 | `record_number` is the 1-based body line | manifest row `(9, 3)` `key_full` vs `grep -v '^#' tranche-9/corpus.txt \| awk 'NR==3'` col 6 | identical (`-4,1:p2 -3,2:p2 0,0:p1`) — join spec **correct as written** |
| R8 | D-563's cap figures | `cat artifacts/wp20b_cap_SUMMARY.txt` | 2048: 294+400 = **694** firings, **0** win-proof; 16384: 41+63 = **104** firings, **1** win-proof; walls 149→125 and 53→50; distinct 49→25, 26→12. **Every one of §4's four quotations is accurate.** |
| R9 | the seat those figures were taken on | `grep argv artifacts/wp20b_cap_out_*.txt` | **`--nodes 400000`** in all six files — not 50 000. See F1. |
| R10 | "the budget every prior census in this project was taken at" | `grep argv artifacts/stage3*_census_*.txt` and `artifacts/wp20b_cap_out_*.txt` | stage3b/3c: `--nodes 50000` ✓. wp20b: `--nodes 400000` ✗. **Claim false.** |
| R11 | win-direction proofs at the *registered* seat | `grep -o "att_proved true" artifacts/stage3b_census_corpus_r{0,1}_v1.txt \| wc -l` | r0: **1** in 361 firings / 24 entries; r1: **4** in 370 / 24. Both at `--nodes 50000 --cap 2048`. See F2. |
| R12 | firings per entry at the registered seat | awk over the `entry` lines of `stage3b_census_corpus_r0_v1.txt` | 24 entries, 361 firings, **mean 15.0**, mean total nodes 47 644 |
| R13 | solver nodes are inside the node budget | `crates/pistol-search/src/pvs.rs:198` (`self.search_nodes + self.solver_nodes`), `stop.rs:80`, `pvs.rs:71` | confirmed. See F3. |
| R14 | `key_seq` is replayable | manifest rows 3–5 and 4371 of `corpus_index 9` | `0,0 -4,1/-3,2 -1,-4/4,-4 …` — exactly the `state_of` grammar. See F11. |
| R15 | early-position share | awk word-count of `key_seq` over all 89 805 rows | ≤4 turns: **9 303 = 10.4 %**. §3's "no content filter" costs about 41 of the 400. |
| R16 | `depth_turns` semantics | `# param depth_meaning` in `tranche-1/corpus.txt` | "a completed search depth" — the **label's** depth, not the position's turn index. See F17. |

---

## MAJOR findings

### F1 — MAJOR. §4's entire measured anchor was taken at `--nodes 400000`; the registered seat is `--nodes 50000`, and the document never says so. §2's supporting claim is false.

**What is wrong.** Every number §4 quotes from D-563 — 694 firings, 104 firings,
the one win-proof, `att_visits 11040`, 149 s→125 s, 53 s→50 s, the ~2x signature
collapse — comes from `artifacts/wp20b_cap_out_*.txt`, and all six of those files
carry `--nodes 400000` on their own `argv` line (R9). §2 then registers
`--nodes 50000` and defends it with *"it is the budget every prior census in this
project was taken at (`artifacts/stage3b_*`, `artifacts/stage3c_*`)"*. That is
false as stated (R10): the `wp20b_cap_*` runs are `trigger_census` runs, they are
censuses, they are prior, and they are at 400 000 — and they are the very runs
§4 is anchored on. The document names the two artifact families that agree with
it and omits the one that does not, which is exactly the wrong-population defect
`docs/process.md` says this project keeps paying for.

**Why it matters.** Cap and node budget do not compose additively. The number of
solver calls that fit in a position is of order `nodes / (2·cap)`; at 400 000 that
is 97 / 12 / 1.5 for the three rungs, at 50 000 it is 12.2 / 1.5 / **0.19**. Every
rate §4 imports — firings per position, and therefore win-proofs per position —
is divided by eight at the registered seat, and the document performs no such
division anywhere. The result is that §4's "honest expectation" and §7's cost are
both computed on a seat the run will not use, in opposite directions (F2, F7).

**Minimal fix.** In §2, delete "every prior census in this project" and write:
*"the budget the stage-3b/3c censuses were taken at, and NOT the budget D-563's
cap figures were taken at, which is `--nodes 400000`
(`artifacts/wp20b_cap_out_*.txt`)."* In §4, restate each imported rate as a
per-firing rate and apply the seat's own firings-per-position, or move the seat
to `--nodes 400000` so the anchor transports unchanged.

### F2 — MAJOR. The tree's own 50 000-node censuses contradict §4's registered expectation that the 2048 arm returns `w = 0`.

**What is wrong.** §4 registers: *"On D-563's rates the 2048 arm is expected to
return `w = 0`"*, and §5 builds the zero-arm exclusion around that prediction. At
the **registered seat** (`--nodes 50000 --cap 2048`) the standing in-tree censuses
hold win-direction proofs (R11): `stage3b_census_corpus_r0_v1.txt` has **1**
`att_proved true` in 361 firings over 24 entries, and
`stage3b_census_corpus_r1_v1.txt` has **4** in 370 firings over 24 entries. The
zero D-563 reports is a zero at cap 2048 **and 400 000 nodes on three-entry bench
fixtures**, where a position's 116 firings are spread over a budget eight times
larger; it is not a zero at this seat.

**Why it matters.** The registered expectation is the document's only stated prior
about what the run will produce, the zero-arm clause exists to handle it, and §4
pre-frames a positive `w` at 2048 as *"new information against D-563"*. It would
not be new information; it is what the two nearest 50 000-node runs already show.
A prediction contradicted by artifacts already in the tree is a pre-registration
that has not read its own evidence.

**Minimal fix.** Replace §4's expectation paragraph with the measured one: at
`--nodes 50000 --cap 2048` the corpus bands give 1/361 and 4/370 win-direction
proofs per firing, so `w(2048) > 0` is the expectation and a zero at 2048 is the
surprise. Keep the zero-arm exclusion — it is correct regardless (see S6) — but
stop attaching it to the wrong arm.

### F3 — MAJOR. At `--nodes 50000` the 131072 rung is not a cap. Solver nodes are inside the node budget, so the arm measures "one uncapped solver call per position", and §4's "the three arms differ in the cap and in nothing else" is false.

**What is wrong.** `crates/pistol-search/src/pvs.rs:198` defines the budget
quantity as `self.search_nodes + self.solver_nodes`; `stop.rs:80` stops when that
reaches the `Stop::Nodes` bound; and `pvs.rs:71` states it in the code's own words
— *"the root's calls are inside the same budget"*. The doc comment at
`pvs.rs:855-857` adds that solver nodes are *"absorbed into the budget the moment
each call returns"*, i.e. the check is **after** the call, so a single call may
overrun the whole budget before anything notices. Measured confirmation in the
tree: at `--nodes 50000 --cap 2048` the mean total is 47 644 nodes (R12) and entry
0 spends 2 486 + 49 619 = 52 105; at `--nodes 400000 --cap 16384` entry 0 spends
3 752 + 415 999 = 419 751, and 23 firings consumed 415 999 solver nodes = 18 086
per firing, i.e. calls routinely run to the cap.

With `--cap 131072` and a 50 000-node budget, `nodes / (2·cap) = 0.19`: the first
firing's attacker call alone can spend 131 072 nodes — **2.6x the registered
budget** — and the search then stops. The arm does not run a search at 50 000
nodes with a 131 072 per-call cap; it runs roughly one solver call per position,
uncapped in practice, at 2.6–5x the node spend of the other two arms.

**Why it matters.** Three things break at once. (a) §4's *"the three arms differ
in the cap and in nothing else"* is false — they differ in effective compute per
position by a factor of about five. (b) The rung is not distinguishable from any
other cap ≥ 50 000, so "131072" is not the quantity being measured and the result
cannot be carried to a census at a different budget. (c) It is the least-powered
arm (F4) while being the most expensive per position, which is the worst possible
allocation of the run's seconds.

**Minimal fix.** Either raise the seat to `--nodes 400000` (which also repairs F1
and F7 and makes D-563's figures transport unchanged), or drop 131072 and register
D-563's own two caps, which is what D-563 asked for. If a third rung is wanted at
50 000 nodes it must sit strictly below the budget — e.g. 8192 between the two —
and §6 must report each arm's mean total nodes per position so the ratio's
denominator is visible.

### F4 — MAJOR. There is no sizing, and D-563 contains an instruction addressed to exactly this document that §4 quotes around.

**What is wrong.** §4 quotes D-563 five times and omits the one paragraph that is
about sizing this run:

> *"ITS SIZE IS NOT REGISTERED HERE AND A FIRST ATTEMPT AT IT WAS WRONG … it
> applied §2's 116 firings-per-ask — a cap-2048 figure — to both arms, two
> paragraphs after measuring that rate collapse 6.3x-7.2x at the larger cap, and
> named no position source. The inputs are all measured and all on the design's
> face (pooled win-proof rate 1 in 798 firings; ~116 firings per ask at cap 2048
> against ~17 at cap 16384), **each arm is sized on its OWN rate**, and the sizing
> belongs to whoever runs it."*

The document is whoever runs it. It sizes nothing. It picks 400 with no stated
derivation and gives every arm the same 400, which is the same defect in a new
costume: equal positions is not equal evidence when the arms' firing rates differ
6.3x by measurement.

Using the registered seat's own measured firing rate (R12: 15.0 firings/entry at
cap 2048, 50 000 nodes) and D-563's measured 6.3x collapse at 16384, and F3's
budget bound at 131072:

| arm | firings on 400 positions | E[w] @ pooled 1/798 | E[w] @ 1/104 (one observation) | E[w] @ in-tree 1/361 | P(w = 0) @ pooled |
|---|---|---|---|---|---|
| 2048 | ~6 000 | 7.5 | 57.7 | 16.6 | 0.001 |
| 16384 | ~950 | **1.19** | 9.2 | 2.6 | **0.30** |
| 131072 | ~400 | **0.50** | 3.9 | 1.1 | **0.61** |

**Why it matters.** The two arms the whole exercise exists to separate are more
likely than not to come back with `w` of 0 or 1. A selection between `w = 1` and
`w = 2` is a selection on one observation, and §5's 10 % tie band cannot express
that (F6). Worse, the arm most likely to return zero is excluded by §5's own
zero clause — so the most probable outcome of the registered run is "2048 wins
because the other two arms had no firings", which is a statement about sample
size, not about caps.

**Minimal fix.** Register a sizing paragraph: state the seat's measured firings
per position per arm, the imported per-firing proof rate, the resulting E[w] per
arm, and choose `n` so the **smallest-firing** arm reaches a stated E[w] (10 at
the pooled rate needs ~7 980 firings, i.e. ~3 350 positions at cap 16384 and
~8 000 at 131072). If that `n` is unaffordable, say so and drop the arm rather
than run it underpowered — dropping 131072 (F3) makes a sized two-arm ladder
affordable.

### F5 — MAJOR. The all-zero branch's registered conclusion is not identified by the data, and §5 forecloses the reading that is.

**What is wrong.** §5 registers: if every arm returns zero, *"this calibration
returns no cap. That is a FINDING and not a run to repeat with a bigger ladder:
it says the label-sweep population does not prove wins at 50 000 nodes."* Given
F4's table, an all-zero ladder is confounded between two hypotheses the run cannot
separate: (i) the population does not prove wins at this budget, and (ii) 400
positions produced too few firings to see a rate of order 1/800. §5 pre-commits to
(i) and explicitly forbids the response that would distinguish them.

**Why it matters.** This is a registered conclusion that the registered instrument
cannot license — the defect a pre-registration exists to prevent, arriving from
the other direction. And the forbidden response is misidentified: the fix for an
undersized run is a bigger **sample**, which §5 has conflated with a bigger
**ladder**.

**Minimal fix.** Replace the conclusion with a bound the data does license: with
`F` firings and zero proofs the 95 % upper bound on the per-firing win-proof rate
is `3/F` (rule of three). Register: *"an all-zero ladder reports, per arm, the
firings observed and the implied `3/F` bound; it licenses no statement about the
population below that bound, and a larger sample — not a larger ladder — is what
would tighten it."*

### F6 — MAJOR. The 10 % tie band is an order of magnitude finer than the sample's own noise, and it is not transitive over three arms.

**What is wrong.** §5: *"two arms are treated as tied when the larger cap's ratio
does not exceed the smaller's by more than 10 %."* On F4's projections `w` is a
count of order 1–10. Two arms at `w = 2` and `w = 3` with comparable `s` differ by
50 % in ratio — far outside the band, so the rule declares a winner — while an
exact test on 2 versus 3 is nowhere near separating them. The band therefore
almost never binds, and the rule degenerates to a bare `argmax w/s` on counts whose
sampling error is of the same size as the counts.

A second, independent defect: pairwise "tied" is not transitive. Ratios of
1.00 / 1.09 / 1.19 make arm 1 tied with arm 2 and arm 2 tied with arm 3, while
arm 3 beats arm 1 by 19 %. §5 says ties go to the smaller cap but never says in
what order the three comparisons are made, so this input has no defined answer.

**Why it matters.** A registered decision rule with an undefined output on a
plausible input is not registered, and a band that cannot fire is not a
precaution. Both leave the after-the-numbers choice §5 exists to forbid.

**Minimal fix.** Two lines. (a) Make the rule total: *"select the SMALLEST cap
whose ratio is within 10 % of the maximum ratio over selectable arms."* That is
well defined for any number of arms and preserves the stated preference. (b) Add
a precision floor: *"if the selected arm's `w` is below `k` (register `k`), the
selection is reported as UNDERPOWERED and the incumbent cap stands"* — otherwise
the 10 % band is decorative next to the count's own error.

### F7 — MAJOR. §3 defers the corpus digest to run time, while a committed sha-index for the manifest *and* for the sixteen corpus files the join reads already exists in the tree.

**What is wrong.** §3: *"Its digest is recorded in the run receipt at run time and
is the manifest's identity for this registration."* §9 then voids a run on *"a
manifest digest that does not match the one the receipt recorded"* — which is
circular: the receipt records whatever it is shown, so the pair pins nothing.
Meanwhile `docs/experiments/arc3_ledger.md:1963-1996` is a **committed** export
block that already sha-indexes `artifacts/arc3r_sweep_deduped_manifest.txt`
(`00f61780…`), `artifacts/arc3r_sweep_raw_manifest.txt` (`8a3f9f92…`) and all
sixteen `tranche-N/corpus.txt` files. I verified three of those against disk
today; all match (R3, R6).

**Why it matters.** `artifacts/` is gitignored (`.gitignore:19`) and
`/home/tom/pistol-runs/` is outside the tree, so every input this registration
reads is unversioned. Hard rule 8's whole mechanism for that case is the committed
sha index, and it exists here. Not citing it means the registration's corpus
identity is whatever is on disk on the day, which is precisely the identity a
pre-registration is supposed to remove.

**Minimal fix.** In §3, replace the sentence with: *"The manifest's identity is
`00f61780cc1654958696786051dbd8de7d1bbab3f5d49153b1694770caf35968`
(`sha256sum`, whole file), sha-indexed at `docs/experiments/arc3_ledger.md:1969`;
its body digest is `f3135fc1…` and it holds 89 805 body rows. The join's inputs
are the sixteen `tranche-N/corpus.txt` files sha-indexed in the same block, and
the fixture build verifies each before reading it."* Then §9's VOID clause has
something to compare against.

### F8 — MAJOR. §8's criterion cannot falsify either of the two defects that would actually wreck this run.

The criterion itself is sound for the defect it names — see S3, it survives that
attack. The finding is what it leaves uncovered, and the uncovered defects are
larger than the covered one.

**(a) The draw is unverified, and it is a named instrument.** §2 registers the §3
draw block as an instrument that "produces both samples". §8's criterion is
per-row (`key_full` = recomputed canonical form), so it passes **for every row of
any sample whatsoever**. A wrong sort key, a wrong column index, a wrong slice, a
manifest that gained a column — none of it is detectable. The block indexes `r[4]`
and `r[4]` is in fact `key_full` (R5, verified against the manifest's own
`# columns:` line, so this one survives), but nothing in the registration checks
that, and the block asserts no schema.

**(b) The cap reaching the solver is unverified, and D-563 is the record of exactly
this failure.** D-563 exists because a census token was set on a seat where it
recorded nothing. If `--cap` were ignored, clamped, or the three arms otherwise
identical, all three would return the same `w` and `s`, §5 would call it a tie,
and the run would report "the cap does not matter" — a conclusion with no
registered check standing against it. Likewise, if `collect_trigger_census()` did
not fire, every arm returns zero rows and §5 reads that as `w = 0` and §9 does not
void it: the D-563 defect returning as a false FINDING.

**Minimal fix.** Add two registered criteria with consequences, both computable
from the census output the run already produces:
- **Draw:** the receipt records `sha256` of the newline-joined sorted `key_full`
  values of the 400 and of the 20, and a **differently written** command (shell
  `cut`/`sort`/`sha256sum`, not the Python block) reproduces both. Mismatch = the
  registration does not proceed.
- **Manipulation:** per arm, `max(att_visits) <= cap`; across arms, total firings
  must fall as the cap rises (D-563 measures 6.3x–7.2x from 2048 to 16384). An arm
  with positive per-entry `firings` and zero census rows is **VOID, never
  `w = 0`**; zero firings at every arm is VOID.

### F9 — MAJOR. §5 states the objective as "a FIXED count of 28", which §1 forbids the document to read — and 28 does no work in the derivation.

**What is wrong.** §1: *"It may not choose the census sample, state the census
count, or read anything about the floor of 28."* §5: *"The census must reach a
FIXED count of 28. Reaching it costs `28 · s(c)/w(c)` seconds…"* The document
violates its own prohibition in the section that matters most, and it does so
gratuitously: `argmin_c N·s(c)/w(c) = argmax_c w(c)/s(c)` for **every** positive
`N`, so the algebra is independent of the constant. The derivation is correct
(S5) and loses nothing at all if the number is never named.

**Why it matters.** §1's prohibition is the boundary between this registration and
the census registration. A document that states the boundary and crosses it in the
same breath licenses a successor to cross it further, and the floor of 28 is
precisely the quantity `docs/experiments/overnight2_ledger.md` §4 and D-537 insist
be fixed independently of anything measured.

**Minimal fix.** In §5 replace "28" with "a fixed count `N`, registered
elsewhere", and add the one clause that makes the point stronger: *"the argmax is
independent of `N`, which is why this document need not know it."*

### F10 — MAJOR. §8's dry-run fixture is **not** disjoint from the census sample; §3 and §8 contradict each other.

**What is wrong.** §3 defines the census sample as *"rows **401 onward**"*. §8
draws the dry-run fixture as *"the last 20 rows"* of the same order and claims it
is *"disjoint from both"*. Rows 89 786–89 805 are members of "rows 401 onward".
The claim is false by the document's own two definitions.

**Why it matters.** `docs/process.md`'s dry-run rule turns on the input never
being the registered workload. Here the dry run does not search those positions —
it only replays and compares keys — so no contamination actually occurs, but the
registration asserts a disjointness property it does not have, and a successor
reading §8 will believe the reservation is airtight when it is not.

**Minimal fix.** One line in §3: *"Census sample: rows 401 through N−20; the final
20 are the dry-run fixture and are reserved from both."*

### F11 — MAJOR. The join to `/home/tom/pistol-runs/` is unnecessary, and the reason §3 gives for it is factually wrong.

**What is wrong.** §3: *"The manifest carries `key_seq`, the canonical sequence;
the fixture needs a replayable turn list, which is the corpus record's own `moves`
field."* The premise is false. `key_seq` **is** a replayable turn list, in exactly
the grammar `trigger_census`'s `state_of` parses (`crates/pistol-search/examples/
trigger_census.rs:54-68`: turns separated by whitespace, the two stones of a turn
by `/`). Verified on real rows (R14): manifest `corpus_index 9` row 3 carries
`key_seq = "0,0 -4,1/-3,2"`, and row 4371 carries a 26-turn sequence of the same
shape. Replaying `key_seq` yields a symmetry image of the record's position, whose
`canonical_form` is the same `key_full` — which is the property §8's own criterion
already relies on.

**Why it matters.** The join is the sole reason §8's defect class exists, the sole
reason the run depends on sixteen files outside the repository, and the sole reason
the fixture build has a stage that can silently address the wrong record. All three
disappear if the fixture is built from the manifest's own `key_seq`. This project's
own history says the out-of-tree dependency is the risky half: CLAUDE.md's worktree
clause and the standing memory note both record artifacts vanishing with their
directories.

**What is genuinely true and should replace the stated reason.** `moves` and
`key_seq` are different spellings — they differ in 10 352 of tranche-1's 12 362
records (measured) because `canonical_sequence` returns a symmetry image — so
replaying `key_seq` searches a symmetric twin of the position that was labelled.
If the author wants the as-played board (e.g. to relate `w` to the label's own
columns), that is a real reason and it is not the one written down.

**Minimal fix.** Either build the fixture from `key_seq` and delete the join, §8's
defect class and the out-of-tree dependency together; or keep the join and replace
§3's justification with the true one — *"`key_seq` is a symmetry image of the
sequence that was labelled, and the calibration searches the as-played board"* —
and pin the sixteen corpus digests per F7.

### F12 — MAJOR. §4's key D-563 quotation is truncated at the clause that constrains it, with no ellipsis.

**What is wrong.** §4 quotes D-563 as *"a DEDICATED calibration run at caps 2048
and 16384, counting win-direction proofs on distinct keys once WP-2.0b's identity
exists"* and closes the quotation mark there. D-563's sentence continues:
*"…, **whose records are excluded from the corpus by construction** — not a
tranche, because a tranche split across two caps would ship a corpus labelled by
two instruments."* The omitted clause is the operative constraint on the run, and
nothing marks the omission.

**Why it matters.** The omitted clause is what §3's disjointness discipline is
answerable to, and a reader of §4 cannot tell that a constraint was dropped. It
also lets §4 present D-563 as licensing a three-cap ladder when the sentence names
two (F3, and MINOR F14).

**Minimal fix.** Quote the sentence whole, and add one line in §3 saying how the
clause is satisfied — the calibration produces no labels and no records enter the
corpus, so the exclusion is discharged by construction rather than by reservation.

### F13 — MAJOR. The document is untracked, and §2 names as an instrument revision a document revision that exists nowhere.

**What is wrong.** §2's instrument table gives the draw block's revision as
*"this document's revision 1"*. The document is untracked at `71fa6f1`; revision 1
was overwritten in place by revision 2 and exists in no commit, no stash, and no
file. Neither a reviewer nor a successor can establish what revision 1's block
said, so "unchanged since revision 1" is unfalsifiable. CLAUDE.md requires each
review to be dispatched against a **named revision** — a commit SHA or a
`git stash create` SHA — and `docs/process.md`'s *Instrument governing revision*
requires the instrument to be named with a revision a change to which reopens
review. Neither is satisfiable for this instrument.

A second defect in the same table: the block as printed **produces nothing**. It
reads, sorts, and exits; there is no slice, no write, and no output. The artefact
that will actually emit the 400-row and 20-row fixtures is therefore not the
registered one, and `docs/process.md` says a run standing on an unregistered
instrument is licensed by argument rather than by text.

**Minimal fix.** Commit the document (or `git stash create`) and name that SHA in
§2. Identify the draw block by the `sha256` of its own text rather than by a
document revision. And extend the block so that what is printed is what runs: sort,
slice, emit the `start moves` fixture lines and the `key_full` expectation file to
named paths, and print both digests.

---

## MINOR findings

### F14 — MINOR. "One octave beyond" is three octaves, and the `att_visits 11040` fact does not support going above 16384.

131072 is 2^17 against 16384's 2^14 — 8x, three octaves. More substantively, §4
argues the rung *"because the only observed proof sat at 11 040 visits and nothing
has ever been measured above 16 384."* 11 040 < 16 384, so that observation argues
that 16 384 **suffices** for the proof that was seen; it says nothing for or
against 131 072. The valid argument exists and is different: proofs needing more
than 16 384 visits are structurally invisible at 16 384, so a rung above it is the
only way to learn whether they exist. **Fix:** say the valid thing, and fix the
octave count — or drop the rung per F3.

### F15 — MINOR. §3 does constrain the census sample, which §1 says it may not.

§1: *"It may not choose the census sample."* §3: *"Census sample: rows **401
onward**, reserved for §4."* Reserving 401+ forecloses the census registration from
drawing any of the first 400 — a real constraint, imposed under a sentence saying
none is. The reservation is *right* (D-563 requires the calibration's records be
excluded), so the fix is not to remove it. **Fix:** in §1, replace "may not choose
the census sample" with "may not size or draw the census sample; it reserves the
calibration's own rows from it, which D-563 requires."

### F16 — MINOR. The cap is chosen at `--nodes 50000` but nothing pins the census's own budget, and the two interact.

The document selects a cap "that the census run of §4 will use" while §2 fixes
`--nodes 50000` for the calibration only. Firings per position scale as
`nodes / (2·cap)`, so a cap selected at 50 000 nodes is not the cap that would be
selected at 400 000 — and the corpus's own labels were produced at
`# param label_go go nodes 400000`. **Fix:** one sentence in §1: *"the cap this
document returns is valid for a census at `--nodes 50000`; a census at another
budget re-opens this calibration."*

### F17 — MINOR. §6's `depth_turns` breakdown is on the label's search depth, not the position's turn index, and "band" means something else in this project.

§6 registers `w(c)` broken down by *"the source row's `depth_turns`"*, offered as
what §4 may *"size a band restriction against"*. `depth_turns` is the manifest's
label column, and `tranche-1/corpus.txt`'s own `# param depth_meaning` defines it
as *"a completed search depth"* (R16) — a property of the labelling search, not of
the position. Meanwhile "band" in this project's census work (stage-3 bands 15 and
35) is a visit-share band. The natural restriction — how many turns are on the
board — is directly derivable from `key_seq`'s word count with no join at all, and
its distribution is measured: 10.4 % of the corpus is ≤4 turns (R15). **Fix:**
report the breakdown by the position's turn count, and either drop `depth_turns`
or say plainly that it is the label's depth.

### F18 — MINOR. §9's VOID list contains deterministic failures a re-run reproduces, and omits the one signature that most needs voiding.

*"A fixture line refused by the rules"* makes `trigger_census` exit 1 — and it does
so on the **first** bad entry (`return fail(&format!("entry {entries}: {why}"))`),
after which §9 says *"a void is re-run"*. Re-running the same fixture fails
identically; the fixture must be repaired, which is a different act. Conversely,
the signature that should void — positive per-entry `firings` with zero census
rows, or zero firings at every arm — is not in the list at all (F8b). **Fix:**
split §9 into "void and re-run" (box busy, transient I/O) and "void and repair"
(refused line, digest mismatch), and add F8b's two signatures to the second.

### F19 — MINOR. The `--gate off` reference arm is unmotivated and "at one cap" is meaningless.

§7 buys *"a `--gate off` reference arm at one cap"*. With `--gate off` the wiring
is `None` (`trigger_census.rs:125`) and the cap is printed but unused, so "at one
cap" names nothing. Neither §5 nor §6 consumes the OFF arm's output. **Fix:** say
what it is for — the OFF-seat node total is the denominator the stage-3 visit-share
bands are defined against (`stage3_rulings.md` §1.2, cited in the instrument's own
header) — or drop it.

### F20 — MINOR. §7's cost is wrong by the same 8x as §4's rates, in the direction that discourages the sample size §4 needs.

§7 derives *"a per-position cost near 17 s, which would put one arm near two
hours"* from D-563's 53 s / 50 s for three entries. Those are 400 000-node entries
(R9). At the registered 50 000 the per-position figure is nearer 2 s and an arm
nearer 15 minutes — with the 131072 arm the exception, since it spends ~2.6x the
budget per position (F3). §7 does flag the bracket as bench-derived, which is
honest, but not that it is eight times the seat. **Why it matters:** an 8x-inflated
cost makes the `n` F4 requires look unaffordable when it is not; the two errors
compound in the same direction. **Fix:** state the cost at the registered budget,
and state it per arm, since F3 makes the arms unequal.

### F21 — MINOR. The draw block asserts no schema.

The block splits on tab and takes `r[4]` with no check on the column count or on
the manifest's `# columns:` line, and a blank body line would raise `IndexError`
rather than a named error (hard rule 3). `r[4]` is correct today (R5). **Fix:**
assert the `# columns:` header equals the expected string and `len(r) == 8`, and
assert the body row count equals 89 805.

---

## QUESTIONS — raised, not asserted

### Q1. Is the sweep corpus as trigger-dense as `bench_positions_v1`?

Every projection in F4 assumes the sweep's positions fire at the bench fixture's
rate (15.0 firings/entry at the registered seat). `bench_positions_v1` is a
*selected* bench fixture; the deduped manifest is an arbitrary cross-section of
labelled play, 10.4 % of it at ≤4 turns (R15). If the sweep corpus fires at, say,
a tenth of that rate, every E[w] in F4's table falls by 10x and an all-zero ladder
becomes the modal outcome. **I could not settle this without building** (see Owed
checks). It is also the cheapest measurement in this whole package — a 20-position
pilot at one cap — and it is what the dry run of §8 should be measuring alongside
its join check.

### Q2. Does `canonical_form` of the empty position serialize as `-`?

The manifest's first row carries `key_seq = "-"` and `key_full = "-"` (the empty
board survives dedupe as a distinct position). §8's criterion compares `key_full`
against a value recomputed by `pistol-core`, which returns an empty
`Vec<(Coord, Player)>` there; whether the receipt's serializer spells that `-` is a
convention of whoever wrote the manifest, not of `pistol-core`. If the governed
400 draws that row (p ≈ 0.45 %) the comparison may fail spuriously. **Fix if
confirmed:** register the serialization explicitly, or exclude the two degenerate
rows (0 and 1 turns, exactly two rows in 89 805) and say so.

### Q3. The manifest reports 2 369 `key_disagreements`.

The header records *"key_disagreements: distinct positions sharing any one key
value with another; order-free — 2369"*. §3 describes the corpus as "one row per
distinct position under three-key agreement" without mentioning that 2.6 % of rows
have a key disagreement. I could not establish whether that affects `w`'s
distinctness claim, since `w` is counted over census `key`s at interior nodes, not
over manifest rows. Flagged so §6's collision column is read with it in view.

---

## What I attacked and it survived — said explicitly

**S1. The instrument's governing revision.** `0f58533`, "unchanged at `71fa6f1`" —
verified by an empty `git diff` over the file (R1). Correct.

**S2. The join specification.** `(corpus_index, record_number)` →
`tranche-<corpus_index>/corpus.txt` body line `record_number`, 1-based. I attacked
this three ways and it held all three: the directory mapping is confirmed by
digest for tranches 1, 9 and 16 (R6, including the 1-vs-9 pair that record counts
alone cannot separate); `record_number` is a **global** body-line index, not the
corpus's own per-game `col[1]`, and the document's phrasing "body line" is the
correct one; and a spot check of manifest row `(9, 3)` against tranche-9 body line
3 matches `key_full` exactly (R7). The join is unnecessary (F11) but it is right.

**S3. §8's referent does not share the suspect stage.** Attacked as instructed. The
fixture's input is the corpus `moves` column reached **through** the join; the
expected value is the manifest's `key_full`, which `wp21_assemble` computed from
the corpus record directly and which never passes through the fixture builder's
join. A join off by any amount lands on a different record, whose position has a
different canonical form, and the deduped manifest guarantees distinct rows have
distinct `key_full` — so the mismatch is detected. `canonical_form` is genuinely
needed rather than a plain sort: I verified on manifest row `(9, 4)` that the
record's raw sorted stone list is **not** `key_full`, because `canonical_form` and
`canonical_sequence` select different symmetries. The criterion is correctly
constructed for the defect class it names. What it misses is F8.

**S4. `w(c)`'s definition.** "Distinct `key` values carrying at least one row with
`att_proved true`" is the right operationalisation of D-537's *"win-proving firings
on disjoint positions"* under D-570's ruling that the identity is `CensusKeys::at`,
and the win-direction-only reading with `def_proved` reported beside it and never
summed is exactly what D-522 established and D-535 preserved when it superseded
D-522 on the gate question. `tools/stage3_allocator_bound.py:133`'s
`won(row) = att_proved == "true"` is the same definition. Correct.

**S5. The argmax algebra.** `argmin_c N·s(c)/w(c) = argmax_c w(c)/s(c)` for any
positive `N`. Correct as algebra — and independent of `N`, which is why F9's fix
costs nothing. The condition it silently needs is **linearity**: `w` must scale
with sample size. It does not, exactly — distinct keys saturate, and D-563 measures
the saturation running *harder* at the larger cap (signatures collapse ~2x). §6
flags the collision column as the thing to size against, but §5 asserts the
equivalence without the condition. **Fix (small):** add to §5 — *"the identity holds
for the AVERAGE rate at `n = 400`; it transports to a larger census only if
proving keys collide across roots at similar rates in every arm, which §6's
rows-per-distinct-key column reports. Arms differing by more than 2x on that
column are reported as not comparable on the ratio."*

**S6. The zero-arm exclusion is derived, not smuggled.** Attacked as instructed and
it holds. An arm with `w = 0` has infinite cost per proof, so `argmin N·s/w`
excludes it automatically; the exclusion is a restatement of the objective, not an
addition to it. It is registered before the numbers, and §5 registers the all-zero
consequence too. Sound. (The *prediction* attached to it is wrong — F2 — but the
clause is not.)

**S7. The draw's unbiasedness and reproducibility.** Ordering by `sha256(key_full)`
is a fixed function of the position's identity, uncorrelated with turn count,
depth, result or tranche, and it reproduces on any machine with no seed and no RNG.
It is a genuinely better choice than a seeded shuffle for this purpose. Sound.

**S8. "No content filter".** Attacked as a possible source of wasted budget; it
survives on the document's own reasoning — filtering to plausibly-firing positions
would bias toward easy proofs and the calibration's job includes pricing the
unfilterable ones. The cost is now measured rather than assumed: 10.4 % of the
corpus is at ≤4 turns (R15), so about 41 of the 400 are positions that essentially
cannot fire. Worth stating in §3 as a number.

**S9. Every D-563 quotation in §4 is accurate.** All four checked line by line
against `artifacts/wp20b_cap_SUMMARY.txt` and `wp20b_cap_out_trigger-rich_on_16384.txt`
(R8): 694 firings / 0 win-proofs at 2048; 104 / 1 at 16384; walls 149→125 and
53→50; signatures 49→25 and 26→12; and the single proof's row does carry
`att_visits 11040`. The problem with §4 is not misquotation — it is the seat those
figures were taken on (F1), the sentence left out (F12), and the arithmetic never
done (F4).

**S10. `overnight2_ledger.md` §3's "Waits on §2's deduped corpus" is stale, and
the document is right to ignore it.** §2 of that ledger still reads "NOT STARTED",
but the deduped manifest's own header is `wp21_assemble` over 16 corpora with
195 874 records and 89 805 distinct positions, and D-596 names *"the census run
over this corpus's positions"* as where the arc's census count comes from. The
document's corpus choice is correct; the ledger line is behind.

---

## Owed checks (not run — a timing-sensitive CI gate holds the box)

1. **Q1's density measurement.** Run `trigger_census --gate on --nodes 50000
   --cap 2048` over a 20-position fixture drawn from the deduped manifest and read
   the mean firings per entry. This is minutes of work, it converts F4's whole
   table from projection to measurement, and it is the single cheapest thing that
   would change what this document registers. It belongs inside §8's dry run.
2. **F3's degeneracy.** The same 20 positions at `--cap 131072 --nodes 50000`,
   reading the per-entry `firings` and `search_nodes + solver_nodes`. My claim that
   the arm collapses to ~1 firing per position and 2.6x the budget is derived from
   `pvs.rs:198` plus the two measured overshoots (52 105 against 50 000; 419 751
   against 400 000), not observed at that cap.
3. **Q2.** Whether the empty position's `canonical_form` serializes as `-` in the
   receipt's own comparison — one unit test, or one row of the dry run.
