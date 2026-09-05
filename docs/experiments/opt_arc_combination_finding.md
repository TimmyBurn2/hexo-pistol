# The combination: three mechanisms together DID buy two turns of depth, and lost anyway. That is the arc's answer.

The arc measured its two perf packages combined (tranche 1: **1.294 / 1.293**
end to end, against **1.327** predicted by multiplying the terms, so they overlap
on a shared hot path). **The five behaviour-changing packages were each measured
alone and never together**, and each landed gated off. This closes that gap.

**Governing document:** `configs/arena_combo_vs_off.toml`. **Seat**:
`tier_t_top_k = 16` (W1's calibrated cap) + `root_reorder = true` (W2) +
`lmr_min_depth_turns = 1, lmr_late_index = 3` (S3), against the committed
zeros. S2's extension is EXCLUDED: it is the one package measured harmful on its
own (8 W / 61 L, D-605), and combining a known loss to see whether it stops
losing is not a question. **Budget** `nodes 200000`, **bounds** H0 = 0,
H1 = +10, alpha = beta = 0.05, cap 600 pairs — the same registration S3 used.

**Sensitivity, run first**: the combination changes most of 24 governed openings
at the match budget, so it is present in play; W1 and W2 alone changed none.

## The result

| | |
|---|---|
| verdict | **h0** — `LLR pair -2.954666` against the bound `-2.9444` |
| n / distinct-n | **910 / 910**, no duplicate games, 455 pairs |
| record for `combo` | **315 W / 391 L**, 204 capped (capped fraction 0.224) |
| pair outcomes | p0 82, p1 77, **p2 180**, p3 67, p4 49 |
| normalized Elo estimate | **−34.19** |
| **deepest turn reached** | **`combo` 8, committed 6** |
| nodes | `combo` 1 632 717 424 over 10 277 searches; committed 1 728 770 648 over 10 313 |
| receipt | `artifacts/opt_arc_combo_sprt_v1.txt`, sha256 `8c64e8fdb6ec5857…` |

## THE FINDING, and it is the one the whole arc was built to produce

**The combination did exactly what it was supposed to do.** On 5.6 % FEWER nodes
it reached **8 turns where the committed engine reached 6** — two whole turns
deeper, the thing every package in tranches 3 and 4 was trying to buy. W1's cap
and S3's reductions compose: each removes work the other does not, and together
they convert that saving into depth in a way neither did alone.

**And it lost.** 315 W / 391 L, h0 crossed at 910 games, normalized Elo −34.

**Two more turns of search made this engine measurably WORSE.** That is not a
failure of the mechanisms; it is a measurement of what depth is worth here.
Every other result in the arc is the same statement from a different angle:

- **S2** bought depth on forcing lines and lost 8 W / 61 L (D-605).
- **S3** alone bought no depth at all and was UNDECIDED (D-607).
- **The combination** bought two turns and lost by more than S3 did.
- **The anchor** shows sealbot three turns deeper and ahead 60 / 40 (D-608).

The ordering is the argument: **more depth correlates with worse play across
this arc's own experiments, and the only variable held constant throughout is
the evaluation function.** D-428 already holds that eval v0 misreads horizons.
A deeper search resolves more positions against a static evaluation, so where
that evaluation is wrong, depth propagates the error faster than it corrects it.

**WHAT THIS RULES OUT.** Search-side depth work is not the lever at eval v0, and
the arc measured that rather than assumed it — four ways. A successor that
proposes another pruning or extension technique against this eval is proposing a
fifth measurement of the same thing.

**WHAT IT LEAVES.** The eval, and the branching factor. Sealbot reaches 5 turns
where this engine reaches 2 (`opt_arc_perf_finding.md`), so the depth gap is
real and the engine that has it also wins — which says the problem is not that
depth is worthless, but that THIS eval cannot use it. The two live candidates
are the codebook net that Stage 2 exists for, and the quiet ball's median width
of 76 that W1's calibration named. Neither is a search-pruning package.
