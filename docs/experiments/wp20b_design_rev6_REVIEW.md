# WP-2.0b DESIGN revision 6 — REVIEW-design (fresh context)

## Header

**Revision reviewed:** `dd0b588d93e1baacc268362aa7aa9f99ea10f778` — a `git stash create`
object on top of `dev` HEAD `a56449b`. **The tree still matches at the end of this
review**: `git diff dd0b588 --stat` is empty on entry and on completion, `git status
--porcelain` carries the same nine paths throughout, and `git worktree list` shows the
main tree only. I created no worktree and modified no file but this one.

I did not write this document and I am not any of the five reviewers it answers.

**Read.** `docs/experiments/wp20b_design.md` in full (1007 lines);
`wp20b_design_rev5_REVIEW.md` in full; `wp20b_design_rev2_REVIEW.md`,
`wp20b_decision_REDTEAM.md`, `wp20b_design_rev3_REVIEW.md`,
`wp20b_design_rev4_REVIEW.md` at the findings they carry; `CLAUDE.md`;
`docs/process.md` §"Dry-run discipline" and §"Cost, replication, and the second
instrument" in full; `docs/decisions.md` D-249, D-465, D-516, D-517, **D-522**, D-523,
D-527, D-530, D-531, D-533, D-537, D-538, D-563, D-564; `docs/experiments/wp21_DISPATCH.md`
§4; `crates/pistol-search/src/census.rs`, `crates/pistol-search/src/pvs.rs`
(`solver_verdict`/`observe`), `crates/pistol-search/src/search.rs`,
`crates/pistol-arena/src/capture.rs`, `capture_file.rs`, `passes.rs`,
`crates/pistol-cli/src/protocol.rs`, `crates/pistol-cli/src/corpus/emit.rs`,
`tools/baseline_snapshot.sh`, `tools/stage3_allocator_bound.py`;
`artifacts/wp20b_cap_RECEIPT.txt`, `wp20b_cap_SUMMARY.txt`, `wp20b_cap_time_a56449b.log`
and all six `wp20b_cap_out_*` outputs. **D-401 was not read.**

**Ran.**

- **Re-derived the distinct-signature counts independently**, four ways, from the
  exported per-entry rows (`/usr/bin/grep` + `sed` + `LC_ALL=C sort -u`).
- **Separated the `proofs` column by solver direction** over the same rows. This is the
  finding.
- Re-derived every other cell of §1.1 (raw firings, `solver_nodes`, wall) from the
  exported artifacts and against `wp20b_cap_SUMMARY.txt`.
- **Ran §9's command block verbatim** over its own registered fixture, using the
  exported pre-change binary `artifacts/pistol_prechange_a56449b` at the registered seat
  `configs/bench_wp18c_solver_on.toml`. This is the second finding.
- Ran the same block with `tools/baseline_snapshot.sh`'s own `entries()`/`tail_of`
  filtering applied, at `nodes 50000`, and timed it — which both prices §9's run and
  corroborates D-517's 9.05.
- Verified ~30 source, config, tool and fixture citations at their lines.

**Could not check.** The 22.99 µs fold cost and the 24 000 (position, symmetry) pairs —
both inherited from the DECISION-RED-TEAM with no artifact in any receipt, as rev 5 also
recorded. D-560's ~119 800 ceiling and the 2.14x duplication are marked ESTIMATED and
inherited. I did not run `tools/ci.sh` or `tools/determinism.sh`; no code has changed.

---

## VERDICT: **FALLS**

**Answer to the standing question.** Revision 6 is not landable, and I do not say that
lightly after five rounds. But the two things that block it are not polish, they are not
"another reviewer's taste", and neither costs a run:

- **Z1.** §1.1 fixed the *disjoint positions* half of W1 and left the *win-proving* half
  in the same defect, inside the sentence that claims to fix it. The `proofs` column sums
  the attacker and defender directions — which **D-522 forbids by name**, in a line
  written because `tools/stage3_census_rank.py` revision 1 made exactly this sum — and
  the summed figure is then offered as D-537's registered quantity. **Separated on the
  design's own exported artifacts, the sample holds ZERO win-direction proofs at cap 2048
  against ONE at cap 16384.** The design's *"it holds four proofs against one"* is, on the
  quantity it names in the same clause, four losses against one win. This is now in
  `docs/decisions.md` D-563 as well.
- **Z2.** §9's newly-printed command block does not work. I ran it verbatim: **all 36
  lines of its own registered fixture are refused, all 36 searches run on the EMPTY BOARD,
  every one returns `bestmove 0,0`, and the pipeline exits 0.** The registered guard would
  produce a complete set of nps and time-to-depth numbers in both arms, measuring nothing,
  at a ratio of ≈1.000 that passes its own 0.95 abort. The dry run whose entire purpose is
  to catch this is written in the future tense and recorded nowhere.

**Everything else in this document is landable and most of it is good.** The C2 and
T1+T3 selections survive; §4's wire format, §5's coldness argument, §6's diff table,
§6.1's arming rule, §6.2's artifact spec, §7's invariants and §8's eighteen tests with
their seven call-removed mutants are correct as far as I could check them, and I found no
new defect in any of them. Thirteen of the fifteen carried findings are discharged, three
of them (W2, X4, Y4) exactly as asked. §1.1's distinct-signature work is real, reproduces
to the digit, and — as I show below — the author chose the signature variant *least*
favourable to their own recommendation, which is the right instinct.

**What must change, and it is a short list.** Split §1.1's proofs column by direction and
restate the two sentences that rest on it, in the design and in D-563 (the data is already
exported; this is minutes). Paste `entries()`/`tail_of` into §9's block, correct its
position count, run the `nodes 2000` dry run, and record its input and output. Register a
bracket or name the rule-5 departure (AA2). Fix the header (AA4). Nothing here needs a new
measurement and nothing here reopens a decision.

---

## W1 REPAIR AUDIT

### 1. The distinct counts, re-derived independently

The design's `distinct` column counts distinct `TriggerColumns` signatures — the emitted
row with the `att_*`/`def_*` fields removed. I re-derived it from the exported outputs
without reference to `wp20b_cap_SUMMARY.txt`:

```
/usr/bin/grep 'trigger_census: row' <file> | sed 's/ att_visits.*//' | LC_ALL=C sort -u | wc -l
```

| fixture | cap | rows (mine) | design `distinct` | **mine** | ratio |
|---|---|---|---|---|---|
| trigger-rich | 2 048 | 294 | 49 | **49** | — |
| trigger-rich | 16 384 | 41 | 25 | **25** | **1.96x** |
| corpus | 2 048 | 400 | 26 | **26** | — |
| corpus | 16 384 | 63 | 12 | **12** | **2.17x** |

**Every cell reproduces exactly.** So do the shares the design quotes — 49/294 = 16.67 %,
26/400 = 6.5 % — and the raw ratios 294/41 = 7.17, 400/63 = 6.35, and the `solver_nodes`
sums 1 160 027 / 1 233 841 / 1 148 739 / 1 236 133, and the wall column against
`wp20b_cap_time_a56449b.log`. `wp20b_cap_SUMMARY.txt`'s six rows match the raw outputs in
every column.

### 2. Is the signature the right one? Yes, and I checked it at the source

The design excludes the answer fields *"because those are cap-dependent OUTCOMES and not
descriptors of the position at the decision, a split `crates/pistol-search/src/census.rs:41-58`
already makes."* That is correct and it is stronger than the citation given. The code does
not merely separate the two structs, it separates them **in time**, at the firing site,
with a comment saying why:

```
crates/pistol-search/src/pvs.rs:616  // The census columns are read HERE, before either call, because they
                                     // describe the position a per-node detector would decide on — after
                                     // the calls the same slices still answer the same way, but the order
                                     // is what makes that a fact about the decision rather than about its
                                     // outcome.
```

`census.rs:41-58` is the `TriggerColumns` struct body; the sentence that *argues* the
split is its doc comment at `:36-40` (*"they are read at a different MOMENT"*). Minor
(AB4), but the doc comment is the better citation.

### 3. Is the ratio robust to the signature choice? Yes — and the design chose conservatively

I computed three alternatives:

| signature | trig 2048 → 16384 | ratio | corpus 2048 → 16384 | ratio |
|---|---|---|---|---|
| **design's** (entry + `TriggerColumns`) | 49 → 25 | **1.96x** | 26 → 12 | **2.17x** |
| `TriggerColumns` only, entry dropped | 49 → 25 | 1.96x | 26 → 12 | 2.17x |
| `TriggerColumns` minus `turns_from_root` | 49 → 25 | 1.96x | 26 → 12 | 2.17x |
| **whole row, answers included** | 61 → 29 | **2.10x** | 105 → 33 | **3.18x** |

Dropping `entry` changes nothing, which means no signature is shared across the three
entries of a fixture — the proxy is not merging positions from different root positions.
Dropping `turns_from_root` changes nothing either. The **finer** signature (answers
included) is a strictly tighter lower bound on distinct positions — within one arm the
solver's answer is a deterministic function of the position and the cap, so adding the
answer columns can only split, never merge — and it moves the ratio **up**, to 2.10x and
3.18x.

**So the correction has not over-shot; if anything it under-shoots.** The design picked
the coarsest of the available signatures, which is the variant least favourable to its own
recommendation. The honest class is *"about 2x, and 2–3x under a finer signature"*, not
*"about 7x"*. Every variant is far from the raw ratio, so W1's central correction stands
and reproduces.

### 4. **The repair is half done, and the missing half inverts the sign of the only direct evidence**

D-537 counts *"WIN-PROVING FIRINGS on disjoint positions"*. Revision 6 corrected
*disjoint positions*. It did not correct *win-proving*, and this project has a standing
ADR about precisely that word.

**D-522**, verbatim:

> *"A trigger census also records `solve_defender`, which answers* does the OPPONENT force
> a win against the mover *and is a proven LOSS.* **`tools/stage3_census_rank.py` revision 1
> scored `att_proved or def_proved` and called the sum PROOFS KEPT**, *so the matrix's
> `opp_hot >= 3` row was chosen against a denominator the gate does not pin. … until it is
> taken,* **WINS is the definition in force and every table states both columns without
> summing them.**"

The committed instrument implements it:

```
tools/stage3_allocator_bound.py:133  def won(row):
                                     """The ATTACKER direction proved. The gate's direction, and the only one
                                     this instrument counts (D-522)."""
                                     return row["att_proved"] == "true"
```

§1.1's table has **one `proofs` column and it is the sum**. Separated on the exported
rows:

| fixture | cap | firings | **win-direction (`att_proved`)** | **loss-direction (`def_proved`)** | design's `proofs` |
|---|---|---|---|---|---|
| trigger-rich | 2 048 | 294 | **0** | **4** | 4 |
| trigger-rich | 16 384 | 41 | **1** | **0** | 1 |
| corpus | 2 048 | 400 | 0 | 0 | 0 |
| corpus | 16 384 | 63 | 0 | 0 | 0 |

All four cap-2048 proofs are `att_proved false … def_proved true`. The single cap-16384
proof is `att_proved true … def_asked false`. I verified the semantics at the site rather
than inferring them: `pvs.rs:672` sets `attacker.proved` from `solve_defender`'s sibling
call under the comment *"The attacker direction: does the MOVER force a policy-game win?"*,
and `pvs.rs:698` sets `defender.proved` under *"does the OPPONENT force a win against the
mover's best defense?"* — which is D-530's *"proven LOSS"*.

Three sentences in the document rest on the summed column, and each says something
different once it is split:

1. §1.1: *"**And the proof RATE moves the other way**: 4/294 = 1.36 % at cap 2048 against
   1/41 = 2.44 % at 16384."* Split: **win-rate 0 % → 2.44 %**, **loss-rate 1.36 % → 0 %**.
   Both move the same way as the summed figure, but neither is the summed figure and
   D-522 forbids the sum in a table by name.
2. §1.1: *"On D-537's own quantity — win-proving firings on disjoint positions — **this
   sample cannot give a ratio**: it holds four proofs against one."* On D-537's own
   quantity the sample holds **zero against one**. It still cannot give a ratio — but the
   sentence reads as *"the small cap looks 4x better and n is too small"*, and the truth is
   *"the small cap scored nothing and the large cap scored the sample's only one"*.
3. §1.1's recommendation: *"**The trade is more distinct positions against deeper
   proofs**, D-537's denominator is disjoint positions, and that is what makes the small
   cap the better buy for THIS purpose."* D-537's denominator is disjoint positions and
   its **numerator is win-proving firings** — and the numerator's only observation in this
   sample is at the large cap, in the direction D-530's mechanism predicts.

**The document contains its own correction, again, in its own voice.** §10.4: *"win-proofs
are a subset of that again, since D-530 records a search finding six proofs `and every one
is a proven LOSS`."* §10.4 knows the subset relation; §1.1 uses the superset as the
quantity D-537 counts.

### 5. Does ~2x support the small-cap recommendation?

**On the quantity measured, yes; on the quantity D-537 counts, the data supports no
recommendation in either direction, and the design should say that instead of "better
buy".**

- **Distinct positions:** small cap wins, 1.96x–2.17x node-matched, 1.64x–2.04x per second
  of wall (49/149 vs 25/125; 26/53 vs 12/50). Solid, reproduced, robust to the signature.
- **Win-proving firings:** 0 vs 1. Not a ratio; not evidence; and what little sign it has
  runs *against* the recommendation and *with* D-530's *"the one row that constrains a cap
  is `g001-t42-p2` at cap >= 16384"*.
- **D-537 counts the product**, and the design measured one factor well and the other not
  at all.

So the corrected number does not weaken the recommendation below the threshold at which
the *measurement* is worth reporting to the operator — it plainly is, it is the best data
anyone has, and §10.1 correctly keeps the ruling with the operator. It weakens the word
**"better buy"**, which asserts a verdict on the product. The repair is one paragraph: the
small cap is the better buy **on D-537's denominator**; on its numerator the sample's only
observation is at the large cap and D-530 gives the mechanism; the operator is choosing
between a measured 2x on positions and an unmeasured factor on proofs.

### 6. Is *"the proxy is coarse because census rows carry no position identity yet"* self-serving?

**No — it is verifiably true, correctly signed, and correctly answered.** `TriggerObservation`
(`census.rs:14-22`) carries `columns`, `attacker`, `defender` and no identity; adding one
is the whole of §2 and §6. The design declares the direction of the coarseness (*"distinct
signatures are a LOWER bound on distinct positions"*) and explicitly refuses to claim a
bias direction for the *ratio*, which is the honest position — my four-variant computation
above is the check that establishes it, and it is a check the design could have run and
did not.

The one thing I would add is that the reasoning is *only* non-self-serving because §9
registers tranche one to close it. It does, and it registers the right comparison (`key_pos`
beside the canonical key). Sound.

---

## DISCHARGE TABLE — W1–W4, X1–X6, Y1–Y5

| # | status | verified at |
|---|---|---|
| **W1** — firings substituted for D-537's quantity | **HALF DISCHARGED** | *Disjoint positions*: discharged and independently re-derived — 49/25/26/12 exact, 1.96x/2.17x, robust across four signature variants (§W1 REPAIR AUDIT). *Win-proving*: **not discharged** — the `proofs` column still sums both solver directions against D-522's *"every table states both columns without summing them"*, and split it is 0-vs-1, not 4-vs-1. See **Z1**. |
| **W2** — "second instrument" false | **DISCHARGED** | §1.1: *"That is a REPLICATION and not a second instrument, and the difference is not cosmetic"*, naming the shared stage (*"the same hand-rolled `SolverWiring` (`trigger_census.rs:125-126`), which bypasses the config path — the stage actually under doubt"*) and the consequence (*"raises confidence in the arithmetic and none in the wiring"*). `trigger_census.rs:125` is `solver: args.gate_on.then_some(SolverWiring {`, `:126` `per_call_node_cap: args.cap,` — exact. The `docs/process.md` quote is verbatim from §"Cost, replication, and the second instrument". **D-563 carries the same withdrawal**, in the MARKED LIMITS clause. Both files fixed. |
| **W3** — wrong ADR, wrong band | **DISCHARGED** | §9 now reads *"9.05 firings — D-517's trigger-rich figure, which supersedes D-516's 6.72 by name"*. D-517's bands are 18.33 / 11.75 / **9.05** and its third is trigger-rich ✓; `bench_solver_positions_v1.txt` is headed *"the TRIGGER-RICH stress class"* ✓, so §9's fixture and its number now match. 9.05 × 22.99 µs = 208 µs ≈ **0.21 ms** ✓. **I corroborated 9.05 directly**: the first three entries of that fixture at the registered seat and `nodes 50000` report `solver_firings` **9, 12, 11**. Residue at **AA1**. |
| **W4** — no command block | **NOT DISCHARGED** | A block is now printed, and **it does not work**. Run verbatim: 36/36 positions refused, 36/36 searches on the empty board, exit 0. The dry run is future-tense and unrecorded. See **Z2**. |
| **X1** — three ranges for one quantity | **DISCHARGED** | Head line `14.5x-36x` (`:5`), F3's table `14.5x` / `36.0x` (`:147-148`), D-563's title `A MEASURED 14.5x-36x`. §1.1 no longer computes an integer-denominator ratio at all. `wp21_DISPATCH.md:95` also says `14.5x-36x`. Four sites, one range. |
| **X2** — = W4 | **NOT DISCHARGED** | See **Z2**. It does not demote to a documentation gap this time: the block exists and is wrong, which is worse than absent. |
| **X3** — sink's route out; wrong file for the write | **DISCHARGED** | §6 gains a `passes.rs` row (*"the write's actual call site: `:43-56` calls `run`, `render` and `manifest_row`"*) and a `capture.rs (run)` row (*"`run`'s signature at `:242` carries the sink through from `ask`"*). Verified: `passes.rs:43` `crate::capture::run`, `:44` `render`, `:56` `manifest_row`; `capture.rs:242` `pub fn run(transcript, label_nodes)`. The `arena.rs` row is corrected to *"the `--capture` census flag only — this file delegates at `:73`"*; `arena.rs:73` is `Mode::Capture(source, nodes) => passes::capture(…)` ✓. Residue at **AB1**. |
| **X4** — not the arena's idiom; no body digest | **DISCHARGED** | §6.2 now specifies *"the `Fixture` header the sibling writers already produce — `# param` / `# derived` lines and a `# body_sha256` payload digest (`capture_file.rs:59-72` is the template and the one call site)"* and gives the digest its rule-8/D-469 reason. `capture_file.rs:59` `let mut fixture = Fixture::new(&[`, `:68-71` `fixture.param(…)`, `:72` `fixture.derived("capture_sha256", …)` ✓; `corpus/emit.rs:6` `pub const BODY_DIGEST: &str = "# body_sha256 ";` ✓; `manifest_row` at `capture_file.rs:106`, called at `passes.rs:56` ✓. Residue at **AB5**. |
| **X5** — the receipt's face contradicts the table | **DISCHARGED** | `artifacts/wp20b_cap_SUMMARY.txt` exists, states its own derivation and the reason the `RESULT` lines are broken (`bc` absent; the harness grepped `^census` where rows are `trigger_census: row `), names the per-entry lines as the record, and carries a derivation timestamp. **All six rows match the raw outputs in every column** — I re-derived firings, distinct, proofs and `solver_nodes` from the artifacts and wall from `wp20b_cap_time_a56449b.log`. §1.1 points at it. Residue at **AA3**. |
| **X6** — an unresolvable bracket | **DISCHARGED AS WRITTEN; NEW DEFECT** | The `[0.98, 1.02]` bracket is gone and §9 registers *"an ABORT and no bracket, because the instrument cannot resolve a bracket"* with the reason stated. But hard rule 5 names three items and two are shipped. Direct answer at **AA2**. |
| **Y1** — new off-by-two in `capture.rs` | **DISCHARGED** | §3.1 now cites `:165 pub fn classify`, `:172` the `INFO_PREFIX` test, `:173 return Step::Ignore;`, `:229 Step::Ignore => continue,`. All four exact. |
| **Y2** — `wp21_DISPATCH.md` §4 | **DISCHARGED** | §4 gains item 4, *"RULE ON THE LABELLING SEAT'S SOLVER GATE, AND ON ITS CAP"*, marked *"logically prior to (1) and (2), not an alternative to them"*, with the 14.5x-36x cost and the cap's ~2x, and a closing *"(4) is owed before (1) can deliver what it promises"*. This is exactly what §10.1 says blocks WP-2.1. |
| **Y3** — stub engine's behaviour unstated | **DISCHARGED** | §6's row: *"it REFUSES a census request by name rather than accepting one and returning no rows (rule 3). A stub that silently honours a request it cannot serve is §3.1's defect one layer down."* One clause, at the site, and it is the right clause. |
| **Y4** — no open question about the two scope departures | **DISCHARGED** | §10.8 added, and it says the load-bearing thing: *"a design does not get to retire two of its dispatch's four scope items on its own authority"*, with the v2-parenthesis observation and *"§2 re-opens rather than being amended"*. |
| **Y5** — two overstatements in §2 and §5 | **HALF DISCHARGED** | (a) **discharged**: §2 now reads *"adds no allocation of its own: it pays `canonical_form`'s transforms and the `collect` that feeds them — `Board::stones()` (`board.rs:91`) yields an iterator and `canonical_form` (`symmetry.rs:165`) takes a slice — both of which C1 pays too"*. (b) **not discharged**, verbatim unchanged: §5 still says *"the root site has the identical shape (`search.rs:304-307`)"*. See **AB2**. |

**13 discharged (two of them half), 2 not.**

---

## NEW FINDINGS

### BLOCKING

#### Z1 — §1.1's `proofs` column sums the two solver directions, which D-522 forbids by name, and split it says the opposite of what the sentence implies

Derived in full in the W1 REPAIR AUDIT §4. In short:

- D-522 is a standing ruling: *"WINS is the definition in force and every table states
  both columns without summing them"*, written because a committed tool made this exact
  sum and a matrix row was selected against it.
- `tools/stage3_allocator_bound.py:133-136` implements the definition: `won(row) =
  row["att_proved"] == "true"`, *"the only one this instrument counts (D-522)"*.
- Split on the design's own exported artifacts: **cap 2048 → 0 win-direction proofs, 4
  loss-direction; cap 16384 → 1 win-direction, 0 loss-direction.**
- §1.1's *"On D-537's own quantity … it holds four proofs against one"* therefore states
  four *losses* against one *win* while naming the win quantity, and §1.1's *"the proof
  RATE moves the other way, 1.36 % → 2.44 %"* is a rate over the forbidden sum.
- **This is now in `docs/decisions.md` D-563** (*"the sample gives four proofs against one
  and cannot carry a ratio at all"*), so the fix reaches two files, exactly as W2's did.

**Why blocking rather than major.** W1 was blocking because §1.1's headline is what the
operator acts on across 38–95 days of machine time. The repair corrected one of the two
substitutions W1 named and left the other, and the one left is the one a standing ADR
already litigated — the review is not asking for a distinction the project has not made,
it is asking for a distinction the project made, wrote down, and built a tool around. And
the correction changes the offer: *"more distinct positions against deeper proofs, and the
sample's only win-proof is at the large cap"* is a materially different trade from *"four
proofs against one"*.

**The repair is minutes and needs no run.** Split the column into `win_proofs` /
`loss_proofs` in the §1.1 table and in `wp20b_cap_SUMMARY.txt` (the data is in the exported
rows; the greps are in this report's header), restate the two sentences, and narrow *"the
better buy for THIS purpose"* to the denominator. §10.4 already carries the vocabulary.

#### Z2 — §9's registered command block measures the empty board, in both arms, at exit 0 — and the dry run that would have caught it is unrecorded

**What I ran**, verbatim from §9, `$BIN` = `artifacts/pistol_prechange_a56449b`,
`$FIXTURE` = the registered `crates/pistol-cli/tests/fixtures/bench_solver_positions_v1.txt`,
one arm, one rep, at the registered seat:

```
while read -r position; do
  printf 'newgame\nposition %s\n%s\n' "$position" "$arm"
done < "$FIXTURE" | "$BIN" --config configs/bench_wp18c_solver_on.toml
```

**Result: 36 `position` verbs, 36 refusals, 36 searches, 36 × `bestmove 0,0`, exit 0.**

Two independent reasons, and the block misses both:

1. The fixture's ten comment lines and its blanks are sent as positions —
   `error Protocol: expected `start` or `set`, got `#``.
2. **Every data line is refused too** — `error Protocol: expected `q,r`` — because each
   carries a trailing ` # anchor g001 turn 36 mover p2` commentary tail.

`tools/baseline_snapshot.sh`, which §9 cites two lines below the block for the `newgame`
rule, has both filters and says why it has them:

```
tools/baseline_snapshot.sh:484  entries() { grep -v '^#' "$1" | grep . || true; }
tools/baseline_snapshot.sh:485  tail_of() { printf '%s' "${1%% #*}"; }
tools/baseline_snapshot.sh:615  printf 'newgame\nposition %s\ngo nodes %s\n' "$(tail_of "$entry")" "$NODES" …
```

**Why this is blocking and not a typo.** The refusal does not abort the session and does
not stop the search: `newgame` has already reset the board, so `go` searches the empty
position and prints a complete, well-formed `info … nps … time …` and `info totals` for
it. Both arms do this identically — the census token can never fire on an empty board —
so the ON/OFF nps ratio comes back at ≈1.000, **passes the 0.95 abort**, and the closure
reports a green perf guard over a measurement of nothing. That is `docs/process.md`'s
named vacuity (*"a criterion that is a property the named defect class PRESERVES"*), which
§1 invokes against F3, and it is §3.1's own thesis — a correct writer whose call is never
made, indistinguishable from the receipt — one layer up, inside the section that registers
the number.

**And the dry run is the second half of the finding.** §9: *"The block's literal commands
**are exercised before this registration's review passes** … and the dry run's input and
its output **are recorded with the pre-registration**"*. `docs/process.md` §"Dry-run
discipline": *"The pre-registration records the dry-run input and its output."* Neither is
in the document and nothing in `artifacts/` matches (`ls artifacts | grep -i dry` returns
eight other packages' dry runs and none of this one's). A `nodes 2000` dry run over 20
entries costs seconds and would have printed 36 `error` lines on its first screen. W4/X2
asked for the block *and* the dry run; the block arrived broken and the dry run did not
arrive.

**The fix, demonstrated.** With `entries()`/`tail_of` applied I ran the first three entries
at `nodes 50000` on the registered seat: **zero refusals**, real searches
(`nodes 51717 / 53859 / 51451`, `solver_firings 9 / 12 / 11`, `nps 12120 / 9683 / 11854`,
`time 4267 / 5561 / 4340` ms). So the block is three characters of `sed`/`grep` away from
being the instrument it claims to be, and once fixed it does produce per-position nps and
`time`, from which time-to-depth is derivable off the per-depth `info` lines.

**One thing the fix does not supply, and §9 should**: the block redirects nowhere, pairs
nothing, and computes no IQR. It generates the numbers on stdout and names no derivation
step for the *"Report per position, paired across arms"* and the *"IQR gate"* its own
trailing comments register. A registered instrument whose output is not captured cannot
produce a receipt.

### MAJOR

#### AA1 — §9's COST registers 12 positions for a 20-entry fixture, which is W3's band confusion one paragraph below W3's own repair

`crates/pistol-cli/tests/fixtures/bench_solver_positions_v1.txt` has **20 entries**
(`grep -v '^#' | grep -c .`). §9's COST reads *"2 arms x **12 positions** x 5 reps on an
armed seat, ~25 minutes"*. Twelve is the **corpus band** count — D-517's own shape is
*"3 of 12 / 5 of 12 / 12 of 20 searches"*, where 20 is trigger-rich and 12 is a corpus
band, and `bench_positions_v1.txt` has 24 entries in two bands of twelve.

This is precisely the substitution W3 named — the corpus band's number taken for the
trigger-rich fixture — surviving into the paragraph after the one that fixed it. It is
MAJOR rather than blocking because it does not change what is measured, only what the
document says the run costs, and `docs/process.md` makes that cost statement a registered
element (*"the proportion between the document and the run is visible on the document's own
face"*). Measured from my `nodes 50000` timing (~4.8 s per search on this seat), the real
run is 2 × 20 × 5 = 200 searches ≈ **16 minutes**, so ~25 min survives as an envelope for
20 positions and was a 2.6x over-estimate for 12 — the arithmetic behind the number does
not reconcile either way.

#### AA2 — registering an abort with no bracket is a hard-rule-5 departure, taken on the design's own authority, when this repository already has the precedented third option

**Direct answer to the sharpest question: yes, the fix traded one rule violation for
another — but a smaller one, and the repair is a single line rather than a revision.**

Hard rule 5 names three registered elements: *"a pre-registered hotspot, expected gain
bracket, and abort threshold"*. §9 ships the hotspot and the abort and deletes the third,
stating a reason. Three observations, in order of weight:

1. **The third option exists and this project has already taken it.** D-249: *"WP-1.5a's
   PRE-REGISTRATION REGISTERS H1 AS EXACTLY `1.000x`, A NO-CHANGE HYPOTHESIS … **That is
   falsifiable and it is not a placeholder**: what a 1.000x bracket catches is an
   ACCIDENTAL DEPENDENCY EDGE … and it is written down here because a reader meeting
   `1.000x` in a rule-5 bracket will otherwise read it as a number someone forgot to fill
   in."* That is X6's problem solved, in the log, with the successor-confusion hazard §9
   worries about answered explicitly. A census change gated off by default has the same
   shape: the predicted ratio *is* 1.000 and what a 1.000x bracket catches is the fold
   escaping its guard onto the non-census arm.
2. **The no-bracket argument is in tension with the instrument §9 registers.** The reason
   given is the 10 % IQR gate — a statement about *within-band* spread. But the block
   registers *"Report per position, **paired across arms**"*, and pairing is exactly the
   technique that resolves effects below a band's IQR. A paired 12-or-20-position, 5-rep
   comparison can see a 2 % shift that the band IQR cannot, so *"the instrument cannot
   resolve a bracket"* is asserted against an instrument the same paragraph designs to
   resolve better than that.
3. **Rule 5's own last clause is the one that covers this and it is not cited.** *"A
   measured structural floor is a finding, not a failure."* §9 has measured (well, derived
   from D-517 × 22.99 µs, and I corroborated the firing count) that the change's cost is
   ~0.21 ms against a ~4.3 s search — three orders below the instrument's floor. That is a
   structural floor, rule 5 says it is a finding, and saying so is a better sentence than
   deleting a registered element.

**Why MAJOR and not blocking.** It changes nothing an implementer builds and nothing a
successor may conclude about the change's performance: the 0.95 abort plus test 17 carry
the whole exclusion, and §9 is admirably explicit about the gap between them (*"a fold
correctly placed and merely slower than 22.99 us … nothing covers it finely"*). But a
design may not delete an element of a hard rule on its own authority — that is §10.8's own
principle, which the same document applies correctly to the dispatch's scope and not to
CLAUDE.md's rule 5. Either register D-249's no-change bracket, or add the departure to
§10 as a line the operator rules on. One line, either way.

#### AA3 — the corrected summary that is now the document of record for §1.1's table is not in the digest receipt

`artifacts/wp20b_cap_RECEIPT.txt` indexes ten files. `artifacts/wp20b_cap_SUMMARY.txt` is
not one of them (`grep -c SUMMARY` → 0), and it was written after the receipt. §1.1 cites
it as *"the corrected summary and its derivation"*, D-563 cites the `wp20b_cap_*`
receipts, and the file now carries the only assembled form of the `distinct` column that
the design's central inference rests on.

Rule 8 and D-469 are the reason X5 was raised in the first place, and the fix for a
receipt whose face was wrong should not itself land outside the receipt. Every number in
it verifies against the raw outputs — I checked all six rows in all six columns — so
nothing is *wrong*; what is missing is the digest that makes a successor able to establish
that without re-deriving it, which is the whole function of the receipt. Re-issue the
receipt over eleven files.

#### AA4 — the document's own face says it is revision 5 and that revision 5's review is outstanding, and cites none of the findings it answers

- `:1` — *"DESIGN, **revision 5**."*
- `:12-15` — *"**Four** fresh-context reviews stand behind this document"*, naming rev 2,
  the RED-TEAM, rev 3 and rev 4, then *"**The review of revision 5 is outstanding.**"*
- `wp20b_design_rev5_REVIEW.md` is in the repository, in the same uncommitted diff, and
  five reviews stand behind this document.
- **No W, X or Y identifier appears anywhere in the design** (`grep -oE '\b(W[1-4]|X[1-6]|Y[1-5])\b'`
  returns nothing), while T1–T3 from the rev-4 round are cited at their sites.

The document's stated practice is *"findings are cited by their IDs"*, and revision 6 is
the round that stopped doing it. The cost is traceability: a successor cannot see which of
revision 6's paragraphs answer which finding, is told a review is outstanding that is on
disk beside the file, and — since the process rule is that *a pre-registration is reviewed
at the revision that GOVERNS the run* — is left with a document that names the wrong
revision for the review that governs it. Mechanical, but it is the kind of mechanical that
D-423 and hard rule 10 exist about.

### MINOR

#### AB1 — §6.2's *"every existing caller stay as they are"* is still false and §6's own table contradicts it two rows above

§6.2: *"**`ask` gains a `&mut Vec<String>` sink** (`capture.rs:181`) rather than a changed
return type, so its `Result<(String, String), _>` contract and **every existing caller stay
as they are**."* `ask` is private (`capture.rs:181 fn ask(`) with exactly one caller
(`capture.rs:255`, inside `run`), and that caller must pass the sink — which §6's new
`capture.rs (run)` row says outright. X3's structural half is discharged; this is the
sentence X3 quoted as surviving verbatim, and it still does. The intended claim is about
the *return* contract; say that.

#### AB2 — Y5(b) unrepaired: the root site's guard is identical, its capture is not

§5, unchanged: *"`canonical_form` needs the stone list and gets it from the closure's own
captured `state` via `state.board().stones()`; the root site has the identical shape
(`crates/pistol-search/src/search.rs:304-307`)."* Verified at the site:

```
crates/pistol-search/src/search.rs:304   let root_columns = self
                                   :305       .census
                                   :306       .is_some()
                                   :307       .then(|| root_census_columns(&mut self.position));
```

The **guard** is identical. The **capture** is not — the root closure captures
`&mut self.position` and calls `root_census_columns`, where the tree closure destructures
and captures `state` (`pvs.rs:604`). An implementer told the shape is identical will look
for a `state` capture that is not there and will have to solve the stone-list access
differently at the root. One clause.

#### AB3 — *"more distinct positions for the same machine time, by a factor of roughly two"* mixes a node-matched ratio with a time-matched claim

1.96x and 2.17x are node-matched (both arms spend `--nodes 400000`). Per second of wall
they are 49/149 vs 25/125 = **1.64x** and 26/53 vs 12/50 = **2.04x**, because §1.1's own
limb 1 records that cap 16384 is slightly *cheaper*. *"Roughly two"* survives; the phrase
*"for the same machine time"* attached to the node-matched number does not, and the
direction of the slip is against the recommendation.

#### AB4 — `census.rs:41-58` is the struct body; the sentence that argues the split is its doc comment

§1.1 cites *"a split `crates/pistol-search/src/census.rs:41-58` already makes"*. `:41-58`
is `#[derive(…)] pub struct TriggerColumns { … pub cover: CoverClass`. The doc comment that
*makes the argument* — *"Separate from `TriggerObservation` because they are read at a
different MOMENT: these describe the decision, the answers describe its outcome"* — is
`:36-40`, and `pvs.rs:616-620` states the temporal ordering at the firing site. Both are
stronger support for the signature choice than the struct body. Widen the citation.

#### AB5 — §6.2 asks the census file for a `Fixture` header *and* *"the same bytes the wire carried"*, and does not say whether the row keeps its `info census ` prefix

The wire line is `info census key <32 hex> …` (§4). §6.2 specifies *"one row per line in
§4's field order, **the same bytes the wire carried**"*. Under a `Fixture` header with a
`# body_sha256` over the payload, whether the `info census ` prefix is part of the payload
is load-bearing twice: it fixes the digest, and it fixes what test 14's oracle compares.
One clause settles it; either answer is defensible.

---

## THE STRONGEST SURVIVING ATTACK ON REVISION 6

**Revision 6 was told the document was written outward from the instrument's output rather
than inward from the claim, and it fixed the two instances the reviewer had already
computed — while both of the round's own new failures are that same habit at sites the
reviewer had not reached.**

The rev-5 review handed revision 6 a worked repair: it printed the distinct-signature
table, it printed 1.96x and 2.17x, it named D-517 and 9.05, it quoted `docs/process.md` on
replication. Revision 6 took all four and installed them correctly. Every finding whose
answer was *in the previous review's text* is discharged, several of them well — W2's
withdrawal is exactly right and reaches D-563, Y4's §10.8 says the load-bearing thing about
authority, X4 found the real idiom and the digest that makes it auditable.

**Every finding whose answer required going one step past the previous review's text is
still open, and both are the same step.** W1 said *"D-537 does not count firings"* and
computed the disjointness half; revision 6 corrected disjointness and left *win-proving*
summed across both solver directions — against **D-522**, a ruling written because a
committed tool made this exact sum, whose own words are *"every table states both columns
without summing them"*, and which §10.4 of this same document paraphrases correctly two
pages later. W4 said *"§9 asserts a command block it does not contain"*; revision 6 supplied
a block that runs its own registered fixture into thirty-six refusals and thirty-six
empty-board searches, and did not run the dry run that exists to catch precisely that,
while writing a paragraph saying the dry run had been run.

So the diagnosis has to be sharpened rather than repeated. It is not that the document
reads its instrument instead of its claim — revision 6 demonstrably can read a claim when
someone has already extracted it. It is that **the document's repairs are validated against
the review text and not against the world**. The distinct-signature repair was checked
against the reviewer's numbers and not against D-522, which is one `grep` away in the same
ADR file that the same paragraph cites four times. The command block was checked against
the reviewer's complaint — *"there is no block"* — and not against the fixture, which is one
`bash` away and which I ran in eleven seconds. In both cases the design has the correct
sentence somewhere else in its own body (§10.4; the `tools/baseline_snapshot.sh` citation
two lines under the block), and in both cases it is the *verification step against
something outside the review* that did not happen.

**That is a fixable habit and it is nearly fixed.** This is the first revision whose
substance I could attack only by running things, and I could not break C2, T1+T3, the
arming rule, the seat rule, the mutant table, or the byte-identity obligation. What
revision 7 owes is small and mechanical, and the discipline it owes is one line long: **run
the thing.** The 0-vs-1 proof split and the 36 refusals were each about sixty seconds of
work over artifacts and binaries already sitting in `artifacts/`, and both of them change
what the operator is being asked to decide.
