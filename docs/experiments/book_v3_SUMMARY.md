# `book_v3` — CLOSED. What shipped, what it cost, and the four findings

**The book exists.** `crates/pistol-cli/tests/fixtures/random_openings_v3.txt`,
**8500 openings**, sha256
`9453763625c83a8a95d31bf5ee62e5ec6db91552d0fe81a604b190067e32f240`, committed as
a rule-7 fixture beside v1 and v2. It is disjoint under canonical form from
book_v1, from all of book_v2, and from the labelled corpus, and internally
distinct — all four counts **0 of 8500**, measured by a script that shares no
draw, no filter and no rendering with the builder. The D-568 reservation is
whole. No strength claim was made and no range of v3 is consumed.

**This package stopped once and was resumed by the operator**, who returned the
blocking decision to the session rather than ruling on it. It then ran three more
rounds under an explicit two-round grant. Detail lives in
`book_v3_registration.md` (premises, sizing, receipts, the fix round);
`matrix_book_v3_storage.md` (the decision); `docs/book_v3_ledger.md` (the
consumed-ranges record).

## The five findings, in the order they cost most

### 1. The dispatch's storage ruling applied the wrong rule, and the enum it named could not host the job

D-645 ruled book_v3 a hard-rule-8 artifact: uncommitted, under `artifacts/`,
resolving through a new manifest. Two things were wrong with that, and the first
is what stopped the package.

**`book_v3` is a rule-7 FIXTURE by this repository's own written test.** D-151
draws the line by CONTENT — *"no engine judgement, no eval, no search result, no
solved value"*, and *"a balance-filtered book WOULD be a book"* — and D-175
applies it to this exact generator in as many words. `k_stones` stays 5 and there
is no balance filter, so v3 is v1's and v2's class.

**And `BookVersion` could not do what the ruling assigned it.** It lives in
pistol-cli, is write-side only — it picks a file name and a header preamble —
and **pistol-arena references it at zero sites**. "V3 resolves through the
manifest" had no site to live at, and the dispatch's own Scope line named the
wrong crate. The digest refusal the ruling wanted already existed as one run-time
refusal (`ArenaError::OpeningsDigest`, re-verified on every load) and two gates.

Settled the way the Process section requires — an OPTION MATRIX attacked by a
fresh-context DECISION-RED-TEAM **before** selection, which BV3-9 had waived on
the "zero free parameters" premise that P1 falsified. Recorded at **D-647**.

### 2. The red team broke my own argument, and the measurement it demanded replaced it

The matrix's first revision answered the strongest attack — that filtering
against a corpus derived from engine play is D-151's *"the moment a curation step
consults an engine"* — by arguing v3 would never be regenerated when the engine
changed. **That is a misreading**: D-151's flip is unconditional, and revision 1
promoted a description of bookhood into a necessary condition on the flip. The
argument is withdrawn.

What replaced it is a measurement nobody had taken. A game passes through exactly
one five-stone position — its own opening, which the book chose and no engine did
— and the corpus was labelled from book_v2 openings `13..3499`. So over all
89 805 deduped rows: **3487 five-stone keys, `corpus₅ \ book_v2 = 0`, and
`(corpus₅ ∩ v3) \ (book_v2 ∩ v3) = 0`.** D-644's corpus clause is extensionally
empty given its own book_v2 clause. That removes the engine-derived input from
the filter entirely — the flip has nothing to fire on — and it is why the filter
reads only committed files, which is what lets the rebuild run on a machine that
does not have the 43 MB corpus.

The red team also caught that revision 1 reported **exact-line** overlap (0 and
2) where the mandated filter is **canonical** (9 and 29): 38 openings are
rejected, not 2, and reaching zero by re-rolling the seed is arithmetically
closed at `P ≈ e⁻⁴³`.

### 3. A test that was green in CI and red under every narrower invocation

REVIEW-impl asked for a test driving the SHIPPED builder rather than a
re-implementation. The test written for it located the tool under `examples/` and
asserted its presence. **A bare `cargo test` builds example targets; `cargo test
--test <name>` does not.** So it passed under `tools/ci.sh`, which runs the bare
form, and failed everywhere else.

The cost was not a red test. **It turned the R3 mutation baseline RED, and a red
baseline makes every mutant verdict in that run meaningless** — a whole mutation
set silently worthless while all 21 gates reported green. Fixed at the target
kind: `build-book-v3` is now a `src/bin/` binary, where cargo guarantees
`CARGO_BIN_EXE_*` and builds it for every integration test.

**The same defect was already in the tree, on this package's own instrument.**
`crates/pistol-arena/tests/sprt_power_tests.rs` uses the identical lookup and its
comment stated the false premise verbatim — *"`cargo test` builds examples, so
the path below exists whenever this test can run"*. All six of its tests fail
under the narrow command in a fresh worktree, and `sprt_power` is what produced
`POWER.txt` and this package's `POWER_v3.txt`. That one keeps its target kind for
a stated reason; the false sentence is corrected, because **the wrong belief was
read out of the tree rather than invented**. Recorded at **D-648**.

### 4. The registered pair cap was a coin flip, and the book survived it by luck

The sweep's first crossing of power 0.90 was 7800 pairs at 0.9001, against 7750's
0.8992 — and **the simulation's standard error at p = 0.90 with 20 000 runs is
0.00212**, so those two numbers are separated by one twentieth of a standard
error. Measured across eight seeds: **at 7800, seed 8 answers 0.8989 — below the
threshold the cap was registered for**; at 7750, two seeds answer above it. At
8000 every seed answers ≥ 0.9039.

`pairs_v3` is corrected to **8000**. **The book does not move**, because
`ceil_to_500(P + 500)` returns 8500 for every `P` in `(7500, 8000]` — the
insensitivity the registration had already recorded is what contained a
registered-number defect to a registered number. Had the rule been sharper, the
same coin flip would have shipped the wrong book. Recorded at **D-649**, together
with a second property probed in the same pass and found to hold: **v3 is
EXTENDABLE** — the first 8500 openings of a 9000-opening v3 are byte-identical,
so a run already part-played can be continued into a larger book if 8500 is ever
short.

### 5. The mutation harness left the mutant compiled, so its own evidence had to be withdrawn

The harness backed a source up with `cp`, mutated it, ran the tests, and restored
with `mv "$file.orig" "$file"` — which puts back the **backup's** timestamp,
older than the artifact cargo had just built from the mutated source. Cargo
compares mtimes, sees a source older than its own artifact, and **rebuilds
nothing**: the cached binary keeps the mutant while `git status` and `git diff`
both read clean. Measured at diagnosis: the restored `filter.rs` timestamped
34 minutes OLDER than the test binary built from it.

It surfaced only by luck — a later baseline went red with `left: 76, right: 38`,
and 76 is exactly twice 38 because the stale binary still carried M7's
`rejected += 2`. **A harness with this defect prints exactly what a correct one
prints**: every mutant still dies, because a mutant compiled in is a mutant under
test. What it silently loses is the ATTRIBUTION — mutant N+1's verdict may be
mutant N's binary, and the baseline licensing the run may be the previous run's
last mutant.

**The first mutation set is withdrawn, not re-interpreted**, and M1–M7 were
re-taken from a purged build cache: baseline GREEN, all seven dead. The harness
now touches the file on restore, asserts `git diff --quiet` on it and aborts the
run if the restore did not land, and touches every source before the baseline.
Recorded at **D-650**.

**A pattern across findings 3, 4 and 5 worth stating once**: each fix was correct
and each exposed a consequence only the full gate set could see — the bin
conversion moved the workspace's shipped-binary count, the new tests crossed
rule 9's line cap, and the mutation runs exposed the harness. The gates earned
their keep three times; the lesson is to read `EXIT=0` before calling anything
green, not to have skipped the fixes.

## Evidence

| what | where |
|---|---|
| sizing sweep | `artifacts/book_v3/POWER_v3.txt` |
| seed stability + extendability | `artifacts/book_v3/STABILITY.txt` `dbae4c4e…86a5d7a` |
| decision red team | `artifacts/book_v3/REDTEAM_storage.md` `b6f27a59…01800e8` |
| REVIEW-impl | `artifacts/book_v3/REVIEW_IMPL.md` |
| mutants M1–M7 (re-taken, D-650) | `artifacts/book_v3/MUTANTS.txt` — baseline GREEN, all seven dead |
| disjointness receipt | `tools/book_v3_disjointness.sh`, four counts 0 of 8500 |
| loadability (a)(b)(c) | `artifacts/book_v3/smoke_report.txt`; `OpeningsDigest` at exit 2 |

Generation cost **0.036 s** for 8500 openings on this workstation — the handoff's
"~4.5 h" is the SPRT RUN at the 2.046 s/opening seat and never was generation.

## What a successor must know

- **v3 is filtered, and its bytes depend on two other files.** That is a knowing
  departure from D-518, which declined exactly this filter for v2. Rebuilding
  needs `configs/random_openings_v3.toml` **and** the committed v1 and v2.
- **The corpus is not an input** and must not become one: keeping it out is what
  lets a fresh clone rebuild and verify the book.
- **Disjointness is at the OPENING.** It does not claim that games played from v3
  never transpose into a position the corpus holds at greater depth. D-644 asks
  for openings and openings are what was delivered.
- **Two things this package did not take, BOTH SINCE TAKEN.** The ruling on
  book_v2's now under-powered 1000-opening holdout is **D-653** (retained whole
  as the large-effect screening book). The stale D-143 citation on the arena's
  load path — `openings.rs:47-48` justified windowing by "content-hash order"
  where these books are emitted in generation order — is **D-667**: the
  conclusion survived, the cited reason did not, and the comment now names the
  emission order each book states in its own header.
