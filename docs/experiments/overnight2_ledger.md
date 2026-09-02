# Overnight arc II — the ledger. A successor continues from THIS, never from memory.

**HOW TO READ IT.** One section per phase, appended to as the phase runs. A
phase is CLOSED only when its line says so and names the receipt. Anything not
written here did not happen.

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

## §1 — WP-2.0b implementation. **IN PROGRESS.**

Governing design: `docs/experiments/wp20b_design.md` revision 8.
Implementation account: `docs/experiments/wp20b_impl.md`.

| obligation | state | receipt |
|---|---|---|
| the diff, against design §6 | done | `docs/experiments/wp20b_impl.md` §4 |
| the five carried findings | closed inside the diff | `wp20b_impl.md` §1 |
| **CI, all 19 gates, at the closure tree** | **EXIT=0** | `artifacts/wp20b_ci_closure_v1.txt` |
| **byte-identity, gate OFF, at the closure binary** | **MATCH both configs, both runs** | `artifacts/wp20b_identity_RECEIPT.txt` |
| **determinism, all seats** | **EXIT=0, 5 seats** | `artifacts/wp20b_determinism_v2.txt` |
| **perf guard, under a REVIEWED registration** | **H1 = 0.9982, NOT REJECTED; no abort** | `artifacts/wp20b_perf_RECEIPT.txt`, raw `…_perf_guard_v2.txt` |
| **mutation receipts (D-553/D-55y)** | **27 registered, 27 dead at their registered test, 0 alive, 0 harness faults, EXIT=0** | `artifacts/wp20b_mutants_v7.txt` |
| **design §9's `key_pos` obligation** | **DISCHARGED, and the answer is ZERO** | `artifacts/wp20b_keypos_*.txt` |
| artifact manifest (rule 8, D-469) | committed, and every digest verifies | `docs/experiments/wp20b_artifacts.md`, live list `artifacts/wp20b_MANIFEST.txt` |
| REVIEW-impl rounds 1 and 2 | both **FAIL**; every finding disposed | four reports in `docs/experiments/` |
| REVIEW-impl round 3 | the last D-565 grants, and under **D-567** a failure STOPS the arc | owed |
| closure D-line, commit, tag | owed, after round 3 | — |

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
| `eff179b` | REVIEW-impl round 2, both reports | obligations **FAIL** (5 BLOCKING, 7 MAJOR); code pending |

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

## §2 — WP-2.1 production label sweep. **REGISTERED, NOT STARTED.**

- Registration: `docs/experiments/wp21_prereg.md` revision 1.
- Ledger row added: `13 | 4487 | 13..4499`. **The book is now fully claimed.**
- Generator: `tools/wp21_tranche_config.py`, with
  `crates/pistol-arena/tests/wp21_tranche_config_tests.rs` driving the shipped
  script. Partition verified contiguous, disjoint, exhaustive.
- **NO TRANCHE HAS RUN, AND THAT IS AN OPERATOR INSTRUCTION RATHER THAN A
  BLOCKER.** The instruction, received during the arc: *"do not run the phase 2
  sweep — only do everything beforehand"*. Everything the sweep needs is landed
  and green; the run itself is not started, and starting it is the operator's
  call. **The resume point for tranche one is this line.**
- **What a successor does to start it**, so the instruction costs nothing later:
  build release at the closure head, take `sha256sum target/release/pistol`,
  then for each tranche `n` in 1..16 run
  `tools/wp21_tranche_config.py --tranche n --out <dir>/tranche-n.toml
  --binary-sha256 <digest>`, and drive each tranche's three passes in the order
  `wp21_prereg.md` §4 registers, eight tranches at a time. The per-tranche
  criteria are §4's table; the void rule and the two-consecutive-voids STOP are
  §4's closing paragraphs.

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
