# WP-2.0b DESIGN revision 7 — REVIEW-design (fresh context)

## Header

**Revision reviewed:** `c3757db4f9439887c483f5be5ff646c6c615bce5` — a `git stash create`
object on top of `dev` HEAD `a56449b`. **The tree still matches at the end of this
review**: `git diff c3757db --stat` is empty on entry and on completion, and
`git worktree list` shows the main tree only. I created no worktree and modified no
file but this one.

I did not write this document and I am not any of the six reviewers it answers.

**Read.** `docs/experiments/wp20b_design.md` in full (1070 lines);
`wp20b_design_rev6_REVIEW.md` in full; `wp20b_design_rev5_REVIEW.md` and the four
earlier reports at the findings they carry; `CLAUDE.md`; `docs/process.md`
§"Dry-run discipline", §"Criterion and defect class", §"Cost, replication, and the
second instrument" in full; `docs/decisions.md` **D-249, D-291, D-423, D-424, D-465,
D-512, D-516, D-517, D-522, D-529, D-530, D-535, D-537, D-560, D-562, D-563, D-564**;
`docs/experiments/wp21_DISPATCH.md` §4 and its rev-6→rev-7 diff;
`tools/stage3_allocator_bound.py`, `tools/baseline_snapshot.sh:480-490`;
`configs/bench_wp18c_solver_on.toml`;
`crates/pistol-cli/tests/fixtures/bench_solver_positions_v1.txt`;
`artifacts/wp20b_cap_RECEIPT.txt`, `wp20b_cap_SUMMARY.txt`, `wp20b_perf_dry_run.txt`
and all six `wp20b_cap_out_*` outputs. **D-401 was not read.**

**Ran.**

- **Re-derived the win/loss proof split independently** from the six exported
  `wp20b_cap_out_*` files (`/usr/bin/grep` + `sed` + `LC_ALL=C sort -u`). Every cell
  reproduces.
- **`sha256sum -c artifacts/wp20b_cap_RECEIPT.txt`** — twelve files, twelve `OK`.
- **Ran §9's registered command block VERBATIM**, byte-for-byte as it appears in
  `c3757db`, with `artifacts/pistol_prechange_a56449b` at the registered seat. **This
  is the first blocking finding.**
- Ran the same block with `tools/baseline_snapshot.sh:485`'s actual `%%` and confirmed
  it reproduces `artifacts/wp20b_perf_dry_run.txt`'s twenty `bestmove` lines exactly.
- **Timed the registered workload**: five entries at `nodes 50000` on the registered
  seat, 25.05 s → 5.01 s/search → **16.7 min for 200 searches**.
- **Composed `search_nodes` across the two caps** from the same exported artifacts.
  **This is the second blocking finding.**
- Verified the D-522/D-535 supersession, the D-249 quotation, the fixture's 20 entries,
  the ~116 firings/ask mean (694/6 = 115.67), and the header's ID-citation claim by
  grep over the document.

**Could not check.** The 22.99 µs fold cost and the 24 000 (position, symmetry) pairs —
still inherited from the DECISION-RED-TEAM with no artifact in any receipt, as rev 5 and
rev 6 both recorded. D-560's ~119 800 ceiling and the 2.14x duplication are marked
ESTIMATED and inherited. I did not run `tools/ci.sh` or `tools/determinism.sh`; no code
has changed.

---

## VERDICT: **FALLS**

### Direct answer to the standing question

**Revision 7 is not landable, and the reason is narrow and specific: both blocking
findings are in §1.1 and §9, both are one paragraph of document work, neither needs a
new measurement, and neither reopens a decision.** Everything an implementer would
actually build — §2's C2 selection, §3's T1+T3, §4's wire format, §5's coldness
argument, §6's diff table, §6.1's arming rule, §6.2's artifact spec, §7's eight
invariants and §8's eighteen tests with their seven call-removed mutants — survived
every attack I could mount, as it did for the rev-6 reviewer. I found no new defect in
any of it.

**What blocks it:**

- **AC1 — §9's registered command block still measures the empty board when run
  verbatim.** The comment-line filter was added and works. The tail filter was
  transcribed as `${line%%%% #*}` — **four** percent signs — where
  `tools/baseline_snapshot.sh:485`, which the same block cites by line number two lines
  above (and misquotes identically), is `${1%% #*}`. Bash parses `%%%% #*` as the
  operator `%%` with the pattern `%% #*`, which matches nothing in the fixture, so the
  ` # anchor …` tail survives. **I ran the block verbatim: 20 of 20 entries refused, 20
  of 20 searches on the empty board, 20 × `bestmove 0,0`, exit 0.** This is Z2's exact
  failure mode, unchanged, in the same registered instrument. The document's own
  MEASURED claim on the line above — *"0 of 20 with them"* — is falsified by the block
  printed beneath it.
- **AC2 — §1.1 prices the cap on two axes and leaves the third, which is the axis F3
  says makes the decision the operator's, unread in §1.1's own exported artifacts.** The
  cap changes the LABEL. `search_nodes` on identical positions at an identical budget
  moves **40 162 → 2 123 (18.9x)** on corpus entry 1 between cap 2048 and cap 16384; the
  share of the node budget the search itself receives is **0.78 %–10.04 % at cap 2048
  and 0.27 %–3.72 % at cap 16384**. F3's stated collapse range, *"0.8 %–10 %"*, is
  exactly the cap-2048 range presented without its cap, and §1.1 — the next section, the
  one that exported the 16384 data — never mentions the label. The tranche-one proposal
  (*"tranche one should run BOTH caps … for the price of one tranche"*), already carried
  into `wp21_DISPATCH.md` §4.4 as the operative recommendation, inherits it: a tranche
  split across two caps ships a production label corpus half of which was labelled with
  up to 19x less search.

**Is that a defect in the DESIGN or in the DOCUMENT?** AC1 is a **document** defect —
two characters — but it is a document defect in a *registered instrument*, and the
document's own sentence (*"an edit to the block reopens this registration"*) is what
stops an implementer from fixing it in place. AC2 is a **design** defect in the narrow
sense that matters: the recommendation it makes to the operator is priced on an
incomplete axis set, and the missing axis is measured in the artifacts the same section
exports.

**Neither costs a run.** AC1 is `%%%%` → `%%` in two places plus one confirmation that
the printed block reproduces the exported dry run. AC2 is one paragraph in §1.1, one
clause in §4.4, and a `grep` over files that are already digested in the receipt.

---

## Z1 AND Z2 REPAIR AUDIT

### Z1 — DISCHARGED, and the withdrawal is the right call

**My independent derivation**, from the exported outputs, without reference to
`wp20b_cap_SUMMARY.txt`:

```
/usr/bin/grep 'trigger_census: row' <f> | /usr/bin/grep -c 'att_proved true'
/usr/bin/grep 'trigger_census: row' <f> | /usr/bin/grep -c 'def_proved true'
/usr/bin/grep 'trigger_census: row' <f> | sed 's/ att_visits.*//' | LC_ALL=C sort -u | wc -l
```

| fixture | cap | rows | **win (`att_proved`)** | **loss (`def_proved`)** | distinct |
|---|---|---|---|---|---|
| trigger-rich | 2 048 | 294 | **0** | **4** | 49 |
| trigger-rich | 16 384 | 41 | **1** | **0** | 25 |
| corpus | 2 048 | 400 | 0 | 0 | 26 |
| corpus | 16 384 | 63 | 0 | 0 | 12 |
| both | OFF | 0 | 0 | 0 | 0 |

**Every cell reproduces.** §1.1's *"0 win-proofs in 694 firings at cap 2048, 1 in 104 at
cap 16384"* is exact (294+400 = 694; 41+63 = 104). `wp20b_cap_SUMMARY.txt` now carries
`win_proof` and `loss_proof` as separate columns with the D-522 non-summing rule stated
in its own derivation header, and all six of its rows match the raw outputs in all eight
columns.

**Is `att_proved` the right predicate?** Yes. `tools/stage3_allocator_bound.py:133-136`
is the shipped definition — `won(row) = row["att_proved"] == "true"`, *"The ATTACKER
direction proved… the only one this instrument counts (D-522)"* — and D-537's registered
quantity is *"WIN-PROVING FIRINGS ON DISJOINT POSITIONS"*, whose own parenthesis names
the direction correction (*"the denominator D-532 had to correct twice, once for
positions and once for direction"*). The design reads it correctly.

**Is D-522's prohibition correctly read?** The prohibition is, the citation is not
current — see **AE2**. D-535 says *"**THIS SUPERSEDES D-522**"* by name and retires the
wins-only narrowing *for the gate*; what it expressly preserves is the half §1.1 uses
(*"the two directions are different quantities, they are reported separately, and a
row's LOSS-side recall is not evidence about its WIN-side recall or the reverse"*). So
the non-summing rule holds and the conclusion is unaffected. The clause *"which is the
gate's direction"* is false under D-535, and the same document cites D-535 correctly at
§4. No conclusion moves; the citation should.

### **Is withdrawing the recommendation the right call, or has it over-corrected?**

**Withdrawing is right, and the document is if anything too generous to the small cap.**
Three reasons, in ascending order of weight:

1. **The n = 1 numerator carries no evidence, and the document says so correctly.** One
   win-proof distributed over 798 firings lands in the 104-firing group with probability
   104/798 ≈ 0.13 under the null of equal rates. A one-sided p ≈ 0.13 is not a signal,
   and §1.1's *"that is n = 1 and carries no rate"* is the honest reading.
2. **But D-537's quantity is not a ratio of the two limbs, and the composed quantity is
   measured directly and reads 0 vs 1.** D-537 registers *"a registered minimum of
   win-proving firings on disjoint positions"* — a single count, deduped by position,
   not a numerator over a denominator (see **AE5**). At matched compute the two arms
   produced **0 and 1**. The 2x denominator advantage only converts into D-537's
   quantity through a proof rate the small cap measured at exactly zero, with a 95 %
   upper bound of ≈ 3/694 = 0.43 %.
3. **And the one win-proof is mechanistically out of the small cap's reach, which the
   document did not check and which I did.** The proving row is
   `artifacts/wp20b_cap_out_trigger-rich_on_16384.txt:34`, `att_visits 11040
   att_proved true`. **11 040 > 2 048**: at cap 2048 that call would have been cut off
   at 2 048 visits and could not have returned a proof. So the sample's only win-proof
   is not a lucky draw at the large cap — it is a proof the small cap structurally could
   not find, which is D-530's *"`g001-t42-p2` at cap >= 16384"* mechanism showing up a
   second time in this sample's own bytes.

So a 2x denominator against an n = 1 numerator **does** leave the question open, and the
withdrawal is correct. What over-corrects in the other direction is nothing in §1.1 —
the section is now scrupulous. The residual is that the document could have made point
(3) in one grep and did not, which is a D-291-shaped miss (*"an estimate that could have
been measured in seconds is a finding"*) that happens to run in the document's own
favour. Recorded as **AE6**, not held against the verdict.

### Z2 — **NOT DISCHARGED**

**What I ran**, reconstructed byte-for-byte from `git show c3757db:…wp20b_design.md`
lines 909-921, `$BIN = artifacts/pistol_prechange_a56449b`, `$FIXTURE` the registered
one, seat `configs/bench_wp18c_solver_on.toml`, `REPS=1`, one arm, at `nodes 2000`:

```
grep -v '^#' "$FIXTURE" | grep . | while read -r line; do
  position="${line%%%% #*}"
  printf 'newgame\nposition %s\n%s\n' "$position" "$arm"
done | "$BIN" --config configs/bench_wp18c_solver_on.toml
```

```
exit                    0
refusals               20     (all on stdout: error Protocol: expected `q,r`)
bestmove lines         20
bestmove 0,0           20     (every search on the empty board)
```

**The block's own arithmetic, pinned at the byte level in the reviewed object:**

```
$ git show c3757db:docs/experiments/wp20b_design.md | sed -n '914p' | od -c
… l i n e % % % % _ # * } " \n
$ line='0,0 p1 1,0 p2 # anchor g001 turn 36 mover p2'
$ printf '[%s]\n' "${line%%%% #*}"   → [0,0 p1 1,0 p2 # anchor g001 turn 36 mover p2]
$ printf '[%s]\n' "${line%% #*}"     → [0,0 p1 1,0 p2]
```

`tools/baseline_snapshot.sh:485` is `tail_of() { printf '%s' "${1%% #*}"; }` — two
percent signs. The document doubles them in **both** places: in the executable line and
in the comment that quotes the cited source line.

**With `%%` restored, the block works and is the block that was dry-run.** I ran the
corrected version at `nodes 2000` on the registered seat: **0 refusals, 20 bestmoves, 0
empty-board bestmoves**, and `diff` of its twenty `bestmove` lines against
`artifacts/wp20b_perf_dry_run.txt` is **empty**. So the author ran the right thing and
printed the wrong thing.

**That is precisely what makes it blocking rather than cosmetic.**
`docs/process.md` §"Dry-run discipline": *"A pre-registration's **literal commands** are
exercised before its review passes."* The literal commands were not. The dry run
exercised a different command and the receipt it produced cannot be attributed to the
registered block. Run as registered, the guard produces a full set of nps and
time-to-depth numbers in both arms at a ratio of ≈1.000, passes its own 0.95 abort, and
reports a green perf guard over a measurement of nothing — `docs/process.md`'s named
vacuity, which §1 of this same document invokes against F3, and §3.1's own thesis (a
correct writer whose call is never made, indistinguishable from the receipt) one layer
up inside the section that registers the number.

**Two things that mitigate it and one that does not.** The 20 refusals go to **stdout**,
not stderr, so they land in any captured record; the block's own trailing comment says
*"A refusal on any line VOIDS the run"*; and the empty-board `info totals` lines carry no
`solver_*` fields at all, which is visibly different from a real search. So an attentive
operator catches it. What does not mitigate it: the document's own
*"an edit to the block reopens this registration"* means the implementer cannot silently
correct it, and the document misquotes the source line in the comment too, so the block
is not self-correcting from its own text — only from `tools/baseline_snapshot.sh:485`,
which it does at least cite by line number.

### On the dry run's own honesty — the one-armed question, answered

**The one-armed dry run is legitimate as far as it goes, is labelled precisely, and is
not the gap.** `docs/process.md` requires the literal commands *"on an input of the SAME
KIND as the registered workload — the same sort of artefact, differing only in
identity"*, and names ATTRIBUTION as what a real instance of the kind exercises. The
registered fixture at a smaller budget is the same kind; the missing arm is missing
because the token does not exist at the pre-change revision, and the document says so in
its own words rather than letting a reader assume both arms ran. The residual risk — that
the ON arm's `go` line is refused and the ON arm produces no searches — is pinned by
tests 5 and 6, and a refusal of the whole `go` line produces *zero* bestmoves rather than
a plausible-looking number, so it is not the silent class.

**What the document should add, and it is one line:** the dry run is **re-taken with both
arms at the closure revision**, where the token exists and it costs seconds, before the
governed run. §9 already declares *"INSTRUMENT REVISION: the closure HEAD at which the
guard runs"*, so the instrument's governing revision is by construction later than this
one; a two-armed dry run there is the natural discharge and closes the only attribution
the one-armed run cannot reach. Recorded as **AE3**.

**The real gap is not the arm count. It is that the arm that DID run was not the arm the
document registers.**

---

## DISCHARGE TABLE — Z1–Z2, AA1–AA4, AB1–AB5

| # | status | verified at |
|---|---|---|
| **Z1** — `proofs` column sums both solver directions | **DISCHARGED** | §1.1's table splits `win-proofs` / `loss-proofs`; the split reproduces exactly on my own derivation (0/4, 1/0, 0/0, 0/0); the predicate is the shipped `tools/stage3_allocator_bound.py:133`; the recommendation is **WITHDRAWN** in §1.1, in `docs/decisions.md` D-563 (*"the sample holds ZERO win-direction proofs in 694 firings at cap 2048 and ONE in 104"*), and in `wp21_DISPATCH.md` §4.4 (*"the measurement does NOT settle"*). Three sites, one story — the multi-site discipline W2's repair established. Citation residue at **AE2**. |
| **Z2** — the registered block searches the empty board | **NOT DISCHARGED** | Run verbatim from `c3757db`: 20/20 refused, 20/20 empty board, exit 0. See **AC1**. |
| **AA1** — COST registers 12 positions, ~25 min, for a 20-entry fixture | **DISCHARGED** | §9 now reads *"2 arms x **20 positions** x 5 reps … ~16 minutes"*. The fixture has 20 entries (`/usr/bin/grep -v '^#' \| /usr/bin/grep -c .`). I measured the registered workload: 5 searches at `nodes 50000` on the registered seat = 25.05 s → 5.01 s/search → **16.7 min** for 200 searches. The number is right. Its stated provenance is not — **AE1**. |
| **AA2** — an abort with no bracket is a rule-5 departure | **DISCHARGED AS WRITTEN; ITS NEW RATIONALE IS FALSE** | §9 registers *"H1 = `1.000x`, A NO-CHANGE HYPOTHESIS"* citing D-249 verbatim and correctly, plus the 0.95 abort. Rule 5's three elements are all present. But the paragraph written to make `1.000x` *"falsifiable rather than decorative"* argues the opposite of the truth. See **AD1**. |
| **AA3** — `wp20b_cap_SUMMARY.txt` outside the digest receipt | **DISCHARGED** | `sha256sum -c artifacts/wp20b_cap_RECEIPT.txt` → **twelve files, twelve `OK`**, including `wp20b_cap_SUMMARY.txt` and `wp20b_perf_dry_run.txt`. `/usr/bin/grep -c RECEIPT` on the receipt → **0**: it no longer digests itself. Clean. |
| **AA4** — the header names the wrong revision, the wrong review count, and cites no finding IDs | **HALF DISCHARGED** | Revision **7** ✓; *"**Six** fresh-context reviews"* naming all six files ✓; *"The review of revision 7 is outstanding"* ✓. The ID half is **worse, not better**: the header now claims *"findings are cited by their IDs (B/M/N, R, P/Q/S, T/U/V, W/X/Y, Z/AA/AB)"* and a grep over the whole document returns **only `T1`, `T2`, `T3`** — nothing from five of the six families. See **AD2**. |
| **AB1** — *"every existing caller stay as they are"* still false | **NOT DISCHARGED** | `:736`, verbatim unchanged. `ask` is private with one caller (`capture.rs:255`), which §6's own `capture.rs (run)` row says must pass the sink. The intended claim is about the *return* contract. One clause. |
| **AB2** — the root site's guard is identical, its capture is not | **NOT DISCHARGED** | `:629`, verbatim unchanged: *"the root site has the identical shape (`search.rs:304-307`)"*. The guard is identical; the root closure captures `&mut self.position` and calls `root_census_columns`, where the tree closure captures `state`. **This one costs implementation time** — an implementer told the shape is identical will look for a `state` capture that is not there. |
| **AB3** — *"for the same machine time"* mixes node-matched with time-matched | **DISCHARGED** | The phrase is gone from the document (`/usr/bin/grep -c 'same machine time'` → 0); the withdrawal rewrote the sentence that carried it. Incidental, but discharged. |
| **AB4** — `census.rs:41-58` is the struct body, not the argument | **NOT DISCHARGED** | `:222` and `:588`, verbatim unchanged. The doc comment at `:36-40` and the temporal ordering at `pvs.rs:616-620` are the stronger support the rev-6 reviewer verified. Cosmetic. |
| **AB5** — is the `info census ` prefix part of the file's payload? | **NOT DISCHARGED** | `:743`, verbatim unchanged. This fixes the `# body_sha256` payload digest **and** test 14's oracle, so two implementers would build differently from it. One clause; either answer is defensible. **Settle this before implementation, not in another revision.** |

**Discharged: 5 (one half, one incidentally). Not discharged: 5, of which one is
blocking.**

Revision 7 addressed every BLOCKING and MAJOR finding and left four of the five MINORs
verbatim. That is a defensible prioritisation under D-424 for AB1 and AB4, which change
no conclusion. It is not defensible for **AB2 and AB5**, which change what an implementer
builds.

---

## NEW FINDINGS

### BLOCKING

#### AC1 — §9's registered block, run verbatim, refuses 20 of 20 entries and searches the empty board; the dry run therefore did not exercise the literal commands

Derived in full in the Z2 audit above. In short:

- The block prints `position="${line%%%% #*}"`. Bash reads the operator as `%%` and the
  pattern as `%% #*`, which matches nothing in the fixture. The ` # anchor …` tail
  survives and every line is refused.
- All 20 fixture entries carry that tail (`/usr/bin/grep -v '^#' | /usr/bin/grep -c ' #'`
  → 20).
- I ran it verbatim: **20 refusals, 20 empty-board searches, 20 × `bestmove 0,0`,
  exit 0.** The document's own line above it — *"MEASURED: 8 of 8 sampled lines refused
  without the filters, **0 of 20 with them**"* — is falsified by the block printed
  beneath it.
- The comment quoting `tools/baseline_snapshot.sh:485` carries the same doubling, so
  the block cannot be repaired from the document's own text — only from the source line
  it cites.
- With `%%` restored the block reproduces `artifacts/wp20b_perf_dry_run.txt`'s twenty
  `bestmove` lines exactly, which establishes both that the author ran the correct thing
  and that the registered thing is not it.

**Why blocking and not a typo.** `docs/process.md` requires *"the **literal** commands"*
to be exercised, and this registration's whole content is which positions the two arms
search. Run as registered the guard is `docs/process.md`'s named vacuity — a criterion
the defect preserves — and the document itself declares *"an edit to the block reopens
this registration"*, so an implementer cannot fix it in place. **The repair is two
characters in two places, plus one line confirming that the printed block reproduces the
exported dry run.**

#### AC2 — §1.1 prices the cap on two axes; the third is the label, it is the axis F3 makes the operator's, and it is in §1.1's own exported artifacts unread

**The measurement, composed from the six files `artifacts/wp20b_cap_RECEIPT.txt` already
digests**, `search_nodes` as a share of the `--nodes 400000` budget:

| entry | cap 2 048 | cap 16 384 | collapse |
|---|---|---|---|
| trigger-rich 0 | 15 619 (3.90 %) | 14 889 (3.72 %) | 1.05x |
| trigger-rich 1 | 5 464 (1.37 %) | 4 716 (1.18 %) | 1.16x |
| trigger-rich 2 | 24 614 (6.15 %) | 6 926 (1.73 %) | **3.55x** |
| corpus 0 | 13 882 (3.47 %) | 3 752 (0.94 %) | **3.70x** |
| corpus 1 | 40 162 (**10.04 %**) | 2 123 (0.53 %) | **18.9x** |
| corpus 2 | 3 138 (0.78 %) | 1 097 (**0.27 %**) | **2.86x** |

Three things follow, and each of them is a conclusion the document currently licenses
wrongly:

1. **F3's stated range is a cap-2048 range presented without its cap.** *"MEASURED at
   `nodes 400000` on identical positions, the gate-on search receives **0.8 %–10 %** of
   its own node budget"* is exactly 0.78 %–10.04 %, the six cap-2048 cells. §1.1 exported
   the 16384 cells in the same run and they extend the range to **0.27 %–3.72 %**, a
   floor 2.9x below the one F3 states. F3's own conclusion — *"the label's DEPTH
   collapses"* — gets stronger, and its stated number is wrong.
2. **§1.1's limb 1 is true and is read as more than it says.** *"The price is flat in the
   cap"* is established for **wall time and `solver_nodes`** and for nothing else. The
   LABEL is not flat in the cap — it moves by up to 18.9x on one position — and §1.1's
   four limbs never mention it, in a document whose §1 F3 is entirely about the solver's
   budget absorption changing labels. *"The cap does not change the price"* is now the
   headline of the ONE LINE FOR THE MORNING and of `wp21_DISPATCH.md` §4.4.
3. **The tranche-one proposal inherits it.** *"Tranche one should run BOTH caps … for the
   price of one tranche"* is the design's recommendation and is already the operative
   text of `wp21_DISPATCH.md` §4.4 (*"the cap is set at tranche one, which runs both"*).
   WP-2.1's tranches produce the **production label corpus** — D-562(3)'s *"census ON
   from game one"* rides the same games — so a two-cap tranche ships a corpus whose two
   halves were labelled at materially different effective search depths, by two
   instruments, with nothing in the design naming it.

**I am not claiming the labels get worse at the large cap.** The direction is genuinely
unestablished: fewer, deeper solver calls can yield exact mate scores, which are better
labels than a deeper alpha-beta score. **That is the point.** The document establishes
the price axis and the yield axis by measurement, and leaves the label axis — the axis
whose measurement is the reason F3 says the whole decision is the operator's — unmeasured
in magnitude *and* unmeasured in direction, while the magnitude sits in files it digested
and cited.

**Why blocking.** §10.1 correctly keeps the ruling with the operator, and §1.1 is what
the operator rules from. A cap decision framed as *"price flat, denominator vs numerator,
run both at tranche one"* is a different decision from one that also says *"and the cap
sets how much of the label budget the label itself gets, by up to 19x, in a direction
nobody has measured."* This is the design's own D-291 standard (an estimate measurable in
seconds is a finding) and its own §10.8 standard (a design does not get to settle on its
own authority what the operator rules on) applied to the paragraph that does the ruling.

**The repair costs no run**: one paragraph in §1.1 adding the label axis with the table
above, a correction of F3's `0.8 %–10 %` to name its cap, and either a qualification of
*"for the price of one tranche"* or its withdrawal — mirrored in `wp21_DISPATCH.md` §4.4,
which is where the operator reads it.

### MAJOR

#### AD1 — the leak-detection rationale for H1 = `1.000x` is backwards: a cost that leaks outside the guard is paid by BOTH arms and moves the ratio TOWARD 1.000

§9, the paragraph written to make the bracket falsifiable:

> *"**WHAT A `1.000x` BRACKET CATCHES HERE** … **a cost that leaks outside the guard.**
> If any part of the census path is paid on the token-OFF arm — a key computed before
> the `is_some()` test, a formatting call hoisted, a `Vec` allocated per search
> regardless — the two arms stop being the same computation and the ratio moves off
> 1.000."*

**It does the opposite.** The two arms are one binary at one seat differing only in the
`go` line's third word. A cost paid *regardless of the token* is paid on the ON arm too.
Writing S for the base search, C for the correctly-guarded census cost and L for the
leak:

- no leak: OFF = S, ON = S + C, ratio = S/(S+C)
- leak: OFF = S + L, ON = S + L + C', ratio = (S+L)/(S+L+C') — **closer to 1.000**

Each of the three examples the document names is common-mode by construction: a key
computed before the `is_some()` test is computed on both arms; a hoisted formatting call
runs on both arms; a `Vec` allocated per search regardless is allocated on both. **The
registered criterion is one the named defect class PRESERVES** — `docs/process.md`
§"Criterion and defect class", the rule the same document invokes against F3 in §1 and
which the rev-6 reviewer invoked against the empty-board block.

**D-249's `1.000x` is falsifiable because its instrument is exact and CROSS-REVISION.**
Its adjudication is *"the baseline snapshot's INVARIANT block being byte-identical
excluding the `revision` line, `binary_sha256` included"*, and what it catches is *"a
solver crate that quietly enters the shipped binary [and] changes the digest"* — a
comparison against a different build, which does not share the suspect input. §9
transplants the number onto a **within-revision, between-arms, noisy timing ratio**,
where the same defect cancels. The precedent's *form* is used correctly (rule 5's third
element is owed and a no-change hypothesis discharges it); the precedent's *mechanism* is
not.

**What is not broken.** The defect class is still excluded — by **test 10**
(`the_non_census_path_does_not_compute_a_canonical_key`) and **test 17**
(`the_fold_is_entered_exactly_once_per_firing`), structurally and at any budget, exactly
as §9 says two paragraphs later (*"THE STRUCTURAL CHECK (test 17) IS WHAT ACTUALLY
EXCLUDES THE DEFECT CLASS"*). So this is MAJOR and not blocking: no defect escapes, and
nothing an implementer builds changes. What is false is the sentence telling a successor
that the bench excludes a class it structurally cannot see.

**The repair, and it makes the bracket genuinely falsifiable.** The externally derived
referent `docs/process.md` asks for first is already exported and already used by §9's
byte-identity obligation: **`artifacts/pistol_prechange_a56449b`**. Register H1 =
`1.000x` on the **post-change OFF arm against the pre-change OFF arm** at the same seat
and fixture — a cross-revision comparison that does not share the census code, and which
a leak moves off 1.000 — and keep the ON/OFF comparison for the 0.95 gross-regression
abort. That is D-249's actual shape, at no extra cost: the pre-change binary is already
on disk and the seat and fixture are already registered.

#### AD2 — the header claims six families of finding IDs are cited at their sites; only `T1`/`T2`/`T3` appear anywhere in the document

`:16-17`: *"findings are cited by their IDs (B/M/N, R, P/Q/S, T/U/V, W/X/Y, Z/AA/AB)"*.

```
$ /usr/bin/grep -oE '(^|[^A-Za-z])(Z[0-9]|AA[0-9]|AB[0-9]|W[0-9]|X[0-9]|Y[0-9])([^0-9A-Za-z]|$)' \
    docs/experiments/wp20b_design.md
(no output)
$ /usr/bin/grep -oE '\b(T[1-3])\b' … | LC_ALL=C sort | uniq -c
      8 T1     2 T2     6 T3
```

AA4 named this and revision 7 fixed the revision number and the review count while
**broadening** the false half: the rev-6 header claimed one practice it did not follow,
this one claims six. The stated cost stands — a successor cannot map a paragraph to the
finding it answers, in a document whose seven rounds are its main evidence of care, and
this review's own discharge table had to be built by reading the rev-6 report rather than
the design. Hard rule 10's drift clause and D-423 are what this is about. **One line:
either cite them, or delete the clause.** The clause is the cheaper deletion and D-424
supports it — but a claim that is simply false on its own face is not something a
reviewer can leave standing.

#### AD3 — the tranche-one proposal registers two caps that are not the two caps §1.1 measured

§1.1: *"running its two halves at **512 and 16384** answers the numerator on the sweep's
own positions at the sweep's own scale."* §1.1's measurement compared **2 048 and
16 384**. Cap 512 is `configs/gate_staged_solver_v0.toml:48`'s value and appears in no
row of §1.1's table, in no exported artifact, and in no receipt.

So the run proposed to settle the question neither replicates nor extends the measurement
that motivates it: the *"~2x more distinct positions at the small cap"* figure that
justifies caring about the small cap is a **2 048** figure, and a 512 arm inherits it only
by extrapolation across a 4x cap change — which is the move D-291 makes a finding, and
which §1.1 itself refused to make one section earlier (*"this revision measured it rather
than extrapolating (D-291)"*).

`wp21_DISPATCH.md` §4.4 now says *"the cap is set at tranche one, which runs both"*
without naming which two, so the ambiguity reaches the operator's decision text. **Either
name 2 048 and 16 384, matching the data, or say in one clause why 512 is the arm worth
buying and that its denominator figure is unmeasured.**

#### AD4 — "for the price of one tranche" states the cost and not the power, when the composition is one multiplication from numbers in §2

§1.1 establishes the requirement itself — *"distinguishing two caps' win-proof rates
needs on the order of **thousands of firings per cap**"* — and then proposes to buy both
caps *"for the price of one tranche"* without saying whether **half** a tranche clears
that bar. Splitting a tranche across two caps halves each arm's firings; the cost claim is
true and the power claim is unmade.

**It is very probably fine, and the document has the numbers to say so.** §2 measures
**~116 firings per ask** (694 over six searches, mean 115.67 — I reproduced it) and D-560
estimates ~256 000 label asks for the whole sweep, so even a small tranche yields firings
in the 10^5–10^6 range against a requirement of 10^3. **That is one line of arithmetic
from two numbers already on the page**, and its absence is the difference between a
proposal a reviewer must take on faith and one that is checked. D-291's standard again.

**And it composes with §9's other tranche-one obligation imperfectly, which should be
said.** §9 registers *"TRANCHE ONE EMITS `key_pos` BESIDE THE CANONICAL KEY, and the two
distinct counts are compared"* — a measurement of the symmetry fold's **in-tree** yield.
The in-tree population is cap-dependent (firings collapse 7.17x/6.35x between the two
caps), so a two-cap tranche measures the fold's yield on two different populations,
**neither of which is the sweep's population** — because the sweep's cap is what tranche
one is being asked to decide. The obligations do not conflict, but the design should say
which half answers the `key_pos` comparison, or that both are reported and the winning
cap's half is the one that governs.

### MINOR

#### AE1 — the ~16 minute COST is correct and its stated provenance is not

§9: *"~16 minutes **measured against the dry run's per-search rate at this seat**"*. The
dry run is at `nodes 2000`: I reran it and it completes 20 searches in **7.26 s**, i.e.
0.36 s/search, which gives 200 searches ≈ **1.2 minutes**, not 16. The ~16 minutes comes
from a `nodes 50000` rate that is neither the dry run's nor exported anywhere. I measured
it — 5 searches, 25.05 s, 5.01 s/search, **16.7 min for 200** — so the number is right to
the digit and only its attribution is wrong. A successor who re-derives from the named
source lands 13x off. One clause; or export the five-entry timing.

#### AE2 — §1.1 cites D-522 three times as the standing ruling; D-535 supersedes it by name, and the same document cites D-535 correctly at §4

`docs/decisions.md` D-535: *"**THIS SUPERSEDES D-522** … the wins-only narrowing was
post-hoc and is retired"*, ruling that *"THE CENSUS GATE RANKS BOTH DIRECTIONS PER D-512
AS REGISTERED"*. What D-535 expressly preserves is the half §1.1 relies on — *"the two
directions are different quantities, they are reported separately, and a row's LOSS-side
recall is not evidence about its WIN-side recall or the reverse"* — so **the non-summing
rule holds and §1.1's conclusion does not move**. What does not survive is §1.1's clause
*"**Win-proof** is the ATTACKER direction, **which is the gate's direction** and the only
one D-537's *win-proving* names"*. The second half is true (D-537's own word); the first
half is what D-535 retired. `wp20b_design.md:583` cites D-535 as *"the standing ruling"*
370 lines later, so the document knows.

Recorded as minor because no conclusion changes — but it is worth naming that the
citation was inherited verbatim from the rev-6 report, which also cited D-522 as standing
without checking its successor. That is the rev-6 diagnosis (*"repairs validated against
the review, not against the world"*) surviving one level down, on a `grep` of the same ADR
file the same paragraph cites four times.

#### AE3 — register the two-armed dry run at the closure revision

Covered in the Z2 audit. The one-armed dry run is legitimate and honestly labelled; the
ON arm cannot exist at the pre-change revision; §9 already declares the instrument's
revision to be the closure HEAD, where both arms exist and a second dry run costs seconds.
One line closes the only attribution the current dry run cannot reach.

#### AE4 — §9's instrument revision is a revision no review has seen

§9: *"**INSTRUMENT REVISION: the closure HEAD at which the guard runs**, restated in the
closure receipt."* CLAUDE.md: *"a pre-registration is reviewed at the revision that
GOVERNS the run — that revision must itself pass a fresh-context review before the first
run it governs."* As written, the block's governing revision is by construction later
than any revision reviewed. §9's registered REVIEW-impl at the closure revision is
probably the discharge; saying so in the same sentence costs a clause and closes a
process gap a successor would otherwise have to reason about.

#### AE5 — D-537's quantity is a single count, not a numerator over a denominator

§1.1: *"D-537's quantity is a **numerator over a denominator** — win-proving firings, on
disjoint positions."* D-537 registers *"a registered minimum of **win-proving firings on
disjoint positions**"* — a count of win-proving firings with same-position firings
counted once. That is one number, not a ratio. The framing is harmless in the direction
§1.1 uses it (the two factors genuinely do pull opposite ways) but it invites a successor
to read the distinct-signature count as a denominator that divides something, when its
role is to cap how many positions can contribute at all.

#### AE6 — the one win-proof's `att_visits` settles more than n = 1 does, and the check is one grep

`artifacts/wp20b_cap_out_trigger-rich_on_16384.txt:34` carries `att_visits 11040
att_proved true`. **11 040 > 2 048**: the small cap could not have found this proof at
all. That converts the numerator observation from *"n = 1, no rate"* into *"n = 1, and
mechanistically out of the small cap's reach"*, which is D-530's mechanism reappearing in
this sample's own bytes rather than as an external corroboration. It strengthens the
withdrawal §1.1 already makes, and it is one grep over a file the receipt already digests.
Not held against the verdict; recorded because §1.1 is otherwise scrupulous about
composing what its artifacts hold.

#### AE7 — carried from rev 6, unrepaired: AB1, AB2, AB4, AB5

Listed in the discharge table. **AB2 and AB5 change what an implementer builds** and
should be settled before implementation rather than in another revision; AB1 and AB4 are
cosmetic and D-424 covers leaving them.

---

## THE STRONGEST SURVIVING ATTACK ON REVISION 7

**Revision 7 was told to stop validating its repairs against the review text and to run
things against the world. It did — and it ran the world it had been pointed at. Both of
this round's blocking findings are one column and two characters outside the frame the
previous review drew.**

The rev-6 diagnosis was *"its repairs are validated against the review, not against the
world"*, and the prescription was one line: **run the thing.** Revision 7 ran the thing.
It re-derived the proof split on the exported rows, exported a corrected summary, put it
in a regenerated receipt that verifies twelve-for-twelve, took a real dry run whose twenty
`bestmove` lines I reproduced byte-for-byte, and carried the withdrawal into all three
documents that stated the recommendation. That is a genuine change of method and it shows.

**But the verification stopped at the boundary of what the previous reviewer had already
touched, twice, in the same two artifacts.**

- The rev-6 reviewer ran the block from *their own shell*, so the author fixed the block
  *in their own shell* — and the block in the **document** was never run. `%%%%` survives
  in the executable line and in the comment quoting the source line, and both were typed
  rather than pasted from `tools/baseline_snapshot.sh:485`. The dry run's own exported
  bytes are the proof that the working command existed; the registered text is the proof
  that it was not the one registered. **The step not taken is copying the block out of the
  document and running that.**
- The rev-6 reviewer computed the proof columns, so the author computed the proof columns.
  The **`search_nodes`** column sits in the same six files, in the same `trigger_census:
  entry` lines the author read to sum `firings` and `solver_nodes`, one field to the left
  — and it is the column §1's own F3 argument runs on, the one that produced the
  *"0.8 %–10 %"* figure the document leads with. Reading it shows the cap moves the label's
  own budget by up to 18.9x and pushes F3's floor down to 0.27 %, which changes the shape
  of the decision §1.1 hands the operator and prices the tranche-one proposal the design
  is recommending into `wp21_DISPATCH.md`. **The step not taken is asking what else the
  artifact answers, rather than what the reviewer asked it.**

So the habit has moved, not resolved: revision 6 validated against the review's *text*,
revision 7 validated against the review's *scope*. The tell is identical in both findings
— the correct fact is present elsewhere in the same document (F3's label argument; the
`tools/baseline_snapshot.sh:485` citation two lines above the block) and is not composed
with the paragraph that needs it. D-423's *"a claim the document makes twice is a defect
waiting"* is the rule this keeps tripping, and here it trips in the useful direction: both
repairs are already written somewhere in the file.

**What this attack does not touch.** I could not break C2, T1+T3, F1, F2, F3's mechanism,
the arming rule, the seat rule, the eighteen tests, the seven call-removed mutants, the
byte-identity obligation, or §10's eight deferrals — the same list the rev-6 reviewer
could not break, now attacked from a different side. **An implementer could build §§2–8 as
written today.** What is not landable is §1.1's advice to the operator and §9's registered
instrument, and both are repairable in an hour without a single new measurement, from
artifacts that are already digested in a receipt that already verifies.
