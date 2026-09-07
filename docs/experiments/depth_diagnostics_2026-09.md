# Depth diagnostics — M7 and M9 read from games already played, 2026-09

**Question**: why did more depth lose (D-609, D-610). Two of the eight
mechanisms `docs/research/search_next_2026-09.md` §1.1 names are testable on
games already recorded — **M7** (short games / under-sampling, §1.4) and **M9**
(type-D nodes, §1.5). This document reads both off the three optimization-arc
SPRT reports. **No engine was run. No remedy is proposed** (D-696).

**Governing revision** `235b6db`. **Instrument** `tools/depth_diagnostics.py`
sha256 `aca899c64692f8f3f5290b6c87fb328f571fa513435951cd86206a534f1d8efd`,
tested by `crates/pistol-arena/tests/depth_diagnostics_tests.rs` under gate 3, whose
attribution guard is shown to discriminate by a mutant that swaps the two
report slots — receipt `artifacts/fast_wins/depth_diag_attr_mutant.txt`.
**Receipt** `artifacts/fast_wins/depth_diagnostics_v1.txt` sha256
`64dccf9186252da1ad24805cedad68162ce45463f910765ea0211ce8e9f30d10` — every
number in §2 is that file's, rendered by the instrument and not retyped from a
run log (D-483).

## §0 The verdict vocabulary, fixed before the count — and where it is not blind

Registered readings, stated here as they were formed before the instrument ran:

- **M7 SUPPORTED** iff (a) `p2` is the **strictly largest** of the five
  pentanomial cells **and** (b) `capped` is the **strictly largest** of the
  three game-result classes {`capped`, `p1_win`, `p2_win`}. Both halves are
  reported separately so a reader holding a different reading of "the largest
  cells" can apply it.
- **M9 SUPPORTED** iff the deeper arm's loss rate in behind-positions exceeds
  its loss rate in ahead-positions by more than the binomial 95 % interval
  half-width at those counts.
- **NEITHER** and **BOTH** are legal verdicts.

**WHERE THIS REGISTRATION IS NOT BLIND, said plainly.** The combination run's
pentanomial and its capped count were already published — `search_next` §1.4
prints `p0 82 / p1 77 / p2 180 / p3 67 / p4 49 … with 204 capped`, and the
dispatch that commissioned this document quotes them. So condition (a) for the
combination run was knowable before the reading was fixed. What is **new here**
and was not knowable: S2's pentanomial, all three runs' game lengths, all three
runs' per-arm reached depth, and the result-class partition condition (b) turns
on.

## §1 What the reports do and do not carry

`arena_report 4` `game` records carry `opening p1 p2 result end forfeit_by
reason turns dup_of nodes_a nodes_b depth_a depth_b llr_game llr_pair`. Reached
depth and game length are recorded per game; **no evaluation is**. The instrument
prints `root-score fields present on 'game' lines: NONE` for all three runs, and
`docs/experiments/fast_wins_CLOSURE.md` §1 P3 traces where the score is lost.

## §2 The numbers

| run | n | cap | pentanomial p0/p1/p2/p3/p4 | capped | mean turns (all / decided) |
|---|---|---|---|---|---|
| `combo_sprt_report.json` | 910 | 40 | 82/77/**180**/67/49 | 204 (0.224) | 25.6 / 21.5 |
| `s2_sprt_report.json` | 90 | 40 | 17/**19**/9/0/0 | 21 (0.233) | 24.3 / 19.5 |
| `s3_sprt_report.json` | 1200 | 40 | 73/102/**273**/92/60 | 286 (0.238) | 25.4 / 20.9 |

| run | arm | mean reached depth | max | wins | mean turns of its wins |
|---|---|---|---|---|---|
| combo | **combo** (deeper) | **5.24** | **8** | 315 | 23.4 |
| combo | committed | 4.34 | 6 | 391 | 20.0 |
| S2 | ext0 | 4.37 | 6 | 61 | 19.8 |
| S2 | ext2 (subject, and the **shallower** arm) | 3.68 | 5 | 8 | 17.1 |
| S3 | lmr0 | 4.38 | 6 | 475 | 20.9 |
| S3 | **lmr1** (deeper) | **4.55** | 6 | 439 | 20.9 |

**S2 IS NOT A DEPTH CASE AND IS REPORTED SO IT IS NOT READ AS ONE.** Its
subject arm `ext2` reached SHALLOWER depth than its control (3.68 against 4.37,
maximum 5 against 6) and lost 8-61 at normalized Elo -391.5. Whatever the
extension cost, it did not buy depth, so S2 bears on neither M7 nor M9 as a
depth mechanism; it is in the table because the dispatch names all three
receipts and because its pentanomial was not previously published.

Result classes: combo **204 capped / 322 p1_win / 384 p2_win**;
S2 **21 / 34 / 35**; S3 **286 / 410 / 504**.

**The instrument's numbers agree with the runs' own summary lines** on every
quantity both state — combo `n 910`, `204 capped`, `capped fraction 0.224`,
`315 W / 391 L`, `first player won 322 of 706`; S2 `8 W / 61 L / 21 capped`,
`p0 17 p1 19 p2 9 p3 0 p4 0`; S3 `439 W / 475 L / 286 capped`,
`p0 73 p1 102 p2 273 p3 92 p4 60`, `first player won 410 of 914`. Digests in
`fast_wins_CLOSURE.md` §1 P2.

## §3 M7 — short games and under-sampling: **NOT SUPPORTED**

Condition (a) holds in two runs of three: `p2` is the strictly largest
pentanomial cell for the combination run (180 against a next-largest 82) and for
S3 (273 against 102), and **fails for S2**, whose largest cell is `p1` (19) and
whose `p3` and `p4` are both zero — the extension arm never won a pair outright
in 45 pairs. Condition (b) **fails in all three runs**: capped games are 204,
21 and 286 against `p2_win` counts of 384, 35 and 504, so `capped` is not the
largest result class anywhere, and at a 40-turn cap the mean decided game runs
19.5–21.5 turns, roughly half the horizon. The conjunction the verdict
vocabulary registers is therefore not met and **M7 is NOT SUPPORTED**. What
survives of it is unchanged and was already visible in §1.4: a fifth to a
quarter of games do reach the cap, and the `p2` mass is large wherever the two
arms are close.

## §4 M9 — type-D nodes: **STOPPED, not decided.** The field gap, named.

The partition M9 needs — the combination run's 910 pairs split by the root-eval
sign at the deeper arm's first move — **cannot be taken from these reports**,
and the dispatch's default fork applies: D2 stops rather than adding a field.

**The gap is not a missing `SearchInfo` field.** `SearchInfo.score` exists
(`crates/pistol-search/src/info.rs:229`) and reaches the wire on the `info
totals` line (`crates/pistol-cli/src/report.rs:45,94`). The arena parses it in
the `--capture` label pass (`crates/pistol-arena/src/labels.rs:156-206`). **What
is missing is per-move persistence of it in the SPRT report**: no `game` record
field is an evaluation, `conclusion.rs:81`'s `score_a` is the pair's game score
(0 / 0.5 / 1), and the `moves` record is coordinates only.

**And the one mode that could recompute it refuses these reports by name.**
`arena --capture` re-asks a report's positions and records each score, but
`crates/pistol-arena/src/capture.rs:135,160` refuse a report whose "two seats do
not attest the same engine". All three runs are A-versus-B. The recovery path is
closed by the harness's own refusal.

**A resume needs exactly one thing**: an SPRT report grammar that writes the
answering seat's root score beside each turn. That is an arena change and an
`arena_report` version bump, and it is not taken here.

## §5 One observation, recorded and not built on

The deeper arm in the combination run reached mean depth 5.24 against 4.34 and a
maximum of 8 turns against 6 — D-609's "two turns of depth" — and **its wins
took longer than the shallower arm's** (23.4 turns against 20.0) while it lost
391 to 315. Separately, `p2_win` exceeds `p1_win` in all three runs
(384/322, 35/34, 504/410), the same direction
`docs/experiments/anchor_v6_seatswap_finding.md` measured against sealbot in a
different harness. Neither is a mechanism claim and neither is one of the eight;
they are recorded so a successor does not rediscover them.
