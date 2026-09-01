# WP-2.0b DESIGN revision 4 — REVIEW-design (fresh context)

## Header

**Revision reviewed:** `22d1d26cdca5623614c95fd30974a2ed1deeb5b2`. This is a
`git stash create` object, not a commit on any branch; it holds the uncommitted
work sitting on top of HEAD `a56449b` ("docs(wp20): the closure head's gate
receipt joins the manifest"). `git diff 22d1d26` was empty at the start of this
review and empty at its end, so **the live working tree still matches the
reviewed revision**. Files never `git add`ed are not in a stash-create object;
`git status` shows the five WP-2.0b documents staged (`A`) plus two modified
tracked files, so all of the reviewed material is inside it.

**I did not write this document** and am not the author of
`wp20b_design_rev2_REVIEW.md`, `wp20b_decision_REDTEAM.md` or
`wp20b_design_rev3_REVIEW.md`.

**Read in full:** `docs/experiments/wp20b_design.md` (revision 4, 1130 lines);
`wp20b_design_rev3_REVIEW.md`; the Q- and S-finding bodies of that report;
`CLAUDE.md`; `docs/process.md` (all 79 lines); `docs/experiments/wp21_DISPATCH.md`
§4 and §5; `docs/decisions.md` D-465, D-508, D-510, D-516, D-530, D-533, D-536,
D-563.

**Read at the tree, at this revision:** `crates/pistol-search/src/census.rs`,
`search.rs` (the census seam, `clear`, the root site, `push_root_census`, the
per-depth report, `:525`), `pvs.rs` (`:245-252`, `:598-640`, `:744-750`,
`:766-772`), `info.rs` (`SearchOutcome`); `crates/pistol-search/examples/trigger_census.rs`;
`crates/pistol-search/tests/trigger_census_cover_tests.rs`;
`crates/pistol-engine/src/engine.rs`, `instance.rs`, `lib.rs`, `position.rs`;
`crates/pistol-arena/src/capture.rs`, `capture_file.rs`, `passes.rs`, `labels.rs`,
`labels_file.rs`, `exchange.rs`; `crates/pistol-arena/tests/capture_tests.rs`,
`crates/pistol-arena/tests/config_tests.rs`;
`crates/pistol-engine/tests/config_schema_tests.rs`;
`crates/pistol-cli/src/budget_token.rs`, `report.rs`;
`crates/pistol-core/src/lib.rs`, `symmetry.rs`, `state.rs`;
`tools/bench_delta.sh`, `tools/baseline_snapshot.sh`, `tools/ci.sh`;
`configs/bench_wp18c_solver_on.toml`, `configs/instrument_v0.toml`,
`configs/gate_staged_solver_v0.toml`, `configs/arena_wp20_label_pilot.toml`;
`artifacts/prechange_*.txt`, `artifacts/wp20b_prechange_RECEIPT.txt`.

**Everything recorded below was produced with `git grep` or `/usr/bin/grep`,
never the harness-wrapped `grep` (D-265).** No file was modified except this
report. No build was run; no worktree was created.

**What I could not check.** (a) Whether the tests §8 names actually kill the
mutants §8 names — that is REVIEW-impl's and the mutation receipt's job, and
nothing is implemented yet. (b) The red team's own measurements (22.99 µs,
95–159 firings, 14.5×/36×, 24 000 fold pairs) — I did not re-run them; I checked
only the arithmetic done ON them and the provenance claims made ABOUT them.
(c) `D-401` was not read (binding instruction). (d) I did not run
`tools/bench_delta.sh` or `tools/baseline_snapshot.sh`; I read them.

---

## VERDICT: **FALLS**

Revision 4's premise is that it stopped transcribing and started deriving. On the
six numbers it puts on its own face, that premise mostly holds: **five of the six
are right** and I reproduced them independently, including two digests the
document cites and the config diff that makes §10.1's "a choice among existing
shapes" fair.

It falls anyway, and not on a number.

**T1 — the arming rule, the centrepiece of this revision, kills
`crates/pistol-search/examples/trigger_census.rs` at runtime.** That example is
the instrument that produced this document's own MEASURED volume basis (§2's
95–159 firings) and F3's MEASURED 14.5×–36×, and it is the instrument three
committed `tools/stage3_*.py` scripts parse. The design cites it three times as
evidence and never once as a file the diff touches. No CI gate runs it, so the
break is invisible until the next person needs the number. **The rule was adopted
verbatim from the rev-3 reviewer's suggested repair, and its callers were not
checked** — which is the rev-3 diagnosis (*"learned to fix the sentence a reviewer
quoted, and has not learned to derive the property"*) recurring for the third
consecutive round, this time on the paragraph written to end it.

**T2 — §9's re-registered bench guard names no instrument, and the only bench in
the tree cannot run it.** `tools/bench_delta.sh` pins `CONFIG="configs/instrument_v0.toml"`
at `:92`, varies the BINARY and not the `go` line, and carries the eval-delta
verdict rule. The design cites `tools/bench_delta.sh:95` as
`bench_wp18c_solver_on.toml`'s "own registered budget" — that line belongs to a
script that reads a different config. `docs/process.md`'s first section is
"Instrument governing revision"; this pre-registration has none.

**T3 — checks (i) and (ii) are one comparison reported twice.** Same seat, same
budget, same two arms, one bracket, one abort threshold — under a heading that
still reads "TWO CHECKS ON TWO NAMED SEATS". P4 asked exactly this question.

And there is **one new wrong number** (U1), in §4, transcribed word for word from
the rev-3 review rather than derived: the enumeration offered as the derivation of
"nine" lists ten items and says "four pairs" where `TriggerColumns` has three.
The bottom line is right; the derivation the revision exists to supply is wrong.

**None of this touches the two selections.** C2 and T1+T3 survive. F1, F2 and F3
survive. The identity fold, the byte arithmetic, the seat rule's *conclusion*, the
D-512 citation correction and the handshake deletion are all correct at the tree.
The code this package describes is very nearly designed. What is not landable is
the arming rule as written and the rule-5 obligation as registered.

---

## THE SIX-NUMBER AUDIT

| # | revision 4's claim | my independent derivation | verdict |
|---|---|---|---|
| 1 | `TriggerObservation` has **seven** sites outside its own module: `lib.rs:52`, `pvs.rs:76`, `search.rs:80`, `search.rs:759`, `search.rs:216`, `pvs.rs:746`, `search.rs:767` | `git grep -n TriggerObservation -- crates/` gives **9** lines; two are in `census.rs` (`:14` the definition, `:37` a doc reference). 9 − 2 = **7**, and the seven paths and line numbers match one for one. None requires `Copy`: two are `Option<Vec<…>>` fields, one a `&mut` parameter, one a return type, one a re-export, two are pushes of freshly-constructed values. | **AGREE** |
| 2 | proof rate = **four in 694 firings, 0.58 %**, from §2's own counts 95+97+102+134+159+107 | 95+97=192; +102=294; +134=428; +159=587; +107=**694**. 4/694 = 0.5764 % → **0.58 %**. §10.5's "well under 100 MB" also checks: 0.58 % of 8.7 GB ≈ 50 MB. | **AGREE** |
| 3 | §4 has **nine** count columns = fourteen fields − `key`, `cover`, two `*_proved`, `defender_visits` | The field list has **14** fields. 14 − 5 = **9**, and the nine are `TriggerColumns`' seven `u32`s (`turns_from_root` + three mover/opponent pairs) + `cover_count` + `attacker_visits` — which is the document's own second spelling and is correct. **But the enumeration the document offers as the derivation is wrong**: *"`turns_from_root`, the four `mover_*`/`opponent_*` pairs' seven columns, `cover_count` and `attacker_visits`"* is 1+7+1+1 = **ten**, and `crates/pistol-search/src/census.rs:44-58` has **three** mover/opponent pairs, not four. §4's D-512 paragraph independently repeats "four pairs" (`:664`). The phrase is copied verbatim from `wp20b_design_rev3_REVIEW.md`'s Q8. | **AGREE on 9, DISAGREE on the derivation** — see U1 |
| 4 | **0.60 ms predicted, abort above 2 ms**, from 26 firings (D-530) at `bench_wp18c_solver_on.toml`'s budget (`tools/bench_delta.sh:95`) × 22.99 µs | Arithmetic: 26 × 22.99 µs = 597.7 µs = **0.60 ms** ✓. **Seat:** `bench_wp18c_solver_on.toml` is real, cap 2048 at `:46`, gate on at `:45` ✓. **Budget citation:** `tools/bench_delta.sh:95` is `NODES=50000`, but `:92` is `CONFIG="configs/instrument_v0.toml"` and the header says *"The config is PINNED: configs/instrument_v0.toml"* — the cited line registers a budget for a DIFFERENT seat and coincides in value. The ON seat's own `nodes 50000` is registered in D-465, not there. **Firing count: wrong fixture.** D-530's counts (0, 0, 13, 13, 1, 1, **26**) are over the D-512 *value* fixture; 26 is that fixture's maximum, from `g001-t42-p2`. The bench seat's fixture is `bench_positions_v1.txt`, and a seat-and-budget-matched MEASURED firing count exists — D-516's **12.00 / 9.73 / 6.72 firings today**, taken on these two committed seats at `nodes 50000` (D-508). Using it gives ~0.28 ms. **Falsifiability: named but unreachable.** The defect class is named ("a fold that is not O(stones) … a hoist, an accidental per-node call, or a quadratic") and the exclusion is named, which `docs/process.md` §"Criterion and defect class" asks for. But no instrument is named, and the registered signal is ~0.03 % of an ON-seat search at that budget — see **T2**. | **DISAGREE** (arithmetic right; seat citation wrong, firing count from the wrong fixture, bound not falsifiable by any named instrument) |
| 5 | 14.5×–36× is a **lower bound at cap 2048**; a one-key arming of `instrument_v0.toml` gives **16384**; D-465 shows the cap is a **weak lever (~2.3× over a 64-fold change)**; therefore "probably within a small factor rather than an order above" | **Direction: right.** D-465: *"an ON-seat sweep over {32, 128, 512, 2048} moves the corpus ratio only 0.045 → 0.103 for a 64-fold cut, because a smaller cap leaves more budget for SEARCH nodes"* — larger cap ⇒ lower ratio ⇒ worse. **Strength: right as arithmetic**, 0.103/0.045 = 2.29 ≈ 2.3×. **The one-key claim: verified.** `diff` of the two configs with comments and blanks stripped and sorted yields exactly two differing lines — `on_search_path` and `per_call_node_cap` — so *"the two files differ in exactly those two keys"* is TRUE, and 16384 is at `instrument_v0.toml:114`. **But the conclusion is a new unmarked estimate.** {32,…,2048} does not contain 16384; the claim about 2048→16384 is an extrapolation *outside the swept range, in the direction not swept*, presented as what *"D-465's own sweep says"*. It is not marked ESTIMATED, in a document that lectures on D-291 two pages earlier. Two further problems: the design renames D-465's quantity *"the corpus **search-node** ratio"* — D-465 says "the corpus ratio" and D-508 identifies it as **the nps ratio**, and this repo has D-477 for exactly this substitution; and **D-530, the very ADR §9 cites for its 26 firings, carries a direct statement about cap 16384** (*"THE ONE ROW THAT CONSTRAINS A CAP IS `g001-t42-p2` AT cap >= 16384"*) that the design does not carry. | **PARTLY DISAGREE** — direction and 2.3× agree; "within a small factor rather than an order above" is an unmarked ESTIMATE, and the quantity is mislabelled |
| 6 | `SearchOutcome`, not `BestMove`: `crates/pistol-search/src/info.rs:237-245`; `git grep BestMove -- crates/` empty | `pub struct SearchOutcome {` is at `info.rs:237`, closing `}` at `:245`, fields `best: Turn`, `info: SearchInfo`, `provenance: Provenance` ✓. `git grep BestMove -- crates/` returns nothing ✓. (Repo-wide it appears only in `CLAUDE.md:62`'s prose sketch and in these WP-2.0b documents — worth knowing, and it does not weaken the claim, which is scoped to `crates/`.) | **AGREE** |

**Score: 3 clean agreements, 1 agreement with a broken derivation, 1 partial, 1
disagreement.** The revision's own premise — *"Every number in this revision is
DERIVED at the tree … and not copied from a report"* — is falsified by #3, where
the derivation is a verbatim copy of the rev-3 reviewer's own miscount, and
strained by #4, where the derivation crosses fixtures and cites the wrong file
for its budget.

**Two things I checked that revision 4 got right and that are worth recording**,
because they are the strongest evidence its method partly worked:

- **§9's pre-change digests reproduce.** `sha256sum` over
  `artifacts/prechange_gate_v0_run1.txt` up to and including the `# timing` marker
  gives `7f8a6f972c10…`, and `prechange_instrument_v0_run1.txt` gives
  `06490795663c…` — both exactly as cited, and run1 == run2 on both configs, so
  the invariance claim is MEASURED as stated. (One wrinkle: §9 says the referent
  is *"everything **above** the `# timing` marker"*; the digest that matches
  **includes** the marker line. Excluding it gives `f4bdfcd9…`. See V11.)
- **P1's mechanism is exactly right.** `tools/baseline_snapshot.sh:578` is
  `grep '^id '`, `:598` is `sed 's/^id /engine_id /'`, and both sit inside the
  `{ … } >>"$INVARIANT"` group that closes at `:610`.
  `artifacts/prechange_gate_v0_run1.txt` carries **ten** `engine_id` lines at
  `:6-15`, above the marker at `:57`. An eleventh would move the invariant digest
  on every seat with the token off. The reason given for deleting R14 is correct.

---

## DISCHARGE TABLE — P1–P4 and Q1–Q8

| finding | status | evidence |
|---|---|---|
| **P1** — delete the R14 handshake | **DISCHARGED** | Mechanism verified above (`baseline_snapshot.sh:578,598`, ten `engine_id` lines, marker at `:57`). Deletion is complete: `/usr/bin/grep -n "handshake\|R14\|advertis"` over the design returns 7 hits, all of them the deletion or a pointer to it (`:50` header, `:575-577` §3.1, `:687-702` §4, `:790` §6's `protocol.rs` row *"No handshake change"*). No test in §8 requires it; §7 has no invariant over it; §9's byte-identity bullet is consistent with it. The D-88 second reason (*"advertised budget kinds are **derived**, never restated"*) is a real second ground. |
| **P2** — derive the seat column | **PARTIAL** | The two rows the reviewer named are now right, and the reasoning is sound: test 5's mutant ("token check removed") survives on a gate-off seat because the empty census prints nothing either way, and test 10's closure is unreachable in both mutant and original. "Either" is gone. **But the stated rule does not cover four of the seventeen rows it is applied to.** The rule's only no-seat clause is *"a test whose subject is refused before a search ever starts"*, which fits tests 6 and 15 (grammar refusals) and does not fit tests 2, 3, 4, 11 — pure `pistol-core` fold tests that never start a search and are never refused. They are seated "pistol-core, no seat", which is the right answer, reached outside the rule. A rule derived to replace a list still needs a clause for the rows the list had. See **U7**. |
| **P3** — the arming rule | **NOT DISCHARGED** | The problem is real: `collect_trigger_census` (`search.rs:206`) arms, `take_trigger_census` (`:216`) *"leav[es] collection ON"* and panics if never started (`:220`), and `Searcher::clear` (`:230-239`) clears the table, the heuristics and the solver and **does not touch `self.census`** — verified line by line. §5's coldness proof would indeed stop being true. **The proposed rule breaks the tree.** `crates/pistol-search/examples/trigger_census.rs` calls `collect_trigger_census()` ONCE at `:167`, then inside its per-entry loop calls `engine.clear()` at `:195` (with a load-bearing comment: *"EVERY ENTRY IS A DIFFERENT GAME, so every entry starts cold"*) and `engine.take_trigger_census()` at `:215`. Under *"`Searcher::clear` sets `self.census = None`"* the first iteration panics. See **T1**. This crate's own tests survive — all three in `trigger_census_cover_tests.rs` (`:91/95`, `:106/110`, `:139/143`) build a fresh `searcher(512)` and never call `clear` between collect and take. So the answer to the dispatch's question is: the rule breaks the example and not the tests, and the example is the one that matters. |
| **P4** — the bench guard | **NOT DISCHARGED** | Arithmetic fixed (26 × 22.99 µs = 0.60 ms), and the criterion now names a defect class and its exclusion, which `docs/process.md` requires. But (i) and (ii) are **the same comparison**: (i) is "solver-ON, token-ON vs token-OFF, on `bench_wp18c_solver_on.toml` at `NODES=50000`"; (ii) is "**the same seat and budget**". Both the wire tax and the per-firing fold are paid under exactly one condition — token ON at a firing — so the single ON/OFF comparison measures their sum and no arm isolates either. One bracket and one abort threshold are registered for the pair. The heading still reads "**TWO CHECKS ON TWO NAMED SEATS**", a rev-3 sentence left standing after rev 4 moved (i) onto (ii)'s seat. See **T3**, and **T2** for the instrument. |
| **Q1** — the `Engine` seam | **PARTIAL** | Two implementors and three call sites verified exactly: `crates/pistol-engine/src/instance.rs:86`, `crates/pistol-arena/src/bin/stub_engine.rs:146`; `crates/pistol-cli/src/protocol.rs:172`, `crates/pistol-cli/tests/movetime_tests.rs:99`, `crates/pistol-engine/tests/engine_tests.rs:119`. `engine.rs:53` is `fn go` with the default body at `:54`; `:63` is the required `go_reporting` ✓. `stub_engine.rs` is now a §6 row ✓. **The third limb fails**: §6.1 says *"the field is added there and §6's `search.rs` row carries it"*, but `SearchOutcome` is defined in `crates/pistol-search/src/info.rs:237`, and §6's `info.rs` row says only *"**NOT** the per-depth `SearchInfo`"* — an exclusion, no addition. The `search.rs` row lists three other changes. The file that gains the field has a diff row that states what does not change. See **U3**. |
| **Q2** — `label_go_line` vs `go_line` | **DISCHARGED** | `capture.rs:112` `pub fn label_go_line(nodes: u64)` delegates to `BudgetSection::go_line()` at `:114`; `arena.rs:190` is the separate match-pass call. The committed test is at `capture_tests.rs:388-395` and asserts precisely `label_go_line(5_000) == BudgetSection::Nodes{value:5_000}.go_line()` — an equality the token breaks on the `label_go_line` seat under any spelling, and whose call also breaks if the signature gains a flag. §6 lists it as moving, and gives the right reason for not touching `go_line()` (it would put the census on every MATCH game). |
| **Q3** — the cap caveat | **PARTIAL** | See audit row 5. Direction and the "weak lever" arithmetic are right and the one-key config claim is verified. The extrapolation past the swept range is unmarked; the ratio is mislabelled; D-530's own cap-16384 sentence is not carried. And the caveat is now stated at length **twice**, in F3 and again in §10.1, in near-identical prose — D-423's exact shape, introduced by the paragraph that fixes Q3. |
| **Q4** — test 5 / test 16 scope contradiction | **DISCHARGED** | It dissolves with P1: with no handshake `id` line, test 5's "whole of that `go`'s output" has no census byte to trip over, and §8's row for test 5 now says *"asserts on the whole of the `go`'s output"* rather than the session's. Test 16 is repurposed to invariant 9. |
| **Q5** — the sink specification | **PARTIAL** | Verified right: `Step` is `pub enum` at `capture.rs:150`, matched exhaustively at `:224-231`; `ask` at `:181` returns `Result<(String, String), _>`; `classify` at `:165` with the `INFO_PREFIX` catch-all at `:172`; `passes.rs:56` prints `capture_file::manifest_row`; and test 14's premise is exact — `configs/arena_wp20_label_pilot.toml` names `configs/instrument_v0.toml` at both `[engine_a]:70` and `[engine_b]:76`. **Three things are still unspecified or wrong.** (a) The route out: `ask` is private and its only caller is `run` (`capture.rs:255`), which is `pub fn run(…) -> Result<Vec<CaptureRecord>, _>` at `:242` and is called from `passes.rs:43`; the rows must travel `ask → run → passes::capture → the file`, and §6 lists neither `passes.rs` nor `run`'s signature. (b) *"`ask` gains a `&mut Vec<String>` sink … so its contract and **every existing caller stay as they are**"* — adding a parameter changes its caller, which is the same sentence-shape revision 4 corrected for `go_reporting` on the page before. (c) The format is not the arena's idiom: `capture_file.rs:59-72` writes a `Fixture` header of `param`/`derived` lines (`param capture_format_version 1`, `experiment_sha256`, `source_sha256`, `label_go`, `capture_sha256`, `games`, `records`) over TAB-separated fixed-arity records, and `:163` reads that header back; a bare *"`census_format 1` kind token"* first line over wire-format rows is a different shape. The `capture_format` string §6.2 seems to be echoing lives at `capture.rs:105`, inside `capture_sha256`'s digest canonicalisation, not in any file. See **U4**, **U5**. |
| **Q6** — `new_game` clears the census | **PARTIAL** | The mechanism is right — `Pistol::new_game` (`instance.rs:74-76`) calls `self.searcher.clear()` — and it now has a diff row, invariant 9 and test 17. But the limb is redundant with the disarm it ships beside: if the engine disarms at the end of every census `go`, `self.census` is already `None` when `new_game` runs, so the `clear` change is unreachable on the engine path. It is the limb that breaks the example (T1), and §6.1 gives it no independent justification beyond "hard rule 4 wants a pin" — which the `stop` call already supplies. See **U8**. |
| **Q7** — the 654/694 denominator | **DISCHARGED** | §10.5 now reads "four proofs in 694 firings, 0.58 %", derived from §2's own six counts, and I reproduce 694 and 0.58 %. |
| **Q8** — the D-551 column audit | **PARTIAL** | "Eleven" is corrected to "nine" and nine is right. The enumeration offered as the derivation is wrong and is a verbatim transcription of the reviewer's own miscount. The stale *"a twelfth column"* is fixed to "a fifteenth". See **U1**. |

---

## NEW FINDINGS

### BLOCKING

#### T1 — the arming rule kills `examples/trigger_census.rs`, the instrument that produced this document's own MEASURED numbers, and the design does not mention it

`crates/pistol-search/examples/trigger_census.rs`, at this revision:

```
crates/pistol-search/examples/trigger_census.rs:166   if args.gate_on {
crates/pistol-search/examples/trigger_census.rs:167       engine.collect_trigger_census();
crates/pistol-search/examples/trigger_census.rs:168   }
...
crates/pistol-search/examples/trigger_census.rs:179   for line in text.lines() {
...
crates/pistol-search/examples/trigger_census.rs:195       engine.clear();
crates/pistol-search/examples/trigger_census.rs:196       let outcome = match engine.search(&state, Stop::Nodes(args.nodes), &mut |_| {}) {
...
crates/pistol-search/examples/trigger_census.rs:215       for row in engine.take_trigger_census() {
```

`collect_trigger_census` is called **once, before the loop**. `clear()` is called
**inside the loop, per entry**, and the comment on it is load-bearing:

> *"EVERY ENTRY IS A DIFFERENT GAME, so every entry starts cold. Without this the
> transposition table carries across positions and a later entry's node count
> depends on an earlier entry's … and the counts stop being comparable with
> `tools/bench_block.sh`, which runs one `newgame` per entry in a fresh process
> and is the seat the bracket's SHARE was derived on."*

So the `clear()` cannot simply be dropped: it is what makes the entries
comparable, which is what makes the numbers this design quotes mean anything.

Apply §6.1's rule — *"`Searcher::clear` sets `self.census = None`"* — and the
first loop iteration disarms the census, the search collects nothing, and
`take_trigger_census` hits its own `.expect("take_trigger_census without
collect_trigger_census")` at `search.rs:220`. **The instrument panics on entry 0
with `--gate on`.**

What that instrument is, in this document's own words:

- §2, the volume basis: *"`crates/pistol-search/examples/trigger_census.rs`
  already exists, takes `--nodes --cap --gate`, and arms its own wiring. The red
  team ran it in two commands (R4)"* → the MEASURED 95–159 firings per ask that
  §2's whole byte arithmetic and §9's tranche-one registration stand on.
- F3, limb 2: *"reproducible through `crates/pistol-search/examples/trigger_census.rs`,
  which arms its own wiring independent of any config"* → the MEASURED 14.5×/36×
  that D-563 carries, that the ONE LINE FOR THE MORNING carries, and that §10.1
  asks the operator to rule 38–95 days of machine time on.
- §3's strongest surviving attack on T1+T3, quoted for the ADR line: *"The census
  already has a working off-wire seam … driven by
  `crates/pistol-search/examples/trigger_census.rs` … which this red team ran at
  the sweep's own budget."*

`/usr/bin/grep -n "example" docs/experiments/wp20b_design.md` returns exactly
those three lines — `:261`, `:331`, `:586`. **§6's diff table has no row for it.
§8 has no test for it. §7 has no invariant over it.**

Two things make this worse than a missed diff row:

1. **No gate catches it.** `tools/ci.sh`'s nineteen gates do not run the example
   (`git grep trigger_census -- tools/` hits only `stage3_allocator_bound.py`,
   `stage3_census_analyse.py` and `stage3_census_rank.py`, all of which *parse its
   stdout*). `cargo test --workspace` compiles examples; it does not run this one.
   The failure is a runtime panic, so gate 3 stays green and the instrument dies
   silently until someone needs it — which, by §9's own registration ("TRANCHE ONE
   EMITS `key_pos` BESIDE THE CANONICAL KEY") and by §10.1's open ruling, is soon.
2. **Three committed `tools/` scripts consume its line format** and would refuse
   on empty input (`stage3_census_analyse.py:91`, `stage3_census_rank.py:135`,
   `stage3_allocator_bound.py:108` all `fail(...)` when no row line is found) — so
   the blast radius is the Stage-3 analysis chain, not one example.

**The provenance of the defect is the finding's point.** `wp20b_design_rev3_REVIEW.md`
P3 wrote: *"The repair is one sentence — the engine arms with
`collect_trigger_census` at …"*. Revision 4 adopted that sentence, added
`stop_trigger_census` and the `clear` limb, wrote *"verified line by line at this
revision"* about `clear` — and did not run the one `git grep` that lists `clear`'s
callers. The doc comment on `collect_trigger_census` that this very document
quotes elsewhere names them out loud: *"the only callers are this crate's own
tests and the `trigger_census` example."*

**The fix is small and mostly already in the design.** `stop_trigger_census`
called at the end of every census `go` discharges invariant 9 and restores §5's
proof by itself; the `clear` limb is what breaks the example and, per U8, is
unreachable on the engine path anyway. Either drop the `clear` limb, or keep it
and give `examples/trigger_census.rs` a §6 diff row that re-arms per entry (a
`collect_trigger_census()` after each `clear()`), stating that the row exists
because the design's own instrument is a caller.

#### T2 — the re-registered bench guard names no instrument, and the only bench in the tree cannot take the measurement

`docs/process.md`'s first section is unambiguous:

> *"THE INSTRUMENT HAS A GOVERNING REVISION TOO. An artefact that produces a
> registered number — a `tools/` script, a scratchpad harness, or a command block
> the document prints — is named in the pre-registration WITH ITS REVISION …
> Without this, a run stands on an instrument whose own review had failed and is
> licensed by argument rather than by this text."*

§9 names none. The only artefact it cites is `tools/bench_delta.sh:95`, and it
cites it for a budget, not as the instrument. That script cannot run either check:

```
tools/bench_delta.sh:92    CONFIG="configs/instrument_v0.toml"
tools/bench_delta.sh:94    FIXTURE="crates/pistol-cli/tests/fixtures/bench_positions_v1.txt"
tools/bench_delta.sh:95    NODES=50000
tools/bench_delta.sh:351           printf 'newgame\nposition %s\ngo %s\n' "$position" "$budget" >>"$WORK/session.$name"
tools/bench_delta.sh:452           if (nps < 1.15) printf "band %s: VERDICT ABORT — nps ratio %.3f is below the pre-registered 1.15 abort threshold…"
```

- Its config is **pinned** to `configs/instrument_v0.toml` — its own header says
  so in capitals — which is the gate-OFF seat. It has no config parameter; its
  usage is `bench_delta.sh SIDE_A SIDE_B [REPS]`, where a SIDE is a binary or a
  revision.
- It varies the **binary**, not the `go` line: `:351` writes the same `$budget`
  for both sides. **Token-ON vs token-OFF is not a comparison this script can
  express.**
- Its bracket and abort clause are the `Eval::delta` bench's (`nps < 1.15` →
  ABORT, `[1.4, 2.5]` bracket). Run on a census change, whose honest result is a
  ratio of ~1.000, it prints `VERDICT ABORT`.

So citing `bench_delta.sh:95` as *"`configs/bench_wp18c_solver_on.toml` at **its
own** registered budget"* attributes a constant to a script that reads a different
config. The seat's own `nodes 50000` is registered in D-465 and `wp18c_design.md`
§6; the values coincide, the citation does not.

**And the registered bound is below the measurement floor of anything this repo
owns.** Scale F3's own MEASURED numbers: 52.75 s for 3 corpus entries at
`nodes 400000` is ~17.6 s per search; at `nodes 50000` that is **~2.2 s per
search** on the ON seat. Against that:

| quantity | value | as a fraction of an ON-seat search |
|---|---|---|
| predicted tax | 0.60 ms | ~0.027 % |
| abort threshold | 2 ms | ~0.09 % |
| `bench_delta.sh`'s own noise gate | IQR > 10 % of median ⇒ `NOISY — verdict withheld` | 10 % |

The abort threshold is **two orders of magnitude below the noise level at which
this project's only bench refuses to render a verdict at all.** A bound no
instrument can resolve is not falsifiable, and `docs/process.md` is explicit that
*"Recording without a criterion is a dry run nothing can fail."*

This is the same defect the previous two rounds found, standing one step further
left each time: revision 2 put the vacuity in the **seat** (both arms structurally
zero), revision 3 moved it to the **threshold** (a magnitude, not a gate),
revision 4 moves it to the **resolution** (an absolute bound below the floor).

Two further `docs/process.md` obligations are unmet by the same paragraph:

- **Dry-run discipline** — *"A pre-registration's literal commands are exercised
  before its review passes, on an input of the SAME KIND … The pre-registration
  records the dry-run input and its output."* §9 prints no commands and records no
  dry run.
- **Cost** — *"A pre-registration states what its governed run COSTS — wall time,
  operator attention, machine hours — so the proportion between the document and
  the run is visible on the document's own face."* Two arms × 24 positions × N
  reps at ~2.2 s a search on an ON seat is knowable to the minute and is not
  stated. This matters here more than usual: §9 registers the guard on the seat
  that F3 measures at 14.5×–36×, and §6.2 already recognises the point in the
  neighbouring paragraph (*"a CI gate is not the place to spend that"*).

**What would discharge it.** Either name a real instrument with its revision — a
harness that drives one binary over two `go` lines on `bench_wp18c_solver_on.toml`,
reports paired per-search wall time, and states its own resolution — and register a
bound that instrument can falsify; or drop the wall-time framing and register the
fold's cost the way it was actually measured (R-COST's 22.99 µs microbenchmark)
plus a **structural** check that kills the named defect class directly, e.g. a
counter asserting the fold is entered exactly `firings` times per search. The
second is cheaper, is genuinely falsifiable, and excludes the hoist and the
per-node call outright rather than by timing.

#### T3 — check (i) and check (ii) are one comparison reported twice, under a heading that says otherwise

§9, as written:

- *"**(i) THE WIRE TAX — solver-ON, token-ON vs token-OFF**, on
  `configs/bench_wp18c_solver_on.toml` at its own registered budget"*
- *"**(ii) THE PER-FIRING FOLD — the same seat and budget.**"*
- then one bracket: *"the ON-token arm's added wall time per search is predicted
  at **0.60 ms** and the run ABORTS if the measured tax exceeds **2 ms**"*

Both costs — formatting/printing the rows, and computing `canonical_key` — are
paid under exactly one condition: the token on, at a firing. Under token-OFF at
the same seat, `self.census` is `None`, the closure at `pvs.rs:623` is not
entered, and no line is printed. **One ON/OFF comparison at one seat measures
their sum, and there is no arm that separates them.** Two labels, one measurement,
one criterion.

The heading is now false on its own terms: *"BENCH GUARD — **TWO CHECKS ON TWO
NAMED SEATS**"* against a body whose (ii) says *"the same seat"*. It is a rev-3
sentence (where (i) was gate-off and (ii) gate-on, genuinely two seats) left
standing after rev 4 correctly moved (i) onto the solver-ON seat. P4's third limb
is discharged; P4's first limb — that the split is real — is not.

Either collapse them honestly into one check with one bracket (which is what the
paragraph already does), or make (ii) a different measurement that isolates the
fold — which is the structural check T2 suggests, and which would make the pair
real.

---

### MAJOR

#### U1 — §4's nine-count enumeration is wrong, and is a verbatim transcription from the report

```
docs/experiments/wp20b_design.md:712-713
- the **nine** counts are decimal `u32`/`u64`/`usize` — one word each: `turns_from_root`,
  the four `mover_*`/`opponent_*` pairs' seven columns, `cover_count` and `attacker_visits`.
```

`crates/pistol-search/src/census.rs:41-58` — `TriggerColumns` — has **three**
mover/opponent pairs (`*_hot`, `*_win_in_one_ply`, `*_live_three`), six columns,
plus `turns_from_root` and `cover`. So "four pairs" is wrong; "seven columns"
double-counts `turns_from_root`, which the same sentence has already listed; and
1 + 7 + 1 + 1 = **ten**, against a claimed nine.

The next sentence gets it right by a second route — *"which is `TriggerColumns`'
seven `u32`s plus `cover_count` plus `attacker_visits`"* — so the document
contains its own correction and does not notice.

`wp20b_design_rev3_REVIEW.md` Q8 reads: *"(`turns_from_root`, four
`mover_*`/`opponent_*` pairs' worth = seven, plus `cover_count` and
`attacker_visits`)"*. **Revision 4 copied it.** In the document whose revision
header says *"Every number in this revision is DERIVED at the tree at `e364497`
and not copied from a report"*, the correction to the miscount is itself a copy of
the miscount's source, and one `sed -n '41,58p' crates/pistol-search/src/census.rs`
would have caught it.

"Four pairs" also stands independently in §4's D-512 paragraph at `:664` —
*"the **detector's** directions — `mover_*` against `opponent_*`, four pairs — each
a field of its own"* — where it is load-bearing for a scope-2 obligation of the
governing dispatch.

The paragraph's force is an **exhaustiveness** claim (*"There is no score field on
a census row and no other multi-word value"*), and an exhaustiveness claim resting
on an enumeration that does not enumerate its own list is exactly what rev-3's Q8
objected to. The fix is to delete the enumeration and keep the rule plus the
`TriggerColumns` route — see THE CUTS.

#### U2 — §9's firing count is taken from a fixture that is not the registered seat's, and a seat-matched MEASURED count exists

D-530's counts are *"MEASURED BY A FULL GOVERNED SEARCH FROM EACH ROW
(`artifacts/stage3c_census_value_fixture_v1.txt`, `nodes 50000`, cap 2048)"* over
the seven rows of the **D-512 value fixture**: `0, 0, 13, 13, 1, 1, 26`. The
design takes **26** — that fixture's maximum, from `g001-t42-p2` — and presents it
as the seat's firing count.

The registered seat's fixture is `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt`
(24 positions). And a firing count measured **on that seat, at that budget** is
already in the ADR log: D-516 records *"against **12.00 / 9.73 / 6.72 firings
today**"* — corpus band 15 / band 35 / trigger-rich — from the premise run D-508
describes as *"ON THE TWO COMMITTED SEATS RE-RUN UNCHANGED … `nodes 50000`"*.

Using the matched number gives 12.00 × 22.99 µs ≈ **0.28 ms**, not 0.60 ms.

The direction is conservative for the abort clause (a higher prediction makes the
2 ms bound looser, not tighter), so this does not by itself make the guard wrong —
it makes the guard's criterion *weaker* than the evidence supports, and it makes
the header table's claim to have derived the number *"at seat
`configs/bench_wp18c_solver_on.toml`'s own budget"* only half true. This is
D-291's own clause in its stated form: an estimate that could have been read off a
MEASURED ADR line in seconds is a finding.

#### U3 — §6.1 points at the wrong §6 row for `SearchOutcome`'s new field, and the right row states only an exclusion

§6.1: *"the field is added **there** and §6's `search.rs` row carries it"* — where
"there" is `SearchOutcome`. But `SearchOutcome` is defined at
`crates/pistol-search/src/info.rs:237-245`, and §6's rows read:

| row | text |
|---|---|
| `crates/pistol-search/src/search.rs` | the root site; `clear` clears `self.census`; a `stop_trigger_census` |
| `crates/pistol-search/src/info.rs` | **NOT the per-depth `SearchInfo`** — see §6.1 |

So the file that gains the public field has a diff row that says only what it does
*not* change, and the sentence claiming otherwise names the wrong file. `search.rs`
does change (it constructs `SearchOutcome` at `:417` and `:520-530`), but that is
a construction-site change, not the field.

This matters beyond tidiness: rev-3's Q1 asked for exactly this decision (*"The
design does not say whether `SearchOutcome` gains a field or the return becomes a
tuple — the one decision M4 asked for"*), and it is a change to a **public
pistol-search struct re-exported by pistol-engine** (`crates/pistol-engine/src/lib.rs:51-53`),
which §6.1 itself calls *"a change to the project's named contract, so it is one
of the ADR lines the closure owes"*. A contract change whose diff row is an
exclusion is how a contract change reaches REVIEW-impl unreviewed.

#### U4 — the sink's route out of `ask` is still unspecified, and one sentence about it is false

§6.2 specifies `ask`'s sink and stops. The rows have three more hops and none is
in §6:

```
crates/pistol-arena/src/capture.rs:181   fn ask(…) -> Result<(String, String), ArenaError>      // private
crates/pistol-arena/src/capture.rs:255       let (totals, bestmove) = ask(…);                   // its ONLY caller
crates/pistol-arena/src/capture.rs:242   pub fn run(transcript, label_nodes) -> Result<Vec<CaptureRecord>, ArenaError>
crates/pistol-arena/src/passes.rs:43         let records = crate::capture::run(&transcript, label_nodes)?;
crates/pistol-arena/src/passes.rs:44-58      render → write → println!(manifest_row(…))
```

`run` is the **public** seam and its signature must change for the rows to leave
`capture.rs`; `passes::capture` is the function that opens the file, writes it and
prints the manifest row. §6 lists `capture.rs` (twice) and `arena.rs`, and lists
neither `passes.rs` nor `run`. §6 claims to be the diff.

And: *"`ask` gains a `&mut Vec<String>` sink rather than a changed return type, so
its `Result<(String, String), _>` contract and **every existing caller stay as they
are**."* Adding a parameter changes its caller — `capture.rs:255` must pass the
sink. The return contract is preserved; the caller is not. This is the same
sentence-shape revision 4 corrected two pages earlier for `go_reporting`
(*"Revision 3 said 'every existing caller is unchanged', which was false"*),
reproduced for a different function inside the same fix round.

#### U5 — §6.2's census file is not written "in the arena's own idiom"

§6.2: *"first line a `census_format 1` kind token in the arena's own idiom
(D-139/D-147/D-160), then one row per line in §4's field order — **the same bytes
the wire carried**"*.

The arena's own idiom, at `crates/pistol-arena/src/capture_file.rs:59-72`, is a
`Fixture` header — a comment block, then `param capture_format_version 1`,
`param experiment_sha256 …`, `param source_sha256 …`, `param label_go …`, then
`derived capture_sha256 …`, `derived games …`, `derived records …` — over
TAB-separated records of a declared arity (`const FIELDS: usize = 5`), read back
through `header(text, "param", "capture_format_version")` at `:163`. A bare
`census_format 1` first line is not that shape. The string §6.2 appears to be
echoing, `capture_format {version}`, lives at `capture.rs:105` **inside
`capture_sha256`'s digest canonicalisation** and is not written to any file.

"The same bytes the wire carried" is also in tension with that idiom: the wire
rows are space-separated key/value pairs, while every arena artifact file this
project ships is TAB-separated with a validating header. The design should either
adopt the `Fixture` shape (and say the rows are the wire's payload inside it) or
state that it is deliberately introducing a second artifact idiom and why.

Related and unresolved: *"the census file gets [a `manifest_row`] too"* names no
module. `capture_file::manifest_row` (`:106`) takes `(transcript, label_go,
rendered, out_path)` and its doc records a rule-8 property (*"PRINTED and never
written"*); a census equivalent is a new public function in a file §6 does not
list.

#### U6 — the cap conclusion is an unmarked extrapolation, on a mislabelled quantity, and omits D-530's own statement about cap 16384

Three separate problems in the paragraph that discharges Q3:

1. **Outside the swept range.** D-465's sweep is `{32, 128, 512, 2048}`. The claim
   is about 16384. *"D-465's own sweep says the true figure is probably within a
   small factor of it rather than an order above"* attributes to a sweep a
   statement about a point it does not contain. Fit the sweep's own power law and
   an 8-fold rise lands at ~1.5× — which is a defensible ESTIMATE and is not
   marked as one, in a document that corrects revision 2 for exactly this
   (*"The old estimate was right; the marking was the defect (D-291), and this is
   what that clause is for"*).
2. **Mislabelled quantity.** The design calls it *"the corpus **search-node**
   ratio"*. D-465 says *"the corpus ratio"*, and D-508 identifies that quantity as
   *"the **nps**-RATIO"* (*"THE ~6x IS THE nps-RATIO SHORTFALL 0.0809 -> 0.5 READ
   AS THOUGH IT WERE A CALL-COUNT FACTOR"*). D-516 uses a third, distinct
   quantity, *"the solver's SHARE of a search's nodes"*, and explicitly warns that
   D-465's cap finding is about the ratio and does not transfer: *"D-465 measured
   the cap dead as a lever on the RATIO; the visit budget is fixed by the bracket
   and `count = visits/cap` … different claims about different quantities."*
   Substituting units across a citation is D-477's named class, and D-508 is a
   whole ADR about this project paying for it.
3. **D-530's cap-16384 sentence is dropped.** The same ADR §9 cites for its 26
   firings says: *"THE ONE ROW THAT CONSTRAINS A CAP IS `g001-t42-p2` AT cap >=
   16384 — at 4096 both directions return `unknown` after 8,192 visits."* That is
   direct evidence about the cap the operator would produce by a one-key arming,
   in the ADR the design is already reading, and it is not carried into F3 or
   §10.1.

Note that a **stronger and untaken** argument for the same conclusion sits inside
F3 itself: the design's own MEASURED *"the search gets 0.8 %–10 % of it and the
solver absorbs the rest"* at cap 2048 means the node mix is already near
saturation, so raising the cap can shift at most the residual 1–10 %. That
bounds the 2048→16384 penalty from the design's own measurement rather than by
extrapolating someone else's sweep. If the conclusion is worth keeping, keep it on
that footing and mark it.

#### U7 — the derived seat rule does not cover four of the seventeen rows it is applied to

THE RULE, as stated: *"a test whose mutant can only manifest at a firing is seated
solver-ON; a test whose subject is refused before a search ever starts has no
seat; and exactly one test is seated solver-OFF, because the absence of firings is
what it pins."*

Applied: *"no seat for 2, 3, 4, 6, 11, 15."* Tests 6 and 15 are grammar refusals
and fit clause 2. Tests **2, 3, 4 and 11** are `pistol-core` unit tests of the
fold — they never start a search and are never refused, so clause 2 does not reach
them; their mutants (transposition-invariance lost, symmetry-invariance lost,
over-folding) manifest in a direct call, so clause 1 does not force them ON
either. **Four of seventeen rows fall through the rule and are seated by the list
the rule was written to replace.**

The repair is one clause: *a test whose subject is a pure function has no seat*.
With it the rule partitions all seventeen; without it, P2's *"derived from a
property and not from a list"* is true of thirteen rows and false of four.

Related, and why this is MAJOR rather than MINOR: **test 9's "mutant it kills"
cell names no mutant** (*"F3's own pin — the fact that makes the sweep's claim
checkable rather than assumed"*), so the rule's clause 1 has no input for the one
row seated solver-OFF. Rev-3's S13 said this; it is unchanged.

#### U8 — the arming rule's two calls have no stated ordering, and one of its two limbs is unreachable

`take_trigger_census` panics when `self.census` is `None` (`search.rs:216-221`).
§6.1 says the engine *"arms with `collect_trigger_census` at the start of every
census `go` and DISARMS at its end"* and adds `stop_trigger_census` — and never
says whether the rows are taken **before** the disarm. If `stop` sets
`self.census = None` and the engine then calls `take`, the census `go` panics. The
order is `collect → search → take → stop`, and a design whose whole §6.1 exists
because a seam's contract was not read owes that line explicitly. Also
unspecified: what `stop_trigger_census` does with rows not yet taken (discard,
return, refuse).

And the `clear` limb changes no reading on the engine path. If the disarm runs at
the end of every census `go`, `self.census` is already `None` when `Pistol::new_game`
(`instance.rs:74-76`) calls `Searcher::clear`. The only case the `clear` limb
catches is a census `go` that returns `Err` before its disarm — which is a real
gap, but the fix for it is to run the disarm on the error path, not to change
`clear`. As written the limb is D-424's own case (a change that changes no
conclusion) **and** it is the limb that breaks the example (T1). Q6's rule-4
argument is satisfied by the disarm plus a test over two consecutive `go`s.

---

### MINOR

- **V1 — S1 is not discharged, verified at the tree.** §2 still cites
  `crates/pistol-search/src/position.rs:102-104` for *"a `Set` position's declared
  `to_move` is checked against that count rather than trusted"*. Those lines are
  inside `static_score_after`/`place`. The text is at
  `crates/pistol-engine/src/position.rs:102-104`: *"The stated `to_move` and
  `phase` are **checked, not trusted**"*. Wrong crate, second round.
- **V2 — S11 is not discharged, both halves.** `capture.rs:227` is cited for
  `Step::Ignore => continue`; the line is **`:229`** (`:224` is `match classify`,
  `:229` the arm). `exchange.rs:199-205` is cited for *"The word after `key`,
  matched whole"*; that doc line is **`:198`**, and `:199-205` is the function
  body. Both were flagged as transcription slips and both were transcribed again.
- **V3 — S6 is not discharged, verbatim.** Invariant 1 still asserts
  *"**`canonical_key` is a pure function of the stones and cannot fail** — but see
  invariant 7"*, and invariant 7 says it panics on coordinate overflow. An
  invariant that states a falsehood and appends a pointer to its own refutation is
  worse than the wording R17 asked for. Say what is true: it cannot move a move
  and cannot end a search within the radius-8 region, and invariant 7 owns the
  bound.
- **V4 — S13 is not discharged.** Test 9's mutant cell names a rationale, not a
  mutant. See U7.
- **V5 — S14 is not discharged.** §4 states the block's position three times
  (*"after the last depth's `info`, before `info totals`, before `bestmove`"*), §6
  puts it in the `protocol.rs` row, and no test in §8 pins it. Test 13 pins the
  **count** of lines, not their **place**. A reordering mutant survives the whole
  table.
- **V6 — S2 is not discharged, and it is now checkable.** *"A `go` line carrying
  the token when the engine was built without census support is a named refusal"*
  (§3.1) and invariant 6 say the same thing twice (D-423), and neither has a
  mechanism: `git grep -n "\[features\]" -- crates/` returns **one** hit, inside a
  test fixture string in `solver_link_check_tests.rs:313`. No crate in this
  workspace has a cargo feature, and §6 creates none. The refusal is over a build
  that cannot be produced — D-424's own case.
- **V7 — S7 is not discharged.** *"~119 800 distinct positions"* is still
  unmarked though D-560 marks its own figures ESTIMATED, and option A's
  *"**MEASURED**-free"* is still a MEASURED marking with no measurement behind it
  (it is an inspection of `pvs.rs:249`, which is a fine ground, differently
  labelled).
- **V8 — S10 is not discharged.** §3.1 still invokes *"D-88's 'additive line
  kind' flip clause"* to license widening the `go` **input** grammar. `info census`
  is an additive line kind; a third word on `go` is not.
- **V9 — S12 is not discharged.** §10.0(a) still calls a dispatch that does not
  mention `wp20s_design.md` §8's rule *"silent drift by definition"*, where hard
  rule 10's drift clause is about ADR lines and that rule has none. (The
  constructive consequence rev-3 offered still stands and is still not taken: if
  the `key_full` disjointness rule is load-bearing for D-537's count, this
  package's closure is where its ADR line belongs.)
- **V10 — S3 is not discharged.** `docs/experiments/wp21_DISPATCH.md` §4 still
  lists three decisions owed and not F3's, in the same uncommitted diff as the
  D-563 that creates the fourth.
- **V11 — the byte-identity referent and the digest that was taken disagree by one
  line.** §9 says the referent is *"everything **above** the `# timing` marker"*.
  The cited digests reproduce only when the marker line is **included**
  (`sed -n '1,/^# timing/p' | sha256sum` → `7f8a6f97…` / `06490795…`; excluding it
  gives `f4bdfcd9…` / a different value). The digest is right and the sentence
  describing it is off by one line — on the one obligation whose whole content is
  which bytes are compared. State the extraction rule, since the closure will have
  to re-take it against the post-change binary.
- **V12 — `Step` is cited as `capture.rs:150-158`; the enum is `:150-157`.**
  Harmless, listed because it is the fourth off-by-a-line in the same document.

---

## THE CUTS

**Is the size itself a defect? Yes — but not because 1130 lines is a lot.** It is a
defect because of *what* the mass is. Roughly 250–300 of those lines are
review-response prose: which revision said what, which reviewer found it, what was
withdrawn. And that layer is where this revision's errors live. Of the defects
above, **U1 is a verbatim transcription from a review report; V1, V2 and V12 are
citation slips carried forward from a review report; T3 is a heading from revision
3 left standing after revision 4 changed the body under it.** Five of the sixteen
findings are artefacts of the history layer. That is the concrete argument for the
cut and it is stronger than a line count: *the document's defect rate is highest
in the part of the document that constrains nothing.*

D-424's test — *"whether the disputed claim changes what anyone may conclude"* —
applied concretely:

1. **The REVISION HEADER, lines 16–70 (~55 lines).** The revision/review table,
   the two-paragraph quotation of the rev-3 reviewer's diagnosis, the six-number
   derivation table, "WHAT REVISION 4 CHANGES", "WHAT REVISION 4 DOES NOT CHANGE".
   No implementer decision and no successor reading turns on any of it; the three
   review reports are the record of the arc and are in the repository.
   **KEEP** the four lines naming the governing revision, the governing dispatch,
   and the two outstanding reviews. **CUT ~50 lines.** If the operator wants the
   six-number table for this round, it belongs in the review-response cover note,
   not in §0 of a design a successor reads in a year.
2. **Every "revision 2 said / revision 3 said" gloss in the body (~90–120 lines).**
   Named concretely: §2's heading *"SWEEP-VOLUME ARITHMETIC, RE-DERIVED, BECAUSE
   REVISION 2's USED TWO DIFFERENT BASES AND WAS WRONG BY ~3x IN ITS OWN
   RECOMMENDATION'S FAVOUR"* and its explanatory paragraph (keep the arithmetic
   block, cut the archaeology); §2's *"WHY C2 OVER C1 — REVISION 2 GAVE TWO
   REASONS AND BOTH WERE DEFECTIVE"* — the first two bullets are **withdrawn**
   arguments and should simply be gone, leaving the third (the real per-firing
   allocation penalty) and the fourth; the `Copy`/seven-sites paragraph, whose
   whole content is that a withdrawn cost row had no users; §4's *"Revision 2
   justified this by a mis-split hazard that does not exist and revision 3
   withdraws it (N4)"* (keep the `EMPTY_FIELD` precedent, which is the ground);
   §8's two paragraphs of seat history before THE RULE; §9's *"Revision 2
   registered one ON-token/OFF spot-check … Revision 3 split the seats and then
   registered (ii) as a magnitude"*; §6.1's *"Revision 3 stated it and stated it
   wrongly at three points"*.
   **The rule, the number and the site survive; the provenance goes.**
3. **F3's repetition — the cut rev-3 asked for and rev 4 did not make.** The
   14.5×–36× / 900–2 300 h / 38–95 days figure now appears at `:5-7`, `:257-259`,
   `:305`, `:600`, `:878`, `:1092` — six places, plus D-563. **And revision 4
   added a seventh repetition of the cap caveat**, stating it at length in F3 and
   again in near-identical prose in §10.1. D-423 is explicit. F3 owns the finding;
   the ONE LINE FOR THE MORNING is a standing format and may carry the headline;
   §3, §6.2 and §10.1 point at F3. **CUT ~30 lines**, and note that this cut is
   also the fix for the class of defect that produced U6 — the cap qualifier was
   right in one copy and thin in the other.
4. **§10.0 in full (~22 lines).** *"Whether v2 ruled on F1 or F2."* Its two limbs
   conclude that the dispatch did not waive F2 and that F1's instruction is
   vacuously satisfiable — both of which F1 and F2 already say in their own
   sections (*"it stays green unchanged, which is the evidence…"*, *"the
   dispatch's option field is empty under the constraint it inherits"*). Nothing
   in §6, §8 or §9 changes on either limb. **DELETE.**
5. **§3.1's "built without census support" sentence and invariant 6.** No cargo
   feature exists and §6 creates none. Said twice (D-423) about a build that
   cannot be produced (D-424). **DELETE both** — or keep one and name the
   mechanism.
6. **Invariant 3 (~2 lines).** The document itself says it *"was already true
   today and could not have caught this"*. An invariant the pre-change tree
   satisfies and this package cannot break constrains nothing. **DELETE**; keep 3a,
   which is the one that can fail.
7. **§4's fourteen-item D-551 audit (~12 lines).** The design already concedes
   *"What the design owes is the RULE and not just the audit"*. Keep the rule and
   the pointer to test 8's field-order pin; delete the item-by-item walk. **This
   cut also removes U1's wrong enumeration**, which is the cleanest illustration
   available that the cut and the correction are the same act.
8. **§8's D-553 narrative (~4 lines).** *"D-553's motivating instance is a receipt
   that read `SURVIVED` after two fresh-context reviews…"* — that story is D-553's
   and is stated there. The rule (sort guards from values) stays.
9. **§2's "AND IT IS CORRECT ON SIDE-TO-MOVE" paragraph** — **KEEP**, against the
   general rule, and I say so explicitly because it looks like a candidate. Its
   last sentence gives the reason: *"a successor will raise the objection again."*
   That is the test D-424 sets, and it passes.

Total: **~200–230 lines**, taking the document to roughly 900, with no claim, no
number, no site and no decision lost — and with three of this review's findings
deleted rather than fixed.

---

## THE STRONGEST SURVIVING ATTACK ON REVISION 4

> **Revision 4 checked the numbers the last reviewer put in a list, and did not
> check the property the last reviewer was pointing at — which is, word for word,
> the diagnosis its own header quotes and promises to answer.**
>
> The header says the fix is method: *"Every number in this revision is DERIVED at
> the tree … and not copied from a report."* Six numbers are audited on the
> document's face, and five of the six survive an independent derivation. But the
> method was applied to the *outputs* the reviewer enumerated, not to the *reasoning*
> that produced them, and every defect in this review sits in the gap between the
> two:
>
> - The one repair the rev-3 reviewer wrote out as a sentence — *"the engine arms
>   with `collect_trigger_census` at the start of every census `go`"* — was adopted
>   verbatim, extended with a `Searcher::clear` limb, annotated *"verified line by
>   line at this revision"*, and never tested against `clear`'s callers. It kills
>   `examples/trigger_census.rs` at runtime: the instrument that produced §2's
>   volume basis, F3's 14.5×–36×, D-563's ADR line, and the input three committed
>   `tools/` scripts parse. The design cites that file three times as evidence and
>   zero times as a file the diff touches, and the doc comment it quotes elsewhere
>   names the callers out loud.
> - The correction to Q8's miscount was copied out of Q8, miscount and all: "four
>   pairs" where `census.rs` has three, "seven columns" that double-count a column
>   the same sentence lists, ten items summing to a claimed nine.
> - The bench guard was re-derived to two decimal places and registered against no
>   instrument. The only bench in the tree pins a different config, varies the
>   binary rather than the `go` line, and withholds a verdict at 10 % noise — while
>   the registered abort threshold is 0.09 % of the search it would be measured in.
>   The vacuity has now been in the seat, then the threshold, then the resolution;
>   three rounds, three locations, one defect.
> - The two checks the guard registers turn out to be the same comparison, under a
>   heading from the previous revision that says they are two.
>
> **The pattern is not carelessness about arithmetic — the arithmetic is now
> good.** It is that this document treats a reviewer's report as the specification
> of what to verify. A review report enumerates what one reader happened to catch;
> a tree answers what is true. Revision 4 derived its numbers at the tree and its
> *properties* at the report, and the property that mattered most this round —
> "does the rule I am adopting work against the code that exists?" — was never
> asked of anything. The cheapest possible check, `git grep -n "\.clear()" --
> crates/pistol-search`, was one command away and would have returned the example
> on line two.
>
> **What follows for the fourth fix round is a method and not a list.** Every rule
> this design states — the arming rule, the seat rule, the grammar rule, the
> word-count rule, the sink's contract — is a universally quantified claim about a
> tree. Each one gets the `git grep` that enumerates its subjects, and the
> enumeration goes in the document beside the rule, the way F3's three armed
> configs and Q1's three call sites already do. Those two are the parts of this
> revision that are right, and they are right for exactly that reason.

---

## What is NOT in dispute

Recorded so the fix round does not re-open what survives:

- **C2 stands** as the identity form, on F2's constraint, with the `canonical_form`
  fold in pistol-core (rule 2), the side-to-move argument, and the strongest
  surviving attack quoted for the ADR line. The exports it needs are where §2 says
  they are (`crates/pistol-core/src/lib.rs:86` and `:90`).
- **T1+T3 stands** as the token's home. Both T2 variants are foreclosed by landed
  tests I verified: `crates/pistol-engine/tests/config_schema_tests.rs:21` and
  `crates/pistol-arena/tests/config_tests.rs:186` both forbid `serde(default` in
  the config module.
- **F1 stands**, including the workspace-shape test staying green unchanged, and
  the re-export route at `crates/pistol-engine/src/lib.rs:47-53` reading exactly as
  quoted.
- **F2 stands.**
- **F3 stands**, retraction included; the three armed configs, the two push sites,
  the wiring guard at `instance.rs:150-152` and the gate at `instrument_v0.toml:113`
  all verified.
- **P1's deletion is right** and its mechanism is verified.
- **§9's pre-change digests are right** and reproduce.
- **The R13 fourth-word refusal is a real requirement**: `budget_token.rs:45-51`'s
  `[_, _, extra, ..]` arm names the **third** word today, so once the census token
  occupies that slot, an unfixed refusal would name the token instead of the
  offending fourth word. Test 15 earns its place.
- **The D-512/D-535 citation correction is right**: `/usr/bin/grep -o "direction"`
  over `docs/decisions.md:1092` returns zero matches.
