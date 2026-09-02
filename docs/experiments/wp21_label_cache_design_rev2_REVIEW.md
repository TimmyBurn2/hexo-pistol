# REVIEW-design ROUND 2 — `wp21_label_cache_design.md` revision 2. **VERDICT: FAIL.**

**REVISION READ**: `docs/experiments/wp21_label_cache_design.md` revision 2, at the
dispatched commit **`fde1497`**.

**DOES IT STILL MATCH HEAD? YES.** `git rev-parse HEAD` →
`fde14977a5f779d23852e6a96f10e0fb3d25b88c`; branch `dev`. Every `file:line` below
is read at that revision.

**READ**: `CLAUDE.md`; `docs/process.md` (including the standing re-derivation
clause at `:81-115`); `tools/SHELL_CHECKLIST.md`; `docs/decisions.md` D-540,
D-553, D-562, D-568, D-570..D-583 (D-576 `:1220`, D-581 `:1230`, D-583 `:1234`
read in full); `docs/experiments/wp21_throughput_prereg.md` revision 3 §2, §2.0,
§2.1, §4.2, §4.3, §4.4, §7.1; `docs/experiments/wp21_prereg.md` revision 4 §1,
§4, §5; `docs/experiments/matrix_label_cache_key.md` revision 3 (§2.1, §4);
`docs/experiments/arc3_ledger.md`; `docs/experiments/wp21_label_cache_design_REVIEW.md`
(round 1) in full; `crates/pistol-arena/src/{capture.rs,capture_file.rs,passes.rs,channel.rs,seats.rs,outpath.rs,labels.rs,usage.rs}`;
`crates/pistol-arena/src/bin/{arena.rs,stub_engine.rs}`;
`crates/pistol-core/src/{symmetry.rs,coord.rs,board.rs}`;
`crates/pistol-search/src/heuristics.rs`;
`tools/{ci.sh,cold_label_check.py,label_cache_count.py,governing_citation_check.sh,design_citation_check.py,file_justification_check.sh}`;
`docs/rule9_justifications.md`.

**NO `cargo` COMMAND WAS RUN.** The tree was not modified except this file.

---

## RE-DERIVATION — commands I chose, each with its scope

`docs/process.md:83-87`: *a count reproduced only by running the document's own
command is NOT reproduced.* None of the below is the document's command, and the
`42` is re-derived from the book with my own implementation of `canonical_form`
rather than by running `tools/opening_prefix_fold.py`.

| # | command, with its scope | returns | the document says |
|---|---|---|---|
| **R1** | `/usr/bin/grep -n 'GATE_TOTAL=\|step "gate' tools/ci.sh \| LC_ALL=C sort -t: -k1,1n` — scope = every gate declaration in the script, not one line of it | `:21 readonly GATE_TOTAL=20`; `:97 gate 6/…: config validation`; `:109 gate 9/…: cross-process determinism`; `:110 gate "determinism" tools/determinism.sh`; last is `:202 gate 20` | §ONE LINE `:15` *"gate 9 of 20"* ✓; **§2 `:162` *"CI gate 9 of 19 (`tools/ci.sh:104-105`)"* — REFUTED on both halves.** `:104-105` is comment prose (`# It runs last of the two engine gates…`) — see **BLOCKING A** |
| **R2** | my own Python re-implementation of `symmetry.rs`'s `rotate`/`Symmetry::ALL`/`transform`/`canonical_form` (field orders taken from `coord.rs:18-24` and `board.rs:13-19`), run over the non-comment lines of `crates/pistol-cli/tests/fixtures/random_openings_v2.txt`, slice `[13:231]` | `k=1` 1/1; **`k=2` exact 213, symmetry 171, extra merges 42**; `k=3` 218/218 | §1.1 `:80` *"213 `k = 2` classes to 171 … at least 42"* — **REPRODUCED exactly** |
| **R3** | same instrument, run over **all sixteen** tranches of the registered partition (`wp21_prereg.md:165-168`: skip 13, 15×218 + 1×217; my loop ends at skip 3500, the holdout's first opening, as `:168` requires) | floors per tranche: **42, 51, 48, 55, 47, 48, 50, 44, 53, 48, 51, 60, 45, 56, 48, 46**; sum **792** | *"at least 42 `key_full` collisions **per tranche**"* (`:80`) and *"MEASURED at 42 searches **a tranche**"* (`:326`) — **true only of tranche one**; see **MAJOR E** |
| **R4** | my own parser over `/home/tom/pistol-runs/wp20pilot-artifacts/capture_v1.txt`, taking the `position` column index from the file's OWN header sentence (`# TAB-separated, five fields: …`) rather than hardcoding field 3 | 742 body records, 347 distinct `position` lines, 395 hits, **hit rate 0.5323 / miss rate 0.4677**, 0 rows off arity | §ONE LINE *"742 asks over 347 distinct questions"*, §1.1 *"the 47% of asks that miss"*, §2.1 *"the 53% of records that are hits"* — **all REPRODUCED** |
| **R5** | `git log -S'wp21_label_cache_design.md' -- tools/governing_citation_check.sh`, and `git show <rev>:tools/governing_citation_check.sh \| grep -c` at `239f21f`, `f1acc57`, `fde1497` | added at **`fde1497` itself**; counts `0`, `0`, `2` | §header `:22` *"The design is **not** on `tools/governing_citation_check.sh`'s list"* and §8 `:312` *"discharged by the IMPL commit"* — **both false at their own revision**; see **MAJOR J** |
| **R6** | `timeout 180 python3 tools/design_citation_check.py docs/experiments/wp21_label_cache_design.md` — scope = the instrument gate 20 actually runs, over this document | `19 citation(s) checked, 1 unreproduced` (the unreproduced one is the `--proposes`'d `label_cache_tests.rs`); exit 0 | §header `:23-24` *"these three would have been caught if it were [on the list]"* — **REFUTED**: it IS on the list, and `tools/ci.sh:104-105` passes green |
| **R7** | `/usr/bin/grep -n "eprintln!\|writeln!(out" crates/pistol-arena/src/bin/stub_engine.rs` plus `git grep -n -i "ask_count\|n_asks\|go_count" -- crates/pistol-arena` | the stub writes one startup line to stderr and protocol answers to stdout; **no ask counter anywhere** | §6 T2 `:278` *"measured from the stub's own count"* — **REFUTED**; see **BLOCKING B** |
| **R8** | `git grep -n -i "hit rate\|hit_rate" -- crates` | nothing | §1.1 `:66` *"printed once per capture, beside the hit rate"* — the hit-rate output still exists nowhere and no §1 row registers it |
| **R9** | `git grep -n "= ask(\|fn ask" -- crates` | `capture.rs:231 fn ask(` (private), one call site `capture.rs:343` | X3's hoist has exactly one call site to preserve — **the hoist is safe**, see *WHAT SURVIVED* |
| **R10** | `git diff 239f21f fde1497 -- docs/experiments/wp21_label_cache_design.md` | §4 and §5 have **no hunks at all** | §header `:4-5` *"Every finding is disposed of at the section that owns it"* — §5 owns MAJOR 11 and §4 owns MAJOR 16; **neither changed a byte** |
| **R11** | `wc -l < crates/pistol-arena/src/capture.rs`; `/usr/bin/grep -n SOFT_CAP= tools/file_justification_check.sh`; `/usr/bin/grep -c "rule 9" <design>` | `389`; `SOFT_CAP=300`; `0` | MAJOR 12 unaddressed |
| **R12** | `/usr/bin/grep -n "no extra search\|materially\|settles D-562" docs/experiments/matrix_label_cache_key.md` | `:251` *"Two counters, no extra search, one sort the arena already does"*; `:253` *"materially above zero **across the sweep**"*; `:256` *"It also settles D-562(2)'s open three-key question"* | the design contradicts all three and records none of it — **MAJOR G** |

---

## DISPOSITION OF ROUND 1 — all 22 findings

| # | round-1 finding | status | evidence |
|---|---|---|---|
| **B1** | memo as a `&mut` parameter; invariant uncheckable | **CLOSED** | §2.0 `:103-122`. `run` takes `LabelCache{Off,On}`; the map is built in `run`. Verified this is realisable: `seats::with_seats` takes `impl FnOnce(&mut [Channel; N])` (`seats.rs:25`), called once at `:55`, so a map declared at `:333` or inside the closure body both compile and both drop with `run`. The invariant is genuinely structural — see *WHAT SURVIVED* 1 |
| **B2(a)** | counters have no route out and no print site | **PARTIAL** | §1 row 3 adds `CaptureCounts` returned beside the records; row 4 adds the `passes.rs:82-96` print site. But §1.1 `:66` still says *"beside the hit rate"* and R8 shows no hit-rate output exists and no row registers one |
| **B2(b)** | `Mode::Capture` tuple arity / new arms | **PARTIAL** | row 5 `:46` says *"the `--label-cache` word, and X1's refusal arms"* — no arity, no arm count; §5 still names no site. See **MAJOR I** |
| **B2(c)** | no test site anywhere | **CLOSED** | §1 row 7 `:48` names `capture_tests.rs` and a new `label_cache_tests.rs`; §6 enumerates six rows |
| **B2(d)** | row 6 already landed; the owed change missing | **PARTIAL** | row 8 `:49` is correct — and I confirmed the ten-record floor is registered at `wp21_prereg.md:345` and absent from `tools/cold_label_check.py` (R: `/usr/bin/grep -n "MIN_SAMPL\|ten " tools/cold_label_check.py` → nothing). **But §4 `:231` still reads *"`tools/cold_label_check.py` **gains** `--partition`, and it is **required** rather than defaulted"***, registering as future work what landed at `f1acc57` — the table and the prose now contradict each other |
| **B3** (normalise side) | side unstated; one of two choices writes non-deterministic bytes | **PARTIAL** | §2.1 `:124-143` states the side. But it picks the side opposite to the registration's own §4.2 wording without saying so, and §5 gains **no** normalise-side mutant though the fix asked for two. See **MAJOR H**, **BLOCKING D** |
| **B3** (`no_tab` half) | — | **REJECTED, and the rejection is RIGHT** | `capture.rs:359` `no_tab(&record)?` sits in `run`, after the record literal closes at `:358`, outside `ask` (`:231-319`). See *ATTACKS REJECTED* 2 |
| **B4** | two guards inside `ask` stop running on 53% of prefixes | **CLOSED** for `unsolicited`; the census half is argued, not guarded | X3 (`:194`) takes the reviewer's recommended fix (a). The census half's mootness argument leans on X1 being a CLI refusal — see **MAJOR K** |
| **B5** | wrong gate named, twice | **OPEN** | §ONE LINE fixed at `:15`; **§2 `:162` is byte-identical to revision 1**. R1 refutes both halves. The header at `:19-24` states the correct facts while the body keeps the wrong ones. See **BLOCKING A** |
| **B6** | governed by a superseded registration revision | **CLOSED** | header `:26-33` names revision 3, §2.0 and §2.1; I verified the registration's title line is *"PRE-REGISTRATION, revision 3"* and that §2.0 (`:242`) and §2.1 (`:266`) exist |
| **M6** | `key_pos` is a zobrist, not a stone list | **CLOSED** | §2 `:152-158`. Verified `labels.rs:202 key_pos: state.key().to_string()` and `:203 key_full: render_key_full(&canonical_form(&stones))` |
| **M7(a)** | D-581's second conjunct dropped | **CLOSED** | §1.1 `:85-88` restores the `AND` |
| **M7(b)** | the `key_full` answer is already measured | **CLOSED** | §1.1 `:77-83`, and R2 reproduces the 42 |
| **M7(iii)** | *"materially"* undefined against the floor | **OPEN** | `/usr/bin/grep -c materially <design>` → 2, both undefined. D-424: a phrase that constrains nothing |
| **M8** | *"settles D-562(2)"* is false | **CLOSED** | §1.1 `:90-94` withdraws it; §9 item 4 repeats the withdrawal. The matrix's surviving copy is **MAJOR G**, not this row |
| **M9** | play-order claim wrong under every reading | **CLOSED** | §2 `:177-180`. Verified `heuristics.rs:89 for played in state.played()` inside `record_cutoff`, and `heuristics.rs:155 && let Some(at) = last_stone(state)` under `if gates.countermove` at `:153` |
| **M10(a)** | nothing can measure the counter cost | **OPEN** | `:74` still *"MEASURED on the dry run"*; the dry run is `--label-nodes 2000` (`wp21_throughput_prereg.md:759-760`) and the counters are still unconditional, so no referent exists to subtract |
| **M10(b)** | the cheaper incremental formulation not costed | **OPEN** | unchanged; only *"eighty"* → *"79"* (`:72`, which is correct: `turn_cap = 40` ⇒ `1 + 39×2 = 79`) |
| **M10(c)** | the matrix says *"no extra search"* and is not amended | **OPEN** | R12; and D-581 `:1230` says it too. See **MAJOR G** |
| **M10** residual | the per-miss replay's error path must be `unreachable!`, not `?` | **OPEN** | `/usr/bin/grep -n "unreachable" <design>` → one hit, `:91`, which is the D-562(2) quotation. `make_turn` appears 0 times |
| **M11** | mutation set: no X2 mutant, no call-removed mutant, X1's mutant is the weak one, five arms | **PARTIAL** | §5 unchanged (R10). The X1 kill criterion did land — but in §3 `:201-206`, not in §5, whose bullet still reads *"X1: the refusal's condition negated"*. See **BLOCKING D**, **MAJOR I** |
| **M12** | hard rule 9: `capture.rs` is 389 lines against a 300 cap and its justification names one subject | **OPEN** | R11; zero mentions of rule 9. The package adds a type, a memo, a per-miss replay, a twelve-image fold and two counters to that file |
| **M13** | a cached capture leaves no trace in any artifact, unlike `--census` | **PARTIAL** | the recommended mechanism (b) landed by accident — §1 row 4 prints the counts — but the design never states the consequence, and `capture_sha256` untouched is still asserted at `:97` as a virtue with no residual beside it |
| **M14** | X1 enforced in the wrong place; the illegal state is representable at `capture::run` | **OPEN** | no in-library refusal anywhere in the document. See **MAJOR K** |
| **M15** | §6 neither states nor disclaims three registered consequences | **PARTIAL** | two are now handled (X3, `no_tab`). The third is not: `/usr/bin/grep -c search_nodes <design>` → **0**, against `wp21_throughput_prereg.md:551-556` |
| **M16** | §4 is sound argument in the wrong document | **OPEN** | §4 unchanged (R10), and now internally contradicted by row 8 (see B2(d)) |
| **M17** | the design is on no citation gate | **CLOSED in the tree, FALSE in the document** | R5: added at `fde1497`. §header `:22` and §8 `:312` both say otherwise. See **MAJOR J** |
| **m18** | *"the one shape gate 9 never takes"* overstates | **OPEN** | `:172` unchanged |
| **m19** | *"against a search of ~885 ms"* on a hit | **OPEN** | `:71-74`, `:184` unchanged |
| **m20** | *"neither of its budgets"* — there are four | **OPEN** | `:173` unchanged |
| **m21** | the counters' memory not costed | **OPEN** | unchanged |

**Tally: 7 CLOSED, 6 PARTIAL, 9 OPEN.** The header's *"Every finding is disposed
of at the section that owns it"* (`:4-5`) is refuted by R10 alone: §4 and §5 —
which own MAJOR 16, MAJOR 11 and BLOCKING 3's mutation half — are byte-identical
to revision 1.

---

# BLOCKING

## BLOCKING A — §2 STILL CARRIES REVISION 1's WRONG GATE CITATION, VERBATIM, IN A REVISION WHOSE HEADER PRINTS THE CORRECT ONE FOUR LINES ABOVE IT

The design, `:19-24`:

> **REVISION 1's ONE LINE SAID "CI gate 6", WHICH IS CONFIG VALIDATION**
> (`tools/ci.sh:97-98`) … **Gate 9 is `:109-110`; `:104-105` is the comment above
> it, which revision 1 also cited wrongly.**

The design, `:162`, unchanged from revision 1:

> `tools/determinism.sh` is **CI gate 9 of 19** (`tools/ci.sh:104-105`) and

R1's scope was every gate declaration in the script, not one line of it:
`tools/ci.sh:21` is `readonly GATE_TOTAL=20`, the last step is `gate 20` at
`:202`, gate 9 is `:109-110`, and `:104-105` is
`# It runs last of the two engine gates because it is the slowest: two processes`
`# over the whole sha-pinned fixture set at two budgets.` — comment prose.

This is round 1's BLOCKING 5 with its stated fix applied to one of the two sites.
It is BLOCKING and not minor for three reasons the document itself supplies:

1. **§2 is the soundness section.** The sentence it appears in is the reduction
   the whole package rests on (*"the cache's soundness is the determinism law …
   already enforces"*). A reader who follows the citation lands on a comment.
2. **The document now knows better in the same file.** A claim a document makes
   twice, in two directions, is D-423's named failure mode, and CLAUDE.md calls it
   *"a defect waiting"*.
3. **The mechanism the design credits with catching it cannot.** R6: the document
   IS on gate 20's list at its own revision, and `design_citation_check.py` returns
   exit 0 with `tools/ci.sh:104-105` green, because `:48-52` only checks that a
   cited range lies inside the file. Round 1 established this at R11 and the design
   asserts the opposite at `:23-24`.

**A note that makes the fix larger than one line.** The wrong text is inherited:
**D-576 itself** (`docs/decisions.md:1220`) says *"`tools/determinism.sh` is CI
**gate 9 of 19** (`tools/ci.sh:104-105`)"*, and D-583 later moved the total to 20
and named `:109-110`'s neighbourhood. D-581 has already corrected two of D-576's
supporting claims; this is a third.

**FIX.** `:162` → *"CI gate 9 of 20 (`tools/ci.sh:109-110`; the total lives once,
at `:21`)"*, and one ADR line recording that D-576's gate citation is superseded
by D-583's `GATE_TOTAL=20` — otherwise the next document to quote D-576
reproduces it a fourth time.

## BLOCKING B — §6's T2 NAMES AN INSTRUMENT THAT DOES NOT EXIST, AND IT IS THE ONLY ROW IN THE SUITE THAT A CACHE WHICH NEVER RUNS WOULD FAIL

§6, `:278`:

> | T2 | the cached run makes **fewer engine asks** than the uncached one,
> **measured from the stub's own count**, so the cache is exercised rather than
> merely present |

**There is no such count.** R7: `crates/pistol-arena/src/bin/stub_engine.rs`
writes exactly one line outside the protocol — `:382`, a startup banner — and
holds no counter of any kind. Its eighteen `Behave` variants (`:132-149`) are
protocol deviations, not instrumentation. `git grep -n -i "ask_count\|n_asks\|go_count"
-- crates/pistol-arena` returns nothing. **And `src/bin/stub_engine.rs` has no row
in §1's change list**, so the design registers a test against an instrument it does
not register building.

**WHY THIS IS BLOCKING RATHER THAN A MISSING ROW.** T2 is the only row in §6 that
can distinguish a working cache from an absent one. Take the design's own
motivating mutant — `bin/arena.rs` parses `--label-cache`, `passes::capture`
ignores it (round 1's MAJOR 11(b), the class D-553 exists for) — and walk §6:

- **T1** (cached capture byte-identical to uncached) — **passes**, trivially: a
  cache that never runs produces the uncached bytes.
- **T3** (the `--census` refusal) — **passes**: it is a `bin/arena.rs` match arm
  and never reaches `passes::capture`.
- **T4** (stray line refused at the same prefix) — **passes**: X3's guard is
  hoisted into `run`'s loop unconditionally, cache or no cache.
- **T5** (counters) — **passes**: §1 row 3 makes the counters unconditional; they
  count misses whether or not anything is memoised.
- **T6** (record order and fields unchanged) — **passes**, for T1's reason.
- **T2** — the only row that fails, and it cannot be taken.

So the suite as registered is green on a package where the cache is dead code.
That is the vacuous-criterion class `docs/process.md` forbids, at the level of the
whole suite rather than one assertion.

**AND THE REGISTRATION'S OWN FALLBACK IS ALSO VACUOUS**, which is why the design
cannot simply point at it. `wp21_throughput_prereg.md:771-772` registers dry-run
limb 4 as *"`tools/label_cache_count.py` reports a hit rate above zero on that
capture, so the cache was actually exercised"*. That instrument reads the capture
FILE and divides asked prefixes by distinct `position` lines
(`tools/label_cache_count.py:167`, `:175`, `:179-181`) — **a number identical for a
cached and an uncached capture**, since T1 requires the two files to be byte-equal.
Limb 4 cannot fail on a cache that never ran either.

**FIX.** Name a real instrument and give it a §1 row. The cheapest that fits the
tree's own conventions is a `Behave` variant that reports its `go` count — the
stub already carries variants whose entire reason is *"without an engine that can
produce one, the guard's CALL is unreachable from any test and a call-removed
mutant survives"* (`stub_engine.rs:73-79`, `:119-126`), which is exactly this
argument. Alternatively `passes::capture` prints the ask count beside the counts
row 4 already registers, and T2 reads stdout — which also discharges MAJOR 13's
auditability half. Either way §1 gains the site and §5 gains the flag-threading
mutant that dies at T2.

## BLOCKING C — §1's CHANGE LIST NO LONGER REGISTERS THE LOOKUP OR THE INSERT, AND §5 SPECIFIES MUTANTS AGAINST THEM AS SITES

Revision 1's row 1 read *"before `ask`, a lookup on the `position` string; after
`ask`, an insert"*. Revision 2's row 1, `:42`, reads in full:

> | 1 | `crates/pistol-arena/src/capture.rs` | `run` takes a `LabelCache` **MODE**
> — a two-state enum, not a map — and **builds the map itself**, beside
> `label_go_line` at `:333` |

`/usr/bin/grep -n "lookup\|insert" <design>` returns four hits, **all four inside
§5** (`:258-261`) and one in §3's X2 row. **The cache's two operations are named
nowhere in the document that is the IMPL contract.** Round 1's BLOCKING 2 was *"the
change list reaches neither end of the code it changes"*; revision 2 added both
ends and deleted the middle.

This is not bookkeeping. D-568's standing law is that a mutation set is specified
against **call sites**; §5 mutates *"the lookup"* and *"the insert"* as though they
were sites, and no section of the document says where either is, on which side of
`ask`, or how the hit path rejoins the miss path's record construction. §7's
rejection of the `no_tab` finding depends on exactly that unstated shape — *"A hit
skips `ask`; it does not skip record construction"* (`:301-302`) is a claim about a
control flow the design never draws. It is derivable from §2.1 (the memo holds
post-`normalise` bytes, so the hit path cannot re-enter `normalise`), but a reviewer
should not have to derive the change list.

**AND §5's KILL CRITERIA POINT AT A TEST §6 DOES NOT HAVE.** `:258` and `:260` both
say *"dies at the hit-count test"*. §6 has no hit-count row: T2 counts **asks**, not
hits, and per BLOCKING B cannot be taken at all. So two of §5's six mutants are
registered as dying at a test that does not exist, and a third (*"dies at the first
record"*, `:259`) names no test.

**FIX.** Restore the lookup and the insert to §1 with their positions relative to
`ask` and to the record literal at `capture.rs:352-358`, and make §5's kill criteria
cite §6's rows by name.

## BLOCKING D — §5 IS BYTE-IDENTICAL TO REVISION 1, SO MAJOR 11 IS UNDISPOSED — AND THE REVISION ADDED A GUARD (X3) AND GAVE IT NO MUTANT

R10: `git diff 239f21f fde1497 -- docs/experiments/wp21_label_cache_design.md`
produces no hunk for §5. Against D-553 (`docs/decisions.md:1174`), standing law —
*"for every guard or invariant, the mutation set includes a call-REMOVED mutant,
and it must die at a test that drives the call site with reachable input"* — the
set at `:257-267` is missing, at this revision:

- **X3's call-removed mutant.** The design's own new guard. Hoisting
  `channel.unsolicited()` out of `ask` into `run`'s loop creates a fresh call site;
  delete it and T4 is the only thing that could notice. §5 does not name it and §3
  does not either.
- **X2's mutant** — still absent, still the finding round 1 raised at MAJOR 11(a),
  and X2 still appears exactly once in the document (`:193`, the §3 table row) with
  no test in §6 and no mutant in §5. A refusal with neither is prose.
- **The normalise-side mutants** BLOCKING 3's fix asked for by name (*"the memo
  stores the normalised totals"* / *"the hit path returns the stored totals
  directly"*). §2.1 now fixes the side; nothing pins it. T1's byte-identity would in
  fact kill the raw-side-skipping variant — that is worth saying in §5, and §5 does
  not say it.
- **The flag-threading mutant** (`passes::capture` ignores its mode) — the class
  BLOCKING B shows nothing else can catch.
- **X1's arm-REMOVED mutant.** §5 `:262` still says *"X1: the refusal's condition
  negated"*. Under the shape §1 row 5 registers there is no condition: X1 is a
  positional slice pattern in `bin/arena.rs`'s `match words` (`:39-86`), so
  *"negate the condition"* is not a well-formed mutation of it. §3 `:201-206` now
  states the right kill criterion — *"the refusal names both words"* — but §5, which
  is the section the mutation receipt is written against at IMPL, was not updated
  to match.

**FIX.** Rewrite §5 against §1's restored sites (BLOCKING C), one call-removed
mutant per guard including X3 and X2, each with its §6 row named and its **driver**
named per D-553.

---

# MAJOR

## MAJOR E — THE 42 IS TRANCHE ONE'S FLOOR, NOT "PER TRANCHE": THE SIXTEEN FLOORS RANGE 42..60 AND I MEASURED THEM IN SECONDS

§1.1, `:80-83`:

> within tranche one's 218 openings the symmetry fold takes 213 `k = 2` classes to
> 171, so **at least 42 `key_full` collisions per tranche are the OPENING BOOK's
> structure** … The counter reports its total and **the run log records that floor
> beside it**

and §9, `:326-327`: *"what the folds are worth is MEASURED at **42 searches a
tranche**, 0.72% of its misses (D-583)"*.

R2 reproduces 213/171/42 for tranche one exactly. R3 runs the same instrument over
the registered partition (`wp21_prereg.md:165-168`), and the sixteen floors are:

```
tranche   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16   sum
floor    42  51  48  55  47  48  50  44  53  48  51  60  45  56  48  46   792
```

So *"at least 42 per tranche"* is a true lower bound — it happens to be the
minimum — and it is the **wrong** floor for fifteen of the sixteen tranches, by 2
to 18. A run log that records `42` beside tranche 12's counter invites exactly the
reading §1.1 exists to prevent: a counter reading of 63 there is 3 above the book's
shape, not 21. The design's own sentence — *"a closure that reported 42 as a finding
would be reporting the book"* — applies to itself at fifteen tranches.

**§9's *"42 searches a tranche"* is simply false as a per-tranche measured value**;
the sweep-wide figure is 792, mean 49.5. The design inherits the wording from
D-583, which says *"WHAT THE FOLD IS WORTH IS 42 EXTRA SEARCHES A TRANCHE"* — so
D-583's own line is stale at the same point, and D-583's flip clause invites the
correction (*"the matrix's measurement flips if a future book's openings fold
differently, which the instrument re-takes in one command"*).

**This is D-291's class**: an estimate reachable by seconds of measurement,
asserted instead. It took me one loop over an instrument I had already written for
tranche one.

**FIX.** §1.1 carries the sixteen floors (or states the floor as *per-tranche,
derived by `tools/opening_prefix_fold.py` at the tranche's own slice, recorded in
each tranche's block*), and §9's *"42 a tranche"* becomes *"42 in tranche one,
42-60 per tranche, 792 across the sweep"*.

## MAJOR F — THE DESIGN MISQUOTES D-581's FLIP CLAUSE IN THE PARAGRAPH THAT CORRECTS REVISION 1's MISQUOTE OF IT, AND "MATERIALLY" IS STILL UNDEFINED

§1.1, `:85-88`:

> **AND D-581's FLIP CLAUSE HAS TWO CONJUNCTS, NOT ONE.** The selection flips only
> if the count is **materially above that floor** AND a seat pins `countermove`
> `false` as a rule rather than as a value.

D-581 (`docs/decisions.md:1230`), last sentence, verbatim:

> Flips: the selection flips only if that count is **materially above zero** AND a
> seat pins `countermove` false as a rule rather than as a value; either alone buys
> nothing.

`matrix_label_cache_key.md:253` says the same, and adds a population the design
also changes: *"If it is materially above zero **across the sweep**"*. The design
restores the conjunct round 1 said was dropped — correctly — and in the same
sentence silently **raises the threshold** from zero to a 42-floor and **narrows the
population** from the sweep to a tranche, attributing the result to D-581. A run log
written against this design applies a trigger the governing ruling does not
register. CLAUDE.md: *"Silent architecture drift is a breach; amend the ADR
instead."*

The change is probably an improvement — D-583 supplies the floor, and a trigger of
*"above zero"* against a guaranteed non-zero book contribution is a trigger that
always fires, which is D-424's other failure mode. That is an argument for an ADR
line, not for making the change inside a design.

**AND *"MATERIALLY"* IS STILL UNQUANTIFIED**, twice (`:69`, `:86`), which was round
1's MAJOR 7(iii) and is unaddressed. Against a floor that is itself wrong for
fifteen tranches (MAJOR E), the phrase now constrains nothing on two axes.

**FIX.** One ADR line amending D-581's flip clause to the floor-relative,
per-tranche form with a number for *"materially"*, and §1.1 cites it instead of
restating it.

## MAJOR G — THE DESIGN CONTRADICTS THE MATRIX AND D-581 ON THREE POINTS AND RECORDS NONE OF THEM, IN A REVISION THAT PUT ITSELF ON THE SAME CITATION GATE AS THE MATRIX

Round 1's MAJOR 10(c) asked for one sentence and got none. The disagreement has
since grown from one item to three, and now includes an ADR:

| the design says | the governing document says | where |
|---|---|---|
| *"THE COST IS ONE REPLAY PER MISS, which the cache does not otherwise need"* (`:71`) | *"Two counters, **no extra search, one sort the arena already does**"* | `matrix_label_cache_key.md:251`; **and D-581 `:1230`: *"Two counters, no extra search"*** |
| flips *"materially above that floor"*, per tranche (`:86`) | *"materially above zero **across the sweep**"* | `matrix_label_cache_key.md:253`; D-581 `:1230` |
| *"THE CLAIM THAT THIS 'SETTLES D-562(2)' IS WITHDRAWN"* (`:90`) | *"**It also settles D-562(2)'s open three-key question in the same pass**"* | `matrix_label_cache_key.md:256`; D-581 `:1230` |

**The design is right on all three** — I checked the first myself: `capture::run`
builds no `GameState` after `asked_prefixes` discards its own (`capture.rs:28-44`,
`:338-364`), and the stones/`canonical_form` work lives in `labels.rs:196-203`, a
different pass over a different file. There is no sort in the capture pass to
piggyback on.

But `matrix_label_cache_key.md` is on `tools/governing_citation_check.sh:51` and
this design added itself to `:53` in the same commit — so as of `fde1497` **CI gate
20 runs over two documents on its own list that contradict each other on what the
counters cost, when the selection flips, and what the pass settles.** A design that
silently contradicts a governing document is the drift CLAUDE.md names.

**FIX.** Either amend the matrix to revision 4 (which reopens its review, and
should carry the D-562(2) withdrawal and the corrected cost), or add one line to
§1.1 recording all three disagreements and stating that the matrix is superseded on
them. Silence is the breach; the design already demonstrates it knows how to record
one, at `:90-94`.

## MAJOR H — §2.1 PICKS THE OPPOSITE SIDE FROM THE REGISTRATION'S OWN WORDING WITHOUT SAYING SO, AND ITS ARGUMENT AGAINST THE ALTERNATIVE MISDESCRIBES IT

The registration, `wp21_throughput_prereg.md:529-530` (§4.2, *"What is
implemented"*):

> A memo in `capture::run`: §2's key … to **the `(totals, bestmove)` the engine
> returned**.

*"the engine returned"* is `ask`'s raw pair (`capture.rs:273`, `return Ok((totals,
line))`), carrying ` nps <n> time <n>`. §2.1 `:130-131` registers the opposite:

> **THE MEMO STORES THE POST-`normalise` PAIR: exactly the two strings the record
> carries.**

**The choice is sound and I am not disputing it** — the two are equivalent up to a
pure function (`capture.rs:66-96` takes a `&str` and reads nothing else), and
storing the record's own bytes does make the hit reproduce them by construction.
Two things are wrong around it.

**(a) THE DIVERGENCE IS NOT RECORDED.** The header says *"THIS DOCUMENT DESIGNS
ONLY WHAT THE REGISTRATION NAMES"* (`:34`), and this is a place where it designs
something else. One sentence — *"§4.2 says 'the (totals, bestmove) the engine
returned'; this design fixes the normalised pair, which is the same value through
a pure function and removes an ordering choice"* — closes it. Without it a
REVIEW-impl reading §4.2 will implement the raw side.

**(b) THE ARGUMENT AGAINST THE ALTERNATIVE IS NOT AN ARGUMENT.** `:141-143`:

> The bad case is a memo storing raw and normalising on the way out, **where the
> answer depends on where the code happens to call it**.

Store raw and build every record through the single existing
`totals: normalise(&totals)?` at `capture.rs:356` and there is exactly one call
site — which is precisely round 1's recommended fix (*"the record-construction
block is shared and not duplicated"*). The design labels the reviewer's own remedy
"the bad case" on a ground that does not hold once the shape is stated, and the
ground it does hold on (a hit path that early-returns and skips `normalise`) is
implementation 3, which the design has already dispatched two sentences earlier.

**FIX.** Keep the side; replace `:141-143` with the true comparison — *both sides
are correct if the record is built at one place; the post-`normalise` side is
chosen because it makes the hit path incapable of re-entering `normalise` at all* —
and record the divergence from §4.2.

## MAJOR I — X1's ARM COUNT IS STILL UNREGISTERED, AND THE UNREGISTERED SPELLING IS REFUSED BY A MESSAGE THAT NAMES NEITHER WORD AND SUGGESTS A REORDER WOULD WORK

The dispatch asks how many arms X1 actually needs. Read `bin/arena.rs:39-86`: the
`match words` is a **positional slice pattern**. The capture mode has two arms
today — `:51-58` (no census) and `:59-74` (trailing `"--census"`). Adding
`--label-cache` as a trailing word makes **five reachable spellings**:

```
[--capture s --out o --label-nodes n]                        legal   (exists, :51)
[--capture s --out o --label-nodes n --census]               legal   (exists, :59)
[--capture s --out o --label-nodes n --label-cache]          legal   (NEW)
[--capture s --out o --label-nodes n --census --label-cache] refused (X1)
[--capture s --out o --label-nodes n --label-cache --census] refused (X1)
```

— four arms, or three if the two refusal orders are written as one or-pattern.
§1 row 5 `:46` says only *"the `--label-cache` word, and X1's **refusal arms**"*;
§3 `:202` discusses deleting *"X1's **arm**"*, singular; §6 T3 `:279` says
*"`--label-cache` with `--census` is refused naming both words"* without saying both
orders are tested.

**If only one order is written, the other falls to the catch-all**
(`bin/arena.rs:79-85`), which returns

> `--config and --out are both required, or --replay, --out and --workers, or
> --capture, --out and --label-nodes, or --labels, --report and --out, **each in
> that order**`

— a message that names neither `--label-cache` nor `--census`, exits 2 like X1
does, and actively tells the operator that reordering would help. It would not:
the other order is X1's own refusal. §3's registered kill criterion (*"the refusal
names both words"*) is then satisfiable by a T3 that tests only the spelling that
was implemented, which is round 1's MAJOR 11(d) unclosed.

**FIX.** §1 row 5 enumerates the five spellings and says which are legal; §6 T3
asserts the refusal names both words **in both orders**; §5 gains the arm-removed
mutant per refusal arm.

## MAJOR J — §8's FIRST OBLIGATION AND THE HEADER'S GATE ARGUMENT ARE BOTH FALSE AT THIS REVISION

Header `:22-24`:

> **The design is not on `tools/governing_citation_check.sh`'s list**, and these
> three would have been caught if it were; adding it is §8's first obligation.

§8 `:312`: *"| **this document joins `tools/governing_citation_check.sh`'s list** |
the IMPL commit — revision 1 carried three wrong citations **that the gate would
have refused** |"*

R5: `docs/experiments/wp21_label_cache_design.md` is at
`tools/governing_citation_check.sh:53` **as of `fde1497`, the commit that carries
this revision**, together with a `--proposes` entry for
`crates/pistol-arena/tests/label_cache_tests.rs` at `:63-64`. It is absent at both
`239f21f` and `f1acc57`. So the document's account of its own status is wrong, and
§8 tells a successor an obligation is outstanding that is discharged.

**And the gate argument is wrong in the other direction.** R6 runs the instrument
gate 20 runs, over this document: exit 0, `19 citation(s) checked`, with
`tools/ci.sh:104-105` (BLOCKING A) **green**, because
`design_citation_check.py:48-52` checks only that a cited range lies inside the
file. Of the *"three"* wrong citations, one (*"CI gate 6"*) is bare prose the
checker never sees, and two are wrong lines of a right file it cannot see. The gate
would have refused **none** of them. `docs/process.md:111-115` says exactly this:
*"WHAT IS MECHANIZED, AND IT IS ONLY THE CHEAP HALF … It catches ROT — a path or
line that moved — and it catches nothing about scope."*

**FIX.** Header → *"the design joined the list at `fde1497`, and the gate is green
on §2's wrong lines, which is why a reviewer and not a gate found them"*; §8's row
→ **discharged**, with the commit named.

## MAJOR K — §3's MOOTNESS ARGUMENT FOR THE CENSUS GUARD USES THE PREMISE §2.0 SPENDS A SECTION DEMOLISHING

§3, `:208-212`:

> **THE SECOND CENSUS GUARD INSIDE `ask` IS MOOT** … **Under X1 a cached run is
> never a census run**, so there is no census row for a hit to miss.

X1 is a `bin/arena.rs` match arm (`:39-86`), and `bin/arena.rs` is a **separate
crate** from the `pistol-arena` library. `capture::run` is `pub`
(`capture.rs:326`) and takes `census: &mut CensusSink` carrying
`request: CensusRequest` (`:210-215`); under §1 row 1 it will also take
`LabelCache::On`. **The combination X1 forbids is representable at the seam where
it does damage**, and the only thing forbidding it lives in another crate.

Three sections earlier, §2.0 `:109-111` rejects revision 1's parameter shape on
precisely this ground:

> `capture::run` is `pub` (`capture.rs:326`), so "every caller" includes ones that
> do not exist yet. **An invariant a REVIEW-impl cannot check by reading the
> function is not an invariant.**

The design therefore holds that `capture::run`'s future callers matter for the
memo's lifetime and do not matter for the census combination, without saying why
the two differ. They do not: `docs/rule9_justifications.md:82` names this exact
shape as the reason the census sink lives in `capture.rs` at all — *"Splitting the
row-keeping from the classifier that recognises a row would put a guard's decision
in one file and its consequence in another, which is exactly the shape that let a
token be honoured by the engine and its rows dropped downstream at exit 0."*

This is round 1's MAJOR 14, undisposed, and revision 2 made it load-bearing by
building §3's mootness argument on top of it.

**FIX.** Round 1's, unchanged: keep the CLI refusal (it is what gives D-200's
no-file-left-behind property, and I re-verified it — see *WHAT SURVIVED* 4) **and**
make the combination unrepresentable at the library seam, either by a first
statement of `run` refusing `census.request == On && cache.is_on()` with its own
call-removed mutant, or by one closed enum so it cannot be spelled.

---

# minor

**minor L.** §7 `:292` presents its reproducer as `` `capture.rs:355-360` ``. The
four quoted lines are `:356-359`; `:355` is `position,` and `:360` is
`out.push(record);`. The rejection's substance is right (see *ATTACKS REJECTED* 2)
and `design_citation_check.py` passes the range because it lies inside the file —
which is the same blind spot BLOCKING A turns on, in the section that rejects a
finding about citations.

**minor M.** §2.0 `:119-120`: *"builds its own `BTreeMap<String, (String, String)>`
locally, beside `label_go_line` at `:333` — **inside the closure**"*. `:333` is
**outside** the closure, which opens at `capture.rs:338`
(`seats::with_seats(&seats, transcript.hang_timeout_ms, |channels| {`); the
registration says so itself at `wp21_throughput_prereg.md:252-255` (*"computed once
at `capture.rs:333`, outside both the game loop (`:340`) and the prefix loop
(`:341`)"*). Both placements compile (`with_seats` takes `impl FnOnce`,
`seats.rs:25`) and both satisfy the invariant, so this changes no conclusion — but
the sentence names two places and an implementer must pick one.

**minor N.** §7 `:304-305`: *"which is what **its registered mutant** is for"*, of
`no_tab`. The mutant exists — `docs/experiments/wp20_pilot_artifacts.md:81`, one of
nine call-removed mutants at `4375ad9`, *"All nine DIE"* — but the design cites no
document for it, and §5 does not carry it, so a reader has no way to check the
claim the rejection leans on.

**minor O.** §3's X3 row says the defect includes a stray line *"missed at the end
of a game"*. The **uncached** pass has the same residual: `ask`'s guard runs before
each ask, so nothing checks after the final prefix of the final game in either pass
(`capture.rs:241-246` is `ask`'s first statement, and `run`'s loops at `:340-362`
end without a trailing drain). X3 restores parity, which is what it claims in its
strong sentence (*"must refuse the same input at the same place"*); the residual it
does not close should be said so a successor does not read the hoist as closing it.

---

# WHAT SURVIVED ATTACK

1. **§2.0's mode-not-a-parameter shape is genuinely structural, and I tried to
   break it.** `seats::with_seats` takes `drive: impl FnOnce(&mut [Channel; N])`
   (`seats.rs:25`) and calls it once (`:55`), so a `BTreeMap` declared at
   `capture.rs:333` and mutably captured, or declared inside the closure body, both
   compile and both drop when `run` returns. `LabelCache { Off, On }` carries no
   payload, so **there is no way to hand a memo in and no way to get one out** —
   the registration's invariant (*"constructed inside `capture::run` and dropped
   with it … never shared between two invocations"*,
   `wp21_throughput_prereg.md:249-251`) becomes readable in one function, which was
   round 1's whole complaint. It is the right seam for `passes::capture`'s
   signature too: `passes.rs:40-46` already threads `census: Option<(PathBuf,
   File)>` from `bin/arena.rs`, and a two-state enum threads identically with no
   lifetime attached.

2. **§2.1's *"fails loudly"* claim about `normalise` is TRUE.** `capture.rs:69-71`:
   `line.find(&nps).ok_or_else(|| refuse(format!("a totals line carries no `nps`
   field: `{line}`")))?`. A second `normalise` of an already-stripped line errors
   rather than passing. And the strip removes the only occurrence — the result is
   `line[..at] + line[end..]` (`:95`) with `at` the first ` nps ` — so no
   already-stripped line can carry a second one. **The design's safety argument
   holds.** (Its side-choice has MAJOR H's problems; the mechanism claim is sound.)

3. **X3's hoist preserves the refusal's message, its position and its meaning.** I
   enumerated `ask` (`capture.rs:231-319`) line by line:
   - `:240` `where_` — a formatting closure over `game` and `k`, **both in scope in
     `run`'s loops** (`:340` `for game in &transcript.games`, `:341` `for k in
     asked_prefixes(game)?`, used at `:353-354`), so the message
     `"game {game}, turn {k}: the engine spoke before it was asked ({stray:?})"`
     reproduces byte for byte.
   - `:241-246` the `unsolicited()` guard — **the first statement of the function**,
     before the sends at `:247`, so hoisting it to the top of the prefix-loop body
     is the same point in the sequence. `ask` is private with exactly **one** call
     site (R9: `capture.rs:231 fn ask(`, `capture.rs:343`), so no other caller loses
     it.
   - `:247-251` the three sends and *"the engine closed its input"* — not skipped in
     any meaningful sense: on a hit nothing is sent, and the next miss's hoisted
     `unsolicited()` returns `Some(Received::Closed)` on a disconnected pipe
     (`channel.rs:199-204`, `Err(TryRecvError::Disconnected) => Some(Received::Closed)`),
     so an engine that died mid-cached-stretch is still caught. The design does not
     claim this and gets it for free.
   - `:252-274` the receive loop, `Closed` / `Overlong` / the `bestmove` arm and its
     *"no totals line this driver recognised"* — every one fires **only in response
     to lines the engine wrote for this ask**. On a hit there is no ask, so there is
     nothing for them to be wrong about; they are structurally moot, not skipped.
   - `:288-295` the census-off refusal — moot under X1 as §3 says, subject to
     **MAJOR K**'s placement objection.
   - `:303-309` the census-row TAB guard — moot for the same reason, and the design
     does not name it; that is correct rather than an omission, since it is
     downstream of the same census-off condition.

   **Nothing else inside `ask` stops running on a hit that the design has not
   named.** X3 is the complete list.

4. **X1's timing claim, re-verified.** `bin/arena.rs:39` opens
   `let (mode, out_path) = match words {`, `:86` closes it, and
   `let claimed = outpath::claim(&out_path)` is `:89`. Every arm, catch-all
   included, returns before it. *"it fires from the argument parse, before `--out`
   is claimed"* is **TRUE**, and the design's *"VERIFIED, not assumed"* line cites
   the right two ranges.

5. **§7's rejection is RIGHT, and the reviewer did not mean something it fails to
   address.** `capture.rs:359` `no_tab(&record)?` is in `run`, after the record
   literal closes at `:358`, outside `ask`. Round 1's own FIX text cited `:359`
   correctly, so the reviewer knew where the call was; its point was that the
   design named no shape for the hit path, which the rejection answers indirectly
   (§2.1's post-`normalise` memo forces the hit to rejoin at the record literal).
   **The rejection is sound; what remains of round 1's concern is BLOCKING C** — the
   change list no longer draws that path at all.

6. **§2's `key_pos` correction.** `labels.rs:202` is `key_pos:
   state.key().to_string()`, a `Key128` rendered as hex; `:203` is
   `key_full: render_key_full(&canonical_form(&stones))`. The design's *"The
   counters compare stone lists and canonical forms, never a digest"* is the right
   repair.

7. **The play-order parenthesis.** `heuristics.rs:89` is
   `for played in state.played()` inside `record_cutoff`; `heuristics.rs:153-155` is
   `if gates.countermove && state.phase() == Phase::First && let Some(at) =
   last_stone(state)`. *"under ALL three gates, not two … the only read that can
   move a CHOICE is `last_stone` at `:155` under `countermove` alone"* is correct
   under both readings, which no earlier document in this arc managed.

8. **742 / 347 / 53% / 47% / 79**, all reproduced independently (R4; `turn_cap = 40`
   at `wp21_prereg.md:149` gives `1 + 39×2 = 79`).

9. **42 for tranche one**, reproduced by my own `canonical_form` (R2) — exactly
   213 → 171. It **is** a genuine floor for the `key_full` counter and not merely an
   estimate: at a fixed prefix depth the miss set is exactly the distinct exact
   prefixes, collisions are `exact − symmetry`, positions at different depths hold
   different stone counts so no cross-depth collision is possible, and the book
   contributes 0 at k=0,1 and 3. The design's *use* of it holds for tranche one and
   fails as a per-tranche number (**MAJOR E**).

10. **The `key_pos` counter has no book floor, and the design correctly claims one
    only for `key_full`.** My fold gives stone-set classes equal to exact classes at
    every depth (k=2: 213 = 213), because a pair token is canonically spelled.

11. **§6's rows T1, T4, T5, T6 are each falsifiable** as written, and T6's
    forfeit / rule-4-win report is the right extra population (`asked_prefixes`
    drops the last prefix of a won game, `capture.rs:42-43`). The suite's defect is
    not any single row but that **only T2 can fail on a cache that never runs**, and
    T2 cannot be taken (**BLOCKING B**).

---

# ATTACKS I ATTEMPTED AND REJECTED

1. **"Building the map inside the closure cannot work, because `with_seats` needs a
   closure that can be called more than once."** REJECTED. `seats.rs:25` is
   `drive: impl FnOnce(&mut [Channel; N]) -> Result<T, ArenaError>`, invoked once at
   `:55`. Both placements the design gestures at compile and both give one map per
   `run`. Only the prose is confused (**minor M**).

2. **"The hit path skips `no_tab`, so round 1's BLOCKING 3 half stands."**
   REJECTED with the same reproducer the design gives, checked at the file:
   `capture.rs:352-358` builds the record, `:359` calls `no_tab`, `:360` pushes.
   `ask` ends at `:319`. A hit that returns a `(totals, bestmove)` pair before the
   record literal still passes through `:359`.

3. **"`normalise` is idempotent, so a raw-side memo would silently double-strip and
   the design's 'fails loudly' is wrong."** REJECTED — this was the dispatch's
   BLOCKING candidate and it does not land. `capture.rs:69-71` errors when ` nps `
   is absent, and `:95` guarantees the first strip removes the only occurrence.

4. **"An engine that dies while the pass serves cached answers is never noticed,
   which X3 does not cover."** REJECTED. `channel.rs:203` maps
   `TryRecvError::Disconnected` to `Some(Received::Closed)`, so the hoisted guard
   fires. The refusal's wording (*"spoke before it was asked (Closed)"*) is
   imprecise, but it is `ask`'s existing wording and not a change this package
   makes.

5. **"T5's zero-counter row is unachievable, because every game shares the `k = 0`
   and `k = 1` prefixes and those collide under `key_full`."** REJECTED. Shared
   prefixes are cache **hits** after the first game, and the counters count
   collisions among **misses** only (§1.1 `:57-60`); at k=0 and k=1 there is exactly
   one miss each and so zero collisions. My fold confirms 1 class under every key at
   k=1.

6. **"The 42 is not a floor for the counter, because the counter counts records and
   the fold counts openings."** REJECTED — see *WHAT SURVIVED* 9. The two coincide
   at a fixed depth.

7. **"`--label-cache` is a code-side default for a tunable and breaches hard rule
   1."** REJECTED, as round 1 rejected it and as
   `wp21_throughput_prereg_rev3_REVIEW.md`'s A11 rejected it before that:
   `usage.rs:14`'s `[--census]` is the tree's convention for a capture-mode switch,
   and a mode selector is not a tunable. What survives from the attack is the
   auditability half — round 1's MAJOR 13, still **PARTIAL**.

8. **"§1 row 4's *'`:82-96` is where every line this pass writes is written'* is
   false, because `census_file::write_into` also writes."** REJECTED. `:76` writes
   to the census FILE, not to stdout; every `println!` in the pass is at `:82`,
   `:87`, `:91`, `:93`, `:94`, `:95`. The row's claim is exact.

---

## THE SHORT VERSION

Revision 2 fixes the three things it says it fixes — the memo is a mode, the
normalise side is stated, the stray-line guard is hoisted — and all three survive
attack. Its rejection of round 1's `no_tab` finding is correct. Its 742/347, its
53%/47%, its 79 and its 42 all reproduce under commands I chose.

It fails on four things. §2 still carries the exact wrong gate citation the header
boasts of having found (**A**). The one test that could fail on a cache that never
runs is measured with an instrument that does not exist, and the registration's
fallback instrument returns the same number cached or uncached (**B**). The change
list dropped the cache's own two operations while the mutation set kept mutating
them (**C**), and the mutation set is byte-identical to revision 1's, so a new guard
shipped with no mutant and MAJOR 11 is undisposed (**D**). Around those, the `42`
is fifteen-sixteenths wrong as a per-tranche floor, and the design changes a
governing ruling's flip clause in prose.

**VERDICT: FAIL** — 4 BLOCKING, 7 MAJOR, 4 minor, with 9 of round 1's 22 findings
still OPEN and 6 PARTIAL.
