# `book_v3` — STOP at R1, on a classification premise

**Read this first.** The package stopped after R0 (premises) and R1 (sizing),
before R2 (implementation). No book was generated, no opening was drawn, no
range was claimed, the D-568 reservation is whole and untouched, and no strength
claim of any kind was made. Detail and every quotation are in
`docs/experiments/book_v3_registration.md`.

## The blocking finding, in one paragraph

`book_v3` is not a **book** under hard rule 8. It is a **fixture** under hard
rule 7, by this repository's own written test — D-151 draws the line ("no engine
judgement, no eval, no search result, no solved value"; "a balance-filtered book
WOULD be a book") and D-175 applies it to this exact generator, in as many
words. BV3-3 rules the opposite: that v3 is a rule-8 artifact living under
`artifacts/`, never committed, resolving through a new manifest. Three of that
ruling's consequences are costs it does not name. The digest refusal it asks for
**already exists three times over** — in-band (`# body_sha256` in the book's own
header, re-verified by `openings::load` on every load, refusing with the named
`ArenaError::OpeningsDigest`), out-of-band (the digests pinned as source
constants in gate 3, whose own comment calls this *"the convention
`tactical_v0.txt` and the corpus fixtures already use (CLAUDE.md rule 7)"* and
notes it catches strictly more than the in-band pin), and by byte-for-byte
regeneration from the committed config. A manifest would be a fourth path and
the weakest of them, desynchronisable by a stale file. `BookVersion` **cannot host the behaviour
assigned to it**: it lives in pistol-cli, is write-side only (it picks a file
name and a header preamble), and pistol-arena references it at zero sites — so
the Scope line "pistol-arena (BookVersion::V3 and its loader)" names the wrong
crate and a mechanism that does not exist. And an uncommitted v3 **loses the
byte-for-byte regeneration test v1 and v2 both have in gate 3**, degrading
BV3-2's own reproducibility requirement from a standing gate to a one-time
receipt.

## Which premise or fork it hit

**P1**, and it hit it by succeeding rather than by failing: P1 asked for the
enum, its variants and every match site, and the answer is that the enum is in
the wrong crate for the job BV3-3 gives it. R0's rule is "any failure: STOP".

The waiver in **BV3-9** — no OPTION MATRIX, no DECISION-RED-TEAM, on the ground
of "zero free parameters" — is what P1 falsifies. Where v3's bytes live and
which digest path governs them is a named decision with more than one viable
option and different failure modes. CLAUDE.md's Process section says that is
settled by an OPTION MATRIX attacked by a fresh-context DECISION-RED-TEAM
**before** selection, and calls an option adopted without one "the same breach as
silent architecture drift". This is not an OPERATOR OVERRULE either: that
instrument is for when the code is done and a document blocks it, and no code was
written.

A fourth item belongs to the same ruling. **BV3-4 demands zero canonical overlap
against v1, v2 and the corpus.** No book here has ever met that: `book_v2_ledger.md`
records v2 against v1 at 1 identical line and **10 positions up to symmetry**,
"what chance gives", pinned by a test. Reaching zero needs a rejection filter —
and filtering against the corpus `key_full` set is a curation step whose criterion
derives from **engine play**, which is verbatim D-151's flip clause ("the moment a
curation step consults an engine"). Whether that flip fires is itself the rule-8
question, and it has to be answered before the generator runs, not after.

## What was completed

- **§0 in full.** The ten paste-block lines are appended at **D-637..D-646** —
  none of the ten bracketed keys existed, so all ten landed. R-D's ROADMAP
  correction landed in the same commit (`0a430d2`): SPSA/Texel tuning moved from
  Stage 4 to Stage 2, and the Stage 4 header was corrected to match, nothing else
  in the tree citing it. `ps` showed no engine process before any run.
- **Two format decisions forced by gate 19**, both on correctness rather than
  taste. The lines land as `D-637 [key]: …` at column 0, unwrapped, one line per
  decision: `tools/decision_key_check.sh` extracts keys with `grep -oE '^D-[0-9]+'`,
  so a pasted `- D-637 …` would not register as a key at all and would be
  invisible to the uniqueness gate. Gate 19 re-run green afterwards: **648 keys,
  no repeat outside the D-276/D-277 exemption**. Gates 17, 20 and 21 also green.
- **D-644's two dangling citations resolved in the line itself**, on the precedent
  of D-565, D-568, D-569 and D-611, because the log is append-only and a dangling
  reference appended now is permanent. `D-56x` is **D-562** (the tree says so at
  `wp21_DISPATCH.md:198`); `D-56q` is **D-568**, which took that number in its own
  first sentence.
- **R0**: P1..P9 quoted at `0a430d2`, in `book_v3_registration.md` §Premises.
- **R1**: run, not merely computed (D-628 §7). Instrument cross-check reproduces
  POWER.txt's 4000 and 8000 rows digit for digit.
  - **`pairs_v3` = 7800 — MEASURED** (smallest cap at power ≥ 0.90; 7750 gives
    0.8992, 7800 gives 0.9001).
  - **Floor = 5233 — DERIVED**, `ln(19)²/t1²`; reproduces `training_pipeline_2026-09.md`
    §5 exactly. It does not bind.
  - **`openings_v3` = 8500 — DERIVED** via `book_v2_registration.md` §4's
    `ceil_to_500(P + 500)`.
  - **The size is insensitive to grid refinement**: `ceil_to_500(P+500)` is 8500
    for every `P` in `(7500, 8000]`, and the crossing is bracketed in `(7750, 7800]`.
  - BV3-1's ">2x ⇒ STOP" fork **did not fire**; 7800/8500 is the expected order.
  - Receipt: `artifacts/book_v3/POWER_v3.txt`, sha256
    `ea77d4416e8d2f39c89d7a4d7aa7319cfc78e5c8022cb524c49628fe612d3dbf`.
- **P8 closed a conditional in R2's favour**: the arena handshake does **not**
  advertise book versions (`identity.rs` is engine identity), so no golden
  transcript refresh arises. The goldens are untouched.

## What was NOT done, and why

R2, R3, R4, R5 — implementation, call-site mutants M1..M6, REVIEW-impl, closure.
All of them depend on the classification question above: R2's first two
obligations are the manifest and the digest refusal, and both change shape or
disappear under the other ruling. No partial book is registered, per the STOP
protocol.

Generation wall time is **unmeasured**, because no generation was run. BV3-7's
attribution stands as recorded in D-646: the handoff's "~4.5 h" is an SPRT run
cost at the 2.046 s/opening seat, not generation.

## What a resume needs

**One operator ruling, on the classification.** Everything else follows from it.

- **If v3 is a fixture (rule 7), which is what D-151 and D-175 say:** v3 is
  generated into `crates/pistol-cli/tests/fixtures/random_openings_v3.txt` and
  committed like its two predecessors; `configs/random_openings_v3.toml` carries
  the seed; the pinning test that regenerates it byte-for-byte joins gate 3;
  **no manifest is built**, because the header's own `body_sha256` and
  `ArenaError::OpeningsDigest` already do that job on every load. BV3-3 and
  BV3-5(c) are struck; D-645 is superseded by a new line. Diff surface becomes
  **pistol-cli**, not pistol-arena. This is the cheaper option and the one the
  standing ADRs point at.
- **If v3 is an artifact (rule 8):** then D-151's and D-175's test needs an ADR
  saying why this book differs from its two predecessors, `artifact_check.sh`
  needs to know about it, and the loss of the gate-3 regeneration pin needs
  accepting on the record.

**Separately, and needed either way:** a ruling on BV3-4's zero-overlap demand —
whether a rejection filter against corpus keys is the curation-consults-an-engine
flip D-151 names, and whether zero is the right target when the measured
precedent between two existing books is 10 symmetry-overlaps of pure chance.

A resume also adds `book_v3_registration.md` to
`tools/governing_citation_check.sh`'s `GOVERNING` list. It is deliberately absent
now: that list names documents that govern a run, and no run was licensed.

## State of the tree

`dev` at `0a430d2`, clean, one commit added by this package (the §0 decisions and
ROADMAP commit). Nothing in pistol-core, pistol-eval, pistol-search, pistol-solver
or pistol-engine was touched; no engine code was touched at all. No branch
`book-v3-stopped` was needed — there is no work in progress to park.
