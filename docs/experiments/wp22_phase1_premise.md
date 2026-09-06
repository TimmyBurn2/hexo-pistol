# WP-2.2 Phase 1 — premise memo (D-477), revision 2.

Quoted at `71fa6f1`. Every claim below is a quotation or a file:line, and the
two findings are marked as such.

## §1 The v0 terms and weights, at file:line

**The whole tunable vector is FIVE INTEGERS.** `configs/eval_v0_weights.toml:49-54`:

```
[table]
1 = 2
2 = 12
3 = 60
4 = 300
5 = 1500
```

**What consumes them** — `crates/pistol-eval/src/handcrafted.rs:179-186`:

```rust
fn contribution(&self, counts: Counts) -> i64 {
    match (counts.p1, counts.p2) {
        (0, 0) => 0,
        (p1, 0) => i64::from(self.weights.window_value(p1)),
        (0, p2) => -i64::from(self.weights.window_value(p2)),
        _ => 0,
    }
}
```

summed over every length-6 window holding a stone
(`handcrafted.rs:191-209`), then clamped and signed for the side to move
(`handcrafted.rs:254-261`). `EVAL_MAX` is `16_000`
(`crates/pistol-eval/src/eval.rs:10`).

**The validation the loader enforces** (`crates/pistol-eval/src/weights.rs:141-177`):
`table.1 >= 1`, strictly increasing, and every entry strictly below
`DECIDED_WINDOW_VALUE = EVAL_MAX`. Six own stones is spliced in as `EVAL_MAX`
and is not a document key (`weights.rs:179-181`).

## §2 THE MODEL IS LINEAR IN THE FIVE WEIGHTS — the fact the whole phase turns on

Because `contribution` reads exactly one table entry per window and sums, the
pre-clamp score is

```
p1_score  =  Σ  f_k · w_k          k = 1..5
f_k  =  (windows holding exactly k P1 stones and no P2 stone)
      − (windows holding exactly k P2 stones and no P1 stone)
```

with the `k = 6` term fixed at `EVAL_MAX` and outside the fit. So a position's
feature vector is five integers, the fit is a five-parameter linear one, and
the observation-to-parameter ratio is not a constraint at any corpus size this
project will ever have. **This is derived from the code above, not assumed**;
§5 registers the check that falsifies it if it is wrong.

## §3 FINDING 1 — the scale of the weight vector is very nearly unidentifiable for PLAY

Alpha-beta's move choice is invariant under multiplying every weight by a
positive constant: every score scales, every comparison survives. So of the
five degrees of freedom, **only the four ratios can change a move**. A fit that
minimises squared error against search scores in absolute units spends most of
its capacity on the one direction that cannot move Elo.

**The one thing that makes scale matter, and it is not a small thing.** The sum
is clamped to `±EVAL_MAX` (`handcrafted.rs:254-257`). Weights scaled up
saturate more positions, and a saturated eval is FLAT — every distinction
inside the clamp is lost. So scale has an optimum set by the clamp rather than
by the labels, and a fit left free to choose it will not find that optimum by
minimising label error.

**Consequence the design must carry**: the fit registers a normalisation, and
the diagnostics report saturation rate. Scale is not fitted against the labels
in the same objective as the ratios.

## §4 FINDING 2 — the dispatch's engine-side deliverable already exists, and one clause of it cannot be built as written

The dispatch asks for *"weights loaded from an artifact behind a config key;
default = the committed literal weights; byte-identity of search output when
the key is absent"*.

**Two thirds of that is already in the tree.**

| dispatch clause | where it already lives |
|---|---|
| weights loaded from an artifact behind a config key | `config.eval.weights_file`, read at `crates/pistol-cli/src/bin/pistol.rs:90`; loader `crates/pistol-eval/src/weights.rs:45-63` |
| artifact digest in the identity line | `crates/pistol-cli/src/bin/pistol.rs:97` emits `weights_sha256 <digest>` |
| every committed config already points at the file | 21 configs carry `weights_file = "configs/eval_v0_weights.toml"` |

**WHAT REFUSES A MISMATCH, corrected in revision 2 because revision 1 named the
wrong code and the error pointed the wrong way.** `handshake.rs:116-164` refuses
a DUPLICATED or malformed weights line and compares nothing across seats. The
refusals of a cross-seat MISMATCH are `crates/pistol-arena/src/capture.rs:151`
(a capture's two seats must attest one engine) and
`crates/pistol-arena/src/replay.rs:239-246` (a replay against its source report).
**This is good news for the SPRT and not bad**: two SPRT seats carrying DIFFERENT
weight tables are not refused, which the design's registered dry run confirms by
running — its report carries a different `weights_sha256` on each seat and a
verdict line. Revision 1's text would have told a successor the run was
impossible.

**The clause that cannot be built as written is "when the key is absent".**
Hard rule 1 forbids a code-side default and makes a missing key a named error,
and `weights_file` is a required field — so absence is a REFUSAL, not a
fallback to literals, and there are no committed literal weights in code to
fall back to. There is therefore no "key absent" byte-identity test to write;
the honest test of the same property is stated in §5.

**This is a premise finding, not a STOP**: nothing about it blocks the phase,
and the phase is smaller than the dispatch assumed because the loader it asks
for is already shipped and already digest-bound.

## §5 What Phase 1 must therefore actually prove

1. **The loader is inert to presentation.** A weights file differing from
   `configs/eval_v0_weights.toml` only in comments and whitespace produces
   **byte-identical** search output at a fixed instrument budget. This is the
   test that replaces the dispatch's absent-key clause.
2. **The loader is not inert to values** — the mutant that must die. A file
   with one table entry perturbed produces DIFFERENT output. A test that
   cannot fail this way is testing nothing.
3. **The linear model of §2 is real.** For a sample of corpus positions, the
   feature vector extracted offline, dotted with the committed weights and
   clamped, equals the engine's own static eval of the same position. An
   externally derived referent: the extractor does not share the engine's
   window bookkeeping. If §2 is wrong, this is what says so, before any
   fitting happens.

## §6 The corpus and the label semantics, quoted from the schema

The corpus header states its own units, and these are quotations from
`/home/tom/pistol-runs/arc3r-sweep/tranche-1/corpus.txt`:

```
# param label_go go nodes 400000
# param opening_turns 3
# param score_units eval is pistol-eval's own integer units; there is no pawn on this board
# param score_sign from the point of view of the side to move at the root
# param mate_counts mate_in and mated_in count every turn from the root, both sides'
# param depth_meaning a completed search depth, except where search_nodes is zero, where it is a proof depth
```

**Column order** is `crates/pistol-arena/src/labels_file.rs:17-50`: `game`,
`turns_played`, `moves`, `key_seq`, `key_pos`, `key_full`, `to_move`,
`score_kind`, `score_value`, `best`, `depth_turns`, `search_nodes`,
`solver_nodes`, `book`, `result`, `end`.

**Three consequences for the trainer, each read off the schema above.**

- `score_units` is *pistol-eval's own integer units* — the SAME units the fit
  produces, so no conversion constant exists to be got wrong, and §3's scale
  finding is about those units.
- `score_sign` is *side to move at the root*, which is exactly
  `Eval::value(side_to_move)`'s convention (`handcrafted.rs:254-261`). The
  label and the model already agree; nothing is negated in the loader.
- `score_kind` is one of `eval`, `mate_in`, `mated_in`. **Only `eval` rows are
  fittable**: a mate score is the search's mate band and is not a number this
  table can produce (`configs/eval_v0_weights.toml:32-34`). Mate rows are
  excluded by kind, which is a property of the row and not of its value.

**The sixteen corpora** are at
`/home/tom/pistol-runs/arc3r-sweep/tranche-{1..16}/corpus.txt`, sha-indexed in
`artifacts/arc3r_sweep_raw_manifest.txt`; the deduped index of 89 805 distinct
positions is `artifacts/arc3r_sweep_deduped_manifest.txt`, whose dedup rule is
three-key agreement with the deeper label winning.

## §7 The holdout

Two holdouts, and they are different kinds.

- **The human corpus is NOT a holdout here, and revision 1's claim that it was
  is withdrawn.** `docs/ROADMAP.md` blocks D-434's Stage-2 calibration holdout
  until a population-grade corpus supersedes the artifact-grade one, and D-453
  licenses statements about the artifact and nothing *"generalizing to the
  platform's players"*. *"Whether a fit to engine labels transfers to human
  play"* is exactly that generalisation. The design deleted its own paragraph
  saying so; this one said the opposite in the same commit.
- **A registered split of the sweep corpus** — the same distribution, so it
  answers overfitting alone.

Both are diagnostics and neither is a gate (§R's R5: SPRT is the only voice).

## §8 What is NOT established here, and must not be read as established

- Nothing about how much Elo a tuned five-vector can be worth. The honest
  expectation is registered with the SPRT, not here.
- Nothing about R2's key-disagreement classes; the trainer's augmentation
  ruling is the ruling's own, and this memo does not take it.
- Nothing about Phase 2. The window-map store API and the per-pattern
  observation counts are Phase 2's premise memo and are not quoted here.
