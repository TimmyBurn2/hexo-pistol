# WP-1.9 — the rule-5 bench bracket, REGISTERED BEFORE THE RUN

**This document is committed before `tools/bench_delta.sh` is invoked for the
shipped implementation.** Registered numbers never move (D-374). If the run lands
outside the bracket, the bracket is what the record reports against — it is not
edited afterwards.

## The hotspot

`Eval::apply` / `Eval::undo` / `Eval::delta`'s per-window map operations, reached
through `pistol_search::ordering::order`. D-192's H1 measured that roundtrip at
**76.27 %** of pooled ordering samples against a pre-registered 20 % threshold —
quoted from the record as the licensing evidence, and NOT re-measured here.

## The comparison

- **Instrument:** `tools/bench_delta.sh rev:723758b rev:07f518b 5`.
- **Baseline `723758b`** — the commit immediately before the implementation.
  Engine-identical to `a5c5661`, which the matrix used: both build a `pistol`
  binary with sha256 `8dc2f922…`, verified, because the commits between them
  changed documentation only.
- **Candidate `07f518b`** — the shipped implementation.
- Config `configs/instrument_v0.toml`, fixed nodes 50000 and depth_turns 2,
  24 positions, 5 reps, both bands. The harness asserts per-position node
  identity and refuses the run if it does not hold.

## What is being tested, and why it is not circular

The matrix already measured this storage shape at **1.783 early / 1.909 late**.
That measurement was of the shape written INLINE in `handcrafted.rs`
(`wp19/mx-O2`, `9a986c6`). **The shipped implementation is not that code**: the
key packing, the hasher and the map live behind a `WindowMap` newtype in their own
module, which is a real change to what the optimiser sees across a module
boundary.

**So the registered hypothesis is that the module split is free**, and the run can
falsify it.

## The bracket, and the abort

| | Value |
|---|---|
| **Expected gain bracket**, whole-engine nps ratio, **both bands** | **[1.60, 2.10]** |
| **Abort threshold** | ratio **< 1.00 in either band** — any regression |
| **Below bracket but above abort** (1.00-1.60) | a FINDING, not an acceptance: the module split costs something, and the record says how much |
| **Above bracket** (> 2.10) | a FINDING owing an explanation, not a silent win |

The bracket is centred on the matrix's measured pair and widened to admit
run-to-run drift, which the matrix observed at ~0.5 % on nps between its
`rustfmt`-clean re-runs and its originals.

**Time-to-depth is the declared cross-check and not independent evidence** — nodes
to depth are identical by search identity, so its ratio is the nps ratio over the
depth-2 node mix. It is reported because rule 5 requires both, and it agreeing is
what a cross-check agreeing looks like, never corroboration.

## What this bench does NOT establish

It is not a strength claim and no SPRT is owed: Track E's identity proof is the
stronger oracle and it has already held (`artifacts/wp19_byte_identity_v1.txt`).
`bench_delta.sh`'s own printed `VERDICT` wording is against **its** `[1.4, 2.5]`
thresholds, which descend from D-220's package and are not this bracket.
