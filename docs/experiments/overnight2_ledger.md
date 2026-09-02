# Overnight arc II — the ledger. A successor continues from THIS, never from memory.

**HOW TO READ IT.** One section per phase, appended to as the phase runs. A
phase is CLOSED only when its line says so and names the receipt. Anything not
written here did not happen.

**THE ARC STOPPED ONCE AND WAS RESUMED.** `docs/experiments/overnight2_STOP.md`
is the account of the D-567 stop; the resumed dispatch's rulings are **D-568**
and its licensed side-streams **D-569**. The sections below carry BOTH runs of
the arc: a row written before the stop still says what it said, and a row the
resumption changed says so on its own face. **§R is the resumption's own
section and is where a successor starts.**

---

## §R — the resumption. Its own first actions, and what they changed.

- **D-568 appended** — the architect's rulings on the stop: B1 and B2 remedied
  per round 3 with the B2 re-run taken, C2 retained with the zero-yield fold
  recorded, a 1,000-opening `book_v2` holdout reserved for governed runs, the
  sweep taking the remaining 3,487, and anchors licensed to use `book_v1`. Its
  standing limb outlives the arc: **a mutation set is specified against call
  sites enumerated by a `git grep` receipt recorded in the mutation document,
  never against prose.**
- **D-569 appended** — three read-only streams licensed alongside the arc: a
  repository audit with a findings ledger, a sealbot opponent study, and a
  search-enhancement gap analysis. None of them may edit the tree.
- **`dev` STAYS UNTOUCHED UNTIL §1 CLOSES GREEN.** The work is on
  `overnight2-stopped`; the merge is §0's second action and it is not taken
  early.

---

## §0 — first actions. **CLOSED.**

- **D-565** appended: the loop grant `D-56p` takes its number. Three rounds per
  review gate, the third remedies-only, a third failure is STOP and split. This
  dispatch only; it expires with it.
- **D-566** appended: `wp20b-stopped` deleted. The ancestry check returns **NO**
  — the branch is a sibling of `a56449b`, not an ancestor of `dev` — so the
  receipt is over CONTENT, path by path:
  `artifacts/ovn2_wp20b_stopped_SUPERSESSION.txt`. Five files exist on `dev` and
  not on the branch; three differ and in each the branch holds the earlier text.
- **dev green at `a6777f4`**: `tools/ci.sh`, all 19 gates, `EXIT=0`, cited from
  the gate lines in `artifacts/ovn2_ci_a6777f4_v1.txt`.

---

## §1 — WP-2.0b implementation. **CLOSED.**

Governing design: `docs/experiments/wp20b_design.md` revision 8.
Implementation account: `docs/experiments/wp20b_impl.md`.

| obligation | state | receipt |
|---|---|---|
| the diff, against design §6 | done | `docs/experiments/wp20b_impl.md` §4 |
| the five carried findings | closed inside the diff | `wp20b_impl.md` §1 |
| **CI, all 19 gates, at the REMEDIED tree** | **19 gate lines, `ci: all gates passed`, EXIT=0**, wall 30 m 37 s | `artifacts/wp20b_ci_closure_v2.txt` |
| **byte-identity, gate OFF, RE-TAKEN at the post-remedy binary** | **MATCH both referents, both runs — and the binary is UNMOVED at `7a7a2347…`** | `artifacts/wp20b_identity_RECEIPT_v2.txt` |
| **determinism, all seats** | **EXIT=0, 5 seats** | `artifacts/wp20b_determinism_v2.txt` |
| **perf guard, RE-RUN after its registration's review** | **H1 = 0.9981, NOT REJECTED; ON/OFF 1.0020, no abort; 0 refusals** | `artifacts/wp20b_perf_RECEIPT.txt`, raw `…_perf_guard_v3.txt` |
| **mutation receipts (D-553/D-55y)** | **31 registered, 31 dead at their registered test, 0 alive, 0 harness faults**, at the REMEDIED tree | `artifacts/wp20b_mutants_v8.txt` |
| **design §9's `key_pos` obligation** | **DISCHARGED, and the answer is ZERO** | `artifacts/wp20b_keypos_*.txt` |
| artifact manifest (rule 8, D-469) | committed, and every digest verifies | `docs/experiments/wp20b_artifacts.md`, live list `artifacts/wp20b_MANIFEST.txt` |
| REVIEW-impl rounds 1 and 2 | both **FAIL**; every finding disposed | four reports in `docs/experiments/` |
| REVIEW-impl round 3 | **FAIL — 2 BLOCKING, 2 MAJOR, 7 minor**; 25 of 30 prior findings CLOSED | `wp20b_impl_REVIEW_r3.md` |
| **B1 and B2 remedied, per D-568, with NO round 4** | **the scoped verification PASSES on all four limbs** | `docs/experiments/wp20b_VERIFICATION.md` |
| artifact index (round 3's M1) | **48 digests in the committed document**, where it carried zero | `docs/experiments/wp20b_artifacts.md` |
| closure D-lines | **D-570 to D-573** appended | `docs/decisions.md` |

**§1 IS CLOSED.** Every obligation above is green at the REMEDIED tree, and the
four receipts that matter attest ONE binary — `7a7a2347…`, the digest the earlier
closure receipts already named. The B1 remedy touched a `///` comment and added
tests; it changed no instruction the engine executes.

**THE CLOSURE REVISION IS `bf1c3ce193232f6779ca138353337d289ed277af`** — a `git stash create` object, because the work is
uncommitted on `dev` (HEAD `a6777f4`, which does not contain the diff). Every receipt
above names it; an earlier draft of two receipts named the PARENT commit, at which the
binary they attest cannot be built.

**THE NAMED REVISIONS, AND WHICH ARTEFACT EACH ONE GOVERNS.** The work is
uncommitted on `dev` (HEAD `a6777f4`), so every named revision of this phase is a
`git stash create` object. **They are NOT interchangeable and an earlier draft of
this line said they were** — a successor who took `8efeda0` as the reviewed tree
would be reading a revision whose own mutation receipt FAILED.

| revision | what it governs | verdict at it |
|---|---|---|
| `8efeda0` | mutation run 1, `artifacts/wp20b_mutants_v1.txt` | **19 dead, 1 ALIVE, EXIT=1** — superseded |
| `dfba9d7` | mutation run 2, `artifacts/wp20b_mutants_v2.txt` | 20 dead, 0 alive, EXIT=0 |
| `f7606cc` | REVIEW-impl round 1, both reports | code PASS (0B/2M), obligations FAIL (3B/8M) |
| `1cd3364` | mutation run 3, `artifacts/wp20b_mutants_v3.txt` | **23 dead, 1 ALIVE** (M24, a fourth vacuous test), EXIT=1 — superseded |
| `0aa0e2e` | mutation run 4, `artifacts/wp20b_mutants_v4.txt` | **ABORTED** after 23 mutants on a stale patch anchor, EXIT=1 — superseded |
| `595f004` | **mutation run 5, `artifacts/wp20b_mutants_v5.txt` — THE RECEIPT** | **26 registered, 26 dead at their registered test, 0 alive, 0 harness faults, EXIT=0.** Its tree is byte-identical to `eff179b`'s, so the receipt covers the reviewed code exactly |
| `eff179b` | REVIEW-impl round 2, both reports | obligations **FAIL** (5 BLOCKING, 7 MAJOR); **code FAIL** (1 BLOCKING, 4 MAJOR, 7 minor) |
| `7103ad1` | **mutation run 7, `artifacts/wp20b_mutants_v7.txt`** | **27 registered, 27 dead at their registered test, 0 alive, EXIT=0**, and its `crates/ tools/ configs/` is IDENTICAL to the tree round 3 reviewed |
| `81a8074` | **REVIEW-impl round 3, the last D-565 grants** | **FAIL — 2 BLOCKING, 2 MAJOR, 7 minor**; 25 of the 30 prior BLOCKING/MAJOR findings CLOSED |

**AN EARLIER DRAFT OF THIS TABLE SAID ROUND 2's CODE REPORT WAS "code pending"**,
and it had existed since 11:09 and returned FAIL. Corrected above (round 3's m1).

**THE CLOSURE REVISION `bf1c3ce` IS NOT THE REVIEWED ONE, AND THE DIFFERENCE IS
DOCUMENTATION ONLY — by the rule landed at `4375ad9` rather than by assertion**
(round 3's m2). `81a8074` is a docs-only descendant of `bf1c3ce`:
`git diff --stat bf1c3ce 81a8074 -- crates/ tools/ configs/` is EMPTY. That is
the terminating rule the receipt-recursion took: a receipt names the revision
whose CODE it attests, and a later revision that changes no code is the same
tree for every purpose a receipt has.

**WHICH MUTATION RUN CERTIFIES THIS CODE: run 7 AT `7103ad1`, AND EARLIER DRAFTS
OF THIS TABLE AND OF `wp20b_impl.md` §5 SAID RUN 5** (round 3's M2). Run 5's tree
at `595f004` differs from the reviewed one in **eleven source and test files** —
`census.rs` and `census_identity_tests.rs` among them, the very files round 2's
remedies rewrote — so run 5 knows nothing about `CensusKeys`, M27, M28 or M29,
and *"the receipt covers the reviewed code exactly"* was false at that revision.
**Run 8 supersedes run 7** for the remedied tree; see §R's closure row.

The byte-identity, determinism and perf-guard receipts were taken between
`8efeda0` and `dfba9d7`, at a tree whose only later source change is `///` doc
comments in `protocol.rs` — verified by the obligations reviewer, and the reason
`target/release/pistol` at `15c94598…` is the binary of the reviewed code.

**THE PERF GUARD'S FIRST RUN IS SUPERSEDED AND ITS NUMBERS ARE NOT THIS
PACKAGE'S.** They are kept here because a superseded measurement is still on the
record: at REPS=5 in a fixed arm order, under the estimator the instrument
actually computed, H1 was **0.9979** and the ON/OFF comparison **1.0012**; the ON
arm wrote **181 census rows** and both OFF arms none, identically across five
reps. **Read under the estimator §3 revision 2 registers — ratio first, then
median — the same raw gives H1 `0.9968`**, which is the point: revision 1
registered one statistic and its instrument computed another, and the gap
between them is why the run is being taken again rather than re-read.

**§9's `key_pos` MEASUREMENT — TAKEN, AND THE ANSWER GOES AGAINST THE OPTION IT
WAS ASKED ABOUT.** `wp20b_design.md` §9 registers *"tranche one emits `key_pos`
beside the canonical key, and the two distinct counts are compared"* as the
two-line answer to §2's strongest surviving attack — that C2 *"pays for a fold
whose only measurement is zero"*, the pilot corpus having agreed at 347 on all
three keys. **A tranche cannot carry it**: the sweep runs census-OFF by D-56p,
and §4's field order is pinned by a report test so the wire may not grow a
column. So it was taken on `crates/pistol-search/examples/trigger_census.rs` —
the instrument every census number in this arc was measured with — over the same
two committed fixtures and the same two caps §1.1 used:

| fixture / cap | firings | distinct `key` | distinct `key_pos` | keys covering >1 position |
|---|---|---|---|---|
| corpus, 2048 | 400 | 320 | 320 | **0** |
| corpus, 16384 | 63 | 57 | 57 | **0** |
| trigger-rich, 2048 | 294 | 287 | 287 | **0** |
| trigger-rich, 16384 | 41 | 40 | 40 | **0** |

**THE IN-TREE SYMMETRY FOLD MERGED NOTHING, on 798 firings.** The two counts are
equal on every cell and no canonical key covers more than one position key.
**This is the red team's point, measured and confirmed at the population it said
had never been measured**: the fold's yield is zero in-tree as well as at the
root. **IT DOES NOT MAKE C2 WRONG, AND IT DOES NOT MAKE C2 THE ONLY
OPTION EITHER — an earlier draft of this line said it did.** What the landed
ruling requires is an identity CONSISTENT WITH `key_full`, and the design's own
matrix marks **three** of five options compliant: C1, C2 and **D'**. The red
team's alternative was option **A** (`key_pos`), which F2 forbids a priori — §8
DEFINES disjointness rather than measuring it, so a zero yield licenses no
identity that folds one equivalence and not the other; that much is untouched.
**What the measurement makes live is D'**, which the matrix's own row calls *"12
transforms, zero sorts, zero allocations — cheaper than C2"* and which was
rejected only for being *"a fourth notion of sameness"*. C2's 22.99 µs a firing
now buys a fold measured at zero yield, and a successor weighing that trade
should know a compliant cheaper option sits one row away. **MARKED LIMITS**: bench fixtures rather than the sweep's own corpus
positions, 798 firings, and one engine's deterministic search — whose
lexicographic tie-break is not symmetry-invariant (D-137), which is the
mechanism `canonical_sequence`'s own doc gives for why mirrored lines are rare.
A successor may reasonably re-take it on corpus positions. Artifacts:
`artifacts/wp20b_keypos_{corpus,trigger-rich}_{2048,16384}.txt`.

---

## §2 — WP-2.1 production label sweep. **REGISTERED AT REVISION 2, NOT STARTED.**

- Registration: `docs/experiments/wp21_prereg.md` **revision 2** — the
  1,000-opening holdout of D-568, with §2's partition and §3's wall arithmetic
  **re-derived for 3,487** rather than scaled in prose. Revision 1's review does
  not transfer; **revision 2 owes a fresh-context review before tranche one**,
  and so does the generator, whose own revision changed with it.
- Ledger rows: `13 | 3487 | 13..3499` for the sweep and
  `3500 | 1000 | 3500..4499` **RESERVED FOR GOVERNED RUNS — NEVER LABELLED**.
  **The book is no longer fully claimed**, which retires the STOP document's
  owed item 3.
- **The dispatch says 3,500 and the arithmetic says 3,487**: the round number is
  the book's remainder before the pilot's thirteen are subtracted. Recorded in
  the registration's own preamble rather than rounded away.
- **The holdout is enforced by a test, not by a sentence**:
  `no_tranche_reaches_the_reserved_holdout` drives the shipped generator over all
  sixteen tranches and fails if any slice crosses `3500`.
- Generator: `tools/wp21_tranche_config.py`, with
  `crates/pistol-arena/tests/wp21_tranche_config_tests.rs` driving the shipped
  script. Partition verified contiguous, disjoint, exhaustive.
- **THE SWEEP IS HELD BY A STANDING OPERATOR INSTRUCTION, RE-IMPOSED DURING THE
  RESUMPTION.** Mid-arc-I the instruction was *"do not run the phase 2 sweep —
  only do everything beforehand"*. The resumed dispatch's §3 reads as directing
  the run, and this ledger briefly recorded the hold as spent on that ground.
  **It is not.** The operator's words during the resumption: *"wait before you
  launch the sweep"*. **TRANCHE ONE DOES NOT START WITHOUT THE OPERATOR SAYING
  SO**, whatever else is green, and a successor reading §3 of the dispatch must
  read this line with it.
- **THREE THINGS ARE OWED BEFORE THE HOLD IS EVEN THE BINDING CONSTRAINT**, and
  they are worth doing while it stands: §1 must close green and be merged (the
  sweep's governing revision is the WP-2.0b closure head); revision 2 of the
  registration must pass its fresh-context review, and so must the generator,
  whose own revision changed with it; and
  `docs/experiments/wp21_throughput_prereg.md`'s two levers decide the sweep's
  concurrency and whether the capture pass searches every position twice.
  **The resume point for tranche one is this line.**
- **What a successor does to start it**, so the instruction costs nothing later:
  build release at the closure head, take `sha256sum target/release/pistol`,
  then for each tranche `n` in 1..16 run
  `tools/wp21_tranche_config.py --tranche n --out <dir>/tranche-n.toml
  --binary-sha256 <digest>`, and drive each tranche's three passes in the order
  `wp21_prereg.md` §4 registers, eight tranches at a time. The per-tranche
  criteria are §4's table; the void rule and the two-consecutive-voids STOP are
  §4's closing paragraphs.

---

## §2b — sealbot anchor v3. **BLOCKED ON A CAPABILITY THE PLATFORM DOES NOT HAVE.**

**THE FINDING, recorded the moment it was found rather than at launch.** The
resumed dispatch's §2 asks for an anchor over *"50 registered openings from
book_v1 … each played both colors"*. **The local matchserver plays exactly one
opening**: a single stone at the origin, `const OPENING: Coord = Coord::new(0, 0)`
at `tools/sealbot/matchserver/src/referee.rs:17`, applied by the referee itself
before either engine is asked. There is no opening file, no skip, no take, and
the transcript records the opening as a fixed sentence.

**SO §2 IS TWO PIECES OF WORK AND THE DISPATCH BUDGETS ONE.** The *"quiet box,
~1-2 h"* is the RUN's cost and it is right; it is not the cost of the capability
the run needs. The capability is designed in
`docs/experiments/anchor_v3_openings_design.md` and carries the obligations any
non-trivial change carries — REVIEW-design, IMPL, REVIEW-impl, RED-TEAM on the
opening data path, the `tools/SHELL_CHECKLIST.md` coverage rule, mutation
receipts, CI gate 16.

**WHAT IS NOT BLOCKED**: everything §2 fixes that is not the openings — the
budget, the seat, the report's shape, the UNVERIFIED-opponent clause and the
no-Elo clause — is registration work and is written against the platform as it
will be.

**THE ONE LIMB NOBODY HAS EVIDENCE ABOUT**: the platform has never handed
sealbot a non-origin, multi-stone setup. The shim's contract can express one
(`setup` is the first ply, `moves` is the rest), which is a reading of the
matchserver's code and NOT a claim about sealbot. The design's dry run exists to
attribute exactly that, and a failure there STOPS the anchor rather than being
worked around.

| step | state | receipt |
|---|---|---|
| design revision 1 | **superseded** | `anchor_v3_openings_design_rev1_SUPERSEDED.md` |
| REVIEW-design round 1, fresh context, at `963c8fdd` | **FAIL — 3 BLOCKING, 7 MAJOR, 6 minor**; 9 attacks rejected | `docs/experiments/anchor_v3_openings_design_REVIEW.md` |
| design revision 2 | written, every finding disposed at the section that owns it | `docs/experiments/anchor_v3_openings_design.md` revision 2 |
| **OPTION MATRIX** for the reader's home (the review's MAJOR 4) | **revision 3, after TWO failed red-team rounds** | `docs/experiments/matrix_anchor_openings_reader.md` |
| DECISION-RED-TEAM round 1, at `5a76a47e` | **1 FATAL, 7 MAJOR, 4 minor** — *"the option survives; the matrix does not"* | `..._REDTEAM.md` |
| DECISION-RED-TEAM round 2, at `c5d900a4` | **2 FATAL, 6 MAJOR, 6 minor** — the derivation held, the RECOMMENDATION did not | `..._REDTEAM_round2.md` |
| DECISION-RED-TEAM round 3, at `60359cfb` | **1 FATAL, 5 MAJOR, 4 minor** — *"the OPTION survives; amend §4, do not re-take"* | `..._REDTEAM_round3.md` |

**THE MATRIX FAILED TWICE AND THE TWO FAILURES ARE DIFFERENT, WHICH IS WHY BOTH
ARE RECORDED.** Round 1's FATAL: §1's refusal table claimed to be derived from
`crates/pistol-arena/src/openings.rs` and was a **verbatim transcription** of the
REVIEW-design finding that raised it — same eight items, same order, same line
ranges — omitting seven sites, one of which (`a_file_with_no_digest_line_is_refused`)
has its own test. Derived properly there are **seventeen** refusal sites and two
scope rules. **That is D-568's standing law broken in a second document class
four hours after the law was written for the first.**

Round 2 confirmed the re-derivation held — it re-ran the printed grep, opened all
seventeen hits and found the mapping bijective — **and killed the recommendation
instead**, on two FATALs. (a) The option revision 2 moved to could not both keep
its claimed benefits and comply with the landed design's §5.4. (b) **Both facts
that moved it were wrong**: the SHA-256 revision 2 called a barrier is a
**92-line, zero-`use`, std-only file already in this repository**
(`crates/pistol-core/tests/common/sha256.rs`, D-37, FIPS-pinned) that a grep
scoped to `crates/pistol-core/src` did not see — **the same scoping defect as the
FATAL, one directory deeper** — and the whole-file-validation rule is free under
the option it was cited against.

**ROUND 3, at `60359cfb`: 1 FATAL, 5 MAJOR, 4 minor — and its verdict is
"the OPTION survives, the RECOMMENDATION does not, do NOT re-take the matrix a
third time; amend §4."** The FATAL is a structural fact nobody had looked up:
the recommendation rested on a differential test between the two readers, and
**the matchserver has no library target** (`main.rs:11-18` declares the modules;
there is no `src/lib.rs`) so no test can reach the reader, and **no gate would
run one if it could** — gate 3 is `cargo test --workspace` and the matchserver is
a DETACHED workspace; gate 16 drives `run_match.sh`, which runs `cargo build` and
never `cargo test`. **The mitigation named a test that no target could hold and
no gate would run**, which is the same defect revision 3 accused revision 2 of.
Round 3 also caught a THIRD scoping error — the line-count denominator counted
`src/bin`, 2 161 lines a library dependency never compiles, so the real figure is
**+24 681, 4 542 -> 29 223, 6.43x** — and a sentence that said "nine" and listed
eight.

**§4 IS AMENDED RATHER THAN RE-TAKEN, as round 3 directed.** The differential
becomes a **refusal-containment suite inside gate 16**: one deliberately
malformed book fixture per refusal row, driving the SHIPPED `run_match.sh`,
asserting the matchserver refuses each by name — a ONE-DIRECTIONAL claim
(*everything the arena refuses, the matchserver refuses*), because the design
requires this reader to refuse strictly more and two readers that must disagree
cannot be tested for agreement. The digest moves to `run_match.sh`'s existing
`sha256sum`. **The ADR line quotes round 3's attack rather than round 1's**,
because round 1's had a remedy and round 3 took it away: what survives is that a
fixture suite closes the gap only *on the fixtures somebody thought to write*,
and **O2 is chosen with that gap open**.

**THE THREE DEFECTS ACROSS THREE ROUNDS WERE ONE DEFECT**: a claim asserted at
the scope where it was convenient rather than derived at the scope where it is
true — a table transcribed from prose; a grep scoped to `src` that missed the
answer in `tests`; a line count scoped to `src` that counted `bin`. **D-568 was
written for the first and the other two happened after it**, so the law needs its
check attached: *print the command with its scope, and compare its hit count to
the prose's before believing either.*

**REVISION 3 RETURNS THE RECOMMENDATION TO WHERE ROUND 1's RED TEAM LEFT IT** and
says so: *"O2 is not chosen here because a third analysis favoured it; it is where
the field has stood since the first attack, and two revisions of this document
were the noise."* It carries four obligations, the load-bearing one being a
**differential test running both readers over the committed books and asserting
the same verdict and the same `Vec<Turn>`** — which is the strongest surviving
attack's own remedy, owned rather than named.

| the anchor's pre-registration | written, **NOT RUNNABLE** — the capability does not exist; every instrument is a REGISTERED SLOT | `docs/experiments/sealbot_anchor_v3_prereg.md` revision 1 |
| REVIEW-design round 2, IMPL, REVIEW-impl, RED-TEAM, dry run, prereg, run | owed | — |

**THE THREE BLOCKING FINDINGS, because a successor should not have to open the
report to know what they were.** (1) A dry-run criterion the shipped binary
cannot satisfy — *"the go line read back from the record"* — which the v2
pre-registration had already struck in as many words. (2) The opening specified
as `&[Coord]` *"in play order"* with no rule for deriving that order from the
book's CANONICAL `a/b` spelling, and a `panic!` pre-committed for when the
derivation is wrong; on `book_v1` the naive decode is correct **by accident**
(every stone inside `max_radius 5`, `LEGAL_RADIUS` 8). (3) The `replay_check`
remedy removed the openings from the second instrument's coverage while nothing
else covered them — a window off-by-one would give 100 games on one opening,
exit 0, and a report echoing `skip`/`take` from the config it was checking.

**WHAT SURVIVED ATTACK**, recorded because a review that only breaks things is
not a review: the premise (the matchserver really does lack opening support and
really is the thing to change), §1's sealbot claim (the shim replays `setup` and
`moves` through one loop, so a five-stone opening needs no shim change), and the
turn-cap arithmetic (57 engine turns against the platform opening's 59).

**AND THE MEASUREMENT THAT MAKES THE WHOLE PACKAGE WORTH ITS COST** is already on
the record: anchor v2 measured **DISTINCT GAMES 2 of 40** on both seats. The
origin stone is forced by game rule 3, so v1 and v2 played no opening at all, and
a `movetime` budget bought no diversity — *"the interval's nominal N is 40 and
its real one is 2"*, in v2 §10.1's own words.

---

## §3 — cap calibration. **NOT REGISTERED.** Waits on §2's deduped corpus.

---

## §4 — census run. **NOT REGISTERED.** Waits on §3's cap.

**D-537's REGISTERED MINIMUM IS COMPUTED HERE, FROM THE LANDED RULE, BEFORE ANY
CENSUS ROW OF THIS ARC EXISTS — which is the whole protective force the rule
was given.** `docs/experiments/wp20s_design.md` §8 defines it as *"the smallest
`n` at which a one-sample binomial test of `p0` against the alternative reaches
the registered level and power"*, with every input fixed there and read from the
**trigger-rich** band as a pair:

```
p0    = 8/14  = 0.571429   the incumbent: the best WRITTEN ordering's win recall
                           (docs/experiments/matrix_stage3_detector.md §5.4,
                            per-search frame, the frame the dispatch names)
p1    = 12/14 = 0.857143   the arc's measured column BOUND for the same band
alpha = 0.05   beta = 0.05 (power 0.95), the pair every committed arena config
                           carries and CLAUDE.md rule 6 judges by

smallest n with a critical c such that
    P(X >= c | p0) <= 0.05   and   P(X >= c | p1) >= 0.95

n = 28, c = 21
    size  P(X >= 21 | p0) = 0.040184
    power P(X >= 21 | p1) = 0.962225
```

**ONE CAVEAT, RECORDED RATHER THAN SMOOTHED OVER, because a successor opening
round 3 will meet it.** The two figures §8 names sit in DIFFERENT COLUMNS of
`matrix_stage3_detector.md` §5.4's table: `0.571` is that table's **per-search**
figure and `0.857` is its **bound over the columns**, which the same section
discusses in the aggregate frame. §8's own rule reads the null and the
alternative *"as a PAIR from one band"* — by BAND, and it says nothing about
frame — and each of its two descriptions matches exactly one cell, so the
reading above is the one the landed rule licenses. **It is written down because
the alternative readings move the number**: a successor who believes the pair
should be read within one frame owes an amendment to §8 under D-518's
off-the-end clause, and may never simply recompute a smaller minimum.

**THE MINIMUM IS 28 WIN-PROVING FIRINGS ON DISJOINT POSITIONS.** It is a FLOOR:
a pre-registration opening detector round 3 may register a larger one with
grounds and may never register a smaller one. Nothing about it was chosen after
seeing data — both recalls come from a CLOSED arc and both error rates from a
committed config — which is what D-537 asks for in its own words.
