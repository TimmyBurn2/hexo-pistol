# REVIEW-design — `docs/experiments/wp20b_design.md` revision 3

## Header

**Revision reviewed:** `e364497c080d6235f4fed6661bf81e2929cc5d2b` — a `git stash
create` object holding the uncommitted work on top of HEAD `a56449b` (`dev`). It
is NOT a commit on any branch and does NOT match HEAD.

**Does it still match the tree?** **YES.** At the end of this review a second
`git stash create` returned `a7a8044`, whose tree is
`a551a87595fc55c1663c3d403e5490eebc39cf71` — byte-identical to `e364497`'s tree.
`git status --porcelain` shows the same six paths it showed at the start. The
only file added afterwards is **this report**, the one file this review was
permitted to write. No source, config, doc or ADR file was modified.

**Reviewer:** fresh context. I did not author the design and I am neither of the
two reviewers whose reports it responds to.

**What I read.** `CLAUDE.md`; `docs/process.md` (Process section obligations as
cited); `docs/experiments/wp20b_design.md` in full; both prior reports in full
(`wp20b_design_rev2_REVIEW.md`, `wp20b_decision_REDTEAM.md`); the uncommitted
tail of `docs/decisions.md` (D-562, D-563, D-564) and the standing lines D-6,
D-8, D-34, D-88, D-441 (through `configs/play_staged_solver_v0.toml`'s header),
D-465, D-481, D-508, D-516, D-520, D-537 (through `wp20s_design.md` §8), D-539,
D-547, D-559, D-560, D-561; the whole diff of `docs/experiments/wp20_dispatches.md`
including both transcribed dispatch texts word by word; `wp21_DISPATCH.md`;
`wp20s_design.md` §8. In source: `crates/pistol-search/src/{census.rs, search.rs,
pvs.rs, info.rs}`, `crates/pistol-engine/src/{engine.rs, instance.rs, lib.rs,
position.rs}`, `crates/pistol-cli/src/{protocol.rs, report.rs}` and
`crates/pistol-cli/tests/common/mod.rs`, `crates/pistol-arena/src/{capture.rs,
exchange.rs, labels.rs, labels_file.rs, config.rs, passes.rs, bin/arena.rs,
bin/stub_engine.rs}`, `crates/pistol-arena/tests/capture_tests.rs`,
`crates/pistol-core/src/{symmetry.rs, zobrist.rs, state.rs}`, all 38 files under
`configs/`, `tools/baseline_snapshot.sh`, `tools/determinism.sh`. Artifacts:
`artifacts/wp20b_prechange_RECEIPT.txt` and the four `prechange_*` records.
**D-401 was not read.**

**What I computed rather than accepted.** §4's row bytes from the field list
(python, exact string construction); the sweep-volume arithmetic; the invariant-7
overflow bound from `Symmetry::apply`'s own arithmetic; the invariant-block
digests of §9's pre-change record (`sha256sum` over
`sed -n '1,/^# timing/p'`), which reproduce **exactly**.

**What I could not check.** (1) **Revision 2 is not recoverable** — `git log
--all` carries no earlier version of the design file, so I could NOT diff rev 2
against rev 3, cannot verify the revision header's implicit negative claims
("WHAT REVISION 3 DOES NOT CHANGE"), and cannot audit D-547's passed-section
freeze mechanically. I checked instead that every row of the header's table
corresponds to text that is actually in the named section. (2) I did not build or
run anything: no `cargo`, no worktree, no `CARGO_TARGET_DIR`, no `tools/ci.sh`,
no `tools/determinism.sh`, no `tools/bench_delta.sh`. (3) I could not re-run the
red team's R-CHECK / R-COST / R-FIRE measurements; I checked their internal
consistency and their transcription instead, which is how Q7 and S9 were found.

---

## VERDICT: **FALLS**

Four BLOCKING (P1–P4), eight MAJOR (Q1–Q8), fifteen MINOR (S1–S15).

**The fix round did the hard half well.** B1/R8's retraction is complete and
correctly scoped — I verified the three arming configs and grepped the whole
document and D-563 for residual universals and found none. R1's re-derived
arithmetic is **exactly right**: I recounted 281 B, 291 B and 859 B from §4's own
field list character by character, and 2.97·10⁷ rows, 8.65 GB, 25.55 GB, a 2.95×
ratio and the ~2.5 GB positional-line corollary all reproduce. §9's pre-change
record is not merely claimed: I recomputed `7f8a6f97…` and `06490795…` from the
committed receipt's own files and both runs' invariant blocks are equal. R3, R4,
R9, R13, R15, R16, M2, M5, M6 and eight of the thirteen N-findings are cleanly
discharged.

**It falls on what the fix round itself introduced**, which is what a fix round
is for a reviewer. The R14 handshake line the design adopts writes into the very
block §9's byte-identity obligation digests — I verified the block contains ten
`engine_id` lines — so the document now carries two obligations that cannot both
hold, one of them a STOP condition (P1). The seat column that is *the* fix for
the blocking finding leaves the same vacuity alive on the two tests whose named
call-removed mutants can only die on an armed seat (P2). The `Engine` seam M4
demanded be stated is stated, and the statement breaks §5's coldness proof
without noticing (P3). And the bench guard M1 asked to be made non-vacuous is
re-registered as an ungated "magnitude" whose stated derivation cannot be
reproduced from its own stated inputs (P4) — the remedy R12 asked for, inverted.

---

## DISCHARGE TABLE

37 rows: B1, M1–M6, N1–N13, R1–R17. `DISCHARGED` / `PARTIAL` / `NOT DISCHARGED`.

| ID | verdict | evidence |
|---|---|---|
| **B1** | **DISCHARGED** | `/usr/bin/grep -rn on_search_path configs/` gives exactly 3 of 18 `true` — the three the design names, at the lines it names. The design and D-563 both carry the retraction at the head of the finding they correct; grepping the design and `decisions.md` for "every/any/no committed config" returns only the retraction, the quoted `play_staged_solver_v0.toml` header, and the token requirement (a different subject). The narrow claim (`configs/instrument_v0.toml:113`, the config both pilot seats run) is true and correctly scoped |
| **M1** | **PARTIAL** | The guard is split onto two named seats, which is the fix. But (ii)'s registered 0.2–0.5 % cannot be derived from its own stated inputs (22.99 µs × 95–159 firings at a 50 000-node bench seat is ~2–3 %; 95–159 was measured at `nodes 400000`), and (ii) is registered **ungated** — "recorded as a magnitude rather than gated". See **P4**. And (i) is vacuous against the one mutant §8 assigns to the non-census path — see **P2** |
| **M2** | **DISCHARGED** | Option B's two cost cells are now "a read of `GameState::played()` inside the closure … same order as C1" and non-census cost "none". `git grep "walking its history"` over the design returns nothing: the inverted quotation is gone |
| **M3** | **DISCHARGED** | Test 14 (`a_capture_run_with_the_census_flag_writes_the_rows_it_was_sent`, seat "solver-ON, driven through `arena --capture`"), invariant 8, and the call-removed mutant "the arena's sink never called" all exist. New defects in the *content* of that limb are **Q2** and **Q5** |
| **M4** | **PARTIAL** | §6.1 exists and states four bullets. Three of them are wrong or incomplete at the site (**Q1**), and the one rule the seam actually needs — when the searcher is armed and disarmed across `go`s — is absent, which is what breaks §5 (**P3**) |
| **M5** | **DISCHARGED** | `wp20_dispatches.md` limb 1 now says the D-527 phrase was already in v1 and the real difference is the range plus D-512/D-537; limb 6 now says the option field was edited and both options kept. I compared both transcribed texts word by word and both corrections are right |
| **M6** | **DISCHARGED** | D-564 lands, corrects D-539's reachability conclusion, keeps the dependency fact, cites `lib.rs:47-53` and `report.rs:2` (both verified verbatim), and states the manifest test stays green unchanged. Correct in substance and in the project's append-only ADR form. One wording note at **S15** |
| **N1** | **DISCHARGED** | §8 row 4 now reads "**v2's fourth mutant, the transposition ruling INVERTED** (N1)"; row 11 is relabelled "the fold OVER-folding — the opposite direction to test 4" |
| **N2** | **DISCHARGED** | "Test 4 pins key equality plus the stated counting rule, not a program (N2)" — the honest form the finding asked for |
| **N3** | **DISCHARGED** | "The sentence is withdrawn", and §4 now states the property test 12's own name states |
| **N4** | **DISCHARGED** | The mis-split hazard is withdrawn by name; the ground is now `labels.rs:14 EMPTY_FIELD` (verified at line 14) read back at `labels_file.rs:302` (verified) |
| **N5** | **DISCHARGED** | `git grep D-54y` over the design returns nothing. §9 now cites "D-481's form" for the STOP-and-split, which is a real line about a fired cap and a drawn split, not a dangling placeholder |
| **N6** | **PARTIAL** | F1, F2 and §10.0 now label v1 against v2 explicitly. **§5 still quotes v1's scope 4** — *"the design proves (quoted site) that no extra hashing or probe is added on the non-census path"* — under a v2 governing header, unlabelled; v2 reads *"quoted site proving no extra hashing on the non-census path"*. See **S4** |
| **N7** | **PARTIAL** | The ambiguity is resolved decisively and correctly (block, not per-depth). The finding's second half — "add the line-order assertion to test 8's fixture" — is not taken: no test in §8 pins that the block lands after the last `info` and before `info totals`. See **S14** |
| **N8** | **PARTIAL** | The per-field/per-row mislabel is fixed properly: the matrix now has two distinct rows, "bytes the IDENTITY FIELD adds" and "bytes of a whole row". "~119 800 distinct positions" is still used unmarked, and option A's cost is still marked "**MEASURED-free**" for a fact established by reading code |
| **N9** | **DISCHARGED** | The volume basis is promoted ESTIMATED → MEASURED with the two commands named and the numbers given, and the design says in terms that the marking was the defect (D-291) |
| **N10** | **NOT DISCHARGED** | §3.1 still reads *"A `go` line carrying the token when the engine was built without census support is a **named refusal**"*. There is still no feature gate, no `cfg` and no second build anywhere in §6. It is now stated **twice** — invariant 6 repeats it. See **S2** |
| **N11** | **NOT DISCHARGED** | `wp21_DISPATCH.md` §4 "Decision owed" still lists exactly three items and none of them is F3's arming question, in the same uncommitted diff as a D-563 that says WP-2.1 may not register a census until that gate is ruled. See **S3** |
| **N12** | **DISCHARGED** | §10.6 answers D-562(2) for the census and leaves the corpus dedup where D-562 put it |
| **N13** | **NOT DISCHARGED** | §8's first call-removed row is unchanged: it still names "the `go` handler's test of the third word, at `crates/pistol-cli/src/protocol.rs`'s `go` arm". The third word is parsed in `budget_token.rs` (`parse_budget`, called at `protocol.rs:169`); the `go` arm holds the conditional emission. Two removable calls, one named. See **S5** |
| **R1** | **DISCHARGED** | Recounted independently. Min row **281 B**, typical **291 B**, C1 **859 B** (291 − 32 + 600) — all three exact from §4's field order. 119 800 × 2.14 = 256 372 asks; × 116 = 2.974·10⁷ rows ("~3.0·10⁷" ✔); × 291 B = 8.65 GB ("~8.7" ✔); × 859 B = 25.55 GB ("~25.5" ✔); 859/291 = **2.952** ("2.95×" ✔). The positional-line corollary: 84 B typical → **2.50 GB** ("~2.5 GB" ✔). The whole arithmetic paragraph is right |
| **R2** | **DISCHARGED** | The `Copy` cost row is withdrawn by name, and the real C1 penalty (an allocation and a `String` build inside the closure, per firing) is stated as the argument. `TriggerObservation` still derives `Copy` and `Key128` is `Copy`, so C2 preserves it. The count "eleven sites" is wrong — `git grep TriggerObservation -- crates/` outside `census.rs` gives **seven** — but the conclusion holds at every one of them (**S9**) |
| **R3** | **DISCHARGED** | D′ is added as a full matrix column, priced honestly ("zero sorts, zero allocations; cheaper than C2"), and killed on the stated ground — its representative is not `canonical_form`'s, so it is the fourth notion of sameness §2.1 names. The three further shapes are recorded as failing |
| **R4** | **DISCHARGED** | The estimate is promoted to MEASURED 95–159, mean ~116, with the instrument and its flags named |
| **R5** | **PARTIAL** | The magnitude is given, four times over. But the measurement was taken at **cap 2048** while the seat a one-key arming would produce (`configs/instrument_v0.toml`) carries **`per_call_node_cap = 16384`**, and D-465 measured larger caps as *worse*. The headline is therefore a lower bound presented as the number. See **Q3** |
| **R6** | **DISCHARGED** | §10.5 carries the row-set question, the proof rate, and the reason the unfiltered set is kept (D-516's denominator). One arithmetic slip in the transcription (**Q7**) |
| **R7** | **PARTIAL** | The sink is named, diffed, invariant-ed (8), tested (14) and given its call-removed mutant, and the problem is real: `classify` returns `Step::Ignore` for every `info ` line at `capture.rs:172-174` and `ask` at `:229` continues past it. `classify` **is** the right place and adding a census arm ahead of the `info` catch-all breaks no committed test — I checked `an_unrecognised_totals_line_refuses_the_run_and_names_the_game_and_turn`, which asserts `classify("info depth_turns 1 nodes 4") == Step::Ignore`, and `totals_of` requires the literal prefix `"info totals "` so a census line cannot be misclassified as Totals. What is *not* discharged is the seam's specification: no `Step` variant, no census-file name, format, schema version or manifest row (**Q5**), and the wrong `go_line` seat (**Q2**) |
| **R8** | **PARTIAL** | The universal is retracted (see B1). Of R8's two named consequences, neither is fully fixed: the bench guard is split but re-registered ungated on a mis-derived bracket (**P4**), and the seat column misses tests 5 and 10 — the two whose call-removed mutants **survive** on the gate-off half of their "either" seat (**P2**) |
| **R9** | **DISCHARGED** | T2 is split into T2a (engine, 18 configs) and T2b (arena, 14 — WP-2.1's actual words), and both `serde(default` bans are cited at the lines R9 gave |
| **R10** | **DISCHARGED** | The emission point is pinned to one block after the last depth. I verified it is reachable: `protocol.rs:170-176` calls `go_reporting`, then `totals_line`, then `bestmove_line`, so rows returned with the outcome can be printed between them. Invariant 3a and test 13 are the right pins. Line ORDER itself is still untested (**S14**) |
| **R11** | **PARTIAL** | The side-to-move argument is written down and it is correct — `state.rs:129-133` says the stone count fixes turn, phase and mover, and `pistol-engine`'s `replay_stones` checks rather than trusts. But the design cites **`crates/pistol-search/src/position.rs:102-104`**, and the quoted text is in **`crates/pistol-engine/src/position.rs`**; `pistol-search/src/position.rs:102-104` is inside `place`. Wrong crate, in the fix for a quotation finding (**S1**) |
| **R12** | **NOT DISCHARGED** | R12's remedy was explicit: the pre-registered bracket must be *"an **upper bound the guard can falsify**, not a gain estimate"*. §9 registers a gain estimate (0.2–0.5 %) and then removes the gate from it entirely. See **P4** |
| **R13** | **DISCHARGED** | §3.1 states the fourth-word rule, `budget_token.rs`'s diff row carries it, and test 15 pins it by name |
| **R14** | **DISCHARGED — and it is the cause of P1** | §4, §6's `protocol.rs` row and test 16 all carry the handshake line. D-88's quoted sentence is verbatim at `decisions.md:202`. The design did not check what the `id` lines feed |
| **R15** | **DISCHARGED** | §3.1 states the `capture_sha256` change, and `capture.rs:103-109` digests exactly the three things it says |
| **R16** | **DISCHARGED** | §10.6 answers it for the census and returns the corpus dedup case to D-562(2) |
| **R17** | **PARTIAL** | Invariant 7 and the `# Panics` obligation are added, and the claim is **true** — I checked it numerically: `rotate` is `(q,r) → (−r, q+r)` with `checked_neg`/`checked_add`, so overflow needs \|q+r\| > 32767; hex distance bounds \|q+r\| ≤ 2·dist and rule 5 grows dist by ≤ 8 per stone, so overflow needs ≳ 2048 stones, ~1000 turns. But R17's actual ask was the wording of invariant 1, and invariant 1 now asserts in bold that `canonical_key` **"cannot fail"** and then points at invariant 7, which says it panics (**S6**). The bound is also asserted with no arithmetic and without citing `Symmetry::apply`'s existing `# Panics`, which already carries the argument |

---

## NEW FINDINGS ON REVISION 3

### BLOCKING

#### P1 — the R14 handshake line breaks §9's byte-identity obligation and §7 invariant 2, and byte-identity mismatch is a STOP condition

**The two claims that cannot both hold.**

- §4 / §6 (`protocol.rs` row) / §8 test 16: *"A census-capable binary advertises
  it on an `id` line"*, `a_census_capable_binary_advertises_the_census_on_its_id_lines`.
  Unconditional — a handshake has no token.
- §7 invariant 2: *"**Off the token, the engine's bytes are the pre-change
  engine's bytes**, over the standing position set."* §9: *"two-binary diff over
  the standing position set, output digest equal to the pre-change engine's. **The
  instrument is `tools/baseline_snapshot.sh`** and the referent is its INVARIANT
  BLOCK."*

**The site.** `tools/baseline_snapshot.sh:598` writes the engine's whole
handshake into the invariant block:

```
tools/baseline_snapshot.sh:578   grep '^id ' "$WORK/hs" >"$WORK/id" || fail …
tools/baseline_snapshot.sh:598           sed 's/^id /engine_id /' "$WORK/id"
```

inside the `{ … } >>"$INVARIANT"` group, and `$INVARIANT` is emitted above the
`# timing` marker (`:795`). **Verified in the design's own pre-change record**:

```
artifacts/prechange_gate_v0_run1.txt:6-15
engine_id name pistol
engine_id version 0.0.1
engine_id protocol v0
engine_id mode instrument
engine_id budgets depth_turns nodes
engine_id config /home/tom/.cache/wp20b-baseline/configs/gate_v0.toml
engine_id eval handcrafted_v0
engine_id tt_bytes 16777216
engine_id candidate_policy radius 1
engine_id weights_sha256 41ef5496…
```

Ten handshake lines, all above the marker. I reproduced §9's two quoted digests
exactly — `sed -n '1,/^# timing/p' | sha256sum` gives `7f8a6f97…` for
`gate_v0` and `06490795…` for `instrument_v0`, equal across both runs — so the
referent and the convention are not in doubt.

**Therefore an eleventh `engine_id census …` line changes the invariant block's
digest on every seat, with the token off, on the first `pistol` handshake.**
§7 invariant 2 is false as written; §9's obligation fails; and §9's own STOP
protocol names *"byte-identity mismatch"* as a STOP, on `wp20b-stopped`. A
package whose design guarantees its own STOP condition has not been designed.

**Why this is BLOCKING and not a wording repair.** The design cannot simply
narrow invariant 2 to "the `go` output", because §9 names the instrument and the
instrument's invariant block is the digest. Either the referent changes (which
re-opens a pre-change record already taken at `a56449b`, so the record must be
re-taken), or the handshake line is dropped (which un-does R14), or the design
states and justifies the digest move explicitly with a new pre-change record.
It is also the one place in this document where an obligation and a feature were
written by different fix-round rows — the header's `§7, §2` row and the `§9` row
— and neither read the other.

**Secondary limb, for whoever fixes it.** D-88's own sentence about handshakes
is *"the budget kinds the handshake advertises are **derived** by asking
`Budget::check_supported`, **never restated**"*. A census advertisement has no
such derivation source, so as designed it is a restated literal in the one place
D-88 forbids restatement. And `protocol.rs:131-135` documents the handshake's set
and order as fixed — a change to it is itself an ADR-line-grade change §6.1's own
reasoning would demand be stated.

#### P2 — the seat column, which is *the* fix for B1/R8, leaves tests 5 and 10 vacuous on the seat it permits them

§8: *"**Every test above that needs a firing runs on an armed seat**"*, and
*"tests 1, 7, 8 and 12 would have passed vacuously"*. Tests 5, 6, 10, 15 and 16
are seated **"either"**. Two of those five need a firing, and their own mutants
prove it.

**Test 5** — `without_the_token_a_go_line_writes_no_census_byte`, killing "token
check removed", which §8's own mutant table says "drives the binary over a `go`
line with no token and asserts on the WHOLE output". Remove the token check on a
**gate-off** seat: the engine collects a census, the census is empty because
`pvs.rs:602` returns before the closure on every node and `search.rs:287-288`
gates the root, zero rows are drained, zero lines are printed, the whole output
is unchanged, **the test passes and the mutant survives**. Test 5 is non-vacuous
only on an armed seat.

**Test 10** — `the_non_census_path_does_not_compute_a_canonical_key`, killing
"the key hoisted out of the census closure at `pvs.rs:623` and `search.rs:304`".
On a gate-off seat those closures are unreachable in *both* the mutant and the
original, because no firing happens at all. Nothing to observe, nothing to
differ, **mutant survives**.

So the two tests that pin the token's whole point and the coldness obligation's
whole point are seated on a plane where their own named mutants live. This is
`docs/process.md`'s named vacuity and D-527's defect class — the exact pair §8
invokes against revision 2 — surviving inside revision 3's remedy for them. The
fix is one word in two rows (`solver-ON`), which is why it is worth being exact
about: the remedy was applied to the four tests the *reviews* listed rather than
derived from the property, and the derivation catches two more.

**Related, and why "either" is the wrong shape anywhere:** an "either" seat is a
licence for an implementer to choose the cheap plane. For tests 6, 15 and 16 that
is harmless (grammar and handshake need no firing). For 5 and 10 it is the
finding. A seat column whose values are `solver-ON` / `solver-OFF` /
`no seat` — with "either" deleted — cannot be got wrong.

#### P3 — §6.1's seam breaks §5's coldness proof, and the rule that would repair it is not stated

§5's proof, which the header lists under "WHAT REVISION 3 DOES NOT CHANGE":
*"**On the non-census path `self.census` is `None`, the closure is never entered**,
and the added cost is the `is_some()` test that is already there."* That is the
whole discharge of the governing dispatch's scope 4.

Under revision 3's own seam the antecedent is false after the first census `go`:

```
crates/pistol-search/src/search.rs:216-221   take_trigger_census → std::mem::take(rows)   // leaves Some(vec![])
crates/pistol-search/src/search.rs:381       run.census = self.census.take();
crates/pistol-search/src/search.rs:523-525   // "so the rows of one search and the next accumulate in one place"
                                             self.census = run.census.take();
```

`collect_trigger_census` arms; `take_trigger_census` **leaves collection ON**;
`search` hands the option to the run and takes it back. There is no
`stop_trigger_census`. And `Searcher::clear()` (`search.rs:230-239`) clears the
table, the heuristics and the solver — **it does not touch `self.census`**, which
I checked line by line.

Consequences the design does not state:

1. **A session that has once asked for a census pays the fold on every later
   `go`.** `go nodes 400000 census` then `go nodes 400000`: on the second search
   `self.census` is `Some(vec![])`, the closure IS entered at every firing, and
   `canonical_key` is computed 95–159 times for rows nobody will read. §5's
   quoted-site proof does not cover it and dispatch scope 4 is not discharged.
2. **Rows cross `go` boundaries.** Those silently-collected rows sit on the
   `Searcher` and are drained by the *next* census `go` unless the engine re-arms
   with a fresh `collect_trigger_census` — which the design never says it does.
   Invariant 3a ("a firing has exactly one LINE") is then violated in the
   direction nobody tested: a firing from a **non-census** search appears on a
   later census search's block.
3. **`new_game` does not clear it.** §6.1 asserts *"**`new_game` clears them**
   with the rest of the per-game state (§9's determinism obligation asserts this;
   §6 now implements it)"* and §9 repeats it. `Pistol::new_game` calls
   `Searcher::clear`, and `clear` does not clear the census. §6's diff table's
   `search.rs` row says only "the same at the root site" — no row for `clear`, no
   invariant, no test. An asserted determinism property with no diff row and no
   pin is exactly the shape D-553's motivating instance had.

The repair is one sentence — *the engine arms with `collect_trigger_census` at
the start of every census `go` and `Searcher::clear` sets `self.census = None`* —
but it is a sentence about the seam M4 said an implementer must not be left to
invent, and §6.1 was the answer to M4.

#### P4 — §9's bench guard (ii) is registered as an unfalsifiable magnitude on a bracket that cannot be derived from its own stated inputs

§9: *"the fold is MEASURED at 22.99 µs a firing (R-COST) against MEASURED 95–159
firings per ask, so **at the bench budget the tax should read 0.2–0.5 %**, which
is at or under the harness's own resolution (R12) — so the registered abort
threshold is on (i), and **(ii) is recorded as a magnitude rather than gated**."*

**The arithmetic does not follow from the inputs.** The 95–159 figure is
`nodes 400000` (§2's own R-FIRE quotation). Seat (ii) is
`configs/bench_wp18c_solver_on.toml`, whose registered bench budget is
`go nodes 50000` (D-465), where the measured firing count is **26** (D-530). At
50 000 nodes, 95–159 firings × 22.99 µs = 2.2–3.7 ms against a ~110 ms search =
**2–3.3 %**, an order above the registered bracket. R12's 0.2–0.5 % came from
**26** firings at 50 000 nodes; its 400 000-node row was 0.30 %. The bracket may
be numerically survivable — it happens to contain both budgets' figures — but the
derivation printed beside it is not reproducible from the numbers printed beside
it, and a pre-registration is reviewed at the revision that governs the run.
Under D-291's second clause and `docs/process.md`, a registered number whose
stated derivation mixes two budgets is a finding before it is a bracket.

**And (ii) as registered cannot fail.** R12's remedy was named and specific:
*"a reason for its pre-registered bracket (rule 5, owed at §9) to be an **upper
bound the guard can falsify**, not a gain estimate."* §9 registers a gain
estimate and then explicitly removes the gate from it. Hard rule 5 requires "a
pre-registered hotspot, expected gain bracket, **and abort threshold**"; §9
supplies a threshold for (i) — a check whose expectation is literally zero — and
none for (ii), the only check that can see the cost at all. A registered output
that no result can contradict is `docs/process.md`'s vacuity moved from the seat
to the threshold, and it is the second time this obligation has been written
without a criterion.

**A third limb the split did not catch.** Check (i) is described as *"the check
the dispatch's 'a logging path that taxes the engine' sentence actually asks
for"*. It cannot see that tax: on a gate-off seat the firing site is unreachable,
so the hoisted-key defect §8 assigns to test 10 costs zero on both sides of (i)
too. The non-census tax that matters is **solver-ON, token-OFF** — which is
precisely the OFF arm of check (ii). Neither check is registered against it.

### MAJOR

#### Q1 — §6.1 misstates the `Engine` seam in three ways, on the interface rule 11 makes the contract

- *"`go_reporting` gains the census request as a parameter; `go`'s default
  forwards it as 'no census', **so every existing caller is unchanged**."* False.
  `go_reporting` is a **required** trait method. Adding a parameter changes both
  implementors — `crates/pistol-engine/src/instance.rs:86` and
  `crates/pistol-arena/src/bin/stub_engine.rs:146` — and all three call sites:
  `crates/pistol-cli/src/protocol.rs:172`,
  `crates/pistol-cli/tests/movetime_tests.rs:99`,
  `crates/pistol-engine/tests/engine_tests.rs:119`. Callers of `go` are
  unchanged; the sentence says every caller.
- *"`go_reporting` **returns them with the `BestMove`**."* There is no `BestMove`
  type in this workspace (`git grep BestMove -- crates/` is empty). The return is
  `SearchOutcome { best, info, provenance }`, a **pistol-search** type
  re-exported by pistol-engine. The design does not say whether `SearchOutcome`
  gains a field or the return becomes a tuple — the one decision M4 asked for.
  Note that if it is a field on `SearchOutcome`, that is a change to a
  pistol-search public struct and §6's `search.rs`/`info.rs` rows do not carry it;
  §6's `info.rs` row says "**NOT** the per-depth `SearchInfo`" and nothing else.
- **`crates/pistol-arena/src/bin/stub_engine.rs` is absent from §6's diff table.**
  It implements `Engine` and must change. §6 claims to be the diff.

#### Q2 — the token is threaded into the wrong function, contaminating the match pass and breaking a committed test neither §6 nor §8 lists

§6: *"the `--capture` census flag, and **the token threaded into
`BudgetSection::go_line()`**"*. `go_line()` is not the capture pass's:

```
crates/pistol-arena/src/bin/arena.rs:188-190   let go_line = config.budget.go_line()…   // the MATCH pass
crates/pistol-arena/src/bin/arena.rs:204       schedule::run(&config, &openings, &identities, &go_line);
crates/pistol-arena/src/capture.rs:112-115     pub fn label_go_line(nodes) { BudgetSection::Nodes{value}.go_line()… }
crates/pistol-arena/src/passes.rs:42           let go_line = crate::capture::label_go_line(label_nodes);
```

Threading it into `go_line()` as written puts the census token on **every match
game's** `go` line — pass 1, which the design's own scope says it does not touch.
The correct seat is `label_go_line`. R15's sentence said only that the token's
*spelling* threads through the arena's config module; the design converted that
into a change to the shared method.

And whichever seat is chosen, this committed test moves:

```
crates/pistol-arena/tests/capture_tests.rs:387-395
fn the_label_go_line_is_the_one_budget_section_spells() {
    assert_eq!(label_go_line(5_000), BudgetSection::Nodes{value: 5_000}.go_line()…);
}
```

It pins exactly the equality T3 breaks on the `label_go_line` seat. §6 lists no
test change and §8 lists no such test. F1 makes a point of the workspace-shape
test staying green *unchanged* as evidence; the design owes the same honesty
about the test that does not.

#### Q3 — F3's headline magnitude was measured at cap 2048; the seat a one-key arming produces carries cap 16384, and larger is worse

`diff` of the two configs' key lines:

```
configs/instrument_v0.toml        on_search_path = false   per_call_node_cap = 16384
configs/gate_staged_solver_v0.toml on_search_path = true    per_call_node_cap = 512
```

Those two files differ in **exactly those two keys and nothing else**, which is
what makes §10.1's *"a choice among existing shapes rather than an invention"*
fair. But the measurement the whole finding rests on — 14.5×/36×, "900–2 300
hours", "38 to 95 days", carried in the ONE LINE FOR THE MORNING, in F3, in §3,
in §10.1 and in **D-563** — was taken at `cap 2048`, and the design says so once
without drawing the consequence: arming `instrument_v0.toml` by flipping its one
gate key gives **cap 16384**, eight times the measured cap. D-465 measured the
direction: *"an ON-seat sweep over {32, 128, 512, 2048} moves the corpus ratio
only 0.045 → 0.103 for a 64-fold cut"* — a bigger cap gives a **worse** ratio. So
14.5×–36× is a **lower bound** for the arming the design tells the operator is
cheap and precedented, and the document presents it as the number. The operator
is being asked to rule on 38–95 days from a measurement of a different seat's cap.
D-563 carries the same omission into the ADR log.

#### Q4 — test 5 and test 16 contradict each other and the design does not scope either

Test 5 *"asserts on the **whole output**, not on the absence of a substring"*.
Test 16 requires the binary to advertise the census on an `id` line. If test 5's
"whole output" is a session's output, the handshake's census `id` line **is** a
census byte and test 5 fails on a correct implementation; if it is the `go`'s
output only, the design should say so, because "whole output" was chosen as the
strong form precisely to forbid narrowing. Both tests are in the same table, one
of them new in this revision, and neither names its scope.

#### Q5 — the sink is named but not specified, and the artifact it creates has no format, no manifest row and no rule-8 treatment

R7 asked where "beside" is and what writes it. §3.1 answers *"`classify` gains a
census arm ahead of the `info` catch-all and `ask` routes those lines to a sink
the capture owns; the rows are written to a census file named beside the capture,
one per capture run."* What is still unstated, all of it decidable now:

- **`Step` must gain a variant.** It is `pub enum Step` (`capture.rs:150-158`), a
  public item of the arena crate, matched exhaustively at `capture.rs:224-231`
  and asserted on in `capture_tests.rs`. §6's `capture.rs` row does not mention
  it. Adding a public enum variant is the same class of seam change §6.1 was
  written for.
- **`ask` returns `Result<(String, String), _>`.** Routing rows means changing
  its signature or giving it a `&mut` sink. Unstated.
- **The census file has no name, no format version, no header, no schema
  discipline and no manifest row.** The capture pass prints
  `capture_file::manifest_row(...)` (`passes.rs:56`) for the file it writes; the
  census file gets nothing. Rule 8 says artifacts are never committed and *"a
  committed manifest may sha-index them"* — the design creates a new artifact
  class and says nothing about how it is indexed, which is the obligation D-469
  and §9's own "artifacts exported with digests" line will land on at closure.
- **Test 14 needs an armed arena seat that does not exist.** Every committed
  arena config's engine seats point at gate-off engine configs
  (`arena_wp20_label_pilot.toml` names `configs/instrument_v0.toml` for both), and
  committed-config changes are out of scope, so the test must synthesise one.
  Fine, but unstated — and by the design's own F3 that test then runs 14.5×–36×
  slower than a gate-off one, in CI.

#### Q6 — `new_game` clearing the census is asserted twice and implemented nowhere

Covered under P3(3); raised separately because it is the *determinism* obligation
rather than the coldness one. §9 lists it under "Determinism"; §6.1 lists it as
implemented; `Searcher::clear` does not do it; §6's table has no row; §7 has no
invariant; §8 has no test. Determinism law (hard rule 4) is the one place this
project does not accept an assertion without a gate.

#### Q7 — §10.5's proof-rate denominator contradicts §2's own firing counts

§10.5: *"`proofs` came back **0, 0, 4, 0, 0, 0 — four in 654 firings, ~0.6 %**"*.
§2, three pages earlier, gives the firing counts of those same six entries:
95, 97, 102, 134, 159, 107 — which sum to **694**, not 654. The conclusion
survives (4/694 = 0.58 %, still "~0.6 %"), but the design carries a number that
its own §2 falsifies, transcribed from the red team's report without being
re-derived. In an arc whose two prior reports are largely quotation audits, a
document that quotes a reviewer's arithmetic without checking it against its own
is repeating the failure mode.

#### Q8 — §4's D-551 audit miscounts its own columns, and the audit is an exhaustiveness claim

§4: *"the **eleven** counts are decimal `u32`/`u64`/`usize` — one word each"*,
followed by separate bullets for `cover`, `attacker_proved`/`defender_proved` and
`defender_visits`. §4's field list has **fourteen** fields; subtract `key`,
`cover`, the two `*_proved` and `defender_visits` and **nine** remain
(`turns_from_root`, four `mover_*`/`opponent_*` pairs' worth = seven, plus
`cover_count` and `attacker_visits`). Nine, not eleven — and this matches
`TriggerColumns`' seven `u32`s plus `cover_count` plus `attacker_visits`. The
closing sentence then says *"the audit expires the moment a **twelfth** column is
added"*, which is already past. The paragraph's whole force is
*"**There is no** score field on a census row **and no other multi-word value**"*
— an exhaustiveness claim, discharging a scope-2 obligation of the governing
dispatch, resting on an enumeration that does not enumerate its own list.

### MINOR

- **S1 — wrong crate in an R11 citation.** §2 cites
  `crates/pistol-search/src/position.rs:102-104` for *"a `Set` position's declared
  `to_move` is checked against that count rather than trusted"*. The text is at
  `crates/pistol-engine/src/position.rs:101-104`; `pistol-search`'s
  `position.rs:102-104` is inside `place`. R11 gave the right crate.
- **S2 — N10 not discharged, and now duplicated.** The "built without census
  support" refusal has no mechanism in §6 and is now asserted in §3.1 *and* as
  invariant 6. D-424 (prose that constrains nothing is deleted) and D-423 (a claim
  the document makes twice) both apply.
- **S3 — N11 not discharged.** `wp21_DISPATCH.md` §4 lists three decisions owed
  and not F3's, in the same diff as the D-563 that creates the fourth and the
  §10.1 that calls it "the operator's".
- **S4 — N6 residue.** §5 quotes v1's scope 4 under a v2 governing header,
  unlabelled, while F1/F2/§10.0 now label theirs.
- **S5 — N13 not discharged.** §8's first call-removed row still names one of two
  removable calls, and names the wrong file for the one it names.
- **S6 — invariant 1 contradicts invariant 7 in bold.** *"**`canonical_key` is a
  pure function of the stones and cannot fail** — but see invariant 7"*, where
  invariant 7 says it panics. R17 asked for the wording ("cannot move a move and
  cannot end a search"); the fix asserts the falsehood and appends a pointer.
- **S7 — N8 residue.** "~119 800 distinct positions" is still unmarked though
  D-560 marks every one of its own figures ESTIMATED; option A's "MEASURED-free"
  is still a marking on a fact read from code.
- **S8 — a standing ADR still carries the retracted universal.** D-465's own
  headline is *"THE GATE STAYS `false` IN EVERY COMMITTED CONFIG"*, in a line that
  itself names `configs/bench_wp18c_solver_{on,off}.toml`. The retraction names
  the premise memo §P2.5 as the inherited source and corrects D-539 with a new ADR
  limb; it does not name D-465, which is what a successor greps. (D-520 glosses
  D-441 as binding deployment configs; D-465 has no such gloss.)
- **S9 — numbers taken from the red team and not re-derived.** "Eleven sites
  reference `TriggerObservation` outside its module" — `git grep` gives seven
  (`lib.rs:52`, `pvs.rs:76`, `pvs.rs:746`, `search.rs:80`, `search.rs:216`,
  `search.rs:759`, `search.rs:767`); the conclusion holds at all seven. §10.5's
  proof-rate paragraph carries no attribution to the red team at all, unlike F3
  and §2 which name it. See also Q7.
- **S10 — D-88's flip clause cited for something it does not cover.** §3.1
  invokes *"D-88's 'additive line kind' flip clause"* to license widening the
  `go` **input** grammar. D-88's flip clause names a machine-readable framing and
  offers an additive **line kind** as the remedy; `info census` is an additive
  line kind, the `go` third word is not. D-88's input-side sentence goes the other
  way: *"the `set` form's grammar is as strict as the tokens it carries"*.
- **S11 — citation slips.** `capture.rs:227` for `Step::Ignore => continue`
  (actually `:229`); `exchange.rs:199-205` for a doc sentence at `:198`. Both
  inherited from the red team unchecked.
- **S12 — §10.0(a) calls it "silent drift by definition" for a rule with no ADR.**
  `wp20s_design.md` §8's `key_full` disjointness rule appears in **no**
  `docs/decisions.md` line (`grep key_full` gives only D-560 and D-562, both about
  corpus key names). Hard rule 10's drift clause is about ADR lines. The ground
  still stands on "a landed, reviewed document that named this package first", but
  the invocation of rule 10 is an overreach — and the constructive consequence is
  that if that rule is load-bearing for D-537's count, this package's closure is
  where its ADR line belongs.
- **S13 — test 9's "mutant it kills" cell names no mutant.** It reads
  *"**F3's own pin** — the fact that makes the sweep's claim checkable"*. That is
  a good reason for the test and not a mutant; the column is being used for two
  different things.
- **S14 — N7's second half.** No test pins that the block lands after the last
  `info` and before `info totals` and `bestmove`, although §4 states the order
  three times and §6 puts it in the `protocol.rs` diff row.
- **S15 — D-564's flip clause forecloses rather than flips.** *"…and never
  whether pistol-cli takes the dependency"* states a standing constraint inside
  the slot that is supposed to say what would change the decision. Harmless, but
  a flip clause that rules out a branch is not a flip clause.

**On size (D-424), since the brief asks.** The document is ~980 lines for a diff
of about a dozen files, and most of it earns its place: §2 and §3 are option
matrices the Process requires, §8 is a test table, §4 is a wire schema. What does
not earn its place is the **repetition of F3**. The 14.5×–36× / 900–2 300 h /
38–95 days figure appears at lines 5–6, 163, 253–257, 563–564 and 945, and again
in D-563 — five times in one document and six in the package. D-423 is explicit:
*"A CLAIM THE DOCUMENT MAKES TWICE IS A DEFECT WAITING — state it once, in the
section that owns it, and have every other section point there instead."* F3 owns
it; the ONE LINE FOR THE MORNING is a standing format and may carry it; §3, §10.1
and D-563 should point at F3. Q3 is the concrete cost of that repetition: the
cap-2048 qualifier is stated once, at the site, and is missing from all four
restatements including the ADR line.

---

## THE STRONGEST SURVIVING ATTACK ON REVISION 3

> **This revision has learned to fix the sentence a reviewer quoted, and has not
> learned to derive the property the reviewer was pointing at — and the evidence
> is that every one of its four blocking defects is the *same* defect the round
> was convened to fix, standing one step to the left.**
>
> B1 and R8 were one finding: **a claim was asserted over a population that was
> never enumerated.** Revision 3 enumerates that population beautifully. It names
> all three arming configs, counts them against eighteen, quotes the committed
> file that had already written the correction, and lands the retraction in the
> ADR line. Then it writes a seat column, and seats tests 5 and 10 on "either" —
> because the reviews had listed tests 1, 7, 8 and 12 and the fix was applied to
> the list rather than derived from the property. On the gate-off half of
> "either", both tests pass and both of their named call-removed mutants live.
> The blocking finding's own remedy re-creates the blocking finding, in the same
> table, four lines below the sentence announcing that it has been fixed.
>
> M1 and R12 were one finding: **a registered criterion that the named defect
> preserves is not a criterion.** Revision 3 splits the guard onto two named
> seats, which is exactly the structural repair asked for, and then registers
> check (ii) as *"a magnitude rather than gated"* — a criterion no result can
> contradict — after quoting R12's own sentence about the harness's resolution
> while omitting R12's own remedy, which was to make the bracket a falsifiable
> upper bound. The vacuity moved from the seat to the threshold and the document
> reports it as discharged. And the bracket it registers cannot be derived from
> the two MEASURED numbers printed beside it, because one of them belongs to a
> different budget.
>
> M4 was one finding: **the seam is unstated and an implementer will invent it in
> the one place this project has forbidden itself to drift.** Revision 3 states
> the seam — and states it wrongly at three points (a "`BestMove`" that does not
> exist, an "every existing caller is unchanged" that is false for two
> implementors and three call sites, a missing `stub_engine.rs` row) — and omits
> the one rule the seam actually needed, the arming and disarming of
> `self.census` across `go`s. Because it omits that rule, §5's coldness proof,
> which the header lists under "WHAT REVISION 3 DOES NOT CHANGE", silently stops
> being true: after one census `go`, `take_trigger_census` leaves collection ON,
> `Searcher::clear` does not clear it, and every later non-census `go` in the
> session enters the closure the proof says is never entered. The proof did not
> change; the thing it was a proof *about* did.
>
> And R14, the smallest MINOR in either report, was adopted in full without
> asking what reads an `id` line. `tools/baseline_snapshot.sh:598` reads them,
> into the invariant block, which is the referent §9 names for byte-identity —
> and §9's pre-change record was already taken, at `a56449b`, with digests this
> review reproduced to the character. So the document now guarantees, on its own
> instrument, the STOP condition its own §9 lists first.
>
> **The pattern is a fix round that treats findings as text edits.** The
> arithmetic that was checked is right — R1's 281/291/859 recompute exactly, and
> that is real work. The arithmetic that was *transcribed* is wrong: 654 firings
> where its own §2 says 694, eleven `TriggerObservation` sites where `git grep`
> says seven, eleven census counts where the schema has nine, 0.2–0.5 % from a
> firing count measured at eight times the budget, 14.5×–36× from a cap eight
> times smaller than the one the operator would actually arm. Every one of those
> came from a report rather than from the tree, and the arc's own diagnosis of
> itself — *"a wrong attribution is the defect this arc keeps paying for"*, §4's
> own words — names the habit precisely while the document goes on doing it.
>
> **The package is still designable and the two selections still stand.** C2 is
> right, T1+T3 is right, F1/F2/F3 are right, and B1's retraction is a model of
> how a finding of that class should be recorded. What the record cannot yet
> support is the header's claim that revision 3 *is* the one fix round: on the
> evidence of this review it is the first of two, and the second one's job is to
> stop editing sentences and start deriving the properties the sentences were
> about — which means, concretely, seating every test by asking what its own
> mutant needs, registering every number against the budget it was measured at,
> and reading what consumes each byte the design proposes to add.
