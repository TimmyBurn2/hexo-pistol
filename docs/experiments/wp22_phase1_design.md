# WP-2.2 Phase 1 — Texel tuning of eval v0: design, revision 2.

Governing revision: `71fa6f1` (`dev`). This revision amends §2's
constraint handling (projection replaced by an exact constrained solve) and §8's
named successor, both forced by
`docs/experiments/wp22_phase1_fit_finding.md`; the amendment reopens this
document's review however small the diff. Premise:
`docs/experiments/wp22_phase1_premise.md`, whose §3 and §4 are two findings this
design is built around rather than a summary of the tree.

## §1 What this phase changes, and what it does not

**It may change exactly five integers** in `configs/eval_v0_weights.toml`, and
only on h1. It changes no code on the search path, no config key, no schema, no
protocol line and no gate. **The loader it would otherwise have had to build
already exists** (premise §4), so this phase is smaller than the dispatch
assumed and its risk is concentrated in the trainer, which is offline, and in
the SPRT, which is the only voice (D-614).

## §2 The trainer, and why it has no seed

**The model is linear in the five weights** (premise §2), so the squared-error
fit has a CLOSED FORM: form `A = Σ g gᵀ` and `b = Σ g y` over rows, solve the
5×5 system. **No seed, no learning rate, no initialisation, no stopping rule
enters the answer** — which is a stronger determinism claim than "seeded and
deterministic given seed + corpus digest" and is the reason to prefer this
objective over an iterative one.

**Stages, each a pure function of the one before:**

1. `tools/texel/features.py` — a position's six-vector. Replicates
   `windows_through` (`crates/pistol-core/src/window.rs:112`) and
   `contribution` (`crates/pistol-eval/src/handcrafted.rs:179`).
2. `tools/texel/extract.py` — walks the deduped manifest, joins to the corpus
   records, writes one row per position: `f1..f6`, label, `to_move`, depth,
   book, result, and the `sha256` of `key_full`. **Every corpus digest it read
   is written into the row file's own header**, so a fit names its inputs by
   content.
3. `tools/texel/fit.py` — the closed-form solve, the projection onto the
   schema's constraint set, and the diagnostics.

**Rows the fit excludes, each by a property of the row and never by its
residual:**

- `score_kind != eval`. A mate score is the search's mate band and is not a
  number this table can produce (premise §6).
- `|label| >= EVAL_MAX`. The prediction is the band edge whatever the weights
  do, so the row carries no gradient in `w` and including it fits the clamp
  rather than the position.

**The split is content-derived**: train and validation are separated by the last
hex digit of the position's `key_full` digest, a 1-in-8 validation slice. No
seed, and the split moves with the corpus rather than with the row order.

**The constraint set, and the correction revision 1 needed.**
`weights.rs:141-177` requires `w1 >= 1`, strict increase, and every entry below
`EVAL_MAX`. Revision 1 PROJECTED the unconstrained solution onto that set;
**that is not the constrained optimum** — a projected point can sit on a face
the true optimum never touches — and on this corpus the two answers differ
(`[1,22,23,69,70]` against `[1,21,22,64,65]`). It is replaced by **exhaustive
enumeration of the sixteen active sets** of the four gap constraints, each a
reduced least-squares problem solved exactly. Still no step size, tolerance or
seed. **Whether any constraint BINDS is reported**, because a solution the
constraint set had to hold back is a different claim from one it did not — and
on this corpus two of the four do bind
(`docs/experiments/wp22_phase1_fit_finding.md`).

## §3 The engine side: nothing is built, and two tests say so

Premise §4 records that `weights_file`, the digest in the identity line, and the
arena's refusal of a mismatch all already ship. **The dispatch's "byte-identity
when the key is absent" cannot be built**: hard rule 1 makes an absent key a
named error and there are no code-side literals to fall back to. The two tests
that carry the property the dispatch was reaching for:

1. **Inert to presentation.** A weights file differing from the committed one
   only in comments and whitespace gives **byte-identical** engine output at a
   fixed instrument budget.
2. **Not inert to values** — the mutant that must die. One table entry
   perturbed gives DIFFERENT output. A test that cannot fail this way tests
   nothing, and this is the call-site mutant D-55y asks for before review.

## §4 The correctness gate the fit rests on

**The extractor is not trusted because it looks right.** `tools/texel/features.py`
re-implements the engine's window bookkeeping in another language, and a
re-implementation that silently disagrees would produce a confident fit to the
wrong features.

**The oracle**: for a registered sample of corpus positions, the value computed
as `clamp(Σ f_k w_k + f_6 · EVAL_MAX)`, signed for the side to move, must equal
the engine's own `HandcraftedV0::value` on the same position, **exactly, for
every position in the sample**. The engine side is a second instrument that does
not share the Python enumeration; it reads the board the search reads.

**A single mismatch stops the phase** — it falsifies premise §2, on which
everything else here rests.

## §5 Diagnostics, which gate nothing (D-614)

Train and validation MSE, Spearman rank correlation on validation, the fitted
ratios against the committed ones, the count of saturated and mate rows
excluded, and the by-depth breakdown. **They live in artifacts, never in a
document** (D-483). They answer whether the trainer is broken, not whether the
engine is stronger.

**The human corpus** (`b2fe61eb…`, D-453 ARTIFACT-GRADE, present locally at
`/home/tom/Projects/hexo-bootstrap-corpus/hexo_human_corpus.jsonl`) carries
**outcome labels only** — no search scores — so it cannot report a label MSE and
is used for outcome correlation alone. That limit is the corpus's, not this
design's.

## §6 Bench

The eval's cost is unchanged **by construction**: the same terms, the same
windows, the same table lookup, five different integers in the table. A
registered spot-check runs anyway, because "by construction" is an argument and
the bench is a measurement. Expected bracket: **no change outside noise**; a
measured change is a finding that something other than the five integers moved.

## §7 SPRT — the only voice

- **Seats**: the committed weights against the fitted weights, everything else
  identical, both seats at the same revision and the same config but for
  `weights_file`.
- **Instrument**: 50 000 nodes, fixed-node so the comparison is node-matched.
- **Openings**: the holdout openings ledger; if the holdout is thin, a
  `book_v3` slice is registered and generated first under `book_v2`'s
  discipline, with consumed ranges receipted.
- **Reported**: n, distinct_n, pentanomial, llr_pair, per-side compute.

**The honest expectation, registered before the run.** The v0 feature set is
five numbers describing how many windows hold how many stones. It cannot see
shape, cannot see whose turn it is beyond the sign, and cannot see a threat.
Re-fitting five integers to search scores is expected to move Elo **little**,
and the ROADMAP's Stage-2 bar of +150 Elo is written for the codebook net, not
for this (D-614).

**h0 is a FINDING and not a failure**: it says the v0 FEATURE SET is the limit
rather than its weights, which is exactly Phase 2's premise — and Phase 2
proceeds on it either way. **h1 moves the committed weights** as an artifact
digest, with the pin re-recorded and R4's cap re-test scheduled (D-613).

## §8 What this design does not decide

**The named successor if h0 is the CENSORING fix, not the link function.**
`wp22_phase1_fit_finding.md` measures that excluding `mate_in` / `mated_in` rows
by kind drops 48 % of every position in the corpus holding a five-stone window,
and drops them on one side of the question — so `w5` is fitted to lines that
did not convert. A mate row is not missing data; it is the observation *"this
position's value is at or beyond the band"*, which a censored likelihood uses.
**The sigmoid-link variant moves behind it.** Both are named HERE, before the
SPRT, which is what keeps the later choice from being a post-hoc one.
