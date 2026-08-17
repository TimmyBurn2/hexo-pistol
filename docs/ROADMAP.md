# pistol — Stage Roadmap

Authored in session 1 per CLAUDE.md. Changed only by ADR (one D-line in
docs/decisions.md per change). Stage gates are exit criteria; a stage is not
done until its gate holds. The research report
(docs/research/minimax_report.md) is the technique prior; SPRT is the judge.

## Stage 0 — Foundations (complete; WP-01..WP-07 landed)

Legal, correct, reproducible engine: board + rules + win detection in
pistol-core, perft-verified pair movegen, lazy 128-bit zobrist with phase bit,
handcrafted eval v0, PVS + iterative deepening + TT, line-protocol CLI,
deterministic instrument mode with a two-process CI gate.

Work packages WP-01..WP-07; gates named per WP. Exit: the engine plays legal
games via the line protocol under all three budget kinds, deterministically in
instrument mode, with perft-verified movegen and sane play at depth 4-6 turns.

Exit gate held at WP-07 (docs/decisions.md D-82..D-93). What it rests on: the
perft oracle (`tools/perft_check.sh`), the cross-process determinism gate
(`tools/determinism.sh`), the sha-pinned tactical fixture at its pre-registered
threshold (`tools/tactical_check.sh`), and self-play through the line protocol
with pistol-core as referee. The one exit criterion that is a *measured* answer
rather than a clean one is the depth: D-81's floor puts depth 4-6 turns inside
the 0.5 s deployment budget only at a narrower candidate radius than the one
that ships, and depth 3 at the shipping radius (docs/decisions.md D-89 records
the numbers). That gap is Stage 1's whole argument, and it is a finding rather
than a failure (CLAUDE.md rule 5).

## Stage 1 — Tactical core

Minimal pistol-arena lands FIRST (paired openings, GSPRT, distinct-game
dedupe, per-side compute accounting) — deviation from the report, which parks
the harness in Stage 5: Hard Rule 6 makes SPRT the judge of every Stage-1
change, so the judge must exist before the first defendant. First arena
experiment (doubles as its shakedown): candidate radius r=2 vs r=3,
fixed-node, paired openings, verdict pre-registered. Then: threat-first staged
pair generation + dominance pruning; killers/history/countermove on pair
moves; threat-only zone-bounded quiescence; upgrade the AND-OR solver to
relevance-zone Deep df-pn (+1+epsilon, GHI). The staged threat-first candidate
generator SUPERSEDES the radius policy as the primary candidate source (radius
stays as a config-selectable fallback policy).

Carried in from the WP-05/WP-06 reviews (docs/decisions.md D-102..D-108), in
order, before the threat machinery:

1. The **differential search oracle** (D-106): a full-width negamax reference
   in the pistol-search test tree, the search's equivalent of the brute-force
   movegen oracle (rule 7) and the from-scratch eval (D-68). It lands before
   threat generation because everything after it is a pruning change, and SPRT
   judges strength rather than soundness — a pruning commit that quietly changes
   the value of the tree is invisible without it. Two reviewers each built one
   ad hoc and each found zero divergences, so this is coverage that persists,
   not a suspected defect.
2. The **movetime-ceiling fix** (D-95): an interruptible or root-staged first
   iteration that can always answer with the best completed root move. On HeXO
   the server owns the clock and hard-clamps the call, so this is a forfeit
   risk rather than a known limitation. Play mode only.
3. The **decided-window floor** as a pre-registered arena experiment (D-105):
   whether a won position should evaluate at the band top rather than summing
   freely. It changes move ordering, so it is a strength claim (rule 6) and
   waits for the judge.

Exit: engine refutes the tactical fixture class at pre-registered thresholds;
every landed change SPRT-positive.

## Stage 2 — Cheap learned eval

Rapfi-style incremental pattern-codebook net: 3 directional maps, length-11
axial windows, integer-quantized + SIMD, incremental under 2-stone moves,
distilled from mantis self-play + human corpora. Acceptance bar
(pre-registered): node-matched SPRT >= +150 Elo vs handcrafted_v0; otherwise
handcrafted stays and the eval budget moves to search.

## Stage 3 — Forcing search

Full TSS/DBS with independent-region decomposition; CTSS conservative defense;
RZOP relevance zones wired into online search for VCDT/VCST detection. Every
threat count, zone radius, and win-density number RE-DERIVED for 3 axes —
never imported from square-board Connect6.

Exit: decisive forcing lines of 15-30 turns in sharp positions at negligible
node cost, SPRT-confirmed strength gain.

## Stage 4 — Parallelism + tuning

Lazy SMP (shared TT, staggered depths), ABDADA fallback if efficiency < 0.4 at
16 cores; SPSA/Texel tuning of eval weights and search margins; PROTOTYPE
gauntlet per the report's verdict table (guarded/verified null-move, LMR,
futility/razoring) — each kept only if SPRT-positive. Deterministic instrument
mode stays single-threaded and untouched.

## Stage 5 — Opening book + full harness

Offline JL-PN/SPDFPN solving of symmetry-distinct openings; 12-fold
canonicalized book; balanced-opening generator; pentanomial paired-game
manager; full reporting fields per the report's Deliverable 4 (instrument,
protocol, n, distinct-n, per-side compute, first-player win rate).
